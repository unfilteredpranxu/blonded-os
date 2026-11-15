# Blonded OS — Future Rust-first application and service migration policy

**Status:** Product requirement only. Do NOT execute during M00 or modify the user's current Arch desktop. This is a future phase after Blonded OS has an independent, bootable, tested, recoverable session.

## Goal

Make the old Intel Core i3-2100 / 4 GB RAM / mechanical HDD host feel **fast, smooth and visually rich** during real study, research, browsing and music multitasking. Use Rust for original Blonded components and favor Rust app replacements where they **measurably** improve cold startup, RAM/PSS, I/O, CPU usage, GUI responsiveness and functionality. The development goal is a full Arch-derived operating system, not just dotfiles or a themed Hyprland install.

**Rules:** A non-Rust app is not automatically heavyweight; a Rust app is not automatically lightweight. Do not replace mature software solely because of its language. Do not claim Rust eliminates bugs. Keep necessary Linux non-Rust kernel, driver, graphics, networking, security, audio and compatibility libraries.

## Two budgets

1. **Desktop steady-idle acceptance:** <=500 MiB **system-wide** total RAM usage with working custom shell, compositing, window management, essential services and an animated wallpaper, measured on target hardware. This remains unverified, and the budget must not rely on disabled essential functionality or active swapping.
2. **Real workload target:** browser research with several tabs/documents + music playback + chat + notes/terminal. Measure each workload separately and in combination: total RAM/PSS, swap/zram, CPU, storage I/O, latency, window/frame pacing, application startup, recovery, audio underruns and user-visible responsiveness. No artificial 500 MiB guarantee with these apps running.

## Replacement candidates and exceptions

- Terminal: evaluate Alacritty against current Kitty for startup/RAM/responsiveness. Both must be measured; never auto-replace user settings.
- File navigation: Yazi (Rust) and GUI alternative only if necessary.
- Text/code editing: Helix (Rust) plus optional usable graphical editor if required for study/dev.
- System diagnostics: bottom (Rust), procs (Rust), ripgrep/fd/eza/zoxide for CLI workflows where beneficial.
- Desktop shell, windows/compositor, wallpaper scenes, themes, launcher, settings, widgets, notifications and music-reactive effects: **original Rust-first Blonded components**.
- Music player: build native Rust UI/local player first; subscription Apple Music playback requires official service support and legal authorization, and may need supported web playback instead of claiming a direct native streaming client.
- Web browsing: Servo and other Rust engines are a **research track**, not proven replacements for a modern accessible, secure daily-study browser; retain a functional existing browser until feature and security parity is established.
- Discord: only authorized integration and supported clients/web APIs; preserve working messaging/voice/video. Do not substitute self-bots or violate service terms.
- Keep necessary base components (systemd, NetworkManager, PipeWire/WirePlumber, drivers, Xorg/Wayland infrastructure as selected, authentication, update/security services). Non-Rust underlying services are acceptable.

## Migration safety gates

1. Inventory actual current apps, autostarts, user services and packages; classify **needed**, **replaceable**, **optional**, **unknown**. Some Docker, proxy, VPN and remote-access services may be part of the user's engineering workflow.
2. Reproduce baseline cold-start, idle RAM/PSS and representative study+music multitasking workload. Record versions and config.
3. Evaluate one replacement at a time with identical workload; keep accessibility, audio, saved data, password/session management and security updates intact.
4. Preserve existing Arch/Hyprland session and a rollback path. Back up configuration and user data. Never uninstall packages, stop services, alter login or remove kernels without explicit user authorization and successful replacement testing.
5. Keep the lightweight UI theme engine adaptive: full/balanced/eco modes, selective redraw, pause hidden animations, avoid full-resolution video decode when a small texture or procedural animation works.
6. Report objective improvement or regression, and leave heavy/unsupported features optional. Security and functionality outrank cosmetic language purity.
7. Only offer removals after independent Blonded OS boots with user apps, audio, networking, input, desktop, recovery and backups proven.

## Future acceptance experience

- Fluid window switching while reading PDFs, solving study questions, taking notes, browsing references and playing music.
- Coherent visual mood themes (pixel ocean, ASCII noir, cozy night, joyful, music-reactive), adjustable per-window blur/opacity/decoration, moving wallpaper and animation.
- No audio stuttering, accidental app termination, swap thrash or multi-second freeze due to animation pressure.
- Functional daily-browser fallback if a Rust-only browser lacks required compatibility.

**Milestone guard:** Record this as a long-term requirement only. M00 remains active; do not auto-advance or implement a migration now.
