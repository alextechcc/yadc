//! Plain numbers: ratios and their units.

use crate::Dim;
use yadc::Dimension;

/// The dimension of `Dimensionless`: `Qty<DIMENSIONLESS, N>` is one.
pub const DIMENSIONLESS: Dim = Dim::DIMENSIONLESS;

/// The units of this module's quantities: each a constant of its own unit
/// type, its size and zero in its quantity's coherent unit, exact
/// (`MILE.quantity(3.0)`, `MILE.value(d)`).
pub mod units {
    use super::*;
    use crate::{Exact, UcumUnit, UcumUnitConsts};

    /// Ratio, a unit of `Dimensionless`: `n` of them is `n · 1.0` coherent units.
    pub const RATIO: UcumUnit<
        DIMENSIONLESS,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "",
                name: "",
            }
        },
    > = UcumUnit;

    /// The number ten for arbitrary powers (`10*`), a unit of `Dimensionless`: `n` of them is `n · 1e1` coherent units.
    pub const THE_NUMBER_TEN_FOR_ARBITRARY_POWERS: UcumUnit<
        DIMENSIONLESS,
        {
            UcumUnitConsts {
                size: Exact::int(10),
                zero: Exact::ZERO,
                symbol: "10*",
                name: "the number ten for arbitrary powers",
            }
        },
    > = UcumUnit;

    /// The number pi (`[pi]`), a unit of `Dimensionless`: `n` of them is `n · 1 * core::f64::consts::PI` coherent units.
    pub const THE_NUMBER_PI: UcumUnit<
        DIMENSIONLESS,
        {
            UcumUnitConsts {
                size: Exact::int(1) * Exact::PI,
                zero: Exact::ZERO,
                symbol: "[pi]",
                name: "the number pi",
            }
        },
    > = UcumUnit;

    /// Percent (`%`), a unit of `Dimensionless`: `n` of them is `n · 1e-2` coherent units.
    pub const PERCENT: UcumUnit<
        DIMENSIONLESS,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -2),
                zero: Exact::ZERO,
                symbol: "%",
                name: "percent",
            }
        },
    > = UcumUnit;

    /// Parts per thousand (`[ppth]`), a unit of `Dimensionless`: `n` of them is `n · 1e-3` coherent units.
    pub const PARTS_PER_THOUSAND: UcumUnit<
        DIMENSIONLESS,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "[ppth]",
                name: "parts per thousand",
            }
        },
    > = UcumUnit;

    /// Parts per million (`[ppm]`), a unit of `Dimensionless`: `n` of them is `n · 1e-6` coherent units.
    pub const PARTS_PER_MILLION: UcumUnit<
        DIMENSIONLESS,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -6),
                zero: Exact::ZERO,
                symbol: "[ppm]",
                name: "parts per million",
            }
        },
    > = UcumUnit;

    /// Parts per billion (`[ppb]`), a unit of `Dimensionless`: `n` of them is `n · 1e-9` coherent units.
    pub const PARTS_PER_BILLION: UcumUnit<
        DIMENSIONLESS,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -9),
                zero: Exact::ZERO,
                symbol: "[ppb]",
                name: "parts per billion",
            }
        },
    > = UcumUnit;

    /// Parts per trillion (`[pptr]`), a unit of `Dimensionless`: `n` of them is `n · 1e-12` coherent units.
    pub const PARTS_PER_TRILLION: UcumUnit<
        DIMENSIONLESS,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -12),
                zero: Exact::ZERO,
                symbol: "[pptr]",
                name: "parts per trillion",
            }
        },
    > = UcumUnit;
}
