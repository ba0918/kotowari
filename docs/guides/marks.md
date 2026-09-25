# テストに印を付ける — `@kotowari[...]`

印（`@kotowari[REQ-001]`）は、IR の項目とテストを結ぶ唯一の線です。
テストの直前のコメントに ID を書くと、kotowari はそのテストをその項目の裏付けとして数えます。
IR を書き終えたら既存のテストに印を付けていくと、どの要求がテストで守られているかが `check` と `status` に現れます。

## 書式

<!-- @kotowari[REQ-core-071:2eb26299, TBL-core-015:48c1da34, REQ-core-073:9d4ed7ba, REQ-core-074:1085a5de] -->

```rust
// @kotowari[REQ-001, EX-001]
```

| 部分 | 形 |
|---|---|
| 始まり | `@kotowari[` |
| 中身 | ID をコンマで区切って並べる。コンマの前後に空白を置いてよい |
| 終わり | `]`（始まりと同じ行） |

- 1つの印に ID をいくつ並べてもかまいません。要求、決定表、性質、シナリオの ID を混ぜてよいです。
- 1行に印を2つ書いても、両方拾われます（`// @kotowari[REQ-001] @kotowari[EX-001]`）。
- コメント記号は見ません。`//` でも `#` でも `///` でも同じです。

ガイドに書く印（`<!-- @kotowari[ID:指紋] -->`）は別の規則で読みます。[ガイドを書く](writing-guides.md) を見てください。

## まず1本付けてみる

<!-- @kotowari[REQ-core-075:c5389c2a, TBL-core-026:05d8938e] -->

例として、要求2つとシナリオ1つだけの小さな IR を用意しました。

```markdown
### REQ-001: 会員の割引
- verification: unit
システムは常に、会員の注文に1割の割引を付ける。

### REQ-002: 会員でない注文
- verification: unit
システムは常に、会員でない注文に割引を付けない。

@id=EX-001 @about=REQ-001 …
Scenario: 1000円の注文は900円になる
```

（抜粋です。`kind` と `source` の行は省いています。）

テストの関数の真上に、コメントで印を書きます。

```rust
// @kotowari[REQ-001, EX-001]
#[test]
fn member_gets_ten_percent_off() {}

// @kotowari[REQ-002]
#[test]
fn non_member_pays_full_price() {}
```

`check` が何も言わずに終了コード 0 で終われば、結び付いています。
どのテストがどの項目に結び付いたかは `list` で見えます。

```console
$ kotowari check --format text
$ echo $?
0
$ kotowari list --format text
REQ-001 unit 会員の割引 docs/ir/discount.md:7 tests=1
  tests/discount.rs:1 member_gets_ten_percent_off
REQ-002 unit 会員でない注文 docs/ir/discount.md:15 tests=1
  tests/discount.rs:5 non_member_pays_full_price
EX-001 - 1000円の注文は900円になる docs/ir/discount.md:27 tests=1
  tests/discount.rs:1 member_gets_ten_percent_off
```

`tests/discount.rs:1` の `1` は、テストの行ではなく印のある行です。

このリポジトリ自身のテストでは、たとえば次のように書いています（`tests/step11_query.rs` から）。

```rust
// @kotowari[REQ-core-156, REQ-core-159, TBL-core-027, EX-core-250]
#[test]
fn req_156_item_has_body_and_referenced_by() {
```

テストの名前を ID で始めるのはこのリポジトリの慣習で、kotowari は名前を見ません。

## 印を置く場所

<!-- @kotowari[REQ-core-075:c5389c2a, TBL-core-016:fcf775b8, TBL-core-035:06109c14] -->

印が結び付くのは、テストの**直前のコメントの塊**にあるものだけです。
塊は、テストの最初の行のすぐ上から上に向かって、空行が来るまで続くコメントの行です。

| 印の位置 | 扱い |
|---|---|
| テストの直前のコメントの塊 | そのテストに結び付く |
| テストの中（関数の本体の先頭のコメントを含む） | 無視する。誤りも出さない |
| どのテストの直前にも無い | 無視する。誤りも出さない |

