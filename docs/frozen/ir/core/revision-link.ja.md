# superseded_by のリンクの検査

[English](revision-link.md) | 日本語

判断の記録の superseded_by の補足の行にあるリンクの形と、先の実在の検査を扱う。適用の範囲（"## Context" の有無を見ずすべての判断の記録に適用する）は record-form.md の REQ-core-129 が決め、補足の行の有無と名前の検査も record-form.md が扱う。

## Requirements

### REQ-core-132: superseded_by のリンク

- kind: algorithm
- source: docs/decision/records/2026-09-17-record-form.md#A4, docs/decision/records/2026-09-17-record-form.md#A6, docs/decision/records/2026-09-17-record-form.md#A7, docs/decision/records/2026-09-17-record-form.md#A18, docs/decision/records/2026-09-17-record-form.md#A19, docs/decision/records/2026-09-17-record-form.md#A20, docs/decision/records/2026-09-17-record-form.md#A21, docs/decision/records/2026-09-17-record-form.md#A22, docs/decision/records/2026-09-17-record-form.md#A25, docs/decision/records/2026-09-17-record-form.md#A27, docs/decision/records/2026-09-17-record-form.md#A29, docs/decision/records/2026-09-17-record-form.md#A32, docs/decision/records/2026-09-17-record-form.md#A36, docs/decision/records/2026-09-17-record-form.md#A37, docs/decision/records/2026-09-17-record-form.md#A38, docs/decision/records/2026-09-17-decision-log.md#A3, docs/decision/records/2026-09-17-decision-log.md#A10, docs/decision/records/2026-09-17-record-form.md#A33, docs/decision/records/2026-09-17-record-form.md#A42, docs/decision/records/2026-09-17-record-form.md#A43, docs/decision/records/2026-09-17-record-form.md#A41, docs/decision/records/2026-09-17-record-form.md#A44
- definition: TBL-core-023
- verification: unit

## Decision tables

### TBL-core-023: superseded_by のリンクの判定

- source: docs/decision/records/2026-09-17-record-form.md#A4, docs/decision/records/2026-09-17-record-form.md#A6, docs/decision/records/2026-09-17-record-form.md#A7, docs/decision/records/2026-09-17-record-form.md#A18, docs/decision/records/2026-09-17-record-form.md#A19, docs/decision/records/2026-09-17-record-form.md#A20, docs/decision/records/2026-09-17-record-form.md#A21, docs/decision/records/2026-09-17-record-form.md#A25, docs/decision/records/2026-09-17-record-form.md#A27, docs/decision/records/2026-09-17-record-form.md#A29, docs/decision/records/2026-09-17-record-form.md#A32, docs/decision/records/2026-09-17-record-form.md#A36, docs/decision/records/2026-09-17-record-form.md#A37, docs/decision/records/2026-09-17-record-form.md#A38, docs/decision/records/2026-09-17-record-form.md#A33, docs/decision/records/2026-09-17-record-form.md#A42, docs/decision/records/2026-09-17-record-form.md#A43, docs/decision/records/2026-09-17-record-form.md#A41, docs/decision/records/2026-09-17-record-form.md#A44, docs/decision/records/2026-09-19-mutants-followup.md#A3, docs/decision/records/2026-09-19-mutants-followup.md#A8

名前が "superseded_by" で値が空でない`補足の行`の値にある "[文字](href)" の形をリンクとする。リンクは、値の "[" から最初の "]" まで、その直後が "(" のときは最初の ")" までとし、"(" と ")" の間を href とする。文字は空でもよい。"]" が無いとき、"]" の直後が "(" でないとき、その "(" に対応する ")" が無いときは、その "[" はこの形に当たらず読み飛ばし、走査はその "[" の次の文字から続ける。リンクとして読めたときは、走査はその ")" の次の文字から続け、リンクの中の "[" から別のリンクを読み始めない。判定はリンクごとに行い、無効なリンク1つにつき、"line" をその`補足の行`にして revision_link_invalid の`誤り`を1件出す。同じ行に同じ href が複数あっても出現ごとに1件出す。detail は href（順1では行の値。値は REQ-core-133 のとおり前後の空白を除いたもので、行の文字そのままではない）。順3の解決は、href の最初の "#" より前を記録のファイルのディレクトリに字面でつなぎ、REQ-core-110 の正規化をかけた後に、".." を左から順に直前の要素を消して解く。消す要素が無ければ置き場の外とする。

| 順 | 条件 | 結果 |
|---|---|---|
| 1 | 値にリンクが1つも無い | revision_link_invalid |
| 2 | href に "#" が無い、または最初の "#" の後が`決定の番号`の形でない | revision_link_invalid |
| 3 | 最初の "#" より前が "/" か "\\" で始まる、または最初の "#" より前が空でなく解決した結果が decisions.records の中でない | revision_link_invalid |
| 4 | 最初の "#" より前が空ならその記録、空でなければ 3 の結果のファイルを先とし、先が読んだ`判断の記録`でない | revision_link_invalid |
| 5 | 先の`決定の節`と`Superseded の節`のどこにも、その番号の`番号の行`が無い | revision_link_invalid |
| 6 | 1 から 5 のどれにも当たらない | 正しい |

## Examples

