#!/bin/sh
# Explicit development launch only. No autostart, installs or host configuration.
set -eu
cd "$(dirname "$0")/../crates"
export CARGO_BUILD_JOBS=1
cargo build --locked -p blonded-compositor --bins
exec ./target/debug/blonded-compositor --nested "$@"
