//! Plain numbers: ratios and their units.

use crate::{Exact, ISQ, SIQuantity, counted};
use consts::*;
use yadc::Dimension;

pub type Dimensionless<N = f64> = SIQuantity<{ ISQ::DIMENSIONLESS }, N>;

/// The constants of this module: the base constants of its quantities
/// (re-exported), then one per multiple, each defined exactly from them -
/// `KILOMETRE` is `1000 * METRE` - and counted in the number type its use
/// asks for, rounded once: `let d: Length = KILOMETRE;` is an `f64` length,
/// `Length<f32>` an `f32` one, `KILOMETRE::<Exact>` the exact value. Each is
/// a unit (`KILOMETRE(3.0)`, `KILOMETRE.value(d)`), and they compose in
/// `const` items: `const LIMIT: Velocity = 50.0 * KILOMETRE / HOUR;`.
pub mod consts {
    use super::*;

    /// The number one: the unit of dimension one.
    pub use crate::base::ONE;

    /// One `Ratio`, as a `Dimensionless`.
    pub const RATIO<N: const From<Exact>>: Dimensionless<N> = counted(ONE::<Exact>);

    /// One `PartPerHundred` (parts per hundred), as a `Dimensionless`.
    pub const PART_PER_HUNDRED<N: const From<Exact>>: Dimensionless<N> = counted(Exact::dec(1, -2) * RATIO::<Exact>);

    /// One `Percent` (%), as a `Dimensionless`.
    pub const PERCENT<N: const From<Exact>>: Dimensionless<N> = counted(Exact::dec(1, -2) * RATIO::<Exact>);

    /// One `PartPerThousand` (parts per thousand), as a `Dimensionless`.
    pub const PART_PER_THOUSAND<N: const From<Exact>>: Dimensionless<N> = counted(Exact::dec(1, -3) * RATIO::<Exact>);

    /// One `PerMille` (‰), as a `Dimensionless`.
    pub const PER_MILLE<N: const From<Exact>>: Dimensionless<N> = counted(Exact::dec(1, -3) * RATIO::<Exact>);

    /// One `PartPerTenThousand` (parts per ten thousand), as a `Dimensionless`.
    pub const PART_PER_TEN_THOUSAND<N: const From<Exact>>: Dimensionless<N> = counted(Exact::dec(1, -4) * RATIO::<Exact>);

    /// One `BasisPoint` (bp), as a `Dimensionless`.
    pub const BASIS_POINT<N: const From<Exact>>: Dimensionless<N> = counted(Exact::dec(1, -4) * RATIO::<Exact>);

    /// One `PartPerMillion` (ppm), as a `Dimensionless`.
    pub const PART_PER_MILLION<N: const From<Exact>>: Dimensionless<N> = counted(Exact::dec(1, -6) * RATIO::<Exact>);

    /// One `PartPerBillion` (ppb), as a `Dimensionless`.
    pub const PART_PER_BILLION<N: const From<Exact>>: Dimensionless<N> = counted(Exact::dec(1, -9) * RATIO::<Exact>);

    /// One `PartPerTrillion` (ppt), as a `Dimensionless`.
    pub const PART_PER_TRILLION<N: const From<Exact>>: Dimensionless<N> = counted(Exact::dec(1, -12) * RATIO::<Exact>);

    /// One `PartPerQuadrillion` (ppq), as a `Dimensionless`.
    pub const PART_PER_QUADRILLION<N: const From<Exact>>: Dimensionless<N> = counted(Exact::dec(1, -15) * RATIO::<Exact>);
}

/// The units of `Dimensionless`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Dimensionless)]
pub enum DimensionlessUnit {
    #[default]
    #[unit("", "", by = RATIO)]
    Ratio,
    #[unit("parts per hundred", "parts per hundred", by = PART_PER_HUNDRED)]
    PartPerHundred,
    #[unit("%", "percent", by = PERCENT)]
    Percent,
    #[unit("parts per thousand", "parts per thousand", by = PART_PER_THOUSAND)]
    PartPerThousand,
    #[unit("‰", "per mille", by = PER_MILLE)]
    PerMille,
    #[unit("parts per ten thousand", "parts per ten thousand", by = PART_PER_TEN_THOUSAND)]
    PartPerTenThousand,
    #[unit("bp", "basis points", by = BASIS_POINT)]
    BasisPoint,
    #[unit("ppm", "parts per million", by = PART_PER_MILLION)]
    PartPerMillion,
    #[unit("ppb", "parts per billion", by = PART_PER_BILLION)]
    PartPerBillion,
    #[unit("ppt", "parts per trillion", by = PART_PER_TRILLION)]
    PartPerTrillion,
    #[unit("ppq", "parts per quadrillion", by = PART_PER_QUADRILLION)]
    PartPerQuadrillion,
}
