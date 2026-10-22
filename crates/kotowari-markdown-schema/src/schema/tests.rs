use super::*;

/// スキーマを読めず、その理由が reason を含むことを確かめる。別の理由で読めないスキーマを
/// 書き損じても通らないように、理由まで見る
fn assert_rejected(yaml: &str, reason: &str) {
    match parse_schema(yaml) {
        Ok(_) => panic!("the schema is accepted:\n{yaml}"),
        Err(error) => assert!(
            error.0.contains(reason),
            "expected {reason:?} in {:?}",
            error.0
        ),
    }
}

const ADR_LIKE: &str = r#"
name: adr
open: false
document:
  title:
    pattern: "^ADR-(?<id>\\d{4}):"
    extract: { path: id, group: id }
  preamble:
    fields:
      - name: 状態
        enum: [承認済み, 却下]
        extract: status
      - name: 日付
        pattern: "^\\d{4}-\\d{2}-\\d{2}"
        separator: ","
    statement:
      required: false
    bullets:
      pattern: "^[^-]"
  sections:
    - name: 状況
      extract: sections.context
      statement:
        required: false
      bullets:
        repeat: { min: 0 }
        extract: sections.reasons
      table:
        header: [用語, 意味]
      codeblock:
        lang: gherkin
        lines: ["^@", "^Scenario:"]
      item:
        id: "REQ-\\d{3,}"
        repeat: { min: 0, max: 5 }
        fields:
          - name: 種類
            enum: [algorithm, ubiquitous]
          - name: 定義
            when: { field: 種類, eq: algorithm }
"#;

// @kotowari[REQ-schema-016, REQ-schema-017]
#[test]
fn loads_a_schema_with_all_rule_kinds() {
    let schema = parse_schema(ADR_LIKE).unwrap();
    assert_eq!(schema.name.as_deref(), Some("adr"));
    assert!(!schema.open);
    assert!(schema.document.title.is_some());
    let preamble = schema.document.preamble.as_ref().unwrap();
    assert_eq!(preamble.fields.len(), 2);
    assert_eq!(preamble.fields[0].name, "状態");
    assert!(preamble.statement.is_some());
    assert!(preamble.bullets.is_some());
    let section = &schema.document.sections[0];
    assert_eq!(section.name, "状況");
    assert!(section.table.is_some());
    assert!(section.codeblock.is_some());
    let item = section.item.as_ref().unwrap();
    assert_eq!(item.id.as_ref().map(|p| p.source()), Some("REQ-\\d{3,}"));
    assert_eq!(item.repeat.as_ref().unwrap().max, Some(5));
    assert_eq!(
        item.fields[1].when.as_ref().unwrap().eq.as_deref(),
        Some("algorithm")
    );
}

// @kotowari[REQ-schema-029]
#[test]
fn csv_is_a_shorthand_for_comma_separator() {
    let yaml = r#"
document:
  preamble:
    fields:
      - name: タグ
        csv: true
"#;
    let schema = parse_schema(yaml).unwrap();
    let field = &schema.document.preamble.unwrap().fields[0];
    assert_eq!(field.effective_separator(), Some(","));
}

// @kotowari[REQ-schema-035]
#[test]
fn extract_accepts_the_shorthand_and_the_named_group() {
    // 略記（`extract: <名前>`）は `{ path: <名前> }` と同じに読む（REQ-schema-048）
    let yaml = r#"
document:
  title:
    pattern: "^ADR-(?<id>\\d{4})"
    extract: { path: id, group: id }
  preamble:
    fields:
      - name: 状態
        extract: status
"#;
    let schema = parse_schema(yaml).unwrap();
    let title = schema.document.title.unwrap();
    let title_extract = title.extract.as_ref().unwrap();
    assert_eq!(title_extract.path(), "id");
    assert_eq!(title_extract.group(), Some("id"));
    let preamble = schema.document.preamble.unwrap();
    let field_extract = preamble.fields[0].extract.as_ref().unwrap();
    assert_eq!(field_extract.path(), "status");
    assert_eq!(field_extract.value(), None);
    assert!(field_extract.of().is_empty());
    assert_eq!(field_extract.group(), None);
}

// @kotowari[REQ-schema-018, EX-schema-008]
#[test]
fn unknown_key_is_an_error() {
    let yaml = "document:\n  bogus: 1\n";
    assert_rejected(yaml, "unknown field `bogus`");
}

// @kotowari[REQ-schema-018]
#[test]
fn type_violation_is_an_error() {
    let yaml = "open: not-a-bool\ndocument:\n  title:\n    pattern: x\n";
    assert_rejected(yaml, "invalid boolean");
}

