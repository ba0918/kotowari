//! mdast から正規化した文書の木を作る。検証と抽出の共通の入力。

use crate::ast::parse_mdast;
use markdown::mdast::Node;

/// 検証・抽出の対象にする文書の木。
///
/// Parsed document data cannot be changed independently of its source.
/// ```compile_fail
/// let mut document = kotowari_markdown_schema::Document::parse("# Original\n").unwrap();
/// document.lines.clear();
/// ```
#[derive(Debug, Default)]
pub struct Document {
    /// 深さ1の見出し（題名）
    pub(crate) titles: Vec<Heading>,
    /// 最初の節より前のブロック
    pub(crate) preamble: Vec<Block>,
    /// 深さ2の節
    pub(crate) sections: Vec<Section>,
    /// 節の外に出た深さ3以上の見出し（深さ4以上は heading_level_mismatch の対象）
    pub(crate) stray_headings: Vec<Heading>,
    /// 前置部領域（最初の節より前）に出た深さ3の見出しとその内側の行。
    /// 題名より後の見出しは宣言済みの前置部の中の未宣言の構造として、open でも
    /// undeclared_heading / undeclared_line の対象になる（REQ-schema-003）。題名より前の
    /// 見出しは open では許す
    pub(crate) stray_preamble_headings: Vec<StrayPreambleHeading>,
    /// 文書の生の行。1始まりの行番号で `raw_line` から引く。指摘が指す行の
    /// 生の文字は組み立て直さずここから取る（REQ-schema-008）
    pub(crate) lines: Vec<String>,
    /// 文書の先頭の frontmatter が占める行数。無ければ 0。行の読み方はこの次の行から読む
    pub(crate) frontmatter_lines: usize,
}

impl Document {
    /// Original line of the first title, when present.
    pub fn title_line(&self) -> Option<usize> {
        self.titles.first().map(|title| title.line)
    }
    /// 1始まりの行番号の生の行。字下げと末尾の空白を含む。範囲の外は None。
    pub fn raw_line(&self, line: usize) -> Option<&str> {
        self.lines
            .get(line.checked_sub(1)?)
            .map(std::string::String::as_str)
    }
}

impl Document {
    /// 最初の節より前の深さ3の見出しを、文書の直下の`項目`として読んだもの（REQ-schema-061）。
    /// スキーマが文書の直下の項目を宣言したときだけ使う。
    pub(crate) fn preamble_items(&self) -> Vec<Item> {
        self.stray_preamble_headings
            .iter()
            .map(|stray| {
                let (id, title, has_id_separator) = split_item_heading(&stray.heading.text);
                Item {
                    id,
                    title,
                    has_id_separator,
                    line: stray.heading.line,
                    blocks: stray.blocks.clone(),
                }
            })
            .collect()
    }
}

impl Document {
    /// 行 `line` の深さ `depth` の見出しに始まる要素の最後の行（REQ-schema-062）。次に現れる
    /// 同じ深さかそれより浅い見出しの手前の行で、無ければ文書の最後の行。見出しは読み方ごとに
    /// 組んだ木の見出しなので、コードブロックの中の見出しの形の行は数えない（TBL-schema-011）。
    pub(crate) fn end_line(&self, line: usize, depth: u8) -> usize {
        let titles = self.titles.iter().map(|h| (h.line, h.depth));
        let sections = self
            .sections
            .iter()
            .flat_map(|s| std::iter::once((s.line, 2)).chain(s.items.iter().map(|i| (i.line, 3))));
        let strays = self
            .stray_preamble_headings
            .iter()
            .map(|s| (s.heading.line, s.heading.depth))
            .chain(self.stray_headings.iter().map(|h| (h.line, h.depth)));
        titles
            .chain(sections)
            .chain(strays)
            .filter(|&(l, d)| l > line && d <= depth)
            .map(|(l, _)| l - 1)
            .min()
            .unwrap_or_else(|| self.last_line())
    }

    /// 文書の最後の行の行番号。文書が行の区切りで終わるとき、その後ろの空の行は数えない。
    fn last_line(&self) -> usize {
        let ends_with_break = self.lines.last().is_some_and(String::is_empty);
        self.lines.len() - usize::from(ends_with_break)
    }
}

/// 前置部領域に出た深さ3の見出しと、その下に続く内側の行。
#[derive(Debug)]
pub struct StrayPreambleHeading {
    pub heading: Heading,
    pub blocks: Vec<Block>,
    /// 題名より前に出たか。題名より前の見出しは open では許し、前置部領域の
    /// 見出しは宣言済みの前置部の内側の未宣言の構造として open でも誤りになる（REQ-schema-003）
    pub before_title: bool,
}

