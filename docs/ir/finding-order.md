# 指摘の並べ方と行

指摘の並べ方と、指摘の "line" の決め方を扱う。

## 要求

### REQ-024: 指摘の並べ方

- 種類: algorithm
- 出典: docs/decision/records/records.md#A61, docs/decision/records/records.md#A70
- 定義: TBL-007, PROP-003
- 検証: property

### REQ-027: 文書全体への指摘

- 種類: ubiquitous
- 出典: docs/decision/records/records.md#A40, docs/decision/records/records.md#A83, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A112, docs/decision/records/records.md#A144
- 検証: unit

kotowari は常に、種類が missing_title、multiple_titles、missing_scope、too_many_lines、too_many_requirements、unparsable_file、glossary_invalid の`指摘`の "line" を null にし、ほかの種類の "line" を TBL-019 のとおりにする。

### REQ-028: 行は1始まり

- 種類: ubiquitous
- 出典: docs/decision/records/records.md#A61
- 検証: unit

kotowari は常に、`指摘`の "line" を1始まりで数える。

## 決定表

### TBL-007: findings の並べ方

- 出典: docs/decision/records/records.md#A61, docs/decision/records/records.md#A70

| 順 | 鍵 | 並べ方 |
|---|---|---|
| 1 | path | バイト順 |
| 2 | line | null が先、その後は小さい順 |
| 3 | kind | バイト順 |
| 4 | detail | バイト順 |

### TBL-019: 指摘の行

- 出典: docs/decision/records/records.md#A144, docs/decision/records/records.md#A114, docs/decision/records/records.md#A121, docs/decision/records/records.md#A139, docs/decision/records/records.md#A108, docs/decision/records/records.md#A61, docs/decision/records/records.md#A72, docs/decision/records/records.md#A153, docs/decision/records/records.md#A154, docs/decision/records/2026-09-16-ir-tree.md#A4

| 種類 | line |
|---|---|
| missing_source | 項目の見出しの行。シナリオはタグの行（無ければ "Scenario:" の行）。用語は表の行 |
| source_invalid | 出典が書かれた行（REQ-115） |
| missing_tag、unknown_tag、invalid_id | タグの行（無ければ "Scenario:" の行） |
| unknown_term、vague_word、unclosed_backtick、missing_document | その行 |
| unknown_field、duplicate_field、unknown_heading、invalid_gherkin_line | その行 |
| unclosed_code_block | 開始の行 |
| invalid_glossary_row | その行 |
| duplicate_term | 重複した側の用語の行（同じ用語集なら2つ目以降、連鎖では根から遠い側） |
| duplicate_id | 2つ目以降の見出しの行（REQ-032） |
| unresolved_reference、invalid_marker | 印なら印のある行（REQ-118）。定義・関係・文の中なら その行。"@about" ならタグの行 |
| missing_field、missing_statement、missing_table、verification_missing、verification_invalid、unknown_kind、algorithm_without_definition、requirement_without_test | 項目の見出しの行 |
| test_without_id | 関数の宣言の行 |

## 性質

### PROP-003: findings は並んでいる

- 出典: docs/decision/records/records.md#A61, docs/decision/records/records.md#A70, docs/decision/records/ir-form.md#出力

"findings" の中で隣り合うどの2つの`指摘`も、TBL-007 の順で比べて後ろのものが前のものより先に来ない。
