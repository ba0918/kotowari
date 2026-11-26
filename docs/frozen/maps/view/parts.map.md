# parts の対応表

元の文書: `docs/ir/view/parts.md`。書き直した文書: `parts.md`。

| 元の ID | 新しい ID | 理由 |
|---|---|---|
| REQ-view-011 | C-view-parts-1, C-view-parts-2 | 部品の種類の一覧と、その描き方。status の札は C-view-parts-2 に分けた |
| REQ-view-012 | C-view-parts-3 | 種類ごとのスキーマを返す公開の関数 |
| REQ-view-013 | C-view-parts-4 | スキーマの object がすべて閉じていることをテストで確かめる |
| REQ-view-014 | C-view-parts-5 | スキーマに合う例を view が描けることをテストで確かめる |
| REQ-view-015 | C-view-parts-6, C-view-parts-7 | "width" に許す値（C-view-parts-6）と、左右に並べる描き方（C-view-parts-7） |
| REQ-view-024 | C-view-parts-8 | flow の列の囲みと矢印。人がブラウザで見て確かめる行 |
| REQ-view-025 | C-view-parts-9 | 狭い画面の flow。人がブラウザで見て確かめる行 |
| TBL-view-001 | C-view-parts-1, C-view-parts-2 | 決定表は「部品の種類は、view が持つ8種の固定の一覧にする」の「決めたこと」の表になった |
| EX-view-007 | C-view-parts-3 | 8種の名前でスキーマが返る場面。知らない名前に何も返さない部分は細部として落とした（下の一覧）が、テストはこの行を指してよい |
| EX-view-008 | C-view-parts-7 | 続けて置いた half の部品2つが左右に並ぶ場面 |
| EX-view-009 | C-view-parts-4 | 閉じていない object があるとテストが落ちる場面 |

## 落とした細部

| 元の要求・表のどの部分か | 記録 | 誰が決めたと記録にあるか |
|---|---|---|
| REQ-view-012: スキーマを JSON Schema の「文字列」で返すこと、表に無い名前には何も返さないこと | whole-picture A82 | LLM（照合レビューの指摘から導いた細部） |
| REQ-view-015: 続けて置いた half の部品を先頭から2つずつ組にし、組にならずに残った1つを幅いっぱいに描くこと | whole-picture A73 | LLM（敵対的レビューの指摘から導いた細部） |
| TBL-view-001: 「Main fields」の列（lead の conclusion と points、flow の columns と箱の title・body・tone、steps の items、cards の cards、status の items、compare の items、decisions の roots、quiz の items） | whole-picture A83 | LLM（照合レビューの指摘から導いた細部） |
| TBL-view-001: flow の箱を格子に並べ、文字の位置を手で決めないこと | whole-picture A83 | LLM |
| TBL-view-001: compare の before を取り消し線で描くこと | whole-picture A83 | LLM |
| TBL-view-001: decisions を根の判断とその下の判断の木にし、判断した者の札を付けること | whole-picture A83 | LLM |
| TBL-view-001: steps を番号付きの段階に、quiz を選ぶと開く答えにすること | whole-picture A85 | LLM（「モックで確かめた描き方」と書いてあるが、決定の行は LLM が導いた細部） |
| TBL-view-001: status の札と compare の見出しに使う UI の文字の鍵の名前（"state_decided"、"state_planned"、"state_open"、"state_dropped"、"compare_before"、"compare_after"、"compare_why"） | localization D1 | 利用者（推奨を採用）だが、中身は「鍵の名前は LLM が決める」という進め方の承認だけ |
| REQ-view-024 / REQ-view-025: 列の囲みの描き方、矢印の形、広い画面と狭い画面の境目の幅 | overview-page-reading D2 | 利用者が実装に任せると決めた。元の文書にも値は無く、新しい文書にも「実装に任せる」とだけ書いた |

## 誰が決めたか見分けられなかったもの

- 無し。REQ-view-015 の2つの文（幅の指定の無い部品は幅いっぱい、"width" に許すのは "half" だけ）は、利用者の決定の言い換えと判断して残した（下の「迷った判断」）。

## 理由や見送った案が記録に無かった決定

