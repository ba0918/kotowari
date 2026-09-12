# 指摘の並べ方と行

指摘の並べ方と、指摘の "line" の決め方を扱う。

## 要求

### REQ-024: 指摘の並べ方

- 種類: algorithm
- 出典: brainstorm/records.md#A61, brainstorm/records.md#A70
- 定義: TBL-007, PROP-003
- 検証: property

### REQ-027: 文書全体への指摘

- 種類: ubiquitous
- 出典: brainstorm/records.md#A40, brainstorm/ir-form.md#検査の種類
- 検証: unit

kotowari は常に、種類が missing_title、multiple_titles、missing_scope、too_many_lines、too_many_requirements の`指摘`の "line" を null にする。

### REQ-028: 行は1始まり

- 種類: ubiquitous
- 出典: brainstorm/records.md#A61
- 検証: unit

kotowari は常に、`指摘`の "line" を1始まりで数える。

## 決定表

### TBL-007: findings の並べ方

- 出典: brainstorm/records.md#A61, brainstorm/records.md#A70

| 順 | 鍵 | 並べ方 |
|---|---|---|
| 1 | path | バイト順 |
| 2 | line | null が先、その後は小さい順 |
| 3 | kind | バイト順 |
| 4 | detail | バイト順 |

## 性質

### PROP-003: findings は並んでいる

- 出典: brainstorm/records.md#A61

"findings" の中で隣り合うどの2つの`指摘`も、TBL-007 の順で比べて後ろのものが前のものより先に来ない。
