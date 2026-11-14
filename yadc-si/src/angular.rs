use crate::ISQ;
use yadc::Dimension;

/// The dimension of `Angle`: `SIQuantity<ANGLE, N>` is one.
pub const ANGLE: ISQ = ISQ {
    rad: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `SolidAngle`: `SIQuantity<SOLID_ANGLE, N>` is one.
pub const SOLID_ANGLE: ISQ = ISQ {
    rad: 2,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `AngularVelocity`: `SIQuantity<ANGULAR_VELOCITY, N>` is one.
pub const ANGULAR_VELOCITY: ISQ = ISQ {
    s: -1,
    rad: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `AngularAcceleration`: `SIQuantity<ANGULAR_ACCELERATION, N>` is one.
pub const ANGULAR_ACCELERATION: ISQ = ISQ {
    s: -2,
    rad: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `AngularJerk`: `SIQuantity<ANGULAR_JERK, N>` is one.
pub const ANGULAR_JERK: ISQ = ISQ {
    s: -3,
    rad: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `AngularAbsement`: `SIQuantity<ANGULAR_ABSEMENT, N>` is one.
pub const ANGULAR_ABSEMENT: ISQ = ISQ {
    s: 1,
    rad: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `AngularMomentum`: `SIQuantity<ANGULAR_MOMENTUM, N>` is one.
pub const ANGULAR_MOMENTUM: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -1,
    rad: -1,
    ..ISQ::DIMENSIONLESS
};

/// The units of this module's quantities: each a constant of its own unit
/// type, its size and zero in its quantity's coherent unit, exact
/// (`MILE.quantity(3.0)`, `MILE.value(d)`).
pub mod units {
    use super::*;
    use crate::{Exact, SIUnit, SIUnitConsts};

    /// Radians (`rad`), a unit of `Angle`: `n` of them is `n · 1.0` coherent units.
    pub const RADIAN: SIUnit<
        ANGLE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "rad",
                name: "radians",
            }
        },
    > = SIUnit;

    /// Revolutions (`r`), a unit of `Angle`: `n` of them is `n · 2.0 * core::f64::consts::PI` coherent units.
    pub const REVOLUTION: SIUnit<
        ANGLE,
        {
            SIUnitConsts {
                size: Exact::int(2) * Exact::PI,
                zero: Exact::ZERO,
                symbol: "r",
                name: "revolutions",
            }
        },
    > = SIUnit;

    /// Degrees (`°`), a unit of `Angle`: `n` of them is `n · core::f64::consts::PI / 180.0` coherent units.
    pub const DEGREE: SIUnit<
        ANGLE,
        {
            SIUnitConsts {
                size: Exact::PI / Exact::int(180),
                zero: Exact::ZERO,
                symbol: "°",
                name: "degrees",
            }
        },
    > = SIUnit;

    /// Gons (`gon`), a unit of `Angle`: `n` of them is `n · core::f64::consts::PI / 200.0` coherent units.
    pub const GON: SIUnit<
        ANGLE,
        {
            SIUnitConsts {
                size: Exact::PI / Exact::int(200),
                zero: Exact::ZERO,
                symbol: "gon",
                name: "gons",
            }
        },
    > = SIUnit;

    /// Mils (`mil`), a unit of `Angle`: `n` of them is `n · 2.0 * core::f64::consts::PI / 6400.0` coherent units.
    pub const MIL: SIUnit<
        ANGLE,
        {
            SIUnitConsts {
                size: Exact::int(2) * Exact::PI / Exact::int(6400),
                zero: Exact::ZERO,
                symbol: "mil",
                name: "mils",
            }
        },
    > = SIUnit;

    /// Minutes (`′`), a unit of `Angle`: `n` of them is `n · core::f64::consts::PI / 10800.0` coherent units.
    pub const MINUTE: SIUnit<
        ANGLE,
        {
            SIUnitConsts {
                size: Exact::PI / Exact::int(10800),
                zero: Exact::ZERO,
                symbol: "′",
                name: "minutes",
            }
        },
    > = SIUnit;

    /// Seconds (`″`), a unit of `Angle`: `n` of them is `n · core::f64::consts::PI / 648000.0` coherent units.
    pub const SECOND: SIUnit<
        ANGLE,
        {
            SIUnitConsts {
                size: Exact::PI / Exact::int(648000),
                zero: Exact::ZERO,
                symbol: "″",
                name: "seconds",
            }
        },
    > = SIUnit;

    /// Steradians (`sr`), a unit of `SolidAngle`: `n` of them is `n · 1.0` coherent units.
    pub const STERADIAN: SIUnit<
        SOLID_ANGLE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "sr",
                name: "steradians",
            }
        },
    > = SIUnit;

    /// Spats (`sp`), a unit of `SolidAngle`: `n` of them is `n · 4.0 * core::f64::consts::PI` coherent units.
    pub const SPAT: SIUnit<
        SOLID_ANGLE,
        {
            SIUnitConsts {
                size: Exact::int(4) * Exact::PI,
                zero: Exact::ZERO,
                symbol: "sp",
                name: "spats",
            }
        },
    > = SIUnit;

    /// Square degrees (`°²`), a unit of `SolidAngle`: `n` of them is `n · (core::f64::consts::PI / 180.0) * (core::f64::consts::PI / 180.0)` coherent units.
    pub const SQUARE_DEGREE: SIUnit<
        SOLID_ANGLE,
        {
            SIUnitConsts {
                size: (Exact::PI / Exact::int(180)) * (Exact::PI / Exact::int(180)),
                zero: Exact::ZERO,
                symbol: "°²",
                name: "square degrees",
            }
        },
    > = SIUnit;

    /// Square minutes (`′²`), a unit of `SolidAngle`: `n` of them is `n · (core::f64::consts::PI / 10800.0) * (core::f64::consts::PI / 10800.0)` coherent units.
    pub const SQUARE_MINUTE: SIUnit<
        SOLID_ANGLE,
        {
            SIUnitConsts {
                size: (Exact::PI / Exact::int(10800)) * (Exact::PI / Exact::int(10800)),
                zero: Exact::ZERO,
                symbol: "′²",
                name: "square minutes",
            }
        },
    > = SIUnit;

    /// Square seconds (`″²`), a unit of `SolidAngle`: `n` of them is `n · (core::f64::consts::PI / 648000.0) * (core::f64::consts::PI / 648000.0)` coherent units.
    pub const SQUARE_SECOND: SIUnit<
        SOLID_ANGLE,
        {
            SIUnitConsts {
                size: (Exact::PI / Exact::int(648000)) * (Exact::PI / Exact::int(648000)),
                zero: Exact::ZERO,
                symbol: "″²",
                name: "square seconds",
            }
        },
    > = SIUnit;

    /// Radians per second (`rad/s`), a unit of `AngularVelocity`: `n` of them is `n · 1.0` coherent units.
    pub const RADIAN_PER_SECOND: SIUnit<
        ANGULAR_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "rad/s",
                name: "radians per second",
            }
        },
    > = SIUnit;

    /// Degrees per second (`°/s`), a unit of `AngularVelocity`: `n` of them is `n · core::f64::consts::PI / 180.0` coherent units.
    pub const DEGREE_PER_SECOND: SIUnit<
        ANGULAR_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::PI / Exact::int(180),
                zero: Exact::ZERO,
                symbol: "°/s",
                name: "degrees per second",
            }
        },
    > = SIUnit;

    /// Revolutions per second (`rps`), a unit of `AngularVelocity`: `n` of them is `n · 2.0 * core::f64::consts::PI` coherent units.
    pub const REVOLUTION_PER_SECOND: SIUnit<
        ANGULAR_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(2) * Exact::PI,
                zero: Exact::ZERO,
                symbol: "rps",
                name: "revolutions per second",
            }
        },
    > = SIUnit;

    /// Revolutions per minute (`rpm`), a unit of `AngularVelocity`: `n` of them is `n · 2.0 * core::f64::consts::PI / 60.0` coherent units.
    pub const REVOLUTION_PER_MINUTE: SIUnit<
        ANGULAR_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(2) * Exact::PI / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "rpm",
                name: "revolutions per minute",
            }
        },
    > = SIUnit;

    /// Revolutions per hour (`rph`), a unit of `AngularVelocity`: `n` of them is `n · 2.0 * core::f64::consts::PI / 3600.0` coherent units.
    pub const REVOLUTION_PER_HOUR: SIUnit<
        ANGULAR_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(2) * Exact::PI / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "rph",
                name: "revolutions per hour",
            }
        },
    > = SIUnit;

    /// Radians per second squared (`rad/s²`), a unit of `AngularAcceleration`: `n` of them is `n · 1.0` coherent units.
    pub const RADIAN_PER_SECOND_SQUARED: SIUnit<
        ANGULAR_ACCELERATION,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "rad/s²",
                name: "radians per second squared",
            }
        },
    > = SIUnit;

    /// Degrees per second squared (`°/s²`), a unit of `AngularAcceleration`: `n` of them is `n · core::f64::consts::PI / 180.0` coherent units.
    pub const DEGREE_PER_SECOND_SQUARED: SIUnit<
        ANGULAR_ACCELERATION,
        {
            SIUnitConsts {
                size: Exact::PI / Exact::int(180),
                zero: Exact::ZERO,
                symbol: "°/s²",
                name: "degrees per second squared",
            }
        },
    > = SIUnit;

    /// Radians per second cubed (`rad/s³`), a unit of `AngularJerk`: `n` of them is `n · 1.0` coherent units.
    pub const RADIAN_PER_SECOND_CUBED: SIUnit<
        ANGULAR_JERK,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "rad/s³",
                name: "radians per second cubed",
            }
        },
    > = SIUnit;

    /// Degrees per second cubed (`°/s³`), a unit of `AngularJerk`: `n` of them is `n · core::f64::consts::PI / 180.0` coherent units.
    pub const DEGREE_PER_SECOND_CUBED: SIUnit<
        ANGULAR_JERK,
        {
            SIUnitConsts {
                size: Exact::PI / Exact::int(180),
                zero: Exact::ZERO,
                symbol: "°/s³",
                name: "degrees per second cubed",
            }
        },
    > = SIUnit;

    /// Radian seconds (`rad · s`), a unit of `AngularAbsement`: `n` of them is `n · 1.0` coherent units.
    pub const RADIAN_SECOND: SIUnit<
        ANGULAR_ABSEMENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "rad · s",
                name: "radian seconds",
            }
        },
    > = SIUnit;

    /// Degree seconds (`° · s`), a unit of `AngularAbsement`: `n` of them is `n · core::f64::consts::PI / 180.0` coherent units.
    pub const DEGREE_SECOND: SIUnit<
        ANGULAR_ABSEMENT,
        {
            SIUnitConsts {
                size: Exact::PI / Exact::int(180),
                zero: Exact::ZERO,
                symbol: "° · s",
                name: "degree seconds",
            }
        },
    > = SIUnit;

    /// Reduced Planck constants (`ħ`), a unit of `AngularMomentum`: `n` of them is `n · 6.62607015e-34 / (2.0 * core::f64::consts::PI)` coherent units.
    pub const REDUCED_PLANCK_CONSTANT: SIUnit<
        ANGULAR_MOMENTUM,
        {
            SIUnitConsts {
                size: Exact::dec(662607015, -42) / (Exact::int(2) * Exact::PI),
                zero: Exact::ZERO,
                symbol: "ħ",
                name: "reduced Planck constants",
            }
        },
    > = SIUnit;

    /// Kilogram square metres per second (`kg · m²/s`), a unit of `AngularMomentum`: `n` of them is `n · 1.0` coherent units.
    pub const KILOGRAM_SQUARE_METRE_PER_SECOND: SIUnit<
        ANGULAR_MOMENTUM,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg · m²/s",
                name: "kilogram square metres per second",
            }
        },
    > = SIUnit;

    /// Newton metre seconds (`N · m · s`), a unit of `AngularMomentum`: `n` of them is `n · 1.0` coherent units.
    pub const NEWTON_METRE_SECOND: SIUnit<
        ANGULAR_MOMENTUM,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "N · m · s",
                name: "newton metre seconds",
            }
        },
    > = SIUnit;
}
