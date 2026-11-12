# Blonded OS

**Status: M00 hardware verification. No bootable ISO or functional custom desktop exists yet.**

Blonded OS is an **Arch Linux-derived OS distribution project** with an original Rust-first desktop stack. The goal is a reproducible bootable installation image, packages, updates, recovery, and a custom desktop environment — **not** merely a theme, dotfiles collection, or standalone GUI app.

## Vision
An extremely configurable desktop whose windows, wallpapers, animations, music-reactive visuals and full visual identity change with the user's mood. Target themes include pixelated ocean waves, ASCII noir, cozy night, joyful and music worlds; these are examples rather than limits.

Every window should eventually support per-app and global controls for transparency (including distinct background opacity), blur, tint, rounded corners, border, shadow, focus, layout and animation. Themes must be live-switchable and safely extensible by AI-generated declarative manifests.

## Target hardware and constraints
Real-host audit dated 2026-11-12: Intel Core i3-2100 (2C/4T), 3.71 GiB RAM, Intel 2nd-generation graphics PCI ID 8086:0102 (i915), rotating SATA HDD, Dell D1918H 1366×768 @ 59.79 Hz. Currently Arch Linux + Hyprland. Hardware Vulkan was **not** found; the Vulkan device observed is software llvmpipe. OpenGL/EGL hardware acceleration remains unverified.

**Release acceptance goal:** <=500 MiB total system memory at steady desktop idle with a moving wallpaper and essential services active; at least three clean-login runs on the real host. Browser, Discord and music player are additional application loads and cannot honestly be promised within that ceiling. This is a target, **not a measured achievement**.

## What is Rust?
Original OS desktop components should be Rust-first. Linux kernel, drivers, audio stack, display system and other essential system dependencies will continue to use their existing languages. Rust is not automatically lightweight and does not eliminate logical bugs.

## Repository map
- `os/`: future ArchISO profile, installer, packages, updates and recovery
- `crates/`: Rust starter workspace and eventual desktop/system components
- `docs/`: architecture, roadmap, benchmarks, sanitized host audit
- `themes/`: declarative theme-format prototype
- `codex/`: one-at-a-time Codex Cloud assignments
- `.github/workflows/`: Rust scaffold checks

## Start here
Read `AGENTS.md`, `docs/audits/M00-host-facts.md`, `docs/PERFORMANCE.md` and `codex/M00-NEXT-TASK.md`. **Work on M00 only.** Codex Cloud hardware is not the real host. Protect the user's existing working Arch/Hyprland session; do not modify disks or login configuration.

Official repository: https://github.com/unfilteredpranxu/blonded-os
