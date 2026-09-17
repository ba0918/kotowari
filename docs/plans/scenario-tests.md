# 実装計画: scenario-tests

## Goal

`kotowari check` が、印のあるテストの無い具体例（シナリオ）を scenario_without_test として出し、具体例の印がその要求の分も満たすようになり、このリポジトリの 68 本の具体例すべてに対応するテストが印で結び付いて check が 0 件になる。

## Specification

正本は仕様 IR `docs/ir/*.md`（以下「IR」）。この計画が対象にする項目は次のとおり。

- `docs/ir/coverage.md#REQ-137`（テストのない具体例）、`#REQ-085`（テストのない要求。具体例の印でも満たされる文に改訂）、`#REQ-087`（問い合わせの無い言語の対応。scenario_without_test も消す側）、具体例 EX-121〜EX-125
- 指摘の形: `docs/ir/findings.md#TBL-008`（scenario_without_test の detail はシナリオの ID）、`docs/ir/finding-order.md#TBL-019`（line はタグの行）
- 既存の要求で境界を決めているもの: `docs/ir/test-markers.md#REQ-071`〜`#REQ-078`（印の構文と結び付け）、`docs/ir/ir-references.md#REQ-054`（存在しない ID）、`#REQ-114`（invalid_id と存在する ID の集合）、`docs/ir/findings.md#REQ-032`（ID の重複と1つ目）、`docs/ir/ir-items.md#REQ-048`、`#REQ-049`（検証の行の欠けと値）、`docs/ir/test-discovery.md#REQ-079`（テストのファイル）、`docs/ir/skill-references.md#REQ-125`（references の種類の表と本体の一致）、`docs/ir/CONTEXT.md` の用語 `シナリオ`、`印`、`テスト`、`テストのファイル`、`問い合わせの無い言語`
- 移行の対象: IR の全文書の `## 具体例` にある `@id=EX-nnn` のシナリオ（2026-09-17 時点で 68 本。各シナリオの Given / When / Then がテストの入力と期待。数はステップ1の実測を正とする）

判断の記録は `docs/decision/records/2026-09-17-scenario-tests.md`（A1〜A15、U2）。IR の形の契約は `docs/decision/records/ir-form.md`（以下「契約」）。

IR の読み方は前の計画と同じ。要求の `- 検証:` が `review` の要求はテストを求めない。IR の各文書の `## 具体例` のシナリオは、その要求の成功の条件と反例で、テストの入力と期待の元にする。

IR と契約と判断の記録は実装の間は読み取り専用である。IR の書き換えが必要だと思ったら、書き換えずに止まって返す。IR の沈黙は実装者が決めてよい意味ではない（`docs/ir/cli-environment.md#REQ-120`）。

この計画の中のテストの置き方と名前の付け方は、IR ではなくこの計画の約束である。

## Approach and why

既存の実装（`src/` 約4,300行、テスト 437 件）を育てる。作り直さない。新しい依存は足さない。

いま要求とテストの対応は `src/tests_discovery.rs` の `discover_and_check` が持つ。テストのファイルから拾った印の ID を1つの集合（`all_marker_ids`）にまとめ、検証が review 以外の要求の ID がその集合に無ければ requirement_without_test を出す。シナリオの `@about` は `src/ir.rs` の `Item::Scenario` が `about` として持ち、タグの行は `tag_line`。

変更は2つの性格に分かれる。

1. **検査の実装**（REQ-137、REQ-085 と REQ-087 の改訂）: 印の集合から、(a) 要求の ID が直接あるか、または `@about` にその要求を持つシナリオの ID があるか、で requirement_without_test を判定し、(b) REQ-137 の適用条件を満たすシナリオの ID が無ければ scenario_without_test を出す。指摘の種類 `ScenarioWithoutTest` を `src/lib.rs` の `finding_kinds!` に足す。`FindingKind::ALL` は macro が作るので、足した時点で `tests/step7_skill_references.rs` の REQ-125 のテストが `skills/kotowari/references/findings.md` の表との不一致で落ちる。同じステップで表に行を足して戻す（担当は implementer）
2. **移行**（判断の記録 A5、A13）: IR の 68 本の具体例それぞれについて、その Given / When / Then を観測している既存のテストを見つけ、その印に EX の ID を足す。観測しているテストが無い具体例には新しくテストを書く。終端は `cargo run -q -- check` の指摘 0 件。この作業は「具体例の Given / Then とテストの assert の突き合わせ」で、判断は「このテストはこの場面を観測しているか」だけ。観測していると言えないなら足さず、新しく書く

読む順は 1 → 2。2 は 1 の検査が出す約 68 件の scenario_without_test を消していく作業で、1 に依存する。

テストの置き方と名前（この計画の約束。前の計画と同じ）:

