# Proposed Blonded OS architecture (not finalized)

1. **Linux foundation:** kernel, firmware, drivers, systemd, filesystems, networking, ALSA/PipeWire and packaging, as shipped by Arch-compatible infrastructure.
2. **Distribution layer:** reproducible ArchISO profile, signed/installable packages, safe update strategy, installer and recovery path.
3. **Rust windowing:** candidate X11-based custom WM via Penrose/x11rb and optional Rust compositor; compare Smithay Wayland route only after actual GPU capability tests.
4. **Rust desktop shell:** session, dock, launcher, notifications, input controls, settings, workspace management and service supervision.
5. **Rust scene/theme runtime:** animated low-resolution wallpapers, opacity, background blur, borders, shadows, window motion, per-app overrides, reactive music displays and safe theme packages.
6. **User apps:** native Rust wherever feasible, with compatibility/research tracks for Servo browser, Discord, Apple Music.

## Engineering gates
- Real Intel HD 2000 OpenGL/EGL capabilities and accelerated renderer remain unverified.
- Vulkan report showed software llvmpipe; **do not** select Vulkan because the loader is present.
- X11 vs Wayland is an open engineering decision, not a brand promise.
- Background-only blur requires a compositor and suitable render path; a window manager alone is insufficient.
- Test window FPS/p95/p99 frame times, CPU, unique memory/PSS and HDD I/O on real hardware.
- Require an always-available fallback TTY and previous Arch/Hyprland session until recovery is proven.
- Declarative themes may not execute arbitrary code by default.

No ISO, compositor, installer or desktop session has been implemented or validated.
