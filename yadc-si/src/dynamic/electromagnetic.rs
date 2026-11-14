//! The units of `electromagnetic`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::dynamic::DynamicSIUnit;
use crate::electromagnetic::units::*;
use crate::electromagnetic::*;
use core::marker::Destruct;
use core::ops::{Add, Div, Mul, Sub};

/// The units of `ElectricCharge`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCharge<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricChargeUnit {
    #[default]
    #[unit(Coulomb)]
    Coulomb,
    #[unit(AmpereHour)]
    AmpereHour,
    #[unit(ElementaryCharge)]
    ElementaryCharge,
    #[unit(AtomicUnitOfCharge)]
    AtomicUnitOfCharge,
    #[unit(Abcoulomb)]
    Abcoulomb,
    #[unit(Faraday)]
    Faraday,
    #[unit(Franklin)]
    Franklin,
    #[unit(Statcoulomb)]
    Statcoulomb,
}

impl From<ElectricChargeUnit> for DynamicSIUnit {
    fn from(unit: ElectricChargeUnit) -> Self {
        match unit {
            ElectricChargeUnit::Coulomb => Coulomb {}.into(),
            ElectricChargeUnit::AmpereHour => AmpereHour {}.into(),
            ElectricChargeUnit::ElementaryCharge => ElementaryCharge {}.into(),
            ElectricChargeUnit::AtomicUnitOfCharge => AtomicUnitOfCharge {}.into(),
            ElectricChargeUnit::Abcoulomb => Abcoulomb {}.into(),
            ElectricChargeUnit::Faraday => Faraday {}.into(),
            ElectricChargeUnit::Franklin => Franklin {}.into(),
            ElectricChargeUnit::Statcoulomb => Statcoulomb {}.into(),
        }
    }
}

/// The units of `ElectricChargeArealDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricChargeArealDensity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricChargeArealDensityUnit {
    #[default]
    #[unit(ElectricChargeArealDensityCoulombPerSquareMetre)]
    CoulombPerSquareMetre,
}

impl From<ElectricChargeArealDensityUnit> for DynamicSIUnit {
    fn from(unit: ElectricChargeArealDensityUnit) -> Self {
        match unit {
            ElectricChargeArealDensityUnit::CoulombPerSquareMetre => {
                ElectricChargeArealDensityCoulombPerSquareMetre {}.into()
            }
        }
    }
}

/// The units of `ElectricChargeLinearDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricChargeLinearDensity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricChargeLinearDensityUnit {
    #[default]
    #[unit(CoulombPerMetre)]
    CoulombPerMetre,
}

impl From<ElectricChargeLinearDensityUnit> for DynamicSIUnit {
    fn from(unit: ElectricChargeLinearDensityUnit) -> Self {
        match unit {
            ElectricChargeLinearDensityUnit::CoulombPerMetre => CoulombPerMetre {}.into(),
        }
    }
}

/// The units of `ElectricChargeVolumetricDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricChargeVolumetricDensity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricChargeVolumetricDensityUnit {
    #[default]
    #[unit(CoulombPerCubicMetre)]
    CoulombPerCubicMetre,
}

