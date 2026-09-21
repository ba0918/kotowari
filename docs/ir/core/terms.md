# 用語と曖昧語と文書名の参照

バッククォートで囲んだ語、曖昧語、文書名の参照の検査を扱う。

## 要求

### REQ-core-063: 対象の行

- 種類: algorithm
- 出典: docs/decision/records/records.md#A42, docs/decision/records/records.md#A53, docs/decision/records/records.md#A56
- 定義: TBL-core-013
- 検証: unit

### REQ-core-064: 用語集に無い語

- 種類: event_driven
- 出典: docs/decision/records/records.md#A31, docs/decision/records/records.md#A42, docs/decision/records/records.md#A56, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A63, docs/decision/records/records.md#A116, docs/decision/records/records.md#A145
- 検証: unit

`対象の行`の二重引用符の外でバッククォートで囲んだもの（前後の空白を除いた文字）が`用語`でも`ID`でもないとき、kotowari は、それがパスかコード片であっても、除いた後の文字を detail にして unknown_term の`誤り`を出す。中身が空の囲みは detail を "``" にする。

### REQ-core-065: 用語集が無いとき

- 種類: event_driven
- 出典: docs/decision/records/records.md#A56, docs/decision/records/records.md#A63, docs/decision/records/records.md#A53, docs/decision/records/2026-09-16-ir-tree.md#A3
- 検証: unit

文書の`連鎖`に`用語集`が1つも無いとき、kotowari は`対象の行`でバッククォートで囲んだもののうち、`ID`でないものをすべて unknown_term の`誤り`にする。

### REQ-core-066: 曖昧語

- 種類: event_driven
- 出典: docs/decision/records/records.md#A21, docs/decision/records/records.md#A41, docs/decision/records/records.md#A42, docs/decision/records/records.md#A53, docs/decision/records/ir-form.md#検査の種類
- 検証: unit

`対象の行`に`曖昧語`が部分一致で含まれるとき、kotowari は vague_word の`誤り`を出す。

### REQ-core-067: 出現ごとに1件

- 種類: ubiquitous
- 出典: docs/decision/records/records.md#A53, docs/decision/records/records.md#A56, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A117, docs/decision/records/records.md#A142
- 検証: unit

kotowari は常に、unknown_term と vague_word の`指摘`を出現ごとに1件出し、`曖昧語`の出現は行の左から最長一致で重ならない形で数える。

### REQ-core-068: 囲み忘れを検出しない

- 種類: prohibition
- 出典: docs/decision/records/records.md#A31, docs/decision/records/2026-09-17-check-reach.md#A3, docs/decision/records/2026-09-17-check-reach.md#A4
- 検証: unit

kotowari は、`用語`をバッククォートで囲み忘れたことを検出してはならない。

### REQ-core-069: 文書名の参照の見つけ方

- 種類: algorithm
- 出典: docs/decision/records/records.md#A42, docs/decision/records/records.md#A47, docs/decision/records/2026-09-16-ir-tree.md#A21
- 定義: TBL-core-014
- 検証: unit

### REQ-core-070: 参照された文書が無い

- 種類: event_driven
- 出典: docs/decision/records/2026-09-16-ir-tree.md#A5, docs/decision/records/2026-09-16-ir-tree.md#A17, docs/decision/records/2026-09-16-ir-tree.md#A18, docs/decision/records/ir-form.md#検査の種類
- 検証: unit

`文書名の参照`が "/" を含まないときにその名前の文書が参照を書いた文書と同じディレクトリに無いとき、"/" を含むときにその`IR`の置き場からの相対パスの文書が無いとき、または並びに "." か ".." の要素を含むとき、kotowari は参照の文字列を detail にして missing_document の`誤り`を出す。文書の有無は読んだ`IR`の文書の中に有るかで見る（ディレクトリのシンボリックリンクの下にあって読まない文書は無いものとする）。同じディレクトリに無い文書を上のディレクトリへ辿って探さない。

## 決定表

### TBL-core-013: 用語と曖昧語の検査の対象

- 出典: docs/decision/records/records.md#A42, docs/decision/records/records.md#A53, docs/decision/records/records.md#A56, docs/decision/records/records.md#A133

| 行 | 検査 |
|---|---|
| 要求の文 | 対象 |
| 性質の文 | 対象 |
| Gherkin の Given、When、Then、And、But の行 | 対象 |
| Gherkin の Scenario の行 | 対象外 |
| "- " の行 | 対象外 |
| タグの行 | 対象外 |
| 用語集の意味の列 | 対象外 |
| 問題の記録の本文 | 対象外 |

### TBL-core-014: 文書名の参照の条件

- 出典: docs/decision/records/records.md#A47, docs/decision/records/ir-form.md#文書名の参照, docs/decision/records/records.md#A73, docs/decision/records/records.md#A118, docs/decision/records/2026-09-16-ir-tree.md#A5, docs/decision/records/2026-09-16-ir-tree.md#A12, docs/decision/records/2026-09-16-ir-tree.md#A14, docs/decision/records/2026-09-16-ir-tree.md#A17, docs/decision/records/2026-09-16-ir-tree.md#A21

