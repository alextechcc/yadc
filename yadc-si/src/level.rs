//! [`Power`] <-> [`PowerLevel`], and a plain ratio <-> [`Gain`]: hand-written,
//! not generated.
//!
//! Decibels count on a logarithmic line, where multiplying is adding: a gain
//! of 3 dB and one of 10 dB make 13 dB, where the ratios 2 and 10 make 20.
//! Both quantities on the line have the logarithmic kind, whose `+` and `-`
//! add and subtract the exponents. A [`Gain`] is the logarithm of a plain
//! power ratio - no exponents - a quantity of its own with plain units (`3.0
//! * DECIBEL`). A [`PowerLevel`] is the logarithm of a power, and carries
//! power's exponents: two levels differ by a gain (watts over watts), a gain
//! moves a level, and two levels do not sum to a level. A level is read on a
//! scale, a zero and a step: [`DBM`](crate::mechanical::consts::DBM) is
//! `Scale(MILLIWATT_LEVEL, DECIBEL)`.
//!
//! The logarithm itself is here and nowhere else: the conversions between
//! the line and the linear quantities. A scale of levels measures levels, and
//! the watts are one std `From` away - `Power::from(DBM.quantity(30.0))` is one watt,
//! `DBM.value(PowerLevel::from(p))` reads it back - while `PowerUnit`'s decibel
//! variants do both steps (`via = PowerLevel`). Both directions read each
//! quantity's number in a unit - the exact constants, rounded to `f64` once,
//! here - and the float maths goes through [`Real`], so it works in `no_std`
//! builds with the `libm` feature.

use crate::dimensionless::{self, Dimensionless};
use crate::mechanical::{self, Gain, Power, PowerLevel};
use yadc::Unit;
use yadc::num_traits::real::Real;

const RATIO: Dimensionless<f64> = dimensionless::consts::RATIO.into();
const BEL: Gain<f64> = mechanical::consts::BEL.into();
const WATT: Power<f64> = mechanical::consts::WATT.into();
const WATT_LEVEL: PowerLevel<f64> = mechanical::consts::WATT_LEVEL.into();

/// The power at a level: a watt, ten times for every bel above
/// `WATT_LEVEL`.
impl From<PowerLevel<f64>> for Power<f64> {
    fn from(level: PowerLevel<f64>) -> Self {
        WATT.quantity(Real::powf(10.0, BEL.value(level - WATT_LEVEL)))
    }
}

/// The level of a power: `lg(power / 1 W)` bels above `WATT_LEVEL`. A zero
/// power is at minus infinity, and a negative one has no level (NaN), as for
/// `log10`.
impl From<Power<f64>> for PowerLevel<f64> {
    fn from(power: Power<f64>) -> Self {
        WATT_LEVEL + BEL.quantity(Real::log10(WATT.value(power)))
    }
}

/// The power ratio a gain is: ten times for every bel.
impl From<Gain<f64>> for Dimensionless<f64> {
    fn from(gain: Gain<f64>) -> Self {
        RATIO.quantity(Real::powf(10.0, BEL.value(gain)))
    }
}

/// The gain a power ratio is: `lg(ratio)` bels.
impl From<Dimensionless<f64>> for Gain<f64> {
    fn from(ratio: Dimensionless<f64>) -> Self {
        BEL.quantity(Real::log10(RATIO.value(ratio)))
    }
}
