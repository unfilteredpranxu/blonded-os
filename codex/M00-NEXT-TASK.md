# Codex Cloud — first assignment: M00 only

Repository: `unfilteredpranxu/blonded-os`.

Before any actions read `AGENTS.md`, `docs/audits/M00-host-facts.md`, `docs/PERFORMANCE.md`, `docs/ARCHITECTURE.md` and `docs/ROADMAP.md`.

**Mission:** Complete the real-host hardware feasibility gap analysis; no compositor, window-manager, installer or ISO code.

1. Extract VERIFIED facts and cite specific lines/sections of checked-in sanitized hardware evidence.
2. Classify graphics capability UNKNOWN until the actual OpenGL/EGL renderer is recorded; explain why software llvmpipe Vulkan is not Intel GPU acceleration.
3. Assess plausibility and risks of X11 custom WM/compositor vs Smithay Wayland for Intel 8086:0102, considering 1366x768@59.79 Hz.
4. Evaluate 500 MiB total steady-idle OS target, distinguishing non-idle audit snapshot from a clean-login test, and document CPU/HDD startup bottlenecks.
5. Propose the minimum set of **read-only, privacy-sanitized host probes** required next. Do not require attaching private reports or using Codex Cloud's hardware as substitute.
6. Write `docs/audits/M00-gap-analysis.md` with VERIFIED / INFERRED / UNKNOWN matrix, feasibility risks, privacy, evidence and exact proposed host tests. Run documentation-only checks.
7. Report files changed, test evidence, unresolved issues and STOP. Do **not** advance to M01.

PROHIBITED: package installation on the physical host, modifying configuration, processes, disk/boot, installing a graphical session, pretending the OS boots, claiming FPS or hardware OpenGL unsupported without direct evidence, making releases, changing project name or license.
