use super::*;
use crate::document::Document;
use crate::schema::parse_schema;
use serde_json::json;

fn values(schema_yaml: &str, doc: &str) -> serde_json::Value {
    let schema = parse_schema(schema_yaml).unwrap();
    let document = Document::parse(doc).unwrap();
    extract_values(&schema, &document.read_as(schema.reading))
}

const ADR: &str = r#"
name: adr
document:
  title:
    pattern: "^ADR-\\d{4}:"
    extract: title
  preamble:
    fields:
      - name: ID
        extract: id
      - name: 状態
        extract: status
      - name: 日付
        extract: date
  sections:
    - name: 状況
      extract: sections.context
      statement:
        required: false
    - name: 決定
      extract: sections.decision
      statement:
        required: false
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: sections.reasons
"#;

fn adr_doc() -> &'static str {
    "# ADR-0001: 印の話\n\n- ID: ADR-0001\n- 状態: 承認済み\n- 日付: 2026-09-16\n\n## 状況\n\n背景。\n\n## 決定\n\n判断。\n\n## 理由\n\n- 理由1\n- 理由2\n"
}

// @kotowari[REQ-schema-036]
#[test]
fn adr_values_are_nested_by_path() {
    let v = values(ADR, adr_doc());
    assert_eq!(v["id"], "ADR-0001");
    assert_eq!(v["title"], "ADR-0001: 印の話");
    assert_eq!(v["status"], "承認済み");
    assert_eq!(v["date"], "2026-09-16");
    assert_eq!(v["sections"]["context"], "背景。");
    assert_eq!(v["sections"]["decision"], "判断。");
    assert_eq!(v["sections"]["reasons"], json!(["- 理由1", "- 理由2"]));
}

// @kotowari[REQ-schema-028, REQ-schema-035]
#[test]
fn undeclared_field_name_line_is_extracted_as_a_bullet() {
    let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
    let doc =
        "## 理由\n\n- 判断の記録かどうかの見分け（A134: 決定の節の見出しを1つ以上持つファイル）\n";
    let v = values(schema, doc);
    assert_eq!(
        v["reasons"],
        json!(["- 判断の記録かどうかの見分け（A134: 決定の節の見出しを1つ以上持つファイル）"]),
        "未宣言の名前の `- 名前: 値` 行は箇条書きとして抽出する（TBL-schema-007、TBL-schema-008）"
    );
}

// @kotowari[REQ-schema-028, REQ-schema-035]
#[test]
fn section_body_includes_undeclared_field_name_line_as_a_bullet() {
    let schema = r#"
document:
  sections:
    - name: 状況
      extract: sections.body
      statement:
        required: false
      bullets:
        repeat: { min: 0 }
"#;
    let doc =
        "## 状況\n\n- 判断の記録かどうかの見分け（A134: 決定の節の見出しを1つ以上持つファイル）\n";
    let v = values(schema, doc);
    assert_eq!(
        v["sections"]["body"],
        "- 判断の記録かどうかの見分け（A134: 決定の節の見出しを1つ以上持つファイル）",
        "節の本文はフィールド行を含めず、箇条書きとして含める（TBL-schema-008）"
    );
}

// @kotowari[REQ-schema-031, REQ-schema-063]
#[test]
fn a_child_line_with_an_undeclared_name_stays_in_the_parent_bullet_value() {
    let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
    let doc = "## 理由\n\n- 親\n  - 子: 値\n";
    let v = values(schema, doc);
    assert_eq!(
        v["reasons"],
        json!(["- 親\n  - 子: 値"]),
        "宣言していない名前の子の行は子の箇条書きとして親の値に入る"
    );
}

// @kotowari[REQ-schema-035]
#[test]
fn undeclared_field_name_line_extract_preserves_original_marker() {
    let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
    let doc = "## 理由\n\n* 判断の記録（A134: 決定の節）\n";
    let v = values(schema, doc);
    assert_eq!(
        v["reasons"],
        json!(["* 判断の記録（A134: 決定の節）"]),
        "未宣言の名前の `* 名前: 値` 行は元のマーカーを保って箇条書きとして抽出する（TBL-schema-008）"
    );
}

// @kotowari[REQ-schema-035]
#[test]
fn section_body_includes_undeclared_field_name_line_with_original_marker() {
    let schema = r#"
document:
  sections:
    - name: 状況
      extract: sections.body
      statement:
        required: false
      bullets:
        repeat: { min: 0 }
"#;
    let doc = "## 状況\n\n* 判断の記録（A134: 決定の節）\n";
    let v = values(schema, doc);
    assert_eq!(
        v["sections"]["body"], "* 判断の記録（A134: 決定の節）",
        "節の本文の箇条書き要素は元のマーカーを保つ（TBL-schema-008）"
    );
}

// @kotowari[REQ-schema-035]
#[test]
fn title_named_group_capture_is_extracted() {
    let schema = r#"
document:
  title:
    pattern: "^ADR-(?<id>\\d{4}):"
    extract: { path: id, group: id }
"#;
    let v = values(schema, "# ADR-0042: 話\n");
    assert_eq!(v["id"], "0042");
}

// @kotowari[REQ-schema-019, REQ-schema-038]
#[test]
fn repeated_node_is_an_array_when_present_and_omits_key_when_absent() {
    let schema = r#"
document:
  preamble:
    fields:
      - name: タグ
        repeat: { min: 0, max: 1 }
        extract: tags
"#;
    let present = values(schema, "# 題名\n\n- タグ: a\n");
    assert_eq!(present["tags"], json!(["a"]));
    let absent = values(schema, "# 題名\n");
    assert!(absent.get("tags").is_none(), "0件のときはキーを省略する");
}

