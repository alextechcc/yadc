//! A measurement: a reading that remembers the unit it is shown in.
//!
//! `Measurement` is a closed set - an enum of yours - whose variants each
//! hold a quantity *and* its unit. The quantity is the variant's member
//! (`#[members(member)]`), so conversions and arithmetic act on it, and the
//! unit rides along:
//!
//! - built from a bare quantity, a reading takes its family's default unit
//!   (`#[members(default)]`) - newtons for a force;
//! - arithmetic goes through the open type (`Via`); a result of the left
//!   operand's quantity keeps the left operand's unit (`#[via(keep)]`), a
//!   new quantity takes its default unit;
//! - printing is one generic function over a quantity and a unit, and a
//!   `match` handing it each variant's pair.
//!
//! Run with: cargo run -p yadc-si --example measurement
#![feature(
    generic_const_exprs,
    adt_const_params,
    unsized_const_params,
    const_trait_impl
)]
#![feature(const_convert, const_default, const_destruct)]
#![allow(incomplete_features)]

use const_derive_more::{Members, Via};
use core::fmt;
use yadc_si::dynamic::DynamicSIQuantity;
use yadc_si::dynamic::mechanical::{AreaUnit, ForceUnit, PressureUnit};
use yadc_si::mechanical::{Area, Force, NEWTON, PASCAL, Pressure};
use yadc_si::{BASE, SIQuantity, SIUnit};

/// A quantity and the unit it is shown in.
#[derive(Clone, Copy, Debug, Members, Via)]
#[members(convert(DynamicSIQuantity))]
#[via(DynamicSIQuantity, Add, Sub, Mul, Div, Mul<f64>, Div<f64>)]
enum Measurement {
    Pressure(
        #[members(member)] Pressure<f64>,
        #[members(default)]
        #[via(keep)]
        PressureUnit,
    ),
    Area(
        #[members(member)] Area<f64>,
        #[members(default)]
        #[via(keep)]
        AreaUnit,
    ),
    Force(
        #[members(member)] Force<f64>,
        #[members(default)]
        #[via(keep)]
        ForceUnit,
    ),
}

/// `n` of a unit chosen at run time, as a quantity in base units `U`.
fn quantity<const U: SIUnit>(unit: impl Into<SIUnit>, n: f64) -> SIQuantity<U, f64, BASE> {
    let value = unit.into().to_base(n);
    SIQuantity::try_from(DynamicSIQuantity {
        unit: U,
        is_base: BASE,
        value,
    })
    .unwrap()
}

/// Print a reading: the number its unit reads, with the caller's flags,
/// then the unit. Once, for every pair.
fn show<const U: SIUnit>(
    f: &mut fmt::Formatter<'_>,
    q: SIQuantity<U, f64, BASE>,
    unit: impl Into<SIUnit> + fmt::Display + Copy,
) -> fmt::Result {
    let base_units = DynamicSIQuantity::from(q).value;
    fmt::Display::fmt(&unit.into().from_base(base_units), f)?;
    write!(f, " {unit}")
}

impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Measurement::Pressure(q, u) => show(f, q, u),
            Measurement::Area(q, u) => show(f, q, u),
            Measurement::Force(q, u) => show(f, q, u),
        }
    }
}

fn main() {
    // A unit builds a quantity from a number.
    let (psi, sq_ft) = (PressureUnit::Psi, AreaUnit::SquareFoot);
    let p = Measurement::Pressure(quantity(psi, 14.7), psi);
    let a = Measurement::Area(quantity(sq_ft, 5.0), sq_ft);
    println!("p       = {p}");
    println!("a       = {a:.3}");

    // The same quantity keeps the left operand's unit ...
    println!("p + p   = {}", (p + p).unwrap());
    println!("p × 2   = {}", p * 2.0);
    // ... a new quantity takes its default unit ...
    println!("p × a   = {:.1}", (p * a).unwrap());
    // ... and a result the set does not list is `None`.
    println!("p × p   = {:?}", (p * p).map(|m| m.to_string()));

    // From a bare quantity: the default unit.
    let f = Measurement::from(2000.0 * NEWTON);
    println!("f       = {f}");
    // Back to the bare quantity, checked.
    println!(
        "as Pa   = {:?}",
        Pressure::<f64>::try_from(p).map(|q| f64::from(q / PASCAL))
    );
    println!("as N    = {:?}", Force::<f64>::try_from(p).is_ok());
}
