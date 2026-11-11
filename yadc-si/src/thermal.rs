use crate::{ISQ, Kind, Scale, StaticSIQuantity};
use consts::*;
use yadc::Dimension;

pub type ThermodynamicTemperature = StaticSIQuantity<
    {
        ISQ {
            K: 1,
            kind: Kind::Absolute,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type TemperatureInterval = StaticSIQuantity<
    {
        ISQ {
            K: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type TemperatureCoefficient = StaticSIQuantity<
    {
        ISQ {
            K: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type TemperatureGradient = StaticSIQuantity<
    {
        ISQ {
            m: -1,
            K: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type HeatCapacity = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -2,
            K: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type HeatFluxDensity = StaticSIQuantity<
    {
        ISQ {
            kg: 1,
            s: -3,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type HeatTransfer = StaticSIQuantity<
    {
        ISQ {
            kg: 1,
            s: -3,
            K: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ThermalConductance = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -3,
            K: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ThermalConductivity = StaticSIQuantity<
    {
        ISQ {
            m: 1,
            kg: 1,
            s: -3,
            K: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ThermalResistance = StaticSIQuantity<
    {
        ISQ {
            m: -2,
            kg: -1,
            s: 3,
            K: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type SpecificHeatCapacity = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            s: -2,
            K: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ArealHeatCapacity = StaticSIQuantity<
    {
        ISQ {
            kg: 1,
            s: -2,
            K: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type VolumetricHeatCapacity = StaticSIQuantity<
    {
        ISQ {
            m: -1,
            kg: 1,
            s: -2,
            K: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type StefanBoltzmannCoefficient = StaticSIQuantity<
    {
        ISQ {
            kg: 1,
            s: -3,
            K: -4,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

/// The constants of this module: the base constants of its quantities
/// (re-exported), then one per multiple, each defined from them - `KILOMETRE`
/// is `1000.0 * METRE`, a `Length` and a unit of its own (`KILOMETRE.value(d)`).
/// Quantity arithmetic is `const`, so they compose in `const` items:
/// `const LIMIT: Velocity = 50.0 * KILOMETRE / HOUR;`.
pub mod consts {
    use super::*;
    use crate::base::{KILOGRAM, METRE, ONE, SECOND};

    /// The kelvin as a step: a temperature difference of 1 K (the same as 1 °C of difference).
    pub use crate::base::KELVIN;

    /// Absolute zero, 0 K: the zero of thermodynamic temperature.
    pub use crate::base::ABSOLUTE_ZERO;

    /// 0 °C, the ice point of water: 273.15 K.
    pub const ICE_POINT: ThermodynamicTemperature = ABSOLUTE_ZERO + 273.15 * KELVIN;

    /// 0 °F: 32 °F below the ice point.
    pub const FAHRENHEIT_ZERO: ThermodynamicTemperature = ICE_POINT - 32.0 * DEGREE_FAHRENHEIT;

    /// The kelvin scale (K): kelvins from absolute zero.
    pub const KELVIN_SCALE: Scale<ThermodynamicTemperature, TemperatureInterval> =
        Scale(ABSOLUTE_ZERO, KELVIN);

    /// The Celsius scale (°C): kelvins from the ice point.
    pub const CELSIUS: Scale<ThermodynamicTemperature, TemperatureInterval> =
        Scale(ICE_POINT, KELVIN);

    /// The Fahrenheit scale (°F): degrees Fahrenheit from 32 °F below the ice point.
    pub const FAHRENHEIT: Scale<ThermodynamicTemperature, TemperatureInterval> =
        Scale(FAHRENHEIT_ZERO, DEGREE_FAHRENHEIT);

    /// The Rankine scale (°R): degrees Fahrenheit from absolute zero.
    pub const RANKINE: Scale<ThermodynamicTemperature, TemperatureInterval> =
        Scale(ABSOLUTE_ZERO, DEGREE_FAHRENHEIT);

    /// One `Fahrenheit` (°F), as a `TemperatureInterval`.
    pub const DEGREE_FAHRENHEIT: TemperatureInterval = (5.0 / 9.0) * KELVIN;

    /// One `PerKelvin` (K⁻¹), as a `TemperatureCoefficient`.
    pub const PER_KELVIN: TemperatureCoefficient = ONE / KELVIN;

    /// One `PpmPerKelvin` (ppm/K), as a `TemperatureCoefficient`.
    pub const PPM_PER_KELVIN: TemperatureCoefficient = 9.9999999999999995e-07 * PER_KELVIN;

    /// One `PpmPerDegreeCelsius` (ppm/°C), as a `TemperatureCoefficient`.
    pub const PPM_PER_DEGREE_CELSIUS: TemperatureCoefficient = 9.9999999999999995e-07 * PER_KELVIN;

    /// One `KelvinPerKilometre` (K/km), as a `TemperatureGradient`.
    pub const KELVIN_PER_KILOMETRE: TemperatureGradient = 0.001 * KELVIN_PER_METRE;

    /// One `KelvinPerHectometre` (K/hm), as a `TemperatureGradient`.
    pub const KELVIN_PER_HECTOMETRE: TemperatureGradient = 0.01 * KELVIN_PER_METRE;

    /// One `KelvinPerMetre` (K/m), as a `TemperatureGradient`.
    pub const KELVIN_PER_METRE: TemperatureGradient = KELVIN / METRE;

    /// One `KelvinPerCentimetre` (K/cm), as a `TemperatureGradient`.
    pub const KELVIN_PER_CENTIMETRE: TemperatureGradient = 100.0 * KELVIN_PER_METRE;

    /// One `KelvinPerMillimetre` (K/mm), as a `TemperatureGradient`.
    pub const KELVIN_PER_MILLIMETRE: TemperatureGradient = 1000.0 * KELVIN_PER_METRE;

    /// One `KelvinPerMicrometre` (K/µm), as a `TemperatureGradient`.
    pub const KELVIN_PER_MICROMETRE: TemperatureGradient = 1000000.0 * KELVIN_PER_METRE;

    /// One `YottagramSquareMetrePerSecondSquaredKelvin` (Yg · m²/(s² · K)), as a `HeatCapacity`.
    pub const YOTTAGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN: HeatCapacity =
        1e+21 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `ZettagramSquareMetrePerSecondSquaredKelvin` (Zg · m²/(s² · K)), as a `HeatCapacity`.
    pub const ZETTAGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN: HeatCapacity =
        1e+18 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `ExagramSquareMetrePerSecondSquaredKelvin` (Eg · m²/(s² · K)), as a `HeatCapacity`.
    pub const EXAGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN: HeatCapacity =
        1000000000000000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `PetagramSquareMetrePerSecondSquaredKelvin` (Pg · m²/(s² · K)), as a `HeatCapacity`.
    pub const PETAGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN: HeatCapacity =
        1000000000000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `TeragramSquareMetrePerSecondSquaredKelvin` (Tg · m²/(s² · K)), as a `HeatCapacity`.
    pub const TERAGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN: HeatCapacity =
        1000000000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `GigagramSquareMetrePerSecondSquaredKelvin` (Gg · m²/(s² · K)), as a `HeatCapacity`.
    pub const GIGAGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN: HeatCapacity =
        1000000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `MegagramSquareMetrePerSecondSquaredKelvin` (Mg · m²/(s² · K)), as a `HeatCapacity`.
    pub const MEGAGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN: HeatCapacity =
        1000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `KilogramSquareMetrePerSecondSquaredKelvin` (kg · m²/(s² · K)), as a `HeatCapacity`.
    pub const KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN: HeatCapacity =
        METRE * METRE * KILOGRAM / (SECOND * SECOND * KELVIN);

    /// One `HectogramSquareMetrePerSecondSquaredKelvin` (hg · m²/(s² · K)), as a `HeatCapacity`.
    pub const HECTOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN: HeatCapacity =
        0.10000000000000001 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `DecagramSquareMetrePerSecondSquaredKelvin` (dag · m²/(s² · K)), as a `HeatCapacity`.
    pub const DECAGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN: HeatCapacity =
        0.01 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `GramSquareMetrePerSecondSquaredKelvin` (g · m²/(s² · K)), as a `HeatCapacity`.
    pub const GRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN: HeatCapacity =
        0.001 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `DecigramSquareMetrePerSecondSquaredKelvin` (dg · m²/(s² · K)), as a `HeatCapacity`.
    pub const DECIGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN: HeatCapacity =
        0.0001 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `CentigramSquareMetrePerSecondSquaredKelvin` (cg · m²/(s² · K)), as a `HeatCapacity`.
    pub const CENTIGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN: HeatCapacity =
        1.0000000000000001e-05 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `MilligramSquareMetrePerSecondSquaredKelvin` (mg · m²/(s² · K)), as a `HeatCapacity`.
    pub const MILLIGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN: HeatCapacity =
        9.9999999999999995e-07 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `MicrogramSquareMetrePerSecondSquaredKelvin` (µg · m²/(s² · K)), as a `HeatCapacity`.
    pub const MICROGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN: HeatCapacity =
        9.9999999999999986e-10 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `NanogramSquareMetrePerSecondSquaredKelvin` (ng · m²/(s² · K)), as a `HeatCapacity`.
    pub const NANOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN: HeatCapacity =
        9.9999999999999998e-13 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `PicogramSquareMetrePerSecondSquaredKelvin` (pg · m²/(s² · K)), as a `HeatCapacity`.
    pub const PICOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN: HeatCapacity =
        1.0000000000000001e-15 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `FemtogramSquareMetrePerSecondSquaredKelvin` (fg · m²/(s² · K)), as a `HeatCapacity`.
    pub const FEMTOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN: HeatCapacity =
        1.0000000000000001e-18 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `AttogramSquareMetrePerSecondSquaredKelvin` (ag · m²/(s² · K)), as a `HeatCapacity`.
    pub const ATTOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN: HeatCapacity =
        1.0000000000000001e-21 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `ZeptogramSquareMetrePerSecondSquaredKelvin` (zg · m²/(s² · K)), as a `HeatCapacity`.
    pub const ZEPTOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN: HeatCapacity =
        9.9999999999999992e-25 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `YoctogramSquareMetrePerSecondSquaredKelvin` (yg · m²/(s² · K)), as a `HeatCapacity`.
    pub const YOCTOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN: HeatCapacity =
        9.9999999999999986e-28 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `YottajoulePerKelvin` (YJ/K), as a `HeatCapacity`.
    pub const YOTTAJOULE_PER_KELVIN: HeatCapacity =
        9.9999999999999998e+23 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `ZettajoulePerKelvin` (ZJ/K), as a `HeatCapacity`.
    pub const ZETTAJOULE_PER_KELVIN: HeatCapacity =
        1e+21 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `ExajoulePerKelvin` (EJ/K), as a `HeatCapacity`.
    pub const EXAJOULE_PER_KELVIN: HeatCapacity =
        1e+18 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `PetajoulePerKelvin` (PJ/K), as a `HeatCapacity`.
    pub const PETAJOULE_PER_KELVIN: HeatCapacity =
        1000000000000000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `TerajoulePerKelvin` (TJ/K), as a `HeatCapacity`.
    pub const TERAJOULE_PER_KELVIN: HeatCapacity =
        1000000000000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `GigajoulePerKelvin` (GJ/K), as a `HeatCapacity`.
    pub const GIGAJOULE_PER_KELVIN: HeatCapacity =
        1000000000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `MegajoulePerKelvin` (MJ/K), as a `HeatCapacity`.
    pub const MEGAJOULE_PER_KELVIN: HeatCapacity =
        1000000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `KilojoulePerKelvin` (kJ/K), as a `HeatCapacity`.
    pub const KILOJOULE_PER_KELVIN: HeatCapacity =
        1000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `HectojoulePerKelvin` (hJ/K), as a `HeatCapacity`.
    pub const HECTOJOULE_PER_KELVIN: HeatCapacity =
        100.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `DecajoulePerKelvin` (daJ/K), as a `HeatCapacity`.
    pub const DECAJOULE_PER_KELVIN: HeatCapacity =
        10.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `JoulePerKelvin` (J/K), as a `HeatCapacity`.
    pub const JOULE_PER_KELVIN: HeatCapacity = KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `DecijoulePerKelvin` (dJ/K), as a `HeatCapacity`.
    pub const DECIJOULE_PER_KELVIN: HeatCapacity =
        0.10000000000000001 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `CentijoulePerKelvin` (cJ/K), as a `HeatCapacity`.
    pub const CENTIJOULE_PER_KELVIN: HeatCapacity =
        0.01 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `MillijoulePerKelvin` (mJ/K), as a `HeatCapacity`.
    pub const MILLIJOULE_PER_KELVIN: HeatCapacity =
        0.001 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `MicrojoulePerKelvin` (µJ/K), as a `HeatCapacity`.
    pub const MICROJOULE_PER_KELVIN: HeatCapacity =
        9.9999999999999995e-07 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `NanojoulePerKelvin` (nJ/K), as a `HeatCapacity`.
    pub const NANOJOULE_PER_KELVIN: HeatCapacity =
        1.0000000000000001e-09 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `PicojoulePerKelvin` (pJ/K), as a `HeatCapacity`.
    pub const PICOJOULE_PER_KELVIN: HeatCapacity =
        9.9999999999999998e-13 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `FemtojoulePerKelvin` (fJ/K), as a `HeatCapacity`.
    pub const FEMTOJOULE_PER_KELVIN: HeatCapacity =
        1.0000000000000001e-15 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `AttojoulePerKelvin` (aJ/K), as a `HeatCapacity`.
    pub const ATTOJOULE_PER_KELVIN: HeatCapacity =
        1.0000000000000001e-18 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `ZeptojoulePerKelvin` (zJ/K), as a `HeatCapacity`.
    pub const ZEPTOJOULE_PER_KELVIN: HeatCapacity =
        9.9999999999999991e-22 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `YoctojoulePerKelvin` (yJ/K), as a `HeatCapacity`.
    pub const YOCTOJOULE_PER_KELVIN: HeatCapacity =
        9.9999999999999992e-25 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `KilojoulePerDegreeCelsius` (kJ/°C), as a `HeatCapacity`.
    pub const KILOJOULE_PER_DEGREE_CELSIUS: HeatCapacity =
        1000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `JoulePerDegreeCelsius` (J/°C), as a `HeatCapacity`.
    pub const JOULE_PER_DEGREE_CELSIUS: HeatCapacity =
        KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `MillijoulePerDegreeCelsius` (mJ/°C), as a `HeatCapacity`.
    pub const MILLIJOULE_PER_DEGREE_CELSIUS: HeatCapacity =
        0.001 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `BtuPerDegreeFahrenheit` (Btu/°F), as a `HeatCapacity`.
    pub const BTU_PER_DEGREE_FAHRENHEIT: HeatCapacity = ((4184.0 * 0.45359237 / 1.8) / (5.0 / 9.0))
        * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `BtuItPerDegreeFahrenheit` (Btu (IT)/°F), as a `HeatCapacity`.
    pub const BTU_IT_PER_DEGREE_FAHRENHEIT: HeatCapacity =
        (1055.05585262 / (5.0 / 9.0)) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `BoltzmannConstant` (k), as a `HeatCapacity`.
    pub const BOLTZMANN_CONSTANT: HeatCapacity =
        1.380649e-23 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `CaloriePerKelvin` (cal/K), as a `HeatCapacity`.
    pub const CALORIE_PER_KELVIN: HeatCapacity =
        4.1840000000000002 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `YottawattPerSquareMetre` (YW/m²), as a `HeatFluxDensity`.
    pub const YOTTAWATT_PER_SQUARE_METRE: HeatFluxDensity =
        9.9999999999999998e+23 * WATT_PER_SQUARE_METRE;

    /// One `ZettawattPerSquareMetre` (ZW/m²), as a `HeatFluxDensity`.
    pub const ZETTAWATT_PER_SQUARE_METRE: HeatFluxDensity = 1e+21 * WATT_PER_SQUARE_METRE;

    /// One `ExawattPerSquareMetre` (EW/m²), as a `HeatFluxDensity`.
    pub const EXAWATT_PER_SQUARE_METRE: HeatFluxDensity = 1e+18 * WATT_PER_SQUARE_METRE;

    /// One `PetawattPerSquareMetre` (PW/m²), as a `HeatFluxDensity`.
    pub const PETAWATT_PER_SQUARE_METRE: HeatFluxDensity =
        1000000000000000.0 * WATT_PER_SQUARE_METRE;

    /// One `TerawattPerSquareMetre` (TW/m²), as a `HeatFluxDensity`.
    pub const TERAWATT_PER_SQUARE_METRE: HeatFluxDensity = 1000000000000.0 * WATT_PER_SQUARE_METRE;

    /// One `GigawattPerSquareMetre` (GW/m²), as a `HeatFluxDensity`.
    pub const GIGAWATT_PER_SQUARE_METRE: HeatFluxDensity = 1000000000.0 * WATT_PER_SQUARE_METRE;

    /// One `MegawattPerSquareMetre` (MW/m²), as a `HeatFluxDensity`.
    pub const MEGAWATT_PER_SQUARE_METRE: HeatFluxDensity = 1000000.0 * WATT_PER_SQUARE_METRE;

    /// One `KilowattPerSquareMetre` (kW/m²), as a `HeatFluxDensity`.
    pub const KILOWATT_PER_SQUARE_METRE: HeatFluxDensity = 1000.0 * WATT_PER_SQUARE_METRE;

    /// One `HectowattPerSquareMetre` (hW/m²), as a `HeatFluxDensity`.
    pub const HECTOWATT_PER_SQUARE_METRE: HeatFluxDensity = 100.0 * WATT_PER_SQUARE_METRE;

    /// One `DecawattPerSquareMetre` (daW/m²), as a `HeatFluxDensity`.
    pub const DECAWATT_PER_SQUARE_METRE: HeatFluxDensity = 10.0 * WATT_PER_SQUARE_METRE;

    /// One `WattPerSquareMetre` (W/m²), as a `HeatFluxDensity`.
    pub const WATT_PER_SQUARE_METRE: HeatFluxDensity = KILOGRAM / (SECOND * SECOND * SECOND);

    /// One `DeciwattPerSquareMetre` (dW/m²), as a `HeatFluxDensity`.
    pub const DECIWATT_PER_SQUARE_METRE: HeatFluxDensity =
        0.10000000000000001 * WATT_PER_SQUARE_METRE;

    /// One `CentiwattPerSquareMetre` (cW/m²), as a `HeatFluxDensity`.
    pub const CENTIWATT_PER_SQUARE_METRE: HeatFluxDensity = 0.01 * WATT_PER_SQUARE_METRE;

    /// One `MilliwattPerSquareMetre` (mW/m²), as a `HeatFluxDensity`.
    pub const MILLIWATT_PER_SQUARE_METRE: HeatFluxDensity = 0.001 * WATT_PER_SQUARE_METRE;

    /// One `MicrowattPerSquareMetre` (µW/m²), as a `HeatFluxDensity`.
    pub const MICROWATT_PER_SQUARE_METRE: HeatFluxDensity =
        9.9999999999999995e-07 * WATT_PER_SQUARE_METRE;

    /// One `NanowattPerSquareMetre` (nW/m²), as a `HeatFluxDensity`.
    pub const NANOWATT_PER_SQUARE_METRE: HeatFluxDensity =
        1.0000000000000001e-09 * WATT_PER_SQUARE_METRE;

    /// One `PicowattPerSquareMetre` (pW/m²), as a `HeatFluxDensity`.
    pub const PICOWATT_PER_SQUARE_METRE: HeatFluxDensity =
        9.9999999999999998e-13 * WATT_PER_SQUARE_METRE;

    /// One `FemtowattPerSquareMetre` (fW/m²), as a `HeatFluxDensity`.
    pub const FEMTOWATT_PER_SQUARE_METRE: HeatFluxDensity =
        1.0000000000000001e-15 * WATT_PER_SQUARE_METRE;

    /// One `AttowattPerSquareMetre` (aW/m²), as a `HeatFluxDensity`.
    pub const ATTOWATT_PER_SQUARE_METRE: HeatFluxDensity =
        1.0000000000000001e-18 * WATT_PER_SQUARE_METRE;

    /// One `ZeptowattPerSquareMetre` (zW/m²), as a `HeatFluxDensity`.
    pub const ZEPTOWATT_PER_SQUARE_METRE: HeatFluxDensity =
        9.9999999999999991e-22 * WATT_PER_SQUARE_METRE;

    /// One `YoctowattPerSquareMetre` (yW/m²), as a `HeatFluxDensity`.
    pub const YOCTOWATT_PER_SQUARE_METRE: HeatFluxDensity =
        9.9999999999999992e-25 * WATT_PER_SQUARE_METRE;

    /// One `YottagramPerSecondCubedKelvin` (Yg/(s³ · K)), as a `HeatTransfer`.
    pub const YOTTAGRAM_PER_SECOND_CUBED_KELVIN: HeatTransfer =
        1e+21 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `ZettagramPerSecondCubedKelvin` (Zg/(s³ · K)), as a `HeatTransfer`.
    pub const ZETTAGRAM_PER_SECOND_CUBED_KELVIN: HeatTransfer =
        1e+18 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `ExagramPerSecondCubedKelvin` (Eg/(s³ · K)), as a `HeatTransfer`.
    pub const EXAGRAM_PER_SECOND_CUBED_KELVIN: HeatTransfer =
        1000000000000000.0 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `PetagramPerSecondCubedKelvin` (Pg/(s³ · K)), as a `HeatTransfer`.
    pub const PETAGRAM_PER_SECOND_CUBED_KELVIN: HeatTransfer =
        1000000000000.0 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `TeragramPerSecondCubedKelvin` (Tg/(s³ · K)), as a `HeatTransfer`.
    pub const TERAGRAM_PER_SECOND_CUBED_KELVIN: HeatTransfer =
        1000000000.0 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `GigagramPerSecondCubedKelvin` (Gg/(s³ · K)), as a `HeatTransfer`.
    pub const GIGAGRAM_PER_SECOND_CUBED_KELVIN: HeatTransfer =
        1000000.0 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `MegagramPerSecondCubedKelvin` (Mg/(s³ · K)), as a `HeatTransfer`.
    pub const MEGAGRAM_PER_SECOND_CUBED_KELVIN: HeatTransfer =
        1000.0 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `KilogramPerSecondCubedKelvin` (kg/(s³ · K)), as a `HeatTransfer`.
    pub const KILOGRAM_PER_SECOND_CUBED_KELVIN: HeatTransfer =
        KILOGRAM / (SECOND * SECOND * SECOND * KELVIN);

    /// One `HectogramPerSecondCubedKelvin` (hg/(s³ · K)), as a `HeatTransfer`.
    pub const HECTOGRAM_PER_SECOND_CUBED_KELVIN: HeatTransfer =
        0.10000000000000001 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `DecagramPerSecondCubedKelvin` (dag/(s³ · K)), as a `HeatTransfer`.
    pub const DECAGRAM_PER_SECOND_CUBED_KELVIN: HeatTransfer =
        0.01 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `GramPerSecondCubedKelvin` (g/(s³ · K)), as a `HeatTransfer`.
    pub const GRAM_PER_SECOND_CUBED_KELVIN: HeatTransfer = 0.001 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `DecigramPerSecondCubedKelvin` (dg/(s³ · K)), as a `HeatTransfer`.
    pub const DECIGRAM_PER_SECOND_CUBED_KELVIN: HeatTransfer =
        0.0001 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `CentigramPerSecondCubedKelvin` (cg/(s³ · K)), as a `HeatTransfer`.
    pub const CENTIGRAM_PER_SECOND_CUBED_KELVIN: HeatTransfer =
        1.0000000000000001e-05 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `MilligramPerSecondCubedKelvin` (mg/(s³ · K)), as a `HeatTransfer`.
    pub const MILLIGRAM_PER_SECOND_CUBED_KELVIN: HeatTransfer =
        9.9999999999999995e-07 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `MicrogramPerSecondCubedKelvin` (µg/(s³ · K)), as a `HeatTransfer`.
    pub const MICROGRAM_PER_SECOND_CUBED_KELVIN: HeatTransfer =
        9.9999999999999986e-10 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `NanogramPerSecondCubedKelvin` (ng/(s³ · K)), as a `HeatTransfer`.
    pub const NANOGRAM_PER_SECOND_CUBED_KELVIN: HeatTransfer =
        9.9999999999999998e-13 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `PicogramPerSecondCubedKelvin` (pg/(s³ · K)), as a `HeatTransfer`.
    pub const PICOGRAM_PER_SECOND_CUBED_KELVIN: HeatTransfer =
        1.0000000000000001e-15 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `FemtogramPerSecondCubedKelvin` (fg/(s³ · K)), as a `HeatTransfer`.
    pub const FEMTOGRAM_PER_SECOND_CUBED_KELVIN: HeatTransfer =
        1.0000000000000001e-18 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `AttogramPerSecondCubedKelvin` (ag/(s³ · K)), as a `HeatTransfer`.
    pub const ATTOGRAM_PER_SECOND_CUBED_KELVIN: HeatTransfer =
        1.0000000000000001e-21 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `ZeptogramPerSecondCubedKelvin` (zg/(s³ · K)), as a `HeatTransfer`.
    pub const ZEPTOGRAM_PER_SECOND_CUBED_KELVIN: HeatTransfer =
        9.9999999999999992e-25 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `YoctogramPerSecondCubedKelvin` (yg/(s³ · K)), as a `HeatTransfer`.
    pub const YOCTOGRAM_PER_SECOND_CUBED_KELVIN: HeatTransfer =
        9.9999999999999986e-28 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `KilogramPerSecondCubedDegreeCelsius` (kg/(s³ · °C)), as a `HeatTransfer`.
    pub const KILOGRAM_PER_SECOND_CUBED_DEGREE_CELSIUS: HeatTransfer =
        KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `YottawattPerSquareMetreKelvin` (YW/(m² · K)), as a `HeatTransfer`.
    pub const YOTTAWATT_PER_SQUARE_METRE_KELVIN: HeatTransfer =
        9.9999999999999998e+23 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `ZettawattPerSquareMetreKelvin` (ZW/(m² · K)), as a `HeatTransfer`.
    pub const ZETTAWATT_PER_SQUARE_METRE_KELVIN: HeatTransfer =
        1e+21 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `ExawattPerSquareMetreKelvin` (EW/(m² · K)), as a `HeatTransfer`.
    pub const EXAWATT_PER_SQUARE_METRE_KELVIN: HeatTransfer =
        1e+18 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `PetawattPerSquareMetreKelvin` (PW/(m² · K)), as a `HeatTransfer`.
    pub const PETAWATT_PER_SQUARE_METRE_KELVIN: HeatTransfer =
        1000000000000000.0 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `TerawattPerSquareMetreKelvin` (TW/(m² · K)), as a `HeatTransfer`.
    pub const TERAWATT_PER_SQUARE_METRE_KELVIN: HeatTransfer =
        1000000000000.0 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `GigawattPerSquareMetreKelvin` (GW/(m² · K)), as a `HeatTransfer`.
    pub const GIGAWATT_PER_SQUARE_METRE_KELVIN: HeatTransfer =
        1000000000.0 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `MegawattPerSquareMetreKelvin` (MW/(m² · K)), as a `HeatTransfer`.
    pub const MEGAWATT_PER_SQUARE_METRE_KELVIN: HeatTransfer =
        1000000.0 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `KilowattPerSquareMetreKelvin` (kW/(m² · K)), as a `HeatTransfer`.
    pub const KILOWATT_PER_SQUARE_METRE_KELVIN: HeatTransfer =
        1000.0 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `HectowattPerSquareMetreKelvin` (hW/(m² · K)), as a `HeatTransfer`.
    pub const HECTOWATT_PER_SQUARE_METRE_KELVIN: HeatTransfer =
        100.0 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `DecawattPerSquareMetreKelvin` (daW/(m² · K)), as a `HeatTransfer`.
    pub const DECAWATT_PER_SQUARE_METRE_KELVIN: HeatTransfer =
        10.0 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `WattPerSquareMetreKelvin` (W/(m² · K)), as a `HeatTransfer`.
    pub const WATT_PER_SQUARE_METRE_KELVIN: HeatTransfer = KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `DeciwattPerSquareMetreKelvin` (dW/(m² · K)), as a `HeatTransfer`.
    pub const DECIWATT_PER_SQUARE_METRE_KELVIN: HeatTransfer =
        0.10000000000000001 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `CentiwattPerSquareMetreKelvin` (cW/(m² · K)), as a `HeatTransfer`.
    pub const CENTIWATT_PER_SQUARE_METRE_KELVIN: HeatTransfer =
        0.01 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `MilliwattPerSquareMetreKelvin` (mW/(m² · K)), as a `HeatTransfer`.
    pub const MILLIWATT_PER_SQUARE_METRE_KELVIN: HeatTransfer =
        0.001 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `MicrowattPerSquareMetreKelvin` (µW/(m² · K)), as a `HeatTransfer`.
    pub const MICROWATT_PER_SQUARE_METRE_KELVIN: HeatTransfer =
        9.9999999999999995e-07 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `NanowattPerSquareMetreKelvin` (nW/(m² · K)), as a `HeatTransfer`.
    pub const NANOWATT_PER_SQUARE_METRE_KELVIN: HeatTransfer =
        1.0000000000000001e-09 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `PicowattPerSquareMetreKelvin` (pW/(m² · K)), as a `HeatTransfer`.
    pub const PICOWATT_PER_SQUARE_METRE_KELVIN: HeatTransfer =
        9.9999999999999998e-13 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `FemtowattPerSquareMetreKelvin` (fW/(m² · K)), as a `HeatTransfer`.
    pub const FEMTOWATT_PER_SQUARE_METRE_KELVIN: HeatTransfer =
        1.0000000000000001e-15 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `AttowattPerSquareMetreKelvin` (aW/(m² · K)), as a `HeatTransfer`.
    pub const ATTOWATT_PER_SQUARE_METRE_KELVIN: HeatTransfer =
        1.0000000000000001e-18 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `ZeptowattPerSquareMetreKelvin` (zW/(m² · K)), as a `HeatTransfer`.
    pub const ZEPTOWATT_PER_SQUARE_METRE_KELVIN: HeatTransfer =
        9.9999999999999991e-22 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `YoctowattPerSquareMetreKelvin` (yW/(m² · K)), as a `HeatTransfer`.
    pub const YOCTOWATT_PER_SQUARE_METRE_KELVIN: HeatTransfer =
        9.9999999999999992e-25 * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `WattPerSquareMetreDegreeCelsius` (W/(m² · °C)), as a `HeatTransfer`.
    pub const WATT_PER_SQUARE_METRE_DEGREE_CELSIUS: HeatTransfer = KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `JoulePerSecondSquareMetreKelvin` (J/(s · m² · K)), as a `HeatTransfer`.
    pub const JOULE_PER_SECOND_SQUARE_METRE_KELVIN: HeatTransfer = KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `JoulePerSecondSquareMetreDegreeCelsius` (J/(s · m² · °C)), as a `HeatTransfer`.
    pub const JOULE_PER_SECOND_SQUARE_METRE_DEGREE_CELSIUS: HeatTransfer =
        KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `BtuItPerHourSquareFootDegreeFahrenheit` (Btu (IT)/(hr · ft² · °F)), as a `HeatTransfer`.
    pub const BTU_IT_PER_HOUR_SQUARE_FOOT_DEGREE_FAHRENHEIT: HeatTransfer = (1055.05585262
        / (3600.0 * 0.3048 * 0.3048 * (5.0 / 9.0)))
        * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `BtuItPerHourSquareInchDegreeFahrenheit` (Btu (IT)/(hr · in² · °F)), as a `HeatTransfer`.
    pub const BTU_IT_PER_HOUR_SQUARE_INCH_DEGREE_FAHRENHEIT: HeatTransfer = (1055.05585262
        / (3600.0 * 0.0254 * 0.0254 * (5.0 / 9.0)))
        * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `BtuItPerMinuteSquareFootDegreeFahrenheit` (Btu (IT)/(min · ft² · °F)), as a `HeatTransfer`.
    pub const BTU_IT_PER_MINUTE_SQUARE_FOOT_DEGREE_FAHRENHEIT: HeatTransfer =
        (1055.05585262 / (60.0 * 0.3048 * 0.3048 * (5.0 / 9.0))) * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `BtuItPerMinuteSquareInchDegreeFahrenheit` (Btu (IT)/(min · in² · °F)), as a `HeatTransfer`.
    pub const BTU_IT_PER_MINUTE_SQUARE_INCH_DEGREE_FAHRENHEIT: HeatTransfer =
        (1055.05585262 / (60.0 * 0.0254 * 0.0254 * (5.0 / 9.0))) * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `BtuItPerSecondSquareFootDegreeFahrenheit` (Btu (IT)/(s · ft² · °F)), as a `HeatTransfer`.
    pub const BTU_IT_PER_SECOND_SQUARE_FOOT_DEGREE_FAHRENHEIT: HeatTransfer =
        (1055.05585262 / (0.3048 * 0.3048 * (5.0 / 9.0))) * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `BtuItPerSecondSquareInchDegreeFahrenheit` (Btu (IT)/(s · in² · °F)), as a `HeatTransfer`.
    pub const BTU_IT_PER_SECOND_SQUARE_INCH_DEGREE_FAHRENHEIT: HeatTransfer =
        (1055.05585262 / (0.0254 * 0.0254 * (5.0 / 9.0))) * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `BtuPerHourSquareFootDegreeFahrenheit` (Btu/(hr · ft² · °F)), as a `HeatTransfer`.
    pub const BTU_PER_HOUR_SQUARE_FOOT_DEGREE_FAHRENHEIT: HeatTransfer =
        ((4184.0 * 0.45359237 / 1.8) / (3600.0 * 0.3048 * 0.3048 * (5.0 / 9.0)))
            * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `BtuPerHourSquareInchDegreeFahrenheit` (Btu/(hr · in² · °F)), as a `HeatTransfer`.
    pub const BTU_PER_HOUR_SQUARE_INCH_DEGREE_FAHRENHEIT: HeatTransfer =
        ((4184.0 * 0.45359237 / 1.8) / (3600.0 * 0.0254 * 0.0254 * (5.0 / 9.0)))
            * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `BtuPerMinuteSquareFootDegreeFahrenheit` (Btu/(min · ft² · °F)), as a `HeatTransfer`.
    pub const BTU_PER_MINUTE_SQUARE_FOOT_DEGREE_FAHRENHEIT: HeatTransfer =
        ((4184.0 * 0.45359237 / 1.8) / (60.0 * 0.3048 * 0.3048 * (5.0 / 9.0)))
            * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `BtuPerMinuteSquareInchDegreeFahrenheit` (Btu/(min · in² · °F)), as a `HeatTransfer`.
    pub const BTU_PER_MINUTE_SQUARE_INCH_DEGREE_FAHRENHEIT: HeatTransfer =
        ((4184.0 * 0.45359237 / 1.8) / (60.0 * 0.0254 * 0.0254 * (5.0 / 9.0)))
            * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `BtuPerSecondSquareFootDegreeFahrenheit` (Btu/(s · ft² · °F)), as a `HeatTransfer`.
    pub const BTU_PER_SECOND_SQUARE_FOOT_DEGREE_FAHRENHEIT: HeatTransfer =
        ((4184.0 * 0.45359237 / 1.8) / (0.3048 * 0.3048 * (5.0 / 9.0)))
            * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `BtuPerSecondSquareInchDegreeFahrenheit` (Btu/(s · in² · °F)), as a `HeatTransfer`.
    pub const BTU_PER_SECOND_SQUARE_INCH_DEGREE_FAHRENHEIT: HeatTransfer =
        ((4184.0 * 0.45359237 / 1.8) / (0.0254 * 0.0254 * (5.0 / 9.0)))
            * KILOGRAM_PER_SECOND_CUBED_KELVIN;

    /// One `YottagramMetreSquaredPerSecondCubedKelvin` (Yg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const YOTTAGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN: ThermalConductance =
        1e+21 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `ZettagramMetreSquaredPerSecondCubedKelvin` (Zg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const ZETTAGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN: ThermalConductance =
        1e+18 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `ExagramMetreSquaredPerSecondCubedKelvin` (Eg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const EXAGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN: ThermalConductance =
        1000000000000000.0 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `PetagramMetreSquaredPerSecondCubedKelvin` (Pg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const PETAGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN: ThermalConductance =
        1000000000000.0 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `TeragramMetreSquaredPerSecondCubedKelvin` (Tg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const TERAGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN: ThermalConductance =
        1000000000.0 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `GigagramMetreSquaredPerSecondCubedKelvin` (Gg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const GIGAGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN: ThermalConductance =
        1000000.0 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `MegagramMetreSquaredPerSecondCubedKelvin` (Mg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const MEGAGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN: ThermalConductance =
        1000.0 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `KilogramMetreSquaredPerSecondCubedKelvin` (kg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN: ThermalConductance =
        METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * KELVIN);

    /// One `HectogramMetreSquaredPerSecondCubedKelvin` (hg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const HECTOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN: ThermalConductance =
        0.10000000000000001 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `DecagramMetreSquaredPerSecondCubedKelvin` (dag · m²/(s³ · K)), as a `ThermalConductance`.
    pub const DECAGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN: ThermalConductance =
        0.01 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `GramMetreSquaredPerSecondCubedKelvin` (g · m/(s³ · K)), as a `ThermalConductance`.
    pub const GRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN: ThermalConductance =
        0.001 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `DecigramMetreSquaredPerSecondCubedKelvin` (dg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const DECIGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN: ThermalConductance =
        0.0001 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `CentigramMetreSquaredPerSecondCubedKelvin` (cg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const CENTIGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN: ThermalConductance =
        1.0000000000000001e-05 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `MilligramMetreSquaredPerSecondCubedKelvin` (mg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const MILLIGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN: ThermalConductance =
        9.9999999999999995e-07 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `MicrogramMetreSquaredPerSecondCubedKelvin` (µg · m/(s³ · K)), as a `ThermalConductance`.
    pub const MICROGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN: ThermalConductance =
        9.9999999999999986e-10 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `NanogramMetreSquaredPerSecondCubedKelvin` (ng · m²/(s³ · K)), as a `ThermalConductance`.
    pub const NANOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN: ThermalConductance =
        9.9999999999999998e-13 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `PicogramMetreSquaredPerSecondCubedKelvin` (pg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const PICOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN: ThermalConductance =
        1.0000000000000001e-15 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `FemtogramMetreSquaredPerSecondCubedKelvin` (fg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const FEMTOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN: ThermalConductance =
        1.0000000000000001e-18 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `AttogramMetreSquaredPerSecondCubedKelvin` (ag · m²/(s³ · K)), as a `ThermalConductance`.
    pub const ATTOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN: ThermalConductance =
        1.0000000000000001e-21 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `ZeptogramMetreSquaredPerSecondCubedKelvin` (zg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const ZEPTOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN: ThermalConductance =
        9.9999999999999992e-25 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `YoctogramMetreSquaredPerSecondCubedKelvin` (yg · m²/(s³ · K)), as a `ThermalConductance`.
    pub const YOCTOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN: ThermalConductance =
        9.9999999999999986e-28 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `YottawattPerKelvin` (YW/K), as a `ThermalConductance`.
    pub const YOTTAWATT_PER_KELVIN: ThermalConductance =
        9.9999999999999998e+23 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `ZettawattPerKelvin` (ZW/K), as a `ThermalConductance`.
    pub const ZETTAWATT_PER_KELVIN: ThermalConductance =
        1e+21 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `ExawattPerKelvin` (EW/K), as a `ThermalConductance`.
    pub const EXAWATT_PER_KELVIN: ThermalConductance =
        1e+18 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `PetawattPerKelvin` (PW/K), as a `ThermalConductance`.
    pub const PETAWATT_PER_KELVIN: ThermalConductance =
        1000000000000000.0 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `TerawattPerKelvin` (TW/K), as a `ThermalConductance`.
    pub const TERAWATT_PER_KELVIN: ThermalConductance =
        1000000000000.0 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `GigawattPerKelvin` (GW/K), as a `ThermalConductance`.
    pub const GIGAWATT_PER_KELVIN: ThermalConductance =
        1000000000.0 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `MegawattPerKelvin` (MW/K), as a `ThermalConductance`.
    pub const MEGAWATT_PER_KELVIN: ThermalConductance =
        1000000.0 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `KilowattPerKelvin` (kW/K), as a `ThermalConductance`.
    pub const KILOWATT_PER_KELVIN: ThermalConductance =
        1000.0 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `HectowattPerKelvin` (hW/K), as a `ThermalConductance`.
    pub const HECTOWATT_PER_KELVIN: ThermalConductance =
        100.0 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `DecawattPerKelvin` (daW/K), as a `ThermalConductance`.
    pub const DECAWATT_PER_KELVIN: ThermalConductance =
        10.0 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `WattPerKelvin` (W/K), as a `ThermalConductance`.
    pub const WATT_PER_KELVIN: ThermalConductance = KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `DeciwattPerKelvin` (dW/K), as a `ThermalConductance`.
    pub const DECIWATT_PER_KELVIN: ThermalConductance =
        0.10000000000000001 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `CentiwattPerKelvin` (cW/K), as a `ThermalConductance`.
    pub const CENTIWATT_PER_KELVIN: ThermalConductance =
        0.01 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `MilliwattPerKelvin` (mW/K), as a `ThermalConductance`.
    pub const MILLIWATT_PER_KELVIN: ThermalConductance =
        0.001 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `MicrowattPerKelvin` (µW/K), as a `ThermalConductance`.
    pub const MICROWATT_PER_KELVIN: ThermalConductance =
        9.9999999999999995e-07 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `NanowattPerKelvin` (nW/K), as a `ThermalConductance`.
    pub const NANOWATT_PER_KELVIN: ThermalConductance =
        1.0000000000000001e-09 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `PicowattPerKelvin` (pW/K), as a `ThermalConductance`.
    pub const PICOWATT_PER_KELVIN: ThermalConductance =
        9.9999999999999998e-13 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `FemtowattPerKelvin` (fW/K), as a `ThermalConductance`.
    pub const FEMTOWATT_PER_KELVIN: ThermalConductance =
        1.0000000000000001e-15 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `AttowattPerKelvin` (aW/K), as a `ThermalConductance`.
    pub const ATTOWATT_PER_KELVIN: ThermalConductance =
        1.0000000000000001e-18 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `ZeptowattPerKelvin` (zW/K), as a `ThermalConductance`.
    pub const ZEPTOWATT_PER_KELVIN: ThermalConductance =
        9.9999999999999991e-22 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `YoctowattPerKelvin` (yW/K), as a `ThermalConductance`.
    pub const YOCTOWATT_PER_KELVIN: ThermalConductance =
        9.9999999999999992e-25 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `KilogramMetreSquaredPerSecondCubedDegreeCelsius` (kg · m²/(s³ · °C)), as a `ThermalConductance`.
    pub const KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_DEGREE_CELSIUS: ThermalConductance =
        KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `KilowattPerDegreeCelsius` (kW/°C), as a `ThermalConductance`.
    pub const KILOWATT_PER_DEGREE_CELSIUS: ThermalConductance =
        1000.0 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `WattPerMetreDegreeCelsius` (W/°C), as a `ThermalConductance`.
    pub const THERMAL_CONDUCTANCE_WATT_PER_METRE_DEGREE_CELSIUS: ThermalConductance =
        KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `MilliwattPerDegreeCelsius` (mW/°C), as a `ThermalConductance`.
    pub const MILLIWATT_PER_DEGREE_CELSIUS: ThermalConductance =
        0.001 * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN;

    /// One `YottagramMetrePerSecondCubedKelvin` (Yg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const YOTTAGRAM_METRE_PER_SECOND_CUBED_KELVIN: ThermalConductivity =
        1e+21 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `ZettagramMetrePerSecondCubedKelvin` (Zg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const ZETTAGRAM_METRE_PER_SECOND_CUBED_KELVIN: ThermalConductivity =
        1e+18 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `ExagramMetrePerSecondCubedKelvin` (Eg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const EXAGRAM_METRE_PER_SECOND_CUBED_KELVIN: ThermalConductivity =
        1000000000000000.0 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `PetagramMetrePerSecondCubedKelvin` (Pg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const PETAGRAM_METRE_PER_SECOND_CUBED_KELVIN: ThermalConductivity =
        1000000000000.0 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `TeragramMetrePerSecondCubedKelvin` (Tg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const TERAGRAM_METRE_PER_SECOND_CUBED_KELVIN: ThermalConductivity =
        1000000000.0 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `GigagramMetrePerSecondCubedKelvin` (Gg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const GIGAGRAM_METRE_PER_SECOND_CUBED_KELVIN: ThermalConductivity =
        1000000.0 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `MegagramMetrePerSecondCubedKelvin` (Mg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const MEGAGRAM_METRE_PER_SECOND_CUBED_KELVIN: ThermalConductivity =
        1000.0 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `KilogramMetrePerSecondCubedKelvin` (kg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN: ThermalConductivity =
        METRE * KILOGRAM / (SECOND * SECOND * SECOND * KELVIN);

    /// One `HectogramMetrePerSecondCubedKelvin` (hg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const HECTOGRAM_METRE_PER_SECOND_CUBED_KELVIN: ThermalConductivity =
        0.10000000000000001 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `DecagramMetrePerSecondCubedKelvin` (dag · m/(s³ · K)), as a `ThermalConductivity`.
    pub const DECAGRAM_METRE_PER_SECOND_CUBED_KELVIN: ThermalConductivity =
        0.01 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `GramMetrePerSecondCubedKelvin` (g · m/(s³ · K)), as a `ThermalConductivity`.
    pub const GRAM_METRE_PER_SECOND_CUBED_KELVIN: ThermalConductivity =
        0.001 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `DecigramMetrePerSecondCubedKelvin` (dg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const DECIGRAM_METRE_PER_SECOND_CUBED_KELVIN: ThermalConductivity =
        0.0001 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `CentigramMetrePerSecondCubedKelvin` (cg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const CENTIGRAM_METRE_PER_SECOND_CUBED_KELVIN: ThermalConductivity =
        1.0000000000000001e-05 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `MilligramMetrePerSecondCubedKelvin` (mg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const MILLIGRAM_METRE_PER_SECOND_CUBED_KELVIN: ThermalConductivity =
        9.9999999999999995e-07 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `MicrogramMetrePerSecondCubedKelvin` (µg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const MICROGRAM_METRE_PER_SECOND_CUBED_KELVIN: ThermalConductivity =
        9.9999999999999986e-10 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `NanogramMetrePerSecondCubedKelvin` (ng · m/(s³ · K)), as a `ThermalConductivity`.
    pub const NANOGRAM_METRE_PER_SECOND_CUBED_KELVIN: ThermalConductivity =
        9.9999999999999998e-13 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `PicogramMetrePerSecondCubedKelvin` (pg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const PICOGRAM_METRE_PER_SECOND_CUBED_KELVIN: ThermalConductivity =
        1.0000000000000001e-15 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `FemtogramMetrePerSecondCubedKelvin` (fg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const FEMTOGRAM_METRE_PER_SECOND_CUBED_KELVIN: ThermalConductivity =
        1.0000000000000001e-18 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `AttogramMetrePerSecondCubedKelvin` (ag · m/(s³ · K)), as a `ThermalConductivity`.
    pub const ATTOGRAM_METRE_PER_SECOND_CUBED_KELVIN: ThermalConductivity =
        1.0000000000000001e-21 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `ZeptogramMetrePerSecondCubedKelvin` (zg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const ZEPTOGRAM_METRE_PER_SECOND_CUBED_KELVIN: ThermalConductivity =
        9.9999999999999992e-25 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `YoctogramMetrePerSecondCubedKelvin` (yg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const YOCTOGRAM_METRE_PER_SECOND_CUBED_KELVIN: ThermalConductivity =
        9.9999999999999986e-28 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `YottawattPerMetreKelvin` (YW/(m · K)), as a `ThermalConductivity`.
    pub const YOTTAWATT_PER_METRE_KELVIN: ThermalConductivity =
        9.9999999999999998e+23 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `ZettawattPerMetreKelvin` (ZW/(m · K)), as a `ThermalConductivity`.
    pub const ZETTAWATT_PER_METRE_KELVIN: ThermalConductivity =
        1e+21 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `ExawattPerMetreKelvin` (EW/(m · K)), as a `ThermalConductivity`.
    pub const EXAWATT_PER_METRE_KELVIN: ThermalConductivity =
        1e+18 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `PetawattPerMetreKelvin` (PW/(m · K)), as a `ThermalConductivity`.
    pub const PETAWATT_PER_METRE_KELVIN: ThermalConductivity =
        1000000000000000.0 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `TerawattPerMetreKelvin` (TW/(m · K)), as a `ThermalConductivity`.
    pub const TERAWATT_PER_METRE_KELVIN: ThermalConductivity =
        1000000000000.0 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `GigawattPerMetreKelvin` (GW/(m · K)), as a `ThermalConductivity`.
    pub const GIGAWATT_PER_METRE_KELVIN: ThermalConductivity =
        1000000000.0 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `MegawattPerMetreKelvin` (MW/(m · K)), as a `ThermalConductivity`.
    pub const MEGAWATT_PER_METRE_KELVIN: ThermalConductivity =
        1000000.0 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `KilowattPerMetreKelvin` (kW/(m · K)), as a `ThermalConductivity`.
    pub const KILOWATT_PER_METRE_KELVIN: ThermalConductivity =
        1000.0 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `HectowattPerMetreKelvin` (hW/(m · K)), as a `ThermalConductivity`.
    pub const HECTOWATT_PER_METRE_KELVIN: ThermalConductivity =
        100.0 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `DecawattPerMetreKelvin` (daW/(m · K)), as a `ThermalConductivity`.
    pub const DECAWATT_PER_METRE_KELVIN: ThermalConductivity =
        10.0 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `WattPerMetreKelvin` (W/(m · K)), as a `ThermalConductivity`.
    pub const WATT_PER_METRE_KELVIN: ThermalConductivity = KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `DeciwattPerMetreKelvin` (dW/(m · K)), as a `ThermalConductivity`.
    pub const DECIWATT_PER_METRE_KELVIN: ThermalConductivity =
        0.10000000000000001 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `CentiwattPerMetreKelvin` (cW/(m · K)), as a `ThermalConductivity`.
    pub const CENTIWATT_PER_METRE_KELVIN: ThermalConductivity =
        0.01 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `MilliwattPerMetreKelvin` (mW/(m · K)), as a `ThermalConductivity`.
    pub const MILLIWATT_PER_METRE_KELVIN: ThermalConductivity =
        0.001 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `MicrowattPerMetreKelvin` (µW/(m · K)), as a `ThermalConductivity`.
    pub const MICROWATT_PER_METRE_KELVIN: ThermalConductivity =
        9.9999999999999995e-07 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `NanowattPerMetreKelvin` (nW/(m · K)), as a `ThermalConductivity`.
    pub const NANOWATT_PER_METRE_KELVIN: ThermalConductivity =
        1.0000000000000001e-09 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `PicowattPerMetreKelvin` (pW/(m · K)), as a `ThermalConductivity`.
    pub const PICOWATT_PER_METRE_KELVIN: ThermalConductivity =
        9.9999999999999998e-13 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `FemtowattPerMetreKelvin` (fW/(m · K)), as a `ThermalConductivity`.
    pub const FEMTOWATT_PER_METRE_KELVIN: ThermalConductivity =
        1.0000000000000001e-15 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `AttowattPerMetreKelvin` (aW/(m · K)), as a `ThermalConductivity`.
    pub const ATTOWATT_PER_METRE_KELVIN: ThermalConductivity =
        1.0000000000000001e-18 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `ZeptowattPerMetreKelvin` (zW/(m · K)), as a `ThermalConductivity`.
    pub const ZEPTOWATT_PER_METRE_KELVIN: ThermalConductivity =
        9.9999999999999991e-22 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `YoctowattPerMetreKelvin` (yW/(m · K)), as a `ThermalConductivity`.
    pub const YOCTOWATT_PER_METRE_KELVIN: ThermalConductivity =
        9.9999999999999992e-25 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `KilogramMetrePerSecondCubedDegreeCelsius` (kg · m/(s³ · °C)), as a `ThermalConductivity`.
    pub const KILOGRAM_METRE_PER_SECOND_CUBED_DEGREE_CELSIUS: ThermalConductivity =
        KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `KilowattPerMetreDegreeCelsius` (kW/(m · °C)), as a `ThermalConductivity`.
    pub const KILOWATT_PER_METRE_DEGREE_CELSIUS: ThermalConductivity =
        1000.0 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `WattPerMetreDegreeCelsius` (W/(m · °C)), as a `ThermalConductivity`.
    pub const THERMAL_CONDUCTIVITY_WATT_PER_METRE_DEGREE_CELSIUS: ThermalConductivity =
        KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `MilliwattPerMetreDegreeCelsius` (mW/(m · °C)), as a `ThermalConductivity`.
    pub const MILLIWATT_PER_METRE_DEGREE_CELSIUS: ThermalConductivity =
        0.001 * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN;

    /// One `KelvinPerYottawatt` (K/YW), as a `ThermalResistance`.
    pub const KELVIN_PER_YOTTAWATT: ThermalResistance = 1.0000000000000001e-24 * KELVIN_PER_WATT;

    /// One `KelvinPerZettawatt` (K/ZW), as a `ThermalResistance`.
    pub const KELVIN_PER_ZETTAWATT: ThermalResistance = 9.9999999999999991e-22 * KELVIN_PER_WATT;

    /// One `KelvinPerExawatt` (K/EW), as a `ThermalResistance`.
    pub const KELVIN_PER_EXAWATT: ThermalResistance = 1.0000000000000001e-18 * KELVIN_PER_WATT;

    /// One `KelvinPerPetawatt` (K/PW), as a `ThermalResistance`.
    pub const KELVIN_PER_PETAWATT: ThermalResistance = 1.0000000000000001e-15 * KELVIN_PER_WATT;

    /// One `KelvinPerTerawatt` (K/TW), as a `ThermalResistance`.
    pub const KELVIN_PER_TERAWATT: ThermalResistance = 9.9999999999999998e-13 * KELVIN_PER_WATT;

    /// One `KelvinPerGigawatt` (K/GW), as a `ThermalResistance`.
    pub const KELVIN_PER_GIGAWATT: ThermalResistance = 1.0000000000000001e-09 * KELVIN_PER_WATT;

    /// One `KelvinPerMegawatt` (K/MW), as a `ThermalResistance`.
    pub const KELVIN_PER_MEGAWATT: ThermalResistance = 9.9999999999999995e-07 * KELVIN_PER_WATT;

    /// One `KelvinPerKilowatt` (K/kw), as a `ThermalResistance`.
    pub const KELVIN_PER_KILOWATT: ThermalResistance = 0.001 * KELVIN_PER_WATT;

    /// One `KelvinPerHectowatt` (K/hW), as a `ThermalResistance`.
    pub const KELVIN_PER_HECTOWATT: ThermalResistance = 0.01 * KELVIN_PER_WATT;

    /// One `KelvinPerDecawatt` (K/daW), as a `ThermalResistance`.
    pub const KELVIN_PER_DECAWATT: ThermalResistance = 0.10000000000000001 * KELVIN_PER_WATT;

    /// One `KelvinPerWatt` (K/W), as a `ThermalResistance`.
    pub const KELVIN_PER_WATT: ThermalResistance =
        SECOND * SECOND * SECOND * KELVIN / (METRE * METRE * KILOGRAM);

    /// One `KelvinPerDeciwatt` (K/dW), as a `ThermalResistance`.
    pub const KELVIN_PER_DECIWATT: ThermalResistance = 10.0 * KELVIN_PER_WATT;

    /// One `KelvinPerCentiwatt` (K/cW), as a `ThermalResistance`.
    pub const KELVIN_PER_CENTIWATT: ThermalResistance = 100.0 * KELVIN_PER_WATT;

    /// One `KelvinPerMilliwatt` (K/mW), as a `ThermalResistance`.
    pub const KELVIN_PER_MILLIWATT: ThermalResistance = 1000.0 * KELVIN_PER_WATT;

    /// One `KelvinPerMicrowatt` (K/µW), as a `ThermalResistance`.
    pub const KELVIN_PER_MICROWATT: ThermalResistance = 1000000.0 * KELVIN_PER_WATT;

    /// One `KelvinPerNanowatt` (K/nW), as a `ThermalResistance`.
    pub const KELVIN_PER_NANOWATT: ThermalResistance = 999999999.99999988 * KELVIN_PER_WATT;

    /// One `KelvinPerPicowatt` (K/pW), as a `ThermalResistance`.
    pub const KELVIN_PER_PICOWATT: ThermalResistance = 1000000000000.0 * KELVIN_PER_WATT;

    /// One `KelvinPerFemtowatt` (K/fW), as a `ThermalResistance`.
    pub const KELVIN_PER_FEMTOWATT: ThermalResistance = 999999999999999.88 * KELVIN_PER_WATT;

    /// One `KelvinPerAttowatt` (K/aW), as a `ThermalResistance`.
    pub const KELVIN_PER_ATTOWATT: ThermalResistance = 9.9999999999999987e+17 * KELVIN_PER_WATT;

    /// One `KelvinPerZeptowatt` (K/zW), as a `ThermalResistance`.
    pub const KELVIN_PER_ZEPTOWATT: ThermalResistance = 1.0000000000000001e+21 * KELVIN_PER_WATT;

    /// One `KelvinPerYoctowatt` (K/yW), as a `ThermalResistance`.
    pub const KELVIN_PER_YOCTOWATT: ThermalResistance = 1.0000000000000001e+24 * KELVIN_PER_WATT;

    /// One `SquareKilometrePerSecondSquaredKelvin` (km²/(s² · K)), as a `SpecificHeatCapacity`.
    pub const SQUARE_KILOMETRE_PER_SECOND_SQUARED_KELVIN: SpecificHeatCapacity =
        1000000.0 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `SquareMetrePerSecondSquaredKelvin` (m²/(s² · K)), as a `SpecificHeatCapacity`.
    pub const SQUARE_METRE_PER_SECOND_SQUARED_KELVIN: SpecificHeatCapacity =
        METRE * METRE / (SECOND * SECOND * KELVIN);

    /// One `SquareCentimetrePerSecondSquaredKelvin` (cm²/(s² · K)), as a `SpecificHeatCapacity`.
    pub const SQUARE_CENTIMETRE_PER_SECOND_SQUARED_KELVIN: SpecificHeatCapacity =
        0.0001 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `SquareMillimetrePerSecondSquaredKelvin` (mm²/(s² · K)), as a `SpecificHeatCapacity`.
    pub const SQUARE_MILLIMETRE_PER_SECOND_SQUARED_KELVIN: SpecificHeatCapacity =
        9.9999999999999995e-07 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `SquareMicrometrePerSecondSquaredKelvin` (µm²/(s² · K)), as a `SpecificHeatCapacity`.
    pub const SQUARE_MICROMETRE_PER_SECOND_SQUARED_KELVIN: SpecificHeatCapacity =
        9.9999999999999998e-13 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `YottajoulePerKilogramKelvin` (YJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const YOTTAJOULE_PER_KILOGRAM_KELVIN: SpecificHeatCapacity =
        9.9999999999999998e+23 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `ZettajoulePerKilogramKelvin` (ZJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const ZETTAJOULE_PER_KILOGRAM_KELVIN: SpecificHeatCapacity =
        1e+21 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `ExajoulePerKilogramKelvin` (EJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const EXAJOULE_PER_KILOGRAM_KELVIN: SpecificHeatCapacity =
        1e+18 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `PetajoulePerKilogramKelvin` (PJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const PETAJOULE_PER_KILOGRAM_KELVIN: SpecificHeatCapacity =
        1000000000000000.0 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `TerajoulePerKilogramKelvin` (TJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const TERAJOULE_PER_KILOGRAM_KELVIN: SpecificHeatCapacity =
        1000000000000.0 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `GigajoulePerKilogramKelvin` (GJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const GIGAJOULE_PER_KILOGRAM_KELVIN: SpecificHeatCapacity =
        1000000000.0 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `MegajoulePerKilogramKelvin` (MJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const MEGAJOULE_PER_KILOGRAM_KELVIN: SpecificHeatCapacity =
        1000000.0 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `KilojoulePerKilogramKelvin` (kJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const KILOJOULE_PER_KILOGRAM_KELVIN: SpecificHeatCapacity =
        1000.0 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `HectojoulePerKilogramKelvin` (hJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const HECTOJOULE_PER_KILOGRAM_KELVIN: SpecificHeatCapacity =
        100.0 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `DecajoulePerKilogramKelvin` (daJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const DECAJOULE_PER_KILOGRAM_KELVIN: SpecificHeatCapacity =
        10.0 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `JoulePerKilogramKelvin` (J/(kg · K)), as a `SpecificHeatCapacity`.
    pub const JOULE_PER_KILOGRAM_KELVIN: SpecificHeatCapacity =
        SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `DecijoulePerKilogramKelvin` (dJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const DECIJOULE_PER_KILOGRAM_KELVIN: SpecificHeatCapacity =
        0.10000000000000001 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `CentijoulePerKilogramKelvin` (cJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const CENTIJOULE_PER_KILOGRAM_KELVIN: SpecificHeatCapacity =
        0.01 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `MillijoulePerKilogramKelvin` (mJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const MILLIJOULE_PER_KILOGRAM_KELVIN: SpecificHeatCapacity =
        0.001 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `MicrojoulePerKilogramKelvin` (µJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const MICROJOULE_PER_KILOGRAM_KELVIN: SpecificHeatCapacity =
        9.9999999999999995e-07 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `NanojoulePerKilogramKelvin` (nJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const NANOJOULE_PER_KILOGRAM_KELVIN: SpecificHeatCapacity =
        1.0000000000000001e-09 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `PicojoulePerKilogramKelvin` (pJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const PICOJOULE_PER_KILOGRAM_KELVIN: SpecificHeatCapacity =
        9.9999999999999998e-13 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `FemtojoulePerKilogramKelvin` (fJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const FEMTOJOULE_PER_KILOGRAM_KELVIN: SpecificHeatCapacity =
        1.0000000000000001e-15 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `AttojoulePerKilogramKelvin` (aJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const ATTOJOULE_PER_KILOGRAM_KELVIN: SpecificHeatCapacity =
        1.0000000000000001e-18 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `ZeptojoulePerKilogramKelvin` (zJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const ZEPTOJOULE_PER_KILOGRAM_KELVIN: SpecificHeatCapacity =
        9.9999999999999991e-22 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `YoctojoulePerKilogramKelvin` (yJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const YOCTOJOULE_PER_KILOGRAM_KELVIN: SpecificHeatCapacity =
        9.9999999999999992e-25 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `KilojoulePerKilogramDegreeCelsius` (kJ/(kg · °C)), as a `SpecificHeatCapacity`.
    pub const KILOJOULE_PER_KILOGRAM_DEGREE_CELSIUS: SpecificHeatCapacity =
        1000.0 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `KilojoulePerGramDegreeCelsius` (kJ/(g · °C)), as a `SpecificHeatCapacity`.
    pub const KILOJOULE_PER_GRAM_DEGREE_CELSIUS: SpecificHeatCapacity =
        1000000.0 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `JoulePerKilogramDegreeCelsius` (J/(kg · °C)), as a `SpecificHeatCapacity`.
    pub const JOULE_PER_KILOGRAM_DEGREE_CELSIUS: SpecificHeatCapacity =
        SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `JoulePerGramDegreeCelsius` (J/(g · °C)), as a `SpecificHeatCapacity`.
    pub const JOULE_PER_GRAM_DEGREE_CELSIUS: SpecificHeatCapacity =
        1000.0 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `MillijoulePerKilogramDegreeCelsius` (mJ/(kg · °C)), as a `SpecificHeatCapacity`.
    pub const MILLIJOULE_PER_KILOGRAM_DEGREE_CELSIUS: SpecificHeatCapacity =
        0.001 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `MillijoulePerGramDegreeCelsius` (mJ/(g · °C)), as a `SpecificHeatCapacity`.
    pub const MILLIJOULE_PER_GRAM_DEGREE_CELSIUS: SpecificHeatCapacity =
        SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `BtuPerOunceDegreeFahrenheit` (Btu/(oz · °F)), as a `SpecificHeatCapacity`.
    pub const BTU_PER_OUNCE_DEGREE_FAHRENHEIT: SpecificHeatCapacity =
        (16.0 * 4184.0) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `BtuItPerOunceDegreeFahrenheit` (Btu (IT)/(oz · °F)), as a `SpecificHeatCapacity`.
    pub const BTU_IT_PER_OUNCE_DEGREE_FAHRENHEIT: SpecificHeatCapacity =
        (16.0 * 4186.8) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `BtuPerPoundDegreeFahrenheit` (Btu/(lb · °F)), as a `SpecificHeatCapacity`.
    pub const BTU_PER_POUND_DEGREE_FAHRENHEIT: SpecificHeatCapacity =
        4184.0 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `BtuItPerPoundDegreeFahrenheit` (Btu (IT)/(lb · °F)), as a `SpecificHeatCapacity`.
    pub const BTU_IT_PER_POUND_DEGREE_FAHRENHEIT: SpecificHeatCapacity =
        4186.8 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `BtuPerTonDegreeFahrenheit` (Btu/(t · °F)), as a `SpecificHeatCapacity`.
    pub const BTU_PER_TON_DEGREE_FAHRENHEIT: SpecificHeatCapacity = ((4184.0 * 0.45359237 / 1.8)
        / (1000.0 * (5.0 / 9.0)))
        * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `BtuItPerTonDegreeFahrenheit` (Btu (IT)/(t · °F)), as a `SpecificHeatCapacity`.
    pub const BTU_IT_PER_TON_DEGREE_FAHRENHEIT: SpecificHeatCapacity =
        (1055.05585262 / (1000.0 * (5.0 / 9.0))) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `CaloriePerKilogramKelvin` (cal/(kg · K)), as a `SpecificHeatCapacity`.
    pub const CALORIE_PER_KILOGRAM_KELVIN: SpecificHeatCapacity =
        4.1840000000000002 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `CaloriePerGramKelvin` (cal/(g · K)), as a `SpecificHeatCapacity`.
    pub const CALORIE_PER_GRAM_KELVIN: SpecificHeatCapacity =
        4184.0 * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN;

    /// One `JoulePerSquareMetreKelvin` (J/(m² · K)), as an `ArealHeatCapacity`.
    pub const JOULE_PER_SQUARE_METRE_KELVIN: ArealHeatCapacity =
        KILOGRAM / (SECOND * SECOND * KELVIN);

    /// One `JoulePerCubicMetreKelvin` (J/(m³ · K)), as a `VolumetricHeatCapacity`.
    pub const JOULE_PER_CUBIC_METRE_KELVIN: VolumetricHeatCapacity =
        KILOGRAM / (METRE * SECOND * SECOND * KELVIN);

    /// One `CaloriePerCubicMetreKelvin` (cal/(m³ · K)), as a `VolumetricHeatCapacity`.
    pub const CALORIE_PER_CUBIC_METRE_KELVIN: VolumetricHeatCapacity =
        4.1840000000000002 * JOULE_PER_CUBIC_METRE_KELVIN;

    /// One `WattPerSquareMetreQuarticKelvin` (W/(m² · K⁴)), as a `StefanBoltzmannCoefficient`.
    pub const WATT_PER_SQUARE_METRE_QUARTIC_KELVIN: StefanBoltzmannCoefficient =
        KILOGRAM / (SECOND * SECOND * SECOND * KELVIN * KELVIN * KELVIN * KELVIN);

    /// One `StefanBoltzmannConstant` (σ), as a `StefanBoltzmannCoefficient`.
    pub const STEFAN_BOLTZMANN_CONSTANT: StefanBoltzmannCoefficient =
        5.670374419e-8 * WATT_PER_SQUARE_METRE_QUARTIC_KELVIN;
}

/// The units of `ThermodynamicTemperature`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ThermodynamicTemperature)]
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
#[unit(of = TemperatureInterval)]
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
#[unit(of = TemperatureCoefficient)]
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
#[unit(of = TemperatureGradient)]
pub enum TemperatureGradientUnit {
    #[unit("K/km", "kelvins per kilometre", by = KELVIN_PER_KILOMETRE)]
    KelvinPerKilometre,
    #[unit("K/hm", "kelvins per hectometre", by = KELVIN_PER_HECTOMETRE)]
    KelvinPerHectometre,
    #[default]
    #[unit("K/m", "kelvins per metre", by = KELVIN_PER_METRE)]
    KelvinPerMetre,
    #[unit("K/cm", "kelvins per centimetre", by = KELVIN_PER_CENTIMETRE)]
    KelvinPerCentimetre,
    #[unit("K/mm", "kelvins per millimetre", by = KELVIN_PER_MILLIMETRE)]
    KelvinPerMillimetre,
    #[unit("K/µm", "kelvins per micrometre", by = KELVIN_PER_MICROMETRE)]
    KelvinPerMicrometre,
}

/// The units of `HeatCapacity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = HeatCapacity)]
pub enum HeatCapacityUnit {
    #[unit("Yg · m²/(s² · K)", "yottagram square metres per second squared kelvin", by = YOTTAGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    YottagramSquareMetrePerSecondSquaredKelvin,
    #[unit("Zg · m²/(s² · K)", "zettagram square metres per second squared kelvin", by = ZETTAGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    ZettagramSquareMetrePerSecondSquaredKelvin,
    #[unit("Eg · m²/(s² · K)", "exagram square metres per second squared kelvin", by = EXAGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    ExagramSquareMetrePerSecondSquaredKelvin,
    #[unit("Pg · m²/(s² · K)", "petagram square metres per second squared kelvin", by = PETAGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    PetagramSquareMetrePerSecondSquaredKelvin,
    #[unit("Tg · m²/(s² · K)", "teragram square metres per second squared kelvin", by = TERAGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    TeragramSquareMetrePerSecondSquaredKelvin,
    #[unit("Gg · m²/(s² · K)", "gigagram square metres per second squared kelvin", by = GIGAGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    GigagramSquareMetrePerSecondSquaredKelvin,
    #[unit("Mg · m²/(s² · K)", "megagram square metres per second squared kelvin", by = MEGAGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    MegagramSquareMetrePerSecondSquaredKelvin,
    #[default]
    #[unit("kg · m²/(s² · K)", "kilogram square metres per second squared kelvin", by = KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    KilogramSquareMetrePerSecondSquaredKelvin,
    #[unit("hg · m²/(s² · K)", "hectogram square metres per second squared kelvin", by = HECTOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    HectogramSquareMetrePerSecondSquaredKelvin,
    #[unit("dag · m²/(s² · K)", "decagram square metres per second squared kelvin", by = DECAGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    DecagramSquareMetrePerSecondSquaredKelvin,
    #[unit("g · m²/(s² · K)", "gram square metres per second squared kelvin", by = GRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    GramSquareMetrePerSecondSquaredKelvin,
    #[unit("dg · m²/(s² · K)", "decigram square metres per second squared kelvin", by = DECIGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    DecigramSquareMetrePerSecondSquaredKelvin,
    #[unit("cg · m²/(s² · K)", "centigram square metres per second squared kelvin", by = CENTIGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    CentigramSquareMetrePerSecondSquaredKelvin,
    #[unit("mg · m²/(s² · K)", "milligram square metres per second squared kelvin", by = MILLIGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    MilligramSquareMetrePerSecondSquaredKelvin,
    #[unit("µg · m²/(s² · K)", "microgram square metres per second squared kelvin", by = MICROGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    MicrogramSquareMetrePerSecondSquaredKelvin,
    #[unit("ng · m²/(s² · K)", "nanogram square metres per second squared kelvin", by = NANOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    NanogramSquareMetrePerSecondSquaredKelvin,
    #[unit("pg · m²/(s² · K)", "picogram square metres per second squared kelvin", by = PICOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    PicogramSquareMetrePerSecondSquaredKelvin,
    #[unit("fg · m²/(s² · K)", "femtogram square metres per second squared kelvin", by = FEMTOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    FemtogramSquareMetrePerSecondSquaredKelvin,
    #[unit("ag · m²/(s² · K)", "attogram square metres per second squared kelvin", by = ATTOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    AttogramSquareMetrePerSecondSquaredKelvin,
    #[unit("zg · m²/(s² · K)", "zeptogram square metres per second squared kelvin", by = ZEPTOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    ZeptogramSquareMetrePerSecondSquaredKelvin,
    #[unit("yg · m²/(s² · K)", "yoctogram square metres per second squared kelvin", by = YOCTOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    YoctogramSquareMetrePerSecondSquaredKelvin,
    #[unit("YJ/K", "yottajoules per kelvin", by = YOTTAJOULE_PER_KELVIN)]
    YottajoulePerKelvin,
    #[unit("ZJ/K", "zettajoules per kelvin", by = ZETTAJOULE_PER_KELVIN)]
    ZettajoulePerKelvin,
    #[unit("EJ/K", "exajoules per kelvin", by = EXAJOULE_PER_KELVIN)]
    ExajoulePerKelvin,
    #[unit("PJ/K", "petajoules per kelvin", by = PETAJOULE_PER_KELVIN)]
    PetajoulePerKelvin,
    #[unit("TJ/K", "terajoules per kelvin", by = TERAJOULE_PER_KELVIN)]
    TerajoulePerKelvin,
    #[unit("GJ/K", "gigajoules per kelvin", by = GIGAJOULE_PER_KELVIN)]
    GigajoulePerKelvin,
    #[unit("MJ/K", "megajoules per kelvin", by = MEGAJOULE_PER_KELVIN)]
    MegajoulePerKelvin,
    #[unit("kJ/K", "kilojoules per kelvin", by = KILOJOULE_PER_KELVIN)]
    KilojoulePerKelvin,
    #[unit("hJ/K", "hectojoules per kelvin", by = HECTOJOULE_PER_KELVIN)]
    HectojoulePerKelvin,
    #[unit("daJ/K", "decajoules per kelvin", by = DECAJOULE_PER_KELVIN)]
    DecajoulePerKelvin,
    #[unit("J/K", "joules per kelvin", by = JOULE_PER_KELVIN)]
    JoulePerKelvin,
    #[unit("dJ/K", "decijoules per kelvin", by = DECIJOULE_PER_KELVIN)]
    DecijoulePerKelvin,
    #[unit("cJ/K", "centijoules per kelvin", by = CENTIJOULE_PER_KELVIN)]
    CentijoulePerKelvin,
    #[unit("mJ/K", "millijoules per kelvin", by = MILLIJOULE_PER_KELVIN)]
    MillijoulePerKelvin,
    #[unit("µJ/K", "microjoules per kelvin", by = MICROJOULE_PER_KELVIN)]
    MicrojoulePerKelvin,
    #[unit("nJ/K", "nanojoules per kelvin", by = NANOJOULE_PER_KELVIN)]
    NanojoulePerKelvin,
    #[unit("pJ/K", "picojoules per kelvin", by = PICOJOULE_PER_KELVIN)]
    PicojoulePerKelvin,
    #[unit("fJ/K", "femtojoules per kelvin", by = FEMTOJOULE_PER_KELVIN)]
    FemtojoulePerKelvin,
    #[unit("aJ/K", "attojoules per kelvin", by = ATTOJOULE_PER_KELVIN)]
    AttojoulePerKelvin,
    #[unit("zJ/K", "zeptojoules per kelvin", by = ZEPTOJOULE_PER_KELVIN)]
    ZeptojoulePerKelvin,
    #[unit("yJ/K", "yoctojoules per kelvin", by = YOCTOJOULE_PER_KELVIN)]
    YoctojoulePerKelvin,
    #[unit("kJ/°C", "kilojoules per degree celsius", by = KILOJOULE_PER_DEGREE_CELSIUS)]
    KilojoulePerDegreeCelsius,
    #[unit("J/°C", "joules per degree celsius", by = JOULE_PER_DEGREE_CELSIUS)]
    JoulePerDegreeCelsius,
    #[unit("mJ/°C", "millijoules per degree celsius", by = MILLIJOULE_PER_DEGREE_CELSIUS)]
    MillijoulePerDegreeCelsius,
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
#[unit(of = HeatFluxDensity)]
pub enum HeatFluxDensityUnit {
    #[unit("YW/m²", "yottawatts per square metre", by = YOTTAWATT_PER_SQUARE_METRE)]
    YottawattPerSquareMetre,
    #[unit("ZW/m²", "zettawatts per square metre", by = ZETTAWATT_PER_SQUARE_METRE)]
    ZettawattPerSquareMetre,
    #[unit("EW/m²", "exawatts per square metre", by = EXAWATT_PER_SQUARE_METRE)]
    ExawattPerSquareMetre,
    #[unit("PW/m²", "petawatts per square metre", by = PETAWATT_PER_SQUARE_METRE)]
    PetawattPerSquareMetre,
    #[unit("TW/m²", "terawatts per square metre", by = TERAWATT_PER_SQUARE_METRE)]
    TerawattPerSquareMetre,
    #[unit("GW/m²", "gigawatts per square metre", by = GIGAWATT_PER_SQUARE_METRE)]
    GigawattPerSquareMetre,
    #[unit("MW/m²", "megawatts per square metre", by = MEGAWATT_PER_SQUARE_METRE)]
    MegawattPerSquareMetre,
    #[unit("kW/m²", "kilowatts per square metre", by = KILOWATT_PER_SQUARE_METRE)]
    KilowattPerSquareMetre,
    #[unit("hW/m²", "hectowatts per square metre", by = HECTOWATT_PER_SQUARE_METRE)]
    HectowattPerSquareMetre,
    #[unit("daW/m²", "decawatts per square metre", by = DECAWATT_PER_SQUARE_METRE)]
    DecawattPerSquareMetre,
    #[default]
    #[unit("W/m²", "watts per square metre", by = WATT_PER_SQUARE_METRE)]
    WattPerSquareMetre,
    #[unit("dW/m²", "deciwatts per square metre", by = DECIWATT_PER_SQUARE_METRE)]
    DeciwattPerSquareMetre,
    #[unit("cW/m²", "centiwatts per square metre", by = CENTIWATT_PER_SQUARE_METRE)]
    CentiwattPerSquareMetre,
    #[unit("mW/m²", "milliwatts per square metre", by = MILLIWATT_PER_SQUARE_METRE)]
    MilliwattPerSquareMetre,
    #[unit("µW/m²", "microwatts per square metre", by = MICROWATT_PER_SQUARE_METRE)]
    MicrowattPerSquareMetre,
    #[unit("nW/m²", "nanowatts per square metre", by = NANOWATT_PER_SQUARE_METRE)]
    NanowattPerSquareMetre,
    #[unit("pW/m²", "picowatts per square metre", by = PICOWATT_PER_SQUARE_METRE)]
    PicowattPerSquareMetre,
    #[unit("fW/m²", "femtowatts per square metre", by = FEMTOWATT_PER_SQUARE_METRE)]
    FemtowattPerSquareMetre,
    #[unit("aW/m²", "attowatts per square metre", by = ATTOWATT_PER_SQUARE_METRE)]
    AttowattPerSquareMetre,
    #[unit("zW/m²", "zeptowatts per square metre", by = ZEPTOWATT_PER_SQUARE_METRE)]
    ZeptowattPerSquareMetre,
    #[unit("yW/m²", "yoctowatts per square metre", by = YOCTOWATT_PER_SQUARE_METRE)]
    YoctowattPerSquareMetre,
}

/// The units of `HeatTransfer`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = HeatTransfer)]
pub enum HeatTransferUnit {
    #[unit("Yg/(s³ · K)", "yottagrams per second cubed kelvin", by = YOTTAGRAM_PER_SECOND_CUBED_KELVIN)]
    YottagramPerSecondCubedKelvin,
    #[unit("Zg/(s³ · K)", "zettagrams per second cubed kelvin", by = ZETTAGRAM_PER_SECOND_CUBED_KELVIN)]
    ZettagramPerSecondCubedKelvin,
    #[unit("Eg/(s³ · K)", "exagrams per second cubed kelvin", by = EXAGRAM_PER_SECOND_CUBED_KELVIN)]
    ExagramPerSecondCubedKelvin,
    #[unit("Pg/(s³ · K)", "petagrams per second cubed kelvin", by = PETAGRAM_PER_SECOND_CUBED_KELVIN)]
    PetagramPerSecondCubedKelvin,
    #[unit("Tg/(s³ · K)", "teragrams per second cubed kelvin", by = TERAGRAM_PER_SECOND_CUBED_KELVIN)]
    TeragramPerSecondCubedKelvin,
    #[unit("Gg/(s³ · K)", "gigagrams per second cubed kelvin", by = GIGAGRAM_PER_SECOND_CUBED_KELVIN)]
    GigagramPerSecondCubedKelvin,
    #[unit("Mg/(s³ · K)", "megagrams per second cubed kelvin", by = MEGAGRAM_PER_SECOND_CUBED_KELVIN)]
    MegagramPerSecondCubedKelvin,
    #[default]
    #[unit("kg/(s³ · K)", "kilograms per second cubed kelvin", by = KILOGRAM_PER_SECOND_CUBED_KELVIN)]
    KilogramPerSecondCubedKelvin,
    #[unit("hg/(s³ · K)", "hectograms per second cubed kelvin", by = HECTOGRAM_PER_SECOND_CUBED_KELVIN)]
    HectogramPerSecondCubedKelvin,
    #[unit("dag/(s³ · K)", "decagrams per second cubed kelvin", by = DECAGRAM_PER_SECOND_CUBED_KELVIN)]
    DecagramPerSecondCubedKelvin,
    #[unit("g/(s³ · K)", "grams per second cubed kelvin", by = GRAM_PER_SECOND_CUBED_KELVIN)]
    GramPerSecondCubedKelvin,
    #[unit("dg/(s³ · K)", "decigrams per second cubed kelvin", by = DECIGRAM_PER_SECOND_CUBED_KELVIN)]
    DecigramPerSecondCubedKelvin,
    #[unit("cg/(s³ · K)", "centigrams per second cubed kelvin", by = CENTIGRAM_PER_SECOND_CUBED_KELVIN)]
    CentigramPerSecondCubedKelvin,
    #[unit("mg/(s³ · K)", "milligrams per second cubed kelvin", by = MILLIGRAM_PER_SECOND_CUBED_KELVIN)]
    MilligramPerSecondCubedKelvin,
    #[unit("µg/(s³ · K)", "micrograms per second cubed kelvin", by = MICROGRAM_PER_SECOND_CUBED_KELVIN)]
    MicrogramPerSecondCubedKelvin,
    #[unit("ng/(s³ · K)", "nanograms per second cubed kelvin", by = NANOGRAM_PER_SECOND_CUBED_KELVIN)]
    NanogramPerSecondCubedKelvin,
    #[unit("pg/(s³ · K)", "picograms per second cubed kelvin", by = PICOGRAM_PER_SECOND_CUBED_KELVIN)]
    PicogramPerSecondCubedKelvin,
    #[unit("fg/(s³ · K)", "femtograms per second cubed kelvin", by = FEMTOGRAM_PER_SECOND_CUBED_KELVIN)]
    FemtogramPerSecondCubedKelvin,
    #[unit("ag/(s³ · K)", "attograms per second cubed kelvin", by = ATTOGRAM_PER_SECOND_CUBED_KELVIN)]
    AttogramPerSecondCubedKelvin,
    #[unit("zg/(s³ · K)", "zeptograms per second cubed kelvin", by = ZEPTOGRAM_PER_SECOND_CUBED_KELVIN)]
    ZeptogramPerSecondCubedKelvin,
    #[unit("yg/(s³ · K)", "yoctograms per second cubed kelvin", by = YOCTOGRAM_PER_SECOND_CUBED_KELVIN)]
    YoctogramPerSecondCubedKelvin,
    #[unit("kg/(s³ · °C)", "kilograms per second cubed degree celsius", by = KILOGRAM_PER_SECOND_CUBED_DEGREE_CELSIUS)]
    KilogramPerSecondCubedDegreeCelsius,
    #[unit("YW/(m² · K)", "yottawatts per square metre kelvin", by = YOTTAWATT_PER_SQUARE_METRE_KELVIN)]
    YottawattPerSquareMetreKelvin,
    #[unit("ZW/(m² · K)", "zettawatts per square metre kelvin", by = ZETTAWATT_PER_SQUARE_METRE_KELVIN)]
    ZettawattPerSquareMetreKelvin,
    #[unit("EW/(m² · K)", "exawatts per square metre kelvin", by = EXAWATT_PER_SQUARE_METRE_KELVIN)]
    ExawattPerSquareMetreKelvin,
    #[unit("PW/(m² · K)", "petawatts per square metre kelvin", by = PETAWATT_PER_SQUARE_METRE_KELVIN)]
    PetawattPerSquareMetreKelvin,
    #[unit("TW/(m² · K)", "terawatts per square metre kelvin", by = TERAWATT_PER_SQUARE_METRE_KELVIN)]
    TerawattPerSquareMetreKelvin,
    #[unit("GW/(m² · K)", "gigawatts per square metre kelvin", by = GIGAWATT_PER_SQUARE_METRE_KELVIN)]
    GigawattPerSquareMetreKelvin,
    #[unit("MW/(m² · K)", "megawatts per square metre kelvin", by = MEGAWATT_PER_SQUARE_METRE_KELVIN)]
    MegawattPerSquareMetreKelvin,
    #[unit("kW/(m² · K)", "kilowatts per square metre kelvin", by = KILOWATT_PER_SQUARE_METRE_KELVIN)]
    KilowattPerSquareMetreKelvin,
    #[unit("hW/(m² · K)", "hectowatts per square metre kelvin", by = HECTOWATT_PER_SQUARE_METRE_KELVIN)]
    HectowattPerSquareMetreKelvin,
    #[unit("daW/(m² · K)", "decawatts per square metre kelvin", by = DECAWATT_PER_SQUARE_METRE_KELVIN)]
    DecawattPerSquareMetreKelvin,
    #[unit("W/(m² · K)", "watts per square metre kelvin", by = WATT_PER_SQUARE_METRE_KELVIN)]
    WattPerSquareMetreKelvin,
    #[unit("dW/(m² · K)", "deciwatts per square metre kelvin", by = DECIWATT_PER_SQUARE_METRE_KELVIN)]
    DeciwattPerSquareMetreKelvin,
    #[unit("cW/(m² · K)", "centiwatts per square metre kelvin", by = CENTIWATT_PER_SQUARE_METRE_KELVIN)]
    CentiwattPerSquareMetreKelvin,
    #[unit("mW/(m² · K)", "milliwatts per square metre kelvin", by = MILLIWATT_PER_SQUARE_METRE_KELVIN)]
    MilliwattPerSquareMetreKelvin,
    #[unit("µW/(m² · K)", "microwatts per square metre kelvin", by = MICROWATT_PER_SQUARE_METRE_KELVIN)]
    MicrowattPerSquareMetreKelvin,
    #[unit("nW/(m² · K)", "nanowatts per square metre kelvin", by = NANOWATT_PER_SQUARE_METRE_KELVIN)]
    NanowattPerSquareMetreKelvin,
    #[unit("pW/(m² · K)", "picowatts per square metre kelvin", by = PICOWATT_PER_SQUARE_METRE_KELVIN)]
    PicowattPerSquareMetreKelvin,
    #[unit("fW/(m² · K)", "femtowatts per square metre kelvin", by = FEMTOWATT_PER_SQUARE_METRE_KELVIN)]
    FemtowattPerSquareMetreKelvin,
    #[unit("aW/(m² · K)", "attowatts per square metre kelvin", by = ATTOWATT_PER_SQUARE_METRE_KELVIN)]
    AttowattPerSquareMetreKelvin,
    #[unit("zW/(m² · K)", "zeptowatts per square metre kelvin", by = ZEPTOWATT_PER_SQUARE_METRE_KELVIN)]
    ZeptowattPerSquareMetreKelvin,
    #[unit("yW/(m² · K)", "yoctowatts per square metre kelvin", by = YOCTOWATT_PER_SQUARE_METRE_KELVIN)]
    YoctowattPerSquareMetreKelvin,
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
#[unit(of = ThermalConductance)]
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
    #[unit("µg · m/(s³ · K)", "micrograms metre squared per second cubed kelvin", by = MICROGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN)]
    MicrogramMetreSquaredPerSecondCubedKelvin,
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
    #[unit("YW/K", "yottawatts per kelvin", by = YOTTAWATT_PER_KELVIN)]
    YottawattPerKelvin,
    #[unit("ZW/K", "zettawatts per kelvin", by = ZETTAWATT_PER_KELVIN)]
    ZettawattPerKelvin,
    #[unit("EW/K", "exawatts per kelvin", by = EXAWATT_PER_KELVIN)]
    ExawattPerKelvin,
    #[unit("PW/K", "petawatts per kelvin", by = PETAWATT_PER_KELVIN)]
    PetawattPerKelvin,
    #[unit("TW/K", "terawatts per kelvin", by = TERAWATT_PER_KELVIN)]
    TerawattPerKelvin,
    #[unit("GW/K", "gigawatts per kelvin", by = GIGAWATT_PER_KELVIN)]
    GigawattPerKelvin,
    #[unit("MW/K", "megawatts per kelvin", by = MEGAWATT_PER_KELVIN)]
    MegawattPerKelvin,
    #[unit("kW/K", "kilowatts per kelvin", by = KILOWATT_PER_KELVIN)]
    KilowattPerKelvin,
    #[unit("hW/K", "hectowatts per kelvin", by = HECTOWATT_PER_KELVIN)]
    HectowattPerKelvin,
    #[unit("daW/K", "decawatts per kelvin", by = DECAWATT_PER_KELVIN)]
    DecawattPerKelvin,
    #[unit("W/K", "watts per kelvin", by = WATT_PER_KELVIN)]
    WattPerKelvin,
    #[unit("dW/K", "deciwatts per kelvin", by = DECIWATT_PER_KELVIN)]
    DeciwattPerKelvin,
    #[unit("cW/K", "centiwatts per kelvin", by = CENTIWATT_PER_KELVIN)]
    CentiwattPerKelvin,
    #[unit("mW/K", "milliwatts per kelvin", by = MILLIWATT_PER_KELVIN)]
    MilliwattPerKelvin,
    #[unit("µW/K", "microwatts per kelvin", by = MICROWATT_PER_KELVIN)]
    MicrowattPerKelvin,
    #[unit("nW/K", "nanowatts per kelvin", by = NANOWATT_PER_KELVIN)]
    NanowattPerKelvin,
    #[unit("pW/K", "picowatts per kelvin", by = PICOWATT_PER_KELVIN)]
    PicowattPerKelvin,
    #[unit("fW/K", "femtowatts per kelvin", by = FEMTOWATT_PER_KELVIN)]
    FemtowattPerKelvin,
    #[unit("aW/K", "attowatts per kelvin", by = ATTOWATT_PER_KELVIN)]
    AttowattPerKelvin,
    #[unit("zW/K", "zeptowatts per kelvin", by = ZEPTOWATT_PER_KELVIN)]
    ZeptowattPerKelvin,
    #[unit("yW/K", "yoctowatts per kelvin", by = YOCTOWATT_PER_KELVIN)]
    YoctowattPerKelvin,
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
#[unit(of = ThermalConductivity)]
pub enum ThermalConductivityUnit {
    #[unit("Yg · m/(s³ · K)", "yottagrams metre per second cubed kelvin", by = YOTTAGRAM_METRE_PER_SECOND_CUBED_KELVIN)]
    YottagramMetrePerSecondCubedKelvin,
    #[unit("Zg · m/(s³ · K)", "zettagrams metre per second cubed kelvin", by = ZETTAGRAM_METRE_PER_SECOND_CUBED_KELVIN)]
    ZettagramMetrePerSecondCubedKelvin,
    #[unit("Eg · m/(s³ · K)", "exagrams metre per second cubed kelvin", by = EXAGRAM_METRE_PER_SECOND_CUBED_KELVIN)]
    ExagramMetrePerSecondCubedKelvin,
    #[unit("Pg · m/(s³ · K)", "petagrams metre per second cubed kelvin", by = PETAGRAM_METRE_PER_SECOND_CUBED_KELVIN)]
    PetagramMetrePerSecondCubedKelvin,
    #[unit("Tg · m/(s³ · K)", "teragrams metre per second cubed kelvin", by = TERAGRAM_METRE_PER_SECOND_CUBED_KELVIN)]
    TeragramMetrePerSecondCubedKelvin,
    #[unit("Gg · m/(s³ · K)", "gigagrams metre per second cubed kelvin", by = GIGAGRAM_METRE_PER_SECOND_CUBED_KELVIN)]
    GigagramMetrePerSecondCubedKelvin,
    #[unit("Mg · m/(s³ · K)", "megagrams metre per second cubed kelvin", by = MEGAGRAM_METRE_PER_SECOND_CUBED_KELVIN)]
    MegagramMetrePerSecondCubedKelvin,
    #[default]
    #[unit("kg · m/(s³ · K)", "kilograms metre per second cubed kelvin", by = KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN)]
    KilogramMetrePerSecondCubedKelvin,
    #[unit("hg · m/(s³ · K)", "hectograms metre per second cubed kelvin", by = HECTOGRAM_METRE_PER_SECOND_CUBED_KELVIN)]
    HectogramMetrePerSecondCubedKelvin,
    #[unit("dag · m/(s³ · K)", "decagrams metre per second cubed kelvin", by = DECAGRAM_METRE_PER_SECOND_CUBED_KELVIN)]
    DecagramMetrePerSecondCubedKelvin,
    #[unit("g · m/(s³ · K)", "grams metre per second cubed kelvin", by = GRAM_METRE_PER_SECOND_CUBED_KELVIN)]
    GramMetrePerSecondCubedKelvin,
    #[unit("dg · m/(s³ · K)", "decigrams metre per second cubed kelvin", by = DECIGRAM_METRE_PER_SECOND_CUBED_KELVIN)]
    DecigramMetrePerSecondCubedKelvin,
    #[unit("cg · m/(s³ · K)", "centigrams metre per second cubed kelvin", by = CENTIGRAM_METRE_PER_SECOND_CUBED_KELVIN)]
    CentigramMetrePerSecondCubedKelvin,
    #[unit("mg · m/(s³ · K)", "milligrams metre per second cubed kelvin", by = MILLIGRAM_METRE_PER_SECOND_CUBED_KELVIN)]
    MilligramMetrePerSecondCubedKelvin,
    #[unit("µg · m/(s³ · K)", "micrograms metre per second cubed kelvin", by = MICROGRAM_METRE_PER_SECOND_CUBED_KELVIN)]
    MicrogramMetrePerSecondCubedKelvin,
    #[unit("ng · m/(s³ · K)", "nanograms metre per second cubed kelvin", by = NANOGRAM_METRE_PER_SECOND_CUBED_KELVIN)]
    NanogramMetrePerSecondCubedKelvin,
    #[unit("pg · m/(s³ · K)", "picograms metre per second cubed kelvin", by = PICOGRAM_METRE_PER_SECOND_CUBED_KELVIN)]
    PicogramMetrePerSecondCubedKelvin,
    #[unit("fg · m/(s³ · K)", "femtograms metre per second cubed kelvin", by = FEMTOGRAM_METRE_PER_SECOND_CUBED_KELVIN)]
    FemtogramMetrePerSecondCubedKelvin,
    #[unit("ag · m/(s³ · K)", "attograms metre per second cubed kelvin", by = ATTOGRAM_METRE_PER_SECOND_CUBED_KELVIN)]
    AttogramMetrePerSecondCubedKelvin,
    #[unit("zg · m/(s³ · K)", "zeptograms metre per second cubed kelvin", by = ZEPTOGRAM_METRE_PER_SECOND_CUBED_KELVIN)]
    ZeptogramMetrePerSecondCubedKelvin,
    #[unit("yg · m/(s³ · K)", "yoctograms metre per second cubed kelvin", by = YOCTOGRAM_METRE_PER_SECOND_CUBED_KELVIN)]
    YoctogramMetrePerSecondCubedKelvin,
    #[unit("YW/(m · K)", "yottawatts per metre kelvin", by = YOTTAWATT_PER_METRE_KELVIN)]
    YottawattPerMetreKelvin,
    #[unit("ZW/(m · K)", "zettawatts per metre kelvin", by = ZETTAWATT_PER_METRE_KELVIN)]
    ZettawattPerMetreKelvin,
    #[unit("EW/(m · K)", "exawatts per metre kelvin", by = EXAWATT_PER_METRE_KELVIN)]
    ExawattPerMetreKelvin,
    #[unit("PW/(m · K)", "petawatts per metre kelvin", by = PETAWATT_PER_METRE_KELVIN)]
    PetawattPerMetreKelvin,
    #[unit("TW/(m · K)", "terawatts per metre kelvin", by = TERAWATT_PER_METRE_KELVIN)]
    TerawattPerMetreKelvin,
    #[unit("GW/(m · K)", "gigawatts per metre kelvin", by = GIGAWATT_PER_METRE_KELVIN)]
    GigawattPerMetreKelvin,
    #[unit("MW/(m · K)", "megawatts per metre kelvin", by = MEGAWATT_PER_METRE_KELVIN)]
    MegawattPerMetreKelvin,
    #[unit("kW/(m · K)", "kilowatts per metre kelvin", by = KILOWATT_PER_METRE_KELVIN)]
    KilowattPerMetreKelvin,
    #[unit("hW/(m · K)", "hectowatts per metre kelvin", by = HECTOWATT_PER_METRE_KELVIN)]
    HectowattPerMetreKelvin,
    #[unit("daW/(m · K)", "decawatts per metre kelvin", by = DECAWATT_PER_METRE_KELVIN)]
    DecawattPerMetreKelvin,
    #[unit("W/(m · K)", "watts per metre kelvin", by = WATT_PER_METRE_KELVIN)]
    WattPerMetreKelvin,
    #[unit("dW/(m · K)", "deciwatts per metre kelvin", by = DECIWATT_PER_METRE_KELVIN)]
    DeciwattPerMetreKelvin,
    #[unit("cW/(m · K)", "centiwatts per metre kelvin", by = CENTIWATT_PER_METRE_KELVIN)]
    CentiwattPerMetreKelvin,
    #[unit("mW/(m · K)", "milliwatts per metre kelvin", by = MILLIWATT_PER_METRE_KELVIN)]
    MilliwattPerMetreKelvin,
    #[unit("µW/(m · K)", "microwatts per metre kelvin", by = MICROWATT_PER_METRE_KELVIN)]
    MicrowattPerMetreKelvin,
    #[unit("nW/(m · K)", "nanowatts per metre kelvin", by = NANOWATT_PER_METRE_KELVIN)]
    NanowattPerMetreKelvin,
    #[unit("pW/(m · K)", "picowatts per metre kelvin", by = PICOWATT_PER_METRE_KELVIN)]
    PicowattPerMetreKelvin,
    #[unit("fW/(m · K)", "femtowatts per metre kelvin", by = FEMTOWATT_PER_METRE_KELVIN)]
    FemtowattPerMetreKelvin,
    #[unit("aW/(m · K)", "attowatts per metre kelvin", by = ATTOWATT_PER_METRE_KELVIN)]
    AttowattPerMetreKelvin,
    #[unit("zW/(m · K)", "zeptowatts per metre kelvin", by = ZEPTOWATT_PER_METRE_KELVIN)]
    ZeptowattPerMetreKelvin,
    #[unit("yW/(m · K)", "yoctowatts per metre kelvin", by = YOCTOWATT_PER_METRE_KELVIN)]
    YoctowattPerMetreKelvin,
    #[unit("kg · m/(s³ · °C)", "kilograms metre per second cubed degree celsius", by = KILOGRAM_METRE_PER_SECOND_CUBED_DEGREE_CELSIUS)]
    KilogramMetrePerSecondCubedDegreeCelsius,
    #[unit("kW/(m · °C)", "kilowatts per metre degree celsius", by = KILOWATT_PER_METRE_DEGREE_CELSIUS)]
    KilowattPerMetreDegreeCelsius,
    #[unit("W/(m · °C)", "watts per metre degree celsius", by = THERMAL_CONDUCTIVITY_WATT_PER_METRE_DEGREE_CELSIUS)]
    WattPerMetreDegreeCelsius,
    #[unit("mW/(m · °C)", "milliwatts per metre degree celsius", by = MILLIWATT_PER_METRE_DEGREE_CELSIUS)]
    MilliwattPerMetreDegreeCelsius,
}

/// The units of `ThermalResistance`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ThermalResistance)]
pub enum ThermalResistanceUnit {
    #[unit("K/YW", "kelvins per yottawatt", by = KELVIN_PER_YOTTAWATT)]
    KelvinPerYottawatt,
    #[unit("K/ZW", "kelvins per zettawatt", by = KELVIN_PER_ZETTAWATT)]
    KelvinPerZettawatt,
    #[unit("K/EW", "kelvins per exawatt", by = KELVIN_PER_EXAWATT)]
    KelvinPerExawatt,
    #[unit("K/PW", "kelvins per petawatt", by = KELVIN_PER_PETAWATT)]
    KelvinPerPetawatt,
    #[unit("K/TW", "kelvins per terawatt", by = KELVIN_PER_TERAWATT)]
    KelvinPerTerawatt,
    #[unit("K/GW", "kelvins per gigawatt", by = KELVIN_PER_GIGAWATT)]
    KelvinPerGigawatt,
    #[unit("K/MW", "kelvins per megawatt", by = KELVIN_PER_MEGAWATT)]
    KelvinPerMegawatt,
    #[unit("K/kw", "kelvins per kilowatt", by = KELVIN_PER_KILOWATT)]
    KelvinPerKilowatt,
    #[unit("K/hW", "kelvins per hectowatt", by = KELVIN_PER_HECTOWATT)]
    KelvinPerHectowatt,
    #[unit("K/daW", "kelvins per decawatt", by = KELVIN_PER_DECAWATT)]
    KelvinPerDecawatt,
    #[default]
    #[unit("K/W", "kelvins per watt", by = KELVIN_PER_WATT)]
    KelvinPerWatt,
    #[unit("K/dW", "kelvins per deciwatt", by = KELVIN_PER_DECIWATT)]
    KelvinPerDeciwatt,
    #[unit("K/cW", "kelvins per centiwatt", by = KELVIN_PER_CENTIWATT)]
    KelvinPerCentiwatt,
    #[unit("K/mW", "kelvins per milliwatt", by = KELVIN_PER_MILLIWATT)]
    KelvinPerMilliwatt,
    #[unit("K/µW", "kelvins per microwatt", by = KELVIN_PER_MICROWATT)]
    KelvinPerMicrowatt,
    #[unit("K/nW", "kelvins per nanowatt", by = KELVIN_PER_NANOWATT)]
    KelvinPerNanowatt,
    #[unit("K/pW", "kelvins per picowatt", by = KELVIN_PER_PICOWATT)]
    KelvinPerPicowatt,
    #[unit("K/fW", "kelvins per femtowatt", by = KELVIN_PER_FEMTOWATT)]
    KelvinPerFemtowatt,
    #[unit("K/aW", "kelvins per attowatt", by = KELVIN_PER_ATTOWATT)]
    KelvinPerAttowatt,
    #[unit("K/zW", "kelvins per zeptowatt", by = KELVIN_PER_ZEPTOWATT)]
    KelvinPerZeptowatt,
    #[unit("K/yW", "kelvins per yoctowatt", by = KELVIN_PER_YOCTOWATT)]
    KelvinPerYoctowatt,
}

/// The units of `SpecificHeatCapacity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificHeatCapacity)]
pub enum SpecificHeatCapacityUnit {
    #[unit("km²/(s² · K)", "square kilometres per second squared kelvin", by = SQUARE_KILOMETRE_PER_SECOND_SQUARED_KELVIN)]
    SquareKilometrePerSecondSquaredKelvin,
    #[default]
    #[unit("m²/(s² · K)", "square metres per second squared kelvin", by = SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    SquareMetrePerSecondSquaredKelvin,
    #[unit("cm²/(s² · K)", "square centimetres per second squared kelvin", by = SQUARE_CENTIMETRE_PER_SECOND_SQUARED_KELVIN)]
    SquareCentimetrePerSecondSquaredKelvin,
    #[unit("mm²/(s² · K)", "square millimetres per second squared kelvin", by = SQUARE_MILLIMETRE_PER_SECOND_SQUARED_KELVIN)]
    SquareMillimetrePerSecondSquaredKelvin,
    #[unit("µm²/(s² · K)", "square micrometres per second squared kelvin", by = SQUARE_MICROMETRE_PER_SECOND_SQUARED_KELVIN)]
    SquareMicrometrePerSecondSquaredKelvin,
    #[unit("YJ/(kg · K)", "yottajoules per kilogram kelvin", by = YOTTAJOULE_PER_KILOGRAM_KELVIN)]
    YottajoulePerKilogramKelvin,
    #[unit("ZJ/(kg · K)", "zettajoules per kilogram kelvin", by = ZETTAJOULE_PER_KILOGRAM_KELVIN)]
    ZettajoulePerKilogramKelvin,
    #[unit("EJ/(kg · K)", "exajoules per kilogram kelvin", by = EXAJOULE_PER_KILOGRAM_KELVIN)]
    ExajoulePerKilogramKelvin,
    #[unit("PJ/(kg · K)", "petajoules per kilogram kelvin", by = PETAJOULE_PER_KILOGRAM_KELVIN)]
    PetajoulePerKilogramKelvin,
    #[unit("TJ/(kg · K)", "terajoules per kilogram kelvin", by = TERAJOULE_PER_KILOGRAM_KELVIN)]
    TerajoulePerKilogramKelvin,
    #[unit("GJ/(kg · K)", "gigajoules per kilogram kelvin", by = GIGAJOULE_PER_KILOGRAM_KELVIN)]
    GigajoulePerKilogramKelvin,
    #[unit("MJ/(kg · K)", "megajoules per kilogram kelvin", by = MEGAJOULE_PER_KILOGRAM_KELVIN)]
    MegajoulePerKilogramKelvin,
    #[unit("kJ/(kg · K)", "kilojoules per kilogram kelvin", by = KILOJOULE_PER_KILOGRAM_KELVIN)]
    KilojoulePerKilogramKelvin,
    #[unit("hJ/(kg · K)", "hectojoules per kilogram kelvin", by = HECTOJOULE_PER_KILOGRAM_KELVIN)]
    HectojoulePerKilogramKelvin,
    #[unit("daJ/(kg · K)", "decajoules per kilogram kelvin", by = DECAJOULE_PER_KILOGRAM_KELVIN)]
    DecajoulePerKilogramKelvin,
    #[unit("J/(kg · K)", "joules per kilogram kelvin", by = JOULE_PER_KILOGRAM_KELVIN)]
    JoulePerKilogramKelvin,
    #[unit("dJ/(kg · K)", "decijoules per kilogram kelvin", by = DECIJOULE_PER_KILOGRAM_KELVIN)]
    DecijoulePerKilogramKelvin,
    #[unit("cJ/(kg · K)", "centijoules per kilogram kelvin", by = CENTIJOULE_PER_KILOGRAM_KELVIN)]
    CentijoulePerKilogramKelvin,
    #[unit("mJ/(kg · K)", "millijoules per kilogram kelvin", by = MILLIJOULE_PER_KILOGRAM_KELVIN)]
    MillijoulePerKilogramKelvin,
    #[unit("µJ/(kg · K)", "microjoules per kilogram kelvin", by = MICROJOULE_PER_KILOGRAM_KELVIN)]
    MicrojoulePerKilogramKelvin,
    #[unit("nJ/(kg · K)", "nanojoules per kilogram kelvin", by = NANOJOULE_PER_KILOGRAM_KELVIN)]
    NanojoulePerKilogramKelvin,
    #[unit("pJ/(kg · K)", "picojoules per kilogram kelvin", by = PICOJOULE_PER_KILOGRAM_KELVIN)]
    PicojoulePerKilogramKelvin,
    #[unit("fJ/(kg · K)", "femtojoules per kilogram kelvin", by = FEMTOJOULE_PER_KILOGRAM_KELVIN)]
    FemtojoulePerKilogramKelvin,
    #[unit("aJ/(kg · K)", "attojoules per kilogram kelvin", by = ATTOJOULE_PER_KILOGRAM_KELVIN)]
    AttojoulePerKilogramKelvin,
    #[unit("zJ/(kg · K)", "zeptojoules per kilogram kelvin", by = ZEPTOJOULE_PER_KILOGRAM_KELVIN)]
    ZeptojoulePerKilogramKelvin,
    #[unit("yJ/(kg · K)", "yoctojoules per kilogram kelvin", by = YOCTOJOULE_PER_KILOGRAM_KELVIN)]
    YoctojoulePerKilogramKelvin,
    #[unit("kJ/(kg · °C)", "kilojoules per kilogram degree celsius", by = KILOJOULE_PER_KILOGRAM_DEGREE_CELSIUS)]
    KilojoulePerKilogramDegreeCelsius,
    #[unit("kJ/(g · °C)", "kilojoules per gram degree celsius", by = KILOJOULE_PER_GRAM_DEGREE_CELSIUS)]
    KilojoulePerGramDegreeCelsius,
    #[unit("J/(kg · °C)", "joules per kilogram degree celsius", by = JOULE_PER_KILOGRAM_DEGREE_CELSIUS)]
    JoulePerKilogramDegreeCelsius,
    #[unit("J/(g · °C)", "joules per gram degree celsius", by = JOULE_PER_GRAM_DEGREE_CELSIUS)]
    JoulePerGramDegreeCelsius,
    #[unit("mJ/(kg · °C)", "millijoules per kilogram degree celsius", by = MILLIJOULE_PER_KILOGRAM_DEGREE_CELSIUS)]
    MillijoulePerKilogramDegreeCelsius,
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
    #[unit("cal/(kg · K)", "calories per kilogram kelvin", by = CALORIE_PER_KILOGRAM_KELVIN)]
    CaloriePerKilogramKelvin,
    #[unit("cal/(g · K)", "calories per gram kelvin", by = CALORIE_PER_GRAM_KELVIN)]
    CaloriePerGramKelvin,
}

/// The units of `ArealHeatCapacity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ArealHeatCapacity)]
pub enum ArealHeatCapacityUnit {
    #[default]
    #[unit("J/(m² · K)", "joules per square metre kelvin", by = JOULE_PER_SQUARE_METRE_KELVIN)]
    JoulePerSquareMetreKelvin,
}

/// The units of `VolumetricHeatCapacity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = VolumetricHeatCapacity)]
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
#[unit(of = StefanBoltzmannCoefficient)]
pub enum StefanBoltzmannCoefficientUnit {
    #[default]
    #[unit("W/(m² · K⁴)", "watts per square metre quartic kelvin", by = WATT_PER_SQUARE_METRE_QUARTIC_KELVIN)]
    WattPerSquareMetreQuarticKelvin,
    #[unit("σ", "Stefan-Boltzmann constants", by = STEFAN_BOLTZMANN_CONSTANT)]
    StefanBoltzmannConstant,
}
