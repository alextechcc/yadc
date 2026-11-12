//! Plain numbers: ratios and their units.

use crate::{Dim, Exact, Qty};
use consts::*;
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
    #[unit("10*", "the number ten for arbitrary powers", by = Dimensionless::<N>::from(THE_NUMBER_TEN_FOR_ARBITRARY_POWERS))]
    TheNumberTenForArbitraryPowers,
    #[unit("[pi]", "the number pi", by = Dimensionless::<N>::from(THE_NUMBER_PI))]
    TheNumberPi,
    #[unit("%", "percent", by = Dimensionless::<N>::from(PERCENT))]
    Percent,
    #[unit("[ppth]", "parts per thousand", by = Dimensionless::<N>::from(PARTS_PER_THOUSAND))]
    PartsPerThousand,
    #[unit("[ppm]", "parts per million", by = Dimensionless::<N>::from(PARTS_PER_MILLION))]
    PartsPerMillion,
    #[unit("[ppb]", "parts per billion", by = Dimensionless::<N>::from(PARTS_PER_BILLION))]
    PartsPerBillion,
    #[unit("[pptr]", "parts per trillion", by = Dimensionless::<N>::from(PARTS_PER_TRILLION))]
    PartsPerTrillion,
}
