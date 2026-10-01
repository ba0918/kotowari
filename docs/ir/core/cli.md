# コマンドと終了

kotowari のコマンド、受ける引数、停止と終了コードを扱う。

## Requirements

### REQ-core-001: コマンドは7つ

- kind: ubiquitous
- source: docs/decision/records/records.md#A19, docs/decision/records/2026-09-17-mutation-tests.md#A8, docs/decision/records/2026-09-17-mutation-tests.md#A41, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-19-read-commands.md#A1, docs/decision/records/2026-09-19-read-commands.md#A10, docs/decision/records/2026-09-20-query-status.md#A1, docs/decision/records/2026-09-20-query-status.md#A18, docs/decision/records/2026-09-24-plan-schema.md#A10, docs/decision/records/2026-10-01-change-conformance.md#A2
- verification: unit

kotowari は常に、"kotowari check"、"kotowari list"、"kotowari mutants"、"kotowari plan"、"kotowari query"、"kotowari status"、"kotowari changes" の7つのコマンドだけを持ち、"kotowari check" の1つのコマンドで`IR`の検査と`テスト`との対応の検査を両方行う。変更に対する照合の欠落と古さの検査は "kotowari changes" が行う（REQ-core-240）。

### REQ-core-002: 受けるオプション

- kind: ubiquitous
- source: docs/decision/records/records.md#A19, docs/decision/records/records.md#A103, docs/decision/records/2026-09-17-mutation-tests.md#A14, docs/decision/records/2026-09-17-mutation-tests.md#A41, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-17-mutation-tests.md#A58, docs/decision/records/2026-09-19-read-commands.md#A7, docs/decision/records/2026-09-19-read-commands.md#A20, docs/decision/records/2026-09-20-query-status.md#A5, docs/decision/records/2026-09-20-query-status.md#A18, docs/decision/records/2026-09-24-plan-schema.md#A15, docs/decision/records/2026-09-24-plan-schema.md#A16, docs/decision/records/2026-09-24-guide-gaps.md#A3
- verification: unit

kotowari は常に、"check"、"list"、"query"、"status" ではオプションとして "--format"、"--config"、"--help"、"--version" だけを受け、"mutants" ではそれに加えて "--tool" を受け、"plan" では "--format"、"--help"、"--version" だけを受け、どのコマンドでも、オプションをコマンドの前後どちらに書いても受け、位置引数とオプションの順を問わない。

### REQ-core-003: 設定のパスの基準

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A3, docs/decision/records/records.md#A60
- verification: unit

kotowari は常に、"--config" に与えたパスを、changes 以外ではカレントディレクトリからの相対パスとして読む。 "changes" は対象の snapshot 内の設定を Git のルートからの相対パスで読む（REQ-core-265）。

### REQ-core-004: 引数の誤り

- kind: event_driven
- source: docs/decision/records/2026-10-01-change-details.md#A3, docs/decision/records/records.md#A60, docs/decision/records/records.md#A103, docs/decision/records/records.md#A136, docs/decision/records/2026-09-17-mutation-tests.md#A41, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-17-mutation-tests.md#A55, docs/decision/records/2026-09-19-read-commands.md#A20, docs/decision/records/2026-09-20-query-status.md#A5, docs/decision/records/2026-09-20-query-status.md#A18, docs/decision/records/2026-09-24-plan-schema.md#A10, docs/decision/records/2026-09-24-plan-schema.md#A15, docs/decision/records/2026-10-01-change-conformance.md#A2
- verification: unit

"--help" も "--version" も無いときに、知らないオプション、"mutants" でないコマンドに付けた "--tool"、"check"、"list"、"mutants"、"plan"、"query"、"status"、"changes" のいずれでもない1つ目の位置引数、"check"、"list"、"status" の後の位置引数、"--format" の知らない値、値の無いオプション、同じオプションの2回目のいずれかを受けたとき、引数が1つも無いとき、オプションだけがあって1つ目の位置引数が無いとき、または "changes" 以外で "--config" の指す先が無いかディレクトリのとき、kotowari は引数の誤りを理由に`停止`する。

### REQ-core-149: mutants の引数

- kind: event_driven
- source: docs/decision/records/2026-09-17-mutation-tests.md#A14, docs/decision/records/2026-09-17-mutation-tests.md#A39, docs/decision/records/2026-09-17-mutation-tests.md#A41
- verification: unit

"--help" も "--version" も無い "kotowari mutants" で、"--tool" が無いとき、"--tool" の値が "cargo-mutants" でないとき、または "mutants" の後の位置引数がちょうど1つでないとき、kotowari は引数の誤りを理由に`停止`する。位置引数は結果のファイルのパスで、カレントディレクトリからの相対パスとして読む。

### REQ-core-005: 停止の出力

- kind: event_driven
- source: docs/decision/records/records.md#A40, docs/decision/records/records.md#A104, docs/decision/records/records.md#A137
- verification: unit

