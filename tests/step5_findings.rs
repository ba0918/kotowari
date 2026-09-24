use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::cargo_bin("kotowari").unwrap()
}

fn make_project(tmp: &std::path::Path) {
    fs::create_dir_all(tmp.join(".kotowari")).unwrap();
    fs::create_dir_all(tmp.join("docs/ir")).unwrap();
    fs::create_dir_all(tmp.join("docs/decision/records")).unwrap();
    fs::create_dir_all(tmp.join("docs/decision/adr")).unwrap();
    fs::write(
        tmp.join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\n",
    )
    .unwrap();
    fs::write(
        tmp.join("docs/decision/records/records.md"),
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

// --- REQ-core-025: 文字の出力に使う種類の語 ---

// @kotowari[REQ-core-025]
#[test]
fn req_025_finding_kind_displays_as_its_own_kind_word() {
    // --format text の出力は "{kind}" を通して種類の語を書く（main.rs）。
    // その Display の実装が種類ごとの語を書くことを直接確かめる。
    assert_eq!(
        kotowari_core::FindingKind::MissingTitle.to_string(),
        "missing_title",
        "Display should write the kind word, not an empty string"
    );
    assert_eq!(
        kotowari_core::FindingKind::SourceInvalid.to_string(),
        "source_invalid"
    );
}

// --- REQ-core-024: FindingKind と str の等価 ---

// @kotowari[REQ-core-024]
#[test]
fn req_024_finding_kind_partial_eq_str_matches_only_its_own_kind_name() {
    // `==` に書いた文字列リテラルは `PartialEq<&str>` を通る（既存のテストで検査済み）。
    // ここでは `PartialEq<str>` 側の実装を `.eq()` で直接呼び、
    // 自分の種類の名前だけに真を返すことを確かめる。
    assert!(
        kotowari_core::FindingKind::MissingDocument.eq("missing_document"),
        "should equal its own kind name"
    );
    assert!(
        !kotowari_core::FindingKind::MissingDocument.eq("missing_title"),
        "should not equal a different kind name"
    );
}

// --- REQ-core-029: 誤りの種類の detail ---

// @kotowari[REQ-core-029, TBL-core-008]
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

// --- REQ-core-030: 警告の種類の detail ---

// @kotowari[REQ-core-030, TBL-core-009]
#[test]
fn req_030_notice_kinds_have_the_detail_of_the_table() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 既定の limits.lines=200 を超える文書
    let content = format!("# Title\n\nScope.\n\n{}", "x\n".repeat(200));
    fs::write(tmp.path().join("docs/ir/a.md"), &content).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let tl = findings_by_kind(&v, "too_many_lines");
    assert_eq!(tl.len(), 1);
    assert_eq!(tl[0]["severity"], "notice");
    // detail は行数
    let line_count: usize = tl[0]["detail"].as_str().unwrap().parse().unwrap();
    assert!(line_count > 200);
}

// --- REQ-core-031: 注意は4種類だけ ---

// @kotowari[REQ-core-031]
#[test]
fn req_031_only_four_kinds_are_notices() {
    // 本体が注意にする種類（"kotowari mutants" の2種類は check の出力には現れない）
    let notices: std::collections::BTreeSet<&str> = kotowari_core::FindingKind::ALL
        .iter()
        .filter(|kind| kind.severity() == "notice")
        .map(|kind| kind.as_str())
        .collect();
    assert_eq!(
        notices,
        std::collections::BTreeSet::from([
            "equivalent_stale",
            "mutant_timeout",
            "too_many_lines",
            "too_many_requirements",
        ])
    );
    // ほかの種類はすべて誤り
    for kind in kotowari_core::FindingKind::ALL {
        if !notices.contains(kind.as_str()) {
            assert_eq!(kind.severity(), "error", "{kind} should be an error");
        }
    }
}

// @kotowari[REQ-core-031]
#[test]
fn req_031_check_reports_no_notice_outside_the_four_kinds() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 行数超過（>200行）と要求数超過（>10件）を出す
    let mut content = String::from("# Title\n\nScope.\n\n## Requirements\n\n");
    for i in 1..=12 {
        content.push_str(&format!(
            "### REQ-{:03}: R{i}\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: unit\n\nStmt.\n\n",
            i
        ));
    }
    // 行数を増やす
    content.push_str(&"x\n".repeat(200));
    fs::write(tmp.path().join("docs/ir/a.md"), &content).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let findings = v["findings"].as_array().unwrap();
    let notices: Vec<_> = findings
        .iter()
        .filter(|f| f["severity"] == "notice")
        .collect();
    // 警告が実際に存在すること
    assert!(
        !notices.is_empty(),
        "test should produce at least one notice"
    );
    // 注意は4種類のどれかだけ
    for w in &notices {
        let kind = w["kind"].as_str().unwrap();
        assert!(
            matches!(
                kind,
                "too_many_lines" | "too_many_requirements" | "mutant_timeout" | "equivalent_stale"
            ),
            "unexpected notice kind: {kind}"
        );
    }
    // 警告でない指摘はすべて error
    for f in findings {
        let sev = f["severity"].as_str().unwrap();
        if sev != "notice" {
            assert_eq!(
                sev, "error",
                "non-notice finding should be error, got {sev}: {:?}",
                f
            );
        }
    }
}

