//! The units of `photometric`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

#[allow(unused_imports)]
use crate::photometric::*;
#[allow(unused_imports)]
use crate::*;

/// The units of `Luminance`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum LuminanceUnit {
    #[default]
    #[value(SIUnit::ONE.times(METRE.unit(), -2).times(CANDELA.unit(), 1))]
    CandelaPerMetreSquared,
    #[value(FOOTLAMBERT.unit())]
    Footlambert,
    #[value(LAMBERT.unit())]
    Lambert,
    #[value(STILB.unit())]
    Stilb,
}

/// The units of `LuminousIntensity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum LuminousIntensityUnit {
    #[default]
    #[value(CANDELA.unit())]
    Candela,
}

/// The units of `Illuminance`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum IlluminanceUnit {
    #[default]
    #[value(LUX.unit())]
    Lux,
    #[value(PHOT.unit())]
    Phot,
    #[value(FOOTCANDLE.unit())]
    Footcandle,
}
