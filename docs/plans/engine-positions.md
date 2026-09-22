# 実装計画: 抽出の鍵の出どころと宣言の形

## Goal

`kotowari-markdown-schema` の`抽出`が、宣言を入れ子の形で受け、行オブジェクトの鍵を`文書`のヘッダ行の文字から作るのをやめ、要素ごとの`導かれる値`に`生の行`を加える。kotowari 側の置き換えはこの計画では行わない。

## Specification

IR の `docs/ir/schema/`。この計画が受け持つのは `docs/ir/schema/extraction.md#REQ-schema-048` と `docs/ir/schema/extraction.md#TBL-schema-008`（`抽出`の宣言の形、`ノード`ごとの要素の単位、`表`の行の鍵の出どころ）と、`docs/ir/schema/cli.md#TBL-schema-009` と `docs/ir/schema/cli.md#REQ-schema-042`（要素オブジェクトの中の鍵が重複するスキーマの`停止`）である。観測できる条件は `docs/ir/schema/extraction.md` の EX-schema-018〜EX-schema-025 と `docs/ir/schema/cli.md` の EX-schema-029・EX-schema-030 にある。

`指摘`の行番号・`ノードの名前`・`生の行`（`docs/ir/schema/cli.md#REQ-schema-008`）はこのブランチで既に実装済み（下の「既にブランチにあるもの」を読むこと）。kotowari 側の置き換え（`docs/ir/core/` の REQ-core-168〜REQ-core-177）は別の計画。判断の記録は `docs/decision/records/2026-09-22-ir-engine.md`（A41〜A67 と Revisions がこの計画の範囲の判断。散文の仕様書にあった要求の番号から IR の項目への対応表は、その記録の `## Context` にある）。

## 既にブランチにあるもの

ブランチ `engine-positions` には base `9e863a7` から 11 コミットが入っている。**やり直しではなく、この上に積む。**

そのまま生きているもの（触る必要はない）:

- `Document` が生の行と行ごとの位置を持つ土台（`60d1b29`）
- `指摘`が行番号・ノードの名前・生の行を持つ（`77051fa`、`e1a8db2`）。仕様の R3・R4・R5
- 単独 CR を行区切りとして数える（`6606205`）
- コメントの仕様への参照（`811b6a3`。その後 `01f6ada` で `mds.md` を畳んだので、参照先は IR の要求・決定表の ID になった）

**この計画が作り直すもの**（仕様が変わって偽になった）:

- `edec48d` が書いた `docs/ir/schema/extraction.md` の `TBL-schema-008` と `REQ-schema-048`
- `3b81920`・`8298727` が入れた`表`と`文`の`導かれる値`の置き方
- `1192e6a` が書いた「行の鍵が`表`の列の名前と衝突したら`停止`」（R18 の重複の`停止`が置き換える。仕様の「却下したもの」に理由がある）
- `60d1b29` が入れた `Heading` の生の行（A46 で外す）

## Approach and why

**宣言の形（R16）を先に置き、意味を後から変える。** `schema.rs` の `Extract`／`Extracts`（`:284`・`:302`）は「書式の並び」で、`serde(untagged)` で3つの書式を読み分けている。これを入れ子の構造体に替えると、`.mds/schemas/*.yaml` と単体テストの中のスキーマが**すべて読めなくなる**ので、型と書き換えを同じステップに入れないとどのステップでもビルドが通らない時間が生まれる。一方で、そのステップでは`抽出`の**出力を1バイトも変えない**ことができる（下記）。構文の入れ替えと意味の変更を分けると、意味を変えるステップで出た差分が本当にその意味の変更によるものだと分かる。

**構文だけの入れ替えで出力が変わらない根拠。** ステップ3が変えるのは宣言の**読み方**だけで、読んだあとの置き方には触れないからである。旧来の並びの各書式は新しい入れ子の各鍵へ1対1で移る（`- a` → `path: a`、`- {path: b, of: line}` → `of: { b: line }`、`- {path: c, group: g}` → `group: g` と `path: c`）。

