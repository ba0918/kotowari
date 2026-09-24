# kotowari ガイド

<!-- @kotowari[REQ-core-001:c859c183] -->

kotowari は、Markdown で書いた仕様（IR）を機械的に検査する CLI です。
要求・決定表・シナリオを決まった形で書き、それぞれが決定の記録に出典を持ち、テストに結び付いているかを `kotowari check` が確かめます。

このガイドは、kotowari を使う開発者のためのリファレンスです。
仕様そのものは [IR](../ir/core/) にあり、ガイドと IR が食い違うときは IR が正です。

## はじめに読む

| やりたいこと | 読むページ |
|---|---|
| コマンドの共通の書式、終了コード、止まったときの読み方を知る | [CLI（全コマンド共通）](cli.md) |
| 置き場やテストのファイルの場所を設定する | [設定ファイル](config.md) |
| 仕様を検査して、指摘を直す | [kotowari check](commands/check.md) と [指摘の種類](findings.md) |
| テストと要求を結び付ける | [テストに印を付ける](marks.md) |

## コマンド

| コマンド | 何をするか |
|---|---|
| [check](commands/check.md) | IR とテストの印を検査し、指摘を出す |
| [list](commands/list.md) | 項目とシナリオを、印の付いたテストと一緒に一覧にする |
| [query](commands/query.md) | 1件の項目の本文と、それを指す項目を出す |
| [status](commands/status.md) | 揃っているかを数と1つの真偽で答える |
| [mutants](commands/mutants.md) | 変異テストの結果を読み、見逃しを指摘にする |
| [plan](commands/plan.md) | 実装計画の形を、同梱のスキーマで検査する |

## リファレンス

- [CLI（全コマンド共通）](cli.md) — 書式、共通のオプション、終了コード、停止の理由
- [設定ファイル](config.md) — `.kotowari/config.yaml` の全キー
- [指摘の種類](findings.md) — 誤りと注意の一覧、detail、直し方
- [テストに印を付ける](marks.md) — `@kotowari[...]` の書き方と、見つかるテスト
- [ガイドを書く](writing-guides.md) — このガイドのような利用者向けの文書に印と指紋を付け、IR の変更に追従させる

## このガイドの保ち方

各節の見出しの下には、その節が説明している IR の項目と指紋が HTML コメントで書かれています（描画したページには出ません）。
IR が変わると `kotowari check` が古くなった節に guide_stale の注意を出すので、その節だけを見直します。
やり方は [ガイドを書く](writing-guides.md) にあります。
