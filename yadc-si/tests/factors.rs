//! Units with an exact definition carry it in full: a unit's size, an exact
//! number of base units, is the product of the sizes of the units it is
//! defined by, equal outright; and quantities, which round it to the number
//! type they count in, agree.
#![feature(generic_const_exprs, adt_const_params, unsized_const_params)]
#![allow(incomplete_features)]

use yadc_si::angular::REDUCED_PLANCK_CONSTANT;
use yadc_si::chemistry::AVOGADRO_CONSTANT;
use yadc_si::electromagnetic::{ELEMENTARY_CHARGE, FARADAY};
use yadc_si::mechanical::{
    ASTRONOMICAL_UNIT, BTU_IT, CUBIC_FOOT, CUBIC_INCH, DAY, FLUID_OUNCE, FOOT, FOOT_POUND,
    FOOT_SURVEY, GALLON, HORSEPOWER, INCH, LIGHT_YEAR, MILE_SURVEY, MILLIMETRE_OF_MERCURY, OUNCE,
    PARSEC, PLANCK_CONSTANT, POUND, POUND_FORCE, PSI, SLUG, SPEED_OF_LIGHT, SQUARE_INCH,
    STANDARD_ATMOSPHERE, STANDARD_GRAVITY, TORR,
};
use yadc_si::mechanical::{Length, Pressure};
use yadc_si::{Exact, METRE, MOLE, RADIAN, SECOND, SIQuantity};

/// A unit's exact size, in base units: read from its constant's type.
fn size<const U: yadc_si::SIUnit, const IS_BASE: bool>(_: SIQuantity<U, Exact, IS_BASE>) -> Exact {
    U.size()
}

#[test]
fn inch_based() {
    assert_eq!(size(INCH) * size(INCH), size(SQUARE_INCH));
    assert_eq!(size(INCH) * size(INCH) * size(INCH), size(CUBIC_INCH));
    assert_eq!(size(INCH) * Exact::int(12), size(FOOT));
    assert_eq!(size(FOOT) * size(FOOT) * size(FOOT), size(CUBIC_FOOT));
    assert_eq!(size(CUBIC_INCH) * Exact::int(231), size(GALLON));
    assert_eq!(size(GALLON) / Exact::int(128), size(FLUID_OUNCE));
    assert_eq!(size(FOOT_SURVEY) * Exact::int(5280), size(MILE_SURVEY));
}

#[test]
fn pound_based() {
    assert_eq!(size(OUNCE) * Exact::int(16), size(POUND));
    assert_eq!(size(POUND) * size(STANDARD_GRAVITY), size(POUND_FORCE));
    assert_eq!(
        size(POUND_FORCE) * size(SECOND) * size(SECOND) / size(FOOT),
        size(SLUG)
    );
    assert_eq!(size(FOOT) * size(POUND_FORCE), size(FOOT_POUND));
    assert_eq!(
        size(FOOT) * size(POUND_FORCE) * Exact::int(550) / size(SECOND),
        size(HORSEPOWER)
    );
    assert_eq!(size(POUND_FORCE) / size(SQUARE_INCH), size(PSI));
}

/// A product or quotient of unit constants is a unit too, its size the
/// product or quotient of theirs: no named "per" unit is needed.
#[test]
fn products_of_units_carry_their_sizes() {
    assert_eq!(size(POUND_FORCE / SQUARE_INCH), size(PSI));
    assert_eq!(size(SLUG / CUBIC_FOOT), size(SLUG) / size(CUBIC_FOOT));
    assert_eq!(size(FOOT * POUND_FORCE), size(FOOT_POUND));
    assert_eq!(size(BTU_IT / POUND), Exact::int(2326));
}

#[test]
fn exact_values() {
    assert_eq!(
        size(LIGHT_YEAR) / size(METRE),
        Exact::int(9_460_730_472_580_800)
    );
    assert_eq!(size(MILLIMETRE_OF_MERCURY), Exact::dec(133_322_387_415, -9));
}

#[test]
fn defined_by_other_units() {
    assert_eq!(size(STANDARD_ATMOSPHERE) / Exact::int(760), size(TORR));
    assert_eq!(
        size(DAY) * Exact::dec(36525, -2) * size(SPEED_OF_LIGHT),
        size(LIGHT_YEAR)
    );
    assert_eq!(
        size(PARSEC) * (Exact::PI / Exact::int(648_000)),
        size(ASTRONOMICAL_UNIT)
    );
    assert_eq!(
        size(ELEMENTARY_CHARGE) * size(AVOGADRO_CONSTANT) * size(MOLE),
        size(FARADAY)
    );
    assert_eq!(
        size(REDUCED_PLANCK_CONSTANT) * (size(RADIAN) * (Exact::int(2) * Exact::PI)),
        size(PLANCK_CONSTANT)
    );
}

/// Quantities convert with the size rounded once: an inch is 0.0254 m, a
/// light year 9460730472580800 m.
#[test]
fn quantities_agree() {
    let inch: Length<f64> = (1.0_f64 * INCH).into();
    assert_eq!(f64::from(inch / (1.0 * METRE)), 0.0254);
    let foot: Length<f64> = (1.0_f64 * FOOT).into();
    let in_inches: SIQuantity<{ INCH.unit() }, f64, false> = foot.into();
    assert!((f64::from(in_inches / (1.0 * INCH)) - 12.0).abs() < 1e-12);
    let ly: Length<f64> = (1.0_f64 * LIGHT_YEAR).into();
    assert_eq!(f64::from(ly / (1.0 * METRE)), 9_460_730_472_580_800.0);
    let atm: Pressure<f64> = (1.0_f64 * STANDARD_ATMOSPHERE).into();
    let in_torr: SIQuantity<{ TORR.unit() }, f64, false> = atm.into();
    assert!((f64::from(in_torr / (1.0 * TORR)) - 760.0).abs() < 1e-9);
}
