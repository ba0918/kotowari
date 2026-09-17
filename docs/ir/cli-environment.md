# 使い方の表示と対象の環境

"--help" と "--version" の扱い、対象の OS、停止と指摘の振り分け、停止の理由と詳細の文言を扱う。

## 要求

### REQ-107: 使い方と版の表示

- 種類: event_driven
- 出典: docs/decision/records/records.md#A103, docs/decision/records/records.md#A136
- 検証: unit

"--help" か "--version" を受けたとき、kotowari はほかの引数を見ず、検査を行わず、使い方か版の文字列を標準出力に出して終了コード0で終わる。"check" が無くてもよい。

### REQ-108: 対象の環境

- 種類: ubiquitous
- 出典: docs/decision/records/records.md#A125
- 検証: review

kotowari は常に、Linux と macOS を対象にする。Windows ではパスの区切りの正規化（REQ-110）だけを行い、それ以外の動作を約束しない。

### REQ-109: 停止と指摘の振り分け

- 種類: invariant
- 出典: docs/decision/records/records.md#A100, docs/decision/records/records.md#A101, docs/decision/records/records.md#P2, docs/decision/records/records.md#A102
- 検証: review

読めない入力、壊れている入力、契約の形に合わない入力に対して、kotowari は`停止`か`誤り`の`指摘`のどちらかを必ず行い、黙って飛ばさない関係が常に成り立つ。ファイルや設定を全体として読む前提が崩れる入力（読めない、UTF-8 でない、設定の構文と型と値の誤り、引数の誤り、glob の構文の誤り）では`停止`し、読めたが局所的に形から外れる入力ではその場所への`誤り`の`指摘`を出す。読まないものは`除外`だけである。

### REQ-120: 黙って読み飛ばさない

- 種類: prohibition
- 出典: docs/decision/records/records.md#P2, docs/decision/records/records.md#A100
- 検証: review

kotowari は、`除外`に列挙していない入力を、`停止`も`指摘`もせずに読み飛ばしてはならない。

### REQ-121: 設定で問い合わせを足せない

- 種類: prohibition
- 出典: docs/decision/records/records.md#A128, docs/decision/records/records.md#A39, docs/decision/records/records.md#A58, docs/decision/records/records.md#A24, docs/decision/adr/0002-tree-sitter.md#理由, docs/decision/records/2026-09-17-check-reach.md#A3, docs/decision/records/2026-09-17-check-reach.md#A4, docs/decision/records/2026-09-17-check-reach.md#A22
- 検証: unit

kotowari は、`設定ファイル`で`問い合わせ`を足すことをしてはならない。Rust 以外の言語の`問い合わせ`は kotowari に`問い合わせ`のファイルを足すことで後から足す。

## 決定表

### TBL-018: 停止の理由の文言

- 出典: docs/decision/records/records.md#A104

| 理由 | 標準エラーの1行目の文言 |
|---|---|
| 設定の誤り | config error |
| 引数の誤り | argument error |
| 読めないファイル | unreadable file |
| UTF-8 でないファイル | non-UTF-8 file |

### TBL-020: 停止の詳細

- 出典: docs/decision/records/records.md#A137, docs/decision/records/records.md#A147, docs/decision/records/records.md#A160, docs/decision/records/records.md#A164

| 理由 | 詳細（英語） |
|---|---|
| 設定の誤り | 設定ファイルの基準のディレクトリからの相対パス（基準の外にあれば "../" を含む）と、誤りの説明 |
| 引数の誤り | 説明の文と、問題の引数の文字。引数が1つも無いときは "expected command: check" |
| 読めないファイル | 相対パスと、OS の誤りの文。カレントディレクトリを取得できないときは "current directory: " と OS の誤りの文 |
| UTF-8 でないファイル | 相対パス |
