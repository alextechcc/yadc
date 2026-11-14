//! The quantities of `us` and their units.

#[allow(unused_imports)]
use crate::*;

// `Area` is `crate::Area`.

/// Acre (`[acr_us]`), a unit of `Area`: `n` of them is `n · 627264e5 / 15499969` base units.
pub const ACRE: Unit<
    {
        UcumUnit::scaled(
            "[acr_us]",
            Exact::int(62726400000) / Exact::int(15499969),
            METER_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// Square rod (`[srd_us]`), a unit of `Area`: `n` of them is `n · 39204e4 / 15499969` base units.
pub const SQUARE_ROD: Unit<
    {
        UcumUnit::scaled(
            "[srd_us]",
            Exact::int(392040000) / Exact::int(15499969),
            METER_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// Square mile (`[smi_us]`), a unit of `Area`: `n` of them is `n · 40144896e6 / 15499969` base units.
pub const SQUARE_MILE: Unit<
    {
        UcumUnit::scaled(
            "[smi_us]",
            Exact::int(40144896000000) / Exact::int(15499969),
            METER_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// Section (`[sct]`), a unit of `Area`: `n` of them is `n · 40144896e6 / 15499969` base units.
pub const SECTION: Unit<
    {
        UcumUnit::scaled(
            "[sct]",
            Exact::int(40144896000000) / Exact::int(15499969),
            METER_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// Township (`[twp]`), a unit of `Area`: `n` of them is `n · 1445216256e6 / 15499969` base units.
pub const TOWNSHIP: Unit<
    {
        UcumUnit::scaled(
            "[twp]",
            Exact::int(1445216256000000) / Exact::int(15499969),
            METER_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// A dry volume, in `N`s of the unit `U`: by default its base unit, `METER_CUBED`.
pub type DryVolume<N, const U: UcumUnit = { METER_CUBED.unit() }, const B: Base = BASE> =
    Qty<U, N, B>;

/// Bushel (`[bu_us]`), a unit of `DryVolume`: `n` of them is `n · 3523907016688e-14` base units.
pub const BUSHEL: Unit<
    {
        UcumUnit::scaled(
            "[bu_us]",
            Exact::dec(3523907016688, -14),
            METER_CUBED.unit(),
        )
    },
> = Unit::ONE;

/// Historical winchester gallon (`[gal_wi]`), a unit of `DryVolume`: `n` of them is `n · 440488377086e-14` base units.
pub const HISTORICAL_WINCHESTER_GALLON: Unit<
    {
        UcumUnit::scaled(
            "[gal_wi]",
            Exact::dec(440488377086, -14),
            METER_CUBED.unit(),
        )
    },
> = Unit::ONE;

/// Peck (`[pk_us]`), a unit of `DryVolume`: `n` of them is `n · 880976754172e-14` base units.
pub const PECK: Unit<
    { UcumUnit::scaled("[pk_us]", Exact::dec(880976754172, -14), METER_CUBED.unit()) },
> = Unit::ONE;

/// Dry quart (`[dqt_us]`), a unit of `DryVolume`: `n` of them is `n · 1101220942715e-15` base units.
pub const DRY_QUART: Unit<
    {
        UcumUnit::scaled(
            "[dqt_us]",
            Exact::dec(1101220942715, -15),
            METER_CUBED.unit(),
        )
    },
> = Unit::ONE;

/// Dry pint (`[dpt_us]`), a unit of `DryVolume`: `n` of them is `n · 5506104713575e-16` base units.
pub const DRY_PINT: Unit<
    {
        UcumUnit::scaled(
            "[dpt_us]",
            Exact::dec(5506104713575, -16),
            METER_CUBED.unit(),
        )
    },
> = Unit::ONE;

/// A fluid volume, in `N`s of the unit `U`: by default its base unit, `METER_CUBED`.
pub type FluidVolume<N, const U: UcumUnit = { METER_CUBED.unit() }, const B: Base = BASE> =
    Qty<U, N, B>;

/// Queen Anne's wine gallon (`[gal_us]`), a unit of `FluidVolume`: `n` of them is `n · 3785411784e-12` base units.
pub const QUEEN_ANNE_S_WINE_GALLON: Unit<
    { UcumUnit::scaled("[gal_us]", Exact::dec(3785411784, -12), METER_CUBED.unit()) },
> = Unit::ONE;

/// Barrel (`[bbl_us]`), a unit of `FluidVolume`: `n` of them is `n · 158987294928e-12` base units.
pub const BARREL: Unit<
    {
        UcumUnit::scaled(
            "[bbl_us]",
            Exact::dec(158987294928, -12),
            METER_CUBED.unit(),
        )
    },
> = Unit::ONE;

/// Quart (`[qt_us]`), a unit of `FluidVolume`: `n` of them is `n · 946352946e-12` base units.
pub const QUART: Unit<
    { UcumUnit::scaled("[qt_us]", Exact::dec(946352946, -12), METER_CUBED.unit()) },
> = Unit::ONE;

/// Pint (`[pt_us]`), a unit of `FluidVolume`: `n` of them is `n · 473176473e-12` base units.
pub const PINT: Unit<
    { UcumUnit::scaled("[pt_us]", Exact::dec(473176473, -12), METER_CUBED.unit()) },
> = Unit::ONE;

/// Gill (`[gil_us]`), a unit of `FluidVolume`: `n` of them is `n · 11829411825e-14` base units.
pub const GILL: Unit<
    { UcumUnit::scaled("[gil_us]", Exact::dec(11829411825, -14), METER_CUBED.unit()) },
> = Unit::ONE;

/// Fluid ounce (`[foz_us]`), a unit of `FluidVolume`: `n` of them is `n · 295735295625e-16` base units.
pub const FLUID_OUNCE: Unit<
    {
        UcumUnit::scaled(
            "[foz_us]",
            Exact::dec(295735295625, -16),
            METER_CUBED.unit(),
        )
    },
> = Unit::ONE;

/// Fluid dram (`[fdr_us]`), a unit of `FluidVolume`: `n` of them is `n · 36966911953125e-19` base units.
pub const FLUID_DRAM: Unit<
    {
        UcumUnit::scaled(
            "[fdr_us]",
            Exact::dec(36966911953125, -19),
            METER_CUBED.unit(),
        )
    },
> = Unit::ONE;

/// Minim (`[min_us]`), a unit of `FluidVolume`: `n` of them is `n · 61611519921875e-21` base units.
pub const MINIM: Unit<
    {
        UcumUnit::scaled(
            "[min_us]",
            Exact::dec(61611519921875, -21),
            METER_CUBED.unit(),
        )
    },
> = Unit::ONE;

/// Cord (`[crd_us]`), a unit of `FluidVolume`: `n` of them is `n · 3624556363776e-12` base units.
pub const CORD: Unit<
    {
        UcumUnit::scaled(
            "[crd_us]",
            Exact::dec(3624556363776, -12),
            METER_CUBED.unit(),
        )
    },
> = Unit::ONE;

/// Metric fluid ounce (`[foz_m]`), a unit of `FluidVolume`: `n` of them is `n · 3e-5` base units.
pub const METRIC_FLUID_OUNCE: Unit<
    { UcumUnit::scaled("[foz_m]", Exact::dec(3, -5), METER_CUBED.unit()) },
> = Unit::ONE;

// `Length` is `crate::Length`.

/// Foot (`[ft_us]`), a unit of `Length`: `n` of them is `n · 12e2 / 3937` base units.
pub const FOOT: Unit<
    { UcumUnit::scaled("[ft_us]", Exact::int(1200) / Exact::int(3937), METER.unit()) },
> = Unit::ONE;

/// Yard (`[yd_us]`), a unit of `Length`: `n` of them is `n · 36e2 / 3937` base units.
pub const YARD: Unit<
    { UcumUnit::scaled("[yd_us]", Exact::int(3600) / Exact::int(3937), METER.unit()) },
> = Unit::ONE;

/// Inch (`[in_us]`), a unit of `Length`: `n` of them is `n · 1e2 / 3937` base units.
pub const INCH: Unit<
    { UcumUnit::scaled("[in_us]", Exact::int(100) / Exact::int(3937), METER.unit()) },
> = Unit::ONE;

/// Rod (`[rd_us]`), a unit of `Length`: `n` of them is `n · 198e2 / 3937` base units.
pub const ROD: Unit<
    {
        UcumUnit::scaled(
            "[rd_us]",
            Exact::int(19800) / Exact::int(3937),
            METER.unit(),
        )
    },
> = Unit::ONE;

/// Surveyor's chain (`[ch_us]`), a unit of `Length`: `n` of them is `n · 792e2 / 3937` base units.
pub const SURVEYOR_S_CHAIN: Unit<
    {
        UcumUnit::scaled(
            "[ch_us]",
            Exact::int(79200) / Exact::int(3937),
            METER.unit(),
        )
    },
> = Unit::ONE;

/// Link for Gunter's chain (`[lk_us]`), a unit of `Length`: `n` of them is `n · 792 / 3937` base units.
pub const LINK_FOR_GUNTER_S_CHAIN: Unit<
    { UcumUnit::scaled("[lk_us]", Exact::int(792) / Exact::int(3937), METER.unit()) },
> = Unit::ONE;

/// Engineer's chain (`[rch_us]`), a unit of `Length`: `n` of them is `n · 12e4 / 3937` base units.
pub const ENGINEER_S_CHAIN: Unit<
    {
        UcumUnit::scaled(
            "[rch_us]",
            Exact::int(120000) / Exact::int(3937),
            METER.unit(),
        )
    },
> = Unit::ONE;

/// Link for Ramden's chain (`[rlk_us]`), a unit of `Length`: `n` of them is `n · 12e2 / 3937` base units.
pub const LINK_FOR_RAMDEN_S_CHAIN: Unit<
    {
        UcumUnit::scaled(
            "[rlk_us]",
            Exact::int(1200) / Exact::int(3937),
            METER.unit(),
        )
    },
> = Unit::ONE;

/// Fathom (`[fth_us]`), a unit of `Length`: `n` of them is `n · 72e2 / 3937` base units.
pub const FATHOM: Unit<
    {
        UcumUnit::scaled(
            "[fth_us]",
            Exact::int(7200) / Exact::int(3937),
            METER.unit(),
        )
    },
> = Unit::ONE;

/// Furlong (`[fur_us]`), a unit of `Length`: `n` of them is `n · 792e3 / 3937` base units.
pub const FURLONG: Unit<
    {
        UcumUnit::scaled(
            "[fur_us]",
            Exact::int(792000) / Exact::int(3937),
            METER.unit(),
        )
    },
> = Unit::ONE;

/// Mile (`[mi_us]`), a unit of `Length`: `n` of them is `n · 6336e3 / 3937` base units.
pub const MILE: Unit<
    {
        UcumUnit::scaled(
            "[mi_us]",
            Exact::int(6336000) / Exact::int(3937),
            METER.unit(),
        )
    },
> = Unit::ONE;

/// Mil (`[mil_us]`), a unit of `Length`: `n` of them is `n · 1 / 39370` base units.
pub const MIL: Unit<
    { UcumUnit::scaled("[mil_us]", Exact::int(1) / Exact::int(39370), METER.unit()) },
> = Unit::ONE;

// `Mass` is `crate::Mass`.

/// Grain (`[gr]`), a unit of `Mass`: `n` of them is `n · 6479891e-8` base units.
pub const GRAIN: Unit<{ UcumUnit::scaled("[gr]", Exact::dec(6479891, -8), GRAM.unit()) }> =
    Unit::ONE;

/// Pound (`[lb_av]`), a unit of `Mass`: `n` of them is `n · 45359237e-5` base units.
pub const POUND: Unit<{ UcumUnit::scaled("[lb_av]", Exact::dec(45359237, -5), GRAM.unit()) }> =
    Unit::ONE;

/// Ounce (`[oz_av]`), a unit of `Mass`: `n` of them is `n · 28349523125e-9` base units.
pub const OUNCE: Unit<{ UcumUnit::scaled("[oz_av]", Exact::dec(28349523125, -9), GRAM.unit()) }> =
    Unit::ONE;

/// Dram (`[dr_av]`), a unit of `Mass`: `n` of them is `n · 17718451953125e-13` base units.
pub const DRAM: Unit<
    { UcumUnit::scaled("[dr_av]", Exact::dec(17718451953125, -13), GRAM.unit()) },
> = Unit::ONE;

/// U.S. hundredweight (`[scwt_av]`), a unit of `Mass`: `n` of them is `n · 45359237e-3` base units.
pub const US_HUNDREDWEIGHT: Unit<
    { UcumUnit::scaled("[scwt_av]", Exact::dec(45359237, -3), GRAM.unit()) },
> = Unit::ONE;

/// British hundredweight (`[lcwt_av]`), a unit of `Mass`: `n` of them is `n · 5080234544e-5` base units.
pub const BRITISH_HUNDREDWEIGHT: Unit<
    { UcumUnit::scaled("[lcwt_av]", Exact::dec(5080234544, -5), GRAM.unit()) },
> = Unit::ONE;

/// U.S. ton (`[ston_av]`), a unit of `Mass`: `n` of them is `n · 90718474e-2` base units.
pub const US_TON: Unit<{ UcumUnit::scaled("[ston_av]", Exact::dec(90718474, -2), GRAM.unit()) }> =
    Unit::ONE;

/// British ton (`[lton_av]`), a unit of `Mass`: `n` of them is `n · 10160469088e-4` base units.
pub const BRITISH_TON: Unit<
    { UcumUnit::scaled("[lton_av]", Exact::dec(10160469088, -4), GRAM.unit()) },
> = Unit::ONE;

/// British stone (`[stone_av]`), a unit of `Mass`: `n` of them is `n · 635029318e-5` base units.
pub const BRITISH_STONE: Unit<
    { UcumUnit::scaled("[stone_av]", Exact::dec(635029318, -5), GRAM.unit()) },
> = Unit::ONE;

/// Pennyweight (`[pwt_tr]`), a unit of `Mass`: `n` of them is `n · 155517384e-8` base units.
pub const PENNYWEIGHT: Unit<
    { UcumUnit::scaled("[pwt_tr]", Exact::dec(155517384, -8), GRAM.unit()) },
> = Unit::ONE;

/// Scruple (`[sc_ap]`), a unit of `Mass`: `n` of them is `n · 12959782e-7` base units.
pub const SCRUPLE: Unit<{ UcumUnit::scaled("[sc_ap]", Exact::dec(12959782, -7), GRAM.unit()) }> =
    Unit::ONE;

/// Drachm (`[dr_ap]`), a unit of `Mass`: `n` of them is `n · 38879346e-7` base units.
pub const DRACHM: Unit<{ UcumUnit::scaled("[dr_ap]", Exact::dec(38879346, -7), GRAM.unit()) }> =
    Unit::ONE;

/// Metric ounce (`[oz_m]`), a unit of `Mass`: `n` of them is `n · 28` base units.
pub const METRIC_OUNCE: Unit<{ UcumUnit::scaled("[oz_m]", Exact::int(28), GRAM.unit()) }> =
    Unit::ONE;

// `Volume` is `crate::Volume`.

/// Tablespoon (`[tbs_us]`), a unit of `Volume`: `n` of them is `n · 1478676478125e-17` base units.
pub const TABLESPOON: Unit<
    {
        UcumUnit::scaled(
            "[tbs_us]",
            Exact::dec(1478676478125, -17),
            METER_CUBED.unit(),
        )
    },
> = Unit::ONE;

/// Teaspoon (`[tsp_us]`), a unit of `Volume`: `n` of them is `n · 492892159375e-17` base units.
pub const TEASPOON: Unit<
    {
        UcumUnit::scaled(
            "[tsp_us]",
            Exact::dec(492892159375, -17),
            METER_CUBED.unit(),
        )
    },
> = Unit::ONE;

/// Cup (`[cup_us]`), a unit of `Volume`: `n` of them is `n · 2365882365e-13` base units.
pub const CUP: Unit<
    { UcumUnit::scaled("[cup_us]", Exact::dec(2365882365, -13), METER_CUBED.unit()) },
> = Unit::ONE;

/// Metric cup (`[cup_m]`), a unit of `Volume`: `n` of them is `n · 24e-5` base units.
pub const METRIC_CUP: Unit<
    { UcumUnit::scaled("[cup_m]", Exact::dec(24, -5), METER_CUBED.unit()) },
> = Unit::ONE;

/// Metric teaspoon (`[tsp_m]`), a unit of `Volume`: `n` of them is `n · 5e-6` base units.
pub const METRIC_TEASPOON: Unit<
    { UcumUnit::scaled("[tsp_m]", Exact::dec(5, -6), METER_CUBED.unit()) },
> = Unit::ONE;

/// Metric tablespoon (`[tbs_m]`), a unit of `Volume`: `n` of them is `n · 15e-6` base units.
pub const METRIC_TABLESPOON: Unit<
    { UcumUnit::scaled("[tbs_m]", Exact::dec(15, -6), METER_CUBED.unit()) },
> = Unit::ONE;
