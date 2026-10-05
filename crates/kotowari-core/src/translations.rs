//! `対`の見分け方、`一致の記録`、`対`の`指摘`（docs/ir/core/translation-pairs.md、
//! docs/ir/core/translation-structure.md）
//!
//! ファイルを探して読むのは呼び出し側で、ここは名前と読んだ中身だけから決める。

use crate::{Finding, FindingKind};
use sha1::{Digest, Sha1};
use std::sync::Arc;

/// `一致の記録`の名前の、拡張子の前の部分（REQ-core-337、REQ-core-339）
const RECORD: &str = ".i18n.yaml";

/// REQ-core-340: "blob "、バイト数の10進、1バイトの 0、バイト列をつないだものの SHA-1 を16進の小文字で
pub fn blob_hash(bytes: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(format!("blob {}\0", bytes.len()).as_bytes());
    hasher.update(bytes);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// ファイル名を、最後の "." の前と、その "." からの拡張子に分ける（REQ-core-337）
fn split_extension(name: &str) -> (&str, &str) {
    match name.rfind('.') {
        Some(dot) => (&name[..dot], &name[dot..]),
        None => (name, ""),
    }
}

/// `対`にする置き場で読んだファイルの名前の見分け（REQ-core-337）
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Classified {
    /// `先頭の言語`の`側`
    First,
    /// `先頭の言語`でない言語の`側`。first はその`対`の`先頭の言語`の`側`のファイル名
    Side { language: String, first: String },
    /// "<幹>.i18n.yaml"。`側`として読まない
    Record,
}

/// ファイル名（ディレクトリを除く）を`言語の一覧`で見分ける（REQ-core-337）
pub fn classify(name: &str, languages: &[String]) -> Classified {
    if name.ends_with(RECORD) {
        return Classified::Record;
    }
    let (rest, extension) = split_extension(name);
    if let Some((stem, tag)) = rest.rsplit_once('.')
        && languages.iter().skip(1).any(|language| language == tag)
    {
        return Classified::Side {
            language: tag.to_string(),
            first: format!("{stem}{extension}"),
        };
    }
    Classified::First
}

/// `先頭の言語`の`側`のパスから、その言語の`側`のパス。`先頭の言語`ならそのまま
pub fn side_path(first: &str, language: &str, languages: &[String]) -> String {
    if languages.first().map(String::as_str) == Some(language) {
        return first.to_string();
    }
    let (directory, name) = split_directory(first);
    let (stem, extension) = split_extension(name);
    format!("{directory}{stem}.{language}{extension}")
}

/// `先頭の言語`の`側`のパスから、`一致の記録`のパス（REQ-core-339）
pub fn record_path(first: &str) -> String {
    let (directory, name) = split_directory(first);
    let (stem, _) = split_extension(name);
    format!("{directory}{stem}{RECORD}")
}

/// パスを、最後の "/" までのディレクトリとファイル名に分ける
fn split_directory(path: &str) -> (&str, &str) {
    match path.rfind('/') {
        Some(slash) => (&path[..=slash], &path[slash + 1..]),
        None => ("", path),
    }
}

/// ファイル名（ディレクトリを除く）
fn file_name(path: &str) -> &str {
    split_directory(path).1
}

/// `対`を読んだ置き場
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Place {
    Ir,
    Guide,
    OverviewData,
    Toc,
}

/// ある`側`の読んだ中身
#[derive(Debug, Clone)]
pub struct SideText {
    blob: String,
    text: Arc<str>,
}

impl SideText {
    /// bytes はファイルのバイト列そのもの、text はそれを読んだ文字（BOM を除いたもの）
    pub fn new(bytes: &[u8], text: impl Into<String>) -> Self {
        Self {
            blob: blob_hash(bytes),
            text: Arc::from(text.into()),
        }
    }
    pub fn blob(&self) -> &str {
        &self.blob
    }
    pub fn text(&self) -> &str {
        &self.text
    }
}

/// `対`の1つの言語の`側`
#[derive(Debug, Clone)]
pub struct Side {
    language: String,
    path: String,
    content: Option<SideText>,
}

impl Side {
    /// path は`基準のディレクトリ`からの相対パス。ファイルが無ければ content は None
    pub fn new(
        language: impl Into<String>,
        path: impl Into<String>,
        content: Option<SideText>,
    ) -> Self {
        Self {
            language: language.into(),
            path: path.into(),
            content,
        }
    }
    pub fn language(&self) -> &str {
        &self.language
    }
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn content(&self) -> Option<&SideText> {
        self.content.as_ref()
    }
}

/// 1つの`対`。sides は`言語の一覧`の順で、最初が`先頭の言語`の`側`
#[derive(Debug, Clone)]
pub struct Pair {
    place: Place,
    sides: Vec<Side>,
    /// `一致の記録`のバイト列。無ければ None
    record: Option<Vec<u8>>,
}

impl Pair {
    /// first は`先頭の言語`の`側`の`基準のディレクトリ`からの相対パス。read はパスからそのファイルの
    /// 中身を返し、ファイルが無ければ None を返す
    pub fn read<E>(
        place: Place,
        first: &str,
        languages: &[String],
        mut read: impl FnMut(&str) -> Result<Option<SideText>, E>,
        record: Option<Vec<u8>>,
    ) -> Result<Self, E> {
        let sides = languages
            .iter()
            .map(|language| {
                let path = side_path(first, language, languages);
                let content = read(&path)?;
                Ok(Side::new(language.as_str(), path, content))
            })
            .collect::<Result<_, E>>()?;
        Ok(Self {
            place,
            sides,
            record,
        })
    }
    pub fn place(&self) -> Place {
        self.place
    }
    /// `先頭の言語`の`側`のパス
    pub fn path(&self) -> &str {
        &self.sides[0].path
    }
    pub fn sides(&self) -> &[Side] {
        &self.sides
    }
    /// `先頭の言語`の`側`
    pub fn first(&self) -> &Side {
        &self.sides[0]
    }
    /// `先頭の言語`でない言語の`側`
    pub fn others(&self) -> &[Side] {
        &self.sides[1..]
    }
}

/// `一致の記録`を読んだもの。誤りなら TBL-core-008 の detail
fn read_record(pair: &Pair) -> Result<std::collections::BTreeMap<String, String>, &'static str> {
    let Some(bytes) = &pair.record else {
        return Err("missing");
    };
    let text = std::str::from_utf8(bytes).map_err(|_| "yaml")?;
    let value: serde_json::Value = if text.trim().is_empty() {
        serde_json::Value::Null
    } else {
        serde_saphyr::from_str(crate::strip_bom(text)).map_err(|_| "yaml")?
    };
    let serde_json::Value::Object(map) = value else {
        return Err("keys");
    };
    let mut record = std::collections::BTreeMap::new();
    for (key, value) in map {
        let serde_json::Value::String(value) = value else {
            return Err("keys");
        };
        record.insert(key, value);
    }
    let names: std::collections::BTreeSet<&str> = pair
        .sides
        .iter()
        .map(|side| file_name(&side.path))
        .collect();
    if !record.keys().map(String::as_str).eq(names.iter().copied()) {
        return Err("keys");
    }
    let hex = |value: &String| {
        value.len() == 40
            && value
                .bytes()
                .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
    };
    if !record.values().all(hex) {
        return Err("value");
    }
    Ok(record)
}

