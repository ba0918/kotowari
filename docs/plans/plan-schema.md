# 計画: kotowari plan で計画書の形を検査する

## Goal

利用者（主に kotowari-plan の工程を回す LLM）が `kotowari plan <path>` を実行すると、計画書1つの形が本体に同梱したスキーマで検査され、形の外れが invalid_plan の誤りとして返る。kotowari-plan の工程はそれを Finishing で回し、雛形の例は本体のスキーマとテストで一致を保つ。

## Specification

IR の置き場は `docs/ir/`。この計画が覆う要求と具体例:

- 新しい要求: `docs/ir/core/plan.md#REQ-core-190`、`#REQ-core-191`、`#REQ-core-192`、`#REQ-core-193`、`#REQ-core-194`、`#REQ-core-196`、`#REQ-core-197`、`docs/ir/core/skill-references.md#REQ-core-195`
- 新しい具体例: `docs/ir/core/plan.md` の EX-core-332〜EX-core-361（EX-core-360 を含む30件）
- 改訂した要求と表: `docs/ir/core/cli.md#REQ-core-001`、`#REQ-core-002`、`#REQ-core-004`、`#TBL-core-001`、`docs/ir/core/cli-environment.md#TBL-core-020`、`docs/ir/core/output.md#TBL-core-005`、`#TBL-core-006`、`docs/ir/core/findings.md#TBL-core-008`、`docs/ir/core/config.md#REQ-core-011`、`docs/ir/core/finding-map.md#REQ-core-172`、`docs/ir/core/cli-environment.md#REQ-core-175`、`docs/ir/core/form-contract.md#REQ-core-179`
- 改訂した具体例: `docs/ir/core/cli.md` の EX-core-219、EX-core-241
- 用語: `docs/ir/core/CONTEXT.md` の「計画書」

背景の決定は `docs/decision/records/2026-09-24-plan-schema.md`（A1〜A35。A2 と A6 は A9 で改めた）。要求は `kotowari query <ID>` で読む。

## Approach and why

`kotowari plan` は `mutants` と同じ「ファイルを1つ受けて読むコマンド」として足す（A10、A16）。引数の解析は `crates/kotowari-core/src/lib.rs` の `parse_args` と `Cli` に `Plan` を足し、実行は新しい関数 `run_plan` に置く。`mutants` の実行（`run_mutants`）が、カレントディレクトリからの相対で読んで基準のディレクトリからの相対で表示する（`display_from_base`）形をすでに持っているので、それに倣う。

計画書のスキーマは IR の3つと同じく `.kotowari/schemas/plan.yaml` に置き、`crates/kotowari-core/src/schema.rs` から `include_str!` で取り込む（A14）。検査は mds のライブラリ（`Document::parse` と `validate`）に任せ、kotowari は IR の対応表（TBL-core-030、`finding_map.rs`）を通さずに、スキーマの側の指摘を1件ずつ invalid_plan へ写すだけにする（REQ-core-193）。形の読み取りを自前で書かないのは IR と同じ方針である。

順序は、引数（外から見える入口）→ 読み取り・停止・出力の配線 → 形の規則の全体 → skill 側の一致 → 工程の skill の書き換え、とする。配線を先に作ると、形の規則を足すたびに `kotowari plan` の実行で結果を観測できる。skill の雛形と例は、本体のスキーマが固まってから書く（REQ-core-195 のテストが本体のスキーマを相手にするため）。

## Scope of change

- `crates/kotowari-core/src/lib.rs`（引数の解析、`run`、`run_plan`、使い方の表示）
- `crates/kotowari-core/src/` の下に新しいモジュール（例: `plan.rs`）を足してよい
- `crates/kotowari-core/src/schema.rs`
- `.kotowari/schemas/plan.yaml`（新規）
- `tests/step1_cli.rs`（5つのコマンドを前提にした既存のテストの期待と名前）、`tests/step7_skill_references.rs`、新しいテストのファイル `tests/step14_plan.rs`
- `agent/skills/kotowari/SKILL.md`、`agent/skills/kotowari/references/findings.md`
- `agent/skills/kotowari-plan/SKILL.md`、`agent/skills/kotowari-plan/references/step-template.md`、`agent/skills/kotowari-plan/references/plan-example.md`（新規）
- `agent/skills/README.md`（コマンドの一覧に触れる行があれば）

IR（`docs/ir/`）と判断の記録は変えない。

## Step order and prerequisites

S1 → S2 → S3 → S4 → S5 → S6 の順。S2 は S1 の `Cli::Plan` を使い、S3 は S2 の配線の上で形の規則を足す。S4 は S3 で固まったスキーマを相手にする。S5 は S4 の雛形と例を指す。S6 は全部の後。

