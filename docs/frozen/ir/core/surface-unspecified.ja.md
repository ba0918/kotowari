# 未記載の面の一覧

[English](surface-unspecified.md) | 日本語

`未記載の面の一覧`の置き場と形、`面`との一致の取り方、一覧の1件への`指摘`を扱う。

## Requirements

### REQ-core-231: 一覧の置き場

- kind: event_driven
- source: docs/decision/records/2026-09-27-surface-check.md#A4, docs/decision/records/2026-09-27-surface-check.md#A6, docs/decision/records/2026-09-27-surface-check.md#A22, docs/decision/records/2026-09-27-surface-check.md#A24
- verification: unit

"kotowari check" か "kotowari status" で "surface.rules" が空の一覧でなく、設定に "surface.unspecified" の鍵が無いとき、または指す先が空（0バイトか注釈だけ）のとき、kotowari は`未記載の面の一覧`を0件として続ける。鍵の指す先が無いか読めないとき、kotowari は読めないファイルを理由に`停止`する。指す先が UTF-8 でないとき、kotowari は UTF-8 でないファイルを理由に`停止`する。指す先が YAML として読めないとき、または最上位が並びでないとき、kotowari は設定の誤りを理由に`停止`し、詳細に`未記載の面の一覧`のファイルの相対パスを出す。"surface.rules" が空の一覧で "surface.unspecified" の鍵があるときは REQ-core-225 のとおり`停止`する。

### REQ-core-232: 一覧の1件との一致

- kind: ubiquitous
- source: docs/decision/records/2026-09-27-surface-check.md#A4, docs/decision/records/2026-09-27-surface-check.md#A6, docs/decision/records/2026-09-27-surface-check.md#A22
- verification: unit

kotowari は常に、形の正しい`未記載の面の一覧`の1件を、"kind" が`面`の種類と、"name" が`面`の名前と、前後の空白を除かずにどちらも同じ文字列のときにその`面`に一致とし、一致した`面`が`IR`になくても surface_without_spec の`誤り`を出さない。同じ内容の1件が2つ以上あること自体は検査しない。

### REQ-core-233: 形の誤った1件

- kind: event_driven
- source: docs/decision/records/2026-09-27-surface-check.md#A4, docs/decision/records/2026-09-27-surface-check.md#A6, docs/decision/records/2026-09-27-surface-check.md#A14, docs/decision/records/2026-09-27-surface-check.md#A15, docs/decision/records/2026-09-27-surface-check.md#A22
- verification: unit

"kotowari check" か "kotowari status" で、`未記載の面の一覧`の1件が鍵と値の組でないとき、"kind"、"name"、"why" のいずれかの鍵が無いとき、この3つ以外の鍵を持つとき、値が文字列でないとき、または "why" が前後の半角空白とタブを除いて空のとき、kotowari は "path" を`未記載の面の一覧`のファイル、"line" を null、detail を一覧に書かれたままの "kind" と "name" を1つの半角空白で区切った文字列にして surface_unspecified_invalid の`誤り`を1件ごとに出し、その1件をどの`面`とも一致させない。detail の "kind" と "name" は、無いか文字列でなければ空の文字列にする。

### REQ-core-234: 要らなくなった1件

- kind: event_driven
- source: docs/decision/records/2026-09-27-surface-check.md#A6, docs/decision/records/2026-09-27-surface-check.md#A14, docs/decision/records/2026-09-27-surface-check.md#A15, docs/decision/records/2026-09-27-surface-check.md#A22
- verification: unit

"kotowari check" か "kotowari status" で、形の正しい`未記載の面の一覧`の1件に一致する`面`が1つも無いとき、または一致する`面`が`IR`にあるとき、kotowari は "path" を`未記載の面の一覧`のファイル、"line" を null、detail を一覧に書かれたままの "kind" と "name" を1つの半角空白で区切った文字列にして surface_unspecified_stale の`注意`を1件ごとに出す。形の誤った1件には出さない。

## Examples

