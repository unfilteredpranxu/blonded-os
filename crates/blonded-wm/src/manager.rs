use std::collections::{BTreeMap, BTreeSet};

use crate::geometry::partition;
use crate::{
    Command, Config, Direction, Error, Event, Layout, Mode, Rect, SnapRegion, Window, WindowId,
    Workspace, WorkspaceId,
};

/// Bounded, event-driven state. All fields are private; callers receive read-only
/// views. Dispatch validates and stages a bounded copy before committing, so
/// errors (including an impossible reflow) preserve the entire previous state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowManager {
    config: Config,
    windows: BTreeMap<WindowId, Window>,
    workspaces: Vec<Workspace>,
    active: WorkspaceId,
}

impl WindowManager {
    pub fn new(config: Config) -> Result<Self, Error> {
        config.validate()?;
        Ok(Self {
            config,
            windows: BTreeMap::new(),
            workspaces: (0..config.workspace_count)
                .map(|id| Workspace {
                    id: WorkspaceId(id),
                    layout: Layout::Floating,
                    focused: None,
                    stacking: Vec::new(),
                })
                .collect(),
            active: WorkspaceId(0),
        })
    }

    pub const fn config(&self) -> Config {
        self.config
    }
    pub const fn active_workspace(&self) -> WorkspaceId {
        self.active
    }
    pub fn window(&self, id: WindowId) -> Result<&Window, Error> {
        self.windows.get(&id).ok_or(Error::UnknownWindow(id))
    }
    pub fn windows(&self) -> impl ExactSizeIterator<Item = &Window> {
        self.windows.values()
    }
    pub fn workspace(&self, id: WorkspaceId) -> Result<&Workspace, Error> {
        self.workspaces
            .get(usize::from(id.0))
            .ok_or(Error::UnknownWorkspace(id))
    }
    pub fn is_visible(&self, id: WindowId) -> bool {
        self.windows
            .get(&id)
            .is_some_and(|w| w.workspace == self.active && !w.minimized)
    }
    pub fn focused(&self) -> Option<WindowId> {
        self.workspace(self.active).ok().and_then(|ws| ws.focused)
    }

    pub fn dispatch(&mut self, command: Command) -> Result<Vec<Event>, Error> {
        self.check_invariants()?;
        let mut next = self.clone();
        next.execute(command)?;
        next.reflow()?;
        next.repair_focus();
        next.check_invariants()?;
        let events = self.diff(&next);
        *self = next;
        Ok(events)
    }

    fn workspace_mut(&mut self, id: WorkspaceId) -> Result<&mut Workspace, Error> {
        self.workspaces
            .get_mut(usize::from(id.0))
            .ok_or(Error::UnknownWorkspace(id))
    }

    fn window_mut(&mut self, id: WindowId) -> Result<&mut Window, Error> {
        self.windows.get_mut(&id).ok_or(Error::UnknownWindow(id))
    }

    fn ensure_unminimized(&self, id: WindowId) -> Result<(), Error> {
        if self.window(id)?.minimized {
            return Err(Error::NotVisible(id));
        }
        Ok(())
    }

    fn ensure_floating(&self, id: WindowId) -> Result<(), Error> {
        self.ensure_unminimized(id)?;
        let window = self.window(id)?;
        if window.mode != Mode::Normal
            || self.workspace(window.workspace)?.layout != Layout::Floating
        {
            return Err(Error::NotFloating(id));
        }
        Ok(())
    }

    fn raise(&mut self, id: WindowId) -> Result<(), Error> {
        self.ensure_unminimized(id)?;
        let ws = self.window(id)?.workspace;
        let stack = &mut self.workspace_mut(ws)?.stacking;
        stack.retain(|entry| *entry != id);
        stack.push(id);
        Ok(())
    }

