# 全体像の元データの検査

`全体像の元データ`の置き場と読み方、形、`部品`の中身、冒頭の lead、扱う`IR`の文書の一覧、`部品`の中の参照、`ガイドの印`の検査を扱う。この検査は kotowari-overview が行い、"kotowari check" と "kotowari status" の`指摘`に加わる（overview-output.md）。

## Requirements

### REQ-core-278: 元データの置き場

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A15, docs/decision/records/2026-10-02-whole-picture.md#A17, docs/decision/records/2026-10-02-whole-picture.md#A32, docs/decision/records/2026-10-02-whole-picture.md#A49, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-02-whole-picture.md#A76
- verification: unit

kotowari は常に、"kotowari check"、"kotowari status"、"kotowari overview build"、"kotowari overview serve" で、設定の "overview.files" の glob に当たり拡張子が小文字の ".md" のファイルを`全体像の元データ`として読み、それ以外のファイルを読まない（`除外`）。"kotowari overview build" と "kotowari overview serve" は、参照を解くために "kotowari check" と同じ設定と置き場から`IR`と`判断の記録`と ADR を読み、その置き場の`停止`（REQ-core-018）も check と同じにする。glob の読み方、走査、`除外`、読めないファイルと UTF-8 でないファイルと先の無いシンボリックリンクでの`停止`は "guides.files" と`ガイド`のとおりにし、隠しディレクトリは "changes.records" と同じく glob がパスの成分で名指ししたものだけを読む。

### REQ-core-279: 元データの鍵が無いとき

- kind: event_driven
- source: docs/decision/records/2026-10-02-whole-picture.md#A32
- verification: unit

`設定ファイル`に "overview" の鍵が無いとき、kotowari は "kotowari check" と "kotowari status" で`全体像の元データ`を1つも読まず、"kotowari overview build" と "kotowari overview serve" では設定の誤りを理由に`停止`する。

### REQ-core-280: ガイドとテストとの重なり

- kind: event_driven
- source: docs/decision/records/2026-10-02-whole-picture.md#A58, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-02-whole-picture.md#A75
- verification: unit

"kotowari check"、"kotowari status"、"kotowari overview build"、"kotowari overview serve" で、1つのファイルが "overview.files" の glob と、"guides.files" か "tests.files" の glob の両方に当たるとき、kotowari は設定の誤りを理由に`停止`し、詳細に、重なったファイルのうちパスのバイト順で最初の1つの`基準のディレクトリ`からの相対パスに ": matched by both overview.files and guides.files" か ": matched by both overview.files and tests.files" を続けた文字列を出す。REQ-core-199 の重なりがあればそれを先に判定し、1つのファイルが3つすべてに当たるときは ": matched by both overview.files and guides.files" を出す。

### REQ-core-281: 元データの形

- kind: algorithm
- source: docs/decision/records/2026-10-02-whole-picture.md#A44, docs/decision/records/2026-10-02-whole-picture.md#A54, docs/decision/records/2026-10-02-whole-picture.md#A55, docs/decision/records/2026-10-02-whole-picture.md#A69, docs/decision/records/2026-10-02-whole-picture.md#A71
- definition: TBL-core-038
- verification: unit

### REQ-core-282: 部品の中身

- kind: event_driven
- source: docs/decision/records/2026-10-02-whole-picture.md#A43, docs/decision/records/2026-10-02-whole-picture.md#A46, docs/decision/records/2026-10-02-whole-picture.md#A47, docs/decision/records/2026-10-02-whole-picture.md#A48, docs/decision/records/2026-10-02-whole-picture.md#A54, docs/decision/records/2026-10-02-whole-picture.md#A69, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-02-whole-picture.md#A74, docs/decision/records/2026-10-02-whole-picture.md#A85, docs/decision/records/2026-10-02-whole-picture.md#A86, docs/decision/records/2026-10-05-overview-page-reading.md#A10
- verification: unit

`全体像の元データ`の情報文字列が "view" と1つ以上の空白と種類の名前であるフェンスのコードブロックについて、種類の名前が描画のエンジンの`部品`の種類に無いとき、kotowari は "line" をフェンスの開始の行、detail を種類の名前にして overview_part_unknown の`誤り`を出す。中身が YAML として読めないとき、または描画のエンジンが公開するその種類のスキーマに合わないとき、kotowari は "line" をフェンスの開始の行、detail を種類の名前と1つの半角空白と合わなかった場所にして overview_part_invalid の`誤り`を出す。ただし YAML として読めないときに YAML の読み取りが誤りの位置を返したら、"line" はその位置の行を`全体像の元データ`のファイルの行に直したものにする。合わなかった場所は、合わなかった値の JSON Pointer に、知らない鍵（"additionalProperties"）と欠けた鍵（"required"）ではその鍵の名前を "/" で足したもので、値の全体なら "(root)"、YAML として読めないときは "(yaml)" と書く。同じ "line" と detail の`誤り`は1件にまとめる。detail に検査のライブラリが作る文を入れない。

