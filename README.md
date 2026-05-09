# use-acoustics

Acoustics primitives for audible ranges, tones, and sound-pressure helpers.

`use-acoustics` is the natural sound-focused sibling to `use-wave`. It begins with compact helpers
for classifying frequencies, working with audible ranges, and turning small acoustic calculations
into reusable Rust values.

The crate starts deliberately small: frequency bands, tone values, wavelength helpers, and basic
sound-pressure-level calculations.

## Workspace crates

| Crate           | Purpose                                         |
| --------------- | ----------------------------------------------- |
| `use-acoustics` | Audible-range helpers and small acoustic values |

## Installation

```toml
[dependencies]
use-acoustics = "0.1.0"
```

## Usage

```rust
use use_acoustics::{classify_frequency_hz, AcousticBand};

assert_eq!(classify_frequency_hz(440.0), Some(AcousticBand::Audible));
```

## Scope

- Audible-range helpers and tone primitives.
- Small sound-pressure and wavelength calculations.
- No room acoustics, filters, or DSP pipeline yet.
