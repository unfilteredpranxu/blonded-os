//! Bounded protocol adapter policy. Geometry/focus/stacking stay in blonded-wm.
use blonded_wm::{Command, Config, Direction, Rect, Size, WindowId, WindowManager, WorkspaceId};

pub const MIN_OUTPUT: (u32, u32) = (120, 96); // 96x72 client minimum + two 12px gaps

#[derive(Debug, PartialEq, Eq)]
pub enum OutputChange {
    Unchanged,
    Applied,
    Suspended(blonded_wm::Error),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragKind {
    Move,
    Resize,
}

struct Drag {
    id: WindowId,
    kind: DragKind,
    origin: (f64, f64),
    geometry: Rect,
    latest: Option<(f64, f64)>,
}

pub struct Policy {
    pub wm: WindowManager,
    pub output_suspended: bool,
    next_id: u64,
    drag: Option<Drag>,
}

impl Policy {
    pub fn new(width: u32, height: u32) -> Result<Self, blonded_wm::Error> {
        Ok(Self {
            wm: WindowManager::new(Config {
                screen: Rect::new(0, 0, width, height)?,
                gap: 12,
                minimum: Size::new(96, 72)?,
                workspace_count: 1,
                max_windows: 16,
            })?,
            output_suspended: false,
            next_id: 1,
            drag: None,
        })
    }

    /// A host may initially ignore minimum-size hints. Keep a valid dormant
    /// snapshot until an actual output size can satisfy the WM constraints.
    pub fn for_output(width: i32, height: i32) -> Result<Self, blonded_wm::Error> {
        let mut policy = Self::new(960, 640)?;
        policy.adapt_output(width, height);
        Ok(policy)
    }

    /// Overflow-safe keyboard geometry requests; WM still clamps the result.
    pub fn adjust_geometry(
        &mut self,
        id: WindowId,
        dx: i32,
        dy: i32,
        resize: bool,
    ) -> Result<(), blonded_wm::Error> {
        let geometry = self.wm.window(id)?.geometry();
        let command = if resize {
            Command::Resize {
                id,
                size: Size::new(
                    (i64::from(geometry.width()) + i64::from(dx)).clamp(1, i64::from(i32::MAX))
                        as u32,
                    (i64::from(geometry.height()) + i64::from(dy)).clamp(1, i64::from(i32::MAX))
                        as u32,
                )?,
            }
        } else {
            Command::Move {
                id,
                x: geometry.x().saturating_add(dx),
                y: geometry.y().saturating_add(dy),
            }
        };
        self.wm.dispatch(command)?;
        Ok(())
    }

    /// Invalid/tiny/zero output sizes suspend presentation and input. The last
    /// successful WM/output snapshot remains intact; a valid resize resumes it.
    pub fn adapt_output(&mut self, width: i32, height: i32) -> OutputChange {
        if !self.output_suspended
            && width == self.wm.config().screen.width() as i32
            && height == self.wm.config().screen.height() as i32
        {
            return OutputChange::Unchanged;
        }
        let result = u32::try_from(width)
            .ok()
            .zip(u32::try_from(height).ok())
            .ok_or(blonded_wm::Error::InvalidGeometry)
            .and_then(|(width, height)| self.configure(width, height));
        match result {
            Ok(()) => {
                self.output_suspended = false;
                OutputChange::Applied
            }
            Err(error) => {
                self.output_suspended = true;
                self.drag = None;
                OutputChange::Suspended(error)
            }
        }
    }

    pub fn map(&mut self) -> Result<WindowId, Box<dyn std::error::Error>> {
        let id = WindowId(self.next_id);
        self.next_id = self.next_id.checked_add(1).ok_or("window ID exhausted")?;
        let offset = (self.wm.windows().len() % 8) as i32 * 36;
        self.wm.dispatch(Command::Register {
            id,
            workspace: WorkspaceId(0),
            geometry: Rect::new(32 + offset, 32 + offset, 420, 280)?,
        })?;
        Ok(id)
    }

    pub fn remove(&mut self, id: WindowId) -> Result<(), blonded_wm::Error> {
        self.wm.dispatch(Command::Remove(id))?;
        if self.drag.as_ref().is_some_and(|drag| drag.id == id) {
            self.drag = None;
        }
        Ok(())
    }

    pub fn focus(&mut self, id: WindowId) -> Result<(), blonded_wm::Error> {
        if self.wm.focused() != Some(id) {
            self.wm.dispatch(Command::Focus(id))?;
        }
        Ok(())
    }

    pub fn cycle(&mut self) -> Result<(), blonded_wm::Error> {
        self.wm.dispatch(Command::CycleFocus(Direction::Next))?;
        Ok(())
    }

