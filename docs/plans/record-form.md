# 実装計画: record-form

## Goal

`kotowari check` が判断の記録を読み、`## Context` を持つ記録で必須の補足の行が無い決定と知らない名前の補足の行を、すべての記録で切れた `superseded_by` のリンクを、誤りとして出す。

## Specification

正本は仕様 IR `docs/ir/*.md`（以下「IR」）。この計画が対象にする項目は次のとおり。

- `docs/ir/record-form.md#REQ-129`（形の検査の対象）、`#REQ-130`（必須の補足の行）、`#REQ-131`（知らない名前の補足の行）、`#REQ-133`（補足の行の名前と値）、`#REQ-134`（補足の行の数を見ない）、`#REQ-135`（判断の記録で読まない行）、`#REQ-136`（記録の読み取りは1つの関数）、`#TBL-022`（節ごとの必須の補足の行）、具体例 EX-101〜EX-105、EX-110、EX-111、EX-113〜EX-115、EX-118
- `docs/ir/revision-link.md#REQ-132`（superseded_by のリンク）、`#TBL-023`（superseded_by のリンクの判定）、具体例 EX-106〜EX-109、EX-112、EX-116、EX-117、EX-119
- 指摘の形: `docs/ir/findings.md#TBL-008`（種類と detail）、`docs/ir/finding-order.md#TBL-019`（指摘の行）。並べ方（`#TBL-007`）は既存の振る舞いで `run_check` の末尾がそのまま効くので、この計画の対象にしない
- 既存の要求で境界を決めているもの: `docs/ir/sources.md#TBL-012`（出典の判定。順2の「番号の行」と「コードブロックの中は除外」は今回の文言）、`#REQ-061`（決定の番号はファイルごと）、`docs/ir/base-directory.md#REQ-110`（パスの正規化）、`docs/ir/skill-references.md#REQ-125`（references の種類の表と本体の一致）、`docs/ir/CONTEXT.md` の用語 `判断の記録`、`決定の節`、`決定の番号`、`番号の行`、`補足の行`、`Superseded の節`、`除外`、`コードブロック`
- review の要求の確かめ方: `docs/trace.md` の REQ-136 の行

判断の記録は `docs/decision/records/2026-09-17-record-form.md`（A1〜A43、R1〜R3）。IR の形の契約は `docs/decision/records/ir-form.md`（以下「契約」。「判断の記録の形」の節が今回の要約）。

IR の読み方は前の計画と同じ。要求の `- 種類:` が `prohibition` の文は起きてはならないこと、`algorithm` の中身は `- 定義:` の指す決定表にある、`- 検証:` が `review` の要求（REQ-136）はテストでなく読む場所を `docs/trace.md` に書いてある。IR の各文書の `## 具体例` のシナリオは、その要求の成功の条件と反例で、テストの入力と期待の元にする。

IR と契約と判断の記録は実装の間は読み取り専用である。IR の書き換えが必要だと思ったら、書き換えずに止まって返す。IR の沈黙は実装者が決めてよい意味ではない（`docs/ir/cli-environment.md#REQ-120`）。

この計画の中のテストの置き方と名前の付け方は、IR ではなくこの計画の約束である。

## Approach and why

既存の実装（`src/` 約3,900行、テスト約400件）を育てる。作り直さない。新しい依存は足さない（判断の記録 A10）。

いま判断の記録を読むのは `src/sources.rs` の `parse_records_file` で、決定の節（Agreements、Prohibitions、Delegated、Rejected）の中の `- 番号 ` の行の番号と `## ` の見出しの一覧だけを集め、`check_source` が出典の判定に使う。コードブロックは見ておらず、Superseded の節、補足の行、リンクは読んでいない。

変更は3つの性格に分かれる。