/// 欠けた側、`一致の記録`、古い側の`指摘`（REQ-core-338、REQ-core-339、REQ-core-341）
fn pair_findings(pair: &Pair, context: &Context, findings: &mut Vec<Finding>) {
    let missing = |path: &str, detail: &str| {
        Finding::new(
            FindingKind::TranslationMissing,
            path.to_string(),
            None,
            detail.to_string(),
        )
    };
    let first = pair.first();
    if first.content.is_none() {
        for side in pair.others().iter().filter(|side| side.content.is_some()) {
            findings.push(missing(&side.path, &first.path));
        }
        return;
    }
    for side in pair.others().iter().filter(|side| side.content.is_none()) {
        findings.push(missing(&first.path, &side.path));
    }
    structure_findings(pair, context, findings);
    match read_record(pair) {
        Err(detail) => findings.push(Finding::new(
            FindingKind::TranslationRecordInvalid,
            record_path(&first.path),
            None,
            detail.to_string(),
        )),
        Ok(record) => {
            for side in &pair.sides {
                let Some(content) = &side.content else {
                    continue;
                };
                let recorded = &record[file_name(&side.path)];
                if *recorded != content.blob {
                    findings.push(Finding::new(
                        FindingKind::TranslationStale,
                        side.path.clone(),
                        None,
                        format!("{recorded} {}", content.blob),
                    ));
                }
            }
        }
    }
}

