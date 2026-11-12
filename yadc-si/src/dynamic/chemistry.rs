//! The units of `chemistry`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::base::{KELVIN, KILOGRAM, METRE, ONE, SECOND};
use crate::chemistry::consts::*;
use crate::chemistry::*;
use core::marker::Destruct;
use yadc::{Unit, num_traits::Num};

/// The units of `AmountOfSubstance`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AmountOfSubstance<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where AmountOfSubstance<N>: [const] Unit<AmountOfSubstance<N>, N> + [const] Destruct)]
pub enum AmountOfSubstanceUnit {
    #[default]
    #[unit("mol", "moles", by = AmountOfSubstance::<N>::from(MOLE))]
    Mole,
    #[unit("particle", "particles", by = AmountOfSubstance::<N>::from(PARTICLE))]
    Particle,
}

/// The units of `MolarConcentration`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarConcentration<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MolarConcentration<N>: [const] Unit<MolarConcentration<N>, N> + [const] Destruct)]
pub enum MolarConcentrationUnit {
    #[default]
    #[unit("mol/m³", "moles per cubic metre", by = MolarConcentration::<N>::from(MOLE / (METRE * METRE * METRE)))]
    MolePerCubicMetre,
    #[unit("mol/L", "moles per litre", by = MolarConcentration::<N>::from((MOLE / (METRE * METRE * METRE)) * Exact::int(1000)))]
    MolePerLitre,
    #[unit("particle/m³", "particles per cubic metre", by = MolarConcentration::<N>::from((MOLE / (METRE * METRE * METRE)) * (Exact::int(1) / Exact::int(602214076000000000000000))))]
    ParticlePerCubicMetre,
}

/// The units of `MolarEnergy`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarEnergy<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MolarEnergy<N>: [const] Unit<MolarEnergy<N>, N> + [const] Destruct)]
pub enum MolarEnergyUnit {
    #[default]
    #[unit("kg · m²/(s² · mol)", "kilograms square metre per second squared mole", by = MolarEnergy::<N>::from(METRE * METRE * KILOGRAM / (SECOND * SECOND * MOLE)))]
    KilogramSquareMetrePerSecondSquaredMole,
    #[unit("J/mol", "joules per mole", by = MolarEnergy::<N>::from(METRE * METRE * KILOGRAM / (SECOND * SECOND * MOLE)))]
    JoulePerMole,
    #[unit("W · h/mol", "watt hours per mole", by = MolarEnergy::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * MOLE)) * Exact::int(3600)))]
    WattHourPerMole,
    #[unit("Btu (IT)/mol", "British thermal units (IT) per mole", by = MolarEnergy::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * MOLE)) * Exact::dec(105505585262, -8)))]
    BtuItPerMole,
    #[unit("Btu/mol", "British thermal units per mole", by = MolarEnergy::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * MOLE)) * (Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1))))]
    BtuPerMole,
    #[unit("cal (IT)/mol", "calories (IT) per mole", by = MolarEnergy::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * MOLE)) * Exact::dec(41868, -4)))]
    CalorieItPerMole,
    #[unit("cal/mol", "calories per mole", by = MolarEnergy::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * MOLE)) * Exact::dec(4184, -3)))]
    CaloriePerMole,
    #[unit("Cal (IT)/mol", "Calories (IT) per mole", by = MolarEnergy::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * MOLE)) * Exact::dec(41868, -1)))]
    CalorieItNutritionPerMole,
    #[unit("Cal/mol", "Calories per mole", by = MolarEnergy::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * MOLE)) * Exact::int(4184)))]
    CalorieNutritionPerMole,
    #[unit("eV/mol", "electronvolts per mole", by = MolarEnergy::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * MOLE)) * Exact::dec(1602176634, -28)))]
    ElectronvoltPerMole,
    #[unit("erg/mol", "ergs per mole", by = MolarEnergy::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * MOLE)) * Exact::dec(1, -7)))]
    ErgPerMole,
    #[unit("ft · pdl/mol", "foot poundals per mole", by = MolarEnergy::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * MOLE)) * (Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(3048, -4))))]
    FootPoundalPerMole,
    #[unit("ft · lbf/mol", "foot pounds-force per mole", by = MolarEnergy::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * MOLE)) * (Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5))))]
    FootPoundForcePerMole,
    #[unit("10¹⁵ Btu (IT)/mol", "quads per mole", by = MolarEnergy::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * MOLE)) * Exact::int(1055055852620000000)))]
    QuadPerMole,
    #[unit("thm (EC)/mol", "therms (EC) per mole", by = MolarEnergy::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * MOLE)) * Exact::int(105506000)))]
    ThermEcPerMole,
    #[unit("thm/mol", "therms per mole", by = MolarEnergy::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * MOLE)) * Exact::int(105480400)))]
    ThermUsPerMole,
    #[unit("t of TNT/mol", "tons of TNT per mole", by = MolarEnergy::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * MOLE)) * Exact::int(4184000000)))]
    TonTntPerMole,
    #[unit("W · s/mol", "watt seconds per mole", by = MolarEnergy::<N>::from(METRE * METRE * KILOGRAM / (SECOND * SECOND * MOLE)))]
    WattSecondPerMole,
    #[unit("J/particle", "joules per particle", by = MolarEnergy::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * MOLE)) * Exact::int(602214076000000000000000)))]
    JoulePerParticle,
    #[unit("eV/particle", "electronvolts per particle", by = MolarEnergy::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * MOLE)) * (Exact::dec(1602176634, -28) * Exact::int(602214076000000000000000))))]
    ElectronvoltPerParticle,
}

