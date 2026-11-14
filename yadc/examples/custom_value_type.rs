//! A quantity's number may be a type of yours, not just `f64`.
//!
//! `MyF64` wraps `f64` and implements the standard operators as `const impl`s
//! (so quantities of it compute in `const` items, which is where a system's
//! constants are defined), plus `num_traits::Zero` (it seeds `Sum`), `One`
//! (`x.inv()`) and `ConstOne` (typed powers).
//!
//! Every quantity constant is a unit counted in the quantity's own numbers:
//! `KILOMETRE.quantity(MyF64(2.5))` is 2.5 km and `KILOMETRE.value(d)` is a
//! `MyF64`. A unit type of ours (`Celsius`: kelvins from the ice point)
//! counts in `MyF64` by its `impl Unit<Temperature, MyF64>`, and an enum of
//! units (`dynamickit::Values`) delegates `Unit<Temperature, MyF64>`. The
//! numbers stay `MyF64` throughout, with no `From<f64>` required.
//!
//! Run with: cargo run -p yadc --example custom_value_type

#![feature(generic_const_exprs, min_adt_const_params, const_trait_impl)]
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

impl const Add for MyF64 {
    type Output = Self;
    fn add(self, r: Self) -> Self {
        Self(self.0 + r.0)
    }
}
impl const Sub for MyF64 {
    type Output = Self;
    fn sub(self, r: Self) -> Self {
        Self(self.0 - r.0)
    }
}
impl const Mul for MyF64 {
    type Output = Self;
    fn mul(self, r: Self) -> Self {
        Self(self.0 * r.0)
    }
}
impl const Div for MyF64 {
    type Output = Self;
    fn div(self, r: Self) -> Self {
        Self(self.0 / r.0)
    }
}
impl const Neg for MyF64 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}
impl const AddAssign for MyF64 {
    fn add_assign(&mut self, r: Self) {
        self.0 += r.0;
    }
}
impl const SubAssign for MyF64 {
    fn sub_assign(&mut self, r: Self) {
        self.0 -= r.0;
    }
}
impl const MulAssign for MyF64 {
    fn mul_assign(&mut self, r: Self) {
        self.0 *= r.0;
    }
}
impl const DivAssign for MyF64 {
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

// --- Constants: the base ones write the field, the rest are built from them --

pub const METRE: Length = Qty(MyF64(1.0));
pub const KELVIN: TemperatureStep = Qty(MyF64(1.0));
pub const ABSOLUTE_ZERO: Temperature = Qty(MyF64(0.0));

pub const KILOMETRE: Length = MyF64(1000.0) * METRE;
pub const SQUARE_METRE: Area = METRE * METRE;
pub const SQUARE_KILOMETRE: Area = KILOMETRE * KILOMETRE;
pub const ICE_POINT: Temperature = ABSOLUTE_ZERO + MyF64(273.15) * KELVIN;

// --- Units count in `MyF64` ---------------------------------------------------

/// A unit type of ours: `n` of it is `n` steps, from a zero if it has one.
/// It counts in `MyF64` because its constants do, and prints its symbol
/// (`{}`) or name (`{:#}`). (`define_unit_system` writes such impls out by
/// hand.)
macro_rules! unit {
    ($name:ident: $q:ty = $step:expr, zero $zero:expr, $symbol:literal, $long:literal) => {
        unit!(@display $name, $symbol, $long);
        impl const Unit<$q, MyF64> for $name {
            fn quantity(&self, value: MyF64) -> $q {
                $zero + $step.quantity(value)
            }
            fn value(&self, quantity: $q) -> MyF64 {
                $step.value(quantity - $zero)
            }
        }
    };
    ($name:ident: $q:ty = $step:expr, $symbol:literal, $long:literal) => {
        unit!(@display $name, $symbol, $long);
        impl const Unit<$q, MyF64> for $name {
            fn quantity(&self, value: MyF64) -> $q {
                $step.quantity(value)
            }
            fn value(&self, quantity: $q) -> MyF64 {
                $step.value(quantity)
            }
        }
    };
    (@display $name:ident, $symbol:literal, $long:literal) => {
        #[derive(Copy, Clone)]
        pub struct $name;
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.pad(if f.alternate() { $long } else { $symbol })
            }
        }
    };
}

