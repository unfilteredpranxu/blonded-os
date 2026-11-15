# Legacy distribution-scaffolding directory — not active

The project originally considered a standalone Arch-derived ISO and installer. **On 2026-11-15, the owner explicitly chose to build Blonded OS directly on the existing Arch installation instead.**

`os/archiso`, `os/installer`, `os/packages`, `os/updates` and `os/recovery` are historical placeholders, not current implementation goals. Do **not** build an ISO, reformat partitions, overwrite the OS or modify boot.

Active work belongs in Rust `crates/` and safe user-owned desktop integration components, first as a manually run test with rollback and zero changes to the existing Hyprland setup. See `docs/IN-PLACE-ARCH-STRATEGY.md`.
