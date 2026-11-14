//! The units of `density`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::density::units::*;
use crate::density::*;
use crate::dynamic::DynamicSIUnit;
use core::marker::Destruct;
use core::ops::{Add, Div, Mul, Sub};

/// The units of `MassDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassDensity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MassDensityUnit {
    #[default]
    #[unit(KilogramPerCubicMetre)]
    KilogramPerCubicMetre,
    #[unit(GramPerCubicMetre)]
    GramPerCubicMetre,
    #[unit(CaratPerCubicMetre)]
    CaratPerCubicMetre,
    #[unit(GrainPerCubicMetre)]
    GrainPerCubicMetre,
    #[unit(HundredweightLongPerCubicMetre)]
    HundredweightLongPerCubicMetre,
    #[unit(HundredweightShortPerCubicMetre)]
    HundredweightShortPerCubicMetre,
    #[unit(OuncePerCubicMetre)]
    OuncePerCubicMetre,
    #[unit(OunceTroyPerCubicMetre)]
    OunceTroyPerCubicMetre,
    #[unit(PennyweightPerCubicMetre)]
    PennyweightPerCubicMetre,
    #[unit(PoundPerCubicMetre)]
    PoundPerCubicMetre,
    #[unit(PoundTroyPerCubicMetre)]
    PoundTroyPerCubicMetre,
    #[unit(SlugPerCubicMetre)]
    SlugPerCubicMetre,
    #[unit(TonAssayPerCubicMetre)]
    TonAssayPerCubicMetre,
    #[unit(TonLongPerCubicMetre)]
    TonLongPerCubicMetre,
    #[unit(TonShortPerCubicMetre)]
    TonShortPerCubicMetre,
    #[unit(TonPerCubicMetre)]
    TonPerCubicMetre,
    #[unit(GrainPerGallon)]
    GrainPerGallon,
    #[unit(OuncePerCubicInch)]
    OuncePerCubicInch,
    #[unit(OuncePerGallonImperial)]
    OuncePerGallonImperial,
    #[unit(OuncePerGallon)]
    OuncePerGallon,
    #[unit(PoundPerCubicFoot)]
    PoundPerCubicFoot,
    #[unit(PoundPerCubicInch)]
    PoundPerCubicInch,
    #[unit(PoundPerCubicYard)]
    PoundPerCubicYard,
    #[unit(PoundPerGallonImperial)]
    PoundPerGallonImperial,
    #[unit(PoundPerGallon)]
    PoundPerGallon,
    #[unit(SlugPerCubicFoot)]
    SlugPerCubicFoot,
    #[unit(TonLongPerCubicYard)]
    TonLongPerCubicYard,
    #[unit(TonShortPerCubicYard)]
    TonShortPerCubicYard,
    #[unit(TonLongPerCubicFoot)]
    TonLongPerCubicFoot,
    #[unit(TonShortPerCubicFoot)]
    TonShortPerCubicFoot,
}

impl From<MassDensityUnit> for DynamicSIUnit {
    fn from(unit: MassDensityUnit) -> Self {
        match unit {
            MassDensityUnit::KilogramPerCubicMetre => KilogramPerCubicMetre {}.into(),
            MassDensityUnit::GramPerCubicMetre => GramPerCubicMetre {}.into(),
            MassDensityUnit::CaratPerCubicMetre => CaratPerCubicMetre {}.into(),
            MassDensityUnit::GrainPerCubicMetre => GrainPerCubicMetre {}.into(),
            MassDensityUnit::HundredweightLongPerCubicMetre => {
                HundredweightLongPerCubicMetre {}.into()
            }
            MassDensityUnit::HundredweightShortPerCubicMetre => {
                HundredweightShortPerCubicMetre {}.into()
            }
            MassDensityUnit::OuncePerCubicMetre => OuncePerCubicMetre {}.into(),
            MassDensityUnit::OunceTroyPerCubicMetre => OunceTroyPerCubicMetre {}.into(),
            MassDensityUnit::PennyweightPerCubicMetre => PennyweightPerCubicMetre {}.into(),
            MassDensityUnit::PoundPerCubicMetre => PoundPerCubicMetre {}.into(),
            MassDensityUnit::PoundTroyPerCubicMetre => PoundTroyPerCubicMetre {}.into(),
            MassDensityUnit::SlugPerCubicMetre => SlugPerCubicMetre {}.into(),
            MassDensityUnit::TonAssayPerCubicMetre => TonAssayPerCubicMetre {}.into(),
            MassDensityUnit::TonLongPerCubicMetre => TonLongPerCubicMetre {}.into(),
            MassDensityUnit::TonShortPerCubicMetre => TonShortPerCubicMetre {}.into(),
            MassDensityUnit::TonPerCubicMetre => TonPerCubicMetre {}.into(),
            MassDensityUnit::GrainPerGallon => GrainPerGallon {}.into(),
            MassDensityUnit::OuncePerCubicInch => OuncePerCubicInch {}.into(),
            MassDensityUnit::OuncePerGallonImperial => OuncePerGallonImperial {}.into(),
            MassDensityUnit::OuncePerGallon => OuncePerGallon {}.into(),
            MassDensityUnit::PoundPerCubicFoot => PoundPerCubicFoot {}.into(),
            MassDensityUnit::PoundPerCubicInch => PoundPerCubicInch {}.into(),
            MassDensityUnit::PoundPerCubicYard => PoundPerCubicYard {}.into(),
            MassDensityUnit::PoundPerGallonImperial => PoundPerGallonImperial {}.into(),
            MassDensityUnit::PoundPerGallon => PoundPerGallon {}.into(),
            MassDensityUnit::SlugPerCubicFoot => SlugPerCubicFoot {}.into(),
            MassDensityUnit::TonLongPerCubicYard => TonLongPerCubicYard {}.into(),
            MassDensityUnit::TonShortPerCubicYard => TonShortPerCubicYard {}.into(),
            MassDensityUnit::TonLongPerCubicFoot => TonLongPerCubicFoot {}.into(),
            MassDensityUnit::TonShortPerCubicFoot => TonShortPerCubicFoot {}.into(),
        }
    }
}

