# 壁打ちの記録: ガイドを書いて見つかった仕様の穴

## Context

利用者向けのガイド（docs/guides/）を IR から書き、例を実際に実行したところ、仕様と実装の食い違いと仕様の書き漏れが見つかり、TODO.md に記録した（2026-09-24）。
内訳は、実装が仕様と違う振る舞いをするもの（gherkin でないブロック、tests.rules の誤りの詳細、位置引数の後ろのオプション）、仕様が決めていない振る舞い（plan の行と text、query の停止の文言、用語集の題名）、利用者に分かりにくい文言と定義（ライブラリの文言、「等価」）、出典の漏れ。
ガイドが仕様どおりの説明になるよう、どちらに合わせるかをここで決める。

Position: 承認待ち（2026-09-24）。第1ラウンド（A1〜A10）で木は尽き、IR は findings、cli、cli-environment、query-rules、plan、config、finding-order の各文書と CONTEXT.md。実装は plan → cycle。IR を直したので docs/guides の該当の節に guide_stale が出ており、実装の後に見直す

## Agreements

- A1 "## Examples" の下の gherkin でないコードブロックには unknown_code_block だけを出し、その中の行に invalid_gherkin_line を出さない。実装を仕様（[REQ-core-113](../../ir/core/ir-references.md#REQ-core-113)）に合わせる
  - why: invalid_gherkin_line は gherkin のブロックの中の行に限る。同じ原因に2種類の指摘を重ねても直す場所は1つで、数が増えるだけ
  - decided_by: 利用者（推奨を採用）

- A2 "tests.rules" の誤り（[REQ-core-189](../../ir/core/query-rules.md#REQ-core-189)）で停止するときの詳細は、ルールのファイルの基準のディレクトリからの相対パスと、誤りの説明にする。停止の理由の場面の表（[TBL-core-001](../../ir/core/cli.md#TBL-core-001)）の設定の誤りにも REQ-core-189 を入れる。仕様を実装に合わせる
  - why: 直すべきはルールのファイルで、そのパスを出すほうが役に立つ。等価の一覧の誤り（REQ-core-148）と同じ扱い
  - decided_by: 利用者（推奨を採用）

- A3 位置引数とオプションの順は、すべてのコマンドで問わない（[REQ-core-002](../../ir/core/cli.md#REQ-core-002) を広げる）。仕様を実装に合わせる
  - why: 位置引数を取るのは query、mutants、plan で、query だけが順を問われるのは覚えにくい。実装は既に受けている
  - decided_by: 利用者（推奨を採用）

- A4 "kotowari plan" で必須の欄が欠けたときの指摘の行は、そのステップの見出しの行にする。"--format text" では指摘の行だけを出し、指摘が0件なら何も出さない。観測した今の振る舞いを仕様にする
  - why: [REQ-core-193](../../ir/core/plan.md#REQ-core-193) は「スキーマの側が出した行」とだけ書き、どの行かと text の形が決まっていなかった。check の text と同じ形になる
  - decided_by: 利用者（推奨を採用）

- A5 query の停止の文言のうち "unknown id: " 以外（位置引数の数と ID の形の誤り）は、仕様で文言を決めない
  - why: 引数の誤りの詳細は [TBL-core-020](../../ir/core/cli-environment.md#TBL-core-020) が形（説明の文と問題の引数の文字）だけを決め、ほかのコマンドでも文言は契約にしていない。"unknown id: " は LLM が読み分けるために決めた
  - decided_by: 利用者（推奨を採用）

- A6 用語集の題名は "# Glossary" でなければならないことを、IR の要求（[REQ-core-174](../../ir/core/findings.md#REQ-core-174)）に書く
  - why: 今は同梱のスキーマの中にしか無く、IR を読んでも分からない
  - decided_by: 利用者（推奨を採用）

- A7 設定のキーの重複で停止するときの説明は "duplicate key: キー"、"tests.rules" のルールの "language" が知らない言語で停止するときの説明は "unknown language: 言語" の1行にし、ライブラリの文言や複数行の抜粋を出さない
  - why: 今はキーの重複で YAML のライブラリの文言と複数行の抜粋がそのまま出て、知らない言語ではどの言語が問題か出ない。説明の文は契約ではないが、利用者が直す手がかりになる
  - decided_by: 利用者（推奨を採用）

- A8 用語集の「等価」を、変異を入れても利用者から観測できる振る舞い（出力と終了コード）が変わらない、という人か LLM の判断、に書き直す。kotowari 自身の標準出力・標準エラー・終了コードで判断する読み方は、kotowari 自身を変異テストするときの読み方として [A1（mutants-followup）](./2026-09-19-mutants-followup.md#A1) に残る
  - why: 今の定義は kotowari のリポジトリでしか意味が通らず、kotowari mutants を使うほかのプロジェクトに当てはまらない
  - decided_by: 利用者（推奨を採用）

- A9 "## Context" の無い判断の記録に補足の行の検査をしない振る舞い（[REQ-core-129](../../ir/core/record-form.md#REQ-core-129)）は変えない
  - why: 古い形の記録を検査から外すための意図した振る舞い。つまずきやすい点は利用者向けのガイド（docs/guides/findings.md）に書いてある
  - decided_by: 利用者（推奨を採用）

- A10 [TBL-core-008](../../ir/core/findings.md#TBL-core-008) の unknown_line、unknown_code_block、glossary_title_invalid、id_domain_mismatch の detail、[TBL-core-019](../../ir/core/finding-order.md#TBL-core-019) の id_domain_mismatch の行、用語集の「除外」の「二重引用符が奇数のときの最後の引用符から行末まで」は、今の定義のとおりとする
  - why: 出典に支えとなる決定が無かった（2026-09-24 の照合で見つかった）。中身は実装とテストで使われていて、変える理由が無い
  - decided_by: 利用者（推奨を採用）
