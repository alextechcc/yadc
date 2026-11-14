use crate::Dim;
use yadc::Dimension;

/// The dimension of `Area`: `Qty<AREA, N>` is one.
pub const AREA: Dim = Dim {
    m: 2,
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

    /// Acre (`[acr_br]`), a unit of `Area`: `n` of them is `n · 4046850049400269056e-15` coherent units.
    pub const ACRE: UcumUnit<
        AREA,
        {
            UcumUnitConsts {
                size: Exact::dec(4046850049400269056, -15),
                zero: Exact::ZERO,
                symbol: "[acr_br]",
                name: "acre",
            }
        },
    > = UcumUnit;

    /// Inch (`[in_br]`), a unit of `Length`: `n` of them is `n · 2539998e-8` coherent units.
    pub const INCH: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(2539998, -8),
                zero: Exact::ZERO,
                symbol: "[in_br]",
                name: "inch",
            }
        },
    > = UcumUnit;

    /// Foot (`[ft_br]`), a unit of `Length`: `n` of them is `n · 30479976e-8` coherent units.
    pub const FOOT: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(30479976, -8),
                zero: Exact::ZERO,
                symbol: "[ft_br]",
                name: "foot",
            }
        },
    > = UcumUnit;

    /// Rod (`[rd_br]`), a unit of `Length`: `n` of them is `n · 502919604e-8` coherent units.
    pub const ROD: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(502919604, -8),
                zero: Exact::ZERO,
                symbol: "[rd_br]",
                name: "rod",
            }
        },
    > = UcumUnit;

    /// Gunter's chain (`[ch_br]`), a unit of `Length`: `n` of them is `n · 2011678416e-8` coherent units.
    pub const GUNTER_S_CHAIN: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(2011678416, -8),
                zero: Exact::ZERO,
                symbol: "[ch_br]",
                name: "Gunter's chain",
            }
        },
    > = UcumUnit;

    /// Link for Gunter's chain (`[lk_br]`), a unit of `Length`: `n` of them is `n · 2011678416e-10` coherent units.
    pub const LINK_FOR_GUNTER_S_CHAIN: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(2011678416, -10),
                zero: Exact::ZERO,
                symbol: "[lk_br]",
                name: "link for Gunter's chain",
            }
        },
    > = UcumUnit;

    /// Fathom (`[fth_br]`), a unit of `Length`: `n` of them is `n · 182879856e-8` coherent units.
    pub const FATHOM: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(182879856, -8),
                zero: Exact::ZERO,
                symbol: "[fth_br]",
                name: "fathom",
            }
        },
    > = UcumUnit;

    /// Pace (`[pc_br]`), a unit of `Length`: `n` of them is `n · 7619994e-7` coherent units.
    pub const PACE: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(7619994, -7),
                zero: Exact::ZERO,
                symbol: "[pc_br]",
                name: "pace",
            }
        },
    > = UcumUnit;

    /// Yard (`[yd_br]`), a unit of `Length`: `n` of them is `n · 91439928e-8` coherent units.
    pub const YARD: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(91439928, -8),
                zero: Exact::ZERO,
                symbol: "[yd_br]",
                name: "yard",
            }
        },
    > = UcumUnit;

    /// Mile (`[mi_br]`), a unit of `Length`: `n` of them is `n · 16093427328e-7` coherent units.
    pub const MILE: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(16093427328, -7),
                zero: Exact::ZERO,
                symbol: "[mi_br]",
                name: "mile",
            }
        },
    > = UcumUnit;

    /// Nautical mile (`[nmi_br]`), a unit of `Length`: `n` of them is `n · 18531825408e-7` coherent units.
    pub const NAUTICAL_MILE: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(18531825408, -7),
                zero: Exact::ZERO,
                symbol: "[nmi_br]",
                name: "nautical mile",
            }
        },
    > = UcumUnit;

    /// Knot (`[kn_br]`), a unit of `Velocity`: `n` of them is `n · 514772928e-9` coherent units.
    pub const KNOT: UcumUnit<
        VELOCITY,
        {
            UcumUnitConsts {
                size: Exact::dec(514772928, -9),
                zero: Exact::ZERO,
                symbol: "[kn_br]",
                name: "knot",
            }
        },
    > = UcumUnit;

    /// Gallon (`[gal_br]`), a unit of `Volume`: `n` of them is `n · 454609e-8` coherent units.
    pub const GALLON: UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(454609, -8),
                zero: Exact::ZERO,
                symbol: "[gal_br]",
                name: "gallon",
            }
        },
    > = UcumUnit;

    /// Peck (`[pk_br]`), a unit of `Volume`: `n` of them is `n · 909218e-8` coherent units.
    pub const PECK: UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(909218, -8),
                zero: Exact::ZERO,
                symbol: "[pk_br]",
                name: "peck",
            }
        },
    > = UcumUnit;

    /// Bushel (`[bu_br]`), a unit of `Volume`: `n` of them is `n · 3636872e-8` coherent units.
    pub const BUSHEL: UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(3636872, -8),
                zero: Exact::ZERO,
                symbol: "[bu_br]",
                name: "bushel",
            }
        },
    > = UcumUnit;

    /// Quart (`[qt_br]`), a unit of `Volume`: `n` of them is `n · 11365225e-10` coherent units.
    pub const QUART: UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(11365225, -10),
                zero: Exact::ZERO,
                symbol: "[qt_br]",
                name: "quart",
            }
        },
    > = UcumUnit;

    /// Pint (`[pt_br]`), a unit of `Volume`: `n` of them is `n · 56826125e-11` coherent units.
    pub const PINT: UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(56826125, -11),
                zero: Exact::ZERO,
                symbol: "[pt_br]",
                name: "pint",
            }
        },
    > = UcumUnit;

    /// Gill (`[gil_br]`), a unit of `Volume`: `n` of them is `n · 1420653125e-13` coherent units.
    pub const GILL: UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(1420653125, -13),
                zero: Exact::ZERO,
                symbol: "[gil_br]",
                name: "gill",
            }
        },
    > = UcumUnit;

    /// Fluid ounce (`[foz_br]`), a unit of `Volume`: `n` of them is `n · 284130625e-13` coherent units.
    pub const FLUID_OUNCE: UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(284130625, -13),
                zero: Exact::ZERO,
                symbol: "[foz_br]",
                name: "fluid ounce",
            }
        },
    > = UcumUnit;

    /// Fluid dram (`[fdr_br]`), a unit of `Volume`: `n` of them is `n · 35516328125e-16` coherent units.
    pub const FLUID_DRAM: UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(35516328125, -16),
                zero: Exact::ZERO,
                symbol: "[fdr_br]",
                name: "fluid dram",
            }
        },
    > = UcumUnit;

    /// Minim (`[min_br]`), a unit of `Volume`: `n` of them is `n · 454609 / 7680000000000` coherent units.
    pub const MINIM: UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::int(454609) / Exact::int(7680000000000),
                zero: Exact::ZERO,
                symbol: "[min_br]",
                name: "minim",
            }
        },
    > = UcumUnit;
}
