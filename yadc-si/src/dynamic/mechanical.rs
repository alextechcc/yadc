//! The units of `mechanical`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::dynamic::DynamicSIUnit;
use crate::mechanical::units::*;
use crate::mechanical::*;
use core::marker::Destruct;
use core::ops::{Add, Div, Mul, Sub};

/// The units of `Length`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum LengthUnit {
    #[default]
    #[unit(Metre)]
    Metre,
    #[unit(Angstrom)]
    Angstrom,
    #[unit(BohrRadius)]
    BohrRadius,
    #[unit(AtomicUnitOfLength)]
    AtomicUnitOfLength,
    #[unit(AstronomicalUnit)]
    AstronomicalUnit,
    #[unit(Chain)]
    Chain,
    #[unit(DataMile)]
    DataMile,
    #[unit(Fathom)]
    Fathom,
    #[unit(Fermi)]
    Fermi,
    #[unit(Foot)]
    Foot,
    #[unit(FootSurvey)]
    FootSurvey,
    #[unit(Inch)]
    Inch,
    #[unit(LightYear)]
    LightYear,
    #[unit(Micron)]
    Micron,
    #[unit(Mil)]
    Mil,
    #[unit(Mile)]
    Mile,
    #[unit(MileSurvey)]
    MileSurvey,
    #[unit(NauticalMile)]
    NauticalMile,
    #[unit(Parsec)]
    Parsec,
    #[unit(PicaComputer)]
    PicaComputer,
    #[unit(PicaPrinters)]
    PicaPrinters,
    #[unit(PointComputer)]
    PointComputer,
    #[unit(PointPrinters)]
    PointPrinters,
    #[unit(Rod)]
    Rod,
    #[unit(Yard)]
    Yard,
    #[unit(Leuga)]
    Leuga,
    #[unit(MillePassus)]
    MillePassus,
    #[unit(Stadium)]
    Stadium,
    #[unit(Actus)]
    Actus,
    #[unit(Decempeda)]
    Decempeda,
    #[unit(Passus)]
    Passus,
    #[unit(Gradus)]
    Gradus,
    #[unit(Cubitum)]
    Cubitum,
    #[unit(Palmipes)]
    Palmipes,
    #[unit(Pes)]
    Pes,
    #[unit(PalmusMaior)]
    PalmusMaior,
    #[unit(Palmus)]
    Palmus,
    #[unit(LengthUncia)]
    Uncia,
    #[unit(Digitus)]
    Digitus,
}

impl From<LengthUnit> for DynamicSIUnit {
    fn from(unit: LengthUnit) -> Self {
        match unit {
            LengthUnit::Metre => Metre {}.into(),
            LengthUnit::Angstrom => Angstrom {}.into(),
            LengthUnit::BohrRadius => BohrRadius {}.into(),
            LengthUnit::AtomicUnitOfLength => AtomicUnitOfLength {}.into(),
            LengthUnit::AstronomicalUnit => AstronomicalUnit {}.into(),
            LengthUnit::Chain => Chain {}.into(),
            LengthUnit::DataMile => DataMile {}.into(),
            LengthUnit::Fathom => Fathom {}.into(),
            LengthUnit::Fermi => Fermi {}.into(),
            LengthUnit::Foot => Foot {}.into(),
            LengthUnit::FootSurvey => FootSurvey {}.into(),
            LengthUnit::Inch => Inch {}.into(),
            LengthUnit::LightYear => LightYear {}.into(),
            LengthUnit::Micron => Micron {}.into(),
            LengthUnit::Mil => Mil {}.into(),
            LengthUnit::Mile => Mile {}.into(),
            LengthUnit::MileSurvey => MileSurvey {}.into(),
            LengthUnit::NauticalMile => NauticalMile {}.into(),
            LengthUnit::Parsec => Parsec {}.into(),
            LengthUnit::PicaComputer => PicaComputer {}.into(),
            LengthUnit::PicaPrinters => PicaPrinters {}.into(),
            LengthUnit::PointComputer => PointComputer {}.into(),
            LengthUnit::PointPrinters => PointPrinters {}.into(),
            LengthUnit::Rod => Rod {}.into(),
            LengthUnit::Yard => Yard {}.into(),
            LengthUnit::Leuga => Leuga {}.into(),
            LengthUnit::MillePassus => MillePassus {}.into(),
            LengthUnit::Stadium => Stadium {}.into(),
            LengthUnit::Actus => Actus {}.into(),
            LengthUnit::Decempeda => Decempeda {}.into(),
            LengthUnit::Passus => Passus {}.into(),
            LengthUnit::Gradus => Gradus {}.into(),
            LengthUnit::Cubitum => Cubitum {}.into(),
            LengthUnit::Palmipes => Palmipes {}.into(),
            LengthUnit::Pes => Pes {}.into(),
            LengthUnit::PalmusMaior => PalmusMaior {}.into(),
            LengthUnit::Palmus => Palmus {}.into(),
            LengthUnit::Uncia => LengthUncia {}.into(),
            LengthUnit::Digitus => Digitus {}.into(),
        }
    }
}

/// The units of `Mass`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Mass<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MassUnit {
    #[default]
    #[unit(Kilogram)]
    Kilogram,
    #[unit(Gram)]
    Gram,
    #[unit(Carat)]
    Carat,
    #[unit(Dalton)]
    Dalton,
    #[unit(ElectronMass)]
    ElectronMass,
    #[unit(ProtonMass)]
    ProtonMass,
    #[unit(Grain)]
    Grain,
    #[unit(HundredweightLong)]
    HundredweightLong,
    #[unit(HundredweightShort)]
    HundredweightShort,
    #[unit(Ounce)]
    Ounce,
    #[unit(OunceTroy)]
    OunceTroy,
    #[unit(Pennyweight)]
    Pennyweight,
    #[unit(Pound)]
    Pound,
    #[unit(PoundTroy)]
    PoundTroy,
    #[unit(Slug)]
    Slug,
    #[unit(TonAssay)]
    TonAssay,
    #[unit(TonLong)]
    TonLong,
    #[unit(TonShort)]
    TonShort,
    #[unit(Ton)]
    Ton,
    #[unit(Libra)]
    Libra,
    #[unit(MassDeunx)]
    Deunx,
    #[unit(MassDextans)]
    Dextans,
    #[unit(MassDodrans)]
    Dodrans,
    #[unit(MassBes)]
    Bes,
    #[unit(MassSeptunx)]
    Septunx,
    #[unit(MassSemis)]
    Semis,
    #[unit(MassQuincunx)]
    Quincunx,
    #[unit(MassTriens)]
    Triens,
    #[unit(QuadransTeruncius)]
    QuadransTeruncius,
    #[unit(MassSextans)]
    Sextans,
    #[unit(Sescuncia)]
    Sescuncia,
    #[unit(MassUncia)]
    Uncia,
    #[unit(Semuncia)]
    Semuncia,
    #[unit(Duella)]
    Duella,
    #[unit(MassSicilicus)]
    Sicilicus,
    #[unit(MassSextula)]
    Sextula,
    #[unit(Semisextula)]
    Semisextula,
    #[unit(MassScrupulum)]
    Scrupulum,
    #[unit(Obolus)]
    Obolus,
    #[unit(Siliqua)]
    Siliqua,
}

