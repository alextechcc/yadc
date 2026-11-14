//! The quantities of `si` and their units.

#[allow(unused_imports)]
use crate::*;

// `AmountOfSubstance` is `crate::AmountOfSubstance`.

/// Mole (`mol`), a unit of `AmountOfSubstance`: `n` of them is `n · 602214076e15` base units.
pub const MOLE: Qty<
    { UcumUnit::scaled("mol", Exact::int(602214076000000000000000), UcumUnit::ONE) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `DoseEquivalent` is `crate::DoseEquivalent`.

/// Sievert (`Sv`), a unit of `DoseEquivalent`: the coherent unit, a product of base units.
pub const SIEVERT: Qty<
    {
        UcumUnit::ONE
            .times(METER.unit(), 2)
            .times(SECOND.unit(), -2)
    },
    Exact,
    BASE,
> = Qty::ONE;

/// An electric capacitance, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ElectricCapacitance<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(METER.unit(), -2)
            .times(SECOND.unit(), 2)
            .times(GRAM.unit(), -1)
            .times(COULOMB.unit(), 2)
    },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Farad (`F`), a unit of `ElectricCapacitance`: `n` of them is `n · 1e-3` base units.
pub const FARAD: Qty<
    {
        UcumUnit::scaled(
            "F",
            Exact::dec(1, -3),
            UcumUnit::ONE
                .times(METER.unit(), -2)
                .times(SECOND.unit(), 2)
                .times(GRAM.unit(), -1)
                .times(COULOMB.unit(), 2),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `ElectricCharge` is `crate::ElectricCharge`.

// `ElectricConductance` is `crate::ElectricConductance`.

/// Siemens (`S`), a unit of `ElectricConductance`: `n` of them is `n · 1e-3` base units.
pub const SIEMENS: Qty<
    {
        UcumUnit::scaled(
            "S",
            Exact::dec(1, -3),
            UcumUnit::ONE
                .times(METER.unit(), -2)
                .times(SECOND.unit(), 1)
                .times(GRAM.unit(), -1)
                .times(COULOMB.unit(), 2),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `ElectricCurrent` is `crate::ElectricCurrent`.

/// Ampère (`A`), a unit of `ElectricCurrent`: the coherent unit, a product of base units.
pub const AMPERE: Qty<
    {
        UcumUnit::ONE
            .times(SECOND.unit(), -1)
            .times(COULOMB.unit(), 1)
    },
    Exact,
    BASE,
> = Qty::ONE;

/// An electric potential, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ElectricPotential<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(METER.unit(), 2)
            .times(SECOND.unit(), -2)
            .times(GRAM.unit(), 1)
            .times(COULOMB.unit(), -1)
    },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Volt (`V`), a unit of `ElectricPotential`: `n` of them is `n · 1e3` base units.
pub const VOLT: Qty<
    {
        UcumUnit::scaled(
            "V",
            Exact::int(1000),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1)
                .times(COULOMB.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// An electric resistance, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ElectricResistance<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(METER.unit(), 2)
            .times(SECOND.unit(), -1)
            .times(GRAM.unit(), 1)
            .times(COULOMB.unit(), -2)
    },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Ohm (`Ohm`), a unit of `ElectricResistance`: `n` of them is `n · 1e3` base units.
pub const OHM: Qty<
    {
        UcumUnit::scaled(
            "Ohm",
            Exact::int(1000),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -1)
                .times(GRAM.unit(), 1)
                .times(COULOMB.unit(), -2),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Energy` is `crate::Energy`.

/// Joule (`J`), a unit of `Energy`: `n` of them is `n · 1e3` base units.
pub const JOULE: Qty<
    {
        UcumUnit::scaled(
            "J",
            Exact::int(1000),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `EnergyDose` is `crate::EnergyDose`.

/// Gray (`Gy`), a unit of `EnergyDose`: the coherent unit, a product of base units.
pub const GRAY: Qty<
    {
        UcumUnit::ONE
            .times(METER.unit(), 2)
            .times(SECOND.unit(), -2)
    },
    Exact,
    BASE,
> = Qty::ONE;

// `Force` is `crate::Force`.

/// Newton (`N`), a unit of `Force`: `n` of them is `n · 1e3` base units.
pub const NEWTON: Qty<
    {
        UcumUnit::scaled(
            "N",
            Exact::int(1000),
            METER.unit().times(SECOND.unit(), -2).times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// A frequency, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Frequency<
    N,
    const U: UcumUnit = { UcumUnit::ONE.times(SECOND.unit(), -1) },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Hertz (`Hz`), a unit of `Frequency`: the coherent unit, a product of base units.
pub const HERTZ: Qty<{ UcumUnit::ONE.times(SECOND.unit(), -1) }, Exact, BASE> = Qty::ONE;

// `Illuminance` is `crate::Illuminance`.

/// Lux (`lx`), a unit of `Illuminance`: the coherent unit, a product of base units.
pub const LUX: Qty<
    {
        UcumUnit::ONE
            .times(METER.unit(), -2)
            .times(RADIAN.unit(), 2)
            .times(CANDELA.unit(), 1)
    },
    Exact,
    BASE,
> = Qty::ONE;

/// An inductance, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Inductance<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(METER.unit(), 2)
            .times(GRAM.unit(), 1)
            .times(COULOMB.unit(), -2)
    },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Henry (`H`), a unit of `Inductance`: `n` of them is `n · 1e3` base units.
pub const HENRY: Qty<
    {
        UcumUnit::scaled(
            "H",
            Exact::int(1000),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(GRAM.unit(), 1)
                .times(COULOMB.unit(), -2),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Length` is `crate::Length`.

/// A luminous flux, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type LuminousFlux<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(RADIAN.unit(), 2)
            .times(CANDELA.unit(), 1)
    },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Lumen (`lm`), a unit of `LuminousFlux`: the coherent unit, a product of base units.
pub const LUMEN: Qty<
    {
        UcumUnit::ONE
            .times(RADIAN.unit(), 2)
            .times(CANDELA.unit(), 1)
    },
    Exact,
    BASE,
> = Qty::ONE;

/// A luminous intensity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type LuminousIntensity<N, const U: UcumUnit = { CANDELA.unit() }, const B: Base = BASE> =
    Qty<U, N, B>;

/// A magnetic flux, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MagneticFlux<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(METER.unit(), 2)
            .times(SECOND.unit(), -1)
            .times(GRAM.unit(), 1)
            .times(COULOMB.unit(), -1)
    },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Weber (`Wb`), a unit of `MagneticFlux`: `n` of them is `n · 1e3` base units.
pub const WEBER: Qty<
    {
        UcumUnit::scaled(
            "Wb",
            Exact::int(1000),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -1)
                .times(GRAM.unit(), 1)
                .times(COULOMB.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `MagneticFluxDensity` is `crate::MagneticFluxDensity`.

/// Tesla (`T`), a unit of `MagneticFluxDensity`: `n` of them is `n · 1e3` base units.
pub const TESLA: Qty<
    {
        UcumUnit::scaled(
            "T",
            Exact::int(1000),
            UcumUnit::ONE
                .times(SECOND.unit(), -1)
                .times(GRAM.unit(), 1)
                .times(COULOMB.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Mass` is `crate::Mass`.

// `PlaneAngle` is `crate::PlaneAngle`.

// `Power` is `crate::Power`.

/// Watt (`W`), a unit of `Power`: `n` of them is `n · 1e3` base units.
pub const WATT: Qty<
    {
        UcumUnit::scaled(
            "W",
            Exact::int(1000),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -3)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Pressure` is `crate::Pressure`.

/// Pascal (`Pa`), a unit of `Pressure`: `n` of them is `n · 1e3` base units.
pub const PASCAL: Qty<
    {
        UcumUnit::scaled(
            "Pa",
            Exact::int(1000),
            UcumUnit::ONE
                .times(METER.unit(), -1)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Radioactivity` is `crate::Radioactivity`.

/// Becquerel (`Bq`), a unit of `Radioactivity`: the coherent unit, a product of base units.
pub const BECQUEREL: Qty<{ UcumUnit::ONE.times(SECOND.unit(), -1) }, Exact, BASE> = Qty::ONE;

// `SolidAngle` is `crate::SolidAngle`.

/// Steradian (`sr`), a unit of `SolidAngle`: the coherent unit, a product of base units.
pub const STERADIAN: Qty<{ UcumUnit::ONE.times(RADIAN.unit(), 2) }, Exact, BASE> = Qty::ONE;

// `Time` is `crate::Time`.
