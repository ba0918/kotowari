# languages の対応表

元: `docs/ir/view/languages.md`。書き直した文書: `languages.md`。

## ID の対応

| 元の ID | 新しい ID | 理由 |
|---|---|---|
| REQ-view-026 | C-view-languages-1 | lang の属性を言語タグにする要求 |
| REQ-view-027 | C-view-languages-3 | 中に文字を持たない禁止。UI の文字以外から取らないことを確かめる行 |
| REQ-view-028 | C-view-languages-4 | "{n}" を数に置き換える要求 |
| REQ-view-029 | C-view-languages-5, C-view-languages-6, C-view-languages-7, C-view-languages-8 | リンク・スクリプト無し・空のとき・リンクの文字の4つに分けた。テストが EX も指していれば EX の行を使う |
| REQ-view-030 | C-view-languages-9 | 本文の無い参照の要求 |
| TBL-view-002 | C-view-languages-3 | 表の鍵をすべて使って描く場面（EX-view-019）で確かめていた。表は1つ目の決定の「決めたこと」の表に移した（鍵の名前は落とした） |
| EX-view-018 | C-view-languages-1, C-view-languages-2 | lang の属性と、古い節のマーカーの値の2つを確かめる場面 |
| EX-view-019 | C-view-languages-3 | UI の文字以外の文字が無いことを確かめる場面 |
| EX-view-020 | C-view-languages-4 | 数の並びの場面 |
| EX-view-021 | C-view-languages-5, C-view-languages-6, C-view-languages-8 | リンクの先、script の要素が無いこと、リンクの文字の3つを確かめる場面 |
| EX-view-022 | C-view-languages-7 | ほかの言語が空の場面 |
| EX-view-023 | C-view-languages-9 | 本文の無い参照の場面 |

テストのマーカーを付け替えるときの補足:

- マーカーが REQ と EX の両方を指しているテスト（`tests/languages.rs` の6つ）は、EX の行の新しい ID だけにする。REQ-view-029 の行は4つを並べているので、REQ の行と合わせると確かめていない行まで指すことになる。
- `tests/rendering.rs` の `req_view_007_footnote_syntax_is_not_drawn_as_footnotes`（REQ-view-007, REQ-view-027）は、REQ-view-027 の分を C-view-languages-3 にする。REQ-view-007 の分は rendering の話題の対応表に従う。
- `tests/languages.rs` の `a_reference_missing_from_the_table_is_its_key_as_text_and_opens_nothing`（REQ-view-008, REQ-view-030）は、REQ-view-030 の分を付け替えずに外す。このテストが確かめるのは参照の表に無い参照（元の REQ-view-008 の振る舞い）で、C-view-languages-9 の「本文の無い1件」ではない。REQ-view-008 の分は rendering の話題の対応表に従う。
- コードのコメント（`src/lib.rs`、`src/parts.rs`、`src/text.rs`、`tests/common/ui.rs`、`tests/languages.rs` の TBL-view-002 の行）も古い ID を書いている。マーカーではないので機械の付け替えの対象外で、手で直す。

## 落とした細部

| どの要求のどの部分か | 誰が決めたと記録にあるか |
|---|---|
| TBL-view-002 の鍵の名前（language_name、index_link、pages、stale_sections、open_items、planned_items、stale_mark、outline_stale、superseded、deferred、compare_before、compare_after、compare_why、state_decided、state_planned、state_open、state_dropped） | brainstorm を進める LLM（localization D1「UI の文字の鍵の名前」。利用者が承認したのは任せることだけ） |
| REQ-view-028 の「10進で」 | 記録に無い。D1 の範囲の書き方の細部と読んだ |
| REQ-view-029 の「ほかの言語の並びの順に」 | brainstorm を進める LLM（D1「view の描画の入力に足す欄の形」） |
| REQ-view-029 の「リンクの先は、その置き場への相対パスに同じページの名前を続けたもの」の形（相対パスを入力に持つこと） | brainstorm を進める LLM（D1「view の描画の入力に足す欄の形」）。リンク先が同じページであることは A20 の決定なので残した |
| EX-view-023 の参照の状態 "current"、参照の文字列 "docs/x.md#A1" | 場面の入力の値。テストの中に置く |
| EX-view-019 の「すべての値を "X" に鍵の名前を続けたもの」「数を取る鍵は末尾に " {n}"」 | 場面の入力の値。確認することの行は「目印の付いた文字」と書き、厳密な値はテストに置く |

## 誰が決めたか見分けられなかったもの

- REQ-view-028 の「10進で」。記録のどの行にも無い。落とした。

## 理由や見送った案が記録に無かった決定

