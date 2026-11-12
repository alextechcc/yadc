use crate::{Dim, Exact, Qty};
use yadc::Dimension;

pub type AmountOfSubstance<N> = Qty<{ Dim::DIMENSIONLESS }, N>;

pub type AmountOfSubstanceDissolvedParticles<N> = Qty<{ Dim::DIMENSIONLESS }, N>;

pub type CatalyticActivity<N> = Qty<
    {
        Dim {
            s: -1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type FluidResistance<N> = Qty<
    {
        Dim {
            m: -4,
            s: -1,
            g: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type GaugeOfCatheters<N> = Qty<
    {
        Dim {
            m: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type LineicNumber<N> = Qty<
    {
        Dim {
            m: -1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type MassConcentration<N> = Qty<
    {
        Dim {
            m: -3,
            g: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type MetabolicCostOfPhysicalActivity<N> = Qty<
    {
        Dim {
            m: 3,
            s: -1,
            g: -1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Pressure<N> = Qty<
    {
        Dim {
            m: -1,
            s: -2,
            g: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type RefractionOfALens<N> = Qty<
    {
        Dim {
            m: -1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type SedimentationCoefficient<N> = Qty<
    {
        Dim {
            s: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type ViewAreaInMicroscope<N> = Qty<{ Dim::DIMENSIONLESS }, N>;

pub type Volume<N> = Qty<
    {
        Dim {
            m: 3,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type XRayAttenuation<N> = Qty<{ Dim::DIMENSIONLESS }, N>;

/// The constants of this module: the base constants of its quantities
/// (re-exported), then one per named unit, each defined exactly from them
/// (`INCH` is `0.0254 * METRE`). A unit written as others is not one:
/// `SLUG / CUBIC_FOOT`, `KILO * METRE` are exact too. Converting rounds,
/// once, where it is written - in a `const`, by the compiler:
/// `const MPH: Velocity<f64> = (MILE / HOUR).into();`, then `MPH(60.0)`.
pub mod consts {
    use super::*;
    use crate::base::GRAM;

    /// The number one: the unit of dimension one.
    pub use crate::base::ONE;

    /// The metre, UCUM's base unit of length.
    pub use crate::base::METRE;

    /// The second, UCUM's base unit of time.
    pub use crate::base::SECOND;

    /// One `Equivalents` (`eq`), as an `AmountOfSubstance`.
    pub const EQUIVALENTS: AmountOfSubstance<Exact> = ONE * Exact::int(602214076000000000000000);

    /// One `Osmole` (`osm`), as an `AmountOfSubstanceDissolvedParticles`.
    pub const OSMOLE: AmountOfSubstanceDissolvedParticles<Exact> =
        ONE * Exact::int(602214076000000000000000);

    /// One `Katal` (`kat`), as a `CatalyticActivity`.
    pub const KATAL: CatalyticActivity<Exact> =
        (ONE / SECOND) * Exact::int(602214076000000000000000);

    /// One `Unit` (`U`), as a `CatalyticActivity`.
    pub const UNIT: CatalyticActivity<Exact> =
        (ONE / SECOND) * (Exact::int(30110703800000000) / Exact::int(3));

    /// One `PeripheralVascularResistanceUnit` (`[PRU]`), as a `FluidResistance`.
    pub const PERIPHERAL_VASCULAR_RESISTANCE_UNIT: FluidResistance<Exact> =
        (GRAM / (METRE * METRE * METRE * METRE * SECOND)) * Exact::int(133322000000);

    /// One `WoodUnit` (`[wood'U]`), as a `FluidResistance`.
    pub const WOOD_UNIT: FluidResistance<Exact> =
        (GRAM / (METRE * METRE * METRE * METRE * SECOND)) * Exact::int(7999320000);

    /// One `French` (`[Ch]`), as a `GaugeOfCatheters`.
    pub const FRENCH: GaugeOfCatheters<Exact> = METRE * (Exact::int(1) / Exact::int(3000));

    /// One `Mesh` (`[mesh_i]`), as a `LineicNumber`.
    pub const MESH: LineicNumber<Exact> = (ONE / METRE) * (Exact::int(5000) / Exact::int(127));

    /// One `GramPercent` (`g%`), as a `MassConcentration`.
    pub const GRAM_PERCENT: MassConcentration<Exact> =
        (GRAM / (METRE * METRE * METRE)) * Exact::int(10000);

    /// One `MetabolicEquivalent` (`[MET]`), as a `MetabolicCostOfPhysicalActivity`.
    pub const METABOLIC_EQUIVALENT: MetabolicCostOfPhysicalActivity<Exact> =
        (METRE * METRE * METRE / (SECOND * GRAM)) * (Exact::int(7) / Exact::int(120000000000));

    /// One `MeterOfWaterColumn` (`m[H2O]`), as a `Pressure`.
    pub const METER_OF_WATER_COLUMN: Pressure<Exact> =
        (GRAM / (METRE * SECOND * SECOND)) * Exact::int(9806650);

    /// One `MeterOfMercuryColumn` (`m[Hg]`), as a `Pressure`.
    pub const METER_OF_MERCURY_COLUMN: Pressure<Exact> =
        (GRAM / (METRE * SECOND * SECOND)) * Exact::int(133322000);

    /// One `InchOfWaterColumn` (`[in_i'H2O]`), as a `Pressure`.
    pub const INCH_OF_WATER_COLUMN: Pressure<Exact> =
        (GRAM / (METRE * SECOND * SECOND)) * Exact::dec(24908891, -2);

    /// One `InchOfMercuryColumn` (`[in_i'Hg]`), as a `Pressure`.
    pub const INCH_OF_MERCURY_COLUMN: Pressure<Exact> =
        (GRAM / (METRE * SECOND * SECOND)) * Exact::dec(33863788, -1);

    /// One `Diopter` (`[diop]`), as a `RefractionOfALens`.
    pub const DIOPTER: RefractionOfALens<Exact> = ONE / METRE;

    /// One `SvedbergUnit` (`[S]`), as a `SedimentationCoefficient`.
    pub const SVEDBERG_UNIT: SedimentationCoefficient<Exact> = SECOND * Exact::dec(1, -13);

    /// One `HighPowerField` (`[HPF]`), as a `ViewAreaInMicroscope`.
    pub const HIGH_POWER_FIELD: ViewAreaInMicroscope<Exact> = ONE;

    /// One `LowPowerField` (`[LPF]`), as a `ViewAreaInMicroscope`.
    pub const LOW_POWER_FIELD: ViewAreaInMicroscope<Exact> = HIGH_POWER_FIELD * Exact::int(100);

    /// One `Drop` (`[drp]`), as a `Volume`.
    pub const DROP: Volume<Exact> = (METRE * METRE * METRE) * Exact::dec(5, -8);

    /// One `HounsfieldUnit` (`[hnsf'U]`), as a `XRayAttenuation`.
    pub const HOUNSFIELD_UNIT: XRayAttenuation<Exact> = ONE;
}
