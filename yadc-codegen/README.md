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
  - the static quantity `Qty<const D: Dim, N>(N)` (`#[derive(Quantity)]`:
    its operators, `Quantity` - every quantity value a unit of its own kind -
    and the extras, `Display` among them; the field is private to the crate);
  - the unit: `QtyUnitConsts { size: Exact, zero: Exact, symbol, name }`, a
    unit's definition, and `QtyUnit<const D: Dim, const C: QtyUnitConsts>`,
    a unit struct, with its one `impl Unit` written out - `n` in a unit is
    `n · size + zero` coherent units, in any `N: const From<Exact>` with `+ - * /`,
    the size and zero converted in `const { }` blocks - and its `Display`
    (`{}` the symbol, `{:#}` the name). That impl is the only place a number
    becomes a quantity;
  - one `pub mod` per family, per `extra_modules` entry and for
    `dimensionless` and `dynamic`.
- `src/dynamic/mod.rs` - the dynamic half: what is chosen at run time, a
  quantity's dimension or a unit. It declares one module of unit enums per
  family (`dynamic::mechanical`, behind the family's feature, and
  `dynamic::dimensionless`), and holds the dynamic quantity,
  `#[derive(dynamickit::Dynamic)]` on a struct
  `DynQty { dim: Dim, value: f64 }`, with no hand-written impl:

  ```rust
  #[dynamic(Qty<const D: Dim, f64> => { dim: D, value: .0 })]
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
  and the dynamic unit, `DynamicQtyUnit { dim: Dim, consts: QtyUnitConsts }`,
  `#[dynamic(QtyUnit<const D: Dim, const C: QtyUnitConsts> => { dim: D, consts: C })]`,
  a unit of `Option<DynQty>` counted in `Option<f64>` (`value` is `None`
  for another dimension).
- `src/dimensionless.rs` - `pub type Dimensionless<N> = Qty<{ Dim::DIMENSIONLESS }, N>;`
  and its `units`; `src/dynamic/dimensionless.rs` - its unit enum.
- `src/<family>.rs` - each quantity's dimension constant and alias, and a
  `units` module with one type per unit:

```rust
use crate::{Dim, Qty};
use yadc::Dimension;

/// The dimension of `Length`.
pub const LENGTH: Dim = Dim { m: 1, ..Dim::DIMENSIONLESS };
pub type Length<N> = Qty<LENGTH, N>;

pub mod units {
    use super::*;
    use crate::{Exact, QtyUnit, QtyUnitConsts};

    /// Inches (`in`), a unit of `Length`: `n` of them is `n · 0.0254` coherent units.
    pub type Inch = QtyUnit<
        LENGTH,
        { QtyUnitConsts { size: Exact::dec(254, -4), zero: Exact::ZERO, symbol: "in", name: "inches" } },
    >;
}
```

- `src/dynamic/<family>.rs` - each quantity's unit enum, each variant
  naming its unit type, and `From` from it to the dynamic unit
  (`dynamic::mechanical::LengthUnit`):

```rust
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone)]
#[unit(of = Length<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum LengthUnit {
    #[default]
    #[unit(Metre)]
    Metre,
    #[unit(Inch)]
    Inch,
}

impl From<LengthUnit> for DynamicQtyUnit {
    fn from(unit: LengthUnit) -> Self {
        match unit {
            LengthUnit::Metre => Metre {}.into(),
            LengthUnit::Inch => Inch {}.into(),
        }
    }
}
```

  The enum derives `yadc::Unit` and `dynamickit::Variants` plus
  `unit_derives`, and `Default` with the coherent unit (the first) as
  `#[default]` - what a menu starts on; `Variants` lists the units for one.
  Its bounds are exactly what `QtyUnit`'s `impl Unit` needs of `N`, and its
  `Display` and `FromStr` are its unit types' (`LengthUnit::Inch.quantity(3.0_f64)`,
  `"in".parse::<LengthUnit>()`).