// @kotowari[REQ-schema-019]
#[test]
fn min_greater_than_max_is_an_error() {
    let yaml = "document:\n  sections:\n    - name: x\n      repeat: { min: 3, max: 1 }\n";
    assert_rejected(yaml, "repeat min is greater than max");
}

// @kotowari[REQ-schema-019]
#[test]
fn negative_repeat_is_an_error() {
    let yaml = "document:\n  sections:\n    - name: x\n      repeat: { min: -1 }\n";
    assert_rejected(yaml, "invalid u64");
}

// @kotowari[REQ-schema-019]
#[test]
fn non_integer_repeat_is_an_error() {
    let yaml = "document:\n  sections:\n    - name: x\n      repeat: { min: 1.5 }\n";
    assert_rejected(yaml, "invalid u64");
}

// @kotowari[REQ-schema-020]
#[test]
fn when_with_both_operators_is_an_error() {
    let yaml = r#"
document:
  preamble:
    fields:
      - name: 定義
        when: { field: 種類, eq: a, ne: b }
"#;
    assert_rejected(yaml, "when must have exactly one of eq or ne");
}

// @kotowari[REQ-schema-020]
#[test]
fn when_with_no_operator_is_an_error() {
    let yaml = r#"
document:
  preamble:
    fields:
      - name: 定義
        when: { field: 種類 }
"#;
    assert_rejected(yaml, "when must have exactly one of eq or ne");
}

// @kotowari[REQ-schema-016]
#[test]
fn invalid_regex_pattern_is_an_error() {
    let yaml = "document:\n  title:\n    pattern: \"(\"\n";
    assert_rejected(yaml, "regex parse error");
}

// @kotowari[REQ-schema-016, REQ-schema-025]
#[test]
fn invalid_item_id_regex_is_an_error() {
    let yaml = "document:\n  sections:\n    - name: x\n      item:\n        id: \"(\"\n";
    assert_rejected(yaml, "regex parse error");
}

// @kotowari[REQ-schema-035]
#[test]
fn capture_extract_outside_title_is_schema_invalid() {
    // `group` の宣言（名前付きキャプチャ）は題名にだけ使える。題名以外のノードで
    // 宣言したときは schema_invalid の停止になる（TBL-schema-008）。
    let cases: &[(&str, &str)] = &[
        (
            "field",
            "document:\n  preamble:\n    fields:\n      - name: 状態\n        extract: { path: s, group: x }\n",
        ),
        (
            "statement",
            "document:\n  preamble:\n    statement:\n      extract: { path: s, group: x }\n",
        ),
        (
            "bullets",
            "document:\n  preamble:\n    bullets:\n      extract: { path: b, group: x }\n",
        ),
        (
            "section",
            "document:\n  sections:\n    - name: 状況\n      extract: { path: sec, group: x }\n",
        ),
        (
            "item",
            "document:\n  sections:\n    - name: 要求\n      item:\n        extract: { path: item, group: x }\n",
        ),
        (
            "table",
            "document:\n  sections:\n    - name: 用語集\n      table:\n        header: [a]\n        extract: { path: t, group: x }\n",
        ),
        (
            "codeblock",
            "document:\n  sections:\n    - name: コード\n      codeblock:\n        extract: { path: c, group: x }\n",
        ),
    ];
    for (node, yaml) in cases {
        // node に Capture 形の extract を宣言したのに schema_invalid にならない
        assert_rejected(
            yaml,
            &format!("{node} extract cannot use the named-group capture form"),
        );
    }
}

// @kotowari[REQ-schema-036, TBL-schema-009]
#[test]
fn an_empty_name_in_the_value_or_an_of_key_of_an_element_object_is_schema_invalid() {
    // "value" と "of" の鍵も同じ置き場の配置パスに数える（TBL-schema-009）ので、
    // ドットで区切った名前が空なら path と同じく schema_invalid にする
    let schema = |extract: &str| {
        format!(
            "document:\n  sections:\n    - name: 用語集\n      table:\n        header: [a]\n        extract: {extract}\n"
        )
    };
    assert!(parse_schema(&schema("{ path: g, value: v, of: { k: line } }")).is_ok());
    for extract in [
        "{ path: g, value: \"\" }",
        "{ path: g, value: \"a..b\" }",
        "{ path: g, of: { \"a..b\": line } }",
        "{ path: g, of: { \".k\": line } }",
    ] {
        let error = parse_schema(&schema(extract)).expect_err(extract);
        assert!(
            error.to_string().contains("has an empty name"),
            "{extract}: {error}"
        );
    }
}

