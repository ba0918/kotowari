# Plan: IR に無い、利用者から見える面の検査を作る

## Goal

設定の `surface.files` と `surface.rules` を書いたプロジェクトで、`kotowari check` と `kotowari status` がコードから利用者に見える面を取り出し、IR に引用されて出てこない面を surface_without_spec の誤りにし、未記載の面の一覧で外した件数を出す。

## Specification

IR は `docs/ir/`。決定は `docs/decision/records/2026-09-27-surface-check.md`（A1〜A25、A21 は A24 で改めた）。この計画が扱うのは次のとおり。

- 新しい要求: `docs/ir/core/surface.md#REQ-core-223`、`#REQ-core-224`、`#REQ-core-225`、`#REQ-core-226`、`#REQ-core-227`、`#REQ-core-228`、`#REQ-core-229`、`#REQ-core-230`（review）、`#REQ-core-236`、`docs/ir/core/surface-unspecified.md#REQ-core-231`、`#REQ-core-232`、`#REQ-core-233`、`#REQ-core-234`、`docs/ir/core/adoption.md#REQ-core-235`（review）
- 新しいシナリオ: EX-core-407〜418、EX-core-426〜429（surface.md）、EX-core-419〜425（surface-unspecified.md）
- 広げた既存の項目: `docs/ir/core/config.md#REQ-core-014`、`#REQ-core-018`、`#TBL-core-004`、`docs/ir/core/cli.md#TBL-core-001`、`docs/ir/core/cli-environment.md#REQ-core-109`、`#TBL-core-020`、`docs/ir/core/findings.md#REQ-core-031`、`#TBL-core-008`、`#TBL-core-009`、`docs/ir/core/finding-order.md#REQ-core-027`、`#TBL-core-019`、`docs/ir/core/output.md#TBL-core-005`、`#TBL-core-006`、`docs/ir/core/status.md#REQ-core-162`、`#TBL-core-028`、`docs/ir/core/list.md#REQ-core-152`、`docs/ir/core/query.md#REQ-core-158`
- 用語: `docs/ir/core/CONTEXT.md` の「面」「面の規則」「面のファイル」「未記載の面の一覧」と、限定した「除外」

要求とシナリオの本文は `kotowari query <ID>` で読む（`CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- query <ID>`）。広げた既存の項目の差分は `git show 029f48b -- docs/ir` で見られる。

## Approach and why

- 面の規則の読み込みとファイルの走査は、`tests.rules` と `tests.files` の実装（`crates/kotowari-core/src/test_queries.rs`、`tests_discovery.rs`、`config.rs`）を使い回す（A1、A20）。ただし面の規則はテストの問い合わせに加えない（REQ-core-224）。YAML の読み方と language の突き合わせ（大文字小文字を区別しない、別名を受ける）は共有し、規則の集まりはテストと別に持つ。
- 未記載の面の一覧の読み込みと1件の検査は、等価の一覧の実装（`crates/kotowari-core/src/equivalents.rs`）と同じ形にする（A22）。
- IR にあるかの判定は、IR を読んだ結果から要求の文、決定表のセル（見出しの行を含む）、シナリオのステップの行だけを取り、二重引用符の対とバッククォートの対（二重引用符の外のものだけ、REQ-core-064 と同じ取り方）の中身を集めた集合と、面の名前を完全一致で比べる（REQ-core-226、A18）。用語の検査の対の取り方の実装を共有する。
- 面は `check` と `status` だけが読む（A14、REQ-core-229）。設定の鍵の組み合わせの誤りは設定を読む段で止めるので、どのコマンドでも止まる（A24、REQ-core-225）。
- 新しい指摘の種類を足すと、既存のテストが落ちる箇所がある。`agent/skills/kotowari/references/findings.md` の Kind の表と本体の種類の一致（REQ-core-125、`tests/step7_skill_references.rs`）、スキルの config.md の準備の YAML と既定の一致（REQ-core-126）、注意の種類の数を固定したテスト（REQ-core-031）。どれもこの計画の変更による失敗なので、表とテストを直す。
- kotowari 自身の設定（`.kotowari/config.yaml`）に面の規則を書く（A9）。規則はサブコマンド（`crates/kotowari-core/src/lib.rs` の `COMMANDS` の中の文字列）と、`"--` で始まる match の腕のフラグの2つで、`.kotowari/rules/` のような置き場に置く。IR に出てこない面が見つかったら、IR の直しは仕様の変更なので作らず、未記載の面の一覧に理由を付けて載せ、載せた面をすべて報告に挙げる。止まるのは、利用者に見える機能なのに仕様に無いと判断し、一覧に載せる理由が書けない面が出たときだけ。
- 仕様を直したので `docs/guides/` に guide_stale の注意が出ている（コミット 029f48b の時点で36件）。実装が決まった後で、`agent/skills/kotowari/references/guides.md` の見直しの手順で節を読み直して指紋を書き写し、ガイドに面の検査の書き方を足す。ガイドへの追記は仕様の要求ではなくこの計画の判断で、新しい設定が利用者向けの文書に無いと使えないため。

