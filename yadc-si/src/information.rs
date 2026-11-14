//! The quantities of `information` and their units.

#[allow(unused_imports)]
use crate::*;

/// An information, in `N`s of the unit `U`: by default its base unit, `BYTE`.
pub type Information<N, const U: SIUnit = { BYTE.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Bytes (`B`), a unit of `Information`: the coherent unit, a product of base units.
pub const BYTE: Unit<{ SIUnit::ONE }, BASE> = Unit::ONE;

/// Bits (`b`), a unit of `Information`: `n` of them is `n · 0.125` base units.
pub const BIT: Unit<{ SIUnit::scaled("b", Exact::dec(125, -3), BYTE.unit()) }> = Unit::ONE;

/// Octets (`o`), a unit of `Information`: `n` of them is `n · 1.0` base units.
pub const OCTET: Unit<{ SIUnit::scaled("o", Exact::int(1), BYTE.unit()) }> = Unit::ONE;

/// Nibbles (`nibble`), a unit of `Information`: `n` of them is `n · 0.5` base units.
pub const NIBBLE: Unit<{ SIUnit::scaled("nibble", Exact::dec(5, -1), BYTE.unit()) }> = Unit::ONE;

/// Crumbs (`crumb`), a unit of `Information`: `n` of them is `n · 0.25` base units.
pub const CRUMB: Unit<{ SIUnit::scaled("crumb", Exact::dec(25, -2), BYTE.unit()) }> = Unit::ONE;

/// Shannons (`Sh`), a unit of `Information`: `n` of them is `n · 0.125` base units.
pub const SHANNON: Unit<{ SIUnit::scaled("Sh", Exact::dec(125, -3), BYTE.unit()) }> = Unit::ONE;

/// An information rate, in `N`s of the unit `U`: by default its base unit, `PER_SECOND`.
pub type InformationRate<N, const U: SIUnit = { PER_SECOND.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// The base unit of `InformationRate`: a product of base units.
pub const PER_SECOND: Unit<{ SIUnit::ONE.times(SECOND.unit(), -1) }, BASE> = Unit::ONE;
