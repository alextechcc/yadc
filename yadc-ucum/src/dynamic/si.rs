//! The units of `si`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::si::consts::*;
use crate::si::*;
use core::marker::Destruct;
use yadc::{Unit, num_traits::Num};

/// The units of `AmountOfSubstance`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AmountOfSubstance<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where AmountOfSubstance<N>: [const] Unit<AmountOfSubstance<N>, N> + [const] Destruct)]
pub enum AmountOfSubstanceUnit {
    #[default]
    #[unit("mol", "mole", by = AmountOfSubstance::<N>::from(MOLE))]
    Mole,
}

/// The units of `DoseEquivalent`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = DoseEquivalent<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where DoseEquivalent<N>: [const] Unit<DoseEquivalent<N>, N> + [const] Destruct)]
pub enum DoseEquivalentUnit {
    #[default]
    #[unit("Sv", "sievert", by = DoseEquivalent::<N>::from(SIEVERT))]
    Sievert,
}

/// The units of `ElectricCapacitance`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCapacitance<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricCapacitance<N>: [const] Unit<ElectricCapacitance<N>, N> + [const] Destruct)]
pub enum ElectricCapacitanceUnit {
    #[default]
    #[unit("F", "farad", by = ElectricCapacitance::<N>::from(FARAD))]
    Farad,
}

/// The units of `ElectricCharge`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCharge<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricCharge<N>: [const] Unit<ElectricCharge<N>, N> + [const] Destruct)]
pub enum ElectricChargeUnit {
    #[default]
    #[unit("C", "coulomb", by = ElectricCharge::<N>::from(COULOMB))]
    Coulomb,
}

/// The units of `ElectricConductance`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricConductance<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricConductance<N>: [const] Unit<ElectricConductance<N>, N> + [const] Destruct)]
pub enum ElectricConductanceUnit {
    #[default]
    #[unit("S", "siemens", by = ElectricConductance::<N>::from(SIEMENS))]
    Siemens,
}

/// The units of `ElectricCurrent`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCurrent<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricCurrent<N>: [const] Unit<ElectricCurrent<N>, N> + [const] Destruct)]
pub enum ElectricCurrentUnit {
    #[default]
    #[unit("A", "ampère", by = ElectricCurrent::<N>::from(AMPERE))]
    Ampere,
}

/// The units of `ElectricPotential`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricPotential<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricPotential<N>: [const] Unit<ElectricPotential<N>, N> + [const] Destruct)]
pub enum ElectricPotentialUnit {
    #[default]
    #[unit("V", "volt", by = ElectricPotential::<N>::from(VOLT))]
    Volt,
}

/// The units of `ElectricResistance`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricResistance<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricResistance<N>: [const] Unit<ElectricResistance<N>, N> + [const] Destruct)]
pub enum ElectricResistanceUnit {
    #[default]
    #[unit("Ohm", "ohm", by = ElectricResistance::<N>::from(OHM))]
    Ohm,
}

/// The units of `Energy`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Energy<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Energy<N>: [const] Unit<Energy<N>, N> + [const] Destruct)]
pub enum EnergyUnit {
    #[default]
    #[unit("J", "joule", by = Energy::<N>::from(JOULE))]
    Joule,
}

/// The units of `EnergyDose`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = EnergyDose<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where EnergyDose<N>: [const] Unit<EnergyDose<N>, N> + [const] Destruct)]
pub enum EnergyDoseUnit {
    #[default]
    #[unit("Gy", "gray", by = EnergyDose::<N>::from(GRAY))]
    Gray,
}

/// The units of `Force`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Force<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Force<N>: [const] Unit<Force<N>, N> + [const] Destruct)]
pub enum ForceUnit {
    #[default]
    #[unit("N", "newton", by = Force::<N>::from(NEWTON))]
    Newton,
}

/// The units of `Frequency`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Frequency<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Frequency<N>: [const] Unit<Frequency<N>, N> + [const] Destruct)]
pub enum FrequencyUnit {
    #[default]
    #[unit("Hz", "hertz", by = Frequency::<N>::from(HERTZ))]
    Hertz,
}

