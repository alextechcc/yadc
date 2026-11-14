use crate::{ISQ, SIQuantity};
use yadc::Dimension;

/// The dimension of `ElectricCharge`.
pub const ELECTRIC_CHARGE: ISQ = ISQ {
    s: 1,
    A: 1,
    ..ISQ::DIMENSIONLESS
};
pub type ElectricCharge<N> = SIQuantity<ELECTRIC_CHARGE, N>;

/// The dimension of `ElectricChargeArealDensity`.
pub const ELECTRIC_CHARGE_AREAL_DENSITY: ISQ = ISQ {
    m: -2,
    s: 1,
    A: 1,
    ..ISQ::DIMENSIONLESS
};
pub type ElectricChargeArealDensity<N> = SIQuantity<ELECTRIC_CHARGE_AREAL_DENSITY, N>;

/// The dimension of `ElectricChargeLinearDensity`.
pub const ELECTRIC_CHARGE_LINEAR_DENSITY: ISQ = ISQ {
    m: -1,
    s: 1,
    A: 1,
    ..ISQ::DIMENSIONLESS
};
pub type ElectricChargeLinearDensity<N> = SIQuantity<ELECTRIC_CHARGE_LINEAR_DENSITY, N>;

/// The dimension of `ElectricChargeVolumetricDensity`.
pub const ELECTRIC_CHARGE_VOLUMETRIC_DENSITY: ISQ = ISQ {
    m: -3,
    s: 1,
    A: 1,
    ..ISQ::DIMENSIONLESS
};
pub type ElectricChargeVolumetricDensity<N> = SIQuantity<ELECTRIC_CHARGE_VOLUMETRIC_DENSITY, N>;

/// The dimension of `ElectricCurrent`.
pub const ELECTRIC_CURRENT: ISQ = ISQ {
    A: 1,
    ..ISQ::DIMENSIONLESS
};
pub type ElectricCurrent<N> = SIQuantity<ELECTRIC_CURRENT, N>;

/// The dimension of `ElectricCurrentDensity`.
pub const ELECTRIC_CURRENT_DENSITY: ISQ = ISQ {
    m: -2,
    A: 1,
    ..ISQ::DIMENSIONLESS
};
pub type ElectricCurrentDensity<N> = SIQuantity<ELECTRIC_CURRENT_DENSITY, N>;

/// The dimension of `SurfaceElectricCurrentDensity`.
pub const SURFACE_ELECTRIC_CURRENT_DENSITY: ISQ = ISQ {
    m: -1,
    A: 1,
    ..ISQ::DIMENSIONLESS
};
pub type SurfaceElectricCurrentDensity<N> = SIQuantity<SURFACE_ELECTRIC_CURRENT_DENSITY, N>;

/// The dimension of `ElectricPotential`.
pub const ELECTRIC_POTENTIAL: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -3,
    A: -1,
    ..ISQ::DIMENSIONLESS
};
pub type ElectricPotential<N> = SIQuantity<ELECTRIC_POTENTIAL, N>;

/// The dimension of `ElectricField`.
pub const ELECTRIC_FIELD: ISQ = ISQ {
    m: 1,
    kg: 1,
    s: -3,
    A: -1,
    ..ISQ::DIMENSIONLESS
};
pub type ElectricField<N> = SIQuantity<ELECTRIC_FIELD, N>;

/// The dimension of `ElectricDisplacementField`.
pub const ELECTRIC_DISPLACEMENT_FIELD: ISQ = ISQ {
    m: -2,
    s: 1,
    A: 1,
    ..ISQ::DIMENSIONLESS
};
pub type ElectricDisplacementField<N> = SIQuantity<ELECTRIC_DISPLACEMENT_FIELD, N>;

/// The dimension of `ElectricFlux`.
pub const ELECTRIC_FLUX: ISQ = ISQ {
    m: 3,
    kg: 1,
    s: -3,
    A: -1,
    ..ISQ::DIMENSIONLESS
};
pub type ElectricFlux<N> = SIQuantity<ELECTRIC_FLUX, N>;

/// The dimension of `ElectricDipoleMoment`.
pub const ELECTRIC_DIPOLE_MOMENT: ISQ = ISQ {
    m: 1,
    s: 1,
    A: 1,
    ..ISQ::DIMENSIONLESS
};
pub type ElectricDipoleMoment<N> = SIQuantity<ELECTRIC_DIPOLE_MOMENT, N>;

/// The dimension of `ElectricQuadrupoleMoment`.
pub const ELECTRIC_QUADRUPOLE_MOMENT: ISQ = ISQ {
    m: 2,
    s: 1,
    A: 1,
    ..ISQ::DIMENSIONLESS
};
pub type ElectricQuadrupoleMoment<N> = SIQuantity<ELECTRIC_QUADRUPOLE_MOMENT, N>;

/// The dimension of `ElectricPermittivity`.
pub const ELECTRIC_PERMITTIVITY: ISQ = ISQ {
    m: -3,
    kg: -1,
    s: 4,
    A: 2,
    ..ISQ::DIMENSIONLESS
};
pub type ElectricPermittivity<N> = SIQuantity<ELECTRIC_PERMITTIVITY, N>;

/// The dimension of `ElectricalConductance`.
pub const ELECTRICAL_CONDUCTANCE: ISQ = ISQ {
    m: -2,
    kg: -1,
    s: 3,
    A: 2,
    ..ISQ::DIMENSIONLESS
};
pub type ElectricalConductance<N> = SIQuantity<ELECTRICAL_CONDUCTANCE, N>;

/// The dimension of `ElectricalConductivity`.
pub const ELECTRICAL_CONDUCTIVITY: ISQ = ISQ {
    m: -3,
    kg: -1,
    s: 3,
    A: 2,
    ..ISQ::DIMENSIONLESS
};
pub type ElectricalConductivity<N> = SIQuantity<ELECTRICAL_CONDUCTIVITY, N>;

/// The dimension of `ElectricalMobility`.
pub const ELECTRICAL_MOBILITY: ISQ = ISQ {
    kg: -1,
    s: 2,
    A: 1,
    ..ISQ::DIMENSIONLESS
};
pub type ElectricalMobility<N> = SIQuantity<ELECTRICAL_MOBILITY, N>;

