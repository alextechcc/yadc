//! The units of `constants`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::constants::units::*;
use crate::constants::*;
use crate::dynamic::DynamicUcumUnit;
use core::marker::Destruct;
use core::ops::{Add, Div, Mul, Sub};

/// The units of `Acceleration`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Acceleration<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum AccelerationUnit {
    #[default]
    #[unit(StandardAccelerationOfFreeFall)]
    StandardAccelerationOfFreeFall,
}

impl From<AccelerationUnit> for DynamicUcumUnit {
    fn from(unit: AccelerationUnit) -> Self {
        match unit {
            AccelerationUnit::StandardAccelerationOfFreeFall => {
                StandardAccelerationOfFreeFall {}.into()
            }
        }
    }
}

/// The units of `Action`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Action<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ActionUnit {
    #[default]
    #[unit(PlanckConstant)]
    PlanckConstant,
}

impl From<ActionUnit> for DynamicUcumUnit {
    fn from(unit: ActionUnit) -> Self {
        match unit {
            ActionUnit::PlanckConstant => PlanckConstant {}.into(),
        }
    }
}

/// The units of `ElectricCharge`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCharge<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricChargeUnit {
    #[default]
    #[unit(ElementaryCharge)]
    ElementaryCharge,
}

impl From<ElectricChargeUnit> for DynamicUcumUnit {
    fn from(unit: ElectricChargeUnit) -> Self {
        match unit {
            ElectricChargeUnit::ElementaryCharge => ElementaryCharge {}.into(),
        }
    }
}

/// The units of `ElectricPermittivity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricPermittivity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricPermittivityUnit {
    #[default]
    #[unit(PermittivityOfVacuum)]
    PermittivityOfVacuum,
}

impl From<ElectricPermittivityUnit> for DynamicUcumUnit {
    fn from(unit: ElectricPermittivityUnit) -> Self {
        match unit {
            ElectricPermittivityUnit::PermittivityOfVacuum => PermittivityOfVacuum {}.into(),
        }
    }
}

/// The units of `Force`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Force<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ForceUnit {
    #[default]
    #[unit(GramForce)]
    GramForce,
    #[unit(PoundForce)]
    PoundForce,
}

impl From<ForceUnit> for DynamicUcumUnit {
    fn from(unit: ForceUnit) -> Self {
        match unit {
            ForceUnit::GramForce => GramForce {}.into(),
            ForceUnit::PoundForce => PoundForce {}.into(),
        }
    }
}

/// The units of `Length`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum LengthUnit {
    #[default]
    #[unit(LightYear)]
    LightYear,
}

impl From<LengthUnit> for DynamicUcumUnit {
    fn from(unit: LengthUnit) -> Self {
        match unit {
            LengthUnit::LightYear => LightYear {}.into(),
        }
    }
}

/// The units of `MagneticPermeability`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticPermeability<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MagneticPermeabilityUnit {
    #[default]
    #[unit(PermeabilityOfVacuum)]
    PermeabilityOfVacuum,
}

impl From<MagneticPermeabilityUnit> for DynamicUcumUnit {
    fn from(unit: MagneticPermeabilityUnit) -> Self {
        match unit {
            MagneticPermeabilityUnit::PermeabilityOfVacuum => PermeabilityOfVacuum {}.into(),
        }
    }
}

/// The units of `Mass`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Mass<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MassUnit {
    #[default]
    #[unit(ElectronMass)]
    ElectronMass,
    #[unit(ProtonMass)]
    ProtonMass,
}

impl From<MassUnit> for DynamicUcumUnit {
    fn from(unit: MassUnit) -> Self {
        match unit {
            MassUnit::ElectronMass => ElectronMass {}.into(),
            MassUnit::ProtonMass => ProtonMass {}.into(),
        }
    }
}

/// The units of `Pressure`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Pressure<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum PressureUnit {
    #[default]
    #[unit(StandardAtmosphere)]
    StandardAtmosphere,
}

impl From<PressureUnit> for DynamicUcumUnit {
    fn from(unit: PressureUnit) -> Self {
        match unit {
            PressureUnit::StandardAtmosphere => StandardAtmosphere {}.into(),
        }
    }
}

/// The units of `UnclassifiedM2GPerS2K`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = UnclassifiedM2GPerS2K<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum UnclassifiedM2GPerS2KUnit {
    #[default]
    #[unit(BoltzmannConstant)]
    BoltzmannConstant,
}

impl From<UnclassifiedM2GPerS2KUnit> for DynamicUcumUnit {
    fn from(unit: UnclassifiedM2GPerS2KUnit) -> Self {
        match unit {
            UnclassifiedM2GPerS2KUnit::BoltzmannConstant => BoltzmannConstant {}.into(),
        }
    }
}

/// The units of `UnclassifiedM3PerS2G`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = UnclassifiedM3PerS2G<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum UnclassifiedM3PerS2GUnit {
    #[default]
    #[unit(NewtonianConstantOfGravitation)]
    NewtonianConstantOfGravitation,
}

impl From<UnclassifiedM3PerS2GUnit> for DynamicUcumUnit {
    fn from(unit: UnclassifiedM3PerS2GUnit) -> Self {
        match unit {
            UnclassifiedM3PerS2GUnit::NewtonianConstantOfGravitation => {
                NewtonianConstantOfGravitation {}.into()
            }
        }
    }
}

/// The units of `Velocity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Velocity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum VelocityUnit {
    #[default]
    #[unit(VelocityOfLight)]
    VelocityOfLight,
}

impl From<VelocityUnit> for DynamicUcumUnit {
    fn from(unit: VelocityUnit) -> Self {
        match unit {
            VelocityUnit::VelocityOfLight => VelocityOfLight {}.into(),
        }
    }
}
