# 使い手と作り

kotowari を使う者、書き出す先、コードの置き場を扱う。

## 要求

### REQ-101: 使い手

- 種類: ubiquitous
- 出典: docs/decision/brainstorm/records.md#A1, docs/decision/brainstorm/records.md#A98
- 検証: review

kotowari は常に、第一に LLM が使う CLI であり、人間が確認のために実行することもある。使う場は、仕様駆動の流れ（brainstorm、`判断の記録`、`IR`、実装）を回す開発者のリポジトリである。

### REQ-102: 状態を保存しない

- 種類: prohibition
- 出典: docs/decision/brainstorm/records.md#A75, docs/decision/brainstorm/2026-09-17-check-reach.md#A3, docs/decision/brainstorm/2026-09-17-check-reach.md#A4
- 検証: unit

kotowari は、状態を保存すること、標準出力と標準エラーのほかに書き出すことをしてはならない。

### REQ-105: crate と CLI の置き場

- 種類: ubiquitous
- 出典: docs/decision/brainstorm/records.md#A8
- 検証: review

kotowari のコードは常に、層が増えるたびに crate を足し、CLI を直下の "src/" で管理する。
