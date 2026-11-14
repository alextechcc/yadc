use crate::ISQ;
use yadc::Dimension;

/// The dimension of `Luminance`: `SIQuantity<LUMINANCE, N>` is one.
pub const LUMINANCE: ISQ = ISQ {
    m: -2,
    cd: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `LuminousIntensity`: `SIQuantity<LUMINOUS_INTENSITY, N>` is one.
pub const LUMINOUS_INTENSITY: ISQ = ISQ {
    cd: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `Illuminance`: `SIQuantity<ILLUMINANCE, N>` is one.
pub const ILLUMINANCE: ISQ = ISQ {
    m: -2,
    cd: 1,
    rad: 2,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `RadiantExposure`: `SIQuantity<RADIANT_EXPOSURE, N>` is one.
pub const RADIANT_EXPOSURE: ISQ = ISQ {
    kg: 1,
    s: -2,
    ..ISQ::DIMENSIONLESS
};

/// The units of this module's quantities: each a constant of its own unit
/// type, its size and zero in its quantity's coherent unit, exact
/// (`MILE.quantity(3.0)`, `MILE.value(d)`).
pub mod units {
    use super::*;
    use crate::{Exact, SIUnit, SIUnitConsts};

    /// Candelas per square metre (`cd/m²`), a unit of `Luminance`: `n` of them is `n · 1.0` coherent units.
    pub const CANDELA_PER_SQUARE_METRE: SIUnit<
        LUMINANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "cd/m²",
                name: "candelas per square metre",
            }
        },
    > = SIUnit;

    /// Candelas per square inch (`cd/in²`), a unit of `Luminance`: `n` of them is `n · 1.0 / (0.0254 * 0.0254)` coherent units.
    pub const CANDELA_PER_SQUARE_INCH: SIUnit<
        LUMINANCE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(254, -4) * Exact::dec(254, -4)),
                zero: Exact::ZERO,
                symbol: "cd/in²",
                name: "candelas per square inch",
            }
        },
    > = SIUnit;

    /// Candelas per square foot (`cd/ft²`), a unit of `Luminance`: `n` of them is `n · 1.0 / (0.3048 * 0.3048)` coherent units.
    pub const CANDELA_PER_SQUARE_FOOT: SIUnit<
        LUMINANCE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(3048, -4) * Exact::dec(3048, -4)),
                zero: Exact::ZERO,
                symbol: "cd/ft²",
                name: "candelas per square foot",
            }
        },
    > = SIUnit;

    /// Footlamberts (`fl`), a unit of `Luminance`: `n` of them is `n · 1.0 / (core::f64::consts::PI * 0.3048 * 0.3048)` coherent units.
    pub const FOOTLAMBERT: SIUnit<
        LUMINANCE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::PI * Exact::dec(3048, -4) * Exact::dec(3048, -4)),
                zero: Exact::ZERO,
                symbol: "fl",
                name: "footlamberts",
            }
        },
    > = SIUnit;

    /// Lamberts (`la`), a unit of `Luminance`: `n` of them is `n · 10000.0 / core::f64::consts::PI` coherent units.
    pub const LAMBERT: SIUnit<
        LUMINANCE,
        {
            SIUnitConsts {
                size: Exact::int(10000) / Exact::PI,
                zero: Exact::ZERO,
                symbol: "la",
                name: "lamberts",
            }
        },
    > = SIUnit;

    /// Stilbs (`sb`), a unit of `Luminance`: `n` of them is `n · 10000.0` coherent units.
    pub const STILB: SIUnit<
        LUMINANCE,
        {
            SIUnitConsts {
                size: Exact::int(10000),
                zero: Exact::ZERO,
                symbol: "sb",
                name: "stilbs",
            }
        },
    > = SIUnit;

    /// Candelas (`cd`), a unit of `LuminousIntensity`: `n` of them is `n · 1.0` coherent units.
    pub const CANDELA: SIUnit<
        LUMINOUS_INTENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "cd",
                name: "candelas",
            }
        },
    > = SIUnit;

    /// Lux (`lx`), a unit of `Illuminance`: `n` of them is `n · 1.0` coherent units.
    pub const LUX: SIUnit<
        ILLUMINANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "lx",
                name: "lux",
            }
        },
    > = SIUnit;

    /// Phot (`ph`), a unit of `Illuminance`: `n` of them is `n · 10000.0` coherent units.
    pub const PHOT: SIUnit<
        ILLUMINANCE,
        {
            SIUnitConsts {
                size: Exact::int(10000),
                zero: Exact::ZERO,
                symbol: "ph",
                name: "phot",
            }
        },
    > = SIUnit;

    /// Footcandles (`fc`), a unit of `Illuminance`: `n` of them is `n · 1.0 / (0.3048 * 0.3048)` coherent units.
    pub const FOOTCANDLE: SIUnit<
        ILLUMINANCE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(3048, -4) * Exact::dec(3048, -4)),
                zero: Exact::ZERO,
                symbol: "fc",
                name: "footcandles",
            }
        },
    > = SIUnit;

    /// Joules per square metre (`J/m²`), a unit of `RadiantExposure`: `n` of them is `n · 1.0` coherent units.
    pub const JOULE_PER_SQUARE_METRE: SIUnit<
        RADIANT_EXPOSURE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/m²",
                name: "joules per square metre",
            }
        },
    > = SIUnit;

    /// Watt hours per square metre (`W · h/m²`), a unit of `RadiantExposure`: `n` of them is `n · 3600.0` coherent units.
    pub const WATT_HOUR_PER_SQUARE_METRE: SIUnit<
        RADIANT_EXPOSURE,
        {
            SIUnitConsts {
                size: Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "W · h/m²",
                name: "watt hours per square metre",
            }
        },
    > = SIUnit;

    /// Microjoules per square millimetre (`µJ/mm²`), a unit of `RadiantExposure`: `n` of them is `n · 1.0` coherent units.
    pub const MICROJOULE_PER_SQUARE_MILLIMETRE: SIUnit<
        RADIANT_EXPOSURE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "µJ/mm²",
                name: "microjoules per square millimetre",
            }
        },
    > = SIUnit;
}
