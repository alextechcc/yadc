use crate::{Exact, ISQ, Kind, SIQuantity, Scale, counted};
use consts::*;
use yadc::Dimension;

pub type ThermodynamicTemperature<N = f64> = SIQuantity<
    {
        ISQ {
            K: 1,
            kind: Kind::Absolute,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type TemperatureInterval<N = f64> = SIQuantity<
    {
        ISQ {
            K: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type TemperatureCoefficient<N = f64> = SIQuantity<
    {
        ISQ {
            K: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type TemperatureGradient<N = f64> = SIQuantity<
    {
        ISQ {
            m: -1,
            K: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type HeatCapacity<N = f64> = SIQuantity<
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

pub type HeatFluxDensity<N = f64> = SIQuantity<
    {
        ISQ {
            kg: 1,
            s: -3,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type HeatTransfer<N = f64> = SIQuantity<
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

pub type ThermalConductance<N = f64> = SIQuantity<
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

pub type ThermalConductivity<N = f64> = SIQuantity<
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

pub type ThermalResistance<N = f64> = SIQuantity<
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

pub type SpecificHeatCapacity<N = f64> = SIQuantity<
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

pub type ArealHeatCapacity<N = f64> = SIQuantity<
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

pub type VolumetricHeatCapacity<N = f64> = SIQuantity<
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

pub type StefanBoltzmannCoefficient<N = f64> = SIQuantity<
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
/// `KILOMETRE` is `1000 * METRE` - and counted in the number type its use
/// asks for, rounded once: `let d: Length = KILOMETRE;` is an `f64` length,
/// `Length<f32>` an `f32` one, `KILOMETRE::<Exact>` the exact value. Each is
/// a unit (`KILOMETRE(3.0)`, `KILOMETRE.value(d)`), and they compose in
/// `const` items: `const LIMIT: Velocity = 50.0 * KILOMETRE / HOUR;`.
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
    pub const KELVIN_SCALE<N: const From<Exact>>: Scale<ThermodynamicTemperature<N>, TemperatureInterval<N>> = Scale(counted(ABSOLUTE_ZERO::<Exact>), counted(KELVIN::<Exact>));

    /// The Celsius scale (°C): kelvins from the ice point.
    pub const CELSIUS<N: const From<Exact>>: Scale<ThermodynamicTemperature<N>, TemperatureInterval<N>> = Scale(counted(ICE_POINT::<Exact>), counted(KELVIN::<Exact>));

    /// The Fahrenheit scale (°F): degrees Fahrenheit from 32 °F below the ice point.
    pub const FAHRENHEIT<N: const From<Exact>>: Scale<ThermodynamicTemperature<N>, TemperatureInterval<N>> = Scale(counted(FAHRENHEIT_ZERO::<Exact>), counted(DEGREE_FAHRENHEIT::<Exact>));

    /// The Rankine scale (°R): degrees Fahrenheit from absolute zero.
    pub const RANKINE<N: const From<Exact>>: Scale<ThermodynamicTemperature<N>, TemperatureInterval<N>> = Scale(counted(ABSOLUTE_ZERO::<Exact>), counted(DEGREE_FAHRENHEIT::<Exact>));

    /// One `Fahrenheit` (°F), as a `TemperatureInterval`.
    pub const DEGREE_FAHRENHEIT<N: const From<Exact>>: TemperatureInterval<N> = counted((Exact::int(5) / Exact::int(9)) * KELVIN::<Exact>);

    /// One `PerKelvin` (K⁻¹), as a `TemperatureCoefficient`.
    pub const PER_KELVIN<N: const From<Exact>>: TemperatureCoefficient<N> = counted(ONE::<Exact> / KELVIN::<Exact>);

    /// One `PpmPerKelvin` (ppm/K), as a `TemperatureCoefficient`.
    pub const PPM_PER_KELVIN<N: const From<Exact>>: TemperatureCoefficient<N> = counted(Exact::dec(1, -6) * PER_KELVIN::<Exact>);

    /// One `PpmPerDegreeCelsius` (ppm/°C), as a `TemperatureCoefficient`.
    pub const PPM_PER_DEGREE_CELSIUS<N: const From<Exact>>: TemperatureCoefficient<N> = counted(Exact::dec(1, -6) * PER_KELVIN::<Exact>);

    /// One `KelvinPerKilometre` (K/km), as a `TemperatureGradient`.
    pub const KELVIN_PER_KILOMETRE<N: const From<Exact>>: TemperatureGradient<N> = counted(Exact::dec(1, -3) * KELVIN_PER_METRE::<Exact>);

    /// One `KelvinPerHectometre` (K/hm), as a `TemperatureGradient`.
    pub const KELVIN_PER_HECTOMETRE<N: const From<Exact>>: TemperatureGradient<N> = counted(Exact::dec(1, -2) * KELVIN_PER_METRE::<Exact>);

    /// One `KelvinPerMetre` (K/m), as a `TemperatureGradient`.
    pub const KELVIN_PER_METRE<N: const From<Exact>>: TemperatureGradient<N> = counted(KELVIN::<Exact> / METRE::<Exact>);

    /// One `KelvinPerCentimetre` (K/cm), as a `TemperatureGradient`.
    pub const KELVIN_PER_CENTIMETRE<N: const From<Exact>>: TemperatureGradient<N> = counted(Exact::int(100) * KELVIN_PER_METRE::<Exact>);

    /// One `KelvinPerMillimetre` (K/mm), as a `TemperatureGradient`.
    pub const KELVIN_PER_MILLIMETRE<N: const From<Exact>>: TemperatureGradient<N> = counted(Exact::int(1000) * KELVIN_PER_METRE::<Exact>);

    /// One `KelvinPerMicrometre` (K/µm), as a `TemperatureGradient`.
    pub const KELVIN_PER_MICROMETRE<N: const From<Exact>>: TemperatureGradient<N> = counted(Exact::int(1000000) * KELVIN_PER_METRE::<Exact>);

    /// One `YottagramSquareMetrePerSecondSquaredKelvin` (Yg · m²/(s² · K)), as a `HeatCapacity`.
    pub const YOTTAGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::int(1000000000000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `ZettagramSquareMetrePerSecondSquaredKelvin` (Zg · m²/(s² · K)), as a `HeatCapacity`.
    pub const ZETTAGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::int(1000000000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `ExagramSquareMetrePerSecondSquaredKelvin` (Eg · m²/(s² · K)), as a `HeatCapacity`.
    pub const EXAGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::int(1000000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `PetagramSquareMetrePerSecondSquaredKelvin` (Pg · m²/(s² · K)), as a `HeatCapacity`.
    pub const PETAGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::int(1000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `TeragramSquareMetrePerSecondSquaredKelvin` (Tg · m²/(s² · K)), as a `HeatCapacity`.
    pub const TERAGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::int(1000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `GigagramSquareMetrePerSecondSquaredKelvin` (Gg · m²/(s² · K)), as a `HeatCapacity`.
    pub const GIGAGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::int(1000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `MegagramSquareMetrePerSecondSquaredKelvin` (Mg · m²/(s² · K)), as a `HeatCapacity`.
    pub const MEGAGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::int(1000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `KilogramSquareMetrePerSecondSquaredKelvin` (kg · m²/(s² · K)), as a `HeatCapacity`.
    pub const KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * KELVIN::<Exact>));

    /// One `HectogramSquareMetrePerSecondSquaredKelvin` (hg · m²/(s² · K)), as a `HeatCapacity`.
    pub const HECTOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1, -1) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `DecagramSquareMetrePerSecondSquaredKelvin` (dag · m²/(s² · K)), as a `HeatCapacity`.
    pub const DECAGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1, -2) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `GramSquareMetrePerSecondSquaredKelvin` (g · m²/(s² · K)), as a `HeatCapacity`.
    pub const GRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1, -3) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `DecigramSquareMetrePerSecondSquaredKelvin` (dg · m²/(s² · K)), as a `HeatCapacity`.
    pub const DECIGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1, -4) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `CentigramSquareMetrePerSecondSquaredKelvin` (cg · m²/(s² · K)), as a `HeatCapacity`.
    pub const CENTIGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1, -5) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `MilligramSquareMetrePerSecondSquaredKelvin` (mg · m²/(s² · K)), as a `HeatCapacity`.
    pub const MILLIGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1, -6) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `MicrogramSquareMetrePerSecondSquaredKelvin` (µg · m²/(s² · K)), as a `HeatCapacity`.
    pub const MICROGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1, -9) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `NanogramSquareMetrePerSecondSquaredKelvin` (ng · m²/(s² · K)), as a `HeatCapacity`.
    pub const NANOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1, -12) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `PicogramSquareMetrePerSecondSquaredKelvin` (pg · m²/(s² · K)), as a `HeatCapacity`.
    pub const PICOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1, -15) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `FemtogramSquareMetrePerSecondSquaredKelvin` (fg · m²/(s² · K)), as a `HeatCapacity`.
    pub const FEMTOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1, -18) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `AttogramSquareMetrePerSecondSquaredKelvin` (ag · m²/(s² · K)), as a `HeatCapacity`.
    pub const ATTOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1, -21) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `ZeptogramSquareMetrePerSecondSquaredKelvin` (zg · m²/(s² · K)), as a `HeatCapacity`.
    pub const ZEPTOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1, -24) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `YoctogramSquareMetrePerSecondSquaredKelvin` (yg · m²/(s² · K)), as a `HeatCapacity`.
    pub const YOCTOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1, -27) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `YottajoulePerKelvin` (YJ/K), as a `HeatCapacity`.
    pub const YOTTAJOULE_PER_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::int(1000000000000000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `ZettajoulePerKelvin` (ZJ/K), as a `HeatCapacity`.
    pub const ZETTAJOULE_PER_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::int(1000000000000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `ExajoulePerKelvin` (EJ/K), as a `HeatCapacity`.
    pub const EXAJOULE_PER_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::int(1000000000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `PetajoulePerKelvin` (PJ/K), as a `HeatCapacity`.
    pub const PETAJOULE_PER_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::int(1000000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `TerajoulePerKelvin` (TJ/K), as a `HeatCapacity`.
    pub const TERAJOULE_PER_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::int(1000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `GigajoulePerKelvin` (GJ/K), as a `HeatCapacity`.
    pub const GIGAJOULE_PER_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::int(1000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `MegajoulePerKelvin` (MJ/K), as a `HeatCapacity`.
    pub const MEGAJOULE_PER_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::int(1000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `KilojoulePerKelvin` (kJ/K), as a `HeatCapacity`.
    pub const KILOJOULE_PER_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::int(1000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `HectojoulePerKelvin` (hJ/K), as a `HeatCapacity`.
    pub const HECTOJOULE_PER_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::int(100) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `DecajoulePerKelvin` (daJ/K), as a `HeatCapacity`.
    pub const DECAJOULE_PER_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::int(10) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `JoulePerKelvin` (J/K), as a `HeatCapacity`.
    pub const JOULE_PER_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `DecijoulePerKelvin` (dJ/K), as a `HeatCapacity`.
    pub const DECIJOULE_PER_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1, -1) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `CentijoulePerKelvin` (cJ/K), as a `HeatCapacity`.
    pub const CENTIJOULE_PER_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1, -2) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `MillijoulePerKelvin` (mJ/K), as a `HeatCapacity`.
    pub const MILLIJOULE_PER_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1, -3) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `MicrojoulePerKelvin` (µJ/K), as a `HeatCapacity`.
    pub const MICROJOULE_PER_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1, -6) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `NanojoulePerKelvin` (nJ/K), as a `HeatCapacity`.
    pub const NANOJOULE_PER_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1, -9) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `PicojoulePerKelvin` (pJ/K), as a `HeatCapacity`.
    pub const PICOJOULE_PER_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1, -12) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `FemtojoulePerKelvin` (fJ/K), as a `HeatCapacity`.
    pub const FEMTOJOULE_PER_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1, -15) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `AttojoulePerKelvin` (aJ/K), as a `HeatCapacity`.
    pub const ATTOJOULE_PER_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1, -18) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `ZeptojoulePerKelvin` (zJ/K), as a `HeatCapacity`.
    pub const ZEPTOJOULE_PER_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1, -21) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `YoctojoulePerKelvin` (yJ/K), as a `HeatCapacity`.
    pub const YOCTOJOULE_PER_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1, -24) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `KilojoulePerDegreeCelsius` (kJ/°C), as a `HeatCapacity`.
    pub const KILOJOULE_PER_DEGREE_CELSIUS<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::int(1000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `JoulePerDegreeCelsius` (J/°C), as a `HeatCapacity`.
    pub const JOULE_PER_DEGREE_CELSIUS<N: const From<Exact>>: HeatCapacity<N> = counted(KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `MillijoulePerDegreeCelsius` (mJ/°C), as a `HeatCapacity`.
    pub const MILLIJOULE_PER_DEGREE_CELSIUS<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1, -3) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `BtuPerDegreeFahrenheit` (Btu/°F), as a `HeatCapacity`.
    pub const BTU_PER_DEGREE_FAHRENHEIT<N: const From<Exact>>: HeatCapacity<N> = counted(((Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1)) / (Exact::int(5) / Exact::int(9))) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `BtuItPerDegreeFahrenheit` (Btu (IT)/°F), as a `HeatCapacity`.
    pub const BTU_IT_PER_DEGREE_FAHRENHEIT<N: const From<Exact>>: HeatCapacity<N> = counted((Exact::dec(105505585262, -8) / (Exact::int(5) / Exact::int(9))) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `BoltzmannConstant` (k), as a `HeatCapacity`.
    pub const BOLTZMANN_CONSTANT<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(1380649, -29) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `CaloriePerKelvin` (cal/K), as a `HeatCapacity`.
    pub const CALORIE_PER_KELVIN<N: const From<Exact>>: HeatCapacity<N> = counted(Exact::dec(4184, -3) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `YottawattPerSquareMetre` (YW/m²), as a `HeatFluxDensity`.
    pub const YOTTAWATT_PER_SQUARE_METRE<N: const From<Exact>>: HeatFluxDensity<N> = counted(Exact::int(1000000000000000000000000) * WATT_PER_SQUARE_METRE::<Exact>);

    /// One `ZettawattPerSquareMetre` (ZW/m²), as a `HeatFluxDensity`.
    pub const ZETTAWATT_PER_SQUARE_METRE<N: const From<Exact>>: HeatFluxDensity<N> = counted(Exact::int(1000000000000000000000) * WATT_PER_SQUARE_METRE::<Exact>);

    /// One `ExawattPerSquareMetre` (EW/m²), as a `HeatFluxDensity`.
    pub const EXAWATT_PER_SQUARE_METRE<N: const From<Exact>>: HeatFluxDensity<N> = counted(Exact::int(1000000000000000000) * WATT_PER_SQUARE_METRE::<Exact>);

    /// One `PetawattPerSquareMetre` (PW/m²), as a `HeatFluxDensity`.
    pub const PETAWATT_PER_SQUARE_METRE<N: const From<Exact>>: HeatFluxDensity<N> = counted(Exact::int(1000000000000000) * WATT_PER_SQUARE_METRE::<Exact>);

    /// One `TerawattPerSquareMetre` (TW/m²), as a `HeatFluxDensity`.
    pub const TERAWATT_PER_SQUARE_METRE<N: const From<Exact>>: HeatFluxDensity<N> = counted(Exact::int(1000000000000) * WATT_PER_SQUARE_METRE::<Exact>);

    /// One `GigawattPerSquareMetre` (GW/m²), as a `HeatFluxDensity`.
    pub const GIGAWATT_PER_SQUARE_METRE<N: const From<Exact>>: HeatFluxDensity<N> = counted(Exact::int(1000000000) * WATT_PER_SQUARE_METRE::<Exact>);

    /// One `MegawattPerSquareMetre` (MW/m²), as a `HeatFluxDensity`.
    pub const MEGAWATT_PER_SQUARE_METRE<N: const From<Exact>>: HeatFluxDensity<N> = counted(Exact::int(1000000) * WATT_PER_SQUARE_METRE::<Exact>);

    /// One `KilowattPerSquareMetre` (kW/m²), as a `HeatFluxDensity`.
    pub const KILOWATT_PER_SQUARE_METRE<N: const From<Exact>>: HeatFluxDensity<N> = counted(Exact::int(1000) * WATT_PER_SQUARE_METRE::<Exact>);

    /// One `HectowattPerSquareMetre` (hW/m²), as a `HeatFluxDensity`.
    pub const HECTOWATT_PER_SQUARE_METRE<N: const From<Exact>>: HeatFluxDensity<N> = counted(Exact::int(100) * WATT_PER_SQUARE_METRE::<Exact>);

    /// One `DecawattPerSquareMetre` (daW/m²), as a `HeatFluxDensity`.
    pub const DECAWATT_PER_SQUARE_METRE<N: const From<Exact>>: HeatFluxDensity<N> = counted(Exact::int(10) * WATT_PER_SQUARE_METRE::<Exact>);

    /// One `WattPerSquareMetre` (W/m²), as a `HeatFluxDensity`.
    pub const WATT_PER_SQUARE_METRE<N: const From<Exact>>: HeatFluxDensity<N> = counted(KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact>));

    /// One `DeciwattPerSquareMetre` (dW/m²), as a `HeatFluxDensity`.
    pub const DECIWATT_PER_SQUARE_METRE<N: const From<Exact>>: HeatFluxDensity<N> = counted(Exact::dec(1, -1) * WATT_PER_SQUARE_METRE::<Exact>);

    /// One `CentiwattPerSquareMetre` (cW/m²), as a `HeatFluxDensity`.
    pub const CENTIWATT_PER_SQUARE_METRE<N: const From<Exact>>: HeatFluxDensity<N> = counted(Exact::dec(1, -2) * WATT_PER_SQUARE_METRE::<Exact>);

    /// One `MilliwattPerSquareMetre` (mW/m²), as a `HeatFluxDensity`.
    pub const MILLIWATT_PER_SQUARE_METRE<N: const From<Exact>>: HeatFluxDensity<N> = counted(Exact::dec(1, -3) * WATT_PER_SQUARE_METRE::<Exact>);

    /// One `MicrowattPerSquareMetre` (µW/m²), as a `HeatFluxDensity`.
    pub const MICROWATT_PER_SQUARE_METRE<N: const From<Exact>>: HeatFluxDensity<N> = counted(Exact::dec(1, -6) * WATT_PER_SQUARE_METRE::<Exact>);

    /// One `NanowattPerSquareMetre` (nW/m²), as a `HeatFluxDensity`.
    pub const NANOWATT_PER_SQUARE_METRE<N: const From<Exact>>: HeatFluxDensity<N> = counted(Exact::dec(1, -9) * WATT_PER_SQUARE_METRE::<Exact>);

    /// One `PicowattPerSquareMetre` (pW/m²), as a `HeatFluxDensity`.
    pub const PICOWATT_PER_SQUARE_METRE<N: const From<Exact>>: HeatFluxDensity<N> = counted(Exact::dec(1, -12) * WATT_PER_SQUARE_METRE::<Exact>);

    /// One `FemtowattPerSquareMetre` (fW/m²), as a `HeatFluxDensity`.
    pub const FEMTOWATT_PER_SQUARE_METRE<N: const From<Exact>>: HeatFluxDensity<N> = counted(Exact::dec(1, -15) * WATT_PER_SQUARE_METRE::<Exact>);

    /// One `AttowattPerSquareMetre` (aW/m²), as a `HeatFluxDensity`.
    pub const ATTOWATT_PER_SQUARE_METRE<N: const From<Exact>>: HeatFluxDensity<N> = counted(Exact::dec(1, -18) * WATT_PER_SQUARE_METRE::<Exact>);

    /// One `ZeptowattPerSquareMetre` (zW/m²), as a `HeatFluxDensity`.
    pub const ZEPTOWATT_PER_SQUARE_METRE<N: const From<Exact>>: HeatFluxDensity<N> = counted(Exact::dec(1, -21) * WATT_PER_SQUARE_METRE::<Exact>);

    /// One `YoctowattPerSquareMetre` (yW/m²), as a `HeatFluxDensity`.
    pub const YOCTOWATT_PER_SQUARE_METRE<N: const From<Exact>>: HeatFluxDensity<N> = counted(Exact::dec(1, -24) * WATT_PER_SQUARE_METRE::<Exact>);

    /// One `YottagramPerSecondCubedKelvin` (Yg/(s³ · K)), as a `HeatTransfer`.
    pub const YOTTAGRAM_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::int(1000000000000000000000) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `ZettagramPerSecondCubedKelvin` (Zg/(s³ · K)), as a `HeatTransfer`.
    pub const ZETTAGRAM_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::int(1000000000000000000) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `ExagramPerSecondCubedKelvin` (Eg/(s³ · K)), as a `HeatTransfer`.
    pub const EXAGRAM_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::int(1000000000000000) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `PetagramPerSecondCubedKelvin` (Pg/(s³ · K)), as a `HeatTransfer`.
    pub const PETAGRAM_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::int(1000000000000) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `TeragramPerSecondCubedKelvin` (Tg/(s³ · K)), as a `HeatTransfer`.
    pub const TERAGRAM_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::int(1000000000) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `GigagramPerSecondCubedKelvin` (Gg/(s³ · K)), as a `HeatTransfer`.
    pub const GIGAGRAM_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::int(1000000) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `MegagramPerSecondCubedKelvin` (Mg/(s³ · K)), as a `HeatTransfer`.
    pub const MEGAGRAM_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::int(1000) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `KilogramPerSecondCubedKelvin` (kg/(s³ · K)), as a `HeatTransfer`.
    pub const KILOGRAM_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * KELVIN::<Exact>));

    /// One `HectogramPerSecondCubedKelvin` (hg/(s³ · K)), as a `HeatTransfer`.
    pub const HECTOGRAM_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::dec(1, -1) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `DecagramPerSecondCubedKelvin` (dag/(s³ · K)), as a `HeatTransfer`.
    pub const DECAGRAM_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::dec(1, -2) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `GramPerSecondCubedKelvin` (g/(s³ · K)), as a `HeatTransfer`.
    pub const GRAM_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::dec(1, -3) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `DecigramPerSecondCubedKelvin` (dg/(s³ · K)), as a `HeatTransfer`.
    pub const DECIGRAM_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::dec(1, -4) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `CentigramPerSecondCubedKelvin` (cg/(s³ · K)), as a `HeatTransfer`.
    pub const CENTIGRAM_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::dec(1, -5) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `MilligramPerSecondCubedKelvin` (mg/(s³ · K)), as a `HeatTransfer`.
    pub const MILLIGRAM_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::dec(1, -6) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `MicrogramPerSecondCubedKelvin` (µg/(s³ · K)), as a `HeatTransfer`.
    pub const MICROGRAM_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::dec(1, -9) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `NanogramPerSecondCubedKelvin` (ng/(s³ · K)), as a `HeatTransfer`.
    pub const NANOGRAM_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::dec(1, -12) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `PicogramPerSecondCubedKelvin` (pg/(s³ · K)), as a `HeatTransfer`.
    pub const PICOGRAM_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::dec(1, -15) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `FemtogramPerSecondCubedKelvin` (fg/(s³ · K)), as a `HeatTransfer`.
    pub const FEMTOGRAM_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::dec(1, -18) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `AttogramPerSecondCubedKelvin` (ag/(s³ · K)), as a `HeatTransfer`.
    pub const ATTOGRAM_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::dec(1, -21) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `ZeptogramPerSecondCubedKelvin` (zg/(s³ · K)), as a `HeatTransfer`.
    pub const ZEPTOGRAM_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::dec(1, -24) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `YoctogramPerSecondCubedKelvin` (yg/(s³ · K)), as a `HeatTransfer`.
    pub const YOCTOGRAM_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::dec(1, -27) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `KilogramPerSecondCubedDegreeCelsius` (kg/(s³ · °C)), as a `HeatTransfer`.
    pub const KILOGRAM_PER_SECOND_CUBED_DEGREE_CELSIUS<N: const From<Exact>>: HeatTransfer<N> = counted(KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `YottawattPerSquareMetreKelvin` (YW/(m² · K)), as a `HeatTransfer`.
    pub const YOTTAWATT_PER_SQUARE_METRE_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::int(1000000000000000000000000) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `ZettawattPerSquareMetreKelvin` (ZW/(m² · K)), as a `HeatTransfer`.
    pub const ZETTAWATT_PER_SQUARE_METRE_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::int(1000000000000000000000) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `ExawattPerSquareMetreKelvin` (EW/(m² · K)), as a `HeatTransfer`.
    pub const EXAWATT_PER_SQUARE_METRE_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::int(1000000000000000000) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `PetawattPerSquareMetreKelvin` (PW/(m² · K)), as a `HeatTransfer`.
    pub const PETAWATT_PER_SQUARE_METRE_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::int(1000000000000000) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `TerawattPerSquareMetreKelvin` (TW/(m² · K)), as a `HeatTransfer`.
    pub const TERAWATT_PER_SQUARE_METRE_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::int(1000000000000) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `GigawattPerSquareMetreKelvin` (GW/(m² · K)), as a `HeatTransfer`.
    pub const GIGAWATT_PER_SQUARE_METRE_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::int(1000000000) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `MegawattPerSquareMetreKelvin` (MW/(m² · K)), as a `HeatTransfer`.
    pub const MEGAWATT_PER_SQUARE_METRE_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::int(1000000) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `KilowattPerSquareMetreKelvin` (kW/(m² · K)), as a `HeatTransfer`.
    pub const KILOWATT_PER_SQUARE_METRE_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::int(1000) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `HectowattPerSquareMetreKelvin` (hW/(m² · K)), as a `HeatTransfer`.
    pub const HECTOWATT_PER_SQUARE_METRE_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::int(100) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `DecawattPerSquareMetreKelvin` (daW/(m² · K)), as a `HeatTransfer`.
    pub const DECAWATT_PER_SQUARE_METRE_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::int(10) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `WattPerSquareMetreKelvin` (W/(m² · K)), as a `HeatTransfer`.
    pub const WATT_PER_SQUARE_METRE_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `DeciwattPerSquareMetreKelvin` (dW/(m² · K)), as a `HeatTransfer`.
    pub const DECIWATT_PER_SQUARE_METRE_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::dec(1, -1) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `CentiwattPerSquareMetreKelvin` (cW/(m² · K)), as a `HeatTransfer`.
    pub const CENTIWATT_PER_SQUARE_METRE_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::dec(1, -2) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `MilliwattPerSquareMetreKelvin` (mW/(m² · K)), as a `HeatTransfer`.
    pub const MILLIWATT_PER_SQUARE_METRE_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::dec(1, -3) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `MicrowattPerSquareMetreKelvin` (µW/(m² · K)), as a `HeatTransfer`.
    pub const MICROWATT_PER_SQUARE_METRE_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::dec(1, -6) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `NanowattPerSquareMetreKelvin` (nW/(m² · K)), as a `HeatTransfer`.
    pub const NANOWATT_PER_SQUARE_METRE_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::dec(1, -9) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `PicowattPerSquareMetreKelvin` (pW/(m² · K)), as a `HeatTransfer`.
    pub const PICOWATT_PER_SQUARE_METRE_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::dec(1, -12) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `FemtowattPerSquareMetreKelvin` (fW/(m² · K)), as a `HeatTransfer`.
    pub const FEMTOWATT_PER_SQUARE_METRE_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::dec(1, -15) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `AttowattPerSquareMetreKelvin` (aW/(m² · K)), as a `HeatTransfer`.
    pub const ATTOWATT_PER_SQUARE_METRE_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::dec(1, -18) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `ZeptowattPerSquareMetreKelvin` (zW/(m² · K)), as a `HeatTransfer`.
    pub const ZEPTOWATT_PER_SQUARE_METRE_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::dec(1, -21) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `YoctowattPerSquareMetreKelvin` (yW/(m² · K)), as a `HeatTransfer`.
    pub const YOCTOWATT_PER_SQUARE_METRE_KELVIN<N: const From<Exact>>: HeatTransfer<N> = counted(Exact::dec(1, -24) * KILOGRAM_PER_SECOND_CUBED_KELVIN::<Exact>);

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

    /// One `MicrogramMetreSquaredPerSecondCubedKelvin` (µg · m/(s³ · K)), as a `ThermalConductance`.
    pub const MICROGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::dec(1, -9) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

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

    /// One `YottawattPerKelvin` (YW/K), as a `ThermalConductance`.
    pub const YOTTAWATT_PER_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::int(1000000000000000000000000) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `ZettawattPerKelvin` (ZW/K), as a `ThermalConductance`.
    pub const ZETTAWATT_PER_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::int(1000000000000000000000) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `ExawattPerKelvin` (EW/K), as a `ThermalConductance`.
    pub const EXAWATT_PER_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::int(1000000000000000000) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `PetawattPerKelvin` (PW/K), as a `ThermalConductance`.
    pub const PETAWATT_PER_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::int(1000000000000000) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `TerawattPerKelvin` (TW/K), as a `ThermalConductance`.
    pub const TERAWATT_PER_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::int(1000000000000) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `GigawattPerKelvin` (GW/K), as a `ThermalConductance`.
    pub const GIGAWATT_PER_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::int(1000000000) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `MegawattPerKelvin` (MW/K), as a `ThermalConductance`.
    pub const MEGAWATT_PER_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::int(1000000) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `KilowattPerKelvin` (kW/K), as a `ThermalConductance`.
    pub const KILOWATT_PER_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::int(1000) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `HectowattPerKelvin` (hW/K), as a `ThermalConductance`.
    pub const HECTOWATT_PER_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::int(100) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `DecawattPerKelvin` (daW/K), as a `ThermalConductance`.
    pub const DECAWATT_PER_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::int(10) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `WattPerKelvin` (W/K), as a `ThermalConductance`.
    pub const WATT_PER_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `DeciwattPerKelvin` (dW/K), as a `ThermalConductance`.
    pub const DECIWATT_PER_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::dec(1, -1) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `CentiwattPerKelvin` (cW/K), as a `ThermalConductance`.
    pub const CENTIWATT_PER_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::dec(1, -2) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `MilliwattPerKelvin` (mW/K), as a `ThermalConductance`.
    pub const MILLIWATT_PER_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::dec(1, -3) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `MicrowattPerKelvin` (µW/K), as a `ThermalConductance`.
    pub const MICROWATT_PER_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::dec(1, -6) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `NanowattPerKelvin` (nW/K), as a `ThermalConductance`.
    pub const NANOWATT_PER_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::dec(1, -9) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `PicowattPerKelvin` (pW/K), as a `ThermalConductance`.
    pub const PICOWATT_PER_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::dec(1, -12) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `FemtowattPerKelvin` (fW/K), as a `ThermalConductance`.
    pub const FEMTOWATT_PER_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::dec(1, -15) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `AttowattPerKelvin` (aW/K), as a `ThermalConductance`.
    pub const ATTOWATT_PER_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::dec(1, -18) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `ZeptowattPerKelvin` (zW/K), as a `ThermalConductance`.
    pub const ZEPTOWATT_PER_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::dec(1, -21) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `YoctowattPerKelvin` (yW/K), as a `ThermalConductance`.
    pub const YOCTOWATT_PER_KELVIN<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::dec(1, -24) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `KilogramMetreSquaredPerSecondCubedDegreeCelsius` (kg · m²/(s³ · °C)), as a `ThermalConductance`.
    pub const KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_DEGREE_CELSIUS<N: const From<Exact>>: ThermalConductance<N> = counted(KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `KilowattPerDegreeCelsius` (kW/°C), as a `ThermalConductance`.
    pub const KILOWATT_PER_DEGREE_CELSIUS<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::int(1000) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `WattPerMetreDegreeCelsius` (W/°C), as a `ThermalConductance`.
    pub const THERMAL_CONDUCTANCE_WATT_PER_METRE_DEGREE_CELSIUS<N: const From<Exact>>: ThermalConductance<N> = counted(KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `MilliwattPerDegreeCelsius` (mW/°C), as a `ThermalConductance`.
    pub const MILLIWATT_PER_DEGREE_CELSIUS<N: const From<Exact>>: ThermalConductance<N> = counted(Exact::dec(1, -3) * KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `YottagramMetrePerSecondCubedKelvin` (Yg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const YOTTAGRAM_METRE_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::int(1000000000000000000000) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `ZettagramMetrePerSecondCubedKelvin` (Zg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const ZETTAGRAM_METRE_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::int(1000000000000000000) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `ExagramMetrePerSecondCubedKelvin` (Eg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const EXAGRAM_METRE_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::int(1000000000000000) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `PetagramMetrePerSecondCubedKelvin` (Pg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const PETAGRAM_METRE_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::int(1000000000000) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `TeragramMetrePerSecondCubedKelvin` (Tg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const TERAGRAM_METRE_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::int(1000000000) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `GigagramMetrePerSecondCubedKelvin` (Gg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const GIGAGRAM_METRE_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::int(1000000) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `MegagramMetrePerSecondCubedKelvin` (Mg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const MEGAGRAM_METRE_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::int(1000) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `KilogramMetrePerSecondCubedKelvin` (kg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * KELVIN::<Exact>));

    /// One `HectogramMetrePerSecondCubedKelvin` (hg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const HECTOGRAM_METRE_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::dec(1, -1) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `DecagramMetrePerSecondCubedKelvin` (dag · m/(s³ · K)), as a `ThermalConductivity`.
    pub const DECAGRAM_METRE_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::dec(1, -2) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `GramMetrePerSecondCubedKelvin` (g · m/(s³ · K)), as a `ThermalConductivity`.
    pub const GRAM_METRE_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::dec(1, -3) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `DecigramMetrePerSecondCubedKelvin` (dg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const DECIGRAM_METRE_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::dec(1, -4) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `CentigramMetrePerSecondCubedKelvin` (cg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const CENTIGRAM_METRE_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::dec(1, -5) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `MilligramMetrePerSecondCubedKelvin` (mg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const MILLIGRAM_METRE_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::dec(1, -6) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `MicrogramMetrePerSecondCubedKelvin` (µg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const MICROGRAM_METRE_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::dec(1, -9) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `NanogramMetrePerSecondCubedKelvin` (ng · m/(s³ · K)), as a `ThermalConductivity`.
    pub const NANOGRAM_METRE_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::dec(1, -12) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `PicogramMetrePerSecondCubedKelvin` (pg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const PICOGRAM_METRE_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::dec(1, -15) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `FemtogramMetrePerSecondCubedKelvin` (fg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const FEMTOGRAM_METRE_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::dec(1, -18) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `AttogramMetrePerSecondCubedKelvin` (ag · m/(s³ · K)), as a `ThermalConductivity`.
    pub const ATTOGRAM_METRE_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::dec(1, -21) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `ZeptogramMetrePerSecondCubedKelvin` (zg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const ZEPTOGRAM_METRE_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::dec(1, -24) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `YoctogramMetrePerSecondCubedKelvin` (yg · m/(s³ · K)), as a `ThermalConductivity`.
    pub const YOCTOGRAM_METRE_PER_SECOND_CUBED_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::dec(1, -27) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `YottawattPerMetreKelvin` (YW/(m · K)), as a `ThermalConductivity`.
    pub const YOTTAWATT_PER_METRE_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::int(1000000000000000000000000) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `ZettawattPerMetreKelvin` (ZW/(m · K)), as a `ThermalConductivity`.
    pub const ZETTAWATT_PER_METRE_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::int(1000000000000000000000) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `ExawattPerMetreKelvin` (EW/(m · K)), as a `ThermalConductivity`.
    pub const EXAWATT_PER_METRE_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::int(1000000000000000000) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `PetawattPerMetreKelvin` (PW/(m · K)), as a `ThermalConductivity`.
    pub const PETAWATT_PER_METRE_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::int(1000000000000000) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `TerawattPerMetreKelvin` (TW/(m · K)), as a `ThermalConductivity`.
    pub const TERAWATT_PER_METRE_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::int(1000000000000) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `GigawattPerMetreKelvin` (GW/(m · K)), as a `ThermalConductivity`.
    pub const GIGAWATT_PER_METRE_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::int(1000000000) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `MegawattPerMetreKelvin` (MW/(m · K)), as a `ThermalConductivity`.
    pub const MEGAWATT_PER_METRE_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::int(1000000) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `KilowattPerMetreKelvin` (kW/(m · K)), as a `ThermalConductivity`.
    pub const KILOWATT_PER_METRE_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::int(1000) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `HectowattPerMetreKelvin` (hW/(m · K)), as a `ThermalConductivity`.
    pub const HECTOWATT_PER_METRE_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::int(100) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `DecawattPerMetreKelvin` (daW/(m · K)), as a `ThermalConductivity`.
    pub const DECAWATT_PER_METRE_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::int(10) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `WattPerMetreKelvin` (W/(m · K)), as a `ThermalConductivity`.
    pub const WATT_PER_METRE_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `DeciwattPerMetreKelvin` (dW/(m · K)), as a `ThermalConductivity`.
    pub const DECIWATT_PER_METRE_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::dec(1, -1) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `CentiwattPerMetreKelvin` (cW/(m · K)), as a `ThermalConductivity`.
    pub const CENTIWATT_PER_METRE_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::dec(1, -2) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `MilliwattPerMetreKelvin` (mW/(m · K)), as a `ThermalConductivity`.
    pub const MILLIWATT_PER_METRE_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::dec(1, -3) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `MicrowattPerMetreKelvin` (µW/(m · K)), as a `ThermalConductivity`.
    pub const MICROWATT_PER_METRE_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::dec(1, -6) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `NanowattPerMetreKelvin` (nW/(m · K)), as a `ThermalConductivity`.
    pub const NANOWATT_PER_METRE_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::dec(1, -9) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `PicowattPerMetreKelvin` (pW/(m · K)), as a `ThermalConductivity`.
    pub const PICOWATT_PER_METRE_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::dec(1, -12) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `FemtowattPerMetreKelvin` (fW/(m · K)), as a `ThermalConductivity`.
    pub const FEMTOWATT_PER_METRE_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::dec(1, -15) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `AttowattPerMetreKelvin` (aW/(m · K)), as a `ThermalConductivity`.
    pub const ATTOWATT_PER_METRE_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::dec(1, -18) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `ZeptowattPerMetreKelvin` (zW/(m · K)), as a `ThermalConductivity`.
    pub const ZEPTOWATT_PER_METRE_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::dec(1, -21) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `YoctowattPerMetreKelvin` (yW/(m · K)), as a `ThermalConductivity`.
    pub const YOCTOWATT_PER_METRE_KELVIN<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::dec(1, -24) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `KilogramMetrePerSecondCubedDegreeCelsius` (kg · m/(s³ · °C)), as a `ThermalConductivity`.
    pub const KILOGRAM_METRE_PER_SECOND_CUBED_DEGREE_CELSIUS<N: const From<Exact>>: ThermalConductivity<N> = counted(KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `KilowattPerMetreDegreeCelsius` (kW/(m · °C)), as a `ThermalConductivity`.
    pub const KILOWATT_PER_METRE_DEGREE_CELSIUS<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::int(1000) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `WattPerMetreDegreeCelsius` (W/(m · °C)), as a `ThermalConductivity`.
    pub const THERMAL_CONDUCTIVITY_WATT_PER_METRE_DEGREE_CELSIUS<N: const From<Exact>>: ThermalConductivity<N> = counted(KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `MilliwattPerMetreDegreeCelsius` (mW/(m · °C)), as a `ThermalConductivity`.
    pub const MILLIWATT_PER_METRE_DEGREE_CELSIUS<N: const From<Exact>>: ThermalConductivity<N> = counted(Exact::dec(1, -3) * KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN::<Exact>);

    /// One `KelvinPerYottawatt` (K/YW), as a `ThermalResistance`.
    pub const KELVIN_PER_YOTTAWATT<N: const From<Exact>>: ThermalResistance<N> = counted(Exact::dec(1, -24) * KELVIN_PER_WATT::<Exact>);

    /// One `KelvinPerZettawatt` (K/ZW), as a `ThermalResistance`.
    pub const KELVIN_PER_ZETTAWATT<N: const From<Exact>>: ThermalResistance<N> = counted(Exact::dec(1, -21) * KELVIN_PER_WATT::<Exact>);

    /// One `KelvinPerExawatt` (K/EW), as a `ThermalResistance`.
    pub const KELVIN_PER_EXAWATT<N: const From<Exact>>: ThermalResistance<N> = counted(Exact::dec(1, -18) * KELVIN_PER_WATT::<Exact>);

    /// One `KelvinPerPetawatt` (K/PW), as a `ThermalResistance`.
    pub const KELVIN_PER_PETAWATT<N: const From<Exact>>: ThermalResistance<N> = counted(Exact::dec(1, -15) * KELVIN_PER_WATT::<Exact>);

    /// One `KelvinPerTerawatt` (K/TW), as a `ThermalResistance`.
    pub const KELVIN_PER_TERAWATT<N: const From<Exact>>: ThermalResistance<N> = counted(Exact::dec(1, -12) * KELVIN_PER_WATT::<Exact>);

    /// One `KelvinPerGigawatt` (K/GW), as a `ThermalResistance`.
    pub const KELVIN_PER_GIGAWATT<N: const From<Exact>>: ThermalResistance<N> = counted(Exact::dec(1, -9) * KELVIN_PER_WATT::<Exact>);

    /// One `KelvinPerMegawatt` (K/MW), as a `ThermalResistance`.
    pub const KELVIN_PER_MEGAWATT<N: const From<Exact>>: ThermalResistance<N> = counted(Exact::dec(1, -6) * KELVIN_PER_WATT::<Exact>);

    /// One `KelvinPerKilowatt` (K/kw), as a `ThermalResistance`.
    pub const KELVIN_PER_KILOWATT<N: const From<Exact>>: ThermalResistance<N> = counted(Exact::dec(1, -3) * KELVIN_PER_WATT::<Exact>);

    /// One `KelvinPerHectowatt` (K/hW), as a `ThermalResistance`.
    pub const KELVIN_PER_HECTOWATT<N: const From<Exact>>: ThermalResistance<N> = counted(Exact::dec(1, -2) * KELVIN_PER_WATT::<Exact>);

    /// One `KelvinPerDecawatt` (K/daW), as a `ThermalResistance`.
    pub const KELVIN_PER_DECAWATT<N: const From<Exact>>: ThermalResistance<N> = counted(Exact::dec(1, -1) * KELVIN_PER_WATT::<Exact>);

    /// One `KelvinPerWatt` (K/W), as a `ThermalResistance`.
    pub const KELVIN_PER_WATT<N: const From<Exact>>: ThermalResistance<N> = counted(SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * KELVIN::<Exact> / (METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact>));

    /// One `KelvinPerDeciwatt` (K/dW), as a `ThermalResistance`.
    pub const KELVIN_PER_DECIWATT<N: const From<Exact>>: ThermalResistance<N> = counted(Exact::int(10) * KELVIN_PER_WATT::<Exact>);

    /// One `KelvinPerCentiwatt` (K/cW), as a `ThermalResistance`.
    pub const KELVIN_PER_CENTIWATT<N: const From<Exact>>: ThermalResistance<N> = counted(Exact::int(100) * KELVIN_PER_WATT::<Exact>);

    /// One `KelvinPerMilliwatt` (K/mW), as a `ThermalResistance`.
    pub const KELVIN_PER_MILLIWATT<N: const From<Exact>>: ThermalResistance<N> = counted(Exact::int(1000) * KELVIN_PER_WATT::<Exact>);

    /// One `KelvinPerMicrowatt` (K/µW), as a `ThermalResistance`.
    pub const KELVIN_PER_MICROWATT<N: const From<Exact>>: ThermalResistance<N> = counted(Exact::int(1000000) * KELVIN_PER_WATT::<Exact>);

    /// One `KelvinPerNanowatt` (K/nW), as a `ThermalResistance`.
    pub const KELVIN_PER_NANOWATT<N: const From<Exact>>: ThermalResistance<N> = counted(Exact::int(1000000000) * KELVIN_PER_WATT::<Exact>);

    /// One `KelvinPerPicowatt` (K/pW), as a `ThermalResistance`.
    pub const KELVIN_PER_PICOWATT<N: const From<Exact>>: ThermalResistance<N> = counted(Exact::int(1000000000000) * KELVIN_PER_WATT::<Exact>);

    /// One `KelvinPerFemtowatt` (K/fW), as a `ThermalResistance`.
    pub const KELVIN_PER_FEMTOWATT<N: const From<Exact>>: ThermalResistance<N> = counted(Exact::int(1000000000000000) * KELVIN_PER_WATT::<Exact>);

    /// One `KelvinPerAttowatt` (K/aW), as a `ThermalResistance`.
    pub const KELVIN_PER_ATTOWATT<N: const From<Exact>>: ThermalResistance<N> = counted(Exact::int(1000000000000000000) * KELVIN_PER_WATT::<Exact>);

    /// One `KelvinPerZeptowatt` (K/zW), as a `ThermalResistance`.
    pub const KELVIN_PER_ZEPTOWATT<N: const From<Exact>>: ThermalResistance<N> = counted(Exact::int(1000000000000000000000) * KELVIN_PER_WATT::<Exact>);

    /// One `KelvinPerYoctowatt` (K/yW), as a `ThermalResistance`.
    pub const KELVIN_PER_YOCTOWATT<N: const From<Exact>>: ThermalResistance<N> = counted(Exact::int(1000000000000000000000000) * KELVIN_PER_WATT::<Exact>);

    /// One `SquareKilometrePerSecondSquaredKelvin` (km²/(s² · K)), as a `SpecificHeatCapacity`.
    pub const SQUARE_KILOMETRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::int(1000000) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `SquareMetrePerSecondSquaredKelvin` (m²/(s² · K)), as a `SpecificHeatCapacity`.
    pub const SQUARE_METRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(METRE::<Exact> * METRE::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * KELVIN::<Exact>));

    /// One `SquareCentimetrePerSecondSquaredKelvin` (cm²/(s² · K)), as a `SpecificHeatCapacity`.
    pub const SQUARE_CENTIMETRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::dec(1, -4) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `SquareMillimetrePerSecondSquaredKelvin` (mm²/(s² · K)), as a `SpecificHeatCapacity`.
    pub const SQUARE_MILLIMETRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::dec(1, -6) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `SquareMicrometrePerSecondSquaredKelvin` (µm²/(s² · K)), as a `SpecificHeatCapacity`.
    pub const SQUARE_MICROMETRE_PER_SECOND_SQUARED_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::dec(1, -12) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `YottajoulePerKilogramKelvin` (YJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const YOTTAJOULE_PER_KILOGRAM_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::int(1000000000000000000000000) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `ZettajoulePerKilogramKelvin` (ZJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const ZETTAJOULE_PER_KILOGRAM_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::int(1000000000000000000000) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `ExajoulePerKilogramKelvin` (EJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const EXAJOULE_PER_KILOGRAM_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::int(1000000000000000000) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `PetajoulePerKilogramKelvin` (PJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const PETAJOULE_PER_KILOGRAM_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::int(1000000000000000) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `TerajoulePerKilogramKelvin` (TJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const TERAJOULE_PER_KILOGRAM_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::int(1000000000000) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `GigajoulePerKilogramKelvin` (GJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const GIGAJOULE_PER_KILOGRAM_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::int(1000000000) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `MegajoulePerKilogramKelvin` (MJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const MEGAJOULE_PER_KILOGRAM_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::int(1000000) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `KilojoulePerKilogramKelvin` (kJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const KILOJOULE_PER_KILOGRAM_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::int(1000) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `HectojoulePerKilogramKelvin` (hJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const HECTOJOULE_PER_KILOGRAM_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::int(100) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `DecajoulePerKilogramKelvin` (daJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const DECAJOULE_PER_KILOGRAM_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::int(10) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `JoulePerKilogramKelvin` (J/(kg · K)), as a `SpecificHeatCapacity`.
    pub const JOULE_PER_KILOGRAM_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `DecijoulePerKilogramKelvin` (dJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const DECIJOULE_PER_KILOGRAM_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::dec(1, -1) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `CentijoulePerKilogramKelvin` (cJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const CENTIJOULE_PER_KILOGRAM_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::dec(1, -2) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `MillijoulePerKilogramKelvin` (mJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const MILLIJOULE_PER_KILOGRAM_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::dec(1, -3) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `MicrojoulePerKilogramKelvin` (µJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const MICROJOULE_PER_KILOGRAM_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::dec(1, -6) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `NanojoulePerKilogramKelvin` (nJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const NANOJOULE_PER_KILOGRAM_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::dec(1, -9) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `PicojoulePerKilogramKelvin` (pJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const PICOJOULE_PER_KILOGRAM_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::dec(1, -12) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `FemtojoulePerKilogramKelvin` (fJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const FEMTOJOULE_PER_KILOGRAM_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::dec(1, -15) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `AttojoulePerKilogramKelvin` (aJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const ATTOJOULE_PER_KILOGRAM_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::dec(1, -18) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `ZeptojoulePerKilogramKelvin` (zJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const ZEPTOJOULE_PER_KILOGRAM_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::dec(1, -21) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `YoctojoulePerKilogramKelvin` (yJ/(kg · K)), as a `SpecificHeatCapacity`.
    pub const YOCTOJOULE_PER_KILOGRAM_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::dec(1, -24) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `KilojoulePerKilogramDegreeCelsius` (kJ/(kg · °C)), as a `SpecificHeatCapacity`.
    pub const KILOJOULE_PER_KILOGRAM_DEGREE_CELSIUS<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::int(1000) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `KilojoulePerGramDegreeCelsius` (kJ/(g · °C)), as a `SpecificHeatCapacity`.
    pub const KILOJOULE_PER_GRAM_DEGREE_CELSIUS<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::int(1000000) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `JoulePerKilogramDegreeCelsius` (J/(kg · °C)), as a `SpecificHeatCapacity`.
    pub const JOULE_PER_KILOGRAM_DEGREE_CELSIUS<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `JoulePerGramDegreeCelsius` (J/(g · °C)), as a `SpecificHeatCapacity`.
    pub const JOULE_PER_GRAM_DEGREE_CELSIUS<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::int(1000) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

    /// One `MillijoulePerKilogramDegreeCelsius` (mJ/(kg · °C)), as a `SpecificHeatCapacity`.
    pub const MILLIJOULE_PER_KILOGRAM_DEGREE_CELSIUS<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::dec(1, -3) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

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

    /// One `CaloriePerKilogramKelvin` (cal/(kg · K)), as a `SpecificHeatCapacity`.
    pub const CALORIE_PER_KILOGRAM_KELVIN<N: const From<Exact>>: SpecificHeatCapacity<N> = counted(Exact::dec(4184, -3) * SQUARE_METRE_PER_SECOND_SQUARED_KELVIN::<Exact>);

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
