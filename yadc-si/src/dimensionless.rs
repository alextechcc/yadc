//! Plain numbers: ratios and their units.

use crate::{ISQ, SIQuantity};
use yadc::Dimension;

pub type Dimensionless<N> = SIQuantity<{ ISQ::DIMENSIONLESS }, N>;

/// The units of this module's quantities: a type each, its size and zero
/// in its quantity's coherent unit, exact (`Mile {}.quantity(3.0_f64)`).
pub mod units {
    use crate::{Exact, ISQ, SIUnit, SIUnitConsts};
    use yadc::Dimension;

    /// Ratio, a unit of `Dimensionless`: `n` of them is `n · 1.0` coherent units.
    pub type Ratio = SIUnit<
        { ISQ::DIMENSIONLESS },
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "",
                name: "",
            }
        },
    >;

    /// Parts per hundred (`parts per hundred`), a unit of `Dimensionless`: `n` of them is `n · 0.01` coherent units.
    pub type PartPerHundred = SIUnit<
        { ISQ::DIMENSIONLESS },
        {
            SIUnitConsts {
                size: Exact::dec(1, -2),
                zero: Exact::ZERO,
                symbol: "parts per hundred",
                name: "parts per hundred",
            }
        },
    >;

    /// Percent (`%`), a unit of `Dimensionless`: `n` of them is `n · 0.01` coherent units.
    pub type Percent = SIUnit<
        { ISQ::DIMENSIONLESS },
        {
            SIUnitConsts {
                size: Exact::dec(1, -2),
                zero: Exact::ZERO,
                symbol: "%",
                name: "percent",
            }
        },
    >;

    /// Parts per thousand (`parts per thousand`), a unit of `Dimensionless`: `n` of them is `n · 0.001` coherent units.
    pub type PartPerThousand = SIUnit<
        { ISQ::DIMENSIONLESS },
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "parts per thousand",
                name: "parts per thousand",
            }
        },
    >;

    /// Per mille (`‰`), a unit of `Dimensionless`: `n` of them is `n · 0.001` coherent units.
    pub type PerMille = SIUnit<
        { ISQ::DIMENSIONLESS },
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "‰",
                name: "per mille",
            }
        },
    >;

    /// Parts per ten thousand (`parts per ten thousand`), a unit of `Dimensionless`: `n` of them is `n · 1.0e-4` coherent units.
    pub type PartPerTenThousand = SIUnit<
        { ISQ::DIMENSIONLESS },
        {
            SIUnitConsts {
                size: Exact::dec(1, -4),
                zero: Exact::ZERO,
                symbol: "parts per ten thousand",
                name: "parts per ten thousand",
            }
        },
    >;

    /// Basis points (`bp`), a unit of `Dimensionless`: `n` of them is `n · 1.0e-4` coherent units.
    pub type BasisPoint = SIUnit<
        { ISQ::DIMENSIONLESS },
        {
            SIUnitConsts {
                size: Exact::dec(1, -4),
                zero: Exact::ZERO,
                symbol: "bp",
                name: "basis points",
            }
        },
    >;

    /// Parts per million (`ppm`), a unit of `Dimensionless`: `n` of them is `n · 1e-06` coherent units.
    pub type PartPerMillion = SIUnit<
        { ISQ::DIMENSIONLESS },
        {
            SIUnitConsts {
                size: Exact::dec(1, -6),
                zero: Exact::ZERO,
                symbol: "ppm",
                name: "parts per million",
            }
        },
    >;

    /// Parts per billion (`ppb`), a unit of `Dimensionless`: `n` of them is `n · 1e-09` coherent units.
    pub type PartPerBillion = SIUnit<
        { ISQ::DIMENSIONLESS },
        {
            SIUnitConsts {
                size: Exact::dec(1, -9),
                zero: Exact::ZERO,
                symbol: "ppb",
                name: "parts per billion",
            }
        },
    >;

    /// Parts per trillion (`ppt`), a unit of `Dimensionless`: `n` of them is `n · 1e-12` coherent units.
    pub type PartPerTrillion = SIUnit<
        { ISQ::DIMENSIONLESS },
        {
            SIUnitConsts {
                size: Exact::dec(1, -12),
                zero: Exact::ZERO,
                symbol: "ppt",
                name: "parts per trillion",
            }
        },
    >;

    /// Parts per quadrillion (`ppq`), a unit of `Dimensionless`: `n` of them is `n · 1e-15` coherent units.
    pub type PartPerQuadrillion = SIUnit<
        { ISQ::DIMENSIONLESS },
        {
            SIUnitConsts {
                size: Exact::dec(1, -15),
                zero: Exact::ZERO,
                symbol: "ppq",
                name: "parts per quadrillion",
            }
        },
    >;
}
