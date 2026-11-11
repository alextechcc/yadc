use crate::{Exact, ISQ, SIQuantity, counted};
use consts::*;
use yadc::Dimension;

pub type ElectricCharge<N> = SIQuantity<
    {
        ISQ {
            s: 1,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricChargeArealDensity<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            s: 1,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricChargeLinearDensity<N> = SIQuantity<
    {
        ISQ {
            m: -1,
            s: 1,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricChargeVolumetricDensity<N> = SIQuantity<
    {
        ISQ {
            m: -3,
            s: 1,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricCurrent<N> = SIQuantity<
    {
        ISQ {
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricCurrentDensity<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type SurfaceElectricCurrentDensity<N> = SIQuantity<
    {
        ISQ {
            m: -1,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricPotential<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -3,
            A: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricField<N> = SIQuantity<
    {
        ISQ {
            m: 1,
            kg: 1,
            s: -3,
            A: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricDisplacementField<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            s: 1,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricFlux<N> = SIQuantity<
    {
        ISQ {
            m: 3,
            kg: 1,
            s: -3,
            A: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricDipoleMoment<N> = SIQuantity<
    {
        ISQ {
            m: 1,
            s: 1,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricQuadrupoleMoment<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            s: 1,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricPermittivity<N> = SIQuantity<
    {
        ISQ {
            m: -3,
            kg: -1,
            s: 4,
            A: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricalConductance<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            kg: -1,
            s: 3,
            A: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricalConductivity<N> = SIQuantity<
    {
        ISQ {
            m: -3,
            kg: -1,
            s: 3,
            A: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricalMobility<N> = SIQuantity<
    {
        ISQ {
            kg: -1,
            s: 2,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricalResistance<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -3,
            A: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricalResistivity<N> = SIQuantity<
    {
        ISQ {
            m: 3,
            kg: 1,
            s: -3,
            A: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Capacitance<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            kg: -1,
            s: 4,
            A: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Inductance<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -2,
            A: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MagneticFieldStrength<N> = SIQuantity<
    {
        ISQ {
            m: -1,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MagneticFlux<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -2,
            A: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MagneticFluxDensity<N> = SIQuantity<
    {
        ISQ {
            kg: 1,
            s: -2,
            A: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MagneticMoment<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MagneticPermeability<N> = SIQuantity<
    {
        ISQ {
            m: 1,
            kg: 1,
            s: -2,
            A: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type GyromagneticRatio<N> = SIQuantity<
    {
        ISQ {
            kg: -1,
            s: 1,
            A: 1,
            rad: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ReducedGyromagneticRatio<N> = SIQuantity<
    {
        ISQ {
            kg: -1,
            s: 1,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

/// The constants of this module: the base constants of its quantities
/// (re-exported), then one per multiple, each defined exactly from them -
/// `INCH` is `0.0254 * METRE` - and counted in the number type its use asks
/// for, rounded once: `let d: Length<f64> = INCH;` is an `f64` length,
/// `Length<f32>` an `f32` one, `INCH::<Exact>` the exact value. Each is a
/// unit (`INCH(3.0)`, `INCH.value(d)`). A prefixed unit is a prefix times
/// one: `KILO * METRE`. They compose in `const` items:
/// `const LIMIT: Velocity<f64> = 50.0 * (KILO * METRE) / HOUR;`.
pub mod consts {
    use super::*;
    use crate::base::{KILOGRAM, METRE, RADIAN, SECOND};

    /// The ampere, the SI base unit of electric current.
    pub use crate::base::AMPERE;

    /// One `Coulomb` (C), as an `ElectricCharge`.
    pub const COULOMB<N: const From<Exact>>: ElectricCharge<N> = counted(SECOND::<Exact> * AMPERE::<Exact>);

    /// One `AmpereHour` (A · h), as an `ElectricCharge`.
    pub const AMPERE_HOUR<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::int(3600) * COULOMB::<Exact>);

    /// One `ElementaryCharge` (e), as an `ElectricCharge`.
    pub const ELEMENTARY_CHARGE<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::dec(1602176634, -28) * COULOMB::<Exact>);

    /// One `AtomicUnitOfCharge` (a.u. of charge), as an `ElectricCharge`.
    pub const ATOMIC_UNIT_OF_CHARGE<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::dec(1602176634, -28) * COULOMB::<Exact>);

    /// One `Abcoulomb` (abC), as an `ElectricCharge`.
    pub const ABCOULOMB<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::int(10) * COULOMB::<Exact>);

    /// One `Faraday` (F), as an `ElectricCharge`.
    pub const FARADAY<N: const From<Exact>>: ElectricCharge<N> = counted((Exact::dec(1602176634, -28) * Exact::int(602214076000000000000000)) * COULOMB::<Exact>);

    /// One `Franklin` (Fr), as an `ElectricCharge`.
    pub const FRANKLIN<N: const From<Exact>>: ElectricCharge<N> = counted((Exact::int(1) / Exact::int(2997924580)) * COULOMB::<Exact>);

    /// One `Statcoulomb` (statC), as an `ElectricCharge`.
    pub const STATCOULOMB<N: const From<Exact>>: ElectricCharge<N> = counted((Exact::int(1) / Exact::int(2997924580)) * COULOMB::<Exact>);

    /// One `CoulombPerSquareMetre` (C/m²), as an `ElectricChargeArealDensity`.
    pub const ELECTRIC_CHARGE_AREAL_DENSITY_COULOMB_PER_SQUARE_METRE<N: const From<Exact>>: ElectricChargeArealDensity<N> = counted(SECOND::<Exact> * AMPERE::<Exact> / (METRE::<Exact> * METRE::<Exact>));

    /// One `CoulombPerMetre` (C/m), as an `ElectricChargeLinearDensity`.
    pub const COULOMB_PER_METRE<N: const From<Exact>>: ElectricChargeLinearDensity<N> = counted(SECOND::<Exact> * AMPERE::<Exact> / METRE::<Exact>);

    /// One `CoulombPerCubicMetre` (C/m³), as an `ElectricChargeVolumetricDensity`.
    pub const COULOMB_PER_CUBIC_METRE<N: const From<Exact>>: ElectricChargeVolumetricDensity<N> = counted(SECOND::<Exact> * AMPERE::<Exact> / (METRE::<Exact> * METRE::<Exact> * METRE::<Exact>));

    /// One `ElementaryChargePerSecond` (e/s), as an `ElectricCurrent`.
    pub const ELEMENTARY_CHARGE_PER_SECOND<N: const From<Exact>>: ElectricCurrent<N> = counted(Exact::dec(1602176634, -28) * AMPERE::<Exact>);

    /// One `AtomicUnitOfChargePerSecond` (a.u. of charge/s), as an `ElectricCurrent`.
    pub const ATOMIC_UNIT_OF_CHARGE_PER_SECOND<N: const From<Exact>>: ElectricCurrent<N> = counted(Exact::dec(1602176634, -28) * AMPERE::<Exact>);

    /// One `Abampere` (abA), as an `ElectricCurrent`.
    pub const ABAMPERE<N: const From<Exact>>: ElectricCurrent<N> = counted(Exact::int(10) * AMPERE::<Exact>);

    /// One `Gilbert` (Gi), as an `ElectricCurrent`.
    pub const GILBERT<N: const From<Exact>>: ElectricCurrent<N> = counted((Exact::int(10) / (Exact::int(4) * Exact::PI)) * AMPERE::<Exact>);

    /// One `Statampere` (statA), as an `ElectricCurrent`.
    pub const STATAMPERE<N: const From<Exact>>: ElectricCurrent<N> = counted((Exact::int(1) / Exact::int(2997924580)) * AMPERE::<Exact>);

    /// One `AmperePerSquareMetre` (A/m²), as an `ElectricCurrentDensity`.
    pub const AMPERE_PER_SQUARE_METRE<N: const From<Exact>>: ElectricCurrentDensity<N> = counted(AMPERE::<Exact> / (METRE::<Exact> * METRE::<Exact>));

    /// One `AmperePerMetre` (A/m), as a `SurfaceElectricCurrentDensity`.
    pub const SURFACE_ELECTRIC_CURRENT_DENSITY_AMPERE_PER_METRE<N: const From<Exact>>: SurfaceElectricCurrentDensity<N> = counted(AMPERE::<Exact> / METRE::<Exact>);

    /// One `Volt` (V), as an `ElectricPotential`.
    pub const VOLT<N: const From<Exact>>: ElectricPotential<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * AMPERE::<Exact>));

    /// One `Abvolt` (abV), as an `ElectricPotential`.
    pub const ABVOLT<N: const From<Exact>>: ElectricPotential<N> = counted(Exact::dec(1, -8) * VOLT::<Exact>);

    /// One `Statvolt` (statV), as an `ElectricPotential`.
    pub const STATVOLT<N: const From<Exact>>: ElectricPotential<N> = counted(Exact::dec(299792458, -6) * VOLT::<Exact>);

    /// One `VoltPerMetre` (V/m), as an `ElectricField`.
    pub const VOLT_PER_METRE<N: const From<Exact>>: ElectricField<N> = counted(METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * AMPERE::<Exact>));

    /// One `VoltPerMil` (V/mil), as an `ElectricField`.
    pub const VOLT_PER_MIL<N: const From<Exact>>: ElectricField<N> = counted((Exact::int(1) / Exact::dec(254, -7)) * VOLT_PER_METRE::<Exact>);

    /// One `AtomicUnitOfElectricField` (a.u. of electric field), as an `ElectricField`.
    pub const ATOMIC_UNIT_OF_ELECTRIC_FIELD<N: const From<Exact>>: ElectricField<N> = counted((Exact::dec(43597447222071, -31) / (Exact::dec(1602176634, -28) * Exact::dec(529177210903, -22))) * VOLT_PER_METRE::<Exact>);

    /// One `CoulombPerSquareMetre` (C/m²), as an `ElectricDisplacementField`.
    pub const ELECTRIC_DISPLACEMENT_FIELD_COULOMB_PER_SQUARE_METRE<N: const From<Exact>>: ElectricDisplacementField<N> = counted(SECOND::<Exact> * AMPERE::<Exact> / (METRE::<Exact> * METRE::<Exact>));

    /// One `VoltMetre` (V ⋅ m), as an `ElectricFlux`.
    pub const VOLT_METRE<N: const From<Exact>>: ElectricFlux<N> = counted(METRE::<Exact> * METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * AMPERE::<Exact>));

    /// One `CoulombMetre` (C · m), as an `ElectricDipoleMoment`.
    pub const COULOMB_METRE<N: const From<Exact>>: ElectricDipoleMoment<N> = counted(METRE::<Exact> * SECOND::<Exact> * AMPERE::<Exact>);

    /// One `AtomicUnitOfChargeCentimetre` (a.u. of charge · cm), as an `ElectricDipoleMoment`.
    pub const ATOMIC_UNIT_OF_CHARGE_CENTIMETRE<N: const From<Exact>>: ElectricDipoleMoment<N> = counted((Exact::dec(1602176634, -28) * Exact::dec(1, -2)) * COULOMB_METRE::<Exact>);

    /// One `ElementaryChargeCentimetre` (e · cm), as an `ElectricDipoleMoment`.
    pub const ELEMENTARY_CHARGE_CENTIMETRE<N: const From<Exact>>: ElectricDipoleMoment<N> = counted((Exact::dec(1602176634, -28) * Exact::dec(1, -2)) * COULOMB_METRE::<Exact>);

    /// One `AtomicUnitOfElectricDipoleMoment` (e · a₀), as an `ElectricDipoleMoment`.
    pub const ATOMIC_UNIT_OF_ELECTRIC_DIPOLE_MOMENT<N: const From<Exact>>: ElectricDipoleMoment<N> = counted((Exact::dec(1602176634, -28) * Exact::dec(529177210903, -22)) * COULOMB_METRE::<Exact>);

    /// One `CoulombSquareMetre` (C · m²), as an `ElectricQuadrupoleMoment`.
    pub const COULOMB_SQUARE_METRE<N: const From<Exact>>: ElectricQuadrupoleMoment<N> = counted(METRE::<Exact> * METRE::<Exact> * SECOND::<Exact> * AMPERE::<Exact>);

    /// One `ElementaryChargeBarn` (e · b), as an `ElectricQuadrupoleMoment`.
    pub const ELEMENTARY_CHARGE_BARN<N: const From<Exact>>: ElectricQuadrupoleMoment<N> = counted((Exact::dec(1602176634, -28) * Exact::dec(1, -28)) * COULOMB_SQUARE_METRE::<Exact>);

    /// One `AtomicUnitOfElectricQuadrupoleMoment` (e · a₀²), as an `ElectricQuadrupoleMoment`.
    pub const ATOMIC_UNIT_OF_ELECTRIC_QUADRUPOLE_MOMENT<N: const From<Exact>>: ElectricQuadrupoleMoment<N> = counted((Exact::dec(1602176634, -28) * Exact::dec(529177210903, -22) * Exact::dec(529177210903, -22)) * COULOMB_SQUARE_METRE::<Exact>);

    /// One `FaradPerMetre` (F/m), as an `ElectricPermittivity`.
    pub const FARAD_PER_METRE<N: const From<Exact>>: ElectricPermittivity<N> = counted(SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * AMPERE::<Exact> * AMPERE::<Exact> / (METRE::<Exact> * METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact>));

    /// One `VacuumPermittivity` (ε₀), as an `ElectricPermittivity`.
    pub const VACUUM_PERMITTIVITY<N: const From<Exact>>: ElectricPermittivity<N> = counted(Exact::dec(88541878128, -22) * FARAD_PER_METRE::<Exact>);

    /// One `Siemens` (S), as an `ElectricalConductance`.
    pub const SIEMENS<N: const From<Exact>>: ElectricalConductance<N> = counted(SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * AMPERE::<Exact> * AMPERE::<Exact> / (METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact>));

    /// One `Mho` (℧), as an `ElectricalConductance`.
    pub const MHO<N: const From<Exact>>: ElectricalConductance<N> = counted(SIEMENS::<Exact>);

    /// One `Abmho` (abmho), as an `ElectricalConductance`.
    pub const ABMHO<N: const From<Exact>>: ElectricalConductance<N> = counted(Exact::int(1000000000) * SIEMENS::<Exact>);

    /// One `Absiemens` (abS), as an `ElectricalConductance`.
    pub const ABSIEMENS<N: const From<Exact>>: ElectricalConductance<N> = counted(Exact::int(1000000000) * SIEMENS::<Exact>);

    /// One `Statsiemens` (statS), as an `ElectricalConductance`.
    pub const STATSIEMENS<N: const From<Exact>>: ElectricalConductance<N> = counted((Exact::int(1) / (Exact::dec(299792458, -6) * Exact::int(2997924580))) * SIEMENS::<Exact>);

    /// One `Statmho` (statmho), as an `ElectricalConductance`.
    pub const STATMHO<N: const From<Exact>>: ElectricalConductance<N> = counted((Exact::int(1) / (Exact::dec(299792458, -6) * Exact::int(2997924580))) * SIEMENS::<Exact>);

    /// One `SiemensPerMetre` (S/m), as an `ElectricalConductivity`.
    pub const SIEMENS_PER_METRE<N: const From<Exact>>: ElectricalConductivity<N> = counted(SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * AMPERE::<Exact> * AMPERE::<Exact> / (METRE::<Exact> * METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact>));

    /// One `SquareMetrePerVoltSecond` (m²/(V · s)), as an `ElectricalMobility`.
    pub const SQUARE_METRE_PER_VOLT_SECOND<N: const From<Exact>>: ElectricalMobility<N> = counted(SECOND::<Exact> * SECOND::<Exact> * AMPERE::<Exact> / KILOGRAM::<Exact>);

    /// One `Ohm` (Ω), as an `ElectricalResistance`.
    pub const OHM<N: const From<Exact>>: ElectricalResistance<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * AMPERE::<Exact> * AMPERE::<Exact>));

    /// One `Abohm` (abΩ), as an `ElectricalResistance`.
    pub const ABOHM<N: const From<Exact>>: ElectricalResistance<N> = counted(Exact::dec(1, -9) * OHM::<Exact>);

    /// One `Statohm` (statΩ), as an `ElectricalResistance`.
    pub const STATOHM<N: const From<Exact>>: ElectricalResistance<N> = counted((Exact::dec(299792458, -6) * Exact::int(2997924580)) * OHM::<Exact>);

    /// One `OhmMetre` (Ω · m), as an `ElectricalResistivity`.
    pub const OHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(METRE::<Exact> * METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * AMPERE::<Exact> * AMPERE::<Exact>));

    /// One `AbohmMetre` (abΩ · m), as an `ElectricalResistivity`.
    pub const ABOHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::dec(1, -9) * OHM_METRE::<Exact>);

    /// One `StatohmMetre` (statΩ · m), as an `ElectricalResistivity`.
    pub const STATOHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted((Exact::dec(299792458, -6) * Exact::int(2997924580)) * OHM_METRE::<Exact>);

    /// One `OhmInch` (Ω · in), as an `ElectricalResistivity`.
    pub const OHM_INCH<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::dec(254, -4) * OHM_METRE::<Exact>);

    /// One `OhmFoot` (Ω · ft), as an `ElectricalResistivity`.
    pub const OHM_FOOT<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::dec(3048, -4) * OHM_METRE::<Exact>);

    /// One `OhmYard` (Ω · yd), as an `ElectricalResistivity`.
    pub const OHM_YARD<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::dec(9144, -4) * OHM_METRE::<Exact>);

    /// One `OhmSquareMillimetrePerMetre` (Ω · mm²/m), as an `ElectricalResistivity`.
    pub const OHM_SQUARE_MILLIMETRE_PER_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::dec(1, -6) * OHM_METRE::<Exact>);

    /// One `Farad` (F), as a `Capacitance`.
    pub const FARAD<N: const From<Exact>>: Capacitance<N> = counted(SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * AMPERE::<Exact> * AMPERE::<Exact> / (METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact>));

    /// One `Abfarad` (abF), as a `Capacitance`.
    pub const ABFARAD<N: const From<Exact>>: Capacitance<N> = counted(Exact::int(1000000000) * FARAD::<Exact>);

    /// One `Statfarad` (statF), as a `Capacitance`.
    pub const STATFARAD<N: const From<Exact>>: Capacitance<N> = counted((Exact::int(1) / (Exact::dec(299792458, -6) * Exact::int(2997924580))) * FARAD::<Exact>);

    /// One `Henry` (H), as an `Inductance`.
    pub const HENRY<N: const From<Exact>>: Inductance<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * AMPERE::<Exact> * AMPERE::<Exact>));

    /// One `Abhenry` (abH), as an `Inductance`.
    pub const ABHENRY<N: const From<Exact>>: Inductance<N> = counted(Exact::dec(1, -9) * HENRY::<Exact>);

    /// One `Stathenry` (statH), as an `Inductance`.
    pub const STATHENRY<N: const From<Exact>>: Inductance<N> = counted((Exact::dec(299792458, -6) * Exact::int(2997924580)) * HENRY::<Exact>);

    /// One `AmperePerMetre` (A/m), as a `MagneticFieldStrength`.
    pub const MAGNETIC_FIELD_STRENGTH_AMPERE_PER_METRE<N: const From<Exact>>: MagneticFieldStrength<N> = counted(AMPERE::<Exact> / METRE::<Exact>);

    /// One `Oersted` (Oe), as a `MagneticFieldStrength`.
    pub const OERSTED<N: const From<Exact>>: MagneticFieldStrength<N> = counted((Exact::int(1000) / (Exact::int(4) * Exact::PI)) * MAGNETIC_FIELD_STRENGTH_AMPERE_PER_METRE::<Exact>);

    /// One `Weber` (Wb), as a `MagneticFlux`.
    pub const WEBER<N: const From<Exact>>: MagneticFlux<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * AMPERE::<Exact>));

    /// One `Maxwell` (Mx), as a `MagneticFlux`.
    pub const MAXWELL<N: const From<Exact>>: MagneticFlux<N> = counted(Exact::dec(1, -8) * WEBER::<Exact>);

    /// One `Tesla` (T), as a `MagneticFluxDensity`.
    pub const TESLA<N: const From<Exact>>: MagneticFluxDensity<N> = counted(KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * AMPERE::<Exact>));

    /// One `Gamma` (γ), as a `MagneticFluxDensity`.
    pub const GAMMA<N: const From<Exact>>: MagneticFluxDensity<N> = counted(Exact::dec(1, -9) * TESLA::<Exact>);

    /// One `Gauss` (G), as a `MagneticFluxDensity`.
    pub const GAUSS<N: const From<Exact>>: MagneticFluxDensity<N> = counted(Exact::dec(1, -4) * TESLA::<Exact>);

    /// One `AmpereSquareMetre` (A · m²), as a `MagneticMoment`.
    pub const AMPERE_SQUARE_METRE<N: const From<Exact>>: MagneticMoment<N> = counted(METRE::<Exact> * METRE::<Exact> * AMPERE::<Exact>);

    /// One `JoulePerTesla` (J/T), as a `MagneticMoment`.
    pub const JOULE_PER_TESLA<N: const From<Exact>>: MagneticMoment<N> = counted(AMPERE_SQUARE_METRE::<Exact>);

    /// One `NewtonMetrePerTesla` (N · m/T), as a `MagneticMoment`.
    pub const NEWTON_METRE_PER_TESLA<N: const From<Exact>>: MagneticMoment<N> = counted(AMPERE_SQUARE_METRE::<Exact>);

    /// One `ErgPerGauss` (erg/G), as a `MagneticMoment`.
    pub const ERG_PER_GAUSS<N: const From<Exact>>: MagneticMoment<N> = counted(Exact::dec(1, -3) * AMPERE_SQUARE_METRE::<Exact>);

    /// One `BohrMagneton` (µ(Bohr)), as a `MagneticMoment`.
    pub const BOHR_MAGNETON<N: const From<Exact>>: MagneticMoment<N> = counted(Exact::dec(92740100783, -34) * AMPERE_SQUARE_METRE::<Exact>);

    /// One `NuclearMagneton` (μ(Nuclear)), as a `MagneticMoment`.
    pub const NUCLEAR_MAGNETON<N: const From<Exact>>: MagneticMoment<N> = counted(Exact::dec(50507837461, -37) * AMPERE_SQUARE_METRE::<Exact>);

    /// One `AtomicUnitOfMagneticDipoleMoment` (ħ · e/mₑ), as a `MagneticMoment`.
    pub const ATOMIC_UNIT_OF_MAGNETIC_DIPOLE_MOMENT<N: const From<Exact>>: MagneticMoment<N> = counted(Exact::dec(185480201566, -34) * AMPERE_SQUARE_METRE::<Exact>);

    /// One `DeuteronMagneticMoment` (μ(deuteron)), as a `MagneticMoment`.
    pub const DEUTERON_MAGNETIC_MOMENT<N: const From<Exact>>: MagneticMoment<N> = counted(Exact::dec(4330735094, -36) * AMPERE_SQUARE_METRE::<Exact>);

    /// One `ProtonMagneticMoment` (μ(proton)), as a `MagneticMoment`.
    pub const PROTON_MAGNETIC_MOMENT<N: const From<Exact>>: MagneticMoment<N> = counted(Exact::dec(141060679736, -37) * AMPERE_SQUARE_METRE::<Exact>);

    /// One `ShieldedProtonMagneticMoment` (μ’(proton)), as a `MagneticMoment`.
    pub const SHIELDED_PROTON_MAGNETIC_MOMENT<N: const From<Exact>>: MagneticMoment<N> = counted(Exact::dec(141057056, -34) * AMPERE_SQUARE_METRE::<Exact>);

    /// One `TritonMagneticMoment` (μ(triton)), as a `MagneticMoment`.
    pub const TRITON_MAGNETIC_MOMENT<N: const From<Exact>>: MagneticMoment<N> = counted(Exact::dec(15046095202, -36) * AMPERE_SQUARE_METRE::<Exact>);

    /// One `HenryPerMetre` (H/m), as a `MagneticPermeability`.
    pub const HENRY_PER_METRE<N: const From<Exact>>: MagneticPermeability<N> = counted(METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * AMPERE::<Exact> * AMPERE::<Exact>));

    /// One `VacuumPermeability` (µ₀), as a `MagneticPermeability`.
    pub const VACUUM_PERMEABILITY<N: const From<Exact>>: MagneticPermeability<N> = counted(Exact::dec(125663706212, -17) * HENRY_PER_METRE::<Exact>);

    /// One `HertzPerTesla` (Hz/T), as a `GyromagneticRatio`.
    pub const HERTZ_PER_TESLA<N: const From<Exact>>: GyromagneticRatio<N> = counted(SECOND::<Exact> * AMPERE::<Exact> * RADIAN::<Exact> / KILOGRAM::<Exact>);

    /// One `RadianPerSecondTesla` (rad/(s · T)), as a `GyromagneticRatio`.
    pub const RADIAN_PER_SECOND_TESLA<N: const From<Exact>>: GyromagneticRatio<N> = counted(HERTZ_PER_TESLA::<Exact>);

    /// One `ProtonGyromagneticRatio` (γₚ), as a `GyromagneticRatio`.
    pub const PROTON_GYROMAGNETIC_RATIO<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::dec(26752218708, -2) * HERTZ_PER_TESLA::<Exact>);

    /// One `NeutronGyromagneticRatio` (γₙ), as a `GyromagneticRatio`.
    pub const NEUTRON_GYROMAGNETIC_RATIO<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::int(183247174) * HERTZ_PER_TESLA::<Exact>);

    /// One `ElectronGyromagneticRatio` (γₑ), as a `GyromagneticRatio`.
    pub const ELECTRON_GYROMAGNETIC_RATIO<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::int(176085962784) * HERTZ_PER_TESLA::<Exact>);

    /// One `ShieldedProtonGyromagneticRatio` (γ'ₚ), as a `GyromagneticRatio`.
    pub const SHIELDED_PROTON_GYROMAGNETIC_RATIO<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::dec(2675153194, -1) * HERTZ_PER_TESLA::<Exact>);

    /// One `ShieldedHelionGyromagneticRatio` (γ'ₕ), as a `GyromagneticRatio`.
    pub const SHIELDED_HELION_GYROMAGNETIC_RATIO<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::dec(20378946078, -2) * HERTZ_PER_TESLA::<Exact>);

    /// One `DeuteronGyromagneticRatio` (γ₂), as a `GyromagneticRatio`.
    pub const DEUTERON_GYROMAGNETIC_RATIO<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::dec(410662888, -1) * HERTZ_PER_TESLA::<Exact>);

    /// One `TritonGyromagneticRatio` (γ₃), as a `GyromagneticRatio`.
    pub const TRITON_GYROMAGNETIC_RATIO<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::int(285349844) * HERTZ_PER_TESLA::<Exact>);

    /// One `Lithium7GyromagneticRatio` (γ₇), as a `GyromagneticRatio`.
    pub const LITHIUM7_GYROMAGNETIC_RATIO<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::int(103975264) * HERTZ_PER_TESLA::<Exact>);

    /// One `Boron11GyromagneticRatio` (γ₁₁), as a `GyromagneticRatio`.
    pub const BORON11_GYROMAGNETIC_RATIO<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::dec(858384143, -1) * HERTZ_PER_TESLA::<Exact>);

    /// One `Carbon13GyromagneticRatio` (γ₁₃), as a `GyromagneticRatio`.
    pub const CARBON13_GYROMAGNETIC_RATIO<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::dec(672787546, -1) * HERTZ_PER_TESLA::<Exact>);

    /// One `Nitrogen14GyromagneticRatio` (γ₁₄), as a `GyromagneticRatio`.
    pub const NITROGEN14_GYROMAGNETIC_RATIO<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::dec(193287922, -1) * HERTZ_PER_TESLA::<Exact>);

    /// One `Flourine19GyromagneticRatio` (γ₁₉), as a `GyromagneticRatio`.
    pub const FLOURINE19_GYROMAGNETIC_RATIO<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::int(251762483) * HERTZ_PER_TESLA::<Exact>);

    /// One `Sodium23GyromagneticRatio` (γ₂₃), as a `GyromagneticRatio`.
    pub const SODIUM23_GYROMAGNETIC_RATIO<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::dec(708035417, -1) * HERTZ_PER_TESLA::<Exact>);

    /// One `Phosphorus31GyromagneticRatio` (γ₃₁), as a `GyromagneticRatio`.
    pub const PHOSPHORUS31_GYROMAGNETIC_RATIO<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::int(108329419) * HERTZ_PER_TESLA::<Exact>);

    /// One `ProtonReducedGyromagneticRatio` (̅γₚ), as a `ReducedGyromagneticRatio`.
    pub const PROTON_REDUCED_GYROMAGNETIC_RATIO<N: const From<Exact>>: ReducedGyromagneticRatio<N> = counted(Exact::dec(42577478461, -3) * (SECOND::<Exact> * AMPERE::<Exact> / KILOGRAM::<Exact>));

    /// One `NeutronReducedGyromagneticRatio` (̅γₙ), as a `ReducedGyromagneticRatio`.
    pub const NEUTRON_REDUCED_GYROMAGNETIC_RATIO<N: const From<Exact>>: ReducedGyromagneticRatio<N> = counted(Exact::dec(291646935, -1) * (SECOND::<Exact> * AMPERE::<Exact> / KILOGRAM::<Exact>));

    /// One `ElectronReducedGyromagneticRatio` (̅γₑ), as a `ReducedGyromagneticRatio`.
    pub const ELECTRON_REDUCED_GYROMAGNETIC_RATIO<N: const From<Exact>>: ReducedGyromagneticRatio<N> = counted(Exact::dec(280249513861, -1) * (SECOND::<Exact> * AMPERE::<Exact> / KILOGRAM::<Exact>));

    /// One `ShieldedProtonReducedGyromagneticRatio` (γ'ₚ), as a `ReducedGyromagneticRatio`.
    pub const SHIELDED_PROTON_REDUCED_GYROMAGNETIC_RATIO<N: const From<Exact>>: ReducedGyromagneticRatio<N> = counted(Exact::dec(4257638543, -2) * (SECOND::<Exact> * AMPERE::<Exact> / KILOGRAM::<Exact>));

    /// One `ShieldedHelionReducedGyromagneticRatio` (γ'ₕ), as a `ReducedGyromagneticRatio`.
    pub const SHIELDED_HELION_REDUCED_GYROMAGNETIC_RATIO<N: const From<Exact>>: ReducedGyromagneticRatio<N> = counted(Exact::dec(32434100033, -3) * (SECOND::<Exact> * AMPERE::<Exact> / KILOGRAM::<Exact>));

    /// One `DeuteronReducedGyromagneticRatio` (̅γ₂), as a `ReducedGyromagneticRatio`.
    pub const DEUTERON_REDUCED_GYROMAGNETIC_RATIO<N: const From<Exact>>: ReducedGyromagneticRatio<N> = counted(Exact::dec(653590285, -2) * (SECOND::<Exact> * AMPERE::<Exact> / KILOGRAM::<Exact>));

    /// One `TritonReducedGyromagneticRatio` (̅γ₃), as a `ReducedGyromagneticRatio`.
    pub const TRITON_REDUCED_GYROMAGNETIC_RATIO<N: const From<Exact>>: ReducedGyromagneticRatio<N> = counted(Exact::dec(454148381, -1) * (SECOND::<Exact> * AMPERE::<Exact> / KILOGRAM::<Exact>));

    /// One `Lithium7ReducedGyromagneticRatio` (̅γ₇), as a `ReducedGyromagneticRatio`.
    pub const LITHIUM7_REDUCED_GYROMAGNETIC_RATIO<N: const From<Exact>>: ReducedGyromagneticRatio<N> = counted(Exact::dec(165481773, -1) * (SECOND::<Exact> * AMPERE::<Exact> / KILOGRAM::<Exact>));

    /// One `Boron11ReducedGyromagneticRatio` (̅γ₁₁), as a `ReducedGyromagneticRatio`.
    pub const BORON11_REDUCED_GYROMAGNETIC_RATIO<N: const From<Exact>>: ReducedGyromagneticRatio<N> = counted(Exact::int(13661608) * (SECOND::<Exact> * AMPERE::<Exact> / KILOGRAM::<Exact>));

    /// One `Carbon13ReducedGyromagneticRatio` (̅γ₁₃), as a `ReducedGyromagneticRatio`.
    pub const CARBON13_REDUCED_GYROMAGNETIC_RATIO<N: const From<Exact>>: ReducedGyromagneticRatio<N> = counted(Exact::dec(107077464, -1) * (SECOND::<Exact> * AMPERE::<Exact> / KILOGRAM::<Exact>));

    /// One `Nitrogen14ReducedGyromagneticRatio` (̅γ₁₄), as a `ReducedGyromagneticRatio`.
    pub const NITROGEN14_REDUCED_GYROMAGNETIC_RATIO<N: const From<Exact>>: ReducedGyromagneticRatio<N> = counted(Exact::dec(307627281, -2) * (SECOND::<Exact> * AMPERE::<Exact> / KILOGRAM::<Exact>));

    /// One `Flourine19ReducedGyromagneticRatio` (̅γ₁₉), as a `ReducedGyromagneticRatio`.
    pub const FLOURINE19_REDUCED_GYROMAGNETIC_RATIO<N: const From<Exact>>: ReducedGyromagneticRatio<N> = counted(Exact::dec(400692437, -1) * (SECOND::<Exact> * AMPERE::<Exact> / KILOGRAM::<Exact>));

    /// One `Sodium23ReducedGyromagneticRatio` (̅γ₂₃), as a `ReducedGyromagneticRatio`.
    pub const SODIUM23_REDUCED_GYROMAGNETIC_RATIO<N: const From<Exact>>: ReducedGyromagneticRatio<N> = counted(Exact::dec(112687336, -1) * (SECOND::<Exact> * AMPERE::<Exact> / KILOGRAM::<Exact>));

    /// One `Phosphorus31ReducedGyromagneticRatio` (̅γ₃₁), as a `ReducedGyromagneticRatio`.
    pub const PHOSPHORUS31_REDUCED_GYROMAGNETIC_RATIO<N: const From<Exact>>: ReducedGyromagneticRatio<N> = counted(Exact::dec(172411625, -1) * (SECOND::<Exact> * AMPERE::<Exact> / KILOGRAM::<Exact>));
}

