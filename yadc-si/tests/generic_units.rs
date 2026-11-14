//! A unit is in a quantity's type: `SIQuantity<U, N, IS_BASE>` is a number
//! of `U`s. Every unit is a constant, one of it; a number times it is a
//! quantity, `From` converts to a base unit of its dimension and back, and
//! products of base units are base units.
#![feature(generic_const_exprs, const_trait_impl, const_ops)]
#![allow(incomplete_features)]

use yadc_si::dynamic::DynamicSIQuantity;
use yadc_si::dynamic::mechanical::LengthUnit;
use yadc_si::mechanical::*;
use yadc_si::thermal::*;
use yadc_si::*;

#[test]
fn user_code() {
    let distance: Length<f64> = (2.0_f64 * KILOMETRE).into();
    let time: Time<f64> = 100.0 * SECOND;
    let speed: Velocity<f64> = distance / time;
    assert_eq!(f64::from(speed / (1.0 * METRE / SECOND)), 20.0);
    let mass: Mass<f64> = 2.0 * KILOGRAM;
    let acceleration: Acceleration<f64> = speed / time;
    let force: Force<f64> = mass * acceleration;
    assert_eq!(f64::from(force / (1.0 * NEWTON)), 0.4);
    let n = f64::from(distance / (1.0 * METRE));
    assert_eq!(n, 2000.0);
}

#[test]
fn a_unit_with_a_zero() {
    let t: ThermodynamicTemperature<f64> = (25.0_f64 * CELSIUS).into();
    assert!((f64::from(t / (1.0 * KELVIN)) - 298.15).abs() < 1e-9);
    // A unit with a zero has no quotient: a number of it is compared.
    let back: SIQuantity<{ CELSIUS.unit() }, f64, false> = t.into();
    assert!(back > 24.999 * CELSIUS && back < 25.001 * CELSIUS);
}

#[test]
fn a_log_unit() {
    let p: Power<f64> = (30.0_f64 * DECIBEL_MILLIWATT).into();
    assert!((f64::from(p / (1.0 * WATT)) - 1.0).abs() < 1e-12);
    let level: SIQuantity<{ DECIBEL_MILLIWATT.unit() }, f64, false> = (2.0_f64 * WATT).into();
    assert!(level > 33.0102 * DECIBEL_MILLIWATT && level < 33.0104 * DECIBEL_MILLIWATT);
}

#[test]
fn kilogram_is_kilo_gram() {
    let kilo_gram: Mass<Exact> = (KILO * GRAM).into();
    assert_eq!(KILOGRAM, kilo_gram);
    let force: Force<f64> = (3.0 * KILOGRAM) * (2.0 * METRE / (SECOND * SECOND));
    assert_eq!(force, 6.0 * NEWTON);
}

/// A quotient of two unit constants, as a constant of its own.
const KILOMETRE_PER_HOUR: SIQuantity<{ KILOMETRE.unit().times(HOUR.unit(), -1) }, Exact, false> =
    KILOMETRE / HOUR;

#[test]
fn a_quotient_of_units() {
    let v = Velocity::<f64>::from(36.0 * KILOMETRE_PER_HOUR);
    assert!((f64::from(v / (1.0 * METRE / SECOND)) - 10.0).abs() < 1e-12);
    assert_eq!((36.0_f64 * KILOMETRE_PER_HOUR).to_string(), "36 h^-1·km");
}

#[test]
fn display() {
    assert_eq!((2.0_f64 * KILOMETRE).to_string(), "2 km");
    let force: Force<f64> = (3.0 * KILOGRAM) * (2.0 * METRE / (SECOND * SECOND));
    assert_eq!(force.to_string(), "6 N");
    assert_eq!((1.5_f64 * METRE / SECOND).to_string(), "1.5 m·s^-1");
}

#[test]
fn unit_enum_round_trip() {
    let u: LengthUnit = "km".parse().unwrap();
    assert_eq!(u, LengthUnit::Kilometre);
    assert_eq!(u.to_string(), "km");
    assert_eq!(SIUnit::from(u), KILOMETRE.unit());
    let unit = SIUnit::from(u);
    assert_eq!(unit.from_base(unit.to_base(3.0)), 3.0);
    assert_eq!(METRE.unit().from_base(unit.to_base(3.0)), 3000.0);
}

#[test]
fn dynamic_arithmetic() {
    let d = DynamicSIQuantity::from(3.0 * METRE);
    let t = DynamicSIQuantity::from(2.0 * SECOND);
    let v = (d / t).unwrap();
    assert_eq!(v.unit, (METRE / SECOND).unit());
    assert_eq!(v.value, 1.5);
    assert!(v.is_base);
    assert!((d + t).is_none());
    let twice = (d + d).unwrap();
    assert_eq!(Length::<f64>::try_from(twice).unwrap(), 6.0 * METRE);
    assert!((DynamicSIQuantity::from(1.0 * CELSIUS) * d).is_none());
}
