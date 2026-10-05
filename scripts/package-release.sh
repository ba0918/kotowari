#!/usr/bin/env bash
#
# ビルドしたバイナリを、README とライセンスと一緒に配布用の tar.gz と sha256 に固める
# （判断の記録 docs/decision/records/2026-09-26-release-flow.md の A7）。dist/ の下に書く。
#
#   package-release.sh <製品> <版> <ターゲット> <バイナリの名前> <README>
#
# 終了コード: 0 固めた、2 引数の数が違う、それ以外 コピーか固めるのに失敗した

set -euo pipefail

if [ "$#" -ne 5 ]; then
    echo "usage: package-release.sh <product> <version> <target> <bin> <readme>" >&2
    exit 2
fi

product="$1"
version="$2"
target="$3"
bin="$4"
readme="$5"

name="${product}-v${version}-${target}"
mkdir -p "dist/$name"
cp "target/$target/release/$bin" "dist/$name/"
cp "$readme" "dist/$name/README.md"
cp LICENSE-MIT LICENSE-APACHE "dist/$name/"
cd dist
tar -czf "$name.tar.gz" "$name"
shasum -a 256 "$name.tar.gz" >"$name.tar.gz.sha256"
