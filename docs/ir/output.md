# 出力の形

"--format" で選ぶ出力の形を扱う。

## 要求

### REQ-021: 出力の形の値

- 種類: ubiquitous
- 出典: docs/decision/records/records.md#A7, docs/decision/records/records.md#A18
- 検証: unit

kotowari は常に、"--format" の値として "json" と "text" の2つを受け、既定を "json" にする。

### REQ-022: JSON を1つ出す

- 種類: event_driven
- 出典: docs/decision/records/records.md#A40, docs/decision/records/ir-form.md#出力
- 検証: unit

"--format" が "json" のとき、kotowari は標準出力に1つの JSON を出す。

### REQ-023: JSON の中身

- 種類: algorithm
- 出典: docs/decision/records/records.md#A40, docs/decision/records/records.md#A56, docs/decision/records/2026-09-17-check-reach.md#A15
- 定義: TBL-005, TBL-006, TBL-021, PROP-002, PROP-004
- 検証: unit

### REQ-025: 文字の出力

- 種類: event_driven
- 出典: docs/decision/records/records.md#A40, docs/decision/records/records.md#A50, docs/decision/records/records.md#A18, docs/decision/records/ir-form.md#出力, docs/decision/records/2026-09-16-notice.md#A1
- 検証: unit

"--format" が "text" のとき、kotowari は1つの`指摘`を1行で "パス:行 [error] 種類 詳細" か "パス:行 [notice] 種類 詳細" の形で出し、角括弧も出す。

### REQ-026: 行の無い指摘の文字の出力

- 種類: event_driven
- 出典: docs/decision/records/records.md#A47
- 検証: unit

"--format" が "text" で`指摘`の "line" が null のとき、kotowari は行を "-" と書く。

### REQ-128: テストのファイルの申告

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-17-check-reach.md#A8, docs/decision/records/2026-09-17-check-reach.md#A14, docs/decision/records/2026-09-17-check-reach.md#A15
- 検証: unit

kotowari は常に、JSON の最上位の "tests" に、読んだ`テストのファイル`を拡張子ごとにまとめ、`TBL-021` の鍵で数と`問い合わせ`の有無を出す。"--format" が "text" のときは出さない。

## 決定表

### TBL-005: JSON の最上位

- 出典: docs/decision/records/records.md#A40, docs/decision/records/records.md#A56, docs/decision/records/ir-form.md#出力, docs/decision/records/2026-09-17-check-reach.md#A8

| 鍵 | 中身 |
|---|---|
| files | 読んだ IR の文書の数（用語集と問題の記録を含む） |
| lines | IR の文書の行数の合計（用語集と問題の記録を含む） |
| findings | 指摘の一覧 |
| counts | 種類ごとの指摘の数 |
| tests | 読んだテストのファイルの拡張子ごとの数と、その拡張子が問い合わせのある言語か（TBL-021） |

### TBL-006: 指摘の鍵

- 出典: docs/decision/records/records.md#A40, docs/decision/records/records.md#A61, docs/decision/records/records.md#A106, docs/decision/records/2026-09-16-ir-tree.md#A13, docs/decision/records/2026-09-16-notice.md#A1

| 鍵 | 中身 |
|---|---|
| kind | 指摘の種類（TBL-008、TBL-009） |
| severity | error か notice |
| path | 基準のディレクトリからの相対パス。正規化した置き場と、置き場からの文書の相対パスを "/" でつなぐ（REQ-110） |
| line | 行（1始まり）。文書全体への指摘は null |
| detail | 種類ごとに TBL-008、TBL-009 で決めた文字列 |

### TBL-021: "tests" の中身

- 出典: docs/decision/records/2026-09-17-check-reach.md#A8, docs/decision/records/2026-09-17-check-reach.md#A14, docs/decision/records/2026-09-17-check-reach.md#A19, docs/decision/records/2026-09-17-check-reach.md#A20, docs/decision/records/2026-09-17-check-reach.md#A26, docs/decision/records/records.md#A128, docs/decision/records/records.md#A165

"tests" はオブジェクトで、鍵は読んだテストのファイルの拡張子、値は "files" と "query" の2つの鍵を持つオブジェクト。テストのファイルが0件なら "tests" は空のオブジェクト。

| 階層 | 鍵 | 値 | 条件 |
|---|---|---|---|
| "tests" の直下 | 拡張子 | "files" と "query" を持つオブジェクト | 読んだテストのファイルの拡張子ごとに1つ。拡張子はファイル名の最後の "." より後ろの文字で、"." を含めない。先頭の "." だけの名前（".rs"）と "." の無い名前は拡張子なし、"foo." の拡張子は空で、いずれも鍵は空文字列。大文字小文字を区別する。並びはバイト順。ファイルはパスごとに1回数え、複数の glob に当たっても1回、シンボリックリンクと実体は別のパスとして数える。glob に当たっても読まないもの（`除外`）は数えない |
| 拡張子の値の中 | files | その拡張子の読んだテストのファイルの数 | unparsable_file を出したファイルも数える |
| 拡張子の値の中 | query | その拡張子が`問い合わせのある言語`なら true、そうでなければ false | 第1版では "rs" だけが true |

## 性質

### PROP-002: counts と findings の一致

- 出典: docs/decision/records/records.md#A40, docs/decision/records/ir-form.md#出力

"counts" の各種類の値は "findings" の中のその種類の`指摘`の数に等しく、"findings" に1件も無い種類は "counts" に無い。

### PROP-004: tests の files の合計

- 出典: docs/decision/records/2026-09-17-check-reach.md#A15

"tests" の各拡張子の "files" の合計は、読んだ`テストのファイル`の数に等しい。

## 具体例

```gherkin
@id=EX-004 @about=REQ-025,REQ-026 @source=docs/decision/records/records.md#A47,docs/decision/records/records.md#A40,docs/decision/records/records.md#A50,docs/decision/records/ir-form.md#検査の種類
Scenario: 題名の無い文書を文字で出す
  Given "docs/ir/a.md" に題名が無い
  When "kotowari check --format text" を実行する
  Then "docs/ir/a.md:- [error] missing_title a.md" の行が出る

@id=EX-035 @about=REQ-128,TBL-021 @source=docs/decision/records/2026-09-17-check-reach.md#A8,docs/decision/records/2026-09-17-check-reach.md#A14,docs/decision/records/2026-09-17-check-reach.md#A19,docs/decision/records/records.md#A128
Scenario: 問い合わせの無い言語のテストのファイルは query が false で申告される
  Given "tests.files" の glob に ".rs" のファイルが2つと ".py" のファイルが1つ当たる
  When "kotowari check" を実行する
  Then JSON の "tests" は "rs" が "files" 2 と "query" true、"py" が "files" 1 と "query" false になる

@id=EX-038 @about=REQ-128,PROP-004 @source=docs/decision/records/2026-09-17-check-reach.md#A14,docs/decision/records/2026-09-17-check-reach.md#A15
Scenario: 読めないテストのファイルも数に入る
  Given "tests.files" の glob に ".rs" のファイルが2つ当たり、片方は tree-sitter で読めない
  When "kotowari check" を実行する
  Then unparsable_file の誤りが1件出て、"tests" の "rs" の "files" は 2 になる
```
