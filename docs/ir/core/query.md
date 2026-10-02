# 1件の読み取り

"kotowari query" が `ID` を1つ受け、その `ID` を持つ`項目`か`シナリオ`を "kotowari list" の1件の形に本文と逆引きを足して出すところを扱う。

## Requirements

### REQ-core-156: query の読み取り

- kind: ubiquitous
- source: docs/decision/records/2026-09-20-query-status.md#A1, docs/decision/records/2026-09-20-query-status.md#A4, docs/decision/records/2026-09-20-query-status.md#A6, docs/decision/records/2026-09-20-query-status.md#A19
- verification: unit

kotowari は常に、"kotowari query" で "kotowari check" と同じ設定と置き場から`IR`の文書と`テストのファイル`を読み、位置引数と同じ `ID` を持つ`項目`と`シナリオ`をすべて "items" に出し、`指摘`を出さず、終了コードを0にする。`IR`の文書に`誤り`があっても、読めた`項目`と`シナリオ`は出す。同じ `ID` を持つものが複数あれば全部出す。

### REQ-core-157: query の引数

- kind: event_driven
- source: docs/decision/records/records.md#A136, docs/decision/records/2026-09-20-query-status.md#A5, docs/decision/records/2026-09-20-query-status.md#A6
- verification: unit

"--help" も "--version" も無い "kotowari query" で、位置引数がちょうど1つでないとき、位置引数が `ID` の形でないとき、または位置引数と同じ `ID` を持つ`項目`も`シナリオ`も無いとき、kotowari は引数の誤りを理由に`停止`する。`ID` を持つものが無いときの詳細は "unknown id: " に位置引数の文字を続けた形である。

### REQ-core-158: query の停止

- kind: event_driven
- source: docs/decision/records/2026-09-20-query-status.md#A19, docs/decision/records/2026-09-24-doc-marks.md#A22, docs/decision/records/2026-09-27-surface-check.md#A14, docs/decision/records/2026-09-27-surface-check.md#A24, docs/decision/records/2026-10-02-whole-picture.md#A73
- verification: unit

"kotowari query" で、"kotowari check" が`停止`する条件（設定の誤り、読めないファイル、引数の誤り）が成り立つとき、kotowari は check と同じ理由と文言で`停止`する。 ただし "kotowari query" は`ガイド`を読まず、`ガイド`の置き場と読み込みによる`停止`（REQ-core-198、REQ-core-199）はしない。`面のファイル`と`面の規則`のファイルと`未記載の面の一覧`も読まず、その読み込みによる`停止`（REQ-core-229）はしない。`全体像の元データ`も読まず、その置き場と読み込みによる`停止`（REQ-core-278、REQ-core-280）はしない。

### REQ-core-159: query の1件の形

- kind: algorithm
- source: docs/decision/records/2026-09-20-query-status.md#A2, docs/decision/records/2026-09-20-query-status.md#A3, docs/decision/records/2026-09-20-query-status.md#A4, docs/decision/records/2026-09-20-query-status.md#A16, docs/decision/records/2026-09-23-ir-engine-gaps.md#A18, docs/decision/records/2026-09-23-ir-engine-gaps.md#A24
- definition: TBL-core-027
- verification: unit

### REQ-core-160: query の並び

- kind: ubiquitous
- source: docs/decision/records/2026-09-20-query-status.md#A15, docs/decision/records/2026-09-20-query-status.md#A20
- verification: unit

kotowari は常に、"items" を "kotowari list" と同じ順に並べ、1件の "referenced_by" を "path" の昇順、同じ "path" の中は "line" の昇順に並べる。

### REQ-core-161: query の出力の形

- kind: ubiquitous
- source: docs/decision/records/2026-09-19-read-commands.md#A7, docs/decision/records/2026-09-20-query-status.md#A4, docs/decision/records/2026-09-20-query-status.md#A5, docs/decision/records/2026-09-20-query-status.md#A7, docs/decision/records/2026-09-20-query-status.md#A13
- verification: unit

kotowari は常に、"kotowari query" の "--format" の値として "json" と "text" の2つだけを受け、既定を "json" にする。"json" では最上位が "items" だけの JSON を1つ出し、"items" は `TBL-core-027` の鍵を持つ1件の並びである。"text" では1件ごとに、"kotowari list" の "text" と同じ1行目と "tests" の行を出し、その後に "body" の各行を2つの半角空白で字下げして出し、最後に "referenced_by" の1件ごとに "  <- ID via パス:行" の行を出す。

