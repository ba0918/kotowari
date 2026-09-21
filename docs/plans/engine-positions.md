# 実装計画: エンジンの抽出と指摘に位置と文脈を足す

## Goal

`kotowari-markdown-schema` が、表と文を行ごとに`抽出`し、`指摘`に行番号とノードの名前と生の行を持たせるようになる。kotowari 側の置き換えはこの計画では行わない。

## Specification

`docs/spec/ir-engine.md`。この計画が受け持つのは R1〜R5 の5件。R6〜R15 は別の計画。判断の記録は `docs/decision/records/2026-09-22-ir-engine.md`（工程を2つに分ける理由は A14、5件を1つの工程にまとめる理由は A32）。

## Approach and why

**生の行は `Document` に持たせる。** `validate(schema, document, open)` は `src` を受けない（`src/validate.rs:12`）。これは契約の入口6つの1つ（`docs/ir/schema/library.md#TBL-schema-010`）なので署名を変えない。`Document::parse(src)` は `src` を持っているので、パースの時点で `Heading` と `Block` に生の行と行ごとの位置を持たせ、`validate` はそれを読む。`Document` の中のフィールドは契約の外（実装の都合）なので変えてよい。

**生の行を再構成では作れない。** 見出しの文字は `inline_text`（`src/document.rs:531`）が組み立てており、`InlineCode` のバッククォートと `Link` の URL を落とす。`ID` と名前をつないでも元の行と一致しない。段落の生テキストも `slice_at` が `trim()` する（`:570`）。1行目の字下げを取るには `original_item_line(..., preserve_indent=true)` の行頭遡り（`:384`）と同じ手が要る。

**`Document` の変更を独立したステップにする。** `Block::Statement` と `Block::Table` に要素を足すと、`src/validate.rs:524` と `:537` の `..` の無い網羅パターンがコンパイルできず、`rows` の型を変えると `validate_table_shape`（`:690`）の引数も壊れる。この追従を抽出の作業に混ぜると、どのステップでもビルドが通らない時間が生まれる。土台を先に置く。

**IR を先に改める。** `docs/ir/schema/` の要求は既にあるものを改める形になる。`docs/ir/schema/extraction.md` は要求が既に 10 件で上限ちょうどなので、足すと `too_many_requirements` の注意が出る。

**順序の理由。** ステップ1（IR）は規律上の順序で、技術的な前提ではない（先に実装しても動く）。ステップ2が土台で、ステップ3〜5 はこれに乗る。ステップ5を最後に置くのは、生の行の置き場がステップ2で決まってからの方が 26 か所の直しが一度で済むからである。

## Scope of change

変えてよい場所:

- `docs/ir/schema/extraction.md`、`docs/ir/schema/cli.md`、`docs/ir/schema/CONTEXT.md`（要求・決定表・具体例・用語）
- `crates/kotowari-markdown-schema/src/document.rs`（`Heading` と `Block` の形、組み立て）
- `crates/kotowari-markdown-schema/src/extract.rs`（`抽出`の組み立て）
- `crates/kotowari-markdown-schema/src/finding.rs`（`Finding` の形）
- `crates/kotowari-markdown-schema/src/validate.rs`（`指摘`の組み立て、パターンと関数の引数の追従）
- `crates/kotowari-markdown-schema/src/schema.rs`（鍵と列の名前の衝突の検査）
- `crates/kotowari-markdown-schema/src/main.rs`（CLI の出力）
- `crates/kotowari-markdown-schema/README.md`、`crates/kotowari-markdown-schema/docs/spec/mds.md`（`表`と`of: line` の説明が古くなる）
- 上の各ファイルの `#[cfg(test)]`、`crates/kotowari-markdown-schema/tests/`

触らない場所:

- `crates/kotowari-core/`、`src/`、`tests/`（kotowari 側は別の計画）
- `.mds/schemas/`（スキーマの宣言を変えるのは kotowari 側の計画）
- `docs/ir/core/`
- `crates/kotowari-markdown-schema/CHANGELOG.md`（公開済みの v0.1.0 の記録。版付けは保留）

