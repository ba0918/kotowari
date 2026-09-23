# テストの印

テストに書く印の構文と、印をテストに結び付ける規則を扱う。

## Requirements

### REQ-core-071: 印の構文

- kind: algorithm
- source: docs/decision/records/records.md#A14, docs/decision/records/records.md#A57
- definition: TBL-core-015
- verification: unit

### REQ-core-072: 形の誤った印

- kind: event_driven
- source: docs/decision/records/records.md#A57, docs/decision/records/records.md#A67, docs/decision/records/records.md#A39, docs/decision/records/records.md#A121, docs/decision/records/records.md#A111, docs/decision/records/2026-09-24-multi-language-tests.md#A15
- verification: unit

`問い合わせのある言語`でどの`テスト`の`直前のコメントの塊`にも無いものを除く`印`について、その中が空か区切りだけのとき、またはその`印`に同じ行の閉じ括弧が無いとき、kotowari は行の文字を detail にして invalid_marker の`誤り`を出す。

### REQ-core-073: 1行に複数の印

- kind: ubiquitous
- source: docs/decision/records/records.md#A57
- verification: unit

kotowari は常に、1行の中の`印`をすべて拾う。

### REQ-core-074: コメント記号を見ない

- kind: ubiquitous
- source: docs/decision/records/records.md#A14
- verification: unit

kotowari は常に、`印`を行のどの位置からも拾い、コメント記号を見ない。

### REQ-core-075: 印の結び付け

- kind: algorithm
- source: docs/decision/records/records.md#A26, docs/decision/records/records.md#A34, docs/decision/records/records.md#A39, docs/decision/records/records.md#A57, docs/decision/records/2026-09-24-multi-language-tests.md#A15, docs/decision/records/2026-09-24-multi-language-tests.md#A16, docs/decision/records/2026-09-24-multi-language-tests.md#A17, docs/decision/records/2026-09-24-multi-language-tests.md#A26, docs/decision/records/2026-09-24-multi-language-tests.md#A39, docs/decision/records/2026-09-24-multi-language-tests.md#A40, docs/decision/records/2026-09-24-multi-language-tests.md#A41, docs/decision/records/2026-09-24-multi-language-tests.md#A50, docs/decision/records/2026-09-24-multi-language-tests.md#A51, docs/decision/records/2026-09-24-multi-language-tests.md#A52
- definition: TBL-core-016, TBL-core-035
- verification: unit

### REQ-core-076: 問い合わせの無い言語の印

- kind: event_driven
- source: docs/decision/records/records.md#A39, docs/decision/records/records.md#A57
- verification: unit

`問い合わせの無い言語`の`テストのファイル`を読むとき、kotowari はコメントかどうかを問わず、ファイルの文字の中の`印`をすべて拾う。

### REQ-core-077: 存在しない ID だけを指す印

- kind: event_driven
- source: docs/decision/records/records.md#A57, docs/decision/records/records.md#A89
- verification: unit

`印`が存在しない`ID`だけを指すとき、kotowari はその`印`の結び付いた`テスト`を`印`のあるものと数える。`印`の角括弧の中の`ID`の形でない要素（"REQ001" のように区切りの無いもの）は、存在しない`ID`を指したものとして unresolved_reference の`誤り`を出す。

### REQ-core-078: review の要求を指す印

- kind: event_driven
- source: docs/decision/records/records.md#A39
- verification: unit

`印`が検証の値 "review" の`要求`を指すとき、kotowari はそれを`誤り`にしない。

### REQ-core-118: 印の指摘の行

- kind: ubiquitous
- source: docs/decision/records/records.md#A121
- verification: unit

kotowari は常に、`印`から出す unresolved_reference と invalid_marker の "line" を`印`のある行（行をまたぐ`印`なら "@kotowari[" のある行）にする。

## Decision tables

### TBL-core-015: 印の構文

- source: docs/decision/records/records.md#A14, docs/decision/records/records.md#A57

| 部分 | 形 |
|---|---|
| 始まり | @kotowari[ |
| 中身 | ID をコンマで区切って並べる。コンマの前後に空白を置いてよい |
| 終わり | ] |

### TBL-core-016: 印の結び付け（問い合わせのある言語）

- source: docs/decision/records/records.md#A26, docs/decision/records/records.md#A34, docs/decision/records/records.md#A39, docs/decision/records/records.md#A57, docs/decision/records/records.md#A67, docs/decision/records/records.md#A121, docs/decision/records/2026-09-24-multi-language-tests.md#A15, docs/decision/records/2026-09-24-multi-language-tests.md#A16, docs/decision/records/2026-09-24-multi-language-tests.md#A26, docs/decision/records/2026-09-24-multi-language-tests.md#A41, docs/decision/records/2026-09-24-multi-language-tests.md#A49, docs/decision/records/2026-09-24-multi-language-tests.md#A52

