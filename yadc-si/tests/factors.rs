//! Units with an exact definition carry it in full: a unit built from the
//! units it is defined by is the unit's own constant. Counted in `Exact`,
//! the two are equal outright, and a unit enum's variant is that value
//! rounded to `f64`.
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use yadc::Unit;
use yadc_si::Exact;
use yadc_si::angular::consts::{RADIAN, REDUCED_PLANCK_CONSTANT};
use yadc_si::chemistry::consts::{AVOGADRO_CONSTANT, MOLE};
use yadc_si::density::MassDensity;
use yadc_si::dynamic::density::MassDensityUnit;
use yadc_si::electromagnetic::consts::{ELEMENTARY_CHARGE, FARADAY};
use yadc_si::mechanical::consts::{
    ASTRONOMICAL_UNIT, BTU_IT, CUBIC_FOOT, CUBIC_INCH, DAY, FLUID_OUNCE, FOOT, FOOT_SURVEY, GALLON,
    HORSEPOWER, INCH, JOULE, KILOGRAM, LIGHT_YEAR, METRE, MILE_SURVEY, MILLIMETRE_OF_MERCURY,
    OUNCE, PARSEC, PASCAL, PLANCK_CONSTANT, POUND, POUND_FORCE, PSI, SECOND, SLUG, SPEED_OF_LIGHT,
    SQUARE_INCH, STANDARD_ATMOSPHERE, STANDARD_GRAVITY, TORR,
};
use yadc_si::mechanical::{AvailableEnergy, Energy, Pressure};
use yadc_si::dynamic::mechanical::{AvailableEnergyUnit, EnergyUnit, PressureUnit};

#[test]
fn inch_based() {
    assert_eq!(INCH * INCH, SQUARE_INCH);
    assert_eq!(INCH * INCH * INCH, CUBIC_INCH);
    assert_eq!(INCH * Exact::int(12), FOOT);
    assert_eq!(FOOT * FOOT * FOOT, CUBIC_FOOT);
    assert_eq!(CUBIC_INCH * Exact::int(231), GALLON);
    assert_eq!(GALLON / Exact::int(128), FLUID_OUNCE);
    assert_eq!(FOOT_SURVEY * Exact::int(5280), MILE_SURVEY);
}

#[test]
fn pound_based() {
    assert_eq!(OUNCE * Exact::int(16), POUND);
    assert_eq!(POUND * STANDARD_GRAVITY, POUND_FORCE);
    assert_eq!(POUND_FORCE * SECOND * SECOND / FOOT, SLUG);
    assert_eq!(
        MassDensity::<f64>::from(SLUG / CUBIC_FOOT),
        MassDensityUnit::SlugPerCubicFoot.quantity(1.0)
    );
    assert_eq!(
        Energy::<f64>::from(FOOT * POUND_FORCE),
        EnergyUnit::FootPound.quantity(1.0)
    );
    assert_eq!(FOOT * POUND_FORCE * Exact::int(550) / SECOND, HORSEPOWER);
    assert_eq!(POUND_FORCE / SQUARE_INCH, PSI);
    assert_eq!(
        Pressure::<f64>::from(POUND_FORCE / SQUARE_INCH),
        PressureUnit::PoundForcePerSquareInch.quantity(1.0)
    );
    assert_eq!(
        AvailableEnergy::<f64>::from(BTU_IT / POUND),
        AvailableEnergyUnit::BtuItPerPound.quantity(1.0)
    );
}

#[test]
fn exact_values() {
    assert_eq!((JOULE / KILOGRAM).value(BTU_IT / POUND), Exact::int(2326));
    assert_eq!(METRE.value(LIGHT_YEAR), Exact::int(9_460_730_472_580_800));
    assert_eq!(
        PASCAL.value(MILLIMETRE_OF_MERCURY),
        Exact::dec(133_322_387_415, -9)
    );
}

#[test]
fn defined_by_other_constants() {
    assert_eq!(STANDARD_ATMOSPHERE / Exact::int(760), TORR);
    assert_eq!(DAY * Exact::dec(36525, -2) * SPEED_OF_LIGHT, LIGHT_YEAR);
    assert_eq!(
        PARSEC * (Exact::PI / Exact::int(648_000)),
        ASTRONOMICAL_UNIT
    );
    assert_eq!(ELEMENTARY_CHARGE * AVOGADRO_CONSTANT * MOLE, FARADAY);
    assert_eq!(
        REDUCED_PLANCK_CONSTANT * (RADIAN * (Exact::int(2) * Exact::PI)),
        PLANCK_CONSTANT
    );
}
