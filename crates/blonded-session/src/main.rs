#![doc = "Blonded OS status-only binary. This is NOT a functioning graphical session."]

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("--status") | Some("--about") => {
            println!("Blonded OS: M00 hardware audit / distribution scaffold");
            println!(
                "Desktop steady-idle RAM ceiling: {} MiB (UNVERIFIED target)",
                blonded_core::DESKTOP_IDLE_MEMORY_LIMIT_MIB
            );
            println!("No bootable OS image or working custom compositor exists yet.");
        }
        Some("--version") => {
            println!("blonded-session {} (scaffold)", env!("CARGO_PKG_VERSION"));
        }
        _ => {
            eprintln!("Blonded OS M00: graphical session not implemented. Try --status.");
            std::process::exit(2);
        }
    }
}
