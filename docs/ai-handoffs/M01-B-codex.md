# M01-B-R2 — Codex CLI
State: REVIEW_PENDING; reported backend warning diagnosed as benign, NOT eliminated; owned frame race fixed.
Branch: `codex/m01-b-nested-poc-20261115`
Base: `fd4decfffa1a6e936d5e29f8c851fdb3ecb1a02c` (M01-B/R1 committed before R2).
Head: `fd4decfffa1a6e936d5e29f8c851fdb3ecb1a02c` (unchanged; R2 uncommitted).
Owner: Codex CLI; High requested; session model/effort selection not independently observable.
Confirmed reported source: Winit 0.30.13's inner Calloop 0.13.0 keyboard-repeat slot 3. Exact “non-existence” warning matches that library; generations change as repeats are replaced. No assumption that outer slot 3 is a frame timer.
Root semantics: a key event cancels an expired repeat whose readiness is already batched; generation lookup rejects it, preserving the replacement and already-delivered key input. Pinned-library regression reproduces exact warning/token shape and zero erroneous callbacks.
Separate fix: outer Calloop 0.14.5 Redraw formerly removed an already-queued frame timer. Now it clears demand; the timer retires via Drop in its own callback or serves fresh demand at the revised deadline. One finite retirement wake, no permanent idle timer/polling.
Runtime source/lines: `crates/blonded-compositor/src/state.rs:122` scheduling/retirement and `:154` begin_frame; `src/main.rs:198` Redraw, `:291` shutdown; `src/main.rs:5` test-module declaration (same crate).
Tests/source: new `crates/blonded-compositor/src/lifecycle_tests.rs:186` onward (seven tests); dev dependencies in crate `Cargo.toml:18`; `crates/Cargo.lock` graph entries only, no version changes.
Other changed files: crate `README.md`, `docs/audits/M01-B-R2-event-source-lifecycle.md`, `docs/AI_TEAM_STATE.md`, this handoff.
Checks (from `crates/`, `CARGO_BUILD_JOBS=1`, existing temporary Cargo cache): `cargo fmt --all -- --check` PASS; `cargo clippy --offline --locked -p blonded-compositor -p blonded-wm --all-targets -- -D warnings` PASS.
Tests/build: `cargo test --offline --locked -p blonded-compositor -p blonded-wm` PASS (**40**: 17 compositor + 23 WM); `cargo build --offline --locked -p blonded-compositor --bins` PASS.
Focused: both pre-fix warning paths reproduced; 1,000 repeated schedules; queued early redraw/cancellation/re-registration; fresh demand/deadline reuse; 64 geometry bursts/2,048 drag motions; queued shutdown. Fixed owned-scheduler cases: zero WARNs, no lost redraw, no residual timer/valid retired token.
Smoke: `python scripts/smoke-nested.py` PASS; 3 real clients, remap IDs 3→4, keyboard focus/frame callbacks/readback, 0700 socket, SIGINT exit 0, zero WM windows/invariants OK, socket/directory removed. Neither warning appeared; smoke did NOT exercise host keyboard repeat.
Graphics/input: owner R2 evidence already VERIFIED Intel HD Graphics 2000/GLES 3.0, physical mouse move/resize, keyboard geometry and shutdown. Preserved; no GPU diagnostics or host input injection. Protected smoke's software renderer does not revise owner hardware results.
Sample: smoke process-only RSS 131,456 KiB; 1 CPU tick/100Hz over 2.000s. No FPS/system-idle/release claim.
Remaining: benign backend 0.13 warning can still recur; private Winit repeat removal needs a backend change to eliminate it behaviorally. No warning suppression, runtime upgrade or fork. New physical interactive R2 confirmation pending; existing compositor feature limits unchanged.
Git BLOCKED: scoped `git add` exit 128, `.git/index.lock: Read-only file system`; files preserved, no commit/reset/permission/sandbox workaround or push.
Evidence: detailed source map, version distinction and test commands in R2 audit report; temporary runtime logs/readback kept local. Host, renderer path and OS settings unchanged.
Next allowed action: ChatGPT/owner review of R2, with explicit distinction between fixed owned scheduler and benign remaining backend warning. **STOP after M01-B-R2; no M02 or UI.**
