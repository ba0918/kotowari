#!/usr/bin/env bash
#
# CI の変異テストの1つの分担を回す（判断の記録 docs/decision/records/2026-10-04-mutants-in-ci.md の A3）。
# mutants-run.yml の各分担がこれを呼び、範囲の形に応じて scripts/mutants.sh の副コマンドを選ぶ。
#
#   mutants-shard.sh <範囲> <分担の番号> <分担の数>
#     <範囲> は "diff <基準の commit か参照>" か "full"
#
# 終了コード: scripts/mutants.sh の終了コード。範囲の形が違えば 2

set -euo pipefail

cd "$(dirname "$0")/.."

if [ "$#" -ne 3 ]; then
    echo "usage: mutants-shard.sh <scope> <shard> <shards>" >&2
    exit 2
fi

read -r mode base <<<"$1"
shard="$2/$3"
case "$mode" in
diff) exec scripts/mutants.sh diff "$base" -- --shard "$shard" ;;
full) exec scripts/mutants.sh full -- --shard "$shard" ;;
*)
    echo "unknown scope: $1" >&2
    exit 2
    ;;
esac