// @kotowari[REQ-schema-019, REQ-schema-038]
#[test]
fn non_repeated_node_is_single_and_omits_key_when_missing() {
    let schema = r#"
document:
  preamble:
    fields:
      - name: 状態
        extract: status
"#;
    let present = values(schema, "# 題名\n\n- 状態: 承認済み\n");
    assert_eq!(present["status"], "承認済み");
    let absent = values(schema, "# 題名\n");
    assert!(absent.get("status").is_none());
}

// @kotowari[REQ-schema-038]
#[test]
fn non_repeated_bullets_omit_key_when_missing() {
    let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        extract: reasons
"#;
    let v = values(schema, "## 理由\n");
    assert!(v.get("reasons").is_none());
}

// @kotowari[REQ-schema-038]
#[test]
fn repeated_bullets_omit_key_when_missing() {
    let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
    let v = values(schema, "## 理由\n");
    assert!(
        v.get("reasons").is_none(),
        "箇条書き0件のときはキーを省略する"
    );
}

// @kotowari[REQ-schema-029, REQ-schema-035, REQ-schema-037, REQ-schema-045]
#[test]
fn separator_trims_elements_and_keeps_empty_ones() {
    let schema = r#"
document:
  preamble:
    fields:
      - name: タグ
        separator: ","
        extract: tags
"#;
    let v = values(schema, "# 題名\n\n- タグ: a, b,,c,\n");
    assert_eq!(v["tags"], json!(["a", "b", "", "c", ""]));
}

// @kotowari[REQ-schema-019, REQ-schema-035, REQ-schema-045]
#[test]
fn repeated_field_with_separator_extracts_array_of_arrays() {
    let schema = r#"
document:
  preamble:
    fields:
      - name: タグ
        repeat: { min: 0 }
        separator: ","
        extract: tags
"#;
    let v = values(schema, "# 題名\n\n- タグ: a,b\n- タグ: c\n");
    assert_eq!(v["tags"], json!([["a", "b"], ["c"]]));
}

// @kotowari[REQ-schema-030, REQ-schema-035, REQ-schema-046, REQ-schema-063]
#[test]
fn separator_split_excludes_continuation_paragraphs() {
    let schema = r#"
document:
  preamble:
    fields:
      - name: タグ
        separator: ","
        extract: tags
"#;
    let doc = "# 題名\n\n- タグ: a,b\n\n  継続,の段落\n";
    let v = values(schema, doc);
    assert_eq!(
        v["tags"],
        json!(["a", "b\n\n  継続,の段落"]),
        "継続段落は値の分割に含めず、末尾の要素に改行で付ける（REQ-schema-046）"
    );
}

// @kotowari[REQ-schema-063]
#[test]
fn a_crlf_document_gives_the_same_values_as_the_lf_document() {
    // CRLF も1つの行区切り。値の行は元の行を改行1つでつなぐので、改行の形に依らず
    // LF の文書と同じ値になる。この規則は読み方に依らない（REQ-schema-063）
    let schema = "document:\n  sections:\n    - name: 理由\n      statement:\n        repeat: { min: 0 }\n        extract: lines\n      fields:\n        - name: タグ\n          required: false\n          separator: \",\"\n          extract: tags\n      bullets:\n        repeat: { min: 0 }\n        extract: reasons\n        children:\n          bullets:\n            repeat: { min: 0 }\n";
    let doc = "# 題名\n\n## 理由\n\n一行目\n二行目\n\n- 親\n  - 子\n- 続く\n\n  継続の段落\n- タグ: a,b\n折り返し,c\n";
    for reading in ["paragraph", "line"] {
        let schema = format!("reading: {reading}\n{schema}");
        let lf = values(&schema, doc);
        assert_eq!(lf["reasons"][0], json!("- 親\n  - 子"), "{reading}: {lf}");
        let crlf = values(&schema, &doc.replace('\n', "\r\n"));
        assert_eq!(crlf, lf, "{reading}");
    }
}

// @kotowari[REQ-schema-029, REQ-schema-035]
#[test]
fn wrapped_line_is_part_of_value_and_subject_to_separator_split() {
    let schema = r#"
document:
  preamble:
    fields:
      - name: タグ
        separator: ","
        extract: tags
"#;
    let doc = "# 題名\n\n- タグ: a,b\n折り返し,c\n";
    let v = values(schema, doc);
    assert_eq!(
        v["tags"],
        json!(["a", "b\n折り返し", "c"]),
        "空行なしの折り返し行は値の一部で、separator の分割対象になる（REQ-schema-045、REQ-schema-046）。継続段落のように末尾要素に付かない"
    );
}

// @kotowari[REQ-schema-032, REQ-schema-035]
#[test]
fn paragraph_after_code_block_lead_is_extracted_as_a_statement() {
    let schema = r#"
document:
  sections:
    - name: 状況
      statement:
        required: false
        extract: note
      codeblock:
        required: false
        lang: python
      bullets:
        repeat: { min: 0 }
"#;
    let doc = "## 状況\n\n- ```python\n  x = 1\n  ```\n\n  後続の段落\n";
    let v = values(schema, doc);
    assert_eq!(
        v["note"], "後続の段落",
        "先頭がコードブロックのリスト項目の後続段落は文として抽出する"
    );
}

