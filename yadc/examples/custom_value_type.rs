//! A quantity's number may be a type of yours, not just `f64`.
//!
//! `MyF64` wraps `f64` and implements the standard operators as `const impl`s
//! (so quantities of it compute in `const` items, which is where a system's
//! constants are defined), plus `num_traits::Zero` (it seeds `Sum`), `One`
//! (`x.inv()`) and `ConstOne` (typed powers).
//!
//! Every quantity constant is a unit counted in the quantity's own numbers,
//! and a unit is called like a function: `KILOMETRE(MyF64(2.5))` is 2.5 km
//! and `KILOMETRE(d)` is a `MyF64`. A scale (°C: a zero and a step,
//! `Scale(ICE_POINT, KELVIN)`) counts in its step's numbers, so in `MyF64`
//! too. A unit enum says what its numbers are with `number = MyF64` in its
//! `#[unit(..)]`. The numbers stay `MyF64` throughout, with no `From<f64>`
//! required.
//!
//! Run with: cargo run -p yadc --example custom_value_type

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
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use yadc::{Dimension, Unit};

// --- Custom numeric newtype ---------------------------------------------------

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct MyF64(f64);

impl std::fmt::Display for MyF64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Forward the formatter, so `{:.1}` etc. reach the inner `f64`.
        std::fmt::Display::fmt(&self.0, f)
    }
}

const impl Add for MyF64 {
    type Output = Self;
    fn add(self, r: Self) -> Self {
        Self(self.0 + r.0)
    }
}
const impl Sub for MyF64 {
    type Output = Self;
    fn sub(self, r: Self) -> Self {
        Self(self.0 - r.0)
    }
}
const impl Mul for MyF64 {
    type Output = Self;
    fn mul(self, r: Self) -> Self {
        Self(self.0 * r.0)
    }
}
const impl Div for MyF64 {
    type Output = Self;
    fn div(self, r: Self) -> Self {
        Self(self.0 / r.0)
    }
}
const impl Neg for MyF64 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}
const impl AddAssign for MyF64 {
    fn add_assign(&mut self, r: Self) {
        self.0 += r.0;
    }
}
const impl SubAssign for MyF64 {
    fn sub_assign(&mut self, r: Self) {
        self.0 -= r.0;
    }
}
const impl MulAssign for MyF64 {
    fn mul_assign(&mut self, r: Self) {
        self.0 *= r.0;
    }
}
const impl DivAssign for MyF64 {
    fn div_assign(&mut self, r: Self) {
        self.0 /= r.0;
    }
}
impl yadc::num_traits::Zero for MyF64 {
    fn zero() -> Self {
        Self(0.0)
    }
    fn is_zero(&self) -> bool {
        self.0 == 0.0
    }
}
// `One` gives `x.inv()` (`1 / x`) on quantities.
impl yadc::num_traits::One for MyF64 {
    fn one() -> Self {
        Self(1.0)
    }
}
// `ConstOne` gives typed powers (`x.powi::<2>()`).
impl yadc::num_traits::ConstOne for MyF64 {
    const ONE: Self = Self(1.0);
}
// --- Unit system using MyF64 -------------------------------------------------
// The system follows the usual pattern (see `define_unit_system`): a kind
// enum, a dimension, a quantity family whose number is `MyF64`.

/// The kinds: an ordinary amount, or a point (a temperature). This system
/// only reads and compares temperatures, so a point - which cannot be
/// scaled or multiplied by mistake - is all it needs; a system that does
/// thermodynamics would make it an amount with a label of its own.
#[derive_const(PartialEq, Default)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, Dimension)]
pub enum Kind {
    #[default]
    Linear,
    #[kind(affine)]
    Affine,
}

#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, Dimension)]
pub struct Dim {
    pub m: i8,
    pub k: i8,
    #[kind]
    pub kind: Kind,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::Quantity)]
pub struct Qty<const D: Dim>(MyF64);

/// A scale: a zero and a step, a unit of the zero's type.
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

// --- Constants: the base ones are the only raw numbers -----------------------

