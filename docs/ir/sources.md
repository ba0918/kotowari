# 出典の検査

出典の書式と、出典の先が実在するかの検査を扱う。

## 要求

### REQ-057: 出典の書式

- 種類: ubiquitous
- 出典: brainstorm/records.md#A3, brainstorm/records.md#A13, brainstorm/records.md#A38
- 検証: unit

kotowari は常に、`出典`を "パス#印" の形だけで読み、最初の "#" でパスと印に分ける。パスに "#" は書けない。

### REQ-058: 出典の判定

- 種類: algorithm
- 出典: brainstorm/records.md#A38, brainstorm/records.md#A48, brainstorm/records.md#A69
- 定義: TBL-012
- 検証: unit

### REQ-059: 出典が無い

- 種類: event_driven
- 出典: brainstorm/records.md#A38, brainstorm/ir-form.md#検査の種類
- 検証: unit

`要求`、`決定表`、`性質`、`問題の記録`の`項目`に出典の行が無いか空のとき、`シナリオ`に "@source" のタグが無いとき、または`用語`の出典の列が空のとき、kotowari は missing_source の`誤り`を出す。

### REQ-060: 用語集とシナリオの出典

- 種類: ubiquitous
- 出典: brainstorm/records.md#A52, brainstorm/ir-form.md#出典
- 検証: unit

kotowari は常に、`用語集`の出典の列と`シナリオ`の "@source" のタグを、出典の行と同じ規則で検査する。

### REQ-061: 決定の番号はファイルごと

- 種類: ubiquitous
- 出典: brainstorm/records.md#A48
- 検証: unit

kotowari は常に、`決定の番号`を`出典`のパスの指す`判断の記録`のファイルの中だけで探す。

### REQ-062: 内容の照合をしない

- 種類: prohibition
- 出典: brainstorm/records.md#A4
- 検証: review

kotowari は、`出典`がその`項目`の内容を本当に述べているかを判定してはならない。

### REQ-106: 形の契約を出典に指せる

- 種類: ubiquitous
- 出典: brainstorm/records.md#A52, brainstorm/records.md#A69
- 検証: unit

kotowari は常に、形の契約の "brainstorm/ir-form.md" を、"## " の見出しで指す`出典`の先として受ける。

## 決定表

### TBL-012: 出典の判定

- 出典: brainstorm/records.md#A38, brainstorm/records.md#A48, brainstorm/records.md#A69

| 順 | 条件 | 結果 |
|---|---|---|
| 1 | "パス#印" の書式でない | source_invalid |
| 2 | パスが decisions.records の中の、決定の番号を持つファイル（判断の記録）で、印が決定の番号の形で、そのファイルの決定の節に "- 印 " で始まる行がある | 正しい |
| 3 | パスが decisions.records の中の、決定の番号を持つファイル（判断の記録）で、2 に当たらない | source_invalid |
| 4 | パスが decisions.records か decisions.adr の中の、判断の記録でない Markdown のファイル（ADR、形の契約、補足の文書）で、そのファイルがあり、印が "## " の見出しの文字と前後の空白を除いて完全一致する | 正しい |
| 5 | パスが decisions.records か decisions.adr の中で、2 から 4 のどれにも当たらない | source_invalid |
| 6 | パスが decisions.records と decisions.adr のどちらの中でもない | source_invalid |

## 具体例

```gherkin
@id=EX-011 @about=REQ-058 @source=brainstorm/records.md#A38
Scenario: 決定の節にある番号は正しい出典である
  Given "decisions.records" が "brainstorm" で、"brainstorm/records.md" の Agreements の節に "- A26 " で始まる行がある
  When 出典 "brainstorm/records.md#A26" を検査する
  Then source_invalid の誤りは出ない

@id=EX-012 @about=REQ-058 @source=brainstorm/records.md#A38,brainstorm/ir-form.md#検査の種類
Scenario: 無い番号は誤りになる
  Given "brainstorm/records.md" に "- A99 " で始まる行が無い
  When 出典 "brainstorm/records.md#A99" を検査する
  Then detail が "brainstorm/records.md#A99" の source_invalid の誤りが出る
```
