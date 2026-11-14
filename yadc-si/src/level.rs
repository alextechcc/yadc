//! Power (`POWER`) <-> power level (`POWER_LEVEL`), and a plain ratio <->
//! gain (`GAIN`): hand-written,
//! not generated.
//!
//! Decibels count on a logarithmic line, where multiplying is adding: a gain
//! of 3 dB and one of 10 dB make 13 dB, where the ratios 2 and 10 make 20.
//! Both quantities on the line have the logarithmic kind, whose `+` and `-`
//! add and subtract the exponents. A gain is the logarithm of a plain
//! power ratio - no exponents - a quantity of its own with plain units
//! (`DECIBEL.quantity(3.0)`). A power level is the logarithm of a power,
//! and carries power's exponents: two levels differ by a gain (watts over
//! watts), a gain moves a level, and two levels do not sum to a level. Its
//! units count from a zero: dBW decibels from one watt, dBm from one
//! milliwatt.
//!
//! The logarithm itself is here and nowhere else: the conversions between
//! the line and the linear quantities, std `From` - `SIQuantity::<POWER, _>::from(level)`,
//! and `SIQuantity::<POWER_LEVEL, _>::from(p)` back. Both directions read each quantity's number in
//! a unit, in any number type with a logarithm ([`Real`]: `f64`, `f32`; in
//! `no_std` builds with the `libm` feature).

use crate::dimensionless::DIMENSIONLESS;
use crate::dimensionless::units::RATIO;
use crate::mechanical::units::{BEL, DECIBEL_WATT, WATT};
use crate::mechanical::{GAIN, POWER, POWER_LEVEL};
use crate::{Exact, SIQuantity, Unit};
use yadc::num_traits::real::Real;

/// Ten, the base of the logarithm.
fn ten<N: From<Exact>>() -> N {
    N::from(Exact::int(10))
}

/// The power at a level: a watt, ten times for every ten dBW.
impl<N: Real + From<Exact>> From<SIQuantity<POWER_LEVEL, N>> for SIQuantity<POWER, N> {
    fn from(level: SIQuantity<POWER_LEVEL, N>) -> Self {
        WATT.quantity(ten::<N>().powf(DECIBEL_WATT.value(level) / ten()))
    }
}

/// The level of a power: `10 lg(power / 1 W)` dBW. A zero power is at minus
/// infinity, and a negative one has no level (NaN), as for `log10`.
impl<N: Real + From<Exact>> From<SIQuantity<POWER, N>> for SIQuantity<POWER_LEVEL, N> {
    fn from(power: SIQuantity<POWER, N>) -> Self {
        DECIBEL_WATT.quantity(ten::<N>() * WATT.value(power).log10())
    }
}

/// The power ratio a gain is: ten times for every bel.
impl<N: Real + From<Exact>> From<SIQuantity<GAIN, N>> for SIQuantity<DIMENSIONLESS, N> {
    fn from(gain: SIQuantity<GAIN, N>) -> Self {
        RATIO.quantity(ten::<N>().powf(BEL.value(gain)))
    }
}

/// The gain a power ratio is: `lg(ratio)` bels.
impl<N: Real + From<Exact>> From<SIQuantity<DIMENSIONLESS, N>> for SIQuantity<GAIN, N> {
    fn from(ratio: SIQuantity<DIMENSIONLESS, N>) -> Self {
        BEL.quantity(RATIO.value(ratio).log10())
    }
}
