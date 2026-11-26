# テストに印を付ける — `@kotowari[...]`

[English](marks.md) | 日本語

印（`@kotowari[C-shop-discount-1]`）は、決定の文書の確認することの行とテストを結ぶ唯一の線です。
テストの直前のコメントに行の ID を書くと、kotowari はそのテストをその行を確かめるテストとして数えます。
決定の文書を書き終えたら既存のテストに印を付けていくと、どの行がテストで確かめられているかが `check` と `status` に現れます。

## 書式

<!-- @kotowari[docs/decisions/core/test-markers.md#a-marker-is-ids-listed-with-commas-and-may-be-anywhere-in-a-line:74293754] -->

```rust
// @kotowari[C-shop-discount-1, C-shop-discount-2]
```

| 部分 | 形 |
|---|---|
| 始まり | `@kotowari[` |
| 中身 | カンマで区切った ID。カンマの前後に空白を置いてよい |
| 終わり | `]`（始まりと同じ行） |

- 1つの印に ID をいくつ並べてもかまいません。どの決定の文書の ID を混ぜてもよいです。
- 1行に印を2つ書いても、両方拾われます（`// @kotowari[C-shop-discount-1] @kotowari[C-shop-discount-2]`）。
- コメント記号は見ません。`//` でも `#` でも `///` でも同じです。

ガイドに書く印（`<!-- @kotowari[行き先:指紋] -->`）は別の規則で読みます。[ガイドを書く](writing-guides.ja.md) を見てください。

## まず1本付けてみる

<!-- @kotowari[docs/decisions/core/test-markers.md#a-marker-is-tied-to-a-test-only-when-it-is-in-the-comment-block-directly-before-that-test:9dac125a, docs/decisions/core/list.md#a-confirmation-row-gets-the-tests-that-point-at-it-and-a-decision-gets-a-fingerprint:64ab7c66, docs/decisions/core/confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it:f6a31de4] -->

例として、実装済みの決定2つに、行を1つずつ持たせた小さな決定の文書を用意しました。

```markdown
## Members get 10% off their orders

Status: implemented

### Decision

An order placed by a member is priced at 10% off.

### Confirmations

- C-shop-discount-1 A member's order of 1000 is priced at 900

## Orders of non-members are not discounted

Status: implemented

### Decision

An order placed by a non-member is priced at the list price.

### Confirmations

- C-shop-discount-2 A guest's order of 1000 is priced at 1000
```

（抜粋です。題名、導入、問題の節は省いています。）

テストの関数の真上に、コメントで印を書きます。

```rust
// @kotowari[C-shop-discount-1]
#[test]
fn member_gets_ten_percent_off() {}

// @kotowari[C-shop-discount-2]
#[test]
fn non_member_pays_full_price() {}
```

`check` が何も出さずに終了コード 0 で終われば、結び付いています。
どのテストがどの行に結び付いたかは `list` で見えます。

```console
$ kotowari check --format text
$ echo $?
0
$ kotowari list --format text
docs/decisions/shop/discount.md#members-get-10-off-their-orders - Members get 10% off their orders docs/decisions/shop/discount.md:5 tests=0
C-shop-discount-1 - A member's order of 1000 is priced at 900 docs/decisions/shop/discount.md:19 tests=1
  tests/discount.rs:1 member_gets_ten_percent_off
docs/decisions/shop/discount.md#orders-of-non-members-are-not-discounted - Orders of non-members are not discounted docs/decisions/shop/discount.md:21 tests=0
C-shop-discount-2 - A guest's order of 1000 is priced at 1000 docs/decisions/shop/discount.md:35 tests=1
  tests/discount.rs:5 non_member_pays_full_price
```

パスで始まる行は決定そのものです。テストが指すのは行なので、決定はいつも `tests=0` です。
`tests/discount.rs:1` の `1` は、テストの行ではなく印の行です。

このリポジトリ自身のテストでは、たとえば次のように書いています（`tests/step23_decision_documents.rs` から）。

```rust
// @kotowari[C-core-query-14]
#[test]
fn query_of_a_decision_puts_the_whole_decision_in_its_body() {
```

kotowari はテストの名前を見ません。

## 印を置く場所

<!-- @kotowari[docs/decisions/core/test-markers.md#a-marker-is-tied-to-a-test-only-when-it-is-in-the-comment-block-directly-before-that-test:9dac125a, docs/decisions/core/test-markers.md#the-preceding-comment-block-consists-of-comment-lines-up-to-a-blank-line-and-the-lines-that-may-be-interposed-per-language:88e06f78] -->

印が結び付くのは、テストの**直前のコメントの塊**にあるものだけです。
塊は、テストの最初の行のすぐ上から上に向かって、空行が来るまで続くコメントの行です。

| 印のある場所 | どうなるか |
|---|---|
| テストの直前のコメントの塊 | そのテストに結び付く |
| テストの中（関数の本体の先頭のコメントを含む） | 誤りにならず、無視される |
| どのテストの直前でもない | 誤りにならず、無視される |

塊の途中に挟んでよい行は言語で決まっています。

| 言語 | 間に挟んでよい行 |
|---|---|
| Rust | 属性の行（`#[...]`）。複数行の属性はその全部の行 |
| Python | デコレータの行（`@...`）。複数行のデコレータはその全部の行 |
| ほか | なし（コメントの行だけ） |

- 空行、コードの行、コードと同じ行にあるコメント（`setup(); // note`）は、そこで塊を切ります。
- 属性やデコレータの本体（`#[doc = "..."]` の文字列）にある印は読みません。
- 最初の行が同じテストが2つ以上あるときは、その行で最初に始まるテストにだけ結び付きます。

全部の規則は [テストの印の仕様](../decisions/core/test-markers.ja.md) にあります。

### Python と TypeScript の例

<!-- @kotowari[docs/decisions/core/test-markers.md#the-preceding-comment-block-consists-of-comment-lines-up-to-a-blank-line-and-the-lines-that-may-be-interposed-per-language:88e06f78, docs/decisions/core/test-discovery.md#a-tests-name-is-taken-from-the-text-the-query-captured:a5dba3c1] -->

Python では、印とテストの関数の間にデコレータがあっても結び付きます。

```python
import pytest

# @kotowari[C-shop-discount-1]
@pytest.mark.parametrize("total", [1000])
def test_member_gets_ten_percent_off(total):
    pass
```

TypeScript では `describe` の中の `it` の直前に書きます。

```ts
describe('discount', () => {
  // @kotowari[C-shop-discount-2]
  it('non-member pays full price', () => {});
});
```

```console
$ kotowari list --format text | grep -A1 '^C-'
C-shop-discount-1 - A member's order of 1000 is priced at 900 docs/decisions/shop/discount.md:19 tests=1
  tests/test_discount.py:3 test_member_gets_ten_percent_off
C-shop-discount-2 - A guest's order of 1000 is priced at 1000 docs/decisions/shop/discount.md:35 tests=1
  tests/discount.test.ts:2 non-member pays full price
```

テストの名前は、Python では関数の名前、TypeScript では `it` の最初の引数から引用符を外したものです。

### マクロの中のテスト

<!-- @kotowari[docs/decisions/core/test-queries.md#in-rust-functions-whose-attribute-path-ends-in-test-are-always-counted-and-attributes-and-macros-are-added-by-configuration:1e514868] -->

`proptest!` のようなマクロの中でも、印の置き方は同じです。
次はこのリポジトリの例で、説明の `///` と印の `//` がどちらもコメントの行として1つの塊になっています（`crates/kotowari-markdown-schema/tests/properties.rs` から）。

```rust
proptest! {
    /// REQ-schema-028: 一覧のマーカーが "-"、"*"、"+" のどれであっても読み分けは変わらない。
    // @kotowari[C-schema-block-2]
    #[test]
    fn markers_do_not_change_how_list_lines_are_read(
```

マクロの中を読ませるには、設定の `tests.rust.macros` にマクロの名前が要ります（[つまずき](#マクロの中のテストが数えられない)）。

## テストの要らない行

<!-- @kotowari[docs/decisions/core/confirmations.md#write-by-reading-at-the-head-of-a-line-not-checked-by-tests:bfbb46c7, docs/decisions/core/test-markers.md#a-marker-that-points-at-a-line-checked-by-reading-is-not-an-error:e433e730, docs/decisions/core/confirmations.md#the-implementing-side-changes-the-status-and-decisions-not-built-yet-stay-proposed:436adde2] -->

`check` がテストを求めるのは、実装済みの決定の行のうち、`（読んで確認）` で始まらない行だけです。

| 行 | 印が無いとき | 印があるとき |
|---|---|---|
| 実装済みの決定の行 | `confirmation_without_test` | テストありと数える |
| `（読んで確認）` で始まる行 | 何も出ない | 何も出ない（誤りではないが、要らない） |
| 提案の決定の行 | 何も出ない | 何も出ない。決定を実装済みに変える前にテストを書いてよい |

```markdown
### Confirmations

- C-shop-discount-3 (by reading) The receipt template has a line for the discount
```

読んで確認する行は、実装する側とレビューが読んで確かめます。`list` では2つ目の欄に `reading` が出て、`status` は `by_reading` に数えます。

## どのテストが見つかるか

<!-- @kotowari[docs/decisions/core/test-discovery.md#test-files-are-selected-with-the-globs-of-testsfiles:f7efafe7, docs/decisions/core/test-discovery.md#tests-are-found-by-applying-queries-to-the-syntax-tree-not-by-searching-lines-with-regular-expressions:ce30c2d1, docs/decisions/core/test-discovery.md#the-correspondence-between-extensions-and-languages-is-the-same-as-ast-grep-and-is-case-sensitive:0894862d] -->

kotowari は次の順でテストを見つけます。

1. 設定の `tests.files` の glob に当たるファイルを読む（[設定ファイル](config.ja.md#glob-の読み方)）。
2. 拡張子から言語を決める。大文字と小文字は区別します（`.py` は Python、`.PY` は決まらない）。
3. その言語に問い合わせ（テストを見つける ast-grep のルール）があれば、構文木を読んでテストを見つける。

拡張子と言語の対応の全部は [テストの見つけ方の仕様](../decisions/core/test-discovery.ja.md) にあります。

### 同梱の問い合わせのある言語

<!-- @kotowari[docs/decisions/core/test-queries.md#queries-are-bundled-only-for-the-four-families-of-languages-the-user-uses:e806bd53, docs/decisions/core/test-queries.md#in-rust-functions-whose-attribute-path-ends-in-test-are-always-counted-and-attributes-and-macros-are-added-by-configuration:1e514868, docs/decisions/core/test-queries.md#in-typescript-and-javascript-calls-of-it-and-test-are-counted:9a76e5a9, docs/decisions/core/test-queries.md#in-python-functions-and-methods-whose-names-start-with-test-are-counted:8ba7df8f, docs/decisions/core/test-queries.md#in-php-methods-and-calls-of-test-are-counted:a1039155, docs/decisions/core/test-discovery.md#overlapping-matches-on-the-same-node-count-as-one-and-a-test-inside-a-test-is-counted-separately:0ccd8b7a] -->

| 言語 | テストと数えるもの | 全部の規則 |
|---|---|---|
| Rust（`.rs`） | 属性のパスの末尾が `test` の関数（`#[test]`、`#[tokio::test]`） | [Rust](../decisions/core/test-queries.ja.md#rust-では属性のパスの末尾が-test-の関数を必ず数え属性とマクロを設定で足す) |
| TypeScript、Tsx、JavaScript | `it(...)`、`test(...)` と、`it.skip(...)`、`it.each(表)(...)` などの形。`describe` は数えない | [TypeScript と JavaScript](../decisions/core/test-queries.ja.md#typescript-と-javascript-ではit-と-test-の呼び出しを数える) |
| Python | 名前が `test` で始まる最上位の関数とクラスのメソッド。関数の中の関数は数えない | [Python](../decisions/core/test-queries.ja.md#python-では名前が-test-で始まる関数とメソッドを数える) |
| Php | 名前が `test` で始まるメソッド、`#[Test]` の付いたメソッド、`@test` の docblock のメソッド、Pest の `test(...)` | [Php](../decisions/core/test-queries.ja.md#php-ではメソッドと-test-の呼び出しを数える) |

`it.skip(...)` のように実行されないテストも数えます。
テストの中にあるテスト（`it` の中の `it`）は別のテストとして数えます。

### 足りないときは設定で足す

<!-- @kotowari[docs/decisions/core/query-rules.md#queries-are-added-with-testsrules-in-the-configuration-and-the-bundled-queries-cannot-be-removed:ec930518, docs/decisions/core/query-rules.md#a-rule-file-is-read-as-an-ast-grep-rule-yaml-as-it-is:10835ba2] -->

| 足したいもの | 設定のキー |
|---|---|
| Rust の別の属性（`#[kani::proof]`） | `tests.rust.attributes` |
| Rust のマクロの中のテスト（`proptest!`） | `tests.rust.macros` |
| それ以外の形や言語 | `tests.rules` に ast-grep のルールのファイルを並べる |

同梱の問い合わせは外せず、設定の分は足されるだけです。
書き方は [設定ファイル](config.ja.md#testsrules) を見てください。

### 問い合わせの無い言語

<!-- @kotowari[docs/decisions/core/test-markers.md#in-files-of-a-language-without-a-query-all-markers-in-the-file-are-picked-up:d8565343, docs/decisions/core/coverage.md#markers-in-files-of-a-language-without-a-query-also-count-toward-satisfying-confirmation-rows:e4917759] -->

Go や Java のように問い合わせの無い言語（拡張子の決まらないファイルを含む）では、kotowari は構文木を読みません。
ファイルの中の印を、コメントかどうかも、どのテストの前かも問わずに全部拾い、行を「テストあり」と数えます。
その代わり、印の無いテストを見つけることはできず、`test_without_id` は出ません。

```go
func TestMember(t *testing.T) {
	// @kotowari[C-shop-discount-1]
}

func TestNonMember(t *testing.T) {}
```

```console
$ kotowari check --format text
docs/decisions/shop/discount.md:35 [error] confirmation_without_test C-shop-discount-2
```

本体の中の印でも C-shop-discount-1 は満たされ、印の無い `TestNonMember` には何も言われません。
テストが正しく付いているかは、人が確かめることになります。

## 印に関わる指摘

<!-- @kotowari[docs/decisions/core/coverage.md#a-test-without-a-marker-gets-a-test_without_id-error:2f609678, docs/decisions/core/test-markers.md#a-malformed-marker-gets-an-invalid_marker-error-pointing-at-the-markers-line:915d8722, docs/decisions/core/test-markers.md#even-a-marker-that-points-only-at-ids-in-no-document-makes-its-test-count-as-one-with-a-marker:5e328dc1, docs/decisions/core/test-discovery.md#a-test-file-with-a-syntax-error-gets-unparsable_file-and-is-skipped:4d52e291, docs/decisions/core/confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it:f6a31de4] -->

| 指摘 | 重さ | 出るとき | 行 | detail |
|---|---|---|---|---|
| `test_without_id` | error | 問い合わせのある言語のテストに印が無い | テストのノードの最初の行（Rust では `fn` の行） | テストの名前（名前が無ければテストの最初の行） |
| `confirmation_without_test` | error | 実装済みの決定の行にテストが無い（[テストの要らない行](#テストの要らない行)） | その行 | 行の ID |
| `unresolved_reference` | error | 印の ID が存在しないか、ID の形でない | 印の行 | その ID |
| `invalid_marker` | error | 印の中が空か区切りだけ、または `]` が同じ行に無い | 印の行 | 印のある行の全体 |
| `unparsable_file` | error | 問い合わせのある言語のテストのファイルに構文の誤りがある。そのファイルは飛ばす | なし（`-`） | ファイルのパス |

存在しない ID だけを指す印でも、テストの側は「印あり」と数えます。
どのテストの直前にも無い印は、`invalid_marker` も `unresolved_reference` も出しません。
指摘の全体は [指摘の一覧](findings.ja.md) にあります。

## よくあるつまずき

以下は、[まず1本付けてみる](#まず1本付けてみる) の例を1か所ずつ壊して実行した結果です。

### 印とテストの間に空行がある

<!-- @kotowari[docs/decisions/core/test-markers.md#the-preceding-comment-block-consists-of-comment-lines-up-to-a-blank-line-and-the-lines-that-may-be-interposed-per-language:88e06f78, docs/decisions/core/coverage.md#a-test-without-a-marker-gets-a-test_without_id-error:2f609678] -->

```rust
// @kotowari[C-shop-discount-1]

#[test]
fn member_gets_ten_percent_off() {}
```

```console
$ kotowari check --format text
docs/decisions/shop/discount.md:19 [error] confirmation_without_test C-shop-discount-1
tests/discount.rs:4 [error] test_without_id member_gets_ten_percent_off
```

空行で塊が切れ、印はどのテストにも付かないので黙って無視されます。
その結果、テストには「印が無い」、行には「テストが無い」が出ます。
空行を消せば直ります。

### 関数の本体の中に書いた

<!-- @kotowari[docs/decisions/core/test-markers.md#a-marker-is-tied-to-a-test-only-when-it-is-in-the-comment-block-directly-before-that-test:9dac125a] -->

```rust
#[test]
fn member_gets_ten_percent_off() {
    // @kotowari[C-shop-discount-1]
}
```

```console
$ kotowari check --format text
docs/decisions/shop/discount.md:19 [error] confirmation_without_test C-shop-discount-1
tests/discount.rs:2 [error] test_without_id member_gets_ten_percent_off
```

空行のときと同じ2件です。印を `#[test]` の上に移します。

### 印とテストの間にコードの行がある

<!-- @kotowari[docs/decisions/core/test-markers.md#the-preceding-comment-block-consists-of-comment-lines-up-to-a-blank-line-and-the-lines-that-may-be-interposed-per-language:88e06f78] -->

```ts
describe('discount', () => {
  // @kotowari[C-shop-discount-2]
  const price = 1000;
  it('non-member pays full price', () => {});
});
```

```console
$ kotowari check --format text
docs/decisions/shop/discount.md:35 [error] confirmation_without_test C-shop-discount-2
tests/discount.test.ts:4 [error] test_without_id non-member pays full price
```

挟んでよいのは Rust の属性と Python のデコレータだけです。
TypeScript ではコメントの行以外が挟まると切れます。

### `describe` の上に書いた

<!-- @kotowari[docs/decisions/core/test-queries.md#in-typescript-and-javascript-calls-of-it-and-test-are-counted:9a76e5a9, docs/decisions/core/test-markers.md#a-marker-is-tied-to-a-test-only-when-it-is-in-the-comment-block-directly-before-that-test:9dac125a] -->

```ts
// @kotowari[C-shop-discount-2]
describe('discount', () => {
  it('non-member pays full price', () => {});
});
```

```console
$ kotowari check --format text
docs/decisions/shop/discount.md:35 [error] confirmation_without_test C-shop-discount-2
tests/discount.test.ts:3 [error] test_without_id non-member pays full price
```

`describe` はテストとして数えないので、その上の印はどのテストの直前でもありません。
印を `it` の直前に移します。

### ID を書き間違えた

<!-- @kotowari[docs/decisions/core/test-markers.md#even-a-marker-that-points-only-at-ids-in-no-document-makes-its-test-count-as-one-with-a-marker:5e328dc1, docs/decisions/core/test-markers.md#a-malformed-marker-gets-an-invalid_marker-error-pointing-at-the-markers-line:915d8722] -->

```rust
// @kotowari[C-shop-discount-10]
#[test]
fn member_gets_ten_percent_off() {}

// @kotowari[C-shop-discount]
#[test]
fn non_member_pays_full_price() {}
```

```console
$ kotowari check --format text
docs/decisions/shop/discount.md:19 [error] confirmation_without_test C-shop-discount-1
docs/decisions/shop/discount.md:35 [error] confirmation_without_test C-shop-discount-2
tests/discount.rs:1 [error] unresolved_reference C-shop-discount-10
tests/discount.rs:5 [error] unresolved_reference C-shop-discount
```

存在しない ID（`C-shop-discount-10`）も、ID の形でないもの（`C-shop-discount`）も `unresolved_reference` になります。
このときテストの側は「印あり」と数えられるので、`test_without_id` は出ません。
直すべきは印の中の ID です。

### 印が閉じていない、中が空

<!-- @kotowari[docs/decisions/core/test-markers.md#a-malformed-marker-gets-an-invalid_marker-error-pointing-at-the-markers-line:915d8722, docs/decisions/core/test-markers.md#a-marker-is-ids-listed-with-commas-and-may-be-anywhere-in-a-line:74293754] -->

```rust
// @kotowari[C-shop-discount-1
#[test]
fn member_gets_ten_percent_off() {}

// @kotowari[]
// @kotowari[C-shop-discount-2]
#[test]
fn non_member_pays_full_price() {}
```

```console
$ kotowari check --format text
docs/decisions/shop/discount.md:19 [error] confirmation_without_test C-shop-discount-1
tests/discount.rs:1 [error] invalid_marker // @kotowari[C-shop-discount-1
tests/discount.rs:3 [error] test_without_id member_gets_ten_percent_off
tests/discount.rs:5 [error] invalid_marker // @kotowari[]
```

`invalid_marker` の detail には、印のある行がそのまま出ます。
印は1行の中で閉じます。`]` を次の行に送ると、閉じていない印になります。

### マクロの中のテストが数えられない

<!-- @kotowari[docs/decisions/core/test-queries.md#in-rust-functions-whose-attribute-path-ends-in-test-are-always-counted-and-attributes-and-macros-are-added-by-configuration:1e514868] -->

`proptest!` の中のテストに印を付けても、設定が無ければマクロの中は読まれません。

```rust
proptest! {
    // @kotowari[C-shop-discount-2]
    #[test]
    fn non_member_pays_full_price(total in 0u32..10_000) {}
}
```

```console
$ kotowari check --format text
docs/decisions/shop/discount.md:35 [error] confirmation_without_test C-shop-discount-2
```

設定にマクロの名前（`!` を除く）を足すと消えます。

```yaml
tests:
  files:
    - "tests/**/*"
  rust:
    macros: [proptest]
```

### テストのファイルが `tests.files` に入っていない

<!-- @kotowari[docs/decisions/core/test-discovery.md#test-files-are-selected-with-the-globs-of-testsfiles:f7efafe7] -->

テストが `spec/` にあるのに、`tests.files` が `tests/**/*` だけのとき:

```console
$ kotowari check --format text
docs/decisions/shop/discount.md:19 [error] confirmation_without_test C-shop-discount-1
docs/decisions/shop/discount.md:35 [error] confirmation_without_test C-shop-discount-2
```

印はあるのに全部「テストが無い」と言われたら、まず glob を疑ってください。

### 拡張子が大文字で、テストが見逃される

<!-- @kotowari[docs/decisions/core/test-discovery.md#the-correspondence-between-extensions-and-languages-is-the-same-as-ast-grep-and-is-case-sensitive:0894862d] -->

`test_discount.PY` に印の無い `def test_forgotten():` を足しても、指摘は出ません。

```console
$ kotowari check --format text
$ kotowari status --format text | grep '^tests'
tests marks=2 PY=1 ts=1
```

`.PY` は Python と見なされず、問い合わせの無い言語として印だけを拾うからです。
同じファイルを `test_discount.py` にすると、今度は見つかります。

```console
$ kotowari check --format text
tests/test_discount.py:8 [error] test_without_id test_forgotten
```

誤りが出ないからといって安心できない例です。
`status` の `tests` の行で、拡張子ごとのファイルの数を確かめてください。

## 関連

- 仕様: [テストの印](../decisions/core/test-markers.ja.md)、[テストの見つけ方](../decisions/core/test-discovery.ja.md)、[同梱の問い合わせ](../decisions/core/test-queries.ja.md)、[確認することとテスト](../decisions/core/confirmations.ja.md)、[テストの無さの検査](../decisions/core/coverage.ja.md)、[設定で足す問い合わせ](../decisions/core/query-rules.ja.md)
- 用語: [用語集](../decisions/core/CONTEXT.ja.md)
- 行の書き方: [決定の文書を書く](decision-documents.ja.md)
- 設定のキー: [設定ファイル](config.ja.md)
- 行とテストの一覧: [kotowari list](commands/list.ja.md)
- 揃っているかを見る: [kotowari status](commands/status.ja.md)
- 指摘の種類: [指摘の一覧](findings.ja.md)