    pub fn configure(&mut self, width: u32, height: u32) -> Result<(), blonded_wm::Error> {
        let mut config = self.wm.config();
        config.screen = Rect::new(0, 0, width, height)?;
        self.wm.dispatch(Command::Configure(config))?;
        self.drag = None;
        Ok(())
    }

    pub fn start_drag(&mut self, id: WindowId, kind: DragKind, pos: (f64, f64)) {
        if let Ok(window) = self.wm.window(id) {
            self.drag = Some(Drag {
                id,
                kind,
                origin: pos,
                geometry: window.geometry(),
                latest: None,
            });
        }
    }

    pub fn dragging(&self) -> bool {
        self.drag.is_some()
    }

    pub fn motion(&mut self, pos: (f64, f64)) {
        if let Some(drag) = &mut self.drag {
            drag.latest = Some(pos);
        }
    }

    /// At most one WM dispatch per scheduled frame, regardless of motion rate.
    pub fn flush_motion(&mut self) -> Result<(), blonded_wm::Error> {
        let Some(drag) = &mut self.drag else {
            return Ok(());
        };
        let Some(pos) = drag.latest.take() else {
            return Ok(());
        };
        let dx = (pos.0 - drag.origin.0).round() as i32;
        let dy = (pos.1 - drag.origin.1).round() as i32;
        let command = match drag.kind {
            DragKind::Move => Command::Move {
                id: drag.id,
                x: drag.geometry.x().saturating_add(dx),
                y: drag.geometry.y().saturating_add(dy),
            },
            DragKind::Resize => Command::Resize {
                id: drag.id,
                size: Size::new(
                    (i64::from(drag.geometry.width()) + i64::from(dx)).clamp(1, i64::from(i32::MAX))
                        as u32,
                    (i64::from(drag.geometry.height()) + i64::from(dy))
                        .clamp(1, i64::from(i32::MAX)) as u32,
                )?,
            },
        };
        self.wm.dispatch(command)?;
        Ok(())
    }