impl From<MassUnit> for DynamicSIUnit {
    fn from(unit: MassUnit) -> Self {
        match unit {
            MassUnit::Kilogram => Kilogram {}.into(),
            MassUnit::Gram => Gram {}.into(),
            MassUnit::Carat => Carat {}.into(),
            MassUnit::Dalton => Dalton {}.into(),
            MassUnit::ElectronMass => ElectronMass {}.into(),
            MassUnit::ProtonMass => ProtonMass {}.into(),
            MassUnit::Grain => Grain {}.into(),
            MassUnit::HundredweightLong => HundredweightLong {}.into(),
            MassUnit::HundredweightShort => HundredweightShort {}.into(),
            MassUnit::Ounce => Ounce {}.into(),
            MassUnit::OunceTroy => OunceTroy {}.into(),
            MassUnit::Pennyweight => Pennyweight {}.into(),
            MassUnit::Pound => Pound {}.into(),
            MassUnit::PoundTroy => PoundTroy {}.into(),
            MassUnit::Slug => Slug {}.into(),
            MassUnit::TonAssay => TonAssay {}.into(),
            MassUnit::TonLong => TonLong {}.into(),
            MassUnit::TonShort => TonShort {}.into(),
            MassUnit::Ton => Ton {}.into(),
            MassUnit::Libra => Libra {}.into(),
            MassUnit::Deunx => MassDeunx {}.into(),
            MassUnit::Dextans => MassDextans {}.into(),
            MassUnit::Dodrans => MassDodrans {}.into(),
            MassUnit::Bes => MassBes {}.into(),
            MassUnit::Septunx => MassSeptunx {}.into(),
            MassUnit::Semis => MassSemis {}.into(),
            MassUnit::Quincunx => MassQuincunx {}.into(),
            MassUnit::Triens => MassTriens {}.into(),
            MassUnit::QuadransTeruncius => QuadransTeruncius {}.into(),
            MassUnit::Sextans => MassSextans {}.into(),
            MassUnit::Sescuncia => Sescuncia {}.into(),
            MassUnit::Uncia => MassUncia {}.into(),
            MassUnit::Semuncia => Semuncia {}.into(),
            MassUnit::Duella => Duella {}.into(),
            MassUnit::Sicilicus => MassSicilicus {}.into(),
            MassUnit::Sextula => MassSextula {}.into(),
            MassUnit::Semisextula => Semisextula {}.into(),
            MassUnit::Scrupulum => MassScrupulum {}.into(),
            MassUnit::Obolus => Obolus {}.into(),
            MassUnit::Siliqua => Siliqua {}.into(),
        }
    }
}

/// The units of `Time`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Time<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum TimeUnit {
    #[default]
    #[unit(Second)]
    Second,
    #[unit(SecondSidereal)]
    SecondSidereal,
    #[unit(Day)]
    Day,
    #[unit(DaySidereal)]
    DaySidereal,
    #[unit(Hour)]
    Hour,
    #[unit(HourSidereal)]
    HourSidereal,
    #[unit(Minute)]
    Minute,
    #[unit(Shake)]
    Shake,
    #[unit(Year)]
    Year,
    #[unit(YearSidereal)]
    YearSidereal,
    #[unit(YearTropical)]
    YearTropical,
}

impl From<TimeUnit> for DynamicSIUnit {
    fn from(unit: TimeUnit) -> Self {
        match unit {
            TimeUnit::Second => Second {}.into(),
            TimeUnit::SecondSidereal => SecondSidereal {}.into(),
            TimeUnit::Day => Day {}.into(),
            TimeUnit::DaySidereal => DaySidereal {}.into(),
            TimeUnit::Hour => Hour {}.into(),
            TimeUnit::HourSidereal => HourSidereal {}.into(),
            TimeUnit::Minute => Minute {}.into(),
            TimeUnit::Shake => Shake {}.into(),
            TimeUnit::Year => Year {}.into(),
            TimeUnit::YearSidereal => YearSidereal {}.into(),
            TimeUnit::YearTropical => YearTropical {}.into(),
        }
    }
}

/// The units of `Instant`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Instant<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum InstantUnit {
    #[default]
    #[unit(UnixSecond)]
    UnixSecond,
    #[unit(UnixMillisecond)]
    UnixMillisecond,
    #[unit(GpsSecond)]
    GpsSecond,
    #[unit(JulianDate)]
    JulianDate,
    #[unit(ModifiedJulianDate)]
    ModifiedJulianDate,
}

impl From<InstantUnit> for DynamicSIUnit {
    fn from(unit: InstantUnit) -> Self {
        match unit {
            InstantUnit::UnixSecond => UnixSecond {}.into(),
            InstantUnit::UnixMillisecond => UnixMillisecond {}.into(),
            InstantUnit::GpsSecond => GpsSecond {}.into(),
            InstantUnit::JulianDate => JulianDate {}.into(),
            InstantUnit::ModifiedJulianDate => ModifiedJulianDate {}.into(),
        }
    }
}

/// The units of `Area`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Area<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum AreaUnit {
    #[default]
    #[unit(SquareMetre)]
    SquareMetre,
    #[unit(Acre)]
    Acre,
    #[unit(Are)]
    Are,
    #[unit(Barn)]
    Barn,
    #[unit(CircularMil)]
    CircularMil,
    #[unit(Hectare)]
    Hectare,
    #[unit(SquareFoot)]
    SquareFoot,
    #[unit(SquareInch)]
    SquareInch,
    #[unit(SquareMile)]
    SquareMile,
    #[unit(SquareYard)]
    SquareYard,
    #[unit(Saltus)]
    Saltus,
    #[unit(Centuria)]
    Centuria,
    #[unit(Heredium)]
    Heredium,
    #[unit(Jugerum)]
    Jugerum,
    #[unit(AreaDeunx)]
    Deunx,
    #[unit(AreaDextans)]
    Dextans,
    #[unit(AreaDodrans)]
    Dodrans,
    #[unit(AreaBes)]
    Bes,
    #[unit(AreaSeptunx)]
    Septunx,
    #[unit(AreaSemis)]
    Semis,
    #[unit(ActusQuadratus)]
    ActusQuadratus,
    #[unit(AreaQuincunx)]
    Quincunx,
    #[unit(AreaTriens)]
    Triens,
    #[unit(Quadrans)]
    Quadrans,
    #[unit(AreaSextans)]
    Sextans,
    #[unit(Clima)]
    Clima,
    #[unit(AreaUncia)]
    Uncia,
    #[unit(Semiuncia)]
    Semiuncia,
    #[unit(AreaSicilicus)]
    Sicilicus,
    #[unit(ActusSimplex)]
    ActusSimplex,
    #[unit(AreaSextula)]
    Sextula,
    #[unit(DuoScrupulum)]
    DuoScrupulum,
    #[unit(AreaScrupulum)]
    Scrupulum,
    #[unit(DimidiumScrupulum)]
    DimidiumScrupulum,
    #[unit(PesQuadratus)]
    PesQuadratus,
}

