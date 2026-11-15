# Blonded OS — Master 100%-customization development blueprint

> **STRATEGY OVERRIDE (2026-11-15):** Owner rejected a new ISO/reinstallation. Historical ISO/installer roadmap items below are superseded by the ACTIVE `docs/ROADMAP.md` and `docs/IN-PLACE-ARCH-STRATEGY.md`. Build native Rust desktop/system components on current Arch, keep Hyprland safe and use reversible user-space integration. Do not implement any ISO, installer or separate OS image.

**Status:** Long-term architectural blueprint, **NOT** authorization to implement stages beyond M00. The official sequential milestone assignments are in `docs/ROADMAP.md`; this document expands them without skipping stage gates.

**Owner's objective:** Arch Linux is the **foundation only**. The project becomes its own bootable, installable, recoverable distribution with a Rust-first original window manager/compositor, shell, bars, wallpapers, widgets, themes, settings, animation, study/music multitasking and optional Rust-first user applications. An imported Hyprland rice/dotfile collection is not an acceptable end product.

## Target / constraints

- Actual development target: Intel i3-2100 2C/4T, ~3794 MiB visible RAM, Intel HD Graphics 2000 SNB GT1 PCI 8086:0102, 1366x768@59.79 Hz Dell D1918H, spinning SATA HDD.
- Physical GPU probes **verified accelerated Intel OpenGL 3.3**, OpenGL ES 3.0 and EGL 1.5. Earlier Vulkan device was software llvmpipe; no hardware Vulkan established. An application dependent on Vulkan, including Zed, is not a default target tool for this device.
- **<=500 MiB total system steady desktop idle**, including animated wallpaper and essential window/system services, is an unverified future release acceptance constraint; full browser + music + chat + PDF multitasking is measured separately.
- No hidden process farms, Electron/Chromium for OS shell, unconditional per-frame polling of widgets, background video decoding by default, or mandatory GPU effects that prevent login on low hardware.
- Linux kernel, firmware, drivers, systemd, audio and some third-party user programs are not Rust. "Rust first" means original Blonded OS programs. The implementation language must never outrank usability, safety or actual benchmark data.
- Uninstall/migrate old applications only after backup, independent working Blonded session, rollback and objective tests.

## Complete user-facing customization matrix

| Subsystem | Required configuration |
|---|---|
| Whole desktop | Live switch full desktop **profiles**: Mac-like (independent reimplementation), feature-rich Hyprland-inspired, Pixel Ocean, cozy, ASCII, custom user-authored |
| Display | Per-monitor resolution/scaling/refresh/rotation, wallpaper/workspace mapping, panel edge and safe layout margins |
| Top/bottom/side bars | Number, edge, per-monitor presence, visibility/auto-hide, spacing, transparency, section layout, app menus and status indicators |
| Docks | Edge, alignment, size/magnification, pinned apps, running indicators, animations, reveal/hide, profile-specific appearance |
| Window control | Floating, tiling, maximized, stacking, snapping, gestures, workspaces, layouts, rules, focus and border behavior |
| Materials | Whole-window opacity, cooperative background-only opacity, behind-window blur, tint, contrast, color, shadows, radius, strokes, focus states |
| Animations | Window open/close/minimize, workspace transitions, dock, notifications, widget motion, curve/speed/duration, reduce-motion adaptation |
| Wallpaper / scene | Animated procedural pixel waves, ASCII rain, video/static/sprite-based backdrops, per-monitor scenes, resource caps, palette extraction |
| Widgets | Music controls, timer, study Pomodoro/clock, weather only if enabled, system indicators, calendar, notes, workspaces, CPU/RAM |
| Theme assets | Fonts, icons, cursor, sounds, palette, backgrounds, control density and optional accessibility override layers |
| System UI | Login, lock, desktop panel, launcher, quick settings, notifications, shutdown controls, file dialogs/portals, network/audio controls |
| Keybindings | Editable combinations, profile-specific bindings, conflict checks, export/import and keyboard navigation |
| Accessibility | Text scaling, high contrast, keyboard-only controls, reduced motion, screen-reader interoperability where achievable |
| Installer and OS | Blonded branding, login entry, preconfigured packages, reproducible ISO, backups, upgrades/recovery |
| Third-party applications | Compositor appearance only where technically possible; do **not** promise that arbitrary applications' internal UI is restylable |

