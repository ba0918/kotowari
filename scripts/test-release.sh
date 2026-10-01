#!/usr/bin/env bash
# 操作仕様 docs/release/change-conformance.md の実 Git fixture。外部公開はしない。
set -euo pipefail
cd "$(dirname "$0")/.."
CARGO_BUILD_JOBS=4 cargo build -q -p kotowari
# helper がない旧実装でも prepare の RED を観測できる。
if [ -f scripts/release-record-paths.rs ]; then
    CARGO_BUILD_JOBS=4 cargo build -q -p kotowari --example release-record-paths
fi
python3 scripts/release-fixture.py
