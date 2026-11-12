use crate::{Dim, Exact, Kind, Qty, Scale};
use consts::*;
use yadc::Dimension;

pub type Energy<N> = Qty<
    {
        Dim {
            m: 2,
            s: -2,
            g: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type LinearMassDensityOfTextileThread<N> = Qty<
    {
        Dim {
            m: -1,
            g: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Power<N> = Qty<
    {
        Dim {
            m: 2,
            s: -3,
            g: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type TemperatureInterval<N> = Qty<
    {
        Dim {
            K: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type ThermodynamicTemperature<N> = Qty<
    {
        Dim {
            K: 1,
            kind: Kind::Absolute,
            ..Dim::DIMENSIONLESS
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
    use crate::base::{GRAM, METRE, SECOND};

    /// The kelvin as a step, UCUM's base unit of temperature: a difference of 1 K.
    pub use crate::base::KELVIN;

    /// Absolute zero, 0 K: the zero of thermodynamic temperature.
    pub use crate::base::ABSOLUTE_ZERO;

    /// One `CalorieAt15DegreeC` (`cal_[15]`), as an `Energy`.
    pub const CALORIE_AT15_DEGREE_C: Energy<Exact> =
        (METRE * METRE * GRAM / (SECOND * SECOND)) * Exact::dec(41858, -1);

    /// One `CalorieAt20DegreeC` (`cal_[20]`), as an `Energy`.
    pub const CALORIE_AT20_DEGREE_C: Energy<Exact> =
        (METRE * METRE * GRAM / (SECOND * SECOND)) * Exact::dec(41819, -1);

    /// One `MeanCalorie` (`cal_m`), as an `Energy`.
    pub const MEAN_CALORIE: Energy<Exact> =
        (METRE * METRE * GRAM / (SECOND * SECOND)) * Exact::dec(419002, -2);

    /// One `InternationalTableCalorie` (`cal_IT`), as an `Energy`.
    pub const INTERNATIONAL_TABLE_CALORIE: Energy<Exact> =
        (METRE * METRE * GRAM / (SECOND * SECOND)) * Exact::dec(41868, -1);

    /// One `ThermochemicalCalorie` (`cal_th`), as an `Energy`.
    pub const THERMOCHEMICAL_CALORIE: Energy<Exact> =
        (METRE * METRE * GRAM / (SECOND * SECOND)) * Exact::int(4184);

    /// One `Calorie` (`cal`), as an `Energy`.
    pub const CALORIE: Energy<Exact> =
        (METRE * METRE * GRAM / (SECOND * SECOND)) * Exact::int(4184);

    /// One `NutritionLabelCalories` (`[Cal]`), as an `Energy`.
    pub const NUTRITION_LABEL_CALORIES: Energy<Exact> =
        (METRE * METRE * GRAM / (SECOND * SECOND)) * Exact::int(4184000);

    /// One `BritishThermalUnitAt39DegreeF` (`[Btu_39]`), as an `Energy`.
    pub const BRITISH_THERMAL_UNIT_AT39_DEGREE_F: Energy<Exact> =
        (METRE * METRE * GRAM / (SECOND * SECOND)) * Exact::int(1059670);

    /// One `BritishThermalUnitAt59DegreeF` (`[Btu_59]`), as an `Energy`.
    pub const BRITISH_THERMAL_UNIT_AT59_DEGREE_F: Energy<Exact> =
        (METRE * METRE * GRAM / (SECOND * SECOND)) * Exact::int(1054800);

    /// One `BritishThermalUnitAt60DegreeF` (`[Btu_60]`), as an `Energy`.
    pub const BRITISH_THERMAL_UNIT_AT60_DEGREE_F: Energy<Exact> =
        (METRE * METRE * GRAM / (SECOND * SECOND)) * Exact::int(1054680);

    /// One `MeanBritishThermalUnit` (`[Btu_m]`), as an `Energy`.
    pub const MEAN_BRITISH_THERMAL_UNIT: Energy<Exact> =
        (METRE * METRE * GRAM / (SECOND * SECOND)) * Exact::int(1055870);

    /// One `InternationalTableBritishThermalUnit` (`[Btu_IT]`), as an `Energy`.
    pub const INTERNATIONAL_TABLE_BRITISH_THERMAL_UNIT: Energy<Exact> =
        (METRE * METRE * GRAM / (SECOND * SECOND)) * Exact::dec(105505585262, -5);

    /// One `ThermochemicalBritishThermalUnit` (`[Btu_th]`), as an `Energy`.
    pub const THERMOCHEMICAL_BRITISH_THERMAL_UNIT: Energy<Exact> =
        (METRE * METRE * GRAM / (SECOND * SECOND)) * Exact::int(1054350);

    /// One `BritishThermalUnit` (`[Btu]`), as an `Energy`.
    pub const BRITISH_THERMAL_UNIT: Energy<Exact> =
        (METRE * METRE * GRAM / (SECOND * SECOND)) * Exact::int(1054350);

    /// One `Tex` (`tex`), as a `LinearMassDensityOfTextileThread`.
    pub const TEX: LinearMassDensityOfTextileThread<Exact> = (GRAM / METRE) * Exact::dec(1, -3);

    /// One `Denier` (`[den]`), as a `LinearMassDensityOfTextileThread`.
    pub const DENIER: LinearMassDensityOfTextileThread<Exact> =
        (GRAM / METRE) * (Exact::int(1) / Exact::int(9000));

    /// One `Horsepower` (`[HP]`), as a `Power`.
    pub const HORSEPOWER: Power<Exact> =
        (METRE * METRE * GRAM / (SECOND * SECOND * SECOND)) * Exact::dec(74569987158227022, -11);

    /// One `DegreeFahrenheit` (`[degF]`), as a `TemperatureInterval`.
    pub const DEGREE_FAHRENHEIT: TemperatureInterval<Exact> =
        KELVIN * (Exact::int(5) / Exact::int(9));

    /// 0 °C, the ice point of water: 273.15 K.
    pub const ICE_POINT: ThermodynamicTemperature<Exact> =
        ABSOLUTE_ZERO + KELVIN * Exact::dec(27315, -2);

    /// 0 °F: 32 °F below the ice point.
    pub const FAHRENHEIT_ZERO: ThermodynamicTemperature<Exact> =
        ICE_POINT - DEGREE_FAHRENHEIT * Exact::int(32);

    /// The kelvin scale: KELVIN counted from ABSOLUTE_ZERO.
    pub const KELVIN_SCALE: Scale<ThermodynamicTemperature<Exact>, TemperatureInterval<Exact>> =
        Scale(ABSOLUTE_ZERO, KELVIN);

    /// The degrees Celsius scale: KELVIN counted from ICE_POINT.
    pub const CELSIUS: Scale<ThermodynamicTemperature<Exact>, TemperatureInterval<Exact>> =
        Scale(ICE_POINT, KELVIN);

    /// The degrees Fahrenheit scale: DEGREE_FAHRENHEIT counted from FAHRENHEIT_ZERO.
    pub const FAHRENHEIT: Scale<ThermodynamicTemperature<Exact>, TemperatureInterval<Exact>> =
        Scale(FAHRENHEIT_ZERO, DEGREE_FAHRENHEIT);

    /// The degrees Rankine scale: DEGREE_FAHRENHEIT counted from ABSOLUTE_ZERO.
    pub const RANKINE: Scale<ThermodynamicTemperature<Exact>, TemperatureInterval<Exact>> =
        Scale(ABSOLUTE_ZERO, DEGREE_FAHRENHEIT);
}

/// The units of `Energy`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Energy<N>, number = N)]
#[unit(for<N> where Energy<N>: [const] yadc::Unit<Energy<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
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

/// The units of `LinearMassDensityOfTextileThread`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LinearMassDensityOfTextileThread<N>, number = N)]
#[unit(for<N> where LinearMassDensityOfTextileThread<N>: [const] yadc::Unit<LinearMassDensityOfTextileThread<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum LinearMassDensityOfTextileThreadUnit {
    #[default]
    #[unit("tex", "tex", by = LinearMassDensityOfTextileThread::<N>::from(TEX))]
    Tex,
    #[unit("[den]", "Denier", by = LinearMassDensityOfTextileThread::<N>::from(DENIER))]
    Denier,
}

/// The units of `Power`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Power<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Power<N>: [const] yadc::Unit<Power<N>, N> + [const] core::marker::Destruct)]
pub enum PowerUnit {
    #[default]
    #[unit("[HP]", "horsepower", by = Power::<N>::from(HORSEPOWER))]
    Horsepower,
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
    #[unit("[degF]", "degrees Fahrenheit", by = TemperatureInterval::<N>::from(DEGREE_FAHRENHEIT))]
    DegreeFahrenheit,
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
    #[unit("Cel", "degrees Celsius", by = Scale::<ThermodynamicTemperature<N>, TemperatureInterval<N>>::from(CELSIUS))]
    DegreeCelsius,
    #[unit("[degF]", "degrees Fahrenheit", by = Scale::<ThermodynamicTemperature<N>, TemperatureInterval<N>>::from(FAHRENHEIT))]
    DegreeFahrenheit,
    #[unit("[degR]", "degrees Rankine", by = Scale::<ThermodynamicTemperature<N>, TemperatureInterval<N>>::from(RANKINE))]
    DegreeRankine,
}
