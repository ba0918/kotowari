# 実装計画: check-reach

## Goal

`kotowari check` の JSON が読んだテストのファイルを拡張子ごとに申告し、review のまま残していた10件の禁止の要求が振る舞いのテストで固定され、スキルの references が本体の値とずれたらこのリポジトリのテストが落ちるようにする。

## Specification

正本は仕様IR `docs/ir/*.md`（以下「IR」）。この計画が対象にする項目は次のとおり。

- `docs/ir/output.md#REQ-128`、`#REQ-023`、`#TBL-005`、`#TBL-021`、`#PROP-004`、具体例 EX-035 と EX-038（JSON の "tests"）
- `docs/ir/skill-references.md#REQ-125`、`#REQ-126`、`#REQ-127`、具体例 EX-036、EX-037、EX-039（references と本体の一致）
- 検証が unit になった10件の禁止の要求: `docs/ir/cli.md#REQ-008`、`docs/ir/config.md#REQ-020`、`docs/ir/ir-references.md#REQ-055`、`#REQ-056`、`docs/ir/sources.md#REQ-062`、`docs/ir/terms.md#REQ-068`、`docs/ir/form-contract.md#REQ-090`、`#REQ-091`、`docs/ir/cli-scope.md#REQ-102`、`docs/ir/cli-environment.md#REQ-121`
- 各テストが与える入力と期待の元: 判断の記録 `docs/decision/brainstorm/2026-09-17-check-reach.md` の A4、A18、A21、A22、A23
- 既存の要求で境界を決めているもの: `docs/ir/test-discovery.md#REQ-079`（テストのファイル）、`#REQ-081`（拡張子と言語）、`#REQ-083`（読めないテストのファイル）、`docs/ir/cli.md#REQ-002`、`#REQ-004`（受けるオプションと引数の誤り）、`docs/ir/config.md#REQ-014`（知らないキー）、`#TBL-004`（キーと既定）、`docs/ir/coverage.md#REQ-087`、`docs/ir/test-markers.md#REQ-076`（問い合わせの無い言語の扱い）、`docs/ir/cli-environment.md#TBL-018`（停止の文言）、`docs/ir/findings.md#TBL-008`、`#TBL-009`（指摘の種類）、`docs/ir/CONTEXT.md` の用語 `テストのファイル`、`問い合わせのある言語`、`除外`

判断の記録は `docs/decision/brainstorm/2026-09-17-check-reach.md`（A1〜A26、R1〜R6、U1）。IR の形は `docs/decision/brainstorm/ir-form.md`（以下「契約」）。

IR の読み方は前の計画と同じ。要求の `- 種類:` が `prohibition` の文は起きてはならないこと、`algorithm` の中身は `- 定義:` の指す決定表と性質にある、`- 検証:` が `review` の要求はテストでなく読む場所を `docs/trace.md` に書く。IR の各文書の `## 具体例` のシナリオは、その要求の成功の条件と反例で、テストの入力と期待の元にする。

IR と契約と判断の記録は実装の間は読み取り専用である。IR の書き換えが必要だと思ったら、書き換えずに止まって返す。IR の沈黙は実装者が決めてよい意味ではない（`docs/ir/cli-environment.md#REQ-120`）。

この計画の中のテストの置き方と名前の付け方は、IR ではなくこの計画の約束である。

## Approach and why

既存の実装（`src/` 約3,900行、テスト約370件）を育てる。作り直さない。新しい依存は足さない。

変更は4つの独立した性格に分かれる。