**`of` を宣言している箇所は `.mds/schemas/` の外にも多い。** `.mds/schemas/ir.yaml` のほか、`src/extract.rs` と `src/schema.rs` の `#[cfg(test)]` と `tests/library.rs` の中のスキーマにもある。`ir.yaml` のものはすべて `item:` の下だが、テストの中には**`フィールド行`・`表`・`文`に宣言したものがある**。ステップ3はこれらを**すべて**新しい構文へ書き換える。対象は `rg -n '^\s*-\s*\{\s*path:' .mds/schemas crates/kotowari-markdown-schema/src crates/kotowari-markdown-schema/tests` と、Rust の文字列リテラルの中に `\n` で畳まれた同じ形で見つかる。`tests/library.rs` は契約テストだが、契約は公開の入口6つの署名であってテストの中の YAML ではないので、書き換えてよい。書き換え漏れは、ステップ3が旧来の並びを`停止`にするのでテストが落ちて露見する。

**既存のテストの期待値が変わるものがある。** ステップ3では変わらないが、意味を変えるステップで次の3群が変わる。各ステップの Done when に明記してあるので、実装役は期待値を黙って書き換えず、そのステップの仕事として直す。

| 変わるテスト | 変わるステップ | 何が変わるか |
|---|---|---|
| `of_line_places_the_line_number_as_a_number`（`extract.rs:1790`）ほか`フィールド行`に `of` を宣言したもの | ステップ5 | 今は`配置パス`と`導かれる値`が最上位に並ぶ（`{"status":"ok","status_line":3}`）。R17 の「分けない`ノード`でも`配置パス`の下にオブジェクトができる」で入れ子になる |
| `表`に `of` を宣言したもの（`extract.rs:1431` 付近・`:1458` 付近） | ステップ5・6 | 要素オブジェクトが `value` の下に入り、鍵の出どころが`文書`のヘッダ行から宣言か位置に変わる |
| `文`に `of` を宣言したもの（`extract.rs:1380` 付近・`:1406` 付近） | ステップ5・7 | 暗黙の `"text"` の鍵が無くなり、`要素の値`が前後の空白を取り除いた文字になる |

これは公開済み v0.1.0 との非互換で、仕様の A42 が許している。ここに無い既存のテストの期待値を変えないといけなくなったら、それは仕様に書いていない振る舞いを変えているので止めて手渡す。

**鍵の出どころ（R1）と要素の単位（R17）を別のステップにする。** 前者は `table_objects`（`extract.rs:465`）が`文書`のヘッダ行から鍵を作るのをやめる話で、後者は `place_all`（`:665`）と各 `extract_*` が要素オブジェクトを組み立てる話である。同じファイルだが変える関数が違い、確かめ方も違う（前者は鍵の名前、後者は入れ子の段）。

**IR とエンジンの仕様を先に改める。** 規律上の順序であって技術的な前提ではない。`docs/ir/schema/extraction.md` は要求が 10 件で上限ちょうどなので、足さずに既存の要求を改める。

## Scope of change

変えてよい場所:

- `docs/ir/schema/extraction.md`、`docs/ir/schema/cli.md`（要求と決定表）
- ~~`crates/kotowari-markdown-schema/docs/spec/mds.md`~~（`01f6ada` で削除。仕様は `docs/ir/schema/` 1つに集約した）。~~`docs/spec/ir-engine.md`~~ も同じく IR へ畳んだ
- `crates/kotowari-markdown-schema/README.md`（`抽出`の宣言の例が古くなる）
- `crates/kotowari-markdown-schema/src/schema.rs`、`src/extract.rs`、`src/document.rs`
- 上の各ファイルの `#[cfg(test)]`、`crates/kotowari-markdown-schema/tests/`
- `.mds/schemas/ir.yaml`、`.mds/schemas/context.yaml`、`.mds/schemas/flags.yaml`

