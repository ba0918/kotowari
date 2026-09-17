kotowari の仕様に基づく（改訂 2026-09-17。本体の版は固定しない）

終了コードを見る。2 なら停止。標準出力は空なので読まない。標準エラーの 1 行目の文言で下の停止の表を引く。0 か 1 なら標準出力の JSON を読み、`findings` の各件を下の表で引く。`severity` が `notice` の件（注意）は終了コードを変えない。

`tests` は、読んだテストのファイルを拡張子ごとにまとめ、その拡張子のファイルの数（`files`）と、その拡張子が問い合わせのある言語か（`query`）を持つ。`query` が true の拡張子（第1版では `rs` だけ）は、tree-sitter でテストの関数を見つけているので、印の無いテストが test_without_id で出る。`query` が false の拡張子は問い合わせの無い言語で、ファイルの文字の中の印をすべて拾い、requirement_without_test と scenario_without_test を消す側に数えるだけで、テストの数は見ない。その言語のテストに印が無くても指摘は出ないので、`query` が false の拡張子の分は検査が届いていない。

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
| record_field_missing | `## Context` の見出しを持つ判断の記録で、番号の行に必須の補足の行が無いか値が空（決定の節は `why`、Undecided は `decides`、Superseded は `superseded_by`） | detail の名前の補足の行をその行の下に足す。値が残っていなければ `not recorded` と書く（理由を後から発明しない）。`superseded_by` だけは改めた先の決定へのリンクが要り、`not recorded` にすると revision_link_invalid になる | brainstorm |
| record_field_unknown | `## Context` の見出しを持つ判断の記録で、補足の行の名前が `why`、`rejected`、`decided_by`、`superseded_by`、`decides`、`related` の6つ以外 | 名前を6つのどれかに直すか、その行を補足の行でない形にする | brainstorm |
| revision_link_invalid | すべての判断の記録で、`superseded_by` の行に `[文字](href)` のリンクが無い、href が `#` と決定の番号を持たない、パスが置き場の外、先が判断の記録でない、先の決定の節と Superseded の節に番号の行が無い | detail の href（リンクが無いときは行の値）を、置き場の中の記録の決定の番号を指す `[文字](href)` のリンクに直す。href はその記録のファイルからの相対パス（出典の形の、基準のディレクトリからのパスではない） | brainstorm |
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
| scenario_without_test | @about に検証が review 以外の要求を持つ具体例に、その ID を含む印が1つも無い（review だけの具体例と要求を挙げない具体例には出ない） | その場面を確かめるテストを書いて印を付ける | implementer（path は IR だが直すのは implementer） |
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
| mutant_survived | `kotowari mutants` で、変異を入れてもテストが全部通った（見逃し。detail は変更の説明） | mutants.md の3分類で調べる | implementer |
| mutant_timeout | `kotowari mutants` で、変異を入れるとテストが時間内に終わらなかった（注意。終了コードは変えない。detail は変更の説明） | 無限ループになる変異で出る。テストが捕まえたとは言い切れないが、壊れていることは観測できている | implementer |
| equivalent_stale | 等価の一覧の1件の `text` の文面が、その `file` のどこにも無い（注意。終了コードは変えない。detail は一覧に書かれたままの `file` と `change`） | その1件の判断をし直す。今のコードで変異がまだ出るなら `text` を今の行の文面に直し、出ないなら1件を消す | implementer |
| equivalent_invalid | 等価の一覧の1件の形が正しくない（detail は一覧に書かれたままの `file` と `change`） | mutants.md の一覧の形に直す。この1件は何も外していない | implementer |

除外の追加や規則の緩めは仕様の変更になるため、スキルの中で決めず brainstorm に戻す。

| 文言 | 対処 |
|---|---|
| config error | 人に返す。設定ファイルの誤りを伝える |
| argument error | 自分で直す。コマンドの引数を確かめる |
| unreadable file | 人に返す。読めないファイルのパスを伝える |
| non-UTF-8 file | 人に返す。UTF-8 でないファイルのパスを伝える |
| results error | 人に返す。結果のファイルが変異テストの道具の形に合っていない。詳細のパスと説明を伝える |
