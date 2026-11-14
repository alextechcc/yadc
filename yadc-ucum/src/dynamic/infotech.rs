//! The units of `infotech`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::dynamic::DynamicUcumUnit;
use crate::infotech::units::*;
use crate::infotech::*;
use core::marker::Destruct;
use core::ops::{Add, Div, Mul, Sub};

/// The units of `AmountOfInformation`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AmountOfInformation<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum AmountOfInformationUnit {
    #[default]
    #[unit(Bit)]
    Bit,
    #[unit(Byte)]
    Byte,
}

impl From<AmountOfInformationUnit> for DynamicUcumUnit {
    fn from(unit: AmountOfInformationUnit) -> Self {
        match unit {
            AmountOfInformationUnit::Bit => Bit {}.into(),
            AmountOfInformationUnit::Byte => Byte {}.into(),
        }
    }
}

/// The units of `SignalTransmissionRate`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SignalTransmissionRate<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum SignalTransmissionRateUnit {
    #[default]
    #[unit(Baud)]
    Baud,
}

impl From<SignalTransmissionRateUnit> for DynamicUcumUnit {
    fn from(unit: SignalTransmissionRateUnit) -> Self {
        match unit {
            SignalTransmissionRateUnit::Baud => Baud {}.into(),
        }
    }
}
