//! Bounded synthetic policy workload; measures no rendering or system idle RAM.
use std::time::Instant;

use blonded_wm::{
    Command, Config, Direction, Layout, Rect, Size, WindowId, WindowManager, WorkspaceId,
    MAX_WINDOWS, MAX_WORKSPACES,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wm = WindowManager::new(Config {
        screen: Rect::new(0, 0, 1366, 768)?,
        gap: 0,
        minimum: Size::new(1, 1)?,
        workspace_count: MAX_WORKSPACES,
        max_windows: MAX_WINDOWS,
    })?;
    for id in 0..MAX_WINDOWS as u64 {
        wm.dispatch(Command::Register {
            id: WindowId(id),
            workspace: WorkspaceId(0),
            geometry: Rect::new(0, 0, 320, 200)?,
        })?;
    }
    let start = Instant::now();
    let operations = 4096_u64;
    let mut events = 0_usize;
    for index in 0..operations {
        let command = match index % 4 {
            0 => Command::SetLayout {
                workspace: WorkspaceId(0),
                layout: Layout::Columns,
            },
            1 => Command::CycleFocus(Direction::Next),
            2 => Command::SetLayout {
                workspace: WorkspaceId(0),
                layout: Layout::Rows,
            },
            _ => Command::Focus(WindowId(index % MAX_WINDOWS as u64)),
        };
        events += wm.dispatch(command)?.len();
    }
    let elapsed = start.elapsed();
    wm.check_invariants()?;
    println!("synthetic headless policy only: windows={} workspaces={} commands={operations} events={events}", wm.windows().len(), wm.config().workspace_count);
    println!(
        "elapsed_us={} mean_ns_per_dispatch={}",
        elapsed.as_micros(),
        elapsed.as_nanos() / u128::from(operations)
    );
    println!("PASS: bounded state and final invariants; no compositor/FPS/system-idle measurement");
    Ok(())
}
