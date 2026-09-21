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
    fs::create_dir_all(tmp.join("docs/decision/records")).unwrap();
    fs::create_dir_all(tmp.join("docs/decision/adr")).unwrap();
    fs::write(
        tmp.join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\n",
    )
    .unwrap();
    // 判断の記録を作る
    fs::write(
        tmp.join("docs/decision/records/records.md"),
        "# 判断の記録\n\n## Agreements\n\n- A1 最初の合意\n- A2 二番目の合意\n\n## Prohibitions\n\n- P1 禁止事項\n\n## Delegated\n\n## Rejected\n\n- R1 却下\n",
    )
    .unwrap();
    // 形の契約を作る
    fs::write(
        tmp.join("docs/decision/records/ir-form.md"),
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

// --- REQ-core-057: 出典の書式 ---

// @kotowari[REQ-core-057]
#[test]
fn req_057_source_splits_at_first_hash_and_allows_commas() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 出典にコンマ区切りで複数の出典を書く
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1, docs/decision/records/records.md#A2\n- 検証: unit\n\nStatement.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    assert!(si.is_empty(), "valid sources should pass: {:?}", si);
}

// --- REQ-core-058: 出典の判定 ---

// @kotowari[REQ-core-058, TBL-core-012, EX-core-011, EX-core-012]
#[test]
fn req_058_number_anchor_looks_for_decision_line_and_other_anchor_for_heading() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 正しい出典（決定の番号）
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: unit\n\nStatement.\n\n### REQ-002: Test2\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A99\n- 検証: unit\n\nStatement2.\n\n### REQ-003: Test3\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/ir-form.md#文書\n- 検証: unit\n\nStatement3.\n\n### REQ-004: Test4\n\n- 種類: ubiquitous\n- 出典: docs/decision/adr/0001-test.md#状況\n- 検証: unit\n\nStatement4.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    // A99 は存在しない → source_invalid
    assert_eq!(si.len(), 1, "only A99 should be invalid: {:?}", si);
    assert_eq!(si[0]["detail"], "docs/decision/records/records.md#A99");
}

// @kotowari[REQ-core-058, TBL-core-012]
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

// @kotowari[REQ-core-058, TBL-core-012]
#[test]
fn req_058_source_path_equal_to_a_place_itself_is_invalid_without_crashing() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // パスが置き場そのもの（末尾に "/..." が無い）は、置き場の中ではない
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/adr#状況\n- 検証: unit\n\nStatement.\n\n### REQ-002: Test2\n\n- 種類: ubiquitous\n- 出典: docs/decision/records#A1\n- 検証: unit\n\nStatement2.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(output.status.code(), Some(1), "should finish with errors, not crash: {:?}", output);
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    assert_eq!(si.len(), 2, "{:?}", si);
}

// --- REQ-core-059: 出典が無い ---

// @kotowari[REQ-core-059, REQ-core-098]
#[test]
fn req_059_missing_source_for_item_scenario_and_term() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 出典の行がない要求 → REQ-core-098 により missing_field detail="出典"
    // 出典の値が空のシナリオ → REQ-core-059 により missing_source
    // 用語の出典が空 → REQ-core-059 により missing_source
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

    // 要求: 出典の行が無い → missing_source detail = ID
    let ms = findings_by_kind(&v, "missing_source");
    assert!(
        ms.iter().any(|f| f["detail"] == "REQ-001" && f["path"].as_str().unwrap().contains("a.md")),
        "absent source line should produce missing_source with detail 'REQ-001': {:?}",
        ms
    );
    let mf = findings_by_kind(&v, "missing_field");
    assert!(
        !mf.iter().any(|f| f["detail"] == "出典" && f["path"].as_str().unwrap().contains("a.md")),
        "missing_field 出典 should not appear when source line is absent: {:?}",
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

// @kotowari[REQ-core-059]
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

// @kotowari[REQ-core-057, REQ-core-060]
#[test]
fn req_060_glossary_trailing_comma_does_not_create_empty_source() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 用語集の出典に末尾コンマ → 空要素ができないこと
    fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n| テスト | テストの意味 | docs/decision/records/records.md#A1, |\n",
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

// --- REQ-core-060: 用語集とシナリオの出典 ---

// @kotowari[REQ-core-060]
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

// --- REQ-core-061: 決定の番号はファイルごと ---

// @kotowari[REQ-core-061]
#[test]
fn req_061_numbers_are_per_file() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 別のファイルに別の番号を持つ
    fs::write(
        tmp.path().join("docs/decision/records/other.md"),
        "# 別の記録\n\n## Agreements\n\n- B1 別の合意\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/other.md#B1\n- 検証: unit\n\nStatement.\n\n### REQ-002: Test2\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#B1\n- 検証: unit\n\nStatement2.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    // records.md に B1 はない → source_invalid
    assert_eq!(si.len(), 1, "B1 should be invalid in records.md: {:?}", si);
    assert!(si[0]["detail"].as_str().unwrap().contains("records.md#B1"));
}

// --- REQ-core-106: 形の契約を出典に指せる ---

// @kotowari[REQ-core-106]
#[test]
fn req_106_form_contract_headings_are_valid_sources() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/ir-form.md#項目\n- 検証: unit\n\nStatement.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    assert!(si.is_empty(), "ir-form.md headings should be valid: {:?}", si);
}

// --- REQ-core-063: 対象の行 ---

// @kotowari[REQ-core-063, TBL-core-013]
#[test]
fn req_063_only_sentences_and_steps_are_checked() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 用語集を作る
    fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n| テスト | 意味 | docs/decision/records/records.md#A1 |\n",
    )
    .unwrap();
    // 要求の文に未知の用語を使う → 検出される
    // フィールドの行に未知の語を使う → 検出されない
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: unit\n\n`未知語`を使う文。\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ut = findings_by_kind(&v, "unknown_term");
    assert!(ut.iter().any(|f| f["detail"] == "未知語"), "should find unknown term in sentence: {:?}", ut);
}

// --- REQ-core-064: 用語集に無い語 ---

// @kotowari[REQ-core-064]
#[test]
fn req_064_unknown_term() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n| IR | 仕様の集まり | docs/decision/records/records.md#A1 |\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: unit\n\n`IR`は良いが`不明な語`は誤り。\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ut = findings_by_kind(&v, "unknown_term");
    assert!(ut.iter().any(|f| f["detail"] == "不明な語"), "should find unknown term: {:?}", ut);
    // IR は用語集にある → unknown_term にならない
    assert!(!ut.iter().any(|f| f["detail"] == "IR"), "IR should be known: {:?}", ut);
}

// --- REQ-core-065: 用語集がないとき ID は通る ---

// @kotowari[REQ-core-065]
#[test]
fn req_065_ids_pass_without_glossary() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // CONTEXT.md なし
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: unit\n\n`REQ-001`を参照する文。\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ut = findings_by_kind(&v, "unknown_term");
    // ID は用語集がなくても通る
    assert!(!ut.iter().any(|f| f["detail"] == "REQ-001"), "IDs should pass without glossary: {:?}", ut);
}

// --- REQ-core-066: 曖昧語 ---

// @kotowari[REQ-core-066]
#[test]
fn req_066_vague_word_substring() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: unit\n\n適切に処理する。\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let vw = findings_by_kind(&v, "vague_word");
    assert!(vw.iter().any(|f| f["detail"] == "適切に"), "should find vague word: {:?}", vw);
}

// @kotowari[REQ-core-014]
#[test]
fn req_014_empty_vague_word_stops_with_config_error() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\nvague_words:\n  - \"\"\n  - 適切に\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: unit\n\n適切に処理する。\n",
    )
    .unwrap();
    let output = cmd()
        .arg("check")
        .current_dir(tmp.path())
        .timeout(std::time::Duration::from_secs(5))
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(2),
        "empty vague word should stop with exit code 2"
    );
    assert!(output.stdout.is_empty(), "stdout should be empty on stop");
}

// --- REQ-core-067: 出現ごとに1件 ---

