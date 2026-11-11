use crate::{Exact, ISQ, SIQuantity, counted};
use consts::*;
use yadc::Dimension;

pub type AmountOfSubstance<N> = SIQuantity<
    {
        ISQ {
            mol: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MolarConcentration<N> = SIQuantity<
    {
        ISQ {
            m: -3,
            mol: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MolarEnergy<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -2,
            mol: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MolarFlux<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            s: -1,
            mol: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MolarHeatCapacity<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -2,
            K: -1,
            mol: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MolarMass<N> = SIQuantity<
    {
        ISQ {
            kg: 1,
            mol: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MolarRadioactivity<N> = SIQuantity<
    {
        ISQ {
            s: -1,
            mol: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MolarVolume<N> = SIQuantity<
    {
        ISQ {
            m: 3,
            mol: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Molality<N> = SIQuantity<
    {
        ISQ {
            kg: -1,
            mol: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MassConcentration<N> = SIQuantity<
    {
        ISQ {
            m: -3,
            kg: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type CatalyticActivity<N> = SIQuantity<
    {
        ISQ {
            s: -1,
            mol: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type CatalyticActivityConcentration<N> = SIQuantity<
    {
        ISQ {
            m: -3,
            s: -1,
            mol: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Radioactivity<N> = SIQuantity<
    {
        ISQ {
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type SpecificRadioactivity<N> = SIQuantity<
    {
        ISQ {
            kg: -1,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type InverseAmountOfSubstance<N> = SIQuantity<
    {
        ISQ {
            mol: -1,
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
    use crate::base::{KELVIN, KILOGRAM, METRE, ONE, SECOND};

    /// The mole, the SI base unit of amount of substance.
    pub use crate::base::MOLE;

    /// One `Particle` (particle), as an `AmountOfSubstance`.
    pub const PARTICLE<N: const From<Exact>>: AmountOfSubstance<N> = counted((Exact::int(1) / Exact::int(602214076000000000000000)) * MOLE::<Exact>);

    /// One `MolePerCubicMetre` (mol/m³), as a `MolarConcentration`.
    pub const MOLE_PER_CUBIC_METRE<N: const From<Exact>>: MolarConcentration<N> = counted(MOLE::<Exact> / (METRE::<Exact> * METRE::<Exact> * METRE::<Exact>));

    /// One `MolePerLitre` (mol/L), as a `MolarConcentration`.
    pub const MOLE_PER_LITRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::int(1000) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `MillimolePerLitre` (mmol/L), as a `MolarConcentration`.
    pub const MILLIMOLE_PER_LITRE<N: const From<Exact>>: MolarConcentration<N> = counted(MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `ParticlePerCubicMetre` (particle/m³), as a `MolarConcentration`.
    pub const PARTICLE_PER_CUBIC_METRE<N: const From<Exact>>: MolarConcentration<N> = counted((Exact::int(1) / Exact::int(602214076000000000000000)) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `KilogramSquareMetrePerSecondSquaredMole` (kg · m²/(s² · mol)), as a `MolarEnergy`.
    pub const KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * MOLE::<Exact>));

    /// One `JoulePerMole` (J/mol), as a `MolarEnergy`.
    pub const JOULE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `WattHourPerMole` (W · h/mol), as a `MolarEnergy`.
    pub const WATT_HOUR_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(3600) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `BtuItPerMole` (Btu (IT)/mol), as a `MolarEnergy`.
    pub const BTU_IT_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::dec(105505585262, -8) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `BtuPerMole` (Btu/mol), as a `MolarEnergy`.
    pub const BTU_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted((Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1)) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `CalorieItPerMole` (cal (IT)/mol), as a `MolarEnergy`.
    pub const CALORIE_IT_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::dec(41868, -4) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `CaloriePerMole` (cal/mol), as a `MolarEnergy`.
    pub const CALORIE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::dec(4184, -3) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `CalorieItNutritionPerMole` (Cal (IT)/mol), as a `MolarEnergy`.
    pub const CALORIE_IT_NUTRITION_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::dec(41868, -1) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `CalorieNutritionPerMole` (Cal/mol), as a `MolarEnergy`.
    pub const CALORIE_NUTRITION_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(4184) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `ElectronvoltPerMole` (eV/mol), as a `MolarEnergy`.
    pub const ELECTRONVOLT_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::dec(1602176634, -28) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `ErgPerMole` (erg/mol), as a `MolarEnergy`.
    pub const ERG_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::dec(1, -7) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `FootPoundalPerMole` (ft · pdl/mol), as a `MolarEnergy`.
    pub const FOOT_POUNDAL_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted((Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(3048, -4)) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `FootPoundForcePerMole` (ft · lbf/mol), as a `MolarEnergy`.
    pub const FOOT_POUND_FORCE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted((Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `QuadPerMole` (10¹⁵ Btu (IT)/mol), as a `MolarEnergy`.
    pub const QUAD_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(1055055852620000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `ThermEcPerMole` (thm (EC)/mol), as a `MolarEnergy`.
    pub const THERM_EC_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(105506000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `ThermUsPerMole` (thm/mol), as a `MolarEnergy`.
    pub const THERM_US_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(105480400) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `TonTntPerMole` (t of TNT/mol), as a `MolarEnergy`.
    pub const TON_TNT_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(4184000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `WattSecondPerMole` (W · s/mol), as a `MolarEnergy`.
    pub const WATT_SECOND_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `JoulePerParticle` (J/particle), as a `MolarEnergy`.
    pub const JOULE_PER_PARTICLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(602214076000000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `ElectronvoltPerParticle` (eV/particle), as a `MolarEnergy`.
    pub const ELECTRONVOLT_PER_PARTICLE<N: const From<Exact>>: MolarEnergy<N> = counted((Exact::dec(1602176634, -28) * Exact::int(602214076000000000000000)) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `MolePerSquareMetreSecond` (mol/(m² · s)), as a `MolarFlux`.
    pub const MOLE_PER_SQUARE_METRE_SECOND<N: const From<Exact>>: MolarFlux<N> = counted(MOLE::<Exact> / (METRE::<Exact> * METRE::<Exact> * SECOND::<Exact>));

    /// One `JoulePerKelvinMole` (J/(K · mol)), as a `MolarHeatCapacity`.
    pub const JOULE_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * KELVIN::<Exact> * MOLE::<Exact>));

    /// One `BtuItPerKelvinMole` (Btu (IT)/(K · mol)), as a `MolarHeatCapacity`.
    pub const BTU_IT_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::dec(105505585262, -8) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `BtuPerKelvinMole` (Btu/(K · mol)), as a `MolarHeatCapacity`.
    pub const BTU_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted((Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1)) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `CalorieItPerKelvinMole` (cal (IT)/(K · mol)), as a `MolarHeatCapacity`.
    pub const CALORIE_IT_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::dec(41868, -4) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `CaloriePerKelvinMole` (cal/(K · mol)), as a `MolarHeatCapacity`.
    pub const CALORIE_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::dec(4184, -3) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `CalorieItNutritionPerKelvinMole` (Cal (IT)/(K · mol)), as a `MolarHeatCapacity`.
    pub const CALORIE_IT_NUTRITION_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::dec(41868, -1) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `CalorieNutritionPerKelvinMole` (Cal/(K · mol)), as a `MolarHeatCapacity`.
    pub const CALORIE_NUTRITION_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::int(4184) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `JoulePerKelvinParticle` (J/(K · particle)), as a `MolarHeatCapacity`.
    pub const JOULE_PER_KELVIN_PARTICLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::int(602214076000000000000000) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `ElectronvoltPerKelvinParticle` (eV/(K · particle)), as a `MolarHeatCapacity`.
    pub const ELECTRONVOLT_PER_KELVIN_PARTICLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted((Exact::dec(1602176634, -28) * Exact::int(602214076000000000000000)) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `MolarGasConstant` (R), as a `MolarHeatCapacity`.
    pub const MOLAR_GAS_CONSTANT<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::dec(8314462618, -9) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `KilogramPerMole` (kg/mol), as a `MolarMass`.
    pub const KILOGRAM_PER_MOLE<N: const From<Exact>>: MolarMass<N> = counted(KILOGRAM::<Exact> / MOLE::<Exact>);

    /// One `GramPerMole` (g/mol), as a `MolarMass`.
    pub const GRAM_PER_MOLE<N: const From<Exact>>: MolarMass<N> = counted(Exact::dec(1, -3) * KILOGRAM_PER_MOLE::<Exact>);

    /// One `BecquerelPerMole` (Bq/mol), as a `MolarRadioactivity`.
    pub const BECQUEREL_PER_MOLE<N: const From<Exact>>: MolarRadioactivity<N> = counted(ONE::<Exact> / (SECOND::<Exact> * MOLE::<Exact>));

    /// One `CuriePerMole` (Ci/mol), as a `MolarRadioactivity`.
    pub const CURIE_PER_MOLE<N: const From<Exact>>: MolarRadioactivity<N> = counted(Exact::int(37000000000) * BECQUEREL_PER_MOLE::<Exact>);

    /// One `DisintegrationsPerMinutePerMole` (dpm/mol), as a `MolarRadioactivity`.
    pub const DISINTEGRATIONS_PER_MINUTE_PER_MOLE<N: const From<Exact>>: MolarRadioactivity<N> = counted((Exact::int(1) / Exact::int(60)) * BECQUEREL_PER_MOLE::<Exact>);

    /// One `CubicMetrePerMole` (m³/mol), as a `MolarVolume`.
    pub const CUBIC_METRE_PER_MOLE<N: const From<Exact>>: MolarVolume<N> = counted(METRE::<Exact> * METRE::<Exact> * METRE::<Exact> / MOLE::<Exact>);

    /// One `CubicMetrePerParticle` (m³/particle), as a `MolarVolume`.
    pub const CUBIC_METRE_PER_PARTICLE<N: const From<Exact>>: MolarVolume<N> = counted(Exact::int(602214076000000000000000) * CUBIC_METRE_PER_MOLE::<Exact>);

    /// One `MolePerKilogram` (mol/kg), as a `Molality`.
    pub const MOLE_PER_KILOGRAM<N: const From<Exact>>: Molality<N> = counted(MOLE::<Exact> / KILOGRAM::<Exact>);

    /// One `KilogramPerCubicMetre` (kg/m³), as a `MassConcentration`.
    pub const KILOGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassConcentration<N> = counted(KILOGRAM::<Exact> / (METRE::<Exact> * METRE::<Exact> * METRE::<Exact>));

    /// One `GramPerCubicMetre` (g/m³), as a `MassConcentration`.
    pub const GRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassConcentration<N> = counted(Exact::dec(1, -3) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `Katal` (kat), as a `CatalyticActivity`.
    pub const KATAL<N: const From<Exact>>: CatalyticActivity<N> = counted(MOLE::<Exact> / SECOND::<Exact>);

    /// One `ParticlePerSecond` (particle/s), as a `CatalyticActivity`.
    pub const PARTICLE_PER_SECOND<N: const From<Exact>>: CatalyticActivity<N> = counted((Exact::int(1) / Exact::int(602214076000000000000000)) * KATAL::<Exact>);

    /// One `MolePerSecond` (mol/s), as a `CatalyticActivity`.
    pub const MOLE_PER_SECOND<N: const From<Exact>>: CatalyticActivity<N> = counted(KATAL::<Exact>);

    /// One `KatalPerCubicMetre` (kat/m³), as a `CatalyticActivityConcentration`.
    pub const KATAL_PER_CUBIC_METRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(MOLE::<Exact> / (METRE::<Exact> * METRE::<Exact> * METRE::<Exact> * SECOND::<Exact>));

    /// One `KatalPerLitre` (kat/L), as a `CatalyticActivityConcentration`.
    pub const KATAL_PER_LITRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::int(1000) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `MillikatalPerLitre` (mkat/L), as a `CatalyticActivityConcentration`.
    pub const MILLIKATAL_PER_LITRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `ParticlePerSecondCubicMetre` (particle · s⁻¹ · m⁻³), as a `CatalyticActivityConcentration`.
    pub const PARTICLE_PER_SECOND_CUBIC_METRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted((Exact::int(1) / Exact::int(602214076000000000000000)) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `Becquerel` (Bq), as a `Radioactivity`.
    pub const BECQUEREL<N: const From<Exact>>: Radioactivity<N> = counted(ONE::<Exact> / SECOND::<Exact>);

    /// One `Curie` (Ci), as a `Radioactivity`.
    pub const CURIE<N: const From<Exact>>: Radioactivity<N> = counted(Exact::int(37000000000) * BECQUEREL::<Exact>);

    /// One `DisintegrationsPerMinute` (dpm), as a `Radioactivity`.
    pub const DISINTEGRATIONS_PER_MINUTE<N: const From<Exact>>: Radioactivity<N> = counted((Exact::int(1) / Exact::int(60)) * BECQUEREL::<Exact>);

    /// One `BecquerelPerKilogram` (Bq/kg), as a `SpecificRadioactivity`.
    pub const BECQUEREL_PER_KILOGRAM<N: const From<Exact>>: SpecificRadioactivity<N> = counted(ONE::<Exact> / (KILOGRAM::<Exact> * SECOND::<Exact>));

    /// One `CuriePerKilogram` (Ci/kg), as a `SpecificRadioactivity`.
    pub const CURIE_PER_KILOGRAM<N: const From<Exact>>: SpecificRadioactivity<N> = counted(Exact::int(37000000000) * BECQUEREL_PER_KILOGRAM::<Exact>);

    /// One `DisintegrationsPerMinutePerKilogram` (dpm/kg), as a `SpecificRadioactivity`.
    pub const DISINTEGRATIONS_PER_MINUTE_PER_KILOGRAM<N: const From<Exact>>: SpecificRadioactivity<N> = counted((Exact::int(1) / Exact::int(60)) * BECQUEREL_PER_KILOGRAM::<Exact>);

    /// One `PerMole` (mol⁻¹), as an `InverseAmountOfSubstance`.
    pub const PER_MOLE<N: const From<Exact>>: InverseAmountOfSubstance<N> = counted(ONE::<Exact> / MOLE::<Exact>);

    /// One `AvogadroConstant` (N_A), as an `InverseAmountOfSubstance`.
    pub const AVOGADRO_CONSTANT<N: const From<Exact>>: InverseAmountOfSubstance<N> = counted(Exact::int(602214076000000000000000) * PER_MOLE::<Exact>);
}

/// The units of `AmountOfSubstance`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AmountOfSubstance<f64>)]
pub enum AmountOfSubstanceUnit {
    #[default]
    #[unit("mol", "moles", by = MOLE)]
    Mole,
    #[unit("particle", "particles", by = PARTICLE)]
    Particle,
}

/// The units of `MolarConcentration`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarConcentration<f64>)]
pub enum MolarConcentrationUnit {
    #[default]
    #[unit("mol/m³", "moles per cubic metre", by = MOLE_PER_CUBIC_METRE)]
    MolePerCubicMetre,
    #[unit("mol/L", "moles per litre", by = MOLE_PER_LITRE)]
    MolePerLitre,
    #[unit("mmol/L", "millimoles per litre", by = MILLIMOLE_PER_LITRE)]
    MillimolePerLitre,
    #[unit("particle/m³", "particles per cubic metre", by = PARTICLE_PER_CUBIC_METRE)]
    ParticlePerCubicMetre,
}

/// The units of `MolarEnergy`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarEnergy<f64>)]
pub enum MolarEnergyUnit {
    #[default]
    #[unit("kg · m²/(s² · mol)", "kilograms square metre per second squared mole", by = KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE)]
    KilogramSquareMetrePerSecondSquaredMole,
    #[unit("J/mol", "joules per mole", by = JOULE_PER_MOLE)]
    JoulePerMole,
    #[unit("W · h/mol", "watt hours per mole", by = WATT_HOUR_PER_MOLE)]
    WattHourPerMole,
    #[unit("Btu (IT)/mol", "British thermal units (IT) per mole", by = BTU_IT_PER_MOLE)]
    BtuItPerMole,
    #[unit("Btu/mol", "British thermal units per mole", by = BTU_PER_MOLE)]
    BtuPerMole,
    #[unit("cal (IT)/mol", "calories (IT) per mole", by = CALORIE_IT_PER_MOLE)]
    CalorieItPerMole,
    #[unit("cal/mol", "calories per mole", by = CALORIE_PER_MOLE)]
    CaloriePerMole,
    #[unit("Cal (IT)/mol", "Calories (IT) per mole", by = CALORIE_IT_NUTRITION_PER_MOLE)]
    CalorieItNutritionPerMole,
    #[unit("Cal/mol", "Calories per mole", by = CALORIE_NUTRITION_PER_MOLE)]
    CalorieNutritionPerMole,
    #[unit("eV/mol", "electronvolts per mole", by = ELECTRONVOLT_PER_MOLE)]
    ElectronvoltPerMole,
    #[unit("erg/mol", "ergs per mole", by = ERG_PER_MOLE)]
    ErgPerMole,
    #[unit("ft · pdl/mol", "foot poundals per mole", by = FOOT_POUNDAL_PER_MOLE)]
    FootPoundalPerMole,
    #[unit("ft · lbf/mol", "foot pounds-force per mole", by = FOOT_POUND_FORCE_PER_MOLE)]
    FootPoundForcePerMole,
    #[unit("10¹⁵ Btu (IT)/mol", "quads per mole", by = QUAD_PER_MOLE)]
    QuadPerMole,
    #[unit("thm (EC)/mol", "therms (EC) per mole", by = THERM_EC_PER_MOLE)]
    ThermEcPerMole,
    #[unit("thm/mol", "therms per mole", by = THERM_US_PER_MOLE)]
    ThermUsPerMole,
    #[unit("t of TNT/mol", "tons of TNT per mole", by = TON_TNT_PER_MOLE)]
    TonTntPerMole,
    #[unit("W · s/mol", "watt seconds per mole", by = WATT_SECOND_PER_MOLE)]
    WattSecondPerMole,
    #[unit("J/particle", "joules per particle", by = JOULE_PER_PARTICLE)]
    JoulePerParticle,
    #[unit("eV/particle", "electronvolts per particle", by = ELECTRONVOLT_PER_PARTICLE)]
    ElectronvoltPerParticle,
}

/// The units of `MolarFlux`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarFlux<f64>)]
pub enum MolarFluxUnit {
    #[default]
    #[unit("mol/(m² · s)", "moles per square metre second", by = MOLE_PER_SQUARE_METRE_SECOND)]
    MolePerSquareMetreSecond,
}

/// The units of `MolarHeatCapacity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarHeatCapacity<f64>)]
pub enum MolarHeatCapacityUnit {
    #[default]
    #[unit("J/(K · mol)", "joules per kelvin mole", by = JOULE_PER_KELVIN_MOLE)]
    JoulePerKelvinMole,
    #[unit("Btu (IT)/(K · mol)", "British thermal units (IT) per kelvin mole", by = BTU_IT_PER_KELVIN_MOLE)]
    BtuItPerKelvinMole,
    #[unit("Btu/(K · mol)", "British thermal units per kelvin mole", by = BTU_PER_KELVIN_MOLE)]
    BtuPerKelvinMole,
    #[unit("cal (IT)/(K · mol)", "calories (IT) per kelvin mole", by = CALORIE_IT_PER_KELVIN_MOLE)]
    CalorieItPerKelvinMole,
    #[unit("cal/(K · mol)", "calories per kelvin mole", by = CALORIE_PER_KELVIN_MOLE)]
    CaloriePerKelvinMole,
    #[unit("Cal (IT)/(K · mol)", "Calories (IT) per kelvin mole", by = CALORIE_IT_NUTRITION_PER_KELVIN_MOLE)]
    CalorieItNutritionPerKelvinMole,
    #[unit("Cal/(K · mol)", "Calories per kelvin mole", by = CALORIE_NUTRITION_PER_KELVIN_MOLE)]
    CalorieNutritionPerKelvinMole,
    #[unit("J/(K · particle)", "joules per kelvin particle", by = JOULE_PER_KELVIN_PARTICLE)]
    JoulePerKelvinParticle,
    #[unit("eV/(K · particle)", "electronvolts per kelvin particle", by = ELECTRONVOLT_PER_KELVIN_PARTICLE)]
    ElectronvoltPerKelvinParticle,
    #[unit("R", "molar gas constants", by = MOLAR_GAS_CONSTANT)]
    MolarGasConstant,
}

/// The units of `MolarMass`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarMass<f64>)]
pub enum MolarMassUnit {
    #[default]
    #[unit("kg/mol", "kilograms per mole", by = KILOGRAM_PER_MOLE)]
    KilogramPerMole,
    #[unit("g/mol", "grams per mole", by = GRAM_PER_MOLE)]
    GramPerMole,
}

/// The units of `MolarRadioactivity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarRadioactivity<f64>)]
pub enum MolarRadioactivityUnit {
    #[default]
    #[unit("Bq/mol", "becquerels per mole", by = BECQUEREL_PER_MOLE)]
    BecquerelPerMole,
    #[unit("Ci/mol", "curies per mole", by = CURIE_PER_MOLE)]
    CuriePerMole,
    #[unit("dpm/mol", "disintegrations per minute per mole", by = DISINTEGRATIONS_PER_MINUTE_PER_MOLE)]
    DisintegrationsPerMinutePerMole,
}

/// The units of `MolarVolume`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarVolume<f64>)]
pub enum MolarVolumeUnit {
    #[default]
    #[unit("m³/mol", "cubic metres per mole", by = CUBIC_METRE_PER_MOLE)]
    CubicMetrePerMole,
    #[unit("m³/particle", "cubic metres per particle", by = CUBIC_METRE_PER_PARTICLE)]
    CubicMetrePerParticle,
}

/// The units of `Molality`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Molality<f64>)]
pub enum MolalityUnit {
    #[default]
    #[unit("mol/kg", "moles per kilogram", by = MOLE_PER_KILOGRAM)]
    MolePerKilogram,
}

/// The units of `MassConcentration`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassConcentration<f64>)]
pub enum MassConcentrationUnit {
    #[default]
    #[unit("kg/m³", "kilograms per cubic metre", by = KILOGRAM_PER_CUBIC_METRE)]
    KilogramPerCubicMetre,
    #[unit("g/m³", "grams per cubic metre", by = GRAM_PER_CUBIC_METRE)]
    GramPerCubicMetre,
}

/// The units of `CatalyticActivity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = CatalyticActivity<f64>)]
pub enum CatalyticActivityUnit {
    #[default]
    #[unit("kat", "katals", by = KATAL)]
    Katal,
    #[unit("particle/s", "particles per second", by = PARTICLE_PER_SECOND)]
    ParticlePerSecond,
    #[unit("mol/s", "moles per second", by = MOLE_PER_SECOND)]
    MolePerSecond,
}

/// The units of `CatalyticActivityConcentration`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = CatalyticActivityConcentration<f64>)]
pub enum CatalyticActivityConcentrationUnit {
    #[default]
    #[unit("kat/m³", "katals per cubic metre", by = KATAL_PER_CUBIC_METRE)]
    KatalPerCubicMetre,
    #[unit("kat/L", "katals per litre", by = KATAL_PER_LITRE)]
    KatalPerLitre,
    #[unit("mkat/L", "millikatals per litre", by = MILLIKATAL_PER_LITRE)]
    MillikatalPerLitre,
    #[unit("particle · s⁻¹ · m⁻³", "particles per second cubic metre", by = PARTICLE_PER_SECOND_CUBIC_METRE)]
    ParticlePerSecondCubicMetre,
}

/// The units of `Radioactivity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Radioactivity<f64>)]
pub enum RadioactivityUnit {
    #[default]
    #[unit("Bq", "becquerels", by = BECQUEREL)]
    Becquerel,
    #[unit("Ci", "curies", by = CURIE)]
    Curie,
    #[unit("dpm", "disintegrations per minute", by = DISINTEGRATIONS_PER_MINUTE)]
    DisintegrationsPerMinute,
}

/// The units of `SpecificRadioactivity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificRadioactivity<f64>)]
pub enum SpecificRadioactivityUnit {
    #[default]
    #[unit("Bq/kg", "becquerels per kilogram", by = BECQUEREL_PER_KILOGRAM)]
    BecquerelPerKilogram,
    #[unit("Ci/kg", "curie per kilogram", by = CURIE_PER_KILOGRAM)]
    CuriePerKilogram,
    #[unit("dpm/kg", "disintegrations per minute per kilogram", by = DISINTEGRATIONS_PER_MINUTE_PER_KILOGRAM)]
    DisintegrationsPerMinutePerKilogram,
}

/// The units of `InverseAmountOfSubstance`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = InverseAmountOfSubstance<f64>)]
pub enum InverseAmountOfSubstanceUnit {
    #[default]
    #[unit("mol⁻¹", "per mole", by = PER_MOLE)]
    PerMole,
    #[unit("N_A", "Avogadro constants", by = AVOGADRO_CONSTANT)]
    AvogadroConstant,
}
