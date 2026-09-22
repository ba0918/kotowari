# 実装計画: IR の読み取りをスキーマのエンジンに置き換える

## Goal

kotowari が `IR` の Markdown の形を自前で読むのをやめ、同じ workspace の `kotowari-markdown-schema`（以下エンジン）が埋め込みのスキーマで読んだ結果を写すようになる。`kotowari check` の出力は、仕様が許す 16 個の差分の外で変わらない。

## Specification

IR の置き場は `docs/ir/`。この計画が受け持つ要求は次のとおり。

エンジン側（`docs/ir/schema/`）:

- `docs/ir/schema/closed-world.md#REQ-schema-055`

kotowari 側（`docs/ir/core/`）:

- `docs/ir/core/form-contract.md#REQ-core-168`、`#REQ-core-169`、`#REQ-core-170`、`#REQ-core-173`、`#REQ-core-177`
- `docs/ir/core/finding-map.md#REQ-core-171`、`#REQ-core-172`
- `docs/ir/core/findings.md#REQ-core-174`
- `docs/ir/core/cli-environment.md#REQ-core-175`、`#REQ-core-176`

決定表は `docs/ir/core/finding-map.md#TBL-core-029` と `#TBL-core-030`、`docs/ir/core/cli-environment.md#TBL-core-018` と `#TBL-core-020`。

判断の記録は `docs/decision/records/2026-09-22-ir-engine.md`（置き換えの判断、A1〜A81）と `docs/decision/records/2026-09-21-mds-spec.md`（エンジンの仕様、A1〜A69）。

本文は `kotowari query <ID>` で読む。文書を開いて探さない。

**着手の前提。** `問題の記録`は両方の置き場で0件で、この計画が待つ人の判断は無い。仕様が黙っている場面に出会ったら、各ステップの Stop and hand back のとおり壁打ちへ戻す。

## Approach and why

**エンジンへの機能追加を先に終わらせる。** `REQ-schema-055` は、宣言していない行の`指摘`にその行をどの規則種別として読んだかを持たせる。これが無いと kotowari は `undeclared_line` を `unknown_field` と `unknown_line` に写し分けられない（`TBL-core-030` の2行）。kotowari 側の工程がこれを待つので、独立したステップとして先に置く。

**写す層を、自前の読み取りを落とす前に作る。** `TBL-core-030` の 39 行を写す関数を先に書き、既存の読み取りと並べて同じ入力で同じ`指摘`が出ることを確かめてから、自前の読み取りを落とす。落としてから写す順にすると、どのステップでも `kotowari check` が通らない時間が生まれる。

**`docs/ir/schema/` の frontmatter は最後に外す。** 今この宣言は `mds check ./` がエンジン側の IR を検査する唯一の手がかりで、外すと検査が効かなくなる。kotowari が文書の種類でスキーマを選べるようになってから外す。

**エンジン側の具体例のテストは独立している。** `EX-schema-018` から `EX-schema-030` の 13 件は、抽出の宣言を作り直した工程で書かれてテストが付かなかったもの。置き換えとは関係なく閉じられるので、待ち時間の無いステップとして置く。

**順序の理由。** ステップ1がエンジンの土台。ステップ2は誰も待たない。ステップ3（埋め込み）とステップ4（写す層）は kotowari 側の土台。ステップ5はその上、ステップ6はステップ5の後、ステップ7はステップ6の後。ステップ8はステップ5の後で、ステップ7と並べてよい。ステップ9が最後。

## Scope of change

変えてよい場所:

- `crates/kotowari-markdown-schema/src/`（`REQ-schema-055` の実装とその `#[cfg(test)]`）
- `crates/kotowari-markdown-schema/tests/`（`EX-schema-018`〜`030` の印）
- `crates/kotowari-core/src/`（埋め込み、写す層、自前の読み取りの削除）
- `tests/`（根の統合テスト。`crates/kotowari-core/` に `tests/` は無く、単体テストは `src/` の `#[cfg(test)]` に置く）
- `docs/ir/core/form-contract.md`、`docs/ir/core/ir-document.md`、`docs/ir/core/cli-environment.md`（`REQ-core-089`、`REQ-core-041`、`REQ-core-120` の確かめ方だけ）
- `docs/ir/schema/` の各文書の frontmatter（ステップ7）
- `skills/kotowari/references/findings.md` と `skills/kotowari/references/ir-form.md`（ステップ8）
- `docs/ir/core/FLAGS.md`（人の判断で閉じた問題の記録を外す）
- `.mds/schemas/` の3つの YAML（`REQ-core-170` と `A77` が要る宣言は着手前に足してあるが、足りないものが見つかったら直してよい）

