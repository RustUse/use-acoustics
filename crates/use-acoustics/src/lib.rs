#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Lower bound of the conventional audible range in hertz.
pub const AUDIBLE_MIN_HZ: f64 = 20.0;

/// Upper bound of the conventional audible range in hertz.
pub const AUDIBLE_MAX_HZ: f64 = 20_000.0;

/// Approximate speed of sound in dry air at 20 C, in meters per second.
pub const SPEED_OF_SOUND_MPS_AT_20C: f64 = 343.0;

/// Coarse acoustic bands used by the scaffold API.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcousticBand {
    Infrasound,
    Audible,
    Ultrasound,
}

impl AcousticBand {
    /// Returns a lowercase label for the acoustic band.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Infrasound => "infrasound",
            Self::Audible => "audible",
            Self::Ultrasound => "ultrasound",
        }
    }
}

/// A simple acoustic tone described by frequency and amplitude.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tone {
    pub frequency_hz: f64,
    pub amplitude: f64,
}

impl Tone {
    /// Creates a new tone value.
    #[must_use]
    pub const fn new(frequency_hz: f64, amplitude: f64) -> Self {
        Self {
            frequency_hz,
            amplitude,
        }
    }

    /// Returns the coarse acoustic band for the tone frequency.
    #[must_use]
    pub fn band(self) -> Option<AcousticBand> {
        classify_frequency_hz(self.frequency_hz)
    }
}

/// Classifies a frequency in hertz into a coarse acoustic band.
#[must_use]
pub fn classify_frequency_hz(frequency_hz: f64) -> Option<AcousticBand> {
    if !frequency_hz.is_finite() || frequency_hz <= 0.0 {
        return None;
    }

    if frequency_hz < AUDIBLE_MIN_HZ {
        Some(AcousticBand::Infrasound)
    } else if frequency_hz <= AUDIBLE_MAX_HZ {
        Some(AcousticBand::Audible)
    } else {
        Some(AcousticBand::Ultrasound)
    }
}

/// Returns whether a frequency falls inside the conventional audible range.
#[must_use]
pub fn is_audible_frequency_hz(frequency_hz: f64) -> bool {
    matches!(
        classify_frequency_hz(frequency_hz),
        Some(AcousticBand::Audible)
    )
}

/// Computes acoustic wavelength from frequency and medium speed.
#[must_use]
pub fn wavelength_meters(frequency_hz: f64, medium_speed_mps: f64) -> Option<f64> {
    if !frequency_hz.is_finite()
        || !medium_speed_mps.is_finite()
        || frequency_hz <= 0.0
        || medium_speed_mps <= 0.0
    {
        return None;
    }

    Some(medium_speed_mps / frequency_hz)
}

/// Converts a pressure ratio into sound pressure level in decibels.
#[must_use]
pub fn sound_pressure_level_db(pressure_pa: f64, reference_pressure_pa: f64) -> Option<f64> {
    if !pressure_pa.is_finite()
        || !reference_pressure_pa.is_finite()
        || pressure_pa <= 0.0
        || reference_pressure_pa <= 0.0
    {
        return None;
    }

    Some(20.0 * (pressure_pa / reference_pressure_pa).log10())
}

/// Common acoustic primitives.
pub mod prelude {
    pub use super::{
        AUDIBLE_MAX_HZ, AUDIBLE_MIN_HZ, AcousticBand, SPEED_OF_SOUND_MPS_AT_20C, Tone,
        classify_frequency_hz, is_audible_frequency_hz, sound_pressure_level_db, wavelength_meters,
    };
}

#[cfg(test)]
mod tests {
    use super::{
        AUDIBLE_MAX_HZ, AUDIBLE_MIN_HZ, AcousticBand, SPEED_OF_SOUND_MPS_AT_20C, Tone,
        classify_frequency_hz, is_audible_frequency_hz, sound_pressure_level_db, wavelength_meters,
    };

    #[test]
    fn classifies_common_frequency_ranges() {
        assert_eq!(classify_frequency_hz(10.0), Some(AcousticBand::Infrasound));
        assert_eq!(classify_frequency_hz(440.0), Some(AcousticBand::Audible));
        assert_eq!(
            classify_frequency_hz(25_000.0),
            Some(AcousticBand::Ultrasound)
        );
        assert_eq!(classify_frequency_hz(0.0), None);
    }

    #[test]
    fn detects_audible_frequencies() {
        assert!(is_audible_frequency_hz(AUDIBLE_MIN_HZ));
        assert!(is_audible_frequency_hz(AUDIBLE_MAX_HZ));
        assert!(!is_audible_frequency_hz(5.0));
    }

    #[test]
    fn computes_wavelengths() {
        let wavelength = wavelength_meters(343.0, SPEED_OF_SOUND_MPS_AT_20C)
            .expect("343 Hz in air should be valid");

        assert!((wavelength - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn computes_sound_pressure_level() {
        assert_eq!(sound_pressure_level_db(0.2, 0.000_02), Some(80.0));
        assert_eq!(sound_pressure_level_db(0.0, 0.000_02), None);
    }

    #[test]
    fn tone_reports_its_band() {
        let tone = Tone::new(440.0, 0.5);

        assert_eq!(tone.band(), Some(AcousticBand::Audible));
    }
}
