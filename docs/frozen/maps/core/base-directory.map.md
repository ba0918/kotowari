# base-directory の対応表

元: `docs/ir/core/base-directory.md`（日本語は `base-directory.ja.md`）。書き直した文書: `base-directory.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-009 | C-core-base-1, C-core-base-2, C-core-base-3, C-core-base-4 | 相対パスの基準は、上に向かって探した最初の .kotowari/ のあるディレクトリにする | 決め方そのもの（algorithm）で、中身は TBL-core-003 と PROP-core-001。表と性質を同じ決定に入れた |
| REQ-core-010 | C-core-base-5, C-core-base-6 | 設定ファイルの値のパスと、出力の "path" は、基準のディレクトリからの相対にする | 設定の値を C-5、出力の "path" を C-6 にした。出典のパスの部分は形に依存するので落とした（下の「形に依存する要求」） |
| REQ-core-110 | C-core-base-6, C-core-base-7, C-core-base-8 | 設定ファイルの値のパスは、比べる前と出力の前に正規化する | 畳み方を C-7、"a/.." を C-8、出力の "path" の作り方を C-6 にした。出典のパスの正規化は形に依存するので落とした |
| TBL-core-003 | C-core-base-1, C-core-base-2, C-core-base-3 | 相対パスの基準は、上に向かって探した最初の .kotowari/ のあるディレクトリにする | 表は「決めたこと」の Markdown の表にした。順1を C-1、順2を C-2、".kotowari" のファイルを無視することを C-3 |
| PROP-core-001 | C-core-base-4 | 相対パスの基準は、上に向かって探した最初の .kotowari/ のあるディレクトリにする | 性質を確認することの行にした |
| EX-core-289 | C-core-base-8 | 設定ファイルの値のパスは、比べる前と出力の前に正規化する | 場面は判断の記録の置き場と source_invalid で書かれていて形に依存する。形に依存しない「置き場の "a/.." を畳んでから比べる」を C-8 にした |
| EX-core-002 | C-core-base-1 | 相対パスの基準は、上に向かって探した最初の .kotowari/ のあるディレクトリにする | そのまま |

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| tests/step1_config.rs | req_009_base_is_the_dir_holding_dot_kotowari | REQ-core-009, TBL-core-003 | C-core-base-1 | 下のディレクトリから実行して、上の ".kotowari/" の設定で通ることを確かめる |
| tests/step1_config.rs | req_009_falls_back_to_cwd | REQ-core-009, TBL-core-003 | C-core-base-2 | そのまま |
| tests/step1_config.rs | req_009_base_from_a_subdirectory_is_the_ancestor_with_dot_kotowari | REQ-core-009, TBL-core-003, EX-core-002 | C-core-base-1 | 指摘の "path" が上のディレクトリからの相対になることまで確かめる |
| tests/step1_config.rs | req_010_config_values_are_relative_to_base | REQ-core-010 | C-core-base-5 | そのまま |
| tests/step1_config.rs | prop_001_config_path_does_not_move_the_base | PROP-core-001 | C-core-base-4 | そのまま |
| tests/step1_config.rs | tbl_003_kotowari_file_is_ignored_in_search | TBL-core-003 | C-core-base-3 | そのまま |
| tests/step1_config.rs | req_110_trailing_slash_in_config_is_normalized_in_path | REQ-core-110 | C-core-base-7 | そのまま |
| tests/step1_config.rs | req_110_dot_segments_are_folded | REQ-core-110 | C-core-base-7 | そのまま |
| tests/step1_config.rs | req_110_dot_alone_normalizes_to_empty_place | REQ-core-110 | C-core-base-7 | "." だけの置き場が空になることは落とした細部で、テストの中に残る |
| tests/step1_config.rs | req_110_ir_dot_produces_bare_filename_path | REQ-core-110 | C-core-base-6 | 置き場が "." のときの "path" の作り方。"./a.md" でなく "a.md" になる細部はテストの中に残る |
| tests/step1_config.rs | req_110_path_carries_the_subdirectory | REQ-core-110, TBL-core-006 | C-core-base-6, C-core-base-7 | 置き場の値 "./docs//ir/./" の正規化（C-7）と、正規化した置き場から作る "path"（C-6）。サブディレクトリの階層を含むことは落とした細部（いちばん近い行 C-6 を指す）。TBL-core-006 は「出力の形」の話題の分 |
| tests/step4_test_discovery.rs | req_033_requirement_without_test_path_carries_the_subdirectory | REQ-core-033, REQ-core-085, REQ-core-110, TBL-core-006 | C-core-base-6 | 落とした細部のテスト。いちばん近い行を指す。この話題の分はサブディレクトリの階層を含む "path"。ほかの ID は別の話題の分。入力が IR の要求で、新しい形では入力を書き直す |
| tests/step3_sources_terms.rs | req_058_records_place_dot_resolves_a_source_at_the_base_root | REQ-core-058, REQ-core-110 | - | 古い形のコードと一緒に消す。判断の記録の置き場 "." と出典の解決で確かめていて、どちらも新しい形で無くなる。"." が空になる細部は C-7 のテストに残る |
| tests/step3_sources_terms.rs | req_110_a_parent_step_in_a_place_is_folded | REQ-core-058, REQ-core-110, EX-core-289 | C-core-base-8 | 置き場の "a/.." を畳むことを確かめる。入力が判断の記録の置き場と出典で、古い形のコードと一緒に消える。C-8 は新しい形のテストが要る（下の「テストの無い行」） |
| tests/step8_record_form.rs | req_132_parent_directory_href_to_superseded_number_passes | REQ-core-132, TBL-core-023, REQ-core-110, EX-core-116 | - | 古い形のコードと一緒に消す。確かめているのは改めのリンクの href の ".." の解決で、設定の値のパスの正規化ではない。改めのリンクは新しい形で無くなる |
| tests/step8_record_form.rs | req_132_a_parent_step_in_the_records_place_is_folded | REQ-core-132, REQ-core-110, EX-core-289 | C-core-base-8 | 置き場の "a/.." を畳むことを確かめる。入力が判断の記録と改めのリンクで、形が無くなるとテストも無くなる |

ガイドのマーカー: `docs/guides/cli.md`（と `.ja.md`）の REQ-core-009、TBL-core-003、PROP-core-001、REQ-core-010、REQ-core-110 と、`docs/guides/config.md`（と `.ja.md`）の同じ ID は、上の「決定の見出し」で付け替える。`docs/guides/commands/plan.md` の REQ-core-010 は計画書の例の文字で、マーカーではない。

## 落とした細部

安全に関わるもの: なし（`safety-dropped.md` にこの話題の行は無い）。

## 戻した細部

無い。

## 落とした細部（そのほか）

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-110: "." か "./" だけの置き場は空の文字列に正規化し、"path" を文書名だけにする | 記録に無い（テストの中だけにある） |
| REQ-core-110: 出力の "path" を置き場からの相対パスと "/" でつなぎ、置き場の下のサブディレクトリの階層も "path" に入れる（REQ-core-110 の文言の改め） | 2026-09-16-ir-tree.md の A13（「A1〜A8 から導いたこと（第2ラウンドで提示し異論なし）」）。LLM が導いてラウンドで示したものなので、agent の細部として落とした。決めたことと C-core-base-6 は、利用者が採った A106 の「正規化した置き場と文書名を "/" でつなぐ」の範囲で書いた |

## 形に依存する要求

| 元の ID | 扱い |
|---|---|
| REQ-core-010 | 「出典のパスは基準のディレクトリからの相対」の部分は消える。出典の行は新しい形で無くなる。設定の値と出力の "path" の部分は残る |
| REQ-core-110 | 「出典のパスを正規化する」の部分は消える。設定の値のパスと出力の "path" の部分は残る |
| EX-core-289 | 新しい形で読み替えて残る。判断の記録の置き場 "decisions.records" と source_invalid の代わりに、決定の文書の置き場など、新しい形の設定の置き場の値に "a/.." を書いて、その中の文書が置き場の中として読まれることを確かめる |

## テストの無い行

- C-core-base-8: 今のテスト（`req_110_a_parent_step_in_a_place_is_folded`、`req_132_a_parent_step_in_the_records_place_is_folded`）は、判断の記録の置き場と出典・改めのリンクで確かめていて、古い形と一緒に無くなる。要るテストは、`kotowari_core::normalize_path` に "docs/x/../ir" を渡して "docs/ir" が返り、"../x" を渡すと ".." が残ることを確かめる単体のテストと、新しい形の置き場の値に "a/.." を書いても、その中の文書の "path" が畳んだ置き場から作られることを確かめる結合のテスト。

## 誰が決めたか見分けられなかったもの

- A37（records.md の第6ラウンド）と A106、A124（第9ラウンド）は、決定の行ごとの decided_by が無く、ラウンドの見出しに「利用者の答え「推奨で」」「すべて推奨をそのまま採った（利用者の答えは「進めて」）」とある。利用者が推奨の一覧を見て採ったものとして、利用者の決定に数えた。

## 理由や見送った案が記録に無かった決定

- 「相対パスの基準は、上に向かって探した最初の .kotowari/ のあるディレクトリにする」: 理由も見送った案も記録に無い（A37、A124）。問題の節に「理由は残っていない」と書いた。
- 「設定ファイルの値のパスと、出力の "path" は、基準のディレクトリからの相対にする」: 理由も見送った案も記録に無い（A37、A13）。
- 「設定ファイルの値のパスは、比べる前と出力の前に正規化する」: 見送った案が記録に無い。理由は review7-gaps の A1 の why だけで、A106 の畳み方そのものの理由は無い。

## 分けた所、まとめた所

- REQ-core-009、TBL-core-003、PROP-core-001 を1つの決定にまとめた。どれも A37 の基準の決め方で、"--config" が基準を変えないことも A37 の同じ文にある。
- A37 の「設定の値、出典、出力の path は基準からの相対」は、基準の決め方と別の決定（REQ-core-010）にした。
- REQ-core-110 の「出力の "path" を置き場と相対パスでつないで作る」は、正規化の決定でなく、相対パスの決定に入れた（C-6）。"path" の作り方で、正規化そのものではないため。

## あとでリンクにする所

残っていない。「出力の形」の話題（output）の "path" の欄は、もうこの話題の「設定ファイルの値のパスと、出力の "path" は…」へリンクしている。

ここで済ませたもの: 冒頭の段落の config.ja.md（話題の文書へ）と cli-environment.ja.md の対象の OS の決定（下書きどうしも `.ja.md`）。

## 利用者に聞くこと

無い。

## 迷った判断と、どう決めたか

- 記録の A37 は第6ラウンドで、利用者の答えは「推奨で」だけである。細部を任せた承認ではなく、示された推奨の一覧を採った答えなので、利用者の決定として残した。第9ラウンド（A106、A124）も同じに扱った。
- ir-tree の A13 は LLM が導いてラウンドで示し、異論が無かったものである。確定の決まり（ラウンドで利用者に示した LLM の決定は agent の細部）に合わせ、"path" をサブディレクトリの階層を含めて作ることは落とした。決めたことと C-core-base-6 からその部分を外した。テストはいちばん近い行 C-6 を指したままにする。
- C-core-base-8 は、今のテストが形に依存し、切り替えのあとに無くなる。行を消すと、利用者が採った review7-gaps の A1（"a/.." を畳む）が決定から消えるので、行は残して「テストの無い行」に挙げた。
- 対象の OS（Windows では "\\" の正規化だけ）は「使い方の表示と対象の環境」の決定に書き、この話題の正規化の決定には "\\" を "/" に直すことだけを書いた。

## 数

- 元: 要求 3、場面 2、決定表 1、性質 1
- 読んだ記録のファイル: 3（records.md、2026-09-16-ir-tree.md、2026-09-24-review7-gaps.md）
- 出来上がり: 決定 3（実装済み 3）、確認することの行 8（うち「（読んで確認）」0）
- 確定で: 戻した細部 0、直した「など」0。C-core-base-6 を、A13 の細部を外して書き直した
- テスト: 16
