# M01-B / M01-B-R1 — Codex CLI
State: REVIEW_PENDING; R1 source preserved, commit BLOCKED by sandbox.
Branch: `codex/m01-b-nested-poc-20261115`
R1 base: `7ded1f2d54ca182f870bb0353d5c1a9a2fac1d61` (original M01-B implementation committed by owner).
Head: `7ded1f2d54ca182f870bb0353d5c1a9a2fac1d61` (unchanged; R1 uncommitted).
Owner: Codex CLI; High requested; session model/effort selection not independently observable.
Backend: unchanged crates.io Smithay **=0.7.0**, upstream SHA `a166cf4c94b5aedc332a65aa1dd753e8148829c3`; winit 0.30.13, Wayland/EGL/GLES; no Vulkan requirement.
Changed source: `crates/blonded-compositor/src/input.rs`, `src/main.rs`, `src/policy.rs`, `src/state.rs` (same crate); no WM core/dependency changes.
Changed docs: crate `README.md`, `docs/audits/M01-B-R1-GPU-diagnostics.md`, `docs/AI_TEAM_STATE.md`, this handoff.
Fix 1: keyboard moves saturate; resize dimensions use clamped i64 sums before conversion. Tests cover positive/negative coordinate limits, maximum dimensions and shrinking below one.
Fix 2: valid outputs down to 120×96 and unusual aspect ratios synchronize WM/output; invalid/zero sizes suspend drawing/input with valid snapshots retained; valid resize resumes. Actual size rechecked before redraw/readback; obsolete modes retired/current mode preferred; startup can remain dormant.
Checks (from `crates/`, `CARGO_BUILD_JOBS=1`, temporary writable Cargo cache): `cargo fmt --all -- --check` PASS; `cargo clippy --offline --locked -p blonded-compositor -p blonded-wm --all-targets -- -D warnings` PASS.
Tests: `cargo test --offline --locked -p blonded-compositor -p blonded-wm` PASS (10 adapter + 23 WM = **33**, seven new regressions); `cargo build --offline --locked -p blonded-compositor --bins` PASS.
Runtime: `python scripts/smoke-nested.py` PASS: 3 real clients, remap IDs 3→4, keyboard focus/frame callbacks/readback, 0700 socket, SIGINT exit 0, zero WM windows/invariants OK and socket/directory removed. Separate 15s run exited 0.
Window: Hyprland query while process alive identified one mapped native Wayland 439×755 window; actual window-only screen capture inspected, both checkerboards visible. Host mapping/screen presentation **VERIFIED**; human physical-monitor observation NOT VERIFIED.
Input: keyboard protocol focus VERIFIED; physical pointer focus/move/resize and live small-window resize **NOT VERIFIED**; policy/Smithay resize integration tests pass. No input injected into host applications.
GPU: **NO hardware acceleration in observed runs**: llvmpipe LLVM 23.1.4, GLES 3.2/Mesa 26.2.7-arch1.1. Standalone GLX/EGL also software; GBM init fails. `/dev/dri` absent (ENOENT), while sysfs identifies 8086:0102/i915/renderD128.
Failure stage: Mesa DRM-device/DRI-screen initialization during `eglInitialize`, before GLES rendering; no inherited software-forcing override found. Missing device visibility is a confirmed blocker; complete host-side cause remains unproven. See diagnostic report for commands/errors/ruled-out causes.
Sample: debug compositor RSS 134,040 KiB; 3 CPU ticks/100Hz over 2.000s (0.03s CPU). No system-idle RAM, FPS, blur or recovery claim.
Limits: popups dismissed; client decoration move/resize requests ignored; no dmabuf/touch/IME/HiDPI validation, size hints/fullscreen/shell UI. Existing limitations remain; no M02 or deployment.
Git BLOCKED: scoped `git add` failed, `.git/index.lock: Read-only file system`; no commit/reset/permission/sandbox workaround or push.
Evidence: local temporary logs/readback/window-only screenshot; sanitized GPU findings checked in. Host configuration/packages/services/boot untouched; repository license unchanged.
Next allowed action: ChatGPT/owner review; one owner-run normal-Hyprland-terminal comparison of render-node visibility and `eglinfo -B -p wayland`, as documented. **STOP after M01-B-R1.**
