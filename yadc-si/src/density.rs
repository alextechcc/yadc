use crate::{ISQ, SIQuantity};
use yadc::Dimension;

/// The dimension of `MassDensity`.
pub const MASS_DENSITY: ISQ = ISQ {
    m: -3,
    kg: 1,
    ..ISQ::DIMENSIONLESS
};
pub type MassDensity<N> = SIQuantity<MASS_DENSITY, N>;

/// The dimension of `LinearMassDensity`.
pub const LINEAR_MASS_DENSITY: ISQ = ISQ {
    m: -1,
    kg: 1,
    ..ISQ::DIMENSIONLESS
};
pub type LinearMassDensity<N> = SIQuantity<LINEAR_MASS_DENSITY, N>;

/// The dimension of `ArealMassDensity`.
pub const AREAL_MASS_DENSITY: ISQ = ISQ {
    m: -2,
    kg: 1,
    ..ISQ::DIMENSIONLESS
};
pub type ArealMassDensity<N> = SIQuantity<AREAL_MASS_DENSITY, N>;

/// The dimension of `LinearNumberDensity`.
pub const LINEAR_NUMBER_DENSITY: ISQ = ISQ {
    m: -1,
    ..ISQ::DIMENSIONLESS
};
pub type LinearNumberDensity<N> = SIQuantity<LINEAR_NUMBER_DENSITY, N>;

/// The dimension of `ArealNumberDensity`.
pub const AREAL_NUMBER_DENSITY: ISQ = ISQ {
    m: -2,
    ..ISQ::DIMENSIONLESS
};
pub type ArealNumberDensity<N> = SIQuantity<AREAL_NUMBER_DENSITY, N>;

/// The dimension of `VolumetricNumberDensity`.
pub const VOLUMETRIC_NUMBER_DENSITY: ISQ = ISQ {
    m: -3,
    ..ISQ::DIMENSIONLESS
};
pub type VolumetricNumberDensity<N> = SIQuantity<VOLUMETRIC_NUMBER_DENSITY, N>;

/// The dimension of `LinearNumberRate`.
pub const LINEAR_NUMBER_RATE: ISQ = ISQ {
    m: -1,
    s: -1,
    ..ISQ::DIMENSIONLESS
};
pub type LinearNumberRate<N> = SIQuantity<LINEAR_NUMBER_RATE, N>;

/// The dimension of `ArealNumberRate`.
pub const AREAL_NUMBER_RATE: ISQ = ISQ {
    m: -2,
    s: -1,
    ..ISQ::DIMENSIONLESS
};
pub type ArealNumberRate<N> = SIQuantity<AREAL_NUMBER_RATE, N>;

/// The dimension of `VolumetricNumberRate`.
pub const VOLUMETRIC_NUMBER_RATE: ISQ = ISQ {
    m: -3,
    s: -1,
    ..ISQ::DIMENSIONLESS
};
pub type VolumetricNumberRate<N> = SIQuantity<VOLUMETRIC_NUMBER_RATE, N>;

/// The dimension of `LinearDensityOfStates`.
pub const LINEAR_DENSITY_OF_STATES: ISQ = ISQ {
    m: -3,
    kg: -1,
    s: 2,
    ..ISQ::DIMENSIONLESS
};
pub type LinearDensityOfStates<N> = SIQuantity<LINEAR_DENSITY_OF_STATES, N>;

/// The dimension of `ArealDensityOfStates`.
pub const AREAL_DENSITY_OF_STATES: ISQ = ISQ {
    m: -4,
    kg: -1,
    s: 2,
    ..ISQ::DIMENSIONLESS
};
pub type ArealDensityOfStates<N> = SIQuantity<AREAL_DENSITY_OF_STATES, N>;

/// The dimension of `VolumetricDensityOfStates`.
pub const VOLUMETRIC_DENSITY_OF_STATES: ISQ = ISQ {
    m: -5,
    kg: -1,
    s: 2,
    ..ISQ::DIMENSIONLESS
};
pub type VolumetricDensityOfStates<N> = SIQuantity<VOLUMETRIC_DENSITY_OF_STATES, N>;

/// The dimension of `LinearPowerDensity`.
pub const LINEAR_POWER_DENSITY: ISQ = ISQ {
    m: 1,
    kg: 1,
    s: -3,
    ..ISQ::DIMENSIONLESS
};
pub type LinearPowerDensity<N> = SIQuantity<LINEAR_POWER_DENSITY, N>;

/// The dimension of `VolumetricPowerDensity`.
pub const VOLUMETRIC_POWER_DENSITY: ISQ = ISQ {
    m: -1,
    kg: 1,
    s: -3,
    ..ISQ::DIMENSIONLESS
};
pub type VolumetricPowerDensity<N> = SIQuantity<VOLUMETRIC_POWER_DENSITY, N>;

/// The units of this module's quantities: a type each, its size and zero
/// in its quantity's coherent unit, exact (`Mile {}.quantity(3.0_f64)`).
pub mod units {
    use super::*;
    use crate::{Exact, SIUnit, SIUnitConsts};

