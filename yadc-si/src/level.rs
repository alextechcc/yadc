//! [`Power`] <-> [`PowerLevel`], and a plain ratio <-> [`Gain`]: hand-written,
//! not generated.
//!
//! Decibels count on a logarithmic line, where multiplying is adding: a gain
//! of 3 dB and one of 10 dB make 13 dB, where the ratios 2 and 10 make 20.
//! Both quantities on the line have the logarithmic kind, whose `+` and `-`
//! add and subtract the exponents. A [`Gain`] is the logarithm of a plain
//! power ratio - no exponents - a quantity of its own with plain units
//! (`Decibel {}.quantity(3.0)`). A [`PowerLevel`] is the logarithm of a power,
//! and carries power's exponents: two levels differ by a gain (watts over
//! watts), a gain moves a level, and two levels do not sum to a level. Its
//! units count from a zero: dBW decibels from one watt, dBm from one
//! milliwatt.
//!
//! The logarithm itself is here and nowhere else: the conversions between
//! the line and the linear quantities, std `From` - `Power::from(level)`, and
//! `PowerLevel::from(p)` back. Both directions read each quantity's number in
//! a unit, in any number type with a logarithm ([`Real`]: `f64`, `f32`; in
//! `no_std` builds with the `libm` feature).

use crate::Exact;
use crate::dimensionless::Dimensionless;
use crate::dimensionless::units::Ratio;
use crate::mechanical::units::{Bel, DecibelWatt, Watt};
use crate::mechanical::{Gain, Power, PowerLevel};
use yadc::Unit;
use yadc::num_traits::real::Real;

/// Ten, the base of the logarithm.
fn ten<N: From<Exact>>() -> N {
    N::from(Exact::int(10))
}

/// The power at a level: a watt, ten times for every ten dBW.
impl<N: Real + const From<Exact>> From<PowerLevel<N>> for Power<N> {
    fn from(level: PowerLevel<N>) -> Self {
        Watt {}.quantity(ten::<N>().powf(DecibelWatt {}.value(level) / ten()))
    }
}

/// The level of a power: `10 lg(power / 1 W)` dBW. A zero power is at minus
/// infinity, and a negative one has no level (NaN), as for `log10`.
impl<N: Real + const From<Exact>> From<Power<N>> for PowerLevel<N> {
    fn from(power: Power<N>) -> Self {
        DecibelWatt {}.quantity(ten::<N>() * Watt {}.value(power).log10())
    }
}

/// The power ratio a gain is: ten times for every bel.
impl<N: Real + const From<Exact>> From<Gain<N>> for Dimensionless<N> {
    fn from(gain: Gain<N>) -> Self {
        Ratio {}.quantity(ten::<N>().powf(Bel {}.value(gain)))
    }
}

/// The gain a power ratio is: `lg(ratio)` bels.
impl<N: Real + const From<Exact>> From<Dimensionless<N>> for Gain<N> {
    fn from(ratio: Dimensionless<N>) -> Self {
        Bel {}.quantity(Ratio {}.value(ratio).log10())
    }
}