1. **読み取りを1つの構造に育てる**（REQ-136、REQ-135、A33〜A35）: `parse_records_file` を、記録の1ファイルを「節の並び。各節は名前と、番号の行の並び。各番号の行は番号・行番号・その下の補足の行の並び（名前、値、行番号、値の中のリンクの並び）」の構造に組み立てる1つの関数にする。コードブロックの中は読まず（見出しも数えない）、番号の行の判定を補足の行より先に行い、名前の形（1文字以上、空白と ":" を含まない）に当たらない行と、最初の番号の行より前の補足の行の形の行は捨てる。出典の判定（`check_source`）はこの構造の決定の節の番号だけを見るように付け替え、振る舞いは変えない（コードブロックの中の番号が出典の先にならなくなる分だけ変わる。EX-118）。Superseded の節の番号は、出典の先にはせず（TBL-012 は決定の節だけ）、リンクの先の判定（TBL-023 順5）でだけ使う
2. **補足の行の検査**（REQ-129〜131、133、134、TBL-022）: 上の構造を受け、`## Context` の見出し（`## ` の後を前後の空白を除いて完全一致）を持つ記録だけに、節ごとの必須の名前の有無と、6つ以外の名前を見る。指摘の種類 `record_field_missing`（line は番号の行、detail は足りない名前）と `record_field_unknown`（line はその行、detail は名前）を `src/lib.rs` の `finding_kinds!` に足す。`FindingKind::ALL` は macro が作るので、足した時点で `tests/step7_skill_references.rs` の REQ-125 のテストが `skills/kotowari/references/findings.md` の表との不一致で落ちる。同じステップで表に行を足して戻す
3. **superseded_by のリンクの検査**（REQ-132、TBL-023）: 名前が `superseded_by` で値が空でない補足の行の値からリンクを走査し、6段の判定を順に当てる。href の "#" より前を記録のファイルのディレクトリに字面でつなぎ、`crate::normalize_path`（REQ-110）をかけた後に ".." を左から解く。`normalize_path` は ".." を解かないので、解く処理はこの検査が持つ。先のファイルは `SourceContext` が読んだ記録の一覧から探す（読み直さない）。指摘の種類 `revision_link_invalid`（line はその補足の行、detail は href。順1では行の値）を足し、references の表にも行を足す

読む順は 1 → 2 → 3。2 と 3 はどちらも 1 の構造に依存する。3 のテストの一部（決定の節を持たないファイルの superseded_by、値が空の superseded_by）は 2 の指摘が出ないことも同時に見るので、3 は 2 の後に置く。

`Finding` の `path` は、既存の出典の指摘と同じく `crate::join_display_path(&cfg.decisions.records, &rel_path)`（基準からの相対）にする。並べ方は `run_check` の末尾の並べ替えがそのまま効く（TBL-007）。`--format text` は種類を問わず同じ形で出すので `src/main.rs` は変えない。

テストの置き方と名前（この計画の約束。前の計画と同じ）:

- テストは `tests/` に置き、ライブラリの公開する関数と型と CLI だけを通す。今回のテストは新しい `tests/step8_record_form.rs` に置く（補足の行とリンクの両方）。fixture は `tests/step3_sources_terms.rs` の `make_project_with_records` と同じ作り（一時ディレクトリに `.kotowari/config.yaml`、`docs/ir`、`docs/decision/records`、`docs/decision/adr` を置いて CLI を `check` で起動し JSON を読む）。`make_project_with_records`（ディレクトリと設定と記録を書く）、`cmd`（CLI の起動）、`parse_json`、`findings_by_kind` の4つは private なので写して使ってよい。EX-118 だけは出典の判定のテストなので `tests/step3_sources_terms.rs` に置く
- 各テスト関数の直前の行に `// @kotowari[REQ-130, TBL-022]` の印を置く（`skills/kotowari/references/mark.md` の形）
- テスト関数の名前は、確かめる IR の ID を先頭に付けた snake_case（例: `req_130_context_record_without_why_is_record_field_missing`）
- テストは RED → GREEN → REFACTOR の順で書き、各段で `CARGO_BUILD_JOBS=4 cargo test` を走らせる。REQ-134 と REQ-135 と EX-102 のような「指摘が出ない」テストは、書いた時点で GREEN になりうるので、対応する検査を入れた後に「出るはずの指摘」を期待に一時的に置いて RED を見てから戻す。その RED の出力（落ちたテスト名と assert の文）を終端報告に引用する。反転した状態はコミットしない
- 既存のテストが新しい仕様と食い違うときは、新しい仕様に合わせて書き直し、コミットの本文に「A34 で改めた」のように改めた決定を書く。`tests/step3_sources_terms.rs` の出典のテストは fixture にコードブロックを持たないので、落ちる既存テストは無い見込み
- コミットは意図ごとに分け、`git add <path>` でファイルを1つずつ。件名は `<type>: <件名>` の形で日本語。帰属の行は付けない

## Scope of change

変えてよいのは、リポジトリ直下の次のものだけである。

