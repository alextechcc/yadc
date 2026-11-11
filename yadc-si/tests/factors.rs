//! Units with an exact definition carry it in full: a unit built from the
//! units it is defined by is the unit's own constant. Counted in `Exact`,
//! the two are equal outright.
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use yadc::Unit;
use yadc_si::Exact;
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

#[test]
fn inch_based() {
    assert_eq!(INCH::<Exact> * INCH::<Exact>, SQUARE_INCH::<Exact>);
    assert_eq!(
        INCH::<Exact> * INCH::<Exact> * INCH::<Exact>,
        CUBIC_INCH::<Exact>
    );
    assert_eq!(Exact::int(12) * INCH::<Exact>, FOOT::<Exact>);
    assert_eq!(
        FOOT::<Exact> * FOOT::<Exact> * FOOT::<Exact>,
        CUBIC_FOOT::<Exact>
    );
    assert_eq!(Exact::int(231) * CUBIC_INCH::<Exact>, GALLON::<Exact>);
    assert_eq!(GALLON::<Exact> / Exact::int(128), FLUID_OUNCE::<Exact>);
    assert_eq!(
        Exact::int(5280) * FOOT_SURVEY::<Exact>,
        MILE_SURVEY::<Exact>
    );
}

#[test]
fn pound_based() {
    assert_eq!(Exact::int(16) * OUNCE::<Exact>, POUND::<Exact>);
    assert_eq!(
        POUND::<Exact> * STANDARD_GRAVITY::<Exact>,
        POUND_FORCE::<Exact>
    );
    assert_eq!(
        POUND_FORCE::<Exact> * SECOND::<Exact> * SECOND::<Exact> / FOOT::<Exact>,
        SLUG::<Exact>
    );
    assert_eq!(
        SLUG::<Exact> / CUBIC_FOOT::<Exact>,
        SLUG_PER_CUBIC_FOOT::<Exact>
    );
    assert_eq!(FOOT::<Exact> * POUND_FORCE::<Exact>, FOOT_POUND::<Exact>);
    assert_eq!(
        Exact::int(550) * FOOT::<Exact> * POUND_FORCE::<Exact> / SECOND::<Exact>,
        HORSEPOWER::<Exact>
    );
    assert_eq!(POUND_FORCE::<Exact> / SQUARE_INCH::<Exact>, PSI::<Exact>);
    assert_eq!(
        POUND_FORCE::<Exact> / SQUARE_INCH::<Exact>,
        POUND_FORCE_PER_SQUARE_INCH::<Exact>
    );
    assert_eq!(BTU_IT::<Exact> / POUND::<Exact>, BTU_IT_PER_POUND::<Exact>);
}

#[test]
fn exact_values() {
    assert_eq!(
        JOULE_PER_KILOGRAM.value(BTU_IT_PER_POUND::<Exact>),
        Exact::int(2326)
    );
    assert_eq!(
        METRE.value(LIGHT_YEAR::<Exact>),
        Exact::int(9_460_730_472_580_800)
    );
    assert_eq!(
        PASCAL.value(MILLIMETRE_OF_MERCURY::<Exact>),
        Exact::dec(133_322_387_415, -9)
    );
}

#[test]
fn defined_by_other_constants() {
    assert_eq!(
        STANDARD_ATMOSPHERE::<Exact> / Exact::int(760),
        TORR::<Exact>
    );
    assert_eq!(
        Exact::dec(36525, -2) * DAY::<Exact> * SPEED_OF_LIGHT::<Exact>,
        LIGHT_YEAR::<Exact>
    );
    assert_eq!(
        PARSEC::<Exact> * (Exact::PI / Exact::int(648_000)),
        ASTRONOMICAL_UNIT::<Exact>
    );
    assert_eq!(
        ELEMENTARY_CHARGE::<Exact> * AVOGADRO_CONSTANT::<Exact> * MOLE::<Exact>,
        FARADAY::<Exact>
    );
    assert_eq!(
        REDUCED_PLANCK_CONSTANT::<Exact> * (Exact::int(2) * Exact::PI * RADIAN::<Exact>),
        PLANCK_CONSTANT::<Exact>
    );
}
