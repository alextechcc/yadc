//! The units of `clinical`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

#[allow(unused_imports)]
use crate::clinical::*;
#[allow(unused_imports)]
use crate::*;

/// The units of `AmountOfSubstance`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum AmountOfSubstanceUnit {
    #[default]
    #[value(ONE.unit())]
    One,
    #[value(EQUIVALENTS.unit())]
    Equivalents,
}

/// The units of `AmountOfSubstanceDissolvedParticles`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum AmountOfSubstanceDissolvedParticlesUnit {
    #[default]
    #[value(ONE.unit())]
    One,
    #[value(OSMOLE.unit())]
    Osmole,
}

/// The units of `CatalyticActivity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum CatalyticActivityUnit {
    #[default]
    #[value(PER_SECOND.unit())]
    PerSecond,
    #[value(KATAL.unit())]
    Katal,
    #[value(UNIT.unit())]
    Unit,
}

/// The units of `FluidResistance`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum FluidResistanceUnit {
    #[default]
    #[value(GRAM_PER_METER_POW4_SECOND.unit())]
    GramPerMeterPow4Second,
    #[value(PERIPHERAL_VASCULAR_RESISTANCE_UNIT.unit())]
    PeripheralVascularResistanceUnit,
    #[value(WOOD_UNIT.unit())]
    WoodUnit,
}

/// The units of `GaugeOfCatheters`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum GaugeOfCathetersUnit {
    #[default]
    #[value(METER.unit())]
    Meter,
    #[value(FRENCH.unit())]
    French,
}

/// The units of `LineicNumber`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum LineicNumberUnit {
    #[default]
    #[value(PER_METER.unit())]
    PerMeter,
    #[value(MESH.unit())]
    Mesh,
}

/// The units of `MassConcentration`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum MassConcentrationUnit {
    #[default]
    #[value(GRAM_PER_METER_CUBED.unit())]
    GramPerMeterCubed,
    #[value(GRAM_PERCENT.unit())]
    GramPercent,
}

/// The units of `MetabolicCostOfPhysicalActivity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum MetabolicCostOfPhysicalActivityUnit {
    #[default]
    #[value(METER_CUBED_PER_SECOND_GRAM.unit())]
    MeterCubedPerSecondGram,
    #[value(METABOLIC_EQUIVALENT.unit())]
    MetabolicEquivalent,
}

/// The units of `Pressure`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum PressureUnit {
    #[default]
    #[value(GRAM_PER_METER_SECOND_SQUARED.unit())]
    GramPerMeterSecondSquared,
    #[value(METER_OF_WATER_COLUMN.unit())]
    MeterOfWaterColumn,
    #[value(METER_OF_MERCURY_COLUMN.unit())]
    MeterOfMercuryColumn,
    #[value(INCH_OF_WATER_COLUMN.unit())]
    InchOfWaterColumn,
    #[value(INCH_OF_MERCURY_COLUMN.unit())]
    InchOfMercuryColumn,
}

/// The units of `RefractionOfALens`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum RefractionOfALensUnit {
    #[default]
    #[value(DIOPTER.unit())]
    Diopter,
}

/// The units of `SedimentationCoefficient`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum SedimentationCoefficientUnit {
    #[default]
    #[value(SECOND.unit())]
    Second,
    #[value(SVEDBERG_UNIT.unit())]
    SvedbergUnit,
}

/// The units of `ViewAreaInMicroscope`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum ViewAreaInMicroscopeUnit {
    #[default]
    #[value(HIGH_POWER_FIELD.unit())]
    HighPowerField,
    #[value(LOW_POWER_FIELD.unit())]
    LowPowerField,
}

/// The units of `Volume`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum VolumeUnit {
    #[default]
    #[value(METER_CUBED.unit())]
    MeterCubed,
    #[value(DROP.unit())]
    Drop,
}

/// The units of `XRayAttenuation`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum XRayAttenuationUnit {
    #[default]
    #[value(HOUNSFIELD_UNIT.unit())]
    HounsfieldUnit,
}
