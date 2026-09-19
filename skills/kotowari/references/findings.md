kotowari の仕様に基づく（改訂 2026-09-20。本体の版は固定しない）

終了コードを見る。2 なら停止。標準出力は空なので読まない。標準エラーの 1 行目の文言で下の停止の表を引く。0 か 1 なら標準出力の JSON を読み、`findings` の各件を下の表で引く。`severity` が `notice` の件（注意）は終了コードを変えない。

`tests` は、読んだテストのファイルを拡張子ごとにまとめ、その拡張子のファイルの数（`files`）と、その拡張子が問い合わせのある言語か（`query`）を持つ。`query` が true の拡張子（第1版では `rs` だけ）は、tree-sitter でテストの関数を見つけているので、印の無いテストが test_without_id で出る。`query` が false の拡張子は問い合わせの無い言語で、ファイルの文字の中の印をすべて拾い、requirement_without_test と scenario_without_test を消す側に数えるだけで、テストの数は見ない。その言語のテストに印が無くても指摘は出ないので、`query` が false の拡張子の分は検査が届いていない。

`kotowari mutants` の JSON の最上位は `findings`、`counts`、`mutants` の3つだけで、`files`、`lines`、`tests` は出ない。指摘の形と終了コードの決まりは check と同じで、下の表で引く。`mutants` の集計の読み方と、mutant_survived を調べる手順は mutants.md。

`kotowari list` は check と同じ設定と置き場から IR とテストのファイルを読み、読めた項目とシナリオを標準出力に出す。 出力は項目の数に比例して大きい（要求 150 件で JSON は 200KB を超える）ので、LLM が読むときは `jq` で要る鍵だけを取る（例: `kotowari list | jq -r '.items[] | select(.tests == []) | .id'`）。指摘は出さず、終了コードは読めれば 0 で、2 になるのは停止したときだけ。停止の理由と文言は check と同じで、下の停止の表で引く。IR に誤りがあっても読めた項目は出るので、list は check の代わりにならない。

`kotowari list` の JSON の最上位は `items` だけで、`findings` も `counts` も `tests` の集計も出ない。`items` の1件が持つ鍵は項目の種類で決まる。`kind` は `requirement`、`table`、`property`、`scenario`、`flag` のいずれかで、どの種類も `id`、`kind`、`name`、`path`、`line`、`sources`、`tests` を持つ。要求はさらに `type`、`verification`、`definition`、`examples`、`how_to_verify` を、決定表と性質は `examples`（ID の昇順）を、問題の記録は `type` と `relations` を持つ。値が無いときは鍵ごと消えず、null か空の並びになる。`how_to_verify` は要求の `- 確かめ方:` の行の値で、検証が review の要求を人がどう確かめるかはここにある。手で保つ対応表は無い。

1件の `tests` は、その ID を印に含むテストの並びで、同じテストに同じ ID の印が複数あれば印の出現ごとに1件。1件は `path`（テストのファイルの基準のディレクトリからの相対パス）、`line`（印のある行。テストの関数の行ではない）、`name`（テストの関数の名前）を持つ。`name` が null の件は問い合わせの無い言語のファイルの印で、そこにテストの関数の名前は無い。

`--format text` では、1つの項目が `ID 検証 名前 パス:行 tests=数` の1行になり（`検証` は要求以外と、`- 検証:` の行の無い要求では `-`）、その直後に `tests` の1件ごとに半角空白2つで字下げした `パス:行 名前` の行が続く（`名前` が null のときは `-`）。項目は `path` の昇順、同じ `path` の中は `line` の昇順で、1件の `tests` も同じ順に並ぶ。

`kotowari query <ID>` は list の1件を詳しくしたもの。読むものと停止の決まりは list と同じで、JSON の最上位も `items` だけ。1件は list の1件の鍵に `body` と `referenced_by` が増えた形で、同じ ID の項目が複数あれば全部出る。`body` は IR の生の行の並びで、項目は見出しの次の行から次の `### ` か `## ` の見出しの前まで、シナリオは `@id` のタグの行から最後のステップまで（先頭と末尾の空の行は落とす）。`referenced_by` はその ID を指している項目とシナリオの並びで、1件は `id`、`kind`、`path`、`line`（指している側の見出しの行。シナリオは `Scenario:` の行）と `via` を持ち、`path` の昇順、同じ `path` の中は `line` の昇順に並ぶ。`via` は `definition`（`- 定義:` の行）、`relations`（`- 関係:` の行）、`about`（`@about` のタグ）、`text`（要求や性質の文、シナリオのステップの中で、二重引用符の外でバッククォートで囲んだ ID。check の unresolved_reference と同じ判定で、地の文や引用符の中の ID は拾わない）のいずれか。位置引数はちょうど1つの ID で、その ID を持つ項目もシナリオも無ければ `argument error: unknown id: ` に続けてその文字を出して停止する。`--format text` では list と同じ1行目とテストの行に続けて、`body` の各行を半角空白2つで字下げし、最後に `referenced_by` の1件ごとに `  <- ID via パス:行` の行を出す。

`kotowari status` は IR が揃っているかを数と真偽で出す。読むものと停止の決まりは check と同じで、指摘は出さず数だけを出す。JSON の最上位は `documents`（文書の数と行数）、`items`（種類ごとの項目の数）、`requirements`（検証の値ごとの数、テストのある要求とない要求の数、review の要求の確かめ方の有無、具体例の無い要求の数）、`scenarios`（テストのある具体例とない具体例の数）、`tests`（印の数と読んだテストのファイル）、`findings`（check の誤りと注意の数）、`complete` の7つ。`complete` が true になるのは check の誤りが 0 件で、かつ問題の記録の項目が 0 件のときだけで、終了コードは complete なら 0、そうでなければ 1、停止は 2。数え方は check と揃えてある。`complete` が false の理由は、`findings` の `error` が 0 でなければ `kotowari check` の指摘に1件ずつあり、`items` の `flag` が 0 でなければ問題の記録（FLAGS.md）にある。`--format text` では群ごとに `群名 鍵=値 鍵=値` の1行が上の順に出て、最後の行は `complete true` か `complete false`。

検証が review の要求には `- 確かめ方:` の行が要る。テストの無い要求を人か LLM がどう確かめるかはこの行にしか書けないので、行が無いか値が空なら missing_field の誤りが detail `確かめ方` で出る。

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