// @kotowari[REQ-core-067]
#[test]
fn req_067_one_finding_per_occurrence() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: unit\n\n適切に適切に処理。\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let vw = findings_by_kind(&v, "vague_word");
    let count = vw.iter().filter(|f| f["detail"] == "適切に").count();
    assert_eq!(count, 2, "should report 2 occurrences of vague word: {:?}", vw);
}

// --- REQ-core-069: 文書名の参照の境界と引用符 ---

// @kotowari[REQ-core-069, TBL-core-014]
#[test]
fn req_069_reference_needs_boundary_and_quotes_are_skipped() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // a.md は存在する、nonexistent.md は存在しない
    // 二重引用符の中は拾わない
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope with a.md reference.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: unit\n\nSee nonexistent.md for details. But \"quoted.md\" is skipped. And adr/0001-test-marker.md is a reference.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let md = findings_by_kind(&v, "missing_document");
    // nonexistent.md → missing_document
    assert!(md.iter().any(|f| f["detail"] == "nonexistent.md"), "should find missing document: {:?}", md);
    // quoted.md → 引用符の中なので拾わない
    assert!(!md.iter().any(|f| f["detail"] == "quoted.md"), "quoted should be skipped: {:?}", md);
    assert_eq!(md.iter().filter(|f| f["detail"] == "adr/0001-test-marker.md").count(), 1, "{:?}", md);
    assert_eq!(md.len(), 2, "{:?}", md);
}

// @kotowari[REQ-core-069, TBL-core-014]
#[test]
fn req_069_quoted_text_ending_in_a_multibyte_character_is_split_at_the_quote() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 閉じ引用符の直前が全角文字でも、引用の外の参照は拾う
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: unit\n\nこの値は\"実験\"で決まる。See nonexistent.md for details.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(output.status.code(), Some(1), "should finish with errors, not crash: {:?}", output);
    let v = parse_json(&output);
    let md = findings_by_kind(&v, "missing_document");
    assert!(md.iter().any(|f| f["detail"] == "nonexistent.md"), "{:?}", md);
}

// @kotowari[REQ-core-069, TBL-core-014]
#[test]
fn req_069_dot_md_at_the_start_of_a_line_is_skipped_and_scanning_continues() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 行頭の ".md" は名前を持たないので参照ではない。その先の参照は拾う
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: unit\n\n.md で終わる名前の文書を読む。See nonexistent.md for details.\n",
    )
    .unwrap();
    let output = cmd()
        .arg("check")
        .current_dir(tmp.path())
        .timeout(std::time::Duration::from_secs(10))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1), "should finish, not hang: {:?}", output);
    let v = parse_json(&output);
    let md = findings_by_kind(&v, "missing_document");
    assert!(md.iter().any(|f| f["detail"] == "nonexistent.md"), "{:?}", md);
}

// --- REQ-core-070: 参照された文書が無い ---

// @kotowari[REQ-core-070]
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

// --- REQ-core-104: 具体的な値は二重引用符で書く ---

// @kotowari[REQ-core-104]
#[test]
fn req_104_quoted_values_are_not_terms() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n| IR | 仕様 | docs/decision/records/records.md#A1 |\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: unit\n\n\"kotowari check\" を実行する。\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ut = findings_by_kind(&v, "unknown_term");
    // 二重引用符の中は用語チェックの対象外
    assert!(!ut.iter().any(|f| f["detail"] == "kotowari check"), "quoted values should not be checked: {:?}", ut);
}

// --- REQ-core-063, REQ-core-064: 性質の文とシナリオの手順でも用語を検査する ---

// @kotowari[REQ-core-063, REQ-core-064]
#[test]
fn req_063_property_statements_and_scenario_steps_are_term_checked() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 用語集を作る（既知の語は「テスト」だけ）
    fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n| テスト | 意味 | docs/decision/records/records.md#A1 |\n",
    )
    .unwrap();
    // 性質の文に未知の用語、シナリオの手順に別の未知の用語を使う
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: unit\n\n`テスト`を使う文。\n\n## 性質\n\n### PROP-001: P\n\n- 出典: docs/decision/records/records.md#A1\n\n`性質側の未知語`を検査する。\n\n## 具体例\n\n```gherkin\n@id=EX-001 @about=REQ-001 @source=docs/decision/records/records.md#A1\nScenario: Test\n  Given `手順側の未知語`を使う\n```\n",
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

// --- TBL-core-012: 判断の記録の決定の行の末尾空白 ---

// @kotowari[REQ-core-058, TBL-core-012]
#[test]
fn tbl_012_decision_line_with_and_without_trailing_text() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/decision/records/records2.md"),
        "# 記録2\n\n## Agreements\n\n- A26 agreed\n- A27\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        concat!(
            "# Title\n\nScope.\n\n## 要求\n\n",
            "### REQ-001: Test1\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records2.md#A26\n- 検証: unit\n\nStatement.\n\n",
            "### REQ-002: Test2\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records2.md#A27\n- 検証: unit\n\nStatement2.\n",
        ),
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    assert!(
        !si.iter().any(|f| {
            let d = f["detail"].as_str().unwrap_or("");
            d.contains("A26") || d.contains("A27")
        }),
        "末尾空白ありの A26 と末尾空白なしの A27 の両方で出典が正しいはず: {:?}",
        si
    );
}

// @kotowari[REQ-core-058, TBL-core-012]
#[test]
fn tbl_012_indented_decision_line_counts() {
    // A158: 決定の行は行頭の空白を除いてから判定する（字下げした行も決定）
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/decision/records/records2.md"),
        "# 記録2\n\n## Agreements\n\n  - A26 indented agreement\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records2.md#A26\n- 検証: unit\n\nStatement.\n\n### REQ-002: Test2\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records2.md#A27\n- 検証: unit\n\nStatement.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    let details: Vec<&str> = si.iter().map(|f| f["detail"].as_str().unwrap_or("")).collect();
    // A26（字下げした決定）は解決し、無い番号 A27 だけが source_invalid になる（検査が走ったことの対）
    assert_eq!(
        details,
        vec!["docs/decision/records/records2.md#A27"],
        "an indented decision line ('  - A26 ...') should still count as decision A26: {:?}",
        si
    );
}

// @kotowari[REQ-core-058, TBL-core-012, EX-core-118]
#[test]
fn tbl_012_number_inside_code_block_is_not_a_source_target() {
    // EX-core-118: コードブロックの中の番号の行は読まないので、出典の先にならない
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/decision/records/x.md"),
        concat!(
            "# 記録 x\n\n## Agreements\n\n- A1 ある合意\n\n",
            "```text\n- A9 コードブロックの中の番号\n```\n",
        ),
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/x.md#A9\n- 検証: unit\n\nStatement.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    let details: Vec<&str> = si.iter().map(|f| f["detail"].as_str().unwrap_or("")).collect();
    assert_eq!(
        details,
        vec!["docs/decision/records/x.md#A9"],
        "a number that appears only inside a code block must not resolve as a source: {:?}",
        si
    );
}

// @kotowari[REQ-core-058, TBL-core-012]
#[test]
fn tbl_012_decision_heading_only_inside_code_block_makes_the_file_not_a_record() {
    // A46: 決定の節の見出しがコードブロックの中にしか無いファイルは判断の記録でなく、
    // 印は "## 見出し" で照合される（TBL-core-012 順4）。決定の番号の印は先にならない
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/decision/records/f.md"),
        concat!(
            "# 補足の文書\n\n## 補足\n\n記録の形の例:\n\n",
            "```markdown\n## Agreements\n\n- A1 例の合意\n```\n",
        ),
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/f.md#補足\n- 検証: unit\n\nStatement.\n\n### REQ-002: Test2\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/f.md#A1\n- 検証: unit\n\nStatement2.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    let details: Vec<&str> = si.iter().map(|f| f["detail"].as_str().unwrap_or("")).collect();
    assert_eq!(
        details,
        vec!["docs/decision/records/f.md#A1"],
        "the heading anchor resolves (not a record) and the number anchor does not: {:?}",
        si
    );
}

