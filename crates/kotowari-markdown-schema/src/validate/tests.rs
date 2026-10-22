use super::*;
use crate::document::Document;
use crate::schema::parse_schema;

fn validate_src(schema_yaml: &str, doc: &str, open: bool) -> Vec<Finding> {
    let schema = parse_schema(schema_yaml).unwrap();
    let document = Document::parse(doc).unwrap();
    validate(&schema, &document.read_as(schema.reading), open)
}

fn kinds(findings: &[Finding]) -> Vec<FindingKind> {
    findings.iter().map(|f| f.kind).collect()
}

const SCHEMA: &str = r#"
name: adr
document:
  title:
    pattern: "^ADR-\\d{4}:"
  preamble:
    fields:
      - name: 状態
  sections:
    - name: 状況
      statement:
        required: false
    - name: 決定
      statement:
        required: false
"#;

fn ok_body() -> &'static str {
    "# ADR-0001: 印\n\n- 状態: 承認済み\n\n## 状況\n\n背景。\n\n## 決定\n\n判断。\n"
}

// @kotowari[REQ-schema-022, EX-schema-010]
#[test]
fn multiple_titles_is_found() {
    let doc = "# ADR-0001: a\n\n# ADR-0002: b\n\n## 状況\n\n背景。\n\n## 決定\n\n判断。\n";
    let findings = validate_src(SCHEMA, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::MultipleTitles));
}

// @kotowari[REQ-schema-022]
#[test]
fn title_pattern_mismatch_is_found() {
    let doc = "# テストの印\n\n- 状態: 承認済み\n\n## 状況\n\n背景。\n\n## 決定\n\n判断。\n";
    let findings = validate_src(SCHEMA, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::TitlePatternMismatch));
}

// @kotowari[REQ-schema-001, REQ-schema-024]
#[test]
fn undeclared_heading_is_found() {
    let doc = "# ADR-0001: a\n\n## 状況\n\n背景。\n\n## 決定\n\n判断。\n\n## 補足\n\n余計な節。\n";
    let findings = validate_src(SCHEMA, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::UndeclaredHeading));
}

// @kotowari[REQ-schema-001]
#[test]
fn undeclared_line_is_found() {
    let doc = "# ADR-0001: a\n\n## 状況\n\n- 宣言外の箇条書き\n\n## 決定\n\n判断。\n";
    let findings = validate_src(SCHEMA, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::UndeclaredLine));
}

// @kotowari[REQ-schema-030]
#[test]
fn continuation_paragraph_is_part_of_the_bullet_not_undeclared() {
    let schema =
        "document:\n  sections:\n    - name: 理由\n      bullets:\n        repeat: { min: 0 }\n";
    let doc = "## 理由\n\n- 親\n\n  続きの段落\n";
    let findings = validate_src(schema, doc, false);
    assert!(!kinds(&findings).contains(&FindingKind::UndeclaredLine));
}

// @kotowari[REQ-schema-030, REQ-schema-032]
#[test]
fn continuation_paragraph_does_not_count_as_statement() {
    let schema = "document:\n  sections:\n    - name: 理由\n      statement:\n        required: true\n      bullets:\n        repeat: { min: 0 }\n";
    let doc = "## 理由\n\n- 親\n\n  続きの段落\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::MissingStatement));
}

// @kotowari[REQ-schema-054]
#[test]
fn bullet_pattern_is_matched_only_on_the_marker_line_not_on_a_lazy_continuation_line() {
    // 段落の読み方では遅延継続の行も lead 段落に入るが、照合はマーカーの行だけに当てる。
    // 行の読み方と同じ結果になる
    let schema = |reading: &str| {
        format!(
            "reading: {reading}\ndocument:\n  sections:\n    - name: 理由\n      bullets:\n        repeat: {{ min: 0 }}\n        pattern: \"^親$\"\n"
        )
    };
    let doc = "## 理由\n\n- 親\n続き\n";
    for reading in ["paragraph", "line"] {
        let findings = validate_src(&schema(reading), doc, true);
        assert!(
            !kinds(&findings).contains(&FindingKind::BulletPatternMismatch),
            "{reading}: {findings:?}"
        );
    }
    let findings = validate_src(&schema("paragraph"), "## 理由\n\n- 子\n続き\n", true);
    let mismatch = findings
        .iter()
        .find(|f| f.kind == FindingKind::BulletPatternMismatch)
        .expect("マーカーの行が pattern に合わなければ指摘する");
    assert!(
        mismatch.detail.contains("bullet \"子\" does"),
        "{mismatch:?}"
    );
}

// @kotowari[REQ-schema-030]
#[test]
fn bullet_pattern_is_not_applied_to_continuation_paragraphs() {
    let schema = "document:\n  sections:\n    - name: 理由\n      bullets:\n        repeat: { min: 0 }\n        pattern: \"^親\"\n";
    let doc = "## 理由\n\n- 親\n\n  続きの段落はパターンに合わない\n";
    let findings = validate_src(schema, doc, false);
    assert!(!kinds(&findings).contains(&FindingKind::BulletPatternMismatch));
}

// @kotowari[REQ-schema-001, REQ-schema-031]
#[test]
fn code_block_child_of_list_item_is_undeclared_in_closed_world() {
    let schema = "document:\n  sections:\n    - name: 理由\n";
    let doc = "## 理由\n\n- 親\n\n  ```\n  x = 1\n  ```\n";
    let findings = validate_src(schema, doc, false);
    let details: Vec<&str> = findings
        .iter()
        .filter(|f| f.kind == FindingKind::UndeclaredLine)
        .map(|f| f.detail.as_str())
        .collect();
    assert!(
        details.contains(&"undeclared code block"),
        "リスト項目の中のコードブロックが undeclared_line になる: {details:?}"
    );
}

// @kotowari[REQ-schema-001, REQ-schema-031]
#[test]
fn code_block_as_first_child_of_list_item_is_undeclared_in_closed_world() {
    let schema =
        "document:\n  sections:\n    - name: 理由\n      bullets:\n        repeat: { min: 0 }\n";
    let doc = "## 理由\n\n- ```python\n  x = 1\n  ```\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        kinds(&findings).contains(&FindingKind::UndeclaredLine),
        "先頭がコードブロックのリスト項目は undeclared_line になる: {:?}",
        kinds(&findings)
    );
}

// @kotowari[TBL-schema-011]
#[test]
fn a_fence_opened_on_a_list_marker_line_closes_at_its_closing_line_in_both_readings() {
    // フェンスで囲んだブロックは字下げの深さに依らずどちらの読み方でもコードブロック。
    // マーカーの行で開いたフェンスの閉じの行を開始と取り違えると、後ろの見出しまで
    // コードブロックに吸い込み、節 B を読めなくなる
    let schema = |reading: &str| {
        format!(
            "reading: {reading}\ndocument:\n  sections:\n    - name: A\n      codeblock:\n        required: false\n        lang: python\n        extract: code\n    - name: B\n      statement:\n        extract: b\n"
        )
    };
    let doc = "## A\n\n- ```python\n  x = 1\n  ```\n\n## B\n\n本文\n";
    for reading in ["paragraph", "line"] {
        let schema = parse_schema(&schema(reading)).unwrap();
        let document = Document::parse(doc).unwrap();
        let findings = validate(&schema, &document, false);
        assert_eq!(kinds(&findings), vec![], "{reading}: {findings:?}");
        assert_eq!(
            crate::extract::extract_values(&schema, &document),
            serde_json::json!({ "code": "x = 1", "b": "本文" }),
            "{reading}"
        );
    }
}

