# 抽出と素の構文木

この文書は、スキーマが宣言した抽出規則から値を組み立てる振る舞いと、検査と別に素の構文木を出す振る舞いを扱う。

## Requirements

### REQ-schema-035: 抽出の書式

- kind: algorithm
- source: docs/decision/records/2026-09-21-mds-spec.md#A4
- definition: TBL-schema-008
- verification: unit

### REQ-schema-036: 配置パス

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A4, docs/decision/records/2026-09-21-mds-spec.md#A46
- verification: unit

mds は常に、`抽出`した値を`配置パス`のドット区切りの名前に沿って入れ子にして置く。

### REQ-schema-037: 値の型は文字列

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A4, docs/decision/records/2026-09-21-mds-spec.md#A23
- verification: property

mds は常に、`文書`から取り出した`抽出`の値を文字列として出し、日付や数値への型変換をしない。エンジンが導く位置情報はこの規則の対象外である。

### REQ-schema-038: 欠けた値はキーを出さない

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A4, docs/decision/records/2026-09-21-mds-spec.md#A47
- verification: unit

`抽出`の対象の`ノード`が`文書`に無いとき、mds はその`配置パス`のキーを出力に出さない。

### REQ-schema-039: 置き場の無い内側の抽出

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A4, docs/decision/records/2026-09-21-mds-spec.md#A22, docs/decision/records/2026-09-21-mds-spec.md#A48, docs/decision/records/2026-09-21-mds-spec.md#A68
- verification: unit

`項目`の内側の`フィールド行`、`文`、`箇条書き`、`表`、`コードブロック`が`抽出`を宣言し、その`項目`自身が`抽出`を宣言していないとき、mds は`停止`する。`箇条書き`の子の`フィールド行`も`項目`の内側に数える。

### REQ-schema-047: 項目のオブジェクトの形

- kind: state_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A22, docs/decision/records/2026-09-22-ir-engine.md#A52, docs/decision/records/2026-09-22-ir-engine.md#A60, docs/decision/records/2026-09-22-ir-engine.md#A69
- verification: unit

`項目`の内側の`ノード`が`抽出`を宣言している間、または`項目`の`抽出`が`要素の値`の置き場か`導かれる値`を宣言している間、mds は`項目`ごとに1つのオブジェクトを組み立て、内側の`配置パス`をそのオブジェクトの中の相対パスとして解決する。どれでもない間は、オブジェクトを組み立てず TBL-schema-008 の略記の形にする。

### REQ-schema-048: 導かれる値

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A22, docs/decision/records/2026-09-21-mds-spec.md#A23, docs/decision/records/2026-09-21-mds-spec.md#A54, docs/decision/records/2026-09-21-mds-spec.md#A64, docs/decision/records/2026-09-21-mds-spec.md#P1, docs/decision/records/2026-09-22-ir-engine.md#A45, docs/decision/records/2026-09-22-ir-engine.md#A49, docs/decision/records/2026-09-22-ir-engine.md#A52, docs/decision/records/2026-09-22-ir-engine.md#A53, docs/decision/records/2026-09-22-ir-engine.md#A56, docs/decision/records/2026-09-22-ir-engine.md#A58, docs/decision/records/2026-09-22-ir-engine.md#A69, docs/decision/records/2026-09-23-ir-engine-gaps.md#A18, docs/decision/records/2026-09-23-ir-engine-gaps.md#A24
- verification: unit

mds は常に、`ノード`の`抽出`の宣言を1つの入れ子として受け、"path" に`配置パス`をちょうど1つ、"value" に`要素の値`の置き場を、"of" に`導かれる値`を取る。"group" の名前付きキャプチャは`題名`にだけ宣言でき、ほかの`ノード`に宣言した`スキーマ`と、`題名`に正規表現が無いか指定の名前付きキャプチャを含まない`スキーマ`は`停止`にする。`導かれる値`は行番号、`項目`の見出しの ID、`項目`の見出しの名前、`生の行`、要素の最後の行（"end"。REQ-schema-062）の5つで、行番号と要素の最後の行は数値で出し、ほかの語は`停止`にする。`項目`の見出しの ID と名前は`項目`にだけ宣言でき、`項目`の外の`ノード`に宣言した`スキーマ`は`停止`にする。要素の最後の行は`項目`と`節`にだけ宣言でき、ほかの`ノード`に宣言した`スキーマ`は`停止`にする。`ノード`が "value" か`導かれる値`を宣言したときは、その`ノード`の要素ごとに1つのオブジェクトを組み立て、"value" と "of" の鍵と内側の`ノード`の`配置パス`を、そのオブジェクトの中の相対パスとして解く。