    fn focus(&mut self, id: WindowId) -> Result<(), Error> {
        if !self.is_visible(id) {
            self.window(id)?;
            return Err(Error::NotVisible(id));
        }
        self.raise(id)?;
        self.workspace_mut(self.active)?.focused = Some(id);
        Ok(())
    }

    fn execute(&mut self, command: Command) -> Result<(), Error> {
        match command {
            Command::Register {
                id,
                workspace,
                geometry,
            } => {
                self.workspace(workspace)?;
                if self.windows.contains_key(&id) {
                    return Err(Error::DuplicateWindow(id));
                }
                if self.windows.len() >= self.config.max_windows {
                    return Err(Error::Capacity);
                }
                let geometry = geometry.fit(self.config.usable()?, self.config.minimum)?;
                self.windows.insert(
                    id,
                    Window {
                        id,
                        workspace,
                        geometry,
                        floating_geometry: geometry,
                        mode: Mode::Normal,
                        minimized: false,
                    },
                );
                let ws = self.workspace_mut(workspace)?;
                ws.stacking.push(id);
                ws.focused = Some(id);
            }
            Command::Remove(id) => {
                let workspace = self.window(id)?.workspace;
                self.windows.remove(&id);
                self.workspace_mut(workspace)?
                    .stacking
                    .retain(|entry| *entry != id);
            }
            Command::Move { id, x, y } => {
                self.ensure_floating(id)?;
                let window = self.window(id)?;
                // Clamp coordinates before constructing to handle even extreme
                // pointer positions without overflowing a rectangle endpoint.
                let bounds = self.config.usable()?;
                let size = window.floating_geometry.size();
                let x = i64::from(x).clamp(
                    i64::from(bounds.x()),
                    bounds.right() - i64::from(size.width()),
                );
                let y = i64::from(y).clamp(
                    i64::from(bounds.y()),
                    bounds.bottom() - i64::from(size.height()),
                );
                self.window_mut(id)?.floating_geometry = Rect::from_size(
                    crate::geometry::coordinate(x)?,
                    crate::geometry::coordinate(y)?,
                    size,
                )?;
            }
            Command::Resize { id, size } => {
                self.ensure_floating(id)?;
                let old = self.window(id)?.floating_geometry;
                let bounds = self.config.usable()?;
                // Resize anchors the top-left; shrink to available space instead
                // of moving that corner. Existing state guarantees minimum room.
                let width = size.width().clamp(
                    self.config.minimum.width(),
                    (bounds.right() - i64::from(old.x())) as u32,
                );
                let height = size.height().clamp(
                    self.config.minimum.height(),
                    (bounds.bottom() - i64::from(old.y())) as u32,
                );
                self.window_mut(id)?.floating_geometry =
                    Rect::new(old.x(), old.y(), width, height)?;
            }
            Command::Focus(id) => self.focus(id)?,
            Command::Raise(id) => self.raise(id)?,
            Command::CycleFocus(direction) => {
                // Stable ID order, independent of raise/focus history.
                let ids: Vec<_> = self
                    .windows
                    .values()
                    .filter(|w| self.is_visible(w.id))
                    .map(|w| w.id)
                    .collect();
                if !ids.is_empty() {
                    let current = self
                        .focused()
                        .and_then(|id| ids.iter().position(|candidate| *candidate == id));
                    let index = match (current, direction) {
                        (Some(index), Direction::Next) => (index + 1) % ids.len(),
                        (Some(index), Direction::Previous) => (index + ids.len() - 1) % ids.len(),
                        (None, Direction::Next) => 0,
                        (None, Direction::Previous) => ids.len() - 1,
                    };
                    self.focus(ids[index])?;
                }
            }
            Command::Maximize(id) => {
                self.ensure_unminimized(id)?;
                self.window_mut(id)?.mode = Mode::Maximized;
            }
            Command::Minimize(id) => self.window_mut(id)?.minimized = true,
            Command::Restore(id) => {
                let window = self.window_mut(id)?;
                if window.minimized {
                    window.minimized = false;
                } else {
                    window.mode = Mode::Normal;
                }
            }
            Command::Snap { id, region } => {
                self.ensure_unminimized(id)?;
                self.window_mut(id)?.mode = Mode::Snapped(region);
            }
            Command::SetLayout { workspace, layout } => {
                self.workspace_mut(workspace)?.layout = layout
            }
            Command::SwitchWorkspace(workspace) => {
                self.workspace(workspace)?;
                self.active = workspace;
            }
            Command::MoveToWorkspace { id, workspace } => {
                self.workspace(workspace)?;
                let old = self.window(id)?.workspace;
                if old != workspace {
                    self.workspace_mut(old)?
                        .stacking
                        .retain(|entry| *entry != id);
                    self.window_mut(id)?.workspace = workspace;
                    self.workspace_mut(workspace)?.stacking.push(id);
                    if !self.window(id)?.minimized {
                        self.workspace_mut(workspace)?.focused = Some(id);
                    }
                }
            }
            Command::Configure(config) => {
                config.validate()?;
                if config.workspace_count != self.config.workspace_count
                    || config.max_windows < self.windows.len()
                {
                    return Err(Error::InvalidConfig);
                }
                self.config = config;
            }
        }
        Ok(())
    }