/// The units of `MolarFlux`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarFlux<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MolarFlux<N>: [const] Unit<MolarFlux<N>, N> + [const] Destruct)]
pub enum MolarFluxUnit {
    #[default]
    #[unit("mol/(m² · s)", "moles per square metre second", by = MolarFlux::<N>::from(MOLE / (METRE * METRE * SECOND)))]
    MolePerSquareMetreSecond,
}

/// The units of `MolarHeatCapacity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarHeatCapacity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MolarHeatCapacity<N>: [const] Unit<MolarHeatCapacity<N>, N> + [const] Destruct)]
pub enum MolarHeatCapacityUnit {
    #[default]
    #[unit("J/(K · mol)", "joules per kelvin mole", by = MolarHeatCapacity::<N>::from(METRE * METRE * KILOGRAM / (SECOND * SECOND * KELVIN * MOLE)))]
    JoulePerKelvinMole,
    #[unit("Btu (IT)/(K · mol)", "British thermal units (IT) per kelvin mole", by = MolarHeatCapacity::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * KELVIN * MOLE)) * Exact::dec(105505585262, -8)))]
    BtuItPerKelvinMole,
    #[unit("Btu/(K · mol)", "British thermal units per kelvin mole", by = MolarHeatCapacity::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * KELVIN * MOLE)) * (Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1))))]
    BtuPerKelvinMole,
    #[unit("cal (IT)/(K · mol)", "calories (IT) per kelvin mole", by = MolarHeatCapacity::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * KELVIN * MOLE)) * Exact::dec(41868, -4)))]
    CalorieItPerKelvinMole,
    #[unit("cal/(K · mol)", "calories per kelvin mole", by = MolarHeatCapacity::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * KELVIN * MOLE)) * Exact::dec(4184, -3)))]
    CaloriePerKelvinMole,
    #[unit("Cal (IT)/(K · mol)", "Calories (IT) per kelvin mole", by = MolarHeatCapacity::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * KELVIN * MOLE)) * Exact::dec(41868, -1)))]
    CalorieItNutritionPerKelvinMole,
    #[unit("Cal/(K · mol)", "Calories per kelvin mole", by = MolarHeatCapacity::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * KELVIN * MOLE)) * Exact::int(4184)))]
    CalorieNutritionPerKelvinMole,
    #[unit("J/(K · particle)", "joules per kelvin particle", by = MolarHeatCapacity::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * KELVIN * MOLE)) * Exact::int(602214076000000000000000)))]
    JoulePerKelvinParticle,
    #[unit("eV/(K · particle)", "electronvolts per kelvin particle", by = MolarHeatCapacity::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * KELVIN * MOLE)) * (Exact::dec(1602176634, -28) * Exact::int(602214076000000000000000))))]
    ElectronvoltPerKelvinParticle,
    #[unit("R", "molar gas constants", by = MolarHeatCapacity::<N>::from(MOLAR_GAS_CONSTANT))]
    MolarGasConstant,
}

/// The units of `MolarMass`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarMass<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MolarMass<N>: [const] Unit<MolarMass<N>, N> + [const] Destruct)]
pub enum MolarMassUnit {
    #[default]
    #[unit("kg/mol", "kilograms per mole", by = MolarMass::<N>::from(KILOGRAM / MOLE))]
    KilogramPerMole,
    #[unit("g/mol", "grams per mole", by = MolarMass::<N>::from((KILOGRAM / MOLE) * Exact::dec(1, -3)))]
    GramPerMole,
}

/// The units of `MolarRadioactivity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarRadioactivity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MolarRadioactivity<N>: [const] Unit<MolarRadioactivity<N>, N> + [const] Destruct)]
pub enum MolarRadioactivityUnit {
    #[default]
    #[unit("Bq/mol", "becquerels per mole", by = MolarRadioactivity::<N>::from(ONE / (SECOND * MOLE)))]
    BecquerelPerMole,
    #[unit("Ci/mol", "curies per mole", by = MolarRadioactivity::<N>::from((ONE / (SECOND * MOLE)) * Exact::int(37000000000)))]
    CuriePerMole,
    #[unit("dpm/mol", "disintegrations per minute per mole", by = MolarRadioactivity::<N>::from((ONE / (SECOND * MOLE)) * (Exact::int(1) / Exact::int(60))))]
    DisintegrationsPerMinutePerMole,
}

