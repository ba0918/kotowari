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

// @kotowari[REQ-058, TBL-012]
#[test]
fn req_058_source_path_equal_to_a_place_itself_is_invalid_without_crashing() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // パスが置き場そのもの（末尾に "/..." が無い）は、置き場の中ではない
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/adr#状況\n- 検証: unit\n\nStatement.\n\n### REQ-002: Test2\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm#A1\n- 検証: unit\n\nStatement2.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(output.status.code(), Some(1), "should finish with errors, not crash: {:?}", output);
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    assert_eq!(si.len(), 2, "{:?}", si);
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

// @kotowari[REQ-014]
#[test]
fn req_014_empty_vague_word_stops_with_config_error() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
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
    assert_eq!(
        output.status.code(),
        Some(2),
        "empty vague word should stop with exit code 2"
    );
    assert!(output.stdout.is_empty(), "stdout should be empty on stop");
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

// @kotowari[REQ-069, TBL-014]
#[test]
fn req_069_quoted_text_ending_in_a_multibyte_character_is_split_at_the_quote() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 閉じ引用符の直前が全角文字でも、引用の外の参照は拾う
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A1\n- 検証: unit\n\nこの値は\"実験\"で決まる。See nonexistent.md for details.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(output.status.code(), Some(1), "should finish with errors, not crash: {:?}", output);
    let v = parse_json(&output);
    let md = findings_by_kind(&v, "missing_document");
    assert!(md.iter().any(|f| f["detail"] == "nonexistent.md"), "{:?}", md);
}

// @kotowari[REQ-069, TBL-014]
#[test]
fn req_069_dot_md_at_the_start_of_a_line_is_skipped_and_scanning_continues() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 行頭の ".md" は名前を持たないので参照ではない。その先の参照は拾う
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A1\n- 検証: unit\n\n.md で終わる名前の文書を読む。See nonexistent.md for details.\n",
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

// --- TBL-012: 判断の記録の決定の行の末尾空白 ---

// @kotowari[REQ-058, TBL-012]
#[test]
fn tbl_012_decision_line_with_and_without_trailing_text() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/decision/brainstorm/records2.md"),
        "# 記録2\n\n## Agreements\n\n- A26 agreed\n- A27\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        concat!(
            "# Title\n\nScope.\n\n## 要求\n\n",
            "### REQ-001: Test1\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records2.md#A26\n- 検証: unit\n\nStatement.\n\n",
            "### REQ-002: Test2\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records2.md#A27\n- 検証: unit\n\nStatement2.\n",
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

// @kotowari[REQ-058, TBL-012]
#[test]
fn tbl_012_indented_decision_line_counts() {
    // A158: 決定の行は行頭の空白を除いてから判定する（字下げした行も決定）
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/decision/brainstorm/records2.md"),
        "# 記録2\n\n## Agreements\n\n  - A26 indented agreement\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records2.md#A26\n- 検証: unit\n\nStatement.\n\n### REQ-002: Test2\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records2.md#A27\n- 検証: unit\n\nStatement.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    let details: Vec<&str> = si.iter().map(|f| f["detail"].as_str().unwrap_or("")).collect();
    // A26（字下げした決定）は解決し、無い番号 A27 だけが source_invalid になる（検査が走ったことの対）
    assert_eq!(
        details,
        vec!["docs/decision/brainstorm/records2.md#A27"],
        "an indented decision line ('  - A26 ...') should still count as decision A26: {:?}",
        si
    );
}

// --- 除外: 隠しディレクトリは辿らない ---

