# 後回し

[English](deferred.md) | 日本語

`要求`を`後回し`にする宣言の書き方と読み方、`後回し`と`テスト`の`印`や参照の食い違いで出す`注意`、`後回し`の扱いをスキルに書くことを扱う。テストの無さの検査から外すところは coverage.md が、数え方は status.md と list.md が扱う。

## Requirements

### REQ-core-208: 後回しの宣言

- kind: ubiquitous
- source: docs/decision/records/2026-09-25-deferred-items.md#A1, docs/decision/records/2026-09-25-deferred-items.md#A10, docs/decision/records/2026-09-25-deferred-items.md#A18, docs/decision/records/2026-09-25-deferred-items.md#A24, docs/decision/records/2026-09-25-deferred-items.md#A25
- verification: unit

kotowari は常に、見出しの下に "- deferred:" の行を持つ`要求`と、`題名`の後で最初の "## " か "### " より前に "- deferred:" の行（文書単位の宣言）を持つ`話題ごとの文書`のすべての`要求`を`後回し`として扱う。同じ`ID`の`要求`が2か所以上にあるときは、REQ-core-032 の1つ目の`要求`で`後回し`かどうかを決める。"- deferred:" の行の値が空でも`出典`として誤りでも`後回し`として扱い、両方の行があっても`指摘`を出さない。

### REQ-core-209: 文書単位の宣言の行

- kind: ubiquitous
- source: docs/decision/records/2026-09-25-deferred-items.md#A1, docs/decision/records/2026-09-25-deferred-items.md#A10, docs/decision/records/2026-09-25-deferred-items.md#A19, docs/decision/records/2026-09-25-deferred-items.md#A24, docs/decision/records/2026-09-25-deferred-items.md#A26
- verification: unit

kotowari は常に、`話題ごとの文書`の`題名`の後で最初の "## " か "### " より前にある "- deferred:" の行を文書単位の宣言として読み、`文書が扱う範囲`の行に数えない。文書単位の "- deferred:" の行が2つ以上あれば2つ目の行に duplicate_field の`誤り`を1件出し、1つ目の行の値だけを読む。`要求`の無い文書の文書単位の宣言にも、要求が無いことを理由とする`指摘`は出さず、出典の検査（REQ-core-210）と duplicate_field は`要求`のある文書と同じに行う。`用語集`と`問題の記録`の文書の同じ位置の "- deferred:" の行には、今までどおり unknown_field の`誤り`を出す。

### REQ-core-210: 後回しの出典

- kind: ubiquitous
- source: docs/decision/records/2026-09-25-deferred-items.md#A2, docs/decision/records/2026-09-25-deferred-items.md#A23
- verification: unit

kotowari は常に、"- deferred:" の行の値をコンマで区切った`出典`の並びとして読み、"- source:" の行と同じ規則（REQ-core-057、REQ-core-058）で検査する。値が空の "- deferred:" の行には、その行を "line" にし detail を "deferred" にした missing_source の`誤り`を出す。

### REQ-core-211: 後回しなのにテストの印がある

- kind: event_driven
- source: docs/decision/records/2026-09-25-deferred-items.md#A11, docs/decision/records/2026-09-25-deferred-items.md#A15, docs/decision/records/2026-09-25-deferred-items.md#A23, docs/decision/records/2026-09-25-deferred-items.md#A24
- verification: unit

`後回し`の`要求`の`ID`か`後回しのシナリオ`の`ID`を含む`印`が1つ以上あるとき、kotowari は "line" をその`要求`の見出しの行（`シナリオ`はタグの行。同じ`ID`が2か所以上にあるときは REQ-core-032 の1つ目のもの）にし detail をその`ID`にした deferred_with_test の`注意`を、その`ID`につき1件出す。`問い合わせの無い言語`の`テストのファイル`から拾った`印`も数える。

### REQ-core-212: 後回しへの依存

- kind: event_driven
- source: docs/decision/records/2026-09-25-deferred-items.md#A12, docs/decision/records/2026-09-25-deferred-items.md#A15, docs/decision/records/2026-09-25-deferred-items.md#A17, docs/decision/records/2026-09-25-deferred-items.md#A21, docs/decision/records/2026-09-25-deferred-items.md#A22, docs/decision/records/2026-09-25-deferred-items.md#A23
- verification: unit

`後回し`でない`要求`か`性質`が "- definition:" の行か`文`の中のバッククォートで囲んだ`ID`で`後回し`の`要求`を指すとき、および`後回しのシナリオ`でない`シナリオ`が "@about" のタグかステップの中のバッククォートで囲んだ`ID`で`後回し`の`要求`を指すとき、kotowari は参照1件ごとに、"line" を参照の書かれた行にし detail を「参照元の`ID`と参照先の`ID`を1つの半角空白で区切った文字列」にした depends_on_deferred の`注意`を出す。`後回し`の`要求`と`後回しのシナリオ`からの参照と、`後回しのシナリオ`への参照には出さない。バッククォートで囲んだ`ID`の判定は REQ-core-054 と同じである。

