//! The units of `thermal`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::dynamic::DynamicSIUnit;
use crate::thermal::units::*;
use crate::thermal::*;
use core::marker::Destruct;
use core::ops::{Add, Div, Mul, Sub};

/// The units of `ThermodynamicTemperature`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ThermodynamicTemperature<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ThermodynamicTemperatureUnit {
    #[default]
    #[unit(KelvinScale)]
    Kelvin,
    #[unit(Celsius)]
    Celsius,
    #[unit(Fahrenheit)]
    Fahrenheit,
    #[unit(Rankine)]
    Rankine,
}

impl From<ThermodynamicTemperatureUnit> for DynamicSIUnit {
    fn from(unit: ThermodynamicTemperatureUnit) -> Self {
        match unit {
            ThermodynamicTemperatureUnit::Kelvin => KelvinScale {}.into(),
            ThermodynamicTemperatureUnit::Celsius => Celsius {}.into(),
            ThermodynamicTemperatureUnit::Fahrenheit => Fahrenheit {}.into(),
            ThermodynamicTemperatureUnit::Rankine => Rankine {}.into(),
        }
    }
}

/// The units of `TemperatureInterval`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = TemperatureInterval<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum TemperatureIntervalUnit {
    #[default]
    #[unit(Kelvin)]
    Kelvin,
    #[unit(DegreeCelsius)]
    Celsius,
    #[unit(DegreeFahrenheit)]
    Fahrenheit,
    #[unit(DegreeRankine)]
    Rankine,
}

impl From<TemperatureIntervalUnit> for DynamicSIUnit {
    fn from(unit: TemperatureIntervalUnit) -> Self {
        match unit {
            TemperatureIntervalUnit::Kelvin => Kelvin {}.into(),
            TemperatureIntervalUnit::Celsius => DegreeCelsius {}.into(),
            TemperatureIntervalUnit::Fahrenheit => DegreeFahrenheit {}.into(),
            TemperatureIntervalUnit::Rankine => DegreeRankine {}.into(),
        }
    }
}

/// The units of `TemperatureCoefficient`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = TemperatureCoefficient<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum TemperatureCoefficientUnit {
    #[default]
    #[unit(PerKelvin)]
    PerKelvin,
    #[unit(PpmPerKelvin)]
    PpmPerKelvin,
    #[unit(PpmPerDegreeCelsius)]
    PpmPerDegreeCelsius,
}

impl From<TemperatureCoefficientUnit> for DynamicSIUnit {
    fn from(unit: TemperatureCoefficientUnit) -> Self {
        match unit {
            TemperatureCoefficientUnit::PerKelvin => PerKelvin {}.into(),
            TemperatureCoefficientUnit::PpmPerKelvin => PpmPerKelvin {}.into(),
            TemperatureCoefficientUnit::PpmPerDegreeCelsius => PpmPerDegreeCelsius {}.into(),
        }
    }
}

/// The units of `TemperatureGradient`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = TemperatureGradient<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum TemperatureGradientUnit {
    #[default]
    #[unit(KelvinPerMetre)]
    KelvinPerMetre,
}

impl From<TemperatureGradientUnit> for DynamicSIUnit {
    fn from(unit: TemperatureGradientUnit) -> Self {
        match unit {
            TemperatureGradientUnit::KelvinPerMetre => KelvinPerMetre {}.into(),
        }
    }
}

/// The units of `HeatCapacity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = HeatCapacity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum HeatCapacityUnit {
    #[default]
    #[unit(KilogramSquareMetrePerSecondSquaredKelvin)]
    KilogramSquareMetrePerSecondSquaredKelvin,
    #[unit(GramSquareMetrePerSecondSquaredKelvin)]
    GramSquareMetrePerSecondSquaredKelvin,
    #[unit(JoulePerKelvin)]
    JoulePerKelvin,
    #[unit(JoulePerDegreeCelsius)]
    JoulePerDegreeCelsius,
    #[unit(BtuPerDegreeFahrenheit)]
    BtuPerDegreeFahrenheit,
    #[unit(BtuItPerDegreeFahrenheit)]
    BtuItPerDegreeFahrenheit,
    #[unit(BoltzmannConstant)]
    BoltzmannConstant,
    #[unit(CaloriePerKelvin)]
    CaloriePerKelvin,
}

