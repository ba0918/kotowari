# cli の対応表

元: `docs/ir/schema/cli.md`（日本語は `cli.ja.md`）。書き直した文書: `cli.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-schema-005 | C-schema-cli-1, C-schema-extract-44 | mds の CLI は kotowari-mds の名前で、check、ast、values、--version を受ける | `--version` を C-1 にした。`ast` の素の構文木は extraction の「ast は、検査とは別に、文書の素の構文木を JSON で出す」（C-schema-extract-44）に一本化し、この決定からはリンクで指す（以前の C-schema-cli-2 は欠番）。`check` と `values` はほかの行（終了コード、出力の形、抽出の話題）で確かめる。`ast --schema` は別の決定 |
| REQ-schema-006 | C-schema-cli-5 | 終了コードは、指摘が無ければ 0、あれば 1、検査を行えなければ 2 にする | 0・1・2 の値違いを1行にまとめた |
| REQ-schema-007 | C-schema-cli-6 | 出力の形は、--format で text と json から選ぶ | `ast` が `json` だけを受けることの確かめは、停止の理由の C-17（`argument_error`）に置いた |
| REQ-schema-066 | C-schema-cli-7 | values の text 出力は、入れ子ごとに空白2つで字下げし、配列の要素に1からの番号を付ける | そのまま |
| REQ-schema-008 | C-schema-cli-8, C-schema-cli-9, C-schema-cli-10, C-schema-cli-11, C-schema-cli-12, C-schema-cli-13 | 指摘は、種類、深刻度、文書のパス、行番号、詳細を持つ | 9つの記録の決定を束ねていたので、基本の組（C-8）、行（C-9、C-10）、ノードの名前（C-11）、生の行（C-12）、出現回数の規則種別（C-13）の5つの決定に分けた。undeclared_line の規則種別は closed-world の決定へ |
| REQ-schema-009 | C-schema-cli-14 | 検査を行えないときは、部分的な結果を出さずに停止する | 停止の条件の表は TBL-schema-009 の決定 |
| REQ-schema-010 | C-schema-cli-25 | ディレクトリの検査は、$schema を持つ文書だけを検査し、. で始まるディレクトリとシンボリックリンクを辿らない | 空の "$schema" で全体を停止することは C-16 と C-17 で確かめる |
| REQ-schema-042 | C-schema-cli-17 | 停止の理由は5つにする | 定義は TBL-schema-009 |
| REQ-schema-043 | C-schema-cli-15, C-schema-cli-29 | 検査を行えないときは、部分的な結果を出さずに停止する | 理由の名前が出ることは C-17。参照先のファイルの中身を流さないこと（安全に関わる細部）を C-29 に戻した |
| REQ-schema-065 | C-schema-cli-24 | 停止の説明の文言は約束せず、問題の欄の名前と、期待した型か受け付ける語を含める | 文言を約束しないこと（記録の A6）も同じ決定に入れた |
| REQ-schema-053 | C-schema-cli-16 | 検査を行えないときは、部分的な結果を出さずに停止する | そのまま |
| REQ-schema-044 | C-schema-cli-26, C-schema-cli-28 | ディレクトリの検査は、$schema を持つ文書だけを検査し、. で始まるディレクトリとシンボリックリンクを辿らない | "." で始まるディレクトリ（と、その中の ".kotowari/"）を C-26、シンボリックリンクを辿らないこと（安全に関わる細部）を C-28 に戻した。".md" 以外のファイルは落とした細部（下の表） |
| REQ-schema-067 | C-schema-cli-3, C-schema-cli-4 | ast --schema は、values --format json と同じ値に、スキーマの name を type として添える | 出力の形を C-3、"name" が `values` を変えないことを C-4 にした |
| TBL-schema-001 | C-schema-cli-5 | 終了コードは、指摘が無ければ 0、あれば 1、検査を行えなければ 2 にする | 決めたことの中の表にした |
| TBL-schema-009 | C-schema-cli-17, C-schema-cli-18, C-schema-cli-19, C-schema-cli-20, C-schema-cli-21, C-schema-cli-22, C-schema-cli-23 | 停止の理由は5つにする | 23の出典を束ねていたので、理由の表（C-17）と、記録で理由と見送った案を持つ条件ごとの決定（配置パスの衝突 C-18・C-19、name と type C-20、2度の宣言 C-21、空の名前 C-22、2回のオプション C-23）に分けた |
| TBL-schema-002 | - | 指摘は、種類、深刻度、文書のパス、行番号、詳細を持つ | 分類の表を決めたことの中に置いた。分類そのものを確かめる行は置かず、個々の種類は他の話題で確かめる |
| PROP-schema-002 | C-schema-cli-27 | values と ast --schema は、検査の合否と関係なく値を組み立てる | 性質の行をやめ、違反のある文書で `values` が値を出して終了コード 0 で終わる行にした（テスト ex_schema_080 が確かめる）。library-extraction の C-schema-libext-6 と同じ振る舞いだったので、この話題に一本化し、あちらからリンクで指す |
| EX-schema-082 | C-schema-cli-3 | ast --schema は、values --format json と同じ値に、スキーマの name を type として添える | そのまま |
| EX-schema-083 | C-schema-cli-3 | ast --schema は、values --format json と同じ値に、スキーマの name を type として添える | EX-082 と同じ行にまとめた |
| EX-schema-081 | C-schema-cli-23 | 同じオプションを2回渡すと、引数の誤りで停止する | そのまま |
| EX-schema-076 | C-schema-cli-21 | 同じ置き場の同じ名前と、表のヘッダの同じ列の名前を、2度宣言すると停止する | そのまま |
| EX-schema-078 | C-schema-cli-21 | 同じ置き場の同じ名前と、表のヘッダの同じ列の名前を、2度宣言すると停止する | EX-076 と同じ行にまとめた |
| EX-schema-077 | C-schema-cli-22 | 配置パスの、ドットで区切った名前が空なら停止する | そのまま |
| EX-schema-073 | C-schema-cli-20 | name を宣言したスキーマで、根の type に値を置くと停止する | そのまま |
| EX-schema-003 | C-schema-cli-5 | 終了コードは、指摘が無ければ 0、あれば 1、検査を行えなければ 2 にする | そのまま |
| EX-schema-004 | C-schema-cli-14, C-schema-cli-17 | 検査を行えないときは、部分的な結果を出さずに停止する | 停止することを C-14、理由の名前を C-17 で確かめる。「指摘を出力しない」は C-15 |
| EX-schema-026 | C-schema-cli-9 | 指摘の行は、違反したノードの開始行か、欠落ならそれを含むノードの開始行にする | そのまま |
| EX-schema-027 | C-schema-cli-11 | 指摘に、宣言したノードの名前を構造として持たせる | そのまま |
| EX-schema-028 | C-schema-cli-12 | 指摘に、指す行の文字をそのまま持たせる | そのまま |
| EX-schema-029 | C-schema-cli-18 | 配置パスが衝突するスキーマは、文書を読まずに停止する | そのまま |
| EX-schema-030 | C-schema-cli-18, C-schema-cli-19 | 配置パスが衝突するスキーマは、文書を読まずに停止する | "a" と "a.b" の停止が C-18、"a.b" と "a.c" が C-19 |
| EX-schema-047 | C-schema-cli-18 | 配置パスが衝突するスキーマは、文書を読まずに停止する | 要素オブジェクトの外の衝突 |
| EX-schema-048 | C-schema-cli-19 | 配置パスが衝突するスキーマは、文書を読まずに停止する | そのまま |
| EX-schema-049 | C-schema-cli-13 | 出現回数の指摘に、数えたノードの規則種別を持たせる | そのまま |
| EX-schema-062 | C-schema-cli-24 | 停止の説明の文言は約束せず、問題の欄の名前と、期待した型か受け付ける語を含める | 値違いの場面を1行にまとめた |
| EX-schema-063 | C-schema-cli-24 | 停止の説明の文言は約束せず、問題の欄の名前と、期待した型か受け付ける語を含める | 同上 |
| EX-schema-064 | C-schema-cli-7 | values の text 出力は、入れ子ごとに空白2つで字下げし、配列の要素に1からの番号を付ける | そのまま |
| EX-schema-065 | C-schema-cli-7 | values の text 出力は、入れ子ごとに空白2つで字下げし、配列の要素に1からの番号を付ける | EX-064 と同じ行にまとめた |

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-mds/tests/url_schema.rs | url_schema_without_cache_and_unreachable_server_stops | REQ-schema-011, REQ-schema-009 | C-schema-cli-17 | 取得できない URL で終了コード 2 と `schema_not_found`。REQ-schema-011 の分は「スキーマの解決」の話題で付け替える |
| crates/kotowari-mds/tests/url_schema.rs | schema_response_over_4mib_stops | REQ-schema-011, REQ-schema-009, EX-schema-066 | C-schema-cli-17 | 4MiB を超える応答で終了コード 2 と `schema_not_found`。REQ-schema-011 と EX-schema-066 の分は「スキーマの解決」の話題で付け替える |
| crates/kotowari-markdown-schema/src/document/tests.rs | title_heading_keeps_the_raw_line_including_inline_code | REQ-schema-008 | C-schema-cli-12 | 指摘が持つ生の行の元になる、文書の行の文字そのままを確かめている（指摘そのものではない） |
| crates/kotowari-markdown-schema/src/schema/tests.rs | an_empty_name_in_the_value_or_an_of_key_of_an_element_object_is_schema_invalid | REQ-schema-036, TBL-schema-009 | C-schema-cli-22 | 落とした細部のテスト。いちばん近い行を指す。確かめているのは "value" と "of" の鍵の空の名前で、空の名前の決まり（review4-gaps の A4）は配置パスだけを決めている。extraction の対応表の側も、空の名前の決定をこの話題に一本化したので、ここに付け替えた |
| crates/kotowari-markdown-schema/src/schema/tests.rs | duplicate_key_between_an_inner_node_and_a_derived_value_is_schema_invalid | REQ-schema-042, REQ-schema-048 | C-schema-cli-18 | 要素オブジェクトの中の内側の配置パスと "of" の鍵の重なりを parse_schema が受けないこと。REQ-schema-048 の分は「抽出と素の構文木」の話題で付け替える |
| crates/kotowari-markdown-schema/src/schema/tests.rs | duplicate_key_between_value_and_a_derived_value_is_schema_invalid | REQ-schema-042, REQ-schema-048 | C-schema-cli-18 | "value" と "of" の鍵の重なり。REQ-schema-048 の分は同上 |
| crates/kotowari-markdown-schema/src/schema/tests.rs | a_parent_key_is_a_duplicate_but_a_sibling_key_is_not | REQ-schema-042, REQ-schema-048 | C-schema-cli-18, C-schema-cli-19 | "a" と "a.b" は受けず、"a.b" と "a.c" は受ける。REQ-schema-048 の分は同上 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | missing_required_field_points_at_the_item_heading_with_the_field_name | REQ-schema-008 | C-schema-cli-9, C-schema-cli-11, C-schema-cli-12 | 欠落の行が項目の見出しの行、ノードの名前、生の行 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | a_field_line_declared_twice_points_at_the_second_line | REQ-schema-008 | C-schema-cli-9, C-schema-cli-11, C-schema-cli-12 | 上限を超えた2つ目の行、ノードの名前、生の行 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | a_table_row_with_the_wrong_column_count_points_at_that_row | REQ-schema-008, REQ-schema-033 | C-schema-cli-9, C-schema-cli-11, C-schema-cli-12 | データ行の行、表にノードの名前が無いこと、生の行。REQ-schema-033 の分は「見出しの下の行の規則」の話題で付け替える |
| crates/kotowari-markdown-schema/src/validate/tests.rs | missing_title_has_neither_a_line_nor_a_node_name | REQ-schema-022, REQ-schema-008 | C-schema-cli-10, C-schema-cli-11, C-schema-cli-12 | 行の無い指摘が行番号・ノードの名前・生の行を持たない。REQ-schema-022 の分は「文書の骨格」の話題で付け替える |
| crates/kotowari-markdown-schema/src/validate/tests.rs | invalid_id_carries_the_raw_heading_line | REQ-schema-008 | C-schema-cli-9, C-schema-cli-12 | 見出しの行と、その生の行 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | missing_required_section_carries_the_section_name | REQ-schema-008 | C-schema-cli-10, C-schema-cli-11 | 節の名前を持ち、含むノードの行が無いので行番号を持たない |
| crates/kotowari-markdown-schema/src/validate/tests.rs | field_order_mismatch_carries_the_name_of_the_field_out_of_order | REQ-schema-041, REQ-schema-008 | C-schema-cli-9, C-schema-cli-11, C-schema-cli-12 | 違反した行、ノードの名前、生の行。REQ-schema-041 の分は「見出しの下の行の規則」の話題で付け替える |
| crates/kotowari-markdown-schema-io/tests/loader.rs | directory_checks_dot_prefixed_documents_but_skips_hidden_directories | REQ-schema-044, REQ-schema-073 | C-schema-cli-26 | この話題の分は、"." で始まるディレクトリの下を読まないこと（SchemaLoader の check の側）。REQ-schema-073 の分は library-io の C-schema-io-2 |
| crates/kotowari-mds/tests/cli.rs | directory_check_reports_dot_prefixed_markdown_and_skips_hidden_directories | REQ-schema-044, REQ-schema-073, REQ-core-313 | C-schema-cli-26 | この話題の分は、".hidden/" の下の文書を検査しないこと。"." で始まるファイルを検査することは REQ-core-313 の分。REQ-schema-073 の分は library-io の C-schema-io-2 |
| crates/kotowari-mds/tests/cli.rs | version_prints_the_command_name_and_version_to_stdout_and_exits_zero | REQ-schema-005 | C-schema-cli-1 | 出力がコマンドの名前で始まることは落とした細部で、テストの中に残る |
| crates/kotowari-mds/tests/cli.rs | ast_outputs_mdast_json_for_the_adr_fixture | REQ-schema-005, REQ-schema-040 | - | 素の構文木を出すことは extraction の C-schema-extract-44 に一本化した。付け替え先は extraction の対応表にある |
| crates/kotowari-mds/tests/cli.rs | ast_with_format_text_stops_with_argument_error | REQ-schema-009, REQ-schema-042, REQ-schema-043 | C-schema-cli-17 | 終了コード 2 と `argument_error` の名前。1行だけかは確かめていない |
| crates/kotowari-mds/tests/cli.rs | unknown_flag_stops_with_argument_error | REQ-schema-009, REQ-schema-042 | C-schema-cli-17 | 知らないフラグで `argument_error` |
| crates/kotowari-mds/tests/cli.rs | ast_schema_outputs_typed_tree | REQ-schema-036, REQ-schema-067 | C-schema-cli-3 | "name" を宣言したスキーマで根の "type" が入ること。配置パスの入れ子は REQ-schema-036 の分 |
| crates/kotowari-mds/tests/cli.rs | ex_schema_082_083_ast_schema_is_values_with_the_schema_name_as_type | REQ-schema-067, EX-schema-082, EX-schema-083 | C-schema-cli-3, C-schema-cli-4 | 2つの出力の比べと、"name" を宣言しても `values` に "type" が入らないこと |
| crates/kotowari-mds/tests/cli.rs | ast_schema_without_schema_stops | REQ-schema-009, REQ-schema-014, REQ-schema-042 | C-schema-cli-14 | "$schema" の無い文書で終了コード 2。理由を `schema_not_found` に分けることは落とした細部（LLM の分類）で、テストの中に残る。REQ-schema-014 の分は「スキーマの解決」の話題 |
| crates/kotowari-mds/tests/cli.rs | check_clean_document_exits_zero_with_no_output | REQ-schema-006, EX-schema-003 | C-schema-cli-5 | 0 と、何も出力しないこと |
| crates/kotowari-mds/tests/cli.rs | check_json_outputs_empty_files_array_for_clean_file | REQ-schema-007 | C-schema-cli-6 | `--format json` で JSON。"files" の配列の形は落とした細部 |
| crates/kotowari-mds/tests/cli.rs | check_json_outputs_empty_files_array_for_clean_directory | REQ-schema-007, REQ-schema-010 | C-schema-cli-6 | ディレクトリでも JSON で出ること。REQ-schema-010 の対象の選び方は確かめていない（文書が1つで、"$schema" を持つ） |
| crates/kotowari-mds/tests/cli.rs | check_document_with_findings_exits_one_with_text | REQ-schema-006, REQ-schema-007 | C-schema-cli-5, C-schema-cli-6 | 指摘があれば 1、text の出力。`--format` を省くと text になることは落とした細部で、テストの中に残る |
| crates/kotowari-mds/tests/cli.rs | check_json_output_has_files_with_findings | REQ-schema-007, REQ-schema-008 | C-schema-cli-6, C-schema-cli-8, C-schema-cli-9 | JSON の出力、指摘の5つの要素、宣言していない節の見出しの行 |
| crates/kotowari-mds/tests/cli.rs | check_skips_bom_and_counts_crlf_lines | REQ-schema-008 | C-schema-cli-9 | 見出しの行を指すこと。BOM と CRLF の行の数え方は落とした細部で、テストの中に残る |
| crates/kotowari-mds/tests/cli.rs | check_missing_title_has_no_line_number | REQ-schema-008, REQ-schema-022 | C-schema-cli-10 | text の出力で題名の欠落に行番号が無いこと。REQ-schema-022 の分は「文書の骨格」の話題 |
| crates/kotowari-mds/tests/cli.rs | check_missing_schema_stops_with_schema_not_found | REQ-schema-009, REQ-schema-011, REQ-schema-042 | C-schema-cli-14 | "$schema" の無い文書で終了コード 2。`schema_not_found` への分類は落とした細部。REQ-schema-011 の分は「スキーマの解決」の話題 |
| crates/kotowari-mds/tests/cli.rs | check_unresolvable_schema_stops_with_schema_not_found | REQ-schema-009, REQ-schema-011, REQ-schema-042 | C-schema-cli-14, C-schema-cli-17 | 参照先の無いスキーマで終了コード 2 と `schema_not_found`。REQ-schema-011 の分は同上 |
| crates/kotowari-mds/tests/cli.rs | check_unreadable_file_stops | REQ-schema-009, REQ-schema-042 | C-schema-cli-17 | 読めない文書で `unreadable_file` |
| crates/kotowari-mds/tests/cli.rs | check_broken_frontmatter_stops_with_frontmatter_invalid | REQ-schema-009, REQ-schema-014, EX-schema-004, REQ-schema-042 | C-schema-cli-14, C-schema-cli-17 | 壊れた frontmatter の `check`。REQ-schema-014 の分は「スキーマの解決」の話題 |
| crates/kotowari-mds/tests/cli.rs | values_with_broken_frontmatter_stops_with_frontmatter_invalid | REQ-schema-009, REQ-schema-014 | C-schema-cli-14, C-schema-cli-17 | `values` の側。REQ-schema-014 の分は同上 |
| crates/kotowari-mds/tests/cli.rs | ast_schema_with_broken_frontmatter_stops_with_frontmatter_invalid | REQ-schema-009, REQ-schema-014 | C-schema-cli-14, C-schema-cli-17 | `ast --schema` の側。REQ-schema-014 の分は同上 |
| crates/kotowari-mds/tests/cli.rs | values_text_defaults_to_indented_text | REQ-schema-007 | C-schema-cli-6, C-schema-cli-7 | text の出力に、入れ子の字下げと配列の番号が付く。`--format` を省くと text になることは落とした細部 |
| crates/kotowari-mds/tests/cli.rs | values_text_indents_multiline_value_two_deeper_than_the_key | REQ-schema-007 | C-schema-cli-6 | text で出ること（`--format` を省いた形）。複数行の値の続きの行の字下げは落とした細部で、テストの中に残る |
| crates/kotowari-mds/tests/cli.rs | values_without_schema_stops_with_schema_not_found | REQ-schema-009, REQ-schema-014, REQ-schema-042 | C-schema-cli-14 | "$schema" の無い文書の `values`。分類は落とした細部。REQ-schema-014 の分は同上 |
| crates/kotowari-mds/tests/cli.rs | check_directory_reports_all_failing_documents | REQ-schema-010 | C-schema-cli-25 | "$schema" を持たない文書が結果に出ず、持つ文書の指摘だけが出る |
| crates/kotowari-mds/tests/cli.rs | check_directory_relaxes_a_schema_that_declares_open | REQ-schema-002, REQ-schema-010 | - | 確かめているのは、ディレクトリの検査でもスキーマの "open: true" が効くこと（closed-world の C-schema-closed-2）。対象の選び方は確かめていない |
| crates/kotowari-mds/tests/cli.rs | check_directory_skips_hidden_directories | REQ-schema-010, REQ-schema-044 | C-schema-cli-26 | ".hidden/" の下の文書を検査しない |
| crates/kotowari-mds/tests/cli.rs | check_directory_does_not_follow_symlinks | REQ-schema-010, REQ-schema-044 | C-schema-cli-28 | 置き場の外の違反のある文書へのシンボリックリンクがあっても、終了コード 0 で何も出さない。安全に関わる細部として戻した行 |
| crates/kotowari-mds/tests/cli.rs | stop_does_not_carry_the_referenced_file_contents | REQ-schema-043 | C-schema-cli-15, C-schema-cli-29 | 停止を1行で出すこと（C-15）と、参照先のファイルの2行目と3行目が標準エラーに出ないこと（C-29。安全に関わる細部として戻した行） |
| crates/kotowari-mds/tests/cli.rs | check_directory_stops_on_invalid_schema | REQ-schema-010, REQ-schema-009, REQ-schema-042 | C-schema-cli-16 | ディレクトリの検査で読めないスキーマに当たると全体が終了コード 2。YAML が読めないスキーマを `schema_invalid` に分けることは落とした細部（LLM の分類） |
| crates/kotowari-mds/tests/cli.rs | check_directory_stops_on_frontmatter_invalid_and_outputs_no_findings | REQ-schema-010, REQ-schema-009, REQ-schema-042, REQ-schema-043, REQ-schema-053 | C-schema-cli-15, C-schema-cli-16, C-schema-cli-17 | 空の "$schema" の文書で全体が停止し、指摘を出さず、`frontmatter_invalid` を出す |
| crates/kotowari-mds/tests/cli.rs | check_directory_stops_on_schema_not_found_and_outputs_no_findings | REQ-schema-010, REQ-schema-009, REQ-schema-042, REQ-schema-043 | C-schema-cli-15, C-schema-cli-16, C-schema-cli-17 | 参照先の無い文書で全体が停止し、指摘を出さず、`schema_not_found` を出す |
| crates/kotowari-mds/tests/cli.rs | check_directory_stops_on_unreadable_file | REQ-schema-010, REQ-schema-009, REQ-schema-042 | C-schema-cli-16, C-schema-cli-17 | 読めない文書で全体が停止し、`unreadable_file` を出す |
| crates/kotowari-mds/tests/cli.rs | check_fixtures_directory_passes | REQ-schema-010 | C-schema-cli-5 | 落とした細部のテスト。いちばん近い行を指す。確かめているのは、リポジトリの fixtures のディレクトリの検査が終了コード 0 で何も出さないこと。対象の選び方は区別していない |
| crates/kotowari-mds/tests/cli.rs | ir_fixture_passes_check_and_extracts_values | REQ-schema-010, REQ-schema-035 | C-schema-cli-5 | 落とした細部のテスト。いちばん近い行を指す。1つのファイルの `check` が終了コード 0 で終わることと、`values` が値を出すこと。ディレクトリの検査ではない |
| crates/kotowari-mds/tests/cli.rs | no_subcommand_stops_with_a_usage_hint_not_the_about_text | REQ-schema-009, REQ-schema-043 | C-schema-cli-5 | 終了コード 2 で終わること。使い方への案内を出すことは落とした細部で、テストの中に残る |
| crates/kotowari-mds/tests/cli.rs | check_json_finding_carries_the_node_name_and_the_raw_line | REQ-schema-008 | C-schema-cli-9, C-schema-cli-11, C-schema-cli-12 | 欠落の行、ノードの名前、生の行を JSON で |
| crates/kotowari-mds/tests/cli.rs | check_json_finding_line_points_at_the_node_or_at_the_one_that_should_contain_it | EX-schema-026 | C-schema-cli-9 | そのまま |
| crates/kotowari-mds/tests/cli.rs | check_json_finding_carries_the_declared_name_only_where_the_node_has_one | EX-schema-027 | C-schema-cli-11 | そのまま |
| crates/kotowari-mds/tests/cli.rs | check_json_invalid_id_finding_carries_the_heading_line_verbatim | EX-schema-028 | C-schema-cli-12 | そのまま |
| crates/kotowari-mds/tests/cli.rs | values_with_colliding_element_keys_stops_without_reporting_the_document | EX-schema-029 | C-schema-cli-18 | `schema_invalid` で停止し、文書の値を出さない |
| crates/kotowari-mds/tests/cli.rs | values_with_paths_sharing_only_a_level_above_the_dot_does_not_stop | EX-schema-030 | C-schema-cli-18, C-schema-cli-19 | "a.b" と "a.c" は停止せず、"a" と "a.b" は停止する |
| crates/kotowari-mds/tests/cli.rs | ex_schema_063_an_unknown_reading_word_names_the_field_and_the_accepted_words | EX-schema-063 | C-schema-cli-24 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_062_a_non_string_schema_value_names_the_field_and_the_expected_type | EX-schema-062 | C-schema-cli-24 | そのまま |
| crates/kotowari-mds/tests/cli.rs | req_schema_065_a_wrongly_typed_schema_value_names_the_field_and_the_expected_type | REQ-schema-065 | C-schema-cli-24 | "open"、"extract"、"extract.of" の型違い |
| crates/kotowari-mds/tests/cli.rs | req_schema_065_an_of_word_not_accepted_by_the_node_names_it_and_the_accepted_words | REQ-schema-065 | C-schema-cli-24 | ノードが受けない "of" の語 |
| crates/kotowari-mds/tests/cli.rs | ex_schema_037_table_rule_without_select_reports_a_mismatched_header_and_select_needs_header | EX-schema-037, TBL-schema-009 | C-schema-cli-17 | この話題の分は、"header" なしの "select" が `schema_invalid` で停止すること。ヘッダの食い違いの指摘は EX-schema-037（「見出しの下の行の規則」の話題）の分 |
| crates/kotowari-mds/tests/cli.rs | ex_schema_047_colliding_paths_outside_element_objects_stop | EX-schema-047 | C-schema-cli-18 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_048_paths_diverging_after_a_shared_level_outside_element_objects_do_not_stop | EX-schema-048 | C-schema-cli-19 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_073_a_root_path_named_type_in_a_named_schema_stops | EX-schema-073 | C-schema-cli-20 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_076_the_same_name_declared_twice_in_one_place_stops | EX-schema-076 | C-schema-cli-21 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_078_a_table_header_naming_a_column_twice_stops | EX-schema-078 | C-schema-cli-21 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_077_a_placement_path_with_an_empty_name_stops | REQ-schema-036, EX-schema-077 | C-schema-cli-22 | REQ-schema-036 の分は「抽出と素の構文木」の話題で付け替える |
| crates/kotowari-mds/tests/cli.rs | ex_schema_081_an_option_given_twice_stops | TBL-schema-009, EX-schema-081 | C-schema-cli-23 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_049_occurrence_finding_carries_the_counted_rule_kind | EX-schema-049 | C-schema-cli-13 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_064_values_text_indents_two_spaces_per_level | EX-schema-064, REQ-schema-066 | C-schema-cli-7 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_065_values_text_numbers_array_elements_from_one | EX-schema-065, REQ-schema-066 | C-schema-cli-7 | そのまま |
| crates/kotowari-mds/tests/cli.rs | req_schema_066_values_text_writes_a_derived_number_after_its_key | REQ-schema-066 | C-schema-cli-7 | 入れ子の字下げ。導かれる数の値を鍵のあとに書くことは落とした細部で、テストの中に残る |
| crates/kotowari-mds/tests/cli.rs | req_schema_066_values_text_indents_the_later_lines_of_an_array_element_one_level_deeper | REQ-schema-066 | C-schema-cli-7 | 配列の番号と字下げ。要素の2行目以降の字下げは落とした細部で、テストの中に残る |

## 落とした細部

### 安全に関わるもの

なし。前の版でここに置いた2件は、今のテストが確かめているので決定に戻した（下の「戻した細部」）。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-schema-044: 拡張子が ".md" でないファイルを辿らない | 実装者（A21）。決めたことでは「配下の文書」とだけ書いた |
| TBL-schema-009: "$schema" の無い文書を対象に指定したときの停止を `schema_not_found` に分けること、スキーマの YAML が読めないときと規則種別の形に反するときを `schema_invalid` に分けること | LLM（2026-10-23-next-release-bundle.md の A48。「承認の時に利用者に示す」）。停止すること自体は利用者の P1 と A56（mds-spec）で裏付けられるので、C-schema-cli-14 に書いた。"$schema" が空か空白だけのときの `frontmatter_invalid` は、利用者の A60（mds-spec）が「frontmatter が壊れているとして」と決めているので残した |
| REQ-schema-005: `ast` の出力が mdast に沿った JSON で、位置情報を含めないこと | 実装者（A19）。この話題では「素の構文木を JSON で」とだけ書いた。形は「抽出と素の構文木」の話題（REQ-schema-040） |
| テスト: `--version` の出力がコマンドの名前で始まること | 元の文書の外（テストの中だけ） |
| テスト: `check --format json` の出力が "files" の配列で、文書ごとに "findings" を持つ形 | 元の文書の外（テストの中だけ） |
| テスト: 指摘の JSON の鍵の名前（kind、severity、path、line、detail、node、text、rule_kind） | 元の文書の外（テストの中だけ）。決めたことには日本語の要素の名前で書いた |
| テスト: BOM を読み飛ばし、CRLF の行を1行と数えて行番号を決めること | 元の文書の外（テストの中だけ） |
| テスト: `values` の text 出力で、複数行の値の続きの行、配列の要素の2行目以降の字下げ、導かれる数の値の書き方 | 元の文書の外（テストの中だけ）。利用者が推奨を採用した A8（mutants-gaps）は、空白2つの字下げと 1 からの番号だけを決めている |
| REQ-schema-007（テストだけ）: `--format` を省いたときは `text` で出すこと | 元の文書と記録の外（A16 は2つから選ぶとだけ決めている）。テスト check_document_with_findings_exits_one_with_text、values_text_defaults_to_indented_text の中に残る |
| TBL-schema-009（テストだけ）: 要素の値の "value" の鍵と導かれる値の "of" の鍵にも、空の名前を許さないこと | 元の文書の外（テスト an_empty_name_in_the_value_or_an_of_key_of_an_element_object_is_schema_invalid のコメントだけ）。利用者の review4-gaps の A4 は配置パスだけを決めている |
| テスト: サブコマンドが無いときに、使い方への案内（"--help"）を出し、説明文をそのまま出さないこと | 元の文書の外（テストの中だけ） |
| TBL-schema-009 の "name" と "type" の衝突の停止の説明の文（`document has the key "<path>", which conflicts with "type", where typed extraction puts "name"`） | 利用者（2026-10-23-next-release-bundle.md の A5、推奨を採用）。停止の説明の文言を約束しないという利用者の決定（mutants-gaps の A6）とぶつかるので、文はコードに任せ、「型付きの抽出が "name" を置く鍵と衝突する、という言い方にする」とだけ書いた |

## 戻した細部

| 細部 | 新しい ID | 確かめているもの |
|---|---|---|
| REQ-schema-044: ディレクトリの検査で、シンボリックリンクを辿らない（記録では実装者の A21。why は「同じ文書を二重に読む経路と、置き場の外へ出る経路を作る」） | C-schema-cli-28（「ディレクトリの検査は、$schema を持つ文書だけを検査し、. で始まるディレクトリとシンボリックリンクを辿らない」） | テスト check_directory_does_not_follow_symlinks（置き場の外の違反のある文書への "link.md" を置いても、終了コード 0 で何も出ない） |
| REQ-schema-043（テストだけ）: 停止の説明に、"$schema" が指した参照先のファイルの中身を流さない | C-schema-cli-29（「検査を行えないときは、部分的な結果を出さずに停止する」） | テスト stop_does_not_carry_the_referenced_file_contents（標準エラーが1行で、参照先の2行目と3行目を含まない）。テストが確かめるのは2行目より後だけなので、決めたことと行は「1行目だけを出し、2行目より後を流さない」とし、中身を一切出さないとは書かなかった。問題の節は、テストのコメントにある「何を防ぐか」を書き、理由は記録に残っていないと書いた |

## 形に依存する要求

無し。どの要求も mds のエンジンと CLI の振る舞いで、古い仕様の形を扱わない。ただし、指摘にノードの名前・生の行・規則種別を持たせる決定（ir-engine の A27、A29、A86）は、記録の理由が「kotowari が IR の指摘を自分の種類へ写し分けるため」と書かれている。振る舞いは形に依存しないので「実装済み」で残し、理由は「呼ぶ側が詳細の文言や行を読み直さずに済む」という形で書いた。

## テストの無い行

無し。前の版の C-schema-cli-27（性質の行）は、違反のある文書で `values` が値を出して終了コード 0 で終わる行に書き直し、テスト ex_schema_080_a_later_title_ends_the_section_and_item_before_it（2つ目の題名で multiple_titles の違反がある文書に、`values` が終了コード 0 で値を出す）を付け替え先にした。付け替えは extraction の対応表のその行に書いた。テストは違反があることを自分では確かめていないので、同じスキーマと文書で `kotowari-mds check --format json` を実際に走らせ、multiple_titles の指摘が1件出て終了コード 1 になり、`values` は終了コード 0 で値を出すことを確かめた。性質（どんな文書でも抽出の結果が検査の合否で変わらない）を確かめる性質のテストは無いので、行にしなかった。

## 誰が決めたか見分けられなかったもの

- `ast` が `json` だけを受けること: 利用者の A44（mds-spec）は「"ast" に "--format text" を与えたときも引数の誤り」と決めている。`ast` の出力が JSON であることは A19（実装者）にあるが、A44 から裏付けられると読んだ。「JSON で出す」行は extraction の C-schema-extract-44 にある。
- 停止の理由の名前のうち `schema_not_found`、`schema_invalid`、`frontmatter_invalid` の綴り: 利用者の決定の中（mutants-gaps の A11、mds-spec の A48・A56、ir-engine の A56）に綴りが出るのは `schema_not_found` と `schema_invalid` で、`frontmatter_invalid` は LLM の A48（next-release-bundle）とテストにだけ綴りが出る。利用者から見える名前なので残した。

## 理由や見送った案が記録に無かった決定

- 「mds の CLI は kotowari-mds の名前で、…」: 受けるコマンドを4つにした理由が記録に無い（A26 の why は not recorded）。名前の理由と見送った案（versions-and-cli-name の A4）だけを書いた。
- 「ast --schema は、…」: 見送った案が記録に無い。
- 「終了コードは、…」: 見送った案が記録に無い。
- 「出力の形は、--format で…」: 見送った案が記録に無い。`ast` が `json` だけを受ける理由が記録に無い（A44 は not recorded）。
- 「values の text 出力は、…」: 見送った案が記録に無い。
- 「指摘は、種類、深刻度、…」: 見送った案が記録に無い。分類の表（A41）の理由は not recorded。
- 「出現回数の指摘に、…」: 見送った案が記録に無い。
- 「検査を行えないときは、…」: 見送った案が記録に無い。A42 の why は not recorded。
- 「停止の理由は5つにする」: 理由も見送った案も記録に無い。問題の節に「理由は残っていない」と書いた。
- 「配置パスが衝突するスキーマは、…」: 見送った案が記録に無い。
- 「配置パスの、ドットで区切った名前が空なら停止する」: 見送った案が記録に無い。
- 「同じオプションを2回渡すと、…」: 見送った案が記録に無い。
- 「停止の説明の文言は約束せず、…」: 見送った案（今の文言を約束にする）の採らなかった理由は why に書かれた中身だけで、問題の節に書いた。
- 「ディレクトリの検査は、…」: 理由も見送った案も記録に無い（A27、A60 は not recorded、kotowari-dir の A4 の why は名前の置き換えだけ）。問題の節に「理由は残っていない」と書いた。
- 「values と ast --schema は、検査の合否と関係なく…」: 理由も見送った案も記録に無い（A56 は not recorded）。

## 分けた所、まとめた所

- REQ-schema-008 を5つの決定に分けた: 基本の組（A17）、行（ir-engine の A21、A25）、ノードの名前（A27、A45）、生の行（A29、A61）、出現回数の規則種別（A86、ir-engine-gaps の A8）。それぞれ別の記録の理由と見送った案を持つため。undeclared_line の規則種別（A74）は REQ-schema-055 と同じ決定なので、closed-world の「宣言していない行の指摘に、その行をどう読んだかの種別を持たせる」に置き、この文書からはリンクで指した。
- TBL-schema-009 を、理由の表の決定と、5つの条件ごとの決定に分けた: 配置パスの衝突（ir-engine の A56・A65、ir-engine-gaps の A11・A19）、"name" と "type"（review2-gaps の A2、next-release-bundle の A5）、2度の宣言（review4-gaps の A3・A5）、空の名前（review4-gaps の A4）、2回のオプション（review7-gaps の A4）。"reading" と "select" の値の停止は、決める話題が「スキーマ言語の骨格」と「見出しの下の行の規則」なので、表の中で言葉で指した。
- REQ-schema-009、REQ-schema-043、REQ-schema-053 を1つの決定「検査を行えないときは、部分的な結果を出さずに停止する」にまとめた。どれも P1 と A42 から来ている。
- REQ-schema-010 と REQ-schema-044 を1つの決定にまとめた。どちらもディレクトリの検査の対象の選び方である。
- TBL-schema-001 は REQ-schema-006 の決定の中の表に、TBL-schema-002 は指摘の基本の組の決定の中の表にした。
- 値違いの場面（EX-082 と EX-083、EX-076 と EX-078、EX-062 と EX-063 と REQ-065 のテスト、EX-064 と EX-065）は、それぞれ1行にまとめた。

## あとでリンクにする所

無し。前の版で挙げた schema の5つの話題（スキーマ言語の骨格、見出しの下の行の規則、文書の骨格、抽出と素の構文木、スキーマの解決）への参照は、どれも見出しへのリンクにした（冒頭の段落、「停止の理由は5つにする」の "reading"・"select"・URL の取得の上限、「指摘は、種類、深刻度、…」の個々の指摘の種類、「mds の CLI は kotowari-mds の名前で、…」の素の構文木、「配置パスが衝突するスキーマは、…」の配置パスと "value"・"of" の鍵、「検査を行えないときは、…」の停止の条件）。

## 迷った判断と、どう決めたか

- REQ-schema-044 の出典は A21（実装者）だけだが、利用者が推奨を採用した kotowari-dir の A4 が「名前が "." で始まるディレクトリはもともと辿らない」「スキーマとキャッシュの置き場を辿らない決まりは ".kotowari/" に置き換わる」と、この規則を前提に決めている。"." で始まるディレクトリ（と ".kotowari/"）を辿らないことは利用者の決定で裏付けられると読んで残した。シンボリックリンクは、安全に関わる細部をテストが確かめているなら決定に戻す、という利用者の決定で C-28 に戻した。".md" 以外のファイルは安全の話ではないので落としたまま。見出しにシンボリックリンクを足したので、見出しの文字が変わった（この見出しへのリンクはほかの話題に無いことを確かめた）。
- next-release-bundle の A48（LLM）の4つの分類: 停止そのものは P1（利用者）が裏付けるので C-schema-cli-14 に書き、分類は表から外した。空の "$schema" の分類だけは A60（利用者）が裏付ける。
- 停止の説明に参照先のファイルの中身を出さないことは、記録にも IR にも無く、テストのコメントにだけある。安全に関わる細部をテストが確かめているので C-29 に戻した。今のコードは停止の説明の1行目だけを出すことで防いでいて、テストも2行目より後しか見ないので、「中身を一切出さない」とは書かなかった。
- `ast` に `--format text` を与えたときの停止は、「出力の形」の決定と「停止の理由」の決定の両方に関わる。確認することの行は、停止の理由の C-17 に置き、出力の形の決定からはリンクで指した。
- 指摘の形の決定の理由（ir-engine の A21、A27、A29、A74、A86）は、kotowari が IR の指摘を写し分けるという、古い仕様の形の事情で書かれている。kotowari の名前と IR の事情を出さず、「呼ぶ側」が詳細の文言や生の行を読み直さずに済む、という中身だけを書いた。A29 の見送った案の理由（A1 の例外が増えて「Markdown を読まない」が形だけになる）も、「呼ぶ側は Markdown を読まず、読むのは mds に任せる、という分担」と書いた。
- CLI のバイナリの名前の理由（他者が公開済みの名前とぶつかる）は、公開先の名前を出さず「Rust のクレートの公開先」と書いた。
- PROP-schema-002（A56）と library-extraction の C-schema-libext-6（values のコマンドの振る舞い）は同じ振る舞いなので、振る舞いが見える CLI の側のこの話題に一本化した（依頼で指定された置き場）。library-extraction の決定は、ライブラリの入口（extract_typed_partial）だけを残し、CLI の振る舞いはこの見出しへのリンクにした。C-libext-6 は欠番。
- 出現回数の指摘の規則種別（document-structure の REQ-schema-057、ir-engine の A86）は、この話題の C-13 と document-structure の C-schema-doc-19 の両方にテストがあった（ex_schema_049 と repeat_min_not_met_carries_the_counted_rule_kind）。「テストのある行の側に残す」では決まらないので、振る舞いが見える側として、指摘の形のほかの決定（ノードの名前、生の行）と JSON の rule_kind を確かめる CLI のテストがあるこの話題に残した。document-structure の C-doc-19 は欠番にし、そのテストの付け替え先を C-13 にした。C-13 の文は、節の文（validate のテスト）と項目の文（CLI のテスト）の両方を書いた。
- 配置パスの空の名前の停止は、この話題の C-22 と extraction の C-schema-extract-3 が同じ決定（review4-gaps の A4）で同じテスト（ex_schema_077）を指していた。停止の条件の決定として、見送った案も同じ形で並ぶこの話題に残し、extraction の C-extract-3 を欠番にしてリンクにした。
- C-schema-cli-4（"name" が `values` を変えない）は、ex_schema_082_083 が "name" を宣言したスキーマの `values` の値に "type" が無いことを確かめているので、テストのある行にした。"name" が `check` の結果を変えないことは、決めたことには書いたが、確かめるテストが無いので行にしていない。
- check_directory_relaxes_a_schema_that_declares_open は、この話題の分（REQ-schema-010）では何も確かめていないので「-」にし、closed-world の対応表で C-schema-closed-2 に付け替えた。

## mds のスキーマの言葉を広げる決定（提案）

出どころ: mds の形の追加の設計案（リポジトリには置いていない） の A1 の「Findings」（名前の規則の出現回数の指摘がノードの名前を持つ）。元の ID は無い。

### 置き換える決定

実装する側は、新しい決定を作ったときに、左の列の文と行を消すか直し、新しい決定の状態を「実装済み」にする。左の列の決定は、それまで実装済みのまま残す。

| 古い決定の見出しと、置き換わる部分 | 置き換える決定の見出しと行 | 理由 |
|---|---|---|
| 「指摘に、宣言したノードの名前を構造として持たせる」の決めたことの2文目（名前は節とフィールド行の指摘にだけ付ける） | 「名前の一覧で選ぶ節と項目の規則の指摘は、その一覧の最初の名前をノードの名前に持つ」（C-schema-cli-30、C-schema-cli-31） | A1。項目の指摘にもノードの名前が付く。C-schema-cli-11 は今の文のままでも成り立つ（題名と表は持たない）ので残る |
| 「指摘は、種類、深刻度、文書のパス、行番号、詳細を持つ」の分類の表 | 置き換えない | block-rules の「閉じないフェンスは、どのスキーマでも unclosed_codeblock の指摘にする」の新しい種類が、5つの分類のどれに入るかは決まっていない（block-rules の対応表の「実装する側が決める細部」）。決まったら、表を事実の書き直しで直す |

### 「提案」で書いた決定の一覧

| 決定 | 変えることになりそうなコード |
|---|---|
| 名前の一覧で選ぶ節と項目の規則の指摘は、その一覧の最初の名前をノードの名前に持つ | `crates/kotowari-markdown-schema/src/validate.rs` の `check_occurrence` のノードの名前（項目の指摘に名前を付けていない所） |

### 実装する側が決める細部

- 正規表現の規則の指摘の詳細に正規表現を入れる文言。
- "subheading" の出現回数の指摘が持つ規則種別の値（「出現回数の指摘に、数えたノードの規則種別を持たせる」）。

### 迷った判断

- 依頼は cli に新しい指摘の種類を表に足すことを挙げたが、この話題には種類の一覧の表が無く、分類の表しか無い。どの分類に入るかを設計が決めていないので、表は直さず、unclosed_codeblock の決定は block-rules に置いた。
- 設計の A1 は、名前の規則の出現回数の指摘がノードの名前を持つと書く。今の決めたことの「名前はその2つの指摘にだけ付け」と食い違うので、kotowari が欠けた節を見分けるのに要る振る舞いとして、提案の決定にした。ID の規則の指摘は、今と同じくノードの名前を持たないと書いた（設計は ID の規則を今の規則のままとする）。

## 数

- 足した提案: 決定 1、確認することの行 2（C-schema-cli-30、C-schema-cli-31）
- 元: 要求 13、場面 21、決定表 3、性質 1
- 読んだ記録のファイル: 11（2026-09-21-mds-spec.md、2026-09-22-ir-engine.md、2026-09-23-ir-engine-gaps.md、2026-09-23-mutants-gaps.md、2026-09-23-versions-and-cli-name.md、2026-09-24-ast-schema-output.md、2026-09-24-review2-gaps.md、2026-09-24-review4-gaps.md、2026-09-24-review7-gaps.md、2026-09-24-kotowari-dir.md、2026-10-23-next-release-bundle.md）
- 出来上がり: 決定 20、確認することの行 28（C-1、C-3〜C-29。C-2 は欠番。うち「（読んで確認）」0）
- テスト: 74
