# 次のリリースの束の整合（2回目）：kotowari の IrDocument が指す型

## Context

[次のリリースの束](./2026-10-23-next-release-bundle.md)の整合のフェーズの2回目で、[1回目](./2026-10-24-next-release-bundle-consistency.md)の直しを含めた差分を IR と読み合わせた。
[A6 (next-release-bundle)](./2026-10-23-next-release-bundle.md#A6) は "IrDocument" を kotowari の結果から届く型の再公開として残すと決め、[REQ-core-309](../../ir/core/library-crates.md#REQ-core-309) は入口の入力と結果から届かない下位の型を kotowari が再公開しないと書く。
しかし "kotowari::IrDocument" が再公開していたのは kotowari_core の根の別名 "IrDocument"（"crates/kotowari-core/src/lib.rs" 50行目、"ir::parse" が返す "ParsedIrDocument"）で、"ReadModel::documents"（"crates/kotowari-core/src/inputs/read.rs" 236行目）が返す "kotowari_core::ir::IrDocument" とは同じ名前の別の型だった。Project のどの操作も ParsedIrDocument を受け取らず返さない。
1回目の記録の Context と [A2 (consistency)](./2026-10-24-next-release-bundle-consistency.md#A2) の根拠にある「kotowari は ParsedIrDocument も ir::parse も再公開していない」は、別名の下で ParsedIrDocument を再公開していたので正確でない。`rg -n 'ParsedIrDocument|ir::parse' crates/kotowari/src` が何も出さなかったのは別名が名前を隠していたためで、A2 の結論（ParsedItem は入口から届かないので再公開をやめる）は変わらない。

Position: 整合のフェーズで決めた（2026-10-24）。

## Agreements

- A1 "kotowari::IrDocument" の再公開先を "kotowari_core::ir::IrDocument"（"ReadModel::documents" が返す型）にする。kotowari_core の根の別名 "IrDocument"（ParsedIrDocument）はそのまま残す
  - why: [REQ-core-309](../../ir/core/library-crates.md#REQ-core-309) は入口の入力と結果から届かない下位の型の再公開を禁じ、[A6 (next-release-bundle)](./2026-10-23-next-release-bundle.md#A6) は IrDocument を結果から届く型として残すと決めた。両方を満たす型は ReadModel::documents が返す ir::IrDocument だけである。ParsedIrDocument に届く "ir::parse" は Project のどの操作も出さず、その入力の IrOptions と SourceText の再公開は [A12 (next-release-bundle)](./2026-10-23-next-release-bundle.md#A12) でやめている。kotowari_core の根の別名は [REQ-core-314](../../ir/core/library-inputs.md#REQ-core-314) の「"ir::parse" は "IrDocument" を作る」と合っていて IR と食い違わず、変えるのは [A8 (next-release-bundle)](./2026-10-23-next-release-bundle.md#A8) が限った範囲の外になる
  - decided_by: 整合のフェーズ（根拠: 直す前に "crates/kotowari/tests/project.rs" の facade の同一性のテストへ `fn same_documents(model: &kotowari::ReadModel) -> &[kotowari::IrDocument] { model.documents() }` を足して `cargo test -p kotowari --test project` を走らせると、E0308 "expected `ParsedIrDocument`, found `kotowari_core::ir::IrDocument`" で compile に失敗した。再公開先を変えたあと同じテストが通り、`cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`、`cargo test --workspace --all-features --locked`、`cargo run -p kotowari --example identity`、`kotowari check` が通る。`rg -n '\bIrDocument\b' crates/kotowari src tests crates/kotowari-overview` で kotowari::IrDocument を使う場所は再公開の1行だけで、kotowari-overview は kotowari_core::ir::IrDocument を直接使う。変更履歴の Changed に BREAKING として書いた）

## Rejected

- A2 "kotowari::IrDocument" の再公開を消す
  - why: A6 が結果から届く型として残すと決めた名前で、ReadModel::documents が返す型は実際に結果から届く。消すのは A6 を誤りだと示さないまま覆すことになる
- A3 kotowari_core の根の別名 "IrDocument"（ParsedIrDocument）を消すか ir::IrDocument に向け直す
  - why: REQ-core-314 は ir::parse が "IrDocument" を作ると書いていて、根の別名はそれと合っている。変えると kotowari-core の利用者の型が変わる別の破壊的変更になり、A8 が「同じ場所で同じ理由が当たるもの」に限った範囲の外になる。直したあとも "kotowari::IrDocument" と "kotowari_core::IrDocument" が別の型である紛らわしさは残り、kotowari-core の名前の整理として次の破壊的なリリースで判断する
- A4 ir::IrDocument から届く "Item" と "DocKind" も kotowari から再公開する
  - why: REQ-core-309 は届かない型の再公開を禁じるだけで、届く型を全部再公開するとは書いていない。A8 は保留の候補と同じ場所のものに限り、[A34 (next-release-bundle)](./2026-10-23-next-release-bundle.md#A34) は全体の棚卸しを退けた
