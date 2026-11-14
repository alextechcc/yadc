//! The units of `clinical`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::clinical::units::*;
use crate::clinical::*;
use crate::dynamic::DynamicUcumUnit;
use crate::{Exact, Qty, Unit};
use core::ops::{Add, Div, Mul, Sub};

/// The units of `AmountOfSubstance`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<AMOUNT_OF_SUBSTANCE, N>, N> {
        fn quantity(&self, value: N) -> Qty<AMOUNT_OF_SUBSTANCE, N>;
        fn value(&self, quantity: Qty<AMOUNT_OF_SUBSTANCE, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum AmountOfSubstanceUnit {
    #[default]
    #[value(EQUIVALENTS)]
    Equivalents,
}

/// The units of `AmountOfSubstanceDissolvedParticles`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<AMOUNT_OF_SUBSTANCE_DISSOLVED_PARTICLES, N>, N> {
        fn quantity(&self, value: N) -> Qty<AMOUNT_OF_SUBSTANCE_DISSOLVED_PARTICLES, N>;
        fn value(&self, quantity: Qty<AMOUNT_OF_SUBSTANCE_DISSOLVED_PARTICLES, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum AmountOfSubstanceDissolvedParticlesUnit {
    #[default]
    #[value(OSMOLE)]
    Osmole,
}

/// The units of `CatalyticActivity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<CATALYTIC_ACTIVITY, N>, N> {
        fn quantity(&self, value: N) -> Qty<CATALYTIC_ACTIVITY, N>;
        fn value(&self, quantity: Qty<CATALYTIC_ACTIVITY, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum CatalyticActivityUnit {
    #[default]
    #[value(KATAL)]
    Katal,
    #[value(UNIT)]
    Unit,
}

/// The units of `FluidResistance`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<FLUID_RESISTANCE, N>, N> {
        fn quantity(&self, value: N) -> Qty<FLUID_RESISTANCE, N>;
        fn value(&self, quantity: Qty<FLUID_RESISTANCE, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum FluidResistanceUnit {
    #[default]
    #[value(PERIPHERAL_VASCULAR_RESISTANCE_UNIT)]
    PeripheralVascularResistanceUnit,
    #[value(WOOD_UNIT)]
    WoodUnit,
}

/// The units of `GaugeOfCatheters`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<GAUGE_OF_CATHETERS, N>, N> {
        fn quantity(&self, value: N) -> Qty<GAUGE_OF_CATHETERS, N>;
        fn value(&self, quantity: Qty<GAUGE_OF_CATHETERS, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum GaugeOfCathetersUnit {
    #[default]
    #[value(FRENCH)]
    French,
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
    #[value(MESH)]
    Mesh,
}

/// The units of `MassConcentration`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<MASS_CONCENTRATION, N>, N> {
        fn quantity(&self, value: N) -> Qty<MASS_CONCENTRATION, N>;
        fn value(&self, quantity: Qty<MASS_CONCENTRATION, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MassConcentrationUnit {
    #[default]
    #[value(GRAM_PERCENT)]
    GramPercent,
}

/// The units of `MetabolicCostOfPhysicalActivity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<METABOLIC_COST_OF_PHYSICAL_ACTIVITY, N>, N> {
        fn quantity(&self, value: N) -> Qty<METABOLIC_COST_OF_PHYSICAL_ACTIVITY, N>;
        fn value(&self, quantity: Qty<METABOLIC_COST_OF_PHYSICAL_ACTIVITY, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MetabolicCostOfPhysicalActivityUnit {
    #[default]
    #[value(METABOLIC_EQUIVALENT)]
    MetabolicEquivalent,
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
    #[value(METER_OF_WATER_COLUMN)]
    MeterOfWaterColumn,
    #[value(METER_OF_MERCURY_COLUMN)]
    MeterOfMercuryColumn,
    #[value(INCH_OF_WATER_COLUMN)]
    InchOfWaterColumn,
    #[value(INCH_OF_MERCURY_COLUMN)]
    InchOfMercuryColumn,
}

/// The units of `RefractionOfALens`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<REFRACTION_OF_A_LENS, N>, N> {
        fn quantity(&self, value: N) -> Qty<REFRACTION_OF_A_LENS, N>;
        fn value(&self, quantity: Qty<REFRACTION_OF_A_LENS, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum RefractionOfALensUnit {
    #[default]
    #[value(DIOPTER)]
    Diopter,
}

/// The units of `SedimentationCoefficient`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<SEDIMENTATION_COEFFICIENT, N>, N> {
        fn quantity(&self, value: N) -> Qty<SEDIMENTATION_COEFFICIENT, N>;
        fn value(&self, quantity: Qty<SEDIMENTATION_COEFFICIENT, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum SedimentationCoefficientUnit {
    #[default]
    #[value(SVEDBERG_UNIT)]
    SvedbergUnit,
}

/// The units of `ViewAreaInMicroscope`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<VIEW_AREA_IN_MICROSCOPE, N>, N> {
        fn quantity(&self, value: N) -> Qty<VIEW_AREA_IN_MICROSCOPE, N>;
        fn value(&self, quantity: Qty<VIEW_AREA_IN_MICROSCOPE, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ViewAreaInMicroscopeUnit {
    #[default]
    #[value(HIGH_POWER_FIELD)]
    HighPowerField,
    #[value(LOW_POWER_FIELD)]
    LowPowerField,
}

/// The units of `Volume`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<VOLUME, N>, N> {
        fn quantity(&self, value: N) -> Qty<VOLUME, N>;
        fn value(&self, quantity: Qty<VOLUME, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum VolumeUnit {
    #[default]
    #[value(DROP)]
    Drop,
}

/// The units of `XRayAttenuation`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<X_RAY_ATTENUATION, N>, N> {
        fn quantity(&self, value: N) -> Qty<X_RAY_ATTENUATION, N>;
        fn value(&self, quantity: Qty<X_RAY_ATTENUATION, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum XRayAttenuationUnit {
    #[default]
    #[value(HOUNSFIELD_UNIT)]
    HounsfieldUnit,
}
