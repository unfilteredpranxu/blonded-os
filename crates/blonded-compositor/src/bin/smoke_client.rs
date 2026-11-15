//! Tiny real xdg-shell/wl_shm client. Safe file writes, no mmap or toolkit required.
use calloop::{
    generic::Generic,
    timer::{TimeoutAction, Timer},
    EventLoop, Interest, Mode, PostAction,
};
use std::{io::Write, os::fd::AsFd, time::Duration};
use wayland_client::{
    delegate_noop,
    protocol::{
        wl_buffer, wl_callback, wl_compositor, wl_keyboard, wl_pointer, wl_registry, wl_seat,
        wl_shm, wl_shm_pool, wl_surface,
    },
    Connection, Dispatch, QueueHandle,
};
use wayland_protocols::xdg::shell::client::{xdg_surface, xdg_toplevel, xdg_wm_base};

#[derive(Default)]
struct Client {
    compositor: Option<wl_compositor::WlCompositor>,
    shm: Option<wl_shm::WlShm>,
    shell: Option<xdg_wm_base::XdgWmBase>,
    surface: Option<wl_surface::WlSurface>,
    xdg: Option<xdg_surface::XdgSurface>,
    top: Option<xdg_toplevel::XdgToplevel>,
    size: (i32, i32),
    title: String,
    done: bool,
    readable: bool,
    error: Option<String>,
    frame_count: u32,
    paused: bool,
    configured: bool,
}

impl Client {
    fn draw(&mut self, qh: &QueueHandle<Self>) -> Result<(), Box<dyn std::error::Error>> {
        let (width, height) = self.size;
        if !(1..=2048).contains(&width) || !(1..=2048).contains(&height) {
            return Err("unsupported smoke buffer size".into());
        }
        let length = width * height * 4;
        let mut file = tempfile::tempfile()?;
        let mut pixels = Vec::with_capacity(length as usize);
        for y in 0..height {
            for x in 0..width {
                let border = x < 8 || y < 8 || x >= width - 8 || y >= height - 8;
                let color: u32 = if border {
                    0xffeeeeee
                } else if self.title.ends_with('B') {
                    if (x / 32 + y / 32) % 2 == 0 {
                        0xff266fa3
                    } else {
                        0xff16446b
                    }
                } else if (x / 32 + y / 32) % 2 == 0 {
                    0xffb64e33
                } else {
                    0xff713221
                };
                pixels.extend_from_slice(&color.to_ne_bytes());
            }
        }
        file.write_all(&pixels)?;
        let shm = self.shm.as_ref().ok_or("wl_shm unavailable")?;
        let pool = shm.create_pool(file.as_fd(), length, qh, ());
        let buffer = pool.create_buffer(
            0,
            width,
            height,
            width * 4,
            wl_shm::Format::Argb8888,
            qh,
            (),
        );
        pool.destroy();
        let surface = self.surface.as_ref().ok_or("surface unavailable")?;
        surface.attach(Some(&buffer), 0, 0);
        surface.damage(0, 0, width, height);
        surface.frame(qh, ());
        surface.commit();
        eprintln!(
            "SMOKE buffer_commit title={:?} size={}x{}",
            self.title, width, height
        );
        Ok(())
    }
}

