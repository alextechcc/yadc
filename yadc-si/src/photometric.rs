use crate::base::{KILOGRAM, METRE, SECOND};
use crate::{Exact, ISQ, SIQuantity};
use consts::*;
use yadc::Dimension;

pub type Luminance<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            cd: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type LuminousIntensity<N> = SIQuantity<
    {
        ISQ {
            cd: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Illuminance<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            cd: 1,
            rad: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type RadiantExposure<N> = SIQuantity<
    {
        ISQ {
            kg: 1,
            s: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

/// The constants of this module: the base constants of its quantities
/// (re-exported), then one per named unit, each defined exactly from them
/// (`INCH` is `0.0254 * METRE`). A unit written as others is not one:
/// `SLUG / CUBIC_FOOT`, `KILO * METRE` are exact too. Converting rounds,
/// once, where it is written - in a `const`, by the compiler:
/// `const MPH: Velocity<f64> = (MILE / HOUR).into();`, then `MPH(60.0)`.
pub mod consts {
    use super::*;
    use crate::base::{METRE, RADIAN};

    /// The candela, the SI base unit of luminous intensity.
    pub use crate::base::CANDELA;

    /// One `Footlambert` (`fl`), as a `Luminance`.
    pub const FOOTLAMBERT: Luminance<Exact> = (CANDELA / (METRE * METRE))
        * (Exact::int(1) / (Exact::PI * Exact::dec(3048, -4) * Exact::dec(3048, -4)));

    /// One `Lambert` (`la`), as a `Luminance`.
    pub const LAMBERT: Luminance<Exact> =
        (CANDELA / (METRE * METRE)) * (Exact::int(10000) / Exact::PI);

    /// One `Stilb` (`sb`), as a `Luminance`.
    pub const STILB: Luminance<Exact> = (CANDELA / (METRE * METRE)) * Exact::int(10000);

    /// One `Lux` (`lx`), as an `Illuminance`.
    pub const LUX: Illuminance<Exact> = CANDELA * RADIAN * RADIAN / (METRE * METRE);

    /// One `Phot` (`ph`), as an `Illuminance`.
    pub const PHOT: Illuminance<Exact> = LUX * Exact::int(10000);

    /// One `Footcandle` (`fc`), as an `Illuminance`.
    pub const FOOTCANDLE: Illuminance<Exact> =
        LUX * (Exact::int(1) / (Exact::dec(3048, -4) * Exact::dec(3048, -4)));
}

/// The units of `Luminance`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Luminance<N>, number = N)]
#[unit(for<N> where Luminance<N>: [const] yadc::Unit<Luminance<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
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

/// The units of `LuminousIntensity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LuminousIntensity<N>, number = N)]
#[unit(for<N> where LuminousIntensity<N>: [const] yadc::Unit<LuminousIntensity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum LuminousIntensityUnit {
    #[default]
    #[unit("cd", "candelas", by = LuminousIntensity::<N>::from(CANDELA))]
    Candela,
}

/// The units of `Illuminance`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Illuminance<N>, number = N)]
#[unit(for<N> where Illuminance<N>: [const] yadc::Unit<Illuminance<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum IlluminanceUnit {
    #[default]
    #[unit("lx", "lux", by = Illuminance::<N>::from(LUX))]
    Lux,
    #[unit("ph", "phot", by = Illuminance::<N>::from(PHOT))]
    Phot,
    #[unit("fc", "footcandles", by = Illuminance::<N>::from(FOOTCANDLE))]
    Footcandle,
}

/// The units of `RadiantExposure`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = RadiantExposure<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, RadiantExposure<N>: [const] yadc::Unit<RadiantExposure<N>, N> + [const] core::marker::Destruct)]
pub enum RadiantExposureUnit {
    #[default]
    #[unit("J/m²", "joules per square metre", by = RadiantExposure::<N>::from(KILOGRAM / (SECOND * SECOND)))]
    JoulePerSquareMetre,
    #[unit("W · h/m²", "watt hours per square metre", by = RadiantExposure::<N>::from((KILOGRAM / (SECOND * SECOND)) * Exact::int(3600)))]
    WattHourPerSquareMetre,
    #[unit("µJ/mm²", "microjoules per square millimetre", by = RadiantExposure::<N>::from(KILOGRAM / (SECOND * SECOND)))]
    MicrojoulePerSquareMillimetre,
}