## Decision tables

### TBL-core-027: query の1件の鍵

- source: docs/decision/records/2026-09-20-query-status.md#A2, docs/decision/records/2026-09-20-query-status.md#A3, docs/decision/records/2026-09-20-query-status.md#A4, docs/decision/records/2026-09-20-query-status.md#A16, docs/decision/records/2026-09-23-ir-engine-gaps.md#A9, docs/decision/records/2026-09-23-ir-engine-gaps.md#A18, docs/decision/records/2026-09-23-ir-engine-gaps.md#A24, docs/decision/records/2026-09-23-ir-engine-gaps.md#A34, docs/decision/records/2026-09-23-ir-english-tokens.md#A2, docs/decision/records/2026-09-24-review6-gaps.md#A2

| 鍵 | 持つ種類 | 中身 |
|---|---|---|
| `TBL-core-026` のすべての鍵 | `TBL-core-026` のとおり | `TBL-core-026` のとおり |
| body | すべて | 本文の行の並び。`項目`は見出しの次の行から、スキーマの側が返すその`項目`の最後の行まで。最後の行は、次の、その`項目`の見出しと同じ深さかそれより浅い見出し（"### "、"## "、"# "）の前の行（無ければ文書の最後の行）で、コードブロックの中の見出しの形の行は数えず、kotowari は生の行から見出しを探さない。`シナリオ`は "@id" のタグの行から最後のステップの行まで。先頭と末尾の空の行は含めない。行の文字はそのまま |
| referenced_by | すべて | その `ID` を指している`項目`と`シナリオ`の並び。1つの`項目`か`シナリオ`が同じ `ID` を同じ via で何度指しても1件にする。1件は "id"、"kind"、"path"、"line"（`TBL-core-026` と同じ意味）と "via" |
| referenced_by の via | すべて | "definition"（"- definition:" の行）、"relations"（"- related:" の行）、"about"（"@about" のタグ）、"text"（`要求`の文、`性質`の文、`シナリオ`のステップの中の、二重引用符の外でバッククォートで囲んだ `ID`。REQ-core-054 と同じ判定）のいずれか |

## Examples