// @kotowari[REQ-schema-035, REQ-schema-063]
#[test]
fn section_body_includes_statements_and_bullets_only() {
    let schema = r#"
document:
  sections:
    - name: 状況
      extract: sections.body
      statement:
        required: false
      bullets:
        repeat: { min: 0 }
      table:
        required: false
        header: [a]
      codeblock:
        required: false
        lang: gherkin
"#;
    let doc = "## 状況\n\n背景。\n\n- 箇条1\n- 箇条2\n\n| a |\n|---|\n| x |\n\n```gherkin\nScenario: 例\n```\n";
    let v = values(schema, doc);
    // 部分の間は元の文書の空行を残し、表とコードブロックは抜く（REQ-schema-063）
    assert_eq!(v["sections"]["body"], "背景。\n\n- 箇条1\n- 箇条2");
}

// @kotowari[REQ-schema-030, REQ-schema-035]
#[test]
fn section_body_includes_continuation_paragraphs_and_nested_bullets() {
    let schema = r#"
document:
  sections:
    - name: 状況
      extract: sections.body
      statement:
        required: false
      bullets:
        repeat: { min: 0 }
"#;
    let doc = "## 状況\n\n- 理由1\n\n  続きの段落\n  - 入れ子\n";
    let v = values(schema, doc);
    // 子の箇条書きの行は、そのままのインデントで親の抽出要素に含める（REQ-schema-031、TBL-schema-008）
    assert_eq!(v["sections"]["body"], "- 理由1\n\n  続きの段落\n  - 入れ子");
}

// @kotowari[REQ-schema-028, REQ-schema-035]
#[test]
fn bullet_extract_preserves_original_markers() {
    let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
    let doc = "## 理由\n\n* 星\n\n+ プラス\n\n- ハイフン\n";
    let v = values(schema, doc);
    assert_eq!(
        v["reasons"],
        json!(["* 星", "+ プラス", "- ハイフン"]),
        "箇条書きの抽出は元のマーカーを保つ（TBL-schema-008）"
    );
}

// @kotowari[REQ-schema-030, REQ-schema-035]
#[test]
fn bullet_extract_preserves_marker_with_continuation() {
    let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
    let doc = "## 理由\n\n* 親\n\n  続きの段落\n";
    let v = values(schema, doc);
    assert_eq!(
        v["reasons"],
        json!(["* 親\n\n  続きの段落"]),
        "継続段落を付けるときも元のマーカーを保つ（TBL-schema-008）"
    );
}

// @kotowari[REQ-schema-030, REQ-schema-035]
#[test]
fn bullet_extract_preserves_whitespace_after_marker_with_continuation() {
    let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
    let doc = "## 理由\n\n-  親\n\n    続きの段落\n";
    let v = values(schema, doc);
    assert_eq!(
        v["reasons"],
        json!(["-  親\n\n    続きの段落"]),
        "継続段落とつなぐときも元の行のマーカー直後の空白を保つ（TBL-schema-008）"
    );
}

// @kotowari[REQ-schema-035, REQ-schema-063]
#[test]
fn bullet_extract_includes_lead_paragraph_on_a_separate_line_from_the_marker() {
    let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
    let doc = "## 理由\n\n- \n  親\n- 次\n";
    let v = values(schema, doc);
    assert_eq!(
        v["reasons"],
        json!(["- \n  親", "- 次"]),
        "マーカー行と別の行にある lead 段落の内容を抽出要素が欠落させない（TBL-schema-008）"
    );
}

// @kotowari[REQ-schema-035]
#[test]
fn bullet_extract_keeps_trailing_whitespace_of_the_marker_line() {
    let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
    let doc = "## 理由\n\n- 親  \n続き\n";
    let v = values(schema, doc);
    assert_eq!(
        v["reasons"],
        json!(["- 親  \n続き"]),
        "ハード改行の末尾空白がマーカー行から剥がれない（TBL-schema-008）"
    );
}

// @kotowari[REQ-schema-035]
#[test]
fn undeclared_field_name_line_extract_preserves_whitespace_after_marker() {
    let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
    let doc = "## 理由\n\n-  判断の記録（A134: 決定の節）\n";
    let v = values(schema, doc);
    assert_eq!(
        v["reasons"],
        json!(["-  判断の記録（A134: 決定の節）"]),
        "未宣言の名前の行を箇条書きとして抽出するときも元の行を使う（TBL-schema-007、TBL-schema-008）"
    );
}

// @kotowari[REQ-schema-030, REQ-schema-035, REQ-schema-063]
#[test]
fn bullet_extract_includes_marker_and_continuation() {
    let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
    let doc = "## 理由\n\n- 親\n\n  続きの段落\n\n- 次\n";
    let v = values(schema, doc);
    assert_eq!(v["reasons"], json!(["- 親\n\n  続きの段落", "- 次"]));
}

// @kotowari[REQ-schema-030, REQ-schema-035, REQ-schema-063]
#[test]
fn bullet_with_multiple_continuations_keeps_the_blank_lines_between_them() {
    let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
    let doc = "## 理由\n\n- 親\n\n  続き1\n\n  続き2\n";
    let v = values(schema, doc);
    assert_eq!(v["reasons"], json!(["- 親\n\n  続き1\n\n  続き2"]));
}

// @kotowari[REQ-schema-028, REQ-schema-035]
#[test]
fn ordered_list_is_not_extracted_as_a_bullet() {
    let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: reasons
"#;
    let doc = "## 理由\n\n1. 順序付き\n\n- 箇条書き\n";
    let v = values(schema, doc);
    assert_eq!(
        v["reasons"],
        json!(["- 箇条書き"]),
        "順序付きリストは箇条書きの対象外で抽出に含めない（TBL-schema-007）"
    );
}

