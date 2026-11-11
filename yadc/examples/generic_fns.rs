//! Unit-system-agnostic generic functions, bounded on yadc's traits.
//!
//! A unit is a unit *of* a quantity through exactly one trait, `Unit<Q>`
//! (`quantity` builds, `value` reads), so a generic function says what it
//! needs in those terms: `U: Unit<Q>` ("this unit
//! turns numbers into `Q`s and back"), plus the std operators it uses on
//! the quantities themselves (`Q: Add<Output = Q>`, `T: Sub<Output = I>`).
//! Units never take part in arithmetic, so the bounds never mention one in an
//! operator. When a function wants all of "adds, scales, and measures with
//! itself", that is one bound, `Q: Quantity`. A single function body works
//! unchanged regardless of which unit system its arguments come from.
//!
//! A quantity constant or a unit enum's value is also called like a
//! function - on a number it builds, on a
//! quantity it reads - so a generic function may ask for nothing but std:
//! `impl Fn(f64) -> Q` and `impl Fn(Q) -> f64` (`convert_with` below), which
//! any closure satisfies too. `Unit<Q>` is the bound to use when both
//! directions come from one value, or behind `&dyn`.
//!
//! This is the capability no other unit library provides: every other library
//! exposes only concrete types, so a function using their conversion or
//! formatting operations is always locked to one unit system.
//!
//! `main()` calls each function with two independent unit systems to
//! demonstrate this:
//! - **`yadc_si`** - the bundled SI implementation
//! - **`nav`**     - a minimal custom nautical system defined in this file
//!
//! Because this file *defines* a dimension and a quantity type, it needs the
//! definer feature set (`min_adt_const_params`, `const_trait_impl`,
//! `const_cmp`, `const_default`, `derive_const`, ...); a crate that only
//! uses `yadc-si` needs just `generic_const_exprs`.
//!
//! Run with: cargo run -p yadc --example generic_fns
#![feature(generic_const_exprs, min_adt_const_params, const_trait_impl)]
#![feature(fn_traits, unboxed_closures, const_destruct)]
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
use std::fmt::Display;
use std::ops::{Add, Div, Sub};
use yadc::{Dimension, Quantity, Unit};

// --- Generic functions --------------------------------------------------------
// `Q` is whatever quantity the units measure - inferred from the unit, never
// named by the caller, and bounded by nothing but what the body does with it.
// The numbers are the units' own (`f64` by default; `Unit<Q, N>` for others).

/// Convert a value from one unit to another: build a `Q`, read it back.
fn convert<Q, N, F, T>(value: N, from: F, to: T) -> N
where
    F: Unit<Q, N>,
    T: Unit<Q, N>,
{
    to.value(from.quantity(value))
}

/// The same conversion, asking only for std's function traits: something
/// that turns a number into a `Q`, and something that reads a `Q` as a
/// number. Every unit is both; so is a closure.
fn convert_with<Q>(value: f64, from: impl Fn(f64) -> Q, to: impl Fn(Q) -> f64) -> f64 {
    to(from(value))
}

/// The average of two values: a value of the same quantity. No unit and no
/// SI knowledge here - the caller reads the result in whatever unit it likes.
fn average<Q>(a: Q, b: Q) -> Q
where
    Q: Add<Output = Q> + Div<f64, Output = Q>,
{
    (a + b) / 2.0
}

/// The difference between two readings, in `unit`. The difference of two
/// absolute temperatures is a temperature *step* (`T - T = I`), so the unit
/// is a unit of `I`, not of `T` - the bounds say exactly that.
fn change<T, I, U>(from: T, to: T, unit: U) -> f64
where
    T: Sub<Output = I>,
    U: Unit<I>,
{
    unit.value(to - from)
}

/// The mean of some quantities: a quantity of the same kind, `None` for
/// none. `Quantity` says the values add and divide by their numbers; with
/// `Number = f64`, the numbers are `f64`s.
fn mean<Q: Quantity<Number = f64>>(values: &[Q]) -> Option<Q> {
    let (first, rest) = values.split_first()?;
    let sum = rest.iter().fold(*first, |sum, v| sum + *v);
    Some(sum / values.len() as f64)
}

/// How many times `step` goes into `q`: every quantity is a unit of its own
/// kind, so `step.value(q)` is the answer - no unit type needed.
fn how_many<Q: Quantity<Number = f64>>(q: Q, step: Q) -> f64 {
    step.value(q)
}

/// A unit need not print, so "a unit of `Q` that prints" is a trait of your
/// own: `Unit<Q>` plus `Display`, implemented for every unit that has both.
trait Shown<Q>: Unit<Q> + Display {}
impl<Q, U: Unit<Q> + Display> Shown<Q> for U {}

/// One quantity in every unit of a list picked at run time: `&dyn Shown<Q>`
/// is "some unit of `Q`", still checked at compile time.
fn table<Q: Copy>(q: Q, units: &[&dyn Shown<Q>]) -> String {
    units
        .iter()
        .map(|u| format!("{:.1} {u}", u.value(q)))
        .collect::<Vec<_>>()
        .join(" = ")
}

// --- Unit system A: yadc-si --------------------------------------------------
use yadc_si::mechanical::*;
use yadc_si::thermal::consts::CELSIUS;
use yadc_si::thermal::TemperatureIntervalUnit;
use yadc_si::KILO;

// --- Unit system B: custom nautical system -----------------------------------
// A completely separate Dim / Qty / Unit stack, with no labels (no `#[kind]`
// field: only the exponents decide). The functions above require no changes
// to accept these types.
//
// `PartialEq`, `Eq` and `ConstParamTy` are derived here, not by the yadc
// derives: a dimension is the const generic parameter of the quantity type,
// which the compiler checks structurally.

