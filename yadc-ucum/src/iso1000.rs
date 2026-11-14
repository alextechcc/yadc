use crate::{Dim, Qty};
use yadc::Dimension;

/// The dimension of `Area`.
pub const AREA: Dim = Dim {
    m: 2,
    ..Dim::DIMENSIONLESS
};
pub type Area<N> = Qty<AREA, N>;

/// The dimension of `Energy`.
pub const ENERGY: Dim = Dim {
    m: 2,
    s: -2,
    g: 1,
    ..Dim::DIMENSIONLESS
};
pub type Energy<N> = Qty<ENERGY, N>;

/// The dimension of `Length`.
pub const LENGTH: Dim = Dim {
    m: 1,
    ..Dim::DIMENSIONLESS
};
pub type Length<N> = Qty<LENGTH, N>;

/// The dimension of `Mass`.
pub const MASS: Dim = Dim {
    g: 1,
    ..Dim::DIMENSIONLESS
};
pub type Mass<N> = Qty<MASS, N>;

/// The dimension of `PlaneAngle`.
pub const PLANE_ANGLE: Dim = Dim {
    rad: 1,
    ..Dim::DIMENSIONLESS
};
pub type PlaneAngle<N> = Qty<PLANE_ANGLE, N>;

/// The dimension of `Pressure`.
pub const PRESSURE: Dim = Dim {
    m: -1,
    s: -2,
    g: 1,
    ..Dim::DIMENSIONLESS
};
pub type Pressure<N> = Qty<PRESSURE, N>;

/// The dimension of `Time`.
pub const TIME: Dim = Dim {
    s: 1,
    ..Dim::DIMENSIONLESS
};
pub type Time<N> = Qty<TIME, N>;

/// The dimension of `Volume`.
pub const VOLUME: Dim = Dim {
    m: 3,
    ..Dim::DIMENSIONLESS
};
pub type Volume<N> = Qty<VOLUME, N>;

/// The units of this module's quantities: a type each, its size and zero
/// in its quantity's coherent unit, exact (`Mile {}.quantity(3.0_f64)`).
pub mod units {
    use super::*;
    use crate::{Exact, UcumUnit, UcumUnitConsts};

    /// Are (`ar`), a unit of `Area`: `n` of them is `n · 1e2` coherent units.
    pub type Are = UcumUnit<
        AREA,
        {
            UcumUnitConsts {
                size: Exact::int(100),
                zero: Exact::ZERO,
                symbol: "ar",
                name: "are",
            }
        },
    >;