// @kotowari[REQ-058]
#[test]
fn req_058_hidden_directory_under_records_is_not_a_source_target() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // .old/ の下に判断の記録らしきファイルを置く
    fs::create_dir_all(tmp.path().join("docs/decision/brainstorm/.old")).unwrap();
    fs::write(
        tmp.path().join("docs/decision/brainstorm/.old/records.md"),
        "# 昔の記録\n\n## Agreements\n\n- A1 old agreement\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/.old/records.md#A1\n- 検証: unit\n\nStatement.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    assert!(
        si.iter().any(|f| f["detail"] == "docs/decision/brainstorm/.old/records.md#A1"),
        "a source pointing under a hidden directory must not resolve: {:?}",
        si
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

// --- is_decision_number ---

// @kotowari[REQ-058, TBL-012]
#[test]
fn tbl_012_is_decision_number_rejects_invalid_forms() {
    assert!(
        !kotowari::sources::is_decision_number("a1"),
        "lowercase letter should not be a decision number"
    );
    assert!(
        !kotowari::sources::is_decision_number("1"),
        "single digit should not be a decision number"
    );
    assert!(
        !kotowari::sources::is_decision_number("11"),
        "digits only should not be a decision number"
    );
    assert!(
        !kotowari::sources::is_decision_number("A"),
        "single uppercase letter should not be a decision number"
    );
    assert!(
        kotowari::sources::is_decision_number("A1"),
        "A1 should be a valid decision number"
    );
    assert!(
        kotowari::sources::is_decision_number("P26"),
        "P26 should be a valid decision number"
    );
}

// @kotowari[REQ-058, TBL-012]
#[test]
fn tbl_012_check_source_outside_records_and_adr_returns_err() {
    let ctx = kotowari::sources::SourceContext {
        records_path: "docs/decision/brainstorm".to_string(),
        adr_path: "docs/decision/adr".to_string(),
        records_files: vec![],
        adr_files: vec![],
        records_other_files: vec![],
    };
    let result = ctx.check_source("somewhere/else.md#heading");
    assert!(result.is_err(), "source outside records/adr should return Err");

    let result2 = ctx.check_source("docs/decision/brainstorm#A1");
    assert!(result2.is_err(), "path equal to records_path (no subpath) should return Err");
}

// --- check_document_references の行番号 ---

// @kotowari[REQ-069, TBL-014]
#[test]
fn req_069_doc_ref_line_number_is_correct() {
    use std::collections::BTreeSet;
    let content = "# Title\n\nScope.\n\nSee nonexistent.md here.\n";
    let doc = kotowari::ir::parse_document("a.md", content);
    let ir_filenames: BTreeSet<String> = [doc.filename.clone()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari::terms::check_document_references(
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

// @kotowari[REQ-069, TBL-014]
#[test]
fn req_069_doc_ref_at_line_end() {
    use std::collections::BTreeSet;
    let content = "# Title\n\nScope with nonexistent.md\n";
    let doc = kotowari::ir::parse_document("a.md", content);
    let ir_filenames: BTreeSet<String> = [doc.filename.clone()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari::terms::check_document_references(&[doc], "docs/ir", &ir_filenames, &mut findings);
    let md: Vec<_> = findings.iter().filter(|f| f.kind == "missing_document").collect();
    assert!(
        md.iter().any(|f| f.detail == "nonexistent.md" && f.line == Some(3)),
        "doc ref at end of line should be detected on line 3: {:?}",
        md
    );
}

// @kotowari[REQ-069, TBL-014]
#[test]
fn req_069_doc_ref_at_line_start() {
    use std::collections::BTreeSet;
    let content = "# Title\n\nnot-found.md is referenced.\n";
    let doc = kotowari::ir::parse_document("a.md", content);
    let ir_filenames: BTreeSet<String> = [doc.filename.clone()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari::terms::check_document_references(&[doc], "docs/ir", &ir_filenames, &mut findings);
    let md: Vec<_> = findings.iter().filter(|f| f.kind == "missing_document").collect();
    assert!(
        md.iter().any(|f| f.detail == "not-found.md" && f.line == Some(3)),
        "doc ref at line start should be detected on line 3: {:?}",
        md
    );
}

// @kotowari[REQ-069, TBL-014]
#[test]
fn req_069_doc_ref_after_punctuation() {
    use std::collections::BTreeSet;
    let content = "# Title\n\nScope.\n\nSee,not-found.md for details.\n";
    let doc = kotowari::ir::parse_document("a.md", content);
    let ir_filenames: BTreeSet<String> = [doc.filename.clone()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari::terms::check_document_references(&[doc], "docs/ir", &ir_filenames, &mut findings);
    let md: Vec<_> = findings.iter().filter(|f| f.kind == "missing_document").collect();
    assert!(
        md.iter().any(|f| f.detail == "not-found.md" && f.line == Some(5)),
        "doc ref after punctuation should be detected on line 5: {:?}",
        md
    );
}

// @kotowari[REQ-069, TBL-014]
#[test]
fn req_069_mdx_extension_not_matched_but_md_after_it_is() {
    use std::collections::BTreeSet;
    let content = "# Title\n\nfoo.mdx bar.md text.\n";
    let doc = kotowari::ir::parse_document("a.md", content);
    let ir_filenames: BTreeSet<String> = [doc.filename.clone()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari::terms::check_document_references(&[doc], "docs/ir", &ir_filenames, &mut findings);
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

// @kotowari[TBL-012]
#[test]
fn tbl_012_file_with_decision_sections_is_a_records_file() {
    // 決定の節の見出しを持つファイルは判断の記録
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 決定の節の見出しを持つが番号の無いファイル
    fs::write(
        tmp.path().join("docs/decision/brainstorm/no-numbers.md"),
        "# No numbers\n\n## Agreements\n\nJust text, no decisions.\n",
    ).unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/no-numbers.md#A1\n- 検証: unit\n\nStatement.\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    // 判断の記録として扱われるので番号 A1 で照合 → 見つからないので source_invalid
    assert!(si.iter().any(|f| f["detail"].as_str().unwrap().contains("no-numbers.md#A1")),
        "file with decision sections should be treated as records: {:?}", si);
}

// @kotowari[TBL-012]
#[test]
fn tbl_012_file_without_decision_sections_matches_headings() {
    // 決定の節の見出しが無いファイルは見出しで照合
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // 決定の節の見出しを持たないファイル
    fs::write(
        tmp.path().join("docs/decision/brainstorm/notes.md"),
        "# Notes\n\n## Overview\n\nSome notes.\n",
    ).unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/notes.md#Overview\n- 検証: unit\n\nStatement.\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    // 見出しで照合 → Overview が見つかるので通る
    assert!(si.is_empty(), "file without decision sections should match by headings: {:?}", si);
}

// @kotowari[TBL-012]
#[test]
fn tbl_012_two_letter_prefix_is_not_a_decision_number() {
    // "AB1" は決定の番号でない（英大文字1文字に1桁以上の数字）
    assert!(!kotowari::sources::is_decision_number("AB1"),
        "AB1 should not be a decision number (two letters)");
    assert!(kotowari::sources::is_decision_number("A1"),
        "A1 should be a decision number");
}

// @kotowari[REQ-061]
#[test]
fn req_061_subheading_does_not_end_a_section() {
    // "### " の小見出しは節を終えない
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    // records.md に ### の小見出しの後に決定の番号を置く
    fs::write(
        tmp.path().join("docs/decision/brainstorm/records.md"),
        "# Records\n\n## Agreements\n\n- A1 First agreement\n\n### Subsection\n\n- A2 Second agreement\n",
    ).unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A2\n- 検証: unit\n\nStatement.\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    // ### は節を終えないので A2 は見つかる
    assert!(si.is_empty(), "### subheading should not end a section: {:?}", si);
}

// @kotowari[REQ-115]
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

// @kotowari[REQ-064]
#[test]
fn req_064_backtick_content_is_trimmed() {
    // "` IR `" は "IR" として照合される
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n| IR | 仕様 | docs/decision/brainstorm/records.md#A1 |\n",
    ).unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A1\n- 検証: unit\n\n` IR `は用語集にある。\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ut = findings_by_kind(&v, "unknown_term");
    assert!(!ut.iter().any(|f| f["detail"] == "IR" || f["detail"] == " IR "),
        "trimmed backtick content should match glossary term: {:?}", ut);
}

// @kotowari[REQ-064]
#[test]
fn req_064_empty_backticks_are_unknown_term() {
    // "``" は detail "``" の unknown_term
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n| IR | 仕様 | docs/decision/brainstorm/records.md#A1 |\n",
    ).unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A1\n- 検証: unit\n\nSee `` here.\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ut = findings_by_kind(&v, "unknown_term");
    assert!(ut.iter().any(|f| f["detail"] == "``"),
        "empty backticks should produce unknown_term with detail '``': {:?}", ut);
}

// @kotowari[REQ-116]
#[test]
fn req_116_odd_backticks_skip_terms_but_check_vague_words() {
    // 奇数バッククォートの行は unclosed_backtick、用語と ID の検査を飛ばし曖昧語は検査する
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n| IR | 仕様 | docs/decision/brainstorm/records.md#A1 |\n",
    ).unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A1\n- 検証: unit\n\n`奇数のバッククォート 適切に処理する。\n",
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

// @kotowari[REQ-116, TBL-008]
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
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A1\n- 検証: unit\n\n  `不完全な引用\n",
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

// @kotowari[REQ-067]
#[test]
fn req_067_overlapping_vague_words_longest_match_once() {
    // "など" と "などの" が両方あるとき、"などの" で1件
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\nvague_words:\n  - など\n  - などの\n",
    ).unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A1\n- 検証: unit\n\nなどの操作をする。\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let vw = findings_by_kind(&v, "vague_word");
    // "などの" で1件だけ
    assert_eq!(vw.len(), 1, "overlapping vague words should match longest once: {:?}", vw);
    assert_eq!(vw[0]["detail"], "などの", "should match 'などの' not 'など': {:?}", vw);
}

// @kotowari[TBL-014]
#[test]
fn tbl_014_md_followed_by_letter_is_not_a_reference() {
    // "a.mdX" は参照でない
    use std::collections::BTreeSet;
    let content = "# Title\n\nScope with a.mdX text.\n";
    let doc = kotowari::ir::parse_document("a.md", content);
    let ir_filenames: BTreeSet<String> = [doc.filename.clone()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari::terms::check_document_references(&[doc], "docs/ir", &ir_filenames, &mut findings);
    let md: Vec<_> = findings.iter().filter(|f| f.kind == "missing_document").collect();
    assert!(md.is_empty(), "a.mdX should not be a reference: {:?}", md);
}

// @kotowari[TBL-014]
#[test]
fn tbl_014_unclosed_quote_hides_the_rest_of_the_line() {
    // 奇数の二重引用符の後は参照を拾わない
    use std::collections::BTreeSet;
    let content = "# Title\n\nScope.\n\nSee \"unclosed quote nonexistent.md here.\n";
    let doc = kotowari::ir::parse_document("a.md", content);
    let ir_filenames: BTreeSet<String> = [doc.filename.clone()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari::terms::check_document_references(&[doc], "docs/ir", &ir_filenames, &mut findings);
    let md: Vec<_> = findings.iter().filter(|f| f.kind == "missing_document").collect();
    assert!(md.is_empty(), "unclosed quote should hide the rest of the line: {:?}", md);
}

// @kotowari[TBL-014]
#[test]
fn tbl_014_text_between_the_second_and_third_quote_is_still_scanned() {
    // 二重引用符が奇数（3つ）のとき、行末を隠すのは「最後の」引用符から先だけ。
    // 最初の引用符で打ち切ってはいけない（2つ目と3つ目の間は引用符の外）。
    use std::collections::BTreeSet;
    let content = "# Title\n\nScope.\n\nSee \"note\" and outside.md here \"trail\n";
    let doc = kotowari::ir::parse_document("a.md", content);
    let ir_filenames: BTreeSet<String> = [doc.filename.clone()].into_iter().collect();
    let mut findings = Vec::new();
    kotowari::terms::check_document_references(&[doc], "docs/ir", &ir_filenames, &mut findings);
    let md: Vec<_> = findings.iter().filter(|f| f.kind == "missing_document").collect();
    assert!(
        md.iter().any(|f| f.detail == "outside.md"),
        "text between the 2nd and 3rd quote is outside quotes and should still be scanned: {:?}",
        md
    );
}

// @kotowari[REQ-117]
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
    let doc = kotowari::ir::parse_document("CONTEXT.md", content);
    let terms: Vec<_> = doc.items.iter()
        .filter(|i| matches!(i, kotowari::ir::Item::GlossaryTerm { .. }))
        .collect();
    assert_eq!(terms.len(), 1, "second table should not be parsed as glossary: {:?}", terms);
    if let kotowari::ir::Item::GlossaryTerm { term, .. } = &terms[0] {
        assert_eq!(term, "テスト", "only first table terms should be parsed");
    }
}

// @kotowari[REQ-117]
#[test]
fn req_117_glossary_without_proper_table_is_invalid() {
    // 用語集にヘッダの列名が違う表しかない → glossary_invalid
    let content = "\
# 用語集

| Name | Meaning | Source |
|---|---|---|
| test | meaning | brainstorm/records.md#A1 |
";
    let doc = kotowari::ir::parse_document("CONTEXT.md", content);
    let config = kotowari::config::Config::default();
    let findings = kotowari::ir::check_documents(&[doc], &config);
    let gi: Vec<_> = findings.iter().filter(|f| f.kind == "glossary_invalid").collect();
    assert!(!gi.is_empty(), "glossary without proper table should produce glossary_invalid: {:?}", gi);
    assert!(gi[0].line.is_none(), "glossary_invalid line should be null");
    assert_eq!(gi[0].detail, "CONTEXT.md", "glossary_invalid detail should be filename");
}

// --- TBL-001: 非 UTF-8 の判断の記録または ADR で停止 ---

// @kotowari[REQ-006, TBL-001]
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
        tmp.path().join("docs/decision/brainstorm/bad.md"),
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

// @kotowari[TBL-012]
#[test]
fn tbl_012_records_file_headings_are_not_sources() {
    // 決定の節の見出しを持つファイルは判断の記録なので、"## " の見出しでは照合しない（A134）
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::write(
        tmp.path().join("docs/decision/brainstorm/notes.md"),
        "# Notes\n\n## Agreements\n\nJust text, no decisions.\n\n## Background\n\nSome background.\n",
    ).unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/notes.md#Background\n- 検証: unit\n\nStatement.\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    assert!(si.iter().any(|f| f["detail"].as_str().unwrap().contains("notes.md#Background")),
        "a records file (has a decision section) must be matched by decision numbers, not headings: {:?}", si);
}

// @kotowari[REQ-018, TBL-001]
#[test]
#[cfg(unix)]
fn req_018_broken_symlink_in_records_dir_stops() {
    use std::os::unix::fs::symlink;
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    symlink(tmp.path().join("nowhere.md"), tmp.path().join("docs/decision/brainstorm/broken.md")).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(output.status.code(), Some(2), "a broken symlink must stop: {:?}", output);
    assert!(output.stdout.is_empty());
}

// @kotowari[REQ-033, REQ-018]
#[test]
#[cfg(unix)]
fn req_033_file_symlink_in_records_dir_is_read() {
    use std::os::unix::fs::symlink;
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    fs::create_dir_all(tmp.path().join("elsewhere")).unwrap();
    fs::write(tmp.path().join("elsewhere/more.md"), "# More\n\n## Agreements\n\n- A7 linked decision\n").unwrap();
    symlink(tmp.path().join("elsewhere/more.md"), tmp.path().join("docs/decision/brainstorm/more.md")).unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/more.md#A7\n- 検証: unit\n\nStatement.\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let si = findings_by_kind(&v, "source_invalid");
    assert!(si.is_empty(), "a file symlink in the records dir must be read: {:?}", si);
}

// @kotowari[REQ-117]
#[test]
fn req_117_glossary_header_without_rows_is_valid() {
    // ヘッダと区切りの行があれば表は「ある」（A148）
    let content = "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n";
    let doc = kotowari::ir::parse_document("CONTEXT.md", content);
    let config = kotowari::config::Config::default();
    let findings = kotowari::ir::check_documents(&[doc], &config);
    assert!(!findings.iter().any(|f| f.kind == "glossary_invalid"), "header + separator with no rows must not be glossary_invalid: {:?}", findings);
}

// @kotowari[REQ-111]
#[test]
fn req_111_bom_in_config_records_adr_and_tests_is_skipped_end_to_end() {
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    let bom = "\u{feff}";
    fs::write(tmp.path().join(".kotowari/config.yaml"), format!("{bom}ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\n")).unwrap();
    fs::write(tmp.path().join("docs/decision/brainstorm/records.md"), format!("{bom}# Records\n\n## Agreements\n\n- A1 first\n")).unwrap();
    fs::write(tmp.path().join("docs/decision/adr/0001-test.md"), format!("{bom}# ADR 0001\n\n## 状況\n\nx\n")).unwrap();
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        format!("{bom}# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A1, docs/decision/adr/0001-test.md#状況\n- 検証: unit\n\nStatement.\n"),
    ).unwrap();
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(tmp.path().join("tests/t.rs"), format!("{bom}// @kotowari[REQ-001]\n#[test]\nfn t() {{}}\n")).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(output.status.code(), Some(0), "BOM in every file kind must be skipped: {:?}", output);
    let v = parse_json(&output);
    assert!(v["findings"].as_array().unwrap().is_empty(), "{:?}", v);
}

// @kotowari[REQ-058, REQ-110]
#[test]
fn req_058_records_place_dot_resolves_a_source_at_the_base_root() {
    // 置き場 "." は空に正規化される（REQ-110）。そのとき基準の直下の判断の記録の出典が解決しなければならない
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
