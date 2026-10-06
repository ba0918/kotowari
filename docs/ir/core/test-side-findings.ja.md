# テスト側の指摘

[English](test-side-findings.md) | 日本語

テストより先に仕様をコミットするときに、"kotowari check" の終了コードの判定から外せる`テスト側の指摘`の範囲と、そのオプションの振る舞いと、フックへの組み込み方の案内を扱う。オプションの受け口と終了コードの表は cli.md が扱う。

## Requirements

### REQ-core-357: テスト側の指摘を許す

- kind: event_driven
- source: docs/decision/records/2026-10-06-spec-first-commit.md#A1, docs/decision/records/2026-10-06-spec-first-commit.md#A2, docs/decision/records/2026-10-06-spec-first-commit.md#A4, docs/decision/records/2026-10-06-spec-first-commit.md#A6
- definition: TBL-core-047
- verification: unit

"kotowari check" に "--allow-test-findings" を付けたとき、kotowari は TBL-core-047 で`テスト側の指摘`に当たる`誤り`を終了コードの判定に数えず、`テスト側の指摘`を含むすべての`指摘`を、オプションの無いときと同じ種類、severity、並びで出力する。

### REQ-core-358: フックへの組み込み方の案内

- kind: ubiquitous
- source: docs/decision/records/2026-10-06-spec-first-commit.md#A7
- verification: review
- how_to_verify: "agent/skills/kotowari/references/findings.md"、"agent/skills/kotowari-adopt/" と "docs/guides/commands/check.md" を読み、pre-commit のフックでは "kotowari check" に "--allow-test-findings" を付け、pre-push のフックと CI では付けないことが、3か所すべてに書いてあることを確かめる

kotowari のスキルと check のガイドは常に、pre-commit のフックでは "--allow-test-findings" を付けて "kotowari check" を走らせ、pre-push のフックと CI では付けずに走らせる組み方を案内する。

## Decision tables

### TBL-core-047: テスト側の指摘の範囲

- source: docs/decision/records/2026-10-06-spec-first-commit.md#A2, docs/decision/records/2026-10-06-spec-first-commit.md#A3, docs/decision/records/2026-10-06-spec-first-commit.md#A9

この表に無い種類の`誤り`は`テスト側の指摘`でない。

| 種類 | `テスト側の指摘`になる条件 |
|---|---|
| requirement_without_test | 常に |
| scenario_without_test | 常に |
| test_without_id | 常に |
| invalid_marker | "path" が`テストのファイル`のとき |
| unresolved_reference | "path" が`テストのファイル`のとき |
| unparsable_file | "path" が`テストのファイル`で、`面のファイル`でないとき |

## Examples

```gherkin
@id=EX-core-547 @about=REQ-core-357,TBL-core-047 @source=docs/decision/records/2026-10-06-spec-first-commit.md#A1,docs/decision/records/2026-10-06-spec-first-commit.md#A2,docs/decision/records/2026-10-06-spec-first-commit.md#A4
Scenario: 仕様だけを先に書いた状態はオプション付きなら通る
  Given "verification: unit" の要求 "REQ-greet-001" があり、その ID を含む印のテストが無い
  And ほかに誤りが無い
  When "kotowari check --allow-test-findings" を実行する
  Then 終了コードは 0 である
  And 出力に severity が "error" の requirement_without_test の指摘が1件ある

@id=EX-core-548 @about=REQ-core-357,TBL-core-002 @source=docs/decision/records/2026-10-06-spec-first-commit.md#A1
Scenario: 同じ状態でもオプションが無ければ止まる
  Given "verification: unit" の要求 "REQ-greet-001" があり、その ID を含む印のテストが無い
  When "kotowari check" を実行する
  Then 終了コードは 1 である

@id=EX-core-549 @about=REQ-core-357,TBL-core-047 @source=docs/decision/records/2026-10-06-spec-first-commit.md#A2
Scenario: オプション付きでも IR の誤りは止める
  Given テストの無い要求があり、IR の文書に用語集に無い語をバッククォートで囲んだ行がある
  When "kotowari check --allow-test-findings" を実行する
  Then 終了コードは 1 である

@id=EX-core-550 @about=REQ-core-357,TBL-core-047 @source=docs/decision/records/2026-10-06-spec-first-commit.md#A3
Scenario: 面のファイルでもあるテストのファイルが読めなければ止める
  Given "tests.files" と "surface.files" の両方に当たる "src/lib.rs" に構文の誤りがある
  When "kotowari check --allow-test-findings" を実行する
  Then 終了コードは 1 である
  And 出力に path が "src/lib.rs" の unparsable_file の指摘がある

@id=EX-core-551 @about=REQ-core-357,TBL-core-047 @source=docs/decision/records/2026-10-06-spec-first-commit.md#A2
Scenario: ガイドの印の形の誤りはオプション付きでも止める
  Given ガイドに中身の空のガイドの印があり、ほかに誤りが無い
  When "kotowari check --allow-test-findings" を実行する
  Then 終了コードは 1 である

@id=EX-core-552 @about=REQ-core-004,REQ-core-357 @source=docs/decision/records/2026-10-06-spec-first-commit.md#A5,docs/decision/records/2026-10-06-spec-first-commit.md#A12,docs/decision/records/records.md#A60,docs/decision/records/records.md#A20
Scenario: check でないコマンドに付けると停止する
  Given 検査できる IR がある
  When "kotowari status --allow-test-findings" を実行する
  Then 終了コードは 2 である
  And 停止の理由は引数の誤りである
```
