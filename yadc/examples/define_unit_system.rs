//! Define a unit system from scratch: the kinds, the dimension, the quantity
//! family, the constants everything is defined from, and units of every
//! shape. No `yadc-si` - every type below is ours, and so is every modelling
//! choice (which axes exist, which kinds).
//!
//! Numbers only go through units. The base constants (`METRE`, one per base
//! unit, plus the zeros of the points and levels) are the only raw numbers; every other constant
//! is defined from them the way a standards document writes it (`PSI =
//! POUND * STANDARD_GRAVITY / (INCH * INCH)`), and every one of them is a
//! unit, called like a function: `PSI(32.0)` is 32 psi, `PASCAL(p)` is how
//! many pascals `p` is. A scale is a constant too - a zero and a step: °C is
//! `Scale(ICE_POINT, KELVIN)` - and so is a level: dBm is
//! `Scale(MILLIWATT_LEVEL, DECIBEL)`, a scale on the logarithmic line. Other
//! units are types of ours: a function (the Beaufort scale, by hand), and
//! enums naming a choice of units, for a menu or a parser.
//!
//! Everything here is static: each quantity's dimension is a fact of its
//! type. A value whose dimension is only known at run time goes into a
//! dynamic type of yours (the `dynamickit` crate helps build one; yadc does not
//! depend on it) - `yadc-si` has one, `DynamicSIQuantity`.
//!
//! Run with: cargo run -p yadc --example define_unit_system
#![feature(generic_const_exprs, min_adt_const_params, const_trait_impl)]
#![feature(
    const_cmp,
    const_default,
    derive_const,
    const_ops,
    const_convert,
    const_try,
    const_destruct
)]
#![feature(fn_traits, unboxed_closures)]
#![allow(incomplete_features)]

use core::marker::ConstParamTy;
use yadc::{CastAs, Dimension, Unit};

// --- Kinds ---------------------------------------------------------------------
// A kind says how a quantity behaves. `#[default]` marks the ordinary one:
// an amount, which adds to its like, scales and multiplies. This system has
// two more, and the dimension below names which is which:
//
// - `Affine`: a point, measured from a zero somebody chose - a temperature.
//   Two points differ by an amount, an amount moves a point, and nothing
//   else: no sum of two, no scaling, no products. (A point is the strict
//   choice for a temperature: nothing doubles a thermometer reading by
//   mistake, and a formula that wants kelvins as an amount writes `t -
//   ABSOLUTE_ZERO`. A system whose physics multiplies temperatures all day
//   would make it an amount with a label instead - `kind_algebra` shows
//   both.)
// - `Log`: a logarithm - a level or a gain, in decibels. Adding logarithms
//   multiplies what they are the logarithms of, so `+` and `-` add and
//   subtract the exponents: a power level less a power level is a gain (the
//   logarithm of watts over watts), and a gain moves a level.
//
// Any further variant would be a label of our own, with `add` / `sub` rules
// (see the `kind_algebra` example).

#[derive_const(PartialEq, Default)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, Dimension)]
pub enum Kind {
    #[default]
    Linear,
    #[kind(affine)]
    Affine,
    #[kind(log)]
    Log,
}

// --- The dimension ---------------------------------------------------------------
// One exponent per base unit, plus the kind in a `#[kind]` field (optional:
// without one, only the exponents decide what adds to what), which names the
// affine and the logarithmic kind. `#[derive(Dimension)]` writes the
// operators: exponents add under `*`, and every two amounts have a product -
// a point and a level have none, so `*` and `/` give an `Option`; exponents
// must match under `+`, so `+` and `-` give an `Option` too, and the kind
// decides the rest. `Dim::DIMENSIONLESS` is dimension one: what `x / x`
// gives. It prints as the product of its axes, `kg·m⁻¹·s⁻²`.

#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, Dimension)]
pub struct Dim {
    pub m: i8,
    pub s: i8,
    pub kg: i8,
    pub k: i8,
    #[kind]
    pub kind: Kind,
}

// --- The quantity family ---------------------------------------------------------
// One generic type: its dimension a const parameter, its number type a
// parameter too (`f64` unless you say otherwise). The derive writes the
// arithmetic (all `const`), makes every member a unit of itself, and makes
// the scale type it names a unit of its zero's type.
// Only this crate can write the field, so the base constants below are the
// only raw numbers. With the number type a parameter, the operators drop an
// `N` in const code, hence `const_destruct` above.

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::Quantity)]
pub struct Qty<const D: Dim, N = f64>(N);