/// The dimension of `ElectricalResistance`.
pub const ELECTRICAL_RESISTANCE: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -3,
    A: -2,
    ..ISQ::DIMENSIONLESS
};
pub type ElectricalResistance<N> = SIQuantity<ELECTRICAL_RESISTANCE, N>;

/// The dimension of `ElectricalResistivity`.
pub const ELECTRICAL_RESISTIVITY: ISQ = ISQ {
    m: 3,
    kg: 1,
    s: -3,
    A: -2,
    ..ISQ::DIMENSIONLESS
};
pub type ElectricalResistivity<N> = SIQuantity<ELECTRICAL_RESISTIVITY, N>;

/// The dimension of `Capacitance`.
pub const CAPACITANCE: ISQ = ISQ {
    m: -2,
    kg: -1,
    s: 4,
    A: 2,
    ..ISQ::DIMENSIONLESS
};
pub type Capacitance<N> = SIQuantity<CAPACITANCE, N>;

/// The dimension of `Inductance`.
pub const INDUCTANCE: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -2,
    A: -2,
    ..ISQ::DIMENSIONLESS
};
pub type Inductance<N> = SIQuantity<INDUCTANCE, N>;

/// The dimension of `MagneticFieldStrength`.
pub const MAGNETIC_FIELD_STRENGTH: ISQ = ISQ {
    m: -1,
    A: 1,
    ..ISQ::DIMENSIONLESS
};
pub type MagneticFieldStrength<N> = SIQuantity<MAGNETIC_FIELD_STRENGTH, N>;

/// The dimension of `MagneticFlux`.
pub const MAGNETIC_FLUX: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -2,
    A: -1,
    ..ISQ::DIMENSIONLESS
};
pub type MagneticFlux<N> = SIQuantity<MAGNETIC_FLUX, N>;

/// The dimension of `MagneticFluxDensity`.
pub const MAGNETIC_FLUX_DENSITY: ISQ = ISQ {
    kg: 1,
    s: -2,
    A: -1,
    ..ISQ::DIMENSIONLESS
};
pub type MagneticFluxDensity<N> = SIQuantity<MAGNETIC_FLUX_DENSITY, N>;

/// The dimension of `MagneticMoment`.
pub const MAGNETIC_MOMENT: ISQ = ISQ {
    m: 2,
    A: 1,
    ..ISQ::DIMENSIONLESS
};
pub type MagneticMoment<N> = SIQuantity<MAGNETIC_MOMENT, N>;

/// The dimension of `MagneticPermeability`.
pub const MAGNETIC_PERMEABILITY: ISQ = ISQ {
    m: 1,
    kg: 1,
    s: -2,
    A: -2,
    ..ISQ::DIMENSIONLESS
};
pub type MagneticPermeability<N> = SIQuantity<MAGNETIC_PERMEABILITY, N>;

/// The dimension of `GyromagneticRatio`.
pub const GYROMAGNETIC_RATIO: ISQ = ISQ {
    kg: -1,
    s: 1,
    A: 1,
    rad: 1,
    ..ISQ::DIMENSIONLESS
};
pub type GyromagneticRatio<N> = SIQuantity<GYROMAGNETIC_RATIO, N>;

/// The dimension of `ReducedGyromagneticRatio`.
pub const REDUCED_GYROMAGNETIC_RATIO: ISQ = ISQ {
    kg: -1,
    s: 1,
    A: 1,
    ..ISQ::DIMENSIONLESS
};
pub type ReducedGyromagneticRatio<N> = SIQuantity<REDUCED_GYROMAGNETIC_RATIO, N>;

/// The units of this module's quantities: a type each, its size and zero
/// in its quantity's coherent unit, exact (`Mile {}.quantity(3.0_f64)`).
pub mod units {
    use super::*;
    use crate::{Exact, SIUnit, SIUnitConsts};