触らない場所:

- `docs/ir/` の要求・決定表・性質・具体例の本文（確かめ方の3件を除く）
- `docs/decision/records/`（新しい決定を足さない。足したくなったら壁打ちに戻す）

## Step order and prerequisites

ステップ1がステップ4の前提。ステップ2は前提を持たず、いつ走らせてもよい。ステップ3とステップ4は互いに独立で、どちらもステップ5の前提。ステップ6はステップ4とステップ5の後。ステップ7はステップ3とステップ6の後。ステップ8はステップ5の後で、ステップ7と並べてよい。ステップ9はステップ1〜8の後。各ステップの Prerequisites はこの並びと同じものを書いている。

## Step 1 — 宣言していない行の指摘に規則種別を持たせる

Purpose: kotowari が `undeclared_line` を2つの写し先に分けられるようにする。
Specification: `docs/ir/schema/closed-world.md#REQ-schema-055`。判断の記録は `docs/decision/records/2026-09-22-ir-engine.md#A74`。
Prerequisites: なし。
May change: `crates/kotowari-markdown-schema/src/finding.rs`、`crates/kotowari-markdown-schema/src/validate.rs`、それぞれの `#[cfg(test)]`、`crates/kotowari-markdown-schema/tests/cli.rs`、`crates/kotowari-markdown-schema/README.md`。

Done when:

- `undeclared_line` の`指摘`が、その行を`フィールド行`、`箇条書き`、順序付きリスト、`文`、`表`、`コードブロック`のどれとして読んだかを持つ
- ほかの種類の`指摘`の形は変わらない
- CLI の JSON がこの値を出す
- `validate(schema, document, open) -> Vec<Finding>` の署名が変わらない

Shown by: test — RED → GREEN → REFACTOR。`crates/kotowari-markdown-schema/src/validate.rs` の `#[cfg(test)]` に、宣言していない `- 名前: 値` の行が`フィールド行`として、宣言していない箇条書きの行が`箇条書き`として、宣言していない表が`表`として返ることを見る3本。`crates/kotowari-markdown-schema/tests/cli.rs` に JSON がこの値を持つことを見る1本。

Left to the implementer: 値の型（専用の列挙か文字列か）。

Stop and hand back if: 宣言していない行を読んだ時点でどの`規則種別`にも決まらない場合があると分かったとき。JSON の鍵の名前は`指摘`の要素を1つ増やす決定なので、`REQ-schema-008` の書き方から一意に決まらないと分かったとき。

## Step 2 — エンジン側の具体例 13 件にテストの印を付ける

Purpose: 抽出の宣言を作り直した工程で書かれた具体例に、対応するテストの印を付けて `scenario_without_test` を消す。
Specification: `docs/ir/schema/extraction.md` の `EX-schema-018`〜`EX-schema-025`、`docs/ir/schema/cli.md` の `EX-schema-026`〜`EX-schema-030`。各具体例の本文は `kotowari query <ID>` で読む。
Prerequisites: なし。
May change: `crates/kotowari-markdown-schema/src/` と `tests/` の `#[cfg(test)]` とテスト関数の印。テストが無い具体例には新しくテストを書く。

Done when:

- 13 件すべてに `@kotowari[EX-schema-nnn]` の印を持つテストがある
- 印の付いたテストが、その具体例の Given・When・Then をそのまま確かめている
- 既存のテストに印を足すだけで済むものは新しく書かない

Shown by: check — `CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format json` の `scenario_without_test` のうち `EX-schema-` で始まるものが0件。

Left to the implementer: 既存のテストに印を足すか新しく書くかの判断。テストを置くモジュール。

Stop and hand back if: 具体例の Given が今の実装では作れない入力を述べていると分かったとき。

## Step 3 — スキーマをバイナリに埋め込み、文書の種類で選ぶ

Purpose: kotowari が実行時にスキーマのファイルを読まずに、文書の種類に応じた形の宣言を持つ。
Specification: `docs/ir/core/form-contract.md#REQ-core-168`。判断の記録は `#A8`、`#A36`。
Prerequisites: なし。
May change: `crates/kotowari-core/src/`（新しいモジュールを足してよい。単体テストは同じファイルの `#[cfg(test)]`）、`crates/kotowari-core/Cargo.toml`（`kotowari-markdown-schema` への path 依存を足す。根の `Cargo.toml` に `[workspace.dependencies]` は無いので、依存はクレートの manifest に直接書く）。

