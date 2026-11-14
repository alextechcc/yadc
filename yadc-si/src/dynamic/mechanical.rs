//! The units of `mechanical`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

#[allow(unused_imports)]
use crate::mechanical::*;
#[allow(unused_imports)]
use crate::*;

/// The units of `Length`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum LengthUnit {
    #[default]
    #[value(METRE.unit())]
    Metre,
    #[value(KILOMETRE.unit())]
    Kilometre,
    #[value(ANGSTROM.unit())]
    Angstrom,
    #[value(BOHR_RADIUS.unit())]
    BohrRadius,
    #[value(ATOMIC_UNIT_OF_LENGTH.unit())]
    AtomicUnitOfLength,
    #[value(ASTRONOMICAL_UNIT.unit())]
    AstronomicalUnit,
    #[value(CHAIN.unit())]
    Chain,
    #[value(DATA_MILE.unit())]
    DataMile,
    #[value(FATHOM.unit())]
    Fathom,
    #[value(FERMI.unit())]
    Fermi,
    #[value(FOOT.unit())]
    Foot,
    #[value(FOOT_SURVEY.unit())]
    FootSurvey,
    #[value(INCH.unit())]
    Inch,
    #[value(LIGHT_YEAR.unit())]
    LightYear,
    #[value(MICRON.unit())]
    Micron,
    #[value(MIL.unit())]
    Mil,
    #[value(MILE.unit())]
    Mile,
    #[value(MILE_SURVEY.unit())]
    MileSurvey,
    #[value(NAUTICAL_MILE.unit())]
    NauticalMile,
    #[value(PARSEC.unit())]
    Parsec,
    #[value(PICA_COMPUTER.unit())]
    PicaComputer,
    #[value(PICA_PRINTERS.unit())]
    PicaPrinters,
    #[value(POINT_COMPUTER.unit())]
    PointComputer,
    #[value(POINT_PRINTERS.unit())]
    PointPrinters,
    #[value(ROD.unit())]
    Rod,
    #[value(YARD.unit())]
    Yard,
    #[value(LEUGA.unit())]
    Leuga,
    #[value(MILLE_PASSUS.unit())]
    MillePassus,
    #[value(STADIUM.unit())]
    Stadium,
    #[value(ACTUS.unit())]
    Actus,
    #[value(DECEMPEDA.unit())]
    Decempeda,
    #[value(PASSUS.unit())]
    Passus,
    #[value(GRADUS.unit())]
    Gradus,
    #[value(CUBITUM.unit())]
    Cubitum,
    #[value(PALMIPES.unit())]
    Palmipes,
    #[value(PES.unit())]
    Pes,
    #[value(PALMUS_MAIOR.unit())]
    PalmusMaior,
    #[value(PALMUS.unit())]
    Palmus,
    #[value(LENGTH_UNCIA.unit())]
    Uncia,
    #[value(DIGITUS.unit())]
    Digitus,
}

