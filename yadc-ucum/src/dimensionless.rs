//! Plain numbers: ratios and their units.

use crate::{Dim, Exact, Qty};
use yadc::Dimension;

pub type Dimensionless<N> = Qty<{ Dim::DIMENSIONLESS }, N>;

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

    /// One `TheNumberTenForArbitraryPowers` (`10*`), as a `Dimensionless`.
    pub const THE_NUMBER_TEN_FOR_ARBITRARY_POWERS: Dimensionless<Exact> = RATIO * Exact::int(10);

    /// One `TheNumberPi` (`[pi]`), as a `Dimensionless`.
    pub const THE_NUMBER_PI: Dimensionless<Exact> = RATIO * (Exact::int(1) * Exact::PI);

    /// One `Percent` (`%`), as a `Dimensionless`.
    pub const PERCENT: Dimensionless<Exact> = RATIO * Exact::dec(1, -2);

    /// One `PartsPerThousand` (`[ppth]`), as a `Dimensionless`.
    pub const PARTS_PER_THOUSAND: Dimensionless<Exact> = RATIO * Exact::dec(1, -3);

    /// One `PartsPerMillion` (`[ppm]`), as a `Dimensionless`.
    pub const PARTS_PER_MILLION: Dimensionless<Exact> = RATIO * Exact::dec(1, -6);

    /// One `PartsPerBillion` (`[ppb]`), as a `Dimensionless`.
    pub const PARTS_PER_BILLION: Dimensionless<Exact> = RATIO * Exact::dec(1, -9);

    /// One `PartsPerTrillion` (`[pptr]`), as a `Dimensionless`.
    pub const PARTS_PER_TRILLION: Dimensionless<Exact> = RATIO * Exact::dec(1, -12);
}