1. **JSON に "tests" を足す**: いま `src/tests_discovery.rs` の `discover_and_check` は glob で集めたファイルを1つずつ読み、拡張子（`Path::extension`）で問い合わせのある言語かを分けているが、その集計を外に返さない。`src/lib.rs` の `CheckResult` は `files`、`lines`、`findings`、`counts` の4つで、`serde::Serialize` でそのまま JSON になる。集計を `discover_and_check` から返し、`CheckResult` に鍵を1つ足す。拡張子の取り方は TBL-021 が Rust の標準（最後の "." より後ろ。先頭の "." だけの名前と "." の無い名前は無し）と同じに決めているので、いまの `Path::extension` をそのまま使い、無いとき（`None`）と空のとき（"foo." の `Some("")`）を空文字列の鍵に畳む。集められたパスは `WalkDir` の各要素に対して `GlobSet` を1回だけ当てて1回だけ push されるので（`collect_test_files`）、複数の glob に当たっても重複せず、重複除去は要らない
2. **10件の禁止の要求を振る舞いのテストで固定する**: 実装は変えない。禁止された機能が反応するはずの入力を与えて何も起きないことを見る（記録 A3）。これらのテストは「いまの実装が既に守っていること」を固定するものなので、書いた時点で GREEN になるのが正常で、RED になったら仕様と実装の食い違いとして止まって返す。テストが落ちうることの確認は、禁止された機能が在ったときに観測されるはずの値を期待に一時的に置いて RED を見てから戻す（コミットしない）ことで行う
3. **references と本体の一致テスト**: `skills/kotowari/references/findings.md` と `config.md` を `env!("CARGO_MANIFEST_DIR")` からの相対で読み、Markdown の表の1列目と YAML のコードブロックだけを最小限に読んで、本体の値（指摘の種類の集合、設定の既定、停止の文言）と比べる。本体側は、種類の一覧を得る手段（いまは `FindingKind::as_str` の match だけで、全種類を列挙する関数が無い）と、設定の等価比較（`Config` に `PartialEq` が無い）が足りないので、その分だけ `src/lib.rs` と `src/config.rs` を触る
4. **references の改訂**: JSON の形が変わるので、`skills/kotowari/references/findings.md`（JSON の読み方）を本体に追従させる（記録 A10、A16）。A10 が挙げる `workflow.md` は最上位の鍵を列挙していないので変えない（ステップ4）

読む順は 1 → 3 → 4 と、2（どこにも依存しない）。3 は 1 に依存しないが、1 で `CheckResult` の形が変わった後に書くほうが JSON の読み方の改訂（4）と同じ流れで確かめられる。

テストの置き方と名前（この計画の約束。前の計画と同じ）:

- テストは `tests/` に置き、ライブラリの公開する関数と型と CLI だけを通す。ファイルは既存の `tests/step1_cli.rs`（CLI の引数）、`tests/step1_config.rs`（設定と停止）、`tests/step2_ir.rs`（文書の読み込み）、`tests/step3_sources_terms.rs`（出典、用語）、`tests/step4_test_discovery.rs`（テストの発見）、`tests/step6_output.rs`（出力の形）に足し、references の一致テストだけ新しい `tests/step7_skill_references.rs` に置く。ファイル名の `stepN` は前の計画の番号で、この計画では変えない
- 各テスト関数の直前の行に `// @kotowari[REQ-128, TBL-021]` の印を置く
- テスト関数の名前は、確かめる IR の ID を先頭に付けた snake_case（例: `req_128_tests_key_lists_files_per_extension`）
- テストは RED → GREEN → REFACTOR の順で書き、各段で `CARGO_BUILD_JOBS=4 cargo test` を走らせる。ステップ2の禁止のテストとステップ3の一致のテストは、書いた時点で GREEN になるのが正常なので、代わりに「その機能が在ったとき（または references がずれたとき）に観測されるはずの値」を期待に一時的に置いて RED を見る。その RED の出力（落ちたテスト名と assert の文）を終端報告に引用し、期待を戻してから GREEN を見る。反転した状態はコミットしない
- 既存のテストが新しい仕様と食い違うときは、新しい仕様に合わせて書き直し、コミットの本文に「A8 で改めた」のように改めた決定を書く。最上位の鍵の数を数える既存テストは無い（`req_023_files_counts_all_docs_and_lines_sums_them` は鍵の有無だけを見る）ので、"tests" の追加で落ちる既存テストは無い見込み
- コミットは意図ごとに分け、`git add <path>` でファイルを1つずつ。件名は `<type>: <件名>` の形で日本語