impl From<AreaUnit> for DynamicSIUnit {
    fn from(unit: AreaUnit) -> Self {
        match unit {
            AreaUnit::SquareMetre => SquareMetre {}.into(),
            AreaUnit::Acre => Acre {}.into(),
            AreaUnit::Are => Are {}.into(),
            AreaUnit::Barn => Barn {}.into(),
            AreaUnit::CircularMil => CircularMil {}.into(),
            AreaUnit::Hectare => Hectare {}.into(),
            AreaUnit::SquareFoot => SquareFoot {}.into(),
            AreaUnit::SquareInch => SquareInch {}.into(),
            AreaUnit::SquareMile => SquareMile {}.into(),
            AreaUnit::SquareYard => SquareYard {}.into(),
            AreaUnit::Saltus => Saltus {}.into(),
            AreaUnit::Centuria => Centuria {}.into(),
            AreaUnit::Heredium => Heredium {}.into(),
            AreaUnit::Jugerum => Jugerum {}.into(),
            AreaUnit::Deunx => AreaDeunx {}.into(),
            AreaUnit::Dextans => AreaDextans {}.into(),
            AreaUnit::Dodrans => AreaDodrans {}.into(),
            AreaUnit::Bes => AreaBes {}.into(),
            AreaUnit::Septunx => AreaSeptunx {}.into(),
            AreaUnit::Semis => AreaSemis {}.into(),
            AreaUnit::ActusQuadratus => ActusQuadratus {}.into(),
            AreaUnit::Quincunx => AreaQuincunx {}.into(),
            AreaUnit::Triens => AreaTriens {}.into(),
            AreaUnit::Quadrans => Quadrans {}.into(),
            AreaUnit::Sextans => AreaSextans {}.into(),
            AreaUnit::Clima => Clima {}.into(),
            AreaUnit::Uncia => AreaUncia {}.into(),
            AreaUnit::Semiuncia => Semiuncia {}.into(),
            AreaUnit::Sicilicus => AreaSicilicus {}.into(),
            AreaUnit::ActusSimplex => ActusSimplex {}.into(),
            AreaUnit::Sextula => AreaSextula {}.into(),
            AreaUnit::DuoScrupulum => DuoScrupulum {}.into(),
            AreaUnit::Scrupulum => AreaScrupulum {}.into(),
            AreaUnit::DimidiumScrupulum => DimidiumScrupulum {}.into(),
            AreaUnit::PesQuadratus => PesQuadratus {}.into(),
        }
    }
}

/// The units of `Volume`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Volume<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum VolumeUnit {
    #[default]
    #[unit(CubicMetre)]
    CubicMetre,
    #[unit(AcreFoot)]
    AcreFoot,
    #[unit(Barrel)]
    Barrel,
    #[unit(Bushel)]
    Bushel,
    #[unit(Cord)]
    Cord,
    #[unit(CubicFoot)]
    CubicFoot,
    #[unit(CubicInch)]
    CubicInch,
    #[unit(CubicMile)]
    CubicMile,
    #[unit(CubicYard)]
    CubicYard,
    #[unit(Cup)]
    Cup,
    #[unit(FluidOunce)]
    FluidOunce,
    #[unit(FluidOunceImperial)]
    FluidOunceImperial,
    #[unit(GallonImperial)]
    GallonImperial,
    #[unit(Gallon)]
    Gallon,
    #[unit(GillImperial)]
    GillImperial,
    #[unit(Gill)]
    Gill,
    #[unit(Litre)]
    Litre,
    #[unit(Peck)]
    Peck,
    #[unit(PintDry)]
    PintDry,
    #[unit(PintLiquid)]
    PintLiquid,
    #[unit(QuartDry)]
    QuartDry,
    #[unit(QuartLiquid)]
    QuartLiquid,
    #[unit(Stere)]
    Stere,
    #[unit(Tablespoon)]
    Tablespoon,
    #[unit(Teaspoon)]
    Teaspoon,
    #[unit(RegisterTon)]
    RegisterTon,
    #[unit(Culeus)]
    Culeus,
    #[unit(AmphoraQuadrantal)]
    AmphoraQuadrantal,
    #[unit(Urna)]
    Urna,
    #[unit(ModiusCastrensis)]
    ModiusCastrensis,
    #[unit(Modius)]
    Modius,
    #[unit(Semimodius)]
    Semimodius,
    #[unit(Congius)]
    Congius,
    #[unit(Sextarius)]
    Sextarius,
    #[unit(Hemina)]
    Hemina,
    #[unit(Quartarius)]
    Quartarius,
    #[unit(Acetabulum)]
    Acetabulum,
    #[unit(Cyathus)]
    Cyathus,
    #[unit(Ligula)]
    Ligula,
}

impl From<VolumeUnit> for DynamicSIUnit {
    fn from(unit: VolumeUnit) -> Self {
        match unit {
            VolumeUnit::CubicMetre => CubicMetre {}.into(),
            VolumeUnit::AcreFoot => AcreFoot {}.into(),
            VolumeUnit::Barrel => Barrel {}.into(),
            VolumeUnit::Bushel => Bushel {}.into(),
            VolumeUnit::Cord => Cord {}.into(),
            VolumeUnit::CubicFoot => CubicFoot {}.into(),
            VolumeUnit::CubicInch => CubicInch {}.into(),
            VolumeUnit::CubicMile => CubicMile {}.into(),
            VolumeUnit::CubicYard => CubicYard {}.into(),
            VolumeUnit::Cup => Cup {}.into(),
            VolumeUnit::FluidOunce => FluidOunce {}.into(),
            VolumeUnit::FluidOunceImperial => FluidOunceImperial {}.into(),
            VolumeUnit::GallonImperial => GallonImperial {}.into(),
            VolumeUnit::Gallon => Gallon {}.into(),
            VolumeUnit::GillImperial => GillImperial {}.into(),
            VolumeUnit::Gill => Gill {}.into(),
            VolumeUnit::Litre => Litre {}.into(),
            VolumeUnit::Peck => Peck {}.into(),
            VolumeUnit::PintDry => PintDry {}.into(),
            VolumeUnit::PintLiquid => PintLiquid {}.into(),
            VolumeUnit::QuartDry => QuartDry {}.into(),
            VolumeUnit::QuartLiquid => QuartLiquid {}.into(),
            VolumeUnit::Stere => Stere {}.into(),
            VolumeUnit::Tablespoon => Tablespoon {}.into(),
            VolumeUnit::Teaspoon => Teaspoon {}.into(),
            VolumeUnit::RegisterTon => RegisterTon {}.into(),
            VolumeUnit::Culeus => Culeus {}.into(),
            VolumeUnit::AmphoraQuadrantal => AmphoraQuadrantal {}.into(),
            VolumeUnit::Urna => Urna {}.into(),
            VolumeUnit::ModiusCastrensis => ModiusCastrensis {}.into(),
            VolumeUnit::Modius => Modius {}.into(),
            VolumeUnit::Semimodius => Semimodius {}.into(),
            VolumeUnit::Congius => Congius {}.into(),
            VolumeUnit::Sextarius => Sextarius {}.into(),
            VolumeUnit::Hemina => Hemina {}.into(),
            VolumeUnit::Quartarius => Quartarius {}.into(),
            VolumeUnit::Acetabulum => Acetabulum {}.into(),
            VolumeUnit::Cyathus => Cyathus {}.into(),
            VolumeUnit::Ligula => Ligula {}.into(),
        }
    }
}

/// The units of `Velocity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Velocity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum VelocityUnit {
    #[default]
    #[unit(MetrePerSecond)]
    MetrePerSecond,
    #[unit(FootPerHour)]
    FootPerHour,
    #[unit(FootPerMinute)]
    FootPerMinute,
    #[unit(FootPerSecond)]
    FootPerSecond,
    #[unit(InchPerSecond)]
    InchPerSecond,
    #[unit(InchPerMinute)]
    InchPerMinute,
    #[unit(Knot)]
    Knot,
    #[unit(MilePerHour)]
    MilePerHour,
    #[unit(MilePerMinute)]
    MilePerMinute,
    #[unit(MilePerSecond)]
    MilePerSecond,
    #[unit(AtomicUnitOfVelocity)]
    AtomicUnitOfVelocity,
    #[unit(NaturalUnitOfVelocity)]
    NaturalUnitOfVelocity,
    #[unit(SpeedOfLight)]
    SpeedOfLight,
    #[unit(KilometrePerHour)]
    KilometrePerHour,
    #[unit(MillimetrePerMinute)]
    MillimetrePerMinute,
}