    fn snap(&self, region: SnapRegion) -> Result<Rect, Error> {
        let usable = self.config.usable()?;
        let horizontal = match region {
            SnapRegion::Left | SnapRegion::TopLeft | SnapRegion::BottomLeft => Some(0),
            SnapRegion::Right | SnapRegion::TopRight | SnapRegion::BottomRight => Some(1),
            _ => None,
        };
        let vertical = match region {
            SnapRegion::Top | SnapRegion::TopLeft | SnapRegion::TopRight => Some(0),
            SnapRegion::Bottom | SnapRegion::BottomLeft | SnapRegion::BottomRight => Some(1),
            _ => None,
        };
        let mut geometry = usable;
        if let Some(index) = horizontal {
            geometry = *partition(geometry, 2, self.config.gap, true, self.config.minimum)?
                .get(index)
                .ok_or(Error::CorruptState)?;
        }
        if let Some(index) = vertical {
            geometry = *partition(geometry, 2, self.config.gap, false, self.config.minimum)?
                .get(index)
                .ok_or(Error::CorruptState)?;
        }
        Ok(geometry)
    }

    fn reflow(&mut self) -> Result<(), Error> {
        let usable = self.config.usable()?;
        for window in self.windows.values_mut() {
            window.floating_geometry = window.floating_geometry.fit(usable, self.config.minimum)?;
        }
        // Compute snap geometry before borrowing individual windows mutably.
        let placements: Result<Vec<_>, Error> = self
            .windows
            .values()
            .map(|window| {
                let geometry = match window.mode {
                    Mode::Normal => window.floating_geometry,
                    Mode::Maximized => usable,
                    Mode::Snapped(region) => self.snap(region)?,
                };
                Ok((window.id, geometry))
            })
            .collect();
        for (id, geometry) in placements? {
            self.window_mut(id)?.geometry = geometry;
        }
        for ws in &self.workspaces {
            if ws.layout == Layout::Floating {
                continue;
            }
            let ids: Vec<_> = self
                .windows
                .values()
                .filter(|w| w.workspace == ws.id && !w.minimized && w.mode == Mode::Normal)
                .map(|w| w.id)
                .collect();
            let tiles = partition(
                usable,
                ids.len(),
                self.config.gap,
                ws.layout == Layout::Columns,
                self.config.minimum,
            )?;
            for (id, geometry) in ids.into_iter().zip(tiles) {
                self.windows
                    .get_mut(&id)
                    .ok_or(Error::CorruptState)?
                    .geometry = geometry;
            }
        }
        Ok(())
    }

