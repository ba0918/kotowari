# Plan: 要求を後回しにする宣言を実装する

## Goal

IR の要求に "- deferred:" の行（要求ごとか文書単位）を書くと、その要求と後回しのシナリオにテストが無くても誤りにならず、形と参照の検査と ID の予約は続き、status と list と query に後回しが出て、後回しとの食い違いが注意で知らされる。

## Specification

IR は `docs/ir/`。決定は `docs/decision/records/2026-09-25-deferred-items.md`（A1〜A26）。この計画が扱うのは次のとおり。

- 新しい要求: `docs/ir/core/deferred.md#REQ-core-208`、`#REQ-core-209`、`#REQ-core-210`、`#REQ-core-211`、`#REQ-core-212`、`#REQ-core-213`
- 新しいシナリオ: EX-core-384〜EX-core-399（EX-core-398 は status.md、EX-core-399 は list.md、ほかは deferred.md）
- 変えた既存の項目: `docs/ir/core/coverage.md#REQ-core-085`、`#REQ-core-137`、`docs/ir/core/ir-items.md#TBL-core-011`、`#REQ-core-046`、`docs/ir/core/findings.md#REQ-core-031`、`#TBL-core-008`、`#TBL-core-009`、`docs/ir/core/finding-order.md#TBL-core-019`、`docs/ir/core/sources.md#REQ-core-115`、`docs/ir/core/form-contract.md#REQ-core-170`、`docs/ir/core/guides.md#REQ-core-203`、`docs/ir/core/status.md#TBL-core-028`、EX-core-261、`docs/ir/core/list.md#TBL-core-026`、`#REQ-core-155`
- 用語: `docs/ir/core/CONTEXT.md` の「後回し」「後回しのシナリオ」「文書が扱う範囲」

要求とシナリオの本文は `kotowari query <ID>` で読む（`CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- query <ID>`）。変えた既存の項目の差分は `git show bce9067 -- docs/ir` で見られる。

## Approach and why

- 読み取りはスキーマで宣言する（REQ-core-169、REQ-core-170）。`.kotowari/schemas/ir.yaml` の要求の fields に "deferred"（コンマ区切り、行番号を取る）を、document の preamble に fields の "deferred" を足す。どちらにも "required: false" を付け、"repeat" は付けない。エンジンは "repeat" も "required: false" も無い欄を必須（1回ちょうど）として扱うので、付け忘れると宣言の無いすべての要求と文書に写し先の無い指摘が出て check が停止する。"required: false" なら上限が1回になり、2つ目の行は duplicate_field に写り、抽出は1つ目の値だけを残す。preamble の fields は kotowari-markdown-schema が既に扱う（`crates/kotowari-markdown-schema/src/extract.rs` の preamble の処理）ので、そのクレートは変えない。用語集と問題の記録のスキーマ（`context.yaml`、`flags.yaml`）は変えないので、そこでは今までどおり unknown_field になる。preamble の statement は "repeat: { min: 1 }" のままにし、宣言の行だけの文書が missing_scope になることを確かめる。
- 「後回しか」と「後回しのシナリオか」の判定は1か所に置き、check のテストの無さの検査（REQ-core-085、REQ-core-137）、2つの注意、status、list が同じ判定を使う。今の `crates/kotowari-core/src/tests_discovery.rs` の `TestCoverage` と `ScenarioCoverage` が check と status で同じ判定を共有しているのと同じ形にする。同じ ID が2か所以上にあるときは REQ-core-032 の1つ目で決める。
- 文書単位の宣言（出典と行）は文書の側に持つ。要求の無い文書の宣言も出典の検査と duplicate_field を受け（REQ-core-209）、S5 の指紋も文書単位の1つ目の行を使うため。
- "- deferred:" の値の出典の検査は "- source:" の検査（`crates/kotowari-core/src/sources.rs` の check_source）を使い、source_invalid の行は "- deferred:" の行にする（REQ-core-115）。値が空のときの missing_source は、"- source:" の空の値を検査している `crates/kotowari-core/src/ir.rs` の既存の処理（行は見出しの行、detail は ID）とは別に、行を "- deferred:" の行、detail を "deferred" にして出す（REQ-core-210）。
- depends_on_deferred の参照は、`crates/kotowari-core/src/ir.rs` の既存の参照の取り出し（行ごとの参照を持つ）を使ってよいが、参照元を要求、性質、シナリオに限る。問題の記録の "- related:" と決定表からの参照には出さない（REQ-core-212）。
- 注意の種類を足すと、既存のテストが2か所で落ちる。`agent/skills/kotowari/references/findings.md` の Kind の表と本体の種類の集合の一致（REQ-core-125、`tests/step7_skill_references.rs`）と、注意の種類を5つに固定している REQ-core-031 のテスト（`tests/step5_findings.rs`）である。どちらもこの計画の変更による失敗なので、S3 で表とテストを7つに合わせて直す。
- 仕様の変更で `docs/guides/` に guide_stale の注意が出ている（コミット bce9067 の時点で33件）。実装が決まった後で、ガイドの書き方（`agent/skills/kotowari/references/guides.md` の見直しの手順）で節を読み直し、指紋を書き写す。ガイドに後回しの説明（"- deferred:" の書き方、status の deferred、2つの注意）を足すのは仕様の要求ではなく、この計画の判断である。ガイドは利用者向けの使い方の文書で、新しい書き方が載っていないと利用者が使えないため。
- 受け入れの目安（決定 A14）は、kakoi のリポジトリの使い捨ての写しで測る。kakoi は、このリポジトリの主の checkout と同じ親ディレクトリにある。実装はワークツリーで行うので、場所は `"$(git rev-parse --path-format=absolute --git-common-dir)/../../kakoi"` で求める。kakoi のファイルは変えない。

