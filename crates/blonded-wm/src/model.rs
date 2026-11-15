use std::fmt;

use crate::{Rect, Size};

pub const MAX_WINDOWS: usize = 256;
pub const MAX_WORKSPACES: u16 = 32;

/// Adapter-assigned opaque identity. Reusing a destroyed ID is the adapter's responsibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WindowId(pub u64);

/// Fixed workspace index, starting at zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorkspaceId(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    pub screen: Rect,
    /// Outer inset and spacing between tiles or snap regions, in logical pixels.
    pub gap: u32,
    pub minimum: Size,
    pub workspace_count: u16,
    pub max_windows: usize,
}

impl Config {
    pub fn validate(self) -> Result<(), Error> {
        if self.workspace_count == 0
            || self.workspace_count > MAX_WORKSPACES
            || self.max_windows == 0
            || self.max_windows > MAX_WINDOWS
        {
            return Err(Error::InvalidConfig);
        }
        let usable = self.usable()?;
        if usable.width() < self.minimum.width() || usable.height() < self.minimum.height() {
            return Err(Error::InvalidConfig);
        }
        Ok(())
    }

    pub fn usable(self) -> Result<Rect, Error> {
        self.screen
            .inset(self.gap)
            .map_err(|_| Error::InvalidConfig)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Floating,
    Columns,
    Rows,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapRegion {
    Left,
    Right,
    Top,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Maximized,
    Snapped(SnapRegion),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Window {
    pub(crate) id: WindowId,
    pub(crate) workspace: WorkspaceId,
    pub(crate) geometry: Rect,
    pub(crate) floating_geometry: Rect,
    pub(crate) mode: Mode,
    pub(crate) minimized: bool,
}

impl Window {
    pub const fn id(&self) -> WindowId {
        self.id
    }
    pub const fn workspace(&self) -> WorkspaceId {
        self.workspace
    }
    pub const fn geometry(&self) -> Rect {
        self.geometry
    }
    pub const fn floating_geometry(&self) -> Rect {
        self.floating_geometry
    }
    pub const fn mode(&self) -> Mode {
        self.mode
    }
    pub const fn is_minimized(&self) -> bool {
        self.minimized
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workspace {
    pub(crate) id: WorkspaceId,
    pub(crate) layout: Layout,
    pub(crate) focused: Option<WindowId>,
    pub(crate) stacking: Vec<WindowId>,
}

impl Workspace {
    pub const fn id(&self) -> WorkspaceId {
        self.id
    }
    pub const fn layout(&self) -> Layout {
        self.layout
    }
    pub const fn focused(&self) -> Option<WindowId> {
        self.focused
    }
    /// Bottom to top, including minimized windows. Filter with `is_visible` to render.
    pub fn stacking(&self) -> &[WindowId] {
        &self.stacking
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Next,
    Previous,
}

/// Commands are processed synchronously, in caller order. Invalid commands have
/// no effects. Move/resize require a normal window in a floating workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Register {
        id: WindowId,
        workspace: WorkspaceId,
        geometry: Rect,
    },
    Remove(WindowId),
    Move {
        id: WindowId,
        x: i32,
        y: i32,
    },
    Resize {
        id: WindowId,
        size: Size,
    },
    Focus(WindowId),
    Raise(WindowId),
    CycleFocus(Direction),
    Maximize(WindowId),
    Minimize(WindowId),
    /// Unminimize first (preserving mode); otherwise return to normal placement.
    Restore(WindowId),
    Snap {
        id: WindowId,
        region: SnapRegion,
    },
    SetLayout {
        workspace: WorkspaceId,
        layout: Layout,
    },
    SwitchWorkspace(WorkspaceId),
    MoveToWorkspace {
        id: WindowId,
        workspace: WorkspaceId,
    },
    /// Workspace count is fixed for the lifetime of a manager.
    Configure(Config),
}

/// Complete post-command snapshots, emitted in this order: removals by ID,
/// registrations/changes by ID, workspace changes by index, active workspace,
/// configuration. Empty batch means no state change. No intermediate state leaks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Removed(WindowId),
    Registered(Window),
    WindowChanged(Window),
    WorkspaceChanged(Workspace),
    ActiveWorkspaceChanged(WorkspaceId),
    ConfigChanged(Config),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidGeometry,
    InvalidConfig,
    UnknownWindow(WindowId),
    DuplicateWindow(WindowId),
    UnknownWorkspace(WorkspaceId),
    Capacity,
    InsufficientSpace,
    NotVisible(WindowId),
    NotFloating(WindowId),
    CorruptState,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidGeometry => {
                f.write_str("geometry must be nonzero with representable endpoints")
            }
            Self::InvalidConfig => f.write_str("invalid screen, gap, minimum size or capacity"),
            Self::UnknownWindow(id) => write!(f, "unknown window {}", id.0),
            Self::DuplicateWindow(id) => write!(f, "duplicate window {}", id.0),
            Self::UnknownWorkspace(id) => write!(f, "unknown workspace {}", id.0),
            Self::Capacity => f.write_str("window capacity reached"),
            Self::InsufficientSpace => f.write_str("layout cannot satisfy minimum window sizes"),
            Self::NotVisible(id) => write!(
                f,
                "window {} is minimized or on an inactive workspace",
                id.0
            ),
            Self::NotFloating(id) => write!(
                f,
                "window {} must be restored in a floating workspace to move/resize",
                id.0
            ),
            Self::CorruptState => f.write_str("window-manager invariant violated"),
        }
    }
}

impl std::error::Error for Error {}
