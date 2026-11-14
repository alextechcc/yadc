//! The units of `us`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::dynamic::DynamicUcumUnit;
use crate::us::units::*;
use crate::us::*;
use core::marker::Destruct;
use core::ops::{Add, Div, Mul, Sub};

/// The units of `Area`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Area<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum AreaUnit {
    #[default]
    #[unit(Acre)]
    Acre,
    #[unit(SquareRod)]
    SquareRod,
    #[unit(SquareMile)]
    SquareMile,
    #[unit(Section)]
    Section,
    #[unit(Township)]
    Township,
}

impl From<AreaUnit> for DynamicUcumUnit {
    fn from(unit: AreaUnit) -> Self {
        match unit {
            AreaUnit::Acre => Acre {}.into(),
            AreaUnit::SquareRod => SquareRod {}.into(),
            AreaUnit::SquareMile => SquareMile {}.into(),
            AreaUnit::Section => Section {}.into(),
            AreaUnit::Township => Township {}.into(),
        }
    }
}

/// The units of `DryVolume`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = DryVolume<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum DryVolumeUnit {
    #[default]
    #[unit(Bushel)]
    Bushel,
    #[unit(HistoricalWinchesterGallon)]
    HistoricalWinchesterGallon,
    #[unit(Peck)]
    Peck,
    #[unit(DryQuart)]
    DryQuart,
    #[unit(DryPint)]
    DryPint,
}

impl From<DryVolumeUnit> for DynamicUcumUnit {
    fn from(unit: DryVolumeUnit) -> Self {
        match unit {
            DryVolumeUnit::Bushel => Bushel {}.into(),
            DryVolumeUnit::HistoricalWinchesterGallon => HistoricalWinchesterGallon {}.into(),
            DryVolumeUnit::Peck => Peck {}.into(),
            DryVolumeUnit::DryQuart => DryQuart {}.into(),
            DryVolumeUnit::DryPint => DryPint {}.into(),
        }
    }
}

/// The units of `FluidVolume`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = FluidVolume<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum FluidVolumeUnit {
    #[default]
    #[unit(QueenAnneSWineGallon)]
    QueenAnneSWineGallon,
    #[unit(Barrel)]
    Barrel,
    #[unit(Quart)]
    Quart,
    #[unit(Pint)]
    Pint,
    #[unit(Gill)]
    Gill,
    #[unit(FluidOunce)]
    FluidOunce,
    #[unit(FluidDram)]
    FluidDram,
    #[unit(Minim)]
    Minim,
    #[unit(Cord)]
    Cord,
    #[unit(MetricFluidOunce)]
    MetricFluidOunce,
}

impl From<FluidVolumeUnit> for DynamicUcumUnit {
    fn from(unit: FluidVolumeUnit) -> Self {
        match unit {
            FluidVolumeUnit::QueenAnneSWineGallon => QueenAnneSWineGallon {}.into(),
            FluidVolumeUnit::Barrel => Barrel {}.into(),
            FluidVolumeUnit::Quart => Quart {}.into(),
            FluidVolumeUnit::Pint => Pint {}.into(),
            FluidVolumeUnit::Gill => Gill {}.into(),
            FluidVolumeUnit::FluidOunce => FluidOunce {}.into(),
            FluidVolumeUnit::FluidDram => FluidDram {}.into(),
            FluidVolumeUnit::Minim => Minim {}.into(),
            FluidVolumeUnit::Cord => Cord {}.into(),
            FluidVolumeUnit::MetricFluidOunce => MetricFluidOunce {}.into(),
        }
    }
}

/// The units of `Length`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum LengthUnit {
    #[default]
    #[unit(Foot)]
    Foot,
    #[unit(Yard)]
    Yard,
    #[unit(Inch)]
    Inch,
    #[unit(Rod)]
    Rod,
    #[unit(SurveyorSChain)]
    SurveyorSChain,
    #[unit(LinkForGunterSChain)]
    LinkForGunterSChain,
    #[unit(EngineerSChain)]
    EngineerSChain,
    #[unit(LinkForRamdenSChain)]
    LinkForRamdenSChain,
    #[unit(Fathom)]
    Fathom,
    #[unit(Furlong)]
    Furlong,
    #[unit(Mile)]
    Mile,
    #[unit(Mil)]
    Mil,
}

