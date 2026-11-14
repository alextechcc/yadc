//! The units of `chemistry`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

#[allow(unused_imports)]
use crate::chemistry::*;
#[allow(unused_imports)]
use crate::*;

/// The units of `AmountOfSubstance`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum AmountOfSubstanceUnit {
    #[default]
    #[value(MOLE.unit())]
    Mole,
    #[value(PARTICLE.unit())]
    Particle,
}

/// The units of `MolarHeatCapacity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum MolarHeatCapacityUnit {
    #[default]
    #[value(SIUnit::ONE.times(METRE.unit(), 2).times(KILOGRAM.unit(), 1).times(SECOND.unit(), -2).times(KELVIN.unit(), -1).times(MOLE.unit(), -1))]
    MetreSquaredKilogramPerSecondSquaredKelvinMole,
    #[value(MOLAR_GAS_CONSTANT.unit())]
    MolarGasConstant,
}

/// The units of `CatalyticActivity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum CatalyticActivityUnit {
    #[default]
    #[value(KATAL.unit())]
    Katal,
}

/// The units of `Radioactivity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum RadioactivityUnit {
    #[default]
    #[value(BECQUEREL.unit())]
    Becquerel,
    #[value(CURIE.unit())]
    Curie,
}

/// The units of `InverseAmountOfSubstance`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum InverseAmountOfSubstanceUnit {
    #[default]
    #[value(SIUnit::ONE.times(MOLE.unit(), -1))]
    PerMole,
    #[value(AVOGADRO_CONSTANT.unit())]
    AvogadroConstant,
}
