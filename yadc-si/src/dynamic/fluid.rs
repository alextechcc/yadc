//! The units of `fluid`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::base::{KILOGRAM, METRE, SECOND};
use crate::fluid::consts::*;
use crate::fluid::*;
use core::marker::Destruct;
use yadc::{Unit, num_traits::Num};

/// The units of `DynamicViscosity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = DynamicViscosity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where DynamicViscosity<N>: [const] Unit<DynamicViscosity<N>, N> + [const] Destruct)]
pub enum DynamicViscosityUnit {
    #[default]
    #[unit("Pa · s", "pascal seconds", by = DynamicViscosity::<N>::from(KILOGRAM / (METRE * SECOND)))]
    PascalSecond,
    #[unit("P", "poises", by = DynamicViscosity::<N>::from(POISE))]
    Poise,
    #[unit("lb/(ft · s)", "pounds per foot second", by = DynamicViscosity::<N>::from((KILOGRAM / (METRE * SECOND)) * (Exact::dec(45359237, -8) / Exact::dec(3048, -4))))]
    PoundPerFootSecond,
    #[unit("slug/(ft · s)", "slugs per foot second", by = DynamicViscosity::<N>::from((KILOGRAM / (METRE * SECOND)) * ((Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4)) / Exact::dec(3048, -4))))]
    SlugPerFootSecond,
    #[unit("g/(cm · s)", "grams per centimetre second", by = DynamicViscosity::<N>::from((KILOGRAM / (METRE * SECOND)) * Exact::dec(1, -1)))]
    GramPerCentimetreSecond,
}

/// The units of `KinematicViscosity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = KinematicViscosity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where KinematicViscosity<N>: [const] Unit<KinematicViscosity<N>, N> + [const] Destruct)]
pub enum KinematicViscosityUnit {
    #[default]
    #[unit("m²/s", "square metres per second", by = KinematicViscosity::<N>::from(METRE * METRE / SECOND))]
    SquareMetrePerSecond,
    #[unit("St", "stokes", by = KinematicViscosity::<N>::from(KINEMATIC_VISCOSITY_STOKES))]
    Stokes,
}

/// The units of `SurfaceTension`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SurfaceTension<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where SurfaceTension<N>: [const] Unit<SurfaceTension<N>, N> + [const] Destruct)]
pub enum SurfaceTensionUnit {
    #[default]
    #[unit("N/m", "newtons per metre", by = SurfaceTension::<N>::from(KILOGRAM / (SECOND * SECOND)))]
    NewtonPerMetre,
    #[unit("kgf/m", "kilograms-force per metre", by = SurfaceTension::<N>::from((KILOGRAM / (SECOND * SECOND)) * Exact::dec(980665, -5)))]
    KilogramForcePerMetre,
    #[unit("ozf/in", "ounces-force per inch", by = SurfaceTension::<N>::from((KILOGRAM / (SECOND * SECOND)) * ((Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::int(16)) / Exact::dec(254, -4))))]
    OunceForcePerInch,
    #[unit("pdl/in", "poundals per inch", by = SurfaceTension::<N>::from((KILOGRAM / (SECOND * SECOND)) * ((Exact::dec(45359237, -8) * Exact::dec(3048, -4)) / Exact::dec(254, -4))))]
    PoundalPerInch,
    #[unit("lbf/in", "pounds-force per inch", by = SurfaceTension::<N>::from((KILOGRAM / (SECOND * SECOND)) * ((Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / Exact::dec(254, -4))))]
    PoundForcePerInch,
    #[unit("J/m²", "joules per square metre", by = SurfaceTension::<N>::from(KILOGRAM / (SECOND * SECOND)))]
    JoulePerSquareMetre,
}

/// The units of `CompressibilityCoefficient`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = CompressibilityCoefficient<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where CompressibilityCoefficient<N>: [const] Unit<CompressibilityCoefficient<N>, N> + [const] Destruct)]
pub enum CompressibilityCoefficientUnit {
    #[default]
    #[unit("Pa⁻¹", "per pascal", by = CompressibilityCoefficient::<N>::from(PER_PASCAL))]
    PerPascal,
    #[unit("atm⁻¹", "per atmosphere", by = CompressibilityCoefficient::<N>::from(PER_ATMOSPHERE))]
    PerAtmosphere,
    #[unit("at⁻¹", "per atmosphere (technical)", by = CompressibilityCoefficient::<N>::from(PER_ATMOSPHERE_TECHNICAL))]
    PerAtmosphereTechnical,
    #[unit("bar⁻¹", "per bar", by = CompressibilityCoefficient::<N>::from(PER_BAR))]
    PerBar,
}

