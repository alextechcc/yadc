//! The units of `intcust`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::dynamic::DynamicUcumUnit;
use crate::intcust::units::*;
use crate::intcust::*;
use crate::{Exact, Qty, Unit};
use core::ops::{Add, Div, Mul, Sub};

/// The units of `Area`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<AREA, N>, N> {
        fn quantity(&self, value: N) -> Qty<AREA, N>;
        fn value(&self, quantity: Qty<AREA, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum AreaUnit {
    #[default]
    #[value(SQUARE_INCH)]
    SquareInch,
    #[value(SQUARE_FOOT)]
    SquareFoot,
    #[value(SQUARE_YARD)]
    SquareYard,
    #[value(CIRCULAR_MIL)]
    CircularMil,
}

/// The units of `DepthOfWater`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<DEPTH_OF_WATER, N>, N> {
        fn quantity(&self, value: N) -> Qty<DEPTH_OF_WATER, N>;
        fn value(&self, quantity: Qty<DEPTH_OF_WATER, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum DepthOfWaterUnit {
    #[default]
    #[value(FATHOM)]
    Fathom,
}

/// The units of `HeightOfHorses`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<HEIGHT_OF_HORSES, N>, N> {
        fn quantity(&self, value: N) -> Qty<HEIGHT_OF_HORSES, N>;
        fn value(&self, quantity: Qty<HEIGHT_OF_HORSES, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum HeightOfHorsesUnit {
    #[default]
    #[value(HAND)]
    Hand,
}

/// The units of `Length`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<LENGTH, N>, N> {
        fn quantity(&self, value: N) -> Qty<LENGTH, N>;
        fn value(&self, quantity: Qty<LENGTH, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum LengthUnit {
    #[default]
    #[value(INCH)]
    Inch,
    #[value(FOOT)]
    Foot,
    #[value(YARD)]
    Yard,
    #[value(MILE)]
    Mile,
    #[value(NAUTICAL_MILE)]
    NauticalMile,
    #[value(MIL)]
    Mil,
}

/// The units of `Velocity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<VELOCITY, N>, N> {
        fn quantity(&self, value: N) -> Qty<VELOCITY, N>;
        fn value(&self, quantity: Qty<VELOCITY, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum VelocityUnit {
    #[default]
    #[value(KNOT)]
    Knot,
}

/// The units of `Volume`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<VOLUME, N>, N> {
        fn quantity(&self, value: N) -> Qty<VOLUME, N>;
        fn value(&self, quantity: Qty<VOLUME, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum VolumeUnit {
    #[default]
    #[value(CUBIC_INCH)]
    CubicInch,
    #[value(CUBIC_FOOT)]
    CubicFoot,
    #[value(CUBIC_YARD)]
    CubicYard,
    #[value(BOARD_FOOT)]
    BoardFoot,
    #[value(CORD)]
    Cord,
}
