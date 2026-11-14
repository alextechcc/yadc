//! A quantity's number may be a type of yours, not just `f64`.
//!
//! `MyF64` wraps `f64` and implements the std operators as `const impl`s, so
//! quantities of it compute in `const` items, where a system's constants
//! are defined; and `Default`, which seeds `Sum`. The system is the one of
//! `define_unit_system`, cut to lengths: the numbers stay `MyF64`
//! throughout.
//!
//! Run with: cargo run -p yadc --example custom_value_type
#![feature(
    generic_const_exprs,
    adt_const_params,
    const_trait_impl,
    const_ops,
    const_cmp
)]
#![feature(derive_const, const_destruct, const_convert, const_default)]
#![allow(incomplete_features)]

use const_derive_more::ConstOps;
use core::fmt;
use core::marker::ConstParamTy;
use core::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

// --- The number type ----------------------------------------------------------

#[derive_const(Default)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct MyF64(f64);

impl fmt::Display for MyF64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Forward the formatter, so `{:.1}` reaches the inner `f64`.
        fmt::Display::fmt(&self.0, f)
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

// --- The system ---------------------------------------------------------------

/// The exponent of metres.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy)]
pub struct Dim {
    pub m: i8,
}

const impl Mul for Dim {
    type Output = Option<Dim>;
    fn mul(self, o: Dim) -> Option<Dim> {
        match self.m.checked_add(o.m) {
            Some(m) => Some(Dim { m }),
            None => None,
        }
    }
}
const impl Div for Dim {
    type Output = Option<Dim>;
    fn div(self, o: Dim) -> Option<Dim> {
        match self.m.checked_sub(o.m) {
            Some(m) => Some(Dim { m }),
            None => None,
        }
    }
}
const impl Add for Dim {
    type Output = Option<Dim>;
    fn add(self, o: Dim) -> Option<Dim> {
        if self == o { Some(self) } else { None }
    }
}
const impl Sub for Dim {
    type Output = Option<Dim>;
    fn sub(self, o: Dim) -> Option<Dim> {
        if self == o { Some(self) } else { None }
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, ConstOps)]
#[const_ops(one = Dim { m: 0 })]
pub struct Qty<const D: Dim, N>(N);

/// The number and the metres: `6250000 m^2`.
impl<const D: Dim, N: fmt::Display> fmt::Display for Qty<D, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)?;
        match D.m {
            0 => Ok(()),
            1 => f.write_str(" m"),
            m => write!(f, " m^{m}"),
        }
    }
}

pub type Ratio<N> = Qty<{ Dim { m: 0 } }, N>;
pub type Length<N> = Qty<{ Dim { m: 1 } }, N>;
pub type Area<N> = Qty<{ Dim { m: 2 } }, N>;

/// A plain ratio is its number. The derive writes this for the primitive
/// number types; a number type of yours writes its own.
impl From<Ratio<MyF64>> for MyF64 {
    fn from(r: Ratio<MyF64>) -> MyF64 {
        r.0
    }
}

pub const METRE: Length<MyF64> = Qty(MyF64(1.0));
pub const KILOMETRE: Length<MyF64> = METRE * MyF64(1000.0);
pub const SQUARE_KILOMETRE: Area<MyF64> = KILOMETRE * KILOMETRE;

/// In `const`, in `MyF64`.
const PLOT: Area<MyF64> = KILOMETRE * MyF64(2.5) * (KILOMETRE * MyF64(2.5));

fn main() {
    let side: Length<MyF64> = KILOMETRE * MyF64(2.5);
    let area: Area<MyF64> = side * side;
    println!("area = {area}");
    let in_km2 = MyF64::from(area / SQUARE_KILOMETRE);
    println!("area = {in_km2} km²");
    println!("same as the const: {}", area == PLOT);

    let walk: Length<MyF64> = [side, side, -side].into_iter().sum();
    println!("walked {:.1}", walk);
}
