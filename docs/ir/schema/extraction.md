---
$schema: ../../../.mds/schemas/ir.yaml
---
# 抽出と素の構文木

この文書は、スキーマが宣言した抽出規則から値を組み立てる振る舞いと、検査と別に素の構文木を出す振る舞いを扱う。

## 要求

### REQ-schema-035: 抽出の書式

- 種類: algorithm
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A4
- 定義: TBL-schema-008
- 検証: unit

### REQ-schema-036: 配置パス

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A4
- 検証: unit

mds は常に、`抽出`した値を`配置パス`のドット区切りの名前に沿って入れ子にして置く。

### REQ-schema-037: 値の型は文字列

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A4
- 検証: property

mds は常に、`文書`から取り出した`抽出`の値を文字列として出し、日付や数値への型変換をしない。エンジンが導く位置情報はこの規則の対象外である。

### REQ-schema-038: 欠けた値はキーを出さない

- 種類: event_driven
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A4
- 検証: unit

`抽出`の対象の`ノード`が`文書`に無いとき、mds はその`配置パス`のキーを出力に出さない。

### REQ-schema-039: 置き場の無い内側の抽出

- 種類: event_driven
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A4, docs/decision/records/2026-09-21-mds-spec.md#A22
- 検証: unit

`項目`の内側の`フィールド行`、`文`、`箇条書き`、`表`、`コードブロック`が`抽出`を宣言し、その`項目`自身が`抽出`を宣言していないとき、mds は`停止`する。

### REQ-schema-047: 項目のオブジェクトの形

- 種類: state_driven
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A22
- 検証: unit

`項目`の内側の`ノード`が`抽出`を宣言している間、または`項目`の`抽出`が`要素の値`の置き場か`導かれる値`を宣言している間、mds は`項目`ごとに1つのオブジェクトを組み立て、内側の`配置パス`をそのオブジェクトの中の相対パスとして解決する。どれでもない間は、見出しと本文をつないだ1つの文字列にする。

### REQ-schema-048: 導かれる値

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A22, docs/decision/records/2026-09-21-mds-spec.md#A23
- 検証: unit

mds は常に、`ノード`の`抽出`の宣言を1つの入れ子として受け、"path" に`配置パス`をちょうど1つ、"value" に`要素の値`の置き場を、"of" に`導かれる値`を、"group" に名前付きキャプチャを取る。`導かれる値`は行番号、`項目`の見出しの ID、`項目`の見出しの名前、`生の行`の4つで、行番号は数値で出し、ほかの語は`停止`にする。`ノード`が "value" か`導かれる値`を宣言したときは、その`ノード`の要素ごとに1つのオブジェクトを組み立て、"value" と "of" の鍵と内側の`ノード`の`配置パス`を、そのオブジェクトの中の相対パスとして解く。

### REQ-schema-040: 素の構文木

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A19
- 検証: review
- 確かめ方: `mds ast` の出力を mdast（unist）の仕様と突き合わせ、ノードの "type" の名前、"children" の入れ子、インライン要素の種別が準拠していることを確認する。準拠は外部の仕様との一致なので、自分のテストでは見られない

mds は常に、素の構文木を mdast に沿った JSON で出し、インライン要素まで含め、位置情報は含めない。

### REQ-schema-045: 区切り文字を宣言したフィールド行の抽出

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A4, docs/decision/records/2026-09-21-mds-spec.md#A10
- 検証: unit

mds は常に、区切り文字を宣言した`フィールド行`を`出現回数`の宣言に関わらず配列へ`抽出`し、`出現回数`の範囲も宣言したときは配列の配列にする。

### REQ-schema-046: 区切りと継続段落の順序

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A4, docs/decision/records/2026-09-21-mds-spec.md#A11
- 検証: unit

mds は常に、区切り文字による分割を`継続段落`を含めない値だけに対して行い、`継続段落`は分割した末尾の要素に改行を挟んで付ける。

## 決定表

### TBL-schema-008: 抽出の形

- 出典: docs/decision/records/2026-09-21-mds-spec.md#A4, docs/decision/records/2026-09-21-mds-spec.md#A13

どの`ノード`でも、"value" も`導かれる値`も宣言しなければ下の「略記の形」をそのまま出し、どちらかを宣言すれば要素ごとにオブジェクトを組み立てて、`要素の値`を "value" の鍵に、`導かれる値`を "of" の鍵に置く。

| `ノード` | 要素の単位 | 略記の形 |
|---|---|---|
| `題名` | 分けない | 見出しの文字列。"group" を宣言したときは正規表現の名前付きキャプチャ |
| `フィールド行` | 分けない | 値の文字列。区切り文字を宣言すれば文字列の配列 |
| `文` | `導かれる値`を宣言したときだけ行、宣言しなければ分けない | 本文の文字列 |
| `節` | 分けない | `文`と`箇条書き`だけをつないだ本文の文字列 |
| `項目` | `項目` | 見出しと本文をつないだ1つの文字列、または内側の`配置パス`をキーにしたオブジェクト（REQ-schema-047） |
| `箇条書き` | 行 | 元の行を保った文字列の配列 |
| `表` | データ行 | 行の配列。行の鍵は "header" を宣言すればその名前、宣言しなければ列の位置（配列）にし、`文書`のヘッダ行の文字は鍵に使わない。`表`が繰り返すときは表ごとの配列になる |
| `コードブロック` | ブロック | ブロック全体の文字列 |

## 性質

### PROP-schema-007: 抽出は閉じた世界の設定に依らない

- 出典: docs/decision/records/2026-09-21-mds-spec.md#A2, docs/decision/records/2026-09-21-mds-spec.md#A4