## Scope of change

- `.kotowari/schemas/ir.yaml`
- `crates/kotowari-core/src/` のうち ir.rs、lib.rs、tests_discovery.rs、sources.rs、finding_map.rs、status.rs、list.rs、query.rs、fingerprint.rs、guides.rs と、判定を置く新しいモジュール（置くなら）
- `tests/` の既存のテストのファイル（step2_ir.rs、step3_sources_terms.rs、step4_test_discovery.rs、step5_findings.rs、step7_skill_references.rs、step10_list.rs、step11_query.rs、step12_status.rs、step15_guides.rs のうち該当するもの）と、後回しのテストを置く新しいファイル（置くなら）
- `agent/skills/` の下の kotowari、kotowari-plan、kotowari-cycle、kotowari-implement、kotowari-review、kotowari-brainstorm の SKILL.md と references
- `docs/guides/` の guide_stale が出ている節と、後回しの説明を足す節

## Step order and prerequisites

S1（読み取り）が先で、S2 から S5 はその結果を使う。S2（判定とテストの無さ）の判定を S3、S4 が使う。S5（指紋）は S1 だけに依存する。S6（スキル）は S3 の注意の種類の名前が決まった後。S7（ガイド）は実装の振る舞いを書くので S1〜S6 の後。S8 が全体を確かめる。

## Verification map

| Step | Requirements | Examples |
|---|---|---|
| S1 | REQ-core-208、REQ-core-209、REQ-core-210、REQ-core-115、REQ-core-170、REQ-core-046、TBL-core-011、TBL-core-008、TBL-core-019 | EX-core-386、EX-core-387、EX-core-388、EX-core-390、EX-core-391 |
| S2 | REQ-core-085、REQ-core-137 | EX-core-384、EX-core-385、EX-core-389 |
| S3 | REQ-core-211、REQ-core-212、REQ-core-031、TBL-core-009、TBL-core-019 | EX-core-392、EX-core-393、EX-core-394、EX-core-395、EX-core-396、EX-core-397 |
| S4 | TBL-core-028、TBL-core-026、REQ-core-155、REQ-core-161、TBL-core-027 | EX-core-261、EX-core-398、EX-core-399 |
| S5 | REQ-core-203 | なし |
| S6 | REQ-core-213 | なし |
| S7 | なし（guide_stale を0件にする。REQ-core-204 が出す注意） | なし |
| S8 | この表のすべて | この表のすべて |

## Left to the implementer

- 関数、型、モジュール、テストの名前と、後回しの判定を置く場所（文書単位の宣言を文書の側に持つことは Approach で決めた）
- テストを既存のファイルに足すか、新しいファイルに置くか

## Stop conditions

- kotowari-markdown-schema の preamble の fields が、行番号の取り出し、重複の検出、statement との共存のどれかで仕様どおりに動かず、そのクレートを変える必要がある（そのクレートの仕様 `docs/ir/schema/` の変更になるので、壁打ちに戻す）
- 仕様が決めていない振る舞いに当たった（仕様の沈黙は実装者が決めてよいという意味ではない）
- 既存のテストが、この計画と関係の無い理由で落ちる

