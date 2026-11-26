//! 性質テスト。例を並べても網羅にならない要求を、生成した入力で確かめる。
#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]

use kotowari_markdown_schema::{Document, Schema, ValidationOptions};
fn extract_values(schema: &Schema, document: &Document) -> serde_json::Value {
    kotowari_markdown_schema::extract_partial(schema, document, ValidationOptions::default())
        .values()
        .clone()
}
fn parse_schema(yaml: &str) -> Result<Schema, kotowari_markdown_schema::SchemaError> {
    Schema::parse(yaml)
}
fn validate(
    schema: &Schema,
    document: &Document,
    relax: bool,
) -> Vec<kotowari_markdown_schema::finding::Finding> {
    kotowari_markdown_schema::validate(schema, document, ValidationOptions { relax })
}
use proptest::prelude::*;
use serde_json::Value;

/// 宣言済みのフィールド行の名前と、宣言していない名前。
const DECLARED: [&str; 2] = ["状態", "日付"];
const UNDECLARED: [&str; 2] = ["備考", "補足"];
const MARKERS: [&str; 3] = ["-", "*", "+"];

/// 箇条書きを宣言しないので、宣言していない名前の行は undeclared_line になる。
/// 宣言した名前の行は値のパターンに照合され、合わなければ field_pattern_mismatch になる。
/// どちらの判定もマーカーに依存してはならない。
const MARKER_SCHEMA: &str = r#"
document:
  sections:
    - name: 記録
      fields:
        - name: 状態
          required: false
          pattern: "^ok"
        - name: 日付
          required: false
          pattern: "^ok"
"#;

fn line_names() -> impl Strategy<Value = Vec<usize>> {
    proptest::collection::vec(0usize..4, 1..6)
}

fn name_at(index: usize) -> &'static str {
    if index < DECLARED.len() {
        DECLARED[index]
    } else {
        UNDECLARED[index - DECLARED.len()]
    }
}

fn document_with(markers: &[usize], names: &[usize], values: &[&str]) -> String {
    let mut out = String::from("## 記録\n\n");
    for (i, (marker, name)) in markers.iter().zip(names).enumerate() {
        out.push_str(&format!(
            "{} {}: {}\n",
            MARKERS[*marker],
            name_at(*name),
            values[i % values.len()]
        ));
    }
    out
}

fn findings_of(schema: &Schema, source: &str) -> Vec<(String, Option<usize>, String)> {
    let document = Document::parse(source).unwrap();
    validate(schema, &document, false)
        .into_iter()
        .map(|f| (format!("{:?}", f.kind()), f.line(), f.detail().to_owned()))
        .collect()
}

proptest! {
    /// REQ-schema-028: 一覧のマーカーが "-"、"*"、"+" のどれであっても読み分けは変わらない。
    // @kotowari[REQ-schema-028, C-schema-block-2]
    #[test]
    fn markers_do_not_change_how_list_lines_are_read(
        names in line_names(),
        values in proptest::collection::vec(prop::sample::select(&["ok", "ng"][..]), 1..4),
    ) {
        let schema = parse_schema(MARKER_SCHEMA).unwrap();
        let hyphens = vec![0usize; names.len()];
        let expected = findings_of(&schema, &document_with(&hyphens, &names, &values));

        // 生成した入力が判定を踏んでいることを確かめる（空振りする性質テストにしない）。
        // values は行ごとに順に使い回すので、実際に使われた値だけを見る
        let used: Vec<&str> = (0..names.len()).map(|i| values[i % values.len()]).collect();
        let touches_classification =
            names.iter().any(|n| *n >= DECLARED.len()) || used.contains(&"ng");
        prop_assert!(
            !touches_classification || !expected.is_empty(),
            "判定を踏む入力なのに指摘が0件だった: {:?} {:?}",
            names,
            values
        );

        for (marker, symbol) in MARKERS.iter().enumerate().skip(1) {
            let markers = vec![marker; names.len()];
            let actual = findings_of(&schema, &document_with(&markers, &names, &values));
            prop_assert_eq!(&actual, &expected, "マーカー {} で読み分けが変わった", symbol);
        }
    }
}

proptest! {
    /// REQ-schema-028: マーカーを1行ごとに混ぜても、すべて "-" のときと読み分けは変わらない。
    // @kotowari[REQ-schema-028, C-schema-block-2]
    #[test]
    fn mixed_markers_read_the_same_as_hyphens(
        names in line_names(),
        values in proptest::collection::vec(prop::sample::select(&["ok", "ng"][..]), 1..4),
        seed in proptest::collection::vec(0usize..MARKERS.len(), 1..6),
    ) {
        let markers: Vec<usize> = (0..names.len()).map(|i| seed[i % seed.len()]).collect();
        let schema = parse_schema(MARKER_SCHEMA).unwrap();
        let hyphens = vec![0usize; names.len()];
        let expected = findings_of(&schema, &document_with(&hyphens, &names, &values));
        let actual = findings_of(&schema, &document_with(&markers, &names, &values));
        prop_assert_eq!(actual, expected);
    }
}

const EXTRACT_SCHEMA: &str = r#"
document:
  title:
    pattern: "^.+"
    extract: title
  sections:
    - name: 記録
      fields:
        - name: 値
          repeat: { min: 0 }
          extract: values
        - name: 並び
          required: false
          separator: ","
          extract: list
      statement:
        required: false
        extract: body
      table:
        required: false
        extract: rows
"#;