### REQ-schema-062: 要素の最後の行

- kind: ubiquitous
- source: docs/decision/records/2026-09-23-ir-engine-gaps.md#A18, docs/decision/records/2026-09-23-ir-engine-gaps.md#A24
- verification: unit

mds は常に、`項目`と`節`の`導かれる値`の "end" を、その見出しの後で次に現れる同じ深さかそれより浅い見出しの手前の行の行番号にし、そのような見出しが無ければ`文書`の最後の行の行番号にする。範囲の末尾の空行も最後の行に含める。

### REQ-schema-040: 素の構文木

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A19
- verification: review
- how_to_verify: "kotowari-mds ast" の出力を mdast（unist）の仕様と突き合わせ、ノードの "type" の名前、"children" の入れ子、インライン要素の種別が準拠していることを確認する。準拠は外部の仕様との一致なので、自分のテストでは見られない

mds は常に、素の構文木を mdast に沿った JSON で出し、インライン要素まで含め、位置情報は含めない。

### REQ-schema-045: 区切り文字を宣言したフィールド行の抽出

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A4, docs/decision/records/2026-09-21-mds-spec.md#A10, docs/decision/records/2026-09-21-mds-spec.md#A50
- verification: unit

mds は常に、区切り文字を宣言した`フィールド行`を`出現回数`の宣言に関わらず配列へ`抽出`し、`出現回数`の範囲も宣言したときは配列の配列にする。`要素の値`の置き場か`導かれる値`も宣言したときは、この配列を`要素の値`の鍵の下に置く。

### REQ-schema-046: 区切りと継続段落の順序

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A4, docs/decision/records/2026-09-21-mds-spec.md#A11, docs/decision/records/2026-09-21-mds-spec.md#A50, docs/decision/records/2026-09-23-extract-original-lines.md#A7
- verification: unit

mds は常に、区切り文字による分割を`継続段落`を含めない値だけに対して行い、`継続段落`は分割した末尾の要素に REQ-schema-063 のつなぎ方で付ける。

### REQ-schema-063: 値の行は元の行のまま

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A11, docs/decision/records/2026-09-23-extract-original-lines.md#A1, docs/decision/records/2026-09-23-extract-original-lines.md#A2, docs/decision/records/2026-09-23-extract-original-lines.md#A3, docs/decision/records/2026-09-23-extract-original-lines.md#A4, docs/decision/records/2026-09-23-extract-original-lines.md#A5, docs/decision/records/2026-09-23-extract-original-lines.md#A6
- verification: unit

mds は常に、`要素の値`に入る行を`文書`の元の行のまま使い、字下げと途中の行の末尾の空白を残し、値の末尾の空白と空行を除く。値は`箇条書き`ではマーカーから、`フィールド行`では名前の後のコロンと空白の後から、`文`では段落の最初の空白でない文字から始める。複数の部分から成る値は、含める部分の間に元の`文書`で空行があれば空行1つで、無ければ改行1つでつなぎ、含めない部分の行を抜いた跡と続いた空行は空行1つにまとめ、空白だけの行は空行として扱う。この規則は`読み方`に依らない。`導かれる値`を宣言して行ごとに分けた`文`の`要素の値`は、この規則の対象外で TBL-schema-008 のとおりにする。

### REQ-schema-064: 見出しの名前とセルの値のインラインの記法

- kind: ubiquitous
- source: docs/decision/records/2026-09-23-mutants-gaps.md#A4
- verification: unit

mds は常に、見出しの名前と`表`のセルの値を、インラインの記法の記号を外した文字で読む。強調、太字、リンク、参照リンクは中の文字をつなげ、インラインコードは中身を残す。

## Decision tables

### TBL-schema-008: 抽出の形

