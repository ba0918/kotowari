#!/usr/bin/env bash
# 準備 → tracked な両役の照合 → 検証済み候補の確定。push は利用者が行う。
# 操作仕様: docs/release/change-conformance.md
set -Eeuo pipefail
trap 'printf "release.sh: 操作に失敗した。保存内容は保持する\n" >&2; exit 1' ERR
cd "$(dirname "$0")/.."
readonly REPO_URL=https://github.com/ba0918/kotowari
readonly VERSION_PATTERN='^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?$'

die() { printf 'release.sh: %s\n' "$1" >&2; exit 1; }
usage() { printf 'usage: %s [prepare | finalize | status | abort] <kotowari | kotowari-mds> <版>\n' "$0" >&2; exit 2; }
if [ "$#" -eq 2 ]; then
    operation=prepare; product="$1"; version="$2"
elif [ "$#" -eq 3 ]; then
    operation="$1"; product="$2"; version="$3"
else
    usage
fi
case "$operation" in prepare | finalize | status | abort) ;; *) usage ;; esac
[[ "$version" =~ $VERSION_PATTERN ]] || usage
readonly tag="${product}-v${version}"
readonly commit_message="chore: ${product} ${version} をリリースする"
# shellcheck source=scripts/release-generate.sh
source scripts/release-generate.sh
select_product
# shellcheck source=scripts/release-state.sh
source scripts/release-state.sh

resume_preparation() {
    local path image side mode blob
    require_main
    [ "$(git rev-parse HEAD)" = "$start_head" ] || die "準備の開始 HEAD が進んだ"
    [ "$(git write-tree)" = "$index_tree" ] || die "準備中に index が変わった"
    [ -z "$(git ls-files --others --exclude-standard)" ] || die "準備中に生成対象外の追加がある"
    while IFS= read -r -d '' path; do
        is_generated "$path" || die "準備中に生成対象外の変更がある: $path"
    done < <(git diff --no-renames --name-only -z HEAD --)
    for path in "${generated[@]}"; do
        image="$(worktree_image "$path")"
        [ "$image" = "$(saved_image "$path" before)" ] || [ "$image" = "$(saved_image "$path" after)" ] || die "準備中の生成ファイルに別の変更がある: $path"
        for side in before after; do
            read -r mode blob <<< "$(saved_image "$path" "$side")"
            git cat-file -e "$blob^{blob}" || die "保存内容を読めない: $path"
        done
    done
    # Every boundary was checked before the first write; no rollback deletes records.
    for path in "${generated[@]}"; do
        read -r mode blob <<< "$(saved_image "$path" after)"
        git cat-file blob "$blob" > "$path"
        if [ "$mode" = 100755 ]; then chmod +x "$path"; else chmod -x "$path"; fi
    done
    set_stage prepared
}

