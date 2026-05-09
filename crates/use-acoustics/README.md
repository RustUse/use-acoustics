# use-acoustics

Composable primitive acoustics utilities for Rust.

`use-acoustics` provides a small, self-contained v0.1 API for common sound-domain calculations:
frequency and period helpers, wavelength conversions, decibel conversions, sound pressure level,
sound intensity level, and simple speed of sound approximations.

## Relationship To RustUse

`use-wave` is the broader wave utility set. `use-acoustics` is the sound-specific sibling that
focuses on reusable primitives and small formulas for acoustics-oriented applications.

## Warning

The formulas in this crate are general-purpose approximations and convenience conversions. They
are not a replacement for specialized acoustic engineering software, calibrated measurements, or
standards-driven analysis.

## Installation

Crates.io:

```toml
[dependencies]
use-acoustics = "0.1"
```

Local path:

```toml
[dependencies]
use-acoustics = { path = "crates/use-acoustics" }
```

## Examples

```rust
use use_acoustics::prelude::*;

let wavelength = wavelength_meters(440.0, SPEED_OF_SOUND_AIR_20C_MPS).unwrap();
assert!((wavelength - 0.7795).abs() < 0.001);

let spl = sound_pressure_level_db(1.0).unwrap();
assert!((spl - 93.9794).abs() < 0.001);
```

```rust
use use_acoustics::prelude::*;

let amplitude_ratio = db_to_amplitude_ratio(20.0).unwrap();
let power_ratio = db_to_power_ratio(10.0).unwrap();

assert!((amplitude_ratio - 10.0).abs() < 1.0e-12);
assert!((power_ratio - 10.0).abs() < 1.0e-12);
```

## Module Overview

| Module       | Purpose                                                  |
| ------------ | -------------------------------------------------------- |
| `decibel`    | Decibel ratios plus SPL and SIL conversions              |
| `frequency`  | Frequency newtype, period, and angular frequency helpers |
| `wavelength` | Frequency and wavelength conversion helpers              |
| `pressure`   | Sound pressure newtype and SPL inverse conversion        |
| `intensity`  | Sound intensity newtype and SIL inverse conversion       |
| `speed`      | Speed of sound helpers                                   |
| `prelude`    | Re-exports for the main public API                       |

## Scope

- decibels and ratios
- sound pressure level and sound intensity level
- frequency, period, and angular frequency
- wavelength conversions
- speed of sound approximations

## Status

Early v0.1 API.
