# 計画: IR の型の語を英語に切り替える

## Goal

kotowari が読む IR の型の語（節の見出し、欄の名前、用語集の題名と列、問題の記録の節）が英語になり、このリポジトリの IR、テスト、kotowari スキル、README がすべてその語で揃う。

## Specification

- 判断の記録 `docs/decision/records/2026-09-23-ir-english-tokens.md`（A1〜A8）。英語の語の対応表は A2、切り替え方は A3 と A7
- IR の置き場 `docs/ir`。この計画は新しい要求を足さない。型の語を名指しする既存の要求と具体例は、承認のコミット `a251caa` で既に英語の語に直してある。対象の ID は次の34件:
  - `docs/ir/core/ir-items.md#REQ-core-044`、`#REQ-core-046`、`#REQ-core-047`、`#REQ-core-048`、`#REQ-core-051`、`#TBL-core-011`、`#EX-core-273`、`#EX-core-274`、`#EX-core-276`
  - `docs/ir/core/form-contract.md#REQ-core-170`、`#REQ-core-179`
  - `docs/ir/core/terms-form.md#REQ-core-117`
  - `docs/ir/core/skill-references.md#REQ-core-125`、`#REQ-core-127`
  - `docs/ir/core/list.md#REQ-core-155`、`#TBL-core-026`、`#EX-core-246`
  - `docs/ir/core/query.md#TBL-core-027`、`#EX-core-250`、`#EX-core-257`、`#EX-core-277`
  - `docs/ir/core/status.md#TBL-core-028`、`#EX-core-258`、`#EX-core-259`
  - `docs/ir/core/findings.md#REQ-core-174`、`#EX-core-266`
  - `docs/ir/core/coverage.md#REQ-core-085`、`#REQ-core-137`
  - `docs/ir/core/ir-missing.md#REQ-core-098`、`docs/ir/core/ir-references.md#REQ-core-054`、`docs/ir/core/decision-records.md#REQ-core-097`、`docs/ir/core/sources.md#REQ-core-115`、`docs/ir/core/finding-map.md#TBL-core-029`、`docs/ir/core/cli-environment.md#EX-core-269`

検証が review の3件（`REQ-core-097`、`REQ-core-170`、`REQ-core-179`）と、今も印の無い `TBL-core-029` を除き、どの ID も既に印のあるテストを持つ。この計画はテストを新しく足すのではなく、既存のテストが日本語の語で組んでいる IR の断片を英語の語に直し、それが通るようにエンジンを切り替える。

## Approach and why

型の語はスキーマ `.mds/schemas/ir.yaml`、`context.yaml`、`flags.yaml` の宣言で決まり、`crates/kotowari-core/src/schema.rs` が `include_str!` で埋め込む。kotowari-core の中で語を文字として持つのは、`src/ir.rs` の用語集のセルの読み取り（`cell("用語")` など）と、`src/finding_map.rs` の欄の名前から指摘の種類への写し（`"出典"`、`"種類"` などの分岐）の2か所。

順序を決めている制約は pre-commit のフック。`lefthook.yml` の `kotowari-check` は、コミットのたびにリポジトリの今のコードで `docs/ir` を検査し、IR 側の誤りがあればコミットを止める。そのため、スキーマとコードの切り替えと、このリポジトリの IR 40文書の書き換えは、同じ1コミットでないと通らない（片方だけだと IR 側の誤りが数百件出る）。テストはフックで走らないので、先に直してよい。

1. テストと fixture の中の IR の断片を英語の語に直す（RED: エンジンが日本語のままなので落ちる）
2. スキーマとコードを切り替え、同じコミットで `docs/ir` の型の語を書き換える（GREEN）
3. kotowari スキル（`skills/kotowari/`）を英語にする（A5、A6）
4. 工程の skill、README、`docs/spec/kotowari-skill.md` に残る日本語の型の語と、A4 の一言を直す

`docs/ir` の書き換えは機械的な置き換えで行う。置き換えるのは行頭の型の語だけ（`## 要求` などの見出しの行、`- 種類:` などの欄の行、用語集の `# 用語集` の題名とヘッダの行 `| 用語 | 意味 | 出典 |`、`FLAGS.md` の `## 問題の記録`）。コードブロック（gherkin）の中と、文の中の語は置き換えない。文の中で型の語を名指ししている箇所は承認のコミットで既に英語になっている。

## Scope of change

- `.mds/schemas/ir.yaml`、`.mds/schemas/context.yaml`、`.mds/schemas/flags.yaml`
- `crates/kotowari-core/src/`（`ir.rs`、`finding_map.rs` と、各ファイルのテストのモジュールの中の IR の断片）
- `tests/`（IR の断片を持つテスト。`step1_cli.rs`、`step1_config.rs`、`step2_ir.rs`、`step3_sources_terms.rs`、`step4_test_discovery.rs`、`step5_findings.rs`、`step6_output.rs`、`step10_list.rs`、`step11_query.rs`、`step12_status.rs`、`step7_skill_references.rs`）と `fixtures/`
- `docs/ir/` の全文書（型の語の行だけ）
- `skills/kotowari/`（SKILL.md と references 7つ）、`skills/kotowari-brainstorm/SKILL.md` ほか型の語を書いている工程の skill
- `README.md`、`docs/spec/kotowari-skill.md`