触らない場所:

- `crates/kotowari-core/`、`src/`、`tests/`（kotowari 側は別の計画）
- `docs/ir/core/`
- `crates/kotowari-markdown-schema/src/finding.rs`（`指摘`の形は R3〜R5 で決着済み）。`src/validate.rs` は、ステップ5で `Heading` の生の行を外したときの追従（コンパイルを通すための参照の差し替え）だけ触れてよく、`指摘`の中身を変えてはならない
- `crates/kotowari-markdown-schema/CHANGELOG.md` と版（仕様の「作らないもの」）
- `docs/ir/` と `docs/decision/records/`（承認済み。偽だと分かったら止めて手渡す）

## Step order and prerequisites

ステップ1と2は規律上の順序で、技術的な前提ではない。ステップ3が土台で、ステップ4〜7はこれに乗る。ステップ4はステップ3だけに依存する。ステップ5→6→7 は同じ `extract.rs` を順に触るので、この順に行う。ステップ8が最後。

## Step 1 — IR の要求を改める

Purpose: `抽出`の宣言の形と鍵の出どころと`導かれる値`の語を、R1・R2・R16〜R18 の振る舞いに合わせる。Specification: `docs/ir/schema/extraction.md#REQ-schema-048`、`docs/ir/schema/extraction.md#TBL-schema-008`、`docs/ir/schema/cli.md#TBL-schema-009`。
Prerequisites: なし。始める前に次の3つを1回ずつ走らせ、結果を `.agents/baseline-engine-positions.md` に追記する（ファイルが無ければ作る。`.agents/` は git が無視する）。ステップ8の検査5はこのファイルの値と比べる。
- `CARGO_BUILD_JOBS=4 cargo test -p kotowari-markdown-schema`（通った件数）
- `CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format text`（指摘の数）
- `CARGO_BUILD_JOBS=4 cargo clippy --workspace --all-targets -- -D warnings`（誤りの件数と、その位置がどのクレートか）
May change: `docs/ir/schema/extraction.md`、`docs/ir/schema/cli.md`。
Done when:
- `REQ-schema-048` が`導かれる値`を4つ（行番号・`項目`の見出しの ID・`項目`の見出しの名前・`生の行`）として述べ、**列の名前との衝突の`停止`の文が無くなっている**（R18 が置き換える。仕様の「却下したもの」に理由がある）
- `REQ-schema-047` か `REQ-schema-048` のいずれかが、`抽出`の宣言が `path`・`value`・`of`・`group` を持つ入れ子であることと、`path` が必ず1つであることを述べる
- `TBL-schema-008` の**8行すべて**が仕様 R17 の表（`ノード`ごとの要素の単位と`要素の値`）と矛盾しない記述になっている。`表`の行は「行の鍵は `header` を宣言すればその名前、しなければ列の位置。`文書`のヘッダ行の文字は鍵に使わない」、`文`の行は「`導かれる値`を宣言したときだけ行ごとの要素」を述べる。`箇条書き`・`コードブロック`・`題名`・`フィールド行`の行も、R16・R17 が要素の単位と `value`・`of` を広げたことに合わせる
- `docs/ir/schema/cli.md` の `TBL-schema-009`（`停止`の理由）の**行数が5のまま**で、「スキーマが形に合わない」の場面に要素オブジェクトの鍵の重複が入っている
- 既存の要求の ID を変えない。要求を足さない

Shown by: check — 次の順。
1. `CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format text` の指摘が0件のまま
2. `grep -c '^### REQ-' docs/ir/schema/extraction.md` が 10、`docs/ir/schema/cli.md` が 9
3. `grep -c '^| ' docs/ir/schema/cli.md` の `TBL-schema-009` の節が見出し・区切り・5行のまま（目で数えてよい）
4. `rg -n 'ヘッダの列の名前と同じとき' docs/ir/schema/` が0件（変更前は `docs/ir/schema/extraction.md` の `REQ-schema-048` に1件ある。「衝突」の語は変更前から `docs/ir/schema/` に0件なので、検査の語に使わない）

