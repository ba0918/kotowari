# rendering の対応表

元: `docs/ir/view/rendering.md`。書き直した文書: `rendering.md`。

| 元の ID | 新しい ID | 理由 |
|---|---|---|
| REQ-view-001 | C-view-rendering-1, C-view-rendering-2 | 描画の入力の中身を C-1、kotowari を知らないことを C-2 にした。`crates/kotowari-markdown-view/tests/rendering.rs` の `req_view_001_the_input_carries_documents_sections_the_reference_table_and_the_contents` は C-1。`rendering.rs` の `req_view_006_preamble_parts_follow_the_lead_in_order_before_the_sections` と `crates/kotowari-overview/tests/overview.rs` の `parts_after_the_lead_and_before_the_first_section_reach_the_page_in_order` は、REQ-view-006 と一緒に指しているので C-10 だけを指せばよい |
| REQ-view-002 | C-view-rendering-5, C-view-rendering-6 | 返すページの名前を C-5、スタイルの参照とページどうしのリンクを C-6 にした。`rendering.rs` の `req_view_002_no_documents_still_give_the_index_and_the_style` は C-5、`req_view_002_pages_refer_to_the_shared_style_and_to_each_other_relatively` は C-6。相対パスであることは落とした細部で、テストの中に残る |
| REQ-view-003 | C-view-rendering-3, C-view-rendering-4 | 検査しないことを C-3、ファイルとネットワークを使わないことを C-4（読んで確認）にした。`crates/kotowari-markdown-view/tests/parts.rs` の `req_view_003_a_value_that_does_not_fit_is_drawn_without_reporting` は C-3。EX-view-006 と一緒に指している `parts.rs` のテストは C-16。`req_view_003_the_characters_of_field_values_are_drawn_as_text_not_markup` は値の文字の扱いを確かめていて、一緒に指している REQ-view-011（部品の話題）の新しい ID を指せばよい |
| REQ-view-004 | C-view-rendering-7 | そのまま |
| REQ-view-005 | C-view-rendering-8, C-view-rendering-9 | 並びと入れ子を C-8、文書の項目を C-9 にした。`rendering.rs` の `req_view_005_the_note_of_the_contents_is_drawn_under_the_index_heading` と `req_view_005_the_note_of_a_nested_group_is_part_of_the_group_heading` は C-8 を指す。説明（note）を描くことは落とした細部で、テストの中に残る |
| REQ-view-006 | C-view-rendering-10 | そのまま |
| REQ-view-007 | C-view-rendering-11, C-view-rendering-12, C-view-rendering-17, C-view-rendering-18 | Markdown を HTML にすることを C-11、参照リンクの解き方を C-12、生の HTML を文字で出すことを C-17、コメントを出さないことを C-18 にした。`rendering.rs` の `req_view_007_markdown_text_follows_commonmark_and_gfm_tables` は C-11、EX-view-024 のテストは C-12。EX-view-003 のテストは C-17 と C-18、`req_view_007_text_next_to_a_comment_is_not_drawn_as_code`、`req_view_007_a_comment_spanning_lines_inside_a_paragraph_keeps_one_paragraph`、`req_view_007_a_comment_line_keeps_the_blocks_around_it_apart` は C-18 を指す。`req_view_007_footnote_syntax_is_not_drawn_as_footnotes` も C-11 |
| REQ-view-008 | C-view-rendering-13, C-view-rendering-14 | 本文を開くこととページの外へリンクしないことを C-13、状態のマーカーを C-14 にした。`rendering.rs` の `req_view_008_deferred_references_are_marked_and_no_reference_links_out` は C-13 と C-14。`crates/kotowari-markdown-view/tests/languages.rs` の `a_reference_missing_from_the_table_is_its_key_as_text_and_opens_nothing` は C-13 |
| REQ-view-009 | C-view-rendering-15 | そのまま |
| REQ-view-010 | C-view-rendering-16, C-view-rendering-19, C-view-rendering-20 | フォントを C-16、スタイルと画像を C-19、文章の塊から外を読み込ませないことを C-20 にした。スクリプトは「一覧と移動」の、どのページにもスクリプトを入れない決定に属する。`parts.rs` の EX-view-006 のテストは C-16 と C-19、`req_view_010_markdown_text_cannot_make_the_page_load_images_scripts_or_styles` は C-20 を指す |
| EX-view-001 | C-view-rendering-5 | そのまま |
| EX-view-002 | C-view-rendering-7 | そのまま |
| EX-view-003 | C-view-rendering-17, C-view-rendering-18 | 一度は落とした細部にしたが、決定に戻した（下の「落とした細部」）。テストは C-17 と C-18 を指す |
| EX-view-004 | C-view-rendering-13, C-view-rendering-14 | 本文と外へのリンクの無さが C-13、置き換え済みのマーカーが C-14 |
| EX-view-005 | C-view-rendering-15 | そのまま |
| EX-view-006 | C-view-rendering-16 | 外の参照が無いことを確かめるので、外のフォントが無いことも確かめる |
| EX-view-010 | C-view-rendering-8, C-view-rendering-9 | 見出し・並び・入れ子が C-8、文書の題名・結論・リンクが C-9 |
| EX-view-024 | C-view-rendering-12 | そのまま |

