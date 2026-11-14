//! The units of `information`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::dynamic::DynamicSIUnit;
use crate::information::units::*;
use crate::information::*;
use core::marker::Destruct;
use core::ops::{Add, Div, Mul, Sub};

/// The units of `Information`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Information<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum InformationUnit {
    #[unit(Bit)]
    Bit,
    #[default]
    #[unit(Byte)]
    Byte,
    #[unit(Octet)]
    Octet,
    #[unit(Nibble)]
    Nibble,
    #[unit(Crumb)]
    Crumb,
    #[unit(Shannon)]
    Shannon,
}

impl From<InformationUnit> for DynamicSIUnit {
    fn from(unit: InformationUnit) -> Self {
        match unit {
            InformationUnit::Bit => Bit {}.into(),
            InformationUnit::Byte => Byte {}.into(),
            InformationUnit::Octet => Octet {}.into(),
            InformationUnit::Nibble => Nibble {}.into(),
            InformationUnit::Crumb => Crumb {}.into(),
            InformationUnit::Shannon => Shannon {}.into(),
        }
    }
}

/// The units of `InformationRate`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = InformationRate<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum InformationRateUnit {
    #[unit(BitPerSecond)]
    BitPerSecond,
    #[default]
    #[unit(BytePerSecond)]
    BytePerSecond,
    #[unit(OctetPerSecond)]
    OctetPerSecond,
}

impl From<InformationRateUnit> for DynamicSIUnit {
    fn from(unit: InformationRateUnit) -> Self {
        match unit {
            InformationRateUnit::BitPerSecond => BitPerSecond {}.into(),
            InformationRateUnit::BytePerSecond => BytePerSecond {}.into(),
            InformationRateUnit::OctetPerSecond => OctetPerSecond {}.into(),
        }
    }
}
