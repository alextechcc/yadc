//! The quantities of `chemistry` and their units.

#[allow(unused_imports)]
use crate::*;

/// An amount of substance, in `N`s of the unit `U`: by default its base unit, `MOLE`.
pub type AmountOfSubstance<N, const U: SIUnit = { MOLE.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Particles (`particle`), a unit of `AmountOfSubstance`: `n` of them is `n · 1.0 / 6.02214076e23` base units.
pub const PARTICLE: Unit<
    {
        SIUnit::scaled(
            "particle",
            Exact::int(1) / Exact::int(602214076000000000000000),
            MOLE.unit(),
        )
    },
> = Unit::ONE;

/// A molar concentration, in `N`s of the unit `U`: by default its base unit, `MOLE_PER_METRE_CUBED`.
pub type MolarConcentration<
    N,
    const U: SIUnit = { MOLE_PER_METRE_CUBED.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `MolarConcentration`: a product of base units.
pub const MOLE_PER_METRE_CUBED: Unit<{ (MOLE / (METRE * METRE * METRE)).unit() }, BASE> = Unit::ONE;

/// A molar energy, in `N`s of the unit `U`: by default its base unit, `METRE_SQUARED_KILOGRAM_PER_SECOND_SQUARED_MOLE`.
pub type MolarEnergy<
    N,
    const U: SIUnit = { METRE_SQUARED_KILOGRAM_PER_SECOND_SQUARED_MOLE.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `MolarEnergy`: a product of base units.
pub const METRE_SQUARED_KILOGRAM_PER_SECOND_SQUARED_MOLE: Unit<
    { (METRE * METRE * KILOGRAM / (SECOND * SECOND * MOLE)).unit() },
    BASE,
> = Unit::ONE;

/// A molar flux, in `N`s of the unit `U`: by default its base unit, `MOLE_PER_METRE_SQUARED_SECOND`.
pub type MolarFlux<
    N,
    const U: SIUnit = { MOLE_PER_METRE_SQUARED_SECOND.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `MolarFlux`: a product of base units.
pub const MOLE_PER_METRE_SQUARED_SECOND: Unit<{ (MOLE / (METRE * METRE * SECOND)).unit() }, BASE> =
    Unit::ONE;

/// A molar heat capacity, in `N`s of the unit `U`: by default its base unit, `METRE_SQUARED_KILOGRAM_PER_SECOND_SQUARED_KELVIN_MOLE`.
pub type MolarHeatCapacity<
    N,
    const U: SIUnit = { METRE_SQUARED_KILOGRAM_PER_SECOND_SQUARED_KELVIN_MOLE.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `MolarHeatCapacity`: a product of base units.
pub const METRE_SQUARED_KILOGRAM_PER_SECOND_SQUARED_KELVIN_MOLE: Unit<
    { (METRE * METRE * KILOGRAM / (SECOND * SECOND * KELVIN * MOLE)).unit() },
    BASE,
> = Unit::ONE;

/// Molar gas constants (`R`), a unit of `MolarHeatCapacity`: `n` of them is `n · 8.314462618` base units.
pub const MOLAR_GAS_CONSTANT: Unit<
    {
        SIUnit::scaled(
            "R",
            Exact::dec(8314462618, -9),
            METRE_SQUARED_KILOGRAM_PER_SECOND_SQUARED_KELVIN_MOLE.unit(),
        )
    },
> = Unit::ONE;

/// A molar mass, in `N`s of the unit `U`: by default its base unit, `KILOGRAM_PER_MOLE`.
pub type MolarMass<N, const U: SIUnit = { KILOGRAM_PER_MOLE.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// The base unit of `MolarMass`: a product of base units.
pub const KILOGRAM_PER_MOLE: Unit<{ (KILOGRAM / MOLE).unit() }, BASE> = Unit::ONE;

/// A molar radioactivity, in `N`s of the unit `U`: by default its base unit, `PER_SECOND_MOLE`.
pub type MolarRadioactivity<N, const U: SIUnit = { PER_SECOND_MOLE.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// The base unit of `MolarRadioactivity`: a product of base units.
pub const PER_SECOND_MOLE: Unit<
    { SIUnit::ONE.times(SECOND.unit(), -1).times(MOLE.unit(), -1) },
    BASE,
> = Unit::ONE;

/// A molar volume, in `N`s of the unit `U`: by default its base unit, `METRE_CUBED_PER_MOLE`.
pub type MolarVolume<N, const U: SIUnit = { METRE_CUBED_PER_MOLE.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// The base unit of `MolarVolume`: a product of base units.
pub const METRE_CUBED_PER_MOLE: Unit<{ (METRE * METRE * METRE / MOLE).unit() }, BASE> = Unit::ONE;

/// A molality, in `N`s of the unit `U`: by default its base unit, `MOLE_PER_KILOGRAM`.
pub type Molality<N, const U: SIUnit = { MOLE_PER_KILOGRAM.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// The base unit of `Molality`: a product of base units.
pub const MOLE_PER_KILOGRAM: Unit<{ (MOLE / KILOGRAM).unit() }, BASE> = Unit::ONE;

/// A mass concentration, in `N`s of the unit `U`: by default its base unit, `KILOGRAM_PER_METRE_CUBED`.
pub type MassConcentration<
    N,
    const U: SIUnit = { KILOGRAM_PER_METRE_CUBED.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `MassConcentration`: a product of base units.
pub const KILOGRAM_PER_METRE_CUBED: Unit<{ (KILOGRAM / (METRE * METRE * METRE)).unit() }, BASE> =
    Unit::ONE;

/// A catalytic activity, in `N`s of the unit `U`: by default its base unit, `KATAL`.
pub type CatalyticActivity<N, const U: SIUnit = { KATAL.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Katals (`kat`), a unit of `CatalyticActivity`: the coherent unit, a product of base units.
pub const KATAL: Unit<{ (MOLE / SECOND).unit() }, BASE> = Unit::ONE;

/// A catalytic activity concentration, in `N`s of the unit `U`: by default its base unit, `MOLE_PER_METRE_CUBED_SECOND`.
pub type CatalyticActivityConcentration<
    N,
    const U: SIUnit = { MOLE_PER_METRE_CUBED_SECOND.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `CatalyticActivityConcentration`: a product of base units.
pub const MOLE_PER_METRE_CUBED_SECOND: Unit<
    { (MOLE / (METRE * METRE * METRE * SECOND)).unit() },
    BASE,
> = Unit::ONE;

/// A radioactivity, in `N`s of the unit `U`: by default its base unit, `BECQUEREL`.
pub type Radioactivity<N, const U: SIUnit = { BECQUEREL.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Becquerels (`Bq`), a unit of `Radioactivity`: the coherent unit, a product of base units.
pub const BECQUEREL: Unit<{ SIUnit::ONE.times(SECOND.unit(), -1) }, BASE> = Unit::ONE;

/// Curies (`Ci`), a unit of `Radioactivity`: `n` of them is `n · 37000000000.0` base units.
pub const CURIE: Unit<{ SIUnit::scaled("Ci", Exact::int(37000000000), BECQUEREL.unit()) }> =
    Unit::ONE;

/// A specific radioactivity, in `N`s of the unit `U`: by default its base unit, `PER_KILOGRAM_SECOND`.
pub type SpecificRadioactivity<
    N,
    const U: SIUnit = { PER_KILOGRAM_SECOND.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `SpecificRadioactivity`: a product of base units.
pub const PER_KILOGRAM_SECOND: Unit<
    {
        SIUnit::ONE
            .times(KILOGRAM.unit(), -1)
            .times(SECOND.unit(), -1)
    },
    BASE,
> = Unit::ONE;

/// An inverse amount of substance, in `N`s of the unit `U`: by default its base unit, `PER_MOLE`.
pub type InverseAmountOfSubstance<N, const U: SIUnit = { PER_MOLE.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// The base unit of `InverseAmountOfSubstance`: a product of base units.
pub const PER_MOLE: Unit<{ SIUnit::ONE.times(MOLE.unit(), -1) }, BASE> = Unit::ONE;

/// Avogadro constants (`N_A`), a unit of `InverseAmountOfSubstance`: `n` of them is `n · 6.02214076e23` base units.
pub const AVOGADRO_CONSTANT: Unit<
    { SIUnit::scaled("N_A", Exact::int(602214076000000000000000), PER_MOLE.unit()) },
> = Unit::ONE;
