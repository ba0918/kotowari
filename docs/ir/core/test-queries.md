# 同梱の問い合わせ

kotowari が言語ごとに同梱する問い合わせと、それぞれの言語で何をテストと数えるかを扱う。

## Requirements

### REQ-core-182: 同梱の問い合わせのある言語

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-multi-language-tests.md#A8, docs/decision/records/2026-09-24-multi-language-tests.md#A21
- verification: unit

kotowari は常に、Rust、TypeScript、Tsx、JavaScript、Python、Php の`問い合わせ`を同梱する。TypeScript と Tsx は別の言語で、同じ中身の`問い合わせ`を両方に同梱する。

### REQ-core-082: Rust のテスト

- kind: algorithm
- source: docs/decision/records/records.md#A26, docs/decision/records/records.md#A39, docs/decision/records/records.md#A49, docs/decision/records/2026-09-24-multi-language-tests.md#A1, docs/decision/records/2026-09-24-multi-language-tests.md#A12
- definition: TBL-core-017
- verification: unit

### REQ-core-183: TypeScript と JavaScript のテスト

- kind: algorithm
- source: docs/decision/records/2026-09-24-multi-language-tests.md#A18, docs/decision/records/2026-09-24-multi-language-tests.md#A25
- definition: TBL-core-032
- verification: unit

### REQ-core-184: Python のテスト

- kind: algorithm
- source: docs/decision/records/2026-09-24-multi-language-tests.md#A19
- definition: TBL-core-033
- verification: unit

### REQ-core-185: Php のテスト

- kind: algorithm
- source: docs/decision/records/2026-09-24-multi-language-tests.md#A20
- definition: TBL-core-034
- verification: unit

## Decision tables

### TBL-core-017: Rust でテストと数えるもの

- source: docs/decision/records/records.md#A26, docs/decision/records/records.md#A39, docs/decision/records/records.md#A49, docs/decision/records/records.md#A47, docs/decision/records/records.md#A121, docs/decision/records/records.md#A122, docs/decision/records/2026-09-24-multi-language-tests.md#A38, docs/decision/records/2026-09-24-multi-language-tests.md#A53, docs/decision/records/2026-09-24-review10-gaps.md#A4

| 対象 | 数え方 |
|---|---|
| 属性のパスの末尾の要素が "test" の関数（"#[test]"、"#[ test ]"、"#[core::prelude::v1::test]"、"#[tokio::test]"） | 同梱の問い合わせに固定し、常に数える |
| tests.rust.attributes の属性の付いた関数 | 属性から "#["、"]"、引数を除いたパスが完全一致すれば数える |
| 上の2行の関数と tests.rust.macros のマクロが、ほかの関数の本体の中にあるとき | 置き場所によらず数える |
| tests.rust.macros のマクロ | 設定には "!" を除いた名前で書く。マクロの名前の末尾の要素が一致すれば、中身を Rust の項目として読み直し、最上位の関数（ほかの関数の本体の中にない関数。"mod" や "impl" の中の関数を含む）ごとに数える。名前は関数の名前。印の結び付けと invalid_marker は通常の関数と同じ |

### TBL-core-032: TypeScript と JavaScript でテストと数えるもの

- source: docs/decision/records/2026-09-24-multi-language-tests.md#A18, docs/decision/records/2026-09-24-multi-language-tests.md#A25, docs/decision/records/2026-09-24-multi-language-tests.md#A13, docs/decision/records/2026-09-24-multi-language-tests.md#A37, docs/decision/records/2026-09-24-multi-language-tests.md#A21, docs/decision/records/2026-09-24-review11-gaps.md#A1

TypeScript、Tsx、JavaScript に共通する。名前は REQ-core-180 のとおり引用符を外す。

| 対象 | 数え方 |
|---|---|
| 呼ぶ側が "it" か "test" の呼び出し（"it(...)"、"test(...)"） | 数える。名前は最初の引数 |
| 呼ぶ側が "it" か "test" に "." と名前が1つ以上続く形の呼び出し（"it.only(...)"、"it.skip(...)"、"it.todo(...)"、"test.concurrent(...)"、"it.only.each(...)"） | 数える。名前は最初の引数。ただし次の行の内側の呼び出しは除く |
| 上の形のうち最後の要素が "each" の呼び出しの結果をさらに呼ぶ形（"it.each(表)(名前, 関数)"、"test.concurrent.each(表)(名前, 関数)"） | 外側の呼び出しを数える。名前は外側の最初の引数。内側の "it.each(表)" は数えない |
| "describe(...)" と、その "." の形 | 数えない。中の呼び出しは上の行のとおり数え、名前に "describe" の名前を付けない |
| 呼ぶ側が "it"、"test" でない呼び出し（"regex.test(s)"） | 数えない |
| タグ付きテンプレート（"test.each`表`" だけ） | 数えない。結果を呼ぶ形（"test.each`表`(名前, 関数)"）は外側の呼び出しを数える |