// --- REQ-core-027: 文書全体への指摘は null ---

// @kotowari[REQ-core-027]
#[test]
fn req_027_document_wide_findings_have_null_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(tmp.path().join("docs/ir/a.md"), "No title\n").unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let mt = findings_by_kind(&v, "missing_title");
    assert!(
        mt[0]["line"].is_null(),
        "missing_title should have null line"
    );
}

// --- REQ-core-028: 行は1始まり ---

// @kotowari[REQ-core-028]
#[test]
fn req_028_lines_start_at_one() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 7行目の見出しで unknown_heading を出す
    // line 1: # Title
    // line 2: (空)
    // line 3: Scope.
    // line 4: (空)
    // line 5: ## Requirements
    // line 6: (空)
    // line 7: ### Bad Heading
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## Requirements\n\n### Bad Heading\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let uh = findings_by_kind(&v, "unknown_heading");
    assert!(!uh.is_empty());
    let line = uh[0]["line"].as_u64().unwrap();
    assert_eq!(line, 7, "### Bad Heading is on line 7 (1-indexed)");
}

// --- REQ-core-024: 指摘の並び ---

// @kotowari[REQ-core-024, TBL-core-007]
#[test]
fn req_024_sorted_by_path_line_kind_detail() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 複数の文書に指摘を出す
    fs::write(
        tmp.path().join("docs/ir/b.md"),
        "# B\n\nScope.\n\n## Requirements\n\n### Bad1\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# A\n\nScope.\n\n## Requirements\n\n### Bad2\n",
    )
    .unwrap();
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

        let overall = path_cmp.then(line_cmp).then(kind_cmp).then(detail_cmp);
        assert!(
            overall != std::cmp::Ordering::Greater,
            "findings not sorted at index {i}: prev=({prev_path},{prev_line:?},{prev_kind},{prev_detail}) curr=({curr_path},{curr_line:?},{curr_kind},{curr_detail})"
        );
    }
}

// --- PROP-core-003: findings は並んでいる ---

