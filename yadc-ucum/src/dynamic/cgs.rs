//! The units of `cgs`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::cgs::units::*;
use crate::cgs::*;
use crate::dynamic::DynamicUcumUnit;
use core::marker::Destruct;
use core::ops::{Add, Div, Mul, Sub};

/// The units of `Acceleration`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Acceleration<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum AccelerationUnit {
    #[default]
    #[unit(Gal)]
    Gal,
}

impl From<AccelerationUnit> for DynamicUcumUnit {
    fn from(unit: AccelerationUnit) -> Self {
        match unit {
            AccelerationUnit::Gal => Gal {}.into(),
        }
    }
}

/// The units of `Brightness`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Brightness<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum BrightnessUnit {
    #[default]
    #[unit(Lambert)]
    Lambert,
}

impl From<BrightnessUnit> for DynamicUcumUnit {
    fn from(unit: BrightnessUnit) -> Self {
        match unit {
            BrightnessUnit::Lambert => Lambert {}.into(),
        }
    }
}

/// The units of `DoseEquivalent`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = DoseEquivalent<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum DoseEquivalentUnit {
    #[default]
    #[unit(RadiationEquivalentMan)]
    RadiationEquivalentMan,
}

impl From<DoseEquivalentUnit> for DynamicUcumUnit {
    fn from(unit: DoseEquivalentUnit) -> Self {
        match unit {
            DoseEquivalentUnit::RadiationEquivalentMan => RadiationEquivalentMan {}.into(),
        }
    }
}

/// The units of `DynamicViscosity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = DynamicViscosity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum DynamicViscosityUnit {
    #[default]
    #[unit(Poise)]
    Poise,
}

impl From<DynamicViscosityUnit> for DynamicUcumUnit {
    fn from(unit: DynamicViscosityUnit) -> Self {
        match unit {
            DynamicViscosityUnit::Poise => Poise {}.into(),
        }
    }
}

/// The units of `ElectricCurrent`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCurrent<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricCurrentUnit {
    #[default]
    #[unit(Biot)]
    Biot,
}

impl From<ElectricCurrentUnit> for DynamicUcumUnit {
    fn from(unit: ElectricCurrentUnit) -> Self {
        match unit {
            ElectricCurrentUnit::Biot => Biot {}.into(),
        }
    }
}

/// The units of `Energy`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Energy<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum EnergyUnit {
    #[default]
    #[unit(Erg)]
    Erg,
}

impl From<EnergyUnit> for DynamicUcumUnit {
    fn from(unit: EnergyUnit) -> Self {
        match unit {
            EnergyUnit::Erg => Erg {}.into(),
        }
    }
}

/// The units of `EnergyDose`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = EnergyDose<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum EnergyDoseUnit {
    #[default]
    #[unit(RadiationAbsorbedDose)]
    RadiationAbsorbedDose,
}

impl From<EnergyDoseUnit> for DynamicUcumUnit {
    fn from(unit: EnergyDoseUnit) -> Self {
        match unit {
            EnergyDoseUnit::RadiationAbsorbedDose => RadiationAbsorbedDose {}.into(),
        }
    }
}

/// The units of `FluxOfMagneticInduction`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = FluxOfMagneticInduction<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum FluxOfMagneticInductionUnit {
    #[default]
    #[unit(Maxwell)]
    Maxwell,
}

impl From<FluxOfMagneticInductionUnit> for DynamicUcumUnit {
    fn from(unit: FluxOfMagneticInductionUnit) -> Self {
        match unit {
            FluxOfMagneticInductionUnit::Maxwell => Maxwell {}.into(),
        }
    }
}

/// The units of `Force`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Force<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ForceUnit {
    #[default]
    #[unit(Dyne)]
    Dyne,
}

impl From<ForceUnit> for DynamicUcumUnit {
    fn from(unit: ForceUnit) -> Self {
        match unit {
            ForceUnit::Dyne => Dyne {}.into(),
        }
    }
}

/// The units of `Illuminance`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Illuminance<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum IlluminanceUnit {
    #[default]
    #[unit(Phot)]
    Phot,
}

impl From<IlluminanceUnit> for DynamicUcumUnit {
    fn from(unit: IlluminanceUnit) -> Self {
        match unit {
            IlluminanceUnit::Phot => Phot {}.into(),
        }
    }
}

/// The units of `IonDose`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = IonDose<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum IonDoseUnit {
    #[default]
    #[unit(Roentgen)]
    Roentgen,
}

impl From<IonDoseUnit> for DynamicUcumUnit {
    fn from(unit: IonDoseUnit) -> Self {
        match unit {
            IonDoseUnit::Roentgen => Roentgen {}.into(),
        }
    }
}

/// The units of `KinematicViscosity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = KinematicViscosity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum KinematicViscosityUnit {
    #[default]
    #[unit(Stokes)]
    Stokes,
}

impl From<KinematicViscosityUnit> for DynamicUcumUnit {
    fn from(unit: KinematicViscosityUnit) -> Self {
        match unit {
            KinematicViscosityUnit::Stokes => Stokes {}.into(),
        }
    }
}

/// The units of `LineicNumber`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LineicNumber<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum LineicNumberUnit {
    #[default]
    #[unit(Kayser)]
    Kayser,
}

impl From<LineicNumberUnit> for DynamicUcumUnit {
    fn from(unit: LineicNumberUnit) -> Self {
        match unit {
            LineicNumberUnit::Kayser => Kayser {}.into(),
        }
    }
}

/// The units of `LumIntensityDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LumIntensityDensity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum LumIntensityDensityUnit {
    #[default]
    #[unit(Stilb)]
    Stilb,
}

impl From<LumIntensityDensityUnit> for DynamicUcumUnit {
    fn from(unit: LumIntensityDensityUnit) -> Self {
        match unit {
            LumIntensityDensityUnit::Stilb => Stilb {}.into(),
        }
    }
}

/// The units of `MagneticFieldIntensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticFieldIntensity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MagneticFieldIntensityUnit {
    #[default]
    #[unit(Oersted)]
    Oersted,
}

impl From<MagneticFieldIntensityUnit> for DynamicUcumUnit {
    fn from(unit: MagneticFieldIntensityUnit) -> Self {
        match unit {
            MagneticFieldIntensityUnit::Oersted => Oersted {}.into(),
        }
    }
}

/// The units of `MagneticFluxDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticFluxDensity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MagneticFluxDensityUnit {
    #[default]
    #[unit(Gauss)]
    Gauss,
}

impl From<MagneticFluxDensityUnit> for DynamicUcumUnit {
    fn from(unit: MagneticFluxDensityUnit) -> Self {
        match unit {
            MagneticFluxDensityUnit::Gauss => Gauss {}.into(),
        }
    }
}

/// The units of `MagneticTension`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticTension<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MagneticTensionUnit {
    #[default]
    #[unit(Gilbert)]
    Gilbert,
}

impl From<MagneticTensionUnit> for DynamicUcumUnit {
    fn from(unit: MagneticTensionUnit) -> Self {
        match unit {
            MagneticTensionUnit::Gilbert => Gilbert {}.into(),
        }
    }
}

/// The units of `Radioactivity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Radioactivity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum RadioactivityUnit {
    #[default]
    #[unit(Curie)]
    Curie,
}

impl From<RadioactivityUnit> for DynamicUcumUnit {
    fn from(unit: RadioactivityUnit) -> Self {
        match unit {
            RadioactivityUnit::Curie => Curie {}.into(),
        }
    }
}