## Scope of change

変えてよいのは、リポジトリ直下の次のものだけである。

- `src/` の下のすべて
- `tests/` の下のすべて
- `skills/kotowari/references/findings.md`（ステップ4）
- `Cargo.lock`（依存の更新はしない。ビルドで変わった分だけ）

`Cargo.toml` は変えない（依存を足さない）。`docs/ir/`、`docs/decision/`、`docs/plans/`、`docs/trace.md`、`docs/spec/`、`skills/kotowari/SKILL.md`、ほかの references、リポジトリ直下の `.kotowari/`、`experiments/`、`fixtures/` は変えない。

## Step order and prerequisites

ステップ1を最初に行う。ステップ3はステップ1の後、ステップ4はステップ3の後。ステップ2はどこにも依存しないので、1〜4 のどこに挟んでもよい。ステップ5は 1〜4 のすべての後。

## Step 1 — JSON の最上位に "tests" を足す

Purpose: 読んだテストのファイルを拡張子ごとに数え、その拡張子が問い合わせのある言語かとともに JSON に出す。Specification: `docs/ir/output.md#REQ-128`、`#TBL-005`、`#TBL-021`、`#PROP-004`、具体例 EX-035、EX-038、`docs/ir/test-discovery.md#REQ-079`、`#REQ-081`、`#REQ-083`、`docs/ir/CONTEXT.md`（`テストのファイル`、`問い合わせのある言語`、`除外`）。
Prerequisites: なし。始める前に `CARGO_BUILD_JOBS=4 cargo test` を1回走らせて全件通ることと、`CARGO_BUILD_JOBS=4 cargo run -q -- check` の出力を記録する（ステップ5でこの記録と比べる）。この時点の終了コードは1で、"findings" は requirement_without_test が14件（この計画の対象の要求）、"counts" は `{"requirement_without_test": 14}` のはずである。
May change: `src/lib.rs`、`src/tests_discovery.rs`、`src/main.rs`（text 形式に出さないことの確認だけ。いま text は findings しか出さないので変更は要らないはず）、`tests/step6_output.rs`、`tests/step4_test_discovery.rs`。
Done when: JSON の最上位に "tests" があり、glob に当たって読んだファイルが拡張子（最後の "." より後ろ、"." を含めない。先頭の "." だけの名前、"." の無い名前、末尾が "." の名前は空文字列）ごとに "files"（数）と "query"（鍵 "rs" だけ true）で並び、鍵はバイト順（JSON の文字列の上で "" → "RS" → "py" → "rs" の順）、テストのファイルが0件なら空のオブジェクトになる。unparsable_file を出したファイルも数える。大文字の ".RS" は "RS" の鍵で "query" は false。同じファイルが複数の glob に当たっても1回、ファイルのシンボリックリンクと実体は別のパスとして2回数える。glob に当たっても読まないもの（`除外`。`docs/ir/test-discovery.md#REQ-079` のソケットなど）は数えない。"tests" の "files" の合計は読んだテストのファイルの数に等しい。"--format text" の出力に "tests" は現れない。`files` と `lines` は変わらず IR の文書だけを数える。
Shown by: test — `req_128_tests_key_lists_files_per_extension_with_query_flag`（EX-035: ".rs" 2つと ".py" 1つ）、`req_128_tests_is_an_empty_object_without_test_files`、`req_128_text_format_does_not_print_tests`、`tbl_021_extension_is_after_the_last_dot_and_dotless_names_share_the_empty_key`（"a.test.rs"、".rs"、"run"、"foo." の4つ）、`tbl_021_uppercase_extension_is_a_separate_key_without_query`、`tbl_021_unparsable_file_is_counted`（EX-038）、`tbl_021_same_path_counts_once_and_symlink_counts_apart`（複数の glob に当たる1ファイルと、リンクと実体の組）、`tbl_021_keys_are_in_byte_order`（""、"RS"、"py"、"rs" の4つを混ぜて JSON の文字列上の出現順を見る）、`tbl_021_excluded_entries_are_not_counted`（glob に当たる位置に `std::os::unix::net::UnixListener` を bind して "tests" に現れないことを見る。既存の `tests/step2_ir.rs` のソケットのテストと同じ作り）、`prop_004_files_sum_equals_the_number_of_read_test_files`（proptest で拡張子の並びを生成し、合計を比べる。手本は `tests/step5_findings.rs` の `prop_003_findings_are_sorted`。1件ごとに CLI を起動するので `proptest::test_runner::Config` で試行回数を絞る）。既存の `req_023_files_counts_all_docs_and_lines_sums_them` が変更なしで通ることで、"files" と "lines" が変わらないことを示す。
Left to the implementer: 集計を運ぶ Rust の型と欄の名前（JSON の鍵の名前は TBL-021 で固定。Rust の名前は自由）。集計を `discover_and_check` の返り値で返すか、引数の可変参照に書くか。
Stop and hand back if: `collect_test_files` がファイルのシンボリックリンクを実体のパスに解決していて、リンクと実体を別に数えられない（REQ-079 は「ファイルのシンボリックリンクは読む」としか言わず、TBL-021 は別に数えると決めている。既存のテスト `req_079_file_symlink_is_read` の作りを見て、解決しているなら止まる）。text 形式が findings 以外の行を出している。

