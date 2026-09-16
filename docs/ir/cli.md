# コマンドと終了

kotowari のコマンド、受ける引数、停止と終了コードを扱う。

## 要求

### REQ-001: コマンドは1つ

- 種類: ubiquitous
- 出典: docs/decision/brainstorm/records.md#A19
- 検証: unit

kotowari は常に、"kotowari check" の1つのコマンドで、`IR`の検査と`テスト`との対応の検査を両方行う。

### REQ-002: 受けるオプション

- 種類: ubiquitous
- 出典: docs/decision/brainstorm/records.md#A19, docs/decision/brainstorm/records.md#A103
- 検証: unit

kotowari は常に、オプションとして "--format"、"--config"、"--help"、"--version" だけを受け、オプションを "check" の前後どちらに書いても受ける。

### REQ-003: 設定のパスの基準

- 種類: ubiquitous
- 出典: docs/decision/brainstorm/records.md#A60
- 検証: unit

kotowari は常に、"--config" に与えたパスをカレントディレクトリからの相対パスとして読む。

### REQ-004: 引数の誤り

- 種類: event_driven
- 出典: docs/decision/brainstorm/records.md#A60, docs/decision/brainstorm/records.md#A103, docs/decision/brainstorm/records.md#A136
- 検証: unit

"--help" も "--version" も無いときに、知らないオプション、"check" 以外の位置引数、"--format" の知らない値、値の無いオプション、同じオプションの2回目のいずれかを受けたとき、引数が1つも無いとき、または "--config" の指す先が無いかディレクトリのとき、kotowari は引数の誤りを理由に`停止`する。

### REQ-005: 停止の出力

- 種類: event_driven
- 出典: docs/decision/brainstorm/records.md#A40, docs/decision/brainstorm/records.md#A104, docs/decision/brainstorm/records.md#A137
- 検証: unit

`停止`するとき、kotowari は標準出力に何も出さず、停止の理由を標準エラーに出す。標準エラーの1行目は TBL-018 の文言に ": " と詳細を続けた形で、詳細は TBL-020 のとおりで、パスを含めるときは`基準のディレクトリ`からの相対パスにし、文言は英語で書く。

### REQ-006: 停止の理由

- 種類: algorithm
- 出典: docs/decision/brainstorm/records.md#A44, docs/decision/brainstorm/records.md#A48, docs/decision/brainstorm/records.md#A60, docs/decision/brainstorm/records.md#A104
- 定義: TBL-001, TBL-018, TBL-020
- 検証: unit

### REQ-007: 終了コード

- 種類: algorithm
- 出典: docs/decision/brainstorm/records.md#A20, docs/decision/brainstorm/records.md#A29
- 定義: TBL-002
- 検証: unit

### REQ-008: 作らないコマンド

- 種類: prohibition
- 出典: docs/decision/brainstorm/records.md#P1, docs/decision/brainstorm/records.md#A99
- 検証: review

kotowari は、人間向けの文書の生成（"render"）、影響範囲の追跡（"trace"）、plan と cycle への受け渡し（"query"）を作ってはならない。

## 決定表

### TBL-001: 停止の理由

- 出典: docs/decision/brainstorm/records.md#A20, docs/decision/brainstorm/records.md#A44, docs/decision/brainstorm/records.md#A48, docs/decision/brainstorm/records.md#A60, docs/decision/brainstorm/records.md#A12, docs/decision/brainstorm/records.md#A41, docs/decision/brainstorm/records.md#A66, docs/decision/brainstorm/records.md#A95, docs/decision/brainstorm/records.md#A96, docs/decision/brainstorm/records.md#A93, docs/decision/brainstorm/records.md#A103, docs/decision/brainstorm/records.md#A105, docs/decision/brainstorm/records.md#A135, docs/decision/brainstorm/records.md#A136, docs/decision/brainstorm/records.md#A146, docs/decision/brainstorm/records.md#A160, docs/decision/brainstorm/2026-09-16-ir-tree.md#A16

| 理由 | 場面 |
|---|---|
| 設定の誤り | REQ-014 の場面 |
| 引数の誤り | REQ-004 の場面 |
| 読めないファイル | 読むファイルを読めない、"ir"、"decisions.records"、"decisions.adr" の指すディレクトリが無いか読めない、"ir"、"decisions.records"、"decisions.adr" の下のディレクトリが読めない、または "tests.files" の走査でディレクトリが読めない、または走査で先の無いシンボリックリンクに出会った、またはカレントディレクトリを取得できない |
| UTF-8 でないファイル | IR の文書、テストのファイル、設定ファイル、判断の記録、ADR のいずれかが UTF-8 でない |

### TBL-002: 終了コード

- 出典: docs/decision/brainstorm/records.md#A20, docs/decision/brainstorm/records.md#A29, docs/decision/brainstorm/records.md#A103

| 終了コード | 場面 |
|---|---|
| 0 | 誤りが無い（注意だけのときを含む）、または "--help" か "--version" で終わった |
| 1 | 誤りが1件以上ある |
| 2 | 停止した |

## 具体例

```gherkin
@id=EX-001 @about=REQ-004,REQ-005 @source=docs/decision/brainstorm/records.md#A60,docs/decision/brainstorm/records.md#A40
Scenario: 知らないオプションで停止する
  Given 検査できる IR がある
  When "kotowari check --verbose" を実行する
  Then 終了コードは 2 である
  And 標準出力には何も出ない
  And 標準エラーに停止の理由が出る
```