- 理由はすべての決定で記録にあった（A5、A9、A8、A20、A32、A34 の why）。
- 見送った案が記録に無かった決定: 「ページの lang の属性と、view が書く文字は、描画の入力の言語に従う」（A5）、「view は UI の文字を中に持たず、描画の入力から受け取る」（A9）、「数の入る文字は、"{n}" の所に数を入れる」（A8）、「どのページにも、ほかの言語の同じページへのリンクを置き、スクリプトは使わない」（A20）、「ほかの言語へのリンクの文字は、その言語自身で書いた言語の名前にする」（A32）。どれも見送った案の節を置いていない。見送った案があるのは A34 の決定だけ。

## 分けた所、まとめた所

- REQ-view-026 と REQ-view-027 は、記録の決定に合わせて分け直した。A5（lang と、view が書く文字が UI の言語で変わること、元の文字は訳さないこと）を1つ目の決定、A9（view は文字を中に持たず入力から受け取る、既定は kotowari の側だけ）を2つ目の決定にした。REQ-view-027 の禁止の文は2つ目の決定に置いた。
- REQ-view-029 は、記録の A20（同じページへのリンク、覚えない、スクリプト無し）と A32（リンクの文字はその言語自身の名前）の2つの決定に分けた。
- TBL-view-002 は、表の ID をやめ、1つ目の決定の「決めたこと」の表にした。鍵の名前の列を落とし、「view が書く文字」を言葉で書いた。

## あとでリンクにする所

| 書き直した文書の所 | 指したい相手 |
|---|---|
| 1つ目の決定の「一覧のページ、文書のページ、部品の話題の決定で決める」 | navigation の話題（文書の状態の数、目次の群の数、目次の中の位置、節のアウトライン）、rendering の話題（参照を開いて見せる、古い節のマーカー）、parts の話題（compare の部品、status の部品の札） |
| 3つ目の決定の「kotowari の設定の話題で決める」 | core の設定の話題の、"{n}" をちょうど1つ含むかを確かめる決定（localization A8 の設定の誤りの部分） |
| 4つ目の決定の問題の「どのページにもスクリプトを入れないという一覧のページの決定（目次の群を畳む決定）」 | navigation の話題の、目次の群を畳む決定（元の REQ-view-018） |
| 4つ目の決定の「kotowari の全体像の話題で決める」 | core の全体像の build の話題の、ほかの言語のページを "<言語タグ>/" の下に書く決定（元の REQ-core-293 にかかる A20） |
| 5つ目の決定の、名前を UI の文字の鍵から取ること | core の UI の文字（labels）の決定 |
| 6つ目の決定の「kotowari の全体像の話題で決める」 | core の全体像の話題の、先頭の言語以外のページで判断の記録の参照に本文を渡さない決定（A34、A38） |
| 6つ目の決定の問題の「判断の記録は訳さない」 | 対にする文書の決定（A11）。ただし判断の記録を持たなくなるので、書き直し先で残るかを確かめてから |

## 迷った判断と、どう決めたか

- A5 と A9 を1つにまとめるか分けるか。REQ-view-027 は2つを束ねていたが、記録では別の決定で、理由も違う（A5 は「kotowari の持ち物は view が書く文字だけ」、A9 は「既定を2か所に持つと食い違う」）。1つの見出しに1つの決定の決まりに従って分けた。
- 決定表を残すか。鍵の名前は D1 で LLM に任された細部なので落とした。一方、view が書く文字の種類（A5 が括弧の中で挙げたもの、A3 の status の札、A32 の言語の名前）と、数が入るかどうか（A8）は利用者の決定から来るので、言葉の表として残した。一覧のページへのリンクとアウトラインの古い節のマーカーは A5 の括弧の中に無いが、A5 の「view が自分で書く文字のすべて」に入るので表に含めた。
- A5 の「CLI の誤りの文は英語のまま」と、A8 の「"{n}" が無いか2つ以上なら設定の誤り」は、view ではなく kotowari の側の振る舞いなので、この文書の決定にはしなかった。A8 の後者は言葉で触れ、あとでリンクにする所に挙げた。
- A20 の「ほかの言語のページを "<言語タグ>/" の下に書く」は kotowari の側の振る舞いだが、リンクの先が同じ構成であることの前提なので、決めたことに1文で書き、kotowari の話題で決めると添えた。
- REQ-view-029 の「ほかの言語が空なら何も描かない」は記録に文として無いが、A20 の「どのページにもほかの言語の同じページへのリンクを置く」からそのまま出る振る舞いなので、決めたことの文と確認することの行に残した。
- 問題の節の「日本語に固定されていて」は経緯だが、なぜ決めたかを伝えるのに要るので残した（決定の文書の決まりで、問題には要る経緯だけを書いてよい）。

## 数

- 元の要求 5、場面 6、決定表 1、性質 0
- 読んだ記録のファイル数 1（`docs/decision/records/2026-10-05-localization.md`。ほかに用語集 `docs/ir/view/CONTEXT.ja.md`）
- 出来上がりの決定 6、確認することの行 9（うち「（読んで確認）」0）
