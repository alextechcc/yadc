# yadc-codegen

Code generator for [yadc](../yadc/) unit systems.

```
yadc-codegen <INPUT.toml> [-o <output-dir>]
cargo run -p yadc-codegen -- yadc-codegen/si.toml -o <output-dir>   # from the workspace root
```

yadc gives you traits and derives to build *your own* unit system; this tool
writes the boilerplate of one from a TOML description. The output is ordinary
Rust - every type and every `#[derive]` is visible in the generated files - so
you can also use it once as a starting point and maintain the result by hand.
[`yadc-si`](../yadc-si/) is produced from [`si.toml`](si.toml) in this directory.

The TOML file is always the single input. It drives two generation modes:

| Mode | Trigger | Use when |
|------|---------|----------|
| **Manual** | `[[family]]` sections present | full control over quantity names, grouping, and unit set |
| **UCUM** | `[ucum]` table present | you want a complete unit library from the authoritative UCUM source |

Both modes emit a ready-to-compile crate (`Cargo.toml` + `src/`) that depends
only on `yadc` and `dynamickit` (and optionally `serde`). The output directory
defaults to `crate_name` from `[system]`. The `src/*.rs` files are laid out by
`rustfmt` (if it is on the `PATH`), so the same input always gives the same,
fmt-clean files.

## What gets generated

- `src/lib.rs` -
  - the label enum `Kind` (`#[derive(Dimension)]`, the `[kind] default`
    variant marked `#[default]`, the `[kind] affine` and `log` variants
    `#[kind(affine)]` and `#[kind(log)]`, each variant's `rules` in its
    `#[kind(..)]`);
  - the dimension struct (`#[derive(Dimension)]`: one `i8` per `[dim]` field
    and a `#[kind]` field);
  - the static quantity `Qty<const D: Dim, N = f64>(N)` (`#[derive(Quantity)]`:
    its operators, `Quantity` - every constant a unit - and the
    extras, `Display` and `Fn` among them, so a constant is called to build
    and to read, `KILOMETRE(3.0)`; the field is private to the crate, so the
    base constants are the only raw numbers);
  - the scale type, `pub struct Scale<P, S>(pub P, pub S);`, with its
    `impl Unit` written out (`n` is `zero + n · step`): `Scale(zero, step)` is
    a unit of its zero's type;
  - the dynamic quantity, `#[derive(dynamickit::Dynamic)]` on a struct
    `DynQty { dim: Dim, value: f64 }`, with no hand-written impl:

    ```rust
    #[dynamic(Qty<const D: Dim> => { dim: D, value: .0 })]
    #[dynamic(shared f64 => { dim: Dim::DIMENSIONLESS, value: self })]
    #[dynamic(impl(
        Mul -> Option<Self>,
        Div -> Option<Self>,
        Add -> Option<Self>,
        Sub -> Option<Self>,
        Mul<f64> -> Option<Self>,
        Div<f64> -> Option<Self>,
        PartialOrd,
        yadc::PartialRoot::partial_root(self, n: i8) -> Option<Self>,
        yadc::PartialPowi::partial_powi(self, n: i8) -> Option<Self>,
    ))]
    ```

    (converting moves the number and records `D`; `TryFrom` checks it on the
    way back; a bare `f64` is the same value as a dimensionless quantity;
    every operator and method is the fields' own, its output stated - `Mul`
    and `Div` are `-> Option<Self>`, since a dimension with a kind has
    partial products), with the dimension's own `* f64` and
    `/ f64` beside it (a dimension scales where it adds to itself as itself);
  - `pub mod base`, the `[[base]]` constants - the only raw numbers;
  - one `pub mod` per family, per `extra_modules` entry and for
    `dimensionless`.
- `src/dimensionless.rs` - `pub type Dimensionless = Qty<{ Dim::DIMENSIONLESS }>;`,
  its constants and unit enum.
