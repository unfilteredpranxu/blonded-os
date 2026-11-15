# Blonded OS — AI team checkpoint

> Short shared state for local **Codex CLI + AGY CLI**. Read this with `AGENTS.md` first. Detailed rules: `docs/AI-TEAM-PROTOCOL.md`.

- **Project:** original Rust-first Blonded desktop/system running **in-place on the existing Arch installation** (NOT ISO/install/reinstall; not Hyprland dotfiles).
- **Canonical repo:** `unfilteredpranxu/blonded-os`.
- **Baseline for new assignment:** `codex/m01-a-integrated-20261115-212449` (contains audited project docs + tested original WM source). The canonical audit branch and `main` remain separate; inspect branch/HEAD before acting. Use an isolated M01-B task branch from this baseline.
- **Milestone:** **M01-A COMPLETE/REVIEWED** — native Rust window management, tested on owner Arch PC. **M01-B IMPLEMENTED/REVIEW_PENDING** — real nested client rendering/lifecycle verified; physical presentation/input and Intel acceleration remain unverified. M00 memory/recovery/FPS gates still block deployment. No new ISO, installer, boot, service or package changes.
- **Active coding issue:** https://github.com/unfilteredpranxu/blonded-os/issues/5 (M01-B nested compositor); previous implemented core issue #2; audit issue #1. Source-integrated PR #4.
- **Current task owner:** Codex CLI; M01-B implementation finished, waiting for ChatGPT/owner review.
- **Assignment status:** `M01-B REVIEW_PENDING` — nested Smithay 0.7.0 crate, WM lifecycle/placement/focus, input controls and real shm clients implemented; 26 scoped tests/fmt/Clippy/build PASS. No M02, deployment or host configuration changes.
- **Agents:** Codex CLI (PC-local default writer); AGY CLI (PC-local reviewer / separately assigned writer); ChatGPT chooses per-task AI and mode. No Codex Cloud, Jules or Gemini CLI.
- **Autonomy preference:** owner pastes one prompt and wants no routine approvals. Use protected Codex `--sandbox workspace-write --ask-for-approval never` or AGY `--mode=auto` when supported; consult `docs/HANDS-OFF-CLI-AUTONOMY.md`. Denied/high-risk actions: record `BLOCKED`, never bypass sandbox or interrupt for routine approvals.
- **Host:** Intel i3-2100 / 4GB / HDD / Intel HD 2000, 1366x768@59.79Hz. Intel GLX accelerated **yes**, OpenGL 3.3, GLES 3.0, EGL 1.5. Earlier Vulkan llvmpipe software. No proven compositor blur/FPS. Current kernel last reported `7.3.4-arch1-1`.
- **Performance gate:** <=500 MiB **whole-system** steady desktop idle **NOT TESTED**; existing >2GB sample contains actively running user apps.
- **Boot:** previous ~91.8s to graphical target; Docker service +21.934s on critical userspace path after NetworkManager wait-online +7.566s. Kernel phase ~44.9s previously; cause unverified. No service changes authorized.
- **UI requirements:** before each *new* component, ask owner visual preferences. Full profile hot-swap; `Super+T` themes, `Super+W` wallpaper/desktop, `Super+O` window materials; specs `docs/THEME-SHELL-CONTRACT.md`. Long-term study/music multitasking.
- **Protected:** existing Arch/Hyprland functioning session, installed packages, boot/disks/services. No ISO or reinstall. No installs/autostart/live configuration changes without separate approval and proven recovery.
- **Latest completed work:** M01-A original Rust `blonded-wm` source integrated at `b3e754ab91ed7b0fff0634aed7245c78f8c11a41`; user independently ran 25 tests successfully; AGY audit PASS on original source; PR #4. M00 physical GPU evidence and other product specifications remain intact.
- **M01-B runtime:** real client buffers/render readback/frame callbacks, unmap/remap/disconnect and SIGINT/timed shutdown PASS; private socket directory verified 0700 and removed. Software llvmpipe GLES 3.2 observed; Intel acceleration, physical presentation and pointer/move/resize remain NOT VERIFIED.
- **M01-B sample:** debug compositor RSS 134,320 KiB; 1 CPU tick (100Hz) over 2.000s. Process-only software sample, no system idle/FPS acceptance. M00 recovery/clean idle gates remain open.
- **Task branch/base/head:** `codex/m01-b-nested-poc-20261115`, base = head `f480beec0c4b4b0a9aa5be68ce8e87404e126fd7`; source is uncommitted. `git add` BLOCKED: read-only `.git/index.lock`; no sandbox bypass. See `docs/ai-handoffs/M01-B-codex.md` and crate README.
- **Next allowed step:** ChatGPT/owner review of preserved M01-B diff/evidence, then separately assigned physical input/accelerated rendering validation or scoped fixes. No automatic milestone progression, session integration or M02.
- **Last checkpoint writer:** Codex CLI, 2026-11-15; actual M01-B checks above executed in protected local workspace. Do not substitute software readback for physical display/input or hardware acceleration evidence.

**Handoff rule:** After assigned work, update owner/status/branch/commit/tests/blockers here (keep <=80 lines), optionally add `docs/ai-handoffs/<TASK-ID>-<agent>.md`, commit on task branch, and stop for review. No hidden shared AI memory, no task continuation unless assigned.