impl From<VelocityUnit> for DynamicSIUnit {
    fn from(unit: VelocityUnit) -> Self {
        match unit {
            VelocityUnit::MetrePerSecond => MetrePerSecond {}.into(),
            VelocityUnit::FootPerHour => FootPerHour {}.into(),
            VelocityUnit::FootPerMinute => FootPerMinute {}.into(),
            VelocityUnit::FootPerSecond => FootPerSecond {}.into(),
            VelocityUnit::InchPerSecond => InchPerSecond {}.into(),
            VelocityUnit::InchPerMinute => InchPerMinute {}.into(),
            VelocityUnit::Knot => Knot {}.into(),
            VelocityUnit::MilePerHour => MilePerHour {}.into(),
            VelocityUnit::MilePerMinute => MilePerMinute {}.into(),
            VelocityUnit::MilePerSecond => MilePerSecond {}.into(),
            VelocityUnit::AtomicUnitOfVelocity => AtomicUnitOfVelocity {}.into(),
            VelocityUnit::NaturalUnitOfVelocity => NaturalUnitOfVelocity {}.into(),
            VelocityUnit::SpeedOfLight => SpeedOfLight {}.into(),
            VelocityUnit::KilometrePerHour => KilometrePerHour {}.into(),
            VelocityUnit::MillimetrePerMinute => MillimetrePerMinute {}.into(),
        }
    }
}

/// The units of `Acceleration`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Acceleration<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum AccelerationUnit {
    #[default]
    #[unit(MetrePerSecondSquared)]
    MetrePerSecondSquared,
    #[unit(FootPerSecondSquared)]
    FootPerSecondSquared,
    #[unit(Galileo)]
    Galileo,
    #[unit(InchPerSecondSquared)]
    InchPerSecondSquared,
    #[unit(StandardGravity)]
    StandardGravity,
    #[unit(MillimetrePerMinuteSquared)]
    MillimetrePerMinuteSquared,
}

impl From<AccelerationUnit> for DynamicSIUnit {
    fn from(unit: AccelerationUnit) -> Self {
        match unit {
            AccelerationUnit::MetrePerSecondSquared => MetrePerSecondSquared {}.into(),
            AccelerationUnit::FootPerSecondSquared => FootPerSecondSquared {}.into(),
            AccelerationUnit::Galileo => Galileo {}.into(),
            AccelerationUnit::InchPerSecondSquared => InchPerSecondSquared {}.into(),
            AccelerationUnit::StandardGravity => StandardGravity {}.into(),
            AccelerationUnit::MillimetrePerMinuteSquared => MillimetrePerMinuteSquared {}.into(),
        }
    }
}

/// The units of `Jerk`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Jerk<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum JerkUnit {
    #[default]
    #[unit(MetrePerSecondCubed)]
    MetrePerSecondCubed,
    #[unit(FootPerSecondCubed)]
    FootPerSecondCubed,
    #[unit(InchPerSecondCubed)]
    InchPerSecondCubed,
    #[unit(KilometrePerMinuteCubed)]
    KilometrePerMinuteCubed,
}

impl From<JerkUnit> for DynamicSIUnit {
    fn from(unit: JerkUnit) -> Self {
        match unit {
            JerkUnit::MetrePerSecondCubed => MetrePerSecondCubed {}.into(),
            JerkUnit::FootPerSecondCubed => FootPerSecondCubed {}.into(),
            JerkUnit::InchPerSecondCubed => InchPerSecondCubed {}.into(),
            JerkUnit::KilometrePerMinuteCubed => KilometrePerMinuteCubed {}.into(),
        }
    }
}

/// The units of `Absement`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Absement<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum AbsementUnit {
    #[default]
    #[unit(MetreSecond)]
    MetreSecond,
    #[unit(FootSecond)]
    FootSecond,
    #[unit(InchSecond)]
    InchSecond,
    #[unit(KilometreHour)]
    KilometreHour,
}

impl From<AbsementUnit> for DynamicSIUnit {
    fn from(unit: AbsementUnit) -> Self {
        match unit {
            AbsementUnit::MetreSecond => MetreSecond {}.into(),
            AbsementUnit::FootSecond => FootSecond {}.into(),
            AbsementUnit::InchSecond => InchSecond {}.into(),
            AbsementUnit::KilometreHour => KilometreHour {}.into(),
        }
    }
}

/// The units of `InverseVelocity`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = InverseVelocity<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum InverseVelocityUnit {
    #[default]
    #[unit(SecondPerMetre)]
    SecondPerMetre,
    #[unit(HourPerFoot)]
    HourPerFoot,
    #[unit(MinutePerFoot)]
    MinutePerFoot,
    #[unit(SecondPerFoot)]
    SecondPerFoot,
    #[unit(SecondPerInch)]
    SecondPerInch,
    #[unit(HourPerMile)]
    HourPerMile,
    #[unit(MinutePerMile)]
    MinutePerMile,
    #[unit(SecondPerMile)]
    SecondPerMile,
    #[unit(MinutePerKilometre)]
    MinutePerKilometre,
    #[unit(HourPerKilometre)]
    HourPerKilometre,
}

impl From<InverseVelocityUnit> for DynamicSIUnit {
    fn from(unit: InverseVelocityUnit) -> Self {
        match unit {
            InverseVelocityUnit::SecondPerMetre => SecondPerMetre {}.into(),
            InverseVelocityUnit::HourPerFoot => HourPerFoot {}.into(),
            InverseVelocityUnit::MinutePerFoot => MinutePerFoot {}.into(),
            InverseVelocityUnit::SecondPerFoot => SecondPerFoot {}.into(),
            InverseVelocityUnit::SecondPerInch => SecondPerInch {}.into(),
            InverseVelocityUnit::HourPerMile => HourPerMile {}.into(),
            InverseVelocityUnit::MinutePerMile => MinutePerMile {}.into(),
            InverseVelocityUnit::SecondPerMile => SecondPerMile {}.into(),
            InverseVelocityUnit::MinutePerKilometre => MinutePerKilometre {}.into(),
            InverseVelocityUnit::HourPerKilometre => HourPerKilometre {}.into(),
        }
    }
}

/// The units of `Force`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Force<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ForceUnit {
    #[default]
    #[unit(Newton)]
    Newton,
    #[unit(Dyne)]
    Dyne,
    #[unit(KilogramForce)]
    KilogramForce,
    #[unit(Kip)]
    Kip,
    #[unit(OunceForce)]
    OunceForce,
    #[unit(Poundal)]
    Poundal,
    #[unit(PoundForce)]
    PoundForce,
    #[unit(TonForce)]
    TonForce,
}

impl From<ForceUnit> for DynamicSIUnit {
    fn from(unit: ForceUnit) -> Self {
        match unit {
            ForceUnit::Newton => Newton {}.into(),
            ForceUnit::Dyne => Dyne {}.into(),
            ForceUnit::KilogramForce => KilogramForce {}.into(),
            ForceUnit::Kip => Kip {}.into(),
            ForceUnit::OunceForce => OunceForce {}.into(),
            ForceUnit::Poundal => Poundal {}.into(),
            ForceUnit::PoundForce => PoundForce {}.into(),
            ForceUnit::TonForce => TonForce {}.into(),
        }
    }
}

