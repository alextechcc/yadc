use crate::{Dim, Qty};
use yadc::Dimension;

/// The dimension of `Area`.
pub const AREA: Dim = Dim {
    m: 2,
    ..Dim::DIMENSIONLESS
};
pub type Area<N> = Qty<AREA, N>;

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

    /// Acre (`[acr_br]`), a unit of `Area`: `n` of them is `n · 4046850049400269056e-15` coherent units.
    pub type Acre = UcumUnit<
        AREA,
        {
            UcumUnitConsts {
                size: Exact::dec(4046850049400269056, -15),
                zero: Exact::ZERO,
                symbol: "[acr_br]",
                name: "acre",
            }
        },
    >;

    /// Inch (`[in_br]`), a unit of `Length`: `n` of them is `n · 2539998e-8` coherent units.
    pub type Inch = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(2539998, -8),
                zero: Exact::ZERO,
                symbol: "[in_br]",
                name: "inch",
            }
        },
    >;

    /// Foot (`[ft_br]`), a unit of `Length`: `n` of them is `n · 30479976e-8` coherent units.
    pub type Foot = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(30479976, -8),
                zero: Exact::ZERO,
                symbol: "[ft_br]",
                name: "foot",
            }
        },
    >;

    /// Rod (`[rd_br]`), a unit of `Length`: `n` of them is `n · 502919604e-8` coherent units.
    pub type Rod = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(502919604, -8),
                zero: Exact::ZERO,
                symbol: "[rd_br]",
                name: "rod",
            }
        },
    >;

    /// Gunter's chain (`[ch_br]`), a unit of `Length`: `n` of them is `n · 2011678416e-8` coherent units.
    pub type GunterSChain = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(2011678416, -8),
                zero: Exact::ZERO,
                symbol: "[ch_br]",
                name: "Gunter's chain",
            }
        },
    >;

    /// Link for Gunter's chain (`[lk_br]`), a unit of `Length`: `n` of them is `n · 2011678416e-10` coherent units.
    pub type LinkForGunterSChain = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(2011678416, -10),
                zero: Exact::ZERO,
                symbol: "[lk_br]",
                name: "link for Gunter's chain",
            }
        },
    >;

    /// Fathom (`[fth_br]`), a unit of `Length`: `n` of them is `n · 182879856e-8` coherent units.
    pub type Fathom = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(182879856, -8),
                zero: Exact::ZERO,
                symbol: "[fth_br]",
                name: "fathom",
            }
        },
    >;

    /// Pace (`[pc_br]`), a unit of `Length`: `n` of them is `n · 7619994e-7` coherent units.
    pub type Pace = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(7619994, -7),
                zero: Exact::ZERO,
                symbol: "[pc_br]",
                name: "pace",
            }
        },
    >;

    /// Yard (`[yd_br]`), a unit of `Length`: `n` of them is `n · 91439928e-8` coherent units.
    pub type Yard = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(91439928, -8),
                zero: Exact::ZERO,
                symbol: "[yd_br]",
                name: "yard",
            }
        },
    >;

    /// Mile (`[mi_br]`), a unit of `Length`: `n` of them is `n · 16093427328e-7` coherent units.
    pub type Mile = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(16093427328, -7),
                zero: Exact::ZERO,
                symbol: "[mi_br]",
                name: "mile",
            }
        },
    >;

    /// Nautical mile (`[nmi_br]`), a unit of `Length`: `n` of them is `n · 18531825408e-7` coherent units.
    pub type NauticalMile = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::dec(18531825408, -7),
                zero: Exact::ZERO,
                symbol: "[nmi_br]",
                name: "nautical mile",
            }
        },
    >;

    /// Knot (`[kn_br]`), a unit of `Velocity`: `n` of them is `n · 514772928e-9` coherent units.
    pub type Knot = UcumUnit<
        VELOCITY,
        {
            UcumUnitConsts {
                size: Exact::dec(514772928, -9),
                zero: Exact::ZERO,
                symbol: "[kn_br]",
                name: "knot",
            }
        },
    >;

    /// Gallon (`[gal_br]`), a unit of `Volume`: `n` of them is `n · 454609e-8` coherent units.
    pub type Gallon = UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(454609, -8),
                zero: Exact::ZERO,
                symbol: "[gal_br]",
                name: "gallon",
            }
        },
    >;

    /// Peck (`[pk_br]`), a unit of `Volume`: `n` of them is `n · 909218e-8` coherent units.
    pub type Peck = UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(909218, -8),
                zero: Exact::ZERO,
                symbol: "[pk_br]",
                name: "peck",
            }
        },
    >;

    /// Bushel (`[bu_br]`), a unit of `Volume`: `n` of them is `n · 3636872e-8` coherent units.
    pub type Bushel = UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(3636872, -8),
                zero: Exact::ZERO,
                symbol: "[bu_br]",
                name: "bushel",
            }
        },
    >;

    /// Quart (`[qt_br]`), a unit of `Volume`: `n` of them is `n · 11365225e-10` coherent units.
    pub type Quart = UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(11365225, -10),
                zero: Exact::ZERO,
                symbol: "[qt_br]",
                name: "quart",
            }
        },
    >;

    /// Pint (`[pt_br]`), a unit of `Volume`: `n` of them is `n · 56826125e-11` coherent units.
    pub type Pint = UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(56826125, -11),
                zero: Exact::ZERO,
                symbol: "[pt_br]",
                name: "pint",
            }
        },
    >;

    /// Gill (`[gil_br]`), a unit of `Volume`: `n` of them is `n · 1420653125e-13` coherent units.
    pub type Gill = UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(1420653125, -13),
                zero: Exact::ZERO,
                symbol: "[gil_br]",
                name: "gill",
            }
        },
    >;

    /// Fluid ounce (`[foz_br]`), a unit of `Volume`: `n` of them is `n · 284130625e-13` coherent units.
    pub type FluidOunce = UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(284130625, -13),
                zero: Exact::ZERO,
                symbol: "[foz_br]",
                name: "fluid ounce",
            }
        },
    >;

    /// Fluid dram (`[fdr_br]`), a unit of `Volume`: `n` of them is `n · 35516328125e-16` coherent units.
    pub type FluidDram = UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(35516328125, -16),
                zero: Exact::ZERO,
                symbol: "[fdr_br]",
                name: "fluid dram",
            }
        },
    >;

    /// Minim (`[min_br]`), a unit of `Volume`: `n` of them is `n · 454609 / 7680000000000` coherent units.
    pub type Minim = UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::int(454609) / Exact::int(7680000000000),
                zero: Exact::ZERO,
                symbol: "[min_br]",
                name: "minim",
            }
        },
    >;
}