#[derive(Debug)]
pub struct Heading {
    pub text: String,
    pub depth: u8,
    pub line: usize,
}

/// 文の1行。生の行とその行番号（1始まり）。行ごとの抽出と、指摘が指す行の
/// 生の文字に使う（TBL-schema-008、REQ-schema-008）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawLine {
    pub line: usize,
    pub text: String,
}

/// 文書の行の範囲。両端を含む1始まりの行番号。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineSpan {
    pub first: usize,
    pub last: usize,
}

#[derive(Debug)]
pub struct Section {
    pub name: String,
    pub line: usize,
    pub blocks: Vec<Block>,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub struct Item {
    /// 見出しの ID 部分（先頭から `:` の直前まで）
    pub id: String,
    /// 見出しの ID より後
    pub title: String,
    /// 見出しに `:` があるか。無いときは invalid_id の対象（TBL-schema-006）
    pub has_id_separator: bool,
    pub line: usize,
    pub blocks: Vec<Block>,
}

/// 文書の1ブロック。行の種別は仕様の「規則種別」に対応する。
#[derive(Debug, Clone)]
pub enum Block {
    Field {
        /// 元の行（マーカーとその直後の空白を含む）。抽出の1要素に使う（TBL-schema-008）
        line_text: String,
        /// マーカーを除いた元の行テキスト（`名前: 値` の原形）。箇条書きとして
        /// 扱うとき（TBL-schema-007）の pattern はこの元の行に適用する
        text: String,
        /// lead 段落の最後の行。lead 段落が無ければマーカーの行（REQ-schema-063）
        lead_end: usize,
        name: String,
        value: String,
        /// フィールド行の子である継続段落の行の範囲。REQ-schema-030 ではフィールド行の一部として扱う
        continuation: Vec<LineSpan>,
        /// 子のブロック（入れ子の箇条書き・コードブロック・表など）。REQ-schema-031
        children: Vec<Block>,
        line: usize,
    },
    Bullet {
        /// 元の行（マーカーとその直後の空白を含む）。抽出の1要素に使う（TBL-schema-008）
        line_text: String,
        text: String,
        /// lead 段落の最後の行。lead 段落が無ければマーカーの行（REQ-schema-063）
        lead_end: usize,
        /// リスト項目の子である継続段落の行の範囲。REQ-schema-030 では箇条書きの一部として扱う
        continuation: Vec<LineSpan>,
        /// 子のブロック（入れ子の箇条書き・コードブロック・表など）。REQ-schema-031
        children: Vec<Block>,
        line: usize,
    },
    /// 順序付きリストの項目（`1. ` など）。箇条書きの対象外（TBL-schema-007）。
    /// 閉じた世界では undeclared_line になる。抽出では値に含めない。
    OrderedList {
        text: String,
        continuation: Vec<String>,
        line: usize,
    },
    Statement {
        text: String,
        line: usize,
        /// 段落が覆う行の生の行と行番号。1行目の字下げを含む（TBL-schema-008）
        raw_lines: Vec<RawLine>,
    },
    Table {
        header: Vec<String>,
        rows: Vec<Vec<String>>,
        line: usize,
        /// データ行ごとの行番号。ヘッダの行と区切りの行は数えない（TBL-schema-008）
        row_lines: Vec<usize>,
    },
    Code {
        lang: Option<String>,
        value: String,
        line: usize,
    },
    /// どの規則種別にも当てはまらないブロック（引用、水平線など）
    Other { line: usize },
}

impl Document {
    /// Markdown を mdast に解析し、文書の木に組み立てる。
    pub fn parse(src: &str) -> Result<Document, crate::ParseError> {
        let root = parse_mdast(src).map_err(crate::ParseError)?;
        let Node::Root(root) = root else {
            return Ok(Document::default());
        };

        let lines = split_lines(src);
        let mut tree = TreeBuilder::default();
        let mut frontmatter_lines = 0;

        for child in root.children {
            let line = child.position().map(|p| p.start.line).unwrap_or(1);
            #[expect(
                clippy::wildcard_enum_match_arm,
                reason = "markdown::mdast::Node is a foreign enum: the remaining node kinds are deliberately handled alike"
            )]
            match child {
                Node::Yaml(_) | Node::Toml(_) => {
                    frontmatter_lines = child.position().map(|p| p.end.line).unwrap_or(0);
                }
                Node::Heading(h) => tree.heading(inline_text(&h.children), h.depth, line),
                other => tree.blocks(blocks_from_node(&other, src, &lines)),
            }
        }
        let mut doc = tree.doc;
        doc.lines = lines;
        doc.frontmatter_lines = frontmatter_lines;
        Ok(doc)
    }
}