    /// Kilograms per cubic metre (`kg/m³`), a unit of `MassDensity`: `n` of them is `n · 1.0` coherent units.
    pub type KilogramPerCubicMetre = SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg/m³",
                name: "kilograms per cubic metre",
            }
        },
    >;

    /// Grams per cubic metre (`g/m³`), a unit of `MassDensity`: `n` of them is `n · 0.001` coherent units.
    pub type GramPerCubicMetre = SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "g/m³",
                name: "grams per cubic metre",
            }
        },
    >;

    /// Carats per cubic metre (`ct/m³`), a unit of `MassDensity`: `n` of them is `n · 0.0002` coherent units.
    pub type CaratPerCubicMetre = SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(2, -4),
                zero: Exact::ZERO,
                symbol: "ct/m³",
                name: "carats per cubic metre",
            }
        },
    >;

    /// Grains per cubic metre (`gr/m³`), a unit of `MassDensity`: `n` of them is `n · 6.479891e-05` coherent units.
    pub type GrainPerCubicMetre = SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(6479891, -11),
                zero: Exact::ZERO,
                symbol: "gr/m³",
                name: "grains per cubic metre",
            }
        },
    >;

    /// Hundredweight (long) per cubic metre (`cwt long/m³`), a unit of `MassDensity`: `n` of them is `n · 50.80234544` coherent units.
    pub type HundredweightLongPerCubicMetre = SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(5080234544, -8),
                zero: Exact::ZERO,
                symbol: "cwt long/m³",
                name: "hundredweight (long) per cubic metre",
            }
        },
    >;

    /// Hundredweight (short) per cubic metre (`cwt short/m³`), a unit of `MassDensity`: `n` of them is `n · 45.359237` coherent units.
    pub type HundredweightShortPerCubicMetre = SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -6),
                zero: Exact::ZERO,
                symbol: "cwt short/m³",
                name: "hundredweight (short) per cubic metre",
            }
        },
    >;

    /// Ounces per cubic metre (`oz/m³`), a unit of `MassDensity`: `n` of them is `n · 0.028349523125` coherent units.
    pub type OuncePerCubicMetre = SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(28349523125, -12),
                zero: Exact::ZERO,
                symbol: "oz/m³",
                name: "ounces per cubic metre",
            }
        },
    >;

    /// Troy ounces per cubic metre (`oz t/m³`), a unit of `MassDensity`: `n` of them is `n · 0.0311034768` coherent units.
    pub type OunceTroyPerCubicMetre = SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(311034768, -10),
                zero: Exact::ZERO,
                symbol: "oz t/m³",
                name: "troy ounces per cubic metre",
            }
        },
    >;

    /// Pennyweight per cubic metre (`dwt/m³`), a unit of `MassDensity`: `n` of them is `n · 0.00155517384` coherent units.
    pub type PennyweightPerCubicMetre = SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(155517384, -11),
                zero: Exact::ZERO,
                symbol: "dwt/m³",
                name: "pennyweight per cubic metre",
            }
        },
    >;

    /// Pounds per cubic metre (`lb/m³`), a unit of `MassDensity`: `n` of them is `n · 0.45359237` coherent units.
    pub type PoundPerCubicMetre = SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "lb/m³",
                name: "pounds per cubic metre",
            }
        },
    >;

    /// Troy pounds per cubic metre (`lb t/m³`), a unit of `MassDensity`: `n` of them is `n · 0.3732417216` coherent units.
    pub type PoundTroyPerCubicMetre = SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(3732417216, -10),
                zero: Exact::ZERO,
                symbol: "lb t/m³",
                name: "troy pounds per cubic metre",
            }
        },
    >;

    /// Slugs per cubic metre (`slug/m³`), a unit of `MassDensity`: `n` of them is `n · 0.45359237 * 9.80665 / 0.3048` coherent units.
    pub type SlugPerCubicMetre = SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "slug/m³",
                name: "slugs per cubic metre",
            }
        },
    >;

    /// Assay tons per cubic metre (`AT/m³`), a unit of `MassDensity`: `n` of them is `n · 0.175 / 6.0` coherent units.
    pub type TonAssayPerCubicMetre = SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(175, -3) / Exact::int(6),
                zero: Exact::ZERO,
                symbol: "AT/m³",
                name: "assay tons per cubic metre",
            }
        },
    >;

    /// Long tons per cubic metre (`2240 lb/m³`), a unit of `MassDensity`: `n` of them is `n · 1016.0469088` coherent units.
    pub type TonLongPerCubicMetre = SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(10160469088, -7),
                zero: Exact::ZERO,
                symbol: "2240 lb/m³",
                name: "long tons per cubic metre",
            }
        },
    >;

    /// Short tons per cubic metre (`2000 lb/m³`), a unit of `MassDensity`: `n` of them is `n · 907.18474` coherent units.
    pub type TonShortPerCubicMetre = SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(90718474, -5),
                zero: Exact::ZERO,
                symbol: "2000 lb/m³",
                name: "short tons per cubic metre",
            }
        },
    >;

    /// Tons per cubic metre (`t/m³`), a unit of `MassDensity`: `n` of them is `n · 1000.0` coherent units.
    pub type TonPerCubicMetre = SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "t/m³",
                name: "tons per cubic metre",
            }
        },
    >;

    /// Grains per gallon (`gr/gal`), a unit of `MassDensity`: `n` of them is `n · 6.479891e-05 / 0.003785411784` coherent units.
    pub type GrainPerGallon = SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(6479891, -11) / Exact::dec(3785411784, -12),
                zero: Exact::ZERO,
                symbol: "gr/gal",
                name: "grains per gallon",
            }
        },
    >;

    /// Ounces per cubic inch (`oz/in³`), a unit of `MassDensity`: `n` of them is `n · 0.028349523125 / (0.0254 * 0.0254 * 0.0254)` coherent units.
    pub type OuncePerCubicInch = SIUnit<
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
    >;

    /// Ounces per Imperial gallon (`oz/gal (UK)`), a unit of `MassDensity`: `n` of them is `n · 0.028349523125 / 0.00454609` coherent units.
    pub type OuncePerGallonImperial = SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(28349523125, -12) / Exact::dec(454609, -8),
                zero: Exact::ZERO,
                symbol: "oz/gal (UK)",
                name: "ounces per Imperial gallon",
            }
        },
    >;

    /// Ounces per gallon (`oz/gal`), a unit of `MassDensity`: `n` of them is `n · 0.028349523125 / 0.003785411784` coherent units.
    pub type OuncePerGallon = SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(28349523125, -12) / Exact::dec(3785411784, -12),
                zero: Exact::ZERO,
                symbol: "oz/gal",
                name: "ounces per gallon",
            }
        },
    >;

    /// Pounds per cubic foot (`lb/ft³`), a unit of `MassDensity`: `n` of them is `n · 0.45359237 / (0.3048 * 0.3048 * 0.3048)` coherent units.
    pub type PoundPerCubicFoot = SIUnit<
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
    >;

    /// Pounds per cubic inch (`lb/in³`), a unit of `MassDensity`: `n` of them is `n · 0.45359237 / (0.0254 * 0.0254 * 0.0254)` coherent units.
    pub type PoundPerCubicInch = SIUnit<
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
    >;

    /// Pounds per cubic yard (`lb/yd³`), a unit of `MassDensity`: `n` of them is `n · 0.45359237 / (0.9144 * 0.9144 * 0.9144)` coherent units.
    pub type PoundPerCubicYard = SIUnit<
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
    >;

    /// Pounds per Imperial gallon (`lb/gal (UK)`), a unit of `MassDensity`: `n` of them is `n · 0.45359237 / 0.00454609` coherent units.
    pub type PoundPerGallonImperial = SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) / Exact::dec(454609, -8),
                zero: Exact::ZERO,
                symbol: "lb/gal (UK)",
                name: "pounds per Imperial gallon",
            }
        },
    >;

    /// Pounds per gallon (`lb/gal`), a unit of `MassDensity`: `n` of them is `n · 0.45359237 / 0.003785411784` coherent units.
    pub type PoundPerGallon = SIUnit<
        MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) / Exact::dec(3785411784, -12),
                zero: Exact::ZERO,
                symbol: "lb/gal",
                name: "pounds per gallon",
            }
        },
    >;

    /// Slugs per cubic foot (`slug/ft³`), a unit of `MassDensity`: `n` of them is `n · (0.45359237 * 9.80665 / 0.3048) / (0.3048 * 0.3048 * 0.3048)` coherent units.
    pub type SlugPerCubicFoot = SIUnit<
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
    >;

    /// Long tons per cubic yard (`2240 lb/yd³`), a unit of `MassDensity`: `n` of them is `n · 1016.0469088 / (0.9144 * 0.9144 * 0.9144)` coherent units.
    pub type TonLongPerCubicYard = SIUnit<
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
    >;

    /// Short tons per cubic yard (`2000 lb/yd³`), a unit of `MassDensity`: `n` of them is `n · 907.18474 / (0.9144 * 0.9144 * 0.9144)` coherent units.
    pub type TonShortPerCubicYard = SIUnit<
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
    >;

    /// Long tons per cubic foot (`2240 lb/ft³`), a unit of `MassDensity`: `n` of them is `n · 1016.0469088 / (0.3048 * 0.3048 * 0.3048)` coherent units.
    pub type TonLongPerCubicFoot = SIUnit<
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
    >;

    /// Short tons per cubic foot (`2000 lb/ft³`), a unit of `MassDensity`: `n` of them is `n · 907.18474 / (0.3048 * 0.3048 * 0.3048)` coherent units.
    pub type TonShortPerCubicFoot = SIUnit<
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
    >;

    /// Kilograms per metre (`kg/m`), a unit of `LinearMassDensity`: `n` of them is `n · 1.0` coherent units.
    pub type KilogramPerMetre = SIUnit<
        LINEAR_MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg/m",
                name: "kilograms per metre",
            }
        },
    >;

    /// Ounces per foot (`oz/ft`), a unit of `LinearMassDensity`: `n` of them is `n · 0.028349523125 / 0.3048` coherent units.
    pub type OuncePerFoot = SIUnit<
        LINEAR_MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(28349523125, -12) / Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "oz/ft",
                name: "ounces per foot",
            }
        },
    >;

    /// Ounces per inch (`oz/in`), a unit of `LinearMassDensity`: `n` of them is `n · 0.028349523125 / 0.0254` coherent units.
    pub type OuncePerInch = SIUnit<
        LINEAR_MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(28349523125, -12) / Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "oz/in",
                name: "ounces per inch",
            }
        },
    >;

    /// Pounds per yard (`lb/yd`), a unit of `LinearMassDensity`: `n` of them is `n · 0.45359237 / 0.9144` coherent units.
    pub type PoundPerYard = SIUnit<
        LINEAR_MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) / Exact::dec(9144, -4),
                zero: Exact::ZERO,
                symbol: "lb/yd",
                name: "pounds per yard",
            }
        },
    >;

    /// Pounds per foot (`lb/ft`), a unit of `LinearMassDensity`: `n` of them is `n · 0.45359237 / 0.3048` coherent units.
    pub type PoundPerFoot = SIUnit<
        LINEAR_MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) / Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "lb/ft",
                name: "pounds per foot",
            }
        },
    >;

    /// Pounds per inch (`lb/in`), a unit of `LinearMassDensity`: `n` of them is `n · 0.45359237 / 0.0254` coherent units.
    pub type PoundPerInch = SIUnit<
        LINEAR_MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) / Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "lb/in",
                name: "pounds per inch",
            }
        },
    >;

    /// Grams per kilometre (`g/km`), a unit of `LinearMassDensity`: `n` of them is `n · 1e-06` coherent units.
    pub type GramPerKilometre = SIUnit<
        LINEAR_MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -6),
                zero: Exact::ZERO,
                symbol: "g/km",
                name: "grams per kilometre",
            }
        },
    >;

    /// Grams per centimetre (`g/cm`), a unit of `LinearMassDensity`: `n` of them is `n · 0.1` coherent units.
    pub type GramPerCentimetre = SIUnit<
        LINEAR_MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -1),
                zero: Exact::ZERO,
                symbol: "g/cm",
                name: "grams per centimetre",
            }
        },
    >;

    /// Kilograms per square metre (`kg/m²`), a unit of `ArealMassDensity`: `n` of them is `n · 1.0` coherent units.
    pub type KilogramPerSquareMetre = SIUnit<
        AREAL_MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg/m²",
                name: "kilograms per square metre",
            }
        },
    >;

    /// Grams per square metre (`g/m²`), a unit of `ArealMassDensity`: `n` of them is `n · 0.001` coherent units.
    pub type GramPerSquareMetre = SIUnit<
        AREAL_MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "g/m²",
                name: "grams per square metre",
            }
        },
    >;

    /// Ounces per square foot (`oz/ft²`), a unit of `ArealMassDensity`: `n` of them is `n · 0.028349523125 / (0.3048 * 0.3048)` coherent units.
    pub type OuncePerSquareFoot = SIUnit<
        AREAL_MASS_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(28349523125, -12) / (Exact::dec(3048, -4) * Exact::dec(3048, -4)),
                zero: Exact::ZERO,
                symbol: "oz/ft²",
                name: "ounces per square foot",
            }
        },
    >;

    /// Per metre (`m⁻¹`), a unit of `LinearNumberDensity`: `n` of them is `n · 1.0` coherent units.
    pub type PerMetre = SIUnit<
        LINEAR_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m⁻¹",
                name: "per metre",
            }
        },
    >;

    /// Per foot (`ft⁻¹`), a unit of `LinearNumberDensity`: `n` of them is `n · 1.0 / 0.3048` coherent units.
    pub type PerFoot = SIUnit<
        LINEAR_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "ft⁻¹",
                name: "per foot",
            }
        },
    >;

    /// Foot (U.S. survey) (`ft (U.S. survey)`), a unit of `LinearNumberDensity`: `n` of them is `n · 1.0 / (1200.0 / 3937.0)` coherent units.
    pub type PerFootSurvey = SIUnit<
        LINEAR_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::int(1200) / Exact::int(3937)),
                zero: Exact::ZERO,
                symbol: "ft (U.S. survey)",
                name: "foot (U.S. survey)",
            }
        },
    >;

    /// Per inch (`in⁻¹`), a unit of `LinearNumberDensity`: `n` of them is `n · 1.0 / 0.0254` coherent units.
    pub type PerInch = SIUnit<
        LINEAR_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "in⁻¹",
                name: "per inch",
            }
        },
    >;

    /// Per mile (`mi⁻¹`), a unit of `LinearNumberDensity`: `n` of them is `n · 1.0 / 1609.344` coherent units.
    pub type PerMile = SIUnit<
        LINEAR_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(1609344, -3),
                zero: Exact::ZERO,
                symbol: "mi⁻¹",
                name: "per mile",
            }
        },
    >;

    /// Per mile (U.S. survey) (`mi⁻¹ (U.S. survey)`), a unit of `LinearNumberDensity`: `n` of them is `n · 1.0 / (6336000.0 / 3937.0)` coherent units.
    pub type PerMileSurvey = SIUnit<
        LINEAR_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::int(6336000) / Exact::int(3937)),
                zero: Exact::ZERO,
                symbol: "mi⁻¹ (U.S. survey)",
                name: "per mile (U.S. survey)",
            }
        },
    >;

    /// Per nautical mile (`M⁻¹`), a unit of `LinearNumberDensity`: `n` of them is `n · 1.0 / 1852.0` coherent units.
    pub type PerNauticalMile = SIUnit<
        LINEAR_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(1852),
                zero: Exact::ZERO,
                symbol: "M⁻¹",
                name: "per nautical mile",
            }
        },
    >;

    /// Per yard (`yd⁻¹`), a unit of `LinearNumberDensity`: `n` of them is `n · 1.0 / 0.9144` coherent units.
    pub type PerYard = SIUnit<
        LINEAR_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(9144, -4),
                zero: Exact::ZERO,
                symbol: "yd⁻¹",
                name: "per yard",
            }
        },
    >;

    /// Per square metre (`m⁻²`), a unit of `ArealNumberDensity`: `n` of them is `n · 1.0` coherent units.
    pub type PerSquareMetre = SIUnit<
        AREAL_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m⁻²",
                name: "per square metre",
            }
        },
    >;

    /// Per acre (`ac⁻²`), a unit of `ArealNumberDensity`: `n` of them is `n · 1.0 / (43560.0 * (1200.0 / 3937.0) * (1200.0 / 3937.0))` coherent units.
    pub type PerAcre = SIUnit<
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
    >;

    /// Per are (`a⁻²`), a unit of `ArealNumberDensity`: `n` of them is `n · 0.01` coherent units.
    pub type PerAre = SIUnit<
        AREAL_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -2),
                zero: Exact::ZERO,
                symbol: "a⁻²",
                name: "per are",
            }
        },
    >;

    /// Per barn (`b⁻²`), a unit of `ArealNumberDensity`: `n` of them is `n · 1e+28` coherent units.
    pub type PerBarn = SIUnit<
        AREAL_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(10000000000000000000000000000),
                zero: Exact::ZERO,
                symbol: "b⁻²",
                name: "per barn",
            }
        },
    >;

    /// Per circular mil (`cmil⁻²`), a unit of `ArealNumberDensity`: `n` of them is `n · 4.0 / (core::f64::consts::PI * 2.54e-05 * 2.54e-05)` coherent units.
    pub type PerCircularMil = SIUnit<
        AREAL_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(4) / (Exact::PI * Exact::dec(254, -7) * Exact::dec(254, -7)),
                zero: Exact::ZERO,
                symbol: "cmil⁻²",
                name: "per circular mil",
            }
        },
    >;

    /// Per hectare (`ha⁻²`), a unit of `ArealNumberDensity`: `n` of them is `n · 0.0001` coherent units.
    pub type PerHectare = SIUnit<
        AREAL_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -4),
                zero: Exact::ZERO,
                symbol: "ha⁻²",
                name: "per hectare",
            }
        },
    >;

    /// Per square foot (`ft⁻²`), a unit of `ArealNumberDensity`: `n` of them is `n · 1.0 / (0.3048 * 0.3048)` coherent units.
    pub type PerSquareFoot = SIUnit<
        AREAL_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(3048, -4) * Exact::dec(3048, -4)),
                zero: Exact::ZERO,
                symbol: "ft⁻²",
                name: "per square foot",
            }
        },
    >;

    /// Per square inch (`in⁻²`), a unit of `ArealNumberDensity`: `n` of them is `n · 1.0 / (0.0254 * 0.0254)` coherent units.
    pub type PerSquareInch = SIUnit<
        AREAL_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(254, -4) * Exact::dec(254, -4)),
                zero: Exact::ZERO,
                symbol: "in⁻²",
                name: "per square inch",
            }
        },
    >;

    /// Per square mile (`mi⁻²`), a unit of `ArealNumberDensity`: `n` of them is `n · 1.0 / (1609.344 * 1609.344)` coherent units.
    pub type PerSquareMile = SIUnit<
        AREAL_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(1609344, -3) * Exact::dec(1609344, -3)),
                zero: Exact::ZERO,
                symbol: "mi⁻²",
                name: "per square mile",
            }
        },
    >;

    /// Per square yard (`yd⁻²`), a unit of `ArealNumberDensity`: `n` of them is `n · 1.0 / (0.9144 * 0.9144)` coherent units.
    pub type PerSquareYard = SIUnit<
        AREAL_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(9144, -4) * Exact::dec(9144, -4)),
                zero: Exact::ZERO,
                symbol: "yd⁻²",
                name: "per square yard",
            }
        },
    >;

    /// Per cubic metre (`m⁻³`), a unit of `VolumetricNumberDensity`: `n` of them is `n · 1.0` coherent units.
    pub type PerCubicMetre = SIUnit<
        VOLUMETRIC_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m⁻³",
                name: "per cubic metre",
            }
        },
    >;

    /// Per cubic foot (`ft⁻³`), a unit of `VolumetricNumberDensity`: `n` of them is `n · 1.0 / (0.3048 * 0.3048 * 0.3048)` coherent units.
    pub type PerCubicFoot = SIUnit<
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
    >;

    /// Per cubic inch (`in⁻³`), a unit of `VolumetricNumberDensity`: `n` of them is `n · 1.0 / (0.0254 * 0.0254 * 0.0254)` coherent units.
    pub type PerCubicInch = SIUnit<
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
    >;

    /// Per cubic mile (`mi⁻³`), a unit of `VolumetricNumberDensity`: `n` of them is `n · 1.0 / (1609.344 * 1609.344 * 1609.344)` coherent units.
    pub type PerCubicMile = SIUnit<
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
    >;

    /// Per cubic yard (`yd⁻³`), a unit of `VolumetricNumberDensity`: `n` of them is `n · 1.0 / (0.9144 * 0.9144 * 0.9144)` coherent units.
    pub type PerCubicYard = SIUnit<
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
    >;

    /// Per fluid ounce (`per fl oz`), a unit of `VolumetricNumberDensity`: `n` of them is `n · 1.0 / 2.95735295625e-05` coherent units.
    pub type PerFluidOunce = SIUnit<
        VOLUMETRIC_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(295735295625, -16),
                zero: Exact::ZERO,
                symbol: "per fl oz",
                name: "per fluid ounce",
            }
        },
    >;

    /// Per Imperial fluid ounce (`per fl oz (UK)`), a unit of `VolumetricNumberDensity`: `n` of them is `n · 1.0 / 2.84130625e-05` coherent units.
    pub type PerFluidOunceImperial = SIUnit<
        VOLUMETRIC_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(284130625, -13),
                zero: Exact::ZERO,
                symbol: "per fl oz (UK)",
                name: "per Imperial fluid ounce",
            }
        },
    >;

    /// Per Imperial gallon (`per gal (UK)`), a unit of `VolumetricNumberDensity`: `n` of them is `n · 1.0 / 0.00454609` coherent units.
    pub type PerGallonImperial = SIUnit<
        VOLUMETRIC_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(454609, -8),
                zero: Exact::ZERO,
                symbol: "per gal (UK)",
                name: "per Imperial gallon",
            }
        },
    >;

    /// Per gallon (`per gal`), a unit of `VolumetricNumberDensity`: `n` of them is `n · 1.0 / 0.003785411784` coherent units.
    pub type PerGallon = SIUnit<
        VOLUMETRIC_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(3785411784, -12),
                zero: Exact::ZERO,
                symbol: "per gal",
                name: "per gallon",
            }
        },
    >;

    /// Per litre (`L⁻¹`), a unit of `VolumetricNumberDensity`: `n` of them is `n · 1000.0` coherent units.
    pub type PerLitre = SIUnit<
        VOLUMETRIC_NUMBER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "L⁻¹",
                name: "per litre",
            }
        },
    >;

    /// Per metre second (`m⁻¹ · s⁻¹`), a unit of `LinearNumberRate`: `n` of them is `n · 1.0` coherent units.
    pub type PerMetreSecond = SIUnit<
        LINEAR_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m⁻¹ · s⁻¹",
                name: "per metre second",
            }
        },
    >;

    /// Per foot second (`ft⁻¹ · s⁻¹`), a unit of `LinearNumberRate`: `n` of them is `n · 1.0 / 0.3048` coherent units.
    pub type PerFootSecond = SIUnit<
        LINEAR_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "ft⁻¹ · s⁻¹",
                name: "per foot second",
            }
        },
    >;

    /// Per foot (U.S. survey) second (`ft⁻¹ (U.S. survey) · s⁻¹`), a unit of `LinearNumberRate`: `n` of them is `n · 1.0 / (1200.0 / 3937.0)` coherent units.
    pub type PerFootSurveySecond = SIUnit<
        LINEAR_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::int(1200) / Exact::int(3937)),
                zero: Exact::ZERO,
                symbol: "ft⁻¹ (U.S. survey) · s⁻¹",
                name: "per foot (U.S. survey) second",
            }
        },
    >;

    /// Per inch second (`in⁻¹ · s⁻¹`), a unit of `LinearNumberRate`: `n` of them is `n · 1.0 / 0.0254` coherent units.
    pub type PerInchSecond = SIUnit<
        LINEAR_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "in⁻¹ · s⁻¹",
                name: "per inch second",
            }
        },
    >;

    /// Per mile second (`mi⁻¹ · s⁻¹`), a unit of `LinearNumberRate`: `n` of them is `n · 1.0 / 1609.344` coherent units.
    pub type PerMileSecond = SIUnit<
        LINEAR_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(1609344, -3),
                zero: Exact::ZERO,
                symbol: "mi⁻¹ · s⁻¹",
                name: "per mile second",
            }
        },
    >;

    /// Per mile (U.S. survey) second (`mi⁻¹ (U.S. survey) · s⁻¹`), a unit of `LinearNumberRate`: `n` of them is `n · 1.0 / (6336000.0 / 3937.0)` coherent units.
    pub type PerMileSurveySecond = SIUnit<
        LINEAR_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::int(6336000) / Exact::int(3937)),
                zero: Exact::ZERO,
                symbol: "mi⁻¹ (U.S. survey) · s⁻¹",
                name: "per mile (U.S. survey) second",
            }
        },
    >;

    /// Per yard second (`yd⁻¹ · s⁻¹`), a unit of `LinearNumberRate`: `n` of them is `n · 1.0 / 0.9144` coherent units.
    pub type PerYardSecond = SIUnit<
        LINEAR_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(9144, -4),
                zero: Exact::ZERO,
                symbol: "yd⁻¹ · s⁻¹",
                name: "per yard second",
            }
        },
    >;

    /// Per square metre second (`m⁻² · s⁻¹`), a unit of `ArealNumberRate`: `n` of them is `n · 1.0` coherent units.
    pub type PerSquareMetreSecond = SIUnit<
        AREAL_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m⁻² · s⁻¹",
                name: "per square metre second",
            }
        },
    >;

    /// Per acre second (`ac⁻¹ · s⁻¹`), a unit of `ArealNumberRate`: `n` of them is `n · 1.0 / (43560.0 * (1200.0 / 3937.0) * (1200.0 / 3937.0))` coherent units.
    pub type PerAcreSecond = SIUnit<
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
    >;

    /// Per are second (`a⁻¹ · s⁻¹`), a unit of `ArealNumberRate`: `n` of them is `n · 0.01` coherent units.
    pub type PerAreSecond = SIUnit<
        AREAL_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -2),
                zero: Exact::ZERO,
                symbol: "a⁻¹ · s⁻¹",
                name: "per are second",
            }
        },
    >;

    /// Per barn second (`b⁻¹ · s⁻¹`), a unit of `ArealNumberRate`: `n` of them is `n · 1e+28` coherent units.
    pub type PerBarnSecond = SIUnit<
        AREAL_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(10000000000000000000000000000),
                zero: Exact::ZERO,
                symbol: "b⁻¹ · s⁻¹",
                name: "per barn second",
            }
        },
    >;

    /// Per circular mil second (`cmil⁻¹ · s⁻¹`), a unit of `ArealNumberRate`: `n` of them is `n · 4.0 / (core::f64::consts::PI * 2.54e-05 * 2.54e-05)` coherent units.
    pub type PerCircularMilSecond = SIUnit<
        AREAL_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(4) / (Exact::PI * Exact::dec(254, -7) * Exact::dec(254, -7)),
                zero: Exact::ZERO,
                symbol: "cmil⁻¹ · s⁻¹",
                name: "per circular mil second",
            }
        },
    >;

    /// Per hectare second (`ha⁻¹ · s⁻¹`), a unit of `ArealNumberRate`: `n` of them is `n · 0.0001` coherent units.
    pub type PerHectareSecond = SIUnit<
        AREAL_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -4),
                zero: Exact::ZERO,
                symbol: "ha⁻¹ · s⁻¹",
                name: "per hectare second",
            }
        },
    >;

    /// Per square foot second (`ft⁻² · s⁻¹`), a unit of `ArealNumberRate`: `n` of them is `n · 1.0 / (0.3048 * 0.3048)` coherent units.
    pub type PerSquareFootSecond = SIUnit<
        AREAL_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(3048, -4) * Exact::dec(3048, -4)),
                zero: Exact::ZERO,
                symbol: "ft⁻² · s⁻¹",
                name: "per square foot second",
            }
        },
    >;

    /// Per square inch second (`in⁻² · s⁻¹`), a unit of `ArealNumberRate`: `n` of them is `n · 1.0 / (0.0254 * 0.0254)` coherent units.
    pub type PerSquareInchSecond = SIUnit<
        AREAL_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(254, -4) * Exact::dec(254, -4)),
                zero: Exact::ZERO,
                symbol: "in⁻² · s⁻¹",
                name: "per square inch second",
            }
        },
    >;

    /// Per square mile second (`mi⁻² · s⁻¹`), a unit of `ArealNumberRate`: `n` of them is `n · 1.0 / (1609.344 * 1609.344)` coherent units.
    pub type PerSquareMileSecond = SIUnit<
        AREAL_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(1609344, -3) * Exact::dec(1609344, -3)),
                zero: Exact::ZERO,
                symbol: "mi⁻² · s⁻¹",
                name: "per square mile second",
            }
        },
    >;

    /// Per square yard second (`yd⁻² · s⁻¹`), a unit of `ArealNumberRate`: `n` of them is `n · 1.0 / (0.9144 * 0.9144)` coherent units.
    pub type PerSquareYardSecond = SIUnit<
        AREAL_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(9144, -4) * Exact::dec(9144, -4)),
                zero: Exact::ZERO,
                symbol: "yd⁻² · s⁻¹",
                name: "per square yard second",
            }
        },
    >;

    /// Per cubic metre second (`m⁻³ · s⁻¹`), a unit of `VolumetricNumberRate`: `n` of them is `n · 1.0` coherent units.
    pub type PerCubicMetreSecond = SIUnit<
        VOLUMETRIC_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m⁻³ · s⁻¹",
                name: "per cubic metre second",
            }
        },
    >;

    /// Per cubic foot second (`ft⁻³ · s⁻¹`), a unit of `VolumetricNumberRate`: `n` of them is `n · 1.0 / (0.3048 * 0.3048 * 0.3048)` coherent units.
    pub type PerCubicFootSecond = SIUnit<
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
    >;

    /// Per cubic inch second (`in⁻³ · s⁻¹`), a unit of `VolumetricNumberRate`: `n` of them is `n · 1.0 / (0.0254 * 0.0254 * 0.0254)` coherent units.
    pub type PerCubicInchSecond = SIUnit<
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
    >;

    /// Per cubic mile second (`mi⁻³ · s⁻¹`), a unit of `VolumetricNumberRate`: `n` of them is `n · 1.0 / (1609.344 * 1609.344 * 1609.344)` coherent units.
    pub type PerCubicMileSecond = SIUnit<
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
    >;

    /// Per cubic yard second (`yd⁻³ · s⁻¹`), a unit of `VolumetricNumberRate`: `n` of them is `n · 1.0 / (0.9144 * 0.9144 * 0.9144)` coherent units.
    pub type PerCubicYardSecond = SIUnit<
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
    >;

    /// Per fluid ounce second (`fl oz⁻¹ · s⁻¹`), a unit of `VolumetricNumberRate`: `n` of them is `n · 1.0 / 2.95735295625e-05` coherent units.
    pub type PerFluidOunceSecond = SIUnit<
        VOLUMETRIC_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(295735295625, -16),
                zero: Exact::ZERO,
                symbol: "fl oz⁻¹ · s⁻¹",
                name: "per fluid ounce second",
            }
        },
    >;

    /// Per Imperial fluid ounce second (`fl oz⁻¹ (UK) · s⁻¹`), a unit of `VolumetricNumberRate`: `n` of them is `n · 1.0 / 2.84130625e-05` coherent units.
    pub type PerFluidOunceImperialSecond = SIUnit<
        VOLUMETRIC_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(284130625, -13),
                zero: Exact::ZERO,
                symbol: "fl oz⁻¹ (UK) · s⁻¹",
                name: "per Imperial fluid ounce second",
            }
        },
    >;

    /// Per Imperial gallon second (`gal⁻¹ (UK) · s⁻¹`), a unit of `VolumetricNumberRate`: `n` of them is `n · 1.0 / 0.00454609` coherent units.
    pub type PerGallonImperialSecond = SIUnit<
        VOLUMETRIC_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(454609, -8),
                zero: Exact::ZERO,
                symbol: "gal⁻¹ (UK) · s⁻¹",
                name: "per Imperial gallon second",
            }
        },
    >;

    /// Per gallon second (`gal⁻¹ · s⁻¹`), a unit of `VolumetricNumberRate`: `n` of them is `n · 1.0 / 0.003785411784` coherent units.
    pub type PerGallonSecond = SIUnit<
        VOLUMETRIC_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(3785411784, -12),
                zero: Exact::ZERO,
                symbol: "gal⁻¹ · s⁻¹",
                name: "per gallon second",
            }
        },
    >;

    /// Per litre second (`L⁻¹ · s⁻¹`), a unit of `VolumetricNumberRate`: `n` of them is `n · 1000.0` coherent units.
    pub type PerLitreSecond = SIUnit<
        VOLUMETRIC_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "L⁻¹ · s⁻¹",
                name: "per litre second",
            }
        },
    >;

    /// Becquerels per cubic metre (`Bq/m³`), a unit of `VolumetricNumberRate`: `n` of them is `n · 1.0` coherent units.
    pub type BecquerelPerCubicMetre = SIUnit<
        VOLUMETRIC_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Bq/m³",
                name: "becquerels per cubic metre",
            }
        },
    >;

    /// Curies per cubic metre (`Ci/m³`), a unit of `VolumetricNumberRate`: `n` of them is `n · 37000000000.0` coherent units.
    pub type CuriePerCubicMetre = SIUnit<
        VOLUMETRIC_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(37000000000),
                zero: Exact::ZERO,
                symbol: "Ci/m³",
                name: "curies per cubic metre",
            }
        },
    >;

    /// Disintegrations per minute per cubic metre (`dpm/m³`), a unit of `VolumetricNumberRate`: `n` of them is `n · 1.0 / 60.0` coherent units.
    pub type DisintegrationsPerMinutePerCubicMetre = SIUnit<
        VOLUMETRIC_NUMBER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "dpm/m³",
                name: "disintegrations per minute per cubic metre",
            }
        },
    >;

    /// States per metre joule (`1/(m · J)`), a unit of `LinearDensityOfStates`: `n` of them is `n · 1.0` coherent units.
    pub type StatePerMetreJoule = SIUnit<
        LINEAR_DENSITY_OF_STATES,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "1/(m · J)",
                name: "states per metre joule",
            }
        },
    >;

    /// States per square metre joule (`1/(m² · J)`), a unit of `ArealDensityOfStates`: `n` of them is `n · 1.0` coherent units.
    pub type StatePerSquareMetreJoule = SIUnit<
        AREAL_DENSITY_OF_STATES,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "1/(m² · J)",
                name: "states per square metre joule",
            }
        },
    >;

    /// States per cubic metre joule (`1/(m³ · J)`), a unit of `VolumetricDensityOfStates`: `n` of them is `n · 1.0` coherent units.
    pub type StatePerCubicMetreJoule = SIUnit<
        VOLUMETRIC_DENSITY_OF_STATES,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "1/(m³ · J)",
                name: "states per cubic metre joule",
            }
        },
    >;

    /// Watts per metre (`W/m`), a unit of `LinearPowerDensity`: `n` of them is `n · 1.0` coherent units.
    pub type WattPerMetre = SIUnit<
        LINEAR_POWER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W/m",
                name: "watts per metre",
            }
        },
    >;

    /// Watts per cubic metre (`W/m³`), a unit of `VolumetricPowerDensity`: `n` of them is `n · 1.0` coherent units.
    pub type WattPerCubicMetre = SIUnit<
        VOLUMETRIC_POWER_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W/m³",
                name: "watts per cubic metre",
            }
        },
    >;
}