    fn repair_focus(&mut self) {
        let windows = &self.windows;
        for ws in &mut self.workspaces {
            let eligible = |id: &WindowId| {
                windows
                    .get(id)
                    .is_some_and(|w| w.workspace == ws.id && !w.minimized)
            };
            if !ws.focused.as_ref().is_some_and(eligible) {
                ws.focused = ws.stacking.iter().rev().find(|id| eligible(id)).copied();
            }
        }
    }

    /// Runtime consistency check for backend diagnostics, also run on both sides
    /// of every dispatch. State cannot be mutated through the public API.
    pub fn check_invariants(&self) -> Result<(), Error> {
        self.config.validate().map_err(|_| Error::CorruptState)?;
        if self.workspaces.len() != usize::from(self.config.workspace_count)
            || self.windows.len() > self.config.max_windows
            || self.workspace(self.active).is_err()
        {
            return Err(Error::CorruptState);
        }
        let usable = self.config.usable()?;
        let mut seen = BTreeSet::new();
        for (index, ws) in self.workspaces.iter().enumerate() {
            if usize::from(ws.id.0) != index || ws.stacking.len() > self.config.max_windows {
                return Err(Error::CorruptState);
            }
            let mut visible = false;
            for id in &ws.stacking {
                let window = self.windows.get(id).ok_or(Error::CorruptState)?;
                if window.id != *id || window.workspace != ws.id || !seen.insert(*id) {
                    return Err(Error::CorruptState);
                }
                visible |= !window.minimized;
                for geometry in [window.geometry, window.floating_geometry] {
                    if !usable.contains(geometry)
                        || geometry.width() < self.config.minimum.width()
                        || geometry.height() < self.config.minimum.height()
                    {
                        return Err(Error::CorruptState);
                    }
                }
            }
            match ws.focused {
                Some(id)
                    if self
                        .windows
                        .get(&id)
                        .is_some_and(|w| w.workspace == ws.id && !w.minimized) => {}
                None if !visible => {}
                _ => return Err(Error::CorruptState),
            }
        }
        if seen.len() != self.windows.len() {
            return Err(Error::CorruptState);
        }
        Ok(())
    }

    fn diff(&self, next: &Self) -> Vec<Event> {
        let mut events = Vec::new();
        for id in self.windows.keys() {
            if !next.windows.contains_key(id) {
                events.push(Event::Removed(*id));
            }
        }
        for (id, window) in &next.windows {
            match self.windows.get(id) {
                None => events.push(Event::Registered(*window)),
                Some(old) if old != window => events.push(Event::WindowChanged(*window)),
                _ => {}
            }
        }
        for (old, new) in self.workspaces.iter().zip(&next.workspaces) {
            if old != new {
                events.push(Event::WorkspaceChanged(new.clone()));
            }
        }
        if self.active != next.active {
            events.push(Event::ActiveWorkspaceChanged(next.active));
        }
        if self.config != next.config {
            events.push(Event::ConfigChanged(next.config));
        }
        events
    }
}

#[cfg(test)]
mod corruption_tests {
    use super::*;
    use crate::Size;

    #[test]
    fn corrupted_state_returns_error_without_dispatching_or_panicking() {
        let config = Config {
            screen: Rect::new(0, 0, 800, 600).unwrap(),
            gap: 4,
            minimum: Size::new(100, 80).unwrap(),
            workspace_count: 2,
            max_windows: 8,
        };
        let mut wm = WindowManager::new(config).unwrap();
        wm.workspaces[0].stacking.push(WindowId(99));
        let before = wm.clone();
        assert_eq!(
            wm.dispatch(Command::CycleFocus(Direction::Next)),
            Err(Error::CorruptState)
        );
        assert_eq!(wm, before);
        wm.workspaces.clear();
        assert_eq!(
            wm.dispatch(Command::SwitchWorkspace(WorkspaceId(0))),
            Err(Error::CorruptState)
        );
    }
}