impl From<ElectricChargeVolumetricDensityUnit> for DynamicSIUnit {
    fn from(unit: ElectricChargeVolumetricDensityUnit) -> Self {
        match unit {
            ElectricChargeVolumetricDensityUnit::CoulombPerCubicMetre => {
                CoulombPerCubicMetre {}.into()
            }
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
    #[unit(ElementaryChargePerSecond)]
    ElementaryChargePerSecond,
    #[unit(AtomicUnitOfChargePerSecond)]
    AtomicUnitOfChargePerSecond,
    #[unit(Abampere)]
    Abampere,
    #[unit(Gilbert)]
    Gilbert,
    #[unit(Statampere)]
    Statampere,
}

impl From<ElectricCurrentUnit> for DynamicSIUnit {
    fn from(unit: ElectricCurrentUnit) -> Self {
        match unit {
            ElectricCurrentUnit::Ampere => Ampere {}.into(),
            ElectricCurrentUnit::ElementaryChargePerSecond => ElementaryChargePerSecond {}.into(),
            ElectricCurrentUnit::AtomicUnitOfChargePerSecond => {
                AtomicUnitOfChargePerSecond {}.into()
            }
            ElectricCurrentUnit::Abampere => Abampere {}.into(),
            ElectricCurrentUnit::Gilbert => Gilbert {}.into(),
            ElectricCurrentUnit::Statampere => Statampere {}.into(),
        }
    }
}

/// The units of `ElectricCurrentDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCurrentDensity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricCurrentDensityUnit {
    #[default]
    #[unit(AmperePerSquareMetre)]
    AmperePerSquareMetre,
}

impl From<ElectricCurrentDensityUnit> for DynamicSIUnit {
    fn from(unit: ElectricCurrentDensityUnit) -> Self {
        match unit {
            ElectricCurrentDensityUnit::AmperePerSquareMetre => AmperePerSquareMetre {}.into(),
        }
    }
}

/// The units of `SurfaceElectricCurrentDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SurfaceElectricCurrentDensity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum SurfaceElectricCurrentDensityUnit {
    #[default]
    #[unit(SurfaceElectricCurrentDensityAmperePerMetre)]
    AmperePerMetre,
}

impl From<SurfaceElectricCurrentDensityUnit> for DynamicSIUnit {
    fn from(unit: SurfaceElectricCurrentDensityUnit) -> Self {
        match unit {
            SurfaceElectricCurrentDensityUnit::AmperePerMetre => {
                SurfaceElectricCurrentDensityAmperePerMetre {}.into()
            }
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
    #[unit(Abvolt)]
    Abvolt,
    #[unit(Statvolt)]
    Statvolt,
}

impl From<ElectricPotentialUnit> for DynamicSIUnit {
    fn from(unit: ElectricPotentialUnit) -> Self {
        match unit {
            ElectricPotentialUnit::Volt => Volt {}.into(),
            ElectricPotentialUnit::Abvolt => Abvolt {}.into(),
            ElectricPotentialUnit::Statvolt => Statvolt {}.into(),
        }
    }
}

/// The units of `ElectricField`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricField<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricFieldUnit {
    #[default]
    #[unit(VoltPerMetre)]
    VoltPerMetre,
    #[unit(VoltPerMil)]
    VoltPerMil,
    #[unit(AtomicUnitOfElectricField)]
    AtomicUnitOfElectricField,
}

impl From<ElectricFieldUnit> for DynamicSIUnit {
    fn from(unit: ElectricFieldUnit) -> Self {
        match unit {
            ElectricFieldUnit::VoltPerMetre => VoltPerMetre {}.into(),
            ElectricFieldUnit::VoltPerMil => VoltPerMil {}.into(),
            ElectricFieldUnit::AtomicUnitOfElectricField => AtomicUnitOfElectricField {}.into(),
        }
    }
}

/// The units of `ElectricDisplacementField`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricDisplacementField<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricDisplacementFieldUnit {
    #[default]
    #[unit(ElectricDisplacementFieldCoulombPerSquareMetre)]
    CoulombPerSquareMetre,
}

impl From<ElectricDisplacementFieldUnit> for DynamicSIUnit {
    fn from(unit: ElectricDisplacementFieldUnit) -> Self {
        match unit {
            ElectricDisplacementFieldUnit::CoulombPerSquareMetre => {
                ElectricDisplacementFieldCoulombPerSquareMetre {}.into()
            }
        }
    }
}

/// The units of `ElectricFlux`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricFlux<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricFluxUnit {
    #[default]
    #[unit(VoltMetre)]
    VoltMetre,
}

impl From<ElectricFluxUnit> for DynamicSIUnit {
    fn from(unit: ElectricFluxUnit) -> Self {
        match unit {
            ElectricFluxUnit::VoltMetre => VoltMetre {}.into(),
        }
    }
}

/// The units of `ElectricDipoleMoment`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricDipoleMoment<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricDipoleMomentUnit {
    #[default]
    #[unit(CoulombMetre)]
    CoulombMetre,
    #[unit(AtomicUnitOfElectricDipoleMoment)]
    AtomicUnitOfElectricDipoleMoment,
    #[unit(AtomicUnitOfChargeCentimetre)]
    AtomicUnitOfChargeCentimetre,
    #[unit(ElementaryChargeCentimetre)]
    ElementaryChargeCentimetre,
}

