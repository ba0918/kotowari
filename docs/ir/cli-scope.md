# 使い手と作り

kotowari を使う者、書き出す先、コードの置き場を扱う。

## 要求

### REQ-101: 使い手

- 種類: ubiquitous
- 出典: brainstorm/records.md#A1
- 検証: review

kotowari は常に、第一に LLM が使う CLI であり、人間が確認のために実行することもある。

### REQ-102: 状態を保存しない

- 種類: prohibition
- 出典: brainstorm/records.md#A75
- 検証: review

kotowari は、状態を保存すること、標準出力と標準エラーのほかに書き出すことをしてはならない。

### REQ-105: crate と CLI の置き場

- 種類: ubiquitous
- 出典: brainstorm/records.md#A8
- 検証: review

kotowari のコードは常に、層が増えるたびに crate を足し、CLI を直下の "src/" で管理する。
