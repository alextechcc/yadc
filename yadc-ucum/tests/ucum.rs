//! UCUM, as its XML defines it: its base units are the dimension's axes, its
//! values are exact, and its codes print and parse.
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use yadc::Unit;
use yadc_ucum::dynamic::heat::ThermodynamicTemperatureUnit;
use yadc_ucum::dynamic::intcust::LengthUnit;
use yadc_ucum::intcust::consts::{FOOT, INCH};
use yadc_ucum::si::consts::{GRAM, METRE, MOLE, NEWTON, ONE, SECOND};
use yadc_ucum::us::consts::INCH as SURVEY_INCH;
use yadc_ucum::{Exact, KILO};

/// UCUM's values are decimals, read exactly: an inch is 0.0254 m, twelve
/// make a foot, and the US survey inch is 100/3937 m.
#[test]
fn values_are_exact() {
    assert_eq!(INCH, METRE * Exact::dec(254, -4));
    assert_eq!(INCH * Exact::int(12), FOOT);
    assert_eq!(SURVEY_INCH, METRE * (Exact::int(100) / Exact::int(3937)));
}

/// The gram is a base unit, so the newton is a thousand of g·m/s²; the mole
/// is a number.
#[test]
fn base_units_are_ucums() {
    assert_eq!(NEWTON, KILO * GRAM * METRE / (SECOND * SECOND));
    assert_eq!(MOLE, ONE * Exact::int(602_214_076_000_000_000_000_000));
}

/// A unit prints and parses as its UCUM code.
#[test]
fn codes_print_and_parse() {
    let inch: LengthUnit = "[in_i]".parse().unwrap();
    assert_eq!(inch, LengthUnit::Inch);
    assert_eq!(inch.to_string(), "[in_i]");
    assert!((LengthUnit::Foot.value(inch.quantity(24.0_f64)) - 2.0).abs() < 1e-15);
    let cel: ThermodynamicTemperatureUnit = "Cel".parse().unwrap();
    let t = cel.quantity(100.0_f64);
    assert!((ThermodynamicTemperatureUnit::Kelvin.value(t) - 373.15).abs() < 1e-12);
}