/// The units of `ElectricCharge`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCharge<f64>)]
pub enum ElectricChargeUnit {
    #[default]
    #[unit("C", "coulombs", by = COULOMB)]
    Coulomb,
    #[unit("A · h", "ampere hours", by = AMPERE_HOUR)]
    AmpereHour,
    #[unit("e", "elementary charges", by = ELEMENTARY_CHARGE)]
    ElementaryCharge,
    #[unit("a.u. of charge", "atomic units of charge", by = ATOMIC_UNIT_OF_CHARGE)]
    AtomicUnitOfCharge,
    #[unit("abC", "abcoulombs", by = ABCOULOMB)]
    Abcoulomb,
    #[unit("F", "faradays", by = FARADAY)]
    Faraday,
    #[unit("Fr", "franklins", by = FRANKLIN)]
    Franklin,
    #[unit("statC", "statcoulombs", by = STATCOULOMB)]
    Statcoulomb,
}

/// The units of `ElectricChargeArealDensity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricChargeArealDensity<f64>)]
pub enum ElectricChargeArealDensityUnit {
    #[default]
    #[unit("C/m²", "coulombs per square metre", by = ELECTRIC_CHARGE_AREAL_DENSITY_COULOMB_PER_SQUARE_METRE)]
    CoulombPerSquareMetre,
}

