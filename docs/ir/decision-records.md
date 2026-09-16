# 判断の記録と ADR

出典の先になる判断の記録と ADR の持ち方を扱う。

## 要求

### REQ-092: 両方を残す

- 種類: invariant
- 出典: docs/decision/brainstorm/records.md#A22, docs/decision/brainstorm/2026-09-17-decision-log.md#A5
- 検証: review

`判断の記録`と、書かれた`ADR`が残る関係が常に成り立つ。`ADR`が1本も無いリポジトリでも成り立つ。

### REQ-093: 判断の記録は brainstorm ごと

- 種類: ubiquitous
- 出典: docs/decision/brainstorm/records.md#A22
- 検証: review

`判断の記録`は常に、brainstorm ごとに日付と題を付けた別のファイルで、上書きされない。

### REQ-094: ADR の節

- 種類: ubiquitous
- 出典: docs/decision/brainstorm/records.md#A43, docs/decision/brainstorm/records.md#A50
- 検証: review

`ADR`は常に、「状況」「決定」「理由」「却下した案」「結果」の5つの節を持つ。

### REQ-095: ADR の決定の節

- 種類: ubiquitous
- 出典: docs/decision/brainstorm/records.md#A22, docs/decision/brainstorm/records.md#A43
- 検証: review

`ADR`の「決定」の節は常に、`判断の記録`の`決定の番号`を指し、振る舞いの文を繰り返さない。

### REQ-096: ADR だけにしない

- 種類: prohibition
- 出典: docs/decision/brainstorm/records.md#R1
- 検証: review

記録の運用は、`ADR`だけにして判断1件ごとに`ADR`を切ることをしてはならない。

### REQ-097: 既存の ADR を消さない

- 種類: prohibition
- 出典: docs/decision/brainstorm/2026-09-17-decision-log.md#A5
- 検証: review

記録の運用は、既存の`ADR`を消すことをしてはならない。

### REQ-103: ADR のファイル名

- 種類: ubiquitous
- 出典: docs/decision/brainstorm/records.md#A3, docs/decision/brainstorm/records.md#A23
- 検証: review

`ADR`のファイル名は常に、"0001-<slug>.md" の形である。
