#!/usr/bin/env bash
#
# 変異テストを走らせ、結果を kotowari mutants に読ませる。
#
#   mutants.sh diff <base> [-- <cargo-mutants の引数>...]  <base> との差分に入る変異だけを走らせる
#   mutants.sh full [-- <cargo-mutants の引数>...]          全体を走らせる
#   mutants.sh plan                                          git の pre-push の行を標準入力から読み、
#                                                            選ぶ副コマンドを1行出す（何も走らせない）
#   mutants.sh hook                                          plan と同じ判定をして、そのまま実行する
#
# 走らせ方は 2026-09-17 の実測で固めた（判断の記録 docs/decision/records/2026-09-17-mutation-tests.md
# の A4、A23、A29）。安定版の cargo だと更新時刻の判定で変異が再コンパイルされず、幻の見逃しが出る
# ので nightly と内容ハッシュの判定を使う。メモリを食い尽くす変異があるので上限つきのサービスで走らせる。

set -euo pipefail

# cargo-mutants が結果を書く先（-o . で直下の mutants.out/ にできる）
readonly RESULTS="mutants.out/outcomes.json"
# 打ち切られたテストが起こした居残りの子プロセスと見なすまでの秒数。
# cargo-mutants はテストを20秒で打ち切るので、これより長く生きているものは居残りしか無い
readonly ORPHAN_SECONDS=60
# 見張りが様子を見る間隔
readonly WATCH_INTERVAL=10

watchdog_pid=""
kill_count_file=""
diff_file=""

die() {
    printf 'mutants.sh: %s\n' "$1" >&2
    exit 1
}

# 60秒以上生きている変異済みの実行ファイル。
# pgrep のパターンを "^/tmp/cargo-mutants-" で始めて固定する（固定しないと見張り自身の bash に当たる）
mutated_processes() {
    pgrep -f '^/tmp/cargo-mutants-' 2>/dev/null | while read -r pid; do
        ps -o pid=,etimes= -p "$pid" 2>/dev/null
    done
}

watchdog_loop() {
    local pid etimes killed
    while true; do
        sleep "$WATCH_INTERVAL"
        killed="$(cat "$kill_count_file" 2>/dev/null || printf '0')"
        while read -r pid etimes; do
            [ -n "${pid:-}" ] || continue
            if [ "$etimes" -ge "$ORPHAN_SECONDS" ]; then
                if kill -9 "$pid" 2>/dev/null; then
                    killed=$((killed + 1))
                fi
            fi
        done < <(mutated_processes)
        printf '%s\n' "$killed" >"$kill_count_file"
    done
}

start_watchdog() {
    kill_count_file="$(mktemp)"
    printf '0\n' >"$kill_count_file"
    watchdog_loop &
    watchdog_pid=$!
}

stop_watchdog() {
    if [ -n "$watchdog_pid" ]; then
        kill "$watchdog_pid" 2>/dev/null || true
        wait "$watchdog_pid" 2>/dev/null || true
        watchdog_pid=""
    fi
}

cleanup() {
    stop_watchdog
    if [ -n "$kill_count_file" ] && [ -f "$kill_count_file" ]; then
        printf 'mutants.sh: killed %s leftover mutated processes\n' \
            "$(cat "$kill_count_file")" >&2
        rm -f -- "$kill_count_file"
        kill_count_file=""
    fi
    if [ -n "$diff_file" ] && [ -f "$diff_file" ]; then
        rm -f -- "$diff_file"
        diff_file=""
    fi
}

trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

# A29: 関門の抜け道を塞ぐ。ソースの #[mutants::skip] はその変異を測らせない
guard_skip_attribute() {
    if grep -rn 'mutants::skip' src/ >/dev/null 2>&1; then
        grep -rn 'mutants::skip' src/ >&2 || true
        die 'found mutants::skip in src/; the judgement belongs in the equivalents list'
    fi
}

# 変異テストを走らせ、結果を kotowari mutants に読ませる
run_mutants() {
    guard_skip_attribute
    # A23: 結果のファイルは毎回その場で作る（古い結果を読ませる余地を無くす）
    rm -rf -- mutants.out

    start_watchdog
    local status=0
    systemd-run --user --wait --collect --pipe \
        -p MemoryMax=12G -p MemorySwapMax=0 -p OOMPolicy=continue \
        --setenv=PATH="$PATH" \
        --setenv=HOME="$HOME" \
        --setenv=CARGO_UNSTABLE_CHECKSUM_FRESHNESS=true \
        --setenv=CARGO_BUILD_JOBS=4 \
        --working-directory="$PWD" \
        -- cargo +nightly mutants -j 1 --no-config -o . "$@" || status=$?
    stop_watchdog

    if [ ! -f "$RESULTS" ]; then
        # 差分に Rust のソースが無いとき、cargo-mutants は結果のファイルを作らずに0で終わる
        if [ "$status" -eq 0 ]; then
            return 0
        fi
        die "the mutation testing tool failed (exit $status) and wrote no result file"
    fi

    # cargo-mutants 自身の終了コードは見逃しや時間切れで0以外になるので使わない
    CARGO_BUILD_JOBS=4 cargo run -q -- mutants --tool cargo-mutants --format text "$RESULTS"
}

run_diff() {
    local base="${1:-}"
    [ -n "$base" ] || die 'usage: mutants.sh diff <base> [-- <cargo-mutants args>...]'
    shift
    if [ "${1:-}" = "--" ]; then
        shift
    fi
    # フックの中では fetch しない。解決できない基準はその場で失敗させる
    git rev-parse --verify --quiet "${base}^{commit}" >/dev/null \
        || die "cannot resolve the base: $base"

    diff_file="$(mktemp)"
    git diff "${base}...HEAD" >"$diff_file"
    run_mutants --in-diff "$diff_file" "$@"
}

run_full() {
    if [ "${1:-}" = "--" ]; then
        shift
    fi
    run_mutants "$@"
}

# git の pre-push は "<ローカルの参照> <ローカルの SHA> <リモートの参照> <リモートの SHA>" を
# 標準入力に渡す。A26: タグの push なら全体、それ以外は差分だけ
choose_from_stdin() {
    local local_ref local_sha remote_ref remote_sha chosen
    chosen='diff origin/main'
    while read -r local_ref local_sha remote_ref remote_sha; do
        case "${local_ref}${remote_ref}" in
        *refs/tags/*) chosen='full' ;;
        esac
    done
    printf '%s\n' "$chosen"
}

main() {
    cd "$(git rev-parse --show-toplevel)"
    local mode="${1:-}"
    case "$mode" in
    diff)
        shift
        run_diff "$@"
        ;;
    full)
        shift
        run_full "$@"
        ;;
    plan)
        choose_from_stdin
        ;;
    hook)
        case "$(choose_from_stdin)" in
        full) run_full ;;
        *) run_diff origin/main ;;
        esac
        ;;
    *)
        die 'usage: mutants.sh <diff <base> | full | plan | hook> [-- <cargo-mutants args>...]'
        ;;
    esac
}

main "$@"