- `src/` の下のすべて
- `tests/` の下のすべて
- `skills/kotowari/references/findings.md`（種類の表に3行。ステップ2で2行、ステップ3で1行）
- `Cargo.lock`（依存の更新はしない。ビルドで変わった分だけ）

`Cargo.toml` は変えない（依存を足さない）。`docs/ir/`、`docs/decision/`、`docs/plans/`、`docs/trace.md`、`docs/spec/`、`skills/kotowari/SKILL.md`、ほかの references、`lefthook.yml`、リポジトリ直下の `.kotowari/`、`experiments/`、`fixtures/` は変えない。インストール済みスキル（`~/.claude/skills/kotowari/`）はリポジトリの外で、マージ後に主セッションが写す。

## Step order and prerequisites

ステップ1 → 2 → 3 → 4 の順に行う。

## Step 1 — 判断の記録を1つの構造に読む

Purpose: 記録の読み取りを、節・番号の行・補足の行・リンクを持つ構造を返す1つの関数にし、出典の判定をその構造に載せ替える。Specification: `docs/ir/record-form.md#REQ-136`、`#REQ-135`、`#REQ-133`（名前と値の読み方）、`docs/ir/revision-link.md#TBL-023`（リンクの走査）、`docs/ir/sources.md#TBL-012`、`#REQ-061`、`docs/ir/CONTEXT.md`（`番号の行`、`補足の行`、`Superseded の節`、`除外`、`コードブロック`）、契約の「判断の記録の形」の節、具体例 EX-118。
Prerequisites: なし。始める前に `CARGO_BUILD_JOBS=4 cargo test` を1回走らせて全件通ることと、`CARGO_BUILD_JOBS=4 cargo run -q -- check` の出力を記録する（ステップ4でこの記録と比べる）。この時点の終了コードは1で、"findings" は requirement_without_test が7件（REQ-129〜135）、"counts" は `{"requirement_without_test": 7}` のはずである。違えば止まって返す。
May change: `src/sources.rs`、`src/lib.rs`（構造の型を公開する場合）、`tests/step3_sources_terms.rs`。
Done when: `src/sources.rs` に、記録の1ファイルを読んで構造を返す公開の関数が1つある。構造は、節（`## ` の見出しごと）と、その中の番号の行（番号と行番号）と、各番号の行の下の補足の行（名前、値、行番号、値の中のリンクの並び。リンクは文字と href）を持つ。番号の行・補足の行・名前と値・リンクの読み方と、読まない行は、用語集の `番号の行`・`補足の行`、REQ-133、REQ-135、TBL-023 の走査の文のとおりで、この計画は写さない。行番号は1始まり（TBL-019 の line に使う）。コードブロックの除外は判断の記録（決定の節の見出しを持つファイル）に効き、決定の節の見出しがコードブロックの中にしか無いファイルは判断の記録でない（A46。用語集の `判断の記録` の「コードブロックの外に」）。決定の節を持たない Markdown（契約、補足の文書）の `## ` の見出しの集め方は今の振る舞いを変えない（TBL-012 の順4はコードブロックに触れておらず、仕様が黙っている範囲を変えないため。この判断は計画の承認の判断点）。出典の判定（`check_source`）は決定の節（4つ）の番号の行だけを見て、既存の出典のテストがすべて変更なしで通る。コードブロックの中にだけある番号を指す出典は source_invalid になる。構造の読み方そのもの（補足の行、リンク、除外）はステップ2と3のテストが観測する。
Shown by: test — `tbl_012_number_inside_code_block_is_not_a_source_target`（EX-118。`tests/step3_sources_terms.rs`）。既存の `req_058_*`、`req_061_numbers_are_per_file`、`req_106_form_contract_headings_are_valid_sources` が変更なしで通ることで、出典の判定が変わっていないことを示す。REQ-136（review）は終端報告に `docs/trace.md` の REQ-136 の行の2点を書く: 記録を読む関数が1つで形の検査と出典の判定がその返す構造だけを読むこと、`rg "lines\(\)" src/sources.rs` の当たりの一覧と、各当たりが読み取り関数の中か、ADR 用の `parse_other_file` のように記録の読み取りでないかの説明。
Left to the implementer: 構造の型と欄の名前、`RecordsFile` を残すか置き換えるか、コードブロックの追跡を `src/ir.rs` の `parse_opening_fence` / `is_closing_fence` と共有するか写すか（判定は契約と同じ形であることを確認済み）。モジュールは `src/sources.rs` に固定する（`docs/trace.md` の REQ-136 の確かめ方がこのパスを名指しするため）。
Stop and hand back if: 既存の出典のテストが落ち、その原因がコードブロック以外にある（IR の文言と今の振る舞いの食い違い）。

