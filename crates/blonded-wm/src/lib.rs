//! Original, deterministic window-management policy, independent of display protocols.
//!
//! A backend translates input/lifecycle notifications to [`Command`] and consumes
//! the returned [`Event`] batch only after a successful dispatch. No graphics,
//! I/O, threads, clock, or host configuration is accessed by this crate.

mod geometry;
mod manager;
mod model;

pub use geometry::{Rect, Size};
pub use manager::WindowManager;
pub use model::*;
