//! The units of `us`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

#[allow(unused_imports)]
use crate::us::*;
#[allow(unused_imports)]
use crate::*;

/// The units of `Area`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum AreaUnit {
    #[default]
    #[value(METER_SQUARED.unit())]
    MeterSquared,
    #[value(ACRE.unit())]
    Acre,
    #[value(SQUARE_ROD.unit())]
    SquareRod,
    #[value(SQUARE_MILE.unit())]
    SquareMile,
    #[value(SECTION.unit())]
    Section,
    #[value(TOWNSHIP.unit())]
    Township,
}

/// The units of `DryVolume`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum DryVolumeUnit {
    #[default]
    #[value(METER_CUBED.unit())]
    MeterCubed,
    #[value(BUSHEL.unit())]
    Bushel,
    #[value(HISTORICAL_WINCHESTER_GALLON.unit())]
    HistoricalWinchesterGallon,
    #[value(PECK.unit())]
    Peck,
    #[value(DRY_QUART.unit())]
    DryQuart,
    #[value(DRY_PINT.unit())]
    DryPint,
}

/// The units of `FluidVolume`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum FluidVolumeUnit {
    #[default]
    #[value(METER_CUBED.unit())]
    MeterCubed,
    #[value(QUEEN_ANNE_S_WINE_GALLON.unit())]
    QueenAnneSWineGallon,
    #[value(BARREL.unit())]
    Barrel,
    #[value(QUART.unit())]
    Quart,
    #[value(PINT.unit())]
    Pint,
    #[value(GILL.unit())]
    Gill,
    #[value(FLUID_OUNCE.unit())]
    FluidOunce,
    #[value(FLUID_DRAM.unit())]
    FluidDram,
    #[value(MINIM.unit())]
    Minim,
    #[value(CORD.unit())]
    Cord,
    #[value(METRIC_FLUID_OUNCE.unit())]
    MetricFluidOunce,
}

/// The units of `Length`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum LengthUnit {
    #[default]
    #[value(METER.unit())]
    Meter,
    #[value(FOOT.unit())]
    Foot,
    #[value(YARD.unit())]
    Yard,
    #[value(INCH.unit())]
    Inch,
    #[value(ROD.unit())]
    Rod,
    #[value(SURVEYOR_S_CHAIN.unit())]
    SurveyorSChain,
    #[value(LINK_FOR_GUNTER_S_CHAIN.unit())]
    LinkForGunterSChain,
    #[value(ENGINEER_S_CHAIN.unit())]
    EngineerSChain,
    #[value(LINK_FOR_RAMDEN_S_CHAIN.unit())]
    LinkForRamdenSChain,
    #[value(FATHOM.unit())]
    Fathom,
    #[value(FURLONG.unit())]
    Furlong,
    #[value(MILE.unit())]
    Mile,
    #[value(MIL.unit())]
    Mil,
}

/// The units of `Mass`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum MassUnit {
    #[default]
    #[value(GRAM.unit())]
    Gram,
    #[value(GRAIN.unit())]
    Grain,
    #[value(POUND.unit())]
    Pound,
    #[value(OUNCE.unit())]
    Ounce,
    #[value(DRAM.unit())]
    Dram,
    #[value(US_HUNDREDWEIGHT.unit())]
    USHundredweight,
    #[value(BRITISH_HUNDREDWEIGHT.unit())]
    BritishHundredweight,
    #[value(US_TON.unit())]
    USTon,
    #[value(BRITISH_TON.unit())]
    BritishTon,
    #[value(BRITISH_STONE.unit())]
    BritishStone,
    #[value(PENNYWEIGHT.unit())]
    Pennyweight,
    #[value(SCRUPLE.unit())]
    Scruple,
    #[value(DRACHM.unit())]
    Drachm,
    #[value(METRIC_OUNCE.unit())]
    MetricOunce,
}

/// The units of `Volume`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum VolumeUnit {
    #[default]
    #[value(METER_CUBED.unit())]
    MeterCubed,
    #[value(TABLESPOON.unit())]
    Tablespoon,
    #[value(TEASPOON.unit())]
    Teaspoon,
    #[value(CUP.unit())]
    Cup,
    #[value(METRIC_CUP.unit())]
    MetricCup,
    #[value(METRIC_TEASPOON.unit())]
    MetricTeaspoon,
    #[value(METRIC_TABLESPOON.unit())]
    MetricTablespoon,
}
