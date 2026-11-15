//! Real Calloop dispatch batches, without a GUI or a GPU. These deliberately
//! distinguish the backend's 0.13 warning from our own 0.14 scheduler.
use std::{
    cell::{Cell, RefCell},
    io::Write,
    rc::Rc,
    sync::{Arc, Mutex, Once},
    time::{Duration, Instant},
};

use calloop::EventLoop;
use smithay::reexports::wayland_server::Display;

use crate::{policy::DragKind, state::State};

// This matches Winit/WaylandSource's before_sleep synthetic readiness. Calloop
// processes it before expired timers already copied out of the timer wheel.
macro_rules! synthetic_source {
    ($module:ident, $api:ident) => {
        mod $module {
            use $api::{EventSource, Poll, PostAction, Readiness, Token, TokenFactory};
            pub struct Source {
                pending: bool,
                token: Option<Token>,
            }
            impl Source {
                pub fn new(pending: bool) -> Self {
                    Self {
                        pending,
                        token: None,
                    }
                }
            }
            impl EventSource for Source {
                type Event = ();
                type Metadata = ();
                type Ret = ();
                type Error = std::io::Error;
                const NEEDS_EXTRA_LIFECYCLE_EVENTS: bool = true;
                fn register(
                    &mut self,
                    _: &mut Poll,
                    factory: &mut TokenFactory,
                ) -> $api::Result<()> {
                    self.token = Some(factory.token());
                    Ok(())
                }
                fn reregister(
                    &mut self,
                    poll: &mut Poll,
                    factory: &mut TokenFactory,
                ) -> $api::Result<()> {
                    self.register(poll, factory)
                }
                fn unregister(&mut self, _: &mut Poll) -> $api::Result<()> {
                    Ok(())
                }
                fn before_sleep(&mut self) -> $api::Result<Option<(Readiness, Token)>> {
                    Ok(if std::mem::take(&mut self.pending) {
                        Some((Readiness::EMPTY, self.token.unwrap()))
                    } else {
                        None
                    })
                }
                fn process_events<F>(
                    &mut self,
                    _: Readiness,
                    _: Token,
                    mut callback: F,
                ) -> std::io::Result<PostAction>
                where
                    F: FnMut((), &mut ()),
                {
                    callback((), &mut ());
                    Ok(PostAction::Continue)
                }
            }
        }
    };
}
synthetic_source!(modern, calloop);
synthetic_source!(legacy, calloop_legacy);

#[derive(Clone, Default)]
struct Output(Arc<Mutex<Vec<u8>>>);
impl Write for Output {
    fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(data);
        Ok(data.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

thread_local! { static LOG_OUTPUT: RefCell<Option<Output>> = const { RefCell::new(None) }; }
struct LegacyLogger;
impl log::Log for LegacyLogger {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        metadata.level() <= log::Level::Warn && metadata.target().starts_with("calloop")
    }
    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            LOG_OUTPUT.with(|out| {
                if let Some(writer) = out.borrow_mut().as_mut() {
                    writeln!(
                        writer,
                        "{} {} {}",
                        record.level(),
                        record.file().unwrap_or("unknown"),
                        record.args()
                    )
                    .unwrap();
                }
            });
        }
    }
    fn flush(&self) {}
}

fn capture_warnings(run: impl FnOnce()) -> String {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        log::set_logger(&LegacyLogger).unwrap();
        log::set_max_level(log::LevelFilter::Warn);
    });
    let output = Output::default();
    LOG_OUTPUT.with(|out| *out.borrow_mut() = Some(output.clone()));
    let writer = output.clone();
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .without_time()
        .with_ansi(false)
        .with_file(true)
        .with_writer(move || writer.clone())
        .finish();
    tracing::subscriber::with_default(subscriber, run);
    LOG_OUTPUT.with(|out| *out.borrow_mut() = None);
    let result = String::from_utf8(output.0.lock().unwrap().clone()).unwrap();
    result
}

