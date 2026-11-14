//! The units of `density`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

#[allow(unused_imports)]
use crate::density::*;
#[allow(unused_imports)]
use crate::*;

/// The units of `MassDensity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(into(SIUnit))]
pub enum MassDensityUnit {
    #[default]
    #[value(KILOGRAM_PER_CUBIC_METRE.unit())]
    KilogramPerCubicMetre,
    #[value(GRAM_PER_CUBIC_METRE.unit())]
    GramPerCubicMetre,
    #[value(CARAT_PER_CUBIC_METRE.unit())]
    CaratPerCubicMetre,
    #[value(GRAIN_PER_CUBIC_METRE.unit())]
    GrainPerCubicMetre,
    #[value(HUNDREDWEIGHT_LONG_PER_CUBIC_METRE.unit())]
    HundredweightLongPerCubicMetre,
    #[value(HUNDREDWEIGHT_SHORT_PER_CUBIC_METRE.unit())]
    HundredweightShortPerCubicMetre,
    #[value(OUNCE_PER_CUBIC_METRE.unit())]
    OuncePerCubicMetre,
    #[value(OUNCE_TROY_PER_CUBIC_METRE.unit())]
    OunceTroyPerCubicMetre,
    #[value(PENNYWEIGHT_PER_CUBIC_METRE.unit())]
    PennyweightPerCubicMetre,
    #[value(POUND_PER_CUBIC_METRE.unit())]
    PoundPerCubicMetre,
    #[value(POUND_TROY_PER_CUBIC_METRE.unit())]
    PoundTroyPerCubicMetre,
    #[value(SLUG_PER_CUBIC_METRE.unit())]
    SlugPerCubicMetre,
    #[value(TON_ASSAY_PER_CUBIC_METRE.unit())]
    TonAssayPerCubicMetre,
    #[value(TON_LONG_PER_CUBIC_METRE.unit())]
    TonLongPerCubicMetre,
    #[value(TON_SHORT_PER_CUBIC_METRE.unit())]
    TonShortPerCubicMetre,
    #[value(TON_PER_CUBIC_METRE.unit())]
    TonPerCubicMetre,
    #[value(GRAIN_PER_GALLON.unit())]
    GrainPerGallon,
    #[value(OUNCE_PER_CUBIC_INCH.unit())]
    OuncePerCubicInch,
    #[value(OUNCE_PER_GALLON_IMPERIAL.unit())]
    OuncePerGallonImperial,
    #[value(OUNCE_PER_GALLON.unit())]
    OuncePerGallon,
    #[value(POUND_PER_CUBIC_FOOT.unit())]
    PoundPerCubicFoot,
    #[value(POUND_PER_CUBIC_INCH.unit())]
    PoundPerCubicInch,
    #[value(POUND_PER_CUBIC_YARD.unit())]
    PoundPerCubicYard,
    #[value(POUND_PER_GALLON_IMPERIAL.unit())]
    PoundPerGallonImperial,
    #[value(POUND_PER_GALLON.unit())]
    PoundPerGallon,
    #[value(SLUG_PER_CUBIC_FOOT.unit())]
    SlugPerCubicFoot,
    #[value(TON_LONG_PER_CUBIC_YARD.unit())]
    TonLongPerCubicYard,
    #[value(TON_SHORT_PER_CUBIC_YARD.unit())]
    TonShortPerCubicYard,
    #[value(TON_LONG_PER_CUBIC_FOOT.unit())]
    TonLongPerCubicFoot,
    #[value(TON_SHORT_PER_CUBIC_FOOT.unit())]
    TonShortPerCubicFoot,
}

/// The units of `LinearMassDensity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(into(SIUnit))]
pub enum LinearMassDensityUnit {
    #[default]
    #[value(KILOGRAM_PER_METRE.unit())]
    KilogramPerMetre,
    #[value(OUNCE_PER_FOOT.unit())]
    OuncePerFoot,
    #[value(OUNCE_PER_INCH.unit())]
    OuncePerInch,
    #[value(POUND_PER_YARD.unit())]
    PoundPerYard,
    #[value(POUND_PER_FOOT.unit())]
    PoundPerFoot,
    #[value(POUND_PER_INCH.unit())]
    PoundPerInch,
    #[value(GRAM_PER_KILOMETRE.unit())]
    GramPerKilometre,
    #[value(GRAM_PER_CENTIMETRE.unit())]
    GramPerCentimetre,
}

