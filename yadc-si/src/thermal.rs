//! The quantities of `thermal` and their units.

#[allow(unused_imports)]
use crate::*;

/// A thermodynamic temperature, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ThermodynamicTemperature<
    N,
    const U: SIUnit = { KELVIN.unit() },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// Degrees Celsius (`°C`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 1.0 + 273.15` base units.
pub const CELSIUS: SIQuantity<
    { SIUnit::offset("°C", Exact::int(1), Exact::dec(27315, -2), KELVIN.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Degrees Fahrenheit (`°F`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 5.0 / 9.0 + 273.15 - 32.0 * 5.0 / 9.0` base units.
pub const FAHRENHEIT: SIQuantity<
    {
        SIUnit::offset(
            "°F",
            Exact::int(5) / Exact::int(9),
            Exact::dec(27315, -2) - Exact::int(32) * Exact::int(5) / Exact::int(9),
            KELVIN.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Degrees Rankine (`°R`), a unit of `ThermodynamicTemperature`: `n` of them is `n · 5.0 / 9.0` base units.
pub const RANKINE: SIQuantity<
    { SIUnit::scaled("°R", Exact::int(5) / Exact::int(9), KELVIN.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// A temperature interval, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type TemperatureInterval<N, const U: SIUnit = { KELVIN.unit() }, const IS_BASE: bool = true> =
    SIQuantity<U, N, IS_BASE>;

/// Degrees Celsius (`°C`), a unit of `TemperatureInterval`: the coherent unit, a product of base units.
pub const DEGREE_CELSIUS: SIQuantity<{ KELVIN.unit() }, Exact, true> = SIQuantity::ONE;

/// Degrees Fahrenheit (`°F`), a unit of `TemperatureInterval`: `n` of them is `n · 5.0 / 9.0` base units.
pub const DEGREE_FAHRENHEIT: SIQuantity<
    { SIUnit::scaled("°F", Exact::int(5) / Exact::int(9), KELVIN.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Degrees Rankine (`°R`), a unit of `TemperatureInterval`: `n` of them is `n · 5.0 / 9.0` base units.
pub const DEGREE_RANKINE: SIQuantity<
    { SIUnit::scaled("°R", Exact::int(5) / Exact::int(9), KELVIN.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// A temperature coefficient, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type TemperatureCoefficient<
    N,
    const U: SIUnit = { SIUnit::ONE.times(KELVIN.unit(), -1) },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// A temperature gradient, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type TemperatureGradient<
    N,
    const U: SIUnit = { SIUnit::ONE.times(METRE.unit(), -1).times(KELVIN.unit(), 1) },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// A heat capacity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type HeatCapacity<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -2)
            .times(KELVIN.unit(), -1)
    },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// Boltzmann constants (`k`), a unit of `HeatCapacity`: `n` of them is `n · 1.380649e-23` base units.
pub const BOLTZMANN_CONSTANT: SIQuantity<
    {
        SIUnit::scaled(
            "k",
            Exact::dec(1380649, -29),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2)
                .times(KELVIN.unit(), -1),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// A heat flux density, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type HeatFluxDensity<
    N,
    const U: SIUnit = { KILOGRAM.unit().times(SECOND.unit(), -3) },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// A heat transfer, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type HeatTransfer<
    N,
    const U: SIUnit = {
        KILOGRAM
            .unit()
            .times(SECOND.unit(), -3)
            .times(KELVIN.unit(), -1)
    },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// A thermal conductance, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ThermalConductance<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -3)
            .times(KELVIN.unit(), -1)
    },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// A thermal conductivity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ThermalConductivity<
    N,
    const U: SIUnit = {
        METRE
            .unit()
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -3)
            .times(KELVIN.unit(), -1)
    },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// A thermal resistance, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ThermalResistance<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), -2)
            .times(KILOGRAM.unit(), -1)
            .times(SECOND.unit(), 3)
            .times(KELVIN.unit(), 1)
    },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// A specific heat capacity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type SpecificHeatCapacity<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(SECOND.unit(), -2)
            .times(KELVIN.unit(), -1)
    },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// An areal heat capacity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ArealHeatCapacity<
    N,
    const U: SIUnit = {
        KILOGRAM
            .unit()
            .times(SECOND.unit(), -2)
            .times(KELVIN.unit(), -1)
    },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// A volumetric heat capacity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type VolumetricHeatCapacity<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), -1)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -2)
            .times(KELVIN.unit(), -1)
    },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// A stefan boltzmann coefficient, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type StefanBoltzmannCoefficient<
    N,
    const U: SIUnit = {
        KILOGRAM
            .unit()
            .times(SECOND.unit(), -3)
            .times(KELVIN.unit(), -4)
    },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// Stefan-Boltzmann constants (`σ`), a unit of `StefanBoltzmannCoefficient`: `n` of them is `n · 5.670374419e-8` base units.
pub const STEFAN_BOLTZMANN_CONSTANT: SIQuantity<
    {
        SIUnit::scaled(
            "σ",
            Exact::dec(5670374419, -17),
            KILOGRAM
                .unit()
                .times(SECOND.unit(), -3)
                .times(KELVIN.unit(), -4),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;
