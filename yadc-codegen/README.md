# yadc-codegen

Code generator for [yadc](../yadc/) unit systems.

```
yadc-codegen <INPUT.toml> [-o <output-dir>]
```

The TOML file is always the single input.  It drives two generation modes:

| Mode | Trigger | Use when |
|------|---------|----------|
| **Manual** | `[[family]]` sections present | full control over quantity names, grouping, and unit set |
| **UCUM** | `[ucum]` table present | you want a complete unit library from the authoritative UCUM source |

Both modes emit a ready-to-compile Rust crate that depends only on `yadc`.
The output directory defaults to `crate_name` from `[system]`.

---

## TOML reference

All fields are shared between both modes.

### `[system]`

```toml
[system]
crate_name   = "my-units"
family_order = ["mechanical", "thermal"]  # controls feature and mod order
unit_derives = ["Debug", "Clone", "Hash"] # optional; appended to #[derive(Unit, ...)]
```

### `[kind]`

Variants for the `Kind` enum.  `Kind` lets the type system distinguish quantities
that share the same numeric dimension (angle vs dimensionless, thermodynamic
temperature vs temperature interval, etc.).

```toml
[kind]
variants = ["Linear", "Affine", "Angle", "SolidAngle", "Invalid"]
```

`Invalid` must be present — it is used as the `Kind::INVALID` sentinel.

### `[dim]`

Base dimension field names for the `Dim` struct.

```toml
[dim]
fields = ["m", "kg", "s", "A", "K", "mol", "cd"]
```

### `[dimensionless]`

Units for the always-present `Dimensionless` type alias.

```toml
[dimensionless]
units = [
    {v = "Ratio",   m = "1.0",    s = "",  l = ""},
    {v = "Percent", m = "1.0e-2", s = "%", l = "percent"},
]
```

---

## Manual mode

Define quantities directly with `[[family]]` / `[[family.quantity]]` sections.
`si.toml` in this directory is the canonical example — it is the source of the
`yadc-si` crate.

### `[[family]]` and `[[family.quantity]]`

Quantities are grouped into families.  Each family becomes a Cargo feature and a
`pub mod` in `lib.rs`.

```toml
[[family]]
name = "mechanical"

[[family.quantity]]
name = "Length"
dims = {m = 1}
units = [
    {v = "Meter",     m = "1.0",    s = "m",  l = "meters"},
    {v = "Kilometer", m = "1000.0", s = "km", l = "kilometers"},
]

[[family.quantity]]
name = "Velocity"
dims = {m = 1, s = -1}
units = [
    {v = "MeterPerSecond", m = "1.0", s = "m/s", l = "meters per second"},
]
```

### Unit fields

| Key | Required | Description |
|-----|----------|-------------|
| `v` | yes | PascalCase enum variant name |
| `m` | yes | Multiplier — a Rust expression pasted verbatim (`"1000.0"`, `"5.0 / 9.0"`). For a nonlinear unit (`scale` set) this is the SI **reference** instead. |
| `s` | yes | Short symbol string (may be `""`) |
| `l` | yes | Long plural name (may be `""`) |
| `o` | no  | Affine offset (see below) |
| `scale` | no | Nonlinear conversion function: `pow10` \| `exp` \| `pow2` \| `neglog10` \| `tan` (see below) |
| `coeff` | no | Coefficient for `scale` (Rust float expr, e.g. `"0.1"` for dB) |

### `kind` — distinguishing same-dimension quantities

```toml
[[family.quantity]]
name = "PlaneAngle"
dims = {}       # same numeric dim as dimensionless
kind = "Angle"
units = [
    {v = "Radian", m = "1.0",                 s = "rad", l = "radians"},
    {v = "Degree", m = "0.017453292519943295", s = "°",  l = "degrees"},
]
```

### `o` — affine quantities

An affine quantity (thermodynamic temperature) has a unit-specific zero point.
Mark it with `kind = "Affine"`. Units whose zero is offset from the base-unit
origin carry an `o` field; conversion is `canonical = (value + o) * m`, done in
scalar value-space, so no separate "delta" type is needed.

`o` satisfies `offset_in_base = multiplier × o`.  For Celsius: `1.0 × 273.15 = 273.15 K`.
For Fahrenheit: `(5/9) × 459.67 ≈ 255.37 K` (= 0 °F in Kelvin).

```toml
[[family.quantity]]
name = "ThermodynamicTemperature"
dims = {K = 1}
kind = "Affine"
units = [
    {v = "Kelvin",     m = "1.0",        s = "K",  l = "kelvin"},
    {v = "Celsius",    m = "1.0",        s = "°C", l = "degrees Celsius",    o = 273.15},
    {v = "Fahrenheit", m = "5.0 / 9.0", s = "°F", l = "degrees Fahrenheit", o = 459.67},
    {v = "Rankine",    m = "5.0 / 9.0", s = "°R", l = "degrees Rankine"},
]
```

### `scale` and `coeff` — nonlinear units

A nonlinear unit converts through a function instead of a multiply. Set `scale`
to the function and `coeff` to its parameter; the `m` field becomes the
**reference**. The stored value is always in base units (e.g. dBW stores watts),
so a nonlinear unit is just another unit of an ordinary quantity.

| `scale` | conversion (canonical from `value`) | use |
|---------|-------------------------------------|-----|
| `pow10` | `m * 10^(coeff·value)` | decibel / bel |
| `exp` | `m * e^(coeff·value)` | neper |
| `pow2` | `m * 2^(coeff·value)` | bit / log2 |
| `neglog10` | `m * 10^(−coeff·value)` | pH |
| `tan` | `m * atan(value / coeff)` | prism diopter, %slope |

