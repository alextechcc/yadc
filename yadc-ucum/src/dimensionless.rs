//! Plain numbers: ratios and their units.

use crate::{Dim, Qty};
use yadc::Dimension;

pub type Dimensionless<N> = Qty<{ Dim::DIMENSIONLESS }, N>;

/// The units of this module's quantities: a type each, its size and zero
/// in its quantity's coherent unit, exact (`Mile {}.quantity(3.0_f64)`).
pub mod units {
    use crate::{Dim, Exact, UcumUnit, UcumUnitConsts};
    use yadc::Dimension;

    /// Ratio, a unit of `Dimensionless`: `n` of them is `n · 1.0` coherent units.
    pub type Ratio = UcumUnit<
        { Dim::DIMENSIONLESS },
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "",
                name: "",
            }
        },
    >;

    /// The number ten for arbitrary powers (`10*`), a unit of `Dimensionless`: `n` of them is `n · 1e1` coherent units.
    pub type TheNumberTenForArbitraryPowers = UcumUnit<
        { Dim::DIMENSIONLESS },
        {
            UcumUnitConsts {
                size: Exact::int(10),
                zero: Exact::ZERO,
                symbol: "10*",
                name: "the number ten for arbitrary powers",
            }
        },
    >;

    /// The number pi (`[pi]`), a unit of `Dimensionless`: `n` of them is `n · 1 * core::f64::consts::PI` coherent units.
    pub type TheNumberPi = UcumUnit<
        { Dim::DIMENSIONLESS },
        {
            UcumUnitConsts {
                size: Exact::int(1) * Exact::PI,
                zero: Exact::ZERO,
                symbol: "[pi]",
                name: "the number pi",
            }
        },
    >;

    /// Percent (`%`), a unit of `Dimensionless`: `n` of them is `n · 1e-2` coherent units.
    pub type Percent = UcumUnit<
        { Dim::DIMENSIONLESS },
        {
            UcumUnitConsts {
                size: Exact::dec(1, -2),
                zero: Exact::ZERO,
                symbol: "%",
                name: "percent",
            }
        },
    >;

    /// Parts per thousand (`[ppth]`), a unit of `Dimensionless`: `n` of them is `n · 1e-3` coherent units.
    pub type PartsPerThousand = UcumUnit<
        { Dim::DIMENSIONLESS },
        {
            UcumUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "[ppth]",
                name: "parts per thousand",
            }
        },
    >;

    /// Parts per million (`[ppm]`), a unit of `Dimensionless`: `n` of them is `n · 1e-6` coherent units.
    pub type PartsPerMillion = UcumUnit<
        { Dim::DIMENSIONLESS },
        {
            UcumUnitConsts {
                size: Exact::dec(1, -6),
                zero: Exact::ZERO,
                symbol: "[ppm]",
                name: "parts per million",
            }
        },
    >;

    /// Parts per billion (`[ppb]`), a unit of `Dimensionless`: `n` of them is `n · 1e-9` coherent units.
    pub type PartsPerBillion = UcumUnit<
        { Dim::DIMENSIONLESS },
        {
            UcumUnitConsts {
                size: Exact::dec(1, -9),
                zero: Exact::ZERO,
                symbol: "[ppb]",
                name: "parts per billion",
            }
        },
    >;

    /// Parts per trillion (`[pptr]`), a unit of `Dimensionless`: `n` of them is `n · 1e-12` coherent units.
    pub type PartsPerTrillion = UcumUnit<
        { Dim::DIMENSIONLESS },
        {
            UcumUnitConsts {
                size: Exact::dec(1, -12),
                zero: Exact::ZERO,
                symbol: "[pptr]",
                name: "parts per trillion",
            }
        },
    >;
}
