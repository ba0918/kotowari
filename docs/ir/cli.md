# コマンドと終了

kotowari のコマンド、受ける引数、停止と終了コードを扱う。

## 要求

### REQ-001: コマンドは1つ

- 種類: ubiquitous
- 出典: brainstorm/records.md#A19
- 検証: unit

kotowari は常に、"kotowari check" の1つのコマンドで、`IR`の検査と`テスト`との対応の検査を両方行う。

### REQ-002: 受けるオプション

- 種類: ubiquitous
- 出典: brainstorm/records.md#A19
- 検証: unit

kotowari は常に、オプションとして "--format" と "--config" だけを受ける。

### REQ-003: 設定のパスの基準

- 種類: ubiquitous
- 出典: brainstorm/records.md#A60
- 検証: unit

kotowari は常に、"--config" に与えたパスをカレントディレクトリからの相対パスとして読む。

### REQ-004: 引数の誤り

- 種類: event_driven
- 出典: brainstorm/records.md#A60
- 検証: unit

知らないオプション、位置引数、"--format" の知らない値のいずれかを受けたとき、または "--config" の指すファイルが無いとき、kotowari は`停止`する。

### REQ-005: 停止の出力

- 種類: event_driven
- 出典: brainstorm/records.md#A40
- 検証: unit

`停止`するとき、kotowari は標準出力に何も出さず、停止の理由を標準エラーに出す。

### REQ-006: 停止の理由

- 種類: algorithm
- 出典: brainstorm/records.md#A44, brainstorm/records.md#A48, brainstorm/records.md#A60
- 定義: TBL-001
- 検証: unit

### REQ-007: 終了コード

- 種類: algorithm
- 出典: brainstorm/records.md#A20, brainstorm/records.md#A29
- 定義: TBL-002
- 検証: unit

### REQ-008: 作らないコマンド

- 種類: prohibition
- 出典: brainstorm/records.md#P1
- 検証: review

kotowari は、人間向けの文書の生成（"render"）、影響範囲の追跡（"trace"）、plan と cycle への受け渡し（"query"）を作ってはならない。

## 決定表

### TBL-001: 停止の理由

- 出典: brainstorm/records.md#A20, brainstorm/records.md#A44, brainstorm/records.md#A48, brainstorm/records.md#A60, brainstorm/records.md#A12, brainstorm/records.md#A41, brainstorm/records.md#A66

| 理由 | 場面 |
|---|---|
| 設定の誤り | REQ-014 の場面 |
| 引数の誤り | REQ-004 の場面 |
| 読めないファイル | 読むファイルを読めない、または "ir"、"decisions.records"、"decisions.adr" の指すディレクトリが無い |
| UTF-8 でないファイル | IR の文書、テストのファイル、設定ファイル、判断の記録、ADR のいずれかが UTF-8 でない |

### TBL-002: 終了コード

- 出典: brainstorm/records.md#A20, brainstorm/records.md#A29

| 終了コード | 場面 |
|---|---|
| 0 | 誤りが無い（警告だけのときを含む） |
| 1 | 誤りが1件以上ある |
| 2 | 停止した |

## 具体例

```gherkin
@id=EX-001 @about=REQ-004,REQ-005 @source=brainstorm/records.md#A60,brainstorm/records.md#A40
Scenario: 知らないオプションで停止する
  Given 検査できる IR がある
  When "kotowari check --verbose" を実行する
  Then 終了コードは 2 である
  And 標準出力には何も出ない
  And 標準エラーに停止の理由が出る
```
