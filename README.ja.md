# kotowari

[English](README.md) | 日本語

kotowariは、決まった形式でMarkdownに書いた仕様を検査するCLIツール。文書の形式に加え、仕様・判断・テストのつながりも機械的に検査する。

LLMに仕様を書かせて実装させると、次のようなずれが起きやすい。

- 仕様に理由のない決定が含まれる
- 決定にテストがない
- テストがどの決定を確かめているのか分からない

kotowariでは、仕様を決定の文書で記述する。話題ごとに1つのMarkdownのファイルに、今有効な決定を、その問題、決めたこと、見送った案、守られていることを示す確認することと一緒に書く。`kotowari check`を実行すると、次の3点を確かめられる。

- 各文書と各決定が決まった形か
- 実装済みの決定の確認することの行のそれぞれを、テストが指しているか
- どのテストがどの行を確かめているか

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
現在のソースツリーでは、CLIパッケージは`kotowari-cli`と`kotowari-mds`です。
チェックアウトから入れる場合は`cargo install --path . --bin kotowari`と`cargo install --path crates/kotowari-mds --bin kotowari-mds`を使います。
上の旧タグの例は、そのタグにある旧パッケージ名を使っています。
Rustからの呼出は[公開クレートAPI](docs/guides/public-crate-api.ja.md)を参照してください。実行ファイルのコマンド、出力、終了コードは変わりません。

既定では、設定と仕様を次の場所に置く。

```
.kotowari/config.yaml        設定（空でも無くてもよい。既定の値で動く）
docs/decisions/<群>/         決定の文書（仕様）。話題ごとに1ファイル
docs/decisions/<群>/CONTEXT.md  用語集（人が読む。check は読まない）
```

決定の決めたことか確認することに「適切に」「必要に応じて」「通常は」「など」が含まれると、曖昧語として指摘される。
この4語を既定の検査対象としているが、`.kotowari/config.yaml`の`vague_words`で変更できる。

決めたことを決定の文書に書く。ここでは`docs/decisions/login/login.md`に保存する。`### Confirmations`の下の行が確かめる結果で、そのIDは`C-login-login-1`。

```markdown
# Login

Covers how failed logins are handled.

## An account is locked for 15 minutes after five wrong passwords in a row

Status: implemented

### Problem

Passwords could be guessed by trying many in a short time.

### Decision

When the password is entered wrongly five times in a row, the account is locked for 15 minutes.

### Confirmations

- C-login-login-1 After five wrong passwords in a row, a sixth attempt with the right password is refused
```

次に、その行を確かめるテストに、行のIDで`@kotowari[ID]`の印を付ける。

```rust
// @kotowari[C-login-login-1]
#[test]
fn locks_after_five_failures() { /* ... */ }
```

ここまで記述して`kotowari check`を実行すると、何も出力せずに終了コード0で終了する。
テストの印を外して実行すると、次の2件が報告される。

```console
$ kotowari check --format text
docs/decisions/login/login.md:19 [error] confirmation_without_test C-login-login-1
src/lib.rs:2 [error] test_without_id locks_after_five_failures
```

`Status: proposed`（決めたが、まだ作っていない）で書いた決定はテストを求めないので、テストより先に仕様をコミットできる。

## リファレンス

詳しい使い方は[リファレンスの目次](docs/guides/index.ja.md)から調べられる。[設定ファイル](docs/guides/config.ja.md)、[テストの印の付け方](docs/guides/marks.ja.md)、[指摘の種類と直し方](docs/guides/findings.ja.md)のほか、各コマンドの説明をまとめている。

## コマンド

| コマンド | 役目 |
|---|---|
| [`kotowari check`](docs/guides/commands/check.ja.md) | 決定の文書の形式とテストの印を検査する。終了コード0は指摘なし、1は誤りあり、2は検査を開始できなかったことを示す |
| [`kotowari list`](docs/guides/commands/list.ja.md) | 決定と確認することの行と、印の付いたテストの一覧 |
| [`kotowari query <ID>`](docs/guides/commands/query.ja.md) | 1つの決定か行の本文、テスト、その決定へのリンク |
| [`kotowari status`](docs/guides/commands/status.ja.md) | 揃っているかを集計し、最後の行に`complete true`か`complete false`を出力する |
| [`kotowari mutants --tool cargo-mutants <結果のファイル>`](docs/guides/commands/mutants.ja.md) | 変異テスト（cargo-mutants）の結果から見逃しを報告する |
| [`kotowari plan <計画のファイル>`](docs/guides/commands/plan.ja.md) | 同梱のスキーマに従って、実装計画のファイルが決まった形式で書かれているかを検査する |
| `kotowari overview build` | 設定の`overview.files`に当たる全体像の元データと`overview.toc`の指す目次を検査し、誤りが無ければ`.kotowari/cache/overview/`の下に、目次の順と入れ子で並べた一覧と全体像ごとのHTMLのページを書く。変わったファイルだけを書き、もう作らないページは消す。設定に`overview`を記述した場合のみ使える |
| `kotowari overview serve [--port <PORT>]` | buildと同じページを書いてから`http://127.0.0.1:<PORT>/`（既定4590）で配り、Ctrl-Cまで続ける |

現在、テストの印を読み取れるのはRustのみ。ただし、決定の文書の検査はほかの言語を使うプロジェクトでも利用できる。

## LLMと一緒に使う

LLMと一緒に使うためのClaude Code用スキルを`agent/skills/`に用意している。

- `kotowari`は、決定の文書の書き方、指摘の直し方、印の置き方をLLMに教える
- `kotowari-*`は、kotowariを使って壁打ちから実装、レビュー、マージまでの工程を進めるためのスキルで、利用は任意

インストール方法は[agent/skills/README.md](agent/skills/README.md)に記載している。

## 同梱ツール

`crates/kotowari-mds`には、Markdown文書をYAMLのスキーマで検査する汎用CLIツール`kotowari-mds`がある。
kotowariはこの仕組みで決定の文書を読み取っているが、`kotowari-mds`は単独でも利用できる。詳しくは[README](crates/kotowari-markdown-schema/README.md)を参照。

## ライセンス

[MIT](LICENSE-MIT)または[Apache-2.0](LICENSE-APACHE)のいずれかを選んで利用できる。
