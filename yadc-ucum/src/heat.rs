use crate::{Dim, Kind, Qty};
use yadc::Dimension;

/// The dimension of `Energy`.
pub const ENERGY: Dim = Dim {
    m: 2,
    s: -2,
    g: 1,
    ..Dim::DIMENSIONLESS
};
pub type Energy<N> = Qty<ENERGY, N>;

/// The dimension of `LinearMassDensityOfTextileThread`.
pub const LINEAR_MASS_DENSITY_OF_TEXTILE_THREAD: Dim = Dim {
    m: -1,
    g: 1,
    ..Dim::DIMENSIONLESS
};
pub type LinearMassDensityOfTextileThread<N> = Qty<LINEAR_MASS_DENSITY_OF_TEXTILE_THREAD, N>;

/// The dimension of `Power`.
pub const POWER: Dim = Dim {
    m: 2,
    s: -3,
    g: 1,
    ..Dim::DIMENSIONLESS
};
pub type Power<N> = Qty<POWER, N>;

/// The dimension of `TemperatureInterval`.
pub const TEMPERATURE_INTERVAL: Dim = Dim {
    K: 1,
    ..Dim::DIMENSIONLESS
};
pub type TemperatureInterval<N> = Qty<TEMPERATURE_INTERVAL, N>;

/// The dimension of `ThermodynamicTemperature`.
pub const THERMODYNAMIC_TEMPERATURE: Dim = Dim {
    K: 1,
    kind: Kind::Absolute,
    ..Dim::DIMENSIONLESS
};
pub type ThermodynamicTemperature<N> = Qty<THERMODYNAMIC_TEMPERATURE, N>;

/// The units of this module's quantities: a type each, its size and zero
/// in its quantity's coherent unit, exact (`Mile {}.quantity(3.0_f64)`).
pub mod units {
    use super::*;
    use crate::{Exact, UcumUnit, UcumUnitConsts};