## Step 2 — 補足の行の検査

Purpose: `## Context` を持つ記録で、節ごとの必須の補足の行の欠けと、6つ以外の名前を誤りにする。Specification: `docs/ir/record-form.md#REQ-129`、`#REQ-130`、`#REQ-131`、`#REQ-133`、`#REQ-134`、`#REQ-135`、`#TBL-022`、`docs/ir/findings.md#TBL-008`、`docs/ir/finding-order.md#TBL-019`、`docs/ir/skill-references.md#REQ-125`、具体例 EX-101〜EX-105、EX-110、EX-111、EX-113〜EX-115。
Prerequisites: ステップ1。
May change: `src/lib.rs`（`finding_kinds!` に `RecordFieldMissing`、`RecordFieldUnknown`。`run_check` から検査を呼ぶ）、`src/sources.rs`（または構造を置いたモジュール）、`tests/step8_record_form.rs`（新規）、`skills/kotowari/references/findings.md`（種類の表に2行。意味と対処と担当は brainstorm）。
Done when: `## Context`（`## ` の後を前後の空白を除いて "Context" と完全一致）を持つ記録だけを対象に、TBL-022 の表の節（Agreements、Prohibitions、Delegated、Rejected は why、Undecided は decides、Superseded は superseded_by）の番号の行に必須の名前の補足の行が無いか値が空なら `record_field_missing`（severity error、line は番号の行、detail は名前）、6つ以外の名前なら `record_field_unknown`（line はその行、detail は名前。値が空でも出る。同じ知らない名前が2行あれば2件）。"not recorded" は有るものと数える。必須の名前が同じ番号の行の下に複数あっても `record_field_missing` を出さず、数を見ない（REQ-134）。表に無い節（Revisions、Context）の行は読まない。`## Context` を持たない記録には出さない。決定の節の見出しを持たないファイルは判断の記録でなく対象外。`path` は基準からの相対（`docs/decision/records/x.md` の形）。REQ-125 のテストが通る（references の表に2行）。
Shown by: test — `req_130_context_record_without_why_is_record_field_missing`（EX-101）、`req_129_record_without_context_is_not_checked`（EX-102）、`req_133_not_recorded_passes_and_blank_value_is_missing`（EX-103）、`req_131_unknown_field_name_is_record_field_unknown`（EX-104）、`req_130_superseded_line_without_superseded_by_is_missing`（EX-105）、`req_135_lines_outside_the_table_sections_and_orphans_are_not_read`（EX-110）、`req_134_duplicate_field_names_pass`（EX-111）、`req_131_unknown_name_on_two_lines_yields_two_findings`（同じ知らない名前が2行。TBL-019 の「その行」を2件で見る）、`req_135_numbered_line_inside_code_block_is_not_read`（EX-113）、`req_135_unclosed_code_block_runs_to_the_end_of_the_file`（番号の行の後に閉じない ``` があり、その後の "- A9 x" と "- why:" の無い "- A8 x" が読まれず、指摘が出ない。A45）、`req_130_unindented_field_line_belongs_to_the_decision`（EX-114。fixture では番号の行と why の間に空行を1つ置き、用語集の「空行を挟んでよい」も同時に見る）、`req_130_decision_line_with_colon_is_not_a_field`（EX-115）、`req_129_file_without_decision_sections_is_not_a_record`（`## Context` と `## Superseded` だけを持つファイルに record_field_missing と record_field_unknown が出ない。A41。revision_link_invalid が出ないことはステップ3のテストが見る）。既存の `req_125_findings_reference_kinds_match_the_code` が通ることで references の表の追従を示す。
Left to the implementer: 検査の関数の置き場と名前。`## Context` の有無を構造に持たせるか検査の側で見るか。fixture の記録の文面（IR のシナリオの Given を満たす範囲で自由）。
Stop and hand back if: TBL-022 の表に無い節に番号の行があるときの扱いが実装上どうしても要る場面（IR は「読まない」と決めている。それ以外の扱いが要ると思ったら止まる）。`FindingKind::ALL` が macro で作られておらず手で列挙している（その場合は列挙にも足すが、REQ-125 のテストがそれを見ていることを確かめる）。