// @kotowari[REQ-core-058, TBL-core-012, EX-core-120]
#[test]
fn tbl_012_heading_inside_code_block_of_a_non_record_file_is_not_a_source_target() {
    // EX-core-120
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/decision/records/g.md"),
        "# 補足の文書\n\n## 補足\n\n本文。\n\n```markdown\n## 例\n```\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/g.md#補足\n- 検証: unit\n\nStatement.\n\n### REQ-002: Test2\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/g.md#例\n- 検証: unit\n\nStatement2.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    let details: Vec<&str> = si.iter().map(|f| f["detail"].as_str().unwrap_or("")).collect();
    assert_eq!(
        details,
        vec!["docs/decision/records/g.md#例"],
        "a heading inside a code block of a non-record file must not resolve as a source: {:?}",
        si
    );
}

// --- 除外: 隠しディレクトリは辿らない ---

// @kotowari[REQ-core-058]
#[test]
fn req_058_hidden_directory_under_records_is_not_a_source_target() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // .old/ の下に判断の記録らしきファイルを置く
    fs::create_dir_all(tmp.path().join("docs/decision/records/.old")).unwrap();
    fs::write(
        tmp.path().join("docs/decision/records/.old/records.md"),
        "# 昔の記録\n\n## Agreements\n\n- A1 old agreement\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/.old/records.md#A1\n- 検証: unit\n\nStatement.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    assert!(
        si.iter().any(|f| f["detail"] == "docs/decision/records/.old/records.md#A1"),
        "a source pointing under a hidden directory must not resolve: {:?}",
        si
    );
}

// --- 除外: ディレクトリでも通常のファイルでもない要素は静かに読み飛ばす ---

// @kotowari[REQ-core-058]
#[test]
#[cfg(unix)]
fn req_058_non_regular_entry_named_md_under_records_is_silently_skipped() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // ディレクトリでもファイルでもない要素（ここでは Unix ドメインソケット）を
    // records の下に ".md" の名前で置く
    let sock_path = tmp.path().join("docs/decision/records/weird.md");
    let _listener = std::os::unix::net::UnixListener::bind(&sock_path).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "a non-regular filesystem entry named *.md under records must be silently skipped, not read: {:?}",
        output
    );
}

// @kotowari[REQ-core-058]
#[test]
#[cfg(unix)]
fn req_058_non_regular_entry_named_md_under_adr_is_silently_skipped() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    let sock_path = tmp.path().join("docs/decision/adr/weird.md");
    let _listener = std::os::unix::net::UnixListener::bind(&sock_path).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "a non-regular filesystem entry named *.md under adr must be silently skipped, not read: {:?}",
        output
    );
}

// --- REQ-core-040: gherkin コードブロック内の文書名参照は対象外 ---

