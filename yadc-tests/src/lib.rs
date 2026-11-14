//! A small unit system, defined here and used by the tests: the
//! compile-fail tests in `tests/ui/use` (the "system defined in another
//! crate" case - those in `tests/ui/define` define their own) and the
//! run-time tests of the derived impls in `tests/derived.rs`. Its units are
//! types with a hand-written `impl Unit`, and enums of them
//! (`dynamickit::Values`).
#![feature(generic_const_exprs, min_adt_const_params, const_trait_impl)]
#![feature(
    const_cmp,
    const_default,
    derive_const,
    const_ops,
    const_convert,
    const_try
)]
#![allow(incomplete_features)]

use core::fmt;
use core::marker::ConstParamTy;
use yadc::{Dimension, Unit};

/// The kinds: how a quantity behaves. `Linear` is an ordinary amount;
/// `Affine` a point, measured from a zero somebody chose; `Log` a logarithm,
/// a level or a gain, in decibels. `Rank` is a label of this system's own,
/// with a rule: it cannot be added to itself.
#[derive_const(PartialEq, Default)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
pub enum Kind {
    #[default]
    Linear,
    #[kind(affine)]
    Affine,
    #[kind(log)]
    Log,
    #[kind(add(Rank) = invalid)]
    Rank,
}

/// The dimension. The kind field is marked `#[kind]`.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
pub struct Dim {
    pub kg: i8,
    pub m: i8,
    pub s: i8,
    pub k: i8,
    #[kind]
    pub kind: Kind,
}

/// The family.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::Quantity)]
pub struct Qty<const D: Dim>(f64);

pub type Number = Qty<{ Dim::DIMENSIONLESS }>;
pub type Mass = Qty<
    {
        Dim {
            kg: 1,
            ..Dim::DIMENSIONLESS
        }
    },
>;
pub type Pressure = Qty<
    {
        Dim {
            kg: 1,
            m: -1,
            s: -2,
            ..Dim::DIMENSIONLESS
        }
    },
>;
pub type Power = Qty<
    {
        Dim {
            kg: 1,
            m: 2,
            s: -3,
            ..Dim::DIMENSIONLESS
        }
    },
>;
pub type Length = Qty<
    {
        Dim {
            m: 1,
            ..Dim::DIMENSIONLESS
        }
    },
>;
pub type Time = Qty<
    {
        Dim {
            s: 1,
            ..Dim::DIMENSIONLESS
        }
    },
>;
pub type Rank = Qty<
    {
        Dim {
            kind: Kind::Rank,
            ..Dim::DIMENSIONLESS
        }
    },
>;
pub type Temperature = Qty<
    {
        Dim {
            k: 1,
            kind: Kind::Affine,
            ..Dim::DIMENSIONLESS
        }
    },
>;
pub type TemperatureStep = Qty<
    {
        Dim {
            k: 1,
            ..Dim::DIMENSIONLESS
        }
    },
>;
/// A height above sea level: a point on the line of lengths.
pub type Altitude = Qty<
    {
        Dim {
            m: 1,
            kind: Kind::Affine,
            ..Dim::DIMENSIONLESS
        }
    },
>;
/// An instant: a point on the line of times.
pub type Instant = Qty<
    {
        Dim {
            s: 1,
            kind: Kind::Affine,
            ..Dim::DIMENSIONLESS
        }
    },
>;
/// A gain: the logarithm of a plain ratio. Gains add where ratios multiply.
pub type Gain = Qty<
    {
        Dim {
            kind: Kind::Log,
            ..Dim::DIMENSIONLESS
        }
    },
>;
/// A power level: the logarithm of a power. Two levels differ by a gain (the
/// logarithm of watts over watts), and a gain moves a level.
pub type PowerLevel = Qty<
    {
        Dim {
            kg: 1,
            m: 2,
            s: -3,
            kind: Kind::Log,
            ..Dim::DIMENSIONLESS
        }
    },
>;

pub const ONE: Number = Qty(1.0);
pub const METRE: Length = Qty(1.0);
pub const SECOND: Time = Qty(1.0);
pub const KELVIN: TemperatureStep = Qty(1.0);
pub const ABSOLUTE_ZERO: Temperature = Qty(0.0);
pub const SEA_LEVEL: Altitude = Qty(0.0);
pub const UNIX_EPOCH: Instant = Qty(0.0);
pub const FIRST: Rank = Qty(1.0);
pub const KILOGRAM: Mass = Qty(1.0);
pub const FOOT: Length = 0.3048 * METRE;
pub const PASCAL: Pressure = KILOGRAM / (METRE * SECOND * SECOND);
pub const KILOPASCAL: Pressure = 1000.0 * PASCAL;
pub const WATT: Power = KILOGRAM * METRE * METRE / (SECOND * SECOND * SECOND);
pub const MILLIWATT: Power = WATT / 1000.0;
pub const DEGREE_FAHRENHEIT: TemperatureStep = KELVIN * (5.0 / 9.0);
pub const ICE_POINT: Temperature = ABSOLUTE_ZERO + 273.15 * KELVIN;
pub const FAHRENHEIT_ZERO: Temperature = ICE_POINT - 32.0 * DEGREE_FAHRENHEIT;

/// The bel: a gain of ten times. The base constant of the logarithmic line.
pub const BEL: Gain = Qty(1.0);
pub const DECIBEL: Gain = 0.1 * BEL;
/// The level of one watt: where the line's numbers start.
pub const WATT_LEVEL: PowerLevel = Qty(0.0);
pub const MILLIWATT_LEVEL: PowerLevel = WATT_LEVEL - 30.0 * DECIBEL;

