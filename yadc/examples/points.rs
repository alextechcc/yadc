//! Points: values whose zero someone chose - altitudes, clock times - and
//! why they are not quantities.
//!
//! A *quantity* has a zero that is a fact. Zero length is no length at all,
//! so "twice as long" means the same thing to everyone: a 3 m plank is twice
//! a 1.5 m plank, whoever measures it.
//!
//! An altitude is different. "5 500 ft" means 5 500 ft *above something* -
//! mean sea level, the runway, the ground under the aircraft - and that
//! something is a convention. Try doubling one: an aircraft 500 ft above a
//! runway that sits 5 000 ft above the sea is 5 500 ft above the sea.
//! Doubling the first number puts it 1 000 ft above the runway (6 000 ft above
//! the sea); doubling the second puts it at 11 000 ft above the sea. Same
//! aircraft, same "×2", two different places. The answer depends on where the
//! zero was put, so `2 × altitude` has no meaning, and neither has
//! `altitude + altitude` or `altitude × altitude`.
//!
//! What *does* have a meaning:
//!
//! - **point − point** is a quantity: the aircraft is 1 500 ft above the
//!   terrain whatever zero you count both from. An `Altitude` minus an
//!   `Altitude` is a `Length`.
//! - **point ± quantity** is a point: climb 500 ft and you are somewhere else.
//!   `Altitude ± Length` is an `Altitude`.
//! - Anything built from those: the midpoint of `a` and `b` is
//!   `a + (b - a) / 2`, which only ever halves a `Length`.
//!
//! **A point is a kind of quantity.** The dimension below has a kind, an enum
//! of ours with two values: `Linear` (the default) for an amount, `Affine`
//! for a point. `#[kind(affine)]` marks `Affine` as the point, and
//! `#[derive(Dimension)]` writes the rules above into the dimension's own
//! operators, so the quantity type follows them with no code of ours: an
//! altitude is a `Qty` like any other, just one that cannot be scaled or
//! multiplied. A clock time works the same way: two instants differ by a
//! duration, and "twice an instant" means nothing.
//!
//! **Which zero is not in the type: it is in the unit.** There is one line of
//! altitudes and one line of instants. A zero is a constant on its line -
//! mean sea level is a base constant of ours, `SEA_LEVEL`; the Unix epoch
//! another, `UNIX_EPOCH`; the GPS epoch a point ten years later on the same
//! line - and counting from one is a scale, a zero and a step:
//! `const FEET_AMSL: Scale<Altitude, Length> = Scale(SEA_LEVEL, FOOT);`. Feet
//! above *this* runway is the same with the runway as its zero,
//! `Scale(runway, FOOT)`, made at run time; GPS seconds and Unix seconds are
//! two scales reading the same instant, as °C and °F read the same
//! temperature. A scale is a unit: `FEET_AMSL.quantity(5_000.0)` is an
//! altitude, `FEET_AMSL.value(aircraft)` a number of feet.
//!
//! Run with: cargo run -p yadc --example points
// This file defines its own kind, dimension and quantity type, so it needs
// the definer's feature set.
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
use core::ops::{Add, Mul, Sub};

use yadc::{Dimension, Quantity, Unit};

// --- The system -------------------------------------------------------------------

/// How a quantity behaves: an amount, like a length or a duration, whose
/// zero is a fact - or a point, whose zero somebody chose.
#[derive_const(PartialEq, Default)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
pub enum Kind {
    #[default]
    Linear,
    #[kind(affine)]
    Affine,
}

/// Two exponent axes, metres and seconds, and the kind.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
pub struct Dim {
    pub m: i8,
    pub s: i8,
    #[kind]
    pub kind: Kind,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::Quantity)]
pub struct Qty<const D: Dim>(f64);

/// A scale: a zero and a step. A unit of the zero's type, counting steps
/// from the zero - which may be a constant, or a point known at run time.
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

