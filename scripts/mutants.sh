#!/usr/bin/env bash
#
# 変異テストを走らせ、結果を kotowari mutants に読ませる。
#
#   mutants.sh diff <base> [-- <cargo-mutants の引数>...]  <base> との差分に入る変異だけを走らせる
#   mutants.sh full [-- <cargo-mutants の引数>...]          全体を走らせる
#   mutants.sh plan                                          git の pre-push と同じ形の行を標準入力から読み、
#                                                            選ぶ副コマンドを1行出す（何も走らせない。
#                                                            リリースの CI がタグから範囲を決めるのに使う）
#
# 環境変数:
#   MUTANTS_TOOLCHAIN   cargo-mutants を動かす日付つきの nightly（既定 nightly-2026-10-03）
#   MUTANTS_CPU_QUOTA   境界の CPU の上限（既定 400%）。MUTANTS_BUILD_JOBS はその中のビルドの並列数（既定 4）
#   MUTANTS_JOBS     同時に回す変異の数（既定 1。手元で3や4に上げると、写しのビルドが CPU を取り合って
#                    遅くなり、偽の時間切れも出た。判断の記録 docs/decision/records/2026-10-04-mutants-in-ci.md の A6）
#   MUTANTS_SERVICE  systemd（既定。メモリ上限つきのユーザーのサービスで回す）、scope（sudo の
#                    systemd-run でメモリ上限つきのスコープに入れて回す。ユーザーの systemd が無い
#                    CI のランナー用。判断の記録 docs/decision/records/2026-10-04-mutants-in-ci.md の A11）
#                    か none（上限なしでそのまま回す）
#
# 見逃し0件の関門は PR とリリースの CI が持つ（判断の記録
# docs/decision/records/2026-10-04-mutants-in-ci.md の A1〜A6）。手元では好きなときに回す。
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
# 後始末でサービスの停止を確かめる回数と間隔（この積が後始末の待ちの上限）
readonly STOP_TRIES=20
readonly STOP_INTERVAL=0.3
readonly JOBS="${MUTANTS_JOBS:-1}"
readonly SERVICE="${MUTANTS_SERVICE:-systemd}"
# 日付で固定した nightly（判断の記録 docs/decision/records/2026-10-04-mutants-in-ci.md の A16）
readonly TOOLCHAIN="${MUTANTS_TOOLCHAIN:-nightly-2026-10-03}"
# 境界の CPU の上限と、その中のビルドの並列数（同じ記録の A15）
readonly CPU_QUOTA="${MUTANTS_CPU_QUOTA:-400%}"
readonly BUILD_JOBS="${MUTANTS_BUILD_JOBS:-4}"

watchdog_pid=""
kill_count_file=""
diff_file=""
# この実行が作るサービスの名前（この実行だけのもの）と、その終わりを待っている systemd-run
run_unit=""
run_pid=""
# cargo-mutants に作業の写しを作らせる、この実行だけの一時ディレクトリ
run_tmpdir=""

die() {
    printf 'mutants.sh: %s\n' "$1" >&2
    exit 1
}

# この実行の一時ディレクトリの下で60秒以上生きている変異済みの実行ファイル。
# pgrep のパターンをその一時ディレクトリで始めて固定する。先頭を固定しないと見張り自身の bash に
# 当たり、"cargo-mutants-" だけに広げると同時に走っている別の実行の変異済みプロセスまで殺す
mutated_processes() {
    pgrep -f "^${run_tmpdir}/cargo-mutants-" 2>/dev/null | while read -r pid; do
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

# サービスがもう走っていないか。active、activating、reloading、deactivating は「まだ」。
# 未登録のときの is-active は inactive か unknown を出して0以外で終わるので、それも「止まった」に入る
service_is_stopped() {
    local state
    state="$(systemctl --user is-active "$1" 2>/dev/null || true)"
    case "$state" in
    active | activating | reloading | deactivating) return 1 ;;
    *) return 0 ;;
    esac
}

# 背景に置いた systemd-run のクライアントを殺す。これを先にやるのは、生きている限り
# こいつだけがサービスを登録しうるからで、先に殺しておけば後は「もう登録されているものを
# 止める」だけになる。実測（2026-09-17）では TERM で数ミリ秒で死に、そのときサービスは
# active のまま残る。だから殺すだけでは足りず、下の stop_service が続けて止める
stop_run_client() {
    if [ -n "$run_pid" ]; then
        kill "$run_pid" 2>/dev/null || true
        wait "$run_pid" 2>/dev/null || true
        run_pid=""
    fi
}