/// The units of `LinearMassDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LinearMassDensity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum LinearMassDensityUnit {
    #[default]
    #[unit(KilogramPerMetre)]
    KilogramPerMetre,
    #[unit(OuncePerFoot)]
    OuncePerFoot,
    #[unit(OuncePerInch)]
    OuncePerInch,
    #[unit(PoundPerYard)]
    PoundPerYard,
    #[unit(PoundPerFoot)]
    PoundPerFoot,
    #[unit(PoundPerInch)]
    PoundPerInch,
    #[unit(GramPerKilometre)]
    GramPerKilometre,
    #[unit(GramPerCentimetre)]
    GramPerCentimetre,
}

impl From<LinearMassDensityUnit> for DynamicSIUnit {
    fn from(unit: LinearMassDensityUnit) -> Self {
        match unit {
            LinearMassDensityUnit::KilogramPerMetre => KilogramPerMetre {}.into(),
            LinearMassDensityUnit::OuncePerFoot => OuncePerFoot {}.into(),
            LinearMassDensityUnit::OuncePerInch => OuncePerInch {}.into(),
            LinearMassDensityUnit::PoundPerYard => PoundPerYard {}.into(),
            LinearMassDensityUnit::PoundPerFoot => PoundPerFoot {}.into(),
            LinearMassDensityUnit::PoundPerInch => PoundPerInch {}.into(),
            LinearMassDensityUnit::GramPerKilometre => GramPerKilometre {}.into(),
            LinearMassDensityUnit::GramPerCentimetre => GramPerCentimetre {}.into(),
        }
    }
}

/// The units of `ArealMassDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ArealMassDensity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ArealMassDensityUnit {
    #[default]
    #[unit(KilogramPerSquareMetre)]
    KilogramPerSquareMetre,
    #[unit(GramPerSquareMetre)]
    GramPerSquareMetre,
    #[unit(OuncePerSquareFoot)]
    OuncePerSquareFoot,
}

impl From<ArealMassDensityUnit> for DynamicSIUnit {
    fn from(unit: ArealMassDensityUnit) -> Self {
        match unit {
            ArealMassDensityUnit::KilogramPerSquareMetre => KilogramPerSquareMetre {}.into(),
            ArealMassDensityUnit::GramPerSquareMetre => GramPerSquareMetre {}.into(),
            ArealMassDensityUnit::OuncePerSquareFoot => OuncePerSquareFoot {}.into(),
        }
    }
}

/// The units of `LinearNumberDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LinearNumberDensity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum LinearNumberDensityUnit {
    #[default]
    #[unit(PerMetre)]
    PerMetre,
    #[unit(PerFoot)]
    PerFoot,
    #[unit(PerFootSurvey)]
    PerFootSurvey,
    #[unit(PerInch)]
    PerInch,
    #[unit(PerMile)]
    PerMile,
    #[unit(PerMileSurvey)]
    PerMileSurvey,
    #[unit(PerNauticalMile)]
    PerNauticalMile,
    #[unit(PerYard)]
    PerYard,
}