// @kotowari[PROP-core-003]
#[test]
fn prop_003_findings_are_sorted() {
    use proptest::prelude::*;

    // run_check を通して、ライブラリが返す findings が
    // TBL-core-007（path → line → kind → detail）で並んでいることを検証する。
    //
    // IR 検査の指摘（後のファイル名の文書）とテスト発見の指摘（前のファイル
    // 名のテストファイル）が交互に追加されるため、ソートしないと壊れる。
    let config = proptest::test_runner::Config {
        cases: 32,
        ..Default::default()
    };
    proptest!(config, |(
        n_docs in 2..5usize,
        n_items in 1..3usize,
    )| {
        let tmp = tempfile::TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
        fs::create_dir_all(tmp.path().join("docs/ir")).unwrap();
        fs::create_dir_all(tmp.path().join("docs/decision/records")).unwrap();
        fs::create_dir_all(tmp.path().join("docs/decision/adr")).unwrap();
        fs::create_dir_all(tmp.path().join("tests")).unwrap();
        fs::write(
            tmp.path().join(".kotowari/config.yaml"),
            "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\n",
        ).unwrap();
        fs::write(
            tmp.path().join("docs/decision/records/records.md"),
            "# Records\n\n## Agreements\n\n- A1 Agreement\n",
        ).unwrap();
        for i in 0..n_docs {
            let filename = format!("doc{i}.md");
            let mut content = format!("# Title {i}\n\nScope {i}.\n\n## Requirements\n\n");
            for j in 0..n_items {
                let id_num = i * 10 + j + 1;
                // 解決できない出典を使い source_invalid を出す。
                // requirement_without_test（テスト発見モジュール）と交互に
                // 追加されるため、ソートしないと順序が壊れる。
                content.push_str(&format!(
                    "### REQ-{:03}: R\n\n- kind: ubiquitous\n- source: nonexistent/path#X{}\n- verification: unit\n\nStatement.\n\n",
                    id_num, id_num
                ));
            }
            fs::write(tmp.path().join(format!("docs/ir/{filename}")), &content).unwrap();
        }
        // 印の無いテストファイルを置く（test_without_id が tests/ パスで出る）
        fs::write(
            tmp.path().join("tests/check.rs"),
            "#[test]\nfn unmarked() {}\n",
        ).unwrap();
        let (result, _) = kotowari_core::run_check(
            tmp.path(),
            kotowari_core::Format::Json,
            None,
        ).expect("run_check should succeed");
        // findings が TBL-core-007 の順で並んでいることを検証する
        let findings = &result.findings;
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
                .then_with(|| a.kind.as_str().cmp(b.kind.as_str()))
                .then_with(|| a.detail.cmp(&b.detail));
            prop_assert!(cmp != std::cmp::Ordering::Greater,
                "findings not sorted at index {}: prev=({},{:?},{},{}) curr=({},{:?},{},{})",
                i, a.path, a.line, a.kind, a.detail, b.path, b.line, b.kind, b.detail);
        }
    });
}

// --- Step 7: 指摘の種類と行の表 ---

// @kotowari[REQ-core-027]
#[test]
fn req_027_glossary_invalid_has_null_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 用語集にヘッダの列名が違う表しかない → glossary_invalid
    fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# Glossary\n\n| Name | Meaning | Source |\n|---|---|---|\n| test | meaning | docs/decision/records/records.md#A1 |\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let gi = findings_by_kind(&v, "glossary_invalid");
    assert!(!gi.is_empty(), "should produce glossary_invalid: {:?}", gi);
    assert!(
        gi[0]["line"].is_null(),
        "glossary_invalid line should be null"
    );
}

// @kotowari[TBL-core-019]
#[test]
fn tbl_019_unclosed_code_block_line_is_the_opening_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## Requirements\n\n### REQ-001: R\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: unit\n\n```\nunclosed\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let uc = findings_by_kind(&v, "unclosed_code_block");
    assert!(!uc.is_empty(), "should produce unclosed_code_block");
    assert_eq!(uc[0]["line"], 13, "line should be the opening line (13)");
}

// @kotowari[TBL-core-019]
#[test]
fn tbl_019_invalid_gherkin_line_and_invalid_id_lines() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## Requirements\n\n### REQ-001: R\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: unit\n\nStatement.\n\n## Examples\n\n```gherkin\n@id=BADID @about=REQ-001 @source=docs/decision/records/records.md#A1\nScenario: Test\n  Given something\nFeature: bad\n```\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ig = findings_by_kind(&v, "invalid_gherkin_line");
    assert!(!ig.is_empty(), "should have invalid_gherkin_line");
    // invalid_gherkin_line の line はその行
    assert_eq!(
        ig[0]["line"], 21,
        "invalid_gherkin_line should be on its own line (21)"
    );
    let ii = findings_by_kind(&v, "invalid_id");
    assert!(!ii.is_empty(), "should have invalid_id");
    // invalid_id の line はタグの行
    assert_eq!(
        ii[0]["line"], 18,
        "invalid_id line should be the tag line (18)"
    );
}

