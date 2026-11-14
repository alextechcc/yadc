//! A prefix is a constant of the plain number's unit: a scale. Every prefix
//! of a product multiplies into the unit's one scale, so a unit has one form
//! however it is built: `KILO * METRE` is the kilometre, `KILO * GRAM` the
//! kilogram, `KILOMETRE / METRE` is `KILO`. A prefix is not a base unit, so
//! neither is a prefixed unit. `{}` prints the scale as a prefix; `{:e}`
//! prints with the prefix that puts the number in [1, 1000).
#![feature(generic_const_exprs, const_trait_impl, const_ops)]
#![allow(incomplete_features)]

use yadc_si::mechanical::{
    FOOT, GRAM, HOUR, KILOMETRE, Length, MILE, MINUTE, Mass, NAUTICAL_MILE, NEWTON, PASCAL, Time,
};
use yadc_si::thermal::DEGREE_CELSIUS;
use yadc_si::{Exact, KILO, KILOGRAM, MEGA, METRE, MICRO, MILLI, NOT_BASE, SECOND, SIQuantity};

/// A megametre: a prefixed base unit.
const MEGAMETRE: SIQuantity<{ MEGA.unit().times(METRE.unit(), 1) }, Exact, NOT_BASE> = MEGA * METRE;
/// A micrometre.
const MICROMETRE: SIQuantity<{ MICRO.unit().times(METRE.unit(), 1) }, Exact, NOT_BASE> =
    MICRO * METRE;

#[test]
fn a_prefix_scales_a_unit() {
    let kilo_gram: Mass<Exact> = (KILO * GRAM).into();
    assert_eq!(kilo_gram, KILOGRAM);
    let mg: Mass<f64> = (250.0_f64 * MILLI * GRAM).into();
    assert!((f64::from(mg / KILOGRAM) - 250e-6).abs() < 1e-18);
    // Any named unit takes a prefix: a kilofoot is 304.8 m exactly.
    let kft: Length<f64> = (1.0_f64 * KILO * FOOT).into();
    assert_eq!(f64::from(kft / METRE), 304.8);
    // A prefix on a unit with a size of its own: milliminutes.
    let mmin: Time<f64> = (5.0_f64 * MILLI * MINUTE).into();
    assert!((f64::from(mmin / SECOND) - 0.3).abs() < 1e-15);
}

#[test]
fn a_prefixed_base_unit() {
    assert_eq!(MEGAMETRE.unit().size(), Exact::int(1_000_000));
    assert_eq!(KILO * METRE, KILOMETRE);
    let road: Length<f64> = (3.0_f64 * MEGAMETRE).into();
    assert_eq!(f64::from(road / METRE), 3e6);
    assert_eq!(f64::from(road / KILOMETRE), 3000.0);
    let hair: Length<f32> = (70.0_f32 * MICROMETRE).into();
    assert!((f32::from(hair / METRE) - 70e-6).abs() < 1e-10);
    let back: SIQuantity<{ MICROMETRE.unit() }, f32, NOT_BASE> = hair.into();
    assert!((f32::from(back / MICROMETRE) - 70.0).abs() < 1e-3);
}

#[test]
fn prefixed_units_print_their_prefix() {
    assert_eq!((2.5_f64 * KILO * GRAM).to_string(), "2.5 kg");
    assert_eq!((1.0_f64 * KILO * FOOT).to_string(), "1 kft");
    assert_eq!((1.0_f64 * MEGAMETRE).to_string(), "1 Mm");
    assert_eq!((1.0_f64 * MICROMETRE).to_string(), "1 μm");
    // A unit's own symbol is not a prefix's: `M` is the nautical mile.
    assert_eq!((1.0_f64 * NAUTICAL_MILE).to_string(), "1 M");
}

#[test]
fn prefixes_multiply_into_one_scale() {
    assert_eq!((KILO * KILO).unit(), (MEGA).unit());
    assert_eq!((KILOMETRE / METRE).unit(), (KILO).unit());
    assert_eq!((KILOMETRE * (MILLI * METRE)).unit(), (METRE * METRE).unit());
    assert_eq!((KILOMETRE * KILOMETRE / METRE).unit(), (MEGAMETRE).unit());
    assert_eq!((KILO * GRAM).unit(), (KILOGRAM).unit());
    assert_eq!((MILLI * KILOGRAM).unit(), (GRAM).unit());
}

#[test]
fn the_scale_prints_as_a_prefix() {
    assert_eq!((1.0_f64 * KILOMETRE * KILOMETRE).to_string(), "1 km^2");
    assert_eq!((1.0_f64 * KILOMETRE / HOUR).to_string(), "1 km/h");
    assert_eq!((1.0_f64 * MILLI * GRAM).to_string(), "1 mg");
    assert_eq!((1.0_f64 * GRAM).to_string(), "1 g");
    assert_eq!((1.0_f64 * KILO * PASCAL).to_string(), "1 kPa");
    // A torque's unit is an energy's: one canonical form, one name.
    assert_eq!((1.0_f64 * KILO * NEWTON * METRE).to_string(), "1 kJ");
    assert_eq!((1.0_f64 * KILO * METRE * METRE).to_string(), "1 1000·m^2");
    assert_eq!((2.0_f64 * KILO).to_string(), "2 k");
}

#[test]
fn engineering_picks_the_best_prefix() {
    assert_eq!(format!("{:e}", 0.002_f64 * KILOGRAM), "2 g");
    assert_eq!(format!("{:e}", 2500.0_f64 * KILOGRAM), "2.5 Mg");
    assert_eq!(format!("{:e}", 25_000.0_f64 * PASCAL), "25 kPa");
    assert_eq!(format!("{:e}", 0.0042_f64 * METRE), "4.2 mm");
    assert_eq!(format!("{:.1e}", 1500.0_f64 * METRE), "1.5 km");
    assert_eq!(format!("{:e}", 3.0e6_f64 * METRE * METRE), "3 km^2");
    assert_eq!(format!("{:e}", 1234.0_f64 * MILLI * METRE), "1.234 m");
    // Units of other sizes, or with a zero, print as built.
    assert_eq!(format!("{:e}", 3.0_f64 * MILE), "3 mi");
    assert_eq!(format!("{:e}", 3000.0_f64 * KILOMETRE / HOUR), "3000 km/h");
    assert_eq!(format!("{:e}", 20.0_f64 * DEGREE_CELSIUS), "20 °C");
    // A plain number prints as a number.
    assert_eq!(format!("{:e}", 2.0_f64 * KILOMETRE / METRE), "2000");
}
