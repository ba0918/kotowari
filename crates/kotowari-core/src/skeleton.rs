//! `対`の`側`の`骨組み`と、その食い違い（REQ-core-344、REQ-core-345、TBL-core-044、TBL-core-045）
//!
//! `骨組み`は文書の種類ごとに TBL-core-044 の行の順に並んだ部分で、部分は要素の並びである。
//! 要素はその`側`の行を持つ。自然言語の文は要素に入れない。

use markdown::mdast::Node;
use serde_json::Value;

/// `対`にする文書の種類（TBL-core-044 の文書の列）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// `話題ごとの文書`
    Topic,
    /// `用語集`
    Glossary,
    /// `問題の記録`
    Flags,
    Guide,
    /// `全体像の元データ`
    OverviewData,
    /// `目次`
    Toc,
}

/// 部分の1つの要素。value が同じなら一致する
#[derive(Debug, Clone)]
pub struct Element {
    pub(crate) value: String,
    pub(crate) line: Option<usize>,
}

fn element(value: impl Into<String>, line: Option<usize>) -> Element {
    Element {
        value: value.into(),
        line,
    }
}

/// 名前の付いた部分の並び
pub(crate) type Skeleton = Vec<(&'static str, Vec<Element>)>;

/// 文書の種類ごとの`骨組み`
pub(crate) fn skeleton(kind: Kind, text: &str) -> Skeleton {
    match kind {
        Kind::Topic => topic(text),
        Kind::Glossary => vec![("glossary", glossary(text))],
        Kind::Flags => vec![("flag", flags(text))],
        Kind::Guide => {
            let markdown = Markdown::new(text);
            vec![
                (
                    "heading",
                    markdown.headings(|depth, _| Some(depth.to_string())),
                ),
                ("mark", marks(text)),
                ("code", markdown.code(|_| true)),
                ("table", markdown.tables(|_| String::new())),
            ]
        }
        Kind::OverviewData => {
            let markdown = Markdown::new(text);
            vec![
                ("frontmatter", markdown.frontmatter()),
                (
                    "heading",
                    markdown.headings(|depth, _| Some(depth.to_string())),
                ),
                ("part", markdown.parts()),
                ("mark", marks(text)),
                ("table", markdown.tables(|_| String::new())),
            ]
        }
        Kind::Toc => vec![("toc", toc(text))],
    }
}

/// 2つの`骨組み`を比べ、最初に一致しない部分の名前と、その部分で最初に食い違った要素の other の行。
/// 食い違いが数の違いか、要素が other に無いことなら行は None（REQ-core-345）
pub(crate) fn compare(first: &Skeleton, other: &Skeleton) -> Option<(&'static str, Option<usize>)> {
    for ((name, left), (_, right)) in first.iter().zip(other) {
        // 行の番号そのものは比べない
        let same = left.len() == right.len()
            && left
                .iter()
                .zip(right)
                .all(|(left, right)| left.value == right.value);
        if same {
            continue;
        }
        let at = left
            .iter()
            .zip(right)
            .position(|(left, right)| left.value != right.value);
        let line = at.and_then(|at| {
            // other に無い要素は、other の次の要素が first の次の要素と同じことで見分ける
            let absent = right.len() < left.len()
                && left.get(at + 1).map(|next| &next.value) == Some(&right[at].value);
            if absent { None } else { right[at].line }
        });
        return Some((name, line));
    }
    None
}

/// 2つの文書の`骨組み`を比べる
pub fn mismatch(kind: Kind, first: &str, other: &str) -> Option<(&'static str, Option<usize>)> {
    compare(&skeleton(kind, first), &skeleton(kind, other))
}

/// 文書を CommonMark と GFM として読んだもの
struct Markdown<'a> {
    text: &'a str,
    root: Option<Node>,
}

fn start_line(node: &Node) -> Option<usize> {
    node.position().map(|position| position.start.line)
}

/// 節を文書の順に集める
fn collect<'n>(node: &'n Node, keep: &impl Fn(&Node) -> bool, out: &mut Vec<&'n Node>) {
    if keep(node) {
        out.push(node);
    }
    for child in node.children().into_iter().flatten() {
        collect(child, keep, out);
    }
}

