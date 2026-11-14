//! The units of `chemistry`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::chemistry::units::*;
use crate::chemistry::*;
use crate::dynamic::DynamicSIUnit;
use crate::{Exact, SIQuantity, Unit};
use core::ops::{Add, Div, Mul, Sub};

/// The units of `AmountOfSubstance`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<AMOUNT_OF_SUBSTANCE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<AMOUNT_OF_SUBSTANCE, N>;
        fn value(&self, quantity: SIQuantity<AMOUNT_OF_SUBSTANCE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum AmountOfSubstanceUnit {
    #[default]
    #[value(MOLE)]
    Mole,
    #[value(PARTICLE)]
    Particle,
}

/// The units of `MolarConcentration`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<MOLAR_CONCENTRATION, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<MOLAR_CONCENTRATION, N>;
        fn value(&self, quantity: SIQuantity<MOLAR_CONCENTRATION, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MolarConcentrationUnit {
    #[default]
    #[value(MOLE_PER_CUBIC_METRE)]
    MolePerCubicMetre,
    #[value(MOLE_PER_LITRE)]
    MolePerLitre,
    #[value(PARTICLE_PER_CUBIC_METRE)]
    ParticlePerCubicMetre,
}

/// The units of `MolarEnergy`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<MOLAR_ENERGY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<MOLAR_ENERGY, N>;
        fn value(&self, quantity: SIQuantity<MOLAR_ENERGY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MolarEnergyUnit {
    #[default]
    #[value(KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE)]
    KilogramSquareMetrePerSecondSquaredMole,
    #[value(JOULE_PER_MOLE)]
    JoulePerMole,
    #[value(WATT_HOUR_PER_MOLE)]
    WattHourPerMole,
    #[value(BTU_IT_PER_MOLE)]
    BtuItPerMole,
    #[value(BTU_PER_MOLE)]
    BtuPerMole,
    #[value(CALORIE_IT_PER_MOLE)]
    CalorieItPerMole,
    #[value(CALORIE_PER_MOLE)]
    CaloriePerMole,
    #[value(CALORIE_IT_NUTRITION_PER_MOLE)]
    CalorieItNutritionPerMole,
    #[value(CALORIE_NUTRITION_PER_MOLE)]
    CalorieNutritionPerMole,
    #[value(ELECTRONVOLT_PER_MOLE)]
    ElectronvoltPerMole,
    #[value(ERG_PER_MOLE)]
    ErgPerMole,
    #[value(FOOT_POUNDAL_PER_MOLE)]
    FootPoundalPerMole,
    #[value(FOOT_POUND_FORCE_PER_MOLE)]
    FootPoundForcePerMole,
    #[value(QUAD_PER_MOLE)]
    QuadPerMole,
    #[value(THERM_EC_PER_MOLE)]
    ThermEcPerMole,
    #[value(THERM_US_PER_MOLE)]
    ThermUsPerMole,
    #[value(TON_TNT_PER_MOLE)]
    TonTntPerMole,
    #[value(WATT_SECOND_PER_MOLE)]
    WattSecondPerMole,
    #[value(JOULE_PER_PARTICLE)]
    JoulePerParticle,
    #[value(ELECTRONVOLT_PER_PARTICLE)]
    ElectronvoltPerParticle,
}

/// The units of `MolarFlux`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<MOLAR_FLUX, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<MOLAR_FLUX, N>;
        fn value(&self, quantity: SIQuantity<MOLAR_FLUX, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MolarFluxUnit {
    #[default]
    #[value(MOLE_PER_SQUARE_METRE_SECOND)]
    MolePerSquareMetreSecond,
}

/// The units of `MolarHeatCapacity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<MOLAR_HEAT_CAPACITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<MOLAR_HEAT_CAPACITY, N>;
        fn value(&self, quantity: SIQuantity<MOLAR_HEAT_CAPACITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MolarHeatCapacityUnit {
    #[default]
    #[value(JOULE_PER_KELVIN_MOLE)]
    JoulePerKelvinMole,
    #[value(BTU_IT_PER_KELVIN_MOLE)]
    BtuItPerKelvinMole,
    #[value(BTU_PER_KELVIN_MOLE)]
    BtuPerKelvinMole,
    #[value(CALORIE_IT_PER_KELVIN_MOLE)]
    CalorieItPerKelvinMole,
    #[value(CALORIE_PER_KELVIN_MOLE)]
    CaloriePerKelvinMole,
    #[value(CALORIE_IT_NUTRITION_PER_KELVIN_MOLE)]
    CalorieItNutritionPerKelvinMole,
    #[value(CALORIE_NUTRITION_PER_KELVIN_MOLE)]
    CalorieNutritionPerKelvinMole,
    #[value(JOULE_PER_KELVIN_PARTICLE)]
    JoulePerKelvinParticle,
    #[value(ELECTRONVOLT_PER_KELVIN_PARTICLE)]
    ElectronvoltPerKelvinParticle,
    #[value(MOLAR_GAS_CONSTANT)]
    MolarGasConstant,
}

/// The units of `MolarMass`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<MOLAR_MASS, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<MOLAR_MASS, N>;
        fn value(&self, quantity: SIQuantity<MOLAR_MASS, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MolarMassUnit {
    #[default]
    #[value(KILOGRAM_PER_MOLE)]
    KilogramPerMole,
    #[value(GRAM_PER_MOLE)]
    GramPerMole,
}

/// The units of `MolarRadioactivity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<MOLAR_RADIOACTIVITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<MOLAR_RADIOACTIVITY, N>;
        fn value(&self, quantity: SIQuantity<MOLAR_RADIOACTIVITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MolarRadioactivityUnit {
    #[default]
    #[value(BECQUEREL_PER_MOLE)]
    BecquerelPerMole,
    #[value(CURIE_PER_MOLE)]
    CuriePerMole,
    #[value(DISINTEGRATIONS_PER_MINUTE_PER_MOLE)]
    DisintegrationsPerMinutePerMole,
}

/// The units of `MolarVolume`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<MOLAR_VOLUME, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<MOLAR_VOLUME, N>;
        fn value(&self, quantity: SIQuantity<MOLAR_VOLUME, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MolarVolumeUnit {
    #[default]
    #[value(CUBIC_METRE_PER_MOLE)]
    CubicMetrePerMole,
    #[value(CUBIC_METRE_PER_PARTICLE)]
    CubicMetrePerParticle,
}

/// The units of `Molality`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<MOLALITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<MOLALITY, N>;
        fn value(&self, quantity: SIQuantity<MOLALITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MolalityUnit {
    #[default]
    #[value(MOLE_PER_KILOGRAM)]
    MolePerKilogram,
}

/// The units of `MassConcentration`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<MASS_CONCENTRATION, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<MASS_CONCENTRATION, N>;
        fn value(&self, quantity: SIQuantity<MASS_CONCENTRATION, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MassConcentrationUnit {
    #[default]
    #[value(KILOGRAM_PER_CUBIC_METRE)]
    KilogramPerCubicMetre,
    #[value(GRAM_PER_CUBIC_METRE)]
    GramPerCubicMetre,
}

/// The units of `CatalyticActivity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<CATALYTIC_ACTIVITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<CATALYTIC_ACTIVITY, N>;
        fn value(&self, quantity: SIQuantity<CATALYTIC_ACTIVITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum CatalyticActivityUnit {
    #[default]
    #[value(KATAL)]
    Katal,
    #[value(PARTICLE_PER_SECOND)]
    ParticlePerSecond,
    #[value(MOLE_PER_SECOND)]
    MolePerSecond,
}

/// The units of `CatalyticActivityConcentration`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<CATALYTIC_ACTIVITY_CONCENTRATION, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<CATALYTIC_ACTIVITY_CONCENTRATION, N>;
        fn value(&self, quantity: SIQuantity<CATALYTIC_ACTIVITY_CONCENTRATION, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum CatalyticActivityConcentrationUnit {
    #[default]
    #[value(KATAL_PER_CUBIC_METRE)]
    KatalPerCubicMetre,
    #[value(KATAL_PER_LITRE)]
    KatalPerLitre,
    #[value(PARTICLE_PER_SECOND_CUBIC_METRE)]
    ParticlePerSecondCubicMetre,
}

/// The units of `Radioactivity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<RADIOACTIVITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<RADIOACTIVITY, N>;
        fn value(&self, quantity: SIQuantity<RADIOACTIVITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum RadioactivityUnit {
    #[default]
    #[value(BECQUEREL)]
    Becquerel,
    #[value(CURIE)]
    Curie,
    #[value(DISINTEGRATIONS_PER_MINUTE)]
    DisintegrationsPerMinute,
}

/// The units of `SpecificRadioactivity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<SPECIFIC_RADIOACTIVITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<SPECIFIC_RADIOACTIVITY, N>;
        fn value(&self, quantity: SIQuantity<SPECIFIC_RADIOACTIVITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum SpecificRadioactivityUnit {
    #[default]
    #[value(BECQUEREL_PER_KILOGRAM)]
    BecquerelPerKilogram,
    #[value(CURIE_PER_KILOGRAM)]
    CuriePerKilogram,
    #[value(DISINTEGRATIONS_PER_MINUTE_PER_KILOGRAM)]
    DisintegrationsPerMinutePerKilogram,
}

/// The units of `InverseAmountOfSubstance`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<INVERSE_AMOUNT_OF_SUBSTANCE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<INVERSE_AMOUNT_OF_SUBSTANCE, N>;
        fn value(&self, quantity: SIQuantity<INVERSE_AMOUNT_OF_SUBSTANCE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum InverseAmountOfSubstanceUnit {
    #[default]
    #[value(PER_MOLE)]
    PerMole,
    #[value(AVOGADRO_CONSTANT)]
    AvogadroConstant,
}
