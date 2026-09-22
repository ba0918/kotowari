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
- 出典: docs/decision/records/2026-09-22-ir-engine.md#A64, docs/decision/records/2026-09-21-mds-spec.md#A10

A64 は要素に分けない`ノード`を並べるだけで本文の作り方を定めていない。区切り文字で値を分けることも A64 に無く、A10 は一覧の行を`フィールド行`と`箇条書き`に読み分けるところまでを定める。TBL-schema-008 からは、`節`の`要素の値`と略記の形（`文`と`箇条書き`だけをつないだ本文の文字列）を決まっていないとして外し、`フィールド行`の区切り文字による文字列の配列も外した。

### FLAG-schema-006: フィールド行を箇条書きの子に置けることに決定が無い

- 種類: gap
- 関係: TBL-schema-004, REQ-schema-031
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A10

`crates/kotowari-markdown-schema/src/schema.rs` の `Children` は `fields` と入れ子の `bullets` を持ち、TBL-schema-004 は`フィールド行`の置ける場所に「`箇条書き`の子」を挙げる。A10 は一覧の行を`フィールド行`と`箇条書き`に読み分けることを定めるだけで、子の一覧に`フィールド行`を宣言できることを定めていない。

### FLAG-schema-007: 複数行にまたがるノードの生の行が指す行に決定が無い

- 種類: gap
- 関係: TBL-schema-008, EX-schema-025
- 出典: docs/decision/records/2026-09-22-ir-engine.md#A64

`crates/kotowari-markdown-schema/src/extract.rs` は`節`と`項目`で見出しの行を、`コードブロック`でフェンスの開始行を要素の行として渡し、`生の行`をその行から引く。A64 は`ノード`ごとの要素の単位を並べ、`生の行`が指す行を一表で定めると述べるだけで、複数行にまたがる`ノード`がどの行を指すかの値を決めていない。TBL-schema-008 の「`生の行`が指す行」の列は、この3つの`ノード`について決まっていないとしてある。EX-schema-025 からは裏付けの無い`項目`と`コードブロック`を外した。

### FLAG-schema-008: 項目の略記の値に決定が無い

- 種類: gap
- 関係: TBL-schema-008, REQ-schema-047
- 出典: docs/decision/records/2026-09-22-ir-engine.md#A63

`crates/kotowari-markdown-schema/src/extract.rs` の `item_value_text` は`項目`の見出しと本文をつないだ1つの文字列を返す。A63 は略記の`抽出`の形を今のままにすると述べて`文`と`表`の値だけを挙げていて、`項目`の値を定めていない。TBL-schema-008 の`項目`の`要素の値`と、オブジェクトを組み立てないときの略記の形は、決まっていないとしてある。

### FLAG-schema-009: 項目の外に id と name を宣言できないことに決定が無い

- 種類: gap
- 関係: REQ-schema-048, EX-schema-024
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A23, docs/decision/records/2026-09-22-ir-engine.md#A54

`crates/kotowari-markdown-schema/src/schema.rs` の `reject_item_only_of` は、`項目`の外の`ノード`に "of" の "id" と "name" を宣言した`スキーマ`を`停止`にする。A23 は`導かれる値`を「行番号、`項目`の見出しの ID と名前」と列挙するだけで、`項目`以外の`ノード`に宣言できないことも、そのときの応答が終了コード 2 であることも定めていない。A54 は`生の行`を宣言できる範囲を行番号と同じ（どの`ノード`にも）と定めるだけで、`項目`の見出しの ID と名前を宣言できる範囲には触れていない。EX-schema-024 は、決定のある「受けない語は`停止`」の場面に置き換えた。

### FLAG-schema-010: 停止の理由の2つの場面に決定が無い

- 種類: gap
- 関係: TBL-schema-009, REQ-schema-014
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A8

`crates/kotowari-markdown-schema/src/main.rs` は、引数の解析が失敗したとき（知らないフラグを含む）に argument_error の`停止`にする。`crates/kotowari-markdown-schema/src/frontmatter.rs` は "$schema" の値が文字列でないときも`停止`にする。A8 は`frontmatter`が YAML のマッピングでないときと "$schema" の値が空か空白だけのときを定めるだけで、値が文字列でないときを定めておらず、CLI の引数の形を定めた決定も無い。TBL-schema-009 からはこの2つを外した。

### FLAG-schema-011: 転記のときの出典が項目を裏付けていない

- 種類: gap
- 関係: REQ-schema-020, REQ-schema-021, REQ-schema-032, TBL-schema-005
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A3

mds の仕様を IR へ転記したときに付けた`出典`が、項目の内容を裏付けていないものがある。典型例は 2026-09-21-mds-spec.md#A3 で、本文は「スキーマは YAML で書く」の1文だけなのに、REQ-schema-020（条件付き規則）、REQ-schema-021（条件の探索範囲）、REQ-schema-032（文の数え方）、TBL-schema-005（出現回数の5つの書き方）、および用語5語の出典になっている。裏付けを確かめ直す対象は次のとおり。

対象の要求は REQ-schema-005、REQ-schema-010、REQ-schema-020、REQ-schema-021、REQ-schema-024、REQ-schema-029、REQ-schema-032、REQ-schema-034、REQ-schema-036、REQ-schema-037、REQ-schema-038、REQ-schema-039、REQ-schema-041、REQ-schema-043、REQ-schema-046、REQ-schema-050 の 16 件。
対象の決定表は TBL-schema-002、TBL-schema-003、TBL-schema-005、TBL-schema-006、TBL-schema-007、TBL-schema-010 の 6 件。
対象の性質は PROP-schema-002、PROP-schema-004、PROP-schema-007 の 3 件。
対象の具体例は EX-schema-016 の 1 件。
対象の用語は用語集（CONTEXT.md）の`規則種別`、`ノード`、`文`、`出現回数`、`条件付き規則`、`前置部`の 6 語。
