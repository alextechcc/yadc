use crate::{ISQ, Kind};
use yadc::Dimension;

/// The dimension of `ThermodynamicTemperature`: `SIQuantity<THERMODYNAMIC_TEMPERATURE, N>` is one.
pub const THERMODYNAMIC_TEMPERATURE: ISQ = ISQ {
    K: 1,
    kind: Kind::Absolute,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `TemperatureInterval`: `SIQuantity<TEMPERATURE_INTERVAL, N>` is one.
pub const TEMPERATURE_INTERVAL: ISQ = ISQ {
    K: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `TemperatureCoefficient`: `SIQuantity<TEMPERATURE_COEFFICIENT, N>` is one.
pub const TEMPERATURE_COEFFICIENT: ISQ = ISQ {
    K: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `TemperatureGradient`: `SIQuantity<TEMPERATURE_GRADIENT, N>` is one.
pub const TEMPERATURE_GRADIENT: ISQ = ISQ {
    m: -1,
    K: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `HeatCapacity`: `SIQuantity<HEAT_CAPACITY, N>` is one.
pub const HEAT_CAPACITY: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -2,
    K: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `HeatFluxDensity`: `SIQuantity<HEAT_FLUX_DENSITY, N>` is one.
pub const HEAT_FLUX_DENSITY: ISQ = ISQ {
    kg: 1,
    s: -3,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `HeatTransfer`: `SIQuantity<HEAT_TRANSFER, N>` is one.
pub const HEAT_TRANSFER: ISQ = ISQ {
    kg: 1,
    s: -3,
    K: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ThermalConductance`: `SIQuantity<THERMAL_CONDUCTANCE, N>` is one.
pub const THERMAL_CONDUCTANCE: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -3,
    K: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ThermalConductivity`: `SIQuantity<THERMAL_CONDUCTIVITY, N>` is one.
pub const THERMAL_CONDUCTIVITY: ISQ = ISQ {
    m: 1,
    kg: 1,
    s: -3,
    K: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ThermalResistance`: `SIQuantity<THERMAL_RESISTANCE, N>` is one.
pub const THERMAL_RESISTANCE: ISQ = ISQ {
    m: -2,
    kg: -1,
    s: 3,
    K: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `SpecificHeatCapacity`: `SIQuantity<SPECIFIC_HEAT_CAPACITY, N>` is one.
pub const SPECIFIC_HEAT_CAPACITY: ISQ = ISQ {
    m: 2,
    s: -2,
    K: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ArealHeatCapacity`: `SIQuantity<AREAL_HEAT_CAPACITY, N>` is one.
pub const AREAL_HEAT_CAPACITY: ISQ = ISQ {
    kg: 1,
    s: -2,
    K: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `VolumetricHeatCapacity`: `SIQuantity<VOLUMETRIC_HEAT_CAPACITY, N>` is one.
pub const VOLUMETRIC_HEAT_CAPACITY: ISQ = ISQ {
    m: -1,
    kg: 1,
    s: -2,
    K: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `StefanBoltzmannCoefficient`: `SIQuantity<STEFAN_BOLTZMANN_COEFFICIENT, N>` is one.
pub const STEFAN_BOLTZMANN_COEFFICIENT: ISQ = ISQ {
    kg: 1,
    s: -3,
    K: -4,
    ..ISQ::DIMENSIONLESS
};

/// The units of this module's quantities: each a constant of its own unit
/// type, its size and zero in its quantity's coherent unit, exact
/// (`MILE.quantity(3.0)`, `MILE.value(d)`).
pub mod units {
    use super::*;
    use crate::{Exact, SIUnit, SIUnitConsts};

    /// Kelvin (`K`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 1.0` coherent units.
    pub const KELVIN_SCALE: SIUnit<
        THERMODYNAMIC_TEMPERATURE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "K",
                name: "kelvin",
            }
        },
    > = SIUnit;

    /// Degrees Celsius (`°C`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 1.0 + 273.15` coherent units.
    pub const CELSIUS: SIUnit<
        THERMODYNAMIC_TEMPERATURE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::dec(27315, -2),
                symbol: "°C",
                name: "degrees Celsius",
            }
        },
    > = SIUnit;

    /// Degrees Fahrenheit (`°F`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 5.0 / 9.0 + 273.15 - 32.0 * 5.0 / 9.0` coherent units.
    pub const FAHRENHEIT: SIUnit<
        THERMODYNAMIC_TEMPERATURE,
        {
            SIUnitConsts {
                size: Exact::int(5) / Exact::int(9),
                zero: Exact::dec(27315, -2) - Exact::int(32) * Exact::int(5) / Exact::int(9),
                symbol: "°F",
                name: "degrees Fahrenheit",
            }
        },
    > = SIUnit;

    /// Degrees Rankine (`°R`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 5.0 / 9.0` coherent units.
    pub const RANKINE: SIUnit<
        THERMODYNAMIC_TEMPERATURE,
        {
            SIUnitConsts {
                size: Exact::int(5) / Exact::int(9),
                zero: Exact::ZERO,
                symbol: "°R",
                name: "degrees Rankine",
            }
        },
    > = SIUnit;

    /// Kelvin (`K`), a unit of `TemperatureInterval`: `n` of them is `n · 1.0` coherent units.
    pub const KELVIN: SIUnit<
        TEMPERATURE_INTERVAL,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "K",
                name: "kelvin",
            }
        },
    > = SIUnit;

    /// Degrees Celsius (`°C`), a unit of `TemperatureInterval`: `n` of them is `n · 1.0` coherent units.
    pub const DEGREE_CELSIUS: SIUnit<
        TEMPERATURE_INTERVAL,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "°C",
                name: "degrees Celsius",
            }
        },
    > = SIUnit;

    /// Degrees Fahrenheit (`°F`), a unit of `TemperatureInterval`: `n` of them is `n · 5.0 / 9.0` coherent units.
    pub const DEGREE_FAHRENHEIT: SIUnit<
        TEMPERATURE_INTERVAL,
        {
            SIUnitConsts {
                size: Exact::int(5) / Exact::int(9),
                zero: Exact::ZERO,
                symbol: "°F",
                name: "degrees Fahrenheit",
            }
        },
    > = SIUnit;

    /// Degrees Rankine (`°R`), a unit of `TemperatureInterval`: `n` of them is `n · 5.0 / 9.0` coherent units.
    pub const DEGREE_RANKINE: SIUnit<
        TEMPERATURE_INTERVAL,
        {
            SIUnitConsts {
                size: Exact::int(5) / Exact::int(9),
                zero: Exact::ZERO,
                symbol: "°R",
                name: "degrees Rankine",
            }
        },
    > = SIUnit;

    /// Per kelvin (`K⁻¹`), a unit of `TemperatureCoefficient`: `n` of them is `n · 1.0` coherent units.
    pub const PER_KELVIN: SIUnit<
        TEMPERATURE_COEFFICIENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "K⁻¹",
                name: "per kelvin",
            }
        },
    > = SIUnit;

    /// Ppm per kelvin (`ppm/K`), a unit of `TemperatureCoefficient`: `n` of them is `n · 1e-06` coherent units.
    pub const PPM_PER_KELVIN: SIUnit<
        TEMPERATURE_COEFFICIENT,
        {
            SIUnitConsts {
                size: Exact::dec(1, -6),
                zero: Exact::ZERO,
                symbol: "ppm/K",
                name: "ppm per kelvin",
            }
        },
    > = SIUnit;

    /// Ppm per degree Celsius (`ppm/°C`), a unit of `TemperatureCoefficient`: `n` of them is `n · 1e-06` coherent units.
    pub const PPM_PER_DEGREE_CELSIUS: SIUnit<
        TEMPERATURE_COEFFICIENT,
        {
            SIUnitConsts {
                size: Exact::dec(1, -6),
                zero: Exact::ZERO,
                symbol: "ppm/°C",
                name: "ppm per degree Celsius",
            }
        },
    > = SIUnit;

    /// Kelvins per metre (`K/m`), a unit of `TemperatureGradient`: `n` of them is `n · 1.0` coherent units.
    pub const KELVIN_PER_METRE: SIUnit<
        TEMPERATURE_GRADIENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "K/m",
                name: "kelvins per metre",
            }
        },
    > = SIUnit;

    /// Kilogram square metres per second squared kelvin (`kg · m²/(s² · K)`), a unit of `HeatCapacity`: `n` of them is `n · 1.0` coherent units.
    pub const KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN: SIUnit<
        HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg · m²/(s² · K)",
                name: "kilogram square metres per second squared kelvin",
            }
        },
    > = SIUnit;

    /// Gram square metres per second squared kelvin (`g · m²/(s² · K)`), a unit of `HeatCapacity`: `n` of them is `n · 0.001` coherent units.
    pub const GRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN: SIUnit<
        HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "g · m²/(s² · K)",
                name: "gram square metres per second squared kelvin",
            }
        },
    > = SIUnit;

    /// Joules per kelvin (`J/K`), a unit of `HeatCapacity`: `n` of them is `n · 1.0` coherent units.
    pub const JOULE_PER_KELVIN: SIUnit<
        HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/K",
                name: "joules per kelvin",
            }
        },
    > = SIUnit;

    /// Joules per degree celsius (`J/°C`), a unit of `HeatCapacity`: `n` of them is `n · 1.0` coherent units.
    pub const JOULE_PER_DEGREE_CELSIUS: SIUnit<
        HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/°C",
                name: "joules per degree celsius",
            }
        },
    > = SIUnit;

    /// British thermal units per degree Fahrenheit (`Btu/°F`), a unit of `HeatCapacity`: `n` of them is `n · (4184.0 * 0.45359237 / 1.8) / (5.0 / 9.0)` coherent units.
    pub const BTU_PER_DEGREE_FAHRENHEIT: SIUnit<
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
    > = SIUnit;

    /// British thermal units (IT) per degree Fahrenheit (`Btu (IT)/°F`), a unit of `HeatCapacity`: `n` of them is `n · 1055.05585262 / (5.0 / 9.0)` coherent units.
    pub const BTU_IT_PER_DEGREE_FAHRENHEIT: SIUnit<
        HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::dec(105505585262, -8) / (Exact::int(5) / Exact::int(9)),
                zero: Exact::ZERO,
                symbol: "Btu (IT)/°F",
                name: "British thermal units (IT) per degree Fahrenheit",
            }
        },
    > = SIUnit;

    /// Boltzmann constants (`k`), a unit of `HeatCapacity`: `n` of them is `n · 1.380649e-23` coherent units.
    pub const BOLTZMANN_CONSTANT: SIUnit<
        HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::dec(1380649, -29),
                zero: Exact::ZERO,
                symbol: "k",
                name: "Boltzmann constants",
            }
        },
    > = SIUnit;

    /// Calories per kelvin (`cal/K`), a unit of `HeatCapacity`: `n` of them is `n · 4.184` coherent units.
    pub const CALORIE_PER_KELVIN: SIUnit<
        HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::dec(4184, -3),
                zero: Exact::ZERO,
                symbol: "cal/K",
                name: "calories per kelvin",
            }
        },
    > = SIUnit;

    /// Watts per square metre (`W/m²`), a unit of `HeatFluxDensity`: `n` of them is `n · 1.0` coherent units.
    pub const WATT_PER_SQUARE_METRE: SIUnit<
        HEAT_FLUX_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W/m²",
                name: "watts per square metre",
            }
        },
    > = SIUnit;

    /// Kilograms per second cubed kelvin (`kg/(s³ · K)`), a unit of `HeatTransfer`: `n` of them is `n · 1.0` coherent units.
    pub const KILOGRAM_PER_SECOND_CUBED_KELVIN: SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg/(s³ · K)",
                name: "kilograms per second cubed kelvin",
            }
        },
    > = SIUnit;

    /// Grams per second cubed kelvin (`g/(s³ · K)`), a unit of `HeatTransfer`: `n` of them is `n · 0.001` coherent units.
    pub const GRAM_PER_SECOND_CUBED_KELVIN: SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "g/(s³ · K)",
                name: "grams per second cubed kelvin",
            }
        },
    > = SIUnit;

    /// Kilograms per second cubed degree celsius (`kg/(s³ · °C)`), a unit of `HeatTransfer`: `n` of them is `n · 1.0` coherent units.
    pub const KILOGRAM_PER_SECOND_CUBED_DEGREE_CELSIUS: SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg/(s³ · °C)",
                name: "kilograms per second cubed degree celsius",
            }
        },
    > = SIUnit;

    /// Watts per square metre kelvin (`W/(m² · K)`), a unit of `HeatTransfer`: `n` of them is `n · 1.0` coherent units.
    pub const WATT_PER_SQUARE_METRE_KELVIN: SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W/(m² · K)",
                name: "watts per square metre kelvin",
            }
        },
    > = SIUnit;

    /// Watts per square metre degree celsius (`W/(m² · °C)`), a unit of `HeatTransfer`: `n` of them is `n · 1.0` coherent units.
    pub const WATT_PER_SQUARE_METRE_DEGREE_CELSIUS: SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W/(m² · °C)",
                name: "watts per square metre degree celsius",
            }
        },
    > = SIUnit;

    /// Joules per second square metre kelvin (`J/(s · m² · K)`), a unit of `HeatTransfer`: `n` of them is `n · 1.0` coherent units.
    pub const JOULE_PER_SECOND_SQUARE_METRE_KELVIN: SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/(s · m² · K)",
                name: "joules per second square metre kelvin",
            }
        },
    > = SIUnit;

    /// Joules per second square metre degree celsius (`J/(s · m² · °C)`), a unit of `HeatTransfer`: `n` of them is `n · 1.0` coherent units.
    pub const JOULE_PER_SECOND_SQUARE_METRE_DEGREE_CELSIUS: SIUnit<
        HEAT_TRANSFER,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/(s · m² · °C)",
                name: "joules per second square metre degree celsius",
            }
        },
    > = SIUnit;

    /// British thermal units (IT) per hour square foot degree Fahrenheit (`Btu (IT)/(hr · ft² · °F)`), a unit of `HeatTransfer`: `n` of them is `n · 1055.05585262 / (3600.0 * 0.3048 * 0.3048 * (5.0 / 9.0))` coherent units.
    pub const BTU_IT_PER_HOUR_SQUARE_FOOT_DEGREE_FAHRENHEIT: SIUnit<
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
    > = SIUnit;

    /// British thermal units (IT) per hour square inch degree Fahrenheit (`Btu (IT)/(hr · in² · °F)`), a unit of `HeatTransfer`: `n` of them is `n · 1055.05585262 / (3600.0 * 0.0254 * 0.0254 * (5.0 / 9.0))` coherent units.
    pub const BTU_IT_PER_HOUR_SQUARE_INCH_DEGREE_FAHRENHEIT: SIUnit<
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
    > = SIUnit;

    /// British thermal units (IT) per minute square foot degree Fahrenheit (`Btu (IT)/(min · ft² · °F)`), a unit of `HeatTransfer`: `n` of them is `n · 1055.05585262 / (60.0 * 0.3048 * 0.3048 * (5.0 / 9.0))` coherent units.
    pub const BTU_IT_PER_MINUTE_SQUARE_FOOT_DEGREE_FAHRENHEIT: SIUnit<
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
    > = SIUnit;

    /// British thermal units (IT) per minute square inch degree Fahrenheit (`Btu (IT)/(min · in² · °F)`), a unit of `HeatTransfer`: `n` of them is `n · 1055.05585262 / (60.0 * 0.0254 * 0.0254 * (5.0 / 9.0))` coherent units.
    pub const BTU_IT_PER_MINUTE_SQUARE_INCH_DEGREE_FAHRENHEIT: SIUnit<
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
    > = SIUnit;

    /// British thermal units (IT) per second square foot degree Fahrenheit (`Btu (IT)/(s · ft² · °F)`), a unit of `HeatTransfer`: `n` of them is `n · 1055.05585262 / (0.3048 * 0.3048 * (5.0 / 9.0))` coherent units.
    pub const BTU_IT_PER_SECOND_SQUARE_FOOT_DEGREE_FAHRENHEIT: SIUnit<
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
    > = SIUnit;

    /// British thermal units (IT) per second square inch degree Fahrenheit (`Btu (IT)/(s · in² · °F)`), a unit of `HeatTransfer`: `n` of them is `n · 1055.05585262 / (0.0254 * 0.0254 * (5.0 / 9.0))` coherent units.
    pub const BTU_IT_PER_SECOND_SQUARE_INCH_DEGREE_FAHRENHEIT: SIUnit<
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
    > = SIUnit;

    /// British thermal units per hour square foot degree Fahrenheit (`Btu/(hr · ft² · °F)`), a unit of `HeatTransfer`: `n` of them is `n · (4184.0 * 0.45359237 / 1.8) / (3600.0 * 0.3048 * 0.3048 * (5.0 / 9.0))` coherent units.
    pub const BTU_PER_HOUR_SQUARE_FOOT_DEGREE_FAHRENHEIT: SIUnit<
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
    > = SIUnit;

    /// British thermal units per hour square inch degree Fahrenheit (`Btu/(hr · in² · °F)`), a unit of `HeatTransfer`: `n` of them is `n · (4184.0 * 0.45359237 / 1.8) / (3600.0 * 0.0254 * 0.0254 * (5.0 / 9.0))` coherent units.
    pub const BTU_PER_HOUR_SQUARE_INCH_DEGREE_FAHRENHEIT: SIUnit<
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
    > = SIUnit;

    /// British thermal units per minute square foot degree Fahrenheit (`Btu/(min · ft² · °F)`), a unit of `HeatTransfer`: `n` of them is `n · (4184.0 * 0.45359237 / 1.8) / (60.0 * 0.3048 * 0.3048 * (5.0 / 9.0))` coherent units.
    pub const BTU_PER_MINUTE_SQUARE_FOOT_DEGREE_FAHRENHEIT: SIUnit<
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
    > = SIUnit;

    /// British thermal units per minute square inch degree Fahrenheit (`Btu/(min · in² · °F)`), a unit of `HeatTransfer`: `n` of them is `n · (4184.0 * 0.45359237 / 1.8) / (60.0 * 0.0254 * 0.0254 * (5.0 / 9.0))` coherent units.
    pub const BTU_PER_MINUTE_SQUARE_INCH_DEGREE_FAHRENHEIT: SIUnit<
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
    > = SIUnit;

    /// British thermal units per second square foot degree Fahrenheit (`Btu/(s · ft² · °F)`), a unit of `HeatTransfer`: `n` of them is `n · (4184.0 * 0.45359237 / 1.8) / (0.3048 * 0.3048 * (5.0 / 9.0))` coherent units.
    pub const BTU_PER_SECOND_SQUARE_FOOT_DEGREE_FAHRENHEIT: SIUnit<
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
    > = SIUnit;

    /// British thermal units per second square inch degree Fahrenheit (`Btu/(s · in² · °F)`), a unit of `HeatTransfer`: `n` of them is `n · (4184.0 * 0.45359237 / 1.8) / (0.0254 * 0.0254 * (5.0 / 9.0))` coherent units.
    pub const BTU_PER_SECOND_SQUARE_INCH_DEGREE_FAHRENHEIT: SIUnit<
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
    > = SIUnit;

    /// Kilograms metre squared per second cubed kelvin (`kg · m²/(s³ · K)`), a unit of `ThermalConductance`: `n` of them is `n · 1.0` coherent units.
    pub const KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN: SIUnit<
        THERMAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg · m²/(s³ · K)",
                name: "kilograms metre squared per second cubed kelvin",
            }
        },
    > = SIUnit;

    /// Grams metre squared per second cubed kelvin (`g · m/(s³ · K)`), a unit of `ThermalConductance`: `n` of them is `n · 0.001` coherent units.
    pub const GRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN: SIUnit<
        THERMAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "g · m/(s³ · K)",
                name: "grams metre squared per second cubed kelvin",
            }
        },
    > = SIUnit;

    /// Watts per kelvin (`W/K`), a unit of `ThermalConductance`: `n` of them is `n · 1.0` coherent units.
    pub const WATT_PER_KELVIN: SIUnit<
        THERMAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W/K",
                name: "watts per kelvin",
            }
        },
    > = SIUnit;

    /// Kilograms metre squared per second cubed degree Celsius (`kg · m²/(s³ · °C)`), a unit of `ThermalConductance`: `n` of them is `n · 1.0` coherent units.
    pub const KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_DEGREE_CELSIUS: SIUnit<
        THERMAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg · m²/(s³ · °C)",
                name: "kilograms metre squared per second cubed degree Celsius",
            }
        },
    > = SIUnit;

    /// Watts per degree Celsius (`W/°C`), a unit of `ThermalConductance`: `n` of them is `n · 1.0` coherent units.
    pub const THERMAL_CONDUCTANCE_WATT_PER_METRE_DEGREE_CELSIUS: SIUnit<
        THERMAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W/°C",
                name: "watts per degree Celsius",
            }
        },
    > = SIUnit;

    /// Kilowatts per degree Celsius (`kW/°C`), a unit of `ThermalConductance`: `n` of them is `n · 1000.0` coherent units.
    pub const KILOWATT_PER_DEGREE_CELSIUS: SIUnit<
        THERMAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "kW/°C",
                name: "kilowatts per degree Celsius",
            }
        },
    > = SIUnit;

    /// Milliwatts per degree Celsius (`mW/°C`), a unit of `ThermalConductance`: `n` of them is `n · 0.001` coherent units.
    pub const MILLIWATT_PER_DEGREE_CELSIUS: SIUnit<
        THERMAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "mW/°C",
                name: "milliwatts per degree Celsius",
            }
        },
    > = SIUnit;

    /// Kilograms metre per second cubed kelvin (`kg · m/(s³ · K)`), a unit of `ThermalConductivity`: `n` of them is `n · 1.0` coherent units.
    pub const KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN: SIUnit<
        THERMAL_CONDUCTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg · m/(s³ · K)",
                name: "kilograms metre per second cubed kelvin",
            }
        },
    > = SIUnit;

    /// Grams metre per second cubed kelvin (`g · m/(s³ · K)`), a unit of `ThermalConductivity`: `n` of them is `n · 0.001` coherent units.
    pub const GRAM_METRE_PER_SECOND_CUBED_KELVIN: SIUnit<
        THERMAL_CONDUCTIVITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "g · m/(s³ · K)",
                name: "grams metre per second cubed kelvin",
            }
        },
    > = SIUnit;

    /// Watts per metre kelvin (`W/(m · K)`), a unit of `ThermalConductivity`: `n` of them is `n · 1.0` coherent units.
    pub const WATT_PER_METRE_KELVIN: SIUnit<
        THERMAL_CONDUCTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W/(m · K)",
                name: "watts per metre kelvin",
            }
        },
    > = SIUnit;

    /// Kilograms metre per second cubed degree celsius (`kg · m/(s³ · °C)`), a unit of `ThermalConductivity`: `n` of them is `n · 1.0` coherent units.
    pub const KILOGRAM_METRE_PER_SECOND_CUBED_DEGREE_CELSIUS: SIUnit<
        THERMAL_CONDUCTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg · m/(s³ · °C)",
                name: "kilograms metre per second cubed degree celsius",
            }
        },
    > = SIUnit;

    /// Watts per metre degree celsius (`W/(m · °C)`), a unit of `ThermalConductivity`: `n` of them is `n · 1.0` coherent units.
    pub const THERMAL_CONDUCTIVITY_WATT_PER_METRE_DEGREE_CELSIUS: SIUnit<
        THERMAL_CONDUCTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W/(m · °C)",
                name: "watts per metre degree celsius",
            }
        },
    > = SIUnit;

    /// Kelvins per watt (`K/W`), a unit of `ThermalResistance`: `n` of them is `n · 1.0` coherent units.
    pub const KELVIN_PER_WATT: SIUnit<
        THERMAL_RESISTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "K/W",
                name: "kelvins per watt",
            }
        },
    > = SIUnit;

    /// Kelvins per kilowatt (`K/kW`), a unit of `ThermalResistance`: `n` of them is `n · 0.001` coherent units.
    pub const KELVIN_PER_KILOWATT: SIUnit<
        THERMAL_RESISTANCE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "K/kW",
                name: "kelvins per kilowatt",
            }
        },
    > = SIUnit;

    /// Square metres per second squared kelvin (`m²/(s² · K)`), a unit of `SpecificHeatCapacity`: `n` of them is `n · 1.0` coherent units.
    pub const SQUARE_METRE_PER_SECOND_SQUARED_KELVIN: SIUnit<
        SPECIFIC_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m²/(s² · K)",
                name: "square metres per second squared kelvin",
            }
        },
    > = SIUnit;

    /// Joules per kilogram kelvin (`J/(kg · K)`), a unit of `SpecificHeatCapacity`: `n` of them is `n · 1.0` coherent units.
    pub const JOULE_PER_KILOGRAM_KELVIN: SIUnit<
        SPECIFIC_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/(kg · K)",
                name: "joules per kilogram kelvin",
            }
        },
    > = SIUnit;

    /// Joules per kilogram degree celsius (`J/(kg · °C)`), a unit of `SpecificHeatCapacity`: `n` of them is `n · 1.0` coherent units.
    pub const JOULE_PER_KILOGRAM_DEGREE_CELSIUS: SIUnit<
        SPECIFIC_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/(kg · °C)",
                name: "joules per kilogram degree celsius",
            }
        },
    > = SIUnit;

    /// Joules per gram degree celsius (`J/(g · °C)`), a unit of `SpecificHeatCapacity`: `n` of them is `n · 1000.0` coherent units.
    pub const JOULE_PER_GRAM_DEGREE_CELSIUS: SIUnit<
        SPECIFIC_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "J/(g · °C)",
                name: "joules per gram degree celsius",
            }
        },
    > = SIUnit;

    /// British thermal units per ounce degree Fahrenheit (`Btu/(oz · °F)`), a unit of `SpecificHeatCapacity`: `n` of them is `n · 16.0 * 4184.0` coherent units.
    pub const BTU_PER_OUNCE_DEGREE_FAHRENHEIT: SIUnit<
        SPECIFIC_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(16) * Exact::int(4184),
                zero: Exact::ZERO,
                symbol: "Btu/(oz · °F)",
                name: "British thermal units per ounce degree Fahrenheit",
            }
        },
    > = SIUnit;

    /// British thermal units (IT) per ounce degree Fahrenheit (`Btu (IT)/(oz · °F)`), a unit of `SpecificHeatCapacity`: `n` of them is `n · 16.0 * 4186.8` coherent units.
    pub const BTU_IT_PER_OUNCE_DEGREE_FAHRENHEIT: SIUnit<
        SPECIFIC_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(16) * Exact::dec(41868, -1),
                zero: Exact::ZERO,
                symbol: "Btu (IT)/(oz · °F)",
                name: "British thermal units (IT) per ounce degree Fahrenheit",
            }
        },
    > = SIUnit;

    /// British thermal units per pound degree Fahrenheit (`Btu/(lb · °F)`), a unit of `SpecificHeatCapacity`: `n` of them is `n · 4184.0` coherent units.
    pub const BTU_PER_POUND_DEGREE_FAHRENHEIT: SIUnit<
        SPECIFIC_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(4184),
                zero: Exact::ZERO,
                symbol: "Btu/(lb · °F)",
                name: "British thermal units per pound degree Fahrenheit",
            }
        },
    > = SIUnit;

    /// British thermal units (IT) per pound degree Fahrenheit (`Btu (IT)/(lb · °F)`), a unit of `SpecificHeatCapacity`: `n` of them is `n · 4186.8` coherent units.
    pub const BTU_IT_PER_POUND_DEGREE_FAHRENHEIT: SIUnit<
        SPECIFIC_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::dec(41868, -1),
                zero: Exact::ZERO,
                symbol: "Btu (IT)/(lb · °F)",
                name: "British thermal units (IT) per pound degree Fahrenheit",
            }
        },
    > = SIUnit;

    /// British thermal units per ton degree Fahrenheit (`Btu/(t · °F)`), a unit of `SpecificHeatCapacity`: `n` of them is `n · (4184.0 * 0.45359237 / 1.8) / (1000.0 * (5.0 / 9.0))` coherent units.
    pub const BTU_PER_TON_DEGREE_FAHRENHEIT: SIUnit<
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
    > = SIUnit;

    /// British thermal units (IT) per ton degree Fahrenheit (`Btu (IT)/(t · °F)`), a unit of `SpecificHeatCapacity`: `n` of them is `n · 1055.05585262 / (1000.0 * (5.0 / 9.0))` coherent units.
    pub const BTU_IT_PER_TON_DEGREE_FAHRENHEIT: SIUnit<
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
    > = SIUnit;

    /// Calories per gram kelvin (`cal/(g · K)`), a unit of `SpecificHeatCapacity`: `n` of them is `n · 4184.0` coherent units.
    pub const CALORIE_PER_GRAM_KELVIN: SIUnit<
        SPECIFIC_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(4184),
                zero: Exact::ZERO,
                symbol: "cal/(g · K)",
                name: "calories per gram kelvin",
            }
        },
    > = SIUnit;

    /// Joules per square metre kelvin (`J/(m² · K)`), a unit of `ArealHeatCapacity`: `n` of them is `n · 1.0` coherent units.
    pub const JOULE_PER_SQUARE_METRE_KELVIN: SIUnit<
        AREAL_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/(m² · K)",
                name: "joules per square metre kelvin",
            }
        },
    > = SIUnit;

    /// Joules per cubic metre kelvin (`J/(m³ · K)`), a unit of `VolumetricHeatCapacity`: `n` of them is `n · 1.0` coherent units.
    pub const JOULE_PER_CUBIC_METRE_KELVIN: SIUnit<
        VOLUMETRIC_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/(m³ · K)",
                name: "joules per cubic metre kelvin",
            }
        },
    > = SIUnit;

    /// Calories per cubic metre kelvin (`cal/(m³ · K)`), a unit of `VolumetricHeatCapacity`: `n` of them is `n · 4.184` coherent units.
    pub const CALORIE_PER_CUBIC_METRE_KELVIN: SIUnit<
        VOLUMETRIC_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::dec(4184, -3),
                zero: Exact::ZERO,
                symbol: "cal/(m³ · K)",
                name: "calories per cubic metre kelvin",
            }
        },
    > = SIUnit;

    /// Watts per square metre quartic kelvin (`W/(m² · K⁴)`), a unit of `StefanBoltzmannCoefficient`: `n` of them is `n · 1.0` coherent units.
    pub const WATT_PER_SQUARE_METRE_QUARTIC_KELVIN: SIUnit<
        STEFAN_BOLTZMANN_COEFFICIENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W/(m² · K⁴)",
                name: "watts per square metre quartic kelvin",
            }
        },
    > = SIUnit;

    /// Stefan-Boltzmann constants (`σ`), a unit of `StefanBoltzmannCoefficient`: `n` of them is `n · 5.670374419e-8` coherent units.
    pub const STEFAN_BOLTZMANN_CONSTANT: SIUnit<
        STEFAN_BOLTZMANN_COEFFICIENT,
        {
            SIUnitConsts {
                size: Exact::dec(5670374419, -17),
                zero: Exact::ZERO,
                symbol: "σ",
                name: "Stefan-Boltzmann constants",
            }
        },
    > = SIUnit;
}
