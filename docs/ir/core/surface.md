# 面の検査

設定の "surface.rules" と "surface.files" でコードから`面`を取り出し、`面`の名前が`IR`に出てくるかを "kotowari check" が確かめるところと、`面`の数の出し方を扱う。`未記載の面の一覧`の置き場と1件の扱いは surface-unspecified.md で扱う。

## Requirements

### REQ-core-223: 面の取り出し

- kind: ubiquitous
- source: docs/decision/records/2026-09-27-surface-check.md#A1, docs/decision/records/2026-09-27-surface-check.md#A5, docs/decision/records/2026-09-27-surface-check.md#A8, docs/decision/records/2026-09-27-surface-check.md#A19, docs/decision/records/2026-09-27-surface-check.md#A20
- verification: unit

kotowari は常に、"surface.rules" が空の一覧でないとき、`面のファイル`のうち拡張子から TBL-core-031 のとおりに決まる言語が`面の規則`の "language" と同じものにその`面の規則`を当て、`面の規則`が当たった構文木の節1つを1つの`面`とする。"language" は REQ-core-189 と同じく大文字小文字を区別せずに突き合わせ、ast-grep の別名（"ts"、"py"）も受ける。`面`の種類はその`面の規則`の "id"、名前はメタ変数 "$NAME" に入った節の文字である。その文字の最初と最後が同じ引用符（一重引用符、二重引用符、バッククォートのいずれか）なら、その1文字ずつを外し、それ以外の形はそのまま名前にする。"$NAME" を捕まえない当たりは`面`にしない。kotowari は`面の規則`を同梱しない。

### REQ-core-224: 面の規則と面のファイルの読み方

- kind: ubiquitous
- source: docs/decision/records/2026-09-27-surface-check.md#A1, docs/decision/records/2026-09-27-surface-check.md#A20, docs/decision/records/2026-09-27-surface-check.md#A23, docs/decision/records/2026-09-27-surface-check.md#A25
- verification: unit

kotowari は常に、"surface.rules" に並んだパスを`基準のディレクトリ`からの相対パスとして読み、その中身を REQ-core-186 と同じく ast-grep のルールの YAML（"---" で区切って複数の`面の規則`を並べてよい）として読み、`面の規則`を`問い合わせ`に加えない。`面の規則`の "files" と "ignores" は REQ-core-187 と同じ読み方で`面のファイル`の`基準のディレクトリ`からの相対パスに当て、"fix"、"message"、"severity"、"note"、"metadata" を`面`の取り出しに使わず、"severity" が "off" の`面の規則`も当てる。"surface.files" の glob の読み方、走査、`除外`、読めないファイルと UTF-8 でないファイルと先の無いシンボリックリンクでの`停止`は、"tests.files" と`テストのファイル`のとおりにする（REQ-core-019、REQ-core-079、REQ-core-018）。

### REQ-core-236: 木にする面のファイル

- kind: ubiquitous
- source: docs/decision/records/2026-09-27-surface-check.md#A20, docs/decision/records/2026-09-27-surface-check.md#A23
- verification: unit

kotowari は常に、`面のファイル`のうち拡張子から TBL-core-031 のとおりに決まる言語がどれかの`面の規則`の "language" と同じものだけを tree-sitter で読み、構文の誤りが1つでもあるとき REQ-core-083 のとおり unparsable_file の`誤り`を出してそのファイルを飛ばす。同じパスに`テストのファイル`として unparsable_file を出したときは重ねて出さない。ほかの`面のファイル`は tree-sitter で読まず、unparsable_file を出さない。

### REQ-core-225: 面の設定の誤り

- kind: event_driven
- source: docs/decision/records/2026-09-27-surface-check.md#A12, docs/decision/records/2026-09-27-surface-check.md#A24
- verification: unit

"surface.rules" が空の一覧でなく "surface.files" が空の一覧のとき、"surface.files" が空の一覧でなく "surface.rules" が空の一覧のとき、または設定に "surface.unspecified" の鍵があって "surface.rules" が空の一覧のとき、kotowari は`設定ファイル`を読むどのコマンドでも設定の誤りを理由に`停止`する。"kotowari check" と "kotowari status" で、"surface.rules" のパスのファイルが REQ-core-189 の条件のどれかに当たるとき、kotowari は "tests.rules" と同じ理由と文言で`停止`する。

### REQ-core-226: IR にある面

- kind: ubiquitous
- source: docs/decision/records/2026-09-27-surface-check.md#A2, docs/decision/records/2026-09-27-surface-check.md#A13, docs/decision/records/2026-09-27-surface-check.md#A18
- verification: unit

