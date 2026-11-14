use crate::{ISQ, SIQuantity};
use yadc::Dimension;

/// The dimension of `Luminance`.
pub const LUMINANCE: ISQ = ISQ {
    m: -2,
    cd: 1,
    ..ISQ::DIMENSIONLESS
};
pub type Luminance<N> = SIQuantity<LUMINANCE, N>;

/// The dimension of `LuminousIntensity`.
pub const LUMINOUS_INTENSITY: ISQ = ISQ {
    cd: 1,
    ..ISQ::DIMENSIONLESS
};
pub type LuminousIntensity<N> = SIQuantity<LUMINOUS_INTENSITY, N>;

/// The dimension of `Illuminance`.
pub const ILLUMINANCE: ISQ = ISQ {
    m: -2,
    cd: 1,
    rad: 2,
    ..ISQ::DIMENSIONLESS
};
pub type Illuminance<N> = SIQuantity<ILLUMINANCE, N>;

/// The dimension of `RadiantExposure`.
pub const RADIANT_EXPOSURE: ISQ = ISQ {
    kg: 1,
    s: -2,
    ..ISQ::DIMENSIONLESS
};
pub type RadiantExposure<N> = SIQuantity<RADIANT_EXPOSURE, N>;

/// The units of this module's quantities: a type each, its size and zero
/// in its quantity's coherent unit, exact (`Mile {}.quantity(3.0_f64)`).
pub mod units {
    use super::*;
    use crate::{Exact, SIUnit, SIUnitConsts};

    /// Candelas per square metre (`cd/m²`), a unit of `Luminance`: `n` of them is `n · 1.0` coherent units.
    pub type CandelaPerSquareMetre = SIUnit<
        LUMINANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "cd/m²",
                name: "candelas per square metre",
            }
        },
    >;

    /// Candelas per square inch (`cd/in²`), a unit of `Luminance`: `n` of them is `n · 1.0 / (0.0254 * 0.0254)` coherent units.
    pub type CandelaPerSquareInch = SIUnit<
        LUMINANCE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(254, -4) * Exact::dec(254, -4)),
                zero: Exact::ZERO,
                symbol: "cd/in²",
                name: "candelas per square inch",
            }
        },
    >;

    /// Candelas per square foot (`cd/ft²`), a unit of `Luminance`: `n` of them is `n · 1.0 / (0.3048 * 0.3048)` coherent units.
    pub type CandelaPerSquareFoot = SIUnit<
        LUMINANCE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(3048, -4) * Exact::dec(3048, -4)),
                zero: Exact::ZERO,
                symbol: "cd/ft²",
                name: "candelas per square foot",
            }
        },
    >;

    /// Footlamberts (`fl`), a unit of `Luminance`: `n` of them is `n · 1.0 / (core::f64::consts::PI * 0.3048 * 0.3048)` coherent units.
    pub type Footlambert = SIUnit<
        LUMINANCE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::PI * Exact::dec(3048, -4) * Exact::dec(3048, -4)),
                zero: Exact::ZERO,
                symbol: "fl",
                name: "footlamberts",
            }
        },
    >;

    /// Lamberts (`la`), a unit of `Luminance`: `n` of them is `n · 10000.0 / core::f64::consts::PI` coherent units.
    pub type Lambert = SIUnit<
        LUMINANCE,
        {
            SIUnitConsts {
                size: Exact::int(10000) / Exact::PI,
                zero: Exact::ZERO,
                symbol: "la",
                name: "lamberts",
            }
        },
    >;

    /// Stilbs (`sb`), a unit of `Luminance`: `n` of them is `n · 10000.0` coherent units.
    pub type Stilb = SIUnit<
        LUMINANCE,
        {
            SIUnitConsts {
                size: Exact::int(10000),
                zero: Exact::ZERO,
                symbol: "sb",
                name: "stilbs",
            }
        },
    >;

    /// Candelas (`cd`), a unit of `LuminousIntensity`: `n` of them is `n · 1.0` coherent units.
    pub type Candela = SIUnit<
        LUMINOUS_INTENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "cd",
                name: "candelas",
            }
        },
    >;

    /// Lux (`lx`), a unit of `Illuminance`: `n` of them is `n · 1.0` coherent units.
    pub type Lux = SIUnit<
        ILLUMINANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "lx",
                name: "lux",
            }
        },
    >;

    /// Phot (`ph`), a unit of `Illuminance`: `n` of them is `n · 10000.0` coherent units.
    pub type Phot = SIUnit<
        ILLUMINANCE,
        {
            SIUnitConsts {
                size: Exact::int(10000),
                zero: Exact::ZERO,
                symbol: "ph",
                name: "phot",
            }
        },
    >;

    /// Footcandles (`fc`), a unit of `Illuminance`: `n` of them is `n · 1.0 / (0.3048 * 0.3048)` coherent units.
    pub type Footcandle = SIUnit<
        ILLUMINANCE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(3048, -4) * Exact::dec(3048, -4)),
                zero: Exact::ZERO,
                symbol: "fc",
                name: "footcandles",
            }
        },
    >;

    /// Joules per square metre (`J/m²`), a unit of `RadiantExposure`: `n` of them is `n · 1.0` coherent units.
    pub type JoulePerSquareMetre = SIUnit<
        RADIANT_EXPOSURE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/m²",
                name: "joules per square metre",
            }
        },
    >;

    /// Watt hours per square metre (`W · h/m²`), a unit of `RadiantExposure`: `n` of them is `n · 3600.0` coherent units.
    pub type WattHourPerSquareMetre = SIUnit<
        RADIANT_EXPOSURE,
        {
            SIUnitConsts {
                size: Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "W · h/m²",
                name: "watt hours per square metre",
            }
        },
    >;

    /// Microjoules per square millimetre (`µJ/mm²`), a unit of `RadiantExposure`: `n` of them is `n · 1.0` coherent units.
    pub type MicrojoulePerSquareMillimetre = SIUnit<
        RADIANT_EXPOSURE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "µJ/mm²",
                name: "microjoules per square millimetre",
            }
        },
    >;
}
