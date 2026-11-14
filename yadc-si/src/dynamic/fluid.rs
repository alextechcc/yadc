//! The units of `fluid`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

#[allow(unused_imports)]
use crate::fluid::*;
#[allow(unused_imports)]
use crate::*;

/// The units of `DynamicViscosity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum DynamicViscosityUnit {
    #[default]
    #[value(PASCAL_SECOND.unit())]
    PascalSecond,
    #[value(POISE.unit())]
    Poise,
}

/// The units of `KinematicViscosity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum KinematicViscosityUnit {
    #[default]
    #[value(KINEMATIC_VISCOSITY_STOKES.unit())]
    Stokes,
}

/// The units of `DiffusionCoefficient`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum DiffusionCoefficientUnit {
    #[default]
    #[value(DIFFUSION_COEFFICIENT_STOKES.unit())]
    Stokes,
}
