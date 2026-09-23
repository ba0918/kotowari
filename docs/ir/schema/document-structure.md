# 文書の骨格

この文書は、題名、前置部、節、項目という文書の骨格を、スキーマがどう検証するかを扱う。

## Requirements

### REQ-schema-022: 題名は1つ

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A5, docs/decision/records/2026-09-23-ir-engine-gaps.md#A17, docs/decision/records/2026-09-23-ir-engine-gaps.md#A12, docs/decision/records/2026-09-24-review2-gaps.md#A1, docs/decision/records/2026-09-24-review4-gaps.md#A1
- verification: unit

`スキーマ`が`題名`を宣言したとき、mds は`文書`が`題名`をちょうど1つ持つことを求め、無いときを1件の`指摘`にし、2つ以上あるときは2つ目以降の`題名`ごとに、その`題名`の行番号と`生の行`を持つ`指摘`を1件ずつ出す。`スキーマ`が`題名`に正規表現を宣言したときは、`題名`が2つ以上あっても1つ目の`題名`を照合し、合わなければその`指摘`も出す。`題名`を宣言しない`スキーマ`は`題名`を求めず、`文書`の`題名`は宣言していない見出しになる。

### REQ-schema-023: 前置部に宣言するもの

- kind: ubiquitous
- source: docs/decision/records/2026-09-22-ir-engine.md#A68
- verification: unit

mds は常に、`前置部`そのものには`抽出`の鍵を持たせず、`前置部`の内側の`ノード`（TBL-schema-004）に宣言させる。

### REQ-schema-024: 節の名前

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A5, docs/decision/records/2026-09-21-mds-spec.md#A2
- verification: unit

mds は常に、`節`を見出しの文字で見分け、`スキーマ`が宣言した名前と一致しない`節`を`指摘`にする。

### REQ-schema-025: 項目の見出しの形

- kind: algorithm
- source: docs/decision/records/2026-09-21-mds-spec.md#A5
- definition: TBL-schema-006
- verification: unit

### REQ-schema-026: 深すぎる見出し

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A5
- verification: unit

深さ4以上の見出しがあるとき、mds はその見出しを`指摘`にする。

### REQ-schema-027: 宣言していない項目

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A2, docs/decision/records/2026-09-21-mds-spec.md#A5
- verification: unit

`項目`を宣言していない`節`の中に深さ3の見出しがあるとき、mds はその見出しと、その内側の行を`指摘`にする。`開いた世界`でも同じである。

### REQ-schema-061: 文書の直下の項目

- kind: event_driven
- source: docs/decision/records/2026-09-23-ir-engine-gaps.md#A15, docs/decision/records/2026-09-23-ir-engine-gaps.md#A23, docs/decision/records/2026-09-23-ir-engine-gaps.md#A41, docs/decision/records/2026-09-23-ir-engine-gaps.md#A45
- verification: unit

`スキーマ`が文書の直下の`項目`を宣言したとき、mds は最初の`節`より前にある深さ3の見出しをその`項目`として読み、`前置部`を最初の`節`か`項目`の手前で終える。同じ`文書`に`節`の下の`項目`と文書の直下の`項目`の両方があってもよい。文書の直下の`項目`を宣言しない`スキーマ`では、`前置部`を宣言したときか`閉じた世界`のとき、最初の`節`より前にある深さ3の見出しと、その内側の行を`指摘`にする。`前置部`を宣言しない`開いた世界`では、未宣言の構造として許す（REQ-schema-002、REQ-schema-003）。

### REQ-schema-056: 必須の項目の欠落

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A69, docs/decision/records/2026-09-24-review4-gaps.md#A2
- verification: unit

mds は常に、`項目`が足りないときは、`出現回数`を書いたかどうかを問わず、下限を割ったものとして`指摘`にする。書かないときの下限は既定のちょうど1個（TBL-schema-005）の1である。`節`の欠落の`指摘`はこのとき出さない。

### REQ-schema-057: 出現回数の指摘の規則種別

- kind: ubiquitous
- source: docs/decision/records/2026-09-22-ir-engine.md#A86
- verification: unit

mds は常に、`出現回数`の下限と上限の`指摘`に、どの`規則種別`の`ノード`を数えたかを持たせる。

## Decision tables

### TBL-schema-006: 項目の見出しの読み方

- source: docs/decision/records/2026-09-21-mds-spec.md#A5, docs/decision/records/2026-09-21-mds-spec.md#A37

| 順 | 見出しの形 | 読み方 |
|---|---|---|
| 1 | 区切りのコロンが無い | ID の形に合わない`指摘` |
| 2 | コロンがあり、その前が`スキーマ`の正規表現に合う | ID と名前として読む |
| 3 | コロンがあり、その前が正規表現に合わない | ID の形に合わない`指摘` |

