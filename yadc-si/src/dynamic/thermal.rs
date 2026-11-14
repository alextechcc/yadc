//! The units of `thermal`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::dynamic::DynamicSIUnit;
use crate::thermal::units::*;
use crate::thermal::*;
use crate::{Exact, SIQuantity, Unit};
use core::ops::{Add, Div, Mul, Sub};

/// The units of `ThermodynamicTemperature`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<THERMODYNAMIC_TEMPERATURE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<THERMODYNAMIC_TEMPERATURE, N>;
        fn value(&self, quantity: SIQuantity<THERMODYNAMIC_TEMPERATURE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ThermodynamicTemperatureUnit {
    #[default]
    #[value(KELVIN_SCALE)]
    Kelvin,
    #[value(CELSIUS)]
    Celsius,
    #[value(FAHRENHEIT)]
    Fahrenheit,
    #[value(RANKINE)]
    Rankine,
}

/// The units of `TemperatureInterval`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<TEMPERATURE_INTERVAL, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<TEMPERATURE_INTERVAL, N>;
        fn value(&self, quantity: SIQuantity<TEMPERATURE_INTERVAL, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum TemperatureIntervalUnit {
    #[default]
    #[value(KELVIN)]
    Kelvin,
    #[value(DEGREE_CELSIUS)]
    Celsius,
    #[value(DEGREE_FAHRENHEIT)]
    Fahrenheit,
    #[value(DEGREE_RANKINE)]
    Rankine,
}

/// The units of `TemperatureCoefficient`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<TEMPERATURE_COEFFICIENT, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<TEMPERATURE_COEFFICIENT, N>;
        fn value(&self, quantity: SIQuantity<TEMPERATURE_COEFFICIENT, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum TemperatureCoefficientUnit {
    #[default]
    #[value(PER_KELVIN)]
    PerKelvin,
    #[value(PPM_PER_KELVIN)]
    PpmPerKelvin,
    #[value(PPM_PER_DEGREE_CELSIUS)]
    PpmPerDegreeCelsius,
}

/// The units of `TemperatureGradient`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<TEMPERATURE_GRADIENT, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<TEMPERATURE_GRADIENT, N>;
        fn value(&self, quantity: SIQuantity<TEMPERATURE_GRADIENT, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum TemperatureGradientUnit {
    #[default]
    #[value(KELVIN_PER_METRE)]
    KelvinPerMetre,
}

/// The units of `HeatCapacity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<HEAT_CAPACITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<HEAT_CAPACITY, N>;
        fn value(&self, quantity: SIQuantity<HEAT_CAPACITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum HeatCapacityUnit {
    #[default]
    #[value(KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    KilogramSquareMetrePerSecondSquaredKelvin,
    #[value(GRAM_SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    GramSquareMetrePerSecondSquaredKelvin,
    #[value(JOULE_PER_KELVIN)]
    JoulePerKelvin,
    #[value(JOULE_PER_DEGREE_CELSIUS)]
    JoulePerDegreeCelsius,
    #[value(BTU_PER_DEGREE_FAHRENHEIT)]
    BtuPerDegreeFahrenheit,
    #[value(BTU_IT_PER_DEGREE_FAHRENHEIT)]
    BtuItPerDegreeFahrenheit,
    #[value(BOLTZMANN_CONSTANT)]
    BoltzmannConstant,
    #[value(CALORIE_PER_KELVIN)]
    CaloriePerKelvin,
}

/// The units of `HeatFluxDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<HEAT_FLUX_DENSITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<HEAT_FLUX_DENSITY, N>;
        fn value(&self, quantity: SIQuantity<HEAT_FLUX_DENSITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum HeatFluxDensityUnit {
    #[default]
    #[value(WATT_PER_SQUARE_METRE)]
    WattPerSquareMetre,
}

/// The units of `HeatTransfer`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<HEAT_TRANSFER, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<HEAT_TRANSFER, N>;
        fn value(&self, quantity: SIQuantity<HEAT_TRANSFER, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum HeatTransferUnit {
    #[default]
    #[value(KILOGRAM_PER_SECOND_CUBED_KELVIN)]
    KilogramPerSecondCubedKelvin,
    #[value(GRAM_PER_SECOND_CUBED_KELVIN)]
    GramPerSecondCubedKelvin,
    #[value(KILOGRAM_PER_SECOND_CUBED_DEGREE_CELSIUS)]
    KilogramPerSecondCubedDegreeCelsius,
    #[value(WATT_PER_SQUARE_METRE_KELVIN)]
    WattPerSquareMetreKelvin,
    #[value(WATT_PER_SQUARE_METRE_DEGREE_CELSIUS)]
    WattPerSquareMetreDegreeCelsius,
    #[value(JOULE_PER_SECOND_SQUARE_METRE_KELVIN)]
    JoulePerSecondSquareMetreKelvin,
    #[value(JOULE_PER_SECOND_SQUARE_METRE_DEGREE_CELSIUS)]
    JoulePerSecondSquareMetreDegreeCelsius,
    #[value(BTU_IT_PER_HOUR_SQUARE_FOOT_DEGREE_FAHRENHEIT)]
    BtuItPerHourSquareFootDegreeFahrenheit,
    #[value(BTU_IT_PER_HOUR_SQUARE_INCH_DEGREE_FAHRENHEIT)]
    BtuItPerHourSquareInchDegreeFahrenheit,
    #[value(BTU_IT_PER_MINUTE_SQUARE_FOOT_DEGREE_FAHRENHEIT)]
    BtuItPerMinuteSquareFootDegreeFahrenheit,
    #[value(BTU_IT_PER_MINUTE_SQUARE_INCH_DEGREE_FAHRENHEIT)]
    BtuItPerMinuteSquareInchDegreeFahrenheit,
    #[value(BTU_IT_PER_SECOND_SQUARE_FOOT_DEGREE_FAHRENHEIT)]
    BtuItPerSecondSquareFootDegreeFahrenheit,
    #[value(BTU_IT_PER_SECOND_SQUARE_INCH_DEGREE_FAHRENHEIT)]
    BtuItPerSecondSquareInchDegreeFahrenheit,
    #[value(BTU_PER_HOUR_SQUARE_FOOT_DEGREE_FAHRENHEIT)]
    BtuPerHourSquareFootDegreeFahrenheit,
    #[value(BTU_PER_HOUR_SQUARE_INCH_DEGREE_FAHRENHEIT)]
    BtuPerHourSquareInchDegreeFahrenheit,
    #[value(BTU_PER_MINUTE_SQUARE_FOOT_DEGREE_FAHRENHEIT)]
    BtuPerMinuteSquareFootDegreeFahrenheit,
    #[value(BTU_PER_MINUTE_SQUARE_INCH_DEGREE_FAHRENHEIT)]
    BtuPerMinuteSquareInchDegreeFahrenheit,
    #[value(BTU_PER_SECOND_SQUARE_FOOT_DEGREE_FAHRENHEIT)]
    BtuPerSecondSquareFootDegreeFahrenheit,
    #[value(BTU_PER_SECOND_SQUARE_INCH_DEGREE_FAHRENHEIT)]
    BtuPerSecondSquareInchDegreeFahrenheit,
}

/// The units of `ThermalConductance`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<THERMAL_CONDUCTANCE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<THERMAL_CONDUCTANCE, N>;
        fn value(&self, quantity: SIQuantity<THERMAL_CONDUCTANCE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ThermalConductanceUnit {
    #[default]
    #[value(KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN)]
    KilogramMetreSquaredPerSecondCubedKelvin,
    #[value(GRAM_METRE_SQUARED_PER_SECOND_CUBED_KELVIN)]
    GramMetreSquaredPerSecondCubedKelvin,
    #[value(WATT_PER_KELVIN)]
    WattPerKelvin,
    #[value(KILOGRAM_METRE_SQUARED_PER_SECOND_CUBED_DEGREE_CELSIUS)]
    KilogramMetreSquaredPerSecondCubedDegreeCelsius,
    #[value(THERMAL_CONDUCTANCE_WATT_PER_METRE_DEGREE_CELSIUS)]
    WattPerMetreDegreeCelsius,
    #[value(KILOWATT_PER_DEGREE_CELSIUS)]
    KilowattPerDegreeCelsius,
    #[value(MILLIWATT_PER_DEGREE_CELSIUS)]
    MilliwattPerDegreeCelsius,
}

/// The units of `ThermalConductivity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<THERMAL_CONDUCTIVITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<THERMAL_CONDUCTIVITY, N>;
        fn value(&self, quantity: SIQuantity<THERMAL_CONDUCTIVITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ThermalConductivityUnit {
    #[default]
    #[value(KILOGRAM_METRE_PER_SECOND_CUBED_KELVIN)]
    KilogramMetrePerSecondCubedKelvin,
    #[value(GRAM_METRE_PER_SECOND_CUBED_KELVIN)]
    GramMetrePerSecondCubedKelvin,
    #[value(WATT_PER_METRE_KELVIN)]
    WattPerMetreKelvin,
    #[value(KILOGRAM_METRE_PER_SECOND_CUBED_DEGREE_CELSIUS)]
    KilogramMetrePerSecondCubedDegreeCelsius,
    #[value(THERMAL_CONDUCTIVITY_WATT_PER_METRE_DEGREE_CELSIUS)]
    WattPerMetreDegreeCelsius,
}

/// The units of `ThermalResistance`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<THERMAL_RESISTANCE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<THERMAL_RESISTANCE, N>;
        fn value(&self, quantity: SIQuantity<THERMAL_RESISTANCE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ThermalResistanceUnit {
    #[default]
    #[value(KELVIN_PER_WATT)]
    KelvinPerWatt,
    #[value(KELVIN_PER_KILOWATT)]
    KelvinPerKilowatt,
}

/// The units of `SpecificHeatCapacity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<SPECIFIC_HEAT_CAPACITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<SPECIFIC_HEAT_CAPACITY, N>;
        fn value(&self, quantity: SIQuantity<SPECIFIC_HEAT_CAPACITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum SpecificHeatCapacityUnit {
    #[default]
    #[value(SQUARE_METRE_PER_SECOND_SQUARED_KELVIN)]
    SquareMetrePerSecondSquaredKelvin,
    #[value(JOULE_PER_KILOGRAM_KELVIN)]
    JoulePerKilogramKelvin,
    #[value(JOULE_PER_KILOGRAM_DEGREE_CELSIUS)]
    JoulePerKilogramDegreeCelsius,
    #[value(JOULE_PER_GRAM_DEGREE_CELSIUS)]
    JoulePerGramDegreeCelsius,
    #[value(BTU_PER_OUNCE_DEGREE_FAHRENHEIT)]
    BtuPerOunceDegreeFahrenheit,
    #[value(BTU_IT_PER_OUNCE_DEGREE_FAHRENHEIT)]
    BtuItPerOunceDegreeFahrenheit,
    #[value(BTU_PER_POUND_DEGREE_FAHRENHEIT)]
    BtuPerPoundDegreeFahrenheit,
    #[value(BTU_IT_PER_POUND_DEGREE_FAHRENHEIT)]
    BtuItPerPoundDegreeFahrenheit,
    #[value(BTU_PER_TON_DEGREE_FAHRENHEIT)]
    BtuPerTonDegreeFahrenheit,
    #[value(BTU_IT_PER_TON_DEGREE_FAHRENHEIT)]
    BtuItPerTonDegreeFahrenheit,
    #[value(CALORIE_PER_GRAM_KELVIN)]
    CaloriePerGramKelvin,
}

/// The units of `ArealHeatCapacity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<AREAL_HEAT_CAPACITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<AREAL_HEAT_CAPACITY, N>;
        fn value(&self, quantity: SIQuantity<AREAL_HEAT_CAPACITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ArealHeatCapacityUnit {
    #[default]
    #[value(JOULE_PER_SQUARE_METRE_KELVIN)]
    JoulePerSquareMetreKelvin,
}

/// The units of `VolumetricHeatCapacity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<VOLUMETRIC_HEAT_CAPACITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<VOLUMETRIC_HEAT_CAPACITY, N>;
        fn value(&self, quantity: SIQuantity<VOLUMETRIC_HEAT_CAPACITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum VolumetricHeatCapacityUnit {
    #[default]
    #[value(JOULE_PER_CUBIC_METRE_KELVIN)]
    JoulePerCubicMetreKelvin,
    #[value(CALORIE_PER_CUBIC_METRE_KELVIN)]
    CaloriePerCubicMetreKelvin,
}

/// The units of `StefanBoltzmannCoefficient`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<STEFAN_BOLTZMANN_COEFFICIENT, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<STEFAN_BOLTZMANN_COEFFICIENT, N>;
        fn value(&self, quantity: SIQuantity<STEFAN_BOLTZMANN_COEFFICIENT, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum StefanBoltzmannCoefficientUnit {
    #[default]
    #[value(WATT_PER_SQUARE_METRE_QUARTIC_KELVIN)]
    WattPerSquareMetreQuarticKelvin,
    #[value(STEFAN_BOLTZMANN_CONSTANT)]
    StefanBoltzmannConstant,
}