- ステップ1のテストは `tests/step4_test_discovery.rs`（要求とテストの対応の既存テスト `req_085_*`、`req_087_*`、`req_088_*` がある）に足す。fixture の作り方は同じファイルの既存テストに倣う（一時ディレクトリに `.kotowari/config.yaml`、`docs/ir`、`docs/decision/records`、`docs/decision/adr`、`tests/` を置いて CLI を `check` で起動し JSON を読む）
- 各テスト関数の直前の行に `// @kotowari[REQ-137, EX-121]` の形の印を置く（`skills/kotowari/references/mark.md` の形）。ステップ1から、確かめる具体例の ID も印に含める
- テスト関数の名前は、確かめる IR の ID を先頭に付けた snake_case
- テストは RED → GREEN → REFACTOR の順で書き、各段で `CARGO_BUILD_JOBS=4 cargo test` を走らせる。「指摘が出ない」テストは、検査を入れた後に「出るはずの指摘」を期待に一時的に置いて RED を見てから戻し、その RED の出力を終端報告に引用する。反転した状態はコミットしない
- 既存のテストが新しい仕様と食い違うときは、新しい仕様に合わせて書き直し、コミットの本文に改めた決定を書く。REQ-085 の判定は「要求の ID を含む印」を含むので、既存の `req_085_*` は変更なしで通る見込み
- ステップ2で既存テストの印に EX を足すときは、そのテストの Given と Then を IR のシナリオと突き合わせ、テストの assert が Then を観測していることを確かめてから足す。観測していないテストに足さない
- コミットは意図ごとに分け、`git add <path>` でファイルを1つずつ。件名は `<type>: <件名>` の形で日本語。帰属の行は付けない。ステップ2は IR の文書ごと（`docs/ir/terms.md` の具体例の分、など）にコミットを分けてよい

## Scope of change

変えてよいのは、リポジトリ直下の次のものだけである。

- `src/` の下のすべて
- `tests/` の下のすべて
- `skills/kotowari/references/findings.md`（種類の表に1行。ステップ1）
- `skills/kotowari/references/mark.md`（EX の印と REQ-087 の文言。ステップ1）
- `Cargo.lock`（依存の更新はしない。ビルドで変わった分だけ）

`Cargo.toml` は変えない。`docs/ir/`、`docs/decision/`、`docs/plans/`、`docs/trace.md`、`docs/spec/`、`skills/kotowari/SKILL.md`、上の2つ以外の references、`lefthook.yml`、`.kotowari/`、`experiments/`、`fixtures/` は変えない。インストール済みスキル（`~/.claude/skills/kotowari/`）はリポジトリの外で、マージ後に主セッションが写す。

## Step order and prerequisites

ステップ1 → 2 → 3 の順に行う。

## Step 1 — 具体例ごとの対応の検査

