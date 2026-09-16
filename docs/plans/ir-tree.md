# 実装計画: ir-tree

## Goal

`kotowari check` が IR の置き場をディレクトリの深さに制限なく読み、用語集と問題の記録をどのディレクトリにも置け、サブディレクトリの文書を文書名の参照で指せ、ID の数字が3桁を超えても通るようにする。

## Specification

正本は仕様IR `docs/ir/*.md`（以下「IR」）。この計画が対象にする項目は次のとおり。

- `docs/ir/ir-document.md#REQ-033`（読む文書）
- `docs/ir/config.md#REQ-018`、`docs/ir/cli.md#TBL-001`（置き場の下のディレクトリが読めないときの停止）
- `docs/ir/base-directory.md#REQ-110`、`docs/ir/output.md#TBL-006`（出力の path）
- `docs/ir/findings.md#TBL-008`（missing_document の detail、missing_title などの detail）、`docs/ir/findings.md#REQ-032`（duplicate_id の1つ目）
- `docs/ir/terms.md#REQ-064`、`#REQ-065`、`#REQ-069`、`#REQ-070`、`#TBL-014`（用語の見え方、文書名の参照）
- `docs/ir/terms-form.md#REQ-117`、`#REQ-123`（用語集の壊れと重複）
- `docs/ir/finding-order.md#TBL-019`（duplicate_term の行）
- `docs/ir/ir-references.md#REQ-124`（ID の形）、`#REQ-114`、`docs/ir/ir-items.md#REQ-043`（形に合わない ID の扱い）
- `docs/ir/CONTEXT.md` の用語 `連鎖`、`用語集`、`問題の記録`、`除外`、`ID`

判断の記録は `docs/decision/brainstorm/2026-09-16-ir-tree.md`（A1〜A20）。IR の形は `docs/decision/brainstorm/ir-form.md`（以下「契約」）。

IR の読み方は前の計画と同じ。要求の `- 種類:` が `prohibition` の文は起きてはならないこと、`algorithm` の中身は `- 定義:` の指す決定表と性質にある、`- 検証:` が `review` の要求はテストでなく読む場所を `docs/trace.md` に書く。IR の各文書の `## 具体例` のシナリオは、その要求の成功の条件と反例で、テストの入力と期待の元にする。

IR と契約は実装の間は読み取り専用である。IR の書き換えが必要だと思ったら、書き換えずに止まって返す。IR の沈黙は実装者が決めてよい意味ではない（`docs/ir/cli-environment.md#REQ-120`）。

この計画の中のテストの置き方と名前の付け方は、IR ではなくこの計画の約束である。

## Approach and why

既存の実装（`src/` 約3,800行、テスト360件）を育てる。作り直さない。

変更は4つの独立した性格に分かれ、1つ目が残りの土台になる。

1. **走査を再帰にし、文書が置き場からの相対パスを持つ**: いま IR の読み取り口（`src/ir.rs` の `load_and_check`）は置き場の直下を1回 `read_dir` するだけで、文書は `filename`（素のファイル名）しか持たない。判断の記録の置き場を読む `src/sources.rs` の `load_all_md` には、再帰・隠しディレクトリの除外・ディレクトリのシンボリックリンクを辿らない・先の無いリンクで停止、がすでにある。IR 側も同じ性質を持たせ、文書に「置き場からの相対パス」と「ディレクトリ」を持たせる。出力の path、読む順、用語集の連鎖、参照の解決はすべてこの相対パスから決まる
2. **用語集を1つから連鎖にする**: いま `collect_glossary_terms` は全文書の `CONTEXT.md` を1つの集合に合流させ、`run_check` がそれを全文書に渡す。これを「文書ごとに、そのディレクトリから根までの `CONTEXT.md` の合計」に変える。duplicate_term はいま `parse_document` の中で1文書のローカルな集合で出しているが、連鎖をまたぐ重複は全文書を読んだ後でしか分からないので、判定を `check_documents` の側（全文書が見える場所）へ移す
3. **文書名の参照の文法を広げる**: `find_doc_refs` は手書きの走査で、".md" の前を英小文字・数字・ハイフンだけ逆に辿るため "/" で止まる。要素を "/" で区切る文法に書き換え、存在の判定を「素のファイル名の集合」から「相対パスの集合と、参照を書いた文書のディレクトリ」に変える。".md" の直後に "#" と "/" を禁じることは同時に入れる。"/" を含む参照を拾えるようにする変更だけを先に入れると、出典の行（"docs/decision/brainstorm/records.md#A12"）が参照として拾われて自己適用が壊れる
4. **ID の桁**: `is_valid_id` の `suffix.len() == 3` を「3桁以上、4桁以上は先頭が "0" でない」に変える。見出し・"@id"・印・既知の ID の集合・バッククォートの中の ID はすべてこの1つの関数を通っているので、変更は1か所

