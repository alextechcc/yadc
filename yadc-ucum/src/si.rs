//! The quantities of `si` and their units.

#[allow(unused_imports)]
use crate::*;

// `AmountOfSubstance` is `crate::AmountOfSubstance`.

/// Mole (`mol`), a unit of `AmountOfSubstance`: `n` of them is `n · 602214076e15` base units.
pub const MOLE: Unit<
    { UcumUnit::scaled("mol", Exact::int(602214076000000000000000), ONE.unit()) },
> = Unit::ONE;

// `DoseEquivalent` is `crate::DoseEquivalent`.

/// Sievert (`Sv`), a unit of `DoseEquivalent`: the coherent unit, a product of base units.
pub const SIEVERT: Unit<{ (METER * METER / (SECOND * SECOND)).unit() }, BASE> = Unit::ONE;

/// An electric capacitance, in `N`s of the unit `U`: by default its base unit, `SECOND_SQUARED_COULOMB_SQUARED_PER_METER_SQUARED_GRAM`.
pub type ElectricCapacitance<
    N,
    const U: UcumUnit = { SECOND_SQUARED_COULOMB_SQUARED_PER_METER_SQUARED_GRAM.unit() },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// The base unit of `ElectricCapacitance`: a product of base units.
pub const SECOND_SQUARED_COULOMB_SQUARED_PER_METER_SQUARED_GRAM: Unit<
    { (SECOND * SECOND * COULOMB * COULOMB / (METER * METER * GRAM)).unit() },
    BASE,
> = Unit::ONE;

/// Farad (`F`), a unit of `ElectricCapacitance`: `n` of them is `n · 1e-3` base units.
pub const FARAD: Unit<
    {
        UcumUnit::scaled(
            "F",
            Exact::dec(1, -3),
            SECOND_SQUARED_COULOMB_SQUARED_PER_METER_SQUARED_GRAM.unit(),
        )
    },
> = Unit::ONE;

// `ElectricCharge` is `crate::ElectricCharge`.

// `ElectricConductance` is `crate::ElectricConductance`.

/// Siemens (`S`), a unit of `ElectricConductance`: `n` of them is `n · 1e-3` base units.
pub const SIEMENS: Unit<
    {
        UcumUnit::scaled(
            "S",
            Exact::dec(1, -3),
            SECOND_COULOMB_SQUARED_PER_METER_SQUARED_GRAM.unit(),
        )
    },
> = Unit::ONE;

// `ElectricCurrent` is `crate::ElectricCurrent`.

/// Ampère (`A`), a unit of `ElectricCurrent`: the coherent unit, a product of base units.
pub const AMPERE: Unit<{ (COULOMB / SECOND).unit() }, BASE> = Unit::ONE;

/// An electric potential, in `N`s of the unit `U`: by default its base unit, `METER_SQUARED_GRAM_PER_SECOND_SQUARED_COULOMB`.
pub type ElectricPotential<
    N,
    const U: UcumUnit = { METER_SQUARED_GRAM_PER_SECOND_SQUARED_COULOMB.unit() },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// The base unit of `ElectricPotential`: a product of base units.
pub const METER_SQUARED_GRAM_PER_SECOND_SQUARED_COULOMB: Unit<
    { (METER * METER * GRAM / (SECOND * SECOND * COULOMB)).unit() },
    BASE,
> = Unit::ONE;

/// Volt (`V`), a unit of `ElectricPotential`: `n` of them is `n · 1e3` base units.
pub const VOLT: Unit<
    {
        UcumUnit::scaled(
            "V",
            Exact::int(1000),
            METER_SQUARED_GRAM_PER_SECOND_SQUARED_COULOMB.unit(),
        )
    },
> = Unit::ONE;

/// An electric resistance, in `N`s of the unit `U`: by default its base unit, `METER_SQUARED_GRAM_PER_SECOND_COULOMB_SQUARED`.
pub type ElectricResistance<
    N,
    const U: UcumUnit = { METER_SQUARED_GRAM_PER_SECOND_COULOMB_SQUARED.unit() },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// The base unit of `ElectricResistance`: a product of base units.
pub const METER_SQUARED_GRAM_PER_SECOND_COULOMB_SQUARED: Unit<
    { (METER * METER * GRAM / (SECOND * COULOMB * COULOMB)).unit() },
    BASE,
> = Unit::ONE;

/// Ohm (`Ohm`), a unit of `ElectricResistance`: `n` of them is `n · 1e3` base units.
pub const OHM: Unit<
    {
        UcumUnit::scaled(
            "Ohm",
            Exact::int(1000),
            METER_SQUARED_GRAM_PER_SECOND_COULOMB_SQUARED.unit(),
        )
    },
> = Unit::ONE;

// `Energy` is `crate::Energy`.