// @kotowari[REQ-core-040]
#[test]
fn req_040_gherkin_code_block_doc_ref_is_not_checked() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        concat!(
            "# Title\n\nScope.\n\n## 要求\n\n",
            "### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: unit\n\nStatement.\n\n",
            "## 具体例\n\n",
            "```gherkin\n",
            "@id=EX-001 @about=REQ-001 @source=docs/decision/records/records.md#A1\n",
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

// --- is_decision_number ---

// @kotowari[REQ-core-058, TBL-core-012]
#[test]
fn tbl_012_is_decision_number_rejects_invalid_forms() {
    assert!(
        !kotowari_core::sources::is_decision_number("a1"),
        "lowercase letter should not be a decision number"
    );
    assert!(
        !kotowari_core::sources::is_decision_number("1"),
        "single digit should not be a decision number"
    );
    assert!(
        !kotowari_core::sources::is_decision_number("11"),
        "digits only should not be a decision number"
    );
    assert!(
        !kotowari_core::sources::is_decision_number("A"),
        "single uppercase letter should not be a decision number"
    );
    assert!(
        kotowari_core::sources::is_decision_number("A1"),
        "A1 should be a valid decision number"
    );
    assert!(
        kotowari_core::sources::is_decision_number("P26"),
        "P26 should be a valid decision number"
    );
}

// @kotowari[REQ-core-058, TBL-core-012]
#[test]
fn tbl_012_check_source_outside_records_and_adr_returns_err() {
    let ctx = kotowari_core::sources::SourceContext {
        records_path: "docs/decision/records".to_string(),
        adr_path: "docs/decision/adr".to_string(),
        records_files: vec![],
        adr_files: vec![],
        records_other_files: vec![],
    };
    let result = ctx.check_source("somewhere/else.md#heading");
    assert!(result.is_err(), "source outside records/adr should return Err");

    let result2 = ctx.check_source("docs/decision/records#A1");
    assert!(result2.is_err(), "path equal to records_path (no subpath) should return Err");
}

// @kotowari[REQ-core-057]
#[test]
fn req_057_split_source_rejects_an_empty_path_or_empty_anchor() {
    assert_eq!(
        kotowari_core::sources::split_source("#anchor"),
        None,
        "an empty path before '#' must be rejected"
    );
    assert_eq!(
        kotowari_core::sources::split_source("docs/x.md#"),
        None,
        "an empty anchor after '#' must be rejected"
    );
    assert_eq!(
        kotowari_core::sources::split_source("docs/x.md#a"),
        Some(("docs/x.md", "a")),
        "a source with both a path and an anchor is accepted"
    );
}

// @kotowari[REQ-core-058]
#[test]
fn req_058_absolute_path_source_is_rejected_even_if_it_would_otherwise_resolve() {
    let ctx = kotowari_core::sources::SourceContext {
        records_path: "docs/decision/records".to_string(),
        adr_path: "docs/decision/adr".to_string(),
        records_files: vec![kotowari_core::sources::parse_records_file(
            "records.md",
            "# 記録\n\n## Agreements\n\n- A1 ある合意\n",
        )],
        adr_files: vec![],
        records_other_files: vec![],
    };
    // 相対パスなら正しい出典
    assert!(
        ctx.check_source("docs/decision/records/records.md#A1").is_ok(),
        "the relative form should resolve"
    );
    // 先頭に "/" を付けると、正規化後に同じ場所を指しても出典として不正
    assert!(
        ctx.check_source("/docs/decision/records/records.md#A1").is_err(),
        "a source starting with '/' must be rejected even if it would resolve after normalization"
    );
}

// @kotowari[REQ-core-058]
#[test]
fn req_058_tie_break_prefers_the_longer_place_when_a_path_matches_both() {
    let ctx = kotowari_core::sources::SourceContext {
        records_path: "docs".to_string(),
        adr_path: "docs/decision".to_string(),
        records_files: vec![],
        adr_files: vec![kotowari_core::sources::OtherFile {
            rel_path: "adr/0001.md".to_string(),
            headings: vec!["Status".to_string()],
        }],
        records_other_files: vec![],
    };
    // "docs/decision/adr/0001.md" は records ("docs") にも adr ("docs/decision") にも
    // 境界を満たして当たる。長い方の置き場（adr）を採るはずなので、adr 側の
    // ファイルの見出しで解決できる。
    let result = ctx.check_source("docs/decision/adr/0001.md#Status");
    assert!(
        result.is_ok(),
        "when a path matches both places, the longer place should win the tie: {:?}",
        result
    );
}

// @kotowari[REQ-core-058]
#[test]
fn req_058_boundary_violating_prefix_does_not_count_as_under_a_place() {
    let ctx = kotowari_core::sources::SourceContext {
        // records_path はたまたま adr の実ファイル名の接頭辞になっているが、
        // 続く文字が "/" でないので "under" ではない（境界を守る）
        records_path: "docs/decision/adr/0001".to_string(),
        adr_path: "docs/decision/adr".to_string(),
        records_files: vec![],
        adr_files: vec![kotowari_core::sources::OtherFile {
            rel_path: "0001-notes.md".to_string(),
            headings: vec!["Status".to_string()],
        }],
        records_other_files: vec![],
    };
    let result = ctx.check_source("docs/decision/adr/0001-notes.md#Status");
    assert!(
        result.is_ok(),
        "a place string that is merely a byte-prefix without a '/' boundary must not count as 'under' it: {:?}",
        result
    );
}

// --- check_document_references の行番号 ---

// @kotowari[REQ-core-069, TBL-core-014]
#[test]
fn req_069_doc_ref_line_number_is_correct() {
    use std::collections::BTreeSet;
    let content = "# Title\n\nScope.\n\nSee nonexistent.md here.\n";
    let doc = kotowari_core::ir::parse_document("a.md", content);
    let ir_filenames: BTreeSet<String> = [doc.filename.clone()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(
        &[doc],
        "docs/ir",
        &ir_filenames,
        &mut findings,
    );
    let md: Vec<_> = findings.iter().filter(|f| f.kind == "missing_document").collect();
    assert!(
        md.iter().any(|f| f.detail == "nonexistent.md" && f.line == Some(5)),
        "missing_document on line 5: {:?}",
        md
    );
}

// --- find_doc_refs の境界 ---

// @kotowari[REQ-core-069, TBL-core-014]
#[test]
fn req_069_doc_ref_at_line_end() {
    use std::collections::BTreeSet;
    let content = "# Title\n\nScope with nonexistent.md\n";
    let doc = kotowari_core::ir::parse_document("a.md", content);
    let ir_filenames: BTreeSet<String> = [doc.filename.clone()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(&[doc], "docs/ir", &ir_filenames, &mut findings);
    let md: Vec<_> = findings.iter().filter(|f| f.kind == "missing_document").collect();
    assert!(
        md.iter().any(|f| f.detail == "nonexistent.md" && f.line == Some(3)),
        "doc ref at end of line should be detected on line 3: {:?}",
        md
    );
}

// @kotowari[REQ-core-069, TBL-core-014]
#[test]
fn req_069_doc_ref_at_line_start() {
    use std::collections::BTreeSet;
    let content = "# Title\n\nnot-found.md is referenced.\n";
    let doc = kotowari_core::ir::parse_document("a.md", content);
    let ir_filenames: BTreeSet<String> = [doc.filename.clone()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(&[doc], "docs/ir", &ir_filenames, &mut findings);
    let md: Vec<_> = findings.iter().filter(|f| f.kind == "missing_document").collect();
    assert!(
        md.iter().any(|f| f.detail == "not-found.md" && f.line == Some(3)),
        "doc ref at line start should be detected on line 3: {:?}",
        md
    );
}

// @kotowari[REQ-core-069, TBL-core-014]
#[test]
fn req_069_doc_ref_after_punctuation() {
    use std::collections::BTreeSet;
    let content = "# Title\n\nScope.\n\nSee,not-found.md for details.\n";
    let doc = kotowari_core::ir::parse_document("a.md", content);
    let ir_filenames: BTreeSet<String> = [doc.filename.clone()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(&[doc], "docs/ir", &ir_filenames, &mut findings);
    let md: Vec<_> = findings.iter().filter(|f| f.kind == "missing_document").collect();
    assert!(
        md.iter().any(|f| f.detail == "not-found.md" && f.line == Some(5)),
        "doc ref after punctuation should be detected on line 5: {:?}",
        md
    );
}

// @kotowari[REQ-core-069, TBL-core-014]
#[test]
fn req_069_mdx_extension_not_matched_but_md_after_it_is() {
    use std::collections::BTreeSet;
    let content = "# Title\n\nfoo.mdx bar.md text.\n";
    let doc = kotowari_core::ir::parse_document("a.md", content);
    let ir_filenames: BTreeSet<String> = [doc.filename.clone()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(&[doc], "docs/ir", &ir_filenames, &mut findings);
    let md: Vec<_> = findings.iter().filter(|f| f.kind == "missing_document").collect();
    assert!(
        !md.iter().any(|f| f.detail.contains("foo")),
        "foo.mdx should not be matched: {:?}",
        md
    );
    assert!(
        md.iter().any(|f| f.detail == "bar.md"),
        "bar.md should be matched: {:?}",
        md
    );
}

// --- Step 5: 出典と用語 ---

// @kotowari[TBL-core-012]
#[test]
fn tbl_012_file_with_decision_sections_is_a_records_file() {
    // 決定の節の見出しを持つファイルは判断の記録
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 決定の節の見出しを持つが番号の無いファイル
    fs::write(
        tmp.path().join("docs/decision/records/no-numbers.md"),
        "# No numbers\n\n## Agreements\n\nJust text, no decisions.\n",
    ).unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/no-numbers.md#A1\n- 検証: unit\n\nStatement.\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    // 判断の記録として扱われるので番号 A1 で照合 → 見つからないので source_invalid
    assert!(si.iter().any(|f| f["detail"].as_str().unwrap().contains("no-numbers.md#A1")),
        "file with decision sections should be treated as records: {:?}", si);
}

// @kotowari[TBL-core-012]
#[test]
fn tbl_012_file_without_decision_sections_matches_headings() {
    // 決定の節の見出しが無いファイルは見出しで照合
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 決定の節の見出しを持たないファイル
    fs::write(
        tmp.path().join("docs/decision/records/notes.md"),
        "# Notes\n\n## Overview\n\nSome notes.\n",
    ).unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/notes.md#Overview\n- 検証: unit\n\nStatement.\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    // 見出しで照合 → Overview が見つかるので通る
    assert!(si.is_empty(), "file without decision sections should match by headings: {:?}", si);
}

// @kotowari[TBL-core-012]
#[test]
fn tbl_012_two_letter_prefix_is_not_a_decision_number() {
    // "AB1" は決定の番号でない（英大文字1文字に1桁以上の数字）
    assert!(!kotowari_core::sources::is_decision_number("AB1"),
        "AB1 should not be a decision number (two letters)");
    assert!(kotowari_core::sources::is_decision_number("A1"),
        "A1 should be a decision number");
}

// @kotowari[REQ-core-061]
#[test]
fn req_061_subheading_does_not_end_a_section() {
    // "### " の小見出しは節を終えない
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // records.md に ### の小見出しの後に決定の番号を置く
    fs::write(
        tmp.path().join("docs/decision/records/records.md"),
        "# Records\n\n## Agreements\n\n- A1 First agreement\n\n### Subsection\n\n- A2 Second agreement\n",
    ).unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A2\n- 検証: unit\n\nStatement.\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    // ### は節を終えないので A2 は見つかる
    assert!(si.is_empty(), "### subheading should not end a section: {:?}", si);
}

// @kotowari[REQ-core-115]
#[test]
fn req_115_source_invalid_line_is_the_source_line() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 項目の出典が無効な場合、line は "- 出典:" の行
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: somewhere/bad.md#X\n- 検証: unit\n\nStatement.\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    assert!(!si.is_empty(), "should have source_invalid");
    // line は出典の行（10行目: "- 出典: somewhere/bad.md#X"）
    assert_eq!(si[0]["line"], 10, "source_invalid line should be the source line (10), got {:?}", si[0]);
}

// @kotowari[REQ-core-115]
#[test]
fn req_115_decision_table_source_invalid_line_is_the_source_line_not_another_field() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 決定表の項目に、出典より前に別の行（種類）を持たせる
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 決定表\n\n### TBL-001: T\n\n- 種類: foo\n- 出典: somewhere/bad.md#X\n\n| a | b |\n|---|---|\n| 1 | 2 |\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    assert!(!si.is_empty(), "should have source_invalid: {:?}", v);
    // line は出典の行（10行目）であって、それより前の種類の行（9行目）ではない
    assert_eq!(si[0]["line"], 10, "source_invalid line should be the 出典 line (10), not an earlier field line: {:?}", si[0]);
}

// @kotowari[REQ-core-115]
#[test]
fn req_115_property_source_invalid_line_is_the_source_line_not_another_field() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 性質の項目に、出典より前に別の行（種類）を持たせる
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 性質\n\n### PROP-001: P\n\n- 種類: foo\n- 出典: somewhere/bad.md#X\n\nStatement.\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    assert!(!si.is_empty(), "should have source_invalid: {:?}", v);
    assert_eq!(si[0]["line"], 10, "source_invalid line should be the 出典 line (10), not an earlier field line: {:?}", si[0]);
}

// @kotowari[REQ-core-115]
#[test]
fn req_115_flag_entry_source_invalid_line_is_the_source_line_not_another_field() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 問題の記録の項目に、出典より前に別の行（種類）を持たせる
    fs::write(
        tmp.path().join("docs/ir/FLAGS.md"),
        "# 問題の記録\n\n### FLAG-001: Issue\n\n- 種類: gap\n- 出典: somewhere/bad.md#X\n- 関係: REQ-001\n\nBody.\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    assert!(!si.is_empty(), "should have source_invalid: {:?}", v);
    assert_eq!(si[0]["line"], 6, "source_invalid line should be the 出典 line (6), not an earlier field line: {:?}", si[0]);
}

// @kotowari[REQ-core-064]
#[test]
fn req_064_backtick_content_is_trimmed() {
    // "` IR `" は "IR" として照合される
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n| IR | 仕様 | docs/decision/records/records.md#A1 |\n",
    ).unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: unit\n\n` IR `は用語集にある。\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ut = findings_by_kind(&v, "unknown_term");
    assert!(!ut.iter().any(|f| f["detail"] == "IR" || f["detail"] == " IR "),
        "trimmed backtick content should match glossary term: {:?}", ut);
}

// @kotowari[REQ-core-064]
#[test]
fn req_064_empty_backticks_are_unknown_term() {
    // "``" は detail "``" の unknown_term
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n| IR | 仕様 | docs/decision/records/records.md#A1 |\n",
    ).unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: unit\n\nSee `` here.\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ut = findings_by_kind(&v, "unknown_term");
    assert!(ut.iter().any(|f| f["detail"] == "``"),
        "empty backticks should produce unknown_term with detail '``': {:?}", ut);
}

// @kotowari[REQ-core-116]
#[test]
fn req_116_odd_backticks_skip_terms_but_check_vague_words() {
    // 奇数バッククォートの行は unclosed_backtick、用語と ID の検査を飛ばし曖昧語は検査する
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n| IR | 仕様 | docs/decision/records/records.md#A1 |\n",
    ).unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: unit\n\n`奇数のバッククォート 適切に処理する。\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ub = findings_by_kind(&v, "unclosed_backtick");
    assert!(!ub.is_empty(), "odd backticks should produce unclosed_backtick: {:?}", ub);
    let vw = findings_by_kind(&v, "vague_word");
    assert!(vw.iter().any(|f| f["detail"] == "適切に"), "vague words should still be checked: {:?}", vw);
    let ut = findings_by_kind(&v, "unknown_term");
    assert!(ut.is_empty(), "unknown_term should not be checked on odd backtick line: {:?}", ut);
}

// @kotowari[REQ-core-116, TBL-core-008]
#[test]
fn tbl_008_unclosed_backtick_detail_keeps_leading_indentation() {
    // A150: unclosed_backtick の detail は読んだ行そのまま（字下げを含む）
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: unit\n\n  `不完全な引用\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ub = findings_by_kind(&v, "unclosed_backtick");
    assert!(
        ub.iter().any(|f| f["detail"] == "  `不完全な引用"),
        "unclosed_backtick detail should preserve the statement's leading indentation: {:?}",
        ub
    );
}