/// The units of `ElectricChargeLinearDensity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricChargeLinearDensity<f64>)]
pub enum ElectricChargeLinearDensityUnit {
    #[default]
    #[unit("C/m", "coulombs per metre", by = COULOMB_PER_METRE)]
    CoulombPerMetre,
}

/// The units of `ElectricChargeVolumetricDensity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricChargeVolumetricDensity<f64>)]
pub enum ElectricChargeVolumetricDensityUnit {
    #[default]
    #[unit("C/m³", "coulombs per cubic metre", by = COULOMB_PER_CUBIC_METRE)]
    CoulombPerCubicMetre,
}

/// The units of `ElectricCurrent`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCurrent<f64>)]
pub enum ElectricCurrentUnit {
    #[default]
    #[unit("A", "amperes", by = AMPERE)]
    Ampere,
    #[unit("e/s", "elementary charges per second", by = ELEMENTARY_CHARGE_PER_SECOND)]
    ElementaryChargePerSecond,
    #[unit("a.u. of charge/s", "atomic units of charge per second", by = ATOMIC_UNIT_OF_CHARGE_PER_SECOND)]
    AtomicUnitOfChargePerSecond,
    #[unit("abA", "abamperes", by = ABAMPERE)]
    Abampere,
    #[unit("Gi", "gilberts", by = GILBERT)]
    Gilbert,
    #[unit("statA", "statamperes", by = STATAMPERE)]
    Statampere,
}