## Step 2 — 10件の禁止の要求を振る舞いのテストで固定する

Purpose: review から unit に変えた10件の禁止の要求それぞれについて、禁止された機能が反応するはずの入力を与え、何も起きないことをテストで固定する。実装は変えない。Specification: `docs/ir/cli.md#REQ-008`、`#REQ-002`、`docs/ir/config.md#REQ-020`、`#REQ-014`、`#TBL-004`、`docs/ir/ir-references.md#REQ-055`、`#REQ-056`、`docs/ir/sources.md#REQ-062`、`docs/ir/terms.md#REQ-068`、`docs/ir/form-contract.md#REQ-090`、`#REQ-091`、`docs/ir/cli-scope.md#REQ-102`、`docs/ir/cli-environment.md#REQ-121`。入力と期待は判断の記録の A4、A18、A21、A22、A23 のとおり。
Prerequisites: なし。
May change: `tests/step1_cli.rs`、`tests/step1_config.rs`、`tests/step2_ir.rs`、`tests/step3_sources_terms.rs`。`src/` は変えない。
Done when: 次の10件が GREEN で、それぞれ「禁止された機能が在ったときに観測されるはずの値」を期待に一時的に置くと RED になることを確かめ、その RED の出力を終端報告に引用し、期待を戻してある。

