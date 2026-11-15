# AI engineering rules — Blonded OS

## Required cross-agent startup (Codex CLI and AGY CLI)

**Hands-off policy:** Read `docs/HANDS-OFF-CLI-AUTONOMY.md`. Owner wants no routine permission interruptions: execute scoped repository edits/builds/tests autonomously using protected sandbox modes; if an action is denied, skip and record `BLOCKED`. Never bypass sandboxes, install/remove packages or modify live host services/disks on your own.

Read `docs/AI_TEAM_STATE.md` **first**, then this file, then `docs/AI-TEAM-PROTOCOL.md` only if this is a new task or handoff. Confirm current branch/HEAD and `git status --short`; only inspect assigned issue and task-relevant files. The shared checkpoint + Git history, not prior chat transcripts, are authoritative records of what the other AI did. Do not run broad repository re-audits or print full logs just to build context. After an assigned task, checkpoint the outcome with exact commits/tests and STOP for ChatGPT/owner review. Do not work on `main` assuming unmerged audit-branch docs exist there.

## Scope
Blonded OS is a **Rust-first system/desktop replacement layer built in-place on the owner's existing Arch Linux machine**. The owner explicitly declined a new ISO, installer or separate installation. Do not build ArchISO or touch partitions/boot. Ultimate deliverable: original Rust window management/compositing, system shell, live themes and native apps integrated safely into the existing Arch base, not merely a Hyprland rice.

## Milestone boundary: M00 partly complete; M01-A isolated Rust coding permitted
1. Read `docs/audits/M00-host-facts.md` and `codex/M00-NEXT-TASK.md`.
2. M00 graphics evidence exists; safe M01-A **repo-only original Rust window-manager core** is now permitted: windows/focus/manual move-resize/tiling/workspaces and tests. This is NOT a theme/configuration-first project. Do not launch/replace the current WM/compositor, change installed services, implement the GUI, installer, ISO or move beyond explicitly assigned M01-A.
3. **Approved coding execution is PC-local Codex CLI and AGY CLI only.** The target PC can supply genuine hardware evidence, but distinguish current host measurements from historical audit snapshots and any optional CI runners. Do not use Codex Cloud or Gemini CLI/Jules.
4. Intel HD 2000 Vulkan acceleration is not established: report showed **llvmpipe CPU Vulkan**.
5. **Newer real-host evidence confirms Intel HD Graphics 2000 accelerated OpenGL 3.3, GLES 3.0 and EGL 1.5.** See `docs/audits/M00-C-graphics-evidence-2026-11-13.md`. Custom compositor frame timing, RAM and blur remain unknown.
6. Treat unknown physical RAM/FPS/recovery measurements as deployment blockers, not reasons to postpone safe Rust source development. Finish assigned work with tests and stop; do not autonomously advance milestones.

## Permanent constraints
- Rust-first custom programs; non-Rust Linux dependencies allowed.
- <=500 MiB at **total system steady desktop idle** is a release acceptance gate, not a current result. Include animated wallpaper; exclude manually launched browser/Discord/music workloads and measure those separately.
- Target display Dell D1918H 1366×768, ~60 Hz; animation performance and GPU acceleration must be measured, never guessed.
- No Electron/Chromium dependency for the foundational shell; browser research is separate.
- All theme packages default to declarative data, no arbitrary shell execution.
- **In-place only:** preserve existing Arch session/login, packages, configs and rollback path. Never repartition, reinstall, replace active Hyprland, enable autostart, run sudo or change the host without separately explicit approval, backups and verified recovery.
- Never commit raw EDID, secrets, cookies, Wi-Fi identifiers, MAC/IP, usernames or private paths.
- Be honest about Discord API limitations, experimental Rust browser engines and Apple Music service authorization/DRM.
- Document licenses and upstream attribution. Do not pick a repository license unilaterally.
- New features need tests, performance checks, error recovery and accessibility considerations.

## Workflow
Follow `docs/AI-DESIGN-AND-IMPLEMENTATION-WORKFLOW.md`: ask owner for each new UI design first, send exactly one prompt to PC-local Codex CLI or AGY CLI, state model/mode based on actual availability, inspect returned evidence, then select only the next task. No Codex Cloud, Gemini CLI, Jules or simultaneous writers. One assignment at a time; obtain physical-host evidence before any actual desktop deployment. M01-A is source-only and allowed while unresolved M00 gates remain open. No claims of boot success, release readiness, 60 FPS or RAM compliance without reproducible physical evidence.
