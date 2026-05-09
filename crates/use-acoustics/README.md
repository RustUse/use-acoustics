# use-acoustics

Small acoustic helpers for frequency ranges, tones, and pressure levels.

`use-acoustics` provides audible-band classification, a small `Tone` value type, wavelength
helpers, and basic sound-pressure-level calculations for lightweight sound-domain code.

## What this crate provides

| Item                        | Purpose                                            |
| --------------------------- | -------------------------------------------------- |
| `AcousticBand`              | Infrasound, audible, and ultrasound classification |
| `AUDIBLE_MIN_HZ` / `MAX_HZ` | Conventional audible-range constants               |
| `classify_frequency_hz()`   | Frequency band classification                      |
| `wavelength_meters()`       | Acoustic wavelength helper                         |
| `sound_pressure_level_db()` | Pressure-ratio to decibel helper                   |
| `Tone`                      | Small frequency-plus-amplitude value type          |

## Installation

```toml
[dependencies]
use-acoustics = "0.1.0"
```

## Example

```rust
use use_acoustics::{is_audible_frequency_hz, sound_pressure_level_db, Tone};

let tone = Tone::new(440.0, 0.6);

assert!(is_audible_frequency_hz(tone.frequency_hz));
assert_eq!(sound_pressure_level_db(0.2, 0.000_02), Some(80.0));
```

## Scope

- Coarse acoustic classification and helpers.
- Small immutable values.
- No filtering, synthesis, or room-modeling yet.
