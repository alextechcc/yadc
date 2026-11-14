//! Plain numbers: ratios, levels and their units.

#[allow(unused_imports)]
use crate::*;

/// A dimensionless, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Dimensionless<N, const U: SIUnit = { SIUnit::ONE }, const IS_BASE: bool = true> =
    SIQuantity<U, N, IS_BASE>;

/// Ratio, a unit of `Dimensionless`: the coherent unit, a product of base units.
pub const RATIO: SIQuantity<{ SIUnit::ONE }, Exact, true> = SIQuantity::ONE;

/// Parts per hundred (`parts per hundred`), a unit of `Dimensionless`: `n` of them is `n · 0.01` base units.
pub const PART_PER_HUNDRED: SIQuantity<
    { SIUnit::scaled("parts per hundred", Exact::dec(1, -2), SIUnit::ONE) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Percent (`%`), a unit of `Dimensionless`: `n` of them is `n · 0.01` base units.
pub const PERCENT: SIQuantity<
    { SIUnit::scaled("%", Exact::dec(1, -2), SIUnit::ONE) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Parts per thousand (`parts per thousand`), a unit of `Dimensionless`: `n` of them is `n · 0.001` base units.
pub const PART_PER_THOUSAND: SIQuantity<
    { SIUnit::scaled("parts per thousand", Exact::dec(1, -3), SIUnit::ONE) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Per mille (`‰`), a unit of `Dimensionless`: `n` of them is `n · 0.001` base units.
pub const PER_MILLE: SIQuantity<
    { SIUnit::scaled("‰", Exact::dec(1, -3), SIUnit::ONE) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Parts per ten thousand (`parts per ten thousand`), a unit of `Dimensionless`: `n` of them is `n · 1.0e-4` base units.
pub const PART_PER_TEN_THOUSAND: SIQuantity<
    { SIUnit::scaled("parts per ten thousand", Exact::dec(1, -4), SIUnit::ONE) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Basis points (`bp`), a unit of `Dimensionless`: `n` of them is `n · 1.0e-4` base units.
pub const BASIS_POINT: SIQuantity<
    { SIUnit::scaled("bp", Exact::dec(1, -4), SIUnit::ONE) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Parts per million (`ppm`), a unit of `Dimensionless`: `n` of them is `n · 1e-06` base units.
pub const PART_PER_MILLION: SIQuantity<
    { SIUnit::scaled("ppm", Exact::dec(1, -6), SIUnit::ONE) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Parts per billion (`ppb`), a unit of `Dimensionless`: `n` of them is `n · 1e-09` base units.
pub const PART_PER_BILLION: SIQuantity<
    { SIUnit::scaled("ppb", Exact::dec(1, -9), SIUnit::ONE) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Parts per trillion (`ppt`), a unit of `Dimensionless`: `n` of them is `n · 1e-12` base units.
pub const PART_PER_TRILLION: SIQuantity<
    { SIUnit::scaled("ppt", Exact::dec(1, -12), SIUnit::ONE) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Parts per quadrillion (`ppq`), a unit of `Dimensionless`: `n` of them is `n · 1e-15` base units.
pub const PART_PER_QUADRILLION: SIQuantity<
    { SIUnit::scaled("ppq", Exact::dec(1, -15), SIUnit::ONE) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Bels (`B`), a unit of `Dimensionless`: `n` of them is `1.0 · 10.0^(n / 1.0)` base units.
pub const BEL: SIQuantity<
    {
        SIUnit::from_log(
            "B",
            Exact::int(10),
            Exact::int(1),
            Exact::int(1),
            SIUnit::ONE,
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Decibels (`dB`), a unit of `Dimensionless`: `n` of them is `1.0 · 10.0^(n / 10.0)` base units.
pub const DECIBEL: SIQuantity<
    {
        SIUnit::from_log(
            "dB",
            Exact::int(10),
            Exact::int(10),
            Exact::int(1),
            SIUnit::ONE,
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Nepers (`Np`), a unit of `Dimensionless`: `n` of them is `1.0 · 2.718281828459045^(n / 0.5)` base units.
pub const NEPER: SIQuantity<
    {
        SIUnit::from_log(
            "Np",
            Exact::dec(2718281828459045, -15),
            Exact::dec(5, -1),
            Exact::int(1),
            SIUnit::ONE,
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;
