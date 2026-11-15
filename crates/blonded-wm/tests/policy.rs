use blonded_wm::*;

fn rect(x: i32, y: i32, width: u32, height: u32) -> Rect {
    Rect::new(x, y, width, height).unwrap()
}

fn config() -> Config {
    Config {
        screen: rect(0, 0, 1000, 700),
        gap: 10,
        minimum: Size::new(100, 80).unwrap(),
        workspace_count: 3,
        max_windows: 16,
    }
}

fn manager() -> WindowManager {
    WindowManager::new(config()).unwrap()
}

fn register(wm: &mut WindowManager, id: u64, workspace: u16) {
    wm.dispatch(Command::Register {
        id: WindowId(id),
        workspace: WorkspaceId(workspace),
        geometry: rect(50, 60, 300, 200),
    })
    .unwrap();
}

fn unchanged(wm: &mut WindowManager, command: Command, error: Error) {
    let before = wm.clone();
    assert_eq!(wm.dispatch(command), Err(error));
    assert_eq!(*wm, before);
    wm.check_invariants().unwrap();
}

#[test]
fn rejects_zero_dimensions_and_overflowing_endpoints() {
    for (width, height) in [(0, 1), (1, 0), (u32::MAX, 1), (1, u32::MAX)] {
        assert_eq!(Size::new(width, height), Err(Error::InvalidGeometry));
        assert_eq!(Rect::new(0, 0, width, height), Err(Error::InvalidGeometry));
    }
    assert_eq!(Rect::new(i32::MAX, 0, 1, 1), Err(Error::InvalidGeometry));
    assert_eq!(Rect::new(0, i32::MAX, 1, 1), Err(Error::InvalidGeometry));
    assert_eq!(
        rect(i32::MIN, i32::MIN, i32::MAX as u32, i32::MAX as u32).right(),
        -1
    );
    assert_eq!(rect(0, 0, i32::MAX as u32, 1).right(), i64::from(i32::MAX));
}

#[test]
fn rejects_invalid_configurations_before_allocation() {
    for invalid in [
        Config {
            workspace_count: 0,
            ..config()
        },
        Config {
            workspace_count: u16::MAX,
            ..config()
        },
        Config {
            max_windows: 0,
            ..config()
        },
        Config {
            max_windows: usize::MAX,
            ..config()
        },
        Config {
            gap: u32::MAX,
            ..config()
        },
        Config {
            gap: 350,
            ..config()
        },
        Config {
            minimum: Size::new(981, 80).unwrap(),
            ..config()
        },
    ] {
        assert_eq!(WindowManager::new(invalid), Err(Error::InvalidConfig));
    }
}

#[test]
fn registration_clamps_geometry_and_emits_complete_events() {
    let mut wm = manager();
    let events = wm
        .dispatch(Command::Register {
            id: WindowId(9),
            workspace: WorkspaceId(0),
            geometry: rect(-100, -500, 2000, 2000),
        })
        .unwrap();
    let window = *wm.window(WindowId(9)).unwrap();
    assert_eq!(window.geometry(), config().usable().unwrap());
    assert_eq!(wm.focused(), Some(WindowId(9)));
    assert_eq!(
        events,
        vec![
            Event::Registered(window),
            Event::WorkspaceChanged(wm.workspace(WorkspaceId(0)).unwrap().clone())
        ]
    );
    unchanged(
        &mut wm,
        Command::Register {
            id: WindowId(9),
            workspace: WorkspaceId(0),
            geometry: rect(0, 0, 1, 1),
        },
        Error::DuplicateWindow(WindowId(9)),
    );
    unchanged(
        &mut wm,
        Command::Register {
            id: WindowId(10),
            workspace: WorkspaceId(99),
            geometry: rect(0, 0, 1, 1),
        },
        Error::UnknownWorkspace(WorkspaceId(99)),
    );
}