`停止`するとき、kotowari は標準出力に何も出さず、停止の理由を標準エラーに出す。標準エラーの1行目は TBL-core-018 の文言に ": " と詳細を続けた形で、詳細は TBL-core-020 のとおりで、パスを含めるときは`基準のディレクトリ`からの相対パスにし、文言は英語で書く。

### REQ-core-006: 停止の理由

- kind: algorithm
- source: docs/decision/records/records.md#A44, docs/decision/records/records.md#A48, docs/decision/records/records.md#A60, docs/decision/records/records.md#A104
- definition: TBL-core-001, TBL-core-018, TBL-core-020
- verification: unit

### REQ-core-007: 終了コード

- kind: algorithm
- source: docs/decision/records/records.md#A20, docs/decision/records/records.md#A29
- definition: TBL-core-002
- verification: unit

### REQ-core-008: 作らないコマンド

- kind: prohibition
- source: docs/decision/records/records.md#P1, docs/decision/records/records.md#A99, docs/decision/records/2026-09-17-check-reach.md#A3, docs/decision/records/2026-09-17-check-reach.md#A4, docs/decision/records/2026-09-17-check-reach.md#A23, docs/decision/records/2026-09-19-read-commands.md#A2, docs/decision/records/2026-09-20-query-status.md#A1
- verification: unit

kotowari は、人間向けの文書の生成（"render"）を作ってはならない。

## Decision tables

### TBL-core-001: 停止の理由

- source: docs/decision/records/records.md#A20, docs/decision/records/records.md#A44, docs/decision/records/records.md#A48, docs/decision/records/records.md#A60, docs/decision/records/records.md#A12, docs/decision/records/records.md#A41, docs/decision/records/records.md#A66, docs/decision/records/records.md#A95, docs/decision/records/records.md#A96, docs/decision/records/records.md#A93, docs/decision/records/records.md#A103, docs/decision/records/records.md#A105, docs/decision/records/records.md#A135, docs/decision/records/records.md#A136, docs/decision/records/records.md#A146, docs/decision/records/records.md#A160, docs/decision/records/2026-09-16-ir-tree.md#A16, docs/decision/records/2026-09-17-mutation-tests.md#A32, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-17-mutation-tests.md#A34, docs/decision/records/2026-09-17-mutation-tests.md#A49, docs/decision/records/2026-09-17-mutation-tests.md#A55, docs/decision/records/2026-09-17-mutation-tests.md#A39, docs/decision/records/2026-09-17-mutation-tests.md#A48, docs/decision/records/2026-09-22-ir-engine.md#A73, docs/decision/records/2026-09-24-plan-schema.md#A16, docs/decision/records/2026-09-24-doc-marks.md#A15, docs/decision/records/2026-09-24-doc-marks.md#A16, docs/decision/records/2026-09-24-guide-gaps.md#A2, docs/decision/records/2026-09-27-surface-check.md#A20, docs/decision/records/2026-09-27-surface-check.md#A22, docs/decision/records/2026-09-27-surface-check.md#A24, docs/decision/records/2026-09-27-surface-check.md#A29

| 理由 | 場面 |
|---|---|
| 設定の誤り | REQ-core-014 の場面と、REQ-core-189 の "tests.rules" の誤りの場面と、REQ-core-225 の面の設定と "surface.rules" の誤りの場面と、REQ-core-231 の未記載の面の一覧が YAML として読めないか最上位が並びでない場面と、REQ-core-148 の等価の一覧が YAML として読めないか最上位が並びでない場面と、REQ-core-199 のガイドとテストの置き場が重なる場面 |
| 引数の誤り | REQ-core-004、REQ-core-149、REQ-core-190 の場面 |
| 読めないファイル | "kotowari plan" の`計画書`のファイルが無いか、ディレクトリか、読めない（REQ-core-197）、読むファイルを読めない（"kotowari mutants" で変異の結果か等価の一覧の1件が指すファイルは除く。REQ-core-141、REQ-core-142）、結果のファイルが無いか読めない、"mutants.equivalents" の指す先が無いか読めない、"kotowari check" か "kotowari status" で "surface.unspecified" の指す先が無いか読めない、"kotowari check" で"ir"、"decisions.records"、"decisions.adr" の指すディレクトリが無いか読めない、"ir"、"decisions.records"、"decisions.adr" の下のディレクトリが読めない、または "tests.files"、"guides.files"、"surface.files" のいずれかの走査でディレクトリが読めない、または走査で先の無いシンボリックリンクに出会った、またはカレントディレクトリを取得できない |
| UTF-8 でないファイル | IR の文書、テストのファイル、ガイド、面のファイル（REQ-core-236 が読むもの）、未記載の面の一覧、設定ファイル、判断の記録、ADR、結果のファイル、等価の一覧、計画書のいずれかが UTF-8 でない |
| 結果の誤り | REQ-core-144 の場面 |
| 写しの誤り | REQ-core-175 の場面 |

