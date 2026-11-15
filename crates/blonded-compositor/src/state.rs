use std::time::{Duration, Instant};

use blonded_wm::WindowId;
use smithay::{
    desktop::{Space, Window, WindowSurfaceType},
    input::{Seat, SeatState},
    reexports::{
        calloop::{
            timer::{TimeoutAction, Timer},
            LoopHandle, RegistrationToken,
        },
        wayland_server::{
            backend::{ClientData, ClientId, DisconnectReason},
            protocol::wl_surface::WlSurface,
            DisplayHandle,
        },
    },
    utils::{IsAlive, Logical, Point, SERIAL_COUNTER},
    wayland::{
        compositor::{CompositorClientState, CompositorState},
        output::OutputManagerState,
        selection::data_device::DataDeviceState,
        shell::xdg::XdgShellState,
        shm::ShmState,
    },
};

use crate::policy::Policy;

pub struct Managed {
    pub window: Window,
    pub id: Option<WindowId>,
    pub configured: Option<(i32, i32, bool)>,
}

pub struct State {
    pub display_handle: DisplayHandle,
    pub compositor_state: CompositorState,
    pub xdg_shell_state: XdgShellState,
    pub shm_state: ShmState,
    pub _output_manager_state: OutputManagerState,
    pub seat_state: SeatState<Self>,
    pub data_device_state: DataDeviceState,
    pub seat: Seat<Self>,
    pub space: Space<Window>,
    pub managed: Vec<Managed>,
    pub policy: Policy,
    pub running: bool,
    pub error: Option<String>,
    pub host_focus: bool,
    pub pointer_pos: Point<f64, Logical>,
    pub pending_pointer: Option<u32>,
    pub consumed_buttons: Vec<u32>,
    pub forwarded_buttons: Vec<u32>,
    pub drag_button: Option<u32>,
    pub consumed_keys: Vec<smithay::backend::input::Keycode>,
    pub handle: LoopHandle<'static, Self>,
    pub request_redraw: Box<dyn Fn()>,
    pub frame_scheduled: bool,
    pub frame_timer: Option<RegistrationToken>,
    pub last_frame: Instant,
    pub start: Instant,
    pub rendered_client: bool,
}

impl State {
    pub fn new(
        display_handle: DisplayHandle,
        handle: LoopHandle<'static, Self>,
        request_redraw: Box<dyn Fn()>,
        width: u32,
        height: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let compositor_state = CompositorState::new::<Self>(&display_handle);
        let xdg_shell_state = XdgShellState::new::<Self>(&display_handle);
        let shm_state = ShmState::new::<Self>(&display_handle, vec![]);
        let output_manager_state = OutputManagerState::new_with_xdg_output::<Self>(&display_handle);
        let data_device_state = DataDeviceState::new::<Self>(&display_handle);
        let mut seat_state = SeatState::new();
        let mut seat = seat_state.new_wl_seat(&display_handle, "blonded-nested");
        seat.add_keyboard(Default::default(), 200, 25)?;
        seat.add_pointer();
        Ok(Self {
            display_handle,
            compositor_state,
            xdg_shell_state,
            shm_state,
            _output_manager_state: output_manager_state,
            seat_state,
            data_device_state,
            seat,
            space: Space::default(),
            managed: Vec::new(),
            policy: Policy::new(width, height)?,
            running: true,
            error: None,
            host_focus: true,
            pointer_pos: (0., 0.).into(),
            pending_pointer: None,
            consumed_buttons: Vec::new(),
            forwarded_buttons: Vec::new(),
            drag_button: None,
            consumed_keys: Vec::new(),
            handle,
            request_redraw,
            frame_scheduled: false,
            frame_timer: None,
            last_frame: Instant::now(),
            start: Instant::now(),
            rendered_client: false,
        })
    }

    pub fn fail(&mut self, error: impl std::fmt::Display) {
        self.error = Some(error.to_string());
        self.running = false;
    }

    /// One pending frame timer, no idle redraw loop and no permanent timer.
    pub fn schedule_frame(&mut self) {
        if self.frame_scheduled {
            return;
        }
        self.frame_scheduled = true;
        let delay = Duration::from_millis(16).saturating_sub(self.last_frame.elapsed());
        match self
            .handle
            .insert_source(Timer::from_duration(delay), |_, _, state| {
                state.frame_timer = None;
                (state.request_redraw)();
                TimeoutAction::Drop
            }) {
            Ok(token) => self.frame_timer = Some(token),
            Err(error) => self.fail(error),
        }
    }

