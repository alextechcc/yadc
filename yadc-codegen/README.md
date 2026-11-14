# yadc-codegen

Writes a [yadc](../) unit-system crate from a TOML description.

```
yadc-codegen <INPUT.toml> [-o <output-dir>]
cargo run -p yadc-codegen -- yadc-codegen/si.toml -o <output-dir>   # from the workspace root
```

The output is ordinary Rust: every type and every `#[derive]` is in the
generated files, so you can also use it once as a starting point and
maintain the result by hand. [`yadc-si`](../yadc-si/) is generated from
[`si.toml`](si.toml), [`yadc-ucum`](../yadc-ucum/) from
[`ucum.toml`](ucum.toml).

The TOML file drives one of two modes:

| Mode | Trigger | Use when |
|------|---------|----------|
| **Manual** | `[[family]]` sections present | you choose the quantities, their grouping and their units |
| **UCUM** | `[ucum]` table present | you want every unit of the UCUM source |

Both write a crate (`Cargo.toml` + `src/`) that depends on
`const_derive_more` and `num-traits` (and optionally `serde`). The output
directory defaults to `crate_name` from `[system]`. The `src/*.rs` files are
laid out by `rustfmt` (if it is on the `PATH`), so the same input always
gives the same files.

## What gets generated

The names below are `yadc-si`'s; `[system]` names the types.

- `src/unit.rs` (from [`templates/unit.rs`](templates/unit.rs)) - the unit
  type `SIUnit`: a product of named units to integer powers, a
  `ConstParamTy`. Base units define the dimensions (`SIUnit::base("m")`);
  every other named unit is a size times other units
  (`SIUnit::scaled("h", Exact::int(3600), SECOND.unit())`), and may count
  from a zero (`SIUnit::offset`) or on a log scale (`SIUnit::from_log`). A
  prefix (`SIUnit::prefix`) fuses with the one unprefixed named unit it
  multiplies. Its std `* / + -` are `const impl`s returning `Option`: `*`
  and `/` are `None` for a unit with a zero or a log scale, `+` and `-` are
  `Some` for one unit only. Also `times`, `same_dimension`,
  `to_base` / `from_base` (a number in base units and back), and `Display`
  (`km/h`, `kg·m^2/s^2`, `kPa`; width, fill and alignment pad the whole
  symbol).
- `src/exact.rs` (from [`templates/exact.rs`](templates/exact.rs)) -
  `Exact`, the exact number unit sizes are written in (`num/den · 10ⁿ ·
  πᵏ`), with `const` arithmetic and std `From` to `f64`, `f32` (the nearest,
  rounded once) and every primitive integer (the nearest, ties to even; a
  panic where it does not fit - in a `const`, a compile error).
- `src/lib.rs` -
  - `Base`, with the constants `BASE` (a base unit or a product of base
    units) and `NOT_BASE` (any other unit), and its std `* / + -`: a
    product is `BASE` where both factors are, a sum needs both the same;
  - the quantity, `SIQuantity<const U: SIUnit, N, const B: Base>(N)` with
    `#[derive(const_derive_more::ConstOps)]` lifting both `U` and `B`
    (mixed with `Exact` for every primitive number type), `Display` (`2
    km`), and serde as its number behind the `serde` feature;
  - two blanket `From` impls: `SIQuantity<U, N, NOT_BASE>` into
    `SIQuantity<V, N, BASE>` where `U` and `V` have one dimension, and back;
    the target type picks the unit;
  - per primitive number type, `From` a quantity of a plain number (of
    dimension one, without a zero or a log scale) into the number, scaled
    by its unit's size: `f64::from(d / KILOMETRE)`. An integer takes only a
    unit of a whole size (`km/m`, not `m/km`);
  - the 20 SI prefixes (`KILO`) and the base units (`METRE`), as constants;
  - the `named` coherent units' symbols, for `Display`;
  - `pub mod` for each family (behind its feature), `dimensionless`,
    `dynamic` and each `extra_modules` entry.