Purpose: 印の無い具体例を scenario_without_test にし、具体例の印が要求の分も満たすようにする。Specification: `docs/ir/coverage.md#REQ-137`、`#REQ-085`、`#REQ-087`、`docs/ir/findings.md#TBL-008`、`docs/ir/finding-order.md#TBL-019`、`docs/ir/findings.md#REQ-032`、`docs/ir/ir-references.md#REQ-114`、`docs/ir/skill-references.md#REQ-125`、具体例 EX-121〜EX-125。
Prerequisites: なし。始める前に `CARGO_BUILD_JOBS=4 cargo test` を1回走らせて全件通ることと、`CARGO_BUILD_JOBS=4 cargo run -q -- check` の出力を記録する。この時点の終了コードは1で、"findings" は requirement_without_test が1件（REQ-137）のはずである。違えば止まって返す。
May change: `src/lib.rs`（`finding_kinds!` に `ScenarioWithoutTest`）、`src/tests_discovery.rs`、`src/ir.rs`（シナリオの `@about` を引く補助が要る場合）、`tests/step4_test_discovery.rs`、`tests/step2_ir.rs`（fixture にシナリオを持ち指摘 0 件を assert する既存テストの印に EX を足す分だけ）、`skills/kotowari/references/findings.md`、`skills/kotowari/references/mark.md`（印に EX を書けることと、EX の印が @about の要求の分も満たすこと、REQ-087 の文言の追従）。
Done when: REQ-137 の文のとおりに scenario_without_test が出る（severity error、line と detail は TBL-019 と TBL-008 のとおり）。REQ-085 は改訂後の文のとおり、`@about` にその要求を持つシナリオの ID を含む印でも満たされる。逆（要求の印だけ）では scenario_without_test が出る。REQ-137 が「出さない」と定める場合と、同じ ID のシナリオの扱い（1つ目の `@about`、指摘は1つ目のタグの行に1件）は文のとおりで、この計画は写さない。問い合わせの無い言語のテストのファイルの印も数える。REQ-125 のテストが通る。fixture にシナリオを持ち指摘 0 件（または終了コード 0）を assert する既存テスト（`tests/step4_test_discovery.rs` の `req_124_four_digit_id_is_valid_in_heading_tag_and_marker` と `req_124_leading_zero_and_short_ids_are_rejected`、`tests/step2_ir.rs` の `req_059_source_tag_present_but_only_commas_does_not_report_missing_source`、`req_112_unclosed_gherkin_block_with_multiple_scenarios_excludes_items`、`req_113_step_right_after_tag_line_reports_both_lines`、`req_113_tags_do_not_leak_into_the_next_untagged_scenario`）は、新しい検査で scenario_without_test が出て落ちうる。落ちたものは fixture のテスト側の印に EX の ID を足すか、assert を種類で絞る形に直し、コミットの本文に REQ-137 で改めたと書く。`skills/kotowari/references/mark.md` に、印に EX の ID を書けること、EX の印がその @about の要求の分も満たすこと（逆は無い）、Rust 以外の言語では scenario_without_test も消す側に数えることを足す。このリポジトリで `cargo run -q -- check` を走らせると、REQ-137 の適用条件を満たし印に EX の無い具体例の分だけ scenario_without_test が出て、それ以外の指摘は 0 件。その一覧（EX の ID、文書、タグの行）と件数を終端報告に書く（ステップ2の入力）。
Shown by: test — `req_137_scenario_without_marker_is_an_error`（EX-121。要求の印はあるが EX の印が無い）、`req_085_scenario_marker_covers_its_requirement`（EX-122）、`req_137_review_only_scenario_is_not_required`（EX-123）、`req_087_marker_in_a_file_without_query_feeds_scenario_coverage`（EX-124。既存の `req_087_*` と同じく `tests.files` を `tests/**/*.py` だけにする）、`req_137_scenario_about_a_table_only_is_not_required`（EX-125）、`req_137_requirement_without_verification_line_does_not_count`（`@about` の要求に `- 検証:` が無い。verification_missing だけが出る）、`req_137_scenario_without_id_is_not_reported`（`@id` の無いシナリオ。missing_tag だけが出る）、`req_137_duplicate_scenario_uses_the_first_about`（同じ EX の ID が2文書にあり、パスのバイト順で1つ目の `@about` は review の要求 REQ-002、2つ目は unit の要求 REQ-001。EX の印は無い。duplicate_id は出るが scenario_without_test は出ない。1つ目を使わない実装なら出るので弁別できる）、`req_137_duplicate_scenario_reports_once_on_the_first`（同じ EX の ID が2文書にあり、どちらの `@about` も unit の要求で、EX の印は無い。scenario_without_test が1つ目のタグの行に1件だけ出る。A16）、`req_085_duplicate_scenario_marker_uses_the_first_about`（同じ EX の ID が2文書にあり、1つ目の `@about` は REQ-001、2つ目は REQ-003。EX の印だけを持つテストがある。REQ-001 の requirement_without_test は消え、REQ-003 のは残る）、`req_137_requirement_with_invalid_verification_value_does_not_count`（`- 検証: e2e` の要求だけを `@about` に持つシナリオ。verification_invalid だけが出る）、`req_137_scenario_with_invalid_id_is_not_reported`（`@id=BADID` のシナリオ。invalid_id だけが出る。`tests/step5_findings.rs` の `tbl_019_invalid_gherkin_line_and_invalid_id_lines` と同じ fixture の形）、`req_137_scenario_about_an_unknown_id_is_not_required`（`@about` が存在しない REQ-999 だけ。unresolved_reference だけが出る）、`req_137_one_marker_may_name_several_scenarios`（`@kotowari[EX-201, EX-202]` の1本と、EX-201 をもう1本が挙げる。どちらにも scenario_without_test が出ない）。既存の `req_085_fixture_has_no_uncovered_requirement`、`req_085_requirement_without_verification_line_gets_no_coverage_finding`、`req_087_unknown_language_only_feeds_coverage`、`req_088_empty_ir_still_checks_tests` が変更なしで通る。`req_125_findings_reference_kinds_match_the_code` が通る。
Left to the implementer: シナリオの ID から `@about` を引く表の持ち方と置き場（`discover_and_check` に渡す形。`collect_known_ids` の隣に集める関数を切ってもよい）。fixture の文面。
Stop and hand back if: 同じ ID のシナリオの「1つ目」を REQ-032 の並び（パスのバイト順、同じ文書では行の小さい方）で決められない。落ちた既存テストの原因が scenario_without_test 以外にある。

## Step 2 — 既存の具体例への印付けと、足りないテスト

