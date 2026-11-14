//! A unit is a constant whose type is its dimension and definition,
//! `SIUnit<D, C>`: `MILE.quantity(3.0)` builds, `MILE.value(d)` reads, in any
//! number type its size converts to. An enum of them is a run-time choice,
//! a unit the same way; `DynamicSIUnit` is a unit of any dimension.
#![feature(generic_const_exprs, const_trait_impl)]
#![allow(incomplete_features)]

use core::ops::{Add, Div, Mul, Sub};
use yadc_si::SIQuantity;
use yadc_si::dynamic::DynamicSIUnit;
use yadc_si::dynamic::mechanical::{LengthUnit, VelocityUnit};
use yadc_si::dynamic::thermal::ThermodynamicTemperatureUnit;
use yadc_si::mechanical::units::{
    DECIBEL_MILLIWATT, FOOT, INCH, JULIAN_DATE, KILOMETRE_PER_HOUR, METRE, METRE_PER_SECOND, MILE,
    UNIX_SECOND, WATT,
};
use yadc_si::mechanical::{LENGTH, POWER};
use yadc_si::thermal::units::{CELSIUS, FAHRENHEIT, KELVIN_SCALE};
use yadc_si::{Exact, Unit};
use yadc_si::{Prefix, Prefixed, SIUnit};

#[test]
fn a_unit_builds_and_reads() {
    let d = MILE.quantity(3.0_f64);
    assert!((METRE.value(d) - 4828.032).abs() < 1e-9);
    assert!((MILE.value(d) - 3.0).abs() < 1e-15);
    assert!((METRE_PER_SECOND.value(KILOMETRE_PER_HOUR.quantity(36.0_f64)) - 10.0).abs() < 1e-12);
}

#[test]
fn any_number_type() {
    let d: SIQuantity<LENGTH, f32> = INCH.quantity(10.0_f32);
    assert!((METRE.value(d) - 0.254).abs() < 1e-6);
    // An integer counts whole metres.
    let k: SIQuantity<LENGTH, i32> = METRE.quantity(1609);
    assert_eq!(METRE.value(k), 1609);
}

/// Folded at compile time.
const MARATHON: SIQuantity<LENGTH, f64> = METRE.quantity(42195.0);

#[test]
fn const_units() {
    assert!((MILE.value(MARATHON) - 26.218757).abs() < 1e-6);
}

/// Generic over the number type: what a unit asks of it.
fn in_miles<
    N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>,
>(
    d: SIQuantity<LENGTH, N>,
) -> N {
    MILE.value(d)
}

/// Generic over the unit: any unit of lengths.
fn round_trip<U: Unit<SIQuantity<LENGTH, f64>, f64>>(u: U, n: f64) -> f64 {
    u.value(u.quantity(n))
}

#[test]
fn generic_code() {
    assert!((in_miles(MARATHON) - 26.218757).abs() < 1e-6);
    assert!((round_trip(FOOT, 2.5) - 2.5).abs() < 1e-12);
    assert!((round_trip(LengthUnit::Inch, 2.5) - 2.5).abs() < 1e-12);
}

#[test]
fn zeros() {
    let t = CELSIUS.quantity(25.0_f64);
    assert!((KELVIN_SCALE.value(t) - 298.15).abs() < 1e-12);
    assert!((FAHRENHEIT.value(t) - 77.0).abs() < 1e-12);
    assert!(UNIX_SECOND.value(JULIAN_DATE.quantity(2440587.5_f64)).abs() < 1e-6);
    let p = SIQuantity::<POWER, f64>::from(DECIBEL_MILLIWATT.quantity(30.0_f64));
    assert!((WATT.value(p) - 1.0).abs() < 1e-12);
}

#[test]
fn prints_and_parses() {
    assert_eq!(format!("{MILE} / {MILE:#}"), "mi / miles");
    assert_eq!("ft".parse::<LengthUnit>(), Ok(LengthUnit::Foot));
    assert_eq!(LengthUnit::Foot.to_string(), "ft");
    assert_eq!(
        "km/h".parse::<VelocityUnit>(),
        Ok(VelocityUnit::KilometrePerHour)
    );
}

#[test]
fn an_enum_is_a_run_time_choice() {
    let setting = LengthUnit::Inch;
    assert!((setting.value(FOOT.quantity(1.0_f64)) - 12.0).abs() < 1e-12);
    let c = ThermodynamicTemperatureUnit::Celsius;
    assert!(c.value(KELVIN_SCALE.quantity(273.15_f64)).abs() < 1e-12);
}

#[test]
fn a_prefixed_unit() {
    let km = Prefixed {
        prefix: Some(Prefix::Kilo),
        unit: LengthUnit::Metre,
    };
    assert!((METRE.value(km.quantity(3.0_f64)) - 3000.0).abs() < 1e-9);
    assert!((km.value(MILE.quantity(1.0_f64)) - 1.609344).abs() < 1e-12);
    assert_eq!(km.to_string(), "km");
    assert_eq!("km".parse::<Prefixed<LengthUnit>>(), Ok(km));
}

/// The open level: a unit of any dimension.
#[test]
fn a_dynamic_unit() {
    let mile = DynamicSIUnit::from(MILE);
    let metre = DynamicSIUnit::from(LengthUnit::Metre);
    let d = mile.quantity(Some(2.0));
    assert!((metre.value(d).unwrap() - 3218.688).abs() < 1e-9);
    assert_eq!(DynamicSIUnit::from(CELSIUS).value(d), None);
    assert!(SIUnit::try_from(mile).is_ok_and(|u: SIUnit<_, _>| u == MILE));
    assert_eq!(mile.to_string(), "mi");
}
