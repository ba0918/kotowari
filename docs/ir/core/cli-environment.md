# 使い方の表示と対象の環境

"--help" と "--version" の扱い、対象の OS、停止と指摘の振り分け、停止の理由と詳細の文言を扱う。

## Requirements

### REQ-core-107: 使い方と版の表示

- kind: event_driven
- source: docs/decision/records/records.md#A103, docs/decision/records/records.md#A136
- verification: unit

"--help" か "--version" を受けたとき、kotowari はほかの引数を見ず、検査を行わず、使い方か版の文字列を標準出力に出して終了コード0で終わる。"check" が無くてもよい。

### REQ-core-108: 対象の環境

- kind: ubiquitous
- source: docs/decision/records/records.md#A125
- verification: review
- how_to_verify: Linux と macOS で `cargo test` が通ることを確認。Windows は動作を約束しない（パスの区切りの正規化 REQ-core-110 だけ）

kotowari は常に、Linux と macOS を対象にする。Windows ではパスの区切りの正規化（REQ-core-110）だけを行い、それ以外の動作を約束しない。

### REQ-core-109: 停止と指摘の振り分け

- kind: invariant
- source: docs/decision/records/records.md#A100, docs/decision/records/records.md#A101, docs/decision/records/records.md#P2, docs/decision/records/records.md#A102, docs/decision/records/2026-09-17-mutation-tests.md#A48, docs/decision/records/2026-09-17-mutation-tests.md#A55, docs/decision/records/2026-09-27-surface-check.md#A6, docs/decision/records/2026-09-27-surface-check.md#A22, docs/decision/records/2026-09-27-surface-check.md#A24
- verification: review
- how_to_verify: `crates/kotowari/src/acquisition.rs` の `load_all` で置き場の存在を検査し、`StopReason` で停止していることを確認。黙って飛ばす経路が無いことを `rg 'filter_map|if let Ok' crates/kotowari/src/ crates/kotowari-core/src/` で確認

読めない入力、壊れている入力、契約の形に合わない入力に対して、kotowari は`停止`か`誤り`の`指摘`のどちらかを必ず行い、黙って飛ばさない関係が常に成り立つ。ファイルや設定を全体として読む前提が崩れる入力（読めない、UTF-8 でない、設定の構文と型と値の誤り、引数の誤り、glob の構文の誤り、結果のファイルの誤り、`等価の一覧`と`未記載の面の一覧`の構文の誤り）では`停止`し、読めたが局所的に形から外れる入力ではその場所への`誤り`の`指摘`を出す。読まないものは`除外`だけである。`変異の結果`か`等価の一覧`の1件が指すファイルが無い、読めない、UTF-8 でないときは、REQ-core-141 と REQ-core-142 のとおり`停止`せず、`誤り`か`注意`の`指摘`に倒す。

### REQ-core-120: 黙って読み飛ばさない

- kind: prohibition
- source: docs/decision/records/records.md#P2, docs/decision/records/records.md#A100
- verification: review
- how_to_verify: `crates/kotowari-core/src/ir.rs` と `crates/kotowari/src/acquisition.rs` で、仕様に列挙されていない振る舞いを黙って決めていないことを確認。`GherkinBlock` の gherkin 解析で有効な行の種類以外を invalid_gherkin_line にし、`read_utf8_file` で読めないファイルを停止にし、`parse_document` が写せない指摘と値を停止にし、`check_documents` が仕様に無い読み飛ばしを持たないことを確認

kotowari は、`除外`に列挙していない入力を、`停止`も`指摘`もせずに読み飛ばしてはならない。

### REQ-core-175: 置き換えで増える停止の境界

- kind: event_driven
- source: docs/decision/records/2026-09-22-ir-engine.md#A9, docs/decision/records/2026-09-22-ir-engine.md#A20, docs/decision/records/2026-09-22-ir-engine.md#A28, docs/decision/records/2026-09-22-ir-engine.md#A36, docs/decision/records/2026-09-24-plan-schema.md#A11
- verification: unit

`IR`の文書について、スキーマの側から受けた値を kotowari の型へ写せないとき、または対応表（TBL-core-030）に写し先が無いとき、kotowari は TBL-core-018 に1つだけ足した理由で`停止`する。`IR`の文書が読めない、UTF-8 でないといった利用者の入力で起きる`停止`の理由と文言は変えず、スキーマを読めないことを理由とする`停止`は持たない。

### REQ-core-176: 形の指摘が出た文書も文書をまたぐ検査を受ける

- kind: ubiquitous
- source: docs/decision/records/2026-09-22-ir-engine.md#A5, docs/decision/records/2026-09-22-ir-engine.md#A18, docs/decision/records/records.md#A100, docs/decision/records/records.md#A101
- verification: unit

kotowari は常に、スキーマの側の`指摘`が出た文書でも、取れた値で文書をまたぐ検査を続け、その文書を検査の対象から外さない。

## Decision tables

### TBL-core-018: 停止の理由の文言

- source: docs/decision/records/records.md#A104, docs/decision/records/2026-09-17-mutation-tests.md#A39, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-22-ir-engine.md#A73, docs/decision/records/2026-10-02-whole-picture.md#A61, docs/decision/records/2026-10-02-whole-picture.md#A72, docs/decision/records/2026-10-04-overview-on-public-api.md#A9

