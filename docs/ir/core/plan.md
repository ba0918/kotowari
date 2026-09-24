# 計画書の検査

"kotowari plan" が`計画書`を1つ読み、本体に同梱したスキーマで形を検査するところを扱う。

## Requirements

### REQ-core-190: plan の引数

- kind: event_driven
- source: docs/decision/records/2026-09-24-plan-schema.md#A10, docs/decision/records/2026-09-24-plan-schema.md#A15, docs/decision/records/2026-09-24-plan-schema.md#A16
- verification: unit

"--help" も "--version" も無い "kotowari plan" で、"plan" の後の位置引数がちょうど1つでないとき、または "--config" を受けたとき、kotowari は引数の誤りを理由に`停止`する。位置引数は`計画書`のファイルのパスで、カレントディレクトリからの相対パスとして読む。

### REQ-core-196: plan が読むもの

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-plan-schema.md#A9, docs/decision/records/2026-09-24-plan-schema.md#A15, docs/decision/records/2026-09-24-plan-schema.md#A32
- verification: unit

kotowari は常に、"kotowari plan" で`計画書`のファイルだけを読み、`設定ファイル`、`IR`、`判断の記録`、`テストのファイル`を読まない。

### REQ-core-197: 計画書を読めないとき

- kind: event_driven
- source: docs/decision/records/2026-09-24-plan-schema.md#A16
- verification: unit

"kotowari plan" で`計画書`のファイルが無いか、ディレクトリか、読めないとき、kotowari は読めないファイルを理由に`停止`し、`計画書`が UTF-8 でないとき、UTF-8 でないファイルを理由に`停止`する。

### REQ-core-191: 同梱のスキーマで読む

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-plan-schema.md#A9, docs/decision/records/2026-09-24-plan-schema.md#A12, docs/decision/records/2026-09-24-plan-schema.md#A14, docs/decision/records/2026-09-24-plan-schema.md#A27
- verification: unit

kotowari は常に、"kotowari plan" で`計画書`をコンパイル時に取り込んだ`計画書`のスキーマで読み、実行時にスキーマのファイルを読まず、`計画書`の先頭の frontmatter を中身を問わず読まずに飛ばし、形の宣言として使わず、`指摘`にも`停止`にもしない。

### REQ-core-192: 計画書の形

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-plan-schema.md#A3, docs/decision/records/2026-09-24-plan-schema.md#A7, docs/decision/records/2026-09-24-plan-schema.md#A19, docs/decision/records/2026-09-24-plan-schema.md#A20, docs/decision/records/2026-09-24-plan-schema.md#A21, docs/decision/records/2026-09-24-plan-schema.md#A22, docs/decision/records/2026-09-24-plan-schema.md#A23, docs/decision/records/2026-09-24-plan-schema.md#A24, docs/decision/records/2026-09-24-plan-schema.md#A25, docs/decision/records/2026-09-24-plan-schema.md#A28, docs/decision/records/2026-09-24-plan-schema.md#A29, docs/decision/records/2026-09-24-plan-schema.md#A31, docs/decision/records/2026-09-24-plan-schema.md#A11, docs/decision/records/2026-09-24-plan-schema.md#A10
- verification: unit

kotowari は常に、"kotowari plan" で`計画書`を1行ずつ読み、次の形を満たす`計画書`だけを`誤り`の無いものとする。`題名`がちょうど1つあり、`題名`と最初の "## " の見出しの間に空でない行が無い。"## Goal"、"## Specification"、"## Approach and why"、"## Scope of change"、"## Step order and prerequisites"、"## Verification map"、"## Left to the implementer"、"## Stop conditions"、"## Out of scope"、"## Steps" の節がちょうど1つずつあり、"## Test command" の節は0個か1個あり、ほかの "## " の節は無い。"## Steps" の節には、"### S" と1桁以上の数字の後に ":" がある見出しのステップが1つ以上あり、最初のステップより前に空でない行もほかの見出しも無い。各ステップの下には、名前が "Purpose"、"Specification"、"Prerequisites"、"May change"、"Done when"、"Shown by"、"Left to the implementer"、"Stop and hand back if" の名前と値の形の一覧の行がこの順で1回ずつあり、ほかの空でない行は無い。一覧の記号は "-"、"*"、"+" のどれでもよい。"Shown by" の値は "test"、"check"、"artifact"、"external" のいずれかの語で始まり、その語の後は空白か値の終わりである。"## Steps" 以外の節の下には、文、番号の無い一覧とその子の一覧、表、コードブロックを置いてよく、番号付きの一覧と "### " 以下の見出しは置けない。節の並び順、ステップの番号が連番か、同じ番号のステップが2つあるか、見出しの ":" の後の名前は検査しない。

