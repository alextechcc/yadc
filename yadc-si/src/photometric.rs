//! The quantities of `photometric` and their units.

#[allow(unused_imports)]
use crate::*;

/// A luminance, in `N`s of the unit `U`: by default its base unit, `CANDELA_PER_METRE_SQUARED`.
pub type Luminance<
    N,
    const U: SIUnit = { CANDELA_PER_METRE_SQUARED.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `Luminance`: a product of base units.
pub const CANDELA_PER_METRE_SQUARED: Unit<{ (CANDELA / (METRE * METRE)).unit() }, BASE> = Unit::ONE;

/// Footlamberts (`fl`), a unit of `Luminance`: `n` of them is `n · 1.0 / (core::f64::consts::PI * 0.3048 * 0.3048)` base units.
pub const FOOTLAMBERT: Unit<
    {
        SIUnit::scaled(
            "fl",
            Exact::int(1) / (Exact::PI * Exact::dec(3048, -4) * Exact::dec(3048, -4)),
            CANDELA_PER_METRE_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// Lamberts (`la`), a unit of `Luminance`: `n` of them is `n · 10000.0 / core::f64::consts::PI` base units.
pub const LAMBERT: Unit<
    {
        SIUnit::scaled(
            "la",
            Exact::int(10000) / Exact::PI,
            CANDELA_PER_METRE_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// Stilbs (`sb`), a unit of `Luminance`: `n` of them is `n · 10000.0` base units.
pub const STILB: Unit<
    { SIUnit::scaled("sb", Exact::int(10000), CANDELA_PER_METRE_SQUARED.unit()) },
> = Unit::ONE;

/// A luminous intensity, in `N`s of the unit `U`: by default its base unit, `CANDELA`.
pub type LuminousIntensity<N, const U: SIUnit = { CANDELA.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// An illuminance, in `N`s of the unit `U`: by default its base unit, `LUX`.
pub type Illuminance<N, const U: SIUnit = { LUX.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Lux (`lx`), a unit of `Illuminance`: the coherent unit, a product of base units.
pub const LUX: Unit<{ (CANDELA * RADIAN * RADIAN / (METRE * METRE)).unit() }, BASE> = Unit::ONE;

/// Phot (`ph`), a unit of `Illuminance`: `n` of them is `n · 10000.0` base units.
pub const PHOT: Unit<{ SIUnit::scaled("ph", Exact::int(10000), LUX.unit()) }> = Unit::ONE;

/// Footcandles (`fc`), a unit of `Illuminance`: `n` of them is `n · 1.0 / (0.3048 * 0.3048)` base units.
pub const FOOTCANDLE: Unit<
    {
        SIUnit::scaled(
            "fc",
            Exact::int(1) / (Exact::dec(3048, -4) * Exact::dec(3048, -4)),
            LUX.unit(),
        )
    },
> = Unit::ONE;

/// A radiant exposure, in `N`s of the unit `U`: by default its base unit, `KILOGRAM_PER_SECOND_SQUARED`.
pub type RadiantExposure<
    N,
    const U: SIUnit = { KILOGRAM_PER_SECOND_SQUARED.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `RadiantExposure`: a product of base units.
pub const KILOGRAM_PER_SECOND_SQUARED: Unit<{ (KILOGRAM / (SECOND * SECOND)).unit() }, BASE> =
    Unit::ONE;
