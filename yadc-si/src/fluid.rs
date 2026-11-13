use crate::{Exact, ISQ, SIQuantity};
use yadc::Dimension;

pub type DynamicViscosity<N> = SIQuantity<
    {
        ISQ {
            m: -1,
            kg: 1,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type KinematicViscosity<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type SurfaceTension<N> = SIQuantity<
    {
        ISQ {
            kg: 1,
            s: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type CompressibilityCoefficient<N> = SIQuantity<
    {
        ISQ {
            m: 1,
            kg: -1,
            s: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MassRate<N> = SIQuantity<
    {
        ISQ {
            kg: 1,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type VolumeRate<N> = SIQuantity<
    {
        ISQ {
            m: 3,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MassFlux<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            kg: 1,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MassPerEnergy<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            s: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type DiffusionCoefficient<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            s: -1,
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
    use crate::base::{KILOGRAM, METRE, SECOND};

    /// One `Poise` (`P`), as a `DynamicViscosity`.
    pub const POISE: DynamicViscosity<Exact> = (KILOGRAM / (METRE * SECOND)) * Exact::dec(1, -1);

    /// One `Stokes` (`St`), as a `KinematicViscosity`.
    pub const KINEMATIC_VISCOSITY_STOKES: KinematicViscosity<Exact> =
        (METRE * METRE / SECOND) * Exact::dec(1, -4);

    /// One `PerPascal` (`Pa⁻¹`), as a `CompressibilityCoefficient`.
    pub const PER_PASCAL: CompressibilityCoefficient<Exact> = METRE * SECOND * SECOND / KILOGRAM;

    /// One `PerAtmosphere` (`atm⁻¹`), as a `CompressibilityCoefficient`.
    pub const PER_ATMOSPHERE: CompressibilityCoefficient<Exact> =
        PER_PASCAL * (Exact::int(1) / Exact::int(101325));

    /// One `PerAtmosphereTechnical` (`at⁻¹`), as a `CompressibilityCoefficient`.
    pub const PER_ATMOSPHERE_TECHNICAL: CompressibilityCoefficient<Exact> =
        PER_PASCAL * (Exact::int(1) / Exact::dec(980665, -1));

    /// One `PerBar` (`bar⁻¹`), as a `CompressibilityCoefficient`.
    pub const PER_BAR: CompressibilityCoefficient<Exact> = PER_PASCAL * Exact::dec(1, -5);

    /// One `Stokes` (`St`), as a `DiffusionCoefficient`.
    pub const DIFFUSION_COEFFICIENT_STOKES: DiffusionCoefficient<Exact> =
        (METRE * METRE / SECOND) * Exact::dec(1, -4);
}