/// The units of `ElectricCurrentDensity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCurrentDensity<f64>)]
pub enum ElectricCurrentDensityUnit {
    #[default]
    #[unit("A/m²", "amperes per square metre", by = AMPERE_PER_SQUARE_METRE)]
    AmperePerSquareMetre,
}

/// The units of `SurfaceElectricCurrentDensity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SurfaceElectricCurrentDensity<f64>)]
pub enum SurfaceElectricCurrentDensityUnit {
    #[default]
    #[unit("A/m", "amperes per metre", by = SURFACE_ELECTRIC_CURRENT_DENSITY_AMPERE_PER_METRE)]
    AmperePerMetre,
}

/// The units of `ElectricPotential`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricPotential<f64>)]
pub enum ElectricPotentialUnit {
    #[default]
    #[unit("V", "volts", by = VOLT)]
    Volt,
    #[unit("abV", "abvolts", by = ABVOLT)]
    Abvolt,
    #[unit("statV", "statvolts", by = STATVOLT)]
    Statvolt,
}

/// The units of `ElectricField`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricField<f64>)]
pub enum ElectricFieldUnit {
    #[default]
    #[unit("V/m", "volts per metre", by = VOLT_PER_METRE)]
    VoltPerMetre,
    #[unit("V/mil", "volts per mil", by = VOLT_PER_MIL)]
    VoltPerMil,
    #[unit("a.u. of electric field", "atomic units of electric field", by = ATOMIC_UNIT_OF_ELECTRIC_FIELD)]
    AtomicUnitOfElectricField,
}

