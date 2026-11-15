//! M01-B: a manually launched Wayland application, never a host session backend.
mod handlers;
mod input;
mod policy;
mod state;

use smithay::{
    backend::{
        egl::context::GlAttributes,
        renderer::{
            damage::OutputDamageTracker, element::surface::WaylandSurfaceRenderElement,
            gles::GlesRenderer, ExportMem, Frame, Renderer, TextureMapping,
        },
        winit::{self, WinitEvent},
    },
    output::{Mode, Output, PhysicalProperties, Subpixel},
    reexports::{
        calloop::{
            generic::Generic,
            signals::{Signal, Signals},
            timer::{TimeoutAction, Timer},
            EventLoop, Interest, Mode as PollMode, PostAction,
        },
        wayland_server::{Display, ListeningSocket},
    },
    utils::{Rectangle, Transform},
};
use state::{ClientState, State};
use std::{
    os::{fd::AsFd, unix::fs::PermissionsExt},
    sync::Arc,
    time::Duration,
};

#[derive(Default)]
struct Options {
    smoke: bool,
    exit_after: Option<u64>,
    capture: Option<std::path::PathBuf>,
}

fn options() -> Result<Option<Options>, Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let mut options = Options::default();
    let mut nested = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--nested" => nested = true,
            "--smoke-test" => options.smoke = true,
            "--exit-after" => {
                options.exit_after = Some(args.next().ok_or("--exit-after needs seconds")?.parse()?)
            }
            "--capture" => {
                options.capture = Some(args.next().ok_or("--capture needs a new .ppm path")?.into())
            }
            "--help" | "-h" => {
                println!("blonded-compositor --nested [--smoke-test] [--exit-after SECONDS] [--capture NEW.ppm]\nAlt+left/right drag: move/resize; Alt+arrows: move; Alt+Shift+arrows: resize; Alt+Tab: focus; Alt+F4: close; Alt+Escape: stop.");
                return Ok(None);
            }
            _ => return Err(format!("unsupported argument: {arg}").into()),
        }
    }
    if !nested {
        return Err("explicit --nested is required (use --help)".into());
    }
    if std::env::var_os("WAYLAND_DISPLAY").is_none() {
        return Err("a parent Wayland session is required".into());
    }
    Ok(Some(options))
}

