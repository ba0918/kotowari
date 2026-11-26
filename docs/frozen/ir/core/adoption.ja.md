# 既存プロジェクトへの導入

[English](adoption.md) | 日本語

既存のプロジェクトに kotowari を途中から入れるための工程のスキル kotowari-adopt が、1つの話題について`旧資料`と実装とテストを突き合わせて仕分け、`IR`と`判断の記録`と`問題の記録`に書くまでの手順を扱う。スキルの文面を人か LLM が読んで確かめる要求だけを持ち、kotowari 本体の振る舞いは変えない。

## Requirements

### REQ-core-214: 導入のスキルと入口

- kind: ubiquitous
- source: docs/decision/records/2026-09-26-adoption.md#A1, docs/decision/records/2026-09-26-adoption.md#A12
- verification: review
- how_to_verify: "agent/skills/kotowari-adopt/" があり、承認を kotowari-brainstorm の承認の手順を名指しして行うと書いていること、kotowari-using-workflow の入口に、既存の仕様やコードを IR に取り込みたい依頼と IR が実装と合っているか分からない依頼を kotowari-adopt に振る行があることを確かめる

"agent/skills/" の下の kotowari のスキルは常に、導入を工程のスキル kotowari-adopt として持ち、その承認を kotowari-brainstorm の承認の手順で行う。kotowari-using-workflow は、既存の仕様やコードを IR に取り込みたい依頼と、IR が実装と合っているか分からない依頼を kotowari-adopt に振る。

### REQ-core-216: 振る舞いと用語の仕分け

- kind: algorithm
- source: docs/decision/records/2026-09-26-adoption.md#A4, docs/decision/records/2026-09-26-adoption.md#A5, docs/decision/records/2026-09-26-adoption.md#A8, docs/decision/records/2026-09-26-adoption.md#A10, docs/decision/records/2026-09-26-adoption.md#A20, docs/decision/records/2026-09-26-adoption.md#A21, docs/decision/records/2026-09-26-adoption.md#A26, docs/decision/records/2026-09-26-adoption.md#A30, docs/decision/records/2026-09-26-adoption.md#A31, docs/decision/records/2026-09-26-adoption.md#A32, docs/decision/records/2026-09-26-adoption.md#A35, docs/decision/records/2026-09-26-adoption.md#A37, docs/decision/records/2026-09-26-adoption.md#A40
- verification: review
- definition: TBL-core-037
- how_to_verify: "agent/skills/kotowari-adopt/" を読み、TBL-core-037 の行がすべて同じ行き先で書いてあることを確かめる

### REQ-core-217: 導入の判断の記録の書き方

- kind: ubiquitous
- source: docs/decision/records/2026-09-26-adoption.md#A9, docs/decision/records/2026-09-26-adoption.md#A21, docs/decision/records/2026-09-26-adoption.md#A24, docs/decision/records/2026-09-26-adoption.md#A26, docs/decision/records/2026-09-26-adoption.md#A31, docs/decision/records/2026-09-26-adoption.md#A36
- verification: review
- how_to_verify: "agent/skills/kotowari-adopt/" を読み、次のすべてが書いてあることを確かめる。1回の導入で`判断の記録`を1つ作り、名前を "YYYY-MM-DD-adopt-<話題>.md" にすること。追認する振る舞いと用語は1行（1語）に1つの決定にし、"decided_by" を「利用者（現状追認の一覧を承認）」とし、`要求`や用語はその行の決定を`出典`にすること。FLAG は kind を問わず1つに1つの「この件は決めずに FLAG に残す」決定を書いてその FLAG の`出典`にし、`旧資料`の場所を決定の本文かリンクで示すこと。`後回し`にする決定と、`旧資料`にあったが作らない Rejected も、同じ`判断の記録`に1行に1つずつ書くこと

kotowari-adopt は常に、追認する振る舞いと用語、FLAG に残す件、`後回し`にする件、作らない件を、1回の導入に1つの`判断の記録`に1件1行の決定として書き、`要求`と用語と FLAG はその行の決定を`出典`にする。

### REQ-core-218: テストの仕分け

