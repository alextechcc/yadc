use crate::base::{KELVIN, KILOGRAM, METRE, ONE, SECOND};
use crate::{Exact, ISQ, SIQuantity};
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
/// (re-exported), then one per named unit, each defined exactly from them
/// (`INCH` is `0.0254 * METRE`). A unit written as others is not one:
/// `SLUG / CUBIC_FOOT`, `KILO * METRE` are exact too. Converting rounds,
/// once, where it is written - in a `const`, by the compiler:
/// `const MPH: Velocity<f64> = (MILE / HOUR).into();`, then `MPH(60.0)`.
pub mod consts {
    use super::*;
    use crate::base::{KELVIN, KILOGRAM, METRE, ONE, SECOND};

    /// The mole, the SI base unit of amount of substance.
    pub use crate::base::MOLE;

    /// One `Particle` (`particle`), as an `AmountOfSubstance`.
    pub const PARTICLE: AmountOfSubstance<Exact> =
        MOLE * (Exact::int(1) / Exact::int(602214076000000000000000));

    /// One `MolarGasConstant` (`R`), as a `MolarHeatCapacity`.
    pub const MOLAR_GAS_CONSTANT: MolarHeatCapacity<Exact> =
        (METRE * METRE * KILOGRAM / (SECOND * SECOND * KELVIN * MOLE)) * Exact::dec(8314462618, -9);

    /// One `Katal` (`kat`), as a `CatalyticActivity`.
    pub const KATAL: CatalyticActivity<Exact> = MOLE / SECOND;

    /// One `Becquerel` (`Bq`), as a `Radioactivity`.
    pub const BECQUEREL: Radioactivity<Exact> = ONE / SECOND;

    /// One `Curie` (`Ci`), as a `Radioactivity`.
    pub const CURIE: Radioactivity<Exact> = BECQUEREL * Exact::int(37000000000);

    /// One `DisintegrationsPerMinute` (`dpm`), as a `Radioactivity`.
    pub const DISINTEGRATIONS_PER_MINUTE: Radioactivity<Exact> =
        BECQUEREL * (Exact::int(1) / Exact::int(60));

    /// One `PerMole` (`mol⁻¹`), as an `InverseAmountOfSubstance`.
    pub const PER_MOLE: InverseAmountOfSubstance<Exact> = ONE / MOLE;

    /// One `AvogadroConstant` (`N_A`), as an `InverseAmountOfSubstance`.
    pub const AVOGADRO_CONSTANT: InverseAmountOfSubstance<Exact> =
        PER_MOLE * Exact::int(602214076000000000000000);
}

/// The units of `AmountOfSubstance`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AmountOfSubstance<N>, number = N)]
#[unit(for<N> where AmountOfSubstance<N>: [const] yadc::Unit<AmountOfSubstance<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum AmountOfSubstanceUnit {
    #[default]
    #[unit("mol", "moles", by = AmountOfSubstance::<N>::from(MOLE))]
    Mole,
    #[unit("particle", "particles", by = AmountOfSubstance::<N>::from(PARTICLE))]
    Particle,
}

/// The units of `MolarConcentration`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarConcentration<N>, number = N)]
#[unit(for<N> where MolarConcentration<N>: [const] yadc::Unit<MolarConcentration<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MolarConcentrationUnit {
    #[default]
    #[unit("mol/m³", "moles per cubic metre", by = MolarConcentration::<N>::from(MOLE / (METRE * METRE * METRE)))]
    MolePerCubicMetre,
    #[unit("mol/L", "moles per litre", by = MolarConcentration::<N>::from((MOLE / (METRE * METRE * METRE)) * Exact::int(1000)))]
    MolePerLitre,
    #[unit("particle/m³", "particles per cubic metre", by = MolarConcentration::<N>::from((MOLE / (METRE * METRE * METRE)) * (Exact::int(1) / Exact::int(602214076000000000000000))))]
    ParticlePerCubicMetre,
}

