//! Plain numbers: ratios and their units.

use crate::{ISQ, StaticSIQuantity};
use consts::*;
use yadc::Dimension;

pub type Dimensionless = StaticSIQuantity<{ ISQ::DIMENSIONLESS }>;

/// The constants of this module: the base constants of its quantities
/// (re-exported), then one per multiple, each defined from them - `KILOMETRE`
/// is `1000.0 * METRE`, a `Length` and a unit of its own (`KILOMETRE.value(d)`).
/// Quantity arithmetic is `const`, so they compose in `const` items:
/// `const LIMIT: Velocity = 50.0 * KILOMETRE / HOUR;`.
pub mod consts {
    use super::*;

    /// The number one: the unit of dimension one.
    pub use crate::base::ONE;

    /// One `Ratio`, as a `Dimensionless`.
    pub const RATIO: Dimensionless = ONE;

    /// One `PartPerHundred` (parts per hundred), as a `Dimensionless`.
    pub const PART_PER_HUNDRED: Dimensionless = 1.0e-2 * RATIO;

    /// One `Percent` (%), as a `Dimensionless`.
    pub const PERCENT: Dimensionless = 1.0e-2 * RATIO;

    /// One `PartPerThousand` (parts per thousand), as a `Dimensionless`.
    pub const PART_PER_THOUSAND: Dimensionless = 1.0e-3 * RATIO;

    /// One `PerMille` (‰), as a `Dimensionless`.
    pub const PER_MILLE: Dimensionless = 1.0e-3 * RATIO;

    /// One `PartPerTenThousand` (parts per ten thousand), as a `Dimensionless`.
    pub const PART_PER_TEN_THOUSAND: Dimensionless = 1.0e-4 * RATIO;

    /// One `BasisPoint` (bp), as a `Dimensionless`.
    pub const BASIS_POINT: Dimensionless = 1.0e-4 * RATIO;

    /// One `PartPerMillion` (ppm), as a `Dimensionless`.
    pub const PART_PER_MILLION: Dimensionless = 1.0e-6 * RATIO;

    /// One `PartPerBillion` (ppb), as a `Dimensionless`.
    pub const PART_PER_BILLION: Dimensionless = 1.0e-9 * RATIO;

    /// One `PartPerTrillion` (ppt), as a `Dimensionless`.
    pub const PART_PER_TRILLION: Dimensionless = 1.0e-12 * RATIO;

    /// One `PartPerQuadrillion` (ppq), as a `Dimensionless`.
    pub const PART_PER_QUADRILLION: Dimensionless = 1.0e-15 * RATIO;
}

/// The units of `Dimensionless`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Dimensionless)]
pub enum DimensionlessUnit {
    #[default]
    #[unit("", "", by = RATIO)]
    Ratio,
    #[unit("parts per hundred", "parts per hundred", by = PART_PER_HUNDRED)]
    PartPerHundred,
    #[unit("%", "percent", by = PERCENT)]
    Percent,
    #[unit("parts per thousand", "parts per thousand", by = PART_PER_THOUSAND)]
    PartPerThousand,
    #[unit("‰", "per mille", by = PER_MILLE)]
    PerMille,
    #[unit("parts per ten thousand", "parts per ten thousand", by = PART_PER_TEN_THOUSAND)]
    PartPerTenThousand,
    #[unit("bp", "basis points", by = BASIS_POINT)]
    BasisPoint,
    #[unit("ppm", "parts per million", by = PART_PER_MILLION)]
    PartPerMillion,
    #[unit("ppb", "parts per billion", by = PART_PER_BILLION)]
    PartPerBillion,
    #[unit("ppt", "parts per trillion", by = PART_PER_TRILLION)]
    PartPerTrillion,
    #[unit("ppq", "parts per quadrillion", by = PART_PER_QUADRILLION)]
    PartPerQuadrillion,
}