- kind: ubiquitous
- source: docs/decision/records/2026-09-26-adopt-verification.md#A2, docs/decision/records/2026-09-26-adoption.md#A29, docs/decision/records/2026-09-26-adoption.md#A34
- verification: review
- how_to_verify: "agent/skills/kotowari-adopt/" を読み、範囲のテストをすべて、`要求`の根拠（`要求`になる行と追認の候補の行を確かめるもの）、FLAG の振る舞いのテスト、実装の細部をなぞるだけ（削除の候補）、残す（IR にしない内部の振る舞いを確かめるものと判断がつかないもの）の4つのどれか1つに振り分けること、その内訳を一覧のラウンドと承認で見せて`判断の記録`の Context に1行で書くこと、kotowari-adopt 自身はテストに`印`を付けず、テストを消さず、移さないことが書いてあることを確かめる

kotowari-adopt は常に、話題の範囲の既存のテストを1件ずつ4つの行き先のどれか1つに振り分けて内訳を示し、テストそのものには手を加えない。

### REQ-core-219: 人に聞く回数

- kind: ubiquitous
- source: docs/decision/records/2026-09-26-adoption.md#A1, docs/decision/records/2026-09-26-adoption.md#A5, docs/decision/records/2026-09-26-adoption.md#A9, docs/decision/records/2026-09-26-adoption.md#A29, docs/decision/records/2026-09-26-adoption.md#A30, docs/decision/records/2026-09-26-adoption.md#A33, docs/decision/records/2026-09-26-adoption.md#A35, docs/decision/records/2026-09-26-adopt-verification.md#A1
- verification: review
- how_to_verify: "agent/skills/kotowari-adopt/" を読み、次のすべてが書いてあることを確かめる。人に聞くのは範囲の確認、一覧の1ラウンド、承認の3回で、設定の準備での問いはこれに数えないこと。一覧の振る舞いの行に`旧資料`の記述、実装の振る舞い、根拠のテストの場所（無ければなし）、`要求`にしたときの検証の値を、用語の行に`旧資料`の定義、実装での使われ方、根拠を並べること。追認の候補は違和感のある行だけを人が外し、迷った行は残すこと。実装の無い`旧資料`の記述と既存の`要求`は作る予定の有無を聞くこと。FLAG にする行とテストの内訳は見せるだけであること。承認は kotowari-brainstorm の承認の手順で1回行うこと

kotowari-adopt は常に、人に聞くのを範囲の確認と一覧の1ラウンドと承認の3回にとどめ、項目ごとの面談をしない。

### REQ-core-220: 承認の後の依頼

- kind: event_driven
- source: docs/decision/records/2026-09-26-adopt-verification.md#A2, docs/decision/records/2026-09-26-adoption.md#A34, docs/decision/records/2026-09-26-adoption.md#A38
- verification: review
- how_to_verify: "agent/skills/kotowari-adopt/" を読み、承認の後に、根拠のテストが`要求`を十分に確かめるかを審査して十分なものだけに`印`を付ける作業、細部をなぞるだけのテストを消すかを人がまとめて決める作業、`印`を付けるテストを設定の "tests.files" の範囲に入れる作業、根拠のテストが無い`要求`のテストを書く作業を1つの依頼文にまとめて人に渡し、工程は kotowari-using-workflow が決めると書いていること、依頼文をファイルに残さないこと、変異テストを回しているプロジェクトでは削除の前後で話題のコードに対して生き残る変異を比べ、増えたら消したテストを戻して残すに入れると依頼文に含めることを確かめる

承認を得たとき、kotowari-adopt は`印`を付ける作業と細部のテストを消すかを決める作業とテストを置き直す作業と根拠のテストが無い`要求`のテストを書く作業を1つの依頼文にして人に渡し、どの工程で回すかは kotowari-using-workflow に任せる。

### REQ-core-221: 導入の案内

- kind: ubiquitous
- source: docs/decision/records/2026-09-26-adopt-verification.md#A2, docs/decision/records/2026-09-26-adoption.md#A27
- verification: review
- how_to_verify: "agent/skills/kotowari-adopt/" を読み、設定の "tests.files" の外のテストは kotowari の対象外で`印`が無くても`誤り`にならないこと、FLAG が残る間は "kotowari status" の complete が false になり、それが回収待ちの一覧として狙いどおりであること、根拠のテストが無い`要求`の requirement_without_test はテストを書くまで残り、push や CI で "kotowari check" を求めるプロジェクトではそれで止まるので、導入と後のテストの作業を同じブランチで進めることが書いてあることを確かめる

kotowari-adopt は常に、`印`の無いテストが`誤り`にならない範囲と、FLAG が残る間 complete が false のままである理由と、テストの無い`要求`が残す`誤り`のために導入と後のテストの作業を同じブランチで進めることを利用者に案内する。

