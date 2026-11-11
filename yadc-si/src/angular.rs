use crate::{Exact, ISQ, SIQuantity, counted};
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
/// (re-exported), then one per multiple, each defined exactly from them -
/// `INCH` is `0.0254 * METRE` - and counted in the number type its use asks
/// for, rounded once: `let d: Length<f64> = INCH;` is an `f64` length,
/// `Length<f32>` an `f32` one, `INCH::<Exact>` the exact value. Each is a
/// unit (`INCH(3.0)`, `INCH.value(d)`). A prefixed unit is a prefix times
/// one: `KILO * METRE`. They compose in `const` items:
/// `const LIMIT: Velocity<f64> = 50.0 * (KILO * METRE) / HOUR;`.
pub mod consts {
    use super::*;
    use crate::base::{KILOGRAM, METRE};

    /// The radian: an exponent axis here, so an angle never mixes with a plain ratio.
    pub use crate::base::RADIAN;

    /// One `Revolution` (r), as an `Angle`.
    pub const REVOLUTION<N: const From<Exact>>: Angle<N> = counted((Exact::int(2) * Exact::PI) * RADIAN::<Exact>);

    /// One `Degree` (°), as an `Angle`.
    pub const DEGREE<N: const From<Exact>>: Angle<N> = counted((Exact::PI / Exact::int(180)) * RADIAN::<Exact>);

    /// One `Gon` (gon), as an `Angle`.
    pub const GON<N: const From<Exact>>: Angle<N> = counted((Exact::PI / Exact::int(200)) * RADIAN::<Exact>);

    /// One `Mil` (mil), as an `Angle`.
    pub const MIL<N: const From<Exact>>: Angle<N> = counted((Exact::int(2) * Exact::PI / Exact::int(6400)) * RADIAN::<Exact>);

    /// One `Minute` (′), as an `Angle`.
    pub const MINUTE<N: const From<Exact>>: Angle<N> = counted((Exact::PI / Exact::int(10800)) * RADIAN::<Exact>);

    /// One `Second` (″), as an `Angle`.
    pub const SECOND<N: const From<Exact>>: Angle<N> = counted((Exact::PI / Exact::int(648000)) * RADIAN::<Exact>);

    /// One `Steradian` (sr), as a `SolidAngle`.
    pub const STERADIAN<N: const From<Exact>>: SolidAngle<N> = counted(RADIAN::<Exact> * RADIAN::<Exact>);

    /// One `Spat` (sp), as a `SolidAngle`.
    pub const SPAT<N: const From<Exact>>: SolidAngle<N> = counted((Exact::int(4) * Exact::PI) * STERADIAN::<Exact>);

    /// One `SquareDegree` (°²), as a `SolidAngle`.
    pub const SQUARE_DEGREE<N: const From<Exact>>: SolidAngle<N> = counted(((Exact::PI / Exact::int(180)) * (Exact::PI / Exact::int(180))) * STERADIAN::<Exact>);

    /// One `SquareMinute` (′²), as a `SolidAngle`.
    pub const SQUARE_MINUTE<N: const From<Exact>>: SolidAngle<N> = counted(((Exact::PI / Exact::int(10800)) * (Exact::PI / Exact::int(10800))) * STERADIAN::<Exact>);

    /// One `SquareSecond` (″²), as a `SolidAngle`.
    pub const SQUARE_SECOND<N: const From<Exact>>: SolidAngle<N> = counted(((Exact::PI / Exact::int(648000)) * (Exact::PI / Exact::int(648000))) * STERADIAN::<Exact>);

    /// One `RadianPerSecond` (rad/s), as an `AngularVelocity`.
    pub const RADIAN_PER_SECOND<N: const From<Exact>>: AngularVelocity<N> = counted(RADIAN::<Exact> / crate::base::SECOND::<Exact>);

    /// One `DegreePerSecond` (°/s), as an `AngularVelocity`.
    pub const DEGREE_PER_SECOND<N: const From<Exact>>: AngularVelocity<N> = counted((Exact::PI / Exact::int(180)) * RADIAN_PER_SECOND::<Exact>);

    /// One `RevolutionPerSecond` (rps), as an `AngularVelocity`.
    pub const REVOLUTION_PER_SECOND<N: const From<Exact>>: AngularVelocity<N> = counted((Exact::int(2) * Exact::PI) * RADIAN_PER_SECOND::<Exact>);

    /// One `RevolutionPerMinute` (rpm), as an `AngularVelocity`.
    pub const REVOLUTION_PER_MINUTE<N: const From<Exact>>: AngularVelocity<N> = counted((Exact::int(2) * Exact::PI / Exact::int(60)) * RADIAN_PER_SECOND::<Exact>);

    /// One `RevolutionPerHour` (rph), as an `AngularVelocity`.
    pub const REVOLUTION_PER_HOUR<N: const From<Exact>>: AngularVelocity<N> = counted((Exact::int(2) * Exact::PI / Exact::int(3600)) * RADIAN_PER_SECOND::<Exact>);

    /// One `RadianPerSecondSquared` (rad/s²), as an `AngularAcceleration`.
    pub const RADIAN_PER_SECOND_SQUARED<N: const From<Exact>>: AngularAcceleration<N> = counted(RADIAN::<Exact> / (crate::base::SECOND::<Exact> * crate::base::SECOND::<Exact>));

    /// One `DegreePerSecondSquared` (°/s²), as an `AngularAcceleration`.
    pub const DEGREE_PER_SECOND_SQUARED<N: const From<Exact>>: AngularAcceleration<N> = counted((Exact::PI / Exact::int(180)) * RADIAN_PER_SECOND_SQUARED::<Exact>);

