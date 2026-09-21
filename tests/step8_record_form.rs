//! 判断の記録の形の検査（REQ-core-129〜REQ-core-135、TBL-core-022、REQ-core-132、TBL-core-023）

use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::cargo_bin("kotowari").unwrap()
}

/// テスト用のプロジェクトを作る（設定と、Context を持たない記録と、記録でない Markdown）
fn make_project_with_records(tmp: &std::path::Path) {
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
        "# 判断の記録\n\n## Agreements\n\n- A1 最初の合意\n- A2 二番目の合意\n\n## Prohibitions\n\n- P1 禁止事項\n\n## Delegated\n\n## Rejected\n\n- R1 却下\n",
    )
    .unwrap();
    fs::write(
        tmp.join("docs/decision/records/ir-form.md"),
        "# IR の形の契約\n\n## 文書\n\n文書の形。\n\n## 項目\n\n項目の形。\n",
    )
    .unwrap();
    fs::write(
        tmp.join("docs/decision/adr/0001-test.md"),
        "# ADR 0001: テスト\n\n## 状況\n\n状況の説明。\n\n## 決定\n\n決定。\n",
    )
    .unwrap();
}

/// 記録を1つ足す
fn write_record(tmp: &std::path::Path, name: &str, content: &str) {
    fs::write(tmp.join("docs/decision/records").join(name), content).unwrap();
}

fn parse_json(output: &std::process::Output) -> serde_json::Value {
    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(&stdout).expect("should be valid JSON")
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

fn check(tmp: &std::path::Path) -> serde_json::Value {
    let output = cmd().arg("check").current_dir(tmp).output().unwrap();
    parse_json(&output)
}

// --- REQ-core-130 / TBL-core-022: 必須の補足の行 ---

// @kotowari[REQ-core-130, TBL-core-022, EX-core-101]
#[test]
fn req_130_context_record_without_why_is_record_field_missing() {
    // EX-core-101
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "x.md",
        "# 記録 x\n\n## Context\n\n背景。\n\n## Agreements\n\n- A1 ある合意\n",
    );
    let v = check(tmp.path());
    let found = findings_by_kind(&v, "record_field_missing");
    assert_eq!(found.len(), 1, "one finding for the missing why: {:?}", found);
    assert_eq!(found[0]["path"], "docs/decision/records/x.md");
    assert_eq!(found[0]["line"], 9);
    assert_eq!(found[0]["detail"], "why");
    assert_eq!(found[0]["severity"], "error");
}

// @kotowari[REQ-core-133, TBL-core-022, EX-core-103]
#[test]
fn req_133_not_recorded_passes_and_blank_value_is_missing() {
    // EX-core-103
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "y.md",
        "# 記録 y\n\n## Context\n\n背景。\n\n## Agreements\n\n- A1 最初の合意\n- why: not recorded\n- A2 二番目の合意\n- why:   \n",
    );
    let v = check(tmp.path());
    let found = findings_by_kind(&v, "record_field_missing");
    assert_eq!(
        found.len(),
        1,
        "\"not recorded\" counts as present and a blank value counts as absent: {:?}",
        found
    );
    assert_eq!(found[0]["line"], 11);
    assert_eq!(found[0]["detail"], "why");
}

// @kotowari[REQ-core-130, TBL-core-022, EX-core-105]
#[test]
fn req_130_superseded_line_without_superseded_by_is_missing() {
    // EX-core-105
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "z.md",
        "# 記録 z\n\n## Context\n\n背景。\n\n## Agreements\n\n- A1 ある合意\n- why: x\n\n## Superseded\n\n- A3 置き換えられた決定\n- why: x\n",
    );
    let v = check(tmp.path());
    let found = findings_by_kind(&v, "record_field_missing");
    assert_eq!(found.len(), 1, "the Superseded line needs superseded_by: {:?}", found);
    assert_eq!(found[0]["line"], 14);
    assert_eq!(found[0]["detail"], "superseded_by");
}

// --- REQ-core-131 / TBL-core-022: 知らない名前の補足の行 ---

