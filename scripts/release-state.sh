# shellcheck shell=bash
# Globals are supplied to and consumed by the sourcing release.sh.
# shellcheck disable=SC2034,SC2154
# Sourced by release.sh. JSON is data; no saved value is executed as shell code.
readonly state_dir=.agents/release
readonly state_file="$state_dir/prepared.json"
readonly lock="$state_dir/lock"

require_ignored_state() {
    local path
    for path in .agents "$state_dir"; do
        [ ! -L "$path" ] || die "状態のディレクトリが symlink: $path"
    done
    for path in "$state_file" "$lock/" "$state_dir/history/"; do
        git check-ignore -q -- "$path" || die "状態とロックの保存先を ignore してから開始する: $path"
    done
}

acquire_lock() {
    require_ignored_state
    mkdir -p "$state_dir"
    mkdir "$lock" 2>/dev/null || die "別の操作のロックがある。保持者の停止を確認してからロックだけを解除する"
    trap 'rm -rf -- "$lock"' EXIT
    trap 'exit 130' INT
    trap 'exit 143' TERM
    trap 'exit 129' HUP
}

# Same-directory rename never exposes a partly written state.
save_state() {
    local tmp
    tmp="$(mktemp "$state_dir/state.XXXXXX")" || die "状態の一時保存先を作れない"
    printf '%s\n' "$state" > "$tmp" || die "状態を書き込めない"
    mv -- "$tmp" "$state_file" || die "状態の保存を確定できない"
}

set_stage() {
    state="$(jq --arg stage "$1" '.stage = $stage' <<< "$state")"
    save_state
}

load_state() {
    [ -f "$state_file" ] && [ ! -L "$state_file" ] || die "準備状態が無い、または symlink"
    state="$(cat "$state_file")"
    # Path list is product-specific. Invalid state never broadens the restore boundary.
    local paths expected_tag="${1:-$tag}"
    paths="$(printf '%s\n' "${generated[@]}" | jq -R . | jq -s .)"
    jq -e --arg product "$product" --arg version "$version" --arg tag "$expected_tag" --argjson paths "$paths" '
        def oid: type == "string" and test("^([0-9a-f]{40}|[0-9a-f]{64})$");
        def nullable_oid: . == null or oid;
        .version == 1 and .product == $product and .release_version == $version and .tag == $tag
        and (.start_head | oid) and (.index_tree | oid)
        and (.date | type == "string" and test("^[0-9]{4}-[0-9]{2}-[0-9]{2}$"))
        and (.stage | IN("preparing", "prepared", "candidate", "complete", "aborted"))
        and has("planned_tree") and (.planned_tree | nullable_oid)
        and has("candidate_commit") and (.candidate_commit | nullable_oid)
        and has("tag_target") and (.tag_target | nullable_oid)
        and has("tag_object") and (.tag_object | nullable_oid)
        and (.files | type == "array" and length == ($paths | length))
        and ([.files[].path] == $paths)
        and all(.files[]; all(.before, .after;
             type == "object" and (.mode | IN("100644", "100755")) and (.blob | oid)))
        and (if .stage == "preparing" then .planned_tree == null and .candidate_commit == null
             elif .stage == "prepared" then .candidate_commit == null
             elif .stage == "candidate" or .stage == "complete" then
                  (.planned_tree | oid) and (.candidate_commit | oid)
             else true end)
        and (if .stage == "complete" then (.tag_object | oid) and .tag_target == .candidate_commit else true end)
    ' <<< "$state" >/dev/null || die "状態の版・型・必須情報が不正、または製品・版が一致しない"
    start_head="$(jq -r .start_head <<< "$state")"
    index_tree="$(jq -r .index_tree <<< "$state")"
    stage="$(jq -r .stage <<< "$state")"
}

worktree_image() {
    local path="$1" parent mode
    # Reject directory symlinks as well as a symlink at the leaf.
    parent="$path"
    while [ "$parent" != . ]; do
        [ ! -L "$parent" ] || die "symlink は渡せない: $path"
        parent="$(dirname "$parent")"
    done
    [ -f "$path" ] || die "通常ファイルではない: $path"
    mode=100644
    [ ! -x "$path" ] || mode=100755
    printf '%s %s\n' "$mode" "$(git hash-object --no-filters -- "$path")"
}

saved_image() {
    jq -r --arg path "$1" --arg side "$2" '.files[] | select(.path == $path) | .[$side] | "\(.mode) \(.blob)"' <<< "$state"
}

require_main() {
    [ "$(git symbolic-ref --quiet --short HEAD || true)" = main ] || die "main の上で実行する"
}

require_clean() {
    [ -z "$(git status --porcelain)" ] || die "tracked・untracked を含めて clean-tree が必要"
}

require_remote_absent() {
    local remote_status=0
    git ls-remote --exit-code --tags origin "refs/tags/$tag" >/dev/null || remote_status=$?
    case "$remote_status" in
        0) die "タグ $tag は origin に既にある。公開済みの版は使い直さない" ;;
        2) ;;
        *) die "origin に届かず、タグ $tag の不在を確かめられない" ;;
    esac
}

