use crate::{Exact, ISQ, SIQuantity};
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
/// `const MPH: Velocity<f64> = (MILE / HOUR).into();`, then `MPH.quantity(60.0)`.
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