// @kotowari[REQ-core-131, TBL-core-022, EX-core-104]
#[test]
fn req_131_unknown_field_name_is_record_field_unknown() {
    // EX-core-104
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "u.md",
        "# 記録 u\n\n## Context\n\n背景。\n\n## Agreements\n\n- A1 ある合意\n- why: x\n- reason: x\n",
    );
    let v = check(tmp.path());
    let unknown = findings_by_kind(&v, "record_field_unknown");
    assert_eq!(unknown.len(), 1, "\"reason\" is not one of the six names: {:?}", unknown);
    assert_eq!(unknown[0]["line"], 11);
    assert_eq!(unknown[0]["detail"], "reason");
    assert_eq!(unknown[0]["severity"], "error");
    let missing = findings_by_kind(&v, "record_field_missing");
    assert!(missing.is_empty(), "why is present, so nothing is missing: {:?}", missing);
}

// @kotowari[REQ-core-131, REQ-core-133, TBL-core-019]
#[test]
fn req_131_unknown_name_on_two_lines_yields_two_findings() {
    // 同じ知らない名前が2行あれば2件。値が空の行も名前の検査を受ける（REQ-core-133）
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "t.md",
        "# 記録 t\n\n## Context\n\n背景。\n\n## Agreements\n\n- A1 ある合意\n- why: x\n- reason: x\n- reason:\n",
    );
    let v = check(tmp.path());
    let unknown = findings_by_kind(&v, "record_field_unknown");
    let lines: Vec<i64> = unknown.iter().map(|f| f["line"].as_i64().unwrap()).collect();
    assert_eq!(lines, vec![11, 12], "one finding per line, blank value included: {:?}", unknown);
    assert!(unknown.iter().all(|f| f["detail"] == "reason"));
}

// --- REQ-core-129 / REQ-core-134 / REQ-core-135: 読まない行と、検査を受けない記録 ---

// @kotowari[REQ-core-129, EX-core-102]
#[test]
fn req_129_record_without_context_is_not_checked() {
    // EX-core-102
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "c.md",
        "# 記録 c\n\n## Agreements\n\n- A1 ある合意\n",
    );
    let v = check(tmp.path());
    assert!(
        findings_by_kind(&v, "record_field_missing").is_empty()
            && findings_by_kind(&v, "record_field_unknown").is_empty(),
        "a record without \"## Context\" is not checked: {:?}",
        v["findings"]
    );
}

// @kotowari[REQ-core-135, EX-core-110]
#[test]
fn req_135_lines_outside_the_table_sections_and_orphans_are_not_read() {
    // EX-core-110
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "v.md",
        concat!(
            "# 記録 v\n\n## Context\n\n背景。\n\n",
            "## Revisions\n\n- A21 は A5 を置き換える\n\n",
            "## Agreements\n\n- why: x\n- A1 ある合意\n- why: x\n- (i) 入れ子でない箇条\n- why : x\n（なし）\n",
        ),
    );
    let v = check(tmp.path());
    let for_file: Vec<&serde_json::Value> = v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["path"] == "docs/decision/records/v.md")
        .collect();
    assert!(
        for_file.is_empty(),
        "lines outside the table's sections, orphan field lines and other shapes are not read: {:?}",
        for_file
    );
}

// @kotowari[REQ-core-134, EX-core-111]
#[test]
fn req_134_duplicate_field_names_pass() {
    // EX-core-111
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "w.md",
        "# 記録 w\n\n## Context\n\n背景。\n\n## Agreements\n\n- A1 ある合意\n- why: x\n- why: x\n",
    );
    let v = check(tmp.path());
    assert!(
        findings_by_kind(&v, "record_field_missing").is_empty()
            && findings_by_kind(&v, "record_field_unknown").is_empty(),
        "the number of field lines with the same name is not checked: {:?}",
        v["findings"]
    );
}

// @kotowari[REQ-core-135, EX-core-113]
#[test]
fn req_135_numbered_line_inside_code_block_is_not_read() {
    // EX-core-113
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "cb.md",
        "# 記録 cb\n\n## Context\n\n背景。\n\n## Agreements\n\n- A1 ある合意\n- why: x\n\n```text\n- A9 コードブロックの中\n```\n",
    );
    let v = check(tmp.path());
    assert!(
        findings_by_kind(&v, "record_field_missing").is_empty()
            && findings_by_kind(&v, "record_field_unknown").is_empty(),
        "a numbered line inside a code block is not read: {:?}",
        v["findings"]
    );
}

