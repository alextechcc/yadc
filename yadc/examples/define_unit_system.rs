//! A unit system written by hand, in yadc's pattern. Everything below is
//! this file's own:
//!
//! - `Dim`, the dimension: std operators whose outputs are `Option`s (`None`
//!   where two dimensions do not combine), and `Dim::ONE`, a plain number's;
//! - `Qty<D, N>`, the quantity, its operators lifted from `N`'s and `Dim`'s
//!   by `#[derive(const_derive_more::ConstOps)]`;
//! - the constants and the aliases;
//! - `From` impls, for degrees Celsius and for a plain number's number;
//! - `Display`.
//!
//! A quantity is counted in the base units (metres, seconds, kelvins); every
//! other unit is a constant, a number reads through it (`f64::from(d /
//! MILE)`).
//!
//! Run with: cargo run -p yadc --example define_unit_system
#![feature(
    generic_const_exprs,
    adt_const_params,
    const_trait_impl,
    const_ops,
    const_cmp
)]
#![feature(derive_const, const_destruct, const_convert)]
#![allow(incomplete_features)]

use const_derive_more::ConstOps;
use core::fmt;
use core::marker::ConstParamTy;
use core::ops::{Add, Div, Mul, Sub};

/// The exponents of metres, seconds and kelvins.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy)]
pub struct Dim {
    pub m: i8,
    pub s: i8,
    pub k: i8,
}

impl Dim {
    /// A plain number's dimension.
    pub const ONE: Dim = Dim { m: 0, s: 0, k: 0 };
}

/// Exponents add; `None` where one does not fit an `i8`.
const impl Mul for Dim {
    type Output = Option<Dim>;
    fn mul(self, o: Dim) -> Option<Dim> {
        match (
            self.m.checked_add(o.m),
            self.s.checked_add(o.s),
            self.k.checked_add(o.k),
        ) {
            (Some(m), Some(s), Some(k)) => Some(Dim { m, s, k }),
            _ => None,
        }
    }
}
const impl Div for Dim {
    type Output = Option<Dim>;
    fn div(self, o: Dim) -> Option<Dim> {
        match (
            self.m.checked_sub(o.m),
            self.s.checked_sub(o.s),
            self.k.checked_sub(o.k),
        ) {
            (Some(m), Some(s), Some(k)) => Some(Dim { m, s, k }),
            _ => None,
        }
    }
}
/// Only one dimension adds to itself.
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

/// `m·s⁻¹`; nothing for a plain number.
impl fmt::Display for Dim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for (symbol, power) in [("m", self.m), ("s", self.s), ("K", self.k)] {
            if power == 0 {
                continue;
            }
            if !first {
                f.write_str("·")?;
            }
            first = false;
            f.write_str(symbol)?;
            if power != 1 {
                for c in power.to_string().chars() {
                    f.write_str(match c {
                        '-' => "⁻",
                        '1' => "¹",
                        '2' => "²",
                        '3' => "³",
                        '4' => "⁴",
                        '5' => "⁵",
                        '6' => "⁶",
                        '7' => "⁷",
                        '8' => "⁸",
                        '9' => "⁹",
                        _ => "⁰",
                    })?;
                }
            }
        }
        Ok(())
    }
}

/// A number of base units of the dimension `D`, counted in `N`.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, ConstOps)]
pub struct Qty<#[ops(one = Dim::ONE)] const D: Dim, N>(N);

/// The number and the dimension: `9.81 m·s⁻²`.
impl<const D: Dim, N: fmt::Display> fmt::Display for Qty<D, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)?;
        if D != Dim::ONE {
            write!(f, " {D}")?;
        }
        Ok(())
    }
}

/// A plain number is its number: `f64::from(d / MILE)`.
impl From<Qty<{ Dim::ONE }, f64>> for f64 {
    fn from(q: Qty<{ Dim::ONE }, f64>) -> f64 {
        q.0
    }
}

pub type Length<N> = Qty<{ Dim { m: 1, s: 0, k: 0 } }, N>;
pub type Time<N> = Qty<{ Dim { m: 0, s: 1, k: 0 } }, N>;
pub type Velocity<N> = Qty<{ Dim { m: 1, s: -1, k: 0 } }, N>;
pub type Temperature<N> = Qty<{ Dim { m: 0, s: 0, k: 1 } }, N>;

// The base units write the field ...
pub const METRE: Length<f64> = Qty(1.0);
pub const SECOND: Time<f64> = Qty(1.0);
pub const KELVIN: Temperature<f64> = Qty(1.0);

// ... and the others are built from them.
pub const KILOMETRE: Length<f64> = 1000.0 * METRE;
pub const MILE: Length<f64> = 1609.344 * METRE;
pub const HOUR: Time<f64> = 3600.0 * SECOND;
pub const KILOMETRE_PER_HOUR: Velocity<f64> = KILOMETRE / HOUR;

/// A number of degrees Celsius: kelvins from the ice point. It is not a
/// multiple of the kelvin, so it converts with std `From`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Celsius(pub f64);

impl From<Celsius> for Temperature<f64> {
    fn from(c: Celsius) -> Self {
        Qty(c.0 + 273.15)
    }
}
impl From<Temperature<f64>> for Celsius {
    fn from(t: Temperature<f64>) -> Self {
        Celsius(t.0 - 273.15)
    }
}

/// Computed by the compiler: every operator above is `const`.
const SPEED_LIMIT: Velocity<f64> = 50.0 * KILOMETRE_PER_HOUR;

fn main() {
    // A number times a unit is a quantity; a quantity over a unit is a plain
    // number.
    let distance = 42.195 * KILOMETRE;
    let time = 3.5 * HOUR;
    let speed: Velocity<f64> = distance / time;
    println!(
        "{:.2} mi in {} h: {:.2} km/h ({speed:.3})",
        f64::from(distance / MILE),
        f64::from(time / HOUR),
        f64::from(speed / KILOMETRE_PER_HOUR),
    );
    println!("over the limit: {}", speed > SPEED_LIMIT);
    // let nope = distance + time;  // error: cannot add: `+` of these parameters has no result

    let noon: Temperature<f64> = Celsius(21.5).into();
    let warmer = noon + 2.0 * KELVIN;
    println!("{noon:.2} is {:?}; 2 K more is {:?}", Celsius::from(noon), Celsius::from(warmer));
}