## Verification map

| ステップ | 確かめる要求 | 確かめる具体例 |
|---|---|---|
| S1 | REQ-core-001、REQ-core-002、REQ-core-004、REQ-core-190、TBL-core-020 | EX-core-219、EX-core-241、EX-core-342、EX-core-343、EX-core-357 |
| S2 | REQ-core-191、REQ-core-193、REQ-core-194、REQ-core-196、REQ-core-197、TBL-core-001、TBL-core-005、TBL-core-006、TBL-core-008 | EX-core-333、EX-core-344、EX-core-345、EX-core-346、EX-core-358、EX-core-359、EX-core-360、EX-core-361 |
| S3 | REQ-core-192 | EX-core-332、EX-core-334〜EX-core-341、EX-core-347〜EX-core-356 |
| S4 | REQ-core-195 | なし |
| S5 | なし（工程の skill の本文。A4、A8、A12、A23） | なし |
| S6 | この計画が覆う全部 | この計画が覆う全部 |

REQ-core-172、REQ-core-175、REQ-core-011、REQ-core-179 は IR の文書と check の側の文言を絞っただけで、コードの振る舞いは変わらない。既存のテストが通り続けることで確かめる（新しいテストは足さない）。

## Left to the implementer

- 新しいモジュールの名前と、`run_plan` の中の関数の分け方
- スキーマの YAML の書き方（規則の名前、`extract` を置くかどうか）。REQ-core-192 の形を満たす限り
- 具体例のテストで、計画書の本文をテストの中の文字列で持つか、`fixtures/` の下のファイルで持つか
- 使い方の表示（`print_help`）に plan の1行を足すときの説明の文言

## Stop conditions

- 同梱のスキーマで REQ-core-192 のどれかの条件が表せないと分かったとき（mds の規則で書けない、書いても具体例の結果にならない）。mds を広げずに brainstorm へ返す（A5、A7）
- mds が同じ番号のステップ（`### S1:` が2つ）を自分で指摘すると分かったとき（A29 はその場合この決定に戻ると決めている）
- `Document::parse` が、mds の使う読み方で計画書に対して `Err` を返しうると分かったとき。仕様はそのときの`停止`の理由を決めていない
- `.kotowari/schemas/plan.yaml` を足したことで、`kotowari check` や `kotowari-mds check` がこのリポジトリの `.kotowari/schemas/` を読んで振る舞いを変えるとき
- 既存のテストが、この計画の変更（コマンドの一覧の文言を除く）で落ちるとき

## Test command

このリポジトリの決まり（`PROJECT.md`）どおり `CARGO_BUILD_JOBS=4 cargo test --workspace`。1つのテストのファイルだけなら `CARGO_BUILD_JOBS=4 cargo test --test step14_plan`。実時間を待つテストは書かない（`PROJECT.md` の変異テストの節）。

## Out of scope

- 計画書を `kotowari check` の範囲に入れること（A10 で棄てた）
- lefthook に `kotowari plan` を載せること（A4）
- mds（`crates/kotowari-markdown-schema`）の変更
- 既存の TBL-core-008 の unknown_line などの detail に出典が無い件（壁打ちの範囲の外）
- 複数モデルのレビューの件（別の計画 `docs/plans/review-panel.md`）

## Steps

## Step 1 — `kotowari plan` の引数を受ける

Purpose: 6つ目のコマンドとして plan を受け、引数の誤りで正しく停止させる。Specification: `docs/ir/core/cli.md#REQ-core-001`、`#REQ-core-002`、`#REQ-core-004`、`docs/ir/core/plan.md#REQ-core-190`、`docs/ir/core/cli-environment.md#TBL-core-020`。
Prerequisites: なし。
May change: `crates/kotowari-core/src/lib.rs`、`tests/step1_cli.rs`、`tests/step14_plan.rs`（新規）。
Done when: `kotowari` を引数なしで実行すると標準エラーの1行目が "argument error: expected command: check, list, mutants, plan, query or status" になり、`kotowari plan`（位置引数0個）、`kotowari plan a.md b.md`、`kotowari plan --config <file> a.md` がどれも終了コード2で "argument error: " から始まり、`--format` と位置引数の順を問わずに受ける。
Shown by: test — EX-core-219、EX-core-241（既存のテストの期待の文言と、"five" を含むテストの名前を6つに直す）、EX-core-342、EX-core-343、EX-core-357、REQ-core-002 の順を問わないことを1件。
Left to the implementer: `Cli::Plan` の持つ値の名前。S2 までの間 `run` の plan の腕が何をするか（S2 で置き換える）。`req_001_five_commands_only` に plan が通る場合を足すのは S2 で行い、S1 ではテストの名前を6つに直すだけにする。
Stop and hand back if: `--config` の誤りの判定が、`--config` の指す先の検査（REQ-core-004 の「ディレクトリのとき」）と順序でぶつかり、どちらの詳細を出すかが仕様から決まらないとき。

