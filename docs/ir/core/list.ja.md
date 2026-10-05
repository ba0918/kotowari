# 項目の一覧

[English](list.md) | 日本語

"kotowari list" が`IR`の`項目`と`シナリオ`と、それらを指す`印`のある`テスト`を、人と LLM が読める形で出すところを扱う。何を読むかは check と同じで、`指摘`は出さない。

## Requirements

### REQ-core-151: list の読み取り

- kind: ubiquitous
- source: docs/decision/records/2026-09-19-read-commands.md#A1, docs/decision/records/2026-09-19-read-commands.md#A2, docs/decision/records/2026-09-19-read-commands.md#A9, docs/decision/records/2026-09-19-read-commands.md#A21
- verification: unit

kotowari は常に、"kotowari list" で "kotowari check" と同じ設定と置き場から`IR`の文書と`テストのファイル`を読み、読めた`項目`と`シナリオ`をすべて標準出力に出し、`指摘`を出さず、終了コードを0にする。`IR`の文書に`誤り`があっても、読めた`項目`と`シナリオ`は出す。

### REQ-core-152: list の停止

- kind: event_driven
- source: docs/decision/records/2026-09-19-read-commands.md#A9, docs/decision/records/2026-09-19-read-commands.md#A20, docs/decision/records/2026-09-24-doc-marks.md#A22, docs/decision/records/2026-09-27-surface-check.md#A14, docs/decision/records/2026-09-27-surface-check.md#A24, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-05-localization.md#A37, docs/decision/records/2026-10-05-localization.md#A21
- verification: unit

"kotowari list" で、"kotowari check" が`停止`する条件（設定の誤り、読めないファイル、引数の誤り）が成り立つとき、kotowari は check と同じ理由と文言で`停止`する。 ただし "kotowari list" は`ガイド`を読まず、`ガイド`の置き場と読み込みによる`停止`（REQ-core-198、REQ-core-199）はしない。`面のファイル`と`面の規則`のファイルと`未記載の面の一覧`も読まず、その読み込みによる`停止`（REQ-core-229）はしない。`全体像の元データ`も読まず、その置き場と読み込みによる`停止`（REQ-core-278、REQ-core-280）はしない。ただし`言語の一覧`の言語が2つ以上のときは、"translations" を出すために`ガイド`、`全体像の元データ`、`目次`の置き場を check と同じに辿り、その置き場と読み込みによる`停止`を check と同じにして、中身を検査しない。

### REQ-core-153: 項目の形

- kind: algorithm
- source: docs/decision/records/2026-09-19-read-commands.md#A3, docs/decision/records/2026-09-19-read-commands.md#A6, docs/decision/records/2026-09-19-read-commands.md#A12, docs/decision/records/2026-09-19-read-commands.md#A13, docs/decision/records/2026-09-19-read-commands.md#A14, docs/decision/records/2026-09-19-read-commands.md#A18
- definition: TBL-core-026
- verification: unit

### REQ-core-154: 一覧の順

- kind: ubiquitous
- source: docs/decision/records/2026-09-19-read-commands.md#A12
- verification: unit

kotowari は常に、"items" の1件を "path" の昇順、同じ "path" の中は "line" の昇順に並べ、1件の "tests" も "path" の昇順、同じ "path" の中は "line" の昇順に並べる。

### REQ-core-155: 出力の形

- kind: ubiquitous
- source: docs/decision/records/2026-09-19-read-commands.md#A7, docs/decision/records/2026-09-19-read-commands.md#A8, docs/decision/records/2026-09-19-read-commands.md#A12, docs/decision/records/2026-09-19-read-commands.md#A19, docs/decision/records/2026-09-19-read-commands.md#A25, docs/decision/records/2026-09-23-ir-english-tokens.md#A2, docs/decision/records/2026-09-25-deferred-items.md#A9, docs/decision/records/2026-10-05-localization.md#A29, docs/decision/records/2026-10-05-localization.md#D2, docs/decision/records/2026-10-05-localization.md#A21, docs/decision/records/2026-10-05-localization.md#D1, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