/// The units of `ElectricDisplacementField`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricDisplacementField<f64>)]
pub enum ElectricDisplacementFieldUnit {
    #[default]
    #[unit("C/m²", "coulombs per square metre", by = ELECTRIC_DISPLACEMENT_FIELD_COULOMB_PER_SQUARE_METRE)]
    CoulombPerSquareMetre,
}

/// The units of `ElectricFlux`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricFlux<f64>)]
pub enum ElectricFluxUnit {
    #[default]
    #[unit("V ⋅ m", "volt metres", by = VOLT_METRE)]
    VoltMetre,
}

/// The units of `ElectricDipoleMoment`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricDipoleMoment<f64>)]
pub enum ElectricDipoleMomentUnit {
    #[default]
    #[unit("C · m", "coulomb metres", by = COULOMB_METRE)]
    CoulombMetre,
    #[unit("a.u. of charge · cm", "atomic unit of charge centimetres", by = ATOMIC_UNIT_OF_CHARGE_CENTIMETRE)]
    AtomicUnitOfChargeCentimetre,
    #[unit("e · cm", "elementary charge centimetres", by = ELEMENTARY_CHARGE_CENTIMETRE)]
    ElementaryChargeCentimetre,
    #[unit("e · a₀", "atomic units of electric dipole moment", by = ATOMIC_UNIT_OF_ELECTRIC_DIPOLE_MOMENT)]
    AtomicUnitOfElectricDipoleMoment,
}

