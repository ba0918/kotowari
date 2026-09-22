# 実装計画: 読み取りの置き換えで変わった振る舞いを戻す

## Goal

`IR` の読み取りをエンジンに置き換えたことで変わった振る舞いが、判断の記録どおりに戻る。エンジン（`kotowari-markdown-schema`、以下エンジン）は既定の読み方を変えずに新しい宣言を受け、kotowari はその宣言を使って置き換えの前の振る舞い（とその記録が決めた例外）を出す。

## Specification

IR の置き場は `docs/ir/`。判断の記録は `docs/decision/records/2026-09-23-ir-engine-gaps.md`（A1〜A42。A3・A14・A24 は Revisions で改めた）。

この計画が受け持つ項目は次のとおり。本文は `kotowari query <ID>` で読む（文書を開いて探さない。要る鍵だけを `jq` で取る）。

エンジン側（`docs/ir/schema/`）:

- 読み方: `REQ-schema-058`、`REQ-schema-060`、`REQ-schema-030`、`REQ-schema-032`、`TBL-schema-011`、`PROP-schema-008`
- 表: `REQ-schema-059`、`REQ-schema-033`
- 文書の直下の項目: `REQ-schema-016`、`REQ-schema-061`、`TBL-schema-004`
- 要素の最後の行: `REQ-schema-048`、`REQ-schema-062`、`TBL-schema-008`
- 配置パスの衝突: `REQ-schema-042`、`TBL-schema-009`
- 題名: `REQ-schema-022`
- 指摘の形: `REQ-schema-008`

kotowari 側（`docs/ir/core/`）:

- 文の読み方と取り込むスキーマ: `REQ-core-178`、`REQ-core-179`、`TBL-core-011`
- 写し: `TBL-core-029`、`TBL-core-030`、`REQ-core-172`、`REQ-core-174`、`TBL-core-008`、`REQ-core-035`
- 用語集: `REQ-core-117`、`REQ-core-122`
- query の本文: `TBL-core-027`、`REQ-core-169`、`REQ-core-170`

テストの無い要求と具体例（着手時点。`kotowari list | jq -r '.items[] | select(.tests == [] and .verification != "review") | .id' | grep -E '^(REQ|EX)'` で取れる 37 件）は、下の各ステップの Shown by が名指す。

**着手の前提。** `問題の記録`は両方の置き場で0件で、`kotowari check` の誤りはテスト側の指摘（requirement_without_test 6 件、scenario_without_test 31 件）だけ。この計画が待つ人の判断は無い。

## Approach and why

**エンジンの既定は変えない。** 記録の A12 は、mds の既定の読み方（CommonMark の段落）を残し、IR が要る読み方はスキーマの宣言で選ぶと決めた。新しい宣言（`reading`、`select`、`document.item`、`end`）は、書かないスキーマの結果を1つも変えない。既定を変えてよいのは余ったセル（A16）と題名が複数あるときの指摘の数（A17）の2つだけ。各エンジンのステップで、宣言を書かないスキーマの既存のテストが緑のままであることが、この約束の証拠になる。

**エンジンのライブラリの入口6つの署名を変えない。** `TBL-schema-010` の入口は契約である。`Document::parse` はスキーマを受けないので、`reading: line` の読み分けは `validate` と `extract_values` の中で（スキーマの `reading` と文書の行を見て）行う形を想定している。`Document::parse` は CommonMark の見出しから題名と節の木を先に組むので、`#` だけの行や`文`の次の `---`・`===` の行が題名や節として木に入る。`reading: line` ではブロックの読み替えだけでは足りず、`Document` が保っている生の行（`lines`）から木ごと組み直すことになる。署名を変えないと進めないと分かったら止める。

**エンジンを先に、kotowari を後に。** kotowari が取り込むスキーマ（`.mds/schemas/`）に新しい宣言を書くには、エンジンがそれを受ける必要がある。エンジンのステップ1〜6は互いに独立で、どの順でもよい。ステップ7でスキーマに宣言を書いて kotowari の文の読み方を戻し、ステップ8で写しの表、ステップ9で query を直す。

