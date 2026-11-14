//! The units of `clinical`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::clinical::units::*;
use crate::clinical::*;
use crate::dynamic::DynamicUcumUnit;
use core::marker::Destruct;
use core::ops::{Add, Div, Mul, Sub};

/// The units of `AmountOfSubstance`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AmountOfSubstance<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum AmountOfSubstanceUnit {
    #[default]
    #[unit(Equivalents)]
    Equivalents,
}

impl From<AmountOfSubstanceUnit> for DynamicUcumUnit {
    fn from(unit: AmountOfSubstanceUnit) -> Self {
        match unit {
            AmountOfSubstanceUnit::Equivalents => Equivalents {}.into(),
        }
    }
}

/// The units of `AmountOfSubstanceDissolvedParticles`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AmountOfSubstanceDissolvedParticles<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum AmountOfSubstanceDissolvedParticlesUnit {
    #[default]
    #[unit(Osmole)]
    Osmole,
}

impl From<AmountOfSubstanceDissolvedParticlesUnit> for DynamicUcumUnit {
    fn from(unit: AmountOfSubstanceDissolvedParticlesUnit) -> Self {
        match unit {
            AmountOfSubstanceDissolvedParticlesUnit::Osmole => Osmole {}.into(),
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
    #[unit(Unit)]
    Unit,
}

impl From<CatalyticActivityUnit> for DynamicUcumUnit {
    fn from(unit: CatalyticActivityUnit) -> Self {
        match unit {
            CatalyticActivityUnit::Katal => Katal {}.into(),
            CatalyticActivityUnit::Unit => Unit {}.into(),
        }
    }
}

/// The units of `FluidResistance`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = FluidResistance<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum FluidResistanceUnit {
    #[default]
    #[unit(PeripheralVascularResistanceUnit)]
    PeripheralVascularResistanceUnit,
    #[unit(WoodUnit)]
    WoodUnit,
}

impl From<FluidResistanceUnit> for DynamicUcumUnit {
    fn from(unit: FluidResistanceUnit) -> Self {
        match unit {
            FluidResistanceUnit::PeripheralVascularResistanceUnit => {
                PeripheralVascularResistanceUnit {}.into()
            }
            FluidResistanceUnit::WoodUnit => WoodUnit {}.into(),
        }
    }
}

/// The units of `GaugeOfCatheters`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = GaugeOfCatheters<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum GaugeOfCathetersUnit {
    #[default]
    #[unit(French)]
    French,
}

impl From<GaugeOfCathetersUnit> for DynamicUcumUnit {
    fn from(unit: GaugeOfCathetersUnit) -> Self {
        match unit {
            GaugeOfCathetersUnit::French => French {}.into(),
        }
    }
}

/// The units of `LineicNumber`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LineicNumber<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum LineicNumberUnit {
    #[default]
    #[unit(Mesh)]
    Mesh,
}

impl From<LineicNumberUnit> for DynamicUcumUnit {
    fn from(unit: LineicNumberUnit) -> Self {
        match unit {
            LineicNumberUnit::Mesh => Mesh {}.into(),
        }
    }
}

/// The units of `MassConcentration`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassConcentration<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MassConcentrationUnit {
    #[default]
    #[unit(GramPercent)]
    GramPercent,
}

impl From<MassConcentrationUnit> for DynamicUcumUnit {
    fn from(unit: MassConcentrationUnit) -> Self {
        match unit {
            MassConcentrationUnit::GramPercent => GramPercent {}.into(),
        }
    }
}

/// The units of `MetabolicCostOfPhysicalActivity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MetabolicCostOfPhysicalActivity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MetabolicCostOfPhysicalActivityUnit {
    #[default]
    #[unit(MetabolicEquivalent)]
    MetabolicEquivalent,
}

impl From<MetabolicCostOfPhysicalActivityUnit> for DynamicUcumUnit {
    fn from(unit: MetabolicCostOfPhysicalActivityUnit) -> Self {
        match unit {
            MetabolicCostOfPhysicalActivityUnit::MetabolicEquivalent => {
                MetabolicEquivalent {}.into()
            }
        }
    }
}

/// The units of `Pressure`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Pressure<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum PressureUnit {
    #[default]
    #[unit(MeterOfWaterColumn)]
    MeterOfWaterColumn,
    #[unit(MeterOfMercuryColumn)]
    MeterOfMercuryColumn,
    #[unit(InchOfWaterColumn)]
    InchOfWaterColumn,
    #[unit(InchOfMercuryColumn)]
    InchOfMercuryColumn,
}

impl From<PressureUnit> for DynamicUcumUnit {
    fn from(unit: PressureUnit) -> Self {
        match unit {
            PressureUnit::MeterOfWaterColumn => MeterOfWaterColumn {}.into(),
            PressureUnit::MeterOfMercuryColumn => MeterOfMercuryColumn {}.into(),
            PressureUnit::InchOfWaterColumn => InchOfWaterColumn {}.into(),
            PressureUnit::InchOfMercuryColumn => InchOfMercuryColumn {}.into(),
        }
    }
}

/// The units of `RefractionOfALens`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = RefractionOfALens<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum RefractionOfALensUnit {
    #[default]
    #[unit(Diopter)]
    Diopter,
}

impl From<RefractionOfALensUnit> for DynamicUcumUnit {
    fn from(unit: RefractionOfALensUnit) -> Self {
        match unit {
            RefractionOfALensUnit::Diopter => Diopter {}.into(),
        }
    }
}

/// The units of `SedimentationCoefficient`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SedimentationCoefficient<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum SedimentationCoefficientUnit {
    #[default]
    #[unit(SvedbergUnit)]
    SvedbergUnit,
}

impl From<SedimentationCoefficientUnit> for DynamicUcumUnit {
    fn from(unit: SedimentationCoefficientUnit) -> Self {
        match unit {
            SedimentationCoefficientUnit::SvedbergUnit => SvedbergUnit {}.into(),
        }
    }
}

/// The units of `ViewAreaInMicroscope`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ViewAreaInMicroscope<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ViewAreaInMicroscopeUnit {
    #[default]
    #[unit(HighPowerField)]
    HighPowerField,
    #[unit(LowPowerField)]
    LowPowerField,
}

impl From<ViewAreaInMicroscopeUnit> for DynamicUcumUnit {
    fn from(unit: ViewAreaInMicroscopeUnit) -> Self {
        match unit {
            ViewAreaInMicroscopeUnit::HighPowerField => HighPowerField {}.into(),
            ViewAreaInMicroscopeUnit::LowPowerField => LowPowerField {}.into(),
        }
    }
}

/// The units of `Volume`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Volume<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum VolumeUnit {
    #[default]
    #[unit(Drop)]
    Drop,
}

impl From<VolumeUnit> for DynamicUcumUnit {
    fn from(unit: VolumeUnit) -> Self {
        match unit {
            VolumeUnit::Drop => Drop {}.into(),
        }
    }
}

/// The units of `XRayAttenuation`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = XRayAttenuation<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum XRayAttenuationUnit {
    #[default]
    #[unit(HounsfieldUnit)]
    HounsfieldUnit,
}

impl From<XRayAttenuationUnit> for DynamicUcumUnit {
    fn from(unit: XRayAttenuationUnit) -> Self {
        match unit {
            XRayAttenuationUnit::HounsfieldUnit => HounsfieldUnit {}.into(),
        }
    }
}
