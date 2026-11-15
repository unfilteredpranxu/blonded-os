// Input protocol dispatch adapted from Smithay smallvil; Blonded owns policy/actions.
use crate::{policy::DragKind, state::State};
use blonded_wm::{Command, Size};
use smithay::{
    backend::input::{
        AbsolutePositionEvent, Axis, AxisSource, ButtonState, Event, InputBackend, InputEvent,
        KeyState, KeyboardKeyEvent, PointerAxisEvent, PointerButtonEvent,
    },
    input::{
        keyboard::{keysyms, FilterResult},
        pointer::{AxisFrame, ButtonEvent, MotionEvent},
    },
    utils::SERIAL_COUNTER,
};

#[derive(Clone, Copy)]
enum Action {
    Cycle,
    Close,
    Quit,
    Geometry(i32, i32, bool),
    Suppress,
}

impl State {
    pub fn lose_input_focus(&mut self) {
        self.pending_pointer = None;
        if let Err(error) = self.policy.end_drag() {
            self.fail(error);
        }
        let keyboard = self.seat.get_keyboard().expect("keyboard initialized");
        let time = self.start.elapsed().as_millis() as u32;
        for code in keyboard.pressed_keys() {
            let suppress = self.consumed_keys.contains(&code);
            keyboard.input::<(), _>(
                self,
                code,
                KeyState::Released,
                SERIAL_COUNTER.next_serial(),
                time,
                |_, _, _| {
                    if suppress {
                        FilterResult::Intercept(())
                    } else {
                        FilterResult::Forward
                    }
                },
            );
        }
        let pointer = self.seat.get_pointer().expect("pointer initialized");
        for button in std::mem::take(&mut self.forwarded_buttons) {
            pointer.button(
                self,
                &ButtonEvent {
                    button,
                    state: ButtonState::Released,
                    serial: SERIAL_COUNTER.next_serial(),
                    time,
                },
            );
        }
        pointer.unset_grab(self, SERIAL_COUNTER.next_serial(), time);
        pointer.frame(self);
        self.consumed_buttons.clear();
        self.consumed_keys.clear();
        self.drag_button = None;
        self.clear_pointer_focus();
    }

    pub fn flush_pointer(&mut self) {
        let Some(time) = self.pending_pointer.take() else {
            return;
        };
        if self.policy.dragging() {
            self.policy.motion((self.pointer_pos.x, self.pointer_pos.y));
            return;
        }
        let pointer = self.seat.get_pointer().expect("pointer initialized");
        pointer.motion(
            self,
            self.surface_under(self.pointer_pos),
            &MotionEvent {
                location: self.pointer_pos,
                serial: SERIAL_COUNTER.next_serial(),
                time,
            },
        );
        pointer.frame(self);
    }

    fn action(&mut self, action: Action) -> Result<(), blonded_wm::Error> {
        match action {
            Action::Cycle => self.policy.cycle()?,
            Action::Quit => self.running = false,
            Action::Close => {
                if let Some(entry) = self
                    .managed
                    .iter()
                    .find(|entry| entry.id.is_some() && entry.id == self.policy.wm.focused())
                {
                    entry
                        .window
                        .toplevel()
                        .expect("Wayland window")
                        .send_close();
                }
            }
            Action::Geometry(dx, dy, resize) => {
                if let Some(id) = self.policy.wm.focused() {
                    let geometry = self.policy.wm.window(id)?.geometry();
                    let command = if resize {
                        Command::Resize {
                            id,
                            size: Size::new(
                                (geometry.width() as i32 + dx).max(1) as u32,
                                (geometry.height() as i32 + dy).max(1) as u32,
                            )?,
                        }
                    } else {
                        Command::Move {
                            id,
                            x: geometry.x() + dx,
                            y: geometry.y() + dy,
                        }
                    };
                    self.policy.wm.dispatch(command)?;
                    eprintln!(
                        "KEY geometry id={} {:?}",
                        id.0,
                        self.policy.wm.window(id)?.geometry()
                    );
                }
            }
            Action::Suppress => {}
        }
        self.sync_policy();
        Ok(())
    }

