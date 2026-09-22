---
$schema: ../../../.mds/schemas/ir.yaml
---
# CLI と結果の出し方

この文書は、mds が持つコマンド、終了コード、出力の形、指摘の形、検査を行えないときの振る舞いを扱う。

## 要求

### REQ-schema-005: コマンドの一覧

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A1, docs/decision/records/2026-09-21-mds-spec.md#A16
- 検証: unit

mds は常に、検査の "check"、素の構文木の "ast"、抽出の "values"、版の "--version" の4つを受ける。

### REQ-schema-006: 終了コードの決め方

- 種類: algorithm
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A15
- 定義: TBL-schema-001
- 検証: unit

### REQ-schema-007: 出力の形

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A16
- 検証: unit

mds は常に、出力の形を "--format" で受け、人間向けの "text" と機械向けの "json" の2つから選ばせる。

### REQ-schema-008: 指摘の形

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A17, docs/decision/records/2026-09-22-ir-engine.md#A25, docs/decision/records/2026-09-22-ir-engine.md#A27, docs/decision/records/2026-09-22-ir-engine.md#A29
- 検証: unit

mds は常に、1件の`指摘`を、種類、深刻度、`文書`のパス、行番号、`ノードの名前`、`生の行`、詳細の7つで表し、行を持たない`指摘`では行番号を省く。行番号は、違反した`ノード`があるならその`ノード`の開始行、`ノード`の欠落ならそれを含む`ノード`の開始行にし、含む`ノード`に行が無いときは省く。`ノードの名前`は宣言上の名前を持つ`ノード`の`指摘`にだけ付け、`生の行`は行番号を持つ`指摘`にだけ付ける。

### REQ-schema-009: 検査を行えないときは停止する

- 種類: event_driven
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A8, docs/decision/records/2026-09-21-mds-spec.md#P1
- 検証: unit

`スキーマ`を解決できないとき、または`frontmatter`が壊れているとき、mds は部分的な結果を出さずに`停止`する。

### REQ-schema-010: ディレクトリの検査

- 種類: event_driven
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A1
- 検証: unit

検査の対象がディレクトリのとき、mds はその下の`スキーマ`を宣言した`文書`だけを集めて検査する。

### REQ-schema-042: 停止の理由

- 種類: algorithm
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A15, docs/decision/records/2026-09-21-mds-spec.md#P1
- 定義: TBL-schema-009
- 検証: unit

### REQ-schema-043: 停止の知らせ方

- 種類: event_driven
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A15, docs/decision/records/2026-09-21-mds-spec.md#A16
- 検証: unit

mds が`停止`するとき、理由の名前と説明を並べた1行だけを標準エラーに出し、`指摘`は1件も出さない。

### REQ-schema-044: ディレクトリ検査で辿らないもの

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A21
- 検証: unit

mds は常に、ディレクトリの検査で、名前が "." で始まるディレクトリ、`スキーマ`とキャッシュの置き場、シンボリックリンク、拡張子が ".md" でないファイルを辿らない。

## 決定表

### TBL-schema-001: 終了コード

- 出典: docs/decision/records/2026-09-21-mds-spec.md#A15

| 順 | 条件 | 終了コード |
|---|---|---|
| 1 | 検査を行えなかった（`スキーマ`を解決できない、`frontmatter`が壊れている、`スキーマ`が形に合わない） | 2 |
| 2 | 1 に当たらず、`指摘`が1件以上ある | 1 |
| 3 | 1 にも 2 にも当たらない | 0 |

### TBL-schema-009: 停止の理由

- 出典: docs/decision/records/2026-09-21-mds-spec.md#A15, docs/decision/records/2026-09-21-mds-spec.md#P1, docs/decision/records/2026-09-21-mds-spec.md#A16, docs/decision/records/2026-09-21-mds-spec.md#A25, docs/decision/records/2026-09-22-ir-engine.md#A56

| 理由 | いつ |
|---|---|
| スキーマが見つからない | 参照先の`スキーマ`が無い、URL の取得に失敗した、または "$schema" の無い`文書`を対象に指定した |
| スキーマが形に合わない | `スキーマ`の YAML が読めない、`規則種別`の形に反する、または1つの要素オブジェクトの中で鍵が重複する |
| frontmatter が壊れている | `frontmatter`が壊れた YAML である、または "$schema" の値が文字列でないか空である |
| 文書が読めない | `文書`のファイルを読めない |
| 引数の誤り | 知らないフラグ、または受けない "--format" の値 |

