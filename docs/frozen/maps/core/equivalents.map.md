# equivalents の対応表

元: `docs/ir/core/equivalents.md`（日本語は `equivalents.ja.md`）。書き直した文書: `equivalents.md`。確認することの ID の短い名前は `equiv`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-141 | C-core-equiv-5, C-core-equiv-6, C-core-equiv-7, C-core-equiv-8, C-core-equiv-9, C-core-equiv-10 | 一覧の1件は、ファイル、変更の説明、行の文面で変異を同定する | 一致したときの振る舞いを C-5、文面で取ることを C-6、ファイルで取ることを C-7、パスの書き方と空白と行の終わりを C-8、同じ文面の行が複数あるときを C-9、ファイルが無いか行が超えるときを C-10 にした |
| REQ-core-142 | C-core-equiv-11, C-core-equiv-12, C-core-equiv-13 | 文面がソースから無くなった1件には、equivalent_stale の注意を出す | 指摘の形を C-11、読めないファイルを C-12、結果に現れるかを見ないことを C-13 にした |
| REQ-core-143 | C-core-equiv-14, C-core-equiv-15, C-core-equiv-16, C-core-equiv-17 | 形の誤った1件は equivalent_invalid の誤りにし、どの見逃しも外さない | 条件と指摘の形を C-14、外さないことと stale を出さないことを C-15、detail を C-16、重複を C-17 にした |
| REQ-core-148 | C-core-equiv-1, C-core-equiv-2, C-core-equiv-3, C-core-equiv-4 | 等価の判断は、設定の mutants.equivalents が指す YAML の一覧に置く | 一覧のファイルの状態ごとの振る舞いを「決めたこと」の表にした。0件で続けるのを C-1、無いときを C-2、並びでないときを C-3、check を C-4 にした。check が鍵の値を検査すること（REQ-core-014）は「設定」の話題 |
| EX-core-211 | C-core-equiv-5 | 一覧の1件は、ファイル、変更の説明、行の文面で変異を同定する | そのまま |
| EX-core-212 | C-core-equiv-6 | 一覧の1件は、ファイル、変更の説明、行の文面で変異を同定する | 行が動いた側 |
| EX-core-213 | C-core-equiv-6, C-core-equiv-11 | 一覧の1件は、ファイル、変更の説明、行の文面で変異を同定する | 書き換えた側が C-6、古くなった1件の注意が C-11 |
| EX-core-214 | C-core-equiv-10 | 一覧の1件は、ファイル、変更の説明、行の文面で変異を同定する | そのまま |
| EX-core-215 | C-core-equiv-14, C-core-equiv-15, C-core-equiv-16 | 形の誤った1件は equivalent_invalid の誤りにし、どの見逃しも外さない | そのまま |
| EX-core-216 | C-core-equiv-14 | 形の誤った1件は equivalent_invalid の誤りにし、どの見逃しも外さない | そのまま |
| EX-core-217 | C-core-equiv-2 | 等価の判断は、設定の mutants.equivalents が指す YAML の一覧に置く | そのまま |
| EX-core-220 | C-core-equiv-16 | 形の誤った1件は equivalent_invalid の誤りにし、どの見逃しも外さない | そのまま |
| EX-core-221 | C-core-equiv-4 | 等価の判断は、設定の mutants.equivalents が指す YAML の一覧に置く | そのまま |
| EX-core-232 | C-core-equiv-9 | 一覧の1件は、ファイル、変更の説明、行の文面で変異を同定する | そのまま |
| EX-core-233 | C-core-equiv-7 | 一覧の1件は、ファイル、変更の説明、行の文面で変異を同定する | そのまま |
| EX-core-234 | C-core-equiv-8 | 一覧の1件は、ファイル、変更の説明、行の文面で変異を同定する | そのまま |
| EX-core-235 | C-core-equiv-10, C-core-equiv-12 | 一覧の1件は、ファイル、変更の説明、行の文面で変異を同定する | 見逃しの側が C-10、一覧の側が C-12 |
| EX-core-236 | C-core-equiv-14 | 形の誤った1件は equivalent_invalid の誤りにし、どの見逃しも外さない | そのまま |
| EX-core-237 | C-core-equiv-17 | 形の誤った1件は equivalent_invalid の誤りにし、どの見逃しも外さない | そのまま |
| EX-core-238 | C-core-equiv-1 | 等価の判断は、設定の mutants.equivalents が指す YAML の一覧に置く | 値違いの場面（0バイト、注釈だけ、鍵が無い）を C-1 の1行にまとめた |
| EX-core-239 | C-core-equiv-3 | 等価の判断は、設定の mutants.equivalents が指す YAML の一覧に置く | そのまま |
| EX-core-243 | C-core-equiv-11 | 文面がソースから無くなった1件には、equivalent_stale の注意を出す | 書かれたままのパスが detail に出ることを、C-11 の行の括弧の中に書いた |

