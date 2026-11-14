//! The quantities of `iso1000` and their units.

#[allow(unused_imports)]
use crate::*;

// `Area` is `crate::Area`.

/// Are (`ar`), a unit of `Area`: `n` of them is `n · 1e2` base units.
pub const ARE: Unit<{ UcumUnit::scaled("ar", Exact::int(100), METER_SQUARED.unit()) }> = Unit::ONE;

// `Energy` is `crate::Energy`.

/// Electronvolt (`eV`), a unit of `Energy`: `n` of them is `n · 1602176634e-25` base units.
pub const ELECTRONVOLT: Unit<
    {
        UcumUnit::scaled(
            "eV",
            Exact::dec(1602176634, -25),
            METER_SQUARED_GRAM_PER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

// `Length` is `crate::Length`.

/// Astronomic unit (`AU`), a unit of `Length`: `n` of them is `n · 149597870691` base units.
pub const ASTRONOMIC_UNIT: Unit<
    { UcumUnit::scaled("AU", Exact::int(149597870691), METER.unit()) },
> = Unit::ONE;

/// Parsec (`pc`), a unit of `Length`: `n` of them is `n · 3085678e10` base units.
pub const PARSEC: Unit<{ UcumUnit::scaled("pc", Exact::int(30856780000000000), METER.unit()) }> =
    Unit::ONE;

// `Mass` is `crate::Mass`.

/// Tonne (`t`), a unit of `Mass`: `n` of them is `n · 1e6` base units.
pub const TONNE: Unit<{ UcumUnit::scaled("t", Exact::int(1000000), GRAM.unit()) }> = Unit::ONE;

/// Unified atomic mass unit (`u`), a unit of `Mass`: `n` of them is `n · 16605390666e-34` base units.
pub const UNIFIED_ATOMIC_MASS_UNIT: Unit<
    { UcumUnit::scaled("u", Exact::dec(16605390666, -34), GRAM.unit()) },
> = Unit::ONE;

// `PlaneAngle` is `crate::PlaneAngle`.

/// Grade (`gon`), a unit of `PlaneAngle`: `n` of them is `n · 5e-3 * core::f64::consts::PI` base units.
pub const GRADE: Unit<{ UcumUnit::scaled("gon", Exact::dec(5, -3) * Exact::PI, RADIAN.unit()) }> =
    Unit::ONE;

/// Degree (`deg`), a unit of `PlaneAngle`: `n` of them is `n · 1 / 180 * core::f64::consts::PI` base units.
pub const DEGREE: Unit<
    {
        UcumUnit::scaled(
            "deg",
            Exact::int(1) / Exact::int(180) * Exact::PI,
            RADIAN.unit(),
        )
    },
> = Unit::ONE;

/// Minute (`'`), a unit of `PlaneAngle`: `n` of them is `n · 1 / 10800 * core::f64::consts::PI` base units.
pub const PLANE_ANGLE_MINUTE: Unit<
    {
        UcumUnit::scaled(
            "'",
            Exact::int(1) / Exact::int(10800) * Exact::PI,
            RADIAN.unit(),
        )
    },
> = Unit::ONE;

/// Second (`''`), a unit of `PlaneAngle`: `n` of them is `n · 1 / 648000 * core::f64::consts::PI` base units.
pub const PLANE_ANGLE_SECOND: Unit<
    {
        UcumUnit::scaled(
            "''",
            Exact::int(1) / Exact::int(648000) * Exact::PI,
            RADIAN.unit(),
        )
    },
> = Unit::ONE;

// `Pressure` is `crate::Pressure`.

/// Bar (`bar`), a unit of `Pressure`: `n` of them is `n · 1e8` base units.
pub const BAR: Unit<
    {
        UcumUnit::scaled(
            "bar",
            Exact::int(100000000),
            GRAM_PER_METER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

// `Time` is `crate::Time`.

/// Minute (`min`), a unit of `Time`: `n` of them is `n · 6e1` base units.
pub const TIME_MINUTE: Unit<{ UcumUnit::scaled("min", Exact::int(60), SECOND.unit()) }> = Unit::ONE;

/// Hour (`h`), a unit of `Time`: `n` of them is `n · 36e2` base units.
pub const HOUR: Unit<{ UcumUnit::scaled("h", Exact::int(3600), SECOND.unit()) }> = Unit::ONE;

/// Day (`d`), a unit of `Time`: `n` of them is `n · 864e2` base units.
pub const DAY: Unit<{ UcumUnit::scaled("d", Exact::int(86400), SECOND.unit()) }> = Unit::ONE;

/// Tropical year (`a_t`), a unit of `Time`: `n` of them is `n · 31556925216e-3` base units.
pub const TROPICAL_YEAR: Unit<
    { UcumUnit::scaled("a_t", Exact::dec(31556925216, -3), SECOND.unit()) },
> = Unit::ONE;

/// Mean Julian year (`a_j`), a unit of `Time`: `n` of them is `n · 315576e2` base units.
pub const MEAN_JULIAN_YEAR: Unit<{ UcumUnit::scaled("a_j", Exact::int(31557600), SECOND.unit()) }> =
    Unit::ONE;

/// Mean Gregorian year (`a_g`), a unit of `Time`: `n` of them is `n · 31556952` base units.
pub const MEAN_GREGORIAN_YEAR: Unit<
    { UcumUnit::scaled("a_g", Exact::int(31556952), SECOND.unit()) },
> = Unit::ONE;

/// Year (`a`), a unit of `Time`: `n` of them is `n · 315576e2` base units.
pub const YEAR: Unit<{ UcumUnit::scaled("a", Exact::int(31557600), SECOND.unit()) }> = Unit::ONE;

/// Week (`wk`), a unit of `Time`: `n` of them is `n · 6048e2` base units.
pub const WEEK: Unit<{ UcumUnit::scaled("wk", Exact::int(604800), SECOND.unit()) }> = Unit::ONE;

/// Synodal month (`mo_s`), a unit of `Time`: `n` of them is `n · 2551442976e-3` base units.
pub const SYNODAL_MONTH: Unit<
    { UcumUnit::scaled("mo_s", Exact::dec(2551442976, -3), SECOND.unit()) },
> = Unit::ONE;

/// Mean Julian month (`mo_j`), a unit of `Time`: `n` of them is `n · 26298e2` base units.
pub const MEAN_JULIAN_MONTH: Unit<
    { UcumUnit::scaled("mo_j", Exact::int(2629800), SECOND.unit()) },
> = Unit::ONE;

/// Mean Gregorian month (`mo_g`), a unit of `Time`: `n` of them is `n · 2629746` base units.
pub const MEAN_GREGORIAN_MONTH: Unit<
    { UcumUnit::scaled("mo_g", Exact::int(2629746), SECOND.unit()) },
> = Unit::ONE;

/// Month (`mo`), a unit of `Time`: `n` of them is `n · 26298e2` base units.
pub const MONTH: Unit<{ UcumUnit::scaled("mo", Exact::int(2629800), SECOND.unit()) }> = Unit::ONE;

// `Volume` is `crate::Volume`.

/// Liter (`l`), a unit of `Volume`: `n` of them is `n · 1e-3` base units.
pub const LITER: Unit<{ UcumUnit::scaled("l", Exact::dec(1, -3), METER_CUBED.unit()) }> = Unit::ONE;
