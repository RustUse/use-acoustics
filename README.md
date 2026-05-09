# use-acoustics

Composable primitive acoustics utilities for Rust.

`use-acoustics` is the sound-specific RustUse set for small, composable acoustic calculations.
It focuses on frequency, wavelength, decibels, sound pressure, sound intensity, and simple speed
of sound approximations without pulling in DSP engines or heavyweight audio dependencies.

## Scope

- decibels
- sound pressure level
- sound intensity level
- frequency
- period
- angular frequency
- wavelength
- speed of sound approximations

## Relationship

- `use-wave` handles broader wave primitives.
- `use-acoustics` handles sound-specific primitives.
- The v0.1 API is intentionally small and mostly self-contained.

## Workspace crates

| Crate           | Purpose                                            |
| --------------- | -------------------------------------------------- |
| `use-acoustics` | Composable primitive acoustics utilities for Rust. |

## Installation

Crates.io:

```toml
[dependencies]
use-acoustics = "0.1"
```

Local workspace or path dependency:

```toml
[dependencies]
use-acoustics = { path = "crates/use-acoustics" }
```

## Examples

Calculate wavelength from frequency:

```rust
use use_acoustics::prelude::*;

let wavelength = wavelength_meters(440.0, SPEED_OF_SOUND_AIR_20C_MPS).unwrap();

assert!((wavelength - 0.7795).abs() < 0.001);
```

Calculate SPL from pressure:

```rust
use use_acoustics::prelude::*;

let spl = sound_pressure_level_db(1.0).unwrap();

assert!((spl - 93.9794).abs() < 0.001);
```

Convert dB to ratios:

```rust
use use_acoustics::prelude::*;

let amplitude_ratio = db_to_amplitude_ratio(20.0).unwrap();
let power_ratio = db_to_power_ratio(10.0).unwrap();

assert!((amplitude_ratio - 10.0).abs() < 1.0e-12);
assert!((power_ratio - 10.0).abs() < 1.0e-12);
```

## Status

Early v0.1 API.
