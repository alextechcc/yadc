use crate::{Dim, Exact, Qty};
use consts::*;
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
/// `const MPH: Velocity<f64> = (MILE / HOUR).into();`, then `MPH(60.0)`.
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

/// The units of `Area`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Area<N>, number = N)]
#[unit(for<N> where Area<N>: [const] yadc::Unit<Area<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum AreaUnit {
    #[default]
    #[unit("ar", "are", by = Area::<N>::from(ARE))]
    Are,
}

/// The units of `Energy`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Energy<N>, number = N)]
#[unit(for<N> where Energy<N>: [const] yadc::Unit<Energy<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum EnergyUnit {
    #[default]
    #[unit("eV", "electronvolt", by = Energy::<N>::from(ELECTRONVOLT))]
    Electronvolt,
}

/// The units of `Length`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length<N>, number = N)]
#[unit(for<N> where Length<N>: [const] yadc::Unit<Length<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum LengthUnit {
    #[default]
    #[unit("AU", "astronomic unit", by = Length::<N>::from(ASTRONOMIC_UNIT))]
    AstronomicUnit,
    #[unit("pc", "parsec", by = Length::<N>::from(PARSEC))]
    Parsec,
}

/// The units of `Mass`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Mass<N>, number = N)]
#[unit(for<N> where Mass<N>: [const] yadc::Unit<Mass<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MassUnit {
    #[default]
    #[unit("t", "tonne", by = Mass::<N>::from(TONNE))]
    Tonne,
    #[unit("u", "unified atomic mass unit", by = Mass::<N>::from(UNIFIED_ATOMIC_MASS_UNIT))]
    UnifiedAtomicMassUnit,
}

/// The units of `PlaneAngle`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = PlaneAngle<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, PlaneAngle<N>: [const] yadc::Unit<PlaneAngle<N>, N> + [const] core::marker::Destruct)]
pub enum PlaneAngleUnit {
    #[default]
    #[unit("gon", "grade", by = PlaneAngle::<N>::from(GRADE))]
    Grade,
    #[unit("deg", "degree", by = PlaneAngle::<N>::from(DEGREE))]
    Degree,
    #[unit("'", "minute", by = PlaneAngle::<N>::from(PLANE_ANGLE_MINUTE))]
    Minute,
    #[unit("''", "second", by = PlaneAngle::<N>::from(SECOND))]
    Second,
}

/// The units of `Pressure`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Pressure<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Pressure<N>: [const] yadc::Unit<Pressure<N>, N> + [const] core::marker::Destruct)]
pub enum PressureUnit {
    #[default]
    #[unit("bar", "bar", by = Pressure::<N>::from(BAR))]
    Bar,
}

/// The units of `Time`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Time<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Time<N>: [const] yadc::Unit<Time<N>, N> + [const] core::marker::Destruct)]
pub enum TimeUnit {
    #[default]
    #[unit("min", "minute", by = Time::<N>::from(TIME_MINUTE))]
    Minute,
    #[unit("h", "hour", by = Time::<N>::from(HOUR))]
    Hour,
    #[unit("d", "day", by = Time::<N>::from(DAY))]
    Day,
    #[unit("a_t", "tropical year", by = Time::<N>::from(TROPICAL_YEAR))]
    TropicalYear,
    #[unit("a_j", "mean Julian year", by = Time::<N>::from(MEAN_JULIAN_YEAR))]
    MeanJulianYear,
    #[unit("a_g", "mean Gregorian year", by = Time::<N>::from(MEAN_GREGORIAN_YEAR))]
    MeanGregorianYear,
    #[unit("a", "year", by = Time::<N>::from(YEAR))]
    Year,
    #[unit("wk", "week", by = Time::<N>::from(WEEK))]
    Week,
    #[unit("mo_s", "synodal month", by = Time::<N>::from(SYNODAL_MONTH))]
    SynodalMonth,
    #[unit("mo_j", "mean Julian month", by = Time::<N>::from(MEAN_JULIAN_MONTH))]
    MeanJulianMonth,
    #[unit("mo_g", "mean Gregorian month", by = Time::<N>::from(MEAN_GREGORIAN_MONTH))]
    MeanGregorianMonth,
    #[unit("mo", "month", by = Time::<N>::from(MONTH))]
    Month,
}

/// The units of `Volume`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Volume<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Volume<N>: [const] yadc::Unit<Volume<N>, N> + [const] core::marker::Destruct)]
pub enum VolumeUnit {
    #[default]
    #[unit("l", "liter", by = Volume::<N>::from(LITER))]
    Liter,
}