    /// Electronvolt (`eV`), a unit of `Energy`: `n` of them is `n · 1602176634e-25` coherent units.
    pub type Electronvolt = UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::dec(1602176634, -25),
                zero: Exact::ZERO,
                symbol: "eV",
                name: "electronvolt",
            }
        },
    >;

    /// Astronomic unit (`AU`), a unit of `Length`: `n` of them is `n · 149597870691` coherent units.
    pub type AstronomicUnit = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(149597870691),
                zero: Exact::ZERO,
                symbol: "AU",
                name: "astronomic unit",
            }
        },
    >;

    /// Parsec (`pc`), a unit of `Length`: `n` of them is `n · 3085678e10` coherent units.
    pub type Parsec = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(30856780000000000),
                zero: Exact::ZERO,
                symbol: "pc",
                name: "parsec",
            }
        },
    >;

    /// Tonne (`t`), a unit of `Mass`: `n` of them is `n · 1e6` coherent units.
    pub type Tonne = UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::int(1000000),
                zero: Exact::ZERO,
                symbol: "t",
                name: "tonne",
            }
        },
    >;

    /// Unified atomic mass unit (`u`), a unit of `Mass`: `n` of them is `n · 16605390666e-34` coherent units.
    pub type UnifiedAtomicMassUnit = UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(16605390666, -34),
                zero: Exact::ZERO,
                symbol: "u",
                name: "unified atomic mass unit",
            }
        },
    >;

    /// Grade (`gon`), a unit of `PlaneAngle`: `n` of them is `n · 5e-3 * core::f64::consts::PI` coherent units.
    pub type Grade = UcumUnit<
        PLANE_ANGLE,
        {
            UcumUnitConsts {
                size: Exact::dec(5, -3) * Exact::PI,
                zero: Exact::ZERO,
                symbol: "gon",
                name: "grade",
            }
        },
    >;

    /// Degree (`deg`), a unit of `PlaneAngle`: `n` of them is `n · 1 / 180 * core::f64::consts::PI` coherent units.
    pub type Degree = UcumUnit<
        PLANE_ANGLE,
        {
            UcumUnitConsts {
                size: Exact::int(1) / Exact::int(180) * Exact::PI,
                zero: Exact::ZERO,
                symbol: "deg",
                name: "degree",
            }
        },
    >;

    /// Minute (`'`), a unit of `PlaneAngle`: `n` of them is `n · 1 / 10800 * core::f64::consts::PI` coherent units.
    pub type PlaneAngleMinute = UcumUnit<
        PLANE_ANGLE,
        {
            UcumUnitConsts {
                size: Exact::int(1) / Exact::int(10800) * Exact::PI,
                zero: Exact::ZERO,
                symbol: "'",
                name: "minute",
            }
        },
    >;

    /// Second (`''`), a unit of `PlaneAngle`: `n` of them is `n · 1 / 648000 * core::f64::consts::PI` coherent units.
    pub type Second = UcumUnit<
        PLANE_ANGLE,
        {
            UcumUnitConsts {
                size: Exact::int(1) / Exact::int(648000) * Exact::PI,
                zero: Exact::ZERO,
                symbol: "''",
                name: "second",
            }
        },
    >;

    /// Bar (`bar`), a unit of `Pressure`: `n` of them is `n · 1e8` coherent units.
    pub type Bar = UcumUnit<
        PRESSURE,
        {
            UcumUnitConsts {
                size: Exact::int(100000000),
                zero: Exact::ZERO,
                symbol: "bar",
                name: "bar",
            }
        },
    >;

    /// Minute (`min`), a unit of `Time`: `n` of them is `n · 6e1` coherent units.
    pub type TimeMinute = UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::int(60),
                zero: Exact::ZERO,
                symbol: "min",
                name: "minute",
            }
        },
    >;

    /// Hour (`h`), a unit of `Time`: `n` of them is `n · 36e2` coherent units.
    pub type Hour = UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "h",
                name: "hour",
            }
        },
    >;

    /// Day (`d`), a unit of `Time`: `n` of them is `n · 864e2` coherent units.
    pub type Day = UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::int(86400),
                zero: Exact::ZERO,
                symbol: "d",
                name: "day",
            }
        },
    >;

    /// Tropical year (`a_t`), a unit of `Time`: `n` of them is `n · 31556925216e-3` coherent units.
    pub type TropicalYear = UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::dec(31556925216, -3),
                zero: Exact::ZERO,
                symbol: "a_t",
                name: "tropical year",
            }
        },
    >;

    /// Mean Julian year (`a_j`), a unit of `Time`: `n` of them is `n · 315576e2` coherent units.
    pub type MeanJulianYear = UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::int(31557600),
                zero: Exact::ZERO,
                symbol: "a_j",
                name: "mean Julian year",
            }
        },
    >;

    /// Mean Gregorian year (`a_g`), a unit of `Time`: `n` of them is `n · 31556952` coherent units.
    pub type MeanGregorianYear = UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::int(31556952),
                zero: Exact::ZERO,
                symbol: "a_g",
                name: "mean Gregorian year",
            }
        },
    >;

    /// Year (`a`), a unit of `Time`: `n` of them is `n · 315576e2` coherent units.
    pub type Year = UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::int(31557600),
                zero: Exact::ZERO,
                symbol: "a",
                name: "year",
            }
        },
    >;

    /// Week (`wk`), a unit of `Time`: `n` of them is `n · 6048e2` coherent units.
    pub type Week = UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::int(604800),
                zero: Exact::ZERO,
                symbol: "wk",
                name: "week",
            }
        },
    >;

    /// Synodal month (`mo_s`), a unit of `Time`: `n` of them is `n · 2551442976e-3` coherent units.
    pub type SynodalMonth = UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::dec(2551442976, -3),
                zero: Exact::ZERO,
                symbol: "mo_s",
                name: "synodal month",
            }
        },
    >;

    /// Mean Julian month (`mo_j`), a unit of `Time`: `n` of them is `n · 26298e2` coherent units.
    pub type MeanJulianMonth = UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::int(2629800),
                zero: Exact::ZERO,
                symbol: "mo_j",
                name: "mean Julian month",
            }
        },
    >;

    /// Mean Gregorian month (`mo_g`), a unit of `Time`: `n` of them is `n · 2629746` coherent units.
    pub type MeanGregorianMonth = UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::int(2629746),
                zero: Exact::ZERO,
                symbol: "mo_g",
                name: "mean Gregorian month",
            }
        },
    >;

    /// Month (`mo`), a unit of `Time`: `n` of them is `n · 26298e2` coherent units.
    pub type Month = UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::int(2629800),
                zero: Exact::ZERO,
                symbol: "mo",
                name: "month",
            }
        },
    >;

    /// Liter (`l`), a unit of `Volume`: `n` of them is `n · 1e-3` coherent units.
    pub type Liter = UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "l",
                name: "liter",
            }
        },
    >;
}