// @kotowari[REQ-schema-030, REQ-schema-035, REQ-schema-046, REQ-schema-063]
#[test]
fn field_extract_includes_continuation_paragraph() {
    let schema = r#"
document:
  preamble:
    fields:
      - name: 状態
        extract: status
"#;
    let doc = "# 題名\n\n- 状態: 承認済み\n\n  継続の段落\n";
    let v = values(schema, doc);
    assert_eq!(
        v["status"], "承認済み\n\n  継続の段落",
        "フィールド行の継続段落は元の空行と字下げのまま値に付く（REQ-schema-063）"
    );
}

// @kotowari[REQ-schema-030, REQ-schema-035, REQ-schema-063]
#[test]
fn item_body_field_line_includes_continuation_paragraph() {
    let schema = r#"
document:
  sections:
    - name: 要求
      item:
        repeat: { min: 0 }
        extract: items
        fields:
          - name: 種類
        statement:
          required: false
"#;
    let doc = "## 要求\n\n### REQ-001: 名前\n\n- 種類: algorithm\n\n  継続の説明\n";
    let v = values(schema, doc);
    // 継続段落はフィールド行の一部なので、項目の本文のフィールド行にも含める（REQ-schema-030）
    assert_eq!(
        v["items"],
        json!(["REQ-001: 名前\n- 種類: algorithm\n\n  継続の説明"])
    );
}

// @kotowari[REQ-schema-035, REQ-schema-063]
#[test]
fn item_extract_is_heading_plus_body() {
    let schema = r#"
document:
  sections:
    - name: 要求
      item:
        repeat: { min: 0 }
        extract: items
        fields:
          - name: 種類
        statement:
          required: false
"#;
    let doc = "## 要求\n\n### REQ-001: 名前\n\n- 種類: algorithm\n\n本文。\n";
    let v = values(schema, doc);
    // 見出しと本文は改行でつなぎ、本文の部分の間は元の文書の空行を残す（REQ-schema-063）
    assert_eq!(
        v["items"],
        json!(["REQ-001: 名前\n- 種類: algorithm\n\n本文。"])
    );
}

// @kotowari[REQ-schema-033, REQ-schema-035]
#[test]
fn table_extract_concatenates_multiple_tables_in_document_order() {
    let schema = r#"
document:
  sections:
    - name: 用語集
      table:
        required: false
        header: [a, b]
        extract: glossary
"#;
    let doc = "## 用語集\n\n| a | b |\n|---|---|\n| 1 | 2 |\n\n| a | b |\n|---|---|\n| 3 | 4 |\n";
    let v = values(schema, doc);
    assert_eq!(
        v["glossary"],
        json!([{ "a": "1", "b": "2" }, { "a": "3", "b": "4" }])
    );
}

// @kotowari[REQ-schema-033, REQ-schema-035]
#[test]
fn repeated_table_extracts_flat_row_objects_without_nesting() {
    let schema = r#"
document:
  sections:
    - name: 用語集
      table:
        repeat: { min: 0 }
        header: [a, b]
        extract: glossary
"#;
    let doc = "## 用語集\n\n| a | b |\n|---|---|\n| 1 | 2 |\n";
    let v = values(schema, doc);
    assert_eq!(v["glossary"], json!([{ "a": "1", "b": "2" }]));
}

// @kotowari[REQ-schema-035, REQ-schema-048]
#[test]
fn statement_with_a_derived_value_extracts_one_object_per_line() {
    let schema = r#"
document:
  sections:
    - name: 記録
      statement:
        extract:
          path: lines
          value: text
          of: { line: line }
"#;
    let doc = "## 記録\n\n 文の1行目\n  字下げの2行目\n   3行目\n";
    let v = values(schema, doc);
    assert_eq!(
        v["lines"],
        json!([
            { "text": "文の1行目", "line": 3 },
            { "text": "字下げの2行目", "line": 4 },
            { "text": "3行目", "line": 5 }
        ]),
        "行が要素の単位になり（TBL-schema-008）、行の数と同じ数のオブジェクトが前後の空白を取り除いた値と行番号を持つ"
    );
}

// @kotowari[REQ-schema-035, REQ-schema-048]
#[test]
fn statement_with_a_derived_value_lists_paragraphs_separated_by_a_blank_line() {
    let schema = r#"
document:
  sections:
    - name: 記録
      statement:
        repeat: { min: 0 }
        extract:
          path: lines
          value: text
          of: { line: line }
"#;
    let doc = "## 記録\n\n1行目\n\n次の段落\n";
    let v = values(schema, doc);
    assert_eq!(
        v["lines"],
        json!([
            { "text": "1行目", "line": 3 },
            { "text": "次の段落", "line": 5 }
        ]),
        "空行を挟んで続く段落も同じ並びに入る（TBL-schema-008）"
    );
}

// @kotowari[REQ-schema-035, REQ-schema-048]
#[test]
fn statement_element_value_is_trimmed_and_raw_keeps_the_original_line() {
    let schema = r#"
document:
  sections:
    - name: 記録
      statement:
        extract:
          path: lines
          value: text
          of: { raw: raw }
"#;
    let doc = "## 記録\n\n  字下げのある行  \n";
    let v = values(schema, doc);
    assert_eq!(
        v["lines"],
        json!([{ "text": "字下げのある行", "raw": "  字下げのある行  " }]),
        "要素の値は前後の空白を取り除いた文字、raw は行そのままで別物（raw は REQ-schema-048）"
    );
}

