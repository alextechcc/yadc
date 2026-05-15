//! Unit-system-agnostic generic functions via `Quantity` and `Unit`.
//!
//! The functions here convert and format through the `Unit` *trait*
//! (`unit.quantity`, `unit.from_quantity`, `unit.format_quantity`). Because those
//! methods live on a trait (not a concrete type), a single function body works
//! unchanged regardless of which unit system its arguments come from.
//!
//! This is the capability no other unit library provides: every other library
//! exposes only concrete types, so a function using their conversion or
//! formatting operations is always locked to one unit system.
//!
//! `main()` calls each function with two independent unit systems to
//! demonstrate this:
//! - **`yadc_si`** - the bundled SI implementation
//! - **`nav`**     - a minimal custom nautical system defined in this file

#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(unsized_const_params)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(const_cmp)]
#![feature(structural_match)]
#![allow(incomplete_features)]

use std::ops::{Add, Div};
use yadc::{Dimension, Format, Quantity, Unit, UnitDisplay};

// --- Generic functions --------------------------------------------------------
// A generic quantity `Q` plus any unit `U` whose `Quantity` is `Q`. `U::Quantity
// = Q` is the only bound needed to tie a unit to its quantity - conversion and
// formatting are the unit-led `Unit` methods (`quantity` / `from_quantity` /
// `format_quantity`), so one function body works for any unit system.

/// Convert a value from one unit to another within the same quantity type.
fn convert<Q, U>(value: Q::V, from: U, to: U) -> Q::V
where
    Q: Quantity,
    U: Unit<Quantity = Q>,
{
    to.from_quantity(from.quantity(value))
}

/// Average two values in `in_unit` and format the result in `display_unit`.
/// The unit system is entirely determined by Q and U - no SI knowledge here.
fn average_fmt<Q, U>(a: f64, b: f64, in_unit: U, display_unit: U, fmt: Format) -> UnitDisplay<f64>
where
    Q: Quantity<V = f64> + Add<Q, Output = Q> + Div<f64, Output = Q>,
    U: Unit<Quantity = Q>,
{
    let qa = in_unit.quantity(a);
    let qb = in_unit.quantity(b);
    display_unit.format_quantity((qa + qb) / 2.0, fmt)
}

// --- Unit system A: yadc-si --------------------------------------------------
use yadc_si::mechanical::*;

// --- Unit system B: custom nautical system -----------------------------------
// A completely separate Kind / Dim / Qty / Unit stack. The functions above
// require no changes to accept these types.

#[derive(Copy, Clone, Debug, yadc::Kind)]
#[allow(dead_code)]
enum NavKind {
    Linear,
    Affine,
    Angle,
    SolidAngle,
    Invalid,
}

#[derive(Copy, Clone, Debug, yadc::Dimension)]
struct NavDim {
    d: i8,
    #[kind]
    kind: NavKind,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::StaticQuantity)]
struct NavQty<const D: NavDim>(f64);

type NavDist = NavQty<
    {
        NavDim {
            d: 1,
            ..NavDim::ZERO
        }
    },
>;

#[derive(Copy, Clone, yadc::Unit)]
#[dim(NavDist)]
enum NavDistUnit {
    #[linear("m", "meters", 1.0)]
    Meter,
    #[linear("nmi", "nautical miles", 1852.0)]
    NauticalMile,
}

// --- main ---------------------------------------------------------------------

fn main() {
    // -- convert ---------------------------------------------------------------

    // SI: kilograms -> pounds
    let lb = convert::<Mass, _>(70.0, MassUnit::Kilogram, MassUnit::Pound);
    println!("SI   70 kg          -> {lb:.2} lb");

    // Nav: nautical miles -> meters  (same generic function, different system)
    let m = convert::<NavDist, _>(3.0, NavDistUnit::NauticalMile, NavDistUnit::Meter);
    println!("Nav  3 nmi          -> {m:.0} m");

    // -- average_fmt -----------------------------------------------------------

    // SI: average two masses in kg, display result in grams
    println!(
        "SI   avg(60 kg, 80 kg) in g  = {}",
        average_fmt::<Mass, _>(
            60.0,
            80.0,
            MassUnit::Kilogram,
            MassUnit::Gram,
            Format::Short
        ),
    );

    // Nav: average two distances in nmi, display result in meters
    println!(
        "Nav  avg(10 nmi, 50 nmi) in m = {}",
        average_fmt::<NavDist, _>(
            10.0,
            50.0,
            NavDistUnit::NauticalMile,
            NavDistUnit::Meter,
            Format::Long,
        ),
    );
}
