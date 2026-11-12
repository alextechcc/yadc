//! The units of `electromagnetic`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::base::{KILOGRAM, METRE, RADIAN, SECOND};
use crate::electromagnetic::consts::*;
use crate::electromagnetic::*;
use core::marker::Destruct;
use yadc::{Unit, num_traits::Num};

/// The units of `ElectricCharge`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCharge<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricCharge<N>: [const] Unit<ElectricCharge<N>, N> + [const] Destruct)]
pub enum ElectricChargeUnit {
    #[default]
    #[unit("C", "coulombs", by = ElectricCharge::<N>::from(COULOMB))]
    Coulomb,
    #[unit("A · h", "ampere hours", by = ElectricCharge::<N>::from(COULOMB * Exact::int(3600)))]
    AmpereHour,
    #[unit("e", "elementary charges", by = ElectricCharge::<N>::from(ELEMENTARY_CHARGE))]
    ElementaryCharge,
    #[unit("a.u. of charge", "atomic units of charge", by = ElectricCharge::<N>::from(ATOMIC_UNIT_OF_CHARGE))]
    AtomicUnitOfCharge,
    #[unit("abC", "abcoulombs", by = ElectricCharge::<N>::from(ABCOULOMB))]
    Abcoulomb,
    #[unit("F", "faradays", by = ElectricCharge::<N>::from(FARADAY))]
    Faraday,
    #[unit("Fr", "franklins", by = ElectricCharge::<N>::from(FRANKLIN))]
    Franklin,
    #[unit("statC", "statcoulombs", by = ElectricCharge::<N>::from(STATCOULOMB))]
    Statcoulomb,
}

/// The units of `ElectricChargeArealDensity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricChargeArealDensity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricChargeArealDensity<N>: [const] Unit<ElectricChargeArealDensity<N>, N> + [const] Destruct)]
pub enum ElectricChargeArealDensityUnit {
    #[default]
    #[unit("C/m²", "coulombs per square metre", by = ElectricChargeArealDensity::<N>::from(SECOND * AMPERE / (METRE * METRE)))]
    CoulombPerSquareMetre,
}

/// The units of `ElectricChargeLinearDensity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricChargeLinearDensity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricChargeLinearDensity<N>: [const] Unit<ElectricChargeLinearDensity<N>, N> + [const] Destruct)]
pub enum ElectricChargeLinearDensityUnit {
    #[default]
    #[unit("C/m", "coulombs per metre", by = ElectricChargeLinearDensity::<N>::from(SECOND * AMPERE / METRE))]
    CoulombPerMetre,
}

/// The units of `ElectricChargeVolumetricDensity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricChargeVolumetricDensity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricChargeVolumetricDensity<N>: [const] Unit<ElectricChargeVolumetricDensity<N>, N> + [const] Destruct)]
pub enum ElectricChargeVolumetricDensityUnit {
    #[default]
    #[unit("C/m³", "coulombs per cubic metre", by = ElectricChargeVolumetricDensity::<N>::from(SECOND * AMPERE / (METRE * METRE * METRE)))]
    CoulombPerCubicMetre,
}

/// The units of `ElectricCurrent`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCurrent<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricCurrent<N>: [const] Unit<ElectricCurrent<N>, N> + [const] Destruct)]
pub enum ElectricCurrentUnit {
    #[default]
    #[unit("A", "amperes", by = ElectricCurrent::<N>::from(AMPERE))]
    Ampere,
    #[unit("e/s", "elementary charges per second", by = ElectricCurrent::<N>::from(AMPERE * Exact::dec(1602176634, -28)))]
    ElementaryChargePerSecond,
    #[unit("a.u. of charge/s", "atomic units of charge per second", by = ElectricCurrent::<N>::from(AMPERE * Exact::dec(1602176634, -28)))]
    AtomicUnitOfChargePerSecond,
    #[unit("abA", "abamperes", by = ElectricCurrent::<N>::from(ABAMPERE))]
    Abampere,
    #[unit("Gi", "gilberts", by = ElectricCurrent::<N>::from(GILBERT))]
    Gilbert,
    #[unit("statA", "statamperes", by = ElectricCurrent::<N>::from(STATAMPERE))]
    Statampere,
}