/// The units of `ElectricQuadrupoleMoment`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricQuadrupoleMoment<f64>)]
pub enum ElectricQuadrupoleMomentUnit {
    #[default]
    #[unit("C · m²", "coulomb square metres", by = COULOMB_SQUARE_METRE)]
    CoulombSquareMetre,
    #[unit("e · b", "elementary charge barns", by = ELEMENTARY_CHARGE_BARN)]
    ElementaryChargeBarn,
    #[unit("e · a₀²", "atomic units of electric quadrupole moment", by = ATOMIC_UNIT_OF_ELECTRIC_QUADRUPOLE_MOMENT)]
    AtomicUnitOfElectricQuadrupoleMoment,
}

/// The units of `ElectricPermittivity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricPermittivity<f64>)]
pub enum ElectricPermittivityUnit {
    #[default]
    #[unit("F/m", "farads per metre", by = FARAD_PER_METRE)]
    FaradPerMetre,
    #[unit("ε₀", "vacuum electric permittivity", by = VACUUM_PERMITTIVITY)]
    VacuumPermittivity,
}

/// The units of `ElectricalConductance`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricalConductance<f64>)]
pub enum ElectricalConductanceUnit {
    #[default]
    #[unit("S", "siemens", by = SIEMENS)]
    Siemens,
    #[unit("℧", "mhos", by = MHO)]
    Mho,
    #[unit("abmho", "abmhos", by = ABMHO)]
    Abmho,
    #[unit("abS", "abmsiemens", by = ABSIEMENS)]
    Absiemens,
    #[unit("statS", "statsiemens", by = STATSIEMENS)]
    Statsiemens,
    #[unit("statmho", "statmhos", by = STATMHO)]
    Statmho,
}