読む順は 1 → 2 と 3（互いに独立）→ 5（自己適用）→ 6（対応の表）。4 はどこにも依存しないので、1 の前でも後でもよい。

再利用の判断: 新しい依存は足さない。再帰の走査は、`walkdir`（すでに依存にあり、テストの走査 `src/tests_discovery.rs` が使っている）か、`src/sources.rs` の `load_all_md` と同じ手書きの走査のどちらかで書く。どちらでも承認された振る舞いは同じなので、実装者に任せる（下の Left to the implementer）。

テストの置き方と名前（この計画の約束。前の計画と同じ）:

- テストは `tests/` に置き、ライブラリの公開する関数と型と CLI だけを通す。ファイルは既存の `tests/step2_ir.rs`（文書の読み込み、ID）、`tests/step3_sources_terms.rs`（用語、参照）、`tests/step1_config.rs`（停止、path）に足す。ファイル名の `stepN` は前の計画の番号で、この計画では変えない
- 各テスト関数の直前の行に `// @kotowari[REQ-033, TBL-014]` の印を置く
- テスト関数の名前は、確かめる IR の ID を先頭に付けた snake_case（例: `req_033_subdirectories_are_read`）
- テストは RED → GREEN → REFACTOR の順で書き、各段で `CARGO_BUILD_JOBS=4 cargo test` を走らせる
- 既存のテストが新しい仕様と食い違うとき（例: `req_033_only_direct_children` はサブディレクトリの文書が読まれないことを確かめている）は、新しい仕様に合わせて書き直し、コミットの本文に「A1 で改めた」のように改めた決定を書く。書き直す前に、そのテストが守っていたほかの振る舞い（そのテストの ほかの assert）が別のテストに残るかを確かめる
- コミットは意図ごとに分け、`git add <path>` でファイルを1つずつ。件名は `<type>: <件名>` の形で日本語

## Scope of change

変えてよいのは、リポジトリ直下の次のものだけである。

- `src/` の下のすべて
- `tests/` の下のすべて
- `docs/trace.md`（ステップ6）
- `Cargo.lock`（依存の更新はしない。ビルドで変わった分だけ）

`Cargo.toml` は変えない（依存を足さない）。`docs/ir/`、`docs/decision/`、`docs/plans/`、`skills/`、リポジトリ直下の `.kotowari/`、`experiments/`、`fixtures/` は変えない。

## Step order and prerequisites

ステップ1を最初に行う。ステップ2と3はどちらもステップ1の後で、互いに依存しないので順は問わない。ステップ4はどこにも依存しない。ステップ5は1〜4のすべての後、ステップ6はステップ5の後に行う。

## Step 1 — 走査を再帰にし、文書に置き場からの相対パスを持たせる