// @kotowari[REQ-core-067]
#[test]
fn req_067_overlapping_vague_words_longest_match_once() {
    // "など" と "などの" が両方あるとき、"などの" で1件
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\nvague_words:\n  - など\n  - などの\n",
    ).unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: unit\n\nなどの操作をする。\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let vw = findings_by_kind(&v, "vague_word");
    // "などの" で1件だけ
    assert_eq!(vw.len(), 1, "overlapping vague words should match longest once: {:?}", vw);
    assert_eq!(vw[0]["detail"], "などの", "should match 'などの' not 'など': {:?}", vw);
}

// @kotowari[TBL-core-014]
#[test]
fn tbl_014_md_followed_by_letter_is_not_a_reference() {
    // "a.mdX" は参照でない
    use std::collections::BTreeSet;
    let content = "# Title\n\nScope with a.mdX text.\n";
    let doc = kotowari_core::ir::parse_document("a.md", content);
    let ir_filenames: BTreeSet<String> = [doc.filename.clone()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(&[doc], "docs/ir", &ir_filenames, &mut findings);
    let md: Vec<_> = findings.iter().filter(|f| f.kind == "missing_document").collect();
    assert!(md.is_empty(), "a.mdX should not be a reference: {:?}", md);
}

// @kotowari[TBL-core-014]
#[test]
fn tbl_014_unclosed_quote_hides_the_rest_of_the_line() {
    // 奇数の二重引用符の後は参照を拾わない
    use std::collections::BTreeSet;
    let content = "# Title\n\nScope.\n\nSee \"unclosed quote nonexistent.md here.\n";
    let doc = kotowari_core::ir::parse_document("a.md", content);
    let ir_filenames: BTreeSet<String> = [doc.filename.clone()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(&[doc], "docs/ir", &ir_filenames, &mut findings);
    let md: Vec<_> = findings.iter().filter(|f| f.kind == "missing_document").collect();
    assert!(md.is_empty(), "unclosed quote should hide the rest of the line: {:?}", md);
}

// @kotowari[TBL-core-014]
#[test]
fn tbl_014_text_between_the_second_and_third_quote_is_still_scanned() {
    // 二重引用符が奇数（3つ）のとき、行末を隠すのは「最後の」引用符から先だけ。
    // 最初の引用符で打ち切ってはいけない（2つ目と3つ目の間は引用符の外）。
    use std::collections::BTreeSet;
    let content = "# Title\n\nScope.\n\nSee \"note\" and outside.md here \"trail\n";
    let doc = kotowari_core::ir::parse_document("a.md", content);
    let ir_filenames: BTreeSet<String> = [doc.filename.clone()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(&[doc], "docs/ir", &ir_filenames, &mut findings);
    let md: Vec<_> = findings.iter().filter(|f| f.kind == "missing_document").collect();
    assert!(
        md.iter().any(|f| f.detail == "outside.md"),
        "text between the 2nd and 3rd quote is outside quotes and should still be scanned: {:?}",
        md
    );
}

// @kotowari[TBL-core-014]
#[test]
fn tbl_014_long_digit_run_before_mdx_does_not_produce_a_spurious_reference() {
    // ".mdx" の直前が長い数字の並びでも、".mdx" は参照として拾わない
    use std::collections::BTreeSet;
    let content = "# Title\n\n01234567890123456789012.mdx\n";
    let doc = kotowari_core::ir::parse_document("a.md", content);
    let ir_filenames: BTreeSet<String> = [doc.filename.clone()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(&[doc], "docs/ir", &ir_filenames, &mut findings);
    let md: Vec<_> = findings.iter().filter(|f| f.kind == "missing_document").collect();
    assert!(
        !md.iter().any(|f| f.detail.contains("01234567890123456789012")),
        "a long digit run before .mdx must not be treated as a .md reference: {:?}",
        md
    );
}

// @kotowari[TBL-core-014]
#[test]
fn tbl_014_md_followed_by_hyphen_is_not_a_reference() {
    // ".md" の直後が "-" のときは参照でない
    use std::collections::BTreeSet;
    let content = "# Title\n\na.md-suffix\n";
    let doc = kotowari_core::ir::parse_document("x.md", content);
    let ir_filenames: BTreeSet<String> = [doc.filename.clone()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(&[doc], "docs/ir", &ir_filenames, &mut findings);
    let md: Vec<_> = findings.iter().filter(|f| f.kind == "missing_document").collect();
    assert!(
        !md.iter().any(|f| f.detail == "a.md"),
        "'a.md-suffix' must not be treated as a reference to 'a.md': {:?}",
        md
    );
}

// @kotowari[TBL-core-014]
#[test]
fn tbl_014_bare_dot_md_with_nothing_before_it_is_not_a_reference() {
    // 直前に文字が無い（空白の直後の）裸の ".md" は参照でない
    use std::collections::BTreeSet;
    let content = "# Title\n\nthe .md file\n";
    let doc = kotowari_core::ir::parse_document("a.md", content);
    let ir_filenames: BTreeSet<String> = [doc.filename.clone()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(&[doc], "docs/ir", &ir_filenames, &mut findings);
    let md: Vec<_> = findings.iter().filter(|f| f.kind == "missing_document").collect();
    assert!(
        !md.iter().any(|f| f.detail == ".md"),
        "a bare '.md' with no name before it must not be treated as a document reference: {:?}",
        md
    );
}

// @kotowari[REQ-core-117]
#[test]
fn req_117_second_table_is_not_glossary() {
    // 用語集の2つ目の表は用語にならない
    let content = "\
# 用語集

| 用語 | 意味 | 出典 |
|---|---|---|
| テスト | 意味 | brainstorm/records.md#A1 |

Some text.

| 用語 | 意味 | 出典 |
|---|---|---|
| 二番目 | 意味2 | brainstorm/records.md#A1 |
";
    let doc = kotowari_core::ir::parse_document("CONTEXT.md", content);
    let terms: Vec<_> = doc.items.iter()
        .filter(|i| matches!(i, kotowari_core::ir::Item::GlossaryTerm { .. }))
        .collect();
    assert_eq!(terms.len(), 1, "second table should not be parsed as glossary: {:?}", terms);
    if let kotowari_core::ir::Item::GlossaryTerm { term, .. } = &terms[0] {
        assert_eq!(term, "テスト", "only first table terms should be parsed");
    }
}

// @kotowari[REQ-core-117]
#[test]
fn req_117_glossary_without_proper_table_is_invalid() {
    // 用語集にヘッダの列名が違う表しかない → glossary_invalid
    let content = "\
# 用語集

| Name | Meaning | Source |
|---|---|---|
| test | meaning | brainstorm/records.md#A1 |
";
    let doc = kotowari_core::ir::parse_document("CONTEXT.md", content);
    let config = kotowari_core::config::Config::default();
    let findings = kotowari_core::ir::check_documents(&[doc], &config);
    let gi: Vec<_> = findings.iter().filter(|f| f.kind == "glossary_invalid").collect();
    assert!(!gi.is_empty(), "glossary without proper table should produce glossary_invalid: {:?}", gi);
    assert!(gi[0].line.is_none(), "glossary_invalid line should be null");
    assert_eq!(gi[0].detail, "CONTEXT.md", "glossary_invalid detail should be filename");
}

// --- TBL-core-001: 非 UTF-8 の判断の記録または ADR で停止 ---

// @kotowari[REQ-core-006, TBL-core-001]
#[test]
fn tbl_001_non_utf8_records_or_adr_stops() {
    // 判断の記録に非 UTF-8 ファイルを置くと停止する
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/decision/records/bad.md"),
        b"\xff\xfe",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(
        output.status.code(),
        Some(2),
        "non-UTF-8 records file should stop with exit code 2"
    );
    assert!(output.stdout.is_empty(), "stdout should be empty on stop");

    // ADR に非 UTF-8 ファイルを置くと停止する
    let tmp2 = TempDir::new().unwrap();
    make_project_with_records(tmp2.path());
    fs::write(
        tmp2.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n",
    )
    .unwrap();
    fs::write(
        tmp2.path().join("docs/decision/adr/bad.md"),
        b"\xff\xfe",
    )
    .unwrap();
    let output2 = cmd().arg("check").current_dir(tmp2.path()).output().unwrap();
    assert_eq!(
        output2.status.code(),
        Some(2),
        "non-UTF-8 ADR file should stop with exit code 2"
    );
    assert!(output2.stdout.is_empty(), "stdout should be empty on stop");
}

// @kotowari[TBL-core-012]
#[test]
fn tbl_012_records_file_headings_are_not_sources() {
    // 決定の節の見出しを持つファイルは判断の記録なので、"## " の見出しでは照合しない（A134）
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/decision/records/notes.md"),
        "# Notes\n\n## Agreements\n\nJust text, no decisions.\n\n## Background\n\nSome background.\n",
    ).unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/notes.md#Background\n- 検証: unit\n\nStatement.\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    assert!(si.iter().any(|f| f["detail"].as_str().unwrap().contains("notes.md#Background")),
        "a records file (has a decision section) must be matched by decision numbers, not headings: {:?}", si);
}

// @kotowari[REQ-core-018, TBL-core-001]
#[test]
#[cfg(unix)]
fn req_018_broken_symlink_in_records_dir_stops() {
    use std::os::unix::fs::symlink;
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    symlink(tmp.path().join("nowhere.md"), tmp.path().join("docs/decision/records/broken.md")).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(output.status.code(), Some(2), "a broken symlink must stop: {:?}", output);
    assert!(output.stdout.is_empty());
}

// @kotowari[REQ-core-033, REQ-core-018]
#[test]
#[cfg(unix)]
fn req_033_file_symlink_in_records_dir_is_read() {
    use std::os::unix::fs::symlink;
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::create_dir_all(tmp.path().join("elsewhere")).unwrap();
    fs::write(tmp.path().join("elsewhere/more.md"), "# More\n\n## Agreements\n\n- A7 linked decision\n").unwrap();
    symlink(tmp.path().join("elsewhere/more.md"), tmp.path().join("docs/decision/records/more.md")).unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/more.md#A7\n- 検証: unit\n\nStatement.\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    assert!(si.is_empty(), "a file symlink in the records dir must be read: {:?}", si);
}

// @kotowari[REQ-core-117]
#[test]
fn req_117_glossary_header_without_rows_is_valid() {
    // ヘッダと区切りの行があれば表は「ある」（A148）
    let content = "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n";
    let doc = kotowari_core::ir::parse_document("CONTEXT.md", content);
    let config = kotowari_core::config::Config::default();
    let findings = kotowari_core::ir::check_documents(&[doc], &config);
    assert!(!findings.iter().any(|f| f.kind == "glossary_invalid"), "header + separator with no rows must not be glossary_invalid: {:?}", findings);
}

// @kotowari[REQ-core-111]
#[test]
fn req_111_bom_in_config_records_adr_and_tests_is_skipped_end_to_end() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    let bom = "\u{feff}";
    fs::write(tmp.path().join(".kotowari/config.yaml"), format!("{bom}ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\n")).unwrap();
    fs::write(tmp.path().join("docs/decision/records/records.md"), format!("{bom}# Records\n\n## Agreements\n\n- A1 first\n")).unwrap();
    fs::write(tmp.path().join("docs/decision/adr/0001-test.md"), format!("{bom}# ADR 0001\n\n## 状況\n\nx\n")).unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        format!("{bom}# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1, docs/decision/adr/0001-test.md#状況\n- 検証: unit\n\nStatement.\n"),
    ).unwrap();
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(tmp.path().join("tests/t.rs"), format!("{bom}// @kotowari[REQ-001]\n#[test]\nfn t() {{}}\n")).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(output.status.code(), Some(0), "BOM in every file kind must be skipped: {:?}", output);
    let v = parse_json(&output);
    assert!(v["findings"].as_array().unwrap().is_empty(), "{:?}", v);
}

// @kotowari[REQ-core-058, REQ-core-110]
#[test]
fn req_058_records_place_dot_resolves_a_source_at_the_base_root() {
    // 置き場 "." は空に正規化される（REQ-core-110）。そのとき基準の直下の判断の記録の出典が解決しなければならない
    let tmp = TempDir::new().unwrap();
    fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    fs::create_dir_all(tmp.path().join("docs/ir")).unwrap();
    fs::create_dir_all(tmp.path().join("adr")).unwrap();
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: .\n  adr: adr\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("records.md"),
        "# 判断の記録\n\n## Agreements\n\n- A1 最初の合意\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: records.md#A1\n- 検証: unit\n\nStatement.\n\n### REQ-002: Test2\n\n- 種類: ubiquitous\n- 出典: records.md#A99\n- 検証: unit\n\nStatement.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    let details: Vec<&str> = si.iter().map(|f| f["detail"].as_str().unwrap_or("")).collect();
    assert_eq!(details, vec!["records.md#A99"], "A1 must resolve and only A99 must be invalid: {:?}", si);
}

fn write_ir(tmp: &std::path::Path, relative: &str, content: &str) {
    let path = tmp.join("docs/ir").join(relative);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, content).unwrap();
}

fn glossary(term: &str) -> String {
    format!("# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n| {term} | 意味 | docs/decision/records/records.md#A1 |\n")
}

fn term_statement(id: &str, statement: &str) -> String {
    format!("# Title\n\nScope.\n\n### {id}: Name\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: review\n\n{statement}\n")
}

// @kotowari[REQ-core-064]
#[test]
fn req_064_term_from_a_sibling_glossary_is_unknown() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_ir(tmp.path(), "network/publish/CONTEXT.md", &glossary("公開"));
    write_ir(tmp.path(), "network/dns/a.md", &term_statement("REQ-001", "`公開`"));
    let result = parse_json(&cmd().arg("check").current_dir(tmp.path()).output().unwrap());
    let unknown = findings_by_kind(&result, "unknown_term");
    assert_eq!(unknown.len(), 1);
    assert_eq!(unknown[0]["path"], "docs/ir/network/dns/a.md");
    assert_eq!(unknown[0]["detail"], "公開");
}

