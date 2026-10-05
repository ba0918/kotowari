# TODO

kotowari と mds（`crates/kotowari-markdown-schema`）の未解決事項の記録。解決したものはこのファイルから落とし、仕様（`docs/ir/`）・判断の記録（`docs/decision/records/`）へ移す。
仕様の参照は IR の ID で書く（mds の散文の仕様 `mds.md` の R 番号は、仕様を IR に移したときに廃止した）。

分類:
- **判断待ち**: 仕様の解釈が割れるもの、または人が決めていないもの。人が決めたら仕様と実装を直す
- **後でやる**: やることは決まっているが、先に済ませることがあって今は手を付けられないもの
- **記録のみ**: 実害が小さい境界ケースや文言の細部。直すなら仕様の変更を伴う
- **参考**: 次に似た判断をするときのための調査の記録

## 判断待ち

## 後でやる

### 紹介ページのガイドへのリンク（kotowari）

- `site/template.html` のガイドへのリンクは `kotowari-v0.3.0` のタグを指し、「Japanese」と添えている。そのタグのガイドは日本語なので今は正しい。ガイドを英語と日本語の対にした後の最初のリリースで、新しいタグを指すようにし、「Japanese」の注記を外す

## 記録のみ

### REQ-core-174 の前からの出典の漏れ（kotowari）

- [REQ-core-174](docs/ir/core/findings.md#REQ-core-174) のうち、unknown_line・unknown_code_block・glossary_title_invalid の "line" を TBL-core-019 のとおりにすること、unknown_code_block と glossary_title_invalid を誤りにすることの出典は ir-engine の A89 で足りるが、"## " の直下の宣言の外の行から一覧を外す条件を支える決定が出典に無い（2026-09-24 の照合で見つかった。今回の変更の範囲の外）

### ID 62: 題名が無い文書の前置部領域の見出し（mds）

- 題名が無い文書では前置部領域の見出しがすべて「題名より前」と扱われ、open では宣言済み前置部でも許される。仕様（[REQ-schema-022](docs/ir/schema/document-structure.md#REQ-schema-022)、[REQ-schema-023](docs/ir/schema/document-structure.md#REQ-schema-023)）は題名の無い文書の扱いを明記していない。`missing_title` が別に立つので実害は無い

### 題名より前の `### FLAG-nnn`（kotowari）

- `document.item` を宣言したスキーマでは、エンジンは題名より前の深さ3の見出しも文書の直下の`項目`として読む（[REQ-schema-061](docs/ir/schema/document-structure.md#REQ-schema-061) の「最初の`節`より前」は題名より前も含む）。スキルの `agent/skills/kotowari/references/ir-form.md` は問題の記録の置き場を「`FLAGS.md` の `## Flags` の下か、題名の後」と書き、題名より前を書いていない

### 使われていない `extract: title`（kotowari）

- `crates/kotowari-core/schemas/` の ir.yaml、context.yaml、flags.yaml は題名を `extract: title` で取り出すが、kotowari-core のコードはその値を読まない。読んでいた `MapContext.title` は既に消えている

### `reading: line` の読み取りの回数（mds）

- `reading: line` の文書は、`validate` と `extract_values` がそれぞれ木を組み直し、見出しの行ごとに Markdown のパースを呼ぶ。段落の読み方より解析の回数が増える。影響の大きさは未計測

## 参考

### ox-content のパーサ（`ox_content_parser`）

- 調査日: 2026-09-17。Markdown パーサの置き換え候補として調査・実験した結果
- **性能（実測）**: 現行の `markdown` クレートとの比較で、パースは 60〜100 倍高速（release、全ノード走査で揃えて測定）。ただし mds の実行時間に占めるパースの割合は約 31% なので、`mds check ./` の実質的な高速化は 1.45 倍程度。200文書（約80KB）では体感できる差は出ない
- **品質（実測）**: CommonMark 0.31.2 の 652/652 例に準拠。ノード種類・行番号は現行の `markdown` クレートと一致（GFM の表・タスクリスト・打ち消しは `ParserOptions::gfm()` で有効化）
- **置き換えのコスト（調査済み）**: `ast.rs` の書き直し（serde が無いため `mds ast` の JSON を手書き）、`document.rs` の書き換え（Node 型の差し替え・行番号ヘルパの追加）、frontmatter の切り出しを mds 側で新規実装、インライン構造の違いへの対応
- **判断（2026-09-17）**: 現状維持。1.45 倍の高速化のために中〜大規模の書き直しとメンテナンスの負担を負う価値が薄い
- **次回の参考**: 新規に Markdown パーサが必要なとき、または mds を大規模な文書集合で使う要件が生じたときに再検討する
