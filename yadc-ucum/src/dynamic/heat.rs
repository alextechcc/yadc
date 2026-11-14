//! The units of `heat`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::dynamic::DynamicUcumUnit;
use crate::heat::units::*;
use crate::heat::*;
use crate::{Exact, Qty, Unit};
use core::ops::{Add, Div, Mul, Sub};

/// The units of `Energy`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<ENERGY, N>, N> {
        fn quantity(&self, value: N) -> Qty<ENERGY, N>;
        fn value(&self, quantity: Qty<ENERGY, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum EnergyUnit {
    #[default]
    #[value(CALORIE_AT15_DEGREE_C)]
    CalorieAt15DegreeC,
    #[value(CALORIE_AT20_DEGREE_C)]
    CalorieAt20DegreeC,
    #[value(MEAN_CALORIE)]
    MeanCalorie,
    #[value(INTERNATIONAL_TABLE_CALORIE)]
    InternationalTableCalorie,
    #[value(THERMOCHEMICAL_CALORIE)]
    ThermochemicalCalorie,
    #[value(CALORIE)]
    Calorie,
    #[value(NUTRITION_LABEL_CALORIES)]
    NutritionLabelCalories,
    #[value(BRITISH_THERMAL_UNIT_AT39_DEGREE_F)]
    BritishThermalUnitAt39DegreeF,
    #[value(BRITISH_THERMAL_UNIT_AT59_DEGREE_F)]
    BritishThermalUnitAt59DegreeF,
    #[value(BRITISH_THERMAL_UNIT_AT60_DEGREE_F)]
    BritishThermalUnitAt60DegreeF,
    #[value(MEAN_BRITISH_THERMAL_UNIT)]
    MeanBritishThermalUnit,
    #[value(INTERNATIONAL_TABLE_BRITISH_THERMAL_UNIT)]
    InternationalTableBritishThermalUnit,
    #[value(THERMOCHEMICAL_BRITISH_THERMAL_UNIT)]
    ThermochemicalBritishThermalUnit,
    #[value(BRITISH_THERMAL_UNIT)]
    BritishThermalUnit,
}

/// The units of `LinearMassDensityOfTextileThread`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<LINEAR_MASS_DENSITY_OF_TEXTILE_THREAD, N>, N> {
        fn quantity(&self, value: N) -> Qty<LINEAR_MASS_DENSITY_OF_TEXTILE_THREAD, N>;
        fn value(&self, quantity: Qty<LINEAR_MASS_DENSITY_OF_TEXTILE_THREAD, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum LinearMassDensityOfTextileThreadUnit {
    #[default]
    #[value(TEX)]
    Tex,
    #[value(DENIER)]
    Denier,
}

/// The units of `Power`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<POWER, N>, N> {
        fn quantity(&self, value: N) -> Qty<POWER, N>;
        fn value(&self, quantity: Qty<POWER, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum PowerUnit {
    #[default]
    #[value(HORSEPOWER)]
    Horsepower,
}

/// The units of `TemperatureInterval`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<TEMPERATURE_INTERVAL, N>, N> {
        fn quantity(&self, value: N) -> Qty<TEMPERATURE_INTERVAL, N>;
        fn value(&self, quantity: Qty<TEMPERATURE_INTERVAL, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum TemperatureIntervalUnit {
    #[default]
    #[value(KELVIN)]
    Kelvin,
    #[value(DEGREE_FAHRENHEIT)]
    DegreeFahrenheit,
}

/// The units of `ThermodynamicTemperature`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<THERMODYNAMIC_TEMPERATURE, N>, N> {
        fn quantity(&self, value: N) -> Qty<THERMODYNAMIC_TEMPERATURE, N>;
        fn value(&self, quantity: Qty<THERMODYNAMIC_TEMPERATURE, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ThermodynamicTemperatureUnit {
    #[default]
    #[value(KELVIN_SCALE)]
    Kelvin,
    #[value(CELSIUS)]
    DegreeCelsius,
    #[value(FAHRENHEIT)]
    DegreeFahrenheit,
    #[value(RANKINE)]
    DegreeRankine,
}
