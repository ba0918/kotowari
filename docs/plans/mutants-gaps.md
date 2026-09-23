# 計画: 変異テストで見つかった仕様の穴を実装とテストで埋める

## Goal

ブランチ mutants-followup で、記録 A1〜A14 の振る舞いが実装に入り、足した要求と具体例のすべてに印のあるテストがあり、前回生き残った変異のうち仕様の判断待ちだった18件が caught になる。

## Specification

- 判断の記録 `docs/decision/records/2026-09-23-mutants-gaps.md`（A1〜A14。A5 は A10 で改めた）
- IR の置き場 `docs/ir`。対象の ID:
  - kotowari: `docs/ir/core/ir-document.md#TBL-core-010`、`#EX-core-281`、`#EX-core-282`
  - mds: `docs/ir/schema/extraction.md#REQ-schema-064`、`#TBL-schema-008`、`#EX-schema-058`〜`#EX-schema-061`、`#TBL-schema-011`（A2。IR は変えていない）
  - mds: `docs/ir/schema/cli.md#REQ-schema-065`、`#REQ-schema-066`、`#TBL-schema-009`、`#EX-schema-062`〜`#EX-schema-065`
  - mds: `docs/ir/schema/schema-resolution.md#TBL-schema-003`、`#REQ-schema-013`、`#EX-schema-066`〜`#EX-schema-070`

要求と具体例は `kotowari query <ID>` で読む。前回生き残った変異の一覧は `/home/mizumi/develop/kotowari/.agents/cycle/mutants-survivors.txt`、その分類は記録の Context の段落にある。

## Approach and why

振る舞いを変えるもの（A1、A2、A6）と、今の振る舞いを追認してテストで押さえるもの（A4、A7〜A10）に分ける。変えるものはテストファーストで RED を確かめてから直す。追認するものは今のコードでテストが通るはずなので、テストを書いたら、対応する変異を当てたときに落ちることを変異の再実行で示す（今のコードで通るテストは RED を作れないため、変異が RED の代わりになる）。

A1 は kotowari-core の `split_lines`（`crates/kotowari-core/src/ir.rs`）を単独の "\r" でも区切るようにする。mds の行の分け方（`crates/kotowari-markdown-schema/src/document.rs` の行の分割）と同じ結果になることが狙い。A2 は mds の行の読み方（`crates/kotowari-markdown-schema/src/line_reading.rs` の表の判定）を、段落の読み方と同じく GFM の表を読むように直す。A6 は serde の誤りの説明に、欄の名前と受け付ける語の一覧を入れる（`reading: foo`、`open: 3`、`"$schema"` の型の違いなど）。

## Scope of change

- `crates/kotowari-core/src/ir.rs` とその周り（A1）
- `crates/kotowari-markdown-schema/src/`（A2、A6）
- `tests/`、`crates/kotowari-markdown-schema/tests/`、各 crate の `#[cfg(test)]`
- `skills/kotowari/references/ir-form.md`（A13。行の数え方の一文）
- `.kotowari/equivalents.yaml`（変異の再実行で等価と分かったものだけ。mutants.md の手順のとおり）

変えないもの: `docs/ir/` と `docs/decision/`（仕様は承認済み）、`scripts/mutants.sh`。

## Step order and prerequisites

1 → 2 → 3 → 4 → 5。1〜4 は互いに独立だが、5 の変異の再実行は1〜4 が終わってからまとめて行う。

## Step 1 — kotowari が単独の CR を行の区切りに数える

Purpose: kotowari と mds の行番号を揃える。Specification: `docs/ir/core/ir-document.md#TBL-core-010`、`#EX-core-281`、`#EX-core-282`、記録 #A1、#A13。
Prerequisites: なし。
May change: `crates/kotowari-core/src/ir.rs`、`tests/`（step2_ir.rs など行の数え方を確かめる所）、`skills/kotowari/references/ir-form.md`。
Done when: EX-core-281 と EX-core-282 の印を付けたテストが、直す前に落ち、直した後に通る。ir-form.md の行の数え方の一文が単独の "\r" を含む。
Shown by: test — `CARGO_BUILD_JOBS=4 cargo test --workspace`。RED と GREEN の両方の出力を報告に載せる。
Left to the implementer: `split_lines` の書き方、テストの名前。
Stop and hand back if: 単独の "\r" を区切りにすると、既存のテストが TBL-core-010 の既存の行（"\r\n" を1行に数える）と食い違う形で落ちる。

## Step 2 — 行の読み方で縦棒の無い GFM の表を表と読む

Purpose: 行の読み方を TBL-schema-011 に合わせる。Specification: `docs/ir/schema/extraction.md#TBL-schema-011`、記録 #A2。
Prerequisites: なし。
May change: `crates/kotowari-markdown-schema/src/line_reading.rs`、mds のテスト。
Done when: 見出しの行に縦棒が無い表（`abc` / `|---|` / `|1|`）と、区切りの行に縦棒が無い1列の表（`| a |` / `:-` / `| 1 |`）を、行の読み方でも段落の読み方と同じく表と読む。TBL-schema-011 の印を付けたテストが直す前に落ち、直した後に通る。
Shown by: test — `CARGO_BUILD_JOBS=4 cargo test -p kotowari-markdown-schema`、最後に `--workspace`。
Left to the implementer: 判定の書き方（GFM の表の規則を満たすこと）。
Stop and hand back if: 表と読むように直すと、行の読み方の既存の具体例（縦棒で始まり区切りの行を持たない行は文、など）と食い違う。

