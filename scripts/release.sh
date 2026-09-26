#!/usr/bin/env bash
#
# 製品の版を上げ、変更履歴を昇格し、チェックを通ったらコミットと注釈付きのタグを作る
# （判断の記録 docs/decision/records/2026-09-26-release-flow.md の A4、A6、A9）。
#
#   release.sh <製品> <版>     製品は kotowari か kotowari-mds。例: release.sh kotowari 0.1.0
#
# 書き換えるのは、製品の版の宣言（Cargo.toml と Cargo.lock）と、変更履歴の Unreleased の節を
# 版の見出しにすることと、比較のリンクである。書き換えてからチェックを回し、通れば1つの
# コミットとタグにする。落ちれば書き換えを戻して止まり、コミットもタグも残さない。
# push はしない。公開は人が打つ push で行う。
#
# 終了コード: 0 タグを作った、1 止まった（何も残さない）、2 引数の形が違う

set -euo pipefail

cd "$(dirname "$0")/.."

readonly REPO_URL="https://github.com/ba0918/kotowari"
readonly VERSION_PATTERN='^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?$'

die() {
    printf 'release.sh: %s\n' "$1" >&2
    exit 1
}

usage() {
    printf 'usage: %s <kotowari | kotowari-mds> <版>\n' "$0" >&2
    exit 2
}

[ "$#" -eq 2 ] || usage
product="$1"
version="$2"
[[ "$version" =~ $VERSION_PATTERN ]] || usage

# 製品ごとの、版の宣言（先頭が置き場、残りは従う宣言）、Cargo.lock のパッケージ、変更履歴
case "$product" in
    kotowari)
        manifests=(Cargo.toml crates/kotowari-core/Cargo.toml)
        lock_packages=(kotowari kotowari-core)
        changelog="CHANGELOG.md"
        ;;
    kotowari-mds)
        manifests=(crates/kotowari-markdown-schema/Cargo.toml)
        lock_packages=(kotowari-markdown-schema)
        changelog="crates/kotowari-markdown-schema/CHANGELOG.md"
        ;;
    *)
        usage
        ;;
esac
readonly tag="${product}-v${version}"

# --- 何も変えずに止まる場合 ---

branch="$(git symbolic-ref --quiet --short HEAD || true)"
[ "$branch" = "main" ] || die "main の上で走らせる（今は ${branch:-detached HEAD}）"
[ -z "$(git status --porcelain)" ] || die "作業ツリーに変更がある。コミットするか片付けてから走らせる"
if git rev-parse --quiet --verify "refs/tags/$tag" > /dev/null; then
    die "タグ $tag が既にある"
fi
[ -f "$changelog" ] || die "変更履歴 $changelog が無い"

# Unreleased の節（見出しの次の行から、次の ## の見出しかリンクの定義の手前まで）に、
# 空行と ### の小見出しのほかの行があるか
unreleased_has_entries() {
    awk '
        /^## \[Unreleased\]/ { inside = 1; next }
        inside && (/^## / || /^\[[^]]+\]: /) { exit }
        inside && !/^[[:space:]]*$/ && !/^### / { found = 1; exit }
        END { exit !found }
    ' "$1"
}
grep -q '^## \[Unreleased\]' "$changelog" || die "$changelog に ## [Unreleased] の見出しが無い"
unreleased_has_entries "$changelog" || die "$changelog の Unreleased の節が空"

# --- 書き換え。ここから先で止まったら、書き換えたファイルを戻す ---

written=("${manifests[@]}" Cargo.lock "$changelog")
commit_made=0
done_ok=0
restore() {
    if [ "$done_ok" -eq 1 ]; then
        return
    fi
    if [ "$commit_made" -eq 1 ]; then
        git reset -q --soft HEAD~1
    fi
    git restore --staged --worktree -- "${written[@]}"
    printf 'release.sh: 書き換えを戻した。コミットもタグも作っていない\n' >&2
}
trap restore EXIT

# $1 のファイルを、標準入力の内容で置き換える（ファイルの権限はそのまま）
replace_with_stdin() {
    local tmp
    tmp="$(mktemp)"
    cat > "$tmp"
    cat "$tmp" > "$1"
    rm -f "$tmp"
}

for manifest in "${manifests[@]}"; do
    awk -v v="$version" '
        /^\[/ { in_package = ($0 == "[package]") }
        in_package && !changed && /^version[[:space:]]*=/ { print "version = \"" v "\""; changed = 1; next }
        { print }
    ' "$manifest" | replace_with_stdin "$manifest"
done

for package in "${lock_packages[@]}"; do
    awk -v name="$package" -v v="$version" '
        /^\[\[package\]\]$/ { in_block = 0 }
        $0 == "name = \"" name "\"" { in_block = 1 }
        in_block && /^version = / { print "version = \"" v "\""; in_block = 0; next }
        { print }
    ' Cargo.lock | replace_with_stdin Cargo.lock
done

# 前のタグ（この製品の、版の順でいちばん新しいもの）。無ければ最初のリリース
previous_tag="$(git tag --list "${product}-v*" --sort=-v:refname | head -n 1)"
if [ -n "$previous_tag" ]; then
    version_link="${REPO_URL}/compare/${previous_tag}...${tag}"
else
    version_link="${REPO_URL}/releases/tag/${tag}"
fi
release_date="$(date +%F)"

# Unreleased の見出しの下に版の見出しを入れ、リンクの定義を並べ直す。
# Unreleased のリンクは新しいタグからの比較に替え、版のリンクをその下に置く
awk -v heading="## [${version}] - ${release_date}" \
    -v unreleased_link="[Unreleased]: ${REPO_URL}/compare/${tag}...HEAD" \
    -v version_link="[${version}]: ${version_link}" '
    /^## \[Unreleased\]/ { print; print ""; print heading; next }
    /^\[Unreleased\]: / { next }
    !linked && /^\[[^]]+\]: / { print unreleased_link; print version_link; linked = 1 }
    { print }
    END { if (!linked) { print ""; print unreleased_link; print version_link } }
' "$changelog" | replace_with_stdin "$changelog"

# --- チェック。軽いものから回す ---

printf 'release.sh: 版のずれを検査する\n' >&2
scripts/check-versions.sh "$tag" || die "版のずれの検査が落ちた"
printf 'release.sh: kotowari check を回す\n' >&2
CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format text || die "kotowari check が終了コード0で終わらなかった"
printf 'release.sh: テスト全件を回す\n' >&2
CARGO_BUILD_JOBS=4 cargo test --workspace || die "テストが落ちた"

# --- コミットとタグ ---

git add -- "${written[@]}"
git commit -q -m "chore: ${product} ${version} をリリースする" || die "コミットできなかった"
commit_made=1
git tag -a "$tag" -m "${product} ${version}" || die "タグを作れなかった"
done_ok=1

cat <<EOF

${tag} を作った（まだ push していない）。公開するには次を打つ:

  git push --atomic origin main ${tag}

タグの push では pre-push のフックが変異テストを全体で回すので時間がかかる。
通れば GitHub Actions がバイナリを付けた GitHub Release を作る。

pre-push で push が拒まれたら、タグもコミットもまだ公開されていないので、次で戻してから直す:

  git tag -d ${tag}
  git reset --keep HEAD~1

直しをコミットしてから、もう一度 scripts/release.sh ${product} ${version} を走らせる。
EOF
