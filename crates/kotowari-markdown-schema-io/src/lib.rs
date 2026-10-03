//! Explicitly rooted acquisition for Markdown schema operations.

use kotowari_markdown_schema::finding::Finding;
use kotowari_markdown_schema::frontmatter::{ResolvedSchema, SchemaRef};
use kotowari_markdown_schema::{
    Document, PartialExtraction, Schema, ValidatedValues, ValidationOptions,
};
use std::path::{Path, PathBuf};
#[cfg(feature = "tokio")]
mod asynchronous;
#[cfg(feature = "tokio")]
pub use asynchronous::{AsyncOptions, AsyncSchemaLoader};

#[derive(Debug, Clone)]
pub struct LoaderOptions {
    pub start: PathBuf,
    pub cache_base: Option<PathBuf>,
}

impl LoaderOptions {
    pub fn new(start: PathBuf) -> Self {
        Self {
            start,
            cache_base: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    InvalidInput,
    UnreadableFile,
    FrontmatterInvalid,
    SchemaNotFound,
    SchemaInvalid,
    RuntimeUnavailable,
    TaskFailure,
}

impl ErrorKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InvalidInput => "invalid_input",
            Self::UnreadableFile => "unreadable_file",
            Self::FrontmatterInvalid => "frontmatter_invalid",
            Self::SchemaNotFound => "schema_not_found",
            Self::SchemaInvalid => "schema_invalid",
            Self::RuntimeUnavailable => "runtime_unavailable",
            Self::TaskFailure => "task_failure",
        }
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    detail: String,
}

impl Error {
    fn new(kind: ErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.detail)
    }
}
impl std::error::Error for Error {}

#[derive(Debug)]
pub struct LoadedDocument {
    schema: Schema,
    document: Document,
}

impl LoadedDocument {
    pub fn schema(&self) -> &Schema {
        &self.schema
    }
    pub fn document(&self) -> &Document {
        &self.document
    }
    pub fn validate(&self, options: ValidationOptions) -> Vec<Finding> {
        kotowari_markdown_schema::validate(&self.schema, &self.document, options)
    }
    pub fn extract_validated(
        &self,
        options: ValidationOptions,
    ) -> Result<ValidatedValues, Vec<Finding>> {
        kotowari_markdown_schema::extract_validated(&self.schema, &self.document, options)
    }
    pub fn extract_partial(&self, options: ValidationOptions) -> PartialExtraction {
        kotowari_markdown_schema::extract_partial(&self.schema, &self.document, options)
    }
    pub fn extract_typed_partial(&self, options: ValidationOptions) -> PartialExtraction {
        kotowari_markdown_schema::extract_typed_partial(&self.schema, &self.document, options)
    }
}

#[derive(Debug)]
pub struct FileResult {
    path: PathBuf,
    findings: Vec<Finding>,
}

impl FileResult {
    pub fn path(&self) -> &Path {
        &self.path
    }
    pub fn findings(&self) -> &[Finding] {
        &self.findings
    }
}

#[derive(Debug)]
pub struct CheckResult {
    files: Vec<FileResult>,
}

impl CheckResult {
    pub fn files(&self) -> &[FileResult] {
        &self.files
    }
}

#[derive(Debug, Clone)]
pub struct SchemaLoader {
    start: PathBuf,
    cache_base: PathBuf,
}

impl SchemaLoader {
    pub fn extract_validated(
        &self,
        path: &Path,
        options: ValidationOptions,
    ) -> Result<Result<ValidatedValues, Vec<Finding>>, Error> {
        Ok(self.load(path)?.extract_validated(options))
    }

    pub fn extract_partial(
        &self,
        path: &Path,
        options: ValidationOptions,
    ) -> Result<PartialExtraction, Error> {
        Ok(self.load(path)?.extract_partial(options))
    }