impl<'a> Markdown<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            text,
            root: crate::markdown::parse(text).ok(),
        }
    }

    fn nodes(&self, keep: impl Fn(&Node) -> bool) -> Vec<&Node> {
        let mut out = Vec::new();
        if let Some(root) = &self.root {
            collect(root, &keep, &mut out);
        }
        out
    }

    /// 節の元の文字
    fn source(&self, node: &Node) -> &'a str {
        node.position().map_or("", |position| {
            &self.text[position.start.offset..position.end.offset]
        })
    }

    /// 見出し。value は深さと見出しの元の文字から作る。None を返した見出しは要素にしない
    fn headings(&self, value: impl Fn(u8, &str) -> Option<String>) -> Vec<Element> {
        self.nodes(|node| matches!(node, Node::Heading(_)))
            .into_iter()
            .filter_map(|node| {
                let Node::Heading(heading) = node else {
                    return None;
                };
                value(heading.depth, self.source(node))
                    .map(|value| element(value, start_line(node)))
            })
            .collect()
    }

    /// コードブロックの情報文字列と中身。keep が真の言語のものだけ
    fn code(&self, keep: impl Fn(Option<&str>) -> bool) -> Vec<Element> {
        self.nodes(|node| matches!(node, Node::Code(code) if keep(code.lang.as_deref())))
            .into_iter()
            .filter_map(|node| {
                let Node::Code(code) = node else {
                    return None;
                };
                let info = [code.lang.as_deref(), code.meta.as_deref()]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>()
                    .join(" ");
                Some(element(format!("{info}\n{}", code.value), start_line(node)))
            })
            .collect()
    }

    /// 表の行ごとに、表の番号と列の数と、cells が各セルの元の文字から作る値
    fn tables(&self, cells: impl Fn(&str) -> String) -> Vec<Element> {
        let mut out = Vec::new();
        for (index, table) in self
            .nodes(|node| matches!(node, Node::Table(_)))
            .into_iter()
            .enumerate()
        {
            for row in table.children().into_iter().flatten() {
                let line = self.source(row).lines().next().unwrap_or("");
                let columns = split_row(line);
                let values: Vec<String> = columns.iter().map(|cell| cells(cell)).collect();
                out.push(element(
                    format!("{index} {} {}", columns.len(), values.join("|")),
                    start_line(row),
                ));
            }
        }
        out
    }

    /// frontmatter の中身
    fn frontmatter(&self) -> Vec<Element> {
        self.nodes(|node| matches!(node, Node::Yaml(_)))
            .into_iter()
            .filter_map(|node| match node {
                Node::Yaml(yaml) => Some(element(yaml.value.clone(), start_line(node))),
                _ => None,
            })
            .collect()
    }

    /// `部品`の種類と、値から TBL-core-045 の文の欄の文字列を除いたもの
    fn parts(&self) -> Vec<Element> {
        self.nodes(|node| matches!(node, Node::Code(code) if code.lang.as_deref() == Some("view")))
            .into_iter()
            .filter_map(|node| {
                let Node::Code(code) = node else {
                    return None;
                };
                let kind = code.meta.as_deref().unwrap_or("").trim();
                let value = match serde_saphyr::from_str::<Value>(&code.value) {
                    Ok(mut value) => {
                        without_sentences(kind, &mut value);
                        value.to_string()
                    }
                    Err(_) => code.value.clone(),
                };
                Some(element(format!("{kind} {value}"), start_line(node)))
            })
            .collect()
    }
}

/// 表の行を、先頭と末尾の "|" を除いて "|" で分けたセル。GFM のとおり、"\" に続く文字は
/// 分ける "|" にならない（"\|" はセルの中の "|"）
fn split_row(line: &str) -> Vec<&str> {
    let trimmed = line.trim();
    let trimmed = trimmed.strip_prefix('|').unwrap_or(trimmed);
    let trimmed = match trimmed.strip_suffix('|') {
        Some(rest) if !ends_with_escape(rest) => rest,
        _ => trimmed,
    };
    let mut cells = Vec::new();
    let mut start = 0;
    let mut escaped = false;
    for (index, c) in trimmed.char_indices() {
        if escaped {
            escaped = false;
        } else if c == '\\' {
            escaped = true;
        } else if c == '|' {
            cells.push(&trimmed[start..index]);
            start = index + 1;
        }
    }
    cells.push(&trimmed[start..]);
    cells
}

