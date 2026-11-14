//! The units of `us`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::dynamic::DynamicUcumUnit;
use crate::us::units::*;
use crate::us::*;
use crate::{Exact, Qty, Unit};
use core::ops::{Add, Div, Mul, Sub};

/// The units of `Area`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<AREA, N>, N> {
        fn quantity(&self, value: N) -> Qty<AREA, N>;
        fn value(&self, quantity: Qty<AREA, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum AreaUnit {
    #[default]
    #[value(ACRE)]
    Acre,
    #[value(SQUARE_ROD)]
    SquareRod,
    #[value(SQUARE_MILE)]
    SquareMile,
    #[value(SECTION)]
    Section,
    #[value(TOWNSHIP)]
    Township,
}

/// The units of `DryVolume`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<DRY_VOLUME, N>, N> {
        fn quantity(&self, value: N) -> Qty<DRY_VOLUME, N>;
        fn value(&self, quantity: Qty<DRY_VOLUME, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum DryVolumeUnit {
    #[default]
    #[value(BUSHEL)]
    Bushel,
    #[value(HISTORICAL_WINCHESTER_GALLON)]
    HistoricalWinchesterGallon,
    #[value(PECK)]
    Peck,
    #[value(DRY_QUART)]
    DryQuart,
    #[value(DRY_PINT)]
    DryPint,
}

/// The units of `FluidVolume`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<FLUID_VOLUME, N>, N> {
        fn quantity(&self, value: N) -> Qty<FLUID_VOLUME, N>;
        fn value(&self, quantity: Qty<FLUID_VOLUME, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum FluidVolumeUnit {
    #[default]
    #[value(QUEEN_ANNE_S_WINE_GALLON)]
    QueenAnneSWineGallon,
    #[value(BARREL)]
    Barrel,
    #[value(QUART)]
    Quart,
    #[value(PINT)]
    Pint,
    #[value(GILL)]
    Gill,
    #[value(FLUID_OUNCE)]
    FluidOunce,
    #[value(FLUID_DRAM)]
    FluidDram,
    #[value(MINIM)]
    Minim,
    #[value(CORD)]
    Cord,
    #[value(METRIC_FLUID_OUNCE)]
    MetricFluidOunce,
}

/// The units of `Length`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<LENGTH, N>, N> {
        fn quantity(&self, value: N) -> Qty<LENGTH, N>;
        fn value(&self, quantity: Qty<LENGTH, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum LengthUnit {
    #[default]
    #[value(FOOT)]
    Foot,
    #[value(YARD)]
    Yard,
    #[value(INCH)]
    Inch,
    #[value(ROD)]
    Rod,
    #[value(SURVEYOR_S_CHAIN)]
    SurveyorSChain,
    #[value(LINK_FOR_GUNTER_S_CHAIN)]
    LinkForGunterSChain,
    #[value(ENGINEER_S_CHAIN)]
    EngineerSChain,
    #[value(LINK_FOR_RAMDEN_S_CHAIN)]
    LinkForRamdenSChain,
    #[value(FATHOM)]
    Fathom,
    #[value(FURLONG)]
    Furlong,
    #[value(MILE)]
    Mile,
    #[value(MIL)]
    Mil,
}

/// The units of `Mass`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<MASS, N>, N> {
        fn quantity(&self, value: N) -> Qty<MASS, N>;
        fn value(&self, quantity: Qty<MASS, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MassUnit {
    #[default]
    #[value(GRAIN)]
    Grain,
    #[value(POUND)]
    Pound,
    #[value(OUNCE)]
    Ounce,
    #[value(DRAM)]
    Dram,
    #[value(US_HUNDREDWEIGHT)]
    USHundredweight,
    #[value(BRITISH_HUNDREDWEIGHT)]
    BritishHundredweight,
    #[value(US_TON)]
    USTon,
    #[value(BRITISH_TON)]
    BritishTon,
    #[value(BRITISH_STONE)]
    BritishStone,
    #[value(PENNYWEIGHT)]
    Pennyweight,
    #[value(SCRUPLE)]
    Scruple,
    #[value(DRACHM)]
    Drachm,
    #[value(METRIC_OUNCE)]
    MetricOunce,
}

/// The units of `Volume`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<VOLUME, N>, N> {
        fn quantity(&self, value: N) -> Qty<VOLUME, N>;
        fn value(&self, quantity: Qty<VOLUME, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum VolumeUnit {
    #[default]
    #[value(TABLESPOON)]
    Tablespoon,
    #[value(TEASPOON)]
    Teaspoon,
    #[value(CUP)]
    Cup,
    #[value(METRIC_CUP)]
    MetricCup,
    #[value(METRIC_TEASPOON)]
    MetricTeaspoon,
    #[value(METRIC_TABLESPOON)]
    MetricTablespoon,
}
