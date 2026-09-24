# Plan: ガイドの印で古くなった節を見つける

## Goal

利用者向けの使い方の文書（ガイド）の HTML コメントに書いた `@kotowari[ID:指紋]` を `kotowari check` と `kotowari status` が読み、IR の今の指紋と違う印に guide_stale の注意を出し、`kotowari list` と `kotowari query` が各項目の指紋を出すようになる。

## Specification

IR は `docs/ir/` にある。決定は `docs/decision/records/2026-09-24-doc-marks.md`。この計画が扱うのは次のとおり。

- 新しい要求: `docs/ir/core/guides.md#REQ-core-198` から `#REQ-core-205` まで、`docs/ir/core/output.md#REQ-core-206`
- 新しい決定表: `docs/ir/core/guides.md#TBL-core-036`
- 変わった既存の項目: `docs/ir/core/config.md#REQ-core-014`、`#REQ-core-018`、`#TBL-core-004`、`docs/ir/core/findings.md#REQ-core-031`、`#TBL-core-008`、`#TBL-core-009`、`docs/ir/core/list.md#REQ-core-152`、`#TBL-core-026`、`docs/ir/core/query.md#REQ-core-158`、`docs/ir/core/status.md#REQ-core-162`、`#TBL-core-028`、`docs/ir/core/output.md#TBL-core-005`、`#TBL-core-006`、`docs/ir/core/cli.md#TBL-core-001`、`docs/ir/core/cli-environment.md#TBL-core-020`、`docs/ir/core/finding-order.md#TBL-core-019`
- シナリオ: EX-core-362 から EX-core-375 まで（すべて `docs/ir/core/guides.md`）

要求の本文は `kotowari query <ID>` で読む（例: `kotowari query REQ-core-204 | jq -r '.items[0].body[]'`）。用語（ガイド、ガイドの印、指紋、除外）は `docs/ir/core/CONTEXT.md` にある。

## Approach and why

- ガイドは `load_all`（`crates/kotowari-core/src/lib.rs`）の中で読まない。`load_all` は list と query も通るが、REQ-core-152 と REQ-core-158 は list と query がガイドを読まず、ガイドが原因で停止しないと決めている。ガイドの読み込み・印の検査・照合は、`run_check` と `run_status` だけが呼ぶ別の関数にする。
- 指紋は query の本文の切り出し（`crates/kotowari-core/src/query.rs` の `body_of`）を土台にする。REQ-core-203 が指紋の範囲を TBL-core-027 の body から決めているので、本文の切り出しを新しく作ると2つが食い違う。list も指紋を出すので、`body_of` は list と query の両方から使える場所に置く。`項目`は `body_of` の結果（先頭と末尾の空の行を除いた後）から "- source:" の出典の行を除き、その後で空の行を除き直さない。REQ-core-203 は body から出典の行を除いた並びと言い、除き直すとは言っていない。`シナリオ`は `Item::Scenario` の steps の行（ステップの行だけで、注釈の行と空の行とタグの行と "Scenario:" の行を含まない）を使う。
- ガイドのファイルの走査は、テストのファイルの走査（`crates/kotowari-core/src/tests_discovery.rs` の `collect_test_files`）と同じ規則（REQ-core-198 が REQ-core-019、REQ-core-079、REQ-core-018 を指す）にする。glob の一覧を引数に取る形にして、テストとガイドの両方から使う。規則を2か所に持つと、片方だけ直す事故が起きる。走査を共有するので、読めないディレクトリと先の無いシンボリックリンクでの停止は既存のテストの走査のテストが確かめていて、ガイドの側で同じテストを重ねない。
- ガイドは CommonMark として読む（REQ-core-200）。Markdown の読み方は mds のエンジンに揃える（決定 A9）ので、`kotowari_markdown_schema::ast::parse_mdast` でガイドを読む。戻り値の `markdown::mdast::Node` を match するため、`crates/kotowari-core/Cargo.toml` に `markdown` クレートを mds と同じ版（"1"）で足す。mdast の `Html` の節（ブロックの形と、段落の中のインラインの形の両方）からコメントを取り出し、`Code`（字下げの形とフェンスの形）と `InlineCode` の節は見ない。frontmatter の節（`parse_mdast` が読む）も `Html` ではないので見ない。
- SHA-256 には `sha2` クレートを使う（Cargo.lock に 0.10 が既にあり、`crates/kotowari-markdown-schema` が直接依存している）。`crates/kotowari-core/Cargo.toml` に直接の依存として足す。
- 変わるのは kotowari の skill（`agent/skills/kotowari/`）も同じ。`tests/step7_skill_references.rs` が、skill の指摘の種類の表とコードの種類の一致（REQ-core-125）、setup の YAML と既定の値の一致（REQ-core-126）を確かめている。guide_stale と "guides.files" を足すステップで、skill の表と YAML も同時に直さないと、そのテストが落ちる。