元の文書に TBL と PROP は無い。

## 落とした細部

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-view-007: 生の HTML を解釈せず文字として出し、HTML のコメントは出さない | LLM（2026-10-02-whole-picture.md の A70。「承認の時に利用者に示す」）。後に決定に戻した（C-view-rendering-17、C-view-rendering-18） |
| REQ-view-007: Markdown を CommonMark と GFM の表として読む（決めたことには、利用者が決めた「見出し、文章、リスト、表、コード」を書いた） | LLM（2026-10-02-whole-picture.md の A82。照合レビューの指摘から） |
| REQ-view-010: フォントに限らず、スクリプト、スタイル、画像もページの外から読み込まない | LLM（A82）。利用者が決めたのはフォントだけ（A37）。言語の切り替えにスクリプトを使わないことは利用者の決定（2026-10-05-localization.md の A20）で、「言語」の話題に属する。後にスタイルと画像を決定に戻した（C-view-rendering-19、C-view-rendering-20） |
| REQ-view-002: ページどうしと "style.css" への参照が相対パスであること | LLM（A82） |
| REQ-view-001: 文書、節、参照の表の項目、目次の群の欄の正確な形（名前、題名、本文を省いてよいこと、など） | LLM（A82）と、細部を LLM に任せた承認（2026-10-05-localization.md の D1） |
| REQ-view-001、REQ-view-008: 状態の値の綴り "current"、"superseded"、"deferred" | LLM（2026-10-02-whole-picture.md の A81） |
| REQ-view-008、REQ-view-009: UI の文字の鍵の名前（"superseded"、"deferred"、"stale_mark"） | 細部を LLM に任せた承認（localization の D1） |
| REQ-view-005: 目次のいちばん外側の目次の群の説明（note）を、一覧の見出しの下に描く | LLM（2026-10-05-overview-index.md の A26。レビューの指摘を推奨で埋めた） |
| REQ-view-004: 時刻、乱数、環境変数をページに入れない | 記録に無い。同じ入力から同じバイトを返すことの言い換えなので、決めたことには書かずテストに任せる |
| REQ-view-008: 参照の表に無い参照の描き方（キーを文字のまま描き、何も開かない） | 元の文書の外（languages.rs のテストのコメント）。本文の無い参照は「言語」の話題 |

## 誰が決めたか見分けられなかったもの

- REQ-view-005: 入れ子の目次の群の説明（note）を、群の見出しとして描くこと。目次の群が説明を持つこと（overview-index の A10）は利用者の決定だが、それを描くと決めた行は見つからなかった。overview-index の A26（LLM）が「入れ子の群と同じく」と前提にしているだけである。決めたことには書かず、テストに任せた。

## 理由や見送った案が記録に無かった決定

- 「Markdown の文章の塊は、view が HTML にする」: 見送った案が記録に無い。節を置かなかった。
- 「置き換え済みと後回しの参照に、マーカーを付ける」: 見送った案が記録に無い。
- 「古い節の見出しの近くに、古い節のマーカーを描く」: 見送った案が記録に無い。
- 「ページは、外からフォントを読み込まない」: 理由が記録に無い（A37 の why は依存の選び方の理由で、フォントには触れていない）。問題の節に「理由は残っていない」と書いた。
- 「Markdown の参照リンクは、同じブロックの中の定義だけで解く」: 見送った案の「ブロックをまたぐ参照リンクを誤りにする」に、採らなかった理由が記録に無い。
- 「同じ描画の入力からは、同じページを返す」: 決めた文そのものは記録に無い。A10（決まった描画で作る）と A29（中身が違うファイルだけを書き直す）を問題に書いた。見送った案は A10 の rejected だけを書いた。