/// The units of `Illuminance`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Illuminance<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Illuminance<N>: [const] Unit<Illuminance<N>, N> + [const] Destruct)]
pub enum IlluminanceUnit {
    #[default]
    #[unit("lx", "lux", by = Illuminance::<N>::from(LUX))]
    Lux,
}

/// The units of `Inductance`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Inductance<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Inductance<N>: [const] Unit<Inductance<N>, N> + [const] Destruct)]
pub enum InductanceUnit {
    #[default]
    #[unit("H", "henry", by = Inductance::<N>::from(HENRY))]
    Henry,
}

/// The units of `Length`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Length<N>: [const] Unit<Length<N>, N> + [const] Destruct)]
pub enum LengthUnit {
    #[default]
    #[unit("m", "meter", by = Length::<N>::from(METER))]
    Meter,
}

/// The units of `LuminousFlux`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LuminousFlux<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where LuminousFlux<N>: [const] Unit<LuminousFlux<N>, N> + [const] Destruct)]
pub enum LuminousFluxUnit {
    #[default]
    #[unit("lm", "lumen", by = LuminousFlux::<N>::from(LUMEN))]
    Lumen,
}

/// The units of `LuminousIntensity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LuminousIntensity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where LuminousIntensity<N>: [const] Unit<LuminousIntensity<N>, N> + [const] Destruct)]
pub enum LuminousIntensityUnit {
    #[default]
    #[unit("cd", "candela", by = LuminousIntensity::<N>::from(CANDELA))]
    Candela,
}

/// The units of `MagneticFlux`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticFlux<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MagneticFlux<N>: [const] Unit<MagneticFlux<N>, N> + [const] Destruct)]
pub enum MagneticFluxUnit {
    #[default]
    #[unit("Wb", "weber", by = MagneticFlux::<N>::from(WEBER))]
    Weber,
}

/// The units of `MagneticFluxDensity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticFluxDensity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MagneticFluxDensity<N>: [const] Unit<MagneticFluxDensity<N>, N> + [const] Destruct)]
pub enum MagneticFluxDensityUnit {
    #[default]
    #[unit("T", "tesla", by = MagneticFluxDensity::<N>::from(TESLA))]
    Tesla,
}

/// The units of `Mass`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Mass<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Mass<N>: [const] Unit<Mass<N>, N> + [const] Destruct)]
pub enum MassUnit {
    #[default]
    #[unit("g", "gram", by = Mass::<N>::from(GRAM))]
    Gram,
}

/// The units of `PlaneAngle`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = PlaneAngle<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where PlaneAngle<N>: [const] Unit<PlaneAngle<N>, N> + [const] Destruct)]
pub enum PlaneAngleUnit {
    #[default]
    #[unit("rad", "radian", by = PlaneAngle::<N>::from(RADIAN))]
    Radian,
}

/// The units of `Power`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Power<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Power<N>: [const] Unit<Power<N>, N> + [const] Destruct)]
pub enum PowerUnit {
    #[default]
    #[unit("W", "watt", by = Power::<N>::from(WATT))]
    Watt,
}

/// The units of `Pressure`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Pressure<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Pressure<N>: [const] Unit<Pressure<N>, N> + [const] Destruct)]
pub enum PressureUnit {
    #[default]
    #[unit("Pa", "pascal", by = Pressure::<N>::from(PASCAL))]
    Pascal,
}

/// The units of `Radioactivity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Radioactivity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Radioactivity<N>: [const] Unit<Radioactivity<N>, N> + [const] Destruct)]
pub enum RadioactivityUnit {
    #[default]
    #[unit("Bq", "becquerel", by = Radioactivity::<N>::from(BECQUEREL))]
    Becquerel,
}

/// The units of `SolidAngle`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SolidAngle<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where SolidAngle<N>: [const] Unit<SolidAngle<N>, N> + [const] Destruct)]
pub enum SolidAngleUnit {
    #[default]
    #[unit("sr", "steradian", by = SolidAngle::<N>::from(STERADIAN))]
    Steradian,
}

/// The units of `Time`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Time<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Time<N>: [const] Unit<Time<N>, N> + [const] Destruct)]
pub enum TimeUnit {
    #[default]
    #[unit("s", "second", by = Time::<N>::from(SECOND))]
    Second,
}
