//! The units of `cgs`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

#[allow(unused_imports)]
use crate::cgs::*;
#[allow(unused_imports)]
use crate::*;

/// The units of `Acceleration`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum AccelerationUnit {
    #[default]
    #[value(METER.unit().times(SECOND.unit(), -2))]
    MeterPerSecondSquared,
    #[value(GAL.unit())]
    Gal,
}

/// The units of `Brightness`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum BrightnessUnit {
    #[default]
    #[value(UcumUnit::ONE.times(METER.unit(), -2).times(CANDELA.unit(), 1))]
    CandelaPerMeterSquared,
    #[value(LAMBERT.unit())]
    Lambert,
}

/// The units of `DoseEquivalent`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum DoseEquivalentUnit {
    #[default]
    #[value(UcumUnit::ONE.times(METER.unit(), 2).times(SECOND.unit(), -2))]
    MeterSquaredPerSecondSquared,
    #[value(RADIATION_EQUIVALENT_MAN.unit())]
    RadiationEquivalentMan,
}

/// The units of `DynamicViscosity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum DynamicViscosityUnit {
    #[default]
    #[value(UcumUnit::ONE.times(METER.unit(), -1).times(SECOND.unit(), -1).times(GRAM.unit(), 1))]
    GramPerMeterSecond,
    #[value(POISE.unit())]
    Poise,
}

/// The units of `ElectricCurrent`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum ElectricCurrentUnit {
    #[default]
    #[value(UcumUnit::ONE.times(SECOND.unit(), -1).times(COULOMB.unit(), 1))]
    CoulombPerSecond,
    #[value(BIOT.unit())]
    Biot,
}

/// The units of `Energy`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum EnergyUnit {
    #[default]
    #[value(UcumUnit::ONE.times(METER.unit(), 2).times(SECOND.unit(), -2).times(GRAM.unit(), 1))]
    MeterSquaredGramPerSecondSquared,
    #[value(ERG.unit())]
    Erg,
}

/// The units of `EnergyDose`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum EnergyDoseUnit {
    #[default]
    #[value(UcumUnit::ONE.times(METER.unit(), 2).times(SECOND.unit(), -2))]
    MeterSquaredPerSecondSquared,
    #[value(RADIATION_ABSORBED_DOSE.unit())]
    RadiationAbsorbedDose,
}

/// The units of `FluxOfMagneticInduction`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum FluxOfMagneticInductionUnit {
    #[default]
    #[value(UcumUnit::ONE.times(METER.unit(), 2).times(SECOND.unit(), -1).times(GRAM.unit(), 1).times(COULOMB.unit(), -1))]
    MeterSquaredGramPerSecondCoulomb,
    #[value(MAXWELL.unit())]
    Maxwell,
}

/// The units of `Force`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum ForceUnit {
    #[default]
    #[value(METER.unit().times(SECOND.unit(), -2).times(GRAM.unit(), 1))]
    MeterGramPerSecondSquared,
    #[value(DYNE.unit())]
    Dyne,
}

/// The units of `Illuminance`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum IlluminanceUnit {
    #[default]
    #[value(UcumUnit::ONE.times(METER.unit(), -2).times(RADIAN.unit(), 2).times(CANDELA.unit(), 1))]
    RadianSquaredCandelaPerMeterSquared,
    #[value(PHOT.unit())]
    Phot,
}

/// The units of `IonDose`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum IonDoseUnit {
    #[default]
    #[value(UcumUnit::ONE.times(GRAM.unit(), -1).times(COULOMB.unit(), 1))]
    CoulombPerGram,
    #[value(ROENTGEN.unit())]
    Roentgen,
}

/// The units of `KinematicViscosity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum KinematicViscosityUnit {
    #[default]
    #[value(UcumUnit::ONE.times(METER.unit(), 2).times(SECOND.unit(), -1))]
    MeterSquaredPerSecond,
    #[value(STOKES.unit())]
    Stokes,
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
    #[value(UcumUnit::ONE.times(METER.unit(), -1))]
    PerMeter,
    #[value(KAYSER.unit())]
    Kayser,
}

/// The units of `LumIntensityDensity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum LumIntensityDensityUnit {
    #[default]
    #[value(UcumUnit::ONE.times(METER.unit(), -2).times(CANDELA.unit(), 1))]
    CandelaPerMeterSquared,
    #[value(STILB.unit())]
    Stilb,
}

/// The units of `MagneticFieldIntensity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum MagneticFieldIntensityUnit {
    #[default]
    #[value(UcumUnit::ONE.times(METER.unit(), -1).times(SECOND.unit(), -1).times(COULOMB.unit(), 1))]
    CoulombPerMeterSecond,
    #[value(OERSTED.unit())]
    Oersted,
}

/// The units of `MagneticFluxDensity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum MagneticFluxDensityUnit {
    #[default]
    #[value(UcumUnit::ONE.times(SECOND.unit(), -1).times(GRAM.unit(), 1).times(COULOMB.unit(), -1))]
    GramPerSecondCoulomb,
    #[value(GAUSS.unit())]
    Gauss,
}

/// The units of `MagneticTension`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum MagneticTensionUnit {
    #[default]
    #[value(UcumUnit::ONE.times(SECOND.unit(), -1).times(COULOMB.unit(), 1))]
    CoulombPerSecond,
    #[value(GILBERT.unit())]
    Gilbert,
}

/// The units of `Radioactivity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum RadioactivityUnit {
    #[default]
    #[value(UcumUnit::ONE.times(SECOND.unit(), -1))]
    PerSecond,
    #[value(CURIE.unit())]
    Curie,
}