変えないもの: `experiments/`、`crates/kotowari-markdown-schema/`（汎用のエンジンで、欄の名前はスキーマが決める。その fixture の日本語の名前はエンジンのテストの題材）、`docs/decision/`（判断の記録と ADR）、`.kotowari/config.yaml` の `vague_words`（A4）、`docs/spec/` の `kotowari-skill.md` 以外の文書。

## Step order and prerequisites

1 → 2 → 3 → 4。2 は 1 のテストを GREEN にする。3 は 2 の後でないと references に書く語が実物と食い違う。4 は 3 の後（工程の skill は kotowari スキルの references を名指しする）。

## Step 1 — テストと fixture の IR の断片を英語の語にする

Purpose: エンジンの切り替えを確かめるテストを先に用意する。Specification: 判断の記録 #A2、#A3、#A7。上の34件の ID。
Prerequisites: ブランチ `ir-english-tokens` の `a251caa` 以降。
May change: `tests/`、`fixtures/`、`crates/kotowari-core/src/` の `#[cfg(test)]` のモジュールの中だけ。
Done when: テストと fixture の中の IR の断片（Markdown の文字列）の型の語が A2 の英語の語になっている。テストが期待する指摘の detail に欄の名前が入るもの（missing_field の detail など）も英語の名前になっている（A7）。`@kotowari[...]` の印と、テストの名前は変えない。
Shown by: test — `CARGO_BUILD_JOBS=4 cargo test --workspace` を走らせ、型の語に触れるテストが落ちることを RED として確かめる（落ちたテストの名前を報告に載せる）。型の語に触れないテストは通ったまま。
Left to the implementer: 置き換えを手で行うかスクリプトで行うか。RED と GREEN（Step 2）を別のコミットにするか1つにするか。
Stop and hand back if: テストの中に、日本語の型の語を受け付けないこと自体を確かめたいように読めるもの（日本語の語をわざと誤りとして使うテスト）がある。A3 は受け付けないと決めたが、そのテストを残すかは決めていない。

## Step 2 — スキーマとコードを切り替え、このリポジトリの IR を書き換える

Purpose: kotowari が英語の型の語で IR を読むようにする。Specification: #A2、#A3、#A7、#A8。`docs/ir/core/form-contract.md#REQ-core-179`、`docs/ir/core/terms-form.md#REQ-core-117`、`docs/ir/core/ir-items.md#TBL-core-011`。
Prerequisites: Step 1。
May change: `.mds/schemas/` の3つ、`crates/kotowari-core/src/ir.rs`、`crates/kotowari-core/src/finding_map.rs`、`docs/ir/` の全文書。
Done when: スキーマの節、欄、題名の `pattern`、表の `header` と、`when` の条件の欄の名前が A2 の英語の語になっている。`ir.rs` のセルの名前と `finding_map.rs` の欄の名前の分岐が英語の名前になっている。`docs/ir/` の全文書の行頭の型の語が英語で、コードブロックの中と文の中は変わっていない。これらが1コミットに入っている。
Shown by: check — 次を順に走らせる。(1) `CARGO_BUILD_JOBS=4 cargo test --workspace` がすべて通る（Step 1 の RED が GREEN になる）。(2) `CARGO_BUILD_JOBS=4 cargo run -q -- check --format text` の誤りが0件。(3) `rg -n '^(## (要求|決定表|性質|具体例|問題の記録)|- (種類|出典|検証|定義|確かめ方|関係):|# 用語集)$|^(## |- |# )(要求|決定表|性質|具体例|問題の記録|種類|出典|検証|定義|確かめ方|関係|用語集)' docs/ir .mds/schemas` が0件。(4) `rg -n '"(種類|出典|検証|定義|確かめ方|関係|用語|意味)"' crates/kotowari-core/src --glob '!**/tests/**'` の当たりがテストのモジュールの中だけ。
Left to the implementer: `docs/ir` の書き換えのスクリプトの書き方（上の Approach の置き換えの範囲を守ること）。
Stop and hand back if: スキーマの宣言を英語にすると、`crates/kotowari-markdown-schema` の側の変更が要る（汎用のエンジンは欄の名前を問わないはずで、要るなら前提が崩れている）。または `docs/ir` の書き換えの後に check の誤りが残り、その原因が型の語の置き換えでない。

## Step 3 — kotowari スキルを英語にする