#[test]
fn floating_moves_and_resizes_are_constrained_without_overflow() {
    let mut wm = manager();
    register(&mut wm, 1, 0);
    wm.dispatch(Command::Move {
        id: WindowId(1),
        x: i32::MAX,
        y: i32::MIN,
    })
    .unwrap();
    assert_eq!(
        wm.window(WindowId(1)).unwrap().geometry(),
        rect(690, 10, 300, 200)
    );
    wm.dispatch(Command::Resize {
        id: WindowId(1),
        size: Size::new(1, 1).unwrap(),
    })
    .unwrap();
    assert_eq!(
        wm.window(WindowId(1)).unwrap().geometry(),
        rect(690, 10, 100, 80)
    );
    wm.dispatch(Command::Resize {
        id: WindowId(1),
        size: Size::new(i32::MAX as u32, i32::MAX as u32).unwrap(),
    })
    .unwrap();
    assert_eq!(
        wm.window(WindowId(1)).unwrap().geometry(),
        rect(690, 10, 300, 680)
    );
    wm.dispatch(Command::Move {
        id: WindowId(1),
        x: i32::MIN,
        y: i32::MAX,
    })
    .unwrap();
    assert_eq!(
        wm.window(WindowId(1)).unwrap().geometry(),
        rect(10, 10, 300, 680)
    );
}

#[test]
fn focus_cycle_is_stable_and_raise_does_not_implicitly_focus() {
    let mut wm = manager();
    for id in 1..=3 {
        register(&mut wm, id, 0);
    }
    wm.dispatch(Command::Raise(WindowId(1))).unwrap();
    assert_eq!(wm.focused(), Some(WindowId(3)));
    assert_eq!(
        wm.workspace(WorkspaceId(0)).unwrap().stacking(),
        &[WindowId(2), WindowId(3), WindowId(1)]
    );
    for expected in [1, 2, 3, 1] {
        wm.dispatch(Command::CycleFocus(Direction::Next)).unwrap();
        assert_eq!(wm.focused(), Some(WindowId(expected)));
        assert_eq!(
            wm.workspace(WorkspaceId(0)).unwrap().stacking().last(),
            Some(&WindowId(expected))
        );
    }
    wm.dispatch(Command::CycleFocus(Direction::Previous))
        .unwrap();
    assert_eq!(wm.focused(), Some(WindowId(3)));
    wm.dispatch(Command::Minimize(WindowId(2))).unwrap();
    wm.dispatch(Command::CycleFocus(Direction::Previous))
        .unwrap();
    assert_eq!(wm.focused(), Some(WindowId(1)));
}

#[test]
fn maximize_minimize_and_restore_preserve_floating_geometry() {
    let mut wm = manager();
    register(&mut wm, 1, 0);
    register(&mut wm, 2, 0);
    let original = wm.window(WindowId(2)).unwrap().geometry();
    wm.dispatch(Command::Maximize(WindowId(2))).unwrap();
    assert_eq!(
        wm.window(WindowId(2)).unwrap().geometry(),
        config().usable().unwrap()
    );
    wm.dispatch(Command::Minimize(WindowId(2))).unwrap();
    assert!(!wm.is_visible(WindowId(2)));
    assert_eq!(wm.focused(), Some(WindowId(1)));
    unchanged(
        &mut wm,
        Command::Focus(WindowId(2)),
        Error::NotVisible(WindowId(2)),
    );
    unchanged(
        &mut wm,
        Command::Maximize(WindowId(2)),
        Error::NotVisible(WindowId(2)),
    );
    wm.dispatch(Command::Restore(WindowId(2))).unwrap();
    assert_eq!(wm.window(WindowId(2)).unwrap().mode(), Mode::Maximized);
    assert_eq!(wm.focused(), Some(WindowId(1)));
    wm.dispatch(Command::Restore(WindowId(2))).unwrap();
    assert_eq!(wm.window(WindowId(2)).unwrap().geometry(), original);
    assert!(wm
        .dispatch(Command::Restore(WindowId(2)))
        .unwrap()
        .is_empty());
}