/// A scale: a zero and a step, `Scale(zero, step)`. A number `n` is `zero +
/// n · step`, and a value reads `(value - zero) / step`.
#[derive(Copy, Clone, Debug)]
pub struct Scale<P, S>(pub P, pub S);

// A scale is a unit of its zero's type: `n` is `zero + n · step`, and a value
// reads `(value - zero) / step`.
const impl<P, S, N> yadc::Unit<P, N> for Scale<P, S>
where
    P: Copy + [const] core::ops::Sub + [const] core::ops::Add<<P as core::ops::Sub>::Output, Output = P>,
    S: [const] yadc::Unit<<P as core::ops::Sub>::Output, N>,
{
    fn quantity(&self, value: N) -> P {
        self.0 + self.1.quantity(value)
    }
    fn value(&self, quantity: P) -> N {
        self.1.value(quantity - self.0)
    }
}

// And it is called like any unit: on a number it builds (`CELSIUS(25.0)`),
// on a value it reads (`CELSIUS(t)`). Std's three function traits, once for
// each direction.
macro_rules! scale_is_called {
    ($arg:ty => $out:ty, $way:ident) => {
        const impl<const D: Dim, N, S> FnOnce<($arg,)> for Scale<Qty<D, N>, S>
        where
            Self: [const] yadc::Unit<Qty<D, N>, N> + [const] core::marker::Destruct,
            N: [const] core::marker::Destruct,
        {
            type Output = $out;
            extern "rust-call" fn call_once(self, (x,): ($arg,)) -> $out {
                yadc::Unit::$way(&self, x)
            }
        }
        const impl<const D: Dim, N, S> FnMut<($arg,)> for Scale<Qty<D, N>, S>
        where
            Self: [const] yadc::Unit<Qty<D, N>, N> + [const] core::marker::Destruct,
            N: [const] core::marker::Destruct,
        {
            extern "rust-call" fn call_mut(&mut self, (x,): ($arg,)) -> $out {
                yadc::Unit::$way(&*self, x)
            }
        }
        const impl<const D: Dim, N, S> Fn<($arg,)> for Scale<Qty<D, N>, S>
        where
            Self: [const] yadc::Unit<Qty<D, N>, N> + [const] core::marker::Destruct,
            N: [const] core::marker::Destruct,
        {
            extern "rust-call" fn call(&self, (x,): ($arg,)) -> $out {
                yadc::Unit::$way(self, x)
            }
        }
    };
}
scale_is_called!(N => Qty<D, N>, quantity);
scale_is_called!(Qty<D, N> => N, value);

// --- Quantities: aliases, spelled relative to `Dim::DIMENSIONLESS` ----------------

pub type Length = Qty<
    {
        Dim {
            m: 1,
            ..Dim::DIMENSIONLESS
        }
    },
>;
pub type Area = Qty<
    {
        Dim {
            m: 2,
            ..Dim::DIMENSIONLESS
        }
    },
>;
pub type Time = Qty<
    {
        Dim {
            s: 1,
            ..Dim::DIMENSIONLESS
        }
    },
>;
pub type Velocity = Qty<
    {
        Dim {
            m: 1,
            s: -1,
            ..Dim::DIMENSIONLESS
        }
    },
>;
pub type Mass = Qty<
    {
        Dim {
            kg: 1,
            ..Dim::DIMENSIONLESS
        }
    },
>;
pub type Acceleration = Qty<
    {
        Dim {
            m: 1,
            s: -2,
            ..Dim::DIMENSIONLESS
        }
    },
>;
pub type Force = Qty<
    {
        Dim {
            kg: 1,
            m: 1,
            s: -2,
            ..Dim::DIMENSIONLESS
        }
    },
>;
pub type Pressure = Qty<
    {
        Dim {
            kg: 1,
            m: -1,
            s: -2,
            ..Dim::DIMENSIONLESS
        }
    },
>;
pub type Power = Qty<
    {
        Dim {
            kg: 1,
            m: 2,
            s: -3,
            ..Dim::DIMENSIONLESS
        }
    },
>;