/// Joule (`J`), a unit of `Energy`: `n` of them is `n · 1e3` base units.
pub const JOULE: Unit<
    {
        UcumUnit::scaled(
            "J",
            Exact::int(1000),
            METER_SQUARED_GRAM_PER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

// `EnergyDose` is `crate::EnergyDose`.

/// Gray (`Gy`), a unit of `EnergyDose`: the coherent unit, a product of base units.
pub const GRAY: Unit<{ (METER * METER / (SECOND * SECOND)).unit() }, BASE> = Unit::ONE;

// `Force` is `crate::Force`.

/// Newton (`N`), a unit of `Force`: `n` of them is `n · 1e3` base units.
pub const NEWTON: Unit<
    { UcumUnit::scaled("N", Exact::int(1000), METER_GRAM_PER_SECOND_SQUARED.unit()) },
> = Unit::ONE;

/// A frequency, in `N`s of the unit `U`: by default its base unit, `HERTZ`.
pub type Frequency<N, const U: UcumUnit = { HERTZ.unit() }, const B: Base = BASE> = Qty<U, N, B>;

/// Hertz (`Hz`), a unit of `Frequency`: the coherent unit, a product of base units.
pub const HERTZ: Unit<{ UcumUnit::ONE.times(SECOND.unit(), -1) }, BASE> = Unit::ONE;

// `Illuminance` is `crate::Illuminance`.

/// Lux (`lx`), a unit of `Illuminance`: the coherent unit, a product of base units.
pub const LUX: Unit<{ (RADIAN * RADIAN * CANDELA / (METER * METER)).unit() }, BASE> = Unit::ONE;

/// An inductance, in `N`s of the unit `U`: by default its base unit, `METER_SQUARED_GRAM_PER_COULOMB_SQUARED`.
pub type Inductance<
    N,
    const U: UcumUnit = { METER_SQUARED_GRAM_PER_COULOMB_SQUARED.unit() },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// The base unit of `Inductance`: a product of base units.
pub const METER_SQUARED_GRAM_PER_COULOMB_SQUARED: Unit<
    { (METER * METER * GRAM / (COULOMB * COULOMB)).unit() },
    BASE,
> = Unit::ONE;

/// Henry (`H`), a unit of `Inductance`: `n` of them is `n · 1e3` base units.
pub const HENRY: Unit<
    {
        UcumUnit::scaled(
            "H",
            Exact::int(1000),
            METER_SQUARED_GRAM_PER_COULOMB_SQUARED.unit(),
        )
    },
> = Unit::ONE;

// `Length` is `crate::Length`.

/// A luminous flux, in `N`s of the unit `U`: by default its base unit, `LUMEN`.
pub type LuminousFlux<N, const U: UcumUnit = { LUMEN.unit() }, const B: Base = BASE> = Qty<U, N, B>;

/// Lumen (`lm`), a unit of `LuminousFlux`: the coherent unit, a product of base units.
pub const LUMEN: Unit<{ (RADIAN * RADIAN * CANDELA).unit() }, BASE> = Unit::ONE;

/// A luminous intensity, in `N`s of the unit `U`: by default its base unit, `CANDELA`.
pub type LuminousIntensity<N, const U: UcumUnit = { CANDELA.unit() }, const B: Base = BASE> =
    Qty<U, N, B>;

/// A magnetic flux, in `N`s of the unit `U`: by default its base unit, `METER_SQUARED_GRAM_PER_SECOND_COULOMB`.
pub type MagneticFlux<
    N,
    const U: UcumUnit = { METER_SQUARED_GRAM_PER_SECOND_COULOMB.unit() },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// The base unit of `MagneticFlux`: a product of base units.
pub const METER_SQUARED_GRAM_PER_SECOND_COULOMB: Unit<
    { (METER * METER * GRAM / (SECOND * COULOMB)).unit() },
    BASE,
> = Unit::ONE;

/// Weber (`Wb`), a unit of `MagneticFlux`: `n` of them is `n · 1e3` base units.
pub const WEBER: Unit<
    {
        UcumUnit::scaled(
            "Wb",
            Exact::int(1000),
            METER_SQUARED_GRAM_PER_SECOND_COULOMB.unit(),
        )
    },
> = Unit::ONE;

// `MagneticFluxDensity` is `crate::MagneticFluxDensity`.

/// Tesla (`T`), a unit of `MagneticFluxDensity`: `n` of them is `n · 1e3` base units.
pub const TESLA: Unit<{ UcumUnit::scaled("T", Exact::int(1000), GRAM_PER_SECOND_COULOMB.unit()) }> =
    Unit::ONE;

// `Mass` is `crate::Mass`.

// `PlaneAngle` is `crate::PlaneAngle`.

// `Power` is `crate::Power`.

/// Watt (`W`), a unit of `Power`: `n` of them is `n · 1e3` base units.
pub const WATT: Unit<
    {
        UcumUnit::scaled(
            "W",
            Exact::int(1000),
            METER_SQUARED_GRAM_PER_SECOND_CUBED.unit(),
        )
    },
> = Unit::ONE;

// `Pressure` is `crate::Pressure`.

/// Pascal (`Pa`), a unit of `Pressure`: `n` of them is `n · 1e3` base units.
pub const PASCAL: Unit<
    { UcumUnit::scaled("Pa", Exact::int(1000), GRAM_PER_METER_SECOND_SQUARED.unit()) },
> = Unit::ONE;

// `Radioactivity` is `crate::Radioactivity`.

/// Becquerel (`Bq`), a unit of `Radioactivity`: the coherent unit, a product of base units.
pub const BECQUEREL: Unit<{ UcumUnit::ONE.times(SECOND.unit(), -1) }, BASE> = Unit::ONE;

// `SolidAngle` is `crate::SolidAngle`.

/// Steradian (`sr`), a unit of `SolidAngle`: the coherent unit, a product of base units.
pub const STERADIAN: Unit<{ (RADIAN * RADIAN).unit() }, BASE> = Unit::ONE;

// `Time` is `crate::Time`.
