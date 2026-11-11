//! The serde add-on: the UCUM string form of a dimension's kind. The derives
//! `UcumSerialize` / `UcumDeserialize` (dimensions) and `UcumKind` (kinds)
//! use it; a system without serde never needs this module.

/// UCUM serialization mapping for a dimension's kind.
///
/// Derived by `#[derive(UcumKind)]`, kept separate from the kind's algebra so
/// that carries no wire-format knowledge. Gate the derive behind your
/// serde feature alongside [`UcumSerialize`](macro@crate::UcumSerialize)
/// / [`UcumDeserialize`](macro@crate::UcumDeserialize), which are its
/// only callers.
///
/// [`ucum_prefix`](Self::ucum_prefix) and [`from_ucum_prefix`](Self::from_ucum_prefix)
/// are inverses (a variant declared `#[ucum(prefix = "..")]` <-> that
/// prefix), so the round-trip mapping lives in exactly one place. A kind
/// with no prefix is written as a `{Name}` annotation (`"K{Affine}"`).
pub trait UcumKind: Copy {
    /// The short name of this kind (its variant identifier), e.g. `"Affine"`.
    fn name(self) -> &'static str;
    /// Inverse of [`name`](Self::name): the kind with this short name, or
    /// `None`. A UCUM string's `{Affine}` annotation is read back through it,
    /// so the kind's own `serde` representation never matters.
    fn from_name(name: &str) -> Option<Self>;
    /// Returns the UCUM unit prefix for a prefix-bearing kind (one declared
    /// `#[ucum(prefix = "..")]`), or `None` for a kind that needs no prefix
    /// in a UCUM string.
    fn ucum_prefix(self) -> Option<&'static str>;
    /// Inverse of [`ucum_prefix`](Self::ucum_prefix): maps a UCUM prefix token
    /// back to its kind, or `None` if the token names no prefix-bearing kind.
    fn from_ucum_prefix(prefix: &str) -> Option<Self>;
}
