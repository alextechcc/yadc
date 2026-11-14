//! The units of `chemistry`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::chemistry::units::*;
use crate::chemistry::*;
use crate::dynamic::DynamicSIUnit;
use core::marker::Destruct;
use core::ops::{Add, Div, Mul, Sub};

/// The units of `AmountOfSubstance`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AmountOfSubstance<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum AmountOfSubstanceUnit {
    #[default]
    #[unit(Mole)]
    Mole,
    #[unit(Particle)]
    Particle,
}

impl From<AmountOfSubstanceUnit> for DynamicSIUnit {
    fn from(unit: AmountOfSubstanceUnit) -> Self {
        match unit {
            AmountOfSubstanceUnit::Mole => Mole {}.into(),
            AmountOfSubstanceUnit::Particle => Particle {}.into(),
        }
    }
}

/// The units of `MolarConcentration`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarConcentration<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MolarConcentrationUnit {
    #[default]
    #[unit(MolePerCubicMetre)]
    MolePerCubicMetre,
    #[unit(MolePerLitre)]
    MolePerLitre,
    #[unit(ParticlePerCubicMetre)]
    ParticlePerCubicMetre,
}

impl From<MolarConcentrationUnit> for DynamicSIUnit {
    fn from(unit: MolarConcentrationUnit) -> Self {
        match unit {
            MolarConcentrationUnit::MolePerCubicMetre => MolePerCubicMetre {}.into(),
            MolarConcentrationUnit::MolePerLitre => MolePerLitre {}.into(),
            MolarConcentrationUnit::ParticlePerCubicMetre => ParticlePerCubicMetre {}.into(),
        }
    }
}

/// The units of `MolarEnergy`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarEnergy<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MolarEnergyUnit {
    #[default]
    #[unit(KilogramSquareMetrePerSecondSquaredMole)]
    KilogramSquareMetrePerSecondSquaredMole,
    #[unit(JoulePerMole)]
    JoulePerMole,
    #[unit(WattHourPerMole)]
    WattHourPerMole,
    #[unit(BtuItPerMole)]
    BtuItPerMole,
    #[unit(BtuPerMole)]
    BtuPerMole,
    #[unit(CalorieItPerMole)]
    CalorieItPerMole,
    #[unit(CaloriePerMole)]
    CaloriePerMole,
    #[unit(CalorieItNutritionPerMole)]
    CalorieItNutritionPerMole,
    #[unit(CalorieNutritionPerMole)]
    CalorieNutritionPerMole,
    #[unit(ElectronvoltPerMole)]
    ElectronvoltPerMole,
    #[unit(ErgPerMole)]
    ErgPerMole,
    #[unit(FootPoundalPerMole)]
    FootPoundalPerMole,
    #[unit(FootPoundForcePerMole)]
    FootPoundForcePerMole,
    #[unit(QuadPerMole)]
    QuadPerMole,
    #[unit(ThermEcPerMole)]
    ThermEcPerMole,
    #[unit(ThermUsPerMole)]
    ThermUsPerMole,
    #[unit(TonTntPerMole)]
    TonTntPerMole,
    #[unit(WattSecondPerMole)]
    WattSecondPerMole,
    #[unit(JoulePerParticle)]
    JoulePerParticle,
    #[unit(ElectronvoltPerParticle)]
    ElectronvoltPerParticle,
}

