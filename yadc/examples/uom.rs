//! Demonstrates `From`/`Into`/`TryFrom` between `uom`'s SI quantities and
//! both `yadc_si::SIQuantity` and `yadc_si::DynamicSIQuantity`.
//!
//! Run with: cargo run -p yadc --example uom
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use std::convert::TryFrom;

use uom::si::f64::{Length, Mass, Velocity};
use uom::si::length::{kilometer, meter};
use uom::si::mass::kilogram;
use uom::si::velocity::meter_per_second;
use yadc_si::mechanical::{MassUnit::Kilogram, VelocityUnit::MetrePerSecond};
use yadc_si::dynamic::DynamicSIQuantity;
use yadc_si::{Prefix, Prefixed, SIQuantity, mechanical};
use yadc::Unit;

fn main() {
    // -- uom -> SIQuantity (From, infallible) -----------------------------
    let uom_dist = Length::new::<kilometer>(42.195);
    let static_dist: SIQuantity<_, f64> = uom_dist.into();
    let km = Prefixed { prefix: Some(Prefix::Kilo), unit: mechanical::LengthUnit::Metre };
    println!(
        "marathon (static, from uom): {} {km}",
        km.value(static_dist)
    );

    // -- SIQuantity -> uom (Into, infallible) ----------------------------
    // A unit builds it: 25 of `MetrePerSecond`.
    let yadc_v = MetrePerSecond.quantity(25.0);
    let uom_v: Velocity = yadc_v.into();
    println!("velocity (to uom): {} m/s", uom_v.get::<meter_per_second>());

    // -- uom -> DynamicSIQuantity (From, infallible) ---------------------------
    // Reading a runtime-dimensioned quantity in a unit goes through the static
    // quantity, checked: `Mass::try_from(d)` is `Err(d)` if the dimension does
    // not match.
    let uom_mass = Mass::new::<kilogram>(70.0);
    let dyn_mass: DynamicSIQuantity = uom_mass.into();
    println!(
        "mass (dynamic, from uom): {:?}",
        mechanical::Mass::try_from(dyn_mass).map(|m| Kilogram.value(m))
    );

    // -- DynamicSIQuantity -> uom (TryFrom, runtime dim check) -----------------
    let dyn_dist: DynamicSIQuantity = uom_dist.into();
    match Length::try_from(dyn_dist) {
        Ok(l) => println!(
            "round-trip DynamicSIQuantity -> uom Length: {} m",
            l.get::<meter>()
        ),
        Err(_) => println!("dim mismatch"),
    }

    // Dim mismatch: DynamicSIQuantity holding a mass can't convert to Length.
    // The error *is* the original `DynamicSIQuantity`, so nothing is lost.
    let dyn_mass2: DynamicSIQuantity = Mass::new::<kilogram>(5.0).into();
    match Length::try_from(dyn_mass2) {
        Ok(_) => println!("unexpected success"),
        Err(recovered) => println!(
            "dim mismatch recovered mass: {:?}",
            mechanical::Mass::try_from(recovered).map(|m| Kilogram.value(m))
        ),
    }
}
