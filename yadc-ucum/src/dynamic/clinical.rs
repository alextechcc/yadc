//! The units of `clinical`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::clinical::consts::*;
use crate::clinical::*;
use core::marker::Destruct;
use yadc::{Unit, num_traits::Num};

/// The units of `AmountOfSubstance`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AmountOfSubstance<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where AmountOfSubstance<N>: [const] Unit<AmountOfSubstance<N>, N> + [const] Destruct)]
pub enum AmountOfSubstanceUnit {
    #[default]
    #[unit("eq", "equivalents", by = AmountOfSubstance::<N>::from(EQUIVALENTS))]
    Equivalents,
}

/// The units of `AmountOfSubstanceDissolvedParticles`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AmountOfSubstanceDissolvedParticles<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where AmountOfSubstanceDissolvedParticles<N>: [const] Unit<AmountOfSubstanceDissolvedParticles<N>, N> + [const] Destruct)]
pub enum AmountOfSubstanceDissolvedParticlesUnit {
    #[default]
    #[unit("osm", "osmole", by = AmountOfSubstanceDissolvedParticles::<N>::from(OSMOLE))]
    Osmole,
}

/// The units of `CatalyticActivity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = CatalyticActivity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where CatalyticActivity<N>: [const] Unit<CatalyticActivity<N>, N> + [const] Destruct)]
pub enum CatalyticActivityUnit {
    #[default]
    #[unit("kat", "katal", by = CatalyticActivity::<N>::from(KATAL))]
    Katal,
    #[unit("U", "Unit", by = CatalyticActivity::<N>::from(UNIT))]
    Unit,
}

/// The units of `FluidResistance`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = FluidResistance<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where FluidResistance<N>: [const] Unit<FluidResistance<N>, N> + [const] Destruct)]
pub enum FluidResistanceUnit {
    #[default]
    #[unit("[PRU]", "peripheral vascular resistance unit", by = FluidResistance::<N>::from(PERIPHERAL_VASCULAR_RESISTANCE_UNIT))]
    PeripheralVascularResistanceUnit,
    #[unit("[wood'U]", "Wood unit", by = FluidResistance::<N>::from(WOOD_UNIT))]
    WoodUnit,
}

/// The units of `GaugeOfCatheters`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = GaugeOfCatheters<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where GaugeOfCatheters<N>: [const] Unit<GaugeOfCatheters<N>, N> + [const] Destruct)]
pub enum GaugeOfCathetersUnit {
    #[default]
    #[unit("[Ch]", "french", by = GaugeOfCatheters::<N>::from(FRENCH))]
    French,
}

/// The units of `LineicNumber`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LineicNumber<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where LineicNumber<N>: [const] Unit<LineicNumber<N>, N> + [const] Destruct)]
pub enum LineicNumberUnit {
    #[default]
    #[unit("[mesh_i]", "mesh", by = LineicNumber::<N>::from(MESH))]
    Mesh,
}

/// The units of `MassConcentration`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassConcentration<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MassConcentration<N>: [const] Unit<MassConcentration<N>, N> + [const] Destruct)]
pub enum MassConcentrationUnit {
    #[default]
    #[unit("g%", "gram percent", by = MassConcentration::<N>::from(GRAM_PERCENT))]
    GramPercent,
}

/// The units of `MetabolicCostOfPhysicalActivity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MetabolicCostOfPhysicalActivity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MetabolicCostOfPhysicalActivity<N>: [const] Unit<MetabolicCostOfPhysicalActivity<N>, N> + [const] Destruct)]
pub enum MetabolicCostOfPhysicalActivityUnit {
    #[default]
    #[unit("[MET]", "metabolic equivalent", by = MetabolicCostOfPhysicalActivity::<N>::from(METABOLIC_EQUIVALENT))]
    MetabolicEquivalent,
}

/// The units of `Pressure`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Pressure<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Pressure<N>: [const] Unit<Pressure<N>, N> + [const] Destruct)]
pub enum PressureUnit {
    #[default]
    #[unit("m[H2O]", "meter of water column", by = Pressure::<N>::from(METER_OF_WATER_COLUMN))]
    MeterOfWaterColumn,
    #[unit("m[Hg]", "meter of mercury column", by = Pressure::<N>::from(METER_OF_MERCURY_COLUMN))]
    MeterOfMercuryColumn,
    #[unit("[in_i'H2O]", "inch of water column", by = Pressure::<N>::from(INCH_OF_WATER_COLUMN))]
    InchOfWaterColumn,
    #[unit("[in_i'Hg]", "inch of mercury column", by = Pressure::<N>::from(INCH_OF_MERCURY_COLUMN))]
    InchOfMercuryColumn,
}

/// The units of `RefractionOfALens`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = RefractionOfALens<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where RefractionOfALens<N>: [const] Unit<RefractionOfALens<N>, N> + [const] Destruct)]
pub enum RefractionOfALensUnit {
    #[default]
    #[unit("[diop]", "diopter", by = RefractionOfALens::<N>::from(DIOPTER))]
    Diopter,
}

/// The units of `SedimentationCoefficient`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SedimentationCoefficient<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where SedimentationCoefficient<N>: [const] Unit<SedimentationCoefficient<N>, N> + [const] Destruct)]
pub enum SedimentationCoefficientUnit {
    #[default]
    #[unit("[S]", "Svedberg unit", by = SedimentationCoefficient::<N>::from(SVEDBERG_UNIT))]
    SvedbergUnit,
}

/// The units of `ViewAreaInMicroscope`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ViewAreaInMicroscope<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ViewAreaInMicroscope<N>: [const] Unit<ViewAreaInMicroscope<N>, N> + [const] Destruct)]
pub enum ViewAreaInMicroscopeUnit {
    #[default]
    #[unit("[HPF]", "high power field", by = ViewAreaInMicroscope::<N>::from(HIGH_POWER_FIELD))]
    HighPowerField,
    #[unit("[LPF]", "low power field", by = ViewAreaInMicroscope::<N>::from(LOW_POWER_FIELD))]
    LowPowerField,
}

/// The units of `Volume`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Volume<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Volume<N>: [const] Unit<Volume<N>, N> + [const] Destruct)]
pub enum VolumeUnit {
    #[default]
    #[unit("[drp]", "drop", by = Volume::<N>::from(DROP))]
    Drop,
}

/// The units of `XRayAttenuation`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = XRayAttenuation<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where XRayAttenuation<N>: [const] Unit<XRayAttenuation<N>, N> + [const] Destruct)]
pub enum XRayAttenuationUnit {
    #[default]
    #[unit("[hnsf'U]", "Hounsfield unit", by = XRayAttenuation::<N>::from(HOUNSFIELD_UNIT))]
    HounsfieldUnit,
}