**このリポジトリの IR の検査結果は変えない。** 置き換えの前後で差分0だったのと同じく、この計画のどのステップでも `kotowari check` が `docs/ir/` と `docs/decision/records/` に新しい指摘を出してはならない（変わってよいのはテスト側の指摘が閉じていくことだけ）。

**置き換えの工程でテストの入力を変えて避けていた2本を元に戻す。** 置き換えの実装役は、`フィールド行`の直後の`文`が吸われる差を避けるために、`tests/step3_sources_terms.rs` の `tbl_008_unclosed_backtick_detail_keeps_leading_indentation` の入力に `"文。\n\n"` を挿し、`tests/step2_ir.rs` の `req_098_review_requirement_without_how_to_verify_is_a_missing_field` のテンプレートの `"{how_to_verify}\n文である。"` を `"{how_to_verify}\n\n文である。"` に変えた。A1・A2 で`文`として読むと決めたので、ステップ7で2本とも置き換えの前の入力（`git show bbc2211:<path>` で読める）に戻し、そのまま通ることを示す。

## Scope of change

変えてよい場所:

- `crates/kotowari-markdown-schema/src/` と `crates/kotowari-markdown-schema/tests/`（ステップ1〜6）
- `crates/kotowari-markdown-schema/README.md`（新しい宣言の説明。ステップ1〜6）
- `.mds/schemas/ir.yaml`、`.mds/schemas/context.yaml`、`.mds/schemas/flags.yaml`（ステップ7）
- `crates/kotowari-core/src/`（ステップ7〜9）
- `tests/`（根の統合テスト。`crates/kotowari-core/` に `tests/` は無く、単体テストは `src/` の `#[cfg(test)]` に置く）
- `skills/kotowari/references/ir-form.md`、`skills/kotowari/references/findings.md`、`PROJECT.md`（ステップ10）

触らない場所:

- `docs/ir/` のすべて（要求・決定表・性質・具体例・用語集・問題の記録）
- `docs/decision/records/`
- テストの作り物の `FLAGS.md` に置き換えの工程で足した `## 問題の記録` の節（A4 のとおり、あっても無くても読めるので残してよい）

## Step order and prerequisites

ステップ1〜6（エンジン）は互いに独立で、前提を持たない。ステップ7はステップ1・2・3・4の後。ステップ8はステップ2・6・7の後。ステップ9はステップ4・7の後。ステップ10はステップ8の後。ステップ11はステップ1〜10の後。

## Step 1 — エンジンが `reading` を受け、`line` のときに行で読む

Purpose: スキーマの最上位の `reading` を受け、`line` のときは見出しの下の行を1行ずつ読み分ける。Specification: `docs/ir/schema/schema-language.md#REQ-schema-060`、`#PROP-schema-008`、`docs/ir/schema/block-rules.md#REQ-schema-058`、`#TBL-schema-011`、`#REQ-schema-030`、`#REQ-schema-032`。判断の記録は A12・A13・A20・A21・A22・A29・A30・A33・A35・A38。
Prerequisites: なし。
May change: `crates/kotowari-markdown-schema/src/`、`crates/kotowari-markdown-schema/tests/`、`crates/kotowari-markdown-schema/README.md`。
Done when: `reading` を書かないスキーマの結果がすべて今と同じで、`reading: line` のスキーマでは `TBL-schema-011` のすべての行（`#` だけの行、`#foo`、`文`の次の `---` と `===` の行を含む）の "line" の列のとおりに読み、`paragraph` と `line` 以外の値のスキーマで停止する。
Shown by: test — RED → GREEN → REFACTOR。`EX-schema-031`、`EX-schema-032`、`EX-schema-033`、`EX-schema-034`、`EX-schema-035`、`EX-schema-039`、`EX-schema-040`、`EX-schema-041` を1本ずつ（具体例の When が CLI を述べるものは `crates/kotowari-markdown-schema/tests/cli.rs` に）書き、各具体例の ID を印に付ける。`EX-schema-033` が見ない `#` だけの行と `===` の行を `reading: line` で`文`として読むことを見る1本を足す（`TBL-schema-011` の印）。`EX-schema-034` と `EX-schema-035` は今の実装で既に通る見込み（段落の読み方は変えない）で、そのときは RED を取らずに印を付けたテストとして置く。宣言を書かないスキーマの既存のテストがすべて緑のまま。
Left to the implementer: 読み分けを実装する場所（`validate` と `extract_values` の中の共通の補助など）。`reading` の値の型。
Stop and hand back if: `TBL-schema-010` の入口のどれかの署名を変えないと `line` の読み分けを `validate` と `extract_values` に届けられないと分かったとき。`TBL-schema-011` の行に当てはまらない行の形（どちらの列にも書かれていない形）に出会ったとき。

