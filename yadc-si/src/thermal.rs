use crate::base::{KILOGRAM, METRE, SECOND};
use crate::{Exact, ISQ, Kind, SIQuantity, Scale};
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
/// (re-exported), then one per named unit, each defined exactly from them
/// (`INCH` is `0.0254 * METRE`). A unit written as others is not one:
/// `SLUG / CUBIC_FOOT`, `KILO * METRE` are exact too. Converting rounds,
/// once, where it is written - in a `const`, by the compiler:
/// `const MPH: Velocity<f64> = (MILE / HOUR).into();`, then `MPH(60.0)`.
pub mod consts {
    use super::*;
    use crate::base::{KILOGRAM, METRE, ONE, SECOND};

    /// The kelvin as a step: a temperature difference of 1 K (the same as 1 °C of difference).
    pub use crate::base::KELVIN;

    /// Absolute zero, 0 K: the zero of thermodynamic temperature.
    pub use crate::base::ABSOLUTE_ZERO;

    /// 0 °C, the ice point of water: 273.15 K.
    pub const ICE_POINT: ThermodynamicTemperature<Exact> =
        ABSOLUTE_ZERO + KELVIN * Exact::dec(27315, -2);

    /// 0 °F: 32 °F below the ice point.
    pub const FAHRENHEIT_ZERO: ThermodynamicTemperature<Exact> =
        ICE_POINT - DEGREE_FAHRENHEIT * Exact::int(32);

    /// The kelvin scale (K): kelvins from absolute zero.
    pub const KELVIN_SCALE: Scale<ThermodynamicTemperature<Exact>, TemperatureInterval<Exact>> =
        Scale(ABSOLUTE_ZERO, KELVIN);

    /// The Celsius scale (°C): kelvins from the ice point.
    pub const CELSIUS: Scale<ThermodynamicTemperature<Exact>, TemperatureInterval<Exact>> =
        Scale(ICE_POINT, KELVIN);

    /// The Fahrenheit scale (°F): degrees Fahrenheit from 32 °F below the ice point.
    pub const FAHRENHEIT: Scale<ThermodynamicTemperature<Exact>, TemperatureInterval<Exact>> =
        Scale(FAHRENHEIT_ZERO, DEGREE_FAHRENHEIT);

    /// The Rankine scale (°R): degrees Fahrenheit from absolute zero.
    pub const RANKINE: Scale<ThermodynamicTemperature<Exact>, TemperatureInterval<Exact>> =
        Scale(ABSOLUTE_ZERO, DEGREE_FAHRENHEIT);

    /// One `Fahrenheit` (°F), as a `TemperatureInterval`.
    pub const DEGREE_FAHRENHEIT: TemperatureInterval<Exact> =
        KELVIN * (Exact::int(5) / Exact::int(9));

    /// One `PerKelvin` (K⁻¹), as a `TemperatureCoefficient`.
    pub const PER_KELVIN: TemperatureCoefficient<Exact> = ONE / KELVIN;

    /// One `BoltzmannConstant` (k), as a `HeatCapacity`.
    pub const BOLTZMANN_CONSTANT: HeatCapacity<Exact> =
        (METRE * METRE * KILOGRAM / (SECOND * SECOND * KELVIN)) * Exact::dec(1380649, -29);

    /// One `StefanBoltzmannConstant` (σ), as a `StefanBoltzmannCoefficient`.
    pub const STEFAN_BOLTZMANN_CONSTANT: StefanBoltzmannCoefficient<Exact> = (KILOGRAM
        / (SECOND * SECOND * SECOND * KELVIN * KELVIN * KELVIN * KELVIN))
        * Exact::dec(5670374419, -17);
}

/// The units of `ThermodynamicTemperature`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ThermodynamicTemperature<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Scale<ThermodynamicTemperature<N>, TemperatureInterval<N>>: [const] yadc::Unit<ThermodynamicTemperature<N>, N> + [const] core::marker::Destruct)]
pub enum ThermodynamicTemperatureUnit {
    #[default]
    #[unit("K", "kelvin", by = Scale::<ThermodynamicTemperature<N>, TemperatureInterval<N>>::from(KELVIN_SCALE))]
    Kelvin,
    #[unit("°C", "degrees Celsius", by = Scale::<ThermodynamicTemperature<N>, TemperatureInterval<N>>::from(CELSIUS))]
    Celsius,
    #[unit("°F", "degrees Fahrenheit", by = Scale::<ThermodynamicTemperature<N>, TemperatureInterval<N>>::from(FAHRENHEIT))]
    Fahrenheit,
    #[unit("°R", "degrees Rankine", by = Scale::<ThermodynamicTemperature<N>, TemperatureInterval<N>>::from(RANKINE))]
    Rankine,
}

