# schema-resolution の対応表

元: `docs/ir/schema/schema-resolution.md`（日本語は `schema-resolution.ja.md`）。書き直した文書: `schema-resolution.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-schema-011 | C-schema-resolve-1 | スキーマは文書の位置からの相対パスか URL で指定する | 定義の TBL-schema-003 を決めたことの表にした。停止の行は「frontmatter が壊れているか…」、URL の上限は「URL のスキーマを取得できないとき…」に分けた |
| REQ-schema-012 | C-schema-resolve-2, C-schema-resolve-13 | スキーマは文書の位置からの相対パスか URL で指定する | 基準のディレクトリを使わないことは決めたことに書き、テストは文書の位置から解くことを確かめる（C-2）。重ねた ".." を打ち消さないこと（安全に関わる細部）を C-13 に戻した |
| REQ-schema-013 | C-schema-resolve-5, C-schema-resolve-6, C-schema-resolve-7, C-schema-resolve-8 | URL のスキーマは取得した内容をキャッシュに置き、壊れていれば取得し直す | キャッシュと読み直しを C-5、壊れたキャッシュの回復を C-6、置き場を C-7 にした。キャッシュに書けないときは別の記録（todo-zero の A1）で理由を持つので、別の決定「キャッシュに書けなくても、取得した内容で検査を続ける」の C-8 にした |
| REQ-schema-014 | C-schema-resolve-3 | frontmatter が壊れているか、スキーマを指していない文書は、検査せずに停止する | そのまま |
| REQ-schema-015 | - | スキーマは文書の位置からの相対パスか URL で指定する | 出典の A1 は "$schema" 以外のキーを読まないことを述べていない。決めた行が記録に見つからないので書かなかった（「誰が決めたか見分けられなかったもの」）。当たる決定は、"$schema" を読む決定 |
| REQ-schema-052 | C-schema-resolve-12 | 停止の説明に URL を載せるときは、認証情報を伏せる | そのまま |
| TBL-schema-003 | C-schema-resolve-1, C-schema-resolve-4, C-schema-resolve-9, C-schema-resolve-10, C-schema-resolve-11 | スキーマは文書の位置からの相対パスか URL で指定する | 表は最初の決定に置いた。1の行の停止と3の行の「読めなければ停止」は C-3 と C-4、2の行の取得の失敗と上限は C-9〜C-11 |
| PROP-schema-003 | - | スキーマは文書の位置からの相対パスか URL で指定する | 出典の A14 は、コマンドによって解決先が変わらないことを述べていない。決めた行が見つからないので書かなかった。テスト url_schema_resolves_the_same_for_all_commands は、3つのコマンドが同じキャッシュを使うこと（C-5）を指す |
| EX-schema-005 | C-schema-resolve-2 | スキーマは文書の位置からの相対パスか URL で指定する | そのまま |
| EX-schema-017 | C-schema-resolve-12 | 停止の説明に URL を載せるときは、認証情報を伏せる | そのまま |
| EX-schema-006 | C-schema-resolve-6 | URL のスキーマは取得した内容をキャッシュに置き、壊れていれば取得し直す | そのまま |
| EX-schema-066 | C-schema-resolve-10 | URL のスキーマを取得できないときと、取得が上限を超えたときは停止する | 4MiB の側 |
| EX-schema-067 | C-schema-resolve-10 | URL のスキーマを取得できないときと、取得が上限を超えたときは停止する | 10秒の側。2つの値違いの場面を1行にまとめた |
| EX-schema-068 | C-schema-resolve-11 | URL のスキーマを取得できないときと、取得が上限を超えたときは停止する | テストは、ちょうど 4MiB の応答が使われることを見る |
| EX-schema-069 | C-schema-resolve-7 | URL のスキーマは取得した内容をキャッシュに置き、壊れていれば取得し直す | 基準のディレクトリがある側 |
| EX-schema-070 | C-schema-resolve-7 | URL のスキーマは取得した内容をキャッシュに置き、壊れていれば取得し直す | 基準のディレクトリが無い側。2つの場面を1行にまとめた |
| EX-schema-093 | C-schema-resolve-8 | キャッシュに書けなくても、取得した内容で検査を続ける | そのまま |

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-markdown-schema/src/frontmatter.rs | reads_relative_schema_ref_and_ignores_other_keys | REQ-schema-011, REQ-schema-015 | C-schema-resolve-1 | "http" で始まらない値を相対パスとして読むこと。ほかのキーを読まないこと（REQ-schema-015）は決定に書かなかったので、その分は付け替え先が無い |
| crates/kotowari-markdown-schema/src/frontmatter.rs | reads_url_schema_ref | REQ-schema-011 | C-schema-resolve-1 | "https://" で始まる値を URL として読む |
| crates/kotowari-markdown-schema/src/frontmatter.rs | document_without_frontmatter_has_no_schema | REQ-schema-014 | C-schema-lib-1 | 落とした細部のテスト。いちばん近い行を指す。frontmatter の無い文書で frontmatter_schema が「参照が無い」を返し、誤りにはならないこと（library の入口の表の frontmatter_schema の行）。停止するかはファイルかディレクトリかで変わり、それを決めるのは呼び出し側（C-schema-resolve-4 と cli のディレクトリの検査）で、このテストはどちらも確かめていない |
| crates/kotowari-markdown-schema/src/frontmatter.rs | frontmatter_without_schema_key_has_no_schema | REQ-schema-014 | C-schema-lib-1 | 落とした細部のテスト。いちばん近い行を指す。同上（"$schema" の無い frontmatter） |
| crates/kotowari-markdown-schema/src/frontmatter.rs | broken_frontmatter_is_an_error | REQ-schema-014, REQ-schema-049 | C-schema-resolve-3 | YAML として壊れた frontmatter が誤りになる。誤りが std::error::Error であることは REQ-schema-049（「ライブラリの入口」の話題）の分 |
| crates/kotowari-markdown-schema/src/frontmatter.rs | non_string_schema_is_an_error | REQ-schema-014 | C-schema-resolve-3 | そのまま |
| crates/kotowari-markdown-schema/src/frontmatter.rs | null_schema_is_an_error | REQ-schema-014 | C-schema-resolve-3 | そのまま |
| crates/kotowari-markdown-schema/src/frontmatter.rs | empty_string_schema_is_an_error | REQ-schema-014 | C-schema-resolve-3 | そのまま |
| crates/kotowari-markdown-schema/src/frontmatter.rs | whitespace_only_schema_is_an_error | REQ-schema-014 | C-schema-resolve-3 | そのまま |
| crates/kotowari-markdown-schema/src/frontmatter.rs | array_schema_is_an_error | REQ-schema-014 | C-schema-resolve-3 | 文字列でない値（配列） |
| crates/kotowari-markdown-schema/src/frontmatter.rs | relative_ref_resolves_against_document_location | REQ-schema-012, EX-schema-005 | C-schema-resolve-2 | そのまま |
| crates/kotowari-markdown-schema/src/frontmatter.rs | consecutive_parent_refs_are_not_collapsed | REQ-schema-012 | C-schema-resolve-2, C-schema-resolve-13 | "doc.md" の "../../wrong.yaml" が "../../wrong.yaml" のファイルに解かれ、重ねた ".." が打ち消されないこと。安全に関わる細部として戻した行（C-13） |
| crates/kotowari-markdown-schema/src/frontmatter.rs | url_ref_stays_a_url | REQ-schema-011 | C-schema-resolve-1 | URL は文書の位置で解かず、URL のまま残る |
| crates/kotowari-markdown-schema/src/frontmatter.rs | frontmatter_that_is_not_a_mapping_is_an_error | REQ-schema-014 | C-schema-resolve-3 | そのまま |
| crates/kotowari-markdown-schema/src/frontmatter.rs | frontmatter_that_is_a_sequence_is_an_error | REQ-schema-014 | C-schema-resolve-3 | マッピングでない frontmatter（並び） |
| crates/kotowari-markdown-schema/tests/library.rs | a_null_schema_reference_has_an_explanatory_public_error | REQ-schema-049, REQ-schema-014 | C-schema-resolve-3 | null の "$schema" が誤りになる。誤りの説明が "$schema" を含む公開の誤りであることは REQ-schema-049（「ライブラリの入口」の話題）の分 |
| crates/kotowari-mds/tests/cli.rs | ast_schema_without_schema_stops | REQ-schema-009, REQ-schema-014, REQ-schema-042 | C-schema-resolve-4 | ファイルを対象にした ast --schema が、"$schema" の無い文書で停止する。停止の理由の名前（REQ-schema-042）と停止そのもの（REQ-schema-009）は「CLI と結果の出し方」の話題の分 |
| crates/kotowari-mds/tests/cli.rs | check_missing_schema_stops_with_schema_not_found | REQ-schema-009, REQ-schema-011, REQ-schema-042 | C-schema-resolve-4 | check の側。REQ-schema-009 と REQ-schema-042 の分は同上 |
| crates/kotowari-mds/tests/cli.rs | check_unresolvable_schema_stops_with_schema_not_found | REQ-schema-009, REQ-schema-011, REQ-schema-042 | C-schema-resolve-4 | 相対パスのスキーマを読めないときの停止。REQ-schema-009 と REQ-schema-042 の分は同上 |
| crates/kotowari-mds/tests/cli.rs | check_broken_frontmatter_stops_with_frontmatter_invalid | REQ-schema-009, REQ-schema-014, EX-schema-004, REQ-schema-042 | C-schema-resolve-3 | check の側。EX-schema-004 と REQ-schema-009、REQ-schema-042 は「CLI と結果の出し方」の話題の分 |
| crates/kotowari-mds/tests/cli.rs | values_with_broken_frontmatter_stops_with_frontmatter_invalid | REQ-schema-009, REQ-schema-014 | C-schema-resolve-3 | values の側。REQ-schema-009 の分は同上 |
| crates/kotowari-mds/tests/cli.rs | ast_schema_with_broken_frontmatter_stops_with_frontmatter_invalid | REQ-schema-009, REQ-schema-014 | C-schema-resolve-3 | ast --schema の側。REQ-schema-009 の分は同上 |
| crates/kotowari-mds/tests/cli.rs | values_without_schema_stops_with_schema_not_found | REQ-schema-009, REQ-schema-014, REQ-schema-042 | C-schema-resolve-4 | values の側。REQ-schema-009 と REQ-schema-042 の分は同上 |
| crates/kotowari-mds/tests/url_schema.rs | url_schema_is_fetched_once_and_cached | REQ-schema-013 | C-schema-resolve-5 | そのまま |
| crates/kotowari-mds/tests/url_schema.rs | url_schema_resolves_the_same_for_all_commands | REQ-schema-011, REQ-schema-013 | C-schema-resolve-5 | check、values、ast --schema と続けて読んでも取得が1回だけで、2回目からキャッシュを読むこと。PROP-schema-003 は決定に書かなかったので、確かめていることのうちキャッシュの分だけをこの行に当てた |
| crates/kotowari-mds/tests/url_schema.rs | url_schema_without_cache_and_unreachable_server_stops | REQ-schema-011, REQ-schema-009 | C-schema-resolve-9 | REQ-schema-009 の分は「CLI と結果の出し方」の話題で付け替える |
| crates/kotowari-mds/tests/url_schema.rs | corrupted_cache_is_refetched_and_recovers | REQ-schema-013, EX-schema-006 | C-schema-resolve-6 | そのまま |
| crates/kotowari-mds/tests/url_schema.rs | schema_response_over_4mib_stops | REQ-schema-011, REQ-schema-009, EX-schema-066 | C-schema-resolve-10 | REQ-schema-009 の分は同上 |
| crates/kotowari-mds/tests/url_schema.rs | ex_schema_068_a_schema_of_exactly_4mib_is_used | EX-schema-068, TBL-schema-003 | C-schema-resolve-11 | そのまま |
| crates/kotowari-mds/tests/url_schema.rs | ex_schema_069_the_cache_goes_under_the_base_directory_above_the_current_one | EX-schema-069 | C-schema-resolve-7 | そのまま |
| crates/kotowari-mds/tests/url_schema.rs | ex_schema_070_without_a_base_directory_the_cache_goes_under_the_current_one | EX-schema-070 | C-schema-resolve-7 | そのまま |
| crates/kotowari-mds/tests/url_schema.rs | userinfo_in_a_schema_url_does_not_reach_the_error_output | REQ-schema-052, EX-schema-017 | C-schema-resolve-12 | そのまま |
| crates/kotowari-mds/tests/url_schema.rs | a_directory_check_fetches_a_shared_url_schema_once_even_when_the_cache_cannot_be_written | REQ-schema-013 | C-schema-resolve-8 | キャッシュのディレクトリを作れなくても検査が終了コード 0 で終わること。1回の check の間に同じ URL を1回だけ取得することは落とした細部で、テストの中に残る |
| crates/kotowari-mds/tests/url_schema.rs | ex_schema_093_a_cache_that_cannot_be_written_does_not_stop_the_check | EX-schema-093, REQ-schema-013 | C-schema-resolve-8 | ディレクトリを作れないときと、ファイルを書けないときの両方 |
| crates/kotowari-markdown-schema-io/src/lib.rs | ex_schema_067_a_fetch_past_the_time_limit_stops | EX-schema-067, TBL-schema-003 | C-schema-resolve-10 | 時間の上限を超えた取得が、スキーマが見つからない失敗になる（上限の値は注入して短くしている） |
| crates/kotowari-markdown-schema-io/src/lib.rs | the_cli_limits_a_schema_fetch_to_ten_seconds | TBL-schema-003 | C-schema-resolve-10 | 上限の値が10秒であること |
| crates/kotowari-markdown-schema-io/src/lib.rs | userinfo_in_a_url_is_hidden_before_it_reaches_a_message | REQ-schema-052 | C-schema-resolve-12 | そのまま |
| crates/kotowari-markdown-schema-io/src/lib.rs | a_url_without_userinfo_is_left_alone | REQ-schema-052 | C-schema-resolve-12 | 認証情報の無い URL はそのまま出る |
| crates/kotowari-markdown-schema-io/src/lib.rs | an_at_sign_in_the_path_is_not_mistaken_for_userinfo | REQ-schema-052 | C-schema-resolve-12 | パスの "@" は authority の認証情報ではないので伏せない |

## 落とした細部

### 安全に関わるもの

なし。前の版でここに置いた1件（重ねた ".." を打ち消さない）は、テストが確かめているので決定に戻した（下の「戻した細部」）。

認証情報を伏せること（A25）、取得の上限とタイムアウト（mutants-gaps の A7）、スキーマを解けないときの停止（A8、A43、P1）は、どれも利用者の決定なので落としていない。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-schema-015: frontmatter の "$schema" 以外のキーを読まず、指摘にもしない | 出典の A1（利用者）の文には無い。決めた行が見つからない。決める記録の行が無いので落としたまま（依頼の指示）。今の振る舞いはテスト reads_relative_schema_ref_and_ignores_other_keys に残る |
| PROP-schema-003: 同じ文書の "$schema" は、検査、抽出、素の構文木のどのコマンドから読んでも同じスキーマに解決する | 出典の A14（利用者）の文には無い。決めた行が見つからない。決める記録の行が無いので落としたまま（依頼の指示）。今の振る舞いのうち3つのコマンドが同じキャッシュを読むことは、テスト url_schema_resolves_the_same_for_all_commands が C-5 として確かめる |
| 1回の check の間に、同じ URL のスキーマを1回だけ取得する（テストの中だけ） | 記録に無い |
| 停止の理由の名前（schema_not_found、frontmatter_invalid） | この話題の元の文書には無い。「CLI と結果の出し方」の話題（TBL-schema-009）の名前 |
| 取得の上限を超えたときの誤りの詳細（"timeout" を含む）と、伏せた形の綴り（"***@"） | 記録に無い（テストの中だけ） |
| A3（kotowari-dir）の「README に .gitignore の1行（"/.kotowari/cache/"）を書く」 | 利用者。製品の振る舞いでなく README の決まりで、この話題の元の要求にも入っていないので書かなかった |
| A20（mds-spec）のスキーマの推奨の置き場 ".mds/schemas/" とキャッシュの置き場 ".mds/cache/" | 実装者（利用者の明示の判断ではない）。kotowari-dir の A2 と A3 で改められ、キャッシュの置き場は A3 のほうを書いた |

## 戻した細部

| 細部 | 新しい ID | 確かめているもの |
|---|---|---|
| 相対パスの ".." を重ねたとき、前の ".." を打ち消さない（打ち消すと、文書が意図しないスキーマで黙って検証される） | C-schema-resolve-13（「スキーマは文書の位置からの相対パスか URL で指定する」） | テスト consecutive_parent_refs_are_not_collapsed。理由は記録に無く、テストのコメントにある「何を防ぐか」を問題の節に書き、理由は記録に残っていないと書いた |

## 形に依存する要求

無し。この話題の元の要求は、どれも mds の製品の振る舞いで、古い仕様の形に依存しない。

## テストの無い行

無し。

## 誰が決めたか見分けられなかったもの

- REQ-schema-015（ほかのキーを読まない）と PROP-schema-003（コマンドによって解決先が変わらない）。どちらも出典の決定の文に無く、ほかの記録にも決めた行が見つからなかった。決めたことにも確認することにも書かなかった。今の振る舞いはテストに残る（reads_relative_schema_ref_and_ignores_other_keys が、ほかのキーがあっても "$schema" を読めることを、url_schema_resolves_the_same_for_all_commands が3つのコマンドで同じキャッシュを使うことを確かめる）。
- "http://" か "https://" で始まる値を URL とすること。A14（利用者）は「相対パスと URL の2つを受ける」だけで、見分け方の綴りは元の決定表にしかない。利用者から見える振る舞いなので、決定表に残した。

## 理由や見送った案が記録に無かった決定

- 「スキーマは文書の位置からの相対パスか URL で指定する」: 相対パスの基準を文書の位置にした理由は記録に無い（A14 の理由は URL を受ける理由）。見送った案が記録に無い。
- 「frontmatter が壊れているか、スキーマを指していない文書は、検査せずに停止する」: "$schema" が文字列でないときの理由が記録に無い（A43 は not recorded）。問題の節にそう書いた。見送った案が記録に無い。
- 「キャッシュに書けなくても、取得した内容で検査を続ける」: 見送った案が記録に無い。
- 「URL のスキーマを取得できないときと、取得が上限を超えたときは停止する」: 見送った案が記録に無い。
- 「停止の説明に URL を載せるときは、認証情報を伏せる」: 見送った案が記録に無い。

## 分けた所、まとめた所

- TBL-schema-003 の行を3つの決定に分けた: 指定の解き方（A14）、停止（A8、A43、P1）、取得の上限（mutants-gaps の A7）。表は、解き方の決定の「決めたこと」に置き、ほかの決定へリンクした。
- REQ-schema-013 を2つの決定に分けた: キャッシュと置き場（A14、mutants-gaps の A9、kotowari-dir の A1 と A3）と、キャッシュに書けないとき（todo-zero の A1。別の理由を持つ）。
- 基準のディレクトリの定義（用語集の語、kotowari-dir の A1）は、このまとまりで使うのがキャッシュの置き場だけなので、キャッシュの決定の中に書いた。
- 取得できないときの停止（P1）は、取得の上限と同じ決定にまとめた。どちらも URL の取得が終わらないときの停止である。

## あとでリンクにする所

無し。前の版で挙げた「CLI と結果の出し方」の話題への参照（「frontmatter が壊れているか、…」のディレクトリを対象にしたときの "$schema" の無い文書の扱い）は、その見出しへのリンクにした。冒頭の段落にも、停止の知らせ方を決める cli の見出しへのリンクを足した

## 迷った判断と、どう決めたか

- REQ-schema-015 と PROP-schema-003 を書くか。どちらも利用者から見える振る舞いだが、出典の決定の文にもほかの記録にも決めた行が無い。決める記録の行が無いので落としたままにする（依頼の指示）。「落とした細部」のそのほかの表に挙げた。
- 停止の決定を、この話題と「CLI と結果の出し方」の話題（REQ-schema-009、TBL-schema-009）のどちらに置くか。何を停止にするか（frontmatter の形と "$schema" の値）はこの話題に、停止をどう知らせるか（終了コード、理由の名前）は cli に置く、という分け方を依頼で確かめた。理由の名前（schema_not_found、frontmatter_invalid）はこの話題に書かず、cli の「停止の理由は5つにする」にだけある。C-3 と C-4 と cli の C-14 は同じテストに指されるが、こちらは停止になる入力を、あちらは3つのコマンドが終了コード 2 で止まることを書いている。
- 基準のディレクトリの定義を、core の話題（kotowari の基準のディレクトリ）と別に書いた。mds が kotowari に依存しないので、mds の決定として持つ。kotowari-dir の A1 の why は「依存の向きの話で置き場の名前には及ばない」と書いている。
- C-schema-resolve-3 はライブラリの誤り（frontmatter.rs、library.rs）と CLI の停止（cli.rs）の両方のテストが指す。行を「frontmatter の誤りになり、check、values、ast --schema は検査せずに停止する」として、どちらのテストも行の一部を確かめる形にした。
- URL の取得はライブラリ（`kotowari-markdown-schema-io`）の中にあるが、決めたことは mds の振る舞いとして書いた。I/O のクレートが CLI の規則をそのまま使い回すことは、書き直し済みの話題（library-io.md）の決定で、そこへリンクした。

## 数

- 元: 要求 6、場面 9、決定表 1、性質 1（ID は計17）
- 読んだ記録のファイル: 4（2026-09-21-mds-spec.md、2026-09-23-mutants-gaps.md、2026-09-24-kotowari-dir.md、2026-10-06-todo-zero.md）
- 出来上がり: 決定 6、確認することの行 13（うち「（読んで確認）」0）
- テスト: 39
