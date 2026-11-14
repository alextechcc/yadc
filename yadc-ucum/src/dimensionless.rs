//! Plain numbers: ratios, levels and their units.

#[allow(unused_imports)]
use crate::*;

/// A dimensionless, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Dimensionless<N, const U: UcumUnit = { UcumUnit::ONE }, const IS_BASE: bool = true> =
    Qty<U, N, IS_BASE>;

/// Ratio, a unit of `Dimensionless`: the coherent unit, a product of base units.
pub const RATIO: Qty<{ UcumUnit::ONE }, Exact, true> = Qty::ONE;

/// The number ten for arbitrary powers (`10*`), a unit of `Dimensionless`: `n` of them is `n · 1e1` base units.
pub const THE_NUMBER_TEN_FOR_ARBITRARY_POWERS: Qty<
    { UcumUnit::scaled("10*", Exact::int(10), UcumUnit::ONE) },
    Exact,
    false,
> = Qty::ONE;

/// The number pi (`[pi]`), a unit of `Dimensionless`: `n` of them is `n · 1 * core::f64::consts::PI` base units.
pub const THE_NUMBER_PI: Qty<
    { UcumUnit::scaled("[pi]", Exact::int(1) * Exact::PI, UcumUnit::ONE) },
    Exact,
    false,
> = Qty::ONE;

/// Percent (`%`), a unit of `Dimensionless`: `n` of them is `n · 1e-2` base units.
pub const PERCENT: Qty<{ UcumUnit::scaled("%", Exact::dec(1, -2), UcumUnit::ONE) }, Exact, false> =
    Qty::ONE;

/// Parts per thousand (`[ppth]`), a unit of `Dimensionless`: `n` of them is `n · 1e-3` base units.
pub const PARTS_PER_THOUSAND: Qty<
    { UcumUnit::scaled("[ppth]", Exact::dec(1, -3), UcumUnit::ONE) },
    Exact,
    false,
> = Qty::ONE;

/// Parts per million (`[ppm]`), a unit of `Dimensionless`: `n` of them is `n · 1e-6` base units.
pub const PARTS_PER_MILLION: Qty<
    { UcumUnit::scaled("[ppm]", Exact::dec(1, -6), UcumUnit::ONE) },
    Exact,
    false,
> = Qty::ONE;

/// Parts per billion (`[ppb]`), a unit of `Dimensionless`: `n` of them is `n · 1e-9` base units.
pub const PARTS_PER_BILLION: Qty<
    { UcumUnit::scaled("[ppb]", Exact::dec(1, -9), UcumUnit::ONE) },
    Exact,
    false,
> = Qty::ONE;

/// Parts per trillion (`[pptr]`), a unit of `Dimensionless`: `n` of them is `n · 1e-12` base units.
pub const PARTS_PER_TRILLION: Qty<
    { UcumUnit::scaled("[pptr]", Exact::dec(1, -12), UcumUnit::ONE) },
    Exact,
    false,
> = Qty::ONE;