# この実行が作ったサービスを止める。中断されたとき、これをしないと cargo-mutants は
# 見張りの無いまま走り続ける。
# 止まったことを確かめるまで繰り返すのは、スクリプトの起動の直後に中断が来た場合のため。
# systemd-run を背景に起こしてから "$!" を変数に入れるまでの隙間で中断されるとクライアントを
# 殺せず、また殺せた場合でも登録の要求がバスの上にいると、1度きりの stop の後でサービスが
# 現れうる。止まったという観測が2回続くまで数えないのは、この「後から現れる」を見落とさないため。
# stop は --no-block にする。既定の stop はジョブの完了まで待ち、その上限は TimeoutStopSec
# （既定90秒）なので、待ちをこちらのループに一本化しないと後始末が長く止まる
stop_service() {
    stop_run_client
    [ -n "$run_unit" ] || return 0
    local unit="${run_unit}.service"
    local tries=0 confirmed=0
    while [ "$tries" -lt "$STOP_TRIES" ]; do
        tries=$((tries + 1))
        systemctl --user stop --no-block "$unit" >/dev/null 2>&1 || true
        if service_is_stopped "$unit"; then
            confirmed=$((confirmed + 1))
            if [ "$confirmed" -ge 2 ]; then
                run_unit=""
                return 0
            fi
        else
            confirmed=0
        fi
        sleep "$STOP_INTERVAL"
    done
    printf 'mutants.sh: could not confirm that the service stopped: %s\n' "$unit" >&2
    run_unit=""
}

cleanup() {
    # 後始末の途中に2度目の信号が来ても、ここから先を最後まで走らせる。
    # INT と TERM の trap は "exit" を呼ぶので、無視に替えないと一時ディレクトリの削除と
    # 殺した件数の出力が飛ぶ。終了コードは、最初の信号の trap が決めたものがそのまま残る
    trap '' INT TERM
    set +e
    stop_service
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
    # 消すのは mktemp -d が返したパスだけ。空や "/" を消しに行かないことを先に確かめる
    case "$run_tmpdir" in
    '' | /) ;;
    *)
        if [ -d "$run_tmpdir" ]; then
            rm -rf -- "$run_tmpdir"
        fi
        run_tmpdir=""
        ;;
    esac
}

trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

# A29: 関門の抜け道を塞ぐ。ソースの #[mutants::skip] はその変異を測らせない
guard_skip_attribute() {
    if grep -rn 'mutants::skip' src/ crates/ >/dev/null 2>&1; then
        grep -rn 'mutants::skip' src/ crates/ >&2 || true
        die 'found mutants::skip in the sources; the judgement belongs in the equivalents list'
    fi
}

