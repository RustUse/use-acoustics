/// Reference sound pressure in air, in pascals.
pub const REFERENCE_SOUND_PRESSURE_AIR_PA: f64 = 20e-6;

/// Reference sound intensity, in watts per square meter.
pub const REFERENCE_SOUND_INTENSITY_W_PER_M2: f64 = 1e-12;

/// Converts an amplitude ratio into decibels.
#[must_use]
pub fn amplitude_ratio_to_db(ratio: f64) -> Option<f64> {
    if !is_valid_positive_scalar(ratio) {
        return None;
    }

    Some(20.0 * ratio.log10())
}

/// Converts a power ratio into decibels.
#[must_use]
pub fn power_ratio_to_db(ratio: f64) -> Option<f64> {
    if !is_valid_positive_scalar(ratio) {
        return None;
    }

    Some(10.0 * ratio.log10())
}

/// Converts decibels into an amplitude ratio.
#[must_use]
pub fn db_to_amplitude_ratio(db: f64) -> Option<f64> {
    if !db.is_finite() {
        return None;
    }

    Some(10.0_f64.powf(db / 20.0))
}

/// Converts decibels into a power ratio.
#[must_use]
pub fn db_to_power_ratio(db: f64) -> Option<f64> {
    if !db.is_finite() {
        return None;
    }

    Some(10.0_f64.powf(db / 10.0))
}

/// Computes sound pressure level in decibels relative to standard air reference pressure.
#[must_use]
pub fn sound_pressure_level_db(pressure_pa: f64) -> Option<f64> {
    if !is_valid_positive_scalar(pressure_pa) {
        return None;
    }

    Some(20.0 * (pressure_pa.log10() - REFERENCE_SOUND_PRESSURE_AIR_PA.log10()))
}

/// Computes sound intensity level in decibels relative to standard air reference intensity.
#[must_use]
pub fn sound_intensity_level_db(intensity_w_per_m2: f64) -> Option<f64> {
    if !is_valid_positive_scalar(intensity_w_per_m2) {
        return None;
    }

    Some(10.0 * (intensity_w_per_m2.log10() - REFERENCE_SOUND_INTENSITY_W_PER_M2.log10()))
}

fn is_valid_positive_scalar(value: f64) -> bool {
    value.is_finite() && value > 0.0
}
