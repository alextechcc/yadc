//! The units of `infotech`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::infotech::consts::*;
use crate::infotech::*;
use core::marker::Destruct;
use yadc::{Unit, num_traits::Num};

/// The units of `AmountOfInformation`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AmountOfInformation<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where AmountOfInformation<N>: [const] Unit<AmountOfInformation<N>, N> + [const] Destruct)]
pub enum AmountOfInformationUnit {
    #[default]
    #[unit("bit", "bit", by = AmountOfInformation::<N>::from(BIT))]
    Bit,
    #[unit("By", "byte", by = AmountOfInformation::<N>::from(BYTE))]
    Byte,
}

/// The units of `SignalTransmissionRate`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SignalTransmissionRate<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where SignalTransmissionRate<N>: [const] Unit<SignalTransmissionRate<N>, N> + [const] Destruct)]
pub enum SignalTransmissionRateUnit {
    #[default]
    #[unit("Bd", "baud", by = SignalTransmissionRate::<N>::from(BAUD))]
    Baud,
}