#[derive(Clone, Copy, Default)]
enum BlockOwner {
    #[default]
    Preamble,
    Section(usize),
    Item {
        section: usize,
        item: usize,
    },
    StrayPreambleHeading(usize),
}

/// 見出しとブロックの並びから、題名・前置部・節・項目の木を組み立てる。
/// 段落の読み方と行の読み方が共有する（TBL-schema-011）。
#[derive(Default)]
pub(crate) struct TreeBuilder {
    doc: Document,
    owner: BlockOwner,
}

impl TreeBuilder {
    pub(crate) fn into_document(self) -> Document {
        self.doc
    }

    pub(crate) fn heading(&mut self, text: String, depth: u8, line: usize) {
        let doc = &mut self.doc;
        match depth {
            1 => {
                doc.titles.push(Heading {
                    text,
                    depth: 1,
                    line,
                });
                // 題名はそれまでの節と項目を終える。要素の範囲（REQ-schema-062）と同じ区切り方にする
                // （review7-gaps の A3）
                self.owner = BlockOwner::Preamble;
            }
            2 => {
                doc.sections.push(Section {
                    name: text,
                    line,
                    blocks: Vec::new(),
                    items: Vec::new(),
                });
                self.owner = BlockOwner::Section(doc.sections.len() - 1);
            }
            3 => match self.owner {
                BlockOwner::Section(sec_idx)
                | BlockOwner::Item {
                    section: sec_idx, ..
                } => {
                    let section = &mut doc.sections[sec_idx];
                    let (id, title, has_id_separator) = split_item_heading(&text);
                    section.items.push(Item {
                        id,
                        title,
                        has_id_separator,
                        line,
                        blocks: Vec::new(),
                    });
                    self.owner = BlockOwner::Item {
                        section: sec_idx,
                        item: section.items.len() - 1,
                    };
                }
                BlockOwner::Preamble | BlockOwner::StrayPreambleHeading(_) => {
                    // 前置部領域の深さ3の見出し。内側の行をここに集める（REQ-schema-003）。
                    // 題名より前に出たかどうかで open の扱いが変わる（REQ-schema-002）
                    doc.stray_preamble_headings.push(StrayPreambleHeading {
                        heading: Heading {
                            text,
                            depth: 3,
                            line,
                        },
                        blocks: Vec::new(),
                        before_title: doc.titles.is_empty(),
                    });
                    self.owner =
                        BlockOwner::StrayPreambleHeading(doc.stray_preamble_headings.len() - 1);
                }
            },
            depth => doc.stray_headings.push(Heading { text, depth, line }),
        }
    }

    pub(crate) fn blocks(&mut self, blocks: Vec<Block>) {
        let doc = &mut self.doc;
        match self.owner {
            BlockOwner::Preamble => doc.preamble.extend(blocks),
            BlockOwner::Section(section) => doc.sections[section].blocks.extend(blocks),
            BlockOwner::Item { section, item } => {
                doc.sections[section].items[item].blocks.extend(blocks);
            }
            BlockOwner::StrayPreambleHeading(heading) => {
                doc.stray_preamble_headings[heading].blocks.extend(blocks);
            }
        }
    }
}

/// 文書を行に分ける。markdown は LF・CRLF・単独の CR のどれでも行を区切って
/// 行番号を進めるので、行番号の索引になるこの分け方も同じ3つで区切る。
fn split_lines(src: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    let mut chars = src.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\n' => lines.push(std::mem::take(&mut current)),
            '\r' => {
                // CRLF は1つの行区切り
                if chars.peek() == Some(&'\n') {
                    chars.next();
                }
                lines.push(std::mem::take(&mut current));
            }
            _ => current.push(c),
        }
    }
    lines.push(current);
    lines
}

/// 1始まりの行番号の生の行。範囲の外は空文字列。
fn raw_line_of(lines: &[String], line: usize) -> String {
    line.checked_sub(1)
        .and_then(|i| lines.get(i))
        .cloned()
        .unwrap_or_default()
}

/// ノードが覆う行の生の行と行番号（TBL-schema-008、REQ-schema-008）。
fn raw_lines_of(lines: &[String], node: &Node) -> Vec<RawLine> {
    let Some(position) = node.position() else {
        return Vec::new();
    };
    (position.start.line..=position.end.line)
        .map(|line| RawLine {
            line,
            text: raw_line_of(lines, line),
        })
        .collect()
}

