//! Plain numbers: ratios, levels and their units.

#[allow(unused_imports)]
use crate::*;

/// A dimensionless, in `N`s of the unit `U`: by default its base unit, `RATIO`.
pub type Dimensionless<N, const U: SIUnit = { RATIO.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Ratio, a unit of `Dimensionless`: the coherent unit, a product of base units.
pub const RATIO: Unit<{ SIUnit::ONE }, BASE> = Unit::ONE;

/// Parts per hundred (`parts per hundred`), a unit of `Dimensionless`: `n` of them is `n · 0.01` base units.
pub const PART_PER_HUNDRED: Unit<
    { SIUnit::scaled("parts per hundred", Exact::dec(1, -2), RATIO.unit()) },
> = Unit::ONE;

/// Percent (`%`), a unit of `Dimensionless`: `n` of them is `n · 0.01` base units.
pub const PERCENT: Unit<{ SIUnit::scaled("%", Exact::dec(1, -2), RATIO.unit()) }> = Unit::ONE;

/// Parts per thousand (`parts per thousand`), a unit of `Dimensionless`: `n` of them is `n · 0.001` base units.
pub const PART_PER_THOUSAND: Unit<
    { SIUnit::scaled("parts per thousand", Exact::dec(1, -3), RATIO.unit()) },
> = Unit::ONE;

/// Per mille (`‰`), a unit of `Dimensionless`: `n` of them is `n · 0.001` base units.
pub const PER_MILLE: Unit<{ SIUnit::scaled("‰", Exact::dec(1, -3), RATIO.unit()) }> = Unit::ONE;

/// Parts per ten thousand (`parts per ten thousand`), a unit of `Dimensionless`: `n` of them is `n · 1.0e-4` base units.
pub const PART_PER_TEN_THOUSAND: Unit<
    { SIUnit::scaled("parts per ten thousand", Exact::dec(1, -4), RATIO.unit()) },
> = Unit::ONE;

/// Basis points (`bp`), a unit of `Dimensionless`: `n` of them is `n · 1.0e-4` base units.
pub const BASIS_POINT: Unit<{ SIUnit::scaled("bp", Exact::dec(1, -4), RATIO.unit()) }> = Unit::ONE;

/// Parts per million (`ppm`), a unit of `Dimensionless`: `n` of them is `n · 1e-06` base units.
pub const PART_PER_MILLION: Unit<{ SIUnit::scaled("ppm", Exact::dec(1, -6), RATIO.unit()) }> =
    Unit::ONE;

/// Parts per billion (`ppb`), a unit of `Dimensionless`: `n` of them is `n · 1e-09` base units.
pub const PART_PER_BILLION: Unit<{ SIUnit::scaled("ppb", Exact::dec(1, -9), RATIO.unit()) }> =
    Unit::ONE;

/// Parts per trillion (`ppt`), a unit of `Dimensionless`: `n` of them is `n · 1e-12` base units.
pub const PART_PER_TRILLION: Unit<{ SIUnit::scaled("ppt", Exact::dec(1, -12), RATIO.unit()) }> =
    Unit::ONE;

/// Parts per quadrillion (`ppq`), a unit of `Dimensionless`: `n` of them is `n · 1e-15` base units.
pub const PART_PER_QUADRILLION: Unit<{ SIUnit::scaled("ppq", Exact::dec(1, -15), RATIO.unit()) }> =
    Unit::ONE;

/// Bels (`B`), a unit of `Dimensionless`: `n` of them is `1.0 · 10.0^(n / 1.0)` base units.
pub const BEL: Unit<
    {
        SIUnit::from_log(
            "B",
            Exact::int(10),
            Exact::int(1),
            Exact::int(1),
            RATIO.unit(),
        )
    },
> = Unit::ONE;

/// Decibels (`dB`), a unit of `Dimensionless`: `n` of them is `1.0 · 10.0^(n / 10.0)` base units.
pub const DECIBEL: Unit<
    {
        SIUnit::from_log(
            "dB",
            Exact::int(10),
            Exact::int(10),
            Exact::int(1),
            RATIO.unit(),
        )
    },
> = Unit::ONE;

/// Nepers (`Np`), a unit of `Dimensionless`: `n` of them is `1.0 · 2.718281828459045^(n / 0.5)` base units.
pub const NEPER: Unit<
    {
        SIUnit::from_log(
            "Np",
            Exact::dec(2718281828459045, -15),
            Exact::dec(5, -1),
            Exact::int(1),
            RATIO.unit(),
        )
    },
> = Unit::ONE;
