# エンジンの指摘の写し先

この文書は、形の読み取りをスキーマに置き換えたあと、スキーマの側が返した指摘を kotowari の指摘の種類へ写す対応を扱う。指摘の種類そのものと detail は findings の文書が定める。

## Requirements

### REQ-core-171: 指摘の対応表

- kind: algorithm
- source: docs/decision/records/2026-09-22-ir-engine.md#A2, docs/decision/records/2026-09-22-ir-engine.md#A10, docs/decision/records/2026-09-22-ir-engine.md#A40
- definition: TBL-core-029, TBL-core-030
- verification: review
- how_to_verify: 人が対応表と `crates/kotowari-markdown-schema/src/finding.rs` の種類の列挙を突き合わせ、過不足が無いことと、REQ-core-027 の9種類を写す行の "line" の扱いが null であることを見る

### REQ-core-172: 写し先の無い指摘は停止にする

- kind: event_driven
- source: docs/decision/records/2026-09-22-ir-engine.md#A28, docs/decision/records/records.md#A100, docs/decision/records/records.md#A101, docs/decision/records/2026-09-23-ir-engine-gaps.md#A27, docs/decision/records/2026-09-23-ir-engine-gaps.md#A36
- verification: unit

対応表（TBL-core-030）に行の無い種類の`指摘`をスキーマの側から受けたとき、または写し先を「発生しない」と書いた行の種類の`指摘`を受けたとき、kotowari は`停止`し、その`指摘`を黙って捨てない。写し先を「出さない」と書いた行の`指摘`だけは`停止`せずに捨て、「出さない」と書けるのは`除外`に列挙した入力の行だけである。

## Decision tables

### TBL-core-029: エンジンの指摘を写す対応表の列

- source: docs/decision/records/2026-09-22-ir-engine.md#A2, docs/decision/records/2026-09-22-ir-engine.md#A3, docs/decision/records/2026-09-22-ir-engine.md#A10, docs/decision/records/2026-09-22-ir-engine.md#A24, docs/decision/records/2026-09-22-ir-engine.md#A28, docs/decision/records/2026-09-22-ir-engine.md#A29, docs/decision/records/2026-09-22-ir-engine.md#A31, docs/decision/records/2026-09-22-ir-engine.md#A40, docs/decision/records/records.md#A92, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-22-ir-engine.md#A71, docs/decision/records/2026-09-22-ir-engine.md#A74, docs/decision/records/2026-09-22-ir-engine.md#A76, docs/decision/records/2026-09-22-ir-engine.md#A77, docs/decision/records/2026-09-22-ir-engine.md#A79, docs/decision/records/2026-09-22-ir-engine.md#A81, docs/decision/records/2026-09-23-ir-engine-gaps.md#A17, docs/decision/records/2026-09-23-ir-engine-gaps.md#A27, docs/decision/records/2026-09-23-ir-english-tokens.md#A2

スキーマの側が出した`指摘`を kotowari の`指摘`の種類へ写す表は、下の5つの列を持つ。名前が同じで意味が違う種類の行には、その旨を書く。スキーマの側の invalid_id は`項目`の見出しの`ID`の形で、kotowari の invalid_id（"@id" の値）とは別物である。スキーマの側の missing_table は、`用語集`の表にも`決定表`の表にも`出現回数`の範囲を宣言したので出ない。代わりに repeat_min_not_met が出て、`用語集`では glossary_invalid、`決定表`では missing_table に分かれる。スキーマの側の missing_required_field は、欠けた`フィールド行`に応じて kotowari の verification_missing、missing_source、missing_field、algorithm_without_definition に分かれる。写し先はこの4つで閉じる。「ノードの名前」の列には、スキーマの側が返す名前のほかに、kotowari が自分で知っている区別（どのスキーマで検証したか、`指摘`が行を持つか）も書く。"- definition:" の行はスキーマの側が "- kind:" の行の値で条件付きに必須と宣言するので、欠けたときは algorithm_without_definition へ写す。