impl From<LinearNumberDensityUnit> for DynamicSIUnit {
    fn from(unit: LinearNumberDensityUnit) -> Self {
        match unit {
            LinearNumberDensityUnit::PerMetre => PerMetre {}.into(),
            LinearNumberDensityUnit::PerFoot => PerFoot {}.into(),
            LinearNumberDensityUnit::PerFootSurvey => PerFootSurvey {}.into(),
            LinearNumberDensityUnit::PerInch => PerInch {}.into(),
            LinearNumberDensityUnit::PerMile => PerMile {}.into(),
            LinearNumberDensityUnit::PerMileSurvey => PerMileSurvey {}.into(),
            LinearNumberDensityUnit::PerNauticalMile => PerNauticalMile {}.into(),
            LinearNumberDensityUnit::PerYard => PerYard {}.into(),
        }
    }
}

/// The units of `ArealNumberDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ArealNumberDensity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ArealNumberDensityUnit {
    #[default]
    #[unit(PerSquareMetre)]
    PerSquareMetre,
    #[unit(PerAcre)]
    PerAcre,
    #[unit(PerAre)]
    PerAre,
    #[unit(PerBarn)]
    PerBarn,
    #[unit(PerCircularMil)]
    PerCircularMil,
    #[unit(PerHectare)]
    PerHectare,
    #[unit(PerSquareFoot)]
    PerSquareFoot,
    #[unit(PerSquareInch)]
    PerSquareInch,
    #[unit(PerSquareMile)]
    PerSquareMile,
    #[unit(PerSquareYard)]
    PerSquareYard,
}

impl From<ArealNumberDensityUnit> for DynamicSIUnit {
    fn from(unit: ArealNumberDensityUnit) -> Self {
        match unit {
            ArealNumberDensityUnit::PerSquareMetre => PerSquareMetre {}.into(),
            ArealNumberDensityUnit::PerAcre => PerAcre {}.into(),
            ArealNumberDensityUnit::PerAre => PerAre {}.into(),
            ArealNumberDensityUnit::PerBarn => PerBarn {}.into(),
            ArealNumberDensityUnit::PerCircularMil => PerCircularMil {}.into(),
            ArealNumberDensityUnit::PerHectare => PerHectare {}.into(),
            ArealNumberDensityUnit::PerSquareFoot => PerSquareFoot {}.into(),
            ArealNumberDensityUnit::PerSquareInch => PerSquareInch {}.into(),
            ArealNumberDensityUnit::PerSquareMile => PerSquareMile {}.into(),
            ArealNumberDensityUnit::PerSquareYard => PerSquareYard {}.into(),
        }
    }
}

/// The units of `VolumetricNumberDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = VolumetricNumberDensity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum VolumetricNumberDensityUnit {
    #[default]
    #[unit(PerCubicMetre)]
    PerCubicMetre,
    #[unit(PerCubicFoot)]
    PerCubicFoot,
    #[unit(PerCubicInch)]
    PerCubicInch,
    #[unit(PerCubicMile)]
    PerCubicMile,
    #[unit(PerCubicYard)]
    PerCubicYard,
    #[unit(PerFluidOunce)]
    PerFluidOunce,
    #[unit(PerFluidOunceImperial)]
    PerFluidOunceImperial,
    #[unit(PerGallonImperial)]
    PerGallonImperial,
    #[unit(PerGallon)]
    PerGallon,
    #[unit(PerLitre)]
    PerLitre,
}

impl From<VolumetricNumberDensityUnit> for DynamicSIUnit {
    fn from(unit: VolumetricNumberDensityUnit) -> Self {
        match unit {
            VolumetricNumberDensityUnit::PerCubicMetre => PerCubicMetre {}.into(),
            VolumetricNumberDensityUnit::PerCubicFoot => PerCubicFoot {}.into(),
            VolumetricNumberDensityUnit::PerCubicInch => PerCubicInch {}.into(),
            VolumetricNumberDensityUnit::PerCubicMile => PerCubicMile {}.into(),
            VolumetricNumberDensityUnit::PerCubicYard => PerCubicYard {}.into(),
            VolumetricNumberDensityUnit::PerFluidOunce => PerFluidOunce {}.into(),
            VolumetricNumberDensityUnit::PerFluidOunceImperial => PerFluidOunceImperial {}.into(),
            VolumetricNumberDensityUnit::PerGallonImperial => PerGallonImperial {}.into(),
            VolumetricNumberDensityUnit::PerGallon => PerGallon {}.into(),
            VolumetricNumberDensityUnit::PerLitre => PerLitre {}.into(),
        }
    }
}

