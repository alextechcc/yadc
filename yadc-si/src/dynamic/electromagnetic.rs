//! The units of `electromagnetic`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::dynamic::DynamicSIUnit;
use crate::electromagnetic::units::*;
use crate::electromagnetic::*;
use crate::{Exact, SIQuantity, Unit};
use core::ops::{Add, Div, Mul, Sub};

/// The units of `ElectricCharge`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ELECTRIC_CHARGE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ELECTRIC_CHARGE, N>;
        fn value(&self, quantity: SIQuantity<ELECTRIC_CHARGE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ElectricChargeUnit {
    #[default]
    #[value(COULOMB)]
    Coulomb,
    #[value(AMPERE_HOUR)]
    AmpereHour,
    #[value(ELEMENTARY_CHARGE)]
    ElementaryCharge,
    #[value(ATOMIC_UNIT_OF_CHARGE)]
    AtomicUnitOfCharge,
    #[value(ABCOULOMB)]
    Abcoulomb,
    #[value(FARADAY)]
    Faraday,
    #[value(FRANKLIN)]
    Franklin,
    #[value(STATCOULOMB)]
    Statcoulomb,
}

/// The units of `ElectricChargeArealDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ELECTRIC_CHARGE_AREAL_DENSITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ELECTRIC_CHARGE_AREAL_DENSITY, N>;
        fn value(&self, quantity: SIQuantity<ELECTRIC_CHARGE_AREAL_DENSITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ElectricChargeArealDensityUnit {
    #[default]
    #[value(ELECTRIC_CHARGE_AREAL_DENSITY_COULOMB_PER_SQUARE_METRE)]
    CoulombPerSquareMetre,
}

/// The units of `ElectricChargeLinearDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ELECTRIC_CHARGE_LINEAR_DENSITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ELECTRIC_CHARGE_LINEAR_DENSITY, N>;
        fn value(&self, quantity: SIQuantity<ELECTRIC_CHARGE_LINEAR_DENSITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ElectricChargeLinearDensityUnit {
    #[default]
    #[value(COULOMB_PER_METRE)]
    CoulombPerMetre,
}

/// The units of `ElectricChargeVolumetricDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ELECTRIC_CHARGE_VOLUMETRIC_DENSITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ELECTRIC_CHARGE_VOLUMETRIC_DENSITY, N>;
        fn value(&self, quantity: SIQuantity<ELECTRIC_CHARGE_VOLUMETRIC_DENSITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ElectricChargeVolumetricDensityUnit {
    #[default]
    #[value(COULOMB_PER_CUBIC_METRE)]
    CoulombPerCubicMetre,
}

/// The units of `ElectricCurrent`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ELECTRIC_CURRENT, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ELECTRIC_CURRENT, N>;
        fn value(&self, quantity: SIQuantity<ELECTRIC_CURRENT, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ElectricCurrentUnit {
    #[default]
    #[value(AMPERE)]
    Ampere,
    #[value(ELEMENTARY_CHARGE_PER_SECOND)]
    ElementaryChargePerSecond,
    #[value(ATOMIC_UNIT_OF_CHARGE_PER_SECOND)]
    AtomicUnitOfChargePerSecond,
    #[value(ABAMPERE)]
    Abampere,
    #[value(GILBERT)]
    Gilbert,
    #[value(STATAMPERE)]
    Statampere,
}

/// The units of `ElectricCurrentDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ELECTRIC_CURRENT_DENSITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ELECTRIC_CURRENT_DENSITY, N>;
        fn value(&self, quantity: SIQuantity<ELECTRIC_CURRENT_DENSITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ElectricCurrentDensityUnit {
    #[default]
    #[value(AMPERE_PER_SQUARE_METRE)]
    AmperePerSquareMetre,
}

/// The units of `SurfaceElectricCurrentDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<SURFACE_ELECTRIC_CURRENT_DENSITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<SURFACE_ELECTRIC_CURRENT_DENSITY, N>;
        fn value(&self, quantity: SIQuantity<SURFACE_ELECTRIC_CURRENT_DENSITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum SurfaceElectricCurrentDensityUnit {
    #[default]
    #[value(SURFACE_ELECTRIC_CURRENT_DENSITY_AMPERE_PER_METRE)]
    AmperePerMetre,
}

/// The units of `ElectricPotential`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ELECTRIC_POTENTIAL, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ELECTRIC_POTENTIAL, N>;
        fn value(&self, quantity: SIQuantity<ELECTRIC_POTENTIAL, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ElectricPotentialUnit {
    #[default]
    #[value(VOLT)]
    Volt,
    #[value(ABVOLT)]
    Abvolt,
    #[value(STATVOLT)]
    Statvolt,
}

/// The units of `ElectricField`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ELECTRIC_FIELD, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ELECTRIC_FIELD, N>;
        fn value(&self, quantity: SIQuantity<ELECTRIC_FIELD, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ElectricFieldUnit {
    #[default]
    #[value(VOLT_PER_METRE)]
    VoltPerMetre,
    #[value(VOLT_PER_MIL)]
    VoltPerMil,
    #[value(ATOMIC_UNIT_OF_ELECTRIC_FIELD)]
    AtomicUnitOfElectricField,
}

/// The units of `ElectricDisplacementField`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ELECTRIC_DISPLACEMENT_FIELD, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ELECTRIC_DISPLACEMENT_FIELD, N>;
        fn value(&self, quantity: SIQuantity<ELECTRIC_DISPLACEMENT_FIELD, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ElectricDisplacementFieldUnit {
    #[default]
    #[value(ELECTRIC_DISPLACEMENT_FIELD_COULOMB_PER_SQUARE_METRE)]
    CoulombPerSquareMetre,
}

/// The units of `ElectricFlux`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ELECTRIC_FLUX, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ELECTRIC_FLUX, N>;
        fn value(&self, quantity: SIQuantity<ELECTRIC_FLUX, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ElectricFluxUnit {
    #[default]
    #[value(VOLT_METRE)]
    VoltMetre,
}

/// The units of `ElectricDipoleMoment`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ELECTRIC_DIPOLE_MOMENT, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ELECTRIC_DIPOLE_MOMENT, N>;
        fn value(&self, quantity: SIQuantity<ELECTRIC_DIPOLE_MOMENT, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ElectricDipoleMomentUnit {
    #[default]
    #[value(COULOMB_METRE)]
    CoulombMetre,
    #[value(ATOMIC_UNIT_OF_ELECTRIC_DIPOLE_MOMENT)]
    AtomicUnitOfElectricDipoleMoment,
    #[value(ATOMIC_UNIT_OF_CHARGE_CENTIMETRE)]
    AtomicUnitOfChargeCentimetre,
    #[value(ELEMENTARY_CHARGE_CENTIMETRE)]
    ElementaryChargeCentimetre,
}

/// The units of `ElectricQuadrupoleMoment`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ELECTRIC_QUADRUPOLE_MOMENT, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ELECTRIC_QUADRUPOLE_MOMENT, N>;
        fn value(&self, quantity: SIQuantity<ELECTRIC_QUADRUPOLE_MOMENT, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ElectricQuadrupoleMomentUnit {
    #[default]
    #[value(COULOMB_SQUARE_METRE)]
    CoulombSquareMetre,
    #[value(ELEMENTARY_CHARGE_BARN)]
    ElementaryChargeBarn,
    #[value(ATOMIC_UNIT_OF_ELECTRIC_QUADRUPOLE_MOMENT)]
    AtomicUnitOfElectricQuadrupoleMoment,
}

/// The units of `ElectricPermittivity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ELECTRIC_PERMITTIVITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ELECTRIC_PERMITTIVITY, N>;
        fn value(&self, quantity: SIQuantity<ELECTRIC_PERMITTIVITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ElectricPermittivityUnit {
    #[default]
    #[value(FARAD_PER_METRE)]
    FaradPerMetre,
    #[value(VACUUM_PERMITTIVITY)]
    VacuumPermittivity,
}

/// The units of `ElectricalConductance`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ELECTRICAL_CONDUCTANCE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ELECTRICAL_CONDUCTANCE, N>;
        fn value(&self, quantity: SIQuantity<ELECTRICAL_CONDUCTANCE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ElectricalConductanceUnit {
    #[default]
    #[value(SIEMENS)]
    Siemens,
    #[value(MHO)]
    Mho,
    #[value(ABMHO)]
    Abmho,
    #[value(ABSIEMENS)]
    Absiemens,
    #[value(STATSIEMENS)]
    Statsiemens,
    #[value(STATMHO)]
    Statmho,
}

/// The units of `ElectricalConductivity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ELECTRICAL_CONDUCTIVITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ELECTRICAL_CONDUCTIVITY, N>;
        fn value(&self, quantity: SIQuantity<ELECTRICAL_CONDUCTIVITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ElectricalConductivityUnit {
    #[default]
    #[value(SIEMENS_PER_METRE)]
    SiemensPerMetre,
}

/// The units of `ElectricalMobility`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ELECTRICAL_MOBILITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ELECTRICAL_MOBILITY, N>;
        fn value(&self, quantity: SIQuantity<ELECTRICAL_MOBILITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ElectricalMobilityUnit {
    #[default]
    #[value(SQUARE_METRE_PER_VOLT_SECOND)]
    SquareMetrePerVoltSecond,
}

/// The units of `ElectricalResistance`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ELECTRICAL_RESISTANCE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ELECTRICAL_RESISTANCE, N>;
        fn value(&self, quantity: SIQuantity<ELECTRICAL_RESISTANCE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ElectricalResistanceUnit {
    #[default]
    #[value(OHM)]
    Ohm,
    #[value(ABOHM)]
    Abohm,
    #[value(STATOHM)]
    Statohm,
}

/// The units of `ElectricalResistivity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ELECTRICAL_RESISTIVITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ELECTRICAL_RESISTIVITY, N>;
        fn value(&self, quantity: SIQuantity<ELECTRICAL_RESISTIVITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ElectricalResistivityUnit {
    #[default]
    #[value(OHM_METRE)]
    OhmMetre,
    #[value(ABOHM_METRE)]
    AbohmMetre,
    #[value(STATOHM_METRE)]
    StatohmMetre,
    #[value(OHM_INCH)]
    OhmInch,
    #[value(OHM_FOOT)]
    OhmFoot,
    #[value(OHM_YARD)]
    OhmYard,
    #[value(OHM_SQUARE_MILLIMETRE_PER_METRE)]
    OhmSquareMillimetrePerMetre,
}

/// The units of `Capacitance`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<CAPACITANCE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<CAPACITANCE, N>;
        fn value(&self, quantity: SIQuantity<CAPACITANCE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum CapacitanceUnit {
    #[default]
    #[value(FARAD)]
    Farad,
    #[value(ABFARAD)]
    Abfarad,
    #[value(STATFARAD)]
    Statfarad,
}

/// The units of `Inductance`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<INDUCTANCE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<INDUCTANCE, N>;
        fn value(&self, quantity: SIQuantity<INDUCTANCE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum InductanceUnit {
    #[default]
    #[value(HENRY)]
    Henry,
    #[value(ABHENRY)]
    Abhenry,
    #[value(STATHENRY)]
    Stathenry,
}

/// The units of `MagneticFieldStrength`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<MAGNETIC_FIELD_STRENGTH, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<MAGNETIC_FIELD_STRENGTH, N>;
        fn value(&self, quantity: SIQuantity<MAGNETIC_FIELD_STRENGTH, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MagneticFieldStrengthUnit {
    #[default]
    #[value(MAGNETIC_FIELD_STRENGTH_AMPERE_PER_METRE)]
    AmperePerMetre,
    #[value(OERSTED)]
    Oersted,
}

/// The units of `MagneticFlux`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<MAGNETIC_FLUX, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<MAGNETIC_FLUX, N>;
        fn value(&self, quantity: SIQuantity<MAGNETIC_FLUX, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MagneticFluxUnit {
    #[default]
    #[value(WEBER)]
    Weber,
    #[value(MAXWELL)]
    Maxwell,
}

/// The units of `MagneticFluxDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<MAGNETIC_FLUX_DENSITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<MAGNETIC_FLUX_DENSITY, N>;
        fn value(&self, quantity: SIQuantity<MAGNETIC_FLUX_DENSITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MagneticFluxDensityUnit {
    #[default]
    #[value(TESLA)]
    Tesla,
    #[value(GAMMA)]
    Gamma,
    #[value(GAUSS)]
    Gauss,
}

/// The units of `MagneticMoment`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<MAGNETIC_MOMENT, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<MAGNETIC_MOMENT, N>;
        fn value(&self, quantity: SIQuantity<MAGNETIC_MOMENT, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MagneticMomentUnit {
    #[default]
    #[value(AMPERE_SQUARE_METRE)]
    AmpereSquareMetre,
    #[value(JOULE_PER_TESLA)]
    JoulePerTesla,
    #[value(NEWTON_METRE_PER_TESLA)]
    NewtonMetrePerTesla,
    #[value(ERG_PER_GAUSS)]
    ErgPerGauss,
    #[value(BOHR_MAGNETON)]
    BohrMagneton,
    #[value(NUCLEAR_MAGNETON)]
    NuclearMagneton,
    #[value(ATOMIC_UNIT_OF_MAGNETIC_DIPOLE_MOMENT)]
    AtomicUnitOfMagneticDipoleMoment,
    #[value(DEUTERON_MAGNETIC_MOMENT)]
    DeuteronMagneticMoment,
    #[value(PROTON_MAGNETIC_MOMENT)]
    ProtonMagneticMoment,
    #[value(SHIELDED_PROTON_MAGNETIC_MOMENT)]
    ShieldedProtonMagneticMoment,
    #[value(TRITON_MAGNETIC_MOMENT)]
    TritonMagneticMoment,
}

/// The units of `MagneticPermeability`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<MAGNETIC_PERMEABILITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<MAGNETIC_PERMEABILITY, N>;
        fn value(&self, quantity: SIQuantity<MAGNETIC_PERMEABILITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MagneticPermeabilityUnit {
    #[default]
    #[value(HENRY_PER_METRE)]
    HenryPerMetre,
    #[value(VACUUM_PERMEABILITY)]
    VacuumPermeability,
}

/// The units of `GyromagneticRatio`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<GYROMAGNETIC_RATIO, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<GYROMAGNETIC_RATIO, N>;
        fn value(&self, quantity: SIQuantity<GYROMAGNETIC_RATIO, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum GyromagneticRatioUnit {
    #[default]
    #[value(HERTZ_PER_TESLA)]
    HertzPerTesla,
    #[value(RADIAN_PER_SECOND_TESLA)]
    RadianPerSecondTesla,
    #[value(PROTON_GYROMAGNETIC_RATIO)]
    ProtonGyromagneticRatio,
    #[value(NEUTRON_GYROMAGNETIC_RATIO)]
    NeutronGyromagneticRatio,
    #[value(ELECTRON_GYROMAGNETIC_RATIO)]
    ElectronGyromagneticRatio,
    #[value(SHIELDED_PROTON_GYROMAGNETIC_RATIO)]
    ShieldedProtonGyromagneticRatio,
    #[value(SHIELDED_HELION_GYROMAGNETIC_RATIO)]
    ShieldedHelionGyromagneticRatio,
    #[value(DEUTERON_GYROMAGNETIC_RATIO)]
    DeuteronGyromagneticRatio,
    #[value(TRITON_GYROMAGNETIC_RATIO)]
    TritonGyromagneticRatio,
    #[value(LITHIUM7_GYROMAGNETIC_RATIO)]
    Lithium7GyromagneticRatio,
    #[value(BORON11_GYROMAGNETIC_RATIO)]
    Boron11GyromagneticRatio,
    #[value(CARBON13_GYROMAGNETIC_RATIO)]
    Carbon13GyromagneticRatio,
    #[value(NITROGEN14_GYROMAGNETIC_RATIO)]
    Nitrogen14GyromagneticRatio,
    #[value(FLOURINE19_GYROMAGNETIC_RATIO)]
    Flourine19GyromagneticRatio,
    #[value(SODIUM23_GYROMAGNETIC_RATIO)]
    Sodium23GyromagneticRatio,
    #[value(PHOSPHORUS31_GYROMAGNETIC_RATIO)]
    Phosphorus31GyromagneticRatio,
}

/// The units of `ReducedGyromagneticRatio`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<REDUCED_GYROMAGNETIC_RATIO, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<REDUCED_GYROMAGNETIC_RATIO, N>;
        fn value(&self, quantity: SIQuantity<REDUCED_GYROMAGNETIC_RATIO, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ReducedGyromagneticRatioUnit {
    #[default]
    #[value(PROTON_REDUCED_GYROMAGNETIC_RATIO)]
    ProtonReducedGyromagneticRatio,
    #[value(NEUTRON_REDUCED_GYROMAGNETIC_RATIO)]
    NeutronReducedGyromagneticRatio,
    #[value(ELECTRON_REDUCED_GYROMAGNETIC_RATIO)]
    ElectronReducedGyromagneticRatio,
    #[value(SHIELDED_PROTON_REDUCED_GYROMAGNETIC_RATIO)]
    ShieldedProtonReducedGyromagneticRatio,
    #[value(SHIELDED_HELION_REDUCED_GYROMAGNETIC_RATIO)]
    ShieldedHelionReducedGyromagneticRatio,
    #[value(DEUTERON_REDUCED_GYROMAGNETIC_RATIO)]
    DeuteronReducedGyromagneticRatio,
    #[value(TRITON_REDUCED_GYROMAGNETIC_RATIO)]
    TritonReducedGyromagneticRatio,
    #[value(LITHIUM7_REDUCED_GYROMAGNETIC_RATIO)]
    Lithium7ReducedGyromagneticRatio,
    #[value(BORON11_REDUCED_GYROMAGNETIC_RATIO)]
    Boron11ReducedGyromagneticRatio,
    #[value(CARBON13_REDUCED_GYROMAGNETIC_RATIO)]
    Carbon13ReducedGyromagneticRatio,
    #[value(NITROGEN14_REDUCED_GYROMAGNETIC_RATIO)]
    Nitrogen14ReducedGyromagneticRatio,
    #[value(FLOURINE19_REDUCED_GYROMAGNETIC_RATIO)]
    Flourine19ReducedGyromagneticRatio,
    #[value(SODIUM23_REDUCED_GYROMAGNETIC_RATIO)]
    Sodium23ReducedGyromagneticRatio,
    #[value(PHOSPHORUS31_REDUCED_GYROMAGNETIC_RATIO)]
    Phosphorus31ReducedGyromagneticRatio,
}
