# テストの見つけ方

テストのファイルの選び方と、tree-sitter と同梱の問い合わせでテストを数える規則を扱う。

## 要求

### REQ-079: テストのファイル

- 種類: ubiquitous
- 出典: brainstorm/records.md#A36, brainstorm/records.md#A47
- 検証: unit

kotowari は常に、"tests.files" の glob に当たるファイルを`テストのファイル`として読む。

### REQ-080: tree-sitter で読む

- 種類: ubiquitous
- 出典: brainstorm/records.md#A24, brainstorm/records.md#A58
- 検証: unit

kotowari は常に、`問い合わせのある言語`の`テストのファイル`を tree-sitter で読み、同梱の`問い合わせ`で`テスト`を見つける。

### REQ-081: 拡張子と言語の対応

- 種類: ubiquitous
- 出典: brainstorm/records.md#A39, brainstorm/records.md#A58, brainstorm/records.md#A24
- 検証: unit

kotowari は常に、`テストのファイル`の言語を同梱の対応で拡張子から決め、003 では ".rs" だけを`問い合わせのある言語`にする。Rust 以外の言語の`問い合わせ`は、利用者の設定ではなく、kotowari に`問い合わせ`のファイルを足すことで後から足せる。

### REQ-082: Rust のテスト

- 種類: algorithm
- 出典: brainstorm/records.md#A26, brainstorm/records.md#A39, brainstorm/records.md#A49
- 定義: TBL-017
- 検証: unit

### REQ-083: 読めないテストのファイル

- 種類: event_driven
- 出典: brainstorm/records.md#A58
- 検証: unit

tree-sitter で読めない`テストのファイル`があるとき、kotowari は unparsable_file の`誤り`を出してそのファイルを飛ばし、`停止`しない。

### REQ-084: 正規表現でテストを見つけない

- 種類: prohibition
- 出典: brainstorm/records.md#R3
- 検証: review

kotowari は、`設定ファイル`に書く正規表現でテストの定義の行を見つけてはならない。

## 決定表

### TBL-017: Rust でテストと数えるもの

- 出典: brainstorm/records.md#A26, brainstorm/records.md#A39, brainstorm/records.md#A49, brainstorm/records.md#A47

| 対象 | 数え方 |
|---|---|
| #[test] の付いた関数 | 同梱の問い合わせに固定し、常に数える |
| tests.rust.attributes の属性の付いた関数 | 属性から "#["、"]"、引数を除いたパスが完全一致すれば数える |
| tests.rust.macros のマクロ | 設定には "!" を除いた名前で書く。マクロの名前の末尾の要素が一致すれば、中身を Rust の項目として読み直し、関数ごとに数える |

## 具体例

```gherkin
@id=EX-017 @about=REQ-082 @source=brainstorm/records.md#A39,brainstorm/records.md#A49,brainstorm/records.md#A47
Scenario: 引数付きの属性も数える
  Given "tests.rust.attributes" が "kani::proof" だけの一覧である
  And "#[kani::proof(unwind = 3)]" の付いた関数がある
  When "kotowari check" を実行する
  Then その関数を`テスト`と数える

@id=EX-018 @about=REQ-082 @source=brainstorm/records.md#A49,brainstorm/records.md#A26,brainstorm/records.md#A39,brainstorm/records.md#A47
Scenario: パスの付いたマクロも数える
  Given "tests.rust.macros" が "proptest" だけの一覧である
  And "proptest::proptest!" の中に関数が2つある
  When "kotowari check" を実行する
  Then `テスト`を2つ数える
```
