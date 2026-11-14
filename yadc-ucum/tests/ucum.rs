//! UCUM, as its XML defines it: its base units are the dimension's axes, its
//! values are exact, and its codes print and parse.
#![feature(generic_const_exprs, adt_const_params, unsized_const_params)]
#![allow(incomplete_features)]

use yadc::Unit;
use yadc_ucum::dynamic::heat::ThermodynamicTemperatureUnit;
use yadc_ucum::dynamic::intcust::LengthUnit;
use yadc_ucum::heat::units::{Celsius, KelvinScale};
use yadc_ucum::intcust::units::{Foot, Inch};
use yadc_ucum::si::units::{Gram, Meter, Mole, Newton};
use yadc_ucum::us::units::Inch as SurveyInch;
use yadc_ucum::{Exact, UcumUnit, UcumUnitConsts};

/// A unit's size in coherent units, read from its type.
fn size<const D: yadc_ucum::Dim, const C: UcumUnitConsts>(_: UcumUnit<D, C>) -> Exact {
    C.size
}

/// UCUM's values are decimals, read exactly: an inch is 0.0254 m, twelve
/// make a foot, and the US survey inch is 100/3937 m.
#[test]
fn values_are_exact() {
    assert_eq!(size(Inch {}), Exact::dec(254, -4));
    assert_eq!(size(Inch {}) * Exact::int(12), size(Foot {}));
    assert_eq!(size(SurveyInch {}), Exact::int(100) / Exact::int(3937));
}

/// The gram is a base unit, so the newton is a thousand of g·m/s²; the mole
/// is a number.
#[test]
fn base_units_are_ucums() {
    assert_eq!(size(Gram {}), Exact::int(1));
    assert_eq!(size(Meter {}), Exact::int(1));
    assert_eq!(size(Newton {}), Exact::int(1000));
    assert_eq!(size(Mole {}), Exact::int(602_214_076_000_000_000_000_000));
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
    assert!((KelvinScale {}.value(Celsius {}.quantity(0.0_f64)) - 273.15).abs() < 1e-12);
}
