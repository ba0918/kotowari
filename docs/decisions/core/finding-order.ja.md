# 指摘の並べ方と行

[English](finding-order.md) | 日本語

kotowari のコマンドが出す指摘を、どの順に並べるかと、指摘の line をどう決めるかを扱う。計画書の指摘の行は[形の指摘は invalid_plan の1種類にまとめ、スキーマの側の種類と詳細を detail に入れる](plan.ja.md#形の指摘は-invalid_plan-の1種類にまとめスキーマの側の種類と詳細を-detail-に入れる)で決める。

## 指摘は path、line、kind、detail の順に並べる

状態: 実装済み

### 問題

check が出す "findings" の並びを決める必要があった。のちに mutants の指摘を足したときは、check の指摘と同じ並べ方にした。check の指摘と同じ形なら、受け取る側が1つの読み方で済むためである。この4つの鍵とその順を選んだ理由は残っていない。

### 決めたこと

"findings" の指摘は、次の鍵を上から順に比べて並べる。mutants の指摘をこれと同じ並べ方にすることは、[集計を必ず出し、等価として外した件数を毎回見せる](mutants.ja.md#集計を必ず出し等価として外した件数を毎回見せる)で決める。

| 順 | 鍵 | 並べ方 |
|---|---|---|
| 1 | path | バイト順 |
| 2 | line | null が先、その後は小さい順 |
| 3 | kind | バイト順 |
| 4 | detail | バイト順 |

### 確認すること

- C-core-forder-1 複数のファイルに指摘が出る check の "findings" は、path のバイト順、同じ path の中では line が null のものが先でその後は小さい順、続いて kind、detail のバイト順に並ぶ
- C-core-forder-2 どんな数のファイルと指摘でも、"findings" の中で隣り合う2つの指摘は、上の順で比べて後ろのものが前のものより先に来ない

## line は1始まりで数え、文書全体にかかる指摘は null にする

状態: 実装済み

### 問題

指摘の "line" の数え方と、1つの行を指せない指摘の扱いを決める必要があった。理由は残っていない。

### 決めたこと

指摘の "line" は1始まりで数える。

1つの行を指せず、文書やファイルの全体にかかる指摘は、"line" を null にする。

どの種類の指摘がこれに当たるかは、それぞれの指摘を決めた話題で決める。文書の題名の missing_title と multiple_titles、読めないファイルの unparsable_file、等価の一覧の equivalent_stale と equivalent_invalid、未記載の面の一覧の surface_unspecified_invalid と surface_unspecified_stale、全体像の元データの overview_lead_missing、overview_document_missing、overview_document_shared、overview_name_conflict、目次の overview_toc_invalid、overview_toc_page_missing、overview_toc_page_unknown、overview_toc_page_duplicate、overview_toc_group_empty、対の translation_missing、translation_record_invalid、translation_stale が、その話題で扱う種類である。

### 確認すること

- C-core-forder-3 文書の7行目にある問題への指摘の "line" は 7 になる
- C-core-forder-4 文書やファイルの全体にかかる指摘は、"line" が null になる（例: 目次のファイルへの overview_toc_invalid、題名の無い決定の文書への missing_title）

## 行を持つ指摘の line は、種類ごとに1か所の表で決める

状態: 実装済み

### 問題

行を持つ指摘がどの行を指すかを決める必要があった。line が null になる種類を除いて、1か所の表にまとめる。表にまとめた理由は残っていない。

### 決めたこと

行を持つ指摘の "line" は、次の表のとおりにする。

| 種類 | line |
|---|---|
| confirmation_without_test | テストが指していない確認することの行 |
| duplicate_id | 2つ目以降の場所の行（[同じ ID が何か所にもあれば、2つ目以降の場所ごとに duplicate_id を出す](findings.ja.md#同じ-id-が何か所にもあれば2つ目以降の場所ごとに-duplicate_id-を出す)） |
| unresolved_reference、invalid_marker | テストのマーカーのある行（[形の誤ったマーカーには invalid_marker の誤りを出し、マーカーの行を指す](test-markers.ja.md#形の誤ったマーカーには-invalid_marker-の誤りを出しマーカーの行を指す)） |
| test_without_id | テストの節の最初の行 |
| unclosed_code_block | 開始の行（[閉じないコードブロックは unclosed_code_block の誤りにし、開始から文書の終わりまでを読まない](ir-input.ja.md#閉じないコードブロックは-unclosed_code_block-の誤りにし開始から文書の終わりまでを読まない)） |

表に無い種類の行は、その種類を決めた話題で決める。mutant_survived と mutant_timeout は変異テストの話題、ガイドのマーカーの invalid_marker と guide_stale はガイドの話題、surface_without_spec は面の検査の話題、全体像の元データと目次の種類は全体像の話題、translation_structure_mismatch、translation_switcher_invalid、link_language_mismatch は対の話題が決める。

### 確認すること

- C-core-forder-5 実装済みの決定の確認することの行にテストが無いと、confirmation_without_test の "line" は、その確認することの行になる
- C-core-forder-6 ID の無いテストへの test_without_id の "line" は、テストの節の最初の行になる（Python のデコレータの付いた関数は "def" の行、PHP の "#[Test]" の付いたメソッドは "#[Test]" の行）

## 曖昧語の指摘 vague_word の line は、その語のある行にする

状態: 実装済み

### 問題

曖昧語の指摘も、行を持つ指摘の1つとして、どの行を指すかを決めておく。理由は残っていない。

### 決めたこと

決定の文書の曖昧語の指摘 vague_word の "line" は、その曖昧語のある行にする。

### 確認すること

- C-core-forder-7 決定の「決めたこと」の1つの行にだけ曖昧語があると、vague_word の "line" はその曖昧語のある行の番号になる
