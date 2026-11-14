//! The units of `misc`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::dynamic::DynamicUcumUnit;
use crate::misc::units::*;
use crate::misc::*;
use crate::{Exact, Qty, Unit};
use core::ops::{Add, Div, Mul, Sub};

/// The units of `ActionArea`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<ACTION_AREA, N>, N> {
        fn quantity(&self, value: N) -> Qty<ACTION_AREA, N>;
        fn value(&self, quantity: Qty<ACTION_AREA, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ActionAreaUnit {
    #[default]
    #[value(BARN)]
    Barn,
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
    #[value(MHO)]
    Mho,
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
    #[value(LINE)]
    Line,
    #[value(POINT)]
    Point,
    #[value(PICA)]
    Pica,
    #[value(PRINTER_S_POINT)]
    PrinterSPoint,
    #[value(PRINTER_S_PICA)]
    PrinterSPica,
    #[value(FRENCH_FOOT)]
    FrenchFoot,
    #[value(FRENCH_INCH)]
    FrenchInch,
    #[value(FRENCH_LINE)]
    FrenchLine,
    #[value(DIDOT_S_POINT)]
    DidotSPoint,
    #[value(DIDOT_S_PICA)]
    DidotSPica,
    #[value(ANGSTROM)]
    Angstrom,
    #[value(SMOOT)]
    Smoot,
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
    #[value(METRIC_CARAT)]
    MetricCarat,
}

/// The units of `MassFraction`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<MASS_FRACTION, N>, N> {
        fn quantity(&self, value: N) -> Qty<MASS_FRACTION, N>;
        fn value(&self, quantity: Qty<MASS_FRACTION, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MassFractionUnit {
    #[default]
    #[value(CARAT_OF_GOLD_ALLOYS)]
    CaratOfGoldAlloys,
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
    #[value(CIRCLE)]
    Circle,
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
    #[value(TECHNICAL_ATMOSPHERE)]
    TechnicalAtmosphere,
    #[value(POUND_PER_SQUARE_INCH)]
    PoundPerSquareInch,
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
    #[value(SPHERE)]
    Sphere,
}

/// The units of `Turbidity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<TURBIDITY, N>, N> {
        fn quantity(&self, value: N) -> Qty<TURBIDITY, N>;
        fn value(&self, quantity: Qty<TURBIDITY, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum TurbidityUnit {
    #[default]
    #[value(NEPHELOMETRIC_TURBIDITY_UNIT)]
    NephelometricTurbidityUnit,
    #[value(FORMAZIN_NEPHELOMETRIC_UNIT)]
    FormazinNephelometricUnit,
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
    #[value(STERE)]
    Stere,
}
