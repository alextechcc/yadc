//! The units of `iso1000`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

#[allow(unused_imports)]
use crate::iso1000::*;
#[allow(unused_imports)]
use crate::*;

/// The units of `Area`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum AreaUnit {
    #[default]
    #[value(ARE.unit())]
    Are,
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
    #[value(ELECTRONVOLT.unit())]
    Electronvolt,
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
    #[value(ASTRONOMIC_UNIT.unit())]
    AstronomicUnit,
    #[value(PARSEC.unit())]
    Parsec,
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
    #[value(TONNE.unit())]
    Tonne,
    #[value(UNIFIED_ATOMIC_MASS_UNIT.unit())]
    UnifiedAtomicMassUnit,
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
    #[value(GRADE.unit())]
    Grade,
    #[value(DEGREE.unit())]
    Degree,
    #[value(PLANE_ANGLE_MINUTE.unit())]
    Minute,
    #[value(PLANE_ANGLE_SECOND.unit())]
    Second,
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
    #[value(BAR.unit())]
    Bar,
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
    #[value(TIME_MINUTE.unit())]
    Minute,
    #[value(HOUR.unit())]
    Hour,
    #[value(DAY.unit())]
    Day,
    #[value(TROPICAL_YEAR.unit())]
    TropicalYear,
    #[value(MEAN_JULIAN_YEAR.unit())]
    MeanJulianYear,
    #[value(MEAN_GREGORIAN_YEAR.unit())]
    MeanGregorianYear,
    #[value(YEAR.unit())]
    Year,
    #[value(WEEK.unit())]
    Week,
    #[value(SYNODAL_MONTH.unit())]
    SynodalMonth,
    #[value(MEAN_JULIAN_MONTH.unit())]
    MeanJulianMonth,
    #[value(MEAN_GREGORIAN_MONTH.unit())]
    MeanGregorianMonth,
    #[value(MONTH.unit())]
    Month,
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
    #[value(LITER.unit())]
    Liter,
}