/// The units of `Momentum`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Momentum<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MomentumUnit {
    #[default]
    #[unit(KilogramMetrePerSecond)]
    KilogramMetrePerSecond,
    #[unit(GramMetrePerSecond)]
    GramMetrePerSecond,
    #[unit(TonMetrePerSecond)]
    TonMetrePerSecond,
    #[unit(KilogramMetrePerMinute)]
    KilogramMetrePerMinute,
    #[unit(KilogramMetrePerHour)]
    KilogramMetrePerHour,
    #[unit(KilogramMetrePerDay)]
    KilogramMetrePerDay,
    #[unit(SlugFootPerSecond)]
    SlugFootPerSecond,
    #[unit(SlugInchPerSecond)]
    SlugInchPerSecond,
    #[unit(PoundFootPerSecond)]
    PoundFootPerSecond,
    #[unit(PoundInchPerSecond)]
    PoundInchPerSecond,
}

impl From<MomentumUnit> for DynamicSIUnit {
    fn from(unit: MomentumUnit) -> Self {
        match unit {
            MomentumUnit::KilogramMetrePerSecond => KilogramMetrePerSecond {}.into(),
            MomentumUnit::GramMetrePerSecond => GramMetrePerSecond {}.into(),
            MomentumUnit::TonMetrePerSecond => TonMetrePerSecond {}.into(),
            MomentumUnit::KilogramMetrePerMinute => KilogramMetrePerMinute {}.into(),
            MomentumUnit::KilogramMetrePerHour => KilogramMetrePerHour {}.into(),
            MomentumUnit::KilogramMetrePerDay => KilogramMetrePerDay {}.into(),
            MomentumUnit::SlugFootPerSecond => SlugFootPerSecond {}.into(),
            MomentumUnit::SlugInchPerSecond => SlugInchPerSecond {}.into(),
            MomentumUnit::PoundFootPerSecond => PoundFootPerSecond {}.into(),
            MomentumUnit::PoundInchPerSecond => PoundInchPerSecond {}.into(),
        }
    }
}

/// The units of `Energy`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Energy<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum EnergyUnit {
    #[default]
    #[unit(Joule)]
    Joule,
    #[unit(WattHour)]
    WattHour,
    #[unit(Electronvolt)]
    Electronvolt,
    #[unit(Hartree)]
    Hartree,
    #[unit(BtuIt)]
    BtuIt,
    #[unit(Btu)]
    Btu,
    #[unit(CalorieIt)]
    CalorieIt,
    #[unit(Calorie)]
    Calorie,
    #[unit(CalorieItNutrition)]
    CalorieItNutrition,
    #[unit(CalorieNutrition)]
    CalorieNutrition,
    #[unit(Erg)]
    Erg,
    #[unit(FootPoundal)]
    FootPoundal,
    #[unit(FootPound)]
    FootPound,
    #[unit(Quad)]
    Quad,
    #[unit(ThermEc)]
    ThermEc,
    #[unit(ThermUs)]
    ThermUs,
    #[unit(TonTnt)]
    TonTnt,
    #[unit(WattSecond)]
    WattSecond,
}

impl From<EnergyUnit> for DynamicSIUnit {
    fn from(unit: EnergyUnit) -> Self {
        match unit {
            EnergyUnit::Joule => Joule {}.into(),
            EnergyUnit::WattHour => WattHour {}.into(),
            EnergyUnit::Electronvolt => Electronvolt {}.into(),
            EnergyUnit::Hartree => Hartree {}.into(),
            EnergyUnit::BtuIt => BtuIt {}.into(),
            EnergyUnit::Btu => Btu {}.into(),
            EnergyUnit::CalorieIt => CalorieIt {}.into(),
            EnergyUnit::Calorie => Calorie {}.into(),
            EnergyUnit::CalorieItNutrition => CalorieItNutrition {}.into(),
            EnergyUnit::CalorieNutrition => CalorieNutrition {}.into(),
            EnergyUnit::Erg => Erg {}.into(),
            EnergyUnit::FootPoundal => FootPoundal {}.into(),
            EnergyUnit::FootPound => FootPound {}.into(),
            EnergyUnit::Quad => Quad {}.into(),
            EnergyUnit::ThermEc => ThermEc {}.into(),
            EnergyUnit::ThermUs => ThermUs {}.into(),
            EnergyUnit::TonTnt => TonTnt {}.into(),
            EnergyUnit::WattSecond => WattSecond {}.into(),
        }
    }
}

/// The units of `AvailableEnergy`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AvailableEnergy<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum AvailableEnergyUnit {
    #[default]
    #[unit(JoulePerKilogram)]
    JoulePerKilogram,
    #[unit(JoulePerGram)]
    JoulePerGram,
    #[unit(BtuItPerPound)]
    BtuItPerPound,
    #[unit(BtuPerPound)]
    BtuPerPound,
    #[unit(CalorieItPerGram)]
    CalorieItPerGram,
    #[unit(CaloriePerGram)]
    CaloriePerGram,
}

impl From<AvailableEnergyUnit> for DynamicSIUnit {
    fn from(unit: AvailableEnergyUnit) -> Self {
        match unit {
            AvailableEnergyUnit::JoulePerKilogram => JoulePerKilogram {}.into(),
            AvailableEnergyUnit::JoulePerGram => JoulePerGram {}.into(),
            AvailableEnergyUnit::BtuItPerPound => BtuItPerPound {}.into(),
            AvailableEnergyUnit::BtuPerPound => BtuPerPound {}.into(),
            AvailableEnergyUnit::CalorieItPerGram => CalorieItPerGram {}.into(),
            AvailableEnergyUnit::CaloriePerGram => CaloriePerGram {}.into(),
        }
    }
}

/// The units of `Power`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Power<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum PowerUnit {
    #[default]
    #[unit(Watt)]
    Watt,
    #[unit(ErgPerSecond)]
    ErgPerSecond,
    #[unit(FootPoundPerHour)]
    FootPoundPerHour,
    #[unit(FootPoundPerMinute)]
    FootPoundPerMinute,
    #[unit(FootPoundPerSecond)]
    FootPoundPerSecond,
    #[unit(Horsepower)]
    Horsepower,
    #[unit(HorsepowerBoiler)]
    HorsepowerBoiler,
    #[unit(HorsepowerElectric)]
    HorsepowerElectric,
    #[unit(HorsepowerMetric)]
    HorsepowerMetric,
    #[unit(HorsepowerImperial)]
    HorsepowerImperial,
    #[unit(HydraulicHorsepower)]
    HydraulicHorsepower,
}

impl From<PowerUnit> for DynamicSIUnit {
    fn from(unit: PowerUnit) -> Self {
        match unit {
            PowerUnit::Watt => Watt {}.into(),
            PowerUnit::ErgPerSecond => ErgPerSecond {}.into(),
            PowerUnit::FootPoundPerHour => FootPoundPerHour {}.into(),
            PowerUnit::FootPoundPerMinute => FootPoundPerMinute {}.into(),
            PowerUnit::FootPoundPerSecond => FootPoundPerSecond {}.into(),
            PowerUnit::Horsepower => Horsepower {}.into(),
            PowerUnit::HorsepowerBoiler => HorsepowerBoiler {}.into(),
            PowerUnit::HorsepowerElectric => HorsepowerElectric {}.into(),
            PowerUnit::HorsepowerMetric => HorsepowerMetric {}.into(),
            PowerUnit::HorsepowerImperial => HorsepowerImperial {}.into(),
            PowerUnit::HydraulicHorsepower => HydraulicHorsepower {}.into(),
        }
    }
}

