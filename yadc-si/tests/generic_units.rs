//! A unit enum counts in any number type its constants convert to: each
//! variant's exact constant, rounded once to that type at compile time.
#![feature(generic_const_exprs, const_trait_impl, const_convert, const_ops)]
#![allow(incomplete_features)]

use yadc::Unit;
use yadc_si::KILO;
use yadc_si::mechanical::consts::METRE;
use yadc_si::mechanical::{Length, LengthUnit, Velocity, VelocityUnit};
use yadc_si::thermal::{ThermodynamicTemperature, ThermodynamicTemperatureUnit};

#[test]
fn one_enum_many_number_types() {
    let d: Length<f64> = LengthUnit::Foot.quantity(3.0);
    assert!((LengthUnit::Metre.value(d) - 0.9144).abs() < 1e-15);
    let d: Length<f32> = LengthUnit::Inch.quantity(10.0);
    assert!((LengthUnit::Metre.value(d) - 0.254).abs() < 1e-6);
}

/// An integer quantity counts whole coherent units (metres), each constant
/// converted to it on its own; an enum in `i32` would have to fit every unit
/// it names (light years) - a compile error.
#[test]
fn integers_count_in_converted_constants() {
    const M: Length<i32> = METRE.into();
    const KM: Length<i32> = (KILO * METRE).into();
    assert_eq!(M.value(KM * 3), 3000);
}

#[test]
fn scales_and_written_out_units() {
    let t: ThermodynamicTemperature<f64> = ThermodynamicTemperatureUnit::Celsius.quantity(25.0);
    assert!((ThermodynamicTemperatureUnit::Kelvin.value(t) - 298.15).abs() < 1e-12);
    // km/h has no constant (it is `KILO * METRE / HOUR`), but is a unit.
    let v: Velocity<f64> = VelocityUnit::KilometrePerHour.quantity(36.0);
    assert!((VelocityUnit::MetrePerSecond.value(v) - 10.0).abs() < 1e-12);
    assert_eq!("km/h".parse::<VelocityUnit>(), Ok(VelocityUnit::KilometrePerHour));
}
