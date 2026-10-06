# kotowari check

[English](check.md) | 日本語

<!-- @kotowari[REQ-core-001:4ea3a019] -->

IR（仕様）の書き方の誤りと、IR とテストの対応の抜けを、1件ずつの指摘として挙げるコマンドです。
IR やテストを書いたら実行し、指摘を直して、誤りが0件になるまで繰り返します。
IR の形の検査とテストとの対応の検査は、この1つのコマンドで両方行います。

## 書式

<!-- @kotowari[REQ-core-002:f86e2efd] -->

```sh
kotowari check [--format json|text] [--config <path>] [--allow-test-findings]
```

位置引数は受けません。

## オプションと引数

<!-- @kotowari[REQ-core-002:f86e2efd, REQ-core-021:ccedd28b, REQ-core-003:8ac6759c, REQ-core-004:7719a0bd] -->

| 名前 | 値 | 既定 | 説明 |
|---|---|---|---|
| `--format` | `json` か `text` | `json` | 出力の形 |
| `--config` | 設定ファイルのパス | 基準のディレクトリの `.kotowari/config.yaml` | 読む設定ファイル。カレントディレクトリからの相対で読む |
| `--allow-test-findings` | なし（値を取らない） | 付けない | テスト側の指摘を終了コードに数えない（[テストより先に仕様をコミットする](#テストより先に仕様をコミットする)）。受けるのは `check` だけで、ほかのコマンドに付けると `argument error` で停止する |
| `--help` | なし | — | 使い方を出して終了コード0で終わる |
| `--version` | なし | — | 版を出して終了コード0で終わる |

オプションの共通の決まり（コマンドの前後どちらにも書ける、など）は [cli.md](../cli.ja.md#共通のオプション) にあります。

## 読むもの

<!-- @kotowari[TBL-core-004:3b95b98e, REQ-core-325:ba483356, REQ-core-336:03710aae, REQ-core-337:40375497] -->

`check` は、設定ファイルが指す置き場から、次のものを読みます。

| 読むもの | 置き場を決める設定のキー | 既定 |
|---|---|---|
| IR の文書（用語集 `CONTEXT.md` と問題の記録 `FLAGS.md` を含む） | `ir` | `docs/ir` |
| 判断の記録（出典の先） | `decisions.records` | `docs/decision/records` |
| ADR（出典の先） | `decisions.adr` | `docs/decision/adr` |
| テストのファイル | `tests.files` | `src/**/*.rs`、`tests/**/*.rs` |
| ガイド | `guides.files` | 空（読まない） |
| 面のファイルと面の規則のファイル | `surface.files`、`surface.rules` | 空（読まない） |
| 未記載の面の一覧 | `surface.unspecified` | 無し（0件） |
| 全体像の元データ | `overview.files` | 無し（`overview` の鍵が無ければ読まない） |
| 全体像の目次 | `overview.toc` | 無し（`overview` の鍵が無ければ読まない。鍵があれば必須で、指す先が無いか読めなければ止まる） |

面の3つは `surface.rules` が空の一覧でないときだけ読みます（[面の検査](../surface.ja.md)）。
`languages` に2つ以上の言語を書いたときは、IR、ガイド、全体像の元データ、目次のほかの言語の側（`foo.en.md`）と一致の記録（`foo.i18n.yaml`）を、先頭の言語の側と同じディレクトリで名前で探して読みます（[言語と対](../config.ja.md#言語と対--languages-と-labels)）。
キーの全部は [config.md](../config.ja.md) にあります。

### 設定ファイル

<!-- @kotowari[REQ-core-011:0b7f52a9, REQ-core-012:50c68e4b] -->

- `--config` が無ければ、基準のディレクトリの `.kotowari/config.yaml` を読みます。
- そのファイルが無ければ、すべて既定の値で検査します。
- 設定ファイルが空（0バイトか注釈だけ）なら、`--config` で指したものでも既定の値で検査します。

基準のディレクトリの決め方は [cli.md](../cli.ja.md#基準のディレクトリ) にあります。

### 置き場が無いとき

<!-- @kotowari[REQ-core-018:6ea3e08f, REQ-core-019:10278003] -->

`ir`、`decisions.records`、`decisions.adr` の3つは、既定のままでもディレクトリが存在している必要があります。
どれかが無い、ディレクトリでない、読めないときは、`unreadable file` で停止します。
使わない置き場でも、空のディレクトリを作っておいてください。

`tests.files`、`guides.files`、`surface.files`、`overview.files` は glob の一覧です。
何にも当たらない glob は誤りになりません。
`**` は再帰として読みます。
隠しディレクトリは glob が名指ししても含めず、ディレクトリのシンボリックリンクは辿りません。
全体像の元データの `overview.files` は例外として、パス成分で明示した隠しディレクトリ（`.kotowari/overview/*.md` など）を読みます。名指ししない隠し配下は広い `**` でも除外し、波括弧の中だけで名指しした隠しディレクトリ（`{.overview,other}/*.md`）は名指しに数えません。

走査でディレクトリが読めないときと、先の無いシンボリックリンクに出会ったときは、`unreadable file` で停止します。

### 文字コードと閉じないコードブロック

<!-- @kotowari[REQ-core-111:a41af8e5, REQ-core-112:2fc914d5] -->

読むファイルは UTF-8 である必要があります。
先頭の BOM は読み飛ばすので、BOM 付きの UTF-8 でも止まりません。
UTF-8 でないファイルがあると `non-UTF-8 file` で停止します。

IR の文書のコードブロックが閉じられずに文書が終わると、開始の行に `unclosed_code_block` の誤りが出ます。
そのときは、開始から文書の終わりまでを検査しません。

## 出力

### text

<!-- @kotowari[REQ-core-025:58025379, REQ-core-026:530a64e3] -->

1行が1件の指摘です。
誤りも注意も無ければ、何も出しません。

```text
パス:行 [error] 種類 詳細
docs/ir/greet/greet.md:18 [error] source_invalid docs/decision/records/2026-09-24-greet.md#A3
docs/ir/greet/greet.md:- [notice] too_many_lines 30
```

| 部分 | 中身 |
|---|---|
| パス | 基準のディレクトリからの相対パス |
| 行 | 1始まりの行。文書全体への指摘では `-` |
| `[error]` / `[notice]` | 誤りか注意か |
| 種類 | 指摘の種類。意味と直し方は [findings.md](../findings.ja.md) |
| 詳細 | 種類ごとに決まった短い文字列 |

text には、下の JSON の `files`、`lines`、`tests`、`guides`、`overview` は出ません。

設定の `surface.rules` が空の一覧でないときは、指摘の行の後の最後の1行に、未記載の面の一覧で外した面の数を `surface: unspecified=数` の形で出します。
指摘が0件でも、数が0でも出します（[面の検査](../surface.ja.md#外した数を見る)）。

### JSON

<!-- @kotowari[REQ-core-022:8e221eee, TBL-core-005:73d0910b, PROP-core-002:35724b25, REQ-core-228:095b2109, REQ-core-288:0f3318d5] -->

既定の出力です。
標準出力に JSON のオブジェクトを1つ出します。

| 鍵 | 型 | 説明 |
|---|---|---|
| `files` | 数 | 読んだ IR の文書の数（用語集と問題の記録を含む。対ならほかの言語の側も数える） |
| `lines` | 数 | IR の文書の行数の合計（用語集と問題の記録を含む。対ならほかの言語の側も数える） |
| `findings` | 配列 | 指摘の一覧。1件の鍵は `kind`、`severity`、`path`、`line`、`detail`（[cli.md](../cli.ja.md#json-の指摘)） |
| `counts` | オブジェクト | 種類ごとの指摘の数。0件の種類は鍵ごと出ない |
| `tests` | オブジェクト | 読んだテストのファイルの、拡張子ごとの数（下の節） |
| `guides` | オブジェクト | 読んだガイドの数と、ガイドの印の数（下の節） |
| `surface` | オブジェクト | `unspecified`（未記載の面の一覧で外した面の、種類と名前の組の数）の鍵1つ。`surface.rules` が空の一覧のときは鍵ごと出ない |
| `overview` | オブジェクト | `files`（読んだ全体像の元データの数）と `marks`（その中の形の正しいガイドの印の1件の数）。数え方は `guides` と同じで、設定に `overview` の鍵が無くても両方0で出る |

`counts` の値は、いつも `findings` の中のその種類の件数と一致します。

### JSON の `tests` と `guides`

<!-- @kotowari[REQ-core-128:e6139f2c, TBL-core-021:d85afd29, REQ-core-206:e2f3dc5c] -->

`tests` と `guides` は、設定の glob が思った所に当たっているかを確かめるための数です。
glob を書き間違えても指摘は出ないので、ここで確かめます。

| 鍵 | 型 | 説明 |
|---|---|---|
| `tests.<拡張子>.files` | 数 | その拡張子の、読んだテストのファイルの数。読めずに `unparsable_file` になったファイルも数える |
| `tests.<拡張子>.query` | 真偽 | その拡張子の言語でテストを見つける問い合わせがあるか。false の言語では、テストの関数を見分けずに印だけを拾う |
| `guides.files` | 数 | 読んだガイドの数 |
| `guides.marks` | 数 | 形の正しいガイドの印の1件（`ID:指紋` の1つ）の数 |

- 拡張子の鍵は `.` を含めません（`rs`、`ts`）。拡張子の無いファイルの鍵は空文字列です。
- テストのファイルが0件なら `tests` は `{}`、ガイドが0件なら `guides` は `{"files":0,"marks":0}` です。
- ファイルはパスごとに1回数えます。複数の glob に当たっても1回です。

拡張子ごとの細部は [output.md の TBL-core-021](../../ir/core/output.ja.md) にあります。

### 指摘の並びと行

<!-- @kotowari[TBL-core-007:15954989, REQ-core-027:a594c5e0, TBL-core-019:d5c9adce] -->

指摘は `path`、`line`（null が先）、`kind`、`detail` の順に並びます（[cli.md](../cli.ja.md#指摘の並び)）。
同じ行の指摘は、種類の名前の順に並びます。

`line` は種類ごとに決まっています。
文書全体への指摘（`missing_title`、`missing_scope`、`too_many_lines` など）と、未記載の面の一覧の1件への指摘、全体像の元データの `overview_lead_missing`、`overview_ir_missing`、`overview_ir_shared`、`overview_name_conflict` と、目次の `overview_toc_invalid`、`overview_toc_page_missing`、`overview_toc_page_unknown`、`overview_toc_page_duplicate`、`overview_toc_group_empty` と、対の `translation_missing`、`translation_record_invalid`、`translation_stale` は null、項目への指摘は項目の見出しの行、シナリオへの指摘はタグの行、`surface_without_spec` は面の節の最初の行です。
`overview_form_invalid` は形に合わない行（文書全体にかかるものは null）、`overview_part_unknown`、`overview_part_invalid`、`overview_ref_unresolved` は部品のフェンスの開始の行です。
ただし部品の中身が YAML として読めない `overview_part_invalid` は、YAML の読み取りが誤りの位置を返せばその行（全体像の元データのファイルの行に直したもの）です。
`translation_structure_mismatch` は最初に食い違った要素の行（数の違いか、要素がその側に無いときは null）、`translation_switcher_invalid` は題名の後の最初の空でない行（無ければ題名の行、題名も無ければ null）、`link_language_mismatch` と `link_to_record` はリンクの行です。
種類ごとの行は [findings.md](../findings.ja.md#種類の一覧) の表にあります。

## 終了コード

<!-- @kotowari[TBL-core-002:36817bf5] -->

| コード | 意味 |
|---|---|
| 0 | 誤りが無い（注意だけのときを含む）。`--allow-test-findings` を付けたときは、誤りがどれもテスト側の指摘のときも |
| 1 | 誤りが1件以上ある。`--allow-test-findings` を付けたときは、テスト側の指摘でない誤りが1件以上ある |
| 2 | 停止した（設定が読めない、置き場が無いなど） |

停止したときは標準出力に何も出さず、理由を標準エラーに出します（[cli.md](../cli.ja.md#停止)）。

## テストより先に仕様をコミットする

<!-- @kotowari[REQ-core-357:352c971a, TBL-core-047:0a2c3aa9] -->

仕様はふつう、それを確かめるテストを書くより先に承認してコミットします。
そのコミットの時点では新しい要求とシナリオにまだテストが無いので、`check` は `requirement_without_test` と `scenario_without_test` を出し、終了コード1で終わります。
`--allow-test-findings` を付けると、`check` はこうしたテスト側の指摘を終了コードに数えません。

どの誤りがテスト側の指摘かは、呼び出す側ではなく kotowari が決めます。

| 種類 | テスト側の指摘になる条件 |
|---|---|
| `requirement_without_test` | いつも |
| `scenario_without_test` | いつも |
| `test_without_id` | いつも |
| `invalid_marker` | パスがテストのファイルのとき |
| `unresolved_reference` | パスがテストのファイルのとき |
| `unparsable_file` | パスがテストのファイルで、面のファイルでないとき |

ほかの誤りは、これまでどおり終了コードを1にします。IR の誤り、形の崩れたガイドの印、読めない面のファイルなどです。
オプションが変えるのは終了コードだけです。指摘、その順序、JSON とテキストの出力はオプションが無いときと同じなので、テスト側の指摘も見えたままです。

```console
$ kotowari check --format text --allow-test-findings
docs/ir/greet/greet.md:15 [error] requirement_without_test REQ-greet-002
$ echo $?
0
```

### フックのどこで使うか

<!-- @kotowari[REQ-core-358:879777ff] -->

- pre-commit のフックでは `kotowari check --allow-test-findings` を走らせます。仕様を承認するコミットが、テストより先だという理由で止まらないようにするためです。
- pre-push のフックと CI では、オプションを付けずに `kotowari check` を走らせます。テストの無い要求やシナリオを残したまま共有のブランチに届けないためです。

## 例

挨拶を返すだけの小さなコマンドの IR を書いた場面です。
置いたのは、次の5つのファイルと1つの空のディレクトリです。

```text
.kotowari/config.yaml                     設定（tests.files に "tests/**/*.rs"）
docs/decision/records/2026-09-24-greet.md 判断の記録（決定は A1 と A2 の2件）
docs/decision/adr/                        空のディレクトリ
docs/ir/greet/CONTEXT.md                  用語集（「挨拶文」だけ）
docs/ir/greet/greet.md                    IR の本体
tests/greet.rs                            テスト2本
```

### 書いたばかりの IR を検査する

<!-- @kotowari[REQ-core-085:288046ea, REQ-core-086:8035b3f8, REQ-core-137:cb66f5a8] -->

IR の本体（`docs/ir/greet/greet.md`）とテストは次のとおりです。

````markdown
# 挨拶

挨拶のコマンドが名前を受けて返す文を扱う。

## Requirements

### REQ-greet-001: 名前を入れて挨拶する

- kind: event_driven
- source: docs/decision/records/2026-09-24-greet.md#A1
- verification: unit

名前を受けたとき、コマンドは「こんにちは、名前さん」の`挨拶文`を出す。

### REQ-greet-002: 空の名前は停止する

- kind: event_driven
- source: docs/decision/records/2026-09-24-greet.md#A3
- verification: unit

名前が空のとき、コマンドは`停止`し、必要に応じて理由を出す。

## Examples

```gherkin
@id=EX-greet-001 @about=REQ-greet-001 @source=docs/decision/records/2026-09-24-greet.md#A1
Scenario: 名前を挨拶文に入れる
  When "greet 太郎" を実行する
  Then "こんにちは、太郎さん" が出る
```
````

```rust
// @kotowari[REQ-greet-001]
#[test]
fn greets_with_name() {}

#[test]
fn rejects_empty_name() {}
```

```console
$ kotowari check --format text
docs/ir/greet/greet.md:15 [error] requirement_without_test REQ-greet-002
docs/ir/greet/greet.md:18 [error] source_invalid docs/decision/records/2026-09-24-greet.md#A3
docs/ir/greet/greet.md:21 [error] unknown_term 停止
docs/ir/greet/greet.md:21 [error] vague_word 必要に応じて
docs/ir/greet/greet.md:26 [error] scenario_without_test EX-greet-001
tests/greet.rs:6 [error] test_without_id rejects_empty_name
$ echo $?
1
$ kotowari check | jq '.findings[0]'
{
  "kind": "requirement_without_test",
  "severity": "error",
  "path": "docs/ir/greet/greet.md",
  "line": 15,
  "detail": "REQ-greet-002"
}
```

### テストの印を直す

<!-- @kotowari[REQ-core-085:288046ea, REQ-core-137:cb66f5a8] -->

`rejects_empty_name` に印を足し、`greets_with_name` の印をシナリオの ID に替えます。
シナリオの印は、そのシナリオの `@about` の要求の分も満たします。
今は作らないと決めた要求なら、印の代わりに[後回し](../deferred.ja.md)を宣言してもこの誤りは消えます。

```rust
// @kotowari[EX-greet-001]
#[test]
fn greets_with_name() {}

// @kotowari[REQ-greet-002]
#[test]
fn rejects_empty_name() {}
```

```console
$ kotowari check --format text
docs/ir/greet/greet.md:18 [error] source_invalid docs/decision/records/2026-09-24-greet.md#A3
docs/ir/greet/greet.md:21 [error] unknown_term 停止
docs/ir/greet/greet.md:21 [error] vague_word 必要に応じて
```

### IR を直して0件にする

<!-- @kotowari[REQ-core-058:0012abf7, REQ-core-064:75e8708a, REQ-core-066:79261ab9] -->

出典を実在する `#A2` に直し、用語集に「停止」を足し、「必要に応じて」を具体的な文に書き替えます。

```markdown
名前が空のとき、コマンドは`停止`し、理由を標準エラーに出す。
```

```console
$ kotowari check --format text
$ echo $?
0
$ kotowari check | jq -c .
{"files":2,"lines":36,"findings":[],"counts":{},"tests":{"rs":{"files":1,"query":true}},"guides":{"files":0,"marks":0}}
```

各指摘の直し方は [findings.md](../findings.ja.md) にあります。

### 注意だけが出る

<!-- @kotowari[REQ-core-038:170fdd3e, TBL-core-002:36817bf5] -->

`limits.lines` を20に下げた設定で試すと、行数の注意が出ます。
注意だけなので終了コードは0です。

```console
$ kotowari check --config notice.yaml --format text
docs/ir/greet/greet.md:- [notice] too_many_lines 30
$ echo $?
0
```

## よくあるつまずき

### 設定ファイルを置いていないのに `unreadable file` で止まる

<!-- @kotowari[REQ-core-018:6ea3e08f, REQ-core-012:50c68e4b] -->

```console
$ kotowari check --format text
unreadable file: docs/decision/adr: No such file or directory (os error 2)
```

設定ファイルが無くても、既定の置き場（`docs/ir`、`docs/decision/records`、`docs/decision/adr`）は存在している必要があります。
ADR を使わないなら、空の `docs/decision/adr/` を作るか、設定の `decisions.adr` で別のディレクトリを指してください。

### テストを書いたのに `tests` が `{}` のまま

<!-- @kotowari[REQ-core-128:e6139f2c, TBL-core-021:d85afd29, REQ-core-015:44b9e418] -->

`tests.files` の glob がテストのファイルに当たっていません。
既定は `src/**/*.rs` と `tests/**/*.rs` だけです。
Rust 以外のテストや、別の場所のテストは、設定の `tests.files` に glob を書いてください。
書いた一覧は既定を置き換えるので、既定の2つも要るなら並べて書きます。

### ガイドを書いたのに `guides.files` が0

<!-- @kotowari[REQ-core-206:e2f3dc5c] -->

`guides.files` の既定は空の一覧なので、設定に書くまでガイドは読まれません。
設定に glob を書いてください（[writing-guides.md](../writing-guides.ja.md)）。

### 印を書いたのに `test_without_id` が消えない

<!-- @kotowari[REQ-core-086:8035b3f8] -->

印の位置が違う可能性があります。
印はテストの直前のコメントの塊に書きます。
関数の本体の中や、空行で切り離したコメントの印は、指摘も出さずに無視されます。

```console
$ kotowari check --format text      # 印を関数の本体の中に書いた
docs/ir/greet/greet.md:15 [error] requirement_without_test REQ-greet-002
tests/greet.rs:6 [error] test_without_id rejects_empty_name
```

詳しくは [marks.md](../marks.ja.md) にあります。

## 関連

- 仕様: [出力の形](../../ir/core/output.ja.md)、[指摘の並べ方と行](../../ir/core/finding-order.ja.md)、[設定](../../ir/core/config.ja.md)、[文字コードとコードブロックの境界](../../ir/core/ir-input.ja.md)
- 全コマンド共通の決まり: [cli.md](../cli.ja.md)
- 指摘の種類と直し方: [findings.md](../findings.ja.md)
- 設定のキー: [config.md](../config.ja.md)
- 全体として揃っているかを見る: [status](status.ja.md)
- 1件ずつの項目とテストを見る: [list](list.ja.md)
