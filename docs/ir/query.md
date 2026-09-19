# 1件の読み取り

"kotowari query" が `ID` を1つ受け、その `ID` を持つ`項目`か`シナリオ`を "kotowari list" の1件の形に本文と逆引きを足して出すところを扱う。

## 要求

### REQ-156: query の読み取り

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-20-query-status.md#A1, docs/decision/records/2026-09-20-query-status.md#A4, docs/decision/records/2026-09-20-query-status.md#A6, docs/decision/records/2026-09-20-query-status.md#A19
- 検証: unit

kotowari は常に、"kotowari query" で "kotowari check" と同じ設定と置き場から`IR`の文書と`テストのファイル`を読み、位置引数と同じ `ID` を持つ`項目`と`シナリオ`をすべて "items" に出し、`指摘`を出さず、終了コードを0にする。`IR`の文書に`誤り`があっても、読めた`項目`と`シナリオ`は出す。同じ `ID` を持つものが複数あれば全部出す。

### REQ-157: query の引数

- 種類: event_driven
- 出典: docs/decision/records/records.md#A136, docs/decision/records/2026-09-20-query-status.md#A5, docs/decision/records/2026-09-20-query-status.md#A6
- 検証: unit

"--help" も "--version" も無い "kotowari query" で、位置引数がちょうど1つでないとき、位置引数が `ID` の形でないとき、または位置引数と同じ `ID` を持つ`項目`も`シナリオ`も無いとき、kotowari は引数の誤りを理由に`停止`する。`ID` を持つものが無いときの詳細は "unknown id: " に位置引数の文字を続けた形である。

### REQ-158: query の停止

- 種類: event_driven
- 出典: docs/decision/records/2026-09-20-query-status.md#A19
- 検証: unit

"kotowari query" で、"kotowari check" が`停止`する条件（設定の誤り、読めないファイル、引数の誤り）が成り立つとき、kotowari は check と同じ理由と文言で`停止`する。

### REQ-159: query の1件の形

- 種類: algorithm
- 出典: docs/decision/records/2026-09-20-query-status.md#A2, docs/decision/records/2026-09-20-query-status.md#A3, docs/decision/records/2026-09-20-query-status.md#A4, docs/decision/records/2026-09-20-query-status.md#A16
- 定義: TBL-027
- 検証: unit

### REQ-160: query の並び

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-20-query-status.md#A15, docs/decision/records/2026-09-20-query-status.md#A20
- 検証: unit

kotowari は常に、"items" を "kotowari list" と同じ順に並べ、1件の "referenced_by" を "path" の昇順、同じ "path" の中は "line" の昇順に並べる。

### REQ-161: query の出力の形

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-19-read-commands.md#A7, docs/decision/records/2026-09-20-query-status.md#A4, docs/decision/records/2026-09-20-query-status.md#A5, docs/decision/records/2026-09-20-query-status.md#A7, docs/decision/records/2026-09-20-query-status.md#A13
- 検証: unit

kotowari は常に、"kotowari query" の "--format" の値として "json" と "text" の2つだけを受け、既定を "json" にする。"json" では最上位が "items" だけの JSON を1つ出し、"items" は `TBL-027` の鍵を持つ1件の並びである。"text" では1件ごとに、"kotowari list" の "text" と同じ1行目と "tests" の行を出し、その後に "body" の各行を2つの半角空白で字下げして出し、最後に "referenced_by" の1件ごとに "  <- ID via パス:行" の行を出す。

## 決定表

### TBL-027: query の1件の鍵

- 出典: docs/decision/records/2026-09-20-query-status.md#A2, docs/decision/records/2026-09-20-query-status.md#A3, docs/decision/records/2026-09-20-query-status.md#A4, docs/decision/records/2026-09-20-query-status.md#A16

| 鍵 | 持つ種類 | 中身 |
|---|---|---|
| `TBL-026` のすべての鍵 | `TBL-026` のとおり | `TBL-026` のとおり |
| body | すべて | 本文の行の並び。`項目`は見出しの次の行から、次の "### " か "## " の見出しの前の行まで。`シナリオ`は "@id" のタグの行から最後のステップの行まで。先頭と末尾の空の行は含めない。行の文字はそのまま |
| referenced_by | すべて | その `ID` を指している`項目`と`シナリオ`の並び。1件は "id"、"kind"、"path"、"line"（`TBL-026` と同じ意味）と "via" |
| referenced_by の via | すべて | "definition"（"- 定義:" の行）、"relations"（"- 関係:" の行）、"about"（"@about" のタグ）、"text"（`要求`の文、`性質`の文、`シナリオ`のステップの中の、二重引用符の外でバッククォートで囲んだ `ID`。REQ-054 と同じ判定）のいずれか |

