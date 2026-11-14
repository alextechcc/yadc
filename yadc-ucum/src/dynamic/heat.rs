//! The units of `heat`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::dynamic::DynamicUcumUnit;
use crate::heat::units::*;
use crate::heat::*;
use core::marker::Destruct;
use core::ops::{Add, Div, Mul, Sub};

/// The units of `Energy`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Energy<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum EnergyUnit {
    #[default]
    #[unit(CalorieAt15DegreeC)]
    CalorieAt15DegreeC,
    #[unit(CalorieAt20DegreeC)]
    CalorieAt20DegreeC,
    #[unit(MeanCalorie)]
    MeanCalorie,
    #[unit(InternationalTableCalorie)]
    InternationalTableCalorie,
    #[unit(ThermochemicalCalorie)]
    ThermochemicalCalorie,
    #[unit(Calorie)]
    Calorie,
    #[unit(NutritionLabelCalories)]
    NutritionLabelCalories,
    #[unit(BritishThermalUnitAt39DegreeF)]
    BritishThermalUnitAt39DegreeF,
    #[unit(BritishThermalUnitAt59DegreeF)]
    BritishThermalUnitAt59DegreeF,
    #[unit(BritishThermalUnitAt60DegreeF)]
    BritishThermalUnitAt60DegreeF,
    #[unit(MeanBritishThermalUnit)]
    MeanBritishThermalUnit,
    #[unit(InternationalTableBritishThermalUnit)]
    InternationalTableBritishThermalUnit,
    #[unit(ThermochemicalBritishThermalUnit)]
    ThermochemicalBritishThermalUnit,
    #[unit(BritishThermalUnit)]
    BritishThermalUnit,
}

impl From<EnergyUnit> for DynamicUcumUnit {
    fn from(unit: EnergyUnit) -> Self {
        match unit {
            EnergyUnit::CalorieAt15DegreeC => CalorieAt15DegreeC {}.into(),
            EnergyUnit::CalorieAt20DegreeC => CalorieAt20DegreeC {}.into(),
            EnergyUnit::MeanCalorie => MeanCalorie {}.into(),
            EnergyUnit::InternationalTableCalorie => InternationalTableCalorie {}.into(),
            EnergyUnit::ThermochemicalCalorie => ThermochemicalCalorie {}.into(),
            EnergyUnit::Calorie => Calorie {}.into(),
            EnergyUnit::NutritionLabelCalories => NutritionLabelCalories {}.into(),
            EnergyUnit::BritishThermalUnitAt39DegreeF => BritishThermalUnitAt39DegreeF {}.into(),
            EnergyUnit::BritishThermalUnitAt59DegreeF => BritishThermalUnitAt59DegreeF {}.into(),
            EnergyUnit::BritishThermalUnitAt60DegreeF => BritishThermalUnitAt60DegreeF {}.into(),
            EnergyUnit::MeanBritishThermalUnit => MeanBritishThermalUnit {}.into(),
            EnergyUnit::InternationalTableBritishThermalUnit => {
                InternationalTableBritishThermalUnit {}.into()
            }
            EnergyUnit::ThermochemicalBritishThermalUnit => {
                ThermochemicalBritishThermalUnit {}.into()
            }
            EnergyUnit::BritishThermalUnit => BritishThermalUnit {}.into(),
        }
    }
}

/// The units of `LinearMassDensityOfTextileThread`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LinearMassDensityOfTextileThread<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum LinearMassDensityOfTextileThreadUnit {
    #[default]
    #[unit(Tex)]
    Tex,
    #[unit(Denier)]
    Denier,
}

impl From<LinearMassDensityOfTextileThreadUnit> for DynamicUcumUnit {
    fn from(unit: LinearMassDensityOfTextileThreadUnit) -> Self {
        match unit {
            LinearMassDensityOfTextileThreadUnit::Tex => Tex {}.into(),
            LinearMassDensityOfTextileThreadUnit::Denier => Denier {}.into(),
        }
    }
}

/// The units of `Power`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Power<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum PowerUnit {
    #[default]
    #[unit(Horsepower)]
    Horsepower,
}

impl From<PowerUnit> for DynamicUcumUnit {
    fn from(unit: PowerUnit) -> Self {
        match unit {
            PowerUnit::Horsepower => Horsepower {}.into(),
        }
    }
}

/// The units of `TemperatureInterval`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = TemperatureInterval<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum TemperatureIntervalUnit {
    #[default]
    #[unit(Kelvin)]
    Kelvin,
    #[unit(DegreeFahrenheit)]
    DegreeFahrenheit,
}

impl From<TemperatureIntervalUnit> for DynamicUcumUnit {
    fn from(unit: TemperatureIntervalUnit) -> Self {
        match unit {
            TemperatureIntervalUnit::Kelvin => Kelvin {}.into(),
            TemperatureIntervalUnit::DegreeFahrenheit => DegreeFahrenheit {}.into(),
        }
    }
}

/// The units of `ThermodynamicTemperature`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ThermodynamicTemperature<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ThermodynamicTemperatureUnit {
    #[default]
    #[unit(KelvinScale)]
    Kelvin,
    #[unit(Celsius)]
    DegreeCelsius,
    #[unit(Fahrenheit)]
    DegreeFahrenheit,
    #[unit(Rankine)]
    DegreeRankine,
}

impl From<ThermodynamicTemperatureUnit> for DynamicUcumUnit {
    fn from(unit: ThermodynamicTemperatureUnit) -> Self {
        match unit {
            ThermodynamicTemperatureUnit::Kelvin => KelvinScale {}.into(),
            ThermodynamicTemperatureUnit::DegreeCelsius => Celsius {}.into(),
            ThermodynamicTemperatureUnit::DegreeFahrenheit => Fahrenheit {}.into(),
            ThermodynamicTemperatureUnit::DegreeRankine => Rankine {}.into(),
        }
    }
}
