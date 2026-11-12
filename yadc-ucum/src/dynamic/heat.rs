//! The units of `heat`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::heat::consts::*;
use crate::heat::*;
use crate::{Exact, Scale};
use core::marker::Destruct;
use yadc::{Unit, num_traits::Num};

/// The units of `Energy`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Energy<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Energy<N>: [const] Unit<Energy<N>, N> + [const] Destruct)]
pub enum EnergyUnit {
    #[default]
    #[unit("cal_[15]", "calorie at 15 °C", by = Energy::<N>::from(CALORIE_AT15_DEGREE_C))]
    CalorieAt15DegreeC,
    #[unit("cal_[20]", "calorie at 20 °C", by = Energy::<N>::from(CALORIE_AT20_DEGREE_C))]
    CalorieAt20DegreeC,
    #[unit("cal_m", "mean calorie", by = Energy::<N>::from(MEAN_CALORIE))]
    MeanCalorie,
    #[unit("cal_IT", "international table calorie", by = Energy::<N>::from(INTERNATIONAL_TABLE_CALORIE))]
    InternationalTableCalorie,
    #[unit("cal_th", "thermochemical calorie", by = Energy::<N>::from(THERMOCHEMICAL_CALORIE))]
    ThermochemicalCalorie,
    #[unit("cal", "calorie", by = Energy::<N>::from(CALORIE))]
    Calorie,
    #[unit("[Cal]", "nutrition label Calories", by = Energy::<N>::from(NUTRITION_LABEL_CALORIES))]
    NutritionLabelCalories,
    #[unit("[Btu_39]", "British thermal unit at 39 °F", by = Energy::<N>::from(BRITISH_THERMAL_UNIT_AT39_DEGREE_F))]
    BritishThermalUnitAt39DegreeF,
    #[unit("[Btu_59]", "British thermal unit at 59 °F", by = Energy::<N>::from(BRITISH_THERMAL_UNIT_AT59_DEGREE_F))]
    BritishThermalUnitAt59DegreeF,
    #[unit("[Btu_60]", "British thermal unit at 60 °F", by = Energy::<N>::from(BRITISH_THERMAL_UNIT_AT60_DEGREE_F))]
    BritishThermalUnitAt60DegreeF,
    #[unit("[Btu_m]", "mean British thermal unit", by = Energy::<N>::from(MEAN_BRITISH_THERMAL_UNIT))]
    MeanBritishThermalUnit,
    #[unit("[Btu_IT]", "international table British thermal unit", by = Energy::<N>::from(INTERNATIONAL_TABLE_BRITISH_THERMAL_UNIT))]
    InternationalTableBritishThermalUnit,
    #[unit("[Btu_th]", "thermochemical British thermal unit", by = Energy::<N>::from(THERMOCHEMICAL_BRITISH_THERMAL_UNIT))]
    ThermochemicalBritishThermalUnit,
    #[unit("[Btu]", "British thermal unit", by = Energy::<N>::from(BRITISH_THERMAL_UNIT))]
    BritishThermalUnit,
}

/// The units of `LinearMassDensityOfTextileThread`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LinearMassDensityOfTextileThread<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where LinearMassDensityOfTextileThread<N>: [const] Unit<LinearMassDensityOfTextileThread<N>, N> + [const] Destruct)]
pub enum LinearMassDensityOfTextileThreadUnit {
    #[default]
    #[unit("tex", "tex", by = LinearMassDensityOfTextileThread::<N>::from(TEX))]
    Tex,
    #[unit("[den]", "Denier", by = LinearMassDensityOfTextileThread::<N>::from(DENIER))]
    Denier,
}

/// The units of `Power`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Power<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Power<N>: [const] Unit<Power<N>, N> + [const] Destruct)]
pub enum PowerUnit {
    #[default]
    #[unit("[HP]", "horsepower", by = Power::<N>::from(HORSEPOWER))]
    Horsepower,
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
    #[unit("[degF]", "degrees Fahrenheit", by = TemperatureInterval::<N>::from(DEGREE_FAHRENHEIT))]
    DegreeFahrenheit,
}

/// The units of `ThermodynamicTemperature`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ThermodynamicTemperature<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Scale<ThermodynamicTemperature<N>, TemperatureInterval<N>>: [const] Unit<ThermodynamicTemperature<N>, N> + [const] Destruct)]
pub enum ThermodynamicTemperatureUnit {
    #[default]
    #[unit("K", "kelvin", by = Scale::<ThermodynamicTemperature<N>, TemperatureInterval<N>>::from(KELVIN_SCALE))]
    Kelvin,
    #[unit("Cel", "degrees Celsius", by = Scale::<ThermodynamicTemperature<N>, TemperatureInterval<N>>::from(CELSIUS))]
    DegreeCelsius,
    #[unit("[degF]", "degrees Fahrenheit", by = Scale::<ThermodynamicTemperature<N>, TemperatureInterval<N>>::from(FAHRENHEIT))]
    DegreeFahrenheit,
    #[unit("[degR]", "degrees Rankine", by = Scale::<ThermodynamicTemperature<N>, TemperatureInterval<N>>::from(RANKINE))]
    DegreeRankine,
}