## Scope of change

- `crates/kotowari-core/src/` のうち config.rs、lib.rs、test_queries.rs、tests_discovery.rs、equivalents.rs、ir.rs、terms.rs、finding_map.rs、status.rs、list.rs、query.rs と、面の検査を置く新しいモジュール
- `.kotowari/schemas/` の指摘の写し先（finding_map が種類を読むなら）
- `tests/` の既存のテストのファイル（step1_config.rs、step5_findings.rs、step6_output.rs、step7_skill_references.rs、step12_status.rs など該当するもの）と、面のテストを置く新しいファイル
- `agent/skills/kotowari/references/`（surface.md は新設、config.md、findings.md）と `agent/skills/kotowari-adopt/SKILL.md`
- `.kotowari/config.yaml` と、kotowari 自身の面の規則のファイルと未記載の面の一覧（要るなら）
- `docs/guides/` の guide_stale が出ている節と、面の検査の説明を足す節

## Step order and prerequisites

S1（設定）が先で、S2 以降はその鍵を使う。EX-core-407、EX-core-419、EX-core-423 は Then で check の出力の "surface" の "unspecified" を確かめ、その鍵は S5 で足すので、S5 の行に置く。新しい指摘の種類を足す段（S3、S4）では、REQ-core-125 のテストが落ちないよう、スキルの findings.md の Kind の表の行も同じ段で足す。S2（面の取り出し）の結果を S3 と S4 が使う。S5（出力と status）は S3 と S4 の数を使う。S6（スキル）は S3〜S5 の種類と鍵の名前が決まった後。S7（kotowari 自身の設定）は S2〜S5 が動いてから。S8（ガイド）は振る舞いが決まった後。S9 で全体を確かめる。

## Verification map

| Step | Requirements | Examples |
|---|---|---|
| S1 | REQ-core-014、REQ-core-225（鍵の組み合わせ）、REQ-core-152、REQ-core-158、TBL-core-004 | EX-core-415、EX-core-416、EX-core-428 |
| S2 | REQ-core-223、REQ-core-224、REQ-core-236、REQ-core-225（規則のファイルの誤り）、REQ-core-018、REQ-core-109（review）、TBL-core-001、TBL-core-020（規則のファイルと走査の場面） | EX-core-417、EX-core-426、EX-core-427 |
| S3 | REQ-core-226、REQ-core-227、TBL-core-019、TBL-core-006、TBL-core-008（surface_without_spec の行） | EX-core-408、EX-core-409、EX-core-410、EX-core-411、EX-core-412、EX-core-418、EX-core-429 |
| S4 | REQ-core-231、REQ-core-232、REQ-core-233、REQ-core-234、REQ-core-027、REQ-core-031、TBL-core-006、TBL-core-008、TBL-core-009、TBL-core-001、TBL-core-020（一覧の場面） | EX-core-420、EX-core-421、EX-core-422、EX-core-424、EX-core-425 |
| S5 | REQ-core-228、REQ-core-229、REQ-core-162、TBL-core-005、TBL-core-028 | EX-core-407、EX-core-413、EX-core-414、EX-core-419、EX-core-423 |
| S6 | REQ-core-230、REQ-core-235 | なし |
| S7 | なし（A9） | なし |
| S8 | なし（guide_stale を0件にする。REQ-core-204 が出す注意） | なし |
| S9 | この表のすべて | この表のすべて |

## Left to the implementer

