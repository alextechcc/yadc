use crate::{ISQ, StaticSIQuantity};
use yadc::{Dimension, Unit};

pub type Information = StaticSIQuantity<{ ISQ { ..ISQ::ZERO } }>;

#[derive(Unit, Copy, Clone)]
#[dim(Information)]
pub enum InformationUnit {
    #[linear("Yb", "yottabits", 1.25e+23)]
    Yottabit,
    #[linear("Zb", "zettabits", 1.25e+20)]
    Zettabit,
    #[linear("Eb", "exabits", 1.25e+17)]
    Exabit,
    #[linear("Pb", "petabits", 125000000000000.0)]
    Petabit,
    #[linear("Tb", "terabits", 125000000000.0)]
    Terabit,
    #[linear("Gb", "gigabits", 125000000.0)]
    Gigabit,
    #[linear("Mb", "megabits", 125000.0)]
    Megabit,
    #[linear("kb", "kilobits", 125.0)]
    Kilobit,
    #[linear("b", "bits", 0.125)]
    Bit,
    #[linear("YB", "yottabytes", 9.9999999999999998e+23)]
    Yottabyte,
    #[linear("ZB", "zettabytes", 1e+21)]
    Zettabyte,
    #[linear("EB", "exabytes", 1e+18)]
    Exabyte,
    #[linear("PB", "petabytes", 1000000000000000.0)]
    Petabyte,
    #[linear("TB", "terabytes", 1000000000000.0)]
    Terabyte,
    #[linear("GB", "gigabytes", 1000000000.0)]
    Gigabyte,
    #[linear("MB", "megabytes", 1000000.0)]
    Megabyte,
    #[linear("kB", "kilobytes", 1000.0)]
    Kilobyte,
    #[linear("B", "bytes", 1.0)]
    Byte,
    #[linear("o", "octets", 1.0)]
    Octet,
    #[linear("nibble", "nibbles", 0.5)]
    Nibble,
    #[linear("crumb", "crumbs", 0.25)]
    Crumb,
    #[linear("Sh", "shannons", 0.125)]
    Shannon,
}

pub type InformationRate = StaticSIQuantity<{ ISQ { s: -1, ..ISQ::ZERO } }>;

#[derive(Unit, Copy, Clone)]
#[dim(InformationRate)]
pub enum InformationRateUnit {
    #[linear("b/s", "bits per second", 0.125)]
    BitPerSecond,
    #[linear("YB/s", "yottabytes per second", 9.9999999999999998e+23)]
    YottabytePerSecond,
    #[linear("ZB/s", "zettabytes per second", 1e+21)]
    ZettabytePerSecond,
    #[linear("EB/s", "exabytes per second", 1e+18)]
    ExabytePerSecond,
    #[linear("PB/s", "petabytes per second", 1000000000000000.0)]
    PetabytePerSecond,
    #[linear("TB/s", "terabytes per second", 1000000000000.0)]
    TerabytePerSecond,
    #[linear("GB/s", "gigabytes per second", 1000000000.0)]
    GigabytePerSecond,
    #[linear("MB/s", "megabytes per second", 1000000.0)]
    MegabytePerSecond,
    #[linear("kB/s", "kilobytes per second", 1000.0)]
    KilobytePerSecond,
    #[linear("B/s", "bytes per second", 1.0)]
    BytePerSecond,
    #[linear("o/s", "octets per second", 1.0)]
    OctetPerSecond,
}
