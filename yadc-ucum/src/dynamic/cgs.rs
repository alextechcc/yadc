//! The units of `cgs`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::cgs::consts::*;
use crate::cgs::*;
use core::marker::Destruct;
use yadc::{Unit, num_traits::Num};

/// The units of `Acceleration`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Acceleration<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Acceleration<N>: [const] Unit<Acceleration<N>, N> + [const] Destruct)]
pub enum AccelerationUnit {
    #[default]
    #[unit("Gal", "Gal", by = Acceleration::<N>::from(GAL))]
    Gal,
}

/// The units of `Brightness`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Brightness<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Brightness<N>: [const] Unit<Brightness<N>, N> + [const] Destruct)]
pub enum BrightnessUnit {
    #[default]
    #[unit("Lmb", "Lambert", by = Brightness::<N>::from(LAMBERT))]
    Lambert,
}

/// The units of `DoseEquivalent`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = DoseEquivalent<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where DoseEquivalent<N>: [const] Unit<DoseEquivalent<N>, N> + [const] Destruct)]
pub enum DoseEquivalentUnit {
    #[default]
    #[unit("REM", "radiation equivalent man", by = DoseEquivalent::<N>::from(RADIATION_EQUIVALENT_MAN))]
    RadiationEquivalentMan,
}

/// The units of `DynamicViscosity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = DynamicViscosity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where DynamicViscosity<N>: [const] Unit<DynamicViscosity<N>, N> + [const] Destruct)]
pub enum DynamicViscosityUnit {
    #[default]
    #[unit("P", "Poise", by = DynamicViscosity::<N>::from(POISE))]
    Poise,
}

/// The units of `ElectricCurrent`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCurrent<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricCurrent<N>: [const] Unit<ElectricCurrent<N>, N> + [const] Destruct)]
pub enum ElectricCurrentUnit {
    #[default]
    #[unit("Bi", "Biot", by = ElectricCurrent::<N>::from(BIOT))]
    Biot,
}

/// The units of `Energy`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Energy<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Energy<N>: [const] Unit<Energy<N>, N> + [const] Destruct)]
pub enum EnergyUnit {
    #[default]
    #[unit("erg", "erg", by = Energy::<N>::from(ERG))]
    Erg,
}

/// The units of `EnergyDose`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = EnergyDose<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where EnergyDose<N>: [const] Unit<EnergyDose<N>, N> + [const] Destruct)]
pub enum EnergyDoseUnit {
    #[default]
    #[unit("RAD", "radiation absorbed dose", by = EnergyDose::<N>::from(RADIATION_ABSORBED_DOSE))]
    RadiationAbsorbedDose,
}

/// The units of `FluxOfMagneticInduction`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = FluxOfMagneticInduction<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where FluxOfMagneticInduction<N>: [const] Unit<FluxOfMagneticInduction<N>, N> + [const] Destruct)]
pub enum FluxOfMagneticInductionUnit {
    #[default]
    #[unit("Mx", "Maxwell", by = FluxOfMagneticInduction::<N>::from(MAXWELL))]
    Maxwell,
}

/// The units of `Force`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Force<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Force<N>: [const] Unit<Force<N>, N> + [const] Destruct)]
pub enum ForceUnit {
    #[default]
    #[unit("dyn", "dyne", by = Force::<N>::from(DYNE))]
    Dyne,
}

/// The units of `Illuminance`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Illuminance<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Illuminance<N>: [const] Unit<Illuminance<N>, N> + [const] Destruct)]
pub enum IlluminanceUnit {
    #[default]
    #[unit("ph", "phot", by = Illuminance::<N>::from(PHOT))]
    Phot,
}

/// The units of `IonDose`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = IonDose<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where IonDose<N>: [const] Unit<IonDose<N>, N> + [const] Destruct)]
pub enum IonDoseUnit {
    #[default]
    #[unit("R", "Roentgen", by = IonDose::<N>::from(ROENTGEN))]
    Roentgen,
}

/// The units of `KinematicViscosity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = KinematicViscosity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where KinematicViscosity<N>: [const] Unit<KinematicViscosity<N>, N> + [const] Destruct)]
pub enum KinematicViscosityUnit {
    #[default]
    #[unit("St", "Stokes", by = KinematicViscosity::<N>::from(STOKES))]
    Stokes,
}

/// The units of `LineicNumber`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LineicNumber<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where LineicNumber<N>: [const] Unit<LineicNumber<N>, N> + [const] Destruct)]
pub enum LineicNumberUnit {
    #[default]
    #[unit("Ky", "Kayser", by = LineicNumber::<N>::from(KAYSER))]
    Kayser,
}

/// The units of `LumIntensityDensity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LumIntensityDensity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where LumIntensityDensity<N>: [const] Unit<LumIntensityDensity<N>, N> + [const] Destruct)]
pub enum LumIntensityDensityUnit {
    #[default]
    #[unit("sb", "stilb", by = LumIntensityDensity::<N>::from(STILB))]
    Stilb,
}

/// The units of `MagneticFieldIntensity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticFieldIntensity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MagneticFieldIntensity<N>: [const] Unit<MagneticFieldIntensity<N>, N> + [const] Destruct)]
pub enum MagneticFieldIntensityUnit {
    #[default]
    #[unit("Oe", "Oersted", by = MagneticFieldIntensity::<N>::from(OERSTED))]
    Oersted,
}

/// The units of `MagneticFluxDensity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticFluxDensity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MagneticFluxDensity<N>: [const] Unit<MagneticFluxDensity<N>, N> + [const] Destruct)]
pub enum MagneticFluxDensityUnit {
    #[default]
    #[unit("G", "Gauss", by = MagneticFluxDensity::<N>::from(GAUSS))]
    Gauss,
}

/// The units of `MagneticTension`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticTension<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MagneticTension<N>: [const] Unit<MagneticTension<N>, N> + [const] Destruct)]
pub enum MagneticTensionUnit {
    #[default]
    #[unit("Gb", "Gilbert", by = MagneticTension::<N>::from(GILBERT))]
    Gilbert,
}

/// The units of `Radioactivity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Radioactivity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Radioactivity<N>: [const] Unit<Radioactivity<N>, N> + [const] Destruct)]
pub enum RadioactivityUnit {
    #[default]
    #[unit("Ci", "Curie", by = Radioactivity::<N>::from(CURIE))]
    Curie,
}
