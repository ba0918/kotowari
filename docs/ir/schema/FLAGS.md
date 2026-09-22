---
$schema: ../../../.mds/schemas/flags.yaml
---
# 問題の記録

この IR が仕様として書きしきれていないと分かっている箇所を記録する。

### FLAG-schema-001: 複数の表を1つの配列へつなぐ分岐に要求が無い

- 種類: gap
- 関係: TBL-schema-008, REQ-schema-035
- 出典: docs/decision/records/2026-09-22-ir-engine.md#A59

`crates/kotowari-markdown-schema/src/extract.rs` の `extract_table` は、`出現回数`を宣言し、かつ "value" か`導かれる値`を宣言したときだけ`配置パス`の直下に`表`ごとの段を作り、それ以外のとき（略記のとき、`出現回数`を宣言しないとき）は同じ`節`や`項目`にある`表`のデータ行を現れた順に1つの配列へつなぐ。TBL-schema-008 は「`表`が繰り返すときは表ごとの配列になる」としか述べず、つなぐ側の分岐を定めた要求が無い。

### FLAG-schema-002: 箇条書きに正規表現を課せることに要求が無い

- 種類: gap
- 関係: TBL-schema-007, REQ-schema-031
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A10

`crates/kotowari-markdown-schema/src/schema.rs` の `Bullets` は "pattern" を受け、合わない行に bullet_pattern_mismatch の`指摘`を出す。TBL-schema-007 は一覧の行を`フィールド行`と`箇条書き`に読み分けるところまでを定め、`箇条書き`の行そのものに正規表現を課せることを定めた要求が無い。

### FLAG-schema-003: 文に正規表現と許可リストを課せることに要求が無い

- 種類: gap
- 関係: REQ-schema-032, REQ-schema-029
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A3

`crates/kotowari-markdown-schema/src/schema.rs` の `Statement` は "pattern" と "enum" を受け、合わない`文`に statement_pattern_mismatch と statement_enum_invalid の`指摘`を出す。REQ-schema-032 は`文`の数え方だけを定め、値の制約を定める REQ-schema-029 は`フィールド行`しか扱っていない。