| 列 | 中身 |
|---|---|
| スキーマの側の種類 | スキーマの側が出す指摘の種類 |
| ノードの名前 | 写し先を分けるのに使う名前、宣言の外の行の種別、`出現回数`を数えた`ノード`の`規則種別`。使わないときは空 |
| kotowari の種類 | 写し先。今のスキーマでは発生しないものは「発生しない」と書き、`除外`として写さずに捨てるものは「出さない」と書く |
| "line" の扱い | そのまま使う、null にする（REQ-core-027 の9種類）、または`項目`の見出しの行に付け直す |
| detail の材料 | 指摘のどの要素から作るか（読んだ行の文字そのまま、読んだ行の文字から "# " を除いたもの、ノードの名前、文書名、`抽出`の`項目`） |

### TBL-core-030: エンジンの指摘の写し先

- source: docs/decision/records/2026-09-22-ir-engine.md#A2, docs/decision/records/2026-09-22-ir-engine.md#A3, docs/decision/records/2026-09-22-ir-engine.md#A10, docs/decision/records/2026-09-22-ir-engine.md#A24, docs/decision/records/2026-09-22-ir-engine.md#A28, docs/decision/records/2026-09-22-ir-engine.md#A31, docs/decision/records/2026-09-22-ir-engine.md#A40, docs/decision/records/2026-09-22-ir-engine.md#A71, docs/decision/records/2026-09-22-ir-engine.md#A74, docs/decision/records/2026-09-22-ir-engine.md#A76, docs/decision/records/2026-09-22-ir-engine.md#A77, docs/decision/records/2026-09-22-ir-engine.md#A79, docs/decision/records/2026-09-22-ir-engine.md#A81, docs/decision/records/2026-09-22-ir-engine.md#A82, docs/decision/records/2026-09-22-ir-engine.md#A86, docs/decision/records/2026-09-22-ir-engine.md#A87, docs/decision/records/2026-09-23-ir-engine-gaps.md#A7, docs/decision/records/2026-09-23-ir-engine-gaps.md#A17, docs/decision/records/2026-09-23-ir-engine-gaps.md#A25, docs/decision/records/2026-09-23-ir-engine-gaps.md#A27, docs/decision/records/2026-09-23-ir-engine-gaps.md#A28, docs/decision/records/2026-09-23-ir-engine-gaps.md#A16, docs/decision/records/2026-09-23-ir-engine-gaps.md#A40

TBL-core-029 の列に沿って、スキーマの側の 24 種類をすべて並べる。「発生しない」の行の`指摘`を受けたときは REQ-core-172 のとおり`停止`し、「出さない」の行の`指摘`は捨てる。

