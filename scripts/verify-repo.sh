#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
for file in README.md AGENTS.md docs/ARCHITECTURE.md docs/PERFORMANCE.md docs/ROADMAP.md docs/audits/M00-host-facts.md codex/M00-NEXT-TASK.md crates/Cargo.toml; do
  test -s "$file" || { echo "Required file missing: $file" >&2; exit 1; }
done
if command -v cargo >/dev/null; then
  (cd crates && cargo fmt --all -- --check && cargo test --workspace)
else
  echo "Cargo unavailable on this host. Rust checks remain unverified locally; GitHub CI is configured."
fi
echo "Scaffold structure present. This does NOT verify bootable OS readiness."
