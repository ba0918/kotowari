# overview-toc の対応表

元: `docs/ir/core/overview-toc.md`（日本語は `overview-toc.ja.md`）。書き直した文書: `overview-toc.md`。短い名前は `ovtoc`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-325 | C-core-ovtoc-1, C-core-ovtoc-2, C-core-ovtoc-3, C-core-ovtoc-16 | 一覧の束ね方と並びは、overview.toc の指す目次のファイル1つに書く | そのまま。overview-index の A7、A10、A11、A17（利用者）。読む4つのコマンドは A16（check と status の指摘）と A12（build と serve を止める）から残した。UTF-8 でないときの停止の理由を "non-UTF-8 file" にすること（A32、LLM）は、安全に関わる細部として C-16 に戻した |
| REQ-core-326 | C-core-ovtoc-4, C-core-ovtoc-5, C-core-ovtoc-6, C-core-ovtoc-15 | 目次のファイルが元データ・ガイド・テストの置き場にも入れば、設定の誤りで停止する | そのまま。A17、A23（利用者）。各走査で実際に読むファイルで判定すること（A28、LLM）は、安全に関わる細部として C-15 に戻した。目次を読む前に判定することは、確かめるテストが無いので戻さず「テストの無い行」に挙げた。詳細を基準のディレクトリからの相対パスにすることは落とした |
| REQ-core-327 | C-core-ovtoc-7, C-core-ovtoc-8 | 目次は、いちばん外側を1つの目次の群とする入れ子で書く | そのまま。A4、A10、A15、A16、A22（利用者）。detail の細部は A34（LLM）で落とした |
| REQ-core-328 | C-core-ovtoc-9, C-core-ovtoc-11 | 全体像のページは、目次にちょうど1回ずつ置く | そのまま。A12、A16（利用者）。先頭の言語の側だけで照らすことは `translation-pairs.md` の C-core-pairs-13 にあるのでリンクにした |
| REQ-core-329 | C-core-ovtoc-9 | 全体像のページは、目次にちょうど1回ずつ置く | そのまま。A5、A12、A16（利用者）。重なりを元データのある名前だけで判定し、元データの無い名前には出てくるたびに overview_toc_page_unknown だけを出すこと（A29、LLM）と、2つ目以降を深さ優先で決めること（A34、LLM）は落とした |
| REQ-core-330 | C-core-ovtoc-10 | 全体像のページは、目次にちょうど1回ずつ置く | そのまま。A12、A16（利用者）。いちばん外側の detail を "(root)" にすることは A34（LLM）で落とした |
| REQ-core-331 | C-core-ovtoc-12 | 形の誤った目次は、元データと照らさない | そのまま。A21（利用者） |
| REQ-core-332 | C-core-ovtoc-13, C-core-ovtoc-14 | 目次は、書いたままの順と入れ子で描画のエンジンに渡す | そのまま。A6、A7、A15（利用者） |
| TBL-core-043 | C-core-ovtoc-7, C-core-ovtoc-8 | 目次は、いちばん外側を1つの目次の群とする入れ子で書く | そのまま。形の表は A10、A15、A22 と、"note" を一行の説明とした A10（利用者）。改行を含む "note" を形の誤りとする明記（A27、LLM）は A10 の「一行の説明」から導けるので、形の表に残した。detail の書き方の細部（鍵の名前を "/" で足す、"(root)"、"(yaml)"、同じ detail を1件にまとめる）は A34（LLM）で落とした |
| EX-core-505 | C-core-ovtoc-2 | 一覧の束ね方と並びは、overview.toc の指す目次のファイル1つに書く | そのまま。"overview.toc" の鍵が無いときの設定の誤りの部分は、設定の話題の C-core-config-18 に当たる |
| EX-core-506 | C-core-ovtoc-4 | 目次のファイルが元データ・ガイド・テストの置き場にも入れば、設定の誤りで停止する | そのまま |
| EX-core-507 | C-core-ovtoc-7, C-core-ovtoc-12 | 形の誤った目次は、元データと照らさない | そのまま。detail の値 "/items/0/color" は落とした細部で、テストの中に残る |
| EX-core-508 | C-core-ovtoc-9, C-core-ovtoc-11 | 全体像のページは、目次にちょうど1回ずつ置く | そのまま |
| EX-core-509 | C-core-ovtoc-10 | 全体像のページは、目次にちょうど1回ずつ置く | そのまま |
| EX-core-510 | C-core-ovtoc-14 | 目次は、書いたままの順と入れ子で描画のエンジンに渡す | そのまま |