Left to the implementer: 文の言い回し。`抽出`の入れ子の形を `REQ-schema-047` と `REQ-schema-048` のどちらに書くか。具体例は `EX-schema-018`〜`EX-schema-025` と `EX-schema-029`・`EX-schema-030` を既に書いてある（`EX-schema-031` 以降が空き）。
Stop and hand back if: 要求を足さずには書けないと分かったとき。`TBL-schema-009` の5つの理由のどれにも当てはめられないと分かったとき（仕様 R18 は `schema_invalid` を使うと定めている）。

## Step 2 — エンジン自身の仕様を改める（実行後に取り消した）

**このステップは `01f6ada` で取り消された。** 当時は `crates/kotowari-markdown-schema/docs/spec/mds.md` を
IR と並ぶ散文の仕様として保つ前提だったが、同じ規則を2か所に書くこと自体が誤りだった。検査が当たるのは
IR だけで、散文の側は書いた人の記憶でしか正しさを保てず、実際にずれていた（この cycle だけで
4件の指摘がその食い違いだった）。`mds.md` とクレート直下の用語集を畳み、仕様を `docs/ir/schema/` 1つにした。

このステップの成果（`92f80ac`）は `01f6ada` に含まれる削除で消えている。
IR の側の改訂はステップ1が担っている。

## Step 3 — 抽出の宣言を入れ子の形にする

Purpose: `抽出`の宣言を `{path, value?, of?, group?}` の入れ子で読み、旧来の書式の並びを受けなくする。**このステップでは`抽出`の出力を変えない。** Specification: `docs/ir/schema/extraction.md#REQ-schema-048`。
Prerequisites: なし（ステップ1と2は規律上の順序）。
May change: `crates/kotowari-markdown-schema/src/schema.rs`、`src/extract.rs`（型の追従だけ）、それぞれの `#[cfg(test)]`、`crates/kotowari-markdown-schema/tests/`、`.mds/schemas/ir.yaml`、`.mds/schemas/context.yaml`、`.mds/schemas/flags.yaml`。
Done when:
- `extract: { path: X, of: { k: line } }` の形が読める。`of` は語から`配置パス`への対応（`{ 鍵: 語 }`）として読む
- `extract: X` の略記が `{ path: X }` と同じに読める
- `extract: { path: X, group: g }` が読める（`group` は `path` の兄弟）
- 旧来の並び（`extract: [- a, - {path: b, of: line}]`）が `schema_invalid` の`停止`になる
- `path` の無い宣言が `schema_invalid` の`停止`になる
- `.mds/schemas/` の3ファイルと、単体テスト・結合テストの中のスキーマ（`tests/library.rs` の契約テストの中の YAML を含む）に**旧来の並びの書式が1件も残っていない**
- **`抽出`の出力が1件も変わらない**（`value` は読めるが置き方はステップ5、`of` の置き方は今のまま）。既存のテストの期待値を1件も書き換えない

Shown by: test — RED → GREEN → REFACTOR。`schema.rs` の `#[cfg(test)]` に、入れ子の宣言が読めることを見る1本、略記が同じに読めることを見る1本、旧来の並びが `SchemaError` になることを見る1本、`path` の無い宣言が `SchemaError` になることを見る1本。出力が変わらないことは、既存の `extract.rs` と `tests/` のテストが緑のまま通ることが示す。
Left to the implementer: 型の作り方（構造体か列挙か、`of` の対応の持ち方）。`Extracts` を残すか単一の構造体にするか。`.mds/schemas/` の書き換えの進め方。
Stop and hand back if: `of` を `{ 鍵: 語 }` で読むと、`項目`の `id`・`name`・`line` を同時に宣言している今の `ir.yaml` の意味が変わると分かったとき。`serde` の `untagged` を外すと `frontmatter` か `Schema` のほかの部分の読み取りが壊れると分かったとき。`.mds/schemas/` を書き換えても `kotowari check` の指摘が0件に戻らないとき。