- REQ-008: `kotowari render`、`kotowari check render` の2つの形で、"render"、"trace"、"query" のそれぞれについて終了コード2、標準エラーの1行目が "argument error" で始まる（6通り）。加えて、知らないオプションが argument error になることを固定している既存の `req_004_unknown_option_stops` の印に REQ-008 を足す（A23 は「REQ-002 のテスト」と書いているが、`req_002_only_format_and_config_options` は `--format json` が通ることしか見ていない。受けるオプションが `docs/ir/cli.md#REQ-002` の4つだけであることの否定側は `req_004_unknown_option_stops` が持つので、印はそちらに付ける。判断の記録の Revisions に同じことを書いてある）
- REQ-020: リポジトリ直下（テストの fixture では基準のディレクトリと同じ場所）に "kotowari.toml" を置き、その中に置き場を変える値（例: `ir = "elsewhere"`）を書いても、既定の "docs/ir" が読まれて結果が変わらない（A4）
- REQ-055: EARS の型に沿わない文（例: 「常に」も「とき」も無い平叙文）を持つ要求を置いても、その行を指す指摘が1件も無い（A4。要求自身の requirement_without_test は行が見出しなので区別できる）
- REQ-056: 種類 contradiction の問題の記録を1件置き、本文に読みを1つだけ書いても、その項目を指す指摘が1件も無い（A4）
- REQ-062: 出典の決定の本文が項目の内容と無関係でも、source_invalid が出ない（A4）
- REQ-068: 用語集にある語をバッククォートで囲まずに書いた文を持つ要求を置いても、その行を指す指摘が1件も無い（A4）
- REQ-090: ".kotowari/schema.yaml" に壊れた YAML を置いても停止せず、結果が変わらない（A4）
- REQ-091: PATH を空のディレクトリだけに向けて check を走らせても、結果が PATH をそのままにしたときと等しい（A18）
- REQ-102: HOME と TMPDIR を一時ディレクトリに向けて check を走らせ、その一時ディレクトリと基準のディレクトリの全ファイルの一覧と中身のバイト列が前後で等しい（A21。ハッシュの crate は依存に無いので、中身をそのまま比べる）
- REQ-121: 設定が受け付ける鍵が TBL-004 の9個で、それ以外の鍵はどの階層に書いても config error で停止する（A22。既存の `req_014_unknown_key_stops` は最上位の1つだけを見ている。この1本は最上位、`decisions`、`tests`、`tests.rust`、`limits` の各階層に知らない鍵を1つずつ入れて全部止まることと、9個をすべて書いた設定が通ることを見る）

Shown by: test — `req_008_render_trace_query_are_argument_errors`、`req_020_kotowari_toml_beside_the_base_is_not_read`、`req_055_non_ears_statement_gets_no_finding_on_its_line`、`req_056_contradiction_flag_with_one_reading_gets_no_finding`、`req_062_source_content_is_not_matched_against_the_item`、`req_068_unquoted_term_gets_no_finding_on_its_line`、`req_090_schema_file_is_not_read`、`req_091_check_runs_the_same_with_an_empty_path`、`req_102_check_writes_nothing_under_home_tmpdir_or_base`、`req_121_only_the_nine_config_keys_are_accepted`（印は `@kotowari[REQ-121, REQ-014]`）。
Left to the implementer: 各テストの fixture の中身（IR の文書の文言、判断の記録の行）。前後の一覧と中身を比べるときの走査の書き方（`walkdir` でも `std::fs::read_dir` の再帰でも同じ）。
Stop and hand back if: どれか1件が、期待を逆にしなくても RED になる（仕様と実装が食い違っている。テストの期待を変えて通してはならない）。`assert_cmd` で環境変数（PATH、HOME、TMPDIR）を差し替えて走らせたとき、kotowari 以外の理由（例: cargo のランナ）で失敗する。

## Step 3 — references と本体の一致をテストにする

