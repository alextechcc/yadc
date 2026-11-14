use crate::Dim;
use yadc::Dimension;

/// The dimension of `Area`: `Qty<AREA, N>` is one.
pub const AREA: Dim = Dim {
    m: 2,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `DryVolume`: `Qty<DRY_VOLUME, N>` is one.
pub const DRY_VOLUME: Dim = Dim {
    m: 3,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `FluidVolume`: `Qty<FLUID_VOLUME, N>` is one.
pub const FLUID_VOLUME: Dim = Dim {
    m: 3,
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

    /// Acre (`[acr_us]`), a unit of `Area`: `n` of them is `n · 627264e5 / 15499969` coherent units.
    pub const ACRE: UcumUnit<
        AREA,
        {
            UcumUnitConsts {
                size: Exact::int(62726400000) / Exact::int(15499969),
                zero: Exact::ZERO,
                symbol: "[acr_us]",
                name: "acre",
            }
        },
    > = UcumUnit;

    /// Square rod (`[srd_us]`), a unit of `Area`: `n` of them is `n · 39204e4 / 15499969` coherent units.
    pub const SQUARE_ROD: UcumUnit<
        AREA,
        {
            UcumUnitConsts {
                size: Exact::int(392040000) / Exact::int(15499969),
                zero: Exact::ZERO,
                symbol: "[srd_us]",
                name: "square rod",
            }
        },
    > = UcumUnit;

    /// Square mile (`[smi_us]`), a unit of `Area`: `n` of them is `n · 40144896e6 / 15499969` coherent units.
    pub const SQUARE_MILE: UcumUnit<
        AREA,
        {
            UcumUnitConsts {
                size: Exact::int(40144896000000) / Exact::int(15499969),
                zero: Exact::ZERO,
                symbol: "[smi_us]",
                name: "square mile",
            }
        },
    > = UcumUnit;

    /// Section (`[sct]`), a unit of `Area`: `n` of them is `n · 40144896e6 / 15499969` coherent units.
    pub const SECTION: UcumUnit<
        AREA,
        {
            UcumUnitConsts {
                size: Exact::int(40144896000000) / Exact::int(15499969),
                zero: Exact::ZERO,
                symbol: "[sct]",
                name: "section",
            }
        },
    > = UcumUnit;

    /// Township (`[twp]`), a unit of `Area`: `n` of them is `n · 1445216256e6 / 15499969` coherent units.
    pub const TOWNSHIP: UcumUnit<
        AREA,
        {
            UcumUnitConsts {
                size: Exact::int(1445216256000000) / Exact::int(15499969),
                zero: Exact::ZERO,
                symbol: "[twp]",
                name: "township",
            }
        },
    > = UcumUnit;

    /// Bushel (`[bu_us]`), a unit of `DryVolume`: `n` of them is `n · 3523907016688e-14` coherent units.
    pub const BUSHEL: UcumUnit<
        DRY_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(3523907016688, -14),
                zero: Exact::ZERO,
                symbol: "[bu_us]",
                name: "bushel",
            }
        },
    > = UcumUnit;

    /// Historical winchester gallon (`[gal_wi]`), a unit of `DryVolume`: `n` of them is `n · 440488377086e-14` coherent units.
    pub const HISTORICAL_WINCHESTER_GALLON: UcumUnit<
        DRY_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(440488377086, -14),
                zero: Exact::ZERO,
                symbol: "[gal_wi]",
                name: "historical winchester gallon",
            }
        },
    > = UcumUnit;

    /// Peck (`[pk_us]`), a unit of `DryVolume`: `n` of them is `n · 880976754172e-14` coherent units.
    pub const PECK: UcumUnit<
        DRY_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(880976754172, -14),
                zero: Exact::ZERO,
                symbol: "[pk_us]",
                name: "peck",
            }
        },
    > = UcumUnit;

    /// Dry quart (`[dqt_us]`), a unit of `DryVolume`: `n` of them is `n · 1101220942715e-15` coherent units.
    pub const DRY_QUART: UcumUnit<
        DRY_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(1101220942715, -15),
                zero: Exact::ZERO,
                symbol: "[dqt_us]",
                name: "dry quart",
            }
        },
    > = UcumUnit;

    /// Dry pint (`[dpt_us]`), a unit of `DryVolume`: `n` of them is `n · 5506104713575e-16` coherent units.
    pub const DRY_PINT: UcumUnit<
        DRY_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(5506104713575, -16),
                zero: Exact::ZERO,
                symbol: "[dpt_us]",
                name: "dry pint",
            }
        },
    > = UcumUnit;

    /// Queen Anne's wine gallon (`[gal_us]`), a unit of `FluidVolume`: `n` of them is `n · 3785411784e-12` coherent units.
    pub const QUEEN_ANNE_S_WINE_GALLON: UcumUnit<
        FLUID_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(3785411784, -12),
                zero: Exact::ZERO,
                symbol: "[gal_us]",
                name: "Queen Anne's wine gallon",
            }
        },
    > = UcumUnit;

    /// Barrel (`[bbl_us]`), a unit of `FluidVolume`: `n` of them is `n · 158987294928e-12` coherent units.
    pub const BARREL: UcumUnit<
        FLUID_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(158987294928, -12),
                zero: Exact::ZERO,
                symbol: "[bbl_us]",
                name: "barrel",
            }
        },
    > = UcumUnit;

    /// Quart (`[qt_us]`), a unit of `FluidVolume`: `n` of them is `n · 946352946e-12` coherent units.
    pub const QUART: UcumUnit<
        FLUID_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(946352946, -12),
                zero: Exact::ZERO,
                symbol: "[qt_us]",
                name: "quart",
            }
        },
    > = UcumUnit;

    /// Pint (`[pt_us]`), a unit of `FluidVolume`: `n` of them is `n · 473176473e-12` coherent units.
    pub const PINT: UcumUnit<
        FLUID_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(473176473, -12),
                zero: Exact::ZERO,
                symbol: "[pt_us]",
                name: "pint",
            }
        },
    > = UcumUnit;

    /// Gill (`[gil_us]`), a unit of `FluidVolume`: `n` of them is `n · 11829411825e-14` coherent units.
    pub const GILL: UcumUnit<
        FLUID_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(11829411825, -14),
                zero: Exact::ZERO,
                symbol: "[gil_us]",
                name: "gill",
            }
        },
    > = UcumUnit;

    /// Fluid ounce (`[foz_us]`), a unit of `FluidVolume`: `n` of them is `n · 295735295625e-16` coherent units.
    pub const FLUID_OUNCE: UcumUnit<
        FLUID_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(295735295625, -16),
                zero: Exact::ZERO,
                symbol: "[foz_us]",
                name: "fluid ounce",
            }
        },
    > = UcumUnit;

    /// Fluid dram (`[fdr_us]`), a unit of `FluidVolume`: `n` of them is `n · 36966911953125e-19` coherent units.
    pub const FLUID_DRAM: UcumUnit<
        FLUID_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(36966911953125, -19),
                zero: Exact::ZERO,
                symbol: "[fdr_us]",
                name: "fluid dram",
            }
        },
    > = UcumUnit;

    /// Minim (`[min_us]`), a unit of `FluidVolume`: `n` of them is `n · 61611519921875e-21` coherent units.
    pub const MINIM: UcumUnit<
        FLUID_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(61611519921875, -21),
                zero: Exact::ZERO,
                symbol: "[min_us]",
                name: "minim",
            }
        },
    > = UcumUnit;

    /// Cord (`[crd_us]`), a unit of `FluidVolume`: `n` of them is `n · 3624556363776e-12` coherent units.
    pub const CORD: UcumUnit<
        FLUID_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(3624556363776, -12),
                zero: Exact::ZERO,
                symbol: "[crd_us]",
                name: "cord",
            }
        },
    > = UcumUnit;

    /// Metric fluid ounce (`[foz_m]`), a unit of `FluidVolume`: `n` of them is `n · 3e-5` coherent units.
    pub const METRIC_FLUID_OUNCE: UcumUnit<
        FLUID_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(3, -5),
                zero: Exact::ZERO,
                symbol: "[foz_m]",
                name: "metric fluid ounce",
            }
        },
    > = UcumUnit;

    /// Foot (`[ft_us]`), a unit of `Length`: `n` of them is `n · 12e2 / 3937` coherent units.
    pub const FOOT: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(1200) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "[ft_us]",
                name: "foot",
            }
        },
    > = UcumUnit;

    /// Yard (`[yd_us]`), a unit of `Length`: `n` of them is `n · 36e2 / 3937` coherent units.
    pub const YARD: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(3600) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "[yd_us]",
                name: "yard",
            }
        },
    > = UcumUnit;

    /// Inch (`[in_us]`), a unit of `Length`: `n` of them is `n · 1e2 / 3937` coherent units.
    pub const INCH: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(100) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "[in_us]",
                name: "inch",
            }
        },
    > = UcumUnit;

    /// Rod (`[rd_us]`), a unit of `Length`: `n` of them is `n · 198e2 / 3937` coherent units.
    pub const ROD: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(19800) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "[rd_us]",
                name: "rod",
            }
        },
    > = UcumUnit;

    /// Surveyor's chain (`[ch_us]`), a unit of `Length`: `n` of them is `n · 792e2 / 3937` coherent units.
    pub const SURVEYOR_S_CHAIN: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(79200) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "[ch_us]",
                name: "Surveyor's chain",
            }
        },
    > = UcumUnit;

    /// Link for Gunter's chain (`[lk_us]`), a unit of `Length`: `n` of them is `n · 792 / 3937` coherent units.
    pub const LINK_FOR_GUNTER_S_CHAIN: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(792) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "[lk_us]",
                name: "link for Gunter's chain",
            }
        },
    > = UcumUnit;

    /// Engineer's chain (`[rch_us]`), a unit of `Length`: `n` of them is `n · 12e4 / 3937` coherent units.
    pub const ENGINEER_S_CHAIN: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(120000) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "[rch_us]",
                name: "Engineer's chain",
            }
        },
    > = UcumUnit;

    /// Link for Ramden's chain (`[rlk_us]`), a unit of `Length`: `n` of them is `n · 12e2 / 3937` coherent units.
    pub const LINK_FOR_RAMDEN_S_CHAIN: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(1200) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "[rlk_us]",
                name: "link for Ramden's chain",
            }
        },
    > = UcumUnit;

    /// Fathom (`[fth_us]`), a unit of `Length`: `n` of them is `n · 72e2 / 3937` coherent units.
    pub const FATHOM: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(7200) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "[fth_us]",
                name: "fathom",
            }
        },
    > = UcumUnit;

    /// Furlong (`[fur_us]`), a unit of `Length`: `n` of them is `n · 792e3 / 3937` coherent units.
    pub const FURLONG: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(792000) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "[fur_us]",
                name: "furlong",
            }
        },
    > = UcumUnit;

    /// Mile (`[mi_us]`), a unit of `Length`: `n` of them is `n · 6336e3 / 3937` coherent units.
    pub const MILE: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(6336000) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "[mi_us]",
                name: "mile",
            }
        },
    > = UcumUnit;

    /// Mil (`[mil_us]`), a unit of `Length`: `n` of them is `n · 1 / 39370` coherent units.
    pub const MIL: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(1) / Exact::int(39370),
                zero: Exact::ZERO,
                symbol: "[mil_us]",
                name: "mil",
            }
        },
    > = UcumUnit;

    /// Grain (`[gr]`), a unit of `Mass`: `n` of them is `n · 6479891e-8` coherent units.
    pub const GRAIN: UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(6479891, -8),
                zero: Exact::ZERO,
                symbol: "[gr]",
                name: "grain",
            }
        },
    > = UcumUnit;

    /// Pound (`[lb_av]`), a unit of `Mass`: `n` of them is `n · 45359237e-5` coherent units.
    pub const POUND: UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(45359237, -5),
                zero: Exact::ZERO,
                symbol: "[lb_av]",
                name: "pound",
            }
        },
    > = UcumUnit;

    /// Ounce (`[oz_av]`), a unit of `Mass`: `n` of them is `n · 28349523125e-9` coherent units.
    pub const OUNCE: UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(28349523125, -9),
                zero: Exact::ZERO,
                symbol: "[oz_av]",
                name: "ounce",
            }
        },
    > = UcumUnit;

    /// Dram (`[dr_av]`), a unit of `Mass`: `n` of them is `n · 17718451953125e-13` coherent units.
    pub const DRAM: UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(17718451953125, -13),
                zero: Exact::ZERO,
                symbol: "[dr_av]",
                name: "dram",
            }
        },
    > = UcumUnit;

    /// U.S. hundredweight (`[scwt_av]`), a unit of `Mass`: `n` of them is `n · 45359237e-3` coherent units.
    pub const US_HUNDREDWEIGHT: UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(45359237, -3),
                zero: Exact::ZERO,
                symbol: "[scwt_av]",
                name: "U.S. hundredweight",
            }
        },
    > = UcumUnit;

    /// British hundredweight (`[lcwt_av]`), a unit of `Mass`: `n` of them is `n · 5080234544e-5` coherent units.
    pub const BRITISH_HUNDREDWEIGHT: UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(5080234544, -5),
                zero: Exact::ZERO,
                symbol: "[lcwt_av]",
                name: "British hundredweight",
            }
        },
    > = UcumUnit;

    /// U.S. ton (`[ston_av]`), a unit of `Mass`: `n` of them is `n · 90718474e-2` coherent units.
    pub const US_TON: UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(90718474, -2),
                zero: Exact::ZERO,
                symbol: "[ston_av]",
                name: "U.S. ton",
            }
        },
    > = UcumUnit;

    /// British ton (`[lton_av]`), a unit of `Mass`: `n` of them is `n · 10160469088e-4` coherent units.
    pub const BRITISH_TON: UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(10160469088, -4),
                zero: Exact::ZERO,
                symbol: "[lton_av]",
                name: "British ton",
            }
        },
    > = UcumUnit;

    /// British stone (`[stone_av]`), a unit of `Mass`: `n` of them is `n · 635029318e-5` coherent units.
    pub const BRITISH_STONE: UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(635029318, -5),
                zero: Exact::ZERO,
                symbol: "[stone_av]",
                name: "British stone",
            }
        },
    > = UcumUnit;

    /// Pennyweight (`[pwt_tr]`), a unit of `Mass`: `n` of them is `n · 155517384e-8` coherent units.
    pub const PENNYWEIGHT: UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(155517384, -8),
                zero: Exact::ZERO,
                symbol: "[pwt_tr]",
                name: "pennyweight",
            }
        },
    > = UcumUnit;

    /// Scruple (`[sc_ap]`), a unit of `Mass`: `n` of them is `n · 12959782e-7` coherent units.
    pub const SCRUPLE: UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(12959782, -7),
                zero: Exact::ZERO,
                symbol: "[sc_ap]",
                name: "scruple",
            }
        },
    > = UcumUnit;

    /// Drachm (`[dr_ap]`), a unit of `Mass`: `n` of them is `n · 38879346e-7` coherent units.
    pub const DRACHM: UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(38879346, -7),
                zero: Exact::ZERO,
                symbol: "[dr_ap]",
                name: "drachm",
            }
        },
    > = UcumUnit;

    /// Metric ounce (`[oz_m]`), a unit of `Mass`: `n` of them is `n · 28` coherent units.
    pub const METRIC_OUNCE: UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::int(28),
                zero: Exact::ZERO,
                symbol: "[oz_m]",
                name: "metric ounce",
            }
        },
    > = UcumUnit;

    /// Tablespoon (`[tbs_us]`), a unit of `Volume`: `n` of them is `n · 1478676478125e-17` coherent units.
    pub const TABLESPOON: UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(1478676478125, -17),
                zero: Exact::ZERO,
                symbol: "[tbs_us]",
                name: "tablespoon",
            }
        },
    > = UcumUnit;

    /// Teaspoon (`[tsp_us]`), a unit of `Volume`: `n` of them is `n · 492892159375e-17` coherent units.
    pub const TEASPOON: UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(492892159375, -17),
                zero: Exact::ZERO,
                symbol: "[tsp_us]",
                name: "teaspoon",
            }
        },
    > = UcumUnit;

    /// Cup (`[cup_us]`), a unit of `Volume`: `n` of them is `n · 2365882365e-13` coherent units.
    pub const CUP: UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(2365882365, -13),
                zero: Exact::ZERO,
                symbol: "[cup_us]",
                name: "cup",
            }
        },
    > = UcumUnit;

    /// Metric cup (`[cup_m]`), a unit of `Volume`: `n` of them is `n · 24e-5` coherent units.
    pub const METRIC_CUP: UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(24, -5),
                zero: Exact::ZERO,
                symbol: "[cup_m]",
                name: "metric cup",
            }
        },
    > = UcumUnit;

    /// Metric teaspoon (`[tsp_m]`), a unit of `Volume`: `n` of them is `n · 5e-6` coherent units.
    pub const METRIC_TEASPOON: UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(5, -6),
                zero: Exact::ZERO,
                symbol: "[tsp_m]",
                name: "metric teaspoon",
            }
        },
    > = UcumUnit;

    /// Metric tablespoon (`[tbs_m]`), a unit of `Volume`: `n` of them is `n · 15e-6` coherent units.
    pub const METRIC_TABLESPOON: UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(15, -6),
                zero: Exact::ZERO,
                symbol: "[tbs_m]",
                name: "metric tablespoon",
            }
        },
    > = UcumUnit;
}