kotowari は常に、"kotowari list" の "--format" の値として "json" と "text" の2つだけを受け、既定を "json" にし、絞り込みのオプションを持たない。"json" では最上位が "items" と、`言語の一覧`の言語が2つ以上のときだけ "translations" を持つ JSON を1つ出し、"items" は`TBL-core-026` の鍵を持つ1件の並びである。"translations" は`対`ごとの1件（`先頭の言語`の`側`の無い`対`も1件）の並びで、1件は "path"（`先頭の言語`の`側`の基準のディレクトリからの相対パス）と "sides"（`言語の一覧`の順に、"language"（言語タグ）、"path"（その`側`の基準のディレクトリからの相対パス）、"blob"（その`側`の blob hash。`側`が無ければ null）を持つ並び）を持ち、"path" の昇順に並ぶ。"text" では "items" の行の後に、"translations" の1件ごとに1行で、"path" に続けて "sides" の1件ごとに1つの半角空白と "言語タグ=blob hash"（無い`側`は "言語タグ=-"）を並べる。"text" では "items" の1件を1行で "ID 検証 名前 パス:行 tests=数" の形で出し（"検証" は要求以外と、"- verification:" の行の無い要求では "-"）、"deferred" が true の1件ではその行の末尾に " deferred" を付け、その直後に "tests" の1件ごとに2つの半角空白で字下げした "パス:行 名前" の行を続ける（"名前" が null のときは "-"）。

## Decision tables

### TBL-core-026: 項目の鍵

- source: docs/decision/records/2026-09-19-read-commands.md#A6, docs/decision/records/2026-09-19-read-commands.md#A13, docs/decision/records/2026-09-19-read-commands.md#A14, docs/decision/records/2026-09-19-read-commands.md#A18, docs/decision/records/2026-09-19-read-commands.md#A22, docs/decision/records/2026-09-19-read-commands.md#A24, docs/decision/records/2026-09-19-read-commands.md#A26, docs/decision/records/2026-09-23-ir-english-tokens.md#A2, docs/decision/records/2026-09-23-ir-english-tokens.md#A8, docs/decision/records/2026-09-24-review6-gaps.md#A1, docs/decision/records/2026-09-24-multi-language-tests.md#A13, docs/decision/records/2026-09-24-doc-marks.md#A14, docs/decision/records/2026-09-25-deferred-items.md#A9

| 鍵 | 持つ種類 | 中身 |
|---|---|---|
| id | すべて | `ID` |
| kind | すべて | "requirement"、"table"、"property"、"scenario"、"flag" のいずれか |
| name | すべて | 見出しの名前。`シナリオ`は "Scenario:" の後の文字から前後の半角空白とタブを除いたもの |
| path | すべて | `項目`のある文書の、基準のディレクトリからの相対パス |
| line | すべて | 見出しの行。`シナリオ`は "Scenario:" の行 |
| type | 要求、問題の記録 | "- kind:" の値。無ければ null |
| verification | 要求 | "- verification:" の値。無ければ null |
| definition | 要求 | "- definition:" の `ID` の並び。無ければ空の並び |
| examples | 要求、決定表、性質 | その `ID` を "@about" に持つ`シナリオ`の `ID` の並び。`ID` の昇順。同じ `ID` の`シナリオ`が2つ以上あるときは REQ-core-032 の1つ目だけを数える |
| how_to_verify | 要求 | "- how_to_verify:" の値。無ければ null |
| relations | 問題の記録 | "- related:" の `ID` の並び |
| sources | すべて | `出典`の並び |
| tests | すべて | その `ID` を`印`に含む`テスト`の並び。同じ`テスト`に同じ `ID` の`印`が複数あれば、`印`の出現ごとに1件。1件は "path"（`テストのファイル`の基準のディレクトリからの相対パス）、"line"（`印`のある行）、"name"（`テスト`の名前。`問い合わせの無い言語`と、名前が null の`テスト`では null） |
| fingerprint | すべて | その`項目`か`シナリオ`の`指紋`（REQ-core-203） |
| deferred | すべて | `後回し`の`要求`と`後回しのシナリオ`は true、ほかは false |

## Examples

