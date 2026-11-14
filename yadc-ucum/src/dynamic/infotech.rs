//! The units of `infotech`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::dynamic::DynamicUcumUnit;
use crate::infotech::units::*;
use crate::infotech::*;
use crate::{Exact, Qty, Unit};
use core::ops::{Add, Div, Mul, Sub};

/// The units of `AmountOfInformation`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<AMOUNT_OF_INFORMATION, N>, N> {
        fn quantity(&self, value: N) -> Qty<AMOUNT_OF_INFORMATION, N>;
        fn value(&self, quantity: Qty<AMOUNT_OF_INFORMATION, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum AmountOfInformationUnit {
    #[default]
    #[value(BIT)]
    Bit,
    #[value(BYTE)]
    Byte,
}

/// The units of `SignalTransmissionRate`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<SIGNAL_TRANSMISSION_RATE, N>, N> {
        fn quantity(&self, value: N) -> Qty<SIGNAL_TRANSMISSION_RATE, N>;
        fn value(&self, quantity: Qty<SIGNAL_TRANSMISSION_RATE, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum SignalTransmissionRateUnit {
    #[default]
    #[value(BAUD)]
    Baud,
}
