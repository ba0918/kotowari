# kotowari ガイド

[English](index.md) | 日本語

<!-- @kotowari[docs/decisions/core/cli.md#kotowari-has-seven-commands-and-the-single-command-check-does-both-the-check-of-the-specification-and-the-check-of-the-correspondence-with-tests:d77a0bd7, docs/decisions/core/decision-documents.md#the-specification-lives-only-in-decision-documents-one-per-topic-holding-the-decisions-now-in-force:d2ea663b] -->

kotowari は、Markdown の決定の文書で書いた仕様を機械的に検査する CLI です。
話題ごとに今有効な決定を、それが守られていることを示す結果と一緒に書き、文書が決まった形かと、その結果のそれぞれにテストが結び付いているかを `kotowari check` が確かめます。

このガイドは、kotowari を使う開発者のためのリファレンスです。
仕様そのものは [決定の文書](../decisions/core/) にあり、ガイドと決定の文書が食い違うときは決定の文書が正です。

## はじめに読む

| やりたいこと | 読むページ |
|---|---|
| コマンドの共通の書式、終了コード、止まったときの読み方を知る | [CLI（全コマンド共通）](cli.ja.md) |
| Rustから型付きAPIを呼ぶ | [公開クレートAPI](public-crate-api.ja.md) |
| 独立した配布アーカイブを検証する | [パッケージ検証](package-validation.ja.md) |
| 決定の文書やテストのファイルの場所を設定する | [設定ファイル](config.ja.md) |
| 仕様を決定の文書で書く | [決定の文書を書く](decision-documents.ja.md) |
| 仕様を検査して、指摘を直す | [kotowari check](commands/check.ja.md) と [指摘の種類](findings.ja.md) |
| テストと確認することの行を結び付ける | [テストに印を付ける](marks.ja.md) |
| 仕様に書かないまま作った機能をコードから見つける | [面の検査](surface.ja.md) |

## コマンド

| コマンド | 何をするか |
|---|---|
| [check](commands/check.ja.md) | 決定の文書とテストの印を検査し、指摘を出す |
| [list](commands/list.ja.md) | 決定と確認することの行を、印の付いたテストと一緒に一覧で出す |
| [query](commands/query.ja.md) | 1つの決定か行の本文と、その決定へリンクしている決定を出す |
| [status](commands/status.ja.md) | 揃っているかを、数と1つの真偽で答える |
| [mutants](commands/mutants.ja.md) | 変異テストの結果を読み、見逃しを指摘にする |
| [plan](commands/plan.ja.md) | 実装計画の形を、同梱のスキーマで検査する |
| [overview](../decisions/core/overview-commands.ja.md) | 全体像の元データから全体像のページを `build` で書き、`serve` で書いて手元で配る |

## リファレンス

- [CLI（全コマンド共通）](cli.ja.md) — 書式、共通のオプション、終了コード、停止の理由
- [設定ファイル](config.ja.md) — `.kotowari/config.yaml` の全キー
- [決定の文書を書く](decision-documents.ja.md) — 話題の文書の形、状態、確認することの行と ID、未決
- [指摘の種類](findings.ja.md) — 誤りと注意の一覧、detail、直し方
- [テストに印を付ける](marks.ja.md) — `@kotowari[...]` の書き方と、見つかるテスト
- [面の検査](surface.ja.md) — 面の規則の書き方、仕様にあるとする場所、未記載の面の一覧
- [ガイドを書く](writing-guides.ja.md) — このガイドのような利用者向けの文書に印と指紋を付け、決定の変更に追従させる

## このガイドの保ち方

<!-- @kotowari[docs/decisions/core/languages-and-guides.md#a-guide-is-stale-when-the-decision-or-confirmations-change:de5a25a2] -->

各節の見出しの下には、その節が説明している決定と指紋が HTML コメントで書かれています（描画したページには出ません）。
そのどれかの決めたことか確認することが変わると、`kotowari check` が古くなった節に guide_stale の注意を出すので、その節だけを見直します。
やり方は [ガイドを書く](writing-guides.ja.md) にあります。
