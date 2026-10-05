#!/usr/bin/env bash
#
# Rust の関門を回す（判断の記録 docs/decision/records/2026-10-06-ci-rust-gates.md）。CI の "Rust gates" と
# 手元の両方の入口。整形の検査、warnings を誤りにした lint（既定の feature と全 feature）、テスト全件の順に回し、
# 最初に落ちたところで止まる。どれも Cargo.lock を書き換えない（--locked）。
#
# 終了コード: 0 すべて通った、それ以外 落ちたコマンドの終了コード

set -euo pipefail

cd "$(dirname "$0")/.."

cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