/// The units of `ArealMassDensity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(into(SIUnit))]
pub enum ArealMassDensityUnit {
    #[default]
    #[value(KILOGRAM_PER_SQUARE_METRE.unit())]
    KilogramPerSquareMetre,
    #[value(GRAM_PER_SQUARE_METRE.unit())]
    GramPerSquareMetre,
    #[value(OUNCE_PER_SQUARE_FOOT.unit())]
    OuncePerSquareFoot,
}

/// The units of `LinearNumberDensity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(into(SIUnit))]
pub enum LinearNumberDensityUnit {
    #[default]
    #[value(PER_METRE.unit())]
    PerMetre,
    #[value(PER_FOOT.unit())]
    PerFoot,
    #[value(PER_FOOT_SURVEY.unit())]
    PerFootSurvey,
    #[value(PER_INCH.unit())]
    PerInch,
    #[value(PER_MILE.unit())]
    PerMile,
    #[value(PER_MILE_SURVEY.unit())]
    PerMileSurvey,
    #[value(PER_NAUTICAL_MILE.unit())]
    PerNauticalMile,
    #[value(PER_YARD.unit())]
    PerYard,
}

/// The units of `ArealNumberDensity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(into(SIUnit))]
pub enum ArealNumberDensityUnit {
    #[default]
    #[value(PER_SQUARE_METRE.unit())]
    PerSquareMetre,
    #[value(PER_ACRE.unit())]
    PerAcre,
    #[value(PER_ARE.unit())]
    PerAre,
    #[value(PER_BARN.unit())]
    PerBarn,
    #[value(PER_CIRCULAR_MIL.unit())]
    PerCircularMil,
    #[value(PER_HECTARE.unit())]
    PerHectare,
    #[value(PER_SQUARE_FOOT.unit())]
    PerSquareFoot,
    #[value(PER_SQUARE_INCH.unit())]
    PerSquareInch,
    #[value(PER_SQUARE_MILE.unit())]
    PerSquareMile,
    #[value(PER_SQUARE_YARD.unit())]
    PerSquareYard,
}

/// The units of `VolumetricNumberDensity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(into(SIUnit))]
pub enum VolumetricNumberDensityUnit {
    #[default]
    #[value(PER_CUBIC_METRE.unit())]
    PerCubicMetre,
    #[value(PER_CUBIC_FOOT.unit())]
    PerCubicFoot,
    #[value(PER_CUBIC_INCH.unit())]
    PerCubicInch,
    #[value(PER_CUBIC_MILE.unit())]
    PerCubicMile,
    #[value(PER_CUBIC_YARD.unit())]
    PerCubicYard,
    #[value(PER_FLUID_OUNCE.unit())]
    PerFluidOunce,
    #[value(PER_FLUID_OUNCE_IMPERIAL.unit())]
    PerFluidOunceImperial,
    #[value(PER_GALLON_IMPERIAL.unit())]
    PerGallonImperial,
    #[value(PER_GALLON.unit())]
    PerGallon,
    #[value(PER_LITRE.unit())]
    PerLitre,
}

/// The units of `LinearNumberRate`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(into(SIUnit))]
pub enum LinearNumberRateUnit {
    #[default]
    #[value(PER_METRE_SECOND.unit())]
    PerMetreSecond,
    #[value(PER_FOOT_SECOND.unit())]
    PerFootSecond,
    #[value(PER_FOOT_SURVEY_SECOND.unit())]
    PerFootSurveySecond,
    #[value(PER_INCH_SECOND.unit())]
    PerInchSecond,
    #[value(PER_MILE_SECOND.unit())]
    PerMileSecond,
    #[value(PER_MILE_SURVEY_SECOND.unit())]
    PerMileSurveySecond,
    #[value(PER_YARD_SECOND.unit())]
    PerYardSecond,
}