#[test]
fn last_window_disappearing_clears_all_references_and_allows_id_reuse() {
    let mut wm = manager();
    register(&mut wm, 7, 0);
    let events = wm.dispatch(Command::Remove(WindowId(7))).unwrap();
    assert_eq!(events.first(), Some(&Event::Removed(WindowId(7))));
    assert_eq!(wm.focused(), None);
    assert!(wm.workspace(WorkspaceId(0)).unwrap().stacking().is_empty());
    assert!(!wm.is_visible(WindowId(7)));
    assert_eq!(wm.windows().len(), 0);
    assert!(wm
        .dispatch(Command::CycleFocus(Direction::Next))
        .unwrap()
        .is_empty());
    unchanged(
        &mut wm,
        Command::Remove(WindowId(7)),
        Error::UnknownWindow(WindowId(7)),
    );
    register(&mut wm, 7, 0);
    assert_eq!(wm.focused(), Some(WindowId(7)));
}

#[test]
fn focused_removal_selects_topmost_eligible_window() {
    let mut wm = manager();
    for id in 1..=3 {
        register(&mut wm, id, 0);
    }
    wm.dispatch(Command::Minimize(WindowId(2))).unwrap();
    wm.dispatch(Command::Remove(WindowId(3))).unwrap();
    assert_eq!(wm.focused(), Some(WindowId(1)));
    wm.dispatch(Command::Remove(WindowId(1))).unwrap();
    assert_eq!(wm.focused(), None);
    wm.dispatch(Command::Restore(WindowId(2))).unwrap();
    assert_eq!(wm.focused(), Some(WindowId(2)));
}

#[test]
fn workspace_focus_stacking_and_geometry_are_isolated() {
    let mut wm = manager();
    register(&mut wm, 1, 0);
    register(&mut wm, 2, 1);
    let other = *wm.window(WindowId(2)).unwrap();
    assert_eq!(wm.focused(), Some(WindowId(1)));
    unchanged(
        &mut wm,
        Command::Focus(WindowId(2)),
        Error::NotVisible(WindowId(2)),
    );
    wm.dispatch(Command::SetLayout {
        workspace: WorkspaceId(0),
        layout: Layout::Columns,
    })
    .unwrap();
    assert_eq!(*wm.window(WindowId(2)).unwrap(), other);
    wm.dispatch(Command::SwitchWorkspace(WorkspaceId(1)))
        .unwrap();
    assert_eq!(wm.focused(), Some(WindowId(2)));
    assert!(!wm.is_visible(WindowId(1)));
    wm.dispatch(Command::MoveToWorkspace {
        id: WindowId(2),
        workspace: WorkspaceId(0),
    })
    .unwrap();
    assert_eq!(wm.focused(), None);
    assert!(wm.workspace(WorkspaceId(1)).unwrap().stacking().is_empty());
    wm.dispatch(Command::SwitchWorkspace(WorkspaceId(0)))
        .unwrap();
    assert_eq!(wm.focused(), Some(WindowId(2)));
    assert_eq!(
        wm.window(WindowId(2)).unwrap().geometry(),
        rect(505, 10, 485, 680)
    );
    assert!(wm
        .dispatch(Command::MoveToWorkspace {
            id: WindowId(2),
            workspace: WorkspaceId(0)
        })
        .unwrap()
        .is_empty());
    unchanged(
        &mut wm,
        Command::SwitchWorkspace(WorkspaceId(u16::MAX)),
        Error::UnknownWorkspace(WorkspaceId(u16::MAX)),
    );
}

#[test]
fn moving_minimized_windows_does_not_steal_focus() {
    let mut wm = manager();
    register(&mut wm, 1, 0);
    register(&mut wm, 2, 1);
    wm.dispatch(Command::Minimize(WindowId(1))).unwrap();
    wm.dispatch(Command::MoveToWorkspace {
        id: WindowId(1),
        workspace: WorkspaceId(1),
    })
    .unwrap();
    assert_eq!(
        wm.workspace(WorkspaceId(1)).unwrap().focused(),
        Some(WindowId(2))
    );
    wm.dispatch(Command::Remove(WindowId(1))).unwrap();
    assert_eq!(
        wm.workspace(WorkspaceId(1)).unwrap().stacking(),
        &[WindowId(2)]
    );
}

