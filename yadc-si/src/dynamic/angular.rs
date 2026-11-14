//! The units of `angular`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

#[allow(unused_imports)]
use crate::angular::*;
#[allow(unused_imports)]
use crate::*;

/// The units of `Angle`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum AngleUnit {
    #[default]
    #[value(RADIAN.unit())]
    Radian,
    #[value(REVOLUTION.unit())]
    Revolution,
    #[value(DEGREE.unit())]
    Degree,
    #[value(GON.unit())]
    Gon,
    #[value(MIL.unit())]
    Mil,
    #[value(MINUTE.unit())]
    Minute,
    #[value(ANGLE_SECOND.unit())]
    Second,
}

/// The units of `SolidAngle`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum SolidAngleUnit {
    #[default]
    #[value(STERADIAN.unit())]
    Steradian,
    #[value(SPAT.unit())]
    Spat,
    #[value(SQUARE_DEGREE.unit())]
    SquareDegree,
    #[value(SQUARE_MINUTE.unit())]
    SquareMinute,
    #[value(SQUARE_SECOND.unit())]
    SquareSecond,
}

/// The units of `AngularAbsement`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum AngularAbsementUnit {
    #[default]
    #[value(RADIAN_SECOND.unit())]
    RadianSecond,
    #[value(DEGREE_SECOND.unit())]
    DegreeSecond,
}

/// The units of `AngularMomentum`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum AngularMomentumUnit {
    #[value(REDUCED_PLANCK_CONSTANT.unit())]
    ReducedPlanckConstant,
    #[default]
    #[value(NEWTON_METRE_SECOND.unit())]
    NewtonMetreSecond,
}