## Test command

```sh
CARGO_BUILD_JOBS=4 cargo test --workspace
```

## Out of scope

- mutants の変更（決定 A7。後回しを見ない）
- 決定表とプロパティの後回し（決定 R1）
- kakoi のリポジトリの文書の書き換え（決定 A14。kakoi 側の作業）
- 版の上げ方（決定 U1。リリースのときに決める）

## Steps

### S1: "- deferred:" の行を読み、出典として検査する

- Purpose: 要求ごとと文書単位の "- deferred:" の行を読んで要求の後回しの状態を持ち、値を出典として検査し、宣言の行を範囲の行に数えない
- Specification: `docs/ir/core/deferred.md#REQ-core-208`, `docs/ir/core/deferred.md#REQ-core-209`, `docs/ir/core/deferred.md#REQ-core-210`, `docs/ir/core/sources.md#REQ-core-115`, `docs/ir/core/form-contract.md#REQ-core-170`, `docs/ir/core/ir-items.md#REQ-core-046`, `docs/ir/core/ir-items.md#TBL-core-011`, `docs/ir/core/findings.md#TBL-core-008`, `docs/ir/core/finding-order.md#TBL-core-019`
- Prerequisites: none
- May change: `.kotowari/schemas/ir.yaml`, `crates/kotowari-core/src/ir.rs`, `crates/kotowari-core/src/sources.rs`, `crates/kotowari-core/src/finding_map.rs`, `crates/kotowari-core/src/lib.rs`, `tests/`
- Done when: EX-core-386、EX-core-387、EX-core-388、EX-core-390、EX-core-391 が通り、"- deferred:" の行が要求の下と話題ごとの文書の題名の後で unknown_field にならず、用語集と問題の記録の題名の後では unknown_field のままで、要求ごとと文書単位の宣言が両方あっても指摘が出ず、要求の無い文書の文書単位の宣言も source_invalid と duplicate_field を受け、値の空の宣言にその行で detail が "deferred" の missing_source が出る
- Shown by: test — EX-core-386、EX-core-387、EX-core-388、EX-core-390、EX-core-391 に1つずつ、問題の記録の宣言が unknown_field になることに1つ、両方の宣言で指摘が出ないことに1つ、要求の無い文書の宣言の source_invalid と duplicate_field に1つ、値の空の宣言の missing_source の行と detail に1つ（REQ-core-210 の印）
- Left to the implementer: none
- Stop and hand back if: preamble の fields で重複した "- deferred:" の2つ目の行を duplicate_field にできない、または行番号が取れない

### S2: 後回しの要求と後回しのシナリオをテストの無さの検査から外す

- Purpose: 後回しと後回しのシナリオの判定を1か所に置き、requirement_without_test と scenario_without_test からそれらを外す
- Specification: `docs/ir/core/coverage.md#REQ-core-085`, `docs/ir/core/coverage.md#REQ-core-137`, `docs/ir/core/deferred.md#REQ-core-208`
- Prerequisites: S1
- May change: `crates/kotowari-core/src/tests_discovery.rs`, `crates/kotowari-core/src/lib.rs`, 判定を置く新しいモジュール, `tests/`
- Done when: EX-core-384、EX-core-385、EX-core-389 が通り、後回しと後回しでない要求を両方指すシナリオには今までどおり scenario_without_test が出て、用語「後回しのシナリオ」の定義の境目が仕様どおりである（"- verification:" の行の無い要求は判定から外すので、それと検証のある後回しの要求を指すシナリオは後回しのシナリオになる）
- Shown by: test — EX-core-384、EX-core-385、EX-core-389 に1つずつ、用語「後回しのシナリオ」の定義の境目（review と後回しだけを指す、検証の無い要求と後回しの要求を指す、同じ ID のシナリオが2つある）に1つずつ、同じ ID の要求が2か所にあり1つ目だけが後回しの場合に1つ（REQ-core-208 の印）
- Left to the implementer: none
- Stop and hand back if: 既存の `TestCoverage` の判定を変えないと後回しを外せず、status の with_tests の数が後回しと無関係な入力で変わる

### S3: deferred_with_test と depends_on_deferred の注意を出す

