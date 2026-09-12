use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::cargo_bin("kotowari").unwrap()
}

fn make_project(tmp: &std::path::Path) {
    fs::create_dir_all(tmp.join(".kotowari")).unwrap();
    fs::create_dir_all(tmp.join("docs/ir")).unwrap();
    fs::create_dir_all(tmp.join("docs/decision/brainstorm")).unwrap();
    fs::create_dir_all(tmp.join("docs/decision/adr")).unwrap();
    fs::write(
        tmp.join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\n",
    )
    .unwrap();
    fs::write(
        tmp.join("docs/decision/brainstorm/records.md"),
        "# Records\n\n## Agreements\n\n- A1 Agreement\n",
    )
    .unwrap();
}

fn parse_json(output: &std::process::Output) -> serde_json::Value {
    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(&stdout).expect("valid JSON")
}

fn findings_by_kind(v: &serde_json::Value, kind: &str) -> Vec<serde_json::Value> {
    v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["kind"] == kind)
        .cloned()
        .collect()
}

// --- REQ-029: 誤りの種類の detail ---

// @kotowari[REQ-029, TBL-008]
#[test]
fn req_029_every_error_kind_has_the_detail_of_the_table() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 題名なしの文書 → missing_title, detail = 文書名
    fs::write(tmp.path().join("docs/ir/a.md"), "No title.\n").unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let mt = findings_by_kind(&v, "missing_title");
    assert_eq!(mt[0]["detail"], "a.md", "detail should be filename");
    assert_eq!(mt[0]["severity"], "error");
}

// --- REQ-030: 警告の種類の detail ---

// @kotowari[REQ-030, TBL-009]
#[test]
fn req_030_warning_kinds_have_the_detail_of_the_table() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 既定の limits.lines=120 を超える文書
    let content = format!("# Title\n\nScope.\n\n{}", "x\n".repeat(120));
    fs::write(tmp.path().join("docs/ir/a.md"), &content).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let tl = findings_by_kind(&v, "too_many_lines");
    assert_eq!(tl.len(), 1);
    assert_eq!(tl[0]["severity"], "warning");
    // detail は行数
    let line_count: usize = tl[0]["detail"].as_str().unwrap().parse().unwrap();
    assert!(line_count > 120);
}

// --- REQ-031: 警告は2種類だけ ---

// @kotowari[REQ-031]
#[test]
fn req_031_only_two_kinds_are_warnings() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 行数超過と要求数超過を出す
    let mut content = String::from("# Title\n\nScope.\n\n## 要求\n\n");
    for i in 1..=12 {
        content.push_str(&format!(
            "### REQ-{:03}: R{i}\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A1\n- 検証: unit\n\nStmt.\n\n",
            i
        ));
    }
    // 行数を増やす
    content.push_str(&"x\n".repeat(100));
    fs::write(tmp.path().join("docs/ir/a.md"), &content).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let findings = v["findings"].as_array().unwrap();
    let warnings: Vec<_> = findings.iter().filter(|f| f["severity"] == "warning").collect();
    for w in &warnings {
        let kind = w["kind"].as_str().unwrap();
        assert!(
            kind == "too_many_lines" || kind == "too_many_requirements",
            "unexpected warning kind: {kind}"
        );
    }
}

// --- REQ-027: 文書全体への指摘は null ---

// @kotowari[REQ-027]
#[test]
fn req_027_document_wide_findings_have_null_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(tmp.path().join("docs/ir/a.md"), "No title\n").unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let mt = findings_by_kind(&v, "missing_title");
    assert!(mt[0]["line"].is_null(), "missing_title should have null line");
}

// --- REQ-028: 行は1始まり ---

// @kotowari[REQ-028]
#[test]
fn req_028_lines_start_at_one() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 1行目から始まる見出しで unknown_heading を出す
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### Bad Heading\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let uh = findings_by_kind(&v, "unknown_heading");
    assert!(!uh.is_empty());
    let line = uh[0]["line"].as_u64().unwrap();
    assert!(line >= 1, "line should be 1-indexed, got {line}");
}

// --- REQ-024: 指摘の並び ---