Purpose: スキルの references に写した本体の値がずれたら、このリポジトリのテストが落ちるようにする。Specification: `docs/ir/skill-references.md#REQ-125`、`#REQ-126`、`#REQ-127`、具体例 EX-036、EX-037、EX-039、`docs/ir/findings.md#TBL-008`、`#TBL-009`、`docs/ir/config.md#TBL-004`、`docs/ir/cli-environment.md#TBL-018`。
Prerequisites: ステップ1（JSON の形が確定してから、references の読み方の改訂と同じ流れで進めるため。技術的な依存は無い）。
May change: `src/lib.rs`（指摘の種類をすべて列挙する手段。停止の文言の先頭を取り出す手段）、`src/config.rs`（`Config` と入れ子の構造体の等価比較）、`tests/step7_skill_references.rs`（新規）。`skills/` は変えない。
Done when: `skills/kotowari/references/findings.md` のヘッダの1列目が「種類」の表の、ヘッダと区切りの行を除いた1列目の集合が、本体の指摘の種類の文字列の集合（本体が列挙する全種類。いまは誤り33種と注意2種の35種）と等しいことをテストが確かめる。同じファイルのヘッダの1列目が「文言」の表の1列目の集合が、本体の停止の理由4つの文言（"config error"、"argument error"、"unreadable file"、"non-UTF-8 file"）の集合と等しいことをテストが確かめる。`skills/kotowari/references/config.md` の setup の手順1の直後にある最初の言語 `yaml` のコードブロックを `Config::parse` に通した結果が `Config::default()` と等しいことをテストが確かめる。references の「種類」の表から "duplicate_term" の行を消す（unresolved_reference の行は2行あるので、そちらを1行消しても集合は変わらない。比較は行数でなく集合）、YAML の "limits.lines" を 120 にする、「文言」の表の "config error" を "configuration error" にする、の3つそれぞれでテストが落ちることを一時的に変えて確かめ、その RED の出力を終端報告に引用してから戻してある（EX-036、EX-037、EX-039）。
Shown by: test — `req_125_findings_reference_kinds_match_the_code`、`req_126_config_reference_setup_yaml_parses_to_the_defaults`、`req_127_findings_reference_stop_wordings_match_the_code`。
Left to the implementer: 指摘の種類を列挙する手段（`FindingKind` に全種類の定数の並びを足す、`strum` のような依存は足さない）。停止の文言の取り出し方（各 `StopReason` を空の詳細で作って `Display` の ": " より前を取る、または文言を返す関数を足す）。`Config` の等価比較（`PartialEq` の derive を勧める。`NonZeroU64` と `Vec<String>` は derive できる）。Markdown の表と YAML ブロックの読み方（行頭が "|" の行を "|" で割る、"```yaml" から "```" までを取る、程度の手書きでよい。Markdown の parser は足さない）。
Stop and hand back if: references/config.md に言語 `yaml` のコードブロックが2つ以上ある（いまは1つ。REQ-126 は「手順1にある」ブロックと言っており、2つ以上あれば手順の見出しで選ぶ規則が要る）。references/findings.md の「種類」の表に、本体に無い種類が書かれている（テストを通すために表を直すのはステップ4の仕事だが、本体に無い種類が references に「ある」場合は本体の欠落か references の誤りかを人が決める）。

## Step 4 — references を JSON の新しい形に追従させる

Purpose: JSON の最上位に "tests" が増えたことを、配布先の LLM が読む references に反映する。Specification: `docs/ir/output.md#TBL-005`、`#TBL-021`、`docs/ir/coverage.md#REQ-087`、`docs/ir/test-markers.md#REQ-076`、判断の記録の A10、A16。
Prerequisites: ステップ1とステップ3。
May change: `skills/kotowari/references/findings.md`。`workflow.md` は A10 が範囲に挙げているが、最上位の鍵を列挙しておらず「check の出力を終端報告に載せる」としか書いていないので、"tests" が増えても文は変わらない。変えない（`docs/spec/kotowari-skill.md` が同じ文を持ち、そちらは変更禁止なので、変えるとずれる）。
Done when: `findings.md` の冒頭（JSON の読み方）に、最上位の "tests" が読んだテストのファイルの拡張子ごとの数（"files"）と、その拡張子が問い合わせのある言語か（"query"）であること、"query" が false の拡張子は`問い合わせの無い言語`として印だけを拾いテストの数は見ないこと（`docs/ir/test-markers.md#REQ-076`、`docs/ir/coverage.md#REQ-087` の主語をそのまま使う）が書かれている。ファイル先頭の改訂日が作業を行った日（YYYY-MM-DD の形）になっている。ステップ3のテストが引き続き GREEN（表の1列目と YAML ブロックは変えない）。
Shown by: artifact — `skills/kotowari/references/findings.md` の差分。check — `CARGO_BUILD_JOBS=4 cargo test`（ステップ3のテストが通る）。
Left to the implementer: 文言。ただし既存の references の書き方（見出しの粒度、表の使い方、二重引用符とバッククォートの使い分け）に合わせる。
Stop and hand back if: 追従のために references の表の1列目や YAML ブロック（ステップ3が読む部分）を変える必要が出た。

## Step 5 — 自己適用と全件の確認

