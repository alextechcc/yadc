//! The units of `dimensionless`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::dimensionless::consts::*;
use crate::dimensionless::*;
use core::marker::Destruct;
use yadc::{Unit, num_traits::Num};

/// The units of `Dimensionless`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Dimensionless<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Dimensionless<N>: [const] Unit<Dimensionless<N>, N> + [const] Destruct)]
pub enum DimensionlessUnit {
    #[default]
    #[unit("", "", by = Dimensionless::<N>::from(RATIO))]
    Ratio,
    #[unit("parts per hundred", "parts per hundred", by = Dimensionless::<N>::from(PART_PER_HUNDRED))]
    PartPerHundred,
    #[unit("%", "percent", by = Dimensionless::<N>::from(PERCENT))]
    Percent,
    #[unit("parts per thousand", "parts per thousand", by = Dimensionless::<N>::from(PART_PER_THOUSAND))]
    PartPerThousand,
    #[unit("‰", "per mille", by = Dimensionless::<N>::from(PER_MILLE))]
    PerMille,
    #[unit("parts per ten thousand", "parts per ten thousand", by = Dimensionless::<N>::from(PART_PER_TEN_THOUSAND))]
    PartPerTenThousand,
    #[unit("bp", "basis points", by = Dimensionless::<N>::from(BASIS_POINT))]
    BasisPoint,
    #[unit("ppm", "parts per million", by = Dimensionless::<N>::from(PART_PER_MILLION))]
    PartPerMillion,
    #[unit("ppb", "parts per billion", by = Dimensionless::<N>::from(PART_PER_BILLION))]
    PartPerBillion,
    #[unit("ppt", "parts per trillion", by = Dimensionless::<N>::from(PART_PER_TRILLION))]
    PartPerTrillion,
    #[unit("ppq", "parts per quadrillion", by = Dimensionless::<N>::from(PART_PER_QUADRILLION))]
    PartPerQuadrillion,
}