#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
struct NavDim {
    d: i8,
    t: i8,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::Quantity)]
struct NavQty<const D: NavDim>(f64);

type NavDist = NavQty<
    {
        NavDim {
            d: 1,
            ..NavDim::DIMENSIONLESS
        }
    },
>;
type NavTime = NavQty<
    {
        NavDim {
            t: 1,
            ..NavDim::DIMENSIONLESS
        }
    },
>;

// The base constants, and the units that name constants defined from them.
const METRE: NavDist = NavQty(1.0);
const SECOND: NavTime = NavQty(1.0);
const NAUTICAL_MILE: NavDist = 1852.0 * METRE;
const HOUR: NavTime = 3600.0 * SECOND;

#[derive(Copy, Clone, yadc::Unit)]
#[unit(of = NavDist)]
enum NavDistUnit {
    #[unit("m", "metres", by = METRE)]
    Metre,
    #[unit("nmi", "nautical miles", by = NAUTICAL_MILE)]
    NauticalMile,
}

/// One unit of time is all this system needs: a unit type.
#[derive(Copy, Clone, yadc::Unit)]
#[unit("h", "hours", of = NavTime, by = HOUR)]
struct Hour;

// --- main ---------------------------------------------------------------------

fn main() {
    // -- convert ---------------------------------------------------------------

    // SI: kilograms -> pounds
    let lb = convert(70.0, MassUnit::Kilogram, MassUnit::Pound);
    println!("SI   70 kg          -> {lb:.2} lb");

    // Nav: nautical miles -> metres  (same generic function, different system)
    let m = convert(3.0, NavDistUnit::NauticalMile, NavDistUnit::Metre);
    println!("Nav  3 nmi          -> {m:.0} m");

    // -- convert_with: units as functions --------------------------------------
    // The units are passed as the functions they are - a unit enum's value, a
    // constant - and a closure stands in where there is no unit: here, a
    // reading in fathoms (six feet), built with a formula.
    let lb = convert_with(70.0, MassUnit::Kilogram, MassUnit::Pound);
    println!("SI   70 kg          -> {lb:.2} lb (by call)");
    let nmi = convert_with(1000.0, |fathoms| 1.8288 * fathoms * METRE, NAUTICAL_MILE);
    println!("Nav  1000 fathoms   -> {nmi:.3} nmi (a closure, then a constant)");

    // -- table -------------------------------------------------------------------
    // Maths is on quantities: 90 km in an hour is a velocity, shown in every
    // unit of a list picked at run time. A prefix scales an SI unit:
    // `KILO * METRE` is a kilometre.

    let kilometre: Length<f64> = KILO * yadc_si::mechanical::consts::METRE;
    let v = kilometre(90.0) / TimeUnit::Hour(1.0);
    println!(
        "SI   90 km/h        -> {}",
        table(v, &[&VelocityUnit::MetrePerSecond, &VelocityUnit::Knot])
    );
    let leg = NavDistUnit::NauticalMile(12.0);
    println!(
        "Nav  12 nmi         -> {}",
        table(leg, &[&NavDistUnit::Metre, &NavDistUnit::NauticalMile])
    );
    // The nav system has no unit of speed, and needs none to compute one.
    let speed = leg / Hour(1.0);
    // A quantity is a unit of its own kind: one metre per second reads it.
    let one_mps = METRE / SECOND;
    println!("Nav  12 kn          -> {:.3} m/s", one_mps(speed));

    // -- average ---------------------------------------------------------------

    // SI: average two masses given in kg, read in grams
    let (kg, gram) = (MassUnit::Kilogram, MassUnit::Gram);
    println!(
        "SI   avg(60 kg, 80 kg) in g  = {} {gram}",
        gram(average(kg(60.0), kg(80.0))),
    );

    // Nav: average two distances given in nmi, read in metres (long name)
    let (nmi, metre) = (NavDistUnit::NauticalMile, NavDistUnit::Metre);
    println!(
        "Nav  avg(10 nmi, 50 nmi) in m = {} {metre:#}",
        metre(average(nmi(10.0), nmi(50.0))),
    );

    // -- mean and how_many: generic over `Quantity` -----------------------------

    let loads = [
        MassUnit::Kilogram(60.0),
        MassUnit::Kilogram(80.0),
        MassUnit::Pound(100.0),
    ];
    println!(
        "SI   mean(60 kg, 80 kg, 100 lb) = {:.2} kg",
        MassUnit::Kilogram(mean(&loads).unwrap())
    );
    let legs = [
        NavDistUnit::NauticalMile(3.0),
        NavDistUnit::Metre(500.0),
    ];
    println!(
        "Nav  mean(3 nmi, 500 m)         = {:.3} nmi",
        NavDistUnit::NauticalMile(mean(&legs).unwrap())
    );
    println!(
        "SI   a marathon in laps of 400 m = {:.1}",
        how_many(
            kilometre(42.195),
            LengthUnit::Metre(400.0)
        )
    );
    println!(
        "Nav  12 nmi in cables (0.1 nmi)  = {:.0}",
        how_many(leg, NAUTICAL_MILE / 10.0)
    );

    // -- change ----------------------------------------------------------------

    let morning = CELSIUS.quantity(37.0);
    let evening = CELSIUS.quantity(38.5);
    let fahrenheit = TemperatureIntervalUnit::Fahrenheit;
    println!(
        "SI   37 °C -> 38.5 °C = {:.1} {fahrenheit}",
        change(morning, evening, fahrenheit)
    );
    println!(
        "Nav  2 nmi -> 5 nmi   = {} m",
        change(
            NavDistUnit::NauticalMile(2.0),
            NavDistUnit::NauticalMile(5.0),
            NavDistUnit::Metre
        )
    );
}