// @kotowari[REQ-schema-035, REQ-schema-048]
#[test]
fn a_continuation_paragraph_is_not_a_statement_element() {
    let schema = r#"
document:
  sections:
    - name: 記録
      fields:
        - name: 状態
          extract: status
      statement:
        required: false
        extract:
          path: lines
          value: text
          of: { line: line }
"#;
    let doc = "## 記録\n\n- 状態: ok\n\n  一覧の行の継続段落。\n\n素の文。\n";
    let v = values(schema, doc);
    assert_eq!(
        v["lines"],
        json!([{ "text": "素の文。", "line": 7 }]),
        "一覧の行の継続段落は文の抽出の要素に入らない（REQ-schema-030）"
    );
}

// @kotowari[REQ-schema-033, REQ-schema-048]
#[test]
fn table_with_a_derived_value_puts_the_row_line_into_each_row_object() {
    let schema = r#"
document:
  sections:
    - name: 用語集
      table:
        header: [a, b]
        extract:
          path: glossary
          value: cells
          of: { line: line }
"#;
    let doc = "## 用語集\n\n| a | b |\n|---|---|\n| 1 | 2 |\n| 3 | 4 |\n| 5 | 6 |\n";
    let v = values(schema, doc);
    assert_eq!(
        v["glossary"],
        json!([
            { "cells": { "a": "1", "b": "2" }, "line": 5 },
            { "cells": { "a": "3", "b": "4" }, "line": 6 },
            { "cells": { "a": "5", "b": "6" }, "line": 7 }
        ]),
        "行ごとのオブジェクトが要素の値と行番号を持つ（TBL-schema-008、REQ-schema-048）"
    );
}

// @kotowari[REQ-schema-033, REQ-schema-048]
#[test]
fn repeated_table_with_a_derived_value_nests_rows_per_table() {
    let schema = r#"
document:
  sections:
    - name: 用語集
      table:
        repeat: { min: 0 }
        header: [a, b]
        extract:
          path: glossary
          value: cells
          of: { line: line }
"#;
    let doc = "## 用語集\n\n| a | b |\n|---|---|\n| 1 | 2 |\n\n| a | b |\n|---|---|\n| 3 | 4 |\n";
    let v = values(schema, doc);
    assert_eq!(
        v["glossary"],
        json!([
            [{ "cells": { "a": "1", "b": "2" }, "line": 5 }],
            [{ "cells": { "a": "3", "b": "4" }, "line": 9 }]
        ]),
        "繰り返す表は配置パスの直下に表ごとの段を作る（TBL-schema-008）"
    );
}

// @kotowari[REQ-schema-033, REQ-schema-035]
#[test]
fn table_without_a_declared_header_gives_each_row_as_an_array() {
    let schema = r#"
document:
  sections:
    - name: 決定表
      table:
        extract: rows
"#;
    let doc = "## 決定表\n\n| 順 | 条件 |\n|---|---|\n| 1 | あれ |\n| 2 | これ |\n";
    let v = values(schema, doc);
    assert_eq!(
        v["rows"],
        json!([["1", "あれ"], ["2", "これ"]]),
        "header を宣言しない表の行は列の位置の配列になる（TBL-schema-008）"
    );
}

// @kotowari[REQ-schema-033, REQ-schema-035]
#[test]
fn table_with_empty_or_repeated_header_cells_keeps_every_column() {
    let empty = "## 決定表\n\n|  |  |\n|---|---|\n| 1 | 2 |\n";
    let repeated = "## 決定表\n\n| a | a |\n|---|---|\n| 1 | 2 |\n";
    let schema = r#"
document:
  sections:
    - name: 決定表
      table:
        extract: rows
"#;
    assert_eq!(
        values(schema, empty)["rows"],
        json!([["1", "2"]]),
        "ヘッダのセルが空でも列の値は1つも失われない（TBL-schema-008）"
    );
    assert_eq!(
        values(schema, repeated)["rows"],
        json!([["1", "2"]]),
        "同じ名前の列が2つあっても列の値は1つも失われない（TBL-schema-008）"
    );
}

// @kotowari[REQ-schema-033, REQ-schema-035]
#[test]
fn declared_header_decides_the_row_keys_when_the_column_count_differs() {
    let schema = r#"
document:
  sections:
    - name: 用語集
      table:
        header: [用語, 意味, 出典]
        extract: glossary
"#;
    let fewer = "## 用語集\n\n| 用語 | 意味 |\n|---|---|\n| 印 | しるし |\n";
    let more = "## 用語集\n\n| 用語 | 意味 | 出典 | 余り |\n|---|---|---|---|\n| 印 | しるし | a.md | 捨てる |\n";
    assert_eq!(
        values(schema, fewer)["glossary"],
        json!([{ "用語": "印", "意味": "しるし" }]),
        "文書の列が足りなければその鍵を省く（TBL-schema-008）"
    );
    assert_eq!(
        values(schema, more)["glossary"],
        json!([{ "用語": "印", "意味": "しるし", "出典": "a.md" }]),
        "文書の列が多ければ余りを捨てる（TBL-schema-008）"
    );
}

// @kotowari[REQ-schema-038]
#[test]
fn table_omits_key_when_no_tables() {
    let schema = r#"
document:
  sections:
    - name: 用語集
      table:
        repeat: { min: 0 }
        header: [a, b]
        extract: glossary
"#;
    let v = values(schema, "## 用語集\n");
    assert!(v.get("glossary").is_none(), "表0件のときはキーを省略する");
}

