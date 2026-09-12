# 用語集

| 用語 | 意味 | 出典 |
|---|---|---|
| IR | 正規化した仕様の Markdown の文書の集まり。設定の "ir" の置き場の直下に置く | brainstorm/records.md#A12, brainstorm/records.md#A32, brainstorm/records.md#A47, brainstorm/records.md#A25, brainstorm/records.md#A52 |
| 基準のディレクトリ | カレントディレクトリから上に向かって探し、最初に見つかった ".kotowari/" のあるディレクトリ。無ければカレントディレクトリ | brainstorm/records.md#A37 |
| 設定ファイル | 読む場所と検査の値を書く YAML のファイル。既定は ".kotowari/config.yaml" | brainstorm/records.md#A2, brainstorm/records.md#A12 |
| 判断の記録 | brainstorm で決めたことを1行1決定で並べたファイル。設定の "decisions.records" の置き場の下に置く | brainstorm/records.md#A22, brainstorm/records.md#A48, adr/0003-records-and-adr.md#状況 |
| ADR | 4つの基準に当たる判断を、状況、決定、理由、却下した案、結果の節とともに残すファイル。設定の "decisions.adr" の置き場の下に置く | brainstorm/records.md#A22, brainstorm/records.md#A43, brainstorm/records.md#A38, brainstorm/records.md#A47 |
| 出典 | 項目の元になった決定か ADR の節を指す "パス#印" の文字列 | brainstorm/records.md#A3, brainstorm/records.md#A38 |
| 決定の番号 | 判断の記録の決定の行の先頭にある番号（"A26"、"P1" の形） | brainstorm/records.md#A13, brainstorm/records.md#A38 |
| 決定の節 | 判断の記録の Agreements、Prohibitions、Delegated、Rejected の節 | brainstorm/records.md#A38 |
| 話題ごとの文書 | IR の文書のうち、用語集と問題の記録を除いたもの | brainstorm/ir-form.md#文書 |
| 用語集 | IR の置き場の直下の "CONTEXT.md" | brainstorm/records.md#A41, brainstorm/records.md#A56 |
| 用語 | 用語集の表の用語の列にある語。IR の文の中ではバッククォートで囲む | brainstorm/records.md#A31, brainstorm/records.md#A42 |
| 問題の記録 | IR の置き場の直下の "FLAGS.md" | brainstorm/records.md#A28, brainstorm/records.md#A56 |
| 題名 | 文書の "# " で始まる行 | brainstorm/records.md#A42 |
| 文書が扱う範囲 | 題名の後、最初の "## " か "### " より前にある空でない行 | brainstorm/records.md#A30, brainstorm/records.md#A55 |
| 項目 | 要求、決定表、性質、問題の記録のそれぞれ。"### ID: 名前" の見出しで始まる | brainstorm/records.md#A42, brainstorm/records.md#A52, brainstorm/ir-form.md#項目 |
| 要求 | "### REQ-nnn: 名前" の見出しで始まる項目 | brainstorm/records.md#A42 |
| 文 | 見出しの下の、一覧でも表でもない空でない行 | brainstorm/records.md#A42 |
| 決定表 | "### TBL-nnn: 名前" の見出しで始まり、Markdown の表を持つ項目 | brainstorm/ir-form.md#項目 |
| 性質 | "### PROP-nnn: 名前" の見出しで始まり、文を持つ項目 | brainstorm/ir-form.md#項目 |
| シナリオ | gherkin のコードブロックの中の "Scenario:" と、その直前の行のタグ | brainstorm/records.md#A27, brainstorm/ir-form.md#項目 |
| ID | "REQ-"、"TBL-"、"PROP-"、"EX-"、"FLAG-" のいずれかに3桁の数字を続けた、項目の識別子 | brainstorm/records.md#A52 |
| 指摘 | 検査で見つけた1件。kind、severity、path、line、detail を持つ | brainstorm/records.md#A40 |
| 誤り | 終了コードを1にする指摘 | brainstorm/records.md#A29 |
| 警告 | 終了コードを変えない指摘 | brainstorm/records.md#A17, brainstorm/records.md#A29 |
| 停止 | 検査を行えずに終了コード2で終わること | brainstorm/records.md#A20, brainstorm/records.md#A40 |
| 印 | テストに書く "@kotowari[ID, ...]" の並び | brainstorm/records.md#A14, brainstorm/records.md#A57 |
| テスト | 問い合わせでテストと数える関数 | brainstorm/records.md#A24, brainstorm/records.md#A26 |
| テストのファイル | 設定の "tests.files" の glob に当たるファイル | brainstorm/records.md#A36, brainstorm/records.md#A47 |
| 問い合わせ | tree-sitter でテストを見つける、言語ごとの問い合わせ。kotowari が同梱する | brainstorm/records.md#A24, brainstorm/records.md#A58 |
| 問い合わせのある言語 | 拡張子から決まる言語のうち、同梱の問い合わせがあるもの。003 では Rust（".rs"）だけ | brainstorm/records.md#A39, brainstorm/records.md#A58 |
| 問い合わせの無い言語 | 拡張子から決まる言語のうち、同梱の問い合わせが無いもの | brainstorm/records.md#A39 |
| 曖昧語 | 設定の "vague_words" に並べた語 | brainstorm/records.md#A21, brainstorm/records.md#A41 |
| 対象の行 | 用語と曖昧語の検査を受ける行。TBL-013 で決める | brainstorm/records.md#A53 |
| 文書名の参照 | コードブロックの外にある文書名の並び。TBL-014 で決める | brainstorm/records.md#A54, brainstorm/records.md#A47 |
