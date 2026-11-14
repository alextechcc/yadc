//! The units of `fluid`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::dynamic::DynamicSIUnit;
use crate::fluid::units::*;
use crate::fluid::*;
use core::marker::Destruct;
use core::ops::{Add, Div, Mul, Sub};

/// The units of `DynamicViscosity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = DynamicViscosity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum DynamicViscosityUnit {
    #[default]
    #[unit(PascalSecond)]
    PascalSecond,
    #[unit(Poise)]
    Poise,
    #[unit(PoundPerFootSecond)]
    PoundPerFootSecond,
    #[unit(SlugPerFootSecond)]
    SlugPerFootSecond,
    #[unit(GramPerCentimetreSecond)]
    GramPerCentimetreSecond,
}

impl From<DynamicViscosityUnit> for DynamicSIUnit {
    fn from(unit: DynamicViscosityUnit) -> Self {
        match unit {
            DynamicViscosityUnit::PascalSecond => PascalSecond {}.into(),
            DynamicViscosityUnit::Poise => Poise {}.into(),
            DynamicViscosityUnit::PoundPerFootSecond => PoundPerFootSecond {}.into(),
            DynamicViscosityUnit::SlugPerFootSecond => SlugPerFootSecond {}.into(),
            DynamicViscosityUnit::GramPerCentimetreSecond => GramPerCentimetreSecond {}.into(),
        }
    }
}

/// The units of `KinematicViscosity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = KinematicViscosity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum KinematicViscosityUnit {
    #[default]
    #[unit(KinematicViscositySquareMetrePerSecond)]
    SquareMetrePerSecond,
    #[unit(KinematicViscosityStokes)]
    Stokes,
}

impl From<KinematicViscosityUnit> for DynamicSIUnit {
    fn from(unit: KinematicViscosityUnit) -> Self {
        match unit {
            KinematicViscosityUnit::SquareMetrePerSecond => {
                KinematicViscositySquareMetrePerSecond {}.into()
            }
            KinematicViscosityUnit::Stokes => KinematicViscosityStokes {}.into(),
        }
    }
}

/// The units of `SurfaceTension`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SurfaceTension<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum SurfaceTensionUnit {
    #[default]
    #[unit(NewtonPerMetre)]
    NewtonPerMetre,
    #[unit(KilogramForcePerMetre)]
    KilogramForcePerMetre,
    #[unit(OunceForcePerInch)]
    OunceForcePerInch,
    #[unit(PoundalPerInch)]
    PoundalPerInch,
    #[unit(PoundForcePerInch)]
    PoundForcePerInch,
    #[unit(JoulePerSquareMetre)]
    JoulePerSquareMetre,
}

impl From<SurfaceTensionUnit> for DynamicSIUnit {
    fn from(unit: SurfaceTensionUnit) -> Self {
        match unit {
            SurfaceTensionUnit::NewtonPerMetre => NewtonPerMetre {}.into(),
            SurfaceTensionUnit::KilogramForcePerMetre => KilogramForcePerMetre {}.into(),
            SurfaceTensionUnit::OunceForcePerInch => OunceForcePerInch {}.into(),
            SurfaceTensionUnit::PoundalPerInch => PoundalPerInch {}.into(),
            SurfaceTensionUnit::PoundForcePerInch => PoundForcePerInch {}.into(),
            SurfaceTensionUnit::JoulePerSquareMetre => JoulePerSquareMetre {}.into(),
        }
    }
}

/// The units of `CompressibilityCoefficient`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = CompressibilityCoefficient<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum CompressibilityCoefficientUnit {
    #[default]
    #[unit(PerPascal)]
    PerPascal,
    #[unit(PerAtmosphere)]
    PerAtmosphere,
    #[unit(PerAtmosphereTechnical)]
    PerAtmosphereTechnical,
    #[unit(PerBar)]
    PerBar,
}

impl From<CompressibilityCoefficientUnit> for DynamicSIUnit {
    fn from(unit: CompressibilityCoefficientUnit) -> Self {
        match unit {
            CompressibilityCoefficientUnit::PerPascal => PerPascal {}.into(),
            CompressibilityCoefficientUnit::PerAtmosphere => PerAtmosphere {}.into(),
            CompressibilityCoefficientUnit::PerAtmosphereTechnical => {
                PerAtmosphereTechnical {}.into()
            }
            CompressibilityCoefficientUnit::PerBar => PerBar {}.into(),
        }
    }
}