- 部品の種類は、view が持つ8種の固定の一覧にする: 見送った案が記録に無い（A13、A36、localization A3、public-api A13 のどれにも rejected が無い）。節を置かなかった。
- レイアウトの指定は、部品の幅を半分にする指定だけにする: 見送った案が記録に無い（A12、A54）。節を置かなかった。
- flow の1つの列の箱は並列を表し、順番は列から列への流れだけで表す: 見送った案が記録に無い（page-reading A8）。節を置かなかった。
- 狭い画面では、flow の列を上から下に積む: 見送った案が記録に無い（page-reading A9）。節を置かなかった。
- 部品の種類は、view が持つ8種の固定の一覧にする: 種類をその場で作らせず固定の一覧にする理由が、whole-picture A13 の why に「基本は推奨でよく、必要になったら検討する」としか無い。問題の節にそのとおり書いた。ほかの理由（Markdown では貧弱で読む負担が大きい）は whole-picture の Revisions にある利用者の答えから、モックの判断は A11 から取った。

部品のスキーマの決定の見送った案のうち2つ（部品の中身の検査を view に持たせる、view は型を公開し kotowari の側が型への読み込みで検査する）は、決定の行の rejected ではなく、whole-picture の Revisions の欄にある利用者の発言の記録から取った。

## 分けた所、まとめた所

- REQ-view-011 と TBL-view-001 を1つの決定（部品の種類は、view が持つ8種の固定の一覧にする）にまとめた。status の札の行は、利用者が別の壁打ち（public-api A13、localization A3）で決めたので、確認することの行を C-view-parts-2 に分けた。
- REQ-view-012、REQ-view-013、REQ-view-014 を1つの決定（部品の形の正は、view が埋め込んで公開する部品のスキーマにする）にまとめた。3つとも A45〜A47 の「形の正を1つにしてテストで揃える」という1つの決定から出ている。
- REQ-view-015 は1つの決定のまま、確認することを2行に分けた。
- REQ-view-024 と REQ-view-025 は、問題（並列か順番か読めない／狭い幅で箱が重なる）と理由が別なので、2つの決定のままにした。
- whole-picture A13 の「描画が知らない種類は誤りにする」は、この話題に入れなかった。誤りを出すのは view ではなく kotowari の側の検査なので、全体像の元データを検査する話題に置くべきもので、そちらを書き直すときに拾う。
- whole-picture A46 の、検査に使うクレートの選び方（既定の機能を切って使う）は、この話題に入れなかった。検査をするのは view ではなく kotowari の側なので、全体像の元データを検査する話題に置くべきもので、そちらを書き直すときに拾う。外部のクレートの名前は書かない決まりなので、残すなら「手入れされている JSON Schema の検査のクレートを、既定の機能を切って使う」のような書き方になる。
- compare の表の見出しを UI の文字から取ること（元の TBL-view-001 の compare の行）は、この文書の決めたことから外した。view が書く文字をすべて UI の文字から取る決まりは言語の話題（元の REQ-view-027）にあり、それを確かめるテストも言語の側（`crates/kotowari-markdown-view/tests/languages.rs`）にしかない。この文書に確認することの行を置くと、実装済みなのに指すテストが無い行になる。status の札は、この話題のテスト（`tbl_view_001_status_draws_a_label_for_each_of_the_four_states`）が UI の文字の札を確かめているので、残した。

## あとでリンクにする所

- 「部品の形の正は…」の決めたこと: 「検査は kotowari の側が、view から受け取ったスキーマで行う（全体像の元データの話題の決定）」→ 全体像の元データを検査する話題（今の `docs/ir/core/overview-data.md` に当たるもの）の、部品の中身を検査する決定へ。
- 「部品の形の正は…」の問題: 「view は描画だけを扱うと決めてあり」→ view が描画のドメインだけを扱うと決めた決定（whole-picture A22。書き直したあとに置かれる話題の決定）へ。
- 「部品の種類は…」の表の lead の行「文書の冒頭に置く」→ 描画の話題の、冒頭の結論の決定（元の REQ-view-006）へ。
- 「部品の種類は…」の決めたこと「UI の文字から取った、その言語の文字で描く」→ 言語の話題の、view が文字を中に持たない決定（元の REQ-view-027）へ。

## 迷った判断と、どう決めたか

