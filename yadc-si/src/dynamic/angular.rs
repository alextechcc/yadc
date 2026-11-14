//! The units of `angular`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::angular::units::*;
use crate::angular::*;
use crate::dynamic::DynamicSIUnit;
use crate::{Exact, SIQuantity, Unit};
use core::ops::{Add, Div, Mul, Sub};

/// The units of `Angle`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ANGLE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ANGLE, N>;
        fn value(&self, quantity: SIQuantity<ANGLE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum AngleUnit {
    #[default]
    #[value(RADIAN)]
    Radian,
    #[value(REVOLUTION)]
    Revolution,
    #[value(DEGREE)]
    Degree,
    #[value(GON)]
    Gon,
    #[value(MIL)]
    Mil,
    #[value(MINUTE)]
    Minute,
    #[value(SECOND)]
    Second,
}

/// The units of `SolidAngle`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<SOLID_ANGLE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<SOLID_ANGLE, N>;
        fn value(&self, quantity: SIQuantity<SOLID_ANGLE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum SolidAngleUnit {
    #[default]
    #[value(STERADIAN)]
    Steradian,
    #[value(SPAT)]
    Spat,
    #[value(SQUARE_DEGREE)]
    SquareDegree,
    #[value(SQUARE_MINUTE)]
    SquareMinute,
    #[value(SQUARE_SECOND)]
    SquareSecond,
}

/// The units of `AngularVelocity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ANGULAR_VELOCITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ANGULAR_VELOCITY, N>;
        fn value(&self, quantity: SIQuantity<ANGULAR_VELOCITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum AngularVelocityUnit {
    #[default]
    #[value(RADIAN_PER_SECOND)]
    RadianPerSecond,
    #[value(DEGREE_PER_SECOND)]
    DegreePerSecond,
    #[value(REVOLUTION_PER_SECOND)]
    RevolutionPerSecond,
    #[value(REVOLUTION_PER_MINUTE)]
    RevolutionPerMinute,
    #[value(REVOLUTION_PER_HOUR)]
    RevolutionPerHour,
}

/// The units of `AngularAcceleration`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ANGULAR_ACCELERATION, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ANGULAR_ACCELERATION, N>;
        fn value(&self, quantity: SIQuantity<ANGULAR_ACCELERATION, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum AngularAccelerationUnit {
    #[default]
    #[value(RADIAN_PER_SECOND_SQUARED)]
    RadianPerSecondSquared,
    #[value(DEGREE_PER_SECOND_SQUARED)]
    DegreePerSecondSquared,
}

/// The units of `AngularJerk`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ANGULAR_JERK, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ANGULAR_JERK, N>;
        fn value(&self, quantity: SIQuantity<ANGULAR_JERK, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum AngularJerkUnit {
    #[default]
    #[value(RADIAN_PER_SECOND_CUBED)]
    RadianPerSecondCubed,
    #[value(DEGREE_PER_SECOND_CUBED)]
    DegreePerSecondCubed,
}

/// The units of `AngularAbsement`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ANGULAR_ABSEMENT, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ANGULAR_ABSEMENT, N>;
        fn value(&self, quantity: SIQuantity<ANGULAR_ABSEMENT, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum AngularAbsementUnit {
    #[default]
    #[value(RADIAN_SECOND)]
    RadianSecond,
    #[value(DEGREE_SECOND)]
    DegreeSecond,
}

/// The units of `AngularMomentum`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ANGULAR_MOMENTUM, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ANGULAR_MOMENTUM, N>;
        fn value(&self, quantity: SIQuantity<ANGULAR_MOMENTUM, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum AngularMomentumUnit {
    #[value(REDUCED_PLANCK_CONSTANT)]
    ReducedPlanckConstant,
    #[default]
    #[value(KILOGRAM_SQUARE_METRE_PER_SECOND)]
    KilogramSquareMetrePerSecond,
    #[value(NEWTON_METRE_SECOND)]
    NewtonMetreSecond,
}