## Step 2 — 表の `select: first` と、余ったセル

Purpose: 表の規則が `select: first` でヘッダの合う最初の表だけを選べるようにし、ヘッダより多いセルを捨てる。Specification: `docs/ir/schema/block-rules.md#REQ-schema-059`、`#REQ-schema-033`、`docs/ir/schema/cli.md#TBL-schema-009`（`header` なしの `select` の停止）。判断の記録は A16・A27・A28・A42。
Prerequisites: なし。
May change: `crates/kotowari-markdown-schema/src/`、`crates/kotowari-markdown-schema/tests/`、`crates/kotowari-markdown-schema/README.md`。
Done when: `select: first` と `header` を宣言した表の規則は、その規則を置いた`ノード`の中でヘッダの合う最初の表だけをその規則の表にし、ほかの表を宣言の外の表にする。`select` を書かない表の規則の振る舞いは変わらない。`header` なしの `select` と、`first` 以外の値の `select` は停止する。セルがヘッダより多い行は指摘にならず余ったセルを捨て、少ない行は今のまま指摘になる。
Shown by: test — RED → GREEN → REFACTOR。`EX-schema-036`、`EX-schema-037`、`EX-schema-038`、`EX-schema-050` を1本ずつ。`EX-schema-037` の Then が `header` なしの `select` の停止を含むので、そのテストに `TBL-schema-009` の印も付ける。
Left to the implementer: `select` の値の型。
Stop and hand back if: 余ったセルを捨てると、既存のテストが固定している`抽出`の形（`TBL-schema-008` の表の行）が変わると分かったとき。

## Step 3 — 文書の直下の`項目`

Purpose: スキーマが `document.item` で、節を挟まずに文書の直下に置く`項目`を宣言できるようにする。Specification: `docs/ir/schema/schema-language.md#REQ-schema-016`、`#TBL-schema-004`、`docs/ir/schema/document-structure.md#REQ-schema-061`、`#REQ-schema-027`。判断の記録は A15・A23・A41。
Prerequisites: なし。
May change: `crates/kotowari-markdown-schema/src/`、`crates/kotowari-markdown-schema/tests/`、`crates/kotowari-markdown-schema/README.md`。
Done when: `document.item` を宣言したスキーマでは、最初の`節`より前の深さ3の見出しをその`項目`として読み、`前置部`を最初の`節`か`項目`の手前で終える。同じ文書に節の下の`項目`があってもよい。宣言しないスキーマでは今のまま、その見出しとその内側の行が指摘になる。
Shown by: test — RED → GREEN → REFACTOR。`EX-schema-043`、`EX-schema-044` を1本ずつ。`EX-schema-044` は今の実装で既に通る見込みで、そのときは RED を取らずに印を付けたテストとして置く。
Left to the implementer: なし。
Stop and hand back if: `document.item` の`項目`と節の下の`項目`の`抽出`を同じ`配置パス`に置くスキーマを、ステップ5の衝突の判定がどう扱うかで、`REQ-schema-061` と `TBL-schema-009` が食い違うと分かったとき（記録の A31 は別のパスに置く前提）。

## Step 4 — `導かれる値` `end`

