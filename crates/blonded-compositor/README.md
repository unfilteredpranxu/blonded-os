# Blonded nested compositor — M01-B

M01-B-R1 update (2026-11-15): keyboard geometry arithmetic now saturates moves
and uses clamped 64-bit sums for dimensions before passing requests to the WM.
Output resizing accepts every size satisfying the current WM minimum and gaps
(120×96 with this configuration), including small and unusual aspect ratios.
An impossible/zero size suspends drawing and input while retaining the last valid
WM/output snapshot; a later valid size resumes presentation. Redraw also checks
the actual backend size before rendering/readback. Obsolete output modes are
retired and the current mode is preferred. Initial impossible sizes start
dormant. Minimum host-window hints now reflect the actual 120×96 boundary.

R1 validation: fmt, Clippy, debug build and **33 scoped tests** passed (10 adapter,
23 WM). Seven new tests cover coordinate/dimension extremes, small/asymmetric
outputs, startup suspension, atomic rejection/recovery and Smithay output/WM
consistency. The existing real-client smoke passed again with clean socket and
resource cleanup. Live small-window resizing and physical pointer dragging are
not verified; resize boundary coverage is through source-level integration tests.
An actual Hyprland query and window-only screen capture now verify a mapped native
Wayland window displaying the clients. Physical input hardware remains unverified.

R1 rendering still uses llvmpipe. The current protected process has no `/dev/dri`,
although sysfs registers the Intel i915 device/render node. Standalone GLX and EGL
Wayland/X11/surfaceless also choose llvmpipe; GBM cannot initialize. See
[`M01-B-R1-GPU-diagnostics.md`](../../docs/audits/M01-B-R1-GPU-diagnostics.md)
for exact errors, ruled-out causes and the single next read-only comparison.
No driver, host configuration, permission or sandbox changes were made.

A manually launched Rust Wayland compositor inside the current Wayland session.
Smithay 0.7.0 supplies protocol/rendering machinery; **blonded-wm** controls
internal windows. The outer host window remains an ordinary Hyprland client.
No direct device/session backend, host configuration, autostart or installation.

From the repository root, with the already installed Rust/native dependencies:

```sh
./scripts/dev-nested.sh --smoke-test
```

This builds with one Cargo job, opens a 960×640 nested window and launches two
real bundled `xdg_toplevel`/`wl_shm` clients with different checkerboards. Their
cascaded requested `(32,32)` and `(68,68)` starting positions, bounds, focus and stacking
come from Blonded policy, clamped to the actual outer window size. The host may
tile the outer window smaller than its requested 960×640. The socket is an absolute path under a unique **0700**
temporary directory. `READY socket=...` prints it for additional test clients.
The parent's `WAYLAND_DISPLAY` remains unchanged. Optional native clients must
support xdg-shell and shm; dmabuf-only clients are unsupported.

Inside the nested window:

- Click to focus and raise; normal pointer buttons/scroll and keyboard go to the client.
- **Alt + left drag** moves; **Alt + right drag** resizes from the bottom-right.
- **Alt + arrows** moves 20 logical pixels; **Alt + Shift + arrows** resizes.
- **Alt + Tab** cycles focus; **Alt + F4** requests client close.
- **Alt + Escape**, closing the outer window, or terminal **Ctrl+C** stops the proof.

Host shortcuts can intercept these keys first. The host owns the outer pointer
cursor; client cursor images are not rendered. Server move/resize gestures are
the supported path; client decoration `xdg_toplevel.move/resize` requests are
ignored. Popups are dismissed, not rendered. No touch, IME, accessibility shell,
decorations, fullscreen, tiling controls, workspaces UI, fractional scaling,
client size-hint negotiation or clipboard/drag-and-drop validation is promised.
Keyboard geometry/focus controls provide an alternative to dragging.

Window registration waits for a root surface buffer. Null-buffer unmap and
toplevel destruction remove the WM ID, mapped space entry and drag target;
remap gets a fresh ID. Disconnect destruction is also reaped after dispatch.
Successful WM snapshots set all placement/stacking/focus/configure state.
Clients must acknowledge sizes and commit new buffers; resize is asynchronous
and content is not stretched while waiting. Pointer hit testing uses actual
committed content geometry. Smithay owns protocol buffer lifetimes; renderer
caches retire unused buffers as commits/rendering progress.

