# 全体像の元データの申告

`全体像の元データ`の検査の結果を "kotowari check" と "kotowari status" の出力にどう加えるかを扱う。検査そのものは overview-data.md が扱う。

## Requirements

### REQ-core-288: check の overview の群

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A64, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-02-whole-picture.md#A78
- verification: unit

kotowari は常に、"kotowari check" の JSON の最上位の "overview" に、読んだ`全体像の元データ`の数を "files"、`全体像の元データ`の中の形の正しい`ガイドの印`の1件の数を "marks" として出す。数え方は "guides" と同じ（REQ-core-206）で、"overview" の鍵が`設定ファイル`に無いときも両方 0 で出す。"--format" が "text" のときは出さない。

### REQ-core-289: status の overview の群

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A64
- verification: unit

kotowari は常に、"kotowari status" に "kotowari check" と同じ "overview" の群（"files" と "marks"）を出す。

### REQ-core-290: 元データの指摘は check の指摘に加わる

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A33, docs/decision/records/2026-10-02-whole-picture.md#A49, docs/decision/records/2026-10-02-whole-picture.md#A50, docs/decision/records/2026-10-02-whole-picture.md#A64, docs/decision/records/2026-10-03-public-crate-api.md#A33, docs/decision/records/2026-10-04-overview-on-public-api.md#A2
- verification: unit

kotowari は常に、`全体像の元データ`の`指摘`を "kotowari check" と "kotowari status" の`指摘`に加え、ほかの`指摘`と合わせて REQ-core-024 の並びで出し、"counts" と終了コードと "complete" にもほかの`指摘`と同じく数える。kotowari ライブラリが`全体像の元データ`の`指摘`と "overview" の群を core の検査に追加の指摘の群として渡し、core がほかの`指摘`と合わせて並べて数える。

## Examples

```gherkin
@id=EX-core-472 @about=REQ-core-288,REQ-core-290 @source=docs/decision/records/2026-10-02-whole-picture.md#A64,docs/decision/records/2026-10-02-whole-picture.md#A50,docs/decision/records/2026-10-02-whole-picture.md#A38,docs/decision/records/2026-10-02-whole-picture.md#A69
Scenario: 元データの誤りは check の findings と counts に入り終了コードは 1
  Given "overview.files" に当たる`全体像の元データ`が2つあり、1つの冒頭が lead でない
  When "kotowari check --format json" を実行する
  Then 終了コードは 1 で、"overview" の "files" は 2、"findings" に overview_lead_missing が1件あり、"counts" の overview_lead_missing は 1 である

@id=EX-core-473 @about=REQ-core-289,REQ-core-290 @source=docs/decision/records/2026-10-02-whole-picture.md#A64,docs/decision/records/2026-10-02-whole-picture.md#A38,docs/decision/records/2026-10-02-whole-picture.md#A78
Scenario: 元データの誤りがあれば status は complete でない
  Given ほかに`誤り`の無い`IR`と、冒頭が lead でない`全体像の元データ`がある
  When "kotowari status --format json" を実行する
  Then "overview" の群があり、"complete" は false で、終了コードは 1 である
```