/// The units of `Gain`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Gain<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum GainUnit {
    #[default]
    #[unit(Bel)]
    Bel,
    #[unit(Decibel)]
    Decibel,
    #[unit(Neper)]
    Neper,
}

impl From<GainUnit> for DynamicSIUnit {
    fn from(unit: GainUnit) -> Self {
        match unit {
            GainUnit::Bel => Bel {}.into(),
            GainUnit::Decibel => Decibel {}.into(),
            GainUnit::Neper => Neper {}.into(),
        }
    }
}

/// The units of `PowerLevel`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = PowerLevel<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum PowerLevelUnit {
    #[default]
    #[unit(DecibelWatt)]
    DecibelWatt,
    #[unit(DecibelMilliwatt)]
    DecibelMilliwatt,
}

impl From<PowerLevelUnit> for DynamicSIUnit {
    fn from(unit: PowerLevelUnit) -> Self {
        match unit {
            PowerLevelUnit::DecibelWatt => DecibelWatt {}.into(),
            PowerLevelUnit::DecibelMilliwatt => DecibelMilliwatt {}.into(),
        }
    }
}

/// The units of `PowerRate`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = PowerRate<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum PowerRateUnit {
    #[default]
    #[unit(WattPerSecond)]
    WattPerSecond,
    #[unit(ErgPerSecondSquared)]
    ErgPerSecondSquared,
    #[unit(FootPoundPerSecondSquared)]
    FootPoundPerSecondSquared,
    #[unit(HorsepowerPerSecond)]
    HorsepowerPerSecond,
    #[unit(HorsepowerPerSecondBoiler)]
    HorsepowerPerSecondBoiler,
    #[unit(HorsepowerPerSecondElectric)]
    HorsepowerPerSecondElectric,
    #[unit(HorsepowerPerSecondMetric)]
    HorsepowerPerSecondMetric,
    #[unit(HorsepowerPerSecondImperial)]
    HorsepowerPerSecondImperial,
    #[unit(HydraulicHorsepowerPerSecond)]
    HydraulicHorsepowerPerSecond,
}

impl From<PowerRateUnit> for DynamicSIUnit {
    fn from(unit: PowerRateUnit) -> Self {
        match unit {
            PowerRateUnit::WattPerSecond => WattPerSecond {}.into(),
            PowerRateUnit::ErgPerSecondSquared => ErgPerSecondSquared {}.into(),
            PowerRateUnit::FootPoundPerSecondSquared => FootPoundPerSecondSquared {}.into(),
            PowerRateUnit::HorsepowerPerSecond => HorsepowerPerSecond {}.into(),
            PowerRateUnit::HorsepowerPerSecondBoiler => HorsepowerPerSecondBoiler {}.into(),
            PowerRateUnit::HorsepowerPerSecondElectric => HorsepowerPerSecondElectric {}.into(),
            PowerRateUnit::HorsepowerPerSecondMetric => HorsepowerPerSecondMetric {}.into(),
            PowerRateUnit::HorsepowerPerSecondImperial => HorsepowerPerSecondImperial {}.into(),
            PowerRateUnit::HydraulicHorsepowerPerSecond => HydraulicHorsepowerPerSecond {}.into(),
        }
    }
}

/// The units of `Action`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Action<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ActionUnit {
    #[default]
    #[unit(JouleSecond)]
    JouleSecond,
    #[unit(AtomicUnitOfAction)]
    AtomicUnitOfAction,
    #[unit(PlanckConstant)]
    PlanckConstant,
    #[unit(ErgSecond)]
    ErgSecond,
    #[unit(ElectronvoltSecond)]
    ElectronvoltSecond,
}

impl From<ActionUnit> for DynamicSIUnit {
    fn from(unit: ActionUnit) -> Self {
        match unit {
            ActionUnit::JouleSecond => JouleSecond {}.into(),
            ActionUnit::AtomicUnitOfAction => AtomicUnitOfAction {}.into(),
            ActionUnit::PlanckConstant => PlanckConstant {}.into(),
            ActionUnit::ErgSecond => ErgSecond {}.into(),
            ActionUnit::ElectronvoltSecond => ElectronvoltSecond {}.into(),
        }
    }
}

/// The units of `Torque`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Torque<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum TorqueUnit {
    #[default]
    #[unit(NewtonMetre)]
    NewtonMetre,
    #[unit(DyneMetre)]
    DyneMetre,
    #[unit(KilogramForceMetre)]
    KilogramForceMetre,
    #[unit(OunceForceInch)]
    OunceForceInch,
    #[unit(PoundForceFoot)]
    PoundForceFoot,
    #[unit(PoundForceInch)]
    PoundForceInch,
}

impl From<TorqueUnit> for DynamicSIUnit {
    fn from(unit: TorqueUnit) -> Self {
        match unit {
            TorqueUnit::NewtonMetre => NewtonMetre {}.into(),
            TorqueUnit::DyneMetre => DyneMetre {}.into(),
            TorqueUnit::KilogramForceMetre => KilogramForceMetre {}.into(),
            TorqueUnit::OunceForceInch => OunceForceInch {}.into(),
            TorqueUnit::PoundForceFoot => PoundForceFoot {}.into(),
            TorqueUnit::PoundForceInch => PoundForceInch {}.into(),
        }
    }
}

/// The units of `MomentOfInertia`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MomentOfInertia<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum MomentOfInertiaUnit {
    #[default]
    #[unit(KilogramSquareMetre)]
    KilogramSquareMetre,
    #[unit(DaltonSquareAngstrom)]
    DaltonSquareAngstrom,
}

impl From<MomentOfInertiaUnit> for DynamicSIUnit {
    fn from(unit: MomentOfInertiaUnit) -> Self {
        match unit {
            MomentOfInertiaUnit::KilogramSquareMetre => KilogramSquareMetre {}.into(),
            MomentOfInertiaUnit::DaltonSquareAngstrom => DaltonSquareAngstrom {}.into(),
        }
    }
}

/// The units of `Pressure`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Pressure<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum PressureUnit {
    #[default]
    #[unit(Pascal)]
    Pascal,
    #[unit(StandardAtmosphere)]
    StandardAtmosphere,
    #[unit(AtmosphereTechnical)]
    AtmosphereTechnical,
    #[unit(Bar)]
    Bar,
    #[unit(CentimetreOfMercury)]
    CentimetreOfMercury,
    #[unit(CentimetreOfWater)]
    CentimetreOfWater,
    #[unit(FootOfMercury)]
    FootOfMercury,
    #[unit(FootOfWater)]
    FootOfWater,
    #[unit(InchOfMercury)]
    InchOfMercury,
    #[unit(InchOfWater)]
    InchOfWater,
    #[unit(KipPerSquareInch)]
    KipPerSquareInch,
    #[unit(MillimetreOfMercury)]
    MillimetreOfMercury,
    #[unit(MillimetreOfWater)]
    MillimetreOfWater,
    #[unit(PoundalPerSquareFoot)]
    PoundalPerSquareFoot,
    #[unit(PoundForcePerSquareFoot)]
    PoundForcePerSquareFoot,
    #[unit(PoundForcePerSquareInch)]
    PoundForcePerSquareInch,
    #[unit(Psi)]
    Psi,
    #[unit(Torr)]
    Torr,
    #[unit(DynePerSquareCentimetre)]
    DynePerSquareCentimetre,
    #[unit(NewtonPerSquareMillimetre)]
    NewtonPerSquareMillimetre,
}

