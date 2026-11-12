use crate::{Exact, ISQ, SIQuantity};
use yadc::Dimension;

pub type MassDensity<N> = SIQuantity<
    {
        ISQ {
            m: -3,
            kg: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type LinearMassDensity<N> = SIQuantity<
    {
        ISQ {
            m: -1,
            kg: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ArealMassDensity<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            kg: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type LinearNumberDensity<N> = SIQuantity<
    {
        ISQ {
            m: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ArealNumberDensity<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type VolumetricNumberDensity<N> = SIQuantity<
    {
        ISQ {
            m: -3,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type LinearNumberRate<N> = SIQuantity<
    {
        ISQ {
            m: -1,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ArealNumberRate<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type VolumetricNumberRate<N> = SIQuantity<
    {
        ISQ {
            m: -3,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type LinearDensityOfStates<N> = SIQuantity<
    {
        ISQ {
            m: -3,
            kg: -1,
            s: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ArealDensityOfStates<N> = SIQuantity<
    {
        ISQ {
            m: -4,
            kg: -1,
            s: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type VolumetricDensityOfStates<N> = SIQuantity<
    {
        ISQ {
            m: -5,
            kg: -1,
            s: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type LinearPowerDensity<N> = SIQuantity<
    {
        ISQ {
            m: 1,
            kg: 1,
            s: -3,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type VolumetricPowerDensity<N> = SIQuantity<
    {
        ISQ {
            m: -1,
            kg: 1,
            s: -3,
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
    use crate::base::{METRE, ONE};

    /// One `PerMetre` (`m⁻¹`), as a `LinearNumberDensity`.
    pub const PER_METRE: LinearNumberDensity<Exact> = ONE / METRE;

    /// One `PerFoot` (`ft⁻¹`), as a `LinearNumberDensity`.
    pub const PER_FOOT: LinearNumberDensity<Exact> =
        PER_METRE * (Exact::int(1) / Exact::dec(3048, -4));

    /// One `PerFootSurvey` (`ft (U.S. survey)`), as a `LinearNumberDensity`.
    pub const PER_FOOT_SURVEY: LinearNumberDensity<Exact> =
        PER_METRE * (Exact::int(1) / (Exact::int(1200) / Exact::int(3937)));

    /// One `PerInch` (`in⁻¹`), as a `LinearNumberDensity`.
    pub const PER_INCH: LinearNumberDensity<Exact> =
        PER_METRE * (Exact::int(1) / Exact::dec(254, -4));

    /// One `PerMile` (`mi⁻¹`), as a `LinearNumberDensity`.
    pub const PER_MILE: LinearNumberDensity<Exact> =
        PER_METRE * (Exact::int(1) / Exact::dec(1609344, -3));

    /// One `PerMileSurvey` (`mi⁻¹ (U.S. survey)`), as a `LinearNumberDensity`.
    pub const PER_MILE_SURVEY: LinearNumberDensity<Exact> =
        PER_METRE * (Exact::int(1) / (Exact::int(6336000) / Exact::int(3937)));

    /// One `PerNauticalMile` (`M⁻¹`), as a `LinearNumberDensity`.
    pub const PER_NAUTICAL_MILE: LinearNumberDensity<Exact> =
        PER_METRE * (Exact::int(1) / Exact::int(1852));

    /// One `PerYard` (`yd⁻¹`), as a `LinearNumberDensity`.
    pub const PER_YARD: LinearNumberDensity<Exact> =
        PER_METRE * (Exact::int(1) / Exact::dec(9144, -4));

    /// One `PerFluidOunce` (`per fl oz`), as a `VolumetricNumberDensity`.
    pub const PER_FLUID_OUNCE: VolumetricNumberDensity<Exact> =
        (ONE / (METRE * METRE * METRE)) * (Exact::int(1) / Exact::dec(295735295625, -16));

    /// One `PerFluidOunceImperial` (`per fl oz (UK)`), as a `VolumetricNumberDensity`.
    pub const PER_FLUID_OUNCE_IMPERIAL: VolumetricNumberDensity<Exact> =
        (ONE / (METRE * METRE * METRE)) * (Exact::int(1) / Exact::dec(284130625, -13));

    /// One `PerGallonImperial` (`per gal (UK)`), as a `VolumetricNumberDensity`.
    pub const PER_GALLON_IMPERIAL: VolumetricNumberDensity<Exact> =
        (ONE / (METRE * METRE * METRE)) * (Exact::int(1) / Exact::dec(454609, -8));

    /// One `PerGallon` (`per gal`), as a `VolumetricNumberDensity`.
    pub const PER_GALLON: VolumetricNumberDensity<Exact> =
        (ONE / (METRE * METRE * METRE)) * (Exact::int(1) / Exact::dec(3785411784, -12));

    /// One `PerLitre` (`L⁻¹`), as a `VolumetricNumberDensity`.
    pub const PER_LITRE: VolumetricNumberDensity<Exact> =
        (ONE / (METRE * METRE * METRE)) * Exact::int(1000);
}
