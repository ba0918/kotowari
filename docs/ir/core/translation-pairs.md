# 対の読み方と一致の記録

`言語の一覧`の読み方、`対`にする文書と`側`の見分け方、欠けた`側`、`一致の記録`と blob hash の照合、`対`を既にある検査でどう扱うかを扱う。`骨組み`の一致、`切り替えの行`、リンクの検査は translation-structure.md が、`UI の文字`は overview-languages.md が扱う。

## Requirements

### REQ-core-334: 言語の一覧

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A16, docs/decision/records/2026-10-05-localization.md#A21, docs/decision/records/2026-10-05-localization.md#A31, docs/decision/records/2026-10-05-localization.md#A27, docs/decision/records/2026-10-05-localization.md#A33
- verification: unit

kotowari は常に、`設定ファイル`の "languages" の値を`言語の一覧`として読み、その最初の言語を`先頭の言語`とする。`言語の一覧`の言語が1つのとき、kotowari は`対`を読まず、translation_missing、translation_record_invalid、translation_stale、translation_structure_mismatch、translation_switcher_invalid、link_language_mismatch、link_to_record の`指摘`を出さない。

### REQ-core-335: 言語の一覧の誤り

- kind: event_driven
- source: docs/decision/records/2026-10-05-localization.md#A21
- verification: unit

`設定ファイル`の "languages" の要素に、空の文字列か、小文字の英字、数字、"-" のほかの文字を含む文字列があるとき、または同じ言語タグが2回以上あるとき、kotowari は設定の誤りを理由に`停止`する。

### REQ-core-336: 対にする文書

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A11, docs/decision/records/2026-10-05-localization.md#A26, docs/decision/records/2026-10-05-localization.md#A38, docs/decision/records/2026-10-05-localization.md#A16, docs/decision/records/2026-10-05-localization.md#A21
- verification: unit

kotowari は常に、`言語の一覧`の言語が2つ以上のとき、`IR`の置き場の文書（`用語集`と`問題の記録`を含む）、`ガイド`、`全体像の元データ`、`目次`を`対`として読む。"decisions.records" と "decisions.adr" の置き場のファイルは`対`として読まない。

### REQ-core-337: 側の見分け方

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A15, docs/decision/records/2026-10-05-localization.md#A31, docs/decision/records/2026-10-05-localization.md#A39, docs/decision/records/2026-10-05-localization.md#D2, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

kotowari は常に、`対`にする置き場で読むファイルのうち、ファイル名の最後の "." からを拡張子とし、拡張子の前が "<幹>.<言語タグ>" の形で、言語タグが`先頭の言語`でない`言語の一覧`の言語であるものを、同じディレクトリの "<幹><拡張子>" の`対`のその言語の`側`として読み、ほかのファイルを`先頭の言語`の`側`として読む。`先頭の言語`の言語タグと`言語の一覧`に無い言語タグを持つ名前は、`先頭の言語`の`側`の名前の一部として読む。"<幹>.i18n.yaml" の名前のファイルは`側`として読まない。1つの`対`が`IR`と`ガイド`のように2つ以上の置き場から読まれても、`対`の`指摘`は1回だけ出す。ほかの言語の`側`は、`先頭の言語`の`側`と同じディレクトリで名前によって探し、"guides.files" と "overview.files" の glob に当たるかを問わない。

### REQ-core-338: 欠けた側

- kind: event_driven
- source: docs/decision/records/2026-10-05-localization.md#A17, docs/decision/records/2026-10-05-localization.md#A27, docs/decision/records/2026-10-05-localization.md#D1, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

`対`の`先頭の言語`の`側`があり、ほかの言語の`側`が無いとき、kotowari は "path" を`先頭の言語`の`側`、"line" を null、detail を無い`側`の`基準のディレクトリ`からの相対パスにして、translation_missing の`誤り`を無い`側`ごとに出す。ほかの言語の`側`があり`先頭の言語`の`側`が無いとき、kotowari は "path" をその`側`、"line" を null、detail を無い`先頭の言語`の`側`の`基準のディレクトリ`からの相対パスにして translation_missing の`誤り`を出し、その`側`をほかの検査で読まない。

### REQ-core-339: 一致の記録の形

