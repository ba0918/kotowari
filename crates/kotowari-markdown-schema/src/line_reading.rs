//! `読み方` "line" の文書の木。生の行を1行ずつ読み分ける（TBL-schema-011 の "line" の列）。
//!
//! 段落の読み方は mdast の木から組むが、行の読み方は CommonMark の段落・遅延継続・
//! 字下げのコードブロック・下線の見出し・引用・HTML を作らないので、`Document` が保つ
//! 生の行から木ごと組み直す。見出し・一覧の行・表・フェンスのコードブロックは
//! どちらの読み方でも同じに読む。

use crate::ast::parse_mdast;
use crate::document::{
    Block, Document, RawLine, TreeBuilder, inline_text, split_field, table_block_from_node,
};
use markdown::mdast::Node;

impl Document {
    /// 同じ生の行を `読み方` "line" で読み直した木（TBL-schema-011）。
    pub(crate) fn read_by_line(&self) -> Document {
        let lines = &self.lines;
        let mut reader = LineReader::default();
        let mut index = self.frontmatter_lines;
        while index < lines.len() {
            index = reader.read(lines, index);
        }
        reader.close_list();
        let mut doc = reader.tree.into_document();
        doc.lines = self.lines.clone();
        doc.frontmatter_lines = self.frontmatter_lines;
        doc
    }
}

/// 開いている一覧の行。子の一覧の行とブロックを受ける（REQ-schema-031）。
struct OpenItem {
    /// マーカーの後の中身が始まる桁。これ以上字下げした行はこの行の子になる
    content_column: usize,
    block: Block,
}

#[derive(Default)]
struct LineReader {
    tree: TreeBuilder,
    /// 開いている一覧の行。先頭が最も浅い
    open: Vec<OpenItem>,
}

impl LineReader {
    /// `index`（0始まり）の行から1つ読み、次に読む行の位置を返す。
    fn read(&mut self, lines: &[String], index: usize) -> usize {
        let text = &lines[index];
        let line = index + 1;
        if text.trim().is_empty() {
            // 空行は一覧を閉じない。字下げした一覧の行は空行を挟んでも子になる
            return index + 1;
        }
        if let Some(fence) = Fence::open(text) {
            let (block, next) = fence.read(lines, index);
            self.attach(block, indent_width(text));
            return next;
        }
        if let Some(depth) = heading_depth(text) {
            self.close_list();
            self.tree.heading(heading_text(text), depth, line);
            return index + 1;
        }
        if let Some((block, next)) = table_at(lines, index) {
            self.attach(block, indent_width(text));
            return next;
        }
        if !is_thematic_break(text)
            && let Some(marker) = ListMarker::parse(text)
        {
            // マーカーの後でフェンスを開く項目は、段落の読み方と同じく中身がコードブロックの
            // 項目として読む。一覧の行として読むと、閉じの行を次のフェンスの開始と取り違える
            if let Some(mut fence) = Fence::open(&marker.content) {
                fence.indent = marker.content_column;
                let (block, next) = fence.read(lines, index);
                self.attach(block, marker.indent);
                return next;
            }
            self.list_line(text, line, &marker);
            return index + 1;
        }
        // それ以外の行は1行ずつ`文`（TBL-schema-011）
        self.close_list();
        self.tree.blocks(vec![Block::Statement {
            text: text.trim().to_string(),
            line,
            raw_lines: vec![RawLine {
                line,
                text: text.clone(),
            }],
        }]);
        index + 1
    }

