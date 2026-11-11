use crate::{Exact, ISQ, SIQuantity, counted};
use consts::*;
use yadc::Dimension;

pub type ElectricCharge<N = f64> = SIQuantity<
    {
        ISQ {
            s: 1,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricChargeArealDensity<N = f64> = SIQuantity<
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

pub type ElectricChargeLinearDensity<N = f64> = SIQuantity<
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

pub type ElectricChargeVolumetricDensity<N = f64> = SIQuantity<
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

pub type ElectricCurrent<N = f64> = SIQuantity<
    {
        ISQ {
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricCurrentDensity<N = f64> = SIQuantity<
    {
        ISQ {
            m: -2,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type SurfaceElectricCurrentDensity<N = f64> = SIQuantity<
    {
        ISQ {
            m: -1,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricPotential<N = f64> = SIQuantity<
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

pub type ElectricField<N = f64> = SIQuantity<
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

pub type ElectricDisplacementField<N = f64> = SIQuantity<
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

pub type ElectricFlux<N = f64> = SIQuantity<
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

pub type ElectricDipoleMoment<N = f64> = SIQuantity<
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

pub type ElectricQuadrupoleMoment<N = f64> = SIQuantity<
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

pub type ElectricPermittivity<N = f64> = SIQuantity<
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

pub type ElectricalConductance<N = f64> = SIQuantity<
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

pub type ElectricalConductivity<N = f64> = SIQuantity<
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

pub type ElectricalMobility<N = f64> = SIQuantity<
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

pub type ElectricalResistance<N = f64> = SIQuantity<
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

pub type ElectricalResistivity<N = f64> = SIQuantity<
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

pub type Capacitance<N = f64> = SIQuantity<
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

pub type Inductance<N = f64> = SIQuantity<
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

pub type MagneticFieldStrength<N = f64> = SIQuantity<
    {
        ISQ {
            m: -1,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MagneticFlux<N = f64> = SIQuantity<
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

pub type MagneticFluxDensity<N = f64> = SIQuantity<
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

pub type MagneticMoment<N = f64> = SIQuantity<
    {
        ISQ {
            m: 2,
            A: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MagneticPermeability<N = f64> = SIQuantity<
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

pub type GyromagneticRatio<N = f64> = SIQuantity<
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

pub type ReducedGyromagneticRatio<N = f64> = SIQuantity<
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
/// `KILOMETRE` is `1000 * METRE` - and counted in the number type its use
/// asks for, rounded once: `let d: Length = KILOMETRE;` is an `f64` length,
/// `Length<f32>` an `f32` one, `KILOMETRE::<Exact>` the exact value. Each is
/// a unit (`KILOMETRE(3.0)`, `KILOMETRE.value(d)`), and they compose in
/// `const` items: `const LIMIT: Velocity = 50.0 * KILOMETRE / HOUR;`.
pub mod consts {
    use super::*;
    use crate::base::{KILOGRAM, METRE, RADIAN, SECOND};

    /// The ampere, the SI base unit of electric current.
    pub use crate::base::AMPERE;

    /// One `Yottacoulomb` (YC), as an `ElectricCharge`.
    pub const YOTTACOULOMB<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::int(1000000000000000000000000) * COULOMB::<Exact>);

    /// One `Zettacoulomb` (ZC), as an `ElectricCharge`.
    pub const ZETTACOULOMB<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::int(1000000000000000000000) * COULOMB::<Exact>);

    /// One `Exacoulomb` (EC), as an `ElectricCharge`.
    pub const EXACOULOMB<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::int(1000000000000000000) * COULOMB::<Exact>);

    /// One `Petacoulomb` (PC), as an `ElectricCharge`.
    pub const PETACOULOMB<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::int(1000000000000000) * COULOMB::<Exact>);

    /// One `Teracoulomb` (TC), as an `ElectricCharge`.
    pub const TERACOULOMB<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::int(1000000000000) * COULOMB::<Exact>);

    /// One `Gigacoulomb` (GC), as an `ElectricCharge`.
    pub const GIGACOULOMB<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::int(1000000000) * COULOMB::<Exact>);

    /// One `Megacoulomb` (MC), as an `ElectricCharge`.
    pub const MEGACOULOMB<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::int(1000000) * COULOMB::<Exact>);

    /// One `Kilocoulomb` (kC), as an `ElectricCharge`.
    pub const KILOCOULOMB<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::int(1000) * COULOMB::<Exact>);

    /// One `Hectocoulomb` (hC), as an `ElectricCharge`.
    pub const HECTOCOULOMB<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::int(100) * COULOMB::<Exact>);

    /// One `Decacoulomb` (daC), as an `ElectricCharge`.
    pub const DECACOULOMB<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::int(10) * COULOMB::<Exact>);

    /// One `Coulomb` (C), as an `ElectricCharge`.
    pub const COULOMB<N: const From<Exact>>: ElectricCharge<N> = counted(SECOND::<Exact> * AMPERE::<Exact>);

    /// One `Decicoulomb` (dC), as an `ElectricCharge`.
    pub const DECICOULOMB<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::dec(1, -1) * COULOMB::<Exact>);

    /// One `Centicoulomb` (cC), as an `ElectricCharge`.
    pub const CENTICOULOMB<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::dec(1, -2) * COULOMB::<Exact>);

    /// One `Millicoulomb` (mC), as an `ElectricCharge`.
    pub const MILLICOULOMB<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::dec(1, -3) * COULOMB::<Exact>);

    /// One `Microcoulomb` (µC), as an `ElectricCharge`.
    pub const MICROCOULOMB<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::dec(1, -6) * COULOMB::<Exact>);

    /// One `Nanocoulomb` (nC), as an `ElectricCharge`.
    pub const NANOCOULOMB<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::dec(1, -9) * COULOMB::<Exact>);

    /// One `Picocoulomb` (pC), as an `ElectricCharge`.
    pub const PICOCOULOMB<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::dec(1, -12) * COULOMB::<Exact>);

    /// One `Femtocoulomb` (fC), as an `ElectricCharge`.
    pub const FEMTOCOULOMB<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::dec(1, -15) * COULOMB::<Exact>);

    /// One `Attocoulomb` (aC), as an `ElectricCharge`.
    pub const ATTOCOULOMB<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::dec(1, -18) * COULOMB::<Exact>);

    /// One `Zeptocoulomb` (zC), as an `ElectricCharge`.
    pub const ZEPTOCOULOMB<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::dec(1, -21) * COULOMB::<Exact>);

    /// One `Yoctocoulomb` (yC), as an `ElectricCharge`.
    pub const YOCTOCOULOMB<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::dec(1, -24) * COULOMB::<Exact>);

    /// One `PetaampereHour` (PA · h), as an `ElectricCharge`.
    pub const PETAAMPERE_HOUR<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::int(3600000000000000000) * COULOMB::<Exact>);

    /// One `TeraampereHour` (TA · h), as an `ElectricCharge`.
    pub const TERAAMPERE_HOUR<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::int(3600000000000000) * COULOMB::<Exact>);

    /// One `GigaampereHour` (GA · h), as an `ElectricCharge`.
    pub const GIGAAMPERE_HOUR<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::int(3600000000000) * COULOMB::<Exact>);

    /// One `MegaampereHour` (MA · h), as an `ElectricCharge`.
    pub const MEGAAMPERE_HOUR<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::int(3600000000) * COULOMB::<Exact>);

    /// One `KiloampereHour` (kA · h), as an `ElectricCharge`.
    pub const KILOAMPERE_HOUR<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::int(3600000) * COULOMB::<Exact>);

    /// One `HectoampereHour` (hA · h), as an `ElectricCharge`.
    pub const HECTOAMPERE_HOUR<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::int(360000) * COULOMB::<Exact>);

    /// One `DecaampereHour` (daA · h), as an `ElectricCharge`.
    pub const DECAAMPERE_HOUR<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::int(36000) * COULOMB::<Exact>);

    /// One `AmpereHour` (A · h), as an `ElectricCharge`.
    pub const AMPERE_HOUR<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::int(3600) * COULOMB::<Exact>);

    /// One `MilliampereHour` (mA · h), as an `ElectricCharge`.
    pub const MILLIAMPERE_HOUR<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::dec(36, -1) * COULOMB::<Exact>);

    /// One `MicroampereHour` (µA · h), as an `ElectricCharge`.
    pub const MICROAMPERE_HOUR<N: const From<Exact>>: ElectricCharge<N> = counted(Exact::dec(36, -4) * COULOMB::<Exact>);

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

    /// One `CoulombPerCentimetre` (C/cm), as an `ElectricChargeLinearDensity`.
    pub const COULOMB_PER_CENTIMETRE<N: const From<Exact>>: ElectricChargeLinearDensity<N> = counted(Exact::int(100) * COULOMB_PER_METRE::<Exact>);

    /// One `CoulombPerCubicMetre` (C/m³), as an `ElectricChargeVolumetricDensity`.
    pub const COULOMB_PER_CUBIC_METRE<N: const From<Exact>>: ElectricChargeVolumetricDensity<N> = counted(SECOND::<Exact> * AMPERE::<Exact> / (METRE::<Exact> * METRE::<Exact> * METRE::<Exact>));

    /// One `Yottaampere` (YA), as an `ElectricCurrent`.
    pub const YOTTAAMPERE<N: const From<Exact>>: ElectricCurrent<N> = counted(Exact::int(1000000000000000000000000) * AMPERE::<Exact>);

    /// One `Zettaampere` (ZA), as an `ElectricCurrent`.
    pub const ZETTAAMPERE<N: const From<Exact>>: ElectricCurrent<N> = counted(Exact::int(1000000000000000000000) * AMPERE::<Exact>);

    /// One `Exaampere` (EA), as an `ElectricCurrent`.
    pub const EXAAMPERE<N: const From<Exact>>: ElectricCurrent<N> = counted(Exact::int(1000000000000000000) * AMPERE::<Exact>);

    /// One `Petaampere` (PA), as an `ElectricCurrent`.
    pub const PETAAMPERE<N: const From<Exact>>: ElectricCurrent<N> = counted(Exact::int(1000000000000000) * AMPERE::<Exact>);

    /// One `Teraampere` (TA), as an `ElectricCurrent`.
    pub const TERAAMPERE<N: const From<Exact>>: ElectricCurrent<N> = counted(Exact::int(1000000000000) * AMPERE::<Exact>);

    /// One `Gigaampere` (GA), as an `ElectricCurrent`.
    pub const GIGAAMPERE<N: const From<Exact>>: ElectricCurrent<N> = counted(Exact::int(1000000000) * AMPERE::<Exact>);

    /// One `Megaampere` (MA), as an `ElectricCurrent`.
    pub const MEGAAMPERE<N: const From<Exact>>: ElectricCurrent<N> = counted(Exact::int(1000000) * AMPERE::<Exact>);

    /// One `Kiloampere` (kA), as an `ElectricCurrent`.
    pub const KILOAMPERE<N: const From<Exact>>: ElectricCurrent<N> = counted(Exact::int(1000) * AMPERE::<Exact>);

    /// One `Hectoampere` (hA), as an `ElectricCurrent`.
    pub const HECTOAMPERE<N: const From<Exact>>: ElectricCurrent<N> = counted(Exact::int(100) * AMPERE::<Exact>);

    /// One `Decaampere` (daA), as an `ElectricCurrent`.
    pub const DECAAMPERE<N: const From<Exact>>: ElectricCurrent<N> = counted(Exact::int(10) * AMPERE::<Exact>);

    /// One `Deciampere` (dA), as an `ElectricCurrent`.
    pub const DECIAMPERE<N: const From<Exact>>: ElectricCurrent<N> = counted(Exact::dec(1, -1) * AMPERE::<Exact>);

    /// One `Centiampere` (cA), as an `ElectricCurrent`.
    pub const CENTIAMPERE<N: const From<Exact>>: ElectricCurrent<N> = counted(Exact::dec(1, -2) * AMPERE::<Exact>);

    /// One `Milliampere` (mA), as an `ElectricCurrent`.
    pub const MILLIAMPERE<N: const From<Exact>>: ElectricCurrent<N> = counted(Exact::dec(1, -3) * AMPERE::<Exact>);

    /// One `Microampere` (µA), as an `ElectricCurrent`.
    pub const MICROAMPERE<N: const From<Exact>>: ElectricCurrent<N> = counted(Exact::dec(1, -6) * AMPERE::<Exact>);

    /// One `Nanoampere` (nA), as an `ElectricCurrent`.
    pub const NANOAMPERE<N: const From<Exact>>: ElectricCurrent<N> = counted(Exact::dec(1, -9) * AMPERE::<Exact>);

    /// One `Picoampere` (pA), as an `ElectricCurrent`.
    pub const PICOAMPERE<N: const From<Exact>>: ElectricCurrent<N> = counted(Exact::dec(1, -12) * AMPERE::<Exact>);

    /// One `Femtoampere` (fA), as an `ElectricCurrent`.
    pub const FEMTOAMPERE<N: const From<Exact>>: ElectricCurrent<N> = counted(Exact::dec(1, -15) * AMPERE::<Exact>);

    /// One `Attoampere` (aA), as an `ElectricCurrent`.
    pub const ATTOAMPERE<N: const From<Exact>>: ElectricCurrent<N> = counted(Exact::dec(1, -18) * AMPERE::<Exact>);

    /// One `Zeptoampere` (zA), as an `ElectricCurrent`.
    pub const ZEPTOAMPERE<N: const From<Exact>>: ElectricCurrent<N> = counted(Exact::dec(1, -21) * AMPERE::<Exact>);

    /// One `Yoctoampere` (yA), as an `ElectricCurrent`.
    pub const YOCTOAMPERE<N: const From<Exact>>: ElectricCurrent<N> = counted(Exact::dec(1, -24) * AMPERE::<Exact>);

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

    /// One `AmperePerCentimetre` (A/cm), as a `SurfaceElectricCurrentDensity`.
    pub const SURFACE_ELECTRIC_CURRENT_DENSITY_AMPERE_PER_CENTIMETRE<N: const From<Exact>>: SurfaceElectricCurrentDensity<N> = counted(Exact::int(100) * SURFACE_ELECTRIC_CURRENT_DENSITY_AMPERE_PER_METRE::<Exact>);

    /// One `AmperePerMillimetre` (A/mm), as a `SurfaceElectricCurrentDensity`.
    pub const SURFACE_ELECTRIC_CURRENT_DENSITY_AMPERE_PER_MILLIMETRE<N: const From<Exact>>: SurfaceElectricCurrentDensity<N> = counted(Exact::int(1000) * SURFACE_ELECTRIC_CURRENT_DENSITY_AMPERE_PER_METRE::<Exact>);

    /// One `AmperePerMicrometre` (A/μm), as a `SurfaceElectricCurrentDensity`.
    pub const SURFACE_ELECTRIC_CURRENT_DENSITY_AMPERE_PER_MICROMETRE<N: const From<Exact>>: SurfaceElectricCurrentDensity<N> = counted(Exact::int(1000000) * SURFACE_ELECTRIC_CURRENT_DENSITY_AMPERE_PER_METRE::<Exact>);

    /// One `Yottavolt` (YV), as an `ElectricPotential`.
    pub const YOTTAVOLT<N: const From<Exact>>: ElectricPotential<N> = counted(Exact::int(1000000000000000000000000) * VOLT::<Exact>);

    /// One `Zettavolt` (ZV), as an `ElectricPotential`.
    pub const ZETTAVOLT<N: const From<Exact>>: ElectricPotential<N> = counted(Exact::int(1000000000000000000000) * VOLT::<Exact>);

    /// One `Exavolt` (EV), as an `ElectricPotential`.
    pub const EXAVOLT<N: const From<Exact>>: ElectricPotential<N> = counted(Exact::int(1000000000000000000) * VOLT::<Exact>);

    /// One `Petavolt` (PV), as an `ElectricPotential`.
    pub const PETAVOLT<N: const From<Exact>>: ElectricPotential<N> = counted(Exact::int(1000000000000000) * VOLT::<Exact>);

    /// One `Teravolt` (TV), as an `ElectricPotential`.
    pub const TERAVOLT<N: const From<Exact>>: ElectricPotential<N> = counted(Exact::int(1000000000000) * VOLT::<Exact>);

    /// One `Gigavolt` (GV), as an `ElectricPotential`.
    pub const GIGAVOLT<N: const From<Exact>>: ElectricPotential<N> = counted(Exact::int(1000000000) * VOLT::<Exact>);

    /// One `Megavolt` (MV), as an `ElectricPotential`.
    pub const MEGAVOLT<N: const From<Exact>>: ElectricPotential<N> = counted(Exact::int(1000000) * VOLT::<Exact>);

    /// One `Kilovolt` (kV), as an `ElectricPotential`.
    pub const KILOVOLT<N: const From<Exact>>: ElectricPotential<N> = counted(Exact::int(1000) * VOLT::<Exact>);

    /// One `Hectovolt` (hV), as an `ElectricPotential`.
    pub const HECTOVOLT<N: const From<Exact>>: ElectricPotential<N> = counted(Exact::int(100) * VOLT::<Exact>);

    /// One `Decavolt` (daV), as an `ElectricPotential`.
    pub const DECAVOLT<N: const From<Exact>>: ElectricPotential<N> = counted(Exact::int(10) * VOLT::<Exact>);

    /// One `Volt` (V), as an `ElectricPotential`.
    pub const VOLT<N: const From<Exact>>: ElectricPotential<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * AMPERE::<Exact>));

    /// One `Decivolt` (dV), as an `ElectricPotential`.
    pub const DECIVOLT<N: const From<Exact>>: ElectricPotential<N> = counted(Exact::dec(1, -1) * VOLT::<Exact>);

    /// One `Centivolt` (cV), as an `ElectricPotential`.
    pub const CENTIVOLT<N: const From<Exact>>: ElectricPotential<N> = counted(Exact::dec(1, -2) * VOLT::<Exact>);

    /// One `Millivolt` (mV), as an `ElectricPotential`.
    pub const MILLIVOLT<N: const From<Exact>>: ElectricPotential<N> = counted(Exact::dec(1, -3) * VOLT::<Exact>);

    /// One `Microvolt` (µV), as an `ElectricPotential`.
    pub const MICROVOLT<N: const From<Exact>>: ElectricPotential<N> = counted(Exact::dec(1, -6) * VOLT::<Exact>);

    /// One `Nanovolt` (nV), as an `ElectricPotential`.
    pub const NANOVOLT<N: const From<Exact>>: ElectricPotential<N> = counted(Exact::dec(1, -9) * VOLT::<Exact>);

    /// One `Picovolt` (pV), as an `ElectricPotential`.
    pub const PICOVOLT<N: const From<Exact>>: ElectricPotential<N> = counted(Exact::dec(1, -12) * VOLT::<Exact>);

    /// One `Femtovolt` (fV), as an `ElectricPotential`.
    pub const FEMTOVOLT<N: const From<Exact>>: ElectricPotential<N> = counted(Exact::dec(1, -15) * VOLT::<Exact>);

    /// One `Attovolt` (aV), as an `ElectricPotential`.
    pub const ATTOVOLT<N: const From<Exact>>: ElectricPotential<N> = counted(Exact::dec(1, -18) * VOLT::<Exact>);

    /// One `Zeptovolt` (zV), as an `ElectricPotential`.
    pub const ZEPTOVOLT<N: const From<Exact>>: ElectricPotential<N> = counted(Exact::dec(1, -21) * VOLT::<Exact>);

    /// One `Yoctovolt` (yV), as an `ElectricPotential`.
    pub const YOCTOVOLT<N: const From<Exact>>: ElectricPotential<N> = counted(Exact::dec(1, -24) * VOLT::<Exact>);

    /// One `Abvolt` (abV), as an `ElectricPotential`.
    pub const ABVOLT<N: const From<Exact>>: ElectricPotential<N> = counted(Exact::dec(1, -8) * VOLT::<Exact>);

    /// One `Statvolt` (statV), as an `ElectricPotential`.
    pub const STATVOLT<N: const From<Exact>>: ElectricPotential<N> = counted(Exact::dec(299792458, -6) * VOLT::<Exact>);

    /// One `VoltPerMetre` (V/m), as an `ElectricField`.
    pub const VOLT_PER_METRE<N: const From<Exact>>: ElectricField<N> = counted(METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * AMPERE::<Exact>));

    /// One `VoltPerCentimetre` (V/cm), as an `ElectricField`.
    pub const VOLT_PER_CENTIMETRE<N: const From<Exact>>: ElectricField<N> = counted(Exact::int(100) * VOLT_PER_METRE::<Exact>);

    /// One `VoltPerMillimetre` (V/mm), as an `ElectricField`.
    pub const VOLT_PER_MILLIMETRE<N: const From<Exact>>: ElectricField<N> = counted(Exact::int(1000) * VOLT_PER_METRE::<Exact>);

    /// One `VoltPerMicrometre` (V/μm), as an `ElectricField`.
    pub const VOLT_PER_MICROMETRE<N: const From<Exact>>: ElectricField<N> = counted(Exact::int(1000000) * VOLT_PER_METRE::<Exact>);

    /// One `KilovoltPerMillimetre` (kV/mm), as an `ElectricField`.
    pub const KILOVOLT_PER_MILLIMETRE<N: const From<Exact>>: ElectricField<N> = counted(Exact::int(1000000) * VOLT_PER_METRE::<Exact>);

    /// One `MegavoltPerMetre` (MV/m), as an `ElectricField`.
    pub const MEGAVOLT_PER_METRE<N: const From<Exact>>: ElectricField<N> = counted(Exact::int(1000000) * VOLT_PER_METRE::<Exact>);

    /// One `MegavoltPerCentimetre` (MV/cm), as an `ElectricField`.
    pub const MEGAVOLT_PER_CENTIMETRE<N: const From<Exact>>: ElectricField<N> = counted(Exact::int(100000000) * VOLT_PER_METRE::<Exact>);

    /// One `VoltPerMil` (V/mil), as an `ElectricField`.
    pub const VOLT_PER_MIL<N: const From<Exact>>: ElectricField<N> = counted((Exact::int(1) / Exact::dec(254, -7)) * VOLT_PER_METRE::<Exact>);

    /// One `AtomicUnitOfElectricField` (a.u. of electric field), as an `ElectricField`.
    pub const ATOMIC_UNIT_OF_ELECTRIC_FIELD<N: const From<Exact>>: ElectricField<N> = counted((Exact::dec(43597447222071, -31) / (Exact::dec(1602176634, -28) * Exact::dec(529177210903, -22))) * VOLT_PER_METRE::<Exact>);

    /// One `CoulombPerSquareMetre` (C/m²), as an `ElectricDisplacementField`.
    pub const ELECTRIC_DISPLACEMENT_FIELD_COULOMB_PER_SQUARE_METRE<N: const From<Exact>>: ElectricDisplacementField<N> = counted(SECOND::<Exact> * AMPERE::<Exact> / (METRE::<Exact> * METRE::<Exact>));

    /// One `VoltMetre` (V ⋅ m), as an `ElectricFlux`.
    pub const VOLT_METRE<N: const From<Exact>>: ElectricFlux<N> = counted(METRE::<Exact> * METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * AMPERE::<Exact>));

    /// One `VoltCentimetre` (V ⋅ cm), as an `ElectricFlux`.
    pub const VOLT_CENTIMETRE<N: const From<Exact>>: ElectricFlux<N> = counted(Exact::dec(1, -2) * VOLT_METRE::<Exact>);

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

    /// One `Yottasiemens` (YS), as an `ElectricalConductance`.
    pub const YOTTASIEMENS<N: const From<Exact>>: ElectricalConductance<N> = counted(Exact::int(1000000000000000000000000) * SIEMENS::<Exact>);

    /// One `Zettasiemens` (ZS), as an `ElectricalConductance`.
    pub const ZETTASIEMENS<N: const From<Exact>>: ElectricalConductance<N> = counted(Exact::int(1000000000000000000000) * SIEMENS::<Exact>);

    /// One `Exasiemens` (ES), as an `ElectricalConductance`.
    pub const EXASIEMENS<N: const From<Exact>>: ElectricalConductance<N> = counted(Exact::int(1000000000000000000) * SIEMENS::<Exact>);

    /// One `Petasiemens` (PS), as an `ElectricalConductance`.
    pub const PETASIEMENS<N: const From<Exact>>: ElectricalConductance<N> = counted(Exact::int(1000000000000000) * SIEMENS::<Exact>);

    /// One `Terasiemens` (TS), as an `ElectricalConductance`.
    pub const TERASIEMENS<N: const From<Exact>>: ElectricalConductance<N> = counted(Exact::int(1000000000000) * SIEMENS::<Exact>);

    /// One `Gigasiemens` (GS), as an `ElectricalConductance`.
    pub const GIGASIEMENS<N: const From<Exact>>: ElectricalConductance<N> = counted(Exact::int(1000000000) * SIEMENS::<Exact>);

    /// One `Megasiemens` (MS), as an `ElectricalConductance`.
    pub const MEGASIEMENS<N: const From<Exact>>: ElectricalConductance<N> = counted(Exact::int(1000000) * SIEMENS::<Exact>);

    /// One `Kilosiemens` (kS), as an `ElectricalConductance`.
    pub const KILOSIEMENS<N: const From<Exact>>: ElectricalConductance<N> = counted(Exact::int(1000) * SIEMENS::<Exact>);

    /// One `Hectosiemens` (hS), as an `ElectricalConductance`.
    pub const HECTOSIEMENS<N: const From<Exact>>: ElectricalConductance<N> = counted(Exact::int(100) * SIEMENS::<Exact>);

    /// One `Decasiemens` (daS), as an `ElectricalConductance`.
    pub const DECASIEMENS<N: const From<Exact>>: ElectricalConductance<N> = counted(Exact::int(10) * SIEMENS::<Exact>);

    /// One `Siemens` (S), as an `ElectricalConductance`.
    pub const SIEMENS<N: const From<Exact>>: ElectricalConductance<N> = counted(SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * AMPERE::<Exact> * AMPERE::<Exact> / (METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact>));

    /// One `Mho` (℧), as an `ElectricalConductance`.
    pub const MHO<N: const From<Exact>>: ElectricalConductance<N> = counted(SIEMENS::<Exact>);

    /// One `Decisiemens` (dS), as an `ElectricalConductance`.
    pub const DECISIEMENS<N: const From<Exact>>: ElectricalConductance<N> = counted(Exact::dec(1, -1) * SIEMENS::<Exact>);

    /// One `Centisiemens` (cS), as an `ElectricalConductance`.
    pub const CENTISIEMENS<N: const From<Exact>>: ElectricalConductance<N> = counted(Exact::dec(1, -2) * SIEMENS::<Exact>);

    /// One `Millisiemens` (mS), as an `ElectricalConductance`.
    pub const MILLISIEMENS<N: const From<Exact>>: ElectricalConductance<N> = counted(Exact::dec(1, -3) * SIEMENS::<Exact>);

    /// One `Microsiemens` (µS), as an `ElectricalConductance`.
    pub const MICROSIEMENS<N: const From<Exact>>: ElectricalConductance<N> = counted(Exact::dec(1, -6) * SIEMENS::<Exact>);

    /// One `Nanosiemens` (nS), as an `ElectricalConductance`.
    pub const NANOSIEMENS<N: const From<Exact>>: ElectricalConductance<N> = counted(Exact::dec(1, -9) * SIEMENS::<Exact>);

    /// One `Picosiemens` (pS), as an `ElectricalConductance`.
    pub const PICOSIEMENS<N: const From<Exact>>: ElectricalConductance<N> = counted(Exact::dec(1, -12) * SIEMENS::<Exact>);

    /// One `Femtosiemens` (fS), as an `ElectricalConductance`.
    pub const FEMTOSIEMENS<N: const From<Exact>>: ElectricalConductance<N> = counted(Exact::dec(1, -15) * SIEMENS::<Exact>);

    /// One `Attosiemens` (aS), as an `ElectricalConductance`.
    pub const ATTOSIEMENS<N: const From<Exact>>: ElectricalConductance<N> = counted(Exact::dec(1, -18) * SIEMENS::<Exact>);

    /// One `Zeptosiemens` (zS), as an `ElectricalConductance`.
    pub const ZEPTOSIEMENS<N: const From<Exact>>: ElectricalConductance<N> = counted(Exact::dec(1, -21) * SIEMENS::<Exact>);

    /// One `Yoctosiemens` (yS), as an `ElectricalConductance`.
    pub const YOCTOSIEMENS<N: const From<Exact>>: ElectricalConductance<N> = counted(Exact::dec(1, -24) * SIEMENS::<Exact>);

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

    /// One `SiemensPerCentimetre` (S/cm), as an `ElectricalConductivity`.
    pub const SIEMENS_PER_CENTIMETRE<N: const From<Exact>>: ElectricalConductivity<N> = counted(Exact::int(100) * SIEMENS_PER_METRE::<Exact>);

    /// One `SquareMetrePerVoltSecond` (m²/(V · s)), as an `ElectricalMobility`.
    pub const SQUARE_METRE_PER_VOLT_SECOND<N: const From<Exact>>: ElectricalMobility<N> = counted(SECOND::<Exact> * SECOND::<Exact> * AMPERE::<Exact> / KILOGRAM::<Exact>);

    /// One `SquareCentimetrePerVoltSecond` (cm²/(V · s)), as an `ElectricalMobility`.
    pub const SQUARE_CENTIMETRE_PER_VOLT_SECOND<N: const From<Exact>>: ElectricalMobility<N> = counted(Exact::dec(1, -4) * SQUARE_METRE_PER_VOLT_SECOND::<Exact>);

    /// One `Yottaohm` (YΩ), as an `ElectricalResistance`.
    pub const YOTTAOHM<N: const From<Exact>>: ElectricalResistance<N> = counted(Exact::int(1000000000000000000000000) * OHM::<Exact>);

    /// One `Zettaohm` (ZΩ), as an `ElectricalResistance`.
    pub const ZETTAOHM<N: const From<Exact>>: ElectricalResistance<N> = counted(Exact::int(1000000000000000000000) * OHM::<Exact>);

    /// One `Exaohm` (EΩ), as an `ElectricalResistance`.
    pub const EXAOHM<N: const From<Exact>>: ElectricalResistance<N> = counted(Exact::int(1000000000000000000) * OHM::<Exact>);

    /// One `Petaohm` (PΩ), as an `ElectricalResistance`.
    pub const PETAOHM<N: const From<Exact>>: ElectricalResistance<N> = counted(Exact::int(1000000000000000) * OHM::<Exact>);

    /// One `Teraohm` (TΩ), as an `ElectricalResistance`.
    pub const TERAOHM<N: const From<Exact>>: ElectricalResistance<N> = counted(Exact::int(1000000000000) * OHM::<Exact>);

    /// One `Gigaohm` (GΩ), as an `ElectricalResistance`.
    pub const GIGAOHM<N: const From<Exact>>: ElectricalResistance<N> = counted(Exact::int(1000000000) * OHM::<Exact>);

    /// One `Megaohm` (MΩ), as an `ElectricalResistance`.
    pub const MEGAOHM<N: const From<Exact>>: ElectricalResistance<N> = counted(Exact::int(1000000) * OHM::<Exact>);

    /// One `Kiloohm` (kΩ), as an `ElectricalResistance`.
    pub const KILOOHM<N: const From<Exact>>: ElectricalResistance<N> = counted(Exact::int(1000) * OHM::<Exact>);

    /// One `Hectoohm` (hΩ), as an `ElectricalResistance`.
    pub const HECTOOHM<N: const From<Exact>>: ElectricalResistance<N> = counted(Exact::int(100) * OHM::<Exact>);

    /// One `Decaohm` (daΩ), as an `ElectricalResistance`.
    pub const DECAOHM<N: const From<Exact>>: ElectricalResistance<N> = counted(Exact::int(10) * OHM::<Exact>);

    /// One `Ohm` (Ω), as an `ElectricalResistance`.
    pub const OHM<N: const From<Exact>>: ElectricalResistance<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * AMPERE::<Exact> * AMPERE::<Exact>));

    /// One `Deciohm` (dΩ), as an `ElectricalResistance`.
    pub const DECIOHM<N: const From<Exact>>: ElectricalResistance<N> = counted(Exact::dec(1, -1) * OHM::<Exact>);

    /// One `Centiohm` (cΩ), as an `ElectricalResistance`.
    pub const CENTIOHM<N: const From<Exact>>: ElectricalResistance<N> = counted(Exact::dec(1, -2) * OHM::<Exact>);

    /// One `Milliohm` (mΩ), as an `ElectricalResistance`.
    pub const MILLIOHM<N: const From<Exact>>: ElectricalResistance<N> = counted(Exact::dec(1, -3) * OHM::<Exact>);

    /// One `Microohm` (µΩ), as an `ElectricalResistance`.
    pub const MICROOHM<N: const From<Exact>>: ElectricalResistance<N> = counted(Exact::dec(1, -6) * OHM::<Exact>);

    /// One `Nanoohm` (nΩ), as an `ElectricalResistance`.
    pub const NANOOHM<N: const From<Exact>>: ElectricalResistance<N> = counted(Exact::dec(1, -9) * OHM::<Exact>);

    /// One `Picoohm` (pΩ), as an `ElectricalResistance`.
    pub const PICOOHM<N: const From<Exact>>: ElectricalResistance<N> = counted(Exact::dec(1, -12) * OHM::<Exact>);

    /// One `Femtoohm` (fΩ), as an `ElectricalResistance`.
    pub const FEMTOOHM<N: const From<Exact>>: ElectricalResistance<N> = counted(Exact::dec(1, -15) * OHM::<Exact>);

    /// One `Attoohm` (aΩ), as an `ElectricalResistance`.
    pub const ATTOOHM<N: const From<Exact>>: ElectricalResistance<N> = counted(Exact::dec(1, -18) * OHM::<Exact>);

    /// One `Zeptoohm` (zΩ), as an `ElectricalResistance`.
    pub const ZEPTOOHM<N: const From<Exact>>: ElectricalResistance<N> = counted(Exact::dec(1, -21) * OHM::<Exact>);

    /// One `Yoctoohm` (yΩ), as an `ElectricalResistance`.
    pub const YOCTOOHM<N: const From<Exact>>: ElectricalResistance<N> = counted(Exact::dec(1, -24) * OHM::<Exact>);

    /// One `Abohm` (abΩ), as an `ElectricalResistance`.
    pub const ABOHM<N: const From<Exact>>: ElectricalResistance<N> = counted(Exact::dec(1, -9) * OHM::<Exact>);

    /// One `Statohm` (statΩ), as an `ElectricalResistance`.
    pub const STATOHM<N: const From<Exact>>: ElectricalResistance<N> = counted((Exact::dec(299792458, -6) * Exact::int(2997924580)) * OHM::<Exact>);

    /// One `YottaohmMetre` (YΩ · m), as an `ElectricalResistivity`.
    pub const YOTTAOHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::int(1000000000000000000000000) * OHM_METRE::<Exact>);

    /// One `ZettaohmMetre` (ZΩ · m), as an `ElectricalResistivity`.
    pub const ZETTAOHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::int(1000000000000000000000) * OHM_METRE::<Exact>);

    /// One `ExaohmMetre` (EΩ · m), as an `ElectricalResistivity`.
    pub const EXAOHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::int(1000000000000000000) * OHM_METRE::<Exact>);

    /// One `PetaohmMetre` (PΩ · m), as an `ElectricalResistivity`.
    pub const PETAOHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::int(1000000000000000) * OHM_METRE::<Exact>);

    /// One `TeraohmMetre` (TΩ · m), as an `ElectricalResistivity`.
    pub const TERAOHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::int(1000000000000) * OHM_METRE::<Exact>);

    /// One `GigaohmMetre` (GΩ · m), as an `ElectricalResistivity`.
    pub const GIGAOHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::int(1000000000) * OHM_METRE::<Exact>);

    /// One `MegaohmMetre` (MΩ · m), as an `ElectricalResistivity`.
    pub const MEGAOHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::int(1000000) * OHM_METRE::<Exact>);

    /// One `KiloohmMetre` (kΩ · m), as an `ElectricalResistivity`.
    pub const KILOOHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::int(1000) * OHM_METRE::<Exact>);

    /// One `HectoohmMetre` (hΩ · m), as an `ElectricalResistivity`.
    pub const HECTOOHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::int(100) * OHM_METRE::<Exact>);

    /// One `DecaohmMetre` (daΩ · m), as an `ElectricalResistivity`.
    pub const DECAOHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::int(10) * OHM_METRE::<Exact>);

    /// One `OhmMetre` (Ω · m), as an `ElectricalResistivity`.
    pub const OHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(METRE::<Exact> * METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * AMPERE::<Exact> * AMPERE::<Exact>));

    /// One `DeciohmMetre` (dΩ · m), as an `ElectricalResistivity`.
    pub const DECIOHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::dec(1, -1) * OHM_METRE::<Exact>);

    /// One `CentiohmMetre` (cΩ · m), as an `ElectricalResistivity`.
    pub const CENTIOHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::dec(1, -2) * OHM_METRE::<Exact>);

    /// One `MilliohmMetre` (mΩ · m), as an `ElectricalResistivity`.
    pub const MILLIOHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::dec(1, -3) * OHM_METRE::<Exact>);

    /// One `MicroohmMetre` (µΩ · m), as an `ElectricalResistivity`.
    pub const MICROOHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::dec(1, -6) * OHM_METRE::<Exact>);

    /// One `NanoohmMetre` (nΩ · m), as an `ElectricalResistivity`.
    pub const NANOOHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::dec(1, -9) * OHM_METRE::<Exact>);

    /// One `PicoohmMetre` (pΩ · m), as an `ElectricalResistivity`.
    pub const PICOOHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::dec(1, -12) * OHM_METRE::<Exact>);

    /// One `FemtoohmMetre` (fΩ · m), as an `ElectricalResistivity`.
    pub const FEMTOOHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::dec(1, -15) * OHM_METRE::<Exact>);

    /// One `AttoohmMetre` (aΩ · m), as an `ElectricalResistivity`.
    pub const ATTOOHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::dec(1, -18) * OHM_METRE::<Exact>);

    /// One `ZeptoohmMetre` (zΩ · m), as an `ElectricalResistivity`.
    pub const ZEPTOOHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::dec(1, -21) * OHM_METRE::<Exact>);

    /// One `YoctoohmMetre` (yΩ · m), as an `ElectricalResistivity`.
    pub const YOCTOOHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::dec(1, -24) * OHM_METRE::<Exact>);

    /// One `AbohmMetre` (abΩ · m), as an `ElectricalResistivity`.
    pub const ABOHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::dec(1, -9) * OHM_METRE::<Exact>);

    /// One `StatohmMetre` (statΩ · m), as an `ElectricalResistivity`.
    pub const STATOHM_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted((Exact::dec(299792458, -6) * Exact::int(2997924580)) * OHM_METRE::<Exact>);

    /// One `OhmCentimetre` (Ω · cm), as an `ElectricalResistivity`.
    pub const OHM_CENTIMETRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::dec(1, -2) * OHM_METRE::<Exact>);

    /// One `AbohmCentimetre` (abΩ · cm), as an `ElectricalResistivity`.
    pub const ABOHM_CENTIMETRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::dec(1, -11) * OHM_METRE::<Exact>);

    /// One `StatohmCentimetre` (statΩ · cm), as an `ElectricalResistivity`.
    pub const STATOHM_CENTIMETRE<N: const From<Exact>>: ElectricalResistivity<N> = counted((Exact::dec(299792458, -6) * Exact::int(2997924580) / Exact::int(100)) * OHM_METRE::<Exact>);

    /// One `OhmInch` (Ω · in), as an `ElectricalResistivity`.
    pub const OHM_INCH<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::dec(254, -4) * OHM_METRE::<Exact>);

    /// One `OhmFoot` (Ω · ft), as an `ElectricalResistivity`.
    pub const OHM_FOOT<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::dec(3048, -4) * OHM_METRE::<Exact>);

    /// One `OhmYard` (Ω · yd), as an `ElectricalResistivity`.
    pub const OHM_YARD<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::dec(9144, -4) * OHM_METRE::<Exact>);

    /// One `OhmSquareMillimetrePerMetre` (Ω · mm²/m), as an `ElectricalResistivity`.
    pub const OHM_SQUARE_MILLIMETRE_PER_METRE<N: const From<Exact>>: ElectricalResistivity<N> = counted(Exact::dec(1, -6) * OHM_METRE::<Exact>);

    /// One `Yottafarad` (YF), as a `Capacitance`.
    pub const YOTTAFARAD<N: const From<Exact>>: Capacitance<N> = counted(Exact::int(1000000000000000000000000) * FARAD::<Exact>);

    /// One `Zettafarad` (ZF), as a `Capacitance`.
    pub const ZETTAFARAD<N: const From<Exact>>: Capacitance<N> = counted(Exact::int(1000000000000000000000) * FARAD::<Exact>);

    /// One `Exafarad` (EF), as a `Capacitance`.
    pub const EXAFARAD<N: const From<Exact>>: Capacitance<N> = counted(Exact::int(1000000000000000000) * FARAD::<Exact>);

    /// One `Petafarad` (PF), as a `Capacitance`.
    pub const PETAFARAD<N: const From<Exact>>: Capacitance<N> = counted(Exact::int(1000000000000000) * FARAD::<Exact>);

    /// One `Terafarad` (TF), as a `Capacitance`.
    pub const TERAFARAD<N: const From<Exact>>: Capacitance<N> = counted(Exact::int(1000000000000) * FARAD::<Exact>);

    /// One `Gigafarad` (GF), as a `Capacitance`.
    pub const GIGAFARAD<N: const From<Exact>>: Capacitance<N> = counted(Exact::int(1000000000) * FARAD::<Exact>);

    /// One `Megafarad` (MF), as a `Capacitance`.
    pub const MEGAFARAD<N: const From<Exact>>: Capacitance<N> = counted(Exact::int(1000000) * FARAD::<Exact>);

    /// One `Kilofarad` (kF), as a `Capacitance`.
    pub const KILOFARAD<N: const From<Exact>>: Capacitance<N> = counted(Exact::int(1000) * FARAD::<Exact>);

    /// One `Hectofarad` (hF), as a `Capacitance`.
    pub const HECTOFARAD<N: const From<Exact>>: Capacitance<N> = counted(Exact::int(100) * FARAD::<Exact>);

    /// One `Decafarad` (daF), as a `Capacitance`.
    pub const DECAFARAD<N: const From<Exact>>: Capacitance<N> = counted(Exact::int(10) * FARAD::<Exact>);

    /// One `Farad` (F), as a `Capacitance`.
    pub const FARAD<N: const From<Exact>>: Capacitance<N> = counted(SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * AMPERE::<Exact> * AMPERE::<Exact> / (METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact>));

    /// One `Decifarad` (dF), as a `Capacitance`.
    pub const DECIFARAD<N: const From<Exact>>: Capacitance<N> = counted(Exact::dec(1, -1) * FARAD::<Exact>);

    /// One `Centifarad` (cF), as a `Capacitance`.
    pub const CENTIFARAD<N: const From<Exact>>: Capacitance<N> = counted(Exact::dec(1, -2) * FARAD::<Exact>);

    /// One `Millifarad` (mF), as a `Capacitance`.
    pub const MILLIFARAD<N: const From<Exact>>: Capacitance<N> = counted(Exact::dec(1, -3) * FARAD::<Exact>);

    /// One `Microfarad` (µF), as a `Capacitance`.
    pub const MICROFARAD<N: const From<Exact>>: Capacitance<N> = counted(Exact::dec(1, -6) * FARAD::<Exact>);

    /// One `Nanofarad` (nF), as a `Capacitance`.
    pub const NANOFARAD<N: const From<Exact>>: Capacitance<N> = counted(Exact::dec(1, -9) * FARAD::<Exact>);

    /// One `Picofarad` (pF), as a `Capacitance`.
    pub const PICOFARAD<N: const From<Exact>>: Capacitance<N> = counted(Exact::dec(1, -12) * FARAD::<Exact>);

    /// One `Femtofarad` (fF), as a `Capacitance`.
    pub const FEMTOFARAD<N: const From<Exact>>: Capacitance<N> = counted(Exact::dec(1, -15) * FARAD::<Exact>);

    /// One `Attofarad` (aF), as a `Capacitance`.
    pub const ATTOFARAD<N: const From<Exact>>: Capacitance<N> = counted(Exact::dec(1, -18) * FARAD::<Exact>);

    /// One `Zeptofarad` (zF), as a `Capacitance`.
    pub const ZEPTOFARAD<N: const From<Exact>>: Capacitance<N> = counted(Exact::dec(1, -21) * FARAD::<Exact>);

    /// One `Yoctofarad` (yF), as a `Capacitance`.
    pub const YOCTOFARAD<N: const From<Exact>>: Capacitance<N> = counted(Exact::dec(1, -24) * FARAD::<Exact>);

    /// One `Abfarad` (abF), as a `Capacitance`.
    pub const ABFARAD<N: const From<Exact>>: Capacitance<N> = counted(Exact::int(1000000000) * FARAD::<Exact>);

    /// One `Statfarad` (statF), as a `Capacitance`.
    pub const STATFARAD<N: const From<Exact>>: Capacitance<N> = counted((Exact::int(1) / (Exact::dec(299792458, -6) * Exact::int(2997924580))) * FARAD::<Exact>);

    /// One `Yottahenry` (YH), as an `Inductance`.
    pub const YOTTAHENRY<N: const From<Exact>>: Inductance<N> = counted(Exact::int(1000000000000000000000000) * HENRY::<Exact>);

    /// One `Zettahenry` (ZH), as an `Inductance`.
    pub const ZETTAHENRY<N: const From<Exact>>: Inductance<N> = counted(Exact::int(1000000000000000000000) * HENRY::<Exact>);

    /// One `Exahenry` (EH), as an `Inductance`.
    pub const EXAHENRY<N: const From<Exact>>: Inductance<N> = counted(Exact::int(1000000000000000000) * HENRY::<Exact>);

    /// One `Petahenry` (PH), as an `Inductance`.
    pub const PETAHENRY<N: const From<Exact>>: Inductance<N> = counted(Exact::int(1000000000000000) * HENRY::<Exact>);

    /// One `Terahenry` (TH), as an `Inductance`.
    pub const TERAHENRY<N: const From<Exact>>: Inductance<N> = counted(Exact::int(1000000000000) * HENRY::<Exact>);

    /// One `Gigahenry` (GH), as an `Inductance`.
    pub const GIGAHENRY<N: const From<Exact>>: Inductance<N> = counted(Exact::int(1000000000) * HENRY::<Exact>);

    /// One `Megahenry` (MH), as an `Inductance`.
    pub const MEGAHENRY<N: const From<Exact>>: Inductance<N> = counted(Exact::int(1000000) * HENRY::<Exact>);

    /// One `Kilohenry` (kH), as an `Inductance`.
    pub const KILOHENRY<N: const From<Exact>>: Inductance<N> = counted(Exact::int(1000) * HENRY::<Exact>);

    /// One `Hectohenry` (hH), as an `Inductance`.
    pub const HECTOHENRY<N: const From<Exact>>: Inductance<N> = counted(Exact::int(100) * HENRY::<Exact>);

    /// One `Decahenry` (daH), as an `Inductance`.
    pub const DECAHENRY<N: const From<Exact>>: Inductance<N> = counted(Exact::int(10) * HENRY::<Exact>);

    /// One `Henry` (H), as an `Inductance`.
    pub const HENRY<N: const From<Exact>>: Inductance<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * AMPERE::<Exact> * AMPERE::<Exact>));

    /// One `Decihenry` (dH), as an `Inductance`.
    pub const DECIHENRY<N: const From<Exact>>: Inductance<N> = counted(Exact::dec(1, -1) * HENRY::<Exact>);

    /// One `Centihenry` (cH), as an `Inductance`.
    pub const CENTIHENRY<N: const From<Exact>>: Inductance<N> = counted(Exact::dec(1, -2) * HENRY::<Exact>);

    /// One `Millihenry` (mH), as an `Inductance`.
    pub const MILLIHENRY<N: const From<Exact>>: Inductance<N> = counted(Exact::dec(1, -3) * HENRY::<Exact>);

    /// One `Microhenry` (µH), as an `Inductance`.
    pub const MICROHENRY<N: const From<Exact>>: Inductance<N> = counted(Exact::dec(1, -6) * HENRY::<Exact>);

    /// One `Nanohenry` (nH), as an `Inductance`.
    pub const NANOHENRY<N: const From<Exact>>: Inductance<N> = counted(Exact::dec(1, -9) * HENRY::<Exact>);

    /// One `Picohenry` (pH), as an `Inductance`.
    pub const PICOHENRY<N: const From<Exact>>: Inductance<N> = counted(Exact::dec(1, -12) * HENRY::<Exact>);

    /// One `Femtohenry` (fH), as an `Inductance`.
    pub const FEMTOHENRY<N: const From<Exact>>: Inductance<N> = counted(Exact::dec(1, -15) * HENRY::<Exact>);

    /// One `Attohenry` (aH), as an `Inductance`.
    pub const ATTOHENRY<N: const From<Exact>>: Inductance<N> = counted(Exact::dec(1, -18) * HENRY::<Exact>);

    /// One `Zeptohenry` (zH), as an `Inductance`.
    pub const ZEPTOHENRY<N: const From<Exact>>: Inductance<N> = counted(Exact::dec(1, -21) * HENRY::<Exact>);

    /// One `Yoctohenry` (yH), as an `Inductance`.
    pub const YOCTOHENRY<N: const From<Exact>>: Inductance<N> = counted(Exact::dec(1, -24) * HENRY::<Exact>);

    /// One `Abhenry` (abH), as an `Inductance`.
    pub const ABHENRY<N: const From<Exact>>: Inductance<N> = counted(Exact::dec(1, -9) * HENRY::<Exact>);

    /// One `Stathenry` (statH), as an `Inductance`.
    pub const STATHENRY<N: const From<Exact>>: Inductance<N> = counted((Exact::dec(299792458, -6) * Exact::int(2997924580)) * HENRY::<Exact>);

    /// One `AmperePerMetre` (A/m), as a `MagneticFieldStrength`.
    pub const MAGNETIC_FIELD_STRENGTH_AMPERE_PER_METRE<N: const From<Exact>>: MagneticFieldStrength<N> = counted(AMPERE::<Exact> / METRE::<Exact>);

    /// One `AmperePerCentimetre` (A/cm), as a `MagneticFieldStrength`.
    pub const MAGNETIC_FIELD_STRENGTH_AMPERE_PER_CENTIMETRE<N: const From<Exact>>: MagneticFieldStrength<N> = counted(Exact::int(100) * MAGNETIC_FIELD_STRENGTH_AMPERE_PER_METRE::<Exact>);

    /// One `AmperePerMillimetre` (A/mm), as a `MagneticFieldStrength`.
    pub const MAGNETIC_FIELD_STRENGTH_AMPERE_PER_MILLIMETRE<N: const From<Exact>>: MagneticFieldStrength<N> = counted(Exact::int(1000) * MAGNETIC_FIELD_STRENGTH_AMPERE_PER_METRE::<Exact>);

    /// One `AmperePerMicrometre` (A/μm), as a `MagneticFieldStrength`.
    pub const MAGNETIC_FIELD_STRENGTH_AMPERE_PER_MICROMETRE<N: const From<Exact>>: MagneticFieldStrength<N> = counted(Exact::int(1000000) * MAGNETIC_FIELD_STRENGTH_AMPERE_PER_METRE::<Exact>);

    /// One `Oersted` (Oe), as a `MagneticFieldStrength`.
    pub const OERSTED<N: const From<Exact>>: MagneticFieldStrength<N> = counted((Exact::int(1000) / (Exact::int(4) * Exact::PI)) * MAGNETIC_FIELD_STRENGTH_AMPERE_PER_METRE::<Exact>);

    /// One `Yottaweber` (YWb), as a `MagneticFlux`.
    pub const YOTTAWEBER<N: const From<Exact>>: MagneticFlux<N> = counted(Exact::int(1000000000000000000000000) * WEBER::<Exact>);

    /// One `Zettaweber` (ZWb), as a `MagneticFlux`.
    pub const ZETTAWEBER<N: const From<Exact>>: MagneticFlux<N> = counted(Exact::int(1000000000000000000000) * WEBER::<Exact>);

    /// One `Exaweber` (EWb), as a `MagneticFlux`.
    pub const EXAWEBER<N: const From<Exact>>: MagneticFlux<N> = counted(Exact::int(1000000000000000000) * WEBER::<Exact>);

    /// One `Petaweber` (PWb), as a `MagneticFlux`.
    pub const PETAWEBER<N: const From<Exact>>: MagneticFlux<N> = counted(Exact::int(1000000000000000) * WEBER::<Exact>);

    /// One `Teraweber` (TWb), as a `MagneticFlux`.
    pub const TERAWEBER<N: const From<Exact>>: MagneticFlux<N> = counted(Exact::int(1000000000000) * WEBER::<Exact>);

    /// One `Gigaweber` (GWb), as a `MagneticFlux`.
    pub const GIGAWEBER<N: const From<Exact>>: MagneticFlux<N> = counted(Exact::int(1000000000) * WEBER::<Exact>);

    /// One `Megaweber` (MWb), as a `MagneticFlux`.
    pub const MEGAWEBER<N: const From<Exact>>: MagneticFlux<N> = counted(Exact::int(1000000) * WEBER::<Exact>);

    /// One `Kiloweber` (kWb), as a `MagneticFlux`.
    pub const KILOWEBER<N: const From<Exact>>: MagneticFlux<N> = counted(Exact::int(1000) * WEBER::<Exact>);

    /// One `Hectoweber` (hWb), as a `MagneticFlux`.
    pub const HECTOWEBER<N: const From<Exact>>: MagneticFlux<N> = counted(Exact::int(100) * WEBER::<Exact>);

    /// One `Decaweber` (daWb), as a `MagneticFlux`.
    pub const DECAWEBER<N: const From<Exact>>: MagneticFlux<N> = counted(Exact::int(10) * WEBER::<Exact>);

    /// One `Weber` (Wb), as a `MagneticFlux`.
    pub const WEBER<N: const From<Exact>>: MagneticFlux<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * AMPERE::<Exact>));

    /// One `Deciweber` (dWb), as a `MagneticFlux`.
    pub const DECIWEBER<N: const From<Exact>>: MagneticFlux<N> = counted(Exact::dec(1, -1) * WEBER::<Exact>);

    /// One `Centiweber` (cWb), as a `MagneticFlux`.
    pub const CENTIWEBER<N: const From<Exact>>: MagneticFlux<N> = counted(Exact::dec(1, -2) * WEBER::<Exact>);

    /// One `Milliweber` (mWb), as a `MagneticFlux`.
    pub const MILLIWEBER<N: const From<Exact>>: MagneticFlux<N> = counted(Exact::dec(1, -3) * WEBER::<Exact>);

    /// One `Microweber` (µWb), as a `MagneticFlux`.
    pub const MICROWEBER<N: const From<Exact>>: MagneticFlux<N> = counted(Exact::dec(1, -6) * WEBER::<Exact>);

    /// One `Nanoweber` (nWb), as a `MagneticFlux`.
    pub const NANOWEBER<N: const From<Exact>>: MagneticFlux<N> = counted(Exact::dec(1, -9) * WEBER::<Exact>);

    /// One `Picoweber` (pWb), as a `MagneticFlux`.
    pub const PICOWEBER<N: const From<Exact>>: MagneticFlux<N> = counted(Exact::dec(1, -12) * WEBER::<Exact>);

    /// One `Femtoweber` (fWb), as a `MagneticFlux`.
    pub const FEMTOWEBER<N: const From<Exact>>: MagneticFlux<N> = counted(Exact::dec(1, -15) * WEBER::<Exact>);

    /// One `Attoweber` (aWb), as a `MagneticFlux`.
    pub const ATTOWEBER<N: const From<Exact>>: MagneticFlux<N> = counted(Exact::dec(1, -18) * WEBER::<Exact>);

    /// One `Zeptoweber` (zWb), as a `MagneticFlux`.
    pub const ZEPTOWEBER<N: const From<Exact>>: MagneticFlux<N> = counted(Exact::dec(1, -21) * WEBER::<Exact>);

    /// One `Yoctoweber` (yWb), as a `MagneticFlux`.
    pub const YOCTOWEBER<N: const From<Exact>>: MagneticFlux<N> = counted(Exact::dec(1, -24) * WEBER::<Exact>);

    /// One `Maxwell` (Mx), as a `MagneticFlux`.
    pub const MAXWELL<N: const From<Exact>>: MagneticFlux<N> = counted(Exact::dec(1, -8) * WEBER::<Exact>);

    /// One `Yottatesla` (YT), as a `MagneticFluxDensity`.
    pub const YOTTATESLA<N: const From<Exact>>: MagneticFluxDensity<N> = counted(Exact::int(1000000000000000000000000) * TESLA::<Exact>);

    /// One `Zettatesla` (ZT), as a `MagneticFluxDensity`.
    pub const ZETTATESLA<N: const From<Exact>>: MagneticFluxDensity<N> = counted(Exact::int(1000000000000000000000) * TESLA::<Exact>);

    /// One `Exatesla` (ET), as a `MagneticFluxDensity`.
    pub const EXATESLA<N: const From<Exact>>: MagneticFluxDensity<N> = counted(Exact::int(1000000000000000000) * TESLA::<Exact>);

    /// One `Petatesla` (PT), as a `MagneticFluxDensity`.
    pub const PETATESLA<N: const From<Exact>>: MagneticFluxDensity<N> = counted(Exact::int(1000000000000000) * TESLA::<Exact>);

    /// One `Teratesla` (TT), as a `MagneticFluxDensity`.
    pub const TERATESLA<N: const From<Exact>>: MagneticFluxDensity<N> = counted(Exact::int(1000000000000) * TESLA::<Exact>);

    /// One `Gigatesla` (GT), as a `MagneticFluxDensity`.
    pub const GIGATESLA<N: const From<Exact>>: MagneticFluxDensity<N> = counted(Exact::int(1000000000) * TESLA::<Exact>);

    /// One `Megatesla` (MT), as a `MagneticFluxDensity`.
    pub const MEGATESLA<N: const From<Exact>>: MagneticFluxDensity<N> = counted(Exact::int(1000000) * TESLA::<Exact>);

    /// One `Kilotesla` (kT), as a `MagneticFluxDensity`.
    pub const KILOTESLA<N: const From<Exact>>: MagneticFluxDensity<N> = counted(Exact::int(1000) * TESLA::<Exact>);

    /// One `Hectotesla` (hT), as a `MagneticFluxDensity`.
    pub const HECTOTESLA<N: const From<Exact>>: MagneticFluxDensity<N> = counted(Exact::int(100) * TESLA::<Exact>);

    /// One `Decatesla` (daT), as a `MagneticFluxDensity`.
    pub const DECATESLA<N: const From<Exact>>: MagneticFluxDensity<N> = counted(Exact::int(10) * TESLA::<Exact>);

    /// One `Tesla` (T), as a `MagneticFluxDensity`.
    pub const TESLA<N: const From<Exact>>: MagneticFluxDensity<N> = counted(KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * AMPERE::<Exact>));

    /// One `Decitesla` (dT), as a `MagneticFluxDensity`.
    pub const DECITESLA<N: const From<Exact>>: MagneticFluxDensity<N> = counted(Exact::dec(1, -1) * TESLA::<Exact>);

    /// One `Centitesla` (cT), as a `MagneticFluxDensity`.
    pub const CENTITESLA<N: const From<Exact>>: MagneticFluxDensity<N> = counted(Exact::dec(1, -2) * TESLA::<Exact>);

    /// One `Millitesla` (mT), as a `MagneticFluxDensity`.
    pub const MILLITESLA<N: const From<Exact>>: MagneticFluxDensity<N> = counted(Exact::dec(1, -3) * TESLA::<Exact>);

    /// One `Microtesla` (µT), as a `MagneticFluxDensity`.
    pub const MICROTESLA<N: const From<Exact>>: MagneticFluxDensity<N> = counted(Exact::dec(1, -6) * TESLA::<Exact>);

    /// One `Nanotesla` (nT), as a `MagneticFluxDensity`.
    pub const NANOTESLA<N: const From<Exact>>: MagneticFluxDensity<N> = counted(Exact::dec(1, -9) * TESLA::<Exact>);

    /// One `Picotesla` (pT), as a `MagneticFluxDensity`.
    pub const PICOTESLA<N: const From<Exact>>: MagneticFluxDensity<N> = counted(Exact::dec(1, -12) * TESLA::<Exact>);

    /// One `Femtotesla` (fT), as a `MagneticFluxDensity`.
    pub const FEMTOTESLA<N: const From<Exact>>: MagneticFluxDensity<N> = counted(Exact::dec(1, -15) * TESLA::<Exact>);

    /// One `Attotesla` (aT), as a `MagneticFluxDensity`.
    pub const ATTOTESLA<N: const From<Exact>>: MagneticFluxDensity<N> = counted(Exact::dec(1, -18) * TESLA::<Exact>);

    /// One `Zeptotesla` (zT), as a `MagneticFluxDensity`.
    pub const ZEPTOTESLA<N: const From<Exact>>: MagneticFluxDensity<N> = counted(Exact::dec(1, -21) * TESLA::<Exact>);

    /// One `Yoctotesla` (yT), as a `MagneticFluxDensity`.
    pub const YOCTOTESLA<N: const From<Exact>>: MagneticFluxDensity<N> = counted(Exact::dec(1, -24) * TESLA::<Exact>);

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

    /// One `YottahertzPerTesla` (YHz/T), as a `GyromagneticRatio`.
    pub const YOTTAHERTZ_PER_TESLA<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::int(1000000000000000000000000) * HERTZ_PER_TESLA::<Exact>);

    /// One `ZettahertzPerTesla` (ZHz/T), as a `GyromagneticRatio`.
    pub const ZETTAHERTZ_PER_TESLA<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::int(1000000000000000000000) * HERTZ_PER_TESLA::<Exact>);

    /// One `ExahertzPerTesla` (EHz/T), as a `GyromagneticRatio`.
    pub const EXAHERTZ_PER_TESLA<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::int(1000000000000000000) * HERTZ_PER_TESLA::<Exact>);

    /// One `PetahertzPerTesla` (PHz/T), as a `GyromagneticRatio`.
    pub const PETAHERTZ_PER_TESLA<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::int(1000000000000000) * HERTZ_PER_TESLA::<Exact>);

    /// One `TerahertzPerTesla` (THz/T), as a `GyromagneticRatio`.
    pub const TERAHERTZ_PER_TESLA<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::int(1000000000000) * HERTZ_PER_TESLA::<Exact>);

    /// One `GigahertzPerTesla` (GHz/T), as a `GyromagneticRatio`.
    pub const GIGAHERTZ_PER_TESLA<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::int(1000000000) * HERTZ_PER_TESLA::<Exact>);

    /// One `MegahertzPerTesla` (MHz/T), as a `GyromagneticRatio`.
    pub const MEGAHERTZ_PER_TESLA<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::int(1000000) * HERTZ_PER_TESLA::<Exact>);

    /// One `KilohertzPerTesla` (kHz/T), as a `GyromagneticRatio`.
    pub const KILOHERTZ_PER_TESLA<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::int(1000) * HERTZ_PER_TESLA::<Exact>);

    /// One `HectohertzPerTesla` (hHz/T), as a `GyromagneticRatio`.
    pub const HECTOHERTZ_PER_TESLA<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::int(100) * HERTZ_PER_TESLA::<Exact>);

    /// One `DecahertzPerTesla` (daHz/T), as a `GyromagneticRatio`.
    pub const DECAHERTZ_PER_TESLA<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::int(10) * HERTZ_PER_TESLA::<Exact>);

    /// One `HertzPerTesla` (Hz/T), as a `GyromagneticRatio`.
    pub const HERTZ_PER_TESLA<N: const From<Exact>>: GyromagneticRatio<N> = counted(SECOND::<Exact> * AMPERE::<Exact> * RADIAN::<Exact> / KILOGRAM::<Exact>);

    /// One `DecihertzPerTesla` (dHz/T), as a `GyromagneticRatio`.
    pub const DECIHERTZ_PER_TESLA<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::dec(1, -1) * HERTZ_PER_TESLA::<Exact>);

    /// One `CentihertzPerTesla` (cHz/T), as a `GyromagneticRatio`.
    pub const CENTIHERTZ_PER_TESLA<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::dec(1, -2) * HERTZ_PER_TESLA::<Exact>);

    /// One `MillihertzPerTesla` (mHz/T), as a `GyromagneticRatio`.
    pub const MILLIHERTZ_PER_TESLA<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::dec(1, -3) * HERTZ_PER_TESLA::<Exact>);

    /// One `MicrohertzPerTesla` (µHz/T), as a `GyromagneticRatio`.
    pub const MICROHERTZ_PER_TESLA<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::dec(1, -6) * HERTZ_PER_TESLA::<Exact>);

    /// One `NanohertzPerTesla` (nHz/T), as a `GyromagneticRatio`.
    pub const NANOHERTZ_PER_TESLA<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::dec(1, -9) * HERTZ_PER_TESLA::<Exact>);

    /// One `PicohertzPerTesla` (pHz/T), as a `GyromagneticRatio`.
    pub const PICOHERTZ_PER_TESLA<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::dec(1, -12) * HERTZ_PER_TESLA::<Exact>);

    /// One `FemtohertzPerTesla` (fHz/T), as a `GyromagneticRatio`.
    pub const FEMTOHERTZ_PER_TESLA<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::dec(1, -15) * HERTZ_PER_TESLA::<Exact>);

    /// One `AttohertzPerTesla` (aHz/T), as a `GyromagneticRatio`.
    pub const ATTOHERTZ_PER_TESLA<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::dec(1, -18) * HERTZ_PER_TESLA::<Exact>);

    /// One `ZeptohertzPerTesla` (zHz/T), as a `GyromagneticRatio`.
    pub const ZEPTOHERTZ_PER_TESLA<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::dec(1, -21) * HERTZ_PER_TESLA::<Exact>);

    /// One `YoctohertzPerTesla` (yHz/T), as a `GyromagneticRatio`.
    pub const YOCTOHERTZ_PER_TESLA<N: const From<Exact>>: GyromagneticRatio<N> = counted(Exact::dec(1, -24) * HERTZ_PER_TESLA::<Exact>);

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
