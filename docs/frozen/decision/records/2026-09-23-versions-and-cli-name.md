# 壁打ちの記録: 2つの製品の版とタグ、CLI の名前、旧リポジトリ

## Context

mds を kotowari のリポジトリに取り込んだあとも、版の持ち方が決まっていなかった。workspace の3つのパッケージ（kotowari、kotowari-core、kotowari-markdown-schema）はそれぞれ `Cargo.toml` に "0.1.0" を持ち、タグは無い。mds は取り込む前のリポジトリ（GitHub の ba0918/mds）で 0.1.0 を `v0.1.0` のタグでリリース済みである。

あわせて、置き換えの記録で未決にしていた CLI の名前（[U1（ir-engine）](./2026-09-22-ir-engine.md#Undecided)）と、取り込む前のリポジトリの扱いを決める。"mds" の名前は crates.io で他者が公開済みである。

Position: 承認済み（2026-09-23）。

## Agreements

- A1 kotowari と mds は別々の版を持つ。タグは製品の名前を前に付けて `kotowari-v0.2.0`、`kotowari-mds-v0.2.0` の形にする。版の置き場はそれぞれの `Cargo.toml` の1か所
  - why: mds は kotowari に縛られない汎用のエンジンとして使う方針なので、kotowari だけが変わったときに mds の版が上がると、mds の利用者にとって版が意味を持たない
  - rejected: workspace 全体で版を1つにし、タグを `v0.2.0` の1本にする。単純だが mds の版が kotowari の変更で動く
  - decided_by: 利用者

- A2 kotowari-core は外に出さない部品として、版を kotowari に合わせる
  - why: kotowari の本体だけが使う部品で、単独でリリースしない
  - decided_by: 利用者（A1 の推奨の一部として採用）
  - superseded_by: [A48: coreも公開APIを管理する公開準備へ変更](./2026-10-03-public-crate-api.md#A48)

- A3 mds の版は取り込む前のリポジトリの 0.1.0 から続ける
  - why: 0.1.0 は既にリリースしてタグを打ってある。取り込みで版を振り直すと、同じ版の番号が別の中身を指す
  - decided_by: 利用者（A1 の推奨の一部として採用）

- A4 mds の CLI のバイナリの名前を "kotowari-mds" にする。製品の略称として文章で "mds" と書くことと、基準のディレクトリの ".mds/" の名前は変えない
  - why: "mds" は crates.io で他者が公開済みで、配布するときにぶつかる。ディレクトリの名前は CLI の名前と別の関心で、変えると既存のスキーマの置き場が動く
  - rejected: kotowari のサブコマンド "kotowari mds" にする。mds の CLI を使うのは kotowari を使わない人（kotowari は mds をライブラリとして呼ぶ）で、IR の道具ごと入れさせることになり、A1 で分けた版も CLI の上では kotowari の版になる
  - decided_by: 利用者（U1）

- A5 取り込む前のリポジトリ（ba0918/mds）は凍結し、GitHub でアーカイブする。README に移転先は書かない
  - why: 利用者は作者だけで、案内を読む相手がいない。ひっそり閉じればよい
  - rejected: README の先頭に移転先を書いてからアーカイブする
  - decided_by: 利用者
