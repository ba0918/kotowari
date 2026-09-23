# CLI と結果の出し方

この文書は、mds が持つコマンド、終了コード、出力の形、指摘の形、検査を行えないときの振る舞いを扱う。

## Requirements

### REQ-schema-005: コマンドの一覧

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A1, docs/decision/records/2026-09-21-mds-spec.md#A16, docs/decision/records/2026-09-21-mds-spec.md#A26, docs/decision/records/2026-09-23-versions-and-cli-name.md#A4
- verification: unit

mds は常に、"kotowari-mds" の名前のコマンドとして、検査の "check"、素の構文木の "ast"、抽出の "values"、版の "--version" の4つを受ける。

### REQ-schema-006: 終了コードの決め方

- kind: algorithm
- source: docs/decision/records/2026-09-21-mds-spec.md#A15
- definition: TBL-schema-001
- verification: unit

### REQ-schema-007: 出力の形

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A16, docs/decision/records/2026-09-21-mds-spec.md#A44
- verification: unit

mds は常に、出力の形を "--format" で受け、人間向けの "text" と機械向けの "json" の2つから選ばせる。ただし "ast" は "json" だけを受け、"text" を与えたときは`停止`する。

### REQ-schema-066: values の text 出力の字下げ

- kind: ubiquitous
- source: docs/decision/records/2026-09-23-mutants-gaps.md#A8
- verification: unit

mds は常に、"values" の "text" の出力で、入れ子が1段深くなるごとに空白2つで字下げし、配列の要素には1から始まる番号と "." を付ける。

### REQ-schema-008: 指摘の形

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A17, docs/decision/records/2026-09-21-mds-spec.md#A45, docs/decision/records/2026-09-21-mds-spec.md#A61, docs/decision/records/2026-09-22-ir-engine.md#A74, docs/decision/records/2026-09-22-ir-engine.md#A25, docs/decision/records/2026-09-22-ir-engine.md#A27, docs/decision/records/2026-09-22-ir-engine.md#A29, docs/decision/records/2026-09-22-ir-engine.md#A86, docs/decision/records/2026-09-23-ir-engine-gaps.md#A8
- verification: unit

mds は常に、1件の`指摘`を、種類、深刻度、`文書`のパス、行番号、`ノードの名前`、`生の行`、種別、詳細の8つで表し、行を持たない`指摘`では行番号を省く。行番号は、違反した`ノード`があるならその`ノード`の開始行、`ノード`の欠落ならそれを含む`ノード`の開始行にし、含む`ノード`に行が無いときは省く。`表`のデータ行のように`ノード`の中の要素が違反したときは、その要素の行にする。`ノードの名前`は宣言上の名前を持つ`ノード`の`指摘`にだけ付け、宣言上の名前を持つのは`節`と`フィールド行`だけであり、種別は undeclared_line の`指摘`には宣言していない行をどう読んだか（REQ-schema-055）を、`出現回数`の下限と上限の`指摘`には数えた`ノード`の`規則種別`（REQ-schema-057）を付け、ほかの`指摘`には付けない。`生の行`は行番号を持つ`指摘`にだけ付ける。

### REQ-schema-009: 検査を行えないときは停止する

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A8, docs/decision/records/2026-09-21-mds-spec.md#A42, docs/decision/records/2026-09-21-mds-spec.md#A62, docs/decision/records/2026-09-21-mds-spec.md#P1
- verification: unit

TBL-schema-009 の`停止`の理由のいずれかに当たったとき、mds は部分的な結果を出さずに`停止`する。

### REQ-schema-010: ディレクトリの検査

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A1, docs/decision/records/2026-09-21-mds-spec.md#A27, docs/decision/records/2026-09-21-mds-spec.md#A60
- verification: unit

検査の対象がディレクトリのとき、mds はその下の`スキーマ`を宣言した`文書`だけを集めて検査する。"$schema" を持たない`文書`は対象外にし、"$schema" はあるが値が空か空白だけの`文書`は宣言と見なさず`停止`する。

