# 実装計画: kotowari-general

## Goal

実験003で作った `kotowari check` を、仕様IRの第2段階（壊れた入力を黙って通さない、境界の規定、新しい検査の種類5つ、`--help` と `--version`、パスの正規化、指摘の行の決め方の表）に合わせて育て、契約の形の IR を持つどのリポジトリでも、読めない・壊れている・形に合わない入力を必ず停止か指摘にする検査ツールにする。

## Specification

正本は仕様IR `docs/ir/*.md`（以下「IR」。23文書、要求120件。REQ-119 は欠番）で、形は `experiments/003-cli/brainstorm/ir-form.md`（以下「契約」）に従う。判断の記録は `experiments/003-cli/brainstorm/records.md`（第9ラウンドの A98〜A144、P2）。

IR の読み方は前の計画（`docs/plans/kotowari-check.md`、git の履歴にある）と同じ。要求の `- 種類:` が `prohibition` の文は起きてはならないこと、`algorithm` の中身は `- 定義:` の指す決定表と性質にある、`- 検証:` が `review` の要求はテストでなく読む場所を `docs/trace.md` に書く。用語は `docs/ir/CONTEXT.md`。

IR と契約は実装の間は読み取り専用である。IR の書き換えが必要だと思ったら、書き換えずに止まって返す。IR の沈黙は実装者が決めてよい意味ではない（`docs/ir/cli-environment.md#REQ-120`）。

この計画の中のテストの置き方と名前の付け方は、IR ではなくこの計画の約束である。

## Approach and why

既存の実装（`src/` 約3,200行、テスト177件、レビュー9周で仕様との食い違い0）を育てる。作り直さない（判断の記録 A126）。

第2段階の変更は、性格で3つに分かれる。

1. **原則の適用**: 黙って飛ばす経路（`filter_map(|e| e.ok())`、握りつぶす `if let Ok`、既定値への黙った置き換え）を、停止か指摘か仕様に列挙した除外のどれかに変える（`docs/ir/cli-environment.md#REQ-109`、`#REQ-120`、`docs/ir/CONTEXT.md` の`除外`）。これは既存の関数の中の1か所ずつの修正で、構造は変えない
2. **境界の規定**: 引数、設定、パス、文字コード、コードブロック、gherkin の行、ID、一覧の行、用語集の表、出典、印、テストの数え方の境界を IR のとおりにする。多くは既存の関数の条件を変えるか、分岐を1つ足す
3. **1か所にまとめる**: パスの正規化（`docs/ir/base-directory.md#REQ-110`）と指摘の行の決め方（`docs/ir/finding-order.md#TBL-019`）は、いま複数の場所に散っている判断を1つの関数か表に集める。detail が「行の文字」の種類は読んだ行の文字そのまま（`docs/ir/findings.md#TBL-008` の注記）

読む順（引数と設定 → 文書 → 項目と gherkin → 出典と用語 → テストと印 → 指摘の種類と行 → 自己適用）で進める。用語集の`除外`の列挙のうち、この段階で新しく決まったもの（ファイルのシンボリックリンクを読む、".kotowari" という名前のファイル、コードブロックの中、奇数の二重引用符の後ろ）は各ステップで確かめ、以前からの振る舞い（隠しディレクトリ、サブディレクトリ、gherkin の外の "Scenario:"、テストの外と本体の途中の印、TBL-013 の対象外の行、"## " の直下の行）は既存のテストが守る。後のステップは前のステップの結果（設定、読んだ文書、ID の集合）を使う。

設計の観点のレビューを実装の前に1回かける（判断の記録 A126）。その結果はこの計画の「設計レビューの所見」の節にあり、各ステップの Left to the implementer と Stop の条件に反映してある。

再利用の判断: 直接の実行時の依存は前の計画の表のまま（`clap` は前の cycle で除いた。引数は自作の読み手）。新しい依存は足さない。BOM の読み飛ばし、コードブロックの境界、パスの正規化、用語集の表の範囲はいずれも数行で、標準ライブラリで書く。足す必要が出たら止まって返す。

テストの置き方と名前（この計画の約束。前の計画と同じ）:

