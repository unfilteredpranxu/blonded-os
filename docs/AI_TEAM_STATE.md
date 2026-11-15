# Blonded OS — AI team checkpoint

> Short shared state for local **Codex CLI + AGY CLI**. Read this with `AGENTS.md` first. Detailed rules: `docs/AI-TEAM-PROTOCOL.md`.

- **Project:** original Arch-derived Rust-first OS distribution, not Hyprland dotfiles.
- **Canonical repo:** `unfilteredpranxu/blonded-os`.
- **Working branch for current documentation:** `codex/m00-issue-1-audit-20261112`; **not yet merged to `main`**. Check local branch + HEAD before acting.
- **Milestone:** M00 hardware/performance/recovery verification **ACTIVE**. Do not start WM, compositor, theme UI, ISO, installer or remove system packages.
- **Issue:** https://github.com/unfilteredpranxu/blonded-os/issues/1
- **Current task owner:** none; waiting for owner/ChatGPT task selection.
- **Assignment status:** `IDLE / REVIEW_PENDING` — existing documentation and physical host probes reviewed conversationally; no implementation authorized.
- **Agents:** Codex CLI (PC-local default writer); AGY CLI (PC-local reviewer / separately assigned writer); ChatGPT chooses per-task AI and mode. No Codex Cloud, Jules or Gemini CLI.
- **Host:** Intel i3-2100 / 4GB / HDD / Intel HD 2000, 1366x768@59.79Hz. Intel GLX accelerated **yes**, OpenGL 3.3, GLES 3.0, EGL 1.5. Earlier Vulkan llvmpipe software. No proven compositor blur/FPS. Current kernel last reported `7.3.4-arch1-1`.
- **Performance gate:** <=500 MiB **whole-system** steady desktop idle **NOT TESTED**; existing >2GB sample contains actively running user apps.
- **Boot:** previous ~91.8s to graphical target; Docker service +21.934s on critical userspace path after NetworkManager wait-online +7.566s. Kernel phase ~44.9s previously; cause unverified. No service changes authorized.
- **UI requirements:** before each *new* component, ask owner visual preferences. Full profile hot-swap; `Super+T` themes, `Super+W` wallpaper/desktop, `Super+O` window materials; specs `docs/THEME-SHELL-CONTRACT.md`. Long-term study/music multitasking.
- **Protected:** original Arch/Hyprland functioning login; never touch boot/disks/services/uninstall anything without explicit approval and proven recovery.
- **Latest completed work:** M00 feasibility audit and sanitized physical-host findings; comprehensive future UI, app-migration, roadmap, and CLI-only workflow specifications on this branch. Check Git log for **exact** latest SHAs.
- **Outstanding:** actual CLI version/options check (`codex --version`; `agy --help`); M00 safe fallback/clean idle baseline still incomplete; compositor decision not approved.
- **Next allowed step:** await ChatGPT-issued **single bounded assignment** after reviewing owner evidence; no autonomous stage progression.
- **Last checkpoint writer:** ChatGPT, 2026-11-15; this file is a requirements/status snapshot, **not** proof of a hardware test or compiled OS.

**Handoff rule:** After assigned work, update owner/status/branch/commit/tests/blockers here (keep <=80 lines), optionally add `docs/ai-handoffs/<TASK-ID>-<agent>.md`, commit on task branch, and stop for review. No hidden shared AI memory, no task continuation unless assigned.