// @kotowari[REQ-schema-039]
#[test]
fn item_internal_extract_without_item_extract_is_schema_invalid() {
    let yaml = r#"
document:
  sections:
    - name: 要求
      item:
        fields:
          - name: 種類
            extract: kind
"#;
    assert_rejected(
        yaml,
        "item internals declare extract but the item itself does not",
    );
}

// @kotowari[REQ-schema-039]
#[test]
fn item_child_field_extract_without_item_extract_is_schema_invalid() {
    // item の bullets.children.fields も「項目の内部のフィールド行」なので、
    // extract を宣言すると schema_invalid（REQ-schema-039）。
    let yaml = r#"
document:
  sections:
    - name: 要求
      item:
        bullets:
          repeat: { min: 0 }
          children:
            fields:
              - name: superseded_by
                extract: superseded_by
"#;
    // item の bullets.children.fields に extract を宣言すると schema_invalid（REQ-schema-039）
    assert_rejected(
        yaml,
        "item internals declare extract but the item itself does not",
    );
}

// @kotowari[REQ-schema-039]
#[test]
fn item_deep_child_field_extract_without_item_extract_is_schema_invalid() {
    // 再帰的な入れ子（item の bullets.children.bullets.children.fields）も
    // 項目の内部のフィールド行なので schema_invalid（REQ-schema-039）。
    let yaml = r#"
document:
  sections:
    - name: 要求
      item:
        bullets:
          repeat: { min: 0 }
          children:
            bullets:
              repeat: { min: 0 }
              children:
                fields:
                  - name: 補足
                    extract: note
"#;
    // item の入れ子の子フィールドに extract を宣言すると schema_invalid（REQ-schema-039）
    assert_rejected(
        yaml,
        "item internals declare extract but the item itself does not",
    );
}

// @kotowari[REQ-schema-035]
#[test]
fn capture_extract_without_pattern_is_schema_invalid() {
    // `group` を宣言した抽出は pattern の名前付きキャプチャを取る。pattern が無い
    // 題名は schema_invalid の停止になる（REQ-schema-048）。
    let yaml = r#"
document:
  title:
    extract: { path: id, group: id }
"#;
    assert_rejected(yaml, "title capture extract requires a pattern");
}

// @kotowari[REQ-schema-035]
#[test]
fn capture_extract_with_pattern_missing_the_group_is_schema_invalid() {
    // pattern が指定の名前付きキャプチャを含まない題名も schema_invalid（TBL-schema-008）。
    let yaml = r#"
document:
  title:
    pattern: "^ADR-(?<other>\\d{4}):"
    extract: { path: id, group: id }
"#;
    assert_rejected(
        yaml,
        "title pattern does not contain a named capture group \"id\"",
    );
}

// @kotowari[REQ-schema-016]
#[test]
fn not_yaml_is_an_error() {
    // 鍵も値の型も正しく、YAML の構文だけが壊れている
    assert_rejected("name: \"unterminated", "unclosed quote");
}

// @kotowari[REQ-schema-031]
#[test]
fn bullets_with_children_loads() {
    let yaml = r#"
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
          bullets:
            repeat: { min: 0 }
            children:
              fields:
                - name: 補足
"#;
    let schema = parse_schema(yaml).unwrap();
    let bullets = schema.document.sections[0].bullets.as_ref().unwrap();
    let children = bullets.children.as_ref().unwrap();
    assert_eq!(children.fields[0].name, "superseded_by");
    let child_bullets = children.bullets.as_ref().unwrap();
    assert!(
        child_bullets.children.as_ref().unwrap().fields[0].name == "補足",
        "children.bullets が再帰的に children を持てる（任意の深さ）"
    );
}

// @kotowari[REQ-schema-031]
#[test]
fn item_bullets_can_have_children() {
    let yaml = r#"
document:
  sections:
    - name: 要求
      item:
        bullets:
          repeat: { min: 0 }
          children:
            fields:
              - name: superseded_by
"#;
    let schema = parse_schema(yaml).unwrap();
    let bullets = schema.document.sections[0]
        .item
        .as_ref()
        .unwrap()
        .bullets
        .as_ref()
        .unwrap();
    assert!(bullets.children.is_some());
}

// @kotowari[REQ-schema-018]
#[test]
fn unknown_key_under_bullets_is_an_error() {
    let yaml = "document:\n  sections:\n    - name: 理由\n      bullets:\n        bogus: 1\n";
    assert_rejected(yaml, "unknown field `bogus`");
}

