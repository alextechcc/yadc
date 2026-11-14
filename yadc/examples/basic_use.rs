//! Tour of the `yadc-si` system, one feature per section.
//!
//! Run with: cargo run -p yadc --example basic_use

// `generic_const_exprs` is all a crate using `yadc-si` needs; `const_trait_impl`
// and `const_ops` only because this one computes quantities in `const` items.
#![feature(generic_const_exprs, const_trait_impl, const_ops)]
#![allow(incomplete_features)]

use yadc_si::dimensionless::*;
use yadc_si::dynamic::DynamicSIQuantity;
use yadc_si::dynamic::mechanical::LengthUnit;
use yadc_si::electromagnetic::*;
use yadc_si::mechanical::*;
use yadc_si::thermal::*;
use yadc_si::*;

/// Kilometres per hour: a quotient of two unit constants, a unit constant
/// itself.
const KILOMETRE_PER_HOUR: SIQuantity<{ KILOMETRE.unit().times(HOUR.unit(), -1) }, Exact, false> =
    KILOMETRE / HOUR;

/// Computed by the compiler: a weight under standard gravity.
const G_N: Acceleration<f64> = 9.80665 * METRE / (SECOND * SECOND);
const WEIGHT: Force<f64> = 70.0 * KILOGRAM * G_N;

fn main() {
    // --- 1. A number times a unit is a quantity -----------------------------
    // Every unit is a constant, one of it. The unit is in the type: `2.0 *
    // KILOMETRE` is two kilometres, and prints so.
    let trip = 2.0_f64 * KILOMETRE;
    println!("trip        = {trip}");

    // --- 2. Conversion is `From`, to and from the base units ---------------
    // A quantity in any unit converts to the base unit of its dimension, the
    // one the annotated type names, and back.
    let distance: Length<f64> = trip.into();
    println!("distance    = {distance}");
    let in_miles: SIQuantity<{ MILE.unit() }, f64, false> = distance.into();
    println!("            = {in_miles:.4}");

    // --- 3. Products of base units are base units --------------------------
    let time: Time<f64> = 100.0 * SECOND;
    let speed: Velocity<f64> = distance / time;
    let mass: Mass<f64> = 2.0 * KILOGRAM;
    let force: Force<f64> = mass * (speed / time);
    println!("speed       = {speed}, force = {force}");
    let kmh: SIQuantity<{ KILOMETRE_PER_HOUR.unit() }, f64, false> = speed.into();
    println!("            = {kmh}");
    // let nope = distance + time;  // error: cannot add: `+` of these parameters has no result

    // --- 4. A quantity over its unit is a number ---------------------------
    // A quotient of one unit is a plain number, and std `From` reads it.
    let n = f64::from(distance / (1.0 * METRE));
    println!("metres      = {n}");
    let p: Power<f64> = (12.0 * VOLT) * (0.5 * AMPERE);
    println!("12 V × 0.5 A = {p}, {} W", f64::from(p / (1.0 * WATT)));

    // --- 5. Units with a zero, and logarithmic units -----------------------
    // °C counts kelvins from the ice point; dBm is a logarithm of the power.
    // Neither has products: they convert to their base unit first.
    let body: ThermodynamicTemperature<f64> = (37.0_f64 * CELSIUS).into();
    let room: ThermodynamicTemperature<f64> = (21.0_f64 * CELSIUS).into();
    println!("body        = {body:.2}, {:.0} above room", body - room);
    let tx: Power<f64> = (30.0_f64 * DECIBEL_MILLIWATT).into();
    println!("30 dBm      = {tx:.3}");
    let ratio: Dimensionless<f64> = (20.0_f64 * DECIBEL).into();
    println!("20 dB       = × {}", f64::from(ratio));

    // --- 6. In `const` ------------------------------------------------------
    println!("70 kg × g_n = {WEIGHT:.1}");

    // --- 7. A unit chosen at run time ---------------------------------------
    // A unit enum names a quantity's units, prints and parses them; its unit
    // converts numbers to and from the base unit.
    let unit: LengthUnit = "mi".parse().unwrap();
    let unit = SIUnit::from(unit);
    println!("{n} m      = {:.4} {unit}", unit.from_base(n));

    // --- 8. A quantity whose unit is chosen at run time ---------------------
    // `DynamicSIQuantity` holds its unit as a value; every operator may have
    // no result.
    let d = DynamicSIQuantity::from(3.0 * METRE);
    let t = DynamicSIQuantity::from(2.0 * SECOND);
    println!("d / t       = {:?}", (d / t).map(|v| v.value));
    println!("d + t       = {:?}", d + t);
    let back: Length<f64> = (d + d).unwrap().try_into().unwrap();
    println!("d + d       = {back}");
}
