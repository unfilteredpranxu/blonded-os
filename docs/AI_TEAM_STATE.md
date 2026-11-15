# Blonded OS — AI team checkpoint

> Short shared state for local **Codex CLI + AGY CLI**. Read this with `AGENTS.md` first. Detailed rules: `docs/AI-TEAM-PROTOCOL.md`.

- **Project:** original Rust-first Blonded desktop/system running **in-place on the existing Arch installation** (NOT ISO/install/reinstall; not Hyprland dotfiles).
- **Canonical repo:** `unfilteredpranxu/blonded-os`.
- **Actual branch:** `codex/m00-issue-1-audit-20261112`; task branch `codex/m01-a-window-management-core` creation **BLOCKED** by read-only `.git`. No branch switch or commit succeeded; source is preserved in the working tree.
- **Actual local base/HEAD:** `4a630c6f4f1ec668dfb450388cc3b953369ded15` (clean at startup).
- **Remote baseline observed:** `4b499fe4ec49cacc6baf0df35a080818d4708849` via `git ls-remote`; GitHub comparison confirms seven newer documentation-only commits. Read latest remote AGENTS/state/strategy; no Rust changes upstream in this comparison. Local fetch **BLOCKED** by read-only `.git/FETCH_HEAD`; remote baseline is not checked out.
- **Milestone:** M01-A original headless Rust window-management core implemented in working tree, per corrected owner priority. Profile/theme engine is deferred. M00 memory/recovery/FPS gates still block desktop deployment.
- **Issue:** https://github.com/unfilteredpranxu/blonded-os/issues/2; inspected live and confirmed supersession of the config-first brief. Hardware audit remains issue #1.
- **Current task owner:** Codex CLI; implementation stopped for ChatGPT/owner review. No parallel writer or follow-on task.
- **Assignment status:** `REVIEW_PENDING` for source/tests; Git branch/update/commit handoff **BLOCKED** by sandbox. Do not merge or claim committed completion.
- **Agents:** Codex CLI (PC-local default writer); AGY CLI (PC-local reviewer / separately assigned writer); ChatGPT chooses per-task AI and mode. No Codex Cloud, Jules or Gemini CLI.
- **Autonomy preference:** owner pastes one prompt and wants no routine approvals. Use protected Codex `--sandbox workspace-write --ask-for-approval never` or AGY `--mode=auto` when supported; consult `docs/HANDS-OFF-CLI-AUTONOMY.md`. Denied/high-risk actions: record `BLOCKED`, never bypass sandbox or interrupt for routine approvals.
- **Host:** Intel i3-2100 / 4GB / HDD / Intel HD 2000, 1366x768@59.79Hz. Intel GLX accelerated **yes**, OpenGL 3.3, GLES 3.0, EGL 1.5. Earlier Vulkan llvmpipe software. No proven compositor blur/FPS. Current kernel last reported `7.3.4-arch1-1`.
- **Performance gate:** <=500 MiB **whole-system** steady desktop idle **NOT TESTED**; existing >2GB sample contains actively running user apps.
- **Boot:** previous ~91.8s to graphical target; Docker service +21.934s on critical userspace path after NetworkManager wait-online +7.566s. Kernel phase ~44.9s previously; cause unverified. No service changes authorized.
- **UI requirements:** before each *new* component, ask owner visual preferences. Full profile hot-swap; `Super+T` themes, `Super+W` wallpaper/desktop, `Super+O` window materials; specs `docs/THEME-SHELL-CONTRACT.md`. Long-term study/music multitasking.
- **Protected:** existing Arch/Hyprland functioning session, installed packages, boot/disks/services. No ISO or reinstall. No installs/autostart/live configuration changes without separate approval and proven recovery.
- **Source changes:** workspace member/lockfile, `crates/blonded-wm/` (geometry/model/dispatcher, headless demo, bounded policy load, tests, README), this checkpoint and `docs/ai-handoffs/M01-A-Codex.md`. Existing crate APIs and CI preserved. No third-party dependencies or repository license selected.
- **Implemented:** registration/removal, constrained floating move/resize, focus/cycle/raise/stacking, maximize/restore/minimize, rows/columns, eight snap regions, workspace isolation/transfer, screen/gap/minimum reconfiguration, deterministic event batches, invariant checks and atomic failure. Hard caps: 256 windows / 32 workspaces.
- **Verification:** `cargo fmt --all -- --check` PASS; `CARGO_BUILD_JOBS=1 cargo clippy --workspace --all-targets --offline -- -D warnings` PASS; `CARGO_BUILD_JOBS=1 cargo test --workspace --offline` PASS (25 tests: 23 new, 2 existing); `CARGO_BUILD_JOBS=1 cargo test --release --offline -p blonded-wm` PASS (23 tests). Coverage includes 6,000-command replay, integer partitions, invalid/overflow geometry, lifecycle, layout, workspace, capacity, corruption and atomic errors.
- **Demo/performance:** `CARGO_BUILD_JOBS=1 cargo run --offline -p blonded-wm --example headless` PASS (19 commands); release `policy_load` PASS (256 windows / 32 workspaces / 4,096 commands, 420,999 us total, 102,783 ns mean/dispatch in this environment). Synthetic policy only; no rendering/FPS/system-idle inference.
- **Environment:** local repository commands; rustc 1.99.0 / cargo 1.99.0; Codex CLI 0.164.0 reported installed. No model/effort or new physical graphics measurement inferred. Cargo builds used one job, offline, with no new dependencies.
- **Unimplemented/unverified:** Wayland/Smithay integration, visible client windows, input/rendering, client-specific hints, multi-output support, UI/animation, live recovery, total idle RAM and compositor FPS. No host/session/settings/package changes made.
- **Next allowed action:** ChatGPT/owner reviews source and evidence; reconcile onto the latest authorized baseline, create the intended task branch and commit only when Git writes are permitted. M01-B requires a separate assignment. No automatic milestone progression.
- **Last checkpoint writer:** Codex CLI, 2026-11-15; source evidence only, not proof of desktop deployment. Detailed handoff: `docs/ai-handoffs/M01-A-Codex.md`.

**Handoff rule:** After assigned work, update owner/status/branch/commit/tests/blockers here (keep <=80 lines), optionally add `docs/ai-handoffs/<TASK-ID>-<agent>.md`, commit on task branch, and stop for review. No hidden shared AI memory, no task continuation unless assigned.