## Step order and prerequisites

ステップ1は規律上の順序（実装より先に IR を改める）で、技術的な前提ではない。ステップ2が土台で、ステップ3と4は互いに独立。ステップ5はステップ2の後。ステップ6が最後。

## Step 1 — IR と用語集を改める

Purpose: `抽出`の形と`指摘`の形の要求を R1〜R5 の振る舞いに合わせ、新しく使う語を用語集に足す。Specification: `docs/spec/ir-engine.md#R1`、`#R2`、`#R3`、`#R4`、`#R5`、`#用語`。
Prerequisites: なし（規律上ステップ2より先に置く）。始める前に `CARGO_BUILD_JOBS=4 cargo test -p kotowari-markdown-schema` を1回走らせて全件通ることと、`CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format text` の指摘が0件であることを記録する。
May change: `docs/ir/schema/extraction.md`、`docs/ir/schema/cli.md`、`docs/ir/schema/CONTEXT.md`。
Done when:
- `TBL-schema-008` の`表`の行が「`導かれる値`を宣言したときは各行のオブジェクトがその鍵を持ち、繰り返す`表`は表ごとの配列になる。宣言しないときは今と同じオブジェクトの配列」になる。`文`の行も同じ条件付きで、宣言したときだけ行ごとのオブジェクトの並びになることを述べる
- `REQ-schema-048` が、`導かれる値`を`項目`・`表`・`文`で要素ごとのオブジェクトの中の相対パスとして解くことを述べる
- `REQ-schema-008` が`指摘`の要素にノードの名前と生の行を足して7つにし、行番号の決め方（違反したノードの開始行、欠落ならそれを含むノードの開始行、含むノードに行が無ければ省く）を述べる。「行を持たない`指摘`では行番号を省く」の既存の規則は残す
- `docs/ir/schema/CONTEXT.md` に `導かれる値`、`ノードの名前`、`生の行` の3語がある。出典は `docs/decision/records/2026-09-22-ir-engine.md` の該当する決定
- 既存の要求の ID を変えない。要求を足さない

Shown by: check — 次の順。
1. `CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format text` の指摘が0件のまま
2. `grep -c '^### REQ-' docs/ir/schema/extraction.md` が 10、`docs/ir/schema/cli.md` が 9
3. `wc -l` がどちらも 200 未満

Left to the implementer: 文の言い回し。具体例（`EX-schema-018` 以降が空き）を足すかどうかと、足すならどの要求に付けるか。
Stop and hand back if: 3語を用語集に足すと既存の IR の文の中のどれかが `duplicate_term` になるとき。要求を足さずには書けないと分かったとき。`TBL-schema-008` の`箇条書き`か`コードブロック`の行も変えないと書けないと分かったとき（仕様は A12 でこの2つに広げないと決めている）。

## Step 2 — Document に生の行と行ごとの位置を持たせる

Purpose: `指摘`と`抽出`が生の行と行ごとの行番号を使えるようにする土台を置く。Specification: `docs/spec/ir-engine.md#R1`、`#R2`、`#R5`。
Prerequisites: なし（ステップ1は規律上の順序）。
May change: `crates/kotowari-markdown-schema/src/document.rs`、`src/validate.rs`（網羅パターンと `validate_table_shape` の引数の追従だけ）、それぞれの `#[cfg(test)]`。
Done when:
- `Heading` が見出しの生の行を持つ。`### REQ-BAD: `名前`` のようにインラインコードを含む見出しでも、`src` の行と一文字も違わない
- `Block::Table` が行ごとの行番号を持つ
- `Block::Statement` が行ごとの生の行と行番号を持つ。1行目の字下げを含む
- `validate` の署名（`(schema, document, open) -> Vec<Finding>`）が変わらない
- `抽出`と`指摘`の出力はこのステップでは変わらない（土台だけ）

