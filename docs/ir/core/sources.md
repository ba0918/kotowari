# 出典の検査

出典の書式と、出典の先が実在するかの検査を扱う。

## Requirements

### REQ-core-057: 出典の書式

- kind: ubiquitous
- source: docs/decision/records/records.md#A3, docs/decision/records/records.md#A13, docs/decision/records/records.md#A38, docs/decision/records/records.md#A84, docs/decision/records/records.md#A106
- verification: unit

kotowari は常に、`出典`を "パス#印" の形だけで読み、最初の "#" でパスと印に分ける。パスに "#" は書けない。パスは`基準のディレクトリ`からの相対で、置き場からの全体を書く（"docs/decision/records/records.md#A26" の形）。パスは REQ-core-110 の正規化の後で置き場と比べる。

### REQ-core-058: 出典の判定

- kind: algorithm
- source: docs/decision/records/records.md#A38, docs/decision/records/records.md#A48, docs/decision/records/records.md#A69
- definition: TBL-core-012
- verification: unit

### REQ-core-059: 出典が無い

- kind: event_driven
- source: docs/decision/records/records.md#A38, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A90
- verification: unit

`要求`、`決定表`、`性質`、`問題の記録`の`項目`に出典の行が無いか空のとき、`シナリオ`に "@source" のタグが無いとき、または`用語`の出典の列が空のとき、kotowari は missing_source の`誤り`を出す。"@id" の無い`シナリオ`では detail は "Scenario:" の行の文字にする。

### REQ-core-060: 用語集とシナリオの出典

- kind: ubiquitous
- source: docs/decision/records/records.md#A52, docs/decision/records/ir-form.md#出典
- verification: unit

kotowari は常に、`用語集`の出典の列と`シナリオ`の "@source" のタグを、出典の行と同じ規則で検査する。

### REQ-core-061: 決定の番号はファイルごと

- kind: ubiquitous
- source: docs/decision/records/records.md#A48, docs/decision/records/records.md#A115
- verification: unit

kotowari は常に、`決定の番号`を`出典`のパスの指す`判断の記録`のファイルの中だけで探す。`決定の節`は "## " の見出しで始まり次の "## " の見出しで終わり、"### " の見出しは節を終えない。

### REQ-core-062: 内容の照合をしない

- kind: prohibition
- source: docs/decision/records/records.md#A4, docs/decision/records/2026-09-17-check-reach.md#A3, docs/decision/records/2026-09-17-check-reach.md#A4
- verification: unit

kotowari は、`出典`がその`項目`の内容を本当に述べているかを判定してはならない。

### REQ-core-106: 形の契約を出典に指せる

- kind: ubiquitous
- source: docs/decision/records/records.md#A52, docs/decision/records/records.md#A69, docs/decision/records/2026-09-16-ir-tree.md#A2
- verification: unit

kotowari は常に、形の契約の "docs/decision/records/ir-form.md" を、"## " の見出しで指す`出典`の先として受ける。

### REQ-core-115: 出典の指摘の行

- kind: ubiquitous
- source: docs/decision/records/records.md#A114, docs/decision/records/2026-09-23-ir-english-tokens.md#A2, docs/decision/records/2026-09-25-deferred-items.md#A24
- verification: unit

kotowari は常に、source_invalid の "line" を`出典`が書かれた行（`項目`なら "- source:" の行、`シナリオ`ならタグの行、`用語`なら表の行、`後回し`の宣言ならその "- deferred:" の行）にする。

## Decision tables

### TBL-core-012: 出典の判定

- source: docs/decision/records/records.md#A38, docs/decision/records/records.md#A48, docs/decision/records/records.md#A69, docs/decision/records/records.md#A91, docs/decision/records/records.md#A115, docs/decision/records/records.md#A134, docs/decision/records/records.md#A158, docs/decision/records/records.md#A165, docs/decision/records/2026-09-17-record-form.md#A33, docs/decision/records/2026-09-17-record-form.md#A34, docs/decision/records/2026-09-17-record-form.md#A47, docs/decision/records/2026-09-24-review5-gaps.md#A3

パスが decisions.records と decisions.adr の両方の中にあるとき（一方の置き場が他方の下にあるとき）は、深い方の置き場の中のファイルとして判定する。

| 順 | 条件 | 結果 |
|---|---|---|
| 1 | "パス#印" の書式でない | source_invalid |
| 2 | パスが decisions.records の中の`判断の記録`（`決定の節`の見出しを1つ以上持つファイル）で、印が決定の番号の形（英大文字1文字に1桁以上の数字）で、そのファイルの決定の節に、印の番号の`番号の行`がある（`コードブロック`の中は除外） | 正しい |
| 3 | パスが decisions.records の中の`判断の記録`で、2 に当たらない | source_invalid |
| 4 | パスが decisions.records か decisions.adr の中の、判断の記録でない Markdown のファイル（ADR、形の契約、補足の文書。ディレクトリでも通常のファイルでもないものは`除外`で、ファイルに数えない）で、そのファイルがあり、印が`コードブロック`の外の "## " の見出しの文字と前後の空白を除いて完全一致する | 正しい |
| 5 | パスが decisions.records か decisions.adr の中で、2 から 4 のどれにも当たらない | source_invalid |
| 6 | パスが decisions.records と decisions.adr のどちらの中でもない | source_invalid |

## Examples

```gherkin
@id=EX-core-285 @about=TBL-core-012 @source=docs/decision/records/2026-09-24-review5-gaps.md#A3
Scenario: 両方の置き場の中のファイルは深い方の置き場で判定する
  Given decisions.records が "docs/decision"、decisions.adr が "docs/decision/adr" の設定がある
  And "docs/decision/adr" の下の ADR の見出しと、"docs/decision/records" の下の`判断の記録`の決定を出典に持つ`要求`がある
  When "kotowari check" を実行する
  Then source_invalid は出ない

@id=EX-core-011 @about=REQ-core-058 @source=docs/decision/records/records.md#A38
Scenario: 決定の節にある番号は正しい出典である
  Given "decisions.records" が "docs/decision/records" で、"docs/decision/records/records.md" の Agreements の節に "- A26 " で始まる行がある
  When 出典 "docs/decision/records/records.md#A26" を検査する
  Then source_invalid の誤りは出ない

@id=EX-core-012 @about=REQ-core-058 @source=docs/decision/records/records.md#A38,docs/decision/records/ir-form.md#検査の種類
Scenario: 無い番号は誤りになる
  Given "decisions.records" が "docs/decision/records" で、"docs/decision/records/records.md" に "- A999 " で始まる行が無い
  When 出典 "docs/decision/records/records.md#A999" を検査する
  Then detail が "docs/decision/records/records.md#A999" の source_invalid の誤りが出る

@id=EX-core-120 @about=REQ-core-058 @source=docs/decision/records/2026-09-17-record-form.md#A47,docs/decision/records/2026-09-17-record-form.md#A45
Scenario: 判断の記録でないファイルのコードブロックの中の見出しは出典の先にならない
  Given "decisions.records" が "docs/decision/records" で、判断の記録でない "docs/decision/records/g.md" が "## 補足" の見出しをコードブロックの外に持ち、"## 例" の見出しをコードブロックの中にだけ持つ
  When 出典 "docs/decision/records/g.md#補足" と "docs/decision/records/g.md#例" を検査する
  Then "docs/decision/records/g.md#補足" に source_invalid は出ず、detail が "docs/decision/records/g.md#例" の source_invalid の誤りが出る
```