#[test]
fn layouts_preserve_float_rectangles_and_use_id_order() {
    let mut wm = manager();
    for id in [3, 1, 2] {
        register(&mut wm, id, 0);
    }
    wm.dispatch(Command::SetLayout {
        workspace: WorkspaceId(0),
        layout: Layout::Columns,
    })
    .unwrap();
    assert_eq!(
        wm.window(WindowId(1)).unwrap().geometry(),
        rect(10, 10, 320, 680)
    );
    assert_eq!(
        wm.window(WindowId(2)).unwrap().geometry(),
        rect(340, 10, 320, 680)
    );
    assert_eq!(
        wm.window(WindowId(3)).unwrap().geometry(),
        rect(670, 10, 320, 680)
    );
    let before = wm.window(WindowId(1)).unwrap().geometry();
    wm.dispatch(Command::Focus(WindowId(1))).unwrap();
    assert_eq!(wm.window(WindowId(1)).unwrap().geometry(), before);
    unchanged(
        &mut wm,
        Command::Move {
            id: WindowId(1),
            x: 10,
            y: 10,
        },
        Error::NotFloating(WindowId(1)),
    );
    unchanged(
        &mut wm,
        Command::Resize {
            id: WindowId(1),
            size: Size::new(10, 10).unwrap(),
        },
        Error::NotFloating(WindowId(1)),
    );
    wm.dispatch(Command::SetLayout {
        workspace: WorkspaceId(0),
        layout: Layout::Rows,
    })
    .unwrap();
    assert_eq!(
        wm.window(WindowId(1)).unwrap().geometry(),
        rect(10, 10, 980, 220)
    );
    assert_eq!(
        wm.window(WindowId(3)).unwrap().geometry(),
        rect(10, 470, 980, 220)
    );
    wm.dispatch(Command::SetLayout {
        workspace: WorkspaceId(0),
        layout: Layout::Floating,
    })
    .unwrap();
    for window in wm.windows() {
        assert_eq!(window.geometry(), rect(50, 60, 300, 200));
    }
}

#[test]
fn minimize_maximize_and_removal_reflow_tiles() {
    let mut wm = manager();
    register(&mut wm, 1, 0);
    register(&mut wm, 2, 0);
    wm.dispatch(Command::SetLayout {
        workspace: WorkspaceId(0),
        layout: Layout::Columns,
    })
    .unwrap();
    wm.dispatch(Command::Minimize(WindowId(2))).unwrap();
    assert_eq!(
        wm.window(WindowId(1)).unwrap().geometry(),
        config().usable().unwrap()
    );
    wm.dispatch(Command::Restore(WindowId(2))).unwrap();
    assert_eq!(wm.window(WindowId(1)).unwrap().geometry().width(), 485);
    wm.dispatch(Command::Maximize(WindowId(2))).unwrap();
    assert_eq!(
        wm.window(WindowId(1)).unwrap().geometry(),
        config().usable().unwrap()
    );
    wm.dispatch(Command::Restore(WindowId(2))).unwrap();
    assert_eq!(wm.window(WindowId(1)).unwrap().geometry().width(), 485);
    let events = wm.dispatch(Command::Remove(WindowId(2))).unwrap();
    assert!(events
        .iter()
        .any(|event| matches!(event, Event::WindowChanged(w) if w.id() == WindowId(1))));
    assert_eq!(
        wm.window(WindowId(1)).unwrap().geometry(),
        config().usable().unwrap()
    );
}

#[test]
fn all_snap_regions_are_exact_and_restore_original_placement() {
    let mut wm = manager();
    register(&mut wm, 1, 0);
    for (region, expected) in [
        (SnapRegion::Left, rect(10, 10, 485, 680)),
        (SnapRegion::Right, rect(505, 10, 485, 680)),
        (SnapRegion::Top, rect(10, 10, 980, 335)),
        (SnapRegion::Bottom, rect(10, 355, 980, 335)),
        (SnapRegion::TopLeft, rect(10, 10, 485, 335)),
        (SnapRegion::TopRight, rect(505, 10, 485, 335)),
        (SnapRegion::BottomLeft, rect(10, 355, 485, 335)),
        (SnapRegion::BottomRight, rect(505, 355, 485, 335)),
    ] {
        wm.dispatch(Command::Snap {
            id: WindowId(1),
            region,
        })
        .unwrap();
        assert_eq!(wm.window(WindowId(1)).unwrap().geometry(), expected);
        unchanged(
            &mut wm,
            Command::Move {
                id: WindowId(1),
                x: 1,
                y: 1,
            },
            Error::NotFloating(WindowId(1)),
        );
        wm.dispatch(Command::Restore(WindowId(1))).unwrap();
        assert_eq!(
            wm.window(WindowId(1)).unwrap().geometry(),
            rect(50, 60, 300, 200)
        );
    }
}

