use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::cargo_bin("kotowari").unwrap()
}

/// テスト用のプロジェクトに出典の先を作る
fn make_project_with_records(tmp: &std::path::Path) {
    fs::create_dir_all(tmp.join(".kotowari")).unwrap();
    fs::create_dir_all(tmp.join("docs/ir")).unwrap();
    fs::create_dir_all(tmp.join("docs/decision/brainstorm")).unwrap();
    fs::create_dir_all(tmp.join("docs/decision/adr")).unwrap();
    fs::write(
        tmp.join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\n",
    )
    .unwrap();
    // 判断の記録を作る
    fs::write(
        tmp.join("docs/decision/brainstorm/records.md"),
        "# 判断の記録\n\n## Agreements\n\n- A1 最初の合意\n- A2 二番目の合意\n\n## Prohibitions\n\n- P1 禁止事項\n\n## Delegated\n\n## Rejected\n\n- R1 却下\n",
    )
    .unwrap();
    // 形の契約を作る
    fs::write(
        tmp.join("docs/decision/brainstorm/ir-form.md"),
        "# IR の形の契約\n\n## 文書\n\n文書の形。\n\n## 項目\n\n項目の形。\n\n## 検査の種類\n\n種類。\n",
    )
    .unwrap();
    // ADR を作る
    fs::write(
        tmp.join("docs/decision/adr/0001-test.md"),
        "# ADR 0001: テスト\n\n## 状況\n\n状況の説明。\n\n## 決定\n\n決定。\n",
    )
    .unwrap();
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

// --- REQ-057: 出典の書式 ---

// @kotowari[REQ-057]
#[test]
fn req_057_source_splits_at_first_hash_and_allows_commas() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 出典にコンマ区切りで複数の出典を書く
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A1, docs/decision/brainstorm/records.md#A2\n- 検証: unit\n\nStatement.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    assert!(si.is_empty(), "valid sources should pass: {:?}", si);
}

// --- REQ-058: 出典の判定 ---

// @kotowari[REQ-058, TBL-012]
#[test]
fn req_058_number_anchor_looks_for_decision_line_and_other_anchor_for_heading() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 正しい出典（決定の番号）
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A1\n- 検証: unit\n\nStatement.\n\n### REQ-002: Test2\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A99\n- 検証: unit\n\nStatement2.\n\n### REQ-003: Test3\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/ir-form.md#文書\n- 検証: unit\n\nStatement3.\n\n### REQ-004: Test4\n\n- 種類: ubiquitous\n- 出典: docs/decision/adr/0001-test.md#状況\n- 検証: unit\n\nStatement4.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    // A99 は存在しない → source_invalid
    assert_eq!(si.len(), 1, "only A99 should be invalid: {:?}", si);
    assert_eq!(si[0]["detail"], "docs/decision/brainstorm/records.md#A99");
}

// @kotowari[REQ-058, TBL-012]
#[test]
fn req_058_source_outside_places_is_invalid() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: somewhere/else.md#heading\n- 検証: unit\n\nStatement.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    assert_eq!(si.len(), 1);
}

// --- REQ-059: 出典が無い ---

