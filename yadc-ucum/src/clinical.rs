//! The quantities of `clinical` and their units.

#[allow(unused_imports)]
use crate::*;

// `AmountOfSubstance` is `crate::AmountOfSubstance`.

/// Equivalents (`eq`), a unit of `AmountOfSubstance`: `n` of them is `n · 602214076e15` base units.
pub const EQUIVALENTS: Unit<
    { UcumUnit::scaled("eq", Exact::int(602214076000000000000000), ONE.unit()) },
> = Unit::ONE;

/// An amount of substance dissolved particles, in `N`s of the unit `U`: by default its base unit, `ONE`.
pub type AmountOfSubstanceDissolvedParticles<
    N,
    const U: UcumUnit = { ONE.unit() },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Osmole (`osm`), a unit of `AmountOfSubstanceDissolvedParticles`: `n` of them is `n · 602214076e15` base units.
pub const OSMOLE: Unit<
    { UcumUnit::scaled("osm", Exact::int(602214076000000000000000), ONE.unit()) },
> = Unit::ONE;

/// A catalytic activity, in `N`s of the unit `U`: by default its base unit, `PER_SECOND`.
pub type CatalyticActivity<N, const U: UcumUnit = { PER_SECOND.unit() }, const B: Base = BASE> =
    Qty<U, N, B>;

/// Katal (`kat`), a unit of `CatalyticActivity`: `n` of them is `n · 602214076e15` base units.
pub const KATAL: Unit<
    {
        UcumUnit::scaled(
            "kat",
            Exact::int(602214076000000000000000),
            PER_SECOND.unit(),
        )
    },
> = Unit::ONE;

/// Unit (`U`), a unit of `CatalyticActivity`: `n` of them is `n · 301107038e8 / 3` base units.
pub const UNIT: Unit<
    {
        UcumUnit::scaled(
            "U",
            Exact::int(30110703800000000) / Exact::int(3),
            PER_SECOND.unit(),
        )
    },
> = Unit::ONE;

/// A fluid resistance, in `N`s of the unit `U`: by default its base unit, `GRAM_PER_METER_POW4_SECOND`.
pub type FluidResistance<
    N,
    const U: UcumUnit = { GRAM_PER_METER_POW4_SECOND.unit() },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// The base unit of `FluidResistance`: a product of base units.
pub const GRAM_PER_METER_POW4_SECOND: Unit<
    { (GRAM / (METER * METER * METER * METER * SECOND)).unit() },
    BASE,
> = Unit::ONE;

/// Peripheral vascular resistance unit (`[PRU]`), a unit of `FluidResistance`: `n` of them is `n · 133322e6` base units.
pub const PERIPHERAL_VASCULAR_RESISTANCE_UNIT: Unit<
    {
        UcumUnit::scaled(
            "[PRU]",
            Exact::int(133322000000),
            GRAM_PER_METER_POW4_SECOND.unit(),
        )
    },
> = Unit::ONE;

/// Wood unit (`[wood'U]`), a unit of `FluidResistance`: `n` of them is `n · 799932e4` base units.
pub const WOOD_UNIT: Unit<
    {
        UcumUnit::scaled(
            "[wood'U]",
            Exact::int(7999320000),
            GRAM_PER_METER_POW4_SECOND.unit(),
        )
    },
> = Unit::ONE;

/// A gauge of catheters, in `N`s of the unit `U`: by default its base unit, `METER`.
pub type GaugeOfCatheters<N, const U: UcumUnit = { METER.unit() }, const B: Base = BASE> =
    Qty<U, N, B>;

/// French (`[Ch]`), a unit of `GaugeOfCatheters`: `n` of them is `n · 1 / 3000` base units.
pub const FRENCH: Unit<
    { UcumUnit::scaled("[Ch]", Exact::int(1) / Exact::int(3000), METER.unit()) },
> = Unit::ONE;

// `LineicNumber` is `crate::LineicNumber`.

/// Mesh (`[mesh_i]`), a unit of `LineicNumber`: `n` of them is `n · 5e3 / 127` base units.
pub const MESH: Unit<
    {
        UcumUnit::scaled(
            "[mesh_i]",
            Exact::int(5000) / Exact::int(127),
            PER_METER.unit(),
        )
    },
> = Unit::ONE;

/// A mass concentration, in `N`s of the unit `U`: by default its base unit, `GRAM_PER_METER_CUBED`.
pub type MassConcentration<
    N,
    const U: UcumUnit = { GRAM_PER_METER_CUBED.unit() },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// The base unit of `MassConcentration`: a product of base units.
pub const GRAM_PER_METER_CUBED: Unit<{ (GRAM / (METER * METER * METER)).unit() }, BASE> = Unit::ONE;

/// Gram percent (`g%`), a unit of `MassConcentration`: `n` of them is `n · 1e4` base units.
pub const GRAM_PERCENT: Unit<
    { UcumUnit::scaled("g%", Exact::int(10000), GRAM_PER_METER_CUBED.unit()) },
> = Unit::ONE;