#[test]
fn failed_layout_snap_registration_and_workspace_transfer_are_atomic() {
    let mut wm = WindowManager::new(Config {
        screen: rect(0, 0, 210, 180),
        ..config()
    })
    .unwrap();
    register(&mut wm, 1, 0);
    register(&mut wm, 2, 0);
    unchanged(
        &mut wm,
        Command::SetLayout {
            workspace: WorkspaceId(0),
            layout: Layout::Columns,
        },
        Error::InsufficientSpace,
    );
    unchanged(
        &mut wm,
        Command::Snap {
            id: WindowId(1),
            region: SnapRegion::Left,
        },
        Error::InsufficientSpace,
    );
    wm.dispatch(Command::SetLayout {
        workspace: WorkspaceId(1),
        layout: Layout::Columns,
    })
    .unwrap();
    register(&mut wm, 3, 1);
    unchanged(
        &mut wm,
        Command::MoveToWorkspace {
            id: WindowId(1),
            workspace: WorkspaceId(1),
        },
        Error::InsufficientSpace,
    );
    unchanged(
        &mut wm,
        Command::Register {
            id: WindowId(4),
            workspace: WorkspaceId(1),
            geometry: rect(0, 0, 100, 80),
        },
        Error::InsufficientSpace,
    );
}

#[test]
fn display_reconfiguration_reflows_all_workspaces_and_is_atomic() {
    let mut wm = manager();
    register(&mut wm, 1, 0);
    register(&mut wm, 2, 1);
    wm.dispatch(Command::Snap {
        id: WindowId(2),
        region: SnapRegion::BottomRight,
    })
    .unwrap();
    let new = Config {
        screen: rect(-800, -600, 800, 600),
        gap: 5,
        minimum: Size::new(150, 120).unwrap(),
        ..config()
    };
    let events = wm.dispatch(Command::Configure(new)).unwrap();
    assert_eq!(wm.config(), new);
    assert_eq!(events.last(), Some(&Event::ConfigChanged(new)));
    for window in wm.windows() {
        assert!(new.usable().unwrap().contains(window.geometry()));
    }
    assert_eq!(
        wm.window(WindowId(2)).unwrap().geometry(),
        rect(-397, -297, 392, 292)
    );
    unchanged(
        &mut wm,
        Command::Configure(Config {
            screen: rect(0, 0, 200, 180),
            ..new
        }),
        Error::InsufficientSpace,
    );
    unchanged(
        &mut wm,
        Command::Configure(Config {
            workspace_count: 2,
            ..new
        }),
        Error::InvalidConfig,
    );
    unchanged(
        &mut wm,
        Command::Configure(Config {
            max_windows: 1,
            ..new
        }),
        Error::InvalidConfig,
    );
}

#[test]
fn unknown_window_commands_are_errors_with_no_effects() {
    let mut wm = manager();
    let id = WindowId(u64::MAX);
    for command in [
        Command::Remove(id),
        Command::Focus(id),
        Command::Raise(id),
        Command::Maximize(id),
        Command::Minimize(id),
        Command::Restore(id),
        Command::Move { id, x: 0, y: 0 },
        Command::Resize {
            id,
            size: Size::new(1, 1).unwrap(),
        },
        Command::Snap {
            id,
            region: SnapRegion::Left,
        },
        Command::MoveToWorkspace {
            id,
            workspace: WorkspaceId(0),
        },
    ] {
        unchanged(&mut wm, command, Error::UnknownWindow(id));
    }
}

