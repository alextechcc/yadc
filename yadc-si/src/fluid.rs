use crate::{ISQ, SIQuantity};
use yadc::Dimension;

/// The dimension of `DynamicViscosity`.
pub const DYNAMIC_VISCOSITY: ISQ = ISQ {
    m: -1,
    kg: 1,
    s: -1,
    ..ISQ::DIMENSIONLESS
};
pub type DynamicViscosity<N> = SIQuantity<DYNAMIC_VISCOSITY, N>;

/// The dimension of `KinematicViscosity`.
pub const KINEMATIC_VISCOSITY: ISQ = ISQ {
    m: 2,
    s: -1,
    ..ISQ::DIMENSIONLESS
};
pub type KinematicViscosity<N> = SIQuantity<KINEMATIC_VISCOSITY, N>;

/// The dimension of `SurfaceTension`.
pub const SURFACE_TENSION: ISQ = ISQ {
    kg: 1,
    s: -2,
    ..ISQ::DIMENSIONLESS
};
pub type SurfaceTension<N> = SIQuantity<SURFACE_TENSION, N>;

/// The dimension of `CompressibilityCoefficient`.
pub const COMPRESSIBILITY_COEFFICIENT: ISQ = ISQ {
    m: 1,
    kg: -1,
    s: 2,
    ..ISQ::DIMENSIONLESS
};
pub type CompressibilityCoefficient<N> = SIQuantity<COMPRESSIBILITY_COEFFICIENT, N>;

/// The dimension of `MassRate`.
pub const MASS_RATE: ISQ = ISQ {
    kg: 1,
    s: -1,
    ..ISQ::DIMENSIONLESS
};
pub type MassRate<N> = SIQuantity<MASS_RATE, N>;

/// The dimension of `VolumeRate`.
pub const VOLUME_RATE: ISQ = ISQ {
    m: 3,
    s: -1,
    ..ISQ::DIMENSIONLESS
};
pub type VolumeRate<N> = SIQuantity<VOLUME_RATE, N>;

/// The dimension of `MassFlux`.
pub const MASS_FLUX: ISQ = ISQ {
    m: -2,
    kg: 1,
    s: -1,
    ..ISQ::DIMENSIONLESS
};
pub type MassFlux<N> = SIQuantity<MASS_FLUX, N>;

/// The dimension of `MassPerEnergy`.
pub const MASS_PER_ENERGY: ISQ = ISQ {
    m: -2,
    s: 2,
    ..ISQ::DIMENSIONLESS
};
pub type MassPerEnergy<N> = SIQuantity<MASS_PER_ENERGY, N>;

/// The dimension of `DiffusionCoefficient`.
pub const DIFFUSION_COEFFICIENT: ISQ = ISQ {
    m: 2,
    s: -1,
    ..ISQ::DIMENSIONLESS
};
pub type DiffusionCoefficient<N> = SIQuantity<DIFFUSION_COEFFICIENT, N>;

/// The units of this module's quantities: a type each, its size and zero
/// in its quantity's coherent unit, exact (`Mile {}.quantity(3.0_f64)`).
pub mod units {
    use super::*;
    use crate::{Exact, SIUnit, SIUnitConsts};