- source: docs/decision/records/2026-09-21-mds-spec.md#A4, docs/decision/records/2026-09-21-mds-spec.md#A13, docs/decision/records/2026-09-21-mds-spec.md#A65, docs/decision/records/2026-09-21-mds-spec.md#A66, docs/decision/records/2026-09-21-mds-spec.md#A50, docs/decision/records/2026-09-21-mds-spec.md#A51, docs/decision/records/2026-09-21-mds-spec.md#A52, docs/decision/records/2026-09-21-mds-spec.md#A53, docs/decision/records/2026-09-21-mds-spec.md#A55, docs/decision/records/2026-09-22-ir-engine.md#A43, docs/decision/records/2026-09-22-ir-engine.md#A44, docs/decision/records/2026-09-22-ir-engine.md#A48, docs/decision/records/2026-09-22-ir-engine.md#A53, docs/decision/records/2026-09-22-ir-engine.md#A57, docs/decision/records/2026-09-22-ir-engine.md#A59, docs/decision/records/2026-09-22-ir-engine.md#A64, docs/decision/records/2026-09-22-ir-engine.md#A69, docs/decision/records/2026-09-22-ir-engine.md#A70, docs/decision/records/2026-09-23-ir-engine-gaps.md#A16, docs/decision/records/2026-09-23-extract-original-lines.md#A3, docs/decision/records/2026-09-23-mutants-gaps.md#A5, docs/decision/records/2026-09-23-mutants-gaps.md#A10

どの`ノード`でも、"value" も`導かれる値`も宣言しなければ下の「略記の形」をそのまま出し、どちらかを宣言すれば要素ごとにオブジェクトを組み立てて、`要素の値`を "value" の鍵に、`導かれる値`を "of" の鍵に置く。

| `ノード` | 要素の単位 | `要素の値` | `生の行`が指す行 | 略記の形 |
|---|---|---|---|---|
| `題名` | 分けない | 見出しの文字列。"group" を宣言したときは名前付きキャプチャが捕まえた文字 | `題名`の見出しの行 | 見出しの文字列。"group" を宣言したときは正規表現の名前付きキャプチャ |
| `フィールド行` | 分けない | 値の文字列（REQ-schema-063）。区切り文字を宣言したときは区切った文字列の配列（REQ-schema-045） | その`フィールド行`の行 | 値の文字列。区切り文字を宣言したときは区切った文字列の配列（REQ-schema-045） |
| `文` | `導かれる値`を宣言したときだけ行、宣言しなければ分けない | 行に分けたときは前後の空白を取り除いた行の文字、分けないときは本文の文字列（REQ-schema-063） | その行 | 本文の文字列（REQ-schema-063） |
| `節` | 分けない | `文`と`箇条書き`の行だけをつないだ本文の文字列（REQ-schema-063）。`フィールド行`、`表`、`コードブロック`、`項目`は含めない | `節`の見出しの行 | `文`と`箇条書き`の行だけをつないだ本文の文字列（REQ-schema-063）。`フィールド行`、`表`、`コードブロック`、`項目`は含めない |
| `項目` | `項目` | 見出しと本文を改行でつないだ文字列。見出しは `ID` と名前だけにし、本文に`フィールド行`・`文`・`箇条書き`を含め、`表`と`コードブロック`は含めない。宣言した`フィールド行`はその行と`継続段落`を入れて子の`箇条書き`を入れず、宣言していない行は子の行まで入れる。本文は REQ-schema-063 のとおりにつなぐ | `項目`の見出しの行 | 内側の`配置パス`をキーにしたオブジェクト（REQ-schema-047）。オブジェクトを組み立てないときは見出しと本文を改行でつないだ文字列。見出しは `ID` と名前だけにし、本文に`フィールド行`・`文`・`箇条書き`を含め、`表`と`コードブロック`は含めない。宣言した`フィールド行`はその行と`継続段落`を入れて子の`箇条書き`を入れず、宣言していない行は子の行まで入れる。本文は REQ-schema-063 のとおりにつなぐ |
| `箇条書き` | 行 | 元の行（`継続段落`と子の`箇条書き`の行を含む。REQ-schema-063） | その行のマーカーの行 | 元の行を保った文字列の配列（REQ-schema-063） |
| `表` | データ行 | データ行の値。`文書`のヘッダ行より多いセルは捨てる（REQ-schema-033）。鍵は "header" を宣言すればその名前、宣言しなければ列の位置（配列）。`要素の値`の置き場か`導かれる値`を宣言した`表`に`出現回数`の範囲も宣言したときは`配置パス`の直下に`表`ごとの段を作り、それ以外は同じ置き場の`表`のデータ行を現れた順に1つの配列へつなぐ | そのデータ行の行 | 行の配列。行の鍵は "header" を宣言すればその名前、宣言しなければ列の位置（配列）にし、`文書`のヘッダ行の文字は鍵に使わない |
| `コードブロック` | ブロック | ブロック全体の文字列 | フェンスの開始行 | ブロック全体の文字列 |

