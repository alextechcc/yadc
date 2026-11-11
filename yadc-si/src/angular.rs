use crate::{Exact, ISQ, SIQuantity, counted};
use consts::*;
use yadc::Dimension;

pub type Angle<N = f64> = SIQuantity<
    {
        ISQ {
            rad: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type SolidAngle<N = f64> = SIQuantity<
    {
        ISQ {
            rad: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type AngularVelocity<N = f64> = SIQuantity<
    {
        ISQ {
            s: -1,
            rad: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type AngularAcceleration<N = f64> = SIQuantity<
    {
        ISQ {
            s: -2,
            rad: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type AngularJerk<N = f64> = SIQuantity<
    {
        ISQ {
            s: -3,
            rad: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type AngularAbsement<N = f64> = SIQuantity<
    {
        ISQ {
            s: 1,
            rad: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type AngularMomentum<N = f64> = SIQuantity<
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
/// `KILOMETRE` is `1000 * METRE` - and counted in the number type its use
/// asks for, rounded once: `let d: Length = KILOMETRE;` is an `f64` length,
/// `Length<f32>` an `f32` one, `KILOMETRE::<Exact>` the exact value. Each is
/// a unit (`KILOMETRE(3.0)`, `KILOMETRE.value(d)`), and they compose in
/// `const` items: `const LIMIT: Velocity = 50.0 * KILOMETRE / HOUR;`.
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

    /// One `YottanewtonMetreSecond` (YN · m · s), as an `AngularMomentum`.
    pub const YOTTANEWTON_METRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::int(1000000000000000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `ZettanewtonMetreSecond` (ZN · m · s), as an `AngularMomentum`.
    pub const ZETTANEWTON_METRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::int(1000000000000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `ExanewtonMetreSecond` (EN · m · s), as an `AngularMomentum`.
    pub const EXANEWTON_METRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::int(1000000000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `PetanewtonMetreSecond` (PN · m · s), as an `AngularMomentum`.
    pub const PETANEWTON_METRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::int(1000000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `TeranewtonMetreSecond` (TN · m · s), as an `AngularMomentum`.
    pub const TERANEWTON_METRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::int(1000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `GiganewtonMetreSecond` (GN · m · s), as an `AngularMomentum`.
    pub const GIGANEWTON_METRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::int(1000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `MeganewtonMetreSecond` (MN · m · s), as an `AngularMomentum`.
    pub const MEGANEWTON_METRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::int(1000000) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `KilonewtonMetreSecond` (kN · m · s), as an `AngularMomentum`.
    pub const KILONEWTON_METRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::int(1000) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `HectonewtonMetreSecond` (hN · m · s), as an `AngularMomentum`.
    pub const HECTONEWTON_METRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::int(100) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `DecanewtonMetreSecond` (daN · m · s), as an `AngularMomentum`.
    pub const DECANEWTON_METRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::int(10) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `NewtonMetreSecond` (N · m · s), as an `AngularMomentum`.
    pub const NEWTON_METRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `DecinewtonMetreSecond` (dN · m · s), as an `AngularMomentum`.
    pub const DECINEWTON_METRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::dec(1, -1) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `CentinewtonMetreSecond` (cN · m · s), as an `AngularMomentum`.
    pub const CENTINEWTON_METRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::dec(1, -2) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `MillinewtonMetreSecond` (mN · m · s), as an `AngularMomentum`.
    pub const MILLINEWTON_METRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::dec(1, -3) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `MicronewtonMetreSecond` (µN · m · s), as an `AngularMomentum`.
    pub const MICRONEWTON_METRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::dec(1, -6) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `NanonewtonMetreSecond` (nN · m · s), as an `AngularMomentum`.
    pub const NANONEWTON_METRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::dec(1, -9) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `PiconewtonMetreSecond` (pN · m · s), as an `AngularMomentum`.
    pub const PICONEWTON_METRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::dec(1, -12) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `FemtonewtonMetreSecond` (fN · m · s), as an `AngularMomentum`.
    pub const FEMTONEWTON_METRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::dec(1, -15) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `AttonewtonMetreSecond` (aN · m · s), as an `AngularMomentum`.
    pub const ATTONEWTON_METRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::dec(1, -18) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `ZeptonewtonMetreSecond` (zN · m · s), as an `AngularMomentum`.
    pub const ZEPTONEWTON_METRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::dec(1, -21) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `YoctonewtonMetreSecond` (yN · m · s), as an `AngularMomentum`.
    pub const YOCTONEWTON_METRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::dec(1, -24) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `NewtonYottametreSecond` (N · Ym · s), as an `AngularMomentum`.
    pub const NEWTON_YOTTAMETRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::int(1000000000000000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `NewtonZettametreSecond` (N · Zm · s), as an `AngularMomentum`.
    pub const NEWTON_ZETTAMETRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::int(1000000000000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `NewtonExametreSecond` (N · Em · s), as an `AngularMomentum`.
    pub const NEWTON_EXAMETRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::int(1000000000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `NewtonPetametreSecond` (N · Pm · s), as an `AngularMomentum`.
    pub const NEWTON_PETAMETRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::int(1000000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `NewtonTerametreSecond` (N · Tm · s), as an `AngularMomentum`.
    pub const NEWTON_TERAMETRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::int(1000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `NewtonGigametreSecond` (N · Gm · s), as an `AngularMomentum`.
    pub const NEWTON_GIGAMETRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::int(1000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `NewtonMegametreSecond` (N · Mm · s), as an `AngularMomentum`.
    pub const NEWTON_MEGAMETRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::int(1000000) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `NewtonKilometreSecond` (N · km · s), as an `AngularMomentum`.
    pub const NEWTON_KILOMETRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::int(1000) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `NewtonHectometreSecond` (N · hm · s), as an `AngularMomentum`.
    pub const NEWTON_HECTOMETRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::int(100) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `NewtonDecametreSecond` (N · dam · s), as an `AngularMomentum`.
    pub const NEWTON_DECAMETRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::int(10) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `NewtonDecimetreSecond` (N · dm · s), as an `AngularMomentum`.
    pub const NEWTON_DECIMETRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::dec(1, -1) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `NewtonCentimetreSecond` (N · cm · s), as an `AngularMomentum`.
    pub const NEWTON_CENTIMETRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::dec(1, -2) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `NewtonMillimetreSecond` (N · mm · s), as an `AngularMomentum`.
    pub const NEWTON_MILLIMETRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::dec(1, -3) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `NewtonMicrometreSecond` (N · µm · s), as an `AngularMomentum`.
    pub const NEWTON_MICROMETRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::dec(1, -6) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `NewtonNanometreSecond` (N · nm · s), as an `AngularMomentum`.
    pub const NEWTON_NANOMETRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::dec(1, -9) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `NewtonPicometreSecond` (N · pm · s), as an `AngularMomentum`.
    pub const NEWTON_PICOMETRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::dec(1, -12) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `NewtonFemtometreSecond` (N · fm · s), as an `AngularMomentum`.
    pub const NEWTON_FEMTOMETRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::dec(1, -15) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `NewtonAttometreSecond` (N · am · s), as an `AngularMomentum`.
    pub const NEWTON_ATTOMETRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::dec(1, -18) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `NewtonZeptometreSecond` (N · zm · s), as an `AngularMomentum`.
    pub const NEWTON_ZEPTOMETRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::dec(1, -21) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `NewtonYoctometreSecond` (N · ym · s), as an `AngularMomentum`.
    pub const NEWTON_YOCTOMETRE_SECOND<N: const From<Exact>>: AngularMomentum<N> = counted(Exact::dec(1, -24) * KILOGRAM_SQUARE_METRE_PER_SECOND::<Exact>);
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