## 具体例

```gherkin
@id=EX-250 @about=REQ-156,REQ-159,TBL-027 @source=docs/decision/records/2026-09-20-query-status.md#A2,docs/decision/records/2026-09-20-query-status.md#A3,docs/decision/records/2026-09-20-query-status.md#A4,docs/decision/records/2026-09-20-query-status.md#A16,docs/decision/records/2026-09-20-query-status.md#A19
Scenario: 1件に本文と逆引きが付く
  Given IR に名前が "例" の要求 "REQ-001" が "docs/ir/a.md" の 7 行目にあり、その下に "- 種類: ubiquitous" と "- 出典:" と "- 検証: unit" の行と文 "文。" がある
  And 同じ文書に "@about=REQ-001" のシナリオ "EX-001" が 20 行目にある
  When "kotowari query REQ-001" を実行する
  Then 終了コードは 0 で、"items" は "id" が "REQ-001" の1件で、その "body" は "- 種類: ubiquitous" の行から "文。" の行までの行の並びで、その "referenced_by" は "id" が "EX-001"、"via" が "about"、"line" が 20 の1件である

@id=EX-251 @about=REQ-157 @source=docs/decision/records/2026-09-20-query-status.md#A6
Scenario: 無い ID は停止する
  Given IR に "REQ-999" を持つ項目もシナリオも無い
  When "kotowari query REQ-999" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は "argument error: unknown id: REQ-999" である

@id=EX-252 @about=REQ-157 @source=docs/decision/records/2026-09-20-query-status.md#A5
Scenario: 位置引数が2つなら停止する
  Given 検査できる IR がある
  When "kotowari query REQ-001 REQ-002" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は "argument error: " で始まる

@id=EX-253 @about=REQ-156 @source=docs/decision/records/2026-09-20-query-status.md#A6
Scenario: 重複した ID は全部出る
  Given IR に "REQ-001" の要求が "docs/ir/a.md" と "docs/ir/b.md" に1つずつある
  When "kotowari query REQ-001" を実行する
  Then 終了コードは 0 で、"items" は "path" が "docs/ir/a.md" と "docs/ir/b.md" の2件である

@id=EX-254 @about=REQ-161 @source=docs/decision/records/2026-09-19-read-commands.md#A7,docs/decision/records/2026-09-19-read-commands.md#A19,docs/decision/records/2026-09-20-query-status.md#A3,docs/decision/records/2026-09-20-query-status.md#A7,docs/decision/records/2026-09-20-query-status.md#A13
Scenario: text は本文と逆引きの行を続ける
  Given EX-250 と同じ IR があり、"tests/a.rs" の 3 行目の印 "@kotowari[REQ-001]" の直後にテスト "req_001_x" がある
  When "kotowari query --format text REQ-001" を実行する
  Then 1行目は "REQ-001 unit 例 docs/ir/a.md:7 tests=1"、2行目は "  tests/a.rs:3 req_001_x"、3行目は "  - 種類: ubiquitous"、最後の行は "  <- EX-001 about docs/ir/a.md:20" である

@id=EX-255 @about=TBL-027 @source=docs/decision/records/2026-09-20-query-status.md#A3,docs/decision/records/2026-09-20-query-status.md#A16
Scenario: シナリオの本文はタグの行から始まる
  Given IR に "@id=EX-001 @about=REQ-001" のタグの行が 19 行目、"Scenario:" の行が 20 行目、ステップが 21 行目から 23 行目にあるシナリオがある
  When "kotowari query EX-001" を実行する
  Then "items" の1件の "body" は 19 行目から 23 行目の5行で、その "referenced_by" は空である

@id=EX-256 @about=REQ-158 @source=docs/decision/records/2026-09-20-query-status.md#A19
Scenario: 設定が読めなければ check と同じく停止する
  Given 設定ファイルが YAML として読めない
  When "kotowari query REQ-001" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は "kotowari check" と同じ文言である

@id=EX-257 @about=TBL-027 @source=docs/decision/records/2026-09-20-query-status.md#A3
Scenario: 定義と文の中の ID が逆引きに出る
  Given IR に決定表 "TBL-001" と、"- 定義: TBL-001" の行を持つ要求 "REQ-001" と、文の中にバッククォートで囲んだ "TBL-001" を書いた要求 "REQ-002" がある
  When "kotowari query TBL-001" を実行する
  Then "items" の1件の "referenced_by" は "id" が "REQ-001" で "via" が "definition" の1件と、"id" が "REQ-002" で "via" が "text" の1件である
```