is_generated() {
    local generated_path
    for generated_path in "${generated[@]}"; do
        [ "$1" != "$generated_path" ] || return 0
    done
    return 1
}

require_no_conflicts() {
    [ -z "$(git ls-files --unmerged)" ] || die "競合した index は渡せない"
}

# Configured records may be added, replaced or deleted; other changes stay generated-only.
inspect_prepared_boundary() {
    local path image staged_mode staged_blob _
    records=()
    require_no_conflicts
    [ "$(git rev-parse HEAD)" = "$start_head" ] || die "準備の開始 HEAD が進んだ"
    for path in "${generated[@]}"; do
        image="$(worktree_image "$path")"
        [ "$image" = "$(saved_image "$path" after)" ] || die "生成内容が変わった: $path"
    done
    while IFS= read -r -d '' path; do
        is_generated "$path" || records+=("$path")
    done < <(git diff --no-renames --name-only -z HEAD --)
    while IFS= read -r -d '' path; do
        if git cat-file -e "$start_head:$path" 2>/dev/null; then
            continue
        fi
        records+=("$path")
    done < <(git ls-files --cached --others --exclude-standard -z)
    CARGO_BUILD_JOBS=4 cargo run -q -p kotowari --example release-record-paths -- "${records[@]}" || die "生成対象と設定された照合記録以外の変更がある"
    for path in "${records[@]}"; do
        if [ -e "$path" ] || [ -L "$path" ]; then
            worktree_image "$path" >/dev/null
        fi
    done
    # Index can be untouched or contain exactly the same fixed worktree content.
    while IFS= read -r -d '' path; do
        if [ -n "$(git ls-files --stage -- "$path")" ]; then
            read -r staged_mode staged_blob _ < <(git ls-files --stage -- "$path")
            [ "$staged_mode $staged_blob" = "$(worktree_image "$path")" ] || die "staged と未 stage の内容が違う: $path"
        else
            [ ! -e "$path" ] && [ ! -L "$path" ] || die "staged 削除と作業内容が違う: $path"
        fi
    done < <(git diff --no-renames --cached --name-only -z HEAD --)
}

recognize_candidate() {
    local head planned parent tree message
    planned="$(jq -r .planned_tree <<< "$state")"
    [ "$planned" != null ] || return 1
    head="$(git rev-parse HEAD)"
    parent="$(git rev-list --parents -n 1 "$head")"
    tree="$(git rev-parse "$head^{tree}")"
    message="$(git log -1 --format=%B "$head")"
    [ "$parent" = "$head $start_head" ] && [ "$tree" = "$planned" ] && [ "$message" = "$commit_message" ] || return 1
    state="$(jq --arg head "$head" '.candidate_commit = $head | .stage = "candidate"' <<< "$state")"
    save_state
    stage=candidate
}

verify_candidate_checkout() {
    require_main
    [ "$(git rev-parse HEAD)" = "$(jq -r .candidate_commit <<< "$state")" ] || die "HEAD が保存候補と違う"
    [ "$(git rev-parse 'HEAD^{tree}')" = "$(jq -r .planned_tree <<< "$state")" ] || die "候補 tree が予定と違う"
    [ "$(git rev-list --parents -n 1 HEAD)" = "$(jq -r .candidate_commit <<< "$state") $start_head" ] || die "候補の親が開始点と違う"
    [ "$(git log -1 --format=%B HEAD)" = "$commit_message" ] || die "候補メッセージが違う"
    require_clean
}

verify_local_tag() {
    if [ "$(jq -r .tag_object <<< "$state")" != null ]; then
        [ "$(git rev-parse "refs/tags/$tag")" = "$(jq -r .tag_object <<< "$state")" ] || die "保存した注釈付きタグ object と違う"
    fi
    [ "$(git cat-file -t "refs/tags/$tag")" = tag ] || die "同名タグが注釈付きでない"
    [ "$(git rev-parse "refs/tags/$tag^{commit}")" = "$(jq -r .candidate_commit <<< "$state")" ] || die "同名タグの対象が保存候補と違う"
    [ "$(git for-each-ref --format='%(contents)' "refs/tags/$tag")" = "$product $version" ] || die "同名タグの注釈が違う"
}

report_result() {
    printf '%s を作った（まだ push していない）。公開は main が通ってからタグを push する:\n\n' "$tag"
    printf '  git push origin main && git push origin %s\n\n' "$tag"
    printf 'pre-push は前の製品タグとの差分の変異テストを回す（前のタグが無ければ全体）。\n'
    printf 'タグの push 後、GitHub Actions が検査と GitHub Release の生成を行う。\n'
    printf 'push 拒否時は git ls-remote --tags origin %s で公開済みか確認する。\n' "$tag"
    printf '未公開の手元履歴の処理は利用者が行い、公開済みの版は使い直さず新しい版で準備する。\n'
}