impl From<MolarEnergyUnit> for DynamicSIUnit {
    fn from(unit: MolarEnergyUnit) -> Self {
        match unit {
            MolarEnergyUnit::KilogramSquareMetrePerSecondSquaredMole => {
                KilogramSquareMetrePerSecondSquaredMole {}.into()
            }
            MolarEnergyUnit::JoulePerMole => JoulePerMole {}.into(),
            MolarEnergyUnit::WattHourPerMole => WattHourPerMole {}.into(),
            MolarEnergyUnit::BtuItPerMole => BtuItPerMole {}.into(),
            MolarEnergyUnit::BtuPerMole => BtuPerMole {}.into(),
            MolarEnergyUnit::CalorieItPerMole => CalorieItPerMole {}.into(),
            MolarEnergyUnit::CaloriePerMole => CaloriePerMole {}.into(),
            MolarEnergyUnit::CalorieItNutritionPerMole => CalorieItNutritionPerMole {}.into(),
            MolarEnergyUnit::CalorieNutritionPerMole => CalorieNutritionPerMole {}.into(),
            MolarEnergyUnit::ElectronvoltPerMole => ElectronvoltPerMole {}.into(),
            MolarEnergyUnit::ErgPerMole => ErgPerMole {}.into(),
            MolarEnergyUnit::FootPoundalPerMole => FootPoundalPerMole {}.into(),
            MolarEnergyUnit::FootPoundForcePerMole => FootPoundForcePerMole {}.into(),
            MolarEnergyUnit::QuadPerMole => QuadPerMole {}.into(),
            MolarEnergyUnit::ThermEcPerMole => ThermEcPerMole {}.into(),
            MolarEnergyUnit::ThermUsPerMole => ThermUsPerMole {}.into(),
            MolarEnergyUnit::TonTntPerMole => TonTntPerMole {}.into(),
            MolarEnergyUnit::WattSecondPerMole => WattSecondPerMole {}.into(),
            MolarEnergyUnit::JoulePerParticle => JoulePerParticle {}.into(),
            MolarEnergyUnit::ElectronvoltPerParticle => ElectronvoltPerParticle {}.into(),
        }
    }
}

/// The units of `MolarFlux`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarFlux<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MolarFluxUnit {
    #[default]
    #[unit(MolePerSquareMetreSecond)]
    MolePerSquareMetreSecond,
}

impl From<MolarFluxUnit> for DynamicSIUnit {
    fn from(unit: MolarFluxUnit) -> Self {
        match unit {
            MolarFluxUnit::MolePerSquareMetreSecond => MolePerSquareMetreSecond {}.into(),
        }
    }
}

/// The units of `MolarHeatCapacity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarHeatCapacity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MolarHeatCapacityUnit {
    #[default]
    #[unit(JoulePerKelvinMole)]
    JoulePerKelvinMole,
    #[unit(BtuItPerKelvinMole)]
    BtuItPerKelvinMole,
    #[unit(BtuPerKelvinMole)]
    BtuPerKelvinMole,
    #[unit(CalorieItPerKelvinMole)]
    CalorieItPerKelvinMole,
    #[unit(CaloriePerKelvinMole)]
    CaloriePerKelvinMole,
    #[unit(CalorieItNutritionPerKelvinMole)]
    CalorieItNutritionPerKelvinMole,
    #[unit(CalorieNutritionPerKelvinMole)]
    CalorieNutritionPerKelvinMole,
    #[unit(JoulePerKelvinParticle)]
    JoulePerKelvinParticle,
    #[unit(ElectronvoltPerKelvinParticle)]
    ElectronvoltPerKelvinParticle,
    #[unit(MolarGasConstant)]
    MolarGasConstant,
}

impl From<MolarHeatCapacityUnit> for DynamicSIUnit {
    fn from(unit: MolarHeatCapacityUnit) -> Self {
        match unit {
            MolarHeatCapacityUnit::JoulePerKelvinMole => JoulePerKelvinMole {}.into(),
            MolarHeatCapacityUnit::BtuItPerKelvinMole => BtuItPerKelvinMole {}.into(),
            MolarHeatCapacityUnit::BtuPerKelvinMole => BtuPerKelvinMole {}.into(),
            MolarHeatCapacityUnit::CalorieItPerKelvinMole => CalorieItPerKelvinMole {}.into(),
            MolarHeatCapacityUnit::CaloriePerKelvinMole => CaloriePerKelvinMole {}.into(),
            MolarHeatCapacityUnit::CalorieItNutritionPerKelvinMole => {
                CalorieItNutritionPerKelvinMole {}.into()
            }
            MolarHeatCapacityUnit::CalorieNutritionPerKelvinMole => {
                CalorieNutritionPerKelvinMole {}.into()
            }
            MolarHeatCapacityUnit::JoulePerKelvinParticle => JoulePerKelvinParticle {}.into(),
            MolarHeatCapacityUnit::ElectronvoltPerKelvinParticle => {
                ElectronvoltPerKelvinParticle {}.into()
            }
            MolarHeatCapacityUnit::MolarGasConstant => MolarGasConstant {}.into(),
        }
    }
}

