use crate::{ISQ, StaticSIQuantity};
use consts::*;
use yadc::Dimension;

pub type Angle = StaticSIQuantity<
    {
        ISQ {
            rad: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type SolidAngle = StaticSIQuantity<
    {
        ISQ {
            rad: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type AngularVelocity = StaticSIQuantity<
    {
        ISQ {
            s: -1,
            rad: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type AngularAcceleration = StaticSIQuantity<
    {
        ISQ {
            s: -2,
            rad: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type AngularJerk = StaticSIQuantity<
    {
        ISQ {
            s: -3,
            rad: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type AngularAbsement = StaticSIQuantity<
    {
        ISQ {
            s: 1,
            rad: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type AngularMomentum = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -1,
            rad: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

/// The constants of this module: the base constants of its quantities
/// (re-exported), then one per multiple, each defined from them - `KILOMETRE`
/// is `1000.0 * METRE`, a `Length` and a unit of its own (`KILOMETRE.value(d)`).
/// Quantity arithmetic is `const`, so they compose in `const` items:
/// `const LIMIT: Velocity = 50.0 * KILOMETRE / HOUR;`.
pub mod consts {
    use super::*;
    use crate::base::{KILOGRAM, METRE};

    /// The radian: an exponent axis here, so an angle never mixes with a plain ratio.
    pub use crate::base::RADIAN;

    /// One `Revolution` (r), as an `Angle`.
    pub const REVOLUTION: Angle = 6.2831853071795862 * RADIAN;

    /// One `Degree` (°), as an `Angle`.
    pub const DEGREE: Angle = 0.017453292519943295 * RADIAN;

    /// One `Gon` (gon), as an `Angle`.
    pub const GON: Angle = 0.015707963267948967 * RADIAN;

    /// One `Mil` (mil), as an `Angle`.
    pub const MIL: Angle = (2.0 * core::f64::consts::PI / 6400.0) * RADIAN;

    /// One `Minute` (′), as an `Angle`.
    pub const MINUTE: Angle = 0.00029088820866572158 * RADIAN;

    /// One `Second` (″), as an `Angle`.
    pub const SECOND: Angle = 4.8481368110953598e-06 * RADIAN;

    /// One `Steradian` (sr), as a `SolidAngle`.
    pub const STERADIAN: SolidAngle = RADIAN * RADIAN;

    /// One `Spat` (sp), as a `SolidAngle`.
    pub const SPAT: SolidAngle = 12.566370614359172 * STERADIAN;

    /// One `SquareDegree` (°²), as a `SolidAngle`.
    pub const SQUARE_DEGREE: SolidAngle = 0.00030461741978670862 * STERADIAN;

    /// One `SquareMinute` (′²), as a `SolidAngle`.
    pub const SQUARE_MINUTE: SolidAngle = 8.4615949940752387e-08 * STERADIAN;

    /// One `SquareSecond` (″²), as a `SolidAngle`.
    pub const SQUARE_SECOND: SolidAngle = 2.3504430539097885e-11 * STERADIAN;

    /// One `RadianPerSecond` (rad/s), as an `AngularVelocity`.
    pub const RADIAN_PER_SECOND: AngularVelocity = RADIAN / crate::base::SECOND;

    /// One `DegreePerSecond` (°/s), as an `AngularVelocity`.
    pub const DEGREE_PER_SECOND: AngularVelocity = 0.017453292519943295 * RADIAN_PER_SECOND;

    /// One `RevolutionPerSecond` (rps), as an `AngularVelocity`.
    pub const REVOLUTION_PER_SECOND: AngularVelocity = 6.2831853071795862 * RADIAN_PER_SECOND;

    /// One `RevolutionPerMinute` (rpm), as an `AngularVelocity`.
    pub const REVOLUTION_PER_MINUTE: AngularVelocity = 0.10471975511965977 * RADIAN_PER_SECOND;

    /// One `RevolutionPerHour` (rph), as an `AngularVelocity`.
    pub const REVOLUTION_PER_HOUR: AngularVelocity = 0.0017453292519943296 * RADIAN_PER_SECOND;

    /// One `RadianPerSecondSquared` (rad/s²), as an `AngularAcceleration`.
    pub const RADIAN_PER_SECOND_SQUARED: AngularAcceleration =
        RADIAN / (crate::base::SECOND * crate::base::SECOND);

    /// One `DegreePerSecondSquared` (°/s²), as an `AngularAcceleration`.
    pub const DEGREE_PER_SECOND_SQUARED: AngularAcceleration =
        0.017453292519943295 * RADIAN_PER_SECOND_SQUARED;

    /// One `RadianPerSecondCubed` (rad/s³), as an `AngularJerk`.
    pub const RADIAN_PER_SECOND_CUBED: AngularJerk =
        RADIAN / (crate::base::SECOND * crate::base::SECOND * crate::base::SECOND);

    /// One `DegreePerSecondCubed` (°/s³), as an `AngularJerk`.
    pub const DEGREE_PER_SECOND_CUBED: AngularJerk = 0.017453292519943295 * RADIAN_PER_SECOND_CUBED;

    /// One `RadianSecond` (rad · s), as an `AngularAbsement`.
    pub const RADIAN_SECOND: AngularAbsement = crate::base::SECOND * RADIAN;

    /// One `DegreeSecond` (° · s), as an `AngularAbsement`.
    pub const DEGREE_SECOND: AngularAbsement = 0.017453292519943295 * RADIAN_SECOND;

    /// One `ReducedPlanckConstant` (ħ), as an `AngularMomentum`.
    pub const REDUCED_PLANCK_CONSTANT: AngularMomentum =
        (6.62607015e-34 / (2.0 * core::f64::consts::PI)) * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `KilogramSquareMetrePerSecond` (kg · m²/s), as an `AngularMomentum`.
    pub const KILOGRAM_SQUARE_METRE_PER_SECOND: AngularMomentum =
        METRE * METRE * KILOGRAM / (crate::base::SECOND * RADIAN);

    /// One `YottanewtonMetreSecond` (YN · m · s), as an `AngularMomentum`.
    pub const YOTTANEWTON_METRE_SECOND: AngularMomentum =
        9.9999999999999998e+23 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `ZettanewtonMetreSecond` (ZN · m · s), as an `AngularMomentum`.
    pub const ZETTANEWTON_METRE_SECOND: AngularMomentum = 1e+21 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `ExanewtonMetreSecond` (EN · m · s), as an `AngularMomentum`.
    pub const EXANEWTON_METRE_SECOND: AngularMomentum = 1e+18 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `PetanewtonMetreSecond` (PN · m · s), as an `AngularMomentum`.
    pub const PETANEWTON_METRE_SECOND: AngularMomentum =
        1000000000000000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `TeranewtonMetreSecond` (TN · m · s), as an `AngularMomentum`.
    pub const TERANEWTON_METRE_SECOND: AngularMomentum =
        1000000000000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `GiganewtonMetreSecond` (GN · m · s), as an `AngularMomentum`.
    pub const GIGANEWTON_METRE_SECOND: AngularMomentum =
        1000000000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `MeganewtonMetreSecond` (MN · m · s), as an `AngularMomentum`.
    pub const MEGANEWTON_METRE_SECOND: AngularMomentum =
        1000000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `KilonewtonMetreSecond` (kN · m · s), as an `AngularMomentum`.
    pub const KILONEWTON_METRE_SECOND: AngularMomentum = 1000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `HectonewtonMetreSecond` (hN · m · s), as an `AngularMomentum`.
    pub const HECTONEWTON_METRE_SECOND: AngularMomentum = 100.0 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `DecanewtonMetreSecond` (daN · m · s), as an `AngularMomentum`.
    pub const DECANEWTON_METRE_SECOND: AngularMomentum = 10.0 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `NewtonMetreSecond` (N · m · s), as an `AngularMomentum`.
    pub const NEWTON_METRE_SECOND: AngularMomentum = KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `DecinewtonMetreSecond` (dN · m · s), as an `AngularMomentum`.
    pub const DECINEWTON_METRE_SECOND: AngularMomentum =
        0.10000000000000001 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `CentinewtonMetreSecond` (cN · m · s), as an `AngularMomentum`.
    pub const CENTINEWTON_METRE_SECOND: AngularMomentum = 0.01 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `MillinewtonMetreSecond` (mN · m · s), as an `AngularMomentum`.
    pub const MILLINEWTON_METRE_SECOND: AngularMomentum = 0.001 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `MicronewtonMetreSecond` (µN · m · s), as an `AngularMomentum`.
    pub const MICRONEWTON_METRE_SECOND: AngularMomentum =
        9.9999999999999995e-07 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `NanonewtonMetreSecond` (nN · m · s), as an `AngularMomentum`.
    pub const NANONEWTON_METRE_SECOND: AngularMomentum =
        1.0000000000000001e-09 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `PiconewtonMetreSecond` (pN · m · s), as an `AngularMomentum`.
    pub const PICONEWTON_METRE_SECOND: AngularMomentum =
        9.9999999999999998e-13 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `FemtonewtonMetreSecond` (fN · m · s), as an `AngularMomentum`.
    pub const FEMTONEWTON_METRE_SECOND: AngularMomentum =
        1.0000000000000001e-15 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `AttonewtonMetreSecond` (aN · m · s), as an `AngularMomentum`.
    pub const ATTONEWTON_METRE_SECOND: AngularMomentum =
        1.0000000000000001e-18 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `ZeptonewtonMetreSecond` (zN · m · s), as an `AngularMomentum`.
    pub const ZEPTONEWTON_METRE_SECOND: AngularMomentum =
        9.9999999999999991e-22 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `YoctonewtonMetreSecond` (yN · m · s), as an `AngularMomentum`.
    pub const YOCTONEWTON_METRE_SECOND: AngularMomentum =
        9.9999999999999992e-25 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `NewtonYottametreSecond` (N · Ym · s), as an `AngularMomentum`.
    pub const NEWTON_YOTTAMETRE_SECOND: AngularMomentum =
        9.9999999999999998e+23 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `NewtonZettametreSecond` (N · Zm · s), as an `AngularMomentum`.
    pub const NEWTON_ZETTAMETRE_SECOND: AngularMomentum = 1e+21 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `NewtonExametreSecond` (N · Em · s), as an `AngularMomentum`.
    pub const NEWTON_EXAMETRE_SECOND: AngularMomentum = 1e+18 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `NewtonPetametreSecond` (N · Pm · s), as an `AngularMomentum`.
    pub const NEWTON_PETAMETRE_SECOND: AngularMomentum =
        1000000000000000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `NewtonTerametreSecond` (N · Tm · s), as an `AngularMomentum`.
    pub const NEWTON_TERAMETRE_SECOND: AngularMomentum =
        1000000000000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `NewtonGigametreSecond` (N · Gm · s), as an `AngularMomentum`.
    pub const NEWTON_GIGAMETRE_SECOND: AngularMomentum =
        1000000000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `NewtonMegametreSecond` (N · Mm · s), as an `AngularMomentum`.
    pub const NEWTON_MEGAMETRE_SECOND: AngularMomentum =
        1000000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `NewtonKilometreSecond` (N · km · s), as an `AngularMomentum`.
    pub const NEWTON_KILOMETRE_SECOND: AngularMomentum = 1000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `NewtonHectometreSecond` (N · hm · s), as an `AngularMomentum`.
    pub const NEWTON_HECTOMETRE_SECOND: AngularMomentum = 100.0 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `NewtonDecametreSecond` (N · dam · s), as an `AngularMomentum`.
    pub const NEWTON_DECAMETRE_SECOND: AngularMomentum = 10.0 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `NewtonDecimetreSecond` (N · dm · s), as an `AngularMomentum`.
    pub const NEWTON_DECIMETRE_SECOND: AngularMomentum =
        0.10000000000000001 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `NewtonCentimetreSecond` (N · cm · s), as an `AngularMomentum`.
    pub const NEWTON_CENTIMETRE_SECOND: AngularMomentum = 0.01 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `NewtonMillimetreSecond` (N · mm · s), as an `AngularMomentum`.
    pub const NEWTON_MILLIMETRE_SECOND: AngularMomentum = 0.001 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `NewtonMicrometreSecond` (N · µm · s), as an `AngularMomentum`.
    pub const NEWTON_MICROMETRE_SECOND: AngularMomentum =
        9.9999999999999995e-07 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `NewtonNanometreSecond` (N · nm · s), as an `AngularMomentum`.
    pub const NEWTON_NANOMETRE_SECOND: AngularMomentum =
        1.0000000000000001e-09 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `NewtonPicometreSecond` (N · pm · s), as an `AngularMomentum`.
    pub const NEWTON_PICOMETRE_SECOND: AngularMomentum =
        9.9999999999999998e-13 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `NewtonFemtometreSecond` (N · fm · s), as an `AngularMomentum`.
    pub const NEWTON_FEMTOMETRE_SECOND: AngularMomentum =
        1.0000000000000001e-15 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `NewtonAttometreSecond` (N · am · s), as an `AngularMomentum`.
    pub const NEWTON_ATTOMETRE_SECOND: AngularMomentum =
        1.0000000000000001e-18 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `NewtonZeptometreSecond` (N · zm · s), as an `AngularMomentum`.
    pub const NEWTON_ZEPTOMETRE_SECOND: AngularMomentum =
        9.9999999999999991e-22 * KILOGRAM_SQUARE_METRE_PER_SECOND;

    /// One `NewtonYoctometreSecond` (N · ym · s), as an `AngularMomentum`.
    pub const NEWTON_YOCTOMETRE_SECOND: AngularMomentum =
        9.9999999999999992e-25 * KILOGRAM_SQUARE_METRE_PER_SECOND;
}

/// The units of `Angle`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Angle)]
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
#[unit(of = SolidAngle)]
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
#[unit(of = AngularVelocity)]
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
#[unit(of = AngularAcceleration)]
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
#[unit(of = AngularJerk)]
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
#[unit(of = AngularAbsement)]
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
#[unit(of = AngularMomentum)]
pub enum AngularMomentumUnit {
    #[unit("ħ", "reduced Planck constants", by = REDUCED_PLANCK_CONSTANT)]
    ReducedPlanckConstant,
    #[default]
    #[unit("kg · m²/s", "kilogram square metres per second", by = KILOGRAM_SQUARE_METRE_PER_SECOND)]
    KilogramSquareMetrePerSecond,
    #[unit("YN · m · s", "yottanewton metre seconds", by = YOTTANEWTON_METRE_SECOND)]
    YottanewtonMetreSecond,
    #[unit("ZN · m · s", "zettanewton metre seconds", by = ZETTANEWTON_METRE_SECOND)]
    ZettanewtonMetreSecond,
    #[unit("EN · m · s", "exanewton metre seconds", by = EXANEWTON_METRE_SECOND)]
    ExanewtonMetreSecond,
    #[unit("PN · m · s", "petanewton metre seconds", by = PETANEWTON_METRE_SECOND)]
    PetanewtonMetreSecond,
    #[unit("TN · m · s", "teranewton metre seconds", by = TERANEWTON_METRE_SECOND)]
    TeranewtonMetreSecond,
    #[unit("GN · m · s", "giganewton metre seconds", by = GIGANEWTON_METRE_SECOND)]
    GiganewtonMetreSecond,
    #[unit("MN · m · s", "meganewton metre seconds", by = MEGANEWTON_METRE_SECOND)]
    MeganewtonMetreSecond,
    #[unit("kN · m · s", "kilonewton metre seconds", by = KILONEWTON_METRE_SECOND)]
    KilonewtonMetreSecond,
    #[unit("hN · m · s", "hectonewton metre seconds", by = HECTONEWTON_METRE_SECOND)]
    HectonewtonMetreSecond,
    #[unit("daN · m · s", "decanewton metre seconds", by = DECANEWTON_METRE_SECOND)]
    DecanewtonMetreSecond,
    #[unit("N · m · s", "newton metre seconds", by = NEWTON_METRE_SECOND)]
    NewtonMetreSecond,
    #[unit("dN · m · s", "decinewton metre seconds", by = DECINEWTON_METRE_SECOND)]
    DecinewtonMetreSecond,
    #[unit("cN · m · s", "centinewton metre seconds", by = CENTINEWTON_METRE_SECOND)]
    CentinewtonMetreSecond,
    #[unit("mN · m · s", "millinewton metre seconds", by = MILLINEWTON_METRE_SECOND)]
    MillinewtonMetreSecond,
    #[unit("µN · m · s", "micronewton metre seconds", by = MICRONEWTON_METRE_SECOND)]
    MicronewtonMetreSecond,
    #[unit("nN · m · s", "nanonewton metre seconds", by = NANONEWTON_METRE_SECOND)]
    NanonewtonMetreSecond,
    #[unit("pN · m · s", "piconewton metre seconds", by = PICONEWTON_METRE_SECOND)]
    PiconewtonMetreSecond,
    #[unit("fN · m · s", "femtonewton metre seconds", by = FEMTONEWTON_METRE_SECOND)]
    FemtonewtonMetreSecond,
    #[unit("aN · m · s", "attonewton metre seconds", by = ATTONEWTON_METRE_SECOND)]
    AttonewtonMetreSecond,
    #[unit("zN · m · s", "zeptonewton metre seconds", by = ZEPTONEWTON_METRE_SECOND)]
    ZeptonewtonMetreSecond,
    #[unit("yN · m · s", "yoctonewton metre seconds", by = YOCTONEWTON_METRE_SECOND)]
    YoctonewtonMetreSecond,
    #[unit("N · Ym · s", "newton yottametre seconds", by = NEWTON_YOTTAMETRE_SECOND)]
    NewtonYottametreSecond,
    #[unit("N · Zm · s", "newton zettametre seconds", by = NEWTON_ZETTAMETRE_SECOND)]
    NewtonZettametreSecond,
    #[unit("N · Em · s", "newton exametre seconds", by = NEWTON_EXAMETRE_SECOND)]
    NewtonExametreSecond,
    #[unit("N · Pm · s", "newton petametre seconds", by = NEWTON_PETAMETRE_SECOND)]
    NewtonPetametreSecond,
    #[unit("N · Tm · s", "newton terametre seconds", by = NEWTON_TERAMETRE_SECOND)]
    NewtonTerametreSecond,
    #[unit("N · Gm · s", "newton gigametre seconds", by = NEWTON_GIGAMETRE_SECOND)]
    NewtonGigametreSecond,
    #[unit("N · Mm · s", "newton megametre seconds", by = NEWTON_MEGAMETRE_SECOND)]
    NewtonMegametreSecond,
    #[unit("N · km · s", "newton kilometre seconds", by = NEWTON_KILOMETRE_SECOND)]
    NewtonKilometreSecond,
    #[unit("N · hm · s", "newton hectometre seconds", by = NEWTON_HECTOMETRE_SECOND)]
    NewtonHectometreSecond,
    #[unit("N · dam · s", "newton decametre seconds", by = NEWTON_DECAMETRE_SECOND)]
    NewtonDecametreSecond,
    #[unit("N · dm · s", "newton decimetre seconds", by = NEWTON_DECIMETRE_SECOND)]
    NewtonDecimetreSecond,
    #[unit("N · cm · s", "newton centimetre seconds", by = NEWTON_CENTIMETRE_SECOND)]
    NewtonCentimetreSecond,
    #[unit("N · mm · s", "newton millimetre seconds", by = NEWTON_MILLIMETRE_SECOND)]
    NewtonMillimetreSecond,
    #[unit("N · µm · s", "newton micrometre seconds", by = NEWTON_MICROMETRE_SECOND)]
    NewtonMicrometreSecond,
    #[unit("N · nm · s", "newton nanometre seconds", by = NEWTON_NANOMETRE_SECOND)]
    NewtonNanometreSecond,
    #[unit("N · pm · s", "newton picometre seconds", by = NEWTON_PICOMETRE_SECOND)]
    NewtonPicometreSecond,
    #[unit("N · fm · s", "newton femtometre seconds", by = NEWTON_FEMTOMETRE_SECOND)]
    NewtonFemtometreSecond,
    #[unit("N · am · s", "newton attometre seconds", by = NEWTON_ATTOMETRE_SECOND)]
    NewtonAttometreSecond,
    #[unit("N · zm · s", "newton zeptometre seconds", by = NEWTON_ZEPTOMETRE_SECOND)]
    NewtonZeptometreSecond,
    #[unit("N · ym · s", "newton yoctometre seconds", by = NEWTON_YOCTOMETRE_SECOND)]
    NewtonYoctometreSecond,
}
