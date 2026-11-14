//! The dynamic half: what is chosen at run time - a quantity's dimension
//! (`DynamicSIQuantity`), a unit of any dimension (`DynamicSIUnit`), and a choice among a
//! quantity's units (each module's unit enums, `dynamic::mechanical::LengthUnit`).

use crate::{ISQ, SIQuantity, SIUnit, SIUnitConsts};
use yadc::{Dimension, Unit};

#[cfg(feature = "angular")]
pub mod angular;
#[cfg(feature = "chemistry")]
pub mod chemistry;
#[cfg(feature = "density")]
pub mod density;
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

/// A quantity of any dimension, decided at run time: the dynamic version of
/// `SIQuantity<D, f64>`, built with dynamickit. Converting (`From`) moves the number into
/// `value` and records `D` in `dim`; `TryFrom<DynamicSIQuantity> for SIQuantity<D, f64>` is the
/// checked way back (the error hands the original back). A bare `f64` is the
/// same value as a dimensionless quantity (`shared`). Numbers go through
/// units: a `DynamicSIUnit` reads and builds it directly.
///
/// Its arithmetic is every field's own: the dimension's operator on `dim`, the
/// number's on `value`. Any of them may have no result - `None` where the dimensions don't combine:
/// a sum of two dimensions, a product with a point or a level - so a calculation reads
/// `(((a * b)? + c)? * 2.0)?`. Values of one dimension are ordered by number,
/// `None` otherwise. A root or power with a run-time exponent is the
/// dimension's and the number's (`None` where the dimension has none).
/// Scaling by a number is the dimension's own too: its `* f64` and `/ f64`,
/// which `#[derive(Dimension)]` writes.
#[derive(Copy, Clone, PartialEq, Debug, dynamickit::Dynamic)]
#[dynamic(SIQuantity<const D: ISQ, f64> => { dim: D, value: .0 })]
#[dynamic(shared f64 => { dim: ISQ::DIMENSIONLESS, value: self })]
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
pub struct DynamicSIQuantity {
    pub(crate) dim: ISQ,
    pub(crate) value: f64,
}

/// A unit of any dimension, decided at run time: the dynamic version of
/// `SIUnit<D, C>`, built with dynamickit - `From` records `D` and `C`,
/// `TryFrom<DynamicSIUnit> for SIUnit<D, C>` is the checked way back. A
/// module's unit enum converts to one (`From<LengthUnit>`).
///
/// A unit of `Option<DynamicSIQuantity>` counted in `Option<f64>`, the way dynamic
/// arithmetic results are: `quantity` builds a quantity of its dimension,
/// and `value` reads one of its dimension - `None` for another, or for
/// `None`. So a reading takes a calculation's result as it comes:
/// `unit.value(a * b)`.
#[derive(Copy, Clone, PartialEq, Debug, dynamickit::Dynamic)]
#[dynamic(SIUnit<const D: ISQ, const C: SIUnitConsts> => { dim: D, consts: C })]
pub struct DynamicSIUnit {
    pub dim: ISQ,
    pub consts: SIUnitConsts,
}

impl Unit<Option<DynamicSIQuantity>, Option<f64>> for DynamicSIUnit {
    fn quantity(&self, value: Option<f64>) -> Option<DynamicSIQuantity> {
        let (size, zero) = (f64::from(self.consts.size), f64::from(self.consts.zero));
        Some(DynamicSIQuantity {
            dim: self.dim,
            value: value? * size + zero,
        })
    }
    fn value(&self, quantity: Option<DynamicSIQuantity>) -> Option<f64> {
        let q = quantity?;
        let (size, zero) = (f64::from(self.consts.size), f64::from(self.consts.zero));
        (q.dim == self.dim).then(|| (q.value - zero) / size)
    }
}

impl core::fmt::Display for DynamicSIUnit {
    /// `{}` the symbol, `{:#}` the name; width, fill and alignment apply.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.pad(if f.alternate() {
            self.consts.name
        } else {
            self.consts.symbol
        })
    }
}