prepare() {
    local previous stage path mode before after files history_file
    require_main
    if [ -e "$state_file" ]; then
        # Terminal preparations can belong to another product/version. Validate with
        # those inputs before archiving, never overwrite malformed or unfinished data.
        previous="$(cat "$state_file")"
        stage="$(jq -r .stage <<< "$previous")" || die "準備状態が不正"
        if [ "$stage" = preparing ] || [ "$stage" = prepared ]; then
            load_state
            if [ "$stage" = preparing ]; then resume_preparation; fi
            report_prepared
            return
        fi
        case "$stage" in complete | aborted) ;; *) die "未完了の準備がある。status で確認する" ;; esac
        local next_product="$product" next_version="$version"
        jq -e '.product | IN("kotowari", "kotowari-mds")' <<< "$previous" >/dev/null || die "保存製品が不正"
        product="$(jq -r .product <<< "$previous")"; version="$(jq -r .release_version <<< "$previous")"
        [[ "$version" =~ $VERSION_PATTERN ]] || die "保存版が不正"
        validate_terminal "$previous"
        product="$next_product"; version="$next_version"; select_product
    fi
    require_clean
    ! git rev-parse --quiet --verify "refs/tags/$tag" >/dev/null || die "タグ $tag が既にある"
    require_remote_absent
    [ -f "$changelog" ] && [ ! -L "$changelog" ] || die "変更履歴が無い、または symlink: $changelog"
    grep -q '^## \[Unreleased\]' "$changelog" || die "Unreleased の見出しが無い"
    unreleased_has_entries || die "Unreleased の節が空"
    start_head="$(git rev-parse HEAD)"
    index_tree="$(git write-tree)"
    release_date="$(date +%F)"
    files='[]'
    for path in "${generated[@]}"; do
        # Require regular tracked files with exactly the index's mode and blob.
        read -r mode before <<< "$(worktree_image "$path")"
        [ "$(git ls-tree "$start_head" -- "$path" | awk '{print $1 " " $3}')" = "$mode $before" ] || die "生成元が通常の tracked 内容と一致しない: $path"
    done
    generate_contents
    for path in "${generated[@]}"; do
        read -r mode before <<< "$(worktree_image "$path")"
        # Persist the blobs before state: resume does not depend on temporary output.
        git hash-object -w --no-filters -- "$path" >/dev/null
        after="$(git hash-object -w --no-filters -- "$lock/generated/$path")"
        files="$(jq --arg path "$path" --arg mode "$mode" --arg before "$before" --arg after "$after" '. + [{path:$path,before:{mode:$mode,blob:$before},after:{mode:$mode,blob:$after}}]' <<< "$files")"
    done
    if [ -n "${previous:-}" ]; then
        mkdir -p "$state_dir/history"
        [ ! -L "$state_dir/history" ] || die "履歴のディレクトリが symlink"
        history_file="$(mktemp "$state_dir/history/prepared.XXXXXX.json")" || die "終端状態の履歴を保存できない"
        printf '%s\n' "$previous" > "$history_file" || die "終端状態の履歴を保存できない"
    fi
    state="$(jq -n --arg product "$product" --arg version "$version" --arg tag "$tag" --arg head "$start_head" --arg date "$release_date" --arg index "$index_tree" --argjson files "$files" '{version:1,product:$product,release_version:$version,tag:$tag,start_head:$head,date:$date,files:$files,index_tree:$index,stage:"preparing",planned_tree:null,candidate_commit:null,tag_target:null,tag_object:null}')"
    save_state
    resume_preparation
    report_prepared
}

# Terminal validation shares the complete schema without changing the requested tag.
validate_terminal() {
    select_product
    load_state "${product}-v${version}"
}

report_prepared() {
    printf '準備済み（commit・tag・push はまだ）。製品 %s / 版 %s / タグ %s\n開始 SHA: %s\n' "$product" "$version" "$tag" "$start_head"
    printf '生成ファイル:\n'; printf '  %s\n' "${generated[@]}"
    printf '生成差分を照合し、実装側と独立 review が開始 SHA を base とした新規記録を別々に追加する。\n'
    printf '確定: scripts/release.sh finalize %s %s\n' "$product" "$version"
}

finalize() {
    local path planned candidate
    load_state
    require_main
    case "$stage" in
        preparing) die "準備が中断された。同じ prepare で配置を完了する" ;;
        aborted) die "中止済み。通常変更を完了して clean-tree から再準備する" ;;
        complete)
            verify_candidate_checkout
            verify_local_tag
            report_result
            return ;;
        prepared)
            if [ "$(git rev-parse HEAD)" != "$start_head" ]; then
                recognize_candidate || die "HEAD は今回の予定候補ではない"
            else
                inspect_prepared_boundary
                CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format text || die "新規記録を含む参照検査が落ちた"
                for path in "${generated[@]}" "${records[@]}"; do git add -- "$path"; done
                # Filters or concurrent writes cannot silently change the fixed input.
                inspect_prepared_boundary
                planned="$(git write-tree)"
                state="$(jq --arg tree "$planned" '.planned_tree = $tree' <<< "$state")"
                save_state
                CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- changes --base HEAD --staged --phase implementation --format text || die "実装段階の照合が落ちた"
                git commit -q -m "$commit_message" || die "コミットできなかった。状態と内容を保持する"
                recognize_candidate || die "フック後のコミットは予定 tree・開始親・メッセージと違う。タグは作らない"
            fi ;;
    esac
    verify_candidate_checkout
    candidate="$(jq -r .candidate_commit <<< "$state")"
    scripts/check-versions.sh "$tag" || die "版のずれの検査が落ちた"
    CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format text || die "候補の check が落ちた"
    CARGO_BUILD_JOBS=4 cargo test --workspace || die "候補のテストが落ちた"
    CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- changes --base "$start_head" --head "$candidate" --phase review --format text || die "候補の独立 review 照合が落ちた"
    # No further work between final checkout/remote checks and the tag operation.
    verify_candidate_checkout
    require_remote_absent
    if git rev-parse --quiet --verify "refs/tags/$tag" >/dev/null; then
        verify_local_tag
    else
        git tag -a "$tag" "$candidate" -m "$product $version" || die "タグを作れなかった"
        verify_local_tag
    fi
    state="$(jq --arg target "$candidate" --arg object "$(git rev-parse "refs/tags/$tag")" '.tag_target = $target | .tag_object = $object' <<< "$state")"
    set_stage complete
    report_result
}

