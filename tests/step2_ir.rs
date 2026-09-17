use kotowari::config::Config;
use kotowari::ir::{self, IrDocument, Item};
use kotowari::Finding;

fn default_config() -> Config {
    Config::default()
}

fn check(docs: &[IrDocument], config: &Config) -> Vec<Finding> {
    ir::check_documents(docs, config)
}

fn find_by_kind<'a>(findings: &'a [Finding], kind: &str) -> Vec<&'a Finding> {
    findings.iter().filter(|f| f.kind == kind).collect()
}

// @kotowari[REQ-033, REQ-037, TBL-005]
#[test]
fn req_033_subdirectories_are_read_at_any_depth() {
    let tmp = tempfile::tempdir().unwrap();
    for dir in ["docs/ir/sub/deep", "docs/ir/empty"] {
        std::fs::create_dir_all(tmp.path().join(dir)).unwrap();
    }
    for file in ["a.md", "sub/b.md", "sub/deep/c.md"] {
        std::fs::write(tmp.path().join("docs/ir").join(file), "# Title\n\nScope.\n").unwrap();
    }
    let (docs, findings) = ir::load_and_check(tmp.path(), &default_config()).unwrap();
    assert_eq!(docs.len(), 3);
    assert_eq!(docs.iter().map(|doc| doc.line_count).sum::<usize>(), 9);
    assert!(findings.is_empty(), "{findings:?}");
}

// --- REQ-034: 題名が無い ---

// @kotowari[REQ-034]
#[test]
fn req_034_missing_title() {
    let doc = ir::parse_document("a.md", "No title here.\n");
    let findings = check(&[doc], &default_config());
    let mt = find_by_kind(&findings, "missing_title");
    assert_eq!(mt.len(), 1);
    assert_eq!(mt[0].detail, "a.md");
    assert!(mt[0].line.is_none());
}

// --- REQ-035: 題名が複数 ---

// @kotowari[REQ-035]
#[test]
fn req_035_multiple_titles() {
    let doc = ir::parse_document("a.md", "# First\n\nScope.\n\n# Second\n");
    let findings = check(&[doc], &default_config());
    let mt = find_by_kind(&findings, "multiple_titles");
    assert_eq!(mt.len(), 1);
    assert_eq!(mt[0].detail, "Second");
}

// --- REQ-036: 範囲の行が無い ---

// @kotowari[REQ-036]
#[test]
fn req_036_missing_scope() {
    let doc = ir::parse_document("a.md", "# Title\n\n## 要求\n");
    let findings = check(&[doc], &default_config());
    let ms = find_by_kind(&findings, "missing_scope");
    assert_eq!(ms.len(), 1);
    assert_eq!(ms[0].detail, "a.md");
}

// @kotowari[REQ-036]
#[test]
fn req_036_glossary_and_flags_need_no_scope() {
    let glossary = ir::parse_document(
        "CONTEXT.md",
        "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n",
    );
    let flags = ir::parse_document("FLAGS.md", "# 問題の記録\n");
    let findings = check(&[glossary, flags], &default_config());
    let ms = find_by_kind(&findings, "missing_scope");
    assert!(ms.is_empty());
}

// --- REQ-037: 行の数え方 ---

// @kotowari[REQ-037, TBL-010]
#[test]
fn req_037_crlf_counts_as_one_line() {
    let doc = ir::parse_document("a.md", "# Title\r\n\r\nScope.\r\n");
    assert_eq!(doc.line_count, 3);
    // \n だけ
    let doc2 = ir::parse_document("b.md", "# Title\n\nScope.\n");
    assert_eq!(doc2.line_count, 3);
    // 改行なし
    let doc3 = ir::parse_document("c.md", "# Title");
    assert_eq!(doc3.line_count, 1);
}

// --- REQ-038: 行数の上限 ---

// @kotowari[REQ-038]
#[test]
fn req_038_too_many_lines_is_a_notice() {
    // 既定の limits.lines は 200
    let content = format!("# Title\n\nScope.\n\n{}", "line\n".repeat(200));
    let doc = ir::parse_document("a.md", &content);
    assert!(doc.line_count > 200);
    let findings = check(&[doc], &default_config());
    let tl = find_by_kind(&findings, "too_many_lines");
    assert_eq!(tl.len(), 1);
    assert_eq!(tl[0].severity, "notice");
    assert!(tl[0].line.is_none());
}

// --- REQ-039: 要求の数の上限 ---

// @kotowari[REQ-039]
#[test]
fn req_039_too_many_requirements_skips_glossary_and_flags() {
    // 既定の limits.requirements は 10。11個の要求を持つ文書
    let mut content = String::from("# Title\n\nScope.\n\n## 要求\n\n");
    for i in 1..=11 {
        content.push_str(&format!(
            "### REQ-{:03}: Req{i}\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\nStatement.\n\n",
            i
        ));
    }
    let doc = ir::parse_document("a.md", &content);
    let findings = check(&[doc], &default_config());
    let tr = find_by_kind(&findings, "too_many_requirements");
    assert_eq!(tr.len(), 1);
    assert_eq!(tr[0].severity, "notice");

    // 用語集は数えない
    let glossary = ir::parse_document("CONTEXT.md", "# 用語集\n");
    let findings2 = check(&[glossary], &default_config());
    let tr2 = find_by_kind(&findings2, "too_many_requirements");
    assert!(tr2.is_empty());
}

// @kotowari[REQ-039]
#[test]
fn req_039_unknown_heading_does_not_inflate_requirement_count() {
    // 10個の正しい要求 + 1個の認識できない見出し → too_many_requirements にならない
    let mut content = String::from("# Title\n\nScope.\n\n## 要求\n\n");
    for i in 1..=10 {
        content.push_str(&format!(
            "### REQ-{:03}: Req{i}\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\nStatement.\n\n",
            i
        ));
    }
    // EX- 接頭辞の見出し（要求ではない）
    content.push_str("### EX-001: Example\n\nSome text.\n");
    let doc = ir::parse_document("a.md", &content);
    let findings = check(&[doc], &default_config());
    let tr = find_by_kind(&findings, "too_many_requirements");
    assert!(
        tr.is_empty(),
        "unknown heading should not inflate requirement count: {:?}",
        tr
    );
}

// --- REQ-040: コードブロックの中はスキップ ---

// @kotowari[REQ-040]
#[test]
fn req_040_code_blocks_are_skipped_except_gherkin() {
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\nStatement.\n\n```\n### BAD-001: Should not be parsed\n```\n\n## 具体例\n\n```gherkin\n@id=EX-001 @about=REQ-001 @source=brainstorm/records.md#A1\nScenario: Test\n  Given something\n```\n";
    let doc = ir::parse_document("a.md", content);

    // 通常のコードブロック内の ### は項目として読まれない
    let items_with_bad = doc.items.iter().any(|i| match i {
        Item::Requirement { id, .. } => id == "BAD-001",
        _ => false,
    });
    assert!(!items_with_bad, "code block content should be skipped");

    // gherkin ブロック内のシナリオは読まれる
    let has_scenario = doc.items.iter().any(|i| matches!(i, Item::Scenario { .. }));
    assert!(has_scenario, "gherkin scenarios should be parsed");
}

// --- REQ-042: すべての項目の形 ---

// @kotowari[REQ-042, TBL-011]
#[test]
fn req_042_reads_every_item_kind_in_the_table() {
    let content = r#"# Title

Scope.

## 要求

### REQ-001: Test Requirement

- 種類: ubiquitous
- 出典: brainstorm/records.md#A1
- 検証: unit

Statement text.

### REQ-002: Algorithm

- 種類: algorithm
- 出典: brainstorm/records.md#A1
- 検証: unit
- 定義: TBL-001

## 決定表

### TBL-001: Test Table

- 出典: brainstorm/records.md#A1

| Col1 | Col2 |
|---|---|
| a | b |

## 性質

### PROP-001: Test Property

- 出典: brainstorm/records.md#A1

Property statement.

## 具体例

```gherkin
@id=EX-001 @about=REQ-001 @source=brainstorm/records.md#A1
Scenario: Test scenario
  Given something
  When something happens
  Then result
```
"#;
    let doc = ir::parse_document("a.md", content);

    let mut has_req = false;
    let mut has_tbl = false;
    let mut has_prop = false;
    let mut has_scenario = false;

    for item in &doc.items {
        match item {
            Item::Requirement { id, .. } if id == "REQ-001" => has_req = true,
            Item::DecisionTable { id, .. } if id == "TBL-001" => has_tbl = true,
            Item::Property { id, .. } if id == "PROP-001" => has_prop = true,
            Item::Scenario { id: Some(id), .. } if id == "EX-001" => has_scenario = true,
            _ => {}
        }
    }

    assert!(has_req, "should parse requirements");
    assert!(has_tbl, "should parse decision tables");
    assert!(has_prop, "should parse properties");
    assert!(has_scenario, "should parse scenarios");
}

// --- REQ-043: 形に合わない見出し ---

// @kotowari[REQ-043]
#[test]
fn req_043_unknown_heading() {
    let content = "# Title\n\nScope.\n\n## 要求\n\n### Bad Heading\n\nSome text.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let uh = find_by_kind(&findings, "unknown_heading");
    assert_eq!(uh.len(), 1);
    // A150/TBL-008: detail は読んだ見出しの行そのまま（"### " を含む）
    assert_eq!(uh[0].detail, "### Bad Heading");
}

// @kotowari[REQ-043, TBL-008]
#[test]
fn req_043_unknown_heading_detail_is_full_heading_text() {
    // コロン付きの認識できない見出し → detail は読んだ見出しの行そのまま（A150）
    let content = "# Title\n\nScope.\n\n## 要求\n\n### EX-001: Example\n\nSome text.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let uh = find_by_kind(&findings, "unknown_heading");
    assert_eq!(uh.len(), 1);
    assert_eq!(
        uh[0].detail, "### EX-001: Example",
        "detail should be the raw heading line, including the leading '### '"
    );
}

// @kotowari[REQ-043, TBL-008]
#[test]
fn req_043_unknown_heading_invalid_id_detail_is_full_heading_text() {
    // 認識できる prefix だが ID 形式が不正（3桁でない） → detail は読んだ見出しの行そのまま（A150）
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-1: Invalid\n\nSome text.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let uh = find_by_kind(&findings, "unknown_heading");
    assert_eq!(uh.len(), 1);
    assert_eq!(
        uh[0].detail, "### REQ-1: Invalid",
        "detail should be the raw heading line, not just the ID"
    );
}

// @kotowari[REQ-043, TBL-008]
#[test]
fn req_043_heading_without_colon_is_unknown() {
    // "### REQ-001" はコロンがないので "### ID: 名前" の形ではなく、unknown_heading になる
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\nStatement.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let uh = find_by_kind(&findings, "unknown_heading");
    assert_eq!(uh.len(), 1, "should produce exactly one unknown_heading");
    // A150/TBL-008: detail は読んだ見出しの行そのまま（"### " を含む）
    assert_eq!(uh[0].detail, "### REQ-001");
}

// --- REQ-044: 知らない行 ---

// @kotowari[REQ-044]
#[test]
fn req_044_unknown_field() {
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n- 優先度: 高\n\nStatement.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let uf = find_by_kind(&findings, "unknown_field");
    assert_eq!(uf.len(), 1);
    assert_eq!(uf[0].detail, "- 優先度: 高");
}

// @kotowari[REQ-044, TBL-008]
#[test]
fn req_044_unknown_field_without_colon_has_line_text_as_detail() {
    // コロンのない "- テキスト" 行 → detail は行の文字
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n- ただのテキスト\n\nStatement.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let uf = find_by_kind(&findings, "unknown_field");
    assert!(
        uf.iter().any(|f| f.detail == "- ただのテキスト"),
        "detail of unknown_field without colon should be the line text: {:?}",
        uf
    );
}

// --- REQ-045: 同じ行の重複 ---

// @kotowari[REQ-045]
#[test]
fn req_045_duplicate_field() {
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n- 種類: prohibition\n\nStatement.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let df = find_by_kind(&findings, "duplicate_field");
    assert_eq!(df.len(), 1);
    assert_eq!(df[0].detail, "種類");
}

// --- REQ-046: 順不同、空行、コンマ ---

// @kotowari[REQ-046]
#[test]
fn req_046_fields_in_any_order_with_blank_lines_and_commas() {
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 検証: unit\n\n- 出典: brainstorm/records.md#A1, brainstorm/records.md#A2\n\n- 種類: algorithm\n- 定義: TBL-001, PROP-001\n\n## 決定表\n\n### TBL-001: T\n\n- 出典: brainstorm/records.md#A1\n\n| A | B |\n|---|---|\n| 1 | 2 |\n\n## 性質\n\n### PROP-001: P\n\n- 出典: brainstorm/records.md#A1\n\nProp statement.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    // 既知のフィールドだけなので unknown_field、duplicate_field は出ない
    let uf = find_by_kind(&findings, "unknown_field");
    let df = find_by_kind(&findings, "duplicate_field");
    assert!(uf.is_empty(), "no unknown fields: {:?}", uf);
    assert!(df.is_empty(), "no duplicate fields: {:?}", df);
}

// --- REQ-047: 文が無い ---

// @kotowari[REQ-047]
#[test]
fn req_047_missing_statement() {
    // algorithm 以外の要求に文がない
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ms = find_by_kind(&findings, "missing_statement");
    assert_eq!(ms.len(), 1);
    assert_eq!(ms[0].detail, "REQ-001");

    // algorithm は文がなくても OK
    let content2 = "# Title\n\nScope.\n\n## 要求\n\n### REQ-002: Algo\n\n- 種類: algorithm\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n- 定義: TBL-001\n\n## 決定表\n\n### TBL-001: T\n\n- 出典: brainstorm/records.md#A1\n\n| A | B |\n|---|---|\n| 1 | 2 |\n";
    let doc2 = ir::parse_document("a.md", content2);
    let findings2 = check(&[doc2], &default_config());
    let ms2 = find_by_kind(&findings2, "missing_statement");
    assert!(ms2.is_empty());
}

