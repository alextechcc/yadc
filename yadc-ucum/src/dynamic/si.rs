//! The units of `si`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::dynamic::DynamicUcumUnit;
use crate::si::units::*;
use crate::si::*;
use core::marker::Destruct;
use core::ops::{Add, Div, Mul, Sub};

/// The units of `AmountOfSubstance`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AmountOfSubstance<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum AmountOfSubstanceUnit {
    #[default]
    #[unit(Mole)]
    Mole,
}

impl From<AmountOfSubstanceUnit> for DynamicUcumUnit {
    fn from(unit: AmountOfSubstanceUnit) -> Self {
        match unit {
            AmountOfSubstanceUnit::Mole => Mole {}.into(),
        }
    }
}

/// The units of `DoseEquivalent`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = DoseEquivalent<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum DoseEquivalentUnit {
    #[default]
    #[unit(Sievert)]
    Sievert,
}

impl From<DoseEquivalentUnit> for DynamicUcumUnit {
    fn from(unit: DoseEquivalentUnit) -> Self {
        match unit {
            DoseEquivalentUnit::Sievert => Sievert {}.into(),
        }
    }
}

/// The units of `ElectricCapacitance`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCapacitance<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricCapacitanceUnit {
    #[default]
    #[unit(Farad)]
    Farad,
}

impl From<ElectricCapacitanceUnit> for DynamicUcumUnit {
    fn from(unit: ElectricCapacitanceUnit) -> Self {
        match unit {
            ElectricCapacitanceUnit::Farad => Farad {}.into(),
        }
    }
}

/// The units of `ElectricCharge`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCharge<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricChargeUnit {
    #[default]
    #[unit(Coulomb)]
    Coulomb,
}

impl From<ElectricChargeUnit> for DynamicUcumUnit {
    fn from(unit: ElectricChargeUnit) -> Self {
        match unit {
            ElectricChargeUnit::Coulomb => Coulomb {}.into(),
        }
    }
}

/// The units of `ElectricConductance`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricConductance<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricConductanceUnit {
    #[default]
    #[unit(Siemens)]
    Siemens,
}

impl From<ElectricConductanceUnit> for DynamicUcumUnit {
    fn from(unit: ElectricConductanceUnit) -> Self {
        match unit {
            ElectricConductanceUnit::Siemens => Siemens {}.into(),
        }
    }
}

/// The units of `ElectricCurrent`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCurrent<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricCurrentUnit {
    #[default]
    #[unit(Ampere)]
    Ampere,
}

impl From<ElectricCurrentUnit> for DynamicUcumUnit {
    fn from(unit: ElectricCurrentUnit) -> Self {
        match unit {
            ElectricCurrentUnit::Ampere => Ampere {}.into(),
        }
    }
}

/// The units of `ElectricPotential`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricPotential<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricPotentialUnit {
    #[default]
    #[unit(Volt)]
    Volt,
}

impl From<ElectricPotentialUnit> for DynamicUcumUnit {
    fn from(unit: ElectricPotentialUnit) -> Self {
        match unit {
            ElectricPotentialUnit::Volt => Volt {}.into(),
        }
    }
}

/// The units of `ElectricResistance`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricResistance<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricResistanceUnit {
    #[default]
    #[unit(Ohm)]
    Ohm,
}

impl From<ElectricResistanceUnit> for DynamicUcumUnit {
    fn from(unit: ElectricResistanceUnit) -> Self {
        match unit {
            ElectricResistanceUnit::Ohm => Ohm {}.into(),
        }
    }
}

/// The units of `Energy`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Energy<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum EnergyUnit {
    #[default]
    #[unit(Joule)]
    Joule,
}

impl From<EnergyUnit> for DynamicUcumUnit {
    fn from(unit: EnergyUnit) -> Self {
        match unit {
            EnergyUnit::Joule => Joule {}.into(),
        }
    }
}

/// The units of `EnergyDose`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = EnergyDose<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum EnergyDoseUnit {
    #[default]
    #[unit(Gray)]
    Gray,
}

impl From<EnergyDoseUnit> for DynamicUcumUnit {
    fn from(unit: EnergyDoseUnit) -> Self {
        match unit {
            EnergyDoseUnit::Gray => Gray {}.into(),
        }
    }
}

/// The units of `Force`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Force<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ForceUnit {
    #[default]
    #[unit(Newton)]
    Newton,
}

impl From<ForceUnit> for DynamicUcumUnit {
    fn from(unit: ForceUnit) -> Self {
        match unit {
            ForceUnit::Newton => Newton {}.into(),
        }
    }
}

/// The units of `Frequency`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Frequency<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum FrequencyUnit {
    #[default]
    #[unit(Hertz)]
    Hertz,
}

impl From<FrequencyUnit> for DynamicUcumUnit {
    fn from(unit: FrequencyUnit) -> Self {
        match unit {
            FrequencyUnit::Hertz => Hertz {}.into(),
        }
    }
}

/// The units of `Illuminance`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Illuminance<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum IlluminanceUnit {
    #[default]
    #[unit(Lux)]
    Lux,
}

