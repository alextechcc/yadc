# yadc — Yet Another Dimensional Crate

A compile time *and* runtime dimensional analysis crate where you get to *own your own types.* You'll own everything and they'll be pissed!

### Why Do We Need This?

As far as I can tell, every other crate that does unit systems defines some amount of the the types you will use, or has macros do it, this means:
- You can't choose the memory layout, or naming. A breaking library change is a breaking change for you too.
- The crate probably only supports limited number types, or has tons of flags for enabling each specific third party type.
- You often can't `#[derive()]` or `impl` your own traits (orphan rule!), so you end up with tons of flags for enabling each third party trait.
- You may have to learn a special DSL, which may not support controlling `pub` or or adding extra fields (none that I've seen) or doc comments.
- If it has a macro to generate everything at once, chances are that the things it generates are hardcoded to work with eachother without traits.
- It's unclear what types get generated, where they are, what they're named.
- You can't create your own special sauce implementation without common ground traits.

Crates like `serde` and `clap` let you define your own types and customize in depth how the derive macros generate standard traits, and I wanted this for dimensional analysis too. Reflection solves some, but not all these problems.

### Quick Start

```rust
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use yadc::StaticQuantity;
use yadc_si::mechanical::*;
use yadc_si::thermal::*;
use yadc_si::electromagnetic::*;

// Bring values in to unit system with from_unit()
let voltage = ElectricPotential::from_unit(12.0, ElectricPotentialUnit::Volt);
let current = ElectricCurrent::from_unit(500.0, ElectricCurrentUnit::Milliampere);

// Standard math operations with compile time dimensional analysis
// Multiplying by the storage type also works
let power: Power = voltage * current * 2.0;

// Take values out of unit system with as_unit() and fmt_as_unit()
let power_in_watts = power.as_unit(PowerUnit::Watt);
println!(
    "{:.2}",
    power.fmt_as_unit(PowerUnit::Watt, yadc::Format::Long),
);

// Dividing the same dimension leads to dimensionless, and dimensionless can
// be turned into the scalar type with Into.
let a = Force::from_unit(9.0, ForceUnit::Newton);
let _b: f64 = (a/a).into();

// Compile time checked 'kind' aware units for angles and temperature deltas.
let body = ThermodynamicTemperature::from_unit(37.0, ThermodynamicTemperatureUnit::Celsius);
let room = ThermodynamicTemperature::from_unit(22.0, ThermodynamicTemperatureUnit::Celsius);
let _fever: TemperatureInterval = body - room;

// Dynamic quantities are also supported
use yadc::DynamicQuantity;
use yadc_si::DynamicSIQuantity;
let dyn_v = DynamicSIQuantity::from_unit(12.0, ElectricPotentialUnit::Volt);
let dyn_i = DynamicSIQuantity::from_unit(2.0, ElectricCurrentUnit::Ampere);
let _bad = (dyn_v + dyn_i).expect("you can't add volts to amps");
);

// Adding new units, unit systems, dimensions is easy, you write the types, and
// use simple #[derive()] macros to connect it all. YADC doesn't hardcode any
// types for you.
pub type Length = StaticSIQuantity<{ ISQ { m: 1, ..Dim::ZERO } }>;

// Derive whatever you want!
#[derive(Unit, JsonSchema, Reflect)]
#[dim(Length)]
pub enum LengthUnit {
    #[unit(1.0, "m", "meters")]
    Meter,
    #[unit(1000.0, "km", "kilometers")]
    Kilometer,
}
```

### Comparison With Other Crates

Sorted by total crates.io downloads. See [column explanations](#column-explanations) below.

| Crate | Stable Rust | Active | `no_std` | Own types | Dim Vector | Stores In Base Units | Custom Value Type | Angles/Affine | Full SI Coverage | Static | Dynamic | Fully Trait Based | UCUM Serde | Additional features |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **yadc** | ✗ | ? | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | dimension only | derive macros, serde |
| [uom](https://crates.io/crates/uom) | ✓ | ✓ | ✓ | ✗ | ✓ | ✓ | fixed list | ✓ | ✓ | ✓ | ✗ | ✗ | ✗ | CGS, natural units, and other unit systems |
| [dimensioned](https://crates.io/crates/dimensioned) | ✓ | ✗ | ✓ | ✗ | ✓ | ✗ | ✓ | ✗ | ✓ | ✓ | ✗ | ✗ | ✗ | CGS, UCUM unit system, natural units |
| [quantities](https://crates.io/crates/quantities) | ✓ | ✓ | ✓ | ✗ | ✗ | ✓ | ✗ | ✗ | ✗ | ✓ | ✗ | ✗ | ✗ | |
| [quantity](https://crates.io/crates/quantity) | ✓ | ✓ | ✗ | ✗ | ✓ | ✓ | ✓ | ✗ | ✓ | ✓ | ✗ | ✗ | ✗ | `ndarray` vector-valued quantities |
| [simple-si-units](https://crates.io/crates/simple-si-units) | ✓ | ✗ | ✓ | ✗ | ✗ | ✓ | ✓ | ✗ | ✗ | ✓ | ✗ | ✗ | ✗ | |
| [runtime_units](https://crates.io/crates/runtime_units) | ✓ | ✓ | ✗ | ✗ | ✓ | ✓ | ✗ | ✗ | ✓ | ✗ | ✓ | ✗ | ✗ | batch array/vector storage, serde, utoipa |
| [dimensional_quantity](https://crates.io/crates/dimensional_quantity) | ✗ | ✓ | ✓ | ✗ | ✓ | ✓ | ✗ | ✗ | ✓ | ✓ | ✗ | ✗ | ✗ | |
| [unit-conversions](https://crates.io/crates/unit-conversions) | ✓ | ✗ | ✗ | ✗ | ✗ | ✗ | ✗ | ✗ | ✗ | ✗ | ✓ | ✗ | ✗ | |
| [diman](https://crates.io/crates/diman) | ✗ | ✓ | ✓ | ✗ | ✓ | ✓ | ✓ | ✗ | ✓ | ✓ | ✗ | ✗ | ✗ | rational exponents (`m^½`) |
| [uy](https://crates.io/crates/uy) | ✓ | ✓ | ✓ | ✗ | ✓ | ✗ | ✗ | ✗ | ✗ | ✓ | ✗ | ✗ | ✗ | |
| [const-units](https://crates.io/crates/const-units) | ✗ | ✗ | ✓ | ✗ | ✓ | ✓ | ✗ | ✗ | ✗ | ✓ | ✓ | ✗ | ✗ | |
| [measurement](https://crates.io/crates/measurement) | ✓ | ✗ | ✗ | ✗ | ✗ | ✓ | ✗ | ✗ | ✗ | ✓ | ✗ | ✗ | ✗ | |
| [fts_units](https://crates.io/crates/fts_units) | ✓ | ✗ | ✗ | ✗ | ~ | ✗ | ✓ | ✗ | ✗ | ✓ | ✗ | ✗ | ✗ | `Vector3` support, no implicit conversions |
| [dyn_quantity](https://crates.io/crates/dyn_quantity) | ✓ | ✓ | ✗ | ✗ | ✓ | ✓ | ✓ | ✓ | ✗ | ✗ | ✓ | ✗ | ✗ | string unit expression parsing, optional uom bridge |
| [simple_units](https://crates.io/crates/simple_units) | ✓ | ✗ | ✗ | ✗ | ✗ | ✓ | ✗ | ✗ | ✗ | ✓ | ✗ | ✗ | ✗ | |
| [whippyunits](https://crates.io/crates/whippyunits) | ✗ | ✓ | ✓ | ✗ | ✓ | ✗ | ✗ | ✓ | ✓ | ✓ | ✗ | ✗ | unit support | LSP, compiler error type renderer |

### Column Explanations

Apologies if I've got some of these columns wrong.

- **Active** — updated within the last two years (checked May 2026). ✗ does not mean unusable - it's not like unit systems change much.
- **`no_std`** — can be used without the Rust standard library (e.g. on embedded targets).
- **Own types** — you define all the involves types in your own crate; the library only provides traits and derives. ✗ means the library defines the struct and you depend on it directly, or macros define it for you.
- **Dim Vector** — multiplying two quantities automatically produces the correct output type without you manually registering the relationship. E.g. `mass * accel` gives you a force-typed value purely from the dimension exponent arithmetic. ✗ means you must define each quantity and its operations by hand, or use explicit algebraic derivation macros.
- **Stores In Base Units** — the stored value is a raw number in canonical base units; the unit plays no role in storage. ✗ means either the unit is baked into the value's scale (so `5 km` stores `5.0`, not `5000.0`) or a unit tag is packed alongside the value at runtime.
- **Custom Value Type** — the underlying numeric type is customizable, so you can use `f32`, `i64`, `Vector3<f32>`, or any custom type. ✗ means the library is hardwired to one type. "fixed list" means the library supports only certain types it has added support for.
- **Angles/Affine** — the type system distinguishes angle quantities from plain dimensionless ratios, and distinguishes absolute temperatures from temperature intervals, so `T − T = ΔT` and `T + T` is a compile error.
- **UCUM Serde** — whether the serde wire format uses [UCUM](https://ucum.org) unit strings. `dim` means the quantity's *dimension* is expressed as a UCUM base-unit composition (e.g. `m.kg.s-2`, `rad`, `K{Affine}`), with the stored value always in SI base units — round-trip lossless for base unit symbols. `unit` means the specific *storage unit* is expressed as a UCUM string (value may not be in SI base). `dimensioned`'s UCUM feature is a compile-time unit system of UCUM-named constants, not a serde format.
- **Full SI Coverage** — ships with a complete, ready-to-use SI implementation covering all ISQ quantities and units.
- **Static** — dimensions are checked at compile time; a wrong-dimension operation fails to compile.
- **Dynamic** — dimensions are checked at runtime; mismatched-dim operations return `Option` or `Result`.
- **Fully Trait Based** — the library exposes public traits (`StaticQuantity`, `Unit`, …) as abstraction boundaries so you can write functions that are generic over *any* conforming quantity type — including quantities from unit systems the library has never seen. ✗ means the library provides only concrete types (even if parameterized); a function written against those types is always tied to one unit system.

### Nightly features required

Up front, this is a pretty experimental crate and relies on a bunch of fairly unstable nightly features, the goal is 'no comprimise, with what nightly has'.

## Highlights

- **Const-generic dimensions.** `Qty<const D: Dim>` parameterizes the quantity type by a dim *value*, so `Mass` and `Force` are genuinely distinct types and `mass + force` doesn't compile.
- **Type-level arithmetic.** `Length * Length → Area`. `Mass * Acceleration → Force`. The output type is derived at compile time by the `StaticQuantity` derive emitting where-clauses on const-fn dim arithmetic.
- **User-owned kind tagging.** A `#[kind]` field on `Dim` carries a user-defined enum (`Linear`, `Affine`, `Angle`, `SolidAngle`, `Invalid`) that distinguishes `Angle` from `Dimensionless` and absolute `ThermodynamicTemperature` from `TemperatureInterval`. `T - T = ΔT` works at the type level; `T + T` fails to compile.
- **No external dependency on a unit graph.** Conversion factors and unit metadata live in user-defined enums with `#[unit(multiplier, "symbol", "long form")]` attributes.
- **Optional `DynQty`** for cases where the dimension is only known at runtime — a separate `DynamicQuantity` trait with `Option`-returning arithmetic.
- **Optional serde** with a UCUM wire format for `Dim` (e.g. `"m.kg.s-2"`, `"rad"`, `"K{Affine}"`) via the `DimensionSerialize` / `DimensionDeserialize` derives.

### Sub-Packages

| Crate | Purpose |
|---|---|
| **[`yadc`](yadc/)** | Core traits (`Dimension`, `Unit`, `StaticQuantity`, `DynamicQuantity`, `Format`) and re-exports of the derive macros. |
| **[`yadc-si`](yadc-si/)** | A ready-made SI implementation built on top of `yadc` — 117 quantities (`Mass`, `Force`, `Energy`, `ThermodynamicTemperature`, …) grouped into 9 feature-gated families. Taken from [`uom`](https://github.com/iliekturtles/uom) |
| **[`yadc-derive`](yadc-derive/)** | Proc-macro derives (`Dimension`, `Kind`, `Unit`, `StaticQuantity`, `DynamicQuantity`, `DimensionSerialize`, `DimensionDeserialize`). Internal — depend on `yadc` instead. |
| **[`yadc-codegen`](yadc-codegen/)** | CLI code generator: reads a `.toml` unit-system definition and emits a ready-to-compile yadc-compatible Rust crate. Run `yadc-codegen <input.toml> [-o <output-dir>]`. The TOML schema covers the `Kind` enum, base dimension fields, dimensionless units, and quantity families (each becoming a Cargo feature and `pub mod`). Quantities specify dimension exponents, units with multipliers and symbols, and optional affine offsets for temperature-style quantities. `si.toml` in that directory is the canonical example and the source of `yadc-si`. |

## Examples
- **`define_unit_system.rs`** — define a custom unit system from scratch (no `yadc-si`).
- **`basic_use.rs`** — tour of the SI quantities: from_unit, mul/div, affine temperature, scalar math, DynQty, etc.
- **`serde_demo.rs`** — JSON round-trip of typed and dynamic quantities.
- **`custom_value_type.rs`** — custom numeric type (`MyF64`) with `#[identities(one=…, zero=…)]`.
- **`generic_fns.rs`** — unit-system-agnostic generic functions (`clamp`, `lerp`, `mean`) called with both SI and a custom nautical unit system.

## Credits
- [`uom`](https://github.com/iliekturtles/uom) for the definitions of the units and the idea of a dimension 'kind'.

## License

Dual-licensed under MIT or BSD-3-Clause. See [`LICENSE-MIT`](LICENSE-MIT) / [`LICENSE-BSD`](LICENSE-BSD).
