use crate::{Exact, ISQ, Kind, SIQuantity, Scale};
use yadc::Dimension;

pub type ThermodynamicTemperature<N> = SIQuantity<
    {
        ISQ {
            K: 1,
            kind: Kind::Absolute,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type TemperatureInterval<N> = SIQuantity<
    {
        ISQ {
            K: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type TemperatureCoefficient<N> = SIQuantity<
    {
        ISQ {
            K: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type TemperatureGradient<N> = SIQuantity<
    {
        ISQ {
            m: -1,
            K: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type HeatCapacity<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -2,
            K: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type HeatFluxDensity<N> = SIQuantity<
    {
        ISQ {
            kg: 1,
            s: -3,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type HeatTransfer<N> = SIQuantity<
    {
        ISQ {
            kg: 1,
            s: -3,
            K: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ThermalConductance<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -3,
            K: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ThermalConductivity<N> = SIQuantity<
    {
        ISQ {
            m: 1,
            kg: 1,
            s: -3,
            K: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ThermalResistance<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            kg: -1,
            s: 3,
            K: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type SpecificHeatCapacity<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            s: -2,
            K: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ArealHeatCapacity<N> = SIQuantity<
    {
        ISQ {
            kg: 1,
            s: -2,
            K: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type VolumetricHeatCapacity<N> = SIQuantity<
    {
        ISQ {
            m: -1,
            kg: 1,
            s: -2,
            K: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type StefanBoltzmannCoefficient<N> = SIQuantity<
    {
        ISQ {
            kg: 1,
            s: -3,
            K: -4,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

/// The constants of this module: the base constants of its quantities
/// (re-exported), then one per named unit, each defined exactly from them
/// (`INCH` is `0.0254 * METRE`). A unit written as others is not one:
/// `SLUG / CUBIC_FOOT`, `KILO * METRE` are exact too. Converting rounds,
/// once, where it is written - in a `const`, by the compiler:
/// `const MPH: Velocity<f64> = (MILE / HOUR).into();`, then `MPH(60.0)`.
pub mod consts {
    use super::*;
    use crate::base::{KILOGRAM, METRE, ONE, SECOND};

    /// The kelvin as a step: a temperature difference of 1 K (the same as 1 °C of difference).
    pub use crate::base::KELVIN;

    /// Absolute zero, 0 K: the zero of thermodynamic temperature.
    pub use crate::base::ABSOLUTE_ZERO;

    /// 0 °C, the ice point of water: 273.15 K.
    pub const ICE_POINT: ThermodynamicTemperature<Exact> =
        ABSOLUTE_ZERO + KELVIN * Exact::dec(27315, -2);

    /// 0 °F: 32 °F below the ice point.
    pub const FAHRENHEIT_ZERO: ThermodynamicTemperature<Exact> =
        ICE_POINT - DEGREE_FAHRENHEIT * Exact::int(32);

    /// The kelvin scale (K): kelvins from absolute zero.
    pub const KELVIN_SCALE: Scale<ThermodynamicTemperature<Exact>, TemperatureInterval<Exact>> =
        Scale(ABSOLUTE_ZERO, KELVIN);

    /// The Celsius scale (°C): kelvins from the ice point.
    pub const CELSIUS: Scale<ThermodynamicTemperature<Exact>, TemperatureInterval<Exact>> =
        Scale(ICE_POINT, KELVIN);

    /// The Fahrenheit scale (°F): degrees Fahrenheit from 32 °F below the ice point.
    pub const FAHRENHEIT: Scale<ThermodynamicTemperature<Exact>, TemperatureInterval<Exact>> =
        Scale(FAHRENHEIT_ZERO, DEGREE_FAHRENHEIT);

    /// The Rankine scale (°R): degrees Fahrenheit from absolute zero.
    pub const RANKINE: Scale<ThermodynamicTemperature<Exact>, TemperatureInterval<Exact>> =
        Scale(ABSOLUTE_ZERO, DEGREE_FAHRENHEIT);

    /// One `Fahrenheit` (`°F`), as a `TemperatureInterval`.
    pub const DEGREE_FAHRENHEIT: TemperatureInterval<Exact> =
        KELVIN * (Exact::int(5) / Exact::int(9));

    /// One `PerKelvin` (`K⁻¹`), as a `TemperatureCoefficient`.
    pub const PER_KELVIN: TemperatureCoefficient<Exact> = ONE / KELVIN;

    /// One `BoltzmannConstant` (`k`), as a `HeatCapacity`.
    pub const BOLTZMANN_CONSTANT: HeatCapacity<Exact> =
        (METRE * METRE * KILOGRAM / (SECOND * SECOND * KELVIN)) * Exact::dec(1380649, -29);

    /// One `StefanBoltzmannConstant` (`σ`), as a `StefanBoltzmannCoefficient`.
    pub const STEFAN_BOLTZMANN_CONSTANT: StefanBoltzmannCoefficient<Exact> = (KILOGRAM
        / (SECOND * SECOND * SECOND * KELVIN * KELVIN * KELVIN * KELVIN))
        * Exact::dec(5670374419, -17);
}