Repeated absolute pointer motion keeps one latest position and one demand frame
timer. Drag policy dispatch happens at most once per scheduled frame (plus a
final release flush). No repeated same-position motion is forwarded, and no
permanent idle timer or unconditional redraw loop runs. The 16ms scheduling
interval and advertised 60,000mHz output mode are **not measured FPS**. GLES 3.0
is requested with vsync; no hardware Vulkan is required. Smithay logs actual GL
version/vendor/renderer on initialization. Read those before claiming acceleration.

Reproducible smoke procedure:

```sh
# Build all bins; no packages are installed.
cd crates
CARGO_BUILD_JOBS=1 cargo build --locked -p blonded-compositor --bins
# Automated finite lifecycle/readback/SIGINT/socket cleanup test, from repo root:
# python scripts/smoke-nested.py
# Terminal 1: finite run; captures ONLY the nested framebuffer to a new file.
./target/debug/blonded-compositor --nested --smoke-test --exit-after 20 --capture /tmp/blonded-proof.ppm
# Terminal 2: substitute the absolute socket from READY (do not change host env).
WAYLAND_DISPLAY=/tmp/blonded-m01b-EXAMPLE/wayland ./target/debug/blonded-smoke-client lifecycle --lifecycle
```

The lifecycle client maps, unmaps after 2s, requests remap after another 1s,
and disconnects after another 1s. Look for `MAP`, `UNMAP`, new higher `MAP id`,
`CLIENT disconnected`, `RENDER submitted_with_client=true`, and client
`frame_callback`. The framebuffer readback provides pixel evidence, while
actual presentation and physical input still require inspecting the outer
window. Check focus via client `pointer_enter`/`keyboard_enter` logs and visible
raise; drag and resize both clients and confirm `DRAG`/`KEY geometry` output.
Close one client and confirm focus returns to the other. Stop with Ctrl+C and
check `STOP windows=0 invariants=Ok(())`; the printed socket and its directory
must disappear. Only children explicitly launched by `--smoke-test` are reaped
at shutdown. Independently launched clients get their Wayland connection closed.
The automated runner writes its logs, framebuffer and a short process sample to
a new private `/tmp/blonded-smoke-*` directory and prints the location. Allow up
to 90 seconds for software Mesa initialization; its initial 25s probe once timed
out before READY while compilation was active. Run the smoke after compilation. It does
not send input to the host or capture any other applications.

Validation from `crates/`:

```sh
cargo fmt --all -- --check
CARGO_BUILD_JOBS=1 cargo clippy --locked -p blonded-compositor -p blonded-wm --all-targets -- -D warnings
CARGO_BUILD_JOBS=1 cargo test --locked -p blonded-compositor -p blonded-wm
CARGO_BUILD_JOBS=1 cargo build --locked -p blonded-compositor --bins
```

See the M01-B handoff in `docs/ai-handoffs/` for actual results, runtime blockers
and the distinction between build, rendered pixels, presentation and physical
input evidence. This proof establishes no desktop RAM/FPS/recovery acceptance.

Observed on 2026-11-15: build/fmt/Clippy and 26 scoped tests passed. The automated
real-client smoke passed with three clients, fresh map IDs 3→4 on remap, keyboard
focus events, frame callbacks, 0700 socket directory and SIGINT exit 0 with zero
WM windows/invariants OK and no socket/directory left. A separate timed six-second
run exited 0. Readback showed both clients at WM-controlled positions in the
host-selected 439×755 nested output. Physical presentation and pointer/move/resize
remain **NOT VERIFIED**: the outer window was not identifiable through the host
Hyprland window query. Attempted targeted host shortcuts found no matching window;
they establish no input success. No host configuration was changed.

Actual renderer: **llvmpipe (LLVM 23.1.4, 256 bits)**, Mesa 26.2.7-arch1.1,
GLES 3.2. EGL reported DRI2 driver initialization BAD_ALLOC before software fallback;
its cause is undiagnosed. This run does not validate Intel hardware acceleration
or invalidate earlier physical-host Intel GL evidence. Debug compositor-only RSS
sample: **134,320 KiB (~131 MiB)**; **1 CPU tick at 100Hz (0.01 CPU seconds) over
2.000s** after the lifecycle client exited. This short software-rendered process
sample excludes the host and clients and is neither a performance benchmark nor
the <=500 MiB whole-system desktop acceptance measurement. No FPS was measured.
The first capture attempt exposed a surfaceless-context swap error; restoring
the window target through render/finish fixed it and the final smoke passed.