### REQ-schema-042: 停止の理由

- kind: algorithm
- source: docs/decision/records/2026-09-21-mds-spec.md#A15, docs/decision/records/2026-09-21-mds-spec.md#P1, docs/decision/records/2026-09-23-ir-engine-gaps.md#A11, docs/decision/records/2026-09-23-ir-engine-gaps.md#A19
- definition: TBL-schema-009
- verification: unit

### REQ-schema-043: 停止の知らせ方

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A15, docs/decision/records/2026-09-21-mds-spec.md#A16, docs/decision/records/2026-09-21-mds-spec.md#A42
- verification: unit

mds が`停止`するとき、理由の名前と説明を並べた1行だけを標準エラーに出し、`指摘`は1件も出さない。

### REQ-schema-065: 値の型や語が違うときの停止の説明

- kind: event_driven
- source: docs/decision/records/2026-09-23-mutants-gaps.md#A6
- verification: unit

`スキーマ`か`frontmatter`の値の型や語が違って mds が`停止`するとき、mds はその説明に、問題のあった欄の名前と、期待した型か受け付ける語の一覧を含める。

### REQ-schema-053: ディレクトリ検査の途中の停止

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A42
- verification: unit

ディレクトリの検査の途中で`停止`の理由に当たったとき、mds は全体を`停止`し、それまでに集めた`指摘`を1件も出さない。

### REQ-schema-044: ディレクトリ検査で辿らないもの

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A21
- verification: unit

mds は常に、ディレクトリの検査で、名前が "." で始まるディレクトリ、`スキーマ`とキャッシュの置き場、シンボリックリンク、拡張子が ".md" でないファイルを辿らない。

## Decision tables

### TBL-schema-001: 終了コード

- source: docs/decision/records/2026-09-21-mds-spec.md#A15

| 順 | 条件 | 終了コード |
|---|---|---|
| 1 | 検査を行えなかった（TBL-schema-009 の`停止`の理由のいずれかに当たった） | 2 |
| 2 | 1 に当たらず、`指摘`が1件以上ある | 1 |
| 3 | 1 にも 2 にも当たらない | 0 |

### TBL-schema-009: 停止の理由

- source: docs/decision/records/2026-09-21-mds-spec.md#A8, docs/decision/records/2026-09-21-mds-spec.md#A15, docs/decision/records/2026-09-21-mds-spec.md#P1, docs/decision/records/2026-09-21-mds-spec.md#A16, docs/decision/records/2026-09-21-mds-spec.md#A25, docs/decision/records/2026-09-21-mds-spec.md#A43, docs/decision/records/2026-09-21-mds-spec.md#A44, docs/decision/records/2026-09-21-mds-spec.md#A62, docs/decision/records/2026-09-22-ir-engine.md#A56, docs/decision/records/2026-09-23-ir-engine-gaps.md#A11, docs/decision/records/2026-09-23-ir-engine-gaps.md#A19, docs/decision/records/2026-09-23-ir-engine-gaps.md#A20, docs/decision/records/2026-09-23-ir-engine-gaps.md#A28, docs/decision/records/2026-09-23-ir-engine-gaps.md#A30, docs/decision/records/2026-09-23-ir-engine-gaps.md#A42, docs/decision/records/2026-09-23-mutants-gaps.md#A11, docs/decision/records/2026-09-24-review2-gaps.md#A2, docs/decision/records/2026-09-24-review4-gaps.md#A3, docs/decision/records/2026-09-24-review4-gaps.md#A4, docs/decision/records/2026-09-24-review4-gaps.md#A5, docs/decision/records/2026-09-24-review7-gaps.md#A4