```toml
# Decibels as units of the dimensionless Ratio (m = reference = 1.0):
[dimensionless]
units = [
    {v = "Ratio",   m = "1.0", s = "",   l = ""},
    {v = "Decibel", m = "1.0", s = "dB", l = "decibels", scale = "pow10", coeff = "0.1"},
]

# dBW / dBm as units of Power (m = SI reference: 1 W and 1 mW):
[[family.quantity]]
name = "Power"
dims = {kg = 1, m = 2, s = -3}
units = [
    {v = "Watt",             m = "1.0",   s = "W",   l = "watts"},
    {v = "DecibelWatt",      m = "1.0",   s = "dBW", l = "decibels-watt",      scale = "pow10", coeff = "0.1"},
    {v = "DecibelMilliwatt", m = "0.001", s = "dBm", l = "decibels-milliwatt", scale = "pow10", coeff = "0.1"},
]
```

Nonlinear conversions use `num-traits` (`Real`), pulled in transitively through
`yadc`; in `no_std` builds enable its `libm` feature.

---

## UCUM mode

Parses `ucum-essence.xml` from the [Unified Code for Units of Measure](https://ucum.org)
project and generates a complete yadc unit crate.  Add a `[ucum]` table to your
TOML to enable this mode — `ucum.toml` in this directory is the canonical example.

### `[ucum]`

```toml
[ucum]
# "latest"  → query GitHub releases API and fetch that tag's ucum-essence.xml
# URL       → fetch the file at this address directly
# file path → read a local copy (pinned version, offline use)
source = "latest"
```

### `[[ucum.base]]` — dimension basis remapping

UCUM uses a different set of base dimensions from ISQ/SI:

| UCUM base | ISQ base | Key difference |
|-----------|----------|----------------|
| `g` (gram) | `kg` | scale ×10⁻³ |
| `C` (coulomb) | `A` (ampere) | charge vs current; `C = A·s` |
| `rad` (radian) | *(no ISQ equiv)* | angle is a real UCUM dim, dimensionless in ISQ |
| *(none)* | `mol` | mol is Avogadro's count in UCUM, a base in ISQ |
| `m, s, K, cd` | `m, s, K, cd` | identical in both systems |

Each `[[ucum.base]]` entry overrides how one UCUM base code is seeded in the
resolver.  Omitted codes are self-seeded (code → its own dim field, ×1.0).

```toml
[[ucum.base]]
code       = "g"
dims       = { kg = 1 }
multiplier = 1e-3

[[ucum.base]]
code       = "C"
dims       = { A = 1, s = 1 }
multiplier = 1.0

[[ucum.base]]
code       = "rad"
dims       = {}       # dimensionless in ISQ
kind       = "Angle"
multiplier = 1.0

[[ucum.base]]
code       = "mol"
dims       = { mol = 1 }
multiplier = 1.0

[[ucum.base]]
code       = "sr"
dims       = {}       # dimensionless in ISQ
kind       = "SolidAngle"
multiplier = 1.0
```

#### Using UCUM-native dimensions

To keep `rad` as a real dimension field (angle is not dimensionless, angular
velocity and frequency are type-distinct without a `Kind` trick):

1. Add `"rad"` to `[dim].fields`
2. Remove the `rad` and `sr` entries from `[[ucum.base]]` (self-seeding handles them)
3. Remove `"Angle"` and `"SolidAngle"` from `[kind].variants` if you no longer need them

The kind algebra (`Angle × Angle = SolidAngle`) only applies when `rad` is seeded
as dimensionless with `kind = "Angle"`.

### What gets generated

Units are grouped by UCUM `class` into Cargo feature-gated families:

| Family | UCUM classes | Contents |
|--------|-------------|----------|
| `si` | `si` | SI base and derived quantities with full metric-prefix expansion |
| `heat` | `heat` | Thermal energy units + `ThermodynamicTemperature` / `TemperatureInterval` |
| `iso1000` | `iso1000` | ISO 1000 units (litre, tonne, bar, …) |
| `intcust` | `intcust` | International customary units |
| `cgs` | `cgs` | Centimetre-gram-second units |
| `infotech` | `infotech` | Bit, byte, and metric-prefixed variants |
| `us` | `us-lengths`, `us-volumes`, `avoirdupois`, `troy`, `apoth` | US customary and apothecary units |
| `british` | `brit-length`, `brit-volumes` | British imperial units |
| `clinical` | `clinical`, `chemical` | Clinical and chemical units |
| `constants` | `const` | Physical constants used as unit scales |
| `misc` | `misc` | Remaining units |

For every `isMetric="yes"` unit the generator also emits metric-prefixed variants
(tera- through atto-), skipping any the UCUM file already defines explicitly.

### What is skipped

- `isSpecial="yes"` units: logarithmic, non-linear, function-based conversions (pH,
  Neper, Bel, dB variants, …).  Celsius and Fahrenheit are the exception — they are
  substituted with hardcoded affine offsets in the `heat` family.
- `isArbitrary="yes"` units: titre, international units, etc. whose conversion factor
  is not defined.
- Units whose expression cannot be fully resolved from known symbols (~79 units,
  mostly those referencing `[pi]` — which is `isSpecial` — or chaining through
  `[gr]` (grain), which is absent from `ucum-essence.xml`).  This takes out most
  US customary, British imperial, troy, and apothecary units.