## Scope of change

- `crates/kotowari-core/Cargo.toml`
  - `sha2` と `markdown` を足すだけ
- `crates/kotowari-core/src/config.rs`
- `crates/kotowari-core/src/lib.rs`
  - `CheckResult`、`run_check`、`run_status`、停止の文言、指摘の種類の一覧（`finding_kinds!`）と重大度（`FindingKind::severity`）。`load_all` にはガイドを足さない
- `crates/kotowari-core/src/query.rs`、`crates/kotowari-core/src/list.rs`
- `crates/kotowari-core/src/status.rs`
- `crates/kotowari-core/src/tests_discovery.rs`
  - 走査の関数を glob の一覧を受ける形にするところだけ
- `crates/kotowari-core/src/guides.rs`（新しく作る。名前は変えてよい）
- `tests/step15_guides.rs`（新しく作る）と、既存のテストのうち list、query、status、check の JSON の鍵を固定しているもの
- `tests/step7_skill_references.rs`
  - req_126 が YAML に書かれていることを確かめる鍵の経路の一覧に "guides.files" を足すところだけ
- `agent/skills/kotowari/SKILL.md`、`agent/skills/kotowari/references/config.md`、`agent/skills/kotowari/references/findings.md`、`agent/skills/kotowari/references/guides.md`（新しく作る）

## Step order and prerequisites

S1 が設定の鍵を足し、S3 以降がそれを読む。S2 の指紋は S5 の照合が使う。S3 がガイドのファイルを集め、S4 がその中の印を読み、S5 が読んだ印を照合する。この順でないと、各ステップのテストが前のステップの出力を持たない。S6 の skill は、S1 から S5 で決まった振る舞いを書くので最後に近い。S7 は全体を確かめる。

## Verification map

| Step | Requirements | Examples |
|---|---|---|
| S1 | REQ-core-014、TBL-core-004 | なし |
| S2 | REQ-core-203、TBL-core-026 | EX-core-372 |
| S3 | REQ-core-198、REQ-core-199、REQ-core-206、REQ-core-018、REQ-core-152、REQ-core-158、REQ-core-163、TBL-core-001、TBL-core-005、TBL-core-020 | EX-core-368、EX-core-369 |
| S4 | REQ-core-200、REQ-core-201、REQ-core-202、REQ-core-206、TBL-core-036、TBL-core-008、TBL-core-019 | EX-core-366、EX-core-367、EX-core-373、EX-core-374 |
| S5 | REQ-core-204、REQ-core-031、REQ-core-162、TBL-core-006、TBL-core-009、TBL-core-019、TBL-core-028 | EX-core-362、EX-core-363、EX-core-364、EX-core-365、EX-core-370、EX-core-371、EX-core-375 |
| S6 | REQ-core-205（review） | なし |
| S7 | この表のすべて | この表のすべて |

## Left to the implementer

- 新しいモジュールと関数と型の名前、テストの関数の名前
- `body_of` を移す先のモジュール
- mdast の HTML の節からコメントと行番号を取り出す方法（節の値の中で "<!--" と "-->" を探すか、別の方法か）。結果が REQ-core-200 と EX-core-374 を満たすこと

## Stop conditions

- `markdown` クレートの mdast で、HTML のコメントの行番号が取れない、またはコードスパンと HTML を区別できない
- 既存のテストが、この計画の変更と関係の無い理由で落ちる
- REQ-core-198 が指す走査の規則を、テストとガイドで共有すると既存のテストの振る舞いが変わる
- 仕様の要求とシナリオが食い違っていて、どちらかを直さないと両方を満たせない

