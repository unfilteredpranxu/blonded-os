# M00-D — Physical-host memory, kernel, pressure and boot sample (2026-11-13)

**Scope:** Redacted evidence transcribed from user-executed read-only diagnostics on the real Arch Linux PC. Not collected from Codex Cloud. The raw terminal transcript is deliberately not committed.

## Recorded facts

| Metric | Actual result | Interpretation / limit |
|---|---|---|
| Running kernel, `uname -r` | `7.2.15-arch1-1` | Current active kernel at time of measurement |
| Installed `linux` package | `7.3.4.arch1-1` | **Kernel mismatch:** updated kernel not yet booted; do not claim the updated kernel is active |
| Installed Mesa | `1:26.2.7-1` | Matches most recent EGL/GLX diagnostics |
| Physical RAM total | 3,794 MiB (reported by `free -m`) | About 3.71 GiB usable |
| Physical RAM `used` | 2,447 MiB | Mixed workload, **not clean desktop idle** |
| Physical RAM `free` | 108 MiB | Not equivalent to available RAM; Linux caches are reclaimable |
| `buff/cache` | 1,651 MiB | Does not imply all cached memory is immediately free |
| `available` | 1,346 MiB | Approximate system memory currently available for applications |
| Shared | 167 MiB | `free -m` value |
| Compressed swap used | 67 MiB out of 1,896 MiB | Swap occupancy alone does not imply current swapping pressure |
| CPU PSI `some` | avg10 2.03%; avg60 0.80%; avg300 0.35% | CPU contention sample, non-idle |
| CPU PSI `full` | avg10 0%; avg60 0%; avg300 0% | No full-CPU stall in rolling intervals reported |
| Memory PSI `some` | avg10 0.01%; avg60 0.06%; avg300 0.01% | Low current sampled memory contention |
| Memory PSI `full` | avg10 0.01%; avg60 0.06%; avg300 0.01% | Low current sampled memory full stalls |
| I/O PSI `some` | avg10 3.64%; avg60 1.33%; avg300 1.18% | Noticeable sampled I/O stalls, cause unproven |
| I/O PSI `full` | avg10 2.83%; avg60 1.02%; avg300 1.03% | System-wide full I/O stalls in sample, not attribution to one process/disk |
| Boot to graphical target | Kernel 44.947s + userspace 46.817s = 91.764s | **Not** login-to-ready-desktop time; critical path unknown |

## Workload contamination

The 20 largest process RSS entries (not additive system memory) include many instances of a browser-like multi-process application, an active AI gateway, one terminal, the current Wayland compositor, remote streaming, a container daemon, a container runtime and XWayland. We deliberately omit private/local process names, arguments and usernames. These active processes invalidate any attempt to certify the <=500 MiB desktop-idle acceptance target from this snapshot. RSS sums double-count shared resident pages and omit kernel/device memory.

## Engineering implications

1. **Remain M00.** No clean three-login Blonded OS memory benchmark exists, nor can one exist before the actual desktop is implemented. A sanitized current-Arch baseline will be useful only for comparison.
2. **Kernel update not activated.** User should save work and plan a normal controlled reboot at a convenient time; do not command a reboot or assume it is safe without awareness of ongoing workloads. Following any reboot, verify `uname -r` and functional display/network/input.
3. **Boot slowness merits critical-chain analysis.** Do not equate `systemd-analyze blame` cumulative values with boot critical path. Do not conclude the HDD is the sole cause from a single I/O pressure snapshot.
4. **500 MiB is a future release gate, not a hardware guarantee.** Rich animations and an active moving wallpaper must be included; manually opened browser/Discord/music applications measured separately.
5. No services were stopped, configs edited, kernel restarted, settings altered, disks benchmarked or graphics backend selected based on this evidence.

## Next physical-host read-only evidence

Read-only commands, after reviewing that no identifying custom unit names will be shared verbatim:

```bash
printf '=== BOOT CRITICAL PATH (review before sharing) ===\n'
systemd-analyze critical-chain --no-pager
printf '=== SLOWEST REPORTED SERVICES (review before sharing) ===\n'
systemd-analyze blame --no-pager | head -20
printf '=== SAMPLE I/O SCHEDULING AND ACTIVITY ===\n'
if command -v vmstat >/dev/null 2>&1; then vmstat 1 6; else echo 'vmstat unavailable'; fi
printf '=== RUNNING KERNEL VERSUS INSTALLED ===\n'
uname -r
pacman -Q linux mesa
```

These commands do not change the system. `vmstat 1 6` measures 5 one-second intervals after the first since-boot row and is not a boot-time benchmark. Do not publish raw journal logs, usernames, private paths or credentials. Any `critical-chain` output may contain custom local service names; sanitize them if needed. Do not schedule or initiate a reboot automatically.
