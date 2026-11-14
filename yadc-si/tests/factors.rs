//! Units with an exact definition carry it in full: a unit's size, an exact
//! number of coherent units, is the product of the sizes of the units it is
//! defined by, equal outright; and the unit types, which round it to the
//! number type they count in, agree.
#![feature(generic_const_exprs, adt_const_params, unsized_const_params)]
#![allow(incomplete_features)]

use yadc::Unit;
use yadc_si::angular::units::{Radian, ReducedPlanckConstant};
use yadc_si::chemistry::units::{AvogadroConstant, Mole};
use yadc_si::density::units::SlugPerCubicFoot;
use yadc_si::electromagnetic::units::{ElementaryCharge, Faraday};
use yadc_si::mechanical::units::{
    AstronomicalUnit, BtuIt, BtuItPerPound, CubicFoot, CubicInch, Day, FluidOunce, Foot, FootPound,
    FootSurvey, Gallon, Horsepower, Inch, JoulePerKilogram, LightYear, Metre, MileSurvey,
    MillimetreOfMercury, Ounce, Parsec, PlanckConstant, Pound, PoundForce, PoundForcePerSquareInch,
    Psi, Second, Slug, SpeedOfLight, SquareInch, StandardAtmosphere, StandardGravity, Torr,
};
use yadc_si::{Exact, ISQ, SIUnit, SIUnitConsts};

/// A unit's exact size, in coherent units of its quantity.
fn size<const D: ISQ, const C: SIUnitConsts>(_: SIUnit<D, C>) -> Exact {
    C.size
}

#[test]
fn inch_based() {
    assert_eq!(size(Inch {}) * size(Inch {}), size(SquareInch {}));
    assert_eq!(
        size(Inch {}) * size(Inch {}) * size(Inch {}),
        size(CubicInch {})
    );
    assert_eq!(size(Inch {}) * Exact::int(12), size(Foot {}));
    assert_eq!(
        size(Foot {}) * size(Foot {}) * size(Foot {}),
        size(CubicFoot {})
    );
    assert_eq!(size(CubicInch {}) * Exact::int(231), size(Gallon {}));
    assert_eq!(size(Gallon {}) / Exact::int(128), size(FluidOunce {}));
    assert_eq!(size(FootSurvey {}) * Exact::int(5280), size(MileSurvey {}));
}

#[test]
fn pound_based() {
    assert_eq!(size(Ounce {}) * Exact::int(16), size(Pound {}));
    assert_eq!(
        size(Pound {}) * size(StandardGravity {}),
        size(PoundForce {})
    );
    assert_eq!(
        size(PoundForce {}) * size(Second {}) * size(Second {}) / size(Foot {}),
        size(Slug {})
    );
    assert_eq!(
        size(Slug {}) / size(CubicFoot {}),
        size(SlugPerCubicFoot {})
    );
    assert_eq!(size(Foot {}) * size(PoundForce {}), size(FootPound {}));
    assert_eq!(
        size(Foot {}) * size(PoundForce {}) * Exact::int(550) / size(Second {}),
        size(Horsepower {})
    );
    assert_eq!(size(PoundForce {}) / size(SquareInch {}), size(Psi {}));
    assert_eq!(size(Psi {}), size(PoundForcePerSquareInch {}));
    assert_eq!(size(BtuIt {}) / size(Pound {}), size(BtuItPerPound {}));
}

#[test]
fn exact_values() {
    assert_eq!(
        size(BtuItPerPound {}) / size(JoulePerKilogram {}),
        Exact::int(2326)
    );
    assert_eq!(
        size(LightYear {}) / size(Metre {}),
        Exact::int(9_460_730_472_580_800)
    );
    assert_eq!(
        size(MillimetreOfMercury {}),
        Exact::dec(133_322_387_415, -9)
    );
}

#[test]
fn defined_by_other_units() {
    assert_eq!(size(StandardAtmosphere {}) / Exact::int(760), size(Torr {}));
    assert_eq!(
        size(Day {}) * Exact::dec(36525, -2) * size(SpeedOfLight {}),
        size(LightYear {})
    );
    assert_eq!(
        size(Parsec {}) * (Exact::PI / Exact::int(648_000)),
        size(AstronomicalUnit {})
    );
    assert_eq!(
        size(ElementaryCharge {}) * size(AvogadroConstant {}) * size(Mole {}),
        size(Faraday {})
    );
    assert_eq!(
        size(ReducedPlanckConstant {}) * (size(Radian {}) * (Exact::int(2) * Exact::PI)),
        size(PlanckConstant {})
    );
}

/// The unit types read and build with the size rounded once: an inch is
/// 0.0254 m, a light year 9460730472580800 m.
#[test]
fn unit_types_agree() {
    assert_eq!(Metre {}.value(Inch {}.quantity(1.0_f64)), 0.0254);
    assert!((Inch {}.value(Foot {}.quantity(1.0_f64)) - 12.0).abs() < 1e-12);
    assert_eq!(
        Metre {}.value(LightYear {}.quantity(1.0_f64)),
        9_460_730_472_580_800.0
    );
    assert!((Torr {}.value(StandardAtmosphere {}.quantity(1.0_f64)) - 760.0).abs() < 1e-9);
}
