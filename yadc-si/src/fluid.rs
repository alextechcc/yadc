use crate::ISQ;
use yadc::Dimension;

/// The dimension of `DynamicViscosity`: `SIQuantity<DYNAMIC_VISCOSITY, N>` is one.
pub const DYNAMIC_VISCOSITY: ISQ = ISQ {
    m: -1,
    kg: 1,
    s: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `KinematicViscosity`: `SIQuantity<KINEMATIC_VISCOSITY, N>` is one.
pub const KINEMATIC_VISCOSITY: ISQ = ISQ {
    m: 2,
    s: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `SurfaceTension`: `SIQuantity<SURFACE_TENSION, N>` is one.
pub const SURFACE_TENSION: ISQ = ISQ {
    kg: 1,
    s: -2,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `CompressibilityCoefficient`: `SIQuantity<COMPRESSIBILITY_COEFFICIENT, N>` is one.
pub const COMPRESSIBILITY_COEFFICIENT: ISQ = ISQ {
    m: 1,
    kg: -1,
    s: 2,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `MassRate`: `SIQuantity<MASS_RATE, N>` is one.
pub const MASS_RATE: ISQ = ISQ {
    kg: 1,
    s: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `VolumeRate`: `SIQuantity<VOLUME_RATE, N>` is one.
pub const VOLUME_RATE: ISQ = ISQ {
    m: 3,
    s: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `MassFlux`: `SIQuantity<MASS_FLUX, N>` is one.
pub const MASS_FLUX: ISQ = ISQ {
    m: -2,
    kg: 1,
    s: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `MassPerEnergy`: `SIQuantity<MASS_PER_ENERGY, N>` is one.
pub const MASS_PER_ENERGY: ISQ = ISQ {
    m: -2,
    s: 2,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `DiffusionCoefficient`: `SIQuantity<DIFFUSION_COEFFICIENT, N>` is one.
pub const DIFFUSION_COEFFICIENT: ISQ = ISQ {
    m: 2,
    s: -1,
    ..ISQ::DIMENSIONLESS
};

/// The units of this module's quantities: each a constant of its own unit
/// type, its size and zero in its quantity's coherent unit, exact
/// (`MILE.quantity(3.0)`, `MILE.value(d)`).
pub mod units {
    use super::*;
    use crate::{Exact, SIUnit, SIUnitConsts};

    /// Pascal seconds (`Pa · s`), a unit of `DynamicViscosity`: `n` of them is `n · 1.0` coherent units.
    pub const PASCAL_SECOND: SIUnit<
        DYNAMIC_VISCOSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Pa · s",
                name: "pascal seconds",
            }
        },
    > = SIUnit;

    /// Poises (`P`), a unit of `DynamicViscosity`: `n` of them is `n · 0.1` coherent units.
    pub const POISE: SIUnit<
        DYNAMIC_VISCOSITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -1),
                zero: Exact::ZERO,
                symbol: "P",
                name: "poises",
            }
        },
    > = SIUnit;

    /// Pounds per foot second (`lb/(ft · s)`), a unit of `DynamicViscosity`: `n` of them is `n · 0.45359237 / 0.3048` coherent units.
    pub const POUND_PER_FOOT_SECOND: SIUnit<
        DYNAMIC_VISCOSITY,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) / Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "lb/(ft · s)",
                name: "pounds per foot second",
            }
        },
    > = SIUnit;

    /// Slugs per foot second (`slug/(ft · s)`), a unit of `DynamicViscosity`: `n` of them is `n · (0.45359237 * 9.80665 / 0.3048) / 0.3048` coherent units.
    pub const SLUG_PER_FOOT_SECOND: SIUnit<
        DYNAMIC_VISCOSITY,
        {
            SIUnitConsts {
                size: (Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4))
                    / Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "slug/(ft · s)",
                name: "slugs per foot second",
            }
        },
    > = SIUnit;

    /// Grams per centimetre second (`g/(cm · s)`), a unit of `DynamicViscosity`: `n` of them is `n · 0.1` coherent units.
    pub const GRAM_PER_CENTIMETRE_SECOND: SIUnit<
        DYNAMIC_VISCOSITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -1),
                zero: Exact::ZERO,
                symbol: "g/(cm · s)",
                name: "grams per centimetre second",
            }
        },
    > = SIUnit;

    /// Square metres per second (`m²/s`), a unit of `KinematicViscosity`: `n` of them is `n · 1.0` coherent units.
    pub const KINEMATIC_VISCOSITY_SQUARE_METRE_PER_SECOND: SIUnit<
        KINEMATIC_VISCOSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m²/s",
                name: "square metres per second",
            }
        },
    > = SIUnit;

    /// Stokes (`St`), a unit of `KinematicViscosity`: `n` of them is `n · 0.0001` coherent units.
    pub const KINEMATIC_VISCOSITY_STOKES: SIUnit<
        KINEMATIC_VISCOSITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -4),
                zero: Exact::ZERO,
                symbol: "St",
                name: "stokes",
            }
        },
    > = SIUnit;

    /// Newtons per metre (`N/m`), a unit of `SurfaceTension`: `n` of them is `n · 1.0` coherent units.
    pub const NEWTON_PER_METRE: SIUnit<
        SURFACE_TENSION,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "N/m",
                name: "newtons per metre",
            }
        },
    > = SIUnit;

    /// Kilograms-force per metre (`kgf/m`), a unit of `SurfaceTension`: `n` of them is `n · 9.80665` coherent units.
    pub const KILOGRAM_FORCE_PER_METRE: SIUnit<
        SURFACE_TENSION,
        {
            SIUnitConsts {
                size: Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "kgf/m",
                name: "kilograms-force per metre",
            }
        },
    > = SIUnit;

    /// Ounces-force per inch (`ozf/in`), a unit of `SurfaceTension`: `n` of them is `n · (0.45359237 * 9.80665 / 16.0) / 0.0254` coherent units.
    pub const OUNCE_FORCE_PER_INCH: SIUnit<
        SURFACE_TENSION,
        {
            SIUnitConsts {
                size: (Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::int(16))
                    / Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "ozf/in",
                name: "ounces-force per inch",
            }
        },
    > = SIUnit;

    /// Poundals per inch (`pdl/in`), a unit of `SurfaceTension`: `n` of them is `n · (0.45359237 * 0.3048) / 0.0254` coherent units.
    pub const POUNDAL_PER_INCH: SIUnit<
        SURFACE_TENSION,
        {
            SIUnitConsts {
                size: (Exact::dec(45359237, -8) * Exact::dec(3048, -4)) / Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "pdl/in",
                name: "poundals per inch",
            }
        },
    > = SIUnit;

    /// Pounds-force per inch (`lbf/in`), a unit of `SurfaceTension`: `n` of them is `n · (0.45359237 * 9.80665) / 0.0254` coherent units.
    pub const POUND_FORCE_PER_INCH: SIUnit<
        SURFACE_TENSION,
        {
            SIUnitConsts {
                size: (Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "lbf/in",
                name: "pounds-force per inch",
            }
        },
    > = SIUnit;

    /// Joules per square metre (`J/m²`), a unit of `SurfaceTension`: `n` of them is `n · 1.0` coherent units.
    pub const JOULE_PER_SQUARE_METRE: SIUnit<
        SURFACE_TENSION,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/m²",
                name: "joules per square metre",
            }
        },
    > = SIUnit;

    /// Per pascal (`Pa⁻¹`), a unit of `CompressibilityCoefficient`: `n` of them is `n · 1.0` coherent units.
    pub const PER_PASCAL: SIUnit<
        COMPRESSIBILITY_COEFFICIENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Pa⁻¹",
                name: "per pascal",
            }
        },
    > = SIUnit;

    /// Per atmosphere (`atm⁻¹`), a unit of `CompressibilityCoefficient`: `n` of them is `n · 1.0 / 101325.0` coherent units.
    pub const PER_ATMOSPHERE: SIUnit<
        COMPRESSIBILITY_COEFFICIENT,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(101325),
                zero: Exact::ZERO,
                symbol: "atm⁻¹",
                name: "per atmosphere",
            }
        },
    > = SIUnit;

    /// Per atmosphere (technical) (`at⁻¹`), a unit of `CompressibilityCoefficient`: `n` of them is `n · 1.0 / 98066.5` coherent units.
    pub const PER_ATMOSPHERE_TECHNICAL: SIUnit<
        COMPRESSIBILITY_COEFFICIENT,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(980665, -1),
                zero: Exact::ZERO,
                symbol: "at⁻¹",
                name: "per atmosphere (technical)",
            }
        },
    > = SIUnit;

    /// Per bar (`bar⁻¹`), a unit of `CompressibilityCoefficient`: `n` of them is `n · 1e-05` coherent units.
    pub const PER_BAR: SIUnit<
        COMPRESSIBILITY_COEFFICIENT,
        {
            SIUnitConsts {
                size: Exact::dec(1, -5),
                zero: Exact::ZERO,
                symbol: "bar⁻¹",
                name: "per bar",
            }
        },
    > = SIUnit;

    /// Kilograms per second (`kg/s`), a unit of `MassRate`: `n` of them is `n · 1.0` coherent units.
    pub const KILOGRAM_PER_SECOND: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg/s",
                name: "kilograms per second",
            }
        },
    > = SIUnit;

    /// Grams per second (`g/s`), a unit of `MassRate`: `n` of them is `n · 0.001` coherent units.
    pub const GRAM_PER_SECOND: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "g/s",
                name: "grams per second",
            }
        },
    > = SIUnit;

    /// Grams per minute (`g/min`), a unit of `MassRate`: `n` of them is `n · 0.001 / 60.0` coherent units.
    pub const GRAM_PER_MINUTE: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "g/min",
                name: "grams per minute",
            }
        },
    > = SIUnit;

    /// Grams per hour (`g/h`), a unit of `MassRate`: `n` of them is `n · 0.001 / 3600.0` coherent units.
    pub const GRAM_PER_HOUR: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3) / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "g/h",
                name: "grams per hour",
            }
        },
    > = SIUnit;

    /// Grams per day (`g/d`), a unit of `MassRate`: `n` of them is `n · 0.001 / 86400.0` coherent units.
    pub const GRAM_PER_DAY: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3) / Exact::int(86400),
                zero: Exact::ZERO,
                symbol: "g/d",
                name: "grams per day",
            }
        },
    > = SIUnit;

    /// Carats per second (`ct/s`), a unit of `MassRate`: `n` of them is `n · 0.0002` coherent units.
    pub const CARAT_PER_SECOND: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(2, -4),
                zero: Exact::ZERO,
                symbol: "ct/s",
                name: "carats per second",
            }
        },
    > = SIUnit;

    /// Grains per second (`gr/s`), a unit of `MassRate`: `n` of them is `n · 6.479891e-05` coherent units.
    pub const GRAIN_PER_SECOND: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(6479891, -11),
                zero: Exact::ZERO,
                symbol: "gr/s",
                name: "grains per second",
            }
        },
    > = SIUnit;

    /// Hundredweight (long) per second (`cwt long/s`), a unit of `MassRate`: `n` of them is `n · 50.80234544` coherent units.
    pub const HUNDREDWEIGHT_LONG_PER_SECOND: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(5080234544, -8),
                zero: Exact::ZERO,
                symbol: "cwt long/s",
                name: "hundredweight (long) per second",
            }
        },
    > = SIUnit;

    /// Hundredweight (short) per second (`cwt short/s`), a unit of `MassRate`: `n` of them is `n · 45.359237` coherent units.
    pub const HUNDREDWEIGHT_SHORT_PER_SECOND: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -6),
                zero: Exact::ZERO,
                symbol: "cwt short/s",
                name: "hundredweight (short) per second",
            }
        },
    > = SIUnit;

    /// Ounces per second (`oz/s`), a unit of `MassRate`: `n` of them is `n · 0.028349523125` coherent units.
    pub const OUNCE_PER_SECOND: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(28349523125, -12),
                zero: Exact::ZERO,
                symbol: "oz/s",
                name: "ounces per second",
            }
        },
    > = SIUnit;

    /// Troy ounces per second (`oz t/s`), a unit of `MassRate`: `n` of them is `n · 0.0311034768` coherent units.
    pub const OUNCE_TROY_PER_SECOND: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(311034768, -10),
                zero: Exact::ZERO,
                symbol: "oz t/s",
                name: "troy ounces per second",
            }
        },
    > = SIUnit;

    /// Pennyweight per second (`dwt/s`), a unit of `MassRate`: `n` of them is `n · 0.00155517384` coherent units.
    pub const PENNYWEIGHT_PER_SECOND: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(155517384, -11),
                zero: Exact::ZERO,
                symbol: "dwt/s",
                name: "pennyweight per second",
            }
        },
    > = SIUnit;

    /// Pounds per second (`lb/s`), a unit of `MassRate`: `n` of them is `n · 0.45359237` coherent units.
    pub const POUND_PER_SECOND: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "lb/s",
                name: "pounds per second",
            }
        },
    > = SIUnit;

    /// Pounds per minute (`lb/min`), a unit of `MassRate`: `n` of them is `n · 0.45359237 / 60.0` coherent units.
    pub const POUND_PER_MINUTE: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "lb/min",
                name: "pounds per minute",
            }
        },
    > = SIUnit;

    /// Pounds per hour (`lb/h`), a unit of `MassRate`: `n` of them is `n · 0.45359237 / 3600.0` coherent units.
    pub const POUND_PER_HOUR: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "lb/h",
                name: "pounds per hour",
            }
        },
    > = SIUnit;

    /// Pounds per day (`lb/d`), a unit of `MassRate`: `n` of them is `n · 0.45359237 / 86400.0` coherent units.
    pub const POUND_PER_DAY: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) / Exact::int(86400),
                zero: Exact::ZERO,
                symbol: "lb/d",
                name: "pounds per day",
            }
        },
    > = SIUnit;

    /// Troy pounds per second (`lb t/s`), a unit of `MassRate`: `n` of them is `n · 0.3732417216` coherent units.
    pub const POUND_TROY_PER_SECOND: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(3732417216, -10),
                zero: Exact::ZERO,
                symbol: "lb t/s",
                name: "troy pounds per second",
            }
        },
    > = SIUnit;

    /// Slugs per second (`slug/s`), a unit of `MassRate`: `n` of them is `n · 0.45359237 * 9.80665 / 0.3048` coherent units.
    pub const SLUG_PER_SECOND: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "slug/s",
                name: "slugs per second",
            }
        },
    > = SIUnit;

    /// Assay tons per second (`AT/s`), a unit of `MassRate`: `n` of them is `n · 0.175 / 6.0` coherent units.
    pub const TON_ASSAY_PER_SECOND: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(175, -3) / Exact::int(6),
                zero: Exact::ZERO,
                symbol: "AT/s",
                name: "assay tons per second",
            }
        },
    > = SIUnit;

    /// Long tons per second (`2240 lb/s`), a unit of `MassRate`: `n` of them is `n · 1016.0469088` coherent units.
    pub const TON_LONG_PER_SECOND: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(10160469088, -7),
                zero: Exact::ZERO,
                symbol: "2240 lb/s",
                name: "long tons per second",
            }
        },
    > = SIUnit;

    /// Short tons per second (`2000 lb/s`), a unit of `MassRate`: `n` of them is `n · 907.18474` coherent units.
    pub const TON_SHORT_PER_SECOND: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(90718474, -5),
                zero: Exact::ZERO,
                symbol: "2000 lb/s",
                name: "short tons per second",
            }
        },
    > = SIUnit;

    /// Short tons per hour (`2000 lb/h`), a unit of `MassRate`: `n` of them is `n · 907.18474 / 3600.0` coherent units.
    pub const TON_SHORT_PER_HOUR: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(90718474, -5) / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "2000 lb/h",
                name: "short tons per hour",
            }
        },
    > = SIUnit;

    /// Tons per second (`t/s`), a unit of `MassRate`: `n` of them is `n · 1000.0` coherent units.
    pub const TON_PER_SECOND: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "t/s",
                name: "tons per second",
            }
        },
    > = SIUnit;

    /// Tons per minute (`t/min`), a unit of `MassRate`: `n` of them is `n · 1000.0 / 60.0` coherent units.
    pub const TON_PER_MINUTE: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1000) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "t/min",
                name: "tons per minute",
            }
        },
    > = SIUnit;

    /// Tons per hour (`t/h`), a unit of `MassRate`: `n` of them is `n · 1000.0 / 3600.0` coherent units.
    pub const TON_PER_HOUR: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1000) / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "t/h",
                name: "tons per hour",
            }
        },
    > = SIUnit;

    /// Tons per day (`t/d`), a unit of `MassRate`: `n` of them is `n · 1000.0 / 86400.0` coherent units.
    pub const TON_PER_DAY: SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1000) / Exact::int(86400),
                zero: Exact::ZERO,
                symbol: "t/d",
                name: "tons per day",
            }
        },
    > = SIUnit;

    /// Cubic metres per second (`m³/s`), a unit of `VolumeRate`: `n` of them is `n · 1.0` coherent units.
    pub const CUBIC_METRE_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m³/s",
                name: "cubic metres per second",
            }
        },
    > = SIUnit;

    /// Cubic metres per minute (`m³/min`), a unit of `VolumeRate`: `n` of them is `n · 1.0 / 60.0` coherent units.
    pub const CUBIC_METRE_PER_MINUTE: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "m³/min",
                name: "cubic metres per minute",
            }
        },
    > = SIUnit;

    /// Cubic metres per hour (`m³/h`), a unit of `VolumeRate`: `n` of them is `n · 1.0 / 3600.0` coherent units.
    pub const CUBIC_METRE_PER_HOUR: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "m³/h",
                name: "cubic metres per hour",
            }
        },
    > = SIUnit;

    /// Litres per second (`L/s`), a unit of `VolumeRate`: `n` of them is `n · 0.001` coherent units.
    pub const LITRE_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "L/s",
                name: "litres per second",
            }
        },
    > = SIUnit;

    /// Litres per minute (`L/min`), a unit of `VolumeRate`: `n` of them is `n · 0.001 / 60.0` coherent units.
    pub const LITRE_PER_MINUTE: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "L/min",
                name: "litres per minute",
            }
        },
    > = SIUnit;

    /// Acre-feet per second (`ac · ft/s`), a unit of `VolumeRate`: `n` of them is `n · 43560.0 * (1200.0 / 3937.0) * (1200.0 / 3937.0) * (1200.0 / 3937.0)` coherent units.
    pub const ACRE_FOOT_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::int(43560)
                    * (Exact::int(1200) / Exact::int(3937))
                    * (Exact::int(1200) / Exact::int(3937))
                    * (Exact::int(1200) / Exact::int(3937)),
                zero: Exact::ZERO,
                symbol: "ac · ft/s",
                name: "acre-feet per second",
            }
        },
    > = SIUnit;

    /// Barrels per second (`bbl/s`), a unit of `VolumeRate`: `n` of them is `n · 0.158987294928` coherent units.
    pub const BARREL_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(158987294928, -12),
                zero: Exact::ZERO,
                symbol: "bbl/s",
                name: "barrels per second",
            }
        },
    > = SIUnit;

    /// Bushels per second (`bu/s`), a unit of `VolumeRate`: `n` of them is `n · 0.03523907016688` coherent units.
    pub const BUSHEL_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(3523907016688, -14),
                zero: Exact::ZERO,
                symbol: "bu/s",
                name: "bushels per second",
            }
        },
    > = SIUnit;

    /// Cords per second (`cords/s`), a unit of `VolumeRate`: `n` of them is `n · 128.0 * 0.3048 * 0.3048 * 0.3048` coherent units.
    pub const CORD_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::int(128)
                    * Exact::dec(3048, -4)
                    * Exact::dec(3048, -4)
                    * Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "cords/s",
                name: "cords per second",
            }
        },
    > = SIUnit;

    /// Cubic feet per second (`ft³/s`), a unit of `VolumeRate`: `n` of them is `n · 0.3048 * 0.3048 * 0.3048` coherent units.
    pub const CUBIC_FOOT_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "ft³/s",
                name: "cubic feet per second",
            }
        },
    > = SIUnit;

    /// Cubic feet per minute (`ft³/min`), a unit of `VolumeRate`: `n` of them is `n · 0.3048 * 0.3048 * 0.3048 / 60.0` coherent units.
    pub const CUBIC_FOOT_PER_MINUTE: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)
                    / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "ft³/min",
                name: "cubic feet per minute",
            }
        },
    > = SIUnit;

    /// Cubic inches per second (`in³/s`), a unit of `VolumeRate`: `n` of them is `n · 0.0254 * 0.0254 * 0.0254` coherent units.
    pub const CUBIC_INCH_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "in³/s",
                name: "cubic inches per second",
            }
        },
    > = SIUnit;

    /// Cubic inches per minute (`in³/min`), a unit of `VolumeRate`: `n` of them is `n · 0.0254 * 0.0254 * 0.0254 / 60.0` coherent units.
    pub const CUBIC_INCH_PER_MINUTE: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4)
                    / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "in³/min",
                name: "cubic inches per minute",
            }
        },
    > = SIUnit;

    /// Cubic miles per second (`mi³/s`), a unit of `VolumeRate`: `n` of them is `n · 1609.344 * 1609.344 * 1609.344` coherent units.
    pub const CUBIC_MILE_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1609344, -3) * Exact::dec(1609344, -3) * Exact::dec(1609344, -3),
                zero: Exact::ZERO,
                symbol: "mi³/s",
                name: "cubic miles per second",
            }
        },
    > = SIUnit;

    /// Cubic yards per second (`yd³/s`), a unit of `VolumeRate`: `n` of them is `n · 0.9144 * 0.9144 * 0.9144` coherent units.
    pub const CUBIC_YARD_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4),
                zero: Exact::ZERO,
                symbol: "yd³/s",
                name: "cubic yards per second",
            }
        },
    > = SIUnit;

    /// Cubic yards per minute (`yd³/min`), a unit of `VolumeRate`: `n` of them is `n · 0.9144 * 0.9144 * 0.9144 / 60.0` coherent units.
    pub const CUBIC_YARD_PER_MINUTE: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4)
                    / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "yd³/min",
                name: "cubic yards per minute",
            }
        },
    > = SIUnit;

    /// Cups per second (`cup/s`), a unit of `VolumeRate`: `n` of them is `n · 0.0002365882365` coherent units.
    pub const CUP_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(2365882365, -13),
                zero: Exact::ZERO,
                symbol: "cup/s",
                name: "cups per second",
            }
        },
    > = SIUnit;

    /// Fluid ounces per second (`fl oz/s`), a unit of `VolumeRate`: `n` of them is `n · 2.95735295625e-05` coherent units.
    pub const FLUID_OUNCE_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(295735295625, -16),
                zero: Exact::ZERO,
                symbol: "fl oz/s",
                name: "fluid ounces per second",
            }
        },
    > = SIUnit;

    /// Imperial fluid ounces per second (`fl oz (UK)/s`), a unit of `VolumeRate`: `n` of them is `n · 2.84130625e-05` coherent units.
    pub const FLUID_OUNCE_IMPERIAL_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(284130625, -13),
                zero: Exact::ZERO,
                symbol: "fl oz (UK)/s",
                name: "Imperial fluid ounces per second",
            }
        },
    > = SIUnit;

    /// Imperial gallons per second (`gal (UK)/s`), a unit of `VolumeRate`: `n` of them is `n · 0.00454609` coherent units.
    pub const GALLON_IMPERIAL_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(454609, -8),
                zero: Exact::ZERO,
                symbol: "gal (UK)/s",
                name: "Imperial gallons per second",
            }
        },
    > = SIUnit;

    /// Gallons per second (`gal/s`), a unit of `VolumeRate`: `n` of them is `n · 0.003785411784` coherent units.
    pub const GALLON_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(3785411784, -12),
                zero: Exact::ZERO,
                symbol: "gal/s",
                name: "gallons per second",
            }
        },
    > = SIUnit;

    /// Gallons per minute (`gal/min`), a unit of `VolumeRate`: `n` of them is `n · 0.003785411784 / 60.0` coherent units.
    pub const GALLON_PER_MINUTE: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(3785411784, -12) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "gal/min",
                name: "gallons per minute",
            }
        },
    > = SIUnit;

    /// Gallons per day (`gal/d`), a unit of `VolumeRate`: `n` of them is `n · 0.003785411784 / 86400.0` coherent units.
    pub const GALLON_PER_DAY: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(3785411784, -12) / Exact::int(86400),
                zero: Exact::ZERO,
                symbol: "gal/d",
                name: "gallons per day",
            }
        },
    > = SIUnit;

    /// Imperial gills per second (`gi (UK)/s`), a unit of `VolumeRate`: `n` of them is `n · 0.0001420653125` coherent units.
    pub const GILL_IMPERIAL_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1420653125, -13),
                zero: Exact::ZERO,
                symbol: "gi (UK)/s",
                name: "Imperial gills per second",
            }
        },
    > = SIUnit;

    /// Gills per second (`gi/s`), a unit of `VolumeRate`: `n` of them is `n · 0.00011829411825` coherent units.
    pub const GILL_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(11829411825, -14),
                zero: Exact::ZERO,
                symbol: "gi/s",
                name: "gills per second",
            }
        },
    > = SIUnit;

    /// Pecks per second (`pk/s`), a unit of `VolumeRate`: `n` of them is `n · 0.00880976754172` coherent units.
    pub const PECK_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(880976754172, -14),
                zero: Exact::ZERO,
                symbol: "pk/s",
                name: "pecks per second",
            }
        },
    > = SIUnit;

    /// Dry pints per second (`dry pt/s`), a unit of `VolumeRate`: `n` of them is `n · 0.0005506104713575` coherent units.
    pub const PINT_DRY_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(5506104713575, -16),
                zero: Exact::ZERO,
                symbol: "dry pt/s",
                name: "dry pints per second",
            }
        },
    > = SIUnit;

    /// Liquid pints per second (`liq pt/s`), a unit of `VolumeRate`: `n` of them is `n · 0.000473176473` coherent units.
    pub const PINT_LIQUID_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(473176473, -12),
                zero: Exact::ZERO,
                symbol: "liq pt/s",
                name: "liquid pints per second",
            }
        },
    > = SIUnit;

    /// Dry quarts per second (`dry qt/s`), a unit of `VolumeRate`: `n` of them is `n · 0.001101220942715` coherent units.
    pub const QUART_DRY_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1101220942715, -15),
                zero: Exact::ZERO,
                symbol: "dry qt/s",
                name: "dry quarts per second",
            }
        },
    > = SIUnit;

    /// Liquid quarts per second (`liq qt/s`), a unit of `VolumeRate`: `n` of them is `n · 0.000946352946` coherent units.
    pub const QUART_LIQUID_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(946352946, -12),
                zero: Exact::ZERO,
                symbol: "liq qt/s",
                name: "liquid quarts per second",
            }
        },
    > = SIUnit;

    /// Steres per second (`st/s`), a unit of `VolumeRate`: `n` of them is `n · 1.0` coherent units.
    pub const STERE_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "st/s",
                name: "steres per second",
            }
        },
    > = SIUnit;

    /// Tablespoons per second (`tbsp/s`), a unit of `VolumeRate`: `n` of them is `n · 1.478676478125e-05` coherent units.
    pub const TABLESPOON_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1478676478125, -17),
                zero: Exact::ZERO,
                symbol: "tbsp/s",
                name: "tablespoons per second",
            }
        },
    > = SIUnit;

    /// Teaspoons per second (`tsp/s`), a unit of `VolumeRate`: `n` of them is `n · 4.92892159375e-06` coherent units.
    pub const TEASPOON_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(492892159375, -17),
                zero: Exact::ZERO,
                symbol: "tsp/s",
                name: "teaspoons per second",
            }
        },
    > = SIUnit;

    /// Register tons per second (`RT/s`), a unit of `VolumeRate`: `n` of them is `n · 100.0 * 0.3048 * 0.3048 * 0.3048` coherent units.
    pub const REGISTER_TON_PER_SECOND: SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::int(100)
                    * Exact::dec(3048, -4)
                    * Exact::dec(3048, -4)
                    * Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "RT/s",
                name: "register tons per second",
            }
        },
    > = SIUnit;

    /// Kilograms per square metre second (`kg/(m² · s)`), a unit of `MassFlux`: `n` of them is `n · 1.0` coherent units.
    pub const KILOGRAM_PER_SQUARE_METRE_SECOND: SIUnit<
        MASS_FLUX,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg/(m² · s)",
                name: "kilograms per square metre second",
            }
        },
    > = SIUnit;

    /// Kilograms per joule (`kg/J`), a unit of `MassPerEnergy`: `n` of them is `n · 1.0` coherent units.
    pub const KILOGRAM_PER_JOULE: SIUnit<
        MASS_PER_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg/J",
                name: "kilograms per joule",
            }
        },
    > = SIUnit;

    /// Grams per joule (`g/J`), a unit of `MassPerEnergy`: `n` of them is `n · 0.001` coherent units.
    pub const GRAM_PER_JOULE: SIUnit<
        MASS_PER_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "g/J",
                name: "grams per joule",
            }
        },
    > = SIUnit;

    /// Pounds per joule (`lb/J`), a unit of `MassPerEnergy`: `n` of them is `n · 0.45359237` coherent units.
    pub const POUND_PER_JOULE: SIUnit<
        MASS_PER_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "lb/J",
                name: "pounds per joule",
            }
        },
    > = SIUnit;

    /// Pounds per watt hour (`lb/Wh`), a unit of `MassPerEnergy`: `n` of them is `n · 0.45359237 / 3600.0` coherent units.
    pub const POUND_PER_WATT_HOUR: SIUnit<
        MASS_PER_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "lb/Wh",
                name: "pounds per watt hour",
            }
        },
    > = SIUnit;

    /// Grams per watt hour (`g/Wh`), a unit of `MassPerEnergy`: `n` of them is `n · 0.001 / 3600.0` coherent units.
    pub const GRAM_PER_WATT_HOUR: SIUnit<
        MASS_PER_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3) / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "g/Wh",
                name: "grams per watt hour",
            }
        },
    > = SIUnit;

    /// Square metres per second (`m²/s`), a unit of `DiffusionCoefficient`: `n` of them is `n · 1.0` coherent units.
    pub const DIFFUSION_COEFFICIENT_SQUARE_METRE_PER_SECOND: SIUnit<
        DIFFUSION_COEFFICIENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m²/s",
                name: "square metres per second",
            }
        },
    > = SIUnit;

    /// Stokes (`St`), a unit of `DiffusionCoefficient`: `n` of them is `n · 0.0001` coherent units.
    pub const DIFFUSION_COEFFICIENT_STOKES: SIUnit<
        DIFFUSION_COEFFICIENT,
        {
            SIUnitConsts {
                size: Exact::dec(1, -4),
                zero: Exact::ZERO,
                symbol: "St",
                name: "Stokes",
            }
        },
    > = SIUnit;
}