/// The units of `MassRate`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassRate<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MassRateUnit {
    #[default]
    #[unit(KilogramPerSecond)]
    KilogramPerSecond,
    #[unit(GramPerSecond)]
    GramPerSecond,
    #[unit(GramPerMinute)]
    GramPerMinute,
    #[unit(GramPerHour)]
    GramPerHour,
    #[unit(GramPerDay)]
    GramPerDay,
    #[unit(CaratPerSecond)]
    CaratPerSecond,
    #[unit(GrainPerSecond)]
    GrainPerSecond,
    #[unit(HundredweightLongPerSecond)]
    HundredweightLongPerSecond,
    #[unit(HundredweightShortPerSecond)]
    HundredweightShortPerSecond,
    #[unit(OuncePerSecond)]
    OuncePerSecond,
    #[unit(OunceTroyPerSecond)]
    OunceTroyPerSecond,
    #[unit(PennyweightPerSecond)]
    PennyweightPerSecond,
    #[unit(PoundPerSecond)]
    PoundPerSecond,
    #[unit(PoundPerMinute)]
    PoundPerMinute,
    #[unit(PoundPerHour)]
    PoundPerHour,
    #[unit(PoundPerDay)]
    PoundPerDay,
    #[unit(PoundTroyPerSecond)]
    PoundTroyPerSecond,
    #[unit(SlugPerSecond)]
    SlugPerSecond,
    #[unit(TonAssayPerSecond)]
    TonAssayPerSecond,
    #[unit(TonLongPerSecond)]
    TonLongPerSecond,
    #[unit(TonShortPerSecond)]
    TonShortPerSecond,
    #[unit(TonShortPerHour)]
    TonShortPerHour,
    #[unit(TonPerSecond)]
    TonPerSecond,
    #[unit(TonPerMinute)]
    TonPerMinute,
    #[unit(TonPerHour)]
    TonPerHour,
    #[unit(TonPerDay)]
    TonPerDay,
}

impl From<MassRateUnit> for DynamicSIUnit {
    fn from(unit: MassRateUnit) -> Self {
        match unit {
            MassRateUnit::KilogramPerSecond => KilogramPerSecond {}.into(),
            MassRateUnit::GramPerSecond => GramPerSecond {}.into(),
            MassRateUnit::GramPerMinute => GramPerMinute {}.into(),
            MassRateUnit::GramPerHour => GramPerHour {}.into(),
            MassRateUnit::GramPerDay => GramPerDay {}.into(),
            MassRateUnit::CaratPerSecond => CaratPerSecond {}.into(),
            MassRateUnit::GrainPerSecond => GrainPerSecond {}.into(),
            MassRateUnit::HundredweightLongPerSecond => HundredweightLongPerSecond {}.into(),
            MassRateUnit::HundredweightShortPerSecond => HundredweightShortPerSecond {}.into(),
            MassRateUnit::OuncePerSecond => OuncePerSecond {}.into(),
            MassRateUnit::OunceTroyPerSecond => OunceTroyPerSecond {}.into(),
            MassRateUnit::PennyweightPerSecond => PennyweightPerSecond {}.into(),
            MassRateUnit::PoundPerSecond => PoundPerSecond {}.into(),
            MassRateUnit::PoundPerMinute => PoundPerMinute {}.into(),
            MassRateUnit::PoundPerHour => PoundPerHour {}.into(),
            MassRateUnit::PoundPerDay => PoundPerDay {}.into(),
            MassRateUnit::PoundTroyPerSecond => PoundTroyPerSecond {}.into(),
            MassRateUnit::SlugPerSecond => SlugPerSecond {}.into(),
            MassRateUnit::TonAssayPerSecond => TonAssayPerSecond {}.into(),
            MassRateUnit::TonLongPerSecond => TonLongPerSecond {}.into(),
            MassRateUnit::TonShortPerSecond => TonShortPerSecond {}.into(),
            MassRateUnit::TonShortPerHour => TonShortPerHour {}.into(),
            MassRateUnit::TonPerSecond => TonPerSecond {}.into(),
            MassRateUnit::TonPerMinute => TonPerMinute {}.into(),
            MassRateUnit::TonPerHour => TonPerHour {}.into(),
            MassRateUnit::TonPerDay => TonPerDay {}.into(),
        }
    }
}