/// The logarithmic line and the linear one: the only logarithm in the system.
impl From<PowerLevel> for Power {
    fn from(level: PowerLevel) -> Power {
        WATT * 10f64.powf(BEL.value(level - WATT_LEVEL))
    }
}
impl From<Power> for PowerLevel {
    fn from(power: Power) -> PowerLevel {
        WATT_LEVEL + BEL.quantity(WATT.value(power).log10())
    }
}

/// The GPS epoch, on the one line of instants: a zero like any other.
pub const GPS_EPOCH: Instant = UNIX_EPOCH + 315_964_800.0 * SECOND;

/// Writes a unit type of this system: `n` of it is `n` steps from a zero,
/// and it prints its symbol (`{}`) or name (`{:#}`).
macro_rules! unit {
    ($(#[$doc:meta])* $name:ident: $q:ty = $zero:expr, $step:expr, $symbol:literal, $long:literal) => {
        $(#[$doc])*
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct $name;
        impl const Unit<$q, f64> for $name {
            fn quantity(&self, value: f64) -> $q {
                $zero + $step.quantity(value)
            }
            fn value(&self, quantity: $q) -> f64 {
                $step.value(quantity - $zero)
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.pad(if f.alternate() { $long } else { $symbol })
            }
        }
    };
}

unit!(
    /// Kelvin, reading a temperature: from absolute zero.
    KelvinScale: Temperature = ABSOLUTE_ZERO, KELVIN, "K", "kelvin"
);
unit!(
    /// Degrees Celsius: kelvins from the ice point.
    Celsius: Temperature = ICE_POINT, KELVIN, "°C", "degrees Celsius"
);
unit!(
    /// Degrees Fahrenheit: steps of 5/9 K from the Fahrenheit zero.
    Fahrenheit: Temperature = FAHRENHEIT_ZERO, DEGREE_FAHRENHEIT, "°F", "degrees Fahrenheit"
);
unit!(
    /// Feet above mean sea level: a unit of points.
    FeetAmsl: Altitude = SEA_LEVEL, FOOT, "ft AMSL", "feet above mean sea level"
);
unit!(
    /// Decibels from one milliwatt: a unit on the logarithmic line.
    Dbm: PowerLevel = MILLIWATT_LEVEL, DECIBEL, "dBm", "decibel-milliwatts"
);
unit!(
    /// Decibels from one watt: another zero on the same line.
    Dbw: PowerLevel = WATT_LEVEL, DECIBEL, "dBW", "decibel-watts"
);

/// Pascals and kilopascals: unit types that do not print.
#[derive(Copy, Clone, Debug)]
pub struct Pascal;
#[derive(Copy, Clone, Debug)]
pub struct Kilopascal;

impl const Unit<Pressure, f64> for Pascal {
    fn quantity(&self, value: f64) -> Pressure {
        PASCAL.quantity(value)
    }
    fn value(&self, quantity: Pressure) -> f64 {
        PASCAL.value(quantity)
    }
}
impl const Unit<Pressure, f64> for Kilopascal {
    fn quantity(&self, value: f64) -> Pressure {
        KILOPASCAL.quantity(value)
    }
    fn value(&self, quantity: Pressure) -> f64 {
        KILOPASCAL.value(quantity)
    }
}

/// A choice of units, for a menu: each variant names the unit it is.
#[derive(Copy, Clone, Debug, PartialEq, dynamickit::Values)]
#[values(delegate(Unit<Temperature, f64> {
    fn quantity(&self, value: f64) -> Temperature;
    fn value(&self, quantity: Temperature) -> f64;
}))]
pub enum TemperatureUnit {
    #[value(KelvinScale)]
    Kelvin,
    #[value(Celsius)]
    Celsius,
    #[value(Fahrenheit)]
    Fahrenheit,
}

/// Units that do not print: `impl()` writes `Unit` alone.
#[derive(Copy, Clone, Debug, dynamickit::Values)]
#[values(
    delegate(Unit<Pressure, f64> {
        fn quantity(&self, value: f64) -> Pressure;
        fn value(&self, quantity: Pressure) -> f64;
    }),
    impl()
)]
pub enum PressureUnit {
    #[value(Kilopascal)]
    Kilopascal,
    #[value(Pascal)]
    Pascal,
}

/// Decibel-milliwatts as a unit of `Power`: through the level, and std
/// `From` to the watts and back. A logarithm is not `const`, so neither is
/// this impl.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DbmPower;

impl Unit<Power, f64> for DbmPower {
    fn quantity(&self, value: f64) -> Power {
        Dbm.quantity(value).into()
    }
    fn value(&self, quantity: Power) -> f64 {
        Dbm.value(quantity.into())
    }
}

/// Feet above a ground level known at run time: a unit that holds its zero.
#[derive(Copy, Clone, Debug)]
pub struct FeetAboveGround {
    pub ground: Altitude,
}

impl const Unit<Altitude, f64> for FeetAboveGround {
    fn quantity(&self, value: f64) -> Altitude {
        self.ground + FOOT.quantity(value)
    }
    fn value(&self, quantity: Altitude) -> f64 {
        FOOT.value(quantity - self.ground)
    }
}

impl fmt::Display for FeetAboveGround {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(if f.alternate() {
            "feet above ground level"
        } else {
            "ft AGL"
        })
    }
}