// --- REQ-048: 検証の行が無い ---

// @kotowari[REQ-048]
#[test]
fn req_048_verification_missing() {
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n\nStatement.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let vm = find_by_kind(&findings, "verification_missing");
    assert_eq!(vm.len(), 1);
    assert_eq!(vm[0].detail, "REQ-001");
}

// --- REQ-049: 検証の値の誤り ---

// @kotowari[REQ-049]
#[test]
fn req_049_verification_invalid() {
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: manual\n\nStatement.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let vi = find_by_kind(&findings, "verification_invalid");
    assert_eq!(vi.len(), 1);
    assert_eq!(vi[0].detail, "manual");
}

// --- REQ-050: 種類の値の誤り ---

// @kotowari[REQ-050]
#[test]
fn req_050_unknown_kind_of_requirement_and_flag() {
    // 要求の種類
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: functional\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\nStatement.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let uk = find_by_kind(&findings, "unknown_kind");
    assert_eq!(uk.len(), 1);
    assert_eq!(uk[0].detail, "functional");

    // 問題の記録の種類
    let flag_content = "# 問題の記録\n\n### FLAG-001: Issue\n\n- 種類: error\n- 関係: REQ-001\n- 出典: brainstorm/records.md#A1\n\nBody.\n";
    let flag_doc = ir::parse_document("FLAGS.md", flag_content);
    // REQ-001 が定義されている文書も必要
    let req_content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\nStatement.\n";
    let req_doc = ir::parse_document("a.md", req_content);
    let findings2 = check(&[flag_doc, req_doc], &default_config());
    let uk2 = find_by_kind(&findings2, "unknown_kind");
    assert_eq!(uk2.len(), 1);
    assert_eq!(uk2[0].detail, "error");
}

// --- REQ-051: 定義の無い algorithm ---

// @kotowari[REQ-051]
#[test]
fn req_051_algorithm_without_definition() {
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Algo\n\n- 種類: algorithm\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let awd = find_by_kind(&findings, "algorithm_without_definition");
    assert_eq!(awd.len(), 1);
    assert_eq!(awd[0].detail, "REQ-001");
}

// @kotowari[REQ-051]
#[test]
fn req_051_algorithm_definition_must_point_to_tbl_or_prop() {
    // 定義が REQ-001 を指す（TBL/PROP ではない） → algorithm_without_definition が出る
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Algo\n\n- 種類: algorithm\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n- 定義: REQ-002\n\n### REQ-002: Other\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\nStatement.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let awd = find_by_kind(&findings, "algorithm_without_definition");
    assert_eq!(
        awd.len(),
        1,
        "definition pointing to REQ should still trigger algorithm_without_definition: {:?}",
        awd
    );
    assert_eq!(awd[0].detail, "REQ-001");
}

// --- REQ-098: 必須の行が無い ---

// @kotowari[REQ-098]
#[test]
fn req_098_missing_field() {
    // 決定表に出典がない → missing_source with detail = ID
    let content = "# Title\n\nScope.\n\n## 決定表\n\n### TBL-001: T\n\n| A |\n|---|\n| 1 |\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ms = find_by_kind(&findings, "missing_source");
    assert!(ms.iter().any(|f| f.detail == "TBL-001"), "missing source for TBL-001");
    let mf = find_by_kind(&findings, "missing_field");
    assert!(!mf.iter().any(|f| f.detail == "出典"), "missing_field 出典 should not appear");
}

// --- REQ-099: 決定表に表がない ---

// @kotowari[REQ-099]
#[test]
fn req_099_missing_table() {
    let content = "# Title\n\nScope.\n\n## 決定表\n\n### TBL-001: T\n\n- 出典: brainstorm/records.md#A1\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let mt = find_by_kind(&findings, "missing_table");
    assert_eq!(mt.len(), 1);
    assert_eq!(mt[0].detail, "TBL-001");
}

// --- REQ-100: gherkin の外の Scenario は無視 ---

// @kotowari[REQ-100]
#[test]
fn req_100_scenario_outside_gherkin_is_ignored() {
    let content = "# Title\n\nScope.\n\nScenario: This should be ignored\n";
    let doc = ir::parse_document("a.md", content);
    let has_scenario = doc.items.iter().any(|i| matches!(i, Item::Scenario { .. }));
    assert!(!has_scenario, "Scenario outside gherkin should be ignored");
}

// --- REQ-042, REQ-053: タグなし連続シナリオ ---

// @kotowari[REQ-042, REQ-053]
#[test]
fn req_053_consecutive_scenarios_without_tags_each_get_missing_tag() {
    // タグ行なしで Scenario: が2つ連続 → 各シナリオに missing_tag が出る
    let content = "# Title\n\nScope.\n\n## 具体例\n\n```gherkin\nScenario: First\n  Given step1\nScenario: Second\n  Given step2\n```\n";
    let doc = ir::parse_document("a.md", content);
    let scenarios: Vec<_> = doc.items.iter().filter(|i| matches!(i, Item::Scenario { .. })).collect();
    assert_eq!(scenarios.len(), 2, "should parse 2 scenarios: {:?}", scenarios);
    let findings = check(&[doc], &default_config());
    let mt = find_by_kind(&findings, "missing_tag");
    // 各シナリオに @id と @about の missing_tag が出る → 4件
    let id_missing: Vec<_> = mt.iter().filter(|f| f.detail == "@id").collect();
    assert_eq!(id_missing.len(), 2, "@id missing should be 2: {:?}", mt);
    let about_missing: Vec<_> = mt.iter().filter(|f| f.detail == "@about").collect();
    assert_eq!(about_missing.len(), 2, "@about missing should be 2: {:?}", mt);
}

// --- REQ-052: 知らないタグ ---

// @kotowari[REQ-052]
#[test]
fn req_052_unknown_tag() {
    let content = "# Title\n\nScope.\n\n## 具体例\n\n```gherkin\n@id=EX-001 @about=REQ-001 @source=brainstorm/records.md#A1 @requirement=REQ-001\nScenario: Test\n  Given something\n```\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\nStatement.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ut = find_by_kind(&findings, "unknown_tag");
    assert_eq!(ut.len(), 1);
    assert_eq!(ut[0].detail, "@requirement");
}

// @kotowari[REQ-052]
#[test]
fn req_052_bare_tag_without_equals_is_unknown() {
    // @wip のように = を持たない裸のタグも unknown_tag になる
    let content = "# Title\n\nScope.\n\n## 具体例\n\n```gherkin\n@id=EX-001 @about=REQ-001 @source=brainstorm/records.md#A1 @wip\nScenario: Test\n  Given something\n```\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\nStatement.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ut = find_by_kind(&findings, "unknown_tag");
    assert!(
        ut.iter().any(|f| f.detail == "@wip"),
        "bare tag @wip should be unknown_tag: {:?}",
        ut
    );
}

// --- REQ-053: 無いタグ ---

// @kotowari[REQ-053]
#[test]
fn req_053_missing_tag() {
    let content = "# Title\n\nScope.\n\n## 具体例\n\n```gherkin\n@source=brainstorm/records.md#A1\nScenario: Test without id and about\n  Given something\n```\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let mt = find_by_kind(&findings, "missing_tag");
    assert!(mt.iter().any(|f| f.detail == "@id"));
    assert!(mt.iter().any(|f| f.detail == "@about"));
}

// @kotowari[REQ-053, REQ-059]
#[test]
fn req_053_tag_with_empty_value_is_treated_as_missing() {
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\nStatement.\n\n## 具体例\n\n```gherkin\n@id= @about=REQ-001 @source=\nScenario: Empty tag values\n  Given something\n```\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let mt = find_by_kind(&findings, "missing_tag");
    assert!(mt.iter().any(|f| f.detail == "@id"), "empty @id= should count as missing: {:?}", mt);
    assert!(!mt.iter().any(|f| f.detail == "@about"), "@about has a value: {:?}", mt);
    let ms = find_by_kind(&findings, "missing_source");
    assert!(
        ms.iter().any(|f| f.detail == "Scenario: Empty tag values"),
        "empty @source= should be missing_source with the Scenario line as detail: {:?}",
        ms
    );
}

// --- REQ-054: 参照切れ ---

// @kotowari[REQ-054]
#[test]
fn req_054_unresolved_reference_in_definition_about_relation_and_sentence() {
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: algorithm\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n- 定義: TBL-999\n\n## 具体例\n\n```gherkin\n@id=EX-001 @about=REQ-999 @source=brainstorm/records.md#A1\nScenario: Ref test\n  Given something\n```\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ur = find_by_kind(&findings, "unresolved_reference");
    let details: Vec<&str> = ur.iter().map(|f| f.detail.as_str()).collect();
    assert!(details.contains(&"TBL-999"), "should find unresolved definition: {:?}", details);
    assert!(details.contains(&"REQ-999"), "should find unresolved about: {:?}", details);
}

// --- REQ-032: ID の重複 ---

// @kotowari[REQ-032]
#[test]
fn req_032_duplicate_id_on_each_later_place_with_its_line() {
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: First\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\nStatement 1.\n\n### REQ-001: Second\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\nStatement 2.\n\n### REQ-001: Third\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\nStatement 3.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let di = find_by_kind(&findings, "duplicate_id");
    // 3か所にある → 2件（2つ目と3つ目）
    assert_eq!(di.len(), 2, "should report 2 duplicates for 3 occurrences: {:?}", di);
    assert!(di.iter().all(|f| f.detail == "REQ-001"));
    // 各指摘に行番号がある
    assert!(di.iter().all(|f| f.line.is_some()));
}

// --- REQ-044, TBL-011: 性質の知らない行 ---

// @kotowari[REQ-044, TBL-011]
#[test]
fn req_044_property_definition_field_is_unknown() {
    // TBL-011 によると性質が持つ行は「- 出典:」だけ。
    // 「- 定義:」は知らない行として unknown_field になる。
    let content = "# Title\n\nScope.\n\n## 性質\n\n### PROP-001: P\n\n- 出典: brainstorm/records.md#A1\n- 定義: TBL-001\n\nProperty statement.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let uf = find_by_kind(&findings, "unknown_field");
    assert!(
        uf.iter().any(|f| f.detail == "- 定義: TBL-001"),
        "PROP の「- 定義:」は unknown_field になるはず: {:?}",
        uf
    );
}

// @kotowari[REQ-059]
#[test]
fn req_059_scenario_without_id_missing_source_detail_is_scenario_text() {
    let content = "# Title\n\nScope.\n\n## 具体例\n\n```gherkin\n@about=REQ-001\nScenario: No id scenario\n  Given something\n```\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ms = find_by_kind(&findings, "missing_source");
    assert!(
        ms.iter().any(|f| f.detail == "Scenario: No id scenario"),
        "@id の無いシナリオの missing_source の detail は Scenario: の行の文字のはず: {:?}",
        ms
    );
}

// @kotowari[REQ-044]
#[test]
fn req_044_prop_unknown_field_does_not_produce_unresolved_reference() {
    let content = "# Title\n\nScope.\n\n## 性質\n\n### PROP-001: P\n\n- 出典: brainstorm/records.md#A1\n- 定義: TBL-999\n\nProperty statement.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let uf = find_by_kind(&findings, "unknown_field");
    assert!(
        uf.iter().any(|f| f.detail == "- 定義: TBL-999"),
        "PROP の「- 定義: TBL-999」は unknown_field になるはず: {:?}",
        uf
    );
    let ur = find_by_kind(&findings, "unresolved_reference");
    assert!(
        ur.is_empty(),
        "PROP の知らない行の中身を参照として読んではいけない: {:?}",
        ur
    );
}

// --- REQ-047, REQ-048, REQ-049, REQ-050, REQ-051, REQ-098: フィールド検査の組み合わせ ---