impl From<IlluminanceUnit> for DynamicUcumUnit {
    fn from(unit: IlluminanceUnit) -> Self {
        match unit {
            IlluminanceUnit::Lux => Lux {}.into(),
        }
    }
}

/// The units of `Inductance`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Inductance<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum InductanceUnit {
    #[default]
    #[unit(Henry)]
    Henry,
}

impl From<InductanceUnit> for DynamicUcumUnit {
    fn from(unit: InductanceUnit) -> Self {
        match unit {
            InductanceUnit::Henry => Henry {}.into(),
        }
    }
}

/// The units of `Length`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum LengthUnit {
    #[default]
    #[unit(Meter)]
    Meter,
}

impl From<LengthUnit> for DynamicUcumUnit {
    fn from(unit: LengthUnit) -> Self {
        match unit {
            LengthUnit::Meter => Meter {}.into(),
        }
    }
}

/// The units of `LuminousFlux`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LuminousFlux<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum LuminousFluxUnit {
    #[default]
    #[unit(Lumen)]
    Lumen,
}

impl From<LuminousFluxUnit> for DynamicUcumUnit {
    fn from(unit: LuminousFluxUnit) -> Self {
        match unit {
            LuminousFluxUnit::Lumen => Lumen {}.into(),
        }
    }
}

/// The units of `LuminousIntensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LuminousIntensity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum LuminousIntensityUnit {
    #[default]
    #[unit(Candela)]
    Candela,
}

impl From<LuminousIntensityUnit> for DynamicUcumUnit {
    fn from(unit: LuminousIntensityUnit) -> Self {
        match unit {
            LuminousIntensityUnit::Candela => Candela {}.into(),
        }
    }
}

/// The units of `MagneticFlux`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticFlux<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MagneticFluxUnit {
    #[default]
    #[unit(Weber)]
    Weber,
}

impl From<MagneticFluxUnit> for DynamicUcumUnit {
    fn from(unit: MagneticFluxUnit) -> Self {
        match unit {
            MagneticFluxUnit::Weber => Weber {}.into(),
        }
    }
}

/// The units of `MagneticFluxDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticFluxDensity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MagneticFluxDensityUnit {
    #[default]
    #[unit(Tesla)]
    Tesla,
}

impl From<MagneticFluxDensityUnit> for DynamicUcumUnit {
    fn from(unit: MagneticFluxDensityUnit) -> Self {
        match unit {
            MagneticFluxDensityUnit::Tesla => Tesla {}.into(),
        }
    }
}

/// The units of `Mass`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Mass<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MassUnit {
    #[default]
    #[unit(Gram)]
    Gram,
}

impl From<MassUnit> for DynamicUcumUnit {
    fn from(unit: MassUnit) -> Self {
        match unit {
            MassUnit::Gram => Gram {}.into(),
        }
    }
}

/// The units of `PlaneAngle`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = PlaneAngle<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum PlaneAngleUnit {
    #[default]
    #[unit(Radian)]
    Radian,
}

impl From<PlaneAngleUnit> for DynamicUcumUnit {
    fn from(unit: PlaneAngleUnit) -> Self {
        match unit {
            PlaneAngleUnit::Radian => Radian {}.into(),
        }
    }
}

/// The units of `Power`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Power<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum PowerUnit {
    #[default]
    #[unit(Watt)]
    Watt,
}

impl From<PowerUnit> for DynamicUcumUnit {
    fn from(unit: PowerUnit) -> Self {
        match unit {
            PowerUnit::Watt => Watt {}.into(),
        }
    }
}

/// The units of `Pressure`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Pressure<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum PressureUnit {
    #[default]
    #[unit(Pascal)]
    Pascal,
}

impl From<PressureUnit> for DynamicUcumUnit {
    fn from(unit: PressureUnit) -> Self {
        match unit {
            PressureUnit::Pascal => Pascal {}.into(),
        }
    }
}

/// The units of `Radioactivity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Radioactivity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum RadioactivityUnit {
    #[default]
    #[unit(Becquerel)]
    Becquerel,
}

impl From<RadioactivityUnit> for DynamicUcumUnit {
    fn from(unit: RadioactivityUnit) -> Self {
        match unit {
            RadioactivityUnit::Becquerel => Becquerel {}.into(),
        }
    }
}

/// The units of `SolidAngle`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SolidAngle<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum SolidAngleUnit {
    #[default]
    #[unit(Steradian)]
    Steradian,
}

impl From<SolidAngleUnit> for DynamicUcumUnit {
    fn from(unit: SolidAngleUnit) -> Self {
        match unit {
            SolidAngleUnit::Steradian => Steradian {}.into(),
        }
    }
}

/// The units of `Time`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Time<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum TimeUnit {
    #[default]
    #[unit(Second)]
    Second,
}

impl From<TimeUnit> for DynamicUcumUnit {
    fn from(unit: TimeUnit) -> Self {
        match unit {
            TimeUnit::Second => Second {}.into(),
        }
    }
}