/// JSON の葉がすべて文字列であることを確かめる。
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "serde_json::Value is a foreign enum: the remaining JSON kinds are deliberately handled alike"
)]
fn every_leaf_is_a_string(value: &Value) -> bool {
    match value {
        Value::String(_) => true,
        Value::Array(items) => items.iter().all(every_leaf_is_a_string),
        Value::Object(map) => map.values().all(every_leaf_is_a_string),
        _ => false,
    }
}

proptest! {
    /// REQ-schema-037: 数や日付に見える値でも、抽出した値は文字列のままである。
    // @kotowari[REQ-schema-037, C-schema-extract-1]
    #[test]
    fn extracted_leaves_are_always_strings(
        values in proptest::collection::vec("[0-9A-Za-z.:+-]{1,12}", 1..6),
        list in proptest::collection::vec("[0-9A-Za-z.:+-]{0,8}", 1..4),
        cell in "[0-9A-Za-z.:+-]{1,12}",
    ) {
        let mut doc = String::from("# 題名\n\n## 記録\n\n");
        for value in &values {
            doc.push_str(&format!("- 値: {value}\n"));
        }
        doc.push_str(&format!("- 並び: {}\n", list.join(",")));
        doc.push_str("\n本文。\n\n| 列 |\n|---|\n");
        doc.push_str(&format!("| {cell} |\n"));

        let schema = parse_schema(EXTRACT_SCHEMA).unwrap();
        let document = Document::parse(&doc).unwrap();
        let extracted = extract_values(&schema, &document);
        prop_assert!(
            every_leaf_is_a_string(&extracted),
            "文字列でない葉があった: {extracted}"
        );
    }
}

/// 開いた世界と閉じた世界で読み比べるスキーマ。題名、前置部、節を宣言する
const WORLD_SCHEMA: &str = r#"
document:
  title:
    pattern: "^題名"
    extract: title
  preamble:
    fields:
      - name: 状態
        required: false
        pattern: "^ok"
        extract: status
  sections:
    - name: 記録
      fields:
        - name: 値
          pattern: "^ok"
          extract: value
        - name: 並び
          required: false
          separator: ","
          extract: list
      statement:
        required: false
        extract: body
"#;

/// 文書を組む断片。宣言したもの、宣言していないもの、出現回数と形に反するものを混ぜる
const WORLD_PIECES: [&str; 12] = [
    "# 題名",
    "# 別の題名",
    "## 記録",
    "## 雑記",
    "### 小見出し",
    "- 状態: ok",
    "- 値: ok",
    "- 値: ng",
    "- 並び: a,b",
    "- 備考: x",
    "本文。",
    "| 列 |\n|---|\n| c |",
];

fn world_document(pieces: &[usize]) -> String {
    pieces
        .iter()
        .map(|i| WORLD_PIECES[*i])
        .collect::<Vec<_>>()
        .join("\n\n")
        + "\n"
}

/// 指摘の (種類, 行, 詳細) ごとの数
fn finding_counts(
    schema: &Schema,
    document: &Document,
    relax: bool,
) -> std::collections::BTreeMap<(String, Option<usize>, String), usize> {
    let mut counts = std::collections::BTreeMap::new();
    for f in validate(schema, document, relax) {
        *counts
            .entry((format!("{:?}", f.kind()), f.line(), f.detail().to_owned()))
            .or_insert(0) += 1;
    }
    counts
}

proptest! {
    /// 開いた世界の指摘は、同じ文書を閉じた世界で検査した指摘にすべて含まれる。
    // @kotowari[C-schema-closed-5]
    #[test]
    fn open_world_findings_are_contained_in_closed_world_findings(
        pieces in proptest::collection::vec(0usize..WORLD_PIECES.len(), 0..10),
    ) {
        let schema = parse_schema(WORLD_SCHEMA).unwrap();
        let document = Document::parse(&world_document(&pieces)).unwrap();
        let open = finding_counts(&schema, &document, true);
        let closed = finding_counts(&schema, &document, false);
        for (finding, count) in &open {
            prop_assert!(
                closed.get(finding).is_some_and(|c| c >= count),
                "開いた世界だけの指摘 {:?}: open {:?} closed {:?}",
                finding,
                open,
                closed
            );
        }
        // 生成した入力が緩める判定を踏んでいることを確かめる（空振りする性質テストにしない）
        if pieces.contains(&3) {
            prop_assert!(
                open.values().sum::<usize>() < closed.values().sum::<usize>(),
                "宣言していない節があるのに閉じた世界の指摘が増えなかった: {:?}",
                pieces
            );
        }
    }
}

proptest! {
    /// 抽出の結果は、スキーマの "open" が真でも偽でも等しい。
    // @kotowari[C-schema-extract-43]
    #[test]
    fn extraction_is_the_same_whether_the_schema_is_open_or_closed(
        pieces in proptest::collection::vec(0usize..WORLD_PIECES.len(), 0..10),
    ) {
        let closed = parse_schema(&format!("open: false\n{WORLD_SCHEMA}")).unwrap();
        let open = parse_schema(&format!("open: true\n{WORLD_SCHEMA}")).unwrap();
        let document = Document::parse(&world_document(&pieces)).unwrap();
        let closed_values = extract_values(&closed, &document);
        prop_assert_eq!(&extract_values(&open, &document), &closed_values);
        // 生成した入力が抽出を踏んでいることを確かめる
        if pieces.contains(&0) {
            prop_assert!(closed_values.get("title").is_some(), "{}", closed_values);
        }
    }
}
