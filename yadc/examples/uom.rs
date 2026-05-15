//! Demonstrates `From`/`Into`/`TryFrom` between `uom`'s SI quantities and
//! both `yadc_si::StaticSIQuantity` and `yadc_si::DynamicSIQuantity`.

#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(unsized_const_params)]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]

use std::convert::TryFrom;

use uom::si::f64::{Length, Mass, Velocity};
use uom::si::length::{kilometer, meter};
use uom::si::mass::kilogram;
use uom::si::velocity::meter_per_second;
use yadc::{Format, Unit};
use yadc_si::{DynamicSIQuantity, StaticSIQuantity, mechanical};

fn main() {
    // -- uom -> StaticSIQuantity (From, infallible) -----------------------------
    let uom_dist = Length::new::<kilometer>(42.195);
    let static_dist: StaticSIQuantity<_> = uom_dist.into();
    println!(
        "marathon (static, from uom): {}",
        mechanical::LengthUnit::Kilometer.format_quantity(static_dist, Format::Short)
    );

    // -- StaticSIQuantity -> uom (Into, infallible) ----------------------------
    let yadc_v = mechanical::VelocityUnit::MeterPerSecond.quantity(25.0);
    let uom_v: Velocity = yadc_v.into();
    println!("velocity (to uom): {} m/s", uom_v.get::<meter_per_second>());

    // -- uom -> DynamicSIQuantity (From, infallible) ---------------------------
    let uom_mass = Mass::new::<kilogram>(70.0);
    let dyn_mass: DynamicSIQuantity = uom_mass.into();
    println!(
        "mass (dynamic, from uom): {:?}",
        mechanical::MassUnit::Kilogram.try_from_quantity(dyn_mass)
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
    let dyn_mass2: DynamicSIQuantity = Mass::new::<kilogram>(5.0).into();
    match Length::try_from(dyn_mass2) {
        Ok(_) => println!("unexpected success"),
        Err(recovered) => println!(
            "dim mismatch recovered mass: {:?}",
            mechanical::MassUnit::Kilogram.try_from_quantity(recovered)
        ),
    }
}
