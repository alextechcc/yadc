use crate::{ISQ, Kind, SIQuantity};
use yadc::Dimension;

/// The dimension of `ThermodynamicTemperature`.
pub const THERMODYNAMIC_TEMPERATURE: ISQ = ISQ {
    K: 1,
    kind: Kind::Absolute,
    ..ISQ::DIMENSIONLESS
};
pub type ThermodynamicTemperature<N> = SIQuantity<THERMODYNAMIC_TEMPERATURE, N>;

/// The dimension of `TemperatureInterval`.
pub const TEMPERATURE_INTERVAL: ISQ = ISQ {
    K: 1,
    ..ISQ::DIMENSIONLESS
};
pub type TemperatureInterval<N> = SIQuantity<TEMPERATURE_INTERVAL, N>;

/// The dimension of `TemperatureCoefficient`.
pub const TEMPERATURE_COEFFICIENT: ISQ = ISQ {
    K: -1,
    ..ISQ::DIMENSIONLESS
};
pub type TemperatureCoefficient<N> = SIQuantity<TEMPERATURE_COEFFICIENT, N>;

/// The dimension of `TemperatureGradient`.
pub const TEMPERATURE_GRADIENT: ISQ = ISQ {
    m: -1,
    K: 1,
    ..ISQ::DIMENSIONLESS
};
pub type TemperatureGradient<N> = SIQuantity<TEMPERATURE_GRADIENT, N>;

/// The dimension of `HeatCapacity`.
pub const HEAT_CAPACITY: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -2,
    K: -1,
    ..ISQ::DIMENSIONLESS
};
pub type HeatCapacity<N> = SIQuantity<HEAT_CAPACITY, N>;

/// The dimension of `HeatFluxDensity`.
pub const HEAT_FLUX_DENSITY: ISQ = ISQ {
    kg: 1,
    s: -3,
    ..ISQ::DIMENSIONLESS
};
pub type HeatFluxDensity<N> = SIQuantity<HEAT_FLUX_DENSITY, N>;

/// The dimension of `HeatTransfer`.
pub const HEAT_TRANSFER: ISQ = ISQ {
    kg: 1,
    s: -3,
    K: -1,
    ..ISQ::DIMENSIONLESS
};
pub type HeatTransfer<N> = SIQuantity<HEAT_TRANSFER, N>;

/// The dimension of `ThermalConductance`.
pub const THERMAL_CONDUCTANCE: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -3,
    K: -1,
    ..ISQ::DIMENSIONLESS
};
pub type ThermalConductance<N> = SIQuantity<THERMAL_CONDUCTANCE, N>;

/// The dimension of `ThermalConductivity`.
pub const THERMAL_CONDUCTIVITY: ISQ = ISQ {
    m: 1,
    kg: 1,
    s: -3,
    K: -1,
    ..ISQ::DIMENSIONLESS
};
pub type ThermalConductivity<N> = SIQuantity<THERMAL_CONDUCTIVITY, N>;

/// The dimension of `ThermalResistance`.
pub const THERMAL_RESISTANCE: ISQ = ISQ {
    m: -2,
    kg: -1,
    s: 3,
    K: 1,
    ..ISQ::DIMENSIONLESS
};
pub type ThermalResistance<N> = SIQuantity<THERMAL_RESISTANCE, N>;

/// The dimension of `SpecificHeatCapacity`.
pub const SPECIFIC_HEAT_CAPACITY: ISQ = ISQ {
    m: 2,
    s: -2,
    K: -1,
    ..ISQ::DIMENSIONLESS
};
pub type SpecificHeatCapacity<N> = SIQuantity<SPECIFIC_HEAT_CAPACITY, N>;

/// The dimension of `ArealHeatCapacity`.
pub const AREAL_HEAT_CAPACITY: ISQ = ISQ {
    kg: 1,
    s: -2,
    K: -1,
    ..ISQ::DIMENSIONLESS
};
pub type ArealHeatCapacity<N> = SIQuantity<AREAL_HEAT_CAPACITY, N>;

/// The dimension of `VolumetricHeatCapacity`.
pub const VOLUMETRIC_HEAT_CAPACITY: ISQ = ISQ {
    m: -1,
    kg: 1,
    s: -2,
    K: -1,
    ..ISQ::DIMENSIONLESS
};
pub type VolumetricHeatCapacity<N> = SIQuantity<VOLUMETRIC_HEAT_CAPACITY, N>;