## Test command

```sh
CARGO_BUILD_JOBS=4 cargo test --workspace
```

## Out of scope

- ガイドそのもの（試作の4本）をこのリポジトリに入れることと、このリポジトリの `.kotowari/config.yaml` に "guides.files" を書くこと（決定 A19）
- 決定の記録の番号への印（決定 R1）
- 照合で見つかった、前からある出典の漏れ（TBL-core-008 と TBL-core-019 の id_domain_mismatch、用語集の除外の奇数の二重引用符）
- kotowari 以外の skill（kotowari-brainstorm、kotowari-cycle など）への追記

## Steps

### S1: 設定に guides.files の鍵を足す

- Purpose: 設定ファイルが "guides.files" を glob の一覧として受け、既定を空の一覧にし、glob として読めない要素で設定の誤りとして停止するようにする
- Specification: `docs/ir/core/config.md#REQ-core-014`, `docs/ir/core/config.md#TBL-core-004`
- Prerequisites: none
- May change: `crates/kotowari-core/src/config.rs`, `agent/skills/kotowari/references/config.md`, `tests/step1_config.rs`, `tests/step7_skill_references.rs`, `tests/step15_guides.rs`
- Done when: "guides.files" を書いた設定が読め、書かない設定では空の一覧になり、glob として読めない要素で終了コード 2 と config error になり、`tests/step7_skill_references.rs` の req_126 の鍵の経路の一覧に "guides.files" があって req_126 が通る
- Shown by: test — "guides.files" の既定、読める値、読めない glob の3つのテストと、既存の req_126_config_reference_setup_yaml_parses_to_the_defaults
- Left to the implementer: none
- Stop and hand back if: 設定の鍵の検査が、"tests.files" の glob の検査を共有できない形になっている

### S2: 項目とシナリオの指紋を計算して list と query に出す

- Purpose: 項目とシナリオの指紋を REQ-core-203 のとおりに計算し、list と query の1件に "fingerprint" の鍵で出す
- Specification: `docs/ir/core/guides.md#REQ-core-203`, `docs/ir/core/list.md#TBL-core-026`
- Prerequisites: none
- May change: `crates/kotowari-core/Cargo.toml`, `crates/kotowari-core/src/query.rs`, `crates/kotowari-core/src/list.rs`, 新しいモジュール, `tests/step10_list.rs`, `tests/step11_query.rs`, `tests/step15_guides.rs`
- Done when: EX-core-362 の IR で `kotowari list` の "REQ-001" の "fingerprint" が "51b1f3da" になり、決定表と、注釈の行を含むシナリオの "fingerprint" が Approach のとおりの行の並びから `sha256sum` で手で計算した値と等しく、見出しの名前と "- source:" の行を変えても、シナリオの "Scenario:" の行とタグの行を変えても list の値が変わらず、行の終わりが "\r\n" の文書でも同じ値になる
- Shown by: test — EX-core-372、決定表とシナリオの値を固定したテスト、名前と出典とタグを変えても list の値が変わらないテスト（印は REQ-core-203。EX-core-364 と EX-core-375 は check で確かめるので S5 で印を付ける）、"\r\n" の文書で指紋が変わらないテスト
- Left to the implementer: `body_of` を置き直す場所
- Stop and hand back if: "- source:" の行を、字下げや値の形で見分けられない本文がある

### S3: ガイドのファイルを集め、置き場の重なりで停止し、数を出す

