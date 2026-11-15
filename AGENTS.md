# AI engineering rules — Blonded OS

## Required cross-agent startup (Codex CLI and AGY CLI)

**Hands-off policy:** Read `docs/HANDS-OFF-CLI-AUTONOMY.md`. Owner wants no routine permission interruptions: execute scoped repository edits/builds/tests autonomously using protected sandbox modes; if an action is denied, skip and record `BLOCKED`. Never bypass sandboxes, install/remove packages or modify live host services/disks on your own.

Read `docs/AI_TEAM_STATE.md` **first**, then this file, then `docs/AI-TEAM-PROTOCOL.md` only if this is a new task or handoff. Confirm current branch/HEAD and `git status --short`; only inspect assigned issue and task-relevant files. The shared checkpoint + Git history, not prior chat transcripts, are authoritative records of what the other AI did. Do not run broad repository re-audits or print full logs just to build context. After an assigned task, checkpoint the outcome with exact commits/tests and STOP for ChatGPT/owner review. Do not work on `main` assuming unmerged audit-branch docs exist there.

## Scope
Blonded OS is an operating-system **distribution** engineering project, not a reskin of an existing desktop. Ultimate deliverable: bootable Arch-derived ISO, installed OS, package/update strategy, recovery, and a Rust-first custom desktop stack.

## Current milestone: M00 ONLY
1. Read `docs/audits/M00-host-facts.md` and `codex/M00-NEXT-TASK.md`.
2. Investigate hardware/rendering/memory feasibility and missing evidence. Do not start WM, compositor, installer, ISO or theme implementation yet.
3. **Approved coding execution is PC-local Codex CLI and AGY CLI only.** The target PC can supply genuine hardware evidence, but distinguish current host measurements from historical audit snapshots and any optional CI runners. Do not use Codex Cloud or Gemini CLI/Jules.
4. Intel HD 2000 Vulkan acceleration is not established: report showed **llvmpipe CPU Vulkan**.
5. **Newer real-host evidence confirms Intel HD Graphics 2000 accelerated OpenGL 3.3, GLES 3.0 and EGL 1.5.** See `docs/audits/M00-C-graphics-evidence-2026-11-13.md`. Custom compositor frame timing, RAM and blur remain unknown.
6. Produce VERIFIED / INFERRED / UNKNOWN findings. Finish with evidence and stop. Do not autonomously advance milestones.

## Permanent constraints
- Rust-first custom programs; non-Rust Linux dependencies allowed.
- <=500 MiB at **total system steady desktop idle** is a release acceptance gate, not a current result. Include animated wallpaper; exclude manually launched browser/Discord/music workloads and measure those separately.
- Target display Dell D1918H 1366×768, ~60 Hz; animation performance and GPU acceleration must be measured, never guessed.
- No Electron/Chromium dependency for the foundational shell; browser research is separate.
- All theme packages default to declarative data, no arbitrary shell execution.
- Preserve existing Arch session/login and rollback path. Never repartition or replace the host without explicit approval, backups and verified recovery.
- Never commit raw EDID, secrets, cookies, Wi-Fi identifiers, MAC/IP, usernames or private paths.
- Be honest about Discord API limitations, experimental Rust browser engines and Apple Music service authorization/DRM.
- Document licenses and upstream attribution. Do not pick a repository license unilaterally.
- New features need tests, performance checks, error recovery and accessibility considerations.

## Workflow
Follow `docs/AI-DESIGN-AND-IMPLEMENTATION-WORKFLOW.md`: ask owner for each new UI design first, send exactly one prompt to PC-local Codex CLI or AGY CLI, state model/mode based on actual availability, inspect returned evidence, then select only the next task. No Codex Cloud, Gemini CLI, Jules or simultaneous writers. One assignment at a time; ask for physical-host evidence before deciding the next milestone. No claims of boot success, release readiness, 60 FPS or RAM compliance without reproducible physical evidence.
