# TODO

kotowari と mds（`crates/kotowari-markdown-schema`）の未解決事項の記録。解決したものはこのファイルから落とし、仕様（`docs/ir/`）・判断の記録（`docs/decision/records/`）へ移す。
仕様の参照は IR の ID で書く（mds の散文の仕様 `mds.md` の R 番号は、仕様を IR に移したときに廃止した）。

分類:
- **判断待ち**: 仕様の解釈が割れるもの、または人が決めていないもの。人が決めたら仕様と実装を直す
- **記録のみ**: 実害が小さい境界ケースや文言の細部。直すなら仕様の変更を伴う
- **参考**: 次に似た判断をするときのための調査の記録

## 判断待ち

### kotowari の skill を他の人へ配る仕組み

- 工程の skill 8つと kotowari スキルは `skills/` に置き、手元へはシンボリックリンクで入れると決めた（[A7（workflow-split）](docs/decision/records/2026-09-23-workflow-split.md#A7)）。他の人へ配る仕組み（agentic-workflow のような plugin marketplace など）は決めていない

### kotowari-brainstorm の照合レビューを中身に応じて段階化する

- 承認の手順2の照合レビューは、IR の項目を触らない壁打ちでも毎回別の LLM を呼び、最大3回まわる。利用者が費用を払い切れない前提（[A12（workflow-split）](docs/decision/records/2026-09-23-workflow-split.md#A12)）に合わない
- 案: 触った IR の項目の数とつながり（`git diff` と `kotowari query` で数えられる）を合図に、照合なし・主セッションが突き合わせる・別の LLM で照合する、の3段にし、決める前に理由を1行書く。段の境の件数は実測で決める
- 品質の敵対的レビュー（Finishing 1）は既に「要求が互いに矛盾しうるときだけ」の条件付き

## 記録のみ

### ID 23: 応答しないサーバのタイムアウト経路のテスト（mds）

- 固定値10秒のタイムアウトは実装済み。この経路だけテスト対象外。仕様がタイムアウト値を契約として宣言していない限り、テストを足しても契約を固定したことにならない

### ID 38 / ID 60: detail 文言を assert するテスト（mds）

- テストが detail の文言（`undeclared code block` など）を固定している。仕様は detail の文言を契約にしていない（[REQ-schema-008](docs/ir/schema/cli.md#REQ-schema-008)）ので、挙動を変えない文言の変更で壊れる

### ID 62: 題名が無い文書の前置部領域の見出し（mds）

- 題名が無い文書では前置部領域の見出しがすべて「題名より前」と扱われ、open では宣言済み前置部でも許される。仕様（[REQ-schema-022](docs/ir/schema/document-structure.md#REQ-schema-022)、[REQ-schema-023](docs/ir/schema/document-structure.md#REQ-schema-023)）は題名の無い文書の扱いを明記していない。`missing_title` が別に立つので実害は無い

### 題名より前の `### FLAG-nnn`（kotowari）

- `document.item` を宣言したスキーマでは、エンジンは題名より前の深さ3の見出しも文書の直下の`項目`として読む（[REQ-schema-061](docs/ir/schema/document-structure.md#REQ-schema-061) の「最初の`節`より前」は題名より前も含む）。スキルの `skills/kotowari/references/ir-form.md` は問題の記録の置き場を「題名の後」と書いている

### 使われていない `MapContext.title`（kotowari）

- `crates/kotowari-core/src/finding_map.rs` の `MapContext.title` は、multiple_titles を`生の行`から写すようになってから読まれていない。3つのスキーマの `extract: title` と対なので、消すなら両方

### `reading: line` の読み取りの回数（mds）

- `reading: line` の文書は、`validate` と `extract_values` がそれぞれ木を組み直し、見出しの行ごとに Markdown のパースを呼ぶ。段落の読み方より解析の回数が増える。影響の大きさは未計測

### tree-sitter-rust の読み違い（kotowari）

- Rust のソースに `unwrap_or(&raw)` と書くと、tree-sitter-rust が `&raw)` を生の借用（`&raw const`）の書き始めと読み、`kotowari check` がそのファイルに unparsable_file を出す。コンパイラは通す（変数名を変えると消えることを確かめた）
- 依存の側の欠陥。回避は変数名を `raw` にしないこと

### 置き換えの照合で読み残した文書（kotowari）

- 置き換えの5回目の照合（2026-09-23）は 43 文書のうち 36 文書しか読めていない。残り7件は `docs/ir/core/` の equivalents、mutants、mutants-input、revision-link、test-discovery、test-markers と `docs/ir/schema/schema-resolution.md`。置き換えと別の題目として後回しにした

### ID 64: 入れ子の深い文書に対する処理時間（mds）

- 実測（release、`mds ast`）: 入れ子の深さ 200 で 0.06 秒、400 で 0.42 秒、800 で 3.84 秒、2000（約 4MB）で 66 秒。超線形に伸びる
- 支配的なコストは依存クレート `markdown` のパースにあり、mds 自身のコードではない。mds 側で打てる手は入力の大きさ・深さに上限を設けることだけ
- その上限は仕様に無い。新設すると、正当な文書を弾く可能性を仕様に持ち込む。人が手で書く文書でも LLM が書く文書でも、この深さは意図して用意しない限り現れない
- **判断（2026-09-22）**: 上限を設けない。上限が要るという要求が実際に出てから決める

### ID 65: `http://` のスキーマの取得（mds）

- `$schema` に `http://` の URL を書くと平文で取得する。経路上で差し替えられたスキーマは、正規表現・enum・必須の宣言を通じて検証の合否をそのまま左右する
- `https://` だけを許すと、社内の平文配布や手元の検証用サーバを使えなくなる。どこから取るかは利用者が決めることである
- キャッシュに期限が無いのは [REQ-schema-013](docs/ir/schema/schema-resolution.md#REQ-schema-013) が定めた振る舞いであり（初回に取得して `.mds/cache/` に保存し、以降はキャッシュを使う）、欠陥ではない
- **判断（2026-09-22）**: `http://` を禁止しない。取得元の信頼は利用者の責任として扱う

## 参考

### ox-content のパーサ（`ox_content_parser`）

- 調査日: 2026-09-17。Markdown パーサの置き換え候補として調査・実験した結果
- **性能（実測）**: 現行の `markdown` クレートとの比較で、パースは 60〜100 倍高速（release、全ノード走査で揃えて測定）。ただし mds の実行時間に占めるパースの割合は約 31% なので、`mds check ./` の実質的な高速化は 1.45 倍程度。200文書（約80KB）では体感できる差は出ない
- **品質（実測）**: CommonMark 0.31.2 の 652/652 例に準拠。ノード種類・行番号は現行の `markdown` クレートと一致（GFM の表・タスクリスト・打ち消しは `ParserOptions::gfm()` で有効化）
- **置き換えのコスト（調査済み）**: `ast.rs` の書き直し（serde が無いため `mds ast` の JSON を手書き）、`document.rs` の書き換え（Node 型の差し替え・行番号ヘルパの追加）、frontmatter の切り出しを mds 側で新規実装、インライン構造の違いへの対応
- **判断（2026-09-17）**: 現状維持。1.45 倍の高速化のために中〜大規模の書き直しとメンテナンスの負担を負う価値が薄い
- **次回の参考**: 新規に Markdown パーサが必要なとき、または mds を大規模な文書集合で使う要件が生じたときに再検討する
