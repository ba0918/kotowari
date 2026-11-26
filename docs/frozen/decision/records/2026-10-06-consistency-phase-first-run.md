# 変更照合を外したブランチの整合のフェーズの判断

## Context

変更照合（kotowari changes と照合記録）を外し、cycle に整合のフェーズを足したブランチ consistency-phase（比較元 d18bdae）の差分を、[REQ-core-359](../../ir/core/consistency-phase.md#REQ-core-359) から [REQ-core-363](../../ir/core/consistency-phase.md#REQ-core-363) と、計画が扱う変更済みの項目と読み合わせた。スキルの文章は製品のコードとして読んだ。IR とコードは、次の2件を除いて食い違わなかった。

Position: 2件を直してコミットし、自分のコミットを含めて読み直して新しい指摘は無かった。

## Agreements

- A1 整合のフェーズの reference に、cycle の以前の扱い（IR は直し役に任せ、矛盾は人に戻す）をこのフェーズの片付け方が置き換えることを書き足す。IR は変えない。
  - why: [REQ-core-361](../../ir/core/consistency-phase.md#REQ-core-361) の how_to_verify は reference を読んでこの置き換えが書いてあることを確かめるとするが、`rg -n -i "fixer|former|replace|person" agent/skills/kotowari-cycle/references/consistency-phase.md` は置き換えを述べる行を返さなかった（置き換えは cycle の SKILL.md の「Cycle never fixes the IR itself and never hands it to the implementer or the fixer」にだけあった）。IR に誤りは示されていないので、コード（スキルの文章）を IR に合わせる
  - decided_by: 整合のフェーズ
- A2 tests/step1_cli.rs の REQ-core-001 の節のコメントを「コマンドは8つ」から「コマンドは7つ」に直す。テストの名前 req_001_six_commands_only はこのブランチの前からの名前なので変えない。
  - why: [REQ-core-001](../../ir/core/cli.md#REQ-core-001) は7つのコマンドだけを持つとし、実測でも `kotowari changes` は "argument error: unknown command: changes" で終了コード2、引数なしは "argument error: expected command: check, list, mutants, overview, plan, query or status" で終了コード2になった。コメントだけが外したコマンドを数えていた（tests/step1_cli.rs:18）
  - decided_by: 整合のフェーズ
