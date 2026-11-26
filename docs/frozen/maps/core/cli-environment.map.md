# cli-environment の対応表

元: `docs/ir/core/cli-environment.md`（日本語は `cli-environment.ja.md`）。書き直した文書: `cli-environment.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-107 | C-core-clienv-1, C-core-clienv-2 | "--help" と "--version" は、ほかの引数より先に受け、使い方か版を出して終わる | "check" 無しで終了コード0を C-1、ほかの引数の誤りに勝つことを C-2 にした。検査を行わないことは決めたことに書いた |
| REQ-core-108 | C-core-clienv-3 | 対象の OS は Linux と macOS で、Windows ではパスの区切りの正規化だけを約束する | 確かめ方（review）をそのまま「（読んで確認）」の行にした |
| REQ-core-109 | C-core-clienv-4, C-core-clienv-5 | 読めない入力と壊れた入力は、停止か誤りの指摘にし、黙って読み飛ばさない | 振り分けは決めたことの表にした。黙って飛ばす経路が無いことを C-4（読んで確認）、読めないものの停止を C-5 にした。変異テストの例外（REQ-core-141、REQ-core-142）は言葉で指した |
| REQ-core-120 | C-core-clienv-4 | 読めない入力と壊れた入力は、停止か誤りの指摘にし、黙って読み飛ばさない | REQ-core-109 と同じ決定（A100、P2）なのでまとめた。確かめ方に並んだ IR の読み取りの関数名は形に依存する |
| REQ-core-175 | C-core-clienv-7, C-core-clienv-13 | kotowari の中の写しの食い違いは、利用者の入力の誤りと分けて mapping error で止める | 形に依存しない部分（エンジンで読む文書の写しの食い違いを mapping error で止める）を残した。下の「形に依存する要求」 |
| REQ-core-176 | C-core-clienv-6 | 読めない入力と壊れた入力は、停止か誤りの指摘にし、黙って読み飛ばさない | 形に依存しない部分（局所的な形の誤りがある文書も文書をまたぐ検査を受ける）を、振り分けの決定に入れた |
| TBL-core-018 | C-core-clienv-7 | 停止の理由は、英語の固定の文言で示す | 表は「決めたこと」の Markdown の表にした。9つの理由を1行で確かめる |
| TBL-core-020 | C-core-clienv-8, C-core-clienv-9, C-core-clienv-10, C-core-clienv-11, C-core-clienv-12 | 停止の詳細は、直す人が見るファイルを基準のディレクトリからの相対パスで示す | 設定の誤り（C-8、C-9）、引数の誤り（C-10、C-11）、読めないファイルと UTF-8 でないファイル（C-12）。写しの誤りの詳細は C-13。ほかの話題が決める詳細（結果の誤りの詳細を含む）は、その話題の見出しへのリンクにした。未記載の面の一覧の詳細は agent の細部として落とした |
| EX-core-268 | C-core-clienv-13 | kotowari の中の写しの食い違いは、利用者の入力の誤りと分けて mapping error で止める | そのまま |
| EX-core-269 | C-core-clienv-6 | 読めない入力と壊れた入力は、停止か誤りの指摘にし、黙って読み飛ばさない | 場面は "- verification:" の行の欠けで書かれていて形に依存する。新しい形で読み替える |

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| tests/step1_cli.rs | req_004_unknown_option_stops | REQ-core-004, REQ-core-005, REQ-core-007, REQ-core-008, EX-core-001, TBL-core-018, TBL-core-020 | C-core-clienv-7, C-core-clienv-11 | 1行目が "argument error: unknown option: --verbose" であることを確かめる。ほかの ID は「コマンドと終了」の話題の分 |
| tests/step1_cli.rs | req_004_no_arguments_names_all_seven_commands | REQ-core-004, REQ-core-005, TBL-core-020, EX-core-219 | C-core-clienv-10 | この話題の分は TBL-core-020 の文言。ほかの ID は「コマンドと終了」の話題の分 |
| tests/step1_cli.rs | req_004_options_without_a_command_names_all_seven_commands | REQ-core-004, TBL-core-020, EX-core-241 | C-core-clienv-10 | 同上 |
| tests/step1_cli.rs | req_107_help_and_version_exit_zero_without_check | REQ-core-107, REQ-core-002, TBL-core-002 | C-core-clienv-1 | ほかの ID は「コマンドと終了」の話題の分 |
| tests/step1_cli.rs | req_107_help_wins_over_argument_errors | REQ-core-107, REQ-core-004 | C-core-clienv-2 | REQ-core-004 は「コマンドと終了」の話題の分 |
| tests/step1_cli.rs | req_005_config_error_detail_path_is_relative_to_base_not_to_cwd | REQ-core-005, TBL-core-020 | C-core-clienv-8 | REQ-core-005 は「コマンドと終了」の話題の分 |
| tests/step1_cli.rs | req_005_config_outside_the_base_is_shown_relative_with_parent_segments | REQ-core-005, TBL-core-020 | C-core-clienv-8 | 基準の外なら "../" を含むことは落とした細部で、テストの中に残る |
| tests/step1_config.rs | req_018_missing_ir_dir_stops_with_its_relative_path_and_the_os_error | REQ-core-005, REQ-core-018, TBL-core-001, TBL-core-018, TBL-core-020 | C-core-clienv-7, C-core-clienv-12 | 1行目が "unreadable file: docs/ir: <OS の誤り>" であることを確かめる。REQ-core-018、TBL-core-001 は「設定」と「コマンドと終了」の話題の分。入力が IR の置き場なので、新しい形では置き場を読み替える |
| tests/step1_config.rs | req_018_unreadable_ir_subdirectory_stops | REQ-core-018, TBL-core-001, TBL-core-020 | C-core-clienv-5, C-core-clienv-12 | 置き場の下の読めないディレクトリで停止し、詳細が相対パスと OS の誤りの文になることを確かめる。同上 |
| crates/kotowari/src/ir_tests.rs | req_033_broken_symlink_in_a_subdirectory_stops | REQ-core-033, REQ-core-018, TBL-core-020 | C-core-clienv-5, C-core-clienv-12 | 置き場の下の壊れたリンクで、読めないファイルを理由に止まり、詳細が相対パスで始まる。REQ-core-033、REQ-core-018 は別の話題の分 |
| crates/kotowari-core/src/finding_map.rs | a_value_that_cannot_be_mapped_stops_with_the_mapping_error_wording | REQ-core-175, EX-core-268 | C-core-clienv-7, C-core-clienv-13 | 1行目が "mapping error: " で始まり、詳細に値の説明が入ることを確かめる。入力が IR の話題の文書の写しで、新しい形では写しが変わる |
| tests/step5_findings.rs | ex_core_269_a_document_with_a_form_finding_still_takes_the_cross_document_checks | REQ-core-176, EX-core-269 | C-core-clienv-6 | 形の誤り（verification_missing）と duplicate_id の両方が出る。入力が IR で、新しい形の文書で書き直す |
| tests/step11_query.rs | req_157_unknown_id_stops | REQ-core-157, TBL-core-020, EX-core-251 | C-core-clienv-7 | この話題の分は1行目の argument error の文言。"unknown id: " の詳細は「1件の読み取り」の話題の分 |
| tests/step18_overview.rs | ex_core_465_overview_data_matched_by_guides_is_a_config_error | EX-core-465, REQ-core-280, TBL-core-020 | C-core-clienv-7 | この話題の分は config error の文言。重なりの詳細は「全体像の元データの検査」の話題の分 |
| tests/step18_overview.rs | req_core_280_the_first_overlap_in_byte_order_is_reported_and_tests_overlap_too | REQ-core-280, TBL-core-020 | C-core-clienv-7 | 同上 |
| tests/step19_overview_build.rs | ex_core_477_errors_in_overview_data_stop_the_build_without_writing | EX-core-477, REQ-core-294, REQ-core-296, TBL-core-001, TBL-core-018, TBL-core-020 | C-core-clienv-7 | この話題の分は overview error の文言。詳細と書かないことは「全体像を作るコマンドと見せるコマンド」の話題の分 |
| tests/step19_overview_build.rs | ex_core_463_build_without_the_overview_key_is_a_config_error | EX-core-463, REQ-core-279, TBL-core-020 | C-core-clienv-7 | この話題の分は config error の文言。"overview is not configured" は全体像のコマンドの話題の分 |
| tests/step19_overview_build.rs | ex_core_504_a_linked_cache_place_stops_without_writing_outside | EX-core-504, REQ-core-324, REQ-core-296, TBL-core-001, TBL-core-018, TBL-core-020 | C-core-clienv-7 | この話題の分は cache error の文言。リンクの置き場で止めることは全体像のコマンドの話題の分 |
| crates/kotowari/tests/project.rs | overview_write_names_the_place_it_could_not_inspect | REQ-core-324, TBL-core-020 | C-core-clienv-7 | ライブラリの誤りの詳細が "cache error: " で始まる。置き場の詳細は全体像のコマンドの話題の分 |
| tests/step20_overview_serve.rs | ex_core_479_a_port_in_use_stops_with_a_port_error | EX-core-479, REQ-core-298, TBL-core-001, TBL-core-018, TBL-core-020 | C-core-clienv-7 | この話題の分は port error の文言。ポートの詳細は落とした細部（全体像のコマンドの話題） |
| tests/step13_test_languages.rs | ex_core_378_a_missing_rule_file_stop_points_at_the_rule_file | REQ-core-189, TBL-core-020, EX-core-378 | C-core-clienv-9 | 詳細がルールのファイルのパスを含み、設定ファイルのパスを含まないことを確かめる。REQ-core-189 は「設定で足す問い合わせ」の話題の分 |
| tests/step17_surface.rs | req_225_a_broken_rule_file_stops_check_and_status_naming_the_rule_file | REQ-core-225, TBL-core-020 | C-core-clienv-9 | REQ-core-225 は「面の検査」の話題の分 |
| tests/step17_surface.rs | ex_core_425_a_list_that_is_not_a_sequence_stops_naming_the_list | EX-core-425, REQ-core-231, TBL-core-020 | C-core-clienv-9 | 落とした細部のテスト。いちばん近い行を指す（未記載の面の一覧のパスを詳細に出すことは agent の細部として落とした）。REQ-core-231 は未記載の面の一覧の話題の分 |
| tests/step17_surface.rs | req_231_a_list_that_is_not_yaml_or_not_utf8_stops_check_and_status | REQ-core-231, TBL-core-001, TBL-core-020 | C-core-clienv-9, C-core-clienv-12 | "non-UTF-8 file: docs/surface.yaml"（C-12）。一覧のパスの config error は落とした細部で、いちばん近い行 C-9 を指す |
| tests/step15_guides.rs | ex_368_check_stops_when_a_guide_is_also_a_test_file | REQ-core-199, TBL-core-001, TBL-core-020, EX-core-368 | C-core-clienv-7 | この話題の分は config error の文言。重なりの詳細は「ガイドの印」の話題の分 |
| tests/step15_guides.rs | req_199_only_the_first_overlapping_file_in_byte_order_is_reported | REQ-core-199, TBL-core-020 | C-core-clienv-7 | 同上 |

REQ-core-108、REQ-core-109、REQ-core-120 を指すテストは無い（確かめ方が review）。C-core-clienv-5 は上の3本のテストが確かめる。

ガイドのマーカー: `docs/guides/cli.md`（と `.ja.md`）の REQ-core-107、REQ-core-109、REQ-core-108、TBL-core-018、TBL-core-020 は、上の「決定の見出し」で付け替える。

## 落とした細部

安全に関わるもの:

| 要求のどの部分 | 誰が決めたと記録にあるか | 戻したか |
|---|---|---|
| TBL-core-020: カレントディレクトリを取得できないときは、読めないファイルを理由に停止し、詳細を "current directory: " と OS の誤りの文にする（停止の条件。落とすと、基準のディレクトリを決められないまま動く経路が決定から消える） | 実験者の裁定（records.md の A160。利用者には聞いていない） | 戻していない。確かめるテストが無い（"current directory" の文字は `src/cli.rs` の `StopReason::UnreadableFile` を作る所にだけあり、テストにも skill の文にも無い）。要るテストは下の「テストの無い行」 |

## 戻した細部

無い。`safety-dropped.md` のこの話題の1件（上の表）は、確かめるものが無いので戻していない。

そのほか:

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| TBL-core-020: 設定ファイルが基準の外にあれば、詳細のパスに "../" を含める | 実験者の裁定（records.md の A164） |
| TBL-core-020: 引数の誤りの詳細の「説明の文」 | 実験者の裁定（records.md の A147）。「問題の引数の文字」は利用者が採った A137 なので残した |
| TBL-core-020: 未記載の面の一覧が壊れているときの config error の詳細に、設定ファイルでなく一覧のファイルの相対パスを出す | 2026-09-27-surface-check.md の A22（decided_by は「利用者（IR を書く段で決めた細部をまとめて採用）」）。細部をまとめて採ったものなので agent の細部として落とした。等価の一覧（2026-09-17-mutation-tests.md の A49）と "tests.rules" のルールのファイル（2026-09-24-guide-gaps.md の A2）は、どちらも「利用者（推奨を採用）」なので残した。"surface.rules" のルールのファイルは、"tests.rules" と同じ扱いで止めると利用者が決めた（surface-check の A12）ので残した |
| TBL-core-020: ポートの誤りの詳細 "127.0.0.1:<ポート>" と ": " と OS の誤りの文 | LLM（2026-10-02-whole-picture.md の A72、A85。承認の時に利用者に示す）。全体像のコマンドの話題でも同じ扱いになる |
| TBL-core-020: "overview" の鍵が無いときの詳細 "overview is not configured"、全体像の元データの置き場の重なりの文言 | LLM（whole-picture の A72、A73、A75）。全体像の話題に属するので、この文書には鍵の名前と場面だけを書いた |
| TBL-core-020: 置き場の誤りの詳細が基準のディレクトリからの相対であること | LLM（2026-10-06-changes-rethink.md の A30。照合で追認）。置き場の誤りの詳細そのものは全体像のコマンドの話題に属する |
| TBL-core-020: 引数が無いときの文言のコマンドの並び（バイト順、今の7つ） | 並びの形は利用者（mutation-tests の A39、read-commands の A20、query-status の A18）。plan を足した版は主セッション（plan-schema の A30）、changes を外した版は LLM（changes-rethink の A23）。コマンドを足し引きするたびに並びを直すことは利用者の決定なので、今の文字列を残した |
| REQ-core-120 の確かめ方: `GherkinBlock`、invalid_gherkin_line、`read_utf8_file`、`parse_document`、`check_documents`、`load_all` を見ること | 確かめ方の手順（IR の読み取りのコードに依存する） |
| REQ-core-109 の確かめ方: `rg 'filter_map|if let Ok'` で黙って飛ばす経路を探すこと | 確かめ方の手順 |
| TBL-core-018: ポートの誤りの名前 "port error" の振る舞い（ポートが使えないときに止まること） | LLM（whole-picture の A72）。名前は利用者から見えるので表に残し、いつ止まるかは全体像のコマンドの話題に任せた |

## 形に依存する要求

| 元の ID | 扱い |
|---|---|
| REQ-core-175 | 新しい形で読み替えて残る。「IR の文書について」は「kotowari がスキーマのエンジンで読む文書」に、「対応表（TBL-core-030）に写し先が無い」は「エンジンの指摘に写し先が無い」に読み替えた。IR の話題の文書の読み取りそのものは新しい形で無くなる。決定の文書はエンジンで読む（form-contract の「決定の文書は、本体に埋め込んだスキーマで mds のエンジンが読む」、提案）ので、写しの対象に決定の文書が入る |
| REQ-core-176 | 新しい形で読み替えて残る。「スキーマの側の指摘が出た IR の文書」を「局所的に形から外れた所のある文書」に読み替え、振り分けの決定に入れた |
| EX-core-269 | 新しい形で読み替えて残る。"- verification:" の行の欠けと要求の ID の重なりの代わりに、決定の文書で、形から外れた所（たとえば状態の行の誤り）と、確認することの ID の重なりを同時に作り、両方の誤りが出ることを確かめる |
| REQ-core-120 | 決めたこと（除外に列挙していない入力を黙って読み飛ばさない）は残る。確かめ方に並んだ IR の読み取りの型と関数（`GherkinBlock`、invalid_gherkin_line、`parse_document`）は、新しい形で無くなる |

## テストの無い行

- 戻せなかった安全の細部（カレントディレクトリを取得できないときの停止）: 要るテストは、作ってから消したディレクトリの中で kotowari を動かす結合のテスト（例えば `sh -c 'mkdir d && cd d && rmdir ../d && exec kotowari check'`）で、終了コード2、空の標準出力、標準エラーの1行目が "unreadable file: current directory: " で始まることを確かめる。テストができたら、「停止の詳細は、直す人が見るファイルを…」の決めたことに「カレントディレクトリを取得できないときは、読めないファイルを理由に停止し、詳細を "current directory: " と OS の誤りの文にする」を足し、確認することの行 C-core-clienv-14「カレントディレクトリを取得できないとき、check は終了コード2で停止し、標準エラーの1行目が "unreadable file: current directory: " で始まる」を足す。
- 実装済みの行には、どれも付け替えたあとのテストがある。ただし C-core-clienv-6 のテスト（`ex_core_269_...`）と C-core-clienv-13 のテスト（`a_value_that_cannot_be_mapped_...`）は IR の文書を入力にしていて、新しい形へ移すときに入力を書き直す必要がある。C-core-clienv-5、C-core-clienv-12 のテストのうち IR の置き場を使うもの（`req_018_...`、`req_033_...`）も、置き場を新しい形の置き場に読み替える。

## 利用者に聞くこと

無い。前の版の問い（決定の文書をエンジンで読むか）には答えが出た（下の「迷った判断」）。

## 誰が決めたか見分けられなかったもの

- records.md の第9ラウンドの決定（A100、A101、A102、A103、A104、A125、A136、A137）には、決定の行ごとの decided_by が無い。ラウンドの見出しに「すべて推奨をそのまま採った（利用者の答えは「進めて」）」とある。利用者が推奨の一覧を見て採ったものとして、利用者の決定に数えた。同じファイルの「実験者の裁定」と「第9ラウンドの続き（実験者が裁定した）」の決定（A147、A160、A164）は、LLM の決定として落とした。
- 2026-09-27-surface-check.md の A22（未記載の面の一覧が壊れているときの停止の詳細）は「IR を書く段で決めた細部をまとめて採用」とある。推奨をまとめて採ったものなので、agent の細部として落とした（C-core-clienv-9 から未記載の面の一覧を外した）。

## 理由や見送った案が記録に無かった決定

- 「"--help" と "--version" は、ほかの引数より先に受け、使い方か版を出して終わる」: 理由も見送った案も無い（A103、A136 は「敵対的レビューの指摘」とだけある）。
- 「対象の OS は Linux と macOS で、Windows ではパスの区切りの正規化だけを約束する」: 理由も見送った案も無い（A125）。
- 「読めない入力と壊れた入力は、停止か誤りの指摘にし、黙って読み飛ばさない」: 見送った案が無い。理由は第9ラウンドの見出しと A98 の理由から書いた。
- 「停止の理由は、英語の固定の文言で示す」: 見送った案は cache error の A9 の rejected だけ。
- 「停止の詳細は、直す人が見るファイルを基準のディレクトリからの相対パスで示す」: 見送った案が無い。
- 「kotowari の中の写しの食い違いは、利用者の入力の誤りと分けて mapping error で止める」: 見送った案は、A36 の rejected（スキーマをファイルから読む）だけで、これはスキーマの置き方の決定の代わりなので、「IR の形の固定」の話題に属するとして書かなかった。

## 分けた所、まとめた所

- REQ-core-109 と REQ-core-120 を1つの決定にまとめた。どちらも A100 と P2 で、120 は 109 の「黙って飛ばさない」の言い換えである。REQ-core-176 の形に依存しない部分も同じ決定に入れた。
- TBL-core-018 と TBL-core-020 は別の決定にした。文言（理由の名前）と、詳細（何を出すか）は別の記録で決まり、理由も違う（A104 の英語にする理由と、A49・A2・A36 の直す人が見るパスの理由）。
- REQ-core-175 は TBL-core-018 の「写しの誤り」と合わせて独立の決定にした。利用者が直せない不具合を分ける理由（A9）と、名前の理由（A73）を持つため。
- TBL-core-020 の行のうち、ほかの話題の要求（REQ-core-148、REQ-core-189、REQ-core-225、REQ-core-231、REQ-core-199、REQ-core-280、REQ-core-279、REQ-core-294、REQ-core-298、REQ-core-324、REQ-core-157）が決める詳細は、等価の一覧・ルールのファイル・未記載の面の一覧のパスを出すことだけを C-core-clienv-9 にまとめて残し、残りは言葉で指した。

## あとでリンクにする所

- 用語集（`docs/decisions/core/CONTEXT.md`。まだ下書きが無い）: 「読めない入力と壊れた入力は…」の除外の中身。用語集は check が読まない人向けの文書なので、見出しへのリンクにはしない見込み
- 「エンジンの指摘の写し先」の話題（finding-map）: 「kotowari の中の写しの食い違いは…」の写し先。finding-map には今有効な決定が無いので、言葉のままにした。「文書の形の固定」の話題（form-contract）の「決定の文書は、本体に埋め込んだスキーマで mds のエンジンが読む」（提案）は、こちらの決定へリンクしている。こちらから form-contract へのリンクは、決めたことが「エンジンで読む文書一般」のままで成り立つので、張っていない

ここで済ませたもの（見出しへのリンクにした。下書きどうしも `.ja.md`）: cli.ja.md の終了コードと停止の出力の決定（冒頭の段落）、overview-commands.ja.md の overview error・port error・cache error の決定、overview-data.ja.md の "overview" の鍵が無いときと置き場の重なりの決定、guides.ja.md の置き場の重なりの決定、query.ja.md の無い ID の決定、equivalents.ja.md の2つの決定（変異の結果と等価の一覧の1件の例外）、mutants-input.ja.md の「形の合わない結果のファイルでは、結果の誤りで停止する」（文言の表と詳細の表の結果の誤り）、surface-unspecified.ja.md（話題の文書へ）

## 迷った判断と、どう決めたか

- REQ-core-175 と REQ-core-176 は、元の文が IR の文書について書かれていて、形に依存する。ただ、mapping error は IR のほか、計画書、全体像の元データ、面の検査、テストのファイルの読み取りでも使われている（コードの StopReason::MappingError の使われ方で確かめた）。利用者が直せない不具合を入力の誤りと分ける（A9）という形に依存しない部分を、決定として残した。REQ-core-176 も、局所的な誤りは指摘にする（A101）の帰結として、振り分けの決定に入れた。
- TBL-core-020 は、ほかの話題の要求が決める詳細を多く含む。同じ決定を2か所に書かないため、この文書には一般の決まり（基準からの相対パス、英語、直す人が見るファイル）と、ほかの話題が持たない行（設定・引数・読めない・UTF-8・結果・写し）を書き、ほかは言葉で指した。名前（"guides.files"、"tests.files"、"overview.files"、"tests.rules"、"surface.rules"、"overview"、"kotowari overview build"、"kotowari overview serve"、"kotowari query"）は、元の文書に出てくるので、指す文の中に残した。
- 引数が無いときの文言は「コマンドと終了」の話題の REQ-core-004 にも関わる。文言は TBL-core-020 が決めているので、この文書に置いた。
- "port error" は名前も振る舞いも LLM が決めた（A72）が、利用者から見える名前なので表に残した。詳細は落とした。
- "results error" の詳細は、mutants-input の「形の合わない結果のファイルでは、結果の誤りで停止する」が決めていて、確認することの行（C-core-mutin-5）もそちらにある。振る舞いが見える側に残し、この文書の詳細の表はその見出しへのリンクにした。文言の表には、9つの理由をそろえて見せるため "results error" の名前を残し、同じ見出しへリンクした。
- 「など」と「のような」を直した（C-core-clienv-2、C-core-clienv-6、C-core-clienv-11 と、「読めない入力と壊れた入力は…」と「kotowari の中の写しの食い違いは…」の決めたことの、計5か所）。挙げ切れない所は「例えば」で例だと分かるようにした。C-core-clienv-11 は、例をテストが確かめる1行目の形にした。
- 記録の読み方は変えていない。records.md の第9ラウンド（「進めて」）は利用者の決定、A145 以降の裁定は agent の細部として数えている（書き直しの決まりの読み方と同じ）。
- C-core-clienv-3 は確かめ方が review だったので、「（読んで確認）」にした。
- 決定の文書をエンジンで読むか: 利用者の答えは「決定の文書は、kotowari に埋め込んだスキーマで mds が読む」。mapping error の対象に決定の文書が入る。決めたことは「エンジンで読む文書一般」で書いてあるので変えない。mapping error のテスト（a_value_that_cannot_be_mapped_stops_with_the_mapping_error_wording）の入力は、付け替えのときに決定の文書に書き直せる。

## 数

- 元: 要求 6、場面 2、決定表 2、性質 0
- 読んだ記録のファイル: 13（records.md、2026-09-17-mutation-tests.md、2026-09-19-read-commands.md、2026-09-20-query-status.md、2026-09-22-ir-engine.md、2026-09-23-ir-english-tokens.md、2026-09-24-doc-marks.md、2026-09-24-guide-gaps.md、2026-09-24-plan-schema.md、2026-09-27-surface-check.md、2026-10-02-whole-picture.md、2026-10-04-overview-on-public-api.md、2026-10-06-changes-rethink.md）
- 出来上がり: 決定 6（実装済み 6）、確認することの行 13（うち「（読んで確認）」2）
- 確定で: 戻した細部 0、戻せなかった細部 1（要るテストは「テストの無い行」）、直した「など」「のような」5
- テスト: 26
