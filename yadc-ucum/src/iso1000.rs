use crate::{Dim, Exact, Qty};
use yadc::Dimension;

pub type Area<N> = Qty<
    {
        Dim {
            m: 2,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Energy<N> = Qty<
    {
        Dim {
            m: 2,
            s: -2,
            g: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Length<N> = Qty<
    {
        Dim {
            m: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Mass<N> = Qty<
    {
        Dim {
            g: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type PlaneAngle<N> = Qty<
    {
        Dim {
            rad: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Pressure<N> = Qty<
    {
        Dim {
            m: -1,
            s: -2,
            g: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Time<N> = Qty<
    {
        Dim {
            s: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Volume<N> = Qty<
    {
        Dim {
            m: 3,
            ..Dim::DIMENSIONLESS
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

    /// The metre, UCUM's base unit of length.
    pub use crate::base::METRE;

    /// The gram, UCUM's base unit of mass.
    pub use crate::base::GRAM;

    /// The radian, UCUM's base unit of plane angle.
    pub use crate::base::RADIAN;

    /// One `Are` (`ar`), as an `Area`.
    pub const ARE: Area<Exact> = (METRE * METRE) * Exact::int(100);

    /// One `Electronvolt` (`eV`), as an `Energy`.
    pub const ELECTRONVOLT: Energy<Exact> = (METRE * METRE * GRAM
        / (crate::base::SECOND * crate::base::SECOND))
        * Exact::dec(1602176634, -25);

    /// One `AstronomicUnit` (`AU`), as a `Length`.
    pub const ASTRONOMIC_UNIT: Length<Exact> = METRE * Exact::int(149597870691);

    /// One `Parsec` (`pc`), as a `Length`.
    pub const PARSEC: Length<Exact> = METRE * Exact::int(30856780000000000);

    /// One `Tonne` (`t`), as a `Mass`.
    pub const TONNE: Mass<Exact> = GRAM * Exact::int(1000000);

    /// One `UnifiedAtomicMassUnit` (`u`), as a `Mass`.
    pub const UNIFIED_ATOMIC_MASS_UNIT: Mass<Exact> = GRAM * Exact::dec(16605390666, -34);

    /// One `Grade` (`gon`), as a `PlaneAngle`.
    pub const GRADE: PlaneAngle<Exact> = RADIAN * (Exact::dec(5, -3) * Exact::PI);

    /// One `Degree` (`deg`), as a `PlaneAngle`.
    pub const DEGREE: PlaneAngle<Exact> = RADIAN * (Exact::int(1) / Exact::int(180) * Exact::PI);

    /// One `Minute` (`'`), as a `PlaneAngle`.
    pub const PLANE_ANGLE_MINUTE: PlaneAngle<Exact> =
        RADIAN * (Exact::int(1) / Exact::int(10800) * Exact::PI);

    /// One `Second` (`''`), as a `PlaneAngle`.
    pub const SECOND: PlaneAngle<Exact> = RADIAN * (Exact::int(1) / Exact::int(648000) * Exact::PI);

    /// One `Bar` (`bar`), as a `Pressure`.
    pub const BAR: Pressure<Exact> =
        (GRAM / (METRE * crate::base::SECOND * crate::base::SECOND)) * Exact::int(100000000);

    /// One `Minute` (`min`), as a `Time`.
    pub const TIME_MINUTE: Time<Exact> = crate::base::SECOND * Exact::int(60);

    /// One `Hour` (`h`), as a `Time`.
    pub const HOUR: Time<Exact> = crate::base::SECOND * Exact::int(3600);

    /// One `Day` (`d`), as a `Time`.
    pub const DAY: Time<Exact> = crate::base::SECOND * Exact::int(86400);

    /// One `TropicalYear` (`a_t`), as a `Time`.
    pub const TROPICAL_YEAR: Time<Exact> = crate::base::SECOND * Exact::dec(31556925216, -3);

    /// One `MeanJulianYear` (`a_j`), as a `Time`.
    pub const MEAN_JULIAN_YEAR: Time<Exact> = crate::base::SECOND * Exact::int(31557600);

    /// One `MeanGregorianYear` (`a_g`), as a `Time`.
    pub const MEAN_GREGORIAN_YEAR: Time<Exact> = crate::base::SECOND * Exact::int(31556952);

    /// One `Year` (`a`), as a `Time`.
    pub const YEAR: Time<Exact> = crate::base::SECOND * Exact::int(31557600);

    /// One `Week` (`wk`), as a `Time`.
    pub const WEEK: Time<Exact> = crate::base::SECOND * Exact::int(604800);

    /// One `SynodalMonth` (`mo_s`), as a `Time`.
    pub const SYNODAL_MONTH: Time<Exact> = crate::base::SECOND * Exact::dec(2551442976, -3);

    /// One `MeanJulianMonth` (`mo_j`), as a `Time`.
    pub const MEAN_JULIAN_MONTH: Time<Exact> = crate::base::SECOND * Exact::int(2629800);

    /// One `MeanGregorianMonth` (`mo_g`), as a `Time`.
    pub const MEAN_GREGORIAN_MONTH: Time<Exact> = crate::base::SECOND * Exact::int(2629746);

    /// One `Month` (`mo`), as a `Time`.
    pub const MONTH: Time<Exact> = crate::base::SECOND * Exact::int(2629800);

    /// One `Liter` (`l`), as a `Volume`.
    pub const LITER: Volume<Exact> = (METRE * METRE * METRE) * Exact::dec(1, -3);
}
