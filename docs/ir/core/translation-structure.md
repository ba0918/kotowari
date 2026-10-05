# 対の骨組みと切り替えの行とリンク

`対`の`側`どうしで一致しなければならない`骨組み`、`IR`と`ガイド`の`側`に置く`切り替えの行`、`対`の`側`の中のリンクの検査、`対`を揃える LLM の手順を扱う。`対`の読み方と`一致の記録`は translation-pairs.md が扱う。

## Requirements

### REQ-core-344: 骨組みの一致

- kind: algorithm
- source: docs/decision/records/2026-10-05-localization.md#A19, docs/decision/records/2026-10-05-localization.md#A26, docs/decision/records/2026-10-05-localization.md#A30, docs/decision/records/2026-10-05-localization.md#A33, docs/decision/records/2026-10-05-localization.md#D3
- definition: TBL-core-044, TBL-core-045
- verification: unit

### REQ-core-345: 骨組みの食い違い

- kind: event_driven
- source: docs/decision/records/2026-10-05-localization.md#A19, docs/decision/records/2026-10-05-localization.md#A27, docs/decision/records/2026-10-05-localization.md#D1, docs/decision/records/2026-10-05-localization.md#D2, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

`先頭の言語`でない言語の`側`の`骨組み`が、`先頭の言語`の`側`の`骨組み`と TBL-core-044 のどれかの部分で一致しないとき、kotowari は "path" をその`側`、detail をその文書の種類の TBL-core-044 の行の順で最初に一致しない部分の名前にして、translation_structure_mismatch の`誤り`を`側`ごとに1件出す。"line" は、その部分の中で`先頭の言語`の`側`の行の順に比べて最初に食い違った要素のその`側`の行とし、食い違いが数の違いか、その要素がその`側`に無いことなら null とする。行の番号そのものは比べない。意味が同じかは検査しない。

### REQ-core-346: 切り替えの行

- kind: event_driven
- source: docs/decision/records/2026-10-05-localization.md#A27, docs/decision/records/2026-10-05-localization.md#A33, docs/decision/records/2026-10-05-localization.md#A36, docs/decision/records/2026-10-05-localization.md#A40, docs/decision/records/2026-10-05-localization.md#D1, docs/decision/records/2026-10-05-localization.md#D2, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

`IR`と`ガイド`の`対`の各`側`で、`題名`の後の最初の空でない行（`題名`が無ければファイルの最初の空でない行）が、`言語の一覧`の順に各言語の`UI の文字`の "language_name" を " | " で区切って並べ、その`側`の言語のものは文字だけ、ほかの言語のものは "[<名前>](<その言語の側のファイル名>)" の形にした行と同じでないとき、kotowari は "path" をその`側`、"line" をその行（その行が無ければ`題名`の行、`題名`も無ければ null）、detail をあるべき行にして translation_switcher_invalid の`誤り`を出す。あるべき行と同じ行だけを`切り替えの行`とし、`切り替えの行`は`文書が扱う範囲`の行に数えず、`用語`、`曖昧語`、`文書名の参照`の検査を受けない。`全体像の元データ`と`目次`の`対`は`切り替えの行`を持たない。

### REQ-core-347: 検査するリンク

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A14, docs/decision/records/2026-10-05-localization.md#A30, docs/decision/records/2026-10-05-localization.md#A33, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

kotowari は常に、`IR`、`ガイド`、`全体像の元データ`の`対`の各`側`で、`切り替えの行`の外にある CommonMark のリンクと画像の行き先、リンクの参照の定義の行き先のうち、スキーム（英字で始まり ":" が続く並び）で始まらず "#" で始まらないものを検査するリンクとし、その行き先の "#" と "?" より前を、その`側`のあるディレクトリから辿ったパスとして読む。

### REQ-core-348: ほかの言語の側へのリンク

- kind: event_driven
- source: docs/decision/records/2026-10-05-localization.md#A14, docs/decision/records/2026-10-05-localization.md#A27, docs/decision/records/2026-10-05-localization.md#A30, docs/decision/records/2026-10-05-localization.md#D1, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

検査するリンクの行き先が、ある`対`の`側`で、その言語がリンクのある`側`の言語と違うとき、kotowari は "path" をリンクのある`側`、"line" をリンクの行、detail を書かれた行き先にして link_language_mismatch の`誤り`を出す。

### REQ-core-349: 判断の記録へのリンク

- kind: event_driven
- source: docs/decision/records/2026-10-05-localization.md#A14, docs/decision/records/2026-10-05-localization.md#A27, docs/decision/records/2026-10-05-localization.md#A38, docs/decision/records/2026-10-05-localization.md#D1, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

検査するリンクの行き先が "decisions.records" か "decisions.adr" の置き場の下のファイルのとき、kotowari は "path" をリンクのある`側`、"line" をリンクの行、detail を書かれた行き先にして link_to_record の`誤り`を出す。