Purpose: IR の置き場の下をディレクトリの深さに制限なく読み、除外と停止をどの深さでも同じに働かせ、どのディレクトリでも "CONTEXT.md" と "FLAGS.md" を用語集と問題の記録にし、出力の path と読む順を置き場からの相対パスで決める。Specification: `docs/ir/ir-document.md#REQ-033`、`docs/ir/config.md#REQ-018`、`docs/ir/cli.md#TBL-001`、`docs/ir/cli-environment.md#TBL-020`（停止の詳細の形）、`docs/ir/base-directory.md#REQ-110`、`docs/ir/output.md#TBL-006`、`docs/ir/findings.md#TBL-008`（missing_title、missing_scope、glossary_invalid の detail はディレクトリを除いたファイル名）、`docs/ir/findings.md#REQ-032`（duplicate_id の1つ目はパスのバイト順）、`docs/ir/CONTEXT.md`（`除外`、`用語集`、`問題の記録`）。
Prerequisites: なし。始める前に `CARGO_BUILD_JOBS=4 cargo test` を1回走らせて全件通ることと、`CARGO_BUILD_JOBS=4 cargo run -q -- check` の出力を記録する（ステップ5でこの記録と比べる）。
May change: `src/ir.rs`、`src/lib.rs`、`src/terms.rs`、`src/sources.rs`、`src/tests_discovery.rs`（文書の相対パスに追従する分だけ）、`tests/step2_ir.rs`、`tests/step1_config.rs`、`tests/step5_findings.rs`、`tests/step6_output.rs`（path の形に追従する分だけ）。
Done when: 置き場の下の任意の深さの ".md" が読まれ、`files` と `lines` に数えられ、その指摘の path が「正規化した置き場」+ "/" + 「置き場からの相対パス」になる。サブディレクトリの中でも、".MD" とディレクトリでも通常のファイルでもないものは読まず指摘も出ず、ファイルのシンボリックリンクは読む。置き場の直下でもサブディレクトリの中でも、隠しディレクトリとディレクトリのシンボリックリンクは辿らず、指摘も停止も出ない（直下の分はいまディレクトリを全部落とす判定で結果的に除外されていて、テストが無い。再帰を入れると分岐が変わるので、直下と深い所の両方を同じテストで固定する）。サブディレクトリの中の先の無いシンボリックリンクと、種類を取れない要素は、読めないファイルを理由に停止する。置き場の下のディレクトリが読めないときは、そのディレクトリの基準からの相対パスと OS の誤りの文を詳細にして停止する（判断の記録の置き場と ADR の置き場の下のディレクトリが読めないときの詳細は、いま設定の値と OS の誤りの文で、この計画では変えない。IR の側だけ、読めなかったディレクトリの相対パスにする）。空のディレクトリは何も起こさない。サブディレクトリの "CONTEXT.md" は用語集（範囲の行が無くても missing_scope を出さず、表が無ければ glossary_invalid）、"FLAGS.md" は問題の記録として読まれる。missing_title、missing_scope、glossary_invalid の detail はサブディレクトリの文書でもディレクトリを除いたファイル名。同じ ID が "a.md" と "a/b.md" にあるとき、duplicate_id はバイト順で後の "a/b.md" の側に出る（"." は "/" より小さい）。既存の `req_033_only_direct_children` は、サブディレクトリの文書が読まれることを確かめるテストに書き直す。
Shown by: test — `req_033_subdirectories_are_read_at_any_depth`（`req_033_only_direct_children` を置き換える。空のディレクトリも含める）、`req_033_hidden_dir_and_dir_symlink_are_not_followed_at_any_depth`、`req_033_broken_symlink_in_a_subdirectory_stops`、`req_033_context_and_flags_in_a_subdirectory_are_glossary_and_flags`、`req_018_unreadable_ir_subdirectory_stops`、`req_110_path_carries_the_subdirectory`、`tbl_008_whole_document_detail_is_the_bare_filename_in_a_subdirectory`、`req_032_first_occurrence_is_bytewise_first_relative_path`。
Left to the implementer: 走査の書き方（`walkdir` の `WalkDir::new(..).follow_links(false)` か、`src/sources.rs` の `load_all_md` と同じ手書きの再帰）。文書の相対パスとディレクトリを持つ欄の名前と型（`IrDocument.filename` は detail に使うので残す）。path を組む `join_display_path` に `doc.filename` を渡している呼び出し（`rg -n join_display_path src/` で全部出る。`src/ir.rs`、`src/terms.rs`、`src/sources.rs`、`src/tests_discovery.rs`）を相対パスに変える方法（欄を1つ足して全部そこを読む形を勧める）。
Stop and hand back if: ディレクトリのシンボリックリンクを辿らないために `file_type()`（リンクを辿らない）を先に見る必要があるが、既存の `req_033_file_symlink_is_read` と `req_033_broken_symlink_in_ir_dir_stops` のどちらかが、その順で通せない。置き場そのものが読めないときの詳細は今までどおり設定の値（既存の `req_005_stderr_carries_the_stop_reason_text` が1行目に "docs/ir" を含むことを見ている）、その下のディレクトリは相対パス、で書き分けるが、それ以外の書き分けが要るなら止まる。