# 変異テストを走らせ、結果を kotowari mutants に読ませる
run_mutants() {
    guard_skip_attribute
    # A23: 結果のファイルは毎回その場で作る（古い結果を読ませる余地を無くす）
    rm -rf -- mutants.out

    # cargo-mutants は TMPDIR の下に作業の写しを作る。この実行だけのディレクトリを渡して、
    # 見張りが見る範囲をこの実行のものだけに絞る（見張りより先に作る。見張りはここで枝分かれする）
    run_tmpdir="$(mktemp -d)"
    [ -n "$run_tmpdir" ] || die 'cannot make the temporary directory for this run'

    case "$JOBS" in
    '' | *[!0-9]* | 0) die "MUTANTS_JOBS must be a positive integer: $JOBS" ;;
    esac
    case "$SERVICE" in
    systemd | scope | none) ;;
    *) die "MUTANTS_SERVICE must be systemd, scope or none: $SERVICE" ;;
    esac

    start_watchdog
    local status=0
    # 背景に置いて wait で待つ。前面の子を待っている間、bash は trap を後回しにするので、
    # 前面のままだと INT と TERM を受けてもその場で cleanup が走らない。
    # 標準入力は渡さない（cargo-mutants は読まない。背景の実行が端末から読むのを避ける）
    # --test-workspace=true: 既定では変異を入れた crate のテストしか走らない。kotowari-core の
    # 振る舞いを確かめるテストの大半はルートの crate の tests/ にあるので、これが無いと
    # kotowari-core の変異がほぼすべて見逃しになる
    # 境界の中から、所属する cgroup とメモリと CPU の上限を最初に出す（A15）。"max" は上限が無いこと
    local -a mutants_command=(sh -c 'g="/sys/fs/cgroup$(cut -d: -f3 /proc/self/cgroup)"
echo "mutants.sh: cgroup $g memory.max $(cat "$g/memory.max" 2>/dev/null) cpu.max $(cat "$g/cpu.max" 2>/dev/null)"
exec "$@"' sh
        cargo "+$TOOLCHAIN" mutants -j "$JOBS" --no-config --workspace --all-features --test-workspace=true -o . "$@")
    if [ "$SERVICE" = systemd ]; then
        # この実行だけの名前を付ける。中断されたとき cleanup がこの名前でサービスを止める
        run_unit="kotowari-mutants-$$"
        systemd-run --user --wait --collect --pipe --unit="$run_unit" \
            -p MemoryMax=12G -p MemorySwapMax=0 -p OOMPolicy=continue \
            -p CPUQuota="$CPU_QUOTA" -p Nice=19 -p IOSchedulingClass=idle \
            --setenv=PATH="$PATH" \
            --setenv=HOME="$HOME" \
            --setenv=CARGO_UNSTABLE_CHECKSUM_FRESHNESS=true \
            --setenv=CARGO_BUILD_FINGERPRINT=content \
            --setenv=CARGO_BUILD_JOBS="$BUILD_JOBS" \
            --setenv=TMPDIR="$run_tmpdir" \
            --working-directory="$PWD" \
            -- "${mutants_command[@]}" </dev/null &
    elif [ "$SERVICE" = scope ]; then
        # sudo は PATH を入れ替えるので、env で今の値を渡し直す。スコープの中は呼んだ利用者で動かす
        sudo systemd-run --scope --quiet --uid="$(id -u)" --gid="$(id -g)" \
            -p MemoryMax=12G -p MemorySwapMax=0 -p OOMPolicy=continue -p CPUQuota="$CPU_QUOTA" \
            -- env PATH="$PATH" HOME="$HOME" CARGO_UNSTABLE_CHECKSUM_FRESHNESS=true \
            CARGO_BUILD_FINGERPRINT=content CARGO_BUILD_JOBS="$BUILD_JOBS" TMPDIR="$run_tmpdir" \
            "${mutants_command[@]}" </dev/null &
    else
        CARGO_UNSTABLE_CHECKSUM_FRESHNESS=true CARGO_BUILD_FINGERPRINT=content \
            CARGO_BUILD_JOBS="$BUILD_JOBS" TMPDIR="$run_tmpdir" "${mutants_command[@]}" </dev/null &
    fi
    run_pid=$!
    wait "$run_pid" || status=$?
    run_pid=""
    run_unit=""
    stop_watchdog

    # 走り切ったことを示す終了コードだけを通す。cargo-mutants 27.1.0 の src/exit_code.rs より、
    # 0 は成功、2 は見逃しあり、3 は時間切れあり。この3つは全部の変異を測り終えている。
    # 残り（1 引数の誤り、4 基準の実行が落ちた、5 と 6 差分が読めない、70 内部の誤り、
    # および途中で殺されたとき）は測り終えていないので、途中までの結果のファイルが残っていても失敗にする
    case "$status" in
    0 | 2 | 3) ;;
    *) die "the mutation testing tool did not run to completion (exit $status)" ;;
    esac

    if [ ! -f "$RESULTS" ]; then
        # 候補が0件のとき、cargo-mutants は結果のファイルを作らずに0で終わる。同じ範囲と分担で
        # 候補を並べ直し、成功して0件のときだけ通す（A17）
        [ "$status" -eq 0 ] || die "the mutation testing tool wrote no result file (exit $status)"
        local listed
        listed="$(CARGO_UNSTABLE_CHECKSUM_FRESHNESS=true CARGO_BUILD_FINGERPRINT=content \
            cargo "+$TOOLCHAIN" mutants --list --no-config --workspace --all-features "$@" </dev/null)" \
            || die 'the mutation testing tool wrote no result file and could not list the candidates'
        [ -z "$listed" ] || die 'the mutation testing tool wrote no result file although candidates exist'
        printf 'mutants.sh: no candidates in this scope\n'
        return 0
    fi

    # 見逃しや時間切れで止めるかどうかは kotowari の側で決める（上の 2 と 3 では止めない）
    CARGO_BUILD_JOBS=4 cargo run -q -p kotowari-cli --bin kotowari -- mutants --tool cargo-mutants --format text "$RESULTS"
}

run_diff() {
    local base="${1:-}"
    [ -n "$base" ] || die 'usage: mutants.sh diff <base> [-- <cargo-mutants args>...]'
    shift
    if [ "${1:-}" = "--" ]; then
        shift
    fi
    # ここでは fetch しない。解決できない基準はその場で失敗させる
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

# 標準入力の行は git の pre-push と同じ "<ローカルの参照> <ローカルの SHA> <リモートの参照> <リモートの SHA>"。
# タグなら、同じ製品の1つ前のリリースのタグとの差分だけを回し、前のタグが無いときだけ全体を回す
# （判断の記録 docs/decision/records/2026-09-27-release-mutants-scope.md の A1。回す場所はリリースの CI、
# docs/decision/records/2026-10-04-mutants-in-ci.md の A4）。それ以外は origin/main との差分だけ
choose_from_stdin() {
    local local_ref local_sha remote_ref chosen tag product previous
    chosen='diff origin/main'
    while read -r local_ref local_sha remote_ref _; do
        case "${local_ref}${remote_ref}" in
        *refs/tags/*)
            tag="${local_ref#refs/tags/}"
            product="${tag%%-v[0-9]*}"
            if previous="$(git describe --tags --abbrev=0 --match "${product}-v[0-9]*" "${local_sha}^" 2>/dev/null)"; then
                chosen="diff ${previous}"
            else
                chosen='full'
            fi
            ;;
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
    *)
        die 'usage: mutants.sh <diff <base> | full | plan> [-- <cargo-mutants args>...]'
        ;;
    esac
}

main "$@"
