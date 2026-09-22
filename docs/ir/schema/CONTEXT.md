# 用語集

mds の仕様 IR で使う用語を置く。意味が一般の用法と違うものだけを載せる。

| 用語 | 意味 | 出典 |
|---|---|---|
| スキーマ | 文書の書式（構造と制約）と抽出規則を宣言する YAML ファイル | docs/decision/records/2026-09-21-mds-spec.md#A1 |
| 文書 | mds が検査と抽出の対象にする Markdown ファイル | docs/decision/records/2026-09-21-mds-spec.md#A1 |
| frontmatter | 文書の先頭にある "---" で挟んだ YAML ブロック | docs/decision/records/2026-09-21-mds-spec.md#A1 |
| 閉じた世界 | スキーマに宣言していない見出しと行を誤りとする検証の流儀 | docs/decision/records/2026-09-21-mds-spec.md#A2 |
| 開いた世界 | 宣言していない構造とその内側の行を許す検証の流儀 | docs/decision/records/2026-09-21-mds-spec.md#A2 |
| 規則種別 | スキーマが文書の構造を記述するノードの種類 | docs/decision/records/2026-09-21-mds-spec.md#A39 |
| ノード | スキーマの木の1要素。検証と抽出の単位 | docs/decision/records/2026-09-21-mds-spec.md#A39 |
| 題名 | 文書の深さ1の見出し | docs/decision/records/2026-09-21-mds-spec.md#A5 |
| 前置部 | 題名の後、最初の節より前の部分。文書の直下の項目を宣言したスキーマでは、最初の節か項目より前の部分 | docs/decision/records/2026-09-21-mds-spec.md#A5, docs/decision/records/2026-09-21-mds-spec.md#A38, docs/decision/records/2026-09-23-ir-engine-gaps.md#A23 |
| 節 | 深さ2の見出し | docs/decision/records/2026-09-21-mds-spec.md#A5 |
| 項目 | ID と名前を持つ深さ3の見出し。節の下に置くか、スキーマが宣言したときは節を挟まずに文書の直下に置く | docs/decision/records/2026-09-21-mds-spec.md#A5, docs/decision/records/2026-09-23-ir-engine-gaps.md#A15, docs/decision/records/2026-09-23-ir-engine-gaps.md#A23 |
| フィールド行 | 一覧のマーカーに続く「名前と値」の形の行で、名前がスキーマの宣言と一致するもの | docs/decision/records/2026-09-21-mds-spec.md#A10 |
| 箇条書き | 一覧のマーカーで始まり、フィールド行でない一覧の行 | docs/decision/records/2026-09-21-mds-spec.md#A10 |
| 文 | 見出しや前置部の下にある、一覧でも表でもない空でない行。読み方が段落のときは複数行にまたがる段落を1つの文として数え、行のときは1行を1つの文として数える | docs/decision/records/2026-09-21-mds-spec.md#A30, docs/decision/records/2026-09-23-ir-engine-gaps.md#A13, docs/decision/records/2026-09-23-ir-engine-gaps.md#A20 |
| 継続段落 | 読み方が段落のときに、一覧の行の後に空行で区切って続く、その行の子である段落。読み方が行のときは作らない | docs/decision/records/2026-09-21-mds-spec.md#A11, docs/decision/records/2026-09-23-ir-engine-gaps.md#A13 |
| 表 | Markdown の表。ヘッダのセル列と列数と、ヘッダの合う最初の表だけを規則の表にする選び方を指定できる | docs/decision/records/2026-09-21-mds-spec.md#A13, docs/decision/records/2026-09-23-ir-engine-gaps.md#A27, docs/decision/records/2026-09-23-ir-engine-gaps.md#A28 |
| コードブロック | フェンスで囲んだブロック。言語と行ごとの規則を指定できる | docs/decision/records/2026-09-21-mds-spec.md#A12 |
| 抽出 | スキーマが各ノードに宣言する、値を取り出す規則 | docs/decision/records/2026-09-21-mds-spec.md#A4 |
| 配置パス | 抽出した値を置く場所を指すドット区切りの名前 | docs/decision/records/2026-09-21-mds-spec.md#A4 |
| 指摘 | 検査が出す1件の結果 | docs/decision/records/2026-09-21-mds-spec.md#A17 |
| 停止 | 検査を行えないときに終了コード 2 で終わること | docs/decision/records/2026-09-21-mds-spec.md#A15 |
| 基準のディレクトリ | カレントディレクトリから上に向かって探し、最初に見つかった ".mds/" のあるディレクトリ | docs/decision/records/2026-09-21-mds-spec.md#A14 |
| 出現回数 | ノードが現れてよい個数の下限と上限 | docs/decision/records/2026-09-21-mds-spec.md#A40 |
| 条件付き規則 | あるフィールド行の値に応じて、別の規則の適用を切り替える条件節 | docs/decision/records/2026-09-21-mds-spec.md#A28 |
| 導かれる値 | 抽出の宣言で、文書の文字ではなく位置や識別子から導く値。行番号、項目の見出しの ID、項目の見出しの名前、生の行、要素の最後の行の5つ | docs/decision/records/2026-09-22-ir-engine.md#A49, docs/decision/records/2026-09-21-mds-spec.md#A23, docs/decision/records/2026-09-23-ir-engine-gaps.md#A18, docs/decision/records/2026-09-23-ir-engine-gaps.md#A24 |
| 要素の値 | 抽出で、その要素そのものから取れる値。導かれる値と区別する | docs/decision/records/2026-09-22-ir-engine.md#A52 |
| ノードの名前 | スキーマが宣言したノードの名前。持つのは節とフィールド行だけで、題名のように宣言上の名前を持たないノードもある | docs/decision/records/2026-09-22-ir-engine.md#A27, docs/decision/records/2026-09-21-mds-spec.md#A45 |
| 生の行 | 指摘が指す行、または抽出の要素の行の文字そのまま。字下げと末尾の空白を含み、組み立て直さない | docs/decision/records/2026-09-22-ir-engine.md#A49, docs/decision/records/2026-09-22-ir-engine.md#A29, docs/decision/records/2026-09-22-ir-engine.md#A57, docs/decision/records/2026-09-22-ir-engine.md#A62 |
| 読み方 | スキーマの最上位で宣言する、見出しの下の行を文と一覧の行に分ける単位。段落（"paragraph"。既定で、CommonMark の段落として読む）と行（"line"。1行ずつ読む）の2つ | docs/decision/records/2026-09-23-ir-engine-gaps.md#A12, docs/decision/records/2026-09-23-ir-engine-gaps.md#A13, docs/decision/records/2026-09-23-ir-engine-gaps.md#A20 |