/// The units of `ElectricCurrentDensity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCurrentDensity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricCurrentDensity<N>: [const] Unit<ElectricCurrentDensity<N>, N> + [const] Destruct)]
pub enum ElectricCurrentDensityUnit {
    #[default]
    #[unit("A/m²", "amperes per square metre", by = ElectricCurrentDensity::<N>::from(AMPERE / (METRE * METRE)))]
    AmperePerSquareMetre,
}

/// The units of `SurfaceElectricCurrentDensity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SurfaceElectricCurrentDensity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where SurfaceElectricCurrentDensity<N>: [const] Unit<SurfaceElectricCurrentDensity<N>, N> + [const] Destruct)]
pub enum SurfaceElectricCurrentDensityUnit {
    #[default]
    #[unit("A/m", "amperes per metre", by = SurfaceElectricCurrentDensity::<N>::from(AMPERE / METRE))]
    AmperePerMetre,
}

/// The units of `ElectricPotential`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricPotential<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricPotential<N>: [const] Unit<ElectricPotential<N>, N> + [const] Destruct)]
pub enum ElectricPotentialUnit {
    #[default]
    #[unit("V", "volts", by = ElectricPotential::<N>::from(VOLT))]
    Volt,
    #[unit("abV", "abvolts", by = ElectricPotential::<N>::from(ABVOLT))]
    Abvolt,
    #[unit("statV", "statvolts", by = ElectricPotential::<N>::from(STATVOLT))]
    Statvolt,
}

/// The units of `ElectricField`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricField<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricField<N>: [const] Unit<ElectricField<N>, N> + [const] Destruct)]
pub enum ElectricFieldUnit {
    #[default]
    #[unit("V/m", "volts per metre", by = ElectricField::<N>::from(METRE * KILOGRAM / (SECOND * SECOND * SECOND * AMPERE)))]
    VoltPerMetre,
    #[unit("V/mil", "volts per mil", by = ElectricField::<N>::from((METRE * KILOGRAM / (SECOND * SECOND * SECOND * AMPERE)) * (Exact::int(1) / Exact::dec(254, -7))))]
    VoltPerMil,
    #[unit("a.u. of electric field", "atomic units of electric field", by = ElectricField::<N>::from(ATOMIC_UNIT_OF_ELECTRIC_FIELD))]
    AtomicUnitOfElectricField,
}

/// The units of `ElectricDisplacementField`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricDisplacementField<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricDisplacementField<N>: [const] Unit<ElectricDisplacementField<N>, N> + [const] Destruct)]
pub enum ElectricDisplacementFieldUnit {
    #[default]
    #[unit("C/m²", "coulombs per square metre", by = ElectricDisplacementField::<N>::from(SECOND * AMPERE / (METRE * METRE)))]
    CoulombPerSquareMetre,
}

/// The units of `ElectricFlux`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricFlux<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricFlux<N>: [const] Unit<ElectricFlux<N>, N> + [const] Destruct)]
pub enum ElectricFluxUnit {
    #[default]
    #[unit("V ⋅ m", "volt metres", by = ElectricFlux::<N>::from(METRE * METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * AMPERE)))]
    VoltMetre,
}

/// The units of `ElectricDipoleMoment`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricDipoleMoment<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricDipoleMoment<N>: [const] Unit<ElectricDipoleMoment<N>, N> + [const] Destruct)]
pub enum ElectricDipoleMomentUnit {
    #[default]
    #[unit("C · m", "coulomb metres", by = ElectricDipoleMoment::<N>::from(METRE * SECOND * AMPERE))]
    CoulombMetre,
    #[unit("e · a₀", "atomic units of electric dipole moment", by = ElectricDipoleMoment::<N>::from((METRE * SECOND * AMPERE) * (Exact::dec(1602176634, -28) * Exact::dec(529177210903, -22))))]
    AtomicUnitOfElectricDipoleMoment,
    #[unit("a.u. of charge · cm", "atomic unit of charge centimetres", by = ElectricDipoleMoment::<N>::from((METRE * SECOND * AMPERE) * (Exact::dec(1602176634, -28) * Exact::dec(1, -2))))]
    AtomicUnitOfChargeCentimetre,
    #[unit("e · cm", "elementary charge centimetres", by = ElectricDipoleMoment::<N>::from((METRE * SECOND * AMPERE) * (Exact::dec(1602176634, -28) * Exact::dec(1, -2))))]
    ElementaryChargeCentimetre,
}

