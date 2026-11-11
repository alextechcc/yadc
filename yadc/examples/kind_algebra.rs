//! Axes and kinds: what composes under `*`, how a quantity behaves under `+`,
//! and what the compiler refuses.
//!
//! A dimension in yadc is a set of exponent *axes* - metres, seconds, kelvin,
//! radians - plus a *kind* (the `#[kind]` field), which says how the quantity
//! behaves. This example tours the kinds, one section each:
//!
//! - **An amount** (the ordinary kind, `Linear` here). Its zero is a fact, so
//!   every two multiply and divide (the exponents add and subtract), every
//!   power and reciprocal exists, and only a root can fail: the square root
//!   of a length would need m^½.
//! - **A point** (`Affine`): measured from a zero somebody chose - a
//!   temperature on a thermometer. Two points differ by an amount, an amount
//!   moves a point, and nothing else: no sum of two points, no scaling, no
//!   products.
//! - **A logarithm** (`Log`): a level or a gain, in decibels. Adding
//!   logarithms multiplies what they are the logarithms of, so `+` and `-`
//!   add and subtract the exponents: a power level less a power level is a
//!   gain, and a gain moves a level.
//! - **A label of your own**: a name that keeps apart two quantities with the
//!   same exponents, and matters to `+` and `-` only. Apparent power (volt-
//!   amperes) is not real power (watts), though both are kg·m²·s⁻³; a
//!   hardness number orders minerals but has no differences, so its rules
//!   say it does not add or subtract at all; and a thermodynamic
//!   temperature - an amount with a true zero, which physics multiplies -
//!   is a label whose rules say two of it differ by a step. The same
//!   temperature is here both ways, a point and that label, side by side.
//!
//! Angles are not a kind but an axis (`rad`), so a solid angle is rad² and a
//! torque is J/rad. Each section prints what works; what does not is shown
//! as commented-out code with the compiler's message. Units are called like
//! functions throughout: on a number they build, on a quantity they read.
//!
//! Run with: cargo run -p yadc --example kind_algebra
#![feature(generic_const_exprs, min_adt_const_params, const_trait_impl)]
#![feature(fn_traits, unboxed_closures, const_destruct)]
#![feature(
    const_cmp,
    const_default,
    derive_const,
    const_ops,
    const_convert,
    const_try
)]
#![allow(incomplete_features)]

use core::marker::ConstParamTy;
use yadc::num_traits::Inv;
use yadc::{Dimension, PartialPowi, PartialRoot, PowiAs, RootAs, Unit};

// --- The kinds ---------------------------------------------------------------
// The `#[default]` variant is the ordinary kind, an amount. `Affine` and
// `Log` are plain variants here: the dimension below names them as its
// affine and its logarithmic kind, and what they do is the dimension's
// business. Any other variant is a label of ours: by default it adds to and
// subtracts from itself and nothing else, and `#[kind(add(..) = ..,
// sub(..) = ..)]` changes that - `= invalid` for "no result". Products and
// quotients need no rules: a label survives only beside a plain number.

#[derive_const(PartialEq, Default)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
pub enum Kind {
    /// An amount: a length, a duration, a temperature step, a ratio.
    #[default]
    Linear,
    /// A point: a temperature.
    #[kind(affine)]
    Affine,
    /// A logarithm: a level, a gain.
    #[kind(log)]
    Log,
    /// Apparent power: volt-amperes, kept apart from watts.
    Apparent,
    /// A hardness number: an order with no differences.
    #[kind(add(Hardness) = invalid, sub(Hardness) = invalid)]
    Hardness,
    /// A thermodynamic temperature: an amount (absolute zero is a fact, so it
    /// scales and multiplies), kept apart from its differences - two differ
    /// by a plain step, and a plain step moves one.
    #[kind(sub(Absolute) = Linear, add(Linear) = Absolute, sub(Linear) = Absolute)]
    Absolute,
}

