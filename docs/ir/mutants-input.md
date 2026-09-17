# 変異テストの結果の読み取り

"kotowari mutants" が変異テストの道具の結果のファイルを`変異の結果`に写すところと、写せない結果のファイルでの`停止`を扱う。`指摘`と集計は mutants.md で扱う。

## 要求

### REQ-138: 道具の結果を変異の結果に写す

- 種類: algorithm
- 出典: docs/decision/records/2026-09-17-mutation-tests.md#A1, docs/decision/records/2026-09-17-mutation-tests.md#A6, docs/decision/records/2026-09-17-mutation-tests.md#A13, docs/decision/records/2026-09-17-mutation-tests.md#A14, docs/decision/records/2026-09-17-mutation-tests.md#A42, docs/decision/records/2026-09-17-mutation-tests.md#A50, docs/decision/records/2026-09-17-mutation-tests.md#A51, docs/decision/records/2026-09-17-mutation-tests.md#A56
- 定義: TBL-024
- 検証: unit

### REQ-144: 結果の誤り

- 種類: event_driven
- 出典: docs/decision/records/2026-09-17-mutation-tests.md#A32, docs/decision/records/2026-09-17-mutation-tests.md#A39, docs/decision/records/2026-09-17-mutation-tests.md#A42, docs/decision/records/2026-09-17-mutation-tests.md#A50, docs/decision/records/2026-09-17-mutation-tests.md#A51
- 検証: unit

結果のファイルが JSON として読めないとき、TBL-024 の写す元の鍵が無いか型が違うとき、1件の結果が TBL-024 のどの行にも当たらないとき、基準の実行の結果が TBL-024 の「基準の実行の成功」でないとき、行が1未満のとき、変更の説明の前置きが TBL-024 の形でないとき、またはファイルが絶対パスか ".." の要素を含むとき、kotowari は結果の誤りを理由に`停止`する。基準の実行が1件も無い結果のファイルでは`停止`しない。TBL-024 に挙げていない鍵は見ない。結果のファイルが無い、読めない、UTF-8 でないときは、読めないファイルか UTF-8 でないファイルを理由に`停止`する。

## 決定表

### TBL-024: cargo-mutants の結果からの写し方

- 出典: docs/decision/records/2026-09-17-mutation-tests.md#A6, docs/decision/records/2026-09-17-mutation-tests.md#A13, docs/decision/records/2026-09-17-mutation-tests.md#A33, docs/decision/records/2026-09-17-mutation-tests.md#A42, docs/decision/records/2026-09-17-mutation-tests.md#A50, docs/decision/records/2026-09-17-mutation-tests.md#A51, docs/decision/records/2026-09-17-mutation-tests.md#A56

"--tool" が "cargo-mutants" のとき、結果のファイルは最上位の "outcomes" の鍵に並びを持つ JSON で、並びの1件ごとに次のとおりに写す。"scenario" が文字列 "Baseline" の1件は基準の実行で、変異の結果にしない。それ以外の1件は "scenario" の下の "Mutant" に変異を持つ。

| 変異の結果の項目 | 写す元 |
|---|---|
| ファイル | "scenario.Mutant.file"（文字列）。基準のディレクトリからの相対パスとして読み、REQ-110 の正規化を掛ける |
| 行 | "scenario.Mutant.span.start.line"（数） |
| 変更の説明 | "scenario.Mutant.name"（文字列）の先頭から、"scenario.Mutant.file" の値、":"、"span.start.line" の値、":"、"span.start.column" の値、": " をこの順につないだ前置きを除いた残り |
| 結果「捕まえた」 | "summary" が "CaughtMutant" |
| 結果「見逃した」 | "summary" が "MissedMutant" |
| 結果「時間切れ」 | "summary" が "Timeout" |
| 結果「ビルド不能」 | "summary" が "Unviable" |
| 基準の実行の成功 | 基準の実行の1件の "summary" が "Success" |

