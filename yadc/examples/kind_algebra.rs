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
//!   amperes) is not real power (watts), though both are kg·m²·s⁻³; and a
//!   hardness number orders minerals but has no differences, so its rules
//!   say it does not add or subtract at all.
//!
//! Angles are not a kind but an axis (`rad`), so a solid angle is rad² and a
//! torque is J/rad. Each section prints what works; what does not is shown
//! as commented-out code with the compiler's message. A quantity constant or
//! a unit enum's value is called like a function throughout: on a number it
//! builds, on a quantity it reads.
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
use core::ops::{Add, Sub};
use yadc::num_traits::Inv;
use yadc::{Dimension, PowiAs, RootAs, Unit};

// --- The kinds ---------------------------------------------------------------
// The `#[default]` variant is the ordinary kind, an amount. `#[kind(affine)]`
// and `#[kind(log)]` mark the point and the logarithm. Any other variant is a
// label of ours: by default it adds to and subtracts from itself and nothing
// else, and `#[kind(add(..) = .., sub(..) = ..)]` changes that - `= invalid`
// for "no result". Products and quotients need no rules: a label survives
// only beside a plain number.

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

/// A scale: a zero and a step. `n` is `zero + n · step`, and a value reads
/// `(value - zero) / step`.
#[derive(Copy, Clone)]
pub struct Scale<P, S>(pub P, pub S);

impl<P, S, N> const Unit<P, N> for Scale<P, S>
where
    P: Copy + [const] Sub + [const] Add<<P as Sub>::Output, Output = P>,
    S: [const] Unit<<P as Sub>::Output, N>,
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
pub type Length = Qty<{ Dim { m: 1, ..Dim::DIMENSIONLESS } }>;
pub type Area = Qty<{ Dim { m: 2, ..Dim::DIMENSIONLESS } }>;
pub type Time = Qty<{ Dim { s: 1, ..Dim::DIMENSIONLESS } }>;
/// A temperature as a thermometer reads it: a point on the line of kelvins.
pub type Temperature = Qty<{ Dim { k: 1, kind: Kind::Affine, ..Dim::DIMENSIONLESS } }>;
/// A temperature step: an amount.
pub type TemperatureStep = Qty<{ Dim { k: 1, ..Dim::DIMENSIONLESS } }>;
pub type Angle = Qty<{ Dim { rad: 1, ..Dim::DIMENSIONLESS } }>;
pub type SolidAngle = Qty<{ Dim { rad: 2, ..Dim::DIMENSIONLESS } }>;
pub type AngularSpeed = Qty<{ Dim { s: -1, rad: 1, ..Dim::DIMENSIONLESS } }>;
pub type Mass = Qty<{ Dim { kg: 1, ..Dim::DIMENSIONLESS } }>;
pub type Energy = Qty<{ Dim { m: 2, kg: 1, s: -2, ..Dim::DIMENSIONLESS } }>;
/// Real power: watts.
pub type Power = Qty<{ Dim { m: 2, kg: 1, s: -3, ..Dim::DIMENSIONLESS } }>;
/// Apparent power: the same exponents as `Power`, a label of its own.
pub type ApparentPower =
    Qty<{ Dim { m: 2, kg: 1, s: -3, kind: Kind::Apparent, ..Dim::DIMENSIONLESS } }>;
/// A power level: the logarithm of a power.
pub type PowerLevel = Qty<{ Dim { m: 2, kg: 1, s: -3, kind: Kind::Log, ..Dim::DIMENSIONLESS } }>;
/// A gain: the logarithm of a plain ratio.
pub type Gain = Qty<{ Dim { kind: Kind::Log, ..Dim::DIMENSIONLESS } }>;
/// A hardness number on the Mohs scale.
pub type Hardness = Qty<{ Dim { kind: Kind::Hardness, ..Dim::DIMENSIONLESS } }>;
/// Energy per radian: torque × angle is the work done turning.
pub type Torque = Qty<{ Dim { m: 2, kg: 1, s: -2, rad: -1, ..Dim::DIMENSIONLESS } }>;