/// The units of `VolumeRate`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = VolumeRate<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum VolumeRateUnit {
    #[default]
    #[unit(CubicMetrePerSecond)]
    CubicMetrePerSecond,
    #[unit(CubicMetrePerMinute)]
    CubicMetrePerMinute,
    #[unit(CubicMetrePerHour)]
    CubicMetrePerHour,
    #[unit(LitrePerSecond)]
    LitrePerSecond,
    #[unit(LitrePerMinute)]
    LitrePerMinute,
    #[unit(AcreFootPerSecond)]
    AcreFootPerSecond,
    #[unit(BarrelPerSecond)]
    BarrelPerSecond,
    #[unit(BushelPerSecond)]
    BushelPerSecond,
    #[unit(CordPerSecond)]
    CordPerSecond,
    #[unit(CubicFootPerSecond)]
    CubicFootPerSecond,
    #[unit(CubicFootPerMinute)]
    CubicFootPerMinute,
    #[unit(CubicInchPerSecond)]
    CubicInchPerSecond,
    #[unit(CubicInchPerMinute)]
    CubicInchPerMinute,
    #[unit(CubicMilePerSecond)]
    CubicMilePerSecond,
    #[unit(CubicYardPerSecond)]
    CubicYardPerSecond,
    #[unit(CubicYardPerMinute)]
    CubicYardPerMinute,
    #[unit(CupPerSecond)]
    CupPerSecond,
    #[unit(FluidOuncePerSecond)]
    FluidOuncePerSecond,
    #[unit(FluidOunceImperialPerSecond)]
    FluidOunceImperialPerSecond,
    #[unit(GallonImperialPerSecond)]
    GallonImperialPerSecond,
    #[unit(GallonPerSecond)]
    GallonPerSecond,
    #[unit(GallonPerMinute)]
    GallonPerMinute,
    #[unit(GallonPerDay)]
    GallonPerDay,
    #[unit(GillImperialPerSecond)]
    GillImperialPerSecond,
    #[unit(GillPerSecond)]
    GillPerSecond,
    #[unit(PeckPerSecond)]
    PeckPerSecond,
    #[unit(PintDryPerSecond)]
    PintDryPerSecond,
    #[unit(PintLiquidPerSecond)]
    PintLiquidPerSecond,
    #[unit(QuartDryPerSecond)]
    QuartDryPerSecond,
    #[unit(QuartLiquidPerSecond)]
    QuartLiquidPerSecond,
    #[unit(SterePerSecond)]
    SterePerSecond,
    #[unit(TablespoonPerSecond)]
    TablespoonPerSecond,
    #[unit(TeaspoonPerSecond)]
    TeaspoonPerSecond,
    #[unit(RegisterTonPerSecond)]
    RegisterTonPerSecond,
}

