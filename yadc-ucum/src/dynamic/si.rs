//! The units of `si`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::dynamic::DynamicUcumUnit;
use crate::si::units::*;
use crate::si::*;
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
    #[value(MOLE)]
    Mole,
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
    #[value(SIEVERT)]
    Sievert,
}

/// The units of `ElectricCapacitance`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<ELECTRIC_CAPACITANCE, N>, N> {
        fn quantity(&self, value: N) -> Qty<ELECTRIC_CAPACITANCE, N>;
        fn value(&self, quantity: Qty<ELECTRIC_CAPACITANCE, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ElectricCapacitanceUnit {
    #[default]
    #[value(FARAD)]
    Farad,
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
    #[value(COULOMB)]
    Coulomb,
}

/// The units of `ElectricConductance`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<ELECTRIC_CONDUCTANCE, N>, N> {
        fn quantity(&self, value: N) -> Qty<ELECTRIC_CONDUCTANCE, N>;
        fn value(&self, quantity: Qty<ELECTRIC_CONDUCTANCE, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ElectricConductanceUnit {
    #[default]
    #[value(SIEMENS)]
    Siemens,
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
    #[value(AMPERE)]
    Ampere,
}

/// The units of `ElectricPotential`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<ELECTRIC_POTENTIAL, N>, N> {
        fn quantity(&self, value: N) -> Qty<ELECTRIC_POTENTIAL, N>;
        fn value(&self, quantity: Qty<ELECTRIC_POTENTIAL, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ElectricPotentialUnit {
    #[default]
    #[value(VOLT)]
    Volt,
}

/// The units of `ElectricResistance`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<ELECTRIC_RESISTANCE, N>, N> {
        fn quantity(&self, value: N) -> Qty<ELECTRIC_RESISTANCE, N>;
        fn value(&self, quantity: Qty<ELECTRIC_RESISTANCE, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ElectricResistanceUnit {
    #[default]
    #[value(OHM)]
    Ohm,
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
    #[value(JOULE)]
    Joule,
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
    #[value(GRAY)]
    Gray,
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
    #[value(NEWTON)]
    Newton,
}

/// The units of `Frequency`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<FREQUENCY, N>, N> {
        fn quantity(&self, value: N) -> Qty<FREQUENCY, N>;
        fn value(&self, quantity: Qty<FREQUENCY, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum FrequencyUnit {
    #[default]
    #[value(HERTZ)]
    Hertz,
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
    #[value(LUX)]
    Lux,
}

/// The units of `Inductance`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<INDUCTANCE, N>, N> {
        fn quantity(&self, value: N) -> Qty<INDUCTANCE, N>;
        fn value(&self, quantity: Qty<INDUCTANCE, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum InductanceUnit {
    #[default]
    #[value(HENRY)]
    Henry,
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
    #[value(METER)]
    Meter,
}

/// The units of `LuminousFlux`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<LUMINOUS_FLUX, N>, N> {
        fn quantity(&self, value: N) -> Qty<LUMINOUS_FLUX, N>;
        fn value(&self, quantity: Qty<LUMINOUS_FLUX, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum LuminousFluxUnit {
    #[default]
    #[value(LUMEN)]
    Lumen,
}

/// The units of `LuminousIntensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<LUMINOUS_INTENSITY, N>, N> {
        fn quantity(&self, value: N) -> Qty<LUMINOUS_INTENSITY, N>;
        fn value(&self, quantity: Qty<LUMINOUS_INTENSITY, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum LuminousIntensityUnit {
    #[default]
    #[value(CANDELA)]
    Candela,
}

/// The units of `MagneticFlux`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<MAGNETIC_FLUX, N>, N> {
        fn quantity(&self, value: N) -> Qty<MAGNETIC_FLUX, N>;
        fn value(&self, quantity: Qty<MAGNETIC_FLUX, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MagneticFluxUnit {
    #[default]
    #[value(WEBER)]
    Weber,
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
    #[value(TESLA)]
    Tesla,
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
    #[value(GRAM)]
    Gram,
}

/// The units of `PlaneAngle`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<PLANE_ANGLE, N>, N> {
        fn quantity(&self, value: N) -> Qty<PLANE_ANGLE, N>;
        fn value(&self, quantity: Qty<PLANE_ANGLE, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum PlaneAngleUnit {
    #[default]
    #[value(RADIAN)]
    Radian,
}

/// The units of `Power`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<POWER, N>, N> {
        fn quantity(&self, value: N) -> Qty<POWER, N>;
        fn value(&self, quantity: Qty<POWER, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum PowerUnit {
    #[default]
    #[value(WATT)]
    Watt,
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
    #[value(PASCAL)]
    Pascal,
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
    #[value(BECQUEREL)]
    Becquerel,
}

/// The units of `SolidAngle`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<SOLID_ANGLE, N>, N> {
        fn quantity(&self, value: N) -> Qty<SOLID_ANGLE, N>;
        fn value(&self, quantity: Qty<SOLID_ANGLE, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum SolidAngleUnit {
    #[default]
    #[value(STERADIAN)]
    Steradian,
}

/// The units of `Time`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<TIME, N>, N> {
        fn quantity(&self, value: N) -> Qty<TIME, N>;
        fn value(&self, quantity: Qty<TIME, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum TimeUnit {
    #[default]
    #[value(SECOND)]
    Second,
}