## Step 2 — 用語集を連鎖にする

Purpose: 文書から見える用語を、その文書のディレクトリから置き場の根までの各 "CONTEXT.md" の合計にし、連鎖に用語集が無い文書では囲んだ語を ID を除いてすべて unknown_term にし、連鎖の中の重複を根から遠い側の行に duplicate_term で出し、壊れた用語集はその用語集の分だけ0語にする。Specification: `docs/ir/terms.md#REQ-064`、`#REQ-065`、`docs/ir/terms-form.md#REQ-117`、`#REQ-123`、`docs/ir/finding-order.md#TBL-019`（duplicate_term の行）、`docs/ir/CONTEXT.md`（`連鎖`、`用語`）。
Prerequisites: ステップ1（文書のディレクトリ）。
May change: `src/terms.rs`、`src/ir.rs`（連鎖をまたぐ duplicate_term の判定を全文書が見える場所に足す）、`src/lib.rs`、`src/sources.rs`（連鎖の重複の行を出典の検査から外す分だけ）、`tests/step3_sources_terms.rs`、`tests/step2_ir.rs`。
Done when: 根の "CONTEXT.md" の用語はどの深さの文書からも見え、"network/CONTEXT.md" の用語は "network/" の下の文書からだけ見え、"network/dns/" の文書から "network/publish/CONTEXT.md" の用語を囲むと unknown_term になる。根に用語集が無く "network/CONTEXT.md" だけあるとき、根の文書は囲んだ語がすべて（ID を除いて）unknown_term になり、"network/" の文書はならない。根と "network/CONTEXT.md" の両方に同じ用語があると、"network/CONTEXT.md" のその行に duplicate_term が1件出て、根には出ず、"network/" の文書でその用語を囲んでも unknown_term は出ない。同じ用語集の中の重複は今までどおり `parse_document` の中で2つ目以降の行に出て、その行は用語にならず出典の検査も受けない（既存の `req_123_duplicate_row_is_not_a_term` を変えない）。連鎖をまたぐ重複の行も同じく用語にならず、出典の検査（missing_source、source_invalid）を受けない。"network/CONTEXT.md" が glossary_invalid のとき、"network/" の文書から根の用語は見える。
Shown by: test — `req_064_term_from_a_parent_glossary_is_visible`、`req_064_term_from_a_sibling_glossary_is_unknown`、`req_065_document_with_no_glossary_in_its_chain_flags_every_backtick`、`req_123_duplicate_across_the_chain_is_reported_on_the_deeper_row`、`req_123_duplicated_term_stays_visible_below`、`req_117_invalid_glossary_hides_only_its_own_terms`。
Left to the implementer: 連鎖を表す型（ディレクトリごとの用語の集合の表を作って文書ごとに合成するか、文書ごとに毎回合成するか）。連鎖をまたぐ重複の行を用語から外す方法（`parse_document` の後で項目から取り除くか、印を付けて検査の側で飛ばすか）。
Stop and hand back if: 既存の `req_123_duplicate_row_is_not_a_term` か `req_123_duplicate_term_reported_for_second_row_onward` が、連鎖の判定を足した後に通せない。

## Step 3 — 文書名の参照の2形

