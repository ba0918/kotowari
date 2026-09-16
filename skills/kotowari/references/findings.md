kotowari の仕様に基づく（改訂 2026-09-16。本体の版は固定しない）

終了コードを見る。2 なら停止。標準出力は空なので読まない。標準エラーの 1 行目の文言で下の停止の表を引く。0 か 1 なら標準出力の JSON を読み、`findings` の各件を下の表で引く。`severity` が `notice` の件（注意）は終了コードを変えない。

| 種類 | 意味 | 対処 | 担当 |
|---|---|---|---|
| missing_title | 題名が無い | `# ` の題名を足す | brainstorm |
| multiple_titles | 題名が2つ以上 | 題名を1つにする | brainstorm |
| missing_scope | 話題ごとの文書に範囲の行が無い | 題名の後に文書が扱う範囲の行を足す | brainstorm |
| unknown_heading | `### ` の見出しが `### ID: 名前` の形でない | 見出しの形を直す | brainstorm |
| unknown_field | 見出しの下に知らない行がある | 行を取り除くか正しい形に直す | brainstorm |
| missing_field | 必須の行が無い（detail に行の名前） | 足りない行を足す | brainstorm |
| missing_table | 決定表に表が無い | Markdown の表を足す | brainstorm |
| duplicate_field | 同じ行が2つ以上 | 重複した行を1つにする | brainstorm |
| missing_source | 出典が無いか空 | `- 出典:` を足す | brainstorm |
| source_invalid | 出典の先が正しくない | 出典の書式を直すか判断の記録の決定を確かめる | brainstorm |
| unknown_term | バッククォートで囲んだ語が用語集にも ID にもない | 用語集に足すか、値なら二重引用符に変える | brainstorm |
| missing_document | 文書名の参照先が無い | 文書を作るか参照を直す | brainstorm |
| missing_statement | 要求か性質に文が無い | 文を足す | brainstorm |
| verification_missing | 要求に検証の行が無い | `- 検証:` を足す | brainstorm |
| verification_invalid | 検証の値が正しくない | unit、property、proof、review のいずれかにする | brainstorm |
| unknown_kind | 種類の値が正しくない | 正しい値にする | brainstorm |
| duplicate_id | 同じ ID が2か所以上 | ID を一意にする | brainstorm |
| unresolved_reference | IR の文書で存在しない ID を参照（定義、@about、関係、文中の ID） | ID を直すか定義を足す | brainstorm |
| unresolved_reference | テストの印で存在しない ID を参照（path がテストのファイル） | 印の ID を存在するものに直す | implementer |
| algorithm_without_definition | algorithm の要求に定義が無い | `- 定義:` で決定表か性質を指す | brainstorm |
| missing_tag | シナリオに `@id` か `@about` が無い | タグを足す | brainstorm |
| unknown_tag | 知らないタグ | `@id`、`@about`、`@source` のいずれかに直す | brainstorm |
| vague_word | 曖昧語が含まれる | 具体的な語に言い換える | brainstorm |
| requirement_without_test | review 以外の要求にテストが無い | テストを書いて印を付ける | implementer（path は IR だが直すのは implementer） |
| test_without_id | テストに印が無い | `@kotowari[ID]` の印を付ける | implementer |
| invalid_marker | 印の形が正しくない | `@kotowari[ID, ...]` の形に直す | implementer |
| unparsable_file | テストファイルを読めない | テストの構文誤りを直す | implementer |
| unclosed_code_block | コードブロックが閉じていない | コードブロックを閉じる | brainstorm |
| invalid_gherkin_line | gherkin に許されない行 | タグ、Scenario、ステップ、注釈、空行だけにする | brainstorm |
| invalid_id | `@id` の値が `EX-nnn` の形でない | 値を `EX-nnn` の形にする | brainstorm |
| glossary_invalid | 用語集に正しい表が無い | `| 用語 | 意味 | 出典 |` のヘッダと区切り行を足す | brainstorm |
| unclosed_backtick | バッククォートが奇数 | バッククォートを閉じる | brainstorm |
| invalid_glossary_row | 用語集の表の行の形が崩れている | セルを3つにし用語を空にしない | brainstorm |
| duplicate_term | 同じ用語集の中か、連鎖の根に近い用語集との重複（根から遠い側の行に出る） | 根から遠い側の行を消すか、別の語に言い換える | brainstorm |
| too_many_lines | 文書の行数が上限を超えた（注意。終了コードは変えない） | 分割の指示ではない。責務の混在を疑って読み直し、範囲の行の外の要求が混じっていれば範囲で説明できる単位に分け、混じっていなければ理由を判断の記録に書いて残す。行数を理由に切らない。用語集はディレクトリごとのまとまりに分けられるならその `CONTEXT.md` に分け、分けられないなら設定の limits.lines を上げてその判断を記録に書く | brainstorm |
| too_many_requirements | 話題ごとの文書の要求の数が上限を超えた（注意。終了コードは変えない） | too_many_lines と同じ。件数を理由に切らない | brainstorm |

除外の追加や規則の緩めは仕様の変更になるため、スキルの中で決めず brainstorm に戻す。

| 文言 | 対処 |
|---|---|
| config error | 人に返す。設定ファイルの誤りを伝える |
| argument error | 自分で直す。コマンドの引数を確かめる |
| unreadable file | 人に返す。読めないファイルのパスを伝える |
| non-UTF-8 file | 人に返す。UTF-8 でないファイルのパスを伝える |
