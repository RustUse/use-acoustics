use std::f64::consts::PI;

use use_acoustics::decibel::{
    REFERENCE_SOUND_INTENSITY_W_PER_M2, REFERENCE_SOUND_PRESSURE_AIR_PA, amplitude_ratio_to_db,
    db_to_amplitude_ratio, db_to_power_ratio, power_ratio_to_db, sound_intensity_level_db,
    sound_pressure_level_db,
};
use use_acoustics::frequency::{FrequencyHz, angular_frequency_rad_per_s, period_seconds};
use use_acoustics::intensity::{SoundIntensityWPerM2, intensity_from_sil_db};
use use_acoustics::pressure::{SoundPressurePa, pressure_from_spl_db};
use use_acoustics::speed::{SPEED_OF_SOUND_AIR_20C_MPS, sound_speed_air_mps};
use use_acoustics::wavelength::{
    WavelengthMeters, frequency_from_wavelength_hz, wavelength_meters,
};

fn assert_approx_eq(left: f64, right: f64, epsilon: f64) {
    assert!(
        (left - right).abs() <= epsilon,
        "left={left}, right={right}, epsilon={epsilon}"
    );
}

fn invalid_positive_inputs() -> [f64; 4] {
    [0.0, -1.0, f64::NAN, f64::INFINITY]
}

#[test]
fn speed_helpers_match_expected_reference_values() {
    assert_approx_eq(SPEED_OF_SOUND_AIR_20C_MPS, 343.0, 1.0e-12);
    assert_approx_eq(sound_speed_air_mps(20.0), 343.42, 1.0e-12);
    assert_approx_eq(sound_speed_air_mps(0.0), 331.3, 1.0e-12);
}

#[test]
fn frequency_newtype_and_helpers_work() {
    let frequency = FrequencyHz::new(440.0).expect("440 Hz should be valid");

    assert_approx_eq(frequency.value(), 440.0, 1.0e-12);
    assert_approx_eq(frequency.period_seconds(), 1.0 / 440.0, 1.0e-12);
    assert_approx_eq(
        frequency.angular_frequency_rad_per_s(),
        2.0 * PI * 440.0,
        1.0e-9,
    );
    assert_approx_eq(
        period_seconds(440.0).expect("440 Hz should be valid"),
        1.0 / 440.0,
        1.0e-12,
    );
    assert_approx_eq(
        angular_frequency_rad_per_s(440.0).expect("440 Hz should be valid"),
        2.0 * PI * 440.0,
        1.0e-9,
    );

    for value in invalid_positive_inputs() {
        assert!(FrequencyHz::new(value).is_none(), "value={value}");
        assert!(period_seconds(value).is_none(), "value={value}");
        assert!(
            angular_frequency_rad_per_s(value).is_none(),
            "value={value}"
        );
    }
}

#[test]
fn wavelength_newtype_and_helpers_work() {
    let wavelength_value = wavelength_meters(440.0, 343.0).expect("inputs should be valid");
    let wavelength = WavelengthMeters::new(wavelength_value).expect("wavelength should be valid");

    assert_approx_eq(wavelength.value(), 343.0 / 440.0, 1.0e-12);
    assert_approx_eq(wavelength_value, 0.779_545_454_545_454_5, 1.0e-12);
    assert_approx_eq(
        frequency_from_wavelength_hz(1.0, 343.0).expect("inputs should be valid"),
        343.0,
        1.0e-12,
    );

    for value in invalid_positive_inputs() {
        assert!(WavelengthMeters::new(value).is_none(), "value={value}");
        assert!(wavelength_meters(value, 343.0).is_none(), "value={value}");
        assert!(wavelength_meters(440.0, value).is_none(), "value={value}");
        assert!(
            frequency_from_wavelength_hz(value, 343.0).is_none(),
            "value={value}"
        );
        assert!(
            frequency_from_wavelength_hz(1.0, value).is_none(),
            "value={value}"
        );
    }
}

