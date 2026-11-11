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
/// `KILOMETRE` is `1000 * METRE` - and counted in the number type its use
/// asks for, rounded once: `let d: Length = KILOMETRE;` is an `f64` length,
/// `Length<f32>` an `f32` one, `KILOMETRE::<Exact>` the exact value. Each is
/// a unit (`KILOMETRE(3.0)`, `KILOMETRE.value(d)`), and they compose in
/// `const` items: `const LIMIT: Velocity = 50.0 * KILOMETRE / HOUR;`.
pub mod consts {
    use super::*;
    use crate::base::SECOND;

    /// The number one: the unit of dimension one.
    pub use crate::base::ONE;

    /// One `Yottabit` (Yb), as an `Information`.
    pub const YOTTABIT<N: const From<Exact>>: Information<N> = counted(Exact::int(125000000000000000000000) * BYTE::<Exact>);

    /// One `Zettabit` (Zb), as an `Information`.
    pub const ZETTABIT<N: const From<Exact>>: Information<N> = counted(Exact::int(125000000000000000000) * BYTE::<Exact>);

    /// One `Exabit` (Eb), as an `Information`.
    pub const EXABIT<N: const From<Exact>>: Information<N> = counted(Exact::int(125000000000000000) * BYTE::<Exact>);

    /// One `Petabit` (Pb), as an `Information`.
    pub const PETABIT<N: const From<Exact>>: Information<N> = counted(Exact::int(125000000000000) * BYTE::<Exact>);

    /// One `Terabit` (Tb), as an `Information`.
    pub const TERABIT<N: const From<Exact>>: Information<N> = counted(Exact::int(125000000000) * BYTE::<Exact>);

    /// One `Gigabit` (Gb), as an `Information`.
    pub const GIGABIT<N: const From<Exact>>: Information<N> = counted(Exact::int(125000000) * BYTE::<Exact>);

    /// One `Megabit` (Mb), as an `Information`.
    pub const MEGABIT<N: const From<Exact>>: Information<N> = counted(Exact::int(125000) * BYTE::<Exact>);

    /// One `Kilobit` (kb), as an `Information`.
    pub const KILOBIT<N: const From<Exact>>: Information<N> = counted(Exact::int(125) * BYTE::<Exact>);

    /// One `Bit` (b), as an `Information`.
    pub const BIT<N: const From<Exact>>: Information<N> = counted(Exact::dec(125, -3) * BYTE::<Exact>);

    /// One `Yottabyte` (YB), as an `Information`.
    pub const YOTTABYTE<N: const From<Exact>>: Information<N> = counted(Exact::int(1000000000000000000000000) * BYTE::<Exact>);

    /// One `Zettabyte` (ZB), as an `Information`.
    pub const ZETTABYTE<N: const From<Exact>>: Information<N> = counted(Exact::int(1000000000000000000000) * BYTE::<Exact>);

    /// One `Exabyte` (EB), as an `Information`.
    pub const EXABYTE<N: const From<Exact>>: Information<N> = counted(Exact::int(1000000000000000000) * BYTE::<Exact>);

    /// One `Petabyte` (PB), as an `Information`.
    pub const PETABYTE<N: const From<Exact>>: Information<N> = counted(Exact::int(1000000000000000) * BYTE::<Exact>);

    /// One `Terabyte` (TB), as an `Information`.
    pub const TERABYTE<N: const From<Exact>>: Information<N> = counted(Exact::int(1000000000000) * BYTE::<Exact>);

    /// One `Gigabyte` (GB), as an `Information`.
    pub const GIGABYTE<N: const From<Exact>>: Information<N> = counted(Exact::int(1000000000) * BYTE::<Exact>);

    /// One `Megabyte` (MB), as an `Information`.
    pub const MEGABYTE<N: const From<Exact>>: Information<N> = counted(Exact::int(1000000) * BYTE::<Exact>);

    /// One `Kilobyte` (kB), as an `Information`.
    pub const KILOBYTE<N: const From<Exact>>: Information<N> = counted(Exact::int(1000) * BYTE::<Exact>);

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

    /// One `YottabytePerSecond` (YB/s), as an `InformationRate`.
    pub const YOTTABYTE_PER_SECOND<N: const From<Exact>>: InformationRate<N> = counted(Exact::int(1000000000000000000000000) * BYTE_PER_SECOND::<Exact>);

    /// One `ZettabytePerSecond` (ZB/s), as an `InformationRate`.
    pub const ZETTABYTE_PER_SECOND<N: const From<Exact>>: InformationRate<N> = counted(Exact::int(1000000000000000000000) * BYTE_PER_SECOND::<Exact>);

    /// One `ExabytePerSecond` (EB/s), as an `InformationRate`.
    pub const EXABYTE_PER_SECOND<N: const From<Exact>>: InformationRate<N> = counted(Exact::int(1000000000000000000) * BYTE_PER_SECOND::<Exact>);

    /// One `PetabytePerSecond` (PB/s), as an `InformationRate`.
    pub const PETABYTE_PER_SECOND<N: const From<Exact>>: InformationRate<N> = counted(Exact::int(1000000000000000) * BYTE_PER_SECOND::<Exact>);

    /// One `TerabytePerSecond` (TB/s), as an `InformationRate`.
    pub const TERABYTE_PER_SECOND<N: const From<Exact>>: InformationRate<N> = counted(Exact::int(1000000000000) * BYTE_PER_SECOND::<Exact>);

    /// One `GigabytePerSecond` (GB/s), as an `InformationRate`.
    pub const GIGABYTE_PER_SECOND<N: const From<Exact>>: InformationRate<N> = counted(Exact::int(1000000000) * BYTE_PER_SECOND::<Exact>);

    /// One `MegabytePerSecond` (MB/s), as an `InformationRate`.
    pub const MEGABYTE_PER_SECOND<N: const From<Exact>>: InformationRate<N> = counted(Exact::int(1000000) * BYTE_PER_SECOND::<Exact>);

    /// One `KilobytePerSecond` (kB/s), as an `InformationRate`.
    pub const KILOBYTE_PER_SECOND<N: const From<Exact>>: InformationRate<N> = counted(Exact::int(1000) * BYTE_PER_SECOND::<Exact>);

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
#[unit(of = InformationRate<f64>)]
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