| 理由 | いつ |
|---|---|
| スキーマが見つからない | 参照先の`スキーマ`が無い、URL の取得に失敗した（応答が 4MiB を超えたとき、取得全体が10秒を超えたときを含む）、または "$schema" の無い`文書`を対象に指定した |
| スキーマが形に合わない | `スキーマ`の YAML が読めない、`規則種別`の形に反する、`配置パス`が衝突する（同じ置き場の中で同じパスか、一方が他方の手前の段にあたるもの。要素オブジェクトの中と、`節`の直下などの要素オブジェクトの外のどちらでも判定し、要素オブジェクトの "value" と "of" の鍵も同じ置き場のパスとして数え、"a.b" と "a.c" のように途中まで同じで先が分かれるものは衝突でない）、"reading" の値が "paragraph" と "line" のどちらでもない、`表`の規則に "header" なしで "select" を書いた、"select" の値が "first" でない、"name" を宣言した`スキーマ`で要素オブジェクトの外の`配置パス`が "type" か "type." で始まる（"ast --schema" が "name" を置く鍵と衝突する）、同じ置き場に同じ名前の`節`か`フィールド行`を2度宣言した、`表`のヘッダに同じ列の名前を2度書いた、または`配置パス`のドットで区切った名前に空のものがある |
| frontmatter が壊れている | `frontmatter`が壊れた YAML である、YAML のマッピングでない、"$schema" の値が空か空白だけである、または "$schema" の値が文字列でない |
| 文書が読めない | `文書`のファイルを読めない |
| 引数の誤り | 受けない "--format" の値、知らないフラグ、同じオプションを2回渡した、または "ast" に "--format text" を与えた |

### TBL-schema-002: 指摘の分類

- source: docs/decision/records/2026-09-21-mds-spec.md#A17, docs/decision/records/2026-09-21-mds-spec.md#A2, docs/decision/records/2026-09-21-mds-spec.md#A41, docs/decision/records/2026-09-21-mds-spec.md#A40

| 分類 | 何を見つけるか |
|---|---|
| 題名 | `題名`が無い、2つ以上ある、正規表現に合わない |
| 欠落 | 必須の`ノード`が無い |
| 形の違反 | 値が正規表現や許可リストに合わない、見出しの深さが合わない、`項目`の `ID` の形が合わない、`フィールド行`の並び順が合わない、`表`のヘッダが合わない、`コードブロック`の言語が合わない |
| 出現回数 | `出現回数`の下限を下回る、上限を超える |
| 閉じた世界 | 宣言していない見出しと行がある |

## Properties

### PROP-schema-002: 抽出は検査の合否から独立している

- source: docs/decision/records/2026-09-21-mds-spec.md#A4, docs/decision/records/2026-09-21-mds-spec.md#A56

`抽出`の結果は、同じ`文書`と同じ`スキーマ`であれば、検査で`指摘`が出たかどうかによって変わらない。

## Examples

