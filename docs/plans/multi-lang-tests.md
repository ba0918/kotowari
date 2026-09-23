# 計画: テストの見つけ方を ast-grep のルールに寄せ、TS/JS・Python・PHP に広げる

## Goal

ブランチ multi-lang-tests で、kotowari が ast-grep のルールでテストを見つけるようになり、Rust に加えて TypeScript、Tsx、JavaScript、Python、Php のテストに test_without_id を出し、`tests.rules` で使う人がルールを足せる。この計画が対象にする要求と具体例のすべてに、印のあるテストがある。

## Specification

- 判断の記録 `docs/decision/records/2026-09-24-multi-language-tests.md`（A1〜A49。A33〜A35 は却下）
- IR の置き場 `docs/ir`。新しく足した要求と具体例（まだテストが無い）:
  - `docs/ir/core/test-discovery.md#REQ-core-180`、`#REQ-core-181`、`#TBL-core-031`、`#EX-core-293`〜`#EX-core-297`
  - `docs/ir/core/test-queries.md#REQ-core-182`〜`#REQ-core-185`、`#TBL-core-032`〜`#TBL-core-034`、`#EX-core-298`〜`#EX-core-305`、`#EX-core-317`
  - `docs/ir/core/query-rules.md#REQ-core-186`〜`#REQ-core-189`、`#EX-core-311`〜`#EX-core-316`、`#EX-core-321`、`#EX-core-322`
  - `docs/ir/core/test-markers.md#TBL-core-035`、`#EX-core-306`〜`#EX-core-309`、`#EX-core-318`〜`#EX-core-320`
  - `docs/ir/core/coverage.md#EX-core-310`
- 意味が変わった既存の項目（テストはあるが、今の振る舞いを確かめているので直す）:
  - `docs/ir/core/test-discovery.md#REQ-core-080`、`#REQ-core-081`、`#REQ-core-083`
  - `docs/ir/core/test-queries.md#REQ-core-082`、`#TBL-core-017`（ファイルを移しただけ。マクロの中の関数の名前の一文を足した）
  - `docs/ir/core/query-rules.md#REQ-core-121`（「設定で問い合わせを足せない」から「同梱の問い合わせは外せない」に変わった）
  - `docs/ir/core/test-markers.md#REQ-core-072`、`#REQ-core-075`、`#TBL-core-016`（関数の本体の先頭のコメントの印を結び付けなくなった）
  - `docs/ir/core/ir-references.md#REQ-core-054`
  - `docs/ir/core/coverage.md#REQ-core-086`、`#EX-core-124`
  - `docs/ir/core/findings.md#TBL-core-008`（test_without_id の行）、`docs/ir/core/finding-order.md#TBL-core-019`（test_without_id の行）
  - `docs/ir/core/output.md#TBL-core-021`、`#EX-core-035`、`docs/ir/core/list.md#TBL-core-026`、`#EX-core-247`
  - `docs/ir/core/config.md#TBL-core-004`（`tests.rules` の行）
- レビューで確かめる要求: `docs/ir/core/test-discovery.md#REQ-core-084`（how_to_verify のとおり）

要求と具体例は `kotowari query <ID>` で読む。用語（`テスト`、`問い合わせ`、`問い合わせのある言語`、`直前のコメントの塊` など）は `docs/ir/core/CONTEXT.md`。

## Approach and why

今の `crates/kotowari-core/src/tests_discovery.rs` は tree-sitter-rust の構文木を手で辿って `#[test]` の関数を探し、兄弟の節を遡って印を集めている。これを次の3層に分ける。