// @kotowari[REQ-schema-018]
#[test]
fn type_violation_under_children_is_an_error() {
    let yaml =
        "document:\n  sections:\n    - name: 理由\n      bullets:\n        children: not-a-map\n";
    assert_rejected(yaml, "expected mapping");
}

// @kotowari[REQ-schema-035]
#[test]
fn children_bullets_with_extract_is_schema_invalid() {
    let yaml = r#"
document:
  sections:
    - name: 決定
      bullets:
        children:
          bullets:
            extract: nested
"#;
    // children.bullets に extract を宣言すると schema_invalid（TBL-schema-008）
    assert_rejected(yaml, "children bullets cannot declare extract");
}

// @kotowari[REQ-schema-035]
#[test]
fn deep_children_bullets_with_extract_is_schema_invalid() {
    let yaml = r#"
document:
  sections:
    - name: 決定
      bullets:
        children:
          bullets:
            children:
              bullets:
                extract: nested
"#;
    // 再帰的な children.bullets にも extract を宣言できない（TBL-schema-008）
    assert_rejected(yaml, "children bullets cannot declare extract");
}

// @kotowari[REQ-schema-035, REQ-schema-036]
#[test]
fn child_field_can_have_extract() {
    let yaml = r#"
document:
  sections:
    - name: 決定
      bullets:
        children:
          fields:
            - name: superseded_by
              extract: superseded_by
"#;
    assert!(
        parse_schema(yaml).is_ok(),
        "子フィールドは自分の extract を持つことができる（REQ-schema-036）"
    );
}

// @kotowari[REQ-schema-039, EX-schema-014]
#[test]
fn item_table_extract_without_item_extract_is_rejected() {
    let yaml = r#"
document:
  sections:
    - name: 決定表
      item:
        table:
          header: [用語, 意味]
          extract: glossary
"#;
    // 項目が extract を持たないまま内部の表に宣言すると停止する（REQ-schema-039）
    assert_rejected(
        yaml,
        "item internals declare extract but the item itself does not",
    );
}

// @kotowari[REQ-schema-039]
#[test]
fn item_codeblock_extract_without_item_extract_is_rejected() {
    let yaml = r#"
document:
  sections:
    - name: 具体例
      item:
        codeblock:
          lang: gherkin
          extract: scenarios
"#;
    // 項目が extract を持たないまま内部のコードブロックに宣言すると停止する（REQ-schema-039）
    assert_rejected(
        yaml,
        "item internals declare extract but the item itself does not",
    );
}

// @kotowari[REQ-schema-019]
#[test]
fn repeat_with_equal_min_and_max_is_accepted() {
    let yaml = r#"
document:
  sections:
    - name: 決定表
      item:
        table:
          repeat: { min: 1, max: 1 }
"#;
    assert!(
        parse_schema(yaml).is_ok(),
        "下限と上限が等しい範囲はちょうどその個数で、形に反しない"
    );
}

// @kotowari[REQ-schema-019]
#[test]
fn item_table_invalid_repeat_is_rejected() {
    let yaml = r#"
document:
  sections:
    - name: 決定表
      item:
        table:
          header: [用語, 意味]
          repeat: { min: 2, max: 1 }
"#;
    // 項目の表の repeat も min > max なら停止する（REQ-schema-019）
    assert_rejected(yaml, "repeat min is greater than max");
}

// @kotowari[REQ-schema-019]
#[test]
fn item_codeblock_invalid_repeat_is_rejected() {
    let yaml = r#"
document:
  sections:
    - name: 具体例
      item:
        codeblock:
          lang: gherkin
          repeat: { min: 2, max: 1 }
"#;
    // 項目のコードブロックの repeat も min > max なら停止する（REQ-schema-019）
    assert_rejected(yaml, "repeat min is greater than max");
}

// @kotowari[REQ-schema-035, REQ-schema-048]
#[test]
fn of_id_outside_an_item_is_rejected() {
    let yaml = r#"
document:
  sections:
    - name: 記録
      fields:
        - name: 状態
          extract: { path: id, of: { id: id } }
"#;
    // of: id は項目にだけ宣言できる（REQ-schema-048）
    assert_rejected(yaml, "field extract cannot use \"id\" in of");
}

// @kotowari[REQ-schema-035, REQ-schema-048]
#[test]
fn unknown_of_value_is_rejected() {
    let yaml = r#"
document:
  sections:
    - name: 記録
      fields:
        - name: 状態
          extract: { path: x, of: { x: column } }
"#;
    // of の知らない語は停止する（REQ-schema-048）
    assert_rejected(yaml, "unknown variant `column`");
}