/// The units of `ElectricQuadrupoleMoment`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricQuadrupoleMoment<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricQuadrupoleMoment<N>: [const] Unit<ElectricQuadrupoleMoment<N>, N> + [const] Destruct)]
pub enum ElectricQuadrupoleMomentUnit {
    #[default]
    #[unit("C · m²", "coulomb square metres", by = ElectricQuadrupoleMoment::<N>::from(METRE * METRE * SECOND * AMPERE))]
    CoulombSquareMetre,
    #[unit("e · b", "elementary charge barns", by = ElectricQuadrupoleMoment::<N>::from((METRE * METRE * SECOND * AMPERE) * (Exact::dec(1602176634, -28) * Exact::dec(1, -28))))]
    ElementaryChargeBarn,
    #[unit("e · a₀²", "atomic units of electric quadrupole moment", by = ElectricQuadrupoleMoment::<N>::from((METRE * METRE * SECOND * AMPERE) * (Exact::dec(1602176634, -28) * Exact::dec(529177210903, -22) * Exact::dec(529177210903, -22))))]
    AtomicUnitOfElectricQuadrupoleMoment,
}

/// The units of `ElectricPermittivity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricPermittivity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricPermittivity<N>: [const] Unit<ElectricPermittivity<N>, N> + [const] Destruct)]
pub enum ElectricPermittivityUnit {
    #[default]
    #[unit("F/m", "farads per metre", by = ElectricPermittivity::<N>::from(SECOND * SECOND * SECOND * SECOND * AMPERE * AMPERE / (METRE * METRE * METRE * KILOGRAM)))]
    FaradPerMetre,
    #[unit("ε₀", "vacuum electric permittivity", by = ElectricPermittivity::<N>::from(VACUUM_PERMITTIVITY))]
    VacuumPermittivity,
}

/// The units of `ElectricalConductance`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricalConductance<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricalConductance<N>: [const] Unit<ElectricalConductance<N>, N> + [const] Destruct)]
pub enum ElectricalConductanceUnit {
    #[default]
    #[unit("S", "siemens", by = ElectricalConductance::<N>::from(SIEMENS))]
    Siemens,
    #[unit("℧", "mhos", by = ElectricalConductance::<N>::from(MHO))]
    Mho,
    #[unit("abmho", "abmhos", by = ElectricalConductance::<N>::from(ABMHO))]
    Abmho,
    #[unit("abS", "abmsiemens", by = ElectricalConductance::<N>::from(ABSIEMENS))]
    Absiemens,
    #[unit("statS", "statsiemens", by = ElectricalConductance::<N>::from(STATSIEMENS))]
    Statsiemens,
    #[unit("statmho", "statmhos", by = ElectricalConductance::<N>::from(STATMHO))]
    Statmho,
}

/// The units of `ElectricalConductivity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricalConductivity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricalConductivity<N>: [const] Unit<ElectricalConductivity<N>, N> + [const] Destruct)]
pub enum ElectricalConductivityUnit {
    #[default]
    #[unit("S/m", "siemens per metre", by = ElectricalConductivity::<N>::from(SECOND * SECOND * SECOND * AMPERE * AMPERE / (METRE * METRE * METRE * KILOGRAM)))]
    SiemensPerMetre,
}

