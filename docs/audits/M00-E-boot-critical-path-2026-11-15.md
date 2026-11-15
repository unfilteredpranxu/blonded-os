# M00-E — Startup critical-chain and short runtime I/O observation (2026-11-15)

Source: real Arch Linux physical host, user-executed `systemd-analyze critical-chain --no-pager`, `systemd-analyze blame --no-pager | head -20` and `vmstat 1 6`. This document contains a sanitized summary only; no full raw logs, hostnames, usernames, unique network namespace identifiers or local paths are committed.

## VERIFIED observations

| Source | Observation |
|---|---|
| critical chain | `graphical.target` at ~46.996 s of userspace startup |
| critical chain | `docker.service` started at ~25.061 s and took **21.934 s**; it appears on the critical dependency chain into `multi-user.target` / `graphical.target` |
| critical chain | `NetworkManager-wait-online.service` started ~17.492 s and took **7.566 s**, upstream of Docker via `network-online.target` |
| critical chain | `NetworkManager.service` ~16.032 s + 1.453 s |
| critical chain | `basic.target` ~16.000 s and `sysinit.target` ~15.550 s |
| blame list | `dev-sda2.device` **9.590 s**. It is NOT shown as an independent additive 9.590 s on the Docker path. |
| blame list | `systemd-journal-flush.service` **5.159 s** and `containerd.service` **3.379 s**; do not sum blame durations to obtain critical time |
| blame list | `user@1000.service` **5.202 s**, with no causal attribution from this result |
| preceding `systemd-analyze time` | kernel **44.947 s** plus userspace **46.817 s** = **91.764 s**; graphical target in the later chain shown at ~46.996 s |
| `vmstat 1 6` | 1 first row of since-boot averages, followed by five short intervals; subsequent CPU idle about 57%, 99%, 98%, 99%, 99%, I/O wait ~2%, 0%, 2%, 1%, 1% |
| `vmstat 1 6` | one short interval saw CPU user 37%, others ~0-1%; the per-interval measurements do not establish prolonged system idleness |
| `vmstat 1 6` | `si` and `so` were zero during the displayed samples; `swpd` was zero in this later snapshot. It differs from the earlier swap snapshot; no inference about whether a reboot or swap reset caused the difference. |

## INTERPRETATIONS / LIMITS

- **Userspace:** Docker's reported ~22 s activation **lies on the critical path**, and the preceding NetworkManager wait-online ~7.6 s is also on that path. These are concrete investigation priorities. However removing either service does **not** guarantee equal boot time savings because other dependencies may become the longest path.
- **Kernel:** about 45 s of boot time is outside the displayed userspace critical chain, and remains unexplained. The 9.590 s storage-device duration and I/O-pressure observations support investigating storage but do not prove HDD fault or kernel slowness causality.
- **Inconsistent/transient mount timing:** original critical-chain output displayed a dynamically named Docker network-namespace mount with an activation timestamp (~44.910 s) nested under `local-fs.target` (~13.745 s). Such output cannot be treated as a simple chronological sum. Do not publish the dynamic mount name or infer that filesystem startup took 45 s.
- **Short live sample:** CPU showed mostly high idle after the first row; not a guarantee about startup, not a GPU benchmark, and not sufficient for the future <500 MiB Blonded desktop RAM target.
- No safe conclusion that Docker, remote development tooling, networking or audio services can be disabled. The user actively develops with containers and services; preserve functionality.

## Next read-only, physical-host probe (proposed, not executed)

Inspect the *dependency relationship*, without stopping anything or printing logs:

```bash
echo "=== DOCKER DEPENDENCIES ==="
systemctl show docker.service -p Type -p After -p Wants -p Requires -p TimeoutStartUSec
echo "=== NETWORK WAIT DEPENDENCIES ==="
systemctl show NetworkManager-wait-online.service -p Type -p After -p Wants -p Requires
echo "=== BOOT ENABLEMENT (OBSERVE ONLY) ==="
systemctl is-enabled docker.service NetworkManager-wait-online.service 2>&1
echo "=== KERNEL VERSION ==="
uname -r
```

Review dependency names before sharing. Running-kernel vs installed-kernel mismatch was previously recorded and may have changed since then; do not assume which kernel was used in this sample. We will not reboot, change boot targets, disable services, or alter user sessions under M00. More complete kernel phase and safe physical-host idle baselines remain outstanding.

**Decision:** M00 ongoing. Store findings and request remaining read-only evidence. Do not begin M01 or claim bootable OS, <500 MiB desktop idle, or stable animated compositor.
