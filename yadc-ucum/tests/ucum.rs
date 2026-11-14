//! UCUM, as its XML defines it: its base units are the dimension's axes, its
//! values are exact, and its codes print and parse.
#![feature(generic_const_exprs, adt_const_params, unsized_const_params)]
#![allow(incomplete_features)]

use yadc_ucum::dynamic::heat::ThermodynamicTemperatureUnit;
use yadc_ucum::dynamic::intcust::LengthUnit;
use yadc_ucum::heat::{CELSIUS, ThermodynamicTemperature};
use yadc_ucum::intcust::{FOOT, INCH};
use yadc_ucum::Length;
use yadc_ucum::si::{MOLE, NEWTON};
use yadc_ucum::us::INCH as SURVEY_INCH;
use yadc_ucum::{Base, Exact, GRAM, KELVIN, METER, NOT_BASE, Qty, SECOND, UcumUnit};

/// A unit's size in base units, read from its constant's type.
fn size<const U: UcumUnit, const B: Base>(_: Qty<U, Exact, B>) -> Exact {
    U.size()
}

/// UCUM's values are decimals, read exactly: an inch is 0.0254 m, twelve
/// make a foot, and the US survey inch is 100/3937 m.
#[test]
fn values_are_exact() {
    assert_eq!(size(INCH), Exact::dec(254, -4));
    assert_eq!(size(INCH) * Exact::int(12), size(FOOT));
    assert_eq!(size(SURVEY_INCH), Exact::int(100) / Exact::int(3937));
}

/// The gram is a base unit, so the newton is a thousand of g·m/s²; the mole
/// is a number.
#[test]
fn base_units_are_ucums() {
    assert_eq!(size(GRAM), Exact::int(1));
    assert_eq!(size(METER), Exact::int(1));
    assert_eq!(size(NEWTON), Exact::int(1000));
    assert!(
        NEWTON
            .unit()
            .same_dimension((GRAM * METER / (SECOND * SECOND)).unit())
    );
    assert_eq!(size(MOLE), Exact::int(602_214_076_000_000_000_000_000));
    assert!(MOLE.unit().same_dimension(UcumUnit::ONE));
}

/// A unit prints and parses as its UCUM code.
#[test]
fn codes_print_and_parse() {
    let inch: LengthUnit = "[in_i]".parse().unwrap();
    assert_eq!(inch, LengthUnit::Inch);
    assert_eq!(inch.to_string(), "[in_i]");
    let metres = UcumUnit::from(inch).to_base(24.0);
    assert!((UcumUnit::from(LengthUnit::Foot).from_base(metres) - 2.0).abs() < 1e-15);
    let cel: ThermodynamicTemperatureUnit = "Cel".parse().unwrap();
    let kelvin = UcumUnit::from(cel).to_base(100.0);
    assert!(
        (UcumUnit::from(ThermodynamicTemperatureUnit::Kelvin).from_base(kelvin) - 373.15).abs()
            < 1e-12
    );
    assert!("[in_x]".parse::<LengthUnit>().is_err());
}

/// The static quantities convert the same way: through a base unit.
#[test]
fn quantities_convert_through_base_units() {
    let foot: Length<f64> = (2.0_f64 * FOOT).into();
    let inches: Qty<{ INCH.unit() }, f64, NOT_BASE> = foot.into();
    assert!((f64::from(inches / INCH) - 24.0).abs() < 1e-12);
    assert!((f64::from(foot / INCH) - 24.0).abs() < 1e-12);
    let t: ThermodynamicTemperature<f64> = (0.0_f64 * CELSIUS).into();
    assert!((f64::from(t / KELVIN) - 273.15).abs() < 1e-12);
    assert_eq!((24.0_f64 * INCH).to_string(), "24 [in_i]");
}
