# Blonded OS

**Status: M00 host assessment partially complete; reversible, in-place Rust foundation work is next. No custom desktop is running yet.**

Blonded OS is an **original Rust-first operating-system experience built directly on the owner's existing Arch Linux installation**. Arch provides the Linux kernel, drivers, hardware, package manager, security and system services. Blonded OS will gradually introduce its own window management/compositing, shell, themes, desktop settings and native apps. **The owner explicitly rejected creating a new ISO, installer or separate OS installation.** This is not merely Hyprland dotfiles; it is a custom desktop/system layer developed and adopted in-place.

## Vision
An extremely configurable desktop whose windows, wallpapers, animations, music-reactive visuals and full visual identity change with the user's mood. Target themes include pixelated ocean waves, ASCII noir, cozy night, joyful and music worlds; these are examples rather than limits.

Every window should eventually support per-app and global controls for transparency (including distinct background opacity), blur, tint, rounded corners, border, shadow, focus, layout and animation. Themes must be live-switchable and safely extensible by AI-generated declarative manifests.

## Target hardware and constraints
Real-host audit dated 2026-11-12: Intel Core i3-2100 (2C/4T), 3.71 GiB RAM, Intel 2nd-generation graphics PCI ID 8086:0102 (i915), rotating SATA HDD, Dell D1918H 1366×768 @ 59.79 Hz. Currently Arch Linux + Hyprland. Hardware Vulkan was **not** found; the Vulkan device observed is software llvmpipe. Real physical-host probes have now verified Intel-accelerated OpenGL 3.3, OpenGL ES 3.0 and EGL 1.5. Custom compositor performance remains untested.

**Release acceptance goal:** <=500 MiB total system memory at steady desktop idle with a moving wallpaper and essential services active; at least three clean-login runs on the real host. Browser, Discord and music player are additional application loads and cannot honestly be promised within that ceiling. This is a target, **not a measured achievement**.

## What is Rust?
Original OS desktop components should be Rust-first. Linux kernel, drivers, audio stack, display system and other essential system dependencies will continue to use their existing languages. Rust is not automatically lightweight and does not eliminate logical bugs.

## Repository map
- `os/`: legacy distribution-planning placeholders; NOT active tasks. In-place session integration/recovery is the selected approach.
- `crates/`: Rust starter workspace and eventual desktop/system components
- `docs/`: architecture, roadmap, benchmarks, sanitized host audit
- `themes/`: declarative theme-format prototype
- `codex/`: historical read-only audit assignment; active coding uses local PC Codex CLI and AGY CLI
- `.github/workflows/`: Rust scaffold checks

## Start here
Read `AGENTS.md`, `docs/AI_TEAM_STATE.md`, `docs/IN-PLACE-ARCH-STRATEGY.md` and `docs/PERFORMANCE.md`. Work in the existing Arch user-owned repository. M00 hardware acceptance remains open; bounded, reversible Rust foundation development is permitted, not system deployment. Keep the working Hyprland session and rollback path fully intact. No ISO, installer, repartitioning or boot changes.

Official repository: https://github.com/unfilteredpranxu/blonded-os
