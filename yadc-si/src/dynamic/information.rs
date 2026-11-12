//! The units of `information`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::base::SECOND;
use crate::information::consts::*;
use crate::information::*;
use core::marker::Destruct;
use yadc::{Unit, num_traits::Num};

/// The units of `Information`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Information<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Information<N>: [const] Unit<Information<N>, N> + [const] Destruct)]
pub enum InformationUnit {
    #[unit("b", "bits", by = Information::<N>::from(BIT))]
    Bit,
    #[default]
    #[unit("B", "bytes", by = Information::<N>::from(BYTE))]
    Byte,
    #[unit("o", "octets", by = Information::<N>::from(OCTET))]
    Octet,
    #[unit("nibble", "nibbles", by = Information::<N>::from(NIBBLE))]
    Nibble,
    #[unit("crumb", "crumbs", by = Information::<N>::from(CRUMB))]
    Crumb,
    #[unit("Sh", "shannons", by = Information::<N>::from(SHANNON))]
    Shannon,
}

/// The units of `InformationRate`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = InformationRate<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where InformationRate<N>: [const] Unit<InformationRate<N>, N> + [const] Destruct)]
pub enum InformationRateUnit {
    #[unit("b/s", "bits per second", by = InformationRate::<N>::from((ONE / SECOND) * Exact::dec(125, -3)))]
    BitPerSecond,
    #[default]
    #[unit("B/s", "bytes per second", by = InformationRate::<N>::from(ONE / SECOND))]
    BytePerSecond,
    #[unit("o/s", "octets per second", by = InformationRate::<N>::from(ONE / SECOND))]
    OctetPerSecond,
}