/// The dimension of `StefanBoltzmannCoefficient`.
pub const STEFAN_BOLTZMANN_COEFFICIENT: ISQ = ISQ {
    kg: 1,
    s: -3,
    K: -4,
    ..ISQ::DIMENSIONLESS
};
pub type StefanBoltzmannCoefficient<N> = SIQuantity<STEFAN_BOLTZMANN_COEFFICIENT, N>;

/// The units of this module's quantities: a type each, its size and zero
/// in its quantity's coherent unit, exact (`Mile {}.quantity(3.0_f64)`).
pub mod units {
    use super::*;
    use crate::{Exact, SIUnit, SIUnitConsts};

    /// Kelvin (`K`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 1.0` coherent units.
    pub type KelvinScale = SIUnit<
        THERMODYNAMIC_TEMPERATURE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "K",
                name: "kelvin",
            }
        },
    >;

    /// Degrees Celsius (`°C`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 1.0 + 273.15` coherent units.
    pub type Celsius = SIUnit<
        THERMODYNAMIC_TEMPERATURE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::dec(27315, -2),
                symbol: "°C",
                name: "degrees Celsius",
            }
        },
    >;

    /// Degrees Fahrenheit (`°F`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 5.0 / 9.0 + 273.15 - 32.0 * 5.0 / 9.0` coherent units.
    pub type Fahrenheit = SIUnit<
        THERMODYNAMIC_TEMPERATURE,
        {
            SIUnitConsts {
                size: Exact::int(5) / Exact::int(9),
                zero: Exact::dec(27315, -2) - Exact::int(32) * Exact::int(5) / Exact::int(9),
                symbol: "°F",
                name: "degrees Fahrenheit",
            }
        },
    >;

    /// Degrees Rankine (`°R`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 5.0 / 9.0` coherent units.
    pub type Rankine = SIUnit<
        THERMODYNAMIC_TEMPERATURE,
        {
            SIUnitConsts {
                size: Exact::int(5) / Exact::int(9),
                zero: Exact::ZERO,
                symbol: "°R",
                name: "degrees Rankine",
            }
        },
    >;

    /// Kelvin (`K`), a unit of `TemperatureInterval`: `n` of them is `n · 1.0` coherent units.
    pub type Kelvin = SIUnit<
        TEMPERATURE_INTERVAL,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "K",
                name: "kelvin",
            }
        },
    >;

    /// Degrees Celsius (`°C`), a unit of `TemperatureInterval`: `n` of them is `n · 1.0` coherent units.
    pub type DegreeCelsius = SIUnit<
        TEMPERATURE_INTERVAL,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "°C",
                name: "degrees Celsius",
            }
        },
    >;

    /// Degrees Fahrenheit (`°F`), a unit of `TemperatureInterval`: `n` of them is `n · 5.0 / 9.0` coherent units.
    pub type DegreeFahrenheit = SIUnit<
        TEMPERATURE_INTERVAL,
        {
            SIUnitConsts {
                size: Exact::int(5) / Exact::int(9),
                zero: Exact::ZERO,
                symbol: "°F",
                name: "degrees Fahrenheit",
            }
        },
    >;

    /// Degrees Rankine (`°R`), a unit of `TemperatureInterval`: `n` of them is `n · 5.0 / 9.0` coherent units.
    pub type DegreeRankine = SIUnit<
        TEMPERATURE_INTERVAL,
        {
            SIUnitConsts {
                size: Exact::int(5) / Exact::int(9),
                zero: Exact::ZERO,
                symbol: "°R",
                name: "degrees Rankine",
            }
        },
    >;

    /// Per kelvin (`K⁻¹`), a unit of `TemperatureCoefficient`: `n` of them is `n · 1.0` coherent units.
    pub type PerKelvin = SIUnit<
        TEMPERATURE_COEFFICIENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "K⁻¹",
                name: "per kelvin",
            }
        },
    >;

    /// Ppm per kelvin (`ppm/K`), a unit of `TemperatureCoefficient`: `n` of them is `n · 1e-06` coherent units.
    pub type PpmPerKelvin = SIUnit<
        TEMPERATURE_COEFFICIENT,
        {
            SIUnitConsts {
                size: Exact::dec(1, -6),
                zero: Exact::ZERO,
                symbol: "ppm/K",
                name: "ppm per kelvin",
            }
        },
    >;

    /// Ppm per degree Celsius (`ppm/°C`), a unit of `TemperatureCoefficient`: `n` of them is `n · 1e-06` coherent units.
    pub type PpmPerDegreeCelsius = SIUnit<
        TEMPERATURE_COEFFICIENT,
        {
            SIUnitConsts {
                size: Exact::dec(1, -6),
                zero: Exact::ZERO,
                symbol: "ppm/°C",
                name: "ppm per degree Celsius",
            }
        },
    >;

    /// Kelvins per metre (`K/m`), a unit of `TemperatureGradient`: `n` of them is `n · 1.0` coherent units.
    pub type KelvinPerMetre = SIUnit<
        TEMPERATURE_GRADIENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "K/m",
                name: "kelvins per metre",
            }
        },
    >;

    /// Kilogram square metres per second squared kelvin (`kg · m²/(s² · K)`), a unit of `HeatCapacity`: `n` of them is `n · 1.0` coherent units.
    pub type KilogramSquareMetrePerSecondSquaredKelvin = SIUnit<
        HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg · m²/(s² · K)",
                name: "kilogram square metres per second squared kelvin",
            }
        },
    >;

    /// Gram square metres per second squared kelvin (`g · m²/(s² · K)`), a unit of `HeatCapacity`: `n` of them is `n · 0.001` coherent units.
    pub type GramSquareMetrePerSecondSquaredKelvin = SIUnit<
        HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "g · m²/(s² · K)",
                name: "gram square metres per second squared kelvin",
            }
        },
    >;

    /// Joules per kelvin (`J/K`), a unit of `HeatCapacity`: `n` of them is `n · 1.0` coherent units.
    pub type JoulePerKelvin = SIUnit<
        HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/K",
                name: "joules per kelvin",
            }
        },
    >;

    /// Joules per degree celsius (`J/°C`), a unit of `HeatCapacity`: `n` of them is `n · 1.0` coherent units.
    pub type JoulePerDegreeCelsius = SIUnit<
        HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/°C",
                name: "joules per degree celsius",
            }
        },
    >;

    /// British thermal units per degree Fahrenheit (`Btu/°F`), a unit of `HeatCapacity`: `n` of them is `n · (4184.0 * 0.45359237 / 1.8) / (5.0 / 9.0)` coherent units.
    pub type BtuPerDegreeFahrenheit = SIUnit<
        HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: (Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1))
                    / (Exact::int(5) / Exact::int(9)),
                zero: Exact::ZERO,
                symbol: "Btu/°F",
                name: "British thermal units per degree Fahrenheit",
            }
        },
    >;

    /// British thermal units (IT) per degree Fahrenheit (`Btu (IT)/°F`), a unit of `HeatCapacity`: `n` of them is `n · 1055.05585262 / (5.0 / 9.0)` coherent units.
    pub type BtuItPerDegreeFahrenheit = SIUnit<
        HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::dec(105505585262, -8) / (Exact::int(5) / Exact::int(9)),
                zero: Exact::ZERO,
                symbol: "Btu (IT)/°F",
                name: "British thermal units (IT) per degree Fahrenheit",
            }
        },
    >;

    /// Boltzmann constants (`k`), a unit of `HeatCapacity`: `n` of them is `n · 1.380649e-23` coherent units.
    pub type BoltzmannConstant = SIUnit<
        HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::dec(1380649, -29),
                zero: Exact::ZERO,
                symbol: "k",
                name: "Boltzmann constants",
            }
        },
    >;

    /// Calories per kelvin (`cal/K`), a unit of `HeatCapacity`: `n` of them is `n · 4.184` coherent units.
    pub type CaloriePerKelvin = SIUnit<
        HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::dec(4184, -3),
                zero: Exact::ZERO,
                symbol: "cal/K",
                name: "calories per kelvin",
            }
        },
    >;

    /// Watts per square metre (`W/m²`), a unit of `HeatFluxDensity`: `n` of them is `n · 1.0` coherent units.
    pub type WattPerSquareMetre = SIUnit<
        HEAT_FLUX_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W/m²",
                name: "watts per square metre",
            }
        },
    >;

    /// Kilograms per second cubed kelvin (`kg/(s³ · K)`), a unit of `HeatTransfer`: `n` of them is `n · 1.0` coherent units.
    pub type KilogramPerSecondCubedKelvin = SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg/(s³ · K)",
                name: "kilograms per second cubed kelvin",
            }
        },
    >;

    /// Grams per second cubed kelvin (`g/(s³ · K)`), a unit of `HeatTransfer`: `n` of them is `n · 0.001` coherent units.
    pub type GramPerSecondCubedKelvin = SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "g/(s³ · K)",
                name: "grams per second cubed kelvin",
            }
        },
    >;

    /// Kilograms per second cubed degree celsius (`kg/(s³ · °C)`), a unit of `HeatTransfer`: `n` of them is `n · 1.0` coherent units.
    pub type KilogramPerSecondCubedDegreeCelsius = SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg/(s³ · °C)",
                name: "kilograms per second cubed degree celsius",
            }
        },
    >;

    /// Watts per square metre kelvin (`W/(m² · K)`), a unit of `HeatTransfer`: `n` of them is `n · 1.0` coherent units.
    pub type WattPerSquareMetreKelvin = SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W/(m² · K)",
                name: "watts per square metre kelvin",
            }
        },
    >;

    /// Watts per square metre degree celsius (`W/(m² · °C)`), a unit of `HeatTransfer`: `n` of them is `n · 1.0` coherent units.
    pub type WattPerSquareMetreDegreeCelsius = SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W/(m² · °C)",
                name: "watts per square metre degree celsius",
            }
        },
    >;

    /// Joules per second square metre kelvin (`J/(s · m² · K)`), a unit of `HeatTransfer`: `n` of them is `n · 1.0` coherent units.
    pub type JoulePerSecondSquareMetreKelvin = SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/(s · m² · K)",
                name: "joules per second square metre kelvin",
            }
        },
    >;

    /// Joules per second square metre degree celsius (`J/(s · m² · °C)`), a unit of `HeatTransfer`: `n` of them is `n · 1.0` coherent units.
    pub type JoulePerSecondSquareMetreDegreeCelsius = SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/(s · m² · °C)",
                name: "joules per second square metre degree celsius",
            }
        },
    >;

    /// British thermal units (IT) per hour square foot degree Fahrenheit (`Btu (IT)/(hr · ft² · °F)`), a unit of `HeatTransfer`: `n` of them is `n · 1055.05585262 / (3600.0 * 0.3048 * 0.3048 * (5.0 / 9.0))` coherent units.
    pub type BtuItPerHourSquareFootDegreeFahrenheit = SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: Exact::dec(105505585262, -8)
                    / (Exact::int(3600)
                        * Exact::dec(3048, -4)
                        * Exact::dec(3048, -4)
                        * (Exact::int(5) / Exact::int(9))),
                zero: Exact::ZERO,
                symbol: "Btu (IT)/(hr · ft² · °F)",
                name: "British thermal units (IT) per hour square foot degree Fahrenheit",
            }
        },
    >;

    /// British thermal units (IT) per hour square inch degree Fahrenheit (`Btu (IT)/(hr · in² · °F)`), a unit of `HeatTransfer`: `n` of them is `n · 1055.05585262 / (3600.0 * 0.0254 * 0.0254 * (5.0 / 9.0))` coherent units.
    pub type BtuItPerHourSquareInchDegreeFahrenheit = SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: Exact::dec(105505585262, -8)
                    / (Exact::int(3600)
                        * Exact::dec(254, -4)
                        * Exact::dec(254, -4)
                        * (Exact::int(5) / Exact::int(9))),
                zero: Exact::ZERO,
                symbol: "Btu (IT)/(hr · in² · °F)",
                name: "British thermal units (IT) per hour square inch degree Fahrenheit",
            }
        },
    >;

    /// British thermal units (IT) per minute square foot degree Fahrenheit (`Btu (IT)/(min · ft² · °F)`), a unit of `HeatTransfer`: `n` of them is `n · 1055.05585262 / (60.0 * 0.3048 * 0.3048 * (5.0 / 9.0))` coherent units.
    pub type BtuItPerMinuteSquareFootDegreeFahrenheit = SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: Exact::dec(105505585262, -8)
                    / (Exact::int(60)
                        * Exact::dec(3048, -4)
                        * Exact::dec(3048, -4)
                        * (Exact::int(5) / Exact::int(9))),
                zero: Exact::ZERO,
                symbol: "Btu (IT)/(min · ft² · °F)",
                name: "British thermal units (IT) per minute square foot degree Fahrenheit",
            }
        },
    >;

    /// British thermal units (IT) per minute square inch degree Fahrenheit (`Btu (IT)/(min · in² · °F)`), a unit of `HeatTransfer`: `n` of them is `n · 1055.05585262 / (60.0 * 0.0254 * 0.0254 * (5.0 / 9.0))` coherent units.
    pub type BtuItPerMinuteSquareInchDegreeFahrenheit = SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: Exact::dec(105505585262, -8)
                    / (Exact::int(60)
                        * Exact::dec(254, -4)
                        * Exact::dec(254, -4)
                        * (Exact::int(5) / Exact::int(9))),
                zero: Exact::ZERO,
                symbol: "Btu (IT)/(min · in² · °F)",
                name: "British thermal units (IT) per minute square inch degree Fahrenheit",
            }
        },
    >;

    /// British thermal units (IT) per second square foot degree Fahrenheit (`Btu (IT)/(s · ft² · °F)`), a unit of `HeatTransfer`: `n` of them is `n · 1055.05585262 / (0.3048 * 0.3048 * (5.0 / 9.0))` coherent units.
    pub type BtuItPerSecondSquareFootDegreeFahrenheit = SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: Exact::dec(105505585262, -8)
                    / (Exact::dec(3048, -4)
                        * Exact::dec(3048, -4)
                        * (Exact::int(5) / Exact::int(9))),
                zero: Exact::ZERO,
                symbol: "Btu (IT)/(s · ft² · °F)",
                name: "British thermal units (IT) per second square foot degree Fahrenheit",
            }
        },
    >;

    /// British thermal units (IT) per second square inch degree Fahrenheit (`Btu (IT)/(s · in² · °F)`), a unit of `HeatTransfer`: `n` of them is `n · 1055.05585262 / (0.0254 * 0.0254 * (5.0 / 9.0))` coherent units.
    pub type BtuItPerSecondSquareInchDegreeFahrenheit = SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: Exact::dec(105505585262, -8)
                    / (Exact::dec(254, -4) * Exact::dec(254, -4) * (Exact::int(5) / Exact::int(9))),
                zero: Exact::ZERO,
                symbol: "Btu (IT)/(s · in² · °F)",
                name: "British thermal units (IT) per second square inch degree Fahrenheit",
            }
        },
    >;

    /// British thermal units per hour square foot degree Fahrenheit (`Btu/(hr · ft² · °F)`), a unit of `HeatTransfer`: `n` of them is `n · (4184.0 * 0.45359237 / 1.8) / (3600.0 * 0.3048 * 0.3048 * (5.0 / 9.0))` coherent units.
    pub type BtuPerHourSquareFootDegreeFahrenheit = SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: (Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1))
                    / (Exact::int(3600)
                        * Exact::dec(3048, -4)
                        * Exact::dec(3048, -4)
                        * (Exact::int(5) / Exact::int(9))),
                zero: Exact::ZERO,
                symbol: "Btu/(hr · ft² · °F)",
                name: "British thermal units per hour square foot degree Fahrenheit",
            }
        },
    >;

    /// British thermal units per hour square inch degree Fahrenheit (`Btu/(hr · in² · °F)`), a unit of `HeatTransfer`: `n` of them is `n · (4184.0 * 0.45359237 / 1.8) / (3600.0 * 0.0254 * 0.0254 * (5.0 / 9.0))` coherent units.
    pub type BtuPerHourSquareInchDegreeFahrenheit = SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: (Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1))
                    / (Exact::int(3600)
                        * Exact::dec(254, -4)
                        * Exact::dec(254, -4)
                        * (Exact::int(5) / Exact::int(9))),
                zero: Exact::ZERO,
                symbol: "Btu/(hr · in² · °F)",
                name: "British thermal units per hour square inch degree Fahrenheit",
            }
        },
    >;

    /// British thermal units per minute square foot degree Fahrenheit (`Btu/(min · ft² · °F)`), a unit of `HeatTransfer`: `n` of them is `n · (4184.0 * 0.45359237 / 1.8) / (60.0 * 0.3048 * 0.3048 * (5.0 / 9.0))` coherent units.
    pub type BtuPerMinuteSquareFootDegreeFahrenheit = SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: (Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1))
                    / (Exact::int(60)
                        * Exact::dec(3048, -4)
                        * Exact::dec(3048, -4)
                        * (Exact::int(5) / Exact::int(9))),
                zero: Exact::ZERO,
                symbol: "Btu/(min · ft² · °F)",
                name: "British thermal units per minute square foot degree Fahrenheit",
            }
        },
    >;

    /// British thermal units per minute square inch degree Fahrenheit (`Btu/(min · in² · °F)`), a unit of `HeatTransfer`: `n` of them is `n · (4184.0 * 0.45359237 / 1.8) / (60.0 * 0.0254 * 0.0254 * (5.0 / 9.0))` coherent units.
    pub type BtuPerMinuteSquareInchDegreeFahrenheit = SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: (Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1))
                    / (Exact::int(60)
                        * Exact::dec(254, -4)
                        * Exact::dec(254, -4)
                        * (Exact::int(5) / Exact::int(9))),
                zero: Exact::ZERO,
                symbol: "Btu/(min · in² · °F)",
                name: "British thermal units per minute square inch degree Fahrenheit",
            }
        },
    >;

    /// British thermal units per second square foot degree Fahrenheit (`Btu/(s · ft² · °F)`), a unit of `HeatTransfer`: `n` of them is `n · (4184.0 * 0.45359237 / 1.8) / (0.3048 * 0.3048 * (5.0 / 9.0))` coherent units.
    pub type BtuPerSecondSquareFootDegreeFahrenheit = SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: (Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1))
                    / (Exact::dec(3048, -4)
                        * Exact::dec(3048, -4)
                        * (Exact::int(5) / Exact::int(9))),
                zero: Exact::ZERO,
                symbol: "Btu/(s · ft² · °F)",
                name: "British thermal units per second square foot degree Fahrenheit",
            }
        },
    >;

    /// British thermal units per second square inch degree Fahrenheit (`Btu/(s · in² · °F)`), a unit of `HeatTransfer`: `n` of them is `n · (4184.0 * 0.45359237 / 1.8) / (0.0254 * 0.0254 * (5.0 / 9.0))` coherent units.
    pub type BtuPerSecondSquareInchDegreeFahrenheit = SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: (Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1))
                    / (Exact::dec(254, -4) * Exact::dec(254, -4) * (Exact::int(5) / Exact::int(9))),
                zero: Exact::ZERO,
                symbol: "Btu/(s · in² · °F)",
                name: "British thermal units per second square inch degree Fahrenheit",
            }
        },
    >;

    /// Kilograms metre squared per second cubed kelvin (`kg · m²/(s³ · K)`), a unit of `ThermalConductance`: `n` of them is `n · 1.0` coherent units.
    pub type KilogramMetreSquaredPerSecondCubedKelvin = SIUnit<
        THERMAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg · m²/(s³ · K)",
                name: "kilograms metre squared per second cubed kelvin",
            }
        },
    >;

    /// Grams metre squared per second cubed kelvin (`g · m/(s³ · K)`), a unit of `ThermalConductance`: `n` of them is `n · 0.001` coherent units.
    pub type GramMetreSquaredPerSecondCubedKelvin = SIUnit<
        THERMAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "g · m/(s³ · K)",
                name: "grams metre squared per second cubed kelvin",
            }
        },
    >;

    /// Watts per kelvin (`W/K`), a unit of `ThermalConductance`: `n` of them is `n · 1.0` coherent units.
    pub type WattPerKelvin = SIUnit<
        THERMAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W/K",
                name: "watts per kelvin",
            }
        },
    >;

    /// Kilograms metre squared per second cubed degree Celsius (`kg · m²/(s³ · °C)`), a unit of `ThermalConductance`: `n` of them is `n · 1.0` coherent units.
    pub type KilogramMetreSquaredPerSecondCubedDegreeCelsius = SIUnit<
        THERMAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg · m²/(s³ · °C)",
                name: "kilograms metre squared per second cubed degree Celsius",
            }
        },
    >;

    /// Watts per degree Celsius (`W/°C`), a unit of `ThermalConductance`: `n` of them is `n · 1.0` coherent units.
    pub type ThermalConductanceWattPerMetreDegreeCelsius = SIUnit<
        THERMAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W/°C",
                name: "watts per degree Celsius",
            }
        },
    >;

    /// Kilowatts per degree Celsius (`kW/°C`), a unit of `ThermalConductance`: `n` of them is `n · 1000.0` coherent units.
    pub type KilowattPerDegreeCelsius = SIUnit<
        THERMAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "kW/°C",
                name: "kilowatts per degree Celsius",
            }
        },
    >;

    /// Milliwatts per degree Celsius (`mW/°C`), a unit of `ThermalConductance`: `n` of them is `n · 0.001` coherent units.
    pub type MilliwattPerDegreeCelsius = SIUnit<
        THERMAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "mW/°C",
                name: "milliwatts per degree Celsius",
            }
        },
    >;

    /// Kilograms metre per second cubed kelvin (`kg · m/(s³ · K)`), a unit of `ThermalConductivity`: `n` of them is `n · 1.0` coherent units.
    pub type KilogramMetrePerSecondCubedKelvin = SIUnit<
        THERMAL_CONDUCTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg · m/(s³ · K)",
                name: "kilograms metre per second cubed kelvin",
            }
        },
    >;

    /// Grams metre per second cubed kelvin (`g · m/(s³ · K)`), a unit of `ThermalConductivity`: `n` of them is `n · 0.001` coherent units.
    pub type GramMetrePerSecondCubedKelvin = SIUnit<
        THERMAL_CONDUCTIVITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "g · m/(s³ · K)",
                name: "grams metre per second cubed kelvin",
            }
        },
    >;

    /// Watts per metre kelvin (`W/(m · K)`), a unit of `ThermalConductivity`: `n` of them is `n · 1.0` coherent units.
    pub type WattPerMetreKelvin = SIUnit<
        THERMAL_CONDUCTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W/(m · K)",
                name: "watts per metre kelvin",
            }
        },
    >;

    /// Kilograms metre per second cubed degree celsius (`kg · m/(s³ · °C)`), a unit of `ThermalConductivity`: `n` of them is `n · 1.0` coherent units.
    pub type KilogramMetrePerSecondCubedDegreeCelsius = SIUnit<
        THERMAL_CONDUCTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg · m/(s³ · °C)",
                name: "kilograms metre per second cubed degree celsius",
            }
        },
    >;

    /// Watts per metre degree celsius (`W/(m · °C)`), a unit of `ThermalConductivity`: `n` of them is `n · 1.0` coherent units.
    pub type ThermalConductivityWattPerMetreDegreeCelsius = SIUnit<
        THERMAL_CONDUCTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W/(m · °C)",
                name: "watts per metre degree celsius",
            }
        },
    >;

    /// Kelvins per watt (`K/W`), a unit of `ThermalResistance`: `n` of them is `n · 1.0` coherent units.
    pub type KelvinPerWatt = SIUnit<
        THERMAL_RESISTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "K/W",
                name: "kelvins per watt",
            }
        },
    >;

    /// Kelvins per kilowatt (`K/kW`), a unit of `ThermalResistance`: `n` of them is `n · 0.001` coherent units.
    pub type KelvinPerKilowatt = SIUnit<
        THERMAL_RESISTANCE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "K/kW",
                name: "kelvins per kilowatt",
            }
        },
    >;

    /// Square metres per second squared kelvin (`m²/(s² · K)`), a unit of `SpecificHeatCapacity`: `n` of them is `n · 1.0` coherent units.
    pub type SquareMetrePerSecondSquaredKelvin = SIUnit<
        SPECIFIC_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m²/(s² · K)",
                name: "square metres per second squared kelvin",
            }
        },
    >;

    /// Joules per kilogram kelvin (`J/(kg · K)`), a unit of `SpecificHeatCapacity`: `n` of them is `n · 1.0` coherent units.
    pub type JoulePerKilogramKelvin = SIUnit<
        SPECIFIC_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/(kg · K)",
                name: "joules per kilogram kelvin",
            }
        },
    >;

    /// Joules per kilogram degree celsius (`J/(kg · °C)`), a unit of `SpecificHeatCapacity`: `n` of them is `n · 1.0` coherent units.
    pub type JoulePerKilogramDegreeCelsius = SIUnit<
        SPECIFIC_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/(kg · °C)",
                name: "joules per kilogram degree celsius",
            }
        },
    >;

    /// Joules per gram degree celsius (`J/(g · °C)`), a unit of `SpecificHeatCapacity`: `n` of them is `n · 1000.0` coherent units.
    pub type JoulePerGramDegreeCelsius = SIUnit<
        SPECIFIC_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "J/(g · °C)",
                name: "joules per gram degree celsius",
            }
        },
    >;

    /// British thermal units per ounce degree Fahrenheit (`Btu/(oz · °F)`), a unit of `SpecificHeatCapacity`: `n` of them is `n · 16.0 * 4184.0` coherent units.
    pub type BtuPerOunceDegreeFahrenheit = SIUnit<
        SPECIFIC_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(16) * Exact::int(4184),
                zero: Exact::ZERO,
                symbol: "Btu/(oz · °F)",
                name: "British thermal units per ounce degree Fahrenheit",
            }
        },
    >;

    /// British thermal units (IT) per ounce degree Fahrenheit (`Btu (IT)/(oz · °F)`), a unit of `SpecificHeatCapacity`: `n` of them is `n · 16.0 * 4186.8` coherent units.
    pub type BtuItPerOunceDegreeFahrenheit = SIUnit<
        SPECIFIC_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(16) * Exact::dec(41868, -1),
                zero: Exact::ZERO,
                symbol: "Btu (IT)/(oz · °F)",
                name: "British thermal units (IT) per ounce degree Fahrenheit",
            }
        },
    >;

    /// British thermal units per pound degree Fahrenheit (`Btu/(lb · °F)`), a unit of `SpecificHeatCapacity`: `n` of them is `n · 4184.0` coherent units.
    pub type BtuPerPoundDegreeFahrenheit = SIUnit<
        SPECIFIC_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(4184),
                zero: Exact::ZERO,
                symbol: "Btu/(lb · °F)",
                name: "British thermal units per pound degree Fahrenheit",
            }
        },
    >;

    /// British thermal units (IT) per pound degree Fahrenheit (`Btu (IT)/(lb · °F)`), a unit of `SpecificHeatCapacity`: `n` of them is `n · 4186.8` coherent units.
    pub type BtuItPerPoundDegreeFahrenheit = SIUnit<
        SPECIFIC_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::dec(41868, -1),
                zero: Exact::ZERO,
                symbol: "Btu (IT)/(lb · °F)",
                name: "British thermal units (IT) per pound degree Fahrenheit",
            }
        },
    >;

    /// British thermal units per ton degree Fahrenheit (`Btu/(t · °F)`), a unit of `SpecificHeatCapacity`: `n` of them is `n · (4184.0 * 0.45359237 / 1.8) / (1000.0 * (5.0 / 9.0))` coherent units.
    pub type BtuPerTonDegreeFahrenheit = SIUnit<
        SPECIFIC_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: (Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1))
                    / (Exact::int(1000) * (Exact::int(5) / Exact::int(9))),
                zero: Exact::ZERO,
                symbol: "Btu/(t · °F)",
                name: "British thermal units per ton degree Fahrenheit",
            }
        },
    >;

    /// British thermal units (IT) per ton degree Fahrenheit (`Btu (IT)/(t · °F)`), a unit of `SpecificHeatCapacity`: `n` of them is `n · 1055.05585262 / (1000.0 * (5.0 / 9.0))` coherent units.
    pub type BtuItPerTonDegreeFahrenheit = SIUnit<
        SPECIFIC_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::dec(105505585262, -8)
                    / (Exact::int(1000) * (Exact::int(5) / Exact::int(9))),
                zero: Exact::ZERO,
                symbol: "Btu (IT)/(t · °F)",
                name: "British thermal units (IT) per ton degree Fahrenheit",
            }
        },
    >;

    /// Calories per gram kelvin (`cal/(g · K)`), a unit of `SpecificHeatCapacity`: `n` of them is `n · 4184.0` coherent units.
    pub type CaloriePerGramKelvin = SIUnit<
        SPECIFIC_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(4184),
                zero: Exact::ZERO,
                symbol: "cal/(g · K)",
                name: "calories per gram kelvin",
            }
        },
    >;

    /// Joules per square metre kelvin (`J/(m² · K)`), a unit of `ArealHeatCapacity`: `n` of them is `n · 1.0` coherent units.
    pub type JoulePerSquareMetreKelvin = SIUnit<
        AREAL_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/(m² · K)",
                name: "joules per square metre kelvin",
            }
        },
    >;

    /// Joules per cubic metre kelvin (`J/(m³ · K)`), a unit of `VolumetricHeatCapacity`: `n` of them is `n · 1.0` coherent units.
    pub type JoulePerCubicMetreKelvin = SIUnit<
        VOLUMETRIC_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/(m³ · K)",
                name: "joules per cubic metre kelvin",
            }
        },
    >;

    /// Calories per cubic metre kelvin (`cal/(m³ · K)`), a unit of `VolumetricHeatCapacity`: `n` of them is `n · 4.184` coherent units.
    pub type CaloriePerCubicMetreKelvin = SIUnit<
        VOLUMETRIC_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::dec(4184, -3),
                zero: Exact::ZERO,
                symbol: "cal/(m³ · K)",
                name: "calories per cubic metre kelvin",
            }
        },
    >;

    /// Watts per square metre quartic kelvin (`W/(m² · K⁴)`), a unit of `StefanBoltzmannCoefficient`: `n` of them is `n · 1.0` coherent units.
    pub type WattPerSquareMetreQuarticKelvin = SIUnit<
        STEFAN_BOLTZMANN_COEFFICIENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W/(m² · K⁴)",
                name: "watts per square metre quartic kelvin",
            }
        },
    >;

    /// Stefan-Boltzmann constants (`σ`), a unit of `StefanBoltzmannCoefficient`: `n` of them is `n · 5.670374419e-8` coherent units.
    pub type StefanBoltzmannConstant = SIUnit<
        STEFAN_BOLTZMANN_COEFFICIENT,
        {
            SIUnitConsts {
                size: Exact::dec(5670374419, -17),
                zero: Exact::ZERO,
                symbol: "σ",
                name: "Stefan-Boltzmann constants",
            }
        },
    >;
}