見送った案のうち、この話題の決定の代わりではないものは書かなかった: A29 の rejected（依存関係の記録。ページを書き出す側の決定）、overview-index の A7 の rejected（目次のファイルの置き方。core の話題）、A37 の rejected（簡易サーバの自作。core の話題）、A16 の rejected（HTML のコミット。core の話題）。

## 分けた所、まとめた所

- REQ-view-007 を2つの決定に分けた: 「Markdown の文章の塊は、view が HTML にする」と「Markdown の参照リンクは、同じブロックの中の定義だけで解く」。参照リンクの決定は別の記録（2026-10-06-todo-zero.md の A2）で、見送った案を持つため。
- REQ-view-008 を2つの決定に分けた: 「参照はページの外へリンクせず、選ぶとその場で本文を開く」（A65）と「置き換え済みと後回しの参照に、マーカーを付ける」（A31、A67）。
- REQ-view-003 と REQ-view-001 の導入の段落（view は kotowari の仕様も判断の記録も知らない）は、それぞれの決定の中に入れた。REQ-view-003 は独立の決定「view は入力を検査せず、ファイルもネットワークも使わない」にした。
- 値違いの場面（EX-view-001 と、文書が無いときのテスト）は C-5 の1行にまとめた。

## あとでリンクにする所

- 「部品」の話題（`docs/ir/view/parts.md`）: 冒頭の段落、「view は入力を検査せず、ファイルもネットワークも使わない」のスキーマの決定
- 「一覧と移動」の話題（`docs/ir/view/navigation.md`）: 冒頭の段落、「一覧、文書ごとのページ、共通のスタイルを、別々のページとして返す」の目次と食い違う入力（元の REQ-view-021）、「文書のページは、題名、アウトライン、lead、冒頭の部品、節の順に描く」のアウトライン（元の REQ-view-022）
- 「言語」の話題（`docs/ir/view/languages.md`）: 冒頭の段落、「参照はページの外へリンクせず、選ぶとその場で本文を開く」の本文の無い参照（元の REQ-view-030）

## 迷った判断と、どう決めたか

- 生の HTML を文字で出すこと（A70）と、スクリプト・スタイル・画像を外から読み込まないこと（A82）は、どちらも安全に関わるが、記録では LLM が決めた細部で、利用者には承認のときにまとめて示しただけである。決まりどおり落とし、テストの中に残した。後に利用者が決定に戻した（C-view-rendering-17〜20）。
- 参照の状態の3つ（A67）と、節ごとに古さを計算して渡すこと（A68）は、LLM が導いた細部だが、記録に「利用者が第21ラウンドで確認した」とある。まとめての承認ではなく、ラウンドの中での確認なので、利用者が決めたこととして残した。綴りは落とした。
- 目次を描画の入力に足したこと（overview-index の A33）は LLM の決定だが、view が渡された目次のとおりに描くこと（A18）は利用者の決定なので、目次が入力にあることは書いた。
- 冒頭の lead の結論を一覧に描くことは、A42（利用者。view が一覧のための題と冒頭の結論を受け取る）と A7（利用者）を根拠に残した。
- 「文書の名前」が元データのファイル名から ".md" を除いたものであること（A82、LLM）は書かず、"<文書の名前>.html" とだけ書いた。ページの名前を元データと同じ名前にすることは A56（利用者）である。
- C-16 は決めたことに合わせてフォントの範囲の文にした。EX-view-006 のテストは外の参照すべてが無いことを確かめるので、この行を確かめるのに足りる。
- 状態を変える前提で、テストで確かめる行にはどれも、それを確かめる既存のテストがあることを、テストの名前で確かめた（上の表）。C-2 と C-4 は既存のテストが無いので「（読んで確認）」にした。

## 数

- 元: 要求 10、場面 8、決定表 0、性質 0
- 読んだ記録のファイル: 6（2026-10-02-whole-picture.md、2026-10-04-overview-on-public-api.md、2026-10-05-overview-index.md、2026-10-05-localization.md、2026-10-05-overview-page-reading.md、2026-10-06-todo-zero.md）
- 出来上がり: 決定 12、確認することの行 16（うち「（読んで確認）」2）