## Step 3 — 停止の説明に欄の名前と受け付ける語の一覧を入れる

Purpose: REQ-schema-065 を満たす。Specification: `docs/ir/schema/cli.md#REQ-schema-065`、`#EX-schema-062`、`#EX-schema-063`、記録 #A6。
Prerequisites: なし。
May change: `crates/kotowari-markdown-schema/src/`（schema.rs、frontmatter.rs、main.rs の停止の組み立て）、mds のテスト。
Done when: 値の型や語が違って停止するときの説明に、欄の名前と、期待した型か受け付ける語の一覧が含まれる。EX-schema-063（`reading: foo`）の印を付けたテストが直す前に落ち、直した後に通る。テストは文言全体でなく、欄の名前と語が含まれることを確かめる（記録 #A6 のとおり文言は契約でない）。
Shown by: test — `CARGO_BUILD_JOBS=4 cargo test -p kotowari-markdown-schema`。
Left to the implementer: 説明の文言、serde の誤りから欄の名前を取り出す方法。
Stop and hand back if: serde の誤りから欄の名前が取れず、スキーマの読み取りの作りを大きく変えないと満たせない。

## Step 4 — 今の振る舞いを追認した要求にテストを付ける

Purpose: 足した要求と具体例に印のあるテストを揃える。Specification: `docs/ir/schema/extraction.md#REQ-schema-064`、`#TBL-schema-008`、`#EX-schema-058`〜`#EX-schema-061`、`docs/ir/schema/cli.md#REQ-schema-066`、`#EX-schema-064`、`#EX-schema-065`、`docs/ir/schema/schema-resolution.md#TBL-schema-003`、`#REQ-schema-013`、`#EX-schema-066`〜`#EX-schema-070`、記録 #A4、#A7〜#A10。
Prerequisites: なし。
May change: mds のテスト（URL のスキーマは既存のテストの手元のサーバの仕組みを使う）。
Done when: 上の ID それぞれに印のあるテストがあり、全部通る。
Shown by: test — `CARGO_BUILD_JOBS=4 cargo test --workspace`。今のコードで通ることは RED にならないので、Step 5 の変異の再実行で落ちることを示す。
Left to the implementer: テストの組み方。
Stop and hand back if: EX-schema-067（取得全体が10秒を超えたら停止）のテストが10秒以上かかる。1本のために、変異テストの1件あたりの時間が10秒以上延びるので、この1本の扱い（時間を注入できる作りにするか、テストの時間を受け入れるか）は利用者が決める。

## Step 5 — 変異を再実行して片付いたことを示す

Purpose: 前回の判断待ちの18件が caught になったことを示す。Specification: 記録 #A3。
Prerequisites: Step 1〜4。
May change: `.kotowari/equivalents.yaml`（mutants.md の等価の手順を踏んだものだけ）。
Done when: 下の再実行で、前回の一覧の18件（ir.rs:310、line_reading.rs:329/353/354、document.rs:664〜667、extract.rs:761、frontmatter.rs:56、main.rs:241 の2件、main.rs:316、main.rs:488/493 の4件、schema.rs:352 の2件、schema.rs:416/494）が caught か等価の一覧に載っている。
Shown by: check — `scripts/mutants.sh full -- -f <ファイル> -F '<関数名>'` をファイルごとに（cargo mutants を直接走らせない。メモリの上限つきで走らせるため）。最後に `CARGO_BUILD_JOBS=4 cargo run -q -- check --format text` の誤りが0。
Left to the implementer: 再実行のまとめ方。
Stop and hand back if: 新しく生き残る変異が出て、その区別に仕様の判断が要る。

## Verification map

| 決定 | Step |
|---|---|
| A1、A13 | 1 |
| A2 | 2 |
| A6 | 3 |
| A4、A7、A8、A9、A10、A11、A12 | 4 |
| A3（判断待ちの18件を片付ける） | 5 |
| A14（cli.md を分けない） | 変えることは無い |

## Left to the implementer

- コミットは Step ごとを基本に、1コミット1関心
- 調べるための一時ファイルは scratchpad（`/tmp/claude-1000/-home-mizumi-develop-kotowari/f69da7f6-4a28-4b1d-90b2-71ddebd7fcdf/scratchpad/`）に置き、リポジトリに入れない

## Stop conditions

一般の4つ（承認した内容からの逸脱か意味の欠落、不可逆・特権・危険な操作、広がる事故、やり方を変えても進まない）に加えて、各 Step の Stop and hand back if。

## Test command

`CARGO_BUILD_JOBS=4 cargo test --workspace`（PROJECT.md のとおり）。

最後の確認: 上の対象の ID のそれぞれで `kotowari query <ID> | jq '.items[0].tests | length'` が1以上。`kotowari check` でブランチで変えたファイルと対象の ID への誤りが0。

## Out of scope

- 英語の曖昧語を既定に足すこと
- 取得のタイムアウト以外の TODO の項目