pub const METRE: Length = Qty(MyF64(1.0));
pub const KELVIN: TemperatureStep = Qty(MyF64(1.0));
pub const ABSOLUTE_ZERO: Temperature = Qty(MyF64(0.0));

pub const KILOMETRE: Length = MyF64(1000.0) * METRE;
pub const SQUARE_METRE: Area = METRE * METRE;
pub const SQUARE_KILOMETRE: Area = KILOMETRE * KILOMETRE;
pub const ICE_POINT: Temperature = ABSOLUTE_ZERO + MyF64(273.15) * KELVIN;

// --- Units count in `MyF64` ---------------------------------------------------

/// Degrees Celsius: steps of one kelvin from the ice point. A scale counts
/// in its step's numbers: `MyF64`.
pub const CELSIUS: Scale<Temperature, TemperatureStep> = Scale(ICE_POINT, KELVIN);

/// Kelvin, on the absolute scale: steps of one kelvin from absolute zero.
pub const KELVIN_SCALE: Scale<Temperature, TemperatureStep> = Scale(ABSOLUTE_ZERO, KELVIN);

/// The scales by name, for printing and for a menu.
#[derive(Copy, Clone, Unit)]
#[unit(of = Temperature, number = MyF64)]
pub enum TemperatureUnit {
    #[unit("K", "kelvin", by = KELVIN_SCALE)]
    Kelvin,
    #[unit("°C", "degrees Celsius", by = CELSIUS)]
    Celsius,
}

/// A choice of units, for a menu.
#[derive(Copy, Clone, Unit)]
#[unit(of = Length, number = MyF64)]
pub enum LengthUnit {
    #[unit("m", "metres", by = METRE)]
    Metre,
    #[unit("km", "kilometres", by = KILOMETRE)]
    Kilometre,
}

#[derive(Copy, Clone, Unit)]
#[unit(of = Area, number = MyF64)]
pub enum AreaUnit {
    #[unit("m²", "square metres", by = SQUARE_METRE)]
    SquareMetre,
    #[unit("km²", "square kilometres", by = SQUARE_KILOMETRE)]
    SquareKilometre,
}

// Units are `const`, in `MyF64` too.
const BOILING: Temperature = CELSIUS.quantity(MyF64(100.0));
const BOILING_IN_KELVIN: MyF64 = KELVIN_SCALE.value(BOILING);

fn main() {
    // A unit is called: on a number it builds, on a quantity it reads. A
    // value of a unit enum is called the same way.
    let km = LengthUnit::Kilometre;
    let side = km(MyF64(2.5));
    let area: Area = side * side;
    // Reading a quantity in a unit gives a `MyF64` again; the unit prints
    // its symbol with `{}`, its name with `{:#}`.
    println!("side = {} {km:#}", km(side));
    let m2 = AreaUnit::SquareMetre;
    println!("area = {:.0} {m2}", m2(area));
    // A constant is a unit too, counting in `MyF64`.
    let in_km2: MyF64 = SQUARE_KILOMETRE(area);
    println!("area in km² = {in_km2}");
    // A quantity prints itself as its number and its dimension.
    println!("area itself = {area}");

    // The zero point is the scale's business, computed in `MyF64` too.
    let (celsius, kelvin) = (TemperatureUnit::Celsius, TemperatureUnit::Kelvin);
    let body = CELSIUS.quantity(MyF64(36.6));
    println!(
        "body = {:.1} {celsius} (= {:.2} {kelvin})",
        CELSIUS.value(body),
        KELVIN_SCALE.value(body)
    );

    // Two temperatures differ by a step, which a step constant reads.
    let fever: TemperatureStep = celsius(MyF64(38.6)) - body;
    println!("fever - body = {:.1} K", KELVIN(fever));
    println!(
        "boiling (const) = {:.2} {celsius} = {BOILING_IN_KELVIN:.2} {kelvin}",
        CELSIUS.value(BOILING)
    );

    // The `Unit` trait is still there, for generic code and `&dyn`.
    let any: &dyn Unit<Length, MyF64> = &KILOMETRE;
    println!("side = {} km (through `&dyn Unit`)", any.value(side));
}