## Step 3 — superseded_by のリンクの検査

Purpose: すべての判断の記録で、superseded_by の行のリンクを解決し、切れていれば誤りにする。Specification: `docs/ir/revision-link.md#REQ-132`、`#TBL-023`、`docs/ir/record-form.md#REQ-129`（適用の範囲）、`#REQ-133`（値が空の superseded_by は対象外）、`docs/ir/base-directory.md#REQ-110`、`docs/ir/findings.md#TBL-008`、`docs/ir/finding-order.md#TBL-019`、`docs/ir/skill-references.md#REQ-125`、`docs/ir/CONTEXT.md`（`Superseded の節`、`判断の記録`）、具体例 EX-106〜EX-109、EX-112、EX-116、EX-117、EX-119。
Prerequisites: ステップ1とステップ2。
May change: `src/lib.rs`（`finding_kinds!` に `RevisionLinkInvalid`。`run_check` から検査を呼ぶ）、`src/sources.rs`（または構造を置いたモジュール）、`tests/step8_record_form.rs`、`skills/kotowari/references/findings.md`（種類の表に1行）。
Done when: 名前が superseded_by で値が空でない補足の行について、リンクごとに TBL-023 の順1〜6 を上から当て、無効なら `revision_link_invalid`（severity error、line はその行、detail は href。順1では行の値）を出現ごとに1件出す。順3は、まず生の href の最初の "#" より前が "/" か "\\" で始まるかを見て、始まれば置き場の外とする（つなぐ前に見る。つないでから `normalize_path` にかけると先頭の "/" が消えて置き場の中に見えるため）。始まらなければ、記録のファイルの基準からの相対パス（`join_display_path(&cfg.decisions.records, &rel_path)`）の親ディレクトリに字面でつなぎ、`normalize_path` をかけた後に ".." を左から順に直前の要素を消して解き、消す要素が無ければ置き場の外とする。"/" か "\" で始まる href は置き場の外。"#" より前が空なら同じ記録。先は `SourceContext` が読んだ判断の記録（決定の節の見出しを持つファイル）の中から探し、無ければ順4。先の決定の節と Superseded の節のどこにも番号の行が無ければ順5。`## Context` の有無を見ない。決定の節の見出しを持たないファイルの superseded_by は見ない。`path` は基準からの相対。REQ-125 のテストが通る（references の表に1行）。このリポジトリ自身の記録（superseded_by の行が15本）に対して `cargo run -q -- check` を走らせて `revision_link_invalid` が0件である。
Shown by: test — `req_132_record_without_context_resolves_two_links_in_superseded`（EX-106。`## Context` を持たない記録の Superseded の行にリンク2つ。records.md の実物ではなく fixture で同じ配置を作る）、`req_132_superseded_by_without_link_is_invalid`（EX-107）、`req_132_href_outside_the_records_place_is_invalid`（EX-108）、`req_132_number_only_in_undecided_is_invalid`（EX-109）、`req_132_heading_anchor_is_invalid`（EX-112）、`req_132_parent_directory_href_to_superseded_number_passes`（EX-116）、`req_132_empty_path_means_the_same_record`（EX-117）、`req_132_target_that_is_not_a_record_is_invalid`（EX-119）、`req_133_empty_superseded_by_is_not_a_link_finding`（Superseded の節の番号の行の下に値が空の superseded_by。revision_link_invalid は出ず、`## Context` を持つ記録では record_field_missing（detail は superseded_by）だけが出て、持たない記録では何も出ない）、`tbl_023_same_href_twice_on_one_line_yields_two_findings`（同じ無効な href を1行に2つ。TBL-023 の「出現ごとに1件」。対応する EX は無いので印は `// @kotowari[REQ-132, TBL-023]`）、`tbl_023_unclosed_link_is_skipped_and_falls_to_no_link`（値が "[A1](#A1" のような閉じない形と、"[A2] を見よ" のように "]" の直後が "(" でない形の2変種。どちらも順1になり detail は前後の空白を除いた値。A44）、`tbl_023_absolute_href_is_outside_the_place`（"/x.md#A1" と "\\x.md#A1"。順3。つなぐ前に見ることの証拠）、`req_135_superseded_by_in_revisions_is_not_read`（Revisions の節の番号の行の下に切れたリンクの superseded_by。指摘が出ない）、`req_129_broken_link_in_a_file_without_decision_sections_is_not_checked`（`## Context` と `## Superseded` だけのファイルの中の切れた superseded_by に、record_field_missing、record_field_unknown、revision_link_invalid のどれも出ない）。
Left to the implementer: ".." を解く関数の置き場（`src/lib.rs` の `normalize_path` の隣か、検査の側か）、リンクの走査の実装（手書きの走査で足りる。正規表現の crate は足さない）。
Stop and hand back if: `normalize_path` が ".." を解いていた、または先頭の "/" を残していた（この計画は「解かない」「先頭の空は保持しない」を `src/lib.rs` の今の実装から読んでいる。違えば順3の手順が変わる）。`SourceContext` が記録の一覧を基準からの相対で持っておらず、href の解決の結果と比べられない（比べ方は自由だが、`join_display_path` で同じ形にできないなら止まる）。