// @kotowari[REQ-core-064]
#[test]
fn req_064_term_from_a_parent_glossary_is_visible() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_ir(tmp.path(), "CONTEXT.md", &glossary("根"));
    write_ir(tmp.path(), "network/CONTEXT.md", &glossary("網"));
    write_ir(tmp.path(), "network/dns/a.md", &term_statement("REQ-001", "`根` `網`"));
    write_ir(tmp.path(), "a.md", &term_statement("REQ-002", "`根` `網`"));
    let result = parse_json(&cmd().arg("check").current_dir(tmp.path()).output().unwrap());
    let unknown = findings_by_kind(&result, "unknown_term");
    assert_eq!(unknown.len(), 1);
    assert_eq!(unknown[0]["path"], "docs/ir/a.md");
    assert_eq!(unknown[0]["detail"], "網");
}

// @kotowari[REQ-core-065, EX-core-026]
#[test]
fn req_065_document_with_no_glossary_in_its_chain_flags_every_backtick() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_ir(tmp.path(), "network/CONTEXT.md", &glossary("網"));
    write_ir(tmp.path(), "a.md", &term_statement("REQ-001", "`網` `未定義` `REQ-001`"));
    write_ir(tmp.path(), "network/a.md", &term_statement("REQ-002", "`網`"));
    let result = parse_json(&cmd().arg("check").current_dir(tmp.path()).output().unwrap());
    let unknown = findings_by_kind(&result, "unknown_term");
    assert_eq!(unknown.iter().map(|f| f["detail"].as_str().unwrap()).collect::<Vec<_>>(), ["未定義", "網"]);
    assert!(unknown.iter().all(|f| f["path"] == "docs/ir/a.md"));
}

