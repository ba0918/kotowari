# Glossary

kotowari-markdown-view（view）の仕様 IR で使う用語を置く。view は kotowari を知らない描画のエンジンで、ここの用語は view の入力と出力の言葉である。

| Term | Meaning | Source |
|---|---|---|
| 描画の入力 | view が1回の描画で受け取るものの全体。`文書`の並びと`参照の表`からなる | docs/decision/records/2026-10-02-whole-picture.md#A25, docs/decision/records/2026-10-02-whole-picture.md#A42 |
| 文書 | `描画の入力`の中の1つの`ページ`の元。名前、題名、冒頭の lead の`部品`、`節`の並びを持つ。ファイルではない | docs/decision/records/2026-10-02-whole-picture.md#A42, docs/decision/records/2026-10-02-whole-picture.md#A68, docs/decision/records/2026-10-02-whole-picture.md#A82 |
| 節 | `文書`の中の "## " の見出し1つとその下の`ブロック`の並び。古いかどうかの真偽を持つ | docs/decision/records/2026-10-02-whole-picture.md#A39, docs/decision/records/2026-10-02-whole-picture.md#A68 |
| ブロック | `節`の中身の単位。Markdown の文章の塊か`部品`のどちらか | docs/decision/records/2026-10-02-whole-picture.md#A42 |
| 部品 | 種類の決まった描画の単位。種類の名前と、その種類のスキーマに合う値を持つ。ブロック・コンポーネントとは言わない | docs/decision/records/2026-10-02-whole-picture.md#A11, docs/decision/records/2026-10-02-whole-picture.md#A36, docs/decision/records/2026-10-02-whole-picture.md#A51 |
| 部品のスキーマ | `部品`の種類ごとに中身の形を宣言する JSON Schema。view が持って公開し、部品の形の正とする | docs/decision/records/2026-10-02-whole-picture.md#A45, docs/decision/records/2026-10-02-whole-picture.md#A46, docs/decision/records/2026-10-02-whole-picture.md#A47 |
| 参照 | `部品`の値の中の、名前が "refs" か "ref" の欄に書いた文字列 | docs/decision/records/2026-10-02-whole-picture.md#A66 |
| 参照の表 | `参照`の文字列から、表示名、本文、状態への対応。view の外で作られて渡される。リンク表とは言わない | docs/decision/records/2026-10-02-whole-picture.md#A25, docs/decision/records/2026-10-02-whole-picture.md#A51, docs/decision/records/2026-10-02-whole-picture.md#A65, docs/decision/records/2026-10-02-whole-picture.md#A67 |
| ページ | view が出す1つのファイルの名前と中身の組。view はファイルを書かず、ページの並びを返す | docs/decision/records/2026-10-02-whole-picture.md#A26, docs/decision/records/2026-10-02-whole-picture.md#A56, docs/decision/records/2026-10-02-whole-picture.md#A22, docs/decision/records/2026-10-02-whole-picture.md#A82 |
