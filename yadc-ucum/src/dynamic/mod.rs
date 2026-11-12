//! The dynamic half: what is chosen at run time - a quantity's dimension
//! (`DynQty`), and its unit (each module's unit enums,
//! `dynamic::mechanical::LengthUnit`).

use crate::{Dim, Qty};
use yadc::Dimension;

#[cfg(feature = "british")]
pub mod british;
#[cfg(feature = "cgs")]
pub mod cgs;
#[cfg(feature = "clinical")]
pub mod clinical;
#[cfg(feature = "constants")]
pub mod constants;
pub mod dimensionless;
#[cfg(feature = "heat")]
pub mod heat;
#[cfg(feature = "infotech")]
pub mod infotech;
#[cfg(feature = "intcust")]
pub mod intcust;
#[cfg(feature = "iso1000")]
pub mod iso1000;
#[cfg(feature = "misc")]
pub mod misc;
#[cfg(feature = "si")]
pub mod si;
#[cfg(feature = "us")]
pub mod us;

/// A quantity of any dimension, decided at run time: the dynamic version of
/// `Qty<D, f64>`, built with dynamickit. Converting (`From`) moves the number into
/// `value` and records `D` in `dim`; `TryFrom<DynQty> for Qty<D, f64>` is the
/// checked way back (the error hands the original back). A bare `f64` is the
/// same value as a dimensionless quantity (`shared`). Numbers still go through
/// units, by way of the static quantity: `DynQty::from(LengthUnit::Mile.quantity(3.0))`,
/// then `Mass::<f64>::try_from(d).map(|m| MassUnit::Kilogram.value(m))`.
///
/// Its arithmetic is every field's own: the dimension's operator on `dim`, the
/// number's on `value`. A product or quotient always exists (`Some`); a sum or a
/// scaling may not (`None` where the dimensions don't combine), so a calculation
/// reads `(((a * b)? + c)? * 2.0)?`. Values of one dimension are ordered by number,
/// `None` otherwise. A root or power with a run-time exponent is the
/// dimension's and the number's (`None` where the dimension has none).
/// Scaling by a number is the dimension's own too: its `* f64` and `/ f64`,
/// which `#[derive(Dimension)]` writes.
#[derive(Copy, Clone, PartialEq, Debug, dynamickit::Dynamic)]
#[dynamic(Qty<const D: Dim, f64> => { dim: D, value: .0 })]
#[dynamic(shared f64 => { dim: Dim::DIMENSIONLESS, value: self })]
#[dynamic(impl(
    Mul -> Option<Self>,
    Div -> Option<Self>,
    Add -> Option<Self>,
    Sub -> Option<Self>,
    Mul<f64> -> Option<Self>,
    Div<f64> -> Option<Self>,
    PartialOrd,
    yadc::PartialRoot::partial_root(self, n: i8) -> Option<Self>,
    yadc::PartialPowi::partial_powi(self, n: i8) -> Option<Self>,
))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DynQty {
    pub(crate) dim: Dim,
    pub(crate) value: f64,
}
