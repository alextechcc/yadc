//! `From`/`Into`/`TryFrom` between `uom`'s SI quantities and `yadc-si`'s
//! (feature `uom`): both store the number in SI base units, so it moves as
//! it is.
//!
//! Run with: cargo run -p yadc --example uom
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use uom::si::f64 as u;
use uom::si::length::kilometer;
use uom::si::mass::kilogram;
use uom::si::velocity::meter_per_second;
use yadc_si::dynamic::DynamicSIQuantity;
use yadc_si::mechanical::*;
use yadc_si::{METRE, SECOND};

fn main() {
    // uom -> yadc-si: the base unit of the dimension.
    let marathon: Length<f64> = u::Length::new::<kilometer>(42.195).into();
    println!("marathon (from uom) = {marathon}");

    // yadc-si -> uom: from a base unit; another unit converts first.
    let v: Velocity<f64> = 25.0 * METRE / SECOND;
    let uom_v: u::Velocity = v.into();
    println!("velocity (to uom)   = {} m/s", uom_v.get::<meter_per_second>());
    let mile: Length<f64> = (1.0_f64 * MILE).into();
    let uom_mile: u::Length = mile.into();
    println!("a mile (to uom)     = {} km", uom_mile.get::<kilometer>());

    // uom -> DynamicSIQuantity, and back with a check of the unit.
    let d = DynamicSIQuantity::from(u::Mass::new::<kilogram>(72.5));
    println!("dynamic (from uom)  = {} {}", d.value, d.unit);
    let m = u::Mass::try_from(d).unwrap();
    println!("back to uom         = {} kg", m.get::<kilogram>());
    let not_a_length = u::Length::try_from(d);
    println!("as a length         = {:?}", not_a_length.is_ok());
    let static_mass: Mass<f64> = d.try_into().unwrap();
    println!("as a static mass    = {static_mass}");
}
