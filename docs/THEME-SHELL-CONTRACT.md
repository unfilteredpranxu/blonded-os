# Blonded OS — Complete desktop-profile, scene and control-center contract

**Status:** Approved product direction / future engineering specification. No implementation is authorized by this document while M00 physical-host feasibility remains active. This is an Arch-derived operating-system project's original Rust-first windowing and shell layer, **not** a Hyprland dotfile/theme project.

## 1. One OS; unlimited desktop identities

A **desktop profile** must be able to replace the entire *presentation and interaction model* of the system without replacing its kernel, application processes, user data or workspace contents. This is more powerful than a palette or wallpaper theme. Profiles are selectable on the fly; each can specify:

- **Screen layout:** top bar / bottom dock / side panels / floating widgets / multi-monitor configuration / edge margins / reserves, auto-hide and reveal zones.
- **Window system:** floating, tiling, stacking, snapping, workspace mapping, per-app placement, window chrome, title bar, borders, corners, shadows, animation and focus rules.
- **Materials:** whole-window opacity, separate background translucency, blur strength/radius, tints, contrast and legibility constraints (global and per-app overrides).
- **Scenery:** still, procedural animated, pixel/ASCII, sprite or video wallpapers; per-monitor variants, frame caps and adaptive animation behavior.
- **Iconography and typography:** scalable icons, fonts, shapes, button placement, spacing, density, cursors and motion language; independent of OS wallpaper.
- **Widgets:** time/date, workspace switcher, music now playing, audio/network indicators, study timer, calendar, CPU/RAM and dynamic visuals.
- **System UI:** notification center, application launcher, quick settings, session menu, lock-screen presentation, panels, settings and overlays.
- **Sound and interactivity:** user interface sound events and optional authorized music-reactive visualization with audio privacy safeguards.

Profiles may be macOS-inspired (independent implementation, no copyrighted proprietary assets distributed), Hyprland-rice-inspired, pixel ocean, ASCII noir, cozy night, joyful, music-reactive, or wholly original. **Do not assume existing Hyprland is required at runtime**: Blonded intends its own Rust-first window-management/compositor stack on an Arch Linux foundation.

## 2. Global keyboard controls (required)

| Shortcut | Action |
|---|---|
| **Super + T** (Windows key + T) | Toggle the **Theme Shell**: searchable profile gallery, live preview and quick apply, profile management, import/export |
| **Super + W** (Windows key + W) | Toggle **Wallpaper & Appearance Studio**: wallpaper choice/motion, color sync, global/per-window opacity, blur, motion presets, panel layout and effects |
| Escape | Close the open shell overlay without discarding the last applied theme |
| Keyboard navigation / Enter | Select, preview and apply a profile; fully usable without mouse |

Shortcuts must be registered in Blonded OS's **own** session dispatcher rather than relying on current Hyprland bindings; conflicts must be diagnosed and remappable by the user. A user can configure additional shortcuts later. Both toggles are intended to be low-latency, not open a new heavyweight process for each invocation.

### Theme Shell
- Presents visual previews that are **precomputed/cached** or efficient live previews where hardware permits; no simultaneous decoding/running of all full wallpaper scenes.
- Search, favorites, recent themes, duplicate/edit/save-as, restore defaults, import/export.
- Distinguish **Preview** (noncommittal) from **Apply** (atomic commit); cancel restores exactly the prior visual state.
- Apply must preserve open applications, virtual desktops, focus where safe, user data, windows and music.
- A failed theme or graphics effect must automatically restore the previous known-good profile.
- Presets may optionally specify per-monitor layouts and ultrawide/portrait adaptations without breaking a 1366×768 display.

### Wallpaper & Appearance Studio
- Wallpaper library with static, procedural, low-res sprite, animated and user-provided sources.
- Wallpaper and UI colors may synchronize automatically, with rate-limited representative color sampling to prevent flicker; users may override colors.
- Per-profile settings and global fallbacks for panel position, dock, widget arrangement, window materials and animation.
- Live before/after, custom themes, legibility checker, reduced-motion mode and reset controls.
- Opening Studio does **not** permanently start a second renderer or a web view.

## 3. Example complete profiles (design examples only)

**Mac-like**
- Translucent compact top menu bar, optionally centered or bottom application dock, mac-style spacing and smooth minimize animations.
- Menus, workspaces and window controls designed by Blonded OS; no Apple proprietary assets, code or exact trademark trade dress copied.
- Optional rounded window chrome, wallpaper color tint, trackpad-friendly interactions.

**Feature-rich shell / Hyprland-inspired**
- Rich top or edge bar, workspace pills, network/audio/resource widgets, quick settings, floating dashboard, animated workspace movement.
- Tiling + floating hybrid, snap/resize rules, dynamic panel and accent effects.
- Independent Rust shell; not a dependency on actual Hyprland.

**Pixel Ocean**
- Procedural pixel waves, frame-capped at small internal resolution, pixel UI with configurable retro borders and small dock.