元の文書に PROP は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-overview/tests/overview.rs | ex_core_507_an_unknown_key_in_a_group_is_a_form_error_and_stops_the_comparison | EX-core-507, REQ-core-327, REQ-core-331, TBL-core-043, REQ-core-027 | C-core-ovtoc-7, C-core-ovtoc-12 | この話題の分だけ。知らないキーが overview_toc_invalid になり、overview_toc_page_missing が出ないことを確かめる。REQ-core-027 の分は指摘の並びの話題（`finding-order.map.md`） |
| crates/kotowari-overview/tests/overview.rs | tbl_core_043_each_form_violation_names_its_place | REQ-core-327, TBL-core-043, REQ-core-027 | C-core-ovtoc-7 | この話題の分だけ。形の違反のそれぞれが overview_toc_invalid になることを確かめる。detail の値は落とした細部で、テストの中に残る |
| crates/kotowari-overview/tests/overview.rs | tbl_core_043_a_well_formed_toc_with_a_note_and_nested_groups_has_no_finding | REQ-core-327, TBL-core-043 | C-core-ovtoc-8 | |
| crates/kotowari-overview/tests/overview.rs | ex_core_508_missing_unknown_and_repeated_names_are_errors | EX-core-508, REQ-core-328, REQ-core-329, REQ-core-027 | C-core-ovtoc-9 | この話題の分だけ。REQ-core-027 の分は指摘の並びの話題 |
| crates/kotowari-overview/tests/overview.rs | req_core_329_an_unknown_name_repeated_is_only_unknown_each_time | REQ-core-329 | C-core-ovtoc-9 | 落とした細部のテスト。いちばん近い行を指す。元データの無い名前には出てくるたびに overview_toc_page_unknown だけを出すこと（A29、LLM）は、テストの中に残る |
| crates/kotowari-overview/tests/overview.rs | req_core_329_the_second_place_is_found_depth_first_in_written_order | REQ-core-329 | C-core-ovtoc-9 | 落とした細部のテスト。いちばん近い行を指す。2つ目以降を深さ優先で決めること（A34、LLM）は、テストの中に残る |
| crates/kotowari-overview/tests/overview.rs | ex_core_509_a_group_without_items_is_an_error | EX-core-509, REQ-core-330, REQ-core-027 | C-core-ovtoc-10 | この話題の分だけ。REQ-core-027 の分は指摘の並びの話題 |
| crates/kotowari-overview/tests/overview.rs | req_core_330_an_empty_outermost_group_is_reported_at_the_root | REQ-core-330, REQ-core-328 | C-core-ovtoc-10 | いちばん外側の空の群に overview_toc_group_empty が出ることを確かめる。detail の "(root)" は落とした細部 |
| crates/kotowari-overview/tests/overview.rs | req_core_332_the_render_input_carries_the_toc_as_written | REQ-core-332 | C-core-ovtoc-13 | |
| crates/kotowari-overview/tests/overview.rs | req_core_294_toc_errors_count_as_errors_and_block_the_pages | REQ-core-294, REQ-core-328 | C-core-ovtoc-11 | この話題の分だけ。目次の誤りでページが返らないことを確かめる。REQ-core-294 の分は全体像の書き出しの話題 |
| crates/kotowari-overview/src/toc.rs | the_embedded_toc_schema_compiles | TBL-core-043 | C-core-ovtoc-7 | 落とした細部のテスト。いちばん近い行を指す。同梱の目次の形のスキーマが読み込めるかを確かめる |
| tests/step18_overview.rs | req_core_325_check_reads_the_toc_and_reports_its_errors_on_the_toc_file | REQ-core-325, REQ-core-328, REQ-core-329, REQ-core-288 | C-core-ovtoc-1 | この話題の分だけ。check が目次を読み、目次のファイルに指摘を出すことを確かめる。目次が元データの数に入らないこと（REQ-core-288）は全体像の元データの話題の分 |
| tests/step18_overview.rs | req_core_325_status_stops_when_the_toc_file_is_missing | REQ-core-325 | C-core-ovtoc-2 | |
| tests/step18_overview.rs | req_core_325_a_toc_that_is_not_utf8_stops | REQ-core-325, TBL-core-001 | C-core-ovtoc-3, C-core-ovtoc-16 | この話題の分だけ。停止と、理由 "non-UTF-8 file" を確かめる。TBL-core-001 の分は CLI の話題 |
| tests/step18_overview.rs | ex_core_506_a_toc_matched_by_the_overview_glob_is_a_config_error | EX-core-506, REQ-core-326, TBL-core-001 | C-core-ovtoc-4 | この話題の分だけ。TBL-core-001 の分は CLI の話題 |
| tests/step18_overview.rs | req_core_326_the_toc_is_compared_with_the_files_each_walk_reads | REQ-core-326 | C-core-ovtoc-5, C-core-ovtoc-15 | 後半の "guides.files" と "tests.files" の詳細が C-5、前半の、元データの glob に当たる YAML の目次を重なりにしないことが C-15 に当たる |
| tests/step18_overview.rs | req_core_326_the_overview_data_overlap_is_judged_before_the_toc_overlap | REQ-core-326, REQ-core-280 | C-core-ovtoc-6 | この話題の分だけ。REQ-core-280 の分は全体像の元データの話題 |
| tests/step19_overview_build.rs | ex_core_505_build_stops_without_the_toc_key_or_the_toc_file | EX-core-505, REQ-core-013, REQ-core-325, TBL-core-001 | C-core-ovtoc-2 | この話題の分だけ。目次のファイルが無いと build が止まる部分。"overview.toc" の鍵が無いときの設定の誤り（REQ-core-013）は設定の話題の C-core-config-18 に当たる |
| tests/step19_overview_build.rs | ex_core_508_build_on_toc_errors_writes_nothing | EX-core-508, REQ-core-294, TBL-core-001 | C-core-ovtoc-11 | この話題の分だけ。REQ-core-294 の分は全体像の書き出しの話題 |
| tests/step19_overview_build.rs | ex_core_510_the_index_follows_the_toc_order | EX-core-510, REQ-core-332 | C-core-ovtoc-14 | |
| tests/step20_overview_serve.rs | req_core_294_serve_on_toc_errors_stops_before_binding | REQ-core-294, REQ-core-328 | C-core-ovtoc-11 | この話題の分だけ。REQ-core-294 の分は全体像の書き出しの話題 |

