# 出力の形

"--format" で選ぶ出力の形を扱う。

## Requirements

### REQ-core-021: 出力の形の値

- kind: ubiquitous
- source: docs/decision/records/records.md#A7, docs/decision/records/records.md#A18
- verification: unit

kotowari は常に、"--format" の値として "json" と "text" の2つを受け、既定を "json" にする。

### REQ-core-022: JSON を1つ出す

- kind: event_driven
- source: docs/decision/records/records.md#A40, docs/decision/records/ir-form.md#出力
- verification: unit

"--format" が "json" のとき、kotowari は標準出力に1つの JSON を出す。

### REQ-core-023: JSON の中身

- kind: algorithm
- source: docs/decision/records/records.md#A40, docs/decision/records/records.md#A56, docs/decision/records/2026-09-17-check-reach.md#A15
- definition: TBL-core-005, TBL-core-006, TBL-core-021, PROP-core-002, PROP-core-004
- verification: unit

### REQ-core-025: 文字の出力

- kind: event_driven
- source: docs/decision/records/records.md#A40, docs/decision/records/records.md#A50, docs/decision/records/records.md#A18, docs/decision/records/ir-form.md#出力, docs/decision/records/2026-09-16-notice.md#A1, docs/decision/records/2026-09-24-review5-gaps.md#A2
- verification: unit

"--format" が "text" のとき、kotowari は1つの`指摘`を1行で "パス:行 [error] 種類 詳細" か "パス:行 [notice] 種類 詳細" の形で出し、角括弧も出す。パスと詳細の中の改行（"\n" と "\r"）は、バックスラッシュと "n" または "r" の2文字で書く。

### REQ-core-026: 行の無い指摘の文字の出力

- kind: event_driven
- source: docs/decision/records/records.md#A47
- verification: unit

"--format" が "text" で`指摘`の "line" が null のとき、kotowari は行を "-" と書く。

### REQ-core-128: テストのファイルの申告

- kind: ubiquitous
- source: docs/decision/records/2026-09-17-check-reach.md#A8, docs/decision/records/2026-09-17-check-reach.md#A14, docs/decision/records/2026-09-17-check-reach.md#A15, docs/decision/records/2026-09-17-mutation-tests.md#A55
- verification: unit

kotowari は常に、"kotowari check" の JSON の最上位の "tests" に、読んだ`テストのファイル`を拡張子ごとにまとめ、`TBL-core-021` の鍵で数と`問い合わせ`の有無を出す。"--format" が "text" のときは出さない。

### REQ-core-206: ガイドの申告

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-doc-marks.md#A17, docs/decision/records/2026-09-24-doc-marks.md#A35
- verification: unit

kotowari は常に、"kotowari check" の JSON の最上位の "guides" に、読んだ`ガイド`の数を "files"、形の正しい`ガイドの印`の1件の数を "marks" として出す。"files" はファイルを TBL-core-021 と同じ数え方（パスごとに1回、複数の glob に当たっても1回、シンボリックリンクと実体は別のパス）で数える。`ガイド`が0件なら、どちらも 0 である。"--format" が "text" のときは出さない。

## Decision tables

### TBL-core-005: check の JSON の最上位

- source: docs/decision/records/records.md#A40, docs/decision/records/records.md#A56, docs/decision/records/ir-form.md#出力, docs/decision/records/2026-09-17-check-reach.md#A8, docs/decision/records/2026-09-17-mutation-tests.md#A55, docs/decision/records/2026-09-24-plan-schema.md#A17, docs/decision/records/2026-09-24-doc-marks.md#A17, docs/decision/records/2026-09-24-doc-marks.md#A35, docs/decision/records/2026-09-27-surface-check.md#A3, docs/decision/records/2026-09-27-surface-check.md#A4, docs/decision/records/2026-09-27-surface-check.md#A16, docs/decision/records/2026-10-02-whole-picture.md#A64

