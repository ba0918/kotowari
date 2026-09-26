# 変更履歴

kotowari の利用者に見える変更を書く。形は [Keep a Changelog](https://keepachangelog.com/ja/1.1.0/) に、版の付け方は [Semantic Versioning](https://semver.org/lang/ja/) に従う。
`agent/skills/` の skill は kotowari と同じタグで出すので、skill の変更もここに書く。同梱の `kotowari-mds` の変更はここに書かない。

## [Unreleased]

## [0.1.0] - 2026-09-26

### Added

- 仕様を「IR」と呼ぶ決まった形の Markdown に書き、機械で検査する CLI `kotowari`。出力は既定で JSON で、人が読むときは `--format text` を付ける。
- `kotowari check`：IR の形、要求の出典にした判断の記録が実在するか、どのテストがどの要求を確かめているかを検査し、指摘を出す。終了コードは 0 が指摘なし、1 が誤りあり、2 が検査に入れなかった。利用者向けのガイドに付けた印が今の IR と食い違っていないかも見る。
- `kotowari list`：IR の項目とシナリオを、印の付いたテストと一緒に一覧にする。
- `kotowari query <ID>`：1件の本文と、それを確かめるテスト、それを指す項目を出す。
- `kotowari status`：揃っているかを数で集計し、最後の行の `complete true` か `complete false` で答える。
- `kotowari plan <ファイル>`：実装の計画のファイルの形を、同梱のスキーマで検査する。
- `kotowari mutants --tool cargo-mutants <結果のファイル>`：変異テスト（cargo-mutants）の結果を読み、見逃しを指摘にする。
- テストの印 `@kotowari[ID]` は、Rust、TypeScript、JavaScript、Python、PHP のテストなら同梱の規則で読む。ほかの言語でも、ast-grep（tree-sitter）が扱える言語なら、設定の `tests.rules` に規則を書けば読める。
- Claude Code の skill を `agent/skills/` に10個。`kotowari` は IR と判断の記録の書き方、指摘の直し方、印の置き方を教える。`kotowari-` で始まる9個は、壁打ちから計画、実装、レビューまでの工程を kotowari の上で回す（任意）。
- 入れ方は2通り。GitHub Release のビルド済みのバイナリ（Linux x86_64、macOS arm64。SHA256 付き）と、タグで版を固定した `cargo install --git https://github.com/ba0918/kotowari --tag kotowari-v0.1.0 kotowari`。skill は `gh skill install` の `--pin` にタグを渡すと版を固定して入れられる。

[Unreleased]: https://github.com/ba0918/kotowari/compare/kotowari-v0.1.0...HEAD
[0.1.0]: https://github.com/ba0918/kotowari/releases/tag/kotowari-v0.1.0