- テストは `tests/` に置き、ライブラリの公開する関数と型と CLI だけを通す
- 各テスト関数の直前の行に `// @kotowari[REQ-001, TBL-002]` の印を置く
- テスト関数の名前は、確かめる IR の ID を先頭に付けた snake_case（例: `req_112_unclosed_code_block_is_an_error`）
- テストは RED → GREEN → REFACTOR の順で書き、各段で `CARGO_BUILD_JOBS=4 cargo test` を走らせる
- 既存のテストが新しい仕様と食い違うとき（例: `req_079_symlink_is_not_followed` はファイルのシンボリックリンクを読まないことを確かめている）は、新しい仕様に合わせて書き直し、コミットの本文に「A102 で改めた」のように改めた決定を書く
- コミットは意図ごとに分け、`git add <path>` でファイルを1つずつ。件名は `<type>: <件名>` の形で日本語

## Scope of change

変えてよいのは、リポジトリ直下の次のものだけである。

- `src/` の下のすべて
- `tests/` の下のすべて
- `fixtures/` の下のすべて（新しい入力の固定が要るときだけ）
- `docs/trace.md`（ステップ7）
- `Cargo.lock`（依存の更新はしない。ビルドで変わった分だけ）

`Cargo.toml` は変えない（依存を足さない）。`docs/ir/`、`docs/plans/`、リポジトリ直下の `.kotowari/`、`experiments/` は変えない。

## Step order and prerequisites

ステップ0a、0b、0c を最初に順に行い、次にステップ1〜4を順に行う。ステップ5と6はどちらもステップ4の後で、互いに依存しないので順は問わない。ステップ7は5と6の両方の後に行う。`tests/` のファイル名の `stepN` は前の計画のステップ番号で、この計画では変えない。ミューテーションテストと対応の表の最終確認は、cycle の外で実験者が行う（実験003の D-21。この計画には入れない）。

## Test command

`CARGO_BUILD_JOBS=4 cargo test`。重いコマンドは同時に1つだけ走らせる。バックグラウンドでは走らせない。

## 設計レビューの所見

実装の前に、設計の観点（層の向き、純粋な部分と入出力の分離、境界で1回の検証、失敗は戻り値、網羅的な match）のレビューを1回かけた（判断の記録 A126）。所見は次のとおり。層の分かれ方は「概ね分かれているが逆流が3か所」で、第2段階を足す前に直すべき構造が5つある。ステップ0でこれを振る舞いを変えずに行い、ステップ1以降はその上に足す。

1. **指摘の種類を型にする**: `Finding` の kind と severity が文字列で、`Finding { kind: "...", severity: "..." }` のリテラルが src に46か所ある。種類を `enum` にし、severity と TBL-019 の line の決め方をその `match` で1か所に持たせる。網羅的な match にすれば、種類を足したときの記入漏れがコンパイルの誤りになる（`docs/ir/findings.md#TBL-008`、`docs/ir/finding-order.md#TBL-019`）
2. **ファイルの読み込みを1本にする**: `fs::read` と `String::from_utf8` と停止への変換が6か所（`lib.rs` に2、`ir.rs`、`sources.rs` に2、`tests_discovery.rs`）に複製されている。`read_utf8_file(&Path) -> Result<String, StopReason>` を1つ作って6か所を置き換える（ステップ0では振る舞いを変えない）。BOM の読み飛ばし（`docs/ir/ir-input.md#REQ-111`、ステップ3）と詳細の相対パス化（`docs/ir/cli-environment.md#TBL-020`、ステップ1）は、後のステップでこの1か所に足す
3. **パスの正規化を入口に置く**: 出力の path を組む `format!("{}/{}", ir_path, filename)` が6か所、置き場との前置一致が `sources.rs` にあり、すべて設定の値をそのまま使っている。`normalize_path(&str) -> String` を純粋な関数として作る（ステップ0は定義だけ）。`Config::parse` で設定の3つのパスを、`sources::split_source` で出典のパスを通すのはステップ2（`docs/ir/base-directory.md#REQ-110`）
4. **解析の指摘の出口を作る**: `parse_document` は `IrDocument` を返すだけで、解析中にしか分からない指摘（unclosed_code_block、invalid_gherkin_line、invalid_id、glossary_invalid）を返す先が無い。`IrDocument` に解析の指摘の欄を足すか、組で返す。あわせて、コードブロックの状態が `ir.rs` と `terms.rs` に別々に複製されている（`terms.rs` 側は記号の種類も個数も見ない）ので、行ごとの分類（ブロックの外、中、gherkin の中、境界、閉じられなかった開始）を返す純粋な関数を切り出し、両方がそれを使う。gherkin の1行の分類（タグ、Scenario、ステップ、注釈、空行、不正）も純粋な関数にし、タグの「直前の行だけ」は「直前の行がタグだったか」の1つの状態で持つ
5. **引数の解析をライブラリに移す**: 引数の読み手は `main.rs` にあり、公開関数から呼べない。`parse_args(&[String]) -> Result<Cli, StopReason>` を `lib.rs` に置く。`--help` の優先と重複の検出をバイナリ起動なしで確かめられるようにする

