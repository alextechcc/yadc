//! The units of `density`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::base::{KILOGRAM, METRE, ONE, SECOND};
use crate::density::consts::*;
use crate::density::*;
use core::marker::Destruct;
use yadc::{Unit, num_traits::Num};

/// The units of `MassDensity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassDensity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MassDensity<N>: [const] Unit<MassDensity<N>, N> + [const] Destruct)]
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

/// The units of `LinearMassDensity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LinearMassDensity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where LinearMassDensity<N>: [const] Unit<LinearMassDensity<N>, N> + [const] Destruct)]
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

/// The units of `ArealMassDensity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ArealMassDensity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ArealMassDensity<N>: [const] Unit<ArealMassDensity<N>, N> + [const] Destruct)]
pub enum ArealMassDensityUnit {
    #[default]
    #[unit("kg/m²", "kilograms per square metre", by = ArealMassDensity::<N>::from(KILOGRAM / (METRE * METRE)))]
    KilogramPerSquareMetre,
    #[unit("g/m²", "grams per square metre", by = ArealMassDensity::<N>::from((KILOGRAM / (METRE * METRE)) * Exact::dec(1, -3)))]
    GramPerSquareMetre,
    #[unit("oz/ft²", "ounces per square foot", by = ArealMassDensity::<N>::from((KILOGRAM / (METRE * METRE)) * (Exact::dec(28349523125, -12) / (Exact::dec(3048, -4) * Exact::dec(3048, -4)))))]
    OuncePerSquareFoot,
}

/// The units of `LinearNumberDensity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LinearNumberDensity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where LinearNumberDensity<N>: [const] Unit<LinearNumberDensity<N>, N> + [const] Destruct)]
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

/// The units of `ArealNumberDensity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ArealNumberDensity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ArealNumberDensity<N>: [const] Unit<ArealNumberDensity<N>, N> + [const] Destruct)]
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

/// The units of `VolumetricNumberDensity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = VolumetricNumberDensity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where VolumetricNumberDensity<N>: [const] Unit<VolumetricNumberDensity<N>, N> + [const] Destruct)]
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

/// The units of `LinearNumberRate`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LinearNumberRate<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where LinearNumberRate<N>: [const] Unit<LinearNumberRate<N>, N> + [const] Destruct)]
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

/// The units of `ArealNumberRate`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ArealNumberRate<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ArealNumberRate<N>: [const] Unit<ArealNumberRate<N>, N> + [const] Destruct)]
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

/// The units of `VolumetricNumberRate`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = VolumetricNumberRate<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where VolumetricNumberRate<N>: [const] Unit<VolumetricNumberRate<N>, N> + [const] Destruct)]
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

/// The units of `LinearDensityOfStates`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LinearDensityOfStates<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where LinearDensityOfStates<N>: [const] Unit<LinearDensityOfStates<N>, N> + [const] Destruct)]
pub enum LinearDensityOfStatesUnit {
    #[default]
    #[unit("1/(m · J)", "states per metre joule", by = LinearDensityOfStates::<N>::from(SECOND * SECOND / (METRE * METRE * METRE * KILOGRAM)))]
    StatePerMetreJoule,
}

/// The units of `ArealDensityOfStates`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ArealDensityOfStates<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ArealDensityOfStates<N>: [const] Unit<ArealDensityOfStates<N>, N> + [const] Destruct)]
pub enum ArealDensityOfStatesUnit {
    #[default]
    #[unit("1/(m² · J)", "states per square metre joule", by = ArealDensityOfStates::<N>::from(SECOND * SECOND / (METRE * METRE * METRE * METRE * KILOGRAM)))]
    StatePerSquareMetreJoule,
}

/// The units of `VolumetricDensityOfStates`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = VolumetricDensityOfStates<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where VolumetricDensityOfStates<N>: [const] Unit<VolumetricDensityOfStates<N>, N> + [const] Destruct)]
pub enum VolumetricDensityOfStatesUnit {
    #[default]
    #[unit("1/(m³ · J)", "states per cubic metre joule", by = VolumetricDensityOfStates::<N>::from(SECOND * SECOND / (METRE * METRE * METRE * METRE * METRE * KILOGRAM)))]
    StatePerCubicMetreJoule,
}

/// The units of `LinearPowerDensity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LinearPowerDensity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where LinearPowerDensity<N>: [const] Unit<LinearPowerDensity<N>, N> + [const] Destruct)]
pub enum LinearPowerDensityUnit {
    #[default]
    #[unit("W/m", "watts per metre", by = LinearPowerDensity::<N>::from(METRE * KILOGRAM / (SECOND * SECOND * SECOND)))]
    WattPerMetre,
}

/// The units of `VolumetricPowerDensity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = VolumetricPowerDensity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where VolumetricPowerDensity<N>: [const] Unit<VolumetricPowerDensity<N>, N> + [const] Destruct)]
pub enum VolumetricPowerDensityUnit {
    #[default]
    #[unit("W/m³", "watts per cubic metre", by = VolumetricPowerDensity::<N>::from(KILOGRAM / (METRE * SECOND * SECOND * SECOND)))]
    WattPerCubicMetre,
}
