use crate::{ISQ, SIQuantity};
use yadc::Dimension;

/// The dimension of `Angle`.
pub const ANGLE: ISQ = ISQ {
    rad: 1,
    ..ISQ::DIMENSIONLESS
};
pub type Angle<N> = SIQuantity<ANGLE, N>;

/// The dimension of `SolidAngle`.
pub const SOLID_ANGLE: ISQ = ISQ {
    rad: 2,
    ..ISQ::DIMENSIONLESS
};
pub type SolidAngle<N> = SIQuantity<SOLID_ANGLE, N>;

/// The dimension of `AngularVelocity`.
pub const ANGULAR_VELOCITY: ISQ = ISQ {
    s: -1,
    rad: 1,
    ..ISQ::DIMENSIONLESS
};
pub type AngularVelocity<N> = SIQuantity<ANGULAR_VELOCITY, N>;

/// The dimension of `AngularAcceleration`.
pub const ANGULAR_ACCELERATION: ISQ = ISQ {
    s: -2,
    rad: 1,
    ..ISQ::DIMENSIONLESS
};
pub type AngularAcceleration<N> = SIQuantity<ANGULAR_ACCELERATION, N>;

/// The dimension of `AngularJerk`.
pub const ANGULAR_JERK: ISQ = ISQ {
    s: -3,
    rad: 1,
    ..ISQ::DIMENSIONLESS
};
pub type AngularJerk<N> = SIQuantity<ANGULAR_JERK, N>;

/// The dimension of `AngularAbsement`.
pub const ANGULAR_ABSEMENT: ISQ = ISQ {
    s: 1,
    rad: 1,
    ..ISQ::DIMENSIONLESS
};
pub type AngularAbsement<N> = SIQuantity<ANGULAR_ABSEMENT, N>;

/// The dimension of `AngularMomentum`.
pub const ANGULAR_MOMENTUM: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -1,
    rad: -1,
    ..ISQ::DIMENSIONLESS
};
pub type AngularMomentum<N> = SIQuantity<ANGULAR_MOMENTUM, N>;

/// The units of this module's quantities: a type each, its size and zero
/// in its quantity's coherent unit, exact (`Mile {}.quantity(3.0_f64)`).
pub mod units {
    use super::*;
    use crate::{Exact, SIUnit, SIUnitConsts};

