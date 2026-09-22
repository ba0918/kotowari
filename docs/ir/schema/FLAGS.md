---
$schema: ../../../.mds/schemas/flags.yaml
---
# 問題の記録

この IR が仕様として書きしきれていないと分かっている箇所を記録する。

## 問題の記録

### FLAG-schema-012: 箇条書きの子のフィールド行が項目の内側に数えられるかに要求が無い

- 種類: gap
- 関係: REQ-schema-039, REQ-schema-031
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A48, docs/decision/records/2026-09-21-mds-spec.md#A34

REQ-schema-039 は`項目`の内側として`フィールド行`、`文`、`箇条書き`、`表`、`コードブロック`を挙げるが、`箇条書き`の子の`フィールド行`が含まれるかを述べていない。実装は含める。散文の仕様にも、それを定めた決定にも、この場合の記述が無い。

### FLAG-schema-013: 必須の項目が欠けたときの指摘の種類に要求が無い

- 種類: gap
- 関係: REQ-schema-008, TBL-schema-002
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A41

A41 は missing_required_section を`節`の欠落として列挙するが、必須の`項目`が欠けたときにどの種類を出すかを定めていない。実装は`項目`の欠落にも missing_required_section を使う。