// @kotowari[REQ-core-135]
#[test]
fn req_135_unclosed_code_block_runs_to_the_end_of_the_file() {
    // A45: 閉じられずに文書が終わるコードブロックは文書の終わりまでが中で、指摘は出さない
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "uc.md",
        "# 記録 uc\n\n## Context\n\n背景。\n\n## Agreements\n\n- A1 ある合意\n- why: x\n\n```text\n- A9 x\n- A8 x\n",
    );
    let v = check(tmp.path());
    let for_file: Vec<_> = v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["path"] == "docs/decision/records/uc.md")
        .collect();
    assert!(for_file.is_empty(), "no finding for an unclosed code block: {:?}", for_file);
}

// @kotowari[REQ-core-130, EX-core-114]
#[test]
fn req_130_unindented_field_line_belongs_to_the_decision() {
    // EX-core-114: 字下げ無しの補足の行も直前の番号の行に付く。空行を挟んでもよい
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "ui.md",
        "# 記録 ui\n\n## Context\n\n背景。\n\n## Agreements\n\n- A1 ある合意\n\n- why: x\n",
    );
    let v = check(tmp.path());
    assert!(
        findings_by_kind(&v, "record_field_missing").is_empty(),
        "an unindented why after a blank line still belongs to A1: {:?}",
        v["findings"]
    );
}

// @kotowari[REQ-core-130, REQ-core-133, EX-core-115]
#[test]
fn req_130_decision_line_with_colon_is_not_a_field() {
    // EX-core-115: 本文にコロンを含む決定の行は番号の行で、補足の行と見ない
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "co.md",
        "# 記録 co\n\n## Context\n\n背景。\n\n## Agreements\n\n- A1 定義を機械的にする: 節にある行\n- why: x\n",
    );
    let v = check(tmp.path());
    assert!(
        findings_by_kind(&v, "record_field_missing").is_empty()
            && findings_by_kind(&v, "record_field_unknown").is_empty(),
        "the decision line is a numbered line, not a field line: {:?}",
        v["findings"]
    );
}

// @kotowari[REQ-core-129]
#[test]
fn req_129_file_without_decision_sections_is_not_a_record() {
    // A41: 決定の節の見出しを持たないファイルは判断の記録でない
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "notrec.md",
        "# 記録でない文書\n\n## Context\n\n背景。\n\n## Superseded\n\n- A3 置き換えられた決定\n",
    );
    let v = check(tmp.path());
    assert!(
        findings_by_kind(&v, "record_field_missing").is_empty()
            && findings_by_kind(&v, "record_field_unknown").is_empty(),
        "a file without a decision section is not a record: {:?}",
        v["findings"]
    );
}

// --- REQ-core-132 / TBL-core-023: superseded_by のリンク ---

/// revision_link_invalid の (line, detail) を並べる
fn link_findings(v: &serde_json::Value) -> Vec<(i64, String)> {
    findings_by_kind(v, "revision_link_invalid")
        .iter()
        .map(|f| {
            (
                f["line"].as_i64().unwrap(),
                f["detail"].as_str().unwrap().to_string(),
            )
        })
        .collect()
}

// @kotowari[REQ-core-132, TBL-core-023, EX-core-107]
#[test]
fn req_132_superseded_by_without_link_is_invalid() {
    // EX-core-107: 順1（値にリンクが1つも無い）。detail は行の値
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "sb.md",
        "# 記録 sb\n\n## Agreements\n\n- A1 ある合意\n- superseded_by: A24\n",
    );
    let v = check(tmp.path());
    assert_eq!(link_findings(&v), vec![(6, "A24".to_string())]);
}

// @kotowari[REQ-core-132, TBL-core-023, EX-core-108]
#[test]
fn req_132_href_outside_the_records_place_is_invalid() {
    // EX-core-108: 順3（解決した結果が置き場の外）
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "out.md",
        "# 記録 out\n\n## Agreements\n\n- A1 ある合意\n- superseded_by: [A1](../../ir/example.md#A1)\n",
    );
    let v = check(tmp.path());
    assert_eq!(
        link_findings(&v),
        vec![(6, "../../ir/example.md#A1".to_string())]
    );
}

// @kotowari[REQ-core-132, TBL-core-023, EX-core-109]
#[test]
fn req_132_number_only_in_undecided_is_invalid() {
    // EX-core-109: 順5（先の決定の節と Superseded の節に番号の行が無い。Undecided は数えない）
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "a.md",
        "# 記録 a\n\n## Agreements\n\n- A1 ある合意\n- superseded_by: [U1](./b.md#U1)\n",
    );
    write_record(
        tmp.path(),
        "b.md",
        "# 記録 b\n\n## Agreements\n\n- B1 何か\n\n## Undecided\n\n- U1 未決\n",
    );
    let v = check(tmp.path());
    assert_eq!(link_findings(&v), vec![(6, "./b.md#U1".to_string())]);
}

