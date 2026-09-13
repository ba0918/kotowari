# 使い方の表示と対象の環境

"--help" と "--version" の扱い、対象の OS、停止の理由の文言を扱う。

## 要求

### REQ-107: 使い方と版の表示

- 種類: event_driven
- 出典: experiments/003-cli/brainstorm/records.md#A103
- 検証: unit

"--help" か "--version" を受けたとき、kotowari は検査を行わず、使い方か版の文字列を標準出力に出して終了コード0で終わる。

### REQ-108: 対象の環境

- 種類: ubiquitous
- 出典: experiments/003-cli/brainstorm/records.md#A125
- 検証: review

kotowari は常に、Linux と macOS を対象にする。Windows ではパスの区切りの正規化（REQ-110）だけを行い、それ以外の動作を約束しない。

### REQ-119: 停止と指摘の振り分け

- 種類: invariant
- 出典: experiments/003-cli/brainstorm/records.md#A100, experiments/003-cli/brainstorm/records.md#A101
- 検証: review

読めない入力、壊れている入力、契約の形に合わない入力に対して、kotowari は`停止`か`誤り`の`指摘`のどちらかを必ず行い、黙って飛ばさない関係が常に成り立つ。ファイルや設定を全体として読む前提が崩れる入力（読めない、UTF-8 でない、設定の構文と型と値の誤り、引数の誤り、glob の構文の誤り）では`停止`し、読めたが局所的に形から外れる入力ではその場所への`誤り`の`指摘`を出す。読まないものは`除外`だけである。

## 決定表

### TBL-018: 停止の理由の文言

- 出典: experiments/003-cli/brainstorm/records.md#A104

| 理由 | 標準エラーの1行目の文言 |
|---|---|
| 設定の誤り | config error |
| 引数の誤り | argument error |
| 読めないファイル | unreadable file |
| UTF-8 でないファイル | non-UTF-8 file |