    pub fn new(options: LoaderOptions) -> Result<Self, Error> {
        if !options.start.is_absolute()
            || options
                .cache_base
                .as_ref()
                .is_some_and(|base| !base.is_absolute())
        {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "start and cache base must be absolute paths",
            ));
        }
        let cache_base = options
            .cache_base
            .unwrap_or_else(|| discover_base(&options.start));
        Ok(Self {
            start: options.start,
            cache_base,
        })
    }

    fn resolve(&self, path: &Path) -> PathBuf {
        self.start.join(path)
    }

    pub fn load(&self, path: &Path) -> Result<LoadedDocument, Error> {
        let absolute = self.resolve(path);
        let source = read_document(&absolute)?;
        let reference = schema_ref(path, &source)?.ok_or_else(|| {
            Error::new(
                ErrorKind::SchemaNotFound,
                format!(
                    "{}: the document has no $schema in frontmatter",
                    path.display()
                ),
            )
        })?;
        self.load_from(&absolute, path, &source, &reference)
    }

    fn load_from(
        &self,
        absolute: &Path,
        display: &Path,
        source: &str,
        reference: &SchemaRef,
    ) -> Result<LoadedDocument, Error> {
        let yaml = self.schema_yaml(absolute, reference)?;
        let schema = Schema::parse(&yaml).map_err(|e| {
            Error::new(
                ErrorKind::SchemaInvalid,
                format!("{}: {}", display.display(), e.0),
            )
        })?;
        let document = Document::parse(source).map_err(|e| {
            Error::new(
                ErrorKind::UnreadableFile,
                format!("{}: {e}", display.display()),
            )
        })?;
        Ok(LoadedDocument { schema, document })
    }

    fn schema_yaml(&self, path: &Path, reference: &SchemaRef) -> Result<String, Error> {
        match kotowari_markdown_schema::resolve_schema(path, reference) {
            ResolvedSchema::File(path) => std::fs::read_to_string(&path).map_err(|e| {
                Error::new(
                    ErrorKind::SchemaNotFound,
                    format!("cannot read schema {}: {e}", path.display()),
                )
            }),
            ResolvedSchema::Url(url) => {
                let cache = cache_path(&self.cache_base, &url);
                if let Ok(content) = std::fs::read_to_string(&cache)
                    && Schema::parse(&content).is_ok()
                {
                    return Ok(content);
                }
                let body = fetch_schema(&url, SCHEMA_FETCH_TIMEOUT)?;
                if let Some(parent) = cache.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let _ = std::fs::write(cache, &body);
                Ok(body)
            }
        }
    }

    pub fn check(&self, path: &Path, options: ValidationOptions) -> Result<CheckResult, Error> {
        let absolute = self.resolve(path);
        if !absolute.is_dir() {
            let pair = self.load(path)?;
            return Ok(CheckResult {
                files: vec![FileResult {
                    path: path.to_path_buf(),
                    findings: pair.validate(options),
                }],
            });
        }
        let mut files = Vec::new();
        let walker = walkdir::WalkDir::new(&absolute)
            .follow_links(false)
            .sort_by_file_name()
            .into_iter()
            .filter_entry(|entry| {
                entry.depth() == 0
                    || !entry.file_type().is_dir()
                    || !entry.file_name().to_string_lossy().starts_with('.')
            });
        for entry in walker {
            let entry = entry.map_err(|e| {
                Error::new(
                    ErrorKind::UnreadableFile,
                    format!("cannot walk {}: {e}", path.display()),
                )
            })?;
            if !entry.file_type().is_file()
                || entry.path().extension().and_then(|ext| ext.to_str()) != Some("md")
            {
                continue;
            }
            let display = path.join(entry.path().strip_prefix(&absolute).unwrap());
            let source = read_document(entry.path())?;
            let Some(reference) = schema_ref(&display, &source)? else {
                continue;
            };
            let pair = self.load_from(entry.path(), &display, &source, &reference)?;
            files.push(FileResult {
                path: display,
                findings: pair.validate(options),
            });
        }
        Ok(CheckResult { files })
    }
}

fn schema_ref(path: &Path, source: &str) -> Result<Option<SchemaRef>, Error> {
    kotowari_markdown_schema::frontmatter_schema(source).map_err(|e| {
        Error::new(
            ErrorKind::FrontmatterInvalid,
            format!("{}: {}", path.display(), e.0),
        )
    })
}