- kind: event_driven
- source: docs/decision/records/2026-10-05-localization.md#A15, docs/decision/records/2026-10-05-localization.md#A27, docs/decision/records/2026-10-05-localization.md#A28, docs/decision/records/2026-10-05-localization.md#A18, docs/decision/records/2026-10-05-localization.md#D1, docs/decision/records/2026-10-05-localization.md#D3, docs/decision/records/2026-10-05-localization.md#A26, docs/decision/records/2026-10-05-localization.md#A39
- verification: unit

`先頭の言語`の`側` "<幹><拡張子>" のある`対`について、同じディレクトリの "<幹>.i18n.yaml" が無いとき、YAML として読めないとき、文字列の鍵から文字列の値への対応表でないとき、鍵の集まりが`言語の一覧`の各言語の`側`のファイル名（ディレクトリを除く）の集まりと同じでないとき、または値に40文字の16進の小文字でないものがあるとき、kotowari は "path" を "<幹>.i18n.yaml" の`基準のディレクトリ`からの相対パス、"line" を null にして translation_record_invalid の`誤り`を1件出す。detail は、無ければ "missing"、YAML として読めなければ "yaml"、対応表でないか鍵の集まりが違えば "keys"、値の形が違えば "value" とし、2つ以上に当たるときはこの順で先のものにする。

### REQ-core-340: blob hash

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A18
- verification: unit

kotowari は常に、ファイルの blob hash を、"blob "、ファイルのバイト数の10進、1バイトの 0、ファイルのバイト列をこの順につないだバイト列の SHA-1 を40文字の16進の小文字で書いたもの（"git hash-object" がそのファイルに出す値と同じ）とする。

### REQ-core-341: 古い側

- kind: event_driven
- source: docs/decision/records/2026-10-05-localization.md#A18, docs/decision/records/2026-10-05-localization.md#A27, docs/decision/records/2026-10-05-localization.md#D2, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

`対`に translation_record_invalid の`誤り`の無い`一致の記録`があり、ファイルのある`側`の今の blob hash が`一致の記録`のその`側`のファイル名の値と違うとき、kotowari は "path" をその`側`、"line" を null、detail を「`一致の記録`の値、今の blob hash」を1つの半角空白で区切った文字列にして、translation_stale の`誤り`を`側`ごとに出す。

### REQ-core-342: IR の対を1つの文書として扱う

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A22, docs/decision/records/2026-10-05-localization.md#A23, docs/decision/records/2026-10-05-localization.md#A24, docs/decision/records/2026-10-05-localization.md#D2, docs/decision/records/2026-10-05-localization.md#A34, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

kotowari は常に、`IR`の`対`について、`項目`と`シナリオ`の読み取り、`ID`の重複、`出典`と`ID`の参照の検査、テストの`印`との照合、`指紋`、"kotowari list" の "items"、"kotowari query" の出力、`面`の判定を`先頭の言語`の`側`だけで行い、ほかの言語の`側`の`項目`と`シナリオ`はほかの言語のページの本文（REQ-core-354）を取るときだけ読む。既にある検査のうち、ほかの言語の`側`に行うのは、`用語`、`曖昧語`、`文書名の参照`の検査（REQ-core-063 から REQ-core-070）、閉じないバッククォート（REQ-core-116）、`用語集`の形の検査（REQ-core-117、REQ-core-122、REQ-core-123、REQ-core-174）だけで、`用語`はその言語の`側`の`用語集`（"CONTEXT.<言語タグ>.md"）の`連鎖`から引き、`曖昧語`は設定の "vague_words" の1つの一覧で調べる。check と status の出力の "files" と "lines" には、すべての`側`を数える。

### REQ-core-343: ガイドと全体像の対

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A22, docs/decision/records/2026-10-05-localization.md#A26, docs/decision/records/2026-10-05-localization.md#A31, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