/// The units of `TemperatureInterval`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = TemperatureInterval<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, TemperatureInterval<N>: [const] yadc::Unit<TemperatureInterval<N>, N> + [const] core::marker::Destruct)]
pub enum TemperatureIntervalUnit {
    #[default]
    #[unit("K", "kelvin", by = TemperatureInterval::<N>::from(KELVIN))]
    Kelvin,
    #[unit("°C", "degrees Celsius", by = TemperatureInterval::<N>::from(KELVIN))]
    Celsius,
    #[unit("°F", "degrees Fahrenheit", by = TemperatureInterval::<N>::from(DEGREE_FAHRENHEIT))]
    Fahrenheit,
    #[unit("°R", "degrees Rankine", by = TemperatureInterval::<N>::from(DEGREE_FAHRENHEIT))]
    Rankine,
}

/// The units of `TemperatureCoefficient`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = TemperatureCoefficient<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, TemperatureCoefficient<N>: [const] yadc::Unit<TemperatureCoefficient<N>, N> + [const] core::marker::Destruct)]
pub enum TemperatureCoefficientUnit {
    #[default]
    #[unit("K⁻¹", "per kelvin", by = TemperatureCoefficient::<N>::from(PER_KELVIN))]
    PerKelvin,
    #[unit("ppm/K", "ppm per kelvin", by = TemperatureCoefficient::<N>::from(PER_KELVIN * Exact::dec(1, -6)))]
    PpmPerKelvin,
    #[unit("ppm/°C", "ppm per degree Celsius", by = TemperatureCoefficient::<N>::from(PER_KELVIN * Exact::dec(1, -6)))]
    PpmPerDegreeCelsius,
}

/// The units of `TemperatureGradient`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = TemperatureGradient<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, TemperatureGradient<N>: [const] yadc::Unit<TemperatureGradient<N>, N> + [const] core::marker::Destruct)]
pub enum TemperatureGradientUnit {
    #[default]
    #[unit("K/m", "kelvins per metre", by = TemperatureGradient::<N>::from(KELVIN / METRE))]
    KelvinPerMetre,
}

/// The units of `HeatCapacity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = HeatCapacity<N>, number = N)]
#[unit(for<N> where HeatCapacity<N>: [const] yadc::Unit<HeatCapacity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum HeatCapacityUnit {
    #[default]
    #[unit("kg · m²/(s² · K)", "kilogram square metres per second squared kelvin", by = HeatCapacity::<N>::from(METRE * METRE * KILOGRAM / (SECOND * SECOND * KELVIN)))]
    KilogramSquareMetrePerSecondSquaredKelvin,
    #[unit("g · m²/(s² · K)", "gram square metres per second squared kelvin", by = HeatCapacity::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * KELVIN)) * Exact::dec(1, -3)))]
    GramSquareMetrePerSecondSquaredKelvin,
    #[unit("J/K", "joules per kelvin", by = HeatCapacity::<N>::from(METRE * METRE * KILOGRAM / (SECOND * SECOND * KELVIN)))]
    JoulePerKelvin,
    #[unit("J/°C", "joules per degree celsius", by = HeatCapacity::<N>::from(METRE * METRE * KILOGRAM / (SECOND * SECOND * KELVIN)))]
    JoulePerDegreeCelsius,
    #[unit("Btu/°F", "British thermal units per degree Fahrenheit", by = HeatCapacity::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * KELVIN)) * ((Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1)) / (Exact::int(5) / Exact::int(9)))))]
    BtuPerDegreeFahrenheit,
    #[unit("Btu (IT)/°F", "British thermal units (IT) per degree Fahrenheit", by = HeatCapacity::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * KELVIN)) * (Exact::dec(105505585262, -8) / (Exact::int(5) / Exact::int(9)))))]
    BtuItPerDegreeFahrenheit,
    #[unit("k", "Boltzmann constants", by = HeatCapacity::<N>::from(BOLTZMANN_CONSTANT))]
    BoltzmannConstant,
    #[unit("cal/K", "calories per kelvin", by = HeatCapacity::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * KELVIN)) * Exact::dec(4184, -3)))]
    CaloriePerKelvin,
}

