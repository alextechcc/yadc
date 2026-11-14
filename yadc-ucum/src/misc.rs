//! The quantities of `misc` and their units.

#[allow(unused_imports)]
use crate::*;

/// An action area, in `N`s of the unit `U`: by default its base unit, `METER_SQUARED`.
pub type ActionArea<N, const U: UcumUnit = { METER_SQUARED.unit() }, const B: Base = BASE> =
    Qty<U, N, B>;

/// Barn (`b`), a unit of `ActionArea`: `n` of them is `n · 1e-28` base units.
pub const BARN: Unit<{ UcumUnit::scaled("b", Exact::dec(1, -28), METER_SQUARED.unit()) }> =
    Unit::ONE;

// `ElectricConductance` is `crate::ElectricConductance`.

/// Mho (`mho`), a unit of `ElectricConductance`: `n` of them is `n · 1e-3` base units.
pub const MHO: Unit<
    {
        UcumUnit::scaled(
            "mho",
            Exact::dec(1, -3),
            SECOND_COULOMB_SQUARED_PER_METER_SQUARED_GRAM.unit(),
        )
    },
> = Unit::ONE;

// `Length` is `crate::Length`.

/// Line (`[lne]`), a unit of `Length`: `n` of them is `n · 127 / 60000` base units.
pub const LINE: Unit<
    { UcumUnit::scaled("[lne]", Exact::int(127) / Exact::int(60000), METER.unit()) },
> = Unit::ONE;

/// Point (`[pnt]`), a unit of `Length`: `n` of them is `n · 127 / 360000` base units.
pub const POINT: Unit<
    { UcumUnit::scaled("[pnt]", Exact::int(127) / Exact::int(360000), METER.unit()) },
> = Unit::ONE;

/// Pica (`[pca]`), a unit of `Length`: `n` of them is `n · 127 / 30000` base units.
pub const PICA: Unit<
    { UcumUnit::scaled("[pca]", Exact::int(127) / Exact::int(30000), METER.unit()) },
> = Unit::ONE;

/// Printer's point (`[pnt_pr]`), a unit of `Length`: `n` of them is `n · 3514598e-10` base units.
pub const PRINTER_S_POINT: Unit<
    { UcumUnit::scaled("[pnt_pr]", Exact::dec(3514598, -10), METER.unit()) },
> = Unit::ONE;

/// Printer's pica (`[pca_pr]`), a unit of `Length`: `n` of them is `n · 42175176e-10` base units.
pub const PRINTER_S_PICA: Unit<
    { UcumUnit::scaled("[pca_pr]", Exact::dec(42175176, -10), METER.unit()) },
> = Unit::ONE;

/// French foot (`[pied]`), a unit of `Length`: `n` of them is `n · 3248e-4` base units.
pub const FRENCH_FOOT: Unit<{ UcumUnit::scaled("[pied]", Exact::dec(3248, -4), METER.unit()) }> =
    Unit::ONE;

/// French inch (`[pouce]`), a unit of `Length`: `n` of them is `n · 203 / 7500` base units.
pub const FRENCH_INCH: Unit<
    { UcumUnit::scaled("[pouce]", Exact::int(203) / Exact::int(7500), METER.unit()) },
> = Unit::ONE;

/// French line (`[ligne]`), a unit of `Length`: `n` of them is `n · 203 / 90000` base units.
pub const FRENCH_LINE: Unit<
    { UcumUnit::scaled("[ligne]", Exact::int(203) / Exact::int(90000), METER.unit()) },
> = Unit::ONE;

/// Didot's point (`[didot]`), a unit of `Length`: `n` of them is `n · 203 / 540000` base units.
pub const DIDOT_S_POINT: Unit<
    {
        UcumUnit::scaled(
            "[didot]",
            Exact::int(203) / Exact::int(540000),
            METER.unit(),
        )
    },
> = Unit::ONE;

/// Didot's pica (`[cicero]`), a unit of `Length`: `n` of them is `n · 203 / 45000` base units.
pub const DIDOT_S_PICA: Unit<
    {
        UcumUnit::scaled(
            "[cicero]",
            Exact::int(203) / Exact::int(45000),
            METER.unit(),
        )
    },