impl From<LengthUnit> for DynamicUcumUnit {
    fn from(unit: LengthUnit) -> Self {
        match unit {
            LengthUnit::Foot => Foot {}.into(),
            LengthUnit::Yard => Yard {}.into(),
            LengthUnit::Inch => Inch {}.into(),
            LengthUnit::Rod => Rod {}.into(),
            LengthUnit::SurveyorSChain => SurveyorSChain {}.into(),
            LengthUnit::LinkForGunterSChain => LinkForGunterSChain {}.into(),
            LengthUnit::EngineerSChain => EngineerSChain {}.into(),
            LengthUnit::LinkForRamdenSChain => LinkForRamdenSChain {}.into(),
            LengthUnit::Fathom => Fathom {}.into(),
            LengthUnit::Furlong => Furlong {}.into(),
            LengthUnit::Mile => Mile {}.into(),
            LengthUnit::Mil => Mil {}.into(),
        }
    }
}

/// The units of `Mass`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Mass<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MassUnit {
    #[default]
    #[unit(Grain)]
    Grain,
    #[unit(Pound)]
    Pound,
    #[unit(Ounce)]
    Ounce,
    #[unit(Dram)]
    Dram,
    #[unit(USHundredweight)]
    USHundredweight,
    #[unit(BritishHundredweight)]
    BritishHundredweight,
    #[unit(USTon)]
    USTon,
    #[unit(BritishTon)]
    BritishTon,
    #[unit(BritishStone)]
    BritishStone,
    #[unit(Pennyweight)]
    Pennyweight,
    #[unit(Scruple)]
    Scruple,
    #[unit(Drachm)]
    Drachm,
    #[unit(MetricOunce)]
    MetricOunce,
}

impl From<MassUnit> for DynamicUcumUnit {
    fn from(unit: MassUnit) -> Self {
        match unit {
            MassUnit::Grain => Grain {}.into(),
            MassUnit::Pound => Pound {}.into(),
            MassUnit::Ounce => Ounce {}.into(),
            MassUnit::Dram => Dram {}.into(),
            MassUnit::USHundredweight => USHundredweight {}.into(),
            MassUnit::BritishHundredweight => BritishHundredweight {}.into(),
            MassUnit::USTon => USTon {}.into(),
            MassUnit::BritishTon => BritishTon {}.into(),
            MassUnit::BritishStone => BritishStone {}.into(),
            MassUnit::Pennyweight => Pennyweight {}.into(),
            MassUnit::Scruple => Scruple {}.into(),
            MassUnit::Drachm => Drachm {}.into(),
            MassUnit::MetricOunce => MetricOunce {}.into(),
        }
    }
}

/// The units of `Volume`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Volume<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum VolumeUnit {
    #[default]
    #[unit(Tablespoon)]
    Tablespoon,
    #[unit(Teaspoon)]
    Teaspoon,
    #[unit(Cup)]
    Cup,
    #[unit(MetricCup)]
    MetricCup,
    #[unit(MetricTeaspoon)]
    MetricTeaspoon,
    #[unit(MetricTablespoon)]
    MetricTablespoon,
}

impl From<VolumeUnit> for DynamicUcumUnit {
    fn from(unit: VolumeUnit) -> Self {
        match unit {
            VolumeUnit::Tablespoon => Tablespoon {}.into(),
            VolumeUnit::Teaspoon => Teaspoon {}.into(),
            VolumeUnit::Cup => Cup {}.into(),
            VolumeUnit::MetricCup => MetricCup {}.into(),
            VolumeUnit::MetricTeaspoon => MetricTeaspoon {}.into(),
            VolumeUnit::MetricTablespoon => MetricTablespoon {}.into(),
        }
    }
}
