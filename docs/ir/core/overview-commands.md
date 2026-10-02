# 全体像を作るコマンドと見せるコマンド

"kotowari overview build" が`全体像の元データ`から`参照の表`と古い節を求めて描画のエンジン（kotowari-markdown-view）に渡し、`全体像`のファイルを書くまでと、"kotowari overview serve" がそれを手元で見せるまでを扱う。

## Requirements

### REQ-core-291: 参照の表を作る

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A25, docs/decision/records/2026-10-02-whole-picture.md#A65, docs/decision/records/2026-10-02-whole-picture.md#A66, docs/decision/records/2026-10-02-whole-picture.md#A67, docs/decision/records/2026-10-02-whole-picture.md#A72, docs/decision/records/2026-10-02-whole-picture.md#A73
- definition: TBL-core-039
- verification: unit

kotowari は常に、"kotowari overview build" で、すべての`全体像の元データ`の`部品`の中の参照（REQ-core-285）の1つずつについて、TBL-core-039 の表示名、本文、状態を持つ`参照の表`の1件を作って描画のエンジンに渡す。

### REQ-core-292: 古い節を求める

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A39, docs/decision/records/2026-10-02-whole-picture.md#A68, docs/decision/records/2026-10-02-whole-picture.md#A73
- verification: unit

kotowari は常に、"kotowari overview build" で、`全体像の元データ`の "## " の見出しの節のうち、その節の中に guide_stale の`注意`を受ける`ガイドの印`が1件以上あるものを古い節として描画のエンジンに渡す。最初の "## " の見出しより前の`ガイドの印`はどの節にも属さず、古い節を作らない。描画のエンジンに渡す見出しの文字からは HTML のコメントを除く。

### REQ-core-293: 全体像を書く

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A16, docs/decision/records/2026-10-02-whole-picture.md#A26, docs/decision/records/2026-10-02-whole-picture.md#A29, docs/decision/records/2026-10-02-whole-picture.md#A56
- verification: unit

kotowari は常に、"kotowari overview build" で、すべての`全体像の元データ`を毎回描画し、描画のエンジンが返した`全体像`のファイルのうち、`基準のディレクトリ`の ".kotowari/cache/overview/" の下に同じ名前で同じバイト列のファイルが無いものだけを書き、その置き場の下にあって今回返されなかったファイルを消す。ファイルの名前は、一覧が "index.html"、`全体像`ごとに`全体像の元データ`のファイル名の ".md" を ".html" にしたもの、スタイルが "style.css" である。

### REQ-core-294: 元データに誤りがあれば書かない

- kind: event_driven
- source: docs/decision/records/2026-10-02-whole-picture.md#A33, docs/decision/records/2026-10-02-whole-picture.md#A61, docs/decision/records/2026-10-02-whole-picture.md#A63, docs/decision/records/2026-10-02-whole-picture.md#A79
- verification: unit

"kotowari overview build" か "kotowari overview serve" で、`全体像の元データ`に REQ-core-278 から REQ-core-286 と REQ-core-305 の`誤り`が1件以上あるとき、kotowari はファイルを1つも書かず消さずに、元データの誤りを理由に`停止`し、詳細を`誤り`の件数と "errors in overview data; run kotowari check" にする。

### REQ-core-295: build の出力

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A62, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-02-whole-picture.md#A78
- verification: unit

kotowari は常に、"kotowari overview build" が書き終えたとき、"--format" が "json"（既定）なら "written"（書いたファイルのパスの一覧）、"removed"（消したファイルのパスの一覧）、"unchanged"（書かなかったファイルの数）の3つの鍵だけを持つ JSON を1つ出し、"text" なら書いたファイルを "written <パス>"、消したファイルを "removed <パス>" の1行ずつ、written の行をすべて出してから removed の行を出す。パスは`基準のディレクトリ`からの相対で、一覧と同じ種類の行はパスのバイト順に並べる。

### REQ-core-296: 書く先は全体像の置き場だけ

- kind: prohibition
- source: docs/decision/records/2026-10-02-whole-picture.md#A28
- verification: unit

kotowari は、"kotowari overview build" と "kotowari overview serve" で、`基準のディレクトリ`の ".kotowari/cache/overview/" の下のほかにファイルを書き、消してはならない。書く先を設定で変えられるようにしてはならない。