// @kotowari[REQ-schema-019, REQ-schema-035]
#[test]
fn repeated_section_extracts_array_of_bodies() {
    let schema = r#"
document:
  sections:
    - name: 状況
      repeat: { min: 0 }
      extract: contexts
      statement:
        required: false
"#;
    let doc = "## 状況\n\n1つ目。\n\n## 状況\n\n2つ目。\n";
    let v = values(schema, doc);
    assert_eq!(v["contexts"], json!(["1つ目。", "2つ目。"]));
}

fn typed(schema_yaml: &str, doc: &str) -> serde_json::Value {
    let schema = parse_schema(schema_yaml).unwrap();
    let document = Document::parse(doc).unwrap();
    extract_typed(&schema, &document.read_as(schema.reading))
}

// @kotowari[REQ-schema-036]
#[test]
fn typed_ast_includes_type_when_schema_is_named() {
    let schema = "name: adr\ndocument:\n  title:\n    extract: title\n";
    let v = typed(schema, "# 題名\n");
    assert_eq!(v["type"], "adr");
    assert_eq!(v["title"], "題名");
}

// @kotowari[REQ-schema-036]
#[test]
fn typed_ast_omits_type_when_schema_is_unnamed() {
    let schema = "document:\n  title:\n    extract: title\n";
    let v = typed(schema, "# 題名\n");
    assert!(v.get("type").is_none());
    assert_eq!(v["title"], "題名");
}

// @kotowari[REQ-schema-031, REQ-schema-035]
#[test]
fn parent_bullet_element_includes_child_bullet_lines_with_indentation() {
    let schema = r#"
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        extract: decisions
        children:
          bullets:
            repeat: { min: 0 }
"#;
    let doc = "## 決定\n\n- A22 判断の記録\n  - 補足の子\n";
    let v = values(schema, doc);
    assert_eq!(
        v["decisions"],
        json!(["- A22 判断の記録\n  - 補足の子"]),
        "親の抽出要素に子の箇条書きの行をそのままのインデントで含める（REQ-schema-031、TBL-schema-008）"
    );
}

// @kotowari[REQ-schema-031, REQ-schema-035]
#[test]
fn declared_child_field_is_excluded_from_the_parent_element() {
    let schema = r#"
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        extract: decisions
        children:
          fields:
            - name: superseded_by
"#;
    let doc = "## 決定\n\n- A22 判断の記録\n  - superseded_by: [A5]\n";
    let v = values(schema, doc);
    assert_eq!(
        v["decisions"],
        json!(["- A22 判断の記録"]),
        "子フィールド行は親の抽出要素に含めない（A12）"
    );
}

// @kotowari[REQ-schema-036]
#[test]
fn child_field_with_own_extract_is_placed_at_its_path() {
    let schema = r#"
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        extract: decisions
        children:
          fields:
            - name: superseded_by
              extract: superseded_by
"#;
    let doc = "## 決定\n\n- A22 判断の記録\n  - superseded_by: [A5]\n";
    let v = values(schema, doc);
    assert_eq!(
        v["decisions"],
        json!(["- A22 判断の記録"]),
        "子フィールド行は親の抽出要素に含めない（A12）"
    );
    assert_eq!(
        v["superseded_by"], "[A5]",
        "子フィールドは自身の extract で配置パスに値を出す（A15）"
    );
}

// @kotowari[REQ-schema-031]
#[test]
fn child_of_declared_field_does_not_shadow_declared_child_field() {
    // 宣言済みフィールド行の下の子は未宣言の構造（REQ-schema-003）なので、子フィールドの
    // 抽出で拾わない。拾うと正当な子フィールドの値を覆い隠す。
    let schema = r#"
document:
  sections:
    - name: 決定
      fields:
        - name: 種類
      bullets:
        repeat: { min: 0 }
        extract: decisions
        children:
          fields:
            - name: superseded_by
              extract: superseded_by
"#;
    let doc = "## 決定\n\n- 種類: algorithm\n  - superseded_by: [余計]\n- A22 判断の記録\n  - superseded_by: [A5]\n";
    let v = values(schema, doc);
    assert_eq!(
        v["superseded_by"], "[A5]",
        "宣言済みフィールド行の下の子は、宣言済みの子フィールドの値を覆い隠さない（REQ-schema-003）"
    );
}

// @kotowari[REQ-schema-031, REQ-schema-035]
#[test]
fn recursive_nesting_is_included_in_the_parent_element() {
    let schema = r#"
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        extract: decisions
        children:
          bullets:
            repeat: { min: 0 }
            children:
              bullets:
                repeat: { min: 0 }
"#;
    let doc = "## 決定\n\n- 親\n  - 子\n    - 孫\n";
    let v = values(schema, doc);
    assert_eq!(
        v["decisions"],
        json!(["- 親\n  - 子\n    - 孫"]),
        "再帰的な入れ子が親の抽出要素に含まれる（TBL-schema-008）"
    );
}

// @kotowari[REQ-schema-031, REQ-schema-035]
#[test]
fn section_body_includes_child_bullet_lines() {
    let schema = r#"
document:
  sections:
    - name: 決定
      extract: sections.body
      bullets:
        repeat: { min: 0 }
        children:
          bullets:
            repeat: { min: 0 }
"#;
    let doc = "## 決定\n\n- 親\n  - 子\n";
    let v = values(schema, doc);
    assert_eq!(
        v["sections"]["body"], "- 親\n  - 子",
        "節の本文は子の箇条書きの行を含む（TBL-schema-008）"
    );
}

