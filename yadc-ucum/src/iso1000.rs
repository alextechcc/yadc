//! The quantities of `iso1000` and their units.

#[allow(unused_imports)]
use crate::*;

// `Area` is `crate::Area`.

/// Are (`ar`), a unit of `Area`: `n` of them is `n · 1e2` base units.
pub const ARE: Qty<
    { UcumUnit::scaled("ar", Exact::int(100), UcumUnit::ONE.times(METER.unit(), 2)) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Energy` is `crate::Energy`.

/// Electronvolt (`eV`), a unit of `Energy`: `n` of them is `n · 1602176634e-25` base units.
pub const ELECTRONVOLT: Qty<
    {
        UcumUnit::scaled(
            "eV",
            Exact::dec(1602176634, -25),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Length` is `crate::Length`.

/// Astronomic unit (`AU`), a unit of `Length`: `n` of them is `n · 149597870691` base units.
pub const ASTRONOMIC_UNIT: Qty<
    { UcumUnit::scaled("AU", Exact::int(149597870691), METER.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Parsec (`pc`), a unit of `Length`: `n` of them is `n · 3085678e10` base units.
pub const PARSEC: Qty<
    { UcumUnit::scaled("pc", Exact::int(30856780000000000), METER.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Mass` is `crate::Mass`.

/// Tonne (`t`), a unit of `Mass`: `n` of them is `n · 1e6` base units.
pub const TONNE: Qty<{ UcumUnit::scaled("t", Exact::int(1000000), GRAM.unit()) }, Exact, NOT_BASE> =
    Qty::ONE;

/// Unified atomic mass unit (`u`), a unit of `Mass`: `n` of them is `n · 16605390666e-34` base units.
pub const UNIFIED_ATOMIC_MASS_UNIT: Qty<
    { UcumUnit::scaled("u", Exact::dec(16605390666, -34), GRAM.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `PlaneAngle` is `crate::PlaneAngle`.

/// Grade (`gon`), a unit of `PlaneAngle`: `n` of them is `n · 5e-3 * core::f64::consts::PI` base units.
pub const GRADE: Qty<
    { UcumUnit::scaled("gon", Exact::dec(5, -3) * Exact::PI, RADIAN.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Degree (`deg`), a unit of `PlaneAngle`: `n` of them is `n · 1 / 180 * core::f64::consts::PI` base units.
pub const DEGREE: Qty<
    {
        UcumUnit::scaled(
            "deg",
            Exact::int(1) / Exact::int(180) * Exact::PI,
            RADIAN.unit(),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Minute (`'`), a unit of `PlaneAngle`: `n` of them is `n · 1 / 10800 * core::f64::consts::PI` base units.
pub const PLANE_ANGLE_MINUTE: Qty<
    {
        UcumUnit::scaled(
            "'",
            Exact::int(1) / Exact::int(10800) * Exact::PI,
            RADIAN.unit(),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Second (`''`), a unit of `PlaneAngle`: `n` of them is `n · 1 / 648000 * core::f64::consts::PI` base units.
pub const PLANE_ANGLE_SECOND: Qty<
    {
        UcumUnit::scaled(
            "''",
            Exact::int(1) / Exact::int(648000) * Exact::PI,
            RADIAN.unit(),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Pressure` is `crate::Pressure`.

/// Bar (`bar`), a unit of `Pressure`: `n` of them is `n · 1e8` base units.
pub const BAR: Qty<
    {
        UcumUnit::scaled(
            "bar",
            Exact::int(100000000),
            UcumUnit::ONE
                .times(METER.unit(), -1)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Time` is `crate::Time`.

/// Minute (`min`), a unit of `Time`: `n` of them is `n · 6e1` base units.
pub const TIME_MINUTE: Qty<
    { UcumUnit::scaled("min", Exact::int(60), SECOND.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Hour (`h`), a unit of `Time`: `n` of them is `n · 36e2` base units.
pub const HOUR: Qty<{ UcumUnit::scaled("h", Exact::int(3600), SECOND.unit()) }, Exact, NOT_BASE> =
    Qty::ONE;

/// Day (`d`), a unit of `Time`: `n` of them is `n · 864e2` base units.
pub const DAY: Qty<{ UcumUnit::scaled("d", Exact::int(86400), SECOND.unit()) }, Exact, NOT_BASE> =
    Qty::ONE;

/// Tropical year (`a_t`), a unit of `Time`: `n` of them is `n · 31556925216e-3` base units.
pub const TROPICAL_YEAR: Qty<
    { UcumUnit::scaled("a_t", Exact::dec(31556925216, -3), SECOND.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Mean Julian year (`a_j`), a unit of `Time`: `n` of them is `n · 315576e2` base units.
pub const MEAN_JULIAN_YEAR: Qty<
    { UcumUnit::scaled("a_j", Exact::int(31557600), SECOND.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Mean Gregorian year (`a_g`), a unit of `Time`: `n` of them is `n · 31556952` base units.
pub const MEAN_GREGORIAN_YEAR: Qty<
    { UcumUnit::scaled("a_g", Exact::int(31556952), SECOND.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Year (`a`), a unit of `Time`: `n` of them is `n · 315576e2` base units.
pub const YEAR: Qty<
    { UcumUnit::scaled("a", Exact::int(31557600), SECOND.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Week (`wk`), a unit of `Time`: `n` of them is `n · 6048e2` base units.
pub const WEEK: Qty<
    { UcumUnit::scaled("wk", Exact::int(604800), SECOND.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Synodal month (`mo_s`), a unit of `Time`: `n` of them is `n · 2551442976e-3` base units.
pub const SYNODAL_MONTH: Qty<
    { UcumUnit::scaled("mo_s", Exact::dec(2551442976, -3), SECOND.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Mean Julian month (`mo_j`), a unit of `Time`: `n` of them is `n · 26298e2` base units.
pub const MEAN_JULIAN_MONTH: Qty<
    { UcumUnit::scaled("mo_j", Exact::int(2629800), SECOND.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Mean Gregorian month (`mo_g`), a unit of `Time`: `n` of them is `n · 2629746` base units.
pub const MEAN_GREGORIAN_MONTH: Qty<
    { UcumUnit::scaled("mo_g", Exact::int(2629746), SECOND.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Month (`mo`), a unit of `Time`: `n` of them is `n · 26298e2` base units.
pub const MONTH: Qty<
    { UcumUnit::scaled("mo", Exact::int(2629800), SECOND.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Volume` is `crate::Volume`.

/// Liter (`l`), a unit of `Volume`: `n` of them is `n · 1e-3` base units.
pub const LITER: Qty<
    { UcumUnit::scaled("l", Exact::dec(1, -3), UcumUnit::ONE.times(METER.unit(), 3)) },
    Exact,
    NOT_BASE,
> = Qty::ONE;
