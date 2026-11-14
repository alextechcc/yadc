//! The units of `thermal`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

#[allow(unused_imports)]
use crate::thermal::*;
#[allow(unused_imports)]
use crate::*;

/// The units of `ThermodynamicTemperature`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum ThermodynamicTemperatureUnit {
    #[default]
    #[value(KELVIN.unit())]
    Kelvin,
    #[value(CELSIUS.unit())]
    Celsius,
    #[value(FAHRENHEIT.unit())]
    Fahrenheit,
    #[value(RANKINE.unit())]
    Rankine,
}

/// The units of `TemperatureInterval`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum TemperatureIntervalUnit {
    #[default]
    #[value(KELVIN.unit())]
    Kelvin,
    #[value(DEGREE_CELSIUS.unit())]
    Celsius,
    #[value(DEGREE_FAHRENHEIT.unit())]
    Fahrenheit,
    #[value(DEGREE_RANKINE.unit())]
    Rankine,
}

/// The units of `HeatCapacity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum HeatCapacityUnit {
    #[default]
    #[value(SIUnit::ONE.times(METRE.unit(), 2).times(KILOGRAM.unit(), 1).times(SECOND.unit(), -2).times(KELVIN.unit(), -1))]
    MetreSquaredKilogramPerSecondSquaredKelvin,
    #[value(BOLTZMANN_CONSTANT.unit())]
    BoltzmannConstant,
}

/// The units of `StefanBoltzmannCoefficient`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum StefanBoltzmannCoefficientUnit {
    #[default]
    #[value(KILOGRAM.unit().times(SECOND.unit(), -3).times(KELVIN.unit(), -4))]
    KilogramPerSecondCubedKelvinPow4,
    #[value(STEFAN_BOLTZMANN_CONSTANT.unit())]
    StefanBoltzmannConstant,
}
