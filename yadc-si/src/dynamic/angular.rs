//! The units of `angular`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::angular::units::*;
use crate::angular::*;
use crate::dynamic::DynamicSIUnit;
use core::marker::Destruct;
use core::ops::{Add, Div, Mul, Sub};

/// The units of `Angle`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Angle<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum AngleUnit {
    #[default]
    #[unit(Radian)]
    Radian,
    #[unit(Revolution)]
    Revolution,
    #[unit(Degree)]
    Degree,
    #[unit(Gon)]
    Gon,
    #[unit(Mil)]
    Mil,
    #[unit(Minute)]
    Minute,
    #[unit(Second)]
    Second,
}

impl From<AngleUnit> for DynamicSIUnit {
    fn from(unit: AngleUnit) -> Self {
        match unit {
            AngleUnit::Radian => Radian {}.into(),
            AngleUnit::Revolution => Revolution {}.into(),
            AngleUnit::Degree => Degree {}.into(),
            AngleUnit::Gon => Gon {}.into(),
            AngleUnit::Mil => Mil {}.into(),
            AngleUnit::Minute => Minute {}.into(),
            AngleUnit::Second => Second {}.into(),
        }
    }
}

/// The units of `SolidAngle`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SolidAngle<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum SolidAngleUnit {
    #[default]
    #[unit(Steradian)]
    Steradian,
    #[unit(Spat)]
    Spat,
    #[unit(SquareDegree)]
    SquareDegree,
    #[unit(SquareMinute)]
    SquareMinute,
    #[unit(SquareSecond)]
    SquareSecond,
}

impl From<SolidAngleUnit> for DynamicSIUnit {
    fn from(unit: SolidAngleUnit) -> Self {
        match unit {
            SolidAngleUnit::Steradian => Steradian {}.into(),
            SolidAngleUnit::Spat => Spat {}.into(),
            SolidAngleUnit::SquareDegree => SquareDegree {}.into(),
            SolidAngleUnit::SquareMinute => SquareMinute {}.into(),
            SolidAngleUnit::SquareSecond => SquareSecond {}.into(),
        }
    }
}

/// The units of `AngularVelocity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AngularVelocity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum AngularVelocityUnit {
    #[default]
    #[unit(RadianPerSecond)]
    RadianPerSecond,
    #[unit(DegreePerSecond)]
    DegreePerSecond,
    #[unit(RevolutionPerSecond)]
    RevolutionPerSecond,
    #[unit(RevolutionPerMinute)]
    RevolutionPerMinute,
    #[unit(RevolutionPerHour)]
    RevolutionPerHour,
}

impl From<AngularVelocityUnit> for DynamicSIUnit {
    fn from(unit: AngularVelocityUnit) -> Self {
        match unit {
            AngularVelocityUnit::RadianPerSecond => RadianPerSecond {}.into(),
            AngularVelocityUnit::DegreePerSecond => DegreePerSecond {}.into(),
            AngularVelocityUnit::RevolutionPerSecond => RevolutionPerSecond {}.into(),
            AngularVelocityUnit::RevolutionPerMinute => RevolutionPerMinute {}.into(),
            AngularVelocityUnit::RevolutionPerHour => RevolutionPerHour {}.into(),
        }
    }
}

/// The units of `AngularAcceleration`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AngularAcceleration<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum AngularAccelerationUnit {
    #[default]
    #[unit(RadianPerSecondSquared)]
    RadianPerSecondSquared,
    #[unit(DegreePerSecondSquared)]
    DegreePerSecondSquared,
}

impl From<AngularAccelerationUnit> for DynamicSIUnit {
    fn from(unit: AngularAccelerationUnit) -> Self {
        match unit {
            AngularAccelerationUnit::RadianPerSecondSquared => RadianPerSecondSquared {}.into(),
            AngularAccelerationUnit::DegreePerSecondSquared => DegreePerSecondSquared {}.into(),
        }
    }
}

/// The units of `AngularJerk`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AngularJerk<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum AngularJerkUnit {
    #[default]
    #[unit(RadianPerSecondCubed)]
    RadianPerSecondCubed,
    #[unit(DegreePerSecondCubed)]
    DegreePerSecondCubed,
}

impl From<AngularJerkUnit> for DynamicSIUnit {
    fn from(unit: AngularJerkUnit) -> Self {
        match unit {
            AngularJerkUnit::RadianPerSecondCubed => RadianPerSecondCubed {}.into(),
            AngularJerkUnit::DegreePerSecondCubed => DegreePerSecondCubed {}.into(),
        }
    }
}

/// The units of `AngularAbsement`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AngularAbsement<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum AngularAbsementUnit {
    #[default]
    #[unit(RadianSecond)]
    RadianSecond,
    #[unit(DegreeSecond)]
    DegreeSecond,
}

impl From<AngularAbsementUnit> for DynamicSIUnit {
    fn from(unit: AngularAbsementUnit) -> Self {
        match unit {
            AngularAbsementUnit::RadianSecond => RadianSecond {}.into(),
            AngularAbsementUnit::DegreeSecond => DegreeSecond {}.into(),
        }
    }
}

/// The units of `AngularMomentum`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AngularMomentum<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum AngularMomentumUnit {
    #[unit(ReducedPlanckConstant)]
    ReducedPlanckConstant,
    #[default]
    #[unit(KilogramSquareMetrePerSecond)]
    KilogramSquareMetrePerSecond,
    #[unit(NewtonMetreSecond)]
    NewtonMetreSecond,
}

impl From<AngularMomentumUnit> for DynamicSIUnit {
    fn from(unit: AngularMomentumUnit) -> Self {
        match unit {
            AngularMomentumUnit::ReducedPlanckConstant => ReducedPlanckConstant {}.into(),
            AngularMomentumUnit::KilogramSquareMetrePerSecond => {
                KilogramSquareMetrePerSecond {}.into()
            }
            AngularMomentumUnit::NewtonMetreSecond => NewtonMetreSecond {}.into(),
        }
    }
}
