# yadc-si

An SI unit system built with [`yadc`](../yadc/). It is not part of yadc's API - it is one example of a system *you* could own, generated from [`yadc-codegen/si.toml`](../yadc-codegen/si.toml). Every modelling choice here (radians as an exponent axis, absolute temperature as an `Absolute` label, an instant as a point, decibels on a logarithmic line) is this crate's, not yadc's.

What it defines, in `lib.rs`:

- `Kind` - how a quantity behaves under `+` and `-`, beside its exponents: `Linear` (`#[default]`: an ordinary amount, which adds to its own kind, scales and multiplies), `Absolute` (a temperature: an amount with a true zero: it scales and multiplies as an amount does, but two of them differ by a linear amount and a linear amount moves one; a label marked `#[kind(absolute)]`), `Affine` (a point, measured from a zero somebody chose - an instant: two differ by a linear amount, an amount moves one, and it has no sum with another, no scaling and no products) and `Log` (a logarithm - a level or a gain: adding logarithms multiplies what they are the logarithms of, so `+` and `-` add and subtract the exponents). The last two are marked on their variants, `#[kind(affine)]` and `#[kind(log)]`.
- `ISQ` - the dimension: one `i8` exponent per SI base unit (`m`, `kg`, `s`, `A`, `K`, `mol`, `cd`) and one for the radian (`rad`), plus the kind. Angle is `rad: 1`, solid angle `rad: 2` (sr = rad²), angular velocity rad/s; torque is J/rad and moment of inertia kg·m²/rad², so torque × angle is energy and an angular speed never mixes with a frequency. Its `*` and `/` are partial (`Option`): a product with a point or a level has no result. It prints as `m·kg·s⁻²`, a kind that is not `Linear` in braces (`K{Absolute}`).
- `SIQuantity<const D: ISQ, N>(N)` - the compile-time-checked quantity (`#[derive(Quantity)]`), stored in coherent SI units (kilograms, metres, kelvin, radians, watts, ...), counted in the number type `N`. Each quantity has a dimension constant and an alias that takes the number type: `pub const LENGTH: ISQ = ISQ { m: 1, ..ISQ::DIMENSIONLESS }; pub type Length<N> = SIQuantity<LENGTH, N>;`, used as `Length<f64>`, `Length<f32>`, `Length<i32>` or `fn f<N>(d: Length<N>)`. Its field is private to this crate, and a quantity value is a unit of its own kind (`q.quantity(n)` is `q * n`, `q.value(x)` is `x / q`). It prints as its number and dimension (`9.81 m·s⁻²`).
- `SIUnitConsts` and `SIUnit<const D: ISQ, const C: SIUnitConsts>` - a unit: a type of its own, its dimension and its definition in const parameters. `SIUnitConsts { size, zero, symbol, name }` is the definition - the size and the zero in the coherent unit, exact, `n` in the unit being `n · size + zero` coherent units - and `SIUnit`'s one hand-written `impl Unit` is the only place a number becomes a quantity, in any number type `N: const From<Exact>` with `+ - * /`; the size and the zero are rounded to `N` once each, at compile time. It prints its symbol (`{}`) or name (`{:#}`).
- `Exact` - the exact number a unit's size and zero are: `num/den · 10ⁿ · πᵏ`, with `const` arithmetic, and std `From` to `f64`, `f32` and every primitive integer.
- `Prefix` - the 20 SI prefixes (`Prefix::Kilo`), with `symbol()` (`k`), `name()` (`kilo`) and `factor()` (an `Exact`); and `Prefixed<U>` - a unit with an optional prefix, `Prefixed { prefix: Some(Prefix::Kilo), unit: LengthUnit::Metre }`: a unit of what `U` measures, which prints `km` (`{:#}`: `kilometres`) and parses back.
- `dynamic::DynamicSIQuantity` - a quantity with the dimension carried at run time, as `dim: ISQ`, built with [`dynamickit`](../dynamickit/)'s `#[derive(Dynamic)]`: `SIQuantity<const D: ISQ, f64> => { dim: D, value: .0 }` (converting moves the number in and records `D`; `TryFrom` checks it on the way back, handing the value back on a mismatch), and `shared f64 => { dim: ISQ::DIMENSIONLESS, value: self }` (a bare number is the same value as a dimensionless quantity). Its operators and methods are every field's own, each with its output stated: `*`, `/`, `+`, `-`, scaling by an `f64`, `partial_powi` and `partial_root` return an `Option` (`None` where the dimensions don't combine: a sum of two dimensions, a product with a point or a level, a scaled point or level), so a calculation reads `((a * b)? + c)?`; values of one dimension are ordered.
- `dynamic::DynamicSIUnit` - a unit with its dimension and definition carried at run time, `{ dim: ISQ, consts: SIUnitConsts }`, made from any `SIUnit` with `#[derive(Dynamic)]` (`TryFrom` is the checked way back) and from any unit enum's value. It is a unit of `Option<DynamicSIQuantity>` counted in `Option<f64>`, as dynamic results come: `value` reads `None` for another dimension.

The crate root re-exports `yadc`'s `Dimension`, `Quantity` and `Unit`. Its one point is the `Instant`; a system with altitudes or positions gives them the affine kind the same way (see [`yadc-tests/src/lib.rs`](../yadc-tests/src/lib.rs)).

There are ~120 quantities, organized into 9 feature-gated families. Each family module (`mechanical`) holds its quantities' dimension constants and aliases and a `units` module with one type per unit (`mechanical::units::Mile`, built as `Mile {}`) - ~860 in all. Each quantity's `<Q>Unit` enum, a run-time choice among its units in any number type they count in, lives in the dynamic half beside `DynamicSIQuantity` - what is chosen at run time, a dimension or a unit: `dynamic::mechanical::LengthUnit`, `LengthUnit::Foot.quantity(3.0_f64)`.

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

`Dimensionless<N>` (`SIQuantity<{ ISQ::DIMENSIONLESS }, N>`) with its units (`Ratio`, `Percent`, `PerMille`, `PartPerMillion`, ...) and `dynamic::dimensionless::DimensionlessUnit` is always present. All families are on by default; disable selectively with `default-features = false` plus an explicit feature list to reduce compile time.

## Nightly features

`yadc-si` defines a unit system, so its own crate root carries
`generic_const_exprs`, `adt_const_params`, `unsized_const_params`,
`const_trait_impl`, `const_ops`, `const_cmp`, `const_default`,
`const_convert`, `const_try`, `const_destruct` and `derive_const` (plus
`allow(incomplete_features)`): the derives write the quantity operators, the
static ↔ dynamic bridge and the units' `Unit` as `const impl`s. A crate that
only *uses* the aliases and units exported here needs just:

```rust
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
```

and, to build quantities in its own `const` items (`const LIMIT: Velocity<f64>
= KilometrePerHour {}.quantity(50.0);`), also `const_trait_impl`; to do
quantity arithmetic in them, `const_trait_impl` and `const_ops`. A crate that
derives `Unit` on an enum of its own adds `const_trait_impl`.

## Usage

A unit turns a number into a quantity and a quantity back into a number, through the `Unit` trait: `quantity` builds, `value` reads. Every unit is a type (`Pound {}.quantity(10.0_f64)`, `Pound {}.value(m)`); so is a unit enum's value, and a quantity value is a unit of its own kind. Maths is on quantities.

```rust
use yadc::Unit;
use yadc_si::dynamic::mechanical::*;
use yadc_si::mechanical::units::*;
use yadc_si::mechanical::*;
use yadc_si::{Prefix, Prefixed};

const LIMIT: Velocity<f64> = KilometrePerHour {}.quantity(50.0); // built at compile time

let mass = Kilogram {}.quantity(70.0_f64);              // a unit builds
let g0: Acceleration<f64> = StandardGravity {}.quantity(1.0); // a physical constant is a unit too
let force: Force<f64> = mass * g0;
let newtons = Newton {}.value(force);                   // ... and a unit reads: 686.47
let lb = Pound {}.value(mass);                          // 154.32...
let v: Velocity<f64> = KilometrePerHour {}.quantity(90.0);
let d: Length<f64> = v * Hour {}.quantity(2.0);         // 180 km
let beta = SpeedOfLight {}.value(v);                    // v in units of c
let ratio = f64::from(force / force);                   // Dimensionless is a plain number
println!("{force:.1}");                                 // 686.5 m·kg·s⁻²: a quantity prints itself
println!("{newtons:.1} {} ({:#})", Newton {}, Newton {}); // 686.5 N (newtons)

// A unit enum names a family's units, for a menu or a parser: std `Display` and
// `FromStr` from its unit types - `{}` is the symbol, `{:#}` the long name. Its
// values are units too, in any number type.
let n = ForceUnit::Newton;
println!("{:.1} {n}", n.value(force));                  // 686.5 N
println!("{:.1} {n:#}", n.value(force));                // 686.5 newtons
let unit: MassUnit = "lb".parse().unwrap();             // Err(yadc::UnknownUnit) for an unknown symbol
let m: Mass<f64> = unit.quantity(10.0);
let m32: Mass<f32> = unit.quantity(10.0);               // the same unit, in f32

// A prefix before a unit's symbol: `Prefixed<U>`, a unit with an optional prefix.
let km: Prefixed<LengthUnit> = "km".parse().unwrap();
assert_eq!(km, Prefixed { prefix: Some(Prefix::Kilo), unit: LengthUnit::Metre });
println!("{:.1} {km} ({km:#})", km.value(d));           // 180.0 km (kilometres)

// The same `Unit` trait is what generic code (`U: Unit<Q, N>`) and `&dyn Unit<Q, N>` use.
fn total<U: Unit<Mass<f64>, f64>>(unit: U, masses: &[Mass<f64>]) -> f64 {
    masses.iter().map(|&m| unit.value(m)).sum()
}
let pounds = total(Pound {}, &[mass, m]);               // 164.32...
```

### Temperatures

`ThermodynamicTemperature` is an amount measured from absolute zero, with the `Absolute` label; `TemperatureInterval` is a difference, a *step*: an ordinary amount. Each has its own units, in `thermal::units`: the temperatures are read in `KelvinScale`, `Celsius`, `Fahrenheit` and `Rankine` - each a size and a zero (°C: 1 K from 273.15 K) - and the steps in `Kelvin`, `DegreeCelsius`, `DegreeFahrenheit` and `DegreeRankine`. `ThermodynamicTemperatureUnit` and `TemperatureIntervalUnit` name them, in any number type.

```rust
use yadc_si::dynamic::thermal::ThermodynamicTemperatureUnit;
use yadc_si::thermal::units::{Celsius, DegreeFahrenheit, Fahrenheit, KelvinScale};
use yadc_si::thermal::*;

let t = Celsius {}.quantity(25.0_f64);                  // 298.15 K
let f = Fahrenheit {}.value(t);                         // 77.0, to rounding
let k = KelvinScale {}.value(t);                        // 298.15
let c = ThermodynamicTemperatureUnit::Celsius.value(t); // 25.0: the enum names the same units
let rise: TemperatureInterval<f64> = t - Celsius {}.quantity(20.0); // T - T = ΔT
let hotter: ThermodynamicTemperature<f64> = t + rise;   // T + ΔT = T
let step = DegreeFahrenheit {}.value(rise);             // 9.0
let mean: ThermodynamicTemperature<f64> = (t + hotter) / 2.0; // an average is a temperature
// rise + t doesn't compile ("cannot add these in this order": write t + rise),
// and Celsius {}.value(rise) doesn't either: a unit of temperatures reads a temperature, not a step.
```

Absolute zero is a fact, so a temperature scales and multiplies as any amount does - in kelvins, whichever unit built it - and the label drops out of a product:

```rust
use yadc::PowiAs;
use yadc_si::chemistry::units::{MolarGasConstant, Mole};
use yadc_si::thermal::units::StefanBoltzmannConstant;

let amount = Mole {}.quantity(2.0_f64);
let r = MolarGasConstant {}.quantity(1.0_f64);
let energy: Energy<f64> = amount * r * t;               // n R T: joules
let twice: ThermodynamicTemperature<f64> = 2.0 * t;     // 596.3 K
let flux = StefanBoltzmannConstant {}.quantity(1.0_f64) * t.powi::<4>(); // σT⁴: W/m²
```

### Levels: dB, dBm, Np

Decibels count on a logarithmic line, where multiplying is adding. Both quantities on it have the `Log` kind, whose `+` and `-` add and subtract the exponents, and both count in bels. A `Gain` is the logarithm of a plain ratio - no exponents - a quantity of its own with plain units (`Bel`, `Decibel`, `Neper`; `GainUnit`), and a `PowerLevel` the logarithm of a power, with power's exponents: two levels differ by a gain (watts over watts), a gain moves a level, and two levels do not sum to a level (their sum is the logarithm of watts squared). A level is read in a unit with a zero, like a temperature: `DecibelWatt` counts decibels from one watt, `DecibelMilliwatt` decibels from one milliwatt (a zero of −3 B); `PowerLevelUnit` names them.

The logarithm itself is in one place, the hand-written [`level.rs`](src/level.rs): std `From` between `PowerLevel` and `Power`, and between `Gain` and a plain ratio, generic over any `N: Real + const From<Exact>` (`num_traits::Real`). A unit of levels measures levels, and the watts are one `From` away.

```rust
use yadc_si::dimensionless::Dimensionless;
use yadc_si::dynamic::mechanical::PowerLevelUnit;

let sent = DecibelMilliwatt {}.quantity(30.0_f64);      // a PowerLevel: 30 dB above 1 mW
let p = Power::from(sent);                              // 1 W
let w = Watt {}.value(p);                               // 1.0
let dbw = DecibelWatt {}.value(PowerLevel::from(p));    // 0.0
let same = PowerLevelUnit::DecibelMilliwatt.value(sent); // 30.0

let received = sent - Decibel {}.quantity(3.0) + Decibel {}.quantity(13.0); // a link budget: plain + and -
let gain: Gain<f64> = received - sent;                  // 10 dB
let ratio = Dimensionless::from(gain);                  // 10.0
let watts = Power::from(received);                      // 10 W
// sent + sent doesn't compile ("cannot add: the labels have no sum"), nor does 2.0 * sent.
```

A gain scales (`3.0 * gain`) and, like a label, leaves a product: an attenuation in dB per kilometre is a plain per-length number, and over a length it is a number of bels again.

```rust
let kilometre = Metre {}.quantity(1000.0_f64);
let attenuation = Decibel {}.quantity(0.2) / kilometre; // a plain per-length number, counting bels
let loss: Gain<f64> = Bel {}.quantity(f64::from(attenuation * (15.0 * kilometre))); // 3 dB
let after = sent - loss;                                // 27 dBm
```

### Instants

An `Instant` is a point in time - the affine kind. There is one line of instants, counted in uniform seconds (leap seconds are not modelled) and stored from the Unix epoch: that is what an instant prints and serializes as. Two instants differ by a `Time`, a `Time` moves an instant, and nothing else: an instant has no sum with another, no scaling and no products. Which zero an instant was read from is not in its type: the units `UnixSecond`, `UnixMillisecond`, `GpsSecond`, `JulianDate` and `ModifiedJulianDate` (`InstantUnit` names them) each have a size and a zero, and all read the same line.

```rust
/// Seconds since a moment of your choosing: a unit with a zero known at run time.
struct SecondsSince(Instant<f64>);
impl Unit<Instant<f64>, f64> for SecondsSince {
    fn quantity(&self, n: f64) -> Instant<f64> { self.0 + Second {}.quantity(n) }
    fn value(&self, t: Instant<f64>) -> f64 { Second {}.value(t - self.0) }
}

let launch = UnixSecond {}.quantity(1_700_000_000.0_f64); // an Instant
let landing = launch + Hour {}.quantity(2.0);           // instant + time = instant
let flight: Time<f64> = landing - launch;               // instant - instant = time
let gps = GpsSecond {}.value(launch);                   // 1_384_035_200.0: another zero, same line
let jd = JulianDate {}.value(launch);                   // 2_460_263.42...
let j2000 = JulianDate {}.quantity(2_451_545.0);        // built in one unit,
let unix = UnixSecond {}.value(j2000);                  // read in another: 946_728_000.0
let elapsed = SecondsSince(launch).value(landing);      // 7200.0
// launch + landing, 2.0 * launch and launch * flight don't compile ("cannot add a point to a
// point", "a point cannot be scaled", "cannot multiply: these have no product").
```

An `f64` of seconds from 1970 resolves today's instants to a fraction of a microsecond.

### Dynamic quantities and units

When the dimension is only known at run time, `DynamicSIQuantity` carries it alongside the value. It uses the same operators and is converted with `From` / `TryFrom`: build a static quantity with a unit and convert it in, convert it back out to a static quantity - checked - and read it with a unit. Every operator returns an `Option`: a sum of two dimensions, or a product with a point or a level, has no result.

```rust
use yadc::PartialRoot;
use yadc_si::dynamic::{DynamicSIQuantity, DynamicSIUnit};

let h: Time<f64> = Hour {}.quantity(2.0);
let q = (DynamicSIQuantity::from(d) / DynamicSIQuantity::from(h)).unwrap();
let v: Velocity<f64> = q.try_into().unwrap();            // Err(q) on a mismatch
let kmh = KilometrePerHour {}.value(v);                  // 90.0
assert_eq!(q + DynamicSIQuantity::from(h), None);        // km/h + h: no result
assert_eq!(Mass::<f64>::try_from(q), Err(q));            // hands `q` back
let side = DynamicSIQuantity::from(SquareMetre {}.quantity(9.0_f64)).partial_root(2); // Some(3 m)
let one = f64::try_from((q / q).unwrap());               // Ok(1.0): a plain number comes out as `f64`
let level = DynamicSIQuantity::from(DecibelWatt {}.quantity(0.0_f64)); // a logarithm
assert_eq!(level * q, None);                             // a level has no products
assert_eq!(level * 2.0, None);                           // nor can it be scaled
let t0 = DynamicSIQuantity::from(UnixSecond {}.quantity(0.0_f64)); // a point
assert_eq!(t0 + t0, None);                               // two points have no sum
assert_eq!(t0 * 2.0, None);                              // a point cannot be scaled
```

A unit of a dimension known only at run time is a `DynamicSIUnit`, made from a unit type or a unit enum's value. It reads and builds `Option`s, as dynamic results come:

```rust
let unit = DynamicSIUnit::from(VelocityUnit::KilometrePerHour);
let read = unit.value(Some(q));                          // Some(90.0)
let none = unit.value(Some(level));                      // None: another dimension
let built = unit.quantity(Some(50.0));                   // Some(50 km/h)
let back = KilometrePerHour::try_from(unit);             // Ok: the checked way back
println!("{unit}");                                      // km/h
```

For a unit picked at run time among the units of one quantity, `&dyn Unit<Length<f64>, f64>` is some unit of length, checked at compile time:

```rust
let lengths: [&dyn Unit<Length<f64>, f64>; 2] = [&km, &LengthUnit::Mile]; // km: a `Prefixed<LengthUnit>`
for u in lengths {
    println!("{:.1}", u.value(d));                       // 180.0, 111.8
}
```

Some unit of *some* quantity, with a static quantity for each, is a set you list yourself: an enum of the quantities your program measures and an enum of their unit enums, visited with `dynamickit`'s `Visit` (each unit family names the quantity it measures in an associated type of yours; one visitor builds, one reads, each written once for every family) and listed for a menu with `Variants`; `dynamickit`'s `Members` and `Via` convert the set through `DynamicSIQuantity` and borrow its arithmetic. See [`examples/`](examples/).

Units are not tied to this crate's quantities. Another crate may make them units of its own quantity - `impl Unit<my::Length, f64> for yadc_si::dynamic::mechanical::LengthUnit` is allowed by the orphan rule - and write a unit of its own by hand: `impl Unit<Length<f64>, f64>` on a type of its own.

See [`yadc/examples/basic_use.rs`](../yadc/examples/basic_use.rs) for a tour.

### Units

Every family module has a generated `units` module: one type per unit, an `SIUnit` with the quantity's dimension and the unit's definition, its size and zero exact, as a standards document writes them.

```rust
pub type Inch = SIUnit<LENGTH, { SIUnitConsts { size: Exact::dec(254, -4), zero: Exact::ZERO, symbol: "in", name: "inches" } }>;
pub type Degree = SIUnit<ANGLE, { SIUnitConsts { size: Exact::PI / Exact::int(180), zero: Exact::ZERO, symbol: "°", name: "degrees" } }>;
pub type Celsius = SIUnit<
    THERMODYNAMIC_TEMPERATURE,
    { SIUnitConsts { size: Exact::int(1), zero: Exact::dec(27315, -2), symbol: "°C", name: "degrees Celsius" } },
>;
```

- **Rounding happens once per unit and number type, at compile time.** `SIUnit`'s `impl Unit` converts the size and the zero to `N` with std `From<Exact>` in a `const { }` block. A float is the nearest one, rounded once (`f32` straight from the exact value, not through `f64`; a power of π rounds once more); an integer is the nearest, ties to even, and a compile error where it does not fit.
- **Any number type.** A unit counts in every `N: const From<Exact>` with `+ - * /`: `f64`, `f32`, any primitive integer, or a number type of yours that implements `const From<Exact>`. An integer quantity counts whole coherent units (`Metre {}.quantity(1609_i32)`), so integers suit units whose size is a whole number of coherent units.
- **Unit enums are generic over their number type.** Each lives in `dynamic::<family>` (`dynamic::mechanical::LengthUnit`), each variant naming its unit type (`#[unit(Foot)] Foot`), with `#[unit(of = Length<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + ..>)]` - the bounds `SIUnit`'s impl needs. Its `Display` and `FromStr` are its unit types', and `From<LengthUnit> for DynamicSIUnit` makes any of its values a dynamic unit.
- **Prefixes are a run-time choice on a unit.** A unit whose symbol is a prefix on another unit's - the kilometre, the millipascal, the kilowatt hour - has no type: it is `Prefixed { prefix: Some(Prefix::Kilo), unit: LengthUnit::Metre }`, and `Prefixed<LengthUnit>` parses `km`. A unit with a prefix on one factor of its symbol - km/h, kW/°C - is a type of its own (`KilometrePerHour`). A unit keeps its prefixed name when it is coherent (`Kilogram`) and when its prefix scales no unit of its quantity (`MillimetreOfMercury`: there is no metre of mercury).
- **Parsing a prefix.** `"km".parse::<Prefixed<LengthUnit>>()` reads a unit's own symbol first - `Pa` is the pascal, `min` the minute, `M` the nautical mile - then a leading prefix on one of the enum's symbols: `km`, `mPa`, and `µm`, `μm` or `um` for micro. It is `Err(yadc::UnknownUnit)` when neither reads.
- **Physical constants are units**, as in UCUM: each is a unit of the quantity it measures (exact, or CODATA 2018), so a quantity reads in it.

  | Unit | Quantity |
  |---|---|
  | `mechanical::units::SpeedOfLight` (c) | `Velocity` |
  | `mechanical::units::PlanckConstant` (h) | `Action` |
  | `angular::units::ReducedPlanckConstant` (ħ) | `AngularMomentum` (J·s/rad, so `h = ħ · 2π rad`) |
  | `mechanical::units::StandardGravity` (g₀) | `Acceleration` |
  | `mechanical::units::GravitationalConstant` (G) | `SpecificGravitationalParameter` (m³/(kg·s²)) |
  | `mechanical::units::ElectronMass` (mₑ), `ProtonMass` (mₚ) | `Mass` |
  | `mechanical::units::StandardAtmosphere` (atm) | `Pressure` |
  | `electromagnetic::units::ElementaryCharge` (e) | `ElectricCharge` |
  | `electromagnetic::units::VacuumPermittivity` (ε₀) | `ElectricPermittivity` |
  | `electromagnetic::units::VacuumPermeability` (µ₀) | `MagneticPermeability` |
  | `thermal::units::BoltzmannConstant` (k) | `HeatCapacity` (per kelvin of *step*, so `k` times a `TemperatureInterval` is an `Energy`) |
  | `thermal::units::StefanBoltzmannConstant` (σ) | `StefanBoltzmannCoefficient` (W/(m²·K⁴)) |
  | `chemistry::units::AvogadroConstant` (N_A) | `InverseAmountOfSubstance` (mol⁻¹) |
  | `chemistry::units::MolarGasConstant` (R) | `MolarHeatCapacity` |

  Each is also a variant of its quantity's unit enum (`VelocityUnit::SpeedOfLight`).

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

- `serde` (off by default; turns on `yadc/ucum`) — `SIQuantity` serializes transparently as its number (the dimension comes from the target type), and `DynamicSIQuantity` as `{ dim, value }` with the dimension as a UCUM string (`"m.kg.s-2"`, `"K{Absolute}"`, `"s{Affine}"`, `"rad"`, `"rad2"`, `"1{Log}"`, `"1"`).
- `uom` (off by default) — `From` / `TryFrom` impls bridging `SIQuantity` / `DynamicSIQuantity` and [`uom`](https://docs.rs/uom)'s typenum-based quantities (uom's angle kinds become `rad` exponents, its temperature kind the `Absolute` label).

## Regenerating

Everything in `src/` except `level.rs` and `uom_bridge.rs` is generated by [`yadc-codegen`](../yadc-codegen/) from [`yadc-codegen/si.toml`](../yadc-codegen/si.toml). To regenerate, from the workspace root:

```sh
cargo run -p yadc-codegen -- yadc-codegen/si.toml -o <dir>
```

then copy `<dir>/src/*.rs` and `<dir>/src/dynamic/*.rs` over `yadc-si/src/` and `yadc-si/src/dynamic/` (never generate straight into `yadc-si`: the generated `Cargo.toml` is a starting point, not this crate's). Keep the hand-written `src/level.rs` (the logarithms between a level and a power, a gain and a ratio) and `src/uom_bridge.rs` (orphan-rule impls that must sit beside `SIQuantity`) - `si.toml` lists them under `extra_modules`, so the generated `lib.rs` declares them but the generator never writes them.

## Origin of the unit data

Symbols, labels and measured conversion factors are derived from the [`iliekturtles/uom`](https://github.com/iliekturtles/uom) project (MIT/Apache-2.0); a unit with an exact definition (inch, pound, standard gravity, survey foot, ...) is written from it (see `yadc-codegen/si.toml`).

## License

MIT OR BSD-3-Clause.