/// The units of `MolarVolume`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarVolume<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MolarVolume<N>: [const] Unit<MolarVolume<N>, N> + [const] Destruct)]
pub enum MolarVolumeUnit {
    #[default]
    #[unit("m³/mol", "cubic metres per mole", by = MolarVolume::<N>::from(METRE * METRE * METRE / MOLE))]
    CubicMetrePerMole,
    #[unit("m³/particle", "cubic metres per particle", by = MolarVolume::<N>::from((METRE * METRE * METRE / MOLE) * Exact::int(602214076000000000000000)))]
    CubicMetrePerParticle,
}

/// The units of `Molality`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Molality<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Molality<N>: [const] Unit<Molality<N>, N> + [const] Destruct)]
pub enum MolalityUnit {
    #[default]
    #[unit("mol/kg", "moles per kilogram", by = Molality::<N>::from(MOLE / KILOGRAM))]
    MolePerKilogram,
}

/// The units of `MassConcentration`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassConcentration<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MassConcentration<N>: [const] Unit<MassConcentration<N>, N> + [const] Destruct)]
pub enum MassConcentrationUnit {
    #[default]
    #[unit("kg/m³", "kilograms per cubic metre", by = MassConcentration::<N>::from(KILOGRAM / (METRE * METRE * METRE)))]
    KilogramPerCubicMetre,
    #[unit("g/m³", "grams per cubic metre", by = MassConcentration::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * Exact::dec(1, -3)))]
    GramPerCubicMetre,
}

/// The units of `CatalyticActivity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = CatalyticActivity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where CatalyticActivity<N>: [const] Unit<CatalyticActivity<N>, N> + [const] Destruct)]
pub enum CatalyticActivityUnit {
    #[default]
    #[unit("kat", "katals", by = CatalyticActivity::<N>::from(KATAL))]
    Katal,
    #[unit("particle/s", "particles per second", by = CatalyticActivity::<N>::from(KATAL * (Exact::int(1) / Exact::int(602214076000000000000000))))]
    ParticlePerSecond,
    #[unit("mol/s", "moles per second", by = CatalyticActivity::<N>::from(KATAL))]
    MolePerSecond,
}

/// The units of `CatalyticActivityConcentration`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = CatalyticActivityConcentration<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where CatalyticActivityConcentration<N>: [const] Unit<CatalyticActivityConcentration<N>, N> + [const] Destruct)]
pub enum CatalyticActivityConcentrationUnit {
    #[default]
    #[unit("kat/m³", "katals per cubic metre", by = CatalyticActivityConcentration::<N>::from(MOLE / (METRE * METRE * METRE * SECOND)))]
    KatalPerCubicMetre,
    #[unit("kat/L", "katals per litre", by = CatalyticActivityConcentration::<N>::from((MOLE / (METRE * METRE * METRE * SECOND)) * Exact::int(1000)))]
    KatalPerLitre,
    #[unit("particle · s⁻¹ · m⁻³", "particles per second cubic metre", by = CatalyticActivityConcentration::<N>::from((MOLE / (METRE * METRE * METRE * SECOND)) * (Exact::int(1) / Exact::int(602214076000000000000000))))]
    ParticlePerSecondCubicMetre,
}

/// The units of `Radioactivity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Radioactivity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Radioactivity<N>: [const] Unit<Radioactivity<N>, N> + [const] Destruct)]
pub enum RadioactivityUnit {
    #[default]
    #[unit("Bq", "becquerels", by = Radioactivity::<N>::from(BECQUEREL))]
    Becquerel,
    #[unit("Ci", "curies", by = Radioactivity::<N>::from(CURIE))]
    Curie,
    #[unit("dpm", "disintegrations per minute", by = Radioactivity::<N>::from(DISINTEGRATIONS_PER_MINUTE))]
    DisintegrationsPerMinute,
}

/// The units of `SpecificRadioactivity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificRadioactivity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where SpecificRadioactivity<N>: [const] Unit<SpecificRadioactivity<N>, N> + [const] Destruct)]
pub enum SpecificRadioactivityUnit {
    #[default]
    #[unit("Bq/kg", "becquerels per kilogram", by = SpecificRadioactivity::<N>::from(ONE / (KILOGRAM * SECOND)))]
    BecquerelPerKilogram,
    #[unit("Ci/kg", "curie per kilogram", by = SpecificRadioactivity::<N>::from((ONE / (KILOGRAM * SECOND)) * Exact::int(37000000000)))]
    CuriePerKilogram,
    #[unit("dpm/kg", "disintegrations per minute per kilogram", by = SpecificRadioactivity::<N>::from((ONE / (KILOGRAM * SECOND)) * (Exact::int(1) / Exact::int(60))))]
    DisintegrationsPerMinutePerKilogram,
}

/// The units of `InverseAmountOfSubstance`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = InverseAmountOfSubstance<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where InverseAmountOfSubstance<N>: [const] Unit<InverseAmountOfSubstance<N>, N> + [const] Destruct)]
pub enum InverseAmountOfSubstanceUnit {
    #[default]
    #[unit("mol⁻¹", "per mole", by = InverseAmountOfSubstance::<N>::from(PER_MOLE))]
    PerMole,
    #[unit("N_A", "Avogadro constants", by = InverseAmountOfSubstance::<N>::from(AVOGADRO_CONSTANT))]
    AvogadroConstant,
}
