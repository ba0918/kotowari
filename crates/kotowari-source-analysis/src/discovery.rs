use crate::comment_block::LineMap;
use crate::test_queries::{ParsedFile, TestQueries};
use ast_grep_core::{Node, tree_sitter::StrDoc};
use ast_grep_language::SupportLang;
use kotowari_core::{config::Config, tests_discovery::DiscoveredTest};
pub fn discover_tests(
    content: &str,
    file_rel: &str,
    lang: SupportLang,
    queries: &TestQueries,
    config: &Config,
) -> Result<Vec<DiscoveredTest>, String> {
    let parsed =
        ParsedFile::parse(content, lang).ok_or_else(|| format!("syntax error in {file_rel}"))?;
    let root = parsed.root();
    let lines = LineMap::new(content, &root, lang);
    // `テスト`ごとの、その節が始まるバイトの位置
    let mut tests: Vec<(usize, DiscoveredTest)> = Vec::new();
    for test in parsed.find_tests(queries, lang, file_rel) {
        let first_line = test.node.start_pos().line();
        let (marker_ids, invalid_markers) = lines.markers_before(first_line);
        tests.push((
            test.node.range().start,
            DiscoveredTest {
                name: test.name,
                first_line_text: line_text(content, first_line),
                file_path: file_rel.to_string(),
                line: first_line + 1,
                marker_ids,
                invalid_markers,
            },
        ));
    }
    if lang == SupportLang::Rust {
        discover_macro_tests(&root, content, &lines, file_rel, config, &mut tests);
    }
    tests.sort_by_key(|(start, _)| *start);
    Ok(unbind_later_tests_on_the_same_line(tests))
}

/// 最初の行が同じ`テスト`が2つ以上あるとき、印はその行で最初に始まる`テスト`にだけ結び付ける
/// （TBL-core-016）。tests は節の始まる位置の順に並んでいる
fn unbind_later_tests_on_the_same_line(tests: Vec<(usize, DiscoveredTest)>) -> Vec<DiscoveredTest> {
    let mut previous_line = None;
    tests
        .into_iter()
        .map(|(_, mut test)| {
            if previous_line == Some(test.line) {
                test.marker_ids.clear();
                test.invalid_markers.clear();
            }
            previous_line = Some(test.line);
            test
        })
        .collect()
}

/// 行の全体の文字から前後の空白を除いたもの（REQ-core-086 の名前が null のときの detail）
fn line_text(content: &str, line: usize) -> String {
    content.lines().nth(line).unwrap_or("").trim().to_string()
}

/// "tests.rust.macros" のマクロを探し、中身を Rust の項目として読み直す（TBL-core-017）
fn discover_macro_tests(
    node: &RustNode<'_>,
    source: &str,
    lines: &LineMap<'_>,
    file_rel: &str,
    config: &Config,
    tests: &mut Vec<(usize, DiscoveredTest)>,
) {
    for child in node.children() {
        match child.kind().as_ref() {
            "macro_invocation" => {
                if is_configured_macro(&child, config) {
                    reparse_macro_body(&child, source, lines, file_rel, tests);
                }
            }
            _ => discover_macro_tests(&child, source, lines, file_rel, config, tests),
        }
    }
}

type RustNode<'r> = Node<'r, StrDoc<SupportLang>>;

/// マクロの名前の末尾の要素が設定のどれかと一致するか
fn is_configured_macro(node: &RustNode<'_>, config: &Config) -> bool {
    let Some(macro_node) = node.field("macro") else {
        return false;
    };
    let macro_name = macro_node.text();
    let last_segment = macro_name.rsplit("::").next().unwrap_or(&macro_name);
    config.tests.rust.macros.iter().any(|m| {
        let m_last = m.rsplit("::").next().unwrap_or(m);
        m_last == last_segment
    })
}

fn reparse_macro_body(
    node: &RustNode<'_>,
    source: &str,
    lines: &LineMap<'_>,
    file_rel: &str,
    tests: &mut Vec<(usize, DiscoveredTest)>,
) {
    let Some(body) = node.children().find(|c| c.kind() == "token_tree") else {
        return;
    };
    let body_text = body.text();
    // token_tree の中身（"{ ... }"、"( ... )"、"[ ... ]" の中）
    let delimited = [('{', '}'), ('(', ')'), ('[', ']')]
        .into_iter()
        .find_map(|(open, close)| body_text.strip_prefix(open)?.strip_suffix(close));
    let inner = delimited.unwrap_or(&body_text);
    let byte_offset = body.range().start + usize::from(delimited.is_some());
    let line_offset = source[..byte_offset].matches('\n').count();

    // REQ-core-083: 読み直したときの構文の誤りは unparsable_file にせず、読めた関数を数える
    let Ok(inner_root) = ast_grep_core::AstGrep::try_new(inner, SupportLang::Rust) else {
        return;
    };
    let inner_lines = LineMap::new(inner, &inner_root.root(), SupportLang::Rust);
    collect_macro_functions(
        &inner_root.root(),
        inner,
        (lines, &inner_lines),
        file_rel,
        (byte_offset, line_offset),
        tests,
    );
}

/// マクロの中の最上位の関数ごとに数える（入れ子の関数は数えない）
fn collect_macro_functions(
    node: &RustNode<'_>,
    inner: &str,
    (lines, inner_lines): (&LineMap<'_>, &LineMap<'_>),
    file_rel: &str,
    (byte_offset, line_offset): (usize, usize),
    tests: &mut Vec<(usize, DiscoveredTest)>,
) {
    for child in node.children() {
        if child.kind() != "function_item" {
            let offset = (byte_offset, line_offset);
            let line_maps = (lines, inner_lines);
            collect_macro_functions(&child, inner, line_maps, file_rel, offset, tests);
            continue;
        }
        let Some(name_node) = child.field("name") else {
            continue;
        };
        let first_line = child.start_pos().line();
        // A26: マクロの外と同じ行の規則で結び付ける。中身の最初の行はマクロの開き括弧と同じ行なので、
        // その上の行はマクロの外にあり、ファイルの行で塊を探す
        let (marker_ids, invalid_markers) = if first_line == 0 {
            lines.markers_before(line_offset)
        } else {
            let (ids, invalid) = inner_lines.markers_before(first_line);
            (
                ids.into_iter()
                    .map(|(id, ln)| (id, line_offset + ln))
                    .collect(),
                invalid
                    .into_iter()
                    .map(|(ln, raw)| (line_offset + ln, raw))
                    .collect(),
            )
        };
        let test = DiscoveredTest {
            // A38: 名前は関数の名前
            name: Some(name_node.text().to_string()),
            first_line_text: line_text(inner, first_line),
            file_path: file_rel.to_string(),
            line: line_offset + first_line + 1,
            marker_ids,
            invalid_markers,
        };
        tests.push((byte_offset + child.range().start, test));
    }
}

/// 単独の "\r" を "\n" に置き換える。TBL-core-010 は単独の "\r" も行の終わりに数えるが、
/// `str::lines()` も tree-sitter の行も "\n" でしか行を分けない。どちらも1バイトなので、
/// 置き換えてもバイトの位置は変わらない
pub(crate) fn lone_cr_to_lf(content: &str) -> String {
    let mut out = String::with_capacity(content.len());
    let mut chars = content.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\r' && chars.peek() != Some(&'\n') {
            out.push('\n');
        } else {
            out.push(c);
        }
    }
    out
}
