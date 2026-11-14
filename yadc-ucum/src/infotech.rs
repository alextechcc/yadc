//! The quantities of `infotech` and their units.

#[allow(unused_imports)]
use crate::*;

/// An amount of information, in `N`s of the unit `U`: by default its base unit, `BIT`.
pub type AmountOfInformation<N, const U: UcumUnit = { BIT.unit() }, const B: Base = BASE> =
    Qty<U, N, B>;

/// Bit (`bit`), a unit of `AmountOfInformation`: the coherent unit, a product of base units.
pub const BIT: Unit<{ UcumUnit::ONE }, BASE> = Unit::ONE;

/// Byte (`By`), a unit of `AmountOfInformation`: `n` of them is `n · 8` base units.
pub const BYTE: Unit<{ UcumUnit::scaled("By", Exact::int(8), BIT.unit()) }> = Unit::ONE;

/// A signal transmission rate, in `N`s of the unit `U`: by default its base unit, `BAUD`.
pub type SignalTransmissionRate<N, const U: UcumUnit = { BAUD.unit() }, const B: Base = BASE> =
    Qty<U, N, B>;

/// Baud (`Bd`), a unit of `SignalTransmissionRate`: the coherent unit, a product of base units.
pub const BAUD: Unit<{ UcumUnit::ONE.times(SECOND.unit(), -1) }, BASE> = Unit::ONE;
