//! Define a unit system from scratch: a kind, a dimension, a quantity family,
//! a few constants and units. No `yadc-si` - every type below is ours.
//!
//! The base constants (`METRE`, `SECOND`, `KELVIN`, `ABSOLUTE_ZERO`) are the
//! only raw numbers. Every other constant is defined from them, and every
//! constant is a unit, called like a function: `KILOMETRE(3.0)` is three
//! kilometres, `MILE(d)` is how many miles `d` is. A scale - a zero and a
//! step - is a unit too: °C is `Scale(ICE_POINT, KELVIN)`.
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
#![feature(fn_traits, unboxed_closures, const_destruct)]
#![allow(incomplete_features)]

use core::marker::ConstParamTy;
use core::ops::{Add, Sub};
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
/// the field, so the base constants below are the only raw numbers.
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

/// A scale: a zero and a step. A number `n` is `zero + n · step`, and a
/// value reads `(value - zero) / step`.
#[derive(Copy, Clone, Debug)]
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

/// Degrees Celsius: kelvins counted from the ice point.
pub const CELSIUS: Scale<Temperature, TemperatureStep> = Scale(ICE_POINT, KELVIN);

/// A choice of units, for a menu or a parser: each variant names the unit
/// it is. The derive writes `Unit`, `Display` and `FromStr`.
#[derive(Copy, Clone, Debug, yadc::Unit)]
#[unit(of = Length)]
pub enum LengthUnit {
    #[unit("m", "metres", by = METRE)]
    Metre,
    #[unit("km", "kilometres", by = KILOMETRE)]
    Kilometre,
    #[unit("mi", "miles", by = MILE)]
    Mile,
}

/// Computed by the compiler: the operators and units are `const`.
pub const BOILING: Temperature = CELSIUS.quantity(100.0);

fn main() {
    // Build with a unit, compute on quantities, read with a unit.
    let distance = KILOMETRE(42.195);
    let time = HOUR(3.5);
    let speed: Velocity = distance / time;
    println!(
        "{:.2} mi in {} h: {:.2} km/h",
        MILE(distance),
        HOUR(time),
        KILOMETRE_PER_HOUR(speed)
    );
    // let nope = distance + time;           // error: cannot add: the dimensions differ

    // A temperature is a point: two differ by a step, a step moves one.
    let morning = CELSIUS.quantity(12.0);
    let noon = morning + 9.5 * KELVIN;
    let rise: TemperatureStep = noon - morning;
    println!(
        "noon is {:.1} °C, {:.1} K warmer",
        CELSIUS.value(noon),
        KELVIN(rise)
    );
    println!("water boils at {:.2} K", KELVIN(BOILING - ABSOLUTE_ZERO));
    // let nope = 2.0 * noon;                // error: a point cannot be scaled

    // A unit picked at run time, by its symbol.
    let unit: LengthUnit = "mi".parse().unwrap();
    println!("{:.3} {unit} ({unit:#})", unit.value(distance));
}