/// The units of `HeatFluxDensity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = HeatFluxDensity<N>, number = N)]
#[unit(for<N> where HeatFluxDensity<N>: [const] yadc::Unit<HeatFluxDensity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum HeatFluxDensityUnit {
    #[default]
    #[unit("W/m²", "watts per square metre", by = HeatFluxDensity::<N>::from(KILOGRAM / (SECOND * SECOND * SECOND)))]
    WattPerSquareMetre,
}

/// The units of `HeatTransfer`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = HeatTransfer<N>, number = N)]
#[unit(for<N> where HeatTransfer<N>: [const] yadc::Unit<HeatTransfer<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum HeatTransferUnit {
    #[default]
    #[unit("kg/(s³ · K)", "kilograms per second cubed kelvin", by = HeatTransfer::<N>::from(KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)))]
    KilogramPerSecondCubedKelvin,
    #[unit("g/(s³ · K)", "grams per second cubed kelvin", by = HeatTransfer::<N>::from((KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)) * Exact::dec(1, -3)))]
    GramPerSecondCubedKelvin,
    #[unit("kg/(s³ · °C)", "kilograms per second cubed degree celsius", by = HeatTransfer::<N>::from(KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)))]
    KilogramPerSecondCubedDegreeCelsius,
    #[unit("W/(m² · K)", "watts per square metre kelvin", by = HeatTransfer::<N>::from(KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)))]
    WattPerSquareMetreKelvin,
    #[unit("W/(m² · °C)", "watts per square metre degree celsius", by = HeatTransfer::<N>::from(KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)))]
    WattPerSquareMetreDegreeCelsius,
    #[unit("J/(s · m² · K)", "joules per second square metre kelvin", by = HeatTransfer::<N>::from(KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)))]
    JoulePerSecondSquareMetreKelvin,
    #[unit("J/(s · m² · °C)", "joules per second square metre degree celsius", by = HeatTransfer::<N>::from(KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)))]
    JoulePerSecondSquareMetreDegreeCelsius,
    #[unit("Btu (IT)/(hr · ft² · °F)", "British thermal units (IT) per hour square foot degree Fahrenheit", by = HeatTransfer::<N>::from((KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)) * (Exact::dec(105505585262, -8) / (Exact::int(3600) * Exact::dec(3048, -4) * Exact::dec(3048, -4) * (Exact::int(5) / Exact::int(9))))))]
    BtuItPerHourSquareFootDegreeFahrenheit,
    #[unit("Btu (IT)/(hr · in² · °F)", "British thermal units (IT) per hour square inch degree Fahrenheit", by = HeatTransfer::<N>::from((KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)) * (Exact::dec(105505585262, -8) / (Exact::int(3600) * Exact::dec(254, -4) * Exact::dec(254, -4) * (Exact::int(5) / Exact::int(9))))))]
    BtuItPerHourSquareInchDegreeFahrenheit,
    #[unit("Btu (IT)/(min · ft² · °F)", "British thermal units (IT) per minute square foot degree Fahrenheit", by = HeatTransfer::<N>::from((KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)) * (Exact::dec(105505585262, -8) / (Exact::int(60) * Exact::dec(3048, -4) * Exact::dec(3048, -4) * (Exact::int(5) / Exact::int(9))))))]
    BtuItPerMinuteSquareFootDegreeFahrenheit,
    #[unit("Btu (IT)/(min · in² · °F)", "British thermal units (IT) per minute square inch degree Fahrenheit", by = HeatTransfer::<N>::from((KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)) * (Exact::dec(105505585262, -8) / (Exact::int(60) * Exact::dec(254, -4) * Exact::dec(254, -4) * (Exact::int(5) / Exact::int(9))))))]
    BtuItPerMinuteSquareInchDegreeFahrenheit,
    #[unit("Btu (IT)/(s · ft² · °F)", "British thermal units (IT) per second square foot degree Fahrenheit", by = HeatTransfer::<N>::from((KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)) * (Exact::dec(105505585262, -8) / (Exact::dec(3048, -4) * Exact::dec(3048, -4) * (Exact::int(5) / Exact::int(9))))))]
    BtuItPerSecondSquareFootDegreeFahrenheit,
    #[unit("Btu (IT)/(s · in² · °F)", "British thermal units (IT) per second square inch degree Fahrenheit", by = HeatTransfer::<N>::from((KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)) * (Exact::dec(105505585262, -8) / (Exact::dec(254, -4) * Exact::dec(254, -4) * (Exact::int(5) / Exact::int(9))))))]
    BtuItPerSecondSquareInchDegreeFahrenheit,
    #[unit("Btu/(hr · ft² · °F)", "British thermal units per hour square foot degree Fahrenheit", by = HeatTransfer::<N>::from((KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)) * ((Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1)) / (Exact::int(3600) * Exact::dec(3048, -4) * Exact::dec(3048, -4) * (Exact::int(5) / Exact::int(9))))))]
    BtuPerHourSquareFootDegreeFahrenheit,
    #[unit("Btu/(hr · in² · °F)", "British thermal units per hour square inch degree Fahrenheit", by = HeatTransfer::<N>::from((KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)) * ((Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1)) / (Exact::int(3600) * Exact::dec(254, -4) * Exact::dec(254, -4) * (Exact::int(5) / Exact::int(9))))))]
    BtuPerHourSquareInchDegreeFahrenheit,
    #[unit("Btu/(min · ft² · °F)", "British thermal units per minute square foot degree Fahrenheit", by = HeatTransfer::<N>::from((KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)) * ((Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1)) / (Exact::int(60) * Exact::dec(3048, -4) * Exact::dec(3048, -4) * (Exact::int(5) / Exact::int(9))))))]
    BtuPerMinuteSquareFootDegreeFahrenheit,
    #[unit("Btu/(min · in² · °F)", "British thermal units per minute square inch degree Fahrenheit", by = HeatTransfer::<N>::from((KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)) * ((Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1)) / (Exact::int(60) * Exact::dec(254, -4) * Exact::dec(254, -4) * (Exact::int(5) / Exact::int(9))))))]
    BtuPerMinuteSquareInchDegreeFahrenheit,
    #[unit("Btu/(s · ft² · °F)", "British thermal units per second square foot degree Fahrenheit", by = HeatTransfer::<N>::from((KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)) * ((Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1)) / (Exact::dec(3048, -4) * Exact::dec(3048, -4) * (Exact::int(5) / Exact::int(9))))))]
    BtuPerSecondSquareFootDegreeFahrenheit,
    #[unit("Btu/(s · in² · °F)", "British thermal units per second square inch degree Fahrenheit", by = HeatTransfer::<N>::from((KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)) * ((Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1)) / (Exact::dec(254, -4) * Exact::dec(254, -4) * (Exact::int(5) / Exact::int(9))))))]
    BtuPerSecondSquareInchDegreeFahrenheit,
}