#[test]
fn capacity_is_bounded_and_removal_releases_it() {
    let mut wm = WindowManager::new(Config {
        max_windows: 1,
        ..config()
    })
    .unwrap();
    register(&mut wm, 1, 0);
    unchanged(
        &mut wm,
        Command::Register {
            id: WindowId(2),
            workspace: WorkspaceId(0),
            geometry: rect(0, 0, 1, 1),
        },
        Error::Capacity,
    );
    wm.dispatch(Command::Remove(WindowId(1))).unwrap();
    register(&mut wm, 2, 0);
}

#[test]
fn hard_capacity_and_maximum_workspace_index_are_usable() {
    let mut wm = WindowManager::new(Config {
        max_windows: MAX_WINDOWS,
        workspace_count: MAX_WORKSPACES,
        gap: 0,
        minimum: Size::new(1, 1).unwrap(),
        ..config()
    })
    .unwrap();
    for id in 0..MAX_WINDOWS as u64 {
        register(&mut wm, id, 0);
    }
    wm.dispatch(Command::SetLayout {
        workspace: WorkspaceId(0),
        layout: Layout::Columns,
    })
    .unwrap();
    assert_eq!(wm.windows().len(), MAX_WINDOWS);
    unchanged(
        &mut wm,
        Command::Register {
            id: WindowId(u64::MAX),
            workspace: WorkspaceId(0),
            geometry: rect(0, 0, 1, 1),
        },
        Error::Capacity,
    );
    let events = wm
        .dispatch(Command::MoveToWorkspace {
            id: WindowId(0),
            workspace: WorkspaceId(MAX_WORKSPACES - 1),
        })
        .unwrap();
    assert!(events.len() <= MAX_WINDOWS + usize::from(MAX_WORKSPACES) + 2);
    wm.dispatch(Command::SwitchWorkspace(WorkspaceId(MAX_WORKSPACES - 1)))
        .unwrap();
    assert_eq!(wm.focused(), Some(WindowId(0)));
    for id in 0..MAX_WINDOWS as u64 {
        wm.dispatch(Command::Remove(WindowId(id))).unwrap();
    }
    assert_eq!(wm.focused(), None);
    for index in 0..MAX_WORKSPACES {
        let ws = wm.workspace(WorkspaceId(index)).unwrap();
        assert!(ws.stacking().is_empty());
        assert_eq!(ws.focused(), None);
    }
}

#[test]
fn restore_that_cannot_fit_the_tiled_workspace_is_atomic() {
    let mut wm = WindowManager::new(Config {
        screen: rect(0, 0, 210, 180),
        ..config()
    })
    .unwrap();
    register(&mut wm, 1, 0);
    register(&mut wm, 2, 0);
    wm.dispatch(Command::Minimize(WindowId(2))).unwrap();
    wm.dispatch(Command::SetLayout {
        workspace: WorkspaceId(0),
        layout: Layout::Columns,
    })
    .unwrap();
    unchanged(
        &mut wm,
        Command::Restore(WindowId(2)),
        Error::InsufficientSpace,
    );
    wm.dispatch(Command::SetLayout {
        workspace: WorkspaceId(0),
        layout: Layout::Floating,
    })
    .unwrap();
    wm.dispatch(Command::Restore(WindowId(2))).unwrap();
    assert!(wm.is_visible(WindowId(2)));
}

#[test]
fn extreme_screen_origins_support_move_resize_snap_and_tiling() {
    for origin in [i32::MIN, -1000, 0, i32::MAX - 1000] {
        let mut wm = WindowManager::new(Config {
            screen: rect(origin, origin, 1000, 700),
            ..config()
        })
        .unwrap();
        register(&mut wm, 1, 0);
        register(&mut wm, 2, 0);
        for (x, y) in [(i32::MIN, i32::MAX), (i32::MAX, i32::MIN)] {
            wm.dispatch(Command::Move {
                id: WindowId(1),
                x,
                y,
            })
            .unwrap();
            wm.dispatch(Command::Resize {
                id: WindowId(1),
                size: Size::new(i32::MAX as u32, i32::MAX as u32).unwrap(),
            })
            .unwrap();
        }
        wm.dispatch(Command::Snap {
            id: WindowId(1),
            region: SnapRegion::BottomRight,
        })
        .unwrap();
        wm.dispatch(Command::Restore(WindowId(1))).unwrap();
        wm.dispatch(Command::SetLayout {
            workspace: WorkspaceId(0),
            layout: Layout::Rows,
        })
        .unwrap();
        wm.check_invariants().unwrap();
    }
}

