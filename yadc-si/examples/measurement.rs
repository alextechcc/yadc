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
//! - printing is one generic visitor over `(quantity, unit)`.
//!
//! Run with: cargo run -p yadc-si --example measurement
#![feature(generic_const_exprs, const_trait_impl)]
#![feature(const_convert, const_default, const_destruct, const_ops)]
#![allow(incomplete_features)]

use core::fmt;
use dynamickit::{Members, Via, Visit, Visitor};
use yadc::Unit;
use yadc_si::KILO;
use yadc_si::dynamic::DynamicSIQuantity;
use yadc_si::mechanical::{Area, AreaUnit, Force, ForceUnit, Pressure, PressureUnit, consts};

const PASCAL: Pressure<f64> = consts::PASCAL.into();
const KILONEWTON: Force<f64> = (KILO * consts::NEWTON).into();

/// A quantity and the unit it is shown in.
#[derive(Clone, Copy, Debug, Visit, Members, Via)]
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

/// Print a reading: the number its unit reads, with the caller's flags,
/// then the unit (`{:#}`: its name). Once, for every pair.
struct Show<'a, 'b>(&'a mut fmt::Formatter<'b>);

impl<Q, U: Unit<Q> + fmt::Display> Visitor<(Q, U)> for Show<'_, '_> {
    type Output = fmt::Result;
    fn visit(self, (quantity, unit): (Q, U)) -> fmt::Result {
        fmt::Display::fmt(&unit.value(quantity), self.0)?;
        if self.0.alternate() {
            write!(self.0, " {unit:#}")
        } else {
            write!(self.0, " {unit}")
        }
    }
}

impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (*self).visit_with(Show(f))
    }
}

fn main() {
    // A unit builds a quantity from a number.
    let (psi, sq_ft) = (PressureUnit::Psi, AreaUnit::SquareFoot);
    let p = Measurement::Pressure(psi.quantity(14.7), psi);
    let a = Measurement::Area(sq_ft.quantity(5.0), sq_ft);
    println!("p       = {p}  ({p:#})");
    println!("a       = {a:.3}");

    // The same quantity keeps the left operand's unit ...
    println!("p + p   = {}", (p + p).unwrap());
    println!("p × 2   = {}", p * 2.0);
    // ... a new quantity takes its default unit ...
    println!("p × a   = {:.1}", (p * a).unwrap());
    // ... and a result the set does not list is `None`.
    println!("p × p   = {:?}", (p * p).map(|m| m.to_string()));

    // From a bare quantity: the default unit.
    let f = Measurement::from(KILONEWTON.quantity(2.0));
    println!("f       = {f}");
    // Back to the bare quantity, checked.
    println!("as Pa   = {:?}", Pressure::try_from(p).map(|q| PASCAL.value(q)));
    println!("as N    = {:?}", Force::try_from(p).is_ok());
}
