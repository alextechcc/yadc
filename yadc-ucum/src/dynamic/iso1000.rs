//! The units of `iso1000`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::dynamic::DynamicUcumUnit;
use crate::iso1000::units::*;
use crate::iso1000::*;
use crate::{Exact, Qty, Unit};
use core::ops::{Add, Div, Mul, Sub};

/// The units of `Area`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<Qty<AREA, N>, N> {
        fn quantity(&self, value: N) -> Qty<AREA, N>;
        fn value(&self, quantity: Qty<AREA, N>) -> N;
    }),
    into(DynamicUcumUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum AreaUnit {
    #[default]
    #[value(ARE)]
    Are,
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
    #[value(ELECTRONVOLT)]
    Electronvolt,
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
    #[value(ASTRONOMIC_UNIT)]
    AstronomicUnit,
    #[value(PARSEC)]
    Parsec,
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
    #[value(TONNE)]
    Tonne,
    #[value(UNIFIED_ATOMIC_MASS_UNIT)]
    UnifiedAtomicMassUnit,
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
    #[value(GRADE)]
    Grade,
    #[value(DEGREE)]
    Degree,
    #[value(PLANE_ANGLE_MINUTE)]
    Minute,
    #[value(SECOND)]
    Second,
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
    #[value(BAR)]
    Bar,
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
    #[value(TIME_MINUTE)]
    Minute,
    #[value(HOUR)]
    Hour,
    #[value(DAY)]
    Day,
    #[value(TROPICAL_YEAR)]
    TropicalYear,
    #[value(MEAN_JULIAN_YEAR)]
    MeanJulianYear,
    #[value(MEAN_GREGORIAN_YEAR)]
    MeanGregorianYear,
    #[value(YEAR)]
    Year,
    #[value(WEEK)]
    Week,
    #[value(SYNODAL_MONTH)]
    SynodalMonth,
    #[value(MEAN_JULIAN_MONTH)]
    MeanJulianMonth,
    #[value(MEAN_GREGORIAN_MONTH)]
    MeanGregorianMonth,
    #[value(MONTH)]
    Month,
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
    #[value(LITER)]
    Liter,
}