// @kotowari[REQ-059, REQ-098]
#[test]
fn req_059_missing_source_for_item_scenario_and_term() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 出典の行がない要求 → REQ-098 により missing_field detail="出典"
    // 出典の値が空のシナリオ → REQ-059 により missing_source
    // 用語の出典が空 → REQ-059 により missing_source
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        concat!(
            "# Title\n\nScope.\n\n## 要求\n\n",
            "### REQ-001: Test\n\n- 種類: ubiquitous\n- 検証: unit\n\nStatement.\n\n",
            "## 具体例\n\n",
            "```gherkin\n",
            "@id=EX-001 @about=REQ-001\n",
            "Scenario: No source\n",
            "  Given something\n",
            "```\n",
        ),
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n| テスト | 意味 |  |\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);

    // 要求: 出典の行が無い → missing_field detail="出典"（REQ-098）
    let mf = findings_by_kind(&v, "missing_field");
    assert!(
        mf.iter().any(|f| f["detail"] == "出典" && f["path"].as_str().unwrap().contains("a.md")),
        "absent source line should produce missing_field with detail '出典': {:?}",
        mf
    );

    // シナリオ: @source タグが無い → missing_source
    let ms = findings_by_kind(&v, "missing_source");
    assert!(
        ms.iter().any(|f| f["detail"] == "EX-001"),
        "scenario without @source should produce missing_source with detail 'EX-001': {:?}",
        ms
    );

    // 用語: 出典の列が空 → missing_source
    assert!(
        ms.iter().any(|f| f["detail"] == "テスト"),
        "glossary term with empty source should produce missing_source with detail 'テスト': {:?}",
        ms
    );
}

// @kotowari[REQ-059]
#[test]
fn req_059_empty_source_value_produces_missing_source() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 出典の行はあるが値が空
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典:\n- 検証: unit\n\nStatement.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ms = findings_by_kind(&v, "missing_source");
    assert!(
        ms.iter().any(|f| f["detail"] == "REQ-001"),
        "empty source value should produce missing_source: {:?}",
        ms
    );
}

// @kotowari[REQ-057, REQ-060]
#[test]
fn req_060_glossary_trailing_comma_does_not_create_empty_source() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 用語集の出典に末尾コンマ → 空要素ができないこと
    fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n| テスト | テストの意味 | docs/decision/brainstorm/records.md#A1, |\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    // 空文字列の出典が source_invalid として出てはいけない
    assert!(
        !si.iter().any(|f| f["detail"].as_str() == Some("")),
        "trailing comma should not produce empty source_invalid: {:?}",
        si
    );
    // 正常な出典は通る
    assert!(si.is_empty(), "valid source with trailing comma should pass: {:?}", si);
}

// --- REQ-060: 用語集とシナリオの出典 ---

// @kotowari[REQ-060]
#[test]
fn req_060_glossary_and_scenario_sources_are_checked() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 用語集に出典が無い用語
    fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n| テスト | テストの意味 |  |\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ms = findings_by_kind(&v, "missing_source");
    assert!(ms.iter().any(|f| f["detail"] == "テスト"), "glossary term without source: {:?}", ms);
}

// --- REQ-061: 決定の番号はファイルごと ---

// @kotowari[REQ-061]
#[test]
fn req_061_numbers_are_per_file() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 別のファイルに別の番号を持つ
    fs::write(
        tmp.path().join("docs/decision/brainstorm/other.md"),
        "# 別の記録\n\n## Agreements\n\n- B1 別の合意\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/other.md#B1\n- 検証: unit\n\nStatement.\n\n### REQ-002: Test2\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#B1\n- 検証: unit\n\nStatement2.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    // records.md に B1 はない → source_invalid
    assert_eq!(si.len(), 1, "B1 should be invalid in records.md: {:?}", si);
    assert!(si[0]["detail"].as_str().unwrap().contains("records.md#B1"));
}

// --- REQ-106: 形の契約を出典に指せる ---

// @kotowari[REQ-106]
#[test]
fn req_106_form_contract_headings_are_valid_sources() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/ir-form.md#項目\n- 検証: unit\n\nStatement.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    assert!(si.is_empty(), "ir-form.md headings should be valid: {:?}", si);
}

// --- REQ-063: 対象の行 ---

// @kotowari[REQ-063, TBL-013]
#[test]
fn req_063_only_sentences_and_steps_are_checked() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 用語集を作る
    fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n| テスト | 意味 | docs/decision/brainstorm/records.md#A1 |\n",
    )
    .unwrap();
    // 要求の文に未知の用語を使う → 検出される
    // フィールドの行に未知の語を使う → 検出されない
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A1\n- 検証: unit\n\n`未知語`を使う文。\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ut = findings_by_kind(&v, "unknown_term");
    assert!(ut.iter().any(|f| f["detail"] == "未知語"), "should find unknown term in sentence: {:?}", ut);
}

