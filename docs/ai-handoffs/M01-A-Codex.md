# M01-A — Codex
State: REVIEW_PENDING (source/tests); BLOCKED (Git branch/fetch/staging/commit)
Owner: Codex CLI, one writer; original headless WM policy, per corrected issue #2.
Intended branch: `codex/m01-a-window-management-core` (creation denied).
Actual branch: `codex/m00-issue-1-audit-20261112`.
Actual base and unchanged HEAD: `4a630c6f4f1ec668dfb450388cc3b953369ded15`; no implementation commit exists.
Latest remote baseline observed: `4b499fe4ec49cacc6baf0df35a080818d4708849` (seven documentation-only commits ahead); latest remote AGENTS/state/strategy inspected using GitHub connector. Fetch denied; not checked out.
Changed: `crates/Cargo.toml`, `crates/Cargo.lock`, `crates/blonded-wm/{Cargo.toml,README.md,src/{lib,geometry,model,manager}.rs,tests/policy.rs,examples/{headless,policy_load}.rs}`, checkpoint and this handoff.
Implemented: registration/removal, IDs/geometry, floating move/resize, focus/cycle/raise/stacking, maximize/minimize/restore, rows/columns, eight edge/corner snaps, isolated workspaces and transfer, screen/gap/minimum reflow, deterministic atomic command/event dispatcher and invariant checks.
Bounds: 256 windows / 32 workspaces; std only; no I/O, polling, background processes, graphics dependency or copied WM code; no repository license selected.
Tests (from `crates/`):
- `cargo fmt --all -- --check` => PASS.
- `CARGO_BUILD_JOBS=1 cargo clippy --workspace --all-targets --offline -- -D warnings` => PASS, no warnings.
- `CARGO_BUILD_JOBS=1 cargo test --workspace --offline` => PASS, 25 tests (23 new + 2 existing), 0 failed; includes 6,000-command replay and integer-partition checks.
- `CARGO_BUILD_JOBS=1 cargo test --release --offline -p blonded-wm` => PASS, 23 tests, 0 failed.
- `CARGO_BUILD_JOBS=1 cargo run --offline -p blonded-wm --example headless` => PASS, 19 commands; final window 1 at (90,110,600,400), window 2 removed, workspace 0 active.
- `CARGO_BUILD_JOBS=1 cargo run --release --offline -p blonded-wm --example policy_load` => PASS, 256 windows / 32 workspaces / 4,096 commands / 528,384 returned events; 420,999 us total, 102,783 ns mean per dispatch, invariant check passed.
Hardware: no new graphics, FPS, whole-system memory or deployment test. Synthetic timing from execution environment only; rustc/cargo 1.99.0, one Cargo job, offline builds.
Genuinely working: typed headless Rust policies and reproducible simulation/tests; errors preserve prior state, window disappearance cleans references and layouts.
Unimplemented: Wayland/Smithay protocol/backend, visible Linux clients, renderer/input, per-client constraints, multiple outputs, UI/animation and live recovery.
BLOCKED: `git fetch` cannot write `.git/FETCH_HEAD`; `git switch -c` and `git add` cannot create `.git/index.lock` (read-only filesystem). No bypass, alternative clone, remote mutation or successful Git write. Original data retained; full scoped diff also saved as `/tmp/blonded-M01A.patch` (local temporary artifact, not committed).
Risks: bounded copy/reflow per command favors atomicity; backend must handle Wayland configure/ack/commit and delayed callbacks on reused IDs. Rendering/RAM/recovery gates remain open. README documents exact policies and adapter contract.
Next allowed action: ChatGPT/owner review; reconcile onto latest baseline and branch/commit once Git writes are permitted. No M01-B or live session changes without a new assignment.