impl From<ElectricDipoleMomentUnit> for DynamicSIUnit {
    fn from(unit: ElectricDipoleMomentUnit) -> Self {
        match unit {
            ElectricDipoleMomentUnit::CoulombMetre => CoulombMetre {}.into(),
            ElectricDipoleMomentUnit::AtomicUnitOfElectricDipoleMoment => {
                AtomicUnitOfElectricDipoleMoment {}.into()
            }
            ElectricDipoleMomentUnit::AtomicUnitOfChargeCentimetre => {
                AtomicUnitOfChargeCentimetre {}.into()
            }
            ElectricDipoleMomentUnit::ElementaryChargeCentimetre => {
                ElementaryChargeCentimetre {}.into()
            }
        }
    }
}

/// The units of `ElectricQuadrupoleMoment`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricQuadrupoleMoment<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricQuadrupoleMomentUnit {
    #[default]
    #[unit(CoulombSquareMetre)]
    CoulombSquareMetre,
    #[unit(ElementaryChargeBarn)]
    ElementaryChargeBarn,
    #[unit(AtomicUnitOfElectricQuadrupoleMoment)]
    AtomicUnitOfElectricQuadrupoleMoment,
}

impl From<ElectricQuadrupoleMomentUnit> for DynamicSIUnit {
    fn from(unit: ElectricQuadrupoleMomentUnit) -> Self {
        match unit {
            ElectricQuadrupoleMomentUnit::CoulombSquareMetre => CoulombSquareMetre {}.into(),
            ElectricQuadrupoleMomentUnit::ElementaryChargeBarn => ElementaryChargeBarn {}.into(),
            ElectricQuadrupoleMomentUnit::AtomicUnitOfElectricQuadrupoleMoment => {
                AtomicUnitOfElectricQuadrupoleMoment {}.into()
            }
        }
    }
}

/// The units of `ElectricPermittivity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricPermittivity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricPermittivityUnit {
    #[default]
    #[unit(FaradPerMetre)]
    FaradPerMetre,
    #[unit(VacuumPermittivity)]
    VacuumPermittivity,
}

impl From<ElectricPermittivityUnit> for DynamicSIUnit {
    fn from(unit: ElectricPermittivityUnit) -> Self {
        match unit {
            ElectricPermittivityUnit::FaradPerMetre => FaradPerMetre {}.into(),
            ElectricPermittivityUnit::VacuumPermittivity => VacuumPermittivity {}.into(),
        }
    }
}

/// The units of `ElectricalConductance`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricalConductance<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricalConductanceUnit {
    #[default]
    #[unit(Siemens)]
    Siemens,
    #[unit(Mho)]
    Mho,
    #[unit(Abmho)]
    Abmho,
    #[unit(Absiemens)]
    Absiemens,
    #[unit(Statsiemens)]
    Statsiemens,
    #[unit(Statmho)]
    Statmho,
}

impl From<ElectricalConductanceUnit> for DynamicSIUnit {
    fn from(unit: ElectricalConductanceUnit) -> Self {
        match unit {
            ElectricalConductanceUnit::Siemens => Siemens {}.into(),
            ElectricalConductanceUnit::Mho => Mho {}.into(),
            ElectricalConductanceUnit::Abmho => Abmho {}.into(),
            ElectricalConductanceUnit::Absiemens => Absiemens {}.into(),
            ElectricalConductanceUnit::Statsiemens => Statsiemens {}.into(),
            ElectricalConductanceUnit::Statmho => Statmho {}.into(),
        }
    }
}

/// The units of `ElectricalConductivity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricalConductivity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricalConductivityUnit {
    #[default]
    #[unit(SiemensPerMetre)]
    SiemensPerMetre,
}

impl From<ElectricalConductivityUnit> for DynamicSIUnit {
    fn from(unit: ElectricalConductivityUnit) -> Self {
        match unit {
            ElectricalConductivityUnit::SiemensPerMetre => SiemensPerMetre {}.into(),
        }
    }
}

