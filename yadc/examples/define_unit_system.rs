//! Define a unit system from scratch: a kind, a dimension, a quantity family,
//! a few constants and units. No `yadc-si` - every type below is ours.
//!
//! The base constants (`METRE`, `SECOND`, `KELVIN`, `ABSOLUTE_ZERO`) write
//! the field; every other constant is defined from them. A quantity value is
//! a unit of its own kind: `KILOMETRE * 3.0` (or `KILOMETRE.quantity(3.0)`)
//! is three kilometres, `MILE.value(d)` is how many miles `d` is.
//!
//! A unit is also a type of ours with an `impl Unit<Q, N>`: degrees Celsius
//! add a zero to kelvins, and the length units of a menu are types an enum
//! can name, each printing its symbol.
//!
//! Run with: cargo run -p yadc --example define_unit_system
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

use core::fmt;
use core::marker::ConstParamTy;
use yadc::{Dimension, Unit};

/// How a quantity behaves. `Linear` is an ordinary amount: it adds, scales
/// and multiplies. `Affine` is a point, measured from a zero somebody chose:
/// two temperatures differ by a step, a step moves one, and nothing else.
#[derive_const(PartialEq, Default)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, Dimension)]
pub enum Kind {
    #[default]
    Linear,
    #[kind(affine)]
    Affine,
}

/// One exponent per base unit, and the kind. `#[derive(Dimension)]` writes
/// the algebra: exponents add under `*` and must match under `+`.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, Dimension)]
pub struct Dim {
    pub m: i8,
    pub s: i8,
    pub k: i8,
    #[kind]
    pub kind: Kind,
}

/// The quantity family: one type per dimension. Only this crate can write
/// the field: the base constants below write it, and every other number
/// becomes a quantity through a unit.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::Quantity)]
pub struct Qty<const D: Dim>(f64);

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
pub type TemperatureStep = Qty<
    {
        Dim {
            k: 1,
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

// The base constants.
pub const METRE: Length = Qty(1.0);
pub const SECOND: Time = Qty(1.0);
pub const KELVIN: TemperatureStep = Qty(1.0);
pub const ABSOLUTE_ZERO: Temperature = Qty(0.0);

// Everything else, from them.
pub const KILOMETRE: Length = 1000.0 * METRE;
pub const MILE: Length = 1609.344 * METRE;
pub const HOUR: Time = 3600.0 * SECOND;
pub const KILOMETRE_PER_HOUR: Velocity = KILOMETRE / HOUR;
pub const ICE_POINT: Temperature = ABSOLUTE_ZERO + 273.15 * KELVIN;

/// Degrees Celsius: kelvins counted from the ice point. A number `n` is
/// `ICE_POINT + n · KELVIN`.
#[derive(Copy, Clone, Debug)]
pub struct Celsius;

impl const Unit<Temperature, f64> for Celsius {
    fn quantity(&self, value: f64) -> Temperature {
        ICE_POINT + KELVIN.quantity(value)
    }
    fn value(&self, quantity: Temperature) -> f64 {
        KELVIN.value(quantity - ICE_POINT)
    }
}

/// Metres, kilometres and miles as types: each counts through its constant,
/// and prints its symbol (`{}`) or name (`{:#}`).
#[derive(Copy, Clone, Debug)]
pub struct Metre;
#[derive(Copy, Clone, Debug)]
pub struct Kilometre;
#[derive(Copy, Clone, Debug)]
pub struct Mile;

impl const Unit<Length, f64> for Metre {
    fn quantity(&self, value: f64) -> Length {
        METRE.quantity(value)
    }
    fn value(&self, quantity: Length) -> f64 {
        METRE.value(quantity)
    }
}
impl const Unit<Length, f64> for Kilometre {
    fn quantity(&self, value: f64) -> Length {
        KILOMETRE.quantity(value)
    }
    fn value(&self, quantity: Length) -> f64 {
        KILOMETRE.value(quantity)
    }
}
impl const Unit<Length, f64> for Mile {
    fn quantity(&self, value: f64) -> Length {
        MILE.quantity(value)
    }
    fn value(&self, quantity: Length) -> f64 {
        MILE.value(quantity)
    }
}

impl fmt::Display for Metre {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(if f.alternate() { "metres" } else { "m" })
    }
}
impl fmt::Display for Kilometre {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(if f.alternate() { "kilometres" } else { "km" })
    }
}
impl fmt::Display for Mile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(if f.alternate() { "miles" } else { "mi" })
    }
}

/// A choice of units, for a menu or a parser: each variant names the unit
/// type it is. The derive writes `Unit`, and `Display` and `FromStr` from
/// each type's `Display`.
#[derive(Copy, Clone, Debug, yadc::Unit)]
#[unit(of = Length, number = f64)]
pub enum LengthUnit {
    #[unit(Metre)]
    Metre,
    #[unit(Kilometre)]
    Kilometre,
    #[unit(Mile)]
    Mile,
}

/// Computed by the compiler: the operators and units are `const`.
pub const BOILING: Temperature = Celsius.quantity(100.0);

fn main() {
    // Build with a unit, compute on quantities, read with a unit.
    let distance = KILOMETRE * 42.195;
    let time = HOUR * 3.5;
    let speed: Velocity = distance / time;
    println!(
        "{:.2} mi in {} h: {:.2} km/h",
        MILE.value(distance),
        HOUR.value(time),
        KILOMETRE_PER_HOUR.value(speed)
    );
    // let nope = distance + time;           // error: cannot add: the dimensions differ

    // A temperature is a point: two differ by a step, a step moves one.
    let morning = Celsius.quantity(12.0);
    let noon = morning + 9.5 * KELVIN;
    let rise: TemperatureStep = noon - morning;
    println!(
        "noon is {:.1} °C, {:.1} K warmer",
        Celsius.value(noon),
        KELVIN.value(rise)
    );
    println!("water boils at {:.2} K", KELVIN.value(BOILING - ABSOLUTE_ZERO));
    // let nope = 2.0 * noon;                // error: a point cannot be scaled

    // A unit picked at run time, by its symbol.
    let unit: LengthUnit = "mi".parse().unwrap();
    println!("{:.3} {unit} ({unit:#})", unit.value(distance));
}