元の文書に TBL と PROP は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| tests/step9_mutants.rs | req_143_blank_why_is_invalid_and_suppresses_nothing | REQ-core-143, EX-core-215 | C-core-equiv-14, C-core-equiv-15, C-core-equiv-16 | equivalent_invalid の severity、path、line、detail と、mutant_survived が残ること、equivalent_stale が出ないこと |
| tests/step9_mutants.rs | req_143_class_other_than_equivalent_is_invalid | REQ-core-143, EX-core-216 | C-core-equiv-14 | "class" が "untested" |
| tests/step9_mutants.rs | req_143_entry_outside_the_base_is_invalid | REQ-core-143, EX-core-236 | C-core-equiv-14 | "file" が "../x/src/a.rs" |
| tests/step9_mutants.rs | req_143_entry_without_file_has_an_empty_detail_prefix | REQ-core-143, EX-core-220 | C-core-equiv-16 | detail が ": replace f with ()" |
| tests/step9_mutants.rs | req_143_duplicate_invalid_entries_yield_one_finding_each | REQ-core-143, EX-core-237 | C-core-equiv-17 | 誤りが2件 |
| tests/step9_mutants.rs | req_143_entry_with_a_key_outside_the_five_is_invalid | REQ-core-143 | C-core-equiv-14, C-core-equiv-15 | 知らない鍵 "note" で誤りが出て、mutant_survived が残る |
| tests/step9_mutants.rs | req_148_missing_list_file_stops | REQ-core-148, EX-core-217 | C-core-equiv-2 | 終了コード2、"unreadable file: docs/equivalents.yaml" で始まる |
| tests/step9_mutants.rs | req_148_empty_list_file_is_zero_entries | REQ-core-148, EX-core-238 | C-core-equiv-1 | 0バイトの一覧で終了コード0 |
| tests/step9_mutants.rs | req_148_list_of_comments_only_is_zero_entries | REQ-core-148 | C-core-equiv-1 | 注釈だけの一覧 |
| tests/step9_mutants.rs | req_148_blank_lines_between_comments_are_still_empty | REQ-core-148 | C-core-equiv-1 | 注釈の間に空行がある一覧 |
| tests/step9_mutants.rs | req_148_list_that_is_not_a_sequence_stops_with_the_list_path | REQ-core-148, EX-core-239 | C-core-equiv-3 | "config error: docs/equivalents.yaml" で始まる |
| tests/step9_mutants.rs | req_148_check_ignores_a_missing_list_file | REQ-core-148, EX-core-221 | C-core-equiv-4 | check が終了コード2でない。準備で "ir"、"decisions.records"、"decisions.adr" の置き場を作っているので、切り替えのときに新しい形の置き場へ書き換える（確かめていることは形に依存しない） |
| tests/step9_mutants.rs | req_141_listed_survivor_is_counted_as_equivalent | REQ-core-139, REQ-core-141, REQ-core-145, EX-core-211 | C-core-equiv-5 | mutant_survived が出ず、survived 0、equivalent 1、終了コード0。REQ-core-139 と REQ-core-145 の分は mutants の対応表に書いた |
| tests/step9_mutants.rs | req_141_moved_line_still_matches | REQ-core-141, EX-core-212 | C-core-equiv-6 | 行が7行目に動いても一致 |
| tests/step9_mutants.rs | req_141_rewritten_line_no_longer_matches_and_entry_goes_stale | REQ-core-141, REQ-core-142, EX-core-213 | C-core-equiv-6, C-core-equiv-11 | 書き換えた行で mutant_survived と、equivalent_stale の severity、path、line、detail |
| tests/step9_mutants.rs | req_141_line_beyond_the_file_does_not_match | REQ-core-141, EX-core-214 | C-core-equiv-10 | 9行目の見逃しが line 9 の mutant_survived になり、終了コード1 |
| tests/step9_mutants.rs | req_141_one_entry_matches_every_line_with_the_same_text | REQ-core-141, EX-core-232 | C-core-equiv-9 | equivalent が2 |
| tests/step9_mutants.rs | req_141_entry_for_another_file_does_not_match | REQ-core-141, EX-core-233 | C-core-equiv-7 | "src/b.rs" の1件は "src/a.rs" の見逃しに一致しない |
| tests/step9_mutants.rs | req_141_path_spelling_tab_indent_and_crlf_still_match | REQ-core-141, EX-core-234 | C-core-equiv-8 | "./src/a.rs"、タブ、"\r\n" |
| tests/step9_mutants.rs | req_141_lone_cr_ends_a_line_of_the_source | REQ-core-141, TBL-core-010 | C-core-equiv-8 | 単独の "\r" で区切ったソースでも一致する。単独の "\r" を行の終わりに数える規則は「IR の文書」の話題の行の数え方（TBL-core-010）の分 |
| tests/step9_mutants.rs | req_111_source_with_a_bom_still_matches_on_its_first_line | REQ-core-111, REQ-core-141 | C-core-equiv-5 | 先頭に BOM のあるソースの1行目でも一致し、equivalent が1。BOM を読み飛ばす規則は「IR の入力」の話題（REQ-core-111）の分 |
| tests/step9_mutants.rs | req_141_non_utf8_source_does_not_stop | REQ-core-141, REQ-core-142, EX-core-235 | C-core-equiv-10, C-core-equiv-12 | 終了コード1で、mutant_survived と equivalent_stale が1件ずつ |
| tests/step9_mutants.rs | req_142_stale_entry_detail_keeps_the_written_path | REQ-core-142, EX-core-243 | C-core-equiv-11 | detail が "./src/a.rs: replace f with ()" |