/// The units of `ElectricalMobility`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricalMobility<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricalMobilityUnit {
    #[default]
    #[unit(SquareMetrePerVoltSecond)]
    SquareMetrePerVoltSecond,
}

impl From<ElectricalMobilityUnit> for DynamicSIUnit {
    fn from(unit: ElectricalMobilityUnit) -> Self {
        match unit {
            ElectricalMobilityUnit::SquareMetrePerVoltSecond => SquareMetrePerVoltSecond {}.into(),
        }
    }
}

/// The units of `ElectricalResistance`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricalResistance<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricalResistanceUnit {
    #[default]
    #[unit(Ohm)]
    Ohm,
    #[unit(Abohm)]
    Abohm,
    #[unit(Statohm)]
    Statohm,
}

impl From<ElectricalResistanceUnit> for DynamicSIUnit {
    fn from(unit: ElectricalResistanceUnit) -> Self {
        match unit {
            ElectricalResistanceUnit::Ohm => Ohm {}.into(),
            ElectricalResistanceUnit::Abohm => Abohm {}.into(),
            ElectricalResistanceUnit::Statohm => Statohm {}.into(),
        }
    }
}

/// The units of `ElectricalResistivity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricalResistivity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricalResistivityUnit {
    #[default]
    #[unit(OhmMetre)]
    OhmMetre,
    #[unit(AbohmMetre)]
    AbohmMetre,
    #[unit(StatohmMetre)]
    StatohmMetre,
    #[unit(OhmInch)]
    OhmInch,
    #[unit(OhmFoot)]
    OhmFoot,
    #[unit(OhmYard)]
    OhmYard,
    #[unit(OhmSquareMillimetrePerMetre)]
    OhmSquareMillimetrePerMetre,
}

impl From<ElectricalResistivityUnit> for DynamicSIUnit {
    fn from(unit: ElectricalResistivityUnit) -> Self {
        match unit {
            ElectricalResistivityUnit::OhmMetre => OhmMetre {}.into(),
            ElectricalResistivityUnit::AbohmMetre => AbohmMetre {}.into(),
            ElectricalResistivityUnit::StatohmMetre => StatohmMetre {}.into(),
            ElectricalResistivityUnit::OhmInch => OhmInch {}.into(),
            ElectricalResistivityUnit::OhmFoot => OhmFoot {}.into(),
            ElectricalResistivityUnit::OhmYard => OhmYard {}.into(),
            ElectricalResistivityUnit::OhmSquareMillimetrePerMetre => {
                OhmSquareMillimetrePerMetre {}.into()
            }
        }
    }
}

/// The units of `Capacitance`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Capacitance<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum CapacitanceUnit {
    #[default]
    #[unit(Farad)]
    Farad,
    #[unit(Abfarad)]
    Abfarad,
    #[unit(Statfarad)]
    Statfarad,
}