struct Harness {
    event_loop: EventLoop<'static, State>,
    state: State,
    requests: Rc<Cell<usize>>,
    _display: Display<State>,
}
impl Harness {
    fn new() -> Self {
        let event_loop = EventLoop::try_new().unwrap();
        let display = Display::new().unwrap();
        let requests = Rc::new(Cell::new(0));
        let counter = requests.clone();
        let state = State::new(
            display.handle(),
            event_loop.handle(),
            Box::new(move || counter.set(counter.get() + 1)),
            960,
            640,
        )
        .unwrap();
        Self {
            event_loop,
            state,
            requests,
            _display: display,
        }
    }
    fn schedule_due(&mut self) {
        self.state.last_frame = Instant::now() - Duration::from_millis(17);
        self.state.schedule_frame();
    }
    fn dispatch(&mut self, timeout: Duration) {
        self.event_loop
            .dispatch(Some(timeout), &mut self.state)
            .unwrap();
    }
    fn assert_retired(&self, token: &calloop::RegistrationToken) {
        assert!(self.state.frame_timer.is_none());
        assert!(self.event_loop.handle().update(token).is_err());
    }
}

#[test]
fn reported_slot_three_warning_is_reproduced_in_winit_dependency_version() {
    struct Data {
        handle: calloop_legacy::LoopHandle<'static, Self>,
        old: Option<calloop_legacy::RegistrationToken>,
        replacement: Option<calloop_legacy::RegistrationToken>,
        repeating: bool,
        repeats: usize,
    }
    let warnings = capture_warnings(|| {
        let mut event_loop = calloop_legacy::EventLoop::<Data>::try_new().unwrap();
        let handle = event_loop.handle();
        // Winit's fixed registrations: Wayland dispatcher, user channel, ping.
        handle
            .insert_source(legacy::Source::new(true), |_, _, data| {
                data.repeating = false; // key release invalidates repeat before removal
                data.handle.remove(data.old.take().unwrap());
                data.replacement = Some(
                    data.handle
                        .insert_source(
                            calloop_legacy::timer::Timer::from_duration(Duration::from_secs(30)),
                            |_, _, data| {
                                data.repeats += 1;
                                calloop_legacy::timer::TimeoutAction::Drop
                            },
                        )
                        .unwrap(),
                );
            })
            .unwrap();
        for _ in 0..2 {
            handle
                .insert_source(legacy::Source::new(false), |_, _, _| {})
                .unwrap();
        }
        let old = handle
            .insert_source(calloop_legacy::timer::Timer::immediate(), |_, _, data| {
                if data.repeating {
                    data.repeats += 1;
                }
                calloop_legacy::timer::TimeoutAction::Drop
            })
            .unwrap();
        assert!(format!("{old:?}").contains("id: 3"));
        let mut data = Data {
            handle: handle.clone(),
            old: Some(old),
            replacement: None,
            repeating: true,
            repeats: 0,
        };
        event_loop
            .dispatch(Some(Duration::ZERO), &mut data)
            .unwrap();
        let replacement = data.replacement.unwrap();
        assert_ne!(old, replacement);
        assert!(format!("{replacement:?}").contains("id: 3"));
        assert!(format!("{replacement:?}").contains("version: 1"));
        assert_eq!(data.repeats, 0); // expired old event cannot hit the replacement
        assert!(handle.update(&old).is_err());
        assert!(handle.update(&replacement).is_ok());
        handle.remove(replacement);
    });
    assert!(
        warnings.contains("calloop-0.13.0/src/loop_logic.rs"),
        "{warnings}"
    );
    assert!(
        warnings.contains("non-existence source: TokenInner { id: 3, version: 0, sub_id: 0 }"),
        "{warnings}"
    );
    println!("Confirmed pinned backend warning: Calloop 0.13, keyboard-repeat slot 3; stale generation rejected, no replacement callback.");
}

#[test]
fn old_frame_removal_pattern_reproduces_distinct_modern_warning() {
    let warnings = capture_warnings(|| {
        let mut h = Harness::new();
        h.event_loop
            .handle()
            .insert_source(modern::Source::new(true), |_, _, state| {
                // The former Redraw implementation, intentionally isolated here.
                if let Some(token) = state.frame_timer.take() {
                    state.handle.remove(token);
                }
                state.begin_frame();
            })
            .unwrap();
        h.schedule_due();
        h.dispatch(Duration::ZERO);
        assert_eq!(h.requests.get(), 0);
    });
    assert!(
        warnings.contains("calloop-0.14.5/src/loop_logic.rs"),
        "{warnings}"
    );
    assert!(warnings.contains("non-existent source"), "{warnings}");
}

