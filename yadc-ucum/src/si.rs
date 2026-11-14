//! The quantities of `si` and their units.

#[allow(unused_imports)]
use crate::*;

/// An amount of substance, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type AmountOfSubstance<N, const U: UcumUnit = { UcumUnit::ONE }, const IS_BASE: bool = true> =
    Qty<U, N, IS_BASE>;

/// Mole (`mol`), a unit of `AmountOfSubstance`: `n` of them is `n · 602214076e15` base units.
pub const MOLE: Qty<
    { UcumUnit::scaled("mol", Exact::int(602214076000000000000000), UcumUnit::ONE) },
    Exact,
    false,
> = Qty::ONE;

/// A dose equivalent, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type DoseEquivalent<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(METER.unit(), 2)
            .times(SECOND.unit(), -2)
    },
    const IS_BASE: bool = true,
> = Qty<U, N, IS_BASE>;

/// Sievert (`Sv`), a unit of `DoseEquivalent`: the coherent unit, a product of base units.
pub const SIEVERT: Qty<
    {
        UcumUnit::ONE
            .times(METER.unit(), 2)
            .times(SECOND.unit(), -2)
    },
    Exact,
    true,
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
    const IS_BASE: bool = true,
> = Qty<U, N, IS_BASE>;

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
    false,
> = Qty::ONE;

/// An electric charge, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ElectricCharge<N, const U: UcumUnit = { COULOMB.unit() }, const IS_BASE: bool = true> =
    Qty<U, N, IS_BASE>;

/// An electric conductance, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ElectricConductance<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(METER.unit(), -2)
            .times(SECOND.unit(), 1)
            .times(GRAM.unit(), -1)
            .times(COULOMB.unit(), 2)
    },
    const IS_BASE: bool = true,
> = Qty<U, N, IS_BASE>;

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
    false,
> = Qty::ONE;

/// An electric current, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ElectricCurrent<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(SECOND.unit(), -1)
            .times(COULOMB.unit(), 1)
    },
    const IS_BASE: bool = true,
> = Qty<U, N, IS_BASE>;

/// Ampère (`A`), a unit of `ElectricCurrent`: the coherent unit, a product of base units.
pub const AMPERE: Qty<
    {
        UcumUnit::ONE
            .times(SECOND.unit(), -1)
            .times(COULOMB.unit(), 1)
    },
    Exact,
    true,
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
    const IS_BASE: bool = true,
> = Qty<U, N, IS_BASE>;

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
    false,
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
    const IS_BASE: bool = true,
> = Qty<U, N, IS_BASE>;

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
    false,
> = Qty::ONE;

/// An energy, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Energy<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(METER.unit(), 2)
            .times(SECOND.unit(), -2)
            .times(GRAM.unit(), 1)
    },
    const IS_BASE: bool = true,
> = Qty<U, N, IS_BASE>;

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
    false,
> = Qty::ONE;

/// An energy dose, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type EnergyDose<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(METER.unit(), 2)
            .times(SECOND.unit(), -2)
    },
    const IS_BASE: bool = true,
> = Qty<U, N, IS_BASE>;

/// Gray (`Gy`), a unit of `EnergyDose`: the coherent unit, a product of base units.
pub const GRAY: Qty<
    {
        UcumUnit::ONE
            .times(METER.unit(), 2)
            .times(SECOND.unit(), -2)
    },
    Exact,
    true,
> = Qty::ONE;

/// A force, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Force<
    N,
    const U: UcumUnit = { METER.unit().times(SECOND.unit(), -2).times(GRAM.unit(), 1) },
    const IS_BASE: bool = true,
> = Qty<U, N, IS_BASE>;

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
    false,
> = Qty::ONE;

/// A frequency, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Frequency<
    N,
    const U: UcumUnit = { UcumUnit::ONE.times(SECOND.unit(), -1) },
    const IS_BASE: bool = true,
> = Qty<U, N, IS_BASE>;

/// Hertz (`Hz`), a unit of `Frequency`: the coherent unit, a product of base units.
pub const HERTZ: Qty<{ UcumUnit::ONE.times(SECOND.unit(), -1) }, Exact, true> = Qty::ONE;