### REQ-core-350: 対を揃える手順

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A25, docs/decision/records/2026-10-05-localization.md#A35, docs/decision/records/2026-10-05-localization.md#A40
- verification: review
- how_to_verify: "agent/skills/kotowari/" の skill を読み、`対`のどれかの`側`を直したら同じ変更でほかのすべての`側`を直すこと、直すときは`一致の記録`の blob hash から前の文を git で取り出して差分だけを訳し、取り出せなければ全文から訳し直すこと、直した後に "kotowari list" の "translations" の値で`一致の記録`を書き直すこと、"languages" に英語でない言語を足すときは "labels.<言語タグ>" を書くことが、それぞれ手順として書かれていることを確かめる

kotowari の skill は常に、`対`を揃える手順を持つ。

## Decision tables

### TBL-core-044: 骨組み

- source: docs/decision/records/2026-10-05-localization.md#A19, docs/decision/records/2026-10-05-localization.md#A26, docs/decision/records/2026-10-05-localization.md#A30, docs/decision/records/2026-10-05-localization.md#D1, docs/decision/records/2026-10-05-localization.md#D2, docs/decision/records/2026-10-05-localization.md#A33, docs/decision/records/2026-10-05-localization.md#D3

どの文書でも、`切り替えの行`と自然言語の文は`骨組み`に入らない。リンクの行き先は、ほかの言語の`対`の`側`を指すものをその`対`の`先頭の言語`の`側`に読み替え、"#" より前だけを比べる。

| 文書 | 部分の名前 | 一致しなければならないもの |
|---|---|---|
| `話題ごとの文書` | heading | "## " と "### " の見出しの深さと並び。"### " の見出しでは`ID`も |
| `話題ごとの文書` | field | `項目`の下と文書単位の "- " の行のうち、"- kind:"、"- source:"、"- verification:"、"- definition:"、"- deferred:"、"- related:" の名前と値と並び、"- how_to_verify:" の行の有無 |
| `話題ごとの文書` | table | 表の数と、各表の行の数と列の数と、各セルの中の`ID`の並び |
| `話題ごとの文書` | gherkin | gherkin のコードブロックの中のタグの行の中身と、タグの行、"Scenario:" の行、ステップの行の始まりの語（Given、When、Then、And、But）の並び |
| `話題ごとの文書` | code | gherkin でないコードブロックの情報文字列と中身 |
| `用語集` | glossary | `用語集`の表の行の数と、各行の出典の列 |
| `問題の記録` | flag | "### FLAG-" の見出しの`ID`と並び、"- kind:"、"- related:"、"- source:" の値 |
| `ガイド` | heading | 見出しの深さと並び |
| `ガイド` | mark | `ガイドの印`の中身と並び |
| `ガイド` | code | コードブロックの情報文字列と中身 |
| `ガイド` | table | 表の数と、各表の行の数と列の数 |
| `ガイド` | link | 検査するリンクの行き先の並び |
| `全体像の元データ` | frontmatter | frontmatter の中身 |
| `全体像の元データ` | heading | 見出しの深さと並び |
| `全体像の元データ` | part | `部品`の種類と並びと、`部品`の値から TBL-core-045 の文の欄の文字列を除いたもの（並びの長さ、鍵、文でない値） |
| `全体像の元データ` | mark | `ガイドの印`の中身と並び |
| `全体像の元データ` | table | 表の数と、各表の行の数と列の数 |
| `全体像の元データ` | link | 検査するリンクの行き先の並び |
| `目次` | toc | `目次の群`の入れ子と、名前の項目の並びと、各`目次の群`の "note" の有無 |

### TBL-core-045: 全体像の部品の文の欄

- source: docs/decision/records/2026-10-05-localization.md#A19, docs/decision/records/2026-10-05-localization.md#A3

ここに無い欄（"tone"、"state"、"refs"、"ref"、"width" と、並びや入れ子の形）は文でなく、`骨組み`に入る。

| `部品`の種類 | 文の欄 |
|---|---|
| lead | conclusion、points の各要素 |
| flow | 箱の title、body |
| steps | title、body |
| cards | title、items の各要素 |
| status | text |
| compare | before、after、why |
| decisions | text、by |
| quiz | q、a |

## Examples