Shown by: test — RED → GREEN → REFACTOR。`document.rs` の `#[cfg(test)]` に、インラインコードを含む見出しの生の行が `src` と一致することを見る1本、字下げのある3行の`文`の行ごとの生の行と行番号が合うことを見る1本、ヘッダとデータ3行の表の行ごとの行番号が合うことを見る1本。
Left to the implementer: 持ち方（組の並びか、並びを別に持つか、専用の型か）。行頭まで遡る処理を `original_item_line` から切り出して共有するか、別に書くか。
Stop and hand back if: mdast の `TableRow` か段落の行が `position` を持たず、`src` の切り出しでも行番号を得られないと分かったとき。`Heading` に生の行を持たせると `Document` を返す既存の公開入口の型が変わり、契約の入口6つのどれかの署名に響くと分かったとき。

## Step 3 — 表の行ごとの行番号を抽出に出す

Purpose: `表`の`抽出`が`導かれる値`を含むとき、行ごとのオブジェクトにその鍵を足す。Specification: `docs/spec/ir-engine.md#R1`。
Prerequisites: ステップ2。
May change: `crates/kotowari-markdown-schema/src/extract.rs`（`extract_table`、`table_objects`）、`src/schema.rs`（鍵と列の名前の衝突の検査）、それぞれの `#[cfg(test)]`。
Done when:
- `導かれる値`を宣言した`表`で、各行のオブジェクトがデータ行の行番号を持つ。ヘッダの行と区切りの行は数えない
- `導かれる値`を宣言した`表`が繰り返すとき、表ごとの配列の中に行のオブジェクトが並ぶ
- `導かれる値`を宣言しない`表`の`抽出`は今と同じ。既存の `table_extract_concatenates_multiple_tables_in_document_order`（`extract.rs:1259`）と `repeated_table_extracts_flat_row_objects_without_nesting`（`:1280`）が緑のまま
- 行の鍵が表の列の名前と衝突するスキーマが `SchemaError` になる

Shown by: test — RED → GREEN → REFACTOR。`extract.rs` の `#[cfg(test)]` に、`導かれる値`を宣言した表で行ごとの行番号が出ることを見る1本、`導かれる値`を宣言した表が繰り返すとき表ごとの配列になることを見る1本。`schema.rs` の `#[cfg(test)]` に、列の名前と鍵が衝突するスキーマが `SchemaError` になることを見る1本。既存の2本が緑のままであることは、この計画のステップ6の `cargo test` が見る。
Left to the implementer: `table_objects` の分け方。衝突の検査を `validate_table` の中に置くか別の関数にするか。
Stop and hand back if: 列の名前が宣言されていない`表`（`Table.header` が `None`。`REQ-schema-033` はヘッダを宣言したときだけ照合する）で、衝突を検査する相手が無いと分かったとき。

## Step 4 — 文の行ごとの抽出

Purpose: `文`の`抽出`が`導かれる値`を含むとき、行ごとのオブジェクトの並びにする。Specification: `docs/spec/ir-engine.md#R2`。
Prerequisites: ステップ2。
May change: `crates/kotowari-markdown-schema/src/extract.rs`（`extract_statement`）、その `#[cfg(test)]`。
Done when:
- `導かれる値`を宣言した`文`で、行の数と同じ数のオブジェクトが出て、それぞれが生の行（1行目の字下げと末尾の空白を含む）と行番号を持つ
- 空行を挟んで続く段落（今は別の `Block::Statement` になり `"\n\n"` で繋がれている）も、行ごとのオブジェクトとして並ぶ
- `導かれる値`を宣言しない`文`の`抽出`は今と同じ1つの文字列。既存の `item_extract_becomes_an_object_when_internals_declare_extract`（`extract.rs:1626`）が緑のまま