#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "markdown::mdast::Node is a foreign enum: the remaining node kinds are deliberately handled alike"
)]
fn blocks_from_node(node: &Node, src: &str, lines: &[String]) -> Vec<Block> {
    let line = start_line(node);
    match node {
        // 画像だけの行（例: `![alt](img.png)`）は文の対象外（REQ-schema-032）
        Node::Paragraph(_) if is_image_only_paragraph(node) => vec![Block::Other { line }],
        Node::Paragraph(_) => {
            vec![Block::Statement {
                text: raw_slice(src, node),
                line,
                raw_lines: raw_lines_of(lines, node),
            }]
        }
        Node::List(list) => {
            let mut out = Vec::new();
            for child in &list.children {
                if let Node::ListItem(item) = child {
                    out.extend(blocks_from_list_item(item, src, lines, list.ordered, false));
                }
            }
            out
        }
        Node::Table(table) => vec![table_block_from_node(table, 0)],
        Node::Code(code) => vec![Block::Code {
            lang: code.lang.clone(),
            value: code.value.clone(),
            line,
        }],
        _ => vec![Block::Other { line }],
    }
}

/// 表のノードを表のブロックにする。`line_offset` はノードの行番号に足す行数で、
/// 文書の一部だけを読み直した表の行番号を文書の行番号に戻す。
pub(crate) fn table_block_from_node(table: &markdown::mdast::Table, line_offset: usize) -> Block {
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut row_lines: Vec<usize> = Vec::new();
    for row in &table.children {
        if let Node::TableRow(row) = row {
            let cells = row.children.iter().map(cell_text).collect();
            rows.push(cells);
            row_lines.push(line_at(row.position.as_ref()) + line_offset);
        }
    }
    let mut rows = rows.into_iter();
    let header: Vec<String> = rows.next().unwrap_or_default();
    // ヘッダより多いセルは捨てる（REQ-schema-033）
    let rows = rows
        .map(|mut row| {
            row.truncate(header.len());
            row
        })
        .collect();
    Block::Table {
        header,
        rows,
        line: line_at(table.position.as_ref()) + line_offset,
        // ヘッダの行は行番号の並びから外す。区切りの行は表のノードに
        // 現れないので、残りがデータ行そのものになる（TBL-schema-008）
        row_lines: row_lines.into_iter().skip(1).collect(),
    }
}

/// リスト項目を1つ以上のブロックにする。先頭の段落がフィールド行か箇条書きかを
/// 決め、続く段落（継続段落）は箇条書きの一部にする（REQ-schema-030）。順序付きリストの
/// 項目は箇条書きの対象外で、閉じた世界では undeclared_line になる（TBL-schema-007）。
/// 入れ子のリストは親の箇条書きの子ブロックとして保持する（REQ-schema-031）。コードブロック・
/// 表などの子もブロックとして子に残し、閉じた世界の undeclared_line の対象にする。
/// 先頭がコードブロック・表などで lead の段落が無いとき、後続の段落は文として
/// 扱う（REQ-schema-030）。文の出現回数・規則（REQ-schema-032）の対象になり、閉じた世界では
/// undeclared_line になる。ただし画像だけの段落は文に数えない（REQ-schema-032）。`preserve_indent` が真のとき、子の行の元のインデントを
/// 保ったまま元の行を取る（TBL-schema-008 の「元の行を保った文字列」のため）。
fn blocks_from_list_item(
    item: &markdown::mdast::ListItem,
    src: &str,
    lines: &[String],
    ordered: bool,
    preserve_indent: bool,
) -> Vec<Block> {
    let item_line = line_at(item.position.as_ref());
    let mut lead_block: Option<Block> = None;
    let mut extra: Vec<Block> = Vec::new();
    for (i, child) in item.children.iter().enumerate() {
        #[expect(
            clippy::wildcard_enum_match_arm,
            reason = "markdown::mdast::Node is a foreign enum: the remaining node kinds are deliberately handled alike"
        )]
        match child {
            // 先頭の段落だけがフィールド行・箇条書き・順序付き項目の lead になる。
            // 続く段落は継続段落として lead に付く（REQ-schema-030）。
            Node::Paragraph(_) => {
                let text = raw_slice(src, child);
                if i == 0 {
                    let lead_end = child.position().map_or(item_line, |p| p.end.line);
                    let line_text = original_item_line(item, src, preserve_indent);
                    lead_block = Some(if ordered {
                        // 順序付きリストは箇条書きの対象外。閉じた世界では undeclared_line（TBL-schema-007）
                        Block::OrderedList {
                            text,
                            continuation: Vec::new(),
                            line: item_line,
                        }
                    } else {
                        match split_field(&text) {
                            Some((name, value)) => Block::Field {
                                line_text,
                                text,
                                lead_end,
                                name,
                                value,
                                continuation: Vec::new(),
                                children: Vec::new(),
                                line: item_line,
                            },
                            None => Block::Bullet {
                                line_text,
                                text,
                                lead_end,
                                continuation: Vec::new(),
                                children: Vec::new(),
                                line: item_line,
                            },
                        }
                    });
                } else if lead_block.is_some() {
                    match &mut lead_block {
                        Some(Block::Field { continuation, .. })
                        | Some(Block::Bullet { continuation, .. }) => {
                            continuation.push(span_of(child))
                        }
                        Some(Block::OrderedList { continuation, .. }) => continuation.push(text),
                        _ => {}
                    }
                } else {
                    // lead が無い項目（先頭がコードブロック・表など）の段落は
                    // 文として扱う。継続段落は lead に付く場合だけだから、この
                    // 段落はどこにも吸われない（REQ-schema-030）。画像だけの段落は
                    // blocks_from_node が文の対象外にする（REQ-schema-032）。
                    extra.extend(blocks_from_node(child, src, lines));
                }
            }
            Node::List(l) => {
                let mut nested = Vec::new();
                for nested_item in &l.children {
                    if let Node::ListItem(ni) = nested_item {
                        nested.extend(blocks_from_list_item(ni, src, lines, l.ordered, true));
                    }
                }
                match &mut lead_block {
                    // 入れ子のリストは親の子として保持する（REQ-schema-031）
                    Some(Block::Field { children, .. }) | Some(Block::Bullet { children, .. }) => {
                        children.extend(nested)
                    }
                    _ => extra.extend(nested),
                }
            }
            // コードブロック・表などのブロックは捨てず、子のブロックとして残す。
            // 引用・水平線などは blocks_from_node が Block::Other にして閉じた
            // 世界でも無視される（REQ-schema-032）。
            other => {
                let blocks = blocks_from_node(other, src, lines);
                match &mut lead_block {
                    Some(Block::Field { children, .. }) | Some(Block::Bullet { children, .. }) => {
                        children.extend(blocks)
                    }
                    _ => extra.extend(blocks),
                }
            }
        }
    }
    // 中身の無い項目（"-" や "1." だけの行）も1本として残す。捨てると宣言していない行が
    // 指摘にならずに消える（REQ-schema-001）
    if item.children.is_empty() {
        lead_block = Some(if ordered {
            Block::OrderedList {
                text: String::new(),
                continuation: Vec::new(),
                line: item_line,
            }
        } else {
            Block::Bullet {
                line_text: original_item_line(item, src, preserve_indent),
                text: String::new(),
                lead_end: item_line,
                continuation: Vec::new(),
                children: Vec::new(),
                line: item_line,
            }
        });
    }
    let mut out = Vec::new();
    if let Some(lead) = lead_block {
        out.push(lead);
    }
    out.extend(extra);
    out
}

