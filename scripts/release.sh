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

update_versions() {
    python3 - "$1" "$2" "$3" <<'PY'
from pathlib import Path
import re, sys, tomllib
root, family, version = Path(sys.argv[1]), sys.argv[2], sys.argv[3]
selected = {'kotowari-cli', 'kotowari', 'kotowari-core', 'kotowari-source-analysis', 'kotowari-overview'} if family == 'kotowari' else {'kotowari-markdown-schema', 'kotowari-markdown-schema-io', 'kotowari-mds', 'kotowari-markdown-view'}
for path in [root / 'Cargo.toml', *sorted((root / 'crates').glob('*/Cargo.toml'))]:
    text = path.read_text()
    manifest = tomllib.loads(text)
    if manifest['package']['name'] in selected:
        text = re.sub(r'(?ms)(^\[package\]\n(?:(?!^\[).)*?^version\s*=\s*)"[^"]+"', lambda match: match[1] + '"' + version + '"', text, count=1)
    for name in selected:
        text = re.sub(r'(?m)^(\s*' + re.escape(name) + r'\s*=\s*\{[^\n]*?\bversion\s*=\s*)"[^"]+"', lambda match: match[1] + '"' + version + '"', text)
    path.write_text(text)
path = root / 'Cargo.lock'
text = path.read_text()
for name in selected:
    text = re.sub(r'(?m)(^name = "' + re.escape(name) + r'"\nversion = )"[^"]+"', lambda match: match[1] + '"' + version + '"', text)
path.write_text(text)
PY
}

if [[ "${BASH_SOURCE[0]}" != "$0" ]]; then
    return
fi

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
        manifests=(Cargo.toml crates/kotowari/Cargo.toml crates/kotowari-core/Cargo.toml crates/kotowari-source-analysis/Cargo.toml crates/kotowari-overview/Cargo.toml)
        lock_packages=(kotowari-cli kotowari kotowari-core kotowari-source-analysis kotowari-overview)
        changelog="CHANGELOG.md"
        ;;
    kotowari-mds)
        manifests=(crates/kotowari-markdown-schema/Cargo.toml crates/kotowari-markdown-schema-io/Cargo.toml crates/kotowari-mds/Cargo.toml crates/kotowari-markdown-view/Cargo.toml)
        lock_packages=(kotowari-markdown-schema kotowari-markdown-schema-io kotowari-mds kotowari-markdown-view)
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
# origin にあるタグは公開済みなので、その版は使い直さない。ls-remote の --exit-code は
# 見つかれば 0、見つからなければ 2 で終わり、それ以外は origin に届かなかったことを表す
remote_status=0
git ls-remote --exit-code --tags origin "refs/tags/$tag" > /dev/null || remote_status=$?
case "$remote_status" in
    0) die "タグ $tag は origin に既にある（公開済み）。この版は使い直さず、新しい版でリリースする" ;;
    2) ;;
    *) die "origin に届かず、タグ $tag が公開済みかを確かめられない" ;;
esac
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

# タグが無いことは上で確かめたので、戻すときにあるタグはこのスクリプトが作ったもの。
# コミットとタグは、作った直後に中断されても戻せるように、フラグでなく今の状態で見る
written=(Cargo.toml crates/kotowari/Cargo.toml crates/kotowari-core/Cargo.toml crates/kotowari-source-analysis/Cargo.toml crates/kotowari-overview/Cargo.toml crates/kotowari-markdown-schema/Cargo.toml crates/kotowari-markdown-schema-io/Cargo.toml crates/kotowari-mds/Cargo.toml crates/kotowari-markdown-view/Cargo.toml Cargo.lock "$changelog")
start_head="$(git rev-parse HEAD)"
done_ok=0
restore() {
    if [ "$done_ok" -eq 1 ]; then
        return
    fi
    if git rev-parse --quiet --verify "refs/tags/$tag" > /dev/null; then
        git tag -d "$tag" > /dev/null
    fi
    if [ "$(git rev-parse HEAD)" != "$start_head" ]; then
        git reset -q --soft "$start_head"
    fi
    git restore --staged --worktree -- "${written[@]}"
    printf 'release.sh: 書き換えを戻した。コミットもタグも残していない\n' >&2
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

update_versions . "$product" "$version"

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
CARGO_BUILD_JOBS=4 cargo run -q -p kotowari-cli --bin kotowari -- check --format text || die "kotowari check が終了コード0で終わらなかった"
printf 'release.sh: テスト全件を回す\n' >&2
CARGO_BUILD_JOBS=4 cargo test --workspace || die "テストが落ちた"

# --- コミットとタグ ---

git add -- "${written[@]}"
git commit -q -m "chore: ${product} ${version} をリリースする" || die "コミットできなかった"
git tag -a "$tag" -m "${product} ${version}" || die "タグを作れなかった"
done_ok=1

cat <<EOF

${tag} を作った（まだ push していない）。main は必須のチェックが通った commit しか受け付けないので、
まずリリースの commit をブランチに push して PR を作り、チェックを通す:

  git push --no-follow-tags origin HEAD:refs/heads/release/${tag}
  gh pr create --base main --head release/${tag} --fill

PR のチェックが通ったら、同じ commit の main とタグを push する:

  git push --no-follow-tags origin main && git push origin ${tag}

main の push が通ってからタグを push する（main が拒まれたらタグは出さない）。
タグの push で GitHub Actions が前のリリースのタグとの差分の変異テストを回し（前のタグが無ければ全体）、
通ればバイナリを付けた GitHub Release を作る。

push が拒まれたら、まず同名のタグが origin にあるかを確かめる:

  git ls-remote --tags origin ${tag}

何も出なければ、タグもコミットもまだ公開されていない。次で戻し、直しをコミットしてから、
もう一度 scripts/release.sh ${product} ${version} を走らせる:

  git tag -d ${tag}
  git reset --keep HEAD~1

出たら、${version} は既に公開されている。その版は使い直さない（origin のタグを消したり
作り直したりしない）。手元のタグとコミットは公開されたものとは別物なので上と同じ2つで捨て、
origin を取り込んでから、新しい版で scripts/release.sh ${product} <新しい版> を走らせる。
EOF
