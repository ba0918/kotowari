# 次のリリースの束の整合：入力群の表の言語と、入口から届かない ParsedItem の再公開

## Context

[次のリリースの束](./2026-10-23-next-release-bundle.md)の整合のフェーズで、差分の変えたコードと IR を読み合わせた。
[TBL-core-042](../../ir/core/library-inputs.md#TBL-core-042) は、テスト情報と面の解析結果の内容に「言語」を挙げていたが、[A10 (next-release-bundle)](./2026-10-23-next-release-bundle.md#A10) で "TestAnalysis" と "SurfaceAnalysis" の language の欄を消したので、表と型が食い違っていた。
[REQ-core-309](../../ir/core/library-crates.md#REQ-core-309) は、kotowari が入口の入力と結果から届かない下位の型を再公開しないと書くが、"kotowari::ParsedItem" の再公開は残っていた。ParsedItem に届くのは "kotowari_core::ir::parse" が返す "ParsedIrDocument" の items だけで、kotowari はどちらも再公開していない。

Position: 整合のフェーズで決めた（2026-10-24）。

## Agreements

- A1 [TBL-core-042](../../ir/core/library-inputs.md#TBL-core-042) のテスト情報と面の解析結果の内容から「言語」を外し、「その言語の問い合わせの有無」だけを残す。英語と日本語の両側を直し、コードは変えない
  - why: 内容に言語を挙げたのは [A49 (public-crate-api)](./2026-10-03-public-crate-api.md#A49) の「既存集計に必要な言語・問い合わせの有無」だが、[A10 (next-release-bundle)](./2026-10-23-next-release-bundle.md#A10) が、core も kotowari も CLI も読まないことを理由に language の欄を消した。問い合わせの有無は has_query が運び、拡張子ごとの集計もそれで足りる。A49 の残り（SourceText、発見結果、解析時の指摘）はそのまま成り立つ
  - decided_by: 整合のフェーズ（根拠: "crates/kotowari-core/src/inputs/types.rs" の TestAnalysis（118行目）と SurfaceAnalysis（127行目）に language の欄が無く、`rg -n 'pub language' crates/kotowari-core/src` は何も出さない。`cargo test --workspace` は通る）
- A2 kotowari から "ParsedItem" の再公開をやめる。ParsedItem は "kotowari_core::ir::parse" から取る
  - why: [REQ-core-309](../../ir/core/library-crates.md#REQ-core-309) は入口の入力と結果から届かない下位の型を再公開しないと決め、[A12 (next-release-bundle)](./2026-10-23-next-release-bundle.md#A12) は同じ理由で IrOptions など6つの再公開をやめた。ParsedItem は ir::parse の結果の項目で、入力側の IrOptions を外したのと同じ場所で同じ理由が当たる（[A8 (next-release-bundle)](./2026-10-23-next-release-bundle.md#A8) の範囲）。Project のどの操作も ParsedItem を受け取らず返さず、ReadModel の documents が返すのは IrDocument である
  - decided_by: 整合のフェーズ（根拠: `rg -n '\bParsedItem\b' crates/kotowari src tests crates/kotowari-overview` が "crates/kotowari/src/lib.rs" の再公開の1行だけを出し、`rg -n 'ParsedIrDocument|ir::parse' crates/kotowari/src` は何も出さない。ParsedItem を返すのは "crates/kotowari-core/src/ir/partial.rs" の ParsedIrDocument::items（47行目）だけで、それを返す parse（59行目）は core の入口。やめたあと `cargo clippy --workspace --all-targets --all-features -- -D warnings`、`cargo test --workspace`、`cargo run -p kotowari --example identity` が通る。変更履歴の Removed の項に ParsedItem を足した）

## Rejected

- A3 TBL-core-042 に「言語」を残し、TestAnalysis と SurfaceAnalysis に language の欄を戻す
  - why: A10 で消すと決めた欄を戻すことになり、誰も読まない欄を外の解析器が埋め続ける
- A4 "kotowari::ParsedItem" の再公開を次の破壊的なリリースまで残す
  - why: REQ-core-309 の禁止に反したまま 0.6.0 を出すことになり、同じ理由の IrOptions だけ外して結果側の ParsedItem が残る。A8 は同じ場所で同じ理由が当たるものを範囲に含める