/// `対`の文書の種類（TBL-core-044）
fn skeleton_kind(pair: &Pair) -> crate::skeleton::Kind {
    use crate::skeleton::Kind;
    match pair.place {
        Place::Ir => match crate::DocKind::of(file_name(pair.path())) {
            crate::DocKind::Topic => Kind::Topic,
            crate::DocKind::Glossary => Kind::Glossary,
            crate::DocKind::Flags => Kind::Flags,
        },
        Place::Guide => Kind::Guide,
        Place::OverviewData => Kind::OverviewData,
        Place::Toc => Kind::Toc,
    }
}

/// 検査に要る設定と、すべての`対`の`側`のパスからその言語と`先頭の言語`の`側`のパスへの対応
pub(crate) struct Context {
    languages: Vec<String>,
    /// `言語の一覧`の順の、各言語の`UI の文字`の "language_name"
    names: Vec<String>,
    records: String,
    adr: String,
    sides: std::collections::BTreeMap<String, (String, String)>,
}

impl Context {
    pub(crate) fn new(pairs: &Pairs, config: &crate::config::Config) -> Self {
        let languages = config.languages();
        let names = languages
            .iter()
            .map(|language| {
                config
                    .ui_text(language)
                    .get("language_name")
                    .unwrap_or("")
                    .to_string()
            })
            .collect();
        let sides = pairs
            .iter()
            .flat_map(|pair| {
                pair.sides.iter().map(|side| {
                    (
                        side.path.clone(),
                        (side.language.clone(), pair.path().to_string()),
                    )
                })
            })
            .collect();
        Self {
            languages,
            names,
            records: config.decisions.records.clone(),
            adr: config.decisions.adr.clone(),
            sides,
        }
    }

    /// その`側`のあるべき`切り替えの行`（REQ-core-346）
    pub(crate) fn switcher(&self, first: &str, language: &str) -> String {
        self.languages
            .iter()
            .zip(&self.names)
            .map(|(other, name)| {
                if other == language {
                    name.clone()
                } else {
                    let target = side_path(first, other, &self.languages);
                    format!("[{name}]({})", file_name(&target))
                }
            })
            .collect::<Vec<_>>()
            .join(" | ")
    }
}

/// `題名`の行と、`題名`の後の最初の空でない行（`題名`が無ければファイルの最初の空でない行）
fn switcher_candidate(text: &str) -> (Option<usize>, Option<(usize, &str)>) {
    let lines = crate::ir::split_lines(text);
    let title = lines.iter().position(|line| line.starts_with("# "));
    let from = title.map_or(0, |title| title + 1);
    let candidate = lines
        .iter()
        .enumerate()
        .skip(from)
        .find(|(_, line)| !line.trim().is_empty())
        .map(|(index, line)| (index + 1, *line));
    (title.map(|title| title + 1), candidate)
}

