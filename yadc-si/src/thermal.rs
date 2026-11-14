//! The quantities of `thermal` and their units.

#[allow(unused_imports)]
use crate::*;

/// A thermodynamic temperature, in `N`s of the unit `U`: by default its base unit, `KELVIN`.
pub type ThermodynamicTemperature<N, const U: SIUnit = { KELVIN.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Degrees Celsius (`°C`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 1.0 + 273.15` base units.
pub const CELSIUS: Unit<
    { SIUnit::offset("°C", Exact::int(1), Exact::dec(27315, -2), KELVIN.unit()) },
> = Unit::ONE;

/// Degrees Fahrenheit (`°F`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 5.0 / 9.0 + 273.15 - 32.0 * 5.0 / 9.0` base units.
pub const FAHRENHEIT: Unit<
    {
        SIUnit::offset(
            "°F",
            Exact::int(5) / Exact::int(9),
            Exact::dec(27315, -2) - Exact::int(32) * Exact::int(5) / Exact::int(9),
            KELVIN.unit(),
        )
    },
> = Unit::ONE;

/// Degrees Rankine (`°R`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 5.0 / 9.0` base units.
pub const RANKINE: Unit<{ SIUnit::scaled("°R", Exact::int(5) / Exact::int(9), KELVIN.unit()) }> =
    Unit::ONE;

/// A temperature interval, in `N`s of the unit `U`: by default its base unit, `KELVIN`.
pub type TemperatureInterval<N, const U: SIUnit = { KELVIN.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Degrees Celsius (`°C`), a unit of `TemperatureInterval`: `n` of them is `n · 1.0` base units.
pub const DEGREE_CELSIUS: Unit<{ SIUnit::scaled("°C", Exact::int(1), KELVIN.unit()) }> = Unit::ONE;

/// Degrees Fahrenheit (`°F`), a unit of `TemperatureInterval`: `n` of them is `n · 5.0 / 9.0` base units.
pub const DEGREE_FAHRENHEIT: Unit<
    { SIUnit::scaled("°F", Exact::int(5) / Exact::int(9), KELVIN.unit()) },
> = Unit::ONE;

/// Degrees Rankine (`°R`), a unit of `TemperatureInterval`: `n` of them is `n · 5.0 / 9.0` base units.
pub const DEGREE_RANKINE: Unit<
    { SIUnit::scaled("°R", Exact::int(5) / Exact::int(9), KELVIN.unit()) },
> = Unit::ONE;

/// A temperature coefficient, in `N`s of the unit `U`: by default its base unit, `PER_KELVIN`.
pub type TemperatureCoefficient<N, const U: SIUnit = { PER_KELVIN.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// The base unit of `TemperatureCoefficient`: a product of base units.
pub const PER_KELVIN: Unit<{ SIUnit::ONE.times(KELVIN.unit(), -1) }, BASE> = Unit::ONE;

/// A temperature gradient, in `N`s of the unit `U`: by default its base unit, `KELVIN_PER_METRE`.
pub type TemperatureGradient<
    N,
    const U: SIUnit = { KELVIN_PER_METRE.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `TemperatureGradient`: a product of base units.
pub const KELVIN_PER_METRE: Unit<{ (KELVIN / METRE).unit() }, BASE> = Unit::ONE;

/// A heat capacity, in `N`s of the unit `U`: by default its base unit, `METRE_SQUARED_KILOGRAM_PER_SECOND_SQUARED_KELVIN`.
pub type HeatCapacity<
    N,
    const U: SIUnit = { METRE_SQUARED_KILOGRAM_PER_SECOND_SQUARED_KELVIN.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `HeatCapacity`: a product of base units.
pub const METRE_SQUARED_KILOGRAM_PER_SECOND_SQUARED_KELVIN: Unit<
    { (METRE * METRE * KILOGRAM / (SECOND * SECOND * KELVIN)).unit() },
    BASE,
> = Unit::ONE;

/// Boltzmann constants (`k`), a unit of `HeatCapacity`: `n` of them is `n · 1.380649e-23` base units.
pub const BOLTZMANN_CONSTANT: Unit<
    {
        SIUnit::scaled(
            "k",
            Exact::dec(1380649, -29),
            METRE_SQUARED_KILOGRAM_PER_SECOND_SQUARED_KELVIN.unit(),
        )
    },
> = Unit::ONE;

/// A heat flux density, in `N`s of the unit `U`: by default its base unit, `KILOGRAM_PER_SECOND_CUBED`.
pub type HeatFluxDensity<
    N,
    const U: SIUnit = { KILOGRAM_PER_SECOND_CUBED.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `HeatFluxDensity`: a product of base units.
pub const KILOGRAM_PER_SECOND_CUBED: Unit<
    { (KILOGRAM / (SECOND * SECOND * SECOND)).unit() },
    BASE,
> = Unit::ONE;

/// A heat transfer, in `N`s of the unit `U`: by default its base unit, `KILOGRAM_PER_SECOND_CUBED_KELVIN`.
pub type HeatTransfer<
    N,
    const U: SIUnit = { KILOGRAM_PER_SECOND_CUBED_KELVIN.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `HeatTransfer`: a product of base units.
pub const KILOGRAM_PER_SECOND_CUBED_KELVIN: Unit<
    { (KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)).unit() },
    BASE,
> = Unit::ONE;

/// A thermal conductance, in `N`s of the unit `U`: by default its base unit, `METRE_SQUARED_KILOGRAM_PER_SECOND_CUBED_KELVIN`.
pub type ThermalConductance<
    N,
    const U: SIUnit = { METRE_SQUARED_KILOGRAM_PER_SECOND_CUBED_KELVIN.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `ThermalConductance`: a product of base units.
pub const METRE_SQUARED_KILOGRAM_PER_SECOND_CUBED_KELVIN: Unit<
    { (METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)).unit() },
    BASE,
> = Unit::ONE;

/// A thermal conductivity, in `N`s of the unit `U`: by default its base unit, `METRE_KILOGRAM_PER_SECOND_CUBED_KELVIN`.
pub type ThermalConductivity<
    N,
    const U: SIUnit = { METRE_KILOGRAM_PER_SECOND_CUBED_KELVIN.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `ThermalConductivity`: a product of base units.
pub const METRE_KILOGRAM_PER_SECOND_CUBED_KELVIN: Unit<
    { (METRE * KILOGRAM / (SECOND * SECOND * SECOND * KELVIN)).unit() },
    BASE,
> = Unit::ONE;

/// A thermal resistance, in `N`s of the unit `U`: by default its base unit, `SECOND_CUBED_KELVIN_PER_METRE_SQUARED_KILOGRAM`.
pub type ThermalResistance<
    N,
    const U: SIUnit = { SECOND_CUBED_KELVIN_PER_METRE_SQUARED_KILOGRAM.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `ThermalResistance`: a product of base units.
pub const SECOND_CUBED_KELVIN_PER_METRE_SQUARED_KILOGRAM: Unit<
    { (SECOND * SECOND * SECOND * KELVIN / (METRE * METRE * KILOGRAM)).unit() },
    BASE,
> = Unit::ONE;

/// A specific heat capacity, in `N`s of the unit `U`: by default its base unit, `METRE_SQUARED_PER_SECOND_SQUARED_KELVIN`.
pub type SpecificHeatCapacity<
    N,
    const U: SIUnit = { METRE_SQUARED_PER_SECOND_SQUARED_KELVIN.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `SpecificHeatCapacity`: a product of base units.
pub const METRE_SQUARED_PER_SECOND_SQUARED_KELVIN: Unit<
    { (METRE * METRE / (SECOND * SECOND * KELVIN)).unit() },
    BASE,
> = Unit::ONE;

/// An areal heat capacity, in `N`s of the unit `U`: by default its base unit, `KILOGRAM_PER_SECOND_SQUARED_KELVIN`.
pub type ArealHeatCapacity<
    N,
    const U: SIUnit = { KILOGRAM_PER_SECOND_SQUARED_KELVIN.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `ArealHeatCapacity`: a product of base units.
pub const KILOGRAM_PER_SECOND_SQUARED_KELVIN: Unit<
    { (KILOGRAM / (SECOND * SECOND * KELVIN)).unit() },
    BASE,
> = Unit::ONE;

/// A volumetric heat capacity, in `N`s of the unit `U`: by default its base unit, `KILOGRAM_PER_METRE_SECOND_SQUARED_KELVIN`.
pub type VolumetricHeatCapacity<
    N,
    const U: SIUnit = { KILOGRAM_PER_METRE_SECOND_SQUARED_KELVIN.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `VolumetricHeatCapacity`: a product of base units.
pub const KILOGRAM_PER_METRE_SECOND_SQUARED_KELVIN: Unit<
    { (KILOGRAM / (METRE * SECOND * SECOND * KELVIN)).unit() },
    BASE,
> = Unit::ONE;

/// A stefan boltzmann coefficient, in `N`s of the unit `U`: by default its base unit, `KILOGRAM_PER_SECOND_CUBED_KELVIN_POW4`.
pub type StefanBoltzmannCoefficient<
    N,
    const U: SIUnit = { KILOGRAM_PER_SECOND_CUBED_KELVIN_POW4.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `StefanBoltzmannCoefficient`: a product of base units.
pub const KILOGRAM_PER_SECOND_CUBED_KELVIN_POW4: Unit<
    { (KILOGRAM / (SECOND * SECOND * SECOND * KELVIN * KELVIN * KELVIN * KELVIN)).unit() },
    BASE,
> = Unit::ONE;

/// Stefan-Boltzmann constants (`σ`), a unit of `StefanBoltzmannCoefficient`: `n` of them is `n · 5.670374419e-8` base units.
pub const STEFAN_BOLTZMANN_CONSTANT: Unit<
    {
        SIUnit::scaled(
            "σ",
            Exact::dec(5670374419, -17),
            KILOGRAM_PER_SECOND_CUBED_KELVIN_POW4.unit(),
        )
    },
> = Unit::ONE;
