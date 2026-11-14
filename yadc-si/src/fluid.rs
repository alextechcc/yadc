//! The quantities of `fluid` and their units.

#[allow(unused_imports)]
use crate::*;

/// A dynamic viscosity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type DynamicViscosity<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), -1)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -1)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Pascal seconds (`Pa · s`), a unit of `DynamicViscosity`: the coherent unit, a product of base units.
pub const PASCAL_SECOND: SIQuantity<
    {
        SIUnit::ONE
            .times(METRE.unit(), -1)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -1)
    },
    Exact,
    BASE,
> = SIQuantity::ONE;

/// Poises (`P`), a unit of `DynamicViscosity`: `n` of them is `n · 0.1` base units.
pub const POISE: SIQuantity<
    {
        SIUnit::scaled(
            "P",
            Exact::dec(1, -1),
            SIUnit::ONE
                .times(METRE.unit(), -1)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// A kinematic viscosity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type KinematicViscosity<
    N,
    const U: SIUnit = { SIUnit::ONE.times(METRE.unit(), 2).times(SECOND.unit(), -1) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Stokes (`St`), a unit of `KinematicViscosity`: `n` of them is `n · 0.0001` base units.
pub const KINEMATIC_VISCOSITY_STOKES: SIQuantity<
    {
        SIUnit::scaled(
            "St",
            Exact::dec(1, -4),
            SIUnit::ONE.times(METRE.unit(), 2).times(SECOND.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// A surface tension, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type SurfaceTension<
    N,
    const U: SIUnit = { KILOGRAM.unit().times(SECOND.unit(), -2) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// A compressibility coefficient, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type CompressibilityCoefficient<
    N,
    const U: SIUnit = {
        METRE
            .unit()
            .times(KILOGRAM.unit(), -1)
            .times(SECOND.unit(), 2)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// A mass rate, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MassRate<
    N,
    const U: SIUnit = { KILOGRAM.unit().times(SECOND.unit(), -1) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// A volume rate, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type VolumeRate<
    N,
    const U: SIUnit = { SIUnit::ONE.times(METRE.unit(), 3).times(SECOND.unit(), -1) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// A mass flux, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MassFlux<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), -2)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -1)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// A mass per energy, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MassPerEnergy<
    N,
    const U: SIUnit = { SIUnit::ONE.times(METRE.unit(), -2).times(SECOND.unit(), 2) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// A diffusion coefficient, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type DiffusionCoefficient<
    N,
    const U: SIUnit = { SIUnit::ONE.times(METRE.unit(), 2).times(SECOND.unit(), -1) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Stokes (`St`), a unit of `DiffusionCoefficient`: `n` of them is `n · 0.0001` base units.
pub const DIFFUSION_COEFFICIENT_STOKES: SIQuantity<
    {
        SIUnit::scaled(
            "St",
            Exact::dec(1, -4),
            SIUnit::ONE.times(METRE.unit(), 2).times(SECOND.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;
