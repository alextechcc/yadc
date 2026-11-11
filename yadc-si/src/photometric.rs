use crate::{Exact, ISQ, SIQuantity, counted};
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
/// (re-exported), then one per multiple, each defined exactly from them -
/// `INCH` is `0.0254 * METRE` - and counted in the number type its use asks
/// for, rounded once: `let d: Length<f64> = INCH;` is an `f64` length,
/// `Length<f32>` an `f32` one, `INCH::<Exact>` the exact value. Each is a
/// unit (`INCH(3.0)`, `INCH.value(d)`). A prefixed unit is a prefix times
/// one: `KILO * METRE`. They compose in `const` items:
/// `const LIMIT: Velocity<f64> = 50.0 * (KILO * METRE) / HOUR;`.
pub mod consts {
    use super::*;
    use crate::base::{KILOGRAM, METRE, RADIAN, SECOND};

    /// The candela, the SI base unit of luminous intensity.
    pub use crate::base::CANDELA;

    /// One `CandelaPerSquareMetre` (cd/m²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_METRE<N: const From<Exact>>: Luminance<N> = counted(CANDELA::<Exact> / (METRE::<Exact> * METRE::<Exact>));

    /// One `CandelaPerSquareInch` (cd/in²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_INCH<N: const From<Exact>>: Luminance<N> = counted((Exact::int(1) / (Exact::dec(254, -4) * Exact::dec(254, -4))) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `CandelaPerSquareFoot` (cd/ft²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_FOOT<N: const From<Exact>>: Luminance<N> = counted((Exact::int(1) / (Exact::dec(3048, -4) * Exact::dec(3048, -4))) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `Footlambert` (fl), as a `Luminance`.
    pub const FOOTLAMBERT<N: const From<Exact>>: Luminance<N> = counted((Exact::int(1) / (Exact::PI * Exact::dec(3048, -4) * Exact::dec(3048, -4))) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `Lambert` (la), as a `Luminance`.
    pub const LAMBERT<N: const From<Exact>>: Luminance<N> = counted((Exact::int(10000) / Exact::PI) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `Stilb` (sb), as a `Luminance`.
    pub const STILB<N: const From<Exact>>: Luminance<N> = counted(Exact::int(10000) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `Lux` (lx), as an `Illuminance`.
    pub const LUX<N: const From<Exact>>: Illuminance<N> = counted(CANDELA::<Exact> * RADIAN::<Exact> * RADIAN::<Exact> / (METRE::<Exact> * METRE::<Exact>));

    /// One `Phot` (ph), as an `Illuminance`.
    pub const PHOT<N: const From<Exact>>: Illuminance<N> = counted(Exact::int(10000) * LUX::<Exact>);

    /// One `Footcandle` (fc), as an `Illuminance`.
    pub const FOOTCANDLE<N: const From<Exact>>: Illuminance<N> = counted((Exact::int(1) / (Exact::dec(3048, -4) * Exact::dec(3048, -4))) * LUX::<Exact>);

    /// One `JoulePerSquareMetre` (J/m²), as a `RadiantExposure`.
    pub const JOULE_PER_SQUARE_METRE<N: const From<Exact>>: RadiantExposure<N> = counted(KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact>));

    /// One `WattHourPerSquareMetre` (W · h/m²), as a `RadiantExposure`.
    pub const WATT_HOUR_PER_SQUARE_METRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(3600) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `MicrojoulePerSquareMillimetre` (µJ/mm²), as a `RadiantExposure`.
    pub const MICROJOULE_PER_SQUARE_MILLIMETRE<N: const From<Exact>>: RadiantExposure<N> = counted(JOULE_PER_SQUARE_METRE::<Exact>);
}

/// The units of `Luminance`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Luminance<f64>)]
pub enum LuminanceUnit {
    #[default]
    #[unit("cd/m²", "candelas per square metre", by = CANDELA_PER_SQUARE_METRE)]
    CandelaPerSquareMetre,
    #[unit("cd/in²", "candelas per square inch", by = CANDELA_PER_SQUARE_INCH)]
    CandelaPerSquareInch,
    #[unit("cd/ft²", "candelas per square foot", by = CANDELA_PER_SQUARE_FOOT)]
    CandelaPerSquareFoot,
    #[unit("fl", "footlamberts", by = FOOTLAMBERT)]
    Footlambert,
    #[unit("la", "lamberts", by = LAMBERT)]
    Lambert,
    #[unit("sb", "stilbs", by = STILB)]
    Stilb,
}

/// The units of `LuminousIntensity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LuminousIntensity<f64>)]
pub enum LuminousIntensityUnit {
    #[default]
    #[unit("cd", "candelas", by = CANDELA)]
    Candela,
}

/// The units of `Illuminance`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Illuminance<f64>)]
pub enum IlluminanceUnit {
    #[default]
    #[unit("lx", "lux", by = LUX)]
    Lux,
    #[unit("ph", "phot", by = PHOT)]
    Phot,
    #[unit("fc", "footcandles", by = FOOTCANDLE)]
    Footcandle,
}

/// The units of `RadiantExposure`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = RadiantExposure<f64>)]
pub enum RadiantExposureUnit {
    #[default]
    #[unit("J/m²", "joules per square metre", by = JOULE_PER_SQUARE_METRE)]
    JoulePerSquareMetre,
    #[unit("W · h/m²", "watt hours per square metre", by = WATT_HOUR_PER_SQUARE_METRE)]
    WattHourPerSquareMetre,
    #[unit("µJ/mm²", "microjoules per square millimetre", by = MICROJOULE_PER_SQUARE_MILLIMETRE)]
    MicrojoulePerSquareMillimetre,
}
