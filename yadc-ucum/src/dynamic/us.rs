//! The units of `us`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::us::consts::*;
use crate::us::*;
use core::marker::Destruct;
use yadc::{Unit, num_traits::Num};

/// The units of `Area`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Area<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Area<N>: [const] Unit<Area<N>, N> + [const] Destruct)]
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

/// The units of `DryVolume`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = DryVolume<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where DryVolume<N>: [const] Unit<DryVolume<N>, N> + [const] Destruct)]
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

/// The units of `FluidVolume`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = FluidVolume<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where FluidVolume<N>: [const] Unit<FluidVolume<N>, N> + [const] Destruct)]
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

/// The units of `Length`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Length<N>: [const] Unit<Length<N>, N> + [const] Destruct)]
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

/// The units of `Mass`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Mass<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Mass<N>: [const] Unit<Mass<N>, N> + [const] Destruct)]
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

/// The units of `Volume`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Volume<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Volume<N>: [const] Unit<Volume<N>, N> + [const] Destruct)]
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
