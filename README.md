# kotowari

kotowariは、決まった形式でMarkdownに書いた仕様を検査するCLIツール。文書の形式に加え、仕様・判断・テストのつながりも機械的に検査する。

LLMに仕様を書かせて実装させると、次のようなずれが起きやすい。

- 仕様に根拠のない決定が含まれる
- 要求にテストがない
- テストがどの要求を確かめているのか分からない

kotowariでは、仕様を「IR」と呼ぶ正規化したMarkdownで記述する。`kotowari check`を実行すると、次の3点を確かめられる。

- 各要求に、その根拠となる決定が判断の記録のどこにあるかが書かれているか
- その決定が実在するか
- どのテストがその要求を確かめているか

LLMが使うことを第一に想定しており、既定ではJSONで結果を出力する。人が読む場合は、`--format text`でテキスト形式に切り替える。

## インストール

リリースのタグは`kotowari-v0.1.0`の形式になっている。  
以下のコマンド例ではバージョン`0.1.0`を指定しているので、インストールしたいバージョンに置き換えて使う。

### mise

[mise](https://mise.jdx.dev/)を使う場合は、`github:`でGitHub Releaseのバイナリをインストールできる。
`version_prefix=kotowari-v`を指定するとタグの先頭の`kotowari-v`が除かれ、バージョンを`0.1.0`のように指定できる。

```console
$ mise use -g 'github:ba0918/kotowari[version_prefix=kotowari-v]@0.1.0'
$ kotowari --version
```

### ビルド済みのバイナリ

[GitHub Release](https://github.com/ba0918/kotowari/releases)で、Linux x86_64とmacOS arm64向けのビルド済みバイナリを配布している。

配布ファイルの名前は`kotowari-v<版>-<ターゲット>.tar.gz`で、`<ターゲット>`にはLinuxの場合は`x86_64-unknown-linux-gnu`、macOSの場合は`aarch64-apple-darwin`が入る。
アーカイブには`kotowari`のバイナリ、README、ライセンスが含まれる。各アーカイブには、SHA256のチェックサムを記録した`.sha256`ファイルも付けている。

```console
$ curl -LO https://github.com/ba0918/kotowari/releases/download/kotowari-v0.1.0/kotowari-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
$ curl -LO https://github.com/ba0918/kotowari/releases/download/kotowari-v0.1.0/kotowari-v0.1.0-x86_64-unknown-linux-gnu.tar.gz.sha256
$ shasum -a 256 -c kotowari-v0.1.0-x86_64-unknown-linux-gnu.tar.gz.sha256
$ tar -xzf kotowari-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
$ install kotowari-v0.1.0-x86_64-unknown-linux-gnu/kotowari ~/.local/bin/
$ kotowari --version
```

インストール先は`~/.local/bin/`に限らず、`PATH`に含まれる任意のディレクトリに変更できる。

### ソースからビルドする

Rustのツールチェーンがあれば、`cargo install`でソースからビルドしてインストールできる。`--tag`でリリースのタグを指定すると、バージョンを固定できる。手元でビルドするため、インストールには時間がかかる。

```console
$ cargo install --git https://github.com/ba0918/kotowari --tag kotowari-v0.1.0 kotowari
$ kotowari --version
```

## 最小の例

既定では、設定や仕様、判断の記録を次の場所に置く。

```
.kotowari/config.yaml        設定（空でも無くてもよい。既定の値で動く）
docs/ir/                     IR（仕様）
docs/ir/CONTEXT.md           用語集
docs/decision/records/       判断の記録
docs/decision/adr/           ADR（空でよい）
```

要求の文に「適切に」「必要に応じて」「通常は」「など」が含まれると、曖昧語として指摘される。
この4語を既定の検査対象としているが、`.kotowari/config.yaml`の`vague_words`で変更できる。

まず、決めたことを判断の記録に書く。1行につき1つの決定を記載し、ここでは`docs/decision/records/2026-01-01-login.md`に保存する。

```markdown
# 壁打ちの記録: ログイン

## Context

ログインの失敗をどう扱うかを決める。

## Agreements

- A1 パスワードを5回続けて間違えたアカウントは15分ロックする
  - why: 総当たりを遅らせる
  - decided_by: 利用者
```

次に、`docs/ir/login.md`に要求を書く。出典には、先ほど記録した決定を指定する。

```markdown
# ログイン

ログインの失敗の扱いを扱う。

## Requirements

### REQ-001: 連続失敗でロックする

- kind: event_driven
- source: docs/decision/records/2026-01-01-login.md#A1
- verification: unit

パスワードを5回続けて間違えたとき、システムはそのアカウントを15分ロックする。
```

最後に、要求を確かめるテストに`@kotowari[ID]`の印を付ける。

```rust
// @kotowari[REQ-001]
#[test]
fn req_001_locks_after_five_failures() { /* ... */ }
```

ここまで記述して`kotowari check`を実行すると、何も出力せずに終了コード0で終了する。
テストの印を外して実行すると、次の2件が報告される。

```console
$ kotowari check --format text
docs/ir/login.md:7 [error] requirement_without_test REQ-001
src/lib.rs:2 [error] test_without_id req_001_locks_after_five_failures
```

出典の決定番号を消したり書き間違えたりしても、同じように指摘が出る。

## リファレンス

詳しい使い方は[リファレンスの目次](docs/guides/index.md)から調べられる。[設定ファイル](docs/guides/config.md)、[テストの印の付け方](docs/guides/marks.md)、[指摘の種類と直し方](docs/guides/findings.md)のほか、各コマンドの説明をまとめている。

## コマンド

| コマンド | 役目 |
|---|---|
| [`kotowari check`](docs/guides/commands/check.md) | IRの形式、出典の実在、テストの印を検査する。終了コード0は指摘なし、1は誤りあり、2は検査を開始できなかったことを示す |
| [`kotowari list`](docs/guides/commands/list.md) | IRの項目と、印の付いたテストの一覧 |
| [`kotowari query <ID>`](docs/guides/commands/query.md) | 1件の本文、テスト、逆引き |
| [`kotowari status`](docs/guides/commands/status.md) | 揃っているかを集計し、最後の行に`complete true`か`complete false`を出力する |
| [`kotowari mutants --tool cargo-mutants <結果のファイル>`](docs/guides/commands/mutants.md) | 変異テスト（cargo-mutants）の結果から見逃しを報告する |
| [`kotowari plan <計画のファイル>`](docs/guides/commands/plan.md) | 同梱のスキーマに従って、実装計画のファイルが決まった形式で書かれているかを検査する |
| [`kotowari changes --base <REV> (--head <REV> \| --staged) --phase <implementation\|review>`](docs/guides/commands/changes.md) | Gitの比較元と対象の差分を照合記録と突き合わせ、照合が漏れている箇所や記録が古くなっている箇所を報告する。設定に`changes`を記述した場合のみ使える。 |

現在、テストの印を読み取れるのはRustのみ。ただし、IRの検査はほかの言語を使うプロジェクトでも利用できる。

## LLMと一緒に使う

LLMと一緒に使うためのClaude Code用スキルを`agent/skills/`に用意している。

- `kotowari`は、IRと判断の記録の書き方、指摘の直し方、印の置き方をLLMに教える
- `kotowari-*`は、kotowariを使って壁打ちから計画、実装、レビューまでの工程を進めるためのスキルで、利用は任意

インストール方法は[agent/skills/README.md](agent/skills/README.md)に記載している。

## 同梱ツール

`crates/kotowari-markdown-schema`には、Markdown文書をYAMLのスキーマで検査する汎用CLIツール`kotowari-mds`がある。
kotowariはこの仕組みでIRを読み取っているが、`kotowari-mds`は単独でも利用できる。詳しくは[README](crates/kotowari-markdown-schema/README.md)を参照。

## ライセンス

[MIT](LICENSE-MIT)または[Apache-2.0](LICENSE-APACHE)のいずれかを選んで利用できる。
