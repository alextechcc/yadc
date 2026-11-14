//! The units of `british`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::british::units::*;
use crate::british::*;
use crate::dynamic::DynamicUcumUnit;
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
    #[value(INCH)]
    Inch,
    #[value(FOOT)]
    Foot,
    #[value(ROD)]
    Rod,
    #[value(GUNTER_S_CHAIN)]
    GunterSChain,
    #[value(LINK_FOR_GUNTER_S_CHAIN)]
    LinkForGunterSChain,
    #[value(FATHOM)]
    Fathom,
    #[value(PACE)]
    Pace,
    #[value(YARD)]
    Yard,
    #[value(MILE)]
    Mile,
    #[value(NAUTICAL_MILE)]
    NauticalMile,
}

/// The units of `Velocity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<VELOCITY, N>, N> {
        fn quantity(&self, value: N) -> Qty<VELOCITY, N>;
        fn value(&self, quantity: Qty<VELOCITY, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum VelocityUnit {
    #[default]
    #[value(KNOT)]
    Knot,
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
    #[value(GALLON)]
    Gallon,
    #[value(PECK)]
    Peck,
    #[value(BUSHEL)]
    Bushel,
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
}
