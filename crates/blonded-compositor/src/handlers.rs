// Protocol glue adapted from Smithay smallvil v0.7.0; see THIRD-PARTY.md.
use smithay::{
    backend::renderer::utils::{on_commit_buffer_handler, with_renderer_surface_state},
    delegate_compositor, delegate_data_device, delegate_output, delegate_seat, delegate_shm,
    delegate_xdg_shell,
    desktop::Window,
    input::{Seat, SeatHandler, SeatState},
    reexports::wayland_server::{
        protocol::{wl_buffer, wl_surface::WlSurface},
        Client, Resource,
    },
    wayland::{
        buffer::BufferHandler,
        compositor::{
            get_parent, is_sync_subsurface, CompositorClientState, CompositorHandler,
            CompositorState,
        },
        output::OutputHandler,
        selection::{
            data_device::{
                set_data_device_focus, ClientDndGrabHandler, DataDeviceHandler, DataDeviceState,
                ServerDndGrabHandler,
            },
            SelectionHandler,
        },
        shell::xdg::{
            PopupSurface, PositionerState, ToplevelSurface, XdgShellHandler, XdgShellState,
        },
        shm::{ShmHandler, ShmState},
    },
};

use crate::state::{ClientState, Managed, State};

impl CompositorHandler for State {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }
    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        &client
            .get_data::<ClientState>()
            .expect("registered client data")
            .compositor_state
    }
    fn commit(&mut self, surface: &WlSurface) {
        on_commit_buffer_handler::<Self>(surface);
        if is_sync_subsurface(surface) {
            return;
        }
        let mut root = surface.clone();
        while let Some(parent) = get_parent(&root) {
            root = parent;
        }
        let Some(index) = self.managed.iter().position(|entry| {
            entry
                .window
                .toplevel()
                .is_some_and(|top| top.wl_surface() == &root)
        }) else {
            return;
        };
        self.managed[index].window.on_commit();
        if surface == &root {
            let top = self.managed[index]
                .window
                .toplevel()
                .expect("Wayland window");
            if !top.is_initial_configure_sent() {
                top.with_pending_state(|state| state.size = Some((420, 280).into()));
                top.send_configure();
            }
            let mapped = with_renderer_surface_state(surface, |state| state.buffer().is_some())
                .unwrap_or(false);
            match (mapped, self.managed[index].id) {
                (true, None) => match self.policy.map() {
                    Ok(id) => {
                        self.managed[index].id = Some(id);
                        eprintln!(
                            "MAP id={} geometry={:?}",
                            id.0,
                            self.policy
                                .wm
                                .window(id)
                                .expect("just registered")
                                .geometry()
                        );
                        self.sync_policy();
                    }
                    Err(error) => {
                        eprintln!("MAP rejected: {error}");
                        top.send_close();
                    }
                },
                (false, Some(_)) => self.unmap(index),
                _ => {}
            }
        }
        self.schedule_frame();
    }
}
impl BufferHandler for State {
    fn buffer_destroyed(&mut self, _: &wl_buffer::WlBuffer) {}
}
impl ShmHandler for State {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}
delegate_compositor!(State);
delegate_shm!(State);

impl XdgShellHandler for State {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }
    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        if self.managed.len() >= 16 {
            surface.send_close();
            return;
        }
        self.managed.push(Managed {
            window: Window::new_wayland_window(surface),
            id: None,
            configured: None,
        });
    }
    fn toplevel_destroyed(&mut self, surface: ToplevelSurface) {
        if let Some(index) = self
            .managed
            .iter()
            .position(|entry| entry.window.toplevel() == Some(&surface))
        {
            self.unmap(index);
            self.managed.remove(index);
        }
    }
    fn new_popup(&mut self, surface: PopupSurface, _: PositionerState) {
        surface.send_popup_done();
    }
    fn reposition_request(&mut self, surface: PopupSurface, _: PositionerState, _: u32) {
        surface.send_popup_done();
    }
    fn grab(
        &mut self,
        surface: PopupSurface,
        _: smithay::reexports::wayland_server::protocol::wl_seat::WlSeat,
        _: smithay::utils::Serial,
    ) {
        surface.send_popup_done();
    }
}
delegate_xdg_shell!(State);

impl SeatHandler for State {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;
    type TouchFocus = WlSurface;
    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }
    fn cursor_image(&mut self, _: &Seat<Self>, _: smithay::input::pointer::CursorImageStatus) {}
    fn focus_changed(&mut self, seat: &Seat<Self>, focused: Option<&WlSurface>) {
        let client = focused.and_then(|surface| self.display_handle.get_client(surface.id()).ok());
        set_data_device_focus(&self.display_handle, seat, client);
    }
}
delegate_seat!(State);
impl SelectionHandler for State {
    type SelectionUserData = ();
}
impl DataDeviceHandler for State {
    fn data_device_state(&self) -> &DataDeviceState {
        &self.data_device_state
    }
}
impl ClientDndGrabHandler for State {}
impl ServerDndGrabHandler for State {}
delegate_data_device!(State);
impl OutputHandler for State {}
delegate_output!(State);