/// 末尾が対になっていない "\" か（そのあとの文字は逃がされる）
fn ends_with_escape(text: &str) -> bool {
    text.bytes().rev().take_while(|b| *b == b'\\').count() % 2 == 1
}

/// TBL-core-045: 部品の種類ごとの文の欄
fn sentence_fields(kind: &str) -> &'static [&'static str] {
    match kind {
        "lead" => &["conclusion", "points"],
        "flow" | "steps" => &["title", "body"],
        "cards" => &["title", "items"],
        "status" => &["text"],
        "compare" => &["before", "after", "why"],
        "decisions" => &["text", "by"],
        "quiz" => &["q", "a"],
        _ => &[],
    }
}

/// 文の欄の文字列を null にする。並びの長さと鍵は残す
fn without_sentences(kind: &str, value: &mut Value) {
    let fields = sentence_fields(kind);
    match value {
        Value::Object(map) => {
            for (key, inner) in map.iter_mut() {
                if fields.contains(&key.as_str()) {
                    blank(inner);
                } else {
                    without_sentences(kind, inner);
                }
            }
        }
        Value::Array(items) => {
            for item in items {
                without_sentences(kind, item);
            }
        }
        _ => {}
    }
}

/// 文の欄の値。文字列は null に、並びは各要素の文字列を null にする
fn blank(value: &mut Value) {
    match value {
        Value::String(_) => *value = Value::Null,
        Value::Array(items) => items.iter_mut().for_each(blank),
        _ => {}
    }
}

/// 形の正しい`ガイドの印`の中身と並び
fn marks(text: &str) -> Vec<Element> {
    crate::guides::mark_entries(text)
        .into_iter()
        .map(|entry| {
            element(
                format!("{}:{}", entry.id, entry.fingerprint),
                Some(entry.line),
            )
        })
        .collect()
}

/// コードブロックの外の行。(行番号, 行)
fn outside_code(text: &str) -> Vec<(usize, &str)> {
    let mut out = Vec::new();
    let mut fence: Option<crate::ir::CodeFence> = None;
    for (index, line) in crate::ir::split_lines(text).into_iter().enumerate() {
        match &fence {
            Some(opening) => {
                if crate::ir::is_closing_fence(line, opening) {
                    fence = None;
                }
            }
            None => match crate::ir::parse_opening_fence(line) {
                Some(opening) => fence = Some(opening),
                None => out.push((index + 1, line)),
            },
        }
    }
    out
}

/// "- 名前:" の行の値
fn field<'l>(line: &'l str, name: &str) -> Option<&'l str> {
    line.strip_prefix("- ")?
        .strip_prefix(name)?
        .strip_prefix(':')
        .map(str::trim)
}

/// "### " の見出しの`ID`（":" より前）
fn heading_id(line: &str) -> String {
    let rest = line.trim_start_matches('#').trim();
    let id = rest.split_once(':').map_or(rest, |(id, _)| id).trim();
    if crate::ir::is_valid_id(id) {
        id.to_string()
    } else {
        String::new()
    }
}

fn topic(text: &str) -> Skeleton {
    let markdown = Markdown::new(text);
    let headings = markdown.headings(|depth, source| match depth {
        2 => Some("##".to_string()),
        3 => Some(format!("### {}", heading_id(source))),
        _ => None,
    });
    let mut fields = Vec::new();
    for (line, text) in outside_code(text) {
        for name in [
            "kind",
            "source",
            "verification",
            "definition",
            "deferred",
            "related",
        ] {
            if let Some(value) = field(text, name) {
                fields.push(element(format!("{name}: {value}"), Some(line)));
            }
        }
        if field(text, "how_to_verify").is_some() {
            fields.push(element("how_to_verify", Some(line)));
        }
    }
    let tables = markdown.tables(|cell| ids(cell).join(" "));
    let gherkin = markdown
        .nodes(|node| matches!(node, Node::Code(code) if code.lang.as_deref() == Some("gherkin")))
        .into_iter()
        .flat_map(|node| {
            let start = start_line(node).unwrap_or(0);
            let Node::Code(code) = node else {
                return Vec::new();
            };
            gherkin_lines(&code.value, start)
        })
        .collect();
    vec![
        ("heading", headings),
        ("field", fields),
        ("table", tables),
        ("gherkin", gherkin),
        ("code", markdown.code(|lang| lang != Some("gherkin"))),
    ]
}

