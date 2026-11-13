use crate::{Exact, ISQ, SIQuantity};
use yadc::Dimension;

pub type AmountOfSubstance<N> = SIQuantity<
    {
        ISQ {
            mol: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MolarConcentration<N> = SIQuantity<
    {
        ISQ {
            m: -3,
            mol: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MolarEnergy<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -2,
            mol: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MolarFlux<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            s: -1,
            mol: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MolarHeatCapacity<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -2,
            K: -1,
            mol: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MolarMass<N> = SIQuantity<
    {
        ISQ {
            kg: 1,
            mol: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MolarRadioactivity<N> = SIQuantity<
    {
        ISQ {
            s: -1,
            mol: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MolarVolume<N> = SIQuantity<
    {
        ISQ {
            m: 3,
            mol: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Molality<N> = SIQuantity<
    {
        ISQ {
            kg: -1,
            mol: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MassConcentration<N> = SIQuantity<
    {
        ISQ {
            m: -3,
            kg: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type CatalyticActivity<N> = SIQuantity<
    {
        ISQ {
            s: -1,
            mol: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type CatalyticActivityConcentration<N> = SIQuantity<
    {
        ISQ {
            m: -3,
            s: -1,
            mol: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Radioactivity<N> = SIQuantity<
    {
        ISQ {
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type SpecificRadioactivity<N> = SIQuantity<
    {
        ISQ {
            kg: -1,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type InverseAmountOfSubstance<N> = SIQuantity<
    {
        ISQ {
            mol: -1,
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
    use crate::base::{KELVIN, KILOGRAM, METRE, ONE, SECOND};

    /// The mole, the SI base unit of amount of substance.
    pub use crate::base::MOLE;

    /// One `Particle` (`particle`), as an `AmountOfSubstance`.
    pub const PARTICLE: AmountOfSubstance<Exact> =
        MOLE * (Exact::int(1) / Exact::int(602214076000000000000000));

    /// One `MolarGasConstant` (`R`), as a `MolarHeatCapacity`.
    pub const MOLAR_GAS_CONSTANT: MolarHeatCapacity<Exact> =
        (METRE * METRE * KILOGRAM / (SECOND * SECOND * KELVIN * MOLE)) * Exact::dec(8314462618, -9);

    /// One `Katal` (`kat`), as a `CatalyticActivity`.
    pub const KATAL: CatalyticActivity<Exact> = MOLE / SECOND;

    /// One `Becquerel` (`Bq`), as a `Radioactivity`.
    pub const BECQUEREL: Radioactivity<Exact> = ONE / SECOND;

    /// One `Curie` (`Ci`), as a `Radioactivity`.
    pub const CURIE: Radioactivity<Exact> = BECQUEREL * Exact::int(37000000000);

    /// One `DisintegrationsPerMinute` (`dpm`), as a `Radioactivity`.
    pub const DISINTEGRATIONS_PER_MINUTE: Radioactivity<Exact> =
        BECQUEREL * (Exact::int(1) / Exact::int(60));

    /// One `PerMole` (`mol⁻¹`), as an `InverseAmountOfSubstance`.
    pub const PER_MOLE: InverseAmountOfSubstance<Exact> = ONE / MOLE;

    /// One `AvogadroConstant` (`N_A`), as an `InverseAmountOfSubstance`.
    pub const AVOGADRO_CONSTANT: InverseAmountOfSubstance<Exact> =
        PER_MOLE * Exact::int(602214076000000000000000);
}
