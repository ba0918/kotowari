# kotowari ガイド

<!-- @kotowari[REQ-core-001:67e34470] -->

kotowari は、Markdown で書いた仕様（IR）を機械的に検査する CLI です。
要求・決定表・シナリオを決まった形で書き、それぞれが決定の記録に出典を持ち、テストに結び付いているかを `kotowari check` が確かめます。

このガイドは、kotowari を使う開発者のためのリファレンスです。
仕様そのものは [IR](../ir/core/) にあり、ガイドと IR が食い違うときは IR が正です。

## はじめに読む

| やりたいこと | 読むページ |
|---|---|
| コマンドの共通の書式、終了コード、止まったときの読み方を知る | [CLI（全コマンド共通）](cli.md) |
| Rustから型付きAPIを呼ぶ | [公開クレートAPI](public-crate-api.md) |
| 独立した配布アーカイブを検証する | [パッケージ検証](package-validation.md) |
| 置き場やテストのファイルの場所を設定する | [設定ファイル](config.md) |
| 仕様を検査して、指摘を直す | [kotowari check](commands/check.md) と [指摘の種類](findings.md) |
| テストと要求を結び付ける | [テストに印を付ける](marks.md) |
| 今は作らない要求をテストの検査から外す | [要求を後回しにする](deferred.md) |
| IR に書かないまま作った機能をコードから見つける | [面の検査](surface.md) |

## コマンド

| コマンド | 何をするか |
|---|---|
| [changes](commands/changes.md) | 変更の照合漏れと鮮度を Git snapshot で検査する |
| [check](commands/check.md) | IR とテストの印を検査し、指摘を出す |
| [list](commands/list.md) | 項目とシナリオを、印の付いたテストと一緒に一覧にする |
| [query](commands/query.md) | 1件の項目の本文と、それを指す項目を出す |
| [status](commands/status.md) | 揃っているかを数と1つの真偽で答える |
| [mutants](commands/mutants.md) | 変異テストの結果を読み、見逃しを指摘にする |
| [plan](commands/plan.md) | 実装計画の形を、同梱のスキーマで検査する |
| [overview](../ir/core/overview-commands.md) | 全体像の元データから全体像のページを `build` で書き、`serve` で書いて手元で配る |

## リファレンス

- [CLI（全コマンド共通）](cli.md) — 書式、共通のオプション、終了コード、停止の理由
- [設定ファイル](config.md) — `.kotowari/config.yaml` の全キー
- [指摘の種類](findings.md) — 誤りと注意の一覧、detail、直し方
- [テストに印を付ける](marks.md) — `@kotowari[...]` の書き方と、見つかるテスト
- [要求を後回しにする](deferred.md) — `- deferred:` の書き方、効き目、食い違いの注意
- [面の検査](surface.md) — 面の規則の書き方、IR にあるとする場所、未記載の面の一覧
- [ガイドを書く](writing-guides.md) — このガイドのような利用者向けの文書に印と指紋を付け、IR の変更に追従させる

## このガイドの保ち方

各節の見出しの下には、その節が説明している IR の項目と指紋が HTML コメントで書かれています（描画したページには出ません）。
IR が変わると `kotowari check` が古くなった節に guide_stale の注意を出すので、その節だけを見直します。
やり方は [ガイドを書く](writing-guides.md) にあります。
