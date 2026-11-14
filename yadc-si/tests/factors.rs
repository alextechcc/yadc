//! Units with an exact definition carry it in full: a unit's size, an exact
//! number of coherent units, is the product of the sizes of the units it is
//! defined by, equal outright; and the units, which round it to the number
//! type they count in, agree.
#![feature(generic_const_exprs, adt_const_params, unsized_const_params)]
#![allow(incomplete_features)]

use yadc::Unit;
use yadc_si::angular::units::{RADIAN, REDUCED_PLANCK_CONSTANT};
use yadc_si::chemistry::units::{AVOGADRO_CONSTANT, MOLE};
use yadc_si::density::units::SLUG_PER_CUBIC_FOOT;
use yadc_si::electromagnetic::units::{ELEMENTARY_CHARGE, FARADAY};
use yadc_si::mechanical::units::{
    ASTRONOMICAL_UNIT, BTU_IT, BTU_IT_PER_POUND, CUBIC_FOOT, CUBIC_INCH, DAY, FLUID_OUNCE, FOOT,
    FOOT_POUND, FOOT_SURVEY, GALLON, HORSEPOWER, INCH, JOULE_PER_KILOGRAM, LIGHT_YEAR, METRE,
    MILE_SURVEY, MILLIMETRE_OF_MERCURY, OUNCE, PARSEC, PLANCK_CONSTANT, POUND, POUND_FORCE,
    POUND_FORCE_PER_SQUARE_INCH, PSI, SECOND, SLUG, SPEED_OF_LIGHT, SQUARE_INCH,
    STANDARD_ATMOSPHERE, STANDARD_GRAVITY, TORR,
};
use yadc_si::{Exact, ISQ, SIUnit, SIUnitConsts};

/// A unit's exact size, in coherent units of its quantity.
fn size<const D: ISQ, const C: SIUnitConsts>(_: SIUnit<D, C>) -> Exact {
    C.size
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
    assert_eq!(size(SLUG) / size(CUBIC_FOOT), size(SLUG_PER_CUBIC_FOOT));
    assert_eq!(size(FOOT) * size(POUND_FORCE), size(FOOT_POUND));
    assert_eq!(
        size(FOOT) * size(POUND_FORCE) * Exact::int(550) / size(SECOND),
        size(HORSEPOWER)
    );
    assert_eq!(size(POUND_FORCE) / size(SQUARE_INCH), size(PSI));
    assert_eq!(size(PSI), size(POUND_FORCE_PER_SQUARE_INCH));
    assert_eq!(size(BTU_IT) / size(POUND), size(BTU_IT_PER_POUND));
}

#[test]
fn exact_values() {
    assert_eq!(
        size(BTU_IT_PER_POUND) / size(JOULE_PER_KILOGRAM),
        Exact::int(2326)
    );
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

/// The units read and build with the size rounded once: an inch is 0.0254
/// m, a light year 9460730472580800 m.
#[test]
fn units_agree() {
    assert_eq!(METRE.value(INCH.quantity(1.0_f64)), 0.0254);
    assert!((INCH.value(FOOT.quantity(1.0_f64)) - 12.0).abs() < 1e-12);
    assert_eq!(
        METRE.value(LIGHT_YEAR.quantity(1.0_f64)),
        9_460_730_472_580_800.0
    );
    assert!((TORR.value(STANDARD_ATMOSPHERE.quantity(1.0_f64)) - 760.0).abs() < 1e-9);
}
