# Performance contract (targets, not present results)

**Release acceptance: <=500 MiB of system-wide RAM at steady desktop idle** on the real i3-2100 machine. Include Linux kernel baseline, X server or compositor, custom WM, animated wallpaper, launcher, panel, IPC, notifications and necessary services.

Close user workloads for this test, including browser, Discord and music player. Record their memory separately in real-use tests; it is not feasible to guarantee 500 MiB with all of them running.

## Protocol
1. Three independent fresh-logins under identical service configurations and native 1366x768 display mode.
2. Animated default wallpaper running, required desktop features operational; wait at least 60 seconds.
3. Measure `free -m`, `MemTotal - MemAvailable` as approximation, PSI, systemd/cgroups, PSS where possible, swap/zram and background process inventory.
4. Report kernel/page-cache effects, page faults and HDD I/O; do not add per-process RSS to claim physical usage.
5. Report animation frame times including p95/p99, interaction latency, rendering backend and CPU/GPU acceleration status.
6. Do not disable essential services just to make the benchmark pass.

## Planning envelope only
OS baseline 220 MiB, WM+compositor+scene 170 MiB, shell/theme services 60 MiB, safety margin 50 MiB; all hypothetical allocations, not measured.

Target approximately 60 FPS at native display refresh *where feasible*, with graceful 30 FPS or lower effects fallback. Neither performance nor compatibility is verified until real hardware testing.