// The same exponents (`k: 1`), different kinds: a temperature is a point, a
// step an amount. Two temperatures differ by a step; a step moves a
// temperature; `step + temperature` does not compile (a sum keeps its left
// operand's type, so write `temperature + step`), nor does `2.0 *
// temperature`.
pub type Temperature = Qty<
    {
        Dim {
            k: 1,
            kind: Kind::Affine,
            ..Dim::DIMENSIONLESS
        }
    },
>;
pub type TemperatureStep = Qty<
    {
        Dim {
            k: 1,
            ..Dim::DIMENSIONLESS
        }
    },
>;

/// A gain: the logarithm of a plain ratio. Gains add where ratios multiply.
pub type Gain = Qty<
    {
        Dim {
            kind: Kind::Log,
            ..Dim::DIMENSIONLESS
        }
    },
>;
/// A power level: the logarithm of a power - the exponents of power, the
/// logarithmic kind. A level less a level is the logarithm of watts over
/// watts: a `Gain`.
pub type PowerLevel = Qty<
    {
        Dim {
            kg: 1,
            m: 2,
            s: -3,
            kind: Kind::Log,
            ..Dim::DIMENSIONLESS
        }
    },
>;

// The same family counting in `f32` - say, for a compact telemetry record.
pub type Length32 = Qty<
    {
        Dim {
            m: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    f32,
>;

// --- Constants -----------------------------------------------------------------
// The base constants are the only raw numbers: one per base unit, written
// here, where `Qty`'s field is visible. A temperature has two: the step (a
// kelvin of difference, an amount) and absolute zero (a point). The
// logarithmic line has two as well: the step (a bel: ten times) and the
// level of a watt.

pub const METRE: Length = Qty(1.0);
pub const SECOND: Time = Qty(1.0);
pub const KILOGRAM: Mass = Qty(1.0);
pub const KELVIN: TemperatureStep = Qty(1.0);
pub const ABSOLUTE_ZERO: Temperature = Qty(0.0);
pub const BEL: Gain = Qty(1.0);
pub const WATT_LEVEL: PowerLevel = Qty(0.0);

// Everything else is defined from them, with the quantities' own `const`
// operators, evaluated by the compiler.

pub const KILOMETRE: Length = 1000.0 * METRE;
pub const NAUTICAL_MILE: Length = 1852.0 * METRE;
pub const INCH: Length = 0.0254 * METRE;
pub const SQUARE_METRE: Area = METRE * METRE;
pub const SQUARE_KILOMETRE: Area = KILOMETRE * KILOMETRE;
pub const HOUR: Time = 3600.0 * SECOND;
pub const POUND: Mass = 0.453_592_37 * KILOGRAM;
pub const METRE_PER_SECOND: Velocity = METRE / SECOND;
pub const KILOMETRE_PER_HOUR: Velocity = KILOMETRE / HOUR;
pub const KNOT: Velocity = NAUTICAL_MILE / HOUR;
pub const STANDARD_GRAVITY: Acceleration = 9.80665 * METRE / (SECOND * SECOND);
pub const NEWTON: Force = KILOGRAM * METRE / (SECOND * SECOND);
pub const PASCAL: Pressure = NEWTON / SQUARE_METRE;
pub const PSI: Pressure = POUND * STANDARD_GRAVITY / (INCH * INCH);
pub const WATT: Power = NEWTON * METRE / SECOND;
pub const MILLIWATT: Power = WATT / 1000.0;
/// A step of one degree Fahrenheit: five ninths of a kelvin.
pub const DEGREE_FAHRENHEIT: TemperatureStep = KELVIN * (5.0 / 9.0);
/// 0 °C: the ice point of water.
pub const ICE_POINT: Temperature = ABSOLUTE_ZERO + 273.15 * KELVIN;
/// 0 °F: 32 degrees Fahrenheit below the ice point.
pub const FAHRENHEIT_ZERO: Temperature = ICE_POINT - 32.0 * DEGREE_FAHRENHEIT;
pub const DECIBEL: Gain = 0.1 * BEL;
/// 0 dBm: the level of a milliwatt, 30 dB below a watt.
pub const MILLIWATT_LEVEL: PowerLevel = WATT_LEVEL - 30.0 * DECIBEL;

// The `f32` constants: the same values, counted in `f32`.
pub const METRE_32: Length32 = METRE.cast::<f32>();
pub const NAUTICAL_MILE_32: Length32 = NAUTICAL_MILE.cast::<f32>();

// --- Scales: a zero and a step ---------------------------------------------------
// A scale is `Scale(zero, step)`: a number `n` is `zero + n · step`. It is a
// constant like any other, and a unit: `CELSIUS(25.0)` is 298.15 K - the
// zero point is the scale's business.

/// Steps of one kelvin counted from the ice point.
pub const CELSIUS: Scale<Temperature, TemperatureStep> = Scale(ICE_POINT, KELVIN);

/// Steps of a degree Fahrenheit from 0 °F.
pub const FAHRENHEIT: Scale<Temperature, TemperatureStep> = Scale(FAHRENHEIT_ZERO, DEGREE_FAHRENHEIT);

/// Kelvin, as a reading of a `Temperature`: counted from absolute zero.
pub const KELVIN_SCALE: Scale<Temperature, TemperatureStep> = Scale(ABSOLUTE_ZERO, KELVIN);

// --- Levels: a scale on the logarithmic line --------------------------------------
// dBm is ten times the base-10 logarithm of the power over 1 mW, so 30 dBm is
// 1 W: it starts at a milliwatt, and each step multiplies by 10^0.1. On the
// logarithmic line that is a scale like °C - a zero and a step.

/// Decibels from one milliwatt.
pub const DBM: Scale<PowerLevel, Gain> = Scale(MILLIWATT_LEVEL, DECIBEL);

// The logarithm itself is written once: the conversions between the line and
// the linear quantity. `DBM` reads levels; the watts are one std `From`
// away: `Power::from(DBM(30.0))`.

impl From<PowerLevel> for Power {
    fn from(level: PowerLevel) -> Power {
        WATT * 10f64.powf(BEL(level - WATT_LEVEL))
    }
}

impl From<Power> for PowerLevel {
    fn from(power: Power) -> PowerLevel {
        WATT_LEVEL + BEL(WATT(power).log10())
    }
}

// --- A function: written by hand ----------------------------------------------
// The Beaufort scale is not a multiple of anything: v = 0.836 · B^1.5 m/s. A
// unit is two inverse functions, so this one is a zero-sized type with a
// hand-written `impl Unit`, both functions in quantity terms.

#[derive(Copy, Clone, Debug)]
pub struct Beaufort;

impl Unit<Velocity> for Beaufort {
    fn quantity(&self, b: f64) -> Velocity {
        0.836 * b.powf(1.5) * METRE_PER_SECOND
    }
    fn value(&self, v: Velocity) -> f64 {
        (METRE_PER_SECOND(v) / 0.836).powf(2.0 / 3.0)
    }
}

// --- Unit enums: a choice of units ---------------------------------------------
// An enum whose variants each name the unit value they are, for a menu, a
// config file or a parser: `by = ..` a quantity constant, a scale, or a unit
// type of ours. `#[derive(Unit)]` writes `Unit`, and `Display` (`{}` the
// symbol, `{:#}` the name) and `FromStr` from the labels.

#[derive(Copy, Clone, Debug, yadc::Unit)]
#[unit(of = Length)]
pub enum LengthUnit {
    #[unit("m", "metres", by = METRE)]
    Metre,
    #[unit("km", "kilometres", by = KILOMETRE)]
    Kilometre,
    #[unit("nmi", "nautical miles", by = NAUTICAL_MILE)]
    NauticalMile,
}

/// The `f32` lengths: units counting in `f32`.
#[derive(Copy, Clone, Debug, yadc::Unit)]
#[unit(of = Length32, number = f32)]
pub enum Length32Unit {
    #[unit("m", "metres", by = METRE_32)]
    Metre,
    #[unit("nmi", "nautical miles", by = NAUTICAL_MILE_32)]
    NauticalMile,
}

#[derive(Copy, Clone, Debug, yadc::Unit)]
#[unit(of = Area)]
pub enum AreaUnit {
    #[unit("m²", "square metres", by = SQUARE_METRE)]
    SquareMetre,
    #[unit("km²", "square kilometres", by = SQUARE_KILOMETRE)]
    SquareKilometre,
}

#[derive(Copy, Clone, Debug, yadc::Unit)]
#[unit(of = Mass)]
pub enum MassUnit {
    #[unit("kg", "kilograms", by = KILOGRAM)]
    Kilogram,
    #[unit("lb", "pounds", by = POUND)]
    Pound,
}

#[derive(Copy, Clone, Debug, yadc::Unit)]
#[unit(of = Pressure)]
pub enum PressureUnit {
    #[unit("Pa", "pascals", by = PASCAL)]
    Pascal,
    #[unit("psi", "pounds per square inch", by = PSI)]
    Psi,
}

/// Kelvin is a scale here too - counted from absolute zero - since a
/// `Temperature` is a point, not an amount.
#[derive(Copy, Clone, Debug, yadc::Unit)]
#[unit(of = Temperature)]
pub enum TemperatureUnit {
    #[unit("K", "kelvin", by = KELVIN_SCALE)]
    Kelvin,
    #[unit("°C", "degrees Celsius", by = CELSIUS)]
    Celsius,
    #[unit("°F", "degrees Fahrenheit", by = FAHRENHEIT)]
    Fahrenheit,
}

/// A step has units of its own: no zero.
#[derive(Copy, Clone, Debug, yadc::Unit)]
#[unit(of = TemperatureStep)]
pub enum TemperatureStepUnit {
    #[unit("K", "kelvin", by = KELVIN)]
    Kelvin,
    #[unit("°F", "degrees Fahrenheit", by = DEGREE_FAHRENHEIT)]
    Fahrenheit,
}

/// Watts and a level in one enum. `DBM` measures a `PowerLevel`; `via`
/// names that type, and the enum converts to and from the watts with the
/// `From`s above - a logarithm, which is not `const`, so neither is this
/// enum's impl (`non_const`).
#[derive(Copy, Clone, Debug, yadc::Unit)]
#[unit(of = Power, non_const)]
pub enum PowerUnit {
    #[unit("W", "watts", by = WATT)]
    Watt,
    #[unit("mW", "milliwatts", by = MILLIWATT)]
    Milliwatt,
    #[unit("dBm", "decibel-milliwatts", by = DBM, via = PowerLevel)]
    DecibelMilliwatt,
}

/// The hand-written Beaufort scale beside plain speeds.
#[derive(Copy, Clone, Debug, yadc::Unit)]
#[unit(of = Velocity, non_const)]
pub enum WindUnit {
    #[unit("m/s", "metres per second", by = METRE_PER_SECOND)]
    MetrePerSecond,
    #[unit("kn", "knots", by = KNOT)]
    Knot,
    #[unit("Bft", "Beaufort", by = Beaufort)]
    Beaufort,
}

// --- Units at compile time ---------------------------------------------------------
// A constant's `Unit`, a scale's and a unit enum's are `const impl`s:
// building and reading works in `const` items too.

pub const WATER_BOILS: Temperature = CELSIUS(100.0);
pub const WATER_BOILS_F: f64 = FAHRENHEIT(WATER_BOILS);
pub const BODY_TEMPERATURE: Temperature = TemperatureUnit::Celsius(36.6);

fn main() {
    use LengthUnit::{Kilometre, Metre, NauticalMile};

    // A unit is called: on a number it builds a quantity, on a quantity it
    // reads one back. Maths is on quantities. A unit prints its symbol with
    // `{}`, its name with `{:#}`.
    let side = Kilometre(2.5);
    let area: Area = side * side;
    let km2 = AreaUnit::SquareKilometre;
    println!("side = {} {Kilometre:#}", Kilometre(side));
    println!("area = {} {km2:#}", km2(area));
    // A quantity prints itself too: its number and its dimension.
    println!("area = {area}");
    // Every constant is a unit too: no enum needed to read a number.
    let m = MassUnit::Pound(10.0);
    println!("10 lb = {:.4} kg", KILOGRAM(m));

    // A unit defined from the base constants: PSI is a pound under standard
    // gravity on a square inch.
    let tyre = PSI(32.0);
    println!("32 psi = {:.0} Pa", PASCAL(tyre));

    // Knots are nautical miles per hour - computed, read with the constant
    // (the enum variant is the same unit with a name, so it prints).
    let v: Velocity = NauticalMile(20.0) / HOUR(1.0);
    println!(
        "{:.0} {} = {:.2} m/s = {:.1} km/h",
        KNOT(v),
        WindUnit::Knot,
        METRE_PER_SECOND(v),
        KILOMETRE_PER_HOUR(v),
    );

    // Scales: the zero point is the scale's business.
    use TemperatureUnit::{Celsius, Fahrenheit};
    let body = CELSIUS(36.6);
    assert_eq!(body, BODY_TEMPERATURE);
    println!(
        "body = {:.1} {Celsius} = {:.1} {Fahrenheit}",
        CELSIUS(body),
        FAHRENHEIT(body)
    );
    println!("water boils at {WATER_BOILS_F:.0} {Fahrenheit} (a compile-time constant)");
    // Two temperatures differ by a step, another type with its own units.
    let fever: TemperatureStep = body - Celsius(20.0);
    let step_f = TemperatureStepUnit::Fahrenheit;
    println!(
        "body - 20 °C = {:.1} K = {:.1} {step_f:#}",
        KELVIN(fever),
        step_f(fever)
    );
    // A step moves a temperature (`warm + step`, not `step + warm`).
    let warm = Celsius(21.5) + KELVIN(1.0);
    println!("21.5 °C + 1 K = {:.1} {Celsius}", CELSIUS(warm));
    // A scale made at run time: degrees above body temperature.
    let above_body = Scale(body, KELVIN);
    println!("38.1 °C is {:.1} K of fever", above_body(Celsius(38.1)));

    // A level's scale builds a level; the watts are one `From` away. The
    // enum's dBm does the conversion itself.
    let sent = DBM(30.0);
    println!("30 dBm = {:.3} W", WATT(Power::from(sent)));
    let dbm = PowerUnit::DecibelMilliwatt;
    println!("2 W = {:.2} {dbm}", dbm(WATT(2.0)));
    // On the logarithmic line multiplying is adding: a level moves by gains.
    let received = sent - DECIBEL(3.0) + DECIBEL(13.0);
    println!(
        "30 dBm - 3 dB + 13 dB = {:.0} {dbm} = {:.0} W",
        DBM(received),
        WATT(Power::from(received))
    );
    // Two levels differ by a gain; `sent + received` does not compile (it
    // would be the logarithm of watts squared, which is no power level).
    let gain: Gain = received - sent;
    println!("gain = {:.0} dB", DECIBEL(gain));
    // A gain per length is a plain rate - a number of bels per metre - and
    // that many bels is a gain again: 0.2 dB/km over 15 km.
    let fibre = DECIBEL(0.2) / KILOMETRE;
    let loss: Gain = (fibre * KILOMETRE(15.0)) * BEL;
    println!(
        "0.2 dB/km over 15 km = {:.1} dB, leaving {:.1} dBm",
        DECIBEL(loss),
        DBM(received - loss)
    );
    // A temperature as an amount, where a formula wants one: the kelvins
    // above absolute zero. Twice as hot is twice that.
    let hot: Temperature = ABSOLUTE_ZERO + 2.0 * (body - ABSOLUTE_ZERO);
    println!("twice as hot as a body: {:.1} {Celsius}", CELSIUS(hot));

    // The function-shaped unit, both ways. Written by hand, it is its two
    // `Unit` methods; in the `WindUnit` enum it is called like the rest.
    let gale = Beaufort.quantity(5.0);
    println!("Bft 5 = {:.2} m/s", METRE_PER_SECOND(gale));

    // A second number type: an `f32` length, with units counting in `f32`.
    // Widening `f32` to `f64` loses nothing, so it is also a plain `From`.
    let compact: Length32 = Length32Unit::NauticalMile(1.5);
    let wide = Length::from(compact);
    println!(
        "1.5 nmi in f32 = {} m; widened = {} m",
        Length32Unit::Metre(compact),
        METRE(wide),
    );

    // Units picked at run time: some unit of length, checked at compile
    // time - an enum, a constant, anything. Behind `&dyn` a unit is its two
    // methods.
    let run = 1500.0 * METRE;
    let any: [&dyn Unit<Length>; 3] = [&Metre, &KILOMETRE, &INCH];
    let numbers = any.map(|u| format!("{:.1}", u.value(run)));
    println!("1500 m in [m, km, in] = [{}]", numbers.join(", "));
    // A choice in an enum also prints and parses.
    for unit in [WindUnit::MetrePerSecond, WindUnit::Knot, WindUnit::Beaufort] {
        println!(
            "{unit:>#18}: {:.2} {unit}",
            unit(METRE_PER_SECOND(10.0))
        );
    }
    let parsed: LengthUnit = "nmi".parse().expect("a length symbol");
    println!(
        "parsed \"nmi\": {parsed:#}, 1500 m = {:.3} {parsed}",
        parsed(run)
    );
}
