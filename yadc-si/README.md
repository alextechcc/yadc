# yadc-si

An SI unit system built with [`yadc`](../yadc/). It is not part of yadc's API - it is one example of a system *you* could own, generated from [`yadc-codegen/si.toml`](../yadc-codegen/si.toml). Every modelling choice here (radians as an exponent axis, absolute temperature as an `Absolute` label, an instant as a point, decibels on a logarithmic line) is this crate's, not yadc's.

What it defines, in `lib.rs`:

- `Kind` - how a quantity behaves under `+` and `-`, beside its exponents: `Linear` (`#[default]`: an ordinary amount, which adds to its own kind, scales and multiplies), `Absolute` (a temperature: an amount with a true zero: it scales and multiplies as an amount does, but two of them differ by a linear amount and a linear amount moves one; a label marked `#[kind(absolute)]`), `Affine` (a point, measured from a zero somebody chose - an instant: two differ by a linear amount, an amount moves one, and it has no sum with another, no scaling and no products) and `Log` (a logarithm - a level or a gain: adding logarithms multiplies what they are the logarithms of, so `+` and `-` add and subtract the exponents). The last two are marked on their variants, `#[kind(affine)]` and `#[kind(log)]`.
- `ISQ` - the dimension: one `i8` exponent per SI base unit (`m`, `kg`, `s`, `A`, `K`, `mol`, `cd`) and one for the radian (`rad`), plus the kind. Angle is `rad: 1`, solid angle `rad: 2` (sr = rad²), angular velocity rad/s; torque is J/rad and moment of inertia kg·m²/rad², so torque × angle is energy and an angular speed never mixes with a frequency. Its `*` and `/` are partial (`Option`): a product with a point or a level has no result. It prints as `m·kg·s⁻²`, a kind that is not `Linear` in braces (`K{Absolute}`).
- `SIQuantity<const D: ISQ, N>(N)` - the compile-time-checked quantity (`#[derive(Quantity)]`), stored in coherent SI units (kilograms, metres, kelvin, radians, watts, ...), counted in the number type `N`. Each quantity's alias takes the number type, and every use says which, or stays generic over it: `pub type Length<N> = SIQuantity<{ ISQ { m: 1, ..ISQ::DIMENSIONLESS } }, N>`, used as `Length<f64>`, `Length<f32>`, `Length<i32>`, `Length<Exact>` (every constant) or `fn f<N>(d: Length<N>)`. Its field is private to this crate: the base constants are the only raw numbers, and everyone else builds and reads through units. It prints as its number and dimension (`9.81 m·s⁻²`).
- `dynamic::DynamicSIQuantity` - the same with the dimension carried at run time, as `dim: ISQ`, built with [`dynamickit`](../dynamickit/)'s `#[derive(Dynamic)]`: `SIQuantity<const D: ISQ, f64> => { dim: D, value: .0 }` (converting moves the number in and records `D`; `TryFrom` checks it on the way back, handing the value back on a mismatch), and `shared f64 => { dim: ISQ::DIMENSIONLESS, value: self }` (a bare number is the same value as a dimensionless quantity). Its operators and methods are every field's own, each with its output stated: `*`, `/`, `+`, `-`, scaling by an `f64`, `partial_powi` and `partial_root` return an `Option` (`None` where the dimensions don't combine: a sum of two dimensions, a product with a point or a level, a scaled point or level), so a calculation reads `((a * b)? + c)?`; values of one dimension are ordered. No impl on it is hand-written.
- `base` - the base constants, the only raw numbers in the crate: `ONE`, `METRE`, `KILOGRAM`, `SECOND`, `AMPERE`, `KELVIN` (a step: a temperature difference of 1 K), `MOLE`, `CANDELA`, `RADIAN`, `ABSOLUTE_ZERO` (a temperature), `UNIX_EPOCH` (the base zero of `Instant`: what an instant is stored, printed and serialized from), `BEL` (a gain of ten times) and `WATT_LEVEL` (the level of one watt).
- `Exact` - the exact number every constant is counted in: `num/den · 10ⁿ · πᵏ`, with `const` arithmetic. An `Exact` scales an exact quantity from the right (`INCH * Exact::int(12)`), and std `From` rounds one to `f64`, `f32`, any primitive integer or a number type of yours - a number, a quantity or a scale (see [Constants](#constants)).
- The SI prefixes, exact dimensionless quantities (`KILO: Dimensionless<Exact>`): `YOTTA`, `ZETTA`, `EXA`, `PETA`, `TERA`, `GIGA`, `MEGA`, `KILO`, `HECTO`, `DECA`, `DECI`, `CENTI`, `MILLI`, `MICRO`, `NANO`, `PICO`, `FEMTO`, `ATTO`, `ZEPTO`, `YOCTO`. A prefix times a unit is the prefixed unit: `KILO * METRE` is a kilometre, `MILLI * WATT` a milliwatt, `KILO * WATT_HOUR` a kilowatt hour.
- `Prefix` - the same 20 as an enum (`Prefix::Kilo`), with `symbol()` (`k`), `name()` (`kilo`) and `factor()` (`KILO`); and `Prefixed<U>` - a unit enum's value with an optional prefix, `Prefixed { prefix: Some(Prefix::Kilo), unit: LengthUnit::Metre }`: a unit of what `U` measures, which prints `km` (`{:#}`: `kilometres`) and parses back.
- `Scale<P, S>(pub P, pub S)` - a scale: a zero and a step. A unit of its zero's type - `Scale(ICE_POINT, KELVIN)` counts kelvins from the ice point - made as a constant or at run time.

The crate root re-exports `yadc`'s `Dimension`, `Quantity` and `Unit`, and holds the prefixes, `Prefix` and `Prefixed`. Its one point is the `Instant`; a system with altitudes or positions gives them the affine kind the same way (see [`yadc-tests/src/lib.rs`](../yadc-tests/src/lib.rs)).

and ~120 quantity aliases (`pub type Length<N> = SIQuantity<{ ISQ { m: 1, ..ISQ::DIMENSIONLESS } }, N>;`), each with a `<Q>Unit` enum naming its units in any number type the constants convert to (`#[unit(of = Length<N>, number = N)]`, `LengthUnit::Foot.quantity(3.0)`; `PowerUnit` holds W, dBW and dBm and counts in `f64`, as its logarithm does; a prefixed unit, kW, is `Prefixed<PowerUnit>`), organized into 9 feature-gated families:

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

`Dimensionless<N>` (`SIQuantity<{ ISQ::DIMENSIONLESS }, N>`) with `DimensionlessUnit` (ratio, %, ‰, ppm, ...) is always present. All families are on by default; disable selectively with `default-features = false` plus an explicit feature list to reduce compile time.

## Nightly features

`yadc-si` defines a unit system, so its own crate root carries
`generic_const_exprs`, `min_adt_const_params`, `const_trait_impl`,
`const_ops`, `const_cmp`, `const_default`, `const_convert`, `const_try`,
`const_destruct` and `derive_const` (plus `allow(incomplete_features)`): the
derives write the quantity operators, the static ↔ dynamic bridge and the
units' `Unit` as `const impl`s. A crate that only *uses* the aliases and units
exported here needs just:

```rust
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
```

and, to convert constants in its own `const` items (`const MI: Length<f64> =
MILE.into();`, below), also `const_trait_impl` and `const_convert`; to do
quantity arithmetic in them, `const_trait_impl` and `const_ops`. A crate that derives `Unit` on an enum of
its own adds `const_trait_impl`.

## Usage

A unit turns a number into a quantity and a quantity back into a number, through the `Unit` trait: `quantity` builds, `value` reads. Every quantity constant is a unit (`POUND.quantity(10.0)`, `POUND.value(m)`), and scaling one by a number builds too (`10.0 * POUND`); so is a unit enum's value, and a scale (`CELSIUS.quantity(25.0)`, `CELSIUS.value(t)`). Maths is on quantities.

The constants are exact (`POUND: Mass<Exact>`); a program converts the ones it uses, once, to the number type of its values - with std `From`, in a `const`, so the compiler does the rounding - and uses those as its units:

```rust
use yadc::Unit;
use yadc_si::mechanical::consts as si;
use yadc_si::mechanical::*;
use yadc_si::{KILO, Prefix, Prefixed};

const KILOGRAM: Mass<f64> = si::KILOGRAM.into();
const POUND: Mass<f64> = si::POUND.into();             // the nearest f64 to 0.45359237 kg
const NEWTON: Force<f64> = si::NEWTON.into();
const METRE_PER_SECOND_SQUARED: Acceleration<f64> = si::METRE_PER_SECOND_SQUARED.into();
const KM: Length<f64> = (KILO * si::METRE).into();      // a prefix times a unit: the kilometre
const HOUR: Time<f64> = si::HOUR.into();
const KMH: Velocity<f64> = (KILO * si::METRE / si::HOUR).into(); // exact until this one rounding

let mass = KILOGRAM.quantity(70.0);                     // a unit builds
let force: Force<f64> = mass * (9.81 * METRE_PER_SECOND_SQUARED); // ... and so does a constant times a number
let newtons = NEWTON.value(force);                      // ... and reads: 686.7
let lb = POUND.value(mass);                             // 154.32...
let v: Velocity<f64> = 90.0 * KM / HOUR;
let kmh = KMH.value(v);                                 // 90.0
let ratio = f64::from(force / force);                   // Dimensionless is a plain number
println!("{force:.1}");                                 // 686.7 m·kg·s⁻²: a quantity prints itself

// A unit enum names a family's units, for a menu or a parser: std `Display` and
// `FromStr` - `{}` is the symbol, `{:#}` the long name. Its values are units too,
// in any number type: `quantity` and `value`.
let n = ForceUnit::Newton;
println!("{:.1} {n}", n.value(force));                  // 686.7 N
println!("{:.1} {n:#}", n.value(force));                // 686.7 newtons
let unit: MassUnit = "lb".parse().unwrap();             // Err(yadc::UnknownUnit) for an unknown symbol
let m: Mass<f64> = unit.quantity(10.0);
let m32: Mass<f32> = unit.quantity(10.0);               // the same unit, in f32

// A prefix before a unit's symbol: `Prefixed<U>`, a unit enum's value with an optional prefix.
let km: Prefixed<LengthUnit> = "km".parse().unwrap();
assert_eq!(km, Prefixed { prefix: Some(Prefix::Kilo), unit: LengthUnit::Metre });
let d: Length<f64> = km.quantity(3.0);                  // 3000 m
println!("{:.1} {km} ({km:#})", km.value(d));           // 3.0 km (kilometres)

// The same `Unit` trait is what generic code (`U: Unit<Q>`) and `&dyn Unit<Q>` use.
let also = POUND.quantity(10.0);
let pounds: Vec<f64> = [mass, also].into_iter().map(|m| POUND.value(m)).collect();
```

### Temperatures

`ThermodynamicTemperature` is an amount measured from absolute zero, with the `Absolute` label; `TemperatureInterval` is a difference, a *step*: an ordinary amount. A step, a zero and a scale are different things with different names: the steps are `KELVIN` and `DEGREE_FAHRENHEIT`; the zeros `ABSOLUTE_ZERO`, `ICE_POINT` (`ABSOLUTE_ZERO + 273.15 * KELVIN`) and `FAHRENHEIT_ZERO`; the scales - steps counted from a zero - are constants too, `Scale(zero, step)`: `KELVIN_SCALE`, `CELSIUS`, `FAHRENHEIT` and `RANKINE` (`pub const CELSIUS: Scale<ThermodynamicTemperature<Exact>, TemperatureInterval<Exact>> = Scale(ICE_POINT, KELVIN);`), exact like every constant and converted with `From` like one: zero and step together. `ThermodynamicTemperatureUnit` names the same scales, in any number type. A scale of your own is another, made at compile time or at run time: `Scale(this_morning, KELVIN)`.

```rust
use yadc_si::Scale;
use yadc_si::thermal::consts as si;
use yadc_si::thermal::*;

type TemperatureScale = Scale<ThermodynamicTemperature<f64>, TemperatureInterval<f64>>;
const CELSIUS: TemperatureScale = si::CELSIUS.into();
const FAHRENHEIT: TemperatureScale = si::FAHRENHEIT.into();
const KELVIN_SCALE: TemperatureScale = si::KELVIN_SCALE.into();
const KELVIN: TemperatureInterval<f64> = si::KELVIN.into();
const DEGREE_FAHRENHEIT: TemperatureInterval<f64> = si::DEGREE_FAHRENHEIT.into();

let t = CELSIUS.quantity(25.0);                         // 298.15 K
let f = FAHRENHEIT.value(t);                            // 77.0, to rounding
let k = KELVIN_SCALE.value(t);                          // 298.15
let c = ThermodynamicTemperatureUnit::Celsius.value(t); // 25.0: the enum names the same units
let rise: TemperatureInterval<f64> = t - CELSIUS.quantity(20.0); // T - T = ΔT
let hotter: ThermodynamicTemperature<f64> = t + rise;   // T + ΔT = T
let step = DEGREE_FAHRENHEIT.value(rise);               // 9.0
let mean: ThermodynamicTemperature<f64> = (t + hotter) / 2.0; // an average is a temperature
let since_dawn = Scale(CELSIUS.quantity(12.0), KELVIN); // a scale made at run time
let warmed = since_dawn.value(t);                       // 13.0
// rise + t doesn't compile ("cannot add these in this order": write t + rise),
// and CELSIUS.value(rise) doesn't either: a scale with a zero reads a temperature, not a step.
```

Absolute zero is a fact, so a temperature scales and multiplies as any amount does - in kelvins, whatever scale built it - and the label drops out of a product:

```rust
use yadc::PowiAs;
use yadc_si::chemistry::{consts as chem, AmountOfSubstance, MolarHeatCapacity};

const MOLE: AmountOfSubstance<f64> = chem::MOLE.into();
const MOLAR_GAS_CONSTANT: MolarHeatCapacity<f64> = chem::MOLAR_GAS_CONSTANT.into();
const STEFAN_BOLTZMANN_CONSTANT: StefanBoltzmannCoefficient<f64> = si::STEFAN_BOLTZMANN_CONSTANT.into();

let energy = 2.0 * MOLE * MOLAR_GAS_CONSTANT * t;        // n R T: joules
let twice: ThermodynamicTemperature<f64> = 2.0 * t;      // 596.3 K
let flux = STEFAN_BOLTZMANN_CONSTANT * t.powi::<4>();    // σT⁴: W/m²
```

### Levels: dB, dBm, Np

Decibels count on a logarithmic line, where multiplying is adding. Both quantities on it have the `Log` kind, whose `+` and `-` add and subtract the exponents. A `Gain` is the logarithm of a plain ratio - no exponents - a quantity of its own with plain units (`BEL`, `DECIBEL`, `NEPER`; `GainUnit`), and a `PowerLevel` the logarithm of a power, with power's exponents: two levels differ by a gain (watts over watts), a gain moves a level, and two levels do not sum to a level (their sum is the logarithm of watts squared). A level is read on a scale like a temperature, a zero and a step: `DBW` is `Scale(WATT_LEVEL, DECIBEL)`, `DBM` is `Scale(MILLIWATT_LEVEL, DECIBEL)`.

The logarithm itself is in one place, the hand-written [`level.rs`](src/level.rs): std `From` between `PowerLevel` and `Power`, and between `Gain` and a plain ratio. A scale of levels measures levels, and the watts are one `From` away. `PowerUnit` names dBW and dBm beside the watts - each `by` the level's scale, `via = PowerLevel` - so its values take a decibel count straight to watts and back; the conversion is not `const`, so neither is that enum's.

```rust
use yadc_si::dimensionless::Dimensionless;
use yadc_si::mechanical::consts as si;

type LevelScale = Scale<PowerLevel<f64>, Gain<f64>>;
const DBM: LevelScale = si::DBM.into();
const DBW: LevelScale = si::DBW.into();
const WATT: Power<f64> = si::WATT.into();
const BEL: Gain<f64> = si::BEL.into();
const DECIBEL: Gain<f64> = si::DECIBEL.into();

let sent = DBM.quantity(30.0);                          // a PowerLevel: 30 dB above 1 mW
let p = Power::<f64>::from(sent);                       // 1 W
let w = WATT.value(p);                                  // 1.0
let dbw = DBW.value(PowerLevel::<f64>::from(p));        // 0.0
let dbm = PowerUnit::DecibelMilliwatt;                  // in the watts' enum (f64): both steps at once
let same = dbm.value(p);                                // 30.0
let one_watt: Power<f64> = dbm.quantity(30.0);

let received = sent - 3.0 * DECIBEL + 13.0 * DECIBEL;   // a link budget: plain + and -
let gain: Gain<f64> = received - sent;                  // 10 dB
let ratio = Dimensionless::<f64>::from(gain);           // 10.0
let watts = Power::<f64>::from(received);               // 10 W
// sent + sent doesn't compile ("cannot add: the labels have no sum"), nor does 2.0 * sent.
```

A gain scales (`3.0 * DECIBEL`) and, like a label, leaves a product: an attenuation in dB per kilometre is a plain per-length number, and over a length it is a number of bels again.

```rust
let attenuation = 0.2 * DECIBEL / KM;                   // per metre, counting bels
let loss: Gain<f64> = (attenuation * (15.0 * KM)) * BEL; // 3 dB
let after = sent - loss;                                // 27 dBm
```

### Instants

An `Instant` is a point in time - the affine kind. There is one line of instants, counted in uniform seconds (leap seconds are not modelled) and stored from `UNIX_EPOCH`, its base zero: that is what an instant prints and serializes as. Two instants differ by a `Time`, a `Time` moves an instant, and nothing else: an instant has no sum with another, no scaling and no products. Which zero an instant was read from is not in its type: the other zeros - `GPS_EPOCH`, `JULIAN_EPOCH`, `MODIFIED_JULIAN_EPOCH` - are constants on the same line, and the scales `UNIX_SECONDS`, `UNIX_MILLISECONDS`, `GPS_SECONDS`, `JULIAN_DATE` and `MODIFIED_JULIAN_DATE` (`InstantUnit` names them) all read it.

```rust
type InstantScale = Scale<Instant<f64>, Time<f64>>;
const UNIX_SECONDS: InstantScale = si::UNIX_SECONDS.into();
const GPS_SECONDS: InstantScale = si::GPS_SECONDS.into();
const JULIAN_DATE: InstantScale = si::JULIAN_DATE.into();
const SECOND: Time<f64> = si::SECOND.into();

let launch = UNIX_SECONDS.quantity(1_700_000_000.0);    // an Instant
let landing = launch + 2.0 * HOUR;                      // instant + time = instant
let flight: Time<f64> = landing - launch;               // instant - instant = time
let gps = GPS_SECONDS.value(launch);                    // 1_384_035_200.0: another zero, same line
let jd = JULIAN_DATE.value(launch);                     // 2_460_263.42...
let j2000 = JULIAN_DATE.quantity(2_451_545.0);          // built on one scale,
let unix = UNIX_SECONDS.value(j2000);                   // read on another: 946_728_000.0
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
use yadc_si::dynamic::DynamicSIQuantity;

const SQUARE_METRE: Area<f64> = si::SQUARE_METRE.into();
const WATT_LEVEL: PowerLevel<f64> = si::WATT_LEVEL.into();

let d = (DynamicSIQuantity::from(90.0 * KM) / DynamicSIQuantity::from(2.0 * HOUR)).unwrap();
let v: Velocity<f64> = d.try_into().unwrap();            // Err(d) on a mismatch
let kmh = KMH.value(v);                                  // 45.0
assert_eq!(d + DynamicSIQuantity::from(HOUR), None);     // km/h + h: no result
assert_eq!(Mass::<f64>::try_from(d), Err(d));            // hands `d` back
let side = DynamicSIQuantity::from(9.0 * SQUARE_METRE).partial_root(2); // Some(3 m)
let n = f64::try_from((d / d).unwrap());                 // Ok(1.0): a plain number comes out as `f64`
let level = DynamicSIQuantity::from(WATT_LEVEL);        // a logarithm
assert_eq!(level * d, None);                             // a level has no products
assert_eq!(level * 2.0, None);                           // nor can it be scaled
let t0 = DynamicSIQuantity::from(UNIX_SECONDS.quantity(0.0)); // a point
assert_eq!(t0 + t0, None);                               // two points have no sum
assert_eq!(t0 * 2.0, None);                              // a point cannot be scaled
```

For a unit picked at run time, `&dyn Unit<Length<f64>>` is some unit of length, checked at compile time:

```rust
let lengths: [&dyn Unit<Length<f64>>; 2] = [&km, &LengthUnit::Mile]; // km: a `Prefixed<LengthUnit>`
for u in lengths {
    println!("{:.1}", u.value(v * HOUR));                // 45.0, 28.0
}
```

Some unit of *some* quantity is a set you list yourself: an enum of the quantities your program measures and an enum of their unit enums, visited with `dynamickit`'s `Visit` (each unit family names the quantity it measures in an associated type of yours; one visitor builds, one reads, each written once for every family) and listed for a menu with `Variants`; `dynamickit`'s `Members` and `Via` convert the set through `DynamicSIQuantity` and borrow its arithmetic. See [`examples/`](examples/).

Units are not tied to this crate's quantities. Another crate may make them units of its own quantity - `impl Unit<my::Length> for yadc_si::mechanical::LengthUnit` is allowed by the orphan rule - and write a unit of its own by hand: `impl Unit<Length<f64>>` on a type of its own.

See [`yadc/examples/basic_use.rs`](../yadc/examples/basic_use.rs) for a tour.

### Constants

Every family module has a generated `consts` module: the base constants of its quantities (re-exported from `base`: `mechanical::consts::METRE`), then one constant per named unit, each defined from them - a coherent unit as their product, everything else as a multiple of it, as a standards document writes it. Every constant is *exact*: its number is an `Exact`, and its type says so.

```rust
pub const METRE: SIQuantity<{ ISQ { m: 1, ..ISQ::DIMENSIONLESS } }, Exact> = SIQuantity(Exact::int(1)); // in `base`
pub const NEWTON: Force<Exact> = METRE * KILOGRAM / (SECOND * SECOND);
pub const INCH: Length<Exact> = METRE * Exact::dec(254, -4);
pub const ICE_POINT: ThermodynamicTemperature<Exact> = ABSOLUTE_ZERO + KELVIN * Exact::dec(27315, -2);
pub const KILO: Dimensionless<Exact> = SIQuantity(Prefix::Kilo.factor()); // a prefix
pub const CELSIUS: Scale<ThermodynamicTemperature<Exact>, TemperatureInterval<Exact>> = Scale(ICE_POINT, KELVIN);
```

- **Constants compose exactly, at compile time.** Quantity arithmetic on `Exact` is `const`, so an expression of constants is another exact constant: `KILO * METRE` (a dimensionless quantity times a length), `INCH * Exact::int(12)` (a bare exact factor, on the right), `MILE / HOUR`, `SLUG / CUBIC_FOOT`, `POUND * STANDARD_GRAVITY / (INCH * INCH)`; `INCH * INCH * INCH == CUBIC_INCH` and `MILE / HOUR == MILE_PER_HOUR` hold exactly.
- **Rounding happens where you write a conversion**, with std `From` / `Into`: from an exact quantity or scale to the same one counted in `f64`, `f32`, any primitive integer or a number type of yours (`let c: Scale<ThermodynamicTemperature<f64>, TemperatureInterval<f64>> = CELSIUS.into();`). A float is the nearest one, rounded once (`f32` straight from the exact value, not through `f64`; a power of π rounds once more); an integer is the nearest, ties to even, and a panic where it does not fit. In a `const` the compiler does all of it, so the program holds one number and does no floating point at run time - on an embedded target, `const STEP: Length<i32> = (KILO * METRE).into();` is the integer 1000, and an integer out of range is a compile error. An integer quantity counts whole coherent units, so `Length::<i32>::from(INCH)` is 0, and `Time::<u32>::from(MILLI * SECOND)` too. Integers therefore go with converted constants, not unit enums: an enum in `i32` must fit every one of its units, and a light year does not - a compile error.
- **A converted constant is the unit of a program's values.** Convert the units you use once, then build and read with them: `const MI: Length<f64> = MILE.into();`, then `MI.quantity(3.0)` or `3.0 * MI` builds, `MI.value(d)` reads. Inside an expression `.into()` has no target to infer, so name it: `Length::<f64>::from(INCH)`.
- **The conversions are two std `From` impls** in `lib.rs`, one per shape: `From<SIQuantity<D, Exact>> for SIQuantity<D, N>` and the same for a `Scale`, for every `N: num_traits::Num + From<Exact>`. They do not overlap std's `From<T> for T`, because `Exact` is not `Num`. A number type of your own gets `.into()` from every constant, and the unit enums count in it, by implementing `Num` and `From<Exact>`.

```rust
use yadc_si::{KILO, Scale};
use yadc_si::mechanical::consts::*;            // METRE, HOUR, SPEED_OF_LIGHT, ...
use yadc_si::thermal::consts::CELSIUS;

const KMH: Velocity<f64> = (KILO * METRE / HOUR).into();    // km/h: exact until this one rounding
const LIMIT: Velocity<f64> = 50.0 * KMH;                     // 50 km/h
const REST_ENERGY: Energy<f64> = (ELECTRON_MASS * SPEED_OF_LIGHT * SPEED_OF_LIGHT).into(); // mc², 8.187e-14 J
const BOILING: ThermodynamicTemperature<f64> =
    Scale::<ThermodynamicTemperature<f64>, TemperatureInterval<f64>>::from(CELSIUS).quantity(100.0);
const G0: Acceleration<f64> = STANDARD_GRAVITY.into();
const C: Velocity<f64> = SPEED_OF_LIGHT.into();
let weight: Force<f64> = G0 * mass;
let beta = C.value(v);                                       // a constant is a unit: v in units of c
```

- **Unit constants.** A named unit (`#[unit("mi", "miles", by = Length::<N>::from(MILE))]`) has `pub const <SCREAMING_SNAKE variant>: <Quantity><Exact>` in its family's `consts`: `mechanical::consts::MILE`, `POUND`, `PSI`, `KNOT`, `LITRE`, `HORSEPOWER`, `BTU_IT`, `dimensionless::consts::PERCENT`; so do the squares and cubes of the metre, inch, foot, yard and mile, `METRE_PER_SECOND`, `METRE_PER_SECOND_SQUARED` and `MILE_PER_HOUR` - about 380 in all. A name two quantities of one module share is prefixed with the quantity (`TIME_MINUTE`, `ANGLE_MINUTE`).
- **A unit written as others is an expression, not a constant.** The slug per cubic foot is `SLUG / CUBIC_FOOT`, the joule per kilogram `JOULE / KILOGRAM`, the foot-pound `FOOT * POUND_FORCE`, the kilometre per hour `KILO * METRE / HOUR`. Its unit-enum variant is its exact size, converted: `#[unit("km/h", "kilometres per hour", by = Velocity::<N>::from(METRE_PER_SECOND * (Exact::int(1000) / Exact::int(3600))))]`.
- **Unit enums are generic over their number type.** `#[unit(of = Length<N>, number = N)]` with `#[unit(for<N> where N: Num + const From<Exact>, ..)]`: each variant's `by` is the exact constant converted with `From` (`by = Length::<N>::from(INCH)`), and the `Unit` derive evaluates every `by` in a `const { }` block - folded at compile time, once per number type, not converted on every call: `LengthUnit::Foot.quantity(3.0)`, `LengthUnit::Foot.value(d)`. An enum with a unit that measures another type (`via`) counts in `f64`: `PowerUnit`, whose decibels take a logarithm.
- **Prefixes are numbers, not units.** A unit whose symbol starts with an SI prefix on another unit - the kilometre, the millipascal, the kilowatt hour - has no constant and no unit-enum variant: it is `KILO * METRE`, `MILLI * PASCAL`, `KILO * WATT_HOUR`, and `Prefixed<LengthUnit>` parses `km`. A unit with a prefix inside its symbol - km/h, N/mm², kW/°C - is a variant without a constant (`VelocityUnit::KilometrePerHour`). A prefix multiplies a unit, not a number (`50.0 * KILO` doesn't compile: write `50.0 * KM` with a converted `KM`). A unit keeps its own name when it is coherent (`KILOGRAM`), when another definition uses it (`DECIBEL`, in `MILLIWATT_LEVEL`), when it is a scale, and when its prefix scales no unit of its quantity (`MILLIMETRE_OF_MERCURY`: there is no metre of mercury).
- **Parsing a prefix.** `"km".parse::<Prefixed<LengthUnit>>()` reads a unit's own symbol first - `Pa` is the pascal, `min` the minute, `M` the nautical mile - then a leading prefix on one of the enum's symbols: `km`, `mPa`, and `µm`, `μm` or `um` for micro. It is `Err(yadc::UnknownUnit)` when neither reads.
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
  | `thermal::consts::BOLTZMANN_CONSTANT` | `HeatCapacityUnit::BoltzmannConstant` (k) | `HeatCapacity` (per kelvin of *step*, so `k` times a `TemperatureInterval` is an `Energy`) |
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

- `serde` (off by default; turns on `yadc/ucum`) — `SIQuantity` serializes transparently as its number (the dimension comes from the target type), and `DynamicSIQuantity` as `{ dim, value }` with the dimension as a UCUM string (`"m.kg.s-2"`, `"K{Absolute}"`, `"s{Affine}"`, `"rad"`, `"rad2"`, `"1{Log}"`, `"1"`).
- `uom` (off by default) — `From` / `TryFrom` impls bridging `SIQuantity` / `DynamicSIQuantity` and [`uom`](https://docs.rs/uom)'s typenum-based quantities (uom's angle kinds become `rad` exponents, its temperature kind the `Absolute` label).

## Regenerating

Everything in `src/` except `level.rs` and `uom_bridge.rs` is generated by [`yadc-codegen`](../yadc-codegen/) from [`yadc-codegen/si.toml`](../yadc-codegen/si.toml). To regenerate, from the workspace root:

```sh
cargo run -p yadc-codegen -- yadc-codegen/si.toml -o <dir>
```

then copy `<dir>/src/*.rs` over `yadc-si/src/` (never generate straight into `yadc-si`: the generated `Cargo.toml` is a starting point, not this crate's). Keep the hand-written `src/level.rs` (the logarithms between a level and a power, a gain and a ratio) and `src/uom_bridge.rs` (orphan-rule impls that must sit beside `SIQuantity`) - `si.toml` lists them under `extra_modules`, so the generated `lib.rs` declares them but the generator never writes them.

## Origin of the unit data

Symbols, labels and measured conversion factors are derived from the [`iliekturtles/uom`](https://github.com/iliekturtles/uom) project (MIT/Apache-2.0); a unit with an exact definition (inch, pound, standard gravity, survey foot, ...) is written from it (see `yadc-codegen/si.toml`).

## License

MIT OR BSD-3-Clause.
