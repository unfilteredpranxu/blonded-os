# M00 sanitized physical-host facts — 2026-11-12

This is a **redacted summary** from the user's real Arch Linux PC; do not commit the full raw audit. Missing evidence is explicitly marked. These values are not from Codex Cloud.

| Property | Observed |
|---|---|
| OS | Arch Linux, kernel 7.2.14-arch1-1 x86_64 |
| Current graphical session | Hyprland 0.57.1, Wayland |
| CPU | Intel Core i3-2100 at 3.10 GHz, 2 physical cores / 4 threads, AVX |
| Graphics device | Intel 2nd-generation integrated graphics PCI 8086:0102, i915 kernel driver, commonly Intel HD Graphics 2000 |
| Vulkan | llvmpipe 26.2.6 **software CPU renderer**; no hardware Vulkan established |
| OpenGL and EGL | UNKNOWN, glxinfo and eglinfo absent in report |
| Monitor | Dell D1918H, VGA-1, preferred 1366x768 at 59.79 Hz, scale 1.0 |
| RAM | MemTotal 3,885,884 kB; `free -m` reports 3,794 MiB total |
| Contaminated memory sample | 2,525 MiB used, 1,269 MiB available, 1,549 MiB buff/cache; multiple applications and services running; **not desktop idle** |
| Swap | 1.9 GiB zram / zstd, ~41.6 MiB used |
| Storage | Rotational SATA disk ~298.1 GiB; ext4 root ~84 GiB free |
| Audio | Intel HDA, PipeWire 1.6.10 with PulseAudio compatibility |
| CPU temperature | Around 36 C at time of measurement |
| Boot timing | ~35.380s kernel + ~52.007s userspace = ~87.387s to graphical target; not measured login-to-desktop |

## Still unknown
- Real hardware OpenGL/EGL renderer and supported acceleration path/extension limits.
- X11 vs Wayland/custom compositor compatibility on this driver.
- Actual sustained animated wallpaper and blur frame times.
- Clean three-login desktop memory baseline for 500 MiB target.
- Exact boot critical path, performance impact of rotating disk.
- Safe installed fallback Xorg/login recovery.

**Privacy:** original source included local username/path and an audio authentication cookie identifier. Do not publish raw host logs, private network identifiers, EDID serials, hardware serials or credentials.
