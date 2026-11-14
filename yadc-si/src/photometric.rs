//! The quantities of `photometric` and their units.

#[allow(unused_imports)]
use crate::*;

/// A luminance, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Luminance<
    N,
    const U: SIUnit = { SIUnit::ONE.times(METRE.unit(), -2).times(CANDELA.unit(), 1) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Footlamberts (`fl`), a unit of `Luminance`: `n` of them is `n · 1.0 / (core::f64::consts::PI * 0.3048 * 0.3048)` base units.
pub const FOOTLAMBERT: SIQuantity<
    {
        SIUnit::scaled(
            "fl",
            Exact::int(1) / (Exact::PI * Exact::dec(3048, -4) * Exact::dec(3048, -4)),
            SIUnit::ONE.times(METRE.unit(), -2).times(CANDELA.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Lamberts (`la`), a unit of `Luminance`: `n` of them is `n · 10000.0 / core::f64::consts::PI` base units.
pub const LAMBERT: SIQuantity<
    {
        SIUnit::scaled(
            "la",
            Exact::int(10000) / Exact::PI,
            SIUnit::ONE.times(METRE.unit(), -2).times(CANDELA.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Stilbs (`sb`), a unit of `Luminance`: `n` of them is `n · 10000.0` base units.
pub const STILB: SIQuantity<
    {
        SIUnit::scaled(
            "sb",
            Exact::int(10000),
            SIUnit::ONE.times(METRE.unit(), -2).times(CANDELA.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// A luminous intensity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type LuminousIntensity<N, const U: SIUnit = { CANDELA.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// An illuminance, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Illuminance<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), -2)
            .times(CANDELA.unit(), 1)
            .times(RADIAN.unit(), 2)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Lux (`lx`), a unit of `Illuminance`: the coherent unit, a product of base units.
pub const LUX: SIQuantity<
    {
        SIUnit::ONE
            .times(METRE.unit(), -2)
            .times(CANDELA.unit(), 1)
            .times(RADIAN.unit(), 2)
    },
    Exact,
    BASE,
> = SIQuantity::ONE;

/// Phot (`ph`), a unit of `Illuminance`: `n` of them is `n · 10000.0` base units.
pub const PHOT: SIQuantity<
    {
        SIUnit::scaled(
            "ph",
            Exact::int(10000),
            SIUnit::ONE
                .times(METRE.unit(), -2)
                .times(CANDELA.unit(), 1)
                .times(RADIAN.unit(), 2),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Footcandles (`fc`), a unit of `Illuminance`: `n` of them is `n · 1.0 / (0.3048 * 0.3048)` base units.
pub const FOOTCANDLE: SIQuantity<
    {
        SIUnit::scaled(
            "fc",
            Exact::int(1) / (Exact::dec(3048, -4) * Exact::dec(3048, -4)),
            SIUnit::ONE
                .times(METRE.unit(), -2)
                .times(CANDELA.unit(), 1)
                .times(RADIAN.unit(), 2),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// A radiant exposure, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type RadiantExposure<
    N,
    const U: SIUnit = { KILOGRAM.unit().times(SECOND.unit(), -2) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;