## Step 4 — 終端の確認

Purpose: 全体が仕様どおりで、このリポジトリ自身の記録に対して誤りが無いことを確かめる。Specification: 上の全部。
Prerequisites: ステップ1〜3。
May change: なし（直しが要れば該当のステップに戻る）。
Done when: `CARGO_BUILD_JOBS=4 cargo test` が全件通り、`CARGO_BUILD_JOBS=4 cargo run -q -- check` の終了コードが0で "findings" が空（ステップ1で記録した7件の requirement_without_test が消え、新しい種類の指摘がこのリポジトリの記録に1件も出ない）。`skills/kotowari/references/findings.md` の種類の表に3行が足されている。
Shown by: check — `CARGO_BUILD_JOBS=4 cargo test`、`CARGO_BUILD_JOBS=4 cargo run -q -- check`（終了コード0）、`CARGO_BUILD_JOBS=4 cargo run -q -- check --format text`（何も出ない）。
Left to the implementer: なし。
Stop and hand back if: 種類を問わず、ステップ1で記録した基準線に無い指摘がこのリポジトリに残る（記録が仕様に合っていないか、実装が仕様と違う。どちらかは主セッションが判断する。記録と IR は実装の間は読み取り専用なので直さない）。

## Verification map

| 仕様の項目 | 証拠を出すステップ |
|---|---|
| record-form.md REQ-136（review） | ステップ1（`docs/trace.md` の REQ-136 の行の2点、読む関数が1つで検査がその構造だけを読むこと、`rg "lines\(\)"` の当たりの説明、を終端報告に書く） |
| record-form.md REQ-135、REQ-133 の名前と値、sources.md TBL-012 の順2 | ステップ1（EX-118）、ステップ2（EX-110、EX-113〜EX-115） |
| record-form.md REQ-129 | ステップ2（補足の行の検査の範囲）、ステップ3（REQ-132 の範囲と、決定の節を持たないファイル） |
| record-form.md REQ-130、REQ-131、REQ-133、REQ-134、TBL-022 | ステップ2（REQ-133 の値が空の superseded_by はステップ3） |
| revision-link.md REQ-132、TBL-023 | ステップ3 |
| findings.md TBL-008、finding-order.md TBL-019 の新しい行 | ステップ2、3（各テストの line と detail の assert） |
| skill-references.md REQ-125 | ステップ2、3（既存の `req_125_*` が通る） |
| 全体 | ステップ4 |

## Left to the implementer

各ステップの欄のとおり。共通: Rust の型・関数・モジュールの名前、テストの fixture の文面、`src/sources.rs` の中の分割。

## Stop conditions

一般の4条件（意味の欠落や承認内容からの逸脱、不可逆・特権・危険な操作、広がる事故、方式を変えても進捗なし）に加えて、各ステップの「Stop and hand back if」。IR・契約・判断の記録を変えたくなったら、変えずに止まって返す。

## Test command

`CARGO_BUILD_JOBS=4 cargo test`。check の確認は `CARGO_BUILD_JOBS=4 cargo run -q -- check`（JSON）と `--format text`。

## Out of scope

- インストール済みスキル `~/.claude/skills/kotowari/references/findings.md` への同期（マージ後に主セッションが行う）
- `docs/trace.md`、IR、契約、判断の記録の変更
- 判断の記録の Revisions の節や、IR の文書名の参照など、今回の検査の対象外の行
- mds への載せ替え（判断の記録の U1、check-reach の U1）
- `lefthook.yml` の変更