/// The units of `ThermalConductance`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ThermalConductance<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, ThermalConductance<N>: [const] yadc::Unit<ThermalConductance<N>, N> + [const] core::marker::Destruct)]
pub enum ThermalConductanceUnit {
    #[default]
    #[unit("kg · m²/(s³ · K)", "kilograms metre squared per second cubed kelvin", by = ThermalConductance::<N>::from(METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)))]
    KilogramMetreSquaredPerSecondCubedKelvin,
    #[unit("g · m/(s³ · K)", "grams metre squared per second cubed kelvin", by = ThermalConductance::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)) * Exact::dec(1, -3)))]
    GramMetreSquaredPerSecondCubedKelvin,
    #[unit("W/K", "watts per kelvin", by = ThermalConductance::<N>::from(METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)))]
    WattPerKelvin,
    #[unit("kg · m²/(s³ · °C)", "kilograms metre squared per second cubed degree Celsius", by = ThermalConductance::<N>::from(METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)))]
    KilogramMetreSquaredPerSecondCubedDegreeCelsius,
    #[unit("W/°C", "watts per degree Celsius", by = ThermalConductance::<N>::from(METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)))]
    WattPerMetreDegreeCelsius,
    #[unit("kW/°C", "kilowatts per degree Celsius", by = ThermalConductance::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)) * Exact::int(1000)))]
    KilowattPerDegreeCelsius,
    #[unit("mW/°C", "milliwatts per degree Celsius", by = ThermalConductance::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)) * Exact::dec(1, -3)))]
    MilliwattPerDegreeCelsius,
}

