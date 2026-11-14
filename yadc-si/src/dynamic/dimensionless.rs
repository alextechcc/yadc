//! The units of `dimensionless`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::dimensionless::units::*;
use crate::dimensionless::*;
use crate::dynamic::DynamicSIUnit;
use crate::{Exact, SIQuantity, Unit};
use core::ops::{Add, Div, Mul, Sub};

/// The units of `Dimensionless`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<DIMENSIONLESS, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<DIMENSIONLESS, N>;
        fn value(&self, quantity: SIQuantity<DIMENSIONLESS, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum DimensionlessUnit {
    #[default]
    #[value(RATIO)]
    Ratio,
    #[value(PART_PER_HUNDRED)]
    PartPerHundred,
    #[value(PERCENT)]
    Percent,
    #[value(PART_PER_THOUSAND)]
    PartPerThousand,
    #[value(PER_MILLE)]
    PerMille,
    #[value(PART_PER_TEN_THOUSAND)]
    PartPerTenThousand,
    #[value(BASIS_POINT)]
    BasisPoint,
    #[value(PART_PER_MILLION)]
    PartPerMillion,
    #[value(PART_PER_BILLION)]
    PartPerBillion,
    #[value(PART_PER_TRILLION)]
    PartPerTrillion,
    #[value(PART_PER_QUADRILLION)]
    PartPerQuadrillion,
}