/// 段落が画像ノードだけで構成されているか。画像だけの行は文として数えない。
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "markdown::mdast::Node is a foreign enum: the remaining node kinds are deliberately handled alike"
)]
fn is_image_only_paragraph(node: &Node) -> bool {
    match node {
        Node::Paragraph(p) => !p.children.is_empty() && p.children.iter().all(is_image),
        _ => false,
    }
}

/// インライン要素が画像か。
fn is_image(node: &Node) -> bool {
    matches!(node, Node::Image(_))
}

/// リスト項目の元の1行目（マーカーとその直後の空白を含む）。行頭のインデントは
/// トップレベルの箇条書きとして扱うため取り除く（TBL-schema-008）。子の行は `preserve_indent`
/// が真で、元のインデントを保ったままの行を取る（TBL-schema-008 の「元の行を保った
/// 文字列」のため）。行末の空白（ハード改行）は元の行の
/// 一部として残す
fn original_item_line(
    item: &markdown::mdast::ListItem,
    src: &str,
    preserve_indent: bool,
) -> String {
    let offset = item.position.as_ref().map(|p| p.start.offset).unwrap_or(0);
    let line_start = if preserve_indent {
        // マーカー行の行頭まで遡って、子のインデントを含む行を取る
        src[..offset]
            .rfind(['\n', '\r'])
            .map(|i| i + 1)
            .unwrap_or(0)
    } else {
        offset
    };
    // 行の区切りは split_lines と同じく LF・CRLF・単独の CR
    let end = src[offset..]
        .find(['\n', '\r'])
        .map(|i| offset + i)
        .unwrap_or(src.len());
    if preserve_indent {
        // 子の行は元のインデントを保ったまま返す
        src[line_start..end].to_string()
    } else {
        src[offset..end].trim_start().to_string()
    }
}

/// `- 名前: 値` の形なら名前と値に分ける。形でなければ None。
pub(crate) fn split_field(text: &str) -> Option<(String, String)> {
    let idx = text.find(':')?;
    let name = text[..idx].trim();
    if name.is_empty() {
        return None;
    }
    let value = text[idx + 1..].trim();
    Some((name.to_string(), value.to_string()))
}

