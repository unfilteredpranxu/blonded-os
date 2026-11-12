# AI engineering rules — Blonded OS

## Scope
Blonded OS is an operating-system **distribution** engineering project, not a reskin of an existing desktop. Ultimate deliverable: bootable Arch-derived ISO, installed OS, package/update strategy, recovery, and a Rust-first custom desktop stack.

## Current milestone: M00 ONLY
1. Read `docs/audits/M00-host-facts.md` and `codex/M00-NEXT-TASK.md`.
2. Investigate hardware/rendering/memory feasibility and missing evidence. Do not start WM, compositor, installer, ISO or theme implementation yet.
3. Codex Cloud environment is NOT the real physical target PC. Do not use cloud GPU/CPU info as physical-host evidence.
4. Intel HD 2000 Vulkan acceleration is not established: report showed **llvmpipe CPU Vulkan**.
5. Missing `glxinfo` and `eglinfo` are missing measurements, not proof hardware OpenGL is absent.
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
One Codex assignment at a time; ask for physical-host evidence before deciding the next milestone. No claims of boot success, release readiness, 60 FPS or RAM compliance without reproducible physical evidence.
