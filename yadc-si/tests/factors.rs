//! Units with an exact definition carry it in full: a unit built from the
//! units it is defined by is the unit's own constant, to the last bits.
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use core::f64::consts::PI;
use yadc::Unit;
use yadc_si::angular::consts::{RADIAN, REDUCED_PLANCK_CONSTANT};
use yadc_si::chemistry::consts::{AVOGADRO_CONSTANT, MOLE};
use yadc_si::density::consts::SLUG_PER_CUBIC_FOOT;
use yadc_si::electromagnetic::consts::{ELEMENTARY_CHARGE, FARADAY};
use yadc_si::mechanical::consts::{
    ASTRONOMICAL_UNIT, BTU_IT, BTU_IT_PER_POUND, CUBIC_FOOT, CUBIC_INCH, DAY, FLUID_OUNCE, FOOT,
    FOOT_POUND, FOOT_SURVEY, GALLON, HORSEPOWER, INCH, JOULE_PER_KILOGRAM, LIGHT_YEAR, METRE,
    MILE_SURVEY, MILLIMETRE_OF_MERCURY, OUNCE, PARSEC, PASCAL, PLANCK_CONSTANT, POUND, POUND_FORCE,
    POUND_FORCE_PER_SQUARE_INCH, PSI, SECOND, SLUG, SPEED_OF_LIGHT, SQUARE_INCH,
    STANDARD_ATMOSPHERE, STANDARD_GRAVITY, TORR,
};

/// `unit.value(q)` of a quantity built from the unit's definition is one, to
/// a few units in the last place.
fn one(ratio: f64) {
    assert!(
        (ratio - 1.0).abs() <= 4.0 * f64::EPSILON,
        "expected 1, got {ratio:e}"
    );
}

#[test]
fn inch_based() {
    one(SQUARE_INCH.value(INCH * INCH));
    one(CUBIC_INCH.value(INCH * INCH * INCH));
    one(FOOT.value(12.0 * INCH));
    one(CUBIC_FOOT.value(FOOT * FOOT * FOOT));
    one(GALLON.value(231.0 * CUBIC_INCH));
    one(FLUID_OUNCE.value(GALLON / 128.0));
    one(MILE_SURVEY.value(5280.0 * FOOT_SURVEY));
}

#[test]
fn pound_based() {
    one(POUND.value(16.0 * OUNCE));
    one(POUND_FORCE.value(POUND * STANDARD_GRAVITY));
    one(SLUG.value(POUND_FORCE * SECOND * SECOND / FOOT));
    one(SLUG_PER_CUBIC_FOOT.value(SLUG / CUBIC_FOOT));
    one(FOOT_POUND.value(FOOT * POUND_FORCE));
    one(HORSEPOWER.value(550.0 * FOOT * POUND_FORCE / SECOND));
    one(PSI.value(POUND_FORCE / SQUARE_INCH));
    one(POUND_FORCE_PER_SQUARE_INCH.value(POUND_FORCE / SQUARE_INCH));
    one(BTU_IT_PER_POUND.value(BTU_IT / POUND));
}

#[test]
fn exact_values() {
    assert_eq!(JOULE_PER_KILOGRAM.value(BTU_IT_PER_POUND), 2326.0);
    assert_eq!(METRE.value(LIGHT_YEAR), 9_460_730_472_580_800.0);
    assert_eq!(PASCAL.value(MILLIMETRE_OF_MERCURY), 133.322387415);
}

#[test]
fn defined_by_other_constants() {
    one(TORR.value(STANDARD_ATMOSPHERE / 760.0));
    one(LIGHT_YEAR.value(365.25 * DAY * SPEED_OF_LIGHT));
    one(ASTRONOMICAL_UNIT.value(PARSEC * (PI / 648_000.0)));
    one(FARADAY.value(ELEMENTARY_CHARGE * AVOGADRO_CONSTANT * MOLE));
    one(PLANCK_CONSTANT.value(REDUCED_PLANCK_CONSTANT * (2.0 * PI * RADIAN)));
}
