//! The units of `dimensionless`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::dimensionless::units::*;
use crate::dimensionless::*;
use crate::dynamic::DynamicSIUnit;
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
    #[unit(PartPerHundred)]
    PartPerHundred,
    #[unit(Percent)]
    Percent,
    #[unit(PartPerThousand)]
    PartPerThousand,
    #[unit(PerMille)]
    PerMille,
    #[unit(PartPerTenThousand)]
    PartPerTenThousand,
    #[unit(BasisPoint)]
    BasisPoint,
    #[unit(PartPerMillion)]
    PartPerMillion,
    #[unit(PartPerBillion)]
    PartPerBillion,
    #[unit(PartPerTrillion)]
    PartPerTrillion,
    #[unit(PartPerQuadrillion)]
    PartPerQuadrillion,
}

impl From<DimensionlessUnit> for DynamicSIUnit {
    fn from(unit: DimensionlessUnit) -> Self {
        match unit {
            DimensionlessUnit::Ratio => Ratio {}.into(),
            DimensionlessUnit::PartPerHundred => PartPerHundred {}.into(),
            DimensionlessUnit::Percent => Percent {}.into(),
            DimensionlessUnit::PartPerThousand => PartPerThousand {}.into(),
            DimensionlessUnit::PerMille => PerMille {}.into(),
            DimensionlessUnit::PartPerTenThousand => PartPerTenThousand {}.into(),
            DimensionlessUnit::BasisPoint => BasisPoint {}.into(),
            DimensionlessUnit::PartPerMillion => PartPerMillion {}.into(),
            DimensionlessUnit::PartPerBillion => PartPerBillion {}.into(),
            DimensionlessUnit::PartPerTrillion => PartPerTrillion {}.into(),
            DimensionlessUnit::PartPerQuadrillion => PartPerQuadrillion {}.into(),
        }
    }
}