/// The units of `ElectricalMobility`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricalMobility<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricalMobility<N>: [const] Unit<ElectricalMobility<N>, N> + [const] Destruct)]
pub enum ElectricalMobilityUnit {
    #[default]
    #[unit("m²/(V · s)", "square metres per volt second", by = ElectricalMobility::<N>::from(SECOND * SECOND * AMPERE / KILOGRAM))]
    SquareMetrePerVoltSecond,
}

/// The units of `ElectricalResistance`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricalResistance<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricalResistance<N>: [const] Unit<ElectricalResistance<N>, N> + [const] Destruct)]
pub enum ElectricalResistanceUnit {
    #[default]
    #[unit("Ω", "ohms", by = ElectricalResistance::<N>::from(OHM))]
    Ohm,
    #[unit("abΩ", "abohms", by = ElectricalResistance::<N>::from(ABOHM))]
    Abohm,
    #[unit("statΩ", "statohms", by = ElectricalResistance::<N>::from(STATOHM))]
    Statohm,
}

/// The units of `ElectricalResistivity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricalResistivity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricalResistivity<N>: [const] Unit<ElectricalResistivity<N>, N> + [const] Destruct)]
pub enum ElectricalResistivityUnit {
    #[default]
    #[unit("Ω · m", "ohm metres", by = ElectricalResistivity::<N>::from(METRE * METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * AMPERE * AMPERE)))]
    OhmMetre,
    #[unit("abΩ · m", "abohm metres", by = ElectricalResistivity::<N>::from((METRE * METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * AMPERE * AMPERE)) * Exact::dec(1, -9)))]
    AbohmMetre,
    #[unit("statΩ · m", "statohm metres", by = ElectricalResistivity::<N>::from((METRE * METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * AMPERE * AMPERE)) * (Exact::dec(299792458, -6) * Exact::int(2997924580))))]
    StatohmMetre,
    #[unit("Ω · in", "ohm inches", by = ElectricalResistivity::<N>::from((METRE * METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * AMPERE * AMPERE)) * Exact::dec(254, -4)))]
    OhmInch,
    #[unit("Ω · ft", "ohm feet", by = ElectricalResistivity::<N>::from((METRE * METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * AMPERE * AMPERE)) * Exact::dec(3048, -4)))]
    OhmFoot,
    #[unit("Ω · yd", "ohm yards", by = ElectricalResistivity::<N>::from((METRE * METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * AMPERE * AMPERE)) * Exact::dec(9144, -4)))]
    OhmYard,
    #[unit("Ω · mm²/m", "ohm square millimetres per metre", by = ElectricalResistivity::<N>::from((METRE * METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * AMPERE * AMPERE)) * Exact::dec(1, -6)))]
    OhmSquareMillimetrePerMetre,
}

/// The units of `Capacitance`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Capacitance<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Capacitance<N>: [const] Unit<Capacitance<N>, N> + [const] Destruct)]
pub enum CapacitanceUnit {
    #[default]
    #[unit("F", "farads", by = Capacitance::<N>::from(FARAD))]
    Farad,
    #[unit("abF", "abfarads", by = Capacitance::<N>::from(ABFARAD))]
    Abfarad,
    #[unit("statF", "statfarads", by = Capacitance::<N>::from(STATFARAD))]
    Statfarad,
}

/// The units of `Inductance`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Inductance<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Inductance<N>: [const] Unit<Inductance<N>, N> + [const] Destruct)]
pub enum InductanceUnit {
    #[default]
    #[unit("H", "henries", by = Inductance::<N>::from(HENRY))]
    Henry,
    #[unit("abH", "abhenries", by = Inductance::<N>::from(ABHENRY))]
    Abhenry,
    #[unit("statH", "stathenries", by = Inductance::<N>::from(STATHENRY))]
    Stathenry,
}

/// The units of `MagneticFieldStrength`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticFieldStrength<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MagneticFieldStrength<N>: [const] Unit<MagneticFieldStrength<N>, N> + [const] Destruct)]
pub enum MagneticFieldStrengthUnit {
    #[default]
    #[unit("A/m", "amperes per metre", by = MagneticFieldStrength::<N>::from(AMPERE / METRE))]
    AmperePerMetre,
    #[unit("Oe", "oersteds", by = MagneticFieldStrength::<N>::from(OERSTED))]
    Oersted,
}

