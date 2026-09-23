# スキーマ言語の骨格

この文書は、スキーマがどの規則種別を持つか、出現回数と条件付き規則をどう書くかを扱う。

## Requirements

### REQ-schema-016: スキーマの形

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A3, docs/decision/records/2026-09-21-mds-spec.md#A5, docs/decision/records/2026-09-23-ir-engine-gaps.md#A15, docs/decision/records/2026-09-23-ir-engine-gaps.md#A23
- verification: unit

mds は常に、`スキーマ`を YAML のマッピングとして読み、`題名`、`前置部`、`節`、文書の直下の`項目`の4つを根の`ノード`として受ける。文書の直下の`項目`は "document.item" に、`節`の下の "item" と同じ形で宣言する。

### REQ-schema-017: 規則種別の一覧

- kind: algorithm
- source: docs/decision/records/2026-09-21-mds-spec.md#A3, docs/decision/records/2026-09-21-mds-spec.md#A5, docs/decision/records/2026-09-21-mds-spec.md#A39, docs/decision/records/2026-09-22-ir-engine.md#A68
- definition: TBL-schema-004
- verification: review
- how_to_verify: `crates/kotowari-markdown-schema/src/schema.rs` の公開する構造体と TBL-schema-004 の行が1対1で対応し、表に無い規則種別が存在しないこと、各構造体が置ける場所が表の「置ける場所」の列と一致することを読んで確認する。表に無い規則種別が足されても検査は通ってしまうため、機械では見られない

### REQ-schema-060: 読み方の宣言

- kind: ubiquitous
- source: docs/decision/records/2026-09-23-ir-engine-gaps.md#A12, docs/decision/records/2026-09-23-ir-engine-gaps.md#A13, docs/decision/records/2026-09-23-ir-engine-gaps.md#A20, docs/decision/records/2026-09-23-ir-engine-gaps.md#A30
- verification: unit

mds は常に、`スキーマ`の最上位の "reading" の鍵を`読み方`の宣言として受け、値に "paragraph" と "line" の2つだけを受け、鍵を書かないときは "paragraph" として読む。"paragraph" と "line" のどちらでもない値の`スキーマ`は`停止`にする。

### REQ-schema-018: 知らないキー

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#P1
- verification: unit

`スキーマ`に規則種別が受けないキーがあるとき、mds は検査を行わずに`停止`する。

### REQ-schema-019: 出現回数の書き方

- kind: algorithm
- source: docs/decision/records/2026-09-21-mds-spec.md#A40
- definition: TBL-schema-005
- verification: unit

### REQ-schema-020: 条件付き規則

- kind: state_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A28
- verification: unit

`条件付き規則`の条件が真である間、mds はそれを添えた制約を適用し、偽である間は適用しない。

### REQ-schema-021: 条件が参照するフィールド行の探索

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A28
- verification: unit

mds は常に、`条件付き規則`が参照する`フィールド行`を同じ`ノード`の下だけから探し、見つからないときは等しい条件を偽、等しくない条件を真として扱う。

## Decision tables

### TBL-schema-004: 規則種別

- source: docs/decision/records/2026-09-21-mds-spec.md#A3, docs/decision/records/2026-09-21-mds-spec.md#A5, docs/decision/records/2026-09-21-mds-spec.md#A12, docs/decision/records/2026-09-21-mds-spec.md#A34, docs/decision/records/2026-09-21-mds-spec.md#A38, docs/decision/records/2026-09-21-mds-spec.md#A39, docs/decision/records/2026-09-22-ir-engine.md#A68, docs/decision/records/2026-09-23-ir-engine-gaps.md#A15, docs/decision/records/2026-09-23-ir-engine-gaps.md#A23

