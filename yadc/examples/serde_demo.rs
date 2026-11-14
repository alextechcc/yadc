//! serde with `yadc-si` quantities (feature `serde`).
//!
//! A quantity serializes as just its number: the unit is in the type, so
//! the deserialization target names it. A struct of quantities derives
//! `Serialize` and `Deserialize` as usual.
//!
//! Run with: cargo run -p yadc --example serde_demo
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use serde::{Deserialize, Serialize};
use yadc_si::mechanical::*;
use yadc_si::thermal::*;
use yadc_si::{KILOGRAM, NOT_BASE, SIQuantity};

/// A record of quantities: one in a base unit, one in hours.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Leg {
    load: Mass<f64>,
    duration: SIQuantity<{ HOUR.unit() }, f64, NOT_BASE>,
    outside: ThermodynamicTemperature<f64>,
}

fn main() {
    let m: Mass<f64> = 1.5 * KILOGRAM;
    let s = serde_json::to_string(&m).unwrap();
    println!("1.5 kg      -> {s}");
    let back: Mass<f64> = serde_json::from_str(&s).unwrap();
    println!("round trip  -> {back}");
    // The number is read in the target's unit.
    let t: Time<f64> = serde_json::from_str("90").unwrap();
    println!("90 as Time  -> {t}");

    let leg = Leg {
        load: 20.0 * KILOGRAM,
        duration: 2.5 * HOUR,
        outside: (12.0_f64 * CELSIUS).into(),
    };
    let json = serde_json::to_string(&leg).unwrap();
    println!("leg         -> {json}");
    let parsed: Leg = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, leg);
    println!("parsed      -> {} for {}", parsed.load, parsed.duration);
}