## Step 4 — 要素オブジェクトの中の鍵の重複を停止にする

Purpose: 1つの要素オブジェクトの中で鍵が重複するスキーマを、スキーマを読んだ時点で弾く。Specification: `docs/ir/schema/cli.md#TBL-schema-009`、`docs/ir/schema/cli.md#REQ-schema-042`。
Prerequisites: ステップ3。
May change: `crates/kotowari-markdown-schema/src/schema.rs`、その `#[cfg(test)]`。
Done when:
- 内側の`ノード`の`配置パス`、`value`、`of` の鍵のいずれかが重複するスキーマが `schema_invalid` の`停止`になる
- 判定が`配置パス`全体で行われ、`a.b` と `a.c` は通り、`a` と `a.b` は`停止`する
- 判定が`文書`を読まずに行われる（`parse_schema` の中で完結し、`Document` を引数に取らない）
- `1192e6a` が入れた「行の鍵が`表`の列の名前と衝突したら`停止`」の検査とそのテストが無くなっている

Shown by: test — RED → GREEN → REFACTOR。`schema.rs` の `#[cfg(test)]` に、内側の`フィールド行`の `extract: line` と外側の `of: { line: line }` が重複して `SchemaError` になることを見る1本、`value` と `of` の鍵の重複を見る1本、`a` と `a.b` が `SchemaError` になり `a.b` と `a.c` は通ることを見る1本。
Left to the implementer: 検査を置く関数。`配置パス`の親子の判定の書き方。
Stop and hand back if: 今の `.mds/schemas/` のどれかがこの検査に引っかかると分かったとき（引っかかるなら、スキーマが間違っているのか検査が広すぎるのかを決める必要がある）。

## Step 5 — 要素の単位と value の置き方、導かれる値の raw

Purpose: `抽出`の**構造**を仕様のとおりにする。`ノード`ごとの要素の単位、`要素の値`を `value` の鍵へ置くこと、`導かれる値`の `raw`、繰り返しの段。Specification: `docs/ir/schema/extraction.md#REQ-schema-048`、`docs/ir/schema/extraction.md#TBL-schema-008`。
Prerequisites: ステップ3。
May change: `crates/kotowari-markdown-schema/src/extract.rs`、`src/schema.rs`（`OfKind` に `raw` を足す）、`src/document.rs`（`Heading` の生の行を外す）、`src/validate.rs`（`Heading` を外したときのコンパイルを通すための追従だけ。`指摘`の中身を変えない）、それぞれの `#[cfg(test)]`。

**このステップが変えないもの（次の2ステップの仕事）**: `表`の行の鍵がどこから来るか（ステップ6。このステップの時点では今までどおり`文書`のヘッダ行の文字のまま）。`文`の`要素の値`が前後の空白を含むかどうか（ステップ7。このステップの時点では今までどおり行そのままの文字）。

Done when:
- `OfKind` が `raw` を受け、`ノード`ごとに仕様 R17 の表の「`raw` が指す行」の列のとおりの行を返す
- 要素の単位が R17 の表の「要素の単位」の列のとおりになる。`コードブロック`は**ブロック**単位、`箇条書き`は**行**単位で、どちらも `value` と `of` を宣言できる（`extract_bullets`（`extract.rs:278`）と `extract_codeblock`（`:504`）は今 `of` を一切見ていないので、新しく作る）
- `value` を宣言した`ノード`で、`要素の値`が要素オブジェクトの中のその鍵に入る
- `value` を省いた`ノード`で、要素オブジェクトが`導かれる値`の鍵と内側の`ノード`の`配置パス`だけを持つ
- 要素に分けない`ノード`（`題名`・`フィールド行`・`節`・`前置部`）でも、`value` か `of` を宣言すれば`配置パス`の下にオブジェクトができる。**既存の `of_line_places_the_line_number_as_a_number`（`extract.rs:1790`）はこの形に変わるので、期待値を入れ子に直す**（黙って書き換えるのではなく、このステップの仕事として直す）
- `repeat` で繰り返す`ノード`が`配置パス`の直下に段を作る
- `statement_line_object`（`extract.rs:261`）が作っていた暗黙の `"text"` の鍵が無くなり、`文`の行の`要素の値`が宣言された `value` の鍵に入る
- `Heading` の生の行が無くなり、`指摘`の生の行が `Document::lines` から取られたままである
- `導かれる値`を宣言しない略記の出力が変わらない