## Properties

### PROP-schema-005: 骨格の深さは3段で閉じている

- source: docs/decision/records/2026-09-21-mds-spec.md#A5

`スキーマ`が宣言できる見出しの深さは、`題名`、`節`、`項目`の3段だけであり、`項目`の下にさらに見出しの`ノード`を宣言する手段は無い。

## Examples

```gherkin
@id=EX-schema-009 @about=REQ-schema-025 @source=docs/decision/records/2026-09-21-mds-spec.md#A5
Scenario: 形に合わない項目の見出しは誤りになる
  Given `項目`の ID の正規表現を宣言した`スキーマ`がある
  When 正規表現に合わない ID を持つ`文書`で "kotowari-mds check" を実行する
  Then ID の形の`指摘`が出る

@id=EX-schema-072 @about=REQ-schema-022 @source=docs/decision/records/2026-09-24-review2-gaps.md#A1
Scenario: 題名が2つあっても1つ目の題名を正規表現と照合する
  Given `題名`に正規表現を宣言した`スキーマ`がある
  And 正規表現に合わない`題名`の後に、2つ目の`題名`を持つ`文書`がある
  When "kotowari-mds check --format json" を実行する
  Then 1つ目の`題名`の行に正規表現に合わない`指摘`が出る
  And 2つ目の`題名`の行に`題名`が複数ある`指摘`が出る

@id=EX-schema-074 @about=REQ-schema-022 @source=docs/decision/records/2026-09-24-review4-gaps.md#A1
Scenario: 題名を宣言しないスキーマは題名を求めない
  Given `題名`を宣言せず、`前置部`に`文`を宣言した`スキーマ`がある
  And `題名`の無い`文書`がある
  When "kotowari-mds check" を実行する
  Then 終了コードは 0 である

@id=EX-schema-075 @about=REQ-schema-056 @source=docs/decision/records/2026-09-24-review4-gaps.md#A2
Scenario: 出現回数を書かない項目が無いと下限を割った指摘になる
  Given `出現回数`を書かない`項目`を`節`に宣言した`スキーマ`がある
  And その`節`に`項目`の無い`文書`がある
  When "kotowari-mds check --format json" を実行する
  Then その`節`の行に下限を割った`指摘`が1件だけ出る

@id=EX-schema-010 @about=REQ-schema-022 @source=docs/decision/records/2026-09-21-mds-spec.md#A5
Scenario: 題名が2つある文書は誤りになる
  Given 深さ1の見出しを2つ持つ`文書`がある
  When "kotowari-mds check" を実行する
  Then `題名`が複数ある`指摘`が出る

@id=EX-schema-042 @about=REQ-schema-022 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A17
Scenario: 題名が3つある文書は2件の指摘になる
  Given 深さ1の見出しを3つ持つ`文書`がある
  When "kotowari-mds check --format json" を実行する
  Then `題名`が複数ある`指摘`が2件出る
  And 2件の行番号と`生の行`は、2つ目と3つ目の`題名`の行とその文字そのままである

@id=EX-schema-043 @about=REQ-schema-061 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A15,docs/decision/records/2026-09-23-ir-engine-gaps.md#A23
Scenario: 文書の直下の項目を宣言すると節の前の項目を読む
  Given "document.item" と`節`の下の "item" に、同じ形の`項目`を別の`配置パス`の`抽出`とともに宣言した`スキーマ`がある
  And `前置部`の`文`の後に、`節`を挟まない深さ3の見出しの`項目`と、`節`の下の`項目`を持つ`文書`がある
  When "kotowari-mds check --format json" と "kotowari-mds values --format json" を実行する
  Then `指摘`は出ず、どちらの`項目`も`抽出`の値に出る
  And `前置部`の`文`の値に文書の直下の`項目`の行は入らない

@id=EX-schema-044 @about=REQ-schema-061 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A23
Scenario: 文書の直下の項目を宣言しなければ節の前の深さ3の見出しは指摘になる
  Given `節`の下にだけ`項目`を宣言した`スキーマ`がある
  And 最初の`節`より前に深さ3の見出しを持つ`文書`がある
  When "kotowari-mds check --format json" を実行する
  Then その見出しに`指摘`が出る

@id=EX-schema-053 @about=REQ-schema-061 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A45
Scenario: 前置部を宣言しない開いた世界では節の前の深さ3の見出しを許す
  Given "open: true" で、文書の直下の`項目`も`前置部`も宣言しない`スキーマ`がある
  And 最初の`節`より前に "### X-1: a" の見出しとその下の行を持つ`文書`がある
  When "kotowari-mds check --format json" を実行する
  Then その見出しにもその下の行にも`指摘`は出ない
```