## 具体例

```gherkin
@id=EX-207 @about=REQ-144 @source=docs/decision/records/2026-09-17-mutation-tests.md#A32,docs/decision/records/2026-09-17-mutation-tests.md#A39,docs/decision/records/2026-09-17-mutation-tests.md#A42
Scenario: 知らない結果の値で停止する
  Given 結果のファイルに "summary" が "Flaky" の1件がある
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then 終了コードは 2 である
  And 標準エラーの1行目は "results error: " で始まる

@id=EX-208 @about=REQ-144 @source=docs/decision/records/2026-09-17-mutation-tests.md#A32,docs/decision/records/2026-09-17-mutation-tests.md#A39,docs/decision/records/2026-09-17-mutation-tests.md#A42
Scenario: 基準の実行が失敗した結果で停止する
  Given 結果のファイルの "scenario" が "Baseline" の1件の "summary" が "Failure" である
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then 終了コードは 2 である
  And 標準エラーの1行目は "results error: " で始まる

@id=EX-222 @about=REQ-144 @source=docs/decision/records/2026-09-17-mutation-tests.md#A32,docs/decision/records/2026-09-17-mutation-tests.md#A39
Scenario: JSON として読めない結果で停止する
  Given 結果のファイルの中身が "{" だけである
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then 終了コードは 2 である
  And 標準エラーの1行目は "results error: " で始まる

@id=EX-223 @about=REQ-144,TBL-024 @source=docs/decision/records/2026-09-17-mutation-tests.md#A32,docs/decision/records/2026-09-17-mutation-tests.md#A39,docs/decision/records/2026-09-17-mutation-tests.md#A50
Scenario: 要る鍵の型が違う結果で停止する
  Given 結果のファイルの変異の1件の "span.start.line" が文字列 "3" である
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then 終了コードは 2 である
  And 標準エラーの1行目は "results error: " で始まる

@id=EX-224 @about=REQ-144,TBL-024 @source=docs/decision/records/2026-09-17-mutation-tests.md#A39,docs/decision/records/2026-09-17-mutation-tests.md#A50
Scenario: 名前の前置きが形に合わない結果で停止する
  Given 結果のファイルの変異の1件の "file" が "src/a.rs"、"span.start" の "line" が 3 で "column" が 5、"name" が "replace f with ()" である
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then 終了コードは 2 である
  And 標準エラーの1行目は "results error: " で始まる

@id=EX-225 @about=REQ-144 @source=docs/decision/records/2026-09-17-mutation-tests.md#A39,docs/decision/records/2026-09-17-mutation-tests.md#A50
Scenario: 行が0の結果で停止する
  Given 結果のファイルの変異の1件の "span.start.line" が 0 である
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then 終了コードは 2 である
  And 標準エラーの1行目は "results error: " で始まる

@id=EX-226 @about=REQ-144 @source=docs/decision/records/2026-09-17-mutation-tests.md#A39,docs/decision/records/2026-09-17-mutation-tests.md#A51
Scenario: 基準のディレクトリの外を指す結果で停止する
  Given 結果のファイルの変異の1件の "file" が "../x/src/a.rs" である
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then 終了コードは 2 である
  And 標準エラーの1行目は "results error: " で始まる

@id=EX-227 @about=REQ-144 @source=docs/decision/records/2026-09-17-mutation-tests.md#A35,docs/decision/records/2026-09-17-mutation-tests.md#A38,docs/decision/records/2026-09-17-mutation-tests.md#A50
Scenario: 基準の実行が無く知らない鍵のある結果も読める
  Given 結果のファイルの "outcomes" に基準の実行が無く、"summary" が "CaughtMutant" の1件に TBL-024 に無い鍵 "extra" がある
  When "kotowari mutants --tool cargo-mutants outcomes.json" を実行する
  Then 終了コードは 0 で、"mutants" の "caught" は 1 になる
```