### REQ-core-193: 形の指摘

- kind: event_driven
- source: docs/decision/records/2026-09-24-plan-schema.md#A11, docs/decision/records/2026-09-24-plan-schema.md#A18, docs/decision/records/2026-09-24-plan-schema.md#A26, docs/decision/records/2026-09-24-plan-schema.md#A33, docs/decision/records/2026-09-24-guide-gaps.md#A4
- verification: unit

"kotowari plan" で`計画書`が REQ-core-192 の形から外れるとき、kotowari はスキーマの側の`指摘`を TBL-core-030 で写さず、1件ごとに、"path" を`計画書`のファイルの`基準のディレクトリ`からの相対パス（正規化し、基準の外なら "../" を含める）、"line" をスキーマの側が出した行（ステップに必須の欄が無いときはそのステップの見出しの行。行が無ければ null）、detail をスキーマの側の種類と詳細を ": " でつないだ文字列にして invalid_plan の`誤り`を出す。

### REQ-core-194: plan の JSON

- kind: event_driven
- source: docs/decision/records/2026-09-24-plan-schema.md#A17, docs/decision/records/2026-09-24-plan-schema.md#A33
- verification: unit

"kotowari plan" で "--format" が "json" のとき、kotowari は最上位に "findings" と "counts" の2つの鍵だけを持つ JSON を出す。"findings" の`指摘`の鍵と "counts" の中身は "kotowari check" と同じである。

### REQ-core-207: plan の text

- kind: event_driven
- source: docs/decision/records/2026-09-24-guide-gaps.md#A4
- verification: unit

"kotowari plan" で "--format" が "text" のとき、kotowari は`指摘`を REQ-core-025 の形で1件1行に出し、ほかの行を出さない。`指摘`が0件なら何も出さない。

## Examples