impl From<PressureUnit> for DynamicSIUnit {
    fn from(unit: PressureUnit) -> Self {
        match unit {
            PressureUnit::Pascal => Pascal {}.into(),
            PressureUnit::StandardAtmosphere => StandardAtmosphere {}.into(),
            PressureUnit::AtmosphereTechnical => AtmosphereTechnical {}.into(),
            PressureUnit::Bar => Bar {}.into(),
            PressureUnit::CentimetreOfMercury => CentimetreOfMercury {}.into(),
            PressureUnit::CentimetreOfWater => CentimetreOfWater {}.into(),
            PressureUnit::FootOfMercury => FootOfMercury {}.into(),
            PressureUnit::FootOfWater => FootOfWater {}.into(),
            PressureUnit::InchOfMercury => InchOfMercury {}.into(),
            PressureUnit::InchOfWater => InchOfWater {}.into(),
            PressureUnit::KipPerSquareInch => KipPerSquareInch {}.into(),
            PressureUnit::MillimetreOfMercury => MillimetreOfMercury {}.into(),
            PressureUnit::MillimetreOfWater => MillimetreOfWater {}.into(),
            PressureUnit::PoundalPerSquareFoot => PoundalPerSquareFoot {}.into(),
            PressureUnit::PoundForcePerSquareFoot => PoundForcePerSquareFoot {}.into(),
            PressureUnit::PoundForcePerSquareInch => PoundForcePerSquareInch {}.into(),
            PressureUnit::Psi => Psi {}.into(),
            PressureUnit::Torr => Torr {}.into(),
            PressureUnit::DynePerSquareCentimetre => DynePerSquareCentimetre {}.into(),
            PressureUnit::NewtonPerSquareMillimetre => NewtonPerSquareMillimetre {}.into(),
        }
    }
}

/// The units of `Frequency`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Frequency<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum FrequencyUnit {
    #[default]
    #[unit(Hertz)]
    Hertz,
    #[unit(CyclePerDay)]
    CyclePerDay,
    #[unit(CyclePerHour)]
    CyclePerHour,
    #[unit(CyclePerMinute)]
    CyclePerMinute,
    #[unit(CyclePerShake)]
    CyclePerShake,
    #[unit(CyclePerYear)]
    CyclePerYear,
}

impl From<FrequencyUnit> for DynamicSIUnit {
    fn from(unit: FrequencyUnit) -> Self {
        match unit {
            FrequencyUnit::Hertz => Hertz {}.into(),
            FrequencyUnit::CyclePerDay => CyclePerDay {}.into(),
            FrequencyUnit::CyclePerHour => CyclePerHour {}.into(),
            FrequencyUnit::CyclePerMinute => CyclePerMinute {}.into(),
            FrequencyUnit::CyclePerShake => CyclePerShake {}.into(),
            FrequencyUnit::CyclePerYear => CyclePerYear {}.into(),
        }
    }
}

/// The units of `FrequencyDrift`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = FrequencyDrift<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum FrequencyDriftUnit {
    #[default]
    #[unit(HertzPerSecond)]
    HertzPerSecond,
}

impl From<FrequencyDriftUnit> for DynamicSIUnit {
    fn from(unit: FrequencyDriftUnit) -> Self {
        match unit {
            FrequencyDriftUnit::HertzPerSecond => HertzPerSecond {}.into(),
        }
    }
}

/// The units of `ReciprocalLength`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ReciprocalLength<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum ReciprocalLengthUnit {
    #[default]
    #[unit(ReciprocalMetre)]
    ReciprocalMetre,
    #[unit(ReciprocalAngstrom)]
    ReciprocalAngstrom,
    #[unit(Diopter)]
    Diopter,
    #[unit(RydbergConstant)]
    RydbergConstant,
}

impl From<ReciprocalLengthUnit> for DynamicSIUnit {
    fn from(unit: ReciprocalLengthUnit) -> Self {
        match unit {
            ReciprocalLengthUnit::ReciprocalMetre => ReciprocalMetre {}.into(),
            ReciprocalLengthUnit::ReciprocalAngstrom => ReciprocalAngstrom {}.into(),
            ReciprocalLengthUnit::Diopter => Diopter {}.into(),
            ReciprocalLengthUnit::RydbergConstant => RydbergConstant {}.into(),
        }
    }
}

/// The units of `Curvature`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Curvature<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum CurvatureUnit {
    #[default]
    #[unit(RadianPerMetre)]
    RadianPerMetre,
    #[unit(DegreePerMetre)]
    DegreePerMetre,
}

impl From<CurvatureUnit> for DynamicSIUnit {
    fn from(unit: CurvatureUnit) -> Self {
        match unit {
            CurvatureUnit::RadianPerMetre => RadianPerMetre {}.into(),
            CurvatureUnit::DegreePerMetre => DegreePerMetre {}.into(),
        }
    }
}

/// The units of `SpecificArea`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificArea<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum SpecificAreaUnit {
    #[default]
    #[unit(SquareMetrePerKilogram)]
    SquareMetrePerKilogram,
    #[unit(SquareMetrePerGram)]
    SquareMetrePerGram,
}

impl From<SpecificAreaUnit> for DynamicSIUnit {
    fn from(unit: SpecificAreaUnit) -> Self {
        match unit {
            SpecificAreaUnit::SquareMetrePerKilogram => SquareMetrePerKilogram {}.into(),
            SpecificAreaUnit::SquareMetrePerGram => SquareMetrePerGram {}.into(),
        }
    }
}

/// The units of `SpecificVolume`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificVolume<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum SpecificVolumeUnit {
    #[default]
    #[unit(CubicMetrePerKilogram)]
    CubicMetrePerKilogram,
    #[unit(CubicMetrePerGram)]
    CubicMetrePerGram,
    #[unit(CubicMetrePerCarat)]
    CubicMetrePerCarat,
    #[unit(CubicMetrePerGrain)]
    CubicMetrePerGrain,
    #[unit(CubicMetrePerHundredweightLong)]
    CubicMetrePerHundredweightLong,
    #[unit(CubicMetrePerHundredweightShort)]
    CubicMetrePerHundredweightShort,
    #[unit(CubicMetrePerOunce)]
    CubicMetrePerOunce,
    #[unit(CubicMetrePerOunceTroy)]
    CubicMetrePerOunceTroy,
    #[unit(CubicMetrePerPennyweight)]
    CubicMetrePerPennyweight,
    #[unit(CubicMetrePerPound)]
    CubicMetrePerPound,
    #[unit(CubicMetrePerPoundTroy)]
    CubicMetrePerPoundTroy,
    #[unit(CubicMetrePerSlug)]
    CubicMetrePerSlug,
    #[unit(CubicMetrePerTonAssay)]
    CubicMetrePerTonAssay,
    #[unit(CubicMetrePerTonLong)]
    CubicMetrePerTonLong,
    #[unit(CubicMetrePerTonShort)]
    CubicMetrePerTonShort,
    #[unit(CubicMetrePerTon)]
    CubicMetrePerTon,
    #[unit(GallonPerGrain)]
    GallonPerGrain,
    #[unit(CubicInchPerOunce)]
    CubicInchPerOunce,
    #[unit(GallonImperialPerOunce)]
    GallonImperialPerOunce,
    #[unit(GallonPerOunce)]
    GallonPerOunce,
    #[unit(CubicFootPerPound)]
    CubicFootPerPound,
    #[unit(CubicInchPerPound)]
    CubicInchPerPound,
    #[unit(CubicYardPerPound)]
    CubicYardPerPound,
    #[unit(GallonImperialPerPound)]
    GallonImperialPerPound,
    #[unit(GallonPerPound)]
    GallonPerPound,
    #[unit(CubicFootPerSlug)]
    CubicFootPerSlug,
    #[unit(CubicYardPerTonLong)]
    CubicYardPerTonLong,
    #[unit(CubicYardPerTonShort)]
    CubicYardPerTonShort,
    #[unit(CubicFootPerTonLong)]
    CubicFootPerTonLong,
    #[unit(CubicFootPerTonShort)]
    CubicFootPerTonShort,
}

