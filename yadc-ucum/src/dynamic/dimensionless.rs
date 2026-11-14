//! The units of `dimensionless`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::dimensionless::units::*;
use crate::dimensionless::*;
use crate::dynamic::DynamicUcumUnit;
use crate::{Exact, Qty, Unit};
use core::ops::{Add, Div, Mul, Sub};

/// The units of `Dimensionless`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<DIMENSIONLESS, N>, N> {
        fn quantity(&self, value: N) -> Qty<DIMENSIONLESS, N>;
        fn value(&self, quantity: Qty<DIMENSIONLESS, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum DimensionlessUnit {
    #[default]
    #[value(RATIO)]
    Ratio,
    #[value(THE_NUMBER_TEN_FOR_ARBITRARY_POWERS)]
    TheNumberTenForArbitraryPowers,
    #[value(THE_NUMBER_PI)]
    TheNumberPi,
    #[value(PERCENT)]
    Percent,
    #[value(PARTS_PER_THOUSAND)]
    PartsPerThousand,
    #[value(PARTS_PER_MILLION)]
    PartsPerMillion,
    #[value(PARTS_PER_BILLION)]
    PartsPerBillion,
    #[value(PARTS_PER_TRILLION)]
    PartsPerTrillion,
}
