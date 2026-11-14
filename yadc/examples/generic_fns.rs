//! Unit-system-agnostic generic functions, bounded on yadc's traits.
//!
//! A unit is a unit *of* a quantity through exactly one trait, `Unit<Q, N>`
//! (`quantity` builds, `value` reads), so a generic function says what it
//! needs in those terms: `U: Unit<Q, N>` ("this unit
//! turns `N`s into `Q`s and back"), plus the std operators it uses on
//! the quantities themselves (`Q: Add<Output = Q>`, `T: Sub<Output = I>`).
//! Units never take part in arithmetic, so the bounds never mention one in an
//! operator. When a function wants all of "adds, scales, and measures with
//! itself", that is one bound, `Q: Quantity`. A single function body works
//! unchanged regardless of which unit system its arguments come from.
//!
//! Every unit meets that one bound: a unit enum, a unit constant, a unit type
//! and a quantity constant alike (a quantity is a unit of its own kind), so
//! `convert` below takes `MassUnit::Kilogram`, `POUND`, `KG` or
//! `NAUTICAL_MILE` the same way.
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
// The numbers are the units' own: `N` in `Unit<Q, N>`.

/// Convert a value from one unit to another: build a `Q`, read it back.
fn convert<Q, N, F, T>(value: N, from: F, to: T) -> N
where
    F: Unit<Q, N>,
    T: Unit<Q, N>,
{
    to.value(from.quantity(value))
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
    U: Unit<I, f64>,
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
/// own: `Unit<Q, N>` plus `Display`, implemented for every unit that has both.
trait Shown<Q, N>: Unit<Q, N> + Display {}
impl<Q, N, U: Unit<Q, N> + Display> Shown<Q, N> for U {}

/// One quantity in every unit of a list picked at run time: `&dyn Shown<Q,
/// f64>` is "some unit of `Q` in `f64`s", still checked at compile time.
fn table<Q: Copy>(q: Q, units: &[&dyn Shown<Q, f64>]) -> String {
    units
        .iter()
        .map(|u| format!("{:.1} {u}", u.value(q)))
        .collect::<Vec<_>>()
        .join(" = ")
}

// --- Unit system A: yadc-si --------------------------------------------------
use yadc_si::dynamic::mechanical::*;
use yadc_si::dynamic::thermal::{TemperatureIntervalUnit, ThermodynamicTemperatureUnit};
use yadc_si::mechanical::units::POUND;
use yadc_si::mechanical::*;
use yadc_si::{Prefix, Prefixed, SIQuantity};

// Quantity constants, built by units at compile time: units too.
const KG: SIQuantity<MASS, f64> = units::KILOGRAM.quantity(1.0);
const LB: SIQuantity<MASS, f64> = POUND.quantity(1.0);

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

// The base constants, and constants defined from them.
const METRE: NavDist = NavQty(1.0);
const SECOND: NavTime = NavQty(1.0);
const NAUTICAL_MILE: NavDist = 1852.0 * METRE;
const FATHOM: NavDist = 1.8288 * METRE;
const HOUR: NavTime = 3600.0 * SECOND;

/// Unit types that count through those constants, and print.
#[derive(Copy, Clone)]
struct Metre;
#[derive(Copy, Clone)]
struct NauticalMile;
/// One unit of time is all this system needs.
#[derive(Copy, Clone)]
struct Hour;

impl const Unit<NavDist, f64> for Metre {
    fn quantity(&self, n: f64) -> NavDist {
        METRE.quantity(n)
    }
    fn value(&self, d: NavDist) -> f64 {
        METRE.value(d)
    }
}
impl const Unit<NavDist, f64> for NauticalMile {
    fn quantity(&self, n: f64) -> NavDist {
        NAUTICAL_MILE.quantity(n)
    }
    fn value(&self, d: NavDist) -> f64 {
        NAUTICAL_MILE.value(d)
    }
}
impl const Unit<NavTime, f64> for Hour {
    fn quantity(&self, n: f64) -> NavTime {
        HOUR.quantity(n)
    }
    fn value(&self, t: NavTime) -> f64 {
        HOUR.value(t)
    }
}
impl Display for Metre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(if f.alternate() { "metres" } else { "m" })
    }
}
impl Display for NauticalMile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(if f.alternate() {
            "nautical miles"
        } else {
            "nmi"
        })
    }
}

/// A choice of the two, for a menu or a parser.
#[derive(Copy, Clone, dynamickit::Values)]
#[values(delegate(Unit<NavDist, f64> {
    fn quantity(&self, value: f64) -> NavDist;
    fn value(&self, quantity: NavDist) -> f64;
}))]
enum NavDistUnit {
    #[value(Metre)]
    Metre,
    #[value(NauticalMile)]
    NauticalMile,
}

