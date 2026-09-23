# 使い手と作り

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
- source: docs/decision/records/records.md#A75, docs/decision/records/2026-09-17-check-reach.md#A3, docs/decision/records/2026-09-17-check-reach.md#A4, docs/decision/records/2026-09-17-check-reach.md#A21
- verification: unit

kotowari は、状態を保存すること、標準出力と標準エラーのほかに書き出すことをしてはならない。

### REQ-core-105: crate と CLI の置き場

- kind: ubiquitous
- source: docs/decision/records/records.md#A8
- verification: review
- how_to_verify: CLI は `src/main.rs` のバイナリ1つ、ライブラリは `crates/kotowari-core/`。モジュールは config, ir, sources, terms, tests_discovery, list, query, status, mutants, cargo_mutants, equivalents, record_form

kotowari のコードは常に、層が増えるたびに crate を足し、CLI を直下の "src/" で管理する。

## Examples

```gherkin
@id=EX-core-042 @about=REQ-core-102 @source=docs/decision/records/2026-09-17-check-reach.md#A21,docs/decision/records/2026-09-17-check-reach.md#A28
Scenario: ホームと一時ディレクトリにも書き出さない
  Given 環境変数 "HOME" と "TMPDIR" が空の一時ディレクトリを指す
  When "kotowari check" を実行する
  Then その一時ディレクトリと`基準のディレクトリ`の全ファイルの一覧と中身が、実行の前後で等しい
```