impl From<HeatCapacityUnit> for DynamicSIUnit {
    fn from(unit: HeatCapacityUnit) -> Self {
        match unit {
            HeatCapacityUnit::KilogramSquareMetrePerSecondSquaredKelvin => {
                KilogramSquareMetrePerSecondSquaredKelvin {}.into()
            }
            HeatCapacityUnit::GramSquareMetrePerSecondSquaredKelvin => {
                GramSquareMetrePerSecondSquaredKelvin {}.into()
            }
            HeatCapacityUnit::JoulePerKelvin => JoulePerKelvin {}.into(),
            HeatCapacityUnit::JoulePerDegreeCelsius => JoulePerDegreeCelsius {}.into(),
            HeatCapacityUnit::BtuPerDegreeFahrenheit => BtuPerDegreeFahrenheit {}.into(),
            HeatCapacityUnit::BtuItPerDegreeFahrenheit => BtuItPerDegreeFahrenheit {}.into(),
            HeatCapacityUnit::BoltzmannConstant => BoltzmannConstant {}.into(),
            HeatCapacityUnit::CaloriePerKelvin => CaloriePerKelvin {}.into(),
        }
    }
}

/// The units of `HeatFluxDensity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = HeatFluxDensity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum HeatFluxDensityUnit {
    #[default]
    #[unit(WattPerSquareMetre)]
    WattPerSquareMetre,
}

impl From<HeatFluxDensityUnit> for DynamicSIUnit {
    fn from(unit: HeatFluxDensityUnit) -> Self {
        match unit {
            HeatFluxDensityUnit::WattPerSquareMetre => WattPerSquareMetre {}.into(),
        }
    }
}

/// The units of `HeatTransfer`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = HeatTransfer<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum HeatTransferUnit {
    #[default]
    #[unit(KilogramPerSecondCubedKelvin)]
    KilogramPerSecondCubedKelvin,
    #[unit(GramPerSecondCubedKelvin)]
    GramPerSecondCubedKelvin,
    #[unit(KilogramPerSecondCubedDegreeCelsius)]
    KilogramPerSecondCubedDegreeCelsius,
    #[unit(WattPerSquareMetreKelvin)]
    WattPerSquareMetreKelvin,
    #[unit(WattPerSquareMetreDegreeCelsius)]
    WattPerSquareMetreDegreeCelsius,
    #[unit(JoulePerSecondSquareMetreKelvin)]
    JoulePerSecondSquareMetreKelvin,
    #[unit(JoulePerSecondSquareMetreDegreeCelsius)]
    JoulePerSecondSquareMetreDegreeCelsius,
    #[unit(BtuItPerHourSquareFootDegreeFahrenheit)]
    BtuItPerHourSquareFootDegreeFahrenheit,
    #[unit(BtuItPerHourSquareInchDegreeFahrenheit)]
    BtuItPerHourSquareInchDegreeFahrenheit,
    #[unit(BtuItPerMinuteSquareFootDegreeFahrenheit)]
    BtuItPerMinuteSquareFootDegreeFahrenheit,
    #[unit(BtuItPerMinuteSquareInchDegreeFahrenheit)]
    BtuItPerMinuteSquareInchDegreeFahrenheit,
    #[unit(BtuItPerSecondSquareFootDegreeFahrenheit)]
    BtuItPerSecondSquareFootDegreeFahrenheit,
    #[unit(BtuItPerSecondSquareInchDegreeFahrenheit)]
    BtuItPerSecondSquareInchDegreeFahrenheit,
    #[unit(BtuPerHourSquareFootDegreeFahrenheit)]
    BtuPerHourSquareFootDegreeFahrenheit,
    #[unit(BtuPerHourSquareInchDegreeFahrenheit)]
    BtuPerHourSquareInchDegreeFahrenheit,
    #[unit(BtuPerMinuteSquareFootDegreeFahrenheit)]
    BtuPerMinuteSquareFootDegreeFahrenheit,
    #[unit(BtuPerMinuteSquareInchDegreeFahrenheit)]
    BtuPerMinuteSquareInchDegreeFahrenheit,
    #[unit(BtuPerSecondSquareFootDegreeFahrenheit)]
    BtuPerSecondSquareFootDegreeFahrenheit,
    #[unit(BtuPerSecondSquareInchDegreeFahrenheit)]
    BtuPerSecondSquareInchDegreeFahrenheit,
}

