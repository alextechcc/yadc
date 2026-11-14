use crate::Dim;
use yadc::Dimension;

/// The dimension of `AmountOfInformation`: `Qty<AMOUNT_OF_INFORMATION, N>` is one.
pub const AMOUNT_OF_INFORMATION: Dim = Dim::DIMENSIONLESS;

/// The dimension of `SignalTransmissionRate`: `Qty<SIGNAL_TRANSMISSION_RATE, N>` is one.
pub const SIGNAL_TRANSMISSION_RATE: Dim = Dim {
    s: -1,
    ..Dim::DIMENSIONLESS
};

/// The units of this module's quantities: each a constant of its own unit
/// type, its size and zero in its quantity's coherent unit, exact
/// (`MILE.quantity(3.0)`, `MILE.value(d)`).
pub mod units {
    use super::*;
    use crate::{Exact, UcumUnit, UcumUnitConsts};

    /// Bit (`bit`), a unit of `AmountOfInformation`: `n` of them is `n · 1` coherent units.
    pub const BIT: UcumUnit<
        AMOUNT_OF_INFORMATION,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "bit",
                name: "bit",
            }
        },
    > = UcumUnit;

    /// Byte (`By`), a unit of `AmountOfInformation`: `n` of them is `n · 8` coherent units.
    pub const BYTE: UcumUnit<
        AMOUNT_OF_INFORMATION,
        {
            UcumUnitConsts {
                size: Exact::int(8),
                zero: Exact::ZERO,
                symbol: "By",
                name: "byte",
            }
        },
    > = UcumUnit;

    /// Baud (`Bd`), a unit of `SignalTransmissionRate`: `n` of them is `n · 1` coherent units.
    pub const BAUD: UcumUnit<
        SIGNAL_TRANSMISSION_RATE,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Bd",
                name: "baud",
            }
        },
    > = UcumUnit;
}
