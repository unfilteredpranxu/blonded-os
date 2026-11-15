# M01-B — Codex CLI
State: REVIEW_PENDING; source preserved, commit BLOCKED by sandbox.
Branch: `codex/m01-b-nested-poc-20261115`
Base: `f480beec0c4b4b0a9aa5be68ce8e87404e126fd7`
Head: `f480beec0c4b4b0a9aa5be68ce8e87404e126fd7` (unchanged; uncommitted working tree).
Owner: Codex CLI 0.164.0; High requested; session model/effort selection not independently observable.
Backend: crates.io Smithay **=0.7.0**, release SHA `a166cf4c94b5aedc332a65aa1dd753e8148829c3`; winit 0.30.13, Wayland/EGL/GLES, no Vulkan requirement.
Changed: `crates/Cargo.toml`, `crates/Cargo.lock`; new `crates/blonded-compositor/Cargo.toml`, `README.md`, `THIRD-PARTY.md`, `licenses/Smithay-MIT.txt`.
Source: new crate `src/{main,state,handlers,input,policy}.rs`, `src/bin/smoke_client.rs`; WM engine source unchanged.
Tools/docs: `scripts/dev-nested.sh`, `scripts/smoke-nested.py`, this handoff and `docs/AI_TEAM_STATE.md`.
Features: private explicitly-0700 socket; actual map/unmap/remap/disconnect → WM lifecycle; WM geometry/focus/stacking → Smithay; demand rendering, coalesced pointer/drag; keyboard controls; bounded shutdown/own-child reaping.
Checks (from `crates/`, `CARGO_BUILD_JOBS=1`, temporary writable Cargo cache): `cargo fmt --all -- --check` PASS; `cargo clippy --locked -p blonded-compositor -p blonded-wm --all-targets -- -D warnings` PASS.
Tests: `cargo test --locked -p blonded-compositor -p blonded-wm` PASS (3 adapter + 23 WM = 26); `cargo build --locked -p blonded-compositor --bins` PASS; shell/Python syntax and `git diff --check` PASS.
Runtime: `python scripts/smoke-nested.py` PASS: 3 real shm clients, remap IDs 3→4, keyboard focus/frame callbacks, framebuffer readback, SIGINT exit 0, zero WM windows/invariants OK, socket/directory removed. Timed 6s run also exit 0.
Display: real client pixel rendering/submission VERIFIED; physical Hyprland presentation NOT VERIFIED (outer window absent from identifiable host window query). Output selected by host was 439×755.
Input: keyboard protocol focus VERIFIED; actual pointer focus/move/resize NOT VERIFIED; policy move/resize/coalescing tests pass. Targeted host shortcut attempts could not identify the outer window.
Renderer: software **llvmpipe (LLVM 23.1.4, 256 bits)**, GLES 3.2/Mesa 26.2.7-arch1.1; DRI2 EGL BAD_ALLOC before fallback. Intel acceleration in this prototype NOT VERIFIED; no GPU/sandbox bypass attempted.
Sample: compositor-only debug RSS 134,320 KiB; 1 CPU tick/100Hz over 2.000s (0.01s CPU). No system-idle RAM, FPS, blur or recovery measurement; release/deployment gates remain open.
Resolved: framebuffer mapping left EGL surfaceless; explicit render/finish restored swap target. Final capture/lifecycle smoke passed. Tempfile directory permissions explicitly fixed and verified 0700.
Limits: popups dismissed; client decoration move/resize requests ignored; no dmabuf, touch/IME, HiDPI validation, client size hints, fullscreen/shell/UI; see crate README for exact interactions and smoke commands.
Git BLOCKED: `git add ...` exit 128, `.git/index.lock: Read-only file system`; no commit/branch/reset workaround, push or host configuration change.
Evidence: local smoke runner prints a private temporary evidence directory with logs/readback/summary; raw host queries and private paths are not committed. Smithay MIT notice retained; repository license unchanged.
Next allowed action: ChatGPT/owner review and separately assigned physical-input/accelerated-renderer validation or scoped fixes. **STOP; no M02 or deployment.**