/// `切り替えの行`と同じ行を空にした文。行の番号は変わらない。言語が1つなら文のまま（REQ-core-346）
pub(crate) fn without_switcher(
    config: &crate::config::Config,
    first: &str,
    language: &str,
    text: &str,
) -> Option<String> {
    if config.languages().len() < 2 {
        return None;
    }
    let expected = Context::new(&Pairs::default(), config).switcher(first, language);
    switcher_line(text, &expected).map(|line| blank_line(text, line))
}

/// あるべき`切り替えの行`と同じ行。無ければ None
pub(crate) fn switcher_line(text: &str, expected: &str) -> Option<usize> {
    match switcher_candidate(text).1 {
        Some((line, found)) if found == expected => Some(line),
        _ => None,
    }
}

/// その行を空にした文。行の番号は変わらない
pub(crate) fn blank_line(text: &str, line: usize) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    let mut number = 1;
    loop {
        let end = rest.find(['\n', '\r']).unwrap_or(rest.len());
        if number != line {
            out.push_str(&rest[..end]);
        }
        let ending = if rest[end..].starts_with("\r\n") {
            2
        } else {
            (end < rest.len()) as usize
        };
        out.push_str(&rest[end..end + ending]);
        rest = &rest[end + ending..];
        number += 1;
        if rest.is_empty() {
            return out;
        }
    }
}

/// `切り替えの行`を持つ`対`か（REQ-core-346、`全体像の元データ`と`目次`は持たない）
fn has_switcher(place: Place) -> bool {
    matches!(place, Place::Ir | Place::Guide)
}

/// リンクを検査する`対`か（REQ-core-347）
fn has_links(place: Place) -> bool {
    !matches!(place, Place::Toc)
}

/// 1つの`側`を検査し、`切り替えの行`を空にした文を返す（REQ-core-346〜REQ-core-349）
fn side_text(pair: &Pair, side: &Side, context: &Context, findings: &mut Vec<Finding>) -> String {
    let text = side.content.as_ref().map_or("", |content| content.text());
    let mut checked = text.to_string();
    if has_switcher(pair.place) {
        let expected = context.switcher(pair.path(), &side.language);
        match switcher_line(text, &expected) {
            Some(line) => checked = blank_line(text, line),
            None => {
                let (title, candidate) = switcher_candidate(text);
                findings.push(Finding::new(
                    FindingKind::TranslationSwitcherInvalid,
                    side.path.clone(),
                    candidate.map(|(line, _)| line).or(title),
                    expected,
                ));
            }
        }
    }
    if has_links(pair.place) {
        for link in links(&checked) {
            let Some(target) = target(&side.path, &link.destination) else {
                continue;
            };
            if let Some((language, _)) = context.sides.get(&target)
                && *language != side.language
            {
                findings.push(Finding::new(
                    FindingKind::LinkLanguageMismatch,
                    side.path.clone(),
                    Some(link.line),
                    link.destination.clone(),
                ));
            }
            let record = |place: &str| crate::sources::is_under_place(&target, place);
            if record(&context.records) || record(&context.adr) {
                findings.push(Finding::new(
                    FindingKind::LinkToRecord,
                    side.path.clone(),
                    Some(link.line),
                    link.destination,
                ));
            }
        }
    }
    checked
}

/// CommonMark のリンクと画像の行き先と、リンクの参照の定義の行き先
struct Link {
    destination: String,
    line: usize,
}

fn links(text: &str) -> Vec<Link> {
    fn walk(node: &markdown::mdast::Node, out: &mut Vec<Link>) {
        use markdown::mdast::Node;
        let destination = match node {
            Node::Link(link) => Some(&link.url),
            Node::Image(image) => Some(&image.url),
            Node::Definition(definition) => Some(&definition.url),
            _ => None,
        };
        if let (Some(destination), Some(position)) = (destination, node.position()) {
            out.push(Link {
                destination: destination.clone(),
                line: position.start.line,
            });
        }
        for child in node.children().into_iter().flatten() {
            walk(child, out);
        }
    }
    let mut out = Vec::new();
    if let Ok(root) = crate::markdown::parse(text) {
        walk(&root, &mut out);
    }
    out
}

