# 設定で足す問い合わせ

[English](query-rules.md) | 日本語

設定の "tests.rules" で足す問い合わせの形と読み方、同梱の問い合わせとの関係、ルールのファイルの誤りを扱う。

## Requirements

### REQ-core-121: 同梱の問い合わせは外せない

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-multi-language-tests.md#A2, docs/decision/records/2026-09-24-multi-language-tests.md#A11
- verification: unit

kotowari は常に、同梱の`問い合わせ`を設定によらず使い、"tests.rules" のファイルの`問い合わせ`はそれに加える。

### REQ-core-186: ルールのファイル

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-multi-language-tests.md#A9, docs/decision/records/2026-09-24-multi-language-tests.md#A10, docs/decision/records/2026-09-24-multi-language-tests.md#A28
- verification: unit

kotowari は常に、"tests.rules" に並んだパスを`基準のディレクトリ`からの相対パスとして読み、その中身を ast-grep のルールの YAML（"---" で区切って複数のルールを並べてよい）として読み、各ルールをその "language" の言語の`問い合わせ`に加える。

### REQ-core-187: ルールを当てるファイル

- kind: event_driven
- source: docs/decision/records/2026-09-24-multi-language-tests.md#A29, docs/decision/records/2026-09-24-multi-language-tests.md#A45
- verification: unit

ルールに "files" か "ignores" があるとき、kotowari はその glob を ast-grep と同じ読み方で`テストのファイル`の`基準のディレクトリ`からの相対パスに当て、"files" に当たらないか "ignores" に当たる`テストのファイル`にはそのルールを当てない。

### REQ-core-188: 使わない項目

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-multi-language-tests.md#A29
- verification: unit

kotowari は常に、ルールの "fix"、"message"、"severity"、"note"、"metadata" を`テスト`の見つけ方に使わず、"severity" が "off" のルールも当てる。

### REQ-core-189: ルールのファイルの誤り

- kind: event_driven
- source: docs/decision/records/2026-09-24-multi-language-tests.md#A22, docs/decision/records/2026-09-24-multi-language-tests.md#A43, docs/decision/records/2026-09-24-multi-language-tests.md#A44, docs/decision/records/2026-09-24-guide-gaps.md#A2, docs/decision/records/2026-09-24-guide-gaps.md#A7
- verification: unit

"tests.rules" のパスのファイルが無いとき、パスがファイルでないとき、読めないとき、UTF-8 でないとき、同じパスが2回並んでいるとき、YAML として読めないとき、ast-grep のルールとして読めないとき、またはルールの "language" が TBL-core-031 の言語に無いとき、kotowari は設定の誤りを理由に`停止`する。"language" は ast-grep と同じく大文字小文字を区別せずに突き合わせ、ast-grep の別名（"ts"、"py"）も受ける。ルールの "id" の重なりは見ない。

## Examples