/// The units of `MagneticFlux`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticFlux<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MagneticFlux<N>: [const] Unit<MagneticFlux<N>, N> + [const] Destruct)]
pub enum MagneticFluxUnit {
    #[default]
    #[unit("Wb", "webers", by = MagneticFlux::<N>::from(WEBER))]
    Weber,
    #[unit("Mx", "maxwells", by = MagneticFlux::<N>::from(MAXWELL))]
    Maxwell,
}

/// The units of `MagneticFluxDensity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticFluxDensity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MagneticFluxDensity<N>: [const] Unit<MagneticFluxDensity<N>, N> + [const] Destruct)]
pub enum MagneticFluxDensityUnit {
    #[default]
    #[unit("T", "teslas", by = MagneticFluxDensity::<N>::from(TESLA))]
    Tesla,
    #[unit("γ", "gammas", by = MagneticFluxDensity::<N>::from(GAMMA))]
    Gamma,
    #[unit("G", "gauss", by = MagneticFluxDensity::<N>::from(GAUSS))]
    Gauss,
}

/// The units of `MagneticMoment`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticMoment<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MagneticMoment<N>: [const] Unit<MagneticMoment<N>, N> + [const] Destruct)]
pub enum MagneticMomentUnit {
    #[default]
    #[unit("A · m²", "ampere square metres", by = MagneticMoment::<N>::from(METRE * METRE * AMPERE))]
    AmpereSquareMetre,
    #[unit("J/T", "joules per tesla", by = MagneticMoment::<N>::from(METRE * METRE * AMPERE))]
    JoulePerTesla,
    #[unit("N · m/T", "newton metres per tesla", by = MagneticMoment::<N>::from(METRE * METRE * AMPERE))]
    NewtonMetrePerTesla,
    #[unit("erg/G", "ergs per gauss", by = MagneticMoment::<N>::from((METRE * METRE * AMPERE) * Exact::dec(1, -3)))]
    ErgPerGauss,
    #[unit("µ(Bohr)", "Bohr magnetons", by = MagneticMoment::<N>::from(BOHR_MAGNETON))]
    BohrMagneton,
    #[unit("μ(Nuclear)", "nuclear magnetons", by = MagneticMoment::<N>::from(NUCLEAR_MAGNETON))]
    NuclearMagneton,
    #[unit("ħ · e/mₑ", " atomic units of magnetic dipole moment", by = MagneticMoment::<N>::from((METRE * METRE * AMPERE) * Exact::dec(185480201566, -34)))]
    AtomicUnitOfMagneticDipoleMoment,
    #[unit("μ(deuteron)", "deuteron magnetic moments", by = MagneticMoment::<N>::from(DEUTERON_MAGNETIC_MOMENT))]
    DeuteronMagneticMoment,
    #[unit("μ(proton)", "proton magnetic moments", by = MagneticMoment::<N>::from(PROTON_MAGNETIC_MOMENT))]
    ProtonMagneticMoment,
    #[unit("μ’(proton)", "shielded proton magnetic moments", by = MagneticMoment::<N>::from(SHIELDED_PROTON_MAGNETIC_MOMENT))]
    ShieldedProtonMagneticMoment,
    #[unit("μ(triton)", "triton magnetic moments", by = MagneticMoment::<N>::from(TRITON_MAGNETIC_MOMENT))]
    TritonMagneticMoment,
}

/// The units of `MagneticPermeability`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticPermeability<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MagneticPermeability<N>: [const] Unit<MagneticPermeability<N>, N> + [const] Destruct)]
pub enum MagneticPermeabilityUnit {
    #[default]
    #[unit("H/m", "henrys per metre", by = MagneticPermeability::<N>::from(METRE * KILOGRAM / (SECOND * SECOND * AMPERE * AMPERE)))]
    HenryPerMetre,
    #[unit("µ₀", "vacuum magnetic permeability", by = MagneticPermeability::<N>::from(VACUUM_PERMEABILITY))]
    VacuumPermeability,
}