/// 検査するリンクの行き先を、その`側`のあるディレクトリから辿った`基準のディレクトリ`からの相対パスに
/// する。スキームか "#" で始まる行き先は検査しないので None（REQ-core-347）
fn target(side: &str, destination: &str) -> Option<String> {
    let mut chars = destination.chars();
    let scheme = chars.next().is_some_and(|c| c.is_ascii_alphabetic())
        && chars
            .take_while(|c| *c != ':')
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '.' | '-'))
        && destination.contains(':');
    if scheme || destination.starts_with('#') {
        return None;
    }
    let path = destination.split(['#', '?']).next().unwrap_or(destination);
    let (directory, _) = split_directory(side);
    Some(crate::normalize_path(&format!("{directory}{path}")))
}

/// `骨組み`の link の部分。ほかの言語の`側`を指す行き先はその`対`の`先頭の言語`の`側`に読み替える
/// （TBL-core-044）
fn link_part(path: &str, text: &str, context: &Context) -> Vec<crate::skeleton::Element> {
    links(text)
        .into_iter()
        .filter_map(|link| {
            let target = target(path, &link.destination)?;
            let target = context
                .sides
                .get(&target)
                .map_or(target, |(_, first)| first.clone());
            Some(crate::skeleton::Element {
                value: target,
                line: Some(link.line),
            })
        })
        .collect()
}

/// REQ-core-345: ほかの言語の`側`の`骨組み`が`先頭の言語`の`側`と食い違えば、`側`ごとに1件
fn structure_findings(pair: &Pair, context: &Context, findings: &mut Vec<Finding>) {
    let kind = skeleton_kind(pair);
    let skeleton = |side: &Side, findings: &mut Vec<Finding>| {
        let text = side_text(pair, side, context, findings);
        let mut skeleton = crate::skeleton::skeleton(kind, &text);
        if matches!(pair.place, Place::Guide | Place::OverviewData) {
            skeleton.push(("link", link_part(&side.path, &text, context)));
        }
        skeleton
    };
    let expected = skeleton(pair.first(), findings);
    for side in pair.others() {
        if side.content.is_none() {
            continue;
        }
        let actual = skeleton(side, findings);
        if let Some((part, line)) = crate::skeleton::compare(&expected, &actual) {
            findings.push(Finding::new(
                FindingKind::TranslationStructureMismatch,
                side.path.clone(),
                line,
                part.to_string(),
            ));
        }
    }
}

/// 読んだ`対`の並び。同じ`先頭の言語`の`側`の`対`は1つにする（REQ-core-337）
#[derive(Debug, Clone, Default)]
pub struct Pairs(std::collections::BTreeMap<String, Pair>);

impl Pairs {
    /// 加える。同じパスの`対`が既にあれば加えない
    pub fn insert(&mut self, pair: Pair) {
        self.0.entry(pair.path().to_string()).or_insert(pair);
    }
    pub fn contains(&self, first: &str) -> bool {
        self.0.contains_key(first)
    }
    pub fn get(&self, first: &str) -> Option<&Pair> {
        self.0.get(first)
    }
    /// `先頭の言語`の`側`のパスのバイト順
    pub fn iter(&self) -> impl Iterator<Item = &Pair> {
        self.0.values()
    }
    pub fn extend(&mut self, other: Pairs) {
        for pair in other.0.into_values() {
            self.insert(pair);
        }
    }
    /// すべての`対`の`指摘`
    pub(crate) fn findings(&self, config: &crate::config::Config) -> Vec<Finding> {
        let context = Context::new(self, config);
        let mut findings = Vec::new();
        for pair in self.iter() {
            pair_findings(pair, &context, &mut findings);
        }
        findings
    }
}