// Kelvin, on the absolute scale, and degrees Celsius: steps of one kelvin
// from absolute zero and from the ice point.
unit!(KelvinScale: Temperature = KELVIN, zero ABSOLUTE_ZERO, "K", "kelvin");
unit!(Celsius: Temperature = KELVIN, zero ICE_POINT, "°C", "degrees Celsius");
unit!(Metre: Length = METRE, "m", "metres");
unit!(Kilometre: Length = KILOMETRE, "km", "kilometres");
unit!(SquareMetre: Area = SQUARE_METRE, "m²", "square metres");
unit!(SquareKilometre: Area = SQUARE_KILOMETRE, "km²", "square kilometres");

/// The temperature units, for printing and for a menu.
#[derive(Copy, Clone, dynamickit::Values)]
#[values(delegate(Unit<Temperature, MyF64> {
    fn quantity(&self, value: MyF64) -> Temperature;
    fn value(&self, quantity: Temperature) -> MyF64;
}))]
pub enum TemperatureUnit {
    #[value(KelvinScale)]
    Kelvin,
    #[value(Celsius)]
    Celsius,
}

/// A choice of units, for a menu.
#[derive(Copy, Clone, dynamickit::Values)]
#[values(delegate(Unit<Length, MyF64> {
    fn quantity(&self, value: MyF64) -> Length;
    fn value(&self, quantity: Length) -> MyF64;
}))]
pub enum LengthUnit {
    #[value(Metre)]
    Metre,
    #[value(Kilometre)]
    Kilometre,
}

#[derive(Copy, Clone, dynamickit::Values)]
#[values(delegate(Unit<Area, MyF64> {
    fn quantity(&self, value: MyF64) -> Area;
    fn value(&self, quantity: Area) -> MyF64;
}))]
pub enum AreaUnit {
    #[value(SquareMetre)]
    SquareMetre,
    #[value(SquareKilometre)]
    SquareKilometre,
}

// Units are `const`, in `MyF64` too.
const BOILING: Temperature = Celsius.quantity(MyF64(100.0));
const BOILING_IN_KELVIN: MyF64 = KelvinScale.value(BOILING);

fn main() {
    // A unit builds with `quantity` and reads with `value`: a value of a unit
    // enum, and a quantity constant alike.
    let km = LengthUnit::Kilometre;
    let side = km.quantity(MyF64(2.5));
    let area: Area = side * side;
    // Reading a quantity in a unit gives a `MyF64` again; the unit prints
    // its symbol with `{}`, its name with `{:#}`.
    println!("side = {} {km:#}", km.value(side));
    let m2 = AreaUnit::SquareMetre;
    println!("area = {:.0} {m2}", m2.value(area));
    // A constant is a unit too, counting in `MyF64`.
    let in_km2: MyF64 = SQUARE_KILOMETRE.value(area);
    println!("area in km² = {in_km2}");
    // A quantity prints itself as its number and its dimension.
    println!("area itself = {area}");

    // The zero point is the unit's business, computed in `MyF64` too.
    let (celsius, kelvin) = (TemperatureUnit::Celsius, TemperatureUnit::Kelvin);
    let body = Celsius.quantity(MyF64(36.6));
    println!(
        "body = {:.1} {celsius} (= {:.2} {kelvin})",
        Celsius.value(body),
        KelvinScale.value(body)
    );

    // Two temperatures differ by a step, which a step constant reads.
    let fever: TemperatureStep = celsius.quantity(MyF64(38.6)) - body;
    println!("fever - body = {:.1} K", KELVIN.value(fever));
    println!(
        "boiling (const) = {:.2} {celsius} = {BOILING_IN_KELVIN:.2} {kelvin}",
        Celsius.value(BOILING)
    );

    // A unit is the `Unit` trait, so it also goes behind `&dyn`.
    let any: &dyn Unit<Length, MyF64> = &KILOMETRE;
    println!("side = {} km (through `&dyn Unit`)", any.value(side));
}
