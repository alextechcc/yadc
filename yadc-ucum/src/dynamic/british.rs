//! The units of `british`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::british::units::*;
use crate::british::*;
use crate::dynamic::DynamicUcumUnit;
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
}

impl From<AreaUnit> for DynamicUcumUnit {
    fn from(unit: AreaUnit) -> Self {
        match unit {
            AreaUnit::Acre => Acre {}.into(),
        }
    }
}

/// The units of `Length`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum LengthUnit {
    #[default]
    #[unit(Inch)]
    Inch,
    #[unit(Foot)]
    Foot,
    #[unit(Rod)]
    Rod,
    #[unit(GunterSChain)]
    GunterSChain,
    #[unit(LinkForGunterSChain)]
    LinkForGunterSChain,
    #[unit(Fathom)]
    Fathom,
    #[unit(Pace)]
    Pace,
    #[unit(Yard)]
    Yard,
    #[unit(Mile)]
    Mile,
    #[unit(NauticalMile)]
    NauticalMile,
}

impl From<LengthUnit> for DynamicUcumUnit {
    fn from(unit: LengthUnit) -> Self {
        match unit {
            LengthUnit::Inch => Inch {}.into(),
            LengthUnit::Foot => Foot {}.into(),
            LengthUnit::Rod => Rod {}.into(),
            LengthUnit::GunterSChain => GunterSChain {}.into(),
            LengthUnit::LinkForGunterSChain => LinkForGunterSChain {}.into(),
            LengthUnit::Fathom => Fathom {}.into(),
            LengthUnit::Pace => Pace {}.into(),
            LengthUnit::Yard => Yard {}.into(),
            LengthUnit::Mile => Mile {}.into(),
            LengthUnit::NauticalMile => NauticalMile {}.into(),
        }
    }
}

/// The units of `Velocity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Velocity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum VelocityUnit {
    #[default]
    #[unit(Knot)]
    Knot,
}

impl From<VelocityUnit> for DynamicUcumUnit {
    fn from(unit: VelocityUnit) -> Self {
        match unit {
            VelocityUnit::Knot => Knot {}.into(),
        }
    }
}

/// The units of `Volume`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Volume<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum VolumeUnit {
    #[default]
    #[unit(Gallon)]
    Gallon,
    #[unit(Peck)]
    Peck,
    #[unit(Bushel)]
    Bushel,
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
}

impl From<VolumeUnit> for DynamicUcumUnit {
    fn from(unit: VolumeUnit) -> Self {
        match unit {
            VolumeUnit::Gallon => Gallon {}.into(),
            VolumeUnit::Peck => Peck {}.into(),
            VolumeUnit::Bushel => Bushel {}.into(),
            VolumeUnit::Quart => Quart {}.into(),
            VolumeUnit::Pint => Pint {}.into(),
            VolumeUnit::Gill => Gill {}.into(),
            VolumeUnit::FluidOunce => FluidOunce {}.into(),
            VolumeUnit::FluidDram => FluidDram {}.into(),
            VolumeUnit::Minim => Minim {}.into(),
        }
    }
}