```gherkin
@id=EX-core-332 @about=REQ-core-192,REQ-core-193 @source=docs/decision/records/2026-09-24-plan-schema.md#A7,docs/decision/records/2026-09-24-plan-schema.md#A20,docs/decision/records/2026-09-24-plan-schema.md#A11
Scenario: 形の揃った計画書には指摘が出ない
  Given 必須の節がそろい、"## Steps" の下に8つの欄を順に持つステップ "### S1: 入力を読む" がある`計画書` "docs/plans/a.md" がある
  When "kotowari plan docs/plans/a.md" を実行する
  Then 終了コードは 0 で、`指摘`は出ない

@id=EX-core-333 @about=REQ-core-192,REQ-core-193 @source=docs/decision/records/2026-09-24-plan-schema.md#A7,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18,docs/decision/records/2026-09-24-plan-schema.md#A26
Scenario: 欄の欠けたステップは誤りになる
  Given カレントディレクトリが`基準のディレクトリ`で、ステップ "### S1: 入力を読む" に "- Done when:" の行が無い`計画書` "docs/plans/a.md" がある
  When "kotowari plan docs/plans/a.md" を実行する
  Then 終了コードは 1 で、invalid_plan の`誤り`が出て、その "path" は "docs/plans/a.md"、detail はスキーマの側の種類に ": " が続く形である

@id=EX-core-334 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A7,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18
Scenario: 4つの語で始まらない Shown by は誤りになる
  Given ステップの "- Shown by:" の値が "manual — 目で見る" の`計画書` "docs/plans/a.md" がある
  When "kotowari plan docs/plans/a.md" を実行する
  Then 終了コードは 1 で、invalid_plan の`誤り`が出る

@id=EX-core-335 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A7,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18
Scenario: 語の続きがある Shown by は誤りになる
  Given ステップの "- Shown by:" の値が "tests — 名前" の`計画書` "docs/plans/a.md" がある
  When "kotowari plan docs/plans/a.md" を実行する
  Then 終了コードは 1 で、invalid_plan の`誤り`が出る

@id=EX-core-336 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A20,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18
Scenario: ステップが1つも無い計画書は誤りになる
  Given "## Steps" の節に見出しが1つも無い`計画書` "docs/plans/a.md" がある
  When "kotowari plan docs/plans/a.md" を実行する
  Then 終了コードは 1 で、invalid_plan の`誤り`が出る

@id=EX-core-337 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A20,docs/decision/records/2026-09-24-plan-schema.md#A11
Scenario: Test command の節は無くてもよい
  Given "## Test command" の節が無く、ほかは形の揃った`計画書` "docs/plans/a.md" がある
  When "kotowari plan docs/plans/a.md" を実行する
  Then 終了コードは 0 である

@id=EX-core-338 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A20,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18
Scenario: 必須の節が2つある計画書は誤りになる
  Given "## Goal" の節が2つある`計画書` "docs/plans/a.md" がある
  When "kotowari plan docs/plans/a.md" を実行する
  Then 終了コードは 1 で、invalid_plan の`誤り`が出る

@id=EX-core-339 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A19,docs/decision/records/2026-09-24-plan-schema.md#A11
Scenario: 計画全体の節には表を置ける
  Given "## Verification map" の節に表を置き、ほかは形の揃った`計画書` "docs/plans/a.md" がある
  When "kotowari plan docs/plans/a.md" を実行する
  Then 終了コードは 0 である

@id=EX-core-340 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A19,docs/decision/records/2026-09-24-plan-schema.md#A23,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18
Scenario: ステップの下の地の文は誤りになる
  Given ステップ "### S1: 入力を読む" の "- Done when:" の行の直後に、空行を挟まず空でない地の文の行がある`計画書` "docs/plans/a.md" がある
  When "kotowari plan docs/plans/a.md" を実行する
  Then 終了コードは 1 で、invalid_plan の`誤り`が出る

@id=EX-core-341 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A7,docs/decision/records/2026-09-24-plan-schema.md#A21,docs/decision/records/2026-09-24-plan-schema.md#A11
Scenario: 並び順と番号の飛びは検査しない
  Given "## Goal" の節が "## Out of scope" の節の後にあり、ステップが "### S1: 入力を読む" と "### S3: 結果を出す" の`計画書` "docs/plans/a.md" がある
  When "kotowari plan docs/plans/a.md" を実行する
  Then 終了コードは 0 である

@id=EX-core-342 @about=REQ-core-190 @source=docs/decision/records/2026-09-24-plan-schema.md#A16,docs/decision/records/records.md#A20,docs/decision/records/records.md#A104
Scenario: 計画書を2つ渡すと停止する
  Given 形の揃った`計画書` "a.md" と "b.md" がある
  When "kotowari plan a.md b.md" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は "argument error: " で始まる

@id=EX-core-343 @about=REQ-core-190 @source=docs/decision/records/2026-09-24-plan-schema.md#A15,docs/decision/records/records.md#A20,docs/decision/records/records.md#A104
Scenario: plan に設定ファイルを渡すと停止する
  Given 形の揃った`計画書` "a.md" と`設定ファイル` ".kotowari/config.yaml" がある
  When "kotowari plan --config .kotowari/config.yaml a.md" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は "argument error: " で始まる

@id=EX-core-344 @about=REQ-core-197 @source=docs/decision/records/2026-09-24-plan-schema.md#A16,docs/decision/records/records.md#A20,docs/decision/records/records.md#A104
Scenario: 無い計画書は読めないファイルで停止する
  Given "docs/plans/none.md" が無い
  When "kotowari plan docs/plans/none.md" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は "unreadable file: " で始まる

@id=EX-core-345 @about=REQ-core-191 @source=docs/decision/records/2026-09-24-plan-schema.md#A9,docs/decision/records/2026-09-24-plan-schema.md#A12,docs/decision/records/2026-09-24-plan-schema.md#A27,docs/decision/records/2026-09-24-plan-schema.md#A11
Scenario: frontmatter のスキーマの宣言は使わない
  Given 先頭の frontmatter に "$schema: missing.yaml" を書き、ほかは形の揃った`計画書` "docs/plans/a.md" があり、"docs/plans/missing.yaml" は無い
  When "kotowari plan docs/plans/a.md" を実行する
  Then 終了コードは 0 である

@id=EX-core-346 @about=REQ-core-194 @source=docs/decision/records/2026-09-24-plan-schema.md#A17,docs/decision/records/2026-09-24-plan-schema.md#A18,docs/decision/records/2026-09-24-plan-schema.md#A33,docs/decision/records/2026-09-24-plan-schema.md#A20,docs/decision/records/records.md#A40,docs/decision/records/2026-09-24-plan-schema.md#A10,docs/decision/records/2026-09-24-plan-schema.md#A15
Scenario: plan の JSON は findings と counts だけを持つ
  Given ステップ "### S1: 入力を読む" に "- Done when:" の行が無い`計画書` "docs/plans/a.md" がある
  When "kotowari plan docs/plans/a.md --format json" を実行する
  Then JSON の最上位の鍵は "findings" と "counts" の2つだけで、"counts" の "invalid_plan" は "findings" の invalid_plan の数に等しい

@id=EX-core-347 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A20,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18
Scenario: 必須の節が欠けた計画書は誤りになる
  Given "## Stop conditions" の節が無い`計画書` "docs/plans/a.md" がある
  When "kotowari plan docs/plans/a.md" を実行する
  Then 終了コードは 1 で、invalid_plan の`誤り`が出る

@id=EX-core-348 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A7,docs/decision/records/2026-09-24-plan-schema.md#A20,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18,docs/decision/records/2026-09-24-plan-schema.md#A31
Scenario: 知らない節のある計画書は誤りになる
  Given "## Notes" の節がある`計画書` "docs/plans/a.md" がある
  When "kotowari plan docs/plans/a.md" を実行する
  Then 終了コードは 1 で、invalid_plan の`誤り`が出る

@id=EX-core-349 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A7,docs/decision/records/2026-09-24-plan-schema.md#A20,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18
Scenario: 欄の順が違うステップは誤りになる
  Given ステップの "- Done when:" の行が "- Purpose:" の行より前にある`計画書` "docs/plans/a.md" がある
  When "kotowari plan docs/plans/a.md" を実行する
  Then 終了コードは 1 で、invalid_plan の`誤り`が出る

@id=EX-core-350 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A20,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18
Scenario: 欄が2回あるステップは誤りになる
  Given ステップに "- Purpose:" の行が2つある`計画書` "docs/plans/a.md" がある
  When "kotowari plan docs/plans/a.md" を実行する
  Then 終了コードは 1 で、invalid_plan の`誤り`が出る

@id=EX-core-351 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A22,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18
Scenario: 最初のステップより前の行は誤りになる
  Given "## Steps" の見出しと "### S1: 入力を読む" の見出しの間に空でない地の文の行がある`計画書` "docs/plans/a.md" がある
  When "kotowari plan docs/plans/a.md" を実行する
  Then 終了コードは 1 で、invalid_plan の`誤り`が出る

@id=EX-core-352 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A7,docs/decision/records/2026-09-24-plan-schema.md#A24,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18
Scenario: S と数字で始まらないステップの見出しは誤りになる
  Given "## Steps" の下の見出しが "### Step 1: 入力を読む" の`計画書` "docs/plans/a.md" がある
  When "kotowari plan docs/plans/a.md" を実行する
  Then 終了コードは 1 で、invalid_plan の`誤り`が出る

@id=EX-core-353 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A25,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18
Scenario: 計画全体の節の番号付きの一覧は誤りになる
  Given "## Step order and prerequisites" の節に "1. S1 を先に行う" の行がある`計画書` "docs/plans/a.md" がある
  When "kotowari plan docs/plans/a.md" を実行する
  Then 終了コードは 1 で、invalid_plan の`誤り`が出る

@id=EX-core-354 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A25,docs/decision/records/2026-09-24-plan-schema.md#A11
Scenario: 計画全体の節には子の一覧を置ける
  Given "## Scope of change" の節に字下げした子の一覧を持つ箇条書きがあり、ほかは形の揃った`計画書` "docs/plans/a.md" がある
  When "kotowari plan docs/plans/a.md" を実行する
  Then 終了コードは 0 である

@id=EX-core-355 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A25,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18
Scenario: 題名と最初の節の間の行は誤りになる
  Given `題名`と "## Goal" の間に空でない地の文の行がある`計画書` "docs/plans/a.md" がある
  When "kotowari plan docs/plans/a.md" を実行する
  Then 終了コードは 1 で、invalid_plan の`誤り`が出る

@id=EX-core-356 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A28,docs/decision/records/2026-09-24-plan-schema.md#A11
Scenario: 欄の一覧の記号はアスタリスクでもよい
  Given ステップの欄の行を "* Purpose:" のように "*" で書き、ほかは形の揃った`計画書` "docs/plans/a.md" がある
  When "kotowari plan docs/plans/a.md" を実行する
  Then 終了コードは 0 である

@id=EX-core-357 @about=REQ-core-190 @source=docs/decision/records/2026-09-24-plan-schema.md#A16,docs/decision/records/records.md#A20,docs/decision/records/records.md#A104
Scenario: 計画書を渡さない plan は停止する
  When "kotowari plan" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は "argument error: " で始まる

@id=EX-core-358 @about=REQ-core-197 @source=docs/decision/records/2026-09-24-plan-schema.md#A16,docs/decision/records/records.md#A20,docs/decision/records/records.md#A104
Scenario: UTF-8 でない計画書は停止する
  Given UTF-8 でないバイトを含む`計画書` "docs/plans/a.md" がある
  When "kotowari plan docs/plans/a.md" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は "non-UTF-8 file: " で始まる

@id=EX-core-359 @about=REQ-core-196 @source=docs/decision/records/2026-09-24-plan-schema.md#A15,docs/decision/records/2026-09-24-plan-schema.md#A11
Scenario: 壊れた設定ファイルがあっても plan は停止しない
  Given YAML として読めない`設定ファイル` ".kotowari/config.yaml" と、形の揃った`計画書` "docs/plans/a.md" がある
  When "kotowari plan docs/plans/a.md" を実行する
  Then 終了コードは 0 である

@id=EX-core-360 @about=REQ-core-193 @source=docs/decision/records/2026-09-24-plan-schema.md#A26
Scenario: 基準の外の計画書の path は "../" を含む
  Given `基準のディレクトリ` "work" の外に、"- Done when:" の行が無いステップを持つ`計画書` "plans/a.md" があり、カレントディレクトリは "work" である
  When "kotowari plan ../plans/a.md" を実行する
  Then invalid_plan の`誤り`の "path" は "../plans/a.md" である

@id=EX-core-361 @about=REQ-core-191 @source=docs/decision/records/2026-09-24-plan-schema.md#A27,docs/decision/records/2026-09-24-plan-schema.md#A11
Scenario: 壊れた frontmatter も読まずに飛ばす
  Given 先頭の frontmatter が YAML として読めず、ほかは形の揃った`計画書` "docs/plans/a.md" がある
  When "kotowari plan docs/plans/a.md" を実行する
  Then 終了コードは 0 である
@id=EX-core-381 @about=REQ-core-193,REQ-core-207 @source=docs/decision/records/2026-09-24-guide-gaps.md#A4,docs/decision/records/ir-form.md#出力,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18,docs/decision/records/2026-09-24-plan-schema.md#A3
Scenario: 欄の欠けたステップは見出しの行を指す
  Given ほかは形の揃った`計画書` "docs/plans/a.md" の "### S1: 作る" のステップに "- Done when:" の行が無い
  When "kotowari plan docs/plans/a.md --format text" を実行する
  Then 終了コードは 1 で、標準出力は "docs/plans/a.md:" に "### S1: 作る" の行の番号と " [error] invalid_plan " を続けた形で始まる行だけである

@id=EX-core-382 @about=REQ-core-207 @source=docs/decision/records/2026-09-24-guide-gaps.md#A4,docs/decision/records/ir-form.md#出力
Scenario: 指摘の無い計画書の text は何も出さない
  Given 形の揃った`計画書` "docs/plans/a.md" がある
  When "kotowari plan docs/plans/a.md --format text" を実行する
  Then 終了コードは 0 で、標準出力は空である
```