// @kotowari[REQ-047, REQ-048, REQ-049, REQ-050, REQ-051, REQ-098]
#[test]
fn req_098_required_lines_are_told_apart_from_empty_values() {
    // (1) すべてのフィールドが揃って値も正しい要求 → 関連する指摘が出ない
    let valid = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Valid\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\nStatement.\n";
    let doc = ir::parse_document("a.md", valid);
    let f = check(&[doc], &default_config());
    assert!(find_by_kind(&f, "missing_field").is_empty(), "valid req should have no missing_field: {:?}", f);
    assert!(find_by_kind(&f, "missing_source").is_empty(), "valid req should have no missing_source: {:?}", f);
    assert!(find_by_kind(&f, "verification_missing").is_empty(), "valid req should have no verification_missing: {:?}", f);
    assert!(find_by_kind(&f, "verification_invalid").is_empty(), "valid req should have no verification_invalid: {:?}", f);
    assert!(find_by_kind(&f, "unknown_kind").is_empty(), "valid req should have no unknown_kind: {:?}", f);
    assert!(find_by_kind(&f, "missing_statement").is_empty(), "valid req should have no missing_statement: {:?}", f);
    assert!(find_by_kind(&f, "algorithm_without_definition").is_empty(), "valid req should have no algorithm_without_definition: {:?}", f);

    // (2) 種類の行がない → missing_field "種類"
    let no_kind = "# Title\n\nScope.\n\n## 要求\n\n### REQ-002: NoKind\n\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\nStatement.\n";
    let doc2 = ir::parse_document("a.md", no_kind);
    let f2 = check(&[doc2], &default_config());
    let mf2 = find_by_kind(&f2, "missing_field");
    assert!(mf2.iter().any(|x| x.detail == "種類"), "should report missing_field 種類: {:?}", mf2);

    // (3) 出典の行がない → missing_source with detail = ID、missing_field "出典" は出ない
    let no_source = "# Title\n\nScope.\n\n## 要求\n\n### REQ-003: NoSource\n\n- 種類: ubiquitous\n- 検証: unit\n\nStatement.\n";
    let doc3 = ir::parse_document("a.md", no_source);
    let f3 = check(&[doc3], &default_config());
    let ms3 = find_by_kind(&f3, "missing_source");
    assert!(ms3.iter().any(|x| x.detail == "REQ-003"), "should report missing_source with ID: {:?}", ms3);
    let mf3 = find_by_kind(&f3, "missing_field");
    assert!(!mf3.iter().any(|x| x.detail == "出典"), "missing_field 出典 should not appear when 出典 line is absent: {:?}", mf3);

    // (4) 出典の行はあるが値が空 → missing_source が出て、missing_field "出典" は出ない
    let empty_source = "# Title\n\nScope.\n\n## 要求\n\n### REQ-004: EmptySource\n\n- 種類: ubiquitous\n- 出典:\n- 検証: unit\n\nStatement.\n";
    let doc4 = ir::parse_document("a.md", empty_source);
    let f4 = check(&[doc4], &default_config());
    let ms4 = find_by_kind(&f4, "missing_source");
    assert!(ms4.iter().any(|x| x.detail == "REQ-004"), "should report missing_source: {:?}", ms4);
    let mf4 = find_by_kind(&f4, "missing_field");
    assert!(!mf4.iter().any(|x| x.detail == "出典"), "missing_field 出典 should not appear when line exists: {:?}", mf4);

    // (5) 決定表の出典の行はあるが値が空 → missing_source
    let tbl_empty_source = "# Title\n\nScope.\n\n## 決定表\n\n### TBL-001: T\n\n- 出典:\n\n| A |\n|---|\n| 1 |\n";
    let doc5 = ir::parse_document("a.md", tbl_empty_source);
    let f5 = check(&[doc5], &default_config());
    let ms5 = find_by_kind(&f5, "missing_source");
    assert!(ms5.iter().any(|x| x.detail == "TBL-001"), "TBL should report missing_source on empty value: {:?}", ms5);
    let mf5 = find_by_kind(&f5, "missing_field");
    assert!(!mf5.iter().any(|x| x.detail == "出典"), "TBL missing_field 出典 should not appear: {:?}", mf5);

    // (6) 性質の出典の行はあるが値が空 → missing_source
    let prop_empty_source = "# Title\n\nScope.\n\n## 性質\n\n### PROP-001: P\n\n- 出典:\n\nProp statement.\n";
    let doc6 = ir::parse_document("a.md", prop_empty_source);
    let f6 = check(&[doc6], &default_config());
    let ms6 = find_by_kind(&f6, "missing_source");
    assert!(ms6.iter().any(|x| x.detail == "PROP-001"), "PROP should report missing_source on empty value: {:?}", ms6);
    let mf6 = find_by_kind(&f6, "missing_field");
    assert!(!mf6.iter().any(|x| x.detail == "出典"), "PROP missing_field 出典 should not appear: {:?}", mf6);
}

// @kotowari[REQ-053]
#[test]
fn gherkin_tags_cleared_after_block_without_scenario() {
    let content = "\
# Title

Scope.

## Examples

```gherkin
@id=EX-001 @about=REQ-001
```

```gherkin
Scenario: bare scenario
  Given something
```
";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let mt = find_by_kind(&findings, "missing_tag");
    assert!(
        mt.iter().any(|f| f.detail == "@id"),
        "second scenario should have missing_tag @id: {:?}",
        mt
    );
    assert!(
        mt.iter().any(|f| f.detail == "@about"),
        "second scenario should have missing_tag @about: {:?}",
        mt
    );
}

// --- split_lines: CRLF と空入力 ---

// @kotowari[REQ-037, TBL-010]
#[test]
fn req_037_crlf_title_and_requirement_line_numbers() {
    let content = "# Title\r\n\r\nScope.\r\n\r\n## 要求\r\n\r\n### REQ-001: Test\r\n\r\n- 種類: ubiquitous\r\n- 出典: brainstorm/records.md#A1\r\n- 検証: unit\r\n\r\nStatement.\r\n";
    let doc = ir::parse_document("a.md", content);
    assert_eq!(doc.title, Some((1, "Title".to_string())));
    let req = doc.items.iter().find(|i| matches!(i, Item::Requirement { id, .. } if id == "REQ-001"));
    assert!(req.is_some(), "should parse REQ-001");
    assert_eq!(req.unwrap().item_line(), 7, "REQ-001 should be on line 7");
    let scope_text: Vec<&str> = doc.scope_lines.iter().map(|(_, s)| s.as_str()).collect();
    assert!(scope_text.contains(&"Scope."), "scope should contain 'Scope.'");
    assert!(
        !scope_text.iter().any(|s| s.contains('\r')),
        "scope text should not contain \\r"
    );
}

// @kotowari[REQ-037, TBL-010]
#[test]
fn req_037_empty_content_has_zero_lines() {
    let doc = ir::parse_document("a.md", "");
    assert_eq!(doc.line_count, 0, "empty content should have 0 lines");
}

// --- gherkin ステップ認識 ---

// @kotowari[REQ-042, TBL-011]
#[test]
fn req_042_gherkin_all_step_keywords_recognized() {
    let content = "\
# Title

Scope.

## 具体例

```gherkin
@id=EX-001 @about=REQ-001 @source=brainstorm/records.md#A1
Scenario: All keywords
  Given a precondition
  When an action occurs
  Then the result is verified
  And an additional condition
  But not this condition
```
";
    let doc = ir::parse_document("a.md", content);
    let scenario = doc.items.iter().find(|i| matches!(i, Item::Scenario { id: Some(id), .. } if id == "EX-001"));
    assert!(scenario.is_some(), "should parse EX-001 scenario");
    if let Item::Scenario { steps, .. } = scenario.unwrap() {
        assert_eq!(steps.len(), 5, "should have 5 steps (Given, When, Then, And, But): {:?}", steps);
    }
}

// --- 用語集テーブルの解析 ---

// @kotowari[REQ-042, TBL-011]
#[test]
fn req_042_glossary_table_parses_terms() {
    let content = "\
# 用語集

| 用語 | 意味 | 出典 |
|---|---|---|
| テスト | テストの意味 | brainstorm/records.md#A1 |
| 検証 | 検証の意味 | brainstorm/records.md#A2 |
";
    let doc = ir::parse_document("CONTEXT.md", content);
    let terms: Vec<_> = doc.items.iter().filter(|i| matches!(i, Item::GlossaryTerm { .. })).collect();
    assert_eq!(terms.len(), 2, "should parse 2 glossary terms: {:?}", terms);
    if let Item::GlossaryTerm { term, line, .. } = &terms[0] {
        assert_eq!(term, "テスト");
        assert_eq!(*line, 5, "first term should be on line 5");
    }
    if let Item::GlossaryTerm { term, line, .. } = &terms[1] {
        assert_eq!(term, "検証");
        assert_eq!(*line, 6, "second term should be on line 6");
    }
}

// --- FLAG の関係と出典のフィールド読み取り ---

// @kotowari[REQ-042, TBL-011]
#[test]
fn req_042_flag_relation_and_source_fields_read() {
    let content = "\
# 問題の記録

### FLAG-001: Issue

- 種類: gap
- 関係: REQ-999
- 出典: brainstorm/records.md#A1

Body text.
";
    let doc = ir::parse_document("FLAGS.md", content);
    let flag = doc.items.iter().find(|i| matches!(i, Item::FlagEntry { id, .. } if id == "FLAG-001"));
    assert!(flag.is_some(), "should parse FLAG-001");
    if let Item::FlagEntry { relations, sources, .. } = flag.unwrap() {
        assert_eq!(relations, &["REQ-999"], "relations should contain REQ-999");
        assert_eq!(sources, &["brainstorm/records.md#A1"], "sources should be read");
    }
}

// @kotowari[REQ-054]
#[test]
fn req_054_flag_relation_to_unknown_id_produces_unresolved_reference() {
    let content = "\
# 問題の記録

### FLAG-001: Issue

- 種類: gap
- 関係: REQ-999
- 出典: brainstorm/records.md#A1

Body text.
";
    let doc = ir::parse_document("FLAGS.md", content);
    let findings = check(&[doc], &default_config());
    let ur = find_by_kind(&findings, "unresolved_reference");
    assert!(
        ur.iter().any(|f| f.detail == "REQ-999"),
        "FLAG relation to unknown ID should produce unresolved_reference: {:?}",
        ur
    );
}

// @kotowari[REQ-098]
#[test]
fn req_098_flag_without_relation_line_produces_missing_field() {
    let content = "\
# 問題の記録

### FLAG-001: Issue

- 種類: gap
- 出典: brainstorm/records.md#A1

Body text.
";
    let doc = ir::parse_document("FLAGS.md", content);
    let findings = check(&[doc], &default_config());
    let mf = find_by_kind(&findings, "missing_field");
    assert!(
        mf.iter().any(|f| f.detail == "関係"),
        "FLAG without relation line should produce missing_field 関係: {:?}",
        mf
    );
}

// --- build_scenario の @source ---

// @kotowari[REQ-042, TBL-011]
#[test]
fn req_042_scenario_source_tag_parsed_into_sources() {
    let content = "\
# Title

Scope.

## 具体例

```gherkin
@id=EX-001 @about=REQ-001 @source=brainstorm/records.md#A1
Scenario: With source
  Given something
```
";
    let doc = ir::parse_document("a.md", content);
    let scenario = doc.items.iter().find(|i| matches!(i, Item::Scenario { id: Some(id), .. } if id == "EX-001"));
    assert!(scenario.is_some(), "should parse EX-001");
    if let Item::Scenario { sources, .. } = scenario.unwrap() {
        assert_eq!(
            sources,
            &["brainstorm/records.md#A1"],
            "@source value should be in sources"
        );
    }
}

// --- check_documents の行数境界値 ---

// @kotowari[REQ-038]
#[test]
fn req_038_exactly_at_limit_no_notice_one_over_notices() {
    let cfg = default_config();
    let limit = cfg.limits.lines.get() as usize;

    let filler_lines = limit - 3;
    let content = format!("# Title\n\nScope.\n{}", "x\n".repeat(filler_lines));
    let doc = ir::parse_document("a.md", &content);
    assert_eq!(doc.line_count, limit, "should be exactly at limit");
    let findings = check(&[doc], &cfg);
    let tl = find_by_kind(&findings, "too_many_lines");
    assert!(tl.is_empty(), "exactly at limit should not produce too_many_lines: {:?}", tl);

    let over_content = format!("# Title\n\nScope.\n{}", "x\n".repeat(filler_lines + 1));
    let over_doc = ir::parse_document("b.md", &over_content);
    assert_eq!(over_doc.line_count, limit + 1, "should be one over limit");
    let over_findings = check(&[over_doc], &cfg);
    let over_tl = find_by_kind(&over_findings, "too_many_lines");
    assert_eq!(over_tl.len(), 1, "one over limit should produce too_many_lines");
}

// --- check_backtick_ids ---

// @kotowari[REQ-054]
#[test]
fn req_054_backtick_id_known_no_finding_unknown_produces_unresolved() {
    let content = "\
# Title

Scope.

## 要求

### REQ-001: Test

- 種類: ubiquitous
- 出典: brainstorm/records.md#A1
- 検証: unit

The `REQ-001` is known but `TBL-999` is not.
";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ur = find_by_kind(&findings, "unresolved_reference");
    assert!(
        ur.iter().any(|f| f.detail == "TBL-999"),
        "unknown backtick ID should produce unresolved_reference: {:?}",
        ur
    );
    assert!(
        !ur.iter().any(|f| f.detail == "REQ-001"),
        "known backtick ID should not produce unresolved_reference: {:?}",
        ur
    );
}

// @kotowari[REQ-054]
#[test]
fn req_054_backtick_id_at_line_start_detected() {
    let content = "\
# Title

Scope.

## 要求

### REQ-001: Test

- 種類: ubiquitous
- 出典: brainstorm/records.md#A1
- 検証: unit

`TBL-999` at the start of the line.
";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ur = find_by_kind(&findings, "unresolved_reference");
    assert!(
        ur.iter().any(|f| f.detail == "TBL-999"),
        "backtick ID at line start should be detected: {:?}",
        ur
    );
}

// @kotowari[REQ-054]
#[test]
fn req_054_two_backtick_ids_on_one_line_both_reported() {
    let content = "\
# Title

Scope.

## 要求

### REQ-001: Test

- 種類: ubiquitous
- 出典: brainstorm/records.md#A1
- 検証: unit

See `TBL-998` and `TBL-999` here.
";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ur = find_by_kind(&findings, "unresolved_reference");
    assert!(
        ur.iter().any(|f| f.detail == "TBL-998"),
        "first backtick ID should be reported: {:?}",
        ur
    );
    assert!(
        ur.iter().any(|f| f.detail == "TBL-999"),
        "second backtick ID should be reported: {:?}",
        ur
    );
}

// @kotowari[REQ-054]
#[test]
fn req_054_backtick_non_id_not_reported_as_unresolved() {
    let content = "\
# Title

Scope.

## 要求

### REQ-001: Test

- 種類: ubiquitous
- 出典: brainstorm/records.md#A1
- 検証: unit

The `foo` word is not an ID.
";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ur = find_by_kind(&findings, "unresolved_reference");
    assert!(
        !ur.iter().any(|f| f.detail == "foo"),
        "non-ID backtick content should not produce unresolved_reference: {:?}",
        ur
    );
}

// --- check_references: Property の文中のバッククォート ID ---

// @kotowari[REQ-054]
#[test]
fn req_054_property_statement_backtick_id_produces_unresolved() {
    let content = "\
# Title

Scope.

## 性質

### PROP-001: P

- 出典: brainstorm/records.md#A1

This property references `TBL-999` which does not exist.
";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ur = find_by_kind(&findings, "unresolved_reference");
    assert!(
        ur.iter().any(|f| f.detail == "TBL-999"),
        "Property statement backtick ID should produce unresolved_reference: {:?}",
        ur
    );
}

// --- TBL 出典が正しく読まれて missing_source にならない ---

// @kotowari[REQ-098]
#[test]
fn req_098_tbl_with_valid_source_no_missing_source() {
    let content = "\
# Title

Scope.

## 決定表

### TBL-001: T

- 出典: brainstorm/records.md#A1

| A | B |
|---|---|
| 1 | 2 |
";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ms = find_by_kind(&findings, "missing_source");
    assert!(
        !ms.iter().any(|f| f.detail == "TBL-001"),
        "TBL with valid source should not produce missing_source: {:?}",
        ms
    );
}