fn read_document(path: &Path) -> Result<String, Error> {
    let bytes = std::fs::read(path).map_err(|e| {
        Error::new(
            ErrorKind::UnreadableFile,
            format!("{}: {e}", path.display()),
        )
    })?;
    let bytes = bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(&bytes);
    String::from_utf8(bytes.to_vec()).map_err(|e| {
        Error::new(
            ErrorKind::UnreadableFile,
            format!("{}: {e}", path.display()),
        )
    })
}

fn discover_base(start: &Path) -> PathBuf {
    start
        .ancestors()
        .find(|path| path.join(".kotowari").is_dir())
        .unwrap_or(start)
        .to_path_buf()
}

fn cache_path(base: &Path, url: &str) -> PathBuf {
    use sha2::{Digest, Sha256};
    let hash = format!("{:x}", Sha256::digest(url.as_bytes()));
    base.join(".kotowari/cache/schemas")
        .join(format!("{hash}.yaml"))
}

const MAX_SCHEMA_BYTES: u64 = 4 * 1024 * 1024;
const SCHEMA_FETCH_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);

fn redact_userinfo(url: &str) -> String {
    let Some(scheme_end) = url.find("://") else {
        return url.to_string();
    };
    let start = scheme_end + 3;
    let end = url[start..]
        .find(['/', '?', '#'])
        .map(|index| start + index)
        .unwrap_or(url.len());
    let Some(at) = url[start..end].rfind('@') else {
        return url.to_string();
    };
    format!("{}***{}", &url[..start], &url[start + at..])
}

fn fetch_schema(url: &str, timeout: std::time::Duration) -> Result<String, Error> {
    let mut response = ureq::get(url)
        .config()
        .timeout_global(Some(timeout))
        .build()
        .call()
        .map_err(|e| {
            Error::new(
                ErrorKind::SchemaNotFound,
                format!("cannot fetch schema {}: {e}", redact_userinfo(url)),
            )
        })?;
    // One extra byte lets ureq observe EOF for a response exactly at the accepted limit.
    response
        .body_mut()
        .with_config()
        .limit(MAX_SCHEMA_BYTES + 1)
        .lossy_utf8(true)
        .read_to_string()
        .map_err(|e| {
            Error::new(
                ErrorKind::SchemaNotFound,
                format!("cannot read schema {}: {e}", redact_userinfo(url)),
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn url_with_userinfo(user: &str, password: &str) -> String {
        format!("https://{user}:{password}@example.com/ir.yaml")
    }

    // @kotowari[EX-schema-067, TBL-schema-003]
    #[test]
    fn ex_schema_067_a_fetch_past_the_time_limit_stops() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let (release, wait) = std::sync::mpsc::channel();
        let server = std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            wait.recv().unwrap();
            drop(stream);
        });
        let stop = fetch_schema(
            &format!("http://127.0.0.1:{port}/schema.yaml"),
            Duration::from_millis(50),
        )
        .unwrap_err();
        release.send(()).unwrap();
        server.join().unwrap();
        assert_eq!(stop.kind(), ErrorKind::SchemaNotFound, "{}", stop.detail());
        assert!(stop.detail().contains("timeout"), "{}", stop.detail());
    }

    // @kotowari[TBL-schema-003]
    #[test]
    fn the_cli_limits_a_schema_fetch_to_ten_seconds() {
        assert_eq!(SCHEMA_FETCH_TIMEOUT, Duration::from_secs(10));
    }

    // @kotowari[REQ-schema-052]
    #[test]
    fn userinfo_in_a_url_is_hidden_before_it_reaches_a_message() {
        let url = url_with_userinfo("example-user", "example-password");
        assert_eq!(redact_userinfo(&url), "https://***@example.com/ir.yaml");
        assert!(!redact_userinfo(&url).contains("example-password"));
    }

    // @kotowari[REQ-schema-052]
    #[test]
    fn a_url_without_userinfo_is_left_alone() {
        assert_eq!(
            redact_userinfo("https://example.com/ir.yaml"),
            "https://example.com/ir.yaml"
        );
    }

    // @kotowari[REQ-schema-052]
    #[test]
    fn an_at_sign_in_the_path_is_not_mistaken_for_userinfo() {
        assert_eq!(
            redact_userinfo("https://example.com/a@b/ir.yaml"),
            "https://example.com/a@b/ir.yaml"
        );
    }
}
