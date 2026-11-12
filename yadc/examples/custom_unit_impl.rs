//! Writing a unit by hand: `impl Unit<Velocity>`, plus `Display` if it should
//! print.
//!
//! What makes a type a unit *of* something is one impl, `Unit<Q>`, with two
//! methods: `quantity` turns a number into a `Q`, `value` reads a `Q` back as
//! a number. Most units are constants: a quantity (`METRE_PER_SECOND`), or a scale,
//! a zero and a step (°C is `Scale(ICE_POINT, KELVIN)`, dBm the same on
//! the logarithmic line). Anything else is two functions on a type of yours.
//!
//! The Beaufort wind scale is such a unit: force `B` is a wind speed of
//! `0.836 · B^1.5` metres per second - not a multiple, not a scale. A
//! hand-written impl is a first-class peer of the constants and of a derived
//! unit enum: it mixes with the `yadc-si` units, including behind `&dyn`.
//!
//! Run with: cargo run -p yadc --example custom_unit_impl
#![feature(generic_const_exprs, const_trait_impl, const_convert)]
#![allow(incomplete_features)]

use std::fmt;

use yadc::Unit;
use yadc_si::dynamic::DynamicSIQuantity;
use yadc_si::mechanical::consts::METRE_PER_SECOND;
use yadc_si::mechanical::Velocity;
use yadc_si::dynamic::mechanical::{MassUnit, VelocityUnit};

/// The unit the Beaufort formula counts in: the exact constant, rounded to
/// `f64` by the compiler.
const MPS: Velocity<f64> = METRE_PER_SECOND.into();

/// The Beaufort scale: a zero-sized type of ours.
#[derive(Copy, Clone)]
struct Beaufort;

// The whole conversion, both directions written in quantity terms: the
// formula counts metres per second, and `MPS` - a quantity, so a unit -
// turns those numbers into a speed and back.
impl Unit<Velocity<f64>> for Beaufort {
    fn quantity(&self, force: f64) -> Velocity<f64> {
        MPS * (0.836 * force.powf(1.5))
    }
    fn value(&self, speed: Velocity<f64>) -> f64 {
        (MPS.value(speed) / 0.836).powf(2.0 / 3.0)
    }
}

// How it prints: `{}` is the symbol, `{:#}` the name. Optional - a unit need
// not print. `pad` lets width and alignment apply, as for a `str`.
impl fmt::Display for Beaufort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(if f.alternate() { "Beaufort" } else { "Bft" })
    }
}

/// Some unit of `Q` that also prints. `Unit<Q>` alone need not print, so a
/// list of printable units is a trait of your own, with a blanket impl.
trait ShownUnit<Q>: Unit<Q> + fmt::Display {}
impl<Q, U: Unit<Q> + fmt::Display> ShownUnit<Q> for U {}

fn main() {
    // Force 8, a gale: a real `Velocity`, readable in any unit of velocity.
    let gale = Beaufort.quantity(8.0);
    let knot = VelocityUnit::Knot;
    println!("force 8 = {:.1} {knot}", knot.value(gale));

    // ... and read back in its own unit.
    println!("        = {:.1} {Beaufort:#}", Beaufort.value(gale));

    // Built by a derived unit, read by ours.
    let breeze = knot.quantity(10.0);
    println!("10 kn   = force {:.1}", Beaufort.value(breeze));

    // Maths is on quantities, whatever unit built them.
    let gusts = gale * 1.5;
    println!("gusts   = force {:.1}", Beaufort.value(gusts));

    // Picked at run time: some unit of velocity, still checked at compile
    // time (a mass unit does not fit here). With `Display` too, it prints.
    // Behind `&dyn` a unit is its two methods.
    let choices: [&dyn ShownUnit<Velocity<f64>>; 3] =
        [&VelocityUnit::MetrePerSecond, &knot, &Beaufort];
    for unit in choices {
        println!("{unit:>#18}: {:.1} {unit}", unit.value(gale));
    }

    // A quantity decided at run time: the open type is read through the
    // static `Velocity`, checked. A mass is not a speed, so it is handed back.
    let reading = DynamicSIQuantity::from(MassUnit::Kilogram.quantity(1.0));
    let force = Velocity::try_from(reading).map(|v| Beaufort.value(v));
    println!("1 kg in Bft: {:?}", force.map_err(|_| "not a velocity"));
}
