# Blonded OS — sequential in-place Arch roadmap

**Strategy decision (2026-11-15):** Owner explicitly rejected a new ISO/installer. Build the Rust-first Blonded desktop **on the existing Arch installation**, then gradually adopt it as a selectable/replaceable desktop session after non-destructive testing. Existing Arch kernel/drivers/pacman remain the system base. Preserve Hyprland and user files.

- **M00 — PARTIAL / OPEN:** real physical GPU acceleration confirmed (Intel OpenGL 3.3/GLES 3.0/EGL 1.5). Clean system memory, fallbacks and frame benchmarks remain unverified. These gate **deployment**, but don't prevent coding isolated Rust tools.
- **M01-A — COMPLETED / REVIEWED:** original Rust window-management state machine, 25 physical-host unit/integration tests PASS; independently reviewed by AGY. Integrated commit `b3e754ab91ed7b0fff0634aed7245c78f8c11a41`, PR #4; no live window rendering yet.
- **M01-B — NEXT:** connect original WM core to a **minimal nested Rust Wayland compositor**, studying Smithay smallvil and winit nested backend, and demonstrate at least one real Wayland client window in a child window of existing Hyprland; isolate socket and input, avoid Vulkan dependency and global session changes. Focus/move/resize where verifiable. Issue #5.
- **M02:** extend nested compositor/surface protocol correctness, window lifecycle and input ergonomics; measure Intel GL/GLES rendering feasibility, memory and hardware capability. No host session replacement.
- **M03:** stable native Rust window manager/compositor vertical slice with working manual floating/tiling/snapping/workspaces; only after nested tests, consider reversible optional session integration. Keep Hyprland fallback.
- **M04:** efficient compositing, opacity, blur, shadows, animation timing, input stability, safe fallback.
- **M05:** configurable Rust shell: bars, docks, app launcher, widgets, notifications, settings, keyboard dispatch.
- **M06:** Rust animated wallpaper/scene engine with low-resolution adaptive effects.
- **M07:** full desktop profiles and native panels: Super+T Theme Shell, Super+W Wallpaper/Desktop Studio, Super+O Window Effects Studio.
- **M08:** optional user-local session integration on existing Arch, audio/network/portal compatibility, easy rollback and recovery; elevated system modifications require separately explicit approval.
- **M09:** Rust-first applications and real-world study/music multitasking, removing only measured unnecessary software after backups and compatibility.
- **M10-M12:** secure browser fallback research, Discord integration and authorized music capabilities as separate tracks.
- **M13:** physical-host acceptance (<=500 MiB steady desktop idle target, performance, three fresh session tests), recovery reliability and in-place release strategy.

**Out of scope by owner request:** ArchISO, new partition, reinstall, standalone OS installer, bootable image as the primary deliverable. Legacy `os/archiso` and `os/installer` placeholder folders must not be treated as current work.

Every milestone needs user review of actual CLI agent output and tests. Never auto-advance.