1. 言語を決める層: 拡張子から言語を決める（TBL-core-031）。ast-grep-language の `SupportLang` の拡張子からの判定（`Language::from_path`）が TBL-core-031 と同じ対応を持つことを実測してある（記録の Context と A47）。
2. テストを見つける層: その言語の`問い合わせ`（同梱のルールと `tests.rules` のルール）を ast-grep で当て、当たった節を`テスト`、メタ変数 `$NAME` を名前にする（REQ-core-180、REQ-core-181）。Rust の `tests.rust.attributes` と `tests.rust.macros` は設定の形を変えずに残す（A12）。マクロの中身の読み直しだけは Rust のコードに残す（A1）。
3. 印を結び付ける層: 言語に依存しない行の規則にする（TBL-core-016、TBL-core-035）。テストの節の最初の行から上へ、コメントだけの行と挟んでよい行が続く間を`直前のコメントの塊`とし、その中の印だけを結び付ける。コメントの判定は tree-sitter の extra の節で行う（4言語で extra になることを実測済み。A16）。

層を分けるのは、言語を足す作業を「ルールを足す」だけにするため（A1、A2）。最初に Rust だけでエンジンを入れ替え、既存の Rust のテストが通ることを確かめてから言語を足す。入れ替えで既存の振る舞いが崩れたときに、原因を言語の追加と混ぜないため。

依存: `ast-grep-core`、`ast-grep-config`、`ast-grep-language`（A6、A46 のとおり既定の機能のまま27個の文法をすべて入れる）を足して実測した 0.45.3 に固定し、`tree-sitter` を 0.27 に上げ、`tree-sitter-rust` の直接の依存を外す（ast-grep-language の Rust の文法を使う）。2026-09-24 に ast-grep 0.45.3 でこの組み合わせがビルドでき、YAML のルールが読めて `$NAME` が取れ、`has_error` で構文の誤りが分かることを実測した。

## Scope of change

- `Cargo.toml`（ワークスペース）、`crates/kotowari-core/Cargo.toml`、`Cargo.lock`
- `crates/kotowari-core/src/tests_discovery.rs` とそこから分ける新しいモジュール、`crates/kotowari-core/src/config.rs`、`crates/kotowari-core/src/lib.rs`（テストの検査の呼び出しと停止の組み立て）。`TestMarker.name` はすでに `Option<String>` で、`list.rs` は null を扱える。型が変わるのは `DiscoveredTest.name`（今は `String`）と、それを test_without_id の detail にしている所（`tests_discovery.rs`）
- 同梱のルールのファイル（置き場所は実装者が決める。バイナリに埋め込む）
- `tests/`（とくに `step4_test_discovery.rs`、`step1_config.rs`、`step6_output.rs`、`step10_list.rs`、`step7_skill_references.rs`）
- `agent/skills/kotowari/references/mark.md`、`agent/skills/kotowari/references/config.md`（A30）
- `.kotowari/equivalents.yaml`（変異テストで等価と分かったものだけ。`agent/skills/kotowari/references/mutants.md` の手順のとおり）

変えないもの: `docs/ir/`、`docs/decision/`（承認済み）、`crates/kotowari-markdown-schema/`、`scripts/mutants.sh`、`.kotowari/config.yaml`。

## Step order and prerequisites

1 → 2 → 3 → 4 → 5 → 6 → 7。Step 1 がエンジンの入れ替えで、2〜5 はその上に乗る。2〜4（言語ごとの同梱のルール）は互いに独立だが、1つずつ終えてコミットする。5 は 2 の TypeScript のルールが無いと具体例が書けない（EX-core-312 が同梱の "it" のルールと重なることを確かめる）。

## Step 1 — エンジンを ast-grep に入れ替え、Rust の振る舞いを保つ