// --- The axes ----------------------------------------------------------------

#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
pub struct Dim {
    pub m: i8,
    pub kg: i8,
    pub s: i8,
    pub k: i8,
    pub rad: i8,
    #[kind]
    pub kind: Kind,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::Quantity)]
pub struct Qty<const D: Dim>(f64);

/// A scale: a zero and a step - a unit of the zero's type, counting steps
/// from the zero.
#[derive(Copy, Clone)]
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

// --- Quantities ----------------------------------------------------------------

pub type Ratio = Qty<{ Dim::DIMENSIONLESS }>;
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
/// A temperature as a thermometer reads it: a point on the line of kelvins.
/// Nothing here can double it or multiply it by mistake.
pub type Temperature = Qty<
    {
        Dim {
            k: 1,
            kind: Kind::Affine,
            ..Dim::DIMENSIONLESS
        }
    },
>;
/// A temperature as thermodynamics uses it: an amount, with the `Absolute`
/// label. `n * R * t` and an average are written as they are on paper.
pub type ThermodynamicTemperature = Qty<
    {
        Dim {
            k: 1,
            kind: Kind::Absolute,
            ..Dim::DIMENSIONLESS
        }
    },
>;
/// A temperature step: an amount.
pub type TemperatureStep = Qty<
    {
        Dim {
            k: 1,
            ..Dim::DIMENSIONLESS
        }
    },
>;
pub type Angle = Qty<
    {
        Dim {
            rad: 1,
            ..Dim::DIMENSIONLESS
        }
    },
>;
pub type SolidAngle = Qty<
    {
        Dim {
            rad: 2,
            ..Dim::DIMENSIONLESS
        }
    },
