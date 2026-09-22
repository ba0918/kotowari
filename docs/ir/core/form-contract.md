# IR の形の固定

IR の形をどこで決めるか、その形を誰が読むか、採らない形の決め方を扱う。

## 要求

### REQ-core-089: 形はコードに固定する

- 種類: ubiquitous
- 出典: docs/decision/records/records.md#A25, docs/decision/records/records.md#A52, docs/decision/records/records.md#A98
- 検証: review
- 確かめ方: `crates/kotowari-core/src/schema.rs` がコンパイル時に取り込んだスキーマで、`crates/kotowari-core/src/ir.rs` の `parse_document` が IR を読むことを確認

kotowari は常に、コードに固定した1つの形で`IR`を読む。

### REQ-core-090: スキーマのファイルを読まない

- 種類: prohibition
- 出典: docs/decision/records/records.md#R4, docs/decision/records/2026-09-17-check-reach.md#A3, docs/decision/records/2026-09-17-check-reach.md#A4
- 検証: unit

kotowari は、`IR`の形を宣言したスキーマのファイルを読んではならない。

### REQ-core-091: 外部の mdschema を使わない

- 種類: prohibition
- 出典: docs/decision/records/records.md#R5, docs/decision/records/2026-09-17-check-reach.md#A3, docs/decision/records/2026-09-17-check-reach.md#A4, docs/decision/records/2026-09-17-check-reach.md#A18
- 検証: unit

kotowari は、外部の mdschema を検査の前段に使ってはならない。

### REQ-core-168: スキーマはバイナリに埋め込み、文書の種類で選ぶ

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-22-ir-engine.md#A8, docs/decision/records/2026-09-22-ir-engine.md#A36
- 検証: unit

kotowari は常に、`IR`の形を宣言したスキーマをコンパイル時に取り込んで持ち、実行時にスキーマのファイルを読まず、`用語集`と`問題の記録`と`話題ごとの文書`の別に応じて取り込んだスキーマを選ぶ。`IR`の文書はスキーマを宣言せず、kotowari は文書の先頭の frontmatter を形の宣言として使わない。

### REQ-core-169: 形の読み取りを自前で持たない

- 種類: prohibition
- 出典: docs/decision/records/2026-09-22-ir-engine.md#A1, docs/decision/records/2026-09-22-ir-engine.md#A23, docs/decision/records/2026-09-22-ir-engine.md#A29, docs/decision/records/2026-09-22-ir-engine.md#A37, docs/decision/records/2026-09-23-ir-engine-gaps.md#A9, docs/decision/records/2026-09-23-ir-engine-gaps.md#A13, docs/decision/records/2026-09-23-ir-engine-gaps.md#A18
- 検証: review
- 確かめ方: `crates/kotowari-core/src/` の下のすべてのモジュール（`ir.rs` と `query.rs` を含む）の関数を見て、`IR`の文書の生の行を読む関数が gherkin の塊の中身と閉じない`コードブロック`の検出の2つに対応するものだけであり、見出し、"- 名前:" の行、Markdown の表、`題名`と`文書が扱う範囲`、`文`、`項目`の本文の範囲を生の行から決める関数が無いことを確認する。query の本文は、スキーマの側が返す`項目`の見出しの行と最後の行の間の行を切り出すだけであることを確認する

kotowari は、`IR`の文書の Markdown の構造を自前で読んではならない。生の行を読んでよいのは、gherkin の塊の中身と、閉じない`コードブロック`の検出の2つだけである。この禁止はモジュールを問わず、query が`項目`の本文の範囲を生の行の "### " と "## " から決めることも含む。

### REQ-core-170: 指摘の行のためにスキーマへ宣言するもの

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-22-ir-engine.md#A6, docs/decision/records/2026-09-22-ir-engine.md#A37, docs/decision/records/2026-09-22-ir-engine.md#A67, docs/decision/records/2026-09-22-ir-engine.md#A72, docs/decision/records/2026-09-23-ir-engine-gaps.md#A18, docs/decision/records/2026-09-23-ir-engine-gaps.md#A24
- 検証: review
- 確かめ方: 置き換えの前後で、source_invalid、gherkin の中の`指摘`、unclosed_backtick、missing_document を出す既存のテストと、query の本文を確かめる既存のテストが通ることを見る

kotowari は常に、スキーマに`出典`の "- 出典:" の行の行番号と、`シナリオ`を包む`コードブロック`の開始行と、`文`の行番号と行の文字そのままと、`項目`の最後の行を取る宣言を置き、query の本文の範囲を`項目`の最後の行から作り、source_invalid の "line"、gherkin の中の`指摘`の文書の先頭から数えた "line"、unclosed_backtick の detail をそこから作り、`文書名の参照`の走査もその`文`の行の文字の上で行う。

