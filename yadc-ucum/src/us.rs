use crate::{Dim, Exact, Qty};
use consts::*;
use yadc::Dimension;

pub type Area<N> = Qty<
    {
        Dim {
            m: 2,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type DryVolume<N> = Qty<
    {
        Dim {
            m: 3,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type FluidVolume<N> = Qty<
    {
        Dim {
            m: 3,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Length<N> = Qty<
    {
        Dim {
            m: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Mass<N> = Qty<
    {
        Dim {
            g: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Volume<N> = Qty<
    {
        Dim {
            m: 3,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

/// The constants of this module: the base constants of its quantities
/// (re-exported), then one per named unit, each defined exactly from them
/// (`INCH` is `0.0254 * METRE`). A unit written as others is not one:
/// `SLUG / CUBIC_FOOT`, `KILO * METRE` are exact too. Converting rounds,
/// once, where it is written - in a `const`, by the compiler:
/// `const MPH: Velocity<f64> = (MILE / HOUR).into();`, then `MPH(60.0)`.
pub mod consts {
    use super::*;

    /// The metre, UCUM's base unit of length.
    pub use crate::base::METRE;

    /// The gram, UCUM's base unit of mass.
    pub use crate::base::GRAM;

    /// One `Acre` (`[acr_us]`), as an `Area`.
    pub const ACRE: Area<Exact> =
        (METRE * METRE) * (Exact::int(62726400000) / Exact::int(15499969));

    /// One `SquareRod` (`[srd_us]`), as an `Area`.
    pub const SQUARE_ROD: Area<Exact> =
        (METRE * METRE) * (Exact::int(392040000) / Exact::int(15499969));

    /// One `SquareMile` (`[smi_us]`), as an `Area`.
    pub const SQUARE_MILE: Area<Exact> =
        (METRE * METRE) * (Exact::int(40144896000000) / Exact::int(15499969));

    /// One `Section` (`[sct]`), as an `Area`.
    pub const SECTION: Area<Exact> =
        (METRE * METRE) * (Exact::int(40144896000000) / Exact::int(15499969));

    /// One `Township` (`[twp]`), as an `Area`.
    pub const TOWNSHIP: Area<Exact> =
        (METRE * METRE) * (Exact::int(1445216256000000) / Exact::int(15499969));

    /// One `Bushel` (`[bu_us]`), as a `DryVolume`.
    pub const BUSHEL: DryVolume<Exact> = (METRE * METRE * METRE) * Exact::dec(3523907016688, -14);

    /// One `HistoricalWinchesterGallon` (`[gal_wi]`), as a `DryVolume`.
    pub const HISTORICAL_WINCHESTER_GALLON: DryVolume<Exact> =
        (METRE * METRE * METRE) * Exact::dec(440488377086, -14);

    /// One `Peck` (`[pk_us]`), as a `DryVolume`.
    pub const PECK: DryVolume<Exact> = (METRE * METRE * METRE) * Exact::dec(880976754172, -14);

    /// One `DryQuart` (`[dqt_us]`), as a `DryVolume`.
    pub const DRY_QUART: DryVolume<Exact> =
        (METRE * METRE * METRE) * Exact::dec(1101220942715, -15);

    /// One `DryPint` (`[dpt_us]`), as a `DryVolume`.
    pub const DRY_PINT: DryVolume<Exact> = (METRE * METRE * METRE) * Exact::dec(5506104713575, -16);

    /// One `QueenAnneSWineGallon` (`[gal_us]`), as a `FluidVolume`.
    pub const QUEEN_ANNE_S_WINE_GALLON: FluidVolume<Exact> =
        (METRE * METRE * METRE) * Exact::dec(3785411784, -12);

    /// One `Barrel` (`[bbl_us]`), as a `FluidVolume`.
    pub const BARREL: FluidVolume<Exact> = (METRE * METRE * METRE) * Exact::dec(158987294928, -12);

    /// One `Quart` (`[qt_us]`), as a `FluidVolume`.
    pub const QUART: FluidVolume<Exact> = (METRE * METRE * METRE) * Exact::dec(946352946, -12);

    /// One `Pint` (`[pt_us]`), as a `FluidVolume`.
    pub const PINT: FluidVolume<Exact> = (METRE * METRE * METRE) * Exact::dec(473176473, -12);

    /// One `Gill` (`[gil_us]`), as a `FluidVolume`.
    pub const GILL: FluidVolume<Exact> = (METRE * METRE * METRE) * Exact::dec(11829411825, -14);

    /// One `FluidOunce` (`[foz_us]`), as a `FluidVolume`.
    pub const FLUID_OUNCE: FluidVolume<Exact> =
        (METRE * METRE * METRE) * Exact::dec(295735295625, -16);

    /// One `FluidDram` (`[fdr_us]`), as a `FluidVolume`.
    pub const FLUID_DRAM: FluidVolume<Exact> =
        (METRE * METRE * METRE) * Exact::dec(36966911953125, -19);

    /// One `Minim` (`[min_us]`), as a `FluidVolume`.
    pub const MINIM: FluidVolume<Exact> = (METRE * METRE * METRE) * Exact::dec(61611519921875, -21);

    /// One `Cord` (`[crd_us]`), as a `FluidVolume`.
    pub const CORD: FluidVolume<Exact> = (METRE * METRE * METRE) * Exact::dec(3624556363776, -12);

    /// One `MetricFluidOunce` (`[foz_m]`), as a `FluidVolume`.
    pub const METRIC_FLUID_OUNCE: FluidVolume<Exact> = (METRE * METRE * METRE) * Exact::dec(3, -5);

    /// One `Foot` (`[ft_us]`), as a `Length`.
    pub const FOOT: Length<Exact> = METRE * (Exact::int(1200) / Exact::int(3937));

    /// One `Yard` (`[yd_us]`), as a `Length`.
    pub const YARD: Length<Exact> = METRE * (Exact::int(3600) / Exact::int(3937));

    /// One `Inch` (`[in_us]`), as a `Length`.
    pub const INCH: Length<Exact> = METRE * (Exact::int(100) / Exact::int(3937));

    /// One `Rod` (`[rd_us]`), as a `Length`.
    pub const ROD: Length<Exact> = METRE * (Exact::int(19800) / Exact::int(3937));

    /// One `SurveyorSChain` (`[ch_us]`), as a `Length`.
    pub const SURVEYOR_S_CHAIN: Length<Exact> = METRE * (Exact::int(79200) / Exact::int(3937));

    /// One `LinkForGunterSChain` (`[lk_us]`), as a `Length`.
    pub const LINK_FOR_GUNTER_S_CHAIN: Length<Exact> = METRE * (Exact::int(792) / Exact::int(3937));

    /// One `EngineerSChain` (`[rch_us]`), as a `Length`.
    pub const ENGINEER_S_CHAIN: Length<Exact> = METRE * (Exact::int(120000) / Exact::int(3937));

    /// One `LinkForRamdenSChain` (`[rlk_us]`), as a `Length`.
    pub const LINK_FOR_RAMDEN_S_CHAIN: Length<Exact> =
        METRE * (Exact::int(1200) / Exact::int(3937));

    /// One `Fathom` (`[fth_us]`), as a `Length`.
    pub const FATHOM: Length<Exact> = METRE * (Exact::int(7200) / Exact::int(3937));

    /// One `Furlong` (`[fur_us]`), as a `Length`.
    pub const FURLONG: Length<Exact> = METRE * (Exact::int(792000) / Exact::int(3937));

    /// One `Mile` (`[mi_us]`), as a `Length`.
    pub const MILE: Length<Exact> = METRE * (Exact::int(6336000) / Exact::int(3937));

    /// One `Mil` (`[mil_us]`), as a `Length`.
    pub const MIL: Length<Exact> = METRE * (Exact::int(1) / Exact::int(39370));

    /// One `Grain` (`[gr]`), as a `Mass`.
    pub const GRAIN: Mass<Exact> = GRAM * Exact::dec(6479891, -8);

    /// One `Pound` (`[lb_av]`), as a `Mass`.
    pub const POUND: Mass<Exact> = GRAM * Exact::dec(45359237, -5);

    /// One `Ounce` (`[oz_av]`), as a `Mass`.
    pub const OUNCE: Mass<Exact> = GRAM * Exact::dec(28349523125, -9);

    /// One `Dram` (`[dr_av]`), as a `Mass`.
    pub const DRAM: Mass<Exact> = GRAM * Exact::dec(17718451953125, -13);

    /// One `USHundredweight` (`[scwt_av]`), as a `Mass`.
    pub const US_HUNDREDWEIGHT: Mass<Exact> = GRAM * Exact::dec(45359237, -3);

    /// One `BritishHundredweight` (`[lcwt_av]`), as a `Mass`.
    pub const BRITISH_HUNDREDWEIGHT: Mass<Exact> = GRAM * Exact::dec(5080234544, -5);

    /// One `USTon` (`[ston_av]`), as a `Mass`.
    pub const US_TON: Mass<Exact> = GRAM * Exact::dec(90718474, -2);

    /// One `BritishTon` (`[lton_av]`), as a `Mass`.
    pub const BRITISH_TON: Mass<Exact> = GRAM * Exact::dec(10160469088, -4);

    /// One `BritishStone` (`[stone_av]`), as a `Mass`.
    pub const BRITISH_STONE: Mass<Exact> = GRAM * Exact::dec(635029318, -5);

    /// One `Pennyweight` (`[pwt_tr]`), as a `Mass`.
    pub const PENNYWEIGHT: Mass<Exact> = GRAM * Exact::dec(155517384, -8);

    /// One `Scruple` (`[sc_ap]`), as a `Mass`.
    pub const SCRUPLE: Mass<Exact> = GRAM * Exact::dec(12959782, -7);

    /// One `Drachm` (`[dr_ap]`), as a `Mass`.
    pub const DRACHM: Mass<Exact> = GRAM * Exact::dec(38879346, -7);

    /// One `MetricOunce` (`[oz_m]`), as a `Mass`.
    pub const METRIC_OUNCE: Mass<Exact> = GRAM * Exact::int(28);

    /// One `Tablespoon` (`[tbs_us]`), as a `Volume`.
    pub const TABLESPOON: Volume<Exact> = (METRE * METRE * METRE) * Exact::dec(1478676478125, -17);

    /// One `Teaspoon` (`[tsp_us]`), as a `Volume`.
    pub const TEASPOON: Volume<Exact> = (METRE * METRE * METRE) * Exact::dec(492892159375, -17);

    /// One `Cup` (`[cup_us]`), as a `Volume`.
    pub const CUP: Volume<Exact> = (METRE * METRE * METRE) * Exact::dec(2365882365, -13);

    /// One `MetricCup` (`[cup_m]`), as a `Volume`.
    pub const METRIC_CUP: Volume<Exact> = (METRE * METRE * METRE) * Exact::dec(24, -5);

    /// One `MetricTeaspoon` (`[tsp_m]`), as a `Volume`.
    pub const METRIC_TEASPOON: Volume<Exact> = (METRE * METRE * METRE) * Exact::dec(5, -6);

    /// One `MetricTablespoon` (`[tbs_m]`), as a `Volume`.
    pub const METRIC_TABLESPOON: Volume<Exact> = (METRE * METRE * METRE) * Exact::dec(15, -6);
}

/// The units of `Area`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Area<N>, number = N)]
#[unit(for<N> where Area<N>: [const] yadc::Unit<Area<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum AreaUnit {
    #[default]
    #[unit("[acr_us]", "acre", by = Area::<N>::from(ACRE))]
    Acre,
    #[unit("[srd_us]", "square rod", by = Area::<N>::from(SQUARE_ROD))]
    SquareRod,
    #[unit("[smi_us]", "square mile", by = Area::<N>::from(SQUARE_MILE))]
    SquareMile,
    #[unit("[sct]", "section", by = Area::<N>::from(SECTION))]
    Section,
    #[unit("[twp]", "township", by = Area::<N>::from(TOWNSHIP))]
    Township,
}

/// The units of `DryVolume`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = DryVolume<N>, number = N)]
#[unit(for<N> where DryVolume<N>: [const] yadc::Unit<DryVolume<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum DryVolumeUnit {
    #[default]
    #[unit("[bu_us]", "bushel", by = DryVolume::<N>::from(BUSHEL))]
    Bushel,
    #[unit("[gal_wi]", "historical winchester gallon", by = DryVolume::<N>::from(HISTORICAL_WINCHESTER_GALLON))]
    HistoricalWinchesterGallon,
    #[unit("[pk_us]", "peck", by = DryVolume::<N>::from(PECK))]
    Peck,
    #[unit("[dqt_us]", "dry quart", by = DryVolume::<N>::from(DRY_QUART))]
    DryQuart,
    #[unit("[dpt_us]", "dry pint", by = DryVolume::<N>::from(DRY_PINT))]
    DryPint,
}

/// The units of `FluidVolume`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = FluidVolume<N>, number = N)]
#[unit(for<N> where FluidVolume<N>: [const] yadc::Unit<FluidVolume<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum FluidVolumeUnit {
    #[default]
    #[unit("[gal_us]", "Queen Anne's wine gallon", by = FluidVolume::<N>::from(QUEEN_ANNE_S_WINE_GALLON))]
    QueenAnneSWineGallon,
    #[unit("[bbl_us]", "barrel", by = FluidVolume::<N>::from(BARREL))]
    Barrel,
    #[unit("[qt_us]", "quart", by = FluidVolume::<N>::from(QUART))]
    Quart,
    #[unit("[pt_us]", "pint", by = FluidVolume::<N>::from(PINT))]
    Pint,
    #[unit("[gil_us]", "gill", by = FluidVolume::<N>::from(GILL))]
    Gill,
    #[unit("[foz_us]", "fluid ounce", by = FluidVolume::<N>::from(FLUID_OUNCE))]
    FluidOunce,
    #[unit("[fdr_us]", "fluid dram", by = FluidVolume::<N>::from(FLUID_DRAM))]
    FluidDram,
    #[unit("[min_us]", "minim", by = FluidVolume::<N>::from(MINIM))]
    Minim,
    #[unit("[crd_us]", "cord", by = FluidVolume::<N>::from(CORD))]
    Cord,
    #[unit("[foz_m]", "metric fluid ounce", by = FluidVolume::<N>::from(METRIC_FLUID_OUNCE))]
    MetricFluidOunce,
}

/// The units of `Length`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length<N>, number = N)]
#[unit(for<N> where Length<N>: [const] yadc::Unit<Length<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum LengthUnit {
    #[default]
    #[unit("[ft_us]", "foot", by = Length::<N>::from(FOOT))]
    Foot,
    #[unit("[yd_us]", "yard", by = Length::<N>::from(YARD))]
    Yard,
    #[unit("[in_us]", "inch", by = Length::<N>::from(INCH))]
    Inch,
    #[unit("[rd_us]", "rod", by = Length::<N>::from(ROD))]
    Rod,
    #[unit("[ch_us]", "Surveyor's chain", by = Length::<N>::from(SURVEYOR_S_CHAIN))]
    SurveyorSChain,
    #[unit("[lk_us]", "link for Gunter's chain", by = Length::<N>::from(LINK_FOR_GUNTER_S_CHAIN))]
    LinkForGunterSChain,
    #[unit("[rch_us]", "Engineer's chain", by = Length::<N>::from(ENGINEER_S_CHAIN))]
    EngineerSChain,
    #[unit("[rlk_us]", "link for Ramden's chain", by = Length::<N>::from(LINK_FOR_RAMDEN_S_CHAIN))]
    LinkForRamdenSChain,
    #[unit("[fth_us]", "fathom", by = Length::<N>::from(FATHOM))]
    Fathom,
    #[unit("[fur_us]", "furlong", by = Length::<N>::from(FURLONG))]
    Furlong,
    #[unit("[mi_us]", "mile", by = Length::<N>::from(MILE))]
    Mile,
    #[unit("[mil_us]", "mil", by = Length::<N>::from(MIL))]
    Mil,
}

/// The units of `Mass`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Mass<N>, number = N)]
#[unit(for<N> where Mass<N>: [const] yadc::Unit<Mass<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MassUnit {
    #[default]
    #[unit("[gr]", "grain", by = Mass::<N>::from(GRAIN))]
    Grain,
    #[unit("[lb_av]", "pound", by = Mass::<N>::from(POUND))]
    Pound,
    #[unit("[oz_av]", "ounce", by = Mass::<N>::from(OUNCE))]
    Ounce,
    #[unit("[dr_av]", "dram", by = Mass::<N>::from(DRAM))]
    Dram,
    #[unit("[scwt_av]", "U.S. hundredweight", by = Mass::<N>::from(US_HUNDREDWEIGHT))]
    USHundredweight,
    #[unit("[lcwt_av]", "British hundredweight", by = Mass::<N>::from(BRITISH_HUNDREDWEIGHT))]
    BritishHundredweight,
    #[unit("[ston_av]", "U.S. ton", by = Mass::<N>::from(US_TON))]
    USTon,
    #[unit("[lton_av]", "British ton", by = Mass::<N>::from(BRITISH_TON))]
    BritishTon,
    #[unit("[stone_av]", "British stone", by = Mass::<N>::from(BRITISH_STONE))]
    BritishStone,
    #[unit("[pwt_tr]", "pennyweight", by = Mass::<N>::from(PENNYWEIGHT))]
    Pennyweight,
    #[unit("[sc_ap]", "scruple", by = Mass::<N>::from(SCRUPLE))]
    Scruple,
    #[unit("[dr_ap]", "drachm", by = Mass::<N>::from(DRACHM))]
    Drachm,
    #[unit("[oz_m]", "metric ounce", by = Mass::<N>::from(METRIC_OUNCE))]
    MetricOunce,
}

/// The units of `Volume`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Volume<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Volume<N>: [const] yadc::Unit<Volume<N>, N> + [const] core::marker::Destruct)]
pub enum VolumeUnit {
    #[default]
    #[unit("[tbs_us]", "tablespoon", by = Volume::<N>::from(TABLESPOON))]
    Tablespoon,
    #[unit("[tsp_us]", "teaspoon", by = Volume::<N>::from(TEASPOON))]
    Teaspoon,
    #[unit("[cup_us]", "cup", by = Volume::<N>::from(CUP))]
    Cup,
    #[unit("[cup_m]", "metric cup", by = Volume::<N>::from(METRIC_CUP))]
    MetricCup,
    #[unit("[tsp_m]", "metric teaspoon", by = Volume::<N>::from(METRIC_TEASPOON))]
    MetricTeaspoon,
    #[unit("[tbs_m]", "metric tablespoon", by = Volume::<N>::from(METRIC_TABLESPOON))]
    MetricTablespoon,
}