/// The units of `GyromagneticRatio`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = GyromagneticRatio<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where GyromagneticRatio<N>: [const] Unit<GyromagneticRatio<N>, N> + [const] Destruct)]
pub enum GyromagneticRatioUnit {
    #[default]
    #[unit("Hz/T", "hertz per tesla", by = GyromagneticRatio::<N>::from(SECOND * AMPERE * RADIAN / KILOGRAM))]
    HertzPerTesla,
    #[unit("rad/(s · T)", "radians per second tesla", by = GyromagneticRatio::<N>::from(SECOND * AMPERE * RADIAN / KILOGRAM))]
    RadianPerSecondTesla,
    #[unit("γₚ", "proton gyromagnetic ratio", by = GyromagneticRatio::<N>::from(PROTON_GYROMAGNETIC_RATIO))]
    ProtonGyromagneticRatio,
    #[unit("γₙ", "neutron gyromagnetic ratio", by = GyromagneticRatio::<N>::from(NEUTRON_GYROMAGNETIC_RATIO))]
    NeutronGyromagneticRatio,
    #[unit("γₑ", "electron gyromagnetic ratio", by = GyromagneticRatio::<N>::from(ELECTRON_GYROMAGNETIC_RATIO))]
    ElectronGyromagneticRatio,
    #[unit("γ'ₚ", "shielded proton gyromagnetic ratio", by = GyromagneticRatio::<N>::from(SHIELDED_PROTON_GYROMAGNETIC_RATIO))]
    ShieldedProtonGyromagneticRatio,
    #[unit("γ'ₕ", "shielded helion gyromagnetic ratio", by = GyromagneticRatio::<N>::from(SHIELDED_HELION_GYROMAGNETIC_RATIO))]
    ShieldedHelionGyromagneticRatio,
    #[unit("γ₂", "deuteron gyromagnetic ratio", by = GyromagneticRatio::<N>::from(DEUTERON_GYROMAGNETIC_RATIO))]
    DeuteronGyromagneticRatio,
    #[unit("γ₃", "triton gyromagnetic ratio", by = GyromagneticRatio::<N>::from(TRITON_GYROMAGNETIC_RATIO))]
    TritonGyromagneticRatio,
    #[unit("γ₇", "lithium7 gyromagnetic ratio", by = GyromagneticRatio::<N>::from(LITHIUM7_GYROMAGNETIC_RATIO))]
    Lithium7GyromagneticRatio,
    #[unit("γ₁₁", "boron11 gyromagnetic ratio", by = GyromagneticRatio::<N>::from(BORON11_GYROMAGNETIC_RATIO))]
    Boron11GyromagneticRatio,
    #[unit("γ₁₃", "carbon13 gyromagnetic ratio", by = GyromagneticRatio::<N>::from(CARBON13_GYROMAGNETIC_RATIO))]
    Carbon13GyromagneticRatio,
    #[unit("γ₁₄", "nitrogen14 gyromagnetic ratio", by = GyromagneticRatio::<N>::from(NITROGEN14_GYROMAGNETIC_RATIO))]
    Nitrogen14GyromagneticRatio,
    #[unit("γ₁₉", "flourine19 gyromagnetic ratio", by = GyromagneticRatio::<N>::from(FLOURINE19_GYROMAGNETIC_RATIO))]
    Flourine19GyromagneticRatio,
    #[unit("γ₂₃", "sodium23 gyromagnetic ratio", by = GyromagneticRatio::<N>::from(SODIUM23_GYROMAGNETIC_RATIO))]
    Sodium23GyromagneticRatio,
    #[unit("γ₃₁", "phosphorus31 gyromagnetic ratio", by = GyromagneticRatio::<N>::from(PHOSPHORUS31_GYROMAGNETIC_RATIO))]
    Phosphorus31GyromagneticRatio,
}