"kotowari check" の JSON の最上位。"kotowari mutants" の JSON の最上位は TBL-core-025、"kotowari plan" の JSON の最上位は REQ-core-194。

| 鍵 | 中身 |
|---|---|
| files | 読んだ IR の文書の数（用語集と問題の記録を含む） |
| lines | IR の文書の行数の合計（用語集と問題の記録を含む） |
| findings | 指摘の一覧 |
| counts | 種類ごとの指摘の数 |
| tests | 読んだテストのファイルの拡張子ごとの数と、その拡張子が問い合わせのある言語か（TBL-core-021） |
| guides | "files"（読んだ`ガイド`の数）と "marks"（形の正しい`ガイドの印`の1件の数）の2つの鍵を持つオブジェクト（REQ-core-206） |
| overview | "files"（読んだ`全体像の元データ`の数）と "marks"（`全体像の元データ`の中の形の正しい`ガイドの印`の1件の数）の2つの鍵を持つオブジェクト（REQ-core-288） |
| surface | "unspecified"（未記載の面の一覧で外した面の数）の鍵1つを持つオブジェクト。"surface.rules" が空の一覧のときは鍵ごと出さない（REQ-core-228） |

### TBL-core-006: 指摘の鍵

- source: docs/decision/records/records.md#A40, docs/decision/records/records.md#A61, docs/decision/records/records.md#A106, docs/decision/records/2026-09-16-ir-tree.md#A13, docs/decision/records/2026-09-16-notice.md#A1, docs/decision/records/2026-09-17-mutation-tests.md#A40, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-24-plan-schema.md#A26, docs/decision/records/2026-09-24-doc-marks.md#A10, docs/decision/records/2026-09-24-doc-marks.md#A11, docs/decision/records/2026-09-24-doc-marks.md#A32, docs/decision/records/2026-09-27-surface-check.md#A6, docs/decision/records/2026-09-27-surface-check.md#A7, docs/decision/records/2026-09-27-surface-check.md#A15, docs/decision/records/2026-09-27-surface-check.md#A22

| 鍵 | 中身 |
|---|---|
| kind | 指摘の種類（TBL-core-008、TBL-core-009） |
| severity | error か notice |
| path | 基準のディレクトリからの相対パス。正規化した置き場と、置き場からの文書の相対パスを "/" でつなぐ（REQ-core-110）。"kotowari mutants" の指摘では変異の結果のファイルか等価の一覧のファイル（REQ-core-139、REQ-core-140、REQ-core-142、REQ-core-143）。"kotowari plan" の指摘では`計画書`のファイル（REQ-core-193）。ガイドへの指摘では`ガイド`の基準のディレクトリからの相対パス（REQ-core-202、REQ-core-204）。surface_without_spec では`面のファイル`、未記載の面の一覧への指摘では未記載の面の一覧のファイル（REQ-core-227、REQ-core-233、REQ-core-234） |
| line | 行（1始まり）。文書全体への指摘は null |
| detail | 種類ごとに TBL-core-008、TBL-core-009 で決めた文字列 |

### TBL-core-021: "tests" の中身

- source: docs/decision/records/2026-09-17-check-reach.md#A8, docs/decision/records/2026-09-17-check-reach.md#A14, docs/decision/records/2026-09-17-check-reach.md#A19, docs/decision/records/2026-09-17-check-reach.md#A20, docs/decision/records/2026-09-17-check-reach.md#A26, docs/decision/records/records.md#A128, docs/decision/records/records.md#A165, docs/decision/records/2026-09-24-multi-language-tests.md#A24, docs/decision/records/2026-09-24-multi-language-tests.md#A8, docs/decision/records/2026-09-24-multi-language-tests.md#A21, docs/decision/records/2026-09-24-multi-language-tests.md#A47

"tests" はオブジェクトで、鍵は読んだテストのファイルの拡張子、値は "files" と "query" の2つの鍵を持つオブジェクト。テストのファイルが0件なら "tests" は空のオブジェクト。