impl Block {
    /// ブロックが現れた行番号（1始まり）。導かれる値の `line` と `raw` の抽出に使う（REQ-schema-048）。
    pub fn line(&self) -> usize {
        match self {
            Block::Field { line, .. }
            | Block::Bullet { line, .. }
            | Block::OrderedList { line, .. }
            | Block::Statement { line, .. }
            | Block::Table { line, .. }
            | Block::Code { line, .. }
            | Block::Other { line } => *line,
        }
    }

    /// 子のブロック（入れ子の箇条書き・コードブロック・表など）。箇条書きでも
    /// フィールド行でもないブロックは空を返す。
    pub fn children(&self) -> &[Block] {
        match self {
            Block::Field { children, .. } | Block::Bullet { children, .. } => children,
            Block::OrderedList { .. }
            | Block::Statement { .. }
            | Block::Table { .. }
            | Block::Code { .. }
            | Block::Other { .. } => &[],
        }
    }
}

/// ノードが覆う行の範囲。位置が無ければ1行目だけとする。
fn span_of(node: &Node) -> LineSpan {
    node.position()
        .map_or(LineSpan { first: 1, last: 1 }, |p| LineSpan {
            first: p.start.line,
            last: p.end.line,
        })
}

/// 項目見出しを ID と題名に分ける。`:` が無ければ全体を ID にし、区切りが
/// 無いことを返す（TBL-schema-006 の判定に使う）。
fn split_item_heading(text: &str) -> (String, String, bool) {
    match text.find(':') {
        Some(idx) => (
            text[..idx].trim().to_string(),
            text[idx + 1..].trim().to_string(),
            true,
        ),
        None => (text.trim().to_string(), String::new(), false),
    }
}

/// インライン要素のテキストを連結する。
pub(crate) fn inline_text(children: &[Node]) -> String {
    let mut out = String::new();
    for child in children {
        #[expect(
            clippy::wildcard_enum_match_arm,
            reason = "markdown::mdast::Node is a foreign enum: the remaining node kinds are deliberately handled alike"
        )]
        match child {
            Node::Text(t) => out.push_str(&t.value),
            Node::InlineCode(c) => out.push_str(&c.value),
            Node::Emphasis(e) => out.push_str(&inline_text(&e.children)),
            Node::Strong(s) => out.push_str(&inline_text(&s.children)),
            Node::Link(l) => out.push_str(&inline_text(&l.children)),
            Node::LinkReference(l) => out.push_str(&inline_text(&l.children)),
            // GFM の打ち消し線も中の文字を残す（review7-gaps の A2）
            Node::Delete(d) => out.push_str(&inline_text(&d.children)),
            _ => {}
        }
    }
    out
}

/// 表のセルのテキスト。
fn cell_text(cell: &Node) -> String {
    if let Node::TableCell(cell) = cell {
        inline_text(&cell.children)
    } else {
        String::new()
    }
}

fn start_line(node: &Node) -> usize {
    line_at(node.position())
}

/// ノードの位置が指す範囲の生テキスト。インラインの Markdown 記法を保持する。
fn raw_slice(src: &str, node: &Node) -> String {
    slice_at(src, node.position())
}

fn line_at(position: Option<&markdown::unist::Position>) -> usize {
    position.map(|p| p.start.line).unwrap_or(1)
}

