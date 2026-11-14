//! The units of `electromagnetic`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

#[allow(unused_imports)]
use crate::electromagnetic::*;
#[allow(unused_imports)]
use crate::*;

/// The units of `ElectricCharge`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum ElectricChargeUnit {
    #[default]
    #[value(COULOMB.unit())]
    Coulomb,
    #[value(AMPERE_HOUR.unit())]
    AmpereHour,
    #[value(ELEMENTARY_CHARGE.unit())]
    ElementaryCharge,
    #[value(ATOMIC_UNIT_OF_CHARGE.unit())]
    AtomicUnitOfCharge,
    #[value(ABCOULOMB.unit())]
    Abcoulomb,
    #[value(FARADAY.unit())]
    Faraday,
    #[value(FRANKLIN.unit())]
    Franklin,
    #[value(STATCOULOMB.unit())]
    Statcoulomb,
}

/// The units of `ElectricCurrent`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum ElectricCurrentUnit {
    #[default]
    #[value(AMPERE.unit())]
    Ampere,
    #[value(ABAMPERE.unit())]
    Abampere,
    #[value(GILBERT.unit())]
    Gilbert,
    #[value(STATAMPERE.unit())]
    Statampere,
}

/// The units of `ElectricPotential`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum ElectricPotentialUnit {
    #[default]
    #[value(VOLT.unit())]
    Volt,
    #[value(ABVOLT.unit())]
    Abvolt,
    #[value(STATVOLT.unit())]
    Statvolt,
}

/// The units of `ElectricField`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum ElectricFieldUnit {
    #[default]
    #[value(METRE_KILOGRAM_PER_SECOND_CUBED_AMPERE.unit())]
    MetreKilogramPerSecondCubedAmpere,
    #[value(ATOMIC_UNIT_OF_ELECTRIC_FIELD.unit())]
    AtomicUnitOfElectricField,
}

/// The units of `ElectricFlux`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum ElectricFluxUnit {
    #[default]
    #[value(VOLT_METRE.unit())]
    VoltMetre,
}

/// The units of `ElectricDipoleMoment`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum ElectricDipoleMomentUnit {
    #[default]
    #[value(COULOMB_METRE.unit())]
    CoulombMetre,
    #[value(ATOMIC_UNIT_OF_ELECTRIC_DIPOLE_MOMENT.unit())]
    AtomicUnitOfElectricDipoleMoment,
    #[value(ATOMIC_UNIT_OF_CHARGE_CENTIMETRE.unit())]
    AtomicUnitOfChargeCentimetre,
    #[value(ELEMENTARY_CHARGE_CENTIMETRE.unit())]
    ElementaryChargeCentimetre,
}

/// The units of `ElectricQuadrupoleMoment`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum ElectricQuadrupoleMomentUnit {
    #[default]
    #[value(COULOMB_SQUARE_METRE.unit())]
    CoulombSquareMetre,
    #[value(ELEMENTARY_CHARGE_BARN.unit())]
    ElementaryChargeBarn,
    #[value(ATOMIC_UNIT_OF_ELECTRIC_QUADRUPOLE_MOMENT.unit())]
    AtomicUnitOfElectricQuadrupoleMoment,
}

/// The units of `ElectricPermittivity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum ElectricPermittivityUnit {
    #[default]
    #[value(SECOND_POW4_AMPERE_SQUARED_PER_METRE_CUBED_KILOGRAM.unit())]
    SecondPow4AmpereSquaredPerMetreCubedKilogram,
    #[value(VACUUM_PERMITTIVITY.unit())]
    VacuumPermittivity,
}

/// The units of `ElectricalConductance`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum ElectricalConductanceUnit {
    #[default]
    #[value(SIEMENS.unit())]
    Siemens,
    #[value(MHO.unit())]
    Mho,
    #[value(ABMHO.unit())]
    Abmho,
    #[value(ABSIEMENS.unit())]
    Absiemens,
    #[value(STATSIEMENS.unit())]
    Statsiemens,
    #[value(STATMHO.unit())]
    Statmho,
}