- `src/<family>.rs` and `src/dimensionless.rs` - per quantity, an alias
  whose unit defaults to its base unit (in the first module with a
  quantity of its name only, so glob imports of two modules don't clash),
  and a constant per unit, one of it. A unit whose symbol is an SI prefix's
  followed by another unit's of its quantity, of the prefix's size times
  that unit's, is their product:

  ```rust
  /// A length, in `N`s of the unit `U`: by default its coherent unit, a
  /// product of base units.
  pub type Length<N, const U: SIUnit = { METRE.unit() }, const B: Base = BASE> =
      SIQuantity<U, N, B>;

  /// Kilometres (`km`), a unit of `Length`: `KILO * METRE`.
  pub const KILOMETRE: SIQuantity<{ KILO.unit().times(METRE.unit(), 1) }, Exact, NOT_BASE> =
      KILO * METRE;

  /// Miles (`mi`), a unit of `Length`: `n` of them is `n · 1609.344` base units.
  pub const MILE: SIQuantity<
      { SIUnit::scaled("mi", Exact::dec(1609344, -3), METRE.unit()) },
      Exact,
      NOT_BASE,
  > = SIQuantity::ONE;
  ```

- `src/dynamic/mod.rs` - the dynamic quantity, `DynamicSIQuantity { unit:
  SIUnit, is_base: Base, value: f64 }` with `#[derive(const_derive_more::Dynamic)]`:
  `From` any `SIQuantity<U, f64, B>`, `TryFrom` back (checked), and
  `* / + -`, scaling by an `f64` and `PartialOrd`, each returning `Option`
  and combining `unit` and `is_base` with their own operators.
- `src/dynamic/<family>.rs` - per quantity, a unit enum
  (`dynamic::mechanical::LengthUnit`) with `const_derive_more`'s `Values`
  and `Variants` plus `unit_derives`: each variant names its unit
  (`#[value(KILOMETRE.unit())]`), `Display` and `FromStr` are its unit's
  symbol, `From<LengthUnit> for SIUnit` converts it, and `Default` is the
  coherent unit - a variant of its own, first, where no unit of the
  quantity is it (`VelocityUnit::MetrePerSecond`).
- `Cargo.toml` - a starting manifest: one feature per family (all on by
  default), `std`, `libm` (one of the two is needed by log units) and
  `serde`. `yadc-si`'s own manifest is maintained by hand (see
  [Regenerating](../yadc-si/README.md#regenerating)).

A crate that defines a system needs the features at the top of the
generated `lib.rs`; a crate that only uses one needs `generic_const_exprs`.

## TOML reference

`[system]`, `[dim]` and `[dimensionless]` are shared between both modes.

### `[system]`

```toml
[system]
crate_name    = "yadc-si"                  # output crate name (and default output dir)
family_order  = ["mechanical", "angular"]  # order of features and `pub mod`s
qty_type      = "SIQuantity"               # quantity type       (default "Qty")
dynqty_type   = "DynamicSIQuantity"        # dynamic quantity    (default "DynQty")
unit_type     = "SIUnit"                   # unit type           (default "QtyUnit")
unit_derives  = ["Copy", "Clone", "PartialEq", "Eq", "Hash", "Debug"]  # on every unit enum
named         = ["N", "J", "W", "Pa"]      # coherent units printed by name
extra_modules = [                          # hand-written modules: declared, never written
    { name = "uom_bridge", feature = "uom" },
]
```

- `unit_derives` should include `Copy` and `Clone`: `Values` does not write
  them.
- `named` lists symbols of size-one units of quantities with a dimension: a
  quantity in that product of base units prints the symbol (`6 N`, not
  `6 kg·m/s^2`). The first listed wins where two share a product.
- `extra_modules` are files you maintain in `src/` yourself. The generated
  `lib.rs` declares each as `pub mod` (behind `feature` when given); the
  generator never writes them.

### `[dim]`

```toml
[dim]
fields = ["m", "kg", "s", "A", "K", "mol", "cd", "rad"]
```

The base units' symbols. Each is the symbol of a size-one unit of a
quantity whose `dims` are that field alone (`Length`, `dims = {m = 1}`,
`Metre`); that unit's constant (`METRE`) is declared at the crate root.
Whether mass is based on the gram or the kilogram, or angle is a base unit
at all, is this list.

### `[dimensionless]`

The units of a plain number, in the `dimensionless` module, as a
quantity's `units`:

```toml
[dimensionless]
units = [
    {v = "Ratio",   m = "1.0",  s = "",  l = ""},
    {v = "Percent", m = "0.01", s = "%", l = "percent"},
    {v = "Decibel", m = "1.0", log = "10.0", per = "10.0", s = "dB", l = "decibels"},
]
```

## Manual mode

### `[[family]]` and `[[family.quantity]]`

```toml
[[family]]
name = "mechanical"           # a Cargo feature and a module

[[family.quantity]]
name = "Length"
dims = {m = 1}                # non-zero exponents of [dim] fields
units = [
    {v = "Metre",     m = "1.0",    s = "m",  l = "metres"},
    {v = "Kilometre", m = "1000.0", s = "km", l = "kilometres"},
    {v = "Inch",      m = "0.0254", s = "in", l = "inches"},
]
```

### Unit fields

| Key | Meaning |
|---|---|
| `v` | the enum variant, and the constant's name unless `t` gives one |
| `m` | the size in base units (for a log unit, its reference), an exact expression |
| `z` | the zero in base units, the same way (`"273.15"`) |
| `log`, `per` | a log scale, both or neither: `n` of the unit is `m · log^(n / per)` base units |
| `s` | the symbol: `Display` and `FromStr` |
| `l` | the long plural name, in the constant's doc comment |
| `t` | the constant's name, when not the variant's (`t = "DegreeCelsius"`, `DEGREE_CELSIUS`) |

A unit has a zero, a log scale, or neither; any other key is an error. A
number `n` of a unit with a zero is `n · m + z` base units. A quantity's
first unit of size one with neither (`m²` for an area, `N · m` for a torque)
is its coherent unit, `BASE`; a later unit of size one (`°C` for a
temperature step) is a named unit of its own, `NOT_BASE`, and converts with
`From`.

`m`, `z`, `log` and `per` are exact expressions - numbers, `+ - * /`,
parentheses and `core::f64::consts::PI` - written out as `Exact`s (`0.0254`
is `Exact::dec(254, -4)`) and evaluated at compile time:

```toml
{v = "Celsius",          m = "1.0", z = "273.15", s = "°C", l = "degrees Celsius"},
{v = "Fahrenheit",       m = "5.0 / 9.0", z = "273.15 - 32.0 * 5.0 / 9.0", s = "°F", l = "degrees Fahrenheit"},
{v = "DecibelMilliwatt", m = "0.001", log = "10.0", per = "10.0", s = "dBm", l = "decibels-milliwatt"},
{v = "Parsec",           m = "149597870700.0 * 648000.0 / core::f64::consts::PI", s = "pc", l = "parsecs"},
```

A unit's constant is its `t`, or else its variant, in SCREAMING_SNAKE_CASE
(`KilometrePerHour` is `KILOMETRE_PER_HOUR`); where two quantities of one
module share a name, it is prefixed with its quantity's (`LENGTH_UNCIA`)
unless a `t` names it.

## UCUM mode

Parses `ucum-essence.xml` from the [Unified Code for Units of Measure](https://ucum.org)
and generates every unit it defines. `ucum.toml` generates `yadc-ucum`:

```bash
cargo run -p yadc-codegen -- yadc-codegen/ucum.toml -o yadc-ucum
```

### `[ucum]`

```toml
[ucum]
# A release's ucum-essence.xml, pinned by its tag. "latest" asks GitHub for
# the newest release; a file path reads a local copy.
source = "https://raw.githubusercontent.com/ucum-org/ucum/v2.2/ucum-essence.xml"
```

Updating to a new UCUM release is changing the tag and regenerating.

### What gets generated

- **UCUM's base units are the base units:** `[dim].fields` lists their
  codes, `m`, `s`, `g`, `rad`, `K`, `C` (the coulomb) and `cd`. The mole is
  a number, as UCUM defines it (6.02214076 × 10²³).
- **Exact values.** Every value in the XML is read exactly - a rational,
  with π kept as a power of π: the international inch is
  `Exact::dec(254, -4)` metres, the US survey inch `Exact::int(100) /
  Exact::int(3937)`.
- **UCUM codes.** A unit's symbol is its case-sensitive UCUM code
  (`[in_i]`, `Cel`).
- **One family per UCUM class**, each a Cargo feature, and within a family
  one quantity per UCUM property and dimension; class `dimless` goes to
  `dimensionless.rs`:

  | Family | UCUM classes |
  |--------|-------------|
  | `si` | `si` |
  | `heat` | `heat`, plus `ThermodynamicTemperature` (`CELSIUS`, `FAHRENHEIT`) and `TemperatureInterval` |
  | `iso1000` | `iso1000` |
  | `intcust` | `intcust` |
  | `cgs` | `cgs` |
  | `infotech` | `infotech` |
  | `us` | `us-lengths`, `us-volumes`, `us-masses`, `avoirdupois`, `troy`, `apoth` |
  | `british` | `brit-length`, `brit-volumes` |
  | `clinical` | `clinical`, `chemical` |
  | `constants` | `const` |
  | `misc` | the rest |

- **Skipped:** `isSpecial` units (logarithmic and function-based: pH, B,
  dB, Np, ...; Celsius and Fahrenheit are written as units with a zero in
  `heat`), `isArbitrary` units (international units, titres), and units
  whose expression does not resolve (the generator prints how many).