    /// Coulombs (`C`), a unit of `ElectricCharge`: `n` of them is `n · 1.0` coherent units.
    pub type Coulomb = SIUnit<
        ELECTRIC_CHARGE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "C",
                name: "coulombs",
            }
        },
    >;

    /// Ampere hours (`A · h`), a unit of `ElectricCharge`: `n` of them is `n · 3600.0` coherent units.
    pub type AmpereHour = SIUnit<
        ELECTRIC_CHARGE,
        {
            SIUnitConsts {
                size: Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "A · h",
                name: "ampere hours",
            }
        },
    >;

    /// Elementary charges (`e`), a unit of `ElectricCharge`: `n` of them is `n · 1.602176634e-19` coherent units.
    pub type ElementaryCharge = SIUnit<
        ELECTRIC_CHARGE,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28),
                zero: Exact::ZERO,
                symbol: "e",
                name: "elementary charges",
            }
        },
    >;

    /// Atomic units of charge (`a.u. of charge`), a unit of `ElectricCharge`: `n` of them is `n · 1.602176634e-19` coherent units.
    pub type AtomicUnitOfCharge = SIUnit<
        ELECTRIC_CHARGE,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28),
                zero: Exact::ZERO,
                symbol: "a.u. of charge",
                name: "atomic units of charge",
            }
        },
    >;

    /// Abcoulombs (`abC`), a unit of `ElectricCharge`: `n` of them is `n · 10.0` coherent units.
    pub type Abcoulomb = SIUnit<
        ELECTRIC_CHARGE,
        {
            SIUnitConsts {
                size: Exact::int(10),
                zero: Exact::ZERO,
                symbol: "abC",
                name: "abcoulombs",
            }
        },
    >;

    /// Faradays (`F`), a unit of `ElectricCharge`: `n` of them is `n · 1.602176634e-19 * 6.02214076e23` coherent units.
    pub type Faraday = SIUnit<
        ELECTRIC_CHARGE,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28) * Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "F",
                name: "faradays",
            }
        },
    >;

    /// Franklins (`Fr`), a unit of `ElectricCharge`: `n` of them is `n · 1.0 / 2997924580.0` coherent units.
    pub type Franklin = SIUnit<
        ELECTRIC_CHARGE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(2997924580),
                zero: Exact::ZERO,
                symbol: "Fr",
                name: "franklins",
            }
        },
    >;

    /// Statcoulombs (`statC`), a unit of `ElectricCharge`: `n` of them is `n · 1.0 / 2997924580.0` coherent units.
    pub type Statcoulomb = SIUnit<
        ELECTRIC_CHARGE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(2997924580),
                zero: Exact::ZERO,
                symbol: "statC",
                name: "statcoulombs",
            }
        },
    >;

    /// Coulombs per square metre (`C/m²`), a unit of `ElectricChargeArealDensity`: `n` of them is `n · 1.0` coherent units.
    pub type ElectricChargeArealDensityCoulombPerSquareMetre = SIUnit<
        ELECTRIC_CHARGE_AREAL_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "C/m²",
                name: "coulombs per square metre",
            }
        },
    >;

    /// Coulombs per metre (`C/m`), a unit of `ElectricChargeLinearDensity`: `n` of them is `n · 1.0` coherent units.
    pub type CoulombPerMetre = SIUnit<
        ELECTRIC_CHARGE_LINEAR_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "C/m",
                name: "coulombs per metre",
            }
        },
    >;

    /// Coulombs per cubic metre (`C/m³`), a unit of `ElectricChargeVolumetricDensity`: `n` of them is `n · 1.0` coherent units.
    pub type CoulombPerCubicMetre = SIUnit<
        ELECTRIC_CHARGE_VOLUMETRIC_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "C/m³",
                name: "coulombs per cubic metre",
            }
        },
    >;

    /// Amperes (`A`), a unit of `ElectricCurrent`: `n` of them is `n · 1.0` coherent units.
    pub type Ampere = SIUnit<
        ELECTRIC_CURRENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "A",
                name: "amperes",
            }
        },
    >;

    /// Elementary charges per second (`e/s`), a unit of `ElectricCurrent`: `n` of them is `n · 1.602176634e-19` coherent units.
    pub type ElementaryChargePerSecond = SIUnit<
        ELECTRIC_CURRENT,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28),
                zero: Exact::ZERO,
                symbol: "e/s",
                name: "elementary charges per second",
            }
        },
    >;

    /// Atomic units of charge per second (`a.u. of charge/s`), a unit of `ElectricCurrent`: `n` of them is `n · 1.602176634e-19` coherent units.
    pub type AtomicUnitOfChargePerSecond = SIUnit<
        ELECTRIC_CURRENT,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28),
                zero: Exact::ZERO,
                symbol: "a.u. of charge/s",
                name: "atomic units of charge per second",
            }
        },
    >;

    /// Abamperes (`abA`), a unit of `ElectricCurrent`: `n` of them is `n · 10.0` coherent units.
    pub type Abampere = SIUnit<
        ELECTRIC_CURRENT,
        {
            SIUnitConsts {
                size: Exact::int(10),
                zero: Exact::ZERO,
                symbol: "abA",
                name: "abamperes",
            }
        },
    >;

    /// Gilberts (`Gi`), a unit of `ElectricCurrent`: `n` of them is `n · 10.0 / (4.0 * core::f64::consts::PI)` coherent units.
    pub type Gilbert = SIUnit<
        ELECTRIC_CURRENT,
        {
            SIUnitConsts {
                size: Exact::int(10) / (Exact::int(4) * Exact::PI),
                zero: Exact::ZERO,
                symbol: "Gi",
                name: "gilberts",
            }
        },
    >;

    /// Statamperes (`statA`), a unit of `ElectricCurrent`: `n` of them is `n · 1.0 / 2997924580.0` coherent units.
    pub type Statampere = SIUnit<
        ELECTRIC_CURRENT,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(2997924580),
                zero: Exact::ZERO,
                symbol: "statA",
                name: "statamperes",
            }
        },
    >;

    /// Amperes per square metre (`A/m²`), a unit of `ElectricCurrentDensity`: `n` of them is `n · 1.0` coherent units.
    pub type AmperePerSquareMetre = SIUnit<
        ELECTRIC_CURRENT_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "A/m²",
                name: "amperes per square metre",
            }
        },
    >;

    /// Amperes per metre (`A/m`), a unit of `SurfaceElectricCurrentDensity`: `n` of them is `n · 1.0` coherent units.
    pub type SurfaceElectricCurrentDensityAmperePerMetre = SIUnit<
        SURFACE_ELECTRIC_CURRENT_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "A/m",
                name: "amperes per metre",
            }
        },
    >;

    /// Volts (`V`), a unit of `ElectricPotential`: `n` of them is `n · 1.0` coherent units.
    pub type Volt = SIUnit<
        ELECTRIC_POTENTIAL,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "V",
                name: "volts",
            }
        },
    >;

    /// Abvolts (`abV`), a unit of `ElectricPotential`: `n` of them is `n · 1e-08` coherent units.
    pub type Abvolt = SIUnit<
        ELECTRIC_POTENTIAL,
        {
            SIUnitConsts {
                size: Exact::dec(1, -8),
                zero: Exact::ZERO,
                symbol: "abV",
                name: "abvolts",
            }
        },
    >;

    /// Statvolts (`statV`), a unit of `ElectricPotential`: `n` of them is `n · 299.792458` coherent units.
    pub type Statvolt = SIUnit<
        ELECTRIC_POTENTIAL,
        {
            SIUnitConsts {
                size: Exact::dec(299792458, -6),
                zero: Exact::ZERO,
                symbol: "statV",
                name: "statvolts",
            }
        },
    >;

    /// Volts per metre (`V/m`), a unit of `ElectricField`: `n` of them is `n · 1.0` coherent units.
    pub type VoltPerMetre = SIUnit<
        ELECTRIC_FIELD,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "V/m",
                name: "volts per metre",
            }
        },
    >;

    /// Volts per mil (`V/mil`), a unit of `ElectricField`: `n` of them is `n · 1.0 / 2.54e-5` coherent units.
    pub type VoltPerMil = SIUnit<
        ELECTRIC_FIELD,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(254, -7),
                zero: Exact::ZERO,
                symbol: "V/mil",
                name: "volts per mil",
            }
        },
    >;

    /// Atomic units of electric field (`a.u. of electric field`), a unit of `ElectricField`: `n` of them is `n · 4.3597447222071e-18 / (1.602176634e-19 * 5.29177210903e-11)` coherent units.
    pub type AtomicUnitOfElectricField = SIUnit<
        ELECTRIC_FIELD,
        {
            SIUnitConsts {
                size: Exact::dec(43597447222071, -31)
                    / (Exact::dec(1602176634, -28) * Exact::dec(529177210903, -22)),
                zero: Exact::ZERO,
                symbol: "a.u. of electric field",
                name: "atomic units of electric field",
            }
        },
    >;

    /// Coulombs per square metre (`C/m²`), a unit of `ElectricDisplacementField`: `n` of them is `n · 1.0` coherent units.
    pub type ElectricDisplacementFieldCoulombPerSquareMetre = SIUnit<
        ELECTRIC_DISPLACEMENT_FIELD,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "C/m²",
                name: "coulombs per square metre",
            }
        },
    >;

    /// Volt metres (`V ⋅ m`), a unit of `ElectricFlux`: `n` of them is `n · 1.0` coherent units.
    pub type VoltMetre = SIUnit<
        ELECTRIC_FLUX,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "V ⋅ m",
                name: "volt metres",
            }
        },
    >;

    /// Coulomb metres (`C · m`), a unit of `ElectricDipoleMoment`: `n` of them is `n · 1.0` coherent units.
    pub type CoulombMetre = SIUnit<
        ELECTRIC_DIPOLE_MOMENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "C · m",
                name: "coulomb metres",
            }
        },
    >;

    /// Atomic units of electric dipole moment (`e · a₀`), a unit of `ElectricDipoleMoment`: `n` of them is `n · 1.602176634e-19 * 5.29177210903e-11` coherent units.
    pub type AtomicUnitOfElectricDipoleMoment = SIUnit<
        ELECTRIC_DIPOLE_MOMENT,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28) * Exact::dec(529177210903, -22),
                zero: Exact::ZERO,
                symbol: "e · a₀",
                name: "atomic units of electric dipole moment",
            }
        },
    >;

    /// Atomic unit of charge centimetres (`a.u. of charge · cm`), a unit of `ElectricDipoleMoment`: `n` of them is `n · 1.602176634e-19 * 0.01` coherent units.
    pub type AtomicUnitOfChargeCentimetre = SIUnit<
        ELECTRIC_DIPOLE_MOMENT,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28) * Exact::dec(1, -2),
                zero: Exact::ZERO,
                symbol: "a.u. of charge · cm",
                name: "atomic unit of charge centimetres",
            }
        },
    >;

    /// Elementary charge centimetres (`e · cm`), a unit of `ElectricDipoleMoment`: `n` of them is `n · 1.602176634e-19 * 0.01` coherent units.
    pub type ElementaryChargeCentimetre = SIUnit<
        ELECTRIC_DIPOLE_MOMENT,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28) * Exact::dec(1, -2),
                zero: Exact::ZERO,
                symbol: "e · cm",
                name: "elementary charge centimetres",
            }
        },
    >;

    /// Coulomb square metres (`C · m²`), a unit of `ElectricQuadrupoleMoment`: `n` of them is `n · 1.0` coherent units.
    pub type CoulombSquareMetre = SIUnit<
        ELECTRIC_QUADRUPOLE_MOMENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "C · m²",
                name: "coulomb square metres",
            }
        },
    >;

    /// Elementary charge barns (`e · b`), a unit of `ElectricQuadrupoleMoment`: `n` of them is `n · 1.602176634e-19 * 1e-28` coherent units.
    pub type ElementaryChargeBarn = SIUnit<
        ELECTRIC_QUADRUPOLE_MOMENT,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28) * Exact::dec(1, -28),
                zero: Exact::ZERO,
                symbol: "e · b",
                name: "elementary charge barns",
            }
        },
    >;

    /// Atomic units of electric quadrupole moment (`e · a₀²`), a unit of `ElectricQuadrupoleMoment`: `n` of them is `n · 1.602176634e-19 * 5.29177210903e-11 * 5.29177210903e-11` coherent units.
    pub type AtomicUnitOfElectricQuadrupoleMoment = SIUnit<
        ELECTRIC_QUADRUPOLE_MOMENT,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28)
                    * Exact::dec(529177210903, -22)
                    * Exact::dec(529177210903, -22),
                zero: Exact::ZERO,
                symbol: "e · a₀²",
                name: "atomic units of electric quadrupole moment",
            }
        },
    >;

    /// Farads per metre (`F/m`), a unit of `ElectricPermittivity`: `n` of them is `n · 1.0` coherent units.
    pub type FaradPerMetre = SIUnit<
        ELECTRIC_PERMITTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "F/m",
                name: "farads per metre",
            }
        },
    >;

    /// Vacuum electric permittivity (`ε₀`), a unit of `ElectricPermittivity`: `n` of them is `n · 8.8541878128e-12` coherent units.
    pub type VacuumPermittivity = SIUnit<
        ELECTRIC_PERMITTIVITY,
        {
            SIUnitConsts {
                size: Exact::dec(88541878128, -22),
                zero: Exact::ZERO,
                symbol: "ε₀",
                name: "vacuum electric permittivity",
            }
        },
    >;

    /// Siemens (`S`), a unit of `ElectricalConductance`: `n` of them is `n · 1.0` coherent units.
    pub type Siemens = SIUnit<
        ELECTRICAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "S",
                name: "siemens",
            }
        },
    >;

    /// Mhos (`℧`), a unit of `ElectricalConductance`: `n` of them is `n · 1.0` coherent units.
    pub type Mho = SIUnit<
        ELECTRICAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "℧",
                name: "mhos",
            }
        },
    >;

    /// Abmhos (`abmho`), a unit of `ElectricalConductance`: `n` of them is `n · 1000000000.0` coherent units.
    pub type Abmho = SIUnit<
        ELECTRICAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1000000000),
                zero: Exact::ZERO,
                symbol: "abmho",
                name: "abmhos",
            }
        },
    >;

    /// Abmsiemens (`abS`), a unit of `ElectricalConductance`: `n` of them is `n · 1000000000.0` coherent units.
    pub type Absiemens = SIUnit<
        ELECTRICAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1000000000),
                zero: Exact::ZERO,
                symbol: "abS",
                name: "abmsiemens",
            }
        },
    >;

    /// Statsiemens (`statS`), a unit of `ElectricalConductance`: `n` of them is `n · 1.0 / (299.792458 * 2997924580.0)` coherent units.
    pub type Statsiemens = SIUnit<
        ELECTRICAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(299792458, -6) * Exact::int(2997924580)),
                zero: Exact::ZERO,
                symbol: "statS",
                name: "statsiemens",
            }
        },
    >;

    /// Statmhos (`statmho`), a unit of `ElectricalConductance`: `n` of them is `n · 1.0 / (299.792458 * 2997924580.0)` coherent units.
    pub type Statmho = SIUnit<
        ELECTRICAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(299792458, -6) * Exact::int(2997924580)),
                zero: Exact::ZERO,
                symbol: "statmho",
                name: "statmhos",
            }
        },
    >;

    /// Siemens per metre (`S/m`), a unit of `ElectricalConductivity`: `n` of them is `n · 1.0` coherent units.
    pub type SiemensPerMetre = SIUnit<
        ELECTRICAL_CONDUCTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "S/m",
                name: "siemens per metre",
            }
        },
    >;

    /// Square metres per volt second (`m²/(V · s)`), a unit of `ElectricalMobility`: `n` of them is `n · 1.0` coherent units.
    pub type SquareMetrePerVoltSecond = SIUnit<
        ELECTRICAL_MOBILITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m²/(V · s)",
                name: "square metres per volt second",
            }
        },
    >;

    /// Ohms (`Ω`), a unit of `ElectricalResistance`: `n` of them is `n · 1.0` coherent units.
    pub type Ohm = SIUnit<
        ELECTRICAL_RESISTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Ω",
                name: "ohms",
            }
        },
    >;

    /// Abohms (`abΩ`), a unit of `ElectricalResistance`: `n` of them is `n · 1e-09` coherent units.
    pub type Abohm = SIUnit<
        ELECTRICAL_RESISTANCE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -9),
                zero: Exact::ZERO,
                symbol: "abΩ",
                name: "abohms",
            }
        },
    >;

    /// Statohms (`statΩ`), a unit of `ElectricalResistance`: `n` of them is `n · 299.792458 * 2997924580.0` coherent units.
    pub type Statohm = SIUnit<
        ELECTRICAL_RESISTANCE,
        {
            SIUnitConsts {
                size: Exact::dec(299792458, -6) * Exact::int(2997924580),
                zero: Exact::ZERO,
                symbol: "statΩ",
                name: "statohms",
            }
        },
    >;

    /// Ohm metres (`Ω · m`), a unit of `ElectricalResistivity`: `n` of them is `n · 1.0` coherent units.
    pub type OhmMetre = SIUnit<
        ELECTRICAL_RESISTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Ω · m",
                name: "ohm metres",
            }
        },
    >;

    /// Abohm metres (`abΩ · m`), a unit of `ElectricalResistivity`: `n` of them is `n · 1e-09` coherent units.
    pub type AbohmMetre = SIUnit<
        ELECTRICAL_RESISTIVITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -9),
                zero: Exact::ZERO,
                symbol: "abΩ · m",
                name: "abohm metres",
            }
        },
    >;

    /// Statohm metres (`statΩ · m`), a unit of `ElectricalResistivity`: `n` of them is `n · 299.792458 * 2997924580.0` coherent units.
    pub type StatohmMetre = SIUnit<
        ELECTRICAL_RESISTIVITY,
        {
            SIUnitConsts {
                size: Exact::dec(299792458, -6) * Exact::int(2997924580),
                zero: Exact::ZERO,
                symbol: "statΩ · m",
                name: "statohm metres",
            }
        },
    >;

    /// Ohm inches (`Ω · in`), a unit of `ElectricalResistivity`: `n` of them is `n · 0.0254` coherent units.
    pub type OhmInch = SIUnit<
        ELECTRICAL_RESISTIVITY,
        {
            SIUnitConsts {
                size: Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "Ω · in",
                name: "ohm inches",
            }
        },
    >;

    /// Ohm feet (`Ω · ft`), a unit of `ElectricalResistivity`: `n` of them is `n · 0.3048` coherent units.
    pub type OhmFoot = SIUnit<
        ELECTRICAL_RESISTIVITY,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "Ω · ft",
                name: "ohm feet",
            }
        },
    >;

    /// Ohm yards (`Ω · yd`), a unit of `ElectricalResistivity`: `n` of them is `n · 0.9144` coherent units.
    pub type OhmYard = SIUnit<
        ELECTRICAL_RESISTIVITY,
        {
            SIUnitConsts {
                size: Exact::dec(9144, -4),
                zero: Exact::ZERO,
                symbol: "Ω · yd",
                name: "ohm yards",
            }
        },
    >;

    /// Ohm square millimetres per metre (`Ω · mm²/m`), a unit of `ElectricalResistivity`: `n` of them is `n · 1e-06` coherent units.
    pub type OhmSquareMillimetrePerMetre = SIUnit<
        ELECTRICAL_RESISTIVITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -6),
                zero: Exact::ZERO,
                symbol: "Ω · mm²/m",
                name: "ohm square millimetres per metre",
            }
        },
    >;

    /// Farads (`F`), a unit of `Capacitance`: `n` of them is `n · 1.0` coherent units.
    pub type Farad = SIUnit<
        CAPACITANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "F",
                name: "farads",
            }
        },
    >;

    /// Abfarads (`abF`), a unit of `Capacitance`: `n` of them is `n · 1000000000.0` coherent units.
    pub type Abfarad = SIUnit<
        CAPACITANCE,
        {
            SIUnitConsts {
                size: Exact::int(1000000000),
                zero: Exact::ZERO,
                symbol: "abF",
                name: "abfarads",
            }
        },
    >;

    /// Statfarads (`statF`), a unit of `Capacitance`: `n` of them is `n · 1.0 / (299.792458 * 2997924580.0)` coherent units.
    pub type Statfarad = SIUnit<
        CAPACITANCE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(299792458, -6) * Exact::int(2997924580)),
                zero: Exact::ZERO,
                symbol: "statF",
                name: "statfarads",
            }
        },
    >;

    /// Henries (`H`), a unit of `Inductance`: `n` of them is `n · 1.0` coherent units.
    pub type Henry = SIUnit<
        INDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "H",
                name: "henries",
            }
        },
    >;

    /// Abhenries (`abH`), a unit of `Inductance`: `n` of them is `n · 1e-09` coherent units.
    pub type Abhenry = SIUnit<
        INDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -9),
                zero: Exact::ZERO,
                symbol: "abH",
                name: "abhenries",
            }
        },
    >;

    /// Stathenries (`statH`), a unit of `Inductance`: `n` of them is `n · 299.792458 * 2997924580.0` coherent units.
    pub type Stathenry = SIUnit<
        INDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::dec(299792458, -6) * Exact::int(2997924580),
                zero: Exact::ZERO,
                symbol: "statH",
                name: "stathenries",
            }
        },
    >;

    /// Amperes per metre (`A/m`), a unit of `MagneticFieldStrength`: `n` of them is `n · 1.0` coherent units.
    pub type MagneticFieldStrengthAmperePerMetre = SIUnit<
        MAGNETIC_FIELD_STRENGTH,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "A/m",
                name: "amperes per metre",
            }
        },
    >;

    /// Oersteds (`Oe`), a unit of `MagneticFieldStrength`: `n` of them is `n · 1000.0 / (4.0 * core::f64::consts::PI)` coherent units.
    pub type Oersted = SIUnit<
        MAGNETIC_FIELD_STRENGTH,
        {
            SIUnitConsts {
                size: Exact::int(1000) / (Exact::int(4) * Exact::PI),
                zero: Exact::ZERO,
                symbol: "Oe",
                name: "oersteds",
            }
        },
    >;

    /// Webers (`Wb`), a unit of `MagneticFlux`: `n` of them is `n · 1.0` coherent units.
    pub type Weber = SIUnit<
        MAGNETIC_FLUX,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Wb",
                name: "webers",
            }
        },
    >;

    /// Maxwells (`Mx`), a unit of `MagneticFlux`: `n` of them is `n · 1e-08` coherent units.
    pub type Maxwell = SIUnit<
        MAGNETIC_FLUX,
        {
            SIUnitConsts {
                size: Exact::dec(1, -8),
                zero: Exact::ZERO,
                symbol: "Mx",
                name: "maxwells",
            }
        },
    >;

    /// Teslas (`T`), a unit of `MagneticFluxDensity`: `n` of them is `n · 1.0` coherent units.
    pub type Tesla = SIUnit<
        MAGNETIC_FLUX_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "T",
                name: "teslas",
            }
        },
    >;

    /// Gammas (`γ`), a unit of `MagneticFluxDensity`: `n` of them is `n · 1e-09` coherent units.
    pub type Gamma = SIUnit<
        MAGNETIC_FLUX_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -9),
                zero: Exact::ZERO,
                symbol: "γ",
                name: "gammas",
            }
        },
    >;

    /// Gauss (`G`), a unit of `MagneticFluxDensity`: `n` of them is `n · 0.0001` coherent units.
    pub type Gauss = SIUnit<
        MAGNETIC_FLUX_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -4),
                zero: Exact::ZERO,
                symbol: "G",
                name: "gauss",
            }
        },
    >;

    /// Ampere square metres (`A · m²`), a unit of `MagneticMoment`: `n` of them is `n · 1.0` coherent units.
    pub type AmpereSquareMetre = SIUnit<
        MAGNETIC_MOMENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "A · m²",
                name: "ampere square metres",
            }
        },
    >;

    /// Joules per tesla (`J/T`), a unit of `MagneticMoment`: `n` of them is `n · 1.0` coherent units.
    pub type JoulePerTesla = SIUnit<
        MAGNETIC_MOMENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/T",
                name: "joules per tesla",
            }
        },
    >;

    /// Newton metres per tesla (`N · m/T`), a unit of `MagneticMoment`: `n` of them is `n · 1.0` coherent units.
    pub type NewtonMetrePerTesla = SIUnit<
        MAGNETIC_MOMENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "N · m/T",
                name: "newton metres per tesla",
            }
        },
    >;

    /// Ergs per gauss (`erg/G`), a unit of `MagneticMoment`: `n` of them is `n · 0.001` coherent units.
    pub type ErgPerGauss = SIUnit<
        MAGNETIC_MOMENT,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "erg/G",
                name: "ergs per gauss",
            }
        },
    >;

    /// Bohr magnetons (`µ(Bohr)`), a unit of `MagneticMoment`: `n` of them is `n · 9.2740100783e-24` coherent units.
    pub type BohrMagneton = SIUnit<
        MAGNETIC_MOMENT,
        {
            SIUnitConsts {
                size: Exact::dec(92740100783, -34),
                zero: Exact::ZERO,
                symbol: "µ(Bohr)",
                name: "Bohr magnetons",
            }
        },
    >;

    /// Nuclear magnetons (`μ(Nuclear)`), a unit of `MagneticMoment`: `n` of them is `n · 5.0507837461e-27` coherent units.
    pub type NuclearMagneton = SIUnit<
        MAGNETIC_MOMENT,
        {
            SIUnitConsts {
                size: Exact::dec(50507837461, -37),
                zero: Exact::ZERO,
                symbol: "μ(Nuclear)",
                name: "nuclear magnetons",
            }
        },
    >;

    ///  atomic units of magnetic dipole moment (`ħ · e/mₑ`), a unit of `MagneticMoment`: `n` of them is `n · 1.85480201566e-23` coherent units.
    pub type AtomicUnitOfMagneticDipoleMoment = SIUnit<
        MAGNETIC_MOMENT,
        {
            SIUnitConsts {
                size: Exact::dec(185480201566, -34),
                zero: Exact::ZERO,
                symbol: "ħ · e/mₑ",
                name: " atomic units of magnetic dipole moment",
            }
        },
    >;

    /// Deuteron magnetic moments (`μ(deuteron)`), a unit of `MagneticMoment`: `n` of them is `n · 4.330735094e-27` coherent units.
    pub type DeuteronMagneticMoment = SIUnit<
        MAGNETIC_MOMENT,
        {
            SIUnitConsts {
                size: Exact::dec(4330735094, -36),
                zero: Exact::ZERO,
                symbol: "μ(deuteron)",
                name: "deuteron magnetic moments",
            }
        },
    >;

    /// Proton magnetic moments (`μ(proton)`), a unit of `MagneticMoment`: `n` of them is `n · 1.41060679736e-26` coherent units.
    pub type ProtonMagneticMoment = SIUnit<
        MAGNETIC_MOMENT,
        {
            SIUnitConsts {
                size: Exact::dec(141060679736, -37),
                zero: Exact::ZERO,
                symbol: "μ(proton)",
                name: "proton magnetic moments",
            }
        },
    >;

    /// Shielded proton magnetic moments (`μ’(proton)`), a unit of `MagneticMoment`: `n` of them is `n · 1.41057056e-26` coherent units.
    pub type ShieldedProtonMagneticMoment = SIUnit<
        MAGNETIC_MOMENT,
        {
            SIUnitConsts {
                size: Exact::dec(141057056, -34),
                zero: Exact::ZERO,
                symbol: "μ’(proton)",
                name: "shielded proton magnetic moments",
            }
        },
    >;

    /// Triton magnetic moments (`μ(triton)`), a unit of `MagneticMoment`: `n` of them is `n · 1.5046095202e-26` coherent units.
    pub type TritonMagneticMoment = SIUnit<
        MAGNETIC_MOMENT,
        {
            SIUnitConsts {
                size: Exact::dec(15046095202, -36),
                zero: Exact::ZERO,
                symbol: "μ(triton)",
                name: "triton magnetic moments",
            }
        },
    >;

    /// Henrys per metre (`H/m`), a unit of `MagneticPermeability`: `n` of them is `n · 1.0` coherent units.
    pub type HenryPerMetre = SIUnit<
        MAGNETIC_PERMEABILITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "H/m",
                name: "henrys per metre",
            }
        },
    >;

    /// Vacuum magnetic permeability (`µ₀`), a unit of `MagneticPermeability`: `n` of them is `n · 1.25663706212e-6` coherent units.
    pub type VacuumPermeability = SIUnit<
        MAGNETIC_PERMEABILITY,
        {
            SIUnitConsts {
                size: Exact::dec(125663706212, -17),
                zero: Exact::ZERO,
                symbol: "µ₀",
                name: "vacuum magnetic permeability",
            }
        },
    >;

    /// Hertz per tesla (`Hz/T`), a unit of `GyromagneticRatio`: `n` of them is `n · 1.0` coherent units.
    pub type HertzPerTesla = SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Hz/T",
                name: "hertz per tesla",
            }
        },
    >;

    /// Radians per second tesla (`rad/(s · T)`), a unit of `GyromagneticRatio`: `n` of them is `n · 1.0` coherent units.
    pub type RadianPerSecondTesla = SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "rad/(s · T)",
                name: "radians per second tesla",
            }
        },
    >;

    /// Proton gyromagnetic ratio (`γₚ`), a unit of `GyromagneticRatio`: `n` of them is `n · 267522187.08` coherent units.
    pub type ProtonGyromagneticRatio = SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(26752218708, -2),
                zero: Exact::ZERO,
                symbol: "γₚ",
                name: "proton gyromagnetic ratio",
            }
        },
    >;

    /// Neutron gyromagnetic ratio (`γₙ`), a unit of `GyromagneticRatio`: `n` of them is `n · 183247174.0` coherent units.
    pub type NeutronGyromagneticRatio = SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::int(183247174),
                zero: Exact::ZERO,
                symbol: "γₙ",
                name: "neutron gyromagnetic ratio",
            }
        },
    >;

    /// Electron gyromagnetic ratio (`γₑ`), a unit of `GyromagneticRatio`: `n` of them is `n · 176085962784.0` coherent units.
    pub type ElectronGyromagneticRatio = SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::int(176085962784),
                zero: Exact::ZERO,
                symbol: "γₑ",
                name: "electron gyromagnetic ratio",
            }
        },
    >;

    /// Shielded proton gyromagnetic ratio (`γ'ₚ`), a unit of `GyromagneticRatio`: `n` of them is `n · 267515319.4` coherent units.
    pub type ShieldedProtonGyromagneticRatio = SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(2675153194, -1),
                zero: Exact::ZERO,
                symbol: "γ'ₚ",
                name: "shielded proton gyromagnetic ratio",
            }
        },
    >;

    /// Shielded helion gyromagnetic ratio (`γ'ₕ`), a unit of `GyromagneticRatio`: `n` of them is `n · 203789460.78` coherent units.
    pub type ShieldedHelionGyromagneticRatio = SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(20378946078, -2),
                zero: Exact::ZERO,
                symbol: "γ'ₕ",
                name: "shielded helion gyromagnetic ratio",
            }
        },
    >;

    /// Deuteron gyromagnetic ratio (`γ₂`), a unit of `GyromagneticRatio`: `n` of them is `n · 41066288.8` coherent units.
    pub type DeuteronGyromagneticRatio = SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(410662888, -1),
                zero: Exact::ZERO,
                symbol: "γ₂",
                name: "deuteron gyromagnetic ratio",
            }
        },
    >;

    /// Triton gyromagnetic ratio (`γ₃`), a unit of `GyromagneticRatio`: `n` of them is `n · 285349844.0` coherent units.
    pub type TritonGyromagneticRatio = SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::int(285349844),
                zero: Exact::ZERO,
                symbol: "γ₃",
                name: "triton gyromagnetic ratio",
            }
        },
    >;

    /// Lithium7 gyromagnetic ratio (`γ₇`), a unit of `GyromagneticRatio`: `n` of them is `n · 103975264.0` coherent units.
    pub type Lithium7GyromagneticRatio = SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::int(103975264),
                zero: Exact::ZERO,
                symbol: "γ₇",
                name: "lithium7 gyromagnetic ratio",
            }
        },
    >;

    /// Boron11 gyromagnetic ratio (`γ₁₁`), a unit of `GyromagneticRatio`: `n` of them is `n · 85838414.3` coherent units.
    pub type Boron11GyromagneticRatio = SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(858384143, -1),
                zero: Exact::ZERO,
                symbol: "γ₁₁",
                name: "boron11 gyromagnetic ratio",
            }
        },
    >;

    /// Carbon13 gyromagnetic ratio (`γ₁₃`), a unit of `GyromagneticRatio`: `n` of them is `n · 67278754.6` coherent units.
    pub type Carbon13GyromagneticRatio = SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(672787546, -1),
                zero: Exact::ZERO,
                symbol: "γ₁₃",
                name: "carbon13 gyromagnetic ratio",
            }
        },
    >;

    /// Nitrogen14 gyromagnetic ratio (`γ₁₄`), a unit of `GyromagneticRatio`: `n` of them is `n · 19328792.2` coherent units.
    pub type Nitrogen14GyromagneticRatio = SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(193287922, -1),
                zero: Exact::ZERO,
                symbol: "γ₁₄",
                name: "nitrogen14 gyromagnetic ratio",
            }
        },
    >;

    /// Flourine19 gyromagnetic ratio (`γ₁₉`), a unit of `GyromagneticRatio`: `n` of them is `n · 251762483.0` coherent units.
    pub type Flourine19GyromagneticRatio = SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::int(251762483),
                zero: Exact::ZERO,
                symbol: "γ₁₉",
                name: "flourine19 gyromagnetic ratio",
            }
        },
    >;

    /// Sodium23 gyromagnetic ratio (`γ₂₃`), a unit of `GyromagneticRatio`: `n` of them is `n · 70803541.7` coherent units.
    pub type Sodium23GyromagneticRatio = SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(708035417, -1),
                zero: Exact::ZERO,
                symbol: "γ₂₃",
                name: "sodium23 gyromagnetic ratio",
            }
        },
    >;

    /// Phosphorus31 gyromagnetic ratio (`γ₃₁`), a unit of `GyromagneticRatio`: `n` of them is `n · 108329419.0` coherent units.
    pub type Phosphorus31GyromagneticRatio = SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::int(108329419),
                zero: Exact::ZERO,
                symbol: "γ₃₁",
                name: "phosphorus31 gyromagnetic ratio",
            }
        },
    >;

    /// Proton reduced gyromagnetic ratio (`̅γₚ`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 42577478.461` coherent units.
    pub type ProtonReducedGyromagneticRatio = SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(42577478461, -3),
                zero: Exact::ZERO,
                symbol: "̅γₚ",
                name: "proton reduced gyromagnetic ratio",
            }
        },
    >;

    /// Neutron reduced gyromagnetic ratio (`̅γₙ`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 29164693.5` coherent units.
    pub type NeutronReducedGyromagneticRatio = SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(291646935, -1),
                zero: Exact::ZERO,
                symbol: "̅γₙ",
                name: "neutron reduced gyromagnetic ratio",
            }
        },
    >;

    /// Electron reduced gyromagnetic ratio (`̅γₑ`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 28024951386.1` coherent units.
    pub type ElectronReducedGyromagneticRatio = SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(280249513861, -1),
                zero: Exact::ZERO,
                symbol: "̅γₑ",
                name: "electron reduced gyromagnetic ratio",
            }
        },
    >;

    /// Shielded proton reduced gyromagnetic ratio (`γ'ₚ`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 42576385.43` coherent units.
    pub type ShieldedProtonReducedGyromagneticRatio = SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(4257638543, -2),
                zero: Exact::ZERO,
                symbol: "γ'ₚ",
                name: "shielded proton reduced gyromagnetic ratio",
            }
        },
    >;

    /// Shielded helion reduced gyromagnetic ratio (`γ'ₕ`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 32434100.033` coherent units.
    pub type ShieldedHelionReducedGyromagneticRatio = SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(32434100033, -3),
                zero: Exact::ZERO,
                symbol: "γ'ₕ",
                name: "shielded helion reduced gyromagnetic ratio",
            }
        },
    >;

    /// Deuteron reduced gyromagnetic ratio (`̅γ₂`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 6535902.85` coherent units.
    pub type DeuteronReducedGyromagneticRatio = SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(653590285, -2),
                zero: Exact::ZERO,
                symbol: "̅γ₂",
                name: "deuteron reduced gyromagnetic ratio",
            }
        },
    >;

    /// Triton reduced gyromagnetic ratio (`̅γ₃`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 45414838.1` coherent units.
    pub type TritonReducedGyromagneticRatio = SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(454148381, -1),
                zero: Exact::ZERO,
                symbol: "̅γ₃",
                name: "triton reduced gyromagnetic ratio",
            }
        },
    >;

    /// Lithium7 reduced gyromagnetic ratio (`̅γ₇`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 16548177.3` coherent units.
    pub type Lithium7ReducedGyromagneticRatio = SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(165481773, -1),
                zero: Exact::ZERO,
                symbol: "̅γ₇",
                name: "lithium7 reduced gyromagnetic ratio",
            }
        },
    >;

    /// Boron11 reduced gyromagnetic ratio (`̅γ₁₁`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 13661608.0` coherent units.
    pub type Boron11ReducedGyromagneticRatio = SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::int(13661608),
                zero: Exact::ZERO,
                symbol: "̅γ₁₁",
                name: "boron11 reduced gyromagnetic ratio",
            }
        },
    >;

    /// Carbon13 reduced gyromagnetic ratio (`̅γ₁₃`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 10707746.4` coherent units.
    pub type Carbon13ReducedGyromagneticRatio = SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(107077464, -1),
                zero: Exact::ZERO,
                symbol: "̅γ₁₃",
                name: "carbon13 reduced gyromagnetic ratio",
            }
        },
    >;

    /// Nitrogen14 reduced gyromagnetic ratio (`̅γ₁₄`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 3076272.81` coherent units.
    pub type Nitrogen14ReducedGyromagneticRatio = SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(307627281, -2),
                zero: Exact::ZERO,
                symbol: "̅γ₁₄",
                name: "nitrogen14 reduced gyromagnetic ratio",
            }
        },
    >;

    /// Flourine19 reduced gyromagnetic ratio (`̅γ₁₉`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 40069243.7` coherent units.
    pub type Flourine19ReducedGyromagneticRatio = SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(400692437, -1),
                zero: Exact::ZERO,
                symbol: "̅γ₁₉",
                name: "flourine19 reduced gyromagnetic ratio",
            }
        },
    >;

    /// Sodium23 reduced gyromagnetic ratio (`̅γ₂₃`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 11268733.6` coherent units.
    pub type Sodium23ReducedGyromagneticRatio = SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(112687336, -1),
                zero: Exact::ZERO,
                symbol: "̅γ₂₃",
                name: "sodium23 reduced gyromagnetic ratio",
            }
        },
    >;

    /// Phosphorus31 reduced gyromagnetic ratio (`̅γ₃₁`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 17241162.5` coherent units.
    pub type Phosphorus31ReducedGyromagneticRatio = SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(172411625, -1),
                zero: Exact::ZERO,
                symbol: "̅γ₃₁",
                name: "phosphorus31 reduced gyromagnetic ratio",
            }
        },
    >;
}