kotowari は常に、`面`の名前が、どこかの`話題ごとの文書`の`要求`の`文`、`決定表`の表のセル（見出しの行のセルを含む）、`シナリオ`のステップの行のどれかに、二重引用符の対かバッククォートの対で囲んだ中身として出てくるとき、その`面`を`IR`にあるとする。二重引用符は行の左から順に対にし、バッククォートは REQ-core-064 のとおり二重引用符の外のものを対にする。囲んだ中身が前後の空白を除かずに名前と同じ文字列のときだけ出てくるとし、名前が中身の一部であるだけでは出てくるとしない。`後回し`の`要求`の`文`に出てくる名前も数える。`性質`の`文`、`文書が扱う範囲`の行、`項目`の見出しの下の "- " で始まる行（"- how_to_verify:" の行を含む）、`問題の記録`の本文、`用語集`に出てくる名前は数えない。

### REQ-core-227: IR に無い面の指摘

- kind: event_driven
- source: docs/decision/records/2026-09-27-surface-check.md#A3, docs/decision/records/2026-09-27-surface-check.md#A4, docs/decision/records/2026-09-27-surface-check.md#A7, docs/decision/records/2026-09-27-surface-check.md#A14, docs/decision/records/2026-09-27-surface-check.md#A15, docs/decision/records/2026-09-27-surface-check.md#A19
- verification: unit

"kotowari check" か "kotowari status" で、`面`が`IR`になく、`未記載の面の一覧`の形の正しいどの1件にも一致しないとき、kotowari は種類と名前の同じ`面`ごとに1件だけ、"path" をそのうち`面のファイル`のパスのバイト順、次に行の小さい順で最初の`面`の`面のファイル`、"line" をその`面`の節の最初の行、detail を種類と名前を1つの半角空白で区切った文字列にして surface_without_spec の`誤り`を出す。"surface.rules" が空の一覧のとき、kotowari は`面`を取り出さず、この`誤り`を出さない。

### REQ-core-228: 外した件数の出力

- kind: event_driven
- source: docs/decision/records/2026-09-27-surface-check.md#A3, docs/decision/records/2026-09-27-surface-check.md#A4, docs/decision/records/2026-09-27-surface-check.md#A16
- verification: unit

"kotowari check" で "surface.rules" が空の一覧でないとき、kotowari は JSON の最上位の "surface" に "unspecified" の鍵1つを持つオブジェクトを出し、"--format" が "text" のときは`指摘`の行の後の最後の1行に "surface: unspecified=数" を出す。数は、`IR`になく`未記載の面の一覧`の形の正しい1件に一致した`面`の、種類と名前の組の数である。`指摘`が0件でも、数が 0 でも出す。"surface.rules" が空の一覧のときは、JSON の "surface" の鍵も文字の行も出さない。

### REQ-core-229: 面を読むコマンド

- kind: ubiquitous
- source: docs/decision/records/2026-09-27-surface-check.md#A10, docs/decision/records/2026-09-27-surface-check.md#A14, docs/decision/records/2026-09-27-surface-check.md#A24
- verification: unit

kotowari は常に、`面のファイル`と`面の規則`のファイルと`未記載の面の一覧`を "kotowari check" と "kotowari status" でだけ読み、"kotowari status" では check と同じ`面`の検査を行って TBL-core-028 の "surface" の数を出す。"kotowari list"、"kotowari query"、"kotowari plan"、"kotowari mutants" はそれらを読まず、その読み込みによる`停止`をしない。"kotowari list" の出力に`面`の数を出さない。

### REQ-core-230: スキルの面の書き方

- kind: ubiquitous
- source: docs/decision/records/2026-09-27-surface-check.md#A5, docs/decision/records/2026-09-27-surface-check.md#A11, docs/decision/records/2026-09-27-surface-check.md#A15
- verification: review
- how_to_verify: "agent/skills/kotowari/references/surface.md" があり、面の検査の書き方と直し方と、面の規則の書き方の例として clap のフラグとサブコマンドを取り出す規則を載せていること、"agent/skills/kotowari/references/config.md" に "surface.files"、"surface.rules"、"surface.unspecified" の鍵が、"agent/skills/kotowari/references/findings.md" に surface_without_spec、surface_unspecified_invalid、surface_unspecified_stale の種類が載っていることを確かめる

"agent/skills/" の下の kotowari スキルは常に、`面`の検査の書き方と直し方を独立した reference に持ち、`面の規則`の書き方の例を載せ、`面`の設定の鍵と`指摘`の種類を既存の reference に載せる。

## Examples

