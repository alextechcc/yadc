//! The quantities of `chemistry` and their units.

#[allow(unused_imports)]
use crate::*;

/// An amount of substance, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type AmountOfSubstance<N, const U: SIUnit = { MOLE.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Particles (`particle`), a unit of `AmountOfSubstance`: `n` of them is `n · 1.0 / 6.02214076e23` base units.
pub const PARTICLE: SIQuantity<
    {
        SIUnit::scaled(
            "particle",
            Exact::int(1) / Exact::int(602214076000000000000000),
            MOLE.unit(),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// A molar concentration, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MolarConcentration<
    N,
    const U: SIUnit = { SIUnit::ONE.times(METRE.unit(), -3).times(MOLE.unit(), 1) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// A molar energy, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MolarEnergy<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -2)
            .times(MOLE.unit(), -1)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// A molar flux, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MolarFlux<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), -2)
            .times(SECOND.unit(), -1)
            .times(MOLE.unit(), 1)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// A molar heat capacity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MolarHeatCapacity<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -2)
            .times(KELVIN.unit(), -1)
            .times(MOLE.unit(), -1)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Molar gas constants (`R`), a unit of `MolarHeatCapacity`: `n` of them is `n · 8.314462618` base units.
pub const MOLAR_GAS_CONSTANT: SIQuantity<
    {
        SIUnit::scaled(
            "R",
            Exact::dec(8314462618, -9),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2)
                .times(KELVIN.unit(), -1)
                .times(MOLE.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// A molar mass, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MolarMass<
    N,
    const U: SIUnit = { KILOGRAM.unit().times(MOLE.unit(), -1) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// A molar radioactivity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MolarRadioactivity<
    N,
    const U: SIUnit = { SIUnit::ONE.times(SECOND.unit(), -1).times(MOLE.unit(), -1) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// A molar volume, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MolarVolume<
    N,
    const U: SIUnit = { SIUnit::ONE.times(METRE.unit(), 3).times(MOLE.unit(), -1) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// A molality, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Molality<
    N,
    const U: SIUnit = { SIUnit::ONE.times(KILOGRAM.unit(), -1).times(MOLE.unit(), 1) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// A mass concentration, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MassConcentration<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), -3)
            .times(KILOGRAM.unit(), 1)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// A catalytic activity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type CatalyticActivity<
    N,
    const U: SIUnit = { SIUnit::ONE.times(SECOND.unit(), -1).times(MOLE.unit(), 1) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Katals (`kat`), a unit of `CatalyticActivity`: the coherent unit, a product of base units.
pub const KATAL: SIQuantity<
    { SIUnit::ONE.times(SECOND.unit(), -1).times(MOLE.unit(), 1) },
    Exact,
    BASE,
> = SIQuantity::ONE;

/// A catalytic activity concentration, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type CatalyticActivityConcentration<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), -3)
            .times(SECOND.unit(), -1)
            .times(MOLE.unit(), 1)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// A radioactivity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Radioactivity<
    N,
    const U: SIUnit = { SIUnit::ONE.times(SECOND.unit(), -1) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Becquerels (`Bq`), a unit of `Radioactivity`: the coherent unit, a product of base units.
pub const BECQUEREL: SIQuantity<{ SIUnit::ONE.times(SECOND.unit(), -1) }, Exact, BASE> =
    SIQuantity::ONE;

/// Curies (`Ci`), a unit of `Radioactivity`: `n` of them is `n · 37000000000.0` base units.
pub const CURIE: SIQuantity<
    {
        SIUnit::scaled(
            "Ci",
            Exact::int(37000000000),
            SIUnit::ONE.times(SECOND.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// A specific radioactivity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type SpecificRadioactivity<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(KILOGRAM.unit(), -1)
            .times(SECOND.unit(), -1)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// An inverse amount of substance, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type InverseAmountOfSubstance<
    N,
    const U: SIUnit = { SIUnit::ONE.times(MOLE.unit(), -1) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Avogadro constants (`N_A`), a unit of `InverseAmountOfSubstance`: `n` of them is `n · 6.02214076e23` base units.
pub const AVOGADRO_CONSTANT: SIQuantity<
    {
        SIUnit::scaled(
            "N_A",
            Exact::int(602214076000000000000000),
            SIUnit::ONE.times(MOLE.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;