```gherkin
@id=EX-core-419 @about=REQ-core-232,REQ-core-228 @source=docs/decision/records/2026-09-27-surface-check.md#A4,docs/decision/records/2026-09-27-surface-check.md#A6,docs/decision/records/2026-09-27-surface-check.md#A15,docs/decision/records/2026-09-27-surface-check.md#A16,docs/decision/records/2026-09-27-surface-check.md#A22
Scenario: 一覧に載った面は誤りにならず、外した件数に数える
  Given EX-core-408 の場面で、`未記載の面の一覧`に "kind" が "flag"、"name" が "--verbose"、"why" が空でない1件がある
  When "kotowari check --format json" を実行する
  Then surface_without_spec の誤りは出ず、JSON の "surface" の "unspecified" は 1 で、終了コードは 0 である

@id=EX-core-420 @about=REQ-core-232 @source=docs/decision/records/2026-09-27-surface-check.md#A6,docs/decision/records/2026-09-27-surface-check.md#A15,docs/decision/records/2026-09-27-surface-check.md#A22
Scenario: 種類の違う1件は一致しない
  Given EX-core-419 の場面で、`未記載の面の一覧`の1件の "kind" が "subcommand" である
  When "kotowari check --format json" を実行する
  Then detail が "flag --verbose" の surface_without_spec の誤りが出る

@id=EX-core-421 @about=REQ-core-233 @source=docs/decision/records/2026-09-27-surface-check.md#A4,docs/decision/records/2026-09-27-surface-check.md#A6,docs/decision/records/2026-09-27-surface-check.md#A15,docs/decision/records/2026-09-27-surface-check.md#A22
Scenario: 理由が空白だけの1件は誤りで、面を外さない
  Given EX-core-419 の場面で、`未記載の面の一覧`の1件の "why" が半角空白だけである
  When "kotowari check --format json" を実行する
  Then "line" が null で detail が "flag --verbose" の surface_unspecified_invalid の誤りと、detail が "flag --verbose" の surface_without_spec の誤りが出る
  And surface_unspecified_stale の注意は出ない

@id=EX-core-422 @about=REQ-core-234 @source=docs/decision/records/2026-09-27-surface-check.md#A6,docs/decision/records/2026-09-27-surface-check.md#A15,docs/decision/records/2026-09-27-surface-check.md#A22
Scenario: コードから消えた面の1件は注意になる
  Given `未記載の面の一覧`に "kind" が "flag"、"name" が "--old" の形の正しい1件があり、どの`面のファイル`にも名前が "--old" の`面`が無い
  When "kotowari check --format json" を実行する
  Then "line" が null で detail が "flag --old" の surface_unspecified_stale の注意が出る

@id=EX-core-423 @about=REQ-core-234 @source=docs/decision/records/2026-09-27-surface-check.md#A6,docs/decision/records/2026-09-27-surface-check.md#A15,docs/decision/records/2026-09-27-surface-check.md#A16,docs/decision/records/2026-09-27-surface-check.md#A22
Scenario: IR に書いた面の1件は注意になる
  Given EX-core-419 の場面で、`要求`の`文`に "--verbose" を二重引用符で囲んで書き足した
  When "kotowari check --format json" を実行する
  Then detail が "flag --verbose" の surface_unspecified_stale の注意が出て、JSON の "surface" の "unspecified" は 0 である

@id=EX-core-424 @about=REQ-core-231 @source=docs/decision/records/2026-09-27-surface-check.md#A6,docs/decision/records/2026-09-27-surface-check.md#A22
Scenario: 一覧の指す先が無いと停止する
  Given EX-core-407 の場面で、設定の "surface.unspecified" が "docs/surface.yaml" で、そのファイルが無い
  When "kotowari check" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は "unreadable file: " で始まる

@id=EX-core-425 @about=REQ-core-231 @source=docs/decision/records/2026-09-27-surface-check.md#A6,docs/decision/records/2026-09-27-surface-check.md#A22
Scenario: 最上位が並びでない一覧は一覧のパスを出して停止する
  Given EX-core-407 の場面で、設定の "surface.unspecified" が "docs/surface.yaml" で、その中身が "kind: flag" の1行である
  When "kotowari check" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は "config error: docs/surface.yaml" で始まる
```
