# 目次の検査

`目次`の置き場と読み方、形、`全体像の元データ`との食い違いの検査と、`目次`を描画のエンジンに渡すことを扱う。検査は`全体像の元データ`の検査（overview-data.md）と同じく kotowari-overview が行い、"kotowari check" と "kotowari status" の`指摘`に加わる（overview-output.md）。

## Requirements

### REQ-core-325: 目次の置き場

- kind: event_driven
- source: docs/decision/records/2026-10-05-overview-index.md#A10, docs/decision/records/2026-10-05-overview-index.md#A11, docs/decision/records/2026-10-05-overview-index.md#A17, docs/decision/records/2026-10-05-overview-index.md#A32
- verification: unit

`設定ファイル`に "overview" の鍵があるとき、kotowari は "kotowari check"、"kotowari status"、"kotowari overview build"、"kotowari overview serve" で、"overview.toc" の指すファイルを`目次`として読む。指す先が無いか読めないとき、kotowari は読めないファイルを理由に`停止`し、UTF-8 でないとき、UTF-8 でないファイルを理由に`停止`する。

### REQ-core-326: 目次と glob の重なり

- kind: event_driven
- source: docs/decision/records/2026-10-05-overview-index.md#A17, docs/decision/records/2026-10-02-whole-picture.md#A58, docs/decision/records/2026-10-05-overview-index.md#A23, docs/decision/records/2026-10-05-overview-index.md#A28, docs/decision/records/2026-10-05-overview-index.md#A32
- verification: unit

"kotowari check"、"kotowari status"、"kotowari overview build"、"kotowari overview serve" で、"overview.toc" の指すファイルが、"overview.files"（REQ-core-278）、"guides.files"、"tests.files" のどれかの走査で読むファイルに入るとき、kotowari は設定の誤りを理由に`停止`し、詳細に、そのファイルの`基準のディレクトリ`からの相対パスに ": matched by both overview.toc and " と、当たった鍵のうちこの順で最初のものの名前を続けた文字列を出す。REQ-core-199 と REQ-core-280 の重なりがあればそれを先に判定し、この判定は REQ-core-325 の`目次`の読み込みより先に行う。

### REQ-core-327: 目次の形

- kind: algorithm
- source: docs/decision/records/2026-10-05-overview-index.md#A10, docs/decision/records/2026-10-05-overview-index.md#A15, docs/decision/records/2026-10-05-overview-index.md#A16, docs/decision/records/2026-10-05-overview-index.md#A22
- definition: TBL-core-043
- verification: unit

### REQ-core-328: 目次に無いページ

- kind: event_driven
- source: docs/decision/records/2026-10-05-overview-index.md#A12, docs/decision/records/2026-10-05-overview-index.md#A16, docs/decision/records/2026-10-05-overview-index.md#A21, docs/decision/records/2026-10-05-localization.md#A31, docs/decision/records/2026-10-05-localization.md#A26, docs/decision/records/2026-10-05-localization.md#A39, docs/decision/records/2026-10-05-localization.md#A22, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

形の正しい`目次`（`対`なら`先頭の言語`の`側`）のどこにも、ある`先頭の言語`の`側`の`全体像の元データ`のファイル名から ".md" を除いた名前の項目が無いとき、kotowari は "path" を`目次`のファイル、"line" を null、detail をその名前にして overview_toc_page_missing の`誤り`を出す。

### REQ-core-329: 元データの無い名前と重なった名前

- kind: event_driven
- source: docs/decision/records/2026-10-05-overview-index.md#A12, docs/decision/records/2026-10-05-overview-index.md#A16, docs/decision/records/2026-10-05-overview-index.md#A29, docs/decision/records/2026-10-05-overview-index.md#A21, docs/decision/records/2026-10-05-overview-index.md#A34, docs/decision/records/2026-10-05-localization.md#A31, docs/decision/records/2026-10-05-localization.md#A26, docs/decision/records/2026-10-05-localization.md#A39, docs/decision/records/2026-10-05-localization.md#A22, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