#[test]
fn repeated_scheduling_owns_one_timer_and_one_redraw_request() {
    let warnings = capture_warnings(|| {
        let mut h = Harness::new();
        h.schedule_due();
        let token = h.state.frame_timer.unwrap();
        for _ in 0..1000 {
            h.state.schedule_frame();
            assert_eq!(h.state.frame_timer, Some(token));
        }
        h.dispatch(Duration::ZERO);
        assert_eq!(h.requests.get(), 1);
        h.assert_retired(&token);
        // The host has not delivered Redraw yet: don't send repeated requests.
        for _ in 0..100 {
            h.state.schedule_frame();
        }
        assert!(h.state.frame_timer.is_none());
        h.state.begin_frame();
        h.dispatch(Duration::ZERO);
        assert!(!h.state.frame_scheduled);
        assert_eq!(h.requests.get(), 1);
    });
    assert!(warnings.is_empty(), "{warnings}");
}

#[test]
fn early_redraw_retires_queued_timer_without_warning_or_extra_redraw() {
    let warnings = capture_warnings(|| {
        let mut h = Harness::new();
        h.event_loop
            .handle()
            .insert_source(modern::Source::new(true), |_, _, state| state.begin_frame())
            .unwrap();
        h.schedule_due();
        let token = h.state.frame_timer.unwrap();
        h.dispatch(Duration::ZERO); // early redraw + expired timer in same batch
        h.assert_retired(&token);
        assert!(!h.state.frame_scheduled);
        assert_eq!(h.requests.get(), 0);
        h.schedule_due(); // source slot can now be reused with a fresh generation
        assert_ne!(h.state.frame_timer, Some(token));
        h.dispatch(Duration::ZERO);
        assert_eq!(h.requests.get(), 1);
    });
    assert!(warnings.is_empty(), "{warnings}");
}

#[test]
fn fresh_demand_after_early_redraw_reuses_timer_and_preserves_new_deadline() {
    let warnings = capture_warnings(|| {
        let mut h = Harness::new();
        let same_timer = Rc::new(Cell::new(false));
        let observed = same_timer.clone();
        h.event_loop
            .handle()
            .insert_source(modern::Source::new(true), move |_, _, state| {
                let timer = state.frame_timer;
                state.begin_frame();
                state.schedule_frame();
                observed.set(state.frame_timer == timer);
            })
            .unwrap();
        h.schedule_due();
        let token = h.state.frame_timer.unwrap();
        h.dispatch(Duration::ZERO);
        assert!(same_timer.get());
        assert_eq!(h.requests.get(), 0); // pacing moved with the early redraw
        assert_eq!(h.state.frame_timer, Some(token));
        h.dispatch(Duration::from_millis(100));
        assert_eq!(h.requests.get(), 1);
        h.assert_retired(&token);
    });
    assert!(warnings.is_empty(), "{warnings}");
}

#[test]
fn rapid_move_resize_and_drag_motion_preserve_redraws_without_idle_timer() {
    let warnings = capture_warnings(|| {
        let mut h = Harness::new();
        let id = h.state.policy.map().unwrap();
        for _ in 0..64 {
            h.state.policy.adjust_geometry(id, 20, -20, false).unwrap();
            h.state.policy.adjust_geometry(id, -20, 20, true).unwrap();
            h.state.policy.start_drag(id, DragKind::Move, (0., 0.));
            for n in 0..32 {
                h.state.policy.motion((f64::from(n), 10.));
                h.state.schedule_frame();
            }
            h.state.policy.end_drag().unwrap();
            // Let the already-armed finite timer fire; request once per burst.
            let token = h.state.frame_timer.unwrap();
            h.dispatch(Duration::from_millis(100));
            h.assert_retired(&token);
            h.state.begin_frame();
            h.state.policy.wm.check_invariants().unwrap();
        }
        assert_eq!(h.requests.get(), 64);
        assert!(!h.state.frame_scheduled);
        assert!(h.state.frame_timer.is_none());
    });
    assert!(warnings.is_empty(), "{warnings}");
}

#[test]
fn shutdown_with_queued_timer_does_not_request_or_register_another_frame() {
    let warnings = capture_warnings(|| {
        let mut h = Harness::new();
        h.event_loop
            .handle()
            .insert_source(modern::Source::new(true), |_, _, state| {
                state.running = false;
                state.begin_frame();
                state.schedule_frame();
            })
            .unwrap();
        h.schedule_due();
        let token = h.state.frame_timer.unwrap();
        h.dispatch(Duration::ZERO);
        h.assert_retired(&token);
        assert!(!h.state.frame_scheduled);
        assert_eq!(h.requests.get(), 0);
        h.state.schedule_frame();
        assert!(h.state.frame_timer.is_none());
    });
    assert!(warnings.is_empty(), "{warnings}");
}