/// The units of `Mass`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum MassUnit {
    #[default]
    #[value(KILOGRAM.unit())]
    Kilogram,
    #[value(GRAM.unit())]
    Gram,
    #[value(CARAT.unit())]
    Carat,
    #[value(DALTON.unit())]
    Dalton,
    #[value(ELECTRON_MASS.unit())]
    ElectronMass,
    #[value(PROTON_MASS.unit())]
    ProtonMass,
    #[value(GRAIN.unit())]
    Grain,
    #[value(HUNDREDWEIGHT_LONG.unit())]
    HundredweightLong,
    #[value(HUNDREDWEIGHT_SHORT.unit())]
    HundredweightShort,
    #[value(OUNCE.unit())]
    Ounce,
    #[value(OUNCE_TROY.unit())]
    OunceTroy,
    #[value(PENNYWEIGHT.unit())]
    Pennyweight,
    #[value(POUND.unit())]
    Pound,
    #[value(POUND_TROY.unit())]
    PoundTroy,
    #[value(SLUG.unit())]
    Slug,
    #[value(TON_ASSAY.unit())]
    TonAssay,
    #[value(TON_LONG.unit())]
    TonLong,
    #[value(TON_SHORT.unit())]
    TonShort,
    #[value(TON.unit())]
    Ton,
    #[value(LIBRA.unit())]
    Libra,
    #[value(MASS_DEUNX.unit())]
    Deunx,
    #[value(MASS_DEXTANS.unit())]
    Dextans,
    #[value(MASS_DODRANS.unit())]
    Dodrans,
    #[value(MASS_BES.unit())]
    Bes,
    #[value(MASS_SEPTUNX.unit())]
    Septunx,
    #[value(MASS_SEMIS.unit())]
    Semis,
    #[value(MASS_QUINCUNX.unit())]
    Quincunx,
    #[value(MASS_TRIENS.unit())]
    Triens,
    #[value(QUADRANS_TERUNCIUS.unit())]
    QuadransTeruncius,
    #[value(MASS_SEXTANS.unit())]
    Sextans,
    #[value(SESCUNCIA.unit())]
    Sescuncia,
    #[value(MASS_UNCIA.unit())]
    Uncia,
    #[value(SEMUNCIA.unit())]
    Semuncia,
    #[value(DUELLA.unit())]
    Duella,
    #[value(MASS_SICILICUS.unit())]
    Sicilicus,
    #[value(MASS_SEXTULA.unit())]
    Sextula,
    #[value(SEMISEXTULA.unit())]
    Semisextula,
    #[value(MASS_SCRUPULUM.unit())]
    Scrupulum,
    #[value(OBOLUS.unit())]
    Obolus,
    #[value(SILIQUA.unit())]
    Siliqua,
}

/// The units of `Time`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum TimeUnit {
    #[default]
    #[value(SECOND.unit())]
    Second,
    #[value(SECOND_SIDEREAL.unit())]
    SecondSidereal,
    #[value(DAY.unit())]
    Day,
    #[value(DAY_SIDEREAL.unit())]
    DaySidereal,
    #[value(HOUR.unit())]
    Hour,
    #[value(HOUR_SIDEREAL.unit())]
    HourSidereal,
    #[value(MINUTE.unit())]
    Minute,
    #[value(SHAKE.unit())]
    Shake,
    #[value(YEAR.unit())]
    Year,
    #[value(YEAR_SIDEREAL.unit())]
    YearSidereal,
    #[value(YEAR_TROPICAL.unit())]
    YearTropical,
}

/// The units of `Instant`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum InstantUnit {
    #[default]
    #[value(UNIX_SECOND.unit())]
    UnixSecond,
    #[value(UNIX_MILLISECOND.unit())]
    UnixMillisecond,
    #[value(GPS_SECOND.unit())]
    GpsSecond,
    #[value(JULIAN_DATE.unit())]
    JulianDate,
    #[value(MODIFIED_JULIAN_DATE.unit())]
    ModifiedJulianDate,
}

