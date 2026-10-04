use crate::{Error, ErrorKind};
use std::{num::NonZeroUsize, sync::Arc};

#[derive(Debug, Clone, Copy)]
pub struct AsyncOptions {
    pub max_concurrency: NonZeroUsize,
}
impl Default for AsyncOptions {
    fn default() -> Self {
        Self {
            max_concurrency: NonZeroUsize::new(1).unwrap(),
        }
    }
}

#[derive(Clone)]
struct Blocking {
    slots: Arc<tokio::sync::Semaphore>,
}
impl Blocking {
    fn new(options: AsyncOptions) -> Self {
        Self {
            slots: Arc::new(tokio::sync::Semaphore::new(options.max_concurrency.get())),
        }
    }
    async fn run<T: Send + 'static>(
        &self,
        work: impl FnOnce() -> Result<T, Error> + Send + 'static,
    ) -> Result<T, Error> {
        let runtime = tokio::runtime::Handle::try_current()
            .map_err(|error| Error::new(ErrorKind::RuntimeUnavailable, error.to_string()))?;
        let permit = self
            .slots
            .clone()
            .acquire_owned()
            .await
            .map_err(|error| Error::new(ErrorKind::TaskFailure, error.to_string()))?;
        runtime
            .spawn_blocking(move || {
                // A detached worker must still own its slot until the synchronous operation finishes.
                let _permit = permit;
                work()
            })
            .await
            .map_err(|error| Error::new(ErrorKind::TaskFailure, error.to_string()))?
    }
}

#[derive(Clone)]
pub struct AsyncSchemaLoader {
    loader: crate::SchemaLoader,
    blocking: Blocking,
}
impl AsyncSchemaLoader {
    pub fn new(options: crate::LoaderOptions, asynchronous: AsyncOptions) -> Result<Self, Error> {
        Ok(Self {
            loader: crate::SchemaLoader::new(options)?,
            blocking: Blocking::new(asynchronous),
        })
    }
    pub async fn load(&self, path: &std::path::Path) -> Result<crate::LoadedDocument, Error> {
        let loader = self.loader.clone();
        let path = path.to_path_buf();
        self.blocking.run(move || loader.load(&path)).await
    }
    pub async fn check(
        &self,
        path: &std::path::Path,
        options: kotowari_markdown_schema::ValidationOptions,
    ) -> Result<crate::CheckResult, Error> {
        let loader = self.loader.clone();
        let path = path.to_path_buf();
        self.blocking
            .run(move || loader.check(&path, options))
            .await
    }
    pub async fn extract_validated(
        &self,
        path: &std::path::Path,
        options: kotowari_markdown_schema::ValidationOptions,
    ) -> Result<
        Result<
            kotowari_markdown_schema::ValidatedValues,
            Vec<kotowari_markdown_schema::finding::Finding>,
        >,
        Error,
    > {
        let loader = self.loader.clone();
        let path = path.to_path_buf();
        self.blocking
            .run(move || loader.extract_validated(&path, options))
            .await
    }
    pub async fn extract_partial(
        &self,
        path: &std::path::Path,
        options: kotowari_markdown_schema::ValidationOptions,
    ) -> Result<kotowari_markdown_schema::PartialExtraction, Error> {
        let loader = self.loader.clone();
        let path = path.to_path_buf();
        self.blocking
            .run(move || loader.extract_partial(&path, options))
            .await
    }
}
#[cfg(test)]
#[path = "async_tests.rs"]
mod tests;
