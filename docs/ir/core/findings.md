# 検査の種類

指摘の種類ごとの重大度と detail の中身を扱う。種類ごとの条件は右の列の要求が決める。

## Requirements

### REQ-core-029: 誤りの種類

- kind: algorithm
- source: docs/decision/records/records.md#A21, docs/decision/records/records.md#A29, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-24-plan-schema.md#A11
- definition: TBL-core-008
- verification: unit

### REQ-core-030: 注意の種類

- kind: algorithm
- source: docs/decision/records/records.md#A29, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-16-notice.md#A2
- definition: TBL-core-009
- verification: unit

### REQ-core-031: 注意は7つだけ

- kind: invariant
- source: docs/decision/records/records.md#A29, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-16-notice.md#A2, docs/decision/records/2026-09-17-mutation-tests.md#A11, docs/decision/records/2026-09-17-mutation-tests.md#A20, docs/decision/records/2026-09-17-mutation-tests.md#A31, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-24-doc-marks.md#A8, docs/decision/records/2026-09-25-deferred-items.md#A11, docs/decision/records/2026-09-25-deferred-items.md#A12
- verification: unit

種類が too_many_lines、too_many_requirements、mutant_timeout、equivalent_stale、guide_stale、deferred_with_test、depends_on_deferred の`指摘`だけが`注意`で、ほかの種類の`指摘`はすべて`誤り`である関係が常に成り立つ。

### REQ-core-032: ID の重複

- kind: event_driven
- source: docs/decision/records/records.md#A47, docs/decision/records/records.md#A61, docs/decision/records/records.md#A72, docs/decision/records/records.md#A113
- verification: unit

同じ`ID`が2か所以上にあるとき、kotowari は2つ目以降の場所ごとに、その見出しの行（`シナリオ`は "Scenario:" の行）を "line" にして duplicate_id の`誤り`を出す。1つ目はパスのバイト順で先の文書、同じ文書の中では行の小さいものである。

### REQ-core-174: 宣言の外の行とコードブロックと用語集の題名

- kind: event_driven
- source: docs/decision/records/2026-09-22-ir-engine.md#A33, docs/decision/records/2026-09-22-ir-engine.md#A35, docs/decision/records/2026-09-22-ir-engine.md#A39, docs/decision/records/2026-09-22-ir-engine.md#A82, docs/decision/records/records.md#A102, docs/decision/records/ir-form.md#文書, docs/decision/records/2026-09-23-ir-engine-gaps.md#A25, docs/decision/records/2026-09-23-ir-engine-gaps.md#A27, docs/decision/records/2026-09-23-ir-english-tokens.md#A2, docs/decision/records/2026-09-24-guide-gaps.md#A1, docs/decision/records/2026-09-24-guide-gaps.md#A6, docs/decision/records/2026-09-24-guide-gaps.md#A10, docs/decision/records/2026-09-22-ir-engine.md#A89
- verification: unit

"## " の見出しの直下で最初の "### " より前に、`コードブロック`の外にあって一覧でも表でもない空でない行があるとき、またはスキーマが宣言していない表（`用語集`の文書の中の表は除く。REQ-core-117）かコードブロックがあるとき、kotowari は unknown_line の`誤り`を、"## Examples" の見出しの下に gherkin でない`コードブロック`があるとき unknown_code_block の`誤り`を、`用語集`の`題名`が "# Glossary" でないとき glossary_title_invalid の`誤り`を出す。gherkin でない`コードブロック`の中の行には invalid_gherkin_line を出さない（REQ-core-113）。detail は TBL-core-008、"line" は TBL-core-019 のとおりにする。

## Decision tables

### TBL-core-008: 誤りの種類と detail

- source: docs/decision/records/2026-09-22-ir-engine.md#A33, docs/decision/records/2026-09-22-ir-engine.md#A39, docs/decision/records/records.md#A142, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A68, docs/decision/records/records.md#A108, docs/decision/records/records.md#A109, docs/decision/records/records.md#A110, docs/decision/records/records.md#A112, docs/decision/records/records.md#A116, docs/decision/records/records.md#A111, docs/decision/records/records.md#A150, docs/decision/records/records.md#A153, docs/decision/records/records.md#A154, docs/decision/records/2026-09-16-ir-tree.md#A5, docs/decision/records/2026-09-16-ir-tree.md#A19, docs/decision/records/2026-09-17-record-form.md#A7, docs/decision/records/2026-09-17-record-form.md#A12, docs/decision/records/2026-09-17-record-form.md#A32, docs/decision/records/2026-09-17-record-form.md#A25, docs/decision/records/2026-09-17-scenario-tests.md#A6, docs/decision/records/2026-09-17-mutation-tests.md#A31, docs/decision/records/2026-09-17-mutation-tests.md#A40, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-17-mutation-tests.md#A57, docs/decision/records/2026-09-22-id-namespace.md#A4, docs/decision/records/2026-09-23-ir-engine-gaps.md#A7, docs/decision/records/2026-09-23-ir-engine-gaps.md#A17, docs/decision/records/2026-09-23-ir-engine-gaps.md#A40, docs/decision/records/2026-09-24-multi-language-tests.md#A31, docs/decision/records/2026-09-24-multi-language-tests.md#A42, docs/decision/records/2026-09-24-plan-schema.md#A18, docs/decision/records/2026-09-24-doc-marks.md#A10, docs/decision/records/2026-09-24-doc-marks.md#A31, docs/decision/records/2026-09-24-guide-gaps.md#A10, docs/decision/records/2026-09-25-deferred-items.md#A2, docs/decision/records/2026-09-25-deferred-items.md#A19

detail が「行の文字」「見出しの文字」「Scenario: の行の文字」の種類では、読んだ行の文字そのまま（字下げと末尾の空白を含み、再構成しない）を入れる。