ほかに直すもの: 黙って飛ばす経路が8か所（`ir.rs` の `read_dir` の `filter_map(|e| e.ok())` と `file_type` の握りつぶし、`sources.rs` の同じ4か所、`tests_discovery.rs` の glob の `if let Ok` と `builder.build()` の失敗で空の一覧を返す箇所）。`known_ids` の計算が `lib.rs` と `ir.rs` の2か所にある（1本にする）。バッククォートの走査が `ir.rs` と `terms.rs` の2本ある（1本にする）。`build_fields_seen` が行を `format!` で再構成している（生の行を持たせる）。`Config` の Raw 型が `Option<T>` で、キーの欠落と YAML の null を区別できない。マクロの中の再解析に `has_error()` の検査が無い。`run_check` が整列と counts の集計を自分で行っている（純粋な関数に出す）。

未確認: 使っている YAML の読み手（`serde_saphyr`）が同じキーの2回目をどう扱うか。ステップ2の Stop の条件に置いた。

## Step 0a — 指摘の種類を型にする（振る舞いを変えない）

Purpose: 指摘の kind と severity を文字列から `enum` に替え、Finding の構築を1つの入口に通し、severity をその `match` で決める。Specification: この段では無し（振る舞いは変えない）。
Prerequisites: なし。始める前に `CARGO_BUILD_JOBS=4 cargo test` を1回走らせて177件通ることと、`CARGO_BUILD_JOBS=4 cargo run -q -- check` の出力を記録する（ステップ0の各段はこの記録と比べる）。
May change: `src/` の下のすべて、`tests/`（テストの名前と印は変えない。公開関数の名前が変わった分だけ追従する）。
Done when: src の中の `Finding { kind: "...", severity: "..." }` のリテラルが無くなり、種類ごとの severity が1か所の `match` にあり、既存のテストがすべて通り、自己適用の出力が記録と同じ。
Shown by: check — `CARGO_BUILD_JOBS=4 cargo test` が 0 failed、`cargo run -q -- check` の出力が記録と一致、`rg -c 'kind: "' src/` が 0、`git diff --stat` に `docs/ir` と `Cargo.toml` が無い。
Left to the implementer: `enum` と構築の入口の名前。JSON の出力で kind を文字列に戻す場所。
Stop and hand back if: 既存のテストのどれかが、振る舞いを変えずには通せない（テストが内部の形を固定していた）。そのテストの名前と固定していた形を書いて返す。

## Step 0b — 入出力を1本にする（振る舞いを変えない）

Purpose: ファイルの読み込みと停止への変換を `read_utf8_file` の1つにまとめ、`normalize_path` を純粋な関数として定義し（呼び出しはステップ2）、`known_ids` の計算を1本にし、バッククォートの走査を1本にする。Specification: この段では無し。
Prerequisites: ステップ0a。
May change: `src/` の下のすべて、`tests/`（追従のみ）。
Done when: `fs::read` と `String::from_utf8` の対が src に1か所だけになり、`known_ids` の計算が1か所、バッククォートの走査が1か所になり、既存のテストがすべて通り、自己適用の出力が記録と同じ。
Shown by: check — `CARGO_BUILD_JOBS=4 cargo test` が 0 failed、`cargo run -q -- check` の出力が記録と一致、`rg -c 'from_utf8' src/` が 1。
Left to the implementer: 関数の名前と置き場。
Stop and hand back if: ステップ0a と同じ。

## Step 0c — 解析の指摘の出口と、引数の解析の移動（振る舞いを変えない）

Purpose: `parse_document` が解析中の指摘を返せるようにし、行の分類（コードブロックの外・中・gherkin の中・境界）と gherkin の1行の分類を純粋な関数に切り出して `ir.rs` と `terms.rs` の両方が使う形にし、引数の解析を `main.rs` から `lib.rs` の公開関数に移す。Specification: この段では無し。
Prerequisites: ステップ0b。
May change: `src/` の下のすべて、`tests/`（追従のみ）。
Done when: `parse_document` の戻り値に解析の指摘の欄（この段では常に空）があり、コードブロックの状態の判定が1か所になり、`terms.rs` に独自のコードブロックの反転が無く、`main.rs` の引数の解析が `lib.rs` の公開関数の呼び出しだけになり、既存のテストがすべて通り、自己適用の出力が記録と同じ。
Shown by: check — `CARGO_BUILD_JOBS=4 cargo test` が 0 failed、`cargo run -q -- check` の出力が記録と一致、`rg -c 'starts_with\("```"\)' src/` が 1。
Left to the implementer: 分類の関数の名前と戻り値の型。解析の指摘は `IrDocument` に欄を足して返す（組で返すと `parse_document` を呼ぶ既存のテスト約50か所の追従が要るので、この段では欄にする）。
Stop and hand back if: ステップ0a と同じ。

