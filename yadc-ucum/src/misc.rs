//! The quantities of `misc` and their units.

#[allow(unused_imports)]
use crate::*;

/// An action area, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ActionArea<
    N,
    const U: UcumUnit = { UcumUnit::ONE.times(METER.unit(), 2) },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Barn (`b`), a unit of `ActionArea`: `n` of them is `n · 1e-28` base units.
pub const BARN: Qty<
    {
        UcumUnit::scaled(
            "b",
            Exact::dec(1, -28),
            UcumUnit::ONE.times(METER.unit(), 2),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `ElectricConductance` is `crate::ElectricConductance`.

/// Mho (`mho`), a unit of `ElectricConductance`: `n` of them is `n · 1e-3` base units.
pub const MHO: Qty<
    {
        UcumUnit::scaled(
            "mho",
            Exact::dec(1, -3),
            UcumUnit::ONE
                .times(METER.unit(), -2)
                .times(SECOND.unit(), 1)
                .times(GRAM.unit(), -1)
                .times(COULOMB.unit(), 2),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Length` is `crate::Length`.

/// Line (`[lne]`), a unit of `Length`: `n` of them is `n · 127 / 60000` base units.
pub const LINE: Qty<
    { UcumUnit::scaled("[lne]", Exact::int(127) / Exact::int(60000), METER.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Point (`[pnt]`), a unit of `Length`: `n` of them is `n · 127 / 360000` base units.
pub const POINT: Qty<
    { UcumUnit::scaled("[pnt]", Exact::int(127) / Exact::int(360000), METER.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Pica (`[pca]`), a unit of `Length`: `n` of them is `n · 127 / 30000` base units.
pub const PICA: Qty<
    { UcumUnit::scaled("[pca]", Exact::int(127) / Exact::int(30000), METER.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Printer's point (`[pnt_pr]`), a unit of `Length`: `n` of them is `n · 3514598e-10` base units.
pub const PRINTER_S_POINT: Qty<
    { UcumUnit::scaled("[pnt_pr]", Exact::dec(3514598, -10), METER.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Printer's pica (`[pca_pr]`), a unit of `Length`: `n` of them is `n · 42175176e-10` base units.
pub const PRINTER_S_PICA: Qty<
    { UcumUnit::scaled("[pca_pr]", Exact::dec(42175176, -10), METER.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// French foot (`[pied]`), a unit of `Length`: `n` of them is `n · 3248e-4` base units.
pub const FRENCH_FOOT: Qty<
    { UcumUnit::scaled("[pied]", Exact::dec(3248, -4), METER.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// French inch (`[pouce]`), a unit of `Length`: `n` of them is `n · 203 / 7500` base units.
pub const FRENCH_INCH: Qty<
    { UcumUnit::scaled("[pouce]", Exact::int(203) / Exact::int(7500), METER.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// French line (`[ligne]`), a unit of `Length`: `n` of them is `n · 203 / 90000` base units.
pub const FRENCH_LINE: Qty<
    { UcumUnit::scaled("[ligne]", Exact::int(203) / Exact::int(90000), METER.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Didot's point (`[didot]`), a unit of `Length`: `n` of them is `n · 203 / 540000` base units.
pub const DIDOT_S_POINT: Qty<
    {
        UcumUnit::scaled(
            "[didot]",
            Exact::int(203) / Exact::int(540000),
            METER.unit(),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Didot's pica (`[cicero]`), a unit of `Length`: `n` of them is `n · 203 / 45000` base units.
pub const DIDOT_S_PICA: Qty<
    {
        UcumUnit::scaled(
            "[cicero]",
            Exact::int(203) / Exact::int(45000),
            METER.unit(),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Ångström (`Ao`), a unit of `Length`: `n` of them is `n · 1e-10` base units.
pub const ANGSTROM: Qty<
    { UcumUnit::scaled("Ao", Exact::dec(1, -10), METER.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Smoot (`[smoot]`), a unit of `Length`: `n` of them is `n · 17018e-4` base units.
pub const SMOOT: Qty<
    { UcumUnit::scaled("[smoot]", Exact::dec(17018, -4), METER.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Mass` is `crate::Mass`.

/// Metric carat (`[car_m]`), a unit of `Mass`: `n` of them is `n · 2e-1` base units.
pub const METRIC_CARAT: Qty<
    { UcumUnit::scaled("[car_m]", Exact::dec(2, -1), GRAM.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// A mass fraction, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MassFraction<N, const U: UcumUnit = { UcumUnit::ONE }, const B: Base = BASE> =
    Qty<U, N, B>;

/// Carat of gold alloys (`[car_Au]`), a unit of `MassFraction`: `n` of them is `n · 1 / 24` base units.
pub const CARAT_OF_GOLD_ALLOYS: Qty<
    { UcumUnit::scaled("[car_Au]", Exact::int(1) / Exact::int(24), UcumUnit::ONE) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `PlaneAngle` is `crate::PlaneAngle`.

/// Circle (`circ`), a unit of `PlaneAngle`: `n` of them is `n · 2 * core::f64::consts::PI` base units.
pub const CIRCLE: Qty<
    { UcumUnit::scaled("circ", Exact::int(2) * Exact::PI, RADIAN.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Pressure` is `crate::Pressure`.

/// Technical atmosphere (`att`), a unit of `Pressure`: `n` of them is `n · 980665e2` base units.
pub const TECHNICAL_ATMOSPHERE: Qty<
    {
        UcumUnit::scaled(
            "att",
            Exact::int(98066500),
            UcumUnit::ONE
                .times(METER.unit(), -1)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Pound per square inch (`[psi]`), a unit of `Pressure`: `n` of them is `n · 8896443230521 / 1290320` base units.
pub const POUND_PER_SQUARE_INCH: Qty<
    {
        UcumUnit::scaled(
            "[psi]",
            Exact::int(8896443230521) / Exact::int(1290320),
            UcumUnit::ONE
                .times(METER.unit(), -1)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `SolidAngle` is `crate::SolidAngle`.

/// Sphere (`sph`), a unit of `SolidAngle`: `n` of them is `n · 4 * core::f64::consts::PI` base units.
pub const SPHERE: Qty<
    {
        UcumUnit::scaled(
            "sph",
            Exact::int(4) * Exact::PI,
            UcumUnit::ONE.times(RADIAN.unit(), 2),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// A turbidity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Turbidity<N, const U: UcumUnit = { UcumUnit::ONE }, const B: Base = BASE> = Qty<U, N, B>;

/// Nephelometric Turbidity Unit (`[NTU]`), a unit of `Turbidity`: the coherent unit, a product of base units.
pub const NEPHELOMETRIC_TURBIDITY_UNIT: Qty<{ UcumUnit::ONE }, Exact, BASE> = Qty::ONE;

/// Formazin Nephelometric Unit (`[FNU]`), a unit of `Turbidity`: `n` of them is `n · 1` base units.
pub const FORMAZIN_NEPHELOMETRIC_UNIT: Qty<
    { UcumUnit::scaled("[FNU]", Exact::int(1), UcumUnit::ONE) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Volume` is `crate::Volume`.

/// Stere (`st`), a unit of `Volume`: the coherent unit, a product of base units.
pub const STERE: Qty<{ UcumUnit::ONE.times(METER.unit(), 3) }, Exact, BASE> = Qty::ONE;
