use crate::Dim;
use yadc::Dimension;

/// The dimension of `Area`: `Qty<AREA, N>` is one.
pub const AREA: Dim = Dim {
    m: 2,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `DepthOfWater`: `Qty<DEPTH_OF_WATER, N>` is one.
pub const DEPTH_OF_WATER: Dim = Dim {
    m: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `HeightOfHorses`: `Qty<HEIGHT_OF_HORSES, N>` is one.
pub const HEIGHT_OF_HORSES: Dim = Dim {
    m: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Length`: `Qty<LENGTH, N>` is one.
pub const LENGTH: Dim = Dim {
    m: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Velocity`: `Qty<VELOCITY, N>` is one.
pub const VELOCITY: Dim = Dim {
    m: 1,
    s: -1,
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

    /// Square inch (`[sin_i]`), a unit of `Area`: `n` of them is `n · 64516e-8` coherent units.
    pub const SQUARE_INCH: UcumUnit<
        AREA,
        {
            UcumUnitConsts {
                size: Exact::dec(64516, -8),
                zero: Exact::ZERO,
                symbol: "[sin_i]",
                name: "square inch",
            }
        },
    > = UcumUnit;

    /// Square foot (`[sft_i]`), a unit of `Area`: `n` of them is `n · 9290304e-8` coherent units.
    pub const SQUARE_FOOT: UcumUnit<
        AREA,
        {
            UcumUnitConsts {
                size: Exact::dec(9290304, -8),
                zero: Exact::ZERO,
                symbol: "[sft_i]",
                name: "square foot",
            }
        },
    > = UcumUnit;

    /// Square yard (`[syd_i]`), a unit of `Area`: `n` of them is `n · 83612736e-8` coherent units.
    pub const SQUARE_YARD: UcumUnit<
        AREA,
        {
            UcumUnitConsts {
                size: Exact::dec(83612736, -8),
                zero: Exact::ZERO,
                symbol: "[syd_i]",
                name: "square yard",
            }
        },
    > = UcumUnit;

    /// Circular mil (`[cml_i]`), a unit of `Area`: `n` of them is `n · 16129e-14 * core::f64::consts::PI` coherent units.
    pub const CIRCULAR_MIL: UcumUnit<
        AREA,
        {
            UcumUnitConsts {
                size: Exact::dec(16129, -14) * Exact::PI,
                zero: Exact::ZERO,
                symbol: "[cml_i]",
                name: "circular mil",
            }
        },
    > = UcumUnit;

    /// Fathom (`[fth_i]`), a unit of `DepthOfWater`: `n` of them is `n · 18288e-4` coherent units.
    pub const FATHOM: UcumUnit<
        DEPTH_OF_WATER,
        {
            UcumUnitConsts {
                size: Exact::dec(18288, -4),
                zero: Exact::ZERO,
                symbol: "[fth_i]",
                name: "fathom",
            }
        },
    > = UcumUnit;

    /// Hand (`[hd_i]`), a unit of `HeightOfHorses`: `n` of them is `n · 1016e-4` coherent units.
    pub const HAND: UcumUnit<
        HEIGHT_OF_HORSES,
        {
            UcumUnitConsts {
                size: Exact::dec(1016, -4),
                zero: Exact::ZERO,
                symbol: "[hd_i]",
                name: "hand",
            }
        },
    > = UcumUnit;

    /// Inch (`[in_i]`), a unit of `Length`: `n` of them is `n · 254e-4` coherent units.
    pub const INCH: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "[in_i]",
                name: "inch",
            }
        },
    > = UcumUnit;

    /// Foot (`[ft_i]`), a unit of `Length`: `n` of them is `n · 3048e-4` coherent units.
    pub const FOOT: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "[ft_i]",
                name: "foot",
            }
        },
    > = UcumUnit;

    /// Yard (`[yd_i]`), a unit of `Length`: `n` of them is `n · 9144e-4` coherent units.
    pub const YARD: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(9144, -4),
                zero: Exact::ZERO,
                symbol: "[yd_i]",
                name: "yard",
            }
        },
    > = UcumUnit;

    /// Mile (`[mi_i]`), a unit of `Length`: `n` of them is `n · 1609344e-3` coherent units.
    pub const MILE: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(1609344, -3),
                zero: Exact::ZERO,
                symbol: "[mi_i]",
                name: "mile",
            }
        },
    > = UcumUnit;

    /// Nautical mile (`[nmi_i]`), a unit of `Length`: `n` of them is `n · 1852` coherent units.
    pub const NAUTICAL_MILE: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(1852),
                zero: Exact::ZERO,
                symbol: "[nmi_i]",
                name: "nautical mile",
            }
        },
    > = UcumUnit;

    /// Mil (`[mil_i]`), a unit of `Length`: `n` of them is `n · 254e-7` coherent units.
    pub const MIL: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(254, -7),
                zero: Exact::ZERO,
                symbol: "[mil_i]",
                name: "mil",
            }
        },
    > = UcumUnit;

    /// Knot (`[kn_i]`), a unit of `Velocity`: `n` of them is `n · 463 / 900` coherent units.
    pub const KNOT: UcumUnit<
        VELOCITY,
        {
            UcumUnitConsts {
                size: Exact::int(463) / Exact::int(900),
                zero: Exact::ZERO,
                symbol: "[kn_i]",
                name: "knot",
            }
        },
    > = UcumUnit;

    /// Cubic inch (`[cin_i]`), a unit of `Volume`: `n` of them is `n · 16387064e-12` coherent units.
    pub const CUBIC_INCH: UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(16387064, -12),
                zero: Exact::ZERO,
                symbol: "[cin_i]",
                name: "cubic inch",
            }
        },
    > = UcumUnit;

    /// Cubic foot (`[cft_i]`), a unit of `Volume`: `n` of them is `n · 28316846592e-12` coherent units.
    pub const CUBIC_FOOT: UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(28316846592, -12),
                zero: Exact::ZERO,
                symbol: "[cft_i]",
                name: "cubic foot",
            }
        },
    > = UcumUnit;

    /// Cubic yard (`[cyd_i]`), a unit of `Volume`: `n` of them is `n · 764554857984e-12` coherent units.
    pub const CUBIC_YARD: UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(764554857984, -12),
                zero: Exact::ZERO,
                symbol: "[cyd_i]",
                name: "cubic yard",
            }
        },
    > = UcumUnit;

    /// Board foot (`[bf_i]`), a unit of `Volume`: `n` of them is `n · 2359737216e-12` coherent units.
    pub const BOARD_FOOT: UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(2359737216, -12),
                zero: Exact::ZERO,
                symbol: "[bf_i]",
                name: "board foot",
            }
        },
    > = UcumUnit;

    /// Cord (`[cr_i]`), a unit of `Volume`: `n` of them is `n · 3624556363776e-12` coherent units.
    pub const CORD: UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(3624556363776, -12),
                zero: Exact::ZERO,
                symbol: "[cr_i]",
                name: "cord",
            }
        },
    > = UcumUnit;
}