```gherkin
@id=EX-core-106 @about=REQ-core-132 @source=docs/decision/records/2026-09-17-record-form.md#A4,docs/decision/records/2026-09-17-record-form.md#A18,docs/decision/records/2026-09-17-record-form.md#A20,docs/decision/records/2026-09-17-record-form.md#A21,docs/decision/records/2026-09-17-record-form.md#A22,docs/decision/records/2026-09-17-record-form.md#A27
Scenario: 古い記録の Superseded の行のリンク2つがどちらも解決されて通る
  Given "## Context" の見出しを持たない判断の記録 "docs/decision/records/records.md" の Superseded の節の "- A54 " の行の下に、"./2026-09-16-ir-tree.md#A21" と "./2026-09-16-ir-tree.md#A5" の2つのリンクを持つ "- superseded_by:" の行があり、"docs/decision/records/2026-09-16-ir-tree.md" の Agreements の節に "- A21 " と "- A5 " で始まる行がある
  When "kotowari check" を実行する
  Then その行を指す revision_link_invalid の誤りは出ない

@id=EX-core-107 @about=REQ-core-132 @source=docs/decision/records/2026-09-17-record-form.md#A7,docs/decision/records/2026-09-17-record-form.md#A21,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A32,docs/decision/records/2026-09-17-record-form.md#A33
Scenario: リンクの無い superseded_by は誤りになる
  Given 判断の記録の Agreements の節の "- A1 " の行の下に "- superseded_by: A24" の行がある
  When "kotowari check" を実行する
  Then "line" がその行で detail が "A24" の revision_link_invalid の誤りが出る

@id=EX-core-108 @about=REQ-core-132 @source=docs/decision/records/2026-09-17-record-form.md#A18,docs/decision/records/2026-09-17-record-form.md#A25,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A32,docs/decision/records/2026-09-17-record-form.md#A36
Scenario: 置き場の外を指す superseded_by は誤りになる
  Given "decisions.records" が "docs/decision/records" で、判断の記録 "docs/decision/records/x.md" の "- A1 " の行の下に "- superseded_by: [A1](../../ir/example.md#A1)" の行がある
  When "kotowari check" を実行する
  Then detail が "../../ir/example.md#A1" の revision_link_invalid の誤りが出る

@id=EX-core-109 @about=REQ-core-132 @source=docs/decision/records/2026-09-17-record-form.md#A20,docs/decision/records/2026-09-17-record-form.md#A25,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A32
Scenario: Undecided にしか無い番号を指す superseded_by は誤りになる
  Given 判断の記録 "docs/decision/records/a.md" の "- A1 " の行の下に "- superseded_by: [U1](./b.md#U1)" の行があり、"docs/decision/records/b.md" は Agreements の節を持ち、"- U1 " で始まる行が Undecided の節にだけある
  When "kotowari check" を実行する
  Then detail が "./b.md#U1" の revision_link_invalid の誤りが出る

@id=EX-core-112 @about=REQ-core-132 @source=docs/decision/records/2026-09-17-record-form.md#A19,docs/decision/records/2026-09-17-record-form.md#A25,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A32
Scenario: 見出しを指す superseded_by は誤りになる
  Given "decisions.records" が "docs/decision/records" で、判断の記録 "docs/decision/records/a.md" の "- A1 " の行の下に "- superseded_by: [出典](./ir-form.md#出典)" の行があり、"docs/decision/records/ir-form.md" に "## 出典" の見出しがある
  When "kotowari check" を実行する
  Then detail が "./ir-form.md#出典" の revision_link_invalid の誤りが出る

@id=EX-core-116 @about=REQ-core-132 @source=docs/decision/records/2026-09-17-record-form.md#A18,docs/decision/records/2026-09-17-record-form.md#A20,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A36
Scenario: 下位ディレクトリの記録から上の記録の Superseded の番号を指すリンクは通る
  Given "decisions.records" が "docs/decision/records" で、判断の記録 "docs/decision/records/sub/a.md" の "- A1 " の行の下に "- superseded_by: [A15](../records.md#A15)" の行があり、"docs/decision/records/records.md" の Superseded の節に "- A15 " で始まる行がある
  When "kotowari check" を実行する
  Then その行を指す revision_link_invalid の誤りは出ない

@id=EX-core-117 @about=REQ-core-132 @source=docs/decision/records/2026-09-17-record-form.md#A20,docs/decision/records/2026-09-17-record-form.md#A25,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A29,docs/decision/records/2026-09-17-record-form.md#A32,docs/decision/records/2026-09-17-record-form.md#A15
Scenario: パスの無いリンクは同じ記録を先にする
  Given 判断の記録 "docs/decision/records/a.md" の Agreements の節に "- A1 " と "- A2 " の行があり、どの節にも "- A9" の番号の行が無く、"- A1 " の行の下に "- superseded_by: [A2](#A2)" と "- superseded_by: [A9](#A9)" の行がある
  When "kotowari check" を実行する
  Then "[A2](#A2)" の行を指す誤りは出ず、detail が "#A9" の revision_link_invalid の誤りが出る

@id=EX-core-119 @about=REQ-core-132 @source=docs/decision/records/2026-09-17-record-form.md#A4,docs/decision/records/2026-09-17-record-form.md#A25,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A32,docs/decision/records/2026-09-17-record-form.md#A41
Scenario: 判断の記録でないファイルを指す superseded_by は誤りになる
  Given "decisions.records" が "docs/decision/records" で、判断の記録 "docs/decision/records/a.md" の "- A1 " の行の下に "- superseded_by: [A1](./ir-form.md#A1)" の行があり、"docs/decision/records/ir-form.md" は決定の節の見出しを持たない
  When "kotowari check" を実行する
  Then detail が "./ir-form.md#A1" の revision_link_invalid の誤りが出る
```
