use crate::{ISQ, SIQuantity};
use yadc::Dimension;

pub type Information<N> = SIQuantity<{ ISQ::DIMENSIONLESS }, N>;

/// The dimension of `InformationRate`.
pub const INFORMATION_RATE: ISQ = ISQ {
    s: -1,
    ..ISQ::DIMENSIONLESS
};
pub type InformationRate<N> = SIQuantity<INFORMATION_RATE, N>;

/// The units of this module's quantities: a type each, its size and zero
/// in its quantity's coherent unit, exact (`Mile {}.quantity(3.0_f64)`).
pub mod units {
    use super::*;
    use crate::{Exact, ISQ, SIUnit, SIUnitConsts};
    use yadc::Dimension;

    /// Bits (`b`), a unit of `Information`: `n` of them is `n · 0.125` coherent units.
    pub type Bit = SIUnit<
        { ISQ::DIMENSIONLESS },
        {
            SIUnitConsts {
                size: Exact::dec(125, -3),
                zero: Exact::ZERO,
                symbol: "b",
                name: "bits",
            }
        },
    >;

    /// Bytes (`B`), a unit of `Information`: `n` of them is `n · 1.0` coherent units.
    pub type Byte = SIUnit<
        { ISQ::DIMENSIONLESS },
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "B",
                name: "bytes",
            }
        },
    >;

    /// Octets (`o`), a unit of `Information`: `n` of them is `n · 1.0` coherent units.
    pub type Octet = SIUnit<
        { ISQ::DIMENSIONLESS },
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "o",
                name: "octets",
            }
        },
    >;

    /// Nibbles (`nibble`), a unit of `Information`: `n` of them is `n · 0.5` coherent units.
    pub type Nibble = SIUnit<
        { ISQ::DIMENSIONLESS },
        {
            SIUnitConsts {
                size: Exact::dec(5, -1),
                zero: Exact::ZERO,
                symbol: "nibble",
                name: "nibbles",
            }
        },
    >;

    /// Crumbs (`crumb`), a unit of `Information`: `n` of them is `n · 0.25` coherent units.
    pub type Crumb = SIUnit<
        { ISQ::DIMENSIONLESS },
        {
            SIUnitConsts {
                size: Exact::dec(25, -2),
                zero: Exact::ZERO,
                symbol: "crumb",
                name: "crumbs",
            }
        },
    >;

    /// Shannons (`Sh`), a unit of `Information`: `n` of them is `n · 0.125` coherent units.
    pub type Shannon = SIUnit<
        { ISQ::DIMENSIONLESS },
        {
            SIUnitConsts {
                size: Exact::dec(125, -3),
                zero: Exact::ZERO,
                symbol: "Sh",
                name: "shannons",
            }
        },
    >;

    /// Bits per second (`b/s`), a unit of `InformationRate`: `n` of them is `n · 0.125` coherent units.
    pub type BitPerSecond = SIUnit<
        INFORMATION_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(125, -3),
                zero: Exact::ZERO,
                symbol: "b/s",
                name: "bits per second",
            }
        },
    >;

    /// Bytes per second (`B/s`), a unit of `InformationRate`: `n` of them is `n · 1.0` coherent units.
    pub type BytePerSecond = SIUnit<
        INFORMATION_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "B/s",
                name: "bytes per second",
            }
        },
    >;

    /// Octets per second (`o/s`), a unit of `InformationRate`: `n` of them is `n · 1.0` coherent units.
    pub type OctetPerSecond = SIUnit<
        INFORMATION_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "o/s",
                name: "octets per second",
            }
        },
    >;
}
