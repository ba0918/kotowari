# kotowari check

[English](check.md) | 日本語

<!-- @kotowari[docs/decisions/core/cli.md#kotowari-has-seven-commands-and-the-single-command-check-does-both-the-check-of-the-specification-and-the-check-of-the-correspondence-with-tests:d77a0bd7] -->

決定の文書（仕様）の書き方の誤りと、その確認することの行とテストの対応の抜けを、1件ずつの指摘として挙げるコマンドです。
決定の文書やテストを書いたら実行し、指摘を直して、誤りが0件になるまで繰り返します。
決定の文書の形の検査とテストとの対応の検査は、この1つのコマンドで両方行います。

## 書式

<!-- @kotowari[docs/decisions/core/cli.md#the-options-each-command-accepts-are-fixed-and-options-and-positional-arguments-may-be-written-in-any-order:94339fbd] -->

```sh
kotowari check [--format json|text] [--config <path>] [--allow-test-findings]
```

位置引数は受けません。

## オプションと引数

<!-- @kotowari[docs/decisions/core/cli.md#the-options-each-command-accepts-are-fixed-and-options-and-positional-arguments-may-be-written-in-any-order:94339fbd, docs/decisions/core/output.md#--format-accepts-json-and-text-with-json-as-the-default:811394a9, docs/decisions/core/config.md#the-configuration-file-is-placed-at-kotowariconfigyaml-in-the-base-directory-and---config-can-point-at-another-file:7c217b47, docs/decisions/core/cli.md#an-option-not-accepted-an-unknown-command-an-extra-positional-argument-or-an-option-with-a-wrong-value-stops-with-an-argument-error:11a07c33] -->

| 名前 | 値 | 既定 | 説明 |
|---|---|---|---|
| `--format` | `json` か `text` | `json` | 出力の形 |
| `--config` | 設定ファイルのパス | 基準のディレクトリの `.kotowari/config.yaml` | 読む設定ファイル。カレントディレクトリからの相対で読む |
| `--allow-test-findings` | なし（値を取らない） | 付けない | テスト側の指摘を終了コードに数えない（[テストより先に仕様をコミットする](#テストより先に仕様をコミットする)）。受けるのは `check` だけで、ほかのコマンドに付けると `argument error` で停止する |
| `--help` | なし | — | 使い方を出して終了コード0で終わる |
| `--version` | なし | — | 版を出して終了コード0で終わる |

オプションの共通の決まり（コマンドの前後どちらにも書ける、など）は [cli.md](../cli.ja.md#共通のオプション) にあります。

## 読むもの

<!-- @kotowari[docs/decisions/core/config.md#the-configuration-keys-are-only-those-in-the-table-and-a-key-not-written-takes-the-default-value-in-the-table:122a3403, docs/decisions/core/config.md#the-location-of-decision-documents-is-pointed-at-by-decisionsdocuments-with-docsdecisions-as-the-default-and-the-keys-for-the-locations-of-the-old-form-are-removed:4a2eacb8, docs/decisions/core/overview-toc.md#how-the-index-is-bundled-and-ordered-is-written-in-the-one-table-of-contents-file-that-overviewtoc-points-at:0883f676, docs/decisions/core/translation-pairs.md#guides-overview-source-data-and-the-table-of-contents-have-a-side-for-every-language-of-the-language-list:a581e9d7, docs/decisions/core/translation-pairs.md#the-sides-of-a-pair-are-told-apart-by-file-names-and-configuration-only:3c4056af] -->

`check` は、設定ファイルが指す置き場から、次のものを読みます。

| 読むもの | 置き場を決める設定のキー | 既定 |
|---|---|---|
| 決定の文書（各群の用語集 `CONTEXT.md` は決定の文書としては読まない） | `decisions.documents` | `docs/decisions` |
| テストのファイル | `tests.files` | `src/**/*.rs`、`tests/**/*.rs` |
| ガイド | `guides.files` | 空（読まない） |
| 面のファイルと面の規則のファイル | `surface.files`、`surface.rules` | 空（読まない） |
| 未記載の面の一覧 | `surface.unspecified` | 無し（0件） |
| 全体像の元データ | `overview.files` | 無し（`overview` の鍵が無ければ読まない） |
| 全体像の目次 | `overview.toc` | 無し（`overview` の鍵が無ければ読まない。鍵があれば必須で、指す先が無いか読めなければ止まる） |

面の3つは `surface.rules` が空の一覧でないときだけ読みます（[面の検査](../surface.ja.md)）。
`languages` に2つ以上の言語を書いたときは、決定の文書、ガイド、全体像の元データ、目次のほかの言語の側（`foo.ja.md`）と一致の記録（`foo.i18n.yaml`）を、先頭の言語の側と同じディレクトリで名前で探して読みます（[言語と対](../config.ja.md#言語と対--languages-と-labels)）。
キーの全部は [config.md](../config.ja.md) にあります。

### 設定ファイル

<!-- @kotowari[docs/decisions/core/config.md#the-configuration-file-is-placed-at-kotowariconfigyaml-in-the-base-directory-and---config-can-point-at-another-file:7c217b47, docs/decisions/core/config.md#when-the-configuration-file-is-missing-or-empty-kotowari-runs-with-the-default-values:d78fb03e] -->

- `--config` が無ければ、基準のディレクトリの `.kotowari/config.yaml` を読みます。
- そのファイルが無ければ、すべて既定の値で検査します。
- 設定ファイルが空（0バイトか注釈だけ）なら、`--config` で指したものでも既定の値で検査します。

基準のディレクトリの決め方は [cli.md](../cli.ja.md#基準のディレクトリ) にあります。

### 置き場が無いとき

<!-- @kotowari[docs/decisions/core/config.md#if-the-location-of-decision-documents-does-not-exist-is-not-a-directory-or-cannot-be-read-kotowari-stops-with-unreadable-file:a2064f46, docs/decisions/core/config.md#the-glob--is-read-recursively-and-hidden-directories-are-entered-only-when-named-by-a-path-component:6f67f22b, docs/decisions/core/coverage.md#the-check-of-tests-is-performed-even-when-there-is-no-decision-document-at-all:8dedcbac] -->

決定の文書の置き場（`decisions.documents`）は、既定のままでもディレクトリが存在している必要があります。
無い、ディレクトリでない、読めないときは、`unreadable file` で停止します。
空のディレクトリならかまいません。決定の文書が無くてもテストは検査し、印の無いテストは指摘します。

`tests.files`、`guides.files`、`surface.files`、`overview.files` は glob の一覧です。
何にも当たらない glob は誤りになりません。
`**` は再帰として読みます。
隠しディレクトリは glob が名指ししても含めず、ディレクトリのシンボリックリンクは辿りません。
全体像の元データの `overview.files` は例外として、パス成分で明示した隠しディレクトリ（`.kotowari/overview/*.md` など）を読みます。名指ししない隠し配下は広い `**` でも除外し、波括弧の中だけで名指しした隠しディレクトリ（`{.overview,other}/*.md`）は名指しに数えません。

走査でディレクトリが読めないときと、先の無いシンボリックリンクに出会ったときは、`unreadable file` で停止します。

### 文字コードと閉じないコードブロック

<!-- @kotowari[docs/decisions/core/ir-input.md#a-leading-utf-8-bom-is-skipped-and-is-not-a-stop-as-a-file-that-is-not-utf-8:52a6c07c, docs/decisions/core/ir-input.md#an-unclosed-code-block-is-an-unclosed_code_block-error-and-from-its-start-to-the-end-of-the-document-is-not-read:c1db707e] -->

読むファイルは UTF-8 である必要があります。
先頭の BOM は読み飛ばすので、BOM 付きの UTF-8 でも止まりません。
UTF-8 でないファイルがあると `non-UTF-8 file` で停止します。

決定の文書のコードブロックが閉じられずに文書が終わると、開始の行に `unclosed_code_block` の誤りが出ます。
そのときは、開始から文書の終わりまでを決定としても行としても読みません。

## 出力

### text

<!-- @kotowari[docs/decisions/core/output.md#--format-text-outputs-one-finding-per-line:b093e38d] -->

1行が1件の指摘です。
誤りも注意も無ければ、何も出しません。

```text
パス:行 [error] 種類 詳細
docs/decisions/greet/greet.md:31 [error] vague_word など
guides/greet.md:5 [notice] guide_stale docs/decisions/greet/greet.md#the-command-greets-the-name-it-is-given 00000000 b08a343a
```

| 部分 | 中身 |
|---|---|
| パス | 基準のディレクトリからの相対パス |
| 行 | 1始まりの行。文書全体への指摘では `-` |
| `[error]` / `[notice]` | 誤りか注意か |
| 種類 | 指摘の種類。意味と直し方は [findings.md](../findings.ja.md) |
| 詳細 | 種類ごとに決まった短い文字列 |

text には、下の JSON の `tests`、`guides`、`overview`、`decisions` は出ません。

設定の `surface.rules` が空の一覧でないときは、指摘の行の後の最後の1行に、未記載の面の一覧で外した面の数を `surface: unspecified=数` の形で出します。
指摘が0件でも、数が0でも出します（[面の検査](../surface.ja.md#外した数を見る)）。

### JSON

<!-- @kotowari[docs/decisions/core/output.md#--format-accepts-json-and-text-with-json-as-the-default:811394a9, docs/decisions/core/output.md#the-top-level-of-checks-json-shows-the-findings-the-count-per-kind-and-the-counts-of-what-was-read:bcec81c7, docs/decisions/core/surface.md#the-number-of-surfaces-excluded-by-the-list-of-unspecified-surfaces-is-always-output-at-the-end-of-the-output-of-check:d8dacbf9, docs/decisions/core/overview-output.md#the-json-of-check-carries-an-overview-group-of-the-same-form-as-guides:c1d88d3a] -->

既定の出力です。
標準出力に JSON のオブジェクトを1つ出します。

| 鍵 | 型 | 説明 |
|---|---|---|
| `findings` | 配列 | 指摘の一覧。1件の鍵は `kind`、`severity`、`path`、`line`、`detail`（[cli.md](../cli.ja.md#json-の指摘)） |
| `counts` | オブジェクト | 種類ごとの指摘の数。0件の種類は鍵ごと出ない |
| `tests` | オブジェクト | 読んだテストのファイルの、拡張子ごとの数（下の節） |
| `guides` | オブジェクト | 読んだガイドの数と、ガイドの印の数（下の節） |
| `overview` | オブジェクト | `files`（読んだ全体像の元データの数）と `marks`（その中の形の正しいガイドの印の1件の数）。数え方は `guides` と同じで、設定に `overview` の鍵が無くても両方0で出る |
| `surface` | オブジェクト | `unspecified`（未記載の面の一覧で外した面の、種類と名前の組の数）の鍵1つ。`surface.rules` が空の一覧のときは鍵ごと出ない |
| `decisions` | オブジェクト | `documents`（読んだ決定の文書の数）、`decisions`（決定の数）、`confirmations`（形の正しい ID を持つ確認することの行の数） |

`counts` の値は、いつも `findings` の中のその種類の件数と一致します。

### JSON の `tests` と `guides`

<!-- @kotowari[docs/decisions/core/output.md#the-tests-of-checks-json-counts-the-test-files-read-per-extension:834f2486, docs/decisions/core/output.md#the-top-level-of-checks-json-shows-the-findings-the-count-per-kind-and-the-counts-of-what-was-read:bcec81c7] -->

`tests` と `guides` は、設定の glob が思った場所に当たっているかを確かめるための数です。
glob を書き間違えても指摘は出ないので、ここで確かめます。

| 鍵 | 型 | 説明 |
|---|---|---|
| `tests.<拡張子>.files` | 数 | その拡張子の読んだテストのファイルの数。構文を読めず `unparsable_file` になったファイルも数える |
| `tests.<拡張子>.query` | 真偽値 | その拡張子の言語に、テストを見つける問い合わせがあるか。false の言語では、テストの関数を見分けずに印だけを拾う |
| `guides.files` | 数 | 読んだガイドの数 |
| `guides.marks` | 数 | 形の正しいガイドの印の1件（`行き先:指紋` の1つずつ）の数 |

- 拡張子の鍵に `.` は含めません（`rs`、`ts`）。拡張子の無いファイルの鍵は空の文字列です。
- テストのファイルが0件なら `tests` は `{}`、ガイドが0件なら `guides` は `{"files":0,"marks":0}` です。
- 1つのファイルは、いくつの glob に当たってもパスごとに1回だけ数えます。

拡張子ごとの細部は [出力の決定](../../decisions/core/output.ja.md#check-の-json-の-tests-は読んだテストのファイルを拡張子ごとに数える) にあります。

### 指摘の並びと行

<!-- @kotowari[docs/decisions/core/finding-order.md#findings-are-arranged-by-path-line-kind-and-detail-in-that-order:430be622, docs/decisions/core/finding-order.md#line-is-counted-from-1-and-a-finding-that-covers-the-whole-document-is-null:b98176aa, docs/decisions/core/finding-order.md#the-line-of-findings-that-have-a-line-is-decided-per-kind-in-one-table:848f58fc] -->

指摘は `path`、`line`（null が先）、`kind`、`detail` の順に並びます（[cli.md](../cli.ja.md#指摘の並び)）。
同じ行の指摘は、種類の名前の順に並びます。

`line` は種類ごとに決まっています。
文書全体かファイル全体への指摘（`missing_title`、`multiple_titles`、`unparsable_file` など）、等価の一覧と未記載の面の一覧への指摘、全体像の元データの `overview_lead_missing`、`overview_document_missing`、`overview_document_shared`、`overview_name_conflict`、目次の指摘、対の `translation_missing`、`translation_record_invalid`、`translation_stale` は null です。
決定への指摘は決定の見出しか状態の行、行への指摘はその行、`surface_without_spec` は面のノードの最初の行です。
種類ごとの行は [findings.md](../findings.ja.md#種類の一覧) の表にあります。

## 終了コード

<!-- @kotowari[docs/decisions/core/cli.md#the-exit-code-is-0-when-there-is-no-error-1-when-there-is-an-error-and-2-when-kotowari-stops:3dc67d73] -->

| コード | 意味 |
|---|---|
| 0 | 誤りが無い（注意だけのときを含む）。`--allow-test-findings` を付けたときは、誤りがどれもテスト側の指摘のときも |
| 1 | 誤りが1件以上ある。`--allow-test-findings` を付けたときは、テスト側の指摘でない誤りが1件以上ある |
| 2 | 停止した（設定が読めない、置き場が無いなど） |

停止したときは標準出力に何も出さず、理由を標準エラーに出します（[cli.md](../cli.ja.md#停止)）。

## テストより先に仕様をコミットする

<!-- @kotowari[docs/decisions/core/test-side-findings.md#with---allow-test-findings-check-does-not-count-test-side-findings-toward-the-exit-code:b75bbdf7, docs/decisions/core/test-side-findings.md#test-side-findings-are-limited-to-errors-caused-by-test-files-and-their-range-is-set-in-a-table:09dd0c43, docs/decisions/core/confirmations.md#the-implementing-side-changes-the-status-and-decisions-not-built-yet-stay-proposed:436adde2] -->

仕様はふつう、それを確かめるテストを書くより先に承認してコミットします。
提案で書いた決定はテストを求めないので、そのコミットを止めません。
ただし実装済みの決定に足した行にはまだテストが無いので、`check` は `confirmation_without_test` を出し、終了コード1で終わります。
`--allow-test-findings` を付けると、`check` はこうしたテスト側の指摘を終了コードに数えません。

どの誤りがテスト側の指摘かは、呼び出す側ではなく kotowari が決めます。

| 種類 | テスト側の指摘になる条件 |
|---|---|
| `confirmation_without_test` | いつも |
| `test_without_id` | いつも |
| `invalid_marker` | パスがテストのファイルのとき |
| `unresolved_reference` | パスがテストのファイルのとき |
| `unparsable_file` | パスがテストのファイルで、面のファイルでないとき |

ほかの誤りは、これまでどおり終了コードを1にします。決定の文書の誤り、形の崩れたガイドの印、読めない面のファイルなどです。
オプションが変えるのは終了コードだけです。指摘、その順序、JSON とテキストの出力はオプションが無いときと同じなので、テスト側の指摘も見えたままです。

```console
$ kotowari check --format text --allow-test-findings
docs/decisions/greet/greet.md:35 [error] confirmation_without_test C-greet-greet-2
tests/greet.rs:6 [error] test_without_id rejects_empty_name
$ echo $?
0
```

### フックのどこで使うか

<!-- @kotowari[docs/decisions/core/test-side-findings.md#how-to-build-the-hooks-is-guided-in-the-skill-and-in-the-guide-of-check:3bde432b] -->

- pre-commit のフックでは `kotowari check --allow-test-findings` を走らせます。仕様を承認するコミットが、テストより先だという理由で止まらないようにするためです。
- pre-push のフックと CI では、オプションを付けずに `kotowari check` を走らせます。テストの無い確認することの行を残したまま共有のブランチに届けないためです。

## 例

挨拶を返すだけの小さなコマンドの決定の文書を書いた場面です。
置いたのは、次の3つのファイルです。

```text
.kotowari/config.yaml           設定（tests.files に "tests/**/*.rs"）
docs/decisions/greet/greet.md   決定の文書
tests/greet.rs                  テスト2本
```

### 書いたばかりの決定の文書を検査する

<!-- @kotowari[docs/decisions/core/coverage.md#a-test-without-a-marker-gets-a-test_without_id-error:2f609678, docs/decisions/core/confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it:f6a31de4] -->

決定の文書（`docs/decisions/greet/greet.md`）とテストは次のとおりです。

```markdown
# Greeting

This topic decides the text the greet command prints for a name.

## The command greets the name it is given

Status: implemented

### Problem

Users wanted to see their own name in the greeting.

### Decision

Given a name, the command prints "Hello, <name>!".

### Confirmations

- C-greet-greet-1 "greet Taro" prints "Hello, Taro!"

## An empty name stops the command

Status: implemented

### Problem

An empty name printed "Hello, !", which looked like a bug. The rule for names is in [names](#names-are-trimmed).

### Decision

When the name is empty, the command stops and prints the reason など.

### Confirmations

- C-greet-greet-2 "greet ''" exits with 2
```

```rust
// @kotowari[C-greet-greet-1]
#[test]
fn greets_with_name() {}

#[test]
fn rejects_empty_name() {}
```

```console
$ kotowari check --format text
docs/decisions/greet/greet.md:27 [error] link_target_missing #names-are-trimmed
docs/decisions/greet/greet.md:31 [error] vague_word など
docs/decisions/greet/greet.md:35 [error] confirmation_without_test C-greet-greet-2
tests/greet.rs:6 [error] test_without_id rejects_empty_name
$ echo $?
1
$ kotowari check | jq '.findings[0]'
{
  "kind": "link_target_missing",
  "severity": "error",
  "path": "docs/decisions/greet/greet.md",
  "line": 27,
  "detail": "#names-are-trimmed"
}
```

### テストの印を直す

`rejects_empty_name` に、それが確かめる行の ID の印を付けます。

```rust
// @kotowari[C-greet-greet-1]
#[test]
fn greets_with_name() {}

// @kotowari[C-greet-greet-2]
#[test]
fn rejects_empty_name() {}
```

```console
$ kotowari check --format text
docs/decisions/greet/greet.md:27 [error] link_target_missing #names-are-trimmed
docs/decisions/greet/greet.md:31 [error] vague_word など
```

### 決定の文書を直して0件にする

<!-- @kotowari[docs/decisions/core/decision-documents.md#to-refer-to-another-decision-link-to-its-heading:41ba3d75, docs/decisions/core/decision-documents.md#keep-the-glossary-as-a-document-for-people-and-drop-the-term-check:270bb62b] -->

無い見出しへのリンクを外し、「など」を具体的な文に書き直します。

```markdown
When the name is empty, the command stops and prints the reason to standard error.
```

```console
$ kotowari check --format text
$ echo $?
0
$ kotowari check | jq -c '{findings, counts, tests, guides, decisions}'
{"findings":[],"counts":{},"tests":{"rs":{"files":1,"query":true}},"guides":{"files":0,"marks":0},"decisions":{"documents":1,"decisions":2,"confirmations":2}}
```

指摘ごとの直し方は [findings.md](../findings.ja.md) にあります。

### 注意だけのとき

<!-- @kotowari[docs/decisions/core/guides.md#a-stale-marker-is-a-guide_stale-notice-and-does-not-change-the-exit-code-or-complete:7b8dc3ce, docs/decisions/core/cli.md#the-exit-code-is-0-when-there-is-no-error-1-when-there-is-an-error-and-2-when-kotowari-stops:3dc67d73] -->

印の指紋が古いガイドも読む設定で試すと、`guide_stale` の注意が出ます。
注意だけなので、終了コードは0です。

```console
$ kotowari check --config notice.yaml --format text
guides/greet.md:5 [notice] guide_stale docs/decisions/greet/greet.md#the-command-greets-the-name-it-is-given 00000000 b08a343a
$ echo $?
0
```

## よくあるつまずき

### 設定ファイルが無いのに `unreadable file` で止まる

<!-- @kotowari[docs/decisions/core/config.md#if-the-location-of-decision-documents-does-not-exist-is-not-a-directory-or-cannot-be-read-kotowari-stops-with-unreadable-file:a2064f46, docs/decisions/core/config.md#when-the-configuration-file-is-missing-or-empty-kotowari-runs-with-the-default-values:d78fb03e] -->

```console
$ kotowari check --format text
unreadable file: docs/decisions: No such file or directory (os error 2)
```

設定ファイルが無くても、既定の決定の文書の置き場（`docs/decisions`）は存在している必要があります。
ディレクトリを作るか、設定の `decisions.documents` を決定の文書のあるディレクトリに向けてください。

### テストを書いたのに `tests` が `{}` のまま

<!-- @kotowari[docs/decisions/core/output.md#the-tests-of-checks-json-counts-the-test-files-read-per-extension:834f2486, docs/decisions/core/config.md#a-list-key-accepts-only-a-list-the-list-written-replaces-the-default-and-an-empty-list-is-a-list-with-no-elements:7c2ef13f] -->

`tests.files` の glob がテストのファイルに当たっていません。
既定は `src/**/*.rs` と `tests/**/*.rs` だけです。
Rust 以外のテストや、別の場所のテストは、設定の `tests.files` に glob を書いてください。
書いた一覧は既定を置き換えるので、既定の2つも要るなら並べて書きます。

### ガイドを書いたのに `guides.files` が0

<!-- @kotowari[docs/decisions/core/output.md#the-top-level-of-checks-json-shows-the-findings-the-count-per-kind-and-the-counts-of-what-was-read:bcec81c7] -->

`guides.files` の既定は空の一覧なので、設定に書くまでガイドは読まれません。
設定に glob を書いてください（[writing-guides.md](../writing-guides.ja.md)）。

### 印を書いたのに `test_without_id` が消えない

<!-- @kotowari[docs/decisions/core/coverage.md#a-test-without-a-marker-gets-a-test_without_id-error:2f609678] -->

印の位置が違う可能性があります。
印はテストの直前のコメントの塊に書きます。
関数の本体の中や、空行で切り離したコメントの印は、指摘も出さずに無視されます。

```console
$ kotowari check --format text      # 印を関数の本体の中に書いた
docs/decisions/greet/greet.md:35 [error] confirmation_without_test C-greet-greet-2
tests/greet.rs:6 [error] test_without_id rejects_empty_name
```

詳しくは [marks.md](../marks.ja.md) にあります。

## 関連

- 仕様: [出力の形](../../decisions/core/output.ja.md)、[指摘の並べ方と行](../../decisions/core/finding-order.ja.md)、[設定](../../decisions/core/config.ja.md)、[文字コードとコードブロックの境界](../../decisions/core/ir-input.ja.md)、[テスト側の指摘](../../decisions/core/test-side-findings.ja.md)
- 全コマンド共通の決まり: [cli.md](../cli.ja.md)
- 指摘の種類と直し方: [findings.md](../findings.ja.md)
- 決定の文書の書き方: [decision-documents.md](../decision-documents.ja.md)
- 設定のキー: [config.md](../config.ja.md)
- 全体として揃っているかを見る: [status](status.ja.md)
- 決定と行とテストを1件ずつ見る: [list](list.ja.md)
