# Plan: ガイドを書いて見つかった仕様の穴を実装とテストで埋める

## Goal

2026-09-24 に仕様で決めた振る舞い（gherkin でないブロック、設定の誤りの1行の説明、tests.rules の誤りの詳細、位置引数の後ろのオプション、plan の行と text）が実装とテストで確かめられ、利用者向けのガイドが今の IR に追従している。

## Specification

IR は `docs/ir/`。決定は `docs/decision/records/2026-09-24-guide-gaps.md`。この計画が扱うのは次のとおり。

- `docs/ir/core/findings.md#REQ-core-174` と EX-core-376、EX-core-377
- `docs/ir/core/query-rules.md#REQ-core-189` と EX-core-378、EX-core-379
- `docs/ir/core/cli.md#REQ-core-002` と EX-core-380
- `docs/ir/core/plan.md#REQ-core-193`、`docs/ir/core/plan.md#REQ-core-207` と EX-core-381、EX-core-382
- `docs/ir/core/config.md#REQ-core-014` と EX-core-383
- `docs/ir/core/guides.md#REQ-core-204`（ガイドの見直し）

要求とシナリオの本文は `kotowari query <ID>` で読む。

## Approach and why

- 実装が仕様と違うのは2か所だけで、ほかは既に仕様どおりの振る舞いをテストで固定する。gherkin でないブロックの中の行に invalid_gherkin_line を出しているのは `crates/kotowari-core/src/ir.rs` のシナリオの読み取りで、gherkin のブロックに限る。設定のキーの重複は `crates/kotowari-core/src/config.rs` の `Config::parse` が YAML のライブラリの誤りをそのまま詳細にしているので、重複だけを見分けて "duplicate key: キー" にする。知らない言語のルールは `crates/kotowari-core/src/test_queries.rs` のルールの読み込みで、ルールとして読む前に "language" を見て "unknown language: 言語" にする。
- 説明の文は仕様の契約ではない（TBL-core-020）ので、テストはシナリオが決めた部分（"duplicate key: ir"、"unknown language: cobol"、標準エラーが1行であること、パス）だけを確かめる。
- 仕様の変更で `docs/guides/` の節に guide_stale の注意が出ている。実装が決まってから、ガイドの書き方（`agent/skills/kotowari/references/guides.md` の見直しの手順）で節を読み直し、ずれていれば直してから指紋を書き写す。ガイドの一部の記述（EX-core-376 の二重の指摘、キーの重複の文言、知らない言語の文言）は今の実装の振る舞いを書いているので、実装を直した後に書き直す。

## Scope of change

- `crates/kotowari-core/src/ir.rs`
- `crates/kotowari-core/src/config.rs`
- `crates/kotowari-core/src/test_queries.rs`
- `tests/` の既存のテストのファイル（step1_config.rs、step2_ir.rs、step13_test_languages.rs、step14_plan.rs、step1_cli.rs のうち該当するもの）
- `docs/guides/` の guide_stale が出ている節と、実装を直したことで記述が古くなる節

## Step order and prerequisites

S1 から S3 は互いに独立している。S4 は S1 から S3 の振る舞いを書くので最後。S5 が全体を確かめる。

## Verification map

| Step | Requirements | Examples |
|---|---|---|
| S1 | REQ-core-174 | EX-core-376、EX-core-377 |
| S2 | REQ-core-014、REQ-core-189 | EX-core-378、EX-core-379、EX-core-383 |
| S3 | REQ-core-002、REQ-core-193、REQ-core-207 | EX-core-380、EX-core-381、EX-core-382 |
| S4 | REQ-core-204 | なし |
| S5 | この表のすべて | この表のすべて |

## Left to the implementer

- 関数の名前、テストの名前、重複したキーの名前を取り出す方法

## Stop conditions

- YAML のライブラリが、重複したキーの名前を取り出せる形で誤りを返さない
- ast-grep のルールの読み込みの前に "language" を読むと、別名（"ts"、"py"）の受け方が変わる
- 既存のテストが、この計画と関係の無い理由で落ちる

## Test command

```sh
CARGO_BUILD_JOBS=4 cargo test --workspace
```

## Out of scope

- TODO.md に残した「REQ-core-174 の前からの出典の漏れ」
- query の停止の文言（決定 A5）と、Context の無い判断の記録の検査（決定 A9）。仕様を変えないと決めた

## Steps

### S1: gherkin でないブロックの中の行を gherkin として読まない