impl Dispatch<wl_registry::WlRegistry, ()> for Client {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            match interface.as_str() {
                "wl_compositor" => {
                    state.compositor = Some(registry.bind(name, version.min(4), qh, ()))
                }
                "wl_shm" => state.shm = Some(registry.bind(name, 1, qh, ())),
                "xdg_wm_base" => state.shell = Some(registry.bind(name, version.min(6), qh, ())),
                "wl_seat" => {
                    let _: wl_seat::WlSeat = registry.bind(name, version.min(7), qh, ());
                }
                _ => {}
            }
        }
    }
}
impl Dispatch<xdg_wm_base::XdgWmBase, ()> for Client {
    fn event(
        _: &mut Self,
        proxy: &xdg_wm_base::XdgWmBase,
        event: xdg_wm_base::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let xdg_wm_base::Event::Ping { serial } = event {
            proxy.pong(serial);
        }
    }
}
impl Dispatch<xdg_surface::XdgSurface, ()> for Client {
    fn event(
        state: &mut Self,
        proxy: &xdg_surface::XdgSurface,
        event: xdg_surface::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let xdg_surface::Event::Configure { serial } = event {
            proxy.ack_configure(serial);
            state.configured = true;
            if state.paused {
                return;
            }
            if let Err(error) = state.draw(qh) {
                state.error = Some(error.to_string());
                state.done = true;
            }
        }
    }
}
impl Dispatch<xdg_toplevel::XdgToplevel, ()> for Client {
    fn event(
        state: &mut Self,
        _: &xdg_toplevel::XdgToplevel,
        event: xdg_toplevel::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            xdg_toplevel::Event::Configure { width, height, .. } if width > 0 && height > 0 => {
                state.size = (width, height)
            }
            xdg_toplevel::Event::Close => state.done = true,
            _ => {}
        }
    }
}
impl Dispatch<wl_callback::WlCallback, ()> for Client {
    fn event(
        state: &mut Self,
        _: &wl_callback::WlCallback,
        _: wl_callback::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        state.frame_count += 1;
        eprintln!(
            "SMOKE frame_callback title={:?} count={}",
            state.title, state.frame_count
        );
    }
}
impl Dispatch<wl_buffer::WlBuffer, ()> for Client {
    fn event(
        _: &mut Self,
        proxy: &wl_buffer::WlBuffer,
        event: wl_buffer::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let wl_buffer::Event::Release = event {
            proxy.destroy();
        }
    }
}
impl Dispatch<wl_seat::WlSeat, ()> for Client {
    fn event(
        _: &mut Self,
        proxy: &wl_seat::WlSeat,
        event: wl_seat::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_seat::Event::Capabilities {
            capabilities: wayland_client::WEnum::Value(capabilities),
        } = event
        {
            if capabilities.contains(wl_seat::Capability::Pointer) {
                proxy.get_pointer(qh, ());
            }
            if capabilities.contains(wl_seat::Capability::Keyboard) {
                proxy.get_keyboard(qh, ());
            }
        }
    }
}
impl Dispatch<wl_pointer::WlPointer, ()> for Client {
    fn event(
        state: &mut Self,
        _: &wl_pointer::WlPointer,
        event: wl_pointer::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            wl_pointer::Event::Enter { .. } => {
                eprintln!("SMOKE pointer_enter title={:?}", state.title)
            }
            wl_pointer::Event::Button {
                button,
                state: button_state,
                ..
            } => eprintln!(
                "SMOKE button title={:?} code={button} state={button_state:?}",
                state.title
            ),
            _ => {}
        }
    }
}
impl Dispatch<wl_keyboard::WlKeyboard, ()> for Client {
    fn event(
        state: &mut Self,
        _: &wl_keyboard::WlKeyboard,
        event: wl_keyboard::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            wl_keyboard::Event::Enter { .. } => {
                eprintln!("SMOKE keyboard_enter title={:?}", state.title)
            }
            wl_keyboard::Event::Key {
                key,
                state: key_state,
                ..
            } => eprintln!(
                "SMOKE key title={:?} code={key} state={key_state:?}",
                state.title
            ),
            _ => {}
        }
    }
}
delegate_noop!(Client: ignore wl_compositor::WlCompositor);
delegate_noop!(Client: ignore wl_shm::WlShm);
delegate_noop!(Client: ignore wl_shm_pool::WlShmPool);
delegate_noop!(Client: ignore wl_surface::WlSurface);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let connection = Connection::connect_to_env()?;
    let mut queue = connection.new_event_queue();
    let qh = queue.handle();
    connection.display().get_registry(&qh, ());
    let mut state = Client {
        title: args
            .first()
            .cloned()
            .unwrap_or_else(|| "Blonded smoke A".into()),
        size: (420, 280),
        ..Default::default()
    };
    queue.roundtrip(&mut state)?;
    let surface = state
        .compositor
        .as_ref()
        .ok_or("wl_compositor unavailable")?
        .create_surface(&qh, ());
    let xdg = state
        .shell
        .as_ref()
        .ok_or("xdg_wm_base unavailable")?
        .get_xdg_surface(&surface, &qh, ());
    let top = xdg.get_toplevel(&qh, ());
    top.set_title(state.title.clone());
    top.set_app_id("blonded-m01b-smoke".into());
    surface.commit();
    state.surface = Some(surface);
    state.xdg = Some(xdg);
    state.top = Some(top);
    let mut event_loop = EventLoop::<Client>::try_new()?;
    event_loop.handle().insert_source(
        Generic::new(
            connection.as_fd().try_clone_to_owned()?,
            Interest::READ,
            Mode::Level,
        ),
        |_, _, state| {
            state.readable = true;
            Ok(PostAction::Continue)
        },
    )?;
    if args.iter().any(|arg| arg == "--lifecycle") {
        let mut stage = 0;
        let lifecycle_qh = qh.clone();
        event_loop.handle().insert_source(
            Timer::from_duration(Duration::from_secs(2)),
            move |_, _, state| {
                stage += 1;
                match stage {
                    1 => {
                        state.paused = true;
                        state.configured = false;
                        let surface = state.surface.as_ref().expect("created surface");
                        surface.attach(None, 0, 0);
                        surface.commit();
                        eprintln!("SMOKE unmap");
                    }
                    2 => {
                        state.paused = false;
                        if state.configured {
                            if let Err(error) = state.draw(&lifecycle_qh) {
                                state.error = Some(error.to_string());
                                state.done = true;
                            }
                        } else {
                            state.surface.as_ref().expect("created surface").commit();
                        }
                        eprintln!("SMOKE remap_requested");
                    }
                    _ => state.done = true,
                }
                if state.done {
                    TimeoutAction::Drop
                } else {
                    TimeoutAction::ToDuration(Duration::from_secs(1))
                }
            },
        )?;
    }
    while !state.done {
        queue.dispatch_pending(&mut state)?;
        connection.flush()?;
        let guard = queue.prepare_read();
        event_loop.dispatch(None, &mut state)?;
        if let Some(guard) = guard {
            if state.readable {
                guard.read()?;
                state.readable = false;
            }
        }
    }
    if let Some(top) = state.top.take() {
        top.destroy();
    }
    if let Some(xdg) = state.xdg.take() {
        xdg.destroy();
    }
    if let Some(surface) = state.surface.take() {
        surface.destroy();
    }
    connection.flush()?;
    if let Some(error) = state.error {
        return Err(error.into());
    }
    if state.frame_count == 0 {
        return Err("no compositor frame callback observed".into());
    }
    eprintln!("SMOKE exit frames={}", state.frame_count);
    Ok(())
}
