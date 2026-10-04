# 判断の記録と ADR

出典の先になる判断の記録と ADR の持ち方を扱う。

## Requirements

### REQ-core-092: 両方を残す

- kind: invariant
- source: docs/decision/records/records.md#A22, docs/decision/records/2026-09-17-decision-log.md#A5
- verification: review
- how_to_verify: 判断の記録と ADR の両方のディレクトリを読むことを `crates/kotowari/src/sources.rs` の `read_texts` で確認

`判断の記録`と、書かれた`ADR`が残る関係が常に成り立つ。`ADR`が1本も無いリポジトリでも成り立つ。

### REQ-core-093: 判断の記録は検討のまとまりごと

- kind: ubiquitous
- source: docs/decision/records/records.md#A22, docs/decision/records/2026-10-01-change-conformance.md#A13
- verification: review
- how_to_verify: 判断の記録を読むスキルと記録を確認し、brainstorm の起動なしでも検討ごとに日付と題を付けて保存し、決定の本文の変更を追加の決定と改訂の参照で追えることを確認する

`判断の記録`は常に、brainstorm の起動の有無を問わず、判断を行った検討のまとまりごとに日付と題を付けた別のファイルで残す。既存の決定の行を別の決定で上書きせず、変更は追加の決定と改訂の参照で残す。

### REQ-core-094: ADR の節

- kind: ubiquitous
- source: docs/decision/records/records.md#A43, docs/decision/records/records.md#A50
- verification: review
- how_to_verify: ADR の節の検査は出典のための見出し照合のみ。5節の構造検査はしない

`ADR`は常に、「状況」「決定」「理由」「却下した案」「結果」の5つの節を持つ。

### REQ-core-095: ADR の決定の節

- kind: ubiquitous
- source: docs/decision/records/records.md#A22, docs/decision/records/records.md#A43
- verification: review
- how_to_verify: ADR の決定の節の検査は出典のための見出し照合のみ

`ADR`の「決定」の節は常に、`判断の記録`の`決定の番号`を指し、振る舞いの文を繰り返さない。

### REQ-core-096: ADR だけにしない

- kind: prohibition
- source: docs/decision/records/records.md#R1
- verification: review
- how_to_verify: kotowari は ADR だけの運用を禁止する検査をしない（設定に両方のパスが必要）

記録の運用は、`ADR`だけにして判断1件ごとに`ADR`を切ることをしてはならない。

### REQ-core-097: 既存の ADR を消さない

- kind: prohibition
- source: docs/decision/records/2026-09-17-decision-log.md#A5, docs/decision/records/2026-09-23-ir-english-tokens.md#A2
- verification: review
- how_to_verify: `- source:` の行、`@source=` のタグ、用語集の出典の列に現れる `docs/decision/adr/` のファイルを列挙し（`rg -o 'docs/decision/adr/[^ ,|]+' docs/ir` の出典の行だけ）、そのファイルがすべて存在することを確認。2026-09-17 時点で 0002 と 0003

記録の運用は、既存の`ADR`を消すことをしてはならない。

### REQ-core-103: ADR のファイル名

- kind: ubiquitous
- source: docs/decision/records/records.md#A3, docs/decision/records/records.md#A23
- verification: review
- how_to_verify: ADR のファイル名形式は出典の検査時に見るが、形式自体は検査しない

`ADR`のファイル名は常に、"0001-<slug>.md" の形である。
