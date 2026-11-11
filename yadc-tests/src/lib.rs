//! A small unit system, defined here and used by the tests: the
//! compile-fail tests in `tests/ui/use` (the "system defined in another
//! crate" case - those in `tests/ui/define` define their own) and the
//! run-time tests of the derived impls in `tests/derived.rs`.
#![feature(generic_const_exprs, min_adt_const_params, const_trait_impl)]
#![feature(fn_traits, unboxed_closures, const_destruct)]
#![feature(
    const_cmp,
    const_default,
    derive_const,
    const_ops,
    const_convert,
    const_try
)]
#![allow(incomplete_features)]

use core::marker::{ConstParamTy, Destruct};
use core::ops::{Add, Sub};
use yadc::{Dimension, Unit};

/// The kinds: how a quantity behaves. `Linear` is an ordinary amount;
/// `Affine` a point - measured from a zero somebody chose; `Log` a logarithm
/// - a level or a gain, in decibels. `Rank` is a label of this system's own,
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

/// The dimension. Its kind field names the two kinds with a behaviour of
/// their own.
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

/// A scale: a zero and a step. A unit of the zero's type - `Scale(ICE_POINT,
/// KELVIN)` counts kelvins from the ice point: `n` is `zero + n · step`, and a
/// value reads `(value - zero) / step`.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Scale<P, S>(pub P, pub S);

const impl<P, S, N> Unit<P, N> for Scale<P, S>
where
    P: Copy + [const] Sub + [const] Add<<P as Sub>::Output, Output = P>,
    S: [const] Unit<<P as Sub>::Output, N>,
{
    fn quantity(&self, value: N) -> P {
        self.0 + self.1.quantity(value)
    }
    fn value(&self, quantity: P) -> N {
        self.1.value(quantity - self.0)
    }
}

/// A scale is called like any unit: on a number it builds (`CELSIUS(25.0)`),
/// on a value it reads (`CELSIUS(t)`). Std's three function traits, once for
/// each direction.
macro_rules! scale_is_called {
    ($arg:ty => $out:ty, $way:ident) => {
        const impl<const D: Dim, S> FnOnce<($arg,)> for Scale<Qty<D>, S>
        where
            Self: [const] Unit<Qty<D>, f64> + [const] Destruct,
        {
            type Output = $out;
            extern "rust-call" fn call_once(self, (x,): ($arg,)) -> $out {
                Unit::$way(&self, x)
            }
        }
        const impl<const D: Dim, S> FnMut<($arg,)> for Scale<Qty<D>, S>
        where
            Self: [const] Unit<Qty<D>, f64> + [const] Destruct,
        {
            extern "rust-call" fn call_mut(&mut self, (x,): ($arg,)) -> $out {
                Unit::$way(&*self, x)
            }
        }
        const impl<const D: Dim, S> Fn<($arg,)> for Scale<Qty<D>, S>
        where
            Self: [const] Unit<Qty<D>, f64> + [const] Destruct,
        {
            extern "rust-call" fn call(&self, (x,): ($arg,)) -> $out {
                Unit::$way(self, x)
            }
        }
    };
}
scale_is_called!(f64 => Qty<D>, quantity);
scale_is_called!(Qty<D> => f64, value);

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

/// A scale: a zero and a step.
pub const CELSIUS: Scale<Temperature, TemperatureStep> = Scale(ICE_POINT, KELVIN);

/// A scale of points: feet above mean sea level.
pub const FEET_AMSL: Scale<Altitude, Length> = Scale(SEA_LEVEL, FOOT);

/// A level: a zero and a step on the logarithmic line - a unit of
/// `PowerLevel`, and of `Power` through the conversion.
pub const DBM: Scale<PowerLevel, Gain> = Scale(MILLIWATT_LEVEL, DECIBEL);

/// A choice of units, for a menu: each variant names the unit value it is.
#[derive(Copy, Clone, Debug, PartialEq, yadc::Unit)]
#[unit(of = Temperature)]
pub enum TemperatureUnit {
    #[unit("K", "kelvin", by = Scale(ABSOLUTE_ZERO, KELVIN))]
    Kelvin,
    #[unit("°C", "degrees Celsius", by = CELSIUS)]
    Celsius,
    #[unit("°F", "degrees Fahrenheit", by = Scale(FAHRENHEIT_ZERO, DEGREE_FAHRENHEIT))]
    Fahrenheit,
}

/// Units without labels: `impl()` writes `Unit` alone.
#[derive(Copy, Clone, Debug, yadc::Unit)]
#[unit(of = Pressure, impl())]
pub enum PressureUnit {
    #[unit(by = KILOPASCAL)]
    Kilopascal,
    #[unit(by = PASCAL)]
    Pascal,
}

/// Units of power, a level among them: `via` names the type the scale
/// measures, and std `From` takes it to the watts and back.
#[derive(Copy, Clone, Debug, PartialEq, yadc::Unit)]
#[unit(of = Power, non_const)]
pub enum PowerUnit {
    #[unit("W", "watts", by = WATT)]
    Watt,
    #[unit("dBm", "decibel-milliwatts", by = DBM, via = PowerLevel)]
    DecibelMilliwatt,
}

/// A unit that holds the unit value it is: known at run time.
#[derive(Copy, Clone, Debug, yadc::Unit)]
#[unit("ft AGL", "feet above ground level", of = Altitude)]
pub struct FeetAboveGround {
    #[unit(by)]
    pub scale: Scale<Altitude, Length>,
}
