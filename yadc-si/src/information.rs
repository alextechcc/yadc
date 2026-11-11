use crate::{Exact, ISQ, SIQuantity, counted};
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
/// (re-exported), then one per multiple, each defined exactly from them -
/// `INCH` is `0.0254 * METRE` - and counted in the number type its use asks
/// for, rounded once: `let d: Length<f64> = INCH;` is an `f64` length,
/// `Length<f32>` an `f32` one, `INCH::<Exact>` the exact value. Each is a
/// unit (`INCH(3.0)`, `INCH.value(d)`). A prefixed unit is a prefix times
/// one: `KILO * METRE`. They compose in `const` items:
/// `const LIMIT: Velocity<f64> = 50.0 * (KILO * METRE) / HOUR;`.
pub mod consts {
    use super::*;
    use crate::base::SECOND;

    /// The number one: the unit of dimension one.
    pub use crate::base::ONE;

    /// One `Bit` (b), as an `Information`.
    pub const BIT<N: const From<Exact>>: Information<N> = counted(Exact::dec(125, -3) * BYTE::<Exact>);

    /// One `Byte` (B), as an `Information`.
    pub const BYTE<N: const From<Exact>>: Information<N> = counted(ONE::<Exact>);

    /// One `Octet` (o), as an `Information`.
    pub const OCTET<N: const From<Exact>>: Information<N> = counted(BYTE::<Exact>);

    /// One `Nibble` (nibble), as an `Information`.
    pub const NIBBLE<N: const From<Exact>>: Information<N> = counted(Exact::dec(5, -1) * BYTE::<Exact>);

    /// One `Crumb` (crumb), as an `Information`.
    pub const CRUMB<N: const From<Exact>>: Information<N> = counted(Exact::dec(25, -2) * BYTE::<Exact>);

    /// One `Shannon` (Sh), as an `Information`.
    pub const SHANNON<N: const From<Exact>>: Information<N> = counted(Exact::dec(125, -3) * BYTE::<Exact>);

    /// One `BitPerSecond` (b/s), as an `InformationRate`.
    pub const BIT_PER_SECOND<N: const From<Exact>>: InformationRate<N> = counted(Exact::dec(125, -3) * BYTE_PER_SECOND::<Exact>);

    /// One `BytePerSecond` (B/s), as an `InformationRate`.
    pub const BYTE_PER_SECOND<N: const From<Exact>>: InformationRate<N> = counted(ONE::<Exact> / SECOND::<Exact>);

    /// One `OctetPerSecond` (o/s), as an `InformationRate`.
    pub const OCTET_PER_SECOND<N: const From<Exact>>: InformationRate<N> = counted(BYTE_PER_SECOND::<Exact>);
}

/// The units of `Information`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Information<f64>)]
pub enum InformationUnit {
    #[unit("b", "bits", by = BIT)]
    Bit,
    #[default]
    #[unit("B", "bytes", by = BYTE)]
    Byte,
    #[unit("o", "octets", by = OCTET)]
    Octet,
    #[unit("nibble", "nibbles", by = NIBBLE)]
    Nibble,
    #[unit("crumb", "crumbs", by = CRUMB)]
    Crumb,
    #[unit("Sh", "shannons", by = SHANNON)]
    Shannon,
}

/// The units of `InformationRate`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = InformationRate<f64>)]
pub enum InformationRateUnit {
    #[unit("b/s", "bits per second", by = BIT_PER_SECOND)]
    BitPerSecond,
    #[default]
    #[unit("B/s", "bytes per second", by = BYTE_PER_SECOND)]
    BytePerSecond,
    #[unit("o/s", "octets per second", by = OCTET_PER_SECOND)]
    OctetPerSecond,
}