/// The units of `LinearNumberRate`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LinearNumberRate<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum LinearNumberRateUnit {
    #[default]
    #[unit(PerMetreSecond)]
    PerMetreSecond,
    #[unit(PerFootSecond)]
    PerFootSecond,
    #[unit(PerFootSurveySecond)]
    PerFootSurveySecond,
    #[unit(PerInchSecond)]
    PerInchSecond,
    #[unit(PerMileSecond)]
    PerMileSecond,
    #[unit(PerMileSurveySecond)]
    PerMileSurveySecond,
    #[unit(PerYardSecond)]
    PerYardSecond,
}

impl From<LinearNumberRateUnit> for DynamicSIUnit {
    fn from(unit: LinearNumberRateUnit) -> Self {
        match unit {
            LinearNumberRateUnit::PerMetreSecond => PerMetreSecond {}.into(),
            LinearNumberRateUnit::PerFootSecond => PerFootSecond {}.into(),
            LinearNumberRateUnit::PerFootSurveySecond => PerFootSurveySecond {}.into(),
            LinearNumberRateUnit::PerInchSecond => PerInchSecond {}.into(),
            LinearNumberRateUnit::PerMileSecond => PerMileSecond {}.into(),
            LinearNumberRateUnit::PerMileSurveySecond => PerMileSurveySecond {}.into(),
            LinearNumberRateUnit::PerYardSecond => PerYardSecond {}.into(),
        }
    }
}

/// The units of `ArealNumberRate`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ArealNumberRate<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ArealNumberRateUnit {
    #[default]
    #[unit(PerSquareMetreSecond)]
    PerSquareMetreSecond,
    #[unit(PerAcreSecond)]
    PerAcreSecond,
    #[unit(PerAreSecond)]
    PerAreSecond,
    #[unit(PerBarnSecond)]
    PerBarnSecond,
    #[unit(PerCircularMilSecond)]
    PerCircularMilSecond,
    #[unit(PerHectareSecond)]
    PerHectareSecond,
    #[unit(PerSquareFootSecond)]
    PerSquareFootSecond,
    #[unit(PerSquareInchSecond)]
    PerSquareInchSecond,
    #[unit(PerSquareMileSecond)]
    PerSquareMileSecond,
    #[unit(PerSquareYardSecond)]
    PerSquareYardSecond,
}

impl From<ArealNumberRateUnit> for DynamicSIUnit {
    fn from(unit: ArealNumberRateUnit) -> Self {
        match unit {
            ArealNumberRateUnit::PerSquareMetreSecond => PerSquareMetreSecond {}.into(),
            ArealNumberRateUnit::PerAcreSecond => PerAcreSecond {}.into(),
            ArealNumberRateUnit::PerAreSecond => PerAreSecond {}.into(),
            ArealNumberRateUnit::PerBarnSecond => PerBarnSecond {}.into(),
            ArealNumberRateUnit::PerCircularMilSecond => PerCircularMilSecond {}.into(),
            ArealNumberRateUnit::PerHectareSecond => PerHectareSecond {}.into(),
            ArealNumberRateUnit::PerSquareFootSecond => PerSquareFootSecond {}.into(),
            ArealNumberRateUnit::PerSquareInchSecond => PerSquareInchSecond {}.into(),
            ArealNumberRateUnit::PerSquareMileSecond => PerSquareMileSecond {}.into(),
            ArealNumberRateUnit::PerSquareYardSecond => PerSquareYardSecond {}.into(),
        }
    }
}

/// The units of `VolumetricNumberRate`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = VolumetricNumberRate<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum VolumetricNumberRateUnit {
    #[default]
    #[unit(PerCubicMetreSecond)]
    PerCubicMetreSecond,
    #[unit(PerCubicFootSecond)]
    PerCubicFootSecond,
    #[unit(PerCubicInchSecond)]
    PerCubicInchSecond,
    #[unit(PerCubicMileSecond)]
    PerCubicMileSecond,
    #[unit(PerCubicYardSecond)]
    PerCubicYardSecond,
    #[unit(PerFluidOunceSecond)]
    PerFluidOunceSecond,
    #[unit(PerFluidOunceImperialSecond)]
    PerFluidOunceImperialSecond,
    #[unit(PerGallonImperialSecond)]
    PerGallonImperialSecond,
    #[unit(PerGallonSecond)]
    PerGallonSecond,
    #[unit(PerLitreSecond)]
    PerLitreSecond,
    #[unit(BecquerelPerCubicMetre)]
    BecquerelPerCubicMetre,
    #[unit(CuriePerCubicMetre)]
    CuriePerCubicMetre,
    #[unit(DisintegrationsPerMinutePerCubicMetre)]
    DisintegrationsPerMinutePerCubicMetre,
}

