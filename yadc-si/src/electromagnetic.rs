use crate::base::{KILOGRAM, METRE, RADIAN, SECOND};
use crate::{Exact, ISQ, SIQuantity};
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
/// (re-exported), then one per named unit, each defined exactly from them
/// (`INCH` is `0.0254 * METRE`). A unit written as others is not one:
/// `SLUG / CUBIC_FOOT`, `KILO * METRE` are exact too. Converting rounds,
/// once, where it is written - in a `const`, by the compiler:
/// `const MPH: Velocity<f64> = (MILE / HOUR).into();`, then `MPH(60.0)`.
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

/// The units of `ElectricCharge`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCharge<N>, number = N)]
#[unit(for<N> where ElectricCharge<N>: [const] yadc::Unit<ElectricCharge<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
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

/// The units of `ElectricChargeArealDensity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricChargeArealDensity<N>, number = N)]
#[unit(for<N> where ElectricChargeArealDensity<N>: [const] yadc::Unit<ElectricChargeArealDensity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ElectricChargeArealDensityUnit {
    #[default]
    #[unit("C/m²", "coulombs per square metre", by = ElectricChargeArealDensity::<N>::from(SECOND * AMPERE / (METRE * METRE)))]
    CoulombPerSquareMetre,
}

/// The units of `ElectricChargeLinearDensity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricChargeLinearDensity<N>, number = N)]
#[unit(for<N> where ElectricChargeLinearDensity<N>: [const] yadc::Unit<ElectricChargeLinearDensity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ElectricChargeLinearDensityUnit {
    #[default]
    #[unit("C/m", "coulombs per metre", by = ElectricChargeLinearDensity::<N>::from(SECOND * AMPERE / METRE))]
    CoulombPerMetre,
}

/// The units of `ElectricChargeVolumetricDensity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricChargeVolumetricDensity<N>, number = N)]
#[unit(for<N> where ElectricChargeVolumetricDensity<N>: [const] yadc::Unit<ElectricChargeVolumetricDensity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ElectricChargeVolumetricDensityUnit {
    #[default]
    #[unit("C/m³", "coulombs per cubic metre", by = ElectricChargeVolumetricDensity::<N>::from(SECOND * AMPERE / (METRE * METRE * METRE)))]
    CoulombPerCubicMetre,
}

/// The units of `ElectricCurrent`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCurrent<N>, number = N)]
#[unit(for<N> where ElectricCurrent<N>: [const] yadc::Unit<ElectricCurrent<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
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

/// The units of `ElectricCurrentDensity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCurrentDensity<N>, number = N)]
#[unit(for<N> where ElectricCurrentDensity<N>: [const] yadc::Unit<ElectricCurrentDensity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ElectricCurrentDensityUnit {
    #[default]
    #[unit("A/m²", "amperes per square metre", by = ElectricCurrentDensity::<N>::from(AMPERE / (METRE * METRE)))]
    AmperePerSquareMetre,
}

/// The units of `SurfaceElectricCurrentDensity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SurfaceElectricCurrentDensity<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, SurfaceElectricCurrentDensity<N>: [const] yadc::Unit<SurfaceElectricCurrentDensity<N>, N> + [const] core::marker::Destruct)]
pub enum SurfaceElectricCurrentDensityUnit {
    #[default]
    #[unit("A/m", "amperes per metre", by = SurfaceElectricCurrentDensity::<N>::from(AMPERE / METRE))]
    AmperePerMetre,
}

/// The units of `ElectricPotential`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricPotential<N>, number = N)]
#[unit(for<N> where ElectricPotential<N>: [const] yadc::Unit<ElectricPotential<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ElectricPotentialUnit {
    #[default]
    #[unit("V", "volts", by = ElectricPotential::<N>::from(VOLT))]
    Volt,
    #[unit("abV", "abvolts", by = ElectricPotential::<N>::from(ABVOLT))]
    Abvolt,
    #[unit("statV", "statvolts", by = ElectricPotential::<N>::from(STATVOLT))]
    Statvolt,
}

/// The units of `ElectricField`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricField<N>, number = N)]
#[unit(for<N> where ElectricField<N>: [const] yadc::Unit<ElectricField<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ElectricFieldUnit {
    #[default]
    #[unit("V/m", "volts per metre", by = ElectricField::<N>::from(METRE * KILOGRAM / (SECOND * SECOND * SECOND * AMPERE)))]
    VoltPerMetre,
    #[unit("V/mil", "volts per mil", by = ElectricField::<N>::from((METRE * KILOGRAM / (SECOND * SECOND * SECOND * AMPERE)) * (Exact::int(1) / Exact::dec(254, -7))))]
    VoltPerMil,
    #[unit("a.u. of electric field", "atomic units of electric field", by = ElectricField::<N>::from(ATOMIC_UNIT_OF_ELECTRIC_FIELD))]
    AtomicUnitOfElectricField,
}

