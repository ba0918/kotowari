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

// --- REQ-033: 直下の *.md だけ読む ---

// @kotowari[REQ-033]
#[test]
fn req_033_only_direct_children() {
    use assert_cmd::Command;
    use tempfile::TempDir;

    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/ir/sub")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/decision/brainstorm")).unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/decision/adr")).unwrap();
    std::fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\n",
    )
    .unwrap();
    // 直下に valid な文書
    std::fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope line.\n",
    )
    .unwrap();
    // サブディレクトリに文書（読まれない）
    std::fs::write(
        tmp.path().join("docs/ir/sub/b.md"),
        "no title here\n",
    )
    .unwrap();

    let output = Command::cargo_bin("kotowari")
        .unwrap()
        .arg("check")
        .current_dir(tmp.path())
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let v: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    // 直下の1文書だけ読む
    assert_eq!(v["files"], 1);
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
fn req_038_too_many_lines_is_a_warning() {
    // 既定の limits.lines は 120
    let content = format!("# Title\n\nScope.\n\n{}", "line\n".repeat(120));
    let doc = ir::parse_document("a.md", &content);
    assert!(doc.line_count > 120);
    let findings = check(&[doc], &default_config());
    let tl = find_by_kind(&findings, "too_many_lines");
    assert_eq!(tl.len(), 1);
    assert_eq!(tl[0].severity, "warning");
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
    assert_eq!(tr[0].severity, "warning");

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
    assert_eq!(uh[0].detail, "Bad Heading");
}

// @kotowari[REQ-043, TBL-008]
#[test]
fn req_043_unknown_heading_detail_is_full_heading_text() {
    // コロン付きの認識できない見出し → detail は見出しの全文
    let content = "# Title\n\nScope.\n\n## 要求\n\n### EX-001: Example\n\nSome text.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let uh = find_by_kind(&findings, "unknown_heading");
    assert_eq!(uh.len(), 1);
    assert_eq!(
        uh[0].detail, "EX-001: Example",
        "detail should be the full heading text"
    );
}

// @kotowari[REQ-043, TBL-008]
#[test]
fn req_043_unknown_heading_invalid_id_detail_is_full_heading_text() {
    // 認識できる prefix だが ID 形式が不正（3桁でない） → detail は見出しの全文
    let content = "# Title\n\nScope.\n\n## 要求\n\n### REQ-1: Invalid\n\nSome text.\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let uh = find_by_kind(&findings, "unknown_heading");
    assert_eq!(uh.len(), 1);
    assert_eq!(
        uh[0].detail, "REQ-1: Invalid",
        "detail should be the full heading text, not just the ID"
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
    assert_eq!(uh[0].detail, "REQ-001");
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
    // 決定表に出典がない
    let content = "# Title\n\nScope.\n\n## 決定表\n\n### TBL-001: T\n\n| A |\n|---|\n| 1 |\n";
    let doc = ir::parse_document("a.md", content);
    let findings = check(&[doc], &default_config());
    let mf = find_by_kind(&findings, "missing_field");
    assert!(mf.iter().any(|f| f.detail == "出典"), "missing source field");
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

    // (3) 出典の行がない → missing_field "出典" が出て、missing_source は出ない
    let no_source = "# Title\n\nScope.\n\n## 要求\n\n### REQ-003: NoSource\n\n- 種類: ubiquitous\n- 検証: unit\n\nStatement.\n";
    let doc3 = ir::parse_document("a.md", no_source);
    let f3 = check(&[doc3], &default_config());
    let mf3 = find_by_kind(&f3, "missing_field");
    assert!(mf3.iter().any(|x| x.detail == "出典"), "should report missing_field 出典: {:?}", mf3);
    let ms3 = find_by_kind(&f3, "missing_source");
    assert!(ms3.is_empty(), "missing_source should not appear when 出典 line is absent: {:?}", ms3);

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
