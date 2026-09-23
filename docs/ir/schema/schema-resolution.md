# スキーマの解決

この文書は、文書の frontmatter が指すスキーマをどう見つけ、取得し、キャッシュするかを扱う。

## Requirements

### REQ-schema-011: スキーマの指定の解決

- kind: algorithm
- source: docs/decision/records/2026-09-21-mds-spec.md#A14, docs/decision/records/2026-09-21-mds-spec.md#A20, docs/decision/records/2026-09-23-mutants-gaps.md#A7
- definition: TBL-schema-003
- verification: unit

### REQ-schema-012: 相対パスの基準

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A14
- verification: unit

mds は常に、`frontmatter`に書いた相対パスを、`文書`の置かれた位置を基準に解決する。`基準のディレクトリ`は相対パスの解決には使わない。

### REQ-schema-013: URL のスキーマのキャッシュ

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A14, docs/decision/records/2026-09-21-mds-spec.md#A20, docs/decision/records/2026-09-23-mutants-gaps.md#A9
- verification: unit

`frontmatter`が URL の`スキーマ`を指したとき、mds は取得した内容を SHA-256 の名前でキャッシュに置き、次からはキャッシュを読む。キャッシュが壊れていれば取得し直して回復する。キャッシュは`基準のディレクトリ`の下に置き、`基準のディレクトリ`が無ければカレントディレクトリの下に置く。

### REQ-schema-014: スキーマを指していない文書

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A8, docs/decision/records/2026-09-21-mds-spec.md#A43, docs/decision/records/2026-09-21-mds-spec.md#P1
- verification: unit

`frontmatter`が YAML のマッピングでないとき、"$schema" の値が空か空白だけのとき、または "$schema" の値が文字列でないとき、mds は`停止`する。

### REQ-schema-015: frontmatter の余分なキー

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A1
- verification: unit

mds は常に、`frontmatter`の "$schema" 以外のキーを読まず、`指摘`にもしない。

### REQ-schema-052: URL の認証情報を伏せる

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A14, docs/decision/records/2026-09-21-mds-spec.md#A25
- verification: unit

`停止`の説明に URL を載せるとき、mds はその authority にある認証情報を伏せる。

## Decision tables

### TBL-schema-003: スキーマの指定の解決

- source: docs/decision/records/2026-09-21-mds-spec.md#A14, docs/decision/records/2026-09-21-mds-spec.md#A8, docs/decision/records/2026-09-21-mds-spec.md#A27, docs/decision/records/2026-09-21-mds-spec.md#A43, docs/decision/records/2026-09-21-mds-spec.md#P1, docs/decision/records/2026-09-23-mutants-gaps.md#A7

| 順 | "$schema" の値 | 解決 |
|---|---|---|
| 1 | 無い（ファイルを対象に指定したとき。ディレクトリのときは REQ-schema-010）、空、空白だけ、文字列でない、`frontmatter`がマッピングでない | `停止` |
| 2 | "http://" か "https://" で始まる | 取得してキャッシュに置く。取得できないとき、応答が 4MiB を超えたとき、または取得全体が10秒を超えたときは`停止` |
| 3 | それ以外 | `文書`の位置からの相対パスとして読む。読めなければ`停止` |

## Properties

### PROP-schema-003: コマンドによって解決先が変わらない

- source: docs/decision/records/2026-09-21-mds-spec.md#A14

同じ`文書`の "$schema" は、検査、`抽出`、素の構文木のどのコマンドから読んでも同じ`スキーマ`に解決する。

## Examples

```gherkin
@id=EX-schema-005 @about=REQ-schema-012 @source=docs/decision/records/2026-09-21-mds-spec.md#A14
Scenario: 相対パスは文書の位置から解決する
  Given `文書`から離れた位置の`スキーマ`を相対パスで指した`文書`がある
  When "kotowari-mds check" を実行する
  Then `スキーマ`は`文書`の位置から解決される
  And 終了コードは 0 である

@id=EX-schema-017 @about=REQ-schema-052 @source=docs/decision/records/2026-09-21-mds-spec.md#A25
Scenario: 認証情報を含む URL は伏せて出す
  Given 認証情報を含む URL の`スキーマ`を指した`文書`があり、取得に失敗する
  When "kotowari-mds check" を実行する
  Then 標準エラーに認証情報は出ない
  And URL は伏せた形で出る

@id=EX-schema-006 @about=REQ-schema-013 @source=docs/decision/records/2026-09-21-mds-spec.md#A14,docs/decision/records/2026-09-21-mds-spec.md#A15
Scenario: 壊れたキャッシュは取得し直して回復する
  Given URL の`スキーマ`を指し、その`スキーマ`をすべて満たす`文書`と、壊れたキャッシュがある
  When "kotowari-mds check" を実行する
  Then `スキーマ`を取得し直す
  And 終了コードは 0 である

@id=EX-schema-066 @about=TBL-schema-003,REQ-schema-011 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A7
Scenario: 4MiB を超える URL のスキーマは停止する
  Given 4MiB を超える応答を返す URL の`スキーマ`を指した`文書`がある
  When "kotowari-mds check" を実行する
  Then 終了コードは 2 である

@id=EX-schema-067 @about=TBL-schema-003,REQ-schema-011 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A7
Scenario: 取得全体が10秒を超える URL のスキーマは停止する
  Given 10秒を超えても応答を終えない URL の`スキーマ`を指した`文書`がある
  When "kotowari-mds check" を実行する
  Then 終了コードは 2 である

@id=EX-schema-068 @about=TBL-schema-003,REQ-schema-011 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A7,docs/decision/records/2026-09-21-mds-spec.md#A15
Scenario: 上限の内で取得できた URL のスキーマは停止しない
  Given 4MiB 以下の応答を10秒以内に返す URL の`スキーマ`を指し、その`スキーマ`をすべて満たす`文書`がある
  When "kotowari-mds check" を実行する
  Then 終了コードは 0 である

@id=EX-schema-069 @about=REQ-schema-013 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A9
Scenario: URL のスキーマのキャッシュは基準のディレクトリの下に置く
  Given ".mds/" のあるディレクトリの下のサブディレクトリがカレントディレクトリで、URL の`スキーマ`を指した`文書`がある
  When "kotowari-mds check" を実行する
  Then キャッシュは ".mds/" のあるディレクトリの下に置かれ、カレントディレクトリの下には置かれない

@id=EX-schema-070 @about=REQ-schema-013 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A9
Scenario: 基準のディレクトリが無ければキャッシュはカレントディレクトリの下に置く
  Given カレントディレクトリから上のどこにも ".mds/" が無く、URL の`スキーマ`を指した`文書`がある
  When "kotowari-mds check" を実行する
  Then キャッシュはカレントディレクトリの下に置かれる
```
