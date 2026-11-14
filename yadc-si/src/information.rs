//! The quantities of `information` and their units.

#[allow(unused_imports)]
use crate::*;

/// An information, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Information<N, const U: SIUnit = { SIUnit::ONE }, const IS_BASE: bool = true> =
    SIQuantity<U, N, IS_BASE>;

/// Bits (`b`), a unit of `Information`: `n` of them is `n · 0.125` base units.
pub const BIT: SIQuantity<{ SIUnit::scaled("b", Exact::dec(125, -3), SIUnit::ONE) }, Exact, false> =
    SIQuantity::ONE;

/// Bytes (`B`), a unit of `Information`: the coherent unit, a product of base units.
pub const BYTE: SIQuantity<{ SIUnit::ONE }, Exact, true> = SIQuantity::ONE;

/// Octets (`o`), a unit of `Information`: the coherent unit, a product of base units.
pub const OCTET: SIQuantity<{ SIUnit::ONE }, Exact, true> = SIQuantity::ONE;

/// Nibbles (`nibble`), a unit of `Information`: `n` of them is `n · 0.5` base units.
pub const NIBBLE: SIQuantity<
    { SIUnit::scaled("nibble", Exact::dec(5, -1), SIUnit::ONE) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Crumbs (`crumb`), a unit of `Information`: `n` of them is `n · 0.25` base units.
pub const CRUMB: SIQuantity<
    { SIUnit::scaled("crumb", Exact::dec(25, -2), SIUnit::ONE) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Shannons (`Sh`), a unit of `Information`: `n` of them is `n · 0.125` base units.
pub const SHANNON: SIQuantity<
    { SIUnit::scaled("Sh", Exact::dec(125, -3), SIUnit::ONE) },
    Exact,
    false,
> = SIQuantity::ONE;

/// An information rate, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type InformationRate<
    N,
    const U: SIUnit = { SIUnit::ONE.times(SECOND.unit(), -1) },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;
