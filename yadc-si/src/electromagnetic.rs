use crate::{Exact, ISQ, SIQuantity};
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
/// (re-exported), then one per named unit, each defined exactly from them
/// (`INCH` is `0.0254 * METRE`). A unit written as others is not one:
/// `SLUG / CUBIC_FOOT`, `KILO * METRE` are exact too. Converting rounds,
/// once, where it is written - in a `const`, by the compiler:
/// `const MPH: Velocity<f64> = (MILE / HOUR).into();`, then `MPH.quantity(60.0)`.
pub mod consts {
    use super::*;
    use crate::base::{KILOGRAM, METRE, RADIAN, SECOND};

    /// The ampere, the SI base unit of electric current.
    pub use crate::base::AMPERE;

    /// One `Coulomb` (`C`), as an `ElectricCharge`.
    pub const COULOMB: ElectricCharge<Exact> = SECOND * AMPERE;

    /// One `ElementaryCharge` (`e`), as an `ElectricCharge`.
    pub const ELEMENTARY_CHARGE: ElectricCharge<Exact> = COULOMB * Exact::dec(1602176634, -28);

    /// One `AtomicUnitOfCharge` (`a.u. of charge`), as an `ElectricCharge`.
    pub const ATOMIC_UNIT_OF_CHARGE: ElectricCharge<Exact> = COULOMB * Exact::dec(1602176634, -28);

    /// One `Abcoulomb` (`abC`), as an `ElectricCharge`.
    pub const ABCOULOMB: ElectricCharge<Exact> = COULOMB * Exact::int(10);

    /// One `Faraday` (`F`), as an `ElectricCharge`.
    pub const FARADAY: ElectricCharge<Exact> =
        COULOMB * (Exact::dec(1602176634, -28) * Exact::int(602214076000000000000000));

    /// One `Franklin` (`Fr`), as an `ElectricCharge`.
    pub const FRANKLIN: ElectricCharge<Exact> = COULOMB * (Exact::int(1) / Exact::int(2997924580));

    /// One `Statcoulomb` (`statC`), as an `ElectricCharge`.
    pub const STATCOULOMB: ElectricCharge<Exact> =
        COULOMB * (Exact::int(1) / Exact::int(2997924580));

    /// One `Abampere` (`abA`), as an `ElectricCurrent`.
    pub const ABAMPERE: ElectricCurrent<Exact> = AMPERE * Exact::int(10);

    /// One `Gilbert` (`Gi`), as an `ElectricCurrent`.
    pub const GILBERT: ElectricCurrent<Exact> =
        AMPERE * (Exact::int(10) / (Exact::int(4) * Exact::PI));

    /// One `Statampere` (`statA`), as an `ElectricCurrent`.
    pub const STATAMPERE: ElectricCurrent<Exact> =
        AMPERE * (Exact::int(1) / Exact::int(2997924580));

