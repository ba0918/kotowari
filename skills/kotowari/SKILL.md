---
name: kotowari
description: "kotowari の IR の書き方、check の使い方、印の置き方、ワークフローの手順を場面ごとに読む。発火語: kotowari、IR、docs/ir、@kotowari、印、mutants、変異テスト"
---

kotowari は、正規化した仕様（IR）を Markdown で書き、`kotowari check` で検査するための道具。変異テストの結果を読む `kotowari mutants` も持つ。このスキルは場面ごとに reference を読ませて、IR の書き方、検査の結果の読み方、印の置き方、ワークフローの手順を伝える。

最初に `kotowari --version` を走らせ、道具があることを確かめる。

- コマンドが見つからない → kotowari の導入を求め、作業を始めない
- 出力から版が読めない → 版を確認できない旨を人に言い、作業を始めない

対象の版は固定しない。references は本体の仕様に追従して改訂し、先頭に改訂日を持つ。

場面の選び方: 人が名指ししたらそれに従う。なければ文脈から選ぶ。

| 場面 | いつ | 読む reference |
|---|---|---|
| setup | 置き場が無い、初めて使う | config.md |
| write | brainstorm の途中で IR を書く | ir-form.md と workflow.md の brainstorm の節 |
| check | `kotowari check` の結果を読む | findings.md |
| mark | テストを書くときに印を置く | mark.md |
| mutants | `kotowari mutants` の結果を読む、見逃しを調べる | mutants.md |
| workflow | plan、cycle、implement の途中 | workflow.md の自分の席の節 |

write の承認の前に collate.md も読む。

場面とワークフローの対応:

- setup — プロジェクトの最初に1回
- write — brainstorm の席で IR を書くとき
- check — brainstorm（承認前）と cycle（終端報告前）と implement（確認コマンド）
- mark — implement と fixer がテストを書くとき
- mutants — push が変異の見逃しで止まったとき、cycle の後始末で見逃しを調べるとき
- workflow — plan、cycle、implement の各席がワークフローの手順を確かめるとき
