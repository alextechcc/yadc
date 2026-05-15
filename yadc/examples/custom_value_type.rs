//! Demonstrates that yadc supports truly arbitrary value types.
//!
//! `MyF64` wraps `f64` and implements the required standard traits, including
//! `num_traits::One` / `Zero` (the multiplicative/additive identities the
//! `StaticQuantity` derive needs to read values back and to seed `Sum`). The `#[linear(...)]` /
//! `#[affine(...)]` scales are written as `MyF64(...)` so the compiler can type
//! them against `Mul<MyF64>` - no `From<f64>` required.

#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(unsized_const_params)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(const_cmp)]
#![feature(structural_match)]
#![allow(incomplete_features)]

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use yadc::{Dimension, Format, StaticQuantity, Unit};

// --- Custom numeric newtype ---------------------------------------------------

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct MyF64(f64);

impl std::fmt::Display for MyF64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Add for MyF64 {
    type Output = Self;
    fn add(self, r: Self) -> Self {
        Self(self.0 + r.0)
    }
}
impl Sub for MyF64 {
    type Output = Self;
    fn sub(self, r: Self) -> Self {
        Self(self.0 - r.0)
    }
}
impl Mul for MyF64 {
    type Output = Self;
    fn mul(self, r: Self) -> Self {
        Self(self.0 * r.0)
    }
}
impl Div for MyF64 {
    type Output = Self;
    fn div(self, r: Self) -> Self {
        Self(self.0 / r.0)
    }
}
impl Neg for MyF64 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}
impl AddAssign for MyF64 {
    fn add_assign(&mut self, r: Self) {
        self.0 += r.0;
    }
}
impl SubAssign for MyF64 {
    fn sub_assign(&mut self, r: Self) {
        self.0 -= r.0;
    }
}
impl MulAssign for MyF64 {
    fn mul_assign(&mut self, r: Self) {
        self.0 *= r.0;
    }
}
impl DivAssign for MyF64 {
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
impl yadc::num_traits::One for MyF64 {
    fn one() -> Self {
        Self(1.0)
    }
}

// --- Unit system using MyF64 -------------------------------------------------

#[derive(Copy, Clone, Debug, yadc::Kind)]
pub enum Kind {
    Linear,
    Affine,
    Angle,
    SolidAngle,
    Invalid,
}

#[derive(Copy, Clone, Debug, Dimension)]
pub struct Dim {
    pub m: i8,
    pub k: i8,
    #[kind]
    pub kind: Kind,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, StaticQuantity)]
pub struct Qty<const D: Dim>(MyF64);

pub type Length = Qty<{ Dim { m: 1, ..Dim::ZERO } }>;
pub type Area = Qty<{ Dim { m: 2, ..Dim::ZERO } }>;
pub type Temperature = Qty<
    {
        Dim {
            k: 1,
            kind: Kind::Affine,
            ..Dim::ZERO
        }
    },
>;
pub type TempDelta = Qty<{ Dim { k: 1, ..Dim::ZERO } }>;

#[derive(Copy, Clone, Unit)]
#[dim(Length)]
pub enum LengthUnit {
    #[linear("m", "meters", MyF64(1.0))]
    Meter,
    #[linear("km", "kilometers", MyF64(1000.0))]
    Kilometer,
}

#[derive(Copy, Clone, Unit)]
#[dim(Area)]
pub enum AreaUnit {
    #[linear("m²", "square meters", MyF64(1.0))]
    SquareMeter,
    #[linear("km²", "square kilometers", MyF64(1_000_000.0))]
    SquareKilometer,
}

#[derive(Copy, Clone, Unit)]
#[dim(Temperature)]
pub enum TempUnit {
    #[linear("K", "kelvin", MyF64(1.0))]
    Kelvin,
    #[affine("°C", "celsius", MyF64(1.0), MyF64(273.15))]
    Celsius,
}

fn main() {
    let side = LengthUnit::Kilometer.quantity(MyF64(2.5));
    let area: Area = side * side;
    println!(
        "side = {}",
        LengthUnit::Kilometer.format_quantity(side, Format::Long)
    );
    println!(
        "area = {:.0}",
        AreaUnit::SquareMeter.format_quantity(area, Format::Short)
    );

    let body = TempUnit::Celsius.quantity(MyF64(36.6));
    println!(
        "body = {} (= {})",
        TempUnit::Celsius.format_quantity(body, Format::Short),
        TempUnit::Kelvin.format_quantity(body, Format::Short)
    );
}