### REQ-core-297: serve で見せる

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A27, docs/decision/records/2026-10-02-whole-picture.md#A30, docs/decision/records/2026-10-02-whole-picture.md#A57, docs/decision/records/2026-10-02-whole-picture.md#A63, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-02-whole-picture.md#A79
- verification: unit

kotowari は常に、"kotowari overview serve" で、`全体像の元データ`の検査、"127.0.0.1" の "--port" の値（既定 "4590"）のポートの確保、"kotowari overview build" と同じ描画と書き込みの順に行い、".kotowari/cache/overview/" の下のファイルを HTTP で配り、標準出力に "http://127.0.0.1:<ポート>/" の1行だけを出して、割り込み（Ctrl-C）まで配り続け、割り込みで終了コード0で終わる。リクエストごとに何も出力しない。".html" は "text/html; charset=utf-8"、".css" は "text/css; charset=utf-8" の Content-Type で返し、置き場の中に無いファイルの道には 404 を返す。

### REQ-core-298: ポートが使えないとき

- kind: event_driven
- source: docs/decision/records/2026-10-02-whole-picture.md#A57, docs/decision/records/2026-10-02-whole-picture.md#A72
- verification: unit

"kotowari overview serve" で、"127.0.0.1" の指定のポートを使えないとき、kotowari はほかのポートを試さずに、読めないファイルでなく引数の誤りでもない、ポートの誤りを理由に`停止`し、詳細を "127.0.0.1:<ポート>" と OS の誤りの文にする。

### REQ-core-299: serve が配る範囲

- kind: prohibition
- source: docs/decision/records/2026-10-02-whole-picture.md#A59, docs/decision/records/2026-10-02-whole-picture.md#A72
- verification: unit

kotowari は、"kotowari overview serve" で、".kotowari/cache/overview/" の外を指すリクエストの道（".." の成分、絶対パス、シンボリックリンクを辿った先が外になるもの）に、その置き場の外のファイルの中身を返してはならず、404 を返す。ディレクトリの一覧を返してはならない。道が "/" のときは "index.html" を返す。

### REQ-core-305: ページの名前の重なり

- kind: event_driven
- source: docs/decision/records/2026-10-02-whole-picture.md#A56, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-02-whole-picture.md#A74
- verification: unit

`全体像の元データ`のファイル名から ".md" を除いた名前が、ほかの`全体像の元データ`と同じとき、または "index" か "style" のとき、kotowari は "kotowari check" と "kotowari status" で、パスのバイト順で2つ目以降の重なった`全体像の元データ`と、名前が "index" か "style" の`全体像の元データ`に、"line" を null、detail をその名前にして overview_name_conflict の`誤り`を出す。

## Decision tables

### TBL-core-039: 参照の表の1件

- source: docs/decision/records/2026-10-02-whole-picture.md#A65, docs/decision/records/2026-10-02-whole-picture.md#A67, docs/decision/records/2026-10-02-whole-picture.md#A72, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-02-whole-picture.md#A81, docs/decision/records/2026-10-02-whole-picture.md#A85, docs/decision/records/2026-10-02-whole-picture.md#A86

| 参照の形 | 表示名 | 本文 | 状態 |
|---|---|---|---|
| `IR`の`要求`の`ID` | `ID` | その`要求`の`文`の並び（algorithm なら名前と "- definition:" の値） | `後回し`なら "deferred"、ほかは "current" |
| `IR`の`決定表`、`性質`の`ID` | `ID` | `決定表`は名前、`性質`は`文`の並び | "current" |
| `シナリオ`の`ID` | `ID` | "Scenario:" の行とステップの行の並び | `後回しのシナリオ`なら "deferred"、ほかは "current" |
| `判断の記録`の決定を指す`出典`の形 | ファイル名から先頭の "YYYY-MM-DD-" と末尾の ".md" を除いたものと、1つの半角空白と`決定の番号` | その`番号の行`の、`決定の番号`より後の文字 | その決定に値の空でない "- superseded_by:" の行があれば "superseded"、ほかは "current" |
| `問題の記録`の`項目`の`ID` | `ID` | その`項目`の本文の行の並び | "current" |
| 判断の記録でない Markdown のファイル（ADR、形の契約、補足の文書）の見出しを指す`出典`の形（TBL-core-012 の4） | ファイル名から末尾の ".md" を除いたものと、1つの半角空白と見出しの文字 | その見出しの節の文の並び | "current" |

## Examples

