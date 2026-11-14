//! The quantities of `heat` and their units.

#[allow(unused_imports)]
use crate::*;

// `Energy` is `crate::Energy`.

/// Calorie at 15 °C (`cal_[15]`), a unit of `Energy`: `n` of them is `n · 41858e-1` base units.
pub const CALORIE_AT15_DEGREE_C: Unit<
    {
        UcumUnit::scaled(
            "cal_[15]",
            Exact::dec(41858, -1),
            METER_SQUARED_GRAM_PER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// Calorie at 20 °C (`cal_[20]`), a unit of `Energy`: `n` of them is `n · 41819e-1` base units.
pub const CALORIE_AT20_DEGREE_C: Unit<
    {
        UcumUnit::scaled(
            "cal_[20]",
            Exact::dec(41819, -1),
            METER_SQUARED_GRAM_PER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// Mean calorie (`cal_m`), a unit of `Energy`: `n` of them is `n · 419002e-2` base units.
pub const MEAN_CALORIE: Unit<
    {
        UcumUnit::scaled(
            "cal_m",
            Exact::dec(419002, -2),
            METER_SQUARED_GRAM_PER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// International table calorie (`cal_IT`), a unit of `Energy`: `n` of them is `n · 41868e-1` base units.
pub const INTERNATIONAL_TABLE_CALORIE: Unit<
    {
        UcumUnit::scaled(
            "cal_IT",
            Exact::dec(41868, -1),
            METER_SQUARED_GRAM_PER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// Thermochemical calorie (`cal_th`), a unit of `Energy`: `n` of them is `n · 4184` base units.
pub const THERMOCHEMICAL_CALORIE: Unit<
    {
        UcumUnit::scaled(
            "cal_th",
            Exact::int(4184),
            METER_SQUARED_GRAM_PER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// Calorie (`cal`), a unit of `Energy`: `n` of them is `n · 4184` base units.
pub const CALORIE: Unit<
    {
        UcumUnit::scaled(
            "cal",
            Exact::int(4184),
            METER_SQUARED_GRAM_PER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// Nutrition label Calories (`[Cal]`), a unit of `Energy`: `n` of them is `n · 4184e3` base units.
pub const NUTRITION_LABEL_CALORIES: Unit<
    {
        UcumUnit::scaled(
            "[Cal]",
            Exact::int(4184000),
            METER_SQUARED_GRAM_PER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// British thermal unit at 39 °F (`[Btu_39]`), a unit of `Energy`: `n` of them is `n · 105967e1` base units.
pub const BRITISH_THERMAL_UNIT_AT39_DEGREE_F: Unit<
    {
        UcumUnit::scaled(
            "[Btu_39]",
            Exact::int(1059670),
            METER_SQUARED_GRAM_PER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// British thermal unit at 59 °F (`[Btu_59]`), a unit of `Energy`: `n` of them is `n · 10548e2` base units.
pub const BRITISH_THERMAL_UNIT_AT59_DEGREE_F: Unit<
    {
        UcumUnit::scaled(
            "[Btu_59]",
            Exact::int(1054800),
            METER_SQUARED_GRAM_PER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// British thermal unit at 60 °F (`[Btu_60]`), a unit of `Energy`: `n` of them is `n · 105468e1` base units.
pub const BRITISH_THERMAL_UNIT_AT60_DEGREE_F: Unit<
    {
        UcumUnit::scaled(
            "[Btu_60]",
            Exact::int(1054680),
            METER_SQUARED_GRAM_PER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// Mean British thermal unit (`[Btu_m]`), a unit of `Energy`: `n` of them is `n · 105587e1` base units.
pub const MEAN_BRITISH_THERMAL_UNIT: Unit<
    {
        UcumUnit::scaled(
            "[Btu_m]",
            Exact::int(1055870),
            METER_SQUARED_GRAM_PER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// International table British thermal unit (`[Btu_IT]`), a unit of `Energy`: `n` of them is `n · 105505585262e-5` base units.
pub const INTERNATIONAL_TABLE_BRITISH_THERMAL_UNIT: Unit<
    {
        UcumUnit::scaled(
            "[Btu_IT]",
            Exact::dec(105505585262, -5),
            METER_SQUARED_GRAM_PER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// Thermochemical British thermal unit (`[Btu_th]`), a unit of `Energy`: `n` of them is `n · 105435e1` base units.
pub const THERMOCHEMICAL_BRITISH_THERMAL_UNIT: Unit<
    {
        UcumUnit::scaled(
            "[Btu_th]",
            Exact::int(1054350),
            METER_SQUARED_GRAM_PER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// British thermal unit (`[Btu]`), a unit of `Energy`: `n` of them is `n · 105435e1` base units.
pub const BRITISH_THERMAL_UNIT: Unit<
    {
        UcumUnit::scaled(
            "[Btu]",
            Exact::int(1054350),
            METER_SQUARED_GRAM_PER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// A linear mass density of textile thread, in `N`s of the unit `U`: by default its base unit, `GRAM_PER_METER`.
pub type LinearMassDensityOfTextileThread<
    N,
    const U: UcumUnit = { GRAM_PER_METER.unit() },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// The base unit of `LinearMassDensityOfTextileThread`: a product of base units.
pub const GRAM_PER_METER: Unit<{ (GRAM / METER).unit() }, BASE> = Unit::ONE;

/// Tex (`tex`), a unit of `LinearMassDensityOfTextileThread`: `n` of them is `n · 1e-3` base units.
pub const TEX: Unit<{ UcumUnit::scaled("tex", Exact::dec(1, -3), GRAM_PER_METER.unit()) }> =
    Unit::ONE;

/// Denier (`[den]`), a unit of `LinearMassDensityOfTextileThread`: `n` of them is `n · 1 / 9000` base units.
pub const DENIER: Unit<
    {
        UcumUnit::scaled(
            "[den]",
            Exact::int(1) / Exact::int(9000),
            GRAM_PER_METER.unit(),
        )
    },
> = Unit::ONE;

// `Power` is `crate::Power`.

/// Horsepower (`[HP]`), a unit of `Power`: `n` of them is `n · 74569987158227022e-11` base units.
pub const HORSEPOWER: Unit<
    {
        UcumUnit::scaled(
            "[HP]",
            Exact::dec(74569987158227022, -11),
            METER_SQUARED_GRAM_PER_SECOND_CUBED.unit(),
        )
    },
> = Unit::ONE;

/// A temperature interval, in `N`s of the unit `U`: by default its base unit, `KELVIN`.
pub type TemperatureInterval<N, const U: UcumUnit = { KELVIN.unit() }, const B: Base = BASE> =
    Qty<U, N, B>;

/// Degrees Fahrenheit (`[degF]`), a unit of `TemperatureInterval`: `n` of them is `n · 5.0 / 9.0` base units.
pub const DEGREE_FAHRENHEIT: Unit<
    { UcumUnit::scaled("[degF]", Exact::int(5) / Exact::int(9), KELVIN.unit()) },
> = Unit::ONE;

/// A thermodynamic temperature, in `N`s of the unit `U`: by default its base unit, `KELVIN`.
pub type ThermodynamicTemperature<N, const U: UcumUnit = { KELVIN.unit() }, const B: Base = BASE> =
    Qty<U, N, B>;

/// Degrees Celsius (`Cel`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 1.0 + 273.15` base units.
pub const CELSIUS: Unit<
    { UcumUnit::offset("Cel", Exact::int(1), Exact::dec(27315, -2), KELVIN.unit()) },
> = Unit::ONE;

/// Degrees Fahrenheit (`[degF]`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 5.0 / 9.0 + 459.67 * 5.0 / 9.0` base units.
pub const FAHRENHEIT: Unit<
    {
        UcumUnit::offset(
            "[degF]",
            Exact::int(5) / Exact::int(9),
            Exact::dec(45967, -2) * Exact::int(5) / Exact::int(9),
            KELVIN.unit(),
        )
    },
> = Unit::ONE;

/// Degrees Rankine (`[degR]`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 5.0 / 9.0` base units.
pub const RANKINE: Unit<
    { UcumUnit::scaled("[degR]", Exact::int(5) / Exact::int(9), KELVIN.unit()) },
> = Unit::ONE;
