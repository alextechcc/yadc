//! The units of `density`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::density::units::*;
use crate::density::*;
use crate::dynamic::DynamicSIUnit;
use crate::{Exact, SIQuantity, Unit};
use core::ops::{Add, Div, Mul, Sub};

/// The units of `MassDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<MASS_DENSITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<MASS_DENSITY, N>;
        fn value(&self, quantity: SIQuantity<MASS_DENSITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MassDensityUnit {
    #[default]
    #[value(KILOGRAM_PER_CUBIC_METRE)]
    KilogramPerCubicMetre,
    #[value(GRAM_PER_CUBIC_METRE)]
    GramPerCubicMetre,
    #[value(CARAT_PER_CUBIC_METRE)]
    CaratPerCubicMetre,
    #[value(GRAIN_PER_CUBIC_METRE)]
    GrainPerCubicMetre,
    #[value(HUNDREDWEIGHT_LONG_PER_CUBIC_METRE)]
    HundredweightLongPerCubicMetre,
    #[value(HUNDREDWEIGHT_SHORT_PER_CUBIC_METRE)]
    HundredweightShortPerCubicMetre,
    #[value(OUNCE_PER_CUBIC_METRE)]
    OuncePerCubicMetre,
    #[value(OUNCE_TROY_PER_CUBIC_METRE)]
    OunceTroyPerCubicMetre,
    #[value(PENNYWEIGHT_PER_CUBIC_METRE)]
    PennyweightPerCubicMetre,
    #[value(POUND_PER_CUBIC_METRE)]
    PoundPerCubicMetre,
    #[value(POUND_TROY_PER_CUBIC_METRE)]
    PoundTroyPerCubicMetre,
    #[value(SLUG_PER_CUBIC_METRE)]
    SlugPerCubicMetre,
    #[value(TON_ASSAY_PER_CUBIC_METRE)]
    TonAssayPerCubicMetre,
    #[value(TON_LONG_PER_CUBIC_METRE)]
    TonLongPerCubicMetre,
    #[value(TON_SHORT_PER_CUBIC_METRE)]
    TonShortPerCubicMetre,
    #[value(TON_PER_CUBIC_METRE)]
    TonPerCubicMetre,
    #[value(GRAIN_PER_GALLON)]
    GrainPerGallon,
    #[value(OUNCE_PER_CUBIC_INCH)]
    OuncePerCubicInch,
    #[value(OUNCE_PER_GALLON_IMPERIAL)]
    OuncePerGallonImperial,
    #[value(OUNCE_PER_GALLON)]
    OuncePerGallon,
    #[value(POUND_PER_CUBIC_FOOT)]
    PoundPerCubicFoot,
    #[value(POUND_PER_CUBIC_INCH)]
    PoundPerCubicInch,
    #[value(POUND_PER_CUBIC_YARD)]
    PoundPerCubicYard,
    #[value(POUND_PER_GALLON_IMPERIAL)]
    PoundPerGallonImperial,
    #[value(POUND_PER_GALLON)]
    PoundPerGallon,
    #[value(SLUG_PER_CUBIC_FOOT)]
    SlugPerCubicFoot,
    #[value(TON_LONG_PER_CUBIC_YARD)]
    TonLongPerCubicYard,
    #[value(TON_SHORT_PER_CUBIC_YARD)]
    TonShortPerCubicYard,
    #[value(TON_LONG_PER_CUBIC_FOOT)]
    TonLongPerCubicFoot,
    #[value(TON_SHORT_PER_CUBIC_FOOT)]
    TonShortPerCubicFoot,
}

/// The units of `LinearMassDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<LINEAR_MASS_DENSITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<LINEAR_MASS_DENSITY, N>;
        fn value(&self, quantity: SIQuantity<LINEAR_MASS_DENSITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum LinearMassDensityUnit {
    #[default]
    #[value(KILOGRAM_PER_METRE)]
    KilogramPerMetre,
    #[value(OUNCE_PER_FOOT)]
    OuncePerFoot,
    #[value(OUNCE_PER_INCH)]
    OuncePerInch,
    #[value(POUND_PER_YARD)]
    PoundPerYard,
    #[value(POUND_PER_FOOT)]
    PoundPerFoot,
    #[value(POUND_PER_INCH)]
    PoundPerInch,
    #[value(GRAM_PER_KILOMETRE)]
    GramPerKilometre,
    #[value(GRAM_PER_CENTIMETRE)]
    GramPerCentimetre,
}

/// The units of `ArealMassDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<AREAL_MASS_DENSITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<AREAL_MASS_DENSITY, N>;
        fn value(&self, quantity: SIQuantity<AREAL_MASS_DENSITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ArealMassDensityUnit {
    #[default]
    #[value(KILOGRAM_PER_SQUARE_METRE)]
    KilogramPerSquareMetre,
    #[value(GRAM_PER_SQUARE_METRE)]
    GramPerSquareMetre,
    #[value(OUNCE_PER_SQUARE_FOOT)]
    OuncePerSquareFoot,
}

/// The units of `LinearNumberDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<LINEAR_NUMBER_DENSITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<LINEAR_NUMBER_DENSITY, N>;
        fn value(&self, quantity: SIQuantity<LINEAR_NUMBER_DENSITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum LinearNumberDensityUnit {
    #[default]
    #[value(PER_METRE)]
    PerMetre,
    #[value(PER_FOOT)]
    PerFoot,
    #[value(PER_FOOT_SURVEY)]
    PerFootSurvey,
    #[value(PER_INCH)]
    PerInch,
    #[value(PER_MILE)]
    PerMile,
    #[value(PER_MILE_SURVEY)]
    PerMileSurvey,
    #[value(PER_NAUTICAL_MILE)]
    PerNauticalMile,
    #[value(PER_YARD)]
    PerYard,
}

/// The units of `ArealNumberDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<AREAL_NUMBER_DENSITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<AREAL_NUMBER_DENSITY, N>;
        fn value(&self, quantity: SIQuantity<AREAL_NUMBER_DENSITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ArealNumberDensityUnit {
    #[default]
    #[value(PER_SQUARE_METRE)]
    PerSquareMetre,
    #[value(PER_ACRE)]
    PerAcre,
    #[value(PER_ARE)]
    PerAre,
    #[value(PER_BARN)]
    PerBarn,
    #[value(PER_CIRCULAR_MIL)]
    PerCircularMil,
    #[value(PER_HECTARE)]
    PerHectare,
    #[value(PER_SQUARE_FOOT)]
    PerSquareFoot,
    #[value(PER_SQUARE_INCH)]
    PerSquareInch,
    #[value(PER_SQUARE_MILE)]
    PerSquareMile,
    #[value(PER_SQUARE_YARD)]
    PerSquareYard,
}

/// The units of `VolumetricNumberDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<VOLUMETRIC_NUMBER_DENSITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<VOLUMETRIC_NUMBER_DENSITY, N>;
        fn value(&self, quantity: SIQuantity<VOLUMETRIC_NUMBER_DENSITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum VolumetricNumberDensityUnit {
    #[default]
    #[value(PER_CUBIC_METRE)]
    PerCubicMetre,
    #[value(PER_CUBIC_FOOT)]
    PerCubicFoot,
    #[value(PER_CUBIC_INCH)]
    PerCubicInch,
    #[value(PER_CUBIC_MILE)]
    PerCubicMile,
    #[value(PER_CUBIC_YARD)]
    PerCubicYard,
    #[value(PER_FLUID_OUNCE)]
    PerFluidOunce,
    #[value(PER_FLUID_OUNCE_IMPERIAL)]
    PerFluidOunceImperial,
    #[value(PER_GALLON_IMPERIAL)]
    PerGallonImperial,
    #[value(PER_GALLON)]
    PerGallon,
    #[value(PER_LITRE)]
    PerLitre,
}

/// The units of `LinearNumberRate`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<LINEAR_NUMBER_RATE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<LINEAR_NUMBER_RATE, N>;
        fn value(&self, quantity: SIQuantity<LINEAR_NUMBER_RATE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum LinearNumberRateUnit {
    #[default]
    #[value(PER_METRE_SECOND)]
    PerMetreSecond,
    #[value(PER_FOOT_SECOND)]
    PerFootSecond,
    #[value(PER_FOOT_SURVEY_SECOND)]
    PerFootSurveySecond,
    #[value(PER_INCH_SECOND)]
    PerInchSecond,
    #[value(PER_MILE_SECOND)]
    PerMileSecond,
    #[value(PER_MILE_SURVEY_SECOND)]
    PerMileSurveySecond,
    #[value(PER_YARD_SECOND)]
    PerYardSecond,
}

/// The units of `ArealNumberRate`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<AREAL_NUMBER_RATE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<AREAL_NUMBER_RATE, N>;
        fn value(&self, quantity: SIQuantity<AREAL_NUMBER_RATE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ArealNumberRateUnit {
    #[default]
    #[value(PER_SQUARE_METRE_SECOND)]
    PerSquareMetreSecond,
    #[value(PER_ACRE_SECOND)]
    PerAcreSecond,
    #[value(PER_ARE_SECOND)]
    PerAreSecond,
    #[value(PER_BARN_SECOND)]
    PerBarnSecond,
    #[value(PER_CIRCULAR_MIL_SECOND)]
    PerCircularMilSecond,
    #[value(PER_HECTARE_SECOND)]
    PerHectareSecond,
    #[value(PER_SQUARE_FOOT_SECOND)]
    PerSquareFootSecond,
    #[value(PER_SQUARE_INCH_SECOND)]
    PerSquareInchSecond,
    #[value(PER_SQUARE_MILE_SECOND)]
    PerSquareMileSecond,
    #[value(PER_SQUARE_YARD_SECOND)]
    PerSquareYardSecond,
}

/// The units of `VolumetricNumberRate`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<VOLUMETRIC_NUMBER_RATE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<VOLUMETRIC_NUMBER_RATE, N>;
        fn value(&self, quantity: SIQuantity<VOLUMETRIC_NUMBER_RATE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum VolumetricNumberRateUnit {
    #[default]
    #[value(PER_CUBIC_METRE_SECOND)]
    PerCubicMetreSecond,
    #[value(PER_CUBIC_FOOT_SECOND)]
    PerCubicFootSecond,
    #[value(PER_CUBIC_INCH_SECOND)]
    PerCubicInchSecond,
    #[value(PER_CUBIC_MILE_SECOND)]
    PerCubicMileSecond,
    #[value(PER_CUBIC_YARD_SECOND)]
    PerCubicYardSecond,
    #[value(PER_FLUID_OUNCE_SECOND)]
    PerFluidOunceSecond,
    #[value(PER_FLUID_OUNCE_IMPERIAL_SECOND)]
    PerFluidOunceImperialSecond,
    #[value(PER_GALLON_IMPERIAL_SECOND)]
    PerGallonImperialSecond,
    #[value(PER_GALLON_SECOND)]
    PerGallonSecond,
    #[value(PER_LITRE_SECOND)]
    PerLitreSecond,
    #[value(BECQUEREL_PER_CUBIC_METRE)]
    BecquerelPerCubicMetre,
    #[value(CURIE_PER_CUBIC_METRE)]
    CuriePerCubicMetre,
    #[value(DISINTEGRATIONS_PER_MINUTE_PER_CUBIC_METRE)]
    DisintegrationsPerMinutePerCubicMetre,
}

/// The units of `LinearDensityOfStates`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<LINEAR_DENSITY_OF_STATES, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<LINEAR_DENSITY_OF_STATES, N>;
        fn value(&self, quantity: SIQuantity<LINEAR_DENSITY_OF_STATES, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum LinearDensityOfStatesUnit {
    #[default]
    #[value(STATE_PER_METRE_JOULE)]
    StatePerMetreJoule,
}

/// The units of `ArealDensityOfStates`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<AREAL_DENSITY_OF_STATES, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<AREAL_DENSITY_OF_STATES, N>;
        fn value(&self, quantity: SIQuantity<AREAL_DENSITY_OF_STATES, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ArealDensityOfStatesUnit {
    #[default]
    #[value(STATE_PER_SQUARE_METRE_JOULE)]
    StatePerSquareMetreJoule,
}

/// The units of `VolumetricDensityOfStates`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<VOLUMETRIC_DENSITY_OF_STATES, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<VOLUMETRIC_DENSITY_OF_STATES, N>;
        fn value(&self, quantity: SIQuantity<VOLUMETRIC_DENSITY_OF_STATES, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum VolumetricDensityOfStatesUnit {
    #[default]
    #[value(STATE_PER_CUBIC_METRE_JOULE)]
    StatePerCubicMetreJoule,
}

/// The units of `LinearPowerDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<LINEAR_POWER_DENSITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<LINEAR_POWER_DENSITY, N>;
        fn value(&self, quantity: SIQuantity<LINEAR_POWER_DENSITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum LinearPowerDensityUnit {
    #[default]
    #[value(WATT_PER_METRE)]
    WattPerMetre,
}

/// The units of `VolumetricPowerDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<VOLUMETRIC_POWER_DENSITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<VOLUMETRIC_POWER_DENSITY, N>;
        fn value(&self, quantity: SIQuantity<VOLUMETRIC_POWER_DENSITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum VolumetricPowerDensityUnit {
    #[default]
    #[value(WATT_PER_CUBIC_METRE)]
    WattPerCubicMetre,
}