Purpose: 言語を決める層・テストを見つける層・印を結び付ける層を作り、Rust を同梱のルールで動かす。Specification: `docs/ir/core/test-discovery.md#REQ-core-080`、`#REQ-core-081`、`#TBL-core-031`、`#REQ-core-083`、`#REQ-core-180`、`#REQ-core-181`、`#EX-core-293`、`#EX-core-295`、`docs/ir/core/test-queries.md#REQ-core-082`、`#TBL-core-017`、`docs/ir/core/test-markers.md#REQ-core-072`、`#REQ-core-075`、`#TBL-core-016`、`#TBL-core-035`、`#EX-core-306`、`docs/ir/core/ir-references.md#REQ-core-054`、`docs/ir/core/coverage.md#REQ-core-086`、`#EX-core-124`、`docs/ir/core/findings.md#TBL-core-008`、`docs/ir/core/finding-order.md#TBL-core-019`、`docs/ir/core/output.md#TBL-core-021`、`#EX-core-035`、`docs/ir/core/list.md#TBL-core-026`、`#EX-core-247`。
Prerequisites: なし。
May change: `Cargo.toml`、`crates/kotowari-core/Cargo.toml`、`Cargo.lock`、`crates/kotowari-core/src/`（mds の crate は触らない）、`tests/`。
Done when:
- `tree-sitter-rust` が直接の依存から消え、ast-grep の3つの crate と tree-sitter 0.27 で全体がビルドできる。
- 既存の Rust のテストの検出と印の結び付けのテストが通る。ただし関数の本体の先頭のコメントの印を結び付けることを確かめていたテストは、EX-core-306 のとおり結び付かないことを確かめる形に書き換える（TBL-core-016 の変更。`kotowari query TBL-core-016` の `tests` にあるテストを1本ずつ読み、本体の先頭の印に頼っているものを探す）。
- `.go` のような同梱の`問い合わせ`の無い言語のファイルは構文木を読まない（EX-core-293）。`.PY` は言語が決まらない（EX-core-295）。
- `.py` を`問い合わせの無い言語`の例として使っている既存のテストの fixture を、すべて `.go` に変える（`rg -n '\.py' tests/` で探す）。Step 3 で `.py` が`問い合わせのある言語`になると、落ちるか意味が変わるため。少なくとも次がある: EX-core-035（`tests/step6_output.rs`）、EX-core-247 と `req_155_text_writes_dash_for_a_null_test_name`（`tests/step10_list.rs`）、EX-core-124、`req_081_only_rs_maps_to_rust`（TBL-core-031 を確かめる形に直す）、`tbl_010_lone_cr_ends_a_line_of_a_test_file`、`req_076_*`、`req_087_unknown_language_only_feeds_coverage`、`req_072_non_query_language_checks_invalid_marker`、`req_054_non_query_language_checks_unresolved_reference`、`req_077_*_non_rs`（`tests/step4_test_discovery.rs`）。REQ-core-076 と REQ-core-087 の`問い合わせの無い言語`の枝を確かめるテストが残ること。
- "query" は言語に`問い合わせ`があるかで決まる（TBL-core-021。この時点では "rs" だけ true でよい）。
- test_without_id の "line" は`テスト`の節の最初の行、detail は名前。名前が null のときの detail は、名前を捕まえないルールが要るので Step 5（EX-core-310）で確かめる。
- このリポジトリ自身の `kotowari check` で、テストのファイルに新しい test_without_id、invalid_marker、unresolved_reference が出ない（本体の先頭の印に頼っていた自身のテストがあれば、印を関数の直前に移す）。
Shown by: test — `CARGO_BUILD_JOBS=4 cargo test --workspace`。EX-core-306 のテストは入れ替えの前に書いて落ちることを確かめる（RED）。EX-core-293 と EX-core-295 は今のコードでも通る（今は "rs" だけを読む）ので、入れ替えで壊れないことを確かめる回帰として書く。EX-core-295 は Step 3 の後にも通ることを確かめる。既存のテストは入れ替えの後も通ること（GREEN）を報告に載せる。最後に `CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format json | jq '.counts'` の前後を比べる。
Left to the implementer: モジュールの分け方と名前、同梱のルールの置き方（YAML のファイルを埋め込むか、コードで組み立てるか）、`#[test]` の末尾の要素の判定と `tests.rust.attributes` をルールで書くかコードで書くか（TBL-core-017 の振る舞いが同じなら可）、コメントだけの行の判定の書き方。
Stop and hand back if:
- ast-grep の API で、当たった節から tree-sitter の節（extra の判定、`has_error`、行と列）に届かない。
- `tests.rust.macros` のマクロの中身を tree-sitter 0.27 の Rust の文法で読み直すと、今のテスト（EX-core-017、EX-core-018 など）の結果が変わる。
- 既存のテストのうち、本体の先頭の印以外の理由で落ちるものがあり、仕様（TBL-core-016、TBL-core-035）のどの行に当たるか決められない。
- `Language::from_path` の拡張子の対応が TBL-core-031 のどれかの行と食い違う（IR は変えない。表と実装のどちらに合わせるかは利用者が決める）。

