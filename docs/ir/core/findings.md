# 検査の種類

指摘の種類ごとの重大度と detail の中身を扱う。種類ごとの条件は右の列の要求が決める。

## 要求

### REQ-core-029: 誤りの種類

- 種類: algorithm
- 出典: docs/decision/records/records.md#A21, docs/decision/records/records.md#A29, docs/decision/records/ir-form.md#検査の種類
- 定義: TBL-core-008
- 検証: unit

### REQ-core-030: 注意の種類

- 種類: algorithm
- 出典: docs/decision/records/records.md#A29, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-16-notice.md#A2
- 定義: TBL-core-009
- 検証: unit

### REQ-core-031: 注意は4つだけ

- 種類: invariant
- 出典: docs/decision/records/records.md#A29, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-16-notice.md#A2, docs/decision/records/2026-09-17-mutation-tests.md#A11, docs/decision/records/2026-09-17-mutation-tests.md#A20, docs/decision/records/2026-09-17-mutation-tests.md#A31, docs/decision/records/2026-09-17-mutation-tests.md#A43
- 検証: unit

種類が too_many_lines、too_many_requirements、mutant_timeout、equivalent_stale の`指摘`だけが`注意`で、ほかの種類の`指摘`はすべて`誤り`である関係が常に成り立つ。

### REQ-core-032: ID の重複

- 種類: event_driven
- 出典: docs/decision/records/records.md#A47, docs/decision/records/records.md#A61, docs/decision/records/records.md#A72, docs/decision/records/records.md#A113
- 検証: unit

同じ`ID`が2か所以上にあるとき、kotowari は2つ目以降の場所ごとに、その見出しの行（`シナリオ`は "Scenario:" の行）を "line" にして duplicate_id の`誤り`を出す。1つ目はパスのバイト順で先の文書、同じ文書の中では行の小さいものである。

## 決定表

### TBL-core-008: 誤りの種類と detail

- 出典: docs/decision/records/records.md#A142, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A68, docs/decision/records/records.md#A108, docs/decision/records/records.md#A109, docs/decision/records/records.md#A110, docs/decision/records/records.md#A112, docs/decision/records/records.md#A116, docs/decision/records/records.md#A111, docs/decision/records/records.md#A150, docs/decision/records/records.md#A153, docs/decision/records/records.md#A154, docs/decision/records/2026-09-16-ir-tree.md#A5, docs/decision/records/2026-09-16-ir-tree.md#A19, docs/decision/records/2026-09-17-record-form.md#A7, docs/decision/records/2026-09-17-record-form.md#A12, docs/decision/records/2026-09-17-record-form.md#A32, docs/decision/records/2026-09-17-record-form.md#A25, docs/decision/records/2026-09-17-scenario-tests.md#A6, docs/decision/records/2026-09-17-mutation-tests.md#A31, docs/decision/records/2026-09-17-mutation-tests.md#A40, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-17-mutation-tests.md#A57, docs/decision/records/2026-09-22-id-namespace.md#A4

detail が「行の文字」「見出しの文字」「Scenario: の行の文字」の種類では、読んだ行の文字そのまま（字下げと末尾の空白を含み、再構成しない）を入れる。

| 種類 | detail | 条件を定める要求 |
|---|---|---|
| missing_title | 文書名（ディレクトリを除いたファイル名） | REQ-core-034 |
| multiple_titles | 2つ目の題名 | REQ-core-035 |
| missing_scope | 文書名（ディレクトリを除いたファイル名） | REQ-core-036 |
| unknown_heading | 見出しの文字 | REQ-core-043 |
| unknown_field | 行の文字 | REQ-core-044 |
| missing_field | 行の名前 | REQ-core-098 |
| missing_table | 決定表の ID | REQ-core-099 |
| duplicate_field | 行の名前 | REQ-core-045 |
| missing_source | 項目の ID か用語 | REQ-core-059 |
| source_invalid | 出典の文字列 | REQ-core-058 |
| unknown_term | 囲んだ文字列 | REQ-core-064、REQ-core-065 |
| missing_document | 文書名の参照の文字列 | REQ-core-070 |
| missing_statement | 項目の ID | REQ-core-047 |
| verification_missing | 要求の ID | REQ-core-048 |
| verification_invalid | 値 | REQ-core-049 |
| unknown_kind | 値 | REQ-core-050 |
| duplicate_id | ID | REQ-core-032 |
| unresolved_reference | ID | REQ-core-054 |
| algorithm_without_definition | 要求の ID | REQ-core-051 |
| missing_tag | 無いタグの名前 | REQ-core-053 |
| unknown_tag | タグの名前 | REQ-core-052 |
| vague_word | 語 | REQ-core-066 |
| requirement_without_test | 要求の ID | REQ-core-085 |
| scenario_without_test | シナリオの ID | REQ-core-137 |
| test_without_id | 関数の名前 | REQ-core-086 |
| invalid_marker | 行の文字 | REQ-core-072 |
| unparsable_file | ファイルのパス | REQ-core-083 |
| unclosed_code_block | 開始の行の文字 | REQ-core-112 |
| invalid_gherkin_line | 行の文字 | REQ-core-113 |
| invalid_id | 値 | REQ-core-114 |
| id_domain_mismatch | ID | REQ-core-167 |
| glossary_invalid | 文書名（ディレクトリを除いたファイル名） | REQ-core-117 |
| unclosed_backtick | 行の文字 | REQ-core-116 |
| invalid_glossary_row | 行の文字 | REQ-core-122 |
| duplicate_term | 用語 | REQ-core-123 |
| record_field_missing | 無い補足の行の名前 | REQ-core-130 |
| record_field_unknown | 補足の行の名前 | REQ-core-131 |
| revision_link_invalid | リンクの href。リンクが無ければ superseded_by の行の値 | REQ-core-132 |
| mutant_survived | 変更の説明 | REQ-core-139 |
| equivalent_invalid | 等価の一覧の1件に書かれたままの "file" と "change" を ": " でつないだ文字列 | REQ-core-143 |

### TBL-core-009: 注意の種類と detail

- 出典: docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-16-notice.md#A2, docs/decision/records/2026-09-17-mutation-tests.md#A31, docs/decision/records/2026-09-17-mutation-tests.md#A40, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-17-mutation-tests.md#A57

| 種類 | detail | 条件を定める要求 |
|---|---|---|
| too_many_lines | 行数 | REQ-core-038 |
| too_many_requirements | 要求の数 | REQ-core-039 |
| mutant_timeout | 変更の説明 | REQ-core-140 |
| equivalent_stale | 等価の一覧の1件に書かれたままの "file" と "change" を ": " でつないだ文字列 | REQ-core-142 |

## 具体例

```gherkin
@id=EX-core-005 @about=REQ-core-032 @source=docs/decision/records/records.md#A61,docs/decision/records/records.md#A47
Scenario: 3か所にある ID は2件の重複になる
  Given "REQ-001" の見出しが3か所にある
  When "kotowari check" を実行する
  Then duplicate_id の誤りが2件出る
  And 1つ目の見出しの行には duplicate_id が出ない
```
