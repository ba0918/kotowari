#!/usr/bin/env bash
#
# ビルドしたバイナリを、README とライセンスと、依存のライセンス文の一覧と一緒に、配布用の tar.gz と
# sha256 に固める。dist/ の下に書く。
# 依存のライセンス文の一覧は scripts/third-party-licenses.sh が作るので、cargo-about が要る。
#
#   package-release.sh <製品> <版> <ターゲット> <パッケージ> <バイナリの名前> <README>
#
# 終了コード: 0 固めた、2 引数の数が違う、それ以外 コピーか一覧の生成か固めるのに失敗した

set -euo pipefail

if [ "$#" -ne 6 ]; then
    echo "usage: package-release.sh <product> <version> <target> <package> <bin> <readme>" >&2
    exit 2
fi

product="$1"
version="$2"
target="$3"
package="$4"
bin="$5"
readme="$6"

name="${product}-v${version}-${target}"
mkdir -p "dist/$name"
cp "target/$target/release/$bin" "dist/$name/"
cp "$readme" "dist/$name/README.md"
cp LICENSE-MIT LICENSE-APACHE "dist/$name/"
"$(dirname "$0")/third-party-licenses.sh" "$package" "$target" "dist/$name/THIRD-PARTY-LICENSES.txt"
cd dist
tar -czf "$name.tar.gz" "$name"
shasum -a 256 "$name.tar.gz" >"$name.tar.gz.sha256"