- Purpose: 後回しにテストの印がある場合と、後回しでない項目が後回しの要求を参照する場合に注意を出す
- Specification: `docs/ir/core/deferred.md#REQ-core-211`, `docs/ir/core/deferred.md#REQ-core-212`, `docs/ir/core/findings.md#REQ-core-031`, `docs/ir/core/findings.md#TBL-core-009`, `docs/ir/core/finding-order.md#TBL-core-019`
- Prerequisites: S2
- May change: `crates/kotowari-core/src/lib.rs`, `crates/kotowari-core/src/ir.rs`, `crates/kotowari-core/src/tests_discovery.rs`, 判定を置く新しいモジュール, `agent/skills/kotowari/references/findings.md`, `tests/`
- Done when: EX-core-392〜EX-core-397 が通り、2つの種類の severity が notice で、`tests/step7_skill_references.rs` の種類の一致のテストと `tests/step5_findings.rs` の REQ-core-031 のテストが7つの注意の種類で通る
- Shown by: test — EX-core-392、EX-core-393、EX-core-394、EX-core-395、EX-core-396、EX-core-397 に1つずつ、"- definition:" の行から参照する場合と`性質`の文から参照する場合に1つずつ、問い合わせの無い言語の印でも deferred_with_test が出る場合に1つ、印が2つあっても deferred_with_test が1件の場合に1つ、同じ ID が2か所にあり1つ目の場所に出る場合に1つ、後回しのシナリオへの参照と問題の記録の "- related:" からの参照に出ない場合に1つ
- Left to the implementer: none
- Stop and hand back if: 既存の参照の取り出しが、仕様の参照（"- definition:"、文とステップの中のバッククォートで囲んだ ID、"@about"）と違うものを返し、それを直すと query の referenced_by が変わる

### S4: status と list と query に後回しを出す

- Purpose: status の requirements と scenarios に deferred を足して with_tests と without_tests から後回しを外し、list と query の1件に deferred を出す
- Specification: `docs/ir/core/status.md#TBL-core-028`, `docs/ir/core/list.md#TBL-core-026`, `docs/ir/core/list.md#REQ-core-155`, `docs/ir/core/query.md#REQ-core-161`, `docs/ir/core/query.md#TBL-core-027`
- Prerequisites: S2
- May change: `crates/kotowari-core/src/status.rs`, `crates/kotowari-core/src/list.rs`, `crates/kotowari-core/src/query.rs`, `tests/`
- Done when: EX-core-261（行の末尾が変わった）、EX-core-398、EX-core-399 が通り、query の text の1行目も list と同じく末尾に " deferred" が付く
- Shown by: test — EX-core-261 の既存のテストを直し、EX-core-398 と EX-core-399 に1つずつ、query の json の "deferred" と text の1行目に1つ（TBL-core-027、REQ-core-161 の印）、例の無い後回しの要求が without_examples に数えられることに1つ（TBL-core-028 の印）
- Left to the implementer: none
- Stop and hand back if: status の json の鍵の並びが TBL-core-028 の表の順と一致しない形でしか出せない

### S5: 文書単位の宣言を要求の指紋に入れる

- Purpose: 文書単位の "- deferred:" の行を持つ文書の要求の指紋に、その1つ目の行を並びの先頭に加える
- Specification: `docs/ir/core/guides.md#REQ-core-203`
- Prerequisites: S1
- May change: `crates/kotowari-core/src/fingerprint.rs`, `crates/kotowari-core/src/guides.rs`, `crates/kotowari-core/src/list.rs`, `tests/`
- Done when: 文書単位の宣言を足すと、その文書の要求の list の fingerprint が変わり、決定表とシナリオの fingerprint は変わらず、宣言の無い文書の要求の fingerprint は今までと同じである
- Shown by: test — REQ-core-203 の文書単位の行の扱いに1つ、要求ごとの "- deferred:" の行が body に入って fingerprint が変わることに1つ
- Left to the implementer: none
- Stop and hand back if: 宣言の無い文書の fingerprint が変わる（既存のガイドの印がすべて古くなる）

### S6: スキルの文面に後回しの扱いを書く

