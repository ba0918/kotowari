//! `直前のコメントの塊`（TBL-core-016、TBL-core-035）
//!
//! 言語に依存しない行の規則で、`テスト`の節の最初の行から上へ、コメントだけの行と
//! 挟んでよい行が空行なしで続く間を塊とする。コメントは tree-sitter の extra の節で見分ける。

use ast_grep_core::Node;
use ast_grep_core::tree_sitter::StrDoc;
use ast_grep_language::SupportLang;

use crate::tests_discovery::parse_markers_in_line;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ByteClass {
    Code,
    Comment,
    Allowed,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LineClass {
    Blank,
    Code,
    /// コメントだけの行
    Comment,
    /// 挟んでよい行（Rust の属性、Python のデコレータ）。コメントが混ざってもよい
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
            for b in &mut bytes[node.range()] {
                // コメントの中に節があっても、外側の分類を保つ
                if *b == ByteClass::Code {
                    *b = class;
                }
            }
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
        if start < source.len() {
            lines.push((start, source.len()));
        }
        LineMap {
            source,
            lines,
            bytes,
        }
    }

    fn class_of(&self, line: usize) -> LineClass {
        let (start, end) = self.lines[line];
        let text = &self.source.as_bytes()[start..end];
        let mut has_allowed = false;
        let mut has_comment = false;
        for (offset, c) in text.iter().enumerate() {
            if c.is_ascii_whitespace() {
                continue;
            }
            match self.bytes[start + offset] {
                ByteClass::Code => return LineClass::Code,
                ByteClass::Allowed => has_allowed = true,
                ByteClass::Comment => has_comment = true,
            }
        }
        if has_allowed {
            LineClass::Allowed
        } else if has_comment || self.inside_comment(end) {
            LineClass::Comment
        } else {
            LineClass::Blank
        }
    }

    /// 空白だけの行が複数行のコメントの途中にあるか（行の終わりの改行がコメントの中）
    fn inside_comment(&self, end: usize) -> bool {
        self.bytes.get(end) == Some(&ByteClass::Comment)
    }

    /// `テスト`の節の最初の行（0始まり）の`直前のコメントの塊`の印を集める。
    /// 返り値: (出現ごとの (ID, 印の行), 空・閉じ括弧の無い印の (行, 行の文字))。行は1始まり
    pub fn markers_before(
        &self,
        first_line: usize,
    ) -> (Vec<(String, usize)>, Vec<(usize, String)>) {
        let mut ids = Vec::new();
        let mut invalid = Vec::new();
        let mut line = first_line;
        let mut block = Vec::new();
        while line > 0 {
            line -= 1;
            match self.class_of(line) {
                LineClass::Comment | LineClass::Allowed => block.push(line),
                LineClass::Blank | LineClass::Code => break,
            }
        }
        block.reverse();
        for line in block {
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
