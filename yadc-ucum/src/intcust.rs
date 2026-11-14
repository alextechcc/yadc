//! The quantities of `intcust` and their units.

#[allow(unused_imports)]
use crate::*;

// `Area` is `crate::Area`.

/// Square inch (`[sin_i]`), a unit of `Area`: `n` of them is `n · 64516e-8` base units.
pub const SQUARE_INCH: Qty<
    {
        UcumUnit::scaled(
            "[sin_i]",
            Exact::dec(64516, -8),
            UcumUnit::ONE.times(METER.unit(), 2),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Square foot (`[sft_i]`), a unit of `Area`: `n` of them is `n · 9290304e-8` base units.
pub const SQUARE_FOOT: Qty<
    {
        UcumUnit::scaled(
            "[sft_i]",
            Exact::dec(9290304, -8),
            UcumUnit::ONE.times(METER.unit(), 2),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Square yard (`[syd_i]`), a unit of `Area`: `n` of them is `n · 83612736e-8` base units.
pub const SQUARE_YARD: Qty<
    {
        UcumUnit::scaled(
            "[syd_i]",
            Exact::dec(83612736, -8),
            UcumUnit::ONE.times(METER.unit(), 2),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Circular mil (`[cml_i]`), a unit of `Area`: `n` of them is `n · 16129e-14 * core::f64::consts::PI` base units.
pub const CIRCULAR_MIL: Qty<
    {
        UcumUnit::scaled(
            "[cml_i]",
            Exact::dec(16129, -14) * Exact::PI,
            UcumUnit::ONE.times(METER.unit(), 2),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// A depth of water, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type DepthOfWater<N, const U: UcumUnit = { METER.unit() }, const B: Base = BASE> = Qty<U, N, B>;

/// Fathom (`[fth_i]`), a unit of `DepthOfWater`: `n` of them is `n · 18288e-4` base units.
pub const FATHOM: Qty<
    { UcumUnit::scaled("[fth_i]", Exact::dec(18288, -4), METER.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// A height of horses, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type HeightOfHorses<N, const U: UcumUnit = { METER.unit() }, const B: Base = BASE> =
    Qty<U, N, B>;

/// Hand (`[hd_i]`), a unit of `HeightOfHorses`: `n` of them is `n · 1016e-4` base units.
pub const HAND: Qty<
    { UcumUnit::scaled("[hd_i]", Exact::dec(1016, -4), METER.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Length` is `crate::Length`.

/// Inch (`[in_i]`), a unit of `Length`: `n` of them is `n · 254e-4` base units.
pub const INCH: Qty<
    { UcumUnit::scaled("[in_i]", Exact::dec(254, -4), METER.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Foot (`[ft_i]`), a unit of `Length`: `n` of them is `n · 3048e-4` base units.
pub const FOOT: Qty<
    { UcumUnit::scaled("[ft_i]", Exact::dec(3048, -4), METER.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Yard (`[yd_i]`), a unit of `Length`: `n` of them is `n · 9144e-4` base units.
pub const YARD: Qty<
    { UcumUnit::scaled("[yd_i]", Exact::dec(9144, -4), METER.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Mile (`[mi_i]`), a unit of `Length`: `n` of them is `n · 1609344e-3` base units.
pub const MILE: Qty<
    { UcumUnit::scaled("[mi_i]", Exact::dec(1609344, -3), METER.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Nautical mile (`[nmi_i]`), a unit of `Length`: `n` of them is `n · 1852` base units.
pub const NAUTICAL_MILE: Qty<
    { UcumUnit::scaled("[nmi_i]", Exact::int(1852), METER.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Mil (`[mil_i]`), a unit of `Length`: `n` of them is `n · 254e-7` base units.
pub const MIL: Qty<
    { UcumUnit::scaled("[mil_i]", Exact::dec(254, -7), METER.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Velocity` is `crate::Velocity`.

/// Knot (`[kn_i]`), a unit of `Velocity`: `n` of them is `n · 463 / 900` base units.
pub const KNOT: Qty<
    {
        UcumUnit::scaled(
            "[kn_i]",
            Exact::int(463) / Exact::int(900),
            METER.unit().times(SECOND.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Volume` is `crate::Volume`.

/// Cubic inch (`[cin_i]`), a unit of `Volume`: `n` of them is `n · 16387064e-12` base units.
pub const CUBIC_INCH: Qty<
    {
        UcumUnit::scaled(
            "[cin_i]",
            Exact::dec(16387064, -12),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Cubic foot (`[cft_i]`), a unit of `Volume`: `n` of them is `n · 28316846592e-12` base units.
pub const CUBIC_FOOT: Qty<
    {
        UcumUnit::scaled(
            "[cft_i]",
            Exact::dec(28316846592, -12),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Cubic yard (`[cyd_i]`), a unit of `Volume`: `n` of them is `n · 764554857984e-12` base units.
pub const CUBIC_YARD: Qty<
    {
        UcumUnit::scaled(
            "[cyd_i]",
            Exact::dec(764554857984, -12),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Board foot (`[bf_i]`), a unit of `Volume`: `n` of them is `n · 2359737216e-12` base units.
pub const BOARD_FOOT: Qty<
    {
        UcumUnit::scaled(
            "[bf_i]",
            Exact::dec(2359737216, -12),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Cord (`[cr_i]`), a unit of `Volume`: `n` of them is `n · 3624556363776e-12` base units.
pub const CORD: Qty<
    {
        UcumUnit::scaled(
            "[cr_i]",
            Exact::dec(3624556363776, -12),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;