| 印の位置 | 扱い |
|---|---|
| テストの直前のコメントの塊 | そのテストに結び付ける。最初の行が同じテストが2つ以上あるときは、その行で最初に始まるテストにだけ結び付ける |
| テストの節の中（関数の本体の先頭のコメントを含む）。ただし節の中にある別のテストの直前のコメントの塊は除き、1行目のとおりその別のテストに結び付ける | 無視し、invalid_marker も unresolved_reference も出さない |
| どのテストの直前のコメントの塊にも無い | 無視し、invalid_marker も unresolved_reference も出さない |
| "tests.rust.macros" のマクロの中の関数 | 上と同じ規則を適用する |

### TBL-core-035: コメントの塊とテストの間に挟んでよい行

- source: docs/decision/records/2026-09-24-multi-language-tests.md#A16, docs/decision/records/2026-09-24-multi-language-tests.md#A17, docs/decision/records/records.md#A39, docs/decision/records/2026-09-24-multi-language-tests.md#A39, docs/decision/records/2026-09-24-multi-language-tests.md#A40, docs/decision/records/2026-09-24-multi-language-tests.md#A50, docs/decision/records/2026-09-24-multi-language-tests.md#A51, docs/decision/records/2026-09-24-review10-gaps.md#A1

直前のコメントの塊は、テストの節の最初の行の直前から上に向かって、空行が来るまで続く、コメントだけの行とこの表の挟んでよい行の塊。コメントだけの行は、前後の空白（Unicode の空白。全角空白と NBSP を含む）を除いた行の文字がすべてコメント（tree-sitter の extra の節）の文字である行で、複数行のコメントの途中の行を含み、コードと同じ行にあるコメントの行は含まない。コメントだけの行と挟んでよい行は空行なしで混ざってよく、複数行にわたる属性とデコレータはその全部の行を挟んでよい行とする。複数行のコメント、属性、デコレータの途中にある空白だけの行は塊を切らない。塊の行のうち、印を読むのはコメントの文字だけで、属性やデコレータの本体にある印は読まない。

| 言語 | 挟んでよい行 |
|---|---|
| Rust | 属性（"#[...]"）の行 |
| Python | デコレータ（"@..."）の行 |
| そのほかの言語 | 無し |

## Examples

