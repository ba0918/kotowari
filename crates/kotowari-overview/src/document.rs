//! 1つの元データを、frontmatter、題名、冒頭のブロック、節と部品に分ける（REQ-core-281、TBL-core-038）

use markdown::mdast::Node;

/// 情報文字列が "view" と1つ以上の空白と種類の名前であるフェンスのコードブロック（REQ-core-282）
#[derive(Debug, Clone)]
pub(crate) struct RawPart {
    pub(crate) kind: String,
    /// フェンスの開始の行
    pub(crate) line: usize,
    pub(crate) content: String,
}

#[derive(Debug, Clone)]
pub(crate) enum RawBlock {
    Markdown(String),
    /// parts の添字
    Part(usize),
}

#[derive(Debug, Clone)]
pub(crate) struct RawSection {
    /// HTML のコメントを除いた見出しの文字（REQ-core-292）
    pub(crate) heading: String,
    pub(crate) line: usize,
    pub(crate) blocks: Vec<RawBlock>,
}

#[derive(Debug, Default)]
pub(crate) struct RawDocument {
    /// 文書の先頭の YAML の frontmatter の中身
    pub(crate) frontmatter: Option<String>,
    pub(crate) title: Option<String>,
    /// 題名の後で最初の、HTML のコメントでも空行でもないブロックが lead の部品なら、その添字（REQ-core-283）
    pub(crate) lead: Option<usize>,
    /// 題名の後、最初の "## " より前の、lead でない部品の添字（REQ-view-006）
    pub(crate) preamble: Vec<usize>,
    pub(crate) parts: Vec<RawPart>,
    pub(crate) sections: Vec<RawSection>,
}

/// kotowari-markdown-schema と同じ読み方（GFM と frontmatter）
fn options() -> markdown::ParseOptions {
    let mut constructs = markdown::Constructs::gfm();
    constructs.frontmatter = true;
    markdown::ParseOptions {
        constructs,
        ..markdown::ParseOptions::default()
    }
}

/// フェンスが部品なら種類の名前を返す
fn part_kind(node: &Node) -> Option<(String, String)> {
    let Node::Code(code) = node else {
        return None;
    };
    if code.lang.as_deref() != Some("view") {
        return None;
    }
    let kind = code.meta.as_deref().map(str::trim).unwrap_or("");
    (!kind.is_empty()).then(|| (kind.to_string(), code.value.clone()))
}

/// HTML のコメントだけの節か
fn is_comment(node: &Node) -> bool {
    let Node::Html(html) = node else {
        return false;
    };
    let mut rest = html.value.trim();
    while let Some(after) = rest.strip_prefix("<!--") {
        let Some(close) = after.find("-->") else {
            return false;
        };
        rest = after[close + 3..].trim_start();
    }
    rest.is_empty()
}

/// 見出しの文字。HTML のコメント（行の中の HTML）を除く
fn plain_text(node: &Node, out: &mut String) {
    match node {
        Node::Html(_) => {}
        Node::Text(text) => out.push_str(&text.value),
        Node::InlineCode(code) => out.push_str(&code.value),
        other => {
            for child in other.children().into_iter().flatten() {
                plain_text(child, out);
            }
        }
    }
}

fn heading_text(node: &Node) -> String {
    let mut text = String::new();
    plain_text(node, &mut text);
    text.trim().to_string()
}

fn line(node: &Node) -> usize {
    node.position().map_or(1, |position| position.start.line)
}

/// 節の中の部品でない連続した節を、元の文字のまま1つの Markdown の塊にする
struct Pending {
    range: Option<(usize, usize)>,
}

impl Pending {
    fn push(&mut self, node: &Node) {
        if let Some(position) = node.position() {
            let (start, end) = (position.start.offset, position.end.offset);
            self.range = Some(match self.range {
                Some((first, _)) => (first, end),
                None => (start, end),
            });
        }
    }
    fn flush(&mut self, text: &str, blocks: &mut Vec<RawBlock>) {
        if let Some((start, end)) = self.range.take() {
            blocks.push(RawBlock::Markdown(text[start..end].to_string()));
        }
    }
}

pub(crate) fn parse(text: &str) -> RawDocument {
    let mut document = RawDocument::default();
    // GFM と frontmatter の読み方は MDX の構文を持たないので誤りを返さない
    let Ok(Node::Root(root)) = markdown::to_mdast(text, &options()) else {
        return document;
    };
    // lead を探すのは題名の後だけ。題名の無い文書は lead を持たない
    let mut looking_for_lead = false;
    let mut pending = Pending { range: None };
    for node in &root.children {
        // frontmatter の読み方は文書の先頭の1つだけを YAML の節にする
        if let Node::Yaml(yaml) = node {
            document.frontmatter = Some(yaml.value.clone());
            continue;
        }
        if let Node::Heading(heading) = node {
            if heading.depth == 1 && document.title.is_none() && document.sections.is_empty() {
                document.title = Some(heading_text(node));
                looking_for_lead = true;
                continue;
            }
            if heading.depth == 2 {
                looking_for_lead = false;
                if let Some(section) = document.sections.last_mut() {
                    pending.flush(text, &mut section.blocks);
                }
                document.sections.push(RawSection {
                    heading: heading_text(node),
                    line: line(node),
                    blocks: Vec::new(),
                });
                continue;
            }
        }
        let part = part_kind(node).map(|(kind, content)| {
            document.parts.push(RawPart {
                kind,
                line: line(node),
                content,
            });
            document.parts.len() - 1
        });
        if looking_for_lead && !is_comment(node) {
            looking_for_lead = false;
            document.lead = part.filter(|index| document.parts[*index].kind == "lead");
        }
        // 題名の後、最初の "## " より前の lead でない部品は、lead に続けて描く
        let Some(section) = document.sections.last_mut() else {
            if let Some(index) = part
                && document.title.is_some()
                && document.lead != Some(index)
            {
                document.preamble.push(index);
            }
            continue;
        };
        match part {
            Some(index) => {
                pending.flush(text, &mut section.blocks);
                section.blocks.push(RawBlock::Part(index));
            }
            None => pending.push(node),
        }
    }
    if let Some(section) = document.sections.last_mut() {
        pending.flush(text, &mut section.blocks);
    }
    document
}
