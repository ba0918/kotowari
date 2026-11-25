#!/usr/bin/env bash
#
# リリースで配るすべてのバイナリについて、依存のライセンス文の一覧が作れるかを確かめる。
# CI の "Rust gates" と手元の両方の入口。cargo-about が要る。
#
# 一覧はリリースのワークフローがタグの push のあとに作る。そこで初めて落ちると、公開したタグは
# 作り直せない。許す一覧に無いライセンスの依存や、一覧に載らないクレートは、ここで先に落とす。
# パッケージとターゲットは .github/workflows/release.yml と scripts/release-meta.sh が配るものに合わせる。
#
# 終了コード: 0 すべて作れた、それ以外 最初に落ちた scripts/third-party-licenses.sh の終了コード

set -euo pipefail

cd "$(dirname "$0")/.."

out="$(mktemp -d)"
trap 'rm -rf "$out"' EXIT

for package in kotowari-cli kotowari-mds; do
    for target in x86_64-unknown-linux-gnu aarch64-apple-darwin; do
        scripts/third-party-licenses.sh "$package" "$target" "$out/$package-$target.txt"
    done
done