塊の途中に挟んでよい行は言語で決まっています。

| 言語 | 挟んでよい行 |
|---|---|
| Rust | 属性（`#[...]`）の行。複数行の属性も全部の行 |
| Python | デコレータ（`@...`）の行。複数行のデコレータも全部の行 |
| そのほか | 無し（コメントの行だけ） |

- 空行、コードの行、コードと同じ行にあるコメント（`setup(); // note`）は、そこで塊を切ります。
- 属性やデコレータの本体（`#[doc = "..."]` の文字列）にある印は読みません。
- 最初の行が同じテストが2つ以上あるときは、その行で最初に始まるテストにだけ結び付きます。

全部の規則は [TBL-core-016](../ir/core/test-markers.md#TBL-core-016) と [TBL-core-035](../ir/core/test-markers.md#TBL-core-035) にあります。

### Python と TypeScript の例

<!-- @kotowari[EX-core-307:6372dbc5, REQ-core-180:169116c9, EX-core-296:ffeb6c9d] -->

Python では、印とテストの関数の間にデコレータがあっても結び付きます。

```python
# @kotowari[REQ-001, EX-001]
@pytest.mark.parametrize("total", [1000])
def test_member_gets_ten_percent_off(total):
    pass
```

TypeScript では `describe` の中の `it` の直前に書きます。

```ts
describe('discount', () => {
  // @kotowari[REQ-002]
  it('non-member pays full price', () => {});
});
```

```console
$ kotowari list --format text
REQ-001 unit 会員の割引 docs/ir/discount.md:7 tests=1
  tests/test_discount.py:3 test_member_gets_ten_percent_off
REQ-002 unit 会員でない注文 docs/ir/discount.md:15 tests=1
  tests/discount.test.ts:2 non-member pays full price
EX-001 - 1000円の注文は900円になる docs/ir/discount.md:27 tests=1
  tests/test_discount.py:3 test_member_gets_ten_percent_off
```

テストの名前は、Python では関数の名前、TypeScript では `it` の最初の引数から引用符を外したものです。

### マクロの中のテスト

<!-- @kotowari[TBL-core-017:5e521eb1, EX-core-018:1cd64885] -->

`proptest!` のようなマクロの中でも、置き方は同じです。
次はこのリポジトリの例で、説明の `///` と印の `//` がどちらもコメントの行として1つの塊になっています（`crates/kotowari-markdown-schema/tests/properties.rs` から）。

```rust
proptest! {
    /// REQ-schema-028: 一覧のマーカーが "-"、"*"、"+" のどれであっても読み分けは変わらない。
    // @kotowari[REQ-schema-028]
    #[test]
    fn markers_do_not_change_how_list_lines_are_read(
```

マクロの中を読ませるには、設定の `tests.rust.macros` にマクロの名前が要ります（[つまずき](#マクロの中のテストが数えられない)）。

## 要求に付けるか、シナリオに付けるか

<!-- @kotowari[REQ-core-085:9c02a2ea, REQ-core-137:192fc62f, EX-core-121:7af1d762, EX-core-122:580060f8] -->

印に書く ID は、要求（`REQ-`）でもシナリオ（`EX-`）でもかまいません。
`check` は2段で見ています。

| 対象 | 誤りになる条件 | 指摘 |
|---|---|---|
| 要求 | その要求の ID を含む印も、その要求を `@about` に持つシナリオの ID を含む印も無い | `requirement_without_test` |
| シナリオ | その ID を含む印が無い | `scenario_without_test` |

つまり、シナリオの印は `@about` の要求の分も満たしますが、逆はありません。

シナリオの ID だけを書いた場合（`// @kotowari[EX-001]`）:

```console
$ kotowari check --format text
$ echo $?
0
```

要求の ID だけを書いた場合（`// @kotowari[REQ-001]`）:

```console
$ kotowari check --format text
docs/ir/discount.md:26 [error] scenario_without_test EX-001
```

シナリオのある要求なら、シナリオの ID を書くのが近道です。
シナリオの無い要求（例の REQ-002）には、要求の ID を直接書きます。

[後回し](deferred.md)の要求は、どちらの誤りにもなりません。
後回しの要求だけ（と review の要求）を `@about` に持つシナリオも同じです。
後回しの ID を印に書くと、`deferred_with_test` の注意が出ます。

### 検証が review の要求

<!-- @kotowari[REQ-core-078:56b1ab82, EX-core-123:8f683517] -->

検証が `review` の要求（人か LLM が目で確かめる要求）にはテストは求められません。
印で指しても誤りにはなりませんが、付けなくてもかまいません。
`review` の要求だけを `@about` に持つシナリオにも、テストは求められません。

### `list` の `tests=0` は「テストが無い」とは限らない

<!-- @kotowari[TBL-core-026:05d8938e, EX-core-122:580060f8] -->

`list` の `tests` は、その ID を**直接**書いた印だけを並べます。
シナリオの ID だけを書いたとき、`list` では要求が `tests=0` に見えますが、`check` と `status` はテストありと数えます。

```console
$ kotowari list --format text
REQ-001 unit 会員の割引 docs/ir/discount.md:7 tests=0
REQ-002 unit 会員でない注文 docs/ir/discount.md:15 tests=1
  tests/discount.rs:5 non_member_pays_full_price
EX-001 - 1000円の注文は900円になる docs/ir/discount.md:27 tests=1
  tests/discount.rs:1 member_gets_ten_percent_off
$ kotowari status --format text | grep requirements
requirements unit=2 property=0 proof=0 review=0 with_tests=2 without_tests=0 review_with_how_to_verify=0 review_without_how_to_verify=0 without_examples=1 deferred=0
```

テストの無い要求を探すときは、`tests=0` の要求のシナリオ側も確かめてください。

## どのテストが見つかるか

<!-- @kotowari[REQ-core-079:0976730b, REQ-core-080:e275e129, REQ-core-081:d861bda1, TBL-core-031:683fed7e] -->

kotowari は次の順でテストを見つけます。

1. 設定の `tests.files` の glob に当たるファイルを読む（[設定ファイル](config.md#glob-の読み方)）。
2. 拡張子から言語を決める。大文字と小文字は区別します（`.py` は Python、`.PY` は決まらない）。
3. その言語に問い合わせ（テストを見つける ast-grep のルール）があれば、構文木を読んでテストを見つける。

拡張子と言語の対応の全部は [TBL-core-031](../ir/core/test-discovery.md#TBL-core-031) にあります。

### 同梱の問い合わせのある言語

<!-- @kotowari[REQ-core-182:acc22ae5, TBL-core-017:5e521eb1, TBL-core-032:fb659f2b, TBL-core-033:f60d396f, TBL-core-034:5276b9bb, REQ-core-181:76c35d7b] -->

| 言語 | テストと数えるもの | 全部の規則 |
|---|---|---|
| Rust（`.rs`） | 属性のパスの末尾が `test` の関数（`#[test]`、`#[tokio::test]`） | [TBL-core-017](../ir/core/test-queries.md#TBL-core-017) |
| TypeScript、Tsx、JavaScript | `it(...)`、`test(...)` と、`it.skip(...)`、`it.each(表)(...)` などの形。`describe` は数えない | [TBL-core-032](../ir/core/test-queries.md#TBL-core-032) |
| Python | 名前が `test` で始まる最上位の関数とクラスのメソッド。関数の中の関数は数えない | [TBL-core-033](../ir/core/test-queries.md#TBL-core-033) |
| Php | 名前が `test` で始まるメソッド、`#[Test]` の付いたメソッド、`@test` の docblock のメソッド、Pest の `test(...)` | [TBL-core-034](../ir/core/test-queries.md#TBL-core-034) |

`it.skip(...)` のように実行しないテストも数えます。
テストの中にあるテスト（`it` の中の `it`）は別のテストとして数えます。

### 足りないときは設定で足す

<!-- @kotowari[REQ-core-121:a1d26f02, REQ-core-186:9ee851c1] -->

| 足したいもの | 設定のキー |
|---|---|
| Rust の別の属性（`#[kani::proof]`） | `tests.rust.attributes` |
| Rust のマクロの中のテスト（`proptest!`） | `tests.rust.macros` |
| それ以外の形や言語 | `tests.rules` に ast-grep のルールのファイルを並べる |

同梱の問い合わせは外せず、設定の分は足されるだけです。
書き方は [設定ファイル](config.md#testsrules) を見てください。

### 問い合わせの無い言語

<!-- @kotowari[REQ-core-076:cbd56987, REQ-core-087:64fd574c, EX-core-124:8278cb29] -->

Go や Java のように問い合わせの無い言語（拡張子の決まらないファイルを含む）では、kotowari は構文木を読みません。
ファイルの中の印を、コメントかどうかも、どのテストの前かも問わずに全部拾い、要求とシナリオの「テストあり」に数えます。
その代わり、印の無いテストを見つけることはできず、`test_without_id` は出ません。

```go
func TestMember(t *testing.T) {
	// @kotowari[REQ-001, EX-001]
}

func TestNonMember(t *testing.T) {}
```

```console
$ kotowari check --format text
docs/ir/discount.md:15 [error] requirement_without_test REQ-002
```

本体の中の印でも REQ-001 と EX-001 は満たされ、印の無い `TestNonMember` には何も言われません。
テストが正しく付いているかは、人が確かめることになります。

## 印に関わる指摘

<!-- @kotowari[REQ-core-086:190ec5a3, REQ-core-072:51247600, REQ-core-077:4f3f71d8, REQ-core-083:9db1b29c, REQ-core-118:6749a6bb] -->

| 指摘 | 重さ | 出る場面 | 行 | detail |
|---|---|---|---|---|
| `test_without_id` | 誤り | 問い合わせのある言語のテストに印が無い | テストの節の最初の行（Rust では `fn` の行） | テストの名前（名前が無ければテストの最初の行） |
| `requirement_without_test` | 誤り | 要求にテストが無い（[上の表](#要求に付けるかシナリオに付けるか)） | 要求の見出し | 要求の ID |
| `scenario_without_test` | 誤り | シナリオにテストが無い | シナリオのタグの行 | シナリオの ID |
| `unresolved_reference` | 誤り | 印の ID が存在しないか、ID の形でない | 印の行 | その ID |
| `invalid_marker` | 誤り | 印の中が空か区切りだけ、または同じ行に `]` が無い | 印の行 | 印のある行の全体 |
| `unparsable_file` | 誤り | 問い合わせのある言語のテストのファイルに構文の誤りがある。そのファイルは飛ばす | 無し（`-`） | ファイルのパス |

存在しない ID だけを指す印でも、テストの側は「印あり」と数えます。
どのテストの直前にも無い印は、`invalid_marker` も `unresolved_reference` も出しません。
指摘の全体は [指摘の一覧](findings.md) にあります。

## よくあるつまずき

以下は、[まず1本付けてみる](#まず1本付けてみる) の例を1か所ずつ壊して実行した結果です。

### 印とテストの間に空行がある

<!-- @kotowari[EX-core-016:46211931, REQ-core-086:190ec5a3] -->

```rust
// @kotowari[REQ-001, EX-001]

#[test]
fn member_gets_ten_percent_off() {}
```

```console
$ kotowari check --format text
docs/ir/discount.md:7 [error] requirement_without_test REQ-001
docs/ir/discount.md:26 [error] scenario_without_test EX-001
tests/discount.rs:4 [error] test_without_id member_gets_ten_percent_off
```

空行で塊が切れ、印はどのテストにも付かないので黙って無視されます。
その結果、テストには「印が無い」、要求とシナリオには「テストが無い」が出ます。
空行を消せば直ります。

### 関数の本体の中に書いた

<!-- @kotowari[EX-core-306:80831f60, TBL-core-016:fcf775b8] -->

```rust
#[test]
fn member_gets_ten_percent_off() {
    // @kotowari[REQ-001, EX-001]
}
```

```console
$ kotowari check --format text
docs/ir/discount.md:7 [error] requirement_without_test REQ-001
docs/ir/discount.md:26 [error] scenario_without_test EX-001
tests/discount.rs:2 [error] test_without_id member_gets_ten_percent_off
```

空行のときと同じ3件です。印を `#[test]` の上に移します。

### 印とテストの間にコードの行がある

<!-- @kotowari[EX-core-309:3b68d952, TBL-core-035:06109c14] -->

```ts
describe('discount', () => {
  // @kotowari[REQ-002]
  const price = 1000;
  it('non-member pays full price', () => {});
});
```

```console
$ kotowari check --format text
docs/ir/discount.md:15 [error] requirement_without_test REQ-002
tests/discount.test.ts:4 [error] test_without_id non-member pays full price
```

挟んでよいのは Rust の属性と Python のデコレータだけです。
TypeScript ではコメントの行以外が挟まると切れます。

### `describe` の上に書いた

<!-- @kotowari[TBL-core-032:fb659f2b, TBL-core-016:fcf775b8] -->

```ts
// @kotowari[REQ-002]
describe('discount', () => {
  it('non-member pays full price', () => {});
});
```

```console
$ kotowari check --format text
docs/ir/discount.md:15 [error] requirement_without_test REQ-002
tests/discount.test.ts:3 [error] test_without_id non-member pays full price
```

`describe` はテストと数えないので、その上の印はどのテストの直前にも無いことになります。
印を `it` の直前に移します。

### ID を書き間違えた

<!-- @kotowari[REQ-core-077:4f3f71d8, REQ-core-118:6749a6bb] -->

```rust
// @kotowari[REQ-010, EX-001]
#[test]
fn member_gets_ten_percent_off() {}

// @kotowari[REQ002]
#[test]
fn non_member_pays_full_price() {}
```

```console
$ kotowari check --format text
docs/ir/discount.md:15 [error] requirement_without_test REQ-002
tests/discount.rs:1 [error] unresolved_reference REQ-010
tests/discount.rs:5 [error] unresolved_reference REQ002
```

存在しない ID（`REQ-010`）も、ID の形でないもの（`REQ002`）も `unresolved_reference` になります。
このときテストの側は「印あり」と数えられるので、`test_without_id` は出ません。
直すべきは印の中の ID です。

### 印が閉じていない、中が空

<!-- @kotowari[REQ-core-072:51247600, TBL-core-015:48c1da34] -->

```rust
// @kotowari[REQ-001, EX-001
#[test]
fn member_gets_ten_percent_off() {}

// @kotowari[]
// @kotowari[REQ-002]
#[test]
fn non_member_pays_full_price() {}
```

```console
$ kotowari check --format text
docs/ir/discount.md:7 [error] requirement_without_test REQ-001
docs/ir/discount.md:26 [error] scenario_without_test EX-001
tests/discount.rs:1 [error] invalid_marker // @kotowari[REQ-001, EX-001
tests/discount.rs:3 [error] test_without_id member_gets_ten_percent_off
tests/discount.rs:5 [error] invalid_marker // @kotowari[]
```

`invalid_marker` の detail には、印のある行がそのまま出ます。
印は1行の中で閉じます。`]` を次の行に送ると、閉じていない印になります。

### マクロの中のテストが数えられない

<!-- @kotowari[REQ-core-082:2cab52bb, EX-core-018:1cd64885] -->

`proptest!` の中に印を付けても、設定が無ければマクロの中は読まれません。

```rust
proptest! {
    // @kotowari[REQ-002]
    #[test]
    fn non_member_pays_full_price(total in 0u32..10_000) {}
}
```

```console
$ kotowari check --format text
docs/ir/discount.md:15 [error] requirement_without_test REQ-002
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

<!-- @kotowari[REQ-core-079:0976730b] -->

テストを `spec/` に置いたまま、`tests.files` が `tests/**/*` だけのとき:

```console
$ kotowari check --format text
docs/ir/discount.md:7 [error] requirement_without_test REQ-001
docs/ir/discount.md:15 [error] requirement_without_test REQ-002
docs/ir/discount.md:26 [error] scenario_without_test EX-001
```

印はあるのに全部「テストが無い」と言われたら、まず glob を疑ってください。

### 拡張子が大文字で、テストが見逃される

<!-- @kotowari[REQ-core-081:d861bda1, EX-core-295:de5afc4b] -->

`test_discount.PY` に印の無い `def test_forgotten():` を足しても、何も言われません。

```console
$ kotowari check --format text
$ kotowari status --format text | grep '^tests'
tests marks=3 PY=1 ts=1
```

`.PY` は Python と見なされず、問い合わせの無い言語として印を拾うだけになるからです。
同じファイルを `test_discount.py` にすると、今度は見つかります。

```console
$ kotowari check --format text
tests/test_discount.py:8 [error] test_without_id test_forgotten
```

誤りが出ないからといって安心できない例です。
`status` の `tests` の行で、拡張子ごとのファイル数を確かめてください。

## なぜこういう作りか

- **印の名前は `@kotowari`。**
  `@spec` のような名前はほかの道具とかぶる可能性があるからです。
  （[records A14](../decision/records/records.md#A14)）
- **関数の本体の先頭の印は結び付けない。**
  本体の先頭がどこかは言語ごとに決め方が違い（PHP の次の行の `{`、Python のコメントが入る位置のクセ）、共通に決めるのが難しいからです。
  切り替えた時点で、kotowari 自身の 1124 個の印のうち本体の先頭にあったのは1個でした。
  結び付かない印のテストには `test_without_id` が出るので、黙って消えることはありません。
  （[multi-language-tests A15](../decision/records/2026-09-24-multi-language-tests.md#A15)）
- **塊を構文木ではなく行で決める。**
  Python のようにコメントが構文木のどこに入るかが言語で違っても、行で決めれば同じ結果になります。
  挟んでよい行が Rust の属性と Python のデコレータだけなのは、Java や PHP ではアノテーションや属性がテストの節の中に入り、外に出るのがこの2つだからです。
  （[multi-language-tests A16](../decision/records/2026-09-24-multi-language-tests.md#A16)、[A17](../decision/records/2026-09-24-multi-language-tests.md#A17)）
- **既定で数えるのは確実なものだけで、残りは設定で足す。**
  何をテストと数えるかは使っているテストのライブラリで変わり、`#[test]` 以外は断言できないからです。
  （[records A26](../decision/records/records.md#A26)、[multi-language-tests A2](../decision/records/2026-09-24-multi-language-tests.md#A2)）
- **テストは正規表現ではなく構文木で見つける。**
  行を正規表現で探すと万能でなく、意図しないものに当たるからです。
  （[records R3](../decision/records/records.md#R3)）
- **ルールの無い言語は、文法を持っていても構文木を読まない。**
  文法を入れただけで振る舞いが変わると、たとえば `.md` を glob に入れている人に構文の誤りが急に出るからです。
  （[multi-language-tests A7](../decision/records/2026-09-24-multi-language-tests.md#A7)）
- **シナリオの印は要求の分も満たす。**
  シナリオは要求の場面なので、場面をテストしたなら要求もテストしたと言えます。
  要求とシナリオの ID を二重に書かずに済みます。
  一方で要求にシナリオが無いこともあるので、要求とシナリオの2段で見ます。
  （[scenario-tests A2](../decision/records/2026-09-17-scenario-tests.md#A2)、[A4](../decision/records/2026-09-17-scenario-tests.md#A4)）

## 関連

- 仕様: [テストの印](../ir/core/test-markers.md)、[テストの見つけ方](../ir/core/test-discovery.md)、[同梱の問い合わせ](../ir/core/test-queries.md)、[要求とテストの対応](../ir/core/coverage.md)、[設定で足す問い合わせ](../ir/core/query-rules.md)
- 用語: [用語集](../ir/core/CONTEXT.md)（印、テスト、直前のコメントの塊、問い合わせのある言語）
- 設定のキー: [設定ファイル](config.md)
- 項目とテストの一覧を見る: [kotowari list](commands/list.md)
- 全体が揃っているかを見る: [kotowari status](commands/status.md)
- 指摘の種類: [指摘の一覧](findings.md)
