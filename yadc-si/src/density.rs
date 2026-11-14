//! The quantities of `density` and their units.

#[allow(unused_imports)]
use crate::*;

/// A mass density, in `N`s of the unit `U`: by default its base unit, `KILOGRAM_PER_METRE_CUBED`.
pub type MassDensity<
    N,
    const U: SIUnit = { KILOGRAM_PER_METRE_CUBED.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `MassDensity`: a product of base units.
pub const KILOGRAM_PER_METRE_CUBED: Unit<{ (KILOGRAM / (METRE * METRE * METRE)).unit() }, BASE> =
    Unit::ONE;

/// A linear mass density, in `N`s of the unit `U`: by default its base unit, `KILOGRAM_PER_METRE`.
pub type LinearMassDensity<
    N,
    const U: SIUnit = { KILOGRAM_PER_METRE.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `LinearMassDensity`: a product of base units.
pub const KILOGRAM_PER_METRE: Unit<{ (KILOGRAM / METRE).unit() }, BASE> = Unit::ONE;

/// An areal mass density, in `N`s of the unit `U`: by default its base unit, `KILOGRAM_PER_METRE_SQUARED`.
pub type ArealMassDensity<
    N,
    const U: SIUnit = { KILOGRAM_PER_METRE_SQUARED.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `ArealMassDensity`: a product of base units.
pub const KILOGRAM_PER_METRE_SQUARED: Unit<{ (KILOGRAM / (METRE * METRE)).unit() }, BASE> =
    Unit::ONE;

/// A linear number density, in `N`s of the unit `U`: by default its base unit, `PER_METRE`.
pub type LinearNumberDensity<N, const U: SIUnit = { PER_METRE.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// The base unit of `LinearNumberDensity`: a product of base units.
pub const PER_METRE: Unit<{ SIUnit::ONE.times(METRE.unit(), -1) }, BASE> = Unit::ONE;

/// An areal number density, in `N`s of the unit `U`: by default its base unit, `PER_METRE_SQUARED`.
pub type ArealNumberDensity<
    N,
    const U: SIUnit = { PER_METRE_SQUARED.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `ArealNumberDensity`: a product of base units.
pub const PER_METRE_SQUARED: Unit<
    { SIUnit::ONE.times(METRE.unit(), -1).times(METRE.unit(), -1) },
    BASE,
> = Unit::ONE;

/// A volumetric number density, in `N`s of the unit `U`: by default its base unit, `PER_METRE_CUBED`.
pub type VolumetricNumberDensity<
    N,
    const U: SIUnit = { PER_METRE_CUBED.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `VolumetricNumberDensity`: a product of base units.
pub const PER_METRE_CUBED: Unit<
    {
        SIUnit::ONE
            .times(METRE.unit(), -1)
            .times(METRE.unit(), -1)
            .times(METRE.unit(), -1)
    },
    BASE,
> = Unit::ONE;

/// A linear number rate, in `N`s of the unit `U`: by default its base unit, `PER_METRE_SECOND`.
pub type LinearNumberRate<N, const U: SIUnit = { PER_METRE_SECOND.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// The base unit of `LinearNumberRate`: a product of base units.
pub const PER_METRE_SECOND: Unit<
    { SIUnit::ONE.times(METRE.unit(), -1).times(SECOND.unit(), -1) },
    BASE,
> = Unit::ONE;

/// An areal number rate, in `N`s of the unit `U`: by default its base unit, `PER_METRE_SQUARED_SECOND`.
pub type ArealNumberRate<
    N,
    const U: SIUnit = { PER_METRE_SQUARED_SECOND.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `ArealNumberRate`: a product of base units.
pub const PER_METRE_SQUARED_SECOND: Unit<
    {
        SIUnit::ONE
            .times(METRE.unit(), -1)
            .times(METRE.unit(), -1)
            .times(SECOND.unit(), -1)
    },
    BASE,
> = Unit::ONE;

/// A volumetric number rate, in `N`s of the unit `U`: by default its base unit, `PER_METRE_CUBED_SECOND`.
pub type VolumetricNumberRate<
    N,
    const U: SIUnit = { PER_METRE_CUBED_SECOND.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `VolumetricNumberRate`: a product of base units.
pub const PER_METRE_CUBED_SECOND: Unit<
    {
        SIUnit::ONE
            .times(METRE.unit(), -1)
            .times(METRE.unit(), -1)
            .times(METRE.unit(), -1)
            .times(SECOND.unit(), -1)
    },
    BASE,
> = Unit::ONE;

/// A linear density of states, in `N`s of the unit `U`: by default its base unit, `SECOND_SQUARED_PER_METRE_CUBED_KILOGRAM`.
pub type LinearDensityOfStates<
    N,
    const U: SIUnit = { SECOND_SQUARED_PER_METRE_CUBED_KILOGRAM.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `LinearDensityOfStates`: a product of base units.
pub const SECOND_SQUARED_PER_METRE_CUBED_KILOGRAM: Unit<
    { (SECOND * SECOND / (METRE * METRE * METRE * KILOGRAM)).unit() },
    BASE,
> = Unit::ONE;

/// An areal density of states, in `N`s of the unit `U`: by default its base unit, `SECOND_SQUARED_PER_METRE_POW4_KILOGRAM`.
pub type ArealDensityOfStates<
    N,
    const U: SIUnit = { SECOND_SQUARED_PER_METRE_POW4_KILOGRAM.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `ArealDensityOfStates`: a product of base units.
pub const SECOND_SQUARED_PER_METRE_POW4_KILOGRAM: Unit<
    { (SECOND * SECOND / (METRE * METRE * METRE * METRE * KILOGRAM)).unit() },
    BASE,
> = Unit::ONE;

/// A volumetric density of states, in `N`s of the unit `U`: by default its base unit, `SECOND_SQUARED_PER_METRE_POW5_KILOGRAM`.
pub type VolumetricDensityOfStates<
    N,
    const U: SIUnit = { SECOND_SQUARED_PER_METRE_POW5_KILOGRAM.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `VolumetricDensityOfStates`: a product of base units.
pub const SECOND_SQUARED_PER_METRE_POW5_KILOGRAM: Unit<
    { (SECOND * SECOND / (METRE * METRE * METRE * METRE * METRE * KILOGRAM)).unit() },
    BASE,
> = Unit::ONE;

/// A linear power density, in `N`s of the unit `U`: by default its base unit, `METRE_KILOGRAM_PER_SECOND_CUBED`.
pub type LinearPowerDensity<
    N,
    const U: SIUnit = { METRE_KILOGRAM_PER_SECOND_CUBED.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `LinearPowerDensity`: a product of base units.
pub const METRE_KILOGRAM_PER_SECOND_CUBED: Unit<
    { (METRE * KILOGRAM / (SECOND * SECOND * SECOND)).unit() },
    BASE,
> = Unit::ONE;

/// A volumetric power density, in `N`s of the unit `U`: by default its base unit, `KILOGRAM_PER_METRE_SECOND_CUBED`.
pub type VolumetricPowerDensity<
    N,
    const U: SIUnit = { KILOGRAM_PER_METRE_SECOND_CUBED.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `VolumetricPowerDensity`: a product of base units.
pub const KILOGRAM_PER_METRE_SECOND_CUBED: Unit<
    { (KILOGRAM / (METRE * SECOND * SECOND * SECOND)).unit() },
    BASE,
> = Unit::ONE;