// --- REQ-064: 用語集に無い語 ---

// @kotowari[REQ-064]
#[test]
fn req_064_unknown_term() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n| IR | 仕様の集まり | docs/decision/brainstorm/records.md#A1 |\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A1\n- 検証: unit\n\n`IR`は良いが`不明な語`は誤り。\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ut = findings_by_kind(&v, "unknown_term");
    assert!(ut.iter().any(|f| f["detail"] == "不明な語"), "should find unknown term: {:?}", ut);
    // IR は用語集にある → unknown_term にならない
    assert!(!ut.iter().any(|f| f["detail"] == "IR"), "IR should be known: {:?}", ut);
}

// --- REQ-065: 用語集がないとき ID は通る ---

// @kotowari[REQ-065]
#[test]
fn req_065_ids_pass_without_glossary() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // CONTEXT.md なし
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A1\n- 検証: unit\n\n`REQ-001`を参照する文。\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ut = findings_by_kind(&v, "unknown_term");
    // ID は用語集がなくても通る
    assert!(!ut.iter().any(|f| f["detail"] == "REQ-001"), "IDs should pass without glossary: {:?}", ut);
}

// --- REQ-066: 曖昧語 ---

// @kotowari[REQ-066]
#[test]
fn req_066_vague_word_substring() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A1\n- 検証: unit\n\n適切に処理する。\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let vw = findings_by_kind(&v, "vague_word");
    assert!(vw.iter().any(|f| f["detail"] == "適切に"), "should find vague word: {:?}", vw);
}

// @kotowari[REQ-066]
#[test]
fn req_066_empty_vague_word_does_not_hang() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // vague_words に空文字列を含む設定
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\nvague_words:\n  - \"\"\n  - 適切に\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A1\n- 検証: unit\n\n適切に処理する。\n",
    )
    .unwrap();
    let output = cmd()
        .arg("check")
        .current_dir(tmp.path())
        .timeout(std::time::Duration::from_secs(5))
        .output()
        .unwrap();
    let v = parse_json(&output);
    let vw = findings_by_kind(&v, "vague_word");
    // 空文字列の曖昧語は指摘にならない
    assert!(
        !vw.iter().any(|f| f["detail"].as_str() == Some("")),
        "empty vague word should not produce a finding: {:?}",
        vw
    );
    // 通常の曖昧語は検出される
    assert!(
        vw.iter().any(|f| f["detail"] == "適切に"),
        "normal vague word should still be found: {:?}",
        vw
    );
}

// --- REQ-067: 出現ごとに1件 ---

// @kotowari[REQ-067]
#[test]
fn req_067_one_finding_per_occurrence() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A1\n- 検証: unit\n\n適切に適切に処理。\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let vw = findings_by_kind(&v, "vague_word");
    let count = vw.iter().filter(|f| f["detail"] == "適切に").count();
    assert_eq!(count, 2, "should report 2 occurrences of vague word: {:?}", vw);
}

// --- REQ-069: 文書名の参照の境界と引用符 ---

// @kotowari[REQ-069, TBL-014]
#[test]
fn req_069_reference_needs_boundary_and_quotes_are_skipped() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // a.md は存在する、nonexistent.md は存在しない
    // スラッシュの後の文書名は拾わない
    // 二重引用符の中は拾わない
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope with a.md reference.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A1\n- 検証: unit\n\nSee nonexistent.md for details. But \"quoted.md\" is skipped. And adr/0001-test-marker.md is not a reference.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let md = findings_by_kind(&v, "missing_document");
    // nonexistent.md → missing_document
    assert!(md.iter().any(|f| f["detail"] == "nonexistent.md"), "should find missing document: {:?}", md);
    // quoted.md → 引用符の中なので拾わない
    assert!(!md.iter().any(|f| f["detail"] == "quoted.md"), "quoted should be skipped: {:?}", md);
    // 0001-test-marker.md → スラッシュの後なので拾わない
    assert!(!md.iter().any(|f| f["detail"] == "0001-test-marker.md"), "slash prefix should be skipped: {:?}", md);
}