Purpose: kotowari 自身の IR に対して `kotowari check` が終了コード0になり、全テストが通ることを確かめる。Specification: `docs/ir/coverage.md#REQ-085`（テストのない要求）、`docs/ir/output.md#TBL-005`。
Prerequisites: ステップ1〜4。
May change: なし（ここで直すものが出たら、該当のステップに戻る）。
Done when: `CARGO_BUILD_JOBS=4 cargo run -q -- check` の終了コードが0で、"findings" が空、"counts" が空、"tests" に "rs" の "files" が `src/` と `tests/` の ".rs" の数で "query" が true、ほかの鍵が無い。ステップ1の前に記録した出力と比べて、変わったのは "tests" が増えたことと、requirement_without_test の14件が "findings" と "counts" から消えたことだけ。`CARGO_BUILD_JOBS=4 cargo test` が全件通る。
Shown by: check — `CARGO_BUILD_JOBS=4 cargo test` → `CARGO_BUILD_JOBS=4 cargo run -q -- check --format json`（終了コード0）→ `CARGO_BUILD_JOBS=4 cargo run -q -- check --format text`（出力が空）。
Left to the implementer: なし。
Stop and hand back if: requirement_without_test が残る（印の付け忘れなら該当のステップで直す。IR の要求に対応するテストが書けないなら止まる）。

## Verification map

| 仕様の項目 | ステップ |
|---|---|
| `docs/ir/output.md#REQ-128`、`#REQ-023`、`#TBL-005`、`#TBL-021`、`#PROP-004`、EX-035、EX-038 | 1、5 |
| `docs/ir/cli.md#REQ-008`、`docs/ir/config.md#REQ-020`、`docs/ir/ir-references.md#REQ-055`、`#REQ-056`、`docs/ir/sources.md#REQ-062`、`docs/ir/terms.md#REQ-068`、`docs/ir/form-contract.md#REQ-090`、`#REQ-091`、`docs/ir/cli-scope.md#REQ-102`、`docs/ir/cli-environment.md#REQ-121` | 2 |
| `docs/ir/skill-references.md#REQ-125`、`#REQ-126`、`#REQ-127`、EX-036、EX-037、EX-039 | 3 |
| 判断の記録 A10（references の改訂）、`docs/ir/coverage.md#REQ-087` | 4 |
| `docs/ir/coverage.md#REQ-085`（自己適用で0件） | 5 |

## Left to the implementer

- Rust 側の型と欄と関数の名前。JSON の鍵と文言は IR で固定
- テストの fixture の具体的な中身
- 集計をどの関数で行うか（`discover_and_check` の中でも、`collect_test_files` の直後でもよい。読まないもの（除外）を数えない点だけ守る）

## Stop conditions

一般の4条件（意味の欠落や承認した内容からの逸脱、不可逆・特権・危険な操作、事故の拡大、方針を変えても進捗が無い）に加えて:

- ステップ2のテストが期待を逆にしなくても RED になる。仕様と実装の食い違いで、テストを変えて通してはならない
- IR、契約、判断の記録を書き換える必要が出た
- `Cargo.toml` に依存を足す必要が出た
- TBL-021 に無い境界に当たった（例: 拡張子が UTF-8 でないファイル名）。REQ-120 のとおり黙って決めない

## Test command

`CARGO_BUILD_JOBS=4 cargo test`。1ファイルだけなら `CARGO_BUILD_JOBS=4 cargo test --test step6_output`。自己適用は `CARGO_BUILD_JOBS=4 cargo run -q -- check`。

## Out of scope

- `docs/trace.md` の更新（review の行だけになっており、この計画の10件は既に除かれている）
- Rust 以外の言語の問い合わせ（`docs/decision/brainstorm/records.md` の U37）
- mds を crate として取り込むこと（判断の記録の U1）
- references の一致テストで、表の2列目以降や config.md の表を読むこと（記録 A11、A12）
- text 形式に "tests" を出すこと（記録 A8）