```gherkin
@id=EX-core-287 @about=TBL-core-027 @source=docs/decision/records/2026-09-24-review6-gaps.md#A2
Scenario: 同じ ID を2回指す項目は逆引きで1件
  Given 文の中で "REQ-002" を2回指す "REQ-001" がある
  When "kotowari query REQ-002" を実行する
  Then "referenced_by" に "REQ-001" の via が "text" の件が1件だけある

@id=EX-core-250 @about=REQ-core-156,REQ-core-159,TBL-core-027 @source=docs/decision/records/2026-09-20-query-status.md#A2,docs/decision/records/2026-09-20-query-status.md#A3,docs/decision/records/2026-09-20-query-status.md#A4,docs/decision/records/2026-09-20-query-status.md#A16,docs/decision/records/2026-09-20-query-status.md#A19,docs/decision/records/2026-09-23-ir-english-tokens.md#A2
Scenario: 1件に本文と逆引きが付く
  Given IR に名前が "例" の要求 "REQ-001" が "docs/ir/a.md" の 7 行目にあり、その下に "- kind: ubiquitous" と "- source:" と "- verification: unit" の行と文 "文。" がある
  And 同じ文書に "@about=REQ-001" のシナリオ "EX-001" が 20 行目にある
  When "kotowari query REQ-001" を実行する
  Then 終了コードは 0 で、"items" は "id" が "REQ-001" の1件で、その "body" は "- kind: ubiquitous" の行から "文。" の行までの行の並びで、その "referenced_by" は "id" が "EX-001"、"via" が "about"、"line" が 20 の1件である

@id=EX-core-251 @about=REQ-core-157 @source=docs/decision/records/2026-09-20-query-status.md#A6
Scenario: 無い ID は停止する
  Given IR に "REQ-999" を持つ項目もシナリオも無い
  When "kotowari query REQ-999" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は "argument error: unknown id: REQ-999" である

@id=EX-core-252 @about=REQ-core-157 @source=docs/decision/records/2026-09-20-query-status.md#A5
Scenario: 位置引数が2つなら停止する
  Given 検査できる IR がある
  When "kotowari query REQ-001 REQ-002" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は "argument error: " で始まる

@id=EX-core-253 @about=REQ-core-156 @source=docs/decision/records/2026-09-20-query-status.md#A6
Scenario: 重複した ID は全部出る
  Given IR に "REQ-001" の要求が "docs/ir/a.md" と "docs/ir/b.md" に1つずつある
  When "kotowari query REQ-001" を実行する
  Then 終了コードは 0 で、"items" は "path" が "docs/ir/a.md" と "docs/ir/b.md" の2件である

@id=EX-core-254 @about=REQ-core-161 @source=docs/decision/records/2026-09-19-read-commands.md#A7,docs/decision/records/2026-09-19-read-commands.md#A19,docs/decision/records/2026-09-20-query-status.md#A3,docs/decision/records/2026-09-20-query-status.md#A7,docs/decision/records/2026-09-20-query-status.md#A13,docs/decision/records/2026-09-23-ir-english-tokens.md#A2
Scenario: text は本文と逆引きの行を続ける
  Given EX-core-250 と同じ IR があり、"tests/a.rs" の 3 行目の印 "@kotowari[REQ-001]" の直後にテスト "req_001_x" がある
  When "kotowari query --format text REQ-001" を実行する
  Then 1行目は "REQ-001 unit 例 docs/ir/a.md:7 tests=1"、2行目は "  tests/a.rs:3 req_001_x"、3行目は "  - kind: ubiquitous"、最後の行は "  <- EX-001 about docs/ir/a.md:20" である

@id=EX-core-255 @about=TBL-core-027 @source=docs/decision/records/2026-09-20-query-status.md#A3,docs/decision/records/2026-09-20-query-status.md#A16
Scenario: シナリオの本文はタグの行から始まる
  Given IR に "@id=EX-001 @about=REQ-001" のタグの行が 19 行目、"Scenario:" の行が 20 行目、ステップが 21 行目から 23 行目にあるシナリオがある
  When "kotowari query EX-001" を実行する
  Then "items" の1件の "body" は 19 行目から 23 行目の5行で、その "referenced_by" は空である

@id=EX-core-256 @about=REQ-core-158 @source=docs/decision/records/2026-09-20-query-status.md#A19
Scenario: 設定が読めなければ check と同じく停止する
  Given 設定ファイルが YAML として読めない
  When "kotowari query REQ-001" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は "kotowari check" と同じ文言である

@id=EX-core-257 @about=TBL-core-027 @source=docs/decision/records/2026-09-20-query-status.md#A3,docs/decision/records/2026-09-23-ir-english-tokens.md#A2
Scenario: 定義と文の中の ID が逆引きに出る
  Given IR に決定表 "TBL-001" と、"- definition: TBL-001" の行を持つ要求 "REQ-001" と、文の中にバッククォートで囲んだ "TBL-001" を書いた要求 "REQ-002" がある
  When "kotowari query TBL-001" を実行する
  Then "items" の1件の "referenced_by" は "id" が "REQ-001" で "via" が "definition" の1件と、"id" が "REQ-002" で "via" が "text" の1件である

@id=EX-core-277 @about=REQ-core-159,TBL-core-027 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A18,docs/decision/records/2026-09-23-ir-engine-gaps.md#A24,docs/decision/records/2026-09-23-ir-english-tokens.md#A2
Scenario: 本文は次の見出しの前で終わり、末尾の空行を落とす
  Given IR の "## Requirements" の節に要求 "REQ-001" と "REQ-002" がこの順にあり、"REQ-002" の文の後に空行を挟んで "## Properties" の節が続き、その下の性質 "PROP-001" が文書の最後の項目である
  When "kotowari query REQ-001"、"kotowari query REQ-002"、"kotowari query PROP-001" を実行する
  Then "REQ-001" の "body" は "REQ-002" の見出しの前の行までで、"REQ-002" の見出しの行を含まない
  And "REQ-002" の "body" は文の行で終わり、末尾の空行と "## Properties" の行を含まない
  And "PROP-001" の "body" は文書の最後の空でない行で終わる

@id=EX-core-280 @about=REQ-core-159,TBL-core-027 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A34,docs/decision/records/2026-09-23-ir-engine-gaps.md#A24
Scenario: 本文のコードブロックの中の見出しの形の行では本文を切らない
  Given IR の要求 "REQ-001" の文の後に "```" のフェンスで囲んだブロックがあり、その中に "## 例" の行があり、ブロックの後に要求 "REQ-002" の見出しが続く
  When "kotowari query REQ-001" を実行する
  Then "REQ-001" の "body" はフェンスの閉じる行までを含み、"## 例" の行で終わらない
```