/// The units of `MolarMass`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarMass<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MolarMassUnit {
    #[default]
    #[unit(KilogramPerMole)]
    KilogramPerMole,
    #[unit(GramPerMole)]
    GramPerMole,
}

impl From<MolarMassUnit> for DynamicSIUnit {
    fn from(unit: MolarMassUnit) -> Self {
        match unit {
            MolarMassUnit::KilogramPerMole => KilogramPerMole {}.into(),
            MolarMassUnit::GramPerMole => GramPerMole {}.into(),
        }
    }
}

/// The units of `MolarRadioactivity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarRadioactivity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MolarRadioactivityUnit {
    #[default]
    #[unit(BecquerelPerMole)]
    BecquerelPerMole,
    #[unit(CuriePerMole)]
    CuriePerMole,
    #[unit(DisintegrationsPerMinutePerMole)]
    DisintegrationsPerMinutePerMole,
}

impl From<MolarRadioactivityUnit> for DynamicSIUnit {
    fn from(unit: MolarRadioactivityUnit) -> Self {
        match unit {
            MolarRadioactivityUnit::BecquerelPerMole => BecquerelPerMole {}.into(),
            MolarRadioactivityUnit::CuriePerMole => CuriePerMole {}.into(),
            MolarRadioactivityUnit::DisintegrationsPerMinutePerMole => {
                DisintegrationsPerMinutePerMole {}.into()
            }
        }
    }
}

/// The units of `MolarVolume`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarVolume<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MolarVolumeUnit {
    #[default]
    #[unit(CubicMetrePerMole)]
    CubicMetrePerMole,
    #[unit(CubicMetrePerParticle)]
    CubicMetrePerParticle,
}

impl From<MolarVolumeUnit> for DynamicSIUnit {
    fn from(unit: MolarVolumeUnit) -> Self {
        match unit {
            MolarVolumeUnit::CubicMetrePerMole => CubicMetrePerMole {}.into(),
            MolarVolumeUnit::CubicMetrePerParticle => CubicMetrePerParticle {}.into(),
        }
    }
}

/// The units of `Molality`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Molality<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MolalityUnit {
    #[default]
    #[unit(MolePerKilogram)]
    MolePerKilogram,
}

impl From<MolalityUnit> for DynamicSIUnit {
    fn from(unit: MolalityUnit) -> Self {
        match unit {
            MolalityUnit::MolePerKilogram => MolePerKilogram {}.into(),
        }
    }
}

/// The units of `MassConcentration`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassConcentration<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MassConcentrationUnit {
    #[default]
    #[unit(KilogramPerCubicMetre)]
    KilogramPerCubicMetre,
    #[unit(GramPerCubicMetre)]
    GramPerCubicMetre,
}

impl From<MassConcentrationUnit> for DynamicSIUnit {
    fn from(unit: MassConcentrationUnit) -> Self {
        match unit {
            MassConcentrationUnit::KilogramPerCubicMetre => KilogramPerCubicMetre {}.into(),
            MassConcentrationUnit::GramPerCubicMetre => GramPerCubicMetre {}.into(),
        }
    }
}

