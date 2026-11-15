# Blonded OS — AI team checkpoint

> Short shared state for local **Codex CLI + AGY CLI**. Read this with `AGENTS.md` first. Detailed rules: `docs/AI-TEAM-PROTOCOL.md`.

- **Project:** original Rust-first Blonded desktop/system running **in-place on the existing Arch installation** (NOT ISO/install/reinstall; not Hyprland dotfiles).
- **Canonical repo:** `unfilteredpranxu/blonded-os`.
- **Baseline for new assignment:** `codex/m01-a-integrated-20261115-212449` (contains audited project docs + tested original WM source). The canonical audit branch and `main` remain separate; inspect branch/HEAD before acting. Use an isolated M01-B task branch from this baseline.
- **Milestone:** **M01-A COMPLETE/REVIEWED** — native Rust window management, tested on owner Arch PC, no graphics yet. **M01-B ACTIVE/READY** — nested Wayland compositor proof, real test client surfaces using Blonded WM policies; no host compositor replacement. M00 memory/recovery/FPS gates still block deployment. No new ISO, installer, boot, service or package changes.
- **Active coding issue:** https://github.com/unfilteredpranxu/blonded-os/issues/5 (M01-B nested compositor); previous implemented core issue #2; audit issue #1. Source-integrated PR #4.
- **Current task owner:** none; waiting for owner/ChatGPT task selection.
- **Assignment status:** `READY FOR M01-B` — one Codex CLI High (if available) task to create a minimal, manually launched, nested Rust Wayland compositor and demonstrate a real client surface. No autostart, host replacement, panel theme UI or unverified performance claim.
- **Agents:** Codex CLI (PC-local default writer); AGY CLI (PC-local reviewer / separately assigned writer); ChatGPT chooses per-task AI and mode. No Codex Cloud, Jules or Gemini CLI.
- **Autonomy preference:** owner pastes one prompt and wants no routine approvals. Use protected Codex `--sandbox workspace-write --ask-for-approval never` or AGY `--mode=auto` when supported; consult `docs/HANDS-OFF-CLI-AUTONOMY.md`. Denied/high-risk actions: record `BLOCKED`, never bypass sandbox or interrupt for routine approvals.
- **Host:** Intel i3-2100 / 4GB / HDD / Intel HD 2000, 1366x768@59.79Hz. Intel GLX accelerated **yes**, OpenGL 3.3, GLES 3.0, EGL 1.5. Earlier Vulkan llvmpipe software. No proven compositor blur/FPS. Current kernel last reported `7.3.4-arch1-1`.
- **Performance gate:** <=500 MiB **whole-system** steady desktop idle **NOT TESTED**; existing >2GB sample contains actively running user apps.
- **Boot:** previous ~91.8s to graphical target; Docker service +21.934s on critical userspace path after NetworkManager wait-online +7.566s. Kernel phase ~44.9s previously; cause unverified. No service changes authorized.
- **UI requirements:** before each *new* component, ask owner visual preferences. Full profile hot-swap; `Super+T` themes, `Super+W` wallpaper/desktop, `Super+O` window materials; specs `docs/THEME-SHELL-CONTRACT.md`. Long-term study/music multitasking.
- **Protected:** existing Arch/Hyprland functioning session, installed packages, boot/disks/services. No ISO or reinstall. No installs/autostart/live configuration changes without separate approval and proven recovery.
- **Latest completed work:** M01-A original Rust `blonded-wm` source integrated at `b3e754ab91ed7b0fff0634aed7245c78f8c11a41`; user independently ran 25 tests successfully; AGY audit PASS on original source; PR #4. M00 physical GPU evidence and other product specifications remain intact.
- **Outstanding:** confirm available AGY CLI versions/models; M00 safe fallback/clean idle baseline incomplete; no real compositor proof or FPS/RAM measurements yet. Smithay nested backend suitability must be demonstrated, not assumed.
- **Next allowed step:** One local Codex CLI High (if offered) prompt for M01-B on an isolated task branch from `codex/m01-a-integrated-20261115-212449`. Implement only nested compositor and real-client proof with safe shutdown, no package/system changes. User returns build/run logs for review; no autonomous M02.
- **Last checkpoint writer:** ChatGPT, 2026-11-15; M01-A tests **were** run on owner's Arch PC; M01-B has not been implemented or verified. This file is a team planning checkpoint, not proof of compositor functionality.

**Handoff rule:** After assigned work, update owner/status/branch/commit/tests/blockers here (keep <=80 lines), optionally add `docs/ai-handoffs/<TASK-ID>-<agent>.md`, commit on task branch, and stop for review. No hidden shared AI memory, no task continuation unless assigned.
