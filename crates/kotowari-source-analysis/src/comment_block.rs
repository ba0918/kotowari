//! `直前のコメントの塊`（TBL-core-016、TBL-core-035）
//!
//! 言語に依存しない行の規則で、`テスト`の節の最初の行から上へ、コメントだけの行と
//! 挟んでよい行が空行なしで続く間を塊とする。コメントは tree-sitter の extra の節で見分ける。

use ast_grep_core::Node;
use ast_grep_core::tree_sitter::StrDoc;
use ast_grep_language::SupportLang;

use kotowari_core::tests_discovery::{InvalidMarkers, MarkerIds, parse_markers_in_line};

#[derive(Clone, Copy, PartialEq, Eq)]
enum ByteClass {
    Code,
    Comment,
    Allowed,
}

/// 1つのファイル（またはマクロの中身）の行ごとの分類
pub struct LineMap<'s> {
    source: &'s str,
    /// 各行の (開始バイト, 終了バイト)。終了は改行を含まない
    lines: Vec<(usize, usize)>,
    bytes: Vec<ByteClass>,
}

/// 挟んでよい行になる節の種類（TBL-core-035）
fn allowed_kinds(lang: SupportLang) -> &'static [&'static str] {
    match lang {
        SupportLang::Rust => &["attribute_item"],
        SupportLang::Python => &["decorator"],
        _ => &[],
    }
}

impl<'s> LineMap<'s> {
    pub fn new(source: &'s str, root: &Node<'_, StrDoc<SupportLang>>, lang: SupportLang) -> Self {
        let mut bytes = vec![ByteClass::Code; source.len()];
        let allowed = allowed_kinds(lang);
        for node in root.dfs() {
            let class = if node.is_extra() {
                ByteClass::Comment
            } else if allowed.contains(&node.kind().as_ref()) {
                ByteClass::Allowed
            } else {
                continue;
            };
            // 深さ優先で親から子の順に来るので、属性やデコレータの中のコメントはコメントになる
            bytes[node.range()].fill(class);
        }
        let mut lines = Vec::new();
        let mut start = 0;
        for (i, c) in source.bytes().enumerate() {
            if c == b'\n' {
                // "\r\n" の "\r" は行の文字に含めない（str::lines と同じ）
                let end = if i > start && source.as_bytes()[i - 1] == b'\r' {
                    i - 1
                } else {
                    i
                };
                lines.push((start, end));
                start = i + 1;
            }
        }
        // 改行で終わらない最後の行。改行で終わるときは空の行が1つ増えるが、
        // その下に`テスト`の節が無いので塊には入らない
        lines.push((start, source.len()));
        LineMap {
            source,
            lines,
            bytes,
        }
    }

    /// 塊に入る行か。空白を除いた文字がすべてコメントか挟んでよい節の文字である行と、
    /// 複数行のコメントや属性の途中にある空白だけの行（行の終わりの改行がその節の中）
    fn joins_block(&self, line: usize) -> bool {
        let (start, end) = self.lines[line];
        let mut has_text = false;
        // 空白は Unicode の空白（全角空白と NBSP を含む）
        for (offset, c) in self.source[start..end].char_indices() {
            if c.is_whitespace() {
                continue;
            }
            if self.bytes[start + offset] == ByteClass::Code {
                return false;
            }
            has_text = true;
        }
        has_text || self.bytes.get(end).is_some_and(|b| *b != ByteClass::Code)
    }

    /// `テスト`の節の最初の行（0始まり）の`直前のコメントの塊`の印を集める。
    /// 返り値: (印の出現, 空・閉じ括弧の無い印)。行は1始まり
    pub fn markers_before(&self, first_line: usize) -> (MarkerIds, InvalidMarkers) {
        let mut ids = Vec::new();
        let mut invalid = Vec::new();
        let mut top = first_line;
        while top > 0 && self.joins_block(top - 1) {
            top -= 1;
        }
        for line in top..first_line {
            let (start, end) = self.lines[line];
            let raw = &self.source[start..end];
            let line_num = line + 1;
            for marker in parse_markers_in_line(&self.comment_text(start, end), line_num) {
                if marker.ids.is_empty() {
                    invalid.push((line_num, raw.to_string()));
                } else {
                    // A152: 同じ ID の印が複数あっても出現ごとに1件数える
                    for id in marker.ids {
                        ids.push((id, line_num));
                    }
                }
            }
        }
        (ids, invalid)
    }

    /// 行のうちコメントの文字だけを残し、ほかを空白にした文字列（挟んでよい行の属性の中の文字を拾わない）
    fn comment_text(&self, start: usize, end: usize) -> String {
        let mut text = String::with_capacity(end - start);
        for (offset, c) in self.source[start..end].char_indices() {
            if self.bytes[start + offset] == ByteClass::Comment {
                text.push(c);
            } else {
                text.push(' ');
            }
        }
        text
    }
}
