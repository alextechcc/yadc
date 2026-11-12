//! The units of `angular`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::angular::consts::*;
use crate::angular::*;
use crate::base::{KILOGRAM, METRE};
use core::marker::Destruct;
use yadc::{Unit, num_traits::Num};

/// The units of `Angle`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Angle<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Angle<N>: [const] Unit<Angle<N>, N> + [const] Destruct)]
pub enum AngleUnit {
    #[default]
    #[unit("rad", "radians", by = Angle::<N>::from(RADIAN))]
    Radian,
    #[unit("r", "revolutions", by = Angle::<N>::from(REVOLUTION))]
    Revolution,
    #[unit("°", "degrees", by = Angle::<N>::from(DEGREE))]
    Degree,
    #[unit("gon", "gons", by = Angle::<N>::from(GON))]
    Gon,
    #[unit("mil", "mils", by = Angle::<N>::from(MIL))]
    Mil,
    #[unit("′", "minutes", by = Angle::<N>::from(MINUTE))]
    Minute,
    #[unit("″", "seconds", by = Angle::<N>::from(SECOND))]
    Second,
}

/// The units of `SolidAngle`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SolidAngle<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where SolidAngle<N>: [const] Unit<SolidAngle<N>, N> + [const] Destruct)]
pub enum SolidAngleUnit {
    #[default]
    #[unit("sr", "steradians", by = SolidAngle::<N>::from(STERADIAN))]
    Steradian,
    #[unit("sp", "spats", by = SolidAngle::<N>::from(SPAT))]
    Spat,
    #[unit("°²", "square degrees", by = SolidAngle::<N>::from(STERADIAN * ((Exact::PI / Exact::int(180)) * (Exact::PI / Exact::int(180)))))]
    SquareDegree,
    #[unit("′²", "square minutes", by = SolidAngle::<N>::from(STERADIAN * ((Exact::PI / Exact::int(10800)) * (Exact::PI / Exact::int(10800)))))]
    SquareMinute,
    #[unit("″²", "square seconds", by = SolidAngle::<N>::from(STERADIAN * ((Exact::PI / Exact::int(648000)) * (Exact::PI / Exact::int(648000)))))]
    SquareSecond,
}

/// The units of `AngularVelocity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AngularVelocity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where AngularVelocity<N>: [const] Unit<AngularVelocity<N>, N> + [const] Destruct)]
pub enum AngularVelocityUnit {
    #[default]
    #[unit("rad/s", "radians per second", by = AngularVelocity::<N>::from(RADIAN / crate::base::SECOND))]
    RadianPerSecond,
    #[unit("°/s", "degrees per second", by = AngularVelocity::<N>::from((RADIAN / crate::base::SECOND) * (Exact::PI / Exact::int(180))))]
    DegreePerSecond,
    #[unit("rps", "revolutions per second", by = AngularVelocity::<N>::from(REVOLUTION_PER_SECOND))]
    RevolutionPerSecond,
    #[unit("rpm", "revolutions per minute", by = AngularVelocity::<N>::from(REVOLUTION_PER_MINUTE))]
    RevolutionPerMinute,
    #[unit("rph", "revolutions per hour", by = AngularVelocity::<N>::from(REVOLUTION_PER_HOUR))]
    RevolutionPerHour,
}

/// The units of `AngularAcceleration`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AngularAcceleration<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where AngularAcceleration<N>: [const] Unit<AngularAcceleration<N>, N> + [const] Destruct)]
pub enum AngularAccelerationUnit {
    #[default]
    #[unit("rad/s²", "radians per second squared", by = AngularAcceleration::<N>::from(RADIAN / (crate::base::SECOND * crate::base::SECOND)))]
    RadianPerSecondSquared,
    #[unit("°/s²", "degrees per second squared", by = AngularAcceleration::<N>::from((RADIAN / (crate::base::SECOND * crate::base::SECOND)) * (Exact::PI / Exact::int(180))))]
    DegreePerSecondSquared,
}

/// The units of `AngularJerk`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AngularJerk<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where AngularJerk<N>: [const] Unit<AngularJerk<N>, N> + [const] Destruct)]
pub enum AngularJerkUnit {
    #[default]
    #[unit("rad/s³", "radians per second cubed", by = AngularJerk::<N>::from(RADIAN / (crate::base::SECOND * crate::base::SECOND * crate::base::SECOND)))]
    RadianPerSecondCubed,
    #[unit("°/s³", "degrees per second cubed", by = AngularJerk::<N>::from((RADIAN / (crate::base::SECOND * crate::base::SECOND * crate::base::SECOND)) * (Exact::PI / Exact::int(180))))]
    DegreePerSecondCubed,
}

/// The units of `AngularAbsement`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AngularAbsement<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where AngularAbsement<N>: [const] Unit<AngularAbsement<N>, N> + [const] Destruct)]
pub enum AngularAbsementUnit {
    #[default]
    #[unit("rad · s", "radian seconds", by = AngularAbsement::<N>::from(crate::base::SECOND * RADIAN))]
    RadianSecond,
    #[unit("° · s", "degree seconds", by = AngularAbsement::<N>::from((crate::base::SECOND * RADIAN) * (Exact::PI / Exact::int(180))))]
    DegreeSecond,
}

/// The units of `AngularMomentum`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AngularMomentum<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where AngularMomentum<N>: [const] Unit<AngularMomentum<N>, N> + [const] Destruct)]
pub enum AngularMomentumUnit {
    #[unit("ħ", "reduced Planck constants", by = AngularMomentum::<N>::from(REDUCED_PLANCK_CONSTANT))]
    ReducedPlanckConstant,
    #[default]
    #[unit("kg · m²/s", "kilogram square metres per second", by = AngularMomentum::<N>::from(METRE * METRE * KILOGRAM / (crate::base::SECOND * RADIAN)))]
    KilogramSquareMetrePerSecond,
    #[unit("N · m · s", "newton metre seconds", by = AngularMomentum::<N>::from(METRE * METRE * KILOGRAM / (crate::base::SECOND * RADIAN)))]
    NewtonMetreSecond,
}
