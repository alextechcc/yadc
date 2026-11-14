//! The quantities of `heat` and their units.

#[allow(unused_imports)]
use crate::*;

// `Energy` is `crate::Energy`.

/// Calorie at 15 °C (`cal_[15]`), a unit of `Energy`: `n` of them is `n · 41858e-1` base units.
pub const CALORIE_AT15_DEGREE_C: Qty<
    {
        UcumUnit::scaled(
            "cal_[15]",
            Exact::dec(41858, -1),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Calorie at 20 °C (`cal_[20]`), a unit of `Energy`: `n` of them is `n · 41819e-1` base units.
pub const CALORIE_AT20_DEGREE_C: Qty<
    {
        UcumUnit::scaled(
            "cal_[20]",
            Exact::dec(41819, -1),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Mean calorie (`cal_m`), a unit of `Energy`: `n` of them is `n · 419002e-2` base units.
pub const MEAN_CALORIE: Qty<
    {
        UcumUnit::scaled(
            "cal_m",
            Exact::dec(419002, -2),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// International table calorie (`cal_IT`), a unit of `Energy`: `n` of them is `n · 41868e-1` base units.
pub const INTERNATIONAL_TABLE_CALORIE: Qty<
    {
        UcumUnit::scaled(
            "cal_IT",
            Exact::dec(41868, -1),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Thermochemical calorie (`cal_th`), a unit of `Energy`: `n` of them is `n · 4184` base units.
pub const THERMOCHEMICAL_CALORIE: Qty<
    {
        UcumUnit::scaled(
            "cal_th",
            Exact::int(4184),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Calorie (`cal`), a unit of `Energy`: `n` of them is `n · 4184` base units.
pub const CALORIE: Qty<
    {
        UcumUnit::scaled(
            "cal",
            Exact::int(4184),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Nutrition label Calories (`[Cal]`), a unit of `Energy`: `n` of them is `n · 4184e3` base units.
pub const NUTRITION_LABEL_CALORIES: Qty<
    {
        UcumUnit::scaled(
            "[Cal]",
            Exact::int(4184000),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// British thermal unit at 39 °F (`[Btu_39]`), a unit of `Energy`: `n` of them is `n · 105967e1` base units.
pub const BRITISH_THERMAL_UNIT_AT39_DEGREE_F: Qty<
    {
        UcumUnit::scaled(
            "[Btu_39]",
            Exact::int(1059670),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// British thermal unit at 59 °F (`[Btu_59]`), a unit of `Energy`: `n` of them is `n · 10548e2` base units.
pub const BRITISH_THERMAL_UNIT_AT59_DEGREE_F: Qty<
    {
        UcumUnit::scaled(
            "[Btu_59]",
            Exact::int(1054800),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// British thermal unit at 60 °F (`[Btu_60]`), a unit of `Energy`: `n` of them is `n · 105468e1` base units.
pub const BRITISH_THERMAL_UNIT_AT60_DEGREE_F: Qty<
    {
        UcumUnit::scaled(
            "[Btu_60]",
            Exact::int(1054680),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Mean British thermal unit (`[Btu_m]`), a unit of `Energy`: `n` of them is `n · 105587e1` base units.
pub const MEAN_BRITISH_THERMAL_UNIT: Qty<
    {
        UcumUnit::scaled(
            "[Btu_m]",
            Exact::int(1055870),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// International table British thermal unit (`[Btu_IT]`), a unit of `Energy`: `n` of them is `n · 105505585262e-5` base units.
pub const INTERNATIONAL_TABLE_BRITISH_THERMAL_UNIT: Qty<
    {
        UcumUnit::scaled(
            "[Btu_IT]",
            Exact::dec(105505585262, -5),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Thermochemical British thermal unit (`[Btu_th]`), a unit of `Energy`: `n` of them is `n · 105435e1` base units.
pub const THERMOCHEMICAL_BRITISH_THERMAL_UNIT: Qty<
    {
        UcumUnit::scaled(
            "[Btu_th]",
            Exact::int(1054350),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// British thermal unit (`[Btu]`), a unit of `Energy`: `n` of them is `n · 105435e1` base units.
pub const BRITISH_THERMAL_UNIT: Qty<
    {
        UcumUnit::scaled(
            "[Btu]",
            Exact::int(1054350),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// A linear mass density of textile thread, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type LinearMassDensityOfTextileThread<
    N,
    const U: UcumUnit = { UcumUnit::ONE.times(METER.unit(), -1).times(GRAM.unit(), 1) },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Tex (`tex`), a unit of `LinearMassDensityOfTextileThread`: `n` of them is `n · 1e-3` base units.
pub const TEX: Qty<
    {
        UcumUnit::scaled(
            "tex",
            Exact::dec(1, -3),
            UcumUnit::ONE.times(METER.unit(), -1).times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Denier (`[den]`), a unit of `LinearMassDensityOfTextileThread`: `n` of them is `n · 1 / 9000` base units.
pub const DENIER: Qty<
    {
        UcumUnit::scaled(
            "[den]",
            Exact::int(1) / Exact::int(9000),
            UcumUnit::ONE.times(METER.unit(), -1).times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Power` is `crate::Power`.

/// Horsepower (`[HP]`), a unit of `Power`: `n` of them is `n · 74569987158227022e-11` base units.
pub const HORSEPOWER: Qty<
    {
        UcumUnit::scaled(
            "[HP]",
            Exact::dec(74569987158227022, -11),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -3)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// A temperature interval, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type TemperatureInterval<N, const U: UcumUnit = { KELVIN.unit() }, const B: Base = BASE> =
    Qty<U, N, B>;

/// Degrees Fahrenheit (`[degF]`), a unit of `TemperatureInterval`: `n` of them is `n · 5.0 / 9.0` base units.
pub const DEGREE_FAHRENHEIT: Qty<
    { UcumUnit::scaled("[degF]", Exact::int(5) / Exact::int(9), KELVIN.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// A thermodynamic temperature, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ThermodynamicTemperature<N, const U: UcumUnit = { KELVIN.unit() }, const B: Base = BASE> =
    Qty<U, N, B>;

/// Degrees Celsius (`Cel`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 1.0 + 273.15` base units.
pub const CELSIUS: Qty<
    { UcumUnit::offset("Cel", Exact::int(1), Exact::dec(27315, -2), KELVIN.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Degrees Fahrenheit (`[degF]`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 5.0 / 9.0 + 459.67 * 5.0 / 9.0` base units.
pub const FAHRENHEIT: Qty<
    {
        UcumUnit::offset(
            "[degF]",
            Exact::int(5) / Exact::int(9),
            Exact::dec(45967, -2) * Exact::int(5) / Exact::int(9),
            KELVIN.unit(),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Degrees Rankine (`[degR]`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 5.0 / 9.0` base units.
pub const RANKINE: Qty<
    { UcumUnit::scaled("[degR]", Exact::int(5) / Exact::int(9), KELVIN.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;
