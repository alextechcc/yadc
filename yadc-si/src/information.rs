use crate::ISQ;
use yadc::Dimension;

/// The dimension of `Information`: `SIQuantity<INFORMATION, N>` is one.
pub const INFORMATION: ISQ = ISQ::DIMENSIONLESS;

/// The dimension of `InformationRate`: `SIQuantity<INFORMATION_RATE, N>` is one.
pub const INFORMATION_RATE: ISQ = ISQ {
    s: -1,
    ..ISQ::DIMENSIONLESS
};

/// The units of this module's quantities: each a constant of its own unit
/// type, its size and zero in its quantity's coherent unit, exact
/// (`MILE.quantity(3.0)`, `MILE.value(d)`).
pub mod units {
    use super::*;
    use crate::{Exact, SIUnit, SIUnitConsts};

    /// Bits (`b`), a unit of `Information`: `n` of them is `n · 0.125` coherent units.
    pub const BIT: SIUnit<
        INFORMATION,
        {
            SIUnitConsts {
                size: Exact::dec(125, -3),
                zero: Exact::ZERO,
                symbol: "b",
                name: "bits",
            }
        },
    > = SIUnit;

    /// Bytes (`B`), a unit of `Information`: `n` of them is `n · 1.0` coherent units.
    pub const BYTE: SIUnit<
        INFORMATION,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "B",
                name: "bytes",
            }
        },
    > = SIUnit;

    /// Octets (`o`), a unit of `Information`: `n` of them is `n · 1.0` coherent units.
    pub const OCTET: SIUnit<
        INFORMATION,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "o",
                name: "octets",
            }
        },
    > = SIUnit;

    /// Nibbles (`nibble`), a unit of `Information`: `n` of them is `n · 0.5` coherent units.
    pub const NIBBLE: SIUnit<
        INFORMATION,
        {
            SIUnitConsts {
                size: Exact::dec(5, -1),
                zero: Exact::ZERO,
                symbol: "nibble",
                name: "nibbles",
            }
        },
    > = SIUnit;

    /// Crumbs (`crumb`), a unit of `Information`: `n` of them is `n · 0.25` coherent units.
    pub const CRUMB: SIUnit<
        INFORMATION,
        {
            SIUnitConsts {
                size: Exact::dec(25, -2),
                zero: Exact::ZERO,
                symbol: "crumb",
                name: "crumbs",
            }
        },
    > = SIUnit;

    /// Shannons (`Sh`), a unit of `Information`: `n` of them is `n · 0.125` coherent units.
    pub const SHANNON: SIUnit<
        INFORMATION,
        {
            SIUnitConsts {
                size: Exact::dec(125, -3),
                zero: Exact::ZERO,
                symbol: "Sh",
                name: "shannons",
            }
        },
    > = SIUnit;

    /// Bits per second (`b/s`), a unit of `InformationRate`: `n` of them is `n · 0.125` coherent units.
    pub const BIT_PER_SECOND: SIUnit<
        INFORMATION_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(125, -3),
                zero: Exact::ZERO,
                symbol: "b/s",
                name: "bits per second",
            }
        },
    > = SIUnit;

    /// Bytes per second (`B/s`), a unit of `InformationRate`: `n` of them is `n · 1.0` coherent units.
    pub const BYTE_PER_SECOND: SIUnit<
        INFORMATION_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "B/s",
                name: "bytes per second",
            }
        },
    > = SIUnit;

    /// Octets per second (`o/s`), a unit of `InformationRate`: `n` of them is `n · 1.0` coherent units.
    pub const OCTET_PER_SECOND: SIUnit<
        INFORMATION_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "o/s",
                name: "octets per second",
            }
        },
    > = SIUnit;
}
