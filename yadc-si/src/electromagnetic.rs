use crate::ISQ;
use yadc::Dimension;

/// The dimension of `ElectricCharge`: `SIQuantity<ELECTRIC_CHARGE, N>` is one.
pub const ELECTRIC_CHARGE: ISQ = ISQ {
    s: 1,
    A: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ElectricChargeArealDensity`: `SIQuantity<ELECTRIC_CHARGE_AREAL_DENSITY, N>` is one.
pub const ELECTRIC_CHARGE_AREAL_DENSITY: ISQ = ISQ {
    m: -2,
    s: 1,
    A: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ElectricChargeLinearDensity`: `SIQuantity<ELECTRIC_CHARGE_LINEAR_DENSITY, N>` is one.
pub const ELECTRIC_CHARGE_LINEAR_DENSITY: ISQ = ISQ {
    m: -1,
    s: 1,
    A: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ElectricChargeVolumetricDensity`: `SIQuantity<ELECTRIC_CHARGE_VOLUMETRIC_DENSITY, N>` is one.
pub const ELECTRIC_CHARGE_VOLUMETRIC_DENSITY: ISQ = ISQ {
    m: -3,
    s: 1,
    A: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ElectricCurrent`: `SIQuantity<ELECTRIC_CURRENT, N>` is one.
pub const ELECTRIC_CURRENT: ISQ = ISQ {
    A: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ElectricCurrentDensity`: `SIQuantity<ELECTRIC_CURRENT_DENSITY, N>` is one.
pub const ELECTRIC_CURRENT_DENSITY: ISQ = ISQ {
    m: -2,
    A: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `SurfaceElectricCurrentDensity`: `SIQuantity<SURFACE_ELECTRIC_CURRENT_DENSITY, N>` is one.
pub const SURFACE_ELECTRIC_CURRENT_DENSITY: ISQ = ISQ {
    m: -1,
    A: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ElectricPotential`: `SIQuantity<ELECTRIC_POTENTIAL, N>` is one.
pub const ELECTRIC_POTENTIAL: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -3,
    A: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ElectricField`: `SIQuantity<ELECTRIC_FIELD, N>` is one.
pub const ELECTRIC_FIELD: ISQ = ISQ {
    m: 1,
    kg: 1,
    s: -3,
    A: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ElectricDisplacementField`: `SIQuantity<ELECTRIC_DISPLACEMENT_FIELD, N>` is one.
pub const ELECTRIC_DISPLACEMENT_FIELD: ISQ = ISQ {
    m: -2,
    s: 1,
    A: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ElectricFlux`: `SIQuantity<ELECTRIC_FLUX, N>` is one.
pub const ELECTRIC_FLUX: ISQ = ISQ {
    m: 3,
    kg: 1,
    s: -3,
    A: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ElectricDipoleMoment`: `SIQuantity<ELECTRIC_DIPOLE_MOMENT, N>` is one.
pub const ELECTRIC_DIPOLE_MOMENT: ISQ = ISQ {
    m: 1,
    s: 1,
    A: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ElectricQuadrupoleMoment`: `SIQuantity<ELECTRIC_QUADRUPOLE_MOMENT, N>` is one.
pub const ELECTRIC_QUADRUPOLE_MOMENT: ISQ = ISQ {
    m: 2,
    s: 1,
    A: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ElectricPermittivity`: `SIQuantity<ELECTRIC_PERMITTIVITY, N>` is one.
pub const ELECTRIC_PERMITTIVITY: ISQ = ISQ {
    m: -3,
    kg: -1,
    s: 4,
    A: 2,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ElectricalConductance`: `SIQuantity<ELECTRICAL_CONDUCTANCE, N>` is one.
pub const ELECTRICAL_CONDUCTANCE: ISQ = ISQ {
    m: -2,
    kg: -1,
    s: 3,
    A: 2,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ElectricalConductivity`: `SIQuantity<ELECTRICAL_CONDUCTIVITY, N>` is one.
pub const ELECTRICAL_CONDUCTIVITY: ISQ = ISQ {
    m: -3,
    kg: -1,
    s: 3,
    A: 2,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ElectricalMobility`: `SIQuantity<ELECTRICAL_MOBILITY, N>` is one.
pub const ELECTRICAL_MOBILITY: ISQ = ISQ {
    kg: -1,
    s: 2,
    A: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ElectricalResistance`: `SIQuantity<ELECTRICAL_RESISTANCE, N>` is one.
pub const ELECTRICAL_RESISTANCE: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -3,
    A: -2,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ElectricalResistivity`: `SIQuantity<ELECTRICAL_RESISTIVITY, N>` is one.
pub const ELECTRICAL_RESISTIVITY: ISQ = ISQ {
    m: 3,
    kg: 1,
    s: -3,
    A: -2,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `Capacitance`: `SIQuantity<CAPACITANCE, N>` is one.
pub const CAPACITANCE: ISQ = ISQ {
    m: -2,
    kg: -1,
    s: 4,
    A: 2,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `Inductance`: `SIQuantity<INDUCTANCE, N>` is one.
pub const INDUCTANCE: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -2,
    A: -2,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `MagneticFieldStrength`: `SIQuantity<MAGNETIC_FIELD_STRENGTH, N>` is one.
pub const MAGNETIC_FIELD_STRENGTH: ISQ = ISQ {
    m: -1,
    A: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `MagneticFlux`: `SIQuantity<MAGNETIC_FLUX, N>` is one.
pub const MAGNETIC_FLUX: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -2,
    A: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `MagneticFluxDensity`: `SIQuantity<MAGNETIC_FLUX_DENSITY, N>` is one.
pub const MAGNETIC_FLUX_DENSITY: ISQ = ISQ {
    kg: 1,
    s: -2,
    A: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `MagneticMoment`: `SIQuantity<MAGNETIC_MOMENT, N>` is one.
pub const MAGNETIC_MOMENT: ISQ = ISQ {
    m: 2,
    A: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `MagneticPermeability`: `SIQuantity<MAGNETIC_PERMEABILITY, N>` is one.
pub const MAGNETIC_PERMEABILITY: ISQ = ISQ {
    m: 1,
    kg: 1,
    s: -2,
    A: -2,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `GyromagneticRatio`: `SIQuantity<GYROMAGNETIC_RATIO, N>` is one.
pub const GYROMAGNETIC_RATIO: ISQ = ISQ {
    kg: -1,
    s: 1,
    A: 1,
    rad: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ReducedGyromagneticRatio`: `SIQuantity<REDUCED_GYROMAGNETIC_RATIO, N>` is one.
pub const REDUCED_GYROMAGNETIC_RATIO: ISQ = ISQ {
    kg: -1,
    s: 1,
    A: 1,
    ..ISQ::DIMENSIONLESS
};

/// The units of this module's quantities: each a constant of its own unit
/// type, its size and zero in its quantity's coherent unit, exact
/// (`MILE.quantity(3.0)`, `MILE.value(d)`).
pub mod units {
    use super::*;
    use crate::{Exact, SIUnit, SIUnitConsts};

    /// Coulombs (`C`), a unit of `ElectricCharge`: `n` of them is `n · 1.0` coherent units.
    pub const COULOMB: SIUnit<
        ELECTRIC_CHARGE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "C",
                name: "coulombs",
            }
        },
    > = SIUnit;

    /// Ampere hours (`A · h`), a unit of `ElectricCharge`: `n` of them is `n · 3600.0` coherent units.
    pub const AMPERE_HOUR: SIUnit<
        ELECTRIC_CHARGE,
        {
            SIUnitConsts {
                size: Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "A · h",
                name: "ampere hours",
            }
        },
    > = SIUnit;

    /// Elementary charges (`e`), a unit of `ElectricCharge`: `n` of them is `n · 1.602176634e-19` coherent units.
    pub const ELEMENTARY_CHARGE: SIUnit<
        ELECTRIC_CHARGE,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28),
                zero: Exact::ZERO,
                symbol: "e",
                name: "elementary charges",
            }
        },
    > = SIUnit;

    /// Atomic units of charge (`a.u. of charge`), a unit of `ElectricCharge`: `n` of them is `n · 1.602176634e-19` coherent units.
    pub const ATOMIC_UNIT_OF_CHARGE: SIUnit<
        ELECTRIC_CHARGE,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28),
                zero: Exact::ZERO,
                symbol: "a.u. of charge",
                name: "atomic units of charge",
            }
        },
    > = SIUnit;

    /// Abcoulombs (`abC`), a unit of `ElectricCharge`: `n` of them is `n · 10.0` coherent units.
    pub const ABCOULOMB: SIUnit<
        ELECTRIC_CHARGE,
        {
            SIUnitConsts {
                size: Exact::int(10),
                zero: Exact::ZERO,
                symbol: "abC",
                name: "abcoulombs",
            }
        },
    > = SIUnit;

    /// Faradays (`F`), a unit of `ElectricCharge`: `n` of them is `n · 1.602176634e-19 * 6.02214076e23` coherent units.
    pub const FARADAY: SIUnit<
        ELECTRIC_CHARGE,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28) * Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "F",
                name: "faradays",
            }
        },
    > = SIUnit;

    /// Franklins (`Fr`), a unit of `ElectricCharge`: `n` of them is `n · 1.0 / 2997924580.0` coherent units.
    pub const FRANKLIN: SIUnit<
        ELECTRIC_CHARGE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(2997924580),
                zero: Exact::ZERO,
                symbol: "Fr",
                name: "franklins",
            }
        },
    > = SIUnit;

    /// Statcoulombs (`statC`), a unit of `ElectricCharge`: `n` of them is `n · 1.0 / 2997924580.0` coherent units.
    pub const STATCOULOMB: SIUnit<
        ELECTRIC_CHARGE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(2997924580),
                zero: Exact::ZERO,
                symbol: "statC",
                name: "statcoulombs",
            }
        },
    > = SIUnit;

    /// Coulombs per square metre (`C/m²`), a unit of `ElectricChargeArealDensity`: `n` of them is `n · 1.0` coherent units.
    pub const ELECTRIC_CHARGE_AREAL_DENSITY_COULOMB_PER_SQUARE_METRE: SIUnit<
        ELECTRIC_CHARGE_AREAL_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "C/m²",
                name: "coulombs per square metre",
            }
        },
    > = SIUnit;

    /// Coulombs per metre (`C/m`), a unit of `ElectricChargeLinearDensity`: `n` of them is `n · 1.0` coherent units.
    pub const COULOMB_PER_METRE: SIUnit<
        ELECTRIC_CHARGE_LINEAR_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "C/m",
                name: "coulombs per metre",
            }
        },
    > = SIUnit;

    /// Coulombs per cubic metre (`C/m³`), a unit of `ElectricChargeVolumetricDensity`: `n` of them is `n · 1.0` coherent units.
    pub const COULOMB_PER_CUBIC_METRE: SIUnit<
        ELECTRIC_CHARGE_VOLUMETRIC_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "C/m³",
                name: "coulombs per cubic metre",
            }
        },
    > = SIUnit;

    /// Amperes (`A`), a unit of `ElectricCurrent`: `n` of them is `n · 1.0` coherent units.
    pub const AMPERE: SIUnit<
        ELECTRIC_CURRENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "A",
                name: "amperes",
            }
        },
    > = SIUnit;

    /// Elementary charges per second (`e/s`), a unit of `ElectricCurrent`: `n` of them is `n · 1.602176634e-19` coherent units.
    pub const ELEMENTARY_CHARGE_PER_SECOND: SIUnit<
        ELECTRIC_CURRENT,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28),
                zero: Exact::ZERO,
                symbol: "e/s",
                name: "elementary charges per second",
            }
        },
    > = SIUnit;

    /// Atomic units of charge per second (`a.u. of charge/s`), a unit of `ElectricCurrent`: `n` of them is `n · 1.602176634e-19` coherent units.
    pub const ATOMIC_UNIT_OF_CHARGE_PER_SECOND: SIUnit<
        ELECTRIC_CURRENT,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28),
                zero: Exact::ZERO,
                symbol: "a.u. of charge/s",
                name: "atomic units of charge per second",
            }
        },
    > = SIUnit;

    /// Abamperes (`abA`), a unit of `ElectricCurrent`: `n` of them is `n · 10.0` coherent units.
    pub const ABAMPERE: SIUnit<
        ELECTRIC_CURRENT,
        {
            SIUnitConsts {
                size: Exact::int(10),
                zero: Exact::ZERO,
                symbol: "abA",
                name: "abamperes",
            }
        },
    > = SIUnit;

    /// Gilberts (`Gi`), a unit of `ElectricCurrent`: `n` of them is `n · 10.0 / (4.0 * core::f64::consts::PI)` coherent units.
    pub const GILBERT: SIUnit<
        ELECTRIC_CURRENT,
        {
            SIUnitConsts {
                size: Exact::int(10) / (Exact::int(4) * Exact::PI),
                zero: Exact::ZERO,
                symbol: "Gi",
                name: "gilberts",
            }
        },
    > = SIUnit;

    /// Statamperes (`statA`), a unit of `ElectricCurrent`: `n` of them is `n · 1.0 / 2997924580.0` coherent units.
    pub const STATAMPERE: SIUnit<
        ELECTRIC_CURRENT,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(2997924580),
                zero: Exact::ZERO,
                symbol: "statA",
                name: "statamperes",
            }
        },
    > = SIUnit;

    /// Amperes per square metre (`A/m²`), a unit of `ElectricCurrentDensity`: `n` of them is `n · 1.0` coherent units.
    pub const AMPERE_PER_SQUARE_METRE: SIUnit<
        ELECTRIC_CURRENT_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "A/m²",
                name: "amperes per square metre",
            }
        },
    > = SIUnit;

    /// Amperes per metre (`A/m`), a unit of `SurfaceElectricCurrentDensity`: `n` of them is `n · 1.0` coherent units.
    pub const SURFACE_ELECTRIC_CURRENT_DENSITY_AMPERE_PER_METRE: SIUnit<
        SURFACE_ELECTRIC_CURRENT_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "A/m",
                name: "amperes per metre",
            }
        },
    > = SIUnit;

    /// Volts (`V`), a unit of `ElectricPotential`: `n` of them is `n · 1.0` coherent units.
    pub const VOLT: SIUnit<
        ELECTRIC_POTENTIAL,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "V",
                name: "volts",
            }
        },
    > = SIUnit;

    /// Abvolts (`abV`), a unit of `ElectricPotential`: `n` of them is `n · 1e-08` coherent units.
    pub const ABVOLT: SIUnit<
        ELECTRIC_POTENTIAL,
        {
            SIUnitConsts {
                size: Exact::dec(1, -8),
                zero: Exact::ZERO,
                symbol: "abV",
                name: "abvolts",
            }
        },
    > = SIUnit;

    /// Statvolts (`statV`), a unit of `ElectricPotential`: `n` of them is `n · 299.792458` coherent units.
    pub const STATVOLT: SIUnit<
        ELECTRIC_POTENTIAL,
        {
            SIUnitConsts {
                size: Exact::dec(299792458, -6),
                zero: Exact::ZERO,
                symbol: "statV",
                name: "statvolts",
            }
        },
    > = SIUnit;

    /// Volts per metre (`V/m`), a unit of `ElectricField`: `n` of them is `n · 1.0` coherent units.
    pub const VOLT_PER_METRE: SIUnit<
        ELECTRIC_FIELD,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "V/m",
                name: "volts per metre",
            }
        },
    > = SIUnit;

    /// Volts per mil (`V/mil`), a unit of `ElectricField`: `n` of them is `n · 1.0 / 2.54e-5` coherent units.
    pub const VOLT_PER_MIL: SIUnit<
        ELECTRIC_FIELD,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(254, -7),
                zero: Exact::ZERO,
                symbol: "V/mil",
                name: "volts per mil",
            }
        },
    > = SIUnit;

    /// Atomic units of electric field (`a.u. of electric field`), a unit of `ElectricField`: `n` of them is `n · 4.3597447222071e-18 / (1.602176634e-19 * 5.29177210903e-11)` coherent units.
    pub const ATOMIC_UNIT_OF_ELECTRIC_FIELD: SIUnit<
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
    > = SIUnit;

    /// Coulombs per square metre (`C/m²`), a unit of `ElectricDisplacementField`: `n` of them is `n · 1.0` coherent units.
    pub const ELECTRIC_DISPLACEMENT_FIELD_COULOMB_PER_SQUARE_METRE: SIUnit<
        ELECTRIC_DISPLACEMENT_FIELD,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "C/m²",
                name: "coulombs per square metre",
            }
        },
    > = SIUnit;

    /// Volt metres (`V ⋅ m`), a unit of `ElectricFlux`: `n` of them is `n · 1.0` coherent units.
    pub const VOLT_METRE: SIUnit<
        ELECTRIC_FLUX,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "V ⋅ m",
                name: "volt metres",
            }
        },
    > = SIUnit;

    /// Coulomb metres (`C · m`), a unit of `ElectricDipoleMoment`: `n` of them is `n · 1.0` coherent units.
    pub const COULOMB_METRE: SIUnit<
        ELECTRIC_DIPOLE_MOMENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "C · m",
                name: "coulomb metres",
            }
        },
    > = SIUnit;

    /// Atomic units of electric dipole moment (`e · a₀`), a unit of `ElectricDipoleMoment`: `n` of them is `n · 1.602176634e-19 * 5.29177210903e-11` coherent units.
    pub const ATOMIC_UNIT_OF_ELECTRIC_DIPOLE_MOMENT: SIUnit<
        ELECTRIC_DIPOLE_MOMENT,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28) * Exact::dec(529177210903, -22),
                zero: Exact::ZERO,
                symbol: "e · a₀",
                name: "atomic units of electric dipole moment",
            }
        },
    > = SIUnit;

    /// Atomic unit of charge centimetres (`a.u. of charge · cm`), a unit of `ElectricDipoleMoment`: `n` of them is `n · 1.602176634e-19 * 0.01` coherent units.
    pub const ATOMIC_UNIT_OF_CHARGE_CENTIMETRE: SIUnit<
        ELECTRIC_DIPOLE_MOMENT,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28) * Exact::dec(1, -2),
                zero: Exact::ZERO,
                symbol: "a.u. of charge · cm",
                name: "atomic unit of charge centimetres",
            }
        },
    > = SIUnit;

    /// Elementary charge centimetres (`e · cm`), a unit of `ElectricDipoleMoment`: `n` of them is `n · 1.602176634e-19 * 0.01` coherent units.
    pub const ELEMENTARY_CHARGE_CENTIMETRE: SIUnit<
        ELECTRIC_DIPOLE_MOMENT,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28) * Exact::dec(1, -2),
                zero: Exact::ZERO,
                symbol: "e · cm",
                name: "elementary charge centimetres",
            }
        },
    > = SIUnit;

    /// Coulomb square metres (`C · m²`), a unit of `ElectricQuadrupoleMoment`: `n` of them is `n · 1.0` coherent units.
    pub const COULOMB_SQUARE_METRE: SIUnit<
        ELECTRIC_QUADRUPOLE_MOMENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "C · m²",
                name: "coulomb square metres",
            }
        },
    > = SIUnit;

    /// Elementary charge barns (`e · b`), a unit of `ElectricQuadrupoleMoment`: `n` of them is `n · 1.602176634e-19 * 1e-28` coherent units.
    pub const ELEMENTARY_CHARGE_BARN: SIUnit<
        ELECTRIC_QUADRUPOLE_MOMENT,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28) * Exact::dec(1, -28),
                zero: Exact::ZERO,
                symbol: "e · b",
                name: "elementary charge barns",
            }
        },
    > = SIUnit;

    /// Atomic units of electric quadrupole moment (`e · a₀²`), a unit of `ElectricQuadrupoleMoment`: `n` of them is `n · 1.602176634e-19 * 5.29177210903e-11 * 5.29177210903e-11` coherent units.
    pub const ATOMIC_UNIT_OF_ELECTRIC_QUADRUPOLE_MOMENT: SIUnit<
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
    > = SIUnit;

    /// Farads per metre (`F/m`), a unit of `ElectricPermittivity`: `n` of them is `n · 1.0` coherent units.
    pub const FARAD_PER_METRE: SIUnit<
        ELECTRIC_PERMITTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "F/m",
                name: "farads per metre",
            }
        },
    > = SIUnit;

    /// Vacuum electric permittivity (`ε₀`), a unit of `ElectricPermittivity`: `n` of them is `n · 8.8541878128e-12` coherent units.
    pub const VACUUM_PERMITTIVITY: SIUnit<
        ELECTRIC_PERMITTIVITY,
        {
            SIUnitConsts {
                size: Exact::dec(88541878128, -22),
                zero: Exact::ZERO,
                symbol: "ε₀",
                name: "vacuum electric permittivity",
            }
        },
    > = SIUnit;

    /// Siemens (`S`), a unit of `ElectricalConductance`: `n` of them is `n · 1.0` coherent units.
    pub const SIEMENS: SIUnit<
        ELECTRICAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "S",
                name: "siemens",
            }
        },
    > = SIUnit;

    /// Mhos (`℧`), a unit of `ElectricalConductance`: `n` of them is `n · 1.0` coherent units.
    pub const MHO: SIUnit<
        ELECTRICAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "℧",
                name: "mhos",
            }
        },
    > = SIUnit;

    /// Abmhos (`abmho`), a unit of `ElectricalConductance`: `n` of them is `n · 1000000000.0` coherent units.
    pub const ABMHO: SIUnit<
        ELECTRICAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1000000000),
                zero: Exact::ZERO,
                symbol: "abmho",
                name: "abmhos",
            }
        },
    > = SIUnit;

    /// Abmsiemens (`abS`), a unit of `ElectricalConductance`: `n` of them is `n · 1000000000.0` coherent units.
    pub const ABSIEMENS: SIUnit<
        ELECTRICAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1000000000),
                zero: Exact::ZERO,
                symbol: "abS",
                name: "abmsiemens",
            }
        },
    > = SIUnit;

    /// Statsiemens (`statS`), a unit of `ElectricalConductance`: `n` of them is `n · 1.0 / (299.792458 * 2997924580.0)` coherent units.
    pub const STATSIEMENS: SIUnit<
        ELECTRICAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(299792458, -6) * Exact::int(2997924580)),
                zero: Exact::ZERO,
                symbol: "statS",
                name: "statsiemens",
            }
        },
    > = SIUnit;

    /// Statmhos (`statmho`), a unit of `ElectricalConductance`: `n` of them is `n · 1.0 / (299.792458 * 2997924580.0)` coherent units.
    pub const STATMHO: SIUnit<
        ELECTRICAL_CONDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(299792458, -6) * Exact::int(2997924580)),
                zero: Exact::ZERO,
                symbol: "statmho",
                name: "statmhos",
            }
        },
    > = SIUnit;

    /// Siemens per metre (`S/m`), a unit of `ElectricalConductivity`: `n` of them is `n · 1.0` coherent units.
    pub const SIEMENS_PER_METRE: SIUnit<
        ELECTRICAL_CONDUCTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "S/m",
                name: "siemens per metre",
            }
        },
    > = SIUnit;

    /// Square metres per volt second (`m²/(V · s)`), a unit of `ElectricalMobility`: `n` of them is `n · 1.0` coherent units.
    pub const SQUARE_METRE_PER_VOLT_SECOND: SIUnit<
        ELECTRICAL_MOBILITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m²/(V · s)",
                name: "square metres per volt second",
            }
        },
    > = SIUnit;

    /// Ohms (`Ω`), a unit of `ElectricalResistance`: `n` of them is `n · 1.0` coherent units.
    pub const OHM: SIUnit<
        ELECTRICAL_RESISTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Ω",
                name: "ohms",
            }
        },
    > = SIUnit;

    /// Abohms (`abΩ`), a unit of `ElectricalResistance`: `n` of them is `n · 1e-09` coherent units.
    pub const ABOHM: SIUnit<
        ELECTRICAL_RESISTANCE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -9),
                zero: Exact::ZERO,
                symbol: "abΩ",
                name: "abohms",
            }
        },
    > = SIUnit;

    /// Statohms (`statΩ`), a unit of `ElectricalResistance`: `n` of them is `n · 299.792458 * 2997924580.0` coherent units.
    pub const STATOHM: SIUnit<
        ELECTRICAL_RESISTANCE,
        {
            SIUnitConsts {
                size: Exact::dec(299792458, -6) * Exact::int(2997924580),
                zero: Exact::ZERO,
                symbol: "statΩ",
                name: "statohms",
            }
        },
    > = SIUnit;

    /// Ohm metres (`Ω · m`), a unit of `ElectricalResistivity`: `n` of them is `n · 1.0` coherent units.
    pub const OHM_METRE: SIUnit<
        ELECTRICAL_RESISTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Ω · m",
                name: "ohm metres",
            }
        },
    > = SIUnit;

    /// Abohm metres (`abΩ · m`), a unit of `ElectricalResistivity`: `n` of them is `n · 1e-09` coherent units.
    pub const ABOHM_METRE: SIUnit<
        ELECTRICAL_RESISTIVITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -9),
                zero: Exact::ZERO,
                symbol: "abΩ · m",
                name: "abohm metres",
            }
        },
    > = SIUnit;

    /// Statohm metres (`statΩ · m`), a unit of `ElectricalResistivity`: `n` of them is `n · 299.792458 * 2997924580.0` coherent units.
    pub const STATOHM_METRE: SIUnit<
        ELECTRICAL_RESISTIVITY,
        {
            SIUnitConsts {
                size: Exact::dec(299792458, -6) * Exact::int(2997924580),
                zero: Exact::ZERO,
                symbol: "statΩ · m",
                name: "statohm metres",
            }
        },
    > = SIUnit;

    /// Ohm inches (`Ω · in`), a unit of `ElectricalResistivity`: `n` of them is `n · 0.0254` coherent units.
    pub const OHM_INCH: SIUnit<
        ELECTRICAL_RESISTIVITY,
        {
            SIUnitConsts {
                size: Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "Ω · in",
                name: "ohm inches",
            }
        },
    > = SIUnit;

    /// Ohm feet (`Ω · ft`), a unit of `ElectricalResistivity`: `n` of them is `n · 0.3048` coherent units.
    pub const OHM_FOOT: SIUnit<
        ELECTRICAL_RESISTIVITY,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "Ω · ft",
                name: "ohm feet",
            }
        },
    > = SIUnit;

    /// Ohm yards (`Ω · yd`), a unit of `ElectricalResistivity`: `n` of them is `n · 0.9144` coherent units.
    pub const OHM_YARD: SIUnit<
        ELECTRICAL_RESISTIVITY,
        {
            SIUnitConsts {
                size: Exact::dec(9144, -4),
                zero: Exact::ZERO,
                symbol: "Ω · yd",
                name: "ohm yards",
            }
        },
    > = SIUnit;

    /// Ohm square millimetres per metre (`Ω · mm²/m`), a unit of `ElectricalResistivity`: `n` of them is `n · 1e-06` coherent units.
    pub const OHM_SQUARE_MILLIMETRE_PER_METRE: SIUnit<
        ELECTRICAL_RESISTIVITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -6),
                zero: Exact::ZERO,
                symbol: "Ω · mm²/m",
                name: "ohm square millimetres per metre",
            }
        },
    > = SIUnit;

    /// Farads (`F`), a unit of `Capacitance`: `n` of them is `n · 1.0` coherent units.
    pub const FARAD: SIUnit<
        CAPACITANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "F",
                name: "farads",
            }
        },
    > = SIUnit;

    /// Abfarads (`abF`), a unit of `Capacitance`: `n` of them is `n · 1000000000.0` coherent units.
    pub const ABFARAD: SIUnit<
        CAPACITANCE,
        {
            SIUnitConsts {
                size: Exact::int(1000000000),
                zero: Exact::ZERO,
                symbol: "abF",
                name: "abfarads",
            }
        },
    > = SIUnit;

    /// Statfarads (`statF`), a unit of `Capacitance`: `n` of them is `n · 1.0 / (299.792458 * 2997924580.0)` coherent units.
    pub const STATFARAD: SIUnit<
        CAPACITANCE,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(299792458, -6) * Exact::int(2997924580)),
                zero: Exact::ZERO,
                symbol: "statF",
                name: "statfarads",
            }
        },
    > = SIUnit;

    /// Henries (`H`), a unit of `Inductance`: `n` of them is `n · 1.0` coherent units.
    pub const HENRY: SIUnit<
        INDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "H",
                name: "henries",
            }
        },
    > = SIUnit;

    /// Abhenries (`abH`), a unit of `Inductance`: `n` of them is `n · 1e-09` coherent units.
    pub const ABHENRY: SIUnit<
        INDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -9),
                zero: Exact::ZERO,
                symbol: "abH",
                name: "abhenries",
            }
        },
    > = SIUnit;

    /// Stathenries (`statH`), a unit of `Inductance`: `n` of them is `n · 299.792458 * 2997924580.0` coherent units.
    pub const STATHENRY: SIUnit<
        INDUCTANCE,
        {
            SIUnitConsts {
                size: Exact::dec(299792458, -6) * Exact::int(2997924580),
                zero: Exact::ZERO,
                symbol: "statH",
                name: "stathenries",
            }
        },
    > = SIUnit;

    /// Amperes per metre (`A/m`), a unit of `MagneticFieldStrength`: `n` of them is `n · 1.0` coherent units.
    pub const MAGNETIC_FIELD_STRENGTH_AMPERE_PER_METRE: SIUnit<
        MAGNETIC_FIELD_STRENGTH,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "A/m",
                name: "amperes per metre",
            }
        },
    > = SIUnit;

    /// Oersteds (`Oe`), a unit of `MagneticFieldStrength`: `n` of them is `n · 1000.0 / (4.0 * core::f64::consts::PI)` coherent units.
    pub const OERSTED: SIUnit<
        MAGNETIC_FIELD_STRENGTH,
        {
            SIUnitConsts {
                size: Exact::int(1000) / (Exact::int(4) * Exact::PI),
                zero: Exact::ZERO,
                symbol: "Oe",
                name: "oersteds",
            }
        },
    > = SIUnit;

    /// Webers (`Wb`), a unit of `MagneticFlux`: `n` of them is `n · 1.0` coherent units.
    pub const WEBER: SIUnit<
        MAGNETIC_FLUX,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Wb",
                name: "webers",
            }
        },
    > = SIUnit;

    /// Maxwells (`Mx`), a unit of `MagneticFlux`: `n` of them is `n · 1e-08` coherent units.
    pub const MAXWELL: SIUnit<
        MAGNETIC_FLUX,
        {
            SIUnitConsts {
                size: Exact::dec(1, -8),
                zero: Exact::ZERO,
                symbol: "Mx",
                name: "maxwells",
            }
        },
    > = SIUnit;

    /// Teslas (`T`), a unit of `MagneticFluxDensity`: `n` of them is `n · 1.0` coherent units.
    pub const TESLA: SIUnit<
        MAGNETIC_FLUX_DENSITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "T",
                name: "teslas",
            }
        },
    > = SIUnit;

    /// Gammas (`γ`), a unit of `MagneticFluxDensity`: `n` of them is `n · 1e-09` coherent units.
    pub const GAMMA: SIUnit<
        MAGNETIC_FLUX_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -9),
                zero: Exact::ZERO,
                symbol: "γ",
                name: "gammas",
            }
        },
    > = SIUnit;

    /// Gauss (`G`), a unit of `MagneticFluxDensity`: `n` of them is `n · 0.0001` coherent units.
    pub const GAUSS: SIUnit<
        MAGNETIC_FLUX_DENSITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -4),
                zero: Exact::ZERO,
                symbol: "G",
                name: "gauss",
            }
        },
    > = SIUnit;

    /// Ampere square metres (`A · m²`), a unit of `MagneticMoment`: `n` of them is `n · 1.0` coherent units.
    pub const AMPERE_SQUARE_METRE: SIUnit<
        MAGNETIC_MOMENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "A · m²",
                name: "ampere square metres",
            }
        },
    > = SIUnit;

    /// Joules per tesla (`J/T`), a unit of `MagneticMoment`: `n` of them is `n · 1.0` coherent units.
    pub const JOULE_PER_TESLA: SIUnit<
        MAGNETIC_MOMENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/T",
                name: "joules per tesla",
            }
        },
    > = SIUnit;

    /// Newton metres per tesla (`N · m/T`), a unit of `MagneticMoment`: `n` of them is `n · 1.0` coherent units.
    pub const NEWTON_METRE_PER_TESLA: SIUnit<
        MAGNETIC_MOMENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "N · m/T",
                name: "newton metres per tesla",
            }
        },
    > = SIUnit;

    /// Ergs per gauss (`erg/G`), a unit of `MagneticMoment`: `n` of them is `n · 0.001` coherent units.
    pub const ERG_PER_GAUSS: SIUnit<
        MAGNETIC_MOMENT,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "erg/G",
                name: "ergs per gauss",
            }
        },
    > = SIUnit;

    /// Bohr magnetons (`µ(Bohr)`), a unit of `MagneticMoment`: `n` of them is `n · 9.2740100783e-24` coherent units.
    pub const BOHR_MAGNETON: SIUnit<
        MAGNETIC_MOMENT,
        {
            SIUnitConsts {
                size: Exact::dec(92740100783, -34),
                zero: Exact::ZERO,
                symbol: "µ(Bohr)",
                name: "Bohr magnetons",
            }
        },
    > = SIUnit;

    /// Nuclear magnetons (`μ(Nuclear)`), a unit of `MagneticMoment`: `n` of them is `n · 5.0507837461e-27` coherent units.
    pub const NUCLEAR_MAGNETON: SIUnit<
        MAGNETIC_MOMENT,
        {
            SIUnitConsts {
                size: Exact::dec(50507837461, -37),
                zero: Exact::ZERO,
                symbol: "μ(Nuclear)",
                name: "nuclear magnetons",
            }
        },
    > = SIUnit;

    ///  atomic units of magnetic dipole moment (`ħ · e/mₑ`), a unit of `MagneticMoment`: `n` of them is `n · 1.85480201566e-23` coherent units.
    pub const ATOMIC_UNIT_OF_MAGNETIC_DIPOLE_MOMENT: SIUnit<
        MAGNETIC_MOMENT,
        {
            SIUnitConsts {
                size: Exact::dec(185480201566, -34),
                zero: Exact::ZERO,
                symbol: "ħ · e/mₑ",
                name: " atomic units of magnetic dipole moment",
            }
        },
    > = SIUnit;

    /// Deuteron magnetic moments (`μ(deuteron)`), a unit of `MagneticMoment`: `n` of them is `n · 4.330735094e-27` coherent units.
    pub const DEUTERON_MAGNETIC_MOMENT: SIUnit<
        MAGNETIC_MOMENT,
        {
            SIUnitConsts {
                size: Exact::dec(4330735094, -36),
                zero: Exact::ZERO,
                symbol: "μ(deuteron)",
                name: "deuteron magnetic moments",
            }
        },
    > = SIUnit;

    /// Proton magnetic moments (`μ(proton)`), a unit of `MagneticMoment`: `n` of them is `n · 1.41060679736e-26` coherent units.
    pub const PROTON_MAGNETIC_MOMENT: SIUnit<
        MAGNETIC_MOMENT,
        {
            SIUnitConsts {
                size: Exact::dec(141060679736, -37),
                zero: Exact::ZERO,
                symbol: "μ(proton)",
                name: "proton magnetic moments",
            }
        },
    > = SIUnit;

    /// Shielded proton magnetic moments (`μ’(proton)`), a unit of `MagneticMoment`: `n` of them is `n · 1.41057056e-26` coherent units.
    pub const SHIELDED_PROTON_MAGNETIC_MOMENT: SIUnit<
        MAGNETIC_MOMENT,
        {
            SIUnitConsts {
                size: Exact::dec(141057056, -34),
                zero: Exact::ZERO,
                symbol: "μ’(proton)",
                name: "shielded proton magnetic moments",
            }
        },
    > = SIUnit;

    /// Triton magnetic moments (`μ(triton)`), a unit of `MagneticMoment`: `n` of them is `n · 1.5046095202e-26` coherent units.
    pub const TRITON_MAGNETIC_MOMENT: SIUnit<
        MAGNETIC_MOMENT,
        {
            SIUnitConsts {
                size: Exact::dec(15046095202, -36),
                zero: Exact::ZERO,
                symbol: "μ(triton)",
                name: "triton magnetic moments",
            }
        },
    > = SIUnit;

    /// Henrys per metre (`H/m`), a unit of `MagneticPermeability`: `n` of them is `n · 1.0` coherent units.
    pub const HENRY_PER_METRE: SIUnit<
        MAGNETIC_PERMEABILITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "H/m",
                name: "henrys per metre",
            }
        },
    > = SIUnit;

    /// Vacuum magnetic permeability (`µ₀`), a unit of `MagneticPermeability`: `n` of them is `n · 1.25663706212e-6` coherent units.
    pub const VACUUM_PERMEABILITY: SIUnit<
        MAGNETIC_PERMEABILITY,
        {
            SIUnitConsts {
                size: Exact::dec(125663706212, -17),
                zero: Exact::ZERO,
                symbol: "µ₀",
                name: "vacuum magnetic permeability",
            }
        },
    > = SIUnit;

    /// Hertz per tesla (`Hz/T`), a unit of `GyromagneticRatio`: `n` of them is `n · 1.0` coherent units.
    pub const HERTZ_PER_TESLA: SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Hz/T",
                name: "hertz per tesla",
            }
        },
    > = SIUnit;

    /// Radians per second tesla (`rad/(s · T)`), a unit of `GyromagneticRatio`: `n` of them is `n · 1.0` coherent units.
    pub const RADIAN_PER_SECOND_TESLA: SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "rad/(s · T)",
                name: "radians per second tesla",
            }
        },
    > = SIUnit;

    /// Proton gyromagnetic ratio (`γₚ`), a unit of `GyromagneticRatio`: `n` of them is `n · 267522187.08` coherent units.
    pub const PROTON_GYROMAGNETIC_RATIO: SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(26752218708, -2),
                zero: Exact::ZERO,
                symbol: "γₚ",
                name: "proton gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Neutron gyromagnetic ratio (`γₙ`), a unit of `GyromagneticRatio`: `n` of them is `n · 183247174.0` coherent units.
    pub const NEUTRON_GYROMAGNETIC_RATIO: SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::int(183247174),
                zero: Exact::ZERO,
                symbol: "γₙ",
                name: "neutron gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Electron gyromagnetic ratio (`γₑ`), a unit of `GyromagneticRatio`: `n` of them is `n · 176085962784.0` coherent units.
    pub const ELECTRON_GYROMAGNETIC_RATIO: SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::int(176085962784),
                zero: Exact::ZERO,
                symbol: "γₑ",
                name: "electron gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Shielded proton gyromagnetic ratio (`γ'ₚ`), a unit of `GyromagneticRatio`: `n` of them is `n · 267515319.4` coherent units.
    pub const SHIELDED_PROTON_GYROMAGNETIC_RATIO: SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(2675153194, -1),
                zero: Exact::ZERO,
                symbol: "γ'ₚ",
                name: "shielded proton gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Shielded helion gyromagnetic ratio (`γ'ₕ`), a unit of `GyromagneticRatio`: `n` of them is `n · 203789460.78` coherent units.
    pub const SHIELDED_HELION_GYROMAGNETIC_RATIO: SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(20378946078, -2),
                zero: Exact::ZERO,
                symbol: "γ'ₕ",
                name: "shielded helion gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Deuteron gyromagnetic ratio (`γ₂`), a unit of `GyromagneticRatio`: `n` of them is `n · 41066288.8` coherent units.
    pub const DEUTERON_GYROMAGNETIC_RATIO: SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(410662888, -1),
                zero: Exact::ZERO,
                symbol: "γ₂",
                name: "deuteron gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Triton gyromagnetic ratio (`γ₃`), a unit of `GyromagneticRatio`: `n` of them is `n · 285349844.0` coherent units.
    pub const TRITON_GYROMAGNETIC_RATIO: SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::int(285349844),
                zero: Exact::ZERO,
                symbol: "γ₃",
                name: "triton gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Lithium7 gyromagnetic ratio (`γ₇`), a unit of `GyromagneticRatio`: `n` of them is `n · 103975264.0` coherent units.
    pub const LITHIUM7_GYROMAGNETIC_RATIO: SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::int(103975264),
                zero: Exact::ZERO,
                symbol: "γ₇",
                name: "lithium7 gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Boron11 gyromagnetic ratio (`γ₁₁`), a unit of `GyromagneticRatio`: `n` of them is `n · 85838414.3` coherent units.
    pub const BORON11_GYROMAGNETIC_RATIO: SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(858384143, -1),
                zero: Exact::ZERO,
                symbol: "γ₁₁",
                name: "boron11 gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Carbon13 gyromagnetic ratio (`γ₁₃`), a unit of `GyromagneticRatio`: `n` of them is `n · 67278754.6` coherent units.
    pub const CARBON13_GYROMAGNETIC_RATIO: SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(672787546, -1),
                zero: Exact::ZERO,
                symbol: "γ₁₃",
                name: "carbon13 gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Nitrogen14 gyromagnetic ratio (`γ₁₄`), a unit of `GyromagneticRatio`: `n` of them is `n · 19328792.2` coherent units.
    pub const NITROGEN14_GYROMAGNETIC_RATIO: SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(193287922, -1),
                zero: Exact::ZERO,
                symbol: "γ₁₄",
                name: "nitrogen14 gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Flourine19 gyromagnetic ratio (`γ₁₉`), a unit of `GyromagneticRatio`: `n` of them is `n · 251762483.0` coherent units.
    pub const FLOURINE19_GYROMAGNETIC_RATIO: SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::int(251762483),
                zero: Exact::ZERO,
                symbol: "γ₁₉",
                name: "flourine19 gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Sodium23 gyromagnetic ratio (`γ₂₃`), a unit of `GyromagneticRatio`: `n` of them is `n · 70803541.7` coherent units.
    pub const SODIUM23_GYROMAGNETIC_RATIO: SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(708035417, -1),
                zero: Exact::ZERO,
                symbol: "γ₂₃",
                name: "sodium23 gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Phosphorus31 gyromagnetic ratio (`γ₃₁`), a unit of `GyromagneticRatio`: `n` of them is `n · 108329419.0` coherent units.
    pub const PHOSPHORUS31_GYROMAGNETIC_RATIO: SIUnit<
        GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::int(108329419),
                zero: Exact::ZERO,
                symbol: "γ₃₁",
                name: "phosphorus31 gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Proton reduced gyromagnetic ratio (`̅γₚ`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 42577478.461` coherent units.
    pub const PROTON_REDUCED_GYROMAGNETIC_RATIO: SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(42577478461, -3),
                zero: Exact::ZERO,
                symbol: "̅γₚ",
                name: "proton reduced gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Neutron reduced gyromagnetic ratio (`̅γₙ`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 29164693.5` coherent units.
    pub const NEUTRON_REDUCED_GYROMAGNETIC_RATIO: SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(291646935, -1),
                zero: Exact::ZERO,
                symbol: "̅γₙ",
                name: "neutron reduced gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Electron reduced gyromagnetic ratio (`̅γₑ`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 28024951386.1` coherent units.
    pub const ELECTRON_REDUCED_GYROMAGNETIC_RATIO: SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(280249513861, -1),
                zero: Exact::ZERO,
                symbol: "̅γₑ",
                name: "electron reduced gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Shielded proton reduced gyromagnetic ratio (`γ'ₚ`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 42576385.43` coherent units.
    pub const SHIELDED_PROTON_REDUCED_GYROMAGNETIC_RATIO: SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(4257638543, -2),
                zero: Exact::ZERO,
                symbol: "γ'ₚ",
                name: "shielded proton reduced gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Shielded helion reduced gyromagnetic ratio (`γ'ₕ`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 32434100.033` coherent units.
    pub const SHIELDED_HELION_REDUCED_GYROMAGNETIC_RATIO: SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(32434100033, -3),
                zero: Exact::ZERO,
                symbol: "γ'ₕ",
                name: "shielded helion reduced gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Deuteron reduced gyromagnetic ratio (`̅γ₂`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 6535902.85` coherent units.
    pub const DEUTERON_REDUCED_GYROMAGNETIC_RATIO: SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(653590285, -2),
                zero: Exact::ZERO,
                symbol: "̅γ₂",
                name: "deuteron reduced gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Triton reduced gyromagnetic ratio (`̅γ₃`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 45414838.1` coherent units.
    pub const TRITON_REDUCED_GYROMAGNETIC_RATIO: SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(454148381, -1),
                zero: Exact::ZERO,
                symbol: "̅γ₃",
                name: "triton reduced gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Lithium7 reduced gyromagnetic ratio (`̅γ₇`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 16548177.3` coherent units.
    pub const LITHIUM7_REDUCED_GYROMAGNETIC_RATIO: SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(165481773, -1),
                zero: Exact::ZERO,
                symbol: "̅γ₇",
                name: "lithium7 reduced gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Boron11 reduced gyromagnetic ratio (`̅γ₁₁`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 13661608.0` coherent units.
    pub const BORON11_REDUCED_GYROMAGNETIC_RATIO: SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::int(13661608),
                zero: Exact::ZERO,
                symbol: "̅γ₁₁",
                name: "boron11 reduced gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Carbon13 reduced gyromagnetic ratio (`̅γ₁₃`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 10707746.4` coherent units.
    pub const CARBON13_REDUCED_GYROMAGNETIC_RATIO: SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(107077464, -1),
                zero: Exact::ZERO,
                symbol: "̅γ₁₃",
                name: "carbon13 reduced gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Nitrogen14 reduced gyromagnetic ratio (`̅γ₁₄`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 3076272.81` coherent units.
    pub const NITROGEN14_REDUCED_GYROMAGNETIC_RATIO: SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(307627281, -2),
                zero: Exact::ZERO,
                symbol: "̅γ₁₄",
                name: "nitrogen14 reduced gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Flourine19 reduced gyromagnetic ratio (`̅γ₁₉`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 40069243.7` coherent units.
    pub const FLOURINE19_REDUCED_GYROMAGNETIC_RATIO: SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(400692437, -1),
                zero: Exact::ZERO,
                symbol: "̅γ₁₉",
                name: "flourine19 reduced gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Sodium23 reduced gyromagnetic ratio (`̅γ₂₃`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 11268733.6` coherent units.
    pub const SODIUM23_REDUCED_GYROMAGNETIC_RATIO: SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(112687336, -1),
                zero: Exact::ZERO,
                symbol: "̅γ₂₃",
                name: "sodium23 reduced gyromagnetic ratio",
            }
        },
    > = SIUnit;

    /// Phosphorus31 reduced gyromagnetic ratio (`̅γ₃₁`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 17241162.5` coherent units.
    pub const PHOSPHORUS31_REDUCED_GYROMAGNETIC_RATIO: SIUnit<
        REDUCED_GYROMAGNETIC_RATIO,
        {
            SIUnitConsts {
                size: Exact::dec(172411625, -1),
                zero: Exact::ZERO,
                symbol: "̅γ₃₁",
                name: "phosphorus31 reduced gyromagnetic ratio",
            }
        },
    > = SIUnit;
}