```gherkin
@id=EX-schema-081 @about=TBL-schema-009 @source=docs/decision/records/2026-09-24-review7-gaps.md#A4
Scenario: 同じオプションを2回渡すと停止する
  When "kotowari-mds check" に "--format" を2回渡して実行する
  Then 終了コードは 2 である

@id=EX-schema-076 @about=TBL-schema-009 @source=docs/decision/records/2026-09-24-review4-gaps.md#A3
Scenario: 同じ名前の節やフィールド行を2度宣言したスキーマは停止する
  Given 同じ名前の`節`を2つ宣言した`スキーマ`と、`前置部`に同じ名前の`フィールド行`を2つ宣言した`スキーマ`がある
  When それぞれの`スキーマ`で "kotowari-mds check" を実行する
  Then どちらも終了コードは 2 である

@id=EX-schema-078 @about=TBL-schema-009 @source=docs/decision/records/2026-09-24-review4-gaps.md#A5
Scenario: 表のヘッダに同じ列の名前を2度書いたスキーマは停止する
  Given `表`のヘッダに同じ列の名前を2つ書いた`スキーマ`がある
  When その`スキーマ`で "kotowari-mds check" を実行する
  Then 終了コードは 2 である

@id=EX-schema-077 @about=REQ-schema-036,TBL-schema-009 @source=docs/decision/records/2026-09-24-review4-gaps.md#A4
Scenario: 空の名前を含む配置パスは停止する
  Given `題名`の`配置パス`を "a..b" にした`スキーマ`がある
  When その`スキーマ`で "kotowari-mds values" を実行する
  Then 終了コードは 2 である

@id=EX-schema-073 @about=TBL-schema-009 @source=docs/decision/records/2026-09-24-review2-gaps.md#A2
Scenario: name を宣言したスキーマで type に値を置くと停止する
  Given "name" を宣言し、`題名`の`抽出`の`配置パス`を "type" にした`スキーマ`がある
  When その`スキーマ`で "kotowari-mds values --format json" を実行する
  Then 終了コードは 2 である
  And "name" を宣言しない同じ`スキーマ`では停止せず、"type" に`題名`の値を置く

@id=EX-schema-003 @about=REQ-schema-006 @source=docs/decision/records/2026-09-21-mds-spec.md#A15
Scenario: 指摘の無い文書は終了コード 0 で終わる
  Given `スキーマ`をすべて満たす`文書`がある
  When "kotowari-mds check" を実行する
  Then 終了コードは 0 である
  And 何も出力しない

@id=EX-schema-004 @about=REQ-schema-009 @source=docs/decision/records/2026-09-21-mds-spec.md#A8
Scenario: frontmatter が壊れていれば停止する
  Given `frontmatter`が YAML のマッピングでない`文書`がある
  When "kotowari-mds check" を実行する
  Then 終了コードは 2 である
  And `指摘`は出力しない

@id=EX-schema-026 @about=REQ-schema-008 @source=docs/decision/records/2026-09-22-ir-engine.md#A21,docs/decision/records/2026-09-22-ir-engine.md#A25
Scenario: 指摘の行は違反したノードか、それを含むノードの開始行になる
  Given 必須の`フィールド行`を落とした`項目`と、宣言した形に合わない値の`フィールド行`を持つ`項目`と、列の足りない`表`を持つ`文書`がある
  When "kotowari-mds check --format json" を実行する
  Then 欠落の`指摘`の行はその`項目`の見出しの行である
  And 形に合わない`フィールド行`の`指摘`の行はその`フィールド行`の行である
  And 列の足りない`表`の`指摘`の行はその行である

@id=EX-schema-027 @about=REQ-schema-008 @source=docs/decision/records/2026-09-22-ir-engine.md#A27
Scenario: 指摘は宣言されたノードの名前を持つ
  Given 必須の`フィールド行`を落とした`項目`と、宣言した形に合わない`題名`を持つ`文書`がある
  When "kotowari-mds check --format json" を実行する
  Then `フィールド行`の`指摘`の`ノードの名前`はスキーマが宣言した名前である
  And `題名`の`指摘`は`ノードの名前`を持たない

@id=EX-schema-028 @about=REQ-schema-008 @source=docs/decision/records/2026-09-22-ir-engine.md#A29
Scenario: 行を持つ指摘は生の行をそのまま持つ
  Given 宣言した形に合わない ID の見出しを持つ`項目`の`文書`がある
  When "kotowari-mds check --format json" を実行する
  Then その`指摘`の`生の行`は`文書`のその行と一文字も違わない

@id=EX-schema-029 @about=REQ-schema-042,TBL-schema-009 @source=docs/decision/records/2026-09-22-ir-engine.md#A56
Scenario: 要素オブジェクトの中で鍵が重なるスキーマは停止する
  Given 内側の`フィールド行`の`配置パス`と外側の`導かれる値`の鍵が重なる`スキーマ`がある
  When "kotowari-mds values" を実行する
  Then 終了コードは 2 である
  And 標準エラーは`スキーマ`が形に合わないことを知らせる
  And `文書`を読まずに`停止`する

@id=EX-schema-030 @about=REQ-schema-042,TBL-schema-009 @source=docs/decision/records/2026-09-22-ir-engine.md#A65
Scenario: ドットの上の段を共有するだけの配置パスは重複でない
  Given "a.b" と "a.c" を並べた`スキーマ`と、"a" と "a.b" を並べた`スキーマ`がある
  When それぞれに "kotowari-mds values" を実行する
  Then 前者は`停止`せず、後者は終了コード 2 で終わる

@id=EX-schema-047 @about=REQ-schema-042,TBL-schema-009 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A11,docs/decision/records/2026-09-23-ir-engine-gaps.md#A19
Scenario: 要素オブジェクトの外で配置パスが衝突するスキーマは停止する
  Given 2つの`節`の直下の`文`に同じ`配置パス`の`抽出`を宣言した`スキーマ`と、"a" と "a.b" を2つの`節`に分けて宣言した`スキーマ`がある
  When それぞれに "kotowari-mds values" を実行する
  Then どちらも終了コードは 2 で、標準エラーは`スキーマ`が形に合わないことを知らせる

@id=EX-schema-048 @about=REQ-schema-042,TBL-schema-009 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A19
Scenario: 要素オブジェクトの外でも先が分かれる配置パスは衝突でない
  Given "a.b" と "a.c" を2つの`節`に分けて宣言した`スキーマ`がある
  When "kotowari-mds values" を実行する
  Then `停止`しない

@id=EX-schema-049 @about=REQ-schema-008 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A8,docs/decision/records/2026-09-22-ir-engine.md#A86
Scenario: 出現回数の指摘は数えたノードの規則種別を持つ
  Given `項目`の`文`に`出現回数`の下限1を宣言した`スキーマ`がある
  And `文`の無い`項目`を持つ`文書`がある
  When "kotowari-mds check --format json" を実行する
  Then 下限を割った`指摘`の種別は`文`である

@id=EX-schema-062 @about=REQ-schema-065 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A6
Scenario: 型の違う frontmatter の値の停止は欄の名前と期待した型を知らせる
  Given "$schema" の値が数値の`frontmatter`を持つ`文書`がある
  When "kotowari-mds check" を実行する
  Then 終了コードは 2 である
  And 標準エラーの説明は "$schema" と、期待した型を含む

@id=EX-schema-063 @about=REQ-schema-065 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A6,docs/decision/records/2026-09-23-ir-engine-gaps.md#A20,docs/decision/records/2026-09-23-ir-engine-gaps.md#A30,docs/decision/records/2026-09-21-mds-spec.md#A15,docs/decision/records/2026-09-21-mds-spec.md#A42
Scenario: 受けない語のスキーマの停止は欄の名前と受け付ける語の一覧を知らせる
  Given "reading" の値が "foo" の`スキーマ`を指した`文書`がある
  When "kotowari-mds check" を実行する
  Then 終了コードは 2 である
  And 標準エラーの説明は "reading" と、"paragraph" と "line" を含む

@id=EX-schema-064 @about=REQ-schema-066 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A8
Scenario: values の text 出力は入れ子ごとに空白2つで字下げする
  Given `配置パス`に "a.b" を宣言した`スキーマ`と、その値を持つ`文書`がある
  When "kotowari-mds values --format text" を実行する
  Then "b" の行は "a" の行より空白2つ深く字下げされる

@id=EX-schema-065 @about=REQ-schema-066 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A8
Scenario: values の text 出力は配列の要素に1から始まる番号を付ける
  Given 2つのデータ行を持つ`表`の`文書`と、その`表`を "header" を宣言せずに`抽出`する`スキーマ`がある
  When "kotowari-mds values --format text" を実行する
  Then 外側の配列の要素は "1." と "2." で始まり、要素の中の配列の要素も "1." から始まる
  And 配列の要素の行は、その配列の鍵の行より空白2つ深く字下げされる
```
