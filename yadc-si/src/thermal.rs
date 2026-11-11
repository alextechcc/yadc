use crate::{Exact, ISQ, Kind, SIQuantity, Scale, counted};
use consts::*;
use yadc::Dimension;

pub type ThermodynamicTemperature<N> = SIQuantity<
    {
        ISQ {
            K: 1,
            kind: Kind::Absolute,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type TemperatureInterval<N> = SIQuantity<
    {
        ISQ {
            K: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type TemperatureCoefficient<N> = SIQuantity<
    {
        ISQ {
            K: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type TemperatureGradient<N> = SIQuantity<
    {
        ISQ {
            m: -1,
            K: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type HeatCapacity<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -2,
            K: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type HeatFluxDensity<N> = SIQuantity<
    {
        ISQ {
            kg: 1,
            s: -3,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type HeatTransfer<N> = SIQuantity<
    {
        ISQ {
            kg: 1,
            s: -3,
            K: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ThermalConductance<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -3,
            K: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ThermalConductivity<N> = SIQuantity<
    {
        ISQ {
            m: 1,
            kg: 1,
            s: -3,
            K: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ThermalResistance<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            kg: -1,
            s: 3,
            K: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type SpecificHeatCapacity<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            s: -2,
            K: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ArealHeatCapacity<N> = SIQuantity<
    {
        ISQ {
            kg: 1,
            s: -2,
            K: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type VolumetricHeatCapacity<N> = SIQuantity<
    {
        ISQ {
            m: -1,
            kg: 1,
            s: -2,
            K: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type StefanBoltzmannCoefficient<N> = SIQuantity<
    {
        ISQ {
            kg: 1,
            s: -3,
            K: -4,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

/// The constants of this module: the base constants of its quantities
/// (re-exported), then one per multiple, each defined exactly from them -
/// `INCH` is `0.0254 * METRE` - and counted in the number type its use asks
/// for, rounded once: `let d: Length<f64> = INCH;` is an `f64` length,
/// `Length<f32>` an `f32` one, `INCH::<Exact>` the exact value. Each is a
/// unit (`INCH(3.0)`, `INCH.value(d)`). A prefixed unit is a prefix times
/// one: `KILO * METRE`. They compose in `const` items:
/// `const LIMIT: Velocity<f64> = 50.0 * (KILO * METRE) / HOUR;`.
pub mod consts {
    use super::*;
    use crate::base::{KILOGRAM, METRE, ONE, SECOND};

    /// The kelvin as a step: a temperature difference of 1 K (the same as 1 °C of difference).
    pub use crate::base::KELVIN;

    /// Absolute zero, 0 K: the zero of thermodynamic temperature.
    pub use crate::base::ABSOLUTE_ZERO;

    /// 0 °C, the ice point of water: 273.15 K.
    pub const ICE_POINT<N: const From<Exact>>: ThermodynamicTemperature<N> = counted(ABSOLUTE_ZERO::<Exact> + Exact::dec(27315, -2) * KELVIN::<Exact>);

    /// 0 °F: 32 °F below the ice point.
    pub const FAHRENHEIT_ZERO<N: const From<Exact>>: ThermodynamicTemperature<N> = counted(ICE_POINT::<Exact> - Exact::int(32) * DEGREE_FAHRENHEIT::<Exact>);

    /// The kelvin scale (K): kelvins from absolute zero.
    pub const KELVIN_SCALE<N: const From<Exact>>: Scale<ThermodynamicTemperature<N>, TemperatureInterval<N>> = Scale(ABSOLUTE_ZERO, KELVIN);

    /// The Celsius scale (°C): kelvins from the ice point.
    pub const CELSIUS<N: const From<Exact>>: Scale<ThermodynamicTemperature<N>, TemperatureInterval<N>> = Scale(ICE_POINT, KELVIN);

    /// The Fahrenheit scale (°F): degrees Fahrenheit from 32 °F below the ice point.
    pub const FAHRENHEIT<N: const From<Exact>>: Scale<ThermodynamicTemperature<N>, TemperatureInterval<N>> = Scale(FAHRENHEIT_ZERO, DEGREE_FAHRENHEIT);

    /// The Rankine scale (°R): degrees Fahrenheit from absolute zero.
    pub const RANKINE<N: const From<Exact>>: Scale<ThermodynamicTemperature<N>, TemperatureInterval<N>> = Scale(ABSOLUTE_ZERO, DEGREE_FAHRENHEIT);

    /// One `Fahrenheit` (°F), as a `TemperatureInterval`.
    pub const DEGREE_FAHRENHEIT<N: const From<Exact>>: TemperatureInterval<N> = counted((Exact::int(5) / Exact::int(9)) * KELVIN::<Exact>);

    /// One `PerKelvin` (K⁻¹), as a `TemperatureCoefficient`.
    pub const PER_KELVIN<N: const From<Exact>>: TemperatureCoefficient<N> = counted(ONE::<Exact> / KELVIN::<Exact>);

    /// One `PpmPerKelvin` (ppm/K), as a `TemperatureCoefficient`.
    pub const PPM_PER_KELVIN<N: const From<Exact>>: TemperatureCoefficient<N> = counted(Exact::dec(1, -6) * PER_KELVIN::<Exact>);

    /// One `PpmPerDegreeCelsius` (ppm/°C), as a `TemperatureCoefficient`.
    pub const PPM_PER_DEGREE_CELSIUS<N: const From<Exact>>: TemperatureCoefficient<N> = counted(Exact::dec(1, -6) * PER_KELVIN::<Exact>);

    /// One `KelvinPerMetre` (K/m), as a `TemperatureGradient`.
    pub const KELVIN_PER_METRE<N: const From<Exact>>: TemperatureGradient<N> = counted(KELVIN::<Exact> / METRE::<Exact>);

    /// One `KilogramSquareMetrePerSecondSquaredKelvin` (kg · m²/(s² · K)), as a `HeatCapacity`.
    pub const KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * KELVIN::<Exact>));

    /// One `GramSquareMetrePerSecondSquaredKelvin` (g · m²/(s² · K)), as a `HeatCapacity`.
    pub const GRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1, -3) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `JoulePerKelvin` (J/K), as a `HeatCapacity`.
    pub const JOULE_PER_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `JoulePerDegreeCelsius` (J/°C), as a `HeatCapacity`.
    pub const JOULE_PER_DEGREE_CELSIUS<N: const From<Exact>>: HeatCapacity<N> = counted(KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `BtuPerDegreeFahrenheit` (Btu/°F), as a `HeatCapacity`.
    pub const BTU_PER_DEGREE_FAHRENHEIT<N: const From<Exact>>: HeatCapacity<N> = counted(((Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1)) / (Exact::int(5) / Exact::int(9))) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `BtuItPerDegreeFahrenheit` (Btu (IT)/°F), as a `HeatCapacity`.
    pub const BTU_IT_PER_DEGREE_FAHRENHEIT<N: const From<Exact>>: HeatCapacity<N> = counted((Exact::dec(105505585262, -8) / (Exact::int(5) / Exact::int(9))) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `BoltzmannConstant` (k), as a `HeatCapacity`.
    pub const BOLTZMANN_CONSTANT<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1380649, -29) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `CaloriePerKelvin` (cal/K), as a `HeatCapacity`.
    pub const CALORIE_PER_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(4184, -3) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `WattPerSquareMetre` (W/m²), as a `HeatFluxDensity`.
    pub const WATT_PER_SQUARE_METRE<N: const From<Exact>>: HeatFluxDensity<N> = counted(KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact>));

    /// One `KilogramPerSecondCubedKelvin` (kg/(s³ · K)), as a `HeatTransfer`.
    pub const KILOGRAM_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * KELVIN::<Exact>));

    /// One `GramPerSecondCubedKelvin` (g/(s³ · K)), as a `HeatTransfer`.
    pub const GRAM_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::dec(1, -3) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `KilogramPerSecondCubedDegreeCelsius` (kg/(s³ · °C)), as a `HeatTransfer`.
    pub const KILOGRAM_PER_SECOND_CUBED_DEGREE_CELSIUS<N: const From<Exact>>: HeatTransfer<N> = counted(KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `WattPerSquareMetreKelvin` (W/(m² · K)), as a `HeatTransfer`.
    pub const WATT_PER_SQUARE_METRE_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `WattPerSquareMetreDegreeCelsius` (W/(m² · °C)), as a `HeatTransfer`.
    pub const WATT_PER_SQUARE_METRE_DEGREE_CELSIUS<N: const From<Exact>>: HeatTransfer<N> = counted(KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `JoulePerSecondSquareMetreKelvin` (J/(s · m² · K)), as a `HeatTransfer`.
    pub const JOULE_PER_SECOND_SQUARE_METRE_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `JoulePerSecondSquareMetreDegreeCelsius` (J/(s · m² · °C)), as a `HeatTransfer`.
    pub const JOULE_PER_SECOND_SQUARE_METRE_DEGREE_CELSIUS<N: const From<Exact>>: HeatTransfer<N> = counted(KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `BtuItPerHourSquareFootDegreeFahrenheit` (Btu (IT)/(hr · ft² · °F)), as a `HeatTransfer`.
    pub const BTU_IT_PER_HOUR_SQUARE_FOOT_DEGREE_FAHRENHEIT<N: const From<Exact>>: HeatTransfer<N> = counted((Exact::dec(105505585262, -8) / (Exact::int(3600) * Exact::dec(3048, -4) * Exact::dec(3048, -4) * (Exact::int(5) / Exact::int(9)))) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `BtuItPerHourSquareInchDegreeFahrenheit` (Btu (IT)/(hr · in² · °F)), as a `HeatTransfer`.
    pub const BTU_IT_PER_HOUR_SQUARE_INCH_DEGREE_FAHRENHEIT<N: const From<Exact>>: HeatTransfer<N> = counted((Exact::dec(105505585262, -8) / (Exact::int(3600) * Exact::dec(254, -4) * Exact::dec(254, -4) * (Exact::int(5) / Exact::int(9)))) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `BtuItPerMinuteSquareFootDegreeFahrenheit` (Btu (IT)/(min · ft² · °F)), as a `HeatTransfer`.
    pub const BTU_IT_PER_MINUTE_SQUARE_FOOT_DEGREE_FAHRENHEIT<N: const From<Exact>>: HeatTransfer<N> = counted((Exact::dec(105505585262, -8) / (Exact::int(60) * Exact::dec(3048, -4) * Exact::dec(3048, -4) * (Exact::int(5) / Exact::int(9)))) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `BtuItPerMinuteSquareInchDegreeFahrenheit` (Btu (IT)/(min · in² · °F)), as a `HeatTransfer`.
    pub const BTU_IT_PER_MINUTE_SQUARE_INCH_DEGREE_FAHRENHEIT<N: const From<Exact>>: HeatTransfer<N> = counted((Exact::dec(105505585262, -8) / (Exact::int(60) * Exact::dec(254, -4) * Exact::dec(254, -4) * (Exact::int(5) / Exact::int(9)))) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `BtuItPerSecondSquareFootDegreeFahrenheit` (Btu (IT)/(s · ft² · °F)), as a `HeatTransfer`.
    pub const BTU_IT_PER_SECOND_SQUARE_FOOT_DEGREE_FAHRENHEIT<N: const From<Exact>>: HeatTransfer<N> = counted((Exact::dec(105505585262, -8) / (Exact::dec(3048, -4) * Exact::dec(3048, -4) * (Exact::int(5) / Exact::int(9)))) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `BtuItPerSecondSquareInchDegreeFahrenheit` (Btu (IT)/(s · in² · °F)), as a `HeatTransfer`.
    pub const BTU_IT_PER_SECOND_SQUARE_INCH_DEGREE_FAHRENHEIT<N: const From<Exact>>: HeatTransfer<N> = counted((Exact::dec(105505585262, -8) / (Exact::dec(254, -4) * Exact::dec(254, -4) * (Exact::int(5) / Exact::int(9)))) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `BtuPerHourSquareFootDegreeFahrenheit` (Btu/(hr · ft² · °F)), as a `HeatTransfer`.
    pub const BTU_PER_HOUR_SQUARE_FOOT_DEGREE_FAHRENHEIT<N: const From<Exact>>: HeatTransfer<N> = counted(((Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1)) / (Exact::int(3600) * Exact::dec(3048, -4) * Exact::dec(3048, -4) * (Exact::int(5) / Exact::int(9)))) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `BtuPerHourSquareInchDegreeFahrenheit` (Btu/(hr · in² · °F)), as a `HeatTransfer`.
    pub const BTU_PER_HOUR_SQUARE_INCH_DEGREE_FAHRENHEIT<N: const From<Exact>>: HeatTransfer<N> = counted(((Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1)) / (Exact::int(3600) * Exact::dec(254, -4) * Exact::dec(254, -4) * (Exact::int(5) / Exact::int(9)))) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `BtuPerMinuteSquareFootDegreeFahrenheit` (Btu/(min · ft² · °F)), as a `HeatTransfer`.
    pub const BTU_PER_MINUTE_SQUARE_FOOT_DEGREE_FAHRENHEIT<N: const From<Exact>>: HeatTransfer<N> = counted(((Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1)) / (Exact::int(60) * Exact::dec(3048, -4) * Exact::dec(3048, -4) * (Exact::int(5) / Exact::int(9)))) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `BtuPerMinuteSquareInchDegreeFahrenheit` (Btu/(min · in² · °F)), as a `HeatTransfer`.
    pub const BTU_PER_MINUTE_SQUARE_INCH_DEGREE_FAHRENHEIT<N: const From<Exact>>: HeatTransfer<N> = counted(((Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1)) / (Exact::int(60) * Exact::dec(254, -4) * Exact::dec(254, -4) * (Exact::int(5) / Exact::int(9)))) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `BtuPerSecondSquareFootDegreeFahrenheit` (Btu/(s · ft² · °F)), as a `HeatTransfer`.
    pub const BTU_PER_SECOND_SQUARE_FOOT_DEGREE_FAHRENHEIT<N: const From<Exact>>: HeatTransfer<N> = counted(((Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1)) / (Exact::dec(3048, -4) * Exact::dec(3048, -4) * (Exact::int(5) / Exact::int(9)))) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `BtuPerSecondSquareInchDegreeFahrenheit` (Btu/(s · in² · °F)), as a `HeatTransfer`.
    pub const BTU_PER_SECOND_SQUARE_INCH_DEGREE_FAHRENHEIT<N: const From<Exact>>: HeatTransfer<N> = counted(((Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1)) / (Exact::dec(254, -4) * Exact::dec(254, -4) * (Exact::int(5) / Exact::int(9)))) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `YottagramMetreSquaredPerSecondCubedKelvin` (Yg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const YOTTAGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::int(1000000000000000000000) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `ZettagramMetreSquaredPerSecondCubedKelvin` (Zg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const ZETTAGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::int(1000000000000000000) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `ExagramMetreSquaredPerSecondCubedKelvin` (Eg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const EXAGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::int(1000000000000000) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `PetagramMetreSquaredPerSecondCubedKelvin` (Pg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const PETAGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::int(1000000000000) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `TeragramMetreSquaredPerSecondCubedKelvin` (Tg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const TERAGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::int(1000000000) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `GigagramMetreSquaredPerSecondCubedKelvin` (Gg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const GIGAGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::int(1000000) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `MegagramMetreSquaredPerSecondCubedKelvin` (Mg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const MEGAGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::int(1000) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `KilogramMetreSquaredPerSecondCubedKelvin` (kg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * KELVIN::<Exact>));

    /// One `HectogramMetreSquaredPerSecondCubedKelvin` (hg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const HECTOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::dec(1, -1) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `DecagramMetreSquaredPerSecondCubedKelvin` (dag · m²/(s³ · K)), as a `ThermalConductance`.
    pub const DECAGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::dec(1, -2) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `GramMetreSquaredPerSecondCubedKelvin` (g · m/(s³ · K)), as a `ThermalConductance`.
    pub const GRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::dec(1, -3) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `DecigramMetreSquaredPerSecondCubedKelvin` (dg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const DECIGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::dec(1, -4) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `CentigramMetreSquaredPerSecondCubedKelvin` (cg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const CENTIGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::dec(1, -5) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `MilligramMetreSquaredPerSecondCubedKelvin` (mg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const MILLIGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::dec(1, -6) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `NanogramMetreSquaredPerSecondCubedKelvin` (ng · m²/(s³ · K)), as a `ThermalConductance`.
    pub const NANOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::dec(1, -12) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `PicogramMetreSquaredPerSecondCubedKelvin` (pg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const PICOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::dec(1, -15) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `FemtogramMetreSquaredPerSecondCubedKelvin` (fg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const FEMTOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::dec(1, -18) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `AttogramMetreSquaredPerSecondCubedKelvin` (ag · m²/(s³ · K)), as a `ThermalConductance`.
    pub const ATTOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::dec(1, -21) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `ZeptogramMetreSquaredPerSecondCubedKelvin` (zg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const ZEPTOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::dec(1, -24) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `YoctogramMetreSquaredPerSecondCubedKelvin` (yg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const YOCTOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::dec(1, -27) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `WattPerKelvin` (W/K), as a `ThermalConductance`.
    pub const WATT_PER_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `KilogramMetreSquaredPerSecondCubedDegreeCelsius` (kg · m²/(s³ · °C)), as a `ThermalConductance`.
    pub const KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_DEGREE_CELSIUS<N: const From<Exact>>: ThermalConductance<N> = counted(KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `KilowattPerDegreeCelsius` (kW/°C), as a `ThermalConductance`.
    pub const KILOWATT_PER_DEGREE_CELSIUS<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::int(1000) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `WattPerMetreDegreeCelsius` (W/°C), as a `ThermalConductance`.
    pub const THERMAL_CONDUCTANCE_WATT_PER_METRE_DEGREE_CELSIUS<N: const From<Exact>>: ThermalConductance<N> = counted(KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `MilliwattPerDegreeCelsius` (mW/°C), as a `ThermalConductance`.
    pub const MILLIWATT_PER_DEGREE_CELSIUS<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::dec(1, -3) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `KilogramMetrePerSecondCubedKelvin` (kg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * KELVIN::<Exact>));

    /// One `GramMetrePerSecondCubedKelvin` (g · m/(s³ · K)), as a `ThermalConductivity`.
    pub const GRAM_METRE_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::dec(1, -3) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `WattPerMetreKelvin` (W/(m · K)), as a `ThermalConductivity`.
    pub const WATT_PER_METRE_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `KilogramMetrePerSecondCubedDegreeCelsius` (kg · m/(s³ · °C)), as a `ThermalConductivity`.
    pub const KILOGRAM_METRE_PER_SECOND_CUBED_DEGREE_CELSIUS<N: const From<Exact>>: ThermalConductivity<N> = counted(KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `WattPerMetreDegreeCelsius` (W/(m · °C)), as a `ThermalConductivity`.
    pub const THERMAL_CONDUCTIVITY_WATT_PER_METRE_DEGREE_CELSIUS<N: const From<Exact>>: ThermalConductivity<N> = counted(KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `KelvinPerKilowatt` (K/kw), as a `ThermalResistance`.
    pub const KELVIN_PER_KILOWATT<N: const From<Exact>>: ThermalResistance<N> = counted(Exact::dec(1, -3) * KELVIN_PER_WATT::<Exact>);

    /// One `KelvinPerWatt` (K/W), as a `ThermalResistance`.
    pub const KELVIN_PER_WATT<N: const From<Exact>>: ThermalResistance<N> = counted(SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * KELVIN::<Exact> / (METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact>));

    /// One `SquareMetrePerSecondSquaredKelvin` (m²/(s² · K)), as a `SpecificHeatCapacity`.
    pub const SQUARE_METRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(METRE::<Exact> * METRE::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * KELVIN::<Exact>));

    /// One `JoulePerKilogramKelvin` (J/(kg · K)), as a `SpecificHeatCapacity`.
    pub const JOULE_PER_KILOGRAM_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `JoulePerKilogramDegreeCelsius` (J/(kg · °C)), as a `SpecificHeatCapacity`.
    pub const JOULE_PER_KILOGRAM_DEGREE_CELSIUS<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `JoulePerGramDegreeCelsius` (J/(g · °C)), as a `SpecificHeatCapacity`.
    pub const JOULE_PER_GRAM_DEGREE_CELSIUS<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::int(1000) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `MillijoulePerGramDegreeCelsius` (mJ/(g · °C)), as a `SpecificHeatCapacity`.
    pub const MILLIJOULE_PER_GRAM_DEGREE_CELSIUS<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `BtuPerOunceDegreeFahrenheit` (Btu/(oz · °F)), as a `SpecificHeatCapacity`.
    pub const BTU_PER_OUNCE_DEGREE_FAHRENHEIT<N: const From<Exact>>: SpecificHeatCapacity<N> = counted((Exact::int(16) * Exact::int(4184)) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `BtuItPerOunceDegreeFahrenheit` (Btu (IT)/(oz · °F)), as a `SpecificHeatCapacity`.
    pub const BTU_IT_PER_OUNCE_DEGREE_FAHRENHEIT<N: const From<Exact>>: SpecificHeatCapacity<N> = counted((Exact::int(16) * Exact::dec(41868, -1)) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `BtuPerPoundDegreeFahrenheit` (Btu/(lb · °F)), as a `SpecificHeatCapacity`.
    pub const BTU_PER_POUND_DEGREE_FAHRENHEIT<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::int(4184) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `BtuItPerPoundDegreeFahrenheit` (Btu (IT)/(lb · °F)), as a `SpecificHeatCapacity`.
    pub const BTU_IT_PER_POUND_DEGREE_FAHRENHEIT<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::dec(41868, -1) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `BtuPerTonDegreeFahrenheit` (Btu/(t · °F)), as a `SpecificHeatCapacity`.
    pub const BTU_PER_TON_DEGREE_FAHRENHEIT<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(((Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1)) / (Exact::int(1000) * (Exact::int(5) / Exact::int(9)))) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `BtuItPerTonDegreeFahrenheit` (Btu (IT)/(t · °F)), as a `SpecificHeatCapacity`.
    pub const BTU_IT_PER_TON_DEGREE_FAHRENHEIT<N: const From<Exact>>: SpecificHeatCapacity<N> = counted((Exact::dec(105505585262, -8) / (Exact::int(1000) * (Exact::int(5) / Exact::int(9)))) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `CaloriePerGramKelvin` (cal/(g · K)), as a `SpecificHeatCapacity`.
    pub const CALORIE_PER_GRAM_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::int(4184) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `JoulePerSquareMetreKelvin` (J/(m² · K)), as an `ArealHeatCapacity`.
    pub const JOULE_PER_SQUARE_METRE_KELVIN<N: const From<Exact>>: ArealHeatCapacity<N> = counted(KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * KELVIN::<Exact>));

    /// One `JoulePerCubicMetreKelvin` (J/(m³ · K)), as a `VolumetricHeatCapacity`.
    pub const JOULE_PER_CUBIC_METRE_KELVIN<N: const From<Exact>>: VolumetricHeatCapacity<N> = counted(KILOGRAM::<Exact> / (METRE::<Exact> * SECOND::<Exact> * SECOND::<Exact> * KELVIN::<Exact>));

    /// One `CaloriePerCubicMetreKelvin` (cal/(m³ · K)), as a `VolumetricHeatCapacity`.
    pub const CALORIE_PER_CUBIC_METRE_KELVIN<N: const From<Exact>>: VolumetricHeatCapacity<N> = counted(Exact::dec(4184, -3) * JOULE_PER_CUBIC_METRE_KELVIN::<Exact>);

    /// One `WattPerSquareMetreQuarticKelvin` (W/(m² · K⁴)), as a `StefanBoltzmannCoefficient`.
    pub const WATT_PER_SQUARE_METRE_QUARTIC_KELVIN<N: const From<Exact>>: StefanBoltzmannCoefficient<N> = counted(KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * KELVIN::<Exact> * KELVIN::<Exact> * KELVIN::<Exact> * KELVIN::<Exact>));

    /// One `StefanBoltzmannConstant` (σ), as a `StefanBoltzmannCoefficient`.
    pub const STEFAN_BOLTZMANN_CONSTANT<N: const From<Exact>>: StefanBoltzmannCoefficient<N> = counted(Exact::dec(5670374419, -17) * WATT_PER_SQUARE_METRE_QUARTIC_KELVIN::<Exact>);
}

/// The units of `ThermodynamicTemperature`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ThermodynamicTemperature<f64>)]
pub enum ThermodynamicTemperatureUnit {
    #[default]
    #[unit("K", "kelvin", by = KELVIN_SCALE)]
    Kelvin,
    #[unit("°C", "degrees Celsius", by = CELSIUS)]
    Celsius,
    #[unit("°F", "degrees Fahrenheit", by = FAHRENHEIT)]
    Fahrenheit,
    #[unit("°R", "degrees Rankine", by = RANKINE)]
    Rankine,
}

/// The units of `TemperatureInterval`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = TemperatureInterval<f64>)]
pub enum TemperatureIntervalUnit {
    #[default]
    #[unit("K", "kelvin", by = KELVIN)]
    Kelvin,
    #[unit("°C", "degrees Celsius", by = KELVIN)]
    Celsius,
    #[unit("°F", "degrees Fahrenheit", by = DEGREE_FAHRENHEIT)]
    Fahrenheit,
    #[unit("°R", "degrees Rankine", by = DEGREE_FAHRENHEIT)]
    Rankine,
}

/// The units of `TemperatureCoefficient`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = TemperatureCoefficient<f64>)]
pub enum TemperatureCoefficientUnit {
    #[default]
    #[unit("K⁻¹", "per kelvin", by = PER_KELVIN)]
    PerKelvin,
    #[unit("ppm/K", "ppm per kelvin", by = PPM_PER_KELVIN)]
    PpmPerKelvin,
    #[unit("ppm/°C", "ppm per degree Celsius", by = PPM_PER_DEGREE_CELSIUS)]
    PpmPerDegreeCelsius,
}

/// The units of `TemperatureGradient`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = TemperatureGradient<f64>)]
pub enum TemperatureGradientUnit {
    #[default]
    #[unit("K/m", "kelvins per metre", by = KELVIN_PER_METRE)]
    KelvinPerMetre,
}

/// The units of `HeatCapacity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = HeatCapacity<f64>)]
pub enum HeatCapacityUnit {
    #[default]
    #[unit("kg · m²/(s² · K)", "kilogram square metres per second squared kelvin", by = KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    KilogramSquareMetrePerSecondSquaredKelvin,
    #[unit("g · m²/(s² · K)", "gram square metres per second squared kelvin", by = GRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    GramSquareMetrePerSecondSquaredKelvin,
    #[unit("J/K", "joules per kelvin", by = JOULE_PER_KELVIN)]
    JoulePerKelvin,
    #[unit("J/°C", "joules per degree celsius", by = JOULE_PER_DEGREE_CELSIUS)]
    JoulePerDegreeCelsius,
    #[unit("Btu/°F", "British thermal units per degree Fahrenheit", by = BTU_PER_DEGREE_FAHRENHEIT)]
    BtuPerDegreeFahrenheit,
    #[unit("Btu (IT)/°F", "British thermal units (IT) per degree Fahrenheit", by = BTU_IT_PER_DEGREE_FAHRENHEIT)]
    BtuItPerDegreeFahrenheit,
    #[unit("k", "Boltzmann constants", by = BOLTZMANN_CONSTANT)]
    BoltzmannConstant,
    #[unit("cal/K", "calories per kelvin", by = CALORIE_PER_KELVIN)]
    CaloriePerKelvin,
}

/// The units of `HeatFluxDensity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = HeatFluxDensity<f64>)]
pub enum HeatFluxDensityUnit {
    #[default]
    #[unit("W/m²", "watts per square metre", by = WATT_PER_SQUARE_METRE)]
    WattPerSquareMetre,
}

/// The units of `HeatTransfer`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = HeatTransfer<f64>)]
pub enum HeatTransferUnit {
    #[default]
    #[unit("kg/(s³ · K)", "kilograms per second cubed kelvin", by = KILOGRAM_PER_SECOND_CUBED_KELVIN)]
    KilogramPerSecondCubedKelvin,
    #[unit("g/(s³ · K)", "grams per second cubed kelvin", by = GRAM_PER_SECOND_CUBED_KELVIN)]
    GramPerSecondCubedKelvin,
    #[unit("kg/(s³ · °C)", "kilograms per second cubed degree celsius", by = KILOGRAM_PER_SECOND_CUBED_DEGREE_CELSIUS)]
    KilogramPerSecondCubedDegreeCelsius,
    #[unit("W/(m² · K)", "watts per square metre kelvin", by = WATT_PER_SQUARE_METRE_KELVIN)]
    WattPerSquareMetreKelvin,
    #[unit("W/(m² · °C)", "watts per square metre degree celsius", by = WATT_PER_SQUARE_METRE_DEGREE_CELSIUS)]
    WattPerSquareMetreDegreeCelsius,
    #[unit("J/(s · m² · K)", "joules per second square metre kelvin", by = JOULE_PER_SECOND_SQUARE_METRE_KELVIN)]
    JoulePerSecondSquareMetreKelvin,
    #[unit("J/(s · m² · °C)", "joules per second square metre degree celsius", by = JOULE_PER_SECOND_SQUARE_METRE_DEGREE_CELSIUS)]
    JoulePerSecondSquareMetreDegreeCelsius,
    #[unit("Btu (IT)/(hr · ft² · °F)", "British thermal units (IT) per hour square foot degree Fahrenheit", by = BTU_IT_PER_HOUR_SQUARE_FOOT_DEGREE_FAHRENHEIT)]
    BtuItPerHourSquareFootDegreeFahrenheit,
    #[unit("Btu (IT)/(hr · in² · °F)", "British thermal units (IT) per hour square inch degree Fahrenheit", by = BTU_IT_PER_HOUR_SQUARE_INCH_DEGREE_FAHRENHEIT)]
    BtuItPerHourSquareInchDegreeFahrenheit,
    #[unit("Btu (IT)/(min · ft² · °F)", "British thermal units (IT) per minute square foot degree Fahrenheit", by = BTU_IT_PER_MINUTE_SQUARE_FOOT_DEGREE_FAHRENHEIT)]
    BtuItPerMinuteSquareFootDegreeFahrenheit,
    #[unit("Btu (IT)/(min · in² · °F)", "British thermal units (IT) per minute square inch degree Fahrenheit", by = BTU_IT_PER_MINUTE_SQUARE_INCH_DEGREE_FAHRENHEIT)]
    BtuItPerMinuteSquareInchDegreeFahrenheit,
    #[unit("Btu (IT)/(s · ft² · °F)", "British thermal units (IT) per second square foot degree Fahrenheit", by = BTU_IT_PER_SECOND_SQUARE_FOOT_DEGREE_FAHRENHEIT)]
    BtuItPerSecondSquareFootDegreeFahrenheit,
    #[unit("Btu (IT)/(s · in² · °F)", "British thermal units (IT) per second square inch degree Fahrenheit", by = BTU_IT_PER_SECOND_SQUARE_INCH_DEGREE_FAHRENHEIT)]
    BtuItPerSecondSquareInchDegreeFahrenheit,
    #[unit("Btu/(hr · ft² · °F)", "British thermal units per hour square foot degree Fahrenheit", by = BTU_PER_HOUR_SQUARE_FOOT_DEGREE_FAHRENHEIT)]
    BtuPerHourSquareFootDegreeFahrenheit,
    #[unit("Btu/(hr · in² · °F)", "British thermal units per hour square inch degree Fahrenheit", by = BTU_PER_HOUR_SQUARE_INCH_DEGREE_FAHRENHEIT)]
    BtuPerHourSquareInchDegreeFahrenheit,
    #[unit("Btu/(min · ft² · °F)", "British thermal units per minute square foot degree Fahrenheit", by = BTU_PER_MINUTE_SQUARE_FOOT_DEGREE_FAHRENHEIT)]
    BtuPerMinuteSquareFootDegreeFahrenheit,
    #[unit("Btu/(min · in² · °F)", "British thermal units per minute square inch degree Fahrenheit", by = BTU_PER_MINUTE_SQUARE_INCH_DEGREE_FAHRENHEIT)]
    BtuPerMinuteSquareInchDegreeFahrenheit,
    #[unit("Btu/(s · ft² · °F)", "British thermal units per second square foot degree Fahrenheit", by = BTU_PER_SECOND_SQUARE_FOOT_DEGREE_FAHRENHEIT)]
    BtuPerSecondSquareFootDegreeFahrenheit,
    #[unit("Btu/(s · in² · °F)", "British thermal units per second square inch degree Fahrenheit", by = BTU_PER_SECOND_SQUARE_INCH_DEGREE_FAHRENHEIT)]
    BtuPerSecondSquareInchDegreeFahrenheit,
}

/// The units of `ThermalConductance`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ThermalConductance<f64>)]
pub enum ThermalConductanceUnit {
    #[unit("Yg · m²/(s³ · K)", "yottagrams metre squared per second cubed kelvin", by = YOTTAGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN)]
    YottagramMetreSquaredPerSecondCubedKelvin,
    #[unit("Zg · m²/(s³ · K)", "zettagrams metre squared per second cubed kelvin", by = ZETTAGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN)]
    ZettagramMetreSquaredPerSecondCubedKelvin,
    #[unit("Eg · m²/(s³ · K)", "exagrams metre squared per second cubed kelvin", by = EXAGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN)]
    ExagramMetreSquaredPerSecondCubedKelvin,
    #[unit("Pg · m²/(s³ · K)", "petagrams metre squared per second cubed kelvin", by = PETAGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN)]
    PetagramMetreSquaredPerSecondCubedKelvin,
    #[unit("Tg · m²/(s³ · K)", "teragrams metre squared per second cubed kelvin", by = TERAGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN)]
    TeragramMetreSquaredPerSecondCubedKelvin,
    #[unit("Gg · m²/(s³ · K)", "gigagrams metre squared per second cubed kelvin", by = GIGAGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN)]
    GigagramMetreSquaredPerSecondCubedKelvin,
    #[unit("Mg · m²/(s³ · K)", "megagrams metre squared per second cubed kelvin", by = MEGAGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN)]
    MegagramMetreSquaredPerSecondCubedKelvin,
    #[default]
    #[unit("kg · m²/(s³ · K)", "kilograms metre squared per second cubed kelvin", by = KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN)]
    KilogramMetreSquaredPerSecondCubedKelvin,
    #[unit("hg · m²/(s³ · K)", "hectograms metre squared per second cubed kelvin", by = HECTOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN)]
    HectogramMetreSquaredPerSecondCubedKelvin,
    #[unit("dag · m²/(s³ · K)", "decagrams metre squared per second cubed kelvin", by = DECAGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN)]
    DecagramMetreSquaredPerSecondCubedKelvin,
    #[unit("g · m/(s³ · K)", "grams metre squared per second cubed kelvin", by = GRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN)]
    GramMetreSquaredPerSecondCubedKelvin,
    #[unit("dg · m²/(s³ · K)", "decigrams metre squared per second cubed kelvin", by = DECIGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN)]
    DecigramMetreSquaredPerSecondCubedKelvin,
    #[unit("cg · m²/(s³ · K)", "centigrams metre squared per second cubed kelvin", by = CENTIGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN)]
    CentigramMetreSquaredPerSecondCubedKelvin,
    #[unit("mg · m²/(s³ · K)", "milligrams metre squared per second cubed kelvin", by = MILLIGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN)]
    MilligramMetreSquaredPerSecondCubedKelvin,
    #[unit("ng · m²/(s³ · K)", "nanograms metre squared per second cubed kelvin", by = NANOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN)]
    NanogramMetreSquaredPerSecondCubedKelvin,
    #[unit("pg · m²/(s³ · K)", "picograms metre squared per second cubed kelvin", by = PICOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN)]
    PicogramMetreSquaredPerSecondCubedKelvin,
    #[unit("fg · m²/(s³ · K)", "femtograms metre squared per second cubed kelvin", by = FEMTOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN)]
    FemtogramMetreSquaredPerSecondCubedKelvin,
    #[unit("ag · m²/(s³ · K)", "attograms metre squared per second cubed kelvin", by = ATTOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN)]
    AttogramMetreSquaredPerSecondCubedKelvin,
    #[unit("zg · m²/(s³ · K)", "zeptograms metre squared per second cubed kelvin", by = ZEPTOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN)]
    ZeptogramMetreSquaredPerSecondCubedKelvin,
    #[unit("yg · m²/(s³ · K)", "yoctograms metre squared per second cubed kelvin", by = YOCTOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN)]
    YoctogramMetreSquaredPerSecondCubedKelvin,
    #[unit("W/K", "watts per kelvin", by = WATT_PER_KELVIN)]
    WattPerKelvin,
    #[unit("kg · m²/(s³ · °C)", "kilograms metre squared per second cubed degree Celsius", by = KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_DEGREE_CELSIUS)]
    KilogramMetreSquaredPerSecondCubedDegreeCelsius,
    #[unit("kW/°C", "kilowatts per degree Celsius", by = KILOWATT_PER_DEGREE_CELSIUS)]
    KilowattPerDegreeCelsius,
    #[unit("W/°C", "watts per degree Celsius", by = THERMAL_CONDUCTANCE_WATT_PER_METRE_DEGREE_CELSIUS)]
    WattPerMetreDegreeCelsius,
    #[unit("mW/°C", "milliwatts per degree Celsius", by = MILLIWATT_PER_DEGREE_CELSIUS)]
    MilliwattPerDegreeCelsius,
}

/// The units of `ThermalConductivity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ThermalConductivity<f64>)]
pub enum ThermalConductivityUnit {
    #[default]
    #[unit("kg · m/(s³ · K)", "kilograms metre per second cubed kelvin", by = KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN)]
    KilogramMetrePerSecondCubedKelvin,
    #[unit("g · m/(s³ · K)", "grams metre per second cubed kelvin", by = GRAM_METRE_PER_SECOND_CUBED_KELVIN)]
    GramMetrePerSecondCubedKelvin,
    #[unit("W/(m · K)", "watts per metre kelvin", by = WATT_PER_METRE_KELVIN)]
    WattPerMetreKelvin,
    #[unit("kg · m/(s³ · °C)", "kilograms metre per second cubed degree celsius", by = KILOGRAM_METRE_PER_SECOND_CUBED_DEGREE_CELSIUS)]
    KilogramMetrePerSecondCubedDegreeCelsius,
    #[unit("W/(m · °C)", "watts per metre degree celsius", by = THERMAL_CONDUCTIVITY_WATT_PER_METRE_DEGREE_CELSIUS)]
    WattPerMetreDegreeCelsius,
}

/// The units of `ThermalResistance`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ThermalResistance<f64>)]
pub enum ThermalResistanceUnit {
    #[unit("K/kw", "kelvins per kilowatt", by = KELVIN_PER_KILOWATT)]
    KelvinPerKilowatt,
    #[default]
    #[unit("K/W", "kelvins per watt", by = KELVIN_PER_WATT)]
    KelvinPerWatt,
}

/// The units of `SpecificHeatCapacity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificHeatCapacity<f64>)]
pub enum SpecificHeatCapacityUnit {
    #[default]
    #[unit("m²/(s² · K)", "square metres per second squared kelvin", by = SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    SquareMetrePerSecondSquaredKelvin,
    #[unit("J/(kg · K)", "joules per kilogram kelvin", by = JOULE_PER_KILOGRAM_KELVIN)]
    JoulePerKilogramKelvin,
    #[unit("J/(kg · °C)", "joules per kilogram degree celsius", by = JOULE_PER_KILOGRAM_DEGREE_CELSIUS)]
    JoulePerKilogramDegreeCelsius,
    #[unit("J/(g · °C)", "joules per gram degree celsius", by = JOULE_PER_GRAM_DEGREE_CELSIUS)]
    JoulePerGramDegreeCelsius,
    #[unit("mJ/(g · °C)", "millijoules per gram degree celsius", by = MILLIJOULE_PER_GRAM_DEGREE_CELSIUS)]
    MillijoulePerGramDegreeCelsius,
    #[unit("Btu/(oz · °F)", "British thermal units per ounce degree Fahrenheit", by = BTU_PER_OUNCE_DEGREE_FAHRENHEIT)]
    BtuPerOunceDegreeFahrenheit,
    #[unit("Btu (IT)/(oz · °F)", "British thermal units (IT) per ounce degree Fahrenheit", by = BTU_IT_PER_OUNCE_DEGREE_FAHRENHEIT)]
    BtuItPerOunceDegreeFahrenheit,
    #[unit("Btu/(lb · °F)", "British thermal units per pound degree Fahrenheit", by = BTU_PER_POUND_DEGREE_FAHRENHEIT)]
    BtuPerPoundDegreeFahrenheit,
    #[unit("Btu (IT)/(lb · °F)", "British thermal units (IT) per pound degree Fahrenheit", by = BTU_IT_PER_POUND_DEGREE_FAHRENHEIT)]
    BtuItPerPoundDegreeFahrenheit,
    #[unit("Btu/(t · °F)", "British thermal units per ton degree Fahrenheit", by = BTU_PER_TON_DEGREE_FAHRENHEIT)]
    BtuPerTonDegreeFahrenheit,
    #[unit("Btu (IT)/(t · °F)", "British thermal units (IT) per ton degree Fahrenheit", by = BTU_IT_PER_TON_DEGREE_FAHRENHEIT)]
    BtuItPerTonDegreeFahrenheit,
    #[unit("cal/(g · K)", "calories per gram kelvin", by = CALORIE_PER_GRAM_KELVIN)]
    CaloriePerGramKelvin,
}

/// The units of `ArealHeatCapacity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ArealHeatCapacity<f64>)]
pub enum ArealHeatCapacityUnit {
    #[default]
    #[unit("J/(m² · K)", "joules per square metre kelvin", by = JOULE_PER_SQUARE_METRE_KELVIN)]
    JoulePerSquareMetreKelvin,
}

/// The units of `VolumetricHeatCapacity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = VolumetricHeatCapacity<f64>)]
pub enum VolumetricHeatCapacityUnit {
    #[default]
    #[unit("J/(m³ · K)", "joules per cubic metre kelvin", by = JOULE_PER_CUBIC_METRE_KELVIN)]
    JoulePerCubicMetreKelvin,
    #[unit("cal/(m³ · K)", "calories per cubic metre kelvin", by = CALORIE_PER_CUBIC_METRE_KELVIN)]
    CaloriePerCubicMetreKelvin,
}

/// The units of `StefanBoltzmannCoefficient`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = StefanBoltzmannCoefficient<f64>)]
pub enum StefanBoltzmannCoefficientUnit {
    #[default]
    #[unit("W/(m² · K⁴)", "watts per square metre quartic kelvin", by = WATT_PER_SQUARE_METRE_QUARTIC_KELVIN)]
    WattPerSquareMetreQuarticKelvin,
    #[unit("σ", "Stefan-Boltzmann constants", by = STEFAN_BOLTZMANN_CONSTANT)]
    StefanBoltzmannConstant,
}