### REQ-core-283: 冒頭の lead

- kind: event_driven
- source: docs/decision/records/2026-10-02-whole-picture.md#A7, docs/decision/records/2026-10-02-whole-picture.md#A38, docs/decision/records/2026-10-02-whole-picture.md#A69, docs/decision/records/2026-10-02-whole-picture.md#A74, docs/decision/records/2026-10-02-whole-picture.md#A77
- verification: unit

`全体像の元データ`の`題名`の後で最初の、HTML のコメントでも空行でもないブロックが、種類の名前が "lead" の`部品`でないとき、kotowari は "line" を null、detail を文書名にして overview_lead_missing の`誤り`を出す。

### REQ-core-284: 扱う IR の文書

- kind: event_driven
- source: docs/decision/records/2026-10-02-whole-picture.md#A19, docs/decision/records/2026-10-02-whole-picture.md#A55, docs/decision/records/2026-10-02-whole-picture.md#A69, docs/decision/records/2026-10-02-whole-picture.md#A74
- verification: unit

`全体像の元データ`の frontmatter の "ir" の1件が、`基準のディレクトリ`からの相対パスで読んだ`IR`の`話題ごとの文書`のどれでもないとき、kotowari は "line" を null、detail をその1件の文字にして overview_ir_missing の`誤り`を出す。1つの`話題ごとの文書`が2つ以上の`全体像の元データ`の "ir" にあるとき、kotowari はパスのバイト順で2つ目以降の`全体像の元データ`ごとに、"line" を null、detail をその`話題ごとの文書`のパスにして overview_ir_shared の`誤り`を出す。

### REQ-core-285: 部品の中の参照

- kind: event_driven
- source: docs/decision/records/2026-10-02-whole-picture.md#A33, docs/decision/records/2026-10-02-whole-picture.md#A66, docs/decision/records/2026-10-02-whole-picture.md#A69, docs/decision/records/2026-10-02-whole-picture.md#A74, docs/decision/records/2026-10-02-whole-picture.md#A77
- verification: unit

スキーマに合う`部品`の値の中の、名前が "refs" の欄の文字列の1件と名前が "ref" の欄の文字列（入れ子の深さを問わない）を参照とし、参照が`ID`の形でその`ID`の`項目`か`シナリオ`が`IR`に無いとき、または参照が "#" を含み`出典`の規則でその先が無いとき、または参照がそのどちらの形でもないとき、kotowari は "line" を`部品`のフェンスの開始の行、detail を参照の文字にして overview_ref_unresolved の`誤り`を1件ごとに出す。

### REQ-core-286: 元データのガイドの印

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A31, docs/decision/records/2026-10-02-whole-picture.md#A39, docs/decision/records/2026-10-02-whole-picture.md#A54, docs/decision/records/2026-10-02-whole-picture.md#A58, docs/decision/records/2026-10-02-whole-picture.md#A69, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-02-whole-picture.md#A77
- verification: unit

kotowari は常に、`全体像の元データ`の中の`ガイドの印`を`ガイド`と同じ規則（REQ-core-200、REQ-core-201、REQ-core-202、REQ-core-203、REQ-core-204）で読んで照合し、invalid_marker の`誤り`と guide_stale の`注意`を、"path" を`全体像の元データ`にして出す。`全体像の元データ`は`ガイド`として二重に読まない。節の`ガイドの印`は、その節の "## " の見出しの次の行に、それだけの行として置く。

### REQ-core-287: 元データの検査は kotowari-overview が行う

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A21, docs/decision/records/2026-10-02-whole-picture.md#A48, docs/decision/records/2026-10-02-whole-picture.md#A49, docs/decision/records/2026-10-03-public-crate-api.md#A33, docs/decision/records/2026-10-04-overview-on-public-api.md#A1, docs/decision/records/2026-10-04-overview-on-public-api.md#A2
- verification: review
- how_to_verify: 解析と検査がkotowari-overviewにあり、同クレートがcoreとmarkdown-schemaとmarkdown-viewに依存してファイル・ネットワーク・環境変数を操作せず、coreがoverviewとviewに依存しないことを確認する。元データの読み込みとcacheへの書き込みがkotowariライブラリに、serveがkotowari-cliにあり、checkへの結果の統合がcoreの追加の指摘の群を通ることを確認する。

