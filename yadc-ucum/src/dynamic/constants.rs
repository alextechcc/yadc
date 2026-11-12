//! The units of `constants`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::constants::consts::*;
use crate::constants::*;
use core::marker::Destruct;
use yadc::{Unit, num_traits::Num};

/// The units of `Acceleration`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Acceleration<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Acceleration<N>: [const] Unit<Acceleration<N>, N> + [const] Destruct)]
pub enum AccelerationUnit {
    #[default]
    #[unit("[g]", "standard acceleration of free fall", by = Acceleration::<N>::from(STANDARD_ACCELERATION_OF_FREE_FALL))]
    StandardAccelerationOfFreeFall,
}

/// The units of `Action`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Action<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Action<N>: [const] Unit<Action<N>, N> + [const] Destruct)]
pub enum ActionUnit {
    #[default]
    #[unit("[h]", "Planck constant", by = Action::<N>::from(PLANCK_CONSTANT))]
    PlanckConstant,
}

/// The units of `ElectricCharge`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCharge<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricCharge<N>: [const] Unit<ElectricCharge<N>, N> + [const] Destruct)]
pub enum ElectricChargeUnit {
    #[default]
    #[unit("[e]", "elementary charge", by = ElectricCharge::<N>::from(ELEMENTARY_CHARGE))]
    ElementaryCharge,
}

/// The units of `ElectricPermittivity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricPermittivity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricPermittivity<N>: [const] Unit<ElectricPermittivity<N>, N> + [const] Destruct)]
pub enum ElectricPermittivityUnit {
    #[default]
    #[unit("[eps_0]", "permittivity of vacuum", by = ElectricPermittivity::<N>::from(PERMITTIVITY_OF_VACUUM))]
    PermittivityOfVacuum,
}

/// The units of `Force`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Force<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Force<N>: [const] Unit<Force<N>, N> + [const] Destruct)]
pub enum ForceUnit {
    #[default]
    #[unit("gf", "gram-force", by = Force::<N>::from(GRAM_FORCE))]
    GramForce,
    #[unit("[lbf_av]", "pound force", by = Force::<N>::from(POUND_FORCE))]
    PoundForce,
}

/// The units of `Length`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Length<N>: [const] Unit<Length<N>, N> + [const] Destruct)]
pub enum LengthUnit {
    #[default]
    #[unit("[ly]", "light-year", by = Length::<N>::from(LIGHT_YEAR))]
    LightYear,
}

/// The units of `MagneticPermeability`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticPermeability<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MagneticPermeability<N>: [const] Unit<MagneticPermeability<N>, N> + [const] Destruct)]
pub enum MagneticPermeabilityUnit {
    #[default]
    #[unit("[mu_0]", "permeability of vacuum", by = MagneticPermeability::<N>::from(PERMEABILITY_OF_VACUUM))]
    PermeabilityOfVacuum,
}

/// The units of `Mass`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Mass<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Mass<N>: [const] Unit<Mass<N>, N> + [const] Destruct)]
pub enum MassUnit {
    #[default]
    #[unit("[m_e]", "electron mass", by = Mass::<N>::from(ELECTRON_MASS))]
    ElectronMass,
    #[unit("[m_p]", "proton mass", by = Mass::<N>::from(PROTON_MASS))]
    ProtonMass,
}

/// The units of `Pressure`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Pressure<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Pressure<N>: [const] Unit<Pressure<N>, N> + [const] Destruct)]
pub enum PressureUnit {
    #[default]
    #[unit("atm", "standard atmosphere", by = Pressure::<N>::from(STANDARD_ATMOSPHERE))]
    StandardAtmosphere,
}

/// The units of `UnclassifiedM2GPerS2K`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = UnclassifiedM2GPerS2K<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where UnclassifiedM2GPerS2K<N>: [const] Unit<UnclassifiedM2GPerS2K<N>, N> + [const] Destruct)]
pub enum UnclassifiedM2GPerS2KUnit {
    #[default]
    #[unit("[k]", "Boltzmann constant", by = UnclassifiedM2GPerS2K::<N>::from(BOLTZMANN_CONSTANT))]
    BoltzmannConstant,
}

/// The units of `UnclassifiedM3PerS2G`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = UnclassifiedM3PerS2G<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where UnclassifiedM3PerS2G<N>: [const] Unit<UnclassifiedM3PerS2G<N>, N> + [const] Destruct)]
pub enum UnclassifiedM3PerS2GUnit {
    #[default]
    #[unit("[G]", "Newtonian constant of gravitation", by = UnclassifiedM3PerS2G::<N>::from(NEWTONIAN_CONSTANT_OF_GRAVITATION))]
    NewtonianConstantOfGravitation,
}

/// The units of `Velocity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Velocity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Velocity<N>: [const] Unit<Velocity<N>, N> + [const] Destruct)]
pub enum VelocityUnit {
    #[default]
    #[unit("[c]", "velocity of light", by = Velocity::<N>::from(VELOCITY_OF_LIGHT))]
    VelocityOfLight,
}
