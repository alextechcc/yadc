use crate::{Dim, Qty};
use yadc::Dimension;

pub type AmountOfInformation<N> = Qty<{ Dim::DIMENSIONLESS }, N>;

/// The dimension of `SignalTransmissionRate`.
pub const SIGNAL_TRANSMISSION_RATE: Dim = Dim {
    s: -1,
    ..Dim::DIMENSIONLESS
};
pub type SignalTransmissionRate<N> = Qty<SIGNAL_TRANSMISSION_RATE, N>;

/// The units of this module's quantities: a type each, its size and zero
/// in its quantity's coherent unit, exact (`Mile {}.quantity(3.0_f64)`).
pub mod units {
    use super::*;
    use crate::{Dim, Exact, UcumUnit, UcumUnitConsts};
    use yadc::Dimension;

    /// Bit (`bit`), a unit of `AmountOfInformation`: `n` of them is `n · 1` coherent units.
    pub type Bit = UcumUnit<
        { Dim::DIMENSIONLESS },
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "bit",
                name: "bit",
            }
        },
    >;

    /// Byte (`By`), a unit of `AmountOfInformation`: `n` of them is `n · 8` coherent units.
    pub type Byte = UcumUnit<
        { Dim::DIMENSIONLESS },
        {
            UcumUnitConsts {
                size: Exact::int(8),
                zero: Exact::ZERO,
                symbol: "By",
                name: "byte",
            }
        },
    >;

    /// Baud (`Bd`), a unit of `SignalTransmissionRate`: `n` of them is `n · 1` coherent units.
    pub type Baud = UcumUnit<
        SIGNAL_TRANSMISSION_RATE,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Bd",
                name: "baud",
            }
        },
    >;
}