| 階層 | 鍵 | 値 | 条件 |
|---|---|---|---|
| "tests" の直下 | 拡張子 | "files" と "query" を持つオブジェクト | 読んだテストのファイルの拡張子ごとに1つ。拡張子はファイル名の最後の "." より後ろの文字で、"." を含めない。先頭の "." だけの名前（".rs"）と "." の無い名前は拡張子なし、"foo." の拡張子は空で、いずれも鍵は空文字列。大文字小文字を区別する。並びはバイト順。ファイルはパスごとに1回数え、複数の glob に当たっても1回、シンボリックリンクと実体は別のパスとして数える。glob に当たっても読まないもの（`除外`）は数えない |
| 拡張子の値の中 | files | その拡張子の読んだテストのファイルの数 | unparsable_file を出したファイルも数える |
| 拡張子の値の中 | query | その拡張子から決まる言語が`問い合わせのある言語`なら true、そうでなければ false | 同梱の問い合わせだけなら "rs"、"ts"、"mts"、"cts"、"tsx"、"js"、"jsx"、"mjs"、"cjs"、"py"、"py3"、"pyi"、"bzl"、"bazel"、"php" が true |

## Properties

### PROP-core-002: counts と findings の一致

- source: docs/decision/records/records.md#A40, docs/decision/records/ir-form.md#出力

"counts" の各種類の値は "findings" の中のその種類の`指摘`の数に等しく、"findings" に1件も無い種類は "counts" に無い。

### PROP-core-004: tests の files の合計

- source: docs/decision/records/2026-09-17-check-reach.md#A15

"tests" の各拡張子の "files" の合計は、読んだ`テストのファイル`の数に等しい。

## Examples

```gherkin
@id=EX-core-286 @about=REQ-core-025 @source=docs/decision/records/2026-09-24-review5-gaps.md#A2
Scenario: 改行を含むファイル名でも1つの指摘は1行
  Given 名前に改行を含み、`題名`の無い`IR`の文書がある
  When "kotowari check --format text" を実行する
  Then どの行もその文書のパスで始まり、パスの改行はバックスラッシュと "n" の2文字で書かれる

@id=EX-core-004 @about=REQ-core-025,REQ-core-026 @source=docs/decision/records/records.md#A47,docs/decision/records/records.md#A40,docs/decision/records/records.md#A50,docs/decision/records/ir-form.md#検査の種類
Scenario: 題名の無い文書を文字で出す
  Given "docs/ir/a.md" に題名が無い
  When "kotowari check --format text" を実行する
  Then "docs/ir/a.md:- [error] missing_title a.md" の行が出る

@id=EX-core-035 @about=REQ-core-128,TBL-core-021 @source=docs/decision/records/2026-09-17-check-reach.md#A8,docs/decision/records/2026-09-17-check-reach.md#A14,docs/decision/records/2026-09-17-check-reach.md#A19,docs/decision/records/records.md#A128,docs/decision/records/2026-09-24-multi-language-tests.md#A7,docs/decision/records/2026-09-24-multi-language-tests.md#A24
Scenario: 問い合わせの無い言語のテストのファイルは query が false で申告される
  Given "tests.files" の glob に ".rs" のファイルが2つと ".go" のファイルが1つ当たり、".go" の`問い合わせ`は無い
  When "kotowari check" を実行する
  Then JSON の "tests" は "rs" が "files" 2 と "query" true、"go" が "files" 1 と "query" false になる

@id=EX-core-038 @about=REQ-core-128,PROP-core-004 @source=docs/decision/records/2026-09-17-check-reach.md#A14,docs/decision/records/2026-09-17-check-reach.md#A15
Scenario: 読めないテストのファイルも数に入る
  Given "tests.files" の glob に ".rs" のファイルが2つ当たり、片方は tree-sitter で読めない
  When "kotowari check" を実行する
  Then unparsable_file の誤りが1件出て、"tests" の "rs" の "files" は 2 になる
```