kotowari は常に、`全体像の元データ`の解析と検査を kotowari-overview のクレートで行い、kotowari-core はそれを知らない。元データのファイルの読み込みと`全体像`のファイルの書き込みは kotowari ライブラリが、HTTP で配る serve は kotowari-cli が行う。検査結果を check へ統合するのは kotowari ライブラリで、core の追加の指摘の群として渡す。

## Decision tables

### TBL-core-038: 元データの形

- source: docs/decision/records/2026-10-02-whole-picture.md#A44, docs/decision/records/2026-10-02-whole-picture.md#A54, docs/decision/records/2026-10-02-whole-picture.md#A55, docs/decision/records/2026-10-02-whole-picture.md#A71, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-02-whole-picture.md#A77, docs/decision/records/2026-10-02-whole-picture.md#A74, docs/decision/records/2026-10-02-whole-picture.md#A85, docs/decision/records/2026-10-04-overview-on-public-api.md#A5, docs/decision/records/2026-10-04-overview-on-public-api.md#A11

形はスキーマ（kotowari-overview のパッケージの中の "crates/kotowari-overview/schemas/overview.yaml" をコンパイル時に取り込む）で宣言し、Markdown としての形の検査は kotowari-markdown-schema が行う。形に合わない所ごとに、"line" をその行（文書全体にかかるものは null）、detail を kotowari-markdown-schema の`指摘`の種類の名前にして overview_form_invalid の`誤り`を出す。frontmatter の行の違反だけは、detail を "frontmatter" にする。

| 部分 | 形 |
|---|---|
| frontmatter | 文書の先頭の YAML。鍵は "ir" だけで、値は1件以上の文字列の一覧。無い、読めない、知らない鍵、"ir" が無いか空か文字列の一覧でない、は形の違反 |
| 題名 | "# " の見出しがちょうど1つ |
| 題名の後 | 最初の "## " の見出しより前に、`部品`と HTML のコメントだけを置ける（最初の`部品`が lead であることは REQ-core-283 が検査する） |
| 節 | "## " の見出し。下に文、箇条書き（入れ子を含む）、番号付きの一覧、表、コードブロック、HTML のコメント、"### " の見出しを置ける |
| 部品 | 情報文字列が "view" と1つ以上の空白と種類の名前であるフェンスのコードブロック。中身は REQ-core-282 が検査する |

## Examples

```gherkin
@id=EX-core-463 @about=REQ-core-279 @source=docs/decision/records/2026-10-02-whole-picture.md#A32,docs/decision/records/2026-10-02-whole-picture.md#A64,docs/decision/records/2026-10-02-whole-picture.md#A78
Scenario: overview の鍵が無ければ check は元データを読まず build は止まる
  Given `設定ファイル`に "overview" の鍵が無く、".kotowari/overview/x.md" に形の崩れたファイルがある
  When "kotowari check --format json" と "kotowari overview build" を実行する
  Then check の "overview" の "files" は 0 で、そのファイルを指す`指摘`は無い
  And build の終了コードは 2 で、標準エラーの1行目は "config error: " で始まる

@id=EX-core-464 @about=REQ-core-278 @source=docs/decision/records/2026-10-02-whole-picture.md#A17,docs/decision/records/2026-10-02-whole-picture.md#A32,docs/decision/records/2026-10-02-whole-picture.md#A64
Scenario: 名指しした隠しディレクトリの元データを読む
  Given "overview.files" が ".kotowari/overview/*.md" で、".kotowari/overview/changes.md" に正しい`全体像の元データ`がある
  When "kotowari check --format json" を実行する
  Then "overview" の "files" は 1 である

@id=EX-core-465 @about=REQ-core-280 @source=docs/decision/records/2026-10-02-whole-picture.md#A58,docs/decision/records/2026-10-02-whole-picture.md#A75
Scenario: ガイドと重なる元データは設定の誤りになる
  Given "overview.files" と "guides.files" の両方が "docs/a.md" に当たる
  When "kotowari check" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は "config error: docs/a.md: matched by both overview.files and guides.files" である

@id=EX-core-466 @about=REQ-core-282 @source=docs/decision/records/2026-10-02-whole-picture.md#A46,docs/decision/records/2026-10-02-whole-picture.md#A69,docs/decision/records/2026-10-02-whole-picture.md#A73,docs/decision/records/2026-10-02-whole-picture.md#A74
Scenario: 知らない部品とスキーマに合わない部品は誤りになる
  Given `全体像の元データ`に "```view chart" のフェンスと、中身に知らない鍵 "color" を持つ "```view cards" のフェンスがある
  When "kotowari check --format json" を実行する
  Then detail が "chart" の overview_part_unknown と、detail が "cards" と半角空白と、"color" の鍵を持つ値の JSON Pointer に "/color" を足したものである overview_part_invalid の`誤り`が出る

@id=EX-core-511 @about=REQ-core-282 @source=docs/decision/records/2026-10-05-overview-page-reading.md#A10
Scenario: YAML として読めない部品は誤りの行を指す
  Given `全体像の元データ`の10行目に "```view cards" のフェンスがあり、その中の13行目が YAML として読めない行である
  When "kotowari check --format json" を実行する
  Then "line" が 13、detail が "cards (yaml)" の overview_part_invalid の`誤り`が1件出る

