#!/usr/bin/env bash
#
# 版の宣言のずれを検査する（判断の記録 docs/decision/records/2026-09-26-release-flow.md の A5、A13）。
#
#   check-versions.sh          kotowari の版（根の Cargo.toml）に、kotowari 系のパッケージ（kotowari、
#                              kotowari-core、kotowari-source-analysis、kotowari-overview）の Cargo.toml と
#                              Cargo.lock の版が揃っているかを見る。kotowari-mds の版
#                              （crates/kotowari-markdown-schema/Cargo.toml）に、Markdown スキーマ系の
#                              パッケージ（schema-io、mds、kotowari-markdown-view）の版が揃っているかも見る
#   check-versions.sh <タグ>   上に加えて、タグ（kotowari-v<版> か kotowari-mds-v<版>）の版が
#                              その製品の Cargo.toml の版と同じかを見る
#
# 終了コード: 0 揃っている、1 ずれがある（ずれた箇所を出す）、2 引数の形が違う

set -euo pipefail

cd "$(dirname "$0")/.."

readonly TAG_PATTERN='^(kotowari|kotowari-mds)-v([0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?)$'

usage() {
    printf 'usage: %s [kotowari-v<版> | kotowari-mds-v<版>]\n' "$0" >&2
    exit 2
}

# Cargo.toml の [package] の表にある version の値を出す
manifest_version() {
    awk '
        /^\[/ { in_package = ($0 == "[package]"); next }
        in_package && /^version[[:space:]]*=/ {
            sub(/^version[[:space:]]*=[[:space:]]*"/, ""); sub(/".*$/, ""); print; exit
        }
    ' "$1"
}

# Cargo.lock から、名前が $2 のパッケージの version の値を出す
lock_version() {
    awk -v name="$2" '
        /^\[\[package\]\]$/ { in_block = 0; next }
        $0 == "name = \"" name "\"" { in_block = 1; next }
        in_block && /^version = / {
            sub(/^version = "/, ""); sub(/".*$/, ""); print; exit
        }
    ' "$1"
}

if [ "$#" -gt 1 ]; then
    usage
fi

tag="${1:-}"
tag_product=""
tag_version=""
if [ -n "$tag" ]; then
    if [[ ! "$tag" =~ $TAG_PATTERN ]]; then
        printf 'check-versions.sh: タグの形が違う: %s\n' "$tag" >&2
        usage
    fi
    tag_product="${BASH_REMATCH[1]}"
    tag_version="${BASH_REMATCH[2]}"
fi

readonly MDS_MANIFEST="crates/kotowari-markdown-schema/Cargo.toml"

kotowari_version="$(manifest_version Cargo.toml)"
if [ -z "$kotowari_version" ]; then
    printf 'check-versions.sh: Cargo.toml の [package] に version が無い\n' >&2
    exit 1
fi
mds_version="$(manifest_version "$MDS_MANIFEST")"
if [ -z "$mds_version" ]; then
    printf 'check-versions.sh: %s の [package] に version が無い\n' "$MDS_MANIFEST" >&2
    exit 1
fi

# 箇所と読んだ版と従う製品を1行ずつ並べる。製品の版に従う宣言のすべて
declarations="crates/kotowari-core/Cargo.toml [package] version	$(manifest_version crates/kotowari-core/Cargo.toml)	kotowari
crates/kotowari/Cargo.toml [package] version	$(manifest_version crates/kotowari/Cargo.toml)	kotowari
crates/kotowari-source-analysis/Cargo.toml [package] version	$(manifest_version crates/kotowari-source-analysis/Cargo.toml)	kotowari
crates/kotowari-overview/Cargo.toml [package] version	$(manifest_version crates/kotowari-overview/Cargo.toml)	kotowari
Cargo.lock kotowari	$(lock_version Cargo.lock kotowari)	kotowari
Cargo.lock kotowari-cli	$(lock_version Cargo.lock kotowari-cli)	kotowari
Cargo.lock kotowari-source-analysis	$(lock_version Cargo.lock kotowari-source-analysis)	kotowari
Cargo.lock kotowari-core	$(lock_version Cargo.lock kotowari-core)	kotowari
Cargo.lock kotowari-overview	$(lock_version Cargo.lock kotowari-overview)	kotowari
Cargo.lock kotowari-markdown-schema	$(lock_version Cargo.lock kotowari-markdown-schema)	kotowari-mds
crates/kotowari-markdown-schema-io/Cargo.toml [package] version	$(manifest_version crates/kotowari-markdown-schema-io/Cargo.toml)	kotowari-mds
crates/kotowari-mds/Cargo.toml [package] version	$(manifest_version crates/kotowari-mds/Cargo.toml)	kotowari-mds
crates/kotowari-markdown-view/Cargo.toml [package] version	$(manifest_version crates/kotowari-markdown-view/Cargo.toml)	kotowari-mds
Cargo.lock kotowari-markdown-schema-io	$(lock_version Cargo.lock kotowari-markdown-schema-io)	kotowari-mds
Cargo.lock kotowari-mds	$(lock_version Cargo.lock kotowari-mds)	kotowari-mds
Cargo.lock kotowari-markdown-view	$(lock_version Cargo.lock kotowari-markdown-view)	kotowari-mds"

status=0
while IFS=$'\t' read -r place version product; do
    case "$product" in
        kotowari) expected="$kotowari_version" ;;
        kotowari-mds) expected="$mds_version" ;;
    esac
    if [ "$version" != "$expected" ]; then
        printf '%s: %s（%s の版は %s）\n' "$place" "${version:-(無い)}" "$product" "$expected"
        status=1
    fi
done <<< "$declarations"

if [ -n "$tag" ]; then
    case "$tag_product" in
        kotowari) product_manifest="Cargo.toml" ;;
        kotowari-mds) product_manifest="$MDS_MANIFEST" ;;
    esac
    product_version="$(manifest_version "$product_manifest")"
    if [ "$product_version" != "$tag_version" ]; then
        printf '%s [package] version: %s（タグ %s の版は %s）\n' \
            "$product_manifest" "${product_version:-(無い)}" "$tag" "$tag_version"
        status=1
    fi
fi

python3 - <<'PY' || status=1
import pathlib, sys, tomllib
files = [pathlib.Path('Cargo.toml'), *sorted(pathlib.Path('crates').glob('*/Cargo.toml'))]
manifests = [(path, tomllib.loads(path.read_text())) for path in files]
versions = {manifest['package']['name']: manifest['package']['version'] for _, manifest in manifests}
for path, manifest in manifests:
    tables = [manifest.get(key, {}) for key in ['dependencies', 'dev-dependencies', 'build-dependencies']]
    for target in manifest.get('target', {}).values():
        tables.extend(target.get(key, {}) for key in ['dependencies', 'dev-dependencies', 'build-dependencies'])
    for table in tables:
        for key, dependency in table.items():
            if isinstance(dependency, dict) and 'path' in dependency:
                name = dependency.get('package', key)
                if name not in versions or dependency.get('version') != versions[name]:
                    print(f'{path}: registry version differs for {name}', file=sys.stderr)
                    sys.exit(1)
PY
exit "$status"
