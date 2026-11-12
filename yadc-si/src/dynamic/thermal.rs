//! The units of `thermal`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::base::{KILOGRAM, METRE, SECOND};
use crate::thermal::consts::*;
use crate::thermal::*;
use crate::{Exact, Scale};
use core::marker::Destruct;
use yadc::{Unit, num_traits::Num};

/// The units of `ThermodynamicTemperature`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ThermodynamicTemperature<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Scale<ThermodynamicTemperature<N>, TemperatureInterval<N>>: [const] Unit<ThermodynamicTemperature<N>, N> + [const] Destruct)]
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

/// The units of `TemperatureInterval`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = TemperatureInterval<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where TemperatureInterval<N>: [const] Unit<TemperatureInterval<N>, N> + [const] Destruct)]
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

/// The units of `TemperatureCoefficient`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = TemperatureCoefficient<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where TemperatureCoefficient<N>: [const] Unit<TemperatureCoefficient<N>, N> + [const] Destruct)]
pub enum TemperatureCoefficientUnit {
    #[default]
    #[unit("K⁻¹", "per kelvin", by = TemperatureCoefficient::<N>::from(PER_KELVIN))]
    PerKelvin,
    #[unit("ppm/K", "ppm per kelvin", by = TemperatureCoefficient::<N>::from(PER_KELVIN * Exact::dec(1, -6)))]
    PpmPerKelvin,
    #[unit("ppm/°C", "ppm per degree Celsius", by = TemperatureCoefficient::<N>::from(PER_KELVIN * Exact::dec(1, -6)))]
    PpmPerDegreeCelsius,
}

/// The units of `TemperatureGradient`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = TemperatureGradient<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where TemperatureGradient<N>: [const] Unit<TemperatureGradient<N>, N> + [const] Destruct)]
pub enum TemperatureGradientUnit {
    #[default]
    #[unit("K/m", "kelvins per metre", by = TemperatureGradient::<N>::from(KELVIN / METRE))]
    KelvinPerMetre,
}

/// The units of `HeatCapacity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = HeatCapacity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where HeatCapacity<N>: [const] Unit<HeatCapacity<N>, N> + [const] Destruct)]
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

/// The units of `HeatFluxDensity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = HeatFluxDensity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where HeatFluxDensity<N>: [const] Unit<HeatFluxDensity<N>, N> + [const] Destruct)]
pub enum HeatFluxDensityUnit {
    #[default]
    #[unit("W/m²", "watts per square metre", by = HeatFluxDensity::<N>::from(KILOGRAM / (SECOND * SECOND * SECOND)))]
    WattPerSquareMetre,
}

/// The units of `HeatTransfer`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = HeatTransfer<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where HeatTransfer<N>: [const] Unit<HeatTransfer<N>, N> + [const] Destruct)]
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

/// The units of `ThermalConductance`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ThermalConductance<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ThermalConductance<N>: [const] Unit<ThermalConductance<N>, N> + [const] Destruct)]
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

/// The units of `ThermalConductivity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ThermalConductivity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ThermalConductivity<N>: [const] Unit<ThermalConductivity<N>, N> + [const] Destruct)]
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

/// The units of `ThermalResistance`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ThermalResistance<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ThermalResistance<N>: [const] Unit<ThermalResistance<N>, N> + [const] Destruct)]
pub enum ThermalResistanceUnit {
    #[default]
    #[unit("K/W", "kelvins per watt", by = ThermalResistance::<N>::from(SECOND * SECOND * SECOND * KELVIN / (METRE * METRE * KILOGRAM)))]
    KelvinPerWatt,
    #[unit("K/kW", "kelvins per kilowatt", by = ThermalResistance::<N>::from((SECOND * SECOND * SECOND * KELVIN / (METRE * METRE * KILOGRAM)) * Exact::dec(1, -3)))]
    KelvinPerKilowatt,
}

/// The units of `SpecificHeatCapacity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificHeatCapacity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where SpecificHeatCapacity<N>: [const] Unit<SpecificHeatCapacity<N>, N> + [const] Destruct)]
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

/// The units of `ArealHeatCapacity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ArealHeatCapacity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ArealHeatCapacity<N>: [const] Unit<ArealHeatCapacity<N>, N> + [const] Destruct)]
pub enum ArealHeatCapacityUnit {
    #[default]
    #[unit("J/(m² · K)", "joules per square metre kelvin", by = ArealHeatCapacity::<N>::from(KILOGRAM / (SECOND * SECOND * KELVIN)))]
    JoulePerSquareMetreKelvin,
}

/// The units of `VolumetricHeatCapacity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = VolumetricHeatCapacity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where VolumetricHeatCapacity<N>: [const] Unit<VolumetricHeatCapacity<N>, N> + [const] Destruct)]
pub enum VolumetricHeatCapacityUnit {
    #[default]
    #[unit("J/(m³ · K)", "joules per cubic metre kelvin", by = VolumetricHeatCapacity::<N>::from(KILOGRAM / (METRE * SECOND * SECOND * KELVIN)))]
    JoulePerCubicMetreKelvin,
    #[unit("cal/(m³ · K)", "calories per cubic metre kelvin", by = VolumetricHeatCapacity::<N>::from((KILOGRAM / (METRE * SECOND * SECOND * KELVIN)) * Exact::dec(4184, -3)))]
    CaloriePerCubicMetreKelvin,
}

/// The units of `StefanBoltzmannCoefficient`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = StefanBoltzmannCoefficient<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where StefanBoltzmannCoefficient<N>: [const] Unit<StefanBoltzmannCoefficient<N>, N> + [const] Destruct)]
pub enum StefanBoltzmannCoefficientUnit {
    #[default]
    #[unit("W/(m² · K⁴)", "watts per square metre quartic kelvin", by = StefanBoltzmannCoefficient::<N>::from(KILOGRAM / (SECOND * SECOND * SECOND * KELVIN * KELVIN * KELVIN * KELVIN)))]
    WattPerSquareMetreQuarticKelvin,
    #[unit("σ", "Stefan-Boltzmann constants", by = StefanBoltzmannCoefficient::<N>::from(STEFAN_BOLTZMANN_CONSTANT))]
    StefanBoltzmannConstant,
}