```gherkin
@id=EX-core-474 @about=REQ-core-291 @source=docs/decision/records/2026-10-02-whole-picture.md#A65,docs/decision/records/2026-10-02-whole-picture.md#A67,docs/decision/records/2026-10-02-whole-picture.md#A72,docs/decision/records/2026-10-02-whole-picture.md#A81,docs/decision/records/2026-10-02-whole-picture.md#A85,docs/decision/records/2026-10-02-whole-picture.md#A66,docs/decision/records/2026-10-02-whole-picture.md#A86
Scenario: 置き換えられた決定と後回しの要求の状態が参照の表に入る
  Given `部品`の "refs" に、値の空でない "- superseded_by:" を持つ決定 "docs/decision/records/2026-01-01-x.md#A1" と、`後回し`の`要求` "REQ-core-900" がある
  When "kotowari overview build" を実行する
  Then 描画のエンジンに渡す`参照の表`で、前者は表示名 "x A1"、状態 "superseded"、後者は表示名 "REQ-core-900"、状態 "deferred" である

@id=EX-core-475 @about=REQ-core-293,REQ-core-295 @source=docs/decision/records/2026-10-02-whole-picture.md#A29,docs/decision/records/2026-10-02-whole-picture.md#A62,docs/decision/records/2026-10-02-whole-picture.md#A56,docs/decision/records/2026-10-02-whole-picture.md#A28,docs/decision/records/2026-10-02-whole-picture.md#A42,docs/decision/records/2026-10-02-whole-picture.md#A82
Scenario: 2回目の build は変わったファイルだけを書く
  Given "a.md" と "b.md" の2つの`全体像の元データ`で1回 build した後、"b.md" の文だけを変えた
  When "kotowari overview build --format json" を実行する
  Then "written" は ".kotowari/cache/overview/b.html" だけで、"removed" は空、"unchanged" は 3 で、"a.html" の更新時刻は変わらない

@id=EX-core-476 @about=REQ-core-293 @source=docs/decision/records/2026-10-02-whole-picture.md#A29,docs/decision/records/2026-10-02-whole-picture.md#A56,docs/decision/records/2026-10-02-whole-picture.md#A62
Scenario: 無くなった元データのページは消える
  Given "a.md" と "b.md" で build した後、"b.md" を消した
  When "kotowari overview build --format json" を実行する
  Then "removed" は ".kotowari/cache/overview/b.html" で、そのファイルは無い

@id=EX-core-477 @about=REQ-core-294,REQ-core-296 @source=docs/decision/records/2026-10-02-whole-picture.md#A33,docs/decision/records/2026-10-02-whole-picture.md#A61,docs/decision/records/2026-10-02-whole-picture.md#A38
Scenario: 元データに誤りがあれば何も書かずに止まる
  Given `題名`の直後に "## " の見出しがあって lead の`部品`が無い`全体像の元データ`だけが`誤り`を持ち、".kotowari/cache/overview/" に前の build のファイルがある
  When "kotowari overview build" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は "overview error: 1 errors in overview data; run kotowari check" で、置き場のファイルの一覧と中身は実行の前と等しい

@id=EX-core-478 @about=REQ-core-297,REQ-core-299 @source=docs/decision/records/2026-10-02-whole-picture.md#A30,docs/decision/records/2026-10-02-whole-picture.md#A59,docs/decision/records/2026-10-02-whole-picture.md#A63,docs/decision/records/2026-10-02-whole-picture.md#A72,docs/decision/records/2026-10-02-whole-picture.md#A56
Scenario: serve は置き場の中だけを配る
  Given 正しい`全体像の元データ`があり、ポート "4591" が空いている
  When "kotowari overview serve --port 4591" を起動し、"/"、"/style.css"、"/../config.yaml"、"/%2e%2e/config.yaml" を要求する
  Then 標準出力は "http://127.0.0.1:4591/" の1行で、"/" には "index.html" の中身、"/style.css" にはその中身が返り、残りの2つには 404 が返る

@id=EX-core-479 @about=REQ-core-298 @source=docs/decision/records/2026-10-02-whole-picture.md#A57,docs/decision/records/2026-10-02-whole-picture.md#A72
Scenario: 使用中のポートでは止まる
  Given "127.0.0.1:4592" をほかのプロセスが使っている
  When "kotowari overview serve --port 4592" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は "port error: 127.0.0.1:4592" で始まる
```
