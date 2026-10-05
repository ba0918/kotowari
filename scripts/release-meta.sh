#!/usr/bin/env bash
#
# リリースのタグから、配る製品の情報を key=value の行で出す（判断の記録
# docs/decision/records/2026-09-26-release-flow.md の A4、A7）。release.yml の meta のジョブが
# $GITHUB_OUTPUT に足す。
#
#   release-meta.sh <タグ>   タグは <製品>-v<版>（kotowari-v0.1.0、kotowari-mds-v0.2.0）
#
# 出力: product、version、package、bin、changelog、readme の6行
# 終了コード: 0 出した、1 タグの形が違う、2 引数の数が違う

set -euo pipefail

if [ "$#" -ne 1 ]; then
    echo "usage: release-meta.sh <tag>" >&2
    exit 2
fi

tag="$1"
if [[ "$tag" =~ ^(kotowari|kotowari-mds)-v([0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?)$ ]]; then
    product="${BASH_REMATCH[1]}"
    version="${BASH_REMATCH[2]}"
else
    echo "::error::タグの形が違う: $tag"
    exit 1
fi

case "$product" in
kotowari)
    package=kotowari-cli
    bin=kotowari
    changelog=CHANGELOG.md
    readme=README.md
    ;;
kotowari-mds)
    package=kotowari-mds
    bin=kotowari-mds
    changelog=crates/kotowari-markdown-schema/CHANGELOG.md
    readme=crates/kotowari-markdown-schema/README.md
    ;;
esac

printf '%s=%s\n' \
    product "$product" \
    version "$version" \
    package "$package" \
    bin "$bin" \
    changelog "$changelog" \
    readme "$readme"