### TBL-schema-002: 指摘の分類

- 出典: docs/decision/records/2026-09-21-mds-spec.md#A17, docs/decision/records/2026-09-21-mds-spec.md#A2

| 分類 | 何を見つけるか |
|---|---|
| 欠落 | 必須の`ノード`が無い |
| 形の違反 | 値が正規表現や許可リストに合わない、`表`のヘッダが合わない |
| 出現回数 | `出現回数`の下限を下回る、上限を超える |
| 閉じた世界 | 宣言していない見出しと行がある |

## 性質

### PROP-schema-002: 抽出は検査の合否から独立している

- 出典: docs/decision/records/2026-09-21-mds-spec.md#A4

`抽出`の結果は、同じ`文書`と同じ`スキーマ`であれば、検査で`指摘`が出たかどうかによって変わらない。

## 具体例

```gherkin
@id=EX-schema-003 @about=REQ-schema-006 @source=docs/decision/records/2026-09-21-mds-spec.md#A15
Scenario: 指摘の無い文書は終了コード 0 で終わる
  Given `スキーマ`をすべて満たす`文書`がある
  When "mds check" を実行する
  Then 終了コードは 0 である
  And 何も出力しない

@id=EX-schema-004 @about=REQ-schema-009 @source=docs/decision/records/2026-09-21-mds-spec.md#A8
Scenario: frontmatter が壊れていれば停止する
  Given `frontmatter`が YAML のマッピングでない`文書`がある
  When "mds check" を実行する
  Then 終了コードは 2 である
  And `指摘`は出力しない

@id=EX-schema-026 @about=REQ-schema-008 @source=docs/decision/records/2026-09-22-ir-engine.md#A21,docs/decision/records/2026-09-22-ir-engine.md#A25
Scenario: 指摘の行は違反したノードか、それを含むノードの開始行になる
  Given 必須の`フィールド行`を落とした`項目`と、宣言した形に合わない値の`フィールド行`を持つ`項目`と、列の足りない`表`を持つ`文書`がある
  When "mds check --format json" を実行する
  Then 欠落の`指摘`の行はその`項目`の見出しの行である
  And 形に合わない`フィールド行`の`指摘`の行はその`フィールド行`の行である
  And 列の足りない`表`の`指摘`の行はその行である

@id=EX-schema-027 @about=REQ-schema-008 @source=docs/decision/records/2026-09-22-ir-engine.md#A27
Scenario: 指摘は宣言されたノードの名前を持つ
  Given 必須の`フィールド行`を落とした`項目`と、宣言した形に合わない`題名`を持つ`文書`がある
  When "mds check --format json" を実行する
  Then `フィールド行`の`指摘`の`ノードの名前`はスキーマが宣言した名前である
  And `題名`の`指摘`は`ノードの名前`を持たない

@id=EX-schema-028 @about=REQ-schema-008 @source=docs/decision/records/2026-09-22-ir-engine.md#A29
Scenario: 行を持つ指摘は生の行をそのまま持つ
  Given 宣言した形に合わない ID の見出しを持つ`項目`の`文書`がある
  When "mds check --format json" を実行する
  Then その`指摘`の`生の行`は`文書`のその行と一文字も違わない

@id=EX-schema-029 @about=REQ-schema-042,TBL-schema-009 @source=docs/decision/records/2026-09-22-ir-engine.md#A56
Scenario: 要素オブジェクトの中で鍵が重なるスキーマは停止する
  Given 内側の`フィールド行`の`配置パス`と外側の`導かれる値`の鍵が重なる`スキーマ`がある
  When "mds values" を実行する
  Then 終了コードは 2 である
  And 標準エラーは`スキーマ`が形に合わないことを知らせる
  And `文書`を読まずに`停止`する

@id=EX-schema-030 @about=REQ-schema-042,TBL-schema-009 @source=docs/decision/records/2026-09-22-ir-engine.md#A65
Scenario: ドットの上の段を共有するだけの配置パスは重複でない
  Given "a.b" と "a.c" を並べた`スキーマ`と、"a" と "a.b" を並べた`スキーマ`がある
  When それぞれに "mds values" を実行する
  Then 前者は`停止`せず、後者は終了コード 2 で終わる
```