## Step 1 — 引数と停止のメッセージ

Purpose: `--help` と `--version` を受け、引数の誤りの場面を広げ、停止の標準エラーの1行目を決まった形にする。Specification: `docs/ir/cli.md#REQ-002`、`#REQ-004`、`#REQ-005`、`#REQ-006`、`#REQ-007`、`docs/ir/cli.md#TBL-001`、`#TBL-002`、`docs/ir/cli-environment.md#REQ-107`、`#REQ-109`、`#TBL-018`、`#TBL-020`。
Prerequisites: ステップ0c（`parse_args` と `read_utf8_file`）。
May change: `src/main.rs`、`src/lib.rs`、`tests/step1_cli.rs`、`tests/step1_config.rs`。
Done when: `--help` と `--version` が（ほかの引数を見ずに、`check` が無くても）標準出力に出て終了コード0で終わる。既存の `req_005_stderr_carries_the_stop_reason_text` は独自の文言（"ir directory not found"）を固定しているので、TBL-020 の形（相対パスと OS の誤りの文）に書き直す。引数が無い、値の無いオプション、同じオプションの2回目、`check` 以外の位置引数、`--config` の先が無いかディレクトリ、のそれぞれで終了コード2になり、標準出力は空。オプションは `check` の前後どちらでも受ける。停止の標準エラーの1行目が TBL-018 の文言、": "、TBL-020 の詳細の形で、英語で出る。
Shown by: test — `req_107_help_and_version_exit_zero_without_check`、`req_107_help_wins_over_argument_errors`、`req_004_no_arguments_stops`、`req_004_option_without_value_stops`、`req_004_repeated_option_stops`、`req_004_config_pointing_to_directory_stops`、`req_002_options_before_or_after_check`、`req_005_stderr_first_line_has_the_reason_wording`（TBL-018 の4つの文言）、`req_005_stderr_detail_path_is_relative`。
Left to the implementer: 使い方の文字列と版の文字列の中身（版は `Cargo.toml` の値）。
Stop and hand back if: 引数の順序や `--help` の優先で IR が定めていない組み合わせ（例: `--version --help`）に出会い、結果を選べない（IR の定めどおり、`--help` か `--version` があればほかを見ない、で決まらない場合）。

## Step 2 — 設定、基準のディレクトリ、パスの正規化

Purpose: 設定の誤りの場面を広げ、空の設定ファイルを既定にし、パスの正規化を1か所に置き、置き場がディレクトリでないときとファイルの種類が取れないときに停止する。Specification: `docs/ir/config.md#REQ-012`、`#REQ-014`、`#REQ-018`、`#REQ-019`、`docs/ir/base-directory.md#REQ-110`、`#TBL-003`、`docs/ir/output.md#TBL-006`（path）、`docs/ir/sources.md#REQ-057`（正規化の後で置き場と比べる）。
Prerequisites: ステップ1（停止の1行目の形。設定の誤りの詳細は TBL-020 の形で出す）。ステップ0b の `normalize_path` を使う。
May change: `src/config.rs`、`src/lib.rs`、`src/sources.rs`、`src/tests_discovery.rs`（glob の構文の検査を `Config::parse` へ移す）、`tests/step1_config.rs`。
Done when: YAML として読めない、同じキーの2回目、null の値、絶対パス、`vague_words` の重複、glob として読めない要素、のそれぞれで設定の誤りで停止する。空の設定ファイル（0バイト、注釈だけ）は `--config` で指しても既定の値で進む。空の一覧は受ける。設定の値と出典のパスは、末尾の "/"、先頭の "./"、途中の "/./" と連続する "/"、"\" を正規化してから比べられ、出力の path は正規化した置き場と文書名を "/" でつないだ形になる（"docs/ir/" と書いても "docs/ir//a.md" にならない）。".kotowari" という名前のファイルは探索で無視される。置き場がファイルのときは停止する。
Shown by: test — `req_014_unparsable_yaml_stops`、`req_014_duplicate_key_stops`、`req_014_null_value_stops`、`req_014_absolute_path_stops`、`req_014_duplicate_vague_word_stops`、`req_014_invalid_glob_stops`、`req_012_empty_config_uses_defaults`、`req_012_comment_only_config_uses_defaults`（空の一覧は既存の `req_016_empty_list_means_none` が守る）、`req_110_trailing_slash_in_config_is_normalized_in_path`、`req_110_dot_segments_are_folded`、`req_018_place_that_is_a_file_stops`、`tbl_003_kotowari_file_is_ignored_in_search`。
Left to the implementer: `Config` の Raw 型を、キーの欠落と null を区別する形（`Option<Option<T>>` か別の型）にする方法。同じキーの2回目の検出は、最上位と入れ子の両方が対象。YAML の読み手が検出するならそれを使う。
Stop and hand back if: 使っている YAML の読み手が同じキーの2回目を検出できない（自前の行の走査で代えない。検出の範囲の判断が要るので止まって返す）。