// --- Step 3: 文書の読み込み ---

// @kotowari[REQ-033]
#[test]
fn req_033_uppercase_md_is_not_read() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/ir")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/decision/records")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/decision/adr")).unwrap();
    std::fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\n",
    ).unwrap();
    // .MD ファイルは読まない
    std::fs::write(tmp.path().join("docs/ir/README.MD"), "# Title\n\nScope.\n").unwrap();
    let output = assert_cmd::Command::cargo_bin("kotowari").unwrap()
        .arg("check")
        .current_dir(tmp.path())
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert_eq!(v["files"], 0, ".MD file should not be read");
}

// @kotowari[REQ-033]
#[test]
fn req_033_file_symlink_is_read() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/ir")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/decision/records")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/decision/adr")).unwrap();
    std::fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\n",
    ).unwrap();
    // 実体を別の場所に作り、シンボリックリンクを ir/ に置く
    let target = tmp.path().join("target.md");
    std::fs::write(&target, "# Title\n\nScope.\n").unwrap();
    std::os::unix::fs::symlink(&target, tmp.path().join("docs/ir/link.md")).unwrap();
    let output = assert_cmd::Command::cargo_bin("kotowari").unwrap()
        .arg("check")
        .current_dir(tmp.path())
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert_eq!(v["files"], 1, "symlinked file should be read");
}

// @kotowari[REQ-111]
#[test]
fn req_111_bom_is_skipped_in_ir_config_records_adr_and_tests() {
    // BOM 付きの文書が正常に読まれることを確認
    let bom = "\u{FEFF}";
    let content = format!("{bom}# Title\n\nScope.\n");
    let doc = ir::parse_document("a.md", &content);
    assert!(doc.title.is_some(), "BOM should not prevent title parsing");
    assert_eq!(doc.title.as_ref().unwrap().1, "Title");
}

// @kotowari[REQ-040]
#[test]
fn req_040_tilde_fence_is_a_code_block() {
    let content = "# Title\n\nScope.\n\n## Section\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\n~~~\nSome code\n~~~\n\nStatement.\n";
    let doc = ir::parse_document("a.md", content);
    // ~~~ で囲んだブロックの中は検査されない
    let req = doc.items.iter().find(|i| i.id() == Some("REQ-001")).unwrap();
    if let Item::Requirement { statements, .. } = req {
        // "Some code" は文として拾われない
        assert!(!statements.iter().any(|(_, s)| s.contains("Some code")),
            "content inside ~~~ block should not be parsed as statement");
    }
}

// @kotowari[REQ-040]
#[test]
fn req_040_longer_fence_needs_same_or_longer_close() {
    let content = "# Title\n\nScope.\n\n## Section\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\n````\n```\nstill inside\n````\n\nStatement.\n";
    let doc = ir::parse_document("a.md", content);
    let req = doc.items.iter().find(|i| i.id() == Some("REQ-001")).unwrap();
    if let Item::Requirement { statements, .. } = req {
        // ``` は ```` を閉じない
        assert!(!statements.iter().any(|(_, s)| s.contains("still inside")),
            "``` should not close ```` block");
        assert!(statements.iter().any(|(_, s)| s.contains("Statement")),
            "Statement after closing ```` should be parsed");
    }
}

// @kotowari[REQ-112]
#[test]
fn req_112_unclosed_code_block_is_an_error() {
    let content = "# Title\n\nScope.\n\n## Section\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\n```\nunclosed content\n";
    let doc = ir::parse_document("a.md", content);
    let uc = doc.parse_findings.iter()
        .find(|f| f.kind == "unclosed_code_block");
    assert!(uc.is_some(), "should produce unclosed_code_block finding");
    let uc = uc.unwrap();
    assert_eq!(uc.line, Some(13), "line should be the opening line");
    assert_eq!(uc.detail, "```", "detail should be the raw opening line");
}

// @kotowari[REQ-112]
#[test]
fn req_112_unclosed_gherkin_block_is_not_checked() {
    let content = "# Title\n\nScope.\n\n## Section\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\n```gherkin\n@id=EX-001 @about=REQ-001 @source=brainstorm/records.md#A1\nScenario: test\n  Given something\n";
    let doc = ir::parse_document("a.md", content);
    // gherkin ブロックが閉じないときも unclosed_code_block が出る
    let uc = doc.parse_findings.iter()
        .find(|f| f.kind == "unclosed_code_block");
    assert!(uc.is_some(), "unclosed gherkin block should produce unclosed_code_block");
    // 閉じないブロック内のシナリオはアイテムにならない
    assert!(doc.items.iter().all(|i| !matches!(i, Item::Scenario { .. })),
        "scenario inside unclosed block should not be parsed");
}

