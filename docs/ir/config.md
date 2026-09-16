# 設定

設定ファイルの場所、キーと既定の値、設定の誤りを扱う。

## 要求

### REQ-011: 設定ファイルの場所

- 種類: event_driven
- 出典: docs/decision/brainstorm/records.md#A2, docs/decision/brainstorm/records.md#A37
- 検証: unit

"--config" を受けないとき、kotowari は`基準のディレクトリ`の ".kotowari/config.yaml" を`設定ファイル`として読む。

### REQ-012: 設定ファイルが無いとき

- 種類: event_driven
- 出典: docs/decision/brainstorm/records.md#A12, docs/decision/brainstorm/records.md#A60, docs/decision/brainstorm/records.md#A105, docs/decision/brainstorm/records.md#A135
- 検証: unit

"--config" を受けずに`設定ファイル`が無いとき、kotowari は既定の値で検査を行う。`設定ファイル`が空（0バイトか注釈だけ）のときは、"--config" で指したものでも既定の値で検査を行う。

### REQ-013: キーと既定の値

- 種類: algorithm
- 出典: docs/decision/brainstorm/records.md#A12, docs/decision/brainstorm/records.md#A23, docs/decision/brainstorm/records.md#A36, docs/decision/brainstorm/records.md#A41, docs/decision/brainstorm/records.md#A47, docs/decision/brainstorm/records.md#A48, docs/decision/brainstorm/records.md#A49, docs/decision/brainstorm/2026-09-16-notice.md#A5
- 定義: TBL-004
- 検証: unit

### REQ-014: 設定の誤り

- 種類: event_driven
- 出典: docs/decision/brainstorm/records.md#A12, docs/decision/brainstorm/records.md#A41, docs/decision/brainstorm/records.md#A20, docs/decision/brainstorm/records.md#A44, docs/decision/brainstorm/records.md#A93, docs/decision/brainstorm/records.md#A105, docs/decision/brainstorm/records.md#A135, docs/decision/brainstorm/records.md#A161
- 検証: unit

`設定ファイル`が YAML として読めないとき、または`設定ファイル`に知らないキー、同じキーの2回目、値が null のキー（"ir:" だけの行。空の一覧は REQ-016 のとおり受ける）、型の違う値、負の数、0、絶対パスの値（先頭が "/" のもの）、"vague_words" の空の文字列の要素か同じ語の2回目、"tests.files" の glob として読めない要素のいずれかがあるとき、kotowari は設定の誤りを理由に`停止`する。

### REQ-015: 一覧は既定を置き換える

- 種類: ubiquitous
- 出典: docs/decision/brainstorm/records.md#A41
- 検証: unit

kotowari は常に、一覧のキーに一覧だけを受け、書かれた一覧で既定の一覧を置き換える。

### REQ-016: 空の一覧

- 種類: event_driven
- 出典: docs/decision/brainstorm/records.md#A59
- 検証: unit

一覧のキーに空の一覧が書かれているとき、kotowari はそのキーを要素の無い一覧として扱う。

### REQ-017: 入れ子のキー

- 種類: ubiquitous
- 出典: docs/decision/brainstorm/records.md#A59
- 検証: unit

kotowari は常に、`設定ファイル`のキーを入れ子の形（"decisions:" の下の "records:"）で読む。

### REQ-018: 置き場が無いとき

- 種類: event_driven
- 出典: docs/decision/brainstorm/records.md#A41, docs/decision/brainstorm/records.md#A47, docs/decision/brainstorm/records.md#A66, docs/decision/brainstorm/records.md#A95, docs/decision/brainstorm/records.md#A96, docs/decision/brainstorm/records.md#A124, docs/decision/brainstorm/records.md#A146, docs/decision/brainstorm/2026-09-16-ir-tree.md#A16
- 検証: unit

"ir"、"decisions.records"、"decisions.adr" の指す先が無いとき、ディレクトリでないとき、または読めないとき、kotowari は読めないファイルを理由に`停止`する。"ir"、"decisions.records"、"decisions.adr" の下のディレクトリが読めないとき、"tests.files" の走査でディレクトリが読めないとき、および走査で先の無いシンボリックリンクに出会ったときも同じ理由で`停止`する。

### REQ-019: glob の読み方

- 種類: ubiquitous
- 出典: docs/decision/brainstorm/records.md#A59, docs/decision/brainstorm/records.md#A102
- 検証: unit

kotowari は常に、glob の "**" を再帰として読み、隠しディレクトリを glob が名指ししても含めず、隠しファイルは glob が当てれば読み、ディレクトリのシンボリックリンクを辿らない。

### REQ-020: 直下の kotowari.toml を読まない

- 種類: prohibition
- 出典: docs/decision/brainstorm/records.md#R6, docs/decision/brainstorm/2026-09-17-check-reach.md#A3, docs/decision/brainstorm/2026-09-17-check-reach.md#A4
- 検証: unit

kotowari は、リポジトリ直下の "kotowari.toml" を`設定ファイル`として読んではならない。

## 決定表

### TBL-004: キーと既定の値

- 出典: docs/decision/brainstorm/records.md#A12, docs/decision/brainstorm/records.md#A23, docs/decision/brainstorm/records.md#A36, docs/decision/brainstorm/records.md#A41, docs/decision/brainstorm/records.md#A47, docs/decision/brainstorm/records.md#A48, docs/decision/brainstorm/records.md#A49, docs/decision/brainstorm/records.md#A62, docs/decision/brainstorm/records.md#A69, docs/decision/brainstorm/2026-09-16-notice.md#A5

| キー | 値 | 既定 |
|---|---|---|
| ir | ディレクトリのパス（1つの文字列） | docs/ir |
| decisions.records | ディレクトリのパス（1つの文字列）。その下のファイルの決定を出典に指せる。判断の記録でない Markdown（形の契約、補足の文書）も置ける | docs/decision/brainstorm |
| decisions.adr | ディレクトリのパス（1つの文字列） | docs/decision/adr |
| tests.files | glob の一覧 | src/\*\*/\*.rs、tests/\*\*/\*.rs |
| tests.rust.attributes | "#[test]" に足す属性のパスの一覧 | 空の一覧 |
| tests.rust.macros | マクロの名前の一覧 | 空の一覧 |
| limits.lines | 数（負の数と0は不可） | 200 |
| limits.requirements | 数（負の数と0は不可） | 10 |
| vague_words | 語の一覧 | 「適切に」「必要に応じて」「通常は」「など」の4語 |

## 具体例

```gherkin
@id=EX-003 @about=REQ-014 @source=docs/decision/brainstorm/records.md#A12,docs/decision/brainstorm/records.md#A20,docs/decision/brainstorm/records.md#A41
Scenario: 知らないキーで停止する
  Given 設定ファイルに "limit:" という知らないキーがある
  When "kotowari check" を実行する
  Then 終了コードは 2 である
```