/// An illuminance, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Illuminance<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(METER.unit(), -2)
            .times(RADIAN.unit(), 2)
            .times(CANDELA.unit(), 1)
    },
    const IS_BASE: bool = true,
> = Qty<U, N, IS_BASE>;

/// Lux (`lx`), a unit of `Illuminance`: the coherent unit, a product of base units.
pub const LUX: Qty<
    {
        UcumUnit::ONE
            .times(METER.unit(), -2)
            .times(RADIAN.unit(), 2)
            .times(CANDELA.unit(), 1)
    },
    Exact,
    true,
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
    const IS_BASE: bool = true,
> = Qty<U, N, IS_BASE>;

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
    false,
> = Qty::ONE;

/// A length, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Length<N, const U: UcumUnit = { METER.unit() }, const IS_BASE: bool = true> =
    Qty<U, N, IS_BASE>;

/// A luminous flux, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type LuminousFlux<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(RADIAN.unit(), 2)
            .times(CANDELA.unit(), 1)
    },
    const IS_BASE: bool = true,
> = Qty<U, N, IS_BASE>;

/// Lumen (`lm`), a unit of `LuminousFlux`: the coherent unit, a product of base units.
pub const LUMEN: Qty<
    {
        UcumUnit::ONE
            .times(RADIAN.unit(), 2)
            .times(CANDELA.unit(), 1)
    },
    Exact,
    true,
> = Qty::ONE;

/// A luminous intensity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type LuminousIntensity<N, const U: UcumUnit = { CANDELA.unit() }, const IS_BASE: bool = true> =
    Qty<U, N, IS_BASE>;

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
    const IS_BASE: bool = true,
> = Qty<U, N, IS_BASE>;

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
    false,
> = Qty::ONE;

/// A magnetic flux density, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MagneticFluxDensity<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(SECOND.unit(), -1)
            .times(GRAM.unit(), 1)
            .times(COULOMB.unit(), -1)
    },
    const IS_BASE: bool = true,
> = Qty<U, N, IS_BASE>;

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
    false,
> = Qty::ONE;

/// A mass, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Mass<N, const U: UcumUnit = { GRAM.unit() }, const IS_BASE: bool = true> =
    Qty<U, N, IS_BASE>;

/// A plane angle, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type PlaneAngle<N, const U: UcumUnit = { RADIAN.unit() }, const IS_BASE: bool = true> =
    Qty<U, N, IS_BASE>;

/// A power, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Power<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(METER.unit(), 2)
            .times(SECOND.unit(), -3)
            .times(GRAM.unit(), 1)
    },
    const IS_BASE: bool = true,
> = Qty<U, N, IS_BASE>;

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
    false,
> = Qty::ONE;

/// A pressure, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Pressure<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(METER.unit(), -1)
            .times(SECOND.unit(), -2)
            .times(GRAM.unit(), 1)
    },
    const IS_BASE: bool = true,
> = Qty<U, N, IS_BASE>;

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
    false,
> = Qty::ONE;

/// A radioactivity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Radioactivity<
    N,
    const U: UcumUnit = { UcumUnit::ONE.times(SECOND.unit(), -1) },
    const IS_BASE: bool = true,
> = Qty<U, N, IS_BASE>;

/// Becquerel (`Bq`), a unit of `Radioactivity`: the coherent unit, a product of base units.
pub const BECQUEREL: Qty<{ UcumUnit::ONE.times(SECOND.unit(), -1) }, Exact, true> = Qty::ONE;

/// A solid angle, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type SolidAngle<
    N,
    const U: UcumUnit = { UcumUnit::ONE.times(RADIAN.unit(), 2) },
    const IS_BASE: bool = true,
> = Qty<U, N, IS_BASE>;

/// Steradian (`sr`), a unit of `SolidAngle`: the coherent unit, a product of base units.
pub const STERADIAN: Qty<{ UcumUnit::ONE.times(RADIAN.unit(), 2) }, Exact, true> = Qty::ONE;

/// A time, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Time<N, const U: UcumUnit = { SECOND.unit() }, const IS_BASE: bool = true> =
    Qty<U, N, IS_BASE>;