/// The units of `ElectricalResistance`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum ElectricalResistanceUnit {
    #[default]
    #[value(OHM.unit())]
    Ohm,
    #[value(ABOHM.unit())]
    Abohm,
    #[value(STATOHM.unit())]
    Statohm,
}

/// The units of `ElectricalResistivity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum ElectricalResistivityUnit {
    #[default]
    #[value(OHM_METRE.unit())]
    OhmMetre,
    #[value(ABOHM_METRE.unit())]
    AbohmMetre,
    #[value(STATOHM_METRE.unit())]
    StatohmMetre,
    #[value(OHM_INCH.unit())]
    OhmInch,
    #[value(OHM_FOOT.unit())]
    OhmFoot,
    #[value(OHM_YARD.unit())]
    OhmYard,
}

/// The units of `Capacitance`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum CapacitanceUnit {
    #[default]
    #[value(FARAD.unit())]
    Farad,
    #[value(ABFARAD.unit())]
    Abfarad,
    #[value(STATFARAD.unit())]
    Statfarad,
}

/// The units of `Inductance`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum InductanceUnit {
    #[default]
    #[value(HENRY.unit())]
    Henry,
    #[value(ABHENRY.unit())]
    Abhenry,
    #[value(STATHENRY.unit())]
    Stathenry,
}

/// The units of `MagneticFieldStrength`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum MagneticFieldStrengthUnit {
    #[default]
    #[value(AMPERE_PER_METRE.unit())]
    AmperePerMetre,
    #[value(OERSTED.unit())]
    Oersted,
}

/// The units of `MagneticFlux`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum MagneticFluxUnit {
    #[default]
    #[value(WEBER.unit())]
    Weber,
    #[value(MAXWELL.unit())]
    Maxwell,
}

/// The units of `MagneticFluxDensity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum MagneticFluxDensityUnit {
    #[default]
    #[value(TESLA.unit())]
    Tesla,
    #[value(GAMMA.unit())]
    Gamma,
    #[value(GAUSS.unit())]
    Gauss,
}

/// The units of `MagneticMoment`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum MagneticMomentUnit {
    #[default]
    #[value(AMPERE_SQUARE_METRE.unit())]
    AmpereSquareMetre,
    #[value(BOHR_MAGNETON.unit())]
    BohrMagneton,
    #[value(NUCLEAR_MAGNETON.unit())]
    NuclearMagneton,
    #[value(ATOMIC_UNIT_OF_MAGNETIC_DIPOLE_MOMENT.unit())]
    AtomicUnitOfMagneticDipoleMoment,
    #[value(DEUTERON_MAGNETIC_MOMENT.unit())]
    DeuteronMagneticMoment,
    #[value(PROTON_MAGNETIC_MOMENT.unit())]
    ProtonMagneticMoment,
    #[value(SHIELDED_PROTON_MAGNETIC_MOMENT.unit())]
    ShieldedProtonMagneticMoment,
    #[value(TRITON_MAGNETIC_MOMENT.unit())]
    TritonMagneticMoment,
}

/// The units of `MagneticPermeability`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum MagneticPermeabilityUnit {
    #[default]
    #[value(METRE_KILOGRAM_PER_SECOND_SQUARED_AMPERE_SQUARED.unit())]
    MetreKilogramPerSecondSquaredAmpereSquared,
    #[value(VACUUM_PERMEABILITY.unit())]
    VacuumPermeability,
}