// --- Constants: the base ones, and the rest from them ---------------------------
// A base constant is a raw number, written where the field is visible: one
// per axis, the zero of each line of points and levels, and one per label -
// a kind is never computed, a constant that has it gives it.

pub const ONE: Ratio = Qty(1.0);
pub const METRE: Length = Qty(1.0);
pub const SECOND: Time = Qty(1.0);
pub const KELVIN: TemperatureStep = Qty(1.0);
pub const ABSOLUTE_ZERO: Temperature = Qty(0.0);
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
/// Torque is energy per radian of turn.
pub const NEWTON_METRE_PER_RADIAN: Torque = JOULE / RADIAN;

/// Degrees Celsius: steps of one kelvin from the ice point.
pub const CELSIUS: Scale<Temperature, TemperatureStep> = Scale(ICE_POINT, KELVIN);
/// Kelvin, as a reading of a temperature: from absolute zero.
pub const KELVIN_SCALE: Scale<Temperature, TemperatureStep> = Scale(ABSOLUTE_ZERO, KELVIN);
/// A scale of levels: decibels from one milliwatt.
pub const DBM: Scale<PowerLevel, Gain> = Scale(MILLIWATT_LEVEL, DECIBEL);

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

// --- Unit enums, for the units that print -----------------------------------

#[derive(Copy, Clone, Unit)]
#[unit(of = Temperature)]
pub enum TemperatureUnit {
    #[unit("K", "kelvin", by = KELVIN_SCALE)]
    KelvinScale,
    #[unit("°C", "degrees Celsius", by = CELSIUS)]
    Celsius,
}

#[derive(Copy, Clone, Unit)]
#[unit(of = Angle)]
pub enum AngleUnit {
    #[unit("rad", "radians", by = RADIAN)]
    Radian,
    #[unit("°", "degrees", by = DEGREE)]
    Degree,
}

#[derive(Copy, Clone, Unit)]
#[unit(of = AngularSpeed)]
pub enum AngularSpeedUnit {
    #[unit("rad/s", "radians per second", by = RADIAN_PER_SECOND)]
    RadianPerSecond,
    #[unit("rpm", "revolutions per minute", by = RPM)]
    RevolutionPerMinute,
}