- 関数、型、モジュール、テストの名前と、面の検査を置くモジュールの分け方
- テストを既存のファイルに足すか、新しいファイルに置くか
- kotowari 自身の面の規則のファイルと未記載の面の一覧の置き場の名前

## Stop conditions

- 要求とシナリオが食い違って見える、または要求が決めていない振る舞いが要る（たとえば面の規則とテストの問い合わせを分けると、既存のテストの問い合わせの振る舞いが変わる）
- tree-sitter か ast-grep の都合で、`$NAME` の取り出しや language の突き合わせが `tests.rules` と同じにできない
- kotowari 自身の IR に出てこない面のうち、一覧に載せる理由が書けない（仕様にあるべきなのに無いと判断した）面が出た

## Test command

```sh
CARGO_BUILD_JOBS=4 cargo test --workspace
CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format text
```

## Out of scope

- A8 の transform で面の名前を整えられるかの確認（仕様にしていない）
- kotowari-adopt の範囲と行の粒度の直し（別に決める）
- `docs/ir/` の変更（仕様の変更は壁打ちに戻す）

## Steps

### S1: 面の設定の鍵と停止

- Purpose: `surface.files`、`surface.rules`、`surface.unspecified` の鍵を読み、鍵の組み合わせの誤りと glob の構文の誤りを仕様どおりにする
- Specification: `docs/ir/core/surface.md#REQ-core-225`, `docs/ir/core/config.md#REQ-core-014`, `docs/ir/core/config.md#TBL-core-004`, `docs/ir/core/list.md#REQ-core-152`, `docs/ir/core/query.md#REQ-core-158`
- Prerequisites: none
- May change: `crates/kotowari-core/src/config.rs`, `crates/kotowari-core/src/lib.rs`, `crates/kotowari-core/src/list.rs`, `crates/kotowari-core/src/query.rs`, `tests/`
- Done when: 3つの鍵が既定どおりに読め、規則だけ・ファイルだけ・一覧だけの組み合わせで設定を読むどのコマンドも設定の誤りで停止し、`surface.files` の glob として読めない要素が設定の誤りになる
- Shown by: test — EX-core-415、EX-core-416、EX-core-428 と、REQ-core-014 の面の glob の場合のテスト
- Left to the implementer: 設定の型への足し方
- Stop and hand back if: 鍵の組み合わせの誤りを設定を読む段で止めると、設定を読まないコマンドの既存の振る舞いが変わる

### S2: 面の取り出し

- Purpose: 面の規則を読み、面のファイルから面（種類、名前、ファイル、行）を取り出す
- Specification: `docs/ir/core/surface.md#REQ-core-223`, `docs/ir/core/surface.md#REQ-core-224`, `docs/ir/core/surface.md#REQ-core-236`, `docs/ir/core/surface.md#REQ-core-225`, `docs/ir/core/config.md#REQ-core-018`, `docs/ir/core/cli-environment.md#REQ-core-109`, `docs/ir/core/cli.md#TBL-core-001`, `docs/ir/core/cli-environment.md#TBL-core-020`
- Prerequisites: S1
- May change: `crates/kotowari-core/src/test_queries.rs`, `crates/kotowari-core/src/tests_discovery.rs`, 面の検査の新しいモジュール, `crates/kotowari-core/src/lib.rs`, `tests/`
- Done when: 規則に当たった節が面になり、`$NAME` の無い当たりは面にならず、引用符1組を外し、language は大文字小文字を区別せず別名も受け、規則の "files" と "ignores" を面のファイルに当て、"severity" が "off" の規則も当て、"fix" や "message" を使わず、規則の言語のファイルだけを木にして構文の誤りに unparsable_file を出し（同じパスにテストのファイルとして出していれば重ねない）、面の規則がテストの問い合わせに加わらない。規則のファイルが REQ-core-189 の条件に当たると check と status だけが設定の誤りで停止し、その詳細は規則のファイルのパスになる。`surface.files` の走査の停止は `tests.files` と同じになる
- Shown by: test — EX-core-417、EX-core-426、EX-core-427 と、面の規則がテストとして数えられないこと、files と ignores と severity off、重ねて出さない unparsable_file、REQ-core-018 の面の走査、TBL-core-020 の規則のファイルの詳細のテスト。REQ-core-109（review）は how_to_verify の事柄を報告で確かめる
- Left to the implementer: 規則の集まりの持ち方
- Stop and hand back if: `$NAME` の取り出しか language の突き合わせを `tests.rules` と同じにできない

