//! Unit-system-agnostic generic functions via `StaticQuantity` and `Unit`.
//!
//! The functions here call `from_unit`, `as_unit`, and `fmt_as_unit` —
//! methods defined on the `StaticQuantity` *trait*.  Because those methods live
//! on a trait (not a concrete type), a single function body works unchanged
//! regardless of which unit system its arguments come from.
//!
//! This is the capability no other unit library provides: every other library
//! exposes only concrete types, so a function using their conversion or
//! formatting operations is always locked to one unit system.
//!
//! `main()` calls each function with two independent unit systems to
//! demonstrate this:
//! - **`yadc_si`** — the bundled SI implementation
//! - **`nav`**     — a minimal custom nautical system defined in this file

#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(unsized_const_params)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(const_cmp)]
#![feature(structural_match)]
#![allow(incomplete_features)]

use std::ops::Add;
use yadc::{CanUseWithUnit, Dimension, Format, StaticQuantity, Unit, UnitDisplay};

// ─── Generic functions ────────────────────────────────────────────────────────
// `CanUseWithUnit<U>` is auto-implemented by #[derive(StaticQuantity)].
// It implies StaticQuantity, Copy, Sub<U::Offset, Output=U::Multiplier>, and
// the where clause U::Multiplier: Add<U::Offset, Output=Self>. The Add bound
// must still be stated explicitly at call sites because it constrains
// U::Multiplier (not Q) and Rust doesn't propagate trait where clauses.

/// Convert a value from one unit to another within the same quantity type.
fn convert<Q, U>(value: Q::V, from: U, to: U) -> Q::V
where
    U: Unit<D = Q::D, V = Q::V>,
    U::Multiplier: Add<U::Offset, Output = Q>,
    Q: CanUseWithUnit<U>,
{
    Q::from_unit(value, from).as_unit(to)
}

/// Average two values in `in_unit` and format the result in `display_unit`.
/// The unit system is entirely determined by Q and U — no SI knowledge here.
fn average_fmt<Q, U>(a: f64, b: f64, in_unit: U, display_unit: U, fmt: Format) -> UnitDisplay<f64>
where
    U: Unit<D = Q::D, V = f64>,
    U::Multiplier: Add<U::Offset, Output = Q>,
    Q: StaticQuantity<V = f64> + CanUseWithUnit<U> + Add<Q, Output = Q>,
{
    let qa = Q::from_unit(a, in_unit);
    let qb = Q::from_unit(b, in_unit);
    ((qa + qb) / 2.0).fmt_as_unit(display_unit, fmt)
}

// ─── Unit system A: yadc-si ──────────────────────────────────────────────────
use yadc_si::mechanical::*;

// ─── Unit system B: custom nautical system ───────────────────────────────────
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
    #[unit(1.0, "m", "meters")]
    Meter,
    #[unit(1852.0, "nmi", "nautical miles")]
    NauticalMile,
}

// ─── main ─────────────────────────────────────────────────────────────────────

fn main() {
    // ── convert ───────────────────────────────────────────────────────────────

    // SI: kilograms → pounds
    let lb = convert::<Mass, _>(70.0, MassUnit::Kilogram, MassUnit::Pound);
    println!("SI   70 kg          → {lb:.2} lb");

    // Nav: nautical miles → meters  (same generic function, different system)
    let m = convert::<NavDist, _>(3.0, NavDistUnit::NauticalMile, NavDistUnit::Meter);
    println!("Nav  3 nmi          → {m:.0} m");

    // ── average_fmt ───────────────────────────────────────────────────────────

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