| 順 | 条件 |
|---|---|
| 1 | コードブロックの外にある |
| 2 | 並びの先頭の直前が、英数字、"_"、"-"、"/"、"."、バッククォートのいずれでもない（行頭を含む。空白、句読点、日本語の文字は境界になる）。並びの中に出る文字（英小文字、数字、ハイフン、"."、"/"）が直前にあるときは境界にならないので、並びの途中から参照を拾うことはない |
| 3 | 要素（英小文字と数字とハイフンの並び、"."、".." のいずれか）を "/" で区切って1つ以上並べ、最後の要素が英小文字と数字とハイフンの並びで ".md" が続き、".md" の直後が英数字、"_"、"-"、"#"、"/" のいずれでもない |
| 4 | 二重引用符の中にない。行の中の二重引用符が奇数のときは、最後の引用符から行末までを引用符の中と見なす |

## 具体例

```gherkin
@id=EX-core-013 @about=REQ-core-064 @source=docs/decision/records/records.md#A42,docs/decision/records/records.md#A56,docs/decision/records/ir-form.md#検査の種類
Scenario: 囲んだパスは誤りになる
  Given `要求`の`文`に "src/main.rs" をバッククォートで囲んで書いている
  When "kotowari check" を実行する
  Then detail が "src/main.rs" の unknown_term の誤りが出る

@id=EX-core-014 @about=REQ-core-069,REQ-core-070 @source=docs/decision/records/2026-09-16-ir-tree.md#A5,docs/decision/records/2026-09-16-ir-tree.md#A15,docs/decision/records/2026-09-16-ir-tree.md#A21
Scenario: 置き場の外のパスは引用符で囲む
  Given 文書に "docs/decision/adr/0001-test-marker.md" と引用符なしで書いている
  When "kotowari check" を実行する
  Then "0001-test-marker.md" だけを指す参照にはならず、"docs/decision/adr/0001-test-marker.md" の missing_document の誤りが出る

@id=EX-core-022 @about=REQ-core-069 @source=docs/decision/records/2026-09-16-ir-tree.md#A14
Scenario: 出典の形は参照にならない
  Given 文書の範囲の行に "docs/decision/records/records.md#A12" と引用符なしで書いている
  When "kotowari check" を実行する
  Then missing_document の誤りは出ない

@id=EX-core-023 @about=REQ-core-070 @source=docs/decision/records/2026-09-16-ir-tree.md#A5
Scenario: 素の名前は同じディレクトリだけを見る
  Given "docs/ir/network/dns/a.md" に "b.md" と書き、"docs/ir/network/b.md" はあるが "docs/ir/network/dns/b.md" は無い
  When "kotowari check" を実行する
  Then "b.md" の missing_document の誤りが出る

@id=EX-core-024 @about=REQ-core-070 @source=docs/decision/records/2026-09-16-ir-tree.md#A5
Scenario: スラッシュを含む名前は置き場からの相対で探す
  Given "docs/ir/network/dns/a.md" に "network/publish/c.md" と書き、"docs/ir/network/publish/c.md" がある
  When "kotowari check" を実行する
  Then missing_document の誤りは出ない

@id=EX-core-025 @about=REQ-core-070 @source=docs/decision/records/2026-09-16-ir-tree.md#A17
Scenario: "." と ".." の要素を含む並びは解決しない
  Given "docs/ir/network/dns/a.md" に "../b.md" と "./c.md" と書き、"docs/ir/network/b.md" と "docs/ir/network/dns/c.md" がある
  When "kotowari check" を実行する
  Then "../b.md" と "./c.md" の missing_document の誤りが1件ずつ出る

@id=EX-core-031 @about=REQ-core-069 @source=docs/decision/records/2026-09-16-ir-tree.md#A17,docs/decision/records/2026-09-16-ir-tree.md#A21
Scenario: ".md" の後に "/" が続く並びは参照にならない
  Given 文書に "a.md/b.md" と引用符なしで書き、"a.md" も "b.md" も "md/b.md" も無い
  When "kotowari check" を実行する
  Then missing_document の誤りは出ない

@id=EX-core-033 @about=REQ-core-069 @source=docs/decision/records/2026-09-16-ir-tree.md#A21
Scenario: 日本語の文字に直接つなげた参照も拾う
  Given 文書の範囲の行に "設定の形はtimeout-config.mdで定める" と書き、"timeout-config.md" が同じディレクトリに無い
  When "kotowari check" を実行する
  Then "timeout-config.md" の missing_document の誤りが出る

@id=EX-core-034 @about=REQ-core-069 @source=docs/decision/records/2026-09-16-ir-tree.md#A21
Scenario: バッククォートで囲んだパスは参照にならない
  Given 文書の`文`に "`a.md`" と書き、"a.md" は無く、`用語集`にも無い
  When "kotowari check" を実行する
  Then "a.md" の unknown_term の誤りが出て、missing_document の誤りは出ない

@id=EX-core-032 @about=REQ-core-070 @source=docs/decision/records/2026-09-16-ir-tree.md#A18
Scenario: 読まない場所の文書への参照は無いものとして扱う
  Given "docs/ir/link" が置き場の外のディレクトリを指すシンボリックリンクで、その下に "d.md" があり、"docs/ir/a.md" に "link/d.md" と書いている
  When "kotowari check" を実行する
  Then "link/d.md" の missing_document の誤りが出る

@id=EX-core-026 @about=REQ-core-065 @source=docs/decision/records/2026-09-16-ir-tree.md#A3
Scenario: 連鎖に用語集が無い文書は囲んだ語がすべて誤りになる
  Given "docs/ir/CONTEXT.md" は無く "docs/ir/network/CONTEXT.md" に`用語`があり、"docs/ir/a.md" の`文`にその`用語`を囲んで書いている
  When "kotowari check" を実行する
  Then "docs/ir/a.md" に unknown_term の誤りが出て、"docs/ir/network/" の文書には出ない
```
