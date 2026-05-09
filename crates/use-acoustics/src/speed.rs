/// Approximate speed of sound in dry air at 20 C, in meters per second.
pub const SPEED_OF_SOUND_AIR_20C_MPS: f64 = 343.0;

/// Computes the approximate speed of sound in dry air for a given temperature in Celsius.
#[must_use]
pub fn sound_speed_air_mps(temperature_celsius: f64) -> f64 {
    331.3 + (0.606 * temperature_celsius)
}
