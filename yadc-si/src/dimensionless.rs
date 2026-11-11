//! Plain numbers: ratios and their units.

use crate::{Exact, ISQ, SIQuantity};
use consts::*;
use yadc::Dimension;

pub type Dimensionless<N> = SIQuantity<{ ISQ::DIMENSIONLESS }, N>;

/// The constants of this module: the base constants of its quantities
/// (re-exported), then one per named unit, each defined exactly from them
/// (`INCH` is `0.0254 * METRE`). A unit written as others is not one:
/// `SLUG / CUBIC_FOOT`, `KILO * METRE` are exact too. Converting rounds,
/// once, where it is written - in a `const`, by the compiler:
/// `const MPH: Velocity<f64> = (MILE / HOUR).into();`, then `MPH(60.0)`.
pub mod consts {
    use super::*;

    /// The number one: the unit of dimension one.
    pub use crate::base::ONE;

    /// One `Ratio`, as a `Dimensionless`.
    pub const RATIO: Dimensionless<Exact> = ONE;

    /// One `PartPerHundred` (parts per hundred), as a `Dimensionless`.
    pub const PART_PER_HUNDRED: Dimensionless<Exact> = RATIO * Exact::dec(1, -2);

    /// One `Percent` (%), as a `Dimensionless`.
    pub const PERCENT: Dimensionless<Exact> = RATIO * Exact::dec(1, -2);

    /// One `PartPerThousand` (parts per thousand), as a `Dimensionless`.
    pub const PART_PER_THOUSAND: Dimensionless<Exact> = RATIO * Exact::dec(1, -3);

    /// One `PerMille` (‰), as a `Dimensionless`.
    pub const PER_MILLE: Dimensionless<Exact> = RATIO * Exact::dec(1, -3);

    /// One `PartPerTenThousand` (parts per ten thousand), as a `Dimensionless`.
    pub const PART_PER_TEN_THOUSAND: Dimensionless<Exact> = RATIO * Exact::dec(1, -4);

    /// One `BasisPoint` (bp), as a `Dimensionless`.
    pub const BASIS_POINT: Dimensionless<Exact> = RATIO * Exact::dec(1, -4);

    /// One `PartPerMillion` (ppm), as a `Dimensionless`.
    pub const PART_PER_MILLION: Dimensionless<Exact> = RATIO * Exact::dec(1, -6);

    /// One `PartPerBillion` (ppb), as a `Dimensionless`.
    pub const PART_PER_BILLION: Dimensionless<Exact> = RATIO * Exact::dec(1, -9);

    /// One `PartPerTrillion` (ppt), as a `Dimensionless`.
    pub const PART_PER_TRILLION: Dimensionless<Exact> = RATIO * Exact::dec(1, -12);

    /// One `PartPerQuadrillion` (ppq), as a `Dimensionless`.
    pub const PART_PER_QUADRILLION: Dimensionless<Exact> = RATIO * Exact::dec(1, -15);
}

/// The units of `Dimensionless`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Dimensionless<N>, number = N)]
#[unit(for<N> where Dimensionless<N>: [const] yadc::Unit<Dimensionless<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum DimensionlessUnit {
    #[default]
    #[unit("", "", by = Dimensionless::<N>::from(RATIO))]
    Ratio,
    #[unit("parts per hundred", "parts per hundred", by = Dimensionless::<N>::from(PART_PER_HUNDRED))]
    PartPerHundred,
    #[unit("%", "percent", by = Dimensionless::<N>::from(PERCENT))]
    Percent,
    #[unit("parts per thousand", "parts per thousand", by = Dimensionless::<N>::from(PART_PER_THOUSAND))]
    PartPerThousand,
    #[unit("‰", "per mille", by = Dimensionless::<N>::from(PER_MILLE))]
    PerMille,
    #[unit("parts per ten thousand", "parts per ten thousand", by = Dimensionless::<N>::from(PART_PER_TEN_THOUSAND))]
    PartPerTenThousand,
    #[unit("bp", "basis points", by = Dimensionless::<N>::from(BASIS_POINT))]
    BasisPoint,
    #[unit("ppm", "parts per million", by = Dimensionless::<N>::from(PART_PER_MILLION))]
    PartPerMillion,
    #[unit("ppb", "parts per billion", by = Dimensionless::<N>::from(PART_PER_BILLION))]
    PartPerBillion,
    #[unit("ppt", "parts per trillion", by = Dimensionless::<N>::from(PART_PER_TRILLION))]
    PartPerTrillion,
    #[unit("ppq", "parts per quadrillion", by = Dimensionless::<N>::from(PART_PER_QUADRILLION))]
    PartPerQuadrillion,
}
