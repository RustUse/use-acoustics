use crate::decibel::REFERENCE_SOUND_PRESSURE_AIR_PA;

/// A validated sound pressure in pascals.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct SoundPressurePa(f64);

impl SoundPressurePa {
    /// Creates a validated sound pressure value.
    #[must_use]
    pub fn new(value: f64) -> Option<Self> {
        if is_valid_positive_scalar(value) {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Returns the stored pressure in pascals.
    #[must_use]
    pub fn value(self) -> f64 {
        self.0
    }

    /// Returns the sound pressure level for this pressure.
    #[must_use]
    pub fn level_db_spl(self) -> f64 {
        20.0 * (self.0.log10() - REFERENCE_SOUND_PRESSURE_AIR_PA.log10())
    }
}

/// Computes sound pressure in pascals from a sound pressure level in decibels.
#[must_use]
pub fn pressure_from_spl_db(db_spl: f64) -> Option<f64> {
    if !db_spl.is_finite() {
        return None;
    }

    let pressure = REFERENCE_SOUND_PRESSURE_AIR_PA * 10.0_f64.powf(db_spl / 20.0);

    if pressure.is_finite() && pressure > 0.0 {
        Some(pressure)
    } else {
        None
    }
}

fn is_valid_positive_scalar(value: f64) -> bool {
    value.is_finite() && value > 0.0
}
