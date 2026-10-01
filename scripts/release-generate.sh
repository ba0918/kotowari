# shellcheck shell=bash
# Globals are supplied to and consumed by the sourcing release.sh.
# shellcheck disable=SC2034,SC2154
# Sourced by release.sh; generate fixed bytes before changing any worktree file.
# Existing package/lock/changelog promotion rules are retained.
select_product() {
    case "$product" in
        kotowari)
            manifests=(Cargo.toml crates/kotowari-core/Cargo.toml)
            lock_packages=(kotowari kotowari-core)
            changelog=CHANGELOG.md ;;
        kotowari-mds)
            manifests=(crates/kotowari-markdown-schema/Cargo.toml)
            lock_packages=(kotowari-markdown-schema)
            changelog=crates/kotowari-markdown-schema/CHANGELOG.md ;;
        *) usage ;;
    esac
    generated=("${manifests[@]}" Cargo.lock "$changelog")
}

unreleased_has_entries() {
    awk '
        /^## \[Unreleased\]/ { inside = 1; next }
        inside && (/^## / || /^\[[^]]+\]: /) { exit }
        inside && !/^[[:space:]]*$/ && !/^### / { found = 1; exit }
        END { exit !found }
    ' "$changelog"
}

generate_contents() {
    local manifest package previous_tag version_link
    mkdir "$lock/generated"
    for manifest in "${manifests[@]}"; do
        mkdir -p "$lock/generated/$(dirname "$manifest")"
        awk -v v="$version" '
            /^\[/ { in_package = ($0 == "[package]") }
            in_package && !changed && /^version[[:space:]]*=/ { print "version = \"" v "\""; changed = 1; next }
            { print }
        ' "$manifest" > "$lock/generated/$manifest"
    done
    cp Cargo.lock "$lock/generated/Cargo.lock"
    for package in "${lock_packages[@]}"; do
        awk -v name="$package" -v v="$version" '
            /^\[\[package\]\]$/ { in_block = 0 }
            $0 == "name = \"" name "\"" { in_block = 1 }
            in_block && /^version = / { print "version = \"" v "\""; in_block = 0; next }
            { print }
        ' "$lock/generated/Cargo.lock" > "$lock/generated/next.lock"
        mv "$lock/generated/next.lock" "$lock/generated/Cargo.lock"
    done
    previous_tag="$(git tag --list "${product}-v*" --sort=-v:refname | sed -n '1p')"
    if [ -n "$previous_tag" ]; then
        version_link="${REPO_URL}/compare/${previous_tag}...${tag}"
    else
        version_link="${REPO_URL}/releases/tag/${tag}"
    fi
    mkdir -p "$lock/generated/$(dirname "$changelog")"
    awk -v heading="## [${version}] - ${release_date}" \
        -v unreleased_link="[Unreleased]: ${REPO_URL}/compare/${tag}...HEAD" \
        -v version_link="[${version}]: ${version_link}" '
        /^## \[Unreleased\]/ { print; print ""; print heading; next }
        /^\[Unreleased\]: / { next }
        !linked && /^\[[^]]+\]: / { print unreleased_link; print version_link; linked = 1 }
        { print }
        END { if (!linked) { print ""; print unreleased_link; print version_link } }
    ' "$changelog" > "$lock/generated/$changelog"
}
