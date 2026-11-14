//! The quantities of `infotech` and their units.

#[allow(unused_imports)]
use crate::*;

/// An amount of information, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type AmountOfInformation<N, const U: UcumUnit = { UcumUnit::ONE }, const B: Base = BASE> =
    Qty<U, N, B>;

/// Bit (`bit`), a unit of `AmountOfInformation`: the coherent unit, a product of base units.
pub const BIT: Qty<{ UcumUnit::ONE }, Exact, BASE> = Qty::ONE;

/// Byte (`By`), a unit of `AmountOfInformation`: `n` of them is `n · 8` base units.
pub const BYTE: Qty<{ UcumUnit::scaled("By", Exact::int(8), UcumUnit::ONE) }, Exact, NOT_BASE> =
    Qty::ONE;

/// A signal transmission rate, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type SignalTransmissionRate<
    N,
    const U: UcumUnit = { UcumUnit::ONE.times(SECOND.unit(), -1) },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Baud (`Bd`), a unit of `SignalTransmissionRate`: the coherent unit, a product of base units.
pub const BAUD: Qty<{ UcumUnit::ONE.times(SECOND.unit(), -1) }, Exact, BASE> = Qty::ONE;