Purpose: 文書名の参照を「要素を "/" で区切った並び」として拾い、素の名前は同じディレクトリ、"/" を含む名前は置き場からの相対で解決し、"." と ".." の要素は解決せず、".md" の直後の "#" と "/" を参照でなくし、detail を参照の文字列にする。Specification: `docs/ir/terms.md#REQ-069`、`#REQ-070`、`#TBL-014`、`docs/ir/findings.md#TBL-008`（missing_document の detail）。
Prerequisites: ステップ1（文書の相対パスとディレクトリ、読んだ文書の集合）。
May change: `src/terms.rs`、`src/lib.rs`（読んだ文書の集合を素のファイル名から相対パスに変える）、`tests/step3_sources_terms.rs`。
Done when: "network/dns/a.md" に "b.md" と書いて "network/b.md" しか無ければ missing_document（上へ辿らない）、"network/dns/b.md" があれば出ない。"network/dns/a.md" に "network/publish/c.md" と書いて "network/publish/c.md" があれば出ない。"../b.md" と "./c.md" は先があっても missing_document。"a.md/b.md" は参照にならない。"docs/decision/brainstorm/records.md#A12" と引用符なしで書いても参照にならない。"docs/decision/adr/0001-test-marker.md" と引用符なしで書くと、その文字列全体を detail にして missing_document（既存の `req_069_reference_needs_boundary_and_quotes_are_skipped` は "adr/0001-test-marker.md" が参照にならないことを、"0001-test-marker.md" の missing_document が無いことで見ている。新しい文法ではこの並びが参照になるので、detail が "adr/0001-test-marker.md" の missing_document が1件出ることを確かめる形に書き直す。今の assert は新しい文法でも真のまま通るので、書き直さないと何も守らない）。ディレクトリのシンボリックリンクの下にある文書への "/" を含む参照は missing_document。detail は一致した並びの全体。全角の "／" は句読点でないので境界にならず、"a//b.md" は要素が空なので参照にならない（どちらも TBL-014 で決まる。指摘は出ない）。
Shown by: test — `tbl_014_slash_separated_path_is_a_reference`、`tbl_014_md_followed_by_hash_or_slash_is_not_a_reference`、`req_070_bare_name_resolves_in_the_same_directory_only`、`req_070_slash_path_resolves_from_the_ir_root`、`req_070_dot_and_dotdot_elements_never_resolve`、`req_070_detail_is_the_whole_reference`、`req_070_document_under_a_directory_symlink_is_missing`。
Left to the implementer: `find_doc_refs` を書き直すか、要素の文法を別の純粋な関数に切り出して呼ぶか。読んだ文書の集合の型（相対パスの `BTreeSet<String>`）。
Stop and hand back if: TBL-014 の条件2と条件3で参照になるかならないかが決まらない並びに出会った。IR に書かれていない並びを参照にするかしないかは決めずに止まる。

## Step 4 — ID の桁

Purpose: ID の数字を「3桁以上、4桁以上は先頭が "0" でない」にする。Specification: `docs/ir/ir-references.md#REQ-124`、`#REQ-114`（"@id" が形に合わないとき）、`docs/ir/ir-items.md#REQ-043`（見出しが形に合わないとき）。
Prerequisites: なし。
May change: `src/ir.rs`（`is_valid_id` とその doc コメント）、`tests/step2_ir.rs`、`tests/step4_test_discovery.rs`。
Done when: "### REQ-1000: 名前" は要求として読まれ、"### REQ-0001: 名前" と "### REQ-1: 名前" は unknown_heading。"@id=EX-1000" はシナリオの ID になり、"@id=EX-0001" は invalid_id。テストの印 "@kotowari[REQ-1000]" は要求 REQ-1000 と照合される（requirement_without_test が出ない）。3桁の ID の振る舞いは変わらない。
Shown by: test — `req_124_four_digit_id_is_valid_in_heading_tag_and_marker`、`req_124_leading_zero_and_short_ids_are_rejected`。
Left to the implementer: none。
Stop and hand back if: `is_valid_id` 以外に桁数を数えている場所が見つかった（調査では `src/ir.rs` の1か所だけ）。

## Step 5 — 自己適用

Purpose: kotowari 自身の IR とテストにかけて誤り0にする。Specification: `docs/ir/coverage.md#REQ-085`（テストのない要求）、`docs/ir/ir-document.md#REQ-038`（行数の警告は残ってよい）。
Prerequisites: ステップ1〜4。
May change: `tests/`（自己適用で足りない印があれば足す）。`src/` は変えない（変えないと通らないなら、それは前のステップの取りこぼしなので、そのステップに戻る）。
Done when: `CARGO_BUILD_JOBS=4 cargo run -q -- check` をリポジトリ直下で走らせて、`findings` に severity が "error" のものが無く、終了コード0。警告（too_many_lines が `docs/ir/ir-document.md` と `docs/ir/terms.md` に1件ずつ）は残ってよい。
Shown by: check — `CARGO_BUILD_JOBS=4 cargo test` が 0 failed、`CARGO_BUILD_JOBS=4 cargo run -q -- check` の出力の `counts` に "requirement_without_test" と "unresolved_reference" が無く、終了コード0。
Left to the implementer: none。
Stop and hand back if: 自己適用で IR の側の不備（新しい規則で IR 自身が引っかかる、たとえば新しい参照の文法で拾われる並びが IR の地の文にある）が出た。IR を直さずに止まる。

