use crate::base::SECOND;
use crate::{Exact, ISQ, SIQuantity};
use consts::*;
use yadc::Dimension;

pub type Information<N> = SIQuantity<{ ISQ::DIMENSIONLESS }, N>;

pub type InformationRate<N> = SIQuantity<
    {
        ISQ {
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

/// The constants of this module: the base constants of its quantities
/// (re-exported), then one per named unit, each defined exactly from them
/// (`INCH` is `0.0254 * METRE`). A unit written as others is not one:
/// `SLUG / CUBIC_FOOT`, `KILO * METRE` are exact too. Converting rounds,
/// once, where it is written - in a `const`, by the compiler:
/// `const MPH: Velocity<f64> = (MILE / HOUR).into();`, then `MPH(60.0)`.
pub mod consts {
    use super::*;

    /// The number one: the unit of dimension one.
    pub use crate::base::ONE;

    /// One `Bit` (`b`), as an `Information`.
    pub const BIT: Information<Exact> = BYTE * Exact::dec(125, -3);

    /// One `Byte` (`B`), as an `Information`.
    pub const BYTE: Information<Exact> = ONE;

    /// One `Octet` (`o`), as an `Information`.
    pub const OCTET: Information<Exact> = BYTE;

    /// One `Nibble` (`nibble`), as an `Information`.
    pub const NIBBLE: Information<Exact> = BYTE * Exact::dec(5, -1);

    /// One `Crumb` (`crumb`), as an `Information`.
    pub const CRUMB: Information<Exact> = BYTE * Exact::dec(25, -2);

    /// One `Shannon` (`Sh`), as an `Information`.
    pub const SHANNON: Information<Exact> = BYTE * Exact::dec(125, -3);
}

/// The units of `Information`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Information<N>, number = N)]
#[unit(for<N> where Information<N>: [const] yadc::Unit<Information<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum InformationUnit {
    #[unit("b", "bits", by = Information::<N>::from(BIT))]
    Bit,
    #[default]
    #[unit("B", "bytes", by = Information::<N>::from(BYTE))]
    Byte,
    #[unit("o", "octets", by = Information::<N>::from(OCTET))]
    Octet,
    #[unit("nibble", "nibbles", by = Information::<N>::from(NIBBLE))]
    Nibble,
    #[unit("crumb", "crumbs", by = Information::<N>::from(CRUMB))]
    Crumb,
    #[unit("Sh", "shannons", by = Information::<N>::from(SHANNON))]
    Shannon,
}

/// The units of `InformationRate`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = InformationRate<N>, number = N)]
#[unit(for<N> where InformationRate<N>: [const] yadc::Unit<InformationRate<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum InformationRateUnit {
    #[unit("b/s", "bits per second", by = InformationRate::<N>::from((ONE / SECOND) * Exact::dec(125, -3)))]
    BitPerSecond,
    #[default]
    #[unit("B/s", "bytes per second", by = InformationRate::<N>::from(ONE / SECOND))]
    BytePerSecond,
    #[unit("o/s", "octets per second", by = InformationRate::<N>::from(ONE / SECOND))]
    OctetPerSecond,
}