Shown by: test — RED → GREEN → REFACTOR。`extract.rs` の `#[cfg(test)]` に、`コードブロック`が2つある`文書`で `of: { k: line }` が塊ごとの開始行を返すことを見る1本、`箇条書き`に `value` と `of: { k: line }` を宣言して行ごとのオブジェクトになることを見る1本、`題名`に `of: { k: raw }` を宣言して見出しの行がそのまま出ることを見る1本、`フィールド行`に `value` と `of` を宣言して`配置パス`の下がオブジェクトになることを見る1本、`value` を省いた`項目`が内側の`配置パス`を落とさないことを見る1本。`導かれる値`を宣言しない略記の出力が変わらないことは、既存のテストが緑のまま通ることが示す。
Left to the implementer: `Extracted` と `place_all`（`extract.rs:642`・`:665`）をどう作り替えるか。`ノード`ごとの行の取り方を1つの関数にまとめるか各所に置くか。`Heading` の生の行を外したときの `document.rs` の単体テストの扱い。
Stop and hand back if: `コードブロック`の`抽出`が今ブロック単位でなく、要素の単位を変えると`導かれる値`を宣言しない出力まで変わると分かったとき。`前置部`に開始行が無く、`of: { k: line }` を宣言したときに何を返すか仕様から決められないとき。上に挙げた「変えないもの」を変えずにはこのステップを終えられないと分かったとき。

## Step 6 — 表の行の鍵の出どころを変える

Purpose: `表`の行の鍵を`文書`のヘッダ行の文字から作るのをやめる。**鍵の出どころだけを変え、要素の構造（ステップ5）には触れない。** Specification: `docs/ir/schema/extraction.md#TBL-schema-008`（`表`の行）。
Prerequisites: ステップ5。
May change: `crates/kotowari-markdown-schema/src/extract.rs`（`extract_table`（`:421`）、`table_objects`（`:465`）の鍵を作る部分だけ）、その `#[cfg(test)]`、`crates/kotowari-markdown-schema/tests/`。
Done when:
- `header` を宣言した`表`の行の鍵が、宣言した名前になる
- `header` を宣言しない`表`の行が配列になる
- `header` の宣言と`文書`の列数が違うとき、宣言した名前の数を正とし、`文書`の列が足りなければその鍵を省き、多ければ余りを捨てる。`table_header_mismatch` の`指摘`は今までどおり出る
- ヘッダのセルが空の`表`（`|  |  |`）と、同じ名前の列が2つある`表`（`| a | a |`）のどちらでも、列の値が1つも失われない
- `.mds/schemas/context.yaml` の`用語集`（`header` を宣言している）の`抽出`の鍵が今と同じ名前のままである

Shown by: test — RED → GREEN → REFACTOR。`extract.rs` の `#[cfg(test)]` に、`header` を宣言しない`表`の行が配列になることを見る1本、ヘッダのセルが空の`表`で2列とも値が残ることを見る1本、宣言と`文書`の列数が違うときの省きと捨てを見る1本。`header` を宣言した`表`の鍵が宣言した名前になることは、`context.yaml` を使う既存のテストが緑のまま通ることが示す（宣言した名前と`文書`のヘッダ行が一致しているので出力が変わらない）。
Left to the implementer: `table_objects` の引数の作り方。配列にする実装の置き場。
Stop and hand back if: `table_header_mismatch` を出す経路（`validate.rs`）と`抽出`の経路が同じ列の情報を使っていて、片方だけ変えられないと分かったとき。`context.yaml` の`用語集`の鍵が変わってしまい、`kotowari check` の指摘が0件に戻らないとき。

