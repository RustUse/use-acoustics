use std::f64::consts::PI;

/// A validated acoustic frequency in hertz.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct FrequencyHz(f64);

impl FrequencyHz {
    /// Creates a validated frequency in hertz.
    #[must_use]
    pub fn new(value: f64) -> Option<Self> {
        if is_valid_frequency(value) {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Returns the stored frequency value.
    #[must_use]
    pub fn value(self) -> f64 {
        self.0
    }

    /// Returns the period in seconds for this frequency.
    #[must_use]
    pub fn period_seconds(self) -> f64 {
        1.0 / self.0
    }

    /// Returns the angular frequency in radians per second.
    #[must_use]
    pub fn angular_frequency_rad_per_s(self) -> f64 {
        2.0 * PI * self.0
    }
}

/// Returns the period in seconds for a validated frequency.
#[must_use]
pub fn period_seconds(frequency_hz: f64) -> Option<f64> {
    FrequencyHz::new(frequency_hz).map(FrequencyHz::period_seconds)
}

/// Returns the angular frequency in radians per second for a validated frequency.
#[must_use]
pub fn angular_frequency_rad_per_s(frequency_hz: f64) -> Option<f64> {
    FrequencyHz::new(frequency_hz).map(FrequencyHz::angular_frequency_rad_per_s)
}

fn is_valid_frequency(value: f64) -> bool {
    value.is_finite() && value > 0.0
}