    /// Calorie at 15 °C (`cal_[15]`), a unit of `Energy`: `n` of them is `n · 41858e-1` coherent units.
    pub type CalorieAt15DegreeC = UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::dec(41858, -1),
                zero: Exact::ZERO,
                symbol: "cal_[15]",
                name: "calorie at 15 °C",
            }
        },
    >;

    /// Calorie at 20 °C (`cal_[20]`), a unit of `Energy`: `n` of them is `n · 41819e-1` coherent units.
    pub type CalorieAt20DegreeC = UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::dec(41819, -1),
                zero: Exact::ZERO,
                symbol: "cal_[20]",
                name: "calorie at 20 °C",
            }
        },
    >;

    /// Mean calorie (`cal_m`), a unit of `Energy`: `n` of them is `n · 419002e-2` coherent units.
    pub type MeanCalorie = UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::dec(419002, -2),
                zero: Exact::ZERO,
                symbol: "cal_m",
                name: "mean calorie",
            }
        },
    >;

    /// International table calorie (`cal_IT`), a unit of `Energy`: `n` of them is `n · 41868e-1` coherent units.
    pub type InternationalTableCalorie = UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::dec(41868, -1),
                zero: Exact::ZERO,
                symbol: "cal_IT",
                name: "international table calorie",
            }
        },
    >;

    /// Thermochemical calorie (`cal_th`), a unit of `Energy`: `n` of them is `n · 4184` coherent units.
    pub type ThermochemicalCalorie = UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::int(4184),
                zero: Exact::ZERO,
                symbol: "cal_th",
                name: "thermochemical calorie",
            }
        },
    >;

    /// Calorie (`cal`), a unit of `Energy`: `n` of them is `n · 4184` coherent units.
    pub type Calorie = UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::int(4184),
                zero: Exact::ZERO,
                symbol: "cal",
                name: "calorie",
            }
        },
    >;

    /// Nutrition label Calories (`[Cal]`), a unit of `Energy`: `n` of them is `n · 4184e3` coherent units.
    pub type NutritionLabelCalories = UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::int(4184000),
                zero: Exact::ZERO,
                symbol: "[Cal]",
                name: "nutrition label Calories",
            }
        },
    >;

    /// British thermal unit at 39 °F (`[Btu_39]`), a unit of `Energy`: `n` of them is `n · 105967e1` coherent units.
    pub type BritishThermalUnitAt39DegreeF = UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::int(1059670),
                zero: Exact::ZERO,
                symbol: "[Btu_39]",
                name: "British thermal unit at 39 °F",
            }
        },
    >;

    /// British thermal unit at 59 °F (`[Btu_59]`), a unit of `Energy`: `n` of them is `n · 10548e2` coherent units.
    pub type BritishThermalUnitAt59DegreeF = UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::int(1054800),
                zero: Exact::ZERO,
                symbol: "[Btu_59]",
                name: "British thermal unit at 59 °F",
            }
        },
    >;

    /// British thermal unit at 60 °F (`[Btu_60]`), a unit of `Energy`: `n` of them is `n · 105468e1` coherent units.
    pub type BritishThermalUnitAt60DegreeF = UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::int(1054680),
                zero: Exact::ZERO,
                symbol: "[Btu_60]",
                name: "British thermal unit at 60 °F",
            }
        },
    >;

    /// Mean British thermal unit (`[Btu_m]`), a unit of `Energy`: `n` of them is `n · 105587e1` coherent units.
    pub type MeanBritishThermalUnit = UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::int(1055870),
                zero: Exact::ZERO,
                symbol: "[Btu_m]",
                name: "mean British thermal unit",
            }
        },
    >;

    /// International table British thermal unit (`[Btu_IT]`), a unit of `Energy`: `n` of them is `n · 105505585262e-5` coherent units.
    pub type InternationalTableBritishThermalUnit = UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::dec(105505585262, -5),
                zero: Exact::ZERO,
                symbol: "[Btu_IT]",
                name: "international table British thermal unit",
            }
        },
    >;

    /// Thermochemical British thermal unit (`[Btu_th]`), a unit of `Energy`: `n` of them is `n · 105435e1` coherent units.
    pub type ThermochemicalBritishThermalUnit = UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::int(1054350),
                zero: Exact::ZERO,
                symbol: "[Btu_th]",
                name: "thermochemical British thermal unit",
            }
        },
    >;

    /// British thermal unit (`[Btu]`), a unit of `Energy`: `n` of them is `n · 105435e1` coherent units.
    pub type BritishThermalUnit = UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::int(1054350),
                zero: Exact::ZERO,
                symbol: "[Btu]",
                name: "British thermal unit",
            }
        },
    >;

    /// Tex (`tex`), a unit of `LinearMassDensityOfTextileThread`: `n` of them is `n · 1e-3` coherent units.
    pub type Tex = UcumUnit<
        LINEAR_MASS_DENSITY_OF_TEXTILE_THREAD,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "tex",
                name: "tex",
            }
        },
    >;

    /// Denier (`[den]`), a unit of `LinearMassDensityOfTextileThread`: `n` of them is `n · 1 / 9000` coherent units.
    pub type Denier = UcumUnit<
        LINEAR_MASS_DENSITY_OF_TEXTILE_THREAD,
        {
            UcumUnitConsts {
                size: Exact::int(1) / Exact::int(9000),
                zero: Exact::ZERO,
                symbol: "[den]",
                name: "Denier",
            }
        },
    >;

    /// Horsepower (`[HP]`), a unit of `Power`: `n` of them is `n · 74569987158227022e-11` coherent units.
    pub type Horsepower = UcumUnit<
        POWER,
        {
            UcumUnitConsts {
                size: Exact::dec(74569987158227022, -11),
                zero: Exact::ZERO,
                symbol: "[HP]",
                name: "horsepower",
            }
        },
    >;

    /// Kelvin (`K`), a unit of `TemperatureInterval`: `n` of them is `n · 1.0` coherent units.
    pub type Kelvin = UcumUnit<
        TEMPERATURE_INTERVAL,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "K",
                name: "kelvin",
            }
        },
    >;

    /// Degrees Fahrenheit (`[degF]`), a unit of `TemperatureInterval`: `n` of them is `n · 5.0 / 9.0` coherent units.
    pub type DegreeFahrenheit = UcumUnit<
        TEMPERATURE_INTERVAL,
        {
            UcumUnitConsts {
                size: Exact::int(5) / Exact::int(9),
                zero: Exact::ZERO,
                symbol: "[degF]",
                name: "degrees Fahrenheit",
            }
        },
    >;

    /// Kelvin (`K`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 1.0` coherent units.
    pub type KelvinScale = UcumUnit<
        THERMODYNAMIC_TEMPERATURE,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "K",
                name: "kelvin",
            }
        },
    >;

    /// Degrees Celsius (`Cel`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 1.0 + 273.15` coherent units.
    pub type Celsius = UcumUnit<
        THERMODYNAMIC_TEMPERATURE,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::dec(27315, -2),
                symbol: "Cel",
                name: "degrees Celsius",
            }
        },
    >;

    /// Degrees Fahrenheit (`[degF]`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 5.0 / 9.0 + 459.67 * 5.0 / 9.0` coherent units.
    pub type Fahrenheit = UcumUnit<
        THERMODYNAMIC_TEMPERATURE,
        {
            UcumUnitConsts {
                size: Exact::int(5) / Exact::int(9),
                zero: Exact::dec(45967, -2) * Exact::int(5) / Exact::int(9),
                symbol: "[degF]",
                name: "degrees Fahrenheit",
            }
        },
    >;

    /// Degrees Rankine (`[degR]`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 5.0 / 9.0` coherent units.
    pub type Rankine = UcumUnit<
        THERMODYNAMIC_TEMPERATURE,
        {
            UcumUnitConsts {
                size: Exact::int(5) / Exact::int(9),
                zero: Exact::ZERO,
                symbol: "[degR]",
                name: "degrees Rankine",
            }
        },
    >;
}
