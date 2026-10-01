# 変更履歴

kotowari の利用者に見える変更を書く。形は [Keep a Changelog](https://keepachangelog.com/ja/1.1.0/) に、版の付け方は [Semantic Versioning](https://semver.org/lang/ja/) に従う。
`agent/skills/` の skill は kotowari と同じタグで出すので、skill の変更もここに書く。同梱の `kotowari-mds` の変更はここに書かない。

## [Unreleased]

### Added

- 配布スキルに、判断の記録への書き戻しと両役の YAML 照合記録、独立 review 後のブランチ全体検査、仕様の穴の処理先を追加した。承認済み要求を保つ委譲範囲の具体的な IR 追加を認め、変更・削除や根拠の無い意味の判断は人へ戻す。
- `kotowari changes` で指定した Git 比較元と commit または index の変更を列挙し、実装・review の照合記録と内容の鮮度を検査できる。設定の `changes` は省略可能で、導入した場合は check/status でも記録の形式と有効な参照を検査する。

### Changed

- **BREAKING** 開発中の照合記録 version: 1 から `state` を除去し、旧キーは拒否する。現在の比較を `.kotowari/changes/` の固定ファイルに上書きし、履歴は Git に残す。旧形式の過去 commit は当時のツール版で再検証する。check/status は明示された隠し記録の形式・参照を全件検査する。通常の探索は `.ignore` で記録を外せる。

## [0.2.0] - 2026-09-27

### Added

- README の入れ方に mise の `github:` で入れる方法（`mise use -g 'github:ba0918/kotowari[version_prefix=kotowari-v]@0.1.0'`）を足した。
- 面の検査。設定の `surface.files` と `surface.rules`（ast-grep の規則）でコードから利用者に見える面（CLI のサブコマンドやフラグなど）を取り出し、名前が IR の要求の文、決定表のセル、シナリオのステップに引用されていなければ `kotowari check` が `surface_without_spec` の誤りにする。面のファイルのうち読むのは規則の言語のものだけで、`src/**` のように広く書いて画像などに当たっても止まらない。規則を書かなければ何も起きない。
- まだ IR にしない面は、`surface.unspecified` が指す一覧に理由付きで載せると外せる。形の誤った1件は `surface_unspecified_invalid` の誤り、要らなくなった1件は `surface_unspecified_stale` の注意になる。外した数は `check` の最後の行 `surface: unspecified=数`（JSON は `surface`）に、面の数は `status` の `surface` の行に出る。
- `kotowari` skill に面の検査の reference `surface.md` を足し、`kotowari-adopt` に一覧の減らし方を書いた。一覧に足すのは `kotowari-brainstorm` と `kotowari-adopt` だけで、実装役は IR に無い面を壁打ちに戻す。

### Changed

- `kotowari-adopt` は範囲の確認で話題の利用者の入口（コマンドや画面の操作）も確かめ、その入口から観測できる振る舞いだけを一覧の行にする。範囲の外で見つけた振る舞いは件数と次の話題の候補だけを見せる。1行は要求1つの候補で、値や文言だけが違うものは1行にまとめて決定表の候補にし、候補が `limits.requirements` を超えたら入口が混ざっていないかを見直す。
- `kotowari status` の出力に `surface` の群（`total`、`specified`、`unspecified`）がいつも入る。面の規則を書いていなければ3つとも 0。

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

[Unreleased]: https://github.com/ba0918/kotowari/compare/kotowari-v0.2.0...HEAD
[0.2.0]: https://github.com/ba0918/kotowari/compare/kotowari-v0.1.0...kotowari-v0.2.0
[0.1.0]: https://github.com/ba0918/kotowari/releases/tag/kotowari-v0.1.0