impl From<HeatTransferUnit> for DynamicSIUnit {
    fn from(unit: HeatTransferUnit) -> Self {
        match unit {
            HeatTransferUnit::KilogramPerSecondCubedKelvin => {
                KilogramPerSecondCubedKelvin {}.into()
            }
            HeatTransferUnit::GramPerSecondCubedKelvin => GramPerSecondCubedKelvin {}.into(),
            HeatTransferUnit::KilogramPerSecondCubedDegreeCelsius => {
                KilogramPerSecondCubedDegreeCelsius {}.into()
            }
            HeatTransferUnit::WattPerSquareMetreKelvin => WattPerSquareMetreKelvin {}.into(),
            HeatTransferUnit::WattPerSquareMetreDegreeCelsius => {
                WattPerSquareMetreDegreeCelsius {}.into()
            }
            HeatTransferUnit::JoulePerSecondSquareMetreKelvin => {
                JoulePerSecondSquareMetreKelvin {}.into()
            }
            HeatTransferUnit::JoulePerSecondSquareMetreDegreeCelsius => {
                JoulePerSecondSquareMetreDegreeCelsius {}.into()
            }
            HeatTransferUnit::BtuItPerHourSquareFootDegreeFahrenheit => {
                BtuItPerHourSquareFootDegreeFahrenheit {}.into()
            }
            HeatTransferUnit::BtuItPerHourSquareInchDegreeFahrenheit => {
                BtuItPerHourSquareInchDegreeFahrenheit {}.into()
            }
            HeatTransferUnit::BtuItPerMinuteSquareFootDegreeFahrenheit => {
                BtuItPerMinuteSquareFootDegreeFahrenheit {}.into()
            }
            HeatTransferUnit::BtuItPerMinuteSquareInchDegreeFahrenheit => {
                BtuItPerMinuteSquareInchDegreeFahrenheit {}.into()
            }
            HeatTransferUnit::BtuItPerSecondSquareFootDegreeFahrenheit => {
                BtuItPerSecondSquareFootDegreeFahrenheit {}.into()
            }
            HeatTransferUnit::BtuItPerSecondSquareInchDegreeFahrenheit => {
                BtuItPerSecondSquareInchDegreeFahrenheit {}.into()
            }
            HeatTransferUnit::BtuPerHourSquareFootDegreeFahrenheit => {
                BtuPerHourSquareFootDegreeFahrenheit {}.into()
            }
            HeatTransferUnit::BtuPerHourSquareInchDegreeFahrenheit => {
                BtuPerHourSquareInchDegreeFahrenheit {}.into()
            }
            HeatTransferUnit::BtuPerMinuteSquareFootDegreeFahrenheit => {
                BtuPerMinuteSquareFootDegreeFahrenheit {}.into()
            }
            HeatTransferUnit::BtuPerMinuteSquareInchDegreeFahrenheit => {
                BtuPerMinuteSquareInchDegreeFahrenheit {}.into()
            }
            HeatTransferUnit::BtuPerSecondSquareFootDegreeFahrenheit => {
                BtuPerSecondSquareFootDegreeFahrenheit {}.into()
            }
            HeatTransferUnit::BtuPerSecondSquareInchDegreeFahrenheit => {
                BtuPerSecondSquareInchDegreeFahrenheit {}.into()
            }
        }
    }
}

/// The units of `ThermalConductance`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ThermalConductance<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ThermalConductanceUnit {
    #[default]
    #[unit(KilogramMetreSquaredPerSecondCubedKelvin)]
    KilogramMetreSquaredPerSecondCubedKelvin,
    #[unit(GramMetreSquaredPerSecondCubedKelvin)]
    GramMetreSquaredPerSecondCubedKelvin,
    #[unit(WattPerKelvin)]
    WattPerKelvin,
    #[unit(KilogramMetreSquaredPerSecondCubedDegreeCelsius)]
    KilogramMetreSquaredPerSecondCubedDegreeCelsius,
    #[unit(ThermalConductanceWattPerMetreDegreeCelsius)]
    WattPerMetreDegreeCelsius,
    #[unit(KilowattPerDegreeCelsius)]
    KilowattPerDegreeCelsius,
    #[unit(MilliwattPerDegreeCelsius)]
    MilliwattPerDegreeCelsius,
}