/// The units of `ThermalConductivity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ThermalConductivity<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, ThermalConductivity<N>: [const] yadc::Unit<ThermalConductivity<N>, N> + [const] core::marker::Destruct)]
pub enum ThermalConductivityUnit {
    #[default]
    #[unit("kg · m/(s³ · K)", "kilograms metre per second cubed kelvin", by = ThermalConductivity::<N>::from(METRE * KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)))]
    KilogramMetrePerSecondCubedKelvin,
    #[unit("g · m/(s³ · K)", "grams metre per second cubed kelvin", by = ThermalConductivity::<N>::from((METRE * KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)) * Exact::dec(1, -3)))]
    GramMetrePerSecondCubedKelvin,
    #[unit("W/(m · K)", "watts per metre kelvin", by = ThermalConductivity::<N>::from(METRE * KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)))]
    WattPerMetreKelvin,
    #[unit("kg · m/(s³ · °C)", "kilograms metre per second cubed degree celsius", by = ThermalConductivity::<N>::from(METRE * KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)))]
    KilogramMetrePerSecondCubedDegreeCelsius,
    #[unit("W/(m · °C)", "watts per metre degree celsius", by = ThermalConductivity::<N>::from(METRE * KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)))]
    WattPerMetreDegreeCelsius,
}

/// The units of `ThermalResistance`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ThermalResistance<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, ThermalResistance<N>: [const] yadc::Unit<ThermalResistance<N>, N> + [const] core::marker::Destruct)]
pub enum ThermalResistanceUnit {
    #[default]
    #[unit("K/W", "kelvins per watt", by = ThermalResistance::<N>::from(SECOND * SECOND * SECOND * KELVIN / (METRE * METRE * KILOGRAM)))]
    KelvinPerWatt,
    #[unit("K/kW", "kelvins per kilowatt", by = ThermalResistance::<N>::from((SECOND * SECOND * SECOND * KELVIN / (METRE * METRE * KILOGRAM)) * Exact::dec(1, -3)))]
    KelvinPerKilowatt,
}

