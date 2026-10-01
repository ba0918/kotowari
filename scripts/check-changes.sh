#!/usr/bin/env bash
set -euo pipefail
script_dir=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
CARGO_BUILD_JOBS=4 cargo run -q -p kotowari --manifest-path "$script_dir/../Cargo.toml" -- \
  changes --base HEAD --staged --phase implementation
