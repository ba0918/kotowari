# 設定

設定ファイルの場所、キーと既定の値、設定の誤りを扱う。

## Requirements

### REQ-core-011: 設定ファイルの場所

- kind: event_driven
- source: docs/decision/records/records.md#A2, docs/decision/records/records.md#A37, docs/decision/records/2026-09-24-plan-schema.md#A15
- verification: unit

"kotowari plan" でないコマンドで "--config" を受けないとき、kotowari は`基準のディレクトリ`の ".kotowari/config.yaml" を`設定ファイル`として読む。

### REQ-core-012: 設定ファイルが無いとき

- kind: event_driven
- source: docs/decision/records/records.md#A12, docs/decision/records/records.md#A60, docs/decision/records/records.md#A105, docs/decision/records/records.md#A135
- verification: unit

"--config" を受けずに`設定ファイル`が無いとき、kotowari は既定の値で検査を行う。`設定ファイル`が空（0バイトか注釈だけ）のときは、"--config" で指したものでも既定の値で検査を行う。

### REQ-core-013: キーと既定の値

- kind: algorithm
- source: docs/decision/records/records.md#A12, docs/decision/records/records.md#A23, docs/decision/records/records.md#A36, docs/decision/records/records.md#A41, docs/decision/records/records.md#A47, docs/decision/records/records.md#A48, docs/decision/records/records.md#A49, docs/decision/records/2026-09-16-notice.md#A5
- definition: TBL-core-004
- verification: unit

### REQ-core-014: 設定の誤り

- kind: event_driven
- source: docs/decision/records/records.md#A12, docs/decision/records/records.md#A41, docs/decision/records/records.md#A20, docs/decision/records/records.md#A44, docs/decision/records/records.md#A93, docs/decision/records/records.md#A105, docs/decision/records/records.md#A135, docs/decision/records/records.md#A161
- verification: unit

`設定ファイル`が YAML として読めないとき、または`設定ファイル`に知らないキー、同じキーの2回目、値が null のキー（"ir:" だけの行。空の一覧は REQ-core-016 のとおり受ける）、型の違う値、負の数、0、絶対パスの値（先頭が "/" のもの）、"vague_words" の空の文字列の要素か同じ語の2回目、"tests.files" の glob として読めない要素のいずれかがあるとき、kotowari は設定の誤りを理由に`停止`する。

### REQ-core-015: 一覧は既定を置き換える

- kind: ubiquitous
- source: docs/decision/records/records.md#A41
- verification: unit

kotowari は常に、一覧のキーに一覧だけを受け、書かれた一覧で既定の一覧を置き換える。

### REQ-core-016: 空の一覧

- kind: event_driven
- source: docs/decision/records/records.md#A59
- verification: unit

一覧のキーに空の一覧が書かれているとき、kotowari はそのキーを要素の無い一覧として扱う。

### REQ-core-017: 入れ子のキー

- kind: ubiquitous
- source: docs/decision/records/records.md#A59
- verification: unit

kotowari は常に、`設定ファイル`のキーを入れ子の形（"decisions:" の下の "records:"）で読む。

### REQ-core-018: 置き場が無いとき

- kind: event_driven
- source: docs/decision/records/records.md#A41, docs/decision/records/records.md#A47, docs/decision/records/records.md#A66, docs/decision/records/records.md#A95, docs/decision/records/records.md#A96, docs/decision/records/records.md#A124, docs/decision/records/records.md#A146, docs/decision/records/2026-09-16-ir-tree.md#A16, docs/decision/records/2026-09-17-mutation-tests.md#A55
- verification: unit

"kotowari check" で、"ir"、"decisions.records"、"decisions.adr" の指す先が無いとき、ディレクトリでないとき、または読めないとき、kotowari は読めないファイルを理由に`停止`する。"ir"、"decisions.records"、"decisions.adr" の下のディレクトリが読めないとき、"tests.files" の走査でディレクトリが読めないとき、および走査で先の無いシンボリックリンクに出会ったときも同じ理由で`停止`する。

### REQ-core-019: glob の読み方

- kind: ubiquitous
- source: docs/decision/records/records.md#A59, docs/decision/records/records.md#A102
- verification: unit

kotowari は常に、glob の "**" を再帰として読み、隠しディレクトリを glob が名指ししても含めず、隠しファイルは glob が当てれば読み、ディレクトリのシンボリックリンクを辿らない。

### REQ-core-020: 直下の kotowari.toml を読まない

- kind: prohibition
- source: docs/decision/records/records.md#R6, docs/decision/records/2026-09-17-check-reach.md#A3, docs/decision/records/2026-09-17-check-reach.md#A4
- verification: unit

kotowari は、リポジトリ直下の "kotowari.toml" を`設定ファイル`として読んではならない。

## Decision tables

### TBL-core-004: キーと既定の値

- source: docs/decision/records/records.md#A12, docs/decision/records/records.md#A23, docs/decision/records/records.md#A36, docs/decision/records/records.md#A41, docs/decision/records/records.md#A47, docs/decision/records/records.md#A48, docs/decision/records/records.md#A49, docs/decision/records/records.md#A62, docs/decision/records/records.md#A69, docs/decision/records/2026-09-16-notice.md#A5, docs/decision/records/2026-09-17-mutation-tests.md#A36, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-17-mutation-tests.md#A16, docs/decision/records/2026-09-24-multi-language-tests.md#A9, docs/decision/records/2026-09-24-multi-language-tests.md#A28

| キー | 値 | 既定 |
|---|---|---|
| ir | ディレクトリのパス（1つの文字列） | docs/ir |
| decisions.records | ディレクトリのパス（1つの文字列）。その下のファイルの決定を出典に指せる。判断の記録でない Markdown（形の契約、補足の文書）も置ける | docs/decision/records |
| decisions.adr | ディレクトリのパス（1つの文字列） | docs/decision/adr |
| tests.files | glob の一覧 | src/\*\*/\*.rs、tests/\*\*/\*.rs |
| tests.rust.attributes | "#[test]" に足す属性のパスの一覧 | 空の一覧 |
| tests.rust.macros | マクロの名前の一覧 | 空の一覧 |
| tests.rules | ast-grep のルールの YAML ファイルのパスの一覧。基準のディレクトリからの相対パスで、glob は使えない（query-rules.md） | 空の一覧 |
| mutants.equivalents | ファイルのパス（1つの文字列）。等価の一覧を指す | 無し（鍵が無ければ等価の一覧は0件） |
| limits.lines | 数（負の数と0は不可） | 200 |
| limits.requirements | 数（負の数と0は不可） | 10 |
| vague_words | 語の一覧 | 「適切に」「必要に応じて」「通常は」「など」の4語 |

## Examples

```gherkin
@id=EX-core-003 @about=REQ-core-014 @source=docs/decision/records/records.md#A12,docs/decision/records/records.md#A20,docs/decision/records/records.md#A41
Scenario: 知らないキーで停止する
  Given 設定ファイルに "limit:" という知らないキーがある
  When "kotowari check" を実行する
  Then 終了コードは 2 である
```