impl From<VolumeRateUnit> for DynamicSIUnit {
    fn from(unit: VolumeRateUnit) -> Self {
        match unit {
            VolumeRateUnit::CubicMetrePerSecond => CubicMetrePerSecond {}.into(),
            VolumeRateUnit::CubicMetrePerMinute => CubicMetrePerMinute {}.into(),
            VolumeRateUnit::CubicMetrePerHour => CubicMetrePerHour {}.into(),
            VolumeRateUnit::LitrePerSecond => LitrePerSecond {}.into(),
            VolumeRateUnit::LitrePerMinute => LitrePerMinute {}.into(),
            VolumeRateUnit::AcreFootPerSecond => AcreFootPerSecond {}.into(),
            VolumeRateUnit::BarrelPerSecond => BarrelPerSecond {}.into(),
            VolumeRateUnit::BushelPerSecond => BushelPerSecond {}.into(),
            VolumeRateUnit::CordPerSecond => CordPerSecond {}.into(),
            VolumeRateUnit::CubicFootPerSecond => CubicFootPerSecond {}.into(),
            VolumeRateUnit::CubicFootPerMinute => CubicFootPerMinute {}.into(),
            VolumeRateUnit::CubicInchPerSecond => CubicInchPerSecond {}.into(),
            VolumeRateUnit::CubicInchPerMinute => CubicInchPerMinute {}.into(),
            VolumeRateUnit::CubicMilePerSecond => CubicMilePerSecond {}.into(),
            VolumeRateUnit::CubicYardPerSecond => CubicYardPerSecond {}.into(),
            VolumeRateUnit::CubicYardPerMinute => CubicYardPerMinute {}.into(),
            VolumeRateUnit::CupPerSecond => CupPerSecond {}.into(),
            VolumeRateUnit::FluidOuncePerSecond => FluidOuncePerSecond {}.into(),
            VolumeRateUnit::FluidOunceImperialPerSecond => FluidOunceImperialPerSecond {}.into(),
            VolumeRateUnit::GallonImperialPerSecond => GallonImperialPerSecond {}.into(),
            VolumeRateUnit::GallonPerSecond => GallonPerSecond {}.into(),
            VolumeRateUnit::GallonPerMinute => GallonPerMinute {}.into(),
            VolumeRateUnit::GallonPerDay => GallonPerDay {}.into(),
            VolumeRateUnit::GillImperialPerSecond => GillImperialPerSecond {}.into(),
            VolumeRateUnit::GillPerSecond => GillPerSecond {}.into(),
            VolumeRateUnit::PeckPerSecond => PeckPerSecond {}.into(),
            VolumeRateUnit::PintDryPerSecond => PintDryPerSecond {}.into(),
            VolumeRateUnit::PintLiquidPerSecond => PintLiquidPerSecond {}.into(),
            VolumeRateUnit::QuartDryPerSecond => QuartDryPerSecond {}.into(),
            VolumeRateUnit::QuartLiquidPerSecond => QuartLiquidPerSecond {}.into(),
            VolumeRateUnit::SterePerSecond => SterePerSecond {}.into(),
            VolumeRateUnit::TablespoonPerSecond => TablespoonPerSecond {}.into(),
            VolumeRateUnit::TeaspoonPerSecond => TeaspoonPerSecond {}.into(),
            VolumeRateUnit::RegisterTonPerSecond => RegisterTonPerSecond {}.into(),
        }
    }
}

/// The units of `MassFlux`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassFlux<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MassFluxUnit {
    #[default]
    #[unit(KilogramPerSquareMetreSecond)]
    KilogramPerSquareMetreSecond,
}

impl From<MassFluxUnit> for DynamicSIUnit {
    fn from(unit: MassFluxUnit) -> Self {
        match unit {
            MassFluxUnit::KilogramPerSquareMetreSecond => KilogramPerSquareMetreSecond {}.into(),
        }
    }
}

/// The units of `MassPerEnergy`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassPerEnergy<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MassPerEnergyUnit {
    #[default]
    #[unit(KilogramPerJoule)]
    KilogramPerJoule,
    #[unit(GramPerJoule)]
    GramPerJoule,
    #[unit(PoundPerJoule)]
    PoundPerJoule,
    #[unit(PoundPerWattHour)]
    PoundPerWattHour,
    #[unit(GramPerWattHour)]
    GramPerWattHour,
}

impl From<MassPerEnergyUnit> for DynamicSIUnit {
    fn from(unit: MassPerEnergyUnit) -> Self {
        match unit {
            MassPerEnergyUnit::KilogramPerJoule => KilogramPerJoule {}.into(),
            MassPerEnergyUnit::GramPerJoule => GramPerJoule {}.into(),
            MassPerEnergyUnit::PoundPerJoule => PoundPerJoule {}.into(),
            MassPerEnergyUnit::PoundPerWattHour => PoundPerWattHour {}.into(),
            MassPerEnergyUnit::GramPerWattHour => GramPerWattHour {}.into(),
        }
    }
}

/// The units of `DiffusionCoefficient`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = DiffusionCoefficient<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum DiffusionCoefficientUnit {
    #[default]
    #[unit(DiffusionCoefficientSquareMetrePerSecond)]
    SquareMetrePerSecond,
    #[unit(DiffusionCoefficientStokes)]
    Stokes,
}

impl From<DiffusionCoefficientUnit> for DynamicSIUnit {
    fn from(unit: DiffusionCoefficientUnit) -> Self {
        match unit {
            DiffusionCoefficientUnit::SquareMetrePerSecond => {
                DiffusionCoefficientSquareMetrePerSecond {}.into()
            }
            DiffusionCoefficientUnit::Stokes => DiffusionCoefficientStokes {}.into(),
        }
    }
}
