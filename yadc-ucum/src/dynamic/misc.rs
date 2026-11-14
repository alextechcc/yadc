//! The units of `misc`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::dynamic::DynamicUcumUnit;
use crate::misc::units::*;
use crate::misc::*;
use core::marker::Destruct;
use core::ops::{Add, Div, Mul, Sub};

/// The units of `ActionArea`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ActionArea<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ActionAreaUnit {
    #[default]
    #[unit(Barn)]
    Barn,
}

impl From<ActionAreaUnit> for DynamicUcumUnit {
    fn from(unit: ActionAreaUnit) -> Self {
        match unit {
            ActionAreaUnit::Barn => Barn {}.into(),
        }
    }
}

/// The units of `ElectricConductance`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricConductance<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ElectricConductanceUnit {
    #[default]
    #[unit(Mho)]
    Mho,
}

impl From<ElectricConductanceUnit> for DynamicUcumUnit {
    fn from(unit: ElectricConductanceUnit) -> Self {
        match unit {
            ElectricConductanceUnit::Mho => Mho {}.into(),
        }
    }
}

/// The units of `Length`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum LengthUnit {
    #[default]
    #[unit(Line)]
    Line,
    #[unit(Point)]
    Point,
    #[unit(Pica)]
    Pica,
    #[unit(PrinterSPoint)]
    PrinterSPoint,
    #[unit(PrinterSPica)]
    PrinterSPica,
    #[unit(FrenchFoot)]
    FrenchFoot,
    #[unit(FrenchInch)]
    FrenchInch,
    #[unit(FrenchLine)]
    FrenchLine,
    #[unit(DidotSPoint)]
    DidotSPoint,
    #[unit(DidotSPica)]
    DidotSPica,
    #[unit(Angstrom)]
    Angstrom,
    #[unit(Smoot)]
    Smoot,
}

impl From<LengthUnit> for DynamicUcumUnit {
    fn from(unit: LengthUnit) -> Self {
        match unit {
            LengthUnit::Line => Line {}.into(),
            LengthUnit::Point => Point {}.into(),
            LengthUnit::Pica => Pica {}.into(),
            LengthUnit::PrinterSPoint => PrinterSPoint {}.into(),
            LengthUnit::PrinterSPica => PrinterSPica {}.into(),
            LengthUnit::FrenchFoot => FrenchFoot {}.into(),
            LengthUnit::FrenchInch => FrenchInch {}.into(),
            LengthUnit::FrenchLine => FrenchLine {}.into(),
            LengthUnit::DidotSPoint => DidotSPoint {}.into(),
            LengthUnit::DidotSPica => DidotSPica {}.into(),
            LengthUnit::Angstrom => Angstrom {}.into(),
            LengthUnit::Smoot => Smoot {}.into(),
        }
    }
}

/// The units of `Mass`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Mass<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MassUnit {
    #[default]
    #[unit(MetricCarat)]
    MetricCarat,
}

impl From<MassUnit> for DynamicUcumUnit {
    fn from(unit: MassUnit) -> Self {
        match unit {
            MassUnit::MetricCarat => MetricCarat {}.into(),
        }
    }
}

/// The units of `MassFraction`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassFraction<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MassFractionUnit {
    #[default]
    #[unit(CaratOfGoldAlloys)]
    CaratOfGoldAlloys,
}

impl From<MassFractionUnit> for DynamicUcumUnit {
    fn from(unit: MassFractionUnit) -> Self {
        match unit {
            MassFractionUnit::CaratOfGoldAlloys => CaratOfGoldAlloys {}.into(),
        }
    }
}

/// The units of `PlaneAngle`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = PlaneAngle<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum PlaneAngleUnit {
    #[default]
    #[unit(Circle)]
    Circle,
}

impl From<PlaneAngleUnit> for DynamicUcumUnit {
    fn from(unit: PlaneAngleUnit) -> Self {
        match unit {
            PlaneAngleUnit::Circle => Circle {}.into(),
        }
    }
}

/// The units of `Pressure`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Pressure<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum PressureUnit {
    #[default]
    #[unit(TechnicalAtmosphere)]
    TechnicalAtmosphere,
    #[unit(PoundPerSquareInch)]
    PoundPerSquareInch,
}

impl From<PressureUnit> for DynamicUcumUnit {
    fn from(unit: PressureUnit) -> Self {
        match unit {
            PressureUnit::TechnicalAtmosphere => TechnicalAtmosphere {}.into(),
            PressureUnit::PoundPerSquareInch => PoundPerSquareInch {}.into(),
        }
    }
}

/// The units of `SolidAngle`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SolidAngle<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum SolidAngleUnit {
    #[default]
    #[unit(Sphere)]
    Sphere,
}

impl From<SolidAngleUnit> for DynamicUcumUnit {
    fn from(unit: SolidAngleUnit) -> Self {
        match unit {
            SolidAngleUnit::Sphere => Sphere {}.into(),
        }
    }
}

/// The units of `Turbidity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Turbidity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum TurbidityUnit {
    #[default]
    #[unit(NephelometricTurbidityUnit)]
    NephelometricTurbidityUnit,
    #[unit(FormazinNephelometricUnit)]
    FormazinNephelometricUnit,
}

impl From<TurbidityUnit> for DynamicUcumUnit {
    fn from(unit: TurbidityUnit) -> Self {
        match unit {
            TurbidityUnit::NephelometricTurbidityUnit => NephelometricTurbidityUnit {}.into(),
            TurbidityUnit::FormazinNephelometricUnit => FormazinNephelometricUnit {}.into(),
        }
    }
}

/// The units of `Volume`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Volume<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum VolumeUnit {
    #[default]
    #[unit(Stere)]
    Stere,
}

impl From<VolumeUnit> for DynamicUcumUnit {
    fn from(unit: VolumeUnit) -> Self {
        match unit {
            VolumeUnit::Stere => Stere {}.into(),
        }
    }
}
