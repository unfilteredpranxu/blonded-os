#![doc = "Blonded OS shared project policy scaffolding. No usable window manager exists yet."]

/// Maximum allowed **system-wide** steady-idle memory, measured in MiB.
pub const DESKTOP_IDLE_MEMORY_LIMIT_MIB: u64 = 500;

/// Configurable animation behavior profiles. Frame rates are targets only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionProfile {
    Full,
    Balanced,
    Eco,
    Safe,
}

impl MotionProfile {
    pub const fn nominal_fps_cap(self) -> u32 {
        match self {
            Self::Full => 60,
            Self::Balanced => 60,
            Self::Eco => 30,
            Self::Safe => 15,
        }
    }
}

/// Whole-window opacity differs from background-only opacity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowMaterial {
    pub window_opacity_percent: u8,
    pub background_opacity_percent: u8,
    pub blur_strength_percent: u8,
}

impl WindowMaterial {
    pub const fn is_valid(self) -> bool {
        self.window_opacity_percent <= 100
            && self.background_opacity_percent <= 100
            && self.blur_strength_percent <= 100
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn motion_profiles_are_bounded() {
        assert!(MotionProfile::Full.nominal_fps_cap() <= 60);
        assert!(MotionProfile::Eco.nominal_fps_cap() < MotionProfile::Full.nominal_fps_cap());
    }

    #[test]
    fn material_percentages_are_checked() {
        let valid = WindowMaterial {
            window_opacity_percent: 100,
            background_opacity_percent: 60,
            blur_strength_percent: 35,
        };
        assert!(valid.is_valid());
        assert!(!WindowMaterial {
            blur_strength_percent: 101,
            ..valid
        }
        .is_valid());
    }
}