| スキーマの側の種類 | ノードの名前 | kotowari の種類 | "line" の扱い | detail の材料 |
|---|---|---|---|---|
| missing_title |  | missing_title | null にする | 文書名 |
| multiple_titles |  | multiple_titles（2つ目以降の`題名`ごとに1件） | null にする | 読んだ行の文字から "# " を除き、前後の空白を除いたもの |
| title_pattern_mismatch | 用語集 | glossary_title_invalid | そのまま使う | 読んだ行の文字そのまま |
| undeclared_heading |  | unknown_heading | そのまま使う | 読んだ行の文字そのまま |
| undeclared_line | 名前と値の形の一覧の行、箇条書き、順序付きリスト | unknown_field | そのまま使う | 読んだ行の文字そのまま |
| undeclared_line | 文 | unknown_line | そのまま使う | 読んだ行の文字そのまま |
| undeclared_line | 表（`用語集`のスキーマ） | 出さない（"select: first" で`用語集`の表にならなかった表。`除外`） | — | — |
| undeclared_line | 表（`用語集`でないスキーマ）、コードブロック | unknown_line | そのまま使う | 読んだ行の文字そのまま |
| missing_required_field | 検証 | verification_missing | そのまま使う | `抽出`の`項目`の`ID` |
| missing_required_field | 出典 | missing_source | そのまま使う | `抽出`の`項目`の`ID` |
| missing_required_field | 種類 | missing_field | そのまま使う | ノードの名前 |
| missing_required_field | 確かめ方 | missing_field | そのまま使う | ノードの名前 |
| missing_required_field | 関係 | missing_field | そのまま使う | ノードの名前 |
| missing_required_field | 定義 | algorithm_without_definition | そのまま使う | `抽出`の`項目`の`ID` |
| missing_required_section |  | 発生しない（節はすべて必須でない） | — | — |
| missing_statement | 前置部の文 | 発生しない（前置部の文に`出現回数`の範囲を宣言した） | — | — |
| missing_statement | 項目の文 | 発生しない（`項目`の`文`に`出現回数`の範囲を宣言した） | — | — |
| missing_bullets |  | 発生しない（箇条書きを宣言していない） | — | — |
| missing_table | 用語集 | 発生しない（`用語集`の表に`出現回数`の範囲を宣言した） | — | — |
| missing_table | 決定表 | 発生しない（`決定表`の表に`出現回数`の範囲を宣言した） | — | — |
| missing_codeblock |  | 発生しない（コードブロックの下限が0） | — | — |
| field_pattern_mismatch |  | 発生しない（出典のパターンを外した） | — | — |
| field_enum_invalid | 種類 | unknown_kind | `項目`の見出しの行に付け直す | `抽出`の`項目`の該当の値 |
| field_enum_invalid | 検証 | verification_invalid | `項目`の見出しの行に付け直す | `抽出`の`項目`の該当の値 |
| statement_pattern_mismatch |  | 発生しない（文にパターンを宣言していない） | — | — |
| statement_enum_invalid |  | 発生しない（文に許可リストを宣言していない） | — | — |
| bullet_pattern_mismatch |  | 発生しない（箇条書きを宣言していない） | — | — |
| heading_level_mismatch |  | unknown_heading | そのまま使う | 読んだ行の文字そのまま |
| invalid_id |  | unknown_heading | そのまま使う | 読んだ行の文字そのまま |
| field_order_mismatch |  | 発生しない（並び順を宣言していない） | — | — |
| table_header_mismatch | 用語集（指摘の行が表の開始行） | 発生しない（`用語集`の表に "select: first" を宣言したので、ヘッダの合わない表は宣言の外の表になる） | — | — |
| table_header_mismatch | 用語集（指摘の行がデータ行。セルがヘッダより少ない行） | invalid_glossary_row | そのまま使う | 読んだ行の文字そのまま |
| codeblock_lang_mismatch |  | unknown_code_block | そのまま使う | 読んだ行の文字そのまま |
| codeblock_line_mismatch |  | 発生しない（gherkin の行のパターンを外した） | — | — |
| repeat_min_not_met | 文で、指摘が行を持たない | missing_scope | null にする | 文書名 |
| repeat_min_not_met | 文で、指摘が行を持つ | missing_statement | そのまま使う | `抽出`の`項目`の`ID` |
| repeat_min_not_met | 表で、指摘が行を持たない | glossary_invalid | null にする | 文書名 |
| repeat_min_not_met | 表で、指摘が行を持つ | missing_table | そのまま使う | `抽出`の`項目`の`ID` |
| repeat_min_not_met | 上のどれにも当たらない | 発生しない（ほかの下限はすべて0） | — | — |
| repeat_max_exceeded | フィールド行の名前 | duplicate_field | そのまま使う | ノードの名前 |
| repeat_max_exceeded | フィールド行のほか | 発生しない（フィールド行以外に上限を課していない） | — | — |

## Examples

```gherkin
@id=EX-core-265 @about=REQ-core-172 @source=docs/decision/records/2026-09-22-ir-engine.md#A28,docs/decision/records/records.md#A100,docs/decision/records/records.md#A101
Scenario: 写し先の無い指摘は停止になる
  Given 対応表 TBL-core-030 に行の無い種類の`指摘`と、写し先を「発生しない」と書いた行の種類の`指摘`がスキーマの側から返る
  When "kotowari check" を実行する
  Then どちらの場合も`停止`する

@id=EX-core-278 @about=REQ-core-035,TBL-core-030 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A7,docs/decision/records/2026-09-23-ir-engine-gaps.md#A17
Scenario: 題名が3つある文書は2つ目以降の題名ごとに誤りになる
  Given "# 一" と "# 二" と "# 三" の3つの`題名`を持つ`話題ごとの文書`がある
  When "kotowari check --format json" を実行する
  Then その文書に multiple_titles の誤りが2件出て、detail は "二" と "三" で、どちらも "line" は null である

@id=EX-core-279 @about=REQ-core-172,TBL-core-030 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A25,docs/decision/records/2026-09-23-ir-engine-gaps.md#A27
Scenario: 用語集の表にならなかった表は写さずに捨てる
  Given "CONTEXT.md" に、ヘッダの合う表の後に、空行を挟んでヘッダの違う2つ目の表がある
  When "kotowari check --format json" を実行する
  Then `停止`せず、"CONTEXT.md" には`指摘`が出ない
```