abort() {
    local path image before after mode blob index_image
    load_state
    if [ "$stage" = aborted ]; then
        report_aborted
        return
    fi
    # A commit may already have landed while the state still says prepared.
    if [ "$stage" = prepared ] && [ "$(git rev-parse HEAD)" != "$start_head" ]; then
        recognize_candidate || die "HEAD は今回の候補と一致しない。生成内容は戻さない"
    fi
    case "$stage" in
        candidate | complete)
            set_stage aborted
            report_aborted
            return ;;
    esac
    require_main
    [ "$(git rev-parse HEAD)" = "$start_head" ] || die "開始 HEAD が進んだ。生成内容は戻さない"
    require_no_conflicts
    for path in "${generated[@]}"; do
        image="$(worktree_image "$path")"; before="$(saved_image "$path" before)"; after="$(saved_image "$path" after)"
        [ "$image" = "$before" ] || [ "$image" = "$after" ] || die "生成対象に別の変更がある。何も戻さない: $path"
        index_image="$(git ls-files --stage -- "$path" | awk '{print $1 " " $2}')"
        [ "$index_image" = "$before" ] || [ "$index_image" = "$after" ] || die "生成対象の index に別の変更がある: $path"
        read -r mode blob <<< "$before"
        git cat-file -e "$blob^{blob}" || die "変更前の保存内容を読めない: $path"
    done
    # All generated files passed before any restore. Retain every new record.
    for path in "${generated[@]}"; do
        read -r mode blob <<< "$(saved_image "$path" before)"
        git cat-file blob "$blob" > "$path"
        if [ "$mode" = 100755 ]; then chmod +x "$path"; else chmod -x "$path"; fi
        git update-index --cacheinfo "$mode,$blob,$path"
    done
    set_stage aborted
    report_aborted
}

report_aborted() {
    printf '操作を中止済み。保存候補: %s / 現在 HEAD: %s\n' "$(jq -r .candidate_commit <<< "$state")" "$(git rev-parse HEAD)"
    printf 'Git の合格・確定を意味しない。履歴・タグ・照合記録は保持する。\n'
}

status() {
    load_state
    printf '%s\n' "$state"
    printf '現在 HEAD: %s\n現在 tree: %s\n現在 index tree: %s\n' "$(git rev-parse HEAD)" "$(git rev-parse 'HEAD^{tree}')" "$(git write-tree)"
    printf '開始 HEAD 一致: %s\n候補 HEAD 一致: %s\n予定 tree 一致: %s\n' \
        "$([ "$(git rev-parse HEAD)" = "$start_head" ] && printf true || printf false)" \
        "$([ "$(git rev-parse HEAD)" = "$(jq -r .candidate_commit <<< "$state")" ] && printf true || printf false)" \
        "$([ "$(git rev-parse 'HEAD^{tree}')" = "$(jq -r .planned_tree <<< "$state")" ] && printf true || printf false)"
    printf '作業ツリーの変更:\n'; git status --short
}

if [ "$operation" != status ]; then acquire_lock; fi
"$operation"
