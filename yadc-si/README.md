# yadc-si

An SI unit system built with [`yadc`](../yadc/). It is not part of yadc's API - it is one example of a system *you* could own, generated from [`yadc-codegen/si.toml`](../yadc-codegen/si.toml). Every modelling choice here (radians as an exponent axis, absolute temperature as an `Absolute` label, an instant as a point, decibels on a logarithmic line) is this crate's, not yadc's.

What it defines, in `lib.rs`:

- `Kind` - how a quantity behaves under `+` and `-`, beside its exponents: `Linear` (`#[default]`: an ordinary amount, which adds to its own kind, scales and multiplies), `Absolute` (a temperature: an amount with a true zero: it scales and multiplies as an amount does, but two of them differ by a linear amount and a linear amount moves one; a label of this system's own, with three rules), `Affine` (a point, measured from a zero somebody chose - an instant: two differ by a linear amount, an amount moves one, and it has no sum with another, no scaling and no products) and `Log` (a logarithm - a level or a gain: adding logarithms multiplies what they are the logarithms of, so `+` and `-` add and subtract the exponents). The last two are marked on their variants, `#[kind(affine)]` and `#[kind(log)]`.
- `ISQ` - the dimension: one `i8` exponent per SI base unit (`m`, `kg`, `s`, `A`, `K`, `mol`, `cd`) and one for the radian (`rad`), plus the kind. Angle is `rad: 1`, solid angle `rad: 2` (sr = rad²), angular velocity rad/s; torque is J/rad and moment of inertia kg·m²/rad², so torque × angle is energy and an angular speed never mixes with a frequency. Its `*` and `/` are partial (`Option`): a product with a point or a level has no result. It prints as `m·kg·s⁻²`, a kind that is not `Linear` in braces (`K{Absolute}`).
- `StaticSIQuantity<const D: ISQ, N = f64>(N)` - the compile-time-checked quantity (`#[derive(Quantity)]`), stored in coherent SI units (kilograms, metres, kelvin, radians, watts, ...), counted in `f64` unless you say otherwise (`KILOPASCAL.cast::<f32>()`). Its field is private to this crate: the base constants are the only raw numbers, and everyone else builds and reads through units. It prints as its number and dimension (`9.81 m·s⁻²`).
- `DynamicSIQuantity` - the same with the dimension carried at run time, as `dim: ISQ`, built with [`dynamickit`](../dynamickit/)'s `#[derive(Dynamic)]`: `StaticSIQuantity<const D: ISQ> => { dim: D, value: .0 }` (converting moves the number in and records `D`; `TryFrom` checks it on the way back, handing the value back on a mismatch), and `shared f64 => { dim: ISQ::DIMENSIONLESS, value: self }` (a bare number is the same value as a dimensionless quantity). Its operators and methods are every field's own, each with its output stated: `*`, `/`, `+`, `-`, scaling by an `f64`, `partial_powi` and `partial_root` return an `Option` (`None` where the dimensions don't combine: a sum of two dimensions, a product with a point or a level, a scaled point or level), so a calculation reads `((a * b)? + c)?`; values of one dimension are ordered. No impl on it is hand-written.
- `base` - the base constants, the only raw numbers in the crate: `ONE`, `METRE`, `KILOGRAM`, `SECOND`, `AMPERE`, `KELVIN` (a step: a temperature difference of 1 K), `MOLE`, `CANDELA`, `RADIAN`, `ABSOLUTE_ZERO` (a temperature), `UNIX_EPOCH` (the base zero of `Instant`: what an instant is stored, printed and serialized from), `BEL` (a gain of ten times) and `WATT_LEVEL` (the level of one watt).
- `Scale<P, S>(pub P, pub S)` - a scale: a zero and a step. A unit of its zero's type - `Scale(ICE_POINT, KELVIN)` counts kelvins from the ice point - made as a constant or at run time.

The crate root re-exports `yadc`'s `Dimension`, `Quantity` and `Unit`. Its one point is the `Instant`; a system with altitudes or positions gives them the affine kind the same way (see [`yadc-tests/src/lib.rs`](../yadc-tests/src/lib.rs)).

and ~120 quantity aliases (`pub type Length = StaticSIQuantity<{ ISQ { m: 1, ..ISQ::DIMENSIONLESS } }>;`), each with a `<Q>Unit` enum naming its units (`PowerUnit` holds W, kW, dBW and dBm), organized into 9 feature-gated families:

| Feature | Examples |
|---|---|
| `mechanical` | Length, Mass, Time, Instant, Force, Energy, Power, PowerLevel, Gain, Velocity, Acceleration, Frequency, Torque, … |
| `angular` | Angle, SolidAngle, AngularVelocity, AngularAcceleration, … |
| `electromagnetic` | ElectricCurrent, ElectricPotential, Capacitance, MagneticFlux, ElectricalResistance, … |
| `thermal` | ThermodynamicTemperature, TemperatureInterval, HeatCapacity, ThermalConductivity, … |
| `fluid` | DynamicViscosity, MassRate, VolumeRate, SurfaceTension, DiffusionCoefficient, … |
| `density` | MassDensity, LinearMassDensity, ArealMassDensity, VolumetricNumberDensity, … |
| `chemistry` | AmountOfSubstance, MolarMass, MolarConcentration, Radioactivity, CatalyticActivity, … |
| `photometric` | Luminance, LuminousIntensity, Illuminance, RadiantExposure |
| `information` | Information, InformationRate |

`Dimensionless` (`StaticSIQuantity<{ ISQ::DIMENSIONLESS }>`) with `DimensionlessUnit` (ratio, %, ‰, ppm, ...) is always present. All families are on by default; disable selectively with `default-features = false` plus an explicit feature list to reduce compile time.

## Nightly features

`yadc-si` defines a unit system, so its own crate root carries
`generic_const_exprs`, `min_adt_const_params`, `const_trait_impl`,
`const_ops`, `const_cmp`, `const_default`, `const_convert`, `const_try`,
`const_destruct`, `derive_const`, `fn_traits` and `unboxed_closures` (plus
`allow(incomplete_features)`): the derives write the quantity operators, the
static ↔ dynamic bridge and the units' `Unit` as `const impl`s, and make
every unit callable through the std function traits. A crate that only *uses*
the aliases and units exported here - calling them included - needs just:

```rust
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
```

and, to do quantity arithmetic in its own `const` items (below), also
`const_trait_impl` and `const_ops`. A crate that derives `Unit` on an enum of
its own adds `const_trait_impl`, `fn_traits` and `unboxed_closures`.

## Usage

A unit turns a number into a quantity and a quantity back into a number, and is called like the function it is: on a number it builds, on a quantity it reads. Every quantity constant and every unit enum's value is one. A scale is a unit too, used through its methods: `CELSIUS.quantity(25.0)`, `CELSIUS.value(t)`. Maths is on quantities.

```rust
use yadc::Unit;
use yadc_si::mechanical::consts::{
    HOUR, KILOGRAM, KILOMETRE, KILOMETRE_PER_HOUR, METRE_PER_SECOND_SQUARED, NEWTON, POUND,
};
use yadc_si::mechanical::*;

let mass = KILOGRAM(70.0);                              // every constant is a unit: called to build
let force: Force = mass * METRE_PER_SECOND_SQUARED(9.81);
let newtons = NEWTON(force);                            // ... and called to read: 686.7
let lb = POUND(mass);                                   // 154.32...
let v: Velocity = KILOMETRE(90.0) / HOUR(1.0);
let kmh = KILOMETRE_PER_HOUR(v);                        // 90.0
let ratio = f64::from(force / force);                   // Dimensionless is a plain number
println!("{force:.1}");                                 // 686.7 m·kg·s⁻²: a quantity prints itself

// A unit enum names a family's units, for a menu or a parser: std `Display` and
// `FromStr` - `{}` is the symbol, `{:#}` the long name. Its values are units too.
let n = ForceUnit::Newton;
println!("{:.1} {n}", n(force));                        // 686.7 N
println!("{:.1} {n:#}", n(force));                      // 686.7 newtons
let unit: MassUnit = "lb".parse().unwrap();             // Err(yadc::UnknownUnit) for an unknown symbol
let m: Mass = unit(10.0);

// The same two functions by name, through the `Unit` trait - what generic code
// (`U: Unit<Q>`) and `&dyn Unit<Q>` use.
let also = POUND.quantity(10.0);
let back = POUND.value(also);
// Or as std functions: `impl Fn(f64) -> Mass`, `impl Fn(Mass) -> f64`.
let pounds: Vec<f64> = [mass, also].into_iter().map(POUND).collect();
```

### Temperatures

`ThermodynamicTemperature` is an amount measured from absolute zero, with the `Absolute` label; `TemperatureInterval` is a difference, a *step*: an ordinary amount. A step, a zero and a scale are different things with different names: the steps are `KELVIN` and `DEGREE_FAHRENHEIT`; the zeros `ABSOLUTE_ZERO`, `ICE_POINT` (`ABSOLUTE_ZERO + 273.15 * KELVIN`) and `FAHRENHEIT_ZERO`; the scales - steps counted from a zero - are constants too, `Scale(zero, step)`: `KELVIN_SCALE`, `CELSIUS`, `FAHRENHEIT` and `RANKINE` (`pub const CELSIUS: Scale<ThermodynamicTemperature, TemperatureInterval> = Scale(ICE_POINT, KELVIN);`). A scale of your own is another, made at compile time or at run time: `Scale(this_morning, KELVIN)`.

```rust
use yadc_si::Scale;
use yadc_si::thermal::consts::{
    CELSIUS, DEGREE_FAHRENHEIT, FAHRENHEIT, KELVIN, KELVIN_SCALE, STEFAN_BOLTZMANN_CONSTANT,
};
use yadc_si::thermal::*;

let t = CELSIUS.quantity(25.0);                                  // 298.15 K
let f = FAHRENHEIT.value(t);                                  // 77.0, to rounding
let k = KELVIN_SCALE.value(t);                                // 298.15
let c = ThermodynamicTemperatureUnit::Celsius.value(t); // 25.0: the enum names the same units
let rise: TemperatureInterval = t - CELSIUS.quantity(20.0);      // T - T = ΔT
let hotter: ThermodynamicTemperature = t + rise;        // T + ΔT = T
let step = DEGREE_FAHRENHEIT(rise);                     // 9.0
let mean: ThermodynamicTemperature = (t + hotter) / 2.0; // an average is a temperature
let since_dawn = Scale(CELSIUS.quantity(12.0), KELVIN);          // a scale made at run time
let warmed = since_dawn.value(t);                       // 13.0
// rise + t doesn't compile ("cannot add these in this order": write t + rise),
// and CELSIUS.value(rise) doesn't either: a scale with a zero reads a temperature, not a step.
```

Absolute zero is a fact, so a temperature scales and multiplies as any amount does - in kelvins, whatever scale built it - and the label drops out of a product:

```rust
use yadc::PowiAs;
use yadc_si::chemistry::consts::{MOLAR_GAS_CONSTANT, MOLE};

let energy = MOLE * MOLAR_GAS_CONSTANT * t;              // n R T: joules
let twice: ThermodynamicTemperature = 2.0 * t;           // 596.3 K
let flux = STEFAN_BOLTZMANN_CONSTANT * t.powi::<4>();    // σT⁴: W/m²
```

### Levels: dB, dBm, Np

Decibels count on a logarithmic line, where multiplying is adding. Both quantities on it have the `Log` kind, whose `+` and `-` add and subtract the exponents. A `Gain` is the logarithm of a plain ratio - no exponents - a quantity of its own with plain units (`BEL`, `DECIBEL`, `NEPER`; `GainUnit`), and a `PowerLevel` the logarithm of a power, with power's exponents: two levels differ by a gain (watts over watts), a gain moves a level, and two levels do not sum to a level (their sum is the logarithm of watts squared). A level is read on a scale like a temperature, a zero and a step: `DBW` is `Scale(WATT_LEVEL, DECIBEL)`, `DBM` is `Scale(MILLIWATT_LEVEL, DECIBEL)`.

The logarithm itself is in one place, the hand-written [`level.rs`](src/level.rs): std `From` between `PowerLevel` and `Power`, and between `Gain` and a plain ratio. A scale of levels measures levels, and the watts are one `From` away. `PowerUnit` names dBW and dBm beside the watts - each `by` the level's scale, `via = PowerLevel` - so its values take a decibel count straight to watts and back; the conversion is not `const`, so neither is that enum's.

```rust
use yadc_si::dimensionless::Dimensionless;
use yadc_si::mechanical::consts::{BEL, DBM, DBW, DECIBEL, WATT};

let sent = DBM.quantity(30.0);                                   // a PowerLevel: 30 dB above 1 mW
let p = Power::from(sent);                              // 1 W
let w = WATT(p);                                        // 1.0
let dbw = DBW.value(PowerLevel::from(p));                     // 0.0
let dbm = PowerUnit::DecibelMilliwatt;                  // in the watts' enum: both steps at once
let same = dbm(p);                                      // 30.0
let one_watt: Power = dbm(30.0);

let received = sent - DECIBEL(3.0) + DECIBEL(13.0);     // a link budget: plain + and -
let gain: Gain = received - sent;                       // 10 dB
let ratio = Dimensionless::from(gain);                  // 10.0
let watts = Power::from(received);                      // 10 W
// sent + sent doesn't compile ("cannot add: the labels have no sum"), nor does 2.0 * sent.
```

A gain scales (`3.0 * DECIBEL`) and, like a label, leaves a product: an attenuation in dB per kilometre is a plain per-length number, and over a length it is a number of bels again.

```rust
let attenuation = DECIBEL(0.2) / KILOMETRE;             // per metre, counting bels
let loss: Gain = (attenuation * KILOMETRE(15.0)) * BEL; // 3 dB
let after = sent - loss;                                // 27 dBm
```

### Instants

An `Instant` is a point in time - the affine kind. There is one line of instants, counted in uniform seconds (leap seconds are not modelled) and stored from `UNIX_EPOCH`, its base zero: that is what an instant prints and serializes as. Two instants differ by a `Time`, a `Time` moves an instant, and nothing else: an instant has no sum with another, no scaling and no products. Which zero an instant was read from is not in its type: the other zeros - `GPS_EPOCH`, `JULIAN_EPOCH`, `MODIFIED_JULIAN_EPOCH` - are constants on the same line, and the scales `UNIX_SECONDS`, `UNIX_MILLISECONDS`, `GPS_SECONDS`, `JULIAN_DATE` and `MODIFIED_JULIAN_DATE` (`InstantUnit` names them) all read it.

```rust
use yadc_si::mechanical::consts::{GPS_SECONDS, HOUR, JULIAN_DATE, SECOND, UNIX_SECONDS};

let launch = UNIX_SECONDS.quantity(1_700_000_000.0);             // an Instant
let landing = launch + HOUR(2.0);                       // instant + time = instant
let flight: Time = landing - launch;                    // instant - instant = time
let gps = GPS_SECONDS.value(launch);                          // 1_384_035_200.0: another zero, same line
let jd = JULIAN_DATE.value(launch);                           // 2_460_263.42...
let j2000 = JULIAN_DATE.quantity(2_451_545.0);                   // built on one scale,
let unix = UNIX_SECONDS.value(j2000);                         // read on another: 946_728_000.0
let mission = Scale(launch, SECOND);                    // seconds since launch: a scale made at run time
let elapsed = mission.value(landing);                   // 7200.0
// launch + landing, 2.0 * launch and launch * SECOND don't compile ("cannot add a point to a
// point", "a point cannot be scaled", "cannot multiply: these have no product").
```

An `f64` of seconds from 1970 resolves today's instants to a fraction of a microsecond.

### Dynamic quantities

When the dimension is only known at run time, `DynamicSIQuantity` carries it alongside the value. It uses the same operators and is converted with `From` / `TryFrom`: build a static quantity with a unit and convert it in, convert it back out to a static quantity - checked - and read it with a unit. Every operator returns an `Option`: a sum of two dimensions, or a product with a point or a level, has no result.

```rust
use yadc::PartialRoot;
use yadc_si::DynamicSIQuantity;
use yadc_si::mechanical::consts::{SQUARE_METRE, WATT_LEVEL};

let d = (DynamicSIQuantity::from(KILOMETRE(90.0)) / DynamicSIQuantity::from(HOUR(2.0))).unwrap();
let v: Velocity = d.try_into().unwrap();                 // Err(d) on a mismatch
let kmh = KILOMETRE_PER_HOUR(v);                         // 45.0
assert_eq!(d + DynamicSIQuantity::from(HOUR(1.0)), None); // km/h + h: no result
assert_eq!(Mass::try_from(d), Err(d));                   // hands `d` back
let side = DynamicSIQuantity::from(SQUARE_METRE(9.0)).partial_root(2); // Some(3 m)
let n = f64::try_from((d / d).unwrap());                 // Ok(1.0): a plain number comes out as `f64`
let level = DynamicSIQuantity::from(WATT_LEVEL);        // a logarithm
assert_eq!(level * d, None);                             // a level has no products
assert_eq!(level * 2.0, None);                           // nor can it be scaled
let t0 = DynamicSIQuantity::from(UNIX_SECONDS.quantity(0.0));    // a point
assert_eq!(t0 + t0, None);                               // two points have no sum
assert_eq!(t0 * 2.0, None);                              // a point cannot be scaled
```

For a unit picked at run time, `&dyn Unit<Length>` is some unit of length, checked at compile time:

```rust
let lengths: [&dyn Unit<Length>; 2] = [&KILOMETRE, &LengthUnit::Mile];
for u in lengths {
    println!("{:.1}", u.value(v * HOUR(1.0)));           // 45.0, 28.0
}
```

Some unit of *some* quantity is a set you list yourself: an enum of the quantities your program measures and an enum of their unit enums, visited with `dynamickit`'s `Visit` (each unit family names the quantity it measures in an associated type of yours; one visitor builds, one reads, each written once for every family) and listed for a menu with `Variants`; `dynamickit`'s `Members` and `Via` convert the set through `DynamicSIQuantity` and borrow its arithmetic. See [`examples/`](examples/).

Units are not tied to this crate's quantities. Another crate may make them units of its own quantity - `impl Unit<my::Length> for yadc_si::mechanical::LengthUnit` is allowed by the orphan rule - and write a unit of its own by hand: `impl Unit<Length>` on a type of its own.

See [`yadc/examples/basic_use.rs`](../yadc/examples/basic_use.rs) for a tour.

### Constants

Every family module has a generated `consts` module: the base constants of its quantities (re-exported from `base`: `mechanical::consts::METRE`), then one constant per unit, each defined from them - a coherent unit as their product, everything else as a multiple of it, as a standards document writes it:

```rust
pub const NEWTON: Force = METRE * KILOGRAM / (SECOND * SECOND);
pub const KILOMETRE: Length = 1000.0 * METRE;
pub const DEGREE_FAHRENHEIT: TemperatureInterval = (5.0 / 9.0) * KELVIN;
pub const ICE_POINT: ThermodynamicTemperature = ABSOLUTE_ZERO + 273.15 * KELVIN;
```

Quantity arithmetic is `const`, so they compose in your `const` items too:

```rust
use yadc_si::mechanical::consts::*;            // KILOMETRE, HOUR, SPEED_OF_LIGHT, ...
use yadc_si::thermal::consts::CELSIUS;

const LIMIT: Velocity = 50.0 * KILOMETRE / HOUR;              // 50 km/h
const REST_ENERGY: Energy = ELECTRON_MASS * SPEED_OF_LIGHT * SPEED_OF_LIGHT; // 8.187e-14 J
const BOILING: ThermodynamicTemperature = CELSIUS.quantity(100.0); // scales are const too
const PSI: Pressure = POUND * STANDARD_GRAVITY / (INCH * INCH); // a unit, defined as a standard writes it
let weight: Force = STANDARD_GRAVITY * mass;
let beta = SPEED_OF_LIGHT(v);                         // a constant is a unit: v in units of c
```

- **Unit constants.** Every multiple (`#[unit("km", "kilometres", by = KILOMETRE)]`) names `pub const <SCREAMING_SNAKE variant>: <Quantity>` in its family's `consts`: `mechanical::consts::KILOMETRE`, `POUND`, `KILOWATT_HOUR`, `dimensionless::consts::PERCENT`. A name two quantities of one module share is prefixed with the quantity (`TIME_MINUTE`, `ANGLE_MINUTE`).
- **Physical constants are units**, as in UCUM: each is a unit of the quantity it measures (exact, or CODATA 2018), so it gets a constant like any unit and a quantity reads in it.

  | Constant | Unit | Quantity |
  |---|---|---|
  | `mechanical::consts::SPEED_OF_LIGHT` | `VelocityUnit::SpeedOfLight` (c) | `Velocity` |
  | `mechanical::consts::PLANCK_CONSTANT` | `ActionUnit::PlanckConstant` (h) | `Action` |
  | `angular::consts::REDUCED_PLANCK_CONSTANT` | `AngularMomentumUnit::ReducedPlanckConstant` (ħ) | `AngularMomentum` (J·s/rad, so `h = ħ · 2π rad`) |
  | `mechanical::consts::STANDARD_GRAVITY` | `AccelerationUnit::StandardGravity` (g₀) | `Acceleration` |
  | `mechanical::consts::GRAVITATIONAL_CONSTANT` | `SpecificGravitationalParameterUnit::GravitationalConstant` (G) | `SpecificGravitationalParameter` (m³/(kg·s²)) |
  | `mechanical::consts::ELECTRON_MASS`, `PROTON_MASS` | `MassUnit::ElectronMass` (mₑ), `ProtonMass` (mₚ) | `Mass` |
  | `mechanical::consts::STANDARD_ATMOSPHERE` | `PressureUnit::StandardAtmosphere` (atm) | `Pressure` |
  | `electromagnetic::consts::ELEMENTARY_CHARGE` | `ElectricChargeUnit::ElementaryCharge` (e) | `ElectricCharge` |
  | `electromagnetic::consts::VACUUM_PERMITTIVITY` | `ElectricPermittivityUnit::VacuumPermittivity` (ε₀) | `ElectricPermittivity` |
  | `electromagnetic::consts::VACUUM_PERMEABILITY` | `MagneticPermeabilityUnit::VacuumPermeability` (µ₀) | `MagneticPermeability` |
  | `thermal::consts::BOLTZMANN_CONSTANT` | `HeatCapacityUnit::BoltzmannConstant` (k) | `HeatCapacity` (per kelvin of *step*, so `k * KELVIN(300.0)` is an `Energy`) |
  | `thermal::consts::STEFAN_BOLTZMANN_CONSTANT` | `StefanBoltzmannCoefficientUnit::StefanBoltzmannConstant` (σ) | `StefanBoltzmannCoefficient` (W/(m²·K⁴)) |
  | `chemistry::consts::AVOGADRO_CONSTANT` | `InverseAmountOfSubstanceUnit::AvogadroConstant` (N_A) | `InverseAmountOfSubstance` (mol⁻¹) |
  | `chemistry::consts::MOLAR_GAS_CONSTANT` | `MolarHeatCapacityUnit::MolarGasConstant` (R) | `MolarHeatCapacity` |

## `std` and `no_std`

| Build | `Cargo.toml` |
|---|---|
| hosted (default) | `yadc-si = "0.1"` |
| `no_std` | `yadc-si = { version = "0.1", default-features = false, features = ["libm", "mechanical"] }` |

`std` and `libm` are mutually redundant but one of them is required: the
level conversions (`dBW`, `dBm`, a gain as a ratio) and the dynamic type's
roots need float maths (`powf`, `log`), which
`f64` has in `core` only through `std` or `libm`. `libm` pulls in software
float math and keeps the whole dependency tree free of `std` (verifiable with
`cargo tree -e features -p yadc-si --no-default-features --features
libm,mechanical`). `serde` works in either mode - it brings in `serde/alloc`,
so a `no_std` target still needs an allocator for it.

## Optional features

- `serde` (off by default; turns on `yadc/ucum`) — `StaticSIQuantity` serializes transparently as its number (the dimension comes from the target type), and `DynamicSIQuantity` as `{ dim, value }` with the dimension as a UCUM string (`"m.kg.s-2"`, `"K{Absolute}"`, `"s{Affine}"`, `"rad"`, `"rad2"`, `"1{Log}"`, `"1"`).
- `uom` (off by default) — `From` / `TryFrom` impls bridging `StaticSIQuantity` / `DynamicSIQuantity` and [`uom`](https://docs.rs/uom)'s typenum-based quantities (uom's angle kinds become `rad` exponents, its temperature kind the `Absolute` label).

## Regenerating

Everything in `src/` except `level.rs` and `uom_bridge.rs` is generated by [`yadc-codegen`](../yadc-codegen/) from [`yadc-codegen/si.toml`](../yadc-codegen/si.toml). To regenerate, from the workspace root:

```sh
cargo run -p yadc-codegen -- yadc-codegen/si.toml -o <dir>
```

then copy `<dir>/src/*.rs` over `yadc-si/src/` (never generate straight into `yadc-si`: the generated `Cargo.toml` is a starting point, not this crate's). Keep the hand-written `src/level.rs` (the logarithms between a level and a power, a gain and a ratio) and `src/uom_bridge.rs` (orphan-rule impls that must sit beside `StaticSIQuantity`) - `si.toml` lists them under `extra_modules`, so the generated `lib.rs` declares them but the generator never writes them.

## Origin of the unit data

Symbols, labels and measured conversion factors are derived from the [`iliekturtles/uom`](https://github.com/iliekturtles/uom) project (MIT/Apache-2.0); a unit with an exact definition (inch, pound, standard gravity, survey foot, ...) is written from it (see `yadc-codegen/si.toml`).

## License

MIT OR BSD-3-Clause.
