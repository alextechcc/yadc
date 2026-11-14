//! The units of `intcust`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

#[allow(unused_imports)]
use crate::intcust::*;
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
    #[value(SQUARE_INCH.unit())]
    SquareInch,
    #[value(SQUARE_FOOT.unit())]
    SquareFoot,
    #[value(SQUARE_YARD.unit())]
    SquareYard,
    #[value(CIRCULAR_MIL.unit())]
    CircularMil,
}

/// The units of `DepthOfWater`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum DepthOfWaterUnit {
    #[default]
    #[value(FATHOM.unit())]
    Fathom,
}

/// The units of `HeightOfHorses`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum HeightOfHorsesUnit {
    #[default]
    #[value(HAND.unit())]
    Hand,
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
    #[value(INCH.unit())]
    Inch,
    #[value(FOOT.unit())]
    Foot,
    #[value(YARD.unit())]
    Yard,
    #[value(MILE.unit())]
    Mile,
    #[value(NAUTICAL_MILE.unit())]
    NauticalMile,
    #[value(MIL.unit())]
    Mil,
}

/// The units of `Velocity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum VelocityUnit {
    #[default]
    #[value(KNOT.unit())]
    Knot,
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
    #[value(CUBIC_INCH.unit())]
    CubicInch,
    #[value(CUBIC_FOOT.unit())]
    CubicFoot,
    #[value(CUBIC_YARD.unit())]
    CubicYard,
    #[value(BOARD_FOOT.unit())]
    BoardFoot,
    #[value(CORD.unit())]
    Cord,
}
