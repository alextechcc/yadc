use crate::Dim;
use yadc::Dimension;

/// The dimension of `Area`: `Qty<AREA, N>` is one.
pub const AREA: Dim = Dim {
    m: 2,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Energy`: `Qty<ENERGY, N>` is one.
pub const ENERGY: Dim = Dim {
    m: 2,
    s: -2,
    g: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Length`: `Qty<LENGTH, N>` is one.
pub const LENGTH: Dim = Dim {
    m: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Mass`: `Qty<MASS, N>` is one.
pub const MASS: Dim = Dim {
    g: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `PlaneAngle`: `Qty<PLANE_ANGLE, N>` is one.
pub const PLANE_ANGLE: Dim = Dim {
    rad: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Pressure`: `Qty<PRESSURE, N>` is one.
pub const PRESSURE: Dim = Dim {
    m: -1,
    s: -2,
    g: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Time`: `Qty<TIME, N>` is one.
pub const TIME: Dim = Dim {
    s: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Volume`: `Qty<VOLUME, N>` is one.
pub const VOLUME: Dim = Dim {
    m: 3,
    ..Dim::DIMENSIONLESS
};

/// The units of this module's quantities: each a constant of its own unit
/// type, its size and zero in its quantity's coherent unit, exact
/// (`MILE.quantity(3.0)`, `MILE.value(d)`).
pub mod units {
    use super::*;
    use crate::{Exact, UcumUnit, UcumUnitConsts};

    /// Are (`ar`), a unit of `Area`: `n` of them is `n · 1e2` coherent units.
    pub const ARE: UcumUnit<
        AREA,
        {
            UcumUnitConsts {
                size: Exact::int(100),
                zero: Exact::ZERO,
                symbol: "ar",
                name: "are",
            }
        },
    > = UcumUnit;

    /// Electronvolt (`eV`), a unit of `Energy`: `n` of them is `n · 1602176634e-25` coherent units.
    pub const ELECTRONVOLT: UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::dec(1602176634, -25),
                zero: Exact::ZERO,
                symbol: "eV",
                name: "electronvolt",
            }
        },
    > = UcumUnit;

    /// Astronomic unit (`AU`), a unit of `Length`: `n` of them is `n · 149597870691` coherent units.
    pub const ASTRONOMIC_UNIT: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(149597870691),
                zero: Exact::ZERO,
                symbol: "AU",
                name: "astronomic unit",
            }
        },
    > = UcumUnit;

    /// Parsec (`pc`), a unit of `Length`: `n` of them is `n · 3085678e10` coherent units.
    pub const PARSEC: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(30856780000000000),
                zero: Exact::ZERO,
                symbol: "pc",
                name: "parsec",
            }
        },
    > = UcumUnit;

    /// Tonne (`t`), a unit of `Mass`: `n` of them is `n · 1e6` coherent units.
    pub const TONNE: UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::int(1000000),
                zero: Exact::ZERO,
                symbol: "t",
                name: "tonne",
            }
        },
    > = UcumUnit;

    /// Unified atomic mass unit (`u`), a unit of `Mass`: `n` of them is `n · 16605390666e-34` coherent units.
    pub const UNIFIED_ATOMIC_MASS_UNIT: UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(16605390666, -34),
                zero: Exact::ZERO,
                symbol: "u",
                name: "unified atomic mass unit",
            }
        },
    > = UcumUnit;

    /// Grade (`gon`), a unit of `PlaneAngle`: `n` of them is `n · 5e-3 * core::f64::consts::PI` coherent units.
    pub const GRADE: UcumUnit<
        PLANE_ANGLE,
        {
            UcumUnitConsts {
                size: Exact::dec(5, -3) * Exact::PI,
                zero: Exact::ZERO,
                symbol: "gon",
                name: "grade",
            }
        },
    > = UcumUnit;

    /// Degree (`deg`), a unit of `PlaneAngle`: `n` of them is `n · 1 / 180 * core::f64::consts::PI` coherent units.
    pub const DEGREE: UcumUnit<
        PLANE_ANGLE,
        {
            UcumUnitConsts {
                size: Exact::int(1) / Exact::int(180) * Exact::PI,
                zero: Exact::ZERO,
                symbol: "deg",
                name: "degree",
            }
        },
    > = UcumUnit;

    /// Minute (`'`), a unit of `PlaneAngle`: `n` of them is `n · 1 / 10800 * core::f64::consts::PI` coherent units.
    pub const PLANE_ANGLE_MINUTE: UcumUnit<
        PLANE_ANGLE,
        {
            UcumUnitConsts {
                size: Exact::int(1) / Exact::int(10800) * Exact::PI,
                zero: Exact::ZERO,
                symbol: "'",
                name: "minute",
            }
        },
    > = UcumUnit;

    /// Second (`''`), a unit of `PlaneAngle`: `n` of them is `n · 1 / 648000 * core::f64::consts::PI` coherent units.
    pub const SECOND: UcumUnit<
        PLANE_ANGLE,
        {
            UcumUnitConsts {
                size: Exact::int(1) / Exact::int(648000) * Exact::PI,
                zero: Exact::ZERO,
                symbol: "''",
                name: "second",
            }
        },
    > = UcumUnit;

    /// Bar (`bar`), a unit of `Pressure`: `n` of them is `n · 1e8` coherent units.
    pub const BAR: UcumUnit<
        PRESSURE,
        {
            UcumUnitConsts {
                size: Exact::int(100000000),
                zero: Exact::ZERO,
                symbol: "bar",
                name: "bar",
            }
        },
    > = UcumUnit;

    /// Minute (`min`), a unit of `Time`: `n` of them is `n · 6e1` coherent units.
    pub const TIME_MINUTE: UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::int(60),
                zero: Exact::ZERO,
                symbol: "min",
                name: "minute",
            }
        },
    > = UcumUnit;

    /// Hour (`h`), a unit of `Time`: `n` of them is `n · 36e2` coherent units.
    pub const HOUR: UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "h",
                name: "hour",
            }
        },
    > = UcumUnit;

    /// Day (`d`), a unit of `Time`: `n` of them is `n · 864e2` coherent units.
    pub const DAY: UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::int(86400),
                zero: Exact::ZERO,
                symbol: "d",
                name: "day",
            }
        },
    > = UcumUnit;

    /// Tropical year (`a_t`), a unit of `Time`: `n` of them is `n · 31556925216e-3` coherent units.
    pub const TROPICAL_YEAR: UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::dec(31556925216, -3),
                zero: Exact::ZERO,
                symbol: "a_t",
                name: "tropical year",
            }
        },
    > = UcumUnit;

    /// Mean Julian year (`a_j`), a unit of `Time`: `n` of them is `n · 315576e2` coherent units.
    pub const MEAN_JULIAN_YEAR: UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::int(31557600),
                zero: Exact::ZERO,
                symbol: "a_j",
                name: "mean Julian year",
            }
        },
    > = UcumUnit;

    /// Mean Gregorian year (`a_g`), a unit of `Time`: `n` of them is `n · 31556952` coherent units.
    pub const MEAN_GREGORIAN_YEAR: UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::int(31556952),
                zero: Exact::ZERO,
                symbol: "a_g",
                name: "mean Gregorian year",
            }
        },
    > = UcumUnit;

    /// Year (`a`), a unit of `Time`: `n` of them is `n · 315576e2` coherent units.
    pub const YEAR: UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::int(31557600),
                zero: Exact::ZERO,
                symbol: "a",
                name: "year",
            }
        },
    > = UcumUnit;

    /// Week (`wk`), a unit of `Time`: `n` of them is `n · 6048e2` coherent units.
    pub const WEEK: UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::int(604800),
                zero: Exact::ZERO,
                symbol: "wk",
                name: "week",
            }
        },
    > = UcumUnit;

    /// Synodal month (`mo_s`), a unit of `Time`: `n` of them is `n · 2551442976e-3` coherent units.
    pub const SYNODAL_MONTH: UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::dec(2551442976, -3),
                zero: Exact::ZERO,
                symbol: "mo_s",
                name: "synodal month",
            }
        },
    > = UcumUnit;

    /// Mean Julian month (`mo_j`), a unit of `Time`: `n` of them is `n · 26298e2` coherent units.
    pub const MEAN_JULIAN_MONTH: UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::int(2629800),
                zero: Exact::ZERO,
                symbol: "mo_j",
                name: "mean Julian month",
            }
        },
    > = UcumUnit;

    /// Mean Gregorian month (`mo_g`), a unit of `Time`: `n` of them is `n · 2629746` coherent units.
    pub const MEAN_GREGORIAN_MONTH: UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::int(2629746),
                zero: Exact::ZERO,
                symbol: "mo_g",
                name: "mean Gregorian month",
            }
        },
    > = UcumUnit;

    /// Month (`mo`), a unit of `Time`: `n` of them is `n · 26298e2` coherent units.
    pub const MONTH: UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::int(2629800),
                zero: Exact::ZERO,
                symbol: "mo",
                name: "month",
            }
        },
    > = UcumUnit;

    /// Liter (`l`), a unit of `Volume`: `n` of them is `n · 1e-3` coherent units.
    pub const LITER: UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "l",
                name: "liter",
            }
        },
    > = UcumUnit;
}
