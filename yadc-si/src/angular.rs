use crate::{Exact, ISQ, SIQuantity};
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
/// `const MPH: Velocity<f64> = (MILE / HOUR).into();`, then `MPH.quantity(60.0)`.
pub mod consts {
    use super::*;
    use crate::base::{KILOGRAM, METRE};

    /// The radian: an exponent axis here, so an angle never mixes with a plain ratio.
    pub use crate::base::RADIAN;

    /// One `Revolution` (`r`), as an `Angle`.
    pub const REVOLUTION: Angle<Exact> = RADIAN * (Exact::int(2) * Exact::PI);

    /// One `Degree` (`°`), as an `Angle`.
    pub const DEGREE: Angle<Exact> = RADIAN * (Exact::PI / Exact::int(180));

    /// One `Gon` (`gon`), as an `Angle`.
    pub const GON: Angle<Exact> = RADIAN * (Exact::PI / Exact::int(200));

    /// One `Mil` (`mil`), as an `Angle`.
    pub const MIL: Angle<Exact> = RADIAN * (Exact::int(2) * Exact::PI / Exact::int(6400));

    /// One `Minute` (`′`), as an `Angle`.
    pub const MINUTE: Angle<Exact> = RADIAN * (Exact::PI / Exact::int(10800));

    /// One `Second` (`″`), as an `Angle`.
    pub const SECOND: Angle<Exact> = RADIAN * (Exact::PI / Exact::int(648000));

    /// One `Steradian` (`sr`), as a `SolidAngle`.
    pub const STERADIAN: SolidAngle<Exact> = RADIAN * RADIAN;

    /// One `Spat` (`sp`), as a `SolidAngle`.
    pub const SPAT: SolidAngle<Exact> = STERADIAN * (Exact::int(4) * Exact::PI);

    /// One `RevolutionPerSecond` (`rps`), as an `AngularVelocity`.
    pub const REVOLUTION_PER_SECOND: AngularVelocity<Exact> =
        (RADIAN / crate::base::SECOND) * (Exact::int(2) * Exact::PI);

    /// One `RevolutionPerMinute` (`rpm`), as an `AngularVelocity`.
    pub const REVOLUTION_PER_MINUTE: AngularVelocity<Exact> =
        (RADIAN / crate::base::SECOND) * (Exact::int(2) * Exact::PI / Exact::int(60));

    /// One `RevolutionPerHour` (`rph`), as an `AngularVelocity`.
    pub const REVOLUTION_PER_HOUR: AngularVelocity<Exact> =
        (RADIAN / crate::base::SECOND) * (Exact::int(2) * Exact::PI / Exact::int(3600));

    /// One `ReducedPlanckConstant` (`ħ`), as an `AngularMomentum`.
    pub const REDUCED_PLANCK_CONSTANT: AngularMomentum<Exact> = (METRE * METRE * KILOGRAM
        / (crate::base::SECOND * RADIAN))
        * (Exact::dec(662607015, -42) / (Exact::int(2) * Exact::PI));
}