/// The units of `MolarEnergy`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarEnergy<N>, number = N)]
#[unit(for<N> where MolarEnergy<N>: [const] yadc::Unit<MolarEnergy<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
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

/// The units of `MolarFlux`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarFlux<N>, number = N)]
#[unit(for<N> where MolarFlux<N>: [const] yadc::Unit<MolarFlux<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MolarFluxUnit {
    #[default]
    #[unit("mol/(m² · s)", "moles per square metre second", by = MolarFlux::<N>::from(MOLE / (METRE * METRE * SECOND)))]
    MolePerSquareMetreSecond,
}

/// The units of `MolarHeatCapacity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarHeatCapacity<N>, number = N)]
#[unit(for<N> where MolarHeatCapacity<N>: [const] yadc::Unit<MolarHeatCapacity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
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

/// The units of `MolarMass`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarMass<N>, number = N)]
#[unit(for<N> where MolarMass<N>: [const] yadc::Unit<MolarMass<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MolarMassUnit {
    #[default]
    #[unit("kg/mol", "kilograms per mole", by = MolarMass::<N>::from(KILOGRAM / MOLE))]
    KilogramPerMole,
    #[unit("g/mol", "grams per mole", by = MolarMass::<N>::from((KILOGRAM / MOLE) * Exact::dec(1, -3)))]
    GramPerMole,
}

/// The units of `MolarRadioactivity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarRadioactivity<N>, number = N)]
#[unit(for<N> where MolarRadioactivity<N>: [const] yadc::Unit<MolarRadioactivity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MolarRadioactivityUnit {
    #[default]
    #[unit("Bq/mol", "becquerels per mole", by = MolarRadioactivity::<N>::from(ONE / (SECOND * MOLE)))]
    BecquerelPerMole,
    #[unit("Ci/mol", "curies per mole", by = MolarRadioactivity::<N>::from((ONE / (SECOND * MOLE)) * Exact::int(37000000000)))]
    CuriePerMole,
    #[unit("dpm/mol", "disintegrations per minute per mole", by = MolarRadioactivity::<N>::from((ONE / (SECOND * MOLE)) * (Exact::int(1) / Exact::int(60))))]
    DisintegrationsPerMinutePerMole,
}

/// The units of `MolarVolume`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarVolume<N>, number = N)]
#[unit(for<N> where MolarVolume<N>: [const] yadc::Unit<MolarVolume<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MolarVolumeUnit {
    #[default]
    #[unit("m³/mol", "cubic metres per mole", by = MolarVolume::<N>::from(METRE * METRE * METRE / MOLE))]
    CubicMetrePerMole,
    #[unit("m³/particle", "cubic metres per particle", by = MolarVolume::<N>::from((METRE * METRE * METRE / MOLE) * Exact::int(602214076000000000000000)))]
    CubicMetrePerParticle,
}

/// The units of `Molality`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Molality<N>, number = N)]
#[unit(for<N> where Molality<N>: [const] yadc::Unit<Molality<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MolalityUnit {
    #[default]
    #[unit("mol/kg", "moles per kilogram", by = Molality::<N>::from(MOLE / KILOGRAM))]
    MolePerKilogram,
}

/// The units of `MassConcentration`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassConcentration<N>, number = N)]
#[unit(for<N> where MassConcentration<N>: [const] yadc::Unit<MassConcentration<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MassConcentrationUnit {
    #[default]
    #[unit("kg/m³", "kilograms per cubic metre", by = MassConcentration::<N>::from(KILOGRAM / (METRE * METRE * METRE)))]
    KilogramPerCubicMetre,
    #[unit("g/m³", "grams per cubic metre", by = MassConcentration::<N>::from((KILOGRAM / (METRE * METRE * METRE)) * Exact::dec(1, -3)))]
    GramPerCubicMetre,
}