    /// 一覧の行を開く。字下げが開いている行の中身の桁に届けば、その行の子になる。
    fn list_line(&mut self, text: &str, line: usize, marker: &ListMarker) {
        self.close_deeper_than(marker.indent);
        let nested = !self.open.is_empty();
        // 子の行は元の字下げを保ち、最上位の行は行頭の字下げを除く（TBL-schema-008）
        let line_text = if nested {
            text.to_string()
        } else {
            text.trim_start().to_string()
        };
        let content = marker.content.clone();
        if marker.ordered {
            // 順序付きの行は子を持たない。子にあたる行はその後ろの兄弟になる（TBL-schema-007）
            self.attach_here(Block::OrderedList {
                text: content,
                continuation: Vec::new(),
                line,
            });
            return;
        }
        let block = match split_field(&content) {
            Some((name, value)) => Block::Field {
                line_text,
                text: content,
                lead_end: line,
                name,
                value,
                continuation: Vec::new(),
                children: Vec::new(),
                line,
            },
            None => Block::Bullet {
                line_text,
                text: content,
                lead_end: line,
                continuation: Vec::new(),
                children: Vec::new(),
                line,
            },
        };
        self.open.push(OpenItem {
            content_column: marker.content_column,
            block,
        });
    }

    /// 字下げ `indent` の表やコードブロックを、届く一覧の行の子か、`ノード`の直下に置く。
    fn attach(&mut self, block: Block, indent: usize) {
        self.close_deeper_than(indent);
        self.attach_here(block);
    }

    fn attach_here(&mut self, block: Block) {
        match self.open.last_mut() {
            Some(parent) => push_child(&mut parent.block, block),
            None => self.tree.blocks(vec![block]),
        }
    }

    /// 中身の桁が `indent` より深い一覧の行を閉じる。
    fn close_deeper_than(&mut self, indent: usize) {
        while self
            .open
            .last()
            .is_some_and(|item| item.content_column > indent)
        {
            self.close_last();
        }
    }

    fn close_list(&mut self) {
        while !self.open.is_empty() {
            self.close_last();
        }
    }

    fn close_last(&mut self) {
        let Some(item) = self.open.pop() else {
            return;
        };
        self.attach_here(item.block);
    }
}

fn push_child(parent: &mut Block, child: Block) {
    if let Block::Field { children, .. } | Block::Bullet { children, .. } = parent {
        children.push(child);
    }
}

/// 行頭の字下げの桁数。タブは次の4の倍数の桁まで進める。
fn indent_width(text: &str) -> usize {
    let mut width = 0;
    for c in text.chars() {
        match c {
            ' ' => width += 1,
            '\t' => width += 4 - width % 4,
            _ => break,
        }
    }
    width
}

/// CommonMark の ATX 見出しの行なら、その深さ（TBL-schema-011）。行頭の空白は3つまで、
/// "#" は1〜6個で、その後が空白か行末。
fn heading_depth(text: &str) -> Option<u8> {
    let spaces = text.len() - text.trim_start_matches(' ').len();
    if spaces > 3 {
        return None;
    }
    let body = &text[spaces..];
    let hashes = body.chars().take_while(|&c| c == '#').count();
    if !(1..=6).contains(&hashes) {
        return None;
    }
    match body[hashes..].chars().next() {
        None | Some(' ' | '\t') => u8::try_from(hashes).ok(),
        _ => None,
    }
}

/// 見出しの文字。段落の読み方と同じくインラインの記法を解いた文字にする。
fn heading_text(text: &str) -> String {
    if let Ok(Node::Root(root)) = parse_mdast(text)
        && let Some(Node::Heading(heading)) = root.children.first()
    {
        return inline_text(&heading.children);
    }
    text.trim().trim_start_matches('#').trim().to_string()
}

/// 水平線の行（同じ記号3つ以上と空白だけ）。行の読み方では`文`になる（TBL-schema-011）。
fn is_thematic_break(text: &str) -> bool {
    let body = text.trim();
    let Some(mark) = body.chars().next() else {
        return false;
    };
    matches!(mark, '-' | '*' | '_')
        && body.chars().all(|c| c == mark || c == ' ' || c == '\t')
        && body.chars().filter(|&c| c == mark).count() >= 3
}

/// 一覧の行のマーカー（TBL-schema-007）。
struct ListMarker {
    indent: usize,
    content_column: usize,
    ordered: bool,
    content: String,
}

