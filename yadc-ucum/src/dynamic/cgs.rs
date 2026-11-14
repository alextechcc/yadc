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
    #[value(CURIE.unit())]
    Curie,
}
