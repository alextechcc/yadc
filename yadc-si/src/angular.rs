//! The quantities of `angular` and their units.

#[allow(unused_imports)]
use crate::*;

/// An angle, in `N`s of the unit `U`: by default its base unit, `RADIAN`.
pub type Angle<N, const U: SIUnit = { RADIAN.unit() }, const B: Base = BASE> = SIQuantity<U, N, B>;

/// Revolutions (`r`), a unit of `Angle`: `n` of them is `n · 2.0 * core::f64::consts::PI` base units.
pub const REVOLUTION: Unit<{ SIUnit::scaled("r", Exact::int(2) * Exact::PI, RADIAN.unit()) }> =
    Unit::ONE;

/// Degrees (`°`), a unit of `Angle`: `n` of them is `n · core::f64::consts::PI / 180.0` base units.
pub const DEGREE: Unit<{ SIUnit::scaled("°", Exact::PI / Exact::int(180), RADIAN.unit()) }> =
    Unit::ONE;

/// Gons (`gon`), a unit of `Angle`: `n` of them is `n · core::f64::consts::PI / 200.0` base units.
pub const GON: Unit<{ SIUnit::scaled("gon", Exact::PI / Exact::int(200), RADIAN.unit()) }> =
    Unit::ONE;

/// Mils (`mil`), a unit of `Angle`: `n` of them is `n · 2.0 * core::f64::consts::PI / 6400.0` base units.
pub const MIL: Unit<
    {
        SIUnit::scaled(
            "mil",
            Exact::int(2) * Exact::PI / Exact::int(6400),
            RADIAN.unit(),
        )
    },
> = Unit::ONE;

/// Minutes (`′`), a unit of `Angle`: `n` of them is `n · core::f64::consts::PI / 10800.0` base units.
pub const MINUTE: Unit<{ SIUnit::scaled("′", Exact::PI / Exact::int(10800), RADIAN.unit()) }> =
    Unit::ONE;

/// Seconds (`″`), a unit of `Angle`: `n` of them is `n · core::f64::consts::PI / 648000.0` base units.
pub const ANGLE_SECOND: Unit<
    { SIUnit::scaled("″", Exact::PI / Exact::int(648000), RADIAN.unit()) },
> = Unit::ONE;

/// A solid angle, in `N`s of the unit `U`: by default its base unit, `STERADIAN`.
pub type SolidAngle<N, const U: SIUnit = { STERADIAN.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Steradians (`sr`), a unit of `SolidAngle`: the coherent unit, a product of base units.
pub const STERADIAN: Unit<{ (RADIAN * RADIAN).unit() }, BASE> = Unit::ONE;

/// Spats (`sp`), a unit of `SolidAngle`: `n` of them is `n · 4.0 * core::f64::consts::PI` base units.
pub const SPAT: Unit<{ SIUnit::scaled("sp", Exact::int(4) * Exact::PI, STERADIAN.unit()) }> =
    Unit::ONE;

/// Square degrees (`°²`), a unit of `SolidAngle`: `n` of them is `n · (core::f64::consts::PI / 180.0) * (core::f64::consts::PI / 180.0)` base units.
pub const SQUARE_DEGREE: Unit<
    {
        SIUnit::scaled(
            "°²",
            (Exact::PI / Exact::int(180)) * (Exact::PI / Exact::int(180)),
            STERADIAN.unit(),
        )
    },
> = Unit::ONE;

/// Square minutes (`′²`), a unit of `SolidAngle`: `n` of them is `n · (core::f64::consts::PI / 10800.0) * (core::f64::consts::PI / 10800.0)` base units.
pub const SQUARE_MINUTE: Unit<
    {
        SIUnit::scaled(
            "′²",
            (Exact::PI / Exact::int(10800)) * (Exact::PI / Exact::int(10800)),
            STERADIAN.unit(),
        )
    },
> = Unit::ONE;

/// Square seconds (`″²`), a unit of `SolidAngle`: `n` of them is `n · (core::f64::consts::PI / 648000.0) * (core::f64::consts::PI / 648000.0)` base units.
pub const SQUARE_SECOND: Unit<
    {
        SIUnit::scaled(
            "″²",
            (Exact::PI / Exact::int(648000)) * (Exact::PI / Exact::int(648000)),
            STERADIAN.unit(),
        )
    },
> = Unit::ONE;

/// An angular velocity, in `N`s of the unit `U`: by default its base unit, `RADIAN_PER_SECOND`.
pub type AngularVelocity<N, const U: SIUnit = { RADIAN_PER_SECOND.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// The base unit of `AngularVelocity`: a product of base units.
pub const RADIAN_PER_SECOND: Unit<{ (RADIAN / SECOND).unit() }, BASE> = Unit::ONE;

/// An angular acceleration, in `N`s of the unit `U`: by default its base unit, `RADIAN_PER_SECOND_SQUARED`.
pub type AngularAcceleration<
    N,
    const U: SIUnit = { RADIAN_PER_SECOND_SQUARED.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `AngularAcceleration`: a product of base units.
pub const RADIAN_PER_SECOND_SQUARED: Unit<{ (RADIAN / (SECOND * SECOND)).unit() }, BASE> =
    Unit::ONE;

/// An angular jerk, in `N`s of the unit `U`: by default its base unit, `RADIAN_PER_SECOND_CUBED`.
pub type AngularJerk<
    N,
    const U: SIUnit = { RADIAN_PER_SECOND_CUBED.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `AngularJerk`: a product of base units.
pub const RADIAN_PER_SECOND_CUBED: Unit<{ (RADIAN / (SECOND * SECOND * SECOND)).unit() }, BASE> =
    Unit::ONE;

/// An angular absement, in `N`s of the unit `U`: by default its base unit, `RADIAN_SECOND`.
pub type AngularAbsement<N, const U: SIUnit = { RADIAN_SECOND.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Radian seconds (`rad · s`), a unit of `AngularAbsement`: the coherent unit, a product of base units.
pub const RADIAN_SECOND: Unit<{ (SECOND * RADIAN).unit() }, BASE> = Unit::ONE;

/// Degree seconds (`° · s`), a unit of `AngularAbsement`: `n` of them is `n · core::f64::consts::PI / 180.0` base units.
pub const DEGREE_SECOND: Unit<
    { SIUnit::scaled("° · s", Exact::PI / Exact::int(180), RADIAN_SECOND.unit()) },
> = Unit::ONE;

/// An angular momentum, in `N`s of the unit `U`: by default its base unit, `NEWTON_METRE_SECOND`.
pub type AngularMomentum<
    N,
    const U: SIUnit = { NEWTON_METRE_SECOND.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Newton metre seconds (`N · m · s`), a unit of `AngularMomentum`: the coherent unit, a product of base units.
pub const NEWTON_METRE_SECOND: Unit<
    { (METRE * METRE * KILOGRAM / (SECOND * RADIAN)).unit() },
    BASE,
> = Unit::ONE;

/// Reduced Planck constants (`ħ`), a unit of `AngularMomentum`: `n` of them is `n · 6.62607015e-34 / (2.0 * core::f64::consts::PI)` base units.
pub const REDUCED_PLANCK_CONSTANT: Unit<
    {
        SIUnit::scaled(
            "ħ",
            Exact::dec(662607015, -42) / (Exact::int(2) * Exact::PI),
            NEWTON_METRE_SECOND.unit(),
        )
    },
> = Unit::ONE;