#[test]
fn decibel_helpers_cover_ratios_and_reference_levels() {
    assert_approx_eq(
        amplitude_ratio_to_db(10.0).expect("ratio should be valid"),
        20.0,
        1.0e-12,
    );
    assert_approx_eq(
        power_ratio_to_db(10.0).expect("ratio should be valid"),
        10.0,
        1.0e-12,
    );
    assert_approx_eq(
        db_to_amplitude_ratio(20.0).expect("db should be valid"),
        10.0,
        1.0e-12,
    );
    assert_approx_eq(
        db_to_power_ratio(10.0).expect("db should be valid"),
        10.0,
        1.0e-12,
    );
    assert_approx_eq(
        sound_pressure_level_db(REFERENCE_SOUND_PRESSURE_AIR_PA).expect("pressure should be valid"),
        0.0,
        1.0e-12,
    );
    assert_approx_eq(
        sound_intensity_level_db(REFERENCE_SOUND_INTENSITY_W_PER_M2)
            .expect("intensity should be valid"),
        0.0,
        1.0e-12,
    );
    assert_approx_eq(
        sound_pressure_level_db(1.0).expect("pressure should be valid"),
        93.979_400_086_720_37,
        1.0e-9,
    );

    for value in invalid_positive_inputs() {
        assert!(amplitude_ratio_to_db(value).is_none(), "value={value}");
        assert!(power_ratio_to_db(value).is_none(), "value={value}");
        assert!(sound_pressure_level_db(value).is_none(), "value={value}");
        assert!(sound_intensity_level_db(value).is_none(), "value={value}");
    }

    assert!(db_to_amplitude_ratio(f64::NAN).is_none());
    assert!(db_to_amplitude_ratio(f64::INFINITY).is_none());
    assert!(db_to_power_ratio(f64::NAN).is_none());
    assert!(db_to_power_ratio(f64::INFINITY).is_none());
}

#[test]
fn pressure_helpers_round_trip_levels() {
    let pressure = SoundPressurePa::new(1.0).expect("1 Pa should be valid");

    assert_approx_eq(pressure.value(), 1.0, 1.0e-12);
    assert_approx_eq(pressure.level_db_spl(), 93.979_400_086_720_37, 1.0e-9);
    assert_approx_eq(
        pressure_from_spl_db(0.0).expect("0 dB SPL should be valid"),
        REFERENCE_SOUND_PRESSURE_AIR_PA,
        1.0e-12,
    );
    assert_approx_eq(
        pressure_from_spl_db(94.0).expect("94 dB SPL should be valid"),
        1.002_374_467_254_545,
        1.0e-12,
    );

    for value in invalid_positive_inputs() {
        assert!(SoundPressurePa::new(value).is_none(), "value={value}");
    }

    assert!(pressure_from_spl_db(f64::NAN).is_none());
    assert!(pressure_from_spl_db(f64::INFINITY).is_none());
}

#[test]
fn intensity_helpers_round_trip_levels() {
    let intensity = SoundIntensityWPerM2::new(1.0).expect("1 W/m^2 should be valid");

    assert_approx_eq(intensity.value(), 1.0, 1.0e-12);
    assert_approx_eq(intensity.level_db_sil(), 120.0, 1.0e-12);
    assert_approx_eq(
        intensity_from_sil_db(0.0).expect("0 dB SIL should be valid"),
        REFERENCE_SOUND_INTENSITY_W_PER_M2,
        1.0e-24,
    );
    assert_approx_eq(
        intensity_from_sil_db(120.0).expect("120 dB SIL should be valid"),
        1.0,
        1.0e-12,
    );

    for value in invalid_positive_inputs() {
        assert!(SoundIntensityWPerM2::new(value).is_none(), "value={value}");
    }

    assert!(intensity_from_sil_db(f64::NAN).is_none());
    assert!(intensity_from_sil_db(f64::INFINITY).is_none());
}

#[test]
fn prelude_reexports_main_public_api() {
    use use_acoustics::prelude::*;

    let frequency = FrequencyHz::new(440.0).expect("frequency should be valid");
    let pressure = SoundPressurePa::new(1.0).expect("pressure should be valid");
    let intensity = SoundIntensityWPerM2::new(1.0).expect("intensity should be valid");

    assert_approx_eq(
        period_seconds(frequency.value()).expect("frequency should be valid"),
        1.0 / 440.0,
        1.0e-12,
    );
    assert_approx_eq(
        wavelength_meters(440.0, SPEED_OF_SOUND_AIR_20C_MPS).expect("inputs should be valid"),
        343.0 / 440.0,
        1.0e-12,
    );
    assert_approx_eq(
        sound_pressure_level_db(pressure.value()).expect("pressure should be valid"),
        pressure.level_db_spl(),
        1.0e-12,
    );
    assert_approx_eq(
        sound_intensity_level_db(intensity.value()).expect("intensity should be valid"),
        intensity.level_db_sil(),
        1.0e-12,
    );
}