## Step 2 — TypeScript、Tsx、JavaScript の同梱のルール

Purpose: TS/JS のテストを数える。Specification: `docs/ir/core/test-queries.md#REQ-core-182`、`#REQ-core-183`、`#TBL-core-032`、`#EX-core-298`〜`#EX-core-301`、`#EX-core-317`、`docs/ir/core/test-discovery.md#EX-core-294`、`#EX-core-296`、`#EX-core-297`、`docs/ir/core/test-markers.md#EX-core-309`、`#EX-core-319`、`#EX-core-320`。
Prerequisites: Step 1。
May change: 同梱のルール、`crates/kotowari-core/src/`、`tests/`。
Done when: 上の具体例それぞれに印のあるテストがあり、通る。TypeScript と Tsx の両方に同じ中身のルールがある（REQ-core-182）。"query" が ts、mts、cts、tsx、js、jsx、mjs、cjs で true になる。
Shown by: test — `CARGO_BUILD_JOBS=4 cargo test --workspace`。各テストはルールを足す前に落ちることを確かめる。
Left to the implementer: ルールの書き方（TBL-core-032 の行を満たすこと。"it.each(表)" の内側を数えないことも含む）。
Stop and hand back if: TBL-core-032 のどれかの行が ast-grep のルールで書けず、コードで補う必要がある。

## Step 3 — Python の同梱のルール

Purpose: Python のテストを数える。Specification: `docs/ir/core/test-queries.md#REQ-core-184`、`#TBL-core-033`、`#EX-core-302`、`docs/ir/core/test-markers.md#EX-core-307`、`#EX-core-308`、`#EX-core-318`。
Prerequisites: Step 1。
May change: 同梱のルール、`crates/kotowari-core/src/`（挟んでよい行の Python のデコレータ）、`tests/`。
Done when: 上の具体例それぞれに印のあるテストがあり、通る。デコレータが複数行でも、コメントとデコレータが混ざっても印が結び付く（TBL-core-035）。"query" が py、py3、pyi、bzl、bazel で true になる。
Shown by: test — `CARGO_BUILD_JOBS=4 cargo test --workspace`。各テストはルールを足す前に落ちることを確かめる。
Left to the implementer: ルールの書き方（ファイルの最上位の関数とクラスの中のメソッドだけで、関数の中の関数を数えない）。
Stop and hand back if: クラスの中のメソッドの直前のコメント（構文木ではクラスの子に入る）が、行の規則で結び付かない。

## Step 4 — Php の同梱のルール

Purpose: PHP のテストを数え、同梱の問い合わせをそろえる。Specification: `docs/ir/core/test-queries.md#REQ-core-182`、`#REQ-core-185`、`#TBL-core-034`、`#EX-core-303`〜`#EX-core-305`。
Prerequisites: Step 1。
May change: 同梱のルール、`tests/`。
Done when: 上の具体例それぞれに印のあるテストがあり、通る。"#[Test]" の付いたメソッドの test_without_id の "line" は "#[Test]" の行（TBL-core-019、A32）。同梱の6言語がそろい（REQ-core-182）、"query" が rs、ts、mts、cts、tsx、js、jsx、mjs、cjs、py、py3、pyi、bzl、bazel、php で true になる。
Shown by: test — `CARGO_BUILD_JOBS=4 cargo test --workspace`。各テストはルールを足す前に落ちることを確かめる。
Left to the implementer: ルールの書き方。"@test" の docblock はメソッドの直前のコメントの節で判定する。
Stop and hand back if: "#[\PHPUnit\Framework\Attributes\Test]" のようなパスの付いた属性の最後の要素が、ルールで取り出せない。

## Step 5 — `tests.rules` で足すルール

