# テストの見つけ方

テストのファイルの選び方と、拡張子から言語を決め、その言語の問い合わせでテストとその名前を見つける規則を扱う。言語ごとに同梱する問い合わせの中身は test-queries.md、設定で足す問い合わせは query-rules.md が扱う。

## Requirements

### REQ-core-079: テストのファイル

- kind: ubiquitous
- source: docs/decision/records/records.md#A36, docs/decision/records/records.md#A47, docs/decision/records/records.md#A94, docs/decision/records/records.md#A102, docs/decision/records/records.md#A146, docs/decision/records/records.md#A159, docs/decision/records/records.md#A165
- verification: unit

kotowari は常に、"tests.files" の glob に当たるファイルを`テストのファイル`として読む。ディレクトリでも通常のファイルでもないもの（ソケット、名前付きパイプ、デバイス）は glob に当たっても読まない（`除外`）。走査は`基準のディレクトリ`の全体（隠しディレクトリを除く）を歩いてから glob で選ぶので、glob に当たらない場所でも読めないディレクトリと先の無いシンボリックリンクで`停止`する。走査ではディレクトリのシンボリックリンクを辿らず、ファイルのシンボリックリンクは読み、先の無いシンボリックリンクでは読めないファイルを理由に`停止`する。

### REQ-core-080: tree-sitter で読む

- kind: ubiquitous
- source: docs/decision/records/records.md#A24, docs/decision/records/records.md#A58, docs/decision/records/2026-09-24-multi-language-tests.md#A1, docs/decision/records/2026-09-24-multi-language-tests.md#A7
- verification: unit

kotowari は常に、`問い合わせのある言語`の`テストのファイル`を tree-sitter で読み、その言語の`問い合わせ`をすべて当てて`テスト`を見つける。`問い合わせの無い言語`の`テストのファイル`は tree-sitter で読まない。

### REQ-core-081: 拡張子と言語の対応

- kind: algorithm
- source: docs/decision/records/records.md#A39, docs/decision/records/records.md#A58, docs/decision/records/records.md#A24, docs/decision/records/records.md#A123, docs/decision/records/2026-09-24-multi-language-tests.md#A3, docs/decision/records/2026-09-24-multi-language-tests.md#A6, docs/decision/records/2026-09-24-multi-language-tests.md#A21
- definition: TBL-core-031
- verification: unit

### REQ-core-083: 読めないテストのファイル

- kind: event_driven
- source: docs/decision/records/records.md#A58, docs/decision/records/records.md#A120, docs/decision/records/records.md#A149, docs/decision/records/2026-09-24-multi-language-tests.md#A23
- verification: unit

tree-sitter で読めない`問い合わせのある言語`の`テストのファイル`（構文の誤りが1つでもあるファイル）があるとき、kotowari は unparsable_file の`誤り`を出してそのファイルを飛ばし、`停止`しない。"tests.rust.macros" のマクロの中身を読み直したときの構文の誤りはこれに含めず、読めた最上位の関数だけを数える。

### REQ-core-084: 正規表現でテストを見つけない

- kind: prohibition
- source: docs/decision/records/records.md#R3, docs/decision/records/2026-09-24-multi-language-tests.md#A2
- verification: review
- how_to_verify: テストを見つけるコードが、テストのファイルの行に正規表現を当てて定義の行を探していないことを確認。設定から来る正規表現は ast-grep のルールの中で構文木の節に当てるものだけ

kotowari は、`設定ファイル`に書く正規表現でテストの定義の行を見つけてはならない。

### REQ-core-180: テストの節と名前

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-multi-language-tests.md#A10, docs/decision/records/2026-09-24-multi-language-tests.md#A13, docs/decision/records/2026-09-24-multi-language-tests.md#A25, docs/decision/records/2026-09-24-multi-language-tests.md#A36
- verification: unit

kotowari は常に、`問い合わせ`が当たった構文木の節を1つの`テスト`とし、メタ変数 "$NAME" に入った節の文字を`テスト`の名前にする。その文字の最初と最後が同じ引用符（一重引用符、二重引用符、バッククォートのいずれか）なら、その1文字ずつを外し、それ以外の形はそのまま名前にする。`問い合わせ`が "$NAME" を捕まえないとき、`テスト`の名前は null にする。

### REQ-core-181: 重なった当たり

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-multi-language-tests.md#A27
- verification: unit

kotowari は常に、同じ節に複数の`問い合わせ`が当たったときその節を1つの`テスト`と数え、`テスト`の節の中でさらに当たった節は別の`テスト`として数える。

