use crate::base::{KILOGRAM, METRE, ONE, SECOND};
use crate::{Exact, ISQ, SIQuantity};
use consts::*;
use yadc::Dimension;

pub type MassDensity<N> = SIQuantity<
    {
        ISQ {
            m: -3,
            kg: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type LinearMassDensity<N> = SIQuantity<
    {
        ISQ {
            m: -1,
            kg: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ArealMassDensity<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            kg: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type LinearNumberDensity<N> = SIQuantity<
    {
        ISQ {
            m: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ArealNumberDensity<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type VolumetricNumberDensity<N> = SIQuantity<
    {
        ISQ {
            m: -3,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type LinearNumberRate<N> = SIQuantity<
    {
        ISQ {
            m: -1,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ArealNumberRate<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type VolumetricNumberRate<N> = SIQuantity<
    {
        ISQ {
            m: -3,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type LinearDensityOfStates<N> = SIQuantity<
    {
        ISQ {
            m: -3,
            kg: -1,
            s: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ArealDensityOfStates<N> = SIQuantity<
    {
        ISQ {
            m: -4,
            kg: -1,
            s: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type VolumetricDensityOfStates<N> = SIQuantity<
    {
        ISQ {
            m: -5,
            kg: -1,
            s: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type LinearPowerDensity<N> = SIQuantity<
    {
        ISQ {
            m: 1,
            kg: 1,
            s: -3,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type VolumetricPowerDensity<N> = SIQuantity<
    {
        ISQ {
            m: -1,
            kg: 1,
            s: -3,
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
    use crate::base::{METRE, ONE};

    /// One `PerMetre` (`m⁻¹`), as a `LinearNumberDensity`.
    pub const PER_METRE: LinearNumberDensity<Exact> = ONE / METRE;

    /// One `PerFoot` (`ft⁻¹`), as a `LinearNumberDensity`.
    pub const PER_FOOT: LinearNumberDensity<Exact> =
        PER_METRE * (Exact::int(1) / Exact::dec(3048, -4));

    /// One `PerFootSurvey` (`ft (U.S. survey)`), as a `LinearNumberDensity`.
    pub const PER_FOOT_SURVEY: LinearNumberDensity<Exact> =
        PER_METRE * (Exact::int(1) / (Exact::int(1200) / Exact::int(3937)));

    /// One `PerInch` (`in⁻¹`), as a `LinearNumberDensity`.
    pub const PER_INCH: LinearNumberDensity<Exact> =
        PER_METRE * (Exact::int(1) / Exact::dec(254, -4));

    /// One `PerMile` (`mi⁻¹`), as a `LinearNumberDensity`.
    pub const PER_MILE: LinearNumberDensity<Exact> =
        PER_METRE * (Exact::int(1) / Exact::dec(1609344, -3));

    /// One `PerMileSurvey` (`mi⁻¹ (U.S. survey)`), as a `LinearNumberDensity`.
    pub const PER_MILE_SURVEY: LinearNumberDensity<Exact> =
        PER_METRE * (Exact::int(1) / (Exact::int(6336000) / Exact::int(3937)));

    /// One `PerNauticalMile` (`M⁻¹`), as a `LinearNumberDensity`.
    pub const PER_NAUTICAL_MILE: LinearNumberDensity<Exact> =
        PER_METRE * (Exact::int(1) / Exact::int(1852));

    /// One `PerYard` (`yd⁻¹`), as a `LinearNumberDensity`.
    pub const PER_YARD: LinearNumberDensity<Exact> =
        PER_METRE * (Exact::int(1) / Exact::dec(9144, -4));

    /// One `PerFluidOunce` (`per fl oz`), as a `VolumetricNumberDensity`.
    pub const PER_FLUID_OUNCE: VolumetricNumberDensity<Exact> =
        (ONE / (METRE * METRE * METRE)) * (Exact::int(1) / Exact::dec(295735295625, -16));

    /// One `PerFluidOunceImperial` (`per fl oz (UK)`), as a `VolumetricNumberDensity`.
    pub const PER_FLUID_OUNCE_IMPERIAL: VolumetricNumberDensity<Exact> =
        (ONE / (METRE * METRE * METRE)) * (Exact::int(1) / Exact::dec(284130625, -13));

    /// One `PerGallonImperial` (`per gal (UK)`), as a `VolumetricNumberDensity`.
    pub const PER_GALLON_IMPERIAL: VolumetricNumberDensity<Exact> =
        (ONE / (METRE * METRE * METRE)) * (Exact::int(1) / Exact::dec(454609, -8));

    /// One `PerGallon` (`per gal`), as a `VolumetricNumberDensity`.
    pub const PER_GALLON: VolumetricNumberDensity<Exact> =
        (ONE / (METRE * METRE * METRE)) * (Exact::int(1) / Exact::dec(3785411784, -12));

    /// One `PerLitre` (`L⁻¹`), as a `VolumetricNumberDensity`.
    pub const PER_LITRE: VolumetricNumberDensity<Exact> =
        (ONE / (METRE * METRE * METRE)) * Exact::int(1000);
}

/// The units of `MassDensity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassDensity<N>, number = N)]
#[unit(for<N> where MassDensity<N>: [const] yadc::Unit<MassDensity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MassDensityUnit {
    #[default]
    #[unit("kg/m³", "kilograms per cubic metre", by = MassDensity::<N>::from(KILOGRAM / (METRE * METRE * METRE)))]
    KilogramPerCubicMetre,
    #[unit("g/m³", "grams per cubic metre", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * Exact::dec(1, -3)))]
    GramPerCubicMetre,
    #[unit("ct/m³", "carats per cubic metre", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * Exact::dec(2, -4)))]
    CaratPerCubicMetre,
    #[unit("gr/m³", "grains per cubic metre", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * Exact::dec(6479891, -11)))]
    GrainPerCubicMetre,
    #[unit("cwt long/m³", "hundredweight (long) per cubic metre", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * Exact::dec(5080234544, -8)))]
    HundredweightLongPerCubicMetre,
    #[unit("cwt short/m³", "hundredweight (short) per cubic metre", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * Exact::dec(45359237, -6)))]
    HundredweightShortPerCubicMetre,
    #[unit("oz/m³", "ounces per cubic metre", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * Exact::dec(28349523125, -12)))]
    OuncePerCubicMetre,
    #[unit("oz t/m³", "troy ounces per cubic metre", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * Exact::dec(311034768, -10)))]
    OunceTroyPerCubicMetre,
    #[unit("dwt/m³", "pennyweight per cubic metre", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * Exact::dec(155517384, -11)))]
    PennyweightPerCubicMetre,
    #[unit("lb/m³", "pounds per cubic metre", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * Exact::dec(45359237, -8)))]
    PoundPerCubicMetre,
    #[unit("lb t/m³", "troy pounds per cubic metre", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * Exact::dec(3732417216, -10)))]
    PoundTroyPerCubicMetre,
    #[unit("slug/m³", "slugs per cubic metre", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * (Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4))))]
    SlugPerCubicMetre,
    #[unit("AT/m³", "assay tons per cubic metre", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * (Exact::dec(175, -3) / Exact::int(6))))]
    TonAssayPerCubicMetre,
    #[unit("2240 lb/m³", "long tons per cubic metre", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * Exact::dec(10160469088, -7)))]
    TonLongPerCubicMetre,
    #[unit("2000 lb/m³", "short tons per cubic metre", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * Exact::dec(90718474, -5)))]
    TonShortPerCubicMetre,
    #[unit("t/m³", "tons per cubic metre", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * Exact::int(1000)))]
    TonPerCubicMetre,
    #[unit("gr/gal", "grains per gallon", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * (Exact::dec(6479891, -11) / Exact::dec(3785411784, -12))))]
    GrainPerGallon,
    #[unit("oz/in³", "ounces per cubic inch", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * (Exact::dec(28349523125, -12) / (Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4)))))]
    OuncePerCubicInch,
    #[unit("oz/gal (UK)", "ounces per Imperial gallon", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * (Exact::dec(28349523125, -12) / Exact::dec(454609, -8))))]
    OuncePerGallonImperial,
    #[unit("oz/gal", "ounces per gallon", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * (Exact::dec(28349523125, -12) / Exact::dec(3785411784, -12))))]
    OuncePerGallon,
    #[unit("lb/ft³", "pounds per cubic foot", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * (Exact::dec(45359237, -8) / (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)))))]
    PoundPerCubicFoot,
    #[unit("lb/in³", "pounds per cubic inch", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * (Exact::dec(45359237, -8) / (Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4)))))]
    PoundPerCubicInch,
    #[unit("lb/yd³", "pounds per cubic yard", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * (Exact::dec(45359237, -8) / (Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4)))))]
    PoundPerCubicYard,
    #[unit("lb/gal (UK)", "pounds per Imperial gallon", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * (Exact::dec(45359237, -8) / Exact::dec(454609, -8))))]
    PoundPerGallonImperial,
    #[unit("lb/gal", "pounds per gallon", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * (Exact::dec(45359237, -8) / Exact::dec(3785411784, -12))))]
    PoundPerGallon,
    #[unit("slug/ft³", "slugs per cubic foot", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * ((Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4)) / (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)))))]
    SlugPerCubicFoot,
    #[unit("2240 lb/yd³", "long tons per cubic yard", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * (Exact::dec(10160469088, -7) / (Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4)))))]
    TonLongPerCubicYard,
    #[unit("2000 lb/yd³", "short tons per cubic yard", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * (Exact::dec(90718474, -5) / (Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4)))))]
    TonShortPerCubicYard,
    #[unit("2240 lb/ft³", "long tons per cubic foot", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * (Exact::dec(10160469088, -7) / (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)))))]
    TonLongPerCubicFoot,
    #[unit("2000 lb/ft³", "short tons per cubic foot", by = MassDensity::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * (Exact::dec(90718474, -5) / (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)))))]
    TonShortPerCubicFoot,
}

/// The units of `LinearMassDensity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LinearMassDensity<N>, number = N)]
#[unit(for<N> where LinearMassDensity<N>: [const] yadc::Unit<LinearMassDensity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum LinearMassDensityUnit {
    #[default]
    #[unit("kg/m", "kilograms per metre", by = LinearMassDensity::<N>::from(KILOGRAM / METRE))]
    KilogramPerMetre,
    #[unit("oz/ft", "ounces per foot", by = LinearMassDensity::<N>::from((KILOGRAM / METRE) * (Exact::dec(28349523125, -12) / Exact::dec(3048, -4))))]
    OuncePerFoot,
    #[unit("oz/in", "ounces per inch", by = LinearMassDensity::<N>::from((KILOGRAM / METRE) * (Exact::dec(28349523125, -12) / Exact::dec(254, -4))))]
    OuncePerInch,
    #[unit("lb/yd", "pounds per yard", by = LinearMassDensity::<N>::from((KILOGRAM / METRE) * (Exact::dec(45359237, -8) / Exact::dec(9144, -4))))]
    PoundPerYard,
    #[unit("lb/ft", "pounds per foot", by = LinearMassDensity::<N>::from((KILOGRAM / METRE) * (Exact::dec(45359237, -8) / Exact::dec(3048, -4))))]
    PoundPerFoot,
    #[unit("lb/in", "pounds per inch", by = LinearMassDensity::<N>::from((KILOGRAM / METRE) * (Exact::dec(45359237, -8) / Exact::dec(254, -4))))]
    PoundPerInch,
    #[unit("g/km", "grams per kilometre", by = LinearMassDensity::<N>::from((KILOGRAM / METRE) * Exact::dec(1, -6)))]
    GramPerKilometre,
    #[unit("g/cm", "grams per centimetre", by = LinearMassDensity::<N>::from((KILOGRAM / METRE) * Exact::dec(1, -1)))]
    GramPerCentimetre,
}

/// The units of `ArealMassDensity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ArealMassDensity<N>, number = N)]
#[unit(for<N> where ArealMassDensity<N>: [const] yadc::Unit<ArealMassDensity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ArealMassDensityUnit {
    #[default]
    #[unit("kg/m²", "kilograms per square metre", by = ArealMassDensity::<N>::from(KILOGRAM / (METRE * METRE)))]
    KilogramPerSquareMetre,
    #[unit("g/m²", "grams per square metre", by = ArealMassDensity::<N>::from((KILOGRAM / (METRE * METRE)) * Exact::dec(1, -3)))]
    GramPerSquareMetre,
    #[unit("oz/ft²", "ounces per square foot", by = ArealMassDensity::<N>::from((KILOGRAM / (METRE * METRE)) * (Exact::dec(28349523125, -12) / (Exact::dec(3048, -4) * Exact::dec(3048, -4)))))]
    OuncePerSquareFoot,
}

/// The units of `LinearNumberDensity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LinearNumberDensity<N>, number = N)]
#[unit(for<N> where LinearNumberDensity<N>: [const] yadc::Unit<LinearNumberDensity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum LinearNumberDensityUnit {
    #[default]
    #[unit("m⁻¹", "per metre", by = LinearNumberDensity::<N>::from(PER_METRE))]
    PerMetre,
    #[unit("ft⁻¹", "per foot", by = LinearNumberDensity::<N>::from(PER_FOOT))]
    PerFoot,
    #[unit("ft (U.S. survey)", "foot (U.S. survey)", by = LinearNumberDensity::<N>::from(PER_FOOT_SURVEY))]
    PerFootSurvey,
    #[unit("in⁻¹", "per inch", by = LinearNumberDensity::<N>::from(PER_INCH))]
    PerInch,
    #[unit("mi⁻¹", "per mile", by = LinearNumberDensity::<N>::from(PER_MILE))]
    PerMile,
    #[unit("mi⁻¹ (U.S. survey)", "per mile (U.S. survey)", by = LinearNumberDensity::<N>::from(PER_MILE_SURVEY))]
    PerMileSurvey,
    #[unit("M⁻¹", "per nautical mile", by = LinearNumberDensity::<N>::from(PER_NAUTICAL_MILE))]
    PerNauticalMile,
    #[unit("yd⁻¹", "per yard", by = LinearNumberDensity::<N>::from(PER_YARD))]
    PerYard,
}

/// The units of `ArealNumberDensity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ArealNumberDensity<N>, number = N)]
#[unit(for<N> where ArealNumberDensity<N>: [const] yadc::Unit<ArealNumberDensity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ArealNumberDensityUnit {
    #[default]
    #[unit("m⁻²", "per square metre", by = ArealNumberDensity::<N>::from(ONE / (METRE * METRE)))]
    PerSquareMetre,
    #[unit("ac⁻²", "per acre", by = ArealNumberDensity::<N>::from((ONE / (METRE * METRE)) * (Exact::int(1) / (Exact::int(43560) * (Exact::int(1200) / Exact::int(3937)) * (Exact::int(1200) / Exact::int(3937))))))]
    PerAcre,
    #[unit("a⁻²", "per are", by = ArealNumberDensity::<N>::from((ONE / (METRE * METRE)) * Exact::dec(1, -2)))]
    PerAre,
    #[unit("b⁻²", "per barn", by = ArealNumberDensity::<N>::from((ONE / (METRE * METRE)) * Exact::int(10000000000000000000000000000)))]
    PerBarn,
    #[unit("cmil⁻²", "per circular mil", by = ArealNumberDensity::<N>::from((ONE / (METRE * METRE)) * (Exact::int(4) / (Exact::PI * Exact::dec(254, -7) * Exact::dec(254, -7)))))]
    PerCircularMil,
    #[unit("ha⁻²", "per hectare", by = ArealNumberDensity::<N>::from((ONE / (METRE * METRE)) * Exact::dec(1, -4)))]
    PerHectare,
    #[unit("ft⁻²", "per square foot", by = ArealNumberDensity::<N>::from((ONE / (METRE * METRE)) * (Exact::int(1) / (Exact::dec(3048, -4) * Exact::dec(3048, -4)))))]
    PerSquareFoot,
    #[unit("in⁻²", "per square inch", by = ArealNumberDensity::<N>::from((ONE / (METRE * METRE)) * (Exact::int(1) / (Exact::dec(254, -4) * Exact::dec(254, -4)))))]
    PerSquareInch,
    #[unit("mi⁻²", "per square mile", by = ArealNumberDensity::<N>::from((ONE / (METRE * METRE)) * (Exact::int(1) / (Exact::dec(1609344, -3) * Exact::dec(1609344, -3)))))]
    PerSquareMile,
    #[unit("yd⁻²", "per square yard", by = ArealNumberDensity::<N>::from((ONE / (METRE * METRE)) * (Exact::int(1) / (Exact::dec(9144, -4) * Exact::dec(9144, -4)))))]
    PerSquareYard,
}

/// The units of `VolumetricNumberDensity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = VolumetricNumberDensity<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, VolumetricNumberDensity<N>: [const] yadc::Unit<VolumetricNumberDensity<N>, N> + [const] core::marker::Destruct)]
pub enum VolumetricNumberDensityUnit {
    #[default]
    #[unit("m⁻³", "per cubic metre", by = VolumetricNumberDensity::<N>::from(ONE / (METRE * METRE * METRE)))]
    PerCubicMetre,
    #[unit("ft⁻³", "per cubic foot", by = VolumetricNumberDensity::<N>::from((ONE / (METRE * METRE * METRE)) * (Exact::int(1) / (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)))))]
    PerCubicFoot,
    #[unit("in⁻³", "per cubic inch", by = VolumetricNumberDensity::<N>::from((ONE / (METRE * METRE * METRE)) * (Exact::int(1) / (Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4)))))]
    PerCubicInch,
    #[unit("mi⁻³", "per cubic mile", by = VolumetricNumberDensity::<N>::from((ONE / (METRE * METRE * METRE)) * (Exact::int(1) / (Exact::dec(1609344, -3) * Exact::dec(1609344, -3) * Exact::dec(1609344, -3)))))]
    PerCubicMile,
    #[unit("yd⁻³", "per cubic yard", by = VolumetricNumberDensity::<N>::from((ONE / (METRE * METRE * METRE)) * (Exact::int(1) / (Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4)))))]
    PerCubicYard,
    #[unit("per fl oz", "per fluid ounce", by = VolumetricNumberDensity::<N>::from(PER_FLUID_OUNCE))]
    PerFluidOunce,
    #[unit("per fl oz (UK)", "per Imperial fluid ounce", by = VolumetricNumberDensity::<N>::from(PER_FLUID_OUNCE_IMPERIAL))]
    PerFluidOunceImperial,
    #[unit("per gal (UK)", "per Imperial gallon", by = VolumetricNumberDensity::<N>::from(PER_GALLON_IMPERIAL))]
    PerGallonImperial,
    #[unit("per gal", "per gallon", by = VolumetricNumberDensity::<N>::from(PER_GALLON))]
    PerGallon,
    #[unit("L⁻¹", "per litre", by = VolumetricNumberDensity::<N>::from(PER_LITRE))]
    PerLitre,
}

/// The units of `LinearNumberRate`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LinearNumberRate<N>, number = N)]
#[unit(for<N> where LinearNumberRate<N>: [const] yadc::Unit<LinearNumberRate<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum LinearNumberRateUnit {
    #[default]
    #[unit("m⁻¹ · s⁻¹", "per metre second", by = LinearNumberRate::<N>::from(ONE / (METRE * SECOND)))]
    PerMetreSecond,
    #[unit("ft⁻¹ · s⁻¹", "per foot second", by = LinearNumberRate::<N>::from((ONE / (METRE * SECOND)) * (Exact::int(1) / Exact::dec(3048, -4))))]
    PerFootSecond,
    #[unit("ft⁻¹ (U.S. survey) · s⁻¹", "per foot (U.S. survey) second", by = LinearNumberRate::<N>::from((ONE / (METRE * SECOND)) * (Exact::int(1) / (Exact::int(1200) / Exact::int(3937)))))]
    PerFootSurveySecond,
    #[unit("in⁻¹ · s⁻¹", "per inch second", by = LinearNumberRate::<N>::from((ONE / (METRE * SECOND)) * (Exact::int(1) / Exact::dec(254, -4))))]
    PerInchSecond,
    #[unit("mi⁻¹ · s⁻¹", "per mile second", by = LinearNumberRate::<N>::from((ONE / (METRE * SECOND)) * (Exact::int(1) / Exact::dec(1609344, -3))))]
    PerMileSecond,
    #[unit("mi⁻¹ (U.S. survey) · s⁻¹", "per mile (U.S. survey) second", by = LinearNumberRate::<N>::from((ONE / (METRE * SECOND)) * (Exact::int(1) / (Exact::int(6336000) / Exact::int(3937)))))]
    PerMileSurveySecond,
    #[unit("yd⁻¹ · s⁻¹", "per yard second", by = LinearNumberRate::<N>::from((ONE / (METRE * SECOND)) * (Exact::int(1) / Exact::dec(9144, -4))))]
    PerYardSecond,
}

/// The units of `ArealNumberRate`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ArealNumberRate<N>, number = N)]
#[unit(for<N> where ArealNumberRate<N>: [const] yadc::Unit<ArealNumberRate<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ArealNumberRateUnit {
    #[default]
    #[unit("m⁻² · s⁻¹", "per square metre second", by = ArealNumberRate::<N>::from(ONE / (METRE * METRE * SECOND)))]
    PerSquareMetreSecond,
    #[unit("ac⁻¹ · s⁻¹", "per acre second", by = ArealNumberRate::<N>::from((ONE / (METRE * METRE * SECOND)) * (Exact::int(1) / (Exact::int(43560) * (Exact::int(1200) / Exact::int(3937)) * (Exact::int(1200) / Exact::int(3937))))))]
    PerAcreSecond,
    #[unit("a⁻¹ · s⁻¹", "per are second", by = ArealNumberRate::<N>::from((ONE / (METRE * METRE * SECOND)) * Exact::dec(1, -2)))]
    PerAreSecond,
    #[unit("b⁻¹ · s⁻¹", "per barn second", by = ArealNumberRate::<N>::from((ONE / (METRE * METRE * SECOND)) * Exact::int(10000000000000000000000000000)))]
    PerBarnSecond,
    #[unit("cmil⁻¹ · s⁻¹", "per circular mil second", by = ArealNumberRate::<N>::from((ONE / (METRE * METRE * SECOND)) * (Exact::int(4) / (Exact::PI * Exact::dec(254, -7) * Exact::dec(254, -7)))))]
    PerCircularMilSecond,
    #[unit("ha⁻¹ · s⁻¹", "per hectare second", by = ArealNumberRate::<N>::from((ONE / (METRE * METRE * SECOND)) * Exact::dec(1, -4)))]
    PerHectareSecond,
    #[unit("ft⁻² · s⁻¹", "per square foot second", by = ArealNumberRate::<N>::from((ONE / (METRE * METRE * SECOND)) * (Exact::int(1) / (Exact::dec(3048, -4) * Exact::dec(3048, -4)))))]
    PerSquareFootSecond,
    #[unit("in⁻² · s⁻¹", "per square inch second", by = ArealNumberRate::<N>::from((ONE / (METRE * METRE * SECOND)) * (Exact::int(1) / (Exact::dec(254, -4) * Exact::dec(254, -4)))))]
    PerSquareInchSecond,
    #[unit("mi⁻² · s⁻¹", "per square mile second", by = ArealNumberRate::<N>::from((ONE / (METRE * METRE * SECOND)) * (Exact::int(1) / (Exact::dec(1609344, -3) * Exact::dec(1609344, -3)))))]
    PerSquareMileSecond,
    #[unit("yd⁻² · s⁻¹", "per square yard second", by = ArealNumberRate::<N>::from((ONE / (METRE * METRE * SECOND)) * (Exact::int(1) / (Exact::dec(9144, -4) * Exact::dec(9144, -4)))))]
    PerSquareYardSecond,
}

/// The units of `VolumetricNumberRate`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = VolumetricNumberRate<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, VolumetricNumberRate<N>: [const] yadc::Unit<VolumetricNumberRate<N>, N> + [const] core::marker::Destruct)]
pub enum VolumetricNumberRateUnit {
    #[default]
    #[unit("m⁻³ · s⁻¹", "per cubic metre second", by = VolumetricNumberRate::<N>::from(ONE / (METRE * METRE * METRE * SECOND)))]
    PerCubicMetreSecond,
    #[unit("ft⁻³ · s⁻¹", "per cubic foot second", by = VolumetricNumberRate::<N>::from((ONE / (METRE * METRE * METRE * SECOND)) * (Exact::int(1) / (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)))))]
    PerCubicFootSecond,
    #[unit("in⁻³ · s⁻¹", "per cubic inch second", by = VolumetricNumberRate::<N>::from((ONE / (METRE * METRE * METRE * SECOND)) * (Exact::int(1) / (Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4)))))]
    PerCubicInchSecond,
    #[unit("mi⁻³ · s⁻¹", "per cubic mile second", by = VolumetricNumberRate::<N>::from((ONE / (METRE * METRE * METRE * SECOND)) * (Exact::int(1) / (Exact::dec(1609344, -3) * Exact::dec(1609344, -3) * Exact::dec(1609344, -3)))))]
    PerCubicMileSecond,
    #[unit("yd⁻³ · s⁻¹", "per cubic yard second", by = VolumetricNumberRate::<N>::from((ONE / (METRE * METRE * METRE * SECOND)) * (Exact::int(1) / (Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4)))))]
    PerCubicYardSecond,
    #[unit("fl oz⁻¹ · s⁻¹", "per fluid ounce second", by = VolumetricNumberRate::<N>::from((ONE / (METRE * METRE * METRE * SECOND)) * (Exact::int(1) / Exact::dec(295735295625, -16))))]
    PerFluidOunceSecond,
    #[unit("fl oz⁻¹ (UK) · s⁻¹", "per Imperial fluid ounce second", by = VolumetricNumberRate::<N>::from((ONE / (METRE * METRE * METRE * SECOND)) * (Exact::int(1) / Exact::dec(284130625, -13))))]
    PerFluidOunceImperialSecond,
    #[unit("gal⁻¹ (UK) · s⁻¹", "per Imperial gallon second", by = VolumetricNumberRate::<N>::from((ONE / (METRE * METRE * METRE * SECOND)) * (Exact::int(1) / Exact::dec(454609, -8))))]
    PerGallonImperialSecond,
    #[unit("gal⁻¹ · s⁻¹", "per gallon second", by = VolumetricNumberRate::<N>::from((ONE / (METRE * METRE * METRE * SECOND)) * (Exact::int(1) / Exact::dec(3785411784, -12))))]
    PerGallonSecond,
    #[unit("L⁻¹ · s⁻¹", "per litre second", by = VolumetricNumberRate::<N>::from((ONE / (METRE * METRE * METRE * SECOND)) * Exact::int(1000)))]
    PerLitreSecond,
    #[unit("Bq/m³", "becquerels per cubic metre", by = VolumetricNumberRate::<N>::from(ONE / (METRE * METRE * METRE * SECOND)))]
    BecquerelPerCubicMetre,
    #[unit("Ci/m³", "curies per cubic metre", by = VolumetricNumberRate::<N>::from((ONE / (METRE * METRE * METRE * SECOND)) * Exact::int(37000000000)))]
    CuriePerCubicMetre,
    #[unit("dpm/m³", "disintegrations per minute per cubic metre", by = VolumetricNumberRate::<N>::from((ONE / (METRE * METRE * METRE * SECOND)) * (Exact::int(1) / Exact::int(60))))]
    DisintegrationsPerMinutePerCubicMetre,
}

/// The units of `LinearDensityOfStates`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LinearDensityOfStates<N>, number = N)]
#[unit(for<N> where LinearDensityOfStates<N>: [const] yadc::Unit<LinearDensityOfStates<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum LinearDensityOfStatesUnit {
    #[default]
    #[unit("1/(m · J)", "states per metre joule", by = LinearDensityOfStates::<N>::from(SECOND * SECOND / (METRE * METRE * METRE * KILOGRAM)))]
    StatePerMetreJoule,
}

/// The units of `ArealDensityOfStates`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ArealDensityOfStates<N>, number = N)]
#[unit(for<N> where ArealDensityOfStates<N>: [const] yadc::Unit<ArealDensityOfStates<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ArealDensityOfStatesUnit {
    #[default]
    #[unit("1/(m² · J)", "states per square metre joule", by = ArealDensityOfStates::<N>::from(SECOND * SECOND / (METRE * METRE * METRE * METRE * KILOGRAM)))]
    StatePerSquareMetreJoule,
}

/// The units of `VolumetricDensityOfStates`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = VolumetricDensityOfStates<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, VolumetricDensityOfStates<N>: [const] yadc::Unit<VolumetricDensityOfStates<N>, N> + [const] core::marker::Destruct)]
pub enum VolumetricDensityOfStatesUnit {
    #[default]
    #[unit("1/(m³ · J)", "states per cubic metre joule", by = VolumetricDensityOfStates::<N>::from(SECOND * SECOND / (METRE * METRE * METRE * METRE * METRE * KILOGRAM)))]
    StatePerCubicMetreJoule,
}

/// The units of `LinearPowerDensity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LinearPowerDensity<N>, number = N)]
#[unit(for<N> where LinearPowerDensity<N>: [const] yadc::Unit<LinearPowerDensity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum LinearPowerDensityUnit {
    #[default]
    #[unit("W/m", "watts per metre", by = LinearPowerDensity::<N>::from(METRE * KILOGRAM / (SECOND * SECOND * SECOND)))]
    WattPerMetre,
}

/// The units of `VolumetricPowerDensity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = VolumetricPowerDensity<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, VolumetricPowerDensity<N>: [const] yadc::Unit<VolumetricPowerDensity<N>, N> + [const] core::marker::Destruct)]
pub enum VolumetricPowerDensityUnit {
    #[default]
    #[unit("W/m³", "watts per cubic metre", by = VolumetricPowerDensity::<N>::from(KILOGRAM / (METRE * SECOND * SECOND * SECOND)))]
    WattPerCubicMetre,
}