形の正しい`目次`（`対`なら`先頭の言語`の`側`）の名前の項目が、どの`先頭の言語`の`側`の`全体像の元データ`のファイル名から ".md" を除いた名前とも同じでないとき、kotowari は "path" を`目次`のファイル、"line" を null、detail をその項目の JSON Pointer にして overview_toc_page_unknown の`誤り`を出す。`全体像の元データ`のある名前の項目が2つ以上あるとき、kotowari は`目次`を書かれた順に深さ優先でたどって2つ目以降の項目ごとに、"path" を`目次`のファイル、"line" を null、detail をその項目の JSON Pointer にして overview_toc_page_duplicate の`誤り`を出す。

### REQ-core-330: 空の目次の群

- kind: event_driven
- source: docs/decision/records/2026-10-05-overview-index.md#A12, docs/decision/records/2026-10-05-overview-index.md#A16, docs/decision/records/2026-10-05-overview-index.md#A21, docs/decision/records/2026-10-05-overview-index.md#A34
- verification: unit

形の正しい`目次`の`目次の群`の "items" が空の並びのとき、kotowari は "path" を`目次`のファイル、"line" を null、detail をその`目次の群`の JSON Pointer（いちばん外側なら "(root)"）にして overview_toc_group_empty の`誤り`を出す。

### REQ-core-331: 形の誤った目次は照合しない

- kind: event_driven
- source: docs/decision/records/2026-10-05-overview-index.md#A12, docs/decision/records/2026-10-05-overview-index.md#A16, docs/decision/records/2026-10-05-overview-index.md#A21
- verification: unit

`目次`に overview_toc_invalid の`誤り`が1件以上あるとき、kotowari はその`目次`に REQ-core-328、REQ-core-329、REQ-core-330 の検査をしない。

### REQ-core-332: 目次を描画のエンジンに渡す

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-overview-index.md#A7, docs/decision/records/2026-10-05-overview-index.md#A10, docs/decision/records/2026-10-05-overview-index.md#A15
- verification: unit

kotowari は常に、"kotowari overview build" と "kotowari overview serve" で、`目次`の "title"、"note"、"items" を書かれたままの順と入れ子で、描画のエンジンの`目次`として渡す。

## Decision tables

### TBL-core-043: 目次の形

- source: docs/decision/records/2026-10-05-overview-index.md#A10, docs/decision/records/2026-10-05-overview-index.md#A15, docs/decision/records/2026-10-05-overview-index.md#A16, docs/decision/records/2026-10-05-overview-index.md#A22, docs/decision/records/2026-10-05-overview-index.md#A27, docs/decision/records/2026-10-05-overview-index.md#A34

形の違反ごとに、"path" を`目次`のファイル、"line" を null、detail を合わなかった場所にして overview_toc_invalid の`誤り`を出す。合わなかった場所の書き方は REQ-core-282 と同じで、合わなかった値の JSON Pointer に、知らない鍵と欠けた鍵ではその鍵の名前を "/" で足したもの、値の全体なら "(root)"、YAML として読めないときは "(yaml)" とする。同じ detail の`誤り`は1件にまとめる。

| 部分 | 形 |
|---|---|
| 最上位 | 1つの`目次の群` |
| `目次の群` | 鍵と値の組。鍵は "title"（必須。空でない文字列）、"note"（省いてよい。改行を含まない文字列）、"items"（必須。名前の項目か`目次の群`の並び。空の並びは REQ-core-330 が扱う）だけ。知らない鍵、欠けた必須の鍵、型の違う値、空の "title" は形の違反 |
| 名前の項目 | 文字列。`全体像の元データ`のファイル名から ".md" を除いた名前を書く。空の文字列は形の違反 |

## Examples