### S3: IR にあるかの判定と surface_without_spec

- Purpose: IR の数える場所から引用された中身を集め、面の名前と比べ、無い面を誤りにする
- Specification: `docs/ir/core/surface.md#REQ-core-226`, `docs/ir/core/surface.md#REQ-core-227`, `docs/ir/core/finding-order.md#TBL-core-019`, `docs/ir/core/output.md#TBL-core-006`, `docs/ir/core/findings.md#TBL-core-008`
- Prerequisites: S2
- May change: `crates/kotowari-core/src/lib.rs`, `crates/kotowari-core/src/ir.rs`, `crates/kotowari-core/src/terms.rs`, 面の検査の新しいモジュール, `agent/skills/kotowari/references/findings.md`, `tests/`
- Done when: 要求の文、決定表のセル（見出しの行を含む）、シナリオのステップの引用の中身と完全一致する名前だけを IR にあるとし、性質の文、範囲の行、"- " の行、問題の記録の本文、用語集は数えず、無い面ごとに最初の1か所（パスのバイト順、行の順）に surface_without_spec を1件出し、スキルの findings.md の Kind の表にその種類の行がある
- Shown by: test — EX-core-408、EX-core-409、EX-core-410、EX-core-411、EX-core-412、EX-core-418、EX-core-429 と、REQ-core-125 のテスト
- Left to the implementer: 引用の中身を集める処理の置き場
- Stop and hand back if: 用語の検査の対の取り方を共有すると用語の検査の振る舞いが変わる

### S4: 未記載の面の一覧

- Purpose: 一覧を読み、一致した面を外し、形の誤りと要らなくなった1件を指摘にする
- Specification: `docs/ir/core/cli.md#TBL-core-001`, `docs/ir/core/cli-environment.md#TBL-core-020`, `docs/ir/core/surface-unspecified.md#REQ-core-231`, `docs/ir/core/surface-unspecified.md#REQ-core-232`, `docs/ir/core/surface-unspecified.md#REQ-core-233`, `docs/ir/core/surface-unspecified.md#REQ-core-234`, `docs/ir/core/finding-order.md#REQ-core-027`, `docs/ir/core/findings.md#REQ-core-031`, `docs/ir/core/output.md#TBL-core-006`, `docs/ir/core/findings.md#TBL-core-008`, `docs/ir/core/findings.md#TBL-core-009`
- Prerequisites: S3
- May change: `crates/kotowari-core/src/lib.rs`, `crates/kotowari-core/src/equivalents.rs`, 面の検査の新しいモジュール, `agent/skills/kotowari/references/findings.md`, `tests/`
- Done when: 一覧の置き場と停止が等価の一覧と同じ形で、kind と name の完全一致で面を外し、形の誤った1件に surface_unspecified_invalid、要らなくなった1件に surface_unspecified_stale（注意）を "line" を null にして出し、一覧の停止の詳細が一覧のファイルのパスになり、スキルの findings.md の Kind の表に2つの種類の行がある
- Shown by: test — EX-core-420、EX-core-421、EX-core-422、EX-core-424、EX-core-425 と、注意の種類の数のテストの更新、REQ-core-125 のテスト
- Left to the implementer: 等価の一覧との共通部分のくくり方
- Stop and hand back if: 等価の一覧と同じ形にすると、等価の一覧の既存の振る舞いが変わる

### S5: check の出力と status

- Purpose: 外した件数を check に出し、status に面の数を出して面の誤りを complete に入れる
- Specification: `docs/ir/core/surface.md#REQ-core-228`, `docs/ir/core/surface.md#REQ-core-229`, `docs/ir/core/status.md#REQ-core-162`, `docs/ir/core/output.md#TBL-core-005`, `docs/ir/core/status.md#TBL-core-028`
- Prerequisites: S3, S4
- May change: `crates/kotowari-core/src/lib.rs`, `crates/kotowari-core/src/status.rs`, `tests/`
- Done when: 規則が空でないとき check の JSON の最上位に "surface" の "unspecified" が、text の最後の行に "surface: unspecified=数" が出て、規則が空なら出ず、status の "surface" の群が total、specified、unspecified を出し、面の誤りで complete が false になる
- Shown by: test — EX-core-407、EX-core-413、EX-core-414、EX-core-419、EX-core-423 と、TBL-core-028 の "surface" の群と REQ-core-162 の面の場合のテスト
- Left to the implementer: none
- Stop and hand back if: check の text の既存の最後の行の扱いと "surface: " の行が両立しない