/// The units of `ReducedGyromagneticRatio`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ReducedGyromagneticRatio<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ReducedGyromagneticRatio<N>: [const] Unit<ReducedGyromagneticRatio<N>, N> + [const] Destruct)]
pub enum ReducedGyromagneticRatioUnit {
    #[default]
    #[unit("̅γₚ", "proton reduced gyromagnetic ratio", by = ReducedGyromagneticRatio::<N>::from(PROTON_REDUCED_GYROMAGNETIC_RATIO))]
    ProtonReducedGyromagneticRatio,
    #[unit("̅γₙ", "neutron reduced gyromagnetic ratio", by = ReducedGyromagneticRatio::<N>::from(NEUTRON_REDUCED_GYROMAGNETIC_RATIO))]
    NeutronReducedGyromagneticRatio,
    #[unit("̅γₑ", "electron reduced gyromagnetic ratio", by = ReducedGyromagneticRatio::<N>::from(ELECTRON_REDUCED_GYROMAGNETIC_RATIO))]
    ElectronReducedGyromagneticRatio,
    #[unit("γ'ₚ", "shielded proton reduced gyromagnetic ratio", by = ReducedGyromagneticRatio::<N>::from(SHIELDED_PROTON_REDUCED_GYROMAGNETIC_RATIO))]
    ShieldedProtonReducedGyromagneticRatio,
    #[unit("γ'ₕ", "shielded helion reduced gyromagnetic ratio", by = ReducedGyromagneticRatio::<N>::from(SHIELDED_HELION_REDUCED_GYROMAGNETIC_RATIO))]
    ShieldedHelionReducedGyromagneticRatio,
    #[unit("̅γ₂", "deuteron reduced gyromagnetic ratio", by = ReducedGyromagneticRatio::<N>::from(DEUTERON_REDUCED_GYROMAGNETIC_RATIO))]
    DeuteronReducedGyromagneticRatio,
    #[unit("̅γ₃", "triton reduced gyromagnetic ratio", by = ReducedGyromagneticRatio::<N>::from(TRITON_REDUCED_GYROMAGNETIC_RATIO))]
    TritonReducedGyromagneticRatio,
    #[unit("̅γ₇", "lithium7 reduced gyromagnetic ratio", by = ReducedGyromagneticRatio::<N>::from(LITHIUM7_REDUCED_GYROMAGNETIC_RATIO))]
    Lithium7ReducedGyromagneticRatio,
    #[unit("̅γ₁₁", "boron11 reduced gyromagnetic ratio", by = ReducedGyromagneticRatio::<N>::from(BORON11_REDUCED_GYROMAGNETIC_RATIO))]
    Boron11ReducedGyromagneticRatio,
    #[unit("̅γ₁₃", "carbon13 reduced gyromagnetic ratio", by = ReducedGyromagneticRatio::<N>::from(CARBON13_REDUCED_GYROMAGNETIC_RATIO))]
    Carbon13ReducedGyromagneticRatio,
    #[unit("̅γ₁₄", "nitrogen14 reduced gyromagnetic ratio", by = ReducedGyromagneticRatio::<N>::from(NITROGEN14_REDUCED_GYROMAGNETIC_RATIO))]
    Nitrogen14ReducedGyromagneticRatio,
    #[unit("̅γ₁₉", "flourine19 reduced gyromagnetic ratio", by = ReducedGyromagneticRatio::<N>::from(FLOURINE19_REDUCED_GYROMAGNETIC_RATIO))]
    Flourine19ReducedGyromagneticRatio,
    #[unit("̅γ₂₃", "sodium23 reduced gyromagnetic ratio", by = ReducedGyromagneticRatio::<N>::from(SODIUM23_REDUCED_GYROMAGNETIC_RATIO))]
    Sodium23ReducedGyromagneticRatio,
    #[unit("̅γ₃₁", "phosphorus31 reduced gyromagnetic ratio", by = ReducedGyromagneticRatio::<N>::from(PHOSPHORUS31_REDUCED_GYROMAGNETIC_RATIO))]
    Phosphorus31ReducedGyromagneticRatio,
}
