# 用語の書き方と用語集の表

二重引用符とバッククォートの書き分け、閉じないバッククォート、用語集の表の範囲を扱う。

## 要求

### REQ-104: 具体的な値は二重引用符で書く

- 種類: ubiquitous
- 出典: docs/decision/brainstorm/records.md#A31
- 検証: unit

`IR`の`文`は常に、具体的な値を二重引用符で書き、バッククォートでは`用語`と`ID`だけを囲む。

### REQ-116: 閉じないバッククォート

- 種類: event_driven
- 出典: docs/decision/brainstorm/records.md#A116, docs/decision/brainstorm/records.md#A138, docs/decision/brainstorm/records.md#A145
- 検証: unit

`対象の行`の二重引用符の外のバッククォートの数が奇数のとき、kotowari は行の文字を detail にして unclosed_backtick の`誤り`を出し、その行では`用語`と`ID`の参照の検査を行わず、`曖昧語`の検査は行う。

### REQ-117: 用語集の表の範囲

- 種類: event_driven
- 出典: docs/decision/brainstorm/records.md#A112, docs/decision/brainstorm/records.md#A141, docs/decision/brainstorm/records.md#A148
- 検証: unit

`用語集`の表は、各セルの前後の空白を除いて "用語"、"意味"、"出典" の3列と一致するヘッダの行と、各セルが3つ以上の "-"（前後に ":" があってもよい）の区切りの行から始まり、空行か表でない行で終わる。表の外の "|" で始まる行は`用語`にしない。`用語集`の文書があるのにこの形のヘッダと区切りの行が無いとき、kotowari は文書名を detail にして glossary_invalid の`誤り`を出し、`用語`を0語として検査を続ける。ヘッダと区切りの行があれば、`用語`の行が0でも表はあるものとして扱う。

### REQ-122: 用語集の表の崩れた行

- 種類: event_driven
- 出典: docs/decision/brainstorm/records.md#A153, docs/decision/brainstorm/records.md#A163
- 検証: unit

`用語集`の表の中に、セル（行の先頭と末尾の "|" を除いて "|" で分けたもの）が3つ未満の行か`用語`のセルが空の行があるとき、kotowari は行の文字を detail にして invalid_glossary_row の`誤り`を出し、その行を`用語`にしない。

### REQ-123: 用語の重複

- 種類: event_driven
- 出典: docs/decision/brainstorm/records.md#A154, docs/decision/brainstorm/records.md#A162
- 検証: unit

`用語集`に同じ`用語`の行が2つ以上あるとき、kotowari は2つ目以降の行ごとに`用語`を detail にして duplicate_term の`誤り`を出し、照合には1つ目を使う。2つ目以降の行は`用語`にしない。