## Properties

### PROP-schema-007: 抽出は閉じた世界の設定に依らない

- source: docs/decision/records/2026-09-21-mds-spec.md#A2, docs/decision/records/2026-09-21-mds-spec.md#A4, docs/decision/records/2026-09-21-mds-spec.md#A56, docs/decision/records/2026-09-21-mds-spec.md#A59

同じ`文書`と同じ`スキーマ`であれば、`抽出`の結果は`閉じた世界`と`開いた世界`のどちらで検査しても変わらない。

## Examples

```gherkin
@id=EX-schema-013 @about=REQ-schema-036 @source=docs/decision/records/2026-09-21-mds-spec.md#A4
Scenario: 配置パスに沿って入れ子の JSON を出す
  Given ドットを含む`配置パス`を宣言した`スキーマ`がある
  When "kotowari-mds values --format json" を実行する
  Then 値はドットで区切った名前の入れ子として出る

@id=EX-schema-014 @about=REQ-schema-039 @source=docs/decision/records/2026-09-21-mds-spec.md#A12,docs/decision/records/2026-09-21-mds-spec.md#A48
Scenario: 項目の内側の抽出は停止する
  Given `項目`自身は`抽出`を宣言せず、`項目`の中の`表`にだけ`抽出`を宣言した`スキーマ`がある
  When "kotowari-mds check" を実行する
  Then 終了コードは 2 である

@id=EX-schema-018 @about=TBL-schema-008,REQ-schema-035 @source=docs/decision/records/2026-09-22-ir-engine.md#A43,docs/decision/records/2026-09-22-ir-engine.md#A44,docs/decision/records/2026-09-22-ir-engine.md#A48
Scenario: 表の行の鍵はスキーマの宣言か列の位置から取る
  Given ヘッダのセルが空の`表`と、同じ名前の列が2つある`表`を持つ`文書`がある
  When "kotowari-mds values --format json" を実行する
  Then "header" を宣言した`表`の行は、宣言した名前を鍵にしたオブジェクトになる
  And "header" を宣言しない`表`の行は列の位置の配列になる
  And どちらの`表`でも列の値は1つも失われない

@id=EX-schema-019 @about=TBL-schema-008,REQ-schema-048 @source=docs/decision/records/2026-09-22-ir-engine.md#A13,docs/decision/records/2026-09-22-ir-engine.md#A59
Scenario: 導かれる値を宣言した表の行番号はデータ行を指す
  Given ヘッダとデータ3行を持つ`表`が2つあり、その`表`に`出現回数`と`導かれる値`を宣言した`スキーマ`がある
  When "kotowari-mds values --format json" を実行する
  Then 行ごとの行番号はその`表`のデータ行の行番号と一致する
  And `配置パス`の直下は`表`ごとの段になる

@id=EX-schema-020 @about=REQ-schema-048,TBL-schema-008 @source=docs/decision/records/2026-09-22-ir-engine.md#A37,docs/decision/records/2026-09-22-ir-engine.md#A57,docs/decision/records/2026-09-22-ir-engine.md#A66
Scenario: 導かれる値を宣言した文は行ごとの要素になる
  Given 3行の`文`と空行を挟んで続く段落を持つ`項目`があり、`文`に`導かれる値`を宣言した`スキーマ`がある
  When "kotowari-mds values --format json" を実行する
  Then 行の数と同じ数の要素が出る
  And 字下げのある行の`要素の値`は前後の空白を取り除いた文字になる
  And 同じ行の`生の行`は字下げと末尾の空白を含む文字になる
  And 一覧の行の`継続段落`は要素に入らない

@id=EX-schema-021 @about=TBL-schema-008,REQ-schema-048 @source=docs/decision/records/2026-09-22-ir-engine.md#A55,docs/decision/records/2026-09-22-ir-engine.md#A63
Scenario: 略記の抽出は要素に分けない
  Given "extract: text" と "extract: rows" の略記だけを宣言した`スキーマ`がある
  When "kotowari-mds values --format json" を実行する
  Then `文`の値は1つの文字列になり、`表`の値は行の並びになる
  And 要素ごとのオブジェクトは作らない

@id=EX-schema-022 @about=REQ-schema-048 @source=docs/decision/records/2026-09-22-ir-engine.md#A45,docs/decision/records/2026-09-22-ir-engine.md#A52,docs/decision/records/2026-09-21-mds-spec.md#A15,docs/decision/records/2026-09-21-mds-spec.md#P1
Scenario: 配置パスの無い抽出は停止する
  Given "path" を書かない`抽出`を宣言した`スキーマ`がある
  When "kotowari-mds values" を実行する
  Then 終了コードは 2 である

@id=EX-schema-023 @about=REQ-schema-048 @source=docs/decision/records/2026-09-22-ir-engine.md#A60,docs/decision/records/2026-09-22-ir-engine.md#A56
Scenario: value を省くと要素は導かれる値の鍵だけを持つ
  Given "value" を書かず "of" だけを書いた`抽出`を宣言した`スキーマ`がある
  When "kotowari-mds values --format json" を実行する
  Then 要素のオブジェクトは`導かれる値`の鍵と、内側の`ノード`が宣言した`配置パス`だけを持つ

@id=EX-schema-024 @about=REQ-schema-048 @source=docs/decision/records/2026-09-22-ir-engine.md#A17,docs/decision/records/2026-09-22-ir-engine.md#A49,docs/decision/records/2026-09-21-mds-spec.md#A23,docs/decision/records/2026-09-21-mds-spec.md#A15,docs/decision/records/2026-09-21-mds-spec.md#P1,docs/decision/records/2026-09-23-ir-engine-gaps.md#A18,docs/decision/records/2026-09-23-ir-engine-gaps.md#A24
Scenario: 受けない語の導かれる値は停止する
  Given `導かれる値`に5つのどれでもない語を宣言した`スキーマ`がある
  When "kotowari-mds values" を実行する
  Then 終了コードは 2 である

@id=EX-schema-025 @about=REQ-schema-048,TBL-schema-008 @source=docs/decision/records/2026-09-22-ir-engine.md#A49,docs/decision/records/2026-09-22-ir-engine.md#A50,docs/decision/records/2026-09-22-ir-engine.md#A54
Scenario: 生の行はどのノードにも宣言できる
  Given `題名`と`表`の行に`生の行`と行番号を宣言した`スキーマ`がある
  When "kotowari-mds values --format json" を実行する
  Then `題名`は見出しの行、`表`の行はそのデータ行をそのまま出す

@id=EX-schema-045 @about=REQ-schema-062,REQ-schema-048 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A18,docs/decision/records/2026-09-23-ir-engine-gaps.md#A24
Scenario: 要素の最後の行は次の同じ深さか浅い見出しの手前になる
  Given `項目`と`節`に行番号と "end" の`導かれる値`を宣言した`スキーマ`がある
  And 1つ目の`節`に2つの`項目`を持ち、2つ目の`項目`の後に空行を挟んで2つ目の`節`が続き、2つ目の`節`が`文書`の最後まで続く`文書`がある
  When "kotowari-mds values --format json" を実行する
  Then 1つ目の`項目`の "end" は2つ目の`項目`の見出しの前の行である
  And 2つ目の`項目`と1つ目の`節`の "end" は2つ目の`節`の見出しの前の空行である
  And 2つ目の`節`の "end" は`文書`の最後の行である

@id=EX-schema-046 @about=REQ-schema-048 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A24,docs/decision/records/2026-09-21-mds-spec.md#P1
Scenario: 項目と節の外の要素の最後の行は停止する
  Given `表`の`抽出`に "end" の`導かれる値`を宣言した`スキーマ`がある
  When "kotowari-mds values" を実行する
  Then 終了コードは 2 である

@id=EX-schema-054 @about=REQ-schema-063,TBL-schema-008 @source=docs/decision/records/2026-09-23-extract-original-lines.md#A1,docs/decision/records/2026-09-23-extract-original-lines.md#A3,docs/decision/records/2026-09-23-extract-original-lines.md#A4
Scenario: 箇条書きの値は字下げと空行を元の行のまま持つ
  Given 2行の lead 段落と、空行を挟んだ`継続段落`と、空行を挟んだ子の`箇条書き`を持つ`箇条書き`と、マーカーだけの行の後に lead 段落と`継続段落`を持つ`箇条書き`の`文書`がある
  When "kotowari-mds values --format json" を実行する
  Then どちらの`箇条書き`の値も、マーカーの後の行を字下げを含む元の行のまま持つ
  And lead 段落と`継続段落`の間と、`継続段落`と子の`箇条書き`の間に空行が1つずつ入る

@id=EX-schema-055 @about=REQ-schema-063,REQ-schema-046 @source=docs/decision/records/2026-09-23-extract-original-lines.md#A1,docs/decision/records/2026-09-23-extract-original-lines.md#A2,docs/decision/records/2026-09-23-extract-original-lines.md#A3,docs/decision/records/2026-09-23-extract-original-lines.md#A7
Scenario: フィールド行の値は名前の後から始まり継続段落を空行で付ける
  Given 2行にわたる値と空行を挟んだ`継続段落`を持つ`フィールド行`と、区切り文字を宣言した同じ形の`フィールド行`の`文書`がある
  When "kotowari-mds values --format json" を実行する
  Then 値は名前の後の空白の後から始まり、2行目の字下げを残す
  And `継続段落`は空行を挟んで値か区切った末尾の要素に付く

@id=EX-schema-056 @about=REQ-schema-063,TBL-schema-008 @source=docs/decision/records/2026-09-23-extract-original-lines.md#A3,docs/decision/records/2026-09-21-mds-spec.md#A51
Scenario: 節の本文は含めない部分を抜いて空行を1つにまとめる
  Given `文`、空行、`フィールド行`、空行、`箇条書き`の順に並び、`文`と`箇条書き`の間に空行を挟まない別の並びも持つ`節`の`文書`がある
  When "kotowari-mds values --format json" を実行する
  Then `フィールド行`を抜いた跡の空行は1つにまとまる
  And 空行を挟まない`文`と`箇条書き`は改行1つでつながる

@id=EX-schema-057 @about=REQ-schema-063 @source=docs/decision/records/2026-09-23-extract-original-lines.md#A6
Scenario: 行で読んでも文の行の間に空行を足さない
  Given "reading: line" を宣言し、`文`の`抽出`に`導かれる値`を宣言しない`スキーマ`がある
  And 空行を挟まない2行の`文`と、空行を挟んだ3行目の`文`を持つ`文書`がある
  When "kotowari-mds values --format json" を実行する
  Then 1行目と2行目は改行1つで、2行目と3行目は空行1つでつながる

@id=EX-schema-058 @about=REQ-schema-064 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A4
Scenario: 見出しの名前とセルの値はインラインの記法の記号を外した文字になる
  Given 名前に太字、リンク、参照リンク、インラインコード、強調を含む`項目`の見出しと、セルに太字、参照リンク、インラインコードを含む`表`を持つ`文書`がある
  And `項目`に見出しの名前の`導かれる値`を、`表`に`抽出`を宣言した`スキーマ`がある
  When "kotowari-mds values --format json" を実行する
  Then 見出しの名前とセルの値は、記号を外して中の文字をつなげた文字になり、インラインコードは中身を残す

@id=EX-schema-059 @about=REQ-schema-064 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A4,docs/decision/records/2026-09-21-mds-spec.md#A2
Scenario: 記号を外した文字が宣言した名前と違う見出しは指摘になる
  Given `節`の名前を "Req" と宣言した`スキーマ`と、"## **Req** x" の見出しを持つ`文書`がある
  When "kotowari-mds check" を実行する
  Then その見出しは "Req x" と読まれ、宣言していない見出しとして`指摘`になる

@id=EX-schema-060 @about=TBL-schema-008,REQ-schema-035 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A5,docs/decision/records/2026-09-23-mutants-gaps.md#A10
Scenario: 項目の本文は宣言したフィールド行の子の行を含めない
  Given 子の`箇条書き`を持つ宣言した`フィールド行`を持つ`項目`の`文書`と、`項目`に略記の`抽出`を宣言した`スキーマ`がある
  When "kotowari-mds values --format json" を実行する
  Then `項目`の本文はその`フィールド行`の行を含み、子の`箇条書き`の行を含まない

@id=EX-schema-061 @about=TBL-schema-008,REQ-schema-035 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A5,docs/decision/records/2026-09-23-mutants-gaps.md#A10
Scenario: 項目の本文は宣言していない行の子の行を含める
  Given 子の`箇条書き`を持つ、`スキーマ`に宣言していない名前の一覧の行を持つ`項目`の`文書`と、`項目`に略記の`抽出`を宣言した`スキーマ`がある
  When "kotowari-mds values --format json" を実行する
  Then `項目`の本文はその行と子の`箇条書き`の行を含む
```