// @kotowari[TBL-core-019]
#[test]
fn tbl_019_unclosed_backtick_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# Glossary\n\n| Term | Meaning | Source |\n|---|---|---|\n| IR | 仕様 | docs/decision/records/records.md#A1 |\n",
    ).unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## Requirements\n\n### REQ-001: R\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: unit\n\n`奇数のバッククォート。\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ub = findings_by_kind(&v, "unclosed_backtick");
    assert!(!ub.is_empty(), "should produce unclosed_backtick");
    assert_eq!(
        ub[0]["line"], 13,
        "unclosed_backtick should be on its own line (13)"
    );
}

// @kotowari[TBL-core-019]
#[test]
fn tbl_019_marker_findings_line_is_the_marker_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    // invalid_marker の line は印の行
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[]\n#[test]\nfn test_a() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let im = findings_by_kind(&v, "invalid_marker");
    assert!(!im.is_empty(), "should have invalid_marker");
    assert_eq!(
        im[0]["line"], 1,
        "invalid_marker line should be the marker line (1)"
    );
}

// @kotowari[TBL-core-019]
#[test]
fn tbl_019_source_invalid_line_is_the_source_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## Requirements\n\n### REQ-001: R\n\n- kind: ubiquitous\n- source: somewhere/bad.md#X\n- verification: unit\n\nStatement.\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    assert!(!si.is_empty(), "should have source_invalid");
    // line は出典の行（10行目）
    assert_eq!(
        si[0]["line"], 10,
        "source_invalid line should be the source line (10)"
    );
}

// --- REQ-core-174: 宣言の外の行・コードブロック・用語集の題名 ---

/// 形の指摘だけを見るための、ほかの検査を通る話題ごとの文書。
const TOPIC_HEAD: &str = "# A\n\n文書が扱う範囲。\n";

// @kotowari[REQ-core-174]
#[test]
fn req_174_a_line_outside_the_declaration_below_a_section_heading_is_unknown_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        format!("{TOPIC_HEAD}\n## Requirements\n\n節の直下の素の行。\n"),
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let found = findings_by_kind(&v, "unknown_line");
    assert_eq!(found.len(), 1, "{:?}", v["findings"]);
    assert_eq!(
        found[0]["detail"], "節の直下の素の行。",
        "detail は行の文字"
    );
    assert_eq!(found[0]["line"], 7, "\"line\" はその行");
    assert_eq!(found[0]["severity"], "error");
    assert_eq!(output.status.code(), Some(1));
}

// @kotowari[REQ-core-174]
#[test]
fn req_174_an_undeclared_table_or_code_block_is_also_unknown_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        format!(
            "{TOPIC_HEAD}\n## Requirements\n\n| a | b |\n|---|---|\n| 1 | 2 |\n\n```text\nx\n```\n"
        ),
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let found = findings_by_kind(&v, "unknown_line");
    let lines: Vec<&serde_json::Value> = found.iter().map(|f| &f["line"]).collect();
    assert_eq!(lines, vec![&serde_json::json!(7), &serde_json::json!(11)]);
}

// @kotowari[REQ-core-174]
#[test]
fn req_174_a_non_gherkin_code_block_under_examples_is_unknown_code_block() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        format!("{TOPIC_HEAD}\n## Examples\n\n```text\nScenario: 例\n```\n"),
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let found = findings_by_kind(&v, "unknown_code_block");
    assert_eq!(found.len(), 1, "{:?}", v["findings"]);
    assert_eq!(found[0]["detail"], "```text", "detail は開始の行の文字");
    assert_eq!(found[0]["line"], 7, "\"line\" は開始の行");
    assert_eq!(found[0]["severity"], "error");
}

// @kotowari[REQ-core-174]
#[test]
fn req_174_a_glossary_title_outside_the_declared_form_is_glossary_title_invalid() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# 用語の一覧\n\n| Term | Meaning | Source |\n|---|---|---|\n| 印 | しるし | docs/decision/records/records.md#A1 |\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let found = findings_by_kind(&v, "glossary_title_invalid");
    assert_eq!(found.len(), 1, "{:?}", v["findings"]);
    assert_eq!(
        found[0]["detail"], "# 用語の一覧",
        "detail は題名の行の文字"
    );
    assert_eq!(found[0]["line"], 1, "\"line\" は題名の行");
    assert_eq!(found[0]["severity"], "error");
}