Shown by: test — RED → GREEN → REFACTOR。`extract.rs` の `#[cfg(test)]` に、字下げのある3行の`文`で行の数と各行の文字と行番号が合うことを見る1本、空行を挟んだ段落も同じ並びに入ることを見る1本。既存の1本が緑のままであることはステップ6の `cargo test` が見る。
Left to the implementer: 並びの組み立て方。
Stop and hand back if: 空行を挟んだ段落を1つの並びに入れると、`REQ-schema-019`（`出現回数`）の`文`の数え方が変わると分かったとき（今は段落ごとに1つの `Block::Statement` なので、`repeat` の上限の判定に響く）。

## Step 5 — 指摘に行番号とノードの名前と生の行を足す

Purpose: `指摘`が、行番号、宣言された`ノード`の名前、その行の生の文字を持つようにする。Specification: `docs/spec/ir-engine.md#R3`、`#R4`、`#R5`。
Prerequisites: ステップ2。
May change: `crates/kotowari-markdown-schema/src/finding.rs`、`src/validate.rs`、`src/main.rs`、`crates/kotowari-markdown-schema/README.md`、`crates/kotowari-markdown-schema/docs/spec/mds.md`、それぞれの `#[cfg(test)]`、`crates/kotowari-markdown-schema/tests/cli.rs`。
Done when:
- 必須の`フィールド行`が無い`項目`の`指摘`がその`項目`の見出しの行を持つ（今は行が無い）
- 同じ`フィールド行`が2つある`指摘`が2つ目の行を持つ（今は行が無い）
- 表の行の列の数が合わない`指摘`がその行を持つ（今はヘッダの行を指している）
- `題名`が無い`指摘`のように、それを含む`ノード`に行が無いものは行番号を省いたまま
- ノードの名前を持つのは`節`と`フィールド行`の`指摘`だけ。`題名`・`文`・`表`・`コードブロック`・`前置部`・`項目`の`指摘`では省く
- 行を持つ`指摘`が、その行の生の文字を一文字も違わずに持つ
- CLI の JSON がノードの名前と生の行を出す。既にある英文の detail は変えない

Shown by: test — RED → GREEN → REFACTOR。`validate.rs` の `#[cfg(test)]` に、必須の`フィールド行`を落とした入力で`項目`の見出しの行とノードの名前 `種類` が付くことを見る1本、同じ`フィールド行`を2つ書いた入力で2つ目の行が付くことを見る1本、列の足りない表の行でその行が付くことを見る1本、`題名`が無い入力で行番号もノードの名前も無いことを見る1本、`ID` の形に合わない見出しで生の行が `src` と一致することを見る1本。`tests/cli.rs` に、JSON の1件がノードの名前と生の行の鍵を持つことを見る1本。
Left to the implementer: `Finding` の新しい要素の型（`Option<String>` か専用の型か）。26 か所の組み立ての直し方（組み立て関数を1つ作るか、各所で埋めるか）。README と `docs/spec/mds.md` の言い回し。
Stop and hand back if: 26 か所のうち、違反した`ノード`も欠落を含む`ノード`も特定できず、行を省くのが正しいかも決められない箇所があるとき。ノードの名前を持つ`ノード`が`節`と`フィールド行`のほかにあると分かったとき。

## Step 6 — 終端の確認

Purpose: 5件が揃ったことと、契約とほかの製品を壊していないことを見る。Specification: `docs/spec/ir-engine.md#R1`〜`#R5`。
Prerequisites: ステップ1〜5。
May change: なし（直しが要れば当該のステップに戻る）。
Done when: 下の Shown by のすべてが通る。
Shown by: check — 次の順に走らせる。
1. `CARGO_BUILD_JOBS=4 cargo test -p kotowari-markdown-schema` が全件通る（既存のテストのうち、`導かれる値`を宣言しない場合の`抽出`の形を固定している3本が緑のままであることを含む）
2. `CARGO_BUILD_JOBS=4 cargo test -p kotowari-markdown-schema --test library` が通る（`TBL-schema-010` の入口6つの契約。`validate` の署名が変わっていないこと）
3. `CARGO_BUILD_JOBS=4 cargo test --workspace` が全件通る（同じ workspace のビルドと kotowari 側のテストが壊れていないこと。kotowari は今このクレートに依存していないので、落ちるとすればビルドか feature の統合が原因）
4. `CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format text` の指摘が0件
5. `CARGO_BUILD_JOBS=4 cargo clippy --workspace --all-targets -- -D warnings` の誤りが基準線から増えていない（基準線はステップ1の前に実測して記録する）
6. `CARGO_BUILD_JOBS=4 cargo +1.89.0 check -p kotowari-markdown-schema --all-targets` が通る（このクレートの宣言した最低の Rust）

