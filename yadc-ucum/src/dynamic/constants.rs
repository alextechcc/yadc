//! The units of `constants`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::constants::units::*;
use crate::constants::*;
use crate::dynamic::DynamicUcumUnit;
use crate::{Exact, Qty, Unit};
use core::ops::{Add, Div, Mul, Sub};

/// The units of `Acceleration`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<ACCELERATION, N>, N> {
        fn quantity(&self, value: N) -> Qty<ACCELERATION, N>;
        fn value(&self, quantity: Qty<ACCELERATION, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum AccelerationUnit {
    #[default]
    #[value(STANDARD_ACCELERATION_OF_FREE_FALL)]
    StandardAccelerationOfFreeFall,
}

/// The units of `Action`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<ACTION, N>, N> {
        fn quantity(&self, value: N) -> Qty<ACTION, N>;
        fn value(&self, quantity: Qty<ACTION, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ActionUnit {
    #[default]
    #[value(PLANCK_CONSTANT)]
    PlanckConstant,
}

/// The units of `ElectricCharge`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<ELECTRIC_CHARGE, N>, N> {
        fn quantity(&self, value: N) -> Qty<ELECTRIC_CHARGE, N>;
        fn value(&self, quantity: Qty<ELECTRIC_CHARGE, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ElectricChargeUnit {
    #[default]
    #[value(ELEMENTARY_CHARGE)]
    ElementaryCharge,
}

/// The units of `ElectricPermittivity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<ELECTRIC_PERMITTIVITY, N>, N> {
        fn quantity(&self, value: N) -> Qty<ELECTRIC_PERMITTIVITY, N>;
        fn value(&self, quantity: Qty<ELECTRIC_PERMITTIVITY, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ElectricPermittivityUnit {
    #[default]
    #[value(PERMITTIVITY_OF_VACUUM)]
    PermittivityOfVacuum,
}

/// The units of `Force`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<FORCE, N>, N> {
        fn quantity(&self, value: N) -> Qty<FORCE, N>;
        fn value(&self, quantity: Qty<FORCE, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ForceUnit {
    #[default]
    #[value(GRAM_FORCE)]
    GramForce,
    #[value(POUND_FORCE)]
    PoundForce,
}

/// The units of `Length`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<LENGTH, N>, N> {
        fn quantity(&self, value: N) -> Qty<LENGTH, N>;
        fn value(&self, quantity: Qty<LENGTH, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum LengthUnit {
    #[default]
    #[value(LIGHT_YEAR)]
    LightYear,
}

/// The units of `MagneticPermeability`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<MAGNETIC_PERMEABILITY, N>, N> {
        fn quantity(&self, value: N) -> Qty<MAGNETIC_PERMEABILITY, N>;
        fn value(&self, quantity: Qty<MAGNETIC_PERMEABILITY, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MagneticPermeabilityUnit {
    #[default]
    #[value(PERMEABILITY_OF_VACUUM)]
    PermeabilityOfVacuum,
}

/// The units of `Mass`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<MASS, N>, N> {
        fn quantity(&self, value: N) -> Qty<MASS, N>;
        fn value(&self, quantity: Qty<MASS, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MassUnit {
    #[default]
    #[value(ELECTRON_MASS)]
    ElectronMass,
    #[value(PROTON_MASS)]
    ProtonMass,
}

/// The units of `Pressure`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<PRESSURE, N>, N> {
        fn quantity(&self, value: N) -> Qty<PRESSURE, N>;
        fn value(&self, quantity: Qty<PRESSURE, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum PressureUnit {
    #[default]
    #[value(STANDARD_ATMOSPHERE)]
    StandardAtmosphere,
}

/// The units of `UnclassifiedM2GPerS2K`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<UNCLASSIFIED_M2_G_PER_S2_K, N>, N> {
        fn quantity(&self, value: N) -> Qty<UNCLASSIFIED_M2_G_PER_S2_K, N>;
        fn value(&self, quantity: Qty<UNCLASSIFIED_M2_G_PER_S2_K, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum UnclassifiedM2GPerS2KUnit {
    #[default]
    #[value(BOLTZMANN_CONSTANT)]
    BoltzmannConstant,
}

/// The units of `UnclassifiedM3PerS2G`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<UNCLASSIFIED_M3_PER_S2_G, N>, N> {
        fn quantity(&self, value: N) -> Qty<UNCLASSIFIED_M3_PER_S2_G, N>;
        fn value(&self, quantity: Qty<UNCLASSIFIED_M3_PER_S2_G, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum UnclassifiedM3PerS2GUnit {
    #[default]
    #[value(NEWTONIAN_CONSTANT_OF_GRAVITATION)]
    NewtonianConstantOfGravitation,
}

/// The units of `Velocity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<VELOCITY, N>, N> {
        fn quantity(&self, value: N) -> Qty<VELOCITY, N>;
        fn value(&self, quantity: Qty<VELOCITY, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum VelocityUnit {
    #[default]
    #[value(VELOCITY_OF_LIGHT)]
    VelocityOfLight,
}
