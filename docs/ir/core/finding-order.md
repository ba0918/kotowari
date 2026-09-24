# 指摘の並べ方と行

指摘の並べ方と、指摘の "line" の決め方を扱う。

## Requirements

### REQ-core-024: 指摘の並べ方

- kind: algorithm
- source: docs/decision/records/records.md#A61, docs/decision/records/records.md#A70
- definition: TBL-core-007, PROP-core-003
- verification: property

### REQ-core-027: 文書全体への指摘

- kind: ubiquitous
- source: docs/decision/records/records.md#A40, docs/decision/records/records.md#A83, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A112, docs/decision/records/records.md#A144, docs/decision/records/2026-09-17-mutation-tests.md#A34, docs/decision/records/2026-09-17-mutation-tests.md#A40, docs/decision/records/2026-09-17-mutation-tests.md#A43
- verification: unit

kotowari は常に、種類が missing_title、multiple_titles、missing_scope、too_many_lines、too_many_requirements、unparsable_file、glossary_invalid、equivalent_stale、equivalent_invalid の`指摘`の "line" を null にし、ほかの種類の "line" を TBL-core-019 のとおりにする。

### REQ-core-028: 行は1始まり

- kind: ubiquitous
- source: docs/decision/records/records.md#A61
- verification: unit

kotowari は常に、`指摘`の "line" を1始まりで数える。

## Decision tables

### TBL-core-007: findings の並べ方

- source: docs/decision/records/records.md#A61, docs/decision/records/records.md#A70

| 順 | 鍵 | 並べ方 |
|---|---|---|
| 1 | path | バイト順 |
| 2 | line | null が先、その後は小さい順 |
| 3 | kind | バイト順 |
| 4 | detail | バイト順 |

### TBL-core-019: 指摘の行

- source: docs/decision/records/2026-09-22-ir-engine.md#A25, docs/decision/records/2026-09-22-ir-engine.md#A33, docs/decision/records/2026-09-22-ir-engine.md#A39, docs/decision/records/2026-09-22-ir-engine.md#A40, docs/decision/records/records.md#A144, docs/decision/records/records.md#A114, docs/decision/records/records.md#A121, docs/decision/records/records.md#A139, docs/decision/records/records.md#A108, docs/decision/records/records.md#A61, docs/decision/records/records.md#A72, docs/decision/records/records.md#A153, docs/decision/records/records.md#A154, docs/decision/records/2026-09-16-ir-tree.md#A4, docs/decision/records/2026-09-17-record-form.md#A7, docs/decision/records/2026-09-17-record-form.md#A12, docs/decision/records/2026-09-17-record-form.md#A32, docs/decision/records/2026-09-17-record-form.md#A25, docs/decision/records/2026-09-17-scenario-tests.md#A6, docs/decision/records/2026-09-17-mutation-tests.md#A40, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-22-id-namespace.md#A3, docs/decision/records/2026-09-24-multi-language-tests.md#A32, docs/decision/records/2026-09-24-doc-marks.md#A10, docs/decision/records/2026-09-24-doc-marks.md#A11, docs/decision/records/2026-09-24-doc-marks.md#A12, docs/decision/records/2026-09-24-doc-marks.md#A32

| 種類 | line |
|---|---|
| missing_source | 項目の見出しの行。シナリオはタグの行（無ければ "Scenario:" の行）。用語は表の行 |
| source_invalid | 出典が書かれた行（REQ-core-115） |
| missing_tag、unknown_tag、invalid_id、scenario_without_test | タグの行（無ければ "Scenario:" の行） |
| unknown_term、vague_word、unclosed_backtick、missing_document | その行 |
| unknown_field、duplicate_field、unknown_heading、invalid_gherkin_line | その行 |
| unclosed_code_block | 開始の行 |
| invalid_glossary_row | その行 |
| duplicate_term | 重複した側の用語の行（同じ用語集なら2つ目以降、連鎖では根から遠い側） |
| duplicate_id | 2つ目以降の見出しの行（REQ-core-032） |
| id_domain_mismatch | 見出しの行。"@id" の値ならタグの行 |
| unresolved_reference、invalid_marker | 印なら印のある行（REQ-core-118）。ガイドの印なら、その始まりの行（REQ-core-202）。定義・関係・文の中なら その行。"@about" ならタグの行 |
| missing_field、missing_statement、missing_table、verification_missing、verification_invalid、unknown_kind、algorithm_without_definition、requirement_without_test | 項目の見出しの行 |
| test_without_id | テストの節の最初の行 |
| record_field_missing | 番号の行 |
| record_field_unknown、revision_link_invalid | その行 |
| mutant_survived、mutant_timeout | 変異の結果の行 |
| guide_stale | ガイドの印の始まりの行（REQ-core-204） |
| unknown_line | その行 |
| unknown_code_block | 開始の行 |
| glossary_title_invalid | 題名の行 |

## Properties

### PROP-core-003: findings は並んでいる

- source: docs/decision/records/records.md#A61, docs/decision/records/records.md#A70, docs/decision/records/ir-form.md#出力

"findings" の中で隣り合うどの2つの`指摘`も、TBL-core-007 の順で比べて後ろのものが前のものより先に来ない。
