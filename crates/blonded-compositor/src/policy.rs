//! Bounded protocol adapter policy. Geometry/focus/stacking stay in blonded-wm.
use blonded_wm::{Command, Config, Direction, Rect, Size, WindowId, WindowManager, WorkspaceId};

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
            next_id: 1,
            drag: None,
        })
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
