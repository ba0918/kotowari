# 使い手と作り

[English](cli-scope.md) | 日本語

kotowari を使う者、書き出す先、コードの置き場を扱う。

## Requirements

### REQ-core-101: 使い手

- kind: ubiquitous
- source: docs/decision/records/records.md#A1, docs/decision/records/records.md#A98
- verification: review
- how_to_verify: CLI の出力は JSON/text で LLM が読みやすい形。`src/main.rs` を確認

kotowari は常に、第一に LLM が使う CLI であり、人間が確認のために実行することもある。使う場は、仕様駆動の流れ（brainstorm、`判断の記録`、`IR`、実装）を回す開発者のリポジトリである。

### REQ-core-102: 状態を保存しない

- kind: prohibition
- source: docs/decision/records/records.md#A75, docs/decision/records/2026-09-17-check-reach.md#A3, docs/decision/records/2026-09-17-check-reach.md#A4, docs/decision/records/2026-09-17-check-reach.md#A21, docs/decision/records/2026-10-02-whole-picture.md#A28, docs/decision/records/2026-10-02-whole-picture.md#A80, docs/decision/records/2026-10-02-whole-picture.md#A72
- verification: unit

kotowari は、状態を保存すること、標準出力と標準エラーのほかに書き出すことをしてはならない。ただし "kotowari overview build" と "kotowari overview serve" が`基準のディレクトリ`の ".kotowari/cache/overview/" の下に書き、その下のファイルを消すことだけは除く（REQ-core-296）。

### REQ-core-105: crate と CLI の置き場

- kind: ubiquitous
- source: docs/decision/records/records.md#A8, docs/decision/records/2026-10-03-public-crate-api.md#A14, docs/decision/records/2026-10-03-public-crate-api.md#A34, docs/decision/records/2026-10-03-public-crate-api.md#A35
- verification: review
- how_to_verify: ルートのkotowari-cliパッケージがsrc/main.rsでkotowariバイナリを提供し、ライブラリがTBL-core-040の責務と依存に従ってcrates配下に分かれていることを確認する。

kotowari のコードは常に、層が増えるたびに crate を足し、CLI を直下の "src/" で管理する。

## Examples

```gherkin
@id=EX-core-042 @about=REQ-core-102 @source=docs/decision/records/2026-09-17-check-reach.md#A21,docs/decision/records/2026-09-17-check-reach.md#A28
Scenario: ホームと一時ディレクトリにも書き出さない
  Given 環境変数 "HOME" と "TMPDIR" が空の一時ディレクトリを指す
  When "kotowari check" を実行する
  Then その一時ディレクトリと`基準のディレクトリ`の全ファイルの一覧と中身が、実行の前後で等しい
```