// @kotowari[REQ-112, REQ-040]
#[test]
fn req_112_unclosed_gherkin_block_with_multiple_scenarios_excludes_items() {
    // 複数シナリオがある閉じない gherkin ブロック。
    // 2番目のタグ行で1番目のシナリオが flush されるが、
    // ブロックが閉じないので items からも除去されなければならない。
    let content = "\
# Title

Scope.

## Section

```gherkin
@id=EX-001 @about=REQ-001 @source=brainstorm/records.md#A1
Scenario: first
  Given something

@id=EX-002 @about=REQ-001 @source=brainstorm/records.md#A1
Scenario: second
  Given another
";
    let doc = ir::parse_document("a.md", content);
    // unclosed_code_block は出る
    let uc = doc.parse_findings.iter()
        .find(|f| f.kind == "unclosed_code_block");
    assert!(uc.is_some(), "should produce unclosed_code_block");
    // 閉じないブロック内のシナリオは items に残らない
    let scenarios: Vec<_> = doc.items.iter()
        .filter(|i| matches!(i, Item::Scenario { .. }))
        .collect();
    assert!(scenarios.is_empty(),
        "scenarios inside unclosed gherkin block should be excluded from items, but found {}",
        scenarios.len());
}

// @kotowari[TBL-010]
#[test]
fn tbl_010_empty_document_has_zero_lines_and_missing_title() {
    let doc = ir::parse_document("a.md", "");
    assert_eq!(doc.line_count, 0, "empty document should have 0 lines");
    assert!(doc.title.is_none(), "empty document should have no title");
    let config = default_config();
    let findings = check(&[doc], &config);
    let mt = find_by_kind(&findings, "missing_title");
    assert!(!mt.is_empty(), "empty document should produce missing_title");
}

// --- Step 4a: 項目の行の形 ---

// @kotowari[REQ-044]
#[test]
fn req_044_star_plus_numbered_and_bare_dash_lines_are_unknown_fields() {
    let content = "# Title\n\nScope.\n\n## Section\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\n* star line\n+ plus line\n1. numbered line\n-\n\nStatement.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let uf = find_by_kind(&findings, "unknown_field");
    assert!(uf.len() >= 4, "should produce at least 4 unknown_field findings, got {}", uf.len());
}

// @kotowari[REQ-044]
#[test]
fn req_044_detail_is_the_raw_line() {
    let content = "# Title\n\nScope.\n\n## Section\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\n  * indented star\n\nStatement.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let uf = find_by_kind(&findings, "unknown_field");
    // detail は字下げを含む読んだ行そのまま
    assert!(uf.iter().any(|f| f.detail.contains("  * indented star")),
        "detail should contain raw indented line, got: {:?}", uf);
}

// @kotowari[REQ-043]
#[test]
fn req_043_deeper_heading_is_unknown_heading() {
    let content = "# Title\n\nScope.\n\n## Section\n\n#### DEEP-001: Deep\n\nSome text.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let uh = find_by_kind(&findings, "unknown_heading");
    assert!(uh.iter().any(|f| f.detail.contains("DEEP-001")),
        "#### heading should produce unknown_heading");
}

// @kotowari[REQ-043]
#[test]
fn req_043_lines_under_unknown_heading_are_not_an_item() {
    let content = "# Title\n\nScope.\n\n## Section\n\n### BADID: X\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n\nStatement.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    // unknown_heading は出るが、missing_field などは出ない
    let uh = find_by_kind(&findings, "unknown_heading");
    assert!(!uh.is_empty(), "should produce unknown_heading for BADID");
    let mf = find_by_kind(&findings, "missing_field");
    assert!(mf.is_empty(), "lines under unknown heading should not be checked as item fields");
}

// @kotowari[REQ-043]
#[test]
fn req_043_valid_prefix_invalid_digits_is_not_an_item() {
    // 有効な接頭辞 REQ- に3桁でない数字 → 項目として構築されない
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-1: Bad\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n- 定義: TBL-999\n\nStatement.\n";
    let doc = ir::parse_document("a.md", content);
    // parse 段で Item::Requirement ではなく Item::UnknownHeading になること
    assert!(
        !doc.items.iter().any(|i| matches!(i, Item::Requirement { .. })),
        "valid prefix + invalid digits must not produce Item::Requirement"
    );
    assert!(
        doc.items.iter().any(|i| matches!(i, Item::UnknownHeading { .. })),
        "valid prefix + invalid digits must produce Item::UnknownHeading"
    );
    // check_documents で項目として検査されないこと（unresolved_reference が出ない）
    let findings = check(&[doc], &default_config());
    let ur = find_by_kind(&findings, "unresolved_reference");
    assert!(ur.is_empty(), "lines under invalid-digit heading should not produce unresolved_reference, got {:?}", ur);
    // unknown_heading は出る
    let uh = find_by_kind(&findings, "unknown_heading");
    assert!(!uh.is_empty(), "should produce unknown_heading for REQ-1");
}

// @kotowari[REQ-045]
#[test]
fn req_045_third_known_line_gives_two_duplicates() {
    let content = "# Title\n\nScope.\n\n## Section\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n- 種類: event_driven\n- 種類: state_driven\n\nStatement.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let df = find_by_kind(&findings, "duplicate_field");
    assert_eq!(df.len(), 2, "3 occurrences of same field should give 2 duplicate_field, got {}", df.len());
}

// @kotowari[REQ-045]
#[test]
fn req_045_unknown_line_repeated_gives_only_unknown_field() {
    let content = "# Title\n\nScope.\n\n## Section\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n- 優先度: 高\n- 優先度: 低\n\nStatement.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let uf = find_by_kind(&findings, "unknown_field");
    let df = find_by_kind(&findings, "duplicate_field");
    assert_eq!(uf.len(), 2, "2 unknown lines should give 2 unknown_field");
    assert_eq!(df.len(), 0, "unknown lines should not give duplicate_field");
}

// @kotowari[REQ-047]
#[test]
fn req_047_requirement_without_kind_line_needs_statement() {
    // 種類の行が無い要求に文が無ければ missing_statement
    let content = "# Title\n\nScope.\n\n## Section\n\n### REQ-001: R\n\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ms = find_by_kind(&findings, "missing_statement");
    assert!(!ms.is_empty(), "requirement without kind and without statement should produce missing_statement");
}

// @kotowari[REQ-032]
#[test]
fn req_032_first_occurrence_is_bytewise_first_path() {
    // duplicate_id の1つ目はパスのバイト順で先の文書
    let doc_a = ir::parse_document("a.md", "# Title\n\nScope.\n\n## Section\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\nStatement.\n");
    let doc_b = ir::parse_document("b.md", "# Title\n\nScope.\n\n## Section\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\nStatement.\n");
    let findings = check(&[doc_a, doc_b], &default_config());
    let di = find_by_kind(&findings, "duplicate_id");
    assert_eq!(di.len(), 1, "should produce exactly 1 duplicate_id");
    assert!(di[0].path.contains("b.md"), "duplicate_id should be on the second (b.md) path, got {:?}", di[0].path);
}

// --- Step 4b: gherkin の行の形と ID の定義 ---

// @kotowari[REQ-113]
#[test]
fn req_113_indented_steps_are_recognized() {
    // 2字下げのステップが正しく読まれる
    let content = "\
# Title

Scope.

## 具体例

```gherkin
@id=EX-001 @about=REQ-001 @source=brainstorm/records.md#A1
Scenario: Indented steps
  Given a precondition
  When an action
  Then a result
```

## 要求

### REQ-001: R

- 種類: ubiquitous
- 出典: brainstorm/records.md#A1
- 検証: unit

Statement.
";
    let doc = ir::parse_document("a.md", content);
    let scenario = doc.items.iter().find(|i| matches!(i, Item::Scenario { id: Some(id), .. } if id == "EX-001"));
    assert!(scenario.is_some(), "should parse EX-001");
    if let Item::Scenario { steps, .. } = scenario.unwrap() {
        assert_eq!(steps.len(), 3, "should have 3 steps: {:?}", steps);
    }
}

// @kotowari[REQ-113]
#[test]
fn req_113_feature_and_examples_lines_are_invalid() {
    let content = "\
# Title

Scope.

## 具体例

```gherkin
@id=EX-001 @about=REQ-001 @source=brainstorm/records.md#A1
Scenario: Test
  Given something
Feature: Bad line
Background: Also bad
Scenario Outline: Bad
Examples: Bad
| data | table |
```

## 要求

### REQ-001: R

- 種類: ubiquitous
- 出典: brainstorm/records.md#A1
- 検証: unit

Statement.
";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ig = find_by_kind(&findings, "invalid_gherkin_line");
    assert!(ig.len() >= 5, "should have at least 5 invalid_gherkin_line findings, got {}: {:?}", ig.len(), ig);
    assert!(ig.iter().any(|f| f.detail.contains("Feature:")), "Feature: should be invalid: {:?}", ig);
    assert!(ig.iter().any(|f| f.detail.contains("Background:")), "Background: should be invalid: {:?}", ig);
    assert!(ig.iter().any(|f| f.detail.contains("Scenario Outline:")), "Scenario Outline: should be invalid: {:?}", ig);
    assert!(ig.iter().any(|f| f.detail.contains("Examples:")), "Examples: should be invalid: {:?}", ig);
}

// @kotowari[REQ-113]
#[test]
fn req_113_tag_line_binds_only_when_immediately_before_scenario() {
    // タグの行と Scenario: の間に行があると結び付かない
    let content = "\
# Title

Scope.

## 具体例

```gherkin
@id=EX-001 @about=REQ-001 @source=brainstorm/records.md#A1
# a comment
Scenario: With gap
  Given something
```

## 要求

### REQ-001: R

- 種類: ubiquitous
- 出典: brainstorm/records.md#A1
- 検証: unit

Statement.
";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let mt = find_by_kind(&findings, "missing_tag");
    // タグが結び付かないので missing_tag が出る
    assert!(mt.iter().any(|f| f.detail == "@id"), "tag should not bind through comment: {:?}", mt);
}

// @kotowari[REQ-052]
#[test]
fn req_052_unbound_tag_line_is_still_checked() {
    // 結び付かないタグの行でも unknown_tag は出る
    let content = "\
# Title

Scope.

## 具体例

```gherkin
@id=EX-001 @about=REQ-001 @source=brainstorm/records.md#A1 @wip
# comment breaks binding
Scenario: Test
  Given something
```

## 要求

### REQ-001: R

- 種類: ubiquitous
- 出典: brainstorm/records.md#A1
- 検証: unit

Statement.
";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    // タグ行は結び付いていないが、unknown_tag は出る
    // 注意: タグが結び付かないとき、そのシナリオのタグは空になる
    // 実際の unknown_tag は結び付かないタグ行にも出るべき
    // ここでの意図は、タグ行からの unknown_tag が出ること
    // parse_document でタグ行をフラッシュする前にタグの検査を行う必要がある
    // 現在の実装では、結び付かないタグは前のシナリオに含まれるか、
    // フラッシュされて新しいシナリオに含まれるか。
    // テストの意図: 結び付かないタグの行の@wip が unknown_tag になること
    let ut = find_by_kind(&findings, "unknown_tag");
    assert!(ut.iter().any(|f| f.detail == "@wip"), "unbound tag line should still produce unknown_tag for @wip: {:?}", ut);
}

// @kotowari[REQ-052]
#[test]
fn req_052_word_without_at_in_tag_line_is_unknown_tag() {
    // タグの行の "@" で始まらない語も unknown_tag
    let content = "\
# Title

Scope.

## 具体例

```gherkin
@id=EX-001 @about=REQ-001 @source=brainstorm/records.md#A1 badword
Scenario: Test
  Given something
```

## 要求

### REQ-001: R

- 種類: ubiquitous
- 出典: brainstorm/records.md#A1
- 検証: unit

Statement.
";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ut = find_by_kind(&findings, "unknown_tag");
    assert!(ut.iter().any(|f| f.detail == "badword"), "word without @ in tag line should be unknown_tag: {:?}", ut);
}

// @kotowari[REQ-114]
#[test]
fn req_114_malformed_id_tag_is_invalid_id_and_not_defined() {
    // @id=EX1 は invalid_id、定義に数えない
    let content = "\
# Title

Scope.

## 具体例

```gherkin
@id=EX1 @about=REQ-001 @source=brainstorm/records.md#A1
Scenario: Bad id
  Given something
```

## 要求

### REQ-001: R

- 種類: ubiquitous
- 出典: brainstorm/records.md#A1
- 検証: unit

Statement.
";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ii = find_by_kind(&findings, "invalid_id");
    assert!(ii.iter().any(|f| f.detail == "EX1"), "should produce invalid_id for EX1: {:?}", ii);
    let mt = find_by_kind(&findings, "missing_tag");
    assert!(!mt.iter().any(|f| f.detail == "@id"), "should not produce missing_tag @id when invalid_id: {:?}", mt);
}

// @kotowari[REQ-114]
#[test]
fn req_114_malformed_id_scenario_missing_source_detail_is_scenario_line() {
    // @id が形に合わない → missing_source の detail は Scenario: の行の文字
    let content = "\
# Title

Scope.

## 具体例

```gherkin
@id=EX1 @about=REQ-001
Scenario: Malformed id test
  Given something
```

## 要求

### REQ-001: R

- 種類: ubiquitous
- 出典: brainstorm/records.md#A1
- 検証: unit

Statement.
";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ms = find_by_kind(&findings, "missing_source");
    assert!(ms.iter().any(|f| f.detail == "Scenario: Malformed id test"),
        "missing_source detail should be the Scenario: line text: {:?}", ms);
}

// @kotowari[REQ-114]
#[test]
fn req_114_malformed_heading_is_not_defined() {
    // ### REQ-1: x は形に合わないので定義に数えない
    let content = "\
# Title

Scope.

## 要求

### REQ-1: Bad

- 種類: ubiquitous
- 出典: brainstorm/records.md#A1
- 検証: unit

Statement.

### REQ-001: R

- 種類: ubiquitous
- 出典: brainstorm/records.md#A1
- 検証: unit

`REQ-1` is referenced.
";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    // REQ-1 は定義されていないので unresolved_reference
    // ただし REQ-1 は is_valid_id を通らないので check_backtick_ids では拾われない
    // unknown_heading で報告される
    let uh = find_by_kind(&findings, "unknown_heading");
    assert!(uh.iter().any(|f| f.detail.contains("REQ-1")), "REQ-1 should be unknown_heading: {:?}", uh);
}

// @kotowari[REQ-054]
#[test]
fn req_054_non_id_definition_value_is_unresolved() {
    // "- 定義: foo" は ID の形でないので unresolved_reference
    let content = "\
# Title

Scope.

## 要求

### REQ-001: R

- 種類: algorithm
- 出典: brainstorm/records.md#A1
- 検証: unit
- 定義: foo
";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ur = find_by_kind(&findings, "unresolved_reference");
    assert!(ur.iter().any(|f| f.detail == "foo"), "non-ID definition value should produce unresolved_reference: {:?}", ur);
}

// @kotowari[REQ-054]
#[test]
fn req_054_backtick_id_in_step_is_checked() {
    // ステップの行のバッククォートで囲んだ ID も存在を検査する
    let content = "\
# Title

Scope.

## 要求

### REQ-001: R

- 種類: ubiquitous
- 出典: brainstorm/records.md#A1
- 検証: unit

Statement.

## 具体例

```gherkin
@id=EX-001 @about=REQ-001 @source=brainstorm/records.md#A1
Scenario: Backtick in step
  Given `TBL-999` does not exist
```
";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ur = find_by_kind(&findings, "unresolved_reference");
    assert!(ur.iter().any(|f| f.detail == "TBL-999"), "backtick ID in step should produce unresolved_reference: {:?}", ur);
}

// @kotowari[REQ-033]
#[test]
#[cfg(unix)]
fn req_033_broken_symlink_in_ir_dir_stops() {
    use std::os::unix::fs::symlink;
    let tmp = tempfile::TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/ir")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/decision/records")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/decision/adr")).unwrap();
    std::fs::write(tmp.path().join(".kotowari/config.yaml"), "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\n").unwrap();
    symlink(tmp.path().join("nowhere.md"), tmp.path().join("docs/ir/broken.md")).unwrap();
    let output = assert_cmd::Command::cargo_bin("kotowari").unwrap().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(output.status.code(), Some(2), "a broken symlink in the IR dir must stop: {:?}", output);
}

// --- 汎用化の実装レビューで見つかった食い違いの回帰テスト ---

// @kotowari[REQ-044, TBL-008]
#[test]
fn req_044_unknown_field_detail_is_raw_line_not_reconstructed() {
    // 名前と値の間の空白が崩れている行でも、detail は読んだ行の文字そのまま
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n  - 優先度:高 \n\nStatement.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let uf = find_by_kind(&findings, "unknown_field");
    assert!(
        uf.iter().any(|f| f.detail == "  - 優先度:高 "),
        "unknown_field detail should be the raw line as read, not reconstructed from name and value: {:?}",
        uf
    );
}

// @kotowari[REQ-100]
#[test]
fn req_100_scenario_line_under_heading_is_excluded_from_statement() {
    // 見出しの下に "Scenario: あ" だけを書いても、文として拾わず用語検査も受けない
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\nScenario: あ\n";
    let doc = ir::parse_document("a.md", content);
    let req = doc
        .items
        .iter()
        .find(|i| matches!(i, Item::Requirement { id, .. } if id == "REQ-001"))
        .expect("REQ-001 should parse");
    if let Item::Requirement { statements, .. } = req {
        assert!(
            statements.is_empty(),
            "a 'Scenario:' line outside a gherkin block must not become a statement: {:?}",
            statements
        );
    }
    let findings = check(&[doc], &default_config());
    let ms = find_by_kind(&findings, "missing_statement");
    assert!(
        ms.iter().any(|f| f.detail == "REQ-001"),
        "missing_statement should fire when only a stray 'Scenario:' line is present: {:?}",
        ms
    );
}

// @kotowari[REQ-059, TBL-008]
#[test]
fn tbl_008_missing_source_scenario_detail_is_raw_scenario_line() {
    // A150: @id の無いシナリオの missing_source detail は、字下げを含む生の Scenario: の行
    let content = "# Title\n\nScope.\n\n## 具体例\n\n```gherkin\n@about=REQ-001\n  Scenario: あ\n  Given something\n```\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ms = find_by_kind(&findings, "missing_source");
    assert!(
        ms.iter().any(|f| f.detail == "  Scenario: あ"),
        "missing_source detail for an @id-less scenario should be the raw Scenario: line: {:?}",
        ms
    );
}

// @kotowari[REQ-052]
#[test]
fn req_052_word_with_equals_not_starting_with_at_keeps_full_word_as_detail() {
    // REQ-052: "@" で始まらない語は、"=" があっても分けずに全体を detail にする
    let content = "# Title\n\nScope.\n\n## 具体例\n\n```gherkin\n@id=EX-001 @about=REQ-001 @source=brainstorm/records.md#A1 foo=bar\nScenario: Test\n  Given something\n```\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\nStatement.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ut = find_by_kind(&findings, "unknown_tag");
    assert!(
        ut.iter().any(|f| f.detail == "foo=bar"),
        "a word without a leading '@' should keep 'foo=bar' whole as the detail: {:?}",
        ut
    );
}

// @kotowari[REQ-054]
#[test]
fn req_054_backtick_id_inside_double_quotes_is_not_checked() {
    // REQ-054/REQ-104: 二重引用符の中のバッククォートの ID は参照の検査を受けない
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\nSee \"`REQ-999`\" for details.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ur = find_by_kind(&findings, "unresolved_reference");
    assert!(
        ur.is_empty(),
        "a backtick ID inside double quotes should not be checked as a reference: {:?}",
        ur
    );
}

// @kotowari[REQ-054, REQ-116]
#[test]
fn req_054_backtick_oddness_counted_outside_quotes_only() {
    // 引用符の中の "`" を数に入れない: 引用符の外だけを見れば偶数なので検査が行われる
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\n`REQ-999` and \"quoted ` mark\" here.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ur = find_by_kind(&findings, "unresolved_reference");
    assert!(
        ur.iter().any(|f| f.detail == "REQ-999"),
        "backtick oddness should be counted outside double quotes only, so the reference check should still run: {:?}",
        ur
    );
}

// @kotowari[REQ-122]
#[test]
fn req_122_glossary_row_with_missing_column_is_invalid() {
    let content = "\
# 用語集

| 用語 | 意味 | 出典 |
|---|---|---|
| テスト | 検証の意味
";
    let doc = ir::parse_document("CONTEXT.md", content);
    let terms: Vec<_> = doc
        .items
        .iter()
        .filter(|i| matches!(i, Item::GlossaryTerm { .. }))
        .collect();
    assert!(
        terms.is_empty(),
        "a row with fewer than 4 columns must not become a term: {:?}",
        terms
    );
    let findings = check(&[doc], &default_config());
    let igr = find_by_kind(&findings, "invalid_glossary_row");
    assert_eq!(igr.len(), 1);
    assert_eq!(igr[0].detail, "| テスト | 検証の意味");
    assert_eq!(igr[0].line, Some(5));
}

// @kotowari[REQ-122]
#[test]
fn req_122_glossary_row_with_empty_term_cell_is_invalid() {
    let content = "\
# 用語集

| 用語 | 意味 | 出典 |
|---|---|---|
|  | 意味 | brainstorm/records.md#A1 |
";
    let doc = ir::parse_document("CONTEXT.md", content);
    let terms: Vec<_> = doc
        .items
        .iter()
        .filter(|i| matches!(i, Item::GlossaryTerm { .. }))
        .collect();
    assert!(
        terms.is_empty(),
        "a row with an empty term cell must not become a term: {:?}",
        terms
    );
    let findings = check(&[doc], &default_config());
    let igr = find_by_kind(&findings, "invalid_glossary_row");
    assert_eq!(igr.len(), 1);
    assert_eq!(igr[0].line, Some(5));
}

// @kotowari[REQ-123]
#[test]
fn req_123_duplicate_term_reported_for_second_row_onward() {
    let content = "\
# 用語集

| 用語 | 意味 | 出典 |
|---|---|---|
| IR | 最初の意味 | brainstorm/records.md#A1 |
| IR | 2つ目の意味 | brainstorm/records.md#A1 |
| IR | 3つ目の意味 | brainstorm/records.md#A1 |
";
    let doc = ir::parse_document("CONTEXT.md", content);
    let findings = check(&[doc], &default_config());
    let dt = find_by_kind(&findings, "duplicate_term");
    assert_eq!(dt.len(), 2, "the 2nd and 3rd rows should each produce a duplicate_term: {:?}", dt);
    assert!(dt.iter().all(|f| f.detail == "IR"));
    assert_eq!(dt[0].line, Some(6));
    assert_eq!(dt[1].line, Some(7));
}

// @kotowari[REQ-113]
#[test]
fn req_113_step_without_preceding_scenario_is_invalid() {
    let content = "# Title\n\nScope.\n\n## 具体例\n\n```gherkin\nThen this step has no Scenario\n```\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ig = find_by_kind(&findings, "invalid_gherkin_line");
    assert!(
        ig.iter().any(|f| f.detail == "Then this step has no Scenario"),
        "a step line with no preceding Scenario: should be invalid_gherkin_line: {:?}",
        ig
    );
}