/// The units of `ArealNumberRate`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(into(SIUnit))]
pub enum ArealNumberRateUnit {
    #[default]
    #[value(PER_SQUARE_METRE_SECOND.unit())]
    PerSquareMetreSecond,
    #[value(PER_ACRE_SECOND.unit())]
    PerAcreSecond,
    #[value(PER_ARE_SECOND.unit())]
    PerAreSecond,
    #[value(PER_BARN_SECOND.unit())]
    PerBarnSecond,
    #[value(PER_CIRCULAR_MIL_SECOND.unit())]
    PerCircularMilSecond,
    #[value(PER_HECTARE_SECOND.unit())]
    PerHectareSecond,
    #[value(PER_SQUARE_FOOT_SECOND.unit())]
    PerSquareFootSecond,
    #[value(PER_SQUARE_INCH_SECOND.unit())]
    PerSquareInchSecond,
    #[value(PER_SQUARE_MILE_SECOND.unit())]
    PerSquareMileSecond,
    #[value(PER_SQUARE_YARD_SECOND.unit())]
    PerSquareYardSecond,
}

/// The units of `VolumetricNumberRate`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(into(SIUnit))]
pub enum VolumetricNumberRateUnit {
    #[default]
    #[value(PER_CUBIC_METRE_SECOND.unit())]
    PerCubicMetreSecond,
    #[value(PER_CUBIC_FOOT_SECOND.unit())]
    PerCubicFootSecond,
    #[value(PER_CUBIC_INCH_SECOND.unit())]
    PerCubicInchSecond,
    #[value(PER_CUBIC_MILE_SECOND.unit())]
    PerCubicMileSecond,
    #[value(PER_CUBIC_YARD_SECOND.unit())]
    PerCubicYardSecond,
    #[value(PER_FLUID_OUNCE_SECOND.unit())]
    PerFluidOunceSecond,
    #[value(PER_FLUID_OUNCE_IMPERIAL_SECOND.unit())]
    PerFluidOunceImperialSecond,
    #[value(PER_GALLON_IMPERIAL_SECOND.unit())]
    PerGallonImperialSecond,
    #[value(PER_GALLON_SECOND.unit())]
    PerGallonSecond,
    #[value(PER_LITRE_SECOND.unit())]
    PerLitreSecond,
    #[value(BECQUEREL_PER_CUBIC_METRE.unit())]
    BecquerelPerCubicMetre,
    #[value(CURIE_PER_CUBIC_METRE.unit())]
    CuriePerCubicMetre,
    #[value(DISINTEGRATIONS_PER_MINUTE_PER_CUBIC_METRE.unit())]
    DisintegrationsPerMinutePerCubicMetre,
}

/// The units of `LinearDensityOfStates`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(into(SIUnit))]
pub enum LinearDensityOfStatesUnit {
    #[default]
    #[value(STATE_PER_METRE_JOULE.unit())]
    StatePerMetreJoule,
}

/// The units of `ArealDensityOfStates`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(into(SIUnit))]
pub enum ArealDensityOfStatesUnit {
    #[default]
    #[value(STATE_PER_SQUARE_METRE_JOULE.unit())]
    StatePerSquareMetreJoule,
}

/// The units of `VolumetricDensityOfStates`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(into(SIUnit))]
pub enum VolumetricDensityOfStatesUnit {
    #[default]
    #[value(STATE_PER_CUBIC_METRE_JOULE.unit())]
    StatePerCubicMetreJoule,
}

/// The units of `LinearPowerDensity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(into(SIUnit))]
pub enum LinearPowerDensityUnit {
    #[default]
    #[value(WATT_PER_METRE.unit())]
    WattPerMetre,
}

/// The units of `VolumetricPowerDensity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(into(SIUnit))]
pub enum VolumetricPowerDensityUnit {
    #[default]
    #[value(WATT_PER_CUBIC_METRE.unit())]
    WattPerCubicMetre,
}