## Step 3 — 文書の読み込み: 拡張子、シンボリックリンク、BOM、コードブロックの境界

Purpose: 読む文書の選び方を IR のとおりにし、BOM を読み飛ばし、コードブロックの境界を ``` と ~~~ の3つ以上で判定し、閉じ忘れを unclosed_code_block にする。Specification: `docs/ir/ir-document.md#REQ-033`、`#REQ-040`、`#TBL-010`、`docs/ir/ir-input.md#REQ-111`、`#REQ-112`、`docs/ir/CONTEXT.md`（`コードブロック`、`除外`）。
Prerequisites: ステップ2。BOM の読み飛ばしはステップ0b の `read_utf8_file` に足すので、IR、判断の記録、ADR、テストのファイルのすべてに同時に効く。
May change: `src/ir.rs`、`src/lib.rs`（`read_utf8_file`）、`src/terms.rs`（行の分類の利用）、`tests/step2_ir.rs`、`tests/step3_sources_terms.rs`。
Done when: ".MD" とサブディレクトリの文書は読まれず、ファイルのシンボリックリンクの文書は読まれる。種類が取れない要素で停止する。BOM 付きの文書、設定ファイル、判断の記録、ADR、テストのファイルが UTF-8 として読まれ、題名と見出しが認識される。UTF-8 でない判断の記録と ADR で停止する（TBL-001）。"~~~" で囲んだブロックの中は検査されない。"````" で開いたブロックは "```" で閉じない。閉じないブロックは開始の行に unclosed_code_block（detail は開始の行の文字そのまま）が出て、そこから文書の終わりまでは gherkin でも検査されない。中身が空の文書は0行で missing_title が出る。
Shown by: test — `req_033_uppercase_md_is_not_read`、`req_033_file_symlink_is_read`、`req_111_bom_is_skipped_in_ir_config_records_adr_and_tests`、`tbl_001_non_utf8_records_or_adr_stops`、`req_040_tilde_fence_is_a_code_block`、`req_040_longer_fence_needs_same_or_longer_close`、`req_112_unclosed_code_block_is_an_error`、`req_112_unclosed_gherkin_block_is_not_checked`、`tbl_010_empty_document_has_zero_lines_and_missing_title`。
Left to the implementer: 行の分類の純粋な関数（ステップ0）に囲みの文字と長さを持たせる形。
Stop and hand back if: `~~~` を境界にすると既存の IR（`docs/ir/*.md`）の中で意図せずブロックになる行がある（計画を書いた時点で行頭の `~~~` は0件と確かめてある。あれば IR の側の問題なので止まって返す）。

## Step 4a — 項目の行の形

Purpose: 一覧の行の種類、深い見出し、detail の生の行、duplicate_field の数え方、種類の行が無い要求の文の検査、duplicate_id の1つ目を IR のとおりにする。Specification: `docs/ir/ir-items.md#REQ-043`、`#REQ-044`、`#REQ-045`、`#REQ-047`、`docs/ir/findings.md#REQ-032`。
Prerequisites: ステップ3。
May change: `src/ir.rs`、`tests/step2_ir.rs`、`tests/step5_findings.rs`。
Done when: "* "、"+ "、"1. "、"-" だけの行が unknown_field（detail は読んだ行そのまま。字下げを含む）になり、"#### " が unknown_heading になり、形に合わない見出しの下の行は項目として読まれない。知っている行の3つ目で duplicate_field が2件、知らない行の重複は unknown_field だけ。"- 種類:" の無い要求に文が無ければ missing_statement。duplicate_id の1つ目はパスのバイト順で先の文書。
Shown by: test — `req_044_star_plus_numbered_and_bare_dash_lines_are_unknown_fields`、`req_044_detail_is_the_raw_line`、`req_043_deeper_heading_is_unknown_heading`、`req_043_lines_under_unknown_heading_are_not_an_item`、`req_045_third_known_line_gives_two_duplicates`、`req_045_unknown_line_repeated_gives_only_unknown_field`、`req_047_requirement_without_kind_line_needs_statement`、`req_032_first_occurrence_is_bytewise_first_path`。
Left to the implementer: `ItemBuilder` に生の行を持たせる形。
Stop and hand back if: 既存の IR（`docs/ir/*.md`）に、新しい規則で unknown_field か unknown_heading になる行がある（IR の側の問題）。

