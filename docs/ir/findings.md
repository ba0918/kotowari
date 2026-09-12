# 検査の種類

指摘の種類ごとの重大度と detail の中身を扱う。種類ごとの条件は右の列の要求が決める。

## 要求

### REQ-029: 誤りの種類

- 種類: algorithm
- 出典: brainstorm/records.md#A21, brainstorm/records.md#A29, brainstorm/ir-form.md#検査の種類
- 定義: TBL-008
- 検証: unit

### REQ-030: 警告の種類

- 種類: algorithm
- 出典: brainstorm/records.md#A29, brainstorm/ir-form.md#検査の種類
- 定義: TBL-009
- 検証: unit

### REQ-031: 警告は2つだけ

- 種類: invariant
- 出典: brainstorm/records.md#A29, brainstorm/ir-form.md#検査の種類
- 検証: unit

種類が too_many_lines か too_many_requirements の`指摘`だけが`警告`で、ほかの種類の`指摘`はすべて`誤り`である関係が常に成り立つ。

### REQ-032: ID の重複

- 種類: event_driven
- 出典: brainstorm/records.md#A47, brainstorm/records.md#A61, brainstorm/records.md#A72
- 検証: unit

同じ`ID`が2か所以上にあるとき、kotowari は2つ目以降の場所ごとに、その見出しの行（`シナリオ`は "Scenario:" の行）を "line" にして duplicate_id の`誤り`を出す。

## 決定表

### TBL-008: 誤りの種類と detail

- 出典: brainstorm/ir-form.md#検査の種類, brainstorm/records.md#A68

| 種類 | detail | 条件を定める要求 |
|---|---|---|
| missing_title | 文書名 | REQ-034 |
| multiple_titles | 2つ目の題名 | REQ-035 |
| missing_scope | 文書名 | REQ-036 |
| unknown_heading | 見出しの文字 | REQ-043 |
| unknown_field | 行の文字 | REQ-044 |
| missing_field | 行の名前 | REQ-098 |
| missing_table | 決定表の ID | REQ-099 |
| duplicate_field | 行の名前 | REQ-045 |
| missing_source | 項目の ID か用語 | REQ-059 |
| source_invalid | 出典の文字列 | REQ-058 |
| unknown_term | 囲んだ文字列 | REQ-064、REQ-065 |
| missing_document | 文書名 | REQ-070 |
| missing_statement | 項目の ID | REQ-047 |
| verification_missing | 要求の ID | REQ-048 |
| verification_invalid | 値 | REQ-049 |
| unknown_kind | 値 | REQ-050 |
| duplicate_id | ID | REQ-032 |
| unresolved_reference | ID | REQ-054 |
| algorithm_without_definition | 要求の ID | REQ-051 |
| missing_tag | 無いタグの名前 | REQ-053 |
| unknown_tag | タグの名前 | REQ-052 |
| vague_word | 語 | REQ-066 |
| requirement_without_test | 要求の ID | REQ-085 |
| test_without_id | 関数の名前 | REQ-086 |
| invalid_marker | 行の文字 | REQ-072 |
| unparsable_file | ファイルのパス | REQ-083 |

### TBL-009: 警告の種類と detail

- 出典: brainstorm/ir-form.md#検査の種類

| 種類 | detail | 条件を定める要求 |
|---|---|---|
| too_many_lines | 行数 | REQ-038 |
| too_many_requirements | 要求の数 | REQ-039 |

## 具体例

```gherkin
@id=EX-005 @about=REQ-032 @source=brainstorm/records.md#A61,brainstorm/records.md#A47
Scenario: 3か所にある ID は2件の重複になる
  Given "REQ-001" の見出しが3か所にある
  When "kotowari check" を実行する
  Then duplicate_id の誤りが2件出る
  And 1つ目の見出しの行には duplicate_id が出ない
```