| 種類 | detail | 条件を定める要求 |
|---|---|---|
| missing_title | 文書名（ディレクトリを除いたファイル名） | REQ-core-034 |
| multiple_titles | その件の題名（2つ目以降の題名の、"# " を除き前後の空白を除いた文字） | REQ-core-035 |
| missing_scope | 文書名（ディレクトリを除いたファイル名） | REQ-core-036 |
| unknown_heading | 見出しの文字 | REQ-core-043 |
| unknown_field | 行の文字 | REQ-core-044 |
| missing_field | 行の名前 | REQ-core-098 |
| missing_table | 決定表の ID | REQ-core-099 |
| duplicate_field | 行の名前 | REQ-core-045、REQ-core-209 |
| missing_source | 項目の ID か用語。値の空の "- deferred:" の行では "deferred" | REQ-core-059、REQ-core-210 |
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
| test_without_id | テストの名前。名前が null ならテストの節の最初の行の全体の文字から前後の空白を除いたもの | REQ-core-086 |
| invalid_marker | 行の文字 | REQ-core-072、REQ-core-202 |
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
| unknown_line | 行の文字 | REQ-core-174 |
| invalid_plan | スキーマの側の種類と詳細を ": " でつないだ文字列 | REQ-core-193 |
| unknown_code_block | 開始の行の文字 | REQ-core-174 |
| glossary_title_invalid | 題名の行の文字 | REQ-core-174 |

### TBL-core-009: 注意の種類と detail

- source: docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-16-notice.md#A2, docs/decision/records/2026-09-17-mutation-tests.md#A31, docs/decision/records/2026-09-17-mutation-tests.md#A40, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-17-mutation-tests.md#A57, docs/decision/records/2026-09-24-doc-marks.md#A8, docs/decision/records/2026-09-24-doc-marks.md#A14, docs/decision/records/2026-09-25-deferred-items.md#A11, docs/decision/records/2026-09-25-deferred-items.md#A12

| 種類 | detail | 条件を定める要求 |
|---|---|---|
| too_many_lines | 行数 | REQ-core-038 |
| too_many_requirements | 要求の数 | REQ-core-039 |
| mutant_timeout | 変更の説明 | REQ-core-140 |
| equivalent_stale | 等価の一覧の1件に書かれたままの "file" と "change" を ": " でつないだ文字列 | REQ-core-142 |
| guide_stale | `ID`、`ガイドの印`の1件に書かれた`指紋`、今の`指紋`を1つの半角空白で区切った文字列 | REQ-core-204 |
| deferred_with_test | `後回し`の`要求`か`後回しのシナリオ`の`ID` | REQ-core-211 |
| depends_on_deferred | 参照元の`ID`と参照先の`ID`を1つの半角空白で区切った文字列 | REQ-core-212 |

## Examples

```gherkin
@id=EX-core-005 @about=REQ-core-032 @source=docs/decision/records/records.md#A61,docs/decision/records/records.md#A47
Scenario: 3か所にある ID は2件の重複になる
  Given "REQ-001" の見出しが3か所にある
  When "kotowari check" を実行する
  Then duplicate_id の誤りが2件出る
  And 1つ目の見出しの行には duplicate_id が出ない

@id=EX-core-266 @about=REQ-core-174 @source=docs/decision/records/2026-09-22-ir-engine.md#A33,docs/decision/records/2026-09-22-ir-engine.md#A35,docs/decision/records/records.md#A102,docs/decision/records/ir-form.md#文書,docs/decision/records/2026-09-23-ir-english-tokens.md#A2
Scenario: 宣言の外の3つの場面はそれぞれ誤りになる
  Given "## " の見出しの直下に`コードブロック`の外の空でない行を持つ`話題ごとの文書`と、"## Examples" の見出しの下に gherkin でない`コードブロック`を持つ`話題ごとの文書`と、`題名`がスキーマの宣言した形でない`用語集`がある
  When "kotowari check --format json" を実行する
  Then unknown_line と unknown_code_block と glossary_title_invalid の`誤り`が1件ずつ出る
  And それぞれの detail は TBL-core-008、"line" は TBL-core-019 のとおりである

@id=EX-core-267 @about=REQ-core-174 @source=docs/decision/records/2026-09-22-ir-engine.md#A33
Scenario: kotowari 自身の IR では3種類とも出ない
  Given このリポジトリの`IR`の文書がある
  When "kotowari check --format json" を実行する
  Then unknown_line と unknown_code_block と glossary_title_invalid の`誤り`は1件も出ない
@id=EX-core-376 @about=REQ-core-174,REQ-core-113 @source=docs/decision/records/2026-09-24-guide-gaps.md#A1,docs/decision/records/2026-09-22-ir-engine.md#A89,docs/decision/records/ir-form.md#出力
Scenario: gherkin でないブロックの中の行は gherkin として読まない
  Given `話題ごとの文書`の "## Examples" の下に、"```text" で始まり中に "メモ" の行を持つ`コードブロック`がある
  When "kotowari check --format text" を実行する
  Then unknown_code_block の`誤り`が出て、"メモ" の行に invalid_gherkin_line は出ない

@id=EX-core-377 @about=REQ-core-174 @source=docs/decision/records/2026-09-24-guide-gaps.md#A6,docs/decision/records/2026-09-24-guide-gaps.md#A10,docs/decision/records/2026-09-22-ir-engine.md#A89,docs/decision/records/ir-form.md#出力
Scenario: 用語集の題名は Glossary でなければならない
  Given `用語集`の`題名`が "# 用語集" である
  When "kotowari check --format text" を実行する
  Then glossary_title_invalid の`誤り`が出る
```