- `src/<family>.rs` - the quantity aliases, a `consts` module with the
  constants (see [below](#the-consts-modules)), then each quantity's unit
  enum, naming them:

```rust
use crate::{Dim, Qty};
use consts::*;
use yadc::Dimension;

pub type Length = Qty<{ Dim { m: 1, ..Dim::DIMENSIONLESS } }>;

pub mod consts {
    use super::*;

    /// The metre, the SI base unit of length.
    pub use crate::base::METRE;

    /// One `Kilometre` (km), as a `Length`.
    pub const KILOMETRE: Length = 1000.0 * METRE;
}

#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone)]
#[unit(of = Length)]
pub enum LengthUnit {
    #[unit("km", "kilometres", by = KILOMETRE)]
    Kilometre,
    #[default]
    #[unit("m", "metres", by = METRE)]
    Metre,
}
```

- `Cargo.toml` - a starting manifest: one feature per family (all on by
  default), `std`, `libm` and `serde`. It knows nothing about the
  `extra_modules`' needs; `yadc-si`'s own manifest is maintained by hand, so
  regenerate into another directory and copy `src/*.rs` (see
  [`yadc-si`](../yadc-si/README.md#regenerating)).

The generated crate root needs the nine features a crate *defining* a system
needs (see yadc's README), plus `const_destruct` for the generic number type
and the dynamic type, and `fn_traits` and `unboxed_closures` for the `Fn`
extra of the derives - the std function traits that make a unit callable. A
crate that only uses the generated one, calling its units included, needs
none of the last three.

### The `consts` modules

The base constants are the only raw numbers; every other constant is a
quantity-typed expression of them, written the way a standards document
writes it:

- **a coherent unit** (the first unit of a quantity with `m = "1.0"`) is the
  product of the base constants for its exponents, in `[dim]` order -
  `NEWTON = METRE * KILOGRAM / (SECOND * SECOND)`, `HERTZ = ONE / SECOND`;
  where a base constant *is* the coherent unit (`METRE`, or `BEL` for a
  quantity with a kind), it is re-exported;
- **a multiple** is `m * COHERENT` - `KILOMETRE = 1000.0 * METRE`,
  `DEGREE_FAHRENHEIT = (5.0 / 9.0) * KELVIN` - or `m * (product)` for a
  quantity with no coherent unit of its own;
- **a quantity's `consts`** are written verbatim: `ICE_POINT = ABSOLUTE_ZERO + 273.15 * KELVIN`;
  with a `ty`, one is a scale, a zero and a step - `CELSIUS:
  Scale<ThermodynamicTemperature, TemperatureInterval> = Scale(ICE_POINT,
  KELVIN)`.

Each family's `consts` module also re-exports the base constants of its
quantities (`mechanical::consts::METRE`, `thermal::consts::ABSOLUTE_ZERO`),
so a family's constants are all in one place. A constant's name is its unit's
variant in SCREAMING_SNAKE_CASE (or `c`); a name two quantities of one module
share is prefixed with the quantity (`TIME_MINUTE`, `ANGLE_MINUTE`); a base
constant a module constant shadows (the angular `SECOND` of arc) is named by
its path, `crate::base::SECOND`.

### How units map to unit enums

Every quantity with units gets a `<Q>Unit` enum, `#[derive(yadc::Unit,
dynamickit::Variants)]` plus `unit_derives`, and `#[derive_const(Default)]` with the
coherent unit as `#[default]` (what a menu starts on; `Variants` lists the units for one).
Each variant names the unit value it is:

| Unit | Variant | Also |
|---|---|---|
| a multiple (`m`) | `#[unit("km", "kilometres", by = KILOMETRE)]` | its constant |
| a unit value (`by`) | `#[unit("°C", "degrees Celsius", by = CELSIUS)]` | nothing: `by` is written verbatim |
| a unit value of another type (`by`, `via`) | `#[unit("dBm", "decibels-milliwatt", by = DBM, via = PowerLevel)]` | nothing |

A `by` names any unit value in scope: another constant (`by = "KELVIN"`, the
degree Celsius of a temperature step), a scale from the quantity's `consts`
(`by = "CELSIUS"`), a scale in place (`by = "Scale(ABSOLUTE_ZERO, KELVIN)"`),
or a path into a hand-written module. A `via` names the type that unit value
measures when it is not the enum's quantity: the two convert with std `From`,
which a hand-written module supplies (a level's scale, in the watts' enum). A
unit whose conversion is not `const` (such a level: the `From` takes a
logarithm) says `non_const = true`, and its enum is `#[unit(of = Q,
non_const)]`.

The enums take the `Unit` derive's extras as they come - `Display`, `FromStr`
and `Fn` - so a unit enum's value prints, parses, and is called (`let u =
LengthUnit::Mile; u(3.0)`).

## TOML reference

`[system]`, `[kind]`, `[dim]`, `[[base]]` and `[dimensionless]` are shared
between both modes.

### `[system]`

```toml
[system]
crate_name   = "yadc-si"                    # output crate name (and default output dir)
family_order = ["mechanical", "angular"]    # order of features and `pub mod`s
dim_type     = "ISQ"                        # dimension struct name      (default "Dim")
qty_type     = "StaticSIQuantity"           # static quantity name       (default "Qty")
dynqty_type  = "DynamicSIQuantity"          # dynamic quantity name      (default "DynQty")
unit_derives = ["Copy", "Clone"]            # extra derives on every unit enum
extra_modules = [                           # hand-written modules: declared, never written
    { name = "level",      feature = "mechanical" },
    { name = "uom_bridge", feature = "uom" },
]
```

`unit_derives` should include `Copy` and `Clone`: the `Unit` derive does not
write them, and a menu copies its units.

`extra_modules` lists files you maintain in the crate's `src/` yourself. The
generated `lib.rs` declares each as `pub mod` (behind `feature` when given), but
the generator never writes or overwrites them, so regenerating preserves them.

### `[kind]`

The variants of the enum `Kind`: how a quantity behaves under `+` and `-`,
beside its exponents.

```toml
[kind]
variants = ["Linear", "Affine", "Log"]
default  = "Linear"     # an ordinary amount: `#[default]`
affine   = "Affine"     # points
log      = "Log"        # levels and gains
```

- **`default`** is the kind of an ordinary amount: it adds to its own kind,
  scales and multiplies.
- **`affine`** names the kind of a point, measured from a zero somebody chose
  (an altitude, an instant): two points differ by an ordinary amount and an
  amount moves a point; a point has no sum with another, no scaling and no
  products.
- **`log`** names the kind of a logarithm (a level, a gain): adding logarithms
  multiplies what they are the logarithms of, so `+` and `-` add and subtract
  the exponents. A gain (no exponents) scales and leaves a product as a label
  does; a level (with exponents) has no products.

Both are optional, and emitted on the variants they name: `#[kind(affine)]`,
`#[kind(log)]`. Naming either
makes the dimension's `*` and `/` partial - a product with a point or a level
has no result - so the generated dynamic type's `*` and `/` return `Option`
too.

A variant may also be a label of the system's own, with `rules` - its sums,
emitted verbatim as that variant's `#[kind(..)]`:

```toml
variants = ["Linear", "Affine", "Log", "Rank"]
rules    = { Rank = ["add(Rank) = invalid"] }
```

`si.toml` names both - its affine quantity is the `Instant`, its logarithmic
ones the `Gain` and the `PowerLevel` - and makes a temperature a label of its
own: an amount with a true zero, which scales and multiplies (`n * R * t`),
kept apart from its differences. `absolute` marks such a label, short for its
three rules: two differ by a plain amount (`sub(X) = Linear`), and a plain
amount moves one (`add(Linear) = X`, `sub(Linear) = X`).

```toml
[kind]
variants = ["Linear", "Absolute", "Affine", "Log"]
default  = "Linear"
affine   = "Affine"
log      = "Log"
absolute = ["Absolute"]
```

A rule is `add(X) = Y` or `sub(X) = Y`, where `X` is a variant and `Y` a
variant or `invalid` (no result; it revokes a default). Without rules a label
adds to and subtracts from itself, and survives `*` and `/` only beside a
plain number. There is no `Invalid` variant: a sum with no result is `None`.
The generator checks this before writing anything: a `default`, `affine`,
`log`, `absolute` or rule key that is not one of `variants`, an `absolute`
that is the default, affine or log variant, a `mul` / `div` rule
(products and quotients are not declarable), a rule that is not `op(X) = Y`,
or an operand that is not a variant.

### `[[base]]`

The base constants - the only raw numbers of the generated crate, in its
`base` module: one per axis (that axis at exponent 1, the default kind: the
products of coherent units are made of them), the number one (dimension one:
`HERTZ = ONE / SECOND`), any zero a quantity needs, and the unit of a
quantity with a kind (`BEL`, `kind = "Log"`).

```toml
[[base]]
name = "METRE"
dims = {m = 1}
doc  = "The metre, the SI base unit of length."

[[base]]
name  = "ABSOLUTE_ZERO"
dims  = {K = 1}
kind  = "Absolute"
value = "0.0"                 # default "1.0"
doc   = "Absolute zero, 0 K: the zero of thermodynamic temperature."
```

### `[dimensionless]`

Units of the always-present `Dimensionless` quantity, in the same format as a
quantity's `units` (below).

```toml
[dimensionless]
units = [
    {v = "Ratio",   m = "1.0",    s = "",   l = ""},
    {v = "Percent", m = "1.0e-2", s = "%",  l = "percent"},
]
```

---

## Manual mode

### `[[family]]` and `[[family.quantity]]`

```toml
[[family]]
name = "mechanical"           # a Cargo feature and a module

[[family.quantity]]
name = "Length"
dims = {m = 1}                # non-zero exponents; absent axes are 0
units = [
    {v = "Kilometre", m = "1000.0", s = "km", l = "kilometres"},
    {v = "Metre",     m = "1.0",    s = "m",  l = "metres"},
]
```

A quantity may also carry `kind = "Absolute"` (a `[kind]` variant) and `consts = [{ name,
value, doc }]` (constants defined by an expression, verbatim; with `ty`, a
constant of another type - a scale, `Scale<Zero, Step>`).

### Unit fields

| Key | Meaning |
|---|---|
| `v` | the enum variant (and, unless `c` says otherwise, the constant's name in SCREAMING_SNAKE_CASE) |
| `s`, `l` | the symbol and the long plural name: `Display` (`{}`, `{:#}`) and `FromStr` |
| `m` | the size in coherent units, a Rust expression (`"1000.0"`, `"5.0 / 9.0"`): a multiple, `m * COHERENT` |
| `c` | the constant's name, when not the variant's (`DEGREE_FAHRENHEIT`) |
| `by` | the unit value the unit is, verbatim, with no constant of its own: a constant, a scale, a path |
| `via` | the type the `by` value measures, when not this quantity: converted with std `From` both ways |
| `non_const` | `true` when the unit's conversion is not `const` (a `via` through a logarithm) |

A unit has `m` or `by`; any other key is an error.

### `kind` on a quantity

A quantity whose `kind` is not the default gets it in its alias
(`Qty<{ Dim { kind: Kind::Log, ..Dim::DIMENSIONLESS } }>`), and its coherent
unit is a `[[base]]` constant with the same `dims` and `kind` (`BEL`): a
product of base constants has the default kind, so it cannot be one. A plain
number becomes a gain by multiplying with it: `bels * BEL`. A quantity of the
affine kind has no "one of it": its units must be scales (`by`).

#### Angle: an axis, not a label

`yadc-si` counts radians on an exponent axis (`rad` in `[dim]`): angles
multiply like exponents (rad² is the steradian, torque is J/rad), so rad/s never
mixes with Hz, nor torque with energy. Where a textbook silently sets rad = 1
(s = r·θ, v = ω·r), the code divides by a radian explicitly. Whether angle is
an axis or not is the system's choice: drop `rad` from `[dim]` and an angle
is a plain ratio.

### Temperatures: scales, steps and zeros

A temperature has the `Absolute` label and is read on scales; a temperature
difference is a step, an ordinary amount. In `si.toml`:

```toml
[[family.quantity]]
name = "ThermodynamicTemperature"
dims = {K = 1}
kind = "Absolute"
consts = [
    {name = "ICE_POINT", value = "ABSOLUTE_ZERO + 273.15 * KELVIN", doc = "0 °C, the ice point of water: 273.15 K."},
    {name = "FAHRENHEIT_ZERO", value = "ICE_POINT - 32.0 * DEGREE_FAHRENHEIT", doc = "0 °F: 32 °F below the ice point."},
    {name = "CELSIUS", ty = "Scale<ThermodynamicTemperature, TemperatureInterval>", value = "Scale(ICE_POINT, KELVIN)", doc = "The Celsius scale (°C): kelvins from the ice point."},
    {name = "FAHRENHEIT", ty = "Scale<ThermodynamicTemperature, TemperatureInterval>", value = "Scale(FAHRENHEIT_ZERO, DEGREE_FAHRENHEIT)", doc = "The Fahrenheit scale (°F)."},
]
units = [
    {v = "Celsius",    s = "°C", l = "degrees Celsius",    by = "CELSIUS"},
    {v = "Fahrenheit", s = "°F", l = "degrees Fahrenheit", by = "FAHRENHEIT"},
]

[[family.quantity]]
name = "TemperatureInterval"
dims = {K = 1}
units = [
    {v = "Kelvin",     m = "1.0",                                s = "K",  l = "kelvin"},
    {v = "Celsius",                     by = "KELVIN",           s = "°C", l = "degrees Celsius"},
    {v = "Fahrenheit", m = "5.0 / 9.0", c = "DEGREE_FAHRENHEIT", s = "°F", l = "degrees Fahrenheit"},
    {v = "Rankine",                     by = "DEGREE_FAHRENHEIT", s = "°R", l = "degrees Rankine"},
]
```

which generates the steps `KELVIN` (re-exported from `base`) and
`DEGREE_FAHRENHEIT`, the zeros `ABSOLUTE_ZERO`, `ICE_POINT` and
`FAHRENHEIT_ZERO`, and the scales `CELSIUS` and `FAHRENHEIT`, each a
`Scale(zero, step)` - a unit of its zero's type, through the scale type's
`impl Unit`.

### Instants: points on a line

A point - the `affine` kind - has one base zero, a `[[base]]` constant with
its `dims`, `kind` and `value = "0.0"`: the point every other is counted from,
and what the quantity is stored, printed and serialized from. Every other zero
is a constant on the same line, and every scale a `Scale` of a zero and a
step. In `si.toml`:

```toml
[[base]]
name  = "UNIX_EPOCH"          # the base zero of the line of instants
dims  = {s = 1}
kind  = "Affine"
value = "0.0"

[[family.quantity]]
name = "Instant"
dims = {s = 1}
kind = "Affine"
consts = [
    {name = "GPS_EPOCH", value = "UNIX_EPOCH + 315964800.0 * SECOND", doc = ".."},
    {name = "UNIX_SECONDS", ty = "Scale<Instant, Time>", value = "Scale(UNIX_EPOCH, SECOND)", doc = ".."},
    {name = "GPS_SECONDS", ty = "Scale<Instant, Time>", value = "Scale(GPS_EPOCH, SECOND)", doc = ".."},
]
units = [
    {v = "UnixSecond", s = "s Unix", l = "seconds of Unix time", by = "UNIX_SECONDS"},
    {v = "GpsSecond",  s = "s GPS",  l = "seconds of GPS time",  by = "GPS_SECONDS"},
]
```

A point has no "one of it", so every unit of an affine quantity is a `by`.
Which zero an instant was read from is not in its type: an instant built from
GPS seconds reads in Unix seconds.

### Levels: gains and logarithmic lines

Decibels count on a logarithmic line, where multiplying is adding. The
generator computes no logarithm: a level is a quantity of the `log` kind, and
the one conversion is a hand-written module (`extra_modules`).

```toml
[[base]]
name = "BEL"                  # a gain of ten times: the unit of the line
kind = "Log"

[[base]]
name  = "WATT_LEVEL"          # where a power level's numbers start
dims  = {m = 2, kg = 1, s = -3}
kind  = "Log"
value = "0.0"

[[family.quantity]]
name = "Gain"                 # the logarithm of a plain ratio: plain units
kind = "Log"
units = [
    {v = "Bel",     m = "1.0", s = "B",  l = "bels"},
    {v = "Decibel", m = "0.1", s = "dB", l = "decibels"},
]

[[family.quantity]]
name = "PowerLevel"           # the logarithm of a power: read on scales
dims = {m = 2, kg = 1, s = -3}
kind = "Log"
consts = [
    {name = "MILLIWATT_LEVEL", value = "WATT_LEVEL - 30.0 * DECIBEL", doc = ".."},
    {name = "DBM", ty = "Scale<PowerLevel, Gain>", value = "Scale(MILLIWATT_LEVEL, DECIBEL)", doc = ".."},
]
units = [{v = "DecibelMilliwatt", s = "dBm", l = "decibels-milliwatt", by = "DBM"}]
```

A level carries the exponents of what it is the level of, and the `log` kind
subtracts them: two power levels differ by a gain (watts over watts), a gain
moves a level, and two levels do not sum to a level. No rule is written.
`yadc-si`'s hand-written `level.rs` holds std `From` between `PowerLevel` and
`Power` (and between `Gain` and a plain ratio). A scale of levels measures
levels; `PowerUnit` lists it beside the watts `via` the level, which is not
`const`:

```toml
{v = "DecibelMilliwatt", s = "dBm", l = "decibels-milliwatt", by = "DBM", via = "PowerLevel", non_const = true},
```

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
| `rad` (radian) | `rad` | a real dim in UCUM and in yadc-si's ISQ (dimensionless in SI proper) |
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
code       = "mol"
dims       = { mol = 1 }
multiplier = 1.0
```

#### Radians

`rad` is in `[dim].fields`, so it self-seeds as an axis and `sr` (defined in
UCUM as `rad2`) resolves to rad² by plain arithmetic - angular velocity and
frequency are type-distinct, as are torque and energy. To treat angles as
dimensionless instead, drop `"rad"` from `[dim].fields` and seed it with
`dims = {}`; an angle is then a plain ratio (a label cannot keep it apart
under `*`).

### What gets generated

Units are grouped by UCUM `class` into Cargo feature-gated families, and
within a family into one quantity per UCUM property, dimension and kind (a
property name shared by two dimensions gets a dimension suffix,
`UnclassifiedM3PerKgS2`). Only class `dimless` goes to `dimensionless.rs`;
a dimensionless unit of another class stays in its family:

| Family | UCUM classes | Contents |
|--------|-------------|----------|
| `si` | `si` | SI base and derived quantities with full metric-prefix expansion |
| `heat` | `heat` | Thermal energy units + `ThermodynamicTemperature` / `TemperatureInterval` |
| `iso1000` | `iso1000` | ISO 1000 units (litre, tonne, bar, …) |
| `intcust` | `intcust` | International customary units |
| `cgs` | `cgs` | Centimetre-gram-second units |
| `infotech` | `infotech` | Bit, byte and baud, with metric-prefixed variants |
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
  substituted with hardcoded scales (a zero and a step) in the `heat` family.
- `isArbitrary="yes"` units: titre, international units, etc. whose conversion factor
  is not defined.
- Units whose expression cannot be resolved from known symbols: none in
  UCUM 2.2; the generator prints how many it resolved and skipped.