/// The units of `CatalyticActivity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = CatalyticActivity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum CatalyticActivityUnit {
    #[default]
    #[unit(Katal)]
    Katal,
    #[unit(ParticlePerSecond)]
    ParticlePerSecond,
    #[unit(MolePerSecond)]
    MolePerSecond,
}

impl From<CatalyticActivityUnit> for DynamicSIUnit {
    fn from(unit: CatalyticActivityUnit) -> Self {
        match unit {
            CatalyticActivityUnit::Katal => Katal {}.into(),
            CatalyticActivityUnit::ParticlePerSecond => ParticlePerSecond {}.into(),
            CatalyticActivityUnit::MolePerSecond => MolePerSecond {}.into(),
        }
    }
}

/// The units of `CatalyticActivityConcentration`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = CatalyticActivityConcentration<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum CatalyticActivityConcentrationUnit {
    #[default]
    #[unit(KatalPerCubicMetre)]
    KatalPerCubicMetre,
    #[unit(KatalPerLitre)]
    KatalPerLitre,
    #[unit(ParticlePerSecondCubicMetre)]
    ParticlePerSecondCubicMetre,
}

impl From<CatalyticActivityConcentrationUnit> for DynamicSIUnit {
    fn from(unit: CatalyticActivityConcentrationUnit) -> Self {
        match unit {
            CatalyticActivityConcentrationUnit::KatalPerCubicMetre => KatalPerCubicMetre {}.into(),
            CatalyticActivityConcentrationUnit::KatalPerLitre => KatalPerLitre {}.into(),
            CatalyticActivityConcentrationUnit::ParticlePerSecondCubicMetre => {
                ParticlePerSecondCubicMetre {}.into()
            }
        }
    }
}

/// The units of `Radioactivity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Radioactivity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum RadioactivityUnit {
    #[default]
    #[unit(Becquerel)]
    Becquerel,
    #[unit(Curie)]
    Curie,
    #[unit(DisintegrationsPerMinute)]
    DisintegrationsPerMinute,
}

impl From<RadioactivityUnit> for DynamicSIUnit {
    fn from(unit: RadioactivityUnit) -> Self {
        match unit {
            RadioactivityUnit::Becquerel => Becquerel {}.into(),
            RadioactivityUnit::Curie => Curie {}.into(),
            RadioactivityUnit::DisintegrationsPerMinute => DisintegrationsPerMinute {}.into(),
        }
    }
}

/// The units of `SpecificRadioactivity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificRadioactivity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum SpecificRadioactivityUnit {
    #[default]
    #[unit(BecquerelPerKilogram)]
    BecquerelPerKilogram,
    #[unit(CuriePerKilogram)]
    CuriePerKilogram,
    #[unit(DisintegrationsPerMinutePerKilogram)]
    DisintegrationsPerMinutePerKilogram,
}

impl From<SpecificRadioactivityUnit> for DynamicSIUnit {
    fn from(unit: SpecificRadioactivityUnit) -> Self {
        match unit {
            SpecificRadioactivityUnit::BecquerelPerKilogram => BecquerelPerKilogram {}.into(),
            SpecificRadioactivityUnit::CuriePerKilogram => CuriePerKilogram {}.into(),
            SpecificRadioactivityUnit::DisintegrationsPerMinutePerKilogram => {
                DisintegrationsPerMinutePerKilogram {}.into()
            }
        }
    }
}

/// The units of `InverseAmountOfSubstance`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = InverseAmountOfSubstance<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum InverseAmountOfSubstanceUnit {
    #[default]
    #[unit(PerMole)]
    PerMole,
    #[unit(AvogadroConstant)]
    AvogadroConstant,
}

impl From<InverseAmountOfSubstanceUnit> for DynamicSIUnit {
    fn from(unit: InverseAmountOfSubstanceUnit) -> Self {
        match unit {
            InverseAmountOfSubstanceUnit::PerMole => PerMole {}.into(),
            InverseAmountOfSubstanceUnit::AvogadroConstant => AvogadroConstant {}.into(),
        }
    }
}
