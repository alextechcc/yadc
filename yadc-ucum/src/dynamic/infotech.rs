//! The units of `infotech`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

#[allow(unused_imports)]
use crate::infotech::*;
#[allow(unused_imports)]
use crate::*;

/// The units of `AmountOfInformation`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum AmountOfInformationUnit {
    #[default]
    #[value(BIT.unit())]
    Bit,
    #[value(BYTE.unit())]
    Byte,
}

/// The units of `SignalTransmissionRate`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum SignalTransmissionRateUnit {
    #[default]
    #[value(BAUD.unit())]
    Baud,
}
