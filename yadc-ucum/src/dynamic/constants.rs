//! The units of `constants`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

#[allow(unused_imports)]
use crate::constants::*;
#[allow(unused_imports)]
use crate::*;

/// The units of `Acceleration`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum AccelerationUnit {
    #[default]
    #[value(STANDARD_ACCELERATION_OF_FREE_FALL.unit())]
    StandardAccelerationOfFreeFall,
}

/// The units of `Action`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum ActionUnit {
    #[default]
    #[value(PLANCK_CONSTANT.unit())]
    PlanckConstant,
}

/// The units of `ElectricCharge`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum ElectricChargeUnit {
    #[default]
    #[value(ELEMENTARY_CHARGE.unit())]
    ElementaryCharge,
}

/// The units of `ElectricPermittivity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum ElectricPermittivityUnit {
    #[default]
    #[value(PERMITTIVITY_OF_VACUUM.unit())]
    PermittivityOfVacuum,
}

/// The units of `Force`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum ForceUnit {
    #[default]
    #[value(GRAM_FORCE.unit())]
    GramForce,
    #[value(POUND_FORCE.unit())]
    PoundForce,
}

/// The units of `Length`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum LengthUnit {
    #[default]
    #[value(LIGHT_YEAR.unit())]
    LightYear,
}

/// The units of `MagneticPermeability`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum MagneticPermeabilityUnit {
    #[default]
    #[value(PERMEABILITY_OF_VACUUM.unit())]
    PermeabilityOfVacuum,
}

/// The units of `Mass`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum MassUnit {
    #[default]
    #[value(ELECTRON_MASS.unit())]
    ElectronMass,
    #[value(PROTON_MASS.unit())]
    ProtonMass,
}

/// The units of `Pressure`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum PressureUnit {
    #[default]
    #[value(STANDARD_ATMOSPHERE.unit())]
    StandardAtmosphere,
}

/// The units of `UnclassifiedM2GPerS2K`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum UnclassifiedM2GPerS2KUnit {
    #[default]
    #[value(BOLTZMANN_CONSTANT.unit())]
    BoltzmannConstant,
}

/// The units of `UnclassifiedM3PerS2G`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum UnclassifiedM3PerS2GUnit {
    #[default]
    #[value(NEWTONIAN_CONSTANT_OF_GRAVITATION.unit())]
    NewtonianConstantOfGravitation,
}

/// The units of `Velocity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(UcumUnit))]
pub enum VelocityUnit {
    #[default]
    #[value(VELOCITY_OF_LIGHT.unit())]
    VelocityOfLight,
}
