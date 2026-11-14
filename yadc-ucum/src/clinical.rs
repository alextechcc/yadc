//! The quantities of `clinical` and their units.

#[allow(unused_imports)]
use crate::*;

// `AmountOfSubstance` is `crate::AmountOfSubstance`.

/// Equivalents (`eq`), a unit of `AmountOfSubstance`: `n` of them is `n · 602214076e15` base units.
pub const EQUIVALENTS: Qty<
    { UcumUnit::scaled("eq", Exact::int(602214076000000000000000), UcumUnit::ONE) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// An amount of substance dissolved particles, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type AmountOfSubstanceDissolvedParticles<
    N,
    const U: UcumUnit = { UcumUnit::ONE },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Osmole (`osm`), a unit of `AmountOfSubstanceDissolvedParticles`: `n` of them is `n · 602214076e15` base units.
pub const OSMOLE: Qty<
    { UcumUnit::scaled("osm", Exact::int(602214076000000000000000), UcumUnit::ONE) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// A catalytic activity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type CatalyticActivity<
    N,
    const U: UcumUnit = { UcumUnit::ONE.times(SECOND.unit(), -1) },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Katal (`kat`), a unit of `CatalyticActivity`: `n` of them is `n · 602214076e15` base units.
pub const KATAL: Qty<
    {
        UcumUnit::scaled(
            "kat",
            Exact::int(602214076000000000000000),
            UcumUnit::ONE.times(SECOND.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Unit (`U`), a unit of `CatalyticActivity`: `n` of them is `n · 301107038e8 / 3` base units.
pub const UNIT: Qty<
    {
        UcumUnit::scaled(
            "U",
            Exact::int(30110703800000000) / Exact::int(3),
            UcumUnit::ONE.times(SECOND.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// A fluid resistance, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type FluidResistance<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(METER.unit(), -4)
            .times(SECOND.unit(), -1)
            .times(GRAM.unit(), 1)
    },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Peripheral vascular resistance unit (`[PRU]`), a unit of `FluidResistance`: `n` of them is `n · 133322e6` base units.
pub const PERIPHERAL_VASCULAR_RESISTANCE_UNIT: Qty<
    {
        UcumUnit::scaled(
            "[PRU]",
            Exact::int(133322000000),
            UcumUnit::ONE
                .times(METER.unit(), -4)
                .times(SECOND.unit(), -1)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Wood unit (`[wood'U]`), a unit of `FluidResistance`: `n` of them is `n · 799932e4` base units.
pub const WOOD_UNIT: Qty<
    {
        UcumUnit::scaled(
            "[wood'U]",
            Exact::int(7999320000),
            UcumUnit::ONE
                .times(METER.unit(), -4)
                .times(SECOND.unit(), -1)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// A gauge of catheters, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type GaugeOfCatheters<N, const U: UcumUnit = { METER.unit() }, const B: Base = BASE> =
    Qty<U, N, B>;

/// French (`[Ch]`), a unit of `GaugeOfCatheters`: `n` of them is `n · 1 / 3000` base units.
pub const FRENCH: Qty<
    { UcumUnit::scaled("[Ch]", Exact::int(1) / Exact::int(3000), METER.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `LineicNumber` is `crate::LineicNumber`.

/// Mesh (`[mesh_i]`), a unit of `LineicNumber`: `n` of them is `n · 5e3 / 127` base units.
pub const MESH: Qty<
    {
        UcumUnit::scaled(
            "[mesh_i]",
            Exact::int(5000) / Exact::int(127),
            UcumUnit::ONE.times(METER.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// A mass concentration, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MassConcentration<
    N,
    const U: UcumUnit = { UcumUnit::ONE.times(METER.unit(), -3).times(GRAM.unit(), 1) },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Gram percent (`g%`), a unit of `MassConcentration`: `n` of them is `n · 1e4` base units.
pub const GRAM_PERCENT: Qty<
    {
        UcumUnit::scaled(
            "g%",
            Exact::int(10000),
            UcumUnit::ONE.times(METER.unit(), -3).times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// A metabolic cost of physical activity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MetabolicCostOfPhysicalActivity<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(METER.unit(), 3)
            .times(SECOND.unit(), -1)
            .times(GRAM.unit(), -1)
    },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Metabolic equivalent (`[MET]`), a unit of `MetabolicCostOfPhysicalActivity`: `n` of them is `n · 7 / 120000000000` base units.
pub const METABOLIC_EQUIVALENT: Qty<
    {
        UcumUnit::scaled(
            "[MET]",
            Exact::int(7) / Exact::int(120000000000),
            UcumUnit::ONE
                .times(METER.unit(), 3)
                .times(SECOND.unit(), -1)
                .times(GRAM.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Pressure` is `crate::Pressure`.

/// Meter of water column (`m[H2O]`), a unit of `Pressure`: `n` of them is `n · 980665e1` base units.
pub const METER_OF_WATER_COLUMN: Qty<
    {
        UcumUnit::scaled(
            "m[H2O]",
            Exact::int(9806650),
            UcumUnit::ONE
                .times(METER.unit(), -1)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Meter of mercury column (`m[Hg]`), a unit of `Pressure`: `n` of them is `n · 133322e3` base units.
pub const METER_OF_MERCURY_COLUMN: Qty<
    {
        UcumUnit::scaled(
            "m[Hg]",
            Exact::int(133322000),
            UcumUnit::ONE
                .times(METER.unit(), -1)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Inch of water column (`[in_i'H2O]`), a unit of `Pressure`: `n` of them is `n · 24908891e-2` base units.
pub const INCH_OF_WATER_COLUMN: Qty<
    {
        UcumUnit::scaled(
            "[in_i'H2O]",
            Exact::dec(24908891, -2),
            UcumUnit::ONE
                .times(METER.unit(), -1)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Inch of mercury column (`[in_i'Hg]`), a unit of `Pressure`: `n` of them is `n · 33863788e-1` base units.
pub const INCH_OF_MERCURY_COLUMN: Qty<
    {
        UcumUnit::scaled(
            "[in_i'Hg]",
            Exact::dec(33863788, -1),
            UcumUnit::ONE
                .times(METER.unit(), -1)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// A refraction of a lens, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type RefractionOfALens<
    N,
    const U: UcumUnit = { UcumUnit::ONE.times(METER.unit(), -1) },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Diopter (`[diop]`), a unit of `RefractionOfALens`: the coherent unit, a product of base units.
pub const DIOPTER: Qty<{ UcumUnit::ONE.times(METER.unit(), -1) }, Exact, BASE> = Qty::ONE;

/// A sedimentation coefficient, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type SedimentationCoefficient<N, const U: UcumUnit = { SECOND.unit() }, const B: Base = BASE> =
    Qty<U, N, B>;

/// Svedberg unit (`[S]`), a unit of `SedimentationCoefficient`: `n` of them is `n · 1e-13` base units.
pub const SVEDBERG_UNIT: Qty<
    { UcumUnit::scaled("[S]", Exact::dec(1, -13), SECOND.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// A view area in microscope, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ViewAreaInMicroscope<N, const U: UcumUnit = { UcumUnit::ONE }, const B: Base = BASE> =
    Qty<U, N, B>;

/// High power field (`[HPF]`), a unit of `ViewAreaInMicroscope`: the coherent unit, a product of base units.
pub const HIGH_POWER_FIELD: Qty<{ UcumUnit::ONE }, Exact, BASE> = Qty::ONE;

/// Low power field (`[LPF]`), a unit of `ViewAreaInMicroscope`: `n` of them is `n · 1e2` base units.
pub const LOW_POWER_FIELD: Qty<
    { UcumUnit::scaled("[LPF]", Exact::int(100), UcumUnit::ONE) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Volume` is `crate::Volume`.

/// Drop (`[drp]`), a unit of `Volume`: `n` of them is `n · 5e-8` base units.
pub const DROP: Qty<
    {
        UcumUnit::scaled(
            "[drp]",
            Exact::dec(5, -8),
            UcumUnit::ONE.times(METER.unit(), 3),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// A x ray attenuation, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type XRayAttenuation<N, const U: UcumUnit = { UcumUnit::ONE }, const B: Base = BASE> =
    Qty<U, N, B>;

/// Hounsfield unit (`[hnsf'U]`), a unit of `XRayAttenuation`: the coherent unit, a product of base units.
pub const HOUNSFIELD_UNIT: Qty<{ UcumUnit::ONE }, Exact, BASE> = Qty::ONE;