/// The units of `MassRate`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassRate<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MassRate<N>: [const] Unit<MassRate<N>, N> + [const] Destruct)]
pub enum MassRateUnit {
    #[default]
    #[unit("kg/s", "kilograms per second", by = MassRate::<N>::from(KILOGRAM / SECOND))]
    KilogramPerSecond,
    #[unit("g/s", "grams per second", by = MassRate::<N>::from((KILOGRAM / SECOND) * Exact::dec(1, -3)))]
    GramPerSecond,
    #[unit("g/min", "grams per minute", by = MassRate::<N>::from((KILOGRAM / SECOND) * (Exact::dec(1, -3) / Exact::int(60))))]
    GramPerMinute,
    #[unit("g/h", "grams per hour", by = MassRate::<N>::from((KILOGRAM / SECOND) * (Exact::dec(1, -3) / Exact::int(3600))))]
    GramPerHour,
    #[unit("g/d", "grams per day", by = MassRate::<N>::from((KILOGRAM / SECOND) * (Exact::dec(1, -3) / Exact::int(86400))))]
    GramPerDay,
    #[unit("ct/s", "carats per second", by = MassRate::<N>::from((KILOGRAM / SECOND) * Exact::dec(2, -4)))]
    CaratPerSecond,
    #[unit("gr/s", "grains per second", by = MassRate::<N>::from((KILOGRAM / SECOND) * Exact::dec(6479891, -11)))]
    GrainPerSecond,
    #[unit("cwt long/s", "hundredweight (long) per second", by = MassRate::<N>::from((KILOGRAM / SECOND) * Exact::dec(5080234544, -8)))]
    HundredweightLongPerSecond,
    #[unit("cwt short/s", "hundredweight (short) per second", by = MassRate::<N>::from((KILOGRAM / SECOND) * Exact::dec(45359237, -6)))]
    HundredweightShortPerSecond,
    #[unit("oz/s", "ounces per second", by = MassRate::<N>::from((KILOGRAM / SECOND) * Exact::dec(28349523125, -12)))]
    OuncePerSecond,
    #[unit("oz t/s", "troy ounces per second", by = MassRate::<N>::from((KILOGRAM / SECOND) * Exact::dec(311034768, -10)))]
    OunceTroyPerSecond,
    #[unit("dwt/s", "pennyweight per second", by = MassRate::<N>::from((KILOGRAM / SECOND) * Exact::dec(155517384, -11)))]
    PennyweightPerSecond,
    #[unit("lb/s", "pounds per second", by = MassRate::<N>::from((KILOGRAM / SECOND) * Exact::dec(45359237, -8)))]
    PoundPerSecond,
    #[unit("lb/min", "pounds per minute", by = MassRate::<N>::from((KILOGRAM / SECOND) * (Exact::dec(45359237, -8) / Exact::int(60))))]
    PoundPerMinute,
    #[unit("lb/h", "pounds per hour", by = MassRate::<N>::from((KILOGRAM / SECOND) * (Exact::dec(45359237, -8) / Exact::int(3600))))]
    PoundPerHour,
    #[unit("lb/d", "pounds per day", by = MassRate::<N>::from((KILOGRAM / SECOND) * (Exact::dec(45359237, -8) / Exact::int(86400))))]
    PoundPerDay,
    #[unit("lb t/s", "troy pounds per second", by = MassRate::<N>::from((KILOGRAM / SECOND) * Exact::dec(3732417216, -10)))]
    PoundTroyPerSecond,
    #[unit("slug/s", "slugs per second", by = MassRate::<N>::from((KILOGRAM / SECOND) * (Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4))))]
    SlugPerSecond,
    #[unit("AT/s", "assay tons per second", by = MassRate::<N>::from((KILOGRAM / SECOND) * (Exact::dec(175, -3) / Exact::int(6))))]
    TonAssayPerSecond,
    #[unit("2240 lb/s", "long tons per second", by = MassRate::<N>::from((KILOGRAM / SECOND) * Exact::dec(10160469088, -7)))]
    TonLongPerSecond,
    #[unit("2000 lb/s", "short tons per second", by = MassRate::<N>::from((KILOGRAM / SECOND) * Exact::dec(90718474, -5)))]
    TonShortPerSecond,
    #[unit("2000 lb/h", "short tons per hour", by = MassRate::<N>::from((KILOGRAM / SECOND) * (Exact::dec(90718474, -5) / Exact::int(3600))))]
    TonShortPerHour,
    #[unit("t/s", "tons per second", by = MassRate::<N>::from((KILOGRAM / SECOND) * Exact::int(1000)))]
    TonPerSecond,
    #[unit("t/min", "tons per minute", by = MassRate::<N>::from((KILOGRAM / SECOND) * (Exact::int(1000) / Exact::int(60))))]
    TonPerMinute,
    #[unit("t/h", "tons per hour", by = MassRate::<N>::from((KILOGRAM / SECOND) * (Exact::int(1000) / Exact::int(3600))))]
    TonPerHour,
    #[unit("t/d", "tons per day", by = MassRate::<N>::from((KILOGRAM / SECOND) * (Exact::int(1000) / Exact::int(86400))))]
    TonPerDay,
}