    /// One `RadianPerSecondCubed` (rad/s³), as an `AngularJerk`.
    pub const RADIAN_PER_SECOND_CUBED<N: const From<Exact>>: AngularJerk<N> = counted(RADIAN::<Exact> / (crate::base::SECOND::<Exact> * crate::base::SECOND::<Exact> * crate::base::SECOND::<Exact>));

    /// One `DegreePerSecondCubed` (°/s³), as an `AngularJerk`.
    pub const DEGREE_PER_SECOND_CUBED<N: const From<Exact>>: AngularJerk<N> = counted((Exact::PI / Exact::int(180)) * RADIAN_PER_SECOND_CUBED::<Exact>);

    /// One `RadianSecond` (rad · s), as an `AngularAbsement`.
    pub const RADIAN_SECOND<N: const From<Exact>>: AngularAbsement<N> = counted(crate::base::SECOND::<Exact> * RADIAN::<Exact>);

    /// One `DegreeSecond` (° · s), as an `AngularAbsement`.
    pub const DEGREE_SECOND<N: const From<Exact>>: AngularAbsement<N> = counted((Exact::PI / Exact::int(180)) * RADIAN_SECOND::<Exact>);

    /// One `ReducedPlanckConstant` (ħ), as an `AngularMomentum`.
    pub const REDUCED_PLANCK_CONSTANT<N: const From<Exact>>: AngularMomentum<N> = counted((Exact::dec(662607015, -42) / (Exact::int(2) * Exact::PI)) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `KilogramSquareMetrePerSecond` (kg · m²/s), as an `AngularMomentum`.
    pub const KILOGRAM_SQUARE_METRE_PER_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (crate::base::SECOND::<Exact> * RADIAN::<Exact>));

    /// One `NewtonMetreSecond` (N · m · s), as an `AngularMomentum`.
    pub const NEWTON_METRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);
}

/// The units of `Angle`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Angle<f64>)]
pub enum AngleUnit {
    #[default]
    #[unit("rad", "radians", by = RADIAN)]
    Radian,
    #[unit("r", "revolutions", by = REVOLUTION)]
    Revolution,
    #[unit("°", "degrees", by = DEGREE)]
    Degree,
    #[unit("gon", "gons", by = GON)]
    Gon,
    #[unit("mil", "mils", by = MIL)]
    Mil,
    #[unit("′", "minutes", by = MINUTE)]
    Minute,
    #[unit("″", "seconds", by = SECOND)]
    Second,
}

/// The units of `SolidAngle`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SolidAngle<f64>)]
pub enum SolidAngleUnit {
    #[default]
    #[unit("sr", "steradians", by = STERADIAN)]
    Steradian,
    #[unit("sp", "spats", by = SPAT)]
    Spat,
    #[unit("°²", "square degrees", by = SQUARE_DEGREE)]
    SquareDegree,
    #[unit("′²", "square minutes", by = SQUARE_MINUTE)]
    SquareMinute,
    #[unit("″²", "square seconds", by = SQUARE_SECOND)]
    SquareSecond,
}

/// The units of `AngularVelocity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AngularVelocity<f64>)]
pub enum AngularVelocityUnit {
    #[default]
    #[unit("rad/s", "radians per second", by = RADIAN_PER_SECOND)]
    RadianPerSecond,
    #[unit("°/s", "degrees per second", by = DEGREE_PER_SECOND)]
    DegreePerSecond,
    #[unit("rps", "revolutions per second", by = REVOLUTION_PER_SECOND)]
    RevolutionPerSecond,
    #[unit("rpm", "revolutions per minute", by = REVOLUTION_PER_MINUTE)]
    RevolutionPerMinute,
    #[unit("rph", "revolutions per hour", by = REVOLUTION_PER_HOUR)]
    RevolutionPerHour,
}

/// The units of `AngularAcceleration`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AngularAcceleration<f64>)]
pub enum AngularAccelerationUnit {
    #[default]
    #[unit("rad/s²", "radians per second squared", by = RADIAN_PER_SECOND_SQUARED)]
    RadianPerSecondSquared,
    #[unit("°/s²", "degrees per second squared", by = DEGREE_PER_SECOND_SQUARED)]
    DegreePerSecondSquared,
}

/// The units of `AngularJerk`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AngularJerk<f64>)]
pub enum AngularJerkUnit {
    #[default]
    #[unit("rad/s³", "radians per second cubed", by = RADIAN_PER_SECOND_CUBED)]
    RadianPerSecondCubed,
    #[unit("°/s³", "degrees per second cubed", by = DEGREE_PER_SECOND_CUBED)]
    DegreePerSecondCubed,
}

/// The units of `AngularAbsement`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AngularAbsement<f64>)]
pub enum AngularAbsementUnit {
    #[default]
    #[unit("rad · s", "radian seconds", by = RADIAN_SECOND)]
    RadianSecond,
    #[unit("° · s", "degree seconds", by = DEGREE_SECOND)]
    DegreeSecond,
}

/// The units of `AngularMomentum`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AngularMomentum<f64>)]
pub enum AngularMomentumUnit {
    #[unit("ħ", "reduced Planck constants", by = REDUCED_PLANCK_CONSTANT)]
    ReducedPlanckConstant,
    #[default]
    #[unit("kg · m²/s", "kilogram square metres per second", by = KILOGRAM_SQUARE_METRE_PER_SECOND)]
    KilogramSquareMetrePerSecond,
    #[unit("N · m · s", "newton metre seconds", by = NEWTON_METRE_SECOND)]
    NewtonMetreSecond,
}