/// The units of `ElectricalConductivity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricalConductivity<f64>)]
pub enum ElectricalConductivityUnit {
    #[default]
    #[unit("S/m", "siemens per metre", by = SIEMENS_PER_METRE)]
    SiemensPerMetre,
}

/// The units of `ElectricalMobility`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricalMobility<f64>)]
pub enum ElectricalMobilityUnit {
    #[default]
    #[unit("m²/(V · s)", "square metres per volt second", by = SQUARE_METRE_PER_VOLT_SECOND)]
    SquareMetrePerVoltSecond,
}

/// The units of `ElectricalResistance`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricalResistance<f64>)]
pub enum ElectricalResistanceUnit {
    #[default]
    #[unit("Ω", "ohms", by = OHM)]
    Ohm,
    #[unit("abΩ", "abohms", by = ABOHM)]
    Abohm,
    #[unit("statΩ", "statohms", by = STATOHM)]
    Statohm,
}

/// The units of `ElectricalResistivity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricalResistivity<f64>)]
pub enum ElectricalResistivityUnit {
    #[default]
    #[unit("Ω · m", "ohm metres", by = OHM_METRE)]
    OhmMetre,
    #[unit("abΩ · m", "abohm metres", by = ABOHM_METRE)]
    AbohmMetre,
    #[unit("statΩ · m", "statohm metres", by = STATOHM_METRE)]
    StatohmMetre,
    #[unit("Ω · in", "ohm inches", by = OHM_INCH)]
    OhmInch,
    #[unit("Ω · ft", "ohm feet", by = OHM_FOOT)]
    OhmFoot,
    #[unit("Ω · yd", "ohm yards", by = OHM_YARD)]
    OhmYard,
    #[unit("Ω · mm²/m", "ohm square millimetres per metre", by = OHM_SQUARE_MILLIMETRE_PER_METRE)]
    OhmSquareMillimetrePerMetre,
}

/// The units of `Capacitance`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Capacitance<f64>)]
pub enum CapacitanceUnit {
    #[default]
    #[unit("F", "farads", by = FARAD)]
    Farad,
    #[unit("abF", "abfarads", by = ABFARAD)]
    Abfarad,
    #[unit("statF", "statfarads", by = STATFARAD)]
    Statfarad,
}

/// The units of `Inductance`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Inductance<f64>)]
pub enum InductanceUnit {
    #[default]
    #[unit("H", "henries", by = HENRY)]
    Henry,
    #[unit("abH", "abhenries", by = ABHENRY)]
    Abhenry,
    #[unit("statH", "stathenries", by = STATHENRY)]
    Stathenry,
}

/// The units of `MagneticFieldStrength`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticFieldStrength<f64>)]
pub enum MagneticFieldStrengthUnit {
    #[default]
    #[unit("A/m", "amperes per metre", by = MAGNETIC_FIELD_STRENGTH_AMPERE_PER_METRE)]
    AmperePerMetre,
    #[unit("Oe", "oersteds", by = OERSTED)]
    Oersted,
}

/// The units of `MagneticFlux`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticFlux<f64>)]
pub enum MagneticFluxUnit {
    #[default]
    #[unit("Wb", "webers", by = WEBER)]
    Weber,
    #[unit("Mx", "maxwells", by = MAXWELL)]
    Maxwell,
}

/// The units of `MagneticFluxDensity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticFluxDensity<f64>)]
pub enum MagneticFluxDensityUnit {
    #[default]
    #[unit("T", "teslas", by = TESLA)]
    Tesla,
    #[unit("γ", "gammas", by = GAMMA)]
    Gamma,
    #[unit("G", "gauss", by = GAUSS)]
    Gauss,
}

/// The units of `MagneticMoment`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticMoment<f64>)]
pub enum MagneticMomentUnit {
    #[default]
    #[unit("A · m²", "ampere square metres", by = AMPERE_SQUARE_METRE)]
    AmpereSquareMetre,
    #[unit("J/T", "joules per tesla", by = JOULE_PER_TESLA)]
    JoulePerTesla,
    #[unit("N · m/T", "newton metres per tesla", by = NEWTON_METRE_PER_TESLA)]
    NewtonMetrePerTesla,
    #[unit("erg/G", "ergs per gauss", by = ERG_PER_GAUSS)]
    ErgPerGauss,
    #[unit("µ(Bohr)", "Bohr magnetons", by = BOHR_MAGNETON)]
    BohrMagneton,
    #[unit("μ(Nuclear)", "nuclear magnetons", by = NUCLEAR_MAGNETON)]
    NuclearMagneton,
    #[unit("ħ · e/mₑ", " atomic units of magnetic dipole moment", by = ATOMIC_UNIT_OF_MAGNETIC_DIPOLE_MOMENT)]
    AtomicUnitOfMagneticDipoleMoment,
    #[unit("μ(deuteron)", "deuteron magnetic moments", by = DEUTERON_MAGNETIC_MOMENT)]
    DeuteronMagneticMoment,
    #[unit("μ(proton)", "proton magnetic moments", by = PROTON_MAGNETIC_MOMENT)]
    ProtonMagneticMoment,
    #[unit("μ’(proton)", "shielded proton magnetic moments", by = SHIELDED_PROTON_MAGNETIC_MOMENT)]
    ShieldedProtonMagneticMoment,
    #[unit("μ(triton)", "triton magnetic moments", by = TRITON_MAGNETIC_MOMENT)]
    TritonMagneticMoment,
}