Purpose: `項目`と`節`の最後の行を `end` で抽出できるようにする。Specification: `docs/ir/schema/extraction.md#REQ-schema-048`、`#REQ-schema-062`、`#TBL-schema-008`。判断の記録は A18・A24・A34。
Prerequisites: なし。
May change: `crates/kotowari-markdown-schema/src/`、`crates/kotowari-markdown-schema/tests/`、`crates/kotowari-markdown-schema/README.md`。
Done when: `end` は、次の同じ深さかそれより浅い見出し（見出しの判定は `TBL-schema-011` のとおり）の手前の行（無ければ文書の最後の行）で、空行を含み、コードブロックの中の見出しの形の行を見出しと数えない。`項目`と`節`の外に `end` を宣言したスキーマは停止する。`導かれる値`の語は5つになる。
Shown by: test — RED → GREEN → REFACTOR。`EX-schema-045`、`EX-schema-046` を1本ずつ。`EX-schema-024`（受けない語の停止）の Given が「5つのどれでもない語」に変わったので、印の付いた既存のテストの入力が `end` を使っていないかを確かめ、使っていれば別の語に替える。
Left to the implementer: なし。
Stop and hand back if: なし。見出しかどうかの判定は `TBL-schema-011` の`読み方`の列に従う（`reading: line` では `end` を決める見出しも1つ以上の "#" の直後に空白が続く行だけ）。

## Step 5 — 配置パスの衝突をスキーマ全体で判定する

Purpose: 要素オブジェクトの外（節の直下など）でも`配置パス`の衝突で停止する。Specification: `docs/ir/schema/cli.md#TBL-schema-009`、`#REQ-schema-042`。判断の記録は A11・A19。
Prerequisites: なし。
May change: `crates/kotowari-markdown-schema/src/`、`crates/kotowari-markdown-schema/tests/`。
Done when: 同じ置き場の中で同じパスか一方が他方の手前の段にあたる`配置パス`は、要素オブジェクトの中でも外でも停止になり、`"a.b"` と `"a.c"` のように先が分かれるものは停止にならない。要素オブジェクトの `"value"` と `"of"` の鍵も同じ置き場のパスとして数える（今の振る舞い）。
Shown by: test — RED → GREEN → REFACTOR。`EX-schema-047`、`EX-schema-048` を1本ずつ。`.mds/schemas/` の3つのスキーマが `parse_schema` を通ることを見る既存のテスト（`crates/kotowari-core/src/schema.rs` の `req_core_168_every_embedded_schema_parses`）が緑のまま。
Left to the implementer: なし。
Stop and hand back if: `.mds/schemas/` の今のスキーマが、広げた判定で停止すると分かったとき（スキーマの直し方は記録に無い）。

## Step 6 — 題名が複数あるときの指摘と、出現回数の指摘の規則種別

Purpose: 2つ目以降の題名ごとに multiple_titles を1件出す。出現回数の指摘が規則種別を持つことに印を付ける。Specification: `docs/ir/schema/document-structure.md#REQ-schema-022`、`docs/ir/schema/cli.md#REQ-schema-008`。判断の記録は A8・A17。
Prerequisites: なし。
May change: `crates/kotowari-markdown-schema/src/`、`crates/kotowari-markdown-schema/tests/`、`crates/kotowari-markdown-schema/README.md`。
Done when: 題名が3つある文書で multiple_titles が2件、それぞれその題名の行と`生の行`を持って出る。
Shown by: test — RED → GREEN → REFACTOR。`EX-schema-042` を1本。`EX-schema-049` は、出現回数の指摘に規則種別を付ける振る舞いが置き換えの工程で既に入っているので、その振る舞いを確かめる既存のテスト（`crates/kotowari-markdown-schema/src/validate.rs` の `repeat_min_not_met_carries_the_counted_rule_kind`）が具体例の Given・When・Then をそのまま確かめているなら印を足し、確かめていなければ具体例どおりのテストを1本書く。
Left to the implementer: なし。
Stop and hand back if: なし。

## Step 7 — 取り込むスキーマに宣言を書き、kotowari の`文`の読み方を戻す