    pub fn surface_under(
        &self,
        pos: Point<f64, Logical>,
    ) -> Option<(WlSurface, Point<f64, Logical>)> {
        self.space
            .element_under(pos)
            .and_then(|(window, location)| {
                window
                    .surface_under(pos - location.to_f64(), WindowSurfaceType::ALL)
                    .map(|(surface, point)| (surface, (point + location).to_f64()))
            })
    }

    pub fn id_under(&self) -> Option<WindowId> {
        let (window, _) = self.space.element_under(self.pointer_pos)?;
        self.managed
            .iter()
            .find(|entry| &entry.window == window)?
            .id
    }

    /// Publish a complete successful WM snapshot to Smithay; never place windows independently.
    pub fn sync_policy(&mut self) {
        let stacking = self
            .policy
            .wm
            .workspace(self.policy.wm.active_workspace())
            .expect("fixed workspace")
            .stacking()
            .to_vec();
        for id in stacking {
            let Some(entry) = self.managed.iter_mut().find(|entry| entry.id == Some(id)) else {
                continue;
            };
            let geometry = self
                .policy
                .wm
                .window(id)
                .expect("stacking invariant")
                .geometry();
            let active = self.host_focus && self.policy.wm.focused() == Some(id);
            self.space
                .map_element(entry.window.clone(), (geometry.x(), geometry.y()), false);
            self.space.raise_element(&entry.window, false);
            let configuration = (geometry.width() as i32, geometry.height() as i32, active);
            if entry.configured != Some(configuration) {
                let toplevel = entry.window.toplevel().expect("Wayland window");
                toplevel.with_pending_state(|state| {
                    state.size = Some((configuration.0, configuration.1).into());
                    if active { state.states.set(smithay::reexports::wayland_protocols::xdg::shell::server::xdg_toplevel::State::Activated); }
                    else { state.states.unset(smithay::reexports::wayland_protocols::xdg::shell::server::xdg_toplevel::State::Activated); }
                });
                toplevel.send_pending_configure();
                entry.configured = Some(configuration);
            }
        }
        let focus = self
            .managed
            .iter()
            .find(|entry| {
                self.host_focus && entry.id.is_some() && entry.id == self.policy.wm.focused()
            })
            .and_then(|entry| entry.window.toplevel())
            .map(|top| top.wl_surface().clone());
        let keyboard = self.seat.get_keyboard().expect("keyboard initialized");
        if keyboard.current_focus() != focus {
            keyboard.set_focus(self, focus, SERIAL_COUNTER.next_serial());
        }
        if self.host_focus && !self.policy.dragging() {
            self.pending_pointer = Some(self.start.elapsed().as_millis() as u32);
        }
        self.schedule_frame();
    }

    pub fn unmap(&mut self, index: usize) {
        if let Some(id) = self.managed[index].id.take() {
            if let Err(error) = self.policy.remove(id) {
                self.fail(error);
            }
            self.space.unmap_elem(&self.managed[index].window);
            self.managed[index].configured = None;
            eprintln!(
                "UNMAP id={} remaining={}",
                id.0,
                self.policy.wm.windows().len()
            );
            self.sync_policy();
            self.clear_pointer_focus();
        }
    }

    pub fn clear_pointer_focus(&mut self) {
        let pointer = self.seat.get_pointer().expect("pointer initialized");
        pointer.motion(
            self,
            None,
            &smithay::input::pointer::MotionEvent {
                location: self.pointer_pos,
                serial: SERIAL_COUNTER.next_serial(),
                time: self.start.elapsed().as_millis() as u32,
            },
        );
        pointer.frame(self);
    }

    pub fn reap(&mut self) {
        for index in (0..self.managed.len()).rev() {
            if !self.managed[index].window.alive() {
                self.unmap(index);
                self.managed.remove(index);
            }
        }
        self.space.refresh();
    }
}

#[derive(Default)]
pub struct ClientState {
    pub compositor_state: CompositorClientState,
}

impl ClientData for ClientState {
    fn initialized(&self, _: ClientId) {
        eprintln!("CLIENT connected");
    }
    fn disconnected(&self, _: ClientId, _: DisconnectReason) {
        eprintln!("CLIENT disconnected");
    }
}