    /// One `Volt` (`V`), as an `ElectricPotential`.
    pub const VOLT: ElectricPotential<Exact> =
        METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * AMPERE);

    /// One `Abvolt` (`abV`), as an `ElectricPotential`.
    pub const ABVOLT: ElectricPotential<Exact> = VOLT * Exact::dec(1, -8);

    /// One `Statvolt` (`statV`), as an `ElectricPotential`.
    pub const STATVOLT: ElectricPotential<Exact> = VOLT * Exact::dec(299792458, -6);

    /// One `AtomicUnitOfElectricField` (`a.u. of electric field`), as an `ElectricField`.
    pub const ATOMIC_UNIT_OF_ELECTRIC_FIELD: ElectricField<Exact> = (METRE * KILOGRAM
        / (SECOND * SECOND * SECOND * AMPERE))
        * (Exact::dec(43597447222071, -31)
            / (Exact::dec(1602176634, -28) * Exact::dec(529177210903, -22)));

    /// One `VacuumPermittivity` (`ε₀`), as an `ElectricPermittivity`.
    pub const VACUUM_PERMITTIVITY: ElectricPermittivity<Exact> =
        (SECOND * SECOND * SECOND * SECOND * AMPERE * AMPERE / (METRE * METRE * METRE * KILOGRAM))
            * Exact::dec(88541878128, -22);

    /// One `Siemens` (`S`), as an `ElectricalConductance`.
    pub const SIEMENS: ElectricalConductance<Exact> =
        SECOND * SECOND * SECOND * AMPERE * AMPERE / (METRE * METRE * KILOGRAM);

    /// One `Mho` (`℧`), as an `ElectricalConductance`.
    pub const MHO: ElectricalConductance<Exact> = SIEMENS;

    /// One `Abmho` (`abmho`), as an `ElectricalConductance`.
    pub const ABMHO: ElectricalConductance<Exact> = SIEMENS * Exact::int(1000000000);

    /// One `Absiemens` (`abS`), as an `ElectricalConductance`.
    pub const ABSIEMENS: ElectricalConductance<Exact> = SIEMENS * Exact::int(1000000000);

    /// One `Statsiemens` (`statS`), as an `ElectricalConductance`.
    pub const STATSIEMENS: ElectricalConductance<Exact> =
        SIEMENS * (Exact::int(1) / (Exact::dec(299792458, -6) * Exact::int(2997924580)));

    /// One `Statmho` (`statmho`), as an `ElectricalConductance`.
    pub const STATMHO: ElectricalConductance<Exact> =
        SIEMENS * (Exact::int(1) / (Exact::dec(299792458, -6) * Exact::int(2997924580)));

    /// One `Ohm` (`Ω`), as an `ElectricalResistance`.
    pub const OHM: ElectricalResistance<Exact> =
        METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * AMPERE * AMPERE);

    /// One `Abohm` (`abΩ`), as an `ElectricalResistance`.
    pub const ABOHM: ElectricalResistance<Exact> = OHM * Exact::dec(1, -9);

    /// One `Statohm` (`statΩ`), as an `ElectricalResistance`.
    pub const STATOHM: ElectricalResistance<Exact> =
        OHM * (Exact::dec(299792458, -6) * Exact::int(2997924580));

    /// One `Farad` (`F`), as a `Capacitance`.
    pub const FARAD: Capacitance<Exact> =
        SECOND * SECOND * SECOND * SECOND * AMPERE * AMPERE / (METRE * METRE * KILOGRAM);

    /// One `Abfarad` (`abF`), as a `Capacitance`.
    pub const ABFARAD: Capacitance<Exact> = FARAD * Exact::int(1000000000);

    /// One `Statfarad` (`statF`), as a `Capacitance`.
    pub const STATFARAD: Capacitance<Exact> =
        FARAD * (Exact::int(1) / (Exact::dec(299792458, -6) * Exact::int(2997924580)));

    /// One `Henry` (`H`), as an `Inductance`.
    pub const HENRY: Inductance<Exact> =
        METRE * METRE * KILOGRAM / (SECOND * SECOND * AMPERE * AMPERE);

    /// One `Abhenry` (`abH`), as an `Inductance`.
    pub const ABHENRY: Inductance<Exact> = HENRY * Exact::dec(1, -9);

    /// One `Stathenry` (`statH`), as an `Inductance`.
    pub const STATHENRY: Inductance<Exact> =
        HENRY * (Exact::dec(299792458, -6) * Exact::int(2997924580));

    /// One `Oersted` (`Oe`), as a `MagneticFieldStrength`.
    pub const OERSTED: MagneticFieldStrength<Exact> =
        (AMPERE / METRE) * (Exact::int(1000) / (Exact::int(4) * Exact::PI));

    /// One `Weber` (`Wb`), as a `MagneticFlux`.
    pub const WEBER: MagneticFlux<Exact> = METRE * METRE * KILOGRAM / (SECOND * SECOND * AMPERE);

    /// One `Maxwell` (`Mx`), as a `MagneticFlux`.
    pub const MAXWELL: MagneticFlux<Exact> = WEBER * Exact::dec(1, -8);

    /// One `Tesla` (`T`), as a `MagneticFluxDensity`.
    pub const TESLA: MagneticFluxDensity<Exact> = KILOGRAM / (SECOND * SECOND * AMPERE);

    /// One `Gamma` (`γ`), as a `MagneticFluxDensity`.
    pub const GAMMA: MagneticFluxDensity<Exact> = TESLA * Exact::dec(1, -9);

    /// One `Gauss` (`G`), as a `MagneticFluxDensity`.
    pub const GAUSS: MagneticFluxDensity<Exact> = TESLA * Exact::dec(1, -4);

    /// One `BohrMagneton` (`µ(Bohr)`), as a `MagneticMoment`.
    pub const BOHR_MAGNETON: MagneticMoment<Exact> =
        (METRE * METRE * AMPERE) * Exact::dec(92740100783, -34);

    /// One `NuclearMagneton` (`μ(Nuclear)`), as a `MagneticMoment`.
    pub const NUCLEAR_MAGNETON: MagneticMoment<Exact> =
        (METRE * METRE * AMPERE) * Exact::dec(50507837461, -37);

    /// One `DeuteronMagneticMoment` (`μ(deuteron)`), as a `MagneticMoment`.
    pub const DEUTERON_MAGNETIC_MOMENT: MagneticMoment<Exact> =
        (METRE * METRE * AMPERE) * Exact::dec(4330735094, -36);

    /// One `ProtonMagneticMoment` (`μ(proton)`), as a `MagneticMoment`.
    pub const PROTON_MAGNETIC_MOMENT: MagneticMoment<Exact> =
        (METRE * METRE * AMPERE) * Exact::dec(141060679736, -37);

    /// One `ShieldedProtonMagneticMoment` (`μ’(proton)`), as a `MagneticMoment`.
    pub const SHIELDED_PROTON_MAGNETIC_MOMENT: MagneticMoment<Exact> =
        (METRE * METRE * AMPERE) * Exact::dec(141057056, -34);

    /// One `TritonMagneticMoment` (`μ(triton)`), as a `MagneticMoment`.
    pub const TRITON_MAGNETIC_MOMENT: MagneticMoment<Exact> =
        (METRE * METRE * AMPERE) * Exact::dec(15046095202, -36);

    /// One `VacuumPermeability` (`µ₀`), as a `MagneticPermeability`.
    pub const VACUUM_PERMEABILITY: MagneticPermeability<Exact> =
        (METRE * KILOGRAM / (SECOND * SECOND * AMPERE * AMPERE)) * Exact::dec(125663706212, -17);

    /// One `ProtonGyromagneticRatio` (`γₚ`), as a `GyromagneticRatio`.
    pub const PROTON_GYROMAGNETIC_RATIO: GyromagneticRatio<Exact> =
        (SECOND * AMPERE * RADIAN / KILOGRAM) * Exact::dec(26752218708, -2);

    /// One `NeutronGyromagneticRatio` (`γₙ`), as a `GyromagneticRatio`.
    pub const NEUTRON_GYROMAGNETIC_RATIO: GyromagneticRatio<Exact> =
        (SECOND * AMPERE * RADIAN / KILOGRAM) * Exact::int(183247174);

    /// One `ElectronGyromagneticRatio` (`γₑ`), as a `GyromagneticRatio`.
    pub const ELECTRON_GYROMAGNETIC_RATIO: GyromagneticRatio<Exact> =
        (SECOND * AMPERE * RADIAN / KILOGRAM) * Exact::int(176085962784);

    /// One `ShieldedProtonGyromagneticRatio` (`γ'ₚ`), as a `GyromagneticRatio`.
    pub const SHIELDED_PROTON_GYROMAGNETIC_RATIO: GyromagneticRatio<Exact> =
        (SECOND * AMPERE * RADIAN / KILOGRAM) * Exact::dec(2675153194, -1);

    /// One `ShieldedHelionGyromagneticRatio` (`γ'ₕ`), as a `GyromagneticRatio`.
    pub const SHIELDED_HELION_GYROMAGNETIC_RATIO: GyromagneticRatio<Exact> =
        (SECOND * AMPERE * RADIAN / KILOGRAM) * Exact::dec(20378946078, -2);

    /// One `DeuteronGyromagneticRatio` (`γ₂`), as a `GyromagneticRatio`.
    pub const DEUTERON_GYROMAGNETIC_RATIO: GyromagneticRatio<Exact> =
        (SECOND * AMPERE * RADIAN / KILOGRAM) * Exact::dec(410662888, -1);

    /// One `TritonGyromagneticRatio` (`γ₃`), as a `GyromagneticRatio`.
    pub const TRITON_GYROMAGNETIC_RATIO: GyromagneticRatio<Exact> =
        (SECOND * AMPERE * RADIAN / KILOGRAM) * Exact::int(285349844);

    /// One `Lithium7GyromagneticRatio` (`γ₇`), as a `GyromagneticRatio`.
    pub const LITHIUM7_GYROMAGNETIC_RATIO: GyromagneticRatio<Exact> =
        (SECOND * AMPERE * RADIAN / KILOGRAM) * Exact::int(103975264);

    /// One `Boron11GyromagneticRatio` (`γ₁₁`), as a `GyromagneticRatio`.
    pub const BORON11_GYROMAGNETIC_RATIO: GyromagneticRatio<Exact> =
        (SECOND * AMPERE * RADIAN / KILOGRAM) * Exact::dec(858384143, -1);

    /// One `Carbon13GyromagneticRatio` (`γ₁₃`), as a `GyromagneticRatio`.
    pub const CARBON13_GYROMAGNETIC_RATIO: GyromagneticRatio<Exact> =
        (SECOND * AMPERE * RADIAN / KILOGRAM) * Exact::dec(672787546, -1);

    /// One `Nitrogen14GyromagneticRatio` (`γ₁₄`), as a `GyromagneticRatio`.
    pub const NITROGEN14_GYROMAGNETIC_RATIO: GyromagneticRatio<Exact> =
        (SECOND * AMPERE * RADIAN / KILOGRAM) * Exact::dec(193287922, -1);

    /// One `Flourine19GyromagneticRatio` (`γ₁₉`), as a `GyromagneticRatio`.
    pub const FLOURINE19_GYROMAGNETIC_RATIO: GyromagneticRatio<Exact> =
        (SECOND * AMPERE * RADIAN / KILOGRAM) * Exact::int(251762483);

    /// One `Sodium23GyromagneticRatio` (`γ₂₃`), as a `GyromagneticRatio`.
    pub const SODIUM23_GYROMAGNETIC_RATIO: GyromagneticRatio<Exact> =
        (SECOND * AMPERE * RADIAN / KILOGRAM) * Exact::dec(708035417, -1);

    /// One `Phosphorus31GyromagneticRatio` (`γ₃₁`), as a `GyromagneticRatio`.
    pub const PHOSPHORUS31_GYROMAGNETIC_RATIO: GyromagneticRatio<Exact> =
        (SECOND * AMPERE * RADIAN / KILOGRAM) * Exact::int(108329419);

    /// One `ProtonReducedGyromagneticRatio` (`̅γₚ`), as a `ReducedGyromagneticRatio`.
    pub const PROTON_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio<Exact> =
        (SECOND * AMPERE / KILOGRAM) * Exact::dec(42577478461, -3);

    /// One `NeutronReducedGyromagneticRatio` (`̅γₙ`), as a `ReducedGyromagneticRatio`.
    pub const NEUTRON_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio<Exact> =
        (SECOND * AMPERE / KILOGRAM) * Exact::dec(291646935, -1);

    /// One `ElectronReducedGyromagneticRatio` (`̅γₑ`), as a `ReducedGyromagneticRatio`.
    pub const ELECTRON_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio<Exact> =
        (SECOND * AMPERE / KILOGRAM) * Exact::dec(280249513861, -1);

    /// One `ShieldedProtonReducedGyromagneticRatio` (`γ'ₚ`), as a `ReducedGyromagneticRatio`.
    pub const SHIELDED_PROTON_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio<Exact> =
        (SECOND * AMPERE / KILOGRAM) * Exact::dec(4257638543, -2);

    /// One `ShieldedHelionReducedGyromagneticRatio` (`γ'ₕ`), as a `ReducedGyromagneticRatio`.
    pub const SHIELDED_HELION_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio<Exact> =
        (SECOND * AMPERE / KILOGRAM) * Exact::dec(32434100033, -3);

    /// One `DeuteronReducedGyromagneticRatio` (`̅γ₂`), as a `ReducedGyromagneticRatio`.
    pub const DEUTERON_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio<Exact> =
        (SECOND * AMPERE / KILOGRAM) * Exact::dec(653590285, -2);

    /// One `TritonReducedGyromagneticRatio` (`̅γ₃`), as a `ReducedGyromagneticRatio`.
    pub const TRITON_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio<Exact> =
        (SECOND * AMPERE / KILOGRAM) * Exact::dec(454148381, -1);

    /// One `Lithium7ReducedGyromagneticRatio` (`̅γ₇`), as a `ReducedGyromagneticRatio`.
    pub const LITHIUM7_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio<Exact> =
        (SECOND * AMPERE / KILOGRAM) * Exact::dec(165481773, -1);

    /// One `Boron11ReducedGyromagneticRatio` (`̅γ₁₁`), as a `ReducedGyromagneticRatio`.
    pub const BORON11_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio<Exact> =
        (SECOND * AMPERE / KILOGRAM) * Exact::int(13661608);

    /// One `Carbon13ReducedGyromagneticRatio` (`̅γ₁₃`), as a `ReducedGyromagneticRatio`.
    pub const CARBON13_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio<Exact> =
        (SECOND * AMPERE / KILOGRAM) * Exact::dec(107077464, -1);

    /// One `Nitrogen14ReducedGyromagneticRatio` (`̅γ₁₄`), as a `ReducedGyromagneticRatio`.
    pub const NITROGEN14_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio<Exact> =
        (SECOND * AMPERE / KILOGRAM) * Exact::dec(307627281, -2);

    /// One `Flourine19ReducedGyromagneticRatio` (`̅γ₁₉`), as a `ReducedGyromagneticRatio`.
    pub const FLOURINE19_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio<Exact> =
        (SECOND * AMPERE / KILOGRAM) * Exact::dec(400692437, -1);

    /// One `Sodium23ReducedGyromagneticRatio` (`̅γ₂₃`), as a `ReducedGyromagneticRatio`.
    pub const SODIUM23_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio<Exact> =
        (SECOND * AMPERE / KILOGRAM) * Exact::dec(112687336, -1);

    /// One `Phosphorus31ReducedGyromagneticRatio` (`̅γ₃₁`), as a `ReducedGyromagneticRatio`.
    pub const PHOSPHORUS31_REDUCED_GYROMAGNETIC_RATIO: ReducedGyromagneticRatio<Exact> =
        (SECOND * AMPERE / KILOGRAM) * Exact::dec(172411625, -1);
}