## Step 4b — gherkin の行の形と ID の定義

Purpose: gherkin の行の検査（字下げを除く、Feature を許さない、タグは直前の行だけ、タグの行の検査）、`@id` の形の検査と ID の集合の作り方、ID の形でない参照の値を IR のとおりにする。Specification: `docs/ir/ir-references.md#REQ-052`、`#REQ-053`、`#REQ-054`、`#REQ-113`、`#REQ-114`、`docs/ir/sources.md#REQ-059`、`docs/ir/terms.md#TBL-013`。
Prerequisites: ステップ4a。
May change: `src/ir.rs`、`src/lib.rs`（ID の集合）、`tests/step2_ir.rs`、`tests/step3_sources_terms.rs`。
Done when: gherkin の中で2字下げのステップが正しく読まれ、"Feature:"、"Background:"、"Scenario Outline:"、"Examples:"、データ表の行が invalid_gherkin_line になり、タグの行と "Scenario:" の間に行があるとタグは結び付かず（missing_tag）、結び付かないタグの行の知らないタグも unknown_tag になり、タグの行の "@" で始まらない語も unknown_tag になる。"@id=EX1" は invalid_id（line はタグの行）で、missing_tag は出ず、その ID は定義に数えられず（印から参照すると unresolved_reference）、そのシナリオの missing_source の detail は "Scenario:" の行の文字になる。"### REQ-1: x" も定義にならない。"- 定義: foo" は unresolved_reference。ステップの行のバッククォートの ID も存在を検査される。
Shown by: test — `req_113_indented_steps_are_recognized`、`req_113_feature_and_examples_lines_are_invalid`、`req_113_tag_line_binds_only_when_immediately_before_scenario`、`req_052_unbound_tag_line_is_still_checked`、`req_052_word_without_at_in_tag_line_is_unknown_tag`、`req_114_malformed_id_tag_is_invalid_id_and_not_defined`、`req_114_malformed_id_scenario_missing_source_detail_is_scenario_line`、`req_114_malformed_heading_is_not_defined`、`req_054_non_id_definition_value_is_unresolved`、`req_054_backtick_id_in_step_is_checked`。
Left to the implementer: gherkin の1行の分類の関数（ステップ0c）の戻り値の形。`known_ids` を1本にした関数の名前。
Stop and hand back if: 既存の IR（`docs/ir/*.md`）の具体例に、新しい規則で invalid_gherkin_line になる行がある（計画を書いた時点で、具体例の行はすべてタグ・Scenario:・ステップで始まることを確かめてある。あれば IR の側の問題）。

## Step 5 — 出典と用語: 判断の記録の見分け方、出典の行、バッククォート、曖昧語、文書名の参照、用語集の表