### TBL-core-002: 終了コード

- source: docs/decision/records/records.md#A20, docs/decision/records/records.md#A29, docs/decision/records/records.md#A103, docs/decision/records/2026-09-16-notice.md#A2

| 終了コード | 場面 |
|---|---|
| 0 | 誤りが無い（注意だけのときを含む）、または "--help" か "--version" で終わった |
| 1 | 誤りが1件以上ある |
| 2 | 停止した |

## Examples

```gherkin
@id=EX-core-001 @about=REQ-core-004,REQ-core-005 @source=docs/decision/records/records.md#A60,docs/decision/records/records.md#A40
Scenario: 知らないオプションで停止する
  Given 検査できる IR がある
  When "kotowari check --verbose" を実行する
  Then 終了コードは 2 である
  And 標準出力には何も出ない
  And 標準エラーに停止の理由が出る

@id=EX-core-218 @about=REQ-core-149 @source=docs/decision/records/2026-09-17-mutation-tests.md#A14,docs/decision/records/2026-09-17-mutation-tests.md#A39,docs/decision/records/2026-09-17-mutation-tests.md#A41
Scenario: 道具の指定が無い mutants は停止する
  Given 読める結果のファイル "outcomes.json" がある
  When "kotowari mutants outcomes.json" を実行する
  Then 終了コードは 2 である
  And 標準エラーの1行目は "argument error: " で始まる

@id=EX-core-219 @about=REQ-core-004,TBL-core-020 @source=docs/decision/records/2026-09-17-mutation-tests.md#A39,docs/decision/records/2026-09-19-read-commands.md#A10,docs/decision/records/2026-09-19-read-commands.md#A20,docs/decision/records/2026-09-20-query-status.md#A18,docs/decision/records/2026-09-24-plan-schema.md#A10,docs/decision/records/2026-09-24-plan-schema.md#A30,docs/decision/records/2026-10-01-change-conformance.md#A2
Scenario: 引数が無いときはコマンドを挙げる
  When "kotowari" を引数なしで実行する
  Then 終了コードは 2 である
  And 標準エラーの理由には REQ-core-001 の7つのコマンド名が挙げられる

@id=EX-core-240 @about=REQ-core-149 @source=docs/decision/records/2026-09-17-mutation-tests.md#A14,docs/decision/records/2026-09-17-mutation-tests.md#A39
Scenario: 知らない道具の名前は停止する
  Given 読める結果のファイル "a.json" がある
  When "kotowari mutants --tool stryker a.json" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は "argument error: " で始まる

@id=EX-core-242 @about=REQ-core-149 @source=docs/decision/records/2026-09-17-mutation-tests.md#A39,docs/decision/records/2026-09-17-mutation-tests.md#A41
Scenario: 結果のファイルを2つ渡すと停止する
  Given 読める結果のファイル "a.json" と "b.json" がある
  When "kotowari mutants --tool cargo-mutants a.json b.json" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は "argument error: " で始まる

@id=EX-core-241 @about=REQ-core-004,TBL-core-020 @source=docs/decision/records/2026-09-17-mutation-tests.md#A39,docs/decision/records/2026-09-17-mutation-tests.md#A55,docs/decision/records/2026-09-19-read-commands.md#A10,docs/decision/records/2026-09-19-read-commands.md#A20,docs/decision/records/2026-09-20-query-status.md#A18,docs/decision/records/2026-09-24-plan-schema.md#A10,docs/decision/records/2026-09-24-plan-schema.md#A30,docs/decision/records/2026-10-01-change-conformance.md#A2
Scenario: オプションだけの実行はコマンドを挙げて停止する
  When "kotowari --format text" を実行する
  Then 終了コードは 2 である
  And 標準エラーの理由には REQ-core-001 の7つのコマンド名が挙げられる

@id=EX-core-244 @about=REQ-core-002 @source=docs/decision/records/2026-09-17-mutation-tests.md#A41,docs/decision/records/2026-09-17-mutation-tests.md#A58
Scenario: mutants のオプションはコマンドの前にも結果のパスの後にも書ける
  Given 結果のファイル "outcomes.json" に "summary" が "CaughtMutant" の1件がある
  When "kotowari --tool cargo-mutants mutants outcomes.json --format text" を実行する
  Then 終了コードは 0 である
@id=EX-core-380 @about=REQ-core-002 @source=docs/decision/records/2026-09-24-guide-gaps.md#A3,docs/decision/records/ir-form.md#出力,docs/decision/records/2026-09-20-query-status.md#A7
Scenario: query の ID の後ろにオプションを書ける
  Given `IR`に "REQ-001" がある
  When "kotowari query REQ-001 --format text" を実行する
  Then 終了コードは 0 である
```