Left to the implementer: なし。
Stop and hand back if: 6 が落ちて、原因が新しく使った言語機能にあるとき（`rust-version` を上げるかは版付けの判断で、この計画の外）。

## Verification map

| 仕様 | 確かめるステップ |
|---|---|
| R1 表の行ごとの行番号 | ステップ1（要求）、ステップ2（土台）、ステップ3（抽出） |
| R2 文の行ごとの抽出 | ステップ1（要求）、ステップ2（土台）、ステップ4（抽出） |
| R3 指摘の行番号 | ステップ1（要求）、ステップ5 |
| R4 指摘のノードの名前 | ステップ1（要求）、ステップ5 |
| R5 指摘が指す行の生の文字 | ステップ1（要求）、ステップ2（土台）、ステップ5 |

## Left to the implementer

- `Document` の中の生の行と行ごとの位置の持ち方
- `Finding` の新しい要素の型
- 26 か所の`指摘`の組み立ての直し方
- 単体テストを置くモジュール（`document.rs`・`extract.rs`・`validate.rs` の `#[cfg(test)]` が既にテストを持つので、そこに足すのが素直）
- README と `docs/spec/mds.md` の言い回し

## この計画が決めたこと（仕様に戻さず決めた判断）

- **生の行は `Document` の中に持たせ、`validate` の署名を変えない。** 契約の入口6つを守るため
- **CLI の JSON にノードの名前と生の行を出す。** 仕様の R3〜R5 の確かめ方が `mds check --format json` の出力を見ることになっているので、出さない選択は仕様に反する
- **それを含む`ノード`に行が無い`指摘`は行番号を省く。** `REQ-schema-008` の既存の規則を残す。kotowari の `REQ-core-027`（9種類の "line" を null）と噛み合う
- **`導かれる値`を宣言しないときの`抽出`の形は今のまま。** 既存のテスト3本を書き換えない
- **宣言上の名前を持つ`ノード`は`節`と`フィールド行`だけ。** ほかは名前を省く

## Stop conditions

- 仕様が黙っている振る舞いを決めないといけなくなったとき（`抽出`の新しい形、`指摘`の新しい要素の意味、`停止`の新しい理由）は仕様に戻す
- 契約の入口6つのどれかの署名を変えないと進めないと分かったとき
- `TBL-schema-008` の`箇条書き`か`コードブロック`も変えないと書けないと分かったとき（A12 が広げないと決めている）
- `crates/kotowari-core/` か `src/` を直さないと `cargo test --workspace` が通らないとき
- 公開済みの v0.1.0 との非互換が、仕様が述べた`表`と`文`の`抽出`の形のほかに見つかったとき

## Test command

`CARGO_BUILD_JOBS=4 cargo test -p kotowari-markdown-schema`。workspace 全体は `CARGO_BUILD_JOBS=4 cargo test --workspace`。`CARGO_BUILD_JOBS=4` はメモリの都合で、`lefthook.yml` の pre-commit と pre-push が同じ形で走らせている。

## Out of scope

- kotowari 側の置き換え（仕様の R6〜R15。別の計画）
- `.mds/schemas/` のスキーマの宣言を変えること（`出典` と`コードブロック`への行番号の宣言は仕様の R8 で kotowari 側の計画）
- `箇条書き`と`コードブロック`の要素ごとの`導かれる値`（A12）
- CLI のバイナリの名前の変更（A26）
- このクレートの版を上げること、`CHANGELOG.md` への追記（版付けは保留の判断）