/// The units of `ElectricDisplacementField`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricDisplacementField<N>, number = N)]
#[unit(for<N> where ElectricDisplacementField<N>: [const] yadc::Unit<ElectricDisplacementField<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ElectricDisplacementFieldUnit {
    #[default]
    #[unit("C/m²", "coulombs per square metre", by = ElectricDisplacementField::<N>::from(SECOND * AMPERE / (METRE * METRE)))]
    CoulombPerSquareMetre,
}

/// The units of `ElectricFlux`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricFlux<N>, number = N)]
#[unit(for<N> where ElectricFlux<N>: [const] yadc::Unit<ElectricFlux<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ElectricFluxUnit {
    #[default]
    #[unit("V ⋅ m", "volt metres", by = ElectricFlux::<N>::from(METRE * METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * AMPERE)))]
    VoltMetre,
}

/// The units of `ElectricDipoleMoment`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricDipoleMoment<N>, number = N)]
#[unit(for<N> where ElectricDipoleMoment<N>: [const] yadc::Unit<ElectricDipoleMoment<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
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

/// The units of `ElectricQuadrupoleMoment`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricQuadrupoleMoment<N>, number = N)]
#[unit(for<N> where ElectricQuadrupoleMoment<N>: [const] yadc::Unit<ElectricQuadrupoleMoment<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ElectricQuadrupoleMomentUnit {
    #[default]
    #[unit("C · m²", "coulomb square metres", by = ElectricQuadrupoleMoment::<N>::from(METRE * METRE * SECOND * AMPERE))]
    CoulombSquareMetre,
    #[unit("e · b", "elementary charge barns", by = ElectricQuadrupoleMoment::<N>::from((METRE * METRE * SECOND * AMPERE) * (Exact::dec(1602176634, -28) * Exact::dec(1, -28))))]
    ElementaryChargeBarn,
    #[unit("e · a₀²", "atomic units of electric quadrupole moment", by = ElectricQuadrupoleMoment::<N>::from((METRE * METRE * SECOND * AMPERE) * (Exact::dec(1602176634, -28) * Exact::dec(529177210903, -22) * Exact::dec(529177210903, -22))))]
    AtomicUnitOfElectricQuadrupoleMoment,
}

/// The units of `ElectricPermittivity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricPermittivity<N>, number = N)]
#[unit(for<N> where ElectricPermittivity<N>: [const] yadc::Unit<ElectricPermittivity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ElectricPermittivityUnit {
    #[default]
    #[unit("F/m", "farads per metre", by = ElectricPermittivity::<N>::from(SECOND * SECOND * SECOND * SECOND * AMPERE * AMPERE / (METRE * METRE * METRE * KILOGRAM)))]
    FaradPerMetre,
    #[unit("ε₀", "vacuum electric permittivity", by = ElectricPermittivity::<N>::from(VACUUM_PERMITTIVITY))]
    VacuumPermittivity,
}

/// The units of `ElectricalConductance`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricalConductance<N>, number = N)]
#[unit(for<N> where ElectricalConductance<N>: [const] yadc::Unit<ElectricalConductance<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
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

/// The units of `ElectricalConductivity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricalConductivity<N>, number = N)]
#[unit(for<N> where ElectricalConductivity<N>: [const] yadc::Unit<ElectricalConductivity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ElectricalConductivityUnit {
    #[default]
    #[unit("S/m", "siemens per metre", by = ElectricalConductivity::<N>::from(SECOND * SECOND * SECOND * AMPERE * AMPERE / (METRE * METRE * METRE * KILOGRAM)))]
    SiemensPerMetre,
}

/// The units of `ElectricalMobility`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricalMobility<N>, number = N)]
#[unit(for<N> where ElectricalMobility<N>: [const] yadc::Unit<ElectricalMobility<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ElectricalMobilityUnit {
    #[default]
    #[unit("m²/(V · s)", "square metres per volt second", by = ElectricalMobility::<N>::from(SECOND * SECOND * AMPERE / KILOGRAM))]
    SquareMetrePerVoltSecond,
}

