use crate::decibel::REFERENCE_SOUND_INTENSITY_W_PER_M2;

/// A validated sound intensity in watts per square meter.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct SoundIntensityWPerM2(f64);

impl SoundIntensityWPerM2 {
    /// Creates a validated sound intensity value.
    #[must_use]
    pub fn new(value: f64) -> Option<Self> {
        if is_valid_positive_scalar(value) {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Returns the stored intensity value.
    #[must_use]
    pub fn value(self) -> f64 {
        self.0
    }

    /// Returns the sound intensity level for this intensity.
    #[must_use]
    pub fn level_db_sil(self) -> f64 {
        10.0 * (self.0.log10() - REFERENCE_SOUND_INTENSITY_W_PER_M2.log10())
    }
}

/// Computes sound intensity in watts per square meter from sound intensity level in decibels.
#[must_use]
pub fn intensity_from_sil_db(db_sil: f64) -> Option<f64> {
    if !db_sil.is_finite() {
        return None;
    }

    let intensity = REFERENCE_SOUND_INTENSITY_W_PER_M2 * 10.0_f64.powf(db_sil / 10.0);

    if intensity.is_finite() && intensity > 0.0 {
        Some(intensity)
    } else {
        None
    }
}

fn is_valid_positive_scalar(value: f64) -> bool {
    value.is_finite() && value > 0.0
}
