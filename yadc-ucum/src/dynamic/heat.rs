//! The units of `heat`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

#[allow(unused_imports)]
use crate::heat::*;
#[allow(unused_imports)]
use crate::*;

/// The units of `Energy`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum EnergyUnit {
    #[default]
    #[value(CALORIE_AT15_DEGREE_C.unit())]
    CalorieAt15DegreeC,
    #[value(CALORIE_AT20_DEGREE_C.unit())]
    CalorieAt20DegreeC,
    #[value(MEAN_CALORIE.unit())]
    MeanCalorie,
    #[value(INTERNATIONAL_TABLE_CALORIE.unit())]
    InternationalTableCalorie,
    #[value(THERMOCHEMICAL_CALORIE.unit())]
    ThermochemicalCalorie,
    #[value(CALORIE.unit())]
    Calorie,
    #[value(NUTRITION_LABEL_CALORIES.unit())]
    NutritionLabelCalories,
    #[value(BRITISH_THERMAL_UNIT_AT39_DEGREE_F.unit())]
    BritishThermalUnitAt39DegreeF,
    #[value(BRITISH_THERMAL_UNIT_AT59_DEGREE_F.unit())]
    BritishThermalUnitAt59DegreeF,
    #[value(BRITISH_THERMAL_UNIT_AT60_DEGREE_F.unit())]
    BritishThermalUnitAt60DegreeF,
    #[value(MEAN_BRITISH_THERMAL_UNIT.unit())]
    MeanBritishThermalUnit,
    #[value(INTERNATIONAL_TABLE_BRITISH_THERMAL_UNIT.unit())]
    InternationalTableBritishThermalUnit,
    #[value(THERMOCHEMICAL_BRITISH_THERMAL_UNIT.unit())]
    ThermochemicalBritishThermalUnit,
    #[value(BRITISH_THERMAL_UNIT.unit())]
    BritishThermalUnit,
}

/// The units of `LinearMassDensityOfTextileThread`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum LinearMassDensityOfTextileThreadUnit {
    #[default]
    #[value(TEX.unit())]
    Tex,
    #[value(DENIER.unit())]
    Denier,
}

/// The units of `Power`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum PowerUnit {
    #[default]
    #[value(HORSEPOWER.unit())]
    Horsepower,
}

/// The units of `TemperatureInterval`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum TemperatureIntervalUnit {
    #[default]
    #[value(KELVIN.unit())]
    Kelvin,
    #[value(DEGREE_FAHRENHEIT.unit())]
    DegreeFahrenheit,
}

/// The units of `ThermodynamicTemperature`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum ThermodynamicTemperatureUnit {
    #[default]
    #[value(KELVIN.unit())]
    Kelvin,
    #[value(CELSIUS.unit())]
    DegreeCelsius,
    #[value(FAHRENHEIT.unit())]
    DegreeFahrenheit,
    #[value(RANKINE.unit())]
    DegreeRankine,
}
