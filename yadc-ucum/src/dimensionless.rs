//! Plain numbers: ratios, levels and their units.

#[allow(unused_imports)]
use crate::*;

/// A dimensionless, in `N`s of the unit `U`: by default its base unit, `RATIO`.
pub type Dimensionless<N, const U: UcumUnit = { RATIO.unit() }, const B: Base = BASE> =
    Qty<U, N, B>;

/// Ratio, a unit of `Dimensionless`: the coherent unit, a product of base units.
pub const RATIO: Unit<{ UcumUnit::ONE }, BASE> = Unit::ONE;

/// The number ten for arbitrary powers (`10*`), a unit of `Dimensionless`: `n` of them is `n · 1e1` base units.
pub const THE_NUMBER_TEN_FOR_ARBITRARY_POWERS: Unit<
    { UcumUnit::scaled("10*", Exact::int(10), RATIO.unit()) },
> = Unit::ONE;

/// The number pi (`[pi]`), a unit of `Dimensionless`: `n` of them is `n · 1 * core::f64::consts::PI` base units.
pub const THE_NUMBER_PI: Unit<
    { UcumUnit::scaled("[pi]", Exact::int(1) * Exact::PI, RATIO.unit()) },
> = Unit::ONE;

/// Percent (`%`), a unit of `Dimensionless`: `n` of them is `n · 1e-2` base units.
pub const PERCENT: Unit<{ UcumUnit::scaled("%", Exact::dec(1, -2), RATIO.unit()) }> = Unit::ONE;

/// Parts per thousand (`[ppth]`), a unit of `Dimensionless`: `n` of them is `n · 1e-3` base units.
pub const PARTS_PER_THOUSAND: Unit<
    { UcumUnit::scaled("[ppth]", Exact::dec(1, -3), RATIO.unit()) },
> = Unit::ONE;

/// Parts per million (`[ppm]`), a unit of `Dimensionless`: `n` of them is `n · 1e-6` base units.
pub const PARTS_PER_MILLION: Unit<{ UcumUnit::scaled("[ppm]", Exact::dec(1, -6), RATIO.unit()) }> =
    Unit::ONE;

/// Parts per billion (`[ppb]`), a unit of `Dimensionless`: `n` of them is `n · 1e-9` base units.
pub const PARTS_PER_BILLION: Unit<{ UcumUnit::scaled("[ppb]", Exact::dec(1, -9), RATIO.unit()) }> =
    Unit::ONE;

/// Parts per trillion (`[pptr]`), a unit of `Dimensionless`: `n` of them is `n · 1e-12` base units.
pub const PARTS_PER_TRILLION: Unit<
    { UcumUnit::scaled("[pptr]", Exact::dec(1, -12), RATIO.unit()) },
> = Unit::ONE;