## 落とした細部

### 安全に関わるもの

無い。書き直しの決まり（安全に関わる落とした細部は決定に戻す）で、ここにあった2件を決定に戻した（下の「戻した細部」）。1件の後半（目次を読む前に判定する）は確かめるテストが無く、戻していない。

### 戻した細部

| 細部 | 新しい ID | 確かめているもの |
|---|---|---|
| REQ-core-326: 重なりは glob の文字列でなく、各置き場の走査で実際に読むファイル（全体像の元データでは拡張子が小文字の ".md" のもの）に目次のファイルが入るかで決める | C-core-ovtoc-15（決定「目次のファイルが元データ・ガイド・テストの置き場にも入れば、…」） | テスト req_core_326_the_toc_is_compared_with_the_files_each_walk_reads の前半 |
| REQ-core-325: 目次のファイルが UTF-8 でないときの停止の理由を、UTF-8 でないファイル（"non-UTF-8 file"）にする | C-core-ovtoc-16（決定「一覧の束ね方と並びは、…」） | テスト req_core_325_a_toc_that_is_not_utf8_stops |

### 戻せなかった細部

| 細部 | 戻すために要るテスト |
|---|---|
| REQ-core-326: 重なりの判定を目次を読む前に行う | 下の「テストの無い行」 |

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-326: 設定の誤りの詳細のパスを、基準のディレクトリからの相対パスにする | LLM（A28） |
| REQ-core-329: overview_toc_page_duplicate は元データのある名前だけで判定し、元データの無い名前には出てくるたびに overview_toc_page_unknown だけを出す | LLM（A29） |
| REQ-core-329: 「2つ目以降」は、目次を書かれた順に深さ優先でたどった順で決める | LLM（A34） |
| REQ-core-330: いちばん外側の群の overview_toc_group_empty の detail を "(root)" にする | LLM（A34） |
| TBL-core-043: overview_toc_invalid の detail の書き方（知らない鍵と欠けた鍵は鍵の名前を "/" で足す、値の全体は "(root)"、YAML として読めないときは "(yaml)"、同じ detail は1件にまとめる） | LLM（A34） |

## 形に依存する要求

無い。目次は全体像の元データのファイル名を並べるもので、仕様の形（IR、判断の記録）に触れない。

### テストの無い行

実装済みの行は、どれも上の表のテストが1本以上指す。戻せなかった細部:

| 細部 | 要るテスト | 行の文の案 |
|---|---|---|
| 重なりの判定を目次を読む前に行う（REQ-core-326、A28） | "overview.files" に当たる位置に、YAML として読めない目次のファイル（".md" の名前）を置き、check の停止が overview_toc_invalid の誤りでなく ": matched by both overview.toc and overview.files" の設定の誤りになることを確かめる | 目次のファイルが置き場と重なり、中身が YAML として読めないとき、check は目次の形の誤りを出さずに重なりの設定の誤りで停止する |

## 誰が決めたか見分けられなかったもの

- 無い。2026-10-05-overview-index.md の A1〜A25、A37 と D1 は decided_by が利用者、A26〜A36 は LLM。2026-10-02-whole-picture.md の A58 は利用者。

## 理由や見送った案が記録に無かった決定

- 「目次のファイルが元データ・ガイド・テストの置き場にも入れば、設定の誤りで停止する」: 見送った案が記録に無い。戻した判定の仕方の理由は A28 の why（元データとして読まれないファイルを誤りにしない）を書いた。
- 「一覧の束ね方と並びは、…」: 戻した停止の理由は A32 の why（元データの読み方にそろえる）を書いた。
- 「形の誤った目次は、元データと照らさない」: 見送った案が記録に無い。

## 分けた所、まとめた所

- REQ-core-327 と TBL-core-043 を1つの決定にした（形の表そのものが決定）。
- REQ-core-328、REQ-core-329、REQ-core-330 を「全体像のページは、目次にちょうど1回ずつ置く」の1つの決定にまとめた。どれも A12 の1つの決定で、理由も同じ。
- REQ-core-331 は理由が別（A21）なので、独立の決定にした。
- build と serve が目次の誤りで止まること（A12）は、全体像の書き出しの話題の REQ-core-294 にもある。目次の食い違いの決定の中で、目次の誤りに限って書き、行（C-core-ovtoc-11）を置いた。

## あとでリンクにする所

- view の rendering と navigation は、冒頭の段落で見出しと文書へのリンクにした
- 全体像の元データの話題（overview-data）と全体像のコマンドの話題（overview-commands）を指す所は、見出しへのリンクにした（元データと同じく止まること、元データの重なり、overview_part_invalid の detail の書き方、build と serve が止まること）

## 迷った判断と、どう決めたか

- 目次の指摘の detail は、すでに書き直された `findings.md` の表にあるので、この文書では種類と、overview_toc_invalid の detail が JSON Pointer であること（A16、利用者）だけを書き、表へリンクした。`finding-order.md` が「line を null にする種類は、それぞれの指摘を決めた話題で決める」としているので、line が null であることはこの文書に書いた。
- 目次を読むコマンドの列挙（check、status、build、serve）は A32（LLM）の明記だが、A16（check と status の指摘に入れる）と A12（build と serve を止める）が4つとも決めているので、決定に残した。
- 目次の重なりの判定を「glob に当たる」（A17 の言葉）と書くと、今のコードの振る舞い（A28 のとおり、各置き場が実際に読むファイルで判定する）と食い違う。決めたことは「置き場にも入る」と書き、判定の仕方（A28）は 書き直しの決まり（安全に関わる落とした細部は決定に戻す）で安全に関わる細部として戻した（C-15）。
- C-16 の停止の理由は、利用者の A17 の「読めないファイルを理由に止める」と、LLM の A32 の「UTF-8 でないファイル」で食い違う。今のコードとテストは A32 のとおりなので、A32 で書いた。
- records.md の A20（第2ラウンド、終了コード）と A104（第9ラウンド、"non-UTF-8 file" を含む停止の理由の固定の文言）は、書き直しの決まりのとおり、推奨をまとめて採った利用者の決定に数えた。前の下書きと同じ読み方で、直す所は無かった。
- 記録の why のうち、外部のプロジェクトの名前を挙げた所（A3 の製品の名前）は、中身（ページの数が非常に多くなりうる）だけを問題に書いた。

## 実装する側が決める細部

- 無い。

## 利用者に聞くこと

- 無い。

## 「提案」で書いた決定の一覧

- 無い。どの決定も、今のコードがそのまま新しい版でも同じに動く。

## 数

- 元: 要求 8、場面 6、決定表 1、性質 0
- 読んだ記録のファイル: 3（2026-10-05-overview-index.md、2026-10-02-whole-picture.md、records.md の A20 と A104）
- 出来上がり: 決定 6（実装済み 6、提案 0）、確認することの行 16（うち「（読んで確認）」0）
- 戻した安全の細部: 2件（C-core-ovtoc-15、C-core-ovtoc-16）。戻せなかった細部: 1件（目次を読む前に判定すること）
- テスト: 21（どれも付け替え先がある）
