//! Writing a unit by hand: `impl Unit<SIQuantity<VELOCITY, f64>, f64>`, plus `Display` if
//! it should print.
//!
//! What makes a type a unit *of* something is one impl, `Unit<Q, N>`, with
//! two methods: `quantity` turns an `N` into a `Q`, `value` reads a `Q` back
//! as an `N`. In `yadc-si` each unit is a constant (`METRE_PER_SECOND`,
//! `KNOT`, `CELSIUS`) whose type's const parameters hold its size and zero,
//! all sharing one impl. Any other unit is two functions on a type of yours.
//!
//! The Beaufort wind scale is such a unit: force `B` is a wind speed of
//! `0.836 · B^1.5` metres per second - not a multiple, not a zero and a step.
//! A hand-written impl is a first-class peer of the `yadc-si` units and of
//! an enum of them: they mix, including behind `&dyn`.
//!
//! Run with: cargo run -p yadc --example custom_unit_impl
#![feature(generic_const_exprs, const_trait_impl)]
#![allow(incomplete_features)]

use std::fmt;

use yadc::Unit;
use yadc_si::SIQuantity;
use yadc_si::dynamic::DynamicSIQuantity;
use yadc_si::dynamic::mechanical::{MassUnit, VelocityUnit};
use yadc_si::mechanical::VELOCITY;
use yadc_si::mechanical::units::METRE_PER_SECOND;

/// The Beaufort scale: a zero-sized type of ours.
#[derive(Copy, Clone)]
struct Beaufort;

// The whole conversion, both directions written in quantity terms: the
// formula counts metres per second, and `METRE_PER_SECOND` - a unit - turns
// those numbers into a speed and back.
impl Unit<SIQuantity<VELOCITY, f64>, f64> for Beaufort {
    fn quantity(&self, force: f64) -> SIQuantity<VELOCITY, f64> {
        METRE_PER_SECOND.quantity(0.836 * force.powf(1.5))
    }
    fn value(&self, speed: SIQuantity<VELOCITY, f64>) -> f64 {
        (METRE_PER_SECOND.value(speed) / 0.836).powf(2.0 / 3.0)
    }
}

// How it prints: `{}` is the symbol, `{:#}` the name. Optional - a unit need
// not print. `pad` lets width and alignment apply, as for a `str`.
impl fmt::Display for Beaufort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(if f.alternate() { "Beaufort" } else { "Bft" })
    }
}

/// Some unit of `Q` in `N` that also prints. `Unit<Q, N>` alone need not
/// print, so a list of printable units is a trait of your own, with a blanket
/// impl.
trait ShownUnit<Q, N>: Unit<Q, N> + fmt::Display {}
impl<Q, N, U: Unit<Q, N> + fmt::Display> ShownUnit<Q, N> for U {}

fn main() {
    // Force 8, a gale: a real `SIQuantity<VELOCITY, f64>`, readable in any unit of velocity.
    let gale = Beaufort.quantity(8.0);
    let knot = VelocityUnit::Knot;
    println!("force 8 = {:.1} {knot}", knot.value(gale));

    // ... and read back in its own unit.
    println!("        = {:.1} {Beaufort:#}", Beaufort.value(gale));

    // Built by a unit of an enum, read by ours.
    let breeze = knot.quantity(10.0);
    println!("10 kn   = force {:.1}", Beaufort.value(breeze));

    // Maths is on quantities, whatever unit built them.
    let gusts = gale * 1.5;
    println!("gusts   = force {:.1}", Beaufort.value(gusts));

    // Picked at run time: some unit of velocity, still checked at compile
    // time (a mass unit does not fit here). With `Display` too, it prints.
    // Behind `&dyn` a unit is its two methods.
    let choices: [&dyn ShownUnit<SIQuantity<VELOCITY, f64>, f64>; 3] =
        [&VelocityUnit::MetrePerSecond, &knot, &Beaufort];
    for unit in choices {
        println!("{unit:>#18}: {:.1} {unit}", unit.value(gale));
    }

    // A quantity decided at run time: the open type is read through the
    // static `SIQuantity<VELOCITY, _>`, checked. A mass is not a speed, so it is handed back.
    let reading = DynamicSIQuantity::from(MassUnit::Kilogram.quantity(1.0));
    let force = SIQuantity::<VELOCITY, _>::try_from(reading).map(|v| Beaufort.value(v));
    println!("1 kg in Bft: {:?}", force.map_err(|_| "not a velocity"));
}
