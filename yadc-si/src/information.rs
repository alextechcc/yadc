use crate::{ISQ, StaticSIQuantity};
use yadc::{Dimension, Unit};

pub type Information = StaticSIQuantity<{ ISQ { ..ISQ::ZERO } }>;

#[derive(Copy, Clone, Unit)]
#[dim(Information)]
pub enum InformationUnit {
    #[unit(1.25e+23, "Yb", "yottabits")]
    Yottabit,
    #[unit(1.25e+20, "Zb", "zettabits")]
    Zettabit,
    #[unit(1.25e+17, "Eb", "exabits")]
    Exabit,
    #[unit(125000000000000.0, "Pb", "petabits")]
    Petabit,
    #[unit(125000000000.0, "Tb", "terabits")]
    Terabit,
    #[unit(125000000.0, "Gb", "gigabits")]
    Gigabit,
    #[unit(125000.0, "Mb", "megabits")]
    Megabit,
    #[unit(125.0, "kb", "kilobits")]
    Kilobit,
    #[unit(0.125, "b", "bits")]
    Bit,
    #[unit(9.9999999999999998e+23, "YB", "yottabytes")]
    Yottabyte,
    #[unit(1e+21, "ZB", "zettabytes")]
    Zettabyte,
    #[unit(1e+18, "EB", "exabytes")]
    Exabyte,
    #[unit(1000000000000000.0, "PB", "petabytes")]
    Petabyte,
    #[unit(1000000000000.0, "TB", "terabytes")]
    Terabyte,
    #[unit(1000000000.0, "GB", "gigabytes")]
    Gigabyte,
    #[unit(1000000.0, "MB", "megabytes")]
    Megabyte,
    #[unit(1000.0, "kB", "kilobytes")]
    Kilobyte,
    #[unit(1.0, "B", "bytes")]
    Byte,
    #[unit(1.0, "o", "octets")]
    Octet,
    #[unit(0.5, "nibble", "nibbles")]
    Nibble,
    #[unit(0.25, "crumb", "crumbs")]
    Crumb,
    #[unit(0.125, "Sh", "shannons")]
    Shannon,
}

pub type InformationRate = StaticSIQuantity<{ ISQ { s: -1, ..ISQ::ZERO } }>;

#[derive(Copy, Clone, Unit)]
#[dim(InformationRate)]
pub enum InformationRateUnit {
    #[unit(0.125, "b/s", "bits per second")]
    BitPerSecond,
    #[unit(9.9999999999999998e+23, "YB/s", "yottabytes per second")]
    YottabytePerSecond,
    #[unit(1e+21, "ZB/s", "zettabytes per second")]
    ZettabytePerSecond,
    #[unit(1e+18, "EB/s", "exabytes per second")]
    ExabytePerSecond,
    #[unit(1000000000000000.0, "PB/s", "petabytes per second")]
    PetabytePerSecond,
    #[unit(1000000000000.0, "TB/s", "terabytes per second")]
    TerabytePerSecond,
    #[unit(1000000000.0, "GB/s", "gigabytes per second")]
    GigabytePerSecond,
    #[unit(1000000.0, "MB/s", "megabytes per second")]
    MegabytePerSecond,
    #[unit(1000.0, "kB/s", "kilobytes per second")]
    KilobytePerSecond,
    #[unit(1.0, "B/s", "bytes per second")]
    BytePerSecond,
    #[unit(1.0, "o/s", "octets per second")]
    OctetPerSecond,
}
