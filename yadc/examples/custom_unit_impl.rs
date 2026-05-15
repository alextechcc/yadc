//! Implementing the `Unit` trait by hand.
//!
//! Conversion lives entirely on the `Unit` trait (`quantity` / `from_quantity`),
//! so a unit does not have to come from `#[derive(Unit)]`. Here we hand-write
//! `impl Unit` for a unit whose conversion factor is only known at *runtime* - a
//! calibrated sensor gain - which the derive (compile-time expressions) cannot
//! express. It interoperates with the derived `yadc-si` units for free.
//!
//! Run with: cargo run --example custom_unit_impl --manifest-path yadc/Cargo.toml

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use yadc::{Format, Unit};
use yadc_si::mechanical::{Length, LengthUnit};

/// A length unit whose scale is a runtime calibration factor: one encoder
/// "tick" equals `meters_per_tick` meters. An enum + `#[derive(Unit)]` cannot
/// capture this - the factor is a value, not a compile-time constant.
#[derive(Copy, Clone)]
struct SensorTicks {
    meters_per_tick: f64,
}

impl Unit for SensorTicks {
    type Quantity = Length;

    fn quantity(self, value: f64) -> Length {
        // Convert ticks -> meters, then lift the scalar into the typed quantity.
        // `Length::from_canonical` is the sanctioned constructor: no private field
        // is touched, and it bypasses the kind-gated scalar `Mul`.
        let meters = value * self.meters_per_tick;
        Length::from_canonical(meters)
    }

    fn from_quantity(self, q: Length) -> f64 {
        // Divide out the unit to recover the dimensionless SI magnitude.
        let meters: f64 = (q / Length::from_canonical(1.0)).into();
        meters / self.meters_per_tick
    }

    fn symbol(self) -> &'static str {
        "tick"
    }
    fn long(self) -> &'static str {
        "sensor ticks"
    }
}

fn main() {
    let encoder = SensorTicks {
        meters_per_tick: 0.0125, // 12.5 mm per tick
    };

    // 800 ticks -> a real `Length`, readable in any derived unit.
    let travel = encoder.quantity(800.0);
    println!(
        "800 ticks = {}",
        LengthUnit::Meter.format_quantity(travel, Format::Long)
    );
    println!(
        "          = {}",
        LengthUnit::Kilometer.format_quantity(travel, Format::Short)
    );

    // `format_quantity` (a provided method) works for the hand impl too.
    println!("round-trip: {}", encoder.format_quantity(travel, Format::Long));

    // The custom unit composes with derived ones: build from km, read as ticks.
    let one_km = LengthUnit::Kilometer.quantity(1.0);
    println!("1 km = {} ticks", encoder.from_quantity(one_km));

    // A different calibration is just a different value of the same unit type.
    let coarse = SensorTicks { meters_per_tick: 1.0 };
    println!("1 km = {} coarse ticks", coarse.from_quantity(one_km));
}
