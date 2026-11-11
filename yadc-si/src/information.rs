use crate::{ISQ, StaticSIQuantity};
use consts::*;
use yadc::Dimension;

pub type Information = StaticSIQuantity<{ ISQ::DIMENSIONLESS }>;

pub type InformationRate = StaticSIQuantity<
    {
        ISQ {
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

/// The constants of this module: the base constants of its quantities
/// (re-exported), then one per multiple, each defined from them - `KILOMETRE`
/// is `1000.0 * METRE`, a `Length` and a unit of its own (`KILOMETRE.value(d)`).
/// Quantity arithmetic is `const`, so they compose in `const` items:
/// `const LIMIT: Velocity = 50.0 * KILOMETRE / HOUR;`.
pub mod consts {
    use super::*;
    use crate::base::SECOND;

    /// The number one: the unit of dimension one.
    pub use crate::base::ONE;

    /// One `Yottabit` (Yb), as an `Information`.
    pub const YOTTABIT: Information = 1.25e+23 * BYTE;

    /// One `Zettabit` (Zb), as an `Information`.
    pub const ZETTABIT: Information = 1.25e+20 * BYTE;

    /// One `Exabit` (Eb), as an `Information`.
    pub const EXABIT: Information = 1.25e+17 * BYTE;

    /// One `Petabit` (Pb), as an `Information`.
    pub const PETABIT: Information = 125000000000000.0 * BYTE;

    /// One `Terabit` (Tb), as an `Information`.
    pub const TERABIT: Information = 125000000000.0 * BYTE;

    /// One `Gigabit` (Gb), as an `Information`.
    pub const GIGABIT: Information = 125000000.0 * BYTE;

    /// One `Megabit` (Mb), as an `Information`.
    pub const MEGABIT: Information = 125000.0 * BYTE;

    /// One `Kilobit` (kb), as an `Information`.
    pub const KILOBIT: Information = 125.0 * BYTE;

    /// One `Bit` (b), as an `Information`.
    pub const BIT: Information = 0.125 * BYTE;

    /// One `Yottabyte` (YB), as an `Information`.
    pub const YOTTABYTE: Information = 9.9999999999999998e+23 * BYTE;

    /// One `Zettabyte` (ZB), as an `Information`.
    pub const ZETTABYTE: Information = 1e+21 * BYTE;

    /// One `Exabyte` (EB), as an `Information`.
    pub const EXABYTE: Information = 1e+18 * BYTE;

    /// One `Petabyte` (PB), as an `Information`.
    pub const PETABYTE: Information = 1000000000000000.0 * BYTE;

    /// One `Terabyte` (TB), as an `Information`.
    pub const TERABYTE: Information = 1000000000000.0 * BYTE;

    /// One `Gigabyte` (GB), as an `Information`.
    pub const GIGABYTE: Information = 1000000000.0 * BYTE;

    /// One `Megabyte` (MB), as an `Information`.
    pub const MEGABYTE: Information = 1000000.0 * BYTE;

    /// One `Kilobyte` (kB), as an `Information`.
    pub const KILOBYTE: Information = 1000.0 * BYTE;

    /// One `Byte` (B), as an `Information`.
    pub const BYTE: Information = ONE;

    /// One `Octet` (o), as an `Information`.
    pub const OCTET: Information = BYTE;

    /// One `Nibble` (nibble), as an `Information`.
    pub const NIBBLE: Information = 0.5 * BYTE;

    /// One `Crumb` (crumb), as an `Information`.
    pub const CRUMB: Information = 0.25 * BYTE;

    /// One `Shannon` (Sh), as an `Information`.
    pub const SHANNON: Information = 0.125 * BYTE;

    /// One `BitPerSecond` (b/s), as an `InformationRate`.
    pub const BIT_PER_SECOND: InformationRate = 0.125 * BYTE_PER_SECOND;

    /// One `YottabytePerSecond` (YB/s), as an `InformationRate`.
    pub const YOTTABYTE_PER_SECOND: InformationRate = 9.9999999999999998e+23 * BYTE_PER_SECOND;

    /// One `ZettabytePerSecond` (ZB/s), as an `InformationRate`.
    pub const ZETTABYTE_PER_SECOND: InformationRate = 1e+21 * BYTE_PER_SECOND;

    /// One `ExabytePerSecond` (EB/s), as an `InformationRate`.
    pub const EXABYTE_PER_SECOND: InformationRate = 1e+18 * BYTE_PER_SECOND;

    /// One `PetabytePerSecond` (PB/s), as an `InformationRate`.
    pub const PETABYTE_PER_SECOND: InformationRate = 1000000000000000.0 * BYTE_PER_SECOND;

    /// One `TerabytePerSecond` (TB/s), as an `InformationRate`.
    pub const TERABYTE_PER_SECOND: InformationRate = 1000000000000.0 * BYTE_PER_SECOND;

    /// One `GigabytePerSecond` (GB/s), as an `InformationRate`.
    pub const GIGABYTE_PER_SECOND: InformationRate = 1000000000.0 * BYTE_PER_SECOND;

    /// One `MegabytePerSecond` (MB/s), as an `InformationRate`.
    pub const MEGABYTE_PER_SECOND: InformationRate = 1000000.0 * BYTE_PER_SECOND;

    /// One `KilobytePerSecond` (kB/s), as an `InformationRate`.
    pub const KILOBYTE_PER_SECOND: InformationRate = 1000.0 * BYTE_PER_SECOND;

    /// One `BytePerSecond` (B/s), as an `InformationRate`.
    pub const BYTE_PER_SECOND: InformationRate = ONE / SECOND;

    /// One `OctetPerSecond` (o/s), as an `InformationRate`.
    pub const OCTET_PER_SECOND: InformationRate = BYTE_PER_SECOND;
}

/// The units of `Information`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Information)]
pub enum InformationUnit {
    #[unit("Yb", "yottabits", by = YOTTABIT)]
    Yottabit,
    #[unit("Zb", "zettabits", by = ZETTABIT)]
    Zettabit,
    #[unit("Eb", "exabits", by = EXABIT)]
    Exabit,
    #[unit("Pb", "petabits", by = PETABIT)]
    Petabit,
    #[unit("Tb", "terabits", by = TERABIT)]
    Terabit,
    #[unit("Gb", "gigabits", by = GIGABIT)]
    Gigabit,
    #[unit("Mb", "megabits", by = MEGABIT)]
    Megabit,
    #[unit("kb", "kilobits", by = KILOBIT)]
    Kilobit,
    #[unit("b", "bits", by = BIT)]
    Bit,
    #[unit("YB", "yottabytes", by = YOTTABYTE)]
    Yottabyte,
    #[unit("ZB", "zettabytes", by = ZETTABYTE)]
    Zettabyte,
    #[unit("EB", "exabytes", by = EXABYTE)]
    Exabyte,
    #[unit("PB", "petabytes", by = PETABYTE)]
    Petabyte,
    #[unit("TB", "terabytes", by = TERABYTE)]
    Terabyte,
    #[unit("GB", "gigabytes", by = GIGABYTE)]
    Gigabyte,
    #[unit("MB", "megabytes", by = MEGABYTE)]
    Megabyte,
    #[unit("kB", "kilobytes", by = KILOBYTE)]
    Kilobyte,
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
#[unit(of = InformationRate)]
pub enum InformationRateUnit {
    #[unit("b/s", "bits per second", by = BIT_PER_SECOND)]
    BitPerSecond,
    #[unit("YB/s", "yottabytes per second", by = YOTTABYTE_PER_SECOND)]
    YottabytePerSecond,
    #[unit("ZB/s", "zettabytes per second", by = ZETTABYTE_PER_SECOND)]
    ZettabytePerSecond,
    #[unit("EB/s", "exabytes per second", by = EXABYTE_PER_SECOND)]
    ExabytePerSecond,
    #[unit("PB/s", "petabytes per second", by = PETABYTE_PER_SECOND)]
    PetabytePerSecond,
    #[unit("TB/s", "terabytes per second", by = TERABYTE_PER_SECOND)]
    TerabytePerSecond,
    #[unit("GB/s", "gigabytes per second", by = GIGABYTE_PER_SECOND)]
    GigabytePerSecond,
    #[unit("MB/s", "megabytes per second", by = MEGABYTE_PER_SECOND)]
    MegabytePerSecond,
    #[unit("kB/s", "kilobytes per second", by = KILOBYTE_PER_SECOND)]
    KilobytePerSecond,
    #[default]
    #[unit("B/s", "bytes per second", by = BYTE_PER_SECOND)]
    BytePerSecond,
    #[unit("o/s", "octets per second", by = OCTET_PER_SECOND)]
    OctetPerSecond,
}