### REQ-core-222: 追認した要求の検証とシナリオ

- kind: ubiquitous
- source: docs/decision/records/2026-09-26-adopt-verification.md#A1, docs/decision/records/2026-09-26-adopt-verification.md#A3
- verification: review
- how_to_verify: "agent/skills/kotowari-adopt/" を読み、追認して書く`要求`の検証の値を、実装の振る舞いは "unit"、入力の全体に成り立つ性質は "property"、コードの外のもの（文書、運用手順）は "review" にして how_to_verify に確かめ方を書くと決めていること、テストが無いことを理由に "review" にしないこと、成功条件と反例の`シナリオ`は必須にせず`旧資料`か根拠のテストに具体的な場面があるときだけ書くことが書いてあることを確かめる

kotowari-adopt は常に、追認して書く`要求`の検証の値を振る舞いの中身で決め、テストの有無では決めず、`シナリオ`は具体的な場面があるときだけ書く。

### REQ-core-235: 未記載の面の一覧を減らす

- kind: ubiquitous
- source: docs/decision/records/2026-09-27-surface-check.md#A4, docs/decision/records/2026-09-27-surface-check.md#A11, docs/decision/records/2026-09-27-surface-check.md#A28, docs/decision/records/2026-09-27-surface-check.md#A15
- verification: review
- how_to_verify: "agent/skills/kotowari-adopt/" を読み、面の規則を最初に書いたときに出る surface_without_spec の面をすべて未記載の面の一覧に載せ、導入の話題ごとに減らすこと、導入で話題の面を要求にしたら未記載の面の一覧からその面の1件を外すこと、FLAG になった面と一覧のラウンドで人が外した面の1件は残すことが書いてあることを確かめる

kotowari-adopt は常に、`面の規則`を最初に書いたときに`IR`に無い`面`をすべて`未記載の面の一覧`に載せ、導入の話題ごとにその話題の`面`を`要求`にして`未記載の面の一覧`から外す。

## Decision tables

### TBL-core-037: 振る舞いと用語の行き先

- source: docs/decision/records/2026-09-26-adoption.md#A4, docs/decision/records/2026-09-26-adoption.md#A5, docs/decision/records/2026-09-26-adoption.md#A8, docs/decision/records/2026-09-26-adoption.md#A10, docs/decision/records/2026-09-26-adoption.md#A20, docs/decision/records/2026-09-26-adoption.md#A21, docs/decision/records/2026-09-26-adoption.md#A26, docs/decision/records/2026-09-26-adoption.md#A30, docs/decision/records/2026-09-26-adoption.md#A31, docs/decision/records/2026-09-26-adoption.md#A32, docs/decision/records/2026-09-26-adoption.md#A35, docs/decision/records/2026-09-26-adoption.md#A37, docs/decision/records/2026-09-26-adoption.md#A40

「旧資料の記述」は話題の既存の IR の要求を含まない。既存の IR の要求は「既存の要求」の行だけで扱う。FLAG はどの kind でも、その FLAG だけの決定を出典にする。

| 対象 | 状況 | 行き先 |
|---|---|---|
| 旧資料の記述 | 判断の記録の置き場にある出典にできる決定で、実装と一致する | その決定を出典にした要求 |
| 旧資料の記述 | 実装と一致する（出典にできる決定は無い） | 追認の候補 |
| 旧資料の記述 | 実装と食い違う | contradiction の FLAG |
| 旧資料の記述 | 読んでも一致か食い違いかの確信が持てない | ambiguity の FLAG |
| 旧資料の記述 | 実装が無く、人が作る予定ありと答えた | 後回しにする決定を出典にした後回しの要求 |
| 旧資料の記述 | 実装が無く、人が作る予定なしと答えた | 判断の記録の Rejected |
| 旧資料に記述の無い振る舞い | 利用者から見え、それを確かめるテストがある | 旧資料なしと示した追認の候補 |
| 旧資料に記述の無い振る舞い | 利用者から見え、テストが無い | gap の FLAG |
| 旧資料に記述の無い振る舞い | 利用者から見えない内部の振る舞い | IR にしない |
| 追認の候補 | 人が一覧で外さなかった | 行ごとの追認の決定を出典にした要求か用語 |
| 追認の候補 | 人が一覧で外した | contradiction の FLAG |
| 既存の要求 | 実装と一致する | 要求をそのまま残す |
| 既存の要求 | 実装と食い違う | 要求を書き換えず、その ID を related に入れた contradiction の FLAG |
| 既存の要求 | 読んでも一致か食い違いかの確信が持てない | 要求を書き換えず、その ID を related に入れた ambiguity の FLAG |
| 既存の要求 | 実装が無く、人が作る予定ありと答えた | 後回しにする決定を出典にした "- deferred:" の行をその要求に足す |
| 既存の要求 | 実装が無く、人が作る予定なしと答えた | 要求を消さず、その ID を related に入れた contradiction の FLAG |
| 旧資料の用語の定義 | 実装とテストでの使われ方と一致する | 追認の候補 |
| 旧資料の用語の定義 | 実装とテストでの使われ方と食い違う | contradiction の FLAG |
| 旧資料の用語の定義 | 読み方が割れる | ambiguity の FLAG |
| 要求の文で使う用語 | 旧資料に定義が無い | 実装での使われ方から書いた定義を追認の候補にする |