```gherkin
@id=EX-core-407 @about=REQ-core-223,REQ-core-226,REQ-core-228 @source=docs/decision/records/2026-09-27-surface-check.md#A1,docs/decision/records/2026-09-27-surface-check.md#A2,docs/decision/records/2026-09-27-surface-check.md#A4,docs/decision/records/2026-09-27-surface-check.md#A8,docs/decision/records/2026-09-27-surface-check.md#A16,docs/decision/records/2026-09-27-surface-check.md#A19
Scenario: IR の要求の文に名前が出てくる面は指摘にならない
  Given "surface.files" が "src/**/*.rs" で、"surface.rules" の "language: rust" で "id: flag" の`面の規則`が "src/cli.rs" の3行目の文字列のリテラル "--format" に当たってそれを "$NAME" に捕まえ、`要求`の`文`に "--format" を二重引用符で囲んで書いてある
  When "kotowari check --format json" を実行する
  Then surface_without_spec の誤りは出ず、JSON の "surface" の "unspecified" は 0 である

@id=EX-core-408 @about=REQ-core-227 @source=docs/decision/records/2026-09-27-surface-check.md#A3,docs/decision/records/2026-09-27-surface-check.md#A7,docs/decision/records/2026-09-27-surface-check.md#A15
Scenario: IR に出てこない面は面の場所に誤りになる
  Given EX-core-407 の`面の規則`が "src/cli.rs" の12行目の文字列のリテラル "--verbose" にも当たり、`IR`のどこにも "--verbose" を囲んだ中身が無く、`未記載の面の一覧`は無い
  When "kotowari check --format json" を実行する
  Then "path" が "src/cli.rs"、"line" が 12、detail が "flag --verbose" の surface_without_spec の誤りが1件出て、終了コードは 1 である

@id=EX-core-409 @about=REQ-core-227 @source=docs/decision/records/2026-09-27-surface-check.md#A7,docs/decision/records/2026-09-27-surface-check.md#A15,docs/decision/records/2026-09-27-surface-check.md#A19
Scenario: 同じ面が2か所にあっても最初の1か所に1件だけ
  Given EX-core-408 の場面で、"src/cli.rs" の12行目と30行目の両方に文字列のリテラル "--verbose" がある
  When "kotowari check --format json" を実行する
  Then detail が "flag --verbose" の surface_without_spec の誤りは "line" が 12 の1件だけである

@id=EX-core-410 @about=REQ-core-226 @source=docs/decision/records/2026-09-27-surface-check.md#A2,docs/decision/records/2026-09-27-surface-check.md#A15,docs/decision/records/2026-09-27-surface-check.md#A18
Scenario: 名前が囲みの中身の一部であるだけでは IR にあるとしない
  Given EX-core-408 の場面で、`要求`の`文`に "--verbose true" を二重引用符で囲んで書いてある
  When "kotowari check --format json" を実行する
  Then detail が "flag --verbose" の surface_without_spec の誤りが出る

@id=EX-core-411 @about=REQ-core-226 @source=docs/decision/records/2026-09-27-surface-check.md#A2,docs/decision/records/2026-09-27-surface-check.md#A13,docs/decision/records/2026-09-27-surface-check.md#A18
Scenario: 決定表のセルと後回しの要求の文とシナリオのステップに出てくる名前も数える
  Given `面の規則`が名前 "list"、"query"、"plan" の3つの`面`を取り出し、"list" は`決定表`のセルに、"query" は`後回し`の`要求`の`文`に、"plan" は`シナリオ`のステップの行に、それぞれ二重引用符で囲んで書いてある
  When "kotowari check --format json" を実行する
  Then surface_without_spec の誤りは出ない

@id=EX-core-412 @about=REQ-core-226 @source=docs/decision/records/2026-09-27-surface-check.md#A2,docs/decision/records/2026-09-27-surface-check.md#A18
Scenario: バッククォートで囲んだ用語の名前も数える
  Given `用語集`に用語 "status" があり、`面の規則`が名前 "status" の`面`を取り出し、`要求`の`文`にその用語をバッククォートで囲んで書いてある
  When "kotowari check --format json" を実行する
  Then surface_without_spec の誤りと unknown_term の誤りは出ない

@id=EX-core-413 @about=REQ-core-227,REQ-core-228 @source=docs/decision/records/2026-09-27-surface-check.md#A3,docs/decision/records/2026-09-27-surface-check.md#A15,docs/decision/records/2026-09-27-surface-check.md#A16
Scenario: 面の規則を書かないプロジェクトには何も起きない
  Given 設定に "surface.rules" と "surface.files" の鍵が無い
  When "kotowari check --format text" を実行する
  Then surface_without_spec の誤りは出ず、"surface: " で始まる行も出ない

@id=EX-core-414 @about=REQ-core-228 @source=docs/decision/records/2026-09-27-surface-check.md#A4,docs/decision/records/2026-09-27-surface-check.md#A16
Scenario: 外した件数は指摘が0件でも最後の行に出る
  Given EX-core-407 の場面で、ほかに`指摘`が無い
  When "kotowari check --format text" を実行する
  Then 標準出力は "surface: unspecified=0" の1行だけである

@id=EX-core-415 @about=REQ-core-225 @source=docs/decision/records/2026-09-27-surface-check.md#A12,docs/decision/records/2026-09-27-surface-check.md#A24
Scenario: 規則だけあって探すファイルが空なら停止する
  Given "surface.rules" が "rules/surface.yml" を持ち、"surface.files" の鍵が無い
  When "kotowari check" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は "config error: " で始まる

@id=EX-core-416 @about=REQ-core-225 @source=docs/decision/records/2026-09-27-surface-check.md#A12,docs/decision/records/2026-09-27-surface-check.md#A24
Scenario: 探すファイルだけあって規則が空なら停止する
  Given "surface.files" が "src/**/*.rs" を持ち、"surface.rules" の鍵が無い
  When "kotowari check" を実行する
  Then 終了コードは 2 である

@id=EX-core-417 @about=REQ-core-225,REQ-core-229 @source=docs/decision/records/2026-09-27-surface-check.md#A12,docs/decision/records/2026-09-27-surface-check.md#A14,docs/decision/records/2026-09-27-surface-check.md#A24
Scenario: 規則のファイルが無いと check は停止し、list は停止しない
  Given "surface.files" が "src/**/*.rs" で、"surface.rules" が "rules/missing.yml" を持ち、そのファイルが無い
  When "kotowari check" と "kotowari list" を実行する
  Then check の終了コードは 2 で、list の終了コードは 0 である

@id=EX-core-418 @about=REQ-core-223 @source=docs/decision/records/2026-09-27-surface-check.md#A1,docs/decision/records/2026-09-27-surface-check.md#A15,docs/decision/records/2026-09-27-surface-check.md#A20
Scenario: 言語の違う面のファイルには規則を当てない
  Given "surface.files" が "src/**" で、"language: rust" の`面の規則`が1つだけあり、"src/a.ts" に同じ形の文字列がある
  When "kotowari check --format json" を実行する
  Then "src/a.ts" に surface_without_spec の誤りは出ない

@id=EX-core-426 @about=REQ-core-236 @source=docs/decision/records/2026-09-27-surface-check.md#A20,docs/decision/records/2026-09-27-surface-check.md#A23
Scenario: 規則の言語でない面のファイルは木にせず誤りにしない
  Given "surface.files" が "src/**" で、`面の規則`は "language: rust" だけで、"src/notes.py" に Python の構文の誤りがある
  When "kotowari check --format json" を実行する
  Then "src/notes.py" に unparsable_file の誤りは出ない

@id=EX-core-427 @about=REQ-core-236 @source=docs/decision/records/2026-09-27-surface-check.md#A20,docs/decision/records/2026-09-27-surface-check.md#A23
Scenario: 規則の言語の面のファイルの構文の誤りは unparsable_file になる
  Given "surface.files" が "src/**/*.rs" で、`面の規則`は "language: rust" で、"src/bad.rs" に Rust の構文の誤りがあり、"tests.files" には当たらない
  When "kotowari check --format json" を実行する
  Then "path" が "src/bad.rs" の unparsable_file の誤りが1件出る

@id=EX-core-428 @about=REQ-core-225,REQ-core-231 @source=docs/decision/records/2026-09-27-surface-check.md#A24
Scenario: 規則が無いのに一覧の鍵だけあると list でも停止する
  Given 設定に "surface.unspecified" が "docs/surface.yaml" で、"surface.rules" と "surface.files" の鍵が無い
  When "kotowari list" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は "config error: " で始まる

@id=EX-core-429 @about=REQ-core-226 @source=docs/decision/records/2026-09-27-surface-check.md#A2,docs/decision/records/2026-09-27-surface-check.md#A15,docs/decision/records/2026-09-27-surface-check.md#A18
Scenario: 性質の文と how_to_verify の行にだけある名前は IR にあるとしない
  Given EX-core-408 の場面で、"--verbose" を二重引用符で囲んで書いたのは`性質`の`文`と`要求`の "- how_to_verify:" の行だけである
  When "kotowari check --format json" を実行する
  Then detail が "flag --verbose" の surface_without_spec の誤りが出る
```
