# Blonded OS — In-place Arch strategy (owner decision, 2026-11-15)

## The product

The owner explicitly rejected creating a new ISO, installer, partition or separate operating-system installation. **Blonded OS will be developed directly on the existing working Arch Linux system**: original Rust-first window management and compositor, theme engine, bars, dock, wallpapers, widgets, system controls, native applications and user experience; Arch/Linux remains the kernel, driver, hardware and package infrastructure.

Do not confuse "use existing system" with "keep Hyprland forever." Hyprland is a **temporary known-good fallback** while developing an independent Blonded runtime. Test new components in isolation before ever offering an optional session entry or replacing existing user workflow.

## Non-destructive adoption phases

1. **Repository-only Rust cores:** build, test and benchmark theme/profile/config models, event handling and settings logic in user-owned repo. Never write to current Hyprland config. No sudo, pacman changes, autostart or login modifications.
2. **Manual user-space prototype:** when authorized, launch Blonded binaries only on user request from existing session or a nested test environment; stop cleanly without affecting unrelated applications.
3. **Isolated custom WM/compositor prototype:** select supported backend from measured Intel HD 2000 OpenGL/GLES features; test under existing session if feasible or with explicit safe fallback in a separate, reversible session.
4. **Optional selectable Blonded session:** after recovery/input/display validation, add a login/session choice with an independent rollback procedure. Installing packages/services requires *separate explicit approval* for the specific action, not blanket CLI automation.
5. **Gradual application migration:** benchmark Rust alternatives first; preserve study/music/browser/Discord workflows; no removals until replacement works and backups exist.

## M00 / M01 boundaries

M00 physical checks established Intel-accelerated GL 3.3, GLES 3.0 and EGL 1.5. Critical Docker/network userspace boot delays are identified. Remaining clean desktop idle/FPS/recovery gates are **not fulfilled**. This prevents claims of ready-to-use replacement desktop or deployment. However, owner now authorized starting **isolated Rust source development**.

M01-A is the first actual implementation: **functional, tested Rust profile/settings engine** for future theme hot swapping, entirely within repo. No UI required yet, so no separate UI style question. This task MUST NOT run the custom compositor, modify an existing desktop, install packages, enable autostart, or rewrite login configuration. It must not pretend profile activation currently changes on-screen desktop.

## Platform constraints

Intel i3-2100, ~4 GB RAM, Intel HD 2000 OpenGL 3.3/GLES 3.0/EGL 1.5, no verified hardware Vulkan, spinning HDD, Dell 1366x768@59.79 Hz. Idle goal <=500 MiB system-wide with animated wallpaper and necessary services is an **unverified acceptance target**. Prioritize bounded allocations, no Electron/Chromium shell, event-driven state, low I/O, fast safe restarts.

## UX requirement ownership

Ask the owner how each new visual feature should look before implementing UI. Already specified shortcuts: Super+T full theme shell, Super+W wallpaper/desktop settings, Super+O window opacity/blur/material settings. The Rust config core may define data structures supporting them without choosing a final screen design.

## Shared AI team

The local Codex CLI is the primary writer. AGY CLI reviews independently when specifically assigned. All prompts and changes are one step at a time, use `AGENTS.md`, `docs/AI_TEAM_STATE.md`, `docs/AI-TEAM-PROTOCOL.md`, preserve active Git state and hand off concise evidence with full SHA and tests.

**Explicitly NOT a current deliverable:** bootable ArchISO, standalone installer, replacement OS image, any new partition, reinstalling Arch, deleting existing Hyprland/other user packages, or upgrading the current system automatically.
