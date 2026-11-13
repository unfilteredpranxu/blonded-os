# M00-C — Sanitized Intel graphics evidence (2026-11-13)

**Source:** user-executed diagnostics in the original Arch Linux Wayland/Hyprland desktop, after a package update. Original raw terminal logs are intentionally NOT committed. This is a physical-host observation, not Codex Cloud hardware.

## Context / changes since the 2026-11-12 report

- `mesa-utils` is installed and provides `eglinfo` and `glxinfo`.
- Current installed Mesa became `26.2.7-arch1.1` after the full pacman system update.
- The update also installed Linux kernel package `7.3.4.arch1-1`; **the currently running kernel was not measured after this update**, and no reboot is claimed.
- `xorg-xwayland` 24.1.15-1 is installed; `xorg-server` was not installed when checked.
- Previous `mesa` 26.2.6 and `llvmpipe` Vulkan observations were from before this update.

## Verified results

| Probe | Physical host observation | Scope/caveat |
|---|---|---|
| `eglinfo -B` GBM | Mesa EGL API 1.5; Intel HD Graphics 2000 SNB GT1; OpenGL 3.3 core/compat; OpenGL ES 3.0; GLSL 3.30 / ES GLSL 3.00 | EGL Intel contexts can be created; not a compositor or FPS benchmark. |
| `eglinfo -B` Wayland | Mesa EGL 1.5, Intel HD Graphics 2000, OpenGL 3.3 and GLES 3.0 | Demonstrates usable EGL Wayland context; not direct compositor driver/buffer proof. |
| `eglinfo -B` X11 | Mesa EGL 1.5, Intel HD Graphics 2000, OpenGL 3.3 and GLES 3.0 | Existing Wayland session includes XWayland; **not** native Xorg validation. |
| `eglinfo -B` surfaceless | Mesa EGL 1.5, Intel HD Graphics 2000, OpenGL 3.3 and GLES 3.0 | Headless/surfaceless context validation only. |
| `eglinfo -B` device platform | Device #0 Intel HD Graphics 2000; Device #1 software llvmpipe OpenGL 4.6 / GLES 3.2 | llvmpipe is an **additional CPU software device**, not the active GLX renderer. |
| `glxinfo -B` | Direct rendering Yes; vendor Intel (0x8086); device Intel HD Graphics 2000 SNB GT1 (0x102); **Accelerated: yes**; Mesa 26.2.7; OpenGL core/compat 3.3 and GLES 3.0 | Strong evidence for hardware-accelerated GLX on existing XWayland display; not a full compositor FPS test. |

`glxinfo` also reported 1536 MB `Video memory` and `Unified memory: yes`. **Do not treat 1536 MB as dedicated VRAM** or as measured physical RAM available to the OS.

## Revised engineering conclusion

**G1 renderer (partial progress):** Hardware-accelerated Intel OpenGL is now verified for GLX, and Intel EGL contexts succeeded on GBM/Wayland/X11/surfaceless paths. This meaningfully improves plausibility of an OpenGL/OpenGL ES-backed compositor with programmable pixel-wave wallpapers, opacity and possibly blur. However, **GPU-accelerated blur/animation at 1366x768 and 59.79 Hz has NOT been benchmarked**, relevant buffer-import/texture/presentation/extension compatibility was not tested, and native Xorg or an alternative Wayland compositor is not proven to work. Do not lock X11, Penrose, ricom or Smithay yet.

**Vulkan:** The earlier Vulkan enumeration found software llvmpipe. No Intel hardware Vulkan path is established; prefer testing GL/GLES candidates. Device #1 `llvmpipe` appearing in an EGL platform list does not invalidate successful Intel GL contexts.

**500 MiB memory gate:** Still UNKNOWN. This new diagnostic output contains no fresh clean-login idle RAM/PSI/boot measurements. The post-upgrade running-kernel version is also UNKNOWN.

**Next physical-host action:** Confirm running kernel vs package after upgrade, and obtain a baseline `free -m`, process/service categories and PSI from the actual host **without changing any services or destroying user work**. A proper three-clean-login idle test will still be required later. No implementation authorized during M00.

## Privacy and scope

The raw log contained local account/prompt information and a failed password attempt, which were excluded. No usernames, device serials, cookies, addresses, environment-variable values, network data or raw logs are committed. No packages were installed by this repository update, and this document does not imply host changes beyond the user's already performed Arch update.
