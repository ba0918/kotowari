# 用語の書き方と用語集の表

二重引用符とバッククォートの書き分け、閉じないバッククォート、用語集の表の範囲を扱う。

## 要求

### REQ-104: 具体的な値は二重引用符で書く

- 種類: ubiquitous
- 出典: experiments/003-cli/brainstorm/records.md#A31
- 検証: unit

`IR`の`文`は常に、具体的な値を二重引用符で書き、バッククォートでは`用語`と`ID`だけを囲む。

### REQ-116: 閉じないバッククォート

- 種類: event_driven
- 出典: experiments/003-cli/brainstorm/records.md#A116
- 検証: unit

`対象の行`のバッククォートの数が奇数のとき、kotowari は行の文字を detail にして unclosed_backtick の`誤り`を出し、その行の`用語`の検査を行わない。

### REQ-117: 用語集の表の範囲

- 種類: event_driven
- 出典: experiments/003-cli/brainstorm/records.md#A112
- 検証: unit

`用語集`の表は "| 用語 | 意味 | 出典 |" のヘッダと区切りの行から始まり、空行か表でない行で終わる。`用語集`の文書があるのにこの形の表が無いとき、kotowari は文書名を detail にして glossary_invalid の`誤り`を出し、`用語`を0語として検査を続ける。