### REQ-core-179: 取り込んだスキーマの読み方と選び方の宣言

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-23-ir-engine-gaps.md#A13, docs/decision/records/2026-09-23-ir-engine-gaps.md#A15, docs/decision/records/2026-09-23-ir-engine-gaps.md#A18, docs/decision/records/2026-09-23-ir-engine-gaps.md#A20, docs/decision/records/2026-09-23-ir-engine-gaps.md#A24, docs/decision/records/2026-09-23-ir-engine-gaps.md#A28, docs/decision/records/2026-09-23-ir-engine-gaps.md#A12, docs/decision/records/2026-09-23-ir-engine-gaps.md#A31, docs/decision/records/2026-09-23-ir-engine-gaps.md#A32
- 検証: review
- 確かめ方: `.mds/schemas/ir.yaml`、`.mds/schemas/context.yaml`、`.mds/schemas/flags.yaml` を読み、3つとも最上位に "reading: line" を書いていること、flags.yaml が`問題の記録`の`項目`を文書の直下（"document.item"、"flags"）と "## 問題の記録" の節の下（"flags_in_section"）の両方に宣言していること、context.yaml の表の規則が "header: [用語, 意味, 出典]" と "select: first" を書いていること、ir.yaml の3種類の`項目`と flags.yaml の`問題の記録`の`項目`の抽出が "end" を取ることを確認する

kotowari は常に、取り込んだ3つのスキーマ（`話題ごとの文書`、`用語集`、`問題の記録`）に "reading: line" を宣言し、`問題の記録`のスキーマには`問題の記録`の`項目`を文書の直下（"flags" に抽出する）と "## 問題の記録" の節の下（"flags_in_section" に抽出する）の両方に宣言し、`用語集`のスキーマには表のヘッダを "用語"、"意味"、"出典" と宣言して "select: first" を添え、`話題ごとの文書`と`問題の記録`のスキーマには`項目`の最後の行（"end"）を取る宣言を置く。エンジンの既定の読み方には頼らない。

### REQ-core-173: kotowari に残す検査

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-22-ir-engine.md#A1, docs/decision/records/2026-09-22-ir-engine.md#A4, docs/decision/records/2026-09-22-ir-engine.md#A23, docs/decision/records/2026-09-22-ir-engine.md#A24, docs/decision/records/2026-09-22-ir-engine.md#A30, docs/decision/records/2026-09-22-ir-engine.md#A37, docs/decision/records/2026-09-22-ir-engine.md#A83, docs/decision/records/2026-09-22-ir-engine.md#P1, docs/decision/records/2026-09-21-mds-spec.md#A6, docs/decision/records/2026-09-22-id-namespace.md#A3, docs/decision/records/records.md#A88, docs/decision/records/ir-form.md#文書名の参照
- 検証: review
- 確かめ方: 置き換えの前後で、これらの`指摘`を出す既存のテストが通ることと、同じ判定がスキーマの側にも宣言されていないことを確認する

kotowari は常に、文書をまたぐ検査（`ID`の重複、`用語`の重複、参照切れ、`ID`の名前と置き場の一致）、行数と`要求`の数の上限、`出典`の検査（TBL-core-012）、`文`に基づく検査（`用語`、`曖昧語`、閉じないバッククォート）、`文書名の参照`、gherkin の中身、閉じない`コードブロック`、`用語集`の行の`用語`のセルが空であることの検査を自分の側で行い、スキーマへ移さない。

### REQ-core-177: 読み取りの置き換えで出力が変わらない

- 種類: invariant
- 出典: docs/decision/records/2026-09-22-ir-engine.md#A5, docs/decision/records/2026-09-22-ir-engine.md#A38, docs/decision/records/2026-09-22-ir-engine.md#A85
- 検証: review
- 確かめ方: 置き換えの前に `kotowari check --format json` の出力を保存し、後の出力と `findings` の並びだけを取り出して `diff` を取り、差分が1件も無いことを見る

読み取りをスキーマに置き換える前と後で、この`IR`の置き場に対する "kotowari check --format json" の`指摘`の並びが1件も変わらない関係が常に成り立つ。種類・detail・"line"・出る順のどれも変わらない。文書そのものを直したことで変わる文書の数と行数の数え上げは、この関係が述べる対象ではない。ほかの`IR`の置き場での差も対象ではない。

## 具体例

```gherkin
@id=EX-core-040 @about=REQ-core-090,REQ-core-168 @source=docs/decision/records/2026-09-17-check-reach.md#A4,docs/decision/records/2026-09-17-check-reach.md#A28
Scenario: スキーマのファイルを置いても読まれない
  Given ".kotowari/schema.yaml" に壊れた YAML がある
  When "kotowari check" を実行する
  Then `停止`せず、置く前と同じ終了コードと標準出力になる

@id=EX-core-041 @about=REQ-core-091 @source=docs/decision/records/2026-09-17-check-reach.md#A18,docs/decision/records/2026-09-17-check-reach.md#A28
Scenario: 外部のコマンドが見つからなくても結果は変わらない
  Given 環境変数 "PATH" が空のディレクトリだけを指す
  When "kotowari check" を実行する
  Then "PATH" をそのままにしたときと同じ終了コード、標準出力、標準エラーになる

@id=EX-core-264 @about=REQ-core-168 @source=docs/decision/records/2026-09-22-ir-engine.md#A36,docs/decision/records/2026-09-22-ir-engine.md#A8
Scenario: スキーマの置き場を消しても出力が変わらない
  Given `IR`のどの文書にも先頭の frontmatter が無く、スキーマのファイルの置き場がある
  When その置き場を退避して "kotowari check --format json" を実行する
  Then 退避する前と同じ終了コードと標準出力になる
```
