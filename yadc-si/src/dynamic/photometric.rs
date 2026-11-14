//! The units of `photometric`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::dynamic::DynamicSIUnit;
use crate::photometric::units::*;
use crate::photometric::*;
use core::marker::Destruct;
use core::ops::{Add, Div, Mul, Sub};

/// The units of `Luminance`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Luminance<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum LuminanceUnit {
    #[default]
    #[unit(CandelaPerSquareMetre)]
    CandelaPerSquareMetre,
    #[unit(CandelaPerSquareInch)]
    CandelaPerSquareInch,
    #[unit(CandelaPerSquareFoot)]
    CandelaPerSquareFoot,
    #[unit(Footlambert)]
    Footlambert,
    #[unit(Lambert)]
    Lambert,
    #[unit(Stilb)]
    Stilb,
}

impl From<LuminanceUnit> for DynamicSIUnit {
    fn from(unit: LuminanceUnit) -> Self {
        match unit {
            LuminanceUnit::CandelaPerSquareMetre => CandelaPerSquareMetre {}.into(),
            LuminanceUnit::CandelaPerSquareInch => CandelaPerSquareInch {}.into(),
            LuminanceUnit::CandelaPerSquareFoot => CandelaPerSquareFoot {}.into(),
            LuminanceUnit::Footlambert => Footlambert {}.into(),
            LuminanceUnit::Lambert => Lambert {}.into(),
            LuminanceUnit::Stilb => Stilb {}.into(),
        }
    }
}

/// The units of `LuminousIntensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LuminousIntensity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum LuminousIntensityUnit {
    #[default]
    #[unit(Candela)]
    Candela,
}

impl From<LuminousIntensityUnit> for DynamicSIUnit {
    fn from(unit: LuminousIntensityUnit) -> Self {
        match unit {
            LuminousIntensityUnit::Candela => Candela {}.into(),
        }
    }
}

/// The units of `Illuminance`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Illuminance<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum IlluminanceUnit {
    #[default]
    #[unit(Lux)]
    Lux,
    #[unit(Phot)]
    Phot,
    #[unit(Footcandle)]
    Footcandle,
}

impl From<IlluminanceUnit> for DynamicSIUnit {
    fn from(unit: IlluminanceUnit) -> Self {
        match unit {
            IlluminanceUnit::Lux => Lux {}.into(),
            IlluminanceUnit::Phot => Phot {}.into(),
            IlluminanceUnit::Footcandle => Footcandle {}.into(),
        }
    }
}

/// The units of `RadiantExposure`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = RadiantExposure<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum RadiantExposureUnit {
    #[default]
    #[unit(JoulePerSquareMetre)]
    JoulePerSquareMetre,
    #[unit(WattHourPerSquareMetre)]
    WattHourPerSquareMetre,
    #[unit(MicrojoulePerSquareMillimetre)]
    MicrojoulePerSquareMillimetre,
}

impl From<RadiantExposureUnit> for DynamicSIUnit {
    fn from(unit: RadiantExposureUnit) -> Self {
        match unit {
            RadiantExposureUnit::JoulePerSquareMetre => JoulePerSquareMetre {}.into(),
            RadiantExposureUnit::WattHourPerSquareMetre => WattHourPerSquareMetre {}.into(),
            RadiantExposureUnit::MicrojoulePerSquareMillimetre => {
                MicrojoulePerSquareMillimetre {}.into()
            }
        }
    }
}
