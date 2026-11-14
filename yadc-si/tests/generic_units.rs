//! A unit is in a quantity's type: `SIQuantity<U, N, B>` is a number of
//! `U`s. Every unit is a constant, one of it; a number times it is a
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
    assert_eq!(f64::from(speed / (METRE / SECOND)), 20.0);
    let mass: Mass<f64> = 2.0 * KILOGRAM;
    let acceleration: Acceleration<f64> = speed / time;
    let force: Force<f64> = mass * acceleration;
    assert_eq!(f64::from(force / NEWTON), 0.4);
    assert_eq!(f64::from(distance / METRE), 2000.0);
    assert_eq!(f64::from(distance / KILOMETRE), 2.0);
}

#[test]
fn a_unit_with_a_zero() {
    let t: ThermodynamicTemperature<f64> = (25.0_f64 * CELSIUS).into();
    assert!((f64::from(t / KELVIN) - 298.15).abs() < 1e-9);
    // A unit with a zero has no quotient: a number of it is compared.
    let back: SIQuantity<{ CELSIUS.unit() }, f64, NOT_BASE> = t.into();
    assert!(back > 24.999 * CELSIUS && back < 25.001 * CELSIUS);
}

#[test]
fn a_log_unit() {
    let p: Power<f64> = (30.0_f64 * DECIBEL_MILLIWATT).into();
    assert!((f64::from(p / WATT) - 1.0).abs() < 1e-12);
    let level: SIQuantity<{ DECIBEL_MILLIWATT.unit() }, f64, NOT_BASE> = (2.0_f64 * WATT).into();
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
const KILOMETRE_PER_HOUR: SIQuantity<{ KILOMETRE.unit().times(HOUR.unit(), -1) }, Exact, NOT_BASE> =
    KILOMETRE / HOUR;

#[test]
fn a_quotient_of_units() {
    let v = Velocity::<f64>::from(36.0 * KILOMETRE_PER_HOUR);
    assert!((f64::from(v / (METRE / SECOND)) - 10.0).abs() < 1e-12);
    assert!((f64::from(v / KILOMETRE_PER_HOUR) - 36.0).abs() < 1e-12);
    assert_eq!((36.0_f64 * KILOMETRE_PER_HOUR).to_string(), "36 km/h");
}

/// A base unit and another multiply into a unit that is not a base unit:
/// `KILO * METRE` is the kilometre, `KILOMETRE / SECOND` km/s.
#[test]
fn base_and_other_units_multiply() {
    assert_eq!(KILO * METRE, KILOMETRE);
    let fast = 2.0_f64 * KILOMETRE / SECOND;
    assert_eq!(fast.to_string(), "2 km/s");
    let v: Velocity<f64> = fast.into();
    assert_eq!(f64::from(v / (METRE / SECOND)), 2000.0);
    let p = 3.0_f64 * KILO * PASCAL;
    assert_eq!(p.to_string(), "3 kPa");
    let p: Pressure<f64> = p.into();
    assert_eq!(f64::from(p / PASCAL), 3000.0);
}

/// A plain number in an integer, where its unit's size is a whole number.
#[test]
fn whole_numbers() {
    assert_eq!(i32::from(2 * KILOMETRE / METRE), 2000);
    assert_eq!(i64::from(3_i64 * HOUR / SECOND), 10_800);
}

/// A number divided by a quantity.
#[test]
fn a_number_over_a_quantity() {
    let f: Frequency<f64> = 1.0 / (2.0 * SECOND);
    assert_eq!(f64::from(f / HERTZ), 0.5);
}

#[test]
fn display() {
    assert_eq!((2.0_f64 * KILOMETRE).to_string(), "2 km");
    let force: Force<f64> = (3.0 * KILOGRAM) * (2.0 * METRE / (SECOND * SECOND));
    assert_eq!(force.to_string(), "6 N");
    assert_eq!((1.5_f64 * METRE / SECOND).to_string(), "1.5 m/s");
    assert_eq!((METRE / (SECOND * SECOND)).unit().to_string(), "m/s^2");
    assert_eq!(
        (KILOGRAM * METRE * METRE / (SECOND * SECOND * KELVIN))
            .unit()
            .to_string(),
        "kg·m^2/(K·s^2)"
    );
    assert_eq!(format!("[{:>5}]", KILOMETRE.unit()), "[   km]");
    assert_eq!(format!("[{:*<4}]", (METRE / SECOND).unit()), "[m/s*]");
    assert_eq!(CELSIUS.unit().size().to_string(), "1");
    // A temperature step of size one with its own symbol is its own unit.
    assert_eq!((5.0_f64 * DEGREE_CELSIUS).to_string(), "5 °C");
    let step: TemperatureInterval<f64> = (5.0_f64 * DEGREE_CELSIUS).into();
    assert_eq!(step.to_string(), "5 K");
    assert_eq!(Exact::dec(27315, -2).to_string(), "273.15");
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
    assert_eq!(v.is_base, BASE);
    assert!((d + t).is_none());
    let twice = (d + d).unwrap();
    assert_eq!(Length::<f64>::try_from(twice).unwrap(), 6.0 * METRE);
    assert!((DynamicSIQuantity::from(1.0 * CELSIUS) * d).is_none());
    // A unit that is not a base unit times one that is.
    let p = DynamicSIQuantity::from(1.0 * BAR);
    let force = (p * (DynamicSIQuantity::from(2.0 * METRE) * d).unwrap()).unwrap();
    assert_eq!(force.is_base, NOT_BASE);
    assert_eq!(force.unit.to_string(), "bar·m^2");
}

/// A number divided by a unit constant, and a unit printed in a column.
#[test]
fn number_over_unit_and_padding() {
    let rate = 1.0 / SECOND;
    assert_eq!(format!("{rate}"), "1 Hz");
    assert_eq!(format!("[{:>4}]", KILOMETRE.unit()), "[  km]");
    assert_eq!(format!("[{:<4}]", KILOMETRE.unit()), "[km  ]");
    assert_eq!(format!("{:.1}", KILOMETRE.unit()), "km");
}