// @kotowari[REQ-113]
#[test]
fn req_113_tag_line_not_immediately_before_scenario_is_invalid() {
    // タグの行の直後が空行で、Scenario: がその次に来る → タグの行自体が invalid_gherkin_line
    let content = "# Title\n\nScope.\n\n## 具体例\n\n```gherkin\n@id=EX-001\n\nScenario: Test\n  Given something\n```\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ig = find_by_kind(&findings, "invalid_gherkin_line");
    assert!(
        ig.iter().any(|f| f.detail == "@id=EX-001"),
        "a tag line not immediately followed by Scenario: should itself be invalid_gherkin_line: {:?}",
        ig
    );
}

// @kotowari[REQ-098]
#[test]
fn req_098_empty_verification_value_is_missing_not_invalid() {
    // A157: "- 検証: " のように値が空の行は、行が無いものとして扱う
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: \n\nStatement.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let vm = find_by_kind(&findings, "verification_missing");
    assert_eq!(vm.len(), 1, "an empty verification value should be treated as a missing line: {:?}", vm);
    let vi = find_by_kind(&findings, "verification_invalid");
    assert!(vi.is_empty(), "an empty verification value must not be verification_invalid: {:?}", vi);
}

// @kotowari[REQ-098]
#[test]
fn req_098_empty_kind_value_is_missing_field() {
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: \n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\nStatement.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let mf = find_by_kind(&findings, "missing_field");
    assert!(
        mf.iter().any(|f| f.detail == "種類"),
        "an empty kind value should be treated as a missing '- 種類:' line: {:?}",
        mf
    );
}

// @kotowari[REQ-098]
#[test]
fn req_098_empty_definition_value_on_algorithm_is_without_definition() {
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: Test\n\n- 種類: algorithm\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n- 定義: \n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ad = find_by_kind(&findings, "algorithm_without_definition");
    assert!(
        ad.iter().any(|f| f.detail == "REQ-001"),
        "an empty definition value on an algorithm requirement should be treated as missing: {:?}",
        ad
    );
}

// @kotowari[REQ-114]
#[test]
fn req_114_malformed_id_still_reports_missing_about() {
    // A151: @id が形に合わなくても、@about が無ければ missing_tag @about は出る
    let content = "# Title\n\nScope.\n\n## 具体例\n\n```gherkin\n@id=REQ-001 @source=brainstorm/records.md#A1\nScenario: Malformed id\n  Given something\n```\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ii = find_by_kind(&findings, "invalid_id");
    assert!(!ii.is_empty(), "a malformed @id should still produce invalid_id: {:?}", ii);
    let mt = find_by_kind(&findings, "missing_tag");
    assert!(
        mt.iter().any(|f| f.detail == "@about"),
        "missing_tag for @about should still fire when @id is malformed: {:?}",
        mt
    );
    assert!(
        !mt.iter().any(|f| f.detail == "@id"),
        "missing_tag for @id must be suppressed when @id is malformed (invalid_id covers it): {:?}",
        mt
    );
}

// @kotowari[REQ-034]
#[test]
fn req_034_lines_before_title_are_ignored() {
    // A156: 題名より前にある空でない行は読まない（除外）
    let content = "Not a title yet.\n\n# Title\n\nScope.\n";
    let doc = ir::parse_document("a.md", content);
    assert_eq!(doc.title, Some((3, "Title".to_string())), "the title should be recognized on line 3");
    let findings = check(&[doc], &default_config());
    let mt = find_by_kind(&findings, "missing_title");
    assert!(mt.is_empty(), "a title on a later line should still count as the title: {:?}", mt);
}

// @kotowari[REQ-113]
#[test]
fn req_113_tags_do_not_leak_into_the_next_untagged_scenario() {
    // タグの付いた1つ目のシナリオの直後に、タグの無い2つ目のシナリオが続くとき、
    // 2つ目のシナリオへタグが漏れて結び付いてはいけない
    let content = "\
# Title

Scope.

## 具体例

```gherkin
@id=EX-001 @about=REQ-001 @source=brainstorm/records.md#A1
Scenario: First
  Given a
Scenario: Second
  Given b
```

## 要求

### REQ-001: R

- 種類: ubiquitous
- 出典: brainstorm/records.md#A1
- 検証: unit

Statement.
";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let mt = find_by_kind(&findings, "missing_tag");
    assert!(
        mt.iter().any(|f| f.detail == "@id"),
        "the second, untagged scenario should be missing @id, not inherit the first scenario's tag: {:?}",
        mt
    );
    assert!(
        mt.iter().any(|f| f.detail == "@about"),
        "the second, untagged scenario should be missing @about too: {:?}",
        mt
    );
    let di = find_by_kind(&findings, "duplicate_id");
    assert!(
        di.is_empty(),
        "the second scenario must not leak EX-001 from the first and register as a duplicate: {:?}",
        di
    );
}

// @kotowari[REQ-113]
#[test]
fn req_113_consecutive_steps_without_scenario_report_only_the_first() {
    // A155: Scenario: の無いブロックで続く2つ目以降のステップは、最初のステップの誤りに含める
    let content = "# Title\n\nScope.\n\n## 具体例\n\n```gherkin\nGiven first orphan\nWhen second orphan\nThen third orphan\n```\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ig = find_by_kind(&findings, "invalid_gherkin_line");
    assert_eq!(ig.len(), 1, "only the first orphan step should be reported: {:?}", ig);
    assert_eq!(ig[0].detail, "Given first orphan");
    assert_eq!(ig[0].line, Some(8));
}

// @kotowari[REQ-113]
#[test]
fn req_113_orphan_step_after_a_blank_line_is_reported_again() {
    // REQ-113 の「直前」は直前の行。空行を挟んだステップは直前にステップが無いので、改めて誤り
    let content = "# Title\n\nScope.\n\n## 具体例\n\n```gherkin\nGiven first orphan\n\nThen after blank\n```\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ig = find_by_kind(&findings, "invalid_gherkin_line");
    let details: Vec<&str> = ig.iter().map(|f| f.detail.as_str()).collect();
    assert_eq!(details, vec!["Given first orphan", "Then after blank"], "{:?}", ig);
}

// @kotowari[REQ-113]
#[test]
fn req_113_step_right_after_tag_line_reports_both_lines() {
    // A155: タグの行の直後がステップなら、タグの行（直後が Scenario: でない）とそのステップ（直前に Scenario: が無い）の両方
    let content = "# Title\n\nScope.\n\n## 具体例\n\n```gherkin\n@id=EX-001\nGiven no scenario line\n```\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ig = find_by_kind(&findings, "invalid_gherkin_line");
    let details: Vec<&str> = ig.iter().map(|f| f.detail.as_str()).collect();
    assert_eq!(details, vec!["@id=EX-001", "Given no scenario line"], "{:?}", ig);
    let ut = find_by_kind(&findings, "unknown_tag");
    assert!(ut.is_empty(), "@id is a known tag, so no unknown_tag: {:?}", ut);
}

// @kotowari[REQ-113]
#[test]
fn req_113_tag_line_right_before_closing_fence_is_invalid() {
    let content = "# Title\n\nScope.\n\n## 具体例\n\n```gherkin\nScenario: ok\n  Given something\n@about=REQ-001 @nope=x\n```\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ig = find_by_kind(&findings, "invalid_gherkin_line");
    assert_eq!(ig.len(), 1, "{:?}", ig);
    assert_eq!(ig[0].detail, "@about=REQ-001 @nope=x");
    assert_eq!(ig[0].line, Some(10));
    // A155: 結び付かないタグの行でも unknown_tag の検査はそのまま行う
    let ut = find_by_kind(&findings, "unknown_tag");
    assert!(ut.iter().any(|f| f.line == Some(10)), "unknown_tag for @nope should still be reported: {:?}", ut);
}

// @kotowari[REQ-122]
#[test]
fn req_122_two_cell_row_with_trailing_pipe_is_invalid() {
    // A163: 判定はセルの数。"| a | b |" はセルが2つなので崩れた行
    let content = "\
# 用語集

| 用語 | 意味 | 出典 |
|---|---|---|
| テスト | 検証の意味 |
";
    let doc = ir::parse_document("CONTEXT.md", content);
    let terms: Vec<_> = doc.items.iter().filter(|i| matches!(i, Item::GlossaryTerm { .. })).collect();
    assert!(terms.is_empty(), "a 2-cell row must not become a term: {:?}", terms);
    let findings = check(&[doc], &default_config());
    let igr = find_by_kind(&findings, "invalid_glossary_row");
    assert_eq!(igr.len(), 1, "{:?}", findings);
    assert_eq!(igr[0].detail, "| テスト | 検証の意味 |");
}

// @kotowari[REQ-122]
#[test]
fn req_122_three_cells_without_trailing_pipe_is_a_term() {
    // A163: 末尾の "|" が無くてもセルが3つあれば用語
    let content = "\
# 用語集

| 用語 | 意味 | 出典 |
|---|---|---|
| テスト | 検証の意味 | brainstorm/records.md#A1
";
    let doc = ir::parse_document("CONTEXT.md", content);
    let terms: Vec<_> = doc.items.iter().filter(|i| matches!(i, Item::GlossaryTerm { .. })).collect();
    assert_eq!(terms.len(), 1, "{:?}", doc.items);
    let findings = check(&[doc], &default_config());
    assert!(find_by_kind(&findings, "invalid_glossary_row").is_empty(), "{:?}", findings);
}

// @kotowari[REQ-123]
#[test]
fn req_123_duplicate_row_is_not_a_term() {
    // A162: 2つ目以降の行は duplicate_term だけを出し、用語にしない（出典が空でも missing_source は出ない）
    let content = "\
# 用語集

| 用語 | 意味 | 出典 |
|---|---|---|
| IR | 最初の意味 | brainstorm/records.md#A1 |
| IR | 2つ目の意味 | |
";
    let doc = ir::parse_document("CONTEXT.md", content);
    let terms: Vec<_> = doc.items.iter().filter(|i| matches!(i, Item::GlossaryTerm { .. })).collect();
    assert_eq!(terms.len(), 1, "the duplicate row must not become a term: {:?}", terms);
    let findings = check(&[doc], &default_config());
    let dt = find_by_kind(&findings, "duplicate_term");
    assert_eq!(dt.len(), 1);
    assert_eq!(dt[0].line, Some(6));
    let ms = find_by_kind(&findings, "missing_source");
    assert!(!ms.iter().any(|f| f.line == Some(6)), "no other check on the duplicate row: {:?}", ms);
}

// --- TBL-010: split_lines の \r\n 処理 ---

// @kotowari[TBL-010]
#[test]
fn tbl_010_split_lines_strips_cr_and_does_not_panic_on_bare_lf() {
    // \r\n は CR を取り除いた1行になる
    assert_eq!(
        ir::split_lines("a\r\nb"),
        vec!["a", "b"],
        "a CRLF line should have its trailing CR stripped from the content"
    );
    // 先頭がいきなり \n （CR無し）でも panic しない
    assert_eq!(
        ir::split_lines("\na"),
        vec!["", "a"],
        "a bare leading \\n must not panic and must produce an empty first line"
    );
}

// --- REQ-117: 用語集の区切り行の判定 ---

// @kotowari[REQ-117]
#[test]
fn req_117_non_dash_row_after_header_is_not_a_valid_separator() {
    // ヘッダの直後の行が "-" だけのセルでなければ、区切り行として認めてはいけない
    let content = "\
# 用語集

| 用語 | 意味 | 出典 |
| foo | bar | baz |
| IR | 仕様 | src |
";
    let doc = ir::parse_document("CONTEXT.md", content);
    assert!(
        doc.items.iter().all(|i| !matches!(i, Item::GlossaryTerm { .. })),
        "without a real '---' separator row, no glossary term should be collected: {:?}",
        doc.items
    );
    let findings = check(&[doc], &default_config());
    let gi = find_by_kind(&findings, "glossary_invalid");
    assert!(
        !gi.is_empty(),
        "a glossary without a valid separator row should be glossary_invalid: {:?}",
        findings
    );
}

// --- REQ-043: "####" 系見出しの検査 ---

// @kotowari[REQ-043]
#[test]
fn req_043_five_hashes_with_space_is_unknown_heading() {
    // #### より深い見出し（5個以上の#）も直後が空白なら unknown_heading
    let content = "# Title\n\nScope.\n\n## Section\n\n##### x\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let uh = find_by_kind(&findings, "unknown_heading");
    assert!(
        uh.iter().any(|f| f.detail.contains("##### x")),
        "a 5-hash heading followed by a space should be unknown_heading: {:?}",
        uh
    );
}

// @kotowari[REQ-043]
#[test]
fn req_043_four_hashes_without_trailing_space_is_not_unknown_heading() {
    // "#" が4つ以上続いても、直後が空白でなければ unknown_heading にしない
    let content = "# Title\n\nScope.\n\n## Section\n\n####x\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let uh = find_by_kind(&findings, "unknown_heading");
    assert!(
        uh.iter().all(|f| !f.detail.contains("####x")),
        "a '####' run not followed by a space must not be unknown_heading: {:?}",
        uh
    );
}

// @kotowari[REQ-043]
#[test]
fn req_043_bare_four_hashes_is_not_unknown_heading() {
    // "####" だけの行（直後に何も無い）は unknown_heading にしない
    let content = "# Title\n\nScope.\n\n## Section\n\n####\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let uh = find_by_kind(&findings, "unknown_heading");
    assert!(
        uh.is_empty(),
        "a bare '####' line with nothing after it must not be unknown_heading: {:?}",
        uh
    );
}

// --- REQ-112: 閉じないコードブロックの前の指摘・項目は残る ---

