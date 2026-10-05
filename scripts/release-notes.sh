#!/usr/bin/env bash
#
# 変更履歴から1つの版の節を切り出して、リリースノートのファイルに書く（判断の記録
# docs/decision/records/2026-09-26-release-flow.md の A8）。
#
#   release-notes.sh <版> <変更履歴> <書き出す先>
#
# 終了コード: 0 書いた、1 変更履歴が無いか、その版の節が無いか空、2 引数の数が違う

set -euo pipefail

if [ "$#" -ne 3 ]; then
    echo "usage: release-notes.sh <version> <changelog> <output>" >&2
    exit 2
fi

version="$1"
changelog="$2"
output="$3"

if [ ! -f "$changelog" ]; then
    echo "::error::変更履歴 $changelog が無い"
    exit 1
fi
awk -v version="$version" '
  index($0, "## [" version "]") == 1 { inside = 1; next }
  inside && (/^## / || /^\[[^]]+\]: /) { exit }
  inside { print }
' "$changelog" >"$output"
if ! grep -q '[^[:space:]]' "$output"; then
    echo "::error::$changelog に $version の節が無いか、空"
    exit 1
fi
