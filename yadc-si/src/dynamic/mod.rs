//! The dynamic half: what is chosen at run time - a quantity's unit
//! (`DynamicSIQuantity`), and a choice among a quantity's units (each module's unit
//! enums, `dynamic::mechanical::LengthUnit`).

use crate::{SIQuantity, SIUnit};

#[cfg(feature = "angular")]
pub mod angular;
#[cfg(feature = "chemistry")]
pub mod chemistry;
pub mod dimensionless;
#[cfg(feature = "electromagnetic")]
pub mod electromagnetic;
#[cfg(feature = "fluid")]
pub mod fluid;
#[cfg(feature = "information")]
pub mod information;
#[cfg(feature = "mechanical")]
pub mod mechanical;
#[cfg(feature = "photometric")]
pub mod photometric;
#[cfg(feature = "thermal")]
pub mod thermal;

/// A quantity of a unit chosen at run time: the dynamic version of
/// `SIQuantity<U, f64, IS_BASE>`, built with const_derive_more. Converting (`From`)
/// moves the number into `value` and records `U` in `unit` and `IS_BASE` in
/// `is_base`; `TryFrom<DynamicSIQuantity> for SIQuantity<U, f64, IS_BASE>` is the checked
/// way back (the error hands the original
/// back). A bare `f64` is the same value as a quantity of the plain number's
/// unit (`shared`).
///
/// Its arithmetic is every field's own: the unit's operator on `unit`, the
/// number's on `value`. Any of them may have no result - `None` where the
/// units don't combine: a sum of two units, a product with a unit that has a
/// zero or a log scale - so a calculation reads `(((a * b)? + c)? * 2.0)?`.
/// Values of one unit are ordered by number, `None` otherwise. A number in
/// another unit is `unit.from_base(other.to_base(value))`.
#[derive(Copy, Clone, PartialEq, Debug, const_derive_more::Dynamic)]
#[dynamic(SIQuantity<const U: SIUnit, f64, const IS_BASE: bool> => { unit: U, is_base: IS_BASE, value: .0 })]
#[dynamic(shared f64 => { unit: SIUnit::ONE, is_base: true, value: self })]
#[dynamic(impl(
    Mul -> Option<Self>,
    Div -> Option<Self>,
    Add -> Option<Self>,
    Sub -> Option<Self>,
    Mul<f64> -> Option<Self>,
    Div<f64> -> Option<Self>,
    PartialOrd,
))]
pub struct DynamicSIQuantity {
    pub unit: SIUnit,
    /// Whether `unit` is a base unit or a product of base units: operands
    /// must agree in it, as the static quantities' `IS_BASE` must.
    #[dynamic(same)]
    pub is_base: bool,
    pub value: f64,
}