fn slice_at(src: &str, position: Option<&markdown::unist::Position>) -> String {
    match position {
        // 複数行にまたがる範囲は、行を改行1つでつなぎ直す。CRLF と単独の CR も
        // split_lines と同じく1つの行区切りとして扱う（REQ-schema-063）
        Some(pos) => src[pos.start.offset..pos.end.offset]
            .trim()
            .replace("\r\n", "\n")
            .replace('\r', "\n"),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn section_blocks(doc: &Document) -> &[Block] {
        assert_eq!(doc.sections.len(), 1, "節が1つあること");
        &doc.sections[0].blocks
    }

    // @kotowari[REQ-schema-031]
    #[test]
    fn nested_list_items_become_children_of_the_parent_bullet() {
        let doc = Document::parse("## 理由\n\n- 親\n  - 子\n    - 孫\n").unwrap();
        let blocks = section_blocks(&doc);
        assert_eq!(blocks.len(), 1);
        let Block::Bullet { text, children, .. } = &blocks[0] else {
            panic!("親が Bullet になる");
        };
        assert_eq!(text, "親");
        assert_eq!(children.len(), 1);
        let Block::Bullet { text, children, .. } = &children[0] else {
            panic!("子が Bullet になる");
        };
        assert_eq!(text, "子");
        assert_eq!(children.len(), 1);
        let Block::Bullet { text, .. } = &children[0] else {
            panic!("孫が Bullet になる");
        };
        assert_eq!(text, "孫");
    }

    // @kotowari[REQ-schema-031]
    #[test]
    fn nested_list_children_keep_their_indentation() {
        let doc = Document::parse("## 理由\n\n- 親\n  - 子\n    - 孫\n").unwrap();
        let blocks = section_blocks(&doc);
        let Block::Bullet {
            line_text,
            children,
            ..
        } = &blocks[0]
        else {
            panic!("親が Bullet になる");
        };
        assert_eq!(line_text, "- 親");
        let Block::Bullet {
            line_text,
            children,
            ..
        } = &children[0]
        else {
            panic!("子が Bullet になる");
        };
        assert_eq!(line_text, "  - 子");
        let Block::Bullet { line_text, .. } = &children[0] else {
            panic!("孫が Bullet になる");
        };
        assert_eq!(line_text, "    - 孫");
    }

    // @kotowari[REQ-schema-030]
    #[test]
    fn continuation_paragraph_attaches_to_the_parent_bullet() {
        let doc = Document::parse("## 理由\n\n- 親\n\n  続きの段落\n  - 子\n").unwrap();
        let blocks = section_blocks(&doc);
        let Block::Bullet {
            continuation,
            children,
            ..
        } = &blocks[0]
        else {
            panic!("親が Bullet になる");
        };
        assert_eq!(continuation, &vec![LineSpan { first: 5, last: 5 }]);
        assert_eq!(children.len(), 1);
    }

    // @kotowari[REQ-schema-031, REQ-schema-034]
    #[test]
    fn code_block_child_of_bullet_remains_a_block() {
        let doc = Document::parse("## 理由\n\n- 親\n\n  ```python\n  x = 1\n  ```\n").unwrap();
        let blocks = section_blocks(&doc);
        let Block::Bullet { children, .. } = &blocks[0] else {
            panic!("親が Bullet になる");
        };
        assert!(matches!(children[0], Block::Code { lang: Some(ref l), .. } if l == "python"));
    }

    // @kotowari[REQ-schema-031, REQ-schema-033]
    #[test]
    fn table_child_of_bullet_remains_a_block() {
        let doc =
            Document::parse("## 理由\n\n- 親\n\n  | a | b |\n  |---|---|\n  | 1 | 2 |\n").unwrap();
        let blocks = section_blocks(&doc);
        let Block::Bullet { children, .. } = &blocks[0] else {
            panic!("親が Bullet になる");
        };
        assert!(
            matches!(children[0], Block::Table { ref header, .. } if header == &vec!["a".to_string(), "b".to_string()])
        );
    }

    // @kotowari[REQ-schema-032]
    #[test]
    fn statement_after_code_block_lead_is_a_statement() {
        let doc =
            Document::parse("## 理由\n\n- ```python\n  x = 1\n  ```\n\n  後続の段落\n").unwrap();
        let blocks = section_blocks(&doc);
        assert!(matches!(blocks[0], Block::Code { .. }));
        assert!(matches!(blocks[1], Block::Statement { ref text, .. } if text == "後続の段落"));
    }

    // @kotowari[REQ-schema-031]
    #[test]
    fn field_line_with_children_keeps_children() {
        let doc = Document::parse("## 理由\n\n- 状態: 承認済み\n  - 子\n").unwrap();
        let blocks = section_blocks(&doc);
        let Block::Field {
            name,
            children,
            line_text,
            ..
        } = &blocks[0]
        else {
            panic!("親の `- 名前: 値` 行が Field になる");
        };
        assert_eq!(name, "状態");
        assert_eq!(line_text, "- 状態: 承認済み");
        assert_eq!(children.len(), 1);
    }

    // @kotowari[REQ-schema-028]
    #[test]
    fn ordered_list_item_children_are_not_attached() {
        let doc = Document::parse("## 理由\n\n1. 順序付き\n   - 箇条書き\n").unwrap();
        let blocks = section_blocks(&doc);
        assert!(matches!(blocks[0], Block::OrderedList { .. }));
        assert!(
            matches!(blocks[1], Block::Bullet { .. }),
            "順序付きの入れ子は対象外でトップレベルに残る"
        );
    }
    // @kotowari[REQ-schema-008]
    #[test]
    fn title_heading_keeps_the_raw_line_including_inline_code() {
        let src = "# 題名 `インライン` と [リンク](https://example.com/)\n";
        let doc = Document::parse(src).unwrap();
        let raw = doc.raw_line(doc.titles[0].line).unwrap();
        assert_eq!(
            raw, "# 題名 `インライン` と [リンク](https://example.com/)",
            "見出しの行の生の文字は src の行と一文字も違わない"
        );
        assert_ne!(
            raw, doc.titles[0].text,
            "組み立て直した見出しの文字とは違う"
        );
    }

    // @kotowari[REQ-schema-035]
    #[test]
    fn statement_keeps_each_raw_line_with_its_line_number() {
        let src = "## 節\n\n 文の1行目\n  字下げの2行目\n   3行目\n";
        let doc = Document::parse(src).unwrap();
        let blocks = section_blocks(&doc);
        let Block::Statement { raw_lines, .. } = &blocks[0] else {
            panic!("段落が文になる");
        };
        assert_eq!(raw_lines.len(), 3);
        assert_eq!(raw_lines[0].line, 3);
        assert_eq!(raw_lines[0].text, " 文の1行目");
        assert_eq!(raw_lines[1].line, 4);
        assert_eq!(raw_lines[1].text, "  字下げの2行目");
        assert_eq!(raw_lines[2].line, 5);
        assert_eq!(raw_lines[2].text, "   3行目");
    }

    // @kotowari[REQ-schema-048]
    #[test]
    fn table_keeps_the_line_number_of_each_data_row() {
        let src = "## 節\n\n| a | b |\n|---|---|\n| 1 | 2 |\n| 3 | 4 |\n| 5 | 6 |\n";
        let doc = Document::parse(src).unwrap();
        let blocks = section_blocks(&doc);
        let Block::Table {
            rows,
            row_lines,
            line,
            ..
        } = &blocks[0]
        else {
            panic!("表になる");
        };
        assert_eq!(*line, 3, "表の開始行はヘッダの行");
        assert_eq!(rows.len(), 3);
        assert_eq!(
            row_lines,
            &vec![5, 6, 7],
            "ヘッダの行と区切りの行は数えない"
        );
    }

    // @kotowari[REQ-schema-062]
    #[test]
    fn end_line_does_not_count_a_heading_shaped_line_inside_a_code_block() {
        let src = "## 節\n\n### A-1: a\n\n```\n## 中\n```\n\n### A-2: b\n";
        let doc = Document::parse(src).unwrap();
        assert_eq!(
            doc.end_line(3, 3),
            8,
            "コードブロックの中の \"## 中\" では終わらない"
        );
        assert_eq!(
            doc.end_line(1, 2),
            9,
            "文書の最後の行。末尾の区切りの後ろは数えない"
        );
        let by_line = doc.read_by_line();
        assert_eq!(by_line.end_line(3, 3), 8, "行の読み方でも同じ");
    }

    // @kotowari[REQ-schema-062]
    #[test]
    fn end_line_is_the_last_line_when_the_document_does_not_end_with_a_break() {
        let doc = Document::parse("## 節\n\n### A-1: a\n\n本文").unwrap();
        assert_eq!(
            doc.end_line(3, 3),
            5,
            "最後の行に区切りが無くてもその行が最後の行"
        );
        assert_eq!(doc.read_by_line().end_line(3, 3), 5, "行の読み方でも同じ");
    }

    // @kotowari[REQ-schema-030]
    #[test]
    fn a_continuation_paragraph_of_an_ordered_item_belongs_to_that_item() {
        let doc = Document::parse("## 理由\n\n1. 順序付き\n\n   続きの段落\n").unwrap();
        let blocks = section_blocks(&doc);
        assert_eq!(blocks.len(), 1, "継続段落は文にならない: {blocks:?}");
        let Block::OrderedList { continuation, .. } = &blocks[0] else {
            panic!("順序付きの行になる: {blocks:?}");
        };
        assert_eq!(continuation, &vec!["続きの段落".to_string()]);
    }

    // @kotowari[REQ-schema-035]
    #[test]
    fn a_lone_cr_breaks_a_line_the_same_way_as_lf_and_crlf() {
        let statement_lines = |src: &str| {
            let doc = Document::parse(src).unwrap();
            let blocks = section_blocks(&doc);
            let Block::Statement { raw_lines, .. } = &blocks[0] else {
                panic!("段落が文になる");
            };
            raw_lines
                .iter()
                .map(|raw| (raw.line, raw.text.clone()))
                .collect::<Vec<_>>()
        };
        let lf = statement_lines("## 節\n\nAAA\nBBB\nCCC\n");
        assert_eq!(
            statement_lines("## 節\r\n\r\nAAA\r\nBBB\r\nCCC\r\n"),
            lf,
            "CRLF の文書も LF と同じ行に分かれる"
        );
        assert_eq!(
            statement_lines("## 節\r\rAAA\rBBB\rCCC\r"),
            lf,
            "単独の CR の文書も LF と同じ行に分かれる"
        );
    }
}