impl From<VolumetricNumberRateUnit> for DynamicSIUnit {
    fn from(unit: VolumetricNumberRateUnit) -> Self {
        match unit {
            VolumetricNumberRateUnit::PerCubicMetreSecond => PerCubicMetreSecond {}.into(),
            VolumetricNumberRateUnit::PerCubicFootSecond => PerCubicFootSecond {}.into(),
            VolumetricNumberRateUnit::PerCubicInchSecond => PerCubicInchSecond {}.into(),
            VolumetricNumberRateUnit::PerCubicMileSecond => PerCubicMileSecond {}.into(),
            VolumetricNumberRateUnit::PerCubicYardSecond => PerCubicYardSecond {}.into(),
            VolumetricNumberRateUnit::PerFluidOunceSecond => PerFluidOunceSecond {}.into(),
            VolumetricNumberRateUnit::PerFluidOunceImperialSecond => {
                PerFluidOunceImperialSecond {}.into()
            }
            VolumetricNumberRateUnit::PerGallonImperialSecond => PerGallonImperialSecond {}.into(),
            VolumetricNumberRateUnit::PerGallonSecond => PerGallonSecond {}.into(),
            VolumetricNumberRateUnit::PerLitreSecond => PerLitreSecond {}.into(),
            VolumetricNumberRateUnit::BecquerelPerCubicMetre => BecquerelPerCubicMetre {}.into(),
            VolumetricNumberRateUnit::CuriePerCubicMetre => CuriePerCubicMetre {}.into(),
            VolumetricNumberRateUnit::DisintegrationsPerMinutePerCubicMetre => {
                DisintegrationsPerMinutePerCubicMetre {}.into()
            }
        }
    }
}

/// The units of `LinearDensityOfStates`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LinearDensityOfStates<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum LinearDensityOfStatesUnit {
    #[default]
    #[unit(StatePerMetreJoule)]
    StatePerMetreJoule,
}

impl From<LinearDensityOfStatesUnit> for DynamicSIUnit {
    fn from(unit: LinearDensityOfStatesUnit) -> Self {
        match unit {
            LinearDensityOfStatesUnit::StatePerMetreJoule => StatePerMetreJoule {}.into(),
        }
    }
}

/// The units of `ArealDensityOfStates`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ArealDensityOfStates<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ArealDensityOfStatesUnit {
    #[default]
    #[unit(StatePerSquareMetreJoule)]
    StatePerSquareMetreJoule,
}

impl From<ArealDensityOfStatesUnit> for DynamicSIUnit {
    fn from(unit: ArealDensityOfStatesUnit) -> Self {
        match unit {
            ArealDensityOfStatesUnit::StatePerSquareMetreJoule => {
                StatePerSquareMetreJoule {}.into()
            }
        }
    }
}

/// The units of `VolumetricDensityOfStates`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = VolumetricDensityOfStates<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum VolumetricDensityOfStatesUnit {
    #[default]
    #[unit(StatePerCubicMetreJoule)]
    StatePerCubicMetreJoule,
}

impl From<VolumetricDensityOfStatesUnit> for DynamicSIUnit {
    fn from(unit: VolumetricDensityOfStatesUnit) -> Self {
        match unit {
            VolumetricDensityOfStatesUnit::StatePerCubicMetreJoule => {
                StatePerCubicMetreJoule {}.into()
            }
        }
    }
}

/// The units of `LinearPowerDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LinearPowerDensity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum LinearPowerDensityUnit {
    #[default]
    #[unit(WattPerMetre)]
    WattPerMetre,
}

impl From<LinearPowerDensityUnit> for DynamicSIUnit {
    fn from(unit: LinearPowerDensityUnit) -> Self {
        match unit {
            LinearPowerDensityUnit::WattPerMetre => WattPerMetre {}.into(),
        }
    }
}

/// The units of `VolumetricPowerDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = VolumetricPowerDensity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum VolumetricPowerDensityUnit {
    #[default]
    #[unit(WattPerCubicMetre)]
    WattPerCubicMetre,
}

impl From<VolumetricPowerDensityUnit> for DynamicSIUnit {
    fn from(unit: VolumetricPowerDensityUnit) -> Self {
        match unit {
            VolumetricPowerDensityUnit::WattPerCubicMetre => WattPerCubicMetre {}.into(),
        }
    }
}