```gherkin
@id=EX-core-505 @about=REQ-core-013,REQ-core-325 @source=docs/decision/records/2026-10-05-overview-index.md#A11,docs/decision/records/2026-10-05-overview-index.md#A17,docs/decision/records/2026-10-05-overview-index.md#A32,docs/decision/records/records.md#A20,docs/decision/records/records.md#A104
Scenario: 目次の鍵が無いか目次のファイルが無ければ止まる
  Given "overview.files" はあるが "overview.toc" の鍵が無い`設定ファイル`と、"overview.toc" が無いファイルを指す`設定ファイル`がある
  When それぞれで "kotowari overview build" を実行する
  Then どちらも終了コードは 2 で、前者の標準エラーの1行目は "config error: " で始まり、後者は読めないファイルを理由に`停止`する

@id=EX-core-506 @about=REQ-core-326 @source=docs/decision/records/2026-10-05-overview-index.md#A17,docs/decision/records/2026-10-05-overview-index.md#A23,docs/decision/records/2026-10-05-overview-index.md#A28,docs/decision/records/records.md#A20,docs/decision/records/records.md#A104,docs/decision/records/2026-10-05-overview-index.md#A32
Scenario: 元データの glob に当たる目次は設定の誤りになる
  Given "overview.files" が ".kotowari/overview/*.md" で、"overview.toc" が ".kotowari/overview/toc.md" である
  When "kotowari check" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は "config error: .kotowari/overview/toc.md: matched by both overview.toc and overview.files" である

@id=EX-core-507 @about=REQ-core-327,REQ-core-331 @source=docs/decision/records/2026-10-05-overview-index.md#A10,docs/decision/records/2026-10-05-overview-index.md#A16,docs/decision/records/2026-10-05-overview-index.md#A21,docs/decision/records/2026-10-05-overview-index.md#A34
Scenario: 知らない鍵を持つ目次の群は形の誤りになり照合しない
  Given `目次`の "items" の1件目が、"title" と "items" と "color" の鍵を持つ`目次の群`で、どの`全体像の元データ`の名前も`目次`に無い
  When "kotowari check --format json" を実行する
  Then "line" が null で detail が "/items/0/color" の overview_toc_invalid の`誤り`が出て、overview_toc_page_missing は出ない

@id=EX-core-508 @about=REQ-core-328,REQ-core-329,REQ-core-294 @source=docs/decision/records/2026-10-05-overview-index.md#A12,docs/decision/records/2026-10-05-overview-index.md#A16,docs/decision/records/2026-10-02-whole-picture.md#A33
Scenario: 目次に無いページと元データの無い名前と重なった名前は誤りになる
  Given `全体像の元データ`が "a.md"、"b.md"、"c.md" で、`目次`の "items" が "a"、"z"、"a" の順である
  When "kotowari check --format json" を実行する
  Then detail が "b" と "c" の overview_toc_page_missing、detail が "/items/1" の overview_toc_page_unknown、detail が "/items/2" の overview_toc_page_duplicate の`誤り`が出る
  And "kotowari overview build" は終了コード 2 で何も書かない

@id=EX-core-509 @about=REQ-core-330 @source=docs/decision/records/2026-10-05-overview-index.md#A12,docs/decision/records/2026-10-05-overview-index.md#A16
Scenario: 項目の無い目次の群は誤りになる
  Given `目次`の "items" が、"a" と、"title" が "空" で "items" が空の並びの`目次の群`である
  When "kotowari check --format json" を実行する
  Then detail が "/items/1" の overview_toc_group_empty の`誤り`が出る

@id=EX-core-510 @about=REQ-core-332 @source=docs/decision/records/2026-10-05-overview-index.md#A7,docs/decision/records/2026-10-05-overview-index.md#A15
Scenario: 一覧は目次に書いた順に並ぶ
  Given `全体像の元データ`が "a.md" と "b.md" で、`目次`の "title" が "kotowari"、"items" が "b"、"a" の順である
  When "kotowari overview build" を実行する
  Then ".kotowari/cache/overview/index.html" の見出しは "kotowari" で、"b" の題名が "a" の題名より前にある
```