## Step 2 — 計画書を読み、指摘を invalid_plan で出す

Purpose: `kotowari plan <path>` が計画書のファイルだけを読み、同梱のスキーマで検証した指摘を invalid_plan に写して JSON と文字で出す配線を作る。Specification: `docs/ir/core/plan.md#REQ-core-191`、`#REQ-core-193`、`#REQ-core-194`、`#REQ-core-196`、`#REQ-core-197`、`docs/ir/core/cli.md#TBL-core-001`、`docs/ir/core/output.md#TBL-core-005`、`#TBL-core-006`、`docs/ir/core/findings.md#TBL-core-008`。
Prerequisites: S1。
May change: `crates/kotowari-core/src/lib.rs`、`crates/kotowari-core/src/schema.rs`、新しいモジュール、`.kotowari/schemas/plan.yaml`（新規）、`tests/step1_cli.rs`、`tests/step14_plan.rs`、`agent/skills/kotowari/references/findings.md`（Kind の表に invalid_plan の1行を足すことだけ。既存の REQ-core-125 のテストが指摘の種類とこの表を突き合わせるので、種類を足すのと同じコミットで足す）。
Done when: スキーマは "reading: line" を宣言し（A23）、題名、計画全体の節の全部（Test command は任意）、`## Steps` の下のステップの項目、ステップの8つの欄を宣言している（閉じた世界なので、形の揃った計画書で指摘0件を返すにはここまで要る）。計画書が無い・ディレクトリ・UTF-8 でないときにそれぞれの理由で停止し、壊れた `.kotowari/config.yaml` があっても停止せず、欄の欠けたステップの計画書で invalid_plan の誤りが "path"、"line"、detail（スキーマの側の種類に ": " と詳細）を持って出て、JSON の最上位が "findings" と "counts" だけで、基準の外の計画書の "path" が "../" を含み、frontmatter は中身を問わず読まれない。
Shown by: test — EX-core-333、EX-core-344、EX-core-345、EX-core-346、EX-core-358、EX-core-359、EX-core-360、EX-core-361、`tests/step1_cli.rs` の REQ-core-001 のテストに形の揃った計画書で `kotowari plan` が終了コード0になる場合を足すこと、`--format text` で invalid_plan が1行 "パス:行 [error] invalid_plan detail" の形で出ること（REQ-core-025 はコマンドを問わない）、REQ-core-191 の同梱のスキーマが `parse_schema` を通ること（`schema.rs` の既存の REQ-core-168 のテストに倣う）、REQ-core-193 の行の無い指摘で "line" が null になること。
Left to the implementer: なし（スキーマの側の種類の文字列は mds の `FindingKind::as_str` をそのまま使う）。
Stop and hand back if: スキーマの側の指摘の詳細が空になる種類があり、detail が「種類: 」で終わってしまうとき。

## Step 3 — 計画書の形の規則をスキーマに書く

Purpose: S2 で宣言した節・ステップ・欄の上に、REQ-core-192 の残りの条件（欄の順と1回ずつ、Shown by の語、一覧の記号、計画全体の節に置けるものと置けないもの、最初のステップより前の行、検査しないもの）を足し、各条件を具体例で確かめる。Specification: `docs/ir/core/plan.md#REQ-core-192`。
Prerequisites: S2。
May change: `.kotowari/schemas/plan.yaml`、`tests/step14_plan.rs`。
Done when: REQ-core-192 の各条件（題名、節の集合と数、Test command の任意、ほかの節の禁止、ステップの見出しと1つ以上、最初のステップより前の行の禁止、8つの欄の順と1回ずつ、ステップの下のほかの行の禁止、一覧の記号、Shown by の語、計画全体の節に置けるものと置けないもの、検査しないもの）が、形の揃った計画書で指摘0件、外した計画書で invalid_plan として観測できる。スキーマは "reading: line" を宣言する（A23）。
Shown by: test — EX-core-332、EX-core-334〜EX-core-341、EX-core-347〜EX-core-356。
Left to the implementer: 規則の宣言の順序とまとめ方。
Stop and hand back if: 上の Stop conditions の1つ目か2つ目に当たったとき。