**Cozy Night**
- Warm colors, atmospheric moving wallpaper, calm window transitions, subtle translucent materials.

A profile can specify **zero, one, or multiple bars**, a dock at any edge, edge auto-hide, and theme-specific widget placement. One profile's bars/panels must be unloaded or reused when another profile becomes active.

## 4. Proposed Rust-first runtime contract

The architecture is provisional until Intel HD Graphics 2000 GPU/renderer and memory/latency feasibility is benchmarked:

- `blonded-session`: boots and supervises Blonded's shell components; user session and safe recovery.
- `blonded-wm` / `blonded-compositor`: window control, compositing, materials and animations (candidate X11 or Wayland path under review).
- `blonded-shell`: lean top/bottom/side panel surfaces, dock, launcher, widgets, notifications, quick settings and overlays.
- `blonded-theme`: schema/versioned manifests, profile validation, loaded-state management, hot reload and atomic rollback.
- `blonded-scene`: GPU-assisted or CPU-fallback wallpaper and procedural animation scheduling.
- `blonded-appearance`: configuration UI surfaced by Super+T and Super+W; **not a browser/Electron runtime**.
- `blonded-bridge`: bounded IPC/events for workspace, audio, wallpaper palette and service status; protect privacy and avoid polling storms.

Prefer one event-driven shell process with shared state and bounded resources over starting a separate daemon per widget or per theme. Build only necessary services and wake primarily on actual changes. Reuse textures and fonts, apply bounded caches, avoid inactive scene rendering; handle multi-monitor configs without per-frame app process restarts.

**Critical distinction:** a compositor controls whole-window opacity and behind-window blur, but cannot universally repaint *inside* arbitrary third-party application UI. True background-only translucency depends on content and rendering cooperation; whole-window opacity fades text too. Never advertise universal internal application styling.

## 5. Theme manifest conceptual schema (NOT active code)

```toml
schema_version = 1
id = "mac-like"
name = "Mac-like"
extends = "base-accessible"

[layout]
top_bar = true
bottom_dock = true
left_panel = false
right_panel = false
dock_edge = "bottom"
dock_auto_hide = false

[windows]
layout = "hybrid"
window_opacity_percent = 100
background_opacity_percent = 84
blur_strength_percent = 28
corner_radius_px = 12

[wallpaper]
kind = "procedural"
scene = "gentle-waves"
preferred_fps = 30
palette_sync = true

[animation]
profile = "balanced"
transition_ms = 220

[widgets]
top = ["workspace", "clock", "network", "sound"]
dock = ["favorites", "running-apps"]
```

This is a **design proposal**. Implementers must validate all fields, paths, asset sizes, versioning and performance before use; do not execute arbitrary code in untrusted themes. User customizations are layered on top of defaults without destroying stock profiles.

## 6. Performance and reliability gates

- **<=500 MiB total physical system memory at steady desktop idle** with essential services, wallpaper animation, windowing and visible default shell. This remains a *target*, not a measured result. Heavy browser, chat and music workloads are separately budgeted.
- The physical Intel i3-2100 / 4 GB RAM / HDD / Intel HD Graphics 2000 @ 1366×768 ~59.79 Hz is the acceptance host.
- Before implementation, measure necessary GPU OpenGL/GLES features; hardware accelerated GLX and multiple Intel EGL context platforms are confirmed, but custom compositing / effects performance is unverified.
- Target responsiveness and smooth presentation; 60 FPS is aspirational for lightweight effects, not a blanket guarantee. Each mode needs meaningful frame-time, CPU, RAM, I/O and input-latency tests.
- Hot switching must not restart applications, reset study material, interrupt playback or hang the desktop; rollback immediately on invalid profiles.
- Minimize work when overlays are closed, windows are hidden or system resources are constrained. Music/volume widgets subscribe to events instead of continually scanning audio.
- All essential functionality and accessibility controls survive profiles and safe mode; support keyboard-only use and reduced motion.
- Avoid unnecessary dependencies on Chromium, Electron, JS runtimes or dozens of always-running panel processes for essential OS surfaces.

## 7. Future validation checklist

1. Super+T overlay opens with a keyboard-only navigable profile gallery, then closes, without persistent excess CPU/worker usage.
2. Super+W exposes current wallpaper, live scene controls, panel layout and window materials; Esc restores previous values when editing a preview.
3. Mac-like profile displays top bar + bottom dock while feature-rich profile displays a different arrangement and widget set; only one active profile's render surfaces remain.
4. Switching themes retains all windows, workspaces, file operations, notes and media playback.
5. On failure, theme reverts without logging out or requiring desktop recovery.
6. Native test on Intel HD 2000 verifies blur/opacity/animation fallback and frame pacing at 1366×768.
7. Three reproducible clean-logins pass the <=500 MiB total-system steady-idle protocol or the release gate stays blocked.

**Milestone boundary:** All of this is a future product requirement, not permission to implement it in M00. The existing OS audit and its outstanding recovery/performance issues remain active.