- Purpose: check と status で "guides.files" のファイルを集め、テストの置き場との重なりで停止し、check の JSON に "guides" の "files" を出す（"marks" はこの段では 0 を出し、S4 で数える）。list と query はガイドを読まない
- Specification: `docs/ir/core/guides.md#REQ-core-198`, `docs/ir/core/guides.md#REQ-core-199`, `docs/ir/core/output.md#REQ-core-206`, `docs/ir/core/config.md#REQ-core-018`, `docs/ir/core/list.md#REQ-core-152`, `docs/ir/core/query.md#REQ-core-158`, `docs/ir/core/status.md#REQ-core-163`, `docs/ir/core/cli.md#TBL-core-001`, `docs/ir/core/output.md#TBL-core-005`, `docs/ir/core/cli-environment.md#TBL-core-020`
- Prerequisites: S1
- May change: `crates/kotowari-core/src/tests_discovery.rs`, `crates/kotowari-core/src/lib.rs`, 新しいモジュール, `tests/step6_output.rs`, `tests/step15_guides.rs`
- Done when: EX-core-368 で check も status も終了コード 2 になって停止の詳細が "docs/guide.md" の相対パスそのものを含み、重なるファイルが2つあるときはパスのバイト順で最初の1つだけが出て、EX-core-369 で "guides" の "files" と "marks" が 0 になり、UTF-8 でないガイドで non-UTF-8 file の停止になり、"guides.files" が IR の置き場のファイルに当たっても停止せず、同じ重なりの設定でも `kotowari list` と `kotowari query` は停止しない
- Shown by: test — EX-core-368（check と status）、重なりが2つのテスト、EX-core-369、UTF-8 でないガイドのテスト、IR の置き場に当たる glob のテスト、重なりの設定で list と query が停止しないテスト
- Left to the implementer: none
- Stop and hand back if: 走査を共有すると、既存のテストの走査の振る舞い（読む順、除外）が変わる

### S4: ガイドの印を読み、形の誤りを invalid_marker にする

- Purpose: ガイドの HTML コメントの中の印だけを読み、1件ずつ ID と指紋に分け、形の誤った印に invalid_marker を出し、形の正しい1件を "marks" に数える
- Specification: `docs/ir/core/guides.md#REQ-core-200`, `docs/ir/core/guides.md#REQ-core-201`, `docs/ir/core/guides.md#REQ-core-202`, `docs/ir/core/guides.md#TBL-core-036`, `docs/ir/core/findings.md#TBL-core-008`, `docs/ir/core/finding-order.md#TBL-core-019`
- Prerequisites: S3
- May change: `crates/kotowari-core/Cargo.toml`, 新しいモジュール, `crates/kotowari-core/src/lib.rs`, `tests/step15_guides.rs`
- Done when: 形の正しい1件を "marks" に数え、EX-core-366 でコードブロックの中とコメントの外の並びを読まず、段落の中のインラインのコメントの印と、1つのコメントに2つある印をどちらも読み、中が空の印と閉じ括弧が同じ行に無い印に detail が行の文字の invalid_marker が出て、EX-core-367 で3行目と5行目に invalid_marker が出て終了コードが 1 になり、EX-core-373 で1件だけ誤った印のどの1件も照合せず "marks" が 0 になり、EX-core-374 で括弧の内側の空白を許して "-->" の後ろの並びを読まない
- Shown by: test — EX-core-366、EX-core-367、EX-core-373、EX-core-374、インラインのコメントと1つのコメントに2つの印のテスト、空の印と閉じない印のテスト。guide_stale が出ないことを確かめるシナリオでは、指摘の kind の文字列 "guide_stale" が無いことを確かめ、S5 の後も通るようにする
- Left to the implementer: `Html` の節の値の中から "<!--" と "-->" の組と行番号を取り出す方法
- Stop and hand back if: `markdown` クレートが HTML ブロックの行番号を返さない、または字下げの形のコードブロックと HTML ブロックを区別しない

### S5: 印を照合して guide_stale を出し、status に数を出す