### TBL-core-033: Python でテストと数えるもの

- source: docs/decision/records/2026-09-24-multi-language-tests.md#A19, docs/decision/records/2026-09-24-multi-language-tests.md#A17, docs/decision/records/2026-09-24-multi-language-tests.md#A32, docs/decision/records/2026-09-24-multi-language-tests.md#A48, docs/decision/records/2026-09-24-review10-gaps.md#A3

| 対象 | 数え方 |
|---|---|
| 名前が "test" で始まる、ファイルの最上位の関数 | 数える。名前は関数の名前 |
| 名前が "test" で始まる、クラスの中のメソッド（クラスが関数の中にあっても） | 数える。名前はメソッドの名前 |
| 上の関数とメソッドにデコレータが付いたもの | 数える。デコレータはテストの節の外にある |
| 関数の中に入れ子になった関数 | 数えない |

### TBL-core-034: Php でテストと数えるもの

- source: docs/decision/records/2026-09-24-multi-language-tests.md#A20, docs/decision/records/2026-09-24-multi-language-tests.md#A17, docs/decision/records/2026-09-24-multi-language-tests.md#A32, docs/decision/records/2026-09-24-multi-language-tests.md#A48

| 対象 | 数え方 |
|---|---|
| 名前が "test" で始まるメソッド | 数える。名前はメソッドの名前 |
| パスの最後の要素が "Test" の属性の付いたメソッド（"#[Test]"、"#[\PHPUnit\Framework\Attributes\Test]"） | 数える。名前はメソッドの名前。属性はテストの節の中にある |
| 直前の "/**" で始まるコメントに "@test" のあるメソッド | 数える。名前はメソッドの名前 |
| 呼ぶ側が "test" か "it" の関数の呼び出し（Pest の "test('...', fn)"） | 数える。名前は最初の引数 |

## Examples

