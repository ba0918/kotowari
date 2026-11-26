# 設定ファイル — `.kotowari/config.yaml`

[English](config.md) | 日本語

kotowari が読む場所（決定の文書、テスト、ガイド、面のファイル、全体像の元データ）と、検査に使う値を書く YAML のファイルです。
ファイルが無ければ既定の値で動くので、既定から変えたいキーだけを書きます。

## 書式

<!-- @kotowari[docs/decisions/core/config.md#the-configuration-keys-are-only-those-in-the-table-and-a-key-not-written-takes-the-default-value-in-the-table:122a3403, docs/decisions/core/config.md#configuration-keys-are-read-in-nested-form:362c94cc, docs/decisions/core/config.md#the-location-of-decision-documents-is-pointed-at-by-decisionsdocuments-with-docsdecisions-as-the-default-and-the-keys-for-the-locations-of-the-old-form-are-removed:4a2eacb8, docs/decisions/core/config.md#the-keys-for-the-limits-on-line-count-and-number-of-requirements-limitslines-and-limitsrequirements-are-removed:cc3f8709] -->

すべてのキーを既定の値で書くと次のようになります。
キーは入れ子の形で書きます（`decisions:` の下に `documents:`）。

```yaml
decisions:
  documents: docs/decisions
tests:
  files:
    - "src/**/*.rs"
    - "tests/**/*.rs"
  rust:
    attributes: []
    macros: []
  rules: []
guides:
  files: []
mutants:
  equivalents: .kotowari/equivalents.yaml   # 既定は「無し」。書いたときだけ読む
surface:
  files: []
  rules: []
  # unspecified: docs/surface-unspecified.yaml   # 既定は「無し」。surface.rules が空の一覧のときは書けない
# overview:                                     # 既定は「無し」。overview build と serve を使うときは書く
#   files:
#     - ".kotowari/overview/*.md"
#   toc: .kotowari/overview-toc.yaml            # 全体像の目次。overview を書くときは必須
# languages: [ja, en]                           # 既定は「無し」（英語だけ）。2つ以上なら文書を言語ごとの対で持つ
# labels:                                       # 既定は「無し」。英語でない言語の UI の文字（鍵はすべて書く）
#   ja:
#     language_name: 日本語
#     pages: "{n} ページ"
vague_words: [適切に, 必要に応じて, 通常は, など]
```

`mutants.equivalents`、`surface.unspecified`、`overview`、`languages`、`labels` は既定が「鍵が無い」状態です。
上の例の値は書き方を示すためのもので、既定ではありません。
`surface.unspecified` は `surface.rules` が空の一覧のまま書くと止まるので、例では `#` で外しています（[`surface.*`](#surfacefilessurfacerulessurfaceunspecified)）。

## キーの一覧

<!-- @kotowari[docs/decisions/core/config.md#the-configuration-keys-are-only-those-in-the-table-and-a-key-not-written-takes-the-default-value-in-the-table:122a3403, docs/decisions/core/overview-toc.md#how-the-index-is-bundled-and-ordered-is-written-in-the-one-table-of-contents-file-that-overviewtoc-points-at:0883f676, docs/decisions/core/config.md#the-location-of-decision-documents-is-pointed-at-by-decisionsdocuments-with-docsdecisions-as-the-default-and-the-keys-for-the-locations-of-the-old-form-are-removed:4a2eacb8, docs/decisions/core/config.md#the-keys-for-the-limits-on-line-count-and-number-of-requirements-limitslines-and-limitsrequirements-are-removed:cc3f8709] -->

| キー | 値の型 | 既定 | 説明 |
|---|---|---|---|
| `decisions.documents` | パス（文字列） | `docs/decisions` | 決定の文書の置き場のディレクトリ（[決定の文書を書く](decision-documents.ja.md)） |
| `tests.files` | glob の一覧 | `src/**/*.rs`、`tests/**/*.rs` | テストのファイルの置き場 |
| `tests.rust.attributes` | 属性のパスの一覧 | 空 | `#[test]` のほかにテストと数える Rust の属性（例 `kani::proof`） |
| `tests.rust.macros` | マクロの名前の一覧 | 空 | 中の関数をテストと数える Rust のマクロ。`!` を除いて書く（例 `proptest`） |
| `tests.rules` | パスの一覧 | 空 | テストを見つける ast-grep のルールの YAML ファイル。glob は使えない |
| `guides.files` | glob の一覧 | 空 | ガイドの置き場。空ならガイドを読まない |
| `mutants.equivalents` | パス（文字列） | 無し | 等価の一覧のファイル。無ければ等価の一覧は0件 |
| `surface.files` | glob の一覧 | 空 | 面を取り出すコードのファイル（面のファイル）の置き場 |
| `surface.rules` | パスの一覧 | 空 | 面を取り出す ast-grep のルールの YAML ファイル。glob は使えない。空の一覧でなければ面の検査をする |
| `surface.unspecified` | パス（文字列） | 無し | 未記載の面の一覧のファイル。無ければ一覧は0件 |
| `overview.files` | glob の一覧 | `overview` の省略可。記載時は必須 | 全体像の元データの置き場。`overview` が無ければ全体像の元データを読まず、`overview build` と `serve` は止まる |
| `overview.toc` | パス（文字列） | `overview` の省略可。記載時は必須 | 全体像の目次のファイル。一覧のページの入れ子と順番を決める。`overview` があれば `check`、`status`、`overview build`、`overview serve` が読み、指す先が無いか読めなければ止まる |
| `vague_words` | 語の一覧 | `適切に`、`必要に応じて`、`通常は`、`など` | 曖昧語。決定の決めたことか確認することに含まれると `vague_word` の誤り |
| `languages` | 言語タグの一覧 | 無し（英語 `en` だけ） | 言語の一覧。最初の言語が接尾辞の無いファイルの言語。2つ以上なら決定の文書、ガイド、全体像の元データ、目次を言語ごとの対で持つ（[言語と対](#言語と対--languages-と-labels)） |
| `labels` | 言語タグから、UI の文字の鍵から文字列への対応 | 無し（英語の文字は kotowari が持つ） | 言語ごとの UI の文字。英語でない言語はすべての鍵を書く |

パスはすべて基準のディレクトリからの相対パスで書きます（[基準のディレクトリ](#基準のディレクトリ)）。
ほかのキーは書けません。古い形のキー `ir`、`decisions.records`、`decisions.adr`、`limits.lines`、`limits.requirements` も、知らないキーと同じく設定の誤りで止まります。
全部の定義は [設定の決定](../decisions/core/config.ja.md) にあります。

## 言語と対 — `languages` と `labels`

<!-- @kotowari[docs/decisions/core/translation-pairs.md#the-language-list-is-decided-by-the-configurations-languages:5f3acf76, docs/decisions/core/translation-pairs.md#if-a-language-tag-has-the-wrong-form-or-is-duplicated-it-stops:a295d74f, docs/decisions/core/translation-pairs.md#guides-overview-source-data-and-the-table-of-contents-have-a-side-for-every-language-of-the-language-list:a581e9d7, docs/decisions/core/translation-pairs.md#the-sides-of-a-pair-are-told-apart-by-file-names-and-configuration-only:3c4056af, docs/decisions/core/translation-pairs.md#the-agreement-record-holds-the-blob-hash-of-each-side-as-of-the-last-time-they-were-confirmed-to-have-the-same-content:f29e5af9, docs/decisions/core/overview-languages.md#kotowari-holds-the-english-ui-strings-and-other-languages-are-written-in-the-configurations-labels:5ea9a22b, docs/decisions/core/overview-languages.md#if-the-ui-string-configuration-is-missing-something-or-a-number-is-placed-wrongly-it-stops-with-a-configuration-error:0777130f] -->

`languages` は言語タグの並びです（例 `[ja, en]`）。
言語タグは小文字の英字、数字、`-` だけからなり、空の文字列、ほかの文字を含む言語タグ、同じ言語タグの2回目は設定の誤りで止まります。
鍵が無いか空の一覧なら英語 `en` だけとみなします。
最初の言語を「先頭の言語」と呼び、そのファイルは接尾辞の無い名前（`foo.md`）を持ちます。

言語が2つ以上のとき、決定の文書（各群の用語集 `CONTEXT.md` を含む）、ガイド、全体像の元データ、目次を、言語ごとのファイルの組（対）として読みます。

- ほかの言語のファイルは同じディレクトリに、拡張子の前に言語タグを入れた名前で置きます（`foo.en.md`、`toc.en.yaml`）。`guides.files` や `overview.files` の glob に当たるかを問わず、名前で探します
- 先頭の言語の言語タグの付いた `foo.ja.md` や、一覧に無い言語の `foo.fr.md` は、ただの文書 `foo.ja.md` として読みます
- 先頭の言語のファイルの横に一致の記録 `foo.i18n.yaml` を置きます。中身は各言語のファイル名（ディレクトリを除く）を鍵、最後に同じ内容だと確かめたときのそのファイルの git の blob hash（`git hash-object` と同じ40文字の16進の小文字）を値にした対応表です。値は `kotowari list` の `translations` から写します（[list](commands/list.ja.md)）
- 欠けた側、一致の記録の誤り、記録と違う hash、文以外の骨組みの食い違い、切り替えの行の誤り、ほかの言語の側へのリンク、決定の文書から `docs/frozen/` へのリンクは、それぞれ誤りになります（[指摘](findings.ja.md)）

```yaml
# docs/decisions/shop/a.i18n.yaml（値は kotowari list の translations から写す。ここでは例）
a.md: 78981922613b2afb6025042ff6bd878ac1994e85
a.en.md: 0c3b7c0b5b2f3a4d1e8f6a9b2c4d5e6f7a8b9c0d
```

`labels` は言語ごとの UI の文字です。
UI の文字は、全体像のページと切り替えの行に kotowari が書く、元データにも決定の文書にも無い文字です。
英語 `en` の UI の文字は kotowari が持ち、`labels.en` に書いた鍵だけを置き換えます。
英語でない言語は、`labels.<言語タグ>` に次のすべての鍵を書きます。
欠けた鍵、知らない鍵、`languages` に無い言語タグ、数を入れる鍵で `{n}` をちょうど1つ含まない値は設定の誤りで止まります。
全体像を使わないプロジェクトでも、`language_name` は切り替えの行に使うので、英語でない言語の `labels` は要ります。

| 鍵 | 使う所 | 数を入れる | 英語の文字 |
|---|---|---|---|
| `language_name` | 切り替えの行と、全体像のほかの言語へのリンクに書くその言語の名前 | いいえ | `English` |
| `index_link` | 目次に名前の無いページから一覧へのリンク | いいえ | `Overview` |
| `pages` | 一覧の目次の群のページの数 | はい | `{n} pages` |
| `stale_sections` | 一覧の古い節の数 | はい | `{n} sections to review` |
| `open_items` | 一覧の札が `open` の項目の数 | はい | `{n} open` |
| `planned_items` | 一覧の札が `planned` の項目の数 | はい | `{n} planned` |
| `stale_mark` | 古い節の見出しの近くの印 | いいえ | `Not reviewed since the decisions changed` |
| `outline_stale` | アウトラインの古い節の印 | いいえ | `not reviewed` |
| `superseded` | 置き換え済みの参照の印 | いいえ | `(superseded)` |
| `deferred` | 後回しの参照の印 | いいえ | `(deferred)` |
| `compare_before`、`compare_after`、`compare_why` | compare の部品の列の見出し | いいえ | `Before`、`After`、`Why` |
| `state_decided`、`state_planned`、`state_open`、`state_dropped` | status の部品の札 `decided`、`planned`、`open`、`dropped` の表示 | いいえ | `Decided`、`Planned`、`Open`、`Dropped` |

## 設定ファイルの場所

<!-- @kotowari[docs/decisions/core/config.md#the-configuration-file-is-placed-at-kotowariconfigyaml-in-the-base-directory-and---config-can-point-at-another-file:7c217b47] -->

| 指定 | 読むファイル |
|---|---|
| `--config` なし | 基準のディレクトリの `.kotowari/config.yaml` |
| `--config <path>` | `<path>`。カレントディレクトリからの相対パスとして読む |

- `kotowari plan` は設定ファイルを読みません。`--config` を付けると引数の誤りで止まります。
- リポジトリの直下に `kotowari.toml` を置いても読みません。

`--config` の指す先が無いときは、引数の誤りで止まります。

```console
$ kotowari check --config nope.yaml
argument error: config file not found: nope.yaml
$ echo $?
2
```

## 基準のディレクトリ

<!-- @kotowari[docs/decisions/core/base-directory.md#the-base-of-relative-paths-is-the-first-directory-with-kotowari-found-by-searching-upward:e421eeba, docs/decisions/core/base-directory.md#paths-in-configuration-values-and-the-path-of-the-output-are-relative-to-the-base-directory:712ec2ff] -->

設定の値のパスと出力の `path` は、すべて基準のディレクトリからの相対パスです。
基準のディレクトリは次の順で決まります。

| 順 | 条件 | 基準のディレクトリ |
|---|---|---|
| 1 | カレントディレクトリから上に向かって、`.kotowari/` ディレクトリのあるディレクトリが見つかる | 最初に見つかったディレクトリ |
| 2 | 見つからない | カレントディレクトリ |

`.kotowari` という名前のファイルは無視して上に進みます。
`--config` に別のファイルを渡しても、基準のディレクトリは変わりません。

サブディレクトリで実行しても、パスは基準のディレクトリからの相対で出ます。

```console
$ cd tests
$ kotowari list --format text
docs/decisions/shop/discount.md#members-get-10-off-their-orders - Members get 10% off their orders docs/decisions/shop/discount.md:5 tests=0
C-shop-discount-1 - A member's order of 1000 is priced at 900 docs/decisions/shop/discount.md:19 tests=1
  tests/discount.rs:1 member_gets_ten_percent_off
docs/decisions/shop/discount.md#orders-of-non-members-are-not-discounted - Orders of non-members are not discounted docs/decisions/shop/discount.md:21 tests=0
C-shop-discount-2 - A guest's order of 1000 is priced at 1000 docs/decisions/shop/discount.md:35 tests=1
  tests/discount.rs:5 non_member_pays_full_price
```

### パスの書き方

<!-- @kotowari[docs/decisions/core/base-directory.md#paths-in-configuration-values-are-normalized-before-comparing-and-before-output:a1dd7f03, docs/decisions/core/base-directory.md#paths-in-configuration-values-and-the-path-of-the-output-are-relative-to-the-base-directory:712ec2ff] -->

設定の値のパスは、比べる前と出力の前に正規化されます。
末尾の `/` と先頭の `./` を除き、`//` と `/./` を1つの `/` に畳み、`\` を `/` に直し、`a/..` を畳みます。
そのため `./docs/decisions/` と `docs/decisions` は同じ置き場です。
先頭が `/` の絶対パスは書けません（[設定の誤り](#設定の誤り)）。

## ファイルが無いとき、空のとき

<!-- @kotowari[docs/decisions/core/config.md#when-the-configuration-file-is-missing-or-empty-kotowari-runs-with-the-default-values:d78fb03e] -->

| 状態 | 動き |
|---|---|
| `--config` なしで `.kotowari/config.yaml` が無い | 既定の値で検査する |
| ファイルが空（0バイトか注釈だけ） | 既定の値で検査する。`--config` で指したファイルでも同じ |
| `--config` の指す先が無い | 引数の誤りで止まる（終了コード 2） |

## 一覧のキー

<!-- @kotowari[docs/decisions/core/config.md#a-list-key-accepts-only-a-list-the-list-written-replaces-the-default-and-an-empty-list-is-a-list-with-no-elements:7c2ef13f] -->

一覧のキー（`tests.files`、`guides.files`、`vague_words` など）には一覧だけを書きます。
書いた一覧は既定の一覧に足されるのではなく、**置き換え**ます。

```yaml
tests:
  files: ["spec/**/*.rs"]   # 既定の src/**/*.rs と tests/**/*.rs は読まれなくなる
```

空の一覧（`[]`）は「要素が無い」の意味で受けます。
`vague_words: []` なら曖昧語の検査は何も出しません。

## glob の読み方

<!-- @kotowari[docs/decisions/core/config.md#the-glob--is-read-recursively-and-hidden-directories-are-entered-only-when-named-by-a-path-component:6f67f22b, docs/decisions/core/test-discovery.md#test-files-are-selected-with-the-globs-of-testsfiles:f7efafe7] -->

`tests.files`、`guides.files`、`surface.files`、`overview.files` の glob は次のように読みます。

| 対象 | 扱い |
|---|---|
| `**` | 再帰（任意の深さのディレクトリ） |
| 隠しディレクトリ（`.` で始まる） | glob が名指ししても含めない |
| 隠しファイル | glob が当てれば読む |
| ディレクトリのシンボリックリンク | 辿らない |
| ファイルのシンボリックリンク | 読む。先が無ければ止まる |
| ソケット、名前付きパイプ、デバイス | 読まない |

全体像の元データの `overview.files` は例外として、パス成分で明示した隠しディレクトリ（`.kotowari/overview/*.md` の `.kotowari` など）を読みます。名指ししない隠し配下は広い `**` でも除外し、波括弧の中だけで名指しした隠しディレクトリ（`{.overview,other}/*.md`）は名指しに数えません。

走査は基準のディレクトリの全体（隠しディレクトリを除く）を歩いてから glob で選びます。
そのため、glob に当たらない場所にある読めないディレクトリでも止まります。

## キーごとの補足

### `decisions.documents`

<!-- @kotowari[docs/decisions/core/config.md#if-the-location-of-decision-documents-does-not-exist-is-not-a-directory-or-cannot-be-read-kotowari-stops-with-unreadable-file:a2064f46, docs/decisions/core/config.md#the-location-of-decision-documents-is-pointed-at-by-decisionsdocuments-with-docsdecisions-as-the-default-and-the-keys-for-the-locations-of-the-old-form-are-removed:4a2eacb8] -->

`kotowari check` は、指す先が無い、ディレクトリでない、読めないときに、読めないファイルを理由に止まります。

```console
$ cat .kotowari/config.yaml
decisions:
  documents: docs/nothing
$ kotowari check --format text
unreadable file: docs/nothing: No such file or directory (os error 2)
$ echo $?
2
```

置き場があって中に文書が無いだけなら、止まらずに検査し、テストの検査も行います。
古い形の `ir`、`decisions.records`、`decisions.adr` はもうキーではありません。`decisions.documents` だけを書きます。

### `tests.rust.attributes` と `tests.rust.macros`

<!-- @kotowari[docs/decisions/core/test-queries.md#in-rust-functions-whose-attribute-path-ends-in-test-are-always-counted-and-attributes-and-macros-are-added-by-configuration:1e514868] -->

| キー | 書き方 | 一致の仕方 |
|---|---|---|
| `tests.rust.attributes` | `#[` と `]` と引数を除いたパス（`kani::proof`） | パスの完全一致。`#[kani::proof(unwind = 3)]` も当たる |
| `tests.rust.macros` | `!` を除いた名前（`proptest`） | 名前の末尾の要素の一致。`proptest::proptest!` も当たる |

`#[test]`（属性のパスの末尾が `test` のもの）は設定によらず常に数えるので、書く必要はありません。
使い方は [テストに印を付ける](marks.ja.md#マクロの中のテストが数えられない) を見てください。

### `tests.rules`

<!-- @kotowari[docs/decisions/core/query-rules.md#queries-are-added-with-testsrules-in-the-configuration-and-the-bundled-queries-cannot-be-removed:ec930518, docs/decisions/core/query-rules.md#a-rule-file-is-read-as-an-ast-grep-rule-yaml-as-it-is:10835ba2, docs/decisions/core/query-rules.md#the-files-and-ignores-of-a-rule-are-honoured-with-the-same-reading-as-ast-grep:12882344, docs/decisions/core/query-rules.md#the-fix-message-severity-note-and-metadata-of-a-rule-are-not-used:701450c7] -->

同梱の問い合わせで見つからない形のテストを、ast-grep のルールで足します。
並べたパスは基準のディレクトリからの相対パスで、glob は使えません。
1つのファイルに `---` で区切って複数のルールを並べてかまいません。

| ルールの項目 | kotowari での扱い |
|---|---|
| `language` | そのルールを足す言語。大文字小文字を区別せず、`ts`、`py` などの別名も受ける |
| `rule` | 当たった構文木の節を1つのテストと数える。メタ変数 `$NAME` に入った文字をテストの名前にする |
| `files`、`ignores` | ast-grep と同じ読み方で、テストのファイルの相対パスに当てて絞る |
| `fix`、`message`、`severity`、`note`、`metadata` | 使わない。`severity: off` のルールも当てる |

同梱の問い合わせは外せません。ルールは同梱の問い合わせに足されるだけです。

例として、TypeScript の `bench(...)` をテストと数えるルールを足します。

```yaml
# .kotowari/config.yaml
tests:
  files:
    - "tests/**/*.rs"
    - "tests/**/*.ts"
  rules:
    - rules/bench.yml
```

```yaml
# rules/bench.yml
id: bench
language: typescript
rule:
  pattern: bench($NAME, $$$)
```

`tests/discount.bench.ts`:

```ts
// @kotowari[C-shop-discount-1]
bench('member discount is fast', () => {});

bench('non-member is fast', () => {});
```

```console
$ kotowari check --format text
tests/discount.bench.ts:4 [error] test_without_id non-member is fast
```

印の無い2つ目の `bench` がテストとして見つかり、`test_without_id` が出ています。
ルールのファイルはそのまま `ast-grep scan -r rules/bench.yml` で試せます。

### `guides.files`

<!-- @kotowari[docs/decisions/core/guides.md#guides-are-chosen-by-the-globs-of-guidesfiles-and-only-check-and-status-read-them:05ee20e8, docs/decisions/core/guides.md#when-the-guide-and-test-locations-overlap-it-is-a-stop-with-a-configuration-error:4b7987f7] -->

ガイドの置き場です。書き方は [ガイドを書く](writing-guides.ja.md) を見てください。

- 鍵が無いか空の一覧なら、ガイドを1つも読みません。
- ガイドを読むのは `kotowari check` と `kotowari status` だけです。
- 1つのファイルが `guides.files` と `tests.files` の両方に当たると、設定の誤りで止まります（[つまずき](#ガイドとテストの置き場が重なって止まる)）。
- 決定の文書の置き場と重なるのはかまいません。そのファイルはガイドとしても読まれます。

### `surface.files`、`surface.rules`、`surface.unspecified`

<!-- @kotowari[docs/decisions/core/surface.md#surface-rules-and-surface-files-are-read-with-the-same-construction-as-testsrules-and-testsfiles:77fb35de, docs/decisions/core/surface.md#when-the-surface-configuration-keys-are-not-all-present-every-command-that-reads-the-configuration-stops:ab3a2fa6, docs/decisions/core/surface.md#only-check-and-status-read-surfaces:77268ec4] -->

コードから利用者に見える面（CLI のサブコマンドやフラグ、設定の鍵など）を取り出し、決定に引用されているかを確かめる検査の設定です。
使い方は [面の検査](surface.ja.md) を見てください。

- `surface.files` の glob の読み方と走査は `tests.files` と同じです。読むのは面の規則の言語のファイルだけで、それが読めないか UTF-8 でないときの停止はテストのファイルと同じです。ほかのファイルは読まないので、`src/**` のように広く書いて画像などに当たっても止まりません。
- `surface.rules` の書き方と、ファイルの誤りで止まる条件は `tests.rules` と同じです。ただし面の規則はテストを見つける問い合わせには加わりません。
- `surface.files` と `surface.rules` は組で書きます。片方だけのとき、`surface.rules` が空で `surface.unspecified` を書いたときは、設定を読むどのコマンドでも設定の誤りで止まります。
- 面のファイル、面の規則のファイル、未記載の面の一覧を読むのは `kotowari check` と `kotowari status` だけです。

```console
$ cat .kotowari/config.yaml
surface:
  rules: ["rules/surface.yml"]
$ kotowari check --format text
config error: .kotowari/config.yaml: surface.rules is set but surface.files is empty
```

### `mutants.equivalents`

<!-- @kotowari[docs/decisions/core/equivalents.md#judgments-of-equivalence-are-placed-in-the-yaml-list-that-mutantsequivalents-in-the-configuration-points-at:820edb3b] -->

等価の一覧のファイルを指します。読むのは `kotowari mutants` だけです。
`kotowari check` は値の形（文字列か、絶対パスでないか）を確かめるだけで、ファイルの有無は見ません。
一覧の書き方は [kotowari mutants](commands/mutants.ja.md) を見てください。

### `vague_words`

<!-- @kotowari[docs/decisions/core/ir-document.md#the-line-count-and-requirement-count-limit-notices-too_many_lines-and-too_many_requirements-are-removed:776e616c, docs/decisions/core/decision-documents.md#keep-the-glossary-as-a-document-for-people-and-drop-the-term-check:270bb62b] -->

決定の決めたことか確認することの節の行に、`vague_words` の語が部分一致で含まれると、その行に `vague_word` の誤りが出ます。対ではどの側も見ます。
問題と見送った案、コードブロック、二重引用符の中は見ません。
文書の行数や決定の数の上限はありません。

```console
$ cat .kotowari/config.yaml
vague_words: [priced]
$ kotowari check --format text
docs/decisions/shop/discount.md:15 [error] vague_word priced
docs/decisions/shop/discount.md:19 [error] vague_word priced
docs/decisions/shop/discount.md:31 [error] vague_word priced
docs/decisions/shop/discount.md:35 [error] vague_word priced
$ echo $?
1
```

## 設定の誤り

<!-- @kotowari[docs/decisions/core/config.md#if-the-configuration-file-cannot-be-read-as-yaml-or-its-keys-and-values-do-not-follow-the-rules-kotowari-stops-with-a-configuration-error:586c1e39, docs/decisions/core/surface.md#when-the-surface-configuration-keys-are-not-all-present-every-command-that-reads-the-configuration-stops:ab3a2fa6, docs/decisions/core/overview-data.md#if-the-overview-source-data-overlaps-the-location-of-guides-or-tests-it-stops-with-a-configuration-error:297c8b21, docs/decisions/core/config.md#the-configuration-keys-are-only-those-in-the-table-and-a-key-not-written-takes-the-default-value-in-the-table:122a3403, docs/decisions/core/overview-toc.md#if-the-table-of-contents-file-is-also-in-the-location-of-source-data-guides-or-tests-it-stops-with-a-configuration-error:2c8a72ee, docs/decisions/core/translation-pairs.md#if-a-language-tag-has-the-wrong-form-or-is-duplicated-it-stops:a295d74f, docs/decisions/core/overview-languages.md#if-the-ui-string-configuration-is-missing-something-or-a-number-is-placed-wrongly-it-stops-with-a-configuration-error:0777130f] -->

次のどれかがあると、kotowari は検査を行わずに止まります（終了コード 2）。
標準エラーの1行目は `config error: ` で始まり、設定ファイルのパスと理由が続きます。
`tests.rules` と `surface.rules` のファイルの誤り、未記載の面の一覧の誤り、置き場の重なりでは、設定ファイルでなく、そのファイルのパスが出ます（下の例）。

| 場面 | 例 |
|---|---|
| YAML として読めない | `guides: [` |
| 知らないキー | `test:`（正しくは `tests:`） |
| 古い形のキー | `ir:`、`decisions.records`、`decisions.adr`、`limits.lines`、`limits.requirements`（知らないキーとして読む） |
| 同じキーの2回目 | `guides:` を2行 |
| 値が null のキー | `guides:` だけの行 |
| 型の違う値 | `files: "tests/**/*"`（一覧でなく文字列） |
| 絶対パス（先頭が `/`） | `documents: /docs/decisions` |
| `vague_words` の空の文字列、同じ語の2回目 | `vague_words: [""]` |
| `tests.files`、`guides.files`、`surface.files`、`overview.files` の glob として読めない要素 | `files: ["tests/[a"]` |
| `surface.files` と `surface.rules` の片方だけ、または `surface.rules` の無い `surface.unspecified` | [`surface.*`](#surfacefilessurfacerulessurfaceunspecified) |
| ガイドとテストの置き場の重なり | [つまずき](#ガイドとテストの置き場が重なって止まる) |
| 全体像の元データの置き場とガイドかテストの置き場の重なり | `config error: docs/a.md: matched by both overview.files and guides.files`（テストなら `tests.files`） |
| `overview` に `toc` が無い | `config error: .kotowari/config.yaml: overview.toc is required` |
| 目次のファイルが `overview.files`、`guides.files`、`tests.files` のどれかの走査で読むファイルに入る | `config error: .kotowari/overview/toc.md: matched by both overview.toc and overview.files`（当たった鍵のうち、この順で最初のもの。目次を `.yaml` にするなど glob の外に置く） |
| `languages` の空の文字列、小文字の英字と数字と `-` のほかの文字を含む言語タグ、同じ言語タグの2回目 | `config error: .kotowari/config.yaml: invalid language tag: "EN"` |
| `labels` の誤り（`languages` に無い言語タグ、知らない鍵、英語でない言語の欠けた鍵、`{n}` をちょうど1つ含まない数の文字） | `config error: .kotowari/config.yaml: labels.ja.stale_mark: missing` |

空の一覧（`files: []`）は誤りではありません。

実際の出力の例です。

```console
$ cat .kotowari/config.yaml
guides:
$ kotowari check --format text
config error: .kotowari/config.yaml: null value for key: guides
```

```console
$ cat .kotowari/config.yaml
decisions:
  documents: /docs/decisions
$ kotowari check --format text
config error: .kotowari/config.yaml: absolute path not allowed for decisions.documents: /docs/decisions
```

```console
$ cat .kotowari/config.yaml
tests:
  files: ["tests/[a"]
$ kotowari check --format text
config error: .kotowari/config.yaml: invalid glob pattern: tests/[a
```

```console
$ cat .kotowari/config.yaml
guides:
  files: ["guides/*.md"]
guides:
  files: ["guides/*.md"]
$ kotowari check --format text
config error: .kotowari/config.yaml: duplicate key: guides
```

同じキーの2回目は、そのキーの名前だけを1行で示します。
YAML の読み方に関わるほかの誤り（知らないキー、型の違い）は、行と列の付いた説明が続きます（[つまずき](#知らないキーで止まる)）。

### `tests.rules` のファイルの誤り

<!-- @kotowari[docs/decisions/core/query-rules.md#if-a-rule-file-has-an-error-it-stops-as-a-configuration-error:b29a5e1d] -->

次のときも設定の誤りで止まります。

- ファイルが無い、ファイルでない、読めない、UTF-8 でない
- 同じパスが2回並んでいる
- YAML として読めない、ast-grep のルールとして読めない
- `language` が kotowari の知らない言語（言語の一覧は [テストに印を付ける](marks.ja.md#どのテストが見つかるか)）

```console
$ cat .kotowari/config.yaml
tests:
  rules: [rules/missing.yml]
$ kotowari check --format text
config error: unreadable file in tests.rules: rules/missing.yml: No such file or directory (os error 2)
```

```console
$ cat rules/c.yml
id: c
language: cobol
rule:
  pattern: x
$ kotowari check --format text
config error: invalid rule in tests.rules: rules/c.yml: unknown language: cobol
```

ルールの `id` が重なっていても誤りにはしません。

## よくあるつまずき

### 知らないキーで止まる

<!-- @kotowari[docs/decisions/core/config.md#if-the-configuration-file-cannot-be-read-as-yaml-or-its-keys-and-values-do-not-follow-the-rules-kotowari-stops-with-a-configuration-error:586c1e39] -->

```console
$ cat .kotowari/config.yaml
tests:
  file: ["tests/**/*.rs"]
$ kotowari check --format text
config error: .kotowari/config.yaml: error: line 2 column 3: unknown field `file`, expected one of files, rust, rules
 --> <input>:2:3
  |
1 | tests:
2 |   file: ["tests/**/*.rs"]
  |   ^ unknown field `file`, expected one of files, rust, rules
$ echo $?
2
```

キーの綴りの誤りです。`expected one of` の後に、その階層で書けるキーが並びます。
入れ子のキー（`decisions.documents`）を `decisions.documents:` と1行に書いた場合も、知らないキーになります。

### テストを足した glob で、元のテストが読まれなくなった

<!-- @kotowari[docs/decisions/core/config.md#a-list-key-accepts-only-a-list-the-list-written-replaces-the-default-and-an-empty-list-is-a-list-with-no-elements:7c2ef13f] -->

`tests.files` を書くと既定の `src/**/*.rs` と `tests/**/*.rs` は消えます。
元の置き場も読ませたいなら、一覧にすべて並べます。

```console
$ cat .kotowari/config.yaml
tests:
  files: ["spec/**/*.rs"]
$ kotowari check --format text
docs/decisions/shop/discount.md:19 [error] confirmation_without_test C-shop-discount-1
docs/decisions/shop/discount.md:35 [error] confirmation_without_test C-shop-discount-2
$ kotowari status --format text | grep '^tests'
tests marks=0
```

`status` の `tests` の行で、読んだテストのファイルの数を拡張子ごとに確かめられます。
`marks=0` で拡張子の数が無ければ、glob が何にも当たっていません。

### ガイドとテストの置き場が重なって止まる

<!-- @kotowari[docs/decisions/core/guides.md#when-the-guide-and-test-locations-overlap-it-is-a-stop-with-a-configuration-error:4b7987f7] -->

```console
$ cat .kotowari/config.yaml
tests:
  files: ["**/*"]
guides:
  files: ["guides/**/*.md"]
$ kotowari check --format text
config error: guides/a.md: matched by both guides.files and tests.files
$ echo $?
2
```

重なったファイルのうち、パスのバイト順で最初の1つが出ます。
`tests.files` を `tests/**/*` のように絞って、ガイドに当たらないようにします。
直して実行し直すと、ほかに重なりがあれば次のファイルが出ます。

### `kotowari.toml` が読まれない

<!-- @kotowari[docs/decisions/core/config.md#the-configuration-file-is-placed-at-kotowariconfigyaml-in-the-base-directory-and---config-can-point-at-another-file:7c217b47] -->

設定は `.kotowari/config.yaml` だけです。
直下の `kotowari.toml` は、中身に誤りがあっても読まれず、既定の値で検査されます。

## 関連

- 仕様: [設定](../decisions/core/config.ja.md)、[基準のディレクトリ](../decisions/core/base-directory.ja.md)、[設定で足す問い合わせ](../decisions/core/query-rules.ja.md)
- 全コマンド共通の書式、終了コード、停止: [CLI](cli.ja.md)
- テストの見つけ方と印: [テストに印を付ける](marks.ja.md)
- ガイドの置き場と印: [ガイドを書く](writing-guides.ja.md)
- 指摘の種類: [指摘の一覧](findings.ja.md)