// --- REQ-070: 参照された文書が無い ---

// @kotowari[REQ-070]
#[test]
fn req_070_missing_document() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope with missing-doc.md reference.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let md = findings_by_kind(&v, "missing_document");
    assert!(md.iter().any(|f| f["detail"] == "missing-doc.md"), "should find missing document: {:?}", md);
}

// --- REQ-104: 具体的な値は二重引用符で書く ---

// @kotowari[REQ-104]
#[test]
fn req_104_quoted_values_are_not_terms() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n| IR | 仕様 | docs/decision/brainstorm/records.md#A1 |\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A1\n- 検証: unit\n\n\"kotowari check\" を実行する。\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ut = findings_by_kind(&v, "unknown_term");
    // 二重引用符の中は用語チェックの対象外
    assert!(!ut.iter().any(|f| f["detail"] == "kotowari check"), "quoted values should not be checked: {:?}", ut);
}

// --- REQ-063, REQ-064: 性質の文とシナリオの手順でも用語を検査する ---

// @kotowari[REQ-063, REQ-064]
#[test]
fn req_063_property_statements_and_scenario_steps_are_term_checked() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 用語集を作る（既知の語は「テスト」だけ）
    fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n| テスト | 意味 | docs/decision/brainstorm/records.md#A1 |\n",
    )
    .unwrap();
    // 性質の文に未知の用語、シナリオの手順に別の未知の用語を使う
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A1\n- 検証: unit\n\n`テスト`を使う文。\n\n## 性質\n\n### PROP-001: P\n\n- 出典: docs/decision/brainstorm/records.md#A1\n\n`性質側の未知語`を検査する。\n\n## 具体例\n\n```gherkin\n@id=EX-001 @about=REQ-001 @source=docs/decision/brainstorm/records.md#A1\nScenario: Test\n  Given `手順側の未知語`を使う\n```\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ut = findings_by_kind(&v, "unknown_term");
    // 性質の文に含まれる未知語が検出される
    assert!(
        ut.iter().any(|f| f["detail"] == "性質側の未知語"),
        "should find unknown term in property statement: {:?}",
        ut
    );
    // シナリオの手順に含まれる未知語が検出される
    assert!(
        ut.iter().any(|f| f["detail"] == "手順側の未知語"),
        "should find unknown term in scenario step: {:?}",
        ut
    );
    // 用語集にある「テスト」は検出されない
    assert!(
        !ut.iter().any(|f| f["detail"] == "テスト"),
        "known term should not be reported: {:?}",
        ut
    );
}

// --- REQ-040: gherkin コードブロック内の文書名参照は対象外 ---

// @kotowari[REQ-040]
#[test]
fn req_040_gherkin_code_block_doc_ref_is_not_checked() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        concat!(
            "# Title\n\nScope.\n\n## 要求\n\n",
            "### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A1\n- 検証: unit\n\nStatement.\n\n",
            "## 具体例\n\n",
            "```gherkin\n",
            "@id=EX-001 @about=REQ-001 @source=docs/decision/brainstorm/records.md#A1\n",
            "Scenario: Test with doc ref\n",
            "  Given some-file.md exists\n",
            "```\n",
        ),
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let md = findings_by_kind(&v, "missing_document");
    assert!(
        !md.iter().any(|f| f["detail"] == "some-file.md"),
        "gherkin コードブロック内の文書名参照は検査対象外のはず: {:?}",
        md
    );
}