// @kotowari[REQ-112]
#[test]
fn req_112_findings_and_items_before_the_unclosed_fence_are_retained() {
    let content = "\
# Title

Scope.

## 具体例

```gherkin
Then orphan step
```

## 要求

### REQ-001: R

```
unclosed content
";
    let doc = ir::parse_document("a.md", content);
    // フェンスが開く行より前の invalid_gherkin_line は残らなければならない
    let ig = doc
        .parse_findings
        .iter()
        .find(|f| f.kind == "invalid_gherkin_line" && f.detail == "Then orphan step");
    assert!(
        ig.is_some(),
        "a parse_finding before the unclosed fence must be retained: {:?}",
        doc.parse_findings
    );
    // unclosed_code_block 自体は出る
    assert!(
        doc.parse_findings.iter().any(|f| f.kind == "unclosed_code_block"),
        "should still produce unclosed_code_block"
    );
    // フェンスより前で組み立て済みの項目も残らなければならない
    assert!(
        doc.items.iter().any(|i| matches!(i, Item::Requirement { id, .. } if id == "REQ-001")),
        "an item completed before the unclosed fence must be retained: {:?}",
        doc.items
    );
}

// --- REQ-098: 値が空でも「知らない行」は無いものとして扱わない ---

// @kotowari[REQ-098]
#[test]
fn req_098_empty_value_of_unknown_field_still_reports_unknown_field() {
    // A157 の「値が空なら行が無いもの」は 種類・検証・定義・関係 だけに限る
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n- foo: \n\nStatement.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let uf = find_by_kind(&findings, "unknown_field");
    assert!(
        uf.iter().any(|f| f.detail.contains("foo")),
        "an empty-valued unknown field name must still be reported as unknown_field: {:?}",
        uf
    );
}

// --- REQ-044: ". " を含む行の数字接頭辞の判定 ---

// @kotowari[REQ-044]
#[test]
fn req_044_dot_space_not_preceded_by_digits_is_a_normal_statement() {
    // "Foo. Bar baz." のように ". " の前が数字でなければ、通常の文として扱う
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\nFoo. Bar baz.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ms = find_by_kind(&findings, "missing_statement");
    assert!(
        ms.is_empty(),
        "a statement containing '. ' with a non-digit prefix should remain a normal statement: {:?}",
        ms
    );
}

// @kotowari[REQ-044]
#[test]
fn req_044_line_starting_with_dot_space_is_a_normal_statement() {
    // ". leading dot text" は数字の接頭辞が無い（空の接頭辞）ので通常の文として扱う
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n\n. leading dot text\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ms = find_by_kind(&findings, "missing_statement");
    assert!(
        ms.is_empty(),
        "a line starting with '. ' (no digit prefix before it) should be a normal statement, not a list marker: {:?}",
        ms
    );
}

// --- REQ-043: 形に合わない TBL-/PROP-/FLAG- の ID ---

// @kotowari[REQ-043]
#[test]
fn req_043_malformed_tbl_id_is_not_an_item() {
    let content = "# Title\n\nScope.\n\n## 決定表\n\n### TBL-1: X\n\n- 出典: brainstorm/records.md#A1\n\n| a | b |\n|---|---|\n| 1 | 2 |\n";
    let doc = ir::parse_document("a.md", content);
    assert!(
        !doc.items.iter().any(|i| matches!(i, Item::DecisionTable { .. })),
        "malformed TBL- id must not produce Item::DecisionTable: {:?}",
        doc.items
    );
    assert!(
        doc.items.iter().any(|i| matches!(i, Item::UnknownHeading { .. })),
        "malformed TBL- id must fall back to Item::UnknownHeading: {:?}",
        doc.items
    );
    let findings = check(&[doc], &default_config());
    let uh = find_by_kind(&findings, "unknown_heading");
    assert!(
        uh.iter().any(|f| f.detail.contains("TBL-1")),
        "should report unknown_heading for a malformed TBL-1: {:?}",
        uh
    );
}

// @kotowari[REQ-043]
#[test]
fn req_043_malformed_prop_id_is_not_an_item() {
    let content = "# Title\n\nScope.\n\n## 性質\n\n### PROP-1: X\n\n- 出典: brainstorm/records.md#A1\n\nStatement.\n";
    let doc = ir::parse_document("a.md", content);
    assert!(
        !doc.items.iter().any(|i| matches!(i, Item::Property { .. })),
        "malformed PROP- id must not produce Item::Property: {:?}",
        doc.items
    );
    assert!(
        doc.items.iter().any(|i| matches!(i, Item::UnknownHeading { .. })),
        "malformed PROP- id must fall back to Item::UnknownHeading: {:?}",
        doc.items
    );
    let findings = check(&[doc], &default_config());
    let uh = find_by_kind(&findings, "unknown_heading");
    assert!(
        uh.iter().any(|f| f.detail.contains("PROP-1")),
        "should report unknown_heading for a malformed PROP-1: {:?}",
        uh
    );
}

// @kotowari[REQ-043]
#[test]
fn req_043_malformed_flag_id_is_not_an_item() {
    let content = "# 問題の記録\n\n### FLAG-1: Issue\n\n- 種類: gap\n- 関係: REQ-999\n- 出典: brainstorm/records.md#A1\n";
    let doc = ir::parse_document("FLAGS.md", content);
    assert!(
        !doc.items.iter().any(|i| matches!(i, Item::FlagEntry { .. })),
        "malformed FLAG- id must not produce Item::FlagEntry: {:?}",
        doc.items
    );
    assert!(
        doc.items.iter().any(|i| matches!(i, Item::UnknownHeading { .. })),
        "malformed FLAG- id must fall back to Item::UnknownHeading: {:?}",
        doc.items
    );
    let findings = check(&[doc], &default_config());
    let uh = find_by_kind(&findings, "unknown_heading");
    assert!(
        uh.iter().any(|f| f.detail.contains("FLAG-1")),
        "should report unknown_heading for a malformed FLAG-1: {:?}",
        uh
    );
}

// --- TBL-011: 性質の "- 出典:" 行の読み方 ---

// @kotowari[TBL-011]
#[test]
fn tbl_011_property_source_field_populates_sources() {
    let content = "# Title\n\nScope.\n\n## 性質\n\n### PROP-001: P\n\n- 出典: brainstorm/records.md#A1\n\nStatement.\n";
    let doc = ir::parse_document("a.md", content);
    let prop = doc
        .items
        .iter()
        .find(|i| matches!(i, Item::Property { id, .. } if id == "PROP-001"));
    assert!(prop.is_some(), "should parse PROP-001: {:?}", doc.items);
    if let Item::Property { sources, .. } = prop.unwrap() {
        assert_eq!(
            sources,
            &["brainstorm/records.md#A1".to_string()],
            "a PROP's own 出典 field should be collected into sources: {:?}",
            sources
        );
    }
    let findings = check(&[doc], &default_config());
    let ms = find_by_kind(&findings, "missing_source");
    assert!(
        ms.is_empty(),
        "a PROP with a 出典 field must not produce missing_source: {:?}",
        ms
    );
}

// --- REQ-052: 結び付かないタグの行の行番号 ---

// @kotowari[REQ-052]
#[test]
fn req_052_dangling_bare_word_in_unbound_tag_line_keeps_its_line_number() {
    // "@" で始まらない語のタグ行が結び付かなくても、unknown_tag の line はタグ行自身の行にする
    let content = "# Title\n\nScope.\n\n## 具体例\n\n```gherkin\n@id=EX-001 badword\n```\n";
    let doc = ir::parse_document("a.md", content);
    let ut = doc
        .parse_findings
        .iter()
        .find(|f| f.kind == "unknown_tag" && f.detail == "badword");
    assert!(
        ut.is_some(),
        "a dangling bare word in an unbound tag line should produce unknown_tag: {:?}",
        doc.parse_findings
    );
    assert_eq!(
        ut.unwrap().line,
        Some(8),
        "the finding's line should be the tag line's own line number, not None"
    );
}

// --- REQ-114: シナリオの id に使う @id の値の形 ---

// @kotowari[REQ-114]
#[test]
fn req_114_well_formed_non_ex_id_is_not_used_as_scenario_id() {
    // @id の値が REQ- の形など、EX- 以外なら「有効な形」でも id として使わない
    let content = "\
# Title

Scope.

## 具体例

```gherkin
@id=REQ-001 @about=REQ-001 @source=brainstorm/records.md#A1
Scenario: uses a wrong-prefix but well-formed id
  Given something
```
";
    let doc = ir::parse_document("a.md", content);
    let scenario = doc.items.iter().find(|i| matches!(i, Item::Scenario { .. }));
    assert!(scenario.is_some(), "should parse the scenario: {:?}", doc.items);
    if let Item::Scenario { id, .. } = scenario.unwrap() {
        assert!(
            id.is_none(),
            "a well-formed but non-EX id must not become the scenario's id: {:?}",
            id
        );
    }
}

// @kotowari[REQ-114]
#[test]
fn req_114_malformed_id_value_is_the_actual_malformed_tag_not_a_bare_at_id() {
    // 2つの "@id" 名のタグがあるとき、報告する値は実際に形が合わない方でなければならない
    let content = "\
# Title

Scope.

## 具体例

```gherkin
@id @id=BAD1 @about=REQ-001 @source=brainstorm/records.md#A1
Scenario: two id-named tags, only the second is malformed
  Given something
```

## 要求

### REQ-001: R

- 種類: ubiquitous
- 出典: brainstorm/records.md#A1
- 検証: unit

Statement.
";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ii = find_by_kind(&findings, "invalid_id");
    assert!(
        ii.iter().any(|f| f.detail == "BAD1"),
        "the reported malformed @id value should be the actually malformed one (BAD1), not an empty bare @id: {:?}",
        ii
    );
}

// @kotowari[REQ-114]
#[test]
fn req_114_malformed_id_value_is_not_stolen_from_an_unrelated_about_tag() {
    // "@about" の値がたまたま ID の形でなくても、malformed_value は "@id" 自身の値でなければならない
    let content = "\
# Title

Scope.

## 具体例

```gherkin
@about=not-an-id-like-value @id=BAD-1 @source=brainstorm/records.md#A1
Scenario: about appears before the malformed id tag
  Given something
```

## 要求

### REQ-001: R

- 種類: ubiquitous
- 出典: brainstorm/records.md#A1
- 検証: unit

Statement.
";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ii = find_by_kind(&findings, "invalid_id");
    assert!(
        ii.iter().any(|f| f.detail == "BAD-1"),
        "the reported malformed @id value must come from the @id tag itself, not an unrelated @about value: {:?}",
        ii
    );
}

// --- REQ-059: @source タグが在るが値が使い物にならないとき ---

// @kotowari[REQ-059]
#[test]
fn req_059_source_tag_present_but_only_commas_does_not_report_missing_source() {
    let content = "\
# Title

Scope.

## 具体例

```gherkin
@id=EX-001 @about=REQ-001 @source=,
Scenario: source tag exists but has no usable value
  Given something
```

## 要求

### REQ-001: R

- 種類: ubiquitous
- 出典: brainstorm/records.md#A1
- 検証: unit

Statement.
";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ms = find_by_kind(&findings, "missing_source");
    assert!(
        ms.is_empty(),
        "a scenario with a @source tag (even an unusable comma-only value) must not produce missing_source: {:?}",
        ms
    );
}

// --- REQ-098: 問題の記録の必須の行 ---

// @kotowari[REQ-098]
#[test]
fn req_098_flag_entirely_missing_kind_line_produces_missing_field() {
    let content = "\
# 問題の記録

### FLAG-001: Issue

- 関係: REQ-999
- 出典: brainstorm/records.md#A1

Body text.
";
    let doc = ir::parse_document("FLAGS.md", content);
    let findings = check(&[doc], &default_config());
    let mf = find_by_kind(&findings, "missing_field");
    assert!(
        mf.iter().any(|f| f.detail == "種類"),
        "a FLAG entry with no 種類 line at all should produce missing_field 種類: {:?}",
        mf
    );
}

// @kotowari[REQ-098]
#[test]
fn req_098_flag_relation_line_with_only_commas_does_not_report_missing_field() {
    let content = "\
# 問題の記録

### FLAG-001: Issue

- 種類: gap
- 関係: ,
- 出典: brainstorm/records.md#A1

Body text.
";
    let doc = ir::parse_document("FLAGS.md", content);
    let findings = check(&[doc], &default_config());
    let mf = find_by_kind(&findings, "missing_field");
    assert!(
        mf.iter().all(|f| f.detail != "関係"),
        "a 関係 line that exists (even if unusable) must not produce missing_field 関係: {:?}",
        mf
    );
}

// --- TBL-019: 複数の行を持つ項目での参照切れの行番号 ---

// @kotowari[TBL-019]
#[test]
fn tbl_019_unresolved_definition_reference_line_is_the_definition_fields_own_line() {
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: algorithm\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n- 定義: TBL-999\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ur = find_by_kind(&findings, "unresolved_reference");
    let f = ur.iter().find(|f| f.detail == "TBL-999");
    assert!(f.is_some(), "should produce unresolved_reference for TBL-999: {:?}", ur);
    assert_eq!(
        f.unwrap().line,
        Some(12),
        "the finding's line should be the 定義 field's own line, not an earlier field's line"
    );
}

// @kotowari[TBL-019]
#[test]
fn tbl_019_unresolved_relation_reference_line_is_the_relation_fields_own_line() {
    let content = "# 問題の記録\n\n### FLAG-001: Issue\n\n- 種類: gap\n- 関係: EX-999\n- 出典: brainstorm/records.md#A1\n";
    let doc = ir::parse_document("FLAGS.md", content);
    let findings = check(&[doc], &default_config());
    let ur = find_by_kind(&findings, "unresolved_reference");
    let f = ur.iter().find(|f| f.detail == "EX-999");
    assert!(f.is_some(), "should produce unresolved_reference for EX-999: {:?}", ur);
    assert_eq!(
        f.unwrap().line,
        Some(6),
        "the finding's line should be the 関係 field's own line, not an earlier field's line"
    );
}

// --- REQ-054: 参照の解決 ---

// @kotowari[REQ-054]
#[test]
fn req_054_valid_and_known_definition_id_produces_no_unresolved_reference() {
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: algorithm\n- 出典: brainstorm/records.md#A1\n- 検証: unit\n- 定義: TBL-001\n\n## 決定表\n\n### TBL-001: T\n\n- 出典: brainstorm/records.md#A1\n\n| a | b |\n|---|---|\n| 1 | 2 |\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let ur = find_by_kind(&findings, "unresolved_reference");
    assert!(
        ur.iter().all(|f| f.detail != "TBL-001"),
        "a well-formed, known definition id must not produce unresolved_reference: {:?}",
        ur
    );
}

// @kotowari[REQ-054]
#[test]
fn req_054_extract_backtick_contents_two_pairs_on_one_line() {
    let result = ir::extract_backtick_contents("`REQ-001` and `TBL-999`");
    assert_eq!(
        result,
        vec!["REQ-001", "TBL-999"],
        "two separate backtick-delimited ids on one line should each be extracted whole, not fused together"
    );
}

// @kotowari[REQ-033]
#[test]
#[cfg(unix)]
fn req_033_hidden_dir_and_dir_symlink_are_not_followed_at_any_depth() {
    use std::os::unix::{fs::symlink, net::UnixListener};
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("outside")).unwrap();
    std::fs::write(tmp.path().join("outside/a.md"), "bad").unwrap();
    for prefix in ["docs/ir", "docs/ir/sub/deep"] {
        let dir = tmp.path().join(prefix);
        std::fs::create_dir_all(dir.join(".hidden")).unwrap();
        symlink("missing", dir.join(".hidden/broken")).unwrap();
        symlink(tmp.path().join("outside"), dir.join("linked")).unwrap();
        std::fs::write(dir.join("a.MD"), "bad").unwrap();
        let _socket = UnixListener::bind(dir.join("socket.md")).unwrap();
        std::fs::write(dir.join("visible.md"), "# Title\n\nScope.\n").unwrap();
        symlink("visible.md", dir.join("file.md")).unwrap();
    }
    let (docs, findings) = ir::load_and_check(tmp.path(), &default_config()).unwrap();
    assert_eq!(docs.len(), 4);
    assert!(findings.is_empty(), "{findings:?}");
}

// @kotowari[REQ-033, REQ-018, TBL-020]
#[test]
#[cfg(unix)]
fn req_033_broken_symlink_in_a_subdirectory_stops() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/ir/sub")).unwrap();
    std::os::unix::fs::symlink("missing", tmp.path().join("docs/ir/sub/broken.md")).unwrap();
    let err = ir::load_and_check(tmp.path(), &default_config()).unwrap_err();
    assert!(matches!(err, kotowari::StopReason::UnreadableFile(ref detail)
        if detail.starts_with("docs/ir/sub/broken.md: ")));
}

// @kotowari[REQ-033, REQ-036, REQ-117]
#[test]
fn req_033_context_and_flags_in_a_subdirectory_are_glossary_and_flags() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/ir/sub")).unwrap();
    for file in ["CONTEXT.md", "FLAGS.md"] {
        std::fs::write(tmp.path().join("docs/ir/sub").join(file), "# Title\n").unwrap();
    }
    let (docs, findings) = ir::load_and_check(tmp.path(), &default_config()).unwrap();
    assert_eq!(docs.iter().map(|d| d.kind).collect::<Vec<_>>(),
        [ir::DocKind::Glossary, ir::DocKind::Flags]);
    assert!(find_by_kind(&findings, "missing_scope").is_empty());
    assert_eq!(find_by_kind(&findings, "glossary_invalid").len(), 1);
}

// @kotowari[TBL-008, REQ-034, REQ-036, REQ-117]
#[test]
fn tbl_008_whole_document_detail_is_the_bare_filename_in_a_subdirectory() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/ir/sub")).unwrap();
    for file in ["CONTEXT.md", "a.md"] {
        std::fs::write(tmp.path().join("docs/ir/sub").join(file), "").unwrap();
    }
    let (_, findings) = ir::load_and_check(tmp.path(), &default_config()).unwrap();
    for (kind, expected) in [("missing_title", vec!["CONTEXT.md", "a.md"]),
        ("missing_scope", vec!["a.md"]), ("glossary_invalid", vec!["CONTEXT.md"])] {
        assert_eq!(find_by_kind(&findings, kind).iter().map(|f| f.detail.as_str()).collect::<Vec<_>>(), expected);
    }
}

// @kotowari[REQ-032]
#[test]
fn req_032_first_occurrence_is_bytewise_first_relative_path() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/ir/a")).unwrap();
    for file in ["a.md", "a/b.md"] {
        std::fs::write(tmp.path().join("docs/ir").join(file), "# Title\n\nScope.\n\n### REQ-001: Name\n").unwrap();
    }
    let (_, findings) = ir::load_and_check(tmp.path(), &default_config()).unwrap();
    let duplicates = find_by_kind(&findings, "duplicate_id");
    assert_eq!(duplicates.len(), 1);
    assert_eq!(duplicates[0].path, "docs/ir/a/b.md");
    assert_eq!(duplicates[0].line, Some(5));
}

// --- REQ-055, REQ-056: 見ないもの ---

/// CLI を通して検査するプロジェクトを一時ディレクトリに作る
fn make_cli_project(tmp: &std::path::Path) {
    for dir in [".kotowari", "docs/ir", "docs/decision/records", "docs/decision/adr"] {
        std::fs::create_dir_all(tmp.join(dir)).unwrap();
    }
    std::fs::write(
        tmp.join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\n",
    )
    .unwrap();
    std::fs::write(
        tmp.join("docs/decision/records/records.md"),
        "# Records\n\n## Agreements\n\n- A1 Agreement\n",
    )
    .unwrap();
}

/// CLI を走らせて JSON を返す
fn run_cli(tmp: &std::path::Path) -> serde_json::Value {
    let output = assert_cmd::Command::cargo_bin("kotowari")
        .unwrap()
        .arg("check")
        .current_dir(tmp)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(&stdout).expect("valid JSON")
}

/// 指定した文書の指定した行を指す指摘を集める
fn findings_on_line(v: &serde_json::Value, path: &str, line: u64) -> Vec<serde_json::Value> {
    v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["path"] == path && f["line"] == line)
        .cloned()
        .collect()
}

// @kotowari[REQ-055]
#[test]
fn req_055_non_ears_statement_gets_no_finding_on_its_line() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    // 13行目の文は「常に」も「とき」も持たない平叙文
    std::fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: unit\n\nこの道具は文書を読む。\n",
    )
    .unwrap();
    let v = run_cli(tmp.path());
    assert!(
        findings_on_line(&v, "docs/ir/a.md", 13).is_empty(),
        "the statement line should get no finding: {v}"
    );
    // 検査そのものは動いている（要求の見出しの行にはテストのない要求の誤りが出る）
    assert!(
        !findings_on_line(&v, "docs/ir/a.md", 7).is_empty(),
        "the requirement heading should still be checked: {v}"
    );
}

// @kotowari[REQ-056]
#[test]
fn req_056_contradiction_flag_with_one_reading_gets_no_finding() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    std::fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: unit\n\nこの道具は文書を読む。\n",
    )
    .unwrap();
    // 種類 contradiction の問題の記録に、読みを1つだけ書く
    std::fs::write(
        tmp.path().join("docs/ir/FLAGS.md"),
        "# 問題の記録\n\n### FLAG-001: 読みが割れる\n\n- 種類: contradiction\n- 関係: REQ-001\n- 出典: docs/decision/records/records.md#A1\n\n読みは1つだけ書いてある。\n",
    )
    .unwrap();
    let v = run_cli(tmp.path());
    let on_flags: Vec<_> = v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["path"] == "docs/ir/FLAGS.md")
        .collect();
    assert!(
        on_flags.is_empty(),
        "the contradiction entry should get no finding: {on_flags:?}"
    );
    // 検査そのものは動いている
    assert!(
        !findings_on_line(&v, "docs/ir/a.md", 7).is_empty(),
        "the requirement heading should still be checked: {v}"
    );
}

// --- ir-document.md の具体例 ---

// @kotowari[REQ-037, EX-007]
#[test]
fn req_037_two_lines_separated_by_crlf_count_as_two() {
    let doc = ir::parse_document("a.md", "a\r\nb");
    assert_eq!(doc.line_count, 2, "CRLF is one line break, not two");
}

// @kotowari[REQ-036, EX-006]
#[test]
fn req_036_glossary_with_only_a_title_and_a_table_has_no_missing_scope() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    std::fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n",
    )
    .unwrap();
    let v = run_cli(tmp.path());
    assert!(
        find_kind_in_json(&v, "missing_scope").is_empty(),
        "a glossary needs no scope line: {v}"
    );
}

// @kotowari[REQ-033, EX-021]
#[test]
fn req_033_context_and_flags_under_a_subdirectory_have_no_missing_scope() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    std::fs::create_dir_all(tmp.path().join("docs/ir/network")).unwrap();
    std::fs::write(
        tmp.path().join("docs/ir/network/CONTEXT.md"),
        "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n",
    )
    .unwrap();
    std::fs::write(tmp.path().join("docs/ir/network/FLAGS.md"), "# 問題の記録\n").unwrap();
    let v = run_cli(tmp.path());
    assert!(
        find_kind_in_json(&v, "missing_scope").is_empty(),
        "CONTEXT.md and FLAGS.md under a subdirectory are a glossary and a flags document: {v}"
    );
}

// @kotowari[REQ-033, EX-020]
#[test]
fn req_033_a_document_deep_in_the_tree_is_read_and_an_empty_directory_is_silent() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    std::fs::create_dir_all(tmp.path().join("docs/ir/network/dns")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/ir/network/empty")).unwrap();
    std::fs::write(
        tmp.path().join("docs/ir/network/dns/timeout.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: unit\n\nStatement.\n",
    )
    .unwrap();
    let v = run_cli(tmp.path());
    let missing = find_kind_in_json(&v, "requirement_without_test");
    assert_eq!(missing.len(), 1, "the deep document is read: {v}");
    assert_eq!(missing[0]["path"], "docs/ir/network/dns/timeout.md");
    assert!(
        v["findings"].as_array().unwrap().iter().all(|f| {
            !f["path"].as_str().unwrap().starts_with("docs/ir/network/empty")
        }),
        "an empty directory gets no finding: {v}"
    );
}

// @kotowari[REQ-033, EX-030]
#[test]
#[cfg(unix)]
fn req_033_hidden_directory_and_directory_symlink_deep_in_the_tree_are_not_read() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    std::fs::create_dir_all(tmp.path().join("docs/ir/network/.draft")).unwrap();
    // 読まれれば題名も範囲も無い文書として指摘が出る中身
    std::fs::write(tmp.path().join("docs/ir/network/.draft/a.md"), "bad").unwrap();
    std::os::unix::fs::symlink(
        tmp.path().join("docs/ir"),
        tmp.path().join("docs/ir/network/link"),
    )
    .unwrap();
    let output = assert_cmd::Command::cargo_bin("kotowari")
        .unwrap()
        .arg("check")
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "neither a finding nor a stop: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let v: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid JSON");
    assert_eq!(v["files"], 0, "no document under either place is read: {v}");
    assert!(v["findings"].as_array().unwrap().is_empty(), "{v}");
}

/// JSON の findings から種類で絞る
fn find_kind_in_json(v: &serde_json::Value, kind: &str) -> Vec<serde_json::Value> {
    v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["kind"] == kind)
        .cloned()
        .collect()
}

// --- ir-references.md の具体例 ---

// @kotowari[REQ-052, EX-009]
#[test]
fn req_052_retired_tag_on_a_scenario_is_unknown_tag() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    std::fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: review\n\nStatement.\n\n## 具体例\n\n```gherkin\n@id=EX-001 @about=REQ-001 @source=docs/decision/records/records.md#A1 @requirement=REQ-001\nScenario: Test\n  Given a\n  When b\n  Then c\n```\n",
    )
    .unwrap();
    let v = run_cli(tmp.path());
    let unknown = find_kind_in_json(&v, "unknown_tag");
    assert_eq!(unknown.len(), 1, "the retired @requirement tag is unknown: {v}");
    assert_eq!(unknown[0]["detail"], "@requirement");
}