## マーカーに足すテスト

この話題の ID を今は指していないが、付け替えたあとの確認することの行を確かめているテスト。今のマーカー（別の話題の ID）は残したまま、この話題の ID を足す。同じテストがほかの話題の対応表にも載っているときは、それぞれの表の ID を合わせて足す。

| テストのファイル | テストの関数名 | 今のマーカーの ID | 足す ID | 理由 |
|---|---|---|---|---|
| tests/step9_mutants.rs | req_140_timeout_is_a_notice_even_when_listed | REQ-core-140, EX-core-206 | C-core-equiv-13 | 今のマーカーはこの話題の ID を指していないが、文面がソースにある1件が、時間切れの変異にしか合わず見逃しに一致しないときに、equivalent_stale が出ないことを確かめている。付け替えるときに足す |
| tests/step9_mutants.rs | req_139_survived_mutant_is_an_error | REQ-core-138, REQ-core-139, TBL-core-024, EX-core-204 | C-core-equiv-1 | 今のマーカーはこの話題の ID を指していないが、設定に "mutants.equivalents" の鍵が無いときに一覧を0件として続けることを確かめている。付け替えるときに足す |

## 落とした細部

### 安全に関わるもの

無い。基準のディレクトリの外を指す1件を誤りにすること（A51）、形の誤った1件に見逃しを外させないこと（A34、A44）、一致しない側に倒すこと（A33、A48）は利用者の決定なので、決めたことに残した。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-141: 行は TBL-core-010 のとおりに区切る（単独の "\r" も行の終わり） | 利用者（2026-09-17-mutation-tests.md の A52「IR の行の数え方と同じ」）。数え方の中身は「IR の文書」の話題の決まりなので、「kotowari がほかのファイルの行を数えるときと同じ」とだけ書き、"\r\n" の "\r" を含めないことだけを残した |
| REQ-core-148: "kotowari check" は鍵の値を REQ-core-014 のとおりに検査する | 利用者（A45）。検査の中身は「設定」の話題なので、「鍵の値の型を見るだけ」と書いた |
| 「等価」の観測の範囲を、kotowari 自身を変異テストするときに標準出力・標準エラー・終了コードで読むこと | 利用者（2026-09-19-mutants-followup.md の A1、A13。2026-09-24-guide-gaps.md の A8 で用語集の文は利用者から観測できる振る舞い一般に書き直された）。語の意味は用語集の側にあるので、冒頭の段落で用語集の今の文を使い、kotowari 自身の読み方は書かなかった |

## 形に依存する要求

無い。テスト req_148_check_ignores_a_missing_list_file の準備だけが古い形の置き場を使う（テストごとの表）。

## テストの無い行