```gherkin
@id=EX-core-017 @about=REQ-core-082 @source=docs/decision/records/records.md#A39,docs/decision/records/records.md#A49,docs/decision/records/records.md#A47
Scenario: 引数付きの属性も数える
  Given "tests.rust.attributes" が "kani::proof" だけの一覧である
  And "#[kani::proof(unwind = 3)]" の付いた関数がある
  When "kotowari check" を実行する
  Then その関数を`テスト`と数える

@id=EX-core-018 @about=REQ-core-082 @source=docs/decision/records/records.md#A49,docs/decision/records/records.md#A26,docs/decision/records/records.md#A39,docs/decision/records/records.md#A47
Scenario: パスの付いたマクロも数える
  Given "tests.rust.macros" が "proptest" だけの一覧である
  And "proptest::proptest!" の中に関数が2つある
  When "kotowari check" を実行する
  Then `テスト`を2つ数える

@id=EX-core-298 @about=REQ-core-183 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A18,docs/decision/records/2026-09-24-multi-language-tests.md#A13
Scenario: skip したテストも数える
  Given "tests.files" が "tests/**/*.ts" を含み、"tests/a.test.ts" に印の無い "it.skip('x', () => {})" がある
  When "kotowari check" を実行する
  Then detail が "x" の test_without_id の誤りが1件出る

@id=EX-core-299 @about=REQ-core-183 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A18,docs/decision/records/2026-09-24-multi-language-tests.md#A27,docs/decision/records/2026-09-24-multi-language-tests.md#A13,docs/decision/records/2026-09-24-multi-language-tests.md#A37
Scenario: each の表のテストは1つと数える
  Given "tests.files" が "tests/**/*.ts" を含み、"tests/a.test.ts" に印の無い "it.each([1])('each %i', (n) => {})" がある
  When "kotowari check" を実行する
  Then detail が "each %i" の test_without_id の誤りが1件だけ出る

@id=EX-core-300 @about=REQ-core-183 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A18
Scenario: describe と it でない呼び出しは数えない
  Given "tests.files" が "tests/**/*.ts" を含み、"tests/a.test.ts" に "describe('d', () => { regex.test(s); })" だけがある
  When "kotowari check" を実行する
  Then test_without_id の誤りは出ない

@id=EX-core-301 @about=REQ-core-182,REQ-core-183 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A21,docs/decision/records/2026-09-24-multi-language-tests.md#A13,docs/decision/records/2026-09-24-multi-language-tests.md#A18
Scenario: tsx のファイルのテストも数える
  Given "tests.files" が "tests/**/*.tsx" を含み、"tests/a.test.tsx" に印の無い "it('x', () => {})" がある
  When "kotowari check" を実行する
  Then detail が "x" の test_without_id の誤りが1件出る

@id=EX-core-302 @about=REQ-core-184 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A19,docs/decision/records/2026-09-24-multi-language-tests.md#A48
Scenario: 入れ子の関数は数えない
  Given "tests.files" が "tests/**/*.py" を含み、"tests/test_a.py" に印の無い "def test_foo():" があってその中に "def test_inner():" があり、"class TestBar:" の中に印の無い "def test_baz(self):" がある
  When "kotowari check" を実行する
  Then detail が "test_foo" と "test_baz" の test_without_id の誤りが1件ずつ出て、"test_inner" の誤りは出ない

@id=EX-core-303 @about=REQ-core-185 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A20,docs/decision/records/2026-09-24-multi-language-tests.md#A48
Scenario: パスの付いた Test の属性のメソッドを数える
  Given "tests.files" が "tests/**/*.php" を含み、"tests/FooTest.php" のクラスに印の無い "#[\PHPUnit\Framework\Attributes\Test]" の付いたメソッド "other" がある
  When "kotowari check" を実行する
  Then detail が "other" の test_without_id の誤りが1件出る

@id=EX-core-304 @about=REQ-core-185 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A20,docs/decision/records/2026-09-24-multi-language-tests.md#A48
Scenario: docblock に @test のあるメソッドを数える
  Given "tests.files" が "tests/**/*.php" を含み、"tests/FooTest.php" のクラスに、"/** @test */" の直後のメソッド "itWorks" がある
  When "kotowari check" を実行する
  Then detail が "itWorks" の test_without_id の誤りが1件出る

@id=EX-core-305 @about=REQ-core-185 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A20,docs/decision/records/2026-09-24-multi-language-tests.md#A13,docs/decision/records/2026-09-24-multi-language-tests.md#A48
Scenario: Pest の test の呼び出しを数える
  Given "tests.files" が "tests/**/*.php" を含み、"tests/FooTest.php" に印の無い "test('adds', function () {});" がある
  When "kotowari check" を実行する
  Then detail が "adds" の test_without_id の誤りが1件出る
@id=EX-core-317 @about=REQ-core-183 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A37,docs/decision/records/2026-09-24-multi-language-tests.md#A13
Scenario: 連なった each の表のテストも1つと数える
  Given "tests.files" が "tests/**/*.ts" を含み、"tests/a.test.ts" に印の無い "it.only.each([1])('each %i', (n) => {})" がある
  When "kotowari check" を実行する
  Then detail が "each %i" の test_without_id の誤りが1件だけ出る
@id=EX-core-326 @about=REQ-core-082 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A53
Scenario: 関数の本体の中のテストも数える
  Given "fn helper() {" の本体の中に、印の無い "#[test]" の付いた関数 "inner" がある
  When "kotowari check" を実行する
  Then detail が "inner" の test_without_id の誤りが1件出る
@id=EX-core-329 @about=REQ-core-184 @source=docs/decision/records/2026-09-24-review10-gaps.md#A3,docs/decision/records/2026-09-24-multi-language-tests.md#A19
Scenario: 関数の中のクラスのメソッドも数える
  Given "tests.files" が "tests/**/*.py" を含み、"tests/test_a.py" の "def test_outer():" の中に "class C:" があり、その中に印の無い "def test_in_class(self):" がある
  When "kotowari check" を実行する
  Then detail が "test_in_class" の test_without_id の誤りが出る

@id=EX-core-330 @about=REQ-core-082 @source=docs/decision/records/2026-09-24-review10-gaps.md#A4
Scenario: マクロの中の mod の関数も数える
  Given "tests.rust.macros" が "proptest" だけの一覧で、"proptest!" の中の "mod nested {" の中に印の無い関数 "inside_module" がある
  When "kotowari check" を実行する
  Then detail が "inside_module" の test_without_id の誤りが1件出る

@id=EX-core-331 @about=REQ-core-183 @source=docs/decision/records/2026-09-24-review11-gaps.md#A1
Scenario: タグ付きテンプレートだけでは数えない
  Given "tests.files" が "tests/**/*.ts" を含み、"tests/a.test.ts" に印の無い "test.each`a`('tagged %s', () => {})" と、その結果を呼ばない "test.each`foo`" がある
  When "kotowari check" を実行する
  Then detail が "tagged %s" の test_without_id の誤りが1件だけ出る
```
