# 出力の形

[English](output.md) | 日本語

"--format" で選ぶ出力の形と、check の JSON の中身と、文字の出力の1行の形を扱う。指摘の種類と detail は[指摘の種類](findings.ja.md)、指摘の並べ方と line は[指摘の並べ方と行](finding-order.ja.md)で決める。mutants の JSON の最上位は[集計を必ず出し、等価として外した件数を毎回見せる](mutants.ja.md#集計を必ず出し等価として外した件数を毎回見せる)、plan の JSON の最上位は[kotowari plan の出力は check と同じ指摘の鍵を使い、JSON は findings と counts だけにする](plan.ja.md#kotowari-plan-の出力は-check-と同じ指摘の鍵を使いjson-は-findings-と-counts-だけにする)で決める。

## "--format" は json と text の2つを受け、既定を json にする

状態: 実装済み

### 問題

kotowari を第一に使うのは LLM なので（[kotowari は、第一に LLM が使う CLI にする](cli-scope.ja.md#kotowari-は第一に-llm-が使う-cli-にする)）、出力は構造化された形が基本になる。人が読む文字の出力は、中身を詰めて作らないと読みづらい。

### 決めたこと

出力の形は "--format" で選ぶ。値は "json" と "text" の2つで、"--format" を付けなければ "json" にする。

"json" のときは、標準出力に1つの JSON を出す。"text" のときの形は[--format text は、1つの指摘を1行で出す](#--format-text-は1つの指摘を1行で出す)で決める。

### 確認すること

- C-core-output-1 "--format" を付けない check は、標準出力の全体が1つの JSON の値になる
- C-core-output-2 "--format json" の check は標準出力の全体が1つの JSON の値で、"--format text" の check は JSON を出さない

## check の JSON の最上位には、指摘と種類ごとの数と、読んだものの数を出す

状態: 実装済み

### 問題

check の JSON の最上位に何を出すかを決める必要があった。テストのファイルやガイドを探す glob を書き誤ると、読んだものが0件になる。数を出さなければ、指摘が0件なのか、何も読まなかったのかが区別できず、誤りが黙って通る。全体像の元データの数も、ガイドと同じ扱いにする。面の規則を書かないプロジェクトには、面の検査について何も起きないようにする。

### 決めたこと

"kotowari check" の JSON の最上位には、次の鍵を出す。

| 鍵 | 中身 |
|---|---|
| findings | 指摘の一覧。1件の鍵は[指摘は kind、severity、path、line、detail の5つの鍵で出す](#指摘は-kindseveritypathlinedetail-の5つの鍵で出す)のとおり |
| counts | 種類ごとの指摘の数。findings に1件も無い種類は持たない |
| tests | 読んだテストのファイルの、拡張子ごとの数（[check の JSON の "tests" は、読んだテストのファイルを拡張子ごとに数える](#check-の-json-の-tests-は読んだテストのファイルを拡張子ごとに数える)） |
| guides | "files"（読んだガイドの数）と "marks"（ガイドのマーカーの中の1件ずつの数）の2つの鍵を持つ。ガイドが0件なら、どちらも 0 |
| overview | "files"（読んだ全体像の元データの数）と "marks"（元データの中のガイドのマーカーの1件ずつの数）の2つの鍵を持つ |
| surface | "unspecified"（未記載の面の一覧で外した面の数）の鍵を1つ持つ。"surface.rules" が空の一覧のときは、鍵ごと出さない |
| decisions | "documents" に、読んだ決定の文書の数を持つ |

### 確認すること

- C-core-output-3 決定の文書の置き場を読んだ check の JSON の最上位に findings、counts、tests、guides、overview、decisions の鍵があり、"surface.rules" に規則があるときは surface の鍵もある
- C-core-output-4 どんな指摘の組でも、counts の各種類の値は findings の中のその種類の指摘の数に等しく、findings に1件も無い種類は counts に無い
- C-core-output-5 guides は、読んだガイドの数を "files" に、マーカーの中の1件ずつの数を "marks" に持ち、ガイドが0件なら両方とも 0 になる

## 指摘は kind、severity、path、line、detail の5つの鍵で出す

状態: 実装済み

### 問題

JSON の1件の指摘が持つ鍵を決める必要があった。理由は残っていない。

### 決めたこと

JSON の指摘は、次の5つの鍵を持つ。

| 鍵 | 中身 |
|---|---|
| kind | 指摘の種類（[指摘の種類](findings.ja.md)） |
| severity | "error"（誤り）か "notice"（注意）。"warning" は使わない |
| path | 基準のディレクトリからの相対パス（[設定ファイルの値のパスと、出力の "path" は、基準のディレクトリからの相対にする](base-directory.ja.md#設定ファイルの値のパスと出力の-path-は基準のディレクトリからの相対にする)）。決定の文書への指摘では、正規化した決定の文書の置き場と、置き場からの文書の相対パスを "/" でつないだもの。決定の文書でないファイルへの指摘の path は、その指摘を決めた話題で決める（例: テストのファイル、ガイド、面のファイル、未記載の面の一覧） |
| line | 行（1始まり）。文書全体への指摘は null（[line は1始まりで数え、文書全体にかかる指摘は null にする](finding-order.ja.md#line-は1始まりで数え文書全体にかかる指摘は-null-にする)） |
| detail | 種類ごとに決めた文字列（[指摘の detail は、その指摘を特定する短い文字列にし、種類ごとに表で決める](findings.ja.md#指摘の-detail-はその指摘を特定する短い文字列にし種類ごとに表で決める)） |

### 確認すること

- C-core-output-6 JSON の指摘は kind、severity、path、line、detail の5つの鍵を持ち、severity は "error" か "notice" で、line は数か null である
- C-core-output-7 置き場の下のディレクトリにある決定の文書への指摘の path は、正規化した置き場と、置き場からの文書の相対パスを "/" でつないだものになる（置き場を "./docs//decisions/./" と書いても "docs/decisions/sub/a.md"）

## --format text は、1つの指摘を1行で出す

状態: 実装済み

### 問題

文字の出力の1行の形を決める必要があった。ファイル名が改行を含むと、1つの指摘が見た目の2行に割れて、行で読む道具の前提が崩れる。集計の数を同じ出力に混ぜても、1つの指摘を1行で出す形が崩れる。

### 決めたこと

"--format" が "text" のとき、1つの指摘を1行で、"パス:行 [error] 種類 詳細" か "パス:行 [notice] 種類 詳細" の形で出す。角括弧も出す。

指摘の line が null のときは、行を "-" と書く。

パスと詳細の中の改行（"\n" と "\r"）は、バックスラッシュと "n" または "r" の2文字で書く。JSON の出力では書き換えない。

check の "--format text" には、指摘の行のほかに、テストのファイルの数を出さない。未記載の面の数の行を最後に出すことは、[未記載の面の一覧で外した面の数を、check の出力の最後に必ず出す](surface.ja.md#未記載の面の一覧で外した面の数をcheck-の出力の最後に必ず出す)で決める。

### 確認すること

- C-core-output-8 "--format text" の check は、1つの指摘を "パス:行 [error] 種類 詳細" か "パス:行 [notice] 種類 詳細" の形の1行で出す
- C-core-output-9 "--format text" で、line が null の指摘は、行の所に "-" を書いた1行になる
- C-core-output-10 "--format text" で、パスか詳細に "\n" か "\r" を含む指摘は、それをバックスラッシュと "n" か "r" の2文字にして、1行で出る
- C-core-output-11 指摘の無い check の "--format text" は、テストのファイルを読んでいても何も出さない

## check の JSON の "tests" は、読んだテストのファイルを拡張子ごとに数える

状態: 実装済み

### 問題

問い合わせの無い言語のテストのファイルは、構文木を読まず、マーカーを拾うだけで済ませている。そのことが出力から読めなかった。問い合わせの無い言語のファイルごとに指摘を出すと、ファイルの問題ではなく道具の届く範囲の問題なのに、ファイルの数だけ指摘が出て邪魔になる。

### 決めたこと

check の JSON の "tests" はオブジェクトにし、読んだテストのファイルの拡張子を鍵、"files" と "query" の2つの鍵を持つオブジェクトを値にする。テストのファイルが0件なら、"tests" は空のオブジェクトにする。

| 鍵 | 値 |
|---|---|
| 拡張子 | ファイル名の最後の "." より後ろの文字。"." を含めない。"." の無い名前、先頭の "." だけの名前（".rs"）、"." で終わる名前（"foo."）は、鍵を空の文字列にする。大文字と小文字を区別する。鍵はバイト順に並べる |
| files | その拡張子の、読んだテストのファイルの数。構文の誤りで unparsable_file を出したファイルも数える。ファイルはパスごとに1回数え、複数の glob に当たっても1回、シンボリックリンクと実体は別のパスとして数える |
| query | その拡張子から決まる言語が、問い合わせのある言語（同梱か設定のルールが1つ以上ある言語。[問い合わせが1つも無い言語のファイルは構文木を読まない](test-discovery.ja.md#問い合わせが1つも無い言語のファイルは構文木を読まない)）なら true、そうでなければ false |

### 確認すること

- C-core-output-12 "tests.files" の glob に ".rs" のファイルが2つと、問い合わせの無い ".go" のファイルが1つ当たると、"tests" は "rs" が files 2・query true、"go" が files 1・query false になり、ほかの鍵を持たない
- C-core-output-13 "tests" の鍵は最後の "." より後ろの文字で、"." の無い名前・".rs"・"foo." は空の文字列の鍵にまとまり、"RS" と "rs" は別の鍵になり、鍵はバイト順に並ぶ
- C-core-output-14 同じファイルが2つの glob に当たっても1回と数え、シンボリックリンクと実体は別に数え、構文の誤りで読めないファイルも files に数える
- C-core-output-15 テストのファイルが0件なら、"tests" は空のオブジェクトになる
- C-core-output-16 どんなテストのファイルの組でも、"tests" の各拡張子の files の合計は、読んだテストのファイルの数に等しい