/// セルの中の`ID`の並び
fn ids(cell: &str) -> Vec<&str> {
    cell.split(|c: char| !(c.is_ascii_alphanumeric() || c == '-'))
        .filter(|word| crate::ir::is_valid_id(word))
        .collect()
}

/// gherkin のコードブロックの中のタグの行と、"Scenario:" の行とステップの行の始まりの語。
/// start はフェンスの開始の行
fn gherkin_lines(value: &str, start: usize) -> Vec<Element> {
    let mut out = Vec::new();
    for (index, line) in crate::ir::split_lines(value).into_iter().enumerate() {
        let line_number = Some(start + 1 + index);
        let trimmed = line.trim();
        if trimmed.starts_with('@') {
            out.push(element(trimmed, line_number));
        } else if trimmed.starts_with("Scenario:") {
            out.push(element("Scenario:", line_number));
        } else if let Some(word) = trimmed.split_whitespace().next()
            && ["Given", "When", "Then", "And", "But"].contains(&word)
        {
            out.push(element(word, line_number));
        }
    }
    out
}

/// `用語集`の表の行ごとの出典の列
fn glossary(text: &str) -> Vec<Element> {
    let markdown = Markdown::new(text);
    let Some(table) = markdown
        .nodes(|node| matches!(node, Node::Table(_)))
        .into_iter()
        .next()
    else {
        return Vec::new();
    };
    table
        .children()
        .into_iter()
        .flatten()
        .skip(1)
        .map(|row| {
            let line = markdown.source(row).lines().next().unwrap_or("");
            let source = split_row(line).get(2).map_or("", |cell| cell.trim());
            element(source, start_line(row))
        })
        .collect()
}

/// `問題の記録`の "### FLAG-" の見出しの`ID`と、"- kind:"、"- related:"、"- source:" の値
fn flags(text: &str) -> Vec<Element> {
    let mut out = Vec::new();
    for (line, text) in outside_code(text) {
        if text.starts_with("### FLAG-") {
            out.push(element(heading_id(text), Some(line)));
        }
        for name in ["kind", "related", "source"] {
            if let Some(value) = field(text, name) {
                out.push(element(format!("{name}: {value}"), Some(line)));
            }
        }
    }
    out
}

/// `目次`の1つの項目。名前か`目次の群`
enum TocNode {
    Name(String),
    Group {
        note: bool,
        items: Vec<serde_saphyr::Spanned<TocNode>>,
    },
}

impl<'de> serde::Deserialize<'de> for TocNode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TocNode;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a page name or a group")
            }
            fn visit_str<E: serde::de::Error>(self, name: &str) -> Result<TocNode, E> {
                Ok(TocNode::Name(name.to_string()))
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<TocNode, A::Error> {
                let (mut note, mut items) = (false, Vec::new());
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "note" => {
                            note = true;
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                        "items" => items = map.next_value()?,
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(TocNode::Group { note, items })
            }
        }
        deserializer.deserialize_any(Visitor)
    }
}

/// `目次の群`の入れ子と、名前の項目の並びと、各`目次の群`の "note" の有無。行はその項目の行
fn toc(text: &str) -> Vec<Element> {
    fn walk(node: &serde_saphyr::Spanned<TocNode>, out: &mut Vec<Element>) {
        let line = usize::try_from(node.referenced.line()).ok();
        match &node.value {
            TocNode::Name(name) => out.push(element(format!("name {name}"), line)),
            TocNode::Group { note, items } => {
                out.push(element(format!("group note={note}"), line));
                for item in items {
                    walk(item, out);
                }
                out.push(element("end", None));
            }
        }
    }
    let mut out = Vec::new();
    if let Ok(root) = serde_saphyr::from_str::<serde_saphyr::Spanned<TocNode>>(text) {
        walk(&root, &mut out);
    }
    out
}