/// The units of `GyromagneticRatio`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum GyromagneticRatioUnit {
    #[default]
    #[value(SECOND_AMPERE_RADIAN_PER_KILOGRAM.unit())]
    SecondAmpereRadianPerKilogram,
    #[value(PROTON_GYROMAGNETIC_RATIO.unit())]
    ProtonGyromagneticRatio,
    #[value(NEUTRON_GYROMAGNETIC_RATIO.unit())]
    NeutronGyromagneticRatio,
    #[value(ELECTRON_GYROMAGNETIC_RATIO.unit())]
    ElectronGyromagneticRatio,
    #[value(SHIELDED_PROTON_GYROMAGNETIC_RATIO.unit())]
    ShieldedProtonGyromagneticRatio,
    #[value(SHIELDED_HELION_GYROMAGNETIC_RATIO.unit())]
    ShieldedHelionGyromagneticRatio,
    #[value(DEUTERON_GYROMAGNETIC_RATIO.unit())]
    DeuteronGyromagneticRatio,
    #[value(TRITON_GYROMAGNETIC_RATIO.unit())]
    TritonGyromagneticRatio,
    #[value(LITHIUM7_GYROMAGNETIC_RATIO.unit())]
    Lithium7GyromagneticRatio,
    #[value(BORON11_GYROMAGNETIC_RATIO.unit())]
    Boron11GyromagneticRatio,
    #[value(CARBON13_GYROMAGNETIC_RATIO.unit())]
    Carbon13GyromagneticRatio,
    #[value(NITROGEN14_GYROMAGNETIC_RATIO.unit())]
    Nitrogen14GyromagneticRatio,
    #[value(FLOURINE19_GYROMAGNETIC_RATIO.unit())]
    Flourine19GyromagneticRatio,
    #[value(SODIUM23_GYROMAGNETIC_RATIO.unit())]
    Sodium23GyromagneticRatio,
    #[value(PHOSPHORUS31_GYROMAGNETIC_RATIO.unit())]
    Phosphorus31GyromagneticRatio,
}

/// The units of `ReducedGyromagneticRatio`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum ReducedGyromagneticRatioUnit {
    #[default]
    #[value(SECOND_AMPERE_PER_KILOGRAM.unit())]
    SecondAmperePerKilogram,
    #[value(PROTON_REDUCED_GYROMAGNETIC_RATIO.unit())]
    ProtonReducedGyromagneticRatio,
    #[value(NEUTRON_REDUCED_GYROMAGNETIC_RATIO.unit())]
    NeutronReducedGyromagneticRatio,
    #[value(ELECTRON_REDUCED_GYROMAGNETIC_RATIO.unit())]
    ElectronReducedGyromagneticRatio,
    #[value(SHIELDED_PROTON_REDUCED_GYROMAGNETIC_RATIO.unit())]
    ShieldedProtonReducedGyromagneticRatio,
    #[value(SHIELDED_HELION_REDUCED_GYROMAGNETIC_RATIO.unit())]
    ShieldedHelionReducedGyromagneticRatio,
    #[value(DEUTERON_REDUCED_GYROMAGNETIC_RATIO.unit())]
    DeuteronReducedGyromagneticRatio,
    #[value(TRITON_REDUCED_GYROMAGNETIC_RATIO.unit())]
    TritonReducedGyromagneticRatio,
    #[value(LITHIUM7_REDUCED_GYROMAGNETIC_RATIO.unit())]
    Lithium7ReducedGyromagneticRatio,
    #[value(BORON11_REDUCED_GYROMAGNETIC_RATIO.unit())]
    Boron11ReducedGyromagneticRatio,
    #[value(CARBON13_REDUCED_GYROMAGNETIC_RATIO.unit())]
    Carbon13ReducedGyromagneticRatio,
    #[value(NITROGEN14_REDUCED_GYROMAGNETIC_RATIO.unit())]
    Nitrogen14ReducedGyromagneticRatio,
    #[value(FLOURINE19_REDUCED_GYROMAGNETIC_RATIO.unit())]
    Flourine19ReducedGyromagneticRatio,
    #[value(SODIUM23_REDUCED_GYROMAGNETIC_RATIO.unit())]
    Sodium23ReducedGyromagneticRatio,
    #[value(PHOSPHORUS31_REDUCED_GYROMAGNETIC_RATIO.unit())]
    Phosphorus31ReducedGyromagneticRatio,
}
