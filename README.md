# kotowari

仕様を Markdown で決まった形に書き、その形と、仕様・判断・テストのつながりを機械で検査する CLI。

LLM に仕様を書かせて実装させると、次のようなずれが起きやすい。

- 仕様が根拠のない決定を含む
- 要求にテストが無い
- テストがどの要求を確かめているのか分からない

kotowari は仕様を「IR」と呼ぶ正規化した Markdown に書かせ、次の3点を `kotowari check` で確かめる。

- 各要求が、どの決定（判断の記録）から来たかを持っているか
- その決定が本当にあるか
- どのテストがその要求を確かめているか

使い手として第一に想定しているのは LLM で、出力は既定で JSON。人が読むときは `--format text` を使う。

## 入れ方

版は `kotowari-v0.1.0` の形のタグで出している。次の3通りのどれかで入れる。例の `0.1.0` は入れたい版に置き換える。

### ビルド済みのバイナリ

[GitHub Release](https://github.com/ba0918/kotowari/releases) に、Linux x86_64 と macOS arm64 のバイナリを置いている。ファイルの名前は `kotowari-v<版>-<ターゲット>.tar.gz` で、ターゲットは Linux が `x86_64-unknown-linux-gnu`、macOS が `aarch64-apple-darwin`。中身は `kotowari` のバイナリと README とライセンスで、それぞれに SHA256 の `.sha256` を付けている。

```console
$ curl -LO https://github.com/ba0918/kotowari/releases/download/kotowari-v0.1.0/kotowari-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
$ curl -LO https://github.com/ba0918/kotowari/releases/download/kotowari-v0.1.0/kotowari-v0.1.0-x86_64-unknown-linux-gnu.tar.gz.sha256
$ shasum -a 256 -c kotowari-v0.1.0-x86_64-unknown-linux-gnu.tar.gz.sha256
$ tar -xzf kotowari-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
$ install kotowari-v0.1.0-x86_64-unknown-linux-gnu/kotowari ~/.local/bin/
$ kotowari --version
```

置き先の `~/.local/bin/` は `PATH` の通った好きな場所に置き換える。

### mise

[mise](https://mise.jdx.dev/) の `github:` で、GitHub Release のバイナリを入れられる。タグの頭の `kotowari-v` を `version_prefix` で外すと、版を `0.1.0` の形で書ける。

```console
$ mise use -g 'github:ba0918/kotowari[version_prefix=kotowari-v]@0.1.0'
$ kotowari --version
```

### ソースからビルドする

Rust のツールチェーンがあれば、タグで版を固定して `cargo install` で入れられる。手元でビルドするので時間がかかる。

```console
$ cargo install --git https://github.com/ba0918/kotowari --tag kotowari-v0.1.0 kotowari
$ kotowari --version
```

## 最小の例

置き場の既定は次のとおり。

```
.kotowari/config.yaml        設定（空でも無くてもよい。既定の値で動く）
docs/ir/                     IR（仕様）
docs/ir/CONTEXT.md           用語集
docs/decision/records/       判断の記録
docs/decision/adr/           ADR（空でよい）
```

要求の文に曖昧語が含まれると指摘が出る。曖昧語の既定は「適切に」「必要に応じて」「通常は」「など」の4語で、`.kotowari/config.yaml` の `vague_words` で上書きできる。

まず判断の記録に、決めたことを1行1決定で書く（`docs/decision/records/2026-01-01-login.md`）。

```markdown
# 壁打ちの記録: ログイン

## Context

ログインの失敗をどう扱うかを決める。

## Agreements

- A1 パスワードを5回続けて間違えたアカウントは15分ロックする
  - why: 総当たりを遅らせる
  - decided_by: 利用者
```

次に、その決定を出典にした要求を IR に書く（`docs/ir/login.md`）。

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

最後に、要求を確かめるテストに印 `@kotowari[ID]` を付ける。

```rust
// @kotowari[REQ-001]
#[test]
fn req_001_locks_after_five_failures() { /* ... */ }
```

これで `kotowari check` を走らせると、何も出さずに終了コード0で終わる。印を外すと、次の2件が出る。

```console
$ kotowari check --format text
docs/ir/login.md:7 [error] requirement_without_test REQ-001
src/lib.rs:2 [error] test_without_id req_001_locks_after_five_failures
```

出典の決定番号を消したり書き間違えたりしても、同じように指摘が出る。

## コマンド

| コマンド | 役目 |
|---|---|
| `kotowari check` | IR の形、出典の実在、テストの印を検査する。終了コードは 0 が指摘なし、1 が誤りあり、2 が検査に入れなかった |
| `kotowari list` | IR の項目と、印の付いたテストの一覧 |
| `kotowari query <ID>` | 1件の本文、テスト、逆引き |
| `kotowari status` | 揃っているかの集計。最後の行が `complete true` か `complete false` |
| `kotowari mutants --tool cargo-mutants <結果のファイル>` | 変異テスト（cargo-mutants）の結果から見逃しを報告する |
| `kotowari plan <計画のファイル>` | 実装の計画のファイルの形を、kotowari に同梱のスキーマで検査する |
| `kotowari changes --base <REV> (--head <REV> \| --staged) --phase <implementation\|review>` | Git の比較元と対象の差分を照合記録と突き合わせ、照合の漏れと古さを報告する。設定に `changes` を書いたときだけ使える（[使い方](docs/guides/commands/changes.md)） |

テストの印を読めるのは今のところ Rust のテストだけ。ほかの言語でも、IR の側の検査は使える。

## LLM と一緒に使う

`agent/skills/` に Claude Code の skill がある。

- `kotowari`：IR と判断の記録の書き方、指摘の直し方、印の置き方を LLM に教える
- `kotowari-*`：壁打ちから計画、実装、レビューまでの工程を kotowari の上で回す（任意）

入れ方は [agent/skills/README.md](agent/skills/README.md) を読む。

## 同梱のもの

`crates/kotowari-markdown-schema` の `kotowari-mds` は、Markdown の文書を YAML のスキーマで検査する汎用の CLI。kotowari はこれを使って IR を読んでいる。kotowari とは別に単独でも使える（[README](crates/kotowari-markdown-schema/README.md)）。

## ライセンス

MIT か Apache-2.0 のどちらか（[LICENSE-MIT](LICENSE-MIT)、[LICENSE-APACHE](LICENSE-APACHE)）。