// @kotowari[REQ-core-123, TBL-core-019]
#[test]
fn req_123_duplicate_across_the_chain_is_reported_on_the_deeper_row() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_ir(tmp.path(), "CONTEXT.md", &glossary("宛先"));
    write_ir(tmp.path(), "network/CONTEXT.md", "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n| 宛先 | 意味 | |\n| 宛先 | 意味 | invalid |\n");
    write_ir(tmp.path(), "network/dns/CONTEXT.md", &glossary("宛先").replace("docs/decision/records/records.md#A1", "invalid"));
    write_ir(tmp.path(), "network/dns/a.md", &term_statement("REQ-001", "`宛先`"));
    let result = parse_json(&cmd().arg("check").current_dir(tmp.path()).output().unwrap());
    let duplicates = findings_by_kind(&result, "duplicate_term");
    assert_eq!(duplicates.len(), 3);
    assert_eq!(duplicates.iter().map(|f| (f["path"].as_str().unwrap(), f["line"].as_u64().unwrap())).collect::<Vec<_>>(),
        [("docs/ir/network/CONTEXT.md", 5), ("docs/ir/network/CONTEXT.md", 6), ("docs/ir/network/dns/CONTEXT.md", 5)]);
    assert!(duplicates.iter().all(|f| f["detail"] == "宛先"));
    assert!(findings_by_kind(&result, "unknown_term").is_empty());
    assert!(findings_by_kind(&result, "missing_source").is_empty());
    assert!(findings_by_kind(&result, "source_invalid").is_empty());
}

// @kotowari[REQ-core-069, TBL-core-014]
#[test]
fn tbl_014_md_followed_by_hash_or_slash_is_not_a_reference() {
    let doc = kotowari_core::ir::parse_document(
        "x.md",
        "# Title\n\na.md#A12 docs/decision/records/records.md#A12 a.md/b.md a//b.md\n",
    );
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(
        &[doc], "docs/ir", &Default::default(), &mut findings,
    );
    assert!(findings.is_empty(), "{:?}", findings);
}

// @kotowari[REQ-core-069, REQ-core-070, TBL-core-014]
#[test]
fn tbl_014_slash_separated_path_is_a_reference() {
    let doc = kotowari_core::ir::parse_document(
        "a.md", "# Title\n\nSee network/dns/b.md.\n",
    );
    let mut findings = Vec::new();
    kotowari_core::terms::check_document_references(
        &[doc], "docs/ir", &Default::default(), &mut findings,
    );
    assert_eq!(findings.len(), 1, "{:?}", findings);
    assert_eq!(findings[0].detail, "network/dns/b.md");
}

// @kotowari[REQ-core-070, TBL-core-014, EX-core-023]
#[test]
fn req_070_bare_name_resolves_in_the_same_directory_only() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::create_dir_all(tmp.path().join("docs/ir/network/dns")).unwrap();
    fs::write(tmp.path().join("docs/ir/network/dns/a.md"), "# Title\n\nSee b.md.\n").unwrap();
    fs::write(tmp.path().join("docs/ir/network/b.md"), "# Title\n\nScope.\n").unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let missing = findings_by_kind(&parse_json(&output), "missing_document");
    assert_eq!(missing.len(), 1, "{:?}", missing);
    assert_eq!(missing[0]["path"], "docs/ir/network/dns/a.md");
    assert_eq!(missing[0]["detail"], "b.md");

    fs::write(tmp.path().join("docs/ir/network/dns/b.md"), "# Title\n\nScope.\n").unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert!(findings_by_kind(&parse_json(&output), "missing_document").is_empty());
}

// @kotowari[REQ-core-070, TBL-core-014, EX-core-024]
#[test]
fn req_070_slash_path_resolves_from_the_ir_root() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_ir(tmp.path(), "network/dns/a.md", "# Title\n\nSee network/publish/c.md.\n");
    write_ir(tmp.path(), "network/publish/c.md", "# Title\n\nScope.\n");
    let result = parse_json(&cmd().arg("check").current_dir(tmp.path()).output().unwrap());
    assert!(findings_by_kind(&result, "missing_document").is_empty(), "{:?}", result);
    assert_eq!(result["files"], 2);
}

// @kotowari[REQ-core-070, TBL-core-014, EX-core-025]
#[test]
fn req_070_dot_and_dotdot_elements_never_resolve() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_ir(tmp.path(), "network/dns/a.md", "# Title\n\n../b.md ./c.md\n");
    write_ir(tmp.path(), "network/b.md", "# Title\n\nScope.\n");
    write_ir(tmp.path(), "network/dns/c.md", "# Title\n\nScope.\n");
    let result = parse_json(&cmd().arg("check").current_dir(tmp.path()).output().unwrap());
    let missing = findings_by_kind(&result, "missing_document");
    assert_eq!(missing.len(), 2, "{:?}", missing);
    for reference in ["../b.md", "./c.md"] {
        assert_eq!(missing.iter().filter(|f| f["detail"] == reference).count(), 1);
    }
    assert!(missing.iter().all(|f| f["path"] == "docs/ir/network/dns/a.md"));
}

// @kotowari[REQ-core-069, REQ-core-070, TBL-core-014, TBL-core-008, EX-core-014]
#[test]
fn req_070_detail_is_the_whole_reference() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_ir(tmp.path(), "a.md", "# Title\n\ndocs/decision/adr/0001-test-marker.md\n");
    let result = parse_json(&cmd().arg("check").current_dir(tmp.path()).output().unwrap());
    let missing = findings_by_kind(&result, "missing_document");
    assert_eq!(missing.len(), 1, "{:?}", missing);
    assert_eq!(missing[0]["detail"], "docs/decision/adr/0001-test-marker.md");
}

