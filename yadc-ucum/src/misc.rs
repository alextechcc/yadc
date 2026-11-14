use crate::{Dim, Qty};
use yadc::Dimension;

/// The dimension of `ActionArea`.
pub const ACTION_AREA: Dim = Dim {
    m: 2,
    ..Dim::DIMENSIONLESS
};
pub type ActionArea<N> = Qty<ACTION_AREA, N>;

/// The dimension of `ElectricConductance`.
pub const ELECTRIC_CONDUCTANCE: Dim = Dim {
    m: -2,
    s: 1,
    g: -1,
    C: 2,
    ..Dim::DIMENSIONLESS
};
pub type ElectricConductance<N> = Qty<ELECTRIC_CONDUCTANCE, N>;

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
pub type MassFraction<N> = Qty<{ Dim::DIMENSIONLESS }, N>;

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

/// The dimension of `SolidAngle`.
pub const SOLID_ANGLE: Dim = Dim {
    rad: 2,
    ..Dim::DIMENSIONLESS
};
pub type SolidAngle<N> = Qty<SOLID_ANGLE, N>;
pub type Turbidity<N> = Qty<{ Dim::DIMENSIONLESS }, N>;

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
    use crate::{Dim, Exact, UcumUnit, UcumUnitConsts};
    use yadc::Dimension;

    /// Barn (`b`), a unit of `ActionArea`: `n` of them is `n · 1e-28` coherent units.
    pub type Barn = UcumUnit<
        ACTION_AREA,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -28),
                zero: Exact::ZERO,
                symbol: "b",
                name: "barn",
            }
        },
    >;

    /// Mho (`mho`), a unit of `ElectricConductance`: `n` of them is `n · 1e-3` coherent units.
    pub type Mho = UcumUnit<
        ELECTRIC_CONDUCTANCE,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "mho",
                name: "mho",
            }
        },
    >;

    /// Line (`[lne]`), a unit of `Length`: `n` of them is `n · 127 / 60000` coherent units.
    pub type Line = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(127) / Exact::int(60000),
                zero: Exact::ZERO,
                symbol: "[lne]",
                name: "line",
            }
        },
    >;

    /// Point (`[pnt]`), a unit of `Length`: `n` of them is `n · 127 / 360000` coherent units.
    pub type Point = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(127) / Exact::int(360000),
                zero: Exact::ZERO,
                symbol: "[pnt]",
                name: "point",
            }
        },
    >;

    /// Pica (`[pca]`), a unit of `Length`: `n` of them is `n · 127 / 30000` coherent units.
    pub type Pica = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(127) / Exact::int(30000),
                zero: Exact::ZERO,
                symbol: "[pca]",
                name: "pica",
            }
        },
    >;

    /// Printer's point (`[pnt_pr]`), a unit of `Length`: `n` of them is `n · 3514598e-10` coherent units.
    pub type PrinterSPoint = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(3514598, -10),
                zero: Exact::ZERO,
                symbol: "[pnt_pr]",
                name: "Printer's point",
            }
        },
    >;

    /// Printer's pica (`[pca_pr]`), a unit of `Length`: `n` of them is `n · 42175176e-10` coherent units.
    pub type PrinterSPica = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(42175176, -10),
                zero: Exact::ZERO,
                symbol: "[pca_pr]",
                name: "Printer's pica",
            }
        },
    >;

    /// French foot (`[pied]`), a unit of `Length`: `n` of them is `n · 3248e-4` coherent units.
    pub type FrenchFoot = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(3248, -4),
                zero: Exact::ZERO,
                symbol: "[pied]",
                name: "French foot",
            }
        },
    >;

    /// French inch (`[pouce]`), a unit of `Length`: `n` of them is `n · 203 / 7500` coherent units.
    pub type FrenchInch = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(203) / Exact::int(7500),
                zero: Exact::ZERO,
                symbol: "[pouce]",
                name: "French inch",
            }
        },
    >;

    /// French line (`[ligne]`), a unit of `Length`: `n` of them is `n · 203 / 90000` coherent units.
    pub type FrenchLine = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(203) / Exact::int(90000),
                zero: Exact::ZERO,
                symbol: "[ligne]",
                name: "French line",
            }
        },
    >;

    /// Didot's point (`[didot]`), a unit of `Length`: `n` of them is `n · 203 / 540000` coherent units.
    pub type DidotSPoint = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(203) / Exact::int(540000),
                zero: Exact::ZERO,
                symbol: "[didot]",
                name: "Didot's point",
            }
        },
    >;

    /// Didot's pica (`[cicero]`), a unit of `Length`: `n` of them is `n · 203 / 45000` coherent units.
    pub type DidotSPica = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(203) / Exact::int(45000),
                zero: Exact::ZERO,
                symbol: "[cicero]",
                name: "Didot's pica",
            }
        },
    >;

    /// Ångström (`Ao`), a unit of `Length`: `n` of them is `n · 1e-10` coherent units.
    pub type Angstrom = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -10),
                zero: Exact::ZERO,
                symbol: "Ao",
                name: "Ångström",
            }
        },
    >;

    /// Smoot (`[smoot]`), a unit of `Length`: `n` of them is `n · 17018e-4` coherent units.
    pub type Smoot = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(17018, -4),
                zero: Exact::ZERO,
                symbol: "[smoot]",
                name: "Smoot",
            }
        },
    >;

    /// Metric carat (`[car_m]`), a unit of `Mass`: `n` of them is `n · 2e-1` coherent units.
    pub type MetricCarat = UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(2, -1),
                zero: Exact::ZERO,
                symbol: "[car_m]",
                name: "metric carat",
            }
        },
    >;

    /// Carat of gold alloys (`[car_Au]`), a unit of `MassFraction`: `n` of them is `n · 1 / 24` coherent units.
    pub type CaratOfGoldAlloys = UcumUnit<
        { Dim::DIMENSIONLESS },
        {
            UcumUnitConsts {
                size: Exact::int(1) / Exact::int(24),
                zero: Exact::ZERO,
                symbol: "[car_Au]",
                name: "carat of gold alloys",
            }
        },
    >;

    /// Circle (`circ`), a unit of `PlaneAngle`: `n` of them is `n · 2 * core::f64::consts::PI` coherent units.
    pub type Circle = UcumUnit<
        PLANE_ANGLE,
        {
            UcumUnitConsts {
                size: Exact::int(2) * Exact::PI,
                zero: Exact::ZERO,
                symbol: "circ",
                name: "circle",
            }
        },
    >;

    /// Technical atmosphere (`att`), a unit of `Pressure`: `n` of them is `n · 980665e2` coherent units.
    pub type TechnicalAtmosphere = UcumUnit<
        PRESSURE,
        {
            UcumUnitConsts {
                size: Exact::int(98066500),
                zero: Exact::ZERO,
                symbol: "att",
                name: "technical atmosphere",
            }
        },
    >;

    /// Pound per square inch (`[psi]`), a unit of `Pressure`: `n` of them is `n · 8896443230521 / 1290320` coherent units.
    pub type PoundPerSquareInch = UcumUnit<
        PRESSURE,
        {
            UcumUnitConsts {
                size: Exact::int(8896443230521) / Exact::int(1290320),
                zero: Exact::ZERO,
                symbol: "[psi]",
                name: "pound per square inch",
            }
        },
    >;

    /// Sphere (`sph`), a unit of `SolidAngle`: `n` of them is `n · 4 * core::f64::consts::PI` coherent units.
    pub type Sphere = UcumUnit<
        SOLID_ANGLE,
        {
            UcumUnitConsts {
                size: Exact::int(4) * Exact::PI,
                zero: Exact::ZERO,
                symbol: "sph",
                name: "sphere",
            }
        },
    >;

    /// Nephelometric Turbidity Unit (`[NTU]`), a unit of `Turbidity`: `n` of them is `n · 1` coherent units.
    pub type NephelometricTurbidityUnit = UcumUnit<
        { Dim::DIMENSIONLESS },
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "[NTU]",
                name: "Nephelometric Turbidity Unit",
            }
        },
    >;

    /// Formazin Nephelometric Unit (`[FNU]`), a unit of `Turbidity`: `n` of them is `n · 1` coherent units.
    pub type FormazinNephelometricUnit = UcumUnit<
        { Dim::DIMENSIONLESS },
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "[FNU]",
                name: "Formazin Nephelometric Unit",
            }
        },
    >;

    /// Stere (`st`), a unit of `Volume`: `n` of them is `n · 1` coherent units.
    pub type Stere = UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "st",
                name: "stere",
            }
        },
    >;
}