// @kotowari[REQ-core-132, TBL-core-023, EX-core-112]
#[test]
fn req_132_heading_anchor_is_invalid() {
    // EX-core-112: 順2（"#" の後が決定の番号の形でない）
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "ir-form.md",
        "# IR の形の契約\n\n## 出典\n\n出典の形。\n",
    );
    write_record(
        tmp.path(),
        "a2.md",
        "# 記録 a2\n\n## Agreements\n\n- A1 ある合意\n- superseded_by: [出典](./ir-form.md#出典)\n",
    );
    let v = check(tmp.path());
    assert_eq!(link_findings(&v), vec![(6, "./ir-form.md#出典".to_string())]);
}

// @kotowari[REQ-core-132, TBL-core-023, EX-core-117]
#[test]
fn req_132_empty_path_means_the_same_record() {
    // EX-core-117: "#" より前が空なら同じ記録
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "a3.md",
        "# 記録 a3\n\n## Agreements\n\n- A1 ある合意\n- superseded_by: [A2](#A2)\n- superseded_by: [A9](#A9)\n- A2 二番目の合意\n",
    );
    let v = check(tmp.path());
    assert_eq!(link_findings(&v), vec![(7, "#A9".to_string())]);
}

// @kotowari[REQ-core-132, TBL-core-023, EX-core-119]
#[test]
fn req_132_target_that_is_not_a_record_is_invalid() {
    // EX-core-119: 順4（先が読んだ判断の記録でない）
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "a4.md",
        "# 記録 a4\n\n## Agreements\n\n- A1 ある合意\n- superseded_by: [A1](./ir-form.md#A1)\n",
    );
    let v = check(tmp.path());
    assert_eq!(link_findings(&v), vec![(6, "./ir-form.md#A1".to_string())]);
}

// @kotowari[REQ-core-132, TBL-core-023]
#[test]
fn tbl_023_same_href_twice_on_one_line_yields_two_findings() {
    // 同じ行に同じ href が2つあれば出現ごとに1件
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "tw.md",
        "# 記録 tw\n\n## Agreements\n\n- A1 ある合意\n- superseded_by: [A9](#A9) と [A9](#A9)\n",
    );
    let v = check(tmp.path());
    assert_eq!(
        link_findings(&v),
        vec![(6, "#A9".to_string()), (6, "#A9".to_string())]
    );
}

// @kotowari[REQ-core-132, TBL-core-023]
#[test]
fn tbl_023_unclosed_link_is_skipped_and_falls_to_no_link() {
    // A44: ")" が無い形と "]" の直後が "(" でない形はリンクでなく、順1になる
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "ul.md",
        "# 記録 ul\n\n## Agreements\n\n- A1 ある合意\n- superseded_by: [A1](#A1\n- A2 二番目の合意\n- superseded_by: [A2] を見よ\n",
    );
    let v = check(tmp.path());
    assert_eq!(
        link_findings(&v),
        vec![
            (6, "[A1](#A1".to_string()),
            (8, "[A2] を見よ".to_string())
        ]
    );
}

// @kotowari[REQ-core-132, TBL-core-023]
#[test]
fn tbl_023_absolute_href_is_outside_the_place() {
    // 順3: "/" か "\" で始まる href は、つなぐ前に置き場の外と決まる
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "ab.md",
        "# 記録 ab\n\n## Agreements\n\n- A1 ある合意\n- superseded_by: [A1](/records.md#A1)\n- A2 二番目の合意\n- superseded_by: [A1](\\records.md#A1)\n",
    );
    let v = check(tmp.path());
    assert_eq!(
        link_findings(&v),
        vec![
            (6, "/records.md#A1".to_string()),
            (8, "\\records.md#A1".to_string())
        ]
    );
}

// @kotowari[TBL-core-023]
#[test]
fn tbl_023_link_scan_resumes_after_the_closing_paren() {
    // リンクとして読めたら走査は ")" の次から続き、href の中の "[" から読み始めない
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "lr.md",
        "# 記録 lr\n\n## Agreements\n\n- A1 ある合意\n- superseded_by: []([)]()\n",
    );
    let v = check(tmp.path());
    assert_eq!(link_findings(&v), vec![(6, "[".to_string())]);
}