impl From<ThermalConductanceUnit> for DynamicSIUnit {
    fn from(unit: ThermalConductanceUnit) -> Self {
        match unit {
            ThermalConductanceUnit::KilogramMetreSquaredPerSecondCubedKelvin => {
                KilogramMetreSquaredPerSecondCubedKelvin {}.into()
            }
            ThermalConductanceUnit::GramMetreSquaredPerSecondCubedKelvin => {
                GramMetreSquaredPerSecondCubedKelvin {}.into()
            }
            ThermalConductanceUnit::WattPerKelvin => WattPerKelvin {}.into(),
            ThermalConductanceUnit::KilogramMetreSquaredPerSecondCubedDegreeCelsius => {
                KilogramMetreSquaredPerSecondCubedDegreeCelsius {}.into()
            }
            ThermalConductanceUnit::WattPerMetreDegreeCelsius => {
                ThermalConductanceWattPerMetreDegreeCelsius {}.into()
            }
            ThermalConductanceUnit::KilowattPerDegreeCelsius => KilowattPerDegreeCelsius {}.into(),
            ThermalConductanceUnit::MilliwattPerDegreeCelsius => {
                MilliwattPerDegreeCelsius {}.into()
            }
        }
    }
}

/// The units of `ThermalConductivity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ThermalConductivity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ThermalConductivityUnit {
    #[default]
    #[unit(KilogramMetrePerSecondCubedKelvin)]
    KilogramMetrePerSecondCubedKelvin,
    #[unit(GramMetrePerSecondCubedKelvin)]
    GramMetrePerSecondCubedKelvin,
    #[unit(WattPerMetreKelvin)]
    WattPerMetreKelvin,
    #[unit(KilogramMetrePerSecondCubedDegreeCelsius)]
    KilogramMetrePerSecondCubedDegreeCelsius,
    #[unit(ThermalConductivityWattPerMetreDegreeCelsius)]
    WattPerMetreDegreeCelsius,
}

impl From<ThermalConductivityUnit> for DynamicSIUnit {
    fn from(unit: ThermalConductivityUnit) -> Self {
        match unit {
            ThermalConductivityUnit::KilogramMetrePerSecondCubedKelvin => {
                KilogramMetrePerSecondCubedKelvin {}.into()
            }
            ThermalConductivityUnit::GramMetrePerSecondCubedKelvin => {
                GramMetrePerSecondCubedKelvin {}.into()
            }
            ThermalConductivityUnit::WattPerMetreKelvin => WattPerMetreKelvin {}.into(),
            ThermalConductivityUnit::KilogramMetrePerSecondCubedDegreeCelsius => {
                KilogramMetrePerSecondCubedDegreeCelsius {}.into()
            }
            ThermalConductivityUnit::WattPerMetreDegreeCelsius => {
                ThermalConductivityWattPerMetreDegreeCelsius {}.into()
            }
        }
    }
}

/// The units of `ThermalResistance`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ThermalResistance<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ThermalResistanceUnit {
    #[default]
    #[unit(KelvinPerWatt)]
    KelvinPerWatt,
    #[unit(KelvinPerKilowatt)]
    KelvinPerKilowatt,
}

impl From<ThermalResistanceUnit> for DynamicSIUnit {
    fn from(unit: ThermalResistanceUnit) -> Self {
        match unit {
            ThermalResistanceUnit::KelvinPerWatt => KelvinPerWatt {}.into(),
            ThermalResistanceUnit::KelvinPerKilowatt => KelvinPerKilowatt {}.into(),
        }
    }
}

/// The units of `SpecificHeatCapacity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificHeatCapacity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum SpecificHeatCapacityUnit {
    #[default]
    #[unit(SquareMetrePerSecondSquaredKelvin)]
    SquareMetrePerSecondSquaredKelvin,
    #[unit(JoulePerKilogramKelvin)]
    JoulePerKilogramKelvin,
    #[unit(JoulePerKilogramDegreeCelsius)]
    JoulePerKilogramDegreeCelsius,
    #[unit(JoulePerGramDegreeCelsius)]
    JoulePerGramDegreeCelsius,
    #[unit(BtuPerOunceDegreeFahrenheit)]
    BtuPerOunceDegreeFahrenheit,
    #[unit(BtuItPerOunceDegreeFahrenheit)]
    BtuItPerOunceDegreeFahrenheit,
    #[unit(BtuPerPoundDegreeFahrenheit)]
    BtuPerPoundDegreeFahrenheit,
    #[unit(BtuItPerPoundDegreeFahrenheit)]
    BtuItPerPoundDegreeFahrenheit,
    #[unit(BtuPerTonDegreeFahrenheit)]
    BtuPerTonDegreeFahrenheit,
    #[unit(BtuItPerTonDegreeFahrenheit)]
    BtuItPerTonDegreeFahrenheit,
    #[unit(CaloriePerGramKelvin)]
    CaloriePerGramKelvin,
}