/// The units of `MagneticPermeability`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticPermeability<f64>)]
pub enum MagneticPermeabilityUnit {
    #[default]
    #[unit("H/m", "henrys per metre", by = HENRY_PER_METRE)]
    HenryPerMetre,
    #[unit("µ₀", "vacuum magnetic permeability", by = VACUUM_PERMEABILITY)]
    VacuumPermeability,
}

/// The units of `GyromagneticRatio`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = GyromagneticRatio<f64>)]
pub enum GyromagneticRatioUnit {
    #[default]
    #[unit("Hz/T", "hertz per tesla", by = HERTZ_PER_TESLA)]
    HertzPerTesla,
    #[unit("rad/(s · T)", "radians per second tesla", by = RADIAN_PER_SECOND_TESLA)]
    RadianPerSecondTesla,
    #[unit("γₚ", "proton gyromagnetic ratio", by = PROTON_GYROMAGNETIC_RATIO)]
    ProtonGyromagneticRatio,
    #[unit("γₙ", "neutron gyromagnetic ratio", by = NEUTRON_GYROMAGNETIC_RATIO)]
    NeutronGyromagneticRatio,
    #[unit("γₑ", "electron gyromagnetic ratio", by = ELECTRON_GYROMAGNETIC_RATIO)]
    ElectronGyromagneticRatio,
    #[unit("γ'ₚ", "shielded proton gyromagnetic ratio", by = SHIELDED_PROTON_GYROMAGNETIC_RATIO)]
    ShieldedProtonGyromagneticRatio,
    #[unit("γ'ₕ", "shielded helion gyromagnetic ratio", by = SHIELDED_HELION_GYROMAGNETIC_RATIO)]
    ShieldedHelionGyromagneticRatio,
    #[unit("γ₂", "deuteron gyromagnetic ratio", by = DEUTERON_GYROMAGNETIC_RATIO)]
    DeuteronGyromagneticRatio,
    #[unit("γ₃", "triton gyromagnetic ratio", by = TRITON_GYROMAGNETIC_RATIO)]
    TritonGyromagneticRatio,
    #[unit("γ₇", "lithium7 gyromagnetic ratio", by = LITHIUM7_GYROMAGNETIC_RATIO)]
    Lithium7GyromagneticRatio,
    #[unit("γ₁₁", "boron11 gyromagnetic ratio", by = BORON11_GYROMAGNETIC_RATIO)]
    Boron11GyromagneticRatio,
    #[unit("γ₁₃", "carbon13 gyromagnetic ratio", by = CARBON13_GYROMAGNETIC_RATIO)]
    Carbon13GyromagneticRatio,
    #[unit("γ₁₄", "nitrogen14 gyromagnetic ratio", by = NITROGEN14_GYROMAGNETIC_RATIO)]
    Nitrogen14GyromagneticRatio,
    #[unit("γ₁₉", "flourine19 gyromagnetic ratio", by = FLOURINE19_GYROMAGNETIC_RATIO)]
    Flourine19GyromagneticRatio,
    #[unit("γ₂₃", "sodium23 gyromagnetic ratio", by = SODIUM23_GYROMAGNETIC_RATIO)]
    Sodium23GyromagneticRatio,
    #[unit("γ₃₁", "phosphorus31 gyromagnetic ratio", by = PHOSPHORUS31_GYROMAGNETIC_RATIO)]
    Phosphorus31GyromagneticRatio,
}

/// The units of `ReducedGyromagneticRatio`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ReducedGyromagneticRatio<f64>)]
pub enum ReducedGyromagneticRatioUnit {
    #[default]
    #[unit("̅γₚ", "proton reduced gyromagnetic ratio", by = PROTON_REDUCED_GYROMAGNETIC_RATIO)]
    ProtonReducedGyromagneticRatio,
    #[unit("̅γₙ", "neutron reduced gyromagnetic ratio", by = NEUTRON_REDUCED_GYROMAGNETIC_RATIO)]
    NeutronReducedGyromagneticRatio,
    #[unit("̅γₑ", "electron reduced gyromagnetic ratio", by = ELECTRON_REDUCED_GYROMAGNETIC_RATIO)]
    ElectronReducedGyromagneticRatio,
    #[unit("γ'ₚ", "shielded proton reduced gyromagnetic ratio", by = SHIELDED_PROTON_REDUCED_GYROMAGNETIC_RATIO)]
    ShieldedProtonReducedGyromagneticRatio,
    #[unit("γ'ₕ", "shielded helion reduced gyromagnetic ratio", by = SHIELDED_HELION_REDUCED_GYROMAGNETIC_RATIO)]
    ShieldedHelionReducedGyromagneticRatio,
    #[unit("̅γ₂", "deuteron reduced gyromagnetic ratio", by = DEUTERON_REDUCED_GYROMAGNETIC_RATIO)]
    DeuteronReducedGyromagneticRatio,
    #[unit("̅γ₃", "triton reduced gyromagnetic ratio", by = TRITON_REDUCED_GYROMAGNETIC_RATIO)]
    TritonReducedGyromagneticRatio,
    #[unit("̅γ₇", "lithium7 reduced gyromagnetic ratio", by = LITHIUM7_REDUCED_GYROMAGNETIC_RATIO)]
    Lithium7ReducedGyromagneticRatio,
    #[unit("̅γ₁₁", "boron11 reduced gyromagnetic ratio", by = BORON11_REDUCED_GYROMAGNETIC_RATIO)]
    Boron11ReducedGyromagneticRatio,
    #[unit("̅γ₁₃", "carbon13 reduced gyromagnetic ratio", by = CARBON13_REDUCED_GYROMAGNETIC_RATIO)]
    Carbon13ReducedGyromagneticRatio,
    #[unit("̅γ₁₄", "nitrogen14 reduced gyromagnetic ratio", by = NITROGEN14_REDUCED_GYROMAGNETIC_RATIO)]
    Nitrogen14ReducedGyromagneticRatio,
    #[unit("̅γ₁₉", "flourine19 reduced gyromagnetic ratio", by = FLOURINE19_REDUCED_GYROMAGNETIC_RATIO)]
    Flourine19ReducedGyromagneticRatio,
    #[unit("̅γ₂₃", "sodium23 reduced gyromagnetic ratio", by = SODIUM23_REDUCED_GYROMAGNETIC_RATIO)]
    Sodium23ReducedGyromagneticRatio,
    #[unit("̅γ₃₁", "phosphorus31 reduced gyromagnetic ratio", by = PHOSPHORUS31_REDUCED_GYROMAGNETIC_RATIO)]
    Phosphorus31ReducedGyromagneticRatio,
}
