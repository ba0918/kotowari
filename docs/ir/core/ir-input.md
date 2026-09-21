# 文字コードとコードブロックの境界

読むファイルの先頭の BOM と、閉じないコードブロックの扱いを扱う。

## 要求

### REQ-core-111: 先頭の BOM

- 種類: event_driven
- 出典: docs/decision/records/records.md#A107
- 検証: unit

読むファイルの先頭に UTF-8 の BOM があるとき、kotowari はそれを読み飛ばし、UTF-8 でないファイルとして`停止`しない。

### REQ-core-112: 閉じないコードブロック

- 種類: event_driven
- 出典: docs/decision/records/records.md#A108
- 検証: unit

`コードブロック`が閉じられずに文書が終わるとき、kotowari は開始の行を "line"、開始の行の文字を detail にして unclosed_code_block の`誤り`を出し、開始から文書の終わりまでを検査の対象から外す。
