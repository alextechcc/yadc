use crate::{Dim, Qty};
use yadc::Dimension;

/// The dimension of `Area`.
pub const AREA: Dim = Dim {
    m: 2,
    ..Dim::DIMENSIONLESS
};
pub type Area<N> = Qty<AREA, N>;

/// The dimension of `DryVolume`.
pub const DRY_VOLUME: Dim = Dim {
    m: 3,
    ..Dim::DIMENSIONLESS
};
pub type DryVolume<N> = Qty<DRY_VOLUME, N>;

/// The dimension of `FluidVolume`.
pub const FLUID_VOLUME: Dim = Dim {
    m: 3,
    ..Dim::DIMENSIONLESS
};
pub type FluidVolume<N> = Qty<FLUID_VOLUME, N>;

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

    /// Acre (`[acr_us]`), a unit of `Area`: `n` of them is `n · 627264e5 / 15499969` coherent units.
    pub type Acre = UcumUnit<
        AREA,
        {
            UcumUnitConsts {
                size: Exact::int(62726400000) / Exact::int(15499969),
                zero: Exact::ZERO,
                symbol: "[acr_us]",
                name: "acre",
            }
        },
    >;

    /// Square rod (`[srd_us]`), a unit of `Area`: `n` of them is `n · 39204e4 / 15499969` coherent units.
    pub type SquareRod = UcumUnit<
        AREA,
        {
            UcumUnitConsts {
                size: Exact::int(392040000) / Exact::int(15499969),
                zero: Exact::ZERO,
                symbol: "[srd_us]",
                name: "square rod",
            }
        },
    >;

    /// Square mile (`[smi_us]`), a unit of `Area`: `n` of them is `n · 40144896e6 / 15499969` coherent units.
    pub type SquareMile = UcumUnit<
        AREA,
        {
            UcumUnitConsts {
                size: Exact::int(40144896000000) / Exact::int(15499969),
                zero: Exact::ZERO,
                symbol: "[smi_us]",
                name: "square mile",
            }
        },
    >;

    /// Section (`[sct]`), a unit of `Area`: `n` of them is `n · 40144896e6 / 15499969` coherent units.
    pub type Section = UcumUnit<
        AREA,
        {
            UcumUnitConsts {
                size: Exact::int(40144896000000) / Exact::int(15499969),
                zero: Exact::ZERO,
                symbol: "[sct]",
                name: "section",
            }
        },
    >;

    /// Township (`[twp]`), a unit of `Area`: `n` of them is `n · 1445216256e6 / 15499969` coherent units.
    pub type Township = UcumUnit<
        AREA,
        {
            UcumUnitConsts {
                size: Exact::int(1445216256000000) / Exact::int(15499969),
                zero: Exact::ZERO,
                symbol: "[twp]",
                name: "township",
            }
        },
    >;

    /// Bushel (`[bu_us]`), a unit of `DryVolume`: `n` of them is `n · 3523907016688e-14` coherent units.
    pub type Bushel = UcumUnit<
        DRY_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(3523907016688, -14),
                zero: Exact::ZERO,
                symbol: "[bu_us]",
                name: "bushel",
            }
        },
    >;

    /// Historical winchester gallon (`[gal_wi]`), a unit of `DryVolume`: `n` of them is `n · 440488377086e-14` coherent units.
    pub type HistoricalWinchesterGallon = UcumUnit<
        DRY_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(440488377086, -14),
                zero: Exact::ZERO,
                symbol: "[gal_wi]",
                name: "historical winchester gallon",
            }
        },
    >;

    /// Peck (`[pk_us]`), a unit of `DryVolume`: `n` of them is `n · 880976754172e-14` coherent units.
    pub type Peck = UcumUnit<
        DRY_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(880976754172, -14),
                zero: Exact::ZERO,
                symbol: "[pk_us]",
                name: "peck",
            }
        },
    >;

    /// Dry quart (`[dqt_us]`), a unit of `DryVolume`: `n` of them is `n · 1101220942715e-15` coherent units.
    pub type DryQuart = UcumUnit<
        DRY_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(1101220942715, -15),
                zero: Exact::ZERO,
                symbol: "[dqt_us]",
                name: "dry quart",
            }
        },
    >;

    /// Dry pint (`[dpt_us]`), a unit of `DryVolume`: `n` of them is `n · 5506104713575e-16` coherent units.
    pub type DryPint = UcumUnit<
        DRY_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(5506104713575, -16),
                zero: Exact::ZERO,
                symbol: "[dpt_us]",
                name: "dry pint",
            }
        },
    >;

    /// Queen Anne's wine gallon (`[gal_us]`), a unit of `FluidVolume`: `n` of them is `n · 3785411784e-12` coherent units.
    pub type QueenAnneSWineGallon = UcumUnit<
        FLUID_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(3785411784, -12),
                zero: Exact::ZERO,
                symbol: "[gal_us]",
                name: "Queen Anne's wine gallon",
            }
        },
    >;

    /// Barrel (`[bbl_us]`), a unit of `FluidVolume`: `n` of them is `n · 158987294928e-12` coherent units.
    pub type Barrel = UcumUnit<
        FLUID_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(158987294928, -12),
                zero: Exact::ZERO,
                symbol: "[bbl_us]",
                name: "barrel",
            }
        },
    >;

    /// Quart (`[qt_us]`), a unit of `FluidVolume`: `n` of them is `n · 946352946e-12` coherent units.
    pub type Quart = UcumUnit<
        FLUID_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(946352946, -12),
                zero: Exact::ZERO,
                symbol: "[qt_us]",
                name: "quart",
            }
        },
    >;

    /// Pint (`[pt_us]`), a unit of `FluidVolume`: `n` of them is `n · 473176473e-12` coherent units.
    pub type Pint = UcumUnit<
        FLUID_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(473176473, -12),
                zero: Exact::ZERO,
                symbol: "[pt_us]",
                name: "pint",
            }
        },
    >;

    /// Gill (`[gil_us]`), a unit of `FluidVolume`: `n` of them is `n · 11829411825e-14` coherent units.
    pub type Gill = UcumUnit<
        FLUID_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(11829411825, -14),
                zero: Exact::ZERO,
                symbol: "[gil_us]",
                name: "gill",
            }
        },
    >;

    /// Fluid ounce (`[foz_us]`), a unit of `FluidVolume`: `n` of them is `n · 295735295625e-16` coherent units.
    pub type FluidOunce = UcumUnit<
        FLUID_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(295735295625, -16),
                zero: Exact::ZERO,
                symbol: "[foz_us]",
                name: "fluid ounce",
            }
        },
    >;

    /// Fluid dram (`[fdr_us]`), a unit of `FluidVolume`: `n` of them is `n · 36966911953125e-19` coherent units.
    pub type FluidDram = UcumUnit<
        FLUID_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(36966911953125, -19),
                zero: Exact::ZERO,
                symbol: "[fdr_us]",
                name: "fluid dram",
            }
        },
    >;

    /// Minim (`[min_us]`), a unit of `FluidVolume`: `n` of them is `n · 61611519921875e-21` coherent units.
    pub type Minim = UcumUnit<
        FLUID_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(61611519921875, -21),
                zero: Exact::ZERO,
                symbol: "[min_us]",
                name: "minim",
            }
        },
    >;

    /// Cord (`[crd_us]`), a unit of `FluidVolume`: `n` of them is `n · 3624556363776e-12` coherent units.
    pub type Cord = UcumUnit<
        FLUID_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(3624556363776, -12),
                zero: Exact::ZERO,
                symbol: "[crd_us]",
                name: "cord",
            }
        },
    >;

    /// Metric fluid ounce (`[foz_m]`), a unit of `FluidVolume`: `n` of them is `n · 3e-5` coherent units.
    pub type MetricFluidOunce = UcumUnit<
        FLUID_VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(3, -5),
                zero: Exact::ZERO,
                symbol: "[foz_m]",
                name: "metric fluid ounce",
            }
        },
    >;

    /// Foot (`[ft_us]`), a unit of `Length`: `n` of them is `n · 12e2 / 3937` coherent units.
    pub type Foot = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(1200) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "[ft_us]",
                name: "foot",
            }
        },
    >;

    /// Yard (`[yd_us]`), a unit of `Length`: `n` of them is `n · 36e2 / 3937` coherent units.
    pub type Yard = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(3600) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "[yd_us]",
                name: "yard",
            }
        },
    >;

    /// Inch (`[in_us]`), a unit of `Length`: `n` of them is `n · 1e2 / 3937` coherent units.
    pub type Inch = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(100) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "[in_us]",
                name: "inch",
            }
        },
    >;

    /// Rod (`[rd_us]`), a unit of `Length`: `n` of them is `n · 198e2 / 3937` coherent units.
    pub type Rod = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(19800) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "[rd_us]",
                name: "rod",
            }
        },
    >;

    /// Surveyor's chain (`[ch_us]`), a unit of `Length`: `n` of them is `n · 792e2 / 3937` coherent units.
    pub type SurveyorSChain = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(79200) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "[ch_us]",
                name: "Surveyor's chain",
            }
        },
    >;

    /// Link for Gunter's chain (`[lk_us]`), a unit of `Length`: `n` of them is `n · 792 / 3937` coherent units.
    pub type LinkForGunterSChain = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(792) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "[lk_us]",
                name: "link for Gunter's chain",
            }
        },
    >;

    /// Engineer's chain (`[rch_us]`), a unit of `Length`: `n` of them is `n · 12e4 / 3937` coherent units.
    pub type EngineerSChain = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(120000) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "[rch_us]",
                name: "Engineer's chain",
            }
        },
    >;

    /// Link for Ramden's chain (`[rlk_us]`), a unit of `Length`: `n` of them is `n · 12e2 / 3937` coherent units.
    pub type LinkForRamdenSChain = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(1200) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "[rlk_us]",
                name: "link for Ramden's chain",
            }
        },
    >;

    /// Fathom (`[fth_us]`), a unit of `Length`: `n` of them is `n · 72e2 / 3937` coherent units.
    pub type Fathom = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(7200) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "[fth_us]",
                name: "fathom",
            }
        },
    >;

    /// Furlong (`[fur_us]`), a unit of `Length`: `n` of them is `n · 792e3 / 3937` coherent units.
    pub type Furlong = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(792000) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "[fur_us]",
                name: "furlong",
            }
        },
    >;

    /// Mile (`[mi_us]`), a unit of `Length`: `n` of them is `n · 6336e3 / 3937` coherent units.
    pub type Mile = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(6336000) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "[mi_us]",
                name: "mile",
            }
        },
    >;

    /// Mil (`[mil_us]`), a unit of `Length`: `n` of them is `n · 1 / 39370` coherent units.
    pub type Mil = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(1) / Exact::int(39370),
                zero: Exact::ZERO,
                symbol: "[mil_us]",
                name: "mil",
            }
        },
    >;

    /// Grain (`[gr]`), a unit of `Mass`: `n` of them is `n · 6479891e-8` coherent units.
    pub type Grain = UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(6479891, -8),
                zero: Exact::ZERO,
                symbol: "[gr]",
                name: "grain",
            }
        },
    >;

    /// Pound (`[lb_av]`), a unit of `Mass`: `n` of them is `n · 45359237e-5` coherent units.
    pub type Pound = UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(45359237, -5),
                zero: Exact::ZERO,
                symbol: "[lb_av]",
                name: "pound",
            }
        },
    >;

    /// Ounce (`[oz_av]`), a unit of `Mass`: `n` of them is `n · 28349523125e-9` coherent units.
    pub type Ounce = UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(28349523125, -9),
                zero: Exact::ZERO,
                symbol: "[oz_av]",
                name: "ounce",
            }
        },
    >;

    /// Dram (`[dr_av]`), a unit of `Mass`: `n` of them is `n · 17718451953125e-13` coherent units.
    pub type Dram = UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(17718451953125, -13),
                zero: Exact::ZERO,
                symbol: "[dr_av]",
                name: "dram",
            }
        },
    >;

    /// U.S. hundredweight (`[scwt_av]`), a unit of `Mass`: `n` of them is `n · 45359237e-3` coherent units.
    pub type USHundredweight = UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(45359237, -3),
                zero: Exact::ZERO,
                symbol: "[scwt_av]",
                name: "U.S. hundredweight",
            }
        },
    >;

    /// British hundredweight (`[lcwt_av]`), a unit of `Mass`: `n` of them is `n · 5080234544e-5` coherent units.
    pub type BritishHundredweight = UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(5080234544, -5),
                zero: Exact::ZERO,
                symbol: "[lcwt_av]",
                name: "British hundredweight",
            }
        },
    >;

    /// U.S. ton (`[ston_av]`), a unit of `Mass`: `n` of them is `n · 90718474e-2` coherent units.
    pub type USTon = UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(90718474, -2),
                zero: Exact::ZERO,
                symbol: "[ston_av]",
                name: "U.S. ton",
            }
        },
    >;

    /// British ton (`[lton_av]`), a unit of `Mass`: `n` of them is `n · 10160469088e-4` coherent units.
    pub type BritishTon = UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(10160469088, -4),
                zero: Exact::ZERO,
                symbol: "[lton_av]",
                name: "British ton",
            }
        },
    >;

    /// British stone (`[stone_av]`), a unit of `Mass`: `n` of them is `n · 635029318e-5` coherent units.
    pub type BritishStone = UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(635029318, -5),
                zero: Exact::ZERO,
                symbol: "[stone_av]",
                name: "British stone",
            }
        },
    >;

    /// Pennyweight (`[pwt_tr]`), a unit of `Mass`: `n` of them is `n · 155517384e-8` coherent units.
    pub type Pennyweight = UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(155517384, -8),
                zero: Exact::ZERO,
                symbol: "[pwt_tr]",
                name: "pennyweight",
            }
        },
    >;

    /// Scruple (`[sc_ap]`), a unit of `Mass`: `n` of them is `n · 12959782e-7` coherent units.
    pub type Scruple = UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(12959782, -7),
                zero: Exact::ZERO,
                symbol: "[sc_ap]",
                name: "scruple",
            }
        },
    >;

    /// Drachm (`[dr_ap]`), a unit of `Mass`: `n` of them is `n · 38879346e-7` coherent units.
    pub type Drachm = UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(38879346, -7),
                zero: Exact::ZERO,
                symbol: "[dr_ap]",
                name: "drachm",
            }
        },
    >;

    /// Metric ounce (`[oz_m]`), a unit of `Mass`: `n` of them is `n · 28` coherent units.
    pub type MetricOunce = UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::int(28),
                zero: Exact::ZERO,
                symbol: "[oz_m]",
                name: "metric ounce",
            }
        },
    >;

    /// Tablespoon (`[tbs_us]`), a unit of `Volume`: `n` of them is `n · 1478676478125e-17` coherent units.
    pub type Tablespoon = UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(1478676478125, -17),
                zero: Exact::ZERO,
                symbol: "[tbs_us]",
                name: "tablespoon",
            }
        },
    >;

    /// Teaspoon (`[tsp_us]`), a unit of `Volume`: `n` of them is `n · 492892159375e-17` coherent units.
    pub type Teaspoon = UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(492892159375, -17),
                zero: Exact::ZERO,
                symbol: "[tsp_us]",
                name: "teaspoon",
            }
        },
    >;

    /// Cup (`[cup_us]`), a unit of `Volume`: `n` of them is `n · 2365882365e-13` coherent units.
    pub type Cup = UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(2365882365, -13),
                zero: Exact::ZERO,
                symbol: "[cup_us]",
                name: "cup",
            }
        },
    >;

    /// Metric cup (`[cup_m]`), a unit of `Volume`: `n` of them is `n · 24e-5` coherent units.
    pub type MetricCup = UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(24, -5),
                zero: Exact::ZERO,
                symbol: "[cup_m]",
                name: "metric cup",
            }
        },
    >;

    /// Metric teaspoon (`[tsp_m]`), a unit of `Volume`: `n` of them is `n · 5e-6` coherent units.
    pub type MetricTeaspoon = UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(5, -6),
                zero: Exact::ZERO,
                symbol: "[tsp_m]",
                name: "metric teaspoon",
            }
        },
    >;

    /// Metric tablespoon (`[tbs_m]`), a unit of `Volume`: `n` of them is `n · 15e-6` coherent units.
    pub type MetricTablespoon = UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(15, -6),
                zero: Exact::ZERO,
                symbol: "[tbs_m]",
                name: "metric tablespoon",
            }
        },
    >;
}