## Step 7 — 文の要素の値と生の行を分ける

Purpose: `文`の`要素の値`を前後の空白を取り除いた文字にし、`継続段落`を`抽出`の要素から外す。**行ごとに分けること自体はステップ5までに済んでいる。** Specification: `docs/ir/schema/extraction.md#TBL-schema-008`（`文`の行）、`docs/ir/schema/extraction.md#REQ-schema-048`。
Prerequisites: ステップ5。
May change: `crates/kotowari-markdown-schema/src/extract.rs`（`extract_statement`（`:202`）、`statement_line_object`（`:261`））、その `#[cfg(test)]`。

**既にできていること（このステップの仕事ではない）**: `導かれる値`を宣言した`文`が行ごとの要素になること、空行で区切って続く段落が同じ並びに入ることは、`statement_with_a_derived_value_extracts_one_object_per_line` と `statement_with_a_derived_value_lists_paragraphs_separated_by_a_blank_line`（`extract.rs:1372` 付近）が既に固定している。これらは緑のまま通ること。

Done when:
- `要素の値`が前後の空白を取り除いた行の文字になる
- `of: { k: raw }` を宣言したとき、`k` が字下げと末尾の空白を含む行そのままになり、`要素の値`と別物である
- 一覧の行の`継続段落`が`文`の`抽出`の要素に入らない
- `導かれる値`を宣言しない`文`の`抽出`が今と同じ1つの文字列である

Shown by: test — RED → GREEN → REFACTOR。`extract.rs` の `#[cfg(test)]` に、字下げと末尾の空白を持つ行で`要素の値`と `raw` が別物になることを見る1本、`フィールド行`の`継続段落`が`文`の要素に入らないことを見る1本。`導かれる値`を宣言しない`文`が1つの文字列のままであることと、行ごとに分かれることは、既存のテストが緑のまま通ることが示す。
Left to the implementer: 前後の空白の取り方（`trim` の範囲）。`継続段落`を外す実装の置き場。
Stop and hand back if: `継続段落`を`文`から外すと、今`文`として数えている`文書`の検証結果（`REQ-schema-019` の`出現回数`）が変わると分かったとき。

## Step 8 — 終端の確認

Purpose: 5件が揃ったことと、契約とほかの製品を壊していないことを見る。Specification: `docs/ir/schema/extraction.md#REQ-schema-048`、`docs/ir/schema/extraction.md#TBL-schema-008`、`docs/ir/schema/cli.md#TBL-schema-009`。
Prerequisites: ステップ1〜7。
May change: なし（直しが要れば当該のステップに戻る）。
Done when: 下の Shown by のすべてが通る。
Shown by: check — 次の順に走らせる。
1. `CARGO_BUILD_JOBS=4 cargo test -p kotowari-markdown-schema` が全件通る
2. `CARGO_BUILD_JOBS=4 cargo test -p kotowari-markdown-schema --test library` が通る（`TBL-schema-010` の入口6つの契約。`validate` の署名が変わっていないこと）
3. `CARGO_BUILD_JOBS=4 cargo test --workspace` が全件通る
4. `CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format text` の指摘が0件
5. `CARGO_BUILD_JOBS=4 cargo clippy -p kotowari-markdown-schema --all-targets -- -D warnings` の指摘が0件、および `CARGO_BUILD_JOBS=4 cargo clippy --workspace --all-targets -- -D warnings` の誤りが `.agents/baseline-engine-positions.md` に記録した数から増えていない（workspace の実行は `crates/kotowari-core/` で止まるので、このクレートが lint されたかは `-p` の実行でしか分からない）
6. `CARGO_BUILD_JOBS=4 cargo +1.89.0 check -p kotowari-markdown-schema --all-targets` が通る