### S6: スキルに面の検査を書く

- Purpose: kotowari スキルの references と kotowari-adopt に面の検査の書き方と一覧の減らし方を書く
- Specification: `docs/ir/core/surface.md#REQ-core-230`, `docs/ir/core/adoption.md#REQ-core-235`
- Prerequisites: S5
- May change: `agent/skills/kotowari/references/surface.md`, `agent/skills/kotowari/references/config.md`, `agent/skills/kotowari/references/findings.md`, `agent/skills/kotowari/SKILL.md`, `agent/skills/kotowari-adopt/SKILL.md`, `tests/step7_skill_references.rs`
- Done when: REQ-core-230 と REQ-core-235 の how_to_verify の事柄がスキルに書かれ、config.md の準備の YAML に既定のある新しい鍵が載り、REQ-core-125 と REQ-core-126 のテストが通る
- Shown by: artifact — 変えたスキルのファイル。要求ごとに how_to_verify の事柄と書いた箇所を対にして挙げ、`CARGO_BUILD_JOBS=4 cargo test --test step7_skill_references` の結果を添える
- Left to the implementer: surface.md の節の分け方と、clap の規則の例の書き方
- Stop and hand back if: スキルに書くと仕様に無い振る舞いを約束することになる

### S7: kotowari 自身に面の規則を書く

- Purpose: kotowari 自身の CLI の面（サブコマンドとフラグ）を面の検査にかける
- Specification: `docs/ir/core/surface.md#REQ-core-223`, `docs/ir/core/surface.md#REQ-core-227`
- Prerequisites: S2, S3, S4, S5
- May change: `.kotowari/config.yaml`, kotowari 自身の面の規則のファイル, kotowari 自身の未記載の面の一覧
- Done when: `kotowari check` が kotowari 自身の面を取り出し（`--format text` の "surface: unspecified=" の行が出る）、surface_without_spec が0件で、一覧に載せた面があればそれぞれ理由がある
- Shown by: check — `CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format text` の最後の行と、取り出した面の数（`status` の "surface" の群）
- Left to the implementer: 規則のファイルと一覧の置き場の名前
- Stop and hand back if: IR に出てこない面のうち、一覧に載せる理由が書けない（利用者に見える機能なのに仕様に無いと判断した）面が出た（IR の直しは作らず、その面を報告に挙げて止まる）

### S8: ガイドの印を直し、面の検査を書く

- Purpose: 仕様の変更で古くなったガイドの印を今の IR に合わせ、面の検査の書き方をガイドに足す
- Specification: `docs/ir/core/surface.md#REQ-core-228`
- Prerequisites: S5
- May change: `docs/guides/`
- Done when: `kotowari check` の guide_stale が0件で、ガイドに面の設定と指摘と一覧の説明がある
- Shown by: check — `CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format json | jq '[.findings[]|select(.kind=="guide_stale")]|length'` が 0
- Left to the implementer: 説明を足す節と文面
- Stop and hand back if: ガイドの節を読み直すと、仕様と食い違う振る舞いが書かれている

### S9: 全体を確かめる

- Purpose: この計画の項目がすべてテストを持ち、テストと検査が通ることを確かめる
- Specification: `docs/ir/core/surface.md#REQ-core-227`
- Prerequisites: S1, S2, S3, S4, S5, S6, S7, S8
- May change: none
- Done when: この計画の要求とシナリオ（review の REQ-core-230、REQ-core-235、REQ-core-109 を除く）で `kotowari query` の tests が空でなく、`kotowari check` がこのブランチで変えたファイルとこの計画の ID に誤りを出さず、`cargo test --workspace` が通る
- Shown by: check — `CARGO_BUILD_JOBS=4 cargo test --workspace` の後に `CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format text`
- Left to the implementer: none
- Stop and hand back if: cargo test がこの変更と関係の無い理由で落ちる
