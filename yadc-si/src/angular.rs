//! The quantities of `angular` and their units.

#[allow(unused_imports)]
use crate::*;

/// An angle, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Angle<N, const U: SIUnit = { RADIAN.unit() }, const IS_BASE: bool = true> =
    SIQuantity<U, N, IS_BASE>;

/// Revolutions (`r`), a unit of `Angle`: `n` of them is `n · 2.0 * core::f64::consts::PI` base units.
pub const REVOLUTION: SIQuantity<
    { SIUnit::scaled("r", Exact::int(2) * Exact::PI, RADIAN.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Degrees (`°`), a unit of `Angle`: `n` of them is `n · core::f64::consts::PI / 180.0` base units.
pub const DEGREE: SIQuantity<
    { SIUnit::scaled("°", Exact::PI / Exact::int(180), RADIAN.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Gons (`gon`), a unit of `Angle`: `n` of them is `n · core::f64::consts::PI / 200.0` base units.
pub const GON: SIQuantity<
    { SIUnit::scaled("gon", Exact::PI / Exact::int(200), RADIAN.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Mils (`mil`), a unit of `Angle`: `n` of them is `n · 2.0 * core::f64::consts::PI / 6400.0` base units.
pub const MIL: SIQuantity<
    {
        SIUnit::scaled(
            "mil",
            Exact::int(2) * Exact::PI / Exact::int(6400),
            RADIAN.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Minutes (`′`), a unit of `Angle`: `n` of them is `n · core::f64::consts::PI / 10800.0` base units.
pub const MINUTE: SIQuantity<
    { SIUnit::scaled("′", Exact::PI / Exact::int(10800), RADIAN.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Seconds (`″`), a unit of `Angle`: `n` of them is `n · core::f64::consts::PI / 648000.0` base units.
pub const ANGLE_SECOND: SIQuantity<
    { SIUnit::scaled("″", Exact::PI / Exact::int(648000), RADIAN.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// A solid angle, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type SolidAngle<
    N,
    const U: SIUnit = { SIUnit::ONE.times(RADIAN.unit(), 2) },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// Steradians (`sr`), a unit of `SolidAngle`: the coherent unit, a product of base units.
pub const STERADIAN: SIQuantity<{ SIUnit::ONE.times(RADIAN.unit(), 2) }, Exact, true> =
    SIQuantity::ONE;

/// Spats (`sp`), a unit of `SolidAngle`: `n` of them is `n · 4.0 * core::f64::consts::PI` base units.
pub const SPAT: SIQuantity<
    {
        SIUnit::scaled(
            "sp",
            Exact::int(4) * Exact::PI,
            SIUnit::ONE.times(RADIAN.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Square degrees (`°²`), a unit of `SolidAngle`: `n` of them is `n · (core::f64::consts::PI / 180.0) * (core::f64::consts::PI / 180.0)` base units.
pub const SQUARE_DEGREE: SIQuantity<
    {
        SIUnit::scaled(
            "°²",
            (Exact::PI / Exact::int(180)) * (Exact::PI / Exact::int(180)),
            SIUnit::ONE.times(RADIAN.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Square minutes (`′²`), a unit of `SolidAngle`: `n` of them is `n · (core::f64::consts::PI / 10800.0) * (core::f64::consts::PI / 10800.0)` base units.
pub const SQUARE_MINUTE: SIQuantity<
    {
        SIUnit::scaled(
            "′²",
            (Exact::PI / Exact::int(10800)) * (Exact::PI / Exact::int(10800)),
            SIUnit::ONE.times(RADIAN.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Square seconds (`″²`), a unit of `SolidAngle`: `n` of them is `n · (core::f64::consts::PI / 648000.0) * (core::f64::consts::PI / 648000.0)` base units.
pub const SQUARE_SECOND: SIQuantity<
    {
        SIUnit::scaled(
            "″²",
            (Exact::PI / Exact::int(648000)) * (Exact::PI / Exact::int(648000)),
            SIUnit::ONE.times(RADIAN.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// An angular velocity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type AngularVelocity<
    N,
    const U: SIUnit = { SIUnit::ONE.times(SECOND.unit(), -1).times(RADIAN.unit(), 1) },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// An angular acceleration, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type AngularAcceleration<
    N,
    const U: SIUnit = { SIUnit::ONE.times(SECOND.unit(), -2).times(RADIAN.unit(), 1) },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// An angular jerk, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type AngularJerk<
    N,
    const U: SIUnit = { SIUnit::ONE.times(SECOND.unit(), -3).times(RADIAN.unit(), 1) },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// An angular absement, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type AngularAbsement<
    N,
    const U: SIUnit = { SECOND.unit().times(RADIAN.unit(), 1) },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// Radian seconds (`rad · s`), a unit of `AngularAbsement`: the coherent unit, a product of base units.
pub const RADIAN_SECOND: SIQuantity<{ SECOND.unit().times(RADIAN.unit(), 1) }, Exact, true> =
    SIQuantity::ONE;

/// Degree seconds (`° · s`), a unit of `AngularAbsement`: `n` of them is `n · core::f64::consts::PI / 180.0` base units.
pub const DEGREE_SECOND: SIQuantity<
    {
        SIUnit::scaled(
            "° · s",
            Exact::PI / Exact::int(180),
            SECOND.unit().times(RADIAN.unit(), 1),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// An angular momentum, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type AngularMomentum<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -1)
            .times(RADIAN.unit(), -1)
    },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// Reduced Planck constants (`ħ`), a unit of `AngularMomentum`: `n` of them is `n · 6.62607015e-34 / (2.0 * core::f64::consts::PI)` base units.
pub const REDUCED_PLANCK_CONSTANT: SIQuantity<
    {
        SIUnit::scaled(
            "ħ",
            Exact::dec(662607015, -42) / (Exact::int(2) * Exact::PI),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -1)
                .times(RADIAN.unit(), -1),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Newton metre seconds (`N · m · s`), a unit of `AngularMomentum`: the coherent unit, a product of base units.
pub const NEWTON_METRE_SECOND: SIQuantity<
    {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -1)
            .times(RADIAN.unit(), -1)
    },
    Exact,
    true,
> = SIQuantity::ONE;
