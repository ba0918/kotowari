# 部品

view が描ける`部品`の種類、`部品のスキーマ`の公開、部品の幅を扱う。`部品`の中身の形の正は`部品のスキーマ`で、view は形を検査しない。

## Requirements

### REQ-view-011: 部品の種類

- kind: algorithm
- source: docs/decision/records/2026-10-02-whole-picture.md#A13, docs/decision/records/2026-10-02-whole-picture.md#A36
- definition: TBL-view-001
- verification: unit

### REQ-view-012: 部品のスキーマを公開する

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A45, docs/decision/records/2026-10-02-whole-picture.md#A46, docs/decision/records/2026-10-02-whole-picture.md#A47, docs/decision/records/2026-10-02-whole-picture.md#A49, docs/decision/records/2026-10-02-whole-picture.md#A82
- verification: unit

view は常に、TBL-view-001 の種類の名前のそれぞれから、その種類の`部品のスキーマ`（JSON Schema の文字列）を返す公開の関数を持ち、`部品のスキーマ`を view の中に埋め込む。TBL-view-001 に無い名前には何も返さない。

### REQ-view-013: 部品のスキーマは閉じている

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A46
- verification: unit

このリポジトリのテストは常に、すべての`部品のスキーマ`の中の、型が object の部分スキーマが "additionalProperties": false を宣言していることを確かめる。

### REQ-view-014: スキーマに合う部品は描ける

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A45
- verification: unit

このリポジトリのテストは常に、TBL-view-001 の種類ごとに、`部品のスキーマ`に合う`部品`の例を1つ以上持ち、すべての例が`部品のスキーマ`に合うことと、view がそれを描いて失敗しないことを確かめる。

### REQ-view-015: 部品の幅

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A12, docs/decision/records/2026-10-02-whole-picture.md#A54, docs/decision/records/2026-10-02-whole-picture.md#A73
- verification: unit

view は常に、同じ`節`の中で続く、値に "width": "half" を持つ`部品`の並びを、先頭から2つずつ組にして左右に並べ、組にならずに残った1つと、それ以外の`部品`を`節`の幅いっぱいに描く。どの`部品のスキーマ`も "width" の値として "half" だけを許す。

### REQ-view-024: flow の列と並列

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-overview-page-reading.md#A8, docs/decision/records/2026-10-05-overview-page-reading.md#D2
- verification: review
- how_to_verify: 列を2つ以上持ち、1つの列に箱を2つ以上縦に並べた flow の`部品`を描いてブラウザで開き、1つの列の箱が1つの囲みの中にあり、矢印が列と列の間にだけあって同じ列の箱の間に無いことを人が見て確かめる

view は常に、flow の`部品`の列ごとにその列の箱を1つの囲みの中に描き、左から右への流れを示す矢印を列と列の間にだけ描いて、同じ列の箱の間には描かない。

### REQ-view-025: 狭い画面の flow

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-overview-page-reading.md#A9, docs/decision/records/2026-10-05-overview-page-reading.md#A15, docs/decision/records/2026-10-05-overview-page-reading.md#D2
- verification: review
- how_to_verify: 列を2つ以上持つ flow の`部品`を描いてブラウザで開き、狭い幅で列が上から下に積まれ、列と列の間の矢印が下を向き、箱が重ならないことを人が見て確かめる

view は常に、狭い画面では flow の`部品`の列を上から下に積み、列と列の間の矢印を下向きに描いて、箱を重ねない。

## Decision tables

### TBL-view-001: 部品の種類

- source: docs/decision/records/2026-10-02-whole-picture.md#A36, docs/decision/records/2026-10-02-whole-picture.md#A66, docs/decision/records/2026-10-02-whole-picture.md#A31, docs/decision/records/2026-10-02-whole-picture.md#A83, docs/decision/records/2026-10-02-whole-picture.md#A7, docs/decision/records/2026-10-02-whole-picture.md#A38, docs/decision/records/2026-10-02-whole-picture.md#A85, docs/decision/records/2026-10-04-overview-on-public-api.md#A13, docs/decision/records/2026-10-05-localization.md#A3, docs/decision/records/2026-10-05-localization.md#A5, docs/decision/records/2026-10-05-localization.md#D1

| 種類の名前 | 描くもの | 主な欄 |
|---|---|---|
| lead | 結論と要点。`文書`の冒頭に置く | conclusion、points |
| flow | 左から右へ流れる列の箱の並び。箱は格子に並べ、文字の位置を手で決めない | columns（列の並び。列は箱の並び。箱は title、body、tone） |
| steps | 番号付きの段階の並び | items（title、body、refs） |
| cards | 見出しと項目の一覧を持つカードの並び | cards（title、items、tone） |
| status | 状態の札と文の並び。札は "decided"、"planned"、"open"、"dropped" で、それぞれ`UI の文字`の "state_decided"、"state_planned"、"state_open"、"state_dropped" の文字で、4つを互いに見分けられる見た目で描く | items（state、text、refs） |
| compare | 前と後と理由の組の並び。前は取り消し線で描き、列の見出しは`UI の文字`の "compare_before"、"compare_after"、"compare_why" の文字にする | items（before、after、why、refs） |
| decisions | 根の判断とその下の判断の木。判断した者の札を付ける | roots（ref、text、by、children） |
| quiz | 問いと、選ぶと開く答えの並び | items（q、a、refs） |

## Examples

```gherkin
@id=EX-view-007 @about=REQ-view-012,REQ-view-011 @source=docs/decision/records/2026-10-02-whole-picture.md#A47,docs/decision/records/2026-10-02-whole-picture.md#A36,docs/decision/records/2026-10-02-whole-picture.md#A46,docs/decision/records/2026-10-02-whole-picture.md#A82
Scenario: 8種のスキーマを返し、知らない名前には返さない
  When view の部品のスキーマを返す関数を TBL-view-001 の8つの名前と "chart" で呼ぶ
  Then 8つの名前には JSON Schema の文字列が返り、"chart" には何も返らない

@id=EX-view-008 @about=REQ-view-015 @source=docs/decision/records/2026-10-02-whole-picture.md#A12,docs/decision/records/2026-10-02-whole-picture.md#A54
Scenario: 続く2つの半分の部品は左右に並ぶ
  Given 1つの節に、"width": "half" の cards の部品が2つ続き、その後に width の無い status の部品がある
  When view で描画する
  Then 2つの cards は1つの横並びの中に描かれ、status はその外に節の幅いっぱいに描かれる

@id=EX-view-009 @about=REQ-view-013,REQ-view-014 @source=docs/decision/records/2026-10-02-whole-picture.md#A45,docs/decision/records/2026-10-02-whole-picture.md#A46
Scenario: スキーマの閉じ方と例の描画をテストが確かめる
  Given 部品のスキーマの1つに "additionalProperties" の無い object の部分スキーマがある
  When このリポジトリのテストを実行する
  Then そのテストが失敗する
```