/// The units of `Area`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum AreaUnit {
    #[default]
    #[value(SQUARE_METRE.unit())]
    SquareMetre,
    #[value(ACRE.unit())]
    Acre,
    #[value(ARE.unit())]
    Are,
    #[value(BARN.unit())]
    Barn,
    #[value(CIRCULAR_MIL.unit())]
    CircularMil,
    #[value(HECTARE.unit())]
    Hectare,
    #[value(SQUARE_FOOT.unit())]
    SquareFoot,
    #[value(SQUARE_INCH.unit())]
    SquareInch,
    #[value(SQUARE_MILE.unit())]
    SquareMile,
    #[value(SQUARE_YARD.unit())]
    SquareYard,
    #[value(SALTUS.unit())]
    Saltus,
    #[value(CENTURIA.unit())]
    Centuria,
    #[value(HEREDIUM.unit())]
    Heredium,
    #[value(JUGERUM.unit())]
    Jugerum,
    #[value(AREA_DEUNX.unit())]
    Deunx,
    #[value(AREA_DEXTANS.unit())]
    Dextans,
    #[value(AREA_DODRANS.unit())]
    Dodrans,
    #[value(AREA_BES.unit())]
    Bes,
    #[value(AREA_SEPTUNX.unit())]
    Septunx,
    #[value(AREA_SEMIS.unit())]
    Semis,
    #[value(ACTUS_QUADRATUS.unit())]
    ActusQuadratus,
    #[value(AREA_QUINCUNX.unit())]
    Quincunx,
    #[value(AREA_TRIENS.unit())]
    Triens,
    #[value(QUADRANS.unit())]
    Quadrans,
    #[value(AREA_SEXTANS.unit())]
    Sextans,
    #[value(CLIMA.unit())]
    Clima,
    #[value(AREA_UNCIA.unit())]
    Uncia,
    #[value(SEMIUNCIA.unit())]
    Semiuncia,
    #[value(AREA_SICILICUS.unit())]
    Sicilicus,
    #[value(ACTUS_SIMPLEX.unit())]
    ActusSimplex,
    #[value(AREA_SEXTULA.unit())]
    Sextula,
    #[value(DUO_SCRUPULUM.unit())]
    DuoScrupulum,
    #[value(AREA_SCRUPULUM.unit())]
    Scrupulum,
    #[value(DIMIDIUM_SCRUPULUM.unit())]
    DimidiumScrupulum,
    #[value(PES_QUADRATUS.unit())]
    PesQuadratus,
}

/// The units of `Volume`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum VolumeUnit {
    #[default]
    #[value(CUBIC_METRE.unit())]
    CubicMetre,
    #[value(ACRE_FOOT.unit())]
    AcreFoot,
    #[value(BARREL.unit())]
    Barrel,
    #[value(BUSHEL.unit())]
    Bushel,
    #[value(CORD.unit())]
    Cord,
    #[value(CUBIC_FOOT.unit())]
    CubicFoot,
    #[value(CUBIC_INCH.unit())]
    CubicInch,
    #[value(CUBIC_MILE.unit())]
    CubicMile,
    #[value(CUBIC_YARD.unit())]
    CubicYard,
    #[value(CUP.unit())]
    Cup,
    #[value(FLUID_OUNCE.unit())]
    FluidOunce,
    #[value(FLUID_OUNCE_IMPERIAL.unit())]
    FluidOunceImperial,
    #[value(GALLON_IMPERIAL.unit())]
    GallonImperial,
    #[value(GALLON.unit())]
    Gallon,
    #[value(GILL_IMPERIAL.unit())]
    GillImperial,
    #[value(GILL.unit())]
    Gill,
    #[value(LITRE.unit())]
    Litre,
    #[value(PECK.unit())]
    Peck,
    #[value(PINT_DRY.unit())]
    PintDry,
    #[value(PINT_LIQUID.unit())]
    PintLiquid,
    #[value(QUART_DRY.unit())]
    QuartDry,
    #[value(QUART_LIQUID.unit())]
    QuartLiquid,
    #[value(STERE.unit())]
    Stere,
    #[value(TABLESPOON.unit())]
    Tablespoon,
    #[value(TEASPOON.unit())]
    Teaspoon,
    #[value(REGISTER_TON.unit())]
    RegisterTon,
    #[value(CULEUS.unit())]
    Culeus,
    #[value(AMPHORA_QUADRANTAL.unit())]
    AmphoraQuadrantal,
    #[value(URNA.unit())]
    Urna,
    #[value(MODIUS_CASTRENSIS.unit())]
    ModiusCastrensis,
    #[value(MODIUS.unit())]
    Modius,
    #[value(SEMIMODIUS.unit())]
    Semimodius,
    #[value(CONGIUS.unit())]
    Congius,
    #[value(SEXTARIUS.unit())]
    Sextarius,
    #[value(HEMINA.unit())]
    Hemina,
    #[value(QUARTARIUS.unit())]
    Quartarius,
    #[value(ACETABULUM.unit())]
    Acetabulum,
    #[value(CYATHUS.unit())]
    Cyathus,
    #[value(LIGULA.unit())]
    Ligula,
}

