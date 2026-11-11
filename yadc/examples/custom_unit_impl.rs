//! Writing a unit by hand: `impl Unit<Velocity>`, plus `Display` if it should
//! print.
//!
//! What makes a type a unit *of* something is one impl, `Unit<Q>`, with two
//! methods: `quantity` turns a number into a `Q`, `value` reads a `Q` back as
//! a number. Most units are constants: a quantity (`KILOMETRE`), or a scale
//! - a zero and a step (°C is `Scale(ICE_POINT, KELVIN)`, dBm the same on
//! the logarithmic line). Anything else is two functions on a type of yours.
//!
//! The Beaufort wind scale is such a unit: force `B` is a wind speed of
//! `0.836 · B^1.5` metres per second - not a multiple, not a scale. A
//! hand-written impl is a first-class peer of the constants and of a derived
//! unit enum: it mixes with the `yadc-si` units, including behind `&dyn`.
//!
//! The constants and the derived units are also called like functions
//! (`kmh(20.0)`, `kmh(gale)`). That is the three std function traits, once
//! per direction, each a line over `Unit` - written out here for `Beaufort`,
//! so `Beaufort(8.0)` works too. It needs two nightly features in the crate
//! that writes the impls, and none in a crate that only calls.
//!
//! Run with: cargo run -p yadc --example custom_unit_impl
#![feature(generic_const_exprs)]
#![feature(fn_traits, unboxed_closures)]
#![allow(incomplete_features)]

use std::fmt;

use yadc::Unit;
use yadc_si::DynamicSIQuantity;
use yadc_si::mechanical::consts::METRE_PER_SECOND;
use yadc_si::mechanical::{MassUnit, Velocity, VelocityUnit};

/// The Beaufort scale: a zero-sized type of ours.
#[derive(Copy, Clone)]
struct Beaufort;

// The whole conversion, both directions written in quantity terms: the
// formula counts metres per second, and `METRE_PER_SECOND` - a constant, so
// a unit - turns those numbers into a speed and back.
impl Unit<Velocity> for Beaufort {
    fn quantity(&self, force: f64) -> Velocity {
        METRE_PER_SECOND(0.836 * force.powf(1.5))
    }
    fn value(&self, speed: Velocity) -> f64 {
        (METRE_PER_SECOND(speed) / 0.836).powf(2.0 / 3.0)
    }
}

// How it prints: `{}` is the symbol, `{:#}` the name. Optional - a unit need
// not print. `pad` lets width and alignment apply, as for a `str`.
impl fmt::Display for Beaufort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(if f.alternate() { "Beaufort" } else { "Bft" })
    }
}

// Called like a function: `FnOnce`, `FnMut` and `Fn` for each direction -
// a number in, a speed out; a speed in, a number out. The argument's type
// picks the direction.
macro_rules! called {
    ($unit:ty: $arg:ty => $out:ty, $method:ident) => {
        impl FnOnce<($arg,)> for $unit {
            type Output = $out;
            extern "rust-call" fn call_once(self, (x,): ($arg,)) -> $out {
                Unit::<Velocity>::$method(&self, x)
            }
        }
        impl FnMut<($arg,)> for $unit {
            extern "rust-call" fn call_mut(&mut self, (x,): ($arg,)) -> $out {
                Unit::<Velocity>::$method(self, x)
            }
        }
        impl Fn<($arg,)> for $unit {
            extern "rust-call" fn call(&self, (x,): ($arg,)) -> $out {
                Unit::<Velocity>::$method(self, x)
            }
        }
    };
}
called!(Beaufort: f64 => Velocity, quantity);
called!(Beaufort: Velocity => f64, value);

/// Some unit of `Q` that also prints. `Unit<Q>` alone need not print, so a
/// list of printable units is a trait of your own, with a blanket impl.
trait ShownUnit<Q>: Unit<Q> + fmt::Display {}
impl<Q, U: Unit<Q> + fmt::Display> ShownUnit<Q> for U {}

fn main() {
    // Force 8, a gale: a real `Velocity`, readable in any unit of velocity.
    let gale = Beaufort(8.0);
    let kmh = VelocityUnit::KilometrePerHour;
    println!("force 8 = {:.1} {kmh}", kmh(gale));

    // ... and read back in its own unit.
    println!("        = {:.1} {Beaufort:#}", Beaufort(gale));

    // Built by a derived unit, read by ours.
    let breeze = kmh(20.0);
    println!("20 km/h = force {:.1}", Beaufort(breeze));

    // Maths is on quantities, whatever unit built them.
    let gusts = gale * 1.5;
    println!("gusts   = force {:.1}", Beaufort(gusts));

    // Picked at run time: some unit of velocity, still checked at compile
    // time (a mass unit does not fit here). With `Display` too, it prints.
    // Behind `&dyn` a unit is its two methods.
    let choices: [&dyn ShownUnit<Velocity>; 3] = [&VelocityUnit::MetrePerSecond, &kmh, &Beaufort];
    for unit in choices {
        println!("{unit:>#18}: {:.1} {unit}", unit.value(gale));
    }

    // A quantity decided at run time: the open type is read through the
    // static `Velocity`, checked. A mass is not a speed, so it is handed back.
    let reading = DynamicSIQuantity::from(MassUnit::Kilogram(1.0));
    let force = Velocity::try_from(reading).map(Beaufort);
    println!("1 kg in Bft: {:?}", force.map_err(|_| "not a velocity"));
}