// @kotowari[REQ-core-132, TBL-core-023, EX-core-106]
#[test]
fn req_132_record_without_context_resolves_two_links_in_superseded() {
    // EX-core-106: "## Context" を持たない記録の Superseded の行のリンク2つがどちらも解決される
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "ir-tree.md",
        "# 記録 ir-tree\n\n## Agreements\n\n- A21 ある合意\n- A5 別の合意\n",
    );
    write_record(
        tmp.path(),
        "old.md",
        "# 記録 old\n\n## Agreements\n\n- A1 ある合意\n\n## Superseded\n\n- A54 置き換えられた決定\n- superseded_by: [ir-tree の A21](./ir-tree.md#A21)、[ir-tree の A5](./ir-tree.md#A5)\n",
    );
    let v = check(tmp.path());
    assert!(
        link_findings(&v).is_empty(),
        "both links resolve: {:?}",
        link_findings(&v)
    );
}

// @kotowari[REQ-core-132, TBL-core-023, REQ-core-110, EX-core-116]
#[test]
fn req_132_parent_directory_href_to_superseded_number_passes() {
    // EX-core-116: 下位ディレクトリの記録から ".." で上の記録の Superseded の番号を指すリンク
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "records.md",
        "# 判断の記録\n\n## Agreements\n\n- A1 最初の合意\n\n## Superseded\n\n- A15 置き換えられた決定\n",
    );
    fs::create_dir_all(tmp.path().join("docs/decision/records/sub")).unwrap();
    write_record(
        tmp.path(),
        "sub/a.md",
        "# 記録 sub/a\n\n## Agreements\n\n- A1 ある合意\n- superseded_by: [A15](../records.md#A15)\n",
    );
    let v = check(tmp.path());
    assert!(
        link_findings(&v).is_empty(),
        "\"../records.md#A15\" resolves inside the place: {:?}",
        link_findings(&v)
    );
}

// @kotowari[REQ-core-133, REQ-core-132]
#[test]
fn req_133_empty_superseded_by_is_not_a_link_finding() {
    // 値が空の superseded_by はリンクの判定を受けない。Context を持つ記録では欠けだけが出る
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "ctx.md",
        "# 記録 ctx\n\n## Context\n\n背景。\n\n## Agreements\n\n- A1 ある合意\n- why: x\n\n## Superseded\n\n- A3 置き換えられた決定\n- superseded_by:\n",
    );
    write_record(
        tmp.path(),
        "noctx.md",
        "# 記録 noctx\n\n## Agreements\n\n- A1 ある合意\n\n## Superseded\n\n- A4 置き換えられた決定\n- superseded_by:\n",
    );
    let v = check(tmp.path());
    assert!(link_findings(&v).is_empty(), "a blank value is not a link: {:?}", link_findings(&v));
    let missing = findings_by_kind(&v, "record_field_missing");
    assert_eq!(missing.len(), 1, "only the record with \"## Context\": {:?}", missing);
    assert_eq!(missing[0]["path"], "docs/decision/records/ctx.md");
    assert_eq!(missing[0]["line"], 14);
    assert_eq!(missing[0]["detail"], "superseded_by");
}

// @kotowari[REQ-core-135, REQ-core-132]
#[test]
fn req_135_superseded_by_in_revisions_is_not_read() {
    // Revisions は TBL-core-022 の表に無い節なので、その中の superseded_by は読まない
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "rev.md",
        "# 記録 rev\n\n## Agreements\n\n- A1 ある合意\n\n## Revisions\n\n- A5 は A9 を置き換える\n- superseded_by: [A9](#A9)\n",
    );
    let v = check(tmp.path());
    assert!(
        link_findings(&v).is_empty(),
        "a superseded_by in Revisions is not read: {:?}",
        link_findings(&v)
    );
}

// @kotowari[REQ-core-129, REQ-core-132]
#[test]
fn req_129_broken_link_in_a_file_without_decision_sections_is_not_checked() {
    // A41: 決定の節の見出しを持たないファイルは判断の記録でなく、どの検査も受けない
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "plain.md",
        "# 記録でない文書\n\n## Context\n\n背景。\n\n## Superseded\n\n- A3 置き換えられた決定\n- superseded_by: [A9](#A9)\n",
    );
    let v = check(tmp.path());
    let for_file: Vec<_> = v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["path"] == "docs/decision/records/plain.md")
        .collect();
    assert!(for_file.is_empty(), "no finding for a file that is not a record: {:?}", for_file);
}
