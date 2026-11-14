//! The units of `fluid`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::dynamic::DynamicSIUnit;
use crate::fluid::units::*;
use crate::fluid::*;
use crate::{Exact, SIQuantity, Unit};
use core::ops::{Add, Div, Mul, Sub};

/// The units of `DynamicViscosity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<DYNAMIC_VISCOSITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<DYNAMIC_VISCOSITY, N>;
        fn value(&self, quantity: SIQuantity<DYNAMIC_VISCOSITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum DynamicViscosityUnit {
    #[default]
    #[value(PASCAL_SECOND)]
    PascalSecond,
    #[value(POISE)]
    Poise,
    #[value(POUND_PER_FOOT_SECOND)]
    PoundPerFootSecond,
    #[value(SLUG_PER_FOOT_SECOND)]
    SlugPerFootSecond,
    #[value(GRAM_PER_CENTIMETRE_SECOND)]
    GramPerCentimetreSecond,
}

/// The units of `KinematicViscosity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<KINEMATIC_VISCOSITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<KINEMATIC_VISCOSITY, N>;
        fn value(&self, quantity: SIQuantity<KINEMATIC_VISCOSITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum KinematicViscosityUnit {
    #[default]
    #[value(KINEMATIC_VISCOSITY_SQUARE_METRE_PER_SECOND)]
    SquareMetrePerSecond,
    #[value(KINEMATIC_VISCOSITY_STOKES)]
    Stokes,
}

/// The units of `SurfaceTension`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<SURFACE_TENSION, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<SURFACE_TENSION, N>;
        fn value(&self, quantity: SIQuantity<SURFACE_TENSION, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum SurfaceTensionUnit {
    #[default]
    #[value(NEWTON_PER_METRE)]
    NewtonPerMetre,
    #[value(KILOGRAM_FORCE_PER_METRE)]
    KilogramForcePerMetre,
    #[value(OUNCE_FORCE_PER_INCH)]
    OunceForcePerInch,
    #[value(POUNDAL_PER_INCH)]
    PoundalPerInch,
    #[value(POUND_FORCE_PER_INCH)]
    PoundForcePerInch,
    #[value(JOULE_PER_SQUARE_METRE)]
    JoulePerSquareMetre,
}

/// The units of `CompressibilityCoefficient`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<COMPRESSIBILITY_COEFFICIENT, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<COMPRESSIBILITY_COEFFICIENT, N>;
        fn value(&self, quantity: SIQuantity<COMPRESSIBILITY_COEFFICIENT, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum CompressibilityCoefficientUnit {
    #[default]
    #[value(PER_PASCAL)]
    PerPascal,
    #[value(PER_ATMOSPHERE)]
    PerAtmosphere,
    #[value(PER_ATMOSPHERE_TECHNICAL)]
    PerAtmosphereTechnical,
    #[value(PER_BAR)]
    PerBar,
}

/// The units of `MassRate`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<MASS_RATE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<MASS_RATE, N>;
        fn value(&self, quantity: SIQuantity<MASS_RATE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MassRateUnit {
    #[default]
    #[value(KILOGRAM_PER_SECOND)]
    KilogramPerSecond,
    #[value(GRAM_PER_SECOND)]
    GramPerSecond,
    #[value(GRAM_PER_MINUTE)]
    GramPerMinute,
    #[value(GRAM_PER_HOUR)]
    GramPerHour,
    #[value(GRAM_PER_DAY)]
    GramPerDay,
    #[value(CARAT_PER_SECOND)]
    CaratPerSecond,
    #[value(GRAIN_PER_SECOND)]
    GrainPerSecond,
    #[value(HUNDREDWEIGHT_LONG_PER_SECOND)]
    HundredweightLongPerSecond,
    #[value(HUNDREDWEIGHT_SHORT_PER_SECOND)]
    HundredweightShortPerSecond,
    #[value(OUNCE_PER_SECOND)]
    OuncePerSecond,
    #[value(OUNCE_TROY_PER_SECOND)]
    OunceTroyPerSecond,
    #[value(PENNYWEIGHT_PER_SECOND)]
    PennyweightPerSecond,
    #[value(POUND_PER_SECOND)]
    PoundPerSecond,
    #[value(POUND_PER_MINUTE)]
    PoundPerMinute,
    #[value(POUND_PER_HOUR)]
    PoundPerHour,
    #[value(POUND_PER_DAY)]
    PoundPerDay,
    #[value(POUND_TROY_PER_SECOND)]
    PoundTroyPerSecond,
    #[value(SLUG_PER_SECOND)]
    SlugPerSecond,
    #[value(TON_ASSAY_PER_SECOND)]
    TonAssayPerSecond,
    #[value(TON_LONG_PER_SECOND)]
    TonLongPerSecond,
    #[value(TON_SHORT_PER_SECOND)]
    TonShortPerSecond,
    #[value(TON_SHORT_PER_HOUR)]
    TonShortPerHour,
    #[value(TON_PER_SECOND)]
    TonPerSecond,
    #[value(TON_PER_MINUTE)]
    TonPerMinute,
    #[value(TON_PER_HOUR)]
    TonPerHour,
    #[value(TON_PER_DAY)]
    TonPerDay,
}

/// The units of `VolumeRate`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<VOLUME_RATE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<VOLUME_RATE, N>;
        fn value(&self, quantity: SIQuantity<VOLUME_RATE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum VolumeRateUnit {
    #[default]
    #[value(CUBIC_METRE_PER_SECOND)]
    CubicMetrePerSecond,
    #[value(CUBIC_METRE_PER_MINUTE)]
    CubicMetrePerMinute,
    #[value(CUBIC_METRE_PER_HOUR)]
    CubicMetrePerHour,
    #[value(LITRE_PER_SECOND)]
    LitrePerSecond,
    #[value(LITRE_PER_MINUTE)]
    LitrePerMinute,
    #[value(ACRE_FOOT_PER_SECOND)]
    AcreFootPerSecond,
    #[value(BARREL_PER_SECOND)]
    BarrelPerSecond,
    #[value(BUSHEL_PER_SECOND)]
    BushelPerSecond,
    #[value(CORD_PER_SECOND)]
    CordPerSecond,
    #[value(CUBIC_FOOT_PER_SECOND)]
    CubicFootPerSecond,
    #[value(CUBIC_FOOT_PER_MINUTE)]
    CubicFootPerMinute,
    #[value(CUBIC_INCH_PER_SECOND)]
    CubicInchPerSecond,
    #[value(CUBIC_INCH_PER_MINUTE)]
    CubicInchPerMinute,
    #[value(CUBIC_MILE_PER_SECOND)]
    CubicMilePerSecond,
    #[value(CUBIC_YARD_PER_SECOND)]
    CubicYardPerSecond,
    #[value(CUBIC_YARD_PER_MINUTE)]
    CubicYardPerMinute,
    #[value(CUP_PER_SECOND)]
    CupPerSecond,
    #[value(FLUID_OUNCE_PER_SECOND)]
    FluidOuncePerSecond,
    #[value(FLUID_OUNCE_IMPERIAL_PER_SECOND)]
    FluidOunceImperialPerSecond,
    #[value(GALLON_IMPERIAL_PER_SECOND)]
    GallonImperialPerSecond,
    #[value(GALLON_PER_SECOND)]
    GallonPerSecond,
    #[value(GALLON_PER_MINUTE)]
    GallonPerMinute,
    #[value(GALLON_PER_DAY)]
    GallonPerDay,
    #[value(GILL_IMPERIAL_PER_SECOND)]
    GillImperialPerSecond,
    #[value(GILL_PER_SECOND)]
    GillPerSecond,
    #[value(PECK_PER_SECOND)]
    PeckPerSecond,
    #[value(PINT_DRY_PER_SECOND)]
    PintDryPerSecond,
    #[value(PINT_LIQUID_PER_SECOND)]
    PintLiquidPerSecond,
    #[value(QUART_DRY_PER_SECOND)]
    QuartDryPerSecond,
    #[value(QUART_LIQUID_PER_SECOND)]
    QuartLiquidPerSecond,
    #[value(STERE_PER_SECOND)]
    SterePerSecond,
    #[value(TABLESPOON_PER_SECOND)]
    TablespoonPerSecond,
    #[value(TEASPOON_PER_SECOND)]
    TeaspoonPerSecond,
    #[value(REGISTER_TON_PER_SECOND)]
    RegisterTonPerSecond,
}

/// The units of `MassFlux`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<MASS_FLUX, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<MASS_FLUX, N>;
        fn value(&self, quantity: SIQuantity<MASS_FLUX, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MassFluxUnit {
    #[default]
    #[value(KILOGRAM_PER_SQUARE_METRE_SECOND)]
    KilogramPerSquareMetreSecond,
}

/// The units of `MassPerEnergy`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<MASS_PER_ENERGY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<MASS_PER_ENERGY, N>;
        fn value(&self, quantity: SIQuantity<MASS_PER_ENERGY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MassPerEnergyUnit {
    #[default]
    #[value(KILOGRAM_PER_JOULE)]
    KilogramPerJoule,
    #[value(GRAM_PER_JOULE)]
    GramPerJoule,
    #[value(POUND_PER_JOULE)]
    PoundPerJoule,
    #[value(POUND_PER_WATT_HOUR)]
    PoundPerWattHour,
    #[value(GRAM_PER_WATT_HOUR)]
    GramPerWattHour,
}

/// The units of `DiffusionCoefficient`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<DIFFUSION_COEFFICIENT, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<DIFFUSION_COEFFICIENT, N>;
        fn value(&self, quantity: SIQuantity<DIFFUSION_COEFFICIENT, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum DiffusionCoefficientUnit {
    #[default]
    #[value(DIFFUSION_COEFFICIENT_SQUARE_METRE_PER_SECOND)]
    SquareMetrePerSecond,
    #[value(DIFFUSION_COEFFICIENT_STOKES)]
    Stokes,
}