// Amounts.
pub type Length = Qty<
    {
        Dim {
            m: 1,
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

// Points: the same exponents, the affine kind.
pub type Altitude = Qty<
    {
        Dim {
            m: 1,
            kind: Kind::Affine,
            ..Dim::DIMENSIONLESS
        }
    },
>;
pub type Instant = Qty<
    {
        Dim {
            s: 1,
            kind: Kind::Affine,
            ..Dim::DIMENSIONLESS
        }
    },
>;

// --- Constants ----------------------------------------------------------------------
// The base constants are the only raw numbers. Each line of points has one:
// the point every other point of its line is counted from.

pub const METRE: Length = Qty(1.0);
pub const SECOND: Time = Qty(1.0);
/// Mean sea level.
pub const SEA_LEVEL: Altitude = Qty(0.0);
/// The Unix epoch (1 January 1970).
pub const UNIX_EPOCH: Instant = Qty(0.0);

// Everything else is defined from them.
pub const FOOT: Length = 0.3048 * METRE;
pub const MINUTE: Time = 60.0 * SECOND;
pub const FOOT_PER_MINUTE: Velocity = FOOT / MINUTE;
/// The GPS epoch (6 January 1980): ten years and five days after the Unix
/// epoch (ignoring leap seconds, which GPS time does not have). Another zero
/// on the same line.
pub const GPS_EPOCH: Instant = UNIX_EPOCH + 315_964_800.0 * SECOND;

// --- Units ------------------------------------------------------------------------
// A point is read with a scale: a zero and a step, `Scale(zero, step)`.

/// Feet above mean sea level.
const FEET_AMSL: Scale<Altitude, Length> = Scale(SEA_LEVEL, FOOT);

/// Metres above mean sea level.
const METRES_AMSL: Scale<Altitude, Length> = Scale(SEA_LEVEL, METRE);

/// Seconds of GPS time, and of Unix time: two scales on the line of
/// instants.
const GPS_SECONDS: Scale<Instant, Time> = Scale(GPS_EPOCH, SECOND);
const UNIX_SECONDS: Scale<Instant, Time> = Scale(UNIX_EPOCH, SECOND);

/// The altitude scales by name: a choice, for a menu, which prints.
#[derive(Copy, Clone, yadc::Unit)]
#[unit(of = Altitude)]
enum AltitudeUnit {
    #[unit("ft AMSL", "feet above mean sea level", by = FEET_AMSL)]
    FeetAmsl,
    #[unit("m AMSL", "metres above mean sea level", by = METRES_AMSL)]
    MetresAmsl,
}

/// The amounts' units: a choice, for a menu.
#[derive(Copy, Clone, yadc::Unit)]
#[unit(of = Length)]
enum LengthUnit {
    #[unit("m", "metres", by = METRE)]
    Metre,
    #[unit("ft", "feet", by = FOOT)]
    Foot,
}

#[derive(Copy, Clone, yadc::Unit)]
#[unit("min", "minutes", of = Time, by = MINUTE)]
struct Minute;

#[derive(Copy, Clone, yadc::Unit)]
#[unit("ft/min", "feet per minute", of = Velocity, by = FOOT_PER_MINUTE)]
struct FootPerMinute;

// Units are `const`, for points as for quantities.
const FL100: Altitude = FEET_AMSL.quantity(10_000.0);

// --- A height bound to its reference --------------------------------------------

/// "So high above *that*": a reference point and a height above it. Unlike a
/// bare altitude, this has a real zero - the reference - so scaling the height
/// means something: twice as high above the runway is twice the height, and
/// the reference stays put. `position()` is where it actually is.
#[derive(Copy, Clone, Debug)]
struct HeightAbove {
    reference: Altitude,
    height: Length,
}

impl HeightAbove {
    fn position(self) -> Altitude {
        self.reference + self.height
    }
}

impl Mul<f64> for HeightAbove {
    type Output = Self;
    fn mul(self, k: f64) -> Self {
        Self {
            reference: self.reference,
            height: self.height * k,
        }
    }
}

/// Halfway between two points, of any kind: `P` is anything whose difference
/// is a quantity `L` that moves it. The point itself is never scaled - only
/// the quantity is, and a `Quantity` can be divided by its numbers.
fn midpoint<P, L>(a: P, b: P) -> P
where
    P: Copy + Sub<Output = L> + Add<L, Output = P>,
    L: Quantity<Number = f64>,
{
    a + (b - a) / 2.0
}

fn main() {
    use AltitudeUnit::{FeetAmsl, MetresAmsl};
    use LengthUnit::{Foot, Metre};

    // --- 1. Build points through units, read them through units -------------
    // Feet above the runway: a scale whose zero is the runway, made here.
    let runway = FEET_AMSL.quantity(5_000.0);
    let agl = Scale(runway, FOOT);
    let aircraft = agl.quantity(1_500.0); // 1 500 ft above *this* runway
    println!(
        "aircraft: {:.0} ft AGL = {:.1} {MetresAmsl} = {:.0} {FeetAmsl}",
        agl.value(aircraft),
        MetresAmsl(aircraft),
        FeetAmsl(aircraft),
    );
    // A point prints itself too: its number and its dimension, kind and all.
    println!("aircraft itself: {aircraft:.1}");

    // --- 2. point - point is an amount ----------------------------------------
    let terrain = FEET_AMSL.quantity(5_800.0); // a ridge under the approach
    let clearance: Length = aircraft - terrain;
    println!(
        "clearance over the ridge: {:.0} {Foot} = {:.1} {Metre}",
        Foot(clearance),
        Metre(clearance),
    );
    // Feet above the ground under the aircraft: the ridge as the zero.
    let above_ridge = Scale(terrain, FOOT);
    println!(
        "height above the ridge: {:.0} ft above the ridge",
        above_ridge.value(aircraft)
    );

    // --- 3. point ± amount is a point ------------------------------------------
    let climb = Foot(500.0);
    let higher: Altitude = aircraft + climb;
    let lower: Altitude = aircraft - climb;
    println!(
        "climb 500 ft: {:.0} {FeetAmsl}; descend 500 ft: {:.0} {FeetAmsl}",
        FeetAmsl(higher),
        FeetAmsl(lower),
    );
    // The clearance, added back onto the ridge, is the aircraft again; in
    // place, too.
    assert_eq!(terrain + clearance, aircraft);
    let mut level = terrain;
    level += clearance;
    assert_eq!(level, aircraft);

    // --- 4. The midpoint only ever halves a distance --------------------------
    let mid = runway + (aircraft - runway) / 2.0;
    assert_eq!(mid, midpoint(runway, aircraft));
    println!(
        "halfway down to the runway: {:.0} {FeetAmsl} = {:.0} ft AGL",
        FeetAmsl(mid),
        agl.value(mid),
    );

    // --- 5. A height bound to a reference *can* be scaled ----------------------
    let pattern = HeightAbove {
        reference: runway,
        height: Foot(1_000.0),
    };
    let twice = pattern * 2.0;
    println!(
        "pattern: {:.0} ft above the runway, at {:.0} ft AMSL; twice as high: {:.0} ft, at {:.0} ft AMSL",
        Foot(pattern.height),
        FeetAmsl(pattern.position()),
        Foot(twice.height),
        FeetAmsl(twice.position()),
    );

    // --- 6. Clock times: the same rules for time --------------------------------
    // Two instants differ by a duration; a duration divides a distance.
    let takeoff = GPS_SECONDS.quantity(1_400_000_000.0);
    let level_off = takeoff + Minute(12.0);
    let cruise = FEET_AMSL.quantity(11_000.0);
    let elapsed: Time = level_off - takeoff;
    let climb_rate: Velocity = (cruise - runway) / elapsed;
    println!(
        "took off at {:.0} s GPS: {:.0} {Minute} of climbing at {:.0} {FootPerMinute}",
        GPS_SECONDS.value(takeoff),
        Minute(elapsed),
        FootPerMinute(climb_rate),
    );
    println!(
        "the midpoint of the climb is at {:.0} s GPS",
        GPS_SECONDS.value(midpoint(takeoff, level_off)),
    );
    // GPS seconds and Unix seconds read the same instant from two zeros, so
    // the take-off needs no conversion to be read on the Unix clock - and an
    // instant built from Unix seconds subtracts from it like any other.
    println!(
        "on the Unix clock: {:.0} s Unix",
        UNIX_SECONDS.value(takeoff)
    );
    let logged = UNIX_SECONDS.quantity(1_715_964_000.0);
    println!(
        "logged {:.0} {Minute} before take-off",
        Minute(takeoff - logged)
    );
    println!("FL100 (a const) is {:.0} ft AGL", agl.value(FL100));

    // --- 7. Every value prints its kind ------------------------------------------
    // A quantity's `Display` is its number and its dimension, and a
    // dimension prints a kind that is not the ordinary one in braces: points
    // and amounts side by side. A point's number is counted from its line's
    // base constant - sea level, the Unix epoch.
    let values: [(&str, &dyn core::fmt::Display); 4] = [
        ("aircraft", &aircraft),
        ("clearance", &clearance),
        ("take-off", &takeoff),
        ("elapsed", &elapsed),
    ];
    for (name, value) in values {
        println!("{name:>9}: {value:.0}");
    }

    // --- 8. What does not compile ---------------------------------------------
    // Each answer would depend on where the zero was put, so there is no impl,
    // and the compiler says why.
    //
    // let _ = 2.0 * aircraft;
    //     error: a point cannot be scaled
    // let _ = aircraft * aircraft;
    //     error: cannot multiply: these have no product
    // let _ = aircraft + aircraft;
    //     error: cannot add a point to a point
    // let _ = climb + aircraft;   // a sum keeps its left operand: `aircraft + climb`
    //     error: cannot add a point to an amount
    // let _ = takeoff - runway;   // an instant and an altitude
    //     error: cannot subtract: the dimensions differ
}
