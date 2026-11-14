//! The units of `cgs`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::cgs::units::*;
use crate::cgs::*;
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
    #[value(GAL)]
    Gal,
}

/// The units of `Brightness`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<BRIGHTNESS, N>, N> {
        fn quantity(&self, value: N) -> Qty<BRIGHTNESS, N>;
        fn value(&self, quantity: Qty<BRIGHTNESS, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum BrightnessUnit {
    #[default]
    #[value(LAMBERT)]
    Lambert,
}

/// The units of `DoseEquivalent`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<DOSE_EQUIVALENT, N>, N> {
        fn quantity(&self, value: N) -> Qty<DOSE_EQUIVALENT, N>;
        fn value(&self, quantity: Qty<DOSE_EQUIVALENT, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum DoseEquivalentUnit {
    #[default]
    #[value(RADIATION_EQUIVALENT_MAN)]
    RadiationEquivalentMan,
}

/// The units of `DynamicViscosity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<DYNAMIC_VISCOSITY, N>, N> {
        fn quantity(&self, value: N) -> Qty<DYNAMIC_VISCOSITY, N>;
        fn value(&self, quantity: Qty<DYNAMIC_VISCOSITY, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum DynamicViscosityUnit {
    #[default]
    #[value(POISE)]
    Poise,
}

/// The units of `ElectricCurrent`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<ELECTRIC_CURRENT, N>, N> {
        fn quantity(&self, value: N) -> Qty<ELECTRIC_CURRENT, N>;
        fn value(&self, quantity: Qty<ELECTRIC_CURRENT, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ElectricCurrentUnit {
    #[default]
    #[value(BIOT)]
    Biot,
}

/// The units of `Energy`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<ENERGY, N>, N> {
        fn quantity(&self, value: N) -> Qty<ENERGY, N>;
        fn value(&self, quantity: Qty<ENERGY, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum EnergyUnit {
    #[default]
    #[value(ERG)]
    Erg,
}

/// The units of `EnergyDose`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<ENERGY_DOSE, N>, N> {
        fn quantity(&self, value: N) -> Qty<ENERGY_DOSE, N>;
        fn value(&self, quantity: Qty<ENERGY_DOSE, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum EnergyDoseUnit {
    #[default]
    #[value(RADIATION_ABSORBED_DOSE)]
    RadiationAbsorbedDose,
}

/// The units of `FluxOfMagneticInduction`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<FLUX_OF_MAGNETIC_INDUCTION, N>, N> {
        fn quantity(&self, value: N) -> Qty<FLUX_OF_MAGNETIC_INDUCTION, N>;
        fn value(&self, quantity: Qty<FLUX_OF_MAGNETIC_INDUCTION, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum FluxOfMagneticInductionUnit {
    #[default]
    #[value(MAXWELL)]
    Maxwell,
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
    #[value(DYNE)]
    Dyne,
}

/// The units of `Illuminance`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<ILLUMINANCE, N>, N> {
        fn quantity(&self, value: N) -> Qty<ILLUMINANCE, N>;
        fn value(&self, quantity: Qty<ILLUMINANCE, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum IlluminanceUnit {
    #[default]
    #[value(PHOT)]
    Phot,
}

/// The units of `IonDose`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<ION_DOSE, N>, N> {
        fn quantity(&self, value: N) -> Qty<ION_DOSE, N>;
        fn value(&self, quantity: Qty<ION_DOSE, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum IonDoseUnit {
    #[default]
    #[value(ROENTGEN)]
    Roentgen,
}

/// The units of `KinematicViscosity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<KINEMATIC_VISCOSITY, N>, N> {
        fn quantity(&self, value: N) -> Qty<KINEMATIC_VISCOSITY, N>;
        fn value(&self, quantity: Qty<KINEMATIC_VISCOSITY, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum KinematicViscosityUnit {
    #[default]
    #[value(STOKES)]
    Stokes,
}

/// The units of `LineicNumber`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<LINEIC_NUMBER, N>, N> {
        fn quantity(&self, value: N) -> Qty<LINEIC_NUMBER, N>;
        fn value(&self, quantity: Qty<LINEIC_NUMBER, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum LineicNumberUnit {
    #[default]
    #[value(KAYSER)]
    Kayser,
}

/// The units of `LumIntensityDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<LUM_INTENSITY_DENSITY, N>, N> {
        fn quantity(&self, value: N) -> Qty<LUM_INTENSITY_DENSITY, N>;
        fn value(&self, quantity: Qty<LUM_INTENSITY_DENSITY, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum LumIntensityDensityUnit {
    #[default]
    #[value(STILB)]
    Stilb,
}

/// The units of `MagneticFieldIntensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<MAGNETIC_FIELD_INTENSITY, N>, N> {
        fn quantity(&self, value: N) -> Qty<MAGNETIC_FIELD_INTENSITY, N>;
        fn value(&self, quantity: Qty<MAGNETIC_FIELD_INTENSITY, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MagneticFieldIntensityUnit {
    #[default]
    #[value(OERSTED)]
    Oersted,
}

/// The units of `MagneticFluxDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<MAGNETIC_FLUX_DENSITY, N>, N> {
        fn quantity(&self, value: N) -> Qty<MAGNETIC_FLUX_DENSITY, N>;
        fn value(&self, quantity: Qty<MAGNETIC_FLUX_DENSITY, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MagneticFluxDensityUnit {
    #[default]
    #[value(GAUSS)]
    Gauss,
}

/// The units of `MagneticTension`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<MAGNETIC_TENSION, N>, N> {
        fn quantity(&self, value: N) -> Qty<MAGNETIC_TENSION, N>;
        fn value(&self, quantity: Qty<MAGNETIC_TENSION, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MagneticTensionUnit {
    #[default]
    #[value(GILBERT)]
    Gilbert,
}

/// The units of `Radioactivity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<RADIOACTIVITY, N>, N> {
        fn quantity(&self, value: N) -> Qty<RADIOACTIVITY, N>;
        fn value(&self, quantity: Qty<RADIOACTIVITY, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum RadioactivityUnit {
    #[default]
    #[value(CURIE)]
    Curie,
}