impl ListMarker {
    fn parse(text: &str) -> Option<ListMarker> {
        let indent = indent_width(text);
        let body = text.trim_start_matches([' ', '\t']);
        let marker_len = if body.starts_with(['-', '*', '+']) {
            1
        } else {
            let digits = body.chars().take_while(char::is_ascii_digit).count();
            if !(1..=9).contains(&digits) || !body[digits..].starts_with(['.', ')']) {
                return None;
            }
            digits + 1
        };
        let rest = &body[marker_len..];
        if !(rest.is_empty() || rest.starts_with([' ', '\t'])) {
            return None;
        }
        let spaces = rest.len() - rest.trim_start_matches([' ', '\t']).len();
        Some(ListMarker {
            indent,
            content_column: indent + marker_len + spaces.clamp(1, 4),
            ordered: marker_len > 1,
            content: rest.trim().to_string(),
        })
    }
}

/// "```" か "~~~" のフェンスの開始（TBL-schema-011）。
struct Fence {
    mark: char,
    length: usize,
    indent: usize,
    lang: Option<String>,
}

impl Fence {
    fn open(text: &str) -> Option<Fence> {
        let body = text.trim_start_matches([' ', '\t']);
        let mark = body.chars().next().filter(|c| matches!(c, '`' | '~'))?;
        let length = body.chars().take_while(|&c| c == mark).count();
        if length < 3 {
            return None;
        }
        let info = body[length..].trim();
        if mark == '`' && info.contains('`') {
            return None;
        }
        Some(Fence {
            mark,
            length,
            indent: indent_width(text),
            lang: info.split_whitespace().next().map(str::to_string),
        })
    }

    fn closes(&self, text: &str) -> bool {
        let body = text.trim();
        body.chars().take_while(|&c| c == self.mark).count() >= self.length
            && body.chars().all(|c| c == self.mark)
    }

    /// 開始の行 `index` から閉じの行までを読み、コードブロックと次の行の位置を返す。
    /// 閉じの行が無ければ文書の最後の行まで。
    fn read(self, lines: &[String], index: usize) -> (Block, usize) {
        let mut body: Vec<&str> = Vec::new();
        let mut next = lines.len();
        for (offset, text) in lines[index + 1..].iter().enumerate() {
            if self.closes(text) {
                next = index + 1 + offset + 1;
                break;
            }
            body.push(strip_indent(text, self.indent));
        }
        let block = Block::Code {
            lang: self.lang,
            value: body.join("\n"),
            line: index + 1,
        };
        (block, next)
    }
}

/// 行頭の空白を `width` 桁まで取り除く。
fn strip_indent(text: &str, width: usize) -> &str {
    let spaces = text.len() - text.trim_start_matches(' ').len();
    &text[spaces.min(width)..]
}

/// 見出しの行と区切りの行で始まる GFM の表を`表`として読む（TBL-schema-011）。
/// 縦棒で始まらない表も受け、表の終わりは GFM に任せる。表にならなければ None で、
/// その行は`文`になる。
fn table_at(lines: &[String], index: usize) -> Option<(Block, usize)> {
    // 区切りの行に置ける文字だけの行が続くときだけ GFM に読ませる。表かどうかは GFM が決める
    if !lines.get(index + 1).is_some_and(|next| {
        next.chars()
            .all(|c| matches!(c, '|' | '-' | ':' | ' ' | '\t'))
    }) {
        return None;
    }
    // 空行までを GFM に読ませ、表が終わった行から先は呼ぶ側が読み直す
    let limit = lines[index..]
        .iter()
        .position(|text| text.trim().is_empty())
        .map_or(lines.len(), |offset| index + offset);
    let chunk: Vec<&str> = lines[index..limit].iter().map(|l| l.trim_start()).collect();
    let root = parse_mdast(&chunk.join("\n")).ok()?;
    let Node::Root(root) = root else {
        return None;
    };
    let Node::Table(table) = root.children.first()? else {
        return None;
    };
    let rows = table.position.as_ref()?.end.line;
    Some((table_block_from_node(table, index), index + rows))
}
