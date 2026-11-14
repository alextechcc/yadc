//! The quantities of `fluid` and their units.

#[allow(unused_imports)]
use crate::*;

/// A dynamic viscosity, in `N`s of the unit `U`: by default its base unit, `PASCAL_SECOND`.
pub type DynamicViscosity<N, const U: SIUnit = { PASCAL_SECOND.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Pascal seconds (`Pa · s`), a unit of `DynamicViscosity`: the coherent unit, a product of base units.
pub const PASCAL_SECOND: Unit<{ (KILOGRAM / (METRE * SECOND)).unit() }, BASE> = Unit::ONE;

/// Poises (`P`), a unit of `DynamicViscosity`: `n` of them is `n · 0.1` base units.
pub const POISE: Unit<{ SIUnit::scaled("P", Exact::dec(1, -1), PASCAL_SECOND.unit()) }> = Unit::ONE;

/// A kinematic viscosity, in `N`s of the unit `U`: by default its base unit, `METRE_SQUARED_PER_SECOND`.
pub type KinematicViscosity<
    N,
    const U: SIUnit = { METRE_SQUARED_PER_SECOND.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `KinematicViscosity`: a product of base units.
pub const METRE_SQUARED_PER_SECOND: Unit<{ (METRE * METRE / SECOND).unit() }, BASE> = Unit::ONE;

/// Stokes (`St`), a unit of `KinematicViscosity`: `n` of them is `n · 0.0001` base units.
pub const KINEMATIC_VISCOSITY_STOKES: Unit<
    { SIUnit::scaled("St", Exact::dec(1, -4), METRE_SQUARED_PER_SECOND.unit()) },
> = Unit::ONE;

/// A surface tension, in `N`s of the unit `U`: by default its base unit, `KILOGRAM_PER_SECOND_SQUARED`.
pub type SurfaceTension<
    N,
    const U: SIUnit = { KILOGRAM_PER_SECOND_SQUARED.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `SurfaceTension`: a product of base units.
pub const KILOGRAM_PER_SECOND_SQUARED: Unit<{ (KILOGRAM / (SECOND * SECOND)).unit() }, BASE> =
    Unit::ONE;

/// A compressibility coefficient, in `N`s of the unit `U`: by default its base unit, `METRE_SECOND_SQUARED_PER_KILOGRAM`.
pub type CompressibilityCoefficient<
    N,
    const U: SIUnit = { METRE_SECOND_SQUARED_PER_KILOGRAM.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `CompressibilityCoefficient`: a product of base units.
pub const METRE_SECOND_SQUARED_PER_KILOGRAM: Unit<
    { (METRE * SECOND * SECOND / KILOGRAM).unit() },
    BASE,
> = Unit::ONE;

/// A mass rate, in `N`s of the unit `U`: by default its base unit, `KILOGRAM_PER_SECOND`.
pub type MassRate<N, const U: SIUnit = { KILOGRAM_PER_SECOND.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// The base unit of `MassRate`: a product of base units.
pub const KILOGRAM_PER_SECOND: Unit<{ (KILOGRAM / SECOND).unit() }, BASE> = Unit::ONE;

/// A volume rate, in `N`s of the unit `U`: by default its base unit, `METRE_CUBED_PER_SECOND`.
pub type VolumeRate<N, const U: SIUnit = { METRE_CUBED_PER_SECOND.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// The base unit of `VolumeRate`: a product of base units.
pub const METRE_CUBED_PER_SECOND: Unit<{ (METRE * METRE * METRE / SECOND).unit() }, BASE> =
    Unit::ONE;

/// A mass flux, in `N`s of the unit `U`: by default its base unit, `KILOGRAM_PER_METRE_SQUARED_SECOND`.
pub type MassFlux<
    N,
    const U: SIUnit = { KILOGRAM_PER_METRE_SQUARED_SECOND.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `MassFlux`: a product of base units.
pub const KILOGRAM_PER_METRE_SQUARED_SECOND: Unit<
    { (KILOGRAM / (METRE * METRE * SECOND)).unit() },
    BASE,
> = Unit::ONE;

/// A mass per energy, in `N`s of the unit `U`: by default its base unit, `SECOND_SQUARED_PER_METRE_SQUARED`.
pub type MassPerEnergy<
    N,
    const U: SIUnit = { SECOND_SQUARED_PER_METRE_SQUARED.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `MassPerEnergy`: a product of base units.
pub const SECOND_SQUARED_PER_METRE_SQUARED: Unit<
    { (SECOND * SECOND / (METRE * METRE)).unit() },
    BASE,
> = Unit::ONE;

/// A diffusion coefficient, in `N`s of the unit `U`: by default its base unit, `METRE_SQUARED_PER_SECOND`.
pub type DiffusionCoefficient<
    N,
    const U: SIUnit = { METRE_SQUARED_PER_SECOND.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Stokes (`St`), a unit of `DiffusionCoefficient`: `n` of them is `n · 0.0001` base units.
pub const DIFFUSION_COEFFICIENT_STOKES: Unit<
    { SIUnit::scaled("St", Exact::dec(1, -4), METRE_SQUARED_PER_SECOND.unit()) },
> = Unit::ONE;
