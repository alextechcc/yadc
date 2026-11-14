//! The units of `si`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

#[allow(unused_imports)]
use crate::si::*;
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
    #[value(MOLE.unit())]
    Mole,
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
    #[value(SIEVERT.unit())]
    Sievert,
}

/// The units of `ElectricCapacitance`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum ElectricCapacitanceUnit {
    #[default]
    #[value(FARAD.unit())]
    Farad,
}

/// The units of `ElectricCharge`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum ElectricChargeUnit {
    #[default]
    #[value(COULOMB.unit())]
    Coulomb,
}

/// The units of `ElectricConductance`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum ElectricConductanceUnit {
    #[default]
    #[value(SIEMENS.unit())]
    Siemens,
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
    #[value(AMPERE.unit())]
    Ampere,
}

/// The units of `ElectricPotential`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum ElectricPotentialUnit {
    #[default]
    #[value(VOLT.unit())]
    Volt,
}

/// The units of `ElectricResistance`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum ElectricResistanceUnit {
    #[default]
    #[value(OHM.unit())]
    Ohm,
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
    #[value(JOULE.unit())]
    Joule,
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
    #[value(GRAY.unit())]
    Gray,
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
    #[value(NEWTON.unit())]
    Newton,
}

/// The units of `Frequency`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum FrequencyUnit {
    #[default]
    #[value(HERTZ.unit())]
    Hertz,
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
    #[value(LUX.unit())]
    Lux,
}

/// The units of `Inductance`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum InductanceUnit {
    #[default]
    #[value(HENRY.unit())]
    Henry,
}

/// The units of `Length`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum LengthUnit {
    #[default]
    #[value(METER.unit())]
    Meter,
}

/// The units of `LuminousFlux`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum LuminousFluxUnit {
    #[default]
    #[value(LUMEN.unit())]
    Lumen,
}

/// The units of `LuminousIntensity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum LuminousIntensityUnit {
    #[default]
    #[value(CANDELA.unit())]
    Candela,
}

/// The units of `MagneticFlux`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum MagneticFluxUnit {
    #[default]
    #[value(WEBER.unit())]
    Weber,
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
    #[value(TESLA.unit())]
    Tesla,
}

/// The units of `Mass`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum MassUnit {
    #[default]
    #[value(GRAM.unit())]
    Gram,
}

/// The units of `PlaneAngle`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum PlaneAngleUnit {
    #[default]
    #[value(RADIAN.unit())]
    Radian,
}

/// The units of `Power`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum PowerUnit {
    #[default]
    #[value(WATT.unit())]
    Watt,
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
    #[value(PASCAL.unit())]
    Pascal,
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
    #[value(BECQUEREL.unit())]
    Becquerel,
}

/// The units of `SolidAngle`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum SolidAngleUnit {
    #[default]
    #[value(STERADIAN.unit())]
    Steradian,
}

/// The units of `Time`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum TimeUnit {
    #[default]
    #[value(SECOND.unit())]
    Second,
}