Purpose: kotowari スキルを工程の skill と同じ言語にし、新しい型の語で説明する。Specification: #A5、#A6、#A4。`docs/ir/core/skill-references.md#REQ-core-125`、`#REQ-core-127`。
Prerequisites: Step 2。
May change: `skills/kotowari/SKILL.md`、`skills/kotowari/references/` の7つ、`tests/step7_skill_references.rs`。
Done when: SKILL.md と references の本文が英語で、型の語はすべて A2 の英語の語で書かれている。意味は訳す前と変わらない（訳すついでに規則を足したり削ったりしない）。`findings.md` の指摘の種類の表の1列目の見出しが `Kind`、停止の文言の表が `Message` で、`tests/step7_skill_references.rs` がその見出しで表を探す。`config.md` の setup の YAML の `vague_words` の値は日本語の4語のまま（A4）。
Shown by: check — (1) `CARGO_BUILD_JOBS=4 cargo test --workspace --test step7_skill_references` が通る。(2) `rg -n '[ぁ-んァ-ヶ一-龠]' skills/kotowari/` の当たりが、`vague_words` の既定の4語と、日本語の本文の例として意図して残したものだけ（残したものは報告に理由と共に挙げる）。
Left to the implementer: 英語の言い回し。references の中の段落の並び。
Stop and hand back if: 訳す途中で、references の規則が IR（`docs/ir`）と食い違っているのを見つけた（訳で直さず、食い違いとして報告する）。

## Step 4 — 工程の skill、README、スキルの仕様の文書を直す

Purpose: 型の語を書いている残りの文書を揃える。Specification: #A2、#A4、#A5。
Prerequisites: Step 3。
May change: `skills/kotowari-*/`（型の語を書いているところだけ）、`README.md`、`docs/spec/kotowari-skill.md`。
Done when: 工程の skill の本文の中の日本語の型の語（例: `kotowari-brainstorm` の `## 具体例`）が英語の語になっている。README の最小の例が英語の型の語で書かれ、曖昧語の既定が日本語の4語で、`.kotowari/config.yaml` の `vague_words` で上書きできることが一言書いてある（A4）。`docs/spec/kotowari-skill.md` の中で、references の言語と型の語に触れる箇所が今回の決定に合い、経緯の段落に今回の記録へのリンクがある。
Shown by: check — (1) README の最小の例を scratchpad のディレクトリに書き出し、その中で `CARGO_BUILD_JOBS=4 cargo run -q --manifest-path <リポジトリ>/Cargo.toml -- check --format text` が何も出さずに終了コード0で終わる。印を外すと README に書いた2件の指摘がその文面で出る。(2) `rg -n '種類:|出典:|検証:|定義:|確かめ方:|関係:|## (要求|決定表|性質|具体例|問題の記録)|# 用語集' skills/kotowari-*/ README.md` が0件。
Left to the implementer: README の一言の置き場所。
Stop and hand back if: なし（一般の停止条件だけ）。

## Verification map

| 決定・要求 | 確かめる Step |
|---|---|
| A1、A2（英語にする語と対応） | 1、2（テストとスキーマ）、3、4（文書） |
| A3（一度に切り替える） | 2（IR の書き換えを同じコミットで） |
| A4（曖昧語の既定は変えない） | 3（config.md の値）、4（README の一言） |
| A5、A6（kotowari スキルの英語化と表の見出し） | 3。`REQ-core-125`、`REQ-core-127` |
| A7（スキーマ、コード、detail） | 1、2。`REQ-core-179`、`EX-core-259` |
| A8（list の type は null） | 振る舞いは変えない。`TBL-core-026` の既存のテストが通ることを 2 で確かめる |
| 上の34件の ID | 1 で断片を直し、2 で通る。最後の確認で印があることを見る |

## Left to the implementer

- コミットの分け方は Step ごとを基本にする。Step 2 は上の制約で1コミットにする
- 置き換えのスクリプトは scratchpad に置き、リポジトリに入れない

## Stop conditions

一般の4つ（承認した内容からの逸脱か意味の欠落、不可逆・特権・危険な操作、広がる事故、やり方を変えても進まない）に加えて:

- 型の語の置き換えで、A2 の表に無い日本語の型の語が見つかった（表を足すのは仕様の変更）
- pre-push の変異テストが、この変更で見逃しを出した（見逃しは fixer か実装者が直すが、原因が文字の置き換えでない場合は報告する）

## Test command

`CARGO_BUILD_JOBS=4 cargo test --workspace`（PROJECT.md のとおり）。

最後の確認（Step 4 の後）:

- 上の34件の ID のそれぞれで `CARGO_BUILD_JOBS=4 cargo run -q -- query <ID> | jq '.items[0].tests | length'` が1以上（review の3件と `TBL-core-029` は除く。`TBL-core-029` は今も印が無く、この計画で増やさない）
- `CARGO_BUILD_JOBS=4 cargo run -q -- check --format json | jq '[.findings[] | select(.severity == "error")] | length'` が0。ブランチで変えたファイルと上の ID への誤りが無いことを見る（今のリポジトリは誤り0なので、全体で0を見れば足りる）
- `lefthook run pre-commit --no-auto-install --all-files`

## Out of scope

- 手元に入れた `kotowari` のバイナリの入れ直し（マージの後に `cargo install --path .`）
- 英語の曖昧語を既定に足すこと（A4）
- ADR の節の見出し（型の語ではない）
- `experiments/` の IR