Done when:

- 3つのスキーマ（`ir`、`ir-context`、`ir-flags`）がコンパイル時に取り込まれている
- `用語集`、`問題の記録`、`話題ごとの文書`の別に応じて取り込んだスキーマを選ぶ関数がある
- 実行時にスキーマのファイルを読む経路が無い
- この時点では検査の結果が変わらない（土台だけ）

Shown by: test — RED → GREEN → REFACTOR。取り込んだ3つのスキーマがエンジンの `parse_schema` を通ることを見る1本、文書の種類ごとに選ぶスキーマの名前が合うことを見る1本。`REQ-core-168` の印をこのテストに付ける。`EX-core-264`（`IR`のどの文書にも frontmatter が無い）の印は、ステップ7で frontmatter を外したあとに付ける。

Left to the implementer: 取り込み方（`include_str!` かビルドスクリプトか）。選ぶ関数の置き場。

Stop and hand back if: 取り込んだスキーマが `parse_schema` を通らないと分かったとき。取り込みはコンパイル時なので、実行時に `parse_schema` が失敗する経路をどう扱うかは仕様が黙っている。`REQ-core-175` は「スキーマを読めないことを理由とする`停止`は持たない」と定めるので、ここを実装役が決めない。

Note: `crates/kotowari-markdown-schema` は `rust-version = "1.89"` を宣言し、`crates/kotowari-core` は依存の都合で 1.90 を要る。依存を足しても、ステップ9の 7 は `-p` でこのクレートだけを検査するので通る。workspace 全体で最低の Rust をそろえようとしない。

## Step 4 — エンジンの指摘を写す層を作る

Purpose: エンジンが返した`指摘`を kotowari の`指摘`へ写す。既存の読み取りはまだ落とさない。
Specification: `docs/ir/core/finding-map.md#REQ-core-171`、`#REQ-core-172`、`#TBL-core-029`、`#TBL-core-030`、`docs/ir/core/cli-environment.md#REQ-core-175`、`#TBL-core-018`、`#TBL-core-020`。
Prerequisites: ステップ1。
May change: `crates/kotowari-core/src/`（新しいモジュール）、`tests/`。

Done when:

- `TBL-core-030` の 39 行を写す関数がある。行に無い種類と「発生しない」の行の種類を受けたときは`停止`する
- `"line"` の扱いが3つとも実装されている（そのまま使う、null にする、`項目`の見出しの行に付け直す）
- detail の材料4つがすべて使える。`抽出`の`項目`は`指摘`の行で突き合わせる
- 新しい`停止`の理由の文言が `mapping error` で、詳細が `TBL-core-020` のとおり
- 既存の自前の読み取りはまだ動いており、`kotowari check` の出力は変わらない

Shown by: test — RED → GREEN → REFACTOR。`TBL-core-030` の 39 行のうち、写し先を持つ 25 行それぞれについて、エンジンの`指摘`を1件与えて写し先の種類・`"line"`・detail が合うことを見るテスト。「発生しない」と書いた 14 行の種類を与えると`停止`することを見るテスト（`EX-core-265`。エンジンの`指摘`の種類は 24 個で閉じているので、表に行の無い種類は作れない。「発生しない」の行で確かめる）。写せない値で`停止`することを見る1本（`EX-core-268`）。

Left to the implementer: 写す関数の分け方。`抽出`の`項目`を行で引くための持ち方。

Stop and hand back if: `TBL-core-030` に行の無いエンジンの種類が見つかったとき。`抽出`の`項目`の行と`指摘`の行が突き合わないものがあると分かったとき。

## Step 5 — 新しい3種類の指摘を出す

Purpose: 置き換えで増える`指摘` 3 種類を kotowari が出す。
Specification: `docs/ir/core/findings.md#REQ-core-174`、`#TBL-core-008`、`docs/ir/core/finding-order.md#TBL-core-019`。具体例は `EX-core-266`、`EX-core-267`。
Prerequisites: ステップ3、ステップ4。
May change: `crates/kotowari-core/src/`、`tests/`。

Done when:

- `unknown_line`、`unknown_code_block`、`glossary_title_invalid` の3種類が出る
- `unknown_line` は、`"## "` の見出しの直下の宣言の外の行に加えて、スキーマが宣言していない表とコードブロックにも出る
- detail と `"line"` が `TBL-core-008` と `TBL-core-019` のとおり
- 3種類とも`誤り`で、終了コードを1にする

Shown by: test — RED → GREEN → REFACTOR。3種類それぞれを出す入力で種類・detail・`"line"` が合うことを見る3本と、宣言の外の表とコードブロックに `unknown_line` が出ることを見る1本。`EX-core-266` と `EX-core-267` に印を付ける。

Left to the implementer: なし。

Stop and hand back if: 3種類のどれかが `TBL-core-030` の写しだけでは出せないと分かったとき。

## Step 6 — 自前の読み取りを落とす

Purpose: kotowari が `IR` の Markdown の構造を自前で読まなくなる。
Specification: `docs/ir/core/form-contract.md#REQ-core-169`、`#REQ-core-170`、`#REQ-core-173`、`docs/ir/core/cli-environment.md#REQ-core-176`。
Prerequisites: ステップ4、ステップ5。
May change: `crates/kotowari-core/src/ir.rs`（大きく減る）、`crates/kotowari-core/src/` のほか、`tests/`、`docs/ir/core/form-contract.md`・`docs/ir/core/ir-document.md`・`docs/ir/core/cli-environment.md` の確かめ方3件。

Done when:

- `crates/kotowari-core/src/ir.rs` に生の行を読む関数が、gherkin の塊の中身と閉じない`コードブロック`の検出の2つに対応するものだけ残る
- 見出し、`"- 名前:"` の行、Markdown の表、`題名`と`文書が扱う範囲`を読む関数が無い
- 文書をまたぐ検査、上限、`出典`の検査、`文`に基づく検査、`文書名の参照`、gherkin の中身、閉じない`コードブロック`、`用語集`の行の`用語`のセルが空であることの検査は kotowari に残る
- `用語`のセルが空であることの検査は`抽出`の`用語集`の行から見る。エンジンは列の数しか見ないので、この判定はエンジンへ移さない
- 形の`指摘`が出た文書も、文書をまたぐ検査を受ける（`EX-core-269`）
- `REQ-core-089`、`REQ-core-041`、`REQ-core-120` の確かめ方が、残らない関数の名前を指していない
- `REQ-core-045`、`REQ-core-047`、`REQ-core-098` の印の付いたテストが、置き換え後の振る舞い（3本以上でも1件、`問題の記録`の`項目`も`文`が要る、値が空の行は値の誤り）を確かめている

Shown by: test — RED → GREEN → REFACTOR。`EX-core-269` に印を付ける。既存の形の`指摘`のテストがすべて緑のまま。確かめ方3件は review の要求なので、書き換えた文が `kotowari check` を通ることで見る。

Left to the implementer: `crates/kotowari-core/src/ir.rs` の残し方。確かめ方の言い回し。

Stop and hand back if: 残す検査のどれかが、エンジンの返す`抽出`だけでは書けないと分かったとき。

## Step 7 — docs/ir/schema の frontmatter を外す

Purpose: `IR` の文書がスキーマを宣言しない状態にする。
Specification: `docs/ir/core/form-contract.md#REQ-core-168`。
Prerequisites: ステップ3、ステップ6。
May change: `docs/ir/schema/` の各文書の先頭の frontmatter。

Done when:

- `docs/ir/schema/` のどの文書も先頭に `$schema` の宣言を持たない
- `kotowari check` の結果が外す前と変わらない（kotowari は文書の種類でスキーマを選ぶ）
- `EX-core-264` に印が付いている

Shown by: check と test — `CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format json` の出力が、外す前と1件も変わらない。`EX-core-264` の印の付いたテストが緑。

Left to the implementer: なし。

Stop and hand back if: 外すと `mds check ./` が使えなくなることが問題だと分かったとき（エンジン側の IR を検査する手段が kotowari だけになる。これは想定どおりだが、実際に困るなら止める）。

## Step 8 — スキルの references を新しい3種類に合わせる

Purpose: スキルが配る`指摘`の一覧が、コードが出す種類の集合と一致する。
Specification: `docs/ir/core/skill-references.md#REQ-core-125`。
Prerequisites: ステップ5。
May change: `skills/kotowari/references/findings.md`、`skills/kotowari/references/ir-form.md`。

Done when:

- `skills/kotowari/references/findings.md` の表に `unknown_line`、`unknown_code_block`、`glossary_title_invalid` の行がある
- `skills/kotowari/references/ir-form.md` の`除外`の列挙から、`"## "` の見出しの直下で最初の `"### "` より前の行が消える
- `REQ-core-125` の一致のテストが通る

Shown by: test — `REQ-core-125` の既存のテストが緑。

Left to the implementer: 行の文の言い回し。

Stop and hand back if: `docs/decision/records/ir-form.md` の`除外`の節も直さないと一致しないと分かったとき（判断の記録なので、直すには壁打ちに戻る）。

## Step 9 — 終端の確認

Purpose: 置き換えが揃ったことと、仕様が許す範囲の外で振る舞いが変わっていないことを見る。
Specification: `docs/ir/core/form-contract.md#REQ-core-177`。
Prerequisites: ステップ1〜8。
May change: なし（直しが要れば当該のステップに戻る）。

Done when: 下の Shown by のすべてが通る。

Shown by: check — 次の順に走らせる。

1. `CARGO_BUILD_JOBS=4 cargo test --workspace` が全件通る
2. `CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format json` の終了コードが0で、`findings` が空
3. `CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- status --format text` の最後の行が `complete true` で、終了コードが0。`complete true` は`問題の記録`が0件であることも求めるので、着手の前提に挙げた3件が閉じていること
4. 置き換えの前に保存した `kotowari check --format json` の出力と `diff` を取り、差分が `REQ-core-177` の 16 個の範囲に収まる。このリポジトリの `IR` では差分が1件も出ない
5. `CARGO_BUILD_JOBS=4 cargo run -q -p kotowari-markdown-schema --bin mds -- check ./` の終了コードが0（ステップ7で frontmatter を外したあとは対象が0件になる）
6. `CARGO_BUILD_JOBS=4 cargo clippy --workspace --all-targets -- -D warnings` の誤りが基準線から増えていない。基準線は着手前に実測してあり、`crates/kotowari-core/` の 21 件（`error` で始まる行は、まとめの2行を含めて 23 行）、`crates/kotowari-markdown-schema/` は 0 件。ステップ6で自前の読み取りが減るぶん、この数は減る見込みで、減るのは構わない
7. `CARGO_BUILD_JOBS=4 cargo +1.89.0 check -p kotowari-markdown-schema --all-targets` が通る

Left to the implementer: なし。

Stop and hand back if: 4 の差分が 16 個の範囲に収まらないとき。

## Verification map

| 要求 | 確かめるステップ |
|---|---|
| REQ-schema-055 | ステップ1 |
| REQ-core-168 | ステップ3、ステップ7 |
| REQ-core-169 | ステップ6 |
| REQ-core-170 | ステップ4、ステップ6 |
| REQ-core-171 | ステップ4 |
| REQ-core-172 | ステップ4 |
| REQ-core-173 | ステップ6 |
| REQ-core-174 | ステップ5 |
| REQ-core-175 | ステップ4 |
| REQ-core-176 | ステップ6 |
| REQ-core-177 | ステップ9 |
| EX-schema-018〜030 | ステップ2 |
| REQ-core-089、REQ-core-041、REQ-core-120（確かめ方の書き換え） | ステップ6 |
| REQ-core-045、REQ-core-047、REQ-core-098（置き換え後の振る舞いへ） | ステップ6 |
| EX-core-264 | ステップ7 |
| EX-core-265、EX-core-268 | ステップ4 |
| EX-core-266、EX-core-267 | ステップ5 |
| EX-core-269 | ステップ6 |

## Stop conditions

- 仕様が黙っている振る舞いを決めないといけなくなったとき（`指摘`の新しい種類、`停止`の新しい理由、写し先の無いエンジンの種類）は壁打ちに戻す
- `TBL-core-030` の行と実物のエンジンの種類に過不足が見つかったとき
- `REQ-core-177` の 16 個の差分に収まらない振る舞いの差が見つかったとき
- エンジンの契約の入口6つのどれかの署名を変えないと進めないと分かったとき

## Test command

`CARGO_BUILD_JOBS=4 cargo test --workspace`。エンジンだけなら `CARGO_BUILD_JOBS=4 cargo test -p kotowari-markdown-schema`。`CARGO_BUILD_JOBS=4` はメモリの都合で、`lefthook.yml` の pre-commit と pre-push が同じ形で走らせている。

## Out of scope

- CLI のバイナリの名前の変更
- 2つの製品の版とタグの持ち方
- kotowari の版を上げること