```gherkin
@id=EX-core-288 @about=TBL-core-026 @source=docs/decision/records/2026-09-24-review6-gaps.md#A1
Scenario: 重複した ID のシナリオは1つ目だけを具体例に数える
  Given 同じ `ID` の`シナリオ`が2つあり、1つ目は "REQ-001" を、2つ目は "REQ-002" を "@about" に持つ
  When "kotowari list --format json" を実行する
  Then "REQ-001" の "examples" はその `ID` を持ち、"REQ-002" の "examples" は空である

@id=EX-core-245 @about=REQ-core-151,REQ-core-153,TBL-core-026 @source=docs/decision/records/2026-09-19-read-commands.md#A2,docs/decision/records/2026-09-19-read-commands.md#A6,docs/decision/records/2026-09-19-read-commands.md#A9,docs/decision/records/2026-09-19-read-commands.md#A18
Scenario: 要求と印のあるテストが1件ずつ出る
  Given IR に名前が "例" の要求 "REQ-001" が "docs/ir/a.md" の 7 行目にあり、検証が "unit" である
  And "tests/a.rs" の 3 行目の印 "@kotowari[REQ-001]" の直後にテスト "req_001_x" がある
  When "kotowari list" を実行する
  Then 終了コードは 0 で、"items" に "id" が "REQ-001"、"kind" が "requirement"、"verification" が "unit" の1件があり、その "tests" は "path" が "tests/a.rs"、"line" が 3、"name" が "req_001_x" の1件である

@id=EX-core-246 @about=REQ-core-151 @source=docs/decision/records/2026-09-19-read-commands.md#A9,docs/decision/records/2026-09-19-read-commands.md#A18,docs/decision/records/2026-09-23-ir-english-tokens.md#A2
Scenario: IR に誤りがあっても読めた項目は出る
  Given IR に "- verification:" の行の無い要求 "REQ-002" がある
  When "kotowari list" を実行する
  Then 終了コードは 0 で、"items" に "id" が "REQ-002"、"verification" が null の1件があり、出力に "findings" は無い

@id=EX-core-247 @about=REQ-core-153,TBL-core-026 @source=docs/decision/records/2026-09-19-read-commands.md#A13,docs/decision/records/2026-09-24-multi-language-tests.md#A7
Scenario: 問い合わせの無い言語のテストは名前が null
  Given "tests/a.go" の 2 行目に印 "@kotowari[REQ-001]" があり、".go" の`問い合わせ`は無い
  When "kotowari list" を実行する
  Then "REQ-001" の "tests" に "path" が "tests/a.go"、"line" が 2、"name" が null の1件がある

@id=EX-core-248 @about=REQ-core-155 @source=docs/decision/records/2026-09-19-read-commands.md#A7,docs/decision/records/2026-09-19-read-commands.md#A19
Scenario: text は1項目1行にテストの行を続ける
  Given EX-core-245 と同じ IR とテストがある
  When "kotowari list --format text" を実行する
  Then 1行目は "REQ-001 unit 例 docs/ir/a.md:7 tests=1" で、2行目は "  tests/a.rs:3 req_001_x" である

@id=EX-core-249 @about=REQ-core-152 @source=docs/decision/records/2026-09-19-read-commands.md#A9,docs/decision/records/2026-09-19-read-commands.md#A20
Scenario: 設定が読めなければ check と同じく停止する
  Given 設定ファイルが YAML として読めない
  When "kotowari list" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は "kotowari check" と同じ文言である

@id=EX-core-399 @about=TBL-core-026,REQ-core-155 @source=docs/decision/records/2026-09-25-deferred-items.md#A9,docs/decision/records/2026-09-25-deferred-items.md#A15,docs/decision/records/2026-09-19-read-commands.md#A19
Scenario: 後回しの要求とシナリオは deferred が付く
  Given "docs/ir/a.md" の 7 行目に名前が "例" で検証が "unit" の後回しの要求 "REQ-001" があり、"@about=REQ-001" のシナリオ "EX-001" と、後回しでない要求 "REQ-002" があり、どの ID を含む印も無い
  When "kotowari list" と "kotowari list --format text" を実行する
  Then json では "REQ-001" と "EX-001" の "deferred" が true、"REQ-002" の "deferred" が false で、text の "REQ-001" の行は "REQ-001 unit 例 docs/ir/a.md:7 tests=0 deferred" である
```
