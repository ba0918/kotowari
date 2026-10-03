# 変更履歴

## [Unreleased]

### Changed

- **BREAKING（Rust API・パッケージ構成）** `Schema`の未検証の構築・変更を非公開にし、`Schema::parse`で意味検証を行う。`extract_validated`は検証に成功した値、`extract_partial`は途中まで得た値と指摘を返す。旧呼出コードは移行が必要。
- 取得を`kotowari-markdown-schema-io`、実行ファイルを`kotowari-mds`パッケージに分離した。バイナリのコマンド、出力、終了コードは変わらない。

### Added

- 再利用する読込み結果を返す`SchemaLoader`と、既定で無効な`tokio` featureの`AsyncSchemaLoader`を追加した。

版の基準はこのクレートの`Cargo.toml`にある。この変更にはリリース、タグ、公開を含めない。