```gherkin
@id=EX-core-015 @about=REQ-core-073 @source=docs/decision/records/records.md#A57,docs/decision/records/records.md#A26,docs/decision/records/records.md#A39
Scenario: 1行の2つの印を両方拾う
  Given `テスト`の直前のコメントに "@kotowari[REQ-001] @kotowari[TBL-002]" がある
  When "kotowari check" を実行する
  Then その`テスト`は "REQ-001" と "TBL-002" に結び付く

@id=EX-core-016 @about=REQ-core-075 @source=docs/decision/records/records.md#A39,docs/decision/records/records.md#A47,docs/decision/records/ir-form.md#検査の種類,docs/decision/records/records.md#A26,docs/decision/records/records.md#A49
Scenario: 空行を挟んだコメントの印は結び付かない
  Given "@kotowari[REQ-001]" のコメントと "#[test]" の関数の間に空行がある
  When "kotowari check" を実行する
  Then その関数に test_without_id の誤りが出る

@id=EX-core-306 @about=REQ-core-075 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A15
Scenario: 関数の本体の先頭の印は結び付かない
  Given "#[test]" の付いた関数の本体の最初の行が "// @kotowari[REQ-001]" で、関数の直前にコメントが無い
  When "kotowari check" を実行する
  Then その関数に test_without_id の誤りが出る

@id=EX-core-307 @about=REQ-core-075 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A16,docs/decision/records/2026-09-24-multi-language-tests.md#A17,docs/decision/records/2026-09-24-multi-language-tests.md#A19
Scenario: デコレータを挟んだ印は結び付く
  Given "tests.files" が "tests/**/*.py" を含み、"tests/test_a.py" に "# @kotowari[REQ-001]" の行、"@pytest.mark.parametrize('a', [1])" の行、"def test_x(a):" の行が空行なしで続く
  When "kotowari check" を実行する
  Then test_without_id の誤りは出ず、"test_x" は "REQ-001" に結び付く

@id=EX-core-308 @about=REQ-core-075 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A16,docs/decision/records/2026-09-24-multi-language-tests.md#A19
Scenario: クラスの中のメソッドの直前のコメントも結び付く
  Given "tests.files" が "tests/**/*.py" を含み、"tests/test_a.py" の "class TestBar:" の中に "# @kotowari[REQ-001]" の行と "def test_baz(self):" の行が空行なしで続く
  When "kotowari check" を実行する
  Then test_without_id の誤りは出ず、"test_baz" は "REQ-001" に結び付く

@id=EX-core-309 @about=REQ-core-075 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A16,docs/decision/records/2026-09-24-multi-language-tests.md#A17,docs/decision/records/2026-09-24-multi-language-tests.md#A18,docs/decision/records/2026-09-24-multi-language-tests.md#A13
Scenario: 挟んでよい行の無い言語でコードの行を挟むと切れる
  Given "tests.files" が "tests/**/*.ts" を含み、"tests/a.test.ts" に "// @kotowari[REQ-001]" の行、"const n = 1;" の行、"it('x', () => {});" の行が空行なしで続く
  When "kotowari check" を実行する
  Then detail が "x" の test_without_id の誤りが出る
@id=EX-core-318 @about=REQ-core-075 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A40,docs/decision/records/2026-09-24-multi-language-tests.md#A17,docs/decision/records/2026-09-24-multi-language-tests.md#A19
Scenario: 複数行のデコレータを挟んでも結び付く
  Given "tests.files" が "tests/**/*.py" を含み、"tests/test_a.py" に "# @kotowari[REQ-001]" の行、3行にわたる "@pytest.mark.parametrize(" のデコレータ、"# note" の行、"def test_x(a):" の行が空行なしで続く
  When "kotowari check" を実行する
  Then test_without_id の誤りは出ず、"test_x" は "REQ-001" に結び付く

@id=EX-core-319 @about=REQ-core-075 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A41,docs/decision/records/2026-09-24-multi-language-tests.md#A27,docs/decision/records/2026-09-24-multi-language-tests.md#A18
Scenario: 入れ子のテストの直前の印は内側に結び付く
  Given "tests.files" が "tests/**/*.ts" を含み、"tests/a.test.ts" の "it('outer', ...)" の直前に "// @kotowari[REQ-001]" があり、その中の "it('inner', ...)" の直前に "// @kotowari[REQ-002]" がある
  When "kotowari check" を実行する
  Then test_without_id の誤りは出ず、"outer" は "REQ-001" に、"inner" は "REQ-002" に結び付く

@id=EX-core-320 @about=REQ-core-075 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A39,docs/decision/records/2026-09-24-multi-language-tests.md#A18,docs/decision/records/2026-09-24-multi-language-tests.md#A13,docs/decision/records/2026-09-24-multi-language-tests.md#A16,docs/decision/records/2026-09-24-multi-language-tests.md#A17
Scenario: コードと同じ行のコメントは塊を切る
  Given "tests.files" が "tests/**/*.ts" を含み、"tests/a.test.ts" に "// @kotowari[REQ-001]" の行、"setup(); // prepare" の行、"it('x', () => {});" の行が空行なしで続く
  When "kotowari check" を実行する
  Then detail が "x" の test_without_id の誤りが出る
@id=EX-core-323 @about=REQ-core-075 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A50,docs/decision/records/2026-09-24-multi-language-tests.md#A40
Scenario: 複数行のデコレータの途中の空行では塊が切れない
  Given "tests.files" が "tests/**/*.py" を含み、"tests/test_a.py" に "# @kotowari[REQ-001]" の行、途中に空白だけの行を含む3行以上の "@pytest.mark.parametrize(" のデコレータ、"def test_x(a):" の行が続き、デコレータの外に空行は無い
  When "kotowari check" を実行する
  Then test_without_id の誤りは出ず、"test_x" は "REQ-001" に結び付く

@id=EX-core-324 @about=REQ-core-075 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A51
Scenario: 属性の本体の印は読まない
  Given 文字列の値に "@kotowari[REQ-999]" を持つ "#[doc = ...]" の属性の行の直後に "#[test]" の付いた関数があり、その直前にコメントは無く、"REQ-999" は存在しない
  When "kotowari check" を実行する
  Then unresolved_reference の誤りは出ず、その関数に test_without_id の誤りが出る

@id=EX-core-325 @about=REQ-core-075 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A52
Scenario: 同じ行に始まるテストは最初のものだけに結び付く
  Given "// @kotowari[REQ-001]" の行の直後の行が "#[test] fn a() {} #[test] fn b() {}" である
  When "kotowari check" を実行する
  Then "a" は "REQ-001" に結び付き、detail が "b" の test_without_id の誤りが1件出る
@id=EX-core-327 @about=REQ-core-075 @source=docs/decision/records/2026-09-24-review10-gaps.md#A1,docs/decision/records/2026-09-24-multi-language-tests.md#A16,docs/decision/records/2026-09-24-multi-language-tests.md#A18
Scenario: 全角空白の後のコメントも塊に入る
  Given "tests.files" が "tests/**/*.ts" を含み、"tests/a.test.ts" に全角空白で始まる "// @kotowari[REQ-001]" の行と "it('x', () => {});" の行が空行なしで続く
  When "kotowari check" を実行する
  Then test_without_id の誤りは出ず、"x" は "REQ-001" に結び付く
```