Left to the implementer: なし。
Stop and hand back if: 6 が落ちて、原因が新しく使った言語機能にあるとき（`rust-version` を上げるかは版付けの判断で、この計画の外）。

## Verification map

| 仕様 | 確かめるステップ |
|---|---|
| R1 表の行の鍵の出どころ | ステップ1（要求）、2（エンジンの仕様）、6（鍵の出どころ） |
| R2 文の行ごとの抽出 | ステップ1、2、5（行ごとに分ける構造）、7（値と生の行を分ける） |
| R16 抽出の宣言の形 | ステップ1、2、3（構文）、5（置き方） |
| R17 導かれる値の語と要素の単位 | ステップ1、2、5 |
| R18 鍵の重複の停止 | ステップ1、2、4 |

ステップ5・6・7 は同じ `extract.rs` を順に触るので、受け持ちを次のとおり分ける。5 が**構造**（要素の単位、`value` の鍵、`raw`、繰り返しの段）、6 が`表`の**鍵の出どころ**、7 が`文`の**値の中身**（trim と`継続段落`）。5 は 6・7 が受け持つものを変えない。

## Left to the implementer

- `抽出`の宣言を表す型の作り方（構造体か列挙か、`of` の対応の持ち方）
- `Extracted` と `place_all` の作り替え方
- `ノード`ごとの行の取り方を1つの関数にまとめるか各所に置くか
- `.mds/schemas/` の3ファイルの書き換えの進め方
- 単体テストを置くモジュール（`schema.rs`・`extract.rs` の `#[cfg(test)]` が既にテストを持つので、そこに足すのが素直）
- `mds.md` と `README.md` の言い回し

## Stop conditions

- 仕様が黙っている振る舞いを決めないといけなくなったとき（`抽出`の新しい形、`導かれる値`の新しい語、`停止`の新しい理由）は仕様に戻す。**前の回でこれを飛ばして実装役が鍵の名前を自分で決めた前例がある**
- 契約の入口6つ（`docs/ir/schema/library.md#TBL-schema-010`）のどれかの署名を変えないと進めないと分かったとき
- `停止`の理由を6つ目として足さないと進めないと分かったとき（仕様 R18 は5つのままと定めている）
- `crates/kotowari-core/` か `src/` を直さないと `cargo test --workspace` が通らないとき
- `.mds/schemas/` を書き換えても `kotowari check` の指摘が0件に戻らないとき
- 公開済みの v0.1.0 との非互換が、**上の「既存のテストの期待値が変わるもの」の表に載る3群（`フィールド行`・`表`・`文`）と、仕様が述べた鍵の出どころのほか**に見つかったとき

## Test command

`CARGO_BUILD_JOBS=4 cargo test -p kotowari-markdown-schema`。workspace 全体は `CARGO_BUILD_JOBS=4 cargo test --workspace`。`CARGO_BUILD_JOBS=4` はメモリの都合で、`lefthook.yml` の pre-commit と pre-push が同じ形で走らせている。

## Out of scope

- kotowari 側の置き換え（仕様の R6〜R15。別の計画）
- **仕様の R8（スキーマに宣言する行番号）。** R8 は R6〜R15 の置き換えの一部で、`出典`の`フィールド行`・`具体例`の`コードブロック`・`文`に行番号と`生の行`を宣言して kotowari が使う話である。この計画は `.mds/schemas/` を新しい構文へ**機械的に移すだけ**で、新しい宣言を足さない。R8 の成功の条件（`source_invalid` の行、gherkin の絶対の行、`unclosed_backtick` の detail）はどれも kotowari 側でしか確かめられない
- `指摘`の形（仕様の R3〜R5。このブランチで実装済み）
- CLI のバイナリの名前の変更
- このクレートの版を上げること、`CHANGELOG.md` への追記