#[test]
fn deterministic_sequence_replay_preserves_invariants_on_success_and_failure() {
    let mut left = manager();
    let mut right = manager();
    let mut seed = 42_u64;
    for step in 0..6000 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let id = WindowId((seed >> 32) % 20);
        let workspace = WorkspaceId(((seed >> 24) % 5) as u16);
        let command = match seed % 15 {
            0 => Command::Register {
                id,
                workspace,
                geometry: rect(-200, 900, 450, 400),
            },
            1 => Command::Remove(id),
            2 => Command::Focus(id),
            3 => Command::Move {
                id,
                x: seed as i32,
                y: (seed >> 32) as i32,
            },
            4 => Command::Resize {
                id,
                size: Size::new(1 + (seed % 2000) as u32, 1 + ((seed >> 20) % 2000) as u32)
                    .unwrap(),
            },
            5 => Command::CycleFocus(Direction::Next),
            6 => Command::Maximize(id),
            7 => Command::Minimize(id),
            8 => Command::Restore(id),
            9 => Command::MoveToWorkspace { id, workspace },
            10 => Command::SwitchWorkspace(workspace),
            11 => Command::SetLayout {
                workspace,
                layout: Layout::Columns,
            },
            12 => Command::SetLayout {
                workspace,
                layout: Layout::Floating,
            },
            13 => Command::Snap {
                id,
                region: SnapRegion::BottomRight,
            },
            _ => Command::Raise(id),
        };
        let before = left.clone();
        let a = left.dispatch(command);
        let b = right.dispatch(command);
        assert_eq!(a, b, "replay step {step}");
        assert_eq!(left, right, "replay step {step}");
        if a.is_err() {
            assert_eq!(left, before, "atomicity step {step}");
        }
        left.check_invariants().unwrap();
    }
}

#[test]
fn integer_tiling_partitions_cover_bounds_without_overlap_or_pixel_loss() {
    for width in 7..40 {
        for count in 1..=6 {
            for gap in 0..=2 {
                let cfg = Config {
                    screen: rect(-20, -10, width, 30),
                    gap,
                    minimum: Size::new(1, 1).unwrap(),
                    workspace_count: 1,
                    max_windows: 6,
                };
                let mut wm = WindowManager::new(cfg).unwrap();
                for id in 0..count {
                    register(&mut wm, id, 0);
                }
                let before = wm.clone();
                let result = wm.dispatch(Command::SetLayout {
                    workspace: WorkspaceId(0),
                    layout: Layout::Columns,
                });
                if width - 2 * gap < count as u32 + gap * (count as u32 - 1) {
                    assert_eq!(result, Err(Error::InsufficientSpace));
                    assert_eq!(wm, before);
                    continue;
                }
                result.unwrap();
                let bounds = cfg.usable().unwrap();
                let mut edge = i64::from(bounds.x());
                let mut total = 0_u32;
                let mut minimum = u32::MAX;
                let mut maximum = 0;
                for window in wm.windows() {
                    let cell = window.geometry();
                    assert_eq!(i64::from(cell.x()), edge);
                    assert_eq!(cell.y(), bounds.y());
                    assert_eq!(cell.height(), bounds.height());
                    assert!(bounds.contains(cell));
                    total += cell.width();
                    minimum = minimum.min(cell.width());
                    maximum = maximum.max(cell.width());
                    edge = cell.right() + i64::from(gap);
                }
                assert_eq!(total + gap * (count as u32 - 1), bounds.width());
                assert!(maximum - minimum <= 1);
            }
        }
    }
}
