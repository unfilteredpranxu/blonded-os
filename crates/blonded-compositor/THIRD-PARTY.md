# Upstream attribution

The protocol dispatch, surface hit testing, seat/selection delegation and basic
GLES output rendering in `src/handlers.rs`, `src/state.rs`, `src/input.rs` and
`src/main.rs` adapt scaffolding from [Smithay smallvil v0.7.0](https://github.com/Smithay/smithay/tree/a166cf4c94b5aedc332a65aa1dd753e8148829c3/smallvil).
The [anvil winit backend](https://github.com/Smithay/smithay/blob/a166cf4c94b5aedc332a65aa1dd753e8148829c3/anvil/src/winit.rs)
was also inspected for backend/error handling. Smithay is MIT licensed; its
copyright and full permission notice are retained in `licenses/Smithay-MIT.txt`.

Smithay is pinned to crates.io **=0.7.0** with the registry checksum and every
resolved dependency in `../Cargo.lock`. Its release tag is commit
`a166cf4c94b5aedc332a65aa1dd753e8148829c3`. The crate uses `backend_winit`,
`wayland_frontend`, and `desktop`; default features are disabled. EGL/GLES is
enabled by winit; Vulkan, DRM/GBM, libinput, libseat, Xwayland and direct session
backends are disabled. Winit's transitive X11 support is compiled by Smithay,
but runtime selection is explicitly Wayland only.

The original `blonded-wm` engine provides geometry, bounds, focus and stacking.
The lifecycle adapter, drag coalescing, private socket, demand scheduling and
bundled shm client are integration work for this proof. This attribution does
not choose a license for the rest of the Blonded repository.