Purpose: 使う人がルールを足せるようにする。Specification: `docs/ir/core/query-rules.md#REQ-core-121`、`#REQ-core-186`〜`#REQ-core-189`、`#EX-core-311`〜`#EX-core-316`、`#EX-core-321`、`#EX-core-322`、`docs/ir/core/config.md#TBL-core-004`、`docs/ir/core/coverage.md#EX-core-310`。
Prerequisites: Step 2。
May change: `crates/kotowari-core/src/config.rs`、`crates/kotowari-core/src/`、`tests/`（とくに `step1_config.rs`）。
Done when:
- 上の具体例それぞれに印のあるテストがあり、通る。
- 今 REQ-core-121 の印を持つテスト（`tests/step1_config.rs` の「設定で問い合わせを足せない」を確かめるもの）を、新しい REQ-core-121（同梱の問い合わせは外せない）を確かめる形に書き換える。
- 名前を捕まえないルールの`テスト`の detail が、節の最初の行の全体から前後の空白を除いたものになる（EX-core-310、REQ-core-086）。
- `tests/step1_config.rs` の定数 `ALL_NINE_KEYS` に `rules: []` を足し、定数名と assert の文言を TBL-core-004 の鍵の数に合わせる。
- 停止の理由は既存の設定の誤りと同じ（`停止`の理由の文言は `kotowari query REQ-core-014` と TBL-core-018 のとおり）。
Shown by: test — `CARGO_BUILD_JOBS=4 cargo test --workspace`。各テストは実装の前に落ちることを確かめる。
Left to the implementer: ルールのファイルの読み方、停止の詳細の文字列（既存の設定の誤りの詳細の書き方に合わせる）。
Stop and hand back if: ast-grep-config が "files" と "ignores" を ast-grep と同じ読み方で当てる手段を持たず、glob の読み方を自前で決めないといけない（A45 は ast-grep と同じ読み方と決めている）。"language" の別名（"ts"、"py"）を ast-grep-config が受けない。

## Step 6 — スキルの references を直す

Purpose: スキル kotowari の references を本体と一致させる。Specification: 記録 #A30、`docs/ir/core/skill-references.md#REQ-core-126`。
Prerequisites: Step 5。
May change: `agent/skills/kotowari/references/mark.md`、`agent/skills/kotowari/references/config.md`、`tests/step7_skill_references.rs`。
Done when:
- `config.md` の設定の表と setup の手順1の YAML に `tests.rules`（既定は空の一覧）がある。`tests/step7_skill_references.rs` の REQ-core-126 のテストの鍵の一覧（今は9本を書き並べている）に `tests.rules` を足し、config.md を直す前に落ちることを確かめる（`Config::parse` は書かれていない鍵を既定で埋めるので、一覧に足さないと古い config.md でも通ってしまう）。
- `mark.md` の「Where marks may go」が`直前のコメントの塊`の規則（本体の先頭の印は結び付かない、Rust の属性と Python のデコレータは挟んでよい）になり、「How tests are recognised」と「Languages other than Rust」が同梱の4言語と `tests.rules` の説明になる。英語で書く（references は英語）。
Shown by: test — `CARGO_BUILD_JOBS=4 cargo test --workspace`（REQ-core-126 のテスト）。`mark.md` は Step 7 のレビューで読む。
Left to the implementer: 文面。
Stop and hand back if: なし。

## Step 7 — 仕上げの確認