## Decision tables

### TBL-core-031: 拡張子と言語の対応

- source: docs/decision/records/2026-09-24-multi-language-tests.md#A3, docs/decision/records/2026-09-24-multi-language-tests.md#A6, docs/decision/records/2026-09-24-multi-language-tests.md#A21, docs/decision/records/records.md#A123, docs/decision/records/2026-09-24-multi-language-tests.md#A7, docs/decision/records/2026-09-17-check-reach.md#A19, docs/decision/records/2026-09-24-multi-language-tests.md#A46, docs/decision/records/2026-09-24-multi-language-tests.md#A47

拡張子はファイル名の最後の "." より後ろの文字で、大文字小文字を区別する。表に無い拡張子のファイルは言語が決まらず、問い合わせの無い言語として扱う。

| 言語 | 拡張子 |
|---|---|
| Bash | bash、bats、cgi、command、env、fcgi、ksh、sh、tmux、tool、zsh |
| C | c、h |
| Cpp | cc、hpp、cpp、c++、hh、cxx、cu、ino |
| CSharp | cs |
| Css | css、scss |
| Dart | dart |
| Elixir | ex、exs |
| Go | go |
| Haskell | hs |
| Hcl | hcl、nomad、tf、tfvars、workflow |
| Html | html、htm、xhtml |
| Java | java |
| JavaScript | cjs、js、mjs、jsx |
| Json | json |
| Kotlin | kt、ktm、kts |
| Lua | lua |
| Markdown | markdown、md |
| Nix | nix |
| Php | php |
| Python | py、py3、pyi、bzl、bazel |
| Ruby | rb、rbw、gemspec |
| Rust | rs |
| Scala | scala、sc、sbt |
| Solidity | sol |
| Swift | swift |
| TypeScript | ts、cts、mts |
| Tsx | tsx |
| Yaml | yaml、yml |

## Examples

```gherkin
@id=EX-core-293 @about=REQ-core-080 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A7,docs/decision/records/2026-09-24-multi-language-tests.md#A23
Scenario: 問い合わせの無い言語のファイルは構文木を読まない
  Given "tests.files" が "tests/**/*.go" を含み、"tests/a.go" に構文の誤りがあり、".go" の`問い合わせ`は無い
  When "kotowari check" を実行する
  Then unparsable_file の誤りは出ない

@id=EX-core-294 @about=REQ-core-083 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A23,docs/decision/records/2026-09-24-multi-language-tests.md#A8,docs/decision/records/2026-09-24-multi-language-tests.md#A21
Scenario: 問い合わせのある言語のファイルの構文の誤り
  Given "tests.files" が "tests/**/*.ts" を含み、"tests/a.test.ts" が "it('x', () => {" で終わる
  When "kotowari check" を実行する
  Then "tests/a.test.ts" の unparsable_file の誤りが1件出る

@id=EX-core-295 @about=REQ-core-081 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A21,docs/decision/records/2026-09-24-multi-language-tests.md#A6,docs/decision/records/records.md#A123
Scenario: 大文字の拡張子は言語が決まらない
  Given "tests.files" が "tests/**/*" を含み、"tests/A.PY" に印の無い "def test_x():" がある
  When "kotowari check" を実行する
  Then test_without_id の誤りは出ない

@id=EX-core-296 @about=REQ-core-180 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A13,docs/decision/records/2026-09-24-multi-language-tests.md#A25,docs/decision/records/2026-09-24-multi-language-tests.md#A18,docs/decision/records/2026-09-24-multi-language-tests.md#A15,docs/decision/records/2026-09-24-multi-language-tests.md#A16
Scenario: 文字列の名前は引用符を外す
  Given "tests.files" が "tests/**/*.ts" を含み、"tests/a.test.ts" の "describe('d', ...)" の中の 3 行目の印 "@kotowari[REQ-001]" の直後に "it('does x', () => {})" がある
  When "kotowari list" を実行する
  Then "REQ-001" の "tests" の "name" は "does x" である

@id=EX-core-297 @about=REQ-core-181 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A27,docs/decision/records/2026-09-24-multi-language-tests.md#A18,docs/decision/records/2026-09-24-multi-language-tests.md#A13
Scenario: テストの中のテストは別に数える
  Given "tests.files" が "tests/**/*.ts" を含み、"tests/a.test.ts" に印の無い "it('outer', ...)" があり、その中に印の無い "it('inner', ...)" がある
  When "kotowari check" を実行する
  Then detail が "outer" と "inner" の test_without_id の誤りが1件ずつ出る
```

