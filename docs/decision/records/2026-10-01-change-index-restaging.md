# 整形後の index を変更照合へ渡す

## Context

変更照合の途中検査は、整形したコミット対象の index を読む必要がある。
既存の rustfmt フックは stage_fixed を使って整形結果をステージし直す。
Lefthook 2.1.14 の一時 Git リポジトリで、後続コマンドの実行時点ではこの更新がまだ反映されないことを確認した。
承認済みの「整形後の index を検査する」を、フックの免除なしで実現する方法を選ぶ。

## Agreements

- A1 rustfmt のコマンド内で整形対象のパスを明示して git add し、その後に変更照合を行う。フックは並列実行をせず、整形・秘密情報・check・changes の優先度を明示する。
  - why: stage_fixed だけの一時リポジトリでは整形後の記録に change_stale が出て終了1となった。rustfmt の直後に対象パスを git add する同じ検証では changes の files=1、covered=1 と終了0を確認した。優先度を逆の名前で指定した試行でも実行順は指定順となった。これにより [REQ-core-276](../../ir/core/change-workflow.md#REQ-core-276) の承認済み対象と段階を保つ。
  - decided_by: 実装役
  - rejected: stage_fixed による最後の再ステージだけに依存する構成。後続の changes が整形前の index を読むため。

## Rejected

- R1 changes の検査を免除する、または作業ツリーを index の補完として読む。
  - why: [REQ-core-265](../../ir/core/changes-inputs.md#REQ-core-265) と [REQ-core-276](../../ir/core/change-workflow.md#REQ-core-276) の承認済み境界に反する。