impl From<SpecificVolumeUnit> for DynamicSIUnit {
    fn from(unit: SpecificVolumeUnit) -> Self {
        match unit {
            SpecificVolumeUnit::CubicMetrePerKilogram => CubicMetrePerKilogram {}.into(),
            SpecificVolumeUnit::CubicMetrePerGram => CubicMetrePerGram {}.into(),
            SpecificVolumeUnit::CubicMetrePerCarat => CubicMetrePerCarat {}.into(),
            SpecificVolumeUnit::CubicMetrePerGrain => CubicMetrePerGrain {}.into(),
            SpecificVolumeUnit::CubicMetrePerHundredweightLong => {
                CubicMetrePerHundredweightLong {}.into()
            }
            SpecificVolumeUnit::CubicMetrePerHundredweightShort => {
                CubicMetrePerHundredweightShort {}.into()
            }
            SpecificVolumeUnit::CubicMetrePerOunce => CubicMetrePerOunce {}.into(),
            SpecificVolumeUnit::CubicMetrePerOunceTroy => CubicMetrePerOunceTroy {}.into(),
            SpecificVolumeUnit::CubicMetrePerPennyweight => CubicMetrePerPennyweight {}.into(),
            SpecificVolumeUnit::CubicMetrePerPound => CubicMetrePerPound {}.into(),
            SpecificVolumeUnit::CubicMetrePerPoundTroy => CubicMetrePerPoundTroy {}.into(),
            SpecificVolumeUnit::CubicMetrePerSlug => CubicMetrePerSlug {}.into(),
            SpecificVolumeUnit::CubicMetrePerTonAssay => CubicMetrePerTonAssay {}.into(),
            SpecificVolumeUnit::CubicMetrePerTonLong => CubicMetrePerTonLong {}.into(),
            SpecificVolumeUnit::CubicMetrePerTonShort => CubicMetrePerTonShort {}.into(),
            SpecificVolumeUnit::CubicMetrePerTon => CubicMetrePerTon {}.into(),
            SpecificVolumeUnit::GallonPerGrain => GallonPerGrain {}.into(),
            SpecificVolumeUnit::CubicInchPerOunce => CubicInchPerOunce {}.into(),
            SpecificVolumeUnit::GallonImperialPerOunce => GallonImperialPerOunce {}.into(),
            SpecificVolumeUnit::GallonPerOunce => GallonPerOunce {}.into(),
            SpecificVolumeUnit::CubicFootPerPound => CubicFootPerPound {}.into(),
            SpecificVolumeUnit::CubicInchPerPound => CubicInchPerPound {}.into(),
            SpecificVolumeUnit::CubicYardPerPound => CubicYardPerPound {}.into(),
            SpecificVolumeUnit::GallonImperialPerPound => GallonImperialPerPound {}.into(),
            SpecificVolumeUnit::GallonPerPound => GallonPerPound {}.into(),
            SpecificVolumeUnit::CubicFootPerSlug => CubicFootPerSlug {}.into(),
            SpecificVolumeUnit::CubicYardPerTonLong => CubicYardPerTonLong {}.into(),
            SpecificVolumeUnit::CubicYardPerTonShort => CubicYardPerTonShort {}.into(),
            SpecificVolumeUnit::CubicFootPerTonLong => CubicFootPerTonLong {}.into(),
            SpecificVolumeUnit::CubicFootPerTonShort => CubicFootPerTonShort {}.into(),
        }
    }
}

/// The units of `SpecificPower`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificPower<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum SpecificPowerUnit {
    #[default]
    #[unit(WattPerKilogram)]
    WattPerKilogram,
    #[unit(SquareMetrePerCubicSecond)]
    SquareMetrePerCubicSecond,
    #[unit(ErgPerSecondPerKilogram)]
    ErgPerSecondPerKilogram,
    #[unit(FootPoundPerHourPerPound)]
    FootPoundPerHourPerPound,
    #[unit(FootPoundPerMinutePerPound)]
    FootPoundPerMinutePerPound,
    #[unit(FootPoundPerSecondPerPound)]
    FootPoundPerSecondPerPound,
    #[unit(HorsepowerPerPound)]
    HorsepowerPerPound,
    #[unit(HorsepowerBoilerPerPound)]
    HorsepowerBoilerPerPound,
    #[unit(HorsepowerElectricPerPound)]
    HorsepowerElectricPerPound,
    #[unit(HorsepowerMetricPerPound)]
    HorsepowerMetricPerPound,
    #[unit(HorsepowerImperialPerPound)]
    HorsepowerImperialPerPound,
    #[unit(HydraulicHorsepowerPerPound)]
    HydraulicHorsepowerPerPound,
}

impl From<SpecificPowerUnit> for DynamicSIUnit {
    fn from(unit: SpecificPowerUnit) -> Self {
        match unit {
            SpecificPowerUnit::WattPerKilogram => WattPerKilogram {}.into(),
            SpecificPowerUnit::SquareMetrePerCubicSecond => SquareMetrePerCubicSecond {}.into(),
            SpecificPowerUnit::ErgPerSecondPerKilogram => ErgPerSecondPerKilogram {}.into(),
            SpecificPowerUnit::FootPoundPerHourPerPound => FootPoundPerHourPerPound {}.into(),
            SpecificPowerUnit::FootPoundPerMinutePerPound => FootPoundPerMinutePerPound {}.into(),
            SpecificPowerUnit::FootPoundPerSecondPerPound => FootPoundPerSecondPerPound {}.into(),
            SpecificPowerUnit::HorsepowerPerPound => HorsepowerPerPound {}.into(),
            SpecificPowerUnit::HorsepowerBoilerPerPound => HorsepowerBoilerPerPound {}.into(),
            SpecificPowerUnit::HorsepowerElectricPerPound => HorsepowerElectricPerPound {}.into(),
            SpecificPowerUnit::HorsepowerMetricPerPound => HorsepowerMetricPerPound {}.into(),
            SpecificPowerUnit::HorsepowerImperialPerPound => HorsepowerImperialPerPound {}.into(),
            SpecificPowerUnit::HydraulicHorsepowerPerPound => HydraulicHorsepowerPerPound {}.into(),
        }
    }
}

/// The units of `SpecificGravitationalParameter`, in any number type they count in.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificGravitationalParameter<N>, number = N, for<N: const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct>)]
pub enum SpecificGravitationalParameterUnit {
    #[default]
    #[unit(CubicMetrePerKilogramSecondSquared)]
    CubicMetrePerKilogramSecondSquared,
    #[unit(GravitationalConstant)]
    GravitationalConstant,
}

impl From<SpecificGravitationalParameterUnit> for DynamicSIUnit {
    fn from(unit: SpecificGravitationalParameterUnit) -> Self {
        match unit {
            SpecificGravitationalParameterUnit::CubicMetrePerKilogramSecondSquared => {
                CubicMetrePerKilogramSecondSquared {}.into()
            }
            SpecificGravitationalParameterUnit::GravitationalConstant => {
                GravitationalConstant {}.into()
            }
        }
    }
}
