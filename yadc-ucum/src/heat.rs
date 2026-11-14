use crate::{Dim, Kind};
use yadc::Dimension;

/// The dimension of `Energy`: `Qty<ENERGY, N>` is one.
pub const ENERGY: Dim = Dim {
    m: 2,
    s: -2,
    g: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `LinearMassDensityOfTextileThread`: `Qty<LINEAR_MASS_DENSITY_OF_TEXTILE_THREAD, N>` is one.
pub const LINEAR_MASS_DENSITY_OF_TEXTILE_THREAD: Dim = Dim {
    m: -1,
    g: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Power`: `Qty<POWER, N>` is one.
pub const POWER: Dim = Dim {
    m: 2,
    s: -3,
    g: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `TemperatureInterval`: `Qty<TEMPERATURE_INTERVAL, N>` is one.
pub const TEMPERATURE_INTERVAL: Dim = Dim {
    K: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `ThermodynamicTemperature`: `Qty<THERMODYNAMIC_TEMPERATURE, N>` is one.
pub const THERMODYNAMIC_TEMPERATURE: Dim = Dim {
    K: 1,
    kind: Kind::Absolute,
    ..Dim::DIMENSIONLESS
};

/// The units of this module's quantities: each a constant of its own unit
/// type, its size and zero in its quantity's coherent unit, exact
/// (`MILE.quantity(3.0)`, `MILE.value(d)`).
pub mod units {
    use super::*;
    use crate::{Exact, UcumUnit, UcumUnitConsts};

    /// Calorie at 15 °C (`cal_[15]`), a unit of `Energy`: `n` of them is `n · 41858e-1` coherent units.
    pub const CALORIE_AT15_DEGREE_C: UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::dec(41858, -1),
                zero: Exact::ZERO,
                symbol: "cal_[15]",
                name: "calorie at 15 °C",
            }
        },
    > = UcumUnit;

    /// Calorie at 20 °C (`cal_[20]`), a unit of `Energy`: `n` of them is `n · 41819e-1` coherent units.
    pub const CALORIE_AT20_DEGREE_C: UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::dec(41819, -1),
                zero: Exact::ZERO,
                symbol: "cal_[20]",
                name: "calorie at 20 °C",
            }
        },
    > = UcumUnit;

    /// Mean calorie (`cal_m`), a unit of `Energy`: `n` of them is `n · 419002e-2` coherent units.
    pub const MEAN_CALORIE: UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::dec(419002, -2),
                zero: Exact::ZERO,
                symbol: "cal_m",
                name: "mean calorie",
            }
        },
    > = UcumUnit;

    /// International table calorie (`cal_IT`), a unit of `Energy`: `n` of them is `n · 41868e-1` coherent units.
    pub const INTERNATIONAL_TABLE_CALORIE: UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::dec(41868, -1),
                zero: Exact::ZERO,
                symbol: "cal_IT",
                name: "international table calorie",
            }
        },
    > = UcumUnit;

    /// Thermochemical calorie (`cal_th`), a unit of `Energy`: `n` of them is `n · 4184` coherent units.
    pub const THERMOCHEMICAL_CALORIE: UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::int(4184),
                zero: Exact::ZERO,
                symbol: "cal_th",
                name: "thermochemical calorie",
            }
        },
    > = UcumUnit;

    /// Calorie (`cal`), a unit of `Energy`: `n` of them is `n · 4184` coherent units.
    pub const CALORIE: UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::int(4184),
                zero: Exact::ZERO,
                symbol: "cal",
                name: "calorie",
            }
        },
    > = UcumUnit;

    /// Nutrition label Calories (`[Cal]`), a unit of `Energy`: `n` of them is `n · 4184e3` coherent units.
    pub const NUTRITION_LABEL_CALORIES: UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::int(4184000),
                zero: Exact::ZERO,
                symbol: "[Cal]",
                name: "nutrition label Calories",
            }
        },
    > = UcumUnit;

    /// British thermal unit at 39 °F (`[Btu_39]`), a unit of `Energy`: `n` of them is `n · 105967e1` coherent units.
    pub const BRITISH_THERMAL_UNIT_AT39_DEGREE_F: UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::int(1059670),
                zero: Exact::ZERO,
                symbol: "[Btu_39]",
                name: "British thermal unit at 39 °F",
            }
        },
    > = UcumUnit;

    /// British thermal unit at 59 °F (`[Btu_59]`), a unit of `Energy`: `n` of them is `n · 10548e2` coherent units.
    pub const BRITISH_THERMAL_UNIT_AT59_DEGREE_F: UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::int(1054800),
                zero: Exact::ZERO,
                symbol: "[Btu_59]",
                name: "British thermal unit at 59 °F",
            }
        },
    > = UcumUnit;

    /// British thermal unit at 60 °F (`[Btu_60]`), a unit of `Energy`: `n` of them is `n · 105468e1` coherent units.
    pub const BRITISH_THERMAL_UNIT_AT60_DEGREE_F: UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::int(1054680),
                zero: Exact::ZERO,
                symbol: "[Btu_60]",
                name: "British thermal unit at 60 °F",
            }
        },
    > = UcumUnit;

    /// Mean British thermal unit (`[Btu_m]`), a unit of `Energy`: `n` of them is `n · 105587e1` coherent units.
    pub const MEAN_BRITISH_THERMAL_UNIT: UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::int(1055870),
                zero: Exact::ZERO,
                symbol: "[Btu_m]",
                name: "mean British thermal unit",
            }
        },
    > = UcumUnit;

    /// International table British thermal unit (`[Btu_IT]`), a unit of `Energy`: `n` of them is `n · 105505585262e-5` coherent units.
    pub const INTERNATIONAL_TABLE_BRITISH_THERMAL_UNIT: UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::dec(105505585262, -5),
                zero: Exact::ZERO,
                symbol: "[Btu_IT]",
                name: "international table British thermal unit",
            }
        },
    > = UcumUnit;

    /// Thermochemical British thermal unit (`[Btu_th]`), a unit of `Energy`: `n` of them is `n · 105435e1` coherent units.
    pub const THERMOCHEMICAL_BRITISH_THERMAL_UNIT: UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::int(1054350),
                zero: Exact::ZERO,
                symbol: "[Btu_th]",
                name: "thermochemical British thermal unit",
            }
        },
    > = UcumUnit;

    /// British thermal unit (`[Btu]`), a unit of `Energy`: `n` of them is `n · 105435e1` coherent units.
    pub const BRITISH_THERMAL_UNIT: UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::int(1054350),
                zero: Exact::ZERO,
                symbol: "[Btu]",
                name: "British thermal unit",
            }
        },
    > = UcumUnit;

    /// Tex (`tex`), a unit of `LinearMassDensityOfTextileThread`: `n` of them is `n · 1e-3` coherent units.
    pub const TEX: UcumUnit<
        LINEAR_MASS_DENSITY_OF_TEXTILE_THREAD,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "tex",
                name: "tex",
            }
        },
    > = UcumUnit;

    /// Denier (`[den]`), a unit of `LinearMassDensityOfTextileThread`: `n` of them is `n · 1 / 9000` coherent units.
    pub const DENIER: UcumUnit<
        LINEAR_MASS_DENSITY_OF_TEXTILE_THREAD,
        {
            UcumUnitConsts {
                size: Exact::int(1) / Exact::int(9000),
                zero: Exact::ZERO,
                symbol: "[den]",
                name: "Denier",
            }
        },
    > = UcumUnit;

    /// Horsepower (`[HP]`), a unit of `Power`: `n` of them is `n · 74569987158227022e-11` coherent units.
    pub const HORSEPOWER: UcumUnit<
        POWER,
        {
            UcumUnitConsts {
                size: Exact::dec(74569987158227022, -11),
                zero: Exact::ZERO,
                symbol: "[HP]",
                name: "horsepower",
            }
        },
    > = UcumUnit;

    /// Kelvin (`K`), a unit of `TemperatureInterval`: `n` of them is `n · 1.0` coherent units.
    pub const KELVIN: UcumUnit<
        TEMPERATURE_INTERVAL,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "K",
                name: "kelvin",
            }
        },
    > = UcumUnit;

    /// Degrees Fahrenheit (`[degF]`), a unit of `TemperatureInterval`: `n` of them is `n · 5.0 / 9.0` coherent units.
    pub const DEGREE_FAHRENHEIT: UcumUnit<
        TEMPERATURE_INTERVAL,
        {
            UcumUnitConsts {
                size: Exact::int(5) / Exact::int(9),
                zero: Exact::ZERO,
                symbol: "[degF]",
                name: "degrees Fahrenheit",
            }
        },
    > = UcumUnit;

    /// Kelvin (`K`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 1.0` coherent units.
    pub const KELVIN_SCALE: UcumUnit<
        THERMODYNAMIC_TEMPERATURE,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "K",
                name: "kelvin",
            }
        },
    > = UcumUnit;

    /// Degrees Celsius (`Cel`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 1.0 + 273.15` coherent units.
    pub const CELSIUS: UcumUnit<
        THERMODYNAMIC_TEMPERATURE,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::dec(27315, -2),
                symbol: "Cel",
                name: "degrees Celsius",
            }
        },
    > = UcumUnit;

    /// Degrees Fahrenheit (`[degF]`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 5.0 / 9.0 + 459.67 * 5.0 / 9.0` coherent units.
    pub const FAHRENHEIT: UcumUnit<
        THERMODYNAMIC_TEMPERATURE,
        {
            UcumUnitConsts {
                size: Exact::int(5) / Exact::int(9),
                zero: Exact::dec(45967, -2) * Exact::int(5) / Exact::int(9),
                symbol: "[degF]",
                name: "degrees Fahrenheit",
            }
        },
    > = UcumUnit;

    /// Degrees Rankine (`[degR]`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 5.0 / 9.0` coherent units.
    pub const RANKINE: UcumUnit<
        THERMODYNAMIC_TEMPERATURE,
        {
            UcumUnitConsts {
                size: Exact::int(5) / Exact::int(9),
                zero: Exact::ZERO,
                symbol: "[degR]",
                name: "degrees Rankine",
            }
        },
    > = UcumUnit;
}
