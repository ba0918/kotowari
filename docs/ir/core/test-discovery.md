# テストの見つけ方

テストのファイルの選び方と、tree-sitter と同梱の問い合わせでテストを数える規則を扱う。

## 要求

### REQ-core-079: テストのファイル

- 種類: ubiquitous
- 出典: docs/decision/records/records.md#A36, docs/decision/records/records.md#A47, docs/decision/records/records.md#A94, docs/decision/records/records.md#A102, docs/decision/records/records.md#A146, docs/decision/records/records.md#A159, docs/decision/records/records.md#A165
- 検証: unit

kotowari は常に、"tests.files" の glob に当たるファイルを`テストのファイル`として読む。ディレクトリでも通常のファイルでもないもの（ソケット、名前付きパイプ、デバイス）は glob に当たっても読まない（`除外`）。走査は`基準のディレクトリ`の全体（隠しディレクトリを除く）を歩いてから glob で選ぶので、glob に当たらない場所でも読めないディレクトリと先の無いシンボリックリンクで`停止`する。走査ではディレクトリのシンボリックリンクを辿らず、ファイルのシンボリックリンクは読み、先の無いシンボリックリンクでは読めないファイルを理由に`停止`する。

### REQ-core-080: tree-sitter で読む

- 種類: ubiquitous
- 出典: docs/decision/records/records.md#A24, docs/decision/records/records.md#A58
- 検証: unit

kotowari は常に、`問い合わせのある言語`の`テストのファイル`を tree-sitter で読み、同梱の`問い合わせ`で`テスト`を見つける。

### REQ-core-081: 拡張子と言語の対応

- 種類: ubiquitous
- 出典: docs/decision/records/records.md#A39, docs/decision/records/records.md#A58, docs/decision/records/records.md#A24, docs/decision/records/records.md#A123, docs/decision/records/records.md#A128
- 検証: unit

kotowari は常に、`テストのファイル`の言語を同梱の対応で拡張子から大文字小文字を区別して決め、第1版では ".rs" だけを`問い合わせのある言語`にする。

### REQ-core-082: Rust のテスト

- 種類: algorithm
- 出典: docs/decision/records/records.md#A26, docs/decision/records/records.md#A39, docs/decision/records/records.md#A49
- 定義: TBL-core-017
- 検証: unit

### REQ-core-083: 読めないテストのファイル

- 種類: event_driven
- 出典: docs/decision/records/records.md#A58, docs/decision/records/records.md#A120, docs/decision/records/records.md#A149
- 検証: unit

tree-sitter で読めない`テストのファイル`（構文の誤りが1つでもあるファイル）があるとき、kotowari は unparsable_file の`誤り`を出してそのファイルを飛ばし、`停止`しない。"tests.rust.macros" のマクロの中身を読み直したときの構文の誤りはこれに含めず、読めた最上位の関数だけを数える。

### REQ-core-084: 正規表現でテストを見つけない

- 種類: prohibition
- 出典: docs/decision/records/records.md#R3
- 検証: review
- 確かめ方: `src/tests_discovery.rs` に正規表現によるテスト検出がないことを確認。tree-sitter のみ使用

kotowari は、`設定ファイル`に書く正規表現でテストの定義の行を見つけてはならない。

## 決定表

### TBL-core-017: Rust でテストと数えるもの

- 出典: docs/decision/records/records.md#A26, docs/decision/records/records.md#A39, docs/decision/records/records.md#A49, docs/decision/records/records.md#A47, docs/decision/records/records.md#A121, docs/decision/records/records.md#A122

| 対象 | 数え方 |
|---|---|
| 属性のパスの末尾の要素が "test" の関数（"#[test]"、"#[ test ]"、"#[core::prelude::v1::test]"、"#[tokio::test]"） | 同梱の問い合わせに固定し、常に数える |
| tests.rust.attributes の属性の付いた関数 | 属性から "#["、"]"、引数を除いたパスが完全一致すれば数える |
| tests.rust.macros のマクロ | 設定には "!" を除いた名前で書く。マクロの名前の末尾の要素が一致すれば、中身を Rust の項目として読み直し、最上位の関数ごとに数える。印の結び付けと invalid_marker は通常の関数と同じ |

## 具体例

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
```
