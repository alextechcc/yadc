use crate::{ISQ, StaticSIQuantity};
use yadc::{Dimension, Unit};

pub type Dimensionless = StaticSIQuantity<{ ISQ { ..ISQ::ZERO } }>;

// Variants mirror uom's `ratio.rs` quantity.
#[derive(Unit, Copy, Clone)]
#[dim(Dimensionless)]
pub enum DimensionlessUnit {
    #[linear("", "", 1.0)]
    Ratio,
    #[linear("parts per hundred", "parts per hundred", 1.0e-2)]
    PartPerHundred,
    #[linear("%", "percent", 1.0e-2)]
    Percent,
    #[linear("parts per thousand", "parts per thousand", 1.0e-3)]
    PartPerThousand,
    #[linear("‰", "per mille", 1.0e-3)]
    PerMille,
    #[linear("parts per ten thousand", "parts per ten thousand", 1.0e-4)]
    PartPerTenThousand,
    #[linear("bp", "basis points", 1.0e-4)]
    BasisPoint,
    #[linear("ppm", "parts per million", 1.0e-6)]
    PartPerMillion,
    #[linear("ppb", "parts per billion", 1.0e-9)]
    PartPerBillion,
    #[linear("ppt", "parts per trillion", 1.0e-12)]
    PartPerTrillion,
    #[linear("ppq", "parts per quadrillion", 1.0e-15)]
    PartPerQuadrillion,
    #[log("B", "bels", 10.0, 1.0)]
    Bel,
    #[log("dB", "decibels", 10.0, 0.1)]
    Decibel,
    #[log("Np", "nepers", core::f64::consts::E, 1.0)]
    Neper,
}

#[cfg(feature = "angular")]
impl ::core::convert::From<crate::angular::Angle> for Dimensionless {
    fn from(q: crate::angular::Angle) -> Self { q / <crate::angular::Angle>::from_canonical(1.0) }
}

#[cfg(feature = "angular")]
impl ::core::convert::From<Dimensionless> for crate::angular::Angle {
    fn from(q: Dimensionless) -> Self { q * <crate::angular::Angle>::from_canonical(1.0) }
}

#[cfg(feature = "angular")]
impl ::core::convert::From<crate::angular::SolidAngle> for Dimensionless {
    fn from(q: crate::angular::SolidAngle) -> Self { q / <crate::angular::SolidAngle>::from_canonical(1.0) }
}

#[cfg(feature = "angular")]
impl ::core::convert::From<Dimensionless> for crate::angular::SolidAngle {
    fn from(q: Dimensionless) -> Self { q * <crate::angular::SolidAngle>::from_canonical(1.0) }
}
