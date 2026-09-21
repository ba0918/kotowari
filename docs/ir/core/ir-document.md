# 文書の読み方と文書全体の検査

IR の文書の選び方、題名と範囲の行、行の数え方、行数と要求の数の注意を扱う。

## 要求

### REQ-core-033: 読む文書

- 種類: ubiquitous
- 出典: docs/decision/records/records.md#A32, docs/decision/records/records.md#A102, docs/decision/records/records.md#A165, docs/decision/records/2026-09-16-ir-tree.md#A1, docs/decision/records/2026-09-16-ir-tree.md#A8, docs/decision/records/2026-09-16-ir-tree.md#A13
- 検証: unit

kotowari は常に、`IR`の置き場の下のディレクトリを深さに制限なく辿り、拡張子が小文字の ".md" のファイルだけを読み、".MD" の文書、ディレクトリでも通常のファイルでもないもの（ソケット、名前付きパイプ、デバイス）を読まない（`除外`）。隠しディレクトリとディレクトリのシンボリックリンクはどの深さでも辿らず（`除外`）、空のディレクトリには`指摘`を出さない。どのディレクトリでも "CONTEXT.md" は`用語集`、"FLAGS.md" は`問題の記録`である。ファイルのシンボリックリンクは読む。種類を取れない要素があるときは読めないファイルを理由に`停止`する。

### REQ-core-034: 題名が無い

- 種類: event_driven
- 出典: docs/decision/records/records.md#A42, docs/decision/records/records.md#A56
- 検証: unit

`IR`の文書に`題名`が無いとき、kotowari は missing_title の`誤り`を出す。

### REQ-core-035: 題名が複数

- 種類: event_driven
- 出典: docs/decision/records/records.md#A42
- 検証: unit

`IR`の文書に`題名`が2つ以上あるとき、kotowari は multiple_titles の`誤り`を出す。

### REQ-core-036: 範囲の行が無い

- 種類: event_driven
- 出典: docs/decision/records/records.md#A30, docs/decision/records/records.md#A41, docs/decision/records/records.md#A55, docs/decision/records/records.md#A56, docs/decision/records/ir-form.md#検査の種類
- 検証: unit

`話題ごとの文書`に`文書が扱う範囲`の行が1行も無いとき、kotowari は missing_scope の`誤り`を出す。

### REQ-core-037: 行の数え方

- 種類: algorithm
- 出典: docs/decision/records/records.md#A33
- 定義: TBL-core-010
- 検証: unit

### REQ-core-038: 行数の上限

- 種類: event_driven
- 出典: docs/decision/records/records.md#A17, docs/decision/records/records.md#A29, docs/decision/records/records.md#A41, docs/decision/records/records.md#A56, docs/decision/records/records.md#A47, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-16-notice.md#A2
- 検証: unit

`IR`の文書の行数が "limits.lines" を超えるとき、kotowari は too_many_lines の`注意`を出す。

### REQ-core-039: 要求の数の上限

- 種類: event_driven
- 出典: docs/decision/records/records.md#A17, docs/decision/records/records.md#A29, docs/decision/records/records.md#A41, docs/decision/records/records.md#A47, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-16-notice.md#A2
- 検証: unit

`話題ごとの文書`の`要求`の数が "limits.requirements" を超えるとき、kotowari は too_many_requirements の`注意`を出す。

### REQ-core-040: コードブロックの中

- 種類: ubiquitous
- 出典: docs/decision/records/records.md#A52, docs/decision/records/records.md#A88, docs/decision/records/records.md#A108, docs/decision/records/records.md#A140
- 検証: unit

kotowari は常に、`コードブロック`の中を検査の対象から外す（`除外`）。閉じていない`コードブロック`は gherkin でも対象から外す。閉じた gherkin のブロックの中の行は、`シナリオ`のタグと`用語`と曖昧語の検査の対象にし、文書名の参照の検査では対象にしない。

### REQ-core-041: 範囲の中身と責務の分離を見ない

- 種類: prohibition
- 出典: docs/decision/records/records.md#A17, docs/decision/records/records.md#A30
- 検証: review
- 確かめ方: `crates/kotowari-core/src/ir.rs` で scope_lines の中身を検査せず存在だけ確認。`check_documents` に範囲の内容検査がないことを確認

kotowari は、`文書が扱う範囲`の中身と行数を検査すること、文書の責務の分離を判定することをしてはならない。

## 決定表

### TBL-core-010: 行の数え方

- 出典: docs/decision/records/records.md#A33, docs/decision/records/records.md#A129

| 場面 | 数え方 |
|---|---|
| "\n" | 1つの行の終わり |
| "\r\n" | 1つの行の終わり（1行に数える） |
| 最後の行に改行が無い | その行も1行に数える |
| 中身が空の文書 | 0行に数える（題名が無いので missing_title を出す） |

## 具体例

```gherkin
@id=EX-core-006 @about=REQ-core-036 @source=docs/decision/records/records.md#A41,docs/decision/records/ir-form.md#検査の種類
Scenario: 用語集は範囲の行が無くてもよい
  Given `用語集`に`題名`と表だけがある
  When "kotowari check" を実行する
  Then `用語集`に missing_scope の誤りは出ない

@id=EX-core-007 @about=REQ-core-037 @source=docs/decision/records/records.md#A33
Scenario: 改行の違いで行数は変わらない
  Given "a\r\nb" と書いた文書がある
  When その文書の行数を数える
  Then 行数は 2 である

@id=EX-core-020 @about=REQ-core-033 @source=docs/decision/records/2026-09-16-ir-tree.md#A1,docs/decision/records/2026-09-16-ir-tree.md#A13,docs/decision/records/records.md#A21
Scenario: 深いディレクトリの文書も読む
  Given "docs/ir/network/dns/timeout.md" に検証が "unit" で`印`の無い`要求`が1つあり、"docs/ir/network/empty/" は空のディレクトリである
  When "kotowari check" を実行する
  Then path が "docs/ir/network/dns/timeout.md" の requirement_without_test の誤りが出て、空のディレクトリに`指摘`は出ない

@id=EX-core-030 @about=REQ-core-033 @source=docs/decision/records/2026-09-16-ir-tree.md#A13
Scenario: 深いディレクトリの中でも隠しディレクトリとディレクトリのシンボリックリンクは辿らない
  Given "docs/ir/network/.draft/a.md" と、"docs/ir/network/link" が "docs/ir/" を指すディレクトリのシンボリックリンクである
  When "kotowari check" を実行する
  Then "docs/ir/network/.draft/a.md" と "docs/ir/network/link/" の下の文書は読まれず、`指摘`も`停止`も出ない

@id=EX-core-021 @about=REQ-core-033 @source=docs/decision/records/2026-09-16-ir-tree.md#A8,docs/decision/records/records.md#A41,docs/decision/records/records.md#A56
Scenario: サブディレクトリの CONTEXT.md と FLAGS.md も用語集と問題の記録になる
  Given "docs/ir/network/CONTEXT.md" と "docs/ir/network/FLAGS.md" がある
  When "kotowari check" を実行する
  Then どちらにも missing_scope の誤りは出ない
```