fn run(options: Options) -> Result<(), Box<dyn std::error::Error>> {
    // Force the Wayland client backend; never fall back to X11 or a direct device backend.
    std::env::set_var("WINIT_UNIX_BACKEND", "wayland");
    let mut event_loop: EventLoop<State> = EventLoop::try_new()?;
    let mut display: Display<State> = Display::new()?;
    let attributes = ::winit::window::Window::default_attributes()
        .with_title(format!(
            "Blonded M01-B — nested proof {}",
            std::process::id()
        ))
        .with_inner_size(::winit::dpi::LogicalSize::new(960., 640.))
        .with_min_inner_size(::winit::dpi::LogicalSize::new(320., 240.));
    let (backend, winit) = winit::init_from_attributes_with_gl_attr::<GlesRenderer>(
        attributes,
        GlAttributes {
            version: (3, 0),
            profile: None,
            debug: false,
            vsync: true,
        },
    )?;
    let size = backend.window_size();
    let backend = std::rc::Rc::new(std::cell::RefCell::new(backend));
    let redraw_backend = backend.clone();
    let mut state = State::new(
        display.handle(),
        event_loop.handle(),
        Box::new(move || redraw_backend.borrow().window().request_redraw()),
        size.w as u32,
        size.h as u32,
    )?;
    let output = Output::new(
        "blonded-nested".into(),
        PhysicalProperties {
            size: (0, 0).into(),
            subpixel: Subpixel::Unknown,
            make: "Blonded".into(),
            model: "Nested proof".into(),
        },
    );
    let _global = output.create_global::<State>(&display.handle());
    // Protocol nominal refresh only, not a measured host refresh/FPS claim.
    let mode = Mode {
        size,
        refresh: 60_000,
    };
    output.change_current_state(
        Some(mode),
        Some(Transform::Flipped180),
        None,
        Some((0, 0).into()),
    );
    output.set_preferred(mode);
    state.space.map_output(&output, (0, 0));
    let mut damage_tracker = OutputDamageTracker::from_output(&output);

    // 0700 directory protects this absolute socket; no host WAYLAND_DISPLAY change.
    let socket_dir = tempfile::Builder::new()
        .prefix("blonded-m01b-")
        .permissions(std::fs::Permissions::from_mode(0o700))
        .tempdir_in("/tmp")?;
    let socket_path = socket_dir.path().join("wayland");
    let socket = ListeningSocket::bind_absolute(socket_path.clone())?;
    event_loop.handle().insert_source(
        Generic::new(socket, Interest::READ, PollMode::Level),
        |_, socket, state| {
            while let Some(stream) = socket.accept()? {
                if let Err(error) = state
                    .display_handle
                    .insert_client(stream, Arc::new(ClientState::default()))
                {
                    state.fail(error);
                }
            }
            Ok(PostAction::Continue)
        },
    )?;
    // Poll a duplicated fd; keep Display owned here, avoiding unsafe Generic::get_mut.
    event_loop.handle().insert_source(
        Generic::new(
            display.as_fd().try_clone_to_owned()?,
            Interest::READ,
            PollMode::Level,
        ),
        |_, _, _| Ok(PostAction::Continue),
    )?;
    event_loop.handle().insert_source(
        Signals::new(&[Signal::SIGINT, Signal::SIGTERM])?,
        |_, _, state| state.running = false,
    )?;
    if let Some(seconds) = options.exit_after {
        event_loop.handle().insert_source(
            Timer::from_duration(Duration::from_secs(seconds)),
            |_, _, state| {
                state.running = false;
                TimeoutAction::Drop
            },
        )?;
    }
    let mut capture = options.capture;
    event_loop.handle().insert_source(winit, move |event, _, state| {
        match event {
            WinitEvent::Resized { size, .. } if size.w >= 320 && size.h >= 240 => {
                if let Err(error) = state.policy.configure(size.w as u32, size.h as u32) { state.fail(error); return; }
                output.change_current_state(Some(Mode { size, refresh: 60_000 }), None, None, None);
                state.sync_policy();
            },
            WinitEvent::Input(event) => state.process_input(event),
            WinitEvent::Focus(focus) => {
                state.host_focus = focus;
                if !focus {
                    state.lose_input_focus();
                }
                state.sync_policy();
            },
            WinitEvent::Redraw => {
                let mut backend = backend.borrow_mut();
                if let Some(timer) = state.frame_timer.take() { state.handle.remove(timer); }
                state.frame_scheduled = false;
                state.last_frame = std::time::Instant::now();
                state.flush_pointer();
                if state.policy.dragging() {
                    if let Err(error) = state.policy.flush_motion() { state.fail(error); return; }
                    // Updating the snapshot does not schedule another idle frame.
                    state.frame_scheduled = true;
                    state.sync_policy();
                    state.frame_scheduled = false;
                }
                let result = (|| -> Result<(), Box<dyn std::error::Error>> {
                    let damage = Rectangle::from_size(backend.window_size());
                    let mut captured = false;
                    {
                        let (renderer, mut framebuffer) = backend.bind()?;
                        smithay::desktop::space::render_output::<_, WaylandSurfaceRenderElement<GlesRenderer>, _, _>(
                            &output, renderer, &mut framebuffer, 1., 0, [&state.space], &[], &mut damage_tracker, [0.08, 0.08, 0.08, 1.],
                        )?;
                        if state.policy.wm.windows().len() > 0 {
                            if let Some(path) = capture.take() {
                                use std::io::Write;
                                let size = framebuffer_size(&state.policy);
                                let mapping = renderer.copy_framebuffer(&framebuffer, Rectangle::from_size(size), smithay::backend::allocator::Fourcc::Abgr8888)?;
                                let flipped = mapping.flipped();
                                let bytes = renderer.map_texture(&mapping)?;
                                // Only this compositor's framebuffer is captured, never the host desktop.
                                let mut file = std::io::BufWriter::new(std::fs::OpenOptions::new().write(true).create_new(true).open(path)?);
                                write!(file, "P6\n{} {}\n255\n", size.w, size.h)?;
                                for y in 0..size.h as usize {
                                    let row = if flipped { size.h as usize - 1 - y } else { y };
                                    for x in 0..size.w as usize {
                                        let offset = (row * size.w as usize + x) * 4;
                                        file.write_all(&bytes[offset..offset + 3])?;
                                    }
                                }
                                file.flush()?;
                                captured = true;
                                eprintln!("CAPTURE own_framebuffer={}x{}", size.w, size.h);
                            }
                        }
                    }
                    // map_texture makes the context surfaceless. bind() returns
                    // a target without making it current; an empty render/finish
                    // restores the EGL window surface without altering pixels.
                    if captured {
                        let (renderer, mut framebuffer) = backend.bind()?;
                        renderer.render(&mut framebuffer, damage.size, Transform::Normal)?.finish()?.wait()?;
                    }
                    backend.submit(Some(&[damage]))?;
                    if !state.rendered_client && state.policy.wm.windows().len() > 0 {
                        eprintln!("RENDER submitted_with_client=true (host presentation still requires visual confirmation)");
                        state.rendered_client = true;
                    }
                    for window in state.space.elements() {
                        window.send_frame(&output, state.start.elapsed(), Some(Duration::ZERO), |_, _| Some(output.clone()));
                    }
                    Ok(())
                })();
                if let Err(error) = result { state.fail(error); }
            },
            WinitEvent::CloseRequested => state.running = false,
            _ => {},
        }
    })?;
    eprintln!(
        "READY socket={} backend=Wayland/EGL/GLES3 smithay=0.7.0",
        socket_path.display()
    );
    let mut children = Children::default();
    if options.smoke {
        let client = std::env::current_exe()?.with_file_name("blonded-smoke-client");
        for title in ["Blonded client A", "Blonded client B"] {
            children.0.push(
                std::process::Command::new(&client)
                    .env("WAYLAND_DISPLAY", &socket_path)
                    .env_remove("WAYLAND_SOCKET")
                    .arg(title)
                    .spawn()?,
            );
        }
    }
    state.schedule_frame();
    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        while state.running {
            event_loop.dispatch(None, &mut state)?;
            display.dispatch_clients(&mut state)?;
            state.reap();
            display.flush_clients()?;
        }
        if let Some(error) = state.error.take() {
            return Err(error.into());
        }
        Ok(())
    })();
    for entry in &state.managed {
        if let Some(top) = entry.window.toplevel() {
            top.send_close();
        }
    }
    let flush_result = display.flush_clients();
    state.lose_input_focus();
    for index in (0..state.managed.len()).rev() {
        state.unmap(index);
    }
    state.managed.clear();
    eprintln!(
        "STOP windows={} invariants={:?}",
        state.policy.wm.windows().len(),
        state.policy.wm.check_invariants()
    );
    drop(display);
    drop(event_loop); // closes listener and removes socket before private directory
    drop(socket_dir);
    drop(children);
    result.and(flush_result.map_err(Into::into))
}

#[derive(Default)]
struct Children(Vec<std::process::Child>);

impl Drop for Children {
    fn drop(&mut self) {
        for child in &mut self.0 {
            if !matches!(child.try_wait(), Ok(Some(_))) {
                let _ = child.kill();
            }
            let _ = child.wait();
        }
    }
}

fn framebuffer_size(policy: &policy::Policy) -> smithay::utils::Size<i32, smithay::utils::Buffer> {
    let screen = policy.wm.config().screen;
    (screen.width() as i32, screen.height() as i32).into()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "warn,smithay::backend::renderer::gles=info".into()),
        )
        .init();
    if let Some(options) = options()? {
        run(options)?;
    }
    Ok(())
}