/// The units of `VolumeRate`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = VolumeRate<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where VolumeRate<N>: [const] Unit<VolumeRate<N>, N> + [const] Destruct)]
pub enum VolumeRateUnit {
    #[default]
    #[unit("m³/s", "cubic metres per second", by = VolumeRate::<N>::from(METRE * METRE * METRE / SECOND))]
    CubicMetrePerSecond,
    #[unit("m³/min", "cubic metres per minute", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * (Exact::int(1) / Exact::int(60))))]
    CubicMetrePerMinute,
    #[unit("m³/h", "cubic metres per hour", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * (Exact::int(1) / Exact::int(3600))))]
    CubicMetrePerHour,
    #[unit("L/s", "litres per second", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * Exact::dec(1, -3)))]
    LitrePerSecond,
    #[unit("L/min", "litres per minute", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * (Exact::dec(1, -3) / Exact::int(60))))]
    LitrePerMinute,
    #[unit("ac · ft/s", "acre-feet per second", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * (Exact::int(43560) * (Exact::int(1200) / Exact::int(3937)) * (Exact::int(1200) / Exact::int(3937)) * (Exact::int(1200) / Exact::int(3937)))))]
    AcreFootPerSecond,
    #[unit("bbl/s", "barrels per second", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * Exact::dec(158987294928, -12)))]
    BarrelPerSecond,
    #[unit("bu/s", "bushels per second", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * Exact::dec(3523907016688, -14)))]
    BushelPerSecond,
    #[unit("cords/s", "cords per second", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * (Exact::int(128) * Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4))))]
    CordPerSecond,
    #[unit("ft³/s", "cubic feet per second", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4))))]
    CubicFootPerSecond,
    #[unit("ft³/min", "cubic feet per minute", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4) / Exact::int(60))))]
    CubicFootPerMinute,
    #[unit("in³/s", "cubic inches per second", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * (Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4))))]
    CubicInchPerSecond,
    #[unit("in³/min", "cubic inches per minute", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * (Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4) / Exact::int(60))))]
    CubicInchPerMinute,
    #[unit("mi³/s", "cubic miles per second", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * (Exact::dec(1609344, -3) * Exact::dec(1609344, -3) * Exact::dec(1609344, -3))))]
    CubicMilePerSecond,
    #[unit("yd³/s", "cubic yards per second", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * (Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4))))]
    CubicYardPerSecond,
    #[unit("yd³/min", "cubic yards per minute", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * (Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4) / Exact::int(60))))]
    CubicYardPerMinute,
    #[unit("cup/s", "cups per second", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * Exact::dec(2365882365, -13)))]
    CupPerSecond,
    #[unit("fl oz/s", "fluid ounces per second", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * Exact::dec(295735295625, -16)))]
    FluidOuncePerSecond,
    #[unit("fl oz (UK)/s", "Imperial fluid ounces per second", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * Exact::dec(284130625, -13)))]
    FluidOunceImperialPerSecond,
    #[unit("gal (UK)/s", "Imperial gallons per second", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * Exact::dec(454609, -8)))]
    GallonImperialPerSecond,
    #[unit("gal/s", "gallons per second", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * Exact::dec(3785411784, -12)))]
    GallonPerSecond,
    #[unit("gal/min", "gallons per minute", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * (Exact::dec(3785411784, -12) / Exact::int(60))))]
    GallonPerMinute,
    #[unit("gal/d", "gallons per day", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * (Exact::dec(3785411784, -12) / Exact::int(86400))))]
    GallonPerDay,
    #[unit("gi (UK)/s", "Imperial gills per second", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * Exact::dec(1420653125, -13)))]
    GillImperialPerSecond,
    #[unit("gi/s", "gills per second", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * Exact::dec(11829411825, -14)))]
    GillPerSecond,
    #[unit("pk/s", "pecks per second", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * Exact::dec(880976754172, -14)))]
    PeckPerSecond,
    #[unit("dry pt/s", "dry pints per second", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * Exact::dec(5506104713575, -16)))]
    PintDryPerSecond,
    #[unit("liq pt/s", "liquid pints per second", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * Exact::dec(473176473, -12)))]
    PintLiquidPerSecond,
    #[unit("dry qt/s", "dry quarts per second", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * Exact::dec(1101220942715, -15)))]
    QuartDryPerSecond,
    #[unit("liq qt/s", "liquid quarts per second", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * Exact::dec(946352946, -12)))]
    QuartLiquidPerSecond,
    #[unit("st/s", "steres per second", by = VolumeRate::<N>::from(METRE * METRE * METRE / SECOND))]
    SterePerSecond,
    #[unit("tbsp/s", "tablespoons per second", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * Exact::dec(1478676478125, -17)))]
    TablespoonPerSecond,
    #[unit("tsp/s", "teaspoons per second", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * Exact::dec(492892159375, -17)))]
    TeaspoonPerSecond,
    #[unit("RT/s", "register tons per second", by = VolumeRate::<N>::from((METRE * METRE * METRE / SECOND) * (Exact::int(100) * Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4))))]
    RegisterTonPerSecond,
}