    /// Pascal seconds (`Pa · s`), a unit of `DynamicViscosity`: `n` of them is `n · 1.0` coherent units.
    pub type PascalSecond = SIUnit<
        DYNAMIC_VISCOSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Pa · s",
                name: "pascal seconds",
            }
        },
    >;

    /// Poises (`P`), a unit of `DynamicViscosity`: `n` of them is `n · 0.1` coherent units.
    pub type Poise = SIUnit<
        DYNAMIC_VISCOSITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -1),
                zero: Exact::ZERO,
                symbol: "P",
                name: "poises",
            }
        },
    >;

    /// Pounds per foot second (`lb/(ft · s)`), a unit of `DynamicViscosity`: `n` of them is `n · 0.45359237 / 0.3048` coherent units.
    pub type PoundPerFootSecond = SIUnit<
        DYNAMIC_VISCOSITY,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) / Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "lb/(ft · s)",
                name: "pounds per foot second",
            }
        },
    >;

    /// Slugs per foot second (`slug/(ft · s)`), a unit of `DynamicViscosity`: `n` of them is `n · (0.45359237 * 9.80665 / 0.3048) / 0.3048` coherent units.
    pub type SlugPerFootSecond = SIUnit<
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
    >;

    /// Grams per centimetre second (`g/(cm · s)`), a unit of `DynamicViscosity`: `n` of them is `n · 0.1` coherent units.
    pub type GramPerCentimetreSecond = SIUnit<
        DYNAMIC_VISCOSITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -1),
                zero: Exact::ZERO,
                symbol: "g/(cm · s)",
                name: "grams per centimetre second",
            }
        },
    >;

    /// Square metres per second (`m²/s`), a unit of `KinematicViscosity`: `n` of them is `n · 1.0` coherent units.
    pub type KinematicViscositySquareMetrePerSecond = SIUnit<
        KINEMATIC_VISCOSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m²/s",
                name: "square metres per second",
            }
        },
    >;

    /// Stokes (`St`), a unit of `KinematicViscosity`: `n` of them is `n · 0.0001` coherent units.
    pub type KinematicViscosityStokes = SIUnit<
        KINEMATIC_VISCOSITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -4),
                zero: Exact::ZERO,
                symbol: "St",
                name: "stokes",
            }
        },
    >;

    /// Newtons per metre (`N/m`), a unit of `SurfaceTension`: `n` of them is `n · 1.0` coherent units.
    pub type NewtonPerMetre = SIUnit<
        SURFACE_TENSION,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "N/m",
                name: "newtons per metre",
            }
        },
    >;

    /// Kilograms-force per metre (`kgf/m`), a unit of `SurfaceTension`: `n` of them is `n · 9.80665` coherent units.
    pub type KilogramForcePerMetre = SIUnit<
        SURFACE_TENSION,
        {
            SIUnitConsts {
                size: Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "kgf/m",
                name: "kilograms-force per metre",
            }
        },
    >;

    /// Ounces-force per inch (`ozf/in`), a unit of `SurfaceTension`: `n` of them is `n · (0.45359237 * 9.80665 / 16.0) / 0.0254` coherent units.
    pub type OunceForcePerInch = SIUnit<
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
    >;

    /// Poundals per inch (`pdl/in`), a unit of `SurfaceTension`: `n` of them is `n · (0.45359237 * 0.3048) / 0.0254` coherent units.
    pub type PoundalPerInch = SIUnit<
        SURFACE_TENSION,
        {
            SIUnitConsts {
                size: (Exact::dec(45359237, -8) * Exact::dec(3048, -4)) / Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "pdl/in",
                name: "poundals per inch",
            }
        },
    >;

    /// Pounds-force per inch (`lbf/in`), a unit of `SurfaceTension`: `n` of them is `n · (0.45359237 * 9.80665) / 0.0254` coherent units.
    pub type PoundForcePerInch = SIUnit<
        SURFACE_TENSION,
        {
            SIUnitConsts {
                size: (Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "lbf/in",
                name: "pounds-force per inch",
            }
        },
    >;

    /// Joules per square metre (`J/m²`), a unit of `SurfaceTension`: `n` of them is `n · 1.0` coherent units.
    pub type JoulePerSquareMetre = SIUnit<
        SURFACE_TENSION,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/m²",
                name: "joules per square metre",
            }
        },
    >;

    /// Per pascal (`Pa⁻¹`), a unit of `CompressibilityCoefficient`: `n` of them is `n · 1.0` coherent units.
    pub type PerPascal = SIUnit<
        COMPRESSIBILITY_COEFFICIENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Pa⁻¹",
                name: "per pascal",
            }
        },
    >;

    /// Per atmosphere (`atm⁻¹`), a unit of `CompressibilityCoefficient`: `n` of them is `n · 1.0 / 101325.0` coherent units.
    pub type PerAtmosphere = SIUnit<
        COMPRESSIBILITY_COEFFICIENT,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(101325),
                zero: Exact::ZERO,
                symbol: "atm⁻¹",
                name: "per atmosphere",
            }
        },
    >;

    /// Per atmosphere (technical) (`at⁻¹`), a unit of `CompressibilityCoefficient`: `n` of them is `n · 1.0 / 98066.5` coherent units.
    pub type PerAtmosphereTechnical = SIUnit<
        COMPRESSIBILITY_COEFFICIENT,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(980665, -1),
                zero: Exact::ZERO,
                symbol: "at⁻¹",
                name: "per atmosphere (technical)",
            }
        },
    >;

    /// Per bar (`bar⁻¹`), a unit of `CompressibilityCoefficient`: `n` of them is `n · 1e-05` coherent units.
    pub type PerBar = SIUnit<
        COMPRESSIBILITY_COEFFICIENT,
        {
            SIUnitConsts {
                size: Exact::dec(1, -5),
                zero: Exact::ZERO,
                symbol: "bar⁻¹",
                name: "per bar",
            }
        },
    >;

    /// Kilograms per second (`kg/s`), a unit of `MassRate`: `n` of them is `n · 1.0` coherent units.
    pub type KilogramPerSecond = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg/s",
                name: "kilograms per second",
            }
        },
    >;

    /// Grams per second (`g/s`), a unit of `MassRate`: `n` of them is `n · 0.001` coherent units.
    pub type GramPerSecond = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "g/s",
                name: "grams per second",
            }
        },
    >;

    /// Grams per minute (`g/min`), a unit of `MassRate`: `n` of them is `n · 0.001 / 60.0` coherent units.
    pub type GramPerMinute = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "g/min",
                name: "grams per minute",
            }
        },
    >;

    /// Grams per hour (`g/h`), a unit of `MassRate`: `n` of them is `n · 0.001 / 3600.0` coherent units.
    pub type GramPerHour = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3) / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "g/h",
                name: "grams per hour",
            }
        },
    >;

    /// Grams per day (`g/d`), a unit of `MassRate`: `n` of them is `n · 0.001 / 86400.0` coherent units.
    pub type GramPerDay = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3) / Exact::int(86400),
                zero: Exact::ZERO,
                symbol: "g/d",
                name: "grams per day",
            }
        },
    >;

    /// Carats per second (`ct/s`), a unit of `MassRate`: `n` of them is `n · 0.0002` coherent units.
    pub type CaratPerSecond = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(2, -4),
                zero: Exact::ZERO,
                symbol: "ct/s",
                name: "carats per second",
            }
        },
    >;

    /// Grains per second (`gr/s`), a unit of `MassRate`: `n` of them is `n · 6.479891e-05` coherent units.
    pub type GrainPerSecond = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(6479891, -11),
                zero: Exact::ZERO,
                symbol: "gr/s",
                name: "grains per second",
            }
        },
    >;

    /// Hundredweight (long) per second (`cwt long/s`), a unit of `MassRate`: `n` of them is `n · 50.80234544` coherent units.
    pub type HundredweightLongPerSecond = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(5080234544, -8),
                zero: Exact::ZERO,
                symbol: "cwt long/s",
                name: "hundredweight (long) per second",
            }
        },
    >;

    /// Hundredweight (short) per second (`cwt short/s`), a unit of `MassRate`: `n` of them is `n · 45.359237` coherent units.
    pub type HundredweightShortPerSecond = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -6),
                zero: Exact::ZERO,
                symbol: "cwt short/s",
                name: "hundredweight (short) per second",
            }
        },
    >;

    /// Ounces per second (`oz/s`), a unit of `MassRate`: `n` of them is `n · 0.028349523125` coherent units.
    pub type OuncePerSecond = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(28349523125, -12),
                zero: Exact::ZERO,
                symbol: "oz/s",
                name: "ounces per second",
            }
        },
    >;

    /// Troy ounces per second (`oz t/s`), a unit of `MassRate`: `n` of them is `n · 0.0311034768` coherent units.
    pub type OunceTroyPerSecond = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(311034768, -10),
                zero: Exact::ZERO,
                symbol: "oz t/s",
                name: "troy ounces per second",
            }
        },
    >;

    /// Pennyweight per second (`dwt/s`), a unit of `MassRate`: `n` of them is `n · 0.00155517384` coherent units.
    pub type PennyweightPerSecond = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(155517384, -11),
                zero: Exact::ZERO,
                symbol: "dwt/s",
                name: "pennyweight per second",
            }
        },
    >;

    /// Pounds per second (`lb/s`), a unit of `MassRate`: `n` of them is `n · 0.45359237` coherent units.
    pub type PoundPerSecond = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "lb/s",
                name: "pounds per second",
            }
        },
    >;

    /// Pounds per minute (`lb/min`), a unit of `MassRate`: `n` of them is `n · 0.45359237 / 60.0` coherent units.
    pub type PoundPerMinute = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "lb/min",
                name: "pounds per minute",
            }
        },
    >;

    /// Pounds per hour (`lb/h`), a unit of `MassRate`: `n` of them is `n · 0.45359237 / 3600.0` coherent units.
    pub type PoundPerHour = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "lb/h",
                name: "pounds per hour",
            }
        },
    >;

    /// Pounds per day (`lb/d`), a unit of `MassRate`: `n` of them is `n · 0.45359237 / 86400.0` coherent units.
    pub type PoundPerDay = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) / Exact::int(86400),
                zero: Exact::ZERO,
                symbol: "lb/d",
                name: "pounds per day",
            }
        },
    >;

    /// Troy pounds per second (`lb t/s`), a unit of `MassRate`: `n` of them is `n · 0.3732417216` coherent units.
    pub type PoundTroyPerSecond = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(3732417216, -10),
                zero: Exact::ZERO,
                symbol: "lb t/s",
                name: "troy pounds per second",
            }
        },
    >;

    /// Slugs per second (`slug/s`), a unit of `MassRate`: `n` of them is `n · 0.45359237 * 9.80665 / 0.3048` coherent units.
    pub type SlugPerSecond = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "slug/s",
                name: "slugs per second",
            }
        },
    >;

    /// Assay tons per second (`AT/s`), a unit of `MassRate`: `n` of them is `n · 0.175 / 6.0` coherent units.
    pub type TonAssayPerSecond = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(175, -3) / Exact::int(6),
                zero: Exact::ZERO,
                symbol: "AT/s",
                name: "assay tons per second",
            }
        },
    >;

    /// Long tons per second (`2240 lb/s`), a unit of `MassRate`: `n` of them is `n · 1016.0469088` coherent units.
    pub type TonLongPerSecond = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(10160469088, -7),
                zero: Exact::ZERO,
                symbol: "2240 lb/s",
                name: "long tons per second",
            }
        },
    >;

    /// Short tons per second (`2000 lb/s`), a unit of `MassRate`: `n` of them is `n · 907.18474` coherent units.
    pub type TonShortPerSecond = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(90718474, -5),
                zero: Exact::ZERO,
                symbol: "2000 lb/s",
                name: "short tons per second",
            }
        },
    >;

    /// Short tons per hour (`2000 lb/h`), a unit of `MassRate`: `n` of them is `n · 907.18474 / 3600.0` coherent units.
    pub type TonShortPerHour = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(90718474, -5) / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "2000 lb/h",
                name: "short tons per hour",
            }
        },
    >;

    /// Tons per second (`t/s`), a unit of `MassRate`: `n` of them is `n · 1000.0` coherent units.
    pub type TonPerSecond = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "t/s",
                name: "tons per second",
            }
        },
    >;

    /// Tons per minute (`t/min`), a unit of `MassRate`: `n` of them is `n · 1000.0 / 60.0` coherent units.
    pub type TonPerMinute = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1000) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "t/min",
                name: "tons per minute",
            }
        },
    >;

    /// Tons per hour (`t/h`), a unit of `MassRate`: `n` of them is `n · 1000.0 / 3600.0` coherent units.
    pub type TonPerHour = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1000) / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "t/h",
                name: "tons per hour",
            }
        },
    >;

    /// Tons per day (`t/d`), a unit of `MassRate`: `n` of them is `n · 1000.0 / 86400.0` coherent units.
    pub type TonPerDay = SIUnit<
        MASS_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1000) / Exact::int(86400),
                zero: Exact::ZERO,
                symbol: "t/d",
                name: "tons per day",
            }
        },
    >;

    /// Cubic metres per second (`m³/s`), a unit of `VolumeRate`: `n` of them is `n · 1.0` coherent units.
    pub type CubicMetrePerSecond = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m³/s",
                name: "cubic metres per second",
            }
        },
    >;

    /// Cubic metres per minute (`m³/min`), a unit of `VolumeRate`: `n` of them is `n · 1.0 / 60.0` coherent units.
    pub type CubicMetrePerMinute = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "m³/min",
                name: "cubic metres per minute",
            }
        },
    >;

    /// Cubic metres per hour (`m³/h`), a unit of `VolumeRate`: `n` of them is `n · 1.0 / 3600.0` coherent units.
    pub type CubicMetrePerHour = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "m³/h",
                name: "cubic metres per hour",
            }
        },
    >;

    /// Litres per second (`L/s`), a unit of `VolumeRate`: `n` of them is `n · 0.001` coherent units.
    pub type LitrePerSecond = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "L/s",
                name: "litres per second",
            }
        },
    >;

    /// Litres per minute (`L/min`), a unit of `VolumeRate`: `n` of them is `n · 0.001 / 60.0` coherent units.
    pub type LitrePerMinute = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "L/min",
                name: "litres per minute",
            }
        },
    >;

    /// Acre-feet per second (`ac · ft/s`), a unit of `VolumeRate`: `n` of them is `n · 43560.0 * (1200.0 / 3937.0) * (1200.0 / 3937.0) * (1200.0 / 3937.0)` coherent units.
    pub type AcreFootPerSecond = SIUnit<
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
    >;

    /// Barrels per second (`bbl/s`), a unit of `VolumeRate`: `n` of them is `n · 0.158987294928` coherent units.
    pub type BarrelPerSecond = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(158987294928, -12),
                zero: Exact::ZERO,
                symbol: "bbl/s",
                name: "barrels per second",
            }
        },
    >;

    /// Bushels per second (`bu/s`), a unit of `VolumeRate`: `n` of them is `n · 0.03523907016688` coherent units.
    pub type BushelPerSecond = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(3523907016688, -14),
                zero: Exact::ZERO,
                symbol: "bu/s",
                name: "bushels per second",
            }
        },
    >;

    /// Cords per second (`cords/s`), a unit of `VolumeRate`: `n` of them is `n · 128.0 * 0.3048 * 0.3048 * 0.3048` coherent units.
    pub type CordPerSecond = SIUnit<
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
    >;

    /// Cubic feet per second (`ft³/s`), a unit of `VolumeRate`: `n` of them is `n · 0.3048 * 0.3048 * 0.3048` coherent units.
    pub type CubicFootPerSecond = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "ft³/s",
                name: "cubic feet per second",
            }
        },
    >;

    /// Cubic feet per minute (`ft³/min`), a unit of `VolumeRate`: `n` of them is `n · 0.3048 * 0.3048 * 0.3048 / 60.0` coherent units.
    pub type CubicFootPerMinute = SIUnit<
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
    >;

    /// Cubic inches per second (`in³/s`), a unit of `VolumeRate`: `n` of them is `n · 0.0254 * 0.0254 * 0.0254` coherent units.
    pub type CubicInchPerSecond = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "in³/s",
                name: "cubic inches per second",
            }
        },
    >;

    /// Cubic inches per minute (`in³/min`), a unit of `VolumeRate`: `n` of them is `n · 0.0254 * 0.0254 * 0.0254 / 60.0` coherent units.
    pub type CubicInchPerMinute = SIUnit<
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
    >;

    /// Cubic miles per second (`mi³/s`), a unit of `VolumeRate`: `n` of them is `n · 1609.344 * 1609.344 * 1609.344` coherent units.
    pub type CubicMilePerSecond = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1609344, -3) * Exact::dec(1609344, -3) * Exact::dec(1609344, -3),
                zero: Exact::ZERO,
                symbol: "mi³/s",
                name: "cubic miles per second",
            }
        },
    >;

    /// Cubic yards per second (`yd³/s`), a unit of `VolumeRate`: `n` of them is `n · 0.9144 * 0.9144 * 0.9144` coherent units.
    pub type CubicYardPerSecond = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4),
                zero: Exact::ZERO,
                symbol: "yd³/s",
                name: "cubic yards per second",
            }
        },
    >;

    /// Cubic yards per minute (`yd³/min`), a unit of `VolumeRate`: `n` of them is `n · 0.9144 * 0.9144 * 0.9144 / 60.0` coherent units.
    pub type CubicYardPerMinute = SIUnit<
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
    >;

    /// Cups per second (`cup/s`), a unit of `VolumeRate`: `n` of them is `n · 0.0002365882365` coherent units.
    pub type CupPerSecond = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(2365882365, -13),
                zero: Exact::ZERO,
                symbol: "cup/s",
                name: "cups per second",
            }
        },
    >;

    /// Fluid ounces per second (`fl oz/s`), a unit of `VolumeRate`: `n` of them is `n · 2.95735295625e-05` coherent units.
    pub type FluidOuncePerSecond = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(295735295625, -16),
                zero: Exact::ZERO,
                symbol: "fl oz/s",
                name: "fluid ounces per second",
            }
        },
    >;

    /// Imperial fluid ounces per second (`fl oz (UK)/s`), a unit of `VolumeRate`: `n` of them is `n · 2.84130625e-05` coherent units.
    pub type FluidOunceImperialPerSecond = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(284130625, -13),
                zero: Exact::ZERO,
                symbol: "fl oz (UK)/s",
                name: "Imperial fluid ounces per second",
            }
        },
    >;

    /// Imperial gallons per second (`gal (UK)/s`), a unit of `VolumeRate`: `n` of them is `n · 0.00454609` coherent units.
    pub type GallonImperialPerSecond = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(454609, -8),
                zero: Exact::ZERO,
                symbol: "gal (UK)/s",
                name: "Imperial gallons per second",
            }
        },
    >;

    /// Gallons per second (`gal/s`), a unit of `VolumeRate`: `n` of them is `n · 0.003785411784` coherent units.
    pub type GallonPerSecond = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(3785411784, -12),
                zero: Exact::ZERO,
                symbol: "gal/s",
                name: "gallons per second",
            }
        },
    >;

    /// Gallons per minute (`gal/min`), a unit of `VolumeRate`: `n` of them is `n · 0.003785411784 / 60.0` coherent units.
    pub type GallonPerMinute = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(3785411784, -12) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "gal/min",
                name: "gallons per minute",
            }
        },
    >;

    /// Gallons per day (`gal/d`), a unit of `VolumeRate`: `n` of them is `n · 0.003785411784 / 86400.0` coherent units.
    pub type GallonPerDay = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(3785411784, -12) / Exact::int(86400),
                zero: Exact::ZERO,
                symbol: "gal/d",
                name: "gallons per day",
            }
        },
    >;

    /// Imperial gills per second (`gi (UK)/s`), a unit of `VolumeRate`: `n` of them is `n · 0.0001420653125` coherent units.
    pub type GillImperialPerSecond = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1420653125, -13),
                zero: Exact::ZERO,
                symbol: "gi (UK)/s",
                name: "Imperial gills per second",
            }
        },
    >;

    /// Gills per second (`gi/s`), a unit of `VolumeRate`: `n` of them is `n · 0.00011829411825` coherent units.
    pub type GillPerSecond = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(11829411825, -14),
                zero: Exact::ZERO,
                symbol: "gi/s",
                name: "gills per second",
            }
        },
    >;

    /// Pecks per second (`pk/s`), a unit of `VolumeRate`: `n` of them is `n · 0.00880976754172` coherent units.
    pub type PeckPerSecond = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(880976754172, -14),
                zero: Exact::ZERO,
                symbol: "pk/s",
                name: "pecks per second",
            }
        },
    >;

    /// Dry pints per second (`dry pt/s`), a unit of `VolumeRate`: `n` of them is `n · 0.0005506104713575` coherent units.
    pub type PintDryPerSecond = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(5506104713575, -16),
                zero: Exact::ZERO,
                symbol: "dry pt/s",
                name: "dry pints per second",
            }
        },
    >;

    /// Liquid pints per second (`liq pt/s`), a unit of `VolumeRate`: `n` of them is `n · 0.000473176473` coherent units.
    pub type PintLiquidPerSecond = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(473176473, -12),
                zero: Exact::ZERO,
                symbol: "liq pt/s",
                name: "liquid pints per second",
            }
        },
    >;

    /// Dry quarts per second (`dry qt/s`), a unit of `VolumeRate`: `n` of them is `n · 0.001101220942715` coherent units.
    pub type QuartDryPerSecond = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1101220942715, -15),
                zero: Exact::ZERO,
                symbol: "dry qt/s",
                name: "dry quarts per second",
            }
        },
    >;

    /// Liquid quarts per second (`liq qt/s`), a unit of `VolumeRate`: `n` of them is `n · 0.000946352946` coherent units.
    pub type QuartLiquidPerSecond = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(946352946, -12),
                zero: Exact::ZERO,
                symbol: "liq qt/s",
                name: "liquid quarts per second",
            }
        },
    >;

    /// Steres per second (`st/s`), a unit of `VolumeRate`: `n` of them is `n · 1.0` coherent units.
    pub type SterePerSecond = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "st/s",
                name: "steres per second",
            }
        },
    >;

    /// Tablespoons per second (`tbsp/s`), a unit of `VolumeRate`: `n` of them is `n · 1.478676478125e-05` coherent units.
    pub type TablespoonPerSecond = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1478676478125, -17),
                zero: Exact::ZERO,
                symbol: "tbsp/s",
                name: "tablespoons per second",
            }
        },
    >;

    /// Teaspoons per second (`tsp/s`), a unit of `VolumeRate`: `n` of them is `n · 4.92892159375e-06` coherent units.
    pub type TeaspoonPerSecond = SIUnit<
        VOLUME_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(492892159375, -17),
                zero: Exact::ZERO,
                symbol: "tsp/s",
                name: "teaspoons per second",
            }
        },
    >;

    /// Register tons per second (`RT/s`), a unit of `VolumeRate`: `n` of them is `n · 100.0 * 0.3048 * 0.3048 * 0.3048` coherent units.
    pub type RegisterTonPerSecond = SIUnit<
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
    >;

    /// Kilograms per square metre second (`kg/(m² · s)`), a unit of `MassFlux`: `n` of them is `n · 1.0` coherent units.
    pub type KilogramPerSquareMetreSecond = SIUnit<
        MASS_FLUX,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg/(m² · s)",
                name: "kilograms per square metre second",
            }
        },
    >;

    /// Kilograms per joule (`kg/J`), a unit of `MassPerEnergy`: `n` of them is `n · 1.0` coherent units.
    pub type KilogramPerJoule = SIUnit<
        MASS_PER_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg/J",
                name: "kilograms per joule",
            }
        },
    >;

    /// Grams per joule (`g/J`), a unit of `MassPerEnergy`: `n` of them is `n · 0.001` coherent units.
    pub type GramPerJoule = SIUnit<
        MASS_PER_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "g/J",
                name: "grams per joule",
            }
        },
    >;

    /// Pounds per joule (`lb/J`), a unit of `MassPerEnergy`: `n` of them is `n · 0.45359237` coherent units.
    pub type PoundPerJoule = SIUnit<
        MASS_PER_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "lb/J",
                name: "pounds per joule",
            }
        },
    >;

    /// Pounds per watt hour (`lb/Wh`), a unit of `MassPerEnergy`: `n` of them is `n · 0.45359237 / 3600.0` coherent units.
    pub type PoundPerWattHour = SIUnit<
        MASS_PER_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "lb/Wh",
                name: "pounds per watt hour",
            }
        },
    >;

    /// Grams per watt hour (`g/Wh`), a unit of `MassPerEnergy`: `n` of them is `n · 0.001 / 3600.0` coherent units.
    pub type GramPerWattHour = SIUnit<
        MASS_PER_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3) / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "g/Wh",
                name: "grams per watt hour",
            }
        },
    >;

    /// Square metres per second (`m²/s`), a unit of `DiffusionCoefficient`: `n` of them is `n · 1.0` coherent units.
    pub type DiffusionCoefficientSquareMetrePerSecond = SIUnit<
        DIFFUSION_COEFFICIENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m²/s",
                name: "square metres per second",
            }
        },
    >;

    /// Stokes (`St`), a unit of `DiffusionCoefficient`: `n` of them is `n · 0.0001` coherent units.
    pub type DiffusionCoefficientStokes = SIUnit<
        DIFFUSION_COEFFICIENT,
        {
            SIUnitConsts {
                size: Exact::dec(1, -4),
                zero: Exact::ZERO,
                symbol: "St",
                name: "Stokes",
            }
        },
    >;
}