impl From<CapacitanceUnit> for DynamicSIUnit {
    fn from(unit: CapacitanceUnit) -> Self {
        match unit {
            CapacitanceUnit::Farad => Farad {}.into(),
            CapacitanceUnit::Abfarad => Abfarad {}.into(),
            CapacitanceUnit::Statfarad => Statfarad {}.into(),
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
    #[unit(Abhenry)]
    Abhenry,
    #[unit(Stathenry)]
    Stathenry,
}

impl From<InductanceUnit> for DynamicSIUnit {
    fn from(unit: InductanceUnit) -> Self {
        match unit {
            InductanceUnit::Henry => Henry {}.into(),
            InductanceUnit::Abhenry => Abhenry {}.into(),
            InductanceUnit::Stathenry => Stathenry {}.into(),
        }
    }
}

/// The units of `MagneticFieldStrength`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticFieldStrength<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MagneticFieldStrengthUnit {
    #[default]
    #[unit(MagneticFieldStrengthAmperePerMetre)]
    AmperePerMetre,
    #[unit(Oersted)]
    Oersted,
}

impl From<MagneticFieldStrengthUnit> for DynamicSIUnit {
    fn from(unit: MagneticFieldStrengthUnit) -> Self {
        match unit {
            MagneticFieldStrengthUnit::AmperePerMetre => {
                MagneticFieldStrengthAmperePerMetre {}.into()
            }
            MagneticFieldStrengthUnit::Oersted => Oersted {}.into(),
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
    #[unit(Maxwell)]
    Maxwell,
}

impl From<MagneticFluxUnit> for DynamicSIUnit {
    fn from(unit: MagneticFluxUnit) -> Self {
        match unit {
            MagneticFluxUnit::Weber => Weber {}.into(),
            MagneticFluxUnit::Maxwell => Maxwell {}.into(),
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
    #[unit(Gamma)]
    Gamma,
    #[unit(Gauss)]
    Gauss,
}

impl From<MagneticFluxDensityUnit> for DynamicSIUnit {
    fn from(unit: MagneticFluxDensityUnit) -> Self {
        match unit {
            MagneticFluxDensityUnit::Tesla => Tesla {}.into(),
            MagneticFluxDensityUnit::Gamma => Gamma {}.into(),
            MagneticFluxDensityUnit::Gauss => Gauss {}.into(),
        }
    }
}

/// The units of `MagneticMoment`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticMoment<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MagneticMomentUnit {
    #[default]
    #[unit(AmpereSquareMetre)]
    AmpereSquareMetre,
    #[unit(JoulePerTesla)]
    JoulePerTesla,
    #[unit(NewtonMetrePerTesla)]
    NewtonMetrePerTesla,
    #[unit(ErgPerGauss)]
    ErgPerGauss,
    #[unit(BohrMagneton)]
    BohrMagneton,
    #[unit(NuclearMagneton)]
    NuclearMagneton,
    #[unit(AtomicUnitOfMagneticDipoleMoment)]
    AtomicUnitOfMagneticDipoleMoment,
    #[unit(DeuteronMagneticMoment)]
    DeuteronMagneticMoment,
    #[unit(ProtonMagneticMoment)]
    ProtonMagneticMoment,
    #[unit(ShieldedProtonMagneticMoment)]
    ShieldedProtonMagneticMoment,
    #[unit(TritonMagneticMoment)]
    TritonMagneticMoment,
}

impl From<MagneticMomentUnit> for DynamicSIUnit {
    fn from(unit: MagneticMomentUnit) -> Self {
        match unit {
            MagneticMomentUnit::AmpereSquareMetre => AmpereSquareMetre {}.into(),
            MagneticMomentUnit::JoulePerTesla => JoulePerTesla {}.into(),
            MagneticMomentUnit::NewtonMetrePerTesla => NewtonMetrePerTesla {}.into(),
            MagneticMomentUnit::ErgPerGauss => ErgPerGauss {}.into(),
            MagneticMomentUnit::BohrMagneton => BohrMagneton {}.into(),
            MagneticMomentUnit::NuclearMagneton => NuclearMagneton {}.into(),
            MagneticMomentUnit::AtomicUnitOfMagneticDipoleMoment => {
                AtomicUnitOfMagneticDipoleMoment {}.into()
            }
            MagneticMomentUnit::DeuteronMagneticMoment => DeuteronMagneticMoment {}.into(),
            MagneticMomentUnit::ProtonMagneticMoment => ProtonMagneticMoment {}.into(),
            MagneticMomentUnit::ShieldedProtonMagneticMoment => {
                ShieldedProtonMagneticMoment {}.into()
            }
            MagneticMomentUnit::TritonMagneticMoment => TritonMagneticMoment {}.into(),
        }
    }
}

/// The units of `MagneticPermeability`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticPermeability<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MagneticPermeabilityUnit {
    #[default]
    #[unit(HenryPerMetre)]
    HenryPerMetre,
    #[unit(VacuumPermeability)]
    VacuumPermeability,
}

impl From<MagneticPermeabilityUnit> for DynamicSIUnit {
    fn from(unit: MagneticPermeabilityUnit) -> Self {
        match unit {
            MagneticPermeabilityUnit::HenryPerMetre => HenryPerMetre {}.into(),
            MagneticPermeabilityUnit::VacuumPermeability => VacuumPermeability {}.into(),
        }
    }
}

/// The units of `GyromagneticRatio`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = GyromagneticRatio<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum GyromagneticRatioUnit {
    #[default]
    #[unit(HertzPerTesla)]
    HertzPerTesla,
    #[unit(RadianPerSecondTesla)]
    RadianPerSecondTesla,
    #[unit(ProtonGyromagneticRatio)]
    ProtonGyromagneticRatio,
    #[unit(NeutronGyromagneticRatio)]
    NeutronGyromagneticRatio,
    #[unit(ElectronGyromagneticRatio)]
    ElectronGyromagneticRatio,
    #[unit(ShieldedProtonGyromagneticRatio)]
    ShieldedProtonGyromagneticRatio,
    #[unit(ShieldedHelionGyromagneticRatio)]
    ShieldedHelionGyromagneticRatio,
    #[unit(DeuteronGyromagneticRatio)]
    DeuteronGyromagneticRatio,
    #[unit(TritonGyromagneticRatio)]
    TritonGyromagneticRatio,
    #[unit(Lithium7GyromagneticRatio)]
    Lithium7GyromagneticRatio,
    #[unit(Boron11GyromagneticRatio)]
    Boron11GyromagneticRatio,
    #[unit(Carbon13GyromagneticRatio)]
    Carbon13GyromagneticRatio,
    #[unit(Nitrogen14GyromagneticRatio)]
    Nitrogen14GyromagneticRatio,
    #[unit(Flourine19GyromagneticRatio)]
    Flourine19GyromagneticRatio,
    #[unit(Sodium23GyromagneticRatio)]
    Sodium23GyromagneticRatio,
    #[unit(Phosphorus31GyromagneticRatio)]
    Phosphorus31GyromagneticRatio,
}

impl From<GyromagneticRatioUnit> for DynamicSIUnit {
    fn from(unit: GyromagneticRatioUnit) -> Self {
        match unit {
            GyromagneticRatioUnit::HertzPerTesla => HertzPerTesla {}.into(),
            GyromagneticRatioUnit::RadianPerSecondTesla => RadianPerSecondTesla {}.into(),
            GyromagneticRatioUnit::ProtonGyromagneticRatio => ProtonGyromagneticRatio {}.into(),
            GyromagneticRatioUnit::NeutronGyromagneticRatio => NeutronGyromagneticRatio {}.into(),
            GyromagneticRatioUnit::ElectronGyromagneticRatio => ElectronGyromagneticRatio {}.into(),
            GyromagneticRatioUnit::ShieldedProtonGyromagneticRatio => {
                ShieldedProtonGyromagneticRatio {}.into()
            }
            GyromagneticRatioUnit::ShieldedHelionGyromagneticRatio => {
                ShieldedHelionGyromagneticRatio {}.into()
            }
            GyromagneticRatioUnit::DeuteronGyromagneticRatio => DeuteronGyromagneticRatio {}.into(),
            GyromagneticRatioUnit::TritonGyromagneticRatio => TritonGyromagneticRatio {}.into(),
            GyromagneticRatioUnit::Lithium7GyromagneticRatio => Lithium7GyromagneticRatio {}.into(),
            GyromagneticRatioUnit::Boron11GyromagneticRatio => Boron11GyromagneticRatio {}.into(),
            GyromagneticRatioUnit::Carbon13GyromagneticRatio => Carbon13GyromagneticRatio {}.into(),
            GyromagneticRatioUnit::Nitrogen14GyromagneticRatio => {
                Nitrogen14GyromagneticRatio {}.into()
            }
            GyromagneticRatioUnit::Flourine19GyromagneticRatio => {
                Flourine19GyromagneticRatio {}.into()
            }
            GyromagneticRatioUnit::Sodium23GyromagneticRatio => Sodium23GyromagneticRatio {}.into(),
            GyromagneticRatioUnit::Phosphorus31GyromagneticRatio => {
                Phosphorus31GyromagneticRatio {}.into()
            }
        }
    }
}

/// The units of `ReducedGyromagneticRatio`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ReducedGyromagneticRatio<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ReducedGyromagneticRatioUnit {
    #[default]
    #[unit(ProtonReducedGyromagneticRatio)]
    ProtonReducedGyromagneticRatio,
    #[unit(NeutronReducedGyromagneticRatio)]
    NeutronReducedGyromagneticRatio,
    #[unit(ElectronReducedGyromagneticRatio)]
    ElectronReducedGyromagneticRatio,
    #[unit(ShieldedProtonReducedGyromagneticRatio)]
    ShieldedProtonReducedGyromagneticRatio,
    #[unit(ShieldedHelionReducedGyromagneticRatio)]
    ShieldedHelionReducedGyromagneticRatio,
    #[unit(DeuteronReducedGyromagneticRatio)]
    DeuteronReducedGyromagneticRatio,
    #[unit(TritonReducedGyromagneticRatio)]
    TritonReducedGyromagneticRatio,
    #[unit(Lithium7ReducedGyromagneticRatio)]
    Lithium7ReducedGyromagneticRatio,
    #[unit(Boron11ReducedGyromagneticRatio)]
    Boron11ReducedGyromagneticRatio,
    #[unit(Carbon13ReducedGyromagneticRatio)]
    Carbon13ReducedGyromagneticRatio,
    #[unit(Nitrogen14ReducedGyromagneticRatio)]
    Nitrogen14ReducedGyromagneticRatio,
    #[unit(Flourine19ReducedGyromagneticRatio)]
    Flourine19ReducedGyromagneticRatio,
    #[unit(Sodium23ReducedGyromagneticRatio)]
    Sodium23ReducedGyromagneticRatio,
    #[unit(Phosphorus31ReducedGyromagneticRatio)]
    Phosphorus31ReducedGyromagneticRatio,
}

impl From<ReducedGyromagneticRatioUnit> for DynamicSIUnit {
    fn from(unit: ReducedGyromagneticRatioUnit) -> Self {
        match unit {
            ReducedGyromagneticRatioUnit::ProtonReducedGyromagneticRatio => {
                ProtonReducedGyromagneticRatio {}.into()
            }
            ReducedGyromagneticRatioUnit::NeutronReducedGyromagneticRatio => {
                NeutronReducedGyromagneticRatio {}.into()
            }
            ReducedGyromagneticRatioUnit::ElectronReducedGyromagneticRatio => {
                ElectronReducedGyromagneticRatio {}.into()
            }
            ReducedGyromagneticRatioUnit::ShieldedProtonReducedGyromagneticRatio => {
                ShieldedProtonReducedGyromagneticRatio {}.into()
            }
            ReducedGyromagneticRatioUnit::ShieldedHelionReducedGyromagneticRatio => {
                ShieldedHelionReducedGyromagneticRatio {}.into()
            }
            ReducedGyromagneticRatioUnit::DeuteronReducedGyromagneticRatio => {
                DeuteronReducedGyromagneticRatio {}.into()
            }
            ReducedGyromagneticRatioUnit::TritonReducedGyromagneticRatio => {
                TritonReducedGyromagneticRatio {}.into()
            }
            ReducedGyromagneticRatioUnit::Lithium7ReducedGyromagneticRatio => {
                Lithium7ReducedGyromagneticRatio {}.into()
            }
            ReducedGyromagneticRatioUnit::Boron11ReducedGyromagneticRatio => {
                Boron11ReducedGyromagneticRatio {}.into()
            }
            ReducedGyromagneticRatioUnit::Carbon13ReducedGyromagneticRatio => {
                Carbon13ReducedGyromagneticRatio {}.into()
            }
            ReducedGyromagneticRatioUnit::Nitrogen14ReducedGyromagneticRatio => {
                Nitrogen14ReducedGyromagneticRatio {}.into()
            }
            ReducedGyromagneticRatioUnit::Flourine19ReducedGyromagneticRatio => {
                Flourine19ReducedGyromagneticRatio {}.into()
            }
            ReducedGyromagneticRatioUnit::Sodium23ReducedGyromagneticRatio => {
                Sodium23ReducedGyromagneticRatio {}.into()
            }
            ReducedGyromagneticRatioUnit::Phosphorus31ReducedGyromagneticRatio => {
                Phosphorus31ReducedGyromagneticRatio {}.into()
            }
        }
    }
}
