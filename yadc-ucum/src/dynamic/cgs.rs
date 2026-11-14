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
    #[value(METER_PER_SECOND_SQUARED.unit())]
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
    #[value(CANDELA_PER_METER_SQUARED.unit())]
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
    #[value(METER_SQUARED_PER_SECOND_SQUARED.unit())]
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
    #[value(GRAM_PER_METER_SECOND.unit())]
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
    #[value(COULOMB_PER_SECOND.unit())]
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
    #[value(METER_SQUARED_GRAM_PER_SECOND_SQUARED.unit())]
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
    #[value(METER_SQUARED_PER_SECOND_SQUARED.unit())]
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
    #[value(METER_SQUARED_GRAM_PER_SECOND_COULOMB.unit())]
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
    #[value(METER_GRAM_PER_SECOND_SQUARED.unit())]
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
    #[value(RADIAN_SQUARED_CANDELA_PER_METER_SQUARED.unit())]
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
    #[value(COULOMB_PER_GRAM.unit())]
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
    #[value(METER_SQUARED_PER_SECOND.unit())]
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
    #[value(PER_METER.unit())]
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
    #[value(CANDELA_PER_METER_SQUARED.unit())]
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
    #[value(COULOMB_PER_METER_SECOND.unit())]
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
    #[value(GRAM_PER_SECOND_COULOMB.unit())]
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
    #[value(COULOMB_PER_SECOND.unit())]
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
    #[value(PER_SECOND.unit())]
    PerSecond,
    #[value(CURIE.unit())]
    Curie,
}