### REQ-core-213: スキルの後回しの扱い

- kind: ubiquitous
- source: docs/decision/records/2026-09-25-deferred-items.md#A13, docs/decision/records/2026-09-25-deferred-items.md#A23
- verification: review
- how_to_verify: "agent/skills/" の下のスキルを読み、kotowari スキルの references の ir-form.md に "- deferred:" の行（要求ごとと文書単位）が、findings.md に deferred_with_test と depends_on_deferred が載っていること、kotowari-plan が`後回し`の`要求`の`ID`を計画に含めないと書いていること、kotowari-cycle と kotowari-review がテストの側の`指摘`を0件にする対象に`後回し`を含めないと書いていること、kotowari-brainstorm が`後回し`にするときに理由の決定を`判断の記録`に書いてそれを出典にすると書いていることを確かめる

"agent/skills/" の下の kotowari のスキルは常に、`後回し`の書き方と、計画、実装のループ、レビュー、壁打ちでの`後回し`の扱いを書いている。

## Examples

```gherkin
@id=EX-core-384 @about=REQ-core-208,REQ-core-210 @source=docs/decision/records/2026-09-25-deferred-items.md#A1,docs/decision/records/2026-09-25-deferred-items.md#A2,docs/decision/records/2026-09-25-deferred-items.md#A4
Scenario: 要求ごとの宣言でテストの無さが消える
  Given "docs/ir/a.md" に検証が "unit" の要求 "REQ-001" があり、その見出しの下に "- deferred: docs/decision/records/r.md#A1" の行があり、"r.md" に決定 "A1" があり、"REQ-001" を含む印はどのテストにも無い
  When "kotowari check" を実行する
  Then "REQ-001" を detail にする requirement_without_test は出ず、source_invalid も出ない

@id=EX-core-385 @about=REQ-core-208,REQ-core-209 @source=docs/decision/records/2026-09-25-deferred-items.md#A1,docs/decision/records/2026-09-25-deferred-items.md#A3,docs/decision/records/2026-09-25-deferred-items.md#A19,docs/decision/records/2026-09-25-deferred-items.md#A4
Scenario: 文書単位の宣言は文書のすべての要求を後回しにする
  Given "docs/ir/a.md" の題名の後に範囲の行と "- deferred: docs/decision/records/r.md#A1" の行があり、検証が "unit" の要求 "REQ-001" と "REQ-002" と "@id=EX-001 @about=REQ-001" のシナリオがあり、どれの ID を含む印も無い
  When "kotowari check" を実行する
  Then requirement_without_test も scenario_without_test も unknown_field も missing_scope も出ない

@id=EX-core-386 @about=REQ-core-209 @source=docs/decision/records/2026-09-25-deferred-items.md#A19
Scenario: 宣言の行だけでは範囲にならない
  Given "docs/ir/a.md" の題名と最初の "## " の間に "- deferred: docs/decision/records/r.md#A1" の行だけがある
  When "kotowari check" を実行する
  Then missing_scope の誤りが出る

@id=EX-core-387 @about=REQ-core-209 @source=docs/decision/records/2026-09-25-deferred-items.md#A19,docs/decision/records/2026-09-25-deferred-items.md#A26
Scenario: 文書単位の宣言の重なりは誤り
  Given "docs/ir/a.md" の範囲の行の間に "- deferred:" の行が2つある
  When "kotowari check" を実行する
  Then 2つ目の行に detail が "deferred" の duplicate_field の誤りが1件出る

@id=EX-core-388 @about=REQ-core-209 @source=docs/decision/records/2026-09-25-deferred-items.md#A10
Scenario: 用語集の宣言は知らない行
  Given "docs/ir/CONTEXT.md" の題名の後に "- deferred: docs/decision/records/r.md#A1" の行がある
  When "kotowari check" を実行する
  Then その行に unknown_field の誤りが出る

@id=EX-core-389 @about=REQ-core-210,REQ-core-208 @source=docs/decision/records/2026-09-25-deferred-items.md#A2,docs/decision/records/2026-09-25-deferred-items.md#A18,docs/decision/records/2026-09-25-deferred-items.md#A23,docs/decision/records/2026-09-25-deferred-items.md#A4
Scenario: 値の空の宣言は出典の誤りで、要求は後回しのまま
  Given 検証が "unit" の要求 "REQ-001" の見出しの下に値の空の "- deferred:" の行があり、"REQ-001" を含む印は無い
  When "kotowari check" を実行する
  Then その行に detail が "deferred" の missing_source の誤りが出て、requirement_without_test は出ない

@id=EX-core-390 @about=REQ-core-210 @source=docs/decision/records/2026-09-25-deferred-items.md#A2,docs/decision/records/2026-09-25-deferred-items.md#A24
Scenario: 宣言の出典の先が無ければ誤り
  Given 要求 "REQ-001" の見出しの下に "- deferred: docs/decision/records/r.md#A9" の行があり、"r.md" に決定 "A9" が無い
  When "kotowari check" を実行する
  Then その行に source_invalid の誤りが出る

@id=EX-core-391 @about=REQ-core-208 @source=docs/decision/records/2026-09-25-deferred-items.md#A1,docs/decision/records/2026-09-25-deferred-items.md#A18,docs/decision/records/2026-09-25-deferred-items.md#A25
Scenario: 形の誤りと ID の重なりは後回しでも誤り
  Given 文書単位の宣言を持つ "docs/ir/a.md" に "- verification:" の行の無い要求 "REQ-001" があり、"docs/ir/b.md" にも "REQ-001" がある
  When "kotowari check" を実行する
  Then verification_missing と duplicate_id の誤りが出る

@id=EX-core-392 @about=REQ-core-211 @source=docs/decision/records/2026-09-25-deferred-items.md#A11
Scenario: 後回しの要求を指す印は注意
  Given 後回しの要求 "REQ-001" と、"@kotowari[REQ-001]" の印を持つテストがある
  When "kotowari check" を実行する
  Then "REQ-001" の見出しの行に detail が "REQ-001" の deferred_with_test の注意が1件出る

@id=EX-core-393 @about=REQ-core-211 @source=docs/decision/records/2026-09-25-deferred-items.md#A11,docs/decision/records/2026-09-25-deferred-items.md#A15
Scenario: 後回しのシナリオを指す印は注意
  Given 後回しの要求 "REQ-001" と "@id=EX-001 @about=REQ-001" のシナリオがあり、"@kotowari[EX-001]" の印を持つテストがある
  When "kotowari check" を実行する
  Then そのタグの行に detail が "EX-001" の deferred_with_test の注意が1件出る

@id=EX-core-394 @about=REQ-core-212 @source=docs/decision/records/2026-09-25-deferred-items.md#A12,docs/decision/records/2026-09-25-deferred-items.md#A21
Scenario: 後回しでない要求の文が後回しの要求を指すと注意
  Given 後回しの要求 "REQ-001" と、`文`の中でバッククォートで囲んだ "REQ-001" を指す後回しでない要求 "REQ-002" がある
  When "kotowari check" を実行する
  Then その`文`の行に detail が "REQ-002 REQ-001" の depends_on_deferred の注意が1件出る

@id=EX-core-395 @about=REQ-core-212 @source=docs/decision/records/2026-09-25-deferred-items.md#A12,docs/decision/records/2026-09-25-deferred-items.md#A15,docs/decision/records/2026-09-25-deferred-items.md#A21,docs/decision/records/2026-09-17-scenario-tests.md#A2,docs/decision/records/2026-09-17-scenario-tests.md#A6
Scenario: 後回しと後回しでない要求を両方指すシナリオは注意とテストの誤り
  Given 後回しの要求 "REQ-001" と検証が "unit" の後回しでない要求 "REQ-002" と "@id=EX-001 @about=REQ-001,REQ-002" のシナリオがあり、"EX-001" を含む印は無い
  When "kotowari check" を実行する
  Then そのタグの行に detail が "EX-001 REQ-001" の depends_on_deferred の注意が出て、detail が "EX-001" の scenario_without_test の誤りも出る

@id=EX-core-396 @about=REQ-core-212 @source=docs/decision/records/2026-09-25-deferred-items.md#A12,docs/decision/records/2026-09-25-deferred-items.md#A22
Scenario: 後回しの側からの参照には出さない
  Given 後回しの要求 "REQ-001" の`文`がバッククォートで囲んだ後回しでない要求 "REQ-002" を指し、後回しの要求 "REQ-003" の`文`がバッククォートで囲んだ "REQ-001" を指す
  When "kotowari check" を実行する
  Then depends_on_deferred の注意は出ない

@id=EX-core-397 @about=REQ-core-212 @source=docs/decision/records/2026-09-25-deferred-items.md#A17,docs/decision/records/2026-09-25-deferred-items.md#A21,docs/decision/records/2026-09-25-deferred-items.md#A12,docs/decision/records/2026-09-25-deferred-items.md#A23
Scenario: 後回しでないシナリオのステップの参照も注意
  Given 後回しの要求 "REQ-001" と、"@about=REQ-002" の後回しでないシナリオ "EX-002" があり、そのステップの1行がバッククォートで囲んだ "REQ-001" を2つ含む
  When "kotowari check" を実行する
  Then そのステップの行に detail が "EX-002 REQ-001" の depends_on_deferred の注意が2件出る
```