/// `基準のディレクトリ`からの相対パスを、`IR`の置き場からの相対パスにする
fn ir_relative(place: &str, path: &str) -> String {
    if place.is_empty() {
        return path.to_string();
    }
    path.strip_prefix(place)
        .and_then(|rest| rest.strip_prefix('/'))
        .unwrap_or(path)
        .to_string()
}

/// `先頭の言語`でない言語ごとの、`先頭の言語`の`側`のある`IR`の`対`のその言語の`側`の文書。
/// 文書の種類は`先頭の言語`の`側`の名前で決まり（REQ-core-033）、文書はパスのバイト順に並ぶ
pub(crate) fn ir_sides(
    pairs: &Pairs,
    config: &crate::config::Config,
) -> Result<std::collections::BTreeMap<String, Vec<crate::ir::IrDocument>>, crate::StopReason> {
    let place = config.ir.as_str();
    let mut sides: std::collections::BTreeMap<String, Vec<crate::ir::IrDocument>> =
        std::collections::BTreeMap::new();
    for pair in pairs.iter().filter(|pair| pair.place == Place::Ir) {
        if pair.first().content.is_none() {
            continue;
        }
        let kind = crate::DocKind::of(file_name(pair.path()));
        for side in pair.others() {
            let Some(content) = &side.content else {
                continue;
            };
            let path = ir_relative(place, &side.path);
            let text = without_switcher(config, pair.path(), &side.language, content.text());
            let text = text.as_deref().unwrap_or(content.text());
            let document = crate::ir::parse_side(&path, kind, text)?;
            sides
                .entry(side.language.clone())
                .or_default()
                .push(document);
        }
    }
    for documents in sides.values_mut() {
        documents.sort_by(|left, right| {
            left.relative_path
                .as_bytes()
                .cmp(right.relative_path.as_bytes())
        });
    }
    Ok(sides)
}

/// REQ-core-342: ほかの言語の`側`に行う検査。`用語`（その言語の`用語集`の`連鎖`から引く）、`曖昧語`、
/// `文書名の参照`、閉じないバッククォート、`用語集`の形
pub(crate) fn side_findings(
    sides: &[crate::ir::IrDocument],
    firsts: &[crate::ir::IrDocument],
    config: &crate::config::Config,
    findings: &mut Vec<Finding>,
) {
    let duplicates = crate::ir::GlossaryDuplicates::new(sides);
    let known_ids = crate::collect_known_ids(firsts);
    crate::terms::check_terms_and_vague_words_with_duplicates(
        sides,
        &known_ids,
        &config.vague_words,
        &config.ir,
        &duplicates,
        findings,
    );
    let paths = firsts
        .iter()
        .map(|document| document.relative_path.clone())
        .collect();
    crate::terms::check_document_references(sides, &config.ir, &paths, findings);
    for (document, rows) in sides.iter().zip(duplicates.rows()) {
        if document.kind != crate::DocKind::Glossary {
            continue;
        }
        let path = crate::join_display_path(&config.ir, &document.relative_path);
        for finding in &document.parse_findings {
            if matches!(
                finding.kind,
                FindingKind::GlossaryInvalid
                    | FindingKind::InvalidGlossaryRow
                    | FindingKind::GlossaryTitleInvalid
                    | FindingKind::DuplicateTerm
            ) {
                let mut finding = finding.clone();
                if finding.path.is_empty() {
                    finding.path = path.clone();
                }
                findings.push(finding);
            }
        }
        for item in &document.items {
            if let crate::ir::Item::GlossaryTerm { term, line, .. } = item
                && rows.contains(line)
            {
                findings.push(Finding::new(
                    FindingKind::DuplicateTerm,
                    path.clone(),
                    Some(*line),
                    term.clone(),
                ));
            }
        }
    }
}