// @kotowari[REQ-schema-039, REQ-schema-047]
#[test]
fn item_internal_extract_with_item_extract_loads() {
    let yaml = r#"
document:
  sections:
    - name: 要求
      item:
        extract:
          path: requirements
          of: { id: id }
        fields:
          - name: 種類
            extract: kind
        table:
          extract: rows
"#;
    assert!(
        parse_schema(yaml).is_ok(),
        "項目が extract を持てば内部にも宣言できる（REQ-schema-039）"
    );
}

// @kotowari[REQ-schema-035, REQ-schema-048]
#[test]
fn nested_extract_declaration_loads() {
    // 抽出の宣言は path・value・of・group を持つ1つの入れ子である（REQ-schema-048）
    let yaml = r#"
document:
  sections:
    - name: 用語集
      table:
        header: [用語, 意味]
        extract: { path: glossary, value: cells, of: { line: line } }
"#;
    let schema = parse_schema(yaml).unwrap();
    let extract = schema.document.sections[0]
        .table
        .as_ref()
        .unwrap()
        .extract
        .as_ref()
        .unwrap();
    assert_eq!(extract.path(), "glossary");
    assert_eq!(extract.value(), Some("cells"));
    assert_eq!(
        extract.of(),
        [("line".to_string(), OfKind::Line)],
        "of は鍵から導かれる値の語への対応として読む（REQ-schema-048）"
    );
}

// @kotowari[REQ-schema-035]
#[test]
fn legacy_sequence_extract_is_schema_invalid() {
    // 書式の並びは受けない（REQ-schema-048）
    let yaml = r#"
document:
  sections:
    - name: 記録
      fields:
        - name: 状態
          extract:
            - status
            - { path: status_line, of: { line: line } }
"#;
    // 旧来の書式の並びは schema_invalid の停止になる（REQ-schema-048）
    assert_rejected(yaml, "expected an extract path, or a mapping with \"path\"");
}

// @kotowari[REQ-schema-035]
#[test]
fn extract_without_a_path_is_schema_invalid() {
    let yaml = r#"
document:
  sections:
    - name: 記録
      fields:
        - name: 状態
          extract: { of: { status_line: line } }
"#;
    // 配置パスの無い宣言は schema_invalid の停止になる（REQ-schema-048）
    assert_rejected(yaml, "extract requires \"path\"");
}

// @kotowari[REQ-schema-042, REQ-schema-048]
#[test]
fn duplicate_key_between_an_inner_node_and_a_derived_value_is_schema_invalid() {
    // 項目のオブジェクトの中で、内側のフィールド行の配置パスと外側の
    // 導かれる値の鍵が重なるスキーマは、文書を読まずに停止する（TBL-schema-009）
    let yaml = r#"
document:
  sections:
    - name: 要求
      item:
        extract: { path: requirements, of: { line: line } }
        fields:
          - name: 状態
            extract: line
"#;
    // 要素オブジェクトの中で鍵が重複したら停止する（TBL-schema-009）
    assert_rejected(yaml, "has the key \"line\" twice");
}

// @kotowari[REQ-schema-042, REQ-schema-048]
#[test]
fn duplicate_key_between_value_and_a_derived_value_is_schema_invalid() {
    let yaml = r#"
document:
  sections:
    - name: 用語集
      table:
        header: [a, b]
        extract: { path: glossary, value: cells, of: { cells: line } }
"#;
    // 要素の値の鍵と導かれる値の鍵が重複したら停止する（TBL-schema-009）
    assert_rejected(yaml, "has the key \"cells\" twice");
}

// @kotowari[REQ-schema-042, REQ-schema-048]
#[test]
fn a_parent_key_is_a_duplicate_but_a_sibling_key_is_not() {
    // 判定は配置パス全体で行う。a と a.b は親子で重複、a.b と a.c は別の鍵（TBL-schema-009）
    let parent_and_child = r#"
document:
  sections:
    - name: 用語集
      table:
        header: [x]
        extract: { path: glossary, value: a, of: { "a.b": line } }
"#;
    let siblings = r#"
document:
  sections:
    - name: 用語集
      table:
        header: [x]
        extract: { path: glossary, value: a.b, of: { "a.c": line } }
"#;
    // a と a.b は片方が他方の親なので重複（TBL-schema-009）
    assert_rejected(parent_and_child, "has the key \"a\" twice");
    assert!(
        parse_schema(siblings).is_ok(),
        "a.b と a.c は入れ子を共有するだけの別の鍵（TBL-schema-009）"
    );
}