// @kotowari[REQ-054, EX-010]
#[test]
fn req_054_scenario_about_an_unknown_requirement_is_unresolved() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    std::fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 具体例\n\n```gherkin\n@id=EX-001 @about=REQ-999 @source=docs/decision/records/records.md#A1\nScenario: Test\n  Given a\n  When b\n  Then c\n```\n",
    )
    .unwrap();
    let v = run_cli(tmp.path());
    let unresolved = find_kind_in_json(&v, "unresolved_reference");
    assert_eq!(unresolved.len(), 1, "@about points nowhere: {v}");
    assert_eq!(unresolved[0]["detail"], "REQ-999");
}

// --- findings.md と ir-items.md の具体例 ---

// @kotowari[REQ-032, EX-005]
#[test]
fn req_032_three_places_yield_two_duplicates_and_none_on_the_first() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    let requirement = "### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: review\n\nStatement.\n\n";
    std::fs::write(
        tmp.path().join("docs/ir/a.md"),
        format!("# Title\n\nScope.\n\n## 要求\n\n{requirement}{requirement}{requirement}"),
    )
    .unwrap();
    let v = run_cli(tmp.path());
    let duplicates = find_kind_in_json(&v, "duplicate_id");
    assert_eq!(duplicates.len(), 2, "three places yield two duplicates: {v}");
    assert!(duplicates.iter().all(|f| f["detail"] == "REQ-001"));
    // 1つ目の見出しは 7 行目
    assert!(
        duplicates.iter().all(|f| f["line"] != 7),
        "the first heading gets no duplicate_id: {v}"
    );
}

// @kotowari[REQ-044, EX-008]
#[test]
fn req_044_priority_line_under_a_requirement_is_an_unknown_field() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    make_cli_project(tmp.path());
    std::fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: review\n- 優先度: 高\n\nStatement.\n",
    )
    .unwrap();
    let v = run_cli(tmp.path());
    let unknown = find_kind_in_json(&v, "unknown_field");
    assert_eq!(unknown.len(), 1, "the 優先度 line is not a known field: {v}");
    assert_eq!(unknown[0]["detail"], "- 優先度: 高");
}