/// The units of `Velocity`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum VelocityUnit {
    #[default]
    #[value(METRE_PER_SECOND.unit())]
    MetrePerSecond,
    #[value(KNOT.unit())]
    Knot,
    #[value(ATOMIC_UNIT_OF_VELOCITY.unit())]
    AtomicUnitOfVelocity,
    #[value(NATURAL_UNIT_OF_VELOCITY.unit())]
    NaturalUnitOfVelocity,
    #[value(SPEED_OF_LIGHT.unit())]
    SpeedOfLight,
}

/// The units of `Acceleration`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum AccelerationUnit {
    #[default]
    #[value(METRE_PER_SECOND_SQUARED.unit())]
    MetrePerSecondSquared,
    #[value(GALILEO.unit())]
    Galileo,
    #[value(STANDARD_GRAVITY.unit())]
    StandardGravity,
}

/// The units of `Absement`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum AbsementUnit {
    #[default]
    #[value(METRE_SECOND.unit())]
    MetreSecond,
    #[value(FOOT_SECOND.unit())]
    FootSecond,
    #[value(INCH_SECOND.unit())]
    InchSecond,
    #[value(KILOMETRE_HOUR.unit())]
    KilometreHour,
}

/// The units of `Force`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum ForceUnit {
    #[default]
    #[value(NEWTON.unit())]
    Newton,
    #[value(DYNE.unit())]
    Dyne,
    #[value(KILOGRAM_FORCE.unit())]
    KilogramForce,
    #[value(KIP.unit())]
    Kip,
    #[value(OUNCE_FORCE.unit())]
    OunceForce,
    #[value(POUNDAL.unit())]
    Poundal,
    #[value(POUND_FORCE.unit())]
    PoundForce,
    #[value(TON_FORCE.unit())]
    TonForce,
}

/// The units of `Energy`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum EnergyUnit {
    #[default]
    #[value(JOULE.unit())]
    Joule,
    #[value(WATT_HOUR.unit())]
    WattHour,
    #[value(ELECTRONVOLT.unit())]
    Electronvolt,
    #[value(HARTREE.unit())]
    Hartree,
    #[value(BTU_IT.unit())]
    BtuIt,
    #[value(BTU.unit())]
    Btu,
    #[value(CALORIE_IT.unit())]
    CalorieIt,
    #[value(CALORIE.unit())]
    Calorie,
    #[value(CALORIE_IT_NUTRITION.unit())]
    CalorieItNutrition,
    #[value(CALORIE_NUTRITION.unit())]
    CalorieNutrition,
    #[value(ERG.unit())]
    Erg,
    #[value(FOOT_POUNDAL.unit())]
    FootPoundal,
    #[value(FOOT_POUND.unit())]
    FootPound,
    #[value(QUAD.unit())]
    Quad,
    #[value(THERM_EC.unit())]
    ThermEc,
    #[value(THERM_US.unit())]
    ThermUs,
    #[value(TON_TNT.unit())]
    TonTnt,
    #[value(WATT_SECOND.unit())]
    WattSecond,
}

/// The units of `Power`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum PowerUnit {
    #[default]
    #[value(WATT.unit())]
    Watt,
    #[value(HORSEPOWER.unit())]
    Horsepower,
    #[value(HORSEPOWER_BOILER.unit())]
    HorsepowerBoiler,
    #[value(HORSEPOWER_ELECTRIC.unit())]
    HorsepowerElectric,
    #[value(HORSEPOWER_METRIC.unit())]
    HorsepowerMetric,
    #[value(HORSEPOWER_IMPERIAL.unit())]
    HorsepowerImperial,
    #[value(HYDRAULIC_HORSEPOWER.unit())]
    HydraulicHorsepower,
    #[value(DECIBEL_WATT.unit())]
    DecibelWatt,
    #[value(DECIBEL_MILLIWATT.unit())]
    DecibelMilliwatt,
}