| 規則種別 | 何を検証するか | 置ける場所 |
|---|---|---|
| `題名` | 深さ1の見出し | `スキーマ`の根 |
| `前置部` | `題名`の後、最初の`節`より前の部分。文書の直下の`項目`を宣言したときは、最初の`節`か`項目`より前の部分 | `スキーマ`の根 |
| `節` | 深さ2の見出し | `スキーマ`の根 |
| `項目` | 深さ3の見出し | `節`の下、`スキーマ`の根（文書の直下の`項目`） |
| `フィールド行` | 名前と値の形の一覧の行 | `前置部`、`節`、`項目`、`箇条書き`の子 |
| `文` | 一覧でも`表`でもない空でない行 | `前置部`、`節`、`項目` |
| `箇条書き` | `フィールド行`でない一覧の行 | `前置部`、`節`、`項目`、`箇条書き`の子 |
| `表` | Markdown の表 | `前置部`、`節`、`項目` |
| `コードブロック` | フェンスで囲んだブロック | `前置部`、`節`、`項目` |

### TBL-schema-005: 出現回数の書き方

- source: docs/decision/records/2026-09-21-mds-spec.md#A40

| 書き方 | 意味 |
|---|---|
| 既定（何も書かない） | ちょうど1個 |
| 必須の宣言を偽にする | 0個か1個 |
| 下限だけを書く | その数以上 |
| 上限だけを書く | 0個からその数まで |
| 下限と上限を書く | その範囲 |

## Properties

### PROP-schema-004: 出現回数の宣言が抽出の形を決める

- source: docs/decision/records/2026-09-21-mds-spec.md#A4, docs/decision/records/2026-09-21-mds-spec.md#A49, docs/decision/records/2026-09-21-mds-spec.md#A50, docs/decision/records/2026-09-21-mds-spec.md#A55

`出現回数`の範囲を宣言した`ノード`の`抽出`は、値が1件でも配列になる。範囲を宣言しない`ノード`の`抽出`は単一の値になる。この対応が当たるのは、区切り文字を宣言しない`フィールド行`、`文`、`節`、`項目`、`題名`、`コードブロック`である。`箇条書き`と`表`は`出現回数`の宣言に関わらず常に配列になり、区切り文字を宣言した`フィールド行`の値も常に配列になる。

### PROP-schema-008: 既定の読み方は段落

- source: docs/decision/records/2026-09-23-ir-engine-gaps.md#A12, docs/decision/records/2026-09-23-ir-engine-gaps.md#A20

同じ`文書`に対して、"reading" を書かない`スキーマ`と、それに "reading: paragraph" だけを足した`スキーマ`は、同じ`指摘`の並びと同じ`抽出`の値を返す。

## Examples

```gherkin
@id=EX-schema-007 @about=REQ-schema-020 @source=docs/decision/records/2026-09-21-mds-spec.md#A28
Scenario: 条件が真のときだけ必須になる
  Given 別の`フィールド行`の値が特定の値のときだけ必須になる`フィールド行`を宣言した`スキーマ`がある
  When 条件を満たす`文書`から、その`フィールド行`を消して "kotowari-mds check" を実行する
  Then 欠落の`指摘`が出る

@id=EX-schema-008 @about=REQ-schema-018 @source=docs/decision/records/2026-09-21-mds-spec.md#P1
Scenario: 知らないキーのあるスキーマは停止する
  Given 規則種別が受けないキーを書いた`スキーマ`がある
  When "kotowari-mds check" を実行する
  Then 終了コードは 2 である

@id=EX-schema-040 @about=REQ-schema-060,PROP-schema-008 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A12,docs/decision/records/2026-09-23-ir-engine-gaps.md#A20,docs/decision/records/2026-09-21-mds-spec.md#A30
Scenario: reading を書かないスキーマは段落で読む
  Given "reading" を書かない`スキーマ`と、"reading: paragraph" を書いた`スキーマ`があり、どちらも`項目`の`文`に`出現回数`の上限1を宣言している
  And 空行を挟まずに続く2行の段落を持つ`項目`の`文書`がある
  When それぞれの`スキーマ`で "kotowari-mds check --format json" と "kotowari-mds values --format json" を実行する
  Then どちらでも`指摘`は出ず（2行の段落を1つの`文`と数える）、"kotowari-mds check" の出力どうしと "kotowari-mds values" の出力どうしはそれぞれ一致する

@id=EX-schema-041 @about=REQ-schema-060 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A20,docs/decision/records/2026-09-23-ir-engine-gaps.md#A30
Scenario: 受けない値の reading は停止する
  Given "reading: word" を書いた`スキーマ`がある
  When "kotowari-mds check" を実行する
  Then 終了コードは 2 である
```
