use crate::Dim;
use yadc::Dimension;

/// The dimension of `ActionArea`: `Qty<ACTION_AREA, N>` is one.
pub const ACTION_AREA: Dim = Dim {
    m: 2,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `ElectricConductance`: `Qty<ELECTRIC_CONDUCTANCE, N>` is one.
pub const ELECTRIC_CONDUCTANCE: Dim = Dim {
    m: -2,
    s: 1,
    g: -1,
    C: 2,
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

/// The dimension of `MassFraction`: `Qty<MASS_FRACTION, N>` is one.
pub const MASS_FRACTION: Dim = Dim::DIMENSIONLESS;

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

/// The dimension of `SolidAngle`: `Qty<SOLID_ANGLE, N>` is one.
pub const SOLID_ANGLE: Dim = Dim {
    rad: 2,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Turbidity`: `Qty<TURBIDITY, N>` is one.
pub const TURBIDITY: Dim = Dim::DIMENSIONLESS;

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

    /// Barn (`b`), a unit of `ActionArea`: `n` of them is `n · 1e-28` coherent units.
    pub const BARN: UcumUnit<
        ACTION_AREA,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -28),
                zero: Exact::ZERO,
                symbol: "b",
                name: "barn",
            }
        },
    > = UcumUnit;

    /// Mho (`mho`), a unit of `ElectricConductance`: `n` of them is `n · 1e-3` coherent units.
    pub const MHO: UcumUnit<
        ELECTRIC_CONDUCTANCE,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "mho",
                name: "mho",
            }
        },
    > = UcumUnit;

    /// Line (`[lne]`), a unit of `Length`: `n` of them is `n · 127 / 60000` coherent units.
    pub const LINE: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(127) / Exact::int(60000),
                zero: Exact::ZERO,
                symbol: "[lne]",
                name: "line",
            }
        },
    > = UcumUnit;

    /// Point (`[pnt]`), a unit of `Length`: `n` of them is `n · 127 / 360000` coherent units.
    pub const POINT: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(127) / Exact::int(360000),
                zero: Exact::ZERO,
                symbol: "[pnt]",
                name: "point",
            }
        },
    > = UcumUnit;

    /// Pica (`[pca]`), a unit of `Length`: `n` of them is `n · 127 / 30000` coherent units.
    pub const PICA: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(127) / Exact::int(30000),
                zero: Exact::ZERO,
                symbol: "[pca]",
                name: "pica",
            }
        },
    > = UcumUnit;

    /// Printer's point (`[pnt_pr]`), a unit of `Length`: `n` of them is `n · 3514598e-10` coherent units.
    pub const PRINTER_S_POINT: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(3514598, -10),
                zero: Exact::ZERO,
                symbol: "[pnt_pr]",
                name: "Printer's point",
            }
        },
    > = UcumUnit;

    /// Printer's pica (`[pca_pr]`), a unit of `Length`: `n` of them is `n · 42175176e-10` coherent units.
    pub const PRINTER_S_PICA: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(42175176, -10),
                zero: Exact::ZERO,
                symbol: "[pca_pr]",
                name: "Printer's pica",
            }
        },
    > = UcumUnit;

    /// French foot (`[pied]`), a unit of `Length`: `n` of them is `n · 3248e-4` coherent units.
    pub const FRENCH_FOOT: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(3248, -4),
                zero: Exact::ZERO,
                symbol: "[pied]",
                name: "French foot",
            }
        },
    > = UcumUnit;

    /// French inch (`[pouce]`), a unit of `Length`: `n` of them is `n · 203 / 7500` coherent units.
    pub const FRENCH_INCH: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(203) / Exact::int(7500),
                zero: Exact::ZERO,
                symbol: "[pouce]",
                name: "French inch",
            }
        },
    > = UcumUnit;

    /// French line (`[ligne]`), a unit of `Length`: `n` of them is `n · 203 / 90000` coherent units.
    pub const FRENCH_LINE: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(203) / Exact::int(90000),
                zero: Exact::ZERO,
                symbol: "[ligne]",
                name: "French line",
            }
        },
    > = UcumUnit;

    /// Didot's point (`[didot]`), a unit of `Length`: `n` of them is `n · 203 / 540000` coherent units.
    pub const DIDOT_S_POINT: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(203) / Exact::int(540000),
                zero: Exact::ZERO,
                symbol: "[didot]",
                name: "Didot's point",
            }
        },
    > = UcumUnit;

    /// Didot's pica (`[cicero]`), a unit of `Length`: `n` of them is `n · 203 / 45000` coherent units.
    pub const DIDOT_S_PICA: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(203) / Exact::int(45000),
                zero: Exact::ZERO,
                symbol: "[cicero]",
                name: "Didot's pica",
            }
        },
    > = UcumUnit;

    /// Ångström (`Ao`), a unit of `Length`: `n` of them is `n · 1e-10` coherent units.
    pub const ANGSTROM: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -10),
                zero: Exact::ZERO,
                symbol: "Ao",
                name: "Ångström",
            }
        },
    > = UcumUnit;

    /// Smoot (`[smoot]`), a unit of `Length`: `n` of them is `n · 17018e-4` coherent units.
    pub const SMOOT: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(17018, -4),
                zero: Exact::ZERO,
                symbol: "[smoot]",
                name: "Smoot",
            }
        },
    > = UcumUnit;

    /// Metric carat (`[car_m]`), a unit of `Mass`: `n` of them is `n · 2e-1` coherent units.
    pub const METRIC_CARAT: UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(2, -1),
                zero: Exact::ZERO,
                symbol: "[car_m]",
                name: "metric carat",
            }
        },
    > = UcumUnit;

    /// Carat of gold alloys (`[car_Au]`), a unit of `MassFraction`: `n` of them is `n · 1 / 24` coherent units.
    pub const CARAT_OF_GOLD_ALLOYS: UcumUnit<
        MASS_FRACTION,
        {
            UcumUnitConsts {
                size: Exact::int(1) / Exact::int(24),
                zero: Exact::ZERO,
                symbol: "[car_Au]",
                name: "carat of gold alloys",
            }
        },
    > = UcumUnit;

    /// Circle (`circ`), a unit of `PlaneAngle`: `n` of them is `n · 2 * core::f64::consts::PI` coherent units.
    pub const CIRCLE: UcumUnit<
        PLANE_ANGLE,
        {
            UcumUnitConsts {
                size: Exact::int(2) * Exact::PI,
                zero: Exact::ZERO,
                symbol: "circ",
                name: "circle",
            }
        },
    > = UcumUnit;

    /// Technical atmosphere (`att`), a unit of `Pressure`: `n` of them is `n · 980665e2` coherent units.
    pub const TECHNICAL_ATMOSPHERE: UcumUnit<
        PRESSURE,
        {
            UcumUnitConsts {
                size: Exact::int(98066500),
                zero: Exact::ZERO,
                symbol: "att",
                name: "technical atmosphere",
            }
        },
    > = UcumUnit;

    /// Pound per square inch (`[psi]`), a unit of `Pressure`: `n` of them is `n · 8896443230521 / 1290320` coherent units.
    pub const POUND_PER_SQUARE_INCH: UcumUnit<
        PRESSURE,
        {
            UcumUnitConsts {
                size: Exact::int(8896443230521) / Exact::int(1290320),
                zero: Exact::ZERO,
                symbol: "[psi]",
                name: "pound per square inch",
            }
        },
    > = UcumUnit;

    /// Sphere (`sph`), a unit of `SolidAngle`: `n` of them is `n · 4 * core::f64::consts::PI` coherent units.
    pub const SPHERE: UcumUnit<
        SOLID_ANGLE,
        {
            UcumUnitConsts {
                size: Exact::int(4) * Exact::PI,
                zero: Exact::ZERO,
                symbol: "sph",
                name: "sphere",
            }
        },
    > = UcumUnit;

    /// Nephelometric Turbidity Unit (`[NTU]`), a unit of `Turbidity`: `n` of them is `n · 1` coherent units.
    pub const NEPHELOMETRIC_TURBIDITY_UNIT: UcumUnit<
        TURBIDITY,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "[NTU]",
                name: "Nephelometric Turbidity Unit",
            }
        },
    > = UcumUnit;

    /// Formazin Nephelometric Unit (`[FNU]`), a unit of `Turbidity`: `n` of them is `n · 1` coherent units.
    pub const FORMAZIN_NEPHELOMETRIC_UNIT: UcumUnit<
        TURBIDITY,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "[FNU]",
                name: "Formazin Nephelometric Unit",
            }
        },
    > = UcumUnit;

    /// Stere (`st`), a unit of `Volume`: `n` of them is `n · 1` coherent units.
    pub const STERE: UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "st",
                name: "stere",
            }
        },
    > = UcumUnit;
}
