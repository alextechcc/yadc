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
//! the watts are one std `From` away - `Power::from(level)`, and
//! `PowerLevel::from(p)` back - while `PowerUnit`'s decibel variants do both
//! steps (`via = PowerLevel`). Both directions read each quantity's number in
//! a unit - the exact constants, converted to the number type in use - in any
//! number type with a logarithm ([`Real`]: `f64`, `f32`; in `no_std` builds
//! with the `libm` feature).

use crate::Exact;
use crate::dimensionless::Dimensionless;
use crate::dimensionless::consts::RATIO;
use crate::mechanical::consts::{BEL, WATT, WATT_LEVEL};
use crate::mechanical::{Gain, Power, PowerLevel};
use core::ops::{Add, Sub};
use yadc::Unit;
use yadc::num_traits::real::Real;

/// Ten, the base of the logarithm.
fn ten<N: From<Exact>>() -> N {
    N::from(Exact::int(10))
}

/// The power at a level: a watt, ten times for every bel above
/// `WATT_LEVEL`.
impl<N> From<PowerLevel<N>> for Power<N>
where
    N: Real + From<Exact>,
    PowerLevel<N>: Sub<Output = Gain<N>>,
    Gain<N>: Unit<Gain<N>, N>,
    Power<N>: Unit<Power<N>, N>,
{
    fn from(level: PowerLevel<N>) -> Self {
        let bel = <Gain<N> as From<Gain<Exact>>>::from(BEL);
        let zero = <PowerLevel<N> as From<PowerLevel<Exact>>>::from(WATT_LEVEL);
        let watt = <Power<N> as From<Power<Exact>>>::from(WATT);
        watt.quantity(ten::<N>().powf(bel.value(level - zero)))
    }
}

/// The level of a power: `lg(power / 1 W)` bels above `WATT_LEVEL`. A zero
/// power is at minus infinity, and a negative one has no level (NaN), as for
/// `log10`.
impl<N> From<Power<N>> for PowerLevel<N>
where
    N: Real + From<Exact>,
    PowerLevel<N>: Add<Gain<N>, Output = PowerLevel<N>>,
    Gain<N>: Unit<Gain<N>, N>,
    Power<N>: Unit<Power<N>, N>,
{
    fn from(power: Power<N>) -> Self {
        let bel = <Gain<N> as From<Gain<Exact>>>::from(BEL);
        let zero = <PowerLevel<N> as From<PowerLevel<Exact>>>::from(WATT_LEVEL);
        let watt = <Power<N> as From<Power<Exact>>>::from(WATT);
        zero + bel.quantity(watt.value(power).log10())
    }
}

/// The power ratio a gain is: ten times for every bel.
impl<N> From<Gain<N>> for Dimensionless<N>
where
    N: Real + From<Exact>,
    Gain<N>: Unit<Gain<N>, N>,
    Dimensionless<N>: Unit<Dimensionless<N>, N>,
{
    fn from(gain: Gain<N>) -> Self {
        let bel = <Gain<N> as From<Gain<Exact>>>::from(BEL);
        let ratio = <Dimensionless<N> as From<Dimensionless<Exact>>>::from(RATIO);
        ratio.quantity(ten::<N>().powf(bel.value(gain)))
    }
}

/// The gain a power ratio is: `lg(ratio)` bels.
impl<N> From<Dimensionless<N>> for Gain<N>
where
    N: Real + From<Exact>,
    Gain<N>: Unit<Gain<N>, N>,
    Dimensionless<N>: Unit<Dimensionless<N>, N>,
{
    fn from(ratio: Dimensionless<N>) -> Self {
        let bel = <Gain<N> as From<Gain<Exact>>>::from(BEL);
        let ratio_unit = <Dimensionless<N> as From<Dimensionless<Exact>>>::from(RATIO);
        bel.quantity(ratio_unit.value(ratio).log10())
    }
}
