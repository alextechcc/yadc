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
#[values(into(SIUnit))]
pub enum DimensionlessUnit {
    #[default]
    #[value(RATIO.unit())]
    Ratio,
    #[value(PART_PER_HUNDRED.unit())]
    PartPerHundred,
    #[value(PERCENT.unit())]
    Percent,
    #[value(PART_PER_THOUSAND.unit())]
    PartPerThousand,
    #[value(PER_MILLE.unit())]
    PerMille,
    #[value(PART_PER_TEN_THOUSAND.unit())]
    PartPerTenThousand,
    #[value(BASIS_POINT.unit())]
    BasisPoint,
    #[value(PART_PER_MILLION.unit())]
    PartPerMillion,
    #[value(PART_PER_BILLION.unit())]
    PartPerBillion,
    #[value(PART_PER_TRILLION.unit())]
    PartPerTrillion,
    #[value(PART_PER_QUADRILLION.unit())]
    PartPerQuadrillion,
    #[value(BEL.unit())]
    Bel,
    #[value(DECIBEL.unit())]
    Decibel,
    #[value(NEPER.unit())]
    Neper,
}