kotowari は常に、`ガイド`の`対`のすべての`側`を`ガイド`として読み、`ガイドの印`の検査と guide_stale の`注意`をどの`側`にも出す。`全体像の元データ`の`対`のすべての`側`に、`全体像の元データ`の形、`部品`、冒頭の lead、参照、`ガイドの印`の検査（REQ-core-281 から REQ-core-283、REQ-core-285、REQ-core-286）を行い、扱う IR の文書の検査（REQ-core-284）、ページの名前の重なり（REQ-core-305）、`目次`との照合（REQ-core-328、REQ-core-329）は`先頭の言語`の`側`だけで行う。`目次`の`対`のすべての`側`に`目次`の形の検査（REQ-core-327、REQ-core-330）を行う。

## Examples

```gherkin
@id=EX-core-512 @about=REQ-core-334 @source=docs/decision/records/2026-10-05-localization.md#A16,docs/decision/records/2026-10-05-localization.md#A27
Scenario: 言語が1つなら対を読まない
  Given 設定の "languages" が無く、`IR`の置き場に "a.md" と "a.en.md" がある
  When "kotowari check --format json" を実行する
  Then translation_missing の`指摘`は出ず、"a.en.md" は`先頭の言語`の`側`として読まれる

@id=EX-core-513 @about=REQ-core-335 @source=docs/decision/records/2026-10-05-localization.md#A21
Scenario: 言語タグの形が違えば止まる
  Given 設定の "languages" が "[ja, EN]" である
  When "kotowari check" を実行する
  Then 終了コードは 2 で、設定の誤りで`停止`する

@id=EX-core-514 @about=REQ-core-336,REQ-core-337,REQ-core-338 @source=docs/decision/records/2026-10-05-localization.md#A11,docs/decision/records/2026-10-05-localization.md#A15,docs/decision/records/2026-10-05-localization.md#A17,docs/decision/records/2026-10-05-localization.md#A27,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: 英語の側が無ければ誤りになる
  Given 設定の "languages" が "[ja, en]" で、`IR`の置き場に "a.md" があり "a.en.md" が無い
  When "kotowari check --format json" を実行する
  Then "path" が "docs/ir/a.md"、detail が "docs/ir/a.en.md" の translation_missing の`誤り`が出る

@id=EX-core-515 @about=REQ-core-338 @source=docs/decision/records/2026-10-05-localization.md#A17,docs/decision/records/2026-10-05-localization.md#A27,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: 先頭の言語の側が無ければその側を誤りにする
  Given 設定の "languages" が "[ja, en]" で、`IR`の置き場に "b.en.md" だけがある
  When "kotowari check --format json" を実行する
  Then "path" が "docs/ir/b.en.md"、detail が "docs/ir/b.md" の translation_missing の`誤り`が出る

@id=EX-core-516 @about=REQ-core-337 @source=docs/decision/records/2026-10-05-localization.md#A31,docs/decision/records/2026-10-05-localization.md#A17,docs/decision/records/2026-10-05-localization.md#A27,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: 先頭の言語や一覧に無い言語の接尾辞は名前の一部になる
  Given 設定の "languages" が "[ja, en]" で、`IR`の置き場に "c.ja.md" と "c.fr.md" があり、"c.ja.en.md" と "c.fr.en.md" が無い
  When "kotowari check --format json" を実行する
  Then detail が "docs/ir/c.ja.en.md" と "docs/ir/c.fr.en.md" の translation_missing の`誤り`が出る

@id=EX-core-517 @about=REQ-core-336 @source=docs/decision/records/2026-10-05-localization.md#A11,docs/decision/records/2026-10-05-localization.md#A38,docs/decision/records/2026-10-05-localization.md#A27
Scenario: 判断の記録は対にしない
  Given 設定の "languages" が "[ja, en]" で、"docs/decision/records/r.md" があり "r.en.md" が無い
  When "kotowari check --format json" を実行する
  Then "docs/decision/records/r.md" に translation_missing の`指摘`は出ない

@id=EX-core-518 @about=REQ-core-339,REQ-core-340,REQ-core-341 @source=docs/decision/records/2026-10-05-localization.md#A18,docs/decision/records/2026-10-05-localization.md#A28,docs/decision/records/2026-10-05-localization.md#A27
Scenario: 記録の hash が今のファイルと同じなら何も出ない
  Given 設定の "languages" が "[ja, en]" で、"guides/a.md" と "guides/a.en.md" の中身がどちらも "a" と改行の2バイトである
  And "guides/a.i18n.yaml" が "a.md" と "a.en.md" の両方に "78981922613b2afb6025042ff6bd878ac1994e85" を持つ
  When "kotowari check --format json" を実行する
  Then "guides/a.md" と "guides/a.en.md" に translation_stale の`指摘`は出ない

@id=EX-core-519 @about=REQ-core-341 @source=docs/decision/records/2026-10-05-localization.md#A18,docs/decision/records/2026-10-05-localization.md#A27,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: 片方だけ直すと古い側の誤りになる
  Given EX-core-518 のファイルがあり、"guides/a.md" の中身を空にする
  When "kotowari check --format json" を実行する
  Then "path" が "guides/a.md"、detail が "78981922613b2afb6025042ff6bd878ac1994e85 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391" の translation_stale の`誤り`が出る

@id=EX-core-520 @about=REQ-core-339 @source=docs/decision/records/2026-10-05-localization.md#A28,docs/decision/records/2026-10-05-localization.md#A27,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: 記録の鍵が側の名前と違えば誤りになる
  Given 設定の "languages" が "[ja, en]" で、"guides/a.md" と "guides/a.en.md" があり、"guides/a.i18n.yaml" の鍵が "a.md" と "a.zh.md" である
  When "kotowari check --format json" を実行する
  Then "path" が "guides/a.i18n.yaml"、detail が "keys" の translation_record_invalid の`誤り`が出て、translation_stale の`指摘`は出ない

@id=EX-core-521 @about=REQ-core-339 @source=docs/decision/records/2026-10-05-localization.md#A27,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: 記録が無ければ誤りになる
  Given 設定の "languages" が "[ja, en]" で、"guides/a.md" と "guides/a.en.md" があり、"guides/a.i18n.yaml" が無い
  When "kotowari check --format json" を実行する
  Then "path" が "guides/a.i18n.yaml"、detail が "missing" の translation_record_invalid の`誤り`が出る

@id=EX-core-522 @about=REQ-core-342 @source=docs/decision/records/2026-10-05-localization.md#A22,docs/decision/records/2026-10-05-localization.md#A24
Scenario: 英語の側の要求は数えず、用語は英語の用語集から引く
  Given 設定の "languages" が "[ja, en]" で、`IR`の "a.md" と "a.en.md" が同じ要求 "REQ-001" を持ち、"a.en.md" の文に用語 "term" をバッククォートで囲んで書く
  And "CONTEXT.en.md" に用語 "term" があり、"CONTEXT.md" には無い
  When "kotowari check --format json" を実行する
  Then duplicate_id と unknown_term の`指摘`は出ず、"kotowari list" の "items" の "REQ-001" は "path" が "docs/ir/a.md" の1件だけである

@id=EX-core-523 @about=REQ-core-342 @source=docs/decision/records/2026-10-05-localization.md#A22
Scenario: 英語の側の用語は日本語の用語集では足りない
  Given EX-core-522 と同じ文書があり、用語 "term" が "CONTEXT.md" にあって "CONTEXT.en.md" に無い
  When "kotowari check --format json" を実行する
  Then "docs/ir/a.en.md" に unknown_term の`誤り`が出る

@id=EX-core-524 @about=REQ-core-343 @source=docs/decision/records/2026-10-05-localization.md#A22
Scenario: ガイドの印の古さはどの側にも出る
  Given 設定の "languages" が "[ja, en]" で、"guides/a.md" と "guides/a.en.md" に同じ古い`ガイドの印`がある
  When "kotowari check --format json" を実行する
  Then "guides/a.md" と "guides/a.en.md" のそれぞれに guide_stale の`注意`が1件出る

@id=EX-core-525 @about=REQ-core-343 @source=docs/decision/records/2026-10-05-localization.md#A22,docs/decision/records/2026-10-05-localization.md#A31
Scenario: 全体像の英語の側は別のページの名前として数えない
  Given 設定の "languages" が "[ja, en]" で、`全体像の元データ` "a.md" と "a.en.md" があり、`目次`の名前の項目は "a" だけである
  When "kotowari check --format json" を実行する
  Then overview_name_conflict と overview_toc_page_missing と overview_ir_shared の`指摘`は出ない
```