| 理由 | 標準エラーの1行目の文言 |
|---|---|
| 設定の誤り | config error |
| 引数の誤り | argument error |
| 読めないファイル | unreadable file |
| UTF-8 でないファイル | non-UTF-8 file |
| 結果の誤り | results error |
| 写しの誤り | mapping error |
| 元データの誤り | overview error |
| ポートの誤り | port error |
| 置き場の誤り | cache error |

### TBL-core-020: 停止の詳細

- source: docs/decision/records/2026-09-22-ir-engine.md#A73, docs/decision/records/records.md#A137, docs/decision/records/records.md#A147, docs/decision/records/records.md#A160, docs/decision/records/records.md#A164, docs/decision/records/2026-09-17-mutation-tests.md#A39, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-17-mutation-tests.md#A49, docs/decision/records/2026-09-17-mutation-tests.md#A55, docs/decision/records/2026-09-19-read-commands.md#A20, docs/decision/records/2026-09-20-query-status.md#A6, docs/decision/records/2026-09-20-query-status.md#A18, docs/decision/records/2026-09-24-plan-schema.md#A10, docs/decision/records/2026-09-24-plan-schema.md#A30, docs/decision/records/2026-09-24-doc-marks.md#A15, docs/decision/records/2026-09-24-doc-marks.md#A28, docs/decision/records/2026-09-24-doc-marks.md#A36, docs/decision/records/2026-09-24-guide-gaps.md#A2, docs/decision/records/2026-09-27-surface-check.md#A20, docs/decision/records/2026-09-27-surface-check.md#A22, docs/decision/records/2026-09-27-surface-check.md#A24, docs/decision/records/2026-10-01-change-conformance.md#A2, docs/decision/records/2026-10-01-change-details.md#A15, docs/decision/records/2026-10-02-whole-picture.md#A61, docs/decision/records/2026-10-02-whole-picture.md#A72, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-02-whole-picture.md#A75, docs/decision/records/2026-10-02-whole-picture.md#A78, docs/decision/records/2026-10-02-whole-picture.md#A85, docs/decision/records/2026-10-04-overview-on-public-api.md#A9

| 理由 | 詳細（英語） |
|---|---|
| 設定の誤り | 設定ファイルの基準のディレクトリからの相対パス（基準の外にあれば "../" を含む）と、誤りの説明。等価の一覧の誤り（REQ-core-148）では等価の一覧のファイルの相対パスと、誤りの説明。"tests.rules" の誤り（REQ-core-189）と "surface.rules" の誤り（REQ-core-225）ではルールのファイルの相対パスと、誤りの説明。未記載の面の一覧の誤り（REQ-core-231）では未記載の面の一覧のファイルの相対パスと、誤りの説明。ガイドとテストの置き場の重なり（REQ-core-199）では重なったファイルのうちパスのバイト順で最初の1つの相対パスに ": matched by both guides.files and tests.files" を続けたもの。`全体像の元データ`の置き場の重なり（REQ-core-280）では同じく ": matched by both overview.files and guides.files" か ": matched by both overview.files and tests.files" を続けたもの。"overview" の鍵が無いまま "kotowari overview build" か "kotowari overview serve" を実行したとき（REQ-core-279）は "overview is not configured" だけ |
| 引数の誤り | 説明の文と、問題の引数の文字。引数が1つも無いときと、1つ目の位置引数が無いときは "expected command: check, changes, list, mutants, overview, plan, query or status"。"kotowari query" で `ID` を持つものが無いときは "unknown id: " と位置引数の文字（REQ-core-157） |
| 読めないファイル | 相対パスと、OS の誤りの文。カレントディレクトリを取得できないときは "current directory: " と OS の誤りの文 |
| UTF-8 でないファイル | 相対パス |
| 結果の誤り | 結果のファイルの相対パスと、誤りの説明 |
| 写しの誤り | 写せなかった`指摘`の種類と`ノードの名前`、または写せなかった値の説明 |
| 元データの誤り | `誤り`の件数と " errors in overview data; run kotowari check"（REQ-core-294） |
| ポートの誤り | "127.0.0.1:<ポート>" と、": " と、OS の誤りの文（REQ-core-298） |
| 置き場の誤り | 問題のパスの`基準のディレクトリ`からの相対パスと、OS の誤りがあれば ": " と OS の誤りの文（REQ-core-324） |

## Examples

```gherkin
@id=EX-core-268 @about=REQ-core-175 @source=docs/decision/records/2026-09-22-ir-engine.md#A9,docs/decision/records/2026-09-22-ir-engine.md#A20
Scenario: 型へ写せない値は停止になる
  Given kotowari の型へ写せない値を返すスキーマを取り込んだビルドがある
  When "kotowari check" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は TBL-core-018 に足した理由の文言である

@id=EX-core-269 @about=REQ-core-176 @source=docs/decision/records/2026-09-22-ir-engine.md#A5,docs/decision/records/2026-09-22-ir-engine.md#A18,docs/decision/records/records.md#A100,docs/decision/records/records.md#A101,docs/decision/records/2026-09-23-ir-english-tokens.md#A2
Scenario: 形の指摘が出た文書も文書をまたぐ検査を受ける
  Given "- verification:" の行が欠けた`要求`があり、その`ID`が別の文書の`要求`の`ID`と重なっている`IR`がある
  When "kotowari check --format json" を実行する
  Then verification_missing と duplicate_id の`誤り`が両方出る
```