    /// Radians (`rad`), a unit of `Angle`: `n` of them is `n · 1.0` coherent units.
    pub type Radian = SIUnit<
        ANGLE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "rad",
                name: "radians",
            }
        },
    >;

    /// Revolutions (`r`), a unit of `Angle`: `n` of them is `n · 2.0 * core::f64::consts::PI` coherent units.
    pub type Revolution = SIUnit<
        ANGLE,
        {
            SIUnitConsts {
                size: Exact::int(2) * Exact::PI,
                zero: Exact::ZERO,
                symbol: "r",
                name: "revolutions",
            }
        },
    >;

    /// Degrees (`°`), a unit of `Angle`: `n` of them is `n · core::f64::consts::PI / 180.0` coherent units.
    pub type Degree = SIUnit<
        ANGLE,
        {
            SIUnitConsts {
                size: Exact::PI / Exact::int(180),
                zero: Exact::ZERO,
                symbol: "°",
                name: "degrees",
            }
        },
    >;

    /// Gons (`gon`), a unit of `Angle`: `n` of them is `n · core::f64::consts::PI / 200.0` coherent units.
    pub type Gon = SIUnit<
        ANGLE,
        {
            SIUnitConsts {
                size: Exact::PI / Exact::int(200),
                zero: Exact::ZERO,
                symbol: "gon",
                name: "gons",
            }
        },
    >;

    /// Mils (`mil`), a unit of `Angle`: `n` of them is `n · 2.0 * core::f64::consts::PI / 6400.0` coherent units.
    pub type Mil = SIUnit<
        ANGLE,
        {
            SIUnitConsts {
                size: Exact::int(2) * Exact::PI / Exact::int(6400),
                zero: Exact::ZERO,
                symbol: "mil",
                name: "mils",
            }
        },
    >;

    /// Minutes (`′`), a unit of `Angle`: `n` of them is `n · core::f64::consts::PI / 10800.0` coherent units.
    pub type Minute = SIUnit<
        ANGLE,
        {
            SIUnitConsts {
                size: Exact::PI / Exact::int(10800),
                zero: Exact::ZERO,
                symbol: "′",
                name: "minutes",
            }
        },
    >;

    /// Seconds (`″`), a unit of `Angle`: `n` of them is `n · core::f64::consts::PI / 648000.0` coherent units.
    pub type Second = SIUnit<
        ANGLE,
        {
            SIUnitConsts {
                size: Exact::PI / Exact::int(648000),
                zero: Exact::ZERO,
                symbol: "″",
                name: "seconds",
            }
        },
    >;

    /// Steradians (`sr`), a unit of `SolidAngle`: `n` of them is `n · 1.0` coherent units.
    pub type Steradian = SIUnit<
        SOLID_ANGLE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "sr",
                name: "steradians",
            }
        },
    >;

    /// Spats (`sp`), a unit of `SolidAngle`: `n` of them is `n · 4.0 * core::f64::consts::PI` coherent units.
    pub type Spat = SIUnit<
        SOLID_ANGLE,
        {
            SIUnitConsts {
                size: Exact::int(4) * Exact::PI,
                zero: Exact::ZERO,
                symbol: "sp",
                name: "spats",
            }
        },
    >;

    /// Square degrees (`°²`), a unit of `SolidAngle`: `n` of them is `n · (core::f64::consts::PI / 180.0) * (core::f64::consts::PI / 180.0)` coherent units.
    pub type SquareDegree = SIUnit<
        SOLID_ANGLE,
        {
            SIUnitConsts {
                size: (Exact::PI / Exact::int(180)) * (Exact::PI / Exact::int(180)),
                zero: Exact::ZERO,
                symbol: "°²",
                name: "square degrees",
            }
        },
    >;

    /// Square minutes (`′²`), a unit of `SolidAngle`: `n` of them is `n · (core::f64::consts::PI / 10800.0) * (core::f64::consts::PI / 10800.0)` coherent units.
    pub type SquareMinute = SIUnit<
        SOLID_ANGLE,
        {
            SIUnitConsts {
                size: (Exact::PI / Exact::int(10800)) * (Exact::PI / Exact::int(10800)),
                zero: Exact::ZERO,
                symbol: "′²",
                name: "square minutes",
            }
        },
    >;

    /// Square seconds (`″²`), a unit of `SolidAngle`: `n` of them is `n · (core::f64::consts::PI / 648000.0) * (core::f64::consts::PI / 648000.0)` coherent units.
    pub type SquareSecond = SIUnit<
        SOLID_ANGLE,
        {
            SIUnitConsts {
                size: (Exact::PI / Exact::int(648000)) * (Exact::PI / Exact::int(648000)),
                zero: Exact::ZERO,
                symbol: "″²",
                name: "square seconds",
            }
        },
    >;

    /// Radians per second (`rad/s`), a unit of `AngularVelocity`: `n` of them is `n · 1.0` coherent units.
    pub type RadianPerSecond = SIUnit<
        ANGULAR_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "rad/s",
                name: "radians per second",
            }
        },
    >;

    /// Degrees per second (`°/s`), a unit of `AngularVelocity`: `n` of them is `n · core::f64::consts::PI / 180.0` coherent units.
    pub type DegreePerSecond = SIUnit<
        ANGULAR_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::PI / Exact::int(180),
                zero: Exact::ZERO,
                symbol: "°/s",
                name: "degrees per second",
            }
        },
    >;

    /// Revolutions per second (`rps`), a unit of `AngularVelocity`: `n` of them is `n · 2.0 * core::f64::consts::PI` coherent units.
    pub type RevolutionPerSecond = SIUnit<
        ANGULAR_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(2) * Exact::PI,
                zero: Exact::ZERO,
                symbol: "rps",
                name: "revolutions per second",
            }
        },
    >;

    /// Revolutions per minute (`rpm`), a unit of `AngularVelocity`: `n` of them is `n · 2.0 * core::f64::consts::PI / 60.0` coherent units.
    pub type RevolutionPerMinute = SIUnit<
        ANGULAR_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(2) * Exact::PI / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "rpm",
                name: "revolutions per minute",
            }
        },
    >;

    /// Revolutions per hour (`rph`), a unit of `AngularVelocity`: `n` of them is `n · 2.0 * core::f64::consts::PI / 3600.0` coherent units.
    pub type RevolutionPerHour = SIUnit<
        ANGULAR_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(2) * Exact::PI / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "rph",
                name: "revolutions per hour",
            }
        },
    >;

    /// Radians per second squared (`rad/s²`), a unit of `AngularAcceleration`: `n` of them is `n · 1.0` coherent units.
    pub type RadianPerSecondSquared = SIUnit<
        ANGULAR_ACCELERATION,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "rad/s²",
                name: "radians per second squared",
            }
        },
    >;

    /// Degrees per second squared (`°/s²`), a unit of `AngularAcceleration`: `n` of them is `n · core::f64::consts::PI / 180.0` coherent units.
    pub type DegreePerSecondSquared = SIUnit<
        ANGULAR_ACCELERATION,
        {
            SIUnitConsts {
                size: Exact::PI / Exact::int(180),
                zero: Exact::ZERO,
                symbol: "°/s²",
                name: "degrees per second squared",
            }
        },
    >;

    /// Radians per second cubed (`rad/s³`), a unit of `AngularJerk`: `n` of them is `n · 1.0` coherent units.
    pub type RadianPerSecondCubed = SIUnit<
        ANGULAR_JERK,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "rad/s³",
                name: "radians per second cubed",
            }
        },
    >;

    /// Degrees per second cubed (`°/s³`), a unit of `AngularJerk`: `n` of them is `n · core::f64::consts::PI / 180.0` coherent units.
    pub type DegreePerSecondCubed = SIUnit<
        ANGULAR_JERK,
        {
            SIUnitConsts {
                size: Exact::PI / Exact::int(180),
                zero: Exact::ZERO,
                symbol: "°/s³",
                name: "degrees per second cubed",
            }
        },
    >;

    /// Radian seconds (`rad · s`), a unit of `AngularAbsement`: `n` of them is `n · 1.0` coherent units.
    pub type RadianSecond = SIUnit<
        ANGULAR_ABSEMENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "rad · s",
                name: "radian seconds",
            }
        },
    >;

    /// Degree seconds (`° · s`), a unit of `AngularAbsement`: `n` of them is `n · core::f64::consts::PI / 180.0` coherent units.
    pub type DegreeSecond = SIUnit<
        ANGULAR_ABSEMENT,
        {
            SIUnitConsts {
                size: Exact::PI / Exact::int(180),
                zero: Exact::ZERO,
                symbol: "° · s",
                name: "degree seconds",
            }
        },
    >;

    /// Reduced Planck constants (`ħ`), a unit of `AngularMomentum`: `n` of them is `n · 6.62607015e-34 / (2.0 * core::f64::consts::PI)` coherent units.
    pub type ReducedPlanckConstant = SIUnit<
        ANGULAR_MOMENTUM,
        {
            SIUnitConsts {
                size: Exact::dec(662607015, -42) / (Exact::int(2) * Exact::PI),
                zero: Exact::ZERO,
                symbol: "ħ",
                name: "reduced Planck constants",
            }
        },
    >;

    /// Kilogram square metres per second (`kg · m²/s`), a unit of `AngularMomentum`: `n` of them is `n · 1.0` coherent units.
    pub type KilogramSquareMetrePerSecond = SIUnit<
        ANGULAR_MOMENTUM,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg · m²/s",
                name: "kilogram square metres per second",
            }
        },
    >;

    /// Newton metre seconds (`N · m · s`), a unit of `AngularMomentum`: `n` of them is `n · 1.0` coherent units.
    pub type NewtonMetreSecond = SIUnit<
        ANGULAR_MOMENTUM,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "N · m · s",
                name: "newton metre seconds",
            }
        },
    >;
}
