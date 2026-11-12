//! The units of `iso1000`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::iso1000::consts::*;
use crate::iso1000::*;
use core::marker::Destruct;
use yadc::{Unit, num_traits::Num};

/// The units of `Area`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Area<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Area<N>: [const] Unit<Area<N>, N> + [const] Destruct)]
pub enum AreaUnit {
    #[default]
    #[unit("ar", "are", by = Area::<N>::from(ARE))]
    Are,
}

/// The units of `Energy`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Energy<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Energy<N>: [const] Unit<Energy<N>, N> + [const] Destruct)]
pub enum EnergyUnit {
    #[default]
    #[unit("eV", "electronvolt", by = Energy::<N>::from(ELECTRONVOLT))]
    Electronvolt,
}

/// The units of `Length`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Length<N>: [const] Unit<Length<N>, N> + [const] Destruct)]
pub enum LengthUnit {
    #[default]
    #[unit("AU", "astronomic unit", by = Length::<N>::from(ASTRONOMIC_UNIT))]
    AstronomicUnit,
    #[unit("pc", "parsec", by = Length::<N>::from(PARSEC))]
    Parsec,
}

/// The units of `Mass`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Mass<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Mass<N>: [const] Unit<Mass<N>, N> + [const] Destruct)]
pub enum MassUnit {
    #[default]
    #[unit("t", "tonne", by = Mass::<N>::from(TONNE))]
    Tonne,
    #[unit("u", "unified atomic mass unit", by = Mass::<N>::from(UNIFIED_ATOMIC_MASS_UNIT))]
    UnifiedAtomicMassUnit,
}

/// The units of `PlaneAngle`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = PlaneAngle<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where PlaneAngle<N>: [const] Unit<PlaneAngle<N>, N> + [const] Destruct)]
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

/// The units of `Pressure`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Pressure<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Pressure<N>: [const] Unit<Pressure<N>, N> + [const] Destruct)]
pub enum PressureUnit {
    #[default]
    #[unit("bar", "bar", by = Pressure::<N>::from(BAR))]
    Bar,
}

/// The units of `Time`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Time<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Time<N>: [const] Unit<Time<N>, N> + [const] Destruct)]
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

/// The units of `Volume`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Volume<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Volume<N>: [const] Unit<Volume<N>, N> + [const] Destruct)]
pub enum VolumeUnit {
    #[default]
    #[unit("l", "liter", by = Volume::<N>::from(LITER))]
    Liter,
}
