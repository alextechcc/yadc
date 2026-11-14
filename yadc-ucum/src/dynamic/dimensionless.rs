//! The units of `dimensionless`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

#[allow(unused_imports)]
use crate::dimensionless::*;
#[allow(unused_imports)]
use crate::*;

/// The units of `Dimensionless`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum DimensionlessUnit {
    #[default]
    #[value(RATIO.unit())]
    Ratio,
    #[value(THE_NUMBER_TEN_FOR_ARBITRARY_POWERS.unit())]
    TheNumberTenForArbitraryPowers,
    #[value(THE_NUMBER_PI.unit())]
    TheNumberPi,
    #[value(PERCENT.unit())]
    Percent,
    #[value(PARTS_PER_THOUSAND.unit())]
    PartsPerThousand,
    #[value(PARTS_PER_MILLION.unit())]
    PartsPerMillion,
    #[value(PARTS_PER_BILLION.unit())]
    PartsPerBillion,
    #[value(PARTS_PER_TRILLION.unit())]
    PartsPerTrillion,
}
