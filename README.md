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

```console
$ cargo install --git https://github.com/ba0918/kotowari kotowari
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

## 要求

### REQ-001: 連続失敗でロックする

- 種類: event_driven
- 出典: docs/decision/records/2026-01-01-login.md#A1
- 検証: unit

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
src/lib.rs:5 [error] test_without_id req_001_locks_after_five_failures
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

テストの印を読めるのは今のところ Rust のテストだけ。ほかの言語でも、IR の側の検査は使える。

## LLM と一緒に使う

`skills/` に Claude Code の skill がある。

- `kotowari`：IR と判断の記録の書き方、指摘の直し方、印の置き方を LLM に教える
- `kotowari-*`：壁打ちから計画、実装、レビューまでの工程を kotowari の上で回す（任意）

入れ方は [skills/README.md](skills/README.md) を読む。

## 同梱のもの

`crates/kotowari-markdown-schema` の `kotowari-mds` は、Markdown の文書を YAML のスキーマで検査する汎用の CLI。kotowari はこれを使って IR を読んでいる。kotowari とは別に単独でも使える（[README](crates/kotowari-markdown-schema/README.md)）。

## ライセンス

MIT か Apache-2.0 のどちらか（[LICENSE-MIT](LICENSE-MIT)、[LICENSE-APACHE](LICENSE-APACHE)）。