    pub fn process_input<I: InputBackend>(&mut self, event: InputEvent<I>) {
        if !self.host_focus {
            return;
        }
        match event {
            InputEvent::Keyboard { event, .. } => {
                let keyboard = self.seat.get_keyboard().expect("keyboard initialized");
                let code = event.key_code();
                let pressed = event.state() == KeyState::Pressed;
                let action = keyboard.input(
                    self,
                    code,
                    event.state(),
                    SERIAL_COUNTER.next_serial(),
                    event.time_msec(),
                    |state, modifiers, key| {
                        if state.consumed_keys.contains(&code) {
                            if !pressed {
                                state.consumed_keys.retain(|value| *value != code);
                            }
                            return FilterResult::Intercept(Action::Suppress);
                        }
                        if !pressed || !modifiers.alt {
                            return FilterResult::Forward;
                        }
                        let action = match key.modified_sym().raw() {
                            keysyms::KEY_Tab => Action::Cycle,
                            keysyms::KEY_F4 => Action::Close,
                            keysyms::KEY_Escape => Action::Quit,
                            keysyms::KEY_Left => Action::Geometry(-20, 0, modifiers.shift),
                            keysyms::KEY_Right => Action::Geometry(20, 0, modifiers.shift),
                            keysyms::KEY_Up => Action::Geometry(0, -20, modifiers.shift),
                            keysyms::KEY_Down => Action::Geometry(0, 20, modifiers.shift),
                            _ => return FilterResult::Forward,
                        };
                        state.consumed_keys.push(code);
                        FilterResult::Intercept(action)
                    },
                );
                if let Some(action) = action {
                    if let Err(error) = self.action(action) {
                        self.fail(error);
                    }
                }
            }
            InputEvent::PointerMotionAbsolute { event, .. } => {
                let config = self.policy.wm.config();
                let position = event.position_transformed(
                    (config.screen.width() as i32, config.screen.height() as i32).into(),
                );
                if position != self.pointer_pos {
                    self.pointer_pos = position;
                    self.pending_pointer = Some(event.time_msec());
                    self.schedule_frame();
                }
            }
            InputEvent::PointerButton { event, .. } => {
                self.flush_pointer();
                let pointer = self.seat.get_pointer().expect("pointer initialized");
                let button = event.button_code();
                if event.state() == ButtonState::Released && self.consumed_buttons.contains(&button)
                {
                    self.consumed_buttons.retain(|value| *value != button);
                    if self.drag_button != Some(button) {
                        return;
                    }
                    self.drag_button = None;
                    if let Err(error) = self.policy.end_drag() {
                        self.fail(error);
                    }
                    eprintln!(
                        "DRAG end focused={:?}",
                        self.policy
                            .wm
                            .focused()
                            .and_then(|id| self.policy.wm.window(id).ok())
                            .map(|window| window.geometry())
                    );
                    self.sync_policy();
                    self.pending_pointer = Some(event.time_msec());
                    self.flush_pointer();
                    return;
                }
                if event.state() == ButtonState::Pressed && !pointer.is_grabbed() {
                    if let Some(id) = self.id_under() {
                        if let Err(error) = self.policy.focus(id) {
                            self.fail(error);
                            return;
                        }
                        self.sync_policy();
                        let alt = self
                            .seat
                            .get_keyboard()
                            .expect("keyboard initialized")
                            .modifier_state()
                            .alt;
                        if alt
                            && (button == 0x110 || button == 0x111)
                            && self.consumed_buttons.is_empty()
                        {
                            let kind = if button == 0x110 {
                                DragKind::Move
                            } else {
                                DragKind::Resize
                            };
                            self.policy.start_drag(
                                id,
                                kind,
                                (self.pointer_pos.x, self.pointer_pos.y),
                            );
                            self.consumed_buttons.push(button);
                            self.drag_button = Some(button);
                            self.clear_pointer_focus();
                            eprintln!("DRAG start id={} kind={kind:?}", id.0);
                            return;
                        }
                    }
                }
                if self.policy.dragging() {
                    if event.state() == ButtonState::Pressed
                        && !self.consumed_buttons.contains(&button)
                    {
                        self.consumed_buttons.push(button);
                    }
                    return;
                }
                if event.state() == ButtonState::Pressed {
                    if !self.forwarded_buttons.contains(&button) {
                        self.forwarded_buttons.push(button);
                    }
                } else {
                    self.forwarded_buttons.retain(|value| *value != button);
                }
                pointer.button(
                    self,
                    &ButtonEvent {
                        button,
                        state: event.state(),
                        serial: SERIAL_COUNTER.next_serial(),
                        time: event.time_msec(),
                    },
                );
                pointer.frame(self);
            }
            InputEvent::PointerAxis { event, .. } => {
                if self.policy.dragging() {
                    return;
                }
                self.flush_pointer();
                let mut frame = AxisFrame::new(event.time_msec()).source(event.source());
                for axis in [Axis::Horizontal, Axis::Vertical] {
                    let amount = event
                        .amount(axis)
                        .unwrap_or_else(|| event.amount_v120(axis).unwrap_or(0.) * 15. / 120.);
                    if amount != 0. {
                        frame = frame.value(axis, amount);
                    }
                    if let Some(discrete) = event.amount_v120(axis) {
                        frame = frame.v120(axis, discrete as i32);
                    }
                    if event.source() == AxisSource::Finger && event.amount(axis) == Some(0.) {
                        frame = frame.stop(axis);
                    }
                }
                let pointer = self.seat.get_pointer().expect("pointer initialized");
                pointer.axis(self, frame);
                pointer.frame(self);
            }
            _ => {}
        }
    }
}
