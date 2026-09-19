# 項目の一覧

"kotowari list" が`IR`の`項目`と`シナリオ`と、それらを指す`印`のある`テスト`を、人と LLM が読める形で出すところを扱う。何を読むかは check と同じで、`指摘`は出さない。

## 要求

### REQ-151: list の読み取り

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-19-read-commands.md#A1, docs/decision/records/2026-09-19-read-commands.md#A2, docs/decision/records/2026-09-19-read-commands.md#A9, docs/decision/records/2026-09-19-read-commands.md#A21
- 検証: unit

kotowari は常に、"kotowari list" で "kotowari check" と同じ設定と置き場から`IR`の文書と`テストのファイル`を読み、読めた`項目`と`シナリオ`をすべて標準出力に出し、`指摘`を出さず、終了コードを0にする。`IR`の文書に`誤り`があっても、読めた`項目`と`シナリオ`は出す。

### REQ-152: list の停止

- 種類: event_driven
- 出典: docs/decision/records/2026-09-19-read-commands.md#A9, docs/decision/records/2026-09-19-read-commands.md#A20
- 検証: unit

"kotowari list" で、"kotowari check" が`停止`する条件（設定の誤り、読めないファイル、引数の誤り）が成り立つとき、kotowari は check と同じ理由と文言で`停止`する。

### REQ-153: 項目の形

- 種類: algorithm
- 出典: docs/decision/records/2026-09-19-read-commands.md#A3, docs/decision/records/2026-09-19-read-commands.md#A6, docs/decision/records/2026-09-19-read-commands.md#A12, docs/decision/records/2026-09-19-read-commands.md#A13, docs/decision/records/2026-09-19-read-commands.md#A14, docs/decision/records/2026-09-19-read-commands.md#A18
- 定義: TBL-026
- 検証: unit

### REQ-154: 一覧の順

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-19-read-commands.md#A12
- 検証: unit

kotowari は常に、"items" の1件を "path" の昇順、同じ "path" の中は "line" の昇順に並べ、1件の "tests" も "path" の昇順、同じ "path" の中は "line" の昇順に並べる。

### REQ-155: 出力の形

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-19-read-commands.md#A7, docs/decision/records/2026-09-19-read-commands.md#A8, docs/decision/records/2026-09-19-read-commands.md#A12, docs/decision/records/2026-09-19-read-commands.md#A19
- 検証: unit

kotowari は常に、"kotowari list" の "--format" の値として "json" と "text" の2つだけを受け、既定を "json" にし、絞り込みのオプションを持たない。"json" では最上位が "items" だけの JSON を1つ出し、"items" は`TBL-026` の鍵を持つ1件の並びである。"text" では "items" の1件を1行で "ID 検証 名前 パス:行 tests=数" の形で出し（"検証" は要求以外では "-"）、その直後に "tests" の1件ごとに2つの半角空白で字下げした "パス:行 名前" の行を続ける（"名前" が null のときは "-"）。

## 決定表

### TBL-026: 項目の鍵

- 出典: docs/decision/records/2026-09-19-read-commands.md#A6, docs/decision/records/2026-09-19-read-commands.md#A13, docs/decision/records/2026-09-19-read-commands.md#A14, docs/decision/records/2026-09-19-read-commands.md#A18, docs/decision/records/2026-09-19-read-commands.md#A22, docs/decision/records/2026-09-19-read-commands.md#A24

| 鍵 | 持つ種類 | 中身 |
|---|---|---|
| id | すべて | `ID` |
| kind | すべて | "requirement"、"table"、"property"、"scenario"、"flag" のいずれか |
| name | すべて | 見出しの名前。`シナリオ`は "Scenario:" の後の文字から前後の半角空白とタブを除いたもの |
| path | すべて | `項目`のある文書の、基準のディレクトリからの相対パス |
| line | すべて | 見出しの行。`シナリオ`は "Scenario:" の行 |
| type | 要求、問題の記録 | "- 種類:" の値。無ければ null |
| verification | 要求 | "- 検証:" の値。無ければ null |
| definition | 要求 | "- 定義:" の `ID` の並び。無ければ空の並び |
| examples | 要求、決定表、性質 | その `ID` を "@about" に持つ`シナリオ`の `ID` の並び |
| how_to_verify | 要求 | "- 確かめ方:" の値。無ければ null |
| relations | 問題の記録 | "- 関係:" の `ID` の並び |
| sources | すべて | `出典`の並び |
| tests | すべて | その `ID` を`印`に含む`テスト`の並び。同じ`テスト`に同じ `ID` の`印`が複数あれば、`印`の出現ごとに1件。1件は "path"（`テストのファイル`の基準のディレクトリからの相対パス）、"line"（`印`のある行）、"name"（`テスト`の関数の名前。`問い合わせの無い言語`では null） |

## 具体例

```gherkin
@id=EX-245 @about=REQ-151,REQ-153,TBL-026 @source=docs/decision/records/2026-09-19-read-commands.md#A2,docs/decision/records/2026-09-19-read-commands.md#A6,docs/decision/records/2026-09-19-read-commands.md#A9,docs/decision/records/2026-09-19-read-commands.md#A18
Scenario: 要求と印のあるテストが1件ずつ出る
  Given IR に名前が "例" の要求 "REQ-001" が "docs/ir/a.md" の 7 行目にあり、検証が "unit" である
  And "tests/a.rs" の 3 行目の印 "@kotowari[REQ-001]" の直後にテスト "req_001_x" がある
  When "kotowari list" を実行する
  Then 終了コードは 0 で、"items" に "id" が "REQ-001"、"kind" が "requirement"、"verification" が "unit" の1件があり、その "tests" は "path" が "tests/a.rs"、"line" が 3、"name" が "req_001_x" の1件である

@id=EX-246 @about=REQ-151 @source=docs/decision/records/2026-09-19-read-commands.md#A9,docs/decision/records/2026-09-19-read-commands.md#A18
Scenario: IR に誤りがあっても読めた項目は出る
  Given IR に "- 検証:" の行の無い要求 "REQ-002" がある
  When "kotowari list" を実行する
  Then 終了コードは 0 で、"items" に "id" が "REQ-002"、"verification" が null の1件があり、出力に "findings" は無い

@id=EX-247 @about=REQ-153,TBL-026 @source=docs/decision/records/2026-09-19-read-commands.md#A13
Scenario: 問い合わせの無い言語のテストは名前が null
  Given "tests/a.py" の 2 行目に印 "@kotowari[REQ-001]" がある
  When "kotowari list" を実行する
  Then "REQ-001" の "tests" に "path" が "tests/a.py"、"line" が 2、"name" が null の1件がある

@id=EX-248 @about=REQ-155 @source=docs/decision/records/2026-09-19-read-commands.md#A7,docs/decision/records/2026-09-19-read-commands.md#A19
Scenario: text は1項目1行にテストの行を続ける
  Given EX-245 と同じ IR とテストがある
  When "kotowari list --format text" を実行する
  Then 1行目は "REQ-001 unit 例 docs/ir/a.md:7 tests=1" で、2行目は "  tests/a.rs:3 req_001_x" である

@id=EX-249 @about=REQ-152 @source=docs/decision/records/2026-09-19-read-commands.md#A9,docs/decision/records/2026-09-19-read-commands.md#A20
Scenario: 設定が読めなければ check と同じく停止する
  Given 設定ファイルが YAML として読めない
  When "kotowari list" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は "kotowari check" と同じ文言である
```
