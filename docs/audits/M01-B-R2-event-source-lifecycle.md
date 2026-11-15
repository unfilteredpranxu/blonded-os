# M01-B-R2 — Calloop source lifecycle

Base: `fd4decfffa1a6e936d5e29f8c851fdb3ecb1a02c` on
`codex/m01-b-nested-poc-20261115`. Root cause identified from pinned source and
reproduced using the actual locked libraries. **The reported backend warning is
diagnosed as benign but NOT eliminated. A separate owned frame-timer race is fixed.**
WARN logging and the pinned runtime dependencies remain unchanged.

## Identify the loop before identifying a token

The executable contains two Calloop versions with the same logging target:

| Loop | Locked version | Registration order / warning evidence |
| --- | --- | --- |
| Blonded/Smithay outer loop | 0.14.5 | listener 0, display readiness 1, signals 2, Winit 3, first frame timer 4, with no exit timer. With `--exit-after`, its timer occupies 3, Winit 4, frame timer 5. |
| Winit Wayland inner loop | 0.13.0 | Wayland dispatcher 0, user channel 1, awakener ping 2; keyboard-repeat timers subsequently reuse slot **3** for the single-seat case. |

The exact reported wording, **“Received an event for non-existence source”**, is
the `log::warn!` in Calloop **0.13.0** `src/loop_logic.rs:530`. The outer
0.14.5 library instead logs **“Received an event for non-existent source”** at
`src/loop_logic.rs:605`. A slot ID is local to a loop, not a global source identity.
Blonded's long-lived outer Winit source is not repeatedly removed on keypress.

Pinned code evidence:

- [Winit 0.30.13 loop registration](https://github.com/rust-windowing/winit/blob/v0.30.13/src/platform_impl/linux/wayland/event_loop/mod.rs): Wayland dispatcher, user channel, then ping.
- [Winit 0.30.13 keyboard lifecycle](https://github.com/rust-windowing/winit/blob/v0.30.13/src/platform_impl/linux/wayland/seat/keyboard/mod.rs): key press calls `key_input` then removes/replaces `repeat_token` around lines 159–167; key release clears `current_repeat` before removing its token around lines 222–225.
- Calloop 0.13.0 and 0.14.5 `src/sys.rs`, `Poll::poll`: fd readiness is collected and expired timers are appended to the returned batch.
- Both versions' `loop_logic.rs`, `dispatch_events`: synthetic readiness is processed first, followed by the already-collected poll batch. `LoopHandle::remove` unregisters a source and cancels entries still in the timer wheel, **not readiness already copied into that batch**.
- Both versions' `list.rs`/`token.rs`: reusing a vacant slot increments its generation. Lookup validates ID **and generation**; an expired old event cannot call a newly registered replacement.
- Calloop 0.14.5 `sources/timer.rs`: `Timer::from_duration` arms one deadline; `TimeoutAction::Drop` returns `PostAction::Remove`. Calloop unregisters the timer after its callback. `ToInstant` reuses the same registration/counter.

## Reported warning: backend keyboard-repeat cancellation

When Wayland keyboard events and an expired repeat are ready in one batch:

1. Winit processes the release or replacement key first, clears/replaces repeat
   state, and removes the old repeat timer.
2. An old timer event remains in Calloop's collected batch.
3. Calloop rejects its removed/stale-generation token and warns. It cannot invoke
   either the removed repeat callback or the replacement timer's callback.

This is correct cancellation of a repeat that must no longer be delivered. Real
key input has already been dispatched by `key_input`; rejecting the stale timer
does not discard that input. Versions 12, 13 and 19 reflect repeat registrations
whose slots were recycled; warnings only appear when readiness overlaps removal.

The new pinned-0.13 regression reproduces **id 3/version 0/sub_id 0**, replaces it
with id 3/version 1, captures the exact warning and verifies zero repeat callbacks,
an invalid old token, and a still-valid replacement. It uses the same synthetic-
before-timer dispatch ordering as Winit/WaylandSource, without requiring a GUI.

**Boundary:** Winit owns this private keyboard lifecycle. Blonded has no public
hook to defer its internal repeat-token removal. The benign warning can recur
with the pinned backend. Eliminating that warning through behavior would require
a separately reviewed backend lifecycle change, such as retiring repeat timers
after the collected batch. No logging filter, dependency fork or unrelated
runtime upgrade was introduced. This is not claimed as a warning-free keyboard fix.

## Separate outer-loop race and minimal correction

The old Redraw callback called `LoopHandle::remove(frame_timer)`. If Winit's
synthetic Redraw ran before an expired frame timer in the same outer batch,
Calloop 0.14.5 could emit its distinct **non-existent** warning. A regression
reproduces that old pattern against 0.14.5, independently of the backend warning.

Now Redraw calls `begin_frame`, clearing demand and updating the pacing timestamp
without removing the registered timer. The timer retains its registration until
its own callback:

- With no demand, a suspended output, or shutdown: clear ownership and return Drop.
- With fresh demand after an early redraw: reuse the same timer, returning
  ToInstant if the new 16ms pacing deadline has not arrived.
- When demand is due: request one host redraw, clear timer ownership, return Drop.

`schedule_frame` never inserts another timer while one is owned, and it rejects
shutdown requests. Cleanup marks rendering stopped even after a dispatch error.
A canceled pending timer may cause **one finite retirement wake**, with no redraw;
there is no permanent idle timer/polling or unconditional redraw loop. There is
no imperative frame-source removal from another event callback. Pointer motion
coalescing, rendering and real client frame callbacks remain in their existing paths.

## Tests and actual runtime

All Cargo commands used one build job and the already available temporary Cargo
cache. From `crates/`:

| Check | Result |
| --- | --- |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --offline --locked -p blonded-compositor -p blonded-wm --all-targets -- -D warnings` | PASS |
| `cargo test --offline --locked -p blonded-compositor -p blonded-wm` | PASS: **40 tests** (17 compositor + 23 WM) |
| `cargo build --offline --locked -p blonded-compositor --bins` | PASS |
| `python scripts/smoke-nested.py` from repository root | PASS |

Seven focused lifecycle tests cover both pre-fix warnings; 1,000 repeated frame
requests; same-batch early redraw/cancellation and re-registration; reuse with a
new pacing deadline; 64 move/resize bursts with 2,048 coalesced drag motions; and
shutdown with an already-queued timer. Fixed-scheduler cases capture **zero WARNs**,
verify retired tokens become invalid, one request per burst and no residual timer.
The two reference regressions explicitly assert the expected library warnings;
test capture is not a production logging change. The Calloop 0.13/log/tracing
direct additions are dev dependencies already present in the lockfile graph;
no dependency versions changed.

The bounded graphical smoke used three real clients, map/remap IDs 3→4, keyboard
focus/frame callbacks/readback, a 0700 socket directory, SIGINT exit 0 and zero
remaining windows with valid invariants. Socket and directory removal passed.
Neither warning appeared in this smoke; **it did not exercise host keyboard repeat**.
The harness sampled process-only RSS 131,456 KiB and 1 CPU tick/100Hz over 2.000s;
this is not desktop idle, FPS or release-performance evidence.

The owner's R2 evidence already verifies Intel HD Graphics 2000/GLES 3.0,
physical mouse dragging/resizing, keyboard geometry controls and clean shutdown.
Those results are preserved. No GPU diagnostics or host input injection were run.
This protected smoke's existing renderer log reports software rendering; it does
not revise the owner's physical-host acceleration result. A new physical
interactive R2 run remains for review, particularly whether any remaining WARN
has the diagnosed 0.13 wording or a different source/version.

Git staging failed with read-only `.git/index.lock`; all changes remain preserved
uncommitted. Stop after M01-B-R2; no M02 or host configuration changes.