## Examples

```gherkin
@id=EX-core-400 @about=REQ-core-216,REQ-core-219 @source=docs/decision/records/2026-09-26-adoption.md#A21,docs/decision/records/2026-09-26-adoption.md#A9
Scenario: 迷った行は追認される
  Given `旧資料`の記述と実装が一致する振る舞いの行が一覧にあり、人はその行に違和感があるかを決められない
  When 人が一覧のラウンドに答える
  Then その行は外されず、行ごとの追認の決定を`出典`にした`要求`になる

@id=EX-core-401 @about=REQ-core-216,REQ-core-217 @source=docs/decision/records/2026-09-26-adoption.md#A21,docs/decision/records/2026-09-26-adoption.md#A31
Scenario: 一覧で外した行は FLAG になる
  Given `旧資料`の記述と実装が一致する振る舞いの行を、人が一覧で外した
  When kotowari-adopt が IR と`判断の記録`を書く
  Then その行は`要求`にならず、kind が "contradiction" の FLAG になり、その FLAG はそれだけの決定を`出典`にする

@id=EX-core-402 @about=REQ-core-216 @source=docs/decision/records/2026-09-26-adoption.md#A20
Scenario: 残っていた決定は追認より先に使う
  Given `判断の記録`の置き場に、話題の振る舞いを決めた`決定の番号`付きの決定があり、実装と一致する
  When kotowari-adopt が仕分ける
  Then その振る舞いは追認の候補にならず、既存の決定を`出典`にした`要求`になる

@id=EX-core-403 @about=REQ-core-216 @source=docs/decision/records/2026-09-26-adoption.md#A30
Scenario: 既存の IR の要求が実装と食い違う
  Given 話題に既存の`要求` "REQ-x-001" があり、実装がその`文`と違う振る舞いをする
  When kotowari-adopt が仕分ける
  Then "REQ-x-001" は書き換えられず、related に "REQ-x-001" を持つ kind が "contradiction" の FLAG ができる

@id=EX-core-404 @about=REQ-core-216 @source=docs/decision/records/2026-09-26-adoption.md#A30
Scenario: 実装の無い既存の要求を作る予定がある
  Given 話題に既存の`要求` "REQ-x-002" があり、実装が無く、人が一覧で作る予定ありと答えた
  When kotowari-adopt が IR と`判断の記録`を書く
  Then "REQ-x-002" は消されず、`後回し`にする決定を`出典`にした "- deferred:" の行が足される

@id=EX-core-405 @about=REQ-core-216 @source=docs/decision/records/2026-09-26-adoption.md#A32,docs/decision/records/2026-09-26-adoption.md#A4
Scenario: 旧資料に無い振る舞いを人が外す
  Given `旧資料`に記述が無く、利用者から見えてテストのある振る舞いが、旧資料なしと示した追認の候補として一覧にあり、人がそれを外した
  When kotowari-adopt が IR と`判断の記録`を書く
  Then その振る舞いは`要求`にならず、kind が "contradiction" の FLAG になる

@id=EX-core-406 @about=REQ-core-218 @source=docs/decision/records/2026-09-26-adoption.md#A34,docs/decision/records/2026-09-26-adoption.md#A29,docs/decision/records/2026-09-26-adopt-verification.md#A2
Scenario: テストの数の差を内訳で示す
  Given 話題の範囲に既存のテストが300件あり、`要求`の根拠になるのは40件
  When kotowari-adopt が一覧のラウンドを示す
  Then 300件すべての行き先の内訳が示され、どのテストにも`印`は付かず、1件も消されない
```