impl From<SpecificHeatCapacityUnit> for DynamicSIUnit {
    fn from(unit: SpecificHeatCapacityUnit) -> Self {
        match unit {
            SpecificHeatCapacityUnit::SquareMetrePerSecondSquaredKelvin => {
                SquareMetrePerSecondSquaredKelvin {}.into()
            }
            SpecificHeatCapacityUnit::JoulePerKilogramKelvin => JoulePerKilogramKelvin {}.into(),
            SpecificHeatCapacityUnit::JoulePerKilogramDegreeCelsius => {
                JoulePerKilogramDegreeCelsius {}.into()
            }
            SpecificHeatCapacityUnit::JoulePerGramDegreeCelsius => {
                JoulePerGramDegreeCelsius {}.into()
            }
            SpecificHeatCapacityUnit::BtuPerOunceDegreeFahrenheit => {
                BtuPerOunceDegreeFahrenheit {}.into()
            }
            SpecificHeatCapacityUnit::BtuItPerOunceDegreeFahrenheit => {
                BtuItPerOunceDegreeFahrenheit {}.into()
            }
            SpecificHeatCapacityUnit::BtuPerPoundDegreeFahrenheit => {
                BtuPerPoundDegreeFahrenheit {}.into()
            }
            SpecificHeatCapacityUnit::BtuItPerPoundDegreeFahrenheit => {
                BtuItPerPoundDegreeFahrenheit {}.into()
            }
            SpecificHeatCapacityUnit::BtuPerTonDegreeFahrenheit => {
                BtuPerTonDegreeFahrenheit {}.into()
            }
            SpecificHeatCapacityUnit::BtuItPerTonDegreeFahrenheit => {
                BtuItPerTonDegreeFahrenheit {}.into()
            }
            SpecificHeatCapacityUnit::CaloriePerGramKelvin => CaloriePerGramKelvin {}.into(),
        }
    }
}

/// The units of `ArealHeatCapacity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ArealHeatCapacity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ArealHeatCapacityUnit {
    #[default]
    #[unit(JoulePerSquareMetreKelvin)]
    JoulePerSquareMetreKelvin,
}

impl From<ArealHeatCapacityUnit> for DynamicSIUnit {
    fn from(unit: ArealHeatCapacityUnit) -> Self {
        match unit {
            ArealHeatCapacityUnit::JoulePerSquareMetreKelvin => JoulePerSquareMetreKelvin {}.into(),
        }
    }
}

/// The units of `VolumetricHeatCapacity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = VolumetricHeatCapacity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum VolumetricHeatCapacityUnit {
    #[default]
    #[unit(JoulePerCubicMetreKelvin)]
    JoulePerCubicMetreKelvin,
    #[unit(CaloriePerCubicMetreKelvin)]
    CaloriePerCubicMetreKelvin,
}

impl From<VolumetricHeatCapacityUnit> for DynamicSIUnit {
    fn from(unit: VolumetricHeatCapacityUnit) -> Self {
        match unit {
            VolumetricHeatCapacityUnit::JoulePerCubicMetreKelvin => {
                JoulePerCubicMetreKelvin {}.into()
            }
            VolumetricHeatCapacityUnit::CaloriePerCubicMetreKelvin => {
                CaloriePerCubicMetreKelvin {}.into()
            }
        }
    }
}

/// The units of `StefanBoltzmannCoefficient`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = StefanBoltzmannCoefficient<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum StefanBoltzmannCoefficientUnit {
    #[default]
    #[unit(WattPerSquareMetreQuarticKelvin)]
    WattPerSquareMetreQuarticKelvin,
    #[unit(StefanBoltzmannConstant)]
    StefanBoltzmannConstant,
}

impl From<StefanBoltzmannCoefficientUnit> for DynamicSIUnit {
    fn from(unit: StefanBoltzmannCoefficientUnit) -> Self {
        match unit {
            StefanBoltzmannCoefficientUnit::WattPerSquareMetreQuarticKelvin => {
                WattPerSquareMetreQuarticKelvin {}.into()
            }
            StefanBoltzmannCoefficientUnit::StefanBoltzmannConstant => {
                StefanBoltzmannConstant {}.into()
            }
        }
    }
}