**Required shortcuts**: Super+T opens Theme Shell, Super+W opens Wallpaper & Desktop Studio, Super+O opens Window Effects Studio. Controls must be low-overhead native Rust overlays (not spawning a heavy standalone app each time). The detailed behavior and safety contract is in `docs/THEME-SHELL-CONTRACT.md`.

### Theme engine design

- Versioned declarative `theme.toml` + `profile.toml` layers; separate `core defaults -> theme defaults -> user global preferences -> per-app rules -> current window temporary edits`, with precise precedence and reset controls.
- Live reconfiguration of bars, docks, widget sets, workspaces, wallpaper, decorations and animation while retaining app processes and study/music work.
- Atomic preview/apply/cancel with hot-swap validation, bounded texture caches, resource ownership and rollback to last-known-good profile.
- Live themed appearance does **not** permit execution of untrusted theme scripts by default. Constrain assets, CPU/memory/render budgets, and validate schema migrations.
- One shared event-driven Rust shell/runtime; dormant themes and scenes must not continuously consume frame/render work. Use software/reduced effects modes on old GPUs.
- Include a GUI theme builder and import/export, but keep the configuration hand-editable for AI-assisted authorship.

## Milestone sequence (one Codex assignment at a time)

### M00 — Hardware, feasibility and recovery **ACTIVE**
- Preserve proven real-host GPU findings and boot bottleneck measurements in redacted docs.
- Complete honest M00 physical readiness: clean existing-session baseline, latency/memory pressure, backend capability gaps, fallback login and measured issues.
- Select no irreversible compositor strategy until evidence and user review. This blueprint does not move M00 forward by itself.

### M01 — Distribution architecture
- Define ArchISO build scripts, target packages/PKGBUILDs, session design, Rust workspace boundaries, CI matrix, source/provenance/license policy, security model, upgrade and recovery design.
- Build in disposable VM/runner where possible; do not replace existing Arch login.

### M02 — Independent Blonded OS live foundation
- Reproducible bootable live ISO, shell/TTY recovery, network/audio basics, input and persistent logs, software-rendering fallback and real hardware smoke test.
- Temporary stock desktop or console may be used only as a test scaffold; no claim of final Blonded desktop.

### M03 — Native Rust window-system vertical slice
- Implement minimal focus, move, resize, maximize/minimize, workspace and input handling on a chosen tested X11/Wayland backend.
- Window rules, cleanup, crash recovery, multi-window test; maintain Hyprland fallback.

### M04 — Graphics/compositor and performance harness
- Opacity, textures, clipping, synchronized presentation, compositing correctness, blur prototype, damage tracking, animation frame pacing.
- Test Intel HD 2000 (GL3.3/GLES3/EGL1.5); implement controlled no-blur, low-resolution, reduced-animation and software-safe recovery modes.
- Never promise 60FPS or full transparency for every third-party app.

### M05 — Rust Blonded shell
- Shared native process for dock, configurable bars/panels, app launcher, workspaces, tray or status indicators, notifications, quick settings, keyboard dispatcher, session controls.
- Accessibility, keyboard navigation, persistent state, error recovery.

### M06 — Rust wallpaper/scene engine
- Static/image/procedural pixel ocean/ASCII rain scenes; lazy asset loading, motion profiles, synchronized colors, audio optional.
- Reuse buffers, suspend offscreen animations, compile shaders ahead when feasible; benchmark p95/p99 and wall power/CPU.

### M07 — Live desktop-profile engine and appearance studios
- Versioned manifest/validation, GUI editor, history, import/export, rollback, per-window rules, profile hot-swaps.
- **Super+T Theme Shell**, **Super+W Wallpaper & Desktop Studio**, **Super+O Window Effects Studio** with per-app/global/theme scopes.
- Tests: switch themes without restarting apps, overlays open/close without retaining heavyweight resources.

### M08 — Desktop integration and recovery
- Audio PipeWire controls, networking, notifications, portals, screen capture/sharing if supported, power/lock, installer and update paths.
- Separate permissions, updates, rollback and privacy tests.

### M09 — Native Rust utility suite + baseline user workflows
- Terminal comparison (Alacritty/Kitty), file manager (Yazi or usable GUI option), Helix, clipboard, screenshot, app search and light monitoring.
- Build "Study" and "Music" workspace workflows: reading PDF + notes + browser references + playback, and no audio underruns.

