use crate::ISQ;
use yadc::Dimension;

/// The dimension of `MassDensity`: `SIQuantity<MASS_DENSITY, N>` is one.
pub const MASS_DENSITY: ISQ = ISQ {
    m: -3,
    kg: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `LinearMassDensity`: `SIQuantity<LINEAR_MASS_DENSITY, N>` is one.
pub const LINEAR_MASS_DENSITY: ISQ = ISQ {
    m: -1,
    kg: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ArealMassDensity`: `SIQuantity<AREAL_MASS_DENSITY, N>` is one.
pub const AREAL_MASS_DENSITY: ISQ = ISQ {
    m: -2,
    kg: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `LinearNumberDensity`: `SIQuantity<LINEAR_NUMBER_DENSITY, N>` is one.
pub const LINEAR_NUMBER_DENSITY: ISQ = ISQ {
    m: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ArealNumberDensity`: `SIQuantity<AREAL_NUMBER_DENSITY, N>` is one.
pub const AREAL_NUMBER_DENSITY: ISQ = ISQ {
    m: -2,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `VolumetricNumberDensity`: `SIQuantity<VOLUMETRIC_NUMBER_DENSITY, N>` is one.
pub const VOLUMETRIC_NUMBER_DENSITY: ISQ = ISQ {
    m: -3,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `LinearNumberRate`: `SIQuantity<LINEAR_NUMBER_RATE, N>` is one.
pub const LINEAR_NUMBER_RATE: ISQ = ISQ {
    m: -1,
    s: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ArealNumberRate`: `SIQuantity<AREAL_NUMBER_RATE, N>` is one.
pub const AREAL_NUMBER_RATE: ISQ = ISQ {
    m: -2,
    s: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `VolumetricNumberRate`: `SIQuantity<VOLUMETRIC_NUMBER_RATE, N>` is one.
pub const VOLUMETRIC_NUMBER_RATE: ISQ = ISQ {
    m: -3,
    s: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `LinearDensityOfStates`: `SIQuantity<LINEAR_DENSITY_OF_STATES, N>` is one.
pub const LINEAR_DENSITY_OF_STATES: ISQ = ISQ {
    m: -3,
    kg: -1,
    s: 2,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ArealDensityOfStates`: `SIQuantity<AREAL_DENSITY_OF_STATES, N>` is one.
pub const AREAL_DENSITY_OF_STATES: ISQ = ISQ {
    m: -4,
    kg: -1,
    s: 2,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `VolumetricDensityOfStates`: `SIQuantity<VOLUMETRIC_DENSITY_OF_STATES, N>` is one.
pub const VOLUMETRIC_DENSITY_OF_STATES: ISQ = ISQ {
    m: -5,
    kg: -1,
    s: 2,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `LinearPowerDensity`: `SIQuantity<LINEAR_POWER_DENSITY, N>` is one.
pub const LINEAR_POWER_DENSITY: ISQ = ISQ {
    m: 1,
    kg: 1,
    s: -3,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `VolumetricPowerDensity`: `SIQuantity<VOLUMETRIC_POWER_DENSITY, N>` is one.
pub const VOLUMETRIC_POWER_DENSITY: ISQ = ISQ {
    m: -1,
    kg: 1,
    s: -3,
    ..ISQ::DIMENSIONLESS
};

/// The units of this module's quantities: each a constant of its own unit
/// type, its size and zero in its quantity's coherent unit, exact
/// (`MILE.quantity(3.0)`, `MILE.value(d)`).
pub mod units {
    use super::*;
    use crate::{Exact, SIUnit, SIUnitConsts};

    /// Kilograms per cubic metre (`kg/m³`), a unit of `MassDensity`: `n` of them is `n · 1.0` coherent units.
    pub const KILOGRAM_PER_CUBIC_METRE: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg/m³",
                name: "kilograms per cubic metre",
            }
        },
    > = SIUnit;

    /// Grams per cubic metre (`g/m³`), a unit of `MassDensity`: `n` of them is `n · 0.001` coherent units.
    pub const GRAM_PER_CUBIC_METRE: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "g/m³",
                name: "grams per cubic metre",
            }
        },
    > = SIUnit;

    /// Carats per cubic metre (`ct/m³`), a unit of `MassDensity`: `n` of them is `n · 0.0002` coherent units.
    pub const CARAT_PER_CUBIC_METRE: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(2, -4),
                zero: Exact::ZERO,
                symbol: "ct/m³",
                name: "carats per cubic metre",
            }
        },
    > = SIUnit;

    /// Grains per cubic metre (`gr/m³`), a unit of `MassDensity`: `n` of them is `n · 6.479891e-05` coherent units.
    pub const GRAIN_PER_CUBIC_METRE: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(6479891, -11),
                zero: Exact::ZERO,
                symbol: "gr/m³",
                name: "grains per cubic metre",
            }
        },
    > = SIUnit;

    /// Hundredweight (long) per cubic metre (`cwt long/m³`), a unit of `MassDensity`: `n` of them is `n · 50.80234544` coherent units.
    pub const HUNDREDWEIGHT_LONG_PER_CUBIC_METRE: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(5080234544, -8),
                zero: Exact::ZERO,
                symbol: "cwt long/m³",
                name: "hundredweight (long) per cubic metre",
            }
        },
    > = SIUnit;

    /// Hundredweight (short) per cubic metre (`cwt short/m³`), a unit of `MassDensity`: `n` of them is `n · 45.359237` coherent units.
    pub const HUNDREDWEIGHT_SHORT_PER_CUBIC_METRE: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -6),
                zero: Exact::ZERO,
                symbol: "cwt short/m³",
                name: "hundredweight (short) per cubic metre",
            }
        },
    > = SIUnit;

    /// Ounces per cubic metre (`oz/m³`), a unit of `MassDensity`: `n` of them is `n · 0.028349523125` coherent units.
    pub const OUNCE_PER_CUBIC_METRE: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(28349523125, -12),
                zero: Exact::ZERO,
                symbol: "oz/m³",
                name: "ounces per cubic metre",
            }
        },
    > = SIUnit;

    /// Troy ounces per cubic metre (`oz t/m³`), a unit of `MassDensity`: `n` of them is `n · 0.0311034768` coherent units.
    pub const OUNCE_TROY_PER_CUBIC_METRE: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(311034768, -10),
                zero: Exact::ZERO,
                symbol: "oz t/m³",
                name: "troy ounces per cubic metre",
            }
        },
    > = SIUnit;

    /// Pennyweight per cubic metre (`dwt/m³`), a unit of `MassDensity`: `n` of them is `n · 0.00155517384` coherent units.
    pub const PENNYWEIGHT_PER_CUBIC_METRE: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(155517384, -11),
                zero: Exact::ZERO,
                symbol: "dwt/m³",
                name: "pennyweight per cubic metre",
            }
        },
    > = SIUnit;

    /// Pounds per cubic metre (`lb/m³`), a unit of `MassDensity`: `n` of them is `n · 0.45359237` coherent units.
    pub const POUND_PER_CUBIC_METRE: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "lb/m³",
                name: "pounds per cubic metre",
            }
        },
    > = SIUnit;

    /// Troy pounds per cubic metre (`lb t/m³`), a unit of `MassDensity`: `n` of them is `n · 0.3732417216` coherent units.
    pub const POUND_TROY_PER_CUBIC_METRE: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(3732417216, -10),
                zero: Exact::ZERO,
                symbol: "lb t/m³",
                name: "troy pounds per cubic metre",
            }
        },
    > = SIUnit;

    /// Slugs per cubic metre (`slug/m³`), a unit of `MassDensity`: `n` of them is `n · 0.45359237 * 9.80665 / 0.3048` coherent units.
    pub const SLUG_PER_CUBIC_METRE: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "slug/m³",
                name: "slugs per cubic metre",
            }
        },
    > = SIUnit;

    /// Assay tons per cubic metre (`AT/m³`), a unit of `MassDensity`: `n` of them is `n · 0.175 / 6.0` coherent units.
    pub const TON_ASSAY_PER_CUBIC_METRE: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(175, -3) / Exact::int(6),
                zero: Exact::ZERO,
                symbol: "AT/m³",
                name: "assay tons per cubic metre",
            }
        },
    > = SIUnit;

    /// Long tons per cubic metre (`2240 lb/m³`), a unit of `MassDensity`: `n` of them is `n · 1016.0469088` coherent units.
    pub const TON_LONG_PER_CUBIC_METRE: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(10160469088, -7),
                zero: Exact::ZERO,
                symbol: "2240 lb/m³",
                name: "long tons per cubic metre",
            }
        },
    > = SIUnit;

    /// Short tons per cubic metre (`2000 lb/m³`), a unit of `MassDensity`: `n` of them is `n · 907.18474` coherent units.
    pub const TON_SHORT_PER_CUBIC_METRE: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(90718474, -5),
                zero: Exact::ZERO,
                symbol: "2000 lb/m³",
                name: "short tons per cubic metre",
            }
        },
    > = SIUnit;

    /// Tons per cubic metre (`t/m³`), a unit of `MassDensity`: `n` of them is `n · 1000.0` coherent units.
    pub const TON_PER_CUBIC_METRE: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "t/m³",
                name: "tons per cubic metre",
            }
        },
    > = SIUnit;

    /// Grains per gallon (`gr/gal`), a unit of `MassDensity`: `n` of them is `n · 6.479891e-05 / 0.003785411784` coherent units.
    pub const GRAIN_PER_GALLON: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(6479891, -11) / Exact::dec(3785411784, -12),
                zero: Exact::ZERO,
                symbol: "gr/gal",
                name: "grains per gallon",
            }
        },
    > = SIUnit;

    /// Ounces per cubic inch (`oz/in³`), a unit of `MassDensity`: `n` of them is `n · 0.028349523125 / (0.0254 * 0.0254 * 0.0254)` coherent units.
    pub const OUNCE_PER_CUBIC_INCH: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(28349523125, -12)
                    / (Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4)),
                zero: Exact::ZERO,
                symbol: "oz/in³",
                name: "ounces per cubic inch",
            }
        },
    > = SIUnit;

    /// Ounces per Imperial gallon (`oz/gal (UK)`), a unit of `MassDensity`: `n` of them is `n · 0.028349523125 / 0.00454609` coherent units.
    pub const OUNCE_PER_GALLON_IMPERIAL: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(28349523125, -12) / Exact::dec(454609, -8),
                zero: Exact::ZERO,
                symbol: "oz/gal (UK)",
                name: "ounces per Imperial gallon",
            }
        },
    > = SIUnit;

    /// Ounces per gallon (`oz/gal`), a unit of `MassDensity`: `n` of them is `n · 0.028349523125 / 0.003785411784` coherent units.
    pub const OUNCE_PER_GALLON: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(28349523125, -12) / Exact::dec(3785411784, -12),
                zero: Exact::ZERO,
                symbol: "oz/gal",
                name: "ounces per gallon",
            }
        },
    > = SIUnit;

    /// Pounds per cubic foot (`lb/ft³`), a unit of `MassDensity`: `n` of them is `n · 0.45359237 / (0.3048 * 0.3048 * 0.3048)` coherent units.
    pub const POUND_PER_CUBIC_FOOT: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8)
                    / (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)),
                zero: Exact::ZERO,
                symbol: "lb/ft³",
                name: "pounds per cubic foot",
            }
        },
    > = SIUnit;

    /// Pounds per cubic inch (`lb/in³`), a unit of `MassDensity`: `n` of them is `n · 0.45359237 / (0.0254 * 0.0254 * 0.0254)` coherent units.
    pub const POUND_PER_CUBIC_INCH: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8)
                    / (Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4)),
                zero: Exact::ZERO,
                symbol: "lb/in³",
                name: "pounds per cubic inch",
            }
        },
    > = SIUnit;

    /// Pounds per cubic yard (`lb/yd³`), a unit of `MassDensity`: `n` of them is `n · 0.45359237 / (0.9144 * 0.9144 * 0.9144)` coherent units.
    pub const POUND_PER_CUBIC_YARD: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8)
                    / (Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4)),
                zero: Exact::ZERO,
                symbol: "lb/yd³",
                name: "pounds per cubic yard",
            }
        },
    > = SIUnit;

    /// Pounds per Imperial gallon (`lb/gal (UK)`), a unit of `MassDensity`: `n` of them is `n · 0.45359237 / 0.00454609` coherent units.
    pub const POUND_PER_GALLON_IMPERIAL: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) / Exact::dec(454609, -8),
                zero: Exact::ZERO,
                symbol: "lb/gal (UK)",
                name: "pounds per Imperial gallon",
            }
        },
    > = SIUnit;

    /// Pounds per gallon (`lb/gal`), a unit of `MassDensity`: `n` of them is `n · 0.45359237 / 0.003785411784` coherent units.
    pub const POUND_PER_GALLON: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) / Exact::dec(3785411784, -12),
                zero: Exact::ZERO,
                symbol: "lb/gal",
                name: "pounds per gallon",
            }
        },
    > = SIUnit;

    /// Slugs per cubic foot (`slug/ft³`), a unit of `MassDensity`: `n` of them is `n · (0.45359237 * 9.80665 / 0.3048) / (0.3048 * 0.3048 * 0.3048)` coherent units.
    pub const SLUG_PER_CUBIC_FOOT: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: (Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4))
                    / (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)),
                zero: Exact::ZERO,
                symbol: "slug/ft³",
                name: "slugs per cubic foot",
            }
        },
    > = SIUnit;

    /// Long tons per cubic yard (`2240 lb/yd³`), a unit of `MassDensity`: `n` of them is `n · 1016.0469088 / (0.9144 * 0.9144 * 0.9144)` coherent units.
    pub const TON_LONG_PER_CUBIC_YARD: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(10160469088, -7)
                    / (Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4)),
                zero: Exact::ZERO,
                symbol: "2240 lb/yd³",
                name: "long tons per cubic yard",
            }
        },
    > = SIUnit;

    /// Short tons per cubic yard (`2000 lb/yd³`), a unit of `MassDensity`: `n` of them is `n · 907.18474 / (0.9144 * 0.9144 * 0.9144)` coherent units.
    pub const TON_SHORT_PER_CUBIC_YARD: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(90718474, -5)
                    / (Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4)),
                zero: Exact::ZERO,
                symbol: "2000 lb/yd³",
                name: "short tons per cubic yard",
            }
        },
    > = SIUnit;

    /// Long tons per cubic foot (`2240 lb/ft³`), a unit of `MassDensity`: `n` of them is `n · 1016.0469088 / (0.3048 * 0.3048 * 0.3048)` coherent units.
    pub const TON_LONG_PER_CUBIC_FOOT: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(10160469088, -7)
                    / (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)),
                zero: Exact::ZERO,
                symbol: "2240 lb/ft³",
                name: "long tons per cubic foot",
            }
        },
    > = SIUnit;

    /// Short tons per cubic foot (`2000 lb/ft³`), a unit of `MassDensity`: `n` of them is `n · 907.18474 / (0.3048 * 0.3048 * 0.3048)` coherent units.
    pub const TON_SHORT_PER_CUBIC_FOOT: SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(90718474, -5)
                    / (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)),
                zero: Exact::ZERO,
                symbol: "2000 lb/ft³",
                name: "short tons per cubic foot",
            }
        },
    > = SIUnit;

    /// Kilograms per metre (`kg/m`), a unit of `LinearMassDensity`: `n` of them is `n · 1.0` coherent units.
    pub const KILOGRAM_PER_METRE: SIUnit<
        LINEAR_MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg/m",
                name: "kilograms per metre",
            }
        },
    > = SIUnit;

    /// Ounces per foot (`oz/ft`), a unit of `LinearMassDensity`: `n` of them is `n · 0.028349523125 / 0.3048` coherent units.
    pub const OUNCE_PER_FOOT: SIUnit<
        LINEAR_MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(28349523125, -12) / Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "oz/ft",
                name: "ounces per foot",
            }
        },
    > = SIUnit;

    /// Ounces per inch (`oz/in`), a unit of `LinearMassDensity`: `n` of them is `n · 0.028349523125 / 0.0254` coherent units.
    pub const OUNCE_PER_INCH: SIUnit<
        LINEAR_MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(28349523125, -12) / Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "oz/in",
                name: "ounces per inch",
            }
        },
    > = SIUnit;

    /// Pounds per yard (`lb/yd`), a unit of `LinearMassDensity`: `n` of them is `n · 0.45359237 / 0.9144` coherent units.
    pub const POUND_PER_YARD: SIUnit<
        LINEAR_MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) / Exact::dec(9144, -4),
                zero: Exact::ZERO,
                symbol: "lb/yd",
                name: "pounds per yard",
            }
        },
    > = SIUnit;

    /// Pounds per foot (`lb/ft`), a unit of `LinearMassDensity`: `n` of them is `n · 0.45359237 / 0.3048` coherent units.
    pub const POUND_PER_FOOT: SIUnit<
        LINEAR_MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) / Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "lb/ft",
                name: "pounds per foot",
            }
        },
    > = SIUnit;

    /// Pounds per inch (`lb/in`), a unit of `LinearMassDensity`: `n` of them is `n · 0.45359237 / 0.0254` coherent units.
    pub const POUND_PER_INCH: SIUnit<
        LINEAR_MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) / Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "lb/in",
                name: "pounds per inch",
            }
        },
    > = SIUnit;

    /// Grams per kilometre (`g/km`), a unit of `LinearMassDensity`: `n` of them is `n · 1e-06` coherent units.
    pub const GRAM_PER_KILOMETRE: SIUnit<
        LINEAR_MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -6),
                zero: Exact::ZERO,
                symbol: "g/km",
                name: "grams per kilometre",
            }
        },
    > = SIUnit;

    /// Grams per centimetre (`g/cm`), a unit of `LinearMassDensity`: `n` of them is `n · 0.1` coherent units.
    pub const GRAM_PER_CENTIMETRE: SIUnit<
        LINEAR_MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -1),
                zero: Exact::ZERO,
                symbol: "g/cm",
                name: "grams per centimetre",
            }
        },
    > = SIUnit;

    /// Kilograms per square metre (`kg/m²`), a unit of `ArealMassDensity`: `n` of them is `n · 1.0` coherent units.
    pub const KILOGRAM_PER_SQUARE_METRE: SIUnit<
        AREAL_MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg/m²",
                name: "kilograms per square metre",
            }
        },
    > = SIUnit;

    /// Grams per square metre (`g/m²`), a unit of `ArealMassDensity`: `n` of them is `n · 0.001` coherent units.
    pub const GRAM_PER_SQUARE_METRE: SIUnit<
        AREAL_MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "g/m²",
                name: "grams per square metre",
            }
        },
    > = SIUnit;

    /// Ounces per square foot (`oz/ft²`), a unit of `ArealMassDensity`: `n` of them is `n · 0.028349523125 / (0.3048 * 0.3048)` coherent units.
    pub const OUNCE_PER_SQUARE_FOOT: SIUnit<
        AREAL_MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(28349523125, -12) / (Exact::dec(3048, -4) * Exact::dec(3048, -4)),
                zero: Exact::ZERO,
                symbol: "oz/ft²",
                name: "ounces per square foot",
            }
        },
    > = SIUnit;

    /// Per metre (`m⁻¹`), a unit of `LinearNumberDensity`: `n` of them is `n · 1.0` coherent units.
    pub const PER_METRE: SIUnit<
        LINEAR_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m⁻¹",
                name: "per metre",
            }
        },
    > = SIUnit;

    /// Per foot (`ft⁻¹`), a unit of `LinearNumberDensity`: `n` of them is `n · 1.0 / 0.3048` coherent units.
    pub const PER_FOOT: SIUnit<
        LINEAR_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "ft⁻¹",
                name: "per foot",
            }
        },
    > = SIUnit;

    /// Foot (U.S. survey) (`ft (U.S. survey)`), a unit of `LinearNumberDensity`: `n` of them is `n · 1.0 / (1200.0 / 3937.0)` coherent units.
    pub const PER_FOOT_SURVEY: SIUnit<
        LINEAR_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::int(1200) / Exact::int(3937)),
                zero: Exact::ZERO,
                symbol: "ft (U.S. survey)",
                name: "foot (U.S. survey)",
            }
        },
    > = SIUnit;

    /// Per inch (`in⁻¹`), a unit of `LinearNumberDensity`: `n` of them is `n · 1.0 / 0.0254` coherent units.
    pub const PER_INCH: SIUnit<
        LINEAR_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "in⁻¹",
                name: "per inch",
            }
        },
    > = SIUnit;

    /// Per mile (`mi⁻¹`), a unit of `LinearNumberDensity`: `n` of them is `n · 1.0 / 1609.344` coherent units.
    pub const PER_MILE: SIUnit<
        LINEAR_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(1609344, -3),
                zero: Exact::ZERO,
                symbol: "mi⁻¹",
                name: "per mile",
            }
        },
    > = SIUnit;

    /// Per mile (U.S. survey) (`mi⁻¹ (U.S. survey)`), a unit of `LinearNumberDensity`: `n` of them is `n · 1.0 / (6336000.0 / 3937.0)` coherent units.
    pub const PER_MILE_SURVEY: SIUnit<
        LINEAR_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::int(6336000) / Exact::int(3937)),
                zero: Exact::ZERO,
                symbol: "mi⁻¹ (U.S. survey)",
                name: "per mile (U.S. survey)",
            }
        },
    > = SIUnit;

    /// Per nautical mile (`M⁻¹`), a unit of `LinearNumberDensity`: `n` of them is `n · 1.0 / 1852.0` coherent units.
    pub const PER_NAUTICAL_MILE: SIUnit<
        LINEAR_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(1852),
                zero: Exact::ZERO,
                symbol: "M⁻¹",
                name: "per nautical mile",
            }
        },
    > = SIUnit;

    /// Per yard (`yd⁻¹`), a unit of `LinearNumberDensity`: `n` of them is `n · 1.0 / 0.9144` coherent units.
    pub const PER_YARD: SIUnit<
        LINEAR_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(9144, -4),
                zero: Exact::ZERO,
                symbol: "yd⁻¹",
                name: "per yard",
            }
        },
    > = SIUnit;

    /// Per square metre (`m⁻²`), a unit of `ArealNumberDensity`: `n` of them is `n · 1.0` coherent units.
    pub const PER_SQUARE_METRE: SIUnit<
        AREAL_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m⁻²",
                name: "per square metre",
            }
        },
    > = SIUnit;

    /// Per acre (`ac⁻²`), a unit of `ArealNumberDensity`: `n` of them is `n · 1.0 / (43560.0 * (1200.0 / 3937.0) * (1200.0 / 3937.0))` coherent units.
    pub const PER_ACRE: SIUnit<
        AREAL_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1)
                    / (Exact::int(43560)
                        * (Exact::int(1200) / Exact::int(3937))
                        * (Exact::int(1200) / Exact::int(3937))),
                zero: Exact::ZERO,
                symbol: "ac⁻²",
                name: "per acre",
            }
        },
    > = SIUnit;

    /// Per are (`a⁻²`), a unit of `ArealNumberDensity`: `n` of them is `n · 0.01` coherent units.
    pub const PER_ARE: SIUnit<
        AREAL_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -2),
                zero: Exact::ZERO,
                symbol: "a⁻²",
                name: "per are",
            }
        },
    > = SIUnit;

    /// Per barn (`b⁻²`), a unit of `ArealNumberDensity`: `n` of them is `n · 1e+28` coherent units.
    pub const PER_BARN: SIUnit<
        AREAL_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(10000000000000000000000000000),
                zero: Exact::ZERO,
                symbol: "b⁻²",
                name: "per barn",
            }
        },
    > = SIUnit;

    /// Per circular mil (`cmil⁻²`), a unit of `ArealNumberDensity`: `n` of them is `n · 4.0 / (core::f64::consts::PI * 2.54e-05 * 2.54e-05)` coherent units.
    pub const PER_CIRCULAR_MIL: SIUnit<
        AREAL_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(4) / (Exact::PI * Exact::dec(254, -7) * Exact::dec(254, -7)),
                zero: Exact::ZERO,
                symbol: "cmil⁻²",
                name: "per circular mil",
            }
        },
    > = SIUnit;

    /// Per hectare (`ha⁻²`), a unit of `ArealNumberDensity`: `n` of them is `n · 0.0001` coherent units.
    pub const PER_HECTARE: SIUnit<
        AREAL_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -4),
                zero: Exact::ZERO,
                symbol: "ha⁻²",
                name: "per hectare",
            }
        },
    > = SIUnit;

    /// Per square foot (`ft⁻²`), a unit of `ArealNumberDensity`: `n` of them is `n · 1.0 / (0.3048 * 0.3048)` coherent units.
    pub const PER_SQUARE_FOOT: SIUnit<
        AREAL_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(3048, -4) * Exact::dec(3048, -4)),
                zero: Exact::ZERO,
                symbol: "ft⁻²",
                name: "per square foot",
            }
        },
    > = SIUnit;

    /// Per square inch (`in⁻²`), a unit of `ArealNumberDensity`: `n` of them is `n · 1.0 / (0.0254 * 0.0254)` coherent units.
    pub const PER_SQUARE_INCH: SIUnit<
        AREAL_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(254, -4) * Exact::dec(254, -4)),
                zero: Exact::ZERO,
                symbol: "in⁻²",
                name: "per square inch",
            }
        },
    > = SIUnit;

    /// Per square mile (`mi⁻²`), a unit of `ArealNumberDensity`: `n` of them is `n · 1.0 / (1609.344 * 1609.344)` coherent units.
    pub const PER_SQUARE_MILE: SIUnit<
        AREAL_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(1609344, -3) * Exact::dec(1609344, -3)),
                zero: Exact::ZERO,
                symbol: "mi⁻²",
                name: "per square mile",
            }
        },
    > = SIUnit;

    /// Per square yard (`yd⁻²`), a unit of `ArealNumberDensity`: `n` of them is `n · 1.0 / (0.9144 * 0.9144)` coherent units.
    pub const PER_SQUARE_YARD: SIUnit<
        AREAL_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(9144, -4) * Exact::dec(9144, -4)),
                zero: Exact::ZERO,
                symbol: "yd⁻²",
                name: "per square yard",
            }
        },
    > = SIUnit;

    /// Per cubic metre (`m⁻³`), a unit of `VolumetricNumberDensity`: `n` of them is `n · 1.0` coherent units.
    pub const PER_CUBIC_METRE: SIUnit<
        VOLUMETRIC_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m⁻³",
                name: "per cubic metre",
            }
        },
    > = SIUnit;

    /// Per cubic foot (`ft⁻³`), a unit of `VolumetricNumberDensity`: `n` of them is `n · 1.0 / (0.3048 * 0.3048 * 0.3048)` coherent units.
    pub const PER_CUBIC_FOOT: SIUnit<
        VOLUMETRIC_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1)
                    / (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)),
                zero: Exact::ZERO,
                symbol: "ft⁻³",
                name: "per cubic foot",
            }
        },
    > = SIUnit;

    /// Per cubic inch (`in⁻³`), a unit of `VolumetricNumberDensity`: `n` of them is `n · 1.0 / (0.0254 * 0.0254 * 0.0254)` coherent units.
    pub const PER_CUBIC_INCH: SIUnit<
        VOLUMETRIC_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1)
                    / (Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4)),
                zero: Exact::ZERO,
                symbol: "in⁻³",
                name: "per cubic inch",
            }
        },
    > = SIUnit;

    /// Per cubic mile (`mi⁻³`), a unit of `VolumetricNumberDensity`: `n` of them is `n · 1.0 / (1609.344 * 1609.344 * 1609.344)` coherent units.
    pub const PER_CUBIC_MILE: SIUnit<
        VOLUMETRIC_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1)
                    / (Exact::dec(1609344, -3) * Exact::dec(1609344, -3) * Exact::dec(1609344, -3)),
                zero: Exact::ZERO,
                symbol: "mi⁻³",
                name: "per cubic mile",
            }
        },
    > = SIUnit;

    /// Per cubic yard (`yd⁻³`), a unit of `VolumetricNumberDensity`: `n` of them is `n · 1.0 / (0.9144 * 0.9144 * 0.9144)` coherent units.
    pub const PER_CUBIC_YARD: SIUnit<
        VOLUMETRIC_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1)
                    / (Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4)),
                zero: Exact::ZERO,
                symbol: "yd⁻³",
                name: "per cubic yard",
            }
        },
    > = SIUnit;

    /// Per fluid ounce (`per fl oz`), a unit of `VolumetricNumberDensity`: `n` of them is `n · 1.0 / 2.95735295625e-05` coherent units.
    pub const PER_FLUID_OUNCE: SIUnit<
        VOLUMETRIC_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(295735295625, -16),
                zero: Exact::ZERO,
                symbol: "per fl oz",
                name: "per fluid ounce",
            }
        },
    > = SIUnit;

    /// Per Imperial fluid ounce (`per fl oz (UK)`), a unit of `VolumetricNumberDensity`: `n` of them is `n · 1.0 / 2.84130625e-05` coherent units.
    pub const PER_FLUID_OUNCE_IMPERIAL: SIUnit<
        VOLUMETRIC_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(284130625, -13),
                zero: Exact::ZERO,
                symbol: "per fl oz (UK)",
                name: "per Imperial fluid ounce",
            }
        },
    > = SIUnit;

    /// Per Imperial gallon (`per gal (UK)`), a unit of `VolumetricNumberDensity`: `n` of them is `n · 1.0 / 0.00454609` coherent units.
    pub const PER_GALLON_IMPERIAL: SIUnit<
        VOLUMETRIC_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(454609, -8),
                zero: Exact::ZERO,
                symbol: "per gal (UK)",
                name: "per Imperial gallon",
            }
        },
    > = SIUnit;

    /// Per gallon (`per gal`), a unit of `VolumetricNumberDensity`: `n` of them is `n · 1.0 / 0.003785411784` coherent units.
    pub const PER_GALLON: SIUnit<
        VOLUMETRIC_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(3785411784, -12),
                zero: Exact::ZERO,
                symbol: "per gal",
                name: "per gallon",
            }
        },
    > = SIUnit;

    /// Per litre (`L⁻¹`), a unit of `VolumetricNumberDensity`: `n` of them is `n · 1000.0` coherent units.
    pub const PER_LITRE: SIUnit<
        VOLUMETRIC_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "L⁻¹",
                name: "per litre",
            }
        },
    > = SIUnit;

    /// Per metre second (`m⁻¹ · s⁻¹`), a unit of `LinearNumberRate`: `n` of them is `n · 1.0` coherent units.
    pub const PER_METRE_SECOND: SIUnit<
        LINEAR_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m⁻¹ · s⁻¹",
                name: "per metre second",
            }
        },
    > = SIUnit;

    /// Per foot second (`ft⁻¹ · s⁻¹`), a unit of `LinearNumberRate`: `n` of them is `n · 1.0 / 0.3048` coherent units.
    pub const PER_FOOT_SECOND: SIUnit<
        LINEAR_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "ft⁻¹ · s⁻¹",
                name: "per foot second",
            }
        },
    > = SIUnit;

    /// Per foot (U.S. survey) second (`ft⁻¹ (U.S. survey) · s⁻¹`), a unit of `LinearNumberRate`: `n` of them is `n · 1.0 / (1200.0 / 3937.0)` coherent units.
    pub const PER_FOOT_SURVEY_SECOND: SIUnit<
        LINEAR_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::int(1200) / Exact::int(3937)),
                zero: Exact::ZERO,
                symbol: "ft⁻¹ (U.S. survey) · s⁻¹",
                name: "per foot (U.S. survey) second",
            }
        },
    > = SIUnit;

    /// Per inch second (`in⁻¹ · s⁻¹`), a unit of `LinearNumberRate`: `n` of them is `n · 1.0 / 0.0254` coherent units.
    pub const PER_INCH_SECOND: SIUnit<
        LINEAR_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "in⁻¹ · s⁻¹",
                name: "per inch second",
            }
        },
    > = SIUnit;

    /// Per mile second (`mi⁻¹ · s⁻¹`), a unit of `LinearNumberRate`: `n` of them is `n · 1.0 / 1609.344` coherent units.
    pub const PER_MILE_SECOND: SIUnit<
        LINEAR_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(1609344, -3),
                zero: Exact::ZERO,
                symbol: "mi⁻¹ · s⁻¹",
                name: "per mile second",
            }
        },
    > = SIUnit;

    /// Per mile (U.S. survey) second (`mi⁻¹ (U.S. survey) · s⁻¹`), a unit of `LinearNumberRate`: `n` of them is `n · 1.0 / (6336000.0 / 3937.0)` coherent units.
    pub const PER_MILE_SURVEY_SECOND: SIUnit<
        LINEAR_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::int(6336000) / Exact::int(3937)),
                zero: Exact::ZERO,
                symbol: "mi⁻¹ (U.S. survey) · s⁻¹",
                name: "per mile (U.S. survey) second",
            }
        },
    > = SIUnit;

    /// Per yard second (`yd⁻¹ · s⁻¹`), a unit of `LinearNumberRate`: `n` of them is `n · 1.0 / 0.9144` coherent units.
    pub const PER_YARD_SECOND: SIUnit<
        LINEAR_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(9144, -4),
                zero: Exact::ZERO,
                symbol: "yd⁻¹ · s⁻¹",
                name: "per yard second",
            }
        },
    > = SIUnit;

    /// Per square metre second (`m⁻² · s⁻¹`), a unit of `ArealNumberRate`: `n` of them is `n · 1.0` coherent units.
    pub const PER_SQUARE_METRE_SECOND: SIUnit<
        AREAL_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m⁻² · s⁻¹",
                name: "per square metre second",
            }
        },
    > = SIUnit;

    /// Per acre second (`ac⁻¹ · s⁻¹`), a unit of `ArealNumberRate`: `n` of them is `n · 1.0 / (43560.0 * (1200.0 / 3937.0) * (1200.0 / 3937.0))` coherent units.
    pub const PER_ACRE_SECOND: SIUnit<
        AREAL_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1)
                    / (Exact::int(43560)
                        * (Exact::int(1200) / Exact::int(3937))
                        * (Exact::int(1200) / Exact::int(3937))),
                zero: Exact::ZERO,
                symbol: "ac⁻¹ · s⁻¹",
                name: "per acre second",
            }
        },
    > = SIUnit;

    /// Per are second (`a⁻¹ · s⁻¹`), a unit of `ArealNumberRate`: `n` of them is `n · 0.01` coherent units.
    pub const PER_ARE_SECOND: SIUnit<
        AREAL_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -2),
                zero: Exact::ZERO,
                symbol: "a⁻¹ · s⁻¹",
                name: "per are second",
            }
        },
    > = SIUnit;

    /// Per barn second (`b⁻¹ · s⁻¹`), a unit of `ArealNumberRate`: `n` of them is `n · 1e+28` coherent units.
    pub const PER_BARN_SECOND: SIUnit<
        AREAL_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(10000000000000000000000000000),
                zero: Exact::ZERO,
                symbol: "b⁻¹ · s⁻¹",
                name: "per barn second",
            }
        },
    > = SIUnit;

    /// Per circular mil second (`cmil⁻¹ · s⁻¹`), a unit of `ArealNumberRate`: `n` of them is `n · 4.0 / (core::f64::consts::PI * 2.54e-05 * 2.54e-05)` coherent units.
    pub const PER_CIRCULAR_MIL_SECOND: SIUnit<
        AREAL_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(4) / (Exact::PI * Exact::dec(254, -7) * Exact::dec(254, -7)),
                zero: Exact::ZERO,
                symbol: "cmil⁻¹ · s⁻¹",
                name: "per circular mil second",
            }
        },
    > = SIUnit;

    /// Per hectare second (`ha⁻¹ · s⁻¹`), a unit of `ArealNumberRate`: `n` of them is `n · 0.0001` coherent units.
    pub const PER_HECTARE_SECOND: SIUnit<
        AREAL_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -4),
                zero: Exact::ZERO,
                symbol: "ha⁻¹ · s⁻¹",
                name: "per hectare second",
            }
        },
    > = SIUnit;

    /// Per square foot second (`ft⁻² · s⁻¹`), a unit of `ArealNumberRate`: `n` of them is `n · 1.0 / (0.3048 * 0.3048)` coherent units.
    pub const PER_SQUARE_FOOT_SECOND: SIUnit<
        AREAL_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(3048, -4) * Exact::dec(3048, -4)),
                zero: Exact::ZERO,
                symbol: "ft⁻² · s⁻¹",
                name: "per square foot second",
            }
        },
    > = SIUnit;

    /// Per square inch second (`in⁻² · s⁻¹`), a unit of `ArealNumberRate`: `n` of them is `n · 1.0 / (0.0254 * 0.0254)` coherent units.
    pub const PER_SQUARE_INCH_SECOND: SIUnit<
        AREAL_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(254, -4) * Exact::dec(254, -4)),
                zero: Exact::ZERO,
                symbol: "in⁻² · s⁻¹",
                name: "per square inch second",
            }
        },
    > = SIUnit;

    /// Per square mile second (`mi⁻² · s⁻¹`), a unit of `ArealNumberRate`: `n` of them is `n · 1.0 / (1609.344 * 1609.344)` coherent units.
    pub const PER_SQUARE_MILE_SECOND: SIUnit<
        AREAL_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(1609344, -3) * Exact::dec(1609344, -3)),
                zero: Exact::ZERO,
                symbol: "mi⁻² · s⁻¹",
                name: "per square mile second",
            }
        },
    > = SIUnit;

    /// Per square yard second (`yd⁻² · s⁻¹`), a unit of `ArealNumberRate`: `n` of them is `n · 1.0 / (0.9144 * 0.9144)` coherent units.
    pub const PER_SQUARE_YARD_SECOND: SIUnit<
        AREAL_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(9144, -4) * Exact::dec(9144, -4)),
                zero: Exact::ZERO,
                symbol: "yd⁻² · s⁻¹",
                name: "per square yard second",
            }
        },
    > = SIUnit;

    /// Per cubic metre second (`m⁻³ · s⁻¹`), a unit of `VolumetricNumberRate`: `n` of them is `n · 1.0` coherent units.
    pub const PER_CUBIC_METRE_SECOND: SIUnit<
        VOLUMETRIC_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m⁻³ · s⁻¹",
                name: "per cubic metre second",
            }
        },
    > = SIUnit;

    /// Per cubic foot second (`ft⁻³ · s⁻¹`), a unit of `VolumetricNumberRate`: `n` of them is `n · 1.0 / (0.3048 * 0.3048 * 0.3048)` coherent units.
    pub const PER_CUBIC_FOOT_SECOND: SIUnit<
        VOLUMETRIC_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1)
                    / (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)),
                zero: Exact::ZERO,
                symbol: "ft⁻³ · s⁻¹",
                name: "per cubic foot second",
            }
        },
    > = SIUnit;

    /// Per cubic inch second (`in⁻³ · s⁻¹`), a unit of `VolumetricNumberRate`: `n` of them is `n · 1.0 / (0.0254 * 0.0254 * 0.0254)` coherent units.
    pub const PER_CUBIC_INCH_SECOND: SIUnit<
        VOLUMETRIC_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1)
                    / (Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4)),
                zero: Exact::ZERO,
                symbol: "in⁻³ · s⁻¹",
                name: "per cubic inch second",
            }
        },
    > = SIUnit;

    /// Per cubic mile second (`mi⁻³ · s⁻¹`), a unit of `VolumetricNumberRate`: `n` of them is `n · 1.0 / (1609.344 * 1609.344 * 1609.344)` coherent units.
    pub const PER_CUBIC_MILE_SECOND: SIUnit<
        VOLUMETRIC_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1)
                    / (Exact::dec(1609344, -3) * Exact::dec(1609344, -3) * Exact::dec(1609344, -3)),
                zero: Exact::ZERO,
                symbol: "mi⁻³ · s⁻¹",
                name: "per cubic mile second",
            }
        },
    > = SIUnit;

    /// Per cubic yard second (`yd⁻³ · s⁻¹`), a unit of `VolumetricNumberRate`: `n` of them is `n · 1.0 / (0.9144 * 0.9144 * 0.9144)` coherent units.
    pub const PER_CUBIC_YARD_SECOND: SIUnit<
        VOLUMETRIC_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1)
                    / (Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4)),
                zero: Exact::ZERO,
                symbol: "yd⁻³ · s⁻¹",
                name: "per cubic yard second",
            }
        },
    > = SIUnit;

    /// Per fluid ounce second (`fl oz⁻¹ · s⁻¹`), a unit of `VolumetricNumberRate`: `n` of them is `n · 1.0 / 2.95735295625e-05` coherent units.
    pub const PER_FLUID_OUNCE_SECOND: SIUnit<
        VOLUMETRIC_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(295735295625, -16),
                zero: Exact::ZERO,
                symbol: "fl oz⁻¹ · s⁻¹",
                name: "per fluid ounce second",
            }
        },
    > = SIUnit;

    /// Per Imperial fluid ounce second (`fl oz⁻¹ (UK) · s⁻¹`), a unit of `VolumetricNumberRate`: `n` of them is `n · 1.0 / 2.84130625e-05` coherent units.
    pub const PER_FLUID_OUNCE_IMPERIAL_SECOND: SIUnit<
        VOLUMETRIC_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(284130625, -13),
                zero: Exact::ZERO,
                symbol: "fl oz⁻¹ (UK) · s⁻¹",
                name: "per Imperial fluid ounce second",
            }
        },
    > = SIUnit;

    /// Per Imperial gallon second (`gal⁻¹ (UK) · s⁻¹`), a unit of `VolumetricNumberRate`: `n` of them is `n · 1.0 / 0.00454609` coherent units.
    pub const PER_GALLON_IMPERIAL_SECOND: SIUnit<
        VOLUMETRIC_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(454609, -8),
                zero: Exact::ZERO,
                symbol: "gal⁻¹ (UK) · s⁻¹",
                name: "per Imperial gallon second",
            }
        },
    > = SIUnit;

    /// Per gallon second (`gal⁻¹ · s⁻¹`), a unit of `VolumetricNumberRate`: `n` of them is `n · 1.0 / 0.003785411784` coherent units.
    pub const PER_GALLON_SECOND: SIUnit<
        VOLUMETRIC_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(3785411784, -12),
                zero: Exact::ZERO,
                symbol: "gal⁻¹ · s⁻¹",
                name: "per gallon second",
            }
        },
    > = SIUnit;

    /// Per litre second (`L⁻¹ · s⁻¹`), a unit of `VolumetricNumberRate`: `n` of them is `n · 1000.0` coherent units.
    pub const PER_LITRE_SECOND: SIUnit<
        VOLUMETRIC_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "L⁻¹ · s⁻¹",
                name: "per litre second",
            }
        },
    > = SIUnit;

    /// Becquerels per cubic metre (`Bq/m³`), a unit of `VolumetricNumberRate`: `n` of them is `n · 1.0` coherent units.
    pub const BECQUEREL_PER_CUBIC_METRE: SIUnit<
        VOLUMETRIC_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Bq/m³",
                name: "becquerels per cubic metre",
            }
        },
    > = SIUnit;

    /// Curies per cubic metre (`Ci/m³`), a unit of `VolumetricNumberRate`: `n` of them is `n · 37000000000.0` coherent units.
    pub const CURIE_PER_CUBIC_METRE: SIUnit<
        VOLUMETRIC_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(37000000000),
                zero: Exact::ZERO,
                symbol: "Ci/m³",
                name: "curies per cubic metre",
            }
        },
    > = SIUnit;

    /// Disintegrations per minute per cubic metre (`dpm/m³`), a unit of `VolumetricNumberRate`: `n` of them is `n · 1.0 / 60.0` coherent units.
    pub const DISINTEGRATIONS_PER_MINUTE_PER_CUBIC_METRE: SIUnit<
        VOLUMETRIC_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "dpm/m³",
                name: "disintegrations per minute per cubic metre",
            }
        },
    > = SIUnit;

    /// States per metre joule (`1/(m · J)`), a unit of `LinearDensityOfStates`: `n` of them is `n · 1.0` coherent units.
    pub const STATE_PER_METRE_JOULE: SIUnit<
        LINEAR_DENSITY_OF_STATES,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "1/(m · J)",
                name: "states per metre joule",
            }
        },
    > = SIUnit;

    /// States per square metre joule (`1/(m² · J)`), a unit of `ArealDensityOfStates`: `n` of them is `n · 1.0` coherent units.
    pub const STATE_PER_SQUARE_METRE_JOULE: SIUnit<
        AREAL_DENSITY_OF_STATES,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "1/(m² · J)",
                name: "states per square metre joule",
            }
        },
    > = SIUnit;

    /// States per cubic metre joule (`1/(m³ · J)`), a unit of `VolumetricDensityOfStates`: `n` of them is `n · 1.0` coherent units.
    pub const STATE_PER_CUBIC_METRE_JOULE: SIUnit<
        VOLUMETRIC_DENSITY_OF_STATES,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "1/(m³ · J)",
                name: "states per cubic metre joule",
            }
        },
    > = SIUnit;

    /// Watts per metre (`W/m`), a unit of `LinearPowerDensity`: `n` of them is `n · 1.0` coherent units.
    pub const WATT_PER_METRE: SIUnit<
        LINEAR_POWER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W/m",
                name: "watts per metre",
            }
        },
    > = SIUnit;

    /// Watts per cubic metre (`W/m³`), a unit of `VolumetricPowerDensity`: `n` of them is `n · 1.0` coherent units.
    pub const WATT_PER_CUBIC_METRE: SIUnit<
        VOLUMETRIC_POWER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W/m³",
                name: "watts per cubic metre",
            }
        },
    > = SIUnit;
}