## Step 4 — skill の references を本体と揃える

Purpose: 例の計画書を置いて本体のスキーマとの一致をテストし、kotowari スキルの references に plan と invalid_plan を足す。Specification: `docs/ir/core/skill-references.md#REQ-core-195`（と既存の REQ-core-125）。
Prerequisites: S3。
May change: `agent/skills/kotowari-plan/references/plan-example.md`（新規）、`agent/skills/kotowari-plan/references/step-template.md`、`agent/skills/kotowari/references/findings.md`、`agent/skills/kotowari/SKILL.md`、`agent/skills/README.md`、`tests/step7_skill_references.rs`。
Done when: `plan-example.md` が REQ-core-192 の形の計画書1つで、本体のコードで読むと指摘0件になり、`step-template.md` は欄の説明（各欄を1行に書くこと、一覧の行で書くこと、frontmatter を置かないこと）と `plan-example.md` へのリンクになり、`findings.md` に `kotowari plan` の出力（最上位は findings と counts、終了コード）の説明があり、kotowari スキルの説明のコマンドの一覧に plan がある。
Shown by: test — REQ-core-195（`tests/step7_skill_references.rs` に1件）、既存の REQ-core-125 のテストが通ること。
Left to the implementer: `step-template.md` の欄の説明の言い回し（今の Guidance per field の中身は残す）。
Stop and hand back if: kotowari スキルの本文に工程の skill（kotowari-plan など）を名指しする必要が出たとき（kotowari スキルは工程の skill を名指ししない。workflow-split A5）。

## Step 5 — kotowari-plan の工程で kotowari plan を回す

Purpose: 工程の skill が新しい書式で計画書を書き、承認を求める前に `kotowari plan` で形を検査するようにする。Specification: `docs/decision/records/2026-09-24-plan-schema.md` の A4、A8、A12、A23（工程の skill の変更は IR に書かない。A2 の改訂後も同じ）。
Prerequisites: S4。
May change: `agent/skills/kotowari-plan/SKILL.md`。
Done when: Finishing の自己点検に「`kotowari plan <計画書のパス>` を回し、終了コード1なら指摘を直して0になるまで回し、2なら理由を人に見せて止まる」が入り、計画書に frontmatter を置かないこと、欄の値を1行に書くことが書かれ、lefthook に載せない旨と矛盾する記述が無い。
Shown by: artifact — `agent/skills/kotowari-plan/SKILL.md`。形式の検査は無い。独立したレビューが A4、A8、A12、A23 と突き合わせて判断できる状態にする。
Left to the implementer: 本文のどこに書くか（Finishing の1の中か、その前の節か）。
Stop and hand back if: 既存の本文の「The plan itself runs neither `kotowari check` nor `kotowari status`」と、`kotowari plan` を回すことが読み手にとって矛盾して見え、どちらかの意味を変えないと直せないとき。

## Step 6 — 計画が覆うものを確かめる

Purpose: この計画が覆う要求と具体例にテストの印があり、変えたファイルと覆う ID に check の誤りが無いことを確かめる。Specification: 上の Specification の全部。
Prerequisites: S1〜S5。
May change: なし（見つかった誤りは該当するステップのファイルで直す）。
Done when: 下の3つのコマンドがすべて期待どおりになる。
Shown by: check — 次の3つを順に実行する。
1) `CARGO_BUILD_JOBS=4 cargo test --workspace` が通る。
2) `for id in REQ-core-190 REQ-core-191 REQ-core-192 REQ-core-193 REQ-core-194 REQ-core-195 REQ-core-196 REQ-core-197 EX-core-219 EX-core-241 $(seq -f 'EX-core-%g' 332 361); do cargo run -q -p kotowari -- query $id | jq -e '.items[0].tests != []' >/dev/null || { echo "no test: $id"; exit 1; }; done` が何も出さずに終わる（REQ-core-179 は verification が review、REQ-core-011、172、175 と表は振る舞いが変わらないので外す）。
3) `base=$(git merge-base main HEAD); files=$(git diff --name-only $base..HEAD | jq -R . | jq -s .); CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format json | jq --argjson files "$files" '[.findings[] | select(.severity == "error") | select((.path as $p | $files | index($p)) or (.detail | test("REQ-core-19[0-7]|EX-core-3[3-6][0-9]")))] | length'` が 0 を出す。
Left to the implementer: なし。
Stop and hand back if: 覆う ID のテストが、Evidence conditions を満たさない形（テストの側の都合の固定）でしか書けないとき。