    pub fn end_drag(&mut self) -> Result<(), blonded_wm::Error> {
        self.flush_motion()?;
        self.drag = None;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extreme_policy(screen: Rect) -> (Policy, WindowId) {
        let mut policy = Policy::new(960, 640).unwrap();
        let mut config = policy.wm.config();
        config.screen = screen;
        config.gap = 0;
        config.minimum = Size::new(1, 1).unwrap();
        policy.wm.dispatch(Command::Configure(config)).unwrap();
        let id = policy.map().unwrap();
        (policy, id)
    }

    #[test]
    fn keyboard_movement_near_positive_coordinate_limit_saturates() {
        let (mut policy, id) =
            extreme_policy(Rect::new(i32::MAX - 1000, i32::MAX - 1000, 1000, 1000).unwrap());
        policy
            .wm
            .dispatch(Command::Resize {
                id,
                size: Size::new(1, 1).unwrap(),
            })
            .unwrap();
        policy
            .wm
            .dispatch(Command::Move {
                id,
                x: i32::MAX - 1,
                y: i32::MAX - 1,
            })
            .unwrap();
        policy.adjust_geometry(id, 20, 20, false).unwrap();
        assert_eq!(
            policy.wm.window(id).unwrap().geometry(),
            Rect::new(i32::MAX - 1, i32::MAX - 1, 1, 1).unwrap()
        );
        policy.wm.check_invariants().unwrap();
    }

    #[test]
    fn keyboard_movement_near_negative_coordinate_limit_saturates() {
        let (mut policy, id) = extreme_policy(Rect::new(i32::MIN, i32::MIN, 1000, 1000).unwrap());
        policy
            .wm
            .dispatch(Command::Move {
                id,
                x: i32::MIN,
                y: i32::MIN,
            })
            .unwrap();
        policy.adjust_geometry(id, -20, i32::MIN, false).unwrap();
        let geometry = policy.wm.window(id).unwrap().geometry();
        assert_eq!((geometry.x(), geometry.y()), (i32::MIN, i32::MIN));
        policy.wm.check_invariants().unwrap();
    }

    #[test]
    fn keyboard_resize_near_dimension_limits_clamps_before_conversion() {
        let (mut policy, id) =
            extreme_policy(Rect::new(0, 0, i32::MAX as u32, i32::MAX as u32).unwrap());
        policy
            .wm
            .dispatch(Command::Move { id, x: 0, y: 0 })
            .unwrap();
        policy
            .wm
            .dispatch(Command::Resize {
                id,
                size: Size::new(i32::MAX as u32, i32::MAX as u32).unwrap(),
            })
            .unwrap();
        policy.adjust_geometry(id, 20, i32::MAX, true).unwrap();
        assert_eq!(
            policy.wm.window(id).unwrap().geometry().size(),
            Size::new(i32::MAX as u32, i32::MAX as u32).unwrap()
        );
        policy
            .adjust_geometry(id, i32::MIN, i32::MIN, true)
            .unwrap();
        policy.adjust_geometry(id, -20, -20, true).unwrap();
        assert_eq!(
            policy.wm.window(id).unwrap().geometry().size(),
            Size::new(1, 1).unwrap()
        );
        policy.wm.check_invariants().unwrap();
    }

    #[test]
    fn output_accepts_minimum_and_unusual_aspect_ratios() {
        let mut policy = Policy::new(960, 640).unwrap();
        let id = policy.map().unwrap();
        for (width, height) in [
            (120, 96),
            (121, 97),
            (120, 2000),
            (2000, 96),
            (240, 160),
            (319, 239),
        ] {
            assert_eq!(policy.adapt_output(width, height), OutputChange::Applied);
            let screen = policy.wm.config().screen;
            assert_eq!(
                (screen.width(), screen.height()),
                (width as u32, height as u32)
            );
            assert!(screen.contains(policy.wm.window(id).unwrap().geometry()));
            assert!(!policy.output_suspended);
            assert_eq!(policy.adapt_output(width, height), OutputChange::Unchanged);
            policy.wm.check_invariants().unwrap();
        }
    }

    #[test]
    fn impossible_outputs_preserve_snapshot_and_recover_including_same_size() {
        let mut policy = Policy::new(960, 640).unwrap();
        let id = policy.map().unwrap();
        for (width, height) in [
            (0, 0),
            (119, 96),
            (120, 95),
            (119, 2000),
            (2000, 95),
            (-1, 96),
            (120, -1),
        ] {
            policy.start_drag(id, DragKind::Move, (0., 0.));
            policy.motion((500., 500.));
            let before = policy.wm.clone();
            assert!(matches!(
                policy.adapt_output(width, height),
                OutputChange::Suspended(_)
            ));
            assert_eq!(policy.wm, before);
            assert!(policy.output_suspended);
            assert!(!policy.dragging());
            assert_eq!(policy.adapt_output(960, 640), OutputChange::Applied);
            assert!(!policy.output_suspended);
            policy.wm.check_invariants().unwrap();
        }
    }

    #[test]
    fn invalid_initial_size_starts_dormant_and_valid_resize_resumes() {
        let mut policy = Policy::for_output(0, 0).unwrap();
        assert!(policy.output_suspended);
        policy.wm.check_invariants().unwrap();
        assert_eq!(
            policy.adapt_output(MIN_OUTPUT.0 as i32, MIN_OUTPUT.1 as i32),
            OutputChange::Applied
        );
        assert!(!policy.output_suspended);
        policy.map().unwrap();
        policy.wm.check_invariants().unwrap();
    }

    #[test]
    fn lifecycle_focus_and_ids_survive_unmap_remap() {
        let mut policy = Policy::new(960, 640).unwrap();
        let first = policy.map().unwrap();
        let second = policy.map().unwrap();
        assert_ne!(
            policy.wm.window(first).unwrap().geometry(),
            policy.wm.window(second).unwrap().geometry()
        );
        policy.cycle().unwrap();
        assert_eq!(policy.wm.focused(), Some(first));
        policy.remove(first).unwrap();
        assert_eq!(policy.wm.focused(), Some(second));
        policy.remove(second).unwrap();
        assert_eq!(policy.wm.focused(), None);
        assert!(policy.map().unwrap() > second);
        policy.wm.check_invariants().unwrap();
    }

    #[test]
    fn motion_is_coalesced_and_disconnect_cancels_drag() {
        let mut policy = Policy::new(960, 640).unwrap();
        let id = policy.map().unwrap();
        policy.start_drag(id, DragKind::Move, (0., 0.));
        for n in 0..1000 {
            policy.motion((f64::from(n), 100.));
        }
        assert_eq!(policy.wm.window(id).unwrap().geometry().x(), 32);
        policy.flush_motion().unwrap();
        let geometry = policy.wm.window(id).unwrap().geometry();
        assert_eq!(geometry.x(), 528); // right edge minus gap and width
        assert_eq!(geometry.y(), 132);
        policy.motion((0., 0.));
        policy.remove(id).unwrap();
        assert!(!policy.dragging());
        policy.flush_motion().unwrap();
        policy.wm.check_invariants().unwrap();
    }

    #[test]
    fn resize_release_and_failed_output_change_are_safe() {
        let mut policy = Policy::new(960, 640).unwrap();
        let id = policy.map().unwrap();
        policy.start_drag(id, DragKind::Resize, (0., 0.));
        policy.motion((-1000., 10000.));
        policy.end_drag().unwrap();
        assert_eq!(
            policy.wm.window(id).unwrap().geometry().size(),
            Size::new(96, 596).unwrap()
        );
        let before = policy.wm.clone();
        assert!(policy.configure(1, 1).is_err());
        assert_eq!(policy.wm, before);
        policy.configure(640, 480).unwrap();
        policy.wm.check_invariants().unwrap();
    }
}