無い。C-core-equiv-1 の「鍵が無い」場面と C-core-equiv-13 は、今のマーカーがこの話題を指していないテストで確かめる（テストごとの表の最後の2行）。

## 誰が決めたか見分けられなかったもの

無い。読んだ決定はすべて decided_by が利用者（推奨を採用）。

## 理由や見送った案が記録に無かった決定

- 「形の誤った1件は equivalent_invalid の誤りにし…」: 見送った案が記録に無い。節を置かなかった。
- 「等価の判断は…」の、指す先が無いときに停止する理由（A34 の why「設定の置き場が無いときと同じ扱い」）は、置き場の決まりがほかの話題にあるので書かなかった。
- 「文面がソースから無くなった1件には…」: 見送った案は「以前の決定」（A9 の、結果に現れない行に注意を出す部分）だけで、検討して退けた別案は記録に無い。

## 分けた所、まとめた所

- 置き場の決定（REQ-core-148）に、一覧を kotowari の側に持つこと（A9）、置き場と形（A16、A36）を入れた。A9 と A16 の見送った案はここに置いた。
- 分類が "equivalent" だけであること（A17）は、形の誤りの決定に入れた。3つの分類（A5）は調べるときの語彙で手順の側の決まりなので、問題の節に理由としてだけ書いた。
- 一致した見逃しを survived に数えず equivalent に数えることは、mutants の集計の決定（表）と、この話題の同定の決定の両方に書いた。確かめる行はこちらの C-5 に置いた。
- 時間切れが一覧で外れないこと（A37）は mutants の時間切れの決定に置き、ここでは「一覧で外せるのは見逃しだけ」とだけ書いた。

## あとでリンクにする所

確定の段で、次を見出しへのリンクにした。下書きどうしのリンクも `<file>.ja.md#<anchor>` に直した。

- base-directory の「設定ファイルの値のパスは、比べる前と出力の前に正規化する」（冒頭の段落の正規化。元の REQ-core-110）
- config の「設定ファイルが YAML として読めないか、キーと値が決まりに合わなければ、設定の誤りで停止する」（冒頭の段落の、設定の鍵の値の検査。元の REQ-core-014）
- ir-document の「kotowari は、"\n"、"\r\n"、単独の "\r" を、それぞれ1つの行の終わりに数える」（「一覧の1件は…」の行の区切り。元の TBL-core-010）

残り: 設定の鍵の一覧（元の TBL-core-004）と、先頭の BOM の読み飛ばし（ir-input。元の REQ-core-111）。この文書の文では指していないので、リンクにする所は無い


## 迷った判断と、どう決めたか

- C-core-equiv-13（結果に現れるかを見ない）を直接確かめるテストは無い。req_140_timeout_is_a_notice_even_when_listed は、一覧の1件が見逃しに一致しない状態で equivalent_stale が出ないことを確かめているので、これを付け替え先にした。「結果のファイルにその変異がまったく無い」場面のテストを足すほうが確かである。
- 行の区切りは「IR の行の数え方と同じ」（A52）と決まっているが、IR は新しい形で無くなる。決めたことには「kotowari がほかのファイルの行を数えるときと同じ」と書いた。行の数え方の決定がどこに移るかで、リンク先を決め直す。
- 「人が読む判断の文書（Markdown）に混ぜる」の見送った案は、記録では「判断の記録に混ぜる」だった。判断の記録の形は無くなるが、退けた理由（機械的な一覧と人が読む文書の性質の違い）は今も成り立つので、形に依存しない言い方に変えて残した。
- 「why が空」の基準（A53 の why「判断の記録の補足の行と同じ」）は、古い形を指す理由なので書かなかった。基準そのもの（前後の半角空白とタブを除いて空）は残した。
- 行番号を外した名前が重なる実測（804件のうち382件）は記録の why にあるので、問題の節に書いた。
- 「のように」を直した: C-8 の「"./src/a.rs" のように書き方だけ違っても」を「"./src/a.rs" と書いた1件でも」にした（テストが確かめる書き方は "./src/a.rs" だけなので、例の範囲を言い切った）。

## 数

- 元: 要求 4、場面 18、決定表 0、性質 0
- 読んだ記録のファイル: 3（2026-09-17-mutation-tests.md、2026-09-19-mutants-followup.md、2026-09-24-guide-gaps.md の A8）
- 出来上がり: 決定 4、確認することの行 17（うち「（読んで確認）」0）
- テスト: 25（この話題の ID を指すもの 23、マーカーを足すもの 2）