## Step 6 — 対応の表

Purpose: 要求とテストの対応の表を、この計画で足したテストと書き直したテストに合わせる。Specification: `docs/ir/coverage.md#REQ-085`。
Prerequisites: ステップ5。
May change: `docs/trace.md`（「要求の確かめ方」の表だけ。「ミューテーションテスト」以降の節は変えない）。
Done when: 表に REQ-124 の行があり、REQ-033、REQ-018、REQ-032、REQ-064、REQ-065、REQ-069、REQ-070、REQ-110、REQ-117、REQ-123 の行が、この計画で足したテストと書き直したテストの名前を持ち、置き換えたテストの古い名前が残っていない。
Shown by: artifact — `docs/trace.md`。`grep -c '^| REQ-' docs/trace.md` が 123 で、`rg -c '^### REQ-' docs/ir/*.md | awk -F: '{s+=$2} END {print s}'` の 123 と同じ。`rg -c 'req_033_only_direct_children' docs/trace.md` が 0。
Left to the implementer: 表の行の書き方は既存の行にそろえる。生成の道具（`experiments/003-cli/tools/gen_trace2.py`）はミューテーションの結果を引数に取るので使わず、手で更新してよい。
Stop and hand back if: none（このステップに固有の条件は無い）。

## Verification map

| IR の項目 | 確かめるステップ |
|---|---|
| `ir-document.md#REQ-033`、`config.md#REQ-018`、`cli.md#TBL-001`、`cli-environment.md#TBL-020`、`base-directory.md#REQ-110`、`output.md#TBL-006`、`findings.md#TBL-008`（文書全体の detail）、`findings.md#REQ-032`、`CONTEXT.md` の `除外`・`用語集`・`問題の記録` | 1 |
| `terms.md#REQ-064`、`#REQ-065`、`terms-form.md#REQ-117`、`#REQ-123`、`finding-order.md#TBL-019`、`CONTEXT.md` の `連鎖`・`用語` | 2 |
| `terms.md#REQ-069`、`#REQ-070`、`#TBL-014`、`findings.md#TBL-008`（missing_document） | 3 |
| `ir-references.md#REQ-124`、`#REQ-114`、`ir-items.md#REQ-043`、`CONTEXT.md` の `ID` | 4 |
| `coverage.md#REQ-085` と自己適用 | 5、6 |

## Left to the implementer

- 関数と型の名前、モジュールの中の切り方
- 再帰の走査の書き方（`walkdir` か手書きか）
- 連鎖を表す型と、duplicate_term の判定を移す先

## Stop conditions

計画全体で、次のときは作業を止めて返す。

- IR か契約に書かれていない振る舞いを決めないと進めない
- IR と契約が食い違う、または IR の中で食い違う
- 新しい実行時の依存が要る
- 既存のテストを書き直すとき、どの決定で改めたかが判断の記録から見つからない
- 処理を10分の上限に収めるために分割できない（バックグラウンドに逃がさない）

## Test command

`CARGO_BUILD_JOBS=4 cargo test`。重いコマンドは同時に1つだけ走らせる。バックグラウンドでは走らせない。

## Out of scope

- スキル `skills/kotowari/` の references（`ir-form.md` の「3桁」、`workflow.md`、`config.md`）と SKILL.md の版の固定。本体の変更の後に別の作業で追従する
- 版の上げ方（判断の記録 U1）。`Cargo.toml` の version はこの計画では変えない
- kakoi など利用側のプロジェクトの IR の移行
- `docs/trace.md` の「ミューテーションテスト」の節とミューテーションの実行
- `experiments/003-cli/tools/gen_trace2.py` の glob をサブディレクトリ対応にすること（kotowari 自身の IR は当面直下に置く）
- CONTEXT-MAP（判断の記録 U3）