Purpose: この計画の対象がすべてテストで押さえられ、ゲートを通ることを示す。Specification: 上のすべて、`docs/ir/core/test-discovery.md#REQ-core-084`。
Prerequisites: Step 1〜6。
May change: `.kotowari/equivalents.yaml`、`tests/`（変異の見逃しを埋めるテスト）。
Done when:
- この計画の Specification に挙げた要求と具体例のすべてで、`kotowari query <ID> | jq '.items[0].tests | length'` が1以上（REQ-core-084 は review なので対象外）。
- `kotowari check --format json` の `findings` に、このブランチが変えたファイル（`git diff --name-only main`）か、この計画の ID を detail に持つ誤りが無い。
- `lefthook run pre-push --no-auto-install` が通る（全テスト、`kotowari check`、変異テスト）。変異の見逃しは `agent/skills/kotowari/references/mutants.md` の手順で、テストを足すか等価として記録する。
- REQ-core-084 の how_to_verify のとおり、テストを見つけるコードが行に正規表現を当てていないことをレビューで確かめる。
- release ビルドの `kotowari` の大きさを報告に載せる（記録 A6 の実測では文法を全部入れると 45.8MB 前後）。
Shown by: check — REQ-core-084 は、テストを見つけるモジュールで正規表現を使っている箇所を列挙し、how_to_verify に照らした結果を報告に載せる。そのうえで上の順に、`CARGO_BUILD_JOBS=4 cargo test --workspace`、`kotowari query` の繰り返し、`kotowari check --format json | jq`、`lefthook run pre-push --no-auto-install`、`CARGO_BUILD_JOBS=4 cargo build --release -p kotowari && ls -l target/release/kotowari`。
Left to the implementer: なし。
Stop and hand back if: 変異テストの件数が多すぎて pre-push が現実的な時間で終わらない（差分が大きいので件数が増える。走らせる前に `cargo mutants --list --in-diff` で件数を見て、千件を超えるなら走らせる前に利用者に伝える）。

## Verification map

| Step | 確かめる ID |
|---|---|
| 1 | REQ-core-076、REQ-core-087、REQ-core-080、REQ-core-081、TBL-core-031、REQ-core-083、REQ-core-180、REQ-core-181、REQ-core-082、TBL-core-017、REQ-core-072、REQ-core-075、TBL-core-016、TBL-core-035、REQ-core-054、REQ-core-086、TBL-core-008、TBL-core-019、TBL-core-021、TBL-core-026、EX-core-293、EX-core-295、EX-core-306、EX-core-124、EX-core-035、EX-core-247 |
| 2 | REQ-core-183、TBL-core-032、EX-core-294、EX-core-296〜EX-core-301、EX-core-309、EX-core-317、EX-core-319、EX-core-320 |
| 3 | REQ-core-184、TBL-core-033、EX-core-302、EX-core-307、EX-core-308、EX-core-318 |
| 4 | REQ-core-182、REQ-core-185、TBL-core-034、EX-core-303〜EX-core-305 |
| 5 | REQ-core-121、REQ-core-186〜REQ-core-189、TBL-core-004、EX-core-310〜EX-core-316、EX-core-321、EX-core-322 |
| 6 | REQ-core-126（既存） |
| 7 | REQ-core-084（review）と全体 |

## Left to the implementer

- モジュールの分け方、関数と型の名前、同梱のルールのファイルの置き場所と書き方。
- テストの組み方（一時ディレクトリにテストのファイルと設定を置く既存のテストの形に合わせる）と名前。名前は `kotowari` の慣習（検証する ID を小文字にして先頭に置く）に合わせる。

## Stop conditions

- 仕様（IR、記録）に無い振る舞いを決めないと進めない。
- 取り消せない操作（push、依存の公開）が要る。push は利用者が決める。
- 直しが別の場所の失敗を連鎖して起こし、止まらない。
- やり方を変えても進まない。

## Test command

`CARGO_BUILD_JOBS=4 cargo test --workspace`（`PROJECT.md` のとおり）。`cargo fmt --all` は打たない（ワークスペースが rustfmt 済みでなく30ファイル以上書き換わる）。自分が書いた行だけ `rustfmt --edition 2024 --check <file>` で見て手で合わせる。実時間を待つテストを書かない（`PROJECT.md` の変異テストの節）。

## Out of scope

- 既定のルールを同梱する言語を Rust、TS/JS、Python、PHP 以外に増やすこと（A8）。
- `tests.files` の既定を変えること（A5）。
- `crates/kotowari-markdown-schema/`。
- 照合で見つかった、今回より前からある文言の裏付けの不足（用語「除外」、TBL-core-008 の id_domain_mismatch の行）。
- `TODO.md` の tree-sitter-rust の読み違いの項目（文法が 0.24 になって直るかは確かめていない。直っていても、この計画では TODO を触らない）。