```gherkin
@id=EX-core-311 @about=REQ-core-186 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A9,docs/decision/records/2026-09-24-multi-language-tests.md#A10,docs/decision/records/2026-09-24-multi-language-tests.md#A13
Scenario: 足したルールでテストを数える
  Given "tests.files" が "tests/**/*.ts" を含み、"tests.rules" が "rules/bench.yml" だけで、そのファイルが "language: typescript" で "bench($NAME, $$$)" に当たるルールを持ち、"tests/a.test.ts" に印の無い "bench('fast', () => {})" がある
  When "kotowari check" を実行する
  Then detail が "fast" の test_without_id の誤りが1件出る

@id=EX-core-312 @about=REQ-core-121,REQ-core-181 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A11,docs/decision/records/2026-09-24-multi-language-tests.md#A27,docs/decision/records/2026-09-24-multi-language-tests.md#A13
Scenario: 同梱のルールと同じ節に当たっても1つと数える
  Given "tests.files" が "tests/**/*.ts" を含み、"tests.rules" の "language: typescript" のルールが "it($NAME, $$$)" に当たり、"tests/a.test.ts" に印の無い "it('x', () => {})" が1つある
  When "kotowari check" を実行する
  Then detail が "x" の test_without_id の誤りが1件だけ出る

@id=EX-core-313 @about=REQ-core-187 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A29,docs/decision/records/2026-09-24-multi-language-tests.md#A18
Scenario: files に当たらないファイルにはルールを当てない
  Given "tests.files" が "tests/**/*.ts" を含み、"tests.rules" の "language: typescript" のルールが "files" に "**/*.spec.ts" を持ち "bench($NAME, $$$)" に当たり、"tests/a.test.ts" に印の無い "bench('fast', () => {})" がある
  When "kotowari check" を実行する
  Then test_without_id の誤りは出ない

@id=EX-core-314 @about=REQ-core-188 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A29,docs/decision/records/2026-09-24-multi-language-tests.md#A13
Scenario: severity が off のルールも当てる
  Given "tests.files" が "tests/**/*.ts" を含み、"tests.rules" の "language: typescript" のルールが "severity: off" を持ち "bench($NAME, $$$)" に当たり、"tests/a.test.ts" に印の無い "bench('fast', () => {})" がある
  When "kotowari check" を実行する
  Then detail が "fast" の test_without_id の誤りが1件出る

@id=EX-core-315 @about=REQ-core-189 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A22
Scenario: ルールのファイルが無ければ停止する
  Given "tests.rules" が "rules/missing.yml" を持ち、そのファイルが無い
  When "kotowari check" を実行する
  Then 終了コードは 2 である

@id=EX-core-316 @about=REQ-core-189 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A22,docs/decision/records/2026-09-24-multi-language-tests.md#A6
Scenario: 知らない言語のルールで停止する
  Given "tests.rules" のファイルのルールが "language: cobol" を持つ
  When "kotowari check" を実行する
  Then 終了コードは 2 である
@id=EX-core-321 @about=REQ-core-189 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A43
Scenario: 同じルールのファイルが2回並ぶと停止する
  Given "tests.rules" が "rules/bench.yml" を2回並べている
  When "kotowari check" を実行する
  Then 終了コードは 2 である

@id=EX-core-322 @about=REQ-core-189 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A44,docs/decision/records/2026-09-24-multi-language-tests.md#A9,docs/decision/records/2026-09-24-multi-language-tests.md#A13
Scenario: language の別名を受ける
  Given "tests.files" が "tests/**/*.ts" を含み、"tests.rules" のルールが "language: ts" で "bench($NAME, $$$)" に当たり、"tests/a.test.ts" に印の無い "bench('fast', () => {})" がある
  When "kotowari check" を実行する
  Then 終了コードは 1 で、detail が "fast" の test_without_id の誤りが1件出る
@id=EX-core-378 @about=REQ-core-189,TBL-core-020 @source=docs/decision/records/2026-09-24-guide-gaps.md#A2,docs/decision/records/ir-form.md#出力,docs/decision/records/2026-09-24-multi-language-tests.md#A22,docs/decision/records/2026-09-24-multi-language-tests.md#A43,docs/decision/records/2026-09-24-multi-language-tests.md#A44
Scenario: 無いルールのファイルで止まるとき、詳細はルールのファイルを指す
  Given "tests.rules" が "rules/missing.yml" を並べ、そのファイルが無い
  When "kotowari check" を実行する
  Then 終了コードは 2 で、標準エラーは "config error: " で始まり "rules/missing.yml" を含み、".kotowari/config.yaml" を含まない

@id=EX-core-379 @about=REQ-core-189 @source=docs/decision/records/2026-09-24-guide-gaps.md#A7,docs/decision/records/2026-09-24-guide-gaps.md#A2,docs/decision/records/ir-form.md#出力
Scenario: 知らない言語のルールで止まるとき、その言語を示す
  Given "tests.rules" のルールのファイル "r.yml" の "language" が "cobol" である
  When "kotowari check" を実行する
  Then 終了コードは 2 で、標準エラーは "r.yml" と "unknown language: cobol" を含む1行である
```
