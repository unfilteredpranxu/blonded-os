# M00 — physical-host feasibility gap analysis

Audit date: 2026-11-12. Assignment: [Issue #1](https://github.com/unfilteredpranxu/blonded-os/issues/1).
Evidence baseline: repository commit `3c3fe38d86777ad36bf3d23c2829b4e184f5ef81`.

## Decision and evidence boundary

**Keep M00 open for physical-host evidence.** Neither accelerated rendering, a selected desktop backend, animated desktop performance nor <=500 MiB total steady-idle memory has been demonstrated. The audit deliverable is complete; hardware acceptance is not. No implementation or milestone progression follows from this report.

VERIFIED below means explicitly recorded in the checked-in sanitized physical-host summary, not independently remeasured by this audit. INFERRED means engineering reasoning or arithmetic from that evidence. UNKNOWN means the necessary physical measurement is absent. Upstream documentation explains mechanisms; it cannot verify this machine.

Read: [AGENTS.md](../../AGENTS.md), [assignment](../../codex/M00-NEXT-TASK.md), [host facts](M00-host-facts.md), [performance contract](../PERFORMANCE.md), [architecture](../ARCHITECTURE.md), and [roadmap](../ROADMAP.md). Host-fact line references below refer to the pinned baseline, not private original logs. No hardware probes were executed for this report. The commands below are proposed for the real Arch PC only; no execution-environment CPU/GPU/memory result is used as target evidence.

## VERIFIED / INFERRED / UNKNOWN matrix

| Status | Finding | Evidence and limit |
|---|---|---|
| VERIFIED | Arch Linux, kernel 7.2.14-arch1-1 x86_64; Hyprland 0.57.1 Wayland session reported | Host facts, Observed table, lines 7–8. Session presence does not establish its active renderer. |
| VERIFIED | Intel Core i3-2100, 3.10 GHz, 2 cores / 4 threads, AVX | Host facts, CPU row, line 9. No workload throughput or startup profile supplied. |
| VERIFIED | Intel integrated PCI 8086:0102, i915 kernel driver; commonly HD Graphics 2000 | Host facts, Graphics device row, line 10. Kernel binding does not identify the userspace GL/EGL driver. |
| VERIFIED | Vulkan report names llvmpipe 26.2.6, a CPU software renderer | Host facts, Vulkan row, line 11. No hardware Vulkan established. |
| UNKNOWN | Actual OpenGL, OpenGL ES and EGL renderer, API/GLSL versions, extensions and usable platforms | Host facts, line 12 and Still unknown. Missing glxinfo/eglinfo means missing measurements, not absent hardware OpenGL. |
| VERIFIED | Dell D1918H on VGA-1; preferred 1366x768 at 59.79 Hz, scale 1.0 | Host facts, Monitor row, line 13. Preferred mode does not prove current mode, vsync or frame delivery. |
| VERIFIED | MemTotal 3,885,884 kB; free reports 3,794 MiB total | Host facts, RAM row, line 14. This is usable system RAM, not a desktop budget measurement. |
| VERIFIED | Non-idle sample: 2,525 MiB used, 1,269 MiB available, 1,549 MiB buff/cache | Host facts, Contaminated memory sample row, line 15; multiple apps/services were running. |
| VERIFIED | 1.9 GiB zram/zstd, about 41.6 MiB swap used | Host facts, Swap row, line 16. Swap occupancy alone cannot establish present pressure. |
| VERIFIED | Rotational SATA disk about 298.1 GiB; ext4 root about 84 GiB free | Host facts, Storage row, line 17. No latency or I/O attribution supplied. |
| VERIFIED | Intel HDA / PipeWire 1.6.10 with PulseAudio compatibility; temperature around 36 C | Host facts, lines 18–19. Temperature is a single observation, not sustained-load evidence. |
| VERIFIED | Reported kernel 35.380 s + userspace 52.007 s = 87.387 s to graphical target | Host facts, Boot timing row, line 20. This is not login-to-usable-desktop latency. |
| INFERRED | Native refresh interval about 16.73 ms; one 32-bit full-screen surface about 4.00 MiB | 1000/59.79 and 1366*768*4/1048576. Buffer copies, blur passes, window textures and driver allocations add costs. These are calculations, not measurements. |
| INFERRED | Software rendering competes with shell/apps on two physical cores; HDD seeks and shader/font loading may slow startup | CPU/storage facts plus software-renderer mechanism. Cause and magnitude are unmeasured. |
| UNKNOWN | Clean three-login idle baseline, PSS/cgroup attribution, CPU/I/O pressure and page faults | Host facts, Still unknown; performance contract, Protocol. |
| UNKNOWN | Xorg installation/working fallback, custom X11 compositor compatibility, Smithay direct DRM/KMS compatibility, sustained animation/blur p95/p99 and interaction latency | Host facts, Still unknown; architecture, Engineering gates. No custom session has been validated. |

## Graphics feasibility and candidate comparison

Mesa documents [LLVMpipe](https://docs.mesa3d.org/drivers/llvmpipe.html) as software rasterization and its [source tree](https://docs.mesa3d.org/sourcetree.html) describes Lavapipe as a software Vulkan frontend using LLVMpipe. A Vulkan loader, advertised API version or successful enumeration therefore does not establish Intel GPU execution. Keep the report's exact llvmpipe observation; the specific ICD configuration was not recorded. It also says nothing conclusive about the separate OpenGL/EGL path. Do not choose Vulkan for the foundational desktop on this evidence.

| Candidate | Plausibility (INFERRED) | Risks and evidence needed (UNKNOWN) |
|---|---|---|
| Rust X11 WM through Penrose/x11rb plus compositor | A staged candidate consistent with the proposed architecture. Window management can be assessed separately from effects. | A WM alone cannot supply background blur. Xorg, Composite/Damage, usable pixmap-to-texture import, GL/EGL context support, presentation synchronization and texture limits must be established. X server plus compositor plus window buffers consume memory. XWayland results are not native Xorg validation. |
| Rust Smithay Wayland compositor | Worth retaining as a candidate: the existing reported Wayland session makes investigation reasonable, but proves no custom backend compatibility. Smithay offers EGL/GLES rendering, so Vulkan is not a prerequisite for this route. | Direct DRM/KMS output, GBM allocation, buffer import/format/modifier compatibility, EGL/GLES contexts, synchronization, seat/input, TTY switching and possibly XWayland need validation. Nested compositor success would still not prove direct scanout or the final RAM budget. |

The [EWMH compositing specification](https://specifications.freedesktop.org/wm/latest/ar01s08.html) describes redirecting windows into offscreen pixmaps and repainting them. [Smithay backend documentation](https://smithay.github.io/smithay/smithay/backend/index.html) separates graphics, session and input responsibilities and describes GBM, EGL/GLES and DRM paths. These are architectural requirements, not compatibility certifications for PCI 8086:0102.

Conditional recommendation: keep X11 as the first comparison candidate in the architecture, without committing to it. If only software GL/EGL is observed, investigate the existing driver path within M00 before making an effects promise. If accelerated GL/EGL is observed, compare the APIs/extensions actually available against both routes. No GL version ceiling, extension set, Crocus selection, 60 FPS result or Smithay compatibility is asserted here. No new X server, compositor or test session should be started under this assignment.

At native refresh, rendering must fit an approximately 16.73 ms presentation interval; a lower-resolution wallpaper does not remove full-screen compositing costs. Blur adds texture reads and intermediate buffers. A 30 FPS/reduced-effects fallback is a planning option from PERFORMANCE.md, not an achieved result or permission to omit animated wallpaper from memory acceptance. Frame pacing, p95/p99 and responsiveness need physical measurement later.

## 500 MiB and startup assessment

The <=500 MiB gate covers the whole steady desktop: kernel baseline, X server or Wayland compositor, WM, animated wallpaper, shell/launcher/panel, IPC/notifications and necessary services. Browser, Discord and music workloads are measured separately. The 220+170+60+50 MiB planning envelope in PERFORMANCE.md totals 500 MiB but is hypothetical. Rust does not remove allocations in graphics drivers, buffers or system services. A small custom stack is plausible in principle; feasibility on this host remains UNKNOWN.

The existing 2,525 MiB used sample cannot pass or fail the proposed desktop: it contains unrelated applications and is not clean-login idle. MemTotal/1024 minus the rounded 1,269 MiB available gives approximately 2,525.8 MiB; rounding and free's accounting prevent treating this as a new measurement. Do not subtract all buff/cache or sum process RSS to claim physical memory. Use (MemTotal - MemAvailable)/1024 from one /proc/meminfo snapshot as the contract's approximation, with PSS/cgroups for attribution and explicit swap/zram/PSI reporting. The kernel documents [MemAvailable and PSS](https://www.kernel.org/doc/html/latest/filesystems/proc.html) and [PSI](https://docs.kernel.org/accounting/psi.html).

For each of three independent comparable logins, wait at least 60 seconds after desktop readiness and take several samples across the following minute. Record the exact wait and service/effects configuration, native active mode and background workload categories. No essential service disabling, cache dropping, swap changes or forced process termination is part of the test. Existing Hyprland baselines can inform M00 but cannot certify an unimplemented Blonded desktop. A low footprint achieved by swapping, nonworking features or missing wallpaper is not a pass. Later acceptance requires every run to meet <=500 MiB using the declared accounting, with pressure and functionality disclosed.

Two cores and rotational storage make CPU shader work, decompression, font/asset reads and simultaneous service startup plausible bottlenecks. The reported 35.380 s kernel and 52.007 s userspace must be attributed before blaming the HDD. systemd blame durations overlap and do not sum to the critical path. Boot-to-graphical-target and login-to-ready-desktop need separate timings; no service removal, upgrade or disk replacement is recommended on this evidence.

## Minimum next physical-host probes — proposed, not executed

Run only in the user's normal local session on the real Arch PC after confirming it is the recorded i3-2100 / 8086:0102 / Dell target. Do not run in cloud, SSH forwarding, a container or a substituted GPU session. These Bash/Python snippets query existing interfaces and tools; no sudo, installs, downloads, session replacement or configuration changes. Diagnostic context queries briefly consume resources, so finish them before any idle samples. A missing command, denied file or failed platform is a recorded gap, not a reason to install or alter permissions.

### P1: context, tools and renderer

Record only tool presence and whether driver overrides are set; do not print environment values or command paths:

```bash
for tool in glxinfo eglinfo xdpyinfo systemd-analyze Xorg; do
  if command -v "$tool" >/dev/null 2>&1; then
    printf '%s: present\n' "$tool"
  else
    printf '%s: missing\n' "$tool"
  fi
done
for flag in LIBGL_ALWAYS_SOFTWARE LIBGL_ALWAYS_INDIRECT MESA_LOADER_DRIVER_OVERRIDE MESA_GL_VERSION_OVERRIDE MESA_GLES_VERSION_OVERRIDE MESA_GLSL_VERSION_OVERRIDE GALLIUM_DRIVER DRI_PRIME EGL_PLATFORM GBM_BACKEND VK_DRIVER_FILES VK_ICD_FILENAMES LD_LIBRARY_PATH LD_PRELOAD; do
  if [[ -v "$flag" ]]; then printf '%s: set\n' "$flag"; else printf '%s: unset\n' "$flag"; fi
done
case "$XDG_SESSION_TYPE" in
  x11|wayland) printf 'session-type: %s\n' "$XDG_SESSION_TYPE" ;;
  *) printf 'session-type: unknown\n' ;;
esac
```

Only if already installed, run each query; retain platform association and numeric exit status. Filters omit display sockets, paths and device serials; manually review even filtered output before sharing:

```bash
if command -v glxinfo >/dev/null 2>&1; then
  LC_ALL=C glxinfo -B 2>/dev/null | awk '/^[[:space:]]*(direct rendering:|Vendor:|Device:|Accelerated:|OpenGL.*(vendor|renderer|version) string:)/'
  printf 'glxinfo-exit: %s\n' "${PIPESTATUS[0]}"
fi
if command -v eglinfo >/dev/null 2>&1; then
  LC_ALL=C eglinfo -B 2>/dev/null | awk '/^[[:space:]]*((GBM|Wayland|X11|Surfaceless|Device) platform:|EGL (API version|vendor string|version string|client APIs):|OpenGL.*(vendor|renderer|version) string:)/'
  printf 'eglinfo-exit: %s\n' "${PIPESTATUS[0]}"
fi
if command -v xdpyinfo >/dev/null 2>&1; then
  LC_ALL=C xdpyinfo -queryExtensions 2>/dev/null | awk '/^[[:space:]]*(Composite|DAMAGE|DRI3|Present|RENDER)[[:space:]]/'
  printf 'xdpyinfo-exit: %s\n' "${PIPESTATUS[0]}"
fi
```

An empty filtered result is UNKNOWN, including unsupported utility flags. Do not turn a global eglinfo failure into failure of every platform: a valid platform may coexist with another platform's error. Record each platform separately as successful/failed/not reported with vendor, renderer, GL/GLES/GLSL version and acceleration indication. Multiple devices or an Intel string in only one section do not establish which renderer the current compositor uses. Direct rendering: Yes alone is insufficient because software renderers can also be direct.

If summary mode omits required extensions, inspect `glxinfo` and `eglinfo` locally, without saving or publishing full output; share only presence/absence/not-reported for GLX_EXT_texture_from_pixmap, EGL_EXT_image_dma_buf_import, EGL_EXT_image_dma_buf_import_modifiers, EGL_KHR_image_base, EGL_KHR_surfaceless_context and EGL_KHR_fence_sync, associated with the platform/context that advertised them. These are candidate-path checks, not a universal minimum extension list. Record GL texture/renderbuffer size limits if exposed; otherwise leave UNKNOWN. GLX in the existing Wayland session tests XWayland only. Native Xorg and direct DRM/KMS verification stay deferred if no already-running safe session exists. If glxinfo/eglinfo remain absent, renderer verification is blocked; do not install them in this assignment.

### P2: memory, pressure and attribution

The following read-only sample prints only numeric memory/counter summaries and fixed labels. Repeat manually at defined intervals on each planned clean login; do not log out, close workloads or interrupt the current session as part of this audit. User-arranged fresh logins are a later evidence collection step.

```bash
free -m
python3 - <<'PY'
from pathlib import Path
import time
print('sample_monotonic_seconds', round(time.monotonic(), 3))
mem = {}
for line in Path('/proc/meminfo').read_text().splitlines():
    key, value = line.split(':', 1)
    mem[key] = int(value.split()[0])
for key in ('MemTotal', 'MemAvailable', 'Cached', 'SReclaimable', 'Shmem', 'SwapTotal', 'SwapFree'):
    print(key + '_KiB', mem.get(key, 'unknown'))
if 'MemAvailable' in mem:
    print('system_used_approx_MiB', (mem['MemTotal'] - mem['MemAvailable']) / 1024)
for kind in ('cpu', 'memory', 'io'):
    try:
        print('PSI_' + kind, Path('/proc/pressure', kind).read_text().strip())
    except OSError:
        print('PSI_' + kind, 'unavailable')
for group in ('system.slice', 'user.slice', 'machine.slice'):
    try:
        print(group + '_memory_bytes', int(Path('/sys/fs/cgroup', group, 'memory.current').read_text()))
    except OSError:
        print(group + '_memory_bytes', 'unavailable')
pss = readable = unreadable = 0
for proc in Path('/proc').iterdir():
    if not proc.name.isdigit():
        continue
    try:
        rows = (proc / 'smaps_rollup').read_text().splitlines()
        pss += sum(int(row.split()[1]) for row in rows if row.startswith('Pss:'))
        readable += 1
    except OSError:
        unreadable += 1
print('readable_process_PSS_KiB', pss, 'readable_count', readable, 'unreadable_or_exited_count', unreadable)
keys = {'pgfault', 'pgmajfault', 'pswpin', 'pswpout'}
for line in Path('/proc/vmstat').read_text().splitlines():
    key, value = line.split()
    if key in keys:
        print(key, int(value))
for stat in sorted(Path('/sys/block').glob('zram*/mm_stat')):
    try:
        print('zram_mm_stat_numeric', ' '.join(str(int(v)) for v in stat.read_text().split()))
    except OSError:
        print('zram_mm_stat_numeric', 'unavailable')
PY
```

PSS enumeration is intrusive relative to pure idle counters; label it as a separate attribution sample after the idle series. A partial readable PSS sum excludes protected/exited processes and kernel/device memory. Cgroup totals include shared accounting effects, are not PSS, and must not be added to the system total or overlapping groups. Swap-used is SwapTotal-SwapFree; zram mm_stat adds compression/backing RAM context, not free capacity. Compare counter and PSI total deltas over known sample intervals; cumulative faults/stalls are not rates. Keep raw bytes/KiB, state MiB conversion, and report all three runs, including failures.

Inspect the local process/service inventory using `ps -e -o comm=` and `systemctl list-units --type=service --state=running --no-pager --no-legend` (also `systemctl --user` with the same arguments). Do not share their raw output. Share only manually reviewed workload categories and counts: desktop, audio, network, authentication/session, indexing/update, browser, chat, music and other. Do not expose usernames, command arguments, custom unit names or cgroup paths. Compare identical service categories across runs and record any difference instead of disabling services.

### P3: boot, HDD and fallback inventory

Inspect `systemd-analyze time`, `systemd-analyze critical-chain` and `systemd-analyze blame` locally, with stderr kept private. Share only kernel/userspace/graphical-target durations and sanitized role-to-role dependencies with durations for the critical chain; no hostnames, custom unit identifiers or mount paths. Blame is supporting evidence, not a critical-path sum. If installed already, `vmstat 1 6` gives a short numeric CPU/I/O/swap view; discard its since-boot first data row and state that five intervals cannot explain an earlier boot. Do not induce cold-cache reads, reboot or start disk benchmarks. HDD causality remains UNKNOWN without corresponding time-aligned I/O evidence.

Locally inspect `/usr/share/xsessions`, `/usr/share/wayland-sessions` and display-manager service state, reporting only installed X11 session present/absent/unknown, existing Wayland entry present/absent/unknown, display manager active/inactive/unknown and fallback TTY recovery tested/not tested. Do not output session file contents or Exec paths; existence is not proof of recovery. No TTY switch, session launch, restart or login change is authorized here. Recovery operation must be separately demonstrated before later desktop testing.

### Privacy and evidence return

Return a small manually reviewed table containing probe ID, date, confirmed target/session kind, tool present/missing, numeric exit status, platform-specific renderer/version/extension results, active mode if already exposed safely by the current compositor, numeric memory/pressure samples and sanitized boot/fallback outcomes. Mark unavailable fields UNKNOWN. Do not attach private audit files, full diagnostics, journals, EDID, environment dumps, network identifiers, hardware serials, cookies, credentials, usernames or private paths. Redact free-text vendor/device strings if they contain any private identifier. Keep the source summary unchanged; request only these sanitized physical-host results for further M00 decisions.

## Recommendation gates and stop

| Gate | Evidence needed | Current disposition |
|---|---|---|
| G1 — renderer | P1 platform-specific real-host GL/EGL renderer, versions, relevant extensions, override context and acceleration attribution | BLOCKED: absent direct evidence. No graphics backend selected. |
| G2 — memory feasibility | P2 three comparable existing-session logins with idle system totals, pressure, swap/zram and attribution | BLOCKED: only contaminated sample available. Future Blonded acceptance also needs the actual full desktop. |
| G3 — startup and recovery | P3 sanitized boot critical path, HDD/CPU attribution limits, installed fallback inventory; later operational recovery evidence | BLOCKED: aggregate boot timing only; safe fallback unverified. Preserve existing Arch/Hyprland login. |
| G4 — effects and backend acceptance | Later explicitly authorized physical testing: direct backend/buffer/input/recovery operation and sustained native-mode animated wallpaper/blur frame times, p95/p99, interaction latency, CPU and memory | DEFERRED: not an implementation task in M00. No FPS or bootability claim. |
| G5 — milestone review | User reviews sanitized physical evidence and unresolved gates before deciding the next assignment | M00 remains active. Do not advance to M01 automatically. |

Only this audit document is added. No package, Rust source, desktop/session configuration, theme, ISO, installer, roadmap or license change is part of the findings commit.

## Documentation validation evidence

Local documentation checks on 2026-11-12 passed: all six repository-relative links resolve; every table row has three columns; final newline, trailing-whitespace and conflict-marker checks pass. All three Bash blocks pass `bash -n`; the embedded Python block passes `ast.parse`, without executing any probe. Arithmetic was independently recalculated: 16.7252 ms per refresh, 4.00195 MiB per full-screen 32-bit surface and 2,525.8086 MiB for the explicitly approximate contaminated-memory calculation. An automated private-path, credential-assignment, address and email pattern scan passed, followed by manual privacy/scope review.

These checks do not test hardware, validate OS boot or certify performance. The scaffold verification script invokes Cargo tests and is therefore outside the documentation-only checks requested here. No new tests or executable probe files are added; the snippets remain documentation for the next physical-host evidence collection.