/// The units of `Action`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum ActionUnit {
    #[default]
    #[value(JOULE_SECOND.unit())]
    JouleSecond,
    #[value(ATOMIC_UNIT_OF_ACTION.unit())]
    AtomicUnitOfAction,
    #[value(PLANCK_CONSTANT.unit())]
    PlanckConstant,
    #[value(ERG_SECOND.unit())]
    ErgSecond,
    #[value(ELECTRONVOLT_SECOND.unit())]
    ElectronvoltSecond,
}

/// The units of `Torque`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum TorqueUnit {
    #[default]
    #[value(NEWTON_METRE.unit())]
    NewtonMetre,
    #[value(DYNE_METRE.unit())]
    DyneMetre,
    #[value(KILOGRAM_FORCE_METRE.unit())]
    KilogramForceMetre,
    #[value(OUNCE_FORCE_INCH.unit())]
    OunceForceInch,
    #[value(POUND_FORCE_FOOT.unit())]
    PoundForceFoot,
    #[value(POUND_FORCE_INCH.unit())]
    PoundForceInch,
}

/// The units of `MomentOfInertia`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum MomentOfInertiaUnit {
    #[default]
    #[value(KILOGRAM_SQUARE_METRE.unit())]
    KilogramSquareMetre,
    #[value(DALTON_SQUARE_ANGSTROM.unit())]
    DaltonSquareAngstrom,
}

/// The units of `Pressure`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum PressureUnit {
    #[default]
    #[value(PASCAL.unit())]
    Pascal,
    #[value(STANDARD_ATMOSPHERE.unit())]
    StandardAtmosphere,
    #[value(ATMOSPHERE_TECHNICAL.unit())]
    AtmosphereTechnical,
    #[value(BAR.unit())]
    Bar,
    #[value(CENTIMETRE_OF_MERCURY.unit())]
    CentimetreOfMercury,
    #[value(CENTIMETRE_OF_WATER.unit())]
    CentimetreOfWater,
    #[value(FOOT_OF_MERCURY.unit())]
    FootOfMercury,
    #[value(FOOT_OF_WATER.unit())]
    FootOfWater,
    #[value(INCH_OF_MERCURY.unit())]
    InchOfMercury,
    #[value(INCH_OF_WATER.unit())]
    InchOfWater,
    #[value(MILLIMETRE_OF_MERCURY.unit())]
    MillimetreOfMercury,
    #[value(MILLIMETRE_OF_WATER.unit())]
    MillimetreOfWater,
    #[value(PSI.unit())]
    Psi,
    #[value(TORR.unit())]
    Torr,
}

/// The units of `Frequency`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum FrequencyUnit {
    #[default]
    #[value(HERTZ.unit())]
    Hertz,
}

/// The units of `ReciprocalLength`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum ReciprocalLengthUnit {
    #[default]
    #[value(RECIPROCAL_METRE.unit())]
    ReciprocalMetre,
    #[value(RECIPROCAL_ANGSTROM.unit())]
    ReciprocalAngstrom,
    #[value(DIOPTER.unit())]
    Diopter,
    #[value(RYDBERG_CONSTANT.unit())]
    RydbergConstant,
}

/// The units of `SpecificGravitationalParameter`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum SpecificGravitationalParameterUnit {
    #[default]
    #[value(METRE_CUBED_PER_KILOGRAM_SECOND_SQUARED.unit())]
    MetreCubedPerKilogramSecondSquared,
    #[value(GRAVITATIONAL_CONSTANT.unit())]
    GravitationalConstant,
}