- Purpose: "## Examples" の下の gherkin でないコードブロックに unknown_code_block だけを出し、用語集の題名が "# Glossary" でないときの glossary_title_invalid をテストで固定する
- Specification: `docs/ir/core/findings.md#REQ-core-174`
- Prerequisites: none
- May change: `crates/kotowari-core/src/ir.rs`, `tests/step2_ir.rs`
- Done when: EX-core-376 の文書で unknown_code_block が出て "メモ" の行に invalid_gherkin_line が出ず、EX-core-377 の用語集で glossary_title_invalid が出る
- Shown by: test — EX-core-376、EX-core-377
- Left to the implementer: none
- Stop and hand back if: gherkin でないブロックの中の行を読まないと、gherkin のブロックの読み取りの既存のテストが落ちる

### S2: 設定と tests.rules の誤りを1行の説明にする

- Purpose: 設定のキーの重複を "duplicate key: キー"、知らない言語のルールを "unknown language: 言語" の1行で止め、tests.rules の無いファイルの詳細がルールのファイルを指すことをテストで固定する
- Specification: `docs/ir/core/config.md#REQ-core-014`, `docs/ir/core/query-rules.md#REQ-core-189`
- Prerequisites: none
- May change: `crates/kotowari-core/src/config.rs`, `crates/kotowari-core/src/test_queries.rs`, `tests/step1_config.rs`, `tests/step13_test_languages.rs`
- Done when: EX-core-383 で標準エラーが "config error: " で始まり "duplicate key: ir" を含む1行になり、EX-core-379 で "r.yml" と "unknown language: cobol" を含む1行になり、EX-core-378 で "rules/missing.yml" を含み ".kotowari/config.yaml" を含まない
- Shown by: test — EX-core-383、EX-core-379、EX-core-378
- Left to the implementer: 重複したキーの名前を取り出す方法
- Stop and hand back if: 計画の Stop conditions の1つ目か2つ目に当たる

### S3: 今の振る舞いを位置引数の順と plan の形のテストで固定する

- Purpose: query の ID の後ろのオプション、plan の欠けた欄の行、plan の text が0件で空になることをテストで固定する
- Specification: `docs/ir/core/cli.md#REQ-core-002`, `docs/ir/core/plan.md#REQ-core-193`, `docs/ir/core/plan.md#REQ-core-207`
- Prerequisites: none
- May change: `tests/step1_cli.rs`, `tests/step14_plan.rs`
- Done when: EX-core-380 で終了コードが 0、EX-core-381 で標準出力が "docs/plans/a.md:5 [error] invalid_plan " で始まる行だけ、EX-core-382 で標準出力が空になる
- Shown by: test — EX-core-380、EX-core-381、EX-core-382
- Left to the implementer: none
- Stop and hand back if: 今の実装がこれらのシナリオと違う振る舞いをする（仕様は今の振る舞いを写したので、違えば観測が誤っている）

### S4: ガイドを今の IR と実装に追従させる

- Purpose: guide_stale の出ている節と、S1 と S2 で記述が古くなった節を読み直して直し、指紋を書き写す
- Specification: `docs/ir/core/guides.md#REQ-core-204`
- Prerequisites: S1, S2, S3
- May change: `docs/guides/`
- Done when: `kotowari check` に docs/guides への指摘が無く、findings.md の gherkin の二重の指摘の記述、config.md と cli.md のキーの重複と知らない言語の文言の記述が今の振る舞いに合っている
- Shown by: artifact — `docs/guides/` の変更。`agent/skills/kotowari/references/guides.md` の見直しの手順のとおりに、節ごとに `kotowari query` の本文と突き合わせて確かめる
- Left to the implementer: 書き直しの文面
- Stop and hand back if: 節を直すのに仕様に無い振る舞いを書く必要が出てくる

### S5: 計画の範囲の検査が揃っていることを確かめる

- Purpose: 扱う要求とシナリオに印のあるテストがあり、誤りとガイドへの指摘が無いことを確かめる
- Specification: `docs/ir/core/plan.md#REQ-core-207`, `docs/ir/core/guides.md#REQ-core-204`
- Prerequisites: S1, S2, S3, S4
- May change: none
- Done when: テストがすべて通り、`kotowari check` の誤りが0件で、docs/guides への指摘が0件
- Shown by: check — `CARGO_BUILD_JOBS=4 cargo test --workspace`、`CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format text | grep -E '\[error\]|docs/guides'`
- Left to the implementer: none
- Stop and hand back if: この計画の外の誤りが新しく出る