@id=EX-core-467 @about=REQ-core-283 @source=docs/decision/records/2026-10-02-whole-picture.md#A38,docs/decision/records/2026-10-02-whole-picture.md#A69,docs/decision/records/2026-10-02-whole-picture.md#A74
Scenario: 冒頭が lead でなければ誤りになる
  Given `題名`の直後の最初の`部品`が "```view steps" の`全体像の元データ` "x.md" がある
  When "kotowari check --format json" を実行する
  Then "line" が null で detail が "x.md" の overview_lead_missing の`誤り`が出る

@id=EX-core-468 @about=REQ-core-284 @source=docs/decision/records/2026-10-02-whole-picture.md#A19,docs/decision/records/2026-10-02-whole-picture.md#A69,docs/decision/records/2026-10-02-whole-picture.md#A74
Scenario: 無い IR の文書と2つの全体像に属する IR の文書は誤りになる
  Given "a.md" と "b.md" の2つの`全体像の元データ`の "ir" が両方 "docs/ir/core/cli.md" を持ち、"a.md" の "ir" が "docs/ir/core/none.md" も持つ
  When "kotowari check --format json" を実行する
  Then "a.md" に detail が "docs/ir/core/none.md" の overview_ir_missing が出て、"b.md" に detail が "docs/ir/core/cli.md" の overview_ir_shared が出る

@id=EX-core-469 @about=REQ-core-285 @source=docs/decision/records/2026-10-02-whole-picture.md#A66,docs/decision/records/2026-10-02-whole-picture.md#A33,docs/decision/records/2026-10-02-whole-picture.md#A69,docs/decision/records/2026-10-02-whole-picture.md#A74
Scenario: 解決できない参照は誤りになる
  Given steps の`部品`の "refs" に "REQ-core-001"、"REQ-core-999"、"docs/decision/records/records.md#A9999"、"foo" がある
  When "kotowari check --format json" を実行する
  Then detail が "REQ-core-999"、"docs/decision/records/records.md#A9999"、"foo" の overview_ref_unresolved の`誤り`が1件ずつ出て、"REQ-core-001" には出ない

@id=EX-core-470 @about=REQ-core-286 @source=docs/decision/records/2026-10-02-whole-picture.md#A39,docs/decision/records/2026-10-02-whole-picture.md#A58,docs/decision/records/2026-10-02-whole-picture.md#A32,docs/decision/records/2026-10-02-whole-picture.md#A69,docs/decision/records/2026-10-02-whole-picture.md#A73,docs/decision/records/2026-10-02-whole-picture.md#A77
Scenario: 元データの古いガイドの印は guide_stale になる
  Given `全体像の元データ`の節の "## " の見出しの次の行に、今の`指紋`と違う`指紋`を書いた`ガイドの印`がある
  When "kotowari check --format json" を実行する
  Then "path" がその`全体像の元データ`の guide_stale の`注意`が1件出る

@id=EX-core-471 @about=REQ-core-281 @source=docs/decision/records/2026-10-02-whole-picture.md#A55,docs/decision/records/2026-10-02-whole-picture.md#A69,docs/decision/records/2026-10-02-whole-picture.md#A71
Scenario: frontmatter に知らない鍵があれば形の誤りになる
  Given frontmatter に "ir" と "title" の鍵を持つ`全体像の元データ`がある
  When "kotowari check --format json" を実行する
  Then overview_form_invalid の`誤り`が出る
```