### M10 — Browser compatibility research
- Evaluate Servo/Rust engines realistically; retain a secure, fully functional fallback browser until browsing, PDF, logins, video and extension requirements pass.
- Don't substitute a broken browser solely to achieve a Rust-only label.

### M11 — Communication/Discord compatibility
- Use supported official APIs/web/clients. Preserve text, voice and other features user needs; no unauthorized self-bots.

### M12 — Music experience
- Rust-native local music UI and playlist/control widget.
- Apple Music only through permitted official integration or supported web client where needed; account, DRM and platform limitations documented.

### M13 — Performance acceptance + release
- Three comparable clean logins with **whole-system <=500 MiB** at steady desktop idle, animated wallpaper and essential services running; if gate fails, don't label the release compliant.
- Cold boot / login ready times, mouse/keyboard latency, animations p95/p99, CPU, IO wait, zram/swap, memory/PSS and disk behavior. Benchmark same image/config on the physical host.
- Extra test: PDF/notes/browser/music simultaneously, multiple workspaces and themes, under sustained disk I/O; preserve reliable audio and app states.
- Sign, document, publish and recovery-test the installer/ISO, update and backup paths.

Each stage has formal acceptance tests, explicit risks, specific modified files, a branch per change and an owner review. Never skip to the next stage because a model claims success.

## AI engineering with existing subscriptions — PC LOCAL CLI ONLY

**Canonical source:** GitHub repository `unfilteredpranxu/blonded-os`. User has ChatGPT Plus and Google AI Pro via Jio, but available models, login authorization, limits and billing for either CLI must be verified in the actual tools.

**Exactly two coding-agent entry points on the Arch PC:**

1. **Codex CLI (`codex`), primary:** one scoped Rust/OS implementation, testing or documentation assignment per handoff. High reasoning if selectable; otherwise use the strongest actually offered option. Avoid simultaneous conflicting edits. No Codex Cloud.
2. **Google Antigravity CLI (`agy`), secondary:** read-only architecture/security/performance audit or an explicitly assigned independent task. Use a high-thinking model when available for complex analysis and an available fast model for routine audits. No Gemini CLI, Jules, Code Assist or Antigravity graphical editor.
3. **ChatGPT (this conversation), orchestration only:** ask owner for each component's visual design before writing implementation prompts, choose Codex/AGY and available mode, prepare one paste-ready task, then review actual terminal logs and GitHub changes supplied by user before issuing any further task.

**Shared state and reproducibility:** GitHub issue/branch, baseline commit SHA, verified tests and project requirements in `AGENTS.md` and `docs/AI-DESIGN-AND-IMPLEMENTATION-WORKFLOW.md`. One implementing agent writes per branch, the other audits read-only. Prompt must state exact authorization and stop conditions; never silently merge or promote a milestone.

**Physical resource limits:** inference may be remote but the CLI terminal tools and local builds consume RAM/CPU on an i3-2100/4 GB/HDD system. Prefer staged builds, conservative Rust compilation parallelism, and optional GitHub Actions build/test runners while keeping actual target-device acceptance on the PC. Never turn hosted CI into an unrelated coding agent.

**No user hand-coding required.** Only copy provided CLI prompts, return summaries/evidence and give explicit approval for sensitive system operations. Do not paste raw private audit transcripts/credentials into public GitHub.

## Rust-first editor choice for the physical PC

- **Helix** is the safest fast local editor: Rust terminal editor with LSP/Tree-sitter; highly compatible with older integrated graphics because it uses terminal rendering.
- **Lapce** is the closest native Rust GUI alternative to VS Code, but it uses Floem/wgpu and may fail or use software graphics on old Intel without Vulkan. Test startup, GPU selection, latency, RAM, CPU and text editing before adopting. This is a *trial*, not a guaranteed speed upgrade.
- **Zed** depends on Vulkan for effective GUI rendering on Linux; given no verified Intel hardware Vulkan, exclude from default workstation setup.
- Main code is implemented by the approved PC-local Codex CLI or AGY CLI with GitHub commits; the user should not be required to hand-code.

## Non-negotiable safety promises

Do not alter the active physical Arch/Hyprland login, bootloader, partitions or existing user apps without explicit approval and proven independent recovery. No claims of full 100% third-party window styling, 100% Rust underlying Linux, constant 60 FPS on HD 2000, unlimited AI quota or fully working ISO unless independently verified.
