#!/usr/bin/env bash
#
# 依存のライセンスと既知の脆弱性を検査する。
# CI の "Rust gates" と手元の両方の入口。何を落とすかは deny.toml に書いてある。
# 脆弱性の一覧をネットワークから取るので、scripts/gates.sh には入れていない。
# 手元で回すには cargo-deny が要る。Cargo.lock は書き換えない（--locked）。
#
# 終了コード: 0 通った、それ以外 cargo-deny の終了コード

set -euo pipefail

cd "$(dirname "$0")/.."

# --workspace を外すと、根のパッケージ（kotowari-cli）から届く依存だけを見て、
# kotowari-mds だけが使う依存（HTTP と TLS のクレート）を検査から落とす
cargo deny --locked --workspace check advisories licenses
