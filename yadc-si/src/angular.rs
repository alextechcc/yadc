use crate::base::{KILOGRAM, METRE};
use crate::{Exact, ISQ, SIQuantity};
use consts::*;
use yadc::Dimension;

pub type Angle<N> = SIQuantity<
    {
        ISQ {
            rad: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type SolidAngle<N> = SIQuantity<
    {
        ISQ {
            rad: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type AngularVelocity<N> = SIQuantity<
    {
        ISQ {
            s: -1,
            rad: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type AngularAcceleration<N> = SIQuantity<
    {
        ISQ {
            s: -2,
            rad: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type AngularJerk<N> = SIQuantity<
    {
        ISQ {
            s: -3,
            rad: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type AngularAbsement<N> = SIQuantity<
    {
        ISQ {
            s: 1,
            rad: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type AngularMomentum<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -1,
            rad: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

/// The constants of this module: the base constants of its quantities
/// (re-exported), then one per named unit, each defined exactly from them
/// (`INCH` is `0.0254 * METRE`). A unit written as others is not one:
/// `SLUG / CUBIC_FOOT`, `KILO * METRE` are exact too. Converting rounds,
/// once, where it is written - in a `const`, by the compiler:
/// `const MPH: Velocity<f64> = (MILE / HOUR).into();`, then `MPH(60.0)`.
pub mod consts {
    use super::*;
    use crate::base::{KILOGRAM, METRE};

    /// The radian: an exponent axis here, so an angle never mixes with a plain ratio.
    pub use crate::base::RADIAN;

    /// One `Revolution` (r), as an `Angle`.
    pub const REVOLUTION: Angle<Exact> = RADIAN * (Exact::int(2) * Exact::PI);

    /// One `Degree` (°), as an `Angle`.
    pub const DEGREE: Angle<Exact> = RADIAN * (Exact::PI / Exact::int(180));

    /// One `Gon` (gon), as an `Angle`.
    pub const GON: Angle<Exact> = RADIAN * (Exact::PI / Exact::int(200));

    /// One `Mil` (mil), as an `Angle`.
    pub const MIL: Angle<Exact> = RADIAN * (Exact::int(2) * Exact::PI / Exact::int(6400));

    /// One `Minute` (′), as an `Angle`.
    pub const MINUTE: Angle<Exact> = RADIAN * (Exact::PI / Exact::int(10800));

    /// One `Second` (″), as an `Angle`.
    pub const SECOND: Angle<Exact> = RADIAN * (Exact::PI / Exact::int(648000));

    /// One `Steradian` (sr), as a `SolidAngle`.
    pub const STERADIAN: SolidAngle<Exact> = RADIAN * RADIAN;

    /// One `Spat` (sp), as a `SolidAngle`.
    pub const SPAT: SolidAngle<Exact> = STERADIAN * (Exact::int(4) * Exact::PI);

    /// One `RevolutionPerSecond` (rps), as an `AngularVelocity`.
    pub const REVOLUTION_PER_SECOND: AngularVelocity<Exact> =
        (RADIAN / crate::base::SECOND) * (Exact::int(2) * Exact::PI);

    /// One `RevolutionPerMinute` (rpm), as an `AngularVelocity`.
    pub const REVOLUTION_PER_MINUTE: AngularVelocity<Exact> =
        (RADIAN / crate::base::SECOND) * (Exact::int(2) * Exact::PI / Exact::int(60));

    /// One `RevolutionPerHour` (rph), as an `AngularVelocity`.
    pub const REVOLUTION_PER_HOUR: AngularVelocity<Exact> =
        (RADIAN / crate::base::SECOND) * (Exact::int(2) * Exact::PI / Exact::int(3600));

    /// One `ReducedPlanckConstant` (ħ), as an `AngularMomentum`.
    pub const REDUCED_PLANCK_CONSTANT: AngularMomentum<Exact> = (METRE * METRE * KILOGRAM
        / (crate::base::SECOND * RADIAN))
        * (Exact::dec(662607015, -42) / (Exact::int(2) * Exact::PI));
}

/// The units of `Angle`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Angle<N>, number = N)]
#[unit(for<N> where Angle<N>: [const] yadc::Unit<Angle<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
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

/// The units of `SolidAngle`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SolidAngle<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, SolidAngle<N>: [const] yadc::Unit<SolidAngle<N>, N> + [const] core::marker::Destruct)]
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

/// The units of `AngularVelocity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AngularVelocity<N>, number = N)]
#[unit(for<N> where AngularVelocity<N>: [const] yadc::Unit<AngularVelocity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
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

/// The units of `AngularAcceleration`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AngularAcceleration<N>, number = N)]
#[unit(for<N> where AngularAcceleration<N>: [const] yadc::Unit<AngularAcceleration<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum AngularAccelerationUnit {
    #[default]
    #[unit("rad/s²", "radians per second squared", by = AngularAcceleration::<N>::from(RADIAN / (crate::base::SECOND * crate::base::SECOND)))]
    RadianPerSecondSquared,
    #[unit("°/s²", "degrees per second squared", by = AngularAcceleration::<N>::from((RADIAN / (crate::base::SECOND * crate::base::SECOND)) * (Exact::PI / Exact::int(180))))]
    DegreePerSecondSquared,
}

/// The units of `AngularJerk`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AngularJerk<N>, number = N)]
#[unit(for<N> where AngularJerk<N>: [const] yadc::Unit<AngularJerk<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum AngularJerkUnit {
    #[default]
    #[unit("rad/s³", "radians per second cubed", by = AngularJerk::<N>::from(RADIAN / (crate::base::SECOND * crate::base::SECOND * crate::base::SECOND)))]
    RadianPerSecondCubed,
    #[unit("°/s³", "degrees per second cubed", by = AngularJerk::<N>::from((RADIAN / (crate::base::SECOND * crate::base::SECOND * crate::base::SECOND)) * (Exact::PI / Exact::int(180))))]
    DegreePerSecondCubed,
}

/// The units of `AngularAbsement`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AngularAbsement<N>, number = N)]
#[unit(for<N> where AngularAbsement<N>: [const] yadc::Unit<AngularAbsement<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum AngularAbsementUnit {
    #[default]
    #[unit("rad · s", "radian seconds", by = AngularAbsement::<N>::from(crate::base::SECOND * RADIAN))]
    RadianSecond,
    #[unit("° · s", "degree seconds", by = AngularAbsement::<N>::from((crate::base::SECOND * RADIAN) * (Exact::PI / Exact::int(180))))]
    DegreeSecond,
}

/// The units of `AngularMomentum`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AngularMomentum<N>, number = N)]
#[unit(for<N> where AngularMomentum<N>: [const] yadc::Unit<AngularMomentum<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum AngularMomentumUnit {
    #[unit("ħ", "reduced Planck constants", by = AngularMomentum::<N>::from(REDUCED_PLANCK_CONSTANT))]
    ReducedPlanckConstant,
    #[default]
    #[unit("kg · m²/s", "kilogram square metres per second", by = AngularMomentum::<N>::from(METRE * METRE * KILOGRAM / (crate::base::SECOND * RADIAN)))]
    KilogramSquareMetrePerSecond,
    #[unit("N · m · s", "newton metre seconds", by = AngularMomentum::<N>::from(METRE * METRE * KILOGRAM / (crate::base::SECOND * RADIAN)))]
    NewtonMetreSecond,
}