// @kotowari[REQ-schema-031, REQ-schema-035]
#[test]
fn item_body_includes_child_bullet_lines() {
    let schema = r#"
document:
  sections:
    - name: 要求
      item:
        repeat: { min: 0 }
        extract: items
        bullets:
          repeat: { min: 0 }
"#;
    let doc = "## 要求\n\n### REQ-001: 名前\n\n- 親\n  - 子\n";
    let v = values(schema, doc);
    assert_eq!(
        v["items"],
        json!(["REQ-001: 名前\n- 親\n  - 子"]),
        "項目の本文は子の箇条書きの行を含む（TBL-schema-008）"
    );
}

// @kotowari[REQ-schema-036]
#[test]
fn deep_child_field_extract_is_placed_at_its_path() {
    let schema = r#"
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        extract: decisions
        children:
          bullets:
            repeat: { min: 0 }
            children:
              fields:
                - name: 補足
                  extract: notes
"#;
    let doc = "## 決定\n\n- 親\n  - 子\n    - 補足: 深い\n";
    let v = values(schema, doc);
    assert_eq!(
        v["decisions"],
        json!(["- 親\n  - 子"]),
        "深いレベルの宣言された子フィールドも親の抽出要素に含めない（REQ-schema-036）"
    );
    assert_eq!(
        v["notes"], "深い",
        "深いレベルの子フィールドも自身の extract で配置パスに値を出す（A15）"
    );
}

// @kotowari[REQ-schema-035]
#[test]
fn item_body_excludes_table_and_code_block() {
    let schema = r#"
document:
  sections:
    - name: 決定表
      item:
        id: "TBL-\\d{3,}"
        repeat: { min: 0 }
        extract: tables
        fields:
          - name: 出典
        table:
          header: [用語, 意味]
        codeblock:
          lang: gherkin
"#;
    let doc = "## 決定表\n\n### TBL-001: 名前\n\n- 出典: docs/a.md\n\n| 用語 | 意味 |\n|---|---|\n| 印 | テストの印 |\n\n```gherkin\nScenario: 印を書く\n```\n";
    let v = values(schema, doc);
    assert_eq!(
        v["tables"],
        json!(["TBL-001: 名前\n- 出典: docs/a.md"]),
        "項目の抽出の本文に表を含めない（TBL-schema-008）"
    );
}

// @kotowari[REQ-schema-023, REQ-schema-033]
#[test]
fn preamble_table_is_extracted_as_row_objects() {
    let schema =
        "document:\n  preamble:\n    table:\n      header: [用語, 意味]\n      extract: glossary\n";
    let doc = "# 用語集\n\n| 用語 | 意味 |\n|---|---|\n| 印 | テストの印 |\n";
    let v = values(schema, doc);
    assert_eq!(
        v["glossary"],
        json!([{"用語": "印", "意味": "テストの印"}]),
        "前置部の表も、スキーマが宣言した header の名前を鍵にしたオブジェクトの配列で抽出する（表の抽出は TBL-schema-008）"
    );
}

// @kotowari[REQ-schema-048]
#[test]
fn of_line_places_the_line_number_as_a_number() {
    let schema = "document:\n  sections:\n    - name: 記録\n      fields:\n        - name: 状態\n          extract: { path: status, of: { status_line: line } }\n";
    let doc = "## 記録\n\n- 状態: ok\n";
    let v = values(schema, doc);
    assert_eq!(
        v["status"],
        json!({ "status_line": 3 }),
        "value を省いた要素オブジェクトは導かれる値の鍵だけを持ち、行番号は数値（REQ-schema-048）"
    );
}

// @kotowari[REQ-schema-047]
#[test]
fn a_path_only_item_extract_becomes_an_object_when_internals_declare_extract() {
    let schema = r#"
document:
  sections:
    - name: 要求
      item:
        id: "REQ-\\d{3,}"
        repeat: { min: 0 }
        extract: requirements
        fields:
          - name: 種類
            extract: kind
"#;
    let doc = "## 要求\n\n### REQ-001: 印の構文\n\n- 種類: algorithm\n";
    let v = values(schema, doc);
    assert_eq!(
        v["requirements"],
        json!([{ "kind": "algorithm" }]),
        "項目の抽出が置き場だけでも、内側の抽出があればオブジェクトになる"
    );
}

// @kotowari[REQ-schema-047, REQ-schema-048]
#[test]
fn item_extract_becomes_an_object_when_internals_declare_extract() {
    let schema = r#"
document:
  sections:
    - name: 要求
      item:
        id: "REQ-\\d{3,}"
        repeat: { min: 0 }
        extract:
          path: requirements
          of: { id: id, name: name, line: line }
        fields:
          - name: 種類
            extract: kind
          - name: 出典
            separator: ","
            extract: sources
        statement:
          extract: text
"#;
    let doc =
        "## 要求\n\n### REQ-001: 印の構文\n\n- 種類: algorithm\n- 出典: a.md, b.md\n\n本文。\n";
    let v = values(schema, doc);
    assert_eq!(
        v["requirements"],
        json!([{
            "kind": "algorithm",
            "sources": ["a.md", "b.md"],
            "text": "本文。",
            "id": "REQ-001",
            "name": "印の構文",
            "line": 3
        }]),
        "内部が extract を宣言したら項目はオブジェクトになる（REQ-schema-047）"
    );
}

// @kotowari[REQ-schema-047]
#[test]
fn item_extract_stays_a_string_without_internal_extract() {
    let schema = "document:\n  sections:\n    - name: 要求\n      item:\n        id: \"REQ-\\\\d{3,}\"\n        repeat: { min: 0 }\n        extract: requirements\n        fields:\n          - name: 種類\n";
    let doc = "## 要求\n\n### REQ-001: 印の構文\n\n- 種類: algorithm\n";
    let v = values(schema, doc);
    assert_eq!(
        v["requirements"],
        json!(["REQ-001: 印の構文\n- 種類: algorithm"]),
        "内部に extract が無ければ今までどおり文字列（REQ-schema-047）"
    );
}