Purpose: 判断の記録を決定の節の見出しで見分け、決定の番号の形と節の終わりを IR のとおりにし、source_invalid の line を出典の行にし、バッククォートの空白除去と空の囲みと奇数の扱い、曖昧語の最長一致、".md" の直後の文字と奇数の二重引用符、用語集の表の範囲と glossary_invalid を実装する。Specification: `docs/ir/sources.md#REQ-061`、`#REQ-115`、`#TBL-012`、`docs/ir/terms.md#REQ-064`、`#REQ-067`、`#TBL-014`、`docs/ir/terms-form.md#REQ-116`、`#REQ-117`、`docs/ir/CONTEXT.md`（`判断の記録`、`決定の番号`、`決定の節`）。
Prerequisites: ステップ4b。
May change: `src/sources.rs`、`src/terms.rs`、`src/ir.rs`（用語集の表の読み方）、`tests/step3_sources_terms.rs`。
Done when: 決定の節の見出しを持つファイルだけが判断の記録として番号で照合され、持たないファイルは見出しで照合される。"AB1" は決定の番号でない。"### " の小見出しは節を終えない。source_invalid の line が "- 出典:" の行（タグの行、表の行）になる。"` IR `" は "IR" として照合され、"``" は detail "``" の unknown_term、奇数の行は unclosed_backtick で用語と ID の検査を飛ばし曖昧語は検査する。"など" と "などの" の両方が曖昧語のとき "などの" で1件。"a.mdX" は参照でない。閉じない二重引用符の後ろは参照を拾わない。用語集の2つ目の表は用語にならず、ヘッダの列名が違う表しか無ければ glossary_invalid（line は null）で用語0語として続く。
Shown by: test — `tbl_012_file_with_decision_sections_is_a_records_file`、`tbl_012_file_without_decision_sections_matches_headings`、`tbl_012_two_letter_prefix_is_not_a_decision_number`、`req_061_subheading_does_not_end_a_section`、`req_115_source_invalid_line_is_the_source_line`、`req_064_backtick_content_is_trimmed`、`req_064_empty_backticks_are_unknown_term`、`req_116_odd_backticks_skip_terms_but_check_vague_words`、`req_067_overlapping_vague_words_longest_match_once`、`tbl_014_md_followed_by_letter_is_not_a_reference`、`tbl_014_unclosed_quote_hides_the_rest_of_the_line`、`req_117_second_table_is_not_glossary`、`req_117_glossary_without_proper_table_is_invalid`。
Left to the implementer: 判断の記録の見分けを読み込み時に1回だけ行う構造。バッククォートの走査を1本にした関数の名前。
Stop and hand back if: 既存の判断の記録（`experiments/003-cli/brainstorm/records.md`）と ADR が新しい見分け方で違う扱いになる（計画を書いた時点で、決定の節の見出しを持つのは records.md だけと確かめてある）。

## Step 6 — テストの数え方と印

Purpose: 属性の末尾の要素が test の関数を数え、マクロの中でも通常の関数と同じ規則で最上位だけ数え、印は1行で閉じ、invalid_marker の detail を生の行に、印の指摘の line を印の行にし、走査でファイルのシンボリックリンクと隠しファイルを読む。Specification: `docs/ir/test-discovery.md#REQ-079`、`#REQ-081`、`#REQ-083`、`#TBL-017`、`docs/ir/test-markers.md#REQ-072`、`#REQ-118`、`#TBL-016`、`docs/ir/coverage.md#REQ-085`、`docs/ir/config.md#REQ-019`。
Prerequisites: ステップ4b（ID の集合）。ステップ5とは独立。
May change: `src/tests_discovery.rs`、`tests/step4_test_discovery.rs`。
Done when: "#[ test ]"、"#[core::prelude::v1::test]"、"#[tokio::test]" の関数が数えられ、`tests.rust.attributes` はパス全体の一致のまま。proptest! の中の入れ子の関数は数えず、本体の先頭のコメントの印が結び付き、空の印に invalid_marker が出る。行をまたぐ印は invalid_marker（line は "@kotowari[" の行、detail は生の行）。unresolved_reference の line は印の行。".RS" は問い合わせの無い言語。"- 検証:" の行が無い要求には requirement_without_test が出ない（verification_missing だけ）。既存の `req_054_marker_unresolved_reference_reports_fn_line` は関数の行を固定しているので、印の行に書き直す。ファイルのシンボリックリンクのテストは読まれ、ディレクトリのリンクは辿らず、".foo.rs" は glob が当てれば読まれる。
Shown by: test — `tbl_017_attribute_path_ending_in_test_is_counted`、`tbl_017_nested_function_in_macro_is_not_counted`、`tbl_017_macro_function_body_marker_binds`、`req_072_marker_spanning_lines_is_invalid`、`req_072_detail_is_the_raw_line`、`req_118_unresolved_reference_line_is_the_marker_line`（既存の `req_054_marker_unresolved_reference_reports_fn_line` を置き換える）、`req_085_requirement_without_verification_line_gets_no_coverage_finding`、`req_081_uppercase_extension_has_no_query`、`req_079_file_symlink_is_read`（既存の `req_079_symlink_is_not_followed` を書き直す）、`req_019_hidden_file_matched_by_glob_is_read`。
Left to the implementer: 属性のパスの末尾の要素の取り方（tree-sitter の節の読み方）。マクロの中の再解析に `has_error()` の検査を入れる場所。
Stop and hand back if: tree-sitter-rust の構文木で `#[ test ]` の属性のパスが取れない。