Purpose: kotowari が取り込む3つのスキーマに新しい宣言を書き、`フィールド行`の直後の行、字下げした行、引用・HTML の行、節の無い `FLAGS.md` の`項目`を置き換えの前と同じに読む。Specification: `docs/ir/core/form-contract.md#REQ-core-179`、`docs/ir/core/ir-items.md#REQ-core-178`、`#TBL-core-011`。判断の記録は A1・A2・A4・A5・A13・A15・A20・A26・A28・A31・A32・A35・A38。
Prerequisites: ステップ1・2・3・4。
May change: `.mds/schemas/ir.yaml`、`.mds/schemas/context.yaml`、`.mds/schemas/flags.yaml`、`crates/kotowari-core/src/`、`tests/`。
Done when: 3つのスキーマが `REQ-core-179` の確かめ方に挙がった宣言をすべて持つ（`reading: line`、flags の文書の直下 `flags` と節の下 `flags_in_section`、context の `header` と `select: first`、ir と flags の`項目`の `end`）。kotowari は `flags` と `flags_in_section` の両方の`項目`を読んで行の順に並べる。上の Approach の2本のテストが置き換えの前の入力のまま通る。
Shown by: test — RED → GREEN → REFACTOR。`EX-core-273`、`EX-core-274`、`EX-core-275`、`EX-core-276` を1本ずつ（`REQ-core-178` の印を少なくとも1本に付ける）。`tbl_008_unclosed_backtick_detail_keeps_leading_indentation` と `req_098_review_requirement_without_how_to_verify_is_a_missing_field` を置き換えの前の入力に戻したものが緑。`REQ-core-179` は review の要求なので、確かめ方の文のとおりに3つのスキーマを読み、ステップ11の報告に結果を書く。
Left to the implementer: `flags` と `flags_in_section` を合わせる場所。
Stop and hand back if: `reading: line` にしたことで、このリポジトリの `docs/ir/` に `kotowari check` が新しい指摘を出すと分かったとき。

## Step 8 — 写しの表と用語集の表を戻す

Purpose: `TBL-core-030` の改訂（multiple_titles、用語集で宣言の外になった表、表の開始行の table_header_mismatch）と、用語集の表の選び方と4列以上の行を実装する。Specification: `docs/ir/core/finding-map.md#TBL-core-029`、`#TBL-core-030`、`#REQ-core-172`、`docs/ir/core/findings.md#REQ-core-174`、`#TBL-core-008`、`docs/ir/core/ir-document.md#REQ-core-035`、`docs/ir/core/terms-form.md#REQ-core-117`、`#REQ-core-122`。判断の記録は A3・A6・A7・A14・A16・A17・A25・A27・A36・A39・A40。
Prerequisites: ステップ2・6・7。
May change: `crates/kotowari-core/src/`、`tests/`。
Done when: `TBL-core-030` の各行（写し先の「出さない」を含む）が実装と一致する。「出さない」の行の指摘は停止せずに捨て、それ以外の行の無い種類と「発生しない」の行の種類は今のまま停止する。用語集の表は題名の後、最初の `## ` の見出しより前でヘッダの合う最初の表になり、4列以上の行は先頭の3列で`用語`になる。multiple_titles は2つ目以降の題名ごとに1件で、detail は `# ` を除き前後の空白を除いた題名の文字、`"line"` は null。
Shown by: test — RED → GREEN → REFACTOR。`EX-core-270`、`EX-core-271`、`EX-core-272`、`EX-core-278`、`EX-core-279` を1本ずつ。`crates/kotowari-core/src/finding_map.rs` の行ごとのテストのうち、改訂した行（multiple_titles、table_header_mismatch、undeclared_line の表）のテストを新しい写し先に合わせる。`req_035_multiple_titles`（`tests/step2_ir.rs`）の期待値は置き換えの前の "Second" に戻る。
Left to the implementer: なし。
Stop and hand back if: `TBL-core-030` に行の無いエンジンの種類、または行があるのに実物のエンジンが出さない種類が見つかったとき。

## Step 9 — query の本文の範囲をエンジンの `end` から作る

