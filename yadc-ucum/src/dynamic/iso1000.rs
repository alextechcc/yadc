//! The units of `iso1000`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::dynamic::DynamicUcumUnit;
use crate::iso1000::units::*;
use crate::iso1000::*;
use core::marker::Destruct;
use core::ops::{Add, Div, Mul, Sub};

/// The units of `Area`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Area<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum AreaUnit {
    #[default]
    #[unit(Are)]
    Are,
}

impl From<AreaUnit> for DynamicUcumUnit {
    fn from(unit: AreaUnit) -> Self {
        match unit {
            AreaUnit::Are => Are {}.into(),
        }
    }
}

/// The units of `Energy`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Energy<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum EnergyUnit {
    #[default]
    #[unit(Electronvolt)]
    Electronvolt,
}

impl From<EnergyUnit> for DynamicUcumUnit {
    fn from(unit: EnergyUnit) -> Self {
        match unit {
            EnergyUnit::Electronvolt => Electronvolt {}.into(),
        }
    }
}

/// The units of `Length`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum LengthUnit {
    #[default]
    #[unit(AstronomicUnit)]
    AstronomicUnit,
    #[unit(Parsec)]
    Parsec,
}

impl From<LengthUnit> for DynamicUcumUnit {
    fn from(unit: LengthUnit) -> Self {
        match unit {
            LengthUnit::AstronomicUnit => AstronomicUnit {}.into(),
            LengthUnit::Parsec => Parsec {}.into(),
        }
    }
}

/// The units of `Mass`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Mass<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MassUnit {
    #[default]
    #[unit(Tonne)]
    Tonne,
    #[unit(UnifiedAtomicMassUnit)]
    UnifiedAtomicMassUnit,
}

impl From<MassUnit> for DynamicUcumUnit {
    fn from(unit: MassUnit) -> Self {
        match unit {
            MassUnit::Tonne => Tonne {}.into(),
            MassUnit::UnifiedAtomicMassUnit => UnifiedAtomicMassUnit {}.into(),
        }
    }
}

/// The units of `PlaneAngle`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = PlaneAngle<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum PlaneAngleUnit {
    #[default]
    #[unit(Grade)]
    Grade,
    #[unit(Degree)]
    Degree,
    #[unit(PlaneAngleMinute)]
    Minute,
    #[unit(Second)]
    Second,
}

impl From<PlaneAngleUnit> for DynamicUcumUnit {
    fn from(unit: PlaneAngleUnit) -> Self {
        match unit {
            PlaneAngleUnit::Grade => Grade {}.into(),
            PlaneAngleUnit::Degree => Degree {}.into(),
            PlaneAngleUnit::Minute => PlaneAngleMinute {}.into(),
            PlaneAngleUnit::Second => Second {}.into(),
        }
    }
}

/// The units of `Pressure`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Pressure<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum PressureUnit {
    #[default]
    #[unit(Bar)]
    Bar,
}

impl From<PressureUnit> for DynamicUcumUnit {
    fn from(unit: PressureUnit) -> Self {
        match unit {
            PressureUnit::Bar => Bar {}.into(),
        }
    }
}

/// The units of `Time`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Time<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum TimeUnit {
    #[default]
    #[unit(TimeMinute)]
    Minute,
    #[unit(Hour)]
    Hour,
    #[unit(Day)]
    Day,
    #[unit(TropicalYear)]
    TropicalYear,
    #[unit(MeanJulianYear)]
    MeanJulianYear,
    #[unit(MeanGregorianYear)]
    MeanGregorianYear,
    #[unit(Year)]
    Year,
    #[unit(Week)]
    Week,
    #[unit(SynodalMonth)]
    SynodalMonth,
    #[unit(MeanJulianMonth)]
    MeanJulianMonth,
    #[unit(MeanGregorianMonth)]
    MeanGregorianMonth,
    #[unit(Month)]
    Month,
}

impl From<TimeUnit> for DynamicUcumUnit {
    fn from(unit: TimeUnit) -> Self {
        match unit {
            TimeUnit::Minute => TimeMinute {}.into(),
            TimeUnit::Hour => Hour {}.into(),
            TimeUnit::Day => Day {}.into(),
            TimeUnit::TropicalYear => TropicalYear {}.into(),
            TimeUnit::MeanJulianYear => MeanJulianYear {}.into(),
            TimeUnit::MeanGregorianYear => MeanGregorianYear {}.into(),
            TimeUnit::Year => Year {}.into(),
            TimeUnit::Week => Week {}.into(),
            TimeUnit::SynodalMonth => SynodalMonth {}.into(),
            TimeUnit::MeanJulianMonth => MeanJulianMonth {}.into(),
            TimeUnit::MeanGregorianMonth => MeanGregorianMonth {}.into(),
            TimeUnit::Month => Month {}.into(),
        }
    }
}

/// The units of `Volume`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Volume<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum VolumeUnit {
    #[default]
    #[unit(Liter)]
    Liter,
}

impl From<VolumeUnit> for DynamicUcumUnit {
    fn from(unit: VolumeUnit) -> Self {
        match unit {
            VolumeUnit::Liter => Liter {}.into(),
        }
    }
}
