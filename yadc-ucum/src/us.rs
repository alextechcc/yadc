//! The quantities of `us` and their units.

#[allow(unused_imports)]
use crate::*;

/// An area, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Area<
    N,
    const U: UcumUnit = { UcumUnit::ONE.times(METER.unit(), 2) },
    const IS_BASE: bool = true,
> = Qty<U, N, IS_BASE>;

/// Acre (`[acr_us]`), a unit of `Area`: `n` of them is `n · 627264e5 / 15499969` base units.
pub const ACRE: Qty<
    {
        UcumUnit::scaled(
            "[acr_us]",
            Exact::int(62726400000) / Exact::int(15499969),
            UcumUnit::ONE.times(METER.unit(), 2),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Square rod (`[srd_us]`), a unit of `Area`: `n` of them is `n · 39204e4 / 15499969` base units.
pub const SQUARE_ROD: Qty<
    {
        UcumUnit::scaled(
            "[srd_us]",
            Exact::int(392040000) / Exact::int(15499969),
            UcumUnit::ONE.times(METER.unit(), 2),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Square mile (`[smi_us]`), a unit of `Area`: `n` of them is `n · 40144896e6 / 15499969` base units.
pub const SQUARE_MILE: Qty<
    {
        UcumUnit::scaled(
            "[smi_us]",
            Exact::int(40144896000000) / Exact::int(15499969),
            UcumUnit::ONE.times(METER.unit(), 2),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Section (`[sct]`), a unit of `Area`: `n` of them is `n · 40144896e6 / 15499969` base units.
pub const SECTION: Qty<
    {
        UcumUnit::scaled(
            "[sct]",
            Exact::int(40144896000000) / Exact::int(15499969),
            UcumUnit::ONE.times(METER.unit(), 2),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Township (`[twp]`), a unit of `Area`: `n` of them is `n · 1445216256e6 / 15499969` base units.
pub const TOWNSHIP: Qty<
    {
        UcumUnit::scaled(
            "[twp]",
            Exact::int(1445216256000000) / Exact::int(15499969),
            UcumUnit::ONE.times(METER.unit(), 2),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// A dry volume, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type DryVolume<
    N,
    const U: UcumUnit = { UcumUnit::ONE.times(METER.unit(), 3) },
    const IS_BASE: bool = true,
> = Qty<U, N, IS_BASE>;

/// Bushel (`[bu_us]`), a unit of `DryVolume`: `n` of them is `n · 3523907016688e-14` base units.
pub const BUSHEL: Qty<
    {
        UcumUnit::scaled(
            "[bu_us]",
            Exact::dec(3523907016688, -14),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Historical winchester gallon (`[gal_wi]`), a unit of `DryVolume`: `n` of them is `n · 440488377086e-14` base units.
pub const HISTORICAL_WINCHESTER_GALLON: Qty<
    {
        UcumUnit::scaled(
            "[gal_wi]",
            Exact::dec(440488377086, -14),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Peck (`[pk_us]`), a unit of `DryVolume`: `n` of them is `n · 880976754172e-14` base units.
pub const PECK: Qty<
    {
        UcumUnit::scaled(
            "[pk_us]",
            Exact::dec(880976754172, -14),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Dry quart (`[dqt_us]`), a unit of `DryVolume`: `n` of them is `n · 1101220942715e-15` base units.
pub const DRY_QUART: Qty<
    {
        UcumUnit::scaled(
            "[dqt_us]",
            Exact::dec(1101220942715, -15),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Dry pint (`[dpt_us]`), a unit of `DryVolume`: `n` of them is `n · 5506104713575e-16` base units.
pub const DRY_PINT: Qty<
    {
        UcumUnit::scaled(
            "[dpt_us]",
            Exact::dec(5506104713575, -16),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// A fluid volume, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type FluidVolume<
    N,
    const U: UcumUnit = { UcumUnit::ONE.times(METER.unit(), 3) },
    const IS_BASE: bool = true,
> = Qty<U, N, IS_BASE>;

/// Queen Anne's wine gallon (`[gal_us]`), a unit of `FluidVolume`: `n` of them is `n · 3785411784e-12` base units.
pub const QUEEN_ANNE_S_WINE_GALLON: Qty<
    {
        UcumUnit::scaled(
            "[gal_us]",
            Exact::dec(3785411784, -12),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Barrel (`[bbl_us]`), a unit of `FluidVolume`: `n` of them is `n · 158987294928e-12` base units.
pub const BARREL: Qty<
    {
        UcumUnit::scaled(
            "[bbl_us]",
            Exact::dec(158987294928, -12),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Quart (`[qt_us]`), a unit of `FluidVolume`: `n` of them is `n · 946352946e-12` base units.
pub const QUART: Qty<
    {
        UcumUnit::scaled(
            "[qt_us]",
            Exact::dec(946352946, -12),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Pint (`[pt_us]`), a unit of `FluidVolume`: `n` of them is `n · 473176473e-12` base units.
pub const PINT: Qty<
    {
        UcumUnit::scaled(
            "[pt_us]",
            Exact::dec(473176473, -12),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Gill (`[gil_us]`), a unit of `FluidVolume`: `n` of them is `n · 11829411825e-14` base units.
pub const GILL: Qty<
    {
        UcumUnit::scaled(
            "[gil_us]",
            Exact::dec(11829411825, -14),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Fluid ounce (`[foz_us]`), a unit of `FluidVolume`: `n` of them is `n · 295735295625e-16` base units.
pub const FLUID_OUNCE: Qty<
    {
        UcumUnit::scaled(
            "[foz_us]",
            Exact::dec(295735295625, -16),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Fluid dram (`[fdr_us]`), a unit of `FluidVolume`: `n` of them is `n · 36966911953125e-19` base units.
pub const FLUID_DRAM: Qty<
    {
        UcumUnit::scaled(
            "[fdr_us]",
            Exact::dec(36966911953125, -19),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Minim (`[min_us]`), a unit of `FluidVolume`: `n` of them is `n · 61611519921875e-21` base units.
pub const MINIM: Qty<
    {
        UcumUnit::scaled(
            "[min_us]",
            Exact::dec(61611519921875, -21),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Cord (`[crd_us]`), a unit of `FluidVolume`: `n` of them is `n · 3624556363776e-12` base units.
pub const CORD: Qty<
    {
        UcumUnit::scaled(
            "[crd_us]",
            Exact::dec(3624556363776, -12),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Metric fluid ounce (`[foz_m]`), a unit of `FluidVolume`: `n` of them is `n · 3e-5` base units.
pub const METRIC_FLUID_OUNCE: Qty<
    {
        UcumUnit::scaled(
            "[foz_m]",
            Exact::dec(3, -5),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// A length, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Length<N, const U: UcumUnit = { METER.unit() }, const IS_BASE: bool = true> =
    Qty<U, N, IS_BASE>;

/// Foot (`[ft_us]`), a unit of `Length`: `n` of them is `n · 12e2 / 3937` base units.
pub const FOOT: Qty<
    { UcumUnit::scaled("[ft_us]", Exact::int(1200) / Exact::int(3937), METER.unit()) },
    Exact,
    false,
> = Qty::ONE;

/// Yard (`[yd_us]`), a unit of `Length`: `n` of them is `n · 36e2 / 3937` base units.
pub const YARD: Qty<
    { UcumUnit::scaled("[yd_us]", Exact::int(3600) / Exact::int(3937), METER.unit()) },
    Exact,
    false,
> = Qty::ONE;

/// Inch (`[in_us]`), a unit of `Length`: `n` of them is `n · 1e2 / 3937` base units.
pub const INCH: Qty<
    { UcumUnit::scaled("[in_us]", Exact::int(100) / Exact::int(3937), METER.unit()) },
    Exact,
    false,
> = Qty::ONE;

/// Rod (`[rd_us]`), a unit of `Length`: `n` of them is `n · 198e2 / 3937` base units.
pub const ROD: Qty<
    {
        UcumUnit::scaled(
            "[rd_us]",
            Exact::int(19800) / Exact::int(3937),
            METER.unit(),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Surveyor's chain (`[ch_us]`), a unit of `Length`: `n` of them is `n · 792e2 / 3937` base units.
pub const SURVEYOR_S_CHAIN: Qty<
    {
        UcumUnit::scaled(
            "[ch_us]",
            Exact::int(79200) / Exact::int(3937),
            METER.unit(),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Link for Gunter's chain (`[lk_us]`), a unit of `Length`: `n` of them is `n · 792 / 3937` base units.
pub const LINK_FOR_GUNTER_S_CHAIN: Qty<
    { UcumUnit::scaled("[lk_us]", Exact::int(792) / Exact::int(3937), METER.unit()) },
    Exact,
    false,
> = Qty::ONE;

/// Engineer's chain (`[rch_us]`), a unit of `Length`: `n` of them is `n · 12e4 / 3937` base units.
pub const ENGINEER_S_CHAIN: Qty<
    {
        UcumUnit::scaled(
            "[rch_us]",
            Exact::int(120000) / Exact::int(3937),
            METER.unit(),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Link for Ramden's chain (`[rlk_us]`), a unit of `Length`: `n` of them is `n · 12e2 / 3937` base units.
pub const LINK_FOR_RAMDEN_S_CHAIN: Qty<
    {
        UcumUnit::scaled(
            "[rlk_us]",
            Exact::int(1200) / Exact::int(3937),
            METER.unit(),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Fathom (`[fth_us]`), a unit of `Length`: `n` of them is `n · 72e2 / 3937` base units.
pub const FATHOM: Qty<
    {
        UcumUnit::scaled(
            "[fth_us]",
            Exact::int(7200) / Exact::int(3937),
            METER.unit(),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Furlong (`[fur_us]`), a unit of `Length`: `n` of them is `n · 792e3 / 3937` base units.
pub const FURLONG: Qty<
    {
        UcumUnit::scaled(
            "[fur_us]",
            Exact::int(792000) / Exact::int(3937),
            METER.unit(),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Mile (`[mi_us]`), a unit of `Length`: `n` of them is `n · 6336e3 / 3937` base units.
pub const MILE: Qty<
    {
        UcumUnit::scaled(
            "[mi_us]",
            Exact::int(6336000) / Exact::int(3937),
            METER.unit(),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Mil (`[mil_us]`), a unit of `Length`: `n` of them is `n · 1 / 39370` base units.
pub const MIL: Qty<
    { UcumUnit::scaled("[mil_us]", Exact::int(1) / Exact::int(39370), METER.unit()) },
    Exact,
    false,
> = Qty::ONE;

/// A mass, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Mass<N, const U: UcumUnit = { GRAM.unit() }, const IS_BASE: bool = true> =
    Qty<U, N, IS_BASE>;

/// Grain (`[gr]`), a unit of `Mass`: `n` of them is `n · 6479891e-8` base units.
pub const GRAIN: Qty<
    { UcumUnit::scaled("[gr]", Exact::dec(6479891, -8), GRAM.unit()) },
    Exact,
    false,
> = Qty::ONE;

/// Pound (`[lb_av]`), a unit of `Mass`: `n` of them is `n · 45359237e-5` base units.
pub const POUND: Qty<
    { UcumUnit::scaled("[lb_av]", Exact::dec(45359237, -5), GRAM.unit()) },
    Exact,
    false,
> = Qty::ONE;

/// Ounce (`[oz_av]`), a unit of `Mass`: `n` of them is `n · 28349523125e-9` base units.
pub const OUNCE: Qty<
    { UcumUnit::scaled("[oz_av]", Exact::dec(28349523125, -9), GRAM.unit()) },
    Exact,
    false,
> = Qty::ONE;

/// Dram (`[dr_av]`), a unit of `Mass`: `n` of them is `n · 17718451953125e-13` base units.
pub const DRAM: Qty<
    { UcumUnit::scaled("[dr_av]", Exact::dec(17718451953125, -13), GRAM.unit()) },
    Exact,
    false,
> = Qty::ONE;

/// U.S. hundredweight (`[scwt_av]`), a unit of `Mass`: `n` of them is `n · 45359237e-3` base units.
pub const US_HUNDREDWEIGHT: Qty<
    { UcumUnit::scaled("[scwt_av]", Exact::dec(45359237, -3), GRAM.unit()) },
    Exact,
    false,
> = Qty::ONE;

/// British hundredweight (`[lcwt_av]`), a unit of `Mass`: `n` of them is `n · 5080234544e-5` base units.
pub const BRITISH_HUNDREDWEIGHT: Qty<
    { UcumUnit::scaled("[lcwt_av]", Exact::dec(5080234544, -5), GRAM.unit()) },
    Exact,
    false,
> = Qty::ONE;

/// U.S. ton (`[ston_av]`), a unit of `Mass`: `n` of them is `n · 90718474e-2` base units.
pub const US_TON: Qty<
    { UcumUnit::scaled("[ston_av]", Exact::dec(90718474, -2), GRAM.unit()) },
    Exact,
    false,
> = Qty::ONE;

/// British ton (`[lton_av]`), a unit of `Mass`: `n` of them is `n · 10160469088e-4` base units.
pub const BRITISH_TON: Qty<
    { UcumUnit::scaled("[lton_av]", Exact::dec(10160469088, -4), GRAM.unit()) },
    Exact,
    false,
> = Qty::ONE;

/// British stone (`[stone_av]`), a unit of `Mass`: `n` of them is `n · 635029318e-5` base units.
pub const BRITISH_STONE: Qty<
    { UcumUnit::scaled("[stone_av]", Exact::dec(635029318, -5), GRAM.unit()) },
    Exact,
    false,
> = Qty::ONE;

/// Pennyweight (`[pwt_tr]`), a unit of `Mass`: `n` of them is `n · 155517384e-8` base units.
pub const PENNYWEIGHT: Qty<
    { UcumUnit::scaled("[pwt_tr]", Exact::dec(155517384, -8), GRAM.unit()) },
    Exact,
    false,
> = Qty::ONE;

/// Scruple (`[sc_ap]`), a unit of `Mass`: `n` of them is `n · 12959782e-7` base units.
pub const SCRUPLE: Qty<
    { UcumUnit::scaled("[sc_ap]", Exact::dec(12959782, -7), GRAM.unit()) },
    Exact,
    false,
> = Qty::ONE;

/// Drachm (`[dr_ap]`), a unit of `Mass`: `n` of them is `n · 38879346e-7` base units.
pub const DRACHM: Qty<
    { UcumUnit::scaled("[dr_ap]", Exact::dec(38879346, -7), GRAM.unit()) },
    Exact,
    false,
> = Qty::ONE;

/// Metric ounce (`[oz_m]`), a unit of `Mass`: `n` of them is `n · 28` base units.
pub const METRIC_OUNCE: Qty<
    { UcumUnit::scaled("[oz_m]", Exact::int(28), GRAM.unit()) },
    Exact,
    false,
> = Qty::ONE;

/// A volume, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Volume<
    N,
    const U: UcumUnit = { UcumUnit::ONE.times(METER.unit(), 3) },
    const IS_BASE: bool = true,
> = Qty<U, N, IS_BASE>;

/// Tablespoon (`[tbs_us]`), a unit of `Volume`: `n` of them is `n · 1478676478125e-17` base units.
pub const TABLESPOON: Qty<
    {
        UcumUnit::scaled(
            "[tbs_us]",
            Exact::dec(1478676478125, -17),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Teaspoon (`[tsp_us]`), a unit of `Volume`: `n` of them is `n · 492892159375e-17` base units.
pub const TEASPOON: Qty<
    {
        UcumUnit::scaled(
            "[tsp_us]",
            Exact::dec(492892159375, -17),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Cup (`[cup_us]`), a unit of `Volume`: `n` of them is `n · 2365882365e-13` base units.
pub const CUP: Qty<
    {
        UcumUnit::scaled(
            "[cup_us]",
            Exact::dec(2365882365, -13),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Metric cup (`[cup_m]`), a unit of `Volume`: `n` of them is `n · 24e-5` base units.
pub const METRIC_CUP: Qty<
    {
        UcumUnit::scaled(
            "[cup_m]",
            Exact::dec(24, -5),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Metric teaspoon (`[tsp_m]`), a unit of `Volume`: `n` of them is `n · 5e-6` base units.
pub const METRIC_TEASPOON: Qty<
    {
        UcumUnit::scaled(
            "[tsp_m]",
            Exact::dec(5, -6),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    false,
> = Qty::ONE;

/// Metric tablespoon (`[tbs_m]`), a unit of `Volume`: `n` of them is `n · 15e-6` base units.
pub const METRIC_TABLESPOON: Qty<
    {
        UcumUnit::scaled(
            "[tbs_m]",
            Exact::dec(15, -6),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    false,
> = Qty::ONE;