/// The units of `ElectricalResistance`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricalResistance<N>, number = N)]
#[unit(for<N> where ElectricalResistance<N>: [const] yadc::Unit<ElectricalResistance<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ElectricalResistanceUnit {
    #[default]
    #[unit("Ω", "ohms", by = ElectricalResistance::<N>::from(OHM))]
    Ohm,
    #[unit("abΩ", "abohms", by = ElectricalResistance::<N>::from(ABOHM))]
    Abohm,
    #[unit("statΩ", "statohms", by = ElectricalResistance::<N>::from(STATOHM))]
    Statohm,
}

/// The units of `ElectricalResistivity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricalResistivity<N>, number = N)]
#[unit(for<N> where ElectricalResistivity<N>: [const] yadc::Unit<ElectricalResistivity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
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

/// The units of `Capacitance`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Capacitance<N>, number = N)]
#[unit(for<N> where Capacitance<N>: [const] yadc::Unit<Capacitance<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum CapacitanceUnit {
    #[default]
    #[unit("F", "farads", by = Capacitance::<N>::from(FARAD))]
    Farad,
    #[unit("abF", "abfarads", by = Capacitance::<N>::from(ABFARAD))]
    Abfarad,
    #[unit("statF", "statfarads", by = Capacitance::<N>::from(STATFARAD))]
    Statfarad,
}

/// The units of `Inductance`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Inductance<N>, number = N)]
#[unit(for<N> where Inductance<N>: [const] yadc::Unit<Inductance<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum InductanceUnit {
    #[default]
    #[unit("H", "henries", by = Inductance::<N>::from(HENRY))]
    Henry,
    #[unit("abH", "abhenries", by = Inductance::<N>::from(ABHENRY))]
    Abhenry,
    #[unit("statH", "stathenries", by = Inductance::<N>::from(STATHENRY))]
    Stathenry,
}

/// The units of `MagneticFieldStrength`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticFieldStrength<N>, number = N)]
#[unit(for<N> where MagneticFieldStrength<N>: [const] yadc::Unit<MagneticFieldStrength<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MagneticFieldStrengthUnit {
    #[default]
    #[unit("A/m", "amperes per metre", by = MagneticFieldStrength::<N>::from(AMPERE / METRE))]
    AmperePerMetre,
    #[unit("Oe", "oersteds", by = MagneticFieldStrength::<N>::from(OERSTED))]
    Oersted,
}

/// The units of `MagneticFlux`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticFlux<N>, number = N)]
#[unit(for<N> where MagneticFlux<N>: [const] yadc::Unit<MagneticFlux<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MagneticFluxUnit {
    #[default]
    #[unit("Wb", "webers", by = MagneticFlux::<N>::from(WEBER))]
    Weber,
    #[unit("Mx", "maxwells", by = MagneticFlux::<N>::from(MAXWELL))]
    Maxwell,
}

/// The units of `MagneticFluxDensity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticFluxDensity<N>, number = N)]
#[unit(for<N> where MagneticFluxDensity<N>: [const] yadc::Unit<MagneticFluxDensity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MagneticFluxDensityUnit {
    #[default]
    #[unit("T", "teslas", by = MagneticFluxDensity::<N>::from(TESLA))]
    Tesla,
    #[unit("γ", "gammas", by = MagneticFluxDensity::<N>::from(GAMMA))]
    Gamma,
    #[unit("G", "gauss", by = MagneticFluxDensity::<N>::from(GAUSS))]
    Gauss,
}

/// The units of `MagneticMoment`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticMoment<N>, number = N)]
#[unit(for<N> where MagneticMoment<N>: [const] yadc::Unit<MagneticMoment<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
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

/// The units of `MagneticPermeability`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticPermeability<N>, number = N)]
#[unit(for<N> where MagneticPermeability<N>: [const] yadc::Unit<MagneticPermeability<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MagneticPermeabilityUnit {
    #[default]
    #[unit("H/m", "henrys per metre", by = MagneticPermeability::<N>::from(METRE * KILOGRAM / (SECOND * SECOND * AMPERE * AMPERE)))]
    HenryPerMetre,
    #[unit("µ₀", "vacuum magnetic permeability", by = MagneticPermeability::<N>::from(VACUUM_PERMEABILITY))]
    VacuumPermeability,
}

/// The units of `GyromagneticRatio`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = GyromagneticRatio<N>, number = N)]
#[unit(for<N> where GyromagneticRatio<N>: [const] yadc::Unit<GyromagneticRatio<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
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

/// The units of `ReducedGyromagneticRatio`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ReducedGyromagneticRatio<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, ReducedGyromagneticRatio<N>: [const] yadc::Unit<ReducedGyromagneticRatio<N>, N> + [const] core::marker::Destruct)]
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
