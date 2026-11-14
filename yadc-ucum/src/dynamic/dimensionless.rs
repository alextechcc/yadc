//! The units of `dimensionless`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::dimensionless::units::*;
use crate::dimensionless::*;
use crate::dynamic::DynamicUcumUnit;
use core::marker::Destruct;
use core::ops::{Add, Div, Mul, Sub};

/// The units of `Dimensionless`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Dimensionless<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum DimensionlessUnit {
    #[default]
    #[unit(Ratio)]
    Ratio,
    #[unit(TheNumberTenForArbitraryPowers)]
    TheNumberTenForArbitraryPowers,
    #[unit(TheNumberPi)]
    TheNumberPi,
    #[unit(Percent)]
    Percent,
    #[unit(PartsPerThousand)]
    PartsPerThousand,
    #[unit(PartsPerMillion)]
    PartsPerMillion,
    #[unit(PartsPerBillion)]
    PartsPerBillion,
    #[unit(PartsPerTrillion)]
    PartsPerTrillion,
}

impl From<DimensionlessUnit> for DynamicUcumUnit {
    fn from(unit: DimensionlessUnit) -> Self {
        match unit {
            DimensionlessUnit::Ratio => Ratio {}.into(),
            DimensionlessUnit::TheNumberTenForArbitraryPowers => {
                TheNumberTenForArbitraryPowers {}.into()
            }
            DimensionlessUnit::TheNumberPi => TheNumberPi {}.into(),
            DimensionlessUnit::Percent => Percent {}.into(),
            DimensionlessUnit::PartsPerThousand => PartsPerThousand {}.into(),
            DimensionlessUnit::PartsPerMillion => PartsPerMillion {}.into(),
            DimensionlessUnit::PartsPerBillion => PartsPerBillion {}.into(),
            DimensionlessUnit::PartsPerTrillion => PartsPerTrillion {}.into(),
        }
    }
}
