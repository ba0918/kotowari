# 用語集

| 用語 | 意味 | 出典 |
|---|---|---|
| IR | 正規化した仕様の Markdown の文書の集まり。設定の "ir" の置き場の直下に置く | experiments/003-cli/brainstorm/records.md#A12, experiments/003-cli/brainstorm/records.md#A32, experiments/003-cli/brainstorm/records.md#A47, experiments/003-cli/brainstorm/records.md#A25, experiments/003-cli/brainstorm/records.md#A52 |
| 基準のディレクトリ | カレントディレクトリから上に向かって探し、最初に見つかった ".kotowari/" のあるディレクトリ。無ければカレントディレクトリ | experiments/003-cli/brainstorm/records.md#A37 |
| 設定ファイル | 読む場所と検査の値を書く YAML のファイル。既定は ".kotowari/config.yaml" | experiments/003-cli/brainstorm/records.md#A2, experiments/003-cli/brainstorm/records.md#A12 |
| 判断の記録 | brainstorm で決めたことを1行1決定で並べたファイル。設定の "decisions.records" の置き場の下に置く | experiments/003-cli/brainstorm/records.md#A22, experiments/003-cli/brainstorm/records.md#A48, experiments/003-cli/adr/0003-records-and-adr.md#状況 |
| ADR | 4つの基準に当たる判断を、状況、決定、理由、却下した案、結果の節とともに残すファイル。設定の "decisions.adr" の置き場の下に置く | experiments/003-cli/brainstorm/records.md#A22, experiments/003-cli/brainstorm/records.md#A43, experiments/003-cli/brainstorm/records.md#A38, experiments/003-cli/brainstorm/records.md#A47 |
| 出典 | 項目の元になった決定か ADR の節を指す "パス#印" の文字列 | experiments/003-cli/brainstorm/records.md#A3, experiments/003-cli/brainstorm/records.md#A38 |
| 決定の番号 | 判断の記録の決定の行の先頭にある番号。英大文字1文字に1桁以上の数字（"A26"、"P1" の形） | experiments/003-cli/brainstorm/records.md#A13, experiments/003-cli/brainstorm/records.md#A38, experiments/003-cli/brainstorm/records.md#A115 |
| 決定の節 | 判断の記録の Agreements、Prohibitions、Delegated、Rejected の節。"## " の見出しで始まり次の "## " の見出しで終わる | experiments/003-cli/brainstorm/records.md#A38, experiments/003-cli/brainstorm/records.md#A115 |
| 話題ごとの文書 | IR の文書のうち、用語集と問題の記録を除いたもの | experiments/003-cli/brainstorm/ir-form.md#文書 |
| 用語集 | IR の置き場の直下の "CONTEXT.md" | experiments/003-cli/brainstorm/records.md#A41, experiments/003-cli/brainstorm/records.md#A56 |
| 用語 | 用語集の表の用語の列にある語。IR の文の中ではバッククォートで囲む | experiments/003-cli/brainstorm/records.md#A31, experiments/003-cli/brainstorm/records.md#A42 |
| 問題の記録 | IR の置き場の直下の "FLAGS.md" | experiments/003-cli/brainstorm/records.md#A28, experiments/003-cli/brainstorm/records.md#A56 |
| 題名 | 文書の "# " で始まる行 | experiments/003-cli/brainstorm/records.md#A42 |
| 文書が扱う範囲 | 題名の後、最初の "## " か "### " より前にある空でない行 | experiments/003-cli/brainstorm/records.md#A30, experiments/003-cli/brainstorm/records.md#A55 |
| 項目 | 要求、決定表、性質、問題の記録のそれぞれ。"### ID: 名前" の見出しで始まる | experiments/003-cli/brainstorm/records.md#A42, experiments/003-cli/brainstorm/records.md#A52, experiments/003-cli/brainstorm/ir-form.md#項目 |
| 要求 | "### REQ-nnn: 名前" の見出しで始まる項目 | experiments/003-cli/brainstorm/records.md#A42 |
| 文 | 見出しの下の、一覧でも表でもない空でない行 | experiments/003-cli/brainstorm/records.md#A42 |
| 決定表 | "### TBL-nnn: 名前" の見出しで始まり、Markdown の表を持つ項目 | experiments/003-cli/brainstorm/ir-form.md#項目 |
| 性質 | "### PROP-nnn: 名前" の見出しで始まり、文を持つ項目 | experiments/003-cli/brainstorm/ir-form.md#項目 |
| シナリオ | gherkin のコードブロックの中の "Scenario:" と、その直前の行のタグ | experiments/003-cli/brainstorm/records.md#A27, experiments/003-cli/brainstorm/ir-form.md#項目 |
| ID | "REQ-"、"TBL-"、"PROP-"、"EX-"、"FLAG-" のいずれかに3桁の数字を続けた、項目の識別子 | experiments/003-cli/brainstorm/records.md#A52 |
| 指摘 | 検査で見つけた1件。kind、severity、path、line、detail を持つ | experiments/003-cli/brainstorm/records.md#A40 |
| 誤り | 終了コードを1にする指摘 | experiments/003-cli/brainstorm/records.md#A29 |
| 警告 | 終了コードを変えない指摘 | experiments/003-cli/brainstorm/records.md#A17, experiments/003-cli/brainstorm/records.md#A29 |
| 停止 | 検査を行えずに終了コード2で終わること | experiments/003-cli/brainstorm/records.md#A20, experiments/003-cli/brainstorm/records.md#A40 |
| 印 | テストに書く "@kotowari[ID, ...]" の並び | experiments/003-cli/brainstorm/records.md#A14, experiments/003-cli/brainstorm/records.md#A57 |
| テスト | 問い合わせでテストと数える関数 | experiments/003-cli/brainstorm/records.md#A24, experiments/003-cli/brainstorm/records.md#A26 |
| テストのファイル | 設定の "tests.files" の glob に当たるファイル | experiments/003-cli/brainstorm/records.md#A36, experiments/003-cli/brainstorm/records.md#A47 |
| 問い合わせ | tree-sitter でテストを見つける、言語ごとの問い合わせ。kotowari が同梱する | experiments/003-cli/brainstorm/records.md#A24, experiments/003-cli/brainstorm/records.md#A58 |
| 問い合わせのある言語 | 拡張子から決まる言語のうち、同梱の問い合わせがあるもの。第1版では Rust（".rs"）だけ | experiments/003-cli/brainstorm/records.md#A39, experiments/003-cli/brainstorm/records.md#A58 |
| 問い合わせの無い言語 | 拡張子から決まる言語のうち、同梱の問い合わせが無いもの | experiments/003-cli/brainstorm/records.md#A39 |
| 曖昧語 | 設定の "vague_words" に並べた語 | experiments/003-cli/brainstorm/records.md#A21, experiments/003-cli/brainstorm/records.md#A41 |
| 対象の行 | 用語と曖昧語の検査を受ける行。TBL-013 で決める | experiments/003-cli/brainstorm/records.md#A53 |
| 文書名の参照 | コードブロックの外にある文書名の並び。TBL-014 で決める | experiments/003-cli/brainstorm/records.md#A54, experiments/003-cli/brainstorm/records.md#A47 |
| 除外 | 仕様が列挙した、kotowari が読まないもの。隠しディレクトリ、ディレクトリのシンボリックリンク、IR の置き場のサブディレクトリ、".md" 以外のファイル、形に合わない見出しの下の行。列挙に無い読み飛ばしは作らない | experiments/003-cli/brainstorm/records.md#A100, experiments/003-cli/brainstorm/records.md#A102 |
| コードブロック | 行頭の3つ以上の "`" か "~" で始まる行から、同じ文字で同じ数以上の行までの部分 | experiments/003-cli/brainstorm/records.md#A108 |
