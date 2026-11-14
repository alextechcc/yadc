use crate::{Dim, Qty};
use yadc::Dimension;

/// The dimension of `Area`.
pub const AREA: Dim = Dim {
    m: 2,
    ..Dim::DIMENSIONLESS
};
pub type Area<N> = Qty<AREA, N>;

/// The dimension of `DepthOfWater`.
pub const DEPTH_OF_WATER: Dim = Dim {
    m: 1,
    ..Dim::DIMENSIONLESS
};
pub type DepthOfWater<N> = Qty<DEPTH_OF_WATER, N>;

/// The dimension of `HeightOfHorses`.
pub const HEIGHT_OF_HORSES: Dim = Dim {
    m: 1,
    ..Dim::DIMENSIONLESS
};
pub type HeightOfHorses<N> = Qty<HEIGHT_OF_HORSES, N>;

/// The dimension of `Length`.
pub const LENGTH: Dim = Dim {
    m: 1,
    ..Dim::DIMENSIONLESS
};
pub type Length<N> = Qty<LENGTH, N>;

/// The dimension of `Velocity`.
pub const VELOCITY: Dim = Dim {
    m: 1,
    s: -1,
    ..Dim::DIMENSIONLESS
};
pub type Velocity<N> = Qty<VELOCITY, N>;

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

    /// Square inch (`[sin_i]`), a unit of `Area`: `n` of them is `n · 64516e-8` coherent units.
    pub type SquareInch = UcumUnit<
        AREA,
        {
            UcumUnitConsts {
                size: Exact::dec(64516, -8),
                zero: Exact::ZERO,
                symbol: "[sin_i]",
                name: "square inch",
            }
        },
    >;

    /// Square foot (`[sft_i]`), a unit of `Area`: `n` of them is `n · 9290304e-8` coherent units.
    pub type SquareFoot = UcumUnit<
        AREA,
        {
            UcumUnitConsts {
                size: Exact::dec(9290304, -8),
                zero: Exact::ZERO,
                symbol: "[sft_i]",
                name: "square foot",
            }
        },
    >;

    /// Square yard (`[syd_i]`), a unit of `Area`: `n` of them is `n · 83612736e-8` coherent units.
    pub type SquareYard = UcumUnit<
        AREA,
        {
            UcumUnitConsts {
                size: Exact::dec(83612736, -8),
                zero: Exact::ZERO,
                symbol: "[syd_i]",
                name: "square yard",
            }
        },
    >;

    /// Circular mil (`[cml_i]`), a unit of `Area`: `n` of them is `n · 16129e-14 * core::f64::consts::PI` coherent units.
    pub type CircularMil = UcumUnit<
        AREA,
        {
            UcumUnitConsts {
                size: Exact::dec(16129, -14) * Exact::PI,
                zero: Exact::ZERO,
                symbol: "[cml_i]",
                name: "circular mil",
            }
        },
    >;

    /// Fathom (`[fth_i]`), a unit of `DepthOfWater`: `n` of them is `n · 18288e-4` coherent units.
    pub type Fathom = UcumUnit<
        DEPTH_OF_WATER,
        {
            UcumUnitConsts {
                size: Exact::dec(18288, -4),
                zero: Exact::ZERO,
                symbol: "[fth_i]",
                name: "fathom",
            }
        },
    >;

    /// Hand (`[hd_i]`), a unit of `HeightOfHorses`: `n` of them is `n · 1016e-4` coherent units.
    pub type Hand = UcumUnit<
        HEIGHT_OF_HORSES,
        {
            UcumUnitConsts {
                size: Exact::dec(1016, -4),
                zero: Exact::ZERO,
                symbol: "[hd_i]",
                name: "hand",
            }
        },
    >;

    /// Inch (`[in_i]`), a unit of `Length`: `n` of them is `n · 254e-4` coherent units.
    pub type Inch = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "[in_i]",
                name: "inch",
            }
        },
    >;

    /// Foot (`[ft_i]`), a unit of `Length`: `n` of them is `n · 3048e-4` coherent units.
    pub type Foot = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "[ft_i]",
                name: "foot",
            }
        },
    >;

    /// Yard (`[yd_i]`), a unit of `Length`: `n` of them is `n · 9144e-4` coherent units.
    pub type Yard = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(9144, -4),
                zero: Exact::ZERO,
                symbol: "[yd_i]",
                name: "yard",
            }
        },
    >;

    /// Mile (`[mi_i]`), a unit of `Length`: `n` of them is `n · 1609344e-3` coherent units.
    pub type Mile = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(1609344, -3),
                zero: Exact::ZERO,
                symbol: "[mi_i]",
                name: "mile",
            }
        },
    >;

    /// Nautical mile (`[nmi_i]`), a unit of `Length`: `n` of them is `n · 1852` coherent units.
    pub type NauticalMile = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(1852),
                zero: Exact::ZERO,
                symbol: "[nmi_i]",
                name: "nautical mile",
            }
        },
    >;

    /// Mil (`[mil_i]`), a unit of `Length`: `n` of them is `n · 254e-7` coherent units.
    pub type Mil = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(254, -7),
                zero: Exact::ZERO,
                symbol: "[mil_i]",
                name: "mil",
            }
        },
    >;

    /// Knot (`[kn_i]`), a unit of `Velocity`: `n` of them is `n · 463 / 900` coherent units.
    pub type Knot = UcumUnit<
        VELOCITY,
        {
            UcumUnitConsts {
                size: Exact::int(463) / Exact::int(900),
                zero: Exact::ZERO,
                symbol: "[kn_i]",
                name: "knot",
            }
        },
    >;

    /// Cubic inch (`[cin_i]`), a unit of `Volume`: `n` of them is `n · 16387064e-12` coherent units.
    pub type CubicInch = UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(16387064, -12),
                zero: Exact::ZERO,
                symbol: "[cin_i]",
                name: "cubic inch",
            }
        },
    >;

    /// Cubic foot (`[cft_i]`), a unit of `Volume`: `n` of them is `n · 28316846592e-12` coherent units.
    pub type CubicFoot = UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(28316846592, -12),
                zero: Exact::ZERO,
                symbol: "[cft_i]",
                name: "cubic foot",
            }
        },
    >;

    /// Cubic yard (`[cyd_i]`), a unit of `Volume`: `n` of them is `n · 764554857984e-12` coherent units.
    pub type CubicYard = UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(764554857984, -12),
                zero: Exact::ZERO,
                symbol: "[cyd_i]",
                name: "cubic yard",
            }
        },
    >;

    /// Board foot (`[bf_i]`), a unit of `Volume`: `n` of them is `n · 2359737216e-12` coherent units.
    pub type BoardFoot = UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(2359737216, -12),
                zero: Exact::ZERO,
                symbol: "[bf_i]",
                name: "board foot",
            }
        },
    >;

    /// Cord (`[cr_i]`), a unit of `Volume`: `n` of them is `n · 3624556363776e-12` coherent units.
    pub type Cord = UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(3624556363776, -12),
                zero: Exact::ZERO,
                symbol: "[cr_i]",
                name: "cord",
            }
        },
    >;
}
