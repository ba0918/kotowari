# 全体像の元データ

[English](overview-data.md) | 日本語

全体像の元データ（全体像のページを描くために LLM が書く、決まった形式の Markdown のファイル）の置き場と読み方、形、部品の中身の検査、冒頭の lead、扱う文書の一覧、部品の中の参照、元データの中のマーカーを扱う。元データの一覧と参照が決定の文書を指すこと、古さの見方は、[全体像の元データは、決定の文書を指す](languages-and-guides.ja.md#全体像の元データは決定の文書を指す)で決める。ここで見つけた指摘を check と status の出力にどう加えるかは[全体像の元データの申告](overview-output.ja.md)で、元データから全体像を作って見せるコマンドは[全体像を作るコマンドと見せるコマンド](overview-commands.ja.md)で決める。

## 全体像の元データは、リポジトリにコミットして保守し、overview.files の glob で選ぶ

状態: 実装済み

### 問題

全体像は話題ごとに1つあり、壁打ちの承認のたびに、関わる全体像を直していく。毎回一から作ると、費用がかかり、見た目もぶれる。直すには前回の元データが要る。古くなった節を見つけるマーカーも、元データの中にある。

元データは人が読む前提にしない。人は描いた結果のページだけを読む。人が直接読まない道具のデータは、設定やキャッシュと同じく ".kotowari/" の下に置く仲間になる。

元データの読み取りで止まる条件を落とすと、壊れた元データやリンク切れのファイルが黙って読み飛ばされ、そのページが欠けたまま検査と書き出しが進む。止める理由は記録に残っていない。

### 決めたこと

全体像の元データは保守の対象にし、リポジトリにコミットする。壁打ちの承認のたびに、前回の元データを直す。

元データは ".kotowari/overview/<話題>.md" に置き、設定の "overview.files" に、元データを指す glob を書く。check、status、"kotowari overview build"、"kotowari overview serve" は、この glob に当たるファイルを全体像の元データとして読む。全体像を有効にする鍵と、隠しディレクトリを glob で名指しして読む決まりは、[設定のキーは表のものだけにし、書かなかったキーは表の既定の値にする](config.ja.md#設定のキーは表のものだけにし書かなかったキーは表の既定の値にする)と[glob の "**" は再帰で読み、隠しディレクトリにはパスの成分で名指ししたときだけ入る](config.ja.md#glob-の--は再帰で読み隠しディレクトリにはパスの成分で名指ししたときだけ入る)で決める。

元データのファイルが UTF-8 でないとき、kotowari は UTF-8 でないファイルを理由に停止する。"overview.files" の走査で先の無いシンボリックリンクに出会ったときは、読めないファイルを理由に停止する。

### 見送った案

- 承認のたびに一から作る一時のデータにする。費用と見た目のぶれが戻る。
- 仕様の文書の置き場の下に置く。仕様の置き場の下は仕様の文書として読む決まりに当たり、仕様でない文書の例外が要る。
- "docs/overview/" に置く。人が読む文書の置き場に、人が読まないデータが並ぶ。

### 確認すること

- C-core-ovdata-1 "overview.files" の glob に当たるファイルを、check が全体像の元データとして読む
- C-core-ovdata-18 UTF-8 でない元データがあると、check は終了コード 2 で止まり、標準エラーの1行目が "non-UTF-8 file: " とそのパスで始まる
- C-core-ovdata-19 "overview.files" の走査で先の無いシンボリックリンクに出会うと、check は終了コード 2 で止まり、標準エラーの1行目が "unreadable file: " で始まる

## overview の鍵が無ければ、overview build と overview serve は設定の誤りで停止する

状態: 実装済み

### 問題

全体像は、設定に "overview" の鍵を書いたときだけ有効にし、黙って既定で有効にしない（[設定のキーは表のものだけにし、書かなかったキーは表の既定の値にする](config.ja.md#設定のキーは表のものだけにし書かなかったキーは表の既定の値にする)）。有効にしていないのに全体像を作るコマンドを実行したときも、黙って何かを作らずに止める。

### 決めたこと

設定に "overview" の鍵が無いとき、"kotowari overview build" と "kotowari overview serve" は、設定の誤りを理由に停止する。check と status は、元データを1つも読まない。

### 確認すること

- C-core-ovdata-2 設定に "overview" の鍵が無いと、"kotowari overview build" と "kotowari overview serve" は設定の誤りで停止する
- C-core-ovdata-17 設定に "overview" の鍵が無いと、check は ".kotowari/overview/" に形の崩れたファイルがあっても読まず、それを指す指摘を出さない

## 全体像の元データがガイドかテストの置き場と重なると、設定の誤りで停止する

状態: 実装済み

### 問題

全体像の元データ、ガイド、テストのファイルは、どれも glob で選ぶので、同じファイルに当たりうる。ガイドとテストの置き場の重なり（[ガイドとテストの置き場が重なると、設定の誤りで停止する](guides.ja.md#ガイドとテストの置き場が重なると設定の誤りで停止する)）と同じに扱う。

対の側の数え方を落とすと、"guides.files" の glob に当たっても先頭の言語の側の無い側と、glob に当たらないが同じ対にあるためにガイドとして読む側が、重なりの判定から外れる。その側は元データとしてもガイドとしても読まれ、マーカーが黙って二重に数えられる。重なりで止める理由は、これを黙って許さないことにある。判定の順と停止の詳細を落とすと、どの停止の文言が出るかが決まらない。判定の順と詳細の文言を決めた理由は、記録に残っていない。

### 決めたこと

1つのファイルが "overview.files" の glob と、"guides.files" か "tests.files" の glob の両方に当たるとき、kotowari は設定の誤りを理由に停止する。check、status、"kotowari overview build"、"kotowari overview serve" のどれでも同じにする。元データのマーカーは、全体像の元データとして1回だけ読む。

停止の詳細は、重なったファイルのうちパスのバイト順で最初の1つの、基準のディレクトリからの相対パスに、": matched by both overview.files and guides.files" か ": matched by both overview.files and tests.files" を続けたものにする。

ガイドとテストの置き場の重なりを、元データの重なりより先に判定する。3つの置き場すべてに当たるファイルでは、ガイドとテストの重なりで止まり、詳細は ": matched by both guides.files and tests.files" で終わる。

ガイドとの重なりには、"guides.files" の glob に当たるファイルに加えて、ガイドとして読むほかの言語の側を数える。"guides.files" の glob に当たる側は、先頭の言語の側が無くても数える。glob に当たらなくても、glob に当たったファイルと同じ対にあるためにガイドとして読む側も数える。

### 確認すること

- C-core-ovdata-3 1つのファイルが "overview.files" と "guides.files" の両方に当たると、check も "kotowari overview serve" も設定の誤りで停止する
- C-core-ovdata-4 1つのファイルが "overview.files" と "tests.files" の両方に当たると、check も "kotowari overview build" も設定の誤りで停止する
- C-core-ovdata-20 重なったファイルが "tests/a.md" と "tests/b.md" のとき、標準エラーの1行目は "config error: tests/a.md: matched by both overview.files and tests.files" になり、ガイドとの重なりでは最後が ": matched by both overview.files and guides.files" になる
- C-core-ovdata-21 1つのファイルが "overview.files"、"guides.files"、"tests.files" の3つすべてに当たると、check も "kotowari overview build" も、詳細が ": matched by both guides.files and tests.files" で終わる設定の誤りで停止する
- C-core-ovdata-22 言語が2つ以上で、先頭の言語の側の無いほかの言語の側だけが "overview.files" と "guides.files" の両方に当たると、check、status、"kotowari overview build" は設定の誤りで停止する

## 全体像の元データは、frontmatter と Markdown の本文で書き、部品は view の付いたフェンスの中に YAML で書く

状態: 実装済み

### 問題

全体像の構成（使う図や表の種類、並べ方）は、題材によって最適なものが変わるので、文書ごとに変えられる。一方で、決まった描画でページを作るには、元データの形が決まっていなければならない。元データの中の部品の書き方は、ほかの Markdown の道具で開いたときに壊れて見えず、中身の書式を十分に検査できることが条件だった。

### 決めたこと

全体像の元データは、先頭の frontmatter と、本文の Markdown で書く。

frontmatter の鍵は "ir" の1つだけで、その全体像が扱う文書のパスを、基準のディレクトリからの相対で並べる（一覧が指すのは決定の文書で、[全体像の元データは、決定の文書を指す](languages-and-guides.ja.md#全体像の元データは決定の文書を指す)で決める。鍵の名前は[改名する](#全体像の元データの一覧の鍵-ir-と指摘-overview_ir_missingoverview_ir_shared-は改名する)）。知らない鍵は誤りにする。題名は frontmatter に入れず、本文の "# " の見出しを使う。

部品は、情報文字列を "view <種類>" にしたフェンスのコードブロックの中に、YAML で書く。フェンスの中なら、ほかの Markdown の道具からは、ただのコードブロックに見える。マーカーは、ガイドと同じく、節の見出しの隣に HTML のコメントで書く。

Markdown としての形の検査は kotowari-markdown-schema が行い、形に合わない所は overview_form_invalid の誤りにする。overview_form_invalid の detail は、[指摘の detail は、その指摘を特定する短い文字列にし、種類ごとに表で決める](findings.ja.md#指摘の-detail-はその指摘を特定する短い文字列にし種類ごとに表で決める)で決める。

### 確認すること

- C-core-ovdata-5 frontmatter に "ir" のほかの鍵がある元データには、check が overview_form_invalid の誤りを出す
- C-core-ovdata-6 題名の "# " の見出しが無い元データには、check が overview_form_invalid の誤りを出す
- C-core-ovdata-7 情報文字列が "view" と種類の名前のフェンスのコードブロックを、check が部品として読み、その種類と中身を検査する

## 部品の中身は、kotowari の側が view の部品のスキーマで検査する

状態: 実装済み

### 問題

部品の種類は、view が持つ固定の一覧にした（[部品の種類は、view が持つ8種の固定の一覧にする](../view/parts.ja.md#部品の種類はview-が持つ8種の固定の一覧にする)）。部品の中身の形は、view が自分の入力の形として部品のスキーマで持つ（[部品の形の正は、view が埋め込んで公開する部品のスキーマにする](../view/parts.ja.md#部品の形の正はview-が埋め込んで公開する部品のスキーマにする)）が、view は検査をしない。元データを読んで誤りを報告するのは、kotowari の側の仕事である。

kotowari-markdown-schema は、元データの Markdown としての形だけを扱う。コードブロックの中身は Markdown ではないので、その検査は kotowari-markdown-schema の責務に入れない。

### 決めたこと

kotowari-markdown-schema が元データの Markdown としての形を検査し、部品のコードブロック（情報文字列と中身）を取り出す。kotowari の側は、中身を YAML として読み、情報文字列で選んだ種類の、view が公開する部品のスキーマで検査して、指摘にする。

種類の名前が view の部品の種類に無い部品は、overview_part_unknown の誤りにする。中身が YAML として読めない部品と、部品のスキーマに合わない部品は、overview_part_invalid の誤りにする。overview_part_invalid の detail は、部品の種類と、合わなかった場所の JSON Pointer にし、検査のライブラリが作る文を入れない。版で変わる文を detail に入れないためである。

中身が YAML として読めないときは、overview_part_invalid の行を、YAML の読み取りが誤りの位置として返した行を元データのファイルの行に直したものにし、detail は種類の名前と "(yaml)" にする。フェンスの開始の行と "(yaml)" だけでは、長い部品で直す場所を探せない。位置が返らないときは、フェンスの開始の行にする。

### 見送った案

- kotowari-markdown-schema に、コードブロックの中身を構造として検査する汎用の機能を足す。コードブロックの中身は Markdown ではない。

### 確認すること

- C-core-ovdata-8 知らない種類の部品には overview_part_unknown の誤りが、部品のスキーマに合わない部品には overview_part_invalid の誤りが出る
- C-core-ovdata-9 overview_part_invalid の detail は、部品の種類と、合わなかった場所の JSON Pointer で、検査のライブラリが作る文を含まない
- C-core-ovdata-10 YAML として読めない部品の overview_part_invalid は、YAML の誤りの行を元データのファイルの行に直して指し、detail は種類の名前と "(yaml)" になる

## 題名の直後の最初の部品は、lead にする

状態: 実装済み

### 問題

全体像の冒頭に結論と要約を置く構成は、形を比べて正解だった。構成は文書ごとに自由だが、冒頭に結論と要約を置くことは、どの文書にも当たる決まりなので、機械で守らせる。

### 決めたこと

全体像の元データの題名の直後の最初の部品は、結論と要約の部品（lead）でなければならない。そうでなければ、check が overview_lead_missing の誤りを出す。

### 確認すること

- C-core-ovdata-11 題名の直後の最初の部品が lead でない元データには、check が overview_lead_missing の誤りを出す

## 1つの決定の文書は、多くても1つの全体像の元データの一覧に入れる

状態: 提案

### 問題

壁打ちで決定の文書を変えたとき、直す全体像が機械で1つに決まるようにしたい。

### 決めたこと

元データの frontmatter の一覧（今の鍵は "ir"）には、その全体像が扱う決定の文書を書く。1つの決定の文書は、多くても1つの全体像の元データの一覧に入れる。2つ以上の元データの一覧にあれば、check が overview_ir_shared の誤りを出す。一覧の1件が無い決定の文書を指すとき（overview_ir_missing）は、[全体像の元データは、決定の文書を指す](languages-and-guides.ja.md#全体像の元データは決定の文書を指す)で決める。鍵と2つの指摘の名前は[改名する](#全体像の元データの一覧の鍵-ir-と指摘-overview_ir_missingoverview_ir_shared-は改名する)。

どの全体像の一覧にも入っていない決定の文書を指摘する検査は、作らない。

### 確認すること

- C-core-ovdata-12 1つの決定の文書が2つの全体像の元データの一覧にあると、check が overview_ir_shared の誤りを出す

## 全体像の元データの一覧の鍵 ir と、指摘 overview_ir_missing、overview_ir_shared は改名する

状態: 提案

### 問題

切り替えのあと、全体像の元データの一覧が指すのは決定の文書で、IR は無くなる。今の鍵 "ir" と、指摘の種類 overview_ir_missing、overview_ir_shared の名前は、無くなる IR を指す語のまま残っている。改名する理由は、これのほかに記録に残っていない。

### 決めたこと

全体像の元データの frontmatter の一覧の鍵 "ir" と、一覧の1件が無い決定の文書を指すときの指摘 overview_ir_missing と、1つの決定の文書が2つ以上の元データの一覧にあるときの指摘 overview_ir_shared を改名する。

### 確認すること

- C-core-ovdata-23 全体像の元データの一覧の鍵と、無い決定の文書を指す指摘と、2つの元データの一覧にある決定の文書の指摘の名前が、"ir"、overview_ir_missing、overview_ir_shared でなくなる

## 部品の中の参照は、refs と ref の欄に書く

状態: 実装済み

### 問題

どの欄が参照かを部品の種類ごとに決めると、部品の種類が増えるたびに、参照の集め方を変えることになる。参照が解決できないまま描くより、check で止めて直させたい。check に入れれば、コミットの前の検査でも捕まる。

### 決めたこと

部品の YAML の中の参照は、名前が "refs"（文字列の一覧）か "ref"（文字列1つ）の欄に書く。kotowari は、部品の中のこの名前の欄を集めて参照として読む。参照が何を指せるかと、どの参照を解決できないとするかは、[全体像の元データは、決定の文書を指す](languages-and-guides.ja.md#全体像の元データは決定の文書を指す)で決める。解決できない参照は、check が overview_ref_unresolved の誤りにする。

### 確認すること

- C-core-ovdata-13 部品の "refs" の1件と "ref" の欄の文字列を、check が参照として読み、解決できない参照に overview_ref_unresolved の誤りを出す

## 全体像の元データのマーカーは、ガイドのマーカーと同じ規則で読む

状態: 実装済み

### 問題

全体像のどの節が、どの決定を写したかを、ガイドと同じくマーカーで持つ。決定が変わったあと、まだ見直していない節を見つけたい。判断のつながりの古さは指摘にせず、ページの上で見えれば足りる。全体像を直すきっかけは、マーカーの古さと、壁打ちの承認のたびの更新である。

### 決めたこと

check は、全体像の元データの中のマーカーを、ガイドのマーカーと同じ規則（[ガイドのマーカー](guides.ja.md)）で読んで照らし、形の誤りは invalid_marker の誤り、古いものは guide_stale の注意にする。指摘の "path" は、その全体像の元データにする。元データは、ガイドとして二重に読まない。

節のマーカーは、その節の "## " の見出しの隣に置く。節が古いかどうかを、マーカーの古さから描画に渡すことは、[マーカーが古い節は、古い節として描画に渡す](overview-commands.ja.md#マーカーが古い節は古い節として描画に渡す)で決める。

### 確認すること

- C-core-ovdata-14 全体像の元データの中のマーカーの1件が古いと、check が "path" をその元データにした guide_stale の注意を出す
- C-core-ovdata-15 全体像の元データの中の形の誤ったマーカーには、check が invalid_marker の誤りを出す

## overview build と overview serve は、テストのファイルのパスだけを集め、中身を読まない

状態: 実装済み

### 問題

全体像の出力は、テストの解析の結果を使わない。使わない入力の問題で書き出しが止まると、利用者には止まる理由が分からない。テストの側の問題は check が指摘する。読まない分だけ、書き出しも速くなる（時間は測っていない）。

### 決めたこと

"kotowari overview build" と "kotowari overview serve" は、テストのファイルのパスだけを集め、その中身と "tests.rules" を読まない。パスは、[全体像の元データがガイドかテストの置き場と重なると、設定の誤りで停止する](#全体像の元データがガイドかテストの置き場と重なると設定の誤りで停止する)の判定に使う。パスを集める走査での停止は、check と同じに残す。

### 確認すること

- C-core-ovdata-16 "tests.files" に UTF-8 でないファイルが当たっても、"kotowari overview build" は止まらずにページを書く

## overview build と overview serve も、check と同じ置き場から決定の文書を読み、同じ条件で停止する

状態: 実装済み

### 問題

"kotowari overview build" と "kotowari overview serve" は、部品の参照を解くために決定の文書を読む。この停止の条件を落とすと、check なら止まる置き場の誤り（置き場が無い、読めない）を、build と serve が通して書き出しうる。check と同じにする理由は、記録に残っていない。

### 決めたこと

"kotowari overview build" と "kotowari overview serve" は、check と同じ設定と置き場から決定の文書を読む。決定の文書の置き場が無いか読めないときは、check と同じ理由と詳細で停止する（[決定の文書の置き場が無いか、ディレクトリでないか、読めなければ、読めないファイルで停止する](config.ja.md#決定の文書の置き場が無いかディレクトリでないか読めなければ読めないファイルで停止する)）。

### 確認すること

- C-core-ovdata-24 決定の文書の置き場が無いと、"kotowari overview build" は終了コード 2 で止まり、標準エラーの1行目が check と同じ "unreadable file: " とその置き場のパスで始まる