// @kotowari[EX-core-266]
#[test]
fn ex_core_266_the_three_places_outside_the_declaration_each_become_an_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        format!("{TOPIC_HEAD}\n## Requirements\n\n節の直下の素の行。\n"),
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/ir/b.md"),
        format!("{TOPIC_HEAD}\n## Examples\n\n```text\nScenario: 例\n```\n"),
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# 用語の一覧\n\n| Term | Meaning | Source |\n|---|---|---|\n| 印 | しるし | docs/decision/records/records.md#A1 |\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    for (kind, detail, line) in [
        ("unknown_line", "節の直下の素の行。", 7),
        ("unknown_code_block", "```text", 7),
        ("glossary_title_invalid", "# 用語の一覧", 1),
    ] {
        let found = findings_by_kind(&v, kind);
        assert_eq!(found.len(), 1, "{kind} は1件: {:?}", v["findings"]);
        assert_eq!(found[0]["detail"], detail, "{kind} の detail");
        assert_eq!(found[0]["line"], line, "{kind} の \"line\"");
    }
}

// @kotowari[REQ-core-176, EX-core-269]
#[test]
fn ex_core_269_a_document_with_a_form_finding_still_takes_the_cross_document_checks() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // a.md の要求は "- verification:" の行を欠き、その ID が b.md の要求と重なる
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        format!("{TOPIC_HEAD}\n## Requirements\n\n### REQ-001: 前\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n\n文。\n"),
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/ir/b.md"),
        format!("{TOPIC_HEAD}\n## Requirements\n\n### REQ-001: 後\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: review\n- how_to_verify: 見る\n\n文。\n"),
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let missing = findings_by_kind(&v, "verification_missing");
    assert_eq!(missing.len(), 1, "{:?}", v["findings"]);
    assert_eq!(missing[0]["path"], "docs/ir/a.md");
    let duplicates = findings_by_kind(&v, "duplicate_id");
    assert_eq!(duplicates.len(), 1, "{:?}", v["findings"]);
    assert_eq!(duplicates[0]["path"], "docs/ir/b.md");
    assert_eq!(duplicates[0]["detail"], "REQ-001");
}

// @kotowari[REQ-core-168, EX-core-264]
#[test]
fn ex_core_264_moving_the_schema_files_away_does_not_change_the_output() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        format!("{TOPIC_HEAD}\n## Requirements\n\n### REQ-001: 例\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n\n文。\n"),
    )
    .unwrap();
    // スキーマのファイルの置き場。中身は読まれないので、読めない形にしておく
    fs::create_dir_all(tmp.path().join(".kotowari/schemas")).unwrap();
    for name in ["ir.yaml", "context.yaml", "flags.yaml"] {
        fs::write(
            tmp.path().join(".kotowari/schemas").join(name),
            "not: [a schema",
        )
        .unwrap();
    }
    let run = || {
        cmd()
            .args(["check", "--format", "json"])
            .current_dir(tmp.path())
            .output()
            .unwrap()
    };
    let before = run();
    fs::rename(
        tmp.path().join(".kotowari/schemas"),
        tmp.path().join("moved-away"),
    )
    .unwrap();
    let after = run();
    assert_eq!(before.status.code(), after.status.code());
    assert_eq!(before.stdout, after.stdout);
    // 検査は実際に行われている（verification_missing が出る）
    assert!(!findings_by_kind(&parse_json(&after), "verification_missing").is_empty());
}

// @kotowari[EX-core-267]
#[test]
fn ex_core_267_the_ir_of_this_repository_has_none_of_the_three() {
    let output = cmd()
        .arg("check")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .unwrap();
    let v = parse_json(&output);
    for kind in [
        "unknown_line",
        "unknown_code_block",
        "glossary_title_invalid",
    ] {
        assert!(
            findings_by_kind(&v, kind).is_empty(),
            "{kind} がこのリポジトリの IR で出ている"
        );
    }
}
