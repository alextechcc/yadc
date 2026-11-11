use crate::{ISQ, StaticSIQuantity};
use consts::*;
use yadc::Dimension;

pub type ElectricCharge = StaticSIQuantity<
    {
        ISQ {
            s: 1,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ElectricChargeArealDensity = StaticSIQuantity<
    {
        ISQ {
            m: -2,
            s: 1,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ElectricChargeLinearDensity = StaticSIQuantity<
    {
        ISQ {
            m: -1,
            s: 1,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ElectricChargeVolumetricDensity = StaticSIQuantity<
    {
        ISQ {
            m: -3,
            s: 1,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ElectricCurrent = StaticSIQuantity<
    {
        ISQ {
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ElectricCurrentDensity = StaticSIQuantity<
    {
        ISQ {
            m: -2,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type SurfaceElectricCurrentDensity = StaticSIQuantity<
    {
        ISQ {
            m: -1,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ElectricPotential = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -3,
            A: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ElectricField = StaticSIQuantity<
    {
        ISQ {
            m: 1,
            kg: 1,
            s: -3,
            A: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ElectricDisplacementField = StaticSIQuantity<
    {
        ISQ {
            m: -2,
            s: 1,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ElectricFlux = StaticSIQuantity<
    {
        ISQ {
            m: 3,
            kg: 1,
            s: -3,
            A: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ElectricDipoleMoment = StaticSIQuantity<
    {
        ISQ {
            m: 1,
            s: 1,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ElectricQuadrupoleMoment = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            s: 1,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ElectricPermittivity = StaticSIQuantity<
    {
        ISQ {
            m: -3,
            kg: -1,
            s: 4,
            A: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ElectricalConductance = StaticSIQuantity<
    {
        ISQ {
            m: -2,
            kg: -1,
            s: 3,
            A: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ElectricalConductivity = StaticSIQuantity<
    {
        ISQ {
            m: -3,
            kg: -1,
            s: 3,
            A: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ElectricalMobility = StaticSIQuantity<
    {
        ISQ {
            kg: -1,
            s: 2,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ElectricalResistance = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -3,
            A: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ElectricalResistivity = StaticSIQuantity<
    {
        ISQ {
            m: 3,
            kg: 1,
            s: -3,
            A: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type Capacitance = StaticSIQuantity<
    {
        ISQ {
            m: -2,
            kg: -1,
            s: 4,
            A: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type Inductance = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -2,
            A: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type MagneticFieldStrength = StaticSIQuantity<
    {
        ISQ {
            m: -1,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type MagneticFlux = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -2,
            A: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type MagneticFluxDensity = StaticSIQuantity<
    {
        ISQ {
            kg: 1,
            s: -2,
            A: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type MagneticMoment = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type MagneticPermeability = StaticSIQuantity<
    {
        ISQ {
            m: 1,
            kg: 1,
            s: -2,
            A: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type GyromagneticRatio = StaticSIQuantity<
    {
        ISQ {
            kg: -1,
            s: 1,
            A: 1,
            rad: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ReducedGyromagneticRatio = StaticSIQuantity<
    {
        ISQ {
            kg: -1,
            s: 1,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

/// The constants of this module: the base constants of its quantities
/// (re-exported), then one per multiple, each defined from them - `KILOMETRE`
/// is `1000.0 * METRE`, a `Length` and a unit of its own (`KILOMETRE.value(d)`).
/// Quantity arithmetic is `const`, so they compose in `const` items:
/// `const LIMIT: Velocity = 50.0 * KILOMETRE / HOUR;`.
pub mod consts {
    use super::*;
    use crate::base::{KILOGRAM, METRE, RADIAN, SECOND};

    /// The ampere, the SI base unit of electric current.
    pub use crate::base::AMPERE;

    /// One `Yottacoulomb` (YC), as an `ElectricCharge`.
    pub const YOTTACOULOMB: ElectricCharge = 9.9999999999999998e+23 * COULOMB;

    /// One `Zettacoulomb` (ZC), as an `ElectricCharge`.
    pub const ZETTACOULOMB: ElectricCharge = 1e+21 * COULOMB;

    /// One `Exacoulomb` (EC), as an `ElectricCharge`.
    pub const EXACOULOMB: ElectricCharge = 1e+18 * COULOMB;

    /// One `Petacoulomb` (PC), as an `ElectricCharge`.
    pub const PETACOULOMB: ElectricCharge = 1000000000000000.0 * COULOMB;

    /// One `Teracoulomb` (TC), as an `ElectricCharge`.
    pub const TERACOULOMB: ElectricCharge = 1000000000000.0 * COULOMB;

    /// One `Gigacoulomb` (GC), as an `ElectricCharge`.
    pub const GIGACOULOMB: ElectricCharge = 1000000000.0 * COULOMB;

    /// One `Megacoulomb` (MC), as an `ElectricCharge`.
    pub const MEGACOULOMB: ElectricCharge = 1000000.0 * COULOMB;

    /// One `Kilocoulomb` (kC), as an `ElectricCharge`.
    pub const KILOCOULOMB: ElectricCharge = 1000.0 * COULOMB;

    /// One `Hectocoulomb` (hC), as an `ElectricCharge`.
    pub const HECTOCOULOMB: ElectricCharge = 100.0 * COULOMB;

    /// One `Decacoulomb` (daC), as an `ElectricCharge`.
    pub const DECACOULOMB: ElectricCharge = 10.0 * COULOMB;

    /// One `Coulomb` (C), as an `ElectricCharge`.
    pub const COULOMB: ElectricCharge = SECOND * AMPERE;

    /// One `Decicoulomb` (dC), as an `ElectricCharge`.
    pub const DECICOULOMB: ElectricCharge = 0.10000000000000001 * COULOMB;

    /// One `Centicoulomb` (cC), as an `ElectricCharge`.
    pub const CENTICOULOMB: ElectricCharge = 0.01 * COULOMB;

    /// One `Millicoulomb` (mC), as an `ElectricCharge`.
    pub const MILLICOULOMB: ElectricCharge = 0.001 * COULOMB;

    /// One `Microcoulomb` (µC), as an `ElectricCharge`.
    pub const MICROCOULOMB: ElectricCharge = 9.9999999999999995e-07 * COULOMB;

    /// One `Nanocoulomb` (nC), as an `ElectricCharge`.
    pub const NANOCOULOMB: ElectricCharge = 1.0000000000000001e-09 * COULOMB;

    /// One `Picocoulomb` (pC), as an `ElectricCharge`.
    pub const PICOCOULOMB: ElectricCharge = 9.9999999999999998e-13 * COULOMB;

    /// One `Femtocoulomb` (fC), as an `ElectricCharge`.
    pub const FEMTOCOULOMB: ElectricCharge = 1.0000000000000001e-15 * COULOMB;

    /// One `Attocoulomb` (aC), as an `ElectricCharge`.
    pub const ATTOCOULOMB: ElectricCharge = 1.0000000000000001e-18 * COULOMB;

    /// One `Zeptocoulomb` (zC), as an `ElectricCharge`.
    pub const ZEPTOCOULOMB: ElectricCharge = 9.9999999999999991e-22 * COULOMB;

    /// One `Yoctocoulomb` (yC), as an `ElectricCharge`.
    pub const YOCTOCOULOMB: ElectricCharge = 9.9999999999999992e-25 * COULOMB;

    /// One `PetaampereHour` (PA · h), as an `ElectricCharge`.
    pub const PETAAMPERE_HOUR: ElectricCharge = 3.6e+18 * COULOMB;

    /// One `TeraampereHour` (TA · h), as an `ElectricCharge`.
    pub const TERAAMPERE_HOUR: ElectricCharge = 3600000000000000.0 * COULOMB;

    /// One `GigaampereHour` (GA · h), as an `ElectricCharge`.
    pub const GIGAAMPERE_HOUR: ElectricCharge = 3600000000000.0 * COULOMB;

    /// One `MegaampereHour` (MA · h), as an `ElectricCharge`.
    pub const MEGAAMPERE_HOUR: ElectricCharge = 3600000000.0 * COULOMB;

    /// One `KiloampereHour` (kA · h), as an `ElectricCharge`.
    pub const KILOAMPERE_HOUR: ElectricCharge = 3600000.0 * COULOMB;

    /// One `HectoampereHour` (hA · h), as an `ElectricCharge`.
    pub const HECTOAMPERE_HOUR: ElectricCharge = 360000.0 * COULOMB;

    /// One `DecaampereHour` (daA · h), as an `ElectricCharge`.
    pub const DECAAMPERE_HOUR: ElectricCharge = 36000.0 * COULOMB;

    /// One `AmpereHour` (A · h), as an `ElectricCharge`.
    pub const AMPERE_HOUR: ElectricCharge = 3600.0 * COULOMB;

    /// One `MilliampereHour` (mA · h), as an `ElectricCharge`.
    pub const MILLIAMPERE_HOUR: ElectricCharge = 3.6000000000000001 * COULOMB;

    /// One `MicroampereHour` (µA · h), as an `ElectricCharge`.
    pub const MICROAMPERE_HOUR: ElectricCharge = 0.0035999999999999999 * COULOMB;

    /// One `ElementaryCharge` (e), as an `ElectricCharge`.
    pub const ELEMENTARY_CHARGE: ElectricCharge = 1.602176634e-19 * COULOMB;

    /// One `AtomicUnitOfCharge` (a.u. of charge), as an `ElectricCharge`.
    pub const ATOMIC_UNIT_OF_CHARGE: ElectricCharge = 1.6021766339999999e-19 * COULOMB;

    /// One `Abcoulomb` (abC), as an `ElectricCharge`.
    pub const ABCOULOMB: ElectricCharge = 10.0 * COULOMB;

    /// One `Faraday` (F), as an `ElectricCharge`.
    pub const FARADAY: ElectricCharge = 96485.309999999998 * COULOMB;

    /// One `Franklin` (Fr), as an `ElectricCharge`.
    pub const FRANKLIN: ElectricCharge = 3.335641e-10 * COULOMB;

    /// One `Statcoulomb` (statC), as an `ElectricCharge`.
    pub const STATCOULOMB: ElectricCharge = 3.335641e-10 * COULOMB;

    /// One `CoulombPerSquareMetre` (C/m²), as an `ElectricChargeArealDensity`.
    pub const ELECTRIC_CHARGE_AREAL_DENSITY_COULOMB_PER_SQUARE_METRE: ElectricChargeArealDensity =
        SECOND * AMPERE / (METRE * METRE);

    /// One `CoulombPerMetre` (C/m), as an `ElectricChargeLinearDensity`.
    pub const COULOMB_PER_METRE: ElectricChargeLinearDensity = SECOND * AMPERE / METRE;

    /// One `CoulombPerCentimetre` (C/cm), as an `ElectricChargeLinearDensity`.
    pub const COULOMB_PER_CENTIMETRE: ElectricChargeLinearDensity = 100.0 * COULOMB_PER_METRE;

    /// One `CoulombPerCubicMetre` (C/m³), as an `ElectricChargeVolumetricDensity`.
    pub const COULOMB_PER_CUBIC_METRE: ElectricChargeVolumetricDensity =
        SECOND * AMPERE / (METRE * METRE * METRE);

    /// One `Yottaampere` (YA), as an `ElectricCurrent`.
    pub const YOTTAAMPERE: ElectricCurrent = 9.9999999999999998e+23 * AMPERE;

    /// One `Zettaampere` (ZA), as an `ElectricCurrent`.
    pub const ZETTAAMPERE: ElectricCurrent = 1e+21 * AMPERE;

    /// One `Exaampere` (EA), as an `ElectricCurrent`.
    pub const EXAAMPERE: ElectricCurrent = 1e+18 * AMPERE;

    /// One `Petaampere` (PA), as an `ElectricCurrent`.
    pub const PETAAMPERE: ElectricCurrent = 1000000000000000.0 * AMPERE;

    /// One `Teraampere` (TA), as an `ElectricCurrent`.
    pub const TERAAMPERE: ElectricCurrent = 1000000000000.0 * AMPERE;

    /// One `Gigaampere` (GA), as an `ElectricCurrent`.
    pub const GIGAAMPERE: ElectricCurrent = 1000000000.0 * AMPERE;

    /// One `Megaampere` (MA), as an `ElectricCurrent`.
    pub const MEGAAMPERE: ElectricCurrent = 1000000.0 * AMPERE;

    /// One `Kiloampere` (kA), as an `ElectricCurrent`.
    pub const KILOAMPERE: ElectricCurrent = 1000.0 * AMPERE;

    /// One `Hectoampere` (hA), as an `ElectricCurrent`.
    pub const HECTOAMPERE: ElectricCurrent = 100.0 * AMPERE;

    /// One `Decaampere` (daA), as an `ElectricCurrent`.
    pub const DECAAMPERE: ElectricCurrent = 10.0 * AMPERE;

    /// One `Deciampere` (dA), as an `ElectricCurrent`.
    pub const DECIAMPERE: ElectricCurrent = 0.10000000000000001 * AMPERE;

    /// One `Centiampere` (cA), as an `ElectricCurrent`.
    pub const CENTIAMPERE: ElectricCurrent = 0.01 * AMPERE;

    /// One `Milliampere` (mA), as an `ElectricCurrent`.
    pub const MILLIAMPERE: ElectricCurrent = 0.001 * AMPERE;

    /// One `Microampere` (µA), as an `ElectricCurrent`.
    pub const MICROAMPERE: ElectricCurrent = 9.9999999999999995e-07 * AMPERE;

    /// One `Nanoampere` (nA), as an `ElectricCurrent`.
    pub const NANOAMPERE: ElectricCurrent = 1.0000000000000001e-09 * AMPERE;

    /// One `Picoampere` (pA), as an `ElectricCurrent`.
    pub const PICOAMPERE: ElectricCurrent = 9.9999999999999998e-13 * AMPERE;

    /// One `Femtoampere` (fA), as an `ElectricCurrent`.
    pub const FEMTOAMPERE: ElectricCurrent = 1.0000000000000001e-15 * AMPERE;

    /// One `Attoampere` (aA), as an `ElectricCurrent`.
    pub const ATTOAMPERE: ElectricCurrent = 1.0000000000000001e-18 * AMPERE;

    /// One `Zeptoampere` (zA), as an `ElectricCurrent`.
    pub const ZEPTOAMPERE: ElectricCurrent = 9.9999999999999991e-22 * AMPERE;

    /// One `Yoctoampere` (yA), as an `ElectricCurrent`.
    pub const YOCTOAMPERE: ElectricCurrent = 9.9999999999999992e-25 * AMPERE;

    /// One `ElementaryChargePerSecond` (e/s), as an `ElectricCurrent`.
    pub const ELEMENTARY_CHARGE_PER_SECOND: ElectricCurrent = 1.6021766339999999e-19 * AMPERE;

    /// One `AtomicUnitOfChargePerSecond` (a.u. of charge/s), as an `ElectricCurrent`.
    pub const ATOMIC_UNIT_OF_CHARGE_PER_SECOND: ElectricCurrent = 1.6021766339999999e-19 * AMPERE;

    /// One `Abampere` (abA), as an `ElectricCurrent`.
    pub const ABAMPERE: ElectricCurrent = 10.0 * AMPERE;

    /// One `Gilbert` (Gi), as an `ElectricCurrent`.
    pub const GILBERT: ElectricCurrent = 0.79577469999999995 * AMPERE;

    /// One `Statampere` (statA), as an `ElectricCurrent`.
    pub const STATAMPERE: ElectricCurrent = 3.335641e-10 * AMPERE;

    /// One `AmperePerSquareMetre` (A/m²), as an `ElectricCurrentDensity`.
    pub const AMPERE_PER_SQUARE_METRE: ElectricCurrentDensity = AMPERE / (METRE * METRE);

    /// One `AmperePerMetre` (A/m), as a `SurfaceElectricCurrentDensity`.
    pub const SURFACE_ELECTRIC_CURRENT_DENSITY_AMPERE_PER_METRE: SurfaceElectricCurrentDensity =
        AMPERE / METRE;

    /// One `AmperePerCentimetre` (A/cm), as a `SurfaceElectricCurrentDensity`.
    pub const SURFACE_ELECTRIC_CURRENT_DENSITY_AMPERE_PER_CENTIMETRE:
        SurfaceElectricCurrentDensity = 100.0 * SURFACE_ELECTRIC_CURRENT_DENSITY_AMPERE_PER_METRE;

    /// One `AmperePerMillimetre` (A/mm), as a `SurfaceElectricCurrentDensity`.
    pub const SURFACE_ELECTRIC_CURRENT_DENSITY_AMPERE_PER_MILLIMETRE:
        SurfaceElectricCurrentDensity = 1000.0 * SURFACE_ELECTRIC_CURRENT_DENSITY_AMPERE_PER_METRE;

    /// One `AmperePerMicrometre` (A/μm), as a `SurfaceElectricCurrentDensity`.
    pub const SURFACE_ELECTRIC_CURRENT_DENSITY_AMPERE_PER_MICROMETRE:
        SurfaceElectricCurrentDensity =
        1000000.0 * SURFACE_ELECTRIC_CURRENT_DENSITY_AMPERE_PER_METRE;

    /// One `Yottavolt` (YV), as an `ElectricPotential`.
    pub const YOTTAVOLT: ElectricPotential = 9.9999999999999998e+23 * VOLT;

    /// One `Zettavolt` (ZV), as an `ElectricPotential`.
    pub const ZETTAVOLT: ElectricPotential = 1e+21 * VOLT;

    /// One `Exavolt` (EV), as an `ElectricPotential`.
    pub const EXAVOLT: ElectricPotential = 1e+18 * VOLT;

    /// One `Petavolt` (PV), as an `ElectricPotential`.
    pub const PETAVOLT: ElectricPotential = 1000000000000000.0 * VOLT;

    /// One `Teravolt` (TV), as an `ElectricPotential`.
    pub const TERAVOLT: ElectricPotential = 1000000000000.0 * VOLT;

    /// One `Gigavolt` (GV), as an `ElectricPotential`.
    pub const GIGAVOLT: ElectricPotential = 1000000000.0 * VOLT;

    /// One `Megavolt` (MV), as an `ElectricPotential`.
    pub const MEGAVOLT: ElectricPotential = 1000000.0 * VOLT;

    /// One `Kilovolt` (kV), as an `ElectricPotential`.
    pub const KILOVOLT: ElectricPotential = 1000.0 * VOLT;

    /// One `Hectovolt` (hV), as an `ElectricPotential`.
    pub const HECTOVOLT: ElectricPotential = 100.0 * VOLT;

    /// One `Decavolt` (daV), as an `ElectricPotential`.
    pub const DECAVOLT: ElectricPotential = 10.0 * VOLT;

    /// One `Volt` (V), as an `ElectricPotential`.
    pub const VOLT: ElectricPotential =
        METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * AMPERE);

    /// One `Decivolt` (dV), as an `ElectricPotential`.
    pub const DECIVOLT: ElectricPotential = 0.10000000000000001 * VOLT;

    /// One `Centivolt` (cV), as an `ElectricPotential`.
    pub const CENTIVOLT: ElectricPotential = 0.01 * VOLT;

    /// One `Millivolt` (mV), as an `ElectricPotential`.
    pub const MILLIVOLT: ElectricPotential = 0.001 * VOLT;

    /// One `Microvolt` (µV), as an `ElectricPotential`.
    pub const MICROVOLT: ElectricPotential = 9.9999999999999995e-07 * VOLT;

    /// One `Nanovolt` (nV), as an `ElectricPotential`.
    pub const NANOVOLT: ElectricPotential = 1.0000000000000001e-09 * VOLT;

    /// One `Picovolt` (pV), as an `ElectricPotential`.
    pub const PICOVOLT: ElectricPotential = 9.9999999999999998e-13 * VOLT;

    /// One `Femtovolt` (fV), as an `ElectricPotential`.
    pub const FEMTOVOLT: ElectricPotential = 1.0000000000000001e-15 * VOLT;

    /// One `Attovolt` (aV), as an `ElectricPotential`.
    pub const ATTOVOLT: ElectricPotential = 1.0000000000000001e-18 * VOLT;

    /// One `Zeptovolt` (zV), as an `ElectricPotential`.
    pub const ZEPTOVOLT: ElectricPotential = 9.9999999999999991e-22 * VOLT;

    /// One `Yoctovolt` (yV), as an `ElectricPotential`.
    pub const YOCTOVOLT: ElectricPotential = 9.9999999999999992e-25 * VOLT;

    /// One `Abvolt` (abV), as an `ElectricPotential`.
    pub const ABVOLT: ElectricPotential = 1e-08 * VOLT;

    /// One `Statvolt` (statV), as an `ElectricPotential`.
    pub const STATVOLT: ElectricPotential = 299.79250000000002 * VOLT;

    /// One `VoltPerMetre` (V/m), as an `ElectricField`.
    pub const VOLT_PER_METRE: ElectricField =
        METRE * KILOGRAM / (SECOND * SECOND * SECOND * AMPERE);

    /// One `VoltPerCentimetre` (V/cm), as an `ElectricField`.
    pub const VOLT_PER_CENTIMETRE: ElectricField = 100.0 * VOLT_PER_METRE;

    /// One `VoltPerMillimetre` (V/mm), as an `ElectricField`.
    pub const VOLT_PER_MILLIMETRE: ElectricField = 1000.0 * VOLT_PER_METRE;

    /// One `VoltPerMicrometre` (V/μm), as an `ElectricField`.
    pub const VOLT_PER_MICROMETRE: ElectricField = 1000000.0 * VOLT_PER_METRE;

    /// One `KilovoltPerMillimetre` (kV/mm), as an `ElectricField`.
    pub const KILOVOLT_PER_MILLIMETRE: ElectricField = 1000000.0 * VOLT_PER_METRE;

    /// One `MegavoltPerMetre` (MV/m), as an `ElectricField`.
    pub const MEGAVOLT_PER_METRE: ElectricField = 1000000.0 * VOLT_PER_METRE;

    /// One `MegavoltPerCentimetre` (MV/cm), as an `ElectricField`.
    pub const MEGAVOLT_PER_CENTIMETRE: ElectricField = 100000000.0 * VOLT_PER_METRE;

    /// One `VoltPerMil` (V/mil), as an `ElectricField`.
    pub const VOLT_PER_MIL: ElectricField = 39370.078740157478 * VOLT_PER_METRE;

    /// One `AtomicUnitOfElectricField` (a.u. of electric field), as an `ElectricField`.
    pub const ATOMIC_UNIT_OF_ELECTRIC_FIELD: ElectricField = 514220674763.25952 * VOLT_PER_METRE;

    /// One `CoulombPerSquareMetre` (C/m²), as an `ElectricDisplacementField`.
    pub const ELECTRIC_DISPLACEMENT_FIELD_COULOMB_PER_SQUARE_METRE: ElectricDisplacementField =
        SECOND * AMPERE / (METRE * METRE);

    /// One `VoltMetre` (V ⋅ m), as an `ElectricFlux`.
    pub const VOLT_METRE: ElectricFlux =
        METRE * METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * AMPERE);

    /// One `VoltCentimetre` (V ⋅ cm), as an `ElectricFlux`.
    pub const VOLT_CENTIMETRE: ElectricFlux = 0.01 * VOLT_METRE;

    /// One `CoulombMetre` (C · m), as an `ElectricDipoleMoment`.
    pub const COULOMB_METRE: ElectricDipoleMoment = METRE * SECOND * AMPERE;

    /// One `AtomicUnitOfChargeCentimetre` (a.u. of charge · cm), as an `ElectricDipoleMoment`.
    pub const ATOMIC_UNIT_OF_CHARGE_CENTIMETRE: ElectricDipoleMoment =
        1.6021766339999998e-21 * COULOMB_METRE;

    /// One `ElementaryChargeCentimetre` (e · cm), as an `ElectricDipoleMoment`.
    pub const ELEMENTARY_CHARGE_CENTIMETRE: ElectricDipoleMoment =
        1.6021766339999998e-21 * COULOMB_METRE;

    /// One `AtomicUnitOfElectricDipoleMoment` (e · a₀), as an `ElectricDipoleMoment`.
    pub const ATOMIC_UNIT_OF_ELECTRIC_DIPOLE_MOMENT: ElectricDipoleMoment =
        8.478353625540766e-30 * COULOMB_METRE;

    /// One `CoulombSquareMetre` (C · m²), as an `ElectricQuadrupoleMoment`.
    pub const COULOMB_SQUARE_METRE: ElectricQuadrupoleMoment = METRE * METRE * SECOND * AMPERE;

    /// One `ElementaryChargeBarn` (e · b), as an `ElectricQuadrupoleMoment`.
    pub const ELEMENTARY_CHARGE_BARN: ElectricQuadrupoleMoment =
        1.6021766339999998e-47 * COULOMB_SQUARE_METRE;

    /// One `AtomicUnitOfElectricQuadrupoleMoment` (e · a₀²), as an `ElectricQuadrupoleMoment`.
    pub const ATOMIC_UNIT_OF_ELECTRIC_QUADRUPOLE_MOMENT: ElectricQuadrupoleMoment =
        4.4865515246129999e-40 * COULOMB_SQUARE_METRE;

    /// One `FaradPerMetre` (F/m), as an `ElectricPermittivity`.
    pub const FARAD_PER_METRE: ElectricPermittivity =
        SECOND * SECOND * SECOND * SECOND * AMPERE * AMPERE / (METRE * METRE * METRE * KILOGRAM);

    /// One `VacuumPermittivity` (ε₀), as an `ElectricPermittivity`.
    pub const VACUUM_PERMITTIVITY: ElectricPermittivity = 8.8541878128e-12 * FARAD_PER_METRE;

    /// One `Yottasiemens` (YS), as an `ElectricalConductance`.
    pub const YOTTASIEMENS: ElectricalConductance = 9.9999999999999998e+23 * SIEMENS;

    /// One `Zettasiemens` (ZS), as an `ElectricalConductance`.
    pub const ZETTASIEMENS: ElectricalConductance = 1e+21 * SIEMENS;

    /// One `Exasiemens` (ES), as an `ElectricalConductance`.
    pub const EXASIEMENS: ElectricalConductance = 1e+18 * SIEMENS;

    /// One `Petasiemens` (PS), as an `ElectricalConductance`.
    pub const PETASIEMENS: ElectricalConductance = 1000000000000000.0 * SIEMENS;

    /// One `Terasiemens` (TS), as an `ElectricalConductance`.
    pub const TERASIEMENS: ElectricalConductance = 1000000000000.0 * SIEMENS;

    /// One `Gigasiemens` (GS), as an `ElectricalConductance`.
    pub const GIGASIEMENS: ElectricalConductance = 1000000000.0 * SIEMENS;

    /// One `Megasiemens` (MS), as an `ElectricalConductance`.
    pub const MEGASIEMENS: ElectricalConductance = 1000000.0 * SIEMENS;

    /// One `Kilosiemens` (kS), as an `ElectricalConductance`.
    pub const KILOSIEMENS: ElectricalConductance = 1000.0 * SIEMENS;

    /// One `Hectosiemens` (hS), as an `ElectricalConductance`.
    pub const HECTOSIEMENS: ElectricalConductance = 100.0 * SIEMENS;

    /// One `Decasiemens` (daS), as an `ElectricalConductance`.
    pub const DECASIEMENS: ElectricalConductance = 10.0 * SIEMENS;

    /// One `Siemens` (S), as an `ElectricalConductance`.
    pub const SIEMENS: ElectricalConductance =
        SECOND * SECOND * SECOND * AMPERE * AMPERE / (METRE * METRE * KILOGRAM);

    /// One `Mho` (℧), as an `ElectricalConductance`.
    pub const MHO: ElectricalConductance = SIEMENS;

    /// One `Decisiemens` (dS), as an `ElectricalConductance`.
    pub const DECISIEMENS: ElectricalConductance = 0.10000000000000001 * SIEMENS;

    /// One `Centisiemens` (cS), as an `ElectricalConductance`.
    pub const CENTISIEMENS: ElectricalConductance = 0.01 * SIEMENS;

    /// One `Millisiemens` (mS), as an `ElectricalConductance`.
    pub const MILLISIEMENS: ElectricalConductance = 0.001 * SIEMENS;

    /// One `Microsiemens` (µS), as an `ElectricalConductance`.
    pub const MICROSIEMENS: ElectricalConductance = 9.9999999999999995e-07 * SIEMENS;

    /// One `Nanosiemens` (nS), as an `ElectricalConductance`.
    pub const NANOSIEMENS: ElectricalConductance = 1.0000000000000001e-09 * SIEMENS;

    /// One `Picosiemens` (pS), as an `ElectricalConductance`.
    pub const PICOSIEMENS: ElectricalConductance = 9.9999999999999998e-13 * SIEMENS;

    /// One `Femtosiemens` (fS), as an `ElectricalConductance`.
    pub const FEMTOSIEMENS: ElectricalConductance = 1.0000000000000001e-15 * SIEMENS;

    /// One `Attosiemens` (aS), as an `ElectricalConductance`.
    pub const ATTOSIEMENS: ElectricalConductance = 1.0000000000000001e-18 * SIEMENS;

    /// One `Zeptosiemens` (zS), as an `ElectricalConductance`.
    pub const ZEPTOSIEMENS: ElectricalConductance = 9.9999999999999991e-22 * SIEMENS;

    /// One `Yoctosiemens` (yS), as an `ElectricalConductance`.
    pub const YOCTOSIEMENS: ElectricalConductance = 9.9999999999999992e-25 * SIEMENS;

    /// One `Abmho` (abmho), as an `ElectricalConductance`.
    pub const ABMHO: ElectricalConductance = 1000000000.0 * SIEMENS;

    /// One `Absiemens` (abS), as an `ElectricalConductance`.
    pub const ABSIEMENS: ElectricalConductance = 1000000000.0 * SIEMENS;

    /// One `Statsiemens` (statS), as an `ElectricalConductance`.
    pub const STATSIEMENS: ElectricalConductance = 1.1126499999999999e-12 * SIEMENS;

    /// One `Statmho` (statmho), as an `ElectricalConductance`.
    pub const STATMHO: ElectricalConductance = 1.1126499999999999e-12 * SIEMENS;

    /// One `SiemensPerMetre` (S/m), as an `ElectricalConductivity`.
    pub const SIEMENS_PER_METRE: ElectricalConductivity =
        SECOND * SECOND * SECOND * AMPERE * AMPERE / (METRE * METRE * METRE * KILOGRAM);

    /// One `SiemensPerCentimetre` (S/cm), as an `ElectricalConductivity`.
    pub const SIEMENS_PER_CENTIMETRE: ElectricalConductivity = 100.0 * SIEMENS_PER_METRE;

    /// One `SquareMetrePerVoltSecond` (m²/(V · s)), as an `ElectricalMobility`.
    pub const SQUARE_METRE_PER_VOLT_SECOND: ElectricalMobility =
        SECOND * SECOND * AMPERE / KILOGRAM;

    /// One `SquareCentimetrePerVoltSecond` (cm²/(V · s)), as an `ElectricalMobility`.
    pub const SQUARE_CENTIMETRE_PER_VOLT_SECOND: ElectricalMobility =
        0.0001 * SQUARE_METRE_PER_VOLT_SECOND;

    /// One `Yottaohm` (YΩ), as an `ElectricalResistance`.
    pub const YOTTAOHM: ElectricalResistance = 9.9999999999999998e+23 * OHM;

    /// One `Zettaohm` (ZΩ), as an `ElectricalResistance`.
    pub const ZETTAOHM: ElectricalResistance = 1e+21 * OHM;

    /// One `Exaohm` (EΩ), as an `ElectricalResistance`.
    pub const EXAOHM: ElectricalResistance = 1e+18 * OHM;

    /// One `Petaohm` (PΩ), as an `ElectricalResistance`.
    pub const PETAOHM: ElectricalResistance = 1000000000000000.0 * OHM;

    /// One `Teraohm` (TΩ), as an `ElectricalResistance`.
    pub const TERAOHM: ElectricalResistance = 1000000000000.0 * OHM;

    /// One `Gigaohm` (GΩ), as an `ElectricalResistance`.
    pub const GIGAOHM: ElectricalResistance = 1000000000.0 * OHM;

    /// One `Megaohm` (MΩ), as an `ElectricalResistance`.
    pub const MEGAOHM: ElectricalResistance = 1000000.0 * OHM;

    /// One `Kiloohm` (kΩ), as an `ElectricalResistance`.
    pub const KILOOHM: ElectricalResistance = 1000.0 * OHM;

    /// One `Hectoohm` (hΩ), as an `ElectricalResistance`.
    pub const HECTOOHM: ElectricalResistance = 100.0 * OHM;

    /// One `Decaohm` (daΩ), as an `ElectricalResistance`.
    pub const DECAOHM: ElectricalResistance = 10.0 * OHM;

    /// One `Ohm` (Ω), as an `ElectricalResistance`.
    pub const OHM: ElectricalResistance =
        METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * AMPERE * AMPERE);

    /// One `Deciohm` (dΩ), as an `ElectricalResistance`.
    pub const DECIOHM: ElectricalResistance = 0.10000000000000001 * OHM;

    /// One `Centiohm` (cΩ), as an `ElectricalResistance`.
    pub const CENTIOHM: ElectricalResistance = 0.01 * OHM;

    /// One `Milliohm` (mΩ), as an `ElectricalResistance`.
    pub const MILLIOHM: ElectricalResistance = 0.001 * OHM;

    /// One `Microohm` (µΩ), as an `ElectricalResistance`.
    pub const MICROOHM: ElectricalResistance = 9.9999999999999995e-07 * OHM;

    /// One `Nanoohm` (nΩ), as an `ElectricalResistance`.
    pub const NANOOHM: ElectricalResistance = 1.0000000000000001e-09 * OHM;

    /// One `Picoohm` (pΩ), as an `ElectricalResistance`.
    pub const PICOOHM: ElectricalResistance = 9.9999999999999998e-13 * OHM;

    /// One `Femtoohm` (fΩ), as an `ElectricalResistance`.
    pub const FEMTOOHM: ElectricalResistance = 1.0000000000000001e-15 * OHM;

    /// One `Attoohm` (aΩ), as an `ElectricalResistance`.
    pub const ATTOOHM: ElectricalResistance = 1.0000000000000001e-18 * OHM;

    /// One `Zeptoohm` (zΩ), as an `ElectricalResistance`.
    pub const ZEPTOOHM: ElectricalResistance = 9.9999999999999991e-22 * OHM;

    /// One `Yoctoohm` (yΩ), as an `ElectricalResistance`.
    pub const YOCTOOHM: ElectricalResistance = 9.9999999999999992e-25 * OHM;

    /// One `Abohm` (abΩ), as an `ElectricalResistance`.
    pub const ABOHM: ElectricalResistance = 1.0000000000000001e-09 * OHM;

    /// One `Statohm` (statΩ), as an `ElectricalResistance`.
    pub const STATOHM: ElectricalResistance = 898755291711.5481 * OHM;

    /// One `YottaohmMetre` (YΩ · m), as an `ElectricalResistivity`.
    pub const YOTTAOHM_METRE: ElectricalResistivity = 9.9999999999999998e+23 * OHM_METRE;

    /// One `ZettaohmMetre` (ZΩ · m), as an `ElectricalResistivity`.
    pub const ZETTAOHM_METRE: ElectricalResistivity = 1e+21 * OHM_METRE;

    /// One `ExaohmMetre` (EΩ · m), as an `ElectricalResistivity`.
    pub const EXAOHM_METRE: ElectricalResistivity = 1e+18 * OHM_METRE;

    /// One `PetaohmMetre` (PΩ · m), as an `ElectricalResistivity`.
    pub const PETAOHM_METRE: ElectricalResistivity = 1000000000000000.0 * OHM_METRE;

    /// One `TeraohmMetre` (TΩ · m), as an `ElectricalResistivity`.
    pub const TERAOHM_METRE: ElectricalResistivity = 1000000000000.0 * OHM_METRE;

    /// One `GigaohmMetre` (GΩ · m), as an `ElectricalResistivity`.
    pub const GIGAOHM_METRE: ElectricalResistivity = 1000000000.0 * OHM_METRE;

    /// One `MegaohmMetre` (MΩ · m), as an `ElectricalResistivity`.
    pub const MEGAOHM_METRE: ElectricalResistivity = 1000000.0 * OHM_METRE;

    /// One `KiloohmMetre` (kΩ · m), as an `ElectricalResistivity`.
    pub const KILOOHM_METRE: ElectricalResistivity = 1000.0 * OHM_METRE;

    /// One `HectoohmMetre` (hΩ · m), as an `ElectricalResistivity`.
    pub const HECTOOHM_METRE: ElectricalResistivity = 100.0 * OHM_METRE;

    /// One `DecaohmMetre` (daΩ · m), as an `ElectricalResistivity`.
    pub const DECAOHM_METRE: ElectricalResistivity = 10.0 * OHM_METRE;

    /// One `OhmMetre` (Ω · m), as an `ElectricalResistivity`.
    pub const OHM_METRE: ElectricalResistivity =
        METRE * METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * AMPERE * AMPERE);

    /// One `DeciohmMetre` (dΩ · m), as an `ElectricalResistivity`.
    pub const DECIOHM_METRE: ElectricalResistivity = 0.10000000000000001 * OHM_METRE;

    /// One `CentiohmMetre` (cΩ · m), as an `ElectricalResistivity`.
    pub const CENTIOHM_METRE: ElectricalResistivity = 0.01 * OHM_METRE;

    /// One `MilliohmMetre` (mΩ · m), as an `ElectricalResistivity`.
    pub const MILLIOHM_METRE: ElectricalResistivity = 0.001 * OHM_METRE;

    /// One `MicroohmMetre` (µΩ · m), as an `ElectricalResistivity`.
    pub const MICROOHM_METRE: ElectricalResistivity = 9.9999999999999995e-07 * OHM_METRE;

    /// One `NanoohmMetre` (nΩ · m), as an `ElectricalResistivity`.
    pub const NANOOHM_METRE: ElectricalResistivity = 1.0000000000000001e-09 * OHM_METRE;

    /// One `PicoohmMetre` (pΩ · m), as an `ElectricalResistivity`.
    pub const PICOOHM_METRE: ElectricalResistivity = 9.9999999999999998e-13 * OHM_METRE;

    /// One `FemtoohmMetre` (fΩ · m), as an `ElectricalResistivity`.
    pub const FEMTOOHM_METRE: ElectricalResistivity = 1.0000000000000001e-15 * OHM_METRE;

    /// One `AttoohmMetre` (aΩ · m), as an `ElectricalResistivity`.
    pub const ATTOOHM_METRE: ElectricalResistivity = 1.0000000000000001e-18 * OHM_METRE;

    /// One `ZeptoohmMetre` (zΩ · m), as an `ElectricalResistivity`.
    pub const ZEPTOOHM_METRE: ElectricalResistivity = 9.9999999999999991e-22 * OHM_METRE;

    /// One `YoctoohmMetre` (yΩ · m), as an `ElectricalResistivity`.
    pub const YOCTOOHM_METRE: ElectricalResistivity = 9.9999999999999992e-25 * OHM_METRE;

    /// One `AbohmMetre` (abΩ · m), as an `ElectricalResistivity`.
    pub const ABOHM_METRE: ElectricalResistivity = 1.0000000000000001e-09 * OHM_METRE;

    /// One `StatohmMetre` (statΩ · m), as an `ElectricalResistivity`.
    pub const STATOHM_METRE: ElectricalResistivity = 898755291711.5481 * OHM_METRE;

    /// One `OhmCentimetre` (Ω · cm), as an `ElectricalResistivity`.
    pub const OHM_CENTIMETRE: ElectricalResistivity = 0.01 * OHM_METRE;

    /// One `AbohmCentimetre` (abΩ · cm), as an `ElectricalResistivity`.
    pub const ABOHM_CENTIMETRE: ElectricalResistivity = 1.0000000000000001e-11 * OHM_METRE;

    /// One `StatohmCentimetre` (statΩ · cm), as an `ElectricalResistivity`.
    pub const STATOHM_CENTIMETRE: ElectricalResistivity = 8987552917.1154804 * OHM_METRE;

    /// One `OhmInch` (Ω · in), as an `ElectricalResistivity`.
    pub const OHM_INCH: ElectricalResistivity = 0.025399999999999999 * OHM_METRE;

    /// One `OhmFoot` (Ω · ft), as an `ElectricalResistivity`.
    pub const OHM_FOOT: ElectricalResistivity = 0.30480000000000002 * OHM_METRE;

    /// One `OhmYard` (Ω · yd), as an `ElectricalResistivity`.
    pub const OHM_YARD: ElectricalResistivity = 0.91439999999999999 * OHM_METRE;

    /// One `OhmSquareMillimetrePerMetre` (Ω · mm²/m), as an `ElectricalResistivity`.
    pub const OHM_SQUARE_MILLIMETRE_PER_METRE: ElectricalResistivity =
        9.9999999999999995e-07 * OHM_METRE;

    /// One `Yottafarad` (YF), as a `Capacitance`.
    pub const YOTTAFARAD: Capacitance = 9.9999999999999998e+23 * FARAD;

    /// One `Zettafarad` (ZF), as a `Capacitance`.
    pub const ZETTAFARAD: Capacitance = 1e+21 * FARAD;

    /// One `Exafarad` (EF), as a `Capacitance`.
    pub const EXAFARAD: Capacitance = 1e+18 * FARAD;

    /// One `Petafarad` (PF), as a `Capacitance`.
    pub const PETAFARAD: Capacitance = 1000000000000000.0 * FARAD;

    /// One `Terafarad` (TF), as a `Capacitance`.
    pub const TERAFARAD: Capacitance = 1000000000000.0 * FARAD;

    /// One `Gigafarad` (GF), as a `Capacitance`.
    pub const GIGAFARAD: Capacitance = 1000000000.0 * FARAD;

    /// One `Megafarad` (MF), as a `Capacitance`.
    pub const MEGAFARAD: Capacitance = 1000000.0 * FARAD;

    /// One `Kilofarad` (kF), as a `Capacitance`.
    pub const KILOFARAD: Capacitance = 1000.0 * FARAD;

    /// One `Hectofarad` (hF), as a `Capacitance`.
    pub const HECTOFARAD: Capacitance = 100.0 * FARAD;

    /// One `Decafarad` (daF), as a `Capacitance`.
    pub const DECAFARAD: Capacitance = 10.0 * FARAD;

    /// One `Farad` (F), as a `Capacitance`.
    pub const FARAD: Capacitance =
        SECOND * SECOND * SECOND * SECOND * AMPERE * AMPERE / (METRE * METRE * KILOGRAM);

    /// One `Decifarad` (dF), as a `Capacitance`.
    pub const DECIFARAD: Capacitance = 0.10000000000000001 * FARAD;

    /// One `Centifarad` (cF), as a `Capacitance`.
    pub const CENTIFARAD: Capacitance = 0.01 * FARAD;

    /// One `Millifarad` (mF), as a `Capacitance`.
    pub const MILLIFARAD: Capacitance = 0.001 * FARAD;

    /// One `Microfarad` (µF), as a `Capacitance`.
    pub const MICROFARAD: Capacitance = 9.9999999999999995e-07 * FARAD;

    /// One `Nanofarad` (nF), as a `Capacitance`.
    pub const NANOFARAD: Capacitance = 1.0000000000000001e-09 * FARAD;

    /// One `Picofarad` (pF), as a `Capacitance`.
    pub const PICOFARAD: Capacitance = 9.9999999999999998e-13 * FARAD;

    /// One `Femtofarad` (fF), as a `Capacitance`.
    pub const FEMTOFARAD: Capacitance = 1.0000000000000001e-15 * FARAD;

    /// One `Attofarad` (aF), as a `Capacitance`.
    pub const ATTOFARAD: Capacitance = 1.0000000000000001e-18 * FARAD;

    /// One `Zeptofarad` (zF), as a `Capacitance`.
    pub const ZEPTOFARAD: Capacitance = 9.9999999999999991e-22 * FARAD;

    /// One `Yoctofarad` (yF), as a `Capacitance`.
    pub const YOCTOFARAD: Capacitance = 9.9999999999999992e-25 * FARAD;

    /// One `Abfarad` (abF), as a `Capacitance`.
    pub const ABFARAD: Capacitance = 1000000000.0 * FARAD;

    /// One `Statfarad` (statF), as a `Capacitance`.
    pub const STATFARAD: Capacitance = 1.1126499999999999e-12 * FARAD;

    /// One `Yottahenry` (YH), as an `Inductance`.
    pub const YOTTAHENRY: Inductance = 9.9999999999999998e+23 * HENRY;

    /// One `Zettahenry` (ZH), as an `Inductance`.
    pub const ZETTAHENRY: Inductance = 1e+21 * HENRY;

    /// One `Exahenry` (EH), as an `Inductance`.
    pub const EXAHENRY: Inductance = 1e+18 * HENRY;

    /// One `Petahenry` (PH), as an `Inductance`.
    pub const PETAHENRY: Inductance = 1000000000000000.0 * HENRY;

    /// One `Terahenry` (TH), as an `Inductance`.
    pub const TERAHENRY: Inductance = 1000000000000.0 * HENRY;

    /// One `Gigahenry` (GH), as an `Inductance`.
    pub const GIGAHENRY: Inductance = 1000000000.0 * HENRY;

    /// One `Megahenry` (MH), as an `Inductance`.
    pub const MEGAHENRY: Inductance = 1000000.0 * HENRY;

    /// One `Kilohenry` (kH), as an `Inductance`.
    pub const KILOHENRY: Inductance = 1000.0 * HENRY;

    /// One `Hectohenry` (hH), as an `Inductance`.
    pub const HECTOHENRY: Inductance = 100.0 * HENRY;

    /// One `Decahenry` (daH), as an `Inductance`.
    pub const DECAHENRY: Inductance = 10.0 * HENRY;

    /// One `Henry` (H), as an `Inductance`.
    pub const HENRY: Inductance = METRE * METRE * KILOGRAM / (SECOND * SECOND * AMPERE * AMPERE);

    /// One `Decihenry` (dH), as an `Inductance`.
    pub const DECIHENRY: Inductance = 0.10000000000000001 * HENRY;

    /// One `Centihenry` (cH), as an `Inductance`.
    pub const CENTIHENRY: Inductance = 0.01 * HENRY;

    /// One `Millihenry` (mH), as an `Inductance`.
    pub const MILLIHENRY: Inductance = 0.001 * HENRY;

    /// One `Microhenry` (µH), as an `Inductance`.
    pub const MICROHENRY: Inductance = 9.9999999999999995e-07 * HENRY;

    /// One `Nanohenry` (nH), as an `Inductance`.
    pub const NANOHENRY: Inductance = 1.0000000000000001e-09 * HENRY;

    /// One `Picohenry` (pH), as an `Inductance`.
    pub const PICOHENRY: Inductance = 9.9999999999999998e-13 * HENRY;

    /// One `Femtohenry` (fH), as an `Inductance`.
    pub const FEMTOHENRY: Inductance = 1.0000000000000001e-15 * HENRY;

    /// One `Attohenry` (aH), as an `Inductance`.
    pub const ATTOHENRY: Inductance = 1.0000000000000001e-18 * HENRY;

    /// One `Zeptohenry` (zH), as an `Inductance`.
    pub const ZEPTOHENRY: Inductance = 9.9999999999999991e-22 * HENRY;

    /// One `Yoctohenry` (yH), as an `Inductance`.
    pub const YOCTOHENRY: Inductance = 9.9999999999999992e-25 * HENRY;

    /// One `Abhenry` (abH), as an `Inductance`.
    pub const ABHENRY: Inductance = 1.0000000000000001e-09 * HENRY;

    /// One `Stathenry` (statH), as an `Inductance`.
    pub const STATHENRY: Inductance = 898755291711.5481 * HENRY;

    /// One `AmperePerMetre` (A/m), as a `MagneticFieldStrength`.
    pub const MAGNETIC_FIELD_STRENGTH_AMPERE_PER_METRE: MagneticFieldStrength = AMPERE / METRE;

    /// One `AmperePerCentimetre` (A/cm), as a `MagneticFieldStrength`.
    pub const MAGNETIC_FIELD_STRENGTH_AMPERE_PER_CENTIMETRE: MagneticFieldStrength =
        100.0 * MAGNETIC_FIELD_STRENGTH_AMPERE_PER_METRE;

    /// One `AmperePerMillimetre` (A/mm), as a `MagneticFieldStrength`.
    pub const MAGNETIC_FIELD_STRENGTH_AMPERE_PER_MILLIMETRE: MagneticFieldStrength =
        1000.0 * MAGNETIC_FIELD_STRENGTH_AMPERE_PER_METRE;

    /// One `AmperePerMicrometre` (A/μm), as a `MagneticFieldStrength`.
    pub const MAGNETIC_FIELD_STRENGTH_AMPERE_PER_MICROMETRE: MagneticFieldStrength =
        1000000.0 * MAGNETIC_FIELD_STRENGTH_AMPERE_PER_METRE;

    /// One `Oersted` (Oe), as a `MagneticFieldStrength`.
    pub const OERSTED: MagneticFieldStrength =
        79.577471545947674 * MAGNETIC_FIELD_STRENGTH_AMPERE_PER_METRE;

    /// One `Yottaweber` (YWb), as a `MagneticFlux`.
    pub const YOTTAWEBER: MagneticFlux = 9.9999999999999998e+23 * WEBER;

    /// One `Zettaweber` (ZWb), as a `MagneticFlux`.
    pub const ZETTAWEBER: MagneticFlux = 1e+21 * WEBER;

    /// One `Exaweber` (EWb), as a `MagneticFlux`.
    pub const EXAWEBER: MagneticFlux = 1e+18 * WEBER;

    /// One `Petaweber` (PWb), as a `MagneticFlux`.
    pub const PETAWEBER: MagneticFlux = 1000000000000000.0 * WEBER;

    /// One `Teraweber` (TWb), as a `MagneticFlux`.
    pub const TERAWEBER: MagneticFlux = 1000000000000.0 * WEBER;

    /// One `Gigaweber` (GWb), as a `MagneticFlux`.
    pub const GIGAWEBER: MagneticFlux = 1000000000.0 * WEBER;

    /// One `Megaweber` (MWb), as a `MagneticFlux`.
    pub const MEGAWEBER: MagneticFlux = 1000000.0 * WEBER;

    /// One `Kiloweber` (kWb), as a `MagneticFlux`.
    pub const KILOWEBER: MagneticFlux = 1000.0 * WEBER;

    /// One `Hectoweber` (hWb), as a `MagneticFlux`.
    pub const HECTOWEBER: MagneticFlux = 100.0 * WEBER;

    /// One `Decaweber` (daWb), as a `MagneticFlux`.
    pub const DECAWEBER: MagneticFlux = 10.0 * WEBER;

    /// One `Weber` (Wb), as a `MagneticFlux`.
    pub const WEBER: MagneticFlux = METRE * METRE * KILOGRAM / (SECOND * SECOND * AMPERE);

    /// One `Deciweber` (dWb), as a `MagneticFlux`.
    pub const DECIWEBER: MagneticFlux = 0.10000000000000001 * WEBER;

    /// One `Centiweber` (cWb), as a `MagneticFlux`.
    pub const CENTIWEBER: MagneticFlux = 0.01 * WEBER;

    /// One `Milliweber` (mWb), as a `MagneticFlux`.
    pub const MILLIWEBER: MagneticFlux = 0.001 * WEBER;

    /// One `Microweber` (µWb), as a `MagneticFlux`.
    pub const MICROWEBER: MagneticFlux = 9.9999999999999995e-07 * WEBER;

    /// One `Nanoweber` (nWb), as a `MagneticFlux`.
    pub const NANOWEBER: MagneticFlux = 1.0000000000000001e-09 * WEBER;

    /// One `Picoweber` (pWb), as a `MagneticFlux`.
    pub const PICOWEBER: MagneticFlux = 9.9999999999999998e-13 * WEBER;

    /// One `Femtoweber` (fWb), as a `MagneticFlux`.
    pub const FEMTOWEBER: MagneticFlux = 1.0000000000000001e-15 * WEBER;

    /// One `Attoweber` (aWb), as a `MagneticFlux`.
    pub const ATTOWEBER: MagneticFlux = 1.0000000000000001e-18 * WEBER;

    /// One `Zeptoweber` (zWb), as a `MagneticFlux`.
    pub const ZEPTOWEBER: MagneticFlux = 9.9999999999999991e-22 * WEBER;

    /// One `Yoctoweber` (yWb), as a `MagneticFlux`.
    pub const YOCTOWEBER: MagneticFlux = 9.9999999999999992e-25 * WEBER;

    /// One `Maxwell` (Mx), as a `MagneticFlux`.
    pub const MAXWELL: MagneticFlux = 1e-08 * WEBER;

    /// One `Yottatesla` (YT), as a `MagneticFluxDensity`.
    pub const YOTTATESLA: MagneticFluxDensity = 9.9999999999999998e+23 * TESLA;

    /// One `Zettatesla` (ZT), as a `MagneticFluxDensity`.
    pub const ZETTATESLA: MagneticFluxDensity = 1e+21 * TESLA;

    /// One `Exatesla` (ET), as a `MagneticFluxDensity`.
    pub const EXATESLA: MagneticFluxDensity = 1e+18 * TESLA;

    /// One `Petatesla` (PT), as a `MagneticFluxDensity`.
    pub const PETATESLA: MagneticFluxDensity = 1000000000000000.0 * TESLA;

    /// One `Teratesla` (TT), as a `MagneticFluxDensity`.
    pub const TERATESLA: MagneticFluxDensity = 1000000000000.0 * TESLA;

    /// One `Gigatesla` (GT), as a `MagneticFluxDensity`.
    pub const GIGATESLA: MagneticFluxDensity = 1000000000.0 * TESLA;

    /// One `Megatesla` (MT), as a `MagneticFluxDensity`.
    pub const MEGATESLA: MagneticFluxDensity = 1000000.0 * TESLA;

    /// One `Kilotesla` (kT), as a `MagneticFluxDensity`.
    pub const KILOTESLA: MagneticFluxDensity = 1000.0 * TESLA;

    /// One `Hectotesla` (hT), as a `MagneticFluxDensity`.
    pub const HECTOTESLA: MagneticFluxDensity = 100.0 * TESLA;

    /// One `Decatesla` (daT), as a `MagneticFluxDensity`.
    pub const DECATESLA: MagneticFluxDensity = 10.0 * TESLA;

    /// One `Tesla` (T), as a `MagneticFluxDensity`.
    pub const TESLA: MagneticFluxDensity = KILOGRAM / (SECOND * SECOND * AMPERE);

    /// One `Decitesla` (dT), as a `MagneticFluxDensity`.
    pub const DECITESLA: MagneticFluxDensity = 0.10000000000000001 * TESLA;

    /// One `Centitesla` (cT), as a `MagneticFluxDensity`.
    pub const CENTITESLA: MagneticFluxDensity = 0.01 * TESLA;

    /// One `Millitesla` (mT), as a `MagneticFluxDensity`.
    pub const MILLITESLA: MagneticFluxDensity = 0.001 * TESLA;

    /// One `Microtesla` (µT), as a `MagneticFluxDensity`.
    pub const MICROTESLA: MagneticFluxDensity = 9.9999999999999995e-07 * TESLA;

    /// One `Nanotesla` (nT), as a `MagneticFluxDensity`.
    pub const NANOTESLA: MagneticFluxDensity = 1.0000000000000001e-09 * TESLA;

    /// One `Picotesla` (pT), as a `MagneticFluxDensity`.
    pub const PICOTESLA: MagneticFluxDensity = 9.9999999999999998e-13 * TESLA;

    /// One `Femtotesla` (fT), as a `MagneticFluxDensity`.
    pub const FEMTOTESLA: MagneticFluxDensity = 1.0000000000000001e-15 * TESLA;

    /// One `Attotesla` (aT), as a `MagneticFluxDensity`.
    pub const ATTOTESLA: MagneticFluxDensity = 1.0000000000000001e-18 * TESLA;

    /// One `Zeptotesla` (zT), as a `MagneticFluxDensity`.
    pub const ZEPTOTESLA: MagneticFluxDensity = 9.9999999999999991e-22 * TESLA;

    /// One `Yoctotesla` (yT), as a `MagneticFluxDensity`.
    pub const YOCTOTESLA: MagneticFluxDensity = 9.9999999999999992e-25 * TESLA;

    /// One `Gamma` (γ), as a `MagneticFluxDensity`.
    pub const GAMMA: MagneticFluxDensity = 1.0000000000000001e-09 * TESLA;

    /// One `Gauss` (G), as a `MagneticFluxDensity`.
    pub const GAUSS: MagneticFluxDensity = 0.0001 * TESLA;

    /// One `AmpereSquareMetre` (A · m²), as a `MagneticMoment`.
    pub const AMPERE_SQUARE_METRE: MagneticMoment = METRE * METRE * AMPERE;

    /// One `JoulePerTesla` (J/T), as a `MagneticMoment`.
    pub const JOULE_PER_TESLA: MagneticMoment = AMPERE_SQUARE_METRE;

    /// One `NewtonMetrePerTesla` (N · m/T), as a `MagneticMoment`.
    pub const NEWTON_METRE_PER_TESLA: MagneticMoment = AMPERE_SQUARE_METRE;

    /// One `ErgPerGauss` (erg/G), as a `MagneticMoment`.
    pub const ERG_PER_GAUSS: MagneticMoment = 0.0009999999999999998 * AMPERE_SQUARE_METRE;

    /// One `BohrMagneton` (µ(Bohr)), as a `MagneticMoment`.
    pub const BOHR_MAGNETON: MagneticMoment = 9.2740100783000004e-24 * AMPERE_SQUARE_METRE;

    /// One `NuclearMagneton` (μ(Nuclear)), as a `MagneticMoment`.
    pub const NUCLEAR_MAGNETON: MagneticMoment = 5.0507837461000003e-27 * AMPERE_SQUARE_METRE;

    /// One `AtomicUnitOfMagneticDipoleMoment` (ħ · e/mₑ), as a `MagneticMoment`.
    pub const ATOMIC_UNIT_OF_MAGNETIC_DIPOLE_MOMENT: MagneticMoment =
        1.8548020156600001e-23 * AMPERE_SQUARE_METRE;

    /// One `DeuteronMagneticMoment` (μ(deuteron)), as a `MagneticMoment`.
    pub const DEUTERON_MAGNETIC_MOMENT: MagneticMoment =
        4.3307350939999997e-27 * AMPERE_SQUARE_METRE;

    /// One `ProtonMagneticMoment` (μ(proton)), as a `MagneticMoment`.
    pub const PROTON_MAGNETIC_MOMENT: MagneticMoment = 1.4106067973599999e-26 * AMPERE_SQUARE_METRE;

    /// One `ShieldedProtonMagneticMoment` (μ’(proton)), as a `MagneticMoment`.
    pub const SHIELDED_PROTON_MAGNETIC_MOMENT: MagneticMoment =
        1.41057056e-26 * AMPERE_SQUARE_METRE;

    /// One `TritonMagneticMoment` (μ(triton)), as a `MagneticMoment`.
    pub const TRITON_MAGNETIC_MOMENT: MagneticMoment = 1.5046095202000001e-26 * AMPERE_SQUARE_METRE;

    /// One `HenryPerMetre` (H/m), as a `MagneticPermeability`.
    pub const HENRY_PER_METRE: MagneticPermeability =
        METRE * KILOGRAM / (SECOND * SECOND * AMPERE * AMPERE);

    /// One `VacuumPermeability` (µ₀), as a `MagneticPermeability`.
    pub const VACUUM_PERMEABILITY: MagneticPermeability = 1.25663706212e-6 * HENRY_PER_METRE;

    /// One `YottahertzPerTesla` (YHz/T), as a `GyromagneticRatio`.
    pub const YOTTAHERTZ_PER_TESLA: GyromagneticRatio = 9.9999999999999998e+23 * HERTZ_PER_TESLA;

    /// One `ZettahertzPerTesla` (ZHz/T), as a `GyromagneticRatio`.
    pub const ZETTAHERTZ_PER_TESLA: GyromagneticRatio = 1e+21 * HERTZ_PER_TESLA;

    /// One `ExahertzPerTesla` (EHz/T), as a `GyromagneticRatio`.
    pub const EXAHERTZ_PER_TESLA: GyromagneticRatio = 1e+18 * HERTZ_PER_TESLA;

    /// One `PetahertzPerTesla` (PHz/T), as a `GyromagneticRatio`.
    pub const PETAHERTZ_PER_TESLA: GyromagneticRatio = 1000000000000000.0 * HERTZ_PER_TESLA;

    /// One `TerahertzPerTesla` (THz/T), as a `GyromagneticRatio`.
    pub const TERAHERTZ_PER_TESLA: GyromagneticRatio = 1000000000000.0 * HERTZ_PER_TESLA;

    /// One `GigahertzPerTesla` (GHz/T), as a `GyromagneticRatio`.
    pub const GIGAHERTZ_PER_TESLA: GyromagneticRatio = 1000000000.0 * HERTZ_PER_TESLA;

    /// One `MegahertzPerTesla` (MHz/T), as a `GyromagneticRatio`.
    pub const MEGAHERTZ_PER_TESLA: GyromagneticRatio = 1000000.0 * HERTZ_PER_TESLA;

    /// One `KilohertzPerTesla` (kHz/T), as a `GyromagneticRatio`.
    pub const KILOHERTZ_PER_TESLA: GyromagneticRatio = 1000.0 * HERTZ_PER_TESLA;

    /// One `HectohertzPerTesla` (hHz/T), as a `GyromagneticRatio`.
    pub const HECTOHERTZ_PER_TESLA: GyromagneticRatio = 100.0 * HERTZ_PER_TESLA;

    /// One `DecahertzPerTesla` (daHz/T), as a `GyromagneticRatio`.
    pub const DECAHERTZ_PER_TESLA: GyromagneticRatio = 10.0 * HERTZ_PER_TESLA;

    /// One `HertzPerTesla` (Hz/T), as a `GyromagneticRatio`.
    pub const HERTZ_PER_TESLA: GyromagneticRatio = SECOND * AMPERE * RADIAN / KILOGRAM;

    /// One `DecihertzPerTesla` (dHz/T), as a `GyromagneticRatio`.
    pub const DECIHERTZ_PER_TESLA: GyromagneticRatio = 0.10000000000000001 * HERTZ_PER_TESLA;

    /// One `CentihertzPerTesla` (cHz/T), as a `GyromagneticRatio`.
    pub const CENTIHERTZ_PER_TESLA: GyromagneticRatio = 0.01 * HERTZ_PER_TESLA;

    /// One `MillihertzPerTesla` (mHz/T), as a `GyromagneticRatio`.
    pub const MILLIHERTZ_PER_TESLA: GyromagneticRatio = 0.001 * HERTZ_PER_TESLA;

    /// One `MicrohertzPerTesla` (µHz/T), as a `GyromagneticRatio`.
    pub const MICROHERTZ_PER_TESLA: GyromagneticRatio = 9.9999999999999995e-07 * HERTZ_PER_TESLA;

    /// One `NanohertzPerTesla` (nHz/T), as a `GyromagneticRatio`.
    pub const NANOHERTZ_PER_TESLA: GyromagneticRatio = 1.0000000000000001e-09 * HERTZ_PER_TESLA;

    /// One `PicohertzPerTesla` (pHz/T), as a `GyromagneticRatio`.
    pub const PICOHERTZ_PER_TESLA: GyromagneticRatio = 9.9999999999999998e-13 * HERTZ_PER_TESLA;

    /// One `FemtohertzPerTesla` (fHz/T), as a `GyromagneticRatio`.
    pub const FEMTOHERTZ_PER_TESLA: GyromagneticRatio = 1.0000000000000001e-15 * HERTZ_PER_TESLA;

    /// One `AttohertzPerTesla` (aHz/T), as a `GyromagneticRatio`.
    pub const ATTOHERTZ_PER_TESLA: GyromagneticRatio = 1.0000000000000001e-18 * HERTZ_PER_TESLA;

    /// One `ZeptohertzPerTesla` (zHz/T), as a `GyromagneticRatio`.
    pub const ZEPTOHERTZ_PER_TESLA: GyromagneticRatio = 9.9999999999999991e-22 * HERTZ_PER_TESLA;

    /// One `YoctohertzPerTesla` (yHz/T), as a `GyromagneticRatio`.
    pub const YOCTOHERTZ_PER_TESLA: GyromagneticRatio = 9.9999999999999992e-25 * HERTZ_PER_TESLA;

    /// One `RadianPerSecondTesla` (rad/(s · T)), as a `GyromagneticRatio`.
    pub const RADIAN_PER_SECOND_TESLA: GyromagneticRatio = HERTZ_PER_TESLA;

    /// One `ProtonGyromagneticRatio` (γₚ), as a `GyromagneticRatio`.
    pub const PROTON_GYROMAGNETIC_RATIO: GyromagneticRatio = 267522187.08000001 * HERTZ_PER_TESLA;

    /// One `NeutronGyromagneticRatio` (γₙ), as a `GyromagneticRatio`.
    pub const NEUTRON_GYROMAGNETIC_RATIO: GyromagneticRatio = 183247174.0 * HERTZ_PER_TESLA;

    /// One `ElectronGyromagneticRatio` (γₑ), as a `GyromagneticRatio`.
    pub const ELECTRON_GYROMAGNETIC_RATIO: GyromagneticRatio = 176085962784.0 * HERTZ_PER_TESLA;

    /// One `ShieldedProtonGyromagneticRatio` (γ'ₚ), as a `GyromagneticRatio`.
    pub const SHIELDED_PROTON_GYROMAGNETIC_RATIO: GyromagneticRatio =
        267515319.40000001 * HERTZ_PER_TESLA;

    /// One `ShieldedHelionGyromagneticRatio` (γ'ₕ), as a `GyromagneticRatio`.
    pub const SHIELDED_HELION_GYROMAGNETIC_RATIO: GyromagneticRatio =
        203789460.78 * HERTZ_PER_TESLA;

    /// One `DeuteronGyromagneticRatio` (γ₂), as a `GyromagneticRatio`.
    pub const DEUTERON_GYROMAGNETIC_RATIO: GyromagneticRatio = 41066288.799999997 * HERTZ_PER_TESLA;

    /// One `TritonGyromagneticRatio` (γ₃), as a `GyromagneticRatio`.
    pub const TRITON_GYROMAGNETIC_RATIO: GyromagneticRatio = 285349844.0 * HERTZ_PER_TESLA;

    /// One `Lithium7GyromagneticRatio` (γ₇), as a `GyromagneticRatio`.
    pub const LITHIUM7_GYROMAGNETIC_RATIO: GyromagneticRatio = 103975264.0 * HERTZ_PER_TESLA;

    /// One `Boron11GyromagneticRatio` (γ₁₁), as a `GyromagneticRatio`.
    pub const BORON11_GYROMAGNETIC_RATIO: GyromagneticRatio = 85838414.299999997 * HERTZ_PER_TESLA;

    /// One `Carbon13GyromagneticRatio` (γ₁₃), as a `GyromagneticRatio`.
    pub const CARBON13_GYROMAGNETIC_RATIO: GyromagneticRatio = 67278754.599999994 * HERTZ_PER_TESLA;

    /// One `Nitrogen14GyromagneticRatio` (γ₁₄), as a `GyromagneticRatio`.
    pub const NITROGEN14_GYROMAGNETIC_RATIO: GyromagneticRatio =
        19328792.199999999 * HERTZ_PER_TESLA;

    /// One `Flourine19GyromagneticRatio` (γ₁₉), as a `GyromagneticRatio`.
    pub const FLOURINE19_GYROMAGNETIC_RATIO: GyromagneticRatio = 251762483.0 * HERTZ_PER_TESLA;

    /// One `Sodium23GyromagneticRatio` (γ₂₃), as a `GyromagneticRatio`.
    pub const SODIUM23_GYROMAGNETIC_RATIO: GyromagneticRatio = 70803541.700000003 * HERTZ_PER_TESLA;

    /// One `Phosphorus31GyromagneticRatio` (γ₃₁), as a `GyromagneticRatio`.
    pub const PHOSPHORUS31_GYROMAGNETIC_RATIO: GyromagneticRatio = 108329419.0 * HERTZ_PER_TESLA;

    /// One `ProtonReducedGyromagneticRatio` (̅γₚ), as a `ReducedGyromagneticRatio`.
    pub const PROTON_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio =
        42577478.461000003 * (SECOND * AMPERE / KILOGRAM);

    /// One `NeutronReducedGyromagneticRatio` (̅γₙ), as a `ReducedGyromagneticRatio`.
    pub const NEUTRON_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio =
        29164693.5 * (SECOND * AMPERE / KILOGRAM);

    /// One `ElectronReducedGyromagneticRatio` (̅γₑ), as a `ReducedGyromagneticRatio`.
    pub const ELECTRON_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio =
        28024951386.099998 * (SECOND * AMPERE / KILOGRAM);

    /// One `ShieldedProtonReducedGyromagneticRatio` (γ'ₚ), as a `ReducedGyromagneticRatio`.
    pub const SHIELDED_PROTON_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio =
        42576385.43 * (SECOND * AMPERE / KILOGRAM);

    /// One `ShieldedHelionReducedGyromagneticRatio` (γ'ₕ), as a `ReducedGyromagneticRatio`.
    pub const SHIELDED_HELION_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio =
        32434100.033 * (SECOND * AMPERE / KILOGRAM);

    /// One `DeuteronReducedGyromagneticRatio` (̅γ₂), as a `ReducedGyromagneticRatio`.
    pub const DEUTERON_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio =
        6535902.8499999996 * (SECOND * AMPERE / KILOGRAM);

    /// One `TritonReducedGyromagneticRatio` (̅γ₃), as a `ReducedGyromagneticRatio`.
    pub const TRITON_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio =
        45414838.100000001 * (SECOND * AMPERE / KILOGRAM);

    /// One `Lithium7ReducedGyromagneticRatio` (̅γ₇), as a `ReducedGyromagneticRatio`.
    pub const LITHIUM7_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio =
        16548177.300000001 * (SECOND * AMPERE / KILOGRAM);

    /// One `Boron11ReducedGyromagneticRatio` (̅γ₁₁), as a `ReducedGyromagneticRatio`.
    pub const BORON11_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio =
        13661608.0 * (SECOND * AMPERE / KILOGRAM);

    /// One `Carbon13ReducedGyromagneticRatio` (̅γ₁₃), as a `ReducedGyromagneticRatio`.
    pub const CARBON13_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio =
        10707746.4 * (SECOND * AMPERE / KILOGRAM);

    /// One `Nitrogen14ReducedGyromagneticRatio` (̅γ₁₄), as a `ReducedGyromagneticRatio`.
    pub const NITROGEN14_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio =
        3076272.8100000001 * (SECOND * AMPERE / KILOGRAM);

    /// One `Flourine19ReducedGyromagneticRatio` (̅γ₁₉), as a `ReducedGyromagneticRatio`.
    pub const FLOURINE19_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio =
        40069243.700000003 * (SECOND * AMPERE / KILOGRAM);

    /// One `Sodium23ReducedGyromagneticRatio` (̅γ₂₃), as a `ReducedGyromagneticRatio`.
    pub const SODIUM23_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio =
        11268733.6 * (SECOND * AMPERE / KILOGRAM);

    /// One `Phosphorus31ReducedGyromagneticRatio` (̅γ₃₁), as a `ReducedGyromagneticRatio`.
    pub const PHOSPHORUS31_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio =
        17241162.5 * (SECOND * AMPERE / KILOGRAM);
}

/// The units of `ElectricCharge`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCharge)]
pub enum ElectricChargeUnit {
    #[unit("YC", "yottacoulombs", by = YOTTACOULOMB)]
    Yottacoulomb,
    #[unit("ZC", "zettacoulombs", by = ZETTACOULOMB)]
    Zettacoulomb,
    #[unit("EC", "exacoulombs", by = EXACOULOMB)]
    Exacoulomb,
    #[unit("PC", "petacoulombs", by = PETACOULOMB)]
    Petacoulomb,
    #[unit("TC", "teracoulombs", by = TERACOULOMB)]
    Teracoulomb,
    #[unit("GC", "gigacoulombs", by = GIGACOULOMB)]
    Gigacoulomb,
    #[unit("MC", "megacoulombs", by = MEGACOULOMB)]
    Megacoulomb,
    #[unit("kC", "kilocoulombs", by = KILOCOULOMB)]
    Kilocoulomb,
    #[unit("hC", "hectocoulombs", by = HECTOCOULOMB)]
    Hectocoulomb,
    #[unit("daC", "decacoulombs", by = DECACOULOMB)]
    Decacoulomb,
    #[default]
    #[unit("C", "coulombs", by = COULOMB)]
    Coulomb,
    #[unit("dC", "decicoulombs", by = DECICOULOMB)]
    Decicoulomb,
    #[unit("cC", "centicoulombs", by = CENTICOULOMB)]
    Centicoulomb,
    #[unit("mC", "millcoulombs", by = MILLICOULOMB)]
    Millicoulomb,
    #[unit("µC", "microcoulombs", by = MICROCOULOMB)]
    Microcoulomb,
    #[unit("nC", "nanocoulombs", by = NANOCOULOMB)]
    Nanocoulomb,
    #[unit("pC", "picocoulombs", by = PICOCOULOMB)]
    Picocoulomb,
    #[unit("fC", "femtocoulombs", by = FEMTOCOULOMB)]
    Femtocoulomb,
    #[unit("aC", "attocoulombs", by = ATTOCOULOMB)]
    Attocoulomb,
    #[unit("zC", "zeptocoulombs", by = ZEPTOCOULOMB)]
    Zeptocoulomb,
    #[unit("yC", "yoctocoulombs", by = YOCTOCOULOMB)]
    Yoctocoulomb,
    #[unit("PA · h", "petaampere hours", by = PETAAMPERE_HOUR)]
    PetaampereHour,
    #[unit("TA · h", "teraampere hours", by = TERAAMPERE_HOUR)]
    TeraampereHour,
    #[unit("GA · h", "gigaampere hours", by = GIGAAMPERE_HOUR)]
    GigaampereHour,
    #[unit("MA · h", "megaampere hours", by = MEGAAMPERE_HOUR)]
    MegaampereHour,
    #[unit("kA · h", "kiloampere hours", by = KILOAMPERE_HOUR)]
    KiloampereHour,
    #[unit("hA · h", "hectoampere hours", by = HECTOAMPERE_HOUR)]
    HectoampereHour,
    #[unit("daA · h", "decaampere hours", by = DECAAMPERE_HOUR)]
    DecaampereHour,
    #[unit("A · h", "ampere hours", by = AMPERE_HOUR)]
    AmpereHour,
    #[unit("mA · h", "milliampere hours", by = MILLIAMPERE_HOUR)]
    MilliampereHour,
    #[unit("µA · h", "microampere hours", by = MICROAMPERE_HOUR)]
    MicroampereHour,
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
#[unit(of = ElectricChargeArealDensity)]
pub enum ElectricChargeArealDensityUnit {
    #[default]
    #[unit("C/m²", "coulombs per square metre", by = ELECTRIC_CHARGE_AREAL_DENSITY_COULOMB_PER_SQUARE_METRE)]
    CoulombPerSquareMetre,
}

/// The units of `ElectricChargeLinearDensity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricChargeLinearDensity)]
pub enum ElectricChargeLinearDensityUnit {
    #[default]
    #[unit("C/m", "coulombs per metre", by = COULOMB_PER_METRE)]
    CoulombPerMetre,
    #[unit("C/cm", "coulombs per centimetre", by = COULOMB_PER_CENTIMETRE)]
    CoulombPerCentimetre,
}

/// The units of `ElectricChargeVolumetricDensity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricChargeVolumetricDensity)]
pub enum ElectricChargeVolumetricDensityUnit {
    #[default]
    #[unit("C/m³", "coulombs per cubic metre", by = COULOMB_PER_CUBIC_METRE)]
    CoulombPerCubicMetre,
}

/// The units of `ElectricCurrent`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCurrent)]
pub enum ElectricCurrentUnit {
    #[unit("YA", "yottaamperes", by = YOTTAAMPERE)]
    Yottaampere,
    #[unit("ZA", "zettaamperes", by = ZETTAAMPERE)]
    Zettaampere,
    #[unit("EA", "exaamperes", by = EXAAMPERE)]
    Exaampere,
    #[unit("PA", "petaamperes", by = PETAAMPERE)]
    Petaampere,
    #[unit("TA", "teraamperes", by = TERAAMPERE)]
    Teraampere,
    #[unit("GA", "gigaamperes", by = GIGAAMPERE)]
    Gigaampere,
    #[unit("MA", "megaamperes", by = MEGAAMPERE)]
    Megaampere,
    #[unit("kA", "kiloamperes", by = KILOAMPERE)]
    Kiloampere,
    #[unit("hA", "hectoamperes", by = HECTOAMPERE)]
    Hectoampere,
    #[unit("daA", "decaamperes", by = DECAAMPERE)]
    Decaampere,
    #[default]
    #[unit("A", "amperes", by = AMPERE)]
    Ampere,
    #[unit("dA", "deciamperes", by = DECIAMPERE)]
    Deciampere,
    #[unit("cA", "centiamperes", by = CENTIAMPERE)]
    Centiampere,
    #[unit("mA", "millamperes", by = MILLIAMPERE)]
    Milliampere,
    #[unit("µA", "microamperes", by = MICROAMPERE)]
    Microampere,
    #[unit("nA", "nanoamperes", by = NANOAMPERE)]
    Nanoampere,
    #[unit("pA", "picoamperes", by = PICOAMPERE)]
    Picoampere,
    #[unit("fA", "femtoamperes", by = FEMTOAMPERE)]
    Femtoampere,
    #[unit("aA", "attoamperes", by = ATTOAMPERE)]
    Attoampere,
    #[unit("zA", "zeptoamperes", by = ZEPTOAMPERE)]
    Zeptoampere,
    #[unit("yA", "yoctoamperes", by = YOCTOAMPERE)]
    Yoctoampere,
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
#[unit(of = ElectricCurrentDensity)]
pub enum ElectricCurrentDensityUnit {
    #[default]
    #[unit("A/m²", "amperes per square metre", by = AMPERE_PER_SQUARE_METRE)]
    AmperePerSquareMetre,
}

/// The units of `SurfaceElectricCurrentDensity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SurfaceElectricCurrentDensity)]
pub enum SurfaceElectricCurrentDensityUnit {
    #[default]
    #[unit("A/m", "amperes per metre", by = SURFACE_ELECTRIC_CURRENT_DENSITY_AMPERE_PER_METRE)]
    AmperePerMetre,
    #[unit("A/cm", "amperes per centimetre", by = SURFACE_ELECTRIC_CURRENT_DENSITY_AMPERE_PER_CENTIMETRE)]
    AmperePerCentimetre,
    #[unit("A/mm", "amperes per millimetre", by = SURFACE_ELECTRIC_CURRENT_DENSITY_AMPERE_PER_MILLIMETRE)]
    AmperePerMillimetre,
    #[unit("A/μm", "amperes per micrometre", by = SURFACE_ELECTRIC_CURRENT_DENSITY_AMPERE_PER_MICROMETRE)]
    AmperePerMicrometre,
}

/// The units of `ElectricPotential`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricPotential)]
pub enum ElectricPotentialUnit {
    #[unit("YV", "yottavolts", by = YOTTAVOLT)]
    Yottavolt,
    #[unit("ZV", "zettavolts", by = ZETTAVOLT)]
    Zettavolt,
    #[unit("EV", "exavolts", by = EXAVOLT)]
    Exavolt,
    #[unit("PV", "petavolts", by = PETAVOLT)]
    Petavolt,
    #[unit("TV", "teravolts", by = TERAVOLT)]
    Teravolt,
    #[unit("GV", "gigavolts", by = GIGAVOLT)]
    Gigavolt,
    #[unit("MV", "megavolts", by = MEGAVOLT)]
    Megavolt,
    #[unit("kV", "kilovolts", by = KILOVOLT)]
    Kilovolt,
    #[unit("hV", "hectovolts", by = HECTOVOLT)]
    Hectovolt,
    #[unit("daV", "decavolts", by = DECAVOLT)]
    Decavolt,
    #[default]
    #[unit("V", "volts", by = VOLT)]
    Volt,
    #[unit("dV", "decivolts", by = DECIVOLT)]
    Decivolt,
    #[unit("cV", "centivolts", by = CENTIVOLT)]
    Centivolt,
    #[unit("mV", "millivolts", by = MILLIVOLT)]
    Millivolt,
    #[unit("µV", "microvolts", by = MICROVOLT)]
    Microvolt,
    #[unit("nV", "nanovolts", by = NANOVOLT)]
    Nanovolt,
    #[unit("pV", "picovolts", by = PICOVOLT)]
    Picovolt,
    #[unit("fV", "femtovolts", by = FEMTOVOLT)]
    Femtovolt,
    #[unit("aV", "attovolts", by = ATTOVOLT)]
    Attovolt,
    #[unit("zV", "zeptovolts", by = ZEPTOVOLT)]
    Zeptovolt,
    #[unit("yV", "yoctovolts", by = YOCTOVOLT)]
    Yoctovolt,
    #[unit("abV", "abvolts", by = ABVOLT)]
    Abvolt,
    #[unit("statV", "statvolts", by = STATVOLT)]
    Statvolt,
}

/// The units of `ElectricField`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricField)]
pub enum ElectricFieldUnit {
    #[default]
    #[unit("V/m", "volts per metre", by = VOLT_PER_METRE)]
    VoltPerMetre,
    #[unit("V/cm", "volts per centimetre", by = VOLT_PER_CENTIMETRE)]
    VoltPerCentimetre,
    #[unit("V/mm", "volts per millimetre", by = VOLT_PER_MILLIMETRE)]
    VoltPerMillimetre,
    #[unit("V/μm", "volts per micrometre", by = VOLT_PER_MICROMETRE)]
    VoltPerMicrometre,
    #[unit("kV/mm", "kilovolts per millimetre", by = KILOVOLT_PER_MILLIMETRE)]
    KilovoltPerMillimetre,
    #[unit("MV/m", "megavolts per metre", by = MEGAVOLT_PER_METRE)]
    MegavoltPerMetre,
    #[unit("MV/cm", "megavolts per centimetre", by = MEGAVOLT_PER_CENTIMETRE)]
    MegavoltPerCentimetre,
    #[unit("V/mil", "volts per mil", by = VOLT_PER_MIL)]
    VoltPerMil,
    #[unit("a.u. of electric field", "atomic units of electric field", by = ATOMIC_UNIT_OF_ELECTRIC_FIELD)]
    AtomicUnitOfElectricField,
}

/// The units of `ElectricDisplacementField`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricDisplacementField)]
pub enum ElectricDisplacementFieldUnit {
    #[default]
    #[unit("C/m²", "coulombs per square metre", by = ELECTRIC_DISPLACEMENT_FIELD_COULOMB_PER_SQUARE_METRE)]
    CoulombPerSquareMetre,
}

/// The units of `ElectricFlux`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricFlux)]
pub enum ElectricFluxUnit {
    #[default]
    #[unit("V ⋅ m", "volt metres", by = VOLT_METRE)]
    VoltMetre,
    #[unit("V ⋅ cm", "volt centimetres", by = VOLT_CENTIMETRE)]
    VoltCentimetre,
}

/// The units of `ElectricDipoleMoment`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricDipoleMoment)]
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
#[unit(of = ElectricQuadrupoleMoment)]
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
#[unit(of = ElectricPermittivity)]
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
#[unit(of = ElectricalConductance)]
pub enum ElectricalConductanceUnit {
    #[unit("YS", "yottasiemens", by = YOTTASIEMENS)]
    Yottasiemens,
    #[unit("ZS", "zettasiemens", by = ZETTASIEMENS)]
    Zettasiemens,
    #[unit("ES", "exasiemens", by = EXASIEMENS)]
    Exasiemens,
    #[unit("PS", "petasiemens", by = PETASIEMENS)]
    Petasiemens,
    #[unit("TS", "terasiemens", by = TERASIEMENS)]
    Terasiemens,
    #[unit("GS", "gigasiemens", by = GIGASIEMENS)]
    Gigasiemens,
    #[unit("MS", "megasiemens", by = MEGASIEMENS)]
    Megasiemens,
    #[unit("kS", "kilosiemens", by = KILOSIEMENS)]
    Kilosiemens,
    #[unit("hS", "hectosiemens", by = HECTOSIEMENS)]
    Hectosiemens,
    #[unit("daS", "decasiemens", by = DECASIEMENS)]
    Decasiemens,
    #[default]
    #[unit("S", "siemens", by = SIEMENS)]
    Siemens,
    #[unit("℧", "mhos", by = MHO)]
    Mho,
    #[unit("dS", "decisiemens", by = DECISIEMENS)]
    Decisiemens,
    #[unit("cS", "centisiemens", by = CENTISIEMENS)]
    Centisiemens,
    #[unit("mS", "millisiemens", by = MILLISIEMENS)]
    Millisiemens,
    #[unit("µS", "microsiemens", by = MICROSIEMENS)]
    Microsiemens,
    #[unit("nS", "nanosiemens", by = NANOSIEMENS)]
    Nanosiemens,
    #[unit("pS", "picosiemens", by = PICOSIEMENS)]
    Picosiemens,
    #[unit("fS", "femtosiemens", by = FEMTOSIEMENS)]
    Femtosiemens,
    #[unit("aS", "attosiemens", by = ATTOSIEMENS)]
    Attosiemens,
    #[unit("zS", "zeptosiemens", by = ZEPTOSIEMENS)]
    Zeptosiemens,
    #[unit("yS", "yoctosiemens", by = YOCTOSIEMENS)]
    Yoctosiemens,
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
#[unit(of = ElectricalConductivity)]
pub enum ElectricalConductivityUnit {
    #[default]
    #[unit("S/m", "siemens per metre", by = SIEMENS_PER_METRE)]
    SiemensPerMetre,
    #[unit("S/cm", "siemens per centimetre", by = SIEMENS_PER_CENTIMETRE)]
    SiemensPerCentimetre,
}

/// The units of `ElectricalMobility`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricalMobility)]
pub enum ElectricalMobilityUnit {
    #[default]
    #[unit("m²/(V · s)", "square metres per volt second", by = SQUARE_METRE_PER_VOLT_SECOND)]
    SquareMetrePerVoltSecond,
    #[unit("cm²/(V · s)", "square centimetres per volt second", by = SQUARE_CENTIMETRE_PER_VOLT_SECOND)]
    SquareCentimetrePerVoltSecond,
}

/// The units of `ElectricalResistance`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricalResistance)]
pub enum ElectricalResistanceUnit {
    #[unit("YΩ", "yottaohms", by = YOTTAOHM)]
    Yottaohm,
    #[unit("ZΩ", "zettaohms", by = ZETTAOHM)]
    Zettaohm,
    #[unit("EΩ", "exaohms", by = EXAOHM)]
    Exaohm,
    #[unit("PΩ", "petaohms", by = PETAOHM)]
    Petaohm,
    #[unit("TΩ", "teraohms", by = TERAOHM)]
    Teraohm,
    #[unit("GΩ", "gigaohms", by = GIGAOHM)]
    Gigaohm,
    #[unit("MΩ", "megaohms", by = MEGAOHM)]
    Megaohm,
    #[unit("kΩ", "kiloohms", by = KILOOHM)]
    Kiloohm,
    #[unit("hΩ", "hectoohms", by = HECTOOHM)]
    Hectoohm,
    #[unit("daΩ", "decaohms", by = DECAOHM)]
    Decaohm,
    #[default]
    #[unit("Ω", "ohms", by = OHM)]
    Ohm,
    #[unit("dΩ", "deciohms", by = DECIOHM)]
    Deciohm,
    #[unit("cΩ", "centiohms", by = CENTIOHM)]
    Centiohm,
    #[unit("mΩ", "milliohms", by = MILLIOHM)]
    Milliohm,
    #[unit("µΩ", "microohms", by = MICROOHM)]
    Microohm,
    #[unit("nΩ", "nanoohms", by = NANOOHM)]
    Nanoohm,
    #[unit("pΩ", "picoohms", by = PICOOHM)]
    Picoohm,
    #[unit("fΩ", "femtoohms", by = FEMTOOHM)]
    Femtoohm,
    #[unit("aΩ", "attoohms", by = ATTOOHM)]
    Attoohm,
    #[unit("zΩ", "zeptoohms", by = ZEPTOOHM)]
    Zeptoohm,
    #[unit("yΩ", "yoctoohms", by = YOCTOOHM)]
    Yoctoohm,
    #[unit("abΩ", "abohms", by = ABOHM)]
    Abohm,
    #[unit("statΩ", "statohms", by = STATOHM)]
    Statohm,
}

/// The units of `ElectricalResistivity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricalResistivity)]
pub enum ElectricalResistivityUnit {
    #[unit("YΩ · m", "yottaohm metres", by = YOTTAOHM_METRE)]
    YottaohmMetre,
    #[unit("ZΩ · m", "zettaohm metres", by = ZETTAOHM_METRE)]
    ZettaohmMetre,
    #[unit("EΩ · m", "exaohm metres", by = EXAOHM_METRE)]
    ExaohmMetre,
    #[unit("PΩ · m", "petaohm metres", by = PETAOHM_METRE)]
    PetaohmMetre,
    #[unit("TΩ · m", "teraohm metres", by = TERAOHM_METRE)]
    TeraohmMetre,
    #[unit("GΩ · m", "gigaohm metres", by = GIGAOHM_METRE)]
    GigaohmMetre,
    #[unit("MΩ · m", "megaohm metres", by = MEGAOHM_METRE)]
    MegaohmMetre,
    #[unit("kΩ · m", "kiloohm metres", by = KILOOHM_METRE)]
    KiloohmMetre,
    #[unit("hΩ · m", "hectoohm metres", by = HECTOOHM_METRE)]
    HectoohmMetre,
    #[unit("daΩ · m", "decaohm metres", by = DECAOHM_METRE)]
    DecaohmMetre,
    #[default]
    #[unit("Ω · m", "ohm metres", by = OHM_METRE)]
    OhmMetre,
    #[unit("dΩ · m", "deciohm metres", by = DECIOHM_METRE)]
    DeciohmMetre,
    #[unit("cΩ · m", "centiohm metres", by = CENTIOHM_METRE)]
    CentiohmMetre,
    #[unit("mΩ · m", "milliohm metres", by = MILLIOHM_METRE)]
    MilliohmMetre,
    #[unit("µΩ · m", "microohm metres", by = MICROOHM_METRE)]
    MicroohmMetre,
    #[unit("nΩ · m", "nanoohm metres", by = NANOOHM_METRE)]
    NanoohmMetre,
    #[unit("pΩ · m", "picoohm metres", by = PICOOHM_METRE)]
    PicoohmMetre,
    #[unit("fΩ · m", "femtoohm metres", by = FEMTOOHM_METRE)]
    FemtoohmMetre,
    #[unit("aΩ · m", "attoohm metres", by = ATTOOHM_METRE)]
    AttoohmMetre,
    #[unit("zΩ · m", "zeptoohm metres", by = ZEPTOOHM_METRE)]
    ZeptoohmMetre,
    #[unit("yΩ · m", "yoctoohm metres", by = YOCTOOHM_METRE)]
    YoctoohmMetre,
    #[unit("abΩ · m", "abohm metres", by = ABOHM_METRE)]
    AbohmMetre,
    #[unit("statΩ · m", "statohm metres", by = STATOHM_METRE)]
    StatohmMetre,
    #[unit("Ω · cm", "ohm centimetres", by = OHM_CENTIMETRE)]
    OhmCentimetre,
    #[unit("abΩ · cm", "abohm centimetres", by = ABOHM_CENTIMETRE)]
    AbohmCentimetre,
    #[unit("statΩ · cm", "statohm centimetres", by = STATOHM_CENTIMETRE)]
    StatohmCentimetre,
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
#[unit(of = Capacitance)]
pub enum CapacitanceUnit {
    #[unit("YF", "yottafarads", by = YOTTAFARAD)]
    Yottafarad,
    #[unit("ZF", "zettafarads", by = ZETTAFARAD)]
    Zettafarad,
    #[unit("EF", "exafarads", by = EXAFARAD)]
    Exafarad,
    #[unit("PF", "petafarads", by = PETAFARAD)]
    Petafarad,
    #[unit("TF", "terafarads", by = TERAFARAD)]
    Terafarad,
    #[unit("GF", "gigafarads", by = GIGAFARAD)]
    Gigafarad,
    #[unit("MF", "megafarads", by = MEGAFARAD)]
    Megafarad,
    #[unit("kF", "kilofarads", by = KILOFARAD)]
    Kilofarad,
    #[unit("hF", "hectofarads", by = HECTOFARAD)]
    Hectofarad,
    #[unit("daF", "decafarads", by = DECAFARAD)]
    Decafarad,
    #[default]
    #[unit("F", "farads", by = FARAD)]
    Farad,
    #[unit("dF", "decifarads", by = DECIFARAD)]
    Decifarad,
    #[unit("cF", "centifarads", by = CENTIFARAD)]
    Centifarad,
    #[unit("mF", "millifarads", by = MILLIFARAD)]
    Millifarad,
    #[unit("µF", "microfarads", by = MICROFARAD)]
    Microfarad,
    #[unit("nF", "nanofarads", by = NANOFARAD)]
    Nanofarad,
    #[unit("pF", "picofarads", by = PICOFARAD)]
    Picofarad,
    #[unit("fF", "femtofarads", by = FEMTOFARAD)]
    Femtofarad,
    #[unit("aF", "attofarads", by = ATTOFARAD)]
    Attofarad,
    #[unit("zF", "zeptofarads", by = ZEPTOFARAD)]
    Zeptofarad,
    #[unit("yF", "yoctofarads", by = YOCTOFARAD)]
    Yoctofarad,
    #[unit("abF", "abfarads", by = ABFARAD)]
    Abfarad,
    #[unit("statF", "statfarads", by = STATFARAD)]
    Statfarad,
}

/// The units of `Inductance`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Inductance)]
pub enum InductanceUnit {
    #[unit("YH", "yottahenries", by = YOTTAHENRY)]
    Yottahenry,
    #[unit("ZH", "zettahenries", by = ZETTAHENRY)]
    Zettahenry,
    #[unit("EH", "exahenries", by = EXAHENRY)]
    Exahenry,
    #[unit("PH", "petahenries", by = PETAHENRY)]
    Petahenry,
    #[unit("TH", "terahenries", by = TERAHENRY)]
    Terahenry,
    #[unit("GH", "gigahenries", by = GIGAHENRY)]
    Gigahenry,
    #[unit("MH", "megahenries", by = MEGAHENRY)]
    Megahenry,
    #[unit("kH", "kilohenries", by = KILOHENRY)]
    Kilohenry,
    #[unit("hH", "hectohenries", by = HECTOHENRY)]
    Hectohenry,
    #[unit("daH", "decahenries", by = DECAHENRY)]
    Decahenry,
    #[default]
    #[unit("H", "henries", by = HENRY)]
    Henry,
    #[unit("dH", "decihenries", by = DECIHENRY)]
    Decihenry,
    #[unit("cH", "centihenries", by = CENTIHENRY)]
    Centihenry,
    #[unit("mH", "millihenries", by = MILLIHENRY)]
    Millihenry,
    #[unit("µH", "microhenries", by = MICROHENRY)]
    Microhenry,
    #[unit("nH", "nanohenries", by = NANOHENRY)]
    Nanohenry,
    #[unit("pH", "picohenries", by = PICOHENRY)]
    Picohenry,
    #[unit("fH", "femtohenries", by = FEMTOHENRY)]
    Femtohenry,
    #[unit("aH", "attohenries", by = ATTOHENRY)]
    Attohenry,
    #[unit("zH", "zeptohenries", by = ZEPTOHENRY)]
    Zeptohenry,
    #[unit("yH", "yoctohenries", by = YOCTOHENRY)]
    Yoctohenry,
    #[unit("abH", "abhenries", by = ABHENRY)]
    Abhenry,
    #[unit("statH", "stathenries", by = STATHENRY)]
    Stathenry,
}

/// The units of `MagneticFieldStrength`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticFieldStrength)]
pub enum MagneticFieldStrengthUnit {
    #[default]
    #[unit("A/m", "amperes per metre", by = MAGNETIC_FIELD_STRENGTH_AMPERE_PER_METRE)]
    AmperePerMetre,
    #[unit("A/cm", "amperes per centimetre", by = MAGNETIC_FIELD_STRENGTH_AMPERE_PER_CENTIMETRE)]
    AmperePerCentimetre,
    #[unit("A/mm", "amperes per millimetre", by = MAGNETIC_FIELD_STRENGTH_AMPERE_PER_MILLIMETRE)]
    AmperePerMillimetre,
    #[unit("A/μm", "amperes per micrometre", by = MAGNETIC_FIELD_STRENGTH_AMPERE_PER_MICROMETRE)]
    AmperePerMicrometre,
    #[unit("Oe", "oersteds", by = OERSTED)]
    Oersted,
}

/// The units of `MagneticFlux`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticFlux)]
pub enum MagneticFluxUnit {
    #[unit("YWb", "yottawebers", by = YOTTAWEBER)]
    Yottaweber,
    #[unit("ZWb", "zettawebers", by = ZETTAWEBER)]
    Zettaweber,
    #[unit("EWb", "exawebers", by = EXAWEBER)]
    Exaweber,
    #[unit("PWb", "petawebers", by = PETAWEBER)]
    Petaweber,
    #[unit("TWb", "terawebers", by = TERAWEBER)]
    Teraweber,
    #[unit("GWb", "gigawebers", by = GIGAWEBER)]
    Gigaweber,
    #[unit("MWb", "megawebers", by = MEGAWEBER)]
    Megaweber,
    #[unit("kWb", "kilowebers", by = KILOWEBER)]
    Kiloweber,
    #[unit("hWb", "hectowebers", by = HECTOWEBER)]
    Hectoweber,
    #[unit("daWb", "decawebers", by = DECAWEBER)]
    Decaweber,
    #[default]
    #[unit("Wb", "webers", by = WEBER)]
    Weber,
    #[unit("dWb", "deciwebers", by = DECIWEBER)]
    Deciweber,
    #[unit("cWb", "centiwebers", by = CENTIWEBER)]
    Centiweber,
    #[unit("mWb", "milliwebers", by = MILLIWEBER)]
    Milliweber,
    #[unit("µWb", "microwebers", by = MICROWEBER)]
    Microweber,
    #[unit("nWb", "nanowebers", by = NANOWEBER)]
    Nanoweber,
    #[unit("pWb", "picowebers", by = PICOWEBER)]
    Picoweber,
    #[unit("fWb", "femtowebers", by = FEMTOWEBER)]
    Femtoweber,
    #[unit("aWb", "attowebers", by = ATTOWEBER)]
    Attoweber,
    #[unit("zWb", "zeptowebers", by = ZEPTOWEBER)]
    Zeptoweber,
    #[unit("yWb", "yoctowebers", by = YOCTOWEBER)]
    Yoctoweber,
    #[unit("Mx", "maxwells", by = MAXWELL)]
    Maxwell,
}

/// The units of `MagneticFluxDensity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticFluxDensity)]
pub enum MagneticFluxDensityUnit {
    #[unit("YT", "yottateslas", by = YOTTATESLA)]
    Yottatesla,
    #[unit("ZT", "zettateslas", by = ZETTATESLA)]
    Zettatesla,
    #[unit("ET", "exateslas", by = EXATESLA)]
    Exatesla,
    #[unit("PT", "petateslas", by = PETATESLA)]
    Petatesla,
    #[unit("TT", "terateslas", by = TERATESLA)]
    Teratesla,
    #[unit("GT", "gigateslas", by = GIGATESLA)]
    Gigatesla,
    #[unit("MT", "megateslas", by = MEGATESLA)]
    Megatesla,
    #[unit("kT", "kiloteslas", by = KILOTESLA)]
    Kilotesla,
    #[unit("hT", "hectoteslas", by = HECTOTESLA)]
    Hectotesla,
    #[unit("daT", "decateslas", by = DECATESLA)]
    Decatesla,
    #[default]
    #[unit("T", "teslas", by = TESLA)]
    Tesla,
    #[unit("dT", "deciteslas", by = DECITESLA)]
    Decitesla,
    #[unit("cT", "centiteslas", by = CENTITESLA)]
    Centitesla,
    #[unit("mT", "milliteslas", by = MILLITESLA)]
    Millitesla,
    #[unit("µT", "microteslas", by = MICROTESLA)]
    Microtesla,
    #[unit("nT", "nanoteslas", by = NANOTESLA)]
    Nanotesla,
    #[unit("pT", "picoteslas", by = PICOTESLA)]
    Picotesla,
    #[unit("fT", "femtoteslas", by = FEMTOTESLA)]
    Femtotesla,
    #[unit("aT", "attoteslas", by = ATTOTESLA)]
    Attotesla,
    #[unit("zT", "zeptoteslas", by = ZEPTOTESLA)]
    Zeptotesla,
    #[unit("yT", "yoctoteslas", by = YOCTOTESLA)]
    Yoctotesla,
    #[unit("γ", "gammas", by = GAMMA)]
    Gamma,
    #[unit("G", "gauss", by = GAUSS)]
    Gauss,
}

/// The units of `MagneticMoment`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticMoment)]
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
#[unit(of = MagneticPermeability)]
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
#[unit(of = GyromagneticRatio)]
pub enum GyromagneticRatioUnit {
    #[unit("YHz/T", "yottahertz per tesla", by = YOTTAHERTZ_PER_TESLA)]
    YottahertzPerTesla,
    #[unit("ZHz/T", "zettahertz per tesla", by = ZETTAHERTZ_PER_TESLA)]
    ZettahertzPerTesla,
    #[unit("EHz/T", "exahertz per tesla", by = EXAHERTZ_PER_TESLA)]
    ExahertzPerTesla,
    #[unit("PHz/T", "petahertz per tesla", by = PETAHERTZ_PER_TESLA)]
    PetahertzPerTesla,
    #[unit("THz/T", "terahertz per tesla", by = TERAHERTZ_PER_TESLA)]
    TerahertzPerTesla,
    #[unit("GHz/T", "gigahertz per tesla", by = GIGAHERTZ_PER_TESLA)]
    GigahertzPerTesla,
    #[unit("MHz/T", "megahertz per tesla", by = MEGAHERTZ_PER_TESLA)]
    MegahertzPerTesla,
    #[unit("kHz/T", "kilohertz per tesla", by = KILOHERTZ_PER_TESLA)]
    KilohertzPerTesla,
    #[unit("hHz/T", "hectohertz per tesla", by = HECTOHERTZ_PER_TESLA)]
    HectohertzPerTesla,
    #[unit("daHz/T", "decahertz per tesla", by = DECAHERTZ_PER_TESLA)]
    DecahertzPerTesla,
    #[default]
    #[unit("Hz/T", "hertz per tesla", by = HERTZ_PER_TESLA)]
    HertzPerTesla,
    #[unit("dHz/T", "decihertz per tesla", by = DECIHERTZ_PER_TESLA)]
    DecihertzPerTesla,
    #[unit("cHz/T", "centihertz per tesla", by = CENTIHERTZ_PER_TESLA)]
    CentihertzPerTesla,
    #[unit("mHz/T", "millihertz per tesla", by = MILLIHERTZ_PER_TESLA)]
    MillihertzPerTesla,
    #[unit("µHz/T", "microhertz per tesla", by = MICROHERTZ_PER_TESLA)]
    MicrohertzPerTesla,
    #[unit("nHz/T", "nanohertz per tesla", by = NANOHERTZ_PER_TESLA)]
    NanohertzPerTesla,
    #[unit("pHz/T", "picohertz per tesla", by = PICOHERTZ_PER_TESLA)]
    PicohertzPerTesla,
    #[unit("fHz/T", "femtohertz per tesla", by = FEMTOHERTZ_PER_TESLA)]
    FemtohertzPerTesla,
    #[unit("aHz/T", "attohertz per tesla", by = ATTOHERTZ_PER_TESLA)]
    AttohertzPerTesla,
    #[unit("zHz/T", "zeptohertz per tesla", by = ZEPTOHERTZ_PER_TESLA)]
    ZeptohertzPerTesla,
    #[unit("yHz/T", "yoctohertz per tesla", by = YOCTOHERTZ_PER_TESLA)]
    YoctohertzPerTesla,
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
#[unit(of = ReducedGyromagneticRatio)]
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
