//! スキル kotowari の references に写した本体の値が、本体のコードと一致することの検査
//! （REQ-core-125、REQ-core-126、REQ-core-127）。突き合わせる相手は本体のコードが持つ値で、IR の表ではない。

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

fn references_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("skills/kotowari/references")
}

fn read_reference(name: &str) -> String {
    let path = references_dir().join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// ヘッダの1列目が `header` の Markdown の表について、
/// ヘッダと区切りの行を除いた1列目を集める
fn first_column_of_table(markdown: &str, header: &str) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    let mut in_table = false;
    for line in markdown.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('|') {
            in_table = false;
            continue;
        }
        let first = trimmed.split('|').nth(1).unwrap_or("").trim().to_string();
        if first == header {
            in_table = true;
            continue;
        }
        if !in_table {
            continue;
        }
        // 区切りの行
        if !first.is_empty() && first.chars().all(|c| c == '-' || c == ':') {
            continue;
        }
        found.insert(first);
    }
    assert!(
        !found.is_empty(),
        "no table with {header} in its first header cell"
    );
    found
}

/// 言語 yaml のコードブロックの中身を取り出す。2つ以上あればどれを読むかが決まらない
fn only_yaml_block(markdown: &str) -> String {
    let mut blocks: Vec<String> = Vec::new();
    let mut current: Option<Vec<&str>> = None;
    for line in markdown.lines() {
        match current {
            None => {
                if line.trim_end() == "```yaml" {
                    current = Some(Vec::new());
                }
            }
            Some(ref mut body) => {
                if line.trim_end() == "```" {
                    blocks.push(body.join("\n"));
                    current = None;
                } else {
                    body.push(line);
                }
            }
        }
    }
    assert_eq!(
        blocks.len(),
        1,
        "expected exactly one yaml code block, found {}",
        blocks.len()
    );
    blocks.pop().unwrap()
}

// --- REQ-core-125: 指摘の種類の一致 ---

// @kotowari[REQ-core-125]
#[test]
fn req_125_findings_reference_kinds_match_the_code() {
    let in_reference = first_column_of_table(&read_reference("findings.md"), "種類");
    let in_code: BTreeSet<String> = kotowari::FindingKind::ALL
        .iter()
        .map(|k| k.as_str().to_string())
        .collect();
    assert_eq!(
        in_reference, in_code,
        "the kinds in references/findings.md should be exactly the kinds the code emits"
    );
}

// --- REQ-core-126: 既定の一致 ---

// @kotowari[REQ-core-126]
#[test]
fn req_126_config_reference_setup_yaml_parses_to_the_defaults() {
    let yaml = only_yaml_block(&read_reference("config.md"));
    // 不在の鍵は既定で埋まるので、9個の鍵の経路が YAML に書かれていることを先に見る（TBL-core-004）
    let tree: serde_json::Value = serde_saphyr::from_str(&yaml)
        .unwrap_or_else(|e| panic!("the setup YAML should be readable as a tree: {e}"));
    for path in [
        "ir",
        "decisions.records",
        "decisions.adr",
        "tests.files",
        "tests.rust.attributes",
        "tests.rust.macros",
        "limits.lines",
        "limits.requirements",
        "vague_words",
    ] {
        let mut node = &tree;
        for key in path.split('.') {
            node = node
                .get(key)
                .unwrap_or_else(|| panic!("the setup YAML should write the key {path}"));
        }
    }
    let parsed = kotowari::config::Config::parse(&yaml)
        .unwrap_or_else(|e| panic!("the setup YAML should parse as a configuration file: {e}"));
    assert_eq!(
        parsed,
        kotowari::config::Config::default(),
        "the setup YAML in references/config.md should parse to the defaults the code holds"
    );
}

// --- REQ-core-127: 停止の文言の一致 ---

// @kotowari[REQ-core-127]
#[test]
fn req_127_findings_reference_stop_wordings_match_the_code() {
    let in_reference = first_column_of_table(&read_reference("findings.md"), "文言");
    // 標準エラーの1行目は StopReason の Display で、詳細の前がこの文言になる
    let in_code: BTreeSet<String> = kotowari::StopReason::WORDINGS
        .iter()
        .map(|wording| wording.to_string())
        .collect();
    assert_eq!(
        in_reference, in_code,
        "the stop wordings in references/findings.md should be exactly the wordings the code prints"
    );
}
