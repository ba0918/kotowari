---
$schema: ../../../.mds/schemas/flags.yaml
---
# 問題の記録

この IR が仕様として書きしきれていないと分かっている箇所を記録する。

### FLAG-schema-001: 出現回数を宣言しない複数の表の置き方に要求が無い

- 種類: gap
- 関係: TBL-schema-008, REQ-schema-035
- 出典: docs/decision/records/2026-09-22-ir-engine.md#A59

`crates/kotowari-markdown-schema/src/extract.rs` の `extract_table` は、`出現回数`を宣言し、かつ "value" か`導かれる値`を宣言したときだけ`配置パス`の直下に`表`ごとの段を作る。`出現回数`を宣言せずに同じ`節`や`項目`に`表`が複数あるときは、要素ごとのオブジェクトを組み立てる宣言であってもデータ行を現れた順に1つの配列へつなぐ。A59 は繰り返すときに段を作ることだけを決めていて、繰り返しを宣言しない複数の`表`をどう置くかを定めた要求が無い。

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

### FLAG-schema-004: 宣言上の名前を持つノードを定めた決定が無い

- 種類: gap
- 関係: REQ-schema-008, EX-schema-027
- 出典: docs/decision/records/2026-09-22-ir-engine.md#A27

`crates/kotowari-markdown-schema/src/schema.rs` の `Section` と `Field` は `name` のフィールドを持ち、`Title` は持たない。REQ-schema-008 は`ノードの名前`を宣言上の名前を持つ`ノード`の`指摘`にだけ付けると定め、用語集の`ノードの名前`と EX-schema-027 は`題名`をその例に挙げる。A27 は`指摘`に宣言されたノードの名前を足すところまでで、どの`規則種別`が宣言上の名前を持つかを定めていない。

### FLAG-schema-005: 要素に分けないノードの要素の値に決定が無い

- 種類: gap
- 関係: TBL-schema-008, REQ-schema-045
- 出典: docs/decision/records/2026-09-22-ir-engine.md#A64

TBL-schema-008 は`節`の`要素の値`を「`文`と`箇条書き`だけをつないだ本文の文字列」、`フィールド行`の`要素の値`を「区切り文字を宣言すれば文字列の配列」と書く。A64 は要素に分けない`ノード`を並べるだけで本文の作り方を定めておらず、区切り文字で値を分けることを定めた決定も無い（A10 は一覧の行を`フィールド行`と`箇条書き`に読み分けるところまで）。

### FLAG-schema-006: フィールド行を箇条書きの子に置けることに決定が無い

- 種類: gap
- 関係: TBL-schema-004, REQ-schema-031
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A10

`crates/kotowari-markdown-schema/src/schema.rs` の `Children` は `fields` と入れ子の `bullets` を持ち、TBL-schema-004 は`フィールド行`の置ける場所に「`箇条書き`の子」を挙げる。A10 は一覧の行を`フィールド行`と`箇条書き`に読み分けることを定めるだけで、子の一覧に`フィールド行`を宣言できることを定めていない。
