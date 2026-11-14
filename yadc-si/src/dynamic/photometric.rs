//! The units of `photometric`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::dynamic::DynamicSIUnit;
use crate::photometric::units::*;
use crate::photometric::*;
use crate::{Exact, SIQuantity, Unit};
use core::ops::{Add, Div, Mul, Sub};

/// The units of `Luminance`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<LUMINANCE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<LUMINANCE, N>;
        fn value(&self, quantity: SIQuantity<LUMINANCE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum LuminanceUnit {
    #[default]
    #[value(CANDELA_PER_SQUARE_METRE)]
    CandelaPerSquareMetre,
    #[value(CANDELA_PER_SQUARE_INCH)]
    CandelaPerSquareInch,
    #[value(CANDELA_PER_SQUARE_FOOT)]
    CandelaPerSquareFoot,
    #[value(FOOTLAMBERT)]
    Footlambert,
    #[value(LAMBERT)]
    Lambert,
    #[value(STILB)]
    Stilb,
}

/// The units of `LuminousIntensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<LUMINOUS_INTENSITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<LUMINOUS_INTENSITY, N>;
        fn value(&self, quantity: SIQuantity<LUMINOUS_INTENSITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum LuminousIntensityUnit {
    #[default]
    #[value(CANDELA)]
    Candela,
}

/// The units of `Illuminance`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ILLUMINANCE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ILLUMINANCE, N>;
        fn value(&self, quantity: SIQuantity<ILLUMINANCE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum IlluminanceUnit {
    #[default]
    #[value(LUX)]
    Lux,
    #[value(PHOT)]
    Phot,
    #[value(FOOTCANDLE)]
    Footcandle,
}

/// The units of `RadiantExposure`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<RADIANT_EXPOSURE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<RADIANT_EXPOSURE, N>;
        fn value(&self, quantity: SIQuantity<RADIANT_EXPOSURE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum RadiantExposureUnit {
    #[default]
    #[value(JOULE_PER_SQUARE_METRE)]
    JoulePerSquareMetre,
    #[value(WATT_HOUR_PER_SQUARE_METRE)]
    WattHourPerSquareMetre,
    #[value(MICROJOULE_PER_SQUARE_MILLIMETRE)]
    MicrojoulePerSquareMillimetre,
}
