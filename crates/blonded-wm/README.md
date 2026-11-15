# Blonded window-management core — M01-A

Original Rust policies for simulated windows, independent of graphics and Linux
protocol bindings. This crate has **no third-party dependencies**, unsafe code,
threads, polling, file access, subprocesses, or display connection. It does not
manage visible application windows. No existing window-manager implementation
was copied. No repository license is selected by this change.

From `crates/`, run:

```sh
CARGO_BUILD_JOBS=1 cargo run --offline -p blonded-wm --example headless
CARGO_BUILD_JOBS=1 cargo test --workspace --offline
cargo fmt --all -- --check
CARGO_BUILD_JOBS=1 cargo clippy --workspace --all-targets --offline -- -D warnings
CARGO_BUILD_JOBS=1 cargo run --release --offline -p blonded-wm --example policy_load
```

The demonstration performs 19 commands, printing the command, event batch,
geometry, focus, visibility and workspace after each step. It restores the first
window to `(90, 110, 600, 400)` after tiling, removes the second window, and ends
on workspace zero. Errors propagate to a nonzero process exit.

The public interface is `WindowManager::new(Config)` and
`dispatch(Command) -> Result<Vec<Event>, Error>`. Inspect the manager with
`window`, `windows`, `workspace`, `focused`, `is_visible`, and
`check_invariants`. Queries borrow immutable state. A failed command preserves
the entire prior state and emits no events. A repeated no-op returns an empty
event batch. Commands and events contain typed data rather than shell strings.

Policies and limits:

- Backend-assigned `u64` window IDs must be unique while registered; all ID values
  are supported. Removal purges stacking and focus references and reflows tiles.
  Stale operations return `UnknownWindow`. The adapter must prevent delayed
  callbacks from targeting a new window if it reuses an old ID.
- A single configurable screen uses half-open logical-pixel rectangles with
  signed origins. Dimensions are positive and at most `i32::MAX`; rectangle
  endpoints must fit `i32`. Wider arithmetic and checked conversions handle
  extreme coordinates. Zero/overflowing geometry is rejected at construction.
- `gap` is both outer screen inset and inter-tile/inter-region spacing. Floating
  registration and moves clamp to usable bounds. Resize anchors the top-left and
  clamps to global minimum sizes and remaining screen space. A minimum of one
  pixel is valid for simulations; a backend should select usable human-scale
  minimums. Client-specific constraints are not implemented yet.
- Floating, equal columns and equal rows are workspace policies. Tiles are in
  ascending window-ID order, independent of focus or stacking. Remainder pixels
  go to earlier tiles so coverage is exact. Impossible arrangements return
  `InsufficientSpace`, including on registration, restore, workspace transfer or
  screen changes. Switch to floating or enlarge the screen before retrying.
- Every window retains a floating rectangle through tiling, maximizing, snapping
  and minimizing. Screen/config changes clamp that saved rectangle as necessary;
  old off-screen coordinates are not retained indefinitely. Manual move/resize
  require normal mode in a floating workspace; other modes return `NotFloating`.
- Maximized windows fill the usable bounds; snaps cover left/right/top/bottom
  halves or four corners with the configured gaps. They temporarily leave the
  normal tile set. Explicit `Snap` commands perform snapping; pointer proximity,
  gestures, hit testing and input bindings belong to a future adapter.
- Minimized windows are excluded from focus and tiling. Restore first reveals a
  minimized window in its previous mode; a subsequent restore returns it to
  normal placement. Restore/maximize/snap do not implicitly focus or raise;
  use `Focus` explicitly when desired.
- Each workspace remembers its focus and bottom-to-top stacking independently.
  Focus raises a visible window on the active workspace. Raise alone preserves
  focus. Focus cycling uses stable ascending ID order, skips minimized windows,
  wraps in both directions and does nothing on an empty workspace. Focus loss
  selects the topmost eligible remaining window, or `None`.
- Registration focuses the new window in its workspace. Moving a nonminimized
  window focuses it in its destination, without switching the active workspace.
  Operations on inactive workspaces are allowed, except explicit `Focus`.
- Workspace count is fixed at construction (1–32). Global window capacity is
  configurable up to 256. State has no unbounded history or event queue. Each
  batch contains at most 256 window events, 32 workspace snapshots, and two
  global events. Callers own their returned batches and must bound any queue.

Dispatch validates invariants, clones bounded state, executes and reflows, checks
invariants again, then publishes the final state and deterministic delta. This
favors atomic failure handling over premature optimization. Storage and working
space are O(N + W); reflow is O(W*N + N log N) with small fixed caps. Returned
workspace stacking lists contain at most N IDs in total. `policy_load` exercises
256 windows, 32 workspaces and 4096 commands, with no external benchmark library.
Its timings concern headless policy only and exclude window creation and rendering.
General allocator exhaustion is subject to Rust's standard allocation behavior.

A future Wayland/Smithay adapter should maintain its own surface-to-ID map,
translate map/unmap/input/output notifications into commands, and process each
successful event batch as one logical transaction. `Registered`/`WindowChanged`
contain full window snapshots; `WorkspaceChanged` contains focus, layout and
stacking. `ActiveWorkspaceChanged` requires recalculating visibility using
`is_visible`; it intentionally does not resend every window. Backend startup can
read a full snapshot from the manager. Geometry is a desired policy result:
Wayland configure/ack sequencing, surface commit, rendering, input routing,
protocol errors and disconnected clients remain adapter responsibilities.

Accessibility at this layer consists of explicit focus, predictable cycling,
minimum geometry and independent move/resize commands usable by keyboard or
pointer adapters. Keyboard mappings, accessible shell UI, client size hints,
multi-output handling, animation, compositor rendering, and live recovery are
unimplemented. Tests and synthetic timings establish no desktop FPS, graphics
acceleration, whole-system idle memory, or deployment readiness.
