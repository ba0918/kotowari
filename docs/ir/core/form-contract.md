# IR の形の固定

IR の形をどこで決めるか、その形を誰が読むか、採らない形の決め方を扱う。

## 要求

### REQ-core-089: 形はコードに固定する

- 種類: ubiquitous
- 出典: docs/decision/records/records.md#A25, docs/decision/records/records.md#A52, docs/decision/records/records.md#A98
- 検証: review
- 確かめ方: `crates/kotowari-core/src/ir.rs` にコードで固定した形で IR を読むことを確認。parse_document 関数

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
- 出典: docs/decision/records/2026-09-22-ir-engine.md#A1, docs/decision/records/2026-09-22-ir-engine.md#A23, docs/decision/records/2026-09-22-ir-engine.md#A29, docs/decision/records/2026-09-22-ir-engine.md#A37
- 検証: review
- 確かめ方: `crates/kotowari-core/src/ir.rs` の関数の一覧を見て、生の行を読む関数が gherkin の塊の中身と閉じない`コードブロック`の検出の2つに対応するものだけであり、見出し、"- 名前:" の行、Markdown の表、`題名`と`文書が扱う範囲`を読む関数が無いことを確認する

kotowari は、`IR`の文書の Markdown の構造を自前で読んではならない。生の行を読んでよいのは、gherkin の塊の中身と、閉じない`コードブロック`の検出の2つだけである。

### REQ-core-170: 指摘の行のためにスキーマへ宣言するもの

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-22-ir-engine.md#A6, docs/decision/records/2026-09-22-ir-engine.md#A37, docs/decision/records/2026-09-22-ir-engine.md#A67, docs/decision/records/2026-09-22-ir-engine.md#A72
- 検証: review
- 確かめ方: 置き換えの前後で、source_invalid、gherkin の中の`指摘`、unclosed_backtick、missing_document を出す既存のテストが通ることを見る

kotowari は常に、スキーマに`出典`の "- 出典:" の行の行番号と、`シナリオ`を包む`コードブロック`の開始行と、`文`の行番号と行の文字そのままを取る宣言を置き、source_invalid の "line"、gherkin の中の`指摘`の文書の先頭から数えた "line"、unclosed_backtick の detail をそこから作り、`文書名の参照`の走査もその`文`の行の文字の上で行う。

### REQ-core-173: kotowari に残す検査

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-22-ir-engine.md#A1, docs/decision/records/2026-09-22-ir-engine.md#A4, docs/decision/records/2026-09-22-ir-engine.md#A23, docs/decision/records/2026-09-22-ir-engine.md#A24, docs/decision/records/2026-09-22-ir-engine.md#A30, docs/decision/records/2026-09-22-ir-engine.md#A37, docs/decision/records/2026-09-22-ir-engine.md#P1, docs/decision/records/2026-09-21-mds-spec.md#A6, docs/decision/records/2026-09-22-id-namespace.md#A3, docs/decision/records/records.md#A88, docs/decision/records/ir-form.md#文書名の参照
- 検証: review
- 確かめ方: 置き換えの前後で、これらの`指摘`を出す既存のテストが通ることと、同じ判定がスキーマの側にも宣言されていないことを確認する

kotowari は常に、文書をまたぐ検査（`ID`の重複、`用語`の重複、参照切れ、`ID`の名前と置き場の一致）、行数と`要求`の数の上限、`出典`の検査（TBL-core-012）、`文`に基づく検査（`用語`、`曖昧語`、閉じないバッククォート）、`文書名の参照`、gherkin の中身、閉じない`コードブロック`の検査を自分の側で行い、スキーマへ移さない。

### REQ-core-177: 読み取りの置き換えで変わる指摘は3種類だけ

- 種類: invariant
- 出典: docs/decision/records/2026-09-22-ir-engine.md#A5, docs/decision/records/2026-09-22-ir-engine.md#A38, docs/decision/records/2026-09-22-ir-engine.md#A72, docs/decision/records/2026-09-22-ir-engine.md#A75
- 検証: review
- 確かめ方: 置き換えの前に `kotowari check --format json` の出力を保存し、後の出力と `diff` を取り、差分が下の8つの範囲に収まることを見る。このリポジトリの`IR`では差分が1件も出ないことも見る

読み取りをスキーマに置き換える前と後で、"kotowari check --format json" の出力の差分が次の8つだけであり、ほかの`指摘`の種類・detail・"line"・出る順に差が無い関係が常に成り立つ。REQ-core-174 の3種類の`指摘`の追加。TBL-core-014 が対象から外した行に書かれていた`文書名の参照`の missing_document の減少。宣言の無い "## " の見出しとその下の行に出る unknown_heading と unknown_line。最初の "## " の見出しより前にある "### " の見出しに出る unknown_heading。"## " の見出しの名前と`項目`の`ID`の接頭辞が食い違う見出しに出る unknown_heading。値が空の "- 種類:" のような行が、行の欠落ではなく値の誤りとして扱われること。同じ名前の "- xxx:" の行が3本以上あるときの`指摘`が、2つ目以降ごとではなく1件になること。`問題の記録`の`項目`に`文`が無いときに出る missing_statement。

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