// @kotowari[REQ-core-033, REQ-core-070, TBL-core-014, EX-core-032]
#[cfg(unix)]
#[test]
fn req_070_document_under_a_directory_symlink_is_missing() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::create_dir_all(tmp.path().join("outside")).unwrap();
    fs::write(tmp.path().join("outside/d.md"), "# Title\n\nScope.\n").unwrap();
    std::os::unix::fs::symlink("../../outside", tmp.path().join("docs/ir/link")).unwrap();
    assert!(tmp.path().join("docs/ir/link/d.md").is_file());
    write_ir(tmp.path(), "a.md", "# Title\n\nlink/d.md\n");
    let result = parse_json(&cmd().arg("check").current_dir(tmp.path()).output().unwrap());
    let missing = findings_by_kind(&result, "missing_document");
    assert_eq!(missing.len(), 1, "{:?}", missing);
    assert_eq!(missing[0]["detail"], "link/d.md");
}

// @kotowari[REQ-core-069, TBL-core-014, EX-core-033]
#[test]
fn tbl_014_reference_after_a_japanese_character_is_recognized() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_ir(tmp.path(), "a.md", "# Title\n\n設定の形はtimeout-config.mdで定める\n");
    let result = parse_json(&cmd().arg("check").current_dir(tmp.path()).output().unwrap());
    let missing = findings_by_kind(&result, "missing_document");
    assert_eq!(missing.len(), 1, "{:?}", missing);
    assert_eq!(missing[0]["detail"], "timeout-config.md");
}

// @kotowari[REQ-core-064, REQ-core-069, TBL-core-014, EX-core-034]
#[test]
fn tbl_014_backticked_path_is_a_term_not_a_reference() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_ir(tmp.path(), "x.md", &term_statement("REQ-001", "`a.md`"));
    let result = parse_json(&cmd().arg("check").current_dir(tmp.path()).output().unwrap());
    let unknown = findings_by_kind(&result, "unknown_term");
    assert_eq!(unknown.len(), 1, "{:?}", unknown);
    assert_eq!(unknown[0]["detail"], "a.md");
    assert!(findings_by_kind(&result, "missing_document").is_empty(), "{:?}", result);
}

// @kotowari[REQ-core-117, REQ-core-064]
#[test]
fn req_117_invalid_glossary_hides_only_its_own_terms() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_ir(tmp.path(), "CONTEXT.md", &glossary("根"));
    write_ir(tmp.path(), "network/CONTEXT.md", &glossary("網").replace("| 用語 | 意味 | 出典 |", "| Name | Meaning | Source |"));
    write_ir(tmp.path(), "network/a.md", &term_statement("REQ-001", "`根` `網`"));
    let result = parse_json(&cmd().arg("check").current_dir(tmp.path()).output().unwrap());
    let invalid = findings_by_kind(&result, "glossary_invalid");
    assert_eq!(invalid.len(), 1, "{:?}", invalid);
    assert_eq!(invalid[0]["path"], "docs/ir/network/CONTEXT.md");
    assert_eq!(invalid[0]["detail"], "CONTEXT.md");
    let unknown = findings_by_kind(&result, "unknown_term");
    assert_eq!(unknown.len(), 1, "{:?}", unknown);
    assert_eq!(unknown[0]["detail"], "網");
    assert_eq!(unknown[0]["path"], "docs/ir/network/a.md");
}

// --- REQ-core-062, REQ-core-068: 見ないもの ---

// @kotowari[REQ-core-062]
#[test]
fn req_062_source_content_is_not_matched_against_the_item() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // REQ-core-001 の出典 A1 は項目の内容とまったく関係が無い。
    // REQ-core-002 の出典は存在しない決定を指すので、出典の検査が動いていることが分かる。
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: 出力の形\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: review\n\nこの道具は JSON を出す。\n\n### REQ-002: 別の要求\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A9\n- 検証: review\n\nこの道具は文書を読む。\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let invalid = findings_by_kind(&v, "source_invalid");
    assert_eq!(
        invalid.len(),
        1,
        "only the source pointing at a missing decision should be reported: {invalid:?}"
    );
    assert_eq!(
        invalid[0]["line"], 18,
        "the reported source should be the missing one, not the unrelated one: {invalid:?}"
    );
}

// @kotowari[REQ-core-068]
#[test]
fn req_068_unquoted_term_gets_no_finding_on_its_line() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n| 指摘 | 検査で見つけた1件 | docs/decision/records/records.md#A1 |\n",
    )
    .unwrap();
    // 13行目の文は、用語集にある「指摘」をバッククォートで囲まずに書いている
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: unit\n\nこの道具は指摘を出す。\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let on_statement: Vec<_> = v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["path"] == "docs/ir/a.md" && f["line"] == 13)
        .collect();
    assert!(
        on_statement.is_empty(),
        "the statement line should get no finding: {on_statement:?}"
    );
    // 検査そのものは動いている（要求の見出しの行にはテストのない要求の誤りが出る）
    let on_heading: Vec<_> = v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["path"] == "docs/ir/a.md" && f["line"] == 7)
        .collect();
    assert!(
        !on_heading.is_empty(),
        "the requirement heading should still be checked: {v}"
    );
}

// @kotowari[REQ-core-123, EX-core-027]
#[test]
fn req_123_same_term_above_and_below_the_chain_is_reported_on_the_lower_row() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_ir(tmp.path(), "CONTEXT.md", &glossary("宛先"));
    write_ir(tmp.path(), "network/CONTEXT.md", &glossary("宛先"));
    write_ir(tmp.path(), "network/a.md", &term_statement("REQ-001", "`宛先`"));
    let result = parse_json(&cmd().arg("check").current_dir(tmp.path()).output().unwrap());
    let duplicates = findings_by_kind(&result, "duplicate_term");
    assert_eq!(duplicates.len(), 1, "only the lower row is the duplicate: {:?}", result);
    assert_eq!(duplicates[0]["path"], "docs/ir/network/CONTEXT.md");
    assert_eq!(duplicates[0]["detail"], "宛先");
    assert!(
        findings_by_kind(&result, "unknown_term").is_empty(),
        "the term is still known under network/: {:?}",
        result
    );
}

// --- terms.md の具体例 ---

// @kotowari[REQ-core-064, EX-core-013]
#[test]
fn req_064_a_backticked_path_outside_the_glossary_is_an_unknown_term() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_ir(tmp.path(), "a.md", &term_statement("REQ-001", "`src/main.rs`"));
    let result = parse_json(&cmd().arg("check").current_dir(tmp.path()).output().unwrap());
    let unknown = findings_by_kind(&result, "unknown_term");
    assert_eq!(unknown.len(), 1, "{:?}", result);
    assert_eq!(unknown[0]["detail"], "src/main.rs");
}

// @kotowari[REQ-core-069, EX-core-022]
#[test]
fn req_069_a_source_shaped_path_in_the_scope_line_is_not_a_reference() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_ir(
        tmp.path(),
        "a.md",
        "# Title\n\n範囲は docs/decision/records/records.md#A12 で決めた。\n",
    );
    let result = parse_json(&cmd().arg("check").current_dir(tmp.path()).output().unwrap());
    assert!(
        findings_by_kind(&result, "missing_document").is_empty(),
        "a path followed by # is a source, not a document reference: {:?}",
        result
    );
}

// @kotowari[REQ-core-069, EX-core-031]
#[test]
fn req_069_md_followed_by_a_slash_is_not_a_reference() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 文書の名前は x.md にする。"a.md" も "b.md" も "md/b.md" も置き場に無い
    write_ir(tmp.path(), "x.md", "# Title\n\na.md/b.md\n");
    let result = parse_json(&cmd().arg("check").current_dir(tmp.path()).output().unwrap());
    assert!(
        findings_by_kind(&result, "missing_document").is_empty(),
        "a path followed by / is not a document reference: {:?}",
        result
    );
}
