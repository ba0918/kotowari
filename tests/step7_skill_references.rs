//! スキル kotowari の references に写した本体の値が、本体のコードと一致することの検査
//! （REQ-125、REQ-126、REQ-127）。突き合わせる相手は本体のコードが持つ値で、IR の表ではない。

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

// --- REQ-125: 指摘の種類の一致 ---

// @kotowari[REQ-125]
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

// --- REQ-126: 既定の一致 ---

// @kotowari[REQ-126]
#[test]
fn req_126_config_reference_setup_yaml_parses_to_the_defaults() {
    let yaml = only_yaml_block(&read_reference("config.md"));
    // 不在の鍵は既定で埋まるので、9個の鍵の経路が YAML に書かれていることを先に見る（TBL-004）
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

// --- REQ-127: 停止の文言の一致 ---

// @kotowari[REQ-127]
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

// --- skill-references.md の具体例: 写しがずれると一致のテストが落ちる ---

// @kotowari[REQ-125, EX-036]
#[test]
fn req_125_a_missing_kind_row_breaks_the_match() {
    let reference = read_reference("findings.md");
    let row = reference
        .lines()
        .find(|l| l.starts_with("| duplicate_term |"))
        .expect("the reference should have the duplicate_term row");
    let without_the_row = reference.replace(&format!("{row}\n"), "");
    let in_reference = first_column_of_table(&without_the_row, "種類");
    let in_code: BTreeSet<String> = kotowari::FindingKind::ALL
        .iter()
        .map(|k| k.as_str().to_string())
        .collect();
    assert_ne!(
        in_reference, in_code,
        "a kind missing from the reference table must break the match the kinds test makes"
    );
}

// @kotowari[REQ-126, EX-037]
#[test]
fn req_126_a_changed_default_in_the_setup_yaml_breaks_the_match() {
    let yaml = only_yaml_block(&read_reference("config.md")).replace("lines: 200", "lines: 120");
    let parsed = kotowari::config::Config::parse(&yaml).expect("still a readable configuration");
    assert_ne!(
        parsed,
        kotowari::config::Config::default(),
        "a default that drifts from the code must break the match the defaults test makes"
    );
}

// @kotowari[REQ-126, EX-043]
#[test]
fn req_126_a_key_dropped_from_the_setup_yaml_breaks_the_match() {
    let yaml = only_yaml_block(&read_reference("config.md")).replace("  requirements: 10\n", "");
    let tree: serde_json::Value =
        serde_saphyr::from_str(&yaml).expect("still a readable tree");
    assert!(
        tree.get("limits").and_then(|l| l.get("requirements")).is_none(),
        "the key is gone from the YAML"
    );
    // 鍵が書かれていることを見る確かめが落ちる。本体の既定は 10 のまま
    assert_eq!(kotowari::config::Config::default().limits.requirements.get(), 10);
}

// @kotowari[REQ-127, EX-039]
#[test]
fn req_127_a_changed_stop_wording_breaks_the_match() {
    let reference = read_reference("findings.md")
        .replace("| config error |", "| configuration error |");
    let in_reference = first_column_of_table(&reference, "文言");
    let in_code: BTreeSet<String> = kotowari::StopReason::WORDINGS
        .iter()
        .map(|wording| wording.to_string())
        .collect();
    assert_ne!(
        in_reference, in_code,
        "a wording that drifts from the code must break the match the wordings test makes"
    );
}