/// The units of `CatalyticActivity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = CatalyticActivity<N>, number = N)]
#[unit(for<N> where CatalyticActivity<N>: [const] yadc::Unit<CatalyticActivity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum CatalyticActivityUnit {
    #[default]
    #[unit("kat", "katals", by = CatalyticActivity::<N>::from(KATAL))]
    Katal,
    #[unit("particle/s", "particles per second", by = CatalyticActivity::<N>::from(KATAL * (Exact::int(1) / Exact::int(602214076000000000000000))))]
    ParticlePerSecond,
    #[unit("mol/s", "moles per second", by = CatalyticActivity::<N>::from(KATAL))]
    MolePerSecond,
}

/// The units of `CatalyticActivityConcentration`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = CatalyticActivityConcentration<N>, number = N)]
#[unit(for<N> where CatalyticActivityConcentration<N>: [const] yadc::Unit<CatalyticActivityConcentration<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum CatalyticActivityConcentrationUnit {
    #[default]
    #[unit("kat/m³", "katals per cubic metre", by = CatalyticActivityConcentration::<N>::from(MOLE / (METRE * METRE * METRE * SECOND)))]
    KatalPerCubicMetre,
    #[unit("kat/L", "katals per litre", by = CatalyticActivityConcentration::<N>::from((MOLE / (METRE * METRE * METRE * SECOND)) * Exact::int(1000)))]
    KatalPerLitre,
    #[unit("particle · s⁻¹ · m⁻³", "particles per second cubic metre", by = CatalyticActivityConcentration::<N>::from((MOLE / (METRE * METRE * METRE * SECOND)) * (Exact::int(1) / Exact::int(602214076000000000000000))))]
    ParticlePerSecondCubicMetre,
}

/// The units of `Radioactivity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Radioactivity<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Radioactivity<N>: [const] yadc::Unit<Radioactivity<N>, N> + [const] core::marker::Destruct)]
pub enum RadioactivityUnit {
    #[default]
    #[unit("Bq", "becquerels", by = Radioactivity::<N>::from(BECQUEREL))]
    Becquerel,
    #[unit("Ci", "curies", by = Radioactivity::<N>::from(CURIE))]
    Curie,
    #[unit("dpm", "disintegrations per minute", by = Radioactivity::<N>::from(DISINTEGRATIONS_PER_MINUTE))]
    DisintegrationsPerMinute,
}

/// The units of `SpecificRadioactivity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificRadioactivity<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, SpecificRadioactivity<N>: [const] yadc::Unit<SpecificRadioactivity<N>, N> + [const] core::marker::Destruct)]
pub enum SpecificRadioactivityUnit {
    #[default]
    #[unit("Bq/kg", "becquerels per kilogram", by = SpecificRadioactivity::<N>::from(ONE / (KILOGRAM * SECOND)))]
    BecquerelPerKilogram,
    #[unit("Ci/kg", "curie per kilogram", by = SpecificRadioactivity::<N>::from((ONE / (KILOGRAM * SECOND)) * Exact::int(37000000000)))]
    CuriePerKilogram,
    #[unit("dpm/kg", "disintegrations per minute per kilogram", by = SpecificRadioactivity::<N>::from((ONE / (KILOGRAM * SECOND)) * (Exact::int(1) / Exact::int(60))))]
    DisintegrationsPerMinutePerKilogram,
}

/// The units of `InverseAmountOfSubstance`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = InverseAmountOfSubstance<N>, number = N)]
#[unit(for<N> where InverseAmountOfSubstance<N>: [const] yadc::Unit<InverseAmountOfSubstance<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum InverseAmountOfSubstanceUnit {
    #[default]
    #[unit("mol⁻¹", "per mole", by = InverseAmountOfSubstance::<N>::from(PER_MOLE))]
    PerMole,
    #[unit("N_A", "Avogadro constants", by = InverseAmountOfSubstance::<N>::from(AVOGADRO_CONSTANT))]
    AvogadroConstant,
}
