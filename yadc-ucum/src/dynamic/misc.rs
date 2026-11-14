//! The units of `misc`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

#[allow(unused_imports)]
use crate::misc::*;
#[allow(unused_imports)]
use crate::*;

/// The units of `ActionArea`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum ActionAreaUnit {
    #[default]
    #[value(UcumUnit::ONE.times(METER.unit(), 2))]
    MeterSquared,
    #[value(BARN.unit())]
    Barn,
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
    #[value(UcumUnit::ONE.times(METER.unit(), -2).times(SECOND.unit(), 1).times(GRAM.unit(), -1).times(COULOMB.unit(), 2))]
    SecondCoulombSquaredPerMeterSquaredGram,
    #[value(MHO.unit())]
    Mho,
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
    #[value(LINE.unit())]
    Line,
    #[value(POINT.unit())]
    Point,
    #[value(PICA.unit())]
    Pica,
    #[value(PRINTER_S_POINT.unit())]
    PrinterSPoint,
    #[value(PRINTER_S_PICA.unit())]
    PrinterSPica,
    #[value(FRENCH_FOOT.unit())]
    FrenchFoot,
    #[value(FRENCH_INCH.unit())]
    FrenchInch,
    #[value(FRENCH_LINE.unit())]
    FrenchLine,
    #[value(DIDOT_S_POINT.unit())]
    DidotSPoint,
    #[value(DIDOT_S_PICA.unit())]
    DidotSPica,
    #[value(ANGSTROM.unit())]
    Angstrom,
    #[value(SMOOT.unit())]
    Smoot,
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
    #[value(METRIC_CARAT.unit())]
    MetricCarat,
}

/// The units of `MassFraction`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum MassFractionUnit {
    #[default]
    #[value(UcumUnit::ONE)]
    One,
    #[value(CARAT_OF_GOLD_ALLOYS.unit())]
    CaratOfGoldAlloys,
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
    #[value(CIRCLE.unit())]
    Circle,
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
    #[value(UcumUnit::ONE.times(METER.unit(), -1).times(SECOND.unit(), -2).times(GRAM.unit(), 1))]
    GramPerMeterSecondSquared,
    #[value(TECHNICAL_ATMOSPHERE.unit())]
    TechnicalAtmosphere,
    #[value(POUND_PER_SQUARE_INCH.unit())]
    PoundPerSquareInch,
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
    #[value(UcumUnit::ONE.times(RADIAN.unit(), 2))]
    RadianSquared,
    #[value(SPHERE.unit())]
    Sphere,
}

/// The units of `Turbidity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum TurbidityUnit {
    #[default]
    #[value(NEPHELOMETRIC_TURBIDITY_UNIT.unit())]
    NephelometricTurbidityUnit,
    #[value(FORMAZIN_NEPHELOMETRIC_UNIT.unit())]
    FormazinNephelometricUnit,
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
    #[value(STERE.unit())]
    Stere,
}
