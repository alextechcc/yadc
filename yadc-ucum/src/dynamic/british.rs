//! The units of `british`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

#[allow(unused_imports)]
use crate::british::*;
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
    #[value(UcumUnit::ONE.times(METER.unit(), 2))]
    MeterSquared,
    #[value(ACRE.unit())]
    Acre,
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
    #[value(INCH.unit())]
    Inch,
    #[value(FOOT.unit())]
    Foot,
    #[value(ROD.unit())]
    Rod,
    #[value(GUNTER_S_CHAIN.unit())]
    GunterSChain,
    #[value(LINK_FOR_GUNTER_S_CHAIN.unit())]
    LinkForGunterSChain,
    #[value(FATHOM.unit())]
    Fathom,
    #[value(PACE.unit())]
    Pace,
    #[value(YARD.unit())]
    Yard,
    #[value(MILE.unit())]
    Mile,
    #[value(NAUTICAL_MILE.unit())]
    NauticalMile,
}

/// The units of `Velocity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum VelocityUnit {
    #[default]
    #[value(METER.unit().times(SECOND.unit(), -1))]
    MeterPerSecond,
    #[value(KNOT.unit())]
    Knot,
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
    #[value(UcumUnit::ONE.times(METER.unit(), 3))]
    MeterCubed,
    #[value(GALLON.unit())]
    Gallon,
    #[value(PECK.unit())]
    Peck,
    #[value(BUSHEL.unit())]
    Bushel,
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
}
