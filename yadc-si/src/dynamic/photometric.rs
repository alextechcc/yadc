//! The units of `photometric`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::base::{KILOGRAM, METRE, SECOND};
use crate::photometric::consts::*;
use crate::photometric::*;
use core::marker::Destruct;
use yadc::{Unit, num_traits::Num};

/// The units of `Luminance`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Luminance<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Luminance<N>: [const] Unit<Luminance<N>, N> + [const] Destruct)]
pub enum LuminanceUnit {
    #[default]
    #[unit("cd/m²", "candelas per square metre", by = Luminance::<N>::from(CANDELA / (METRE * METRE)))]
    CandelaPerSquareMetre,
    #[unit("cd/in²", "candelas per square inch", by = Luminance::<N>::from((CANDELA / (METRE * METRE)) * (Exact::int(1) / (Exact::dec(254, -4) * Exact::dec(254, -4)))))]
    CandelaPerSquareInch,
    #[unit("cd/ft²", "candelas per square foot", by = Luminance::<N>::from((CANDELA / (METRE * METRE)) * (Exact::int(1) / (Exact::dec(3048, -4) * Exact::dec(3048, -4)))))]
    CandelaPerSquareFoot,
    #[unit("fl", "footlamberts", by = Luminance::<N>::from(FOOTLAMBERT))]
    Footlambert,
    #[unit("la", "lamberts", by = Luminance::<N>::from(LAMBERT))]
    Lambert,
    #[unit("sb", "stilbs", by = Luminance::<N>::from(STILB))]
    Stilb,
}

/// The units of `LuminousIntensity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LuminousIntensity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where LuminousIntensity<N>: [const] Unit<LuminousIntensity<N>, N> + [const] Destruct)]
pub enum LuminousIntensityUnit {
    #[default]
    #[unit("cd", "candelas", by = LuminousIntensity::<N>::from(CANDELA))]
    Candela,
}

/// The units of `Illuminance`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Illuminance<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Illuminance<N>: [const] Unit<Illuminance<N>, N> + [const] Destruct)]
pub enum IlluminanceUnit {
    #[default]
    #[unit("lx", "lux", by = Illuminance::<N>::from(LUX))]
    Lux,
    #[unit("ph", "phot", by = Illuminance::<N>::from(PHOT))]
    Phot,
    #[unit("fc", "footcandles", by = Illuminance::<N>::from(FOOTCANDLE))]
    Footcandle,
}

/// The units of `RadiantExposure`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = RadiantExposure<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where RadiantExposure<N>: [const] Unit<RadiantExposure<N>, N> + [const] Destruct)]
pub enum RadiantExposureUnit {
    #[default]
    #[unit("J/m²", "joules per square metre", by = RadiantExposure::<N>::from(KILOGRAM / (SECOND * SECOND)))]
    JoulePerSquareMetre,
    #[unit("W · h/m²", "watt hours per square metre", by = RadiantExposure::<N>::from((KILOGRAM / (SECOND * SECOND)) * Exact::int(3600)))]
    WattHourPerSquareMetre,
    #[unit("µJ/mm²", "microjoules per square millimetre", by = RadiantExposure::<N>::from(KILOGRAM / (SECOND * SECOND)))]
    MicrojoulePerSquareMillimetre,
}
