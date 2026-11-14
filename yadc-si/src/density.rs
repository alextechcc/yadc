//! The quantities of `density` and their units.

#[allow(unused_imports)]
use crate::*;

/// A mass density, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MassDensity<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), -3)
            .times(KILOGRAM.unit(), 1)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// A linear mass density, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type LinearMassDensity<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), -1)
            .times(KILOGRAM.unit(), 1)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// An areal mass density, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ArealMassDensity<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), -2)
            .times(KILOGRAM.unit(), 1)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// A linear number density, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type LinearNumberDensity<
    N,
    const U: SIUnit = { SIUnit::ONE.times(METRE.unit(), -1) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// An areal number density, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ArealNumberDensity<
    N,
    const U: SIUnit = { SIUnit::ONE.times(METRE.unit(), -2) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// A volumetric number density, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type VolumetricNumberDensity<
    N,
    const U: SIUnit = { SIUnit::ONE.times(METRE.unit(), -3) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// A linear number rate, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type LinearNumberRate<
    N,
    const U: SIUnit = { SIUnit::ONE.times(METRE.unit(), -1).times(SECOND.unit(), -1) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// An areal number rate, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ArealNumberRate<
    N,
    const U: SIUnit = { SIUnit::ONE.times(METRE.unit(), -2).times(SECOND.unit(), -1) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// A volumetric number rate, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type VolumetricNumberRate<
    N,
    const U: SIUnit = { SIUnit::ONE.times(METRE.unit(), -3).times(SECOND.unit(), -1) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// A linear density of states, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type LinearDensityOfStates<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), -3)
            .times(KILOGRAM.unit(), -1)
            .times(SECOND.unit(), 2)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// An areal density of states, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ArealDensityOfStates<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), -4)
            .times(KILOGRAM.unit(), -1)
            .times(SECOND.unit(), 2)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// A volumetric density of states, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type VolumetricDensityOfStates<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), -5)
            .times(KILOGRAM.unit(), -1)
            .times(SECOND.unit(), 2)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// A linear power density, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type LinearPowerDensity<
    N,
    const U: SIUnit = {
        METRE
            .unit()
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -3)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// A volumetric power density, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type VolumetricPowerDensity<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), -1)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -3)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;