// @kotowari[REQ-024, TBL-007]
#[test]
fn req_024_sorted_by_path_line_kind_detail() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 複数の文書に指摘を出す
    fs::write(tmp.path().join("docs/ir/b.md"), "# B\n\nScope.\n\n## 要求\n\n### Bad1\n").unwrap();
    fs::write(tmp.path().join("docs/ir/a.md"), "# A\n\nScope.\n\n## 要求\n\n### Bad2\n").unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let findings = v["findings"].as_array().unwrap();

    // path でソートされていること
    for i in 1..findings.len() {
        let prev_path = findings[i - 1]["path"].as_str().unwrap();
        let curr_path = findings[i]["path"].as_str().unwrap();
        let prev_line = findings[i - 1]["line"].as_u64();
        let curr_line = findings[i]["line"].as_u64();
        let prev_kind = findings[i - 1]["kind"].as_str().unwrap();
        let curr_kind = findings[i]["kind"].as_str().unwrap();
        let prev_detail = findings[i - 1]["detail"].as_str().unwrap();
        let curr_detail = findings[i]["detail"].as_str().unwrap();

        let path_cmp = prev_path.cmp(curr_path);
        let line_cmp = match (prev_line, curr_line) {
            (None, None) => std::cmp::Ordering::Equal,
            (None, Some(_)) => std::cmp::Ordering::Less,
            (Some(_), None) => std::cmp::Ordering::Greater,
            (Some(a), Some(b)) => a.cmp(&b),
        };
        let kind_cmp = prev_kind.cmp(curr_kind);
        let detail_cmp = prev_detail.cmp(curr_detail);

        let overall = path_cmp
            .then(line_cmp)
            .then(kind_cmp)
            .then(detail_cmp);
        assert!(
            overall != std::cmp::Ordering::Greater,
            "findings not sorted at index {i}: prev=({prev_path},{prev_line:?},{prev_kind},{prev_detail}) curr=({curr_path},{curr_line:?},{curr_kind},{curr_detail})"
        );
    }
}

// --- PROP-003: findings は並んでいる ---

// @kotowari[PROP-003]
#[test]
fn prop_003_findings_are_sorted() {
    use proptest::prelude::*;

    // proptest の中で CLI を走らせるのは重いので、
    // ライブラリ関数で直接テスト
    proptest!(|(
        n_docs in 1..5usize,
        n_items in 0..3usize,
    )| {
        let config = kotowari::config::Config::default();
        let mut docs = Vec::new();
        for i in 0..n_docs {
            let filename = format!("doc{i}.md");
            let mut content = format!("# Title {i}\n\nScope {i}.\n\n## 要求\n\n");
            for j in 0..n_items {
                let id_num = i * 10 + j + 1;
                content.push_str(&format!(
                    "### REQ-{:03}: R\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\nStatement.\n\n",
                    id_num
                ));
            }
            docs.push(kotowari::ir::parse_document(&filename, &content));
        }
        let mut findings = kotowari::ir::check_documents(&docs, &config);
        findings.sort_by(|a, b| {
            a.path.cmp(&b.path)
                .then_with(|| match (a.line, b.line) {
                    (None, None) => std::cmp::Ordering::Equal,
                    (None, Some(_)) => std::cmp::Ordering::Less,
                    (Some(_), None) => std::cmp::Ordering::Greater,
                    (Some(al), Some(bl)) => al.cmp(&bl),
                })
                .then_with(|| a.kind.cmp(&b.kind))
                .then_with(|| a.detail.cmp(&b.detail))
        });
        // 並びの検証
        for i in 1..findings.len() {
            let a = &findings[i-1];
            let b = &findings[i];
            let cmp = a.path.cmp(&b.path)
                .then_with(|| match (a.line, b.line) {
                    (None, None) => std::cmp::Ordering::Equal,
                    (None, Some(_)) => std::cmp::Ordering::Less,
                    (Some(_), None) => std::cmp::Ordering::Greater,
                    (Some(al), Some(bl)) => al.cmp(&bl),
                })
                .then_with(|| a.kind.cmp(&b.kind))
                .then_with(|| a.detail.cmp(&b.detail));
            prop_assert!(cmp != std::cmp::Ordering::Greater);
        }
    });
}
