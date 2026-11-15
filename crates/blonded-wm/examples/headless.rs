//! A simulation only; no display sockets or real application windows are used.
use blonded_wm::{
    Command, Config, Direction, Layout, Rect, Size, SnapRegion, WindowId, WindowManager,
    WorkspaceId,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("BLONDED WM: headless policy simulation; no visible Linux windows");
    let mut wm = WindowManager::new(Config {
        screen: Rect::new(0, 0, 1366, 768)?,
        gap: 8,
        minimum: Size::new(120, 80)?,
        workspace_count: 3,
        max_windows: 64,
    })?;
    let first = WindowId(1);
    let second = WindowId(2);
    let home = WorkspaceId(0);
    let study = WorkspaceId(1);
    let commands = [
        Command::Register {
            id: first,
            workspace: home,
            geometry: Rect::new(40, 60, 500, 320)?,
        },
        Command::Register {
            id: second,
            workspace: home,
            geometry: Rect::new(200, 140, 480, 360)?,
        },
        Command::Move {
            id: first,
            x: 90,
            y: 110,
        },
        Command::Resize {
            id: first,
            size: Size::new(600, 400)?,
        },
        Command::Focus(first),
        Command::CycleFocus(Direction::Next),
        Command::Snap {
            id: first,
            region: SnapRegion::Left,
        },
        Command::Restore(first),
        Command::Maximize(second),
        Command::Minimize(second),
        Command::Restore(second),
        Command::Restore(second),
        Command::SetLayout {
            workspace: home,
            layout: Layout::Columns,
        },
        Command::MoveToWorkspace {
            id: second,
            workspace: study,
        },
        Command::SwitchWorkspace(study),
        Command::Focus(second),
        Command::Remove(second),
        Command::SwitchWorkspace(home),
        Command::SetLayout {
            workspace: home,
            layout: Layout::Floating,
        },
    ];
    for (step, command) in commands.into_iter().enumerate() {
        println!("\nstep {}: {command:?}", step + 1);
        let events = wm.dispatch(command)?;
        for event in events {
            println!("  event: {event:?}");
        }
        println!(
            "  active={:?} focus={:?}",
            wm.active_workspace(),
            wm.focused()
        );
        for window in wm.windows() {
            println!(
                "  window={} workspace={} rect={:?} mode={:?} minimized={} visible={}",
                window.id().0,
                window.workspace().0,
                window.geometry(),
                window.mode(),
                window.is_minimized(),
                wm.is_visible(window.id())
            );
        }
        wm.check_invariants()?;
    }
    println!(
        "\nPASS: {} commands completed; invariants hold",
        commands.len()
    );
    Ok(())
}