// @kotowari[REQ-schema-001, REQ-schema-031]
#[test]
fn table_as_first_child_of_list_item_is_undeclared_in_closed_world() {
    let schema =
        "document:\n  sections:\n    - name: 理由\n      bullets:\n        repeat: { min: 0 }\n";
    let doc = "## 理由\n\n- | a | b |\n  |---|---|\n  | 1 | 2 |\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        kinds(&findings).contains(&FindingKind::UndeclaredLine),
        "先頭が表のリスト項目は undeclared_line になる: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-001, REQ-schema-031]
#[test]
fn table_child_of_list_item_is_undeclared_in_closed_world() {
    let schema = "document:\n  sections:\n    - name: 理由\n";
    let doc = "## 理由\n\n- 親\n\n  | a | b |\n  |---|---|\n  | 1 | 2 |\n";
    let findings = validate_src(schema, doc, false);
    let details: Vec<&str> = findings
        .iter()
        .filter(|f| f.kind == FindingKind::UndeclaredLine)
        .map(|f| f.detail.as_str())
        .collect();
    assert!(
        details.contains(&"undeclared table"),
        "リスト項目の中の表が undeclared_line になる: {details:?}"
    );
}

// @kotowari[REQ-schema-001, REQ-schema-032]
#[test]
fn paragraph_after_code_block_lead_is_undeclared_line_in_closed_world() {
    let schema = "document:\n  sections:\n    - name: 理由\n      bullets:\n        repeat: { min: 0 }\n      codeblock:\n        required: false\n        lang: python\n";
    let doc = "## 理由\n\n- ```python\n  x = 1\n  ```\n\n  後続の段落\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        kinds(&findings).contains(&FindingKind::UndeclaredLine),
        "先頭がコードブロックのリスト項目の後続段落は undeclared_line になる: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-001, REQ-schema-032]
#[test]
fn paragraph_after_table_lead_is_undeclared_line_in_closed_world() {
    let schema = "document:\n  sections:\n    - name: 理由\n      table:\n        required: false\n        header: [a, b]\n      bullets:\n        repeat: { min: 0 }\n";
    let doc = "## 理由\n\n- | a | b |\n  |---|---|\n  | 1 | 2 |\n\n  後続の段落\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        kinds(&findings).contains(&FindingKind::UndeclaredLine),
        "先頭が表のリスト項目の後続段落は undeclared_line になる: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-032]
#[test]
fn paragraph_after_code_block_lead_counts_as_a_statement() {
    let schema = "document:\n  sections:\n    - name: 状況\n      statement:\n        required: true\n      codeblock:\n        required: false\n        lang: python\n      bullets:\n        repeat: { min: 0 }\n";
    let doc = "## 状況\n\n- ```python\n  x = 1\n  ```\n\n  後続の段落\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        !kinds(&findings).contains(&FindingKind::MissingStatement),
        "先頭がコードブロックのリスト項目の後続段落は文として数える: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-032]
#[test]
fn paragraph_after_code_block_lead_is_subject_to_statement_pattern() {
    let schema = "document:\n  sections:\n    - name: 状況\n      statement:\n        pattern: \"^状況\"\n      codeblock:\n        required: false\n        lang: python\n      bullets:\n        repeat: { min: 0 }\n";
    let doc = "## 状況\n\n- ```python\n  x = 1\n  ```\n\n  後続の段落\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        kinds(&findings).contains(&FindingKind::StatementPatternMismatch),
        "先頭がコードブロックのリスト項目の後続段落に文の pattern を適用する: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-032]
#[test]
fn image_only_paragraph_after_code_block_lead_is_not_a_statement() {
    let schema = "document:\n  sections:\n    - name: 状況\n      statement:\n        required: true\n      codeblock:\n        required: false\n        lang: python\n      bullets:\n        repeat: { min: 0 }\n";
    let doc = "## 状況\n\n- ```python\n  x = 1\n  ```\n\n  ![alt](img.png)\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        kinds(&findings).contains(&FindingKind::MissingStatement),
        "先頭がコードブロックのリスト項目の画像だけの段落は文に数えない（REQ-schema-032）: {:?}",
        kinds(&findings)
    );
    assert!(
        !kinds(&findings).contains(&FindingKind::UndeclaredLine),
        "先頭がコードブロックのリスト項目の画像だけの段落は undeclared_line にしない（REQ-schema-032）: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-031]
#[test]
fn nested_list_items_are_not_counted_as_top_level_bullets() {
    // 親の bullets の本数はトップレベルの親だけを数え、子は数えない（REQ-schema-031）。
    let schema =
        "document:\n  sections:\n    - name: 理由\n      bullets:\n        repeat: { min: 2 }\n";
    let doc = "## 理由\n\n- 親\n  - 子\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        kinds(&findings).contains(&FindingKind::RepeatMinNotMet),
        "子は親の本数に数えず、min: 2 が満たされない（REQ-schema-031）: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-019]
#[test]
fn two_top_level_bullets_satisfy_min_two() {
    let schema =
        "document:\n  sections:\n    - name: 理由\n      bullets:\n        repeat: { min: 2 }\n";
    let doc = "## 理由\n\n- 親\n  - 子\n- 別\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        !kinds(&findings).contains(&FindingKind::RepeatMinNotMet),
        "トップレベルの親が2本あれば満たす: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-031]
#[test]
fn declared_child_field_passes() {
    let schema = r#"
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        children:
          fields:
            - name: superseded_by
"#;
    let doc = "## 決定\n\n- A22 判断の記録\n  - superseded_by: [A5]\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        !kinds(&findings).contains(&FindingKind::UndeclaredLine),
        "宣言された子フィールドは通る（REQ-schema-031）: {:?}",
        kinds(&findings)
    );
    assert!(!kinds(&findings).contains(&FindingKind::MissingRequiredField));
}

// @kotowari[REQ-schema-028, REQ-schema-031]
#[test]
fn undeclared_child_field_name_is_a_bullet_and_undeclared_without_rule() {
    let schema = r#"
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        children:
          fields:
            - name: superseded_by
"#;
    let doc = "## 決定\n\n- A22 判断の記録\n  - 備考: 補足\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        kinds(&findings).contains(&FindingKind::UndeclaredLine),
        "一致しない `- 名前: 値` は箇条書きとして扱い、children.bullets が無いので undeclared_line（TBL-schema-007、REQ-schema-031）: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-031]
#[test]
fn child_list_without_children_rule_is_undeclared_line() {
    let schema =
        "document:\n  sections:\n    - name: 理由\n      bullets:\n        repeat: { min: 0 }\n";
    let doc = "## 理由\n\n- 親\n  - 子\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        kinds(&findings).contains(&FindingKind::UndeclaredLine),
        "親が children を持たないのに子リストがある場合は undeclared_line（REQ-schema-001）: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-031]
#[test]
fn child_list_under_declared_field_is_undeclared_line() {
    // フィールド行は children 宣言を持たない。宣言済みフィールド行の下の
    // 子リストは、閉じた世界では undeclared_line（REQ-schema-001）。
    let schema = r#"
document:
  sections:
    - name: 状況
      fields:
        - name: 状態
      statement:
        required: false
"#;
    let doc = "## 状況\n\n- 状態: 承認済み\n  - 子箇条書き\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        kinds(&findings).contains(&FindingKind::UndeclaredLine),
        "宣言済みフィールド行の下の子リストは undeclared_line（REQ-schema-001）: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-031]
#[test]
fn child_list_under_declared_child_field_is_undeclared_line() {
    // 子フィールド（children.fields の宣言）も children 宣言を持たない。
    // その下の子リストは undeclared_line（REQ-schema-001）。
    let schema = r#"
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        children:
          fields:
            - name: superseded_by
"#;
    let doc = "## 決定\n\n- A22 判断の記録\n  - superseded_by: [A5]\n    - さらに子\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        kinds(&findings).contains(&FindingKind::UndeclaredLine),
        "宣言済みの子フィールド行の下の子リストは undeclared_line（REQ-schema-001）: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-031, REQ-schema-028]
#[test]
fn a_child_line_with_an_undeclared_name_is_checked_as_a_child_bullet() {
    let schema = r#"
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        children:
          bullets:
            repeat: { min: 0 }
            pattern: "^子"
"#;
    let doc = "## 決定\n\n- 親\n  - 名前: 値\n";
    let findings = validate_src(schema, doc, false);
    assert_eq!(
        kinds(&findings),
        vec![FindingKind::BulletPatternMismatch],
        "宣言していない名前の子の行は子の箇条書きとして照合する"
    );
}

// @kotowari[REQ-schema-021]
#[test]
fn child_bullet_when_references_a_sibling_field() {
    // children.bullets の when は、同じ children ノードの下の兄弟の
    // children.fields を参照する（REQ-schema-021）。when が真のときだけ pattern が効く。
    let schema = r#"
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        children:
          fields:
            - name: 種類
          bullets:
            repeat: { min: 0 }
            pattern: "^子"
            when: { field: 種類, eq: algorithm }
"#;
    let doc = "## 決定\n\n- 親\n  - 種類: algorithm\n  - 違反\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        kinds(&findings).contains(&FindingKind::BulletPatternMismatch),
        "children.bullets の when は兄弟の children.fields を参照する（REQ-schema-021）: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-020, REQ-schema-031]
#[test]
fn children_are_validated_when_parent_bullets_when_is_true() {
    // 親の bullets の when が真のとき、子は children の宣言に照合する（REQ-schema-031）。
    let schema = r#"
document:
  sections:
    - name: 決定
      fields:
        - name: 種類
      bullets:
        repeat: { min: 0 }
        when: { field: 種類, eq: algorithm }
        children:
          fields:
            - name: superseded_by
"#;
    let doc = "## 決定\n\n- 種類: algorithm\n\n- A22 判断の記録\n  - 備考: 補足\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        kinds(&findings).contains(&FindingKind::UndeclaredLine),
        "when が真のとき子の未宣言行は undeclared_line（REQ-schema-031）: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-020, REQ-schema-031]
#[test]
fn children_are_validated_when_parent_bullets_when_is_false() {
    // 親の bullets の when は required / pattern / enum にだけ効く（REQ-schema-020）。
    // children の照合（REQ-schema-031）は when の真偽に関わらず実行する。
    let schema = r#"
document:
  sections:
    - name: 決定
      fields:
        - name: 種類
      bullets:
        repeat: { min: 0 }
        when: { field: 種類, eq: algorithm }
        children:
          fields:
            - name: superseded_by
"#;
    let doc = "## 決定\n\n- 種類: other\n\n- A22 判断の記録\n  - 備考: 補足\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        kinds(&findings).contains(&FindingKind::UndeclaredLine),
        "親の bullets の when が偽でも子は children の宣言に照合する（REQ-schema-031）: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-031]
#[test]
fn recursive_children_bullets_match() {
    let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        children:
          bullets:
            repeat: { min: 0 }
            pattern: "^子"
            children:
              bullets:
                repeat: { min: 0 }
                pattern: "^孫"
"#;
    let doc = "## 理由\n\n- 親\n  - 子\n    - 孫\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        !kinds(&findings).contains(&FindingKind::BulletPatternMismatch),
        "再帰的な children.bullets に照合する（REQ-schema-031）: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-031]
#[test]
fn recursive_children_bullets_pattern_is_enforced() {
    let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 0 }
        children:
          bullets:
            repeat: { min: 0 }
            pattern: "^子"
            children:
              bullets:
                repeat: { min: 0 }
                pattern: "^孫"
"#;
    let doc = "## 理由\n\n- 親\n  - 子\n    - 違反\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        kinds(&findings).contains(&FindingKind::BulletPatternMismatch),
        "子の子の箇条書きにも children.bullets の pattern を適用する（REQ-schema-031）: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-019, REQ-schema-031]
#[test]
fn child_bullets_are_counted_by_children_rule_not_parent_repeat() {
    let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        repeat: { min: 1 }
        children:
          bullets:
            repeat: { min: 2 }
"#;
    let ok = validate_src(schema, "## 理由\n\n- 親\n  - 子1\n  - 子2\n", false);
    assert!(
        !kinds(&ok).contains(&FindingKind::RepeatMinNotMet),
        "親1本・子2本で両方満たす（REQ-schema-019、REQ-schema-031）: {:?}",
        kinds(&ok)
    );
    let short = validate_src(schema, "## 理由\n\n- 親\n  - 子1\n", false);
    assert!(
        kinds(&short).contains(&FindingKind::RepeatMinNotMet),
        "子が1本しか無ければ children.bullets の min を満たさない（REQ-schema-019、REQ-schema-031）: {:?}",
        kinds(&short)
    );
}

// @kotowari[REQ-schema-003, REQ-schema-031]
#[test]
fn undeclared_child_is_undeclared_even_when_open() {
    let schema = r#"
open: true
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        children:
          fields:
            - name: superseded_by
"#;
    let doc = "## 決定\n\n- A22 判断の記録\n  - 備考: 補足\n";
    let findings = validate_src(schema, doc, true);
    assert!(
        kinds(&findings).contains(&FindingKind::UndeclaredLine),
        "宣言済みの構造の中の未宣言の子は open でも undeclared_line（REQ-schema-003）: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-029, REQ-schema-031]
#[test]
fn declared_child_field_pattern_is_enforced() {
    let schema = r#"
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        children:
          fields:
            - name: superseded_by
              pattern: "^\\["
"#;
    let doc = "## 決定\n\n- A22 判断の記録\n  - superseded_by: 未指定\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        kinds(&findings).contains(&FindingKind::FieldPatternMismatch),
        "子フィールドの pattern は値に適用する（REQ-schema-029）: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-004, REQ-schema-031]
#[test]
fn missing_required_child_field_is_found() {
    let schema = r#"
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        children:
          fields:
            - name: superseded_by
"#;
    let doc = "## 決定\n\n- A22 判断の記録\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        kinds(&findings).contains(&FindingKind::MissingRequiredField),
        "宣言された子フィールドが無ければ missing_required_field（REQ-schema-004、REQ-schema-031）: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-001, REQ-schema-031]
#[test]
fn nested_list_items_are_each_reported_in_closed_world() {
    let schema = "document:\n  sections:\n    - name: 理由\n";
    let doc = "## 理由\n\n- 親\n  - 子\n";
    let findings = validate_src(schema, doc, false);
    let undeclared = findings
        .iter()
        .filter(|f| f.kind == FindingKind::UndeclaredLine)
        .count();
    assert_eq!(undeclared, 2);
}

// @kotowari[REQ-schema-028]
#[test]
fn ordered_list_is_not_a_bullet_and_is_undeclared_in_closed_world() {
    let schema =
        "document:\n  sections:\n    - name: 理由\n      bullets:\n        pattern: \"^親\"\n";
    let doc = "## 理由\n\n1. 子\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        !kinds(&findings).contains(&FindingKind::BulletPatternMismatch),
        "順序付きリストは箇条書きの対象外なので pattern を適用しない（TBL-schema-007）"
    );
    assert!(kinds(&findings).contains(&FindingKind::UndeclaredLine));
}

// @kotowari[REQ-schema-028]
#[test]
fn ordered_list_does_not_satisfy_required_bullets() {
    let schema =
        "document:\n  sections:\n    - name: 理由\n      bullets:\n        required: true\n";
    let doc = "## 理由\n\n1. 順序付き\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::MissingBullets));
}

// @kotowari[REQ-schema-030]
#[test]
fn field_continuation_is_part_of_the_field_not_undeclared_or_statement() {
    let schema = "document:\n  preamble:\n    fields:\n      - name: 状態\n    statement:\n      required: true\n";
    let doc = "# 題名\n\n- 状態: 承認済み\n\n  継続の段落\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::MissingStatement));
    assert!(
        !kinds(&findings).contains(&FindingKind::UndeclaredLine),
        "継続段落はフィールド行の一部で undeclared_line にしない（REQ-schema-030）"
    );
}

// @kotowari[REQ-schema-004, REQ-schema-024]
#[test]
fn missing_required_section_is_found() {
    let doc = "# ADR-0001: a\n\n- 状態: 承認済み\n\n## 状況\n\n背景。\n";
    let findings = validate_src(SCHEMA, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::MissingRequiredSection));
}

// @kotowari[REQ-schema-026]
#[test]
fn heading_level_mismatch_is_found() {
    let doc = "# ADR-0001: a\n\n## 状況\n\n#### 深すぎ\n\n## 決定\n\n判断。\n";
    let findings = validate_src(SCHEMA, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::HeadingLevelMismatch));
}

// @kotowari[REQ-schema-025, EX-schema-009]
#[test]
fn invalid_item_id_is_found() {
    let schema = r#"
document:
  sections:
    - name: 要求
      item:
        id: "REQ-\\d{3,}"
        statement:
          required: false
"#;
    let doc = "## 要求\n\n### XYZ-001: 名前\n\n本文。\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::InvalidId));
}

// @kotowari[REQ-schema-028]
#[test]
fn declared_field_line_passes() {
    let findings = validate_src(SCHEMA, ok_body(), false);
    assert!(!kinds(&findings).contains(&FindingKind::UndeclaredLine));
    assert!(!kinds(&findings).contains(&FindingKind::MissingRequiredField));
}

// @kotowari[REQ-schema-028, EX-schema-011]
#[test]
fn undeclared_field_name_line_counts_as_a_bullet() {
    let schema =
        "document:\n  sections:\n    - name: 理由\n      bullets:\n        repeat: { min: 1 }\n";
    let doc =
        "## 理由\n\n- 判断の記録かどうかの見分け（A134: 決定の節の見出しを1つ以上持つファイル）\n";
    let findings = validate_src(schema, doc, false);
    let ks = kinds(&findings);
    assert!(
        !ks.contains(&FindingKind::UndeclaredLine),
        "未宣言の名前の `- 名前: 値` 行は箇条書きとして扱う（TBL-schema-007）: {ks:?}"
    );
    assert!(
        !ks.contains(&FindingKind::RepeatMinNotMet),
        "箇条書きとして本数に数える（TBL-schema-007）: {ks:?}"
    );
}

// @kotowari[REQ-schema-028]
#[test]
fn undeclared_field_name_line_is_subject_to_bullet_pattern() {
    let schema = "document:\n  sections:\n    - name: 理由\n      bullets:\n        repeat: { min: 0 }\n        pattern: \"^A134\"\n";
    let doc =
        "## 理由\n\n- 判断の記録かどうかの見分け（A134: 決定の節の見出しを1つ以上持つファイル）\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        kinds(&findings).contains(&FindingKind::BulletPatternMismatch),
        "箇条書きとして pattern を適用する（TBL-schema-007）"
    );
}

// @kotowari[REQ-schema-028]
#[test]
fn undeclared_field_name_line_matches_bullet_pattern_on_original_text() {
    let schema = "document:\n  sections:\n    - name: 理由\n      bullets:\n        repeat: { min: 0 }\n        pattern: \"^名前:値$\"\n";
    let doc = "## 理由\n\n- 名前:値\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        !kinds(&findings).contains(&FindingKind::BulletPatternMismatch),
        "pattern は元の行に適用する: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-001, REQ-schema-028]
#[test]
fn undeclared_field_name_line_is_undeclared_when_no_bullets_declared() {
    let schema =
        "document:\n  sections:\n    - name: 理由\n      statement:\n        required: false\n";
    let doc =
        "## 理由\n\n- 判断の記録かどうかの見分け（A134: 決定の節の見出しを1つ以上持つファイル）\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::UndeclaredLine));
}

// @kotowari[REQ-schema-002, REQ-schema-004]
#[test]
fn open_allows_undeclared_heading_and_line_but_not_missing_required() {
    let schema = r#"
open: true
document:
  title:
    pattern: "^ADR-\\d{4}:"
  sections:
    - name: 状況
      statement:
        required: false
    - name: 決定
      statement:
        required: false
"#;
    let doc = "# ADR-0001: a\n\n## 状況\n\n背景。\n\n## 補足\n\n- 宣言外\n";
    let findings = validate_src(schema, doc, true);
    let ks = kinds(&findings);
    assert!(!ks.contains(&FindingKind::UndeclaredHeading));
    assert!(!ks.contains(&FindingKind::UndeclaredLine));
    assert!(ks.contains(&FindingKind::MissingRequiredSection));
}

// @kotowari[REQ-schema-001, REQ-schema-022]
#[test]
fn title_without_rule_is_undeclared_heading_in_closed_world() {
    let schema =
        "document:\n  sections:\n    - name: 状況\n      statement:\n        required: false\n";
    let doc = "# 題名\n\n## 状況\n\n背景。\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::UndeclaredHeading));
}

// @kotowari[REQ-schema-002]
#[test]
fn title_without_rule_is_allowed_when_open() {
    let schema =
        "document:\n  sections:\n    - name: 状況\n      statement:\n        required: false\n";
    let doc = "# 題名\n\n## 状況\n\n背景。\n";
    let findings = validate_src(schema, doc, true);
    assert!(!kinds(&findings).contains(&FindingKind::UndeclaredHeading));
}

// @kotowari[REQ-schema-001]
#[test]
fn undeclared_section_lines_are_flagged_in_closed_world() {
    let schema =
        "document:\n  sections:\n    - name: 状況\n      statement:\n        required: false\n";
    let doc =
        "# 題名\n\n## 状況\n\n背景。\n\n## 補足\n\n- 余計な箇条書き\n\n### 補足の項目\n\n中身。\n";
    let findings = validate_src(schema, doc, false);
    let headings = findings
        .iter()
        .filter(|f| f.kind == FindingKind::UndeclaredHeading)
        .count();
    let lines = findings
        .iter()
        .filter(|f| f.kind == FindingKind::UndeclaredLine)
        .count();
    assert!(
        headings >= 2,
        "節と項目の見出しが undeclared_heading になる"
    );
    assert!(
        lines >= 2,
        "節の中の箇条書きと項目の中の行が undeclared_line になる"
    );
}

// @kotowari[REQ-schema-001, REQ-schema-023]
#[test]
fn undeclared_preamble_lines_are_flagged_in_closed_world() {
    let schema =
        "document:\n  sections:\n    - name: 状況\n      statement:\n        required: false\n";
    let doc = "# 題名\n\n- 状態: 承認済み\n\n## 状況\n\n背景。\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::UndeclaredLine));
}

// @kotowari[REQ-schema-002, REQ-schema-023]
#[test]
fn undeclared_preamble_lines_are_allowed_when_open() {
    let schema =
        "document:\n  sections:\n    - name: 状況\n      statement:\n        required: false\n";
    let doc = "# 題名\n\n- 状態: 承認済み\n\n## 状況\n\n背景。\n";
    let findings = validate_src(schema, doc, true);
    assert!(!kinds(&findings).contains(&FindingKind::UndeclaredLine));
}

// @kotowari[REQ-schema-002]
#[test]
fn undeclared_section_lines_are_allowed_when_open() {
    let schema =
        "document:\n  sections:\n    - name: 状況\n      statement:\n        required: false\n";
    let doc = "# 題名\n\n## 状況\n\n背景。\n\n## 補足\n\n- 余計な箇条書き\n";
    let findings = validate_src(schema, doc, true);
    assert!(!kinds(&findings).contains(&FindingKind::UndeclaredHeading));
    assert!(!kinds(&findings).contains(&FindingKind::UndeclaredLine));
}

// @kotowari[REQ-schema-003, REQ-schema-027]
#[test]
fn undeclared_item_inside_declared_section_is_flagged_even_when_open() {
    let schema = r#"
document:
  sections:
    - name: 要求
      statement:
        required: false
"#;
    let doc = "## 要求\n\n### REQ-001: 名前\n\n本文。\n";
    let findings = validate_src(schema, doc, true);
    assert!(kinds(&findings).contains(&FindingKind::UndeclaredHeading));
    assert!(kinds(&findings).contains(&FindingKind::UndeclaredLine));
}

// @kotowari[REQ-schema-003, REQ-schema-023]
#[test]
fn stray_heading_in_declared_preamble_is_undeclared_even_when_open() {
    let schema = r#"
document:
  preamble:
    statement:
      required: false
"#;
    let doc = "# 題名\n\n### 補足\n\n中身。\n";
    let findings = validate_src(schema, doc, true);
    assert!(
        kinds(&findings).contains(&FindingKind::UndeclaredHeading),
        "宣言済みの前置部の中の ### 見出しは open でも undeclared_heading（REQ-schema-003）: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-003, REQ-schema-023]
#[test]
fn stray_heading_lines_in_declared_preamble_are_undeclared_even_when_open() {
    let schema = r#"
document:
  preamble:
    statement:
      required: false
"#;
    let doc = "# 題名\n\n### 補足\n\n中身。\n";
    let findings = validate_src(schema, doc, true);
    assert!(
        kinds(&findings).contains(&FindingKind::UndeclaredLine),
        "宣言済みの前置部の中の見出しの内側の行は open でも undeclared_line（REQ-schema-003）: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-002]
#[test]
fn stray_heading_before_title_is_allowed_when_open() {
    let schema = r#"
document:
  preamble:
    statement:
      required: false
"#;
    let doc = "### 前置\n\n補足。\n\n# 題名\n";
    let findings = validate_src(schema, doc, true);
    assert!(
        !kinds(&findings).contains(&FindingKind::UndeclaredHeading),
        "題名より前の見出しは open では許す（REQ-schema-002）: {:?}",
        kinds(&findings)
    );
    assert!(
        !kinds(&findings).contains(&FindingKind::UndeclaredLine),
        "題名より前の見出しの内側の行も open では許す（REQ-schema-002）: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-001]
#[test]
fn stray_heading_before_title_is_undeclared_in_closed_world() {
    let schema = r#"
document:
  preamble:
    statement:
      required: false
"#;
    let doc = "### 前置\n\n補足。\n\n# 題名\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        kinds(&findings).contains(&FindingKind::UndeclaredHeading),
        "題名より前の見出しは閉じた世界で undeclared_heading（REQ-schema-001）: {:?}",
        kinds(&findings)
    );
    assert!(
        kinds(&findings).contains(&FindingKind::UndeclaredLine),
        "題名より前の見出しの内側の行は閉じた世界で undeclared_line（REQ-schema-001）: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-003, EX-schema-002]
#[test]
fn undeclared_line_in_declared_section_is_flagged_even_when_open() {
    let schema = r#"
document:
  sections:
    - name: 状況
      statement:
        required: false
"#;
    let doc = "## 状況\n\n本文。\n\n- 宣言外の箇条書き\n";
    let findings = validate_src(schema, doc, true);
    assert!(kinds(&findings).contains(&FindingKind::UndeclaredLine));
}

// ---- 行の規則（TBL-schema-007、REQ-schema-029〜REQ-schema-034） ----

// @kotowari[REQ-schema-004]
#[test]
fn missing_required_field_is_found() {
    let schema = "document:\n  preamble:\n    fields:\n      - name: 状態\n";
    let doc = "# 題名\n\n## 状況\n\n本文。\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::MissingRequiredField));
}

// @kotowari[REQ-schema-029]
#[test]
fn field_pattern_mismatch_is_found() {
    let schema = r#"
document:
  preamble:
    fields:
      - name: 日付
        pattern: "^\\d{4}-\\d{2}-\\d{2}"
"#;
    let doc = "# 題名\n\n- 日付: yesterday\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::FieldPatternMismatch));
}

// @kotowari[REQ-schema-029]
#[test]
fn field_enum_invalid_is_found() {
    let schema = r#"
document:
  preamble:
    fields:
      - name: 状態
        enum: [承認済み, 却下]
"#;
    let doc = "# 題名\n\n- 状態: 保留\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::FieldEnumInvalid));
}

// @kotowari[REQ-schema-029]
#[test]
fn separator_splits_value_before_enum_check() {
    let schema = r#"
document:
  preamble:
    fields:
      - name: タグ
        separator: ","
        enum: [a, b]
"#;
    let doc = "# 題名\n\n- タグ: a,c\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::FieldEnumInvalid));
}

// @kotowari[REQ-schema-029]
#[test]
fn separator_elements_are_trimmed_before_enum_check() {
    let schema = r#"
document:
  preamble:
    fields:
      - name: タグ
        separator: ","
        enum: [a, b, c]
"#;
    let doc = "# 題名\n\n- タグ: a, c\n";
    let findings = validate_src(schema, doc, false);
    assert!(!kinds(&findings).contains(&FindingKind::FieldEnumInvalid));
}

// @kotowari[REQ-schema-004, REQ-schema-032]
#[test]
fn missing_statement_is_found() {
    let schema = "document:\n  sections:\n    - name: 状況\n      statement: {}\n";
    let doc = "## 状況\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::MissingStatement));
}

// @kotowari[REQ-schema-054]
#[test]
fn statement_pattern_mismatch_is_found() {
    let schema = r#"
document:
  sections:
    - name: 状況
      statement:
        pattern: "^状況"
"#;
    let doc = "## 状況\n\n背景。\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::StatementPatternMismatch));
}

// @kotowari[REQ-schema-054]
#[test]
fn statement_enum_invalid_is_found() {
    let schema = r#"
document:
  sections:
    - name: 状況
      statement:
        enum: [a, b]
"#;
    let doc = "## 状況\n\n本文。\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::StatementEnumInvalid));
}

// @kotowari[REQ-schema-004, REQ-schema-019]
#[test]
fn missing_bullets_is_found() {
    let schema = "document:\n  sections:\n    - name: 理由\n      bullets: {}\n";
    let doc = "## 理由\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::MissingBullets));
}

// @kotowari[REQ-schema-054]
#[test]
fn bullet_pattern_mismatch_is_found() {
    let schema = r#"
document:
  sections:
    - name: 理由
      bullets:
        pattern: "^理由"
"#;
    let doc = "## 理由\n\n- その他\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::BulletPatternMismatch));
}

// @kotowari[REQ-schema-033]
#[test]
fn missing_table_is_found() {
    let schema = r#"
document:
  sections:
    - name: 用語集
      table:
        header: [用語, 意味]
"#;
    let doc = "## 用語集\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::MissingTable));
}

// @kotowari[REQ-schema-033]
#[test]
fn table_header_mismatch_is_found() {
    let schema = r#"
document:
  sections:
    - name: 用語集
      table:
        header: [用語, 意味]
"#;
    let doc = "## 用語集\n\n| 用語 |\n|---|\n| a |\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::TableHeaderMismatch));
}

// @kotowari[REQ-schema-034]
#[test]
fn missing_codeblock_is_found() {
    let schema = r#"
document:
  sections:
    - name: 具体例
      codeblock:
        lang: gherkin
"#;
    let doc = "## 具体例\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::MissingCodeblock));
}

// @kotowari[REQ-schema-034]
#[test]
fn codeblock_lang_mismatch_is_found() {
    let schema = r#"
document:
  sections:
    - name: 具体例
      codeblock:
        lang: gherkin
"#;
    let doc = "## 具体例\n\n```python\nx = 1\n```\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::CodeblockLangMismatch));
}

// @kotowari[REQ-schema-034]
#[test]
fn codeblock_line_mismatch_is_found() {
    let schema = r#"
document:
  sections:
    - name: 具体例
      codeblock:
        lang: gherkin
        lines: ["^Scenario:", "^Given "]
"#;
    let doc = "## 具体例\n\n```gherkin\nScenario: 印を書く\nBad line\n```\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::CodeblockLineMismatch));
}

// @kotowari[REQ-schema-034]
#[test]
fn codeblock_line_matching_keeps_trailing_whitespace() {
    let schema = r#"
document:
  sections:
    - name: 具体例
      codeblock:
        lang: gherkin
        lines: ["^Scenario:$"]
"#;
    // 末尾の空白まで取り除くと "$" で閉じた正規表現が通ってしまう
    let doc = "## 具体例\n\n```gherkin\nScenario:  \n```\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::CodeblockLineMismatch));
}

// @kotowari[REQ-schema-034]
#[test]
fn codeblock_line_matching_trims_leading_whitespace_and_skips_blank_lines() {
    let schema = r#"
document:
  sections:
    - name: 具体例
      codeblock:
        lang: gherkin
        lines: ["^Scenario:", "^Given "]
"#;
    let doc = "## 具体例\n\n```gherkin\n  Scenario: 印を書く\n\n   Given 文\n```\n";
    let findings = validate_src(schema, doc, false);
    assert!(!kinds(&findings).contains(&FindingKind::CodeblockLineMismatch));
}

// @kotowari[REQ-schema-032]
#[test]
fn blockquote_and_thematic_break_lines_are_ignored_in_closed_world() {
    let schema =
        "document:\n  sections:\n    - name: 状況\n      statement:\n        required: false\n";
    let doc = "## 状況\n\n> 引用\n\n---\n\n本文。\n";
    let findings = validate_src(schema, doc, false);
    assert!(!kinds(&findings).contains(&FindingKind::UndeclaredLine));
}

// @kotowari[REQ-schema-032]
#[test]
fn image_only_line_is_not_counted_as_statement() {
    let schema =
        "document:\n  sections:\n    - name: 状況\n      statement:\n        required: true\n";
    let doc = "## 状況\n\n![alt](img.png)\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::MissingStatement));
    assert!(!kinds(&findings).contains(&FindingKind::UndeclaredLine));
}

// ---- 出現回数・条件付き規則・並び順（REQ-schema-019、REQ-schema-020、REQ-schema-041） ----

// @kotowari[REQ-schema-019]
#[test]
fn repeat_min_not_met_for_section_is_found() {
    let schema = r#"
document:
  sections:
    - name: 理由
      repeat: { min: 2 }
"#;
    let doc = "## 理由\n\n本文。\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::RepeatMinNotMet));
}

// @kotowari[REQ-schema-056]
#[test]
fn missing_required_item_is_reported_as_repeat_min_not_met() {
    let schema = r#"
document:
  sections:
    - name: 要求
      item:
        id: "REQ-\\d{3}"
        repeat: { min: 1 }
"#;
    let doc = "## 要求\n";
    let findings = validate_src(schema, doc, false);
    let ks = kinds(&findings);
    // 項目の欠落は出現回数の下限として出る。節の欠落の種類は使わない
    assert!(ks.contains(&FindingKind::RepeatMinNotMet));
    assert!(!ks.contains(&FindingKind::MissingRequiredSection));
}

// @kotowari[REQ-schema-019]
#[test]
fn repeat_min_not_met_for_field_is_found() {
    let schema = r#"
document:
  preamble:
    fields:
      - name: タグ
        repeat: { min: 2 }
"#;
    let doc = "# 題名\n\n- タグ: a\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::RepeatMinNotMet));
}

// @kotowari[REQ-schema-019]
#[test]
fn repeat_max_exceeded_for_optional_section_is_found() {
    let schema = r#"
document:
  sections:
    - name: 補足
      required: false
      statement:
        required: false
"#;
    let doc = "## 補足\n\n1つ目。\n\n## 補足\n\n2つ目。\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::RepeatMaxExceeded));
}

// @kotowari[REQ-schema-019]
#[test]
fn repeat_max_exceeded_for_field_is_found() {
    let schema = r#"
document:
  preamble:
    fields:
      - name: タグ
        repeat: { max: 2 }
"#;
    let doc = "# 題名\n\n- タグ: a\n- タグ: b\n- タグ: c\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::RepeatMaxExceeded));
}

// @kotowari[REQ-schema-019]
#[test]
fn repeat_with_max_only_omits_min_as_zero() {
    let schema =
        "document:\n  preamble:\n    fields:\n      - name: タグ\n        repeat: { max: 2 }\n";
    let doc = "# 題名\n";
    let findings = validate_src(schema, doc, false);
    assert!(!kinds(&findings).contains(&FindingKind::RepeatMinNotMet));
}

// @kotowari[REQ-schema-019]
#[test]
fn item_with_no_repeat_requires_exactly_one() {
    let schema = r#"
document:
  sections:
    - name: 要求
      item:
        id: "REQ-\\d{3,}"
        statement:
          required: false
"#;
    let doc = "## 要求\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::RepeatMinNotMet));
}

// @kotowari[REQ-schema-019]
#[test]
fn item_repeat_max_exceeded_is_found() {
    let schema = r#"
document:
  sections:
    - name: 要求
      item:
        id: "REQ-\\d{3,}"
        repeat: { max: 1 }
        statement:
          required: false
"#;
    let doc = "## 要求\n\n### REQ-001: a\n\n本文。\n\n### REQ-002: b\n\n本文。\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::RepeatMaxExceeded));
}

// @kotowari[REQ-schema-019]
#[test]
fn item_with_required_false_is_optional() {
    let schema = r#"
document:
  sections:
    - name: 要求
      item:
        id: "REQ-\\d{3,}"
        required: false
        statement:
          required: false
"#;
    let doc = "## 要求\n";
    let findings = validate_src(schema, doc, false);
    assert!(!kinds(&findings).contains(&FindingKind::RepeatMinNotMet));
    assert!(!kinds(&findings).contains(&FindingKind::MissingRequiredSection));
}

// @kotowari[REQ-schema-025, EX-schema-009]
#[test]
fn item_heading_without_colon_is_invalid_id() {
    let schema = r#"
document:
  sections:
    - name: 要求
      item:
        statement:
          required: false
"#;
    let doc = "## 要求\n\n### REQ-001\n\n本文。\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::InvalidId));
}

// @kotowari[REQ-schema-041]
#[test]
fn a_repeated_field_in_a_row_is_still_in_declared_order() {
    let schema = r#"
document:
  sections:
    - name: 要求
      ordered: true
      fields:
        - name: 種類
          repeat: { max: 2 }
        - name: 定義
"#;
    let doc = "## 要求\n\n- 種類: algorithm\n- 種類: ubiquitous\n- 定義: REQ-001\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        !kinds(&findings).contains(&FindingKind::FieldOrderMismatch),
        "同じフィールド行が続いても書いた順に反しない: {:?}",
        kinds(&findings)
    );
}

// @kotowari[REQ-schema-041]
#[test]
fn fields_in_declared_order_pass() {
    let schema = r#"
document:
  sections:
    - name: 要求
      ordered: true
      fields:
        - name: 種類
        - name: 定義
"#;
    let doc = "## 要求\n\n- 種類: algorithm\n- 定義: REQ-001\n";
    let findings = validate_src(schema, doc, false);
    assert!(!kinds(&findings).contains(&FindingKind::FieldOrderMismatch));
}

// @kotowari[REQ-schema-020, EX-schema-007]
#[test]
fn when_eq_true_requires_the_field() {
    let schema = r#"
document:
  sections:
    - name: 要求
      fields:
        - name: 種類
        - name: 定義
          when: { field: 種類, eq: algorithm }
"#;
    let doc = "## 要求\n\n- 種類: algorithm\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::MissingRequiredField));
}

// @kotowari[REQ-schema-020]
#[test]
fn when_eq_false_skips_the_requirement() {
    let schema = r#"
document:
  sections:
    - name: 要求
      fields:
        - name: 種類
        - name: 定義
          when: { field: 種類, eq: algorithm }
"#;
    let doc = "## 要求\n\n- 種類: ubiquitous\n";
    let findings = validate_src(schema, doc, false);
    assert!(!kinds(&findings).contains(&FindingKind::MissingRequiredField));
}

// @kotowari[REQ-schema-021]
#[test]
fn when_with_missing_reference_makes_eq_false() {
    let schema = r#"
document:
  sections:
    - name: 要求
      fields:
        - name: 定義
          when: { field: 種類, eq: algorithm }
"#;
    let doc = "## 要求\n";
    let findings = validate_src(schema, doc, false);
    assert!(!kinds(&findings).contains(&FindingKind::MissingRequiredField));
}

// @kotowari[REQ-schema-021]
#[test]
fn when_with_missing_reference_makes_ne_true() {
    let schema = r#"
document:
  sections:
    - name: 要求
      fields:
        - name: 定義
          when: { field: 種類, ne: algorithm }
"#;
    let doc = "## 要求\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::MissingRequiredField));
}

// @kotowari[REQ-schema-021]
#[test]
fn when_eq_is_false_when_reference_name_is_undeclared() {
    let schema = r#"
document:
  sections:
    - name: 要求
      statement:
        required: true
        when: { field: 種類, eq: algorithm }
      bullets:
        repeat: { min: 0 }
"#;
    let doc = "## 要求\n\n- 種類: algorithm\n";
    let findings = validate_src(schema, doc, false);
    assert!(!kinds(&findings).contains(&FindingKind::MissingStatement));
}

// @kotowari[REQ-schema-021]
#[test]
fn when_ne_is_true_when_reference_name_is_undeclared() {
    let schema = r#"
document:
  sections:
    - name: 要求
      statement:
        required: true
        when: { field: 種類, ne: algorithm }
      bullets:
        repeat: { min: 0 }
"#;
    let doc = "## 要求\n\n- 種類: algorithm\n";
    let findings = validate_src(schema, doc, false);
    assert!(kinds(&findings).contains(&FindingKind::MissingStatement));
}

// @kotowari[REQ-schema-033]
#[test]
fn table_declared_on_an_item_is_not_undeclared() {
    let schema = r#"
document:
  sections:
    - name: 決定表
      item:
        id: "TBL-\\d{3,}"
        repeat: { min: 0 }
        table:
          header: [用語, 意味]
"#;
    let doc = "## 決定表\n\n### TBL-001: 名前\n\n| 用語 | 意味 |\n|---|---|\n| 印 | テストの印 |\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        !kinds(&findings).contains(&FindingKind::UndeclaredLine),
        "項目に宣言した表は undeclared_line にしない（TBL-schema-004）"
    );
}

// @kotowari[REQ-schema-033]
#[test]
fn table_header_mismatch_on_an_item_is_found() {
    let schema = r#"
document:
  sections:
    - name: 決定表
      item:
        id: "TBL-\\d{3,}"
        repeat: { min: 0 }
        table:
          header: [用語, 意味]
"#;
    let doc = "## 決定表\n\n### TBL-001: 名前\n\n| 語 | 訳 |\n|---|---|\n| 印 | mark |\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        kinds(&findings).contains(&FindingKind::TableHeaderMismatch),
        "項目の表のヘッダ不一致は table_header_mismatch にする（TBL-schema-004、REQ-schema-033）"
    );
}

// @kotowari[REQ-schema-034]
#[test]
fn code_block_declared_on_an_item_is_not_undeclared() {
    let schema = r#"
document:
  sections:
    - name: 具体例
      item:
        id: "EX-\\d{3,}"
        repeat: { min: 0 }
        codeblock:
          lang: gherkin
"#;
    let doc = "## 具体例\n\n### EX-001: 名前\n\n```gherkin\nScenario: 印を書く\n```\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        !kinds(&findings).contains(&FindingKind::UndeclaredLine),
        "項目に宣言したコードブロックは undeclared_line にしない（TBL-schema-004）"
    );
}

// @kotowari[REQ-schema-033, EX-schema-012]
#[test]
fn table_without_declared_header_accepts_any_header() {
    let schema = "document:\n  sections:\n    - name: 決定表\n      table: {}\n";
    let doc = "## 決定表\n\n| 項目 | 置く場所 | 文 |\n|---|---|---|\n| 要求 | 節の下 | 持つ |\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        !kinds(&findings).contains(&FindingKind::TableHeaderMismatch),
        "header を宣言しないときはヘッダと列数を検査しない（REQ-schema-033）"
    );
}

// @kotowari[REQ-schema-033]
#[test]
fn table_without_declared_header_is_still_required() {
    let schema = "document:\n  sections:\n    - name: 決定表\n      table: {}\n";
    let doc = "## 決定表\n\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        kinds(&findings).contains(&FindingKind::MissingTable),
        "header を宣言しなくても表の有無は検査する（REQ-schema-033）"
    );
}

// @kotowari[REQ-schema-023, REQ-schema-033]
#[test]
fn table_declared_on_the_preamble_is_not_undeclared() {
    let schema = "document:\n  preamble:\n    table:\n      header: [用語, 意味]\n";
    let doc = "# 用語集\n\n| 用語 | 意味 |\n|---|---|\n| 印 | テストの印 |\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        !kinds(&findings).contains(&FindingKind::UndeclaredLine),
        "前置部に宣言した表は undeclared_line にしない"
    );
}

// @kotowari[REQ-schema-023, REQ-schema-034]
#[test]
fn code_block_declared_on_the_preamble_is_not_undeclared() {
    let schema = "document:\n  preamble:\n    codeblock:\n      lang: gherkin\n";
    let doc = "# 具体例\n\n```gherkin\nScenario: 印を書く\n```\n";
    let findings = validate_src(schema, doc, false);
    assert!(
        !kinds(&findings).contains(&FindingKind::UndeclaredLine),
        "前置部に宣言したコードブロックは undeclared_line にしない"
    );
}
const ITEM_SCHEMA: &str = r#"
document:
  sections:
    - name: 要求
      item:
        id: "REQ-\\d+"
        repeat: { min: 0 }
        fields:
          - name: 種類
"#;

fn only(findings: &[Finding], kind: FindingKind) -> &Finding {
    let found: Vec<&Finding> = findings.iter().filter(|f| f.kind == kind).collect();
    assert_eq!(found.len(), 1, "{kind:?} が1件だけ出る: {findings:?}");
    found[0]
}

// @kotowari[REQ-schema-008]
#[test]
fn missing_required_field_points_at_the_item_heading_with_the_field_name() {
    let doc = "## 要求\n\n### REQ-001: 名前\n";
    let findings = validate_src(ITEM_SCHEMA, doc, false);
    let finding = only(&findings, FindingKind::MissingRequiredField);
    assert_eq!(finding.line, Some(3), "欠落は含む項目の見出しの行を指す");
    assert_eq!(finding.node.as_deref(), Some("種類"));
    assert_eq!(finding.raw.as_deref(), Some("### REQ-001: 名前"));
}

// @kotowari[REQ-schema-008]
#[test]
fn a_field_line_declared_twice_points_at_the_second_line() {
    let schema = "document:\n  preamble:\n    fields:\n      - name: 状態\n";
    let doc = "- 状態: a\n- 状態: b\n";
    let findings = validate_src(schema, doc, true);
    let finding = only(&findings, FindingKind::RepeatMaxExceeded);
    assert_eq!(finding.line, Some(2), "上限を超えた2つ目の行を指す");
    assert_eq!(finding.node.as_deref(), Some("状態"));
    assert_eq!(finding.raw.as_deref(), Some("- 状態: b"));
}

// @kotowari[REQ-schema-008, REQ-schema-033]
#[test]
fn a_table_row_with_the_wrong_column_count_points_at_that_row() {
    let schema =
        "document:\n  sections:\n    - name: 用語集\n      table:\n        header: [a, b]\n";
    let doc = "## 用語集\n\n| a | b |\n|---|---|\n| 1 | 2 |\n| 3 |\n";
    let findings = validate_src(schema, doc, false);
    let finding = only(&findings, FindingKind::TableHeaderMismatch);
    assert_eq!(finding.line, Some(6), "ヘッダの行ではなくその行を指す");
    assert_eq!(finding.raw.as_deref(), Some("| 3 |"));
    assert_eq!(finding.node, None, "表は宣言上の名前を持たない");
}

// @kotowari[REQ-schema-033]
#[test]
fn rows_are_counted_against_the_documents_header_not_the_declared_one() {
    let schema =
        "document:\n  sections:\n    - name: 用語集\n      table:\n        header: [a, b]\n";
    let doc = "## 用語集\n\n| a | b | c |\n|---|---|---|\n| 1 | 2 | 3 |\n| 4 | 5 | 6 |\n";
    let findings = validate_src(schema, doc, false);
    let finding = only(&findings, FindingKind::TableHeaderMismatch);
    assert_eq!(
        finding.line,
        Some(3),
        "文書のヘッダ行と合うデータ行は指摘せず、ヘッダの食い違いの1件だけを出す"
    );
}

// @kotowari[REQ-schema-022, REQ-schema-008]
#[test]
fn missing_title_has_neither_a_line_nor_a_node_name() {
    let doc = "## 状況\n\n背景。\n\n## 決定\n\n判断。\n";
    let findings = validate_src(SCHEMA, doc, false);
    let finding = only(&findings, FindingKind::MissingTitle);
    assert_eq!(finding.line, None, "含むノードに行が無いので省く");
    assert_eq!(finding.node, None);
    assert_eq!(finding.raw, None);
}

// @kotowari[REQ-schema-008]
#[test]
fn invalid_id_carries_the_raw_heading_line() {
    let doc = "## 要求\n\n### REQ-BAD: `名前`\n\n- 種類: x\n";
    let findings = validate_src(ITEM_SCHEMA, doc, false);
    let finding = only(&findings, FindingKind::InvalidId);
    assert_eq!(finding.line, Some(3));
    assert_eq!(
        finding.raw.as_deref(),
        Some("### REQ-BAD: `名前`"),
        "生の行は src の行と一文字も違わない"
    );
}

// @kotowari[REQ-schema-008]
#[test]
fn missing_required_section_carries_the_section_name() {
    let schema =
        "document:\n  sections:\n    - name: 状況\n      statement:\n        required: false\n";
    let doc = "";
    let findings = validate_src(schema, doc, true);
    let finding = only(&findings, FindingKind::MissingRequiredSection);
    assert_eq!(finding.node.as_deref(), Some("状況"));
    assert_eq!(finding.line, None, "文書そのものには含むノードの行が無い");
}

// @kotowari[REQ-schema-041, REQ-schema-008]
#[test]
fn field_order_mismatch_carries_the_name_of_the_field_out_of_order() {
    let schema = "document:\n  sections:\n    - name: 要求\n      ordered: true\n      fields:\n        - name: 種類\n        - name: 定義\n";
    let doc = "## 要求\n\n- 定義: REQ-001\n- 種類: algorithm\n";
    let findings = validate_src(schema, doc, false);
    let finding = only(&findings, FindingKind::FieldOrderMismatch);
    assert_eq!(finding.line, Some(4), "宣言の順に反した行を指す");
    assert_eq!(finding.node.as_deref(), Some("種類"));
    assert_eq!(finding.raw.as_deref(), Some("- 種類: algorithm"));
}

// @kotowari[REQ-schema-055]
#[test]
fn undeclared_field_line_carries_the_field_rule_kind() {
    let schema = "document:\n  sections:\n    - name: 理由\n";
    let doc = "## 理由\n\n- 種類: ubiquitous\n";
    let findings = validate_src(schema, doc, false);
    let finding = only(&findings, FindingKind::UndeclaredLine);
    assert_eq!(finding.rule_kind, Some(RuleKind::Field));
}

// @kotowari[REQ-schema-055]
#[test]
fn undeclared_bullet_carries_the_bullets_rule_kind() {
    let schema = "document:\n  sections:\n    - name: 理由\n";
    let doc = "## 理由\n\n- ただの箇条書き\n";
    let findings = validate_src(schema, doc, false);
    let finding = only(&findings, FindingKind::UndeclaredLine);
    assert_eq!(finding.rule_kind, Some(RuleKind::Bullets));
}

// @kotowari[REQ-schema-001]
#[test]
fn an_empty_bullet_item_is_an_undeclared_line() {
    let schema = "document:\n  sections:\n    - name: 理由\n";
    let doc = "## 理由\n\n-\n";
    let findings = validate_src(schema, doc, false);
    let finding = only(&findings, FindingKind::UndeclaredLine);
    assert_eq!(finding.rule_kind, Some(RuleKind::Bullets));
    assert_eq!(finding.line, Some(3));
    assert_eq!(finding.raw.as_deref(), Some("-"));
}

// @kotowari[REQ-schema-001]
#[test]
fn an_empty_ordered_list_item_is_an_undeclared_line() {
    let schema = "document:\n  sections:\n    - name: 理由\n";
    let doc = "## 理由\n\n1.\n";
    let findings = validate_src(schema, doc, false);
    let finding = only(&findings, FindingKind::UndeclaredLine);
    assert_eq!(finding.rule_kind, Some(RuleKind::OrderedList));
    assert_eq!(finding.line, Some(3));
}

// @kotowari[REQ-schema-055]
#[test]
fn undeclared_table_carries_the_table_rule_kind() {
    let schema = "document:\n  sections:\n    - name: 理由\n";
    let doc = "## 理由\n\n| a | b |\n|---|---|\n| 1 | 2 |\n";
    let findings = validate_src(schema, doc, false);
    let finding = only(&findings, FindingKind::UndeclaredLine);
    assert_eq!(finding.rule_kind, Some(RuleKind::Table));
}

// @kotowari[REQ-schema-057]
#[test]
fn repeat_min_not_met_carries_the_counted_rule_kind() {
    let schema =
        "document:\n  sections:\n    - name: 理由\n      statement:\n        repeat: { min: 1 }\n";
    let doc = "## 理由\n";
    let findings = validate_src(schema, doc, false);
    let finding = only(&findings, FindingKind::RepeatMinNotMet);
    assert_eq!(finding.rule_kind, Some(RuleKind::Statement));
}
