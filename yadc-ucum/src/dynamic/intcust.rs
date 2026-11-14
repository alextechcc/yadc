//! The units of `intcust`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::dynamic::DynamicUcumUnit;
use crate::intcust::units::*;
use crate::intcust::*;
use core::marker::Destruct;
use core::ops::{Add, Div, Mul, Sub};

/// The units of `Area`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Area<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum AreaUnit {
    #[default]
    #[unit(SquareInch)]
    SquareInch,
    #[unit(SquareFoot)]
    SquareFoot,
    #[unit(SquareYard)]
    SquareYard,
    #[unit(CircularMil)]
    CircularMil,
}

impl From<AreaUnit> for DynamicUcumUnit {
    fn from(unit: AreaUnit) -> Self {
        match unit {
            AreaUnit::SquareInch => SquareInch {}.into(),
            AreaUnit::SquareFoot => SquareFoot {}.into(),
            AreaUnit::SquareYard => SquareYard {}.into(),
            AreaUnit::CircularMil => CircularMil {}.into(),
        }
    }
}

/// The units of `DepthOfWater`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = DepthOfWater<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum DepthOfWaterUnit {
    #[default]
    #[unit(Fathom)]
    Fathom,
}

impl From<DepthOfWaterUnit> for DynamicUcumUnit {
    fn from(unit: DepthOfWaterUnit) -> Self {
        match unit {
            DepthOfWaterUnit::Fathom => Fathom {}.into(),
        }
    }
}

/// The units of `HeightOfHorses`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = HeightOfHorses<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum HeightOfHorsesUnit {
    #[default]
    #[unit(Hand)]
    Hand,
}

impl From<HeightOfHorsesUnit> for DynamicUcumUnit {
    fn from(unit: HeightOfHorsesUnit) -> Self {
        match unit {
            HeightOfHorsesUnit::Hand => Hand {}.into(),
        }
    }
}

/// The units of `Length`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum LengthUnit {
    #[default]
    #[unit(Inch)]
    Inch,
    #[unit(Foot)]
    Foot,
    #[unit(Yard)]
    Yard,
    #[unit(Mile)]
    Mile,
    #[unit(NauticalMile)]
    NauticalMile,
    #[unit(Mil)]
    Mil,
}

impl From<LengthUnit> for DynamicUcumUnit {
    fn from(unit: LengthUnit) -> Self {
        match unit {
            LengthUnit::Inch => Inch {}.into(),
            LengthUnit::Foot => Foot {}.into(),
            LengthUnit::Yard => Yard {}.into(),
            LengthUnit::Mile => Mile {}.into(),
            LengthUnit::NauticalMile => NauticalMile {}.into(),
            LengthUnit::Mil => Mil {}.into(),
        }
    }
}

/// The units of `Velocity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Velocity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum VelocityUnit {
    #[default]
    #[unit(Knot)]
    Knot,
}

impl From<VelocityUnit> for DynamicUcumUnit {
    fn from(unit: VelocityUnit) -> Self {
        match unit {
            VelocityUnit::Knot => Knot {}.into(),
        }
    }
}

/// The units of `Volume`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Volume<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum VolumeUnit {
    #[default]
    #[unit(CubicInch)]
    CubicInch,
    #[unit(CubicFoot)]
    CubicFoot,
    #[unit(CubicYard)]
    CubicYard,
    #[unit(BoardFoot)]
    BoardFoot,
    #[unit(Cord)]
    Cord,
}

impl From<VolumeUnit> for DynamicUcumUnit {
    fn from(unit: VolumeUnit) -> Self {
        match unit {
            VolumeUnit::CubicInch => CubicInch {}.into(),
            VolumeUnit::CubicFoot => CubicFoot {}.into(),
            VolumeUnit::CubicYard => CubicYard {}.into(),
            VolumeUnit::BoardFoot => BoardFoot {}.into(),
            VolumeUnit::Cord => Cord {}.into(),
        }
    }
}