fn main() {
    use AngleUnit::{Degree, Radian};

    // --- Amounts: every product and quotient exists --------------------------------
    // Exponents add under `*` and subtract under `/`, whatever they are, so
    // every power and every reciprocal of an amount exists too.
    println!("== Amounts: every product and quotient exists");
    let a = METRE(3.0);
    let b = METRE(4.0);
    println!("3 m × 4 m        = {} m²", SQUARE_METRE(a * b));
    println!("(3 m)²           = {} m²", SQUARE_METRE(a.powi::<2>()));
    // A ratio of two lengths is a plain number, read in the unit `ONE`.
    println!("3 m / 4 m        = {}", ONE(a / b));
    let floor = SQUARE_METRE(16.0);
    println!("√(16 m²)         = {} m", METRE(floor.root::<2>()));
    let per_radian = Radian(2.0).inv();
    println!("1 / (2 rad) × 1 rad = {}", ONE(per_radian * Radian(1.0)));
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
    use TemperatureUnit::Celsius;
    let morning = CELSIUS.quantity(15.0);
    let noon = CELSIUS.quantity(22.5);
    let warming: TemperatureStep = noon - morning;
    println!("22.5 °C - 15 °C  = {} K", KELVIN(warming));
    println!("15 °C + 7.5 K    = {} {Celsius}", Celsius(morning + warming));
    println!("22.5 °C - 7.5 K  = {} {Celsius}", Celsius(noon - warming));
    let mean: Temperature = morning + (noon - morning) / 2.0;
    println!("mean of the two  = {:.2} {Celsius}", Celsius(mean));
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
    //     error: a point or a logarithm has no roots

    // --- A logarithm: adding is multiplying -------------------------------------------
    // A power level is the logarithm of a power, a gain the logarithm of a
    // plain ratio. `+` and `-` add and subtract the exponents, as `*` and `/`
    // do for the things themselves: level - level is watts over watts, a
    // gain; level ± gain is watts times a ratio, a level. `DBM` is a scale on
    // the line of levels - a zero and a step - and the watts are one `From`
    // away.
    println!("\n== A logarithm (levels and gains)");
    let sent = DBM.quantity(30.0);
    let received = sent - DECIBEL(3.0) + DECIBEL(13.0);
    println!("30 dBm - 3 dB + 13 dB = {:.0} dBm", DBM.value(received));
    let gain: Gain = received - sent;
    println!("received - sent  = {:.0} dB", DECIBEL(gain));
    println!("40 dBm           = {:.0} W", WATT(Power::from(received)));
    let two_watts = WATT(2.0);
    println!("2 W              = {:.2} dBm", DBM.value(PowerLevel::from(two_watts)));
    // A gain per length is a plain rate - bels per metre - and that many
    // bels is a gain again: a kind is given by a constant that has it.
    let fibre = DECIBEL(0.2) / KILOMETRE;
    let loss: Gain = (fibre * KILOMETRE(15.0)) * BEL;
    println!("0.2 dB/km × 15 km = {:.1} dB", DECIBEL(loss));
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
    let load: ApparentPower = VOLT_AMPERE(500.0);
    let more: ApparentPower = VOLT_AMPERE(250.0);
    println!("500 VA + 250 VA  = {} VA", VOLT_AMPERE(load + more));
    println!("500 VA × 1.5     = {} VA", VOLT_AMPERE(load * 1.5));
    let real: Power = WATT(400.0);
    println!("400 W / 500 VA   = {}   (a power factor)", ONE(real / load));
    let used: Energy = load * SECOND(60.0);
    println!("500 VA × 60 s    = {} J   (the label drops)", JOULE(used));
    // let _ = real + load;
    //     error: cannot add: the labels have no sum

    // A hardness number orders minerals, and that is all: its rules say it
    // has no sum and no difference, so it cannot be scaled either.
    println!("\n== A label with rules (hardness)");
    println!("talc < quartz < diamond: {}", TALC < QUARTZ && QUARTZ < DIAMOND);
    println!("quartz itself    = {QUARTZ}");
    // let _ = QUARTZ + TALC;
    //     error: cannot add: the labels have no sum
    // let _ = DIAMOND - QUARTZ;
    //     error: cannot subtract: the labels have no difference
    // let _ = 2.0 * TALC;
    //     error: cannot scale: this label cannot be added to itself

    // --- Radians: an axis -------------------------------------------------------------
    // An angle is rad¹, a solid angle rad², an angular speed rad/s and a
    // torque J/rad - so the radians cancel where physics says they do.
    println!("\n== Radians (an exponent axis)");
    let turn = Degree(90.0);
    println!("90° + 45°        = {:.1} {Degree}", Degree(turn + Degree(45.0)));
    println!("90° in radians   = {:.4} {Radian}", Radian(turn));
    let patch: SolidAngle = Radian(0.1) * Radian(0.2);
    println!("0.1 rad × 0.2 rad = {:.3} sr", STERADIAN(patch));
    let sky = STERADIAN(0.25);
    println!("√(0.25 sr)       = {} {Radian}", Radian(sky.root::<2>()));
    let spin = AngularSpeedUnit::RevolutionPerMinute(30.0);
    let swept: Angle = spin * SECOND(1.0);
    println!("30 rpm × 1 s     = {:.1} {Degree}", Degree(swept));
    let torque = NEWTON_METRE_PER_RADIAN(10.0);
    let work: Energy = torque * turn;
    println!("10 N·m/rad × 90° = {:.3} J", JOULE(work));
    // Where geometry drops a radian, say so: an arc is r·θ / (1 rad).
    let radius = METRE(2.0);
    let arc: Length = radius * turn / Radian(1.0);
    println!("arc of 90° at 2 m = {:.4} m", METRE(arc));
    println!("90° / 45°        = {}", ONE(turn / Degree(45.0)));
    // An angle is not a ratio, and a torque is not an energy:
    // let _ = turn + ONE;
    //     error: cannot add: the dimensions differ
    // let _: Energy = torque * 1.0;
    //     error: mismatched types (expected `Dim { .., rad: 0, .. }`,
    //            found `Dim { .., rad: -1, .. }`)
}