/// A metabolic cost of physical activity, in `N`s of the unit `U`: by default its base unit, `METER_CUBED_PER_SECOND_GRAM`.
pub type MetabolicCostOfPhysicalActivity<
    N,
    const U: UcumUnit = { METER_CUBED_PER_SECOND_GRAM.unit() },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// The base unit of `MetabolicCostOfPhysicalActivity`: a product of base units.
pub const METER_CUBED_PER_SECOND_GRAM: Unit<
    { (METER * METER * METER / (SECOND * GRAM)).unit() },
    BASE,
> = Unit::ONE;

/// Metabolic equivalent (`[MET]`), a unit of `MetabolicCostOfPhysicalActivity`: `n` of them is `n · 7 / 120000000000` base units.
pub const METABOLIC_EQUIVALENT: Unit<
    {
        UcumUnit::scaled(
            "[MET]",
            Exact::int(7) / Exact::int(120000000000),
            METER_CUBED_PER_SECOND_GRAM.unit(),
        )
    },
> = Unit::ONE;

// `Pressure` is `crate::Pressure`.

/// Meter of water column (`m[H2O]`), a unit of `Pressure`: `n` of them is `n · 980665e1` base units.
pub const METER_OF_WATER_COLUMN: Unit<
    {
        UcumUnit::scaled(
            "m[H2O]",
            Exact::int(9806650),
            GRAM_PER_METER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// Meter of mercury column (`m[Hg]`), a unit of `Pressure`: `n` of them is `n · 133322e3` base units.
pub const METER_OF_MERCURY_COLUMN: Unit<
    {
        UcumUnit::scaled(
            "m[Hg]",
            Exact::int(133322000),
            GRAM_PER_METER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// Inch of water column (`[in_i'H2O]`), a unit of `Pressure`: `n` of them is `n · 24908891e-2` base units.
pub const INCH_OF_WATER_COLUMN: Unit<
    {
        UcumUnit::scaled(
            "[in_i'H2O]",
            Exact::dec(24908891, -2),
            GRAM_PER_METER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// Inch of mercury column (`[in_i'Hg]`), a unit of `Pressure`: `n` of them is `n · 33863788e-1` base units.
pub const INCH_OF_MERCURY_COLUMN: Unit<
    {
        UcumUnit::scaled(
            "[in_i'Hg]",
            Exact::dec(33863788, -1),
            GRAM_PER_METER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// A refraction of a lens, in `N`s of the unit `U`: by default its base unit, `DIOPTER`.
pub type RefractionOfALens<N, const U: UcumUnit = { DIOPTER.unit() }, const B: Base = BASE> =
    Qty<U, N, B>;

/// Diopter (`[diop]`), a unit of `RefractionOfALens`: the coherent unit, a product of base units.
pub const DIOPTER: Unit<{ UcumUnit::ONE.times(METER.unit(), -1) }, BASE> = Unit::ONE;

/// A sedimentation coefficient, in `N`s of the unit `U`: by default its base unit, `SECOND`.
pub type SedimentationCoefficient<N, const U: UcumUnit = { SECOND.unit() }, const B: Base = BASE> =
    Qty<U, N, B>;

/// Svedberg unit (`[S]`), a unit of `SedimentationCoefficient`: `n` of them is `n · 1e-13` base units.
pub const SVEDBERG_UNIT: Unit<{ UcumUnit::scaled("[S]", Exact::dec(1, -13), SECOND.unit()) }> =
    Unit::ONE;

/// A view area in microscope, in `N`s of the unit `U`: by default its base unit, `HIGH_POWER_FIELD`.
pub type ViewAreaInMicroscope<
    N,
    const U: UcumUnit = { HIGH_POWER_FIELD.unit() },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// High power field (`[HPF]`), a unit of `ViewAreaInMicroscope`: the coherent unit, a product of base units.
pub const HIGH_POWER_FIELD: Unit<{ UcumUnit::ONE }, BASE> = Unit::ONE;

/// Low power field (`[LPF]`), a unit of `ViewAreaInMicroscope`: `n` of them is `n · 1e2` base units.
pub const LOW_POWER_FIELD: Unit<
    { UcumUnit::scaled("[LPF]", Exact::int(100), HIGH_POWER_FIELD.unit()) },
> = Unit::ONE;

// `Volume` is `crate::Volume`.

/// Drop (`[drp]`), a unit of `Volume`: `n` of them is `n · 5e-8` base units.
pub const DROP: Unit<{ UcumUnit::scaled("[drp]", Exact::dec(5, -8), METER_CUBED.unit()) }> =
    Unit::ONE;

/// A x ray attenuation, in `N`s of the unit `U`: by default its base unit, `HOUNSFIELD_UNIT`.
pub type XRayAttenuation<N, const U: UcumUnit = { HOUNSFIELD_UNIT.unit() }, const B: Base = BASE> =
    Qty<U, N, B>;

/// Hounsfield unit (`[hnsf'U]`), a unit of `XRayAttenuation`: the coherent unit, a product of base units.
pub const HOUNSFIELD_UNIT: Unit<{ UcumUnit::ONE }, BASE> = Unit::ONE;