- Purpose: kotowari のスキルに "- deferred:" の書き方を、工程のスキルに後回しの扱いを書く
- Specification: `docs/ir/core/deferred.md#REQ-core-213`
- Prerequisites: S3
- May change: `agent/skills/kotowari/SKILL.md`, `agent/skills/kotowari/references/`, `agent/skills/kotowari-plan/SKILL.md`, `agent/skills/kotowari-cycle/SKILL.md`, `agent/skills/kotowari-implement/SKILL.md`, `agent/skills/kotowari-review/SKILL.md`, `agent/skills/kotowari-review/references/`, `agent/skills/kotowari-brainstorm/SKILL.md`
- Done when: REQ-core-213 の how_to_verify に挙げた5つ（ir-form.md の行、findings.md の2つの種類、plan、cycle と review、brainstorm）がそれぞれのスキルに書かれ、kotowari-implement のテスト側の指摘を直す規則にも後回しを含めないことが書かれている（REQ-core-213 の文の「実装のループ」）
- Shown by: artifact — 変えた SKILL.md と references の差分を、REQ-core-213 の how_to_verify と突き合わせて読む
- Left to the implementer: 文面の言い回しと、各スキルの中で書く位置
- Stop and hand back if: スキルの既存の文面が後回しの扱いと矛盾する決まり（たとえば「テスト側の指摘はすべて0件にする」）を持ち、書き足すだけでは食い違いが残る

### S7: ガイドを後回しに追従させる

- Purpose: guide_stale の出ている節を読み直して後回しの説明を足し、指紋を書き写す
- Specification: `docs/ir/core/guides.md#REQ-core-204`
- Prerequisites: S1, S2, S3, S4, S5, S6
- May change: `docs/guides/`
- Done when: `kotowari check` の guide_stale が0件で、`docs/guides/` に "- deferred:" の書き方と、status の deferred の意味と、2つの注意の意味が書かれている（後者は計画の判断。Approach を見る）
- Shown by: check — `CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format json | jq '[.findings[] | select(.kind == "guide_stale")] | length'` が 0
- Left to the implementer: 後回しの説明を置く節（既存の節に足すか新しい節にするか）
- Stop and hand back if: 節を読み直して、ガイドが仕様と違うことを書いていると分かったが、それが後回しと無関係である

### S8: 全体を確かめ、kakoi の写しで受け入れの目安を測る

- Purpose: この計画の項目がすべてテストで確かめられ、kakoi の IR で後回しの宣言が要望どおりに効くことを測る
- Specification: `docs/ir/core/deferred.md#REQ-core-208`, `docs/ir/core/coverage.md#REQ-core-085`, `docs/ir/core/coverage.md#REQ-core-137`
- Prerequisites: S1, S2, S3, S4, S5, S6, S7
- May change: none
- Done when: REQ-core-208〜REQ-core-212 と EX-core-384〜EX-core-399 のすべてで `kotowari query <ID>` の tests が空でなく、S1〜S5 の Shown by に挙げたテストが変えた既存の項目の ID の印を持ち、`kotowari check` がこのブランチで変えたファイルとこの計画の ID に誤りを出さず、kakoi の写しで次の4つが成り立つ: (1) 宣言の前に "docs/ir/network/publish-config/"、"publish-lifetime/"、"publish-target/"、"link-local/" の下の12文書に requirement_without_test と scenario_without_test がそれぞれ1件以上あり、宣言の後にどちらも0件になり、宣言の行に source_invalid が出ない、(2) それ以外の文書のそれらの件数は宣言の前後で変わらない、(3) 12文書のどれかの要求から "- verification:" の行を消すと verification_missing の誤りが出る、(4) 12文書のどれかの要求の ID を別の文書の要求にも書くと duplicate_id の誤りが出る
- Shown by: external — `CARGO_BUILD_JOBS=4 cargo test --workspace` が通ることを見る。次に Approach の式で求めた kakoi の場所で `git -C <kakoi> archive HEAD` を一時ディレクトリに展開し（kakoi の作業ツリーは変えない）、このブランチでビルドした kotowari をその中で `check --format json` にかけて (1) と (2) の件数を jq で数え、12文書の題名の後に kakoi の判断の記録の後回しを決めた決定を出典にした "- deferred:" の行を足してもう一度数え、(3) と (4) の変更を順に入れて誤りを見る。後回しを決めた決定が kakoi の記録に見つからなければ、写しの中の、kakoi の設定（".kotowari/config.yaml"）の "decisions.records" が指す置き場の下に仮の判断の記録を置いてそれを出典にする。数えた件数と使った出典を報告に書く
- Left to the implementer: none
- Stop and hand back if: kakoi の写しで (1) が0件にならず、その原因が kakoi の文書の書き方（たとえば "@about" が後回しでない要求も指す）ではなく実装にある
