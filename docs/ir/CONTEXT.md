# 用語集

| 用語 | 意味 | 出典 |
|---|---|---|
| IR | 正規化した仕様の Markdown の文書の集まり。設定の "ir" の置き場の下に、ディレクトリの深さに制限なく置く | docs/decision/records/records.md#A12, docs/decision/records/records.md#A32, docs/decision/records/records.md#A47, docs/decision/records/records.md#A25, docs/decision/records/records.md#A52, docs/decision/records/2026-09-16-ir-tree.md#A1 |
| 基準のディレクトリ | カレントディレクトリから上に向かって探し、最初に見つかった ".kotowari/" のあるディレクトリ。無ければカレントディレクトリ | docs/decision/records/records.md#A37 |
| 設定ファイル | 読む場所と検査の値を書く YAML のファイル。既定は ".kotowari/config.yaml" | docs/decision/records/records.md#A2, docs/decision/records/records.md#A12 |
| 判断の記録 | brainstorm で決めたことを1行1決定で並べたファイル。決定の行の下に、理由などの補足の行を持てる（字下げの幅は問わない）。設定の "decisions.records" の置き場の下に置き、決定の節の見出し（"## Agreements"、"## Prohibitions"、"## Delegated"、"## Rejected"）をコードブロックの外に1つ以上持つことで見分ける | docs/decision/records/records.md#A22, docs/decision/records/records.md#A48, docs/decision/adr/0003-records-and-adr.md#状況, docs/decision/records/records.md#A134, docs/decision/records/2026-09-17-decision-log.md#A1, docs/decision/records/2026-09-17-record-form.md#A5, docs/decision/records/2026-09-17-record-form.md#A46 |
| ADR | 4つの基準に当たる判断を、状況、決定、理由、却下した案、結果の節とともに残したファイル。設定の "decisions.adr" の置き場の下に置く。2026-09-17 以降は新しく書かず、既存のものを出典の先として残す | docs/decision/records/records.md#A22, docs/decision/records/records.md#A43, docs/decision/records/records.md#A38, docs/decision/records/records.md#A47, docs/decision/records/2026-09-17-decision-log.md#A5 |
| 出典 | 項目の元になった決定か ADR の節を指す "パス#印" の文字列 | docs/decision/records/records.md#A3, docs/decision/records/records.md#A38 |
| 決定の番号 | 判断の記録の決定の行の先頭にある番号。英大文字1文字に1桁以上の数字（"A26"、"P1" の形） | docs/decision/records/records.md#A13, docs/decision/records/records.md#A38, docs/decision/records/records.md#A115 |
| 決定の節 | 判断の記録の Agreements、Prohibitions、Delegated、Rejected の節。"## " の見出しで始まり次の "## " の見出しで終わる | docs/decision/records/records.md#A38, docs/decision/records/records.md#A115, docs/decision/records/records.md#A134 |
| 番号の行 | 判断の記録の節にある、行頭の空白を除いて "- " と決定の番号の形の文字に空白が続く行か、"- " と決定の番号の形の文字だけの行。決定の節では決定の行、Undecided と Superseded の節でも同じ形。番号の行の判定は補足の行より先 | docs/decision/records/2026-09-17-record-form.md#A30, docs/decision/records/2026-09-17-record-form.md#A33, docs/decision/records/2026-09-17-record-form.md#A14 |
| 補足の行 | 番号の行の次から、次の番号の行か "## " の見出しまでの間にある、行頭の空白を除いて "- 名前:" の形の行。名前は "- " の直後から最初の ":" までで1文字以上あり空白と ":" を含まず、値は ":" の後の前後の空白を除いた文字。字下げの幅は問わず、空行を挟んでよい。番号の行は補足の行と見ない | docs/decision/records/2026-09-17-record-form.md#A5, docs/decision/records/2026-09-17-record-form.md#A33, docs/decision/records/2026-09-17-record-form.md#A27, docs/decision/records/2026-09-17-decision-log.md#A1 |
| Superseded の節 | 判断の記録の "## Superseded" の節。置き換えられた決定の行を置く。決定の節ではなく、そこを指す出典は無効 | docs/decision/records/2026-09-17-decision-log.md#A12, docs/decision/records/2026-09-17-record-form.md#A3 |
| 話題ごとの文書 | IR の文書のうち、用語集と問題の記録を除いたもの | docs/decision/records/ir-form.md#文書 |
| 用語集 | IR の置き場のどのディレクトリにも置ける "CONTEXT.md"。文書から見えるのは、その文書の連鎖にある用語集 | docs/decision/records/records.md#A41, docs/decision/records/records.md#A56, docs/decision/records/2026-09-16-ir-tree.md#A3, docs/decision/records/2026-09-16-ir-tree.md#A8 |
| 連鎖 | 文書のあるディレクトリから IR の置き場の根までの各ディレクトリの "CONTEXT.md" の並び。根に近い方が先 | docs/decision/records/2026-09-16-ir-tree.md#A3, docs/decision/records/2026-09-16-ir-tree.md#A4 |
| 用語 | 文書の連鎖にある用語集の表の用語の列にある語。IR の文の中ではバッククォートで囲む | docs/decision/records/records.md#A31, docs/decision/records/records.md#A42, docs/decision/records/2026-09-16-ir-tree.md#A3 |
| 問題の記録 | IR の置き場のどのディレクトリにも置ける "FLAGS.md" | docs/decision/records/records.md#A28, docs/decision/records/records.md#A56, docs/decision/records/2026-09-16-ir-tree.md#A8 |
| 題名 | 文書の "# " で始まる行 | docs/decision/records/records.md#A42 |
| 文書が扱う範囲 | 題名の後、最初の "## " か "### " より前にある空でない行 | docs/decision/records/records.md#A30, docs/decision/records/records.md#A55 |
| 項目 | 要求、決定表、性質、問題の記録のそれぞれ。"### ID: 名前" の見出しで始まる | docs/decision/records/records.md#A42, docs/decision/records/records.md#A52, docs/decision/records/ir-form.md#項目 |
| 要求 | "### REQ-nnn: 名前" の見出しで始まる項目 | docs/decision/records/records.md#A42 |
| 文 | 見出しの下の、一覧でも表でもない空でない行 | docs/decision/records/records.md#A42 |
| 決定表 | "### TBL-nnn: 名前" の見出しで始まり、Markdown の表を持つ項目 | docs/decision/records/ir-form.md#項目 |
| 性質 | "### PROP-nnn: 名前" の見出しで始まり、文を持つ項目 | docs/decision/records/ir-form.md#項目 |
| シナリオ | gherkin のコードブロックの中の "Scenario:" と、その直前の行のタグ | docs/decision/records/records.md#A27, docs/decision/records/ir-form.md#項目 |
| ID | "REQ-"、"TBL-"、"PROP-"、"EX-"、"FLAG-" のいずれかに3桁以上の数字（4桁以上のときは先頭が "0" でない）を続けた、項目の識別子 | docs/decision/records/records.md#A52, docs/decision/records/2026-09-16-ir-tree.md#A6, docs/decision/records/2026-09-16-ir-tree.md#A11 |
| 指摘 | 検査で見つけた1件。kind、severity、path、line、detail を持つ | docs/decision/records/records.md#A40 |
| 誤り | 終了コードを1にする指摘 | docs/decision/records/records.md#A29 |
| 注意 | 終了コードを変えない指摘。severity は "notice" | docs/decision/records/records.md#A17, docs/decision/records/records.md#A29, docs/decision/records/2026-09-16-notice.md#A1, docs/decision/records/2026-09-16-notice.md#A2 |
| 停止 | 検査を行えずに終了コード2で終わること | docs/decision/records/records.md#A20, docs/decision/records/records.md#A40 |
| 印 | テストに書く "@kotowari[ID, ...]" の並び | docs/decision/records/records.md#A14, docs/decision/records/records.md#A57 |
| テスト | 問い合わせでテストと数える関数 | docs/decision/records/records.md#A24, docs/decision/records/records.md#A26 |
| テストのファイル | 設定の "tests.files" の glob に当たるファイル | docs/decision/records/records.md#A36, docs/decision/records/records.md#A47 |
| 問い合わせ | tree-sitter でテストを見つける、言語ごとの問い合わせ。kotowari が同梱する | docs/decision/records/records.md#A24, docs/decision/records/records.md#A58 |
| 問い合わせのある言語 | 拡張子から決まる言語のうち、同梱の問い合わせがあるもの。第1版では Rust（".rs"）だけ | docs/decision/records/records.md#A39, docs/decision/records/records.md#A58, docs/decision/records/records.md#A128 |
| 問い合わせの無い言語 | 拡張子から決まる言語のうち、同梱の問い合わせが無いもの | docs/decision/records/records.md#A39 |
| 曖昧語 | 設定の "vague_words" に並べた語 | docs/decision/records/records.md#A21, docs/decision/records/records.md#A41 |
| 対象の行 | 用語と曖昧語の検査を受ける行。TBL-013 で決める | docs/decision/records/records.md#A53 |
| 文書名の参照 | コードブロックの外にある文書名の並び。TBL-014 で決める | docs/decision/records/records.md#A47, docs/decision/records/2026-09-16-ir-tree.md#A21 |
| 除外 | 仕様が列挙した、kotowari が指摘を出さずに読まないか見ないもの。隠しディレクトリ、ディレクトリのシンボリックリンク、ディレクトリでも通常のファイルでもないもの（ソケット、名前付きパイプ、デバイス）、".md" 以外のファイル、".kotowari" という名前のファイル、形に合わない見出しの下の行、コードブロックの中（gherkin を除く）、gherkin のブロックの外の "Scenario:" の行、"## " の見出しの直下で最初の "### " より前の空でない行、題名より前の空でない行、テストの外と関数の本体の途中にある印、行の中の二重引用符の中、二重引用符が奇数のときの最後の引用符から行末まで、TBL-013 で対象外の行。判断の記録では、TBL-022 の表に無い節（Revisions と "## Context" を含む）にある番号の行の形の行と補足の行の形の行、節の最初の番号の行より前にある補足の行の形の行、番号の行でも補足の行でも見出しでもない行（箇条でない本文の行と最初の "## " の見出しより前の行を含む）、コードブロックの中（gherkin を含む。中の "## " の見出しも数えない。閉じられずに文書が終わるときは終わりまで）。列挙に無い読み飛ばしは作らない。IR の置き場のサブディレクトリは除外でなく読む | docs/decision/records/2026-09-16-ir-tree.md#A1, docs/decision/records/records.md#A100, docs/decision/records/records.md#A102, docs/decision/records/records.md#A110, docs/decision/records/records.md#A145, docs/decision/records/records.md#A156, docs/decision/records/records.md#A165, docs/decision/records/2026-09-17-record-form.md#A14, docs/decision/records/2026-09-17-record-form.md#A24, docs/decision/records/2026-09-17-record-form.md#A27, docs/decision/records/2026-09-17-record-form.md#A34, docs/decision/records/2026-09-17-record-form.md#A35, docs/decision/records/2026-09-17-record-form.md#A45 |
| コードブロック | 行頭の3つ以上の "`" か "~" で始まる行から、同じ文字で同じ数以上の行までの部分 | docs/decision/records/records.md#A108 |
