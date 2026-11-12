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
    #[unit("10*", "the number ten for arbitrary powers", by = Dimensionless::<N>::from(THE_NUMBER_TEN_FOR_ARBITRARY_POWERS))]
    TheNumberTenForArbitraryPowers,
    #[unit("[pi]", "the number pi", by = Dimensionless::<N>::from(THE_NUMBER_PI))]
    TheNumberPi,
    #[unit("%", "percent", by = Dimensionless::<N>::from(PERCENT))]
    Percent,
    #[unit("[ppth]", "parts per thousand", by = Dimensionless::<N>::from(PARTS_PER_THOUSAND))]
    PartsPerThousand,
    #[unit("[ppm]", "parts per million", by = Dimensionless::<N>::from(PARTS_PER_MILLION))]
    PartsPerMillion,
    #[unit("[ppb]", "parts per billion", by = Dimensionless::<N>::from(PARTS_PER_BILLION))]
    PartsPerBillion,
    #[unit("[pptr]", "parts per trillion", by = Dimensionless::<N>::from(PARTS_PER_TRILLION))]
    PartsPerTrillion,
}
