//! Plain numbers: ratios and their units.

use crate::ISQ;
use yadc::Dimension;

/// The dimension of `Dimensionless`: `SIQuantity<DIMENSIONLESS, N>` is one.
pub const DIMENSIONLESS: ISQ = ISQ::DIMENSIONLESS;

/// The units of this module's quantities: each a constant of its own unit
/// type, its size and zero in its quantity's coherent unit, exact
/// (`MILE.quantity(3.0)`, `MILE.value(d)`).
pub mod units {
    use super::*;
    use crate::{Exact, SIUnit, SIUnitConsts};

    /// Ratio, a unit of `Dimensionless`: `n` of them is `n · 1.0` coherent units.
    pub const RATIO: SIUnit<
        DIMENSIONLESS,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "",
                name: "",
            }
        },
    > = SIUnit;

    /// Parts per hundred (`parts per hundred`), a unit of `Dimensionless`: `n` of them is `n · 0.01` coherent units.
    pub const PART_PER_HUNDRED: SIUnit<
        DIMENSIONLESS,
        {
            SIUnitConsts {
                size: Exact::dec(1, -2),
                zero: Exact::ZERO,
                symbol: "parts per hundred",
                name: "parts per hundred",
            }
        },
    > = SIUnit;

    /// Percent (`%`), a unit of `Dimensionless`: `n` of them is `n · 0.01` coherent units.
    pub const PERCENT: SIUnit<
        DIMENSIONLESS,
        {
            SIUnitConsts {
                size: Exact::dec(1, -2),
                zero: Exact::ZERO,
                symbol: "%",
                name: "percent",
            }
        },
    > = SIUnit;

    /// Parts per thousand (`parts per thousand`), a unit of `Dimensionless`: `n` of them is `n · 0.001` coherent units.
    pub const PART_PER_THOUSAND: SIUnit<
        DIMENSIONLESS,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "parts per thousand",
                name: "parts per thousand",
            }
        },
    > = SIUnit;

    /// Per mille (`‰`), a unit of `Dimensionless`: `n` of them is `n · 0.001` coherent units.
    pub const PER_MILLE: SIUnit<
        DIMENSIONLESS,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "‰",
                name: "per mille",
            }
        },
    > = SIUnit;

    /// Parts per ten thousand (`parts per ten thousand`), a unit of `Dimensionless`: `n` of them is `n · 1.0e-4` coherent units.
    pub const PART_PER_TEN_THOUSAND: SIUnit<
        DIMENSIONLESS,
        {
            SIUnitConsts {
                size: Exact::dec(1, -4),
                zero: Exact::ZERO,
                symbol: "parts per ten thousand",
                name: "parts per ten thousand",
            }
        },
    > = SIUnit;

    /// Basis points (`bp`), a unit of `Dimensionless`: `n` of them is `n · 1.0e-4` coherent units.
    pub const BASIS_POINT: SIUnit<
        DIMENSIONLESS,
        {
            SIUnitConsts {
                size: Exact::dec(1, -4),
                zero: Exact::ZERO,
                symbol: "bp",
                name: "basis points",
            }
        },
    > = SIUnit;

    /// Parts per million (`ppm`), a unit of `Dimensionless`: `n` of them is `n · 1e-06` coherent units.
    pub const PART_PER_MILLION: SIUnit<
        DIMENSIONLESS,
        {
            SIUnitConsts {
                size: Exact::dec(1, -6),
                zero: Exact::ZERO,
                symbol: "ppm",
                name: "parts per million",
            }
        },
    > = SIUnit;

    /// Parts per billion (`ppb`), a unit of `Dimensionless`: `n` of them is `n · 1e-09` coherent units.
    pub const PART_PER_BILLION: SIUnit<
        DIMENSIONLESS,
        {
            SIUnitConsts {
                size: Exact::dec(1, -9),
                zero: Exact::ZERO,
                symbol: "ppb",
                name: "parts per billion",
            }
        },
    > = SIUnit;

    /// Parts per trillion (`ppt`), a unit of `Dimensionless`: `n` of them is `n · 1e-12` coherent units.
    pub const PART_PER_TRILLION: SIUnit<
        DIMENSIONLESS,
        {
            SIUnitConsts {
                size: Exact::dec(1, -12),
                zero: Exact::ZERO,
                symbol: "ppt",
                name: "parts per trillion",
            }
        },
    > = SIUnit;

    /// Parts per quadrillion (`ppq`), a unit of `Dimensionless`: `n` of them is `n · 1e-15` coherent units.
    pub const PART_PER_QUADRILLION: SIUnit<
        DIMENSIONLESS,
        {
            SIUnitConsts {
                size: Exact::dec(1, -15),
                zero: Exact::ZERO,
                symbol: "ppq",
                name: "parts per quadrillion",
            }
        },
    > = SIUnit;
}