> = Unit::ONE;

/// Ångström (`Ao`), a unit of `Length`: `n` of them is `n · 1e-10` base units.
pub const ANGSTROM: Unit<{ UcumUnit::scaled("Ao", Exact::dec(1, -10), METER.unit()) }> = Unit::ONE;

/// Smoot (`[smoot]`), a unit of `Length`: `n` of them is `n · 17018e-4` base units.
pub const SMOOT: Unit<{ UcumUnit::scaled("[smoot]", Exact::dec(17018, -4), METER.unit()) }> =
    Unit::ONE;

// `Mass` is `crate::Mass`.

/// Metric carat (`[car_m]`), a unit of `Mass`: `n` of them is `n · 2e-1` base units.
pub const METRIC_CARAT: Unit<{ UcumUnit::scaled("[car_m]", Exact::dec(2, -1), GRAM.unit()) }> =
    Unit::ONE;

/// A mass fraction, in `N`s of the unit `U`: by default its base unit, `ONE`.
pub type MassFraction<N, const U: UcumUnit = { ONE.unit() }, const B: Base = BASE> = Qty<U, N, B>;

/// Carat of gold alloys (`[car_Au]`), a unit of `MassFraction`: `n` of them is `n · 1 / 24` base units.
pub const CARAT_OF_GOLD_ALLOYS: Unit<
    { UcumUnit::scaled("[car_Au]", Exact::int(1) / Exact::int(24), ONE.unit()) },
> = Unit::ONE;

// `PlaneAngle` is `crate::PlaneAngle`.

/// Circle (`circ`), a unit of `PlaneAngle`: `n` of them is `n · 2 * core::f64::consts::PI` base units.
pub const CIRCLE: Unit<{ UcumUnit::scaled("circ", Exact::int(2) * Exact::PI, RADIAN.unit()) }> =
    Unit::ONE;

// `Pressure` is `crate::Pressure`.

/// Technical atmosphere (`att`), a unit of `Pressure`: `n` of them is `n · 980665e2` base units.
pub const TECHNICAL_ATMOSPHERE: Unit<
    {
        UcumUnit::scaled(
            "att",
            Exact::int(98066500),
            GRAM_PER_METER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// Pound per square inch (`[psi]`), a unit of `Pressure`: `n` of them is `n · 8896443230521 / 1290320` base units.
pub const POUND_PER_SQUARE_INCH: Unit<
    {
        UcumUnit::scaled(
            "[psi]",
            Exact::int(8896443230521) / Exact::int(1290320),
            GRAM_PER_METER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

// `SolidAngle` is `crate::SolidAngle`.

/// Sphere (`sph`), a unit of `SolidAngle`: `n` of them is `n · 4 * core::f64::consts::PI` base units.
pub const SPHERE: Unit<
    { UcumUnit::scaled("sph", Exact::int(4) * Exact::PI, RADIAN_SQUARED.unit()) },
> = Unit::ONE;

/// A turbidity, in `N`s of the unit `U`: by default its base unit, `NEPHELOMETRIC_TURBIDITY_UNIT`.
pub type Turbidity<
    N,
    const U: UcumUnit = { NEPHELOMETRIC_TURBIDITY_UNIT.unit() },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Nephelometric Turbidity Unit (`[NTU]`), a unit of `Turbidity`: the coherent unit, a product of base units.
pub const NEPHELOMETRIC_TURBIDITY_UNIT: Unit<{ UcumUnit::ONE }, BASE> = Unit::ONE;

/// Formazin Nephelometric Unit (`[FNU]`), a unit of `Turbidity`: `n` of them is `n · 1` base units.
pub const FORMAZIN_NEPHELOMETRIC_UNIT: Unit<
    { UcumUnit::scaled("[FNU]", Exact::int(1), NEPHELOMETRIC_TURBIDITY_UNIT.unit()) },
> = Unit::ONE;

// `Volume` is `crate::Volume`.

/// Stere (`st`), a unit of `Volume`: the coherent unit, a product of base units.
pub const STERE: Unit<{ (METER * METER * METER).unit() }, BASE> = Unit::ONE;
