# ライブラリの検証と値の抽出

[English](library-extraction.md) | 日本語

繰り返し使うスキーマと文書の型、検証を通った値と部分の抽出の区別、CLI の抽出との関係を扱う。公開する入口の一覧は[「ライブラリの入口」](library.ja.md)の話題の[依存するクレートは、決めた入口だけで参照の読み取りから抽出までを通せる](library.ja.md#依存するクレートは決めた入口だけで参照の読み取りから抽出までを通せる)で決める。

## スキーマは意味の検査を通したものだけを返し、解析済みのものを繰り返し使える

状態: 実装済み

### 問題

同じ内容を検証し、抽出するたびに、スキーマと文書を解析し直さずに済むようにしたい。

スキーマを解析する入口が、スキーマか、形の違反を返すことは、利用者が実装を確かめて追認した。その理由は残っていない。

`Schema` を外から組み立てたり、欄を書き換えたり、検証せずに直接 Deserialize したりできると、意味の検査を通らないスキーマで検証と抽出が行われる。検証の迂回が起きない手順にするためである。

### 決めたこと

`Schema::parse` は、スキーマの YAML を解析し、意味の検査に違反していれば失敗を返し、`Schema` を返さない。

`Schema::parse` が返した `Schema` と、`Document::parse` が返した `Document` は、何度でも検証と抽出に渡せ、そのたびに解析し直すことを求めない。

`Schema` は不変の型にする。crate の外からは、欄を指定して組み立てることも、欄を書き換えることも、`Schema::parse` を通らずに直接 Deserialize することもできない。

### 確認すること

- C-schema-libext-1 意味の検査に違反するスキーマの YAML には、`Schema::parse` が失敗を返し、`Schema` を返さない
- C-schema-libext-2 1つの解析済みの `Schema` と `Document` で `extract_validated` を繰り返し呼べ、毎回、検証を通った値が返る
- C-schema-libext-7 （読んで確認）`crates/kotowari-markdown-schema/src/schema.rs` の `Schema` の compile_fail の doctest が、crate の外から欄を指定して組み立てること、欄を書き換えること、`serde_saphyr::from_str` で直接 Deserialize することの3つを、どれもコンパイルできないものとして確かめている

## 値の抽出の通常の入口は、違反の無い文書からだけ値を返す

状態: 実装済み

### 問題

利用者が、検証を通った値と、違反のある文書から部分的に取れた値とを、取り違えないようにしたい。

### 決めたこと

`extract_validated` は文書を検証し、違反が無ければ、検証を通った値 `ValidatedValues` を返す。違反があれば指摘を返し、`ValidatedValues` は返さない。

違反のある文書から取れる値が欲しいときは、別の入口 `extract_partial` を明示して呼ぶ。`extract_partial` は、指摘と、取れた値を持つ `PartialExtraction` を返す。

どちらも、値は `serde_json::Value` で渡す。

### 確認すること

- C-schema-libext-3 違反のある文書に、`extract_validated` は指摘を返して `ValidatedValues` を返さず、`extract_partial` は同じ指摘と取れた値を返す

## ライブラリの検証は、スキーマの open を打ち消さない

状態: 実装済み

### 問題

検査は既定で閉じた世界にする。書式を固定するのが目的なので、宣言していないものを黙って通すと目的を果たさない。緩める指定は、緩める方向にしか働かない（[開いた世界は、宣言していない構造とその内側だけを許す](closed-world.ja.md#開いた世界は宣言していない構造とその内側だけを許す)で決める）。ライブラリの検証の指定でも、スキーマの open の設定を取り違えないようにしたい。

### 決めたこと

`ValidationOptions` の既定は、スキーマの open に従う。利用者が緩める指定をしたときは、スキーマの open との論理和で検証する。

| スキーマの open | `ValidationOptions` の緩める指定 | 検証 |
|---|---|---|
| 偽 | 偽（既定） | 閉じた世界 |
| 偽 | 真 | 開いた世界 |
| 真 | 偽（既定） | 開いた世界 |
| 真 | 真 | 開いた世界 |

### 確認すること

- C-schema-libext-4 スキーマの open と `ValidationOptions` の緩める指定のどちらかが真なら、`validate`、`extract_partial`、`extract_validated` は宣言していない見出しを誤りにせず、どちらも偽なら誤りにする

## CLI の values と ast --schema は、違反のある文書でも今までどおり値を出す

状態: 実装済み

### 問題

ライブラリの入口を作り直しても、既存の CLI のコマンド、引数、終了コード、出力の約束は壊さない。CLI の利用者の入力や処理を壊さないためである。

### 決めたこと

mds の `values` と `ast --schema` は、部分の抽出の値を使う。違反のある文書でも値を出し、文書の違反を理由に終了コードと出力を変えないことは、CLI の側の[values と ast --schema は、検査の合否と関係なく値を組み立てる](cli.ja.md#values-と-ast---schema-は検査の合否と関係なく値を組み立てる)で決める。

ライブラリは、そのために、素の構文木の JSON を `ast_json` で、型付きの抽出の JSON を `extract_typed_partial` で提供する。それぞれの JSON の形は、[ast は、検査とは別に、文書の素の構文木を JSON で出す](extraction.ja.md#ast-は検査とは別に文書の素の構文木を-json-で出す)と[ast --schema は、values --format json と同じ値に、スキーマの name を type として添える](cli.ja.md#ast---schema-はvalues---format-json-と同じ値にスキーマの-name-を-type-として添える)で決める。

### 確認すること

- C-schema-libext-5 違反のある文書にも、`extract_typed_partial` は、取れた値とスキーマの名前を入れた型付きの抽出の JSON を、指摘と一緒に返す