Purpose: IR の全具体例に、その Given / When / Then を観測するテストを印で結び付け、scenario_without_test を 0 件にする。Specification: IR の全文書の `## 具体例`（各シナリオの Given / When / Then）、`docs/ir/coverage.md#REQ-137`、`docs/ir/test-markers.md#REQ-071`〜`#REQ-078`。
Prerequisites: ステップ1。ステップ1の終端で記録した scenario_without_test の一覧（EX の ID、文書、タグの行）が入力。
May change: `tests/` の下のすべて（既存の印への EX の追加と、新しいテスト）。`src/` は変えない（検査の直しが要ればステップ1に戻る）。
Done when: `CARGO_BUILD_JOBS=4 cargo run -q -- check` の指摘が 0 件（終了コード0）。「観測している」の判定は次の3つをすべて満たすこと: (1) シナリオの When が "kotowari check" を実行する場面なら、テストも CLI を `check` で起動している（ライブラリの関数を直接呼ぶテストには結び付けない）。(2) Given の各行に当たる入力を fixture が持つ。(3) Then の各行（And を含む）に当たる assert がある。候補は「その EX の `@about` の要求の ID を印に含む既存のテスト」で、その中に3つを満たすものが無ければ新しく書く。1本のテストを複数の EX に結び付けてよいが、各 EX の Then をそれぞれ assert していること。新しいテストは、その要求を既に検査している既存の `tests/stepN_*.rs` に置く。中間の到達点として、IR の文書ごとに、その文書の具体例の scenario_without_test が 0 件になった時点でコミットする。終端報告に、EX ごとに「結び付けたテスト名」と「既存 / 新規」の一覧を書く。
Shown by: check — `CARGO_BUILD_JOBS=4 cargo run -q -- check --format text`（何も出ない）、`CARGO_BUILD_JOBS=4 cargo test`（全件通過）。新しく書いたテストは test — 名前は要求の ID を先頭に置き、印に EX を含める。RED は「そのテストが無いと scenario_without_test が出る」ことで、ステップ1の基準線で観測済みなので、テストごとの反転は要らない。
Left to the implementer: 突き合わせの進め方（文書ごと、要求ごと）、コミットの分け方（文書ごとを推奨）、新しいテストの fixture の文面。判断の記録 A13 のとおり、既存テストへの印付けの突き合わせは bulk-executor（別の安いモデル）に文書ごとに回してよい。ただし「観測している」と言えるかの判定と、新しいテストを書く作業は実装役が行う。
Stop and hand back if: シナリオの Given / When / Then を、今の実装では観測できない（仕様と実装が食い違っている、または Then が観測可能でない）。その EX の一覧と理由を添えて止まる。観測できないものが一部でも、他の EX の作業は文書ごとにコミットしてから止まる。

## Step 3 — 終端の確認

Purpose: 全体が仕様どおりで、このリポジトリで check が 0 件であることを確かめる。Specification: 上の全部。
Prerequisites: ステップ1〜2。
May change: なし（直しが要れば該当のステップに戻る）。
Done when: `CARGO_BUILD_JOBS=4 cargo test` が全件通り、`CARGO_BUILD_JOBS=4 cargo run -q -- check` の終了コードが0で "findings" が空。`skills/kotowari/references/findings.md` の種類の表に scenario_without_test の行がある。
Shown by: check — `CARGO_BUILD_JOBS=4 cargo test`、`CARGO_BUILD_JOBS=4 cargo run -q -- check`（終了コード0）、`CARGO_BUILD_JOBS=4 cargo run -q -- check --format text`（何も出ない）。
Left to the implementer: なし。
Stop and hand back if: 種類を問わず、ステップ1の基準線に無い指摘が残る。

## Verification map

| 仕様の項目 | 証拠を出すステップ |
|---|---|
| coverage.md REQ-137、REQ-085、REQ-087、EX-121〜125 | ステップ1 |
| findings.md TBL-008、finding-order.md TBL-019 の新しい行 | ステップ1（各テストの line と detail の assert） |
| skill-references.md REQ-125 | ステップ1（既存の `req_125_*` が通る） |
| IR の全具体例の対応（判断の記録 A5、A13） | ステップ2、3 |

## Left to the implementer

各ステップの欄のとおり。共通: Rust の型・関数の名前、fixture の文面。

## Stop conditions

一般の4条件に加えて、各ステップの「Stop and hand back if」。IR・契約・判断の記録を変えたくなったら、変えずに止まって返す。

## Test command

`CARGO_BUILD_JOBS=4 cargo test`。check の確認は `CARGO_BUILD_JOBS=4 cargo run -q -- check`（JSON）と `--format text`。

## Out of scope

- インストール済みスキル `~/.claude/skills/kotowari/references/findings.md` への同期（マージ後に主セッションが行う）
- IR、契約、判断の記録の変更。具体例の文言の直し（観測できない場面は止まって返す）
- U2（具体例の無い unit の要求に具体例を求める指摘）
- 変異テストの取り込み（別の壁打ち）