// @kotowari[REQ-schema-035, REQ-schema-048]
#[test]
fn codeblock_with_a_derived_value_gives_one_object_per_block() {
    let schema = r#"
document:
  sections:
    - name: 具体例
      codeblock:
        lang: gherkin
        repeat: { min: 0 }
        extract: { path: scenarios, value: code, of: { line: line } }
"#;
    let doc = "## 具体例\n\n```gherkin\nScenario: 1つ目\n```\n\n```gherkin\nScenario: 2つ目\n```\n";
    let v = values(schema, doc);
    assert_eq!(
        v["scenarios"],
        json!([
            { "code": "Scenario: 1つ目", "line": 3 },
            { "code": "Scenario: 2つ目", "line": 7 }
        ]),
        "コードブロックはブロックが要素の単位で、行はフェンスの開始行（TBL-schema-008）"
    );
}

// @kotowari[REQ-schema-035, REQ-schema-048]
#[test]
fn bullets_with_a_derived_value_give_one_object_per_line() {
    let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        extract: { path: reasons, value: text, of: { line: line } }
"#;
    let doc = "## 理由\n\n- 理由1\n- 理由2\n";
    let v = values(schema, doc);
    assert_eq!(
        v["reasons"],
        json!([
            { "text": "- 理由1", "line": 3 },
            { "text": "- 理由2", "line": 4 }
        ]),
        "箇条書きは行が要素の単位になる（TBL-schema-008）"
    );
}

// @kotowari[REQ-schema-035, REQ-schema-048]
#[test]
fn title_with_a_raw_derived_value_gives_the_heading_line() {
    let schema =
        "document:\n  title:\n    extract: { path: title, value: text, of: { raw: raw } }\n";
    let doc = "# 題名 `インライン`\n";
    let v = values(schema, doc);
    assert_eq!(
        v["title"]["raw"], "# 題名 `インライン`",
        "raw は見出しの行そのままで、組み立て直さない（REQ-schema-048）"
    );
    assert_eq!(
        v["title"]["text"], "題名 インライン",
        "要素の値は今までの題名の文字のまま"
    );
}

// @kotowari[REQ-schema-035, REQ-schema-048]
#[test]
fn field_with_a_value_and_a_derived_value_nests_under_the_path() {
    let schema = "document:\n  sections:\n    - name: 記録\n      fields:\n        - name: 状態\n          extract: { path: status, value: text, of: { line: line } }\n";
    let doc = "## 記録\n\n- 状態: ok\n";
    let v = values(schema, doc);
    assert_eq!(
        v["status"],
        json!({ "text": "ok", "line": 3 }),
        "分けないノードでも value か of を宣言すれば配置パスの下にオブジェクトができる（REQ-schema-048）"
    );
}

// @kotowari[REQ-schema-047, REQ-schema-048]
#[test]
fn item_without_a_value_keeps_the_inner_placement_paths() {
    let schema = r#"
document:
  sections:
    - name: 要求
      item:
        id: "REQ-\\d{3,}"
        repeat: { min: 0 }
        extract: { path: requirements, of: { line: line } }
        fields:
          - name: 種類
            extract: kind
"#;
    let doc = "## 要求\n\n### REQ-001: 印の構文\n\n- 種類: algorithm\n";
    let v = values(schema, doc);
    assert_eq!(
        v["requirements"],
        json!([{ "kind": "algorithm", "line": 3 }]),
        "value を省いた要素オブジェクトは導かれる値と内側の配置パスだけを持つ（REQ-schema-047、REQ-schema-048）"
    );
}

// @kotowari[REQ-schema-047]
#[test]
fn item_object_keys_come_from_the_schema_not_the_document() {
    // 同じ配置パスを宣言していれば、文書の見出しの語とフィールド行の名前が
    // 変わっても出力のキーは変わらない。出力の契約はスキーマが持つ（A22）。
    let japanese = "document:\n  sections:\n    - name: 蔵書\n      item:\n        repeat: { min: 0 }\n        extract: { path: books, of: { code: id } }\n        fields:\n          - name: 著者\n            extract: author\n";
    let english = "document:\n  sections:\n    - name: Books\n      item:\n        repeat: { min: 0 }\n        extract: { path: books, of: { code: id } }\n        fields:\n          - name: Author\n            extract: author\n";
    let japanese_doc = "## 蔵書\n\n### ISBN-1: 吾輩は猫である\n\n- 著者: 夏目漱石\n";
    let english_doc = "## Books\n\n### ISBN-1: I Am a Cat\n\n- Author: Soseki Natsume\n";

    let from_japanese = values(japanese, japanese_doc);
    let from_english = values(english, english_doc);

    assert_eq!(
        from_japanese["books"],
        json!([{ "author": "夏目漱石", "code": "ISBN-1" }])
    );
    assert_eq!(
        from_english["books"],
        json!([{ "author": "Soseki Natsume", "code": "ISBN-1" }])
    );
    assert_eq!(
        from_japanese["books"][0]
            .as_object()
            .unwrap()
            .keys()
            .collect::<Vec<_>>(),
        from_english["books"][0]
            .as_object()
            .unwrap()
            .keys()
            .collect::<Vec<_>>(),
        "文書の語が違っても出力のキーは同じ"
    );
}