```gherkin
@id=EX-core-526 @about=REQ-core-344,REQ-core-345 @source=docs/decision/records/2026-10-05-localization.md#A19,docs/decision/records/2026-10-05-localization.md#A27
Scenario: 文だけが違えば骨組みは一致する
  Given 設定の "languages" が "[ja, en]" で、`IR`の "a.md" と "a.en.md" が同じ見出しと "- " の行とシナリオのタグを持ち、文とステップの行の文字だけが違う
  When "kotowari check --format json" を実行する
  Then translation_structure_mismatch の`指摘`は出ない

@id=EX-core-527 @about=REQ-core-345 @source=docs/decision/records/2026-10-05-localization.md#A19,docs/decision/records/2026-10-05-localization.md#A27,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: 出典が違えば最初に食い違った行に誤りが出る
  Given EX-core-526 の文書があり、"a.en.md" の 9 行目の "- source:" の行の値だけが "a.md" と違う
  When "kotowari check --format json" を実行する
  Then "path" が "docs/ir/a.en.md"、"line" が 9、detail が "field" の translation_structure_mismatch の`誤り`が1件出る

@id=EX-core-528 @about=REQ-core-344 @source=docs/decision/records/2026-10-05-localization.md#A19,docs/decision/records/2026-10-05-localization.md#A27,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: 全体像の部品の文でない欄が違えば誤りになる
  Given 設定の "languages" が "[ja, en]" で、`全体像の元データ` ".kotowari/overview/a.md" と ".kotowari/overview/a.en.md" の status の`部品`の "state" が "open" と "decided" で違い、"text" も違う
  When "kotowari check --format json" を実行する
  Then "path" が ".kotowari/overview/a.en.md"、detail が "part" の translation_structure_mismatch の`誤り`が出る

@id=EX-core-529 @about=REQ-core-344 @source=docs/decision/records/2026-10-05-localization.md#A26,docs/decision/records/2026-10-05-localization.md#A27,docs/decision/records/2026-10-05-localization.md#D3
Scenario: 目次の題名だけが違えば一致する
  Given 設定の "languages" が "[ja, en]" で、`目次` "toc.yaml" と "toc.en.yaml" の入れ子と名前の項目が同じで、"title" と "note" の文字だけが違う
  When "kotowari check --format json" を実行する
  Then translation_structure_mismatch の`指摘`は出ない

@id=EX-core-530 @about=REQ-core-346 @source=docs/decision/records/2026-10-05-localization.md#A33,docs/decision/records/2026-10-05-localization.md#A40,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: 切り替えの行が正しければ何も出ない
  Given 設定の "languages" が "[ja, en]" で "labels.ja.language_name" が "日本語" であり、"guides/a.md" の`題名`の次の空でない行が "日本語 | [English](a.en.md)"、"guides/a.en.md" の行が "[日本語](a.md) | English" である
  When "kotowari check --format json" を実行する
  Then translation_switcher_invalid の`指摘`は出ない

@id=EX-core-531 @about=REQ-core-346 @source=docs/decision/records/2026-10-05-localization.md#A33,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: 切り替えの行が無ければあるべき行を出す
  Given EX-core-530 の設定があり、"guides/b.md" の`題名`の次の空でない行が "本文。" である
  When "kotowari check --format json" を実行する
  Then "path" が "guides/b.md"、detail が "日本語 | [English](b.en.md)" の translation_switcher_invalid の`誤り`が出る

@id=EX-core-532 @about=REQ-core-346 @source=docs/decision/records/2026-10-05-localization.md#A36,docs/decision/records/2026-10-05-localization.md#A33
Scenario: 全体像の元データは切り替えの行を持たない
  Given 設定の "languages" が "[ja, en]" で、`全体像の元データ` "a.md" の`題名`の次が lead の`部品`である
  When "kotowari check --format json" を実行する
  Then "a.md" に translation_switcher_invalid の`指摘`は出ない

@id=EX-core-533 @about=REQ-core-347,REQ-core-348 @source=docs/decision/records/2026-10-05-localization.md#A14,docs/decision/records/2026-10-05-localization.md#A30,docs/decision/records/2026-10-05-localization.md#A27,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: 英語のガイドが日本語の IR を指せば誤りになる
  Given 設定の "languages" が "[ja, en]" で、`IR`の "docs/ir/a.md" と "docs/ir/a.en.md" があり、"guides/g.en.md" の 5 行目に "../docs/ir/a.md#REQ-001" へのリンクがある
  When "kotowari check --format json" を実行する
  Then "path" が "guides/g.en.md"、"line" が 5、detail が "../docs/ir/a.md#REQ-001" の link_language_mismatch の`誤り`が出る

@id=EX-core-534 @about=REQ-core-347,REQ-core-348 @source=docs/decision/records/2026-10-05-localization.md#A14,docs/decision/records/2026-10-05-localization.md#A30,docs/decision/records/2026-10-05-localization.md#A27
Scenario: 同じ言語の側と外の URL へのリンクは誤りにしない
  Given EX-core-533 の`IR`があり、"guides/g.en.md" に "../docs/ir/a.en.md#other" と "https://example.com/a.md" と "#top" へのリンクがある
  When "kotowari check --format json" を実行する
  Then "guides/g.en.md" に link_language_mismatch の`指摘`は出ない

@id=EX-core-535 @about=REQ-core-349 @source=docs/decision/records/2026-10-05-localization.md#A14,docs/decision/records/2026-10-05-localization.md#A38,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: 対の文書から判断の記録へリンクすれば誤りになる
  Given 設定の "languages" が "[ja, en]" で、"guides/g.md" の 3 行目に "../docs/decision/records/r.md#A1" へのリンクがある
  When "kotowari check --format json" を実行する
  Then "path" が "guides/g.md"、"line" が 3 の link_to_record の`誤り`が出る
```