/// The units of `MassFlux`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassFlux<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MassFlux<N>: [const] Unit<MassFlux<N>, N> + [const] Destruct)]
pub enum MassFluxUnit {
    #[default]
    #[unit("kg/(m² · s)", "kilograms per square metre second", by = MassFlux::<N>::from(KILOGRAM / (METRE * METRE * SECOND)))]
    KilogramPerSquareMetreSecond,
}

/// The units of `MassPerEnergy`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassPerEnergy<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MassPerEnergy<N>: [const] Unit<MassPerEnergy<N>, N> + [const] Destruct)]
pub enum MassPerEnergyUnit {
    #[default]
    #[unit("kg/J", "kilograms per joule", by = MassPerEnergy::<N>::from(SECOND * SECOND / (METRE * METRE)))]
    KilogramPerJoule,
    #[unit("g/J", "grams per joule", by = MassPerEnergy::<N>::from((SECOND * SECOND / (METRE * METRE)) * Exact::dec(1, -3)))]
    GramPerJoule,
    #[unit("lb/J", "pounds per joule", by = MassPerEnergy::<N>::from((SECOND * SECOND / (METRE * METRE)) * Exact::dec(45359237, -8)))]
    PoundPerJoule,
    #[unit("lb/Wh", "pounds per watt hour", by = MassPerEnergy::<N>::from((SECOND * SECOND / (METRE * METRE)) * (Exact::dec(45359237, -8) / Exact::int(3600))))]
    PoundPerWattHour,
    #[unit("g/Wh", "grams per watt hour", by = MassPerEnergy::<N>::from((SECOND * SECOND / (METRE * METRE)) * (Exact::dec(1, -3) / Exact::int(3600))))]
    GramPerWattHour,
}

/// The units of `DiffusionCoefficient`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = DiffusionCoefficient<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where DiffusionCoefficient<N>: [const] Unit<DiffusionCoefficient<N>, N> + [const] Destruct)]
pub enum DiffusionCoefficientUnit {
    #[default]
    #[unit("m²/s", "square metres per second", by = DiffusionCoefficient::<N>::from(METRE * METRE / SECOND))]
    SquareMetrePerSecond,
    #[unit("St", "Stokes", by = DiffusionCoefficient::<N>::from(DIFFUSION_COEFFICIENT_STOKES))]
    Stokes,
}
