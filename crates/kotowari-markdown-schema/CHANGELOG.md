# 変更履歴

## [Unreleased]

### Changed

- 最小対応 Rust バージョン（`rust-version`）を 1.89 から 1.99 に上げた（`kotowari-markdown-schema`、`kotowari-markdown-schema-io`、`kotowari-markdown-view`、`kotowari-mds`）。
- **BREAKING（Rust API・パッケージ構成）** `Schema`の未検証の構築・変更を非公開にし、`Schema::parse`で意味検証を行う。`extract_validated`は検証に成功した値、`extract_partial`は途中まで得た値と指摘を返す。旧呼出コードは移行が必要。
- 取得を`kotowari-markdown-schema-io`、実行ファイルを`kotowari-mds`パッケージに分離した。バイナリのコマンド、出力、終了コードは変わらない。

### Added

- 再利用する読込み結果を返す`SchemaLoader`と、既定で無効な`tokio` featureの`AsyncSchemaLoader`を追加した。
- Markdownの文章と種類の決まった部品からなる文書を、メモリの中で静的なHTMLのページにする`kotowari-markdown-view`クレートを追加した。8種の部品のJSON Schemaを`part_schema`で返す。ファイルを読み書きせず、ページは外から何も読み込まない。

### Fixed

- スキーマの`extract`の`value`と`of`の鍵で、ドットで区切った名前が空のもの（`""`や`a..b`）を`path`と同じく`schema_invalid`で止めるようにした。これまでは通り、抽出の値に空文字列の鍵ができていた。
- 段落の読み方で、箇条書きの`pattern`をマーカーの行だけに当てるようにした。これまでは遅延継続の行（字下げせずに続けた行）も含めて照合し、行の読み方と結果が食い違っていた。
- CRLF の文書で、段落の読み方の箇条書き・フィールド行の値に`\r`が混ざらないようにした。値の行は LF の文書と同じく改行1つでつなぐ。これまでは子の行や折り返し行との間が`\r\n`になり、行の読み方とも値が食い違っていた。

版の基準はこのクレートの`Cargo.toml`にある。この変更にはリリース、タグ、公開を含めない。