## Step 7 — 指摘の種類と行の表、自己適用、対応の表

Purpose: 新しい5つの種類を誤りの表と重大度に組み込み、指摘の line を TBL-019 のとおりにし（ステップ0a の `enum` の `match` に写す）、kotowari 自身の IR とテストにかけて指摘0にし、対応の表を更新する。Specification: `docs/ir/findings.md#TBL-008`、`docs/ir/finding-order.md#REQ-027`、`#TBL-019`、`docs/ir/output.md#TBL-006`、`docs/ir/coverage.md#REQ-085`。
Prerequisites: ステップ5と6。
May change: `src/`（line の決め方）、`tests/step5_findings.rs`、`tests/step6_output.rs`、`docs/trace.md`。
Done when: glossary_invalid の line が null で、新しい種類（unclosed_code_block、invalid_gherkin_line、invalid_id、unclosed_backtick）と、印・出典・duplicate_id の line が TBL-019 のとおり。`cargo run -- check` をリポジトリ直下で走らせて指摘0、終了コード0。`docs/trace.md` の「要求の確かめ方」の表に、要求120件すべての行があり（新しい要求はテスト名を、`review` の要求 REQ-108、REQ-109、REQ-120、REQ-121 は読む場所を手で書く）、ミューテーションの節はそのまま残す（実験者が cycle の外で更新する）。
Shown by: test — `req_027_glossary_invalid_has_null_line`、`tbl_019_unclosed_code_block_line_is_the_opening_line`、`tbl_019_invalid_gherkin_line_and_invalid_id_lines`、`tbl_019_unclosed_backtick_line`、`tbl_019_marker_findings_line_is_the_marker_line`、`tbl_019_source_invalid_line_is_the_source_line`。それに check — `CARGO_BUILD_JOBS=4 cargo run -q -- check` の出力が `{"files":23,"lines":...,"findings":[],"counts":{}}` で終了コード0。artifact — `docs/trace.md`（`grep -c '^| REQ-' docs/trace.md` が `rg -c '^### REQ-' docs/ir/*.md` の合計と同じ）。
Left to the implementer: `run_check` の整列と集計を切り出した純粋な関数の名前。
Stop and hand back if: 自己適用で IR の側の不備（新しい規則で IR 自身が引っかかる）が出た。

## Verification map

| IR の文書 | 確かめるステップ |
|---|---|
| `cli.md`、`cli-environment.md` | 1 |
| `config.md`、`base-directory.md`、`output.md`（TBL-006） | 2 |
| `ir-document.md`、`ir-input.md` | 3 |
| `ir-items.md`、`findings.md`（REQ-032） | 4a |
| `ir-references.md`、`terms.md`（TBL-013）、`sources.md`（REQ-059） | 4b |
| `ir-missing.md` | 変更なし（既存のテストのまま） |
| `sources.md`（REQ-059 を除く）、`terms.md`（TBL-013 を除く）、`terms-form.md` | 5 |
| `test-discovery.md`、`test-markers.md`、`coverage.md` | 6 |
| `findings.md`（TBL-008）、`finding-order.md` | 7 |
| `cli-scope.md`、`form-contract.md`、`decision-records.md`（review の要求） | 7（`docs/trace.md` に読む場所） |

## Left to the implementer

- 関数と型の名前、モジュールの中の切り方（判断の記録 D1）
- 内部の誤りの型の設計（`StopReason` に理由と詳細を分けて持たせるかどうか。TBL-018 と TBL-020 を型で表すなら分ける）
- 使い方の文字列の文面

## Stop conditions

計画全体で、次のときは作業を止めて返す。

- IR か契約に書かれていない振る舞いを決めないと進めない
- IR と契約が食い違う、または IR の中で食い違う
- 新しい実行時の依存が要る
- 既存のテストを書き直すとき、どの決定で改めたかが判断の記録から見つからない
- 処理を10分の上限に収めるために分割できない（バックグラウンドに逃がさない）

## Out of scope

- `render`、`trace`、`query`（`docs/ir/cli.md#REQ-008`）
- IR の形を設定で変えること（判断の記録 R9、U39）
- 判断の記録と IR の置き場の移動（判断の記録 R7）
- Rust 以外の言語の問い合わせ（`docs/ir/test-discovery.md#REQ-081`、判断の記録 U37）
- 配布、ライセンス、CI（判断の記録 U38）
- ミューテーションテストの走らせ直しと見逃しの分類、対応の表の最終確認（実験者が cycle の外で行う。実験003の D-21）
- 性能の要求（判断の記録 R8）
