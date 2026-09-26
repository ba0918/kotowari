# kotowari check

<!-- @kotowari[REQ-core-001:c859c183] -->

IR（仕様）の書き方の誤りと、IR とテストの対応の抜けを、1件ずつの指摘として挙げるコマンドです。
IR やテストを書いたら実行し、指摘を直して、誤りが0件になるまで繰り返します。
IR の形の検査とテストとの対応の検査は、この1つのコマンドで両方行います。

## 書式

<!-- @kotowari[REQ-core-002:06abb59d] -->

```sh
kotowari check [--format json|text] [--config <path>]
```

位置引数は受けません。

## オプションと引数

<!-- @kotowari[REQ-core-002:06abb59d, REQ-core-021:14bd7b25, REQ-core-003:ccf703c7] -->

| 名前 | 値 | 既定 | 説明 |
|---|---|---|---|
| `--format` | `json` か `text` | `json` | 出力の形 |
| `--config` | 設定ファイルのパス | 基準のディレクトリの `.kotowari/config.yaml` | 読む設定ファイル。カレントディレクトリからの相対で読む |
| `--help` | なし | — | 使い方を出して終了コード0で終わる |
| `--version` | なし | — | 版を出して終了コード0で終わる |

オプションの共通の決まり（コマンドの前後どちらにも書ける、など）は [cli.md](../cli.md#共通のオプション) にあります。

## 読むもの

<!-- @kotowari[TBL-core-004:97227aba] -->

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

面の3つは `surface.rules` が空の一覧でないときだけ読みます（[面の検査](../surface.md)）。
キーの全部は [config.md](../config.md) にあります。

### 設定ファイル

<!-- @kotowari[REQ-core-011:549c5c91, REQ-core-012:c9ab4bdd] -->

- `--config` が無ければ、基準のディレクトリの `.kotowari/config.yaml` を読みます。
- そのファイルが無ければ、すべて既定の値で検査します。
- 設定ファイルが空（0バイトか注釈だけ）なら、`--config` で指したものでも既定の値で検査します。

基準のディレクトリの決め方は [cli.md](../cli.md#基準のディレクトリ) にあります。

### 置き場が無いとき

<!-- @kotowari[REQ-core-018:64a197f1, REQ-core-019:04ed8450] -->

`ir`、`decisions.records`、`decisions.adr` の3つは、既定のままでもディレクトリが存在している必要があります。
どれかが無い、ディレクトリでない、読めないときは、`unreadable file` で停止します。
使わない置き場でも、空のディレクトリを作っておいてください。

`tests.files`、`guides.files`、`surface.files` は glob の一覧です。
何にも当たらない glob は誤りになりません。
`**` は再帰として読みます。
隠しディレクトリは glob が名指ししても含めず、ディレクトリのシンボリックリンクは辿りません。
走査でディレクトリが読めないときと、先の無いシンボリックリンクに出会ったときは、`unreadable file` で停止します。

### 文字コードと閉じないコードブロック

<!-- @kotowari[REQ-core-111:2c762a2c, REQ-core-112:204f8368] -->

読むファイルは UTF-8 である必要があります。
先頭の BOM は読み飛ばすので、BOM 付きの UTF-8 でも止まりません。
UTF-8 でないファイルがあると `non-UTF-8 file` で停止します。

IR の文書のコードブロックが閉じられずに文書が終わると、開始の行に `unclosed_code_block` の誤りが出ます。
そのときは、開始から文書の終わりまでを検査しません。

## 出力

### text

<!-- @kotowari[REQ-core-025:b4d331d2, REQ-core-026:70f612a2] -->

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
| 種類 | 指摘の種類。意味と直し方は [findings.md](../findings.md) |
| 詳細 | 種類ごとに決まった短い文字列 |

text には、下の JSON の `files`、`lines`、`tests`、`guides` は出ません。

設定の `surface.rules` が空の一覧でないときは、指摘の行の後の最後の1行に、未記載の面の一覧で外した面の数を `surface: unspecified=数` の形で出します。
指摘が0件でも、数が0でも出します（[面の検査](../surface.md#外した数を見る)）。

### JSON

<!-- @kotowari[REQ-core-022:e6e6163f, TBL-core-005:c0473587, PROP-core-002:2b264d92, REQ-core-228:59fca9bf] -->

既定の出力です。
標準出力に JSON のオブジェクトを1つ出します。

| 鍵 | 型 | 説明 |
|---|---|---|
| `files` | 数 | 読んだ IR の文書の数（用語集と問題の記録を含む） |
| `lines` | 数 | IR の文書の行数の合計（用語集と問題の記録を含む） |
| `findings` | 配列 | 指摘の一覧。1件の鍵は `kind`、`severity`、`path`、`line`、`detail`（[cli.md](../cli.md#json-の指摘)） |
| `counts` | オブジェクト | 種類ごとの指摘の数。0件の種類は鍵ごと出ない |
| `tests` | オブジェクト | 読んだテストのファイルの、拡張子ごとの数（下の節） |
| `guides` | オブジェクト | 読んだガイドの数と、ガイドの印の数（下の節） |
| `surface` | オブジェクト | `unspecified`（未記載の面の一覧で外した面の、種類と名前の組の数）の鍵1つ。`surface.rules` が空の一覧のときは鍵ごと出ない |

`counts` の値は、いつも `findings` の中のその種類の件数と一致します。

### JSON の `tests` と `guides`

<!-- @kotowari[REQ-core-128:fff622e0, TBL-core-021:f3d23680, REQ-core-206:461185b6] -->

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

拡張子ごとの細部は [output.md の TBL-core-021](../../ir/core/output.md) にあります。

### 指摘の並びと行

<!-- @kotowari[TBL-core-007:d8427397, REQ-core-027:6e77baf3, TBL-core-019:126b34ea] -->

指摘は `path`、`line`（null が先）、`kind`、`detail` の順に並びます（[cli.md](../cli.md#指摘の並び)）。
同じ行の指摘は、種類の名前の順に並びます。

`line` は種類ごとに決まっています。
文書全体への指摘（`missing_title`、`missing_scope`、`too_many_lines` など）と、未記載の面の一覧の1件への指摘は null、項目への指摘は項目の見出しの行、シナリオへの指摘はタグの行、`surface_without_spec` は面の節の最初の行です。
種類ごとの行は [findings.md](../findings.md#種類の一覧) の表にあります。

## 終了コード

<!-- @kotowari[TBL-core-002:46c482a8] -->

| コード | 意味 |
|---|---|
| 0 | 誤りが無い（注意だけのときを含む） |
| 1 | 誤りが1件以上ある |
| 2 | 停止した（設定が読めない、置き場が無いなど） |

停止したときは標準出力に何も出さず、理由を標準エラーに出します（[cli.md](../cli.md#停止)）。

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

<!-- @kotowari[REQ-core-085:9c02a2ea, REQ-core-086:190ec5a3, REQ-core-137:192fc62f] -->

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

<!-- @kotowari[REQ-core-085:9c02a2ea, REQ-core-137:192fc62f] -->

`rejects_empty_name` に印を足し、`greets_with_name` の印をシナリオの ID に替えます。
シナリオの印は、そのシナリオの `@about` の要求の分も満たします。
今は作らないと決めた要求なら、印の代わりに[後回し](../deferred.md)を宣言してもこの誤りは消えます。

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

<!-- @kotowari[REQ-core-058:0012abf7, REQ-core-064:8b495194, REQ-core-066:5abc2374] -->

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

各指摘の直し方は [findings.md](../findings.md) にあります。

### 注意だけが出る

<!-- @kotowari[REQ-core-038:4dafbd6a, TBL-core-002:46c482a8] -->

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

<!-- @kotowari[REQ-core-018:64a197f1, REQ-core-012:c9ab4bdd] -->

```console
$ kotowari check --format text
unreadable file: docs/decision/adr: No such file or directory (os error 2)
```

設定ファイルが無くても、既定の置き場（`docs/ir`、`docs/decision/records`、`docs/decision/adr`）は存在している必要があります。
ADR を使わないなら、空の `docs/decision/adr/` を作るか、設定の `decisions.adr` で別のディレクトリを指してください。

### テストを書いたのに `tests` が `{}` のまま

<!-- @kotowari[REQ-core-128:fff622e0, TBL-core-021:f3d23680, REQ-core-015:0e28306b] -->

`tests.files` の glob がテストのファイルに当たっていません。
既定は `src/**/*.rs` と `tests/**/*.rs` だけです。
Rust 以外のテストや、別の場所のテストは、設定の `tests.files` に glob を書いてください。
書いた一覧は既定を置き換えるので、既定の2つも要るなら並べて書きます。

### ガイドを書いたのに `guides.files` が0

<!-- @kotowari[REQ-core-206:461185b6] -->

`guides.files` の既定は空の一覧なので、設定に書くまでガイドは読まれません。
設定に glob を書いてください（[writing-guides.md](../writing-guides.md)）。

### 印を書いたのに `test_without_id` が消えない

<!-- @kotowari[REQ-core-086:190ec5a3] -->

印の位置が違う可能性があります。
印はテストの直前のコメントの塊に書きます。
関数の本体の中や、空行で切り離したコメントの印は、指摘も出さずに無視されます。

```console
$ kotowari check --format text      # 印を関数の本体の中に書いた
docs/ir/greet/greet.md:15 [error] requirement_without_test REQ-greet-002
tests/greet.rs:6 [error] test_without_id rejects_empty_name
```

詳しくは [marks.md](../marks.md) にあります。

## なぜこういう作りか

- **注意は終了コードを変えない。**
  行数や要求の数の超過を「対応必須」と読むと、責務が同じ内容を行数で切ってしまうことが実際に起きました。
  責務の分離は意味の判断なので、CLI は目安を示すだけにしています。
  （[records.md A17](../../decision/records/records.md#A17)、[2026-09-16-notice.md A1](../../decision/records/2026-09-16-notice.md#A1)）
- **`tests` と `guides` の数を JSON に出す。**
  glob の誤りでファイルが0件になっても、指摘としては何も出ません。
  数を出しておけば、黙って何も検査していない状態に気付けます。
  （[2026-09-17-check-reach.md A8](../../decision/records/2026-09-17-check-reach.md#A8)、[2026-09-24-doc-marks.md A17](../../decision/records/2026-09-24-doc-marks.md#A17)）
- **text に `tests` と `guides` を出さない。**
  text の1指摘1行の形を崩さないためです。
  （[2026-09-17-check-reach.md A8](../../decision/records/2026-09-17-check-reach.md#A8)）
- **置き場が無ければ止まる。**
  読めない入力を黙って飛ばさない、という全体の方針に従っています。
  置き場を書き間違えたまま「誤り0件」と答えることがありません。
  （[records.md A41](../../decision/records/records.md#A41)、[A100](../../decision/records/records.md#A100)）

## 関連

- 仕様: [出力の形](../../ir/core/output.md)、[指摘の並べ方と行](../../ir/core/finding-order.md)、[設定](../../ir/core/config.md)、[文字コードとコードブロックの境界](../../ir/core/ir-input.md)
- 全コマンド共通の決まり: [cli.md](../cli.md)
- 指摘の種類と直し方: [findings.md](../findings.md)
- 設定のキー: [config.md](../config.md)
- 全体として揃っているかを見る: [status](status.md)
- 1件ずつの項目とテストを見る: [list](list.md)