/// The units of `SpecificHeatCapacity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificHeatCapacity<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, SpecificHeatCapacity<N>: [const] yadc::Unit<SpecificHeatCapacity<N>, N> + [const] core::marker::Destruct)]
pub enum SpecificHeatCapacityUnit {
    #[default]
    #[unit("m²/(s² · K)", "square metres per second squared kelvin", by = SpecificHeatCapacity::<N>::from(METRE * METRE / (SECOND * SECOND * KELVIN)))]
    SquareMetrePerSecondSquaredKelvin,
    #[unit("J/(kg · K)", "joules per kilogram kelvin", by = SpecificHeatCapacity::<N>::from(METRE * METRE / (SECOND * SECOND * KELVIN)))]
    JoulePerKilogramKelvin,
    #[unit("J/(kg · °C)", "joules per kilogram degree celsius", by = SpecificHeatCapacity::<N>::from(METRE * METRE / (SECOND * SECOND * KELVIN)))]
    JoulePerKilogramDegreeCelsius,
    #[unit("J/(g · °C)", "joules per gram degree celsius", by = SpecificHeatCapacity::<N>::from((METRE * METRE / (SECOND * SECOND * KELVIN)) * Exact::int(1000)))]
    JoulePerGramDegreeCelsius,
    #[unit("Btu/(oz · °F)", "British thermal units per ounce degree Fahrenheit", by = SpecificHeatCapacity::<N>::from((METRE * METRE / (SECOND * SECOND * KELVIN)) * (Exact::int(16) * Exact::int(4184))))]
    BtuPerOunceDegreeFahrenheit,
    #[unit("Btu (IT)/(oz · °F)", "British thermal units (IT) per ounce degree Fahrenheit", by = SpecificHeatCapacity::<N>::from((METRE * METRE / (SECOND * SECOND * KELVIN)) * (Exact::int(16) * Exact::dec(41868, -1))))]
    BtuItPerOunceDegreeFahrenheit,
    #[unit("Btu/(lb · °F)", "British thermal units per pound degree Fahrenheit", by = SpecificHeatCapacity::<N>::from((METRE * METRE / (SECOND * SECOND * KELVIN)) * Exact::int(4184)))]
    BtuPerPoundDegreeFahrenheit,
    #[unit("Btu (IT)/(lb · °F)", "British thermal units (IT) per pound degree Fahrenheit", by = SpecificHeatCapacity::<N>::from((METRE * METRE / (SECOND * SECOND * KELVIN)) * Exact::dec(41868, -1)))]
    BtuItPerPoundDegreeFahrenheit,
    #[unit("Btu/(t · °F)", "British thermal units per ton degree Fahrenheit", by = SpecificHeatCapacity::<N>::from((METRE * METRE / (SECOND * SECOND * KELVIN)) * ((Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1)) / (Exact::int(1000) * (Exact::int(5) / Exact::int(9))))))]
    BtuPerTonDegreeFahrenheit,
    #[unit("Btu (IT)/(t · °F)", "British thermal units (IT) per ton degree Fahrenheit", by = SpecificHeatCapacity::<N>::from((METRE * METRE / (SECOND * SECOND * KELVIN)) * (Exact::dec(105505585262, -8) / (Exact::int(1000) * (Exact::int(5) / Exact::int(9))))))]
    BtuItPerTonDegreeFahrenheit,
    #[unit("cal/(g · K)", "calories per gram kelvin", by = SpecificHeatCapacity::<N>::from((METRE * METRE / (SECOND * SECOND * KELVIN)) * Exact::int(4184)))]
    CaloriePerGramKelvin,
}

/// The units of `ArealHeatCapacity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ArealHeatCapacity<N>, number = N)]
#[unit(for<N> where ArealHeatCapacity<N>: [const] yadc::Unit<ArealHeatCapacity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ArealHeatCapacityUnit {
    #[default]
    #[unit("J/(m² · K)", "joules per square metre kelvin", by = ArealHeatCapacity::<N>::from(KILOGRAM / (SECOND * SECOND * KELVIN)))]
    JoulePerSquareMetreKelvin,
}

/// The units of `VolumetricHeatCapacity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = VolumetricHeatCapacity<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, VolumetricHeatCapacity<N>: [const] yadc::Unit<VolumetricHeatCapacity<N>, N> + [const] core::marker::Destruct)]
pub enum VolumetricHeatCapacityUnit {
    #[default]
    #[unit("J/(m³ · K)", "joules per cubic metre kelvin", by = VolumetricHeatCapacity::<N>::from(KILOGRAM / (METRE * SECOND * SECOND * KELVIN)))]
    JoulePerCubicMetreKelvin,
    #[unit("cal/(m³ · K)", "calories per cubic metre kelvin", by = VolumetricHeatCapacity::<N>::from((KILOGRAM / (METRE * SECOND * SECOND * KELVIN)) * Exact::dec(4184, -3)))]
    CaloriePerCubicMetreKelvin,
}

/// The units of `StefanBoltzmannCoefficient`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = StefanBoltzmannCoefficient<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, StefanBoltzmannCoefficient<N>: [const] yadc::Unit<StefanBoltzmannCoefficient<N>, N> + [const] core::marker::Destruct)]
pub enum StefanBoltzmannCoefficientUnit {
    #[default]
    #[unit("W/(m² · K⁴)", "watts per square metre quartic kelvin", by = StefanBoltzmannCoefficient::<N>::from(KILOGRAM / (SECOND * SECOND * SECOND * KELVIN * KELVIN * KELVIN * KELVIN)))]
    WattPerSquareMetreQuarticKelvin,
    #[unit("σ", "Stefan-Boltzmann constants", by = StefanBoltzmannCoefficient::<N>::from(STEFAN_BOLTZMANN_CONSTANT))]
    StefanBoltzmannConstant,
}
