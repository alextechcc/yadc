//! The units of `information`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::dynamic::DynamicSIUnit;
use crate::information::units::*;
use crate::information::*;
use crate::{Exact, SIQuantity, Unit};
use core::ops::{Add, Div, Mul, Sub};

/// The units of `Information`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<INFORMATION, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<INFORMATION, N>;
        fn value(&self, quantity: SIQuantity<INFORMATION, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum InformationUnit {
    #[value(BIT)]
    Bit,
    #[default]
    #[value(BYTE)]
    Byte,
    #[value(OCTET)]
    Octet,
    #[value(NIBBLE)]
    Nibble,
    #[value(CRUMB)]
    Crumb,
    #[value(SHANNON)]
    Shannon,
}

/// The units of `InformationRate`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<INFORMATION_RATE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<INFORMATION_RATE, N>;
        fn value(&self, quantity: SIQuantity<INFORMATION_RATE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum InformationRateUnit {
    #[value(BIT_PER_SECOND)]
    BitPerSecond,
    #[default]
    #[value(BYTE_PER_SECOND)]
    BytePerSecond,
    #[value(OCTET_PER_SECOND)]
    OctetPerSecond,
}
