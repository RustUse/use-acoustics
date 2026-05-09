/// A validated wavelength in meters.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct WavelengthMeters(f64);

impl WavelengthMeters {
    /// Creates a validated wavelength in meters.
    #[must_use]
    pub fn new(value: f64) -> Option<Self> {
        if is_valid_positive_scalar(value) {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Returns the stored wavelength in meters.
    #[must_use]
    pub fn value(self) -> f64 {
        self.0
    }
}

/// Computes wavelength in meters from frequency and propagation speed.
#[must_use]
pub fn wavelength_meters(frequency_hz: f64, speed_mps: f64) -> Option<f64> {
    if !is_valid_positive_scalar(frequency_hz) || !is_valid_positive_scalar(speed_mps) {
        return None;
    }

    Some(speed_mps / frequency_hz)
}

/// Computes frequency in hertz from wavelength and propagation speed.
#[must_use]
pub fn frequency_from_wavelength_hz(wavelength_meters: f64, speed_mps: f64) -> Option<f64> {
    if !is_valid_positive_scalar(wavelength_meters) || !is_valid_positive_scalar(speed_mps) {
        return None;
    }

    Some(speed_mps / wavelength_meters)
}

fn is_valid_positive_scalar(value: f64) -> bool {
    value.is_finite() && value > 0.0
}