- Purpose: 形の正しい印の1件を IR の今の指紋と照らし、違えば guide_stale の注意を出し、status に "guides" の群を出す
- Specification: `docs/ir/core/guides.md#REQ-core-204`, `docs/ir/core/findings.md#REQ-core-031`, `docs/ir/core/findings.md#TBL-core-009`, `docs/ir/core/output.md#TBL-core-006`, `docs/ir/core/finding-order.md#TBL-core-019`, `docs/ir/core/status.md#REQ-core-162`, `docs/ir/core/status.md#TBL-core-028`
- Prerequisites: S2, S4
- May change: 新しいモジュール, `crates/kotowari-core/src/lib.rs`, `crates/kotowari-core/src/status.rs`, `agent/skills/kotowari/references/findings.md`, `tests/step12_status.rs`, `tests/step15_guides.rs`
- Done when: EX-core-362 で指摘が出ずに "marks" が 1 になり、EX-core-363 で detail が "REQ-001 51b1f3da " で始まる guide_stale が出て終了コードが 0 のまま status の "complete" が true になり、EX-core-365 で detail が "REQ-009 51b1f3da -" になり、EX-core-364 と EX-core-375 で名前と出典とタグだけを変えても check が guide_stale を出さず、EX-core-370 で2つ目の項目と一致すれば出ず、同じ ID が2か所にあってどちらとも一致しないとき detail の3つ目が1つ目の項目の指紋になり、EX-core-371 で "EX-001" の1件だけが出て、guide_stale の line が印の始まりの行になり、status の text に "guides files=1 marks=1" の形の行が出て、`tests/step7_skill_references.rs` の req_125 が通る
- Shown by: test — EX-core-362、EX-core-363、EX-core-364、EX-core-365、EX-core-370、EX-core-371、EX-core-375、同じ ID の2か所がどちらとも一致しないテスト、status の "guides" の群のテスト、既存の req_125_findings_reference_kinds_match_the_code。終了コード 0 を確かめるテストの組み立てには、requirement_without_test の誤りが出ないよう、要求とシナリオに印を付けたテストのファイルを足す
- Left to the implementer: none
- Stop and hand back if: guide_stale を注意にすると、既存の注意の扱い（終了コード、並び）のどこかと食い違う

### S6: kotowari の skill にガイドを書く場面を足す

- Purpose: kotowari の skill に、ガイドを書く場面と見直す場面を足す
- Specification: `docs/ir/core/guides.md#REQ-core-205`
- Prerequisites: S5
- May change: `agent/skills/kotowari/SKILL.md`, `agent/skills/kotowari/references/guides.md`
- Done when: SKILL.md の場面の表にガイドの場面の行があり、references/guides.md が REQ-core-205 の how_to_verify の3つ（印を置く細かさの目安、`kotowari query` で項目を確かめて印に足す手順、節を見直してから guide_stale の detail の今の指紋を書き写す手順）を持つ
- Shown by: artifact — `agent/skills/kotowari/references/guides.md` と `agent/skills/kotowari/SKILL.md`。REQ-core-205 の how_to_verify のとおりに読んで確かめる
- Left to the implementer: 書き方と節の立て方。ほかの references と同じく英語で、先頭に改訂日の行を置く
- Stop and hand back if: 場面の説明に、仕様に無い決まり（kotowari が検査すること）を書く必要が出てくる

### S7: 計画の範囲の検査が揃っていることを確かめる

- Purpose: この計画が扱う要求とシナリオのすべてに印のあるテストがあり、変えたファイルと扱う ID に誤りが無いことを確かめる
- Specification: `docs/ir/core/guides.md#REQ-core-198`, `docs/ir/core/guides.md#REQ-core-204`, `docs/ir/core/output.md#REQ-core-206`
- Prerequisites: S1, S2, S3, S4, S5, S6
- May change: none
- Done when: REQ-core-198 から REQ-core-204、REQ-core-206、TBL-core-036、EX-core-362 から EX-core-375 の `kotowari query` の "tests" が空でなく、`kotowari check` の誤りが main で同じコマンドを実行したときにも出るもの（計画の前からある誤り）だけで、テストがすべて通る
- Shown by: check — `CARGO_BUILD_JOBS=4 cargo test --workspace`、`for id in REQ-core-198 REQ-core-199 REQ-core-200 REQ-core-201 REQ-core-202 REQ-core-203 REQ-core-204 REQ-core-206 TBL-core-036 $(seq -f 'EX-core-%g' 362 375); do CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- query $id | jq -e --arg id $id '.items[0].tests != []' > /dev/null || echo "no test: $id"; done`、`CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format text | grep '\[error\]'`
- Left to the implementer: none
- Stop and hand back if: この計画の外の要求やシナリオの誤りが、この計画の変更で新しく出る
