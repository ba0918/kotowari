#!/usr/bin/env bash
#
# 配布するバイナリに入る依存のライセンス文と著作権表示を、1つのテキストに書き出す。
# MIT、ISC、BSD-3-Clause などは、複製にライセンス文と著作権表示を添えることを求める。
# cargo-about が要る。設定は scripts/third-party/about.toml、形は scripts/third-party/about.hbs。
# Cargo.lock は書き換えない（--locked）。
#
#   third-party-licenses.sh <パッケージ> <ターゲット> <出力先>
#
# 書き出したあと、そのパッケージとターゲットでバイナリに入るクレート（cargo tree の通常の依存）が
# すべて一覧に載っているかを確かめる。載っていないクレートがあれば、名前を出して落ちる。
#
# 終了コード: 0 書き出した、1 一覧に載っていないクレートがある、2 引数の数が違う、
#             それ以外 cargo か cargo-about が落ちた

set -euo pipefail

if [ "$#" -ne 3 ]; then
    echo "usage: third-party-licenses.sh <package> <target> <output>" >&2
    exit 2
fi

package="$1"
target="$2"
case "$3" in
/*) output="$3" ;;
*) output="$PWD/$3" ;;
esac

cd "$(dirname "$0")/.."

# cargo tree の1行は "<名前> v<版>"、ワークスペースのパッケージは後ろに " (<ディレクトリ>)" が付く。
# 手続きマクロの " (proc-macro)" と、2回目に出た印の " (*)" は落とす
linked="$(cargo tree --locked -p "$package" -e normal --target "$target" --prefix none --format '{p}' |
    sed -e 's/ (\*)$//' -e 's/ (proc-macro)$//' | sort -u)"

# cargo-about は、渡したマニフェストのパッケージを依存の起点にする。根のマニフェストのままだと
# kotowari-cli の依存だけを見るので、パッケージのマニフェストを渡す
directory="$(printf '%s\n' "$linked" | sed -n "s/^$package v[^ ]* (\(.*\))\$/\1/p")"
if [ -z "$directory" ]; then
    echo "third-party-licenses.sh: $package はワークスペースのパッケージではない" >&2
    exit 2
fi

cargo about generate --locked --fail \
    --config scripts/third-party/about.toml \
    --manifest-path "$directory/Cargo.toml" \
    --target "$target" \
    --output-file "$output" \
    scripts/third-party/about.hbs

missing="$(printf '%s\n' "$linked" | sed -e 's/ (.*)$//' -e 's/ v/ /' | while IFS= read -r crate; do
    grep -qF -- "- $crate (" "$output" || printf '%s\n' "$crate"
done)"
if [ -n "$missing" ]; then
    echo "third-party-licenses.sh: 一覧に載っていないクレートがある（$package、$target）:" >&2
    printf '%s\n' "$missing" >&2
    exit 1
fi