>;
pub type AngularSpeed = Qty<
    {
        Dim {
            s: -1,
            rad: 1,
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
pub type Energy = Qty<
    {
        Dim {
            m: 2,
            kg: 1,
            s: -2,
            ..Dim::DIMENSIONLESS
        }
    },
>;
/// Real power: watts.
pub type Power = Qty<
    {
        Dim {
            m: 2,
            kg: 1,
            s: -3,
            ..Dim::DIMENSIONLESS
        }
    },
>;
/// Apparent power: the same exponents as `Power`, a label of its own.
pub type ApparentPower = Qty<
    {
        Dim {
            m: 2,
            kg: 1,
            s: -3,
            kind: Kind::Apparent,
            ..Dim::DIMENSIONLESS
        }
    },
>;
/// A power level: the logarithm of a power.
pub type PowerLevel = Qty<
    {
        Dim {
            m: 2,
            kg: 1,
            s: -3,
            kind: Kind::Log,
            ..Dim::DIMENSIONLESS
        }
    },
>;
/// A gain: the logarithm of a plain ratio.
pub type Gain = Qty<
    {
        Dim {
            kind: Kind::Log,
            ..Dim::DIMENSIONLESS
        }
    },
>;
/// A hardness number on the Mohs scale.
pub type Hardness = Qty<
    {
        Dim {
            kind: Kind::Hardness,
            ..Dim::DIMENSIONLESS
        }
    },
>;
/// Energy per radian: torque × angle is the work done turning.
pub type Torque = Qty<
    {
        Dim {
            m: 2,
            kg: 1,
            s: -2,
            rad: -1,
            ..Dim::DIMENSIONLESS
        }
    },
>;

/// K⁴: a power of a temperature step, as in σT⁴.
pub type KelvinToTheFourth = Qty<
    {
        Dim {
            k: 4,
            ..Dim::DIMENSIONLESS
        }
    },
>;

// --- Constants: the base ones, and the rest from them ---------------------------
// A base constant is a raw number, written where the field is visible: one
// per axis, the zero of each line of points and levels, and one per label -
// a kind is never computed, a constant that has it gives it.

pub const ONE: Ratio = Qty(1.0);
pub const METRE: Length = Qty(1.0);
pub const SECOND: Time = Qty(1.0);
pub const KELVIN: TemperatureStep = Qty(1.0);
pub const ABSOLUTE_ZERO: Temperature = Qty(0.0);
/// No temperature at all: the zero of the thermodynamic amount.
pub const ZERO_KELVIN: ThermodynamicTemperature = Qty(0.0);
pub const RADIAN: Angle = Qty(1.0);
pub const KILOGRAM: Mass = Qty(1.0);
/// The bel: a gain of ten times.
pub const BEL: Gain = Qty(1.0);
/// The level of one watt.
pub const WATT_LEVEL: PowerLevel = Qty(0.0);
/// One volt-ampere of apparent power.
pub const VOLT_AMPERE: ApparentPower = Qty(1.0);
/// Mohs hardness numbers: a hardness is not built from a unit - it has no
/// "one of it" to count - so each is a constant.
pub const TALC: Hardness = Qty(1.0);
pub const QUARTZ: Hardness = Qty(7.0);
pub const DIAMOND: Hardness = Qty(10.0);

pub const KILOMETRE: Length = 1000.0 * METRE;
pub const SQUARE_METRE: Area = METRE * METRE;
pub const ICE_POINT: Temperature = ABSOLUTE_ZERO + 273.15 * KELVIN;
pub const DEGREE: Angle = RADIAN * (core::f64::consts::PI / 180.0);
pub const STERADIAN: SolidAngle = RADIAN * RADIAN;
pub const RADIAN_PER_SECOND: AngularSpeed = RADIAN / SECOND;
pub const RPM: AngularSpeed = core::f64::consts::TAU * RADIAN / (60.0 * SECOND);
pub const JOULE: Energy = KILOGRAM * SQUARE_METRE / (SECOND * SECOND);
pub const WATT: Power = JOULE / SECOND;
pub const DECIBEL: Gain = 0.1 * BEL;
/// The level of one milliwatt: 30 dB below a watt.
pub const MILLIWATT_LEVEL: PowerLevel = WATT_LEVEL - 30.0 * DECIBEL;
/// K⁴: a plain power of kelvin, as in σT⁴.
pub const KELVIN_TO_THE_FOURTH: KelvinToTheFourth = KELVIN.powi::<4>();
/// Torque is energy per radian of turn.
pub const NEWTON_METRE_PER_RADIAN: Torque = JOULE / RADIAN;

// The logarithm itself: the two conversions between the line of levels and
// the watts. A scale of levels reads levels; the watts are one `From` away.

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

// --- A unit or two for each, to build and print with --------------------------

#[derive(Copy, Clone, Unit)]
#[unit("m", "metres", of = Length, by = METRE)]
pub struct Metre;

#[derive(Copy, Clone, Unit)]
#[unit("m²", "square metres", of = Area, by = SQUARE_METRE)]
pub struct SquareMetre;

#[derive(Copy, Clone, Unit)]
#[unit("s", "seconds", of = Time, by = SECOND)]
pub struct Second;

/// A scale: steps of one kelvin from the ice point - a zero and a step.
pub const CELSIUS: Scale<Temperature, TemperatureStep> = Scale(ICE_POINT, KELVIN);

/// Kelvin, as a reading of a temperature: from absolute zero.
pub const KELVIN_SCALE: Scale<Temperature, TemperatureStep> = Scale(ABSOLUTE_ZERO, KELVIN);

/// The same two scales on the thermodynamic amount: a scale is a zero and a
/// step whatever the zero's kind, as long as two of it differ by the step's
/// type and the step's type moves one.
pub const THERMODYNAMIC_KELVIN: Scale<ThermodynamicTemperature, TemperatureStep> =
    Scale(ZERO_KELVIN, KELVIN);
pub const THERMODYNAMIC_CELSIUS: Scale<ThermodynamicTemperature, TemperatureStep> =
    Scale(ZERO_KELVIN + 273.15 * KELVIN, KELVIN);

/// The scales by name, which print.
#[derive(Copy, Clone, Unit)]
#[unit(of = Temperature)]
pub enum TemperatureUnit {
    #[unit("K", "kelvin", by = KELVIN_SCALE)]
    KelvinScale,
    #[unit("°C", "degrees Celsius", by = CELSIUS)]
    Celsius,
}

/// Kelvin, as a step: no zero.
#[derive(Copy, Clone, Unit)]
#[unit("K", "kelvin", of = TemperatureStep, by = KELVIN)]
pub struct Kelvin;

/// A scale of levels: decibels from one milliwatt.
pub const DBM: Scale<PowerLevel, Gain> = Scale(MILLIWATT_LEVEL, DECIBEL);

#[derive(Copy, Clone, Unit)]
#[unit("dB", "decibels", of = Gain, by = DECIBEL)]
pub struct Decibel;

#[derive(Copy, Clone, Unit)]
#[unit("W", "watts", of = Power, by = WATT)]
pub struct Watt;

#[derive(Copy, Clone, Unit)]
#[unit("VA", "volt-amperes", of = ApparentPower, by = VOLT_AMPERE)]
pub struct VoltAmpere;

#[derive(Copy, Clone, Unit)]
#[unit(of = Angle)]
pub enum AngleUnit {
    #[unit("rad", "radians", by = RADIAN)]
    Radian,
    #[unit("°", "degrees", by = DEGREE)]
    Degree,
}

#[derive(Copy, Clone, Unit)]
#[unit("sr", "steradians", of = SolidAngle, by = STERADIAN)]
pub struct Steradian;

#[derive(Copy, Clone, Unit)]
#[unit(of = AngularSpeed)]
pub enum AngularSpeedUnit {
    #[unit("rad/s", "radians per second", by = RADIAN_PER_SECOND)]
    RadianPerSecond,
    #[unit("rpm", "revolutions per minute", by = RPM)]
    RevolutionPerMinute,
}

#[derive(Copy, Clone, Unit)]
#[unit("J", "joules", of = Energy, by = JOULE)]
pub struct Joule;

#[derive(Copy, Clone, Unit)]
#[unit("N·m/rad", "newton metres per radian", of = Torque, by = NEWTON_METRE_PER_RADIAN)]
pub struct NewtonMetrePerRadian;

fn main() {
    use AngleUnit::{Degree, Radian};

    // --- Amounts: every product and quotient exists --------------------------------
    // Exponents add under `*` and subtract under `/`, whatever they are, so
    // every power and every reciprocal of an amount exists too.
    println!("== Amounts: every product and quotient exists");
    let a = Metre(3.0);
    let b = Metre(4.0);
    println!(
        "3 m × 4 m        = {} {SquareMetre}",
        SquareMetre(a * b)
    );
    println!(
        "(3 m)²           = {} {SquareMetre}",
        SquareMetre(a.powi::<2>())
    );
    // A ratio of two lengths is a plain number, read in the unit `ONE`.
    println!("3 m / 4 m        = {}", ONE(a / b));
    let floor = SquareMetre(16.0);
    println!(
        "√(16 m²)         = {} {Metre}",
        Metre(floor.root::<2>())
    );
    let per_radian = Radian(2.0).inv();
    println!(
        "1 / (2 rad) × 1 rad = {}",
        ONE(per_radian * Radian(1.0))
    );
    // Only roots can fail - with whole-number exponents a length has no
    // square root (m^½):
    // let _ = a.root::<2>();
    //     error: this root has no dimension

    // --- A point: measured from a zero somebody chose ---------------------------------
    // T - T is a step; T ± step is a temperature; a midpoint only ever halves
    // a step. Where a formula wants the temperature as an amount - the
    // kelvins above absolute zero, as in σT⁴ - it says so: `t - ABSOLUTE_ZERO`.
    println!("\n== A point (a temperature)");
    // A scale is a constant; its enum variant is the same unit with a name.
    use TemperatureUnit::{Celsius, KelvinScale};
    let morning = CELSIUS.quantity(15.0);
    let noon = CELSIUS.quantity(22.5);
    let warming: TemperatureStep = noon - morning;
    println!("22.5 °C - 15 °C  = {} {Kelvin}", Kelvin(warming));
    println!(
        "15 °C + 7.5 K    = {} {Celsius}",
        Celsius(morning + warming)
    );
    println!(
        "22.5 °C - 7.5 K  = {} {Celsius}",
        Celsius(noon - warming)
    );
    let mean: Temperature = morning + (noon - morning) / 2.0;
    println!("mean of the two  = {:.2} {Celsius}", Celsius(mean));
    // Twice as hot: twice the kelvins above absolute zero, moved back onto
    // the line.
    let cold = KELVIN_SCALE.quantity(150.0);
    let doubled: Temperature = ABSOLUTE_ZERO + 2.0 * (cold - ABSOLUTE_ZERO);
    println!(
        "twice 150 K      = {} {KelvinScale}",
        KelvinScale(doubled)
    );
    let hot = KelvinScale(300.0);
    let hot4: KelvinToTheFourth = (hot - ABSOLUTE_ZERO).powi::<4>();
    println!(
        "(300 K)⁴         = {:.3e} K⁴",
        KELVIN_TO_THE_FOURTH(hot4)
    );
    // Everything else would depend on where the zero was put:
    // let _ = warming + morning;   // a sum keeps its left operand: `morning + warming`
    //     error: cannot add a point to an amount
    // let _ = morning + noon;
    //     error: cannot add a point to a point
    // let _ = 2.0 * morning;
    //     error: a point cannot be scaled
    // let _ = morning * morning;
    //     error: cannot multiply: these have no product
    // let _ = morning.root::<2>();
    //     error: a point has no roots

    // --- A logarithm: adding is multiplying -------------------------------------------
    // A power level is the logarithm of a power, a gain the logarithm of a
    // plain ratio. `+` and `-` add and subtract the exponents, as `*` and `/`
    // do for the things themselves: level - level is watts over watts, a
    // gain; level ± gain is watts times a ratio, a level. `DBM` is a scale on
    // the line of levels - a zero and a step - and the watts are one `From`
    // away.
    println!("\n== A logarithm (levels and gains)");
    let sent = DBM.quantity(30.0);
    let received = sent - Decibel(3.0) + Decibel(13.0);
    println!("30 dBm - 3 dB + 13 dB = {:.0} dBm", DBM.value(received));
    let gain: Gain = received - sent;
    println!("received - sent  = {:.0} {Decibel}", Decibel(gain));
    println!(
        "40 dBm           = {:.0} {Watt}",
        Watt(Power::from(received))
    );
    let two_watts = Watt(2.0);
    println!(
        "2 W              = {:.2} dBm",
        DBM.value(PowerLevel::from(two_watts))
    );
    // A gain per length is a plain rate - bels per metre - and that many
    // bels is a gain again: a kind is given by a constant that has it.
    let fibre = Decibel(0.2) / KILOMETRE;
    let loss: Gain = (fibre * KILOMETRE(15.0)) * BEL;
    println!("0.2 dB/km × 15 km = {:.1} {Decibel}", Decibel(loss));
    // Two levels sum to the logarithm of watts squared, which is no power
    // level; doubling a level squares the power; and a level has no products.
    // let _ = sent + received;
    //     error: cannot add: the labels have no sum
    // let _ = gain + sent;   // a sum keeps its left operand: `sent + gain`
    //     error: cannot add: the dimensions differ
    // let _ = 2.0 * sent;
    //     error: cannot scale: this label cannot be added to itself
    // let _ = sent * a;
    //     error: cannot multiply: these have no product
    // let _ = sent + two_watts;   // only a logarithm adds to a logarithm
    //     error: cannot add: the labels have no sum

    // --- Labels of your own: `+` only within one -----------------------------------------
    // Apparent power adds to apparent power, scales, and divides into a
    // plain ratio - but it is not real power. Multiplied by anything with
    // units the label drops.
    println!("\n== A label (apparent power)");
    let load: ApparentPower = VoltAmpere(500.0);
    let more: ApparentPower = VoltAmpere(250.0);
    println!(
        "500 VA + 250 VA  = {} {VoltAmpere}",
        VoltAmpere(load + more)
    );
    println!(
        "500 VA × 1.5     = {} {VoltAmpere}",
        VoltAmpere(load * 1.5)
    );
    let real: Power = Watt(400.0);
    println!("400 W / 500 VA   = {}   (a power factor)", ONE(real / load));
    let used: Energy = load * Second(60.0);
    println!(
        "500 VA × 60 s    = {} {Joule}   (the label drops)",
        Joule(used)
    );
    // A power factor times an apparent power is a plain power: the label
    // survives only beside a plain *number*, and reading it out of the
    // ratio says which you mean.
    let delivered: Power = ONE(real / load) * WATT * VoltAmpere(load);
    println!("0.8 × 500 VA     = {} {Watt}", Watt(delivered));
    // let _ = real + load;
    //     error: cannot add: the labels have no sum

    // A hardness number orders minerals, and that is all: its rules say it
    // has no sum and no difference, so it cannot be scaled either.
    println!("\n== A label with rules (hardness)");
    println!(
        "talc < quartz < diamond: {}",
        TALC < QUARTZ && QUARTZ < DIAMOND
    );
    println!("quartz itself    = {QUARTZ}");
    // let _ = QUARTZ + TALC;
    //     error: cannot add: the labels have no sum
    // let _ = DIAMOND - QUARTZ;
    //     error: cannot subtract: the labels have no difference
    // let _ = 2.0 * TALC;
    //     error: cannot scale: this label cannot be added to itself

    // The same temperature as thermodynamics uses it: an amount with a true
    // zero, so it scales, averages and multiplies as written - and its
    // rules still keep it apart from its differences. Choose the point for a
    // thermometer reading, where doubling is a bug; choose this where the
    // physics multiplies temperatures.
    println!("\n== A label with rules (a thermodynamic temperature)");
    let (cool, warm) = (THERMODYNAMIC_CELSIUS.quantity(15.0), THERMODYNAMIC_CELSIUS.quantity(22.5));
    let rise: TemperatureStep = warm - cool;
    println!("22.5 °C - 15 °C  = {} {Kelvin}", Kelvin(rise));
    println!(
        "mean of the two  = {:.2} °C",
        THERMODYNAMIC_CELSIUS.value((cool + warm) / 2.0)
    );
    println!(
        "twice 150 K      = {} K",
        THERMODYNAMIC_KELVIN.value(2.0 * THERMODYNAMIC_KELVIN.quantity(150.0))
    );
    let t4: KelvinToTheFourth = THERMODYNAMIC_KELVIN.quantity(300.0).powi::<4>();
    println!("(300 K)⁴         = {:.3e} K⁴", KELVIN_TO_THE_FOURTH(t4));
    // let _ = rise + cool;   // a sum keeps its left operand: `cool + rise`
    //     error: cannot add: the labels have no sum
    // let _ = THERMODYNAMIC_CELSIUS.value(rise);   // a scale reads temperatures
    //     error: mismatched types

    // --- Radians: an axis -------------------------------------------------------------
    // An angle is rad¹, a solid angle rad², an angular speed rad/s and a
    // torque J/rad - so the radians cancel where physics says they do.
    println!("\n== Radians (an exponent axis)");
    let turn = Degree(90.0);
    println!(
        "90° + 45°        = {:.1} {Degree}",
        Degree(turn + Degree(45.0))
    );
    println!("90° in radians   = {:.4} {Radian}", Radian(turn));
    let patch: SolidAngle = Radian(0.1) * Radian(0.2);
    println!(
        "0.1 rad × 0.2 rad = {:.3} {Steradian}",
        Steradian(patch)
    );
    let sky = Steradian(0.25);
    println!(
        "√(0.25 sr)       = {} {Radian}",
        Radian(sky.root::<2>())
    );
    let spin = AngularSpeedUnit::RevolutionPerMinute(30.0);
    let swept: Angle = spin * Second(1.0);
    println!("30 rpm × 1 s     = {:.1} {Degree}", Degree(swept));
    let torque = NewtonMetrePerRadian(10.0);
    let work: Energy = torque * turn;
    println!("10 N·m/rad × 90° = {:.3} {Joule}", Joule(work));
    // Where geometry drops a radian, say so: an arc is r·θ / (1 rad).
    let radius = Metre(2.0);
    let arc: Length = radius * turn / Radian(1.0);
    println!("arc of 90° at 2 m = {:.4} {Metre}", Metre(arc));
    println!(
        "90° / 45°        = {}",
        ONE(turn / Degree(45.0))
    );
    // An angle is not a ratio, and a torque is not an energy:
    // let _ = turn + ONE;
    //     error: cannot add: the dimensions differ
    // let _: Energy = torque * 1.0;
    //     error: mismatched types (expected `Dim { .., rad: 0, .. }`,
    //            found `Dim { .., rad: -1, .. }`)

    // --- The rules themselves ------------------------------------------------------
    // The same rules, asked of the dimensions directly - a quantity's
    // dimension is its type's const parameter. Every operator returns an
    // `Option<Dim>`: `None` where there is no result. A dimension prints as
    // the product of its axes, then a kind that is not the ordinary one.
    println!("\n== The rules, asked of the dimensions");
    fn dim_of<const D: Dim>(_: Qty<D>) -> Dim {
        D
    }
    fn show(d: Option<Dim>) -> String {
        match d {
            Some(d) if d == Dim::DIMENSIONLESS => "1".into(),
            Some(d) => d.to_string(),
            None => "none".into(),
        }
    }
    let (length, step, temperature) = (dim_of(METRE), dim_of(KELVIN), dim_of(ICE_POINT));
    let (level, gain, apparent) = (dim_of(WATT_LEVEL), dim_of(BEL), dim_of(VOLT_AMPERE));
    let (power, hardness) = (dim_of(WATT), dim_of(QUARTZ));
    let absolute = dim_of(ZERO_KELVIN);
    #[allow(clippy::eq_op)] // `x - x` is the point
    let rules: [(&str, Option<Dim>); 19] = [
        ("m × m", length * length),
        ("(m)²", length.partial_powi(2)),
        ("√m", length.partial_root(2)),
        ("T - T", temperature - temperature),
        ("T + step", temperature + step),
        ("step + T", step + temperature),
        ("T + T", temperature + temperature),
        ("T × m", temperature * length),
        ("level - level", level - level),
        ("level + gain", level + gain),
        ("level + level", level + level),
        ("level × m", level * length),
        ("gain / m", gain / length),
        ("VA + W", apparent + power),
        ("VA × 1", apparent * Dim::DIMENSIONLESS),
        ("hardness + hardness", hardness + hardness),
        ("abs - abs", absolute - absolute),
        ("abs + abs", absolute + absolute),
        ("abs × m", absolute * length),
    ];
    for (op, result) in rules {
        println!("{op:<20} = {}", show(result));
    }
    // The labels alone, asked of the kind enum: its `*` and `/` always have
    // a result - a label survives only beside the ordinary kind.
    println!(
        "Apparent × Linear    = {}, Apparent × Apparent = {}",
        Kind::Apparent * Kind::Linear,
        Kind::Apparent * Kind::Apparent
    );
}