Purpose: `query` が生の行の見出しを探して本文の範囲を決める経路を無くす。Specification: `docs/ir/core/query.md#TBL-core-027`、`docs/ir/core/form-contract.md#REQ-core-169`、`#REQ-core-170`。判断の記録は A9・A18・A24・A34。
Prerequisites: ステップ4・7。
May change: `crates/kotowari-core/src/`、`tests/`。
Done when: `crates/kotowari-core/src/query.rs` の本文の範囲が`抽出`の `end` から決まり、kotowari のどのモジュールにも、`IR`の文書の生の行で `"### "` や `"## "` を探して範囲を決める処理が無い。`crates/kotowari-core/src/sources.rs` が`判断の記録`の生の行を `"## "` で読む経路は、`REQ-core-169` の対象（`IR`の文書）の外なので変えない。`シナリオ`の本文の範囲（タグの行から最後のステップの行まで）は変わらない。
Shown by: test — RED → GREEN → REFACTOR。`EX-core-277`、`EX-core-280` を1本ずつ。query の本文を確かめる既存のテストがすべて緑のまま。
Left to the implementer: `end` を`項目`に持たせる持ち方。
Stop and hand back if: `シナリオ`の本文の範囲にも生の行の見出しの探索が使われていると分かったとき（`REQ-core-169` の禁止と、シナリオの範囲の決め方の両方を満たす方法が仕様に無い）。

## Step 10 — スキルの references と PROJECT.md を今の振る舞いに合わせる

Purpose: スキルが配る説明と、リポジトリの案内が、この計画の後の振る舞いと一致する。Specification: `findings.md` の表は `docs/ir/core/skill-references.md#REQ-core-125`、`#REQ-core-127`。`PROJECT.md` は判断の記録の A10。`ir-form.md` の散文を定める要求は `docs/ir/` に無く、ステップ7・8で変えた振る舞いの要求（`REQ-core-178`、`TBL-core-011`、`REQ-core-117`、`REQ-core-122`、`REQ-core-035`）と利用者向けの説明を食い違わせないための追随である。
Prerequisites: ステップ8。
May change: `skills/kotowari/references/ir-form.md`、`skills/kotowari/references/findings.md`、`PROJECT.md`。
Done when: `ir-form.md` の FLAGS の置き場（節の有無を問わない）、multiple_titles、用語集の表の選び方と4列以上の行、`除外`の列挙（`用語集`の表以外の表）、`文`の読み方の記述が `docs/ir/core/` の要求と一致する。`findings.md` の multiple_titles の行が一致する。`PROJECT.md` の `mds check docs/ir/schema` の段落が、`docs/ir/schema/` も `kotowari check` だけで検査するという事実に書き換わっている。
Shown by: artifact — `skills/kotowari/references/ir-form.md`、`skills/kotowari/references/findings.md`、`PROJECT.md`。`REQ-core-125` と `REQ-core-127` の既存のテスト（`tests/step7_skill_references.rs`）が緑。
Left to the implementer: 文の言い回し。
Stop and hand back if: `docs/decision/records/ir-form.md` の`除外`の節も直さないと一致しないと分かったとき（判断の記録なので、直すには壁打ちに戻る）。

## Step 11 — 終端の確認

Purpose: 揃ったことと、このリポジトリの IR の検査結果が変わっていないことを見る。Specification: 上のすべて。
Prerequisites: ステップ1〜10。
May change: なし（直しが要れば当該のステップに戻る）。
Done when: 下の Shown by のすべてが通る。
Shown by: check — 次の順に走らせる。

1. `CARGO_BUILD_JOBS=4 cargo test --workspace` が全件通る
2. `CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format json` の終了コードが0で、`findings` のうち `"severity": "error"` のものが0件。`"severity": "notice"` のものは、記録の A37 で残すと決めた5件（`docs/ir/core/ir-items.md` の too_many_requirements、`docs/ir/schema/block-rules.md` と `docs/ir/schema/extraction.md` の too_many_lines と too_many_requirements）だけ
3. `CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- status --format text` の最後の行が `complete true` で、終了コードが0
4. `CARGO_BUILD_JOBS=4 cargo run -q -p kotowari-markdown-schema --bin mds -- check ./` の終了コードが0
5. `CARGO_BUILD_JOBS=4 cargo clippy --workspace --all-targets -- -D warnings` の `error` で始まる行が 14 行（着手前の実測）から増えていない。`crates/kotowari-markdown-schema/` を指すものは0
6. `CARGO_BUILD_JOBS=4 cargo +1.89.0 check -p kotowari-markdown-schema --all-targets` が通る

