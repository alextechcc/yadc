use crate::{ISQ, StaticSIQuantity};
use yadc::{Dimension, Unit};

pub type Dimensionless = StaticSIQuantity<{ ISQ { ..ISQ::ZERO } }>;

// Variants mirror uom's `ratio.rs` quantity.
#[derive(Copy, Clone, Unit)]
#[dim(Dimensionless)]
pub enum DimensionlessUnit {
    #[unit(1.0, "", "")]
    Ratio,
    #[unit(1.0e-2, "parts per hundred", "parts per hundred")]
    PartPerHundred,
    #[unit(1.0e-2, "%", "percent")]
    Percent,
    #[unit(1.0e-3, "parts per thousand", "parts per thousand")]
    PartPerThousand,
    #[unit(1.0e-3, "‰", "per mille")]
    PerMille,
    #[unit(1.0e-4, "parts per ten thousand", "parts per ten thousand")]
    PartPerTenThousand,
    #[unit(1.0e-4, "bp", "basis points")]
    BasisPoint,
    #[unit(1.0e-6, "ppm", "parts per million")]
    PartPerMillion,
    #[unit(1.0e-9, "ppb", "parts per billion")]
    PartPerBillion,
    #[unit(1.0e-12, "ppt", "parts per trillion")]
    PartPerTrillion,
    #[unit(1.0e-15, "ppq", "parts per quadrillion")]
    PartPerQuadrillion,
}