- A83 と A85 の描き方の細部（取り消し線、判断した者の札、番号付き、選ぶと開く答え）を落とすかどうか。A85 には「モックで確かめた描き方（A11）」とあり、モックは利用者が見て良いと言ったものだが、決定の行の decided_by は LLM で、承認のときに示しただけになっている。決まりどおり落とし、表の「描くもの」は A36（利用者）の言い方（結論と要約、流れ図、段階、カード、状態の一覧、前と後、判断のつながり、問い）だけにした。flow の「左から右へ流れる」は page-reading A8（利用者）にあるので残した。
- C-view-parts-1 の粒度。種類ごとのテスト（`tbl_view_001_*` の8本）が1行を指す形にした。落とした細部（取り消し線など）はテストの中に残り、文書の行は「表の『描くもの』のとおりに描かれる」までにした。値違いの場面を1行にまとめる決まりに当たると判断した。
- ブラウザで人が見て確かめる行（元の verification: review、how_to_verify）を「（読んで確認）」で書いた。形の決まりには「人や LLM が読んで確かめる行」としか無く、描いたページを見て確かめる行の書き方が無い。テストを求めない行という意味では同じなので、「（読んで確認）」を使い、行の中に「ブラウザで開くと」と確かめ方を書いた。
- 機械で付け替えると、1つの元の ID が2行に分かれた所と、元のマーカーの組み合わせのせいで、確かめていない行を指すテストが出る（ファイルはどれも `crates/kotowari-markdown-view/tests/parts.rs`）。
  - REQ-view-011 と TBL-view-001 はどちらも C-view-parts-1 と C-view-parts-2 に当たるので、`tbl_view_001_*` の10本すべてが両方を指す。C-view-parts-2 を本当に確かめるのは `tbl_view_001_status_draws_a_label_for_each_of_the_four_states` と `tbl_view_001_the_four_status_badges_are_drawn_distinguishably` の2本で、ほかの8本は C-view-parts-1 だけを指すべき。
  - REQ-view-015 は C-view-parts-6 と C-view-parts-7 に当たるので、REQ-view-015 を指す3本が両方を指す。`req_view_015_every_schema_accepts_only_half_as_the_width` は C-view-parts-6 だけ、`ex_view_008_two_following_half_parts_sit_side_by_side` と `req_view_015_a_half_part_left_over_and_others_are_drawn_full_width` は C-view-parts-7 だけを指すべき。
  - `ex_view_007_schemas_exist_for_the_eight_kinds_and_not_for_others` は REQ-view-011 も指しているので、C-view-parts-1 と C-view-parts-2 も指すことになる。C-view-parts-3 だけを指すべき。
  - `req_view_014_every_kind_has_examples_that_fit_their_schema_and_render` は EX-view-009 も指しているので、C-view-parts-4 も指すことになる。C-view-parts-5 だけを指すべき。
  - `req_view_003_the_characters_of_field_values_are_drawn_as_text_not_markup` は REQ-view-011 も指しているので、C-view-parts-1 と C-view-parts-2 も指すことになる。このテストは描画の話題の行を確かめるもので、この話題の ID は外すべき。
  - どの行も、ほかのテストが正しく指しているので、check の数え方は崩れない。付け替えのあとで、余分な ID を手で外す。
- REQ-view-015 の2つの文（幅の指定の無い部品は節の幅いっぱいに描く、部品のスキーマが "width" に許すのは "half" だけ）を残すかどうか。whole-picture A12（利用者）は「最初は部品の幅を半分にする指定だけにする」「隣り合う半分の部品を左右に並べる」で、A54（利用者）は "width: half" で幅を指定すると決めている。指定の無い部品の幅と、スキーマで "half" 以外を拒むことは記録の文には無いが、「半分の指定だけ」の言い換えと読めるので、利用者の決定として残した。元の EX-view-008 の出典も A12 と A54 だけである。落とすと `req_view_015_every_schema_accepts_only_half_as_the_width` が、確かめる行の無いテストになる。
- 部品のスキーマを「埋め込む」こと。whole-picture A49（利用者）の一部で、A49 はのちに2か所（検査の結果をまとめる先、読み書きと serve の置き場）を置き換えられたが、view がスキーマを埋め込んで公開する部分は置き換えられていないので、残した。

## 数

- 元の文書: 要求 7（REQ-view-011〜015、REQ-view-024、REQ-view-025）、場面 3（EX-view-007〜009）、決定表 1（TBL-view-001）、性質 0
- 読んだ記録のファイル: 4（`2026-10-02-whole-picture.md`、`2026-10-05-overview-page-reading.md`、`2026-10-04-overview-on-public-api.md`、`2026-10-05-localization.md`）。置き換えの有無を確かめるために、記録の置き場全体を ID で検索した
- 出来上がり: 決定 5、確認することの行 9（うち「（読んで確認）」2）