// --- main ---------------------------------------------------------------------

fn main() {
    // -- convert ---------------------------------------------------------------

    // SI: kilograms -> pounds
    let lb = convert(70.0, MassUnit::Kilogram, MassUnit::Pound);
    println!("SI   70 kg          -> {lb:.2} lb");

    // Nav: nautical miles -> metres  (same generic function, different system)
    let m = convert(3.0, NavDistUnit::NauticalMile, NavDistUnit::Metre);
    println!("Nav  3 nmi          -> {m:.0} m");

    // -- convert, with constants as units -----------------------------------
    // A quantity constant is a unit of its own kind, so the same function
    // takes constants: here a reading in fathoms (six feet), `FATHOM`.
    let lb = convert(70.0, KG, LB);
    println!("SI   70 kg          -> {lb:.2} lb (constants)");
    let lb = convert(70.0, KG, POUND);
    println!("SI   70 kg          -> {lb:.2} lb (a unit constant)");
    let nmi = convert(1000.0, FATHOM, NAUTICAL_MILE);
    println!("Nav  1000 fathoms   -> {nmi:.3} nmi (constants)");

    // -- table -------------------------------------------------------------------
    // Maths is on quantities: 90 km in an hour is a velocity, shown in every
    // unit of a list picked at run time. A prefix scales an SI unit: `km`
    // is the metre with the prefix kilo.

    let km = Prefixed {
        prefix: Some(Prefix::Kilo),
        unit: units::METRE,
    };
    let v = km.quantity(90.0) / TimeUnit::Hour.quantity(1.0);
    println!(
        "SI   90 km/h        -> {}",
        table(v, &[&VelocityUnit::MetrePerSecond, &VelocityUnit::Knot])
    );
    let leg = NavDistUnit::NauticalMile.quantity(12.0);
    println!(
        "Nav  12 nmi         -> {}",
        table(leg, &[&NavDistUnit::Metre, &NavDistUnit::NauticalMile])
    );
    // The nav system has no unit of speed, and needs none to compute one.
    let speed = leg / Hour.quantity(1.0);
    // A quantity is a unit of its own kind: one metre per second reads it.
    let one_mps = METRE / SECOND;
    println!("Nav  12 kn          -> {:.3} m/s", one_mps.value(speed));

    // -- average ---------------------------------------------------------------

    // SI: average two masses given in kg, read in grams
    let (kg, gram) = (MassUnit::Kilogram, MassUnit::Gram);
    println!(
        "SI   avg(60 kg, 80 kg) in g  = {} {gram}",
        gram.value(average(kg.quantity(60.0), kg.quantity(80.0))),
    );

    // Nav: average two distances given in nmi, read in metres (long name)
    let (nmi, metre) = (NavDistUnit::NauticalMile, NavDistUnit::Metre);
    println!(
        "Nav  avg(10 nmi, 50 nmi) in m = {} {metre:#}",
        metre.value(average(nmi.quantity(10.0), nmi.quantity(50.0))),
    );

    // -- mean and how_many: generic over `Quantity` -----------------------------

    let loads = [KG * 60.0, KG * 80.0, LB * 100.0];
    println!(
        "SI   mean(60 kg, 80 kg, 100 lb) = {:.2} kg",
        KG.value(mean(&loads).unwrap())
    );
    let legs = [
        NavDistUnit::NauticalMile.quantity(3.0),
        NavDistUnit::Metre.quantity(500.0),
    ];
    println!(
        "Nav  mean(3 nmi, 500 m)         = {:.3} nmi",
        NavDistUnit::NauticalMile.value(mean(&legs).unwrap())
    );
    println!(
        "SI   a marathon in laps of 400 m = {:.1}",
        how_many(km.quantity(42.195), LengthUnit::Metre.quantity(400.0))
    );
    println!(
        "Nav  12 nmi in cables (0.1 nmi)  = {:.0}",
        how_many(leg, NAUTICAL_MILE / 10.0)
    );

    // -- change ----------------------------------------------------------------

    let celsius = ThermodynamicTemperatureUnit::Celsius;
    let morning = celsius.quantity(37.0);
    let evening = celsius.quantity(38.5);
    let fahrenheit = TemperatureIntervalUnit::Fahrenheit;
    println!(
        "SI   37 °C -> 38.5 °C = {:.1} {fahrenheit}",
        change(morning, evening, fahrenheit)
    );
    println!(
        "Nav  2 nmi -> 5 nmi   = {} m",
        change(NAUTICAL_MILE * 2.0, NAUTICAL_MILE * 5.0, NavDistUnit::Metre)
    );
}