- `src/prefix.rs` (written from `templates/prefix.rs`, beside `src/exact.rs`) -
  the 20 SI prefixes as the enum `Prefix` (`Prefix::Kilo`, an exact
  `factor()`), and `Prefixed<U>`, a unit with an optional `Prefix` - a unit
  of what `U` measures - which parses a leading prefix on a unit's own
  symbol: `km` as the kilo of `LengthUnit::Metre`. A prefixed unit has no
  type of its own (see [below](#prefixed-units)).

- `Cargo.toml` - a starting manifest: one feature per family (all on by
  default), `std`, `libm` and `serde`. It knows nothing about the
  `extra_modules`' needs; `yadc-si`'s own manifest is maintained by hand, so
  regenerate into another directory and copy `src/*.rs` and
  `src/dynamic/*.rs` (see
  [`yadc-si`](../yadc-si/README.md#regenerating)).

`QtyUnit`, `QtyUnitConsts` and `DynamicQtyUnit` are named by `unit_type`
(`yadc-si`: `SIUnit`, `SIUnitConsts`, `DynamicSIUnit`). The generated crate
root needs the features a crate *defining* a system needs (see yadc's
README). A crate that only uses the generated one needs `generic_const_exprs`,
plus `const_trait_impl` to build quantities in `const` items.

`src/exact.rs` (written from `templates/exact.rs`) holds `Exact`, the exact
number a unit's size and zero are written in, with std `From` to `f64`,
`f32` (the nearest, rounded once) and every primitive integer (the nearest,
ties to even; a panic where it does not fit - in a `const`, a compile error).

### Unit sizes and zeros

A unit's `m` and `z` are its exact definition in coherent units - `"0.0254"`,
`"1000.0 / 3600.0"`, `"core::f64::consts::PI / 180.0"` (numbers, `+ - * /`,
parentheses and π) - never a rounded result. The generator writes their
literals as `Exact`s (`0.0254` is `Exact::dec(254, -4)`,
`core::f64::consts::PI` is `Exact::PI`) and the expression is evaluated
exactly, at compile time.

A unit's type is named by its `t`, or else its variant; where two
quantities of one module share a variant name, the type is prefixed with its
quantity (`TemperatureIntervalKelvin`) unless a `t` names it.

### Prefixed units

The input lists no unit whose symbol is a prefix on another unit's:
`Prefixed<LengthUnit>` parses `km` and `km²`. The generator emits exactly the
units its input lists; `si.toml` lists ~860: the units with no leading SI
prefix, those that keep one in their name - a coherent unit (`Kilogram`), a
unit whose prefix scales no unit of its quantity (`MillimetreOfMercury`) -
and those with a prefix on one factor of the symbol (`KilometrePerHour`,
km/h).

## TOML reference

`[system]`, `[kind]`, `[dim]` and `[dimensionless]` are shared
between both modes.

### `[system]`

```toml
[system]
crate_name   = "yadc-si"                    # output crate name (and default output dir)
family_order = ["mechanical", "angular"]    # order of features and `pub mod`s
dim_type     = "ISQ"                        # dimension struct name      (default "Dim")
qty_type     = "SIQuantity"           # static quantity name       (default "Qty")
dynqty_type  = "DynamicSIQuantity"          # dynamic quantity name      (default "DynQty")
unit_type    = "SIUnit"                     # unit type name (default "QtyUnit"); its definition
                                            # is `SIUnitConsts`, its dynamic version `DynamicSIUnit`
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
    {v = "Metre", m = "1.0",    s = "m",  l = "metres"},
    {v = "Inch",  m = "0.0254", s = "in", l = "inches"},
]
```

A quantity may also carry `kind = "Absolute"` (a `[kind]` variant).

### Unit fields

| Key | Meaning |
|---|---|
| `v` | the enum variant, and the unit type's name unless `t` gives one |
| `m` | the size in coherent units, an exact expression (`"0.0254"`, `"5.0 / 9.0"`) |
| `z` | the zero in coherent units, the same way (`"273.15"`); `0` when absent |
| `s`, `l` | the symbol and the long plural name: `Display` (`{}`, `{:#}`) and `FromStr` |
| `t` | the unit type's name, when not the variant's (`t = "KelvinScale"`) |

A number `n` in a unit is `n · m + z` coherent units. Any other key is an
error.

### `kind` on a quantity

A quantity whose `kind` is not the default gets it in its dimension constant
(`Dim { kind: Kind::Log, ..Dim::DIMENSIONLESS }`), and its units are units of
that kind: a plain number becomes a gain through a unit, `Bel {}.quantity(n)`.
A quantity of the affine kind has no "one of it": every unit of it says, by
its zero, which point its numbers count from.

#### Angle: an axis, not a label

`yadc-si` counts radians on an exponent axis (`rad` in `[dim]`): angles
multiply like exponents (rad² is the steradian, torque is J/rad), so rad/s never
mixes with Hz, nor torque with energy. Where a textbook silently sets rad = 1
(s = r·θ, v = ω·r), the code divides by a radian explicitly. Whether angle is
an axis or not is the system's choice: drop `rad` from `[dim]` and an angle
is a plain ratio.

### Temperatures: units with a zero

A temperature has the `Absolute` label and is read in units with a zero; a
temperature difference is a step, an ordinary amount, read in units without
one. In `si.toml`:

```toml
[[family.quantity]]
name = "ThermodynamicTemperature"
dims = {K = 1}
kind = "Absolute"
units = [
    {v = "Kelvin", m = "1.0", s = "K", l = "kelvin", t = "KelvinScale"},
    {v = "Celsius", m = "1.0", z = "273.15", s = "°C", l = "degrees Celsius"},
    {v = "Fahrenheit", m = "5.0 / 9.0", z = "273.15 - 32.0 * 5.0 / 9.0", s = "°F", l = "degrees Fahrenheit"},
    {v = "Rankine", m = "5.0 / 9.0", s = "°R", l = "degrees Rankine"},
]

[[family.quantity]]
name = "TemperatureInterval"
dims = {K = 1}
units = [
    {v = "Kelvin", m = "1.0", s = "K", l = "kelvin"},
    {v = "Celsius", m = "1.0", s = "°C", l = "degrees Celsius", t = "DegreeCelsius"},
    {v = "Fahrenheit", m = "5.0 / 9.0", s = "°F", l = "degrees Fahrenheit", t = "DegreeFahrenheit"},
    {v = "Rankine", m = "5.0 / 9.0", s = "°R", l = "degrees Rankine", t = "DegreeRankine"},
]
```

which generates the temperature units `KelvinScale`, `Celsius`, `Fahrenheit`
and `Rankine`, and the step units `Kelvin`, `DegreeCelsius`,
`DegreeFahrenheit` and `DegreeRankine`; the enums keep the variant names
(`ThermodynamicTemperatureUnit::Celsius`, `TemperatureIntervalUnit::Celsius`).

### Instants: points on a line

A point - the `affine` kind - is stored from the zero of its coherent unit:
what it prints and serializes as. Every unit is a size and a zero on that one
line. In `si.toml`:

```toml
[[family.quantity]]
name = "Instant"
dims = {s = 1}
kind = "Affine"
units = [
    {v = "UnixSecond", m = "1.0", s = "s Unix", l = "seconds of Unix time"},
    {v = "GpsSecond", m = "1.0", z = "315964800.0", s = "s GPS", l = "seconds of GPS time"},
    {v = "JulianDate", m = "86400.0", z = "-2440587.5 * 86400.0", s = "JD", l = "Julian date"},
]
```

Which zero an instant was read from is not in its type: an instant built
from GPS seconds reads in Unix seconds.

### Levels: gains and logarithmic lines

Decibels count on a logarithmic line, where multiplying is adding. The
generator computes no logarithm: a level is a quantity of the `log` kind,
counted in bels, and the one conversion is a hand-written module
(`extra_modules`).

```toml
[[family.quantity]]
name = "Gain"                 # the logarithm of a plain ratio: plain units
kind = "Log"
units = [
    {v = "Bel",     m = "1.0", s = "B",  l = "bels"},
    {v = "Decibel", m = "0.1", s = "dB", l = "decibels"},
]

[[family.quantity]]
name = "PowerLevel"           # the logarithm of a power, in bels from one watt
dims = {m = 2, kg = 1, s = -3}
kind = "Log"
units = [
    {v = "DecibelWatt",      m = "0.1", s = "dBW", l = "decibels-watt"},
    {v = "DecibelMilliwatt", m = "0.1", z = "-3.0", s = "dBm", l = "decibels-milliwatt"},
]
```

A level carries the exponents of what it is the level of, and the `log` kind
subtracts them: two power levels differ by a gain (watts over watts), a gain
moves a level, and two levels do not sum to a level. No rule is written.
`yadc-si`'s hand-written `level.rs` holds std `From` between `PowerLevel` and
`Power` (and between `Gain` and a plain ratio).

## UCUM mode

Parses `ucum-essence.xml` from the [Unified Code for Units of Measure](https://ucum.org)
project and generates a complete yadc unit crate, as UCUM defines it. Add a
`[ucum]` table to your TOML to enable this mode — `ucum.toml` in this
directory is the canonical example, and generates the workspace's
`yadc-ucum` crate:

```bash
cargo run -p yadc-codegen -- yadc-codegen/ucum.toml -o yadc-ucum
```

### `[ucum]`

```toml
[ucum]
# A release's ucum-essence.xml, pinned by its tag: the same source gives the
# same crate. "latest" asks GitHub for the newest release; a file path reads
# a local copy.
source = "https://raw.githubusercontent.com/ucum-org/ucum/v2.2/ucum-essence.xml"
```

Updating to a new UCUM release is changing the tag and regenerating.

### UCUM's base units are the axes

Each of UCUM's seven base units is an axis of the dimension, named by its
code: `m`, `s`, `g`, `rad`, `K`, `C` (the coulomb: charge, not current),
`cd`. `[dim].fields` lists them. The mole is no axis: UCUM defines it as a
number, 6.02214076 × 10²³, so `AmountOfSubstance` is dimensionless. The
coherent units are UCUM's base units and their products: the newton is a
size of 1000 (g·m/s²), the ampere a coulomb per second.

### Exact values, UCUM codes

Every value in the XML is a decimal, and is read exactly - as a rational,
with π (`[pi]`, a long decimal in the XML) kept as a power of π - so each
unit's size is exact: the international inch is `Exact::dec(254, -4)`
metres, the US survey inch `Exact::int(100) / Exact::int(3937)`, the degree
π/180 radians.

A unit type prints, and a unit enum's variant prints and parses, as the unit's UCUM code,
case-sensitive (`[in_i]`, `Cel`, `[degF]`); its name (`{:#}`) is UCUM's
(`inch`).

### What gets generated

Units are grouped by UCUM `class` into Cargo feature-gated families, and
within a family into one quantity per UCUM property, dimension and kind (a
property name shared by two dimensions gets a dimension suffix,
`UnclassifiedM3PerKgS2`). Only class `dimless` goes to `dimensionless.rs`;
a dimensionless unit of another class stays in its family:

| Family | UCUM classes | Contents |
|--------|-------------|----------|
| `si` | `si` | SI base and derived quantities |
| `heat` | `heat` | Thermal energy units + `ThermodynamicTemperature` / `TemperatureInterval` |
| `iso1000` | `iso1000` | ISO 1000 units (litre, tonne, bar, …) |
| `intcust` | `intcust` | International customary units |
| `cgs` | `cgs` | Centimetre-gram-second units |
| `infotech` | `infotech` | Bit, byte and baud |
| `us` | `us-lengths`, `us-volumes`, `avoirdupois`, `troy`, `apoth` | US customary and apothecary units |
| `british` | `brit-length`, `brit-volumes` | British imperial units |
| `clinical` | `clinical`, `chemical` | Clinical and chemical units |
| `constants` | `const` | Physical constants as units |
| `misc` | `misc` | Remaining units |

A unit gets no prefixed types or variants: `Prefixed<..Unit>` covers them,
as in `yadc-si`. The units are `UcumUnit` types (`unit_type = "UcumUnit"`).

### What is skipped

- `isSpecial="yes"` units: logarithmic, non-linear, function-based conversions (pH,
  Neper, Bel, dB variants, …).  Celsius and Fahrenheit are the exception — they are
  substituted with hardcoded units with a zero (`Celsius`, `Fahrenheit`) in the `heat` family.
- `isArbitrary="yes"` units: titre, international units, etc. whose conversion factor
  is not defined.
- Units whose expression cannot be resolved from known symbols: none in
  UCUM 2.2; the generator prints how many it resolved and skipped.
