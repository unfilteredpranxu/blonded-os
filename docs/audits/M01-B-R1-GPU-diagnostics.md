# M01-B-R1 — graphics diagnosis, 2026-11-15

These are current observations from the protected PC-local agent process. They
are separate from the owner's earlier successful Intel probes in
`M00-C-graphics-evidence-2026-11-13.md`. No package, driver, permission, service,
Hyprland configuration or sandbox boundary was changed.

## Device access and configuration

- Sysfs identifies `card1` and `renderD128`, major/minor `226:1` and `226:128`,
  vendor/device **8086:0102**, with **i915** on the display device. Hyprland's
  read-only `systeminfo` query independently identifies the same Intel controller.
- **`/dev/dri` is absent in this process's filesystem view.** Opening the expected
  `/dev/dri/card1` and `/dev/dri/renderD128` returns **ENOENT (2)**. There is no
  accessible render node here. Node ownership/mode and host-side group access
  cannot be checked through absent nodes; this is not an observed EACCES error.
  Sysfs registration does not grant a process access to the device.
- Installed graphics tools: `eglinfo`, `glxinfo`, `lspci`, `hyprctl`; `strace`,
  `drm_info` and `glmark2-wayland` are absent. Nothing was installed.
- The GLVND EGL vendor JSON selects `libEGL_mesa.so.0`; no `/etc/glvnd/egl_vendor.d`
  override exists. Installed DRI modules include `crocus_dri.so`, `iris_dri.so`,
  `swrast_dri.so` and `zink_dri.so`. File presence establishes no working GPU path.
  Default GLX actually attempts **crocus**, then fails DRM device discovery.
- No `/etc/drirc`, user `.drirc` or user `.config/drirc` was found. Checked
  `/etc/environment`, `/etc/profile.d` and packaged drirc rules contain no relevant
  software-forcing/application-specific override for these probes or Blonded.
  Packaged crocus rules inspected concern other applications.
- No inherited `LIBGL_*`, `MESA_*`, `GALLIUM_*`, `DRI_*`, `EGL_*`, `GBM_*`,
  `__GL*` or `__EGL_*` override was observed. Explicit checks also found
  `LD_LIBRARY_PATH`, `LD_PRELOAD`, `VK_ICD_FILENAMES`, `VK_DRIVER_FILES`,
  `mesa_glthread` and `vblank_mode` unset. Display routing is Wayland plus XWayland.
- A temporary `WAYLAND_DEBUG=client EGL_LOG_LEVEL=debug LIBGL_DEBUG=verbose`
  probe changed logging only. Its parent registry includes **wl_drm** and
  **zwp_linux_dmabuf_v1**; missing advertisement of both GPU protocols is ruled
  out. This does not validate device-FD transfer/import compatibility.

## Execution paths actually tested

All commands ran in the same protected process context with existing libraries.
`timeout` bounds were 40–50 seconds; none of these probes timed out.

| Path / command | Exit | Renderer and API observed |
| --- | --- | --- |
| `glxinfo -B` | 0 | **llvmpipe (LLVM 23.1.4, 256 bits)**, Accelerated: **no**, GL 4.6 / GLES 3.2 |
| `eglinfo -B -p wayland` | 0 | Same llvmpipe, Mesa EGL 1.5, GL 4.6 / GLES 3.2 |
| `eglinfo -B -p x11` | 0 | Same llvmpipe, Mesa EGL 1.5, GL 4.6 / GLES 3.2 |
| `eglinfo -B -p surfaceless` | 0 | Same llvmpipe, Mesa EGL 1.5, GL 4.6 / GLES 3.2 |
| `EGL_LOG_LEVEL=debug LIBGL_DEBUG=verbose eglinfo -B -p gbm` | 1 | `eglinfo: eglInitialize failed`; no renderer established |
| R1 debug nested compositor, GLES 3.0 requested, two smoke clients plus lifecycle client | 0 | Same llvmpipe, **OpenGL ES 3.2 Mesa 26.2.7-arch1.1**, GL vendor Mesa |

GLX failure before fallback:

```text
MESA: error: Failed to query drm device.
glx: failed to create dri3 screen
failed to load driver: crocus
```

Wayland EGL failure before fallback (also reproduced without Smithay):

```text
MESA-EGL: warning: failed to get driver name for fd -1
MESA-LOADER: failed to retrieve device information
MESA: error: ZINK: failed to choose pdev
MESA-EGL: warning: egl: failed to create dri2 screen
```

The compositor logs the more specific EGL error before creating its GLES renderer:

```text
[EGL] 0x3003 (BAD_ALLOC) eglInitialize: DRI2: failed to get driver name
GL Version: "OpenGL ES 3.2 Mesa 26.2.7-arch1.1"
GL Renderer: "llvmpipe (LLVM 23.1.4, 256 bits)"
```

The failing hardware stage is **Mesa's DRM device/DRI screen initialization
during `eglInitialize`**, before GLES context/rendering and WM/client content.
Smithay 0.7.0's inspected native-display path selects Wayland KHR/EXT platforms
and calls `eglInitialize` before context creation. The same failure in standalone
EGL/GLX is evidence against a Smithay-specific WM, readback or EGL-config defect
as the sole explanation. A successful tool exit after fallback is not hardware
success. The later ZINK error does not establish Vulkan as a Blonded requirement.

**Diagnosis:** missing device visibility is a confirmed hardware-access blocker
in this execution context and is consistent with the loader failures. It is not
a complete proof that no other driver/FD-transfer problem exists. No alternative
driver was forced or declared working. Default crocus selection, installed files
and earlier Intel success do not prove crocus can initialize inside this boundary.
Hardware acceleration is **NO for the observed runs**; the current unrestricted
host rendering path remains **NOT VERIFIED**. Earlier Intel GL evidence remains
valid historical evidence; these probes do not establish a machine-wide regression.

## Runtime evidence and remaining gate

`python scripts/smoke-nested.py` passed: three real clients, remap IDs 3→4,
keyboard-focus events, frame callbacks/readback, 0700 socket directory, SIGINT
exit 0, zero WM windows with valid invariants, socket and directory removed.
The final harness sampled compositor-only debug RSS **134,040 KiB** and **3 CPU
ticks at 100Hz over 2.000s**. This is a short software-rendered process sample,
not whole-system idle, FPS, a benchmark or release acceptance evidence.

A separate 15-second run was queried while alive: Hyprland identified one mapped
**native Wayland** Blonded window, **439×755**. An actual `grim` capture restricted
to that window was inspected and showed both checkerboards. Host window mapping
and screen-capture presentation are **VERIFIED**. Human observation of a physical
monitor and physical pointer dragging/keyboard hardware remain **NOT VERIFIED**.
The finite run exited 0. No host input was injected or configuration changed.
Logs and images remain temporary local artifacts; private host output is excluded
from this checked-in report.

**Single next supported diagnostic:** the owner runs the same read-only
node-visibility/Wayland-EGL comparison in the already-working normal Hyprland
terminal, outside this agent's device-restricted execution context:

```sh
stat -c '%F mode=%a' /dev/dri/renderD128
eglinfo -B -p wayland
```

Return only node presence/mode, renderer/API and relevant errors. This comparison
requires no install, sudo, permission changes, unrestricted agent invocation or
sandbox workaround. No system fix is supported until that comparison is reviewed.