同じ`文書`と同じ`スキーマ`であれば、`抽出`の結果は`閉じた世界`と`開いた世界`のどちらで検査しても変わらない。

## 具体例

```gherkin
@id=EX-schema-013 @about=REQ-schema-036 @source=docs/decision/records/2026-09-21-mds-spec.md#A4
Scenario: 配置パスに沿って入れ子の JSON を出す
  Given ドットを含む`配置パス`を宣言した`スキーマ`がある
  When "mds values --format json" を実行する
  Then 値はドットで区切った名前の入れ子として出る

@id=EX-schema-014 @about=REQ-schema-039 @source=docs/decision/records/2026-09-21-mds-spec.md#A12
Scenario: 項目の内側の抽出は停止する
  Given `項目`の中の`表`に`抽出`を宣言した`スキーマ`がある
  When "mds check" を実行する
  Then 終了コードは 2 である

@id=EX-schema-018 @about=TBL-schema-008,REQ-schema-035 @source=docs/decision/records/2026-09-22-ir-engine.md#A43,docs/decision/records/2026-09-22-ir-engine.md#A44,docs/decision/records/2026-09-22-ir-engine.md#A48
Scenario: 表の行の鍵はスキーマの宣言か列の位置から取る
  Given ヘッダのセルが空の`表`と、同じ名前の列が2つある`表`を持つ`文書`がある
  When "mds values --format json" を実行する
  Then "header" を宣言した`表`の行は、宣言した名前を鍵にしたオブジェクトになる
  And "header" を宣言しない`表`の行は列の位置の配列になる
  And どちらの`表`でも列の値は1つも失われない

@id=EX-schema-019 @about=TBL-schema-008,REQ-schema-048 @source=docs/decision/records/2026-09-22-ir-engine.md#A13,docs/decision/records/2026-09-22-ir-engine.md#A59
Scenario: 導かれる値を宣言した表の行番号はデータ行を指す
  Given ヘッダとデータ3行を持つ`表`に`導かれる値`を宣言した`スキーマ`がある
  When "mds values --format json" を実行する
  Then 行ごとの行番号はその`表`のデータ行の行番号と一致する
  And `表`が繰り返すときは`配置パス`の直下に`表`ごとの段ができる

@id=EX-schema-020 @about=REQ-schema-048,TBL-schema-008 @source=docs/decision/records/2026-09-22-ir-engine.md#A37,docs/decision/records/2026-09-22-ir-engine.md#A57,docs/decision/records/2026-09-22-ir-engine.md#A66
Scenario: 導かれる値を宣言した文は行ごとの要素になる
  Given 3行の`文`と空行を挟んで続く段落を持つ`項目`があり、`文`に`導かれる値`を宣言した`スキーマ`がある
  When "mds values --format json" を実行する
  Then 行の数と同じ数の要素が出る
  And 字下げのある行の`要素の値`は前後の空白を取り除いた文字になる
  And 同じ行の`生の行`は字下げと末尾の空白を含む文字になる
  And 一覧の行の`継続段落`は要素に入らない

@id=EX-schema-021 @about=TBL-schema-008,REQ-schema-048 @source=docs/decision/records/2026-09-22-ir-engine.md#A55,docs/decision/records/2026-09-22-ir-engine.md#A63
Scenario: 略記の抽出は要素に分けない
  Given "extract: text" と "extract: rows" の略記だけを宣言した`スキーマ`がある
  When "mds values --format json" を実行する
  Then `文`の値は1つの文字列になり、`表`の値は行の並びになる
  And 要素ごとのオブジェクトは作らない

@id=EX-schema-022 @about=REQ-schema-048 @source=docs/decision/records/2026-09-22-ir-engine.md#A45,docs/decision/records/2026-09-22-ir-engine.md#A52
Scenario: 配置パスの無い抽出は停止する
  Given "path" を書かない`抽出`を宣言した`スキーマ`がある
  When "mds values" を実行する
  Then 終了コードは 2 である

@id=EX-schema-023 @about=REQ-schema-048 @source=docs/decision/records/2026-09-22-ir-engine.md#A60
Scenario: value を省くと要素は導かれる値の鍵だけを持つ
  Given "value" を書かず "of" だけを書いた`抽出`を宣言した`スキーマ`がある
  When "mds values --format json" を実行する
  Then 要素のオブジェクトは`導かれる値`の鍵と、内側の`ノード`が宣言した`配置パス`だけを持つ

@id=EX-schema-024 @about=REQ-schema-048 @source=docs/decision/records/2026-09-22-ir-engine.md#A49,docs/decision/records/2026-09-22-ir-engine.md#A54
Scenario: 項目にだけ宣言できる導かれる値を表に宣言すると停止する
  Given `表`に見出しの ID を取る`導かれる値`を宣言した`スキーマ`がある
  When "mds values" を実行する
  Then 終了コードは 2 である

@id=EX-schema-025 @about=REQ-schema-048,TBL-schema-008 @source=docs/decision/records/2026-09-22-ir-engine.md#A49,docs/decision/records/2026-09-22-ir-engine.md#A50,docs/decision/records/2026-09-22-ir-engine.md#A58
Scenario: 生の行はどのノードにも宣言できる
  Given `題名`と`表`の行と`項目`と`コードブロック`に`生の行`と行番号を宣言した`スキーマ`がある
  When "mds values --format json" を実行する
  Then `題名`は見出しの行、`表`の行はそのデータ行、`項目`は見出しの行をそのまま出す
  And `コードブロック`はブロックごとにフェンスの開始行を出す
```