あわせて、ステップ7の `REQ-core-179` の確かめ方の結果を報告に書く。

Left to the implementer: なし。
Stop and hand back if: 2 で `docs/ir/` か `docs/decision/records/` のファイルに指摘が出たとき。

## Verification map

| 仕様の項目 | 確かめるステップ |
|---|---|
| REQ-schema-058、REQ-schema-060、TBL-schema-011、PROP-schema-008、REQ-schema-030、REQ-schema-032 | ステップ1 |
| REQ-schema-059、REQ-schema-033 | ステップ2 |
| REQ-schema-016、REQ-schema-061、TBL-schema-004 | ステップ3 |
| REQ-schema-048、REQ-schema-062、TBL-schema-008 | ステップ4 |
| REQ-schema-042、TBL-schema-009 | ステップ2、ステップ5 |
| REQ-schema-022、REQ-schema-008 | ステップ6 |
| REQ-core-178、REQ-core-179、TBL-core-011 | ステップ7、ステップ11 |
| TBL-core-029、TBL-core-030、REQ-core-172、REQ-core-174、TBL-core-008、REQ-core-035、REQ-core-117、REQ-core-122 | ステップ8 |
| TBL-core-027、REQ-core-169、REQ-core-170 | ステップ9 |
| REQ-core-125、REQ-core-127 | ステップ10 |
| EX-schema-031〜035、EX-schema-039〜041 | ステップ1 |
| EX-schema-036〜038、EX-schema-050 | ステップ2 |
| EX-schema-043、EX-schema-044 | ステップ3 |
| EX-schema-045、EX-schema-046 | ステップ4 |
| EX-schema-047、EX-schema-048 | ステップ5 |
| EX-schema-042、EX-schema-049 | ステップ6 |
| EX-core-273〜276 | ステップ7 |
| EX-core-270〜272、EX-core-278、EX-core-279 | ステップ8 |
| EX-core-277、EX-core-280 | ステップ9 |

## Left to the implementer

各ステップの Left to the implementer に書いたものだけ。モジュールの分け方、補助関数の抜き出し、テスト関数の名前（`mark.md` の慣習どおり、確かめる ID を先頭に付ける）はどのステップでも任せる。

## Stop conditions

- 仕様が黙っている振る舞いを決めないといけなくなったとき（新しい指摘の種類、新しい停止の理由、写し先の無いエンジンの種類、`TBL-schema-011` に無い行の形）は壁打ちに戻す
- エンジンのライブラリの入口6つ（`TBL-schema-010`）のどれかの署名を変えないと進めないと分かったとき
- `reading` を書かないスキーマの既存のテストが、エンジンの既定を変えた2件（余ったセル、題名の数）以外の理由で赤になったとき
- このリポジトリの `docs/ir/` と `docs/decision/records/` に `kotowari check` が新しい指摘を出したとき

## Test command

`CARGO_BUILD_JOBS=4 cargo test --workspace`。エンジンだけなら `CARGO_BUILD_JOBS=4 cargo test -p kotowari-markdown-schema`。`CARGO_BUILD_JOBS=4` はメモリの都合で、`lefthook.yml` の pre-commit と pre-push も同じ形で走らせている。

## Out of scope

- mds の版を上げること、変更履歴を書くこと（既定の振る舞いが2件変わるが、版の扱いは置き換えの判断の記録 A61 のとおりこの工程の要求にしない）
- CLI のバイナリの名前の変更
- 置き換えの工程の見直しで記録だけにした指摘（`Document` の行の保持量、`include_str!` がクレートの外を指すことなど）
