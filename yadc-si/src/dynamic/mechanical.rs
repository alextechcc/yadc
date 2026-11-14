//! The units of `mechanical`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::dynamic::DynamicSIUnit;
use crate::mechanical::units::*;
use crate::mechanical::*;
use crate::{Exact, SIQuantity, Unit};
use core::ops::{Add, Div, Mul, Sub};

/// The units of `Length`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<LENGTH, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<LENGTH, N>;
        fn value(&self, quantity: SIQuantity<LENGTH, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum LengthUnit {
    #[default]
    #[value(METRE)]
    Metre,
    #[value(ANGSTROM)]
    Angstrom,
    #[value(BOHR_RADIUS)]
    BohrRadius,
    #[value(ATOMIC_UNIT_OF_LENGTH)]
    AtomicUnitOfLength,
    #[value(ASTRONOMICAL_UNIT)]
    AstronomicalUnit,
    #[value(CHAIN)]
    Chain,
    #[value(DATA_MILE)]
    DataMile,
    #[value(FATHOM)]
    Fathom,
    #[value(FERMI)]
    Fermi,
    #[value(FOOT)]
    Foot,
    #[value(FOOT_SURVEY)]
    FootSurvey,
    #[value(INCH)]
    Inch,
    #[value(LIGHT_YEAR)]
    LightYear,
    #[value(MICRON)]
    Micron,
    #[value(MIL)]
    Mil,
    #[value(MILE)]
    Mile,
    #[value(MILE_SURVEY)]
    MileSurvey,
    #[value(NAUTICAL_MILE)]
    NauticalMile,
    #[value(PARSEC)]
    Parsec,
    #[value(PICA_COMPUTER)]
    PicaComputer,
    #[value(PICA_PRINTERS)]
    PicaPrinters,
    #[value(POINT_COMPUTER)]
    PointComputer,
    #[value(POINT_PRINTERS)]
    PointPrinters,
    #[value(ROD)]
    Rod,
    #[value(YARD)]
    Yard,
    #[value(LEUGA)]
    Leuga,
    #[value(MILLE_PASSUS)]
    MillePassus,
    #[value(STADIUM)]
    Stadium,
    #[value(ACTUS)]
    Actus,
    #[value(DECEMPEDA)]
    Decempeda,
    #[value(PASSUS)]
    Passus,
    #[value(GRADUS)]
    Gradus,
    #[value(CUBITUM)]
    Cubitum,
    #[value(PALMIPES)]
    Palmipes,
    #[value(PES)]
    Pes,
    #[value(PALMUS_MAIOR)]
    PalmusMaior,
    #[value(PALMUS)]
    Palmus,
    #[value(LENGTH_UNCIA)]
    Uncia,
    #[value(DIGITUS)]
    Digitus,
}

/// The units of `Mass`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<MASS, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<MASS, N>;
        fn value(&self, quantity: SIQuantity<MASS, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MassUnit {
    #[default]
    #[value(KILOGRAM)]
    Kilogram,
    #[value(GRAM)]
    Gram,
    #[value(CARAT)]
    Carat,
    #[value(DALTON)]
    Dalton,
    #[value(ELECTRON_MASS)]
    ElectronMass,
    #[value(PROTON_MASS)]
    ProtonMass,
    #[value(GRAIN)]
    Grain,
    #[value(HUNDREDWEIGHT_LONG)]
    HundredweightLong,
    #[value(HUNDREDWEIGHT_SHORT)]
    HundredweightShort,
    #[value(OUNCE)]
    Ounce,
    #[value(OUNCE_TROY)]
    OunceTroy,
    #[value(PENNYWEIGHT)]
    Pennyweight,
    #[value(POUND)]
    Pound,
    #[value(POUND_TROY)]
    PoundTroy,
    #[value(SLUG)]
    Slug,
    #[value(TON_ASSAY)]
    TonAssay,
    #[value(TON_LONG)]
    TonLong,
    #[value(TON_SHORT)]
    TonShort,
    #[value(TON)]
    Ton,
    #[value(LIBRA)]
    Libra,
    #[value(MASS_DEUNX)]
    Deunx,
    #[value(MASS_DEXTANS)]
    Dextans,
    #[value(MASS_DODRANS)]
    Dodrans,
    #[value(MASS_BES)]
    Bes,
    #[value(MASS_SEPTUNX)]
    Septunx,
    #[value(MASS_SEMIS)]
    Semis,
    #[value(MASS_QUINCUNX)]
    Quincunx,
    #[value(MASS_TRIENS)]
    Triens,
    #[value(QUADRANS_TERUNCIUS)]
    QuadransTeruncius,
    #[value(MASS_SEXTANS)]
    Sextans,
    #[value(SESCUNCIA)]
    Sescuncia,
    #[value(MASS_UNCIA)]
    Uncia,
    #[value(SEMUNCIA)]
    Semuncia,
    #[value(DUELLA)]
    Duella,
    #[value(MASS_SICILICUS)]
    Sicilicus,
    #[value(MASS_SEXTULA)]
    Sextula,
    #[value(SEMISEXTULA)]
    Semisextula,
    #[value(MASS_SCRUPULUM)]
    Scrupulum,
    #[value(OBOLUS)]
    Obolus,
    #[value(SILIQUA)]
    Siliqua,
}

/// The units of `Time`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<TIME, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<TIME, N>;
        fn value(&self, quantity: SIQuantity<TIME, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum TimeUnit {
    #[default]
    #[value(SECOND)]
    Second,
    #[value(SECOND_SIDEREAL)]
    SecondSidereal,
    #[value(DAY)]
    Day,
    #[value(DAY_SIDEREAL)]
    DaySidereal,
    #[value(HOUR)]
    Hour,
    #[value(HOUR_SIDEREAL)]
    HourSidereal,
    #[value(MINUTE)]
    Minute,
    #[value(SHAKE)]
    Shake,
    #[value(YEAR)]
    Year,
    #[value(YEAR_SIDEREAL)]
    YearSidereal,
    #[value(YEAR_TROPICAL)]
    YearTropical,
}

/// The units of `Instant`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<INSTANT, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<INSTANT, N>;
        fn value(&self, quantity: SIQuantity<INSTANT, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum InstantUnit {
    #[default]
    #[value(UNIX_SECOND)]
    UnixSecond,
    #[value(UNIX_MILLISECOND)]
    UnixMillisecond,
    #[value(GPS_SECOND)]
    GpsSecond,
    #[value(JULIAN_DATE)]
    JulianDate,
    #[value(MODIFIED_JULIAN_DATE)]
    ModifiedJulianDate,
}

/// The units of `Area`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<AREA, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<AREA, N>;
        fn value(&self, quantity: SIQuantity<AREA, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum AreaUnit {
    #[default]
    #[value(SQUARE_METRE)]
    SquareMetre,
    #[value(ACRE)]
    Acre,
    #[value(ARE)]
    Are,
    #[value(BARN)]
    Barn,
    #[value(CIRCULAR_MIL)]
    CircularMil,
    #[value(HECTARE)]
    Hectare,
    #[value(SQUARE_FOOT)]
    SquareFoot,
    #[value(SQUARE_INCH)]
    SquareInch,
    #[value(SQUARE_MILE)]
    SquareMile,
    #[value(SQUARE_YARD)]
    SquareYard,
    #[value(SALTUS)]
    Saltus,
    #[value(CENTURIA)]
    Centuria,
    #[value(HEREDIUM)]
    Heredium,
    #[value(JUGERUM)]
    Jugerum,
    #[value(AREA_DEUNX)]
    Deunx,
    #[value(AREA_DEXTANS)]
    Dextans,
    #[value(AREA_DODRANS)]
    Dodrans,
    #[value(AREA_BES)]
    Bes,
    #[value(AREA_SEPTUNX)]
    Septunx,
    #[value(AREA_SEMIS)]
    Semis,
    #[value(ACTUS_QUADRATUS)]
    ActusQuadratus,
    #[value(AREA_QUINCUNX)]
    Quincunx,
    #[value(AREA_TRIENS)]
    Triens,
    #[value(QUADRANS)]
    Quadrans,
    #[value(AREA_SEXTANS)]
    Sextans,
    #[value(CLIMA)]
    Clima,
    #[value(AREA_UNCIA)]
    Uncia,
    #[value(SEMIUNCIA)]
    Semiuncia,
    #[value(AREA_SICILICUS)]
    Sicilicus,
    #[value(ACTUS_SIMPLEX)]
    ActusSimplex,
    #[value(AREA_SEXTULA)]
    Sextula,
    #[value(DUO_SCRUPULUM)]
    DuoScrupulum,
    #[value(AREA_SCRUPULUM)]
    Scrupulum,
    #[value(DIMIDIUM_SCRUPULUM)]
    DimidiumScrupulum,
    #[value(PES_QUADRATUS)]
    PesQuadratus,
}

/// The units of `Volume`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<VOLUME, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<VOLUME, N>;
        fn value(&self, quantity: SIQuantity<VOLUME, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum VolumeUnit {
    #[default]
    #[value(CUBIC_METRE)]
    CubicMetre,
    #[value(ACRE_FOOT)]
    AcreFoot,
    #[value(BARREL)]
    Barrel,
    #[value(BUSHEL)]
    Bushel,
    #[value(CORD)]
    Cord,
    #[value(CUBIC_FOOT)]
    CubicFoot,
    #[value(CUBIC_INCH)]
    CubicInch,
    #[value(CUBIC_MILE)]
    CubicMile,
    #[value(CUBIC_YARD)]
    CubicYard,
    #[value(CUP)]
    Cup,
    #[value(FLUID_OUNCE)]
    FluidOunce,
    #[value(FLUID_OUNCE_IMPERIAL)]
    FluidOunceImperial,
    #[value(GALLON_IMPERIAL)]
    GallonImperial,
    #[value(GALLON)]
    Gallon,
    #[value(GILL_IMPERIAL)]
    GillImperial,
    #[value(GILL)]
    Gill,
    #[value(LITRE)]
    Litre,
    #[value(PECK)]
    Peck,
    #[value(PINT_DRY)]
    PintDry,
    #[value(PINT_LIQUID)]
    PintLiquid,
    #[value(QUART_DRY)]
    QuartDry,
    #[value(QUART_LIQUID)]
    QuartLiquid,
    #[value(STERE)]
    Stere,
    #[value(TABLESPOON)]
    Tablespoon,
    #[value(TEASPOON)]
    Teaspoon,
    #[value(REGISTER_TON)]
    RegisterTon,
    #[value(CULEUS)]
    Culeus,
    #[value(AMPHORA_QUADRANTAL)]
    AmphoraQuadrantal,
    #[value(URNA)]
    Urna,
    #[value(MODIUS_CASTRENSIS)]
    ModiusCastrensis,
    #[value(MODIUS)]
    Modius,
    #[value(SEMIMODIUS)]
    Semimodius,
    #[value(CONGIUS)]
    Congius,
    #[value(SEXTARIUS)]
    Sextarius,
    #[value(HEMINA)]
    Hemina,
    #[value(QUARTARIUS)]
    Quartarius,
    #[value(ACETABULUM)]
    Acetabulum,
    #[value(CYATHUS)]
    Cyathus,
    #[value(LIGULA)]
    Ligula,
}

/// The units of `Velocity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<VELOCITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<VELOCITY, N>;
        fn value(&self, quantity: SIQuantity<VELOCITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum VelocityUnit {
    #[default]
    #[value(METRE_PER_SECOND)]
    MetrePerSecond,
    #[value(FOOT_PER_HOUR)]
    FootPerHour,
    #[value(FOOT_PER_MINUTE)]
    FootPerMinute,
    #[value(FOOT_PER_SECOND)]
    FootPerSecond,
    #[value(INCH_PER_SECOND)]
    InchPerSecond,
    #[value(INCH_PER_MINUTE)]
    InchPerMinute,
    #[value(KNOT)]
    Knot,
    #[value(MILE_PER_HOUR)]
    MilePerHour,
    #[value(MILE_PER_MINUTE)]
    MilePerMinute,
    #[value(MILE_PER_SECOND)]
    MilePerSecond,
    #[value(ATOMIC_UNIT_OF_VELOCITY)]
    AtomicUnitOfVelocity,
    #[value(NATURAL_UNIT_OF_VELOCITY)]
    NaturalUnitOfVelocity,
    #[value(SPEED_OF_LIGHT)]
    SpeedOfLight,
    #[value(KILOMETRE_PER_HOUR)]
    KilometrePerHour,
    #[value(MILLIMETRE_PER_MINUTE)]
    MillimetrePerMinute,
}

/// The units of `Acceleration`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ACCELERATION, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ACCELERATION, N>;
        fn value(&self, quantity: SIQuantity<ACCELERATION, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum AccelerationUnit {
    #[default]
    #[value(METRE_PER_SECOND_SQUARED)]
    MetrePerSecondSquared,
    #[value(FOOT_PER_SECOND_SQUARED)]
    FootPerSecondSquared,
    #[value(GALILEO)]
    Galileo,
    #[value(INCH_PER_SECOND_SQUARED)]
    InchPerSecondSquared,
    #[value(STANDARD_GRAVITY)]
    StandardGravity,
    #[value(MILLIMETRE_PER_MINUTE_SQUARED)]
    MillimetrePerMinuteSquared,
}

/// The units of `Jerk`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<JERK, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<JERK, N>;
        fn value(&self, quantity: SIQuantity<JERK, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum JerkUnit {
    #[default]
    #[value(METRE_PER_SECOND_CUBED)]
    MetrePerSecondCubed,
    #[value(FOOT_PER_SECOND_CUBED)]
    FootPerSecondCubed,
    #[value(INCH_PER_SECOND_CUBED)]
    InchPerSecondCubed,
    #[value(KILOMETRE_PER_MINUTE_CUBED)]
    KilometrePerMinuteCubed,
}

/// The units of `Absement`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ABSEMENT, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ABSEMENT, N>;
        fn value(&self, quantity: SIQuantity<ABSEMENT, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum AbsementUnit {
    #[default]
    #[value(METRE_SECOND)]
    MetreSecond,
    #[value(FOOT_SECOND)]
    FootSecond,
    #[value(INCH_SECOND)]
    InchSecond,
    #[value(KILOMETRE_HOUR)]
    KilometreHour,
}

/// The units of `InverseVelocity`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<INVERSE_VELOCITY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<INVERSE_VELOCITY, N>;
        fn value(&self, quantity: SIQuantity<INVERSE_VELOCITY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum InverseVelocityUnit {
    #[default]
    #[value(SECOND_PER_METRE)]
    SecondPerMetre,
    #[value(HOUR_PER_FOOT)]
    HourPerFoot,
    #[value(MINUTE_PER_FOOT)]
    MinutePerFoot,
    #[value(SECOND_PER_FOOT)]
    SecondPerFoot,
    #[value(SECOND_PER_INCH)]
    SecondPerInch,
    #[value(HOUR_PER_MILE)]
    HourPerMile,
    #[value(MINUTE_PER_MILE)]
    MinutePerMile,
    #[value(SECOND_PER_MILE)]
    SecondPerMile,
    #[value(MINUTE_PER_KILOMETRE)]
    MinutePerKilometre,
    #[value(HOUR_PER_KILOMETRE)]
    HourPerKilometre,
}

/// The units of `Force`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<FORCE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<FORCE, N>;
        fn value(&self, quantity: SIQuantity<FORCE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ForceUnit {
    #[default]
    #[value(NEWTON)]
    Newton,
    #[value(DYNE)]
    Dyne,
    #[value(KILOGRAM_FORCE)]
    KilogramForce,
    #[value(KIP)]
    Kip,
    #[value(OUNCE_FORCE)]
    OunceForce,
    #[value(POUNDAL)]
    Poundal,
    #[value(POUND_FORCE)]
    PoundForce,
    #[value(TON_FORCE)]
    TonForce,
}

/// The units of `Momentum`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<MOMENTUM, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<MOMENTUM, N>;
        fn value(&self, quantity: SIQuantity<MOMENTUM, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MomentumUnit {
    #[default]
    #[value(KILOGRAM_METRE_PER_SECOND)]
    KilogramMetrePerSecond,
    #[value(GRAM_METRE_PER_SECOND)]
    GramMetrePerSecond,
    #[value(TON_METRE_PER_SECOND)]
    TonMetrePerSecond,
    #[value(KILOGRAM_METRE_PER_MINUTE)]
    KilogramMetrePerMinute,
    #[value(KILOGRAM_METRE_PER_HOUR)]
    KilogramMetrePerHour,
    #[value(KILOGRAM_METRE_PER_DAY)]
    KilogramMetrePerDay,
    #[value(SLUG_FOOT_PER_SECOND)]
    SlugFootPerSecond,
    #[value(SLUG_INCH_PER_SECOND)]
    SlugInchPerSecond,
    #[value(POUND_FOOT_PER_SECOND)]
    PoundFootPerSecond,
    #[value(POUND_INCH_PER_SECOND)]
    PoundInchPerSecond,
}

/// The units of `Energy`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ENERGY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ENERGY, N>;
        fn value(&self, quantity: SIQuantity<ENERGY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum EnergyUnit {
    #[default]
    #[value(JOULE)]
    Joule,
    #[value(WATT_HOUR)]
    WattHour,
    #[value(ELECTRONVOLT)]
    Electronvolt,
    #[value(HARTREE)]
    Hartree,
    #[value(BTU_IT)]
    BtuIt,
    #[value(BTU)]
    Btu,
    #[value(CALORIE_IT)]
    CalorieIt,
    #[value(CALORIE)]
    Calorie,
    #[value(CALORIE_IT_NUTRITION)]
    CalorieItNutrition,
    #[value(CALORIE_NUTRITION)]
    CalorieNutrition,
    #[value(ERG)]
    Erg,
    #[value(FOOT_POUNDAL)]
    FootPoundal,
    #[value(FOOT_POUND)]
    FootPound,
    #[value(QUAD)]
    Quad,
    #[value(THERM_EC)]
    ThermEc,
    #[value(THERM_US)]
    ThermUs,
    #[value(TON_TNT)]
    TonTnt,
    #[value(WATT_SECOND)]
    WattSecond,
}

/// The units of `AvailableEnergy`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<AVAILABLE_ENERGY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<AVAILABLE_ENERGY, N>;
        fn value(&self, quantity: SIQuantity<AVAILABLE_ENERGY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum AvailableEnergyUnit {
    #[default]
    #[value(JOULE_PER_KILOGRAM)]
    JoulePerKilogram,
    #[value(JOULE_PER_GRAM)]
    JoulePerGram,
    #[value(BTU_IT_PER_POUND)]
    BtuItPerPound,
    #[value(BTU_PER_POUND)]
    BtuPerPound,
    #[value(CALORIE_IT_PER_GRAM)]
    CalorieItPerGram,
    #[value(CALORIE_PER_GRAM)]
    CaloriePerGram,
}

/// The units of `Power`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<POWER, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<POWER, N>;
        fn value(&self, quantity: SIQuantity<POWER, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum PowerUnit {
    #[default]
    #[value(WATT)]
    Watt,
    #[value(ERG_PER_SECOND)]
    ErgPerSecond,
    #[value(FOOT_POUND_PER_HOUR)]
    FootPoundPerHour,
    #[value(FOOT_POUND_PER_MINUTE)]
    FootPoundPerMinute,
    #[value(FOOT_POUND_PER_SECOND)]
    FootPoundPerSecond,
    #[value(HORSEPOWER)]
    Horsepower,
    #[value(HORSEPOWER_BOILER)]
    HorsepowerBoiler,
    #[value(HORSEPOWER_ELECTRIC)]
    HorsepowerElectric,
    #[value(HORSEPOWER_METRIC)]
    HorsepowerMetric,
    #[value(HORSEPOWER_IMPERIAL)]
    HorsepowerImperial,
    #[value(HYDRAULIC_HORSEPOWER)]
    HydraulicHorsepower,
}

/// The units of `Gain`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<GAIN, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<GAIN, N>;
        fn value(&self, quantity: SIQuantity<GAIN, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum GainUnit {
    #[default]
    #[value(BEL)]
    Bel,
    #[value(DECIBEL)]
    Decibel,
    #[value(NEPER)]
    Neper,
}

/// The units of `PowerLevel`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<POWER_LEVEL, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<POWER_LEVEL, N>;
        fn value(&self, quantity: SIQuantity<POWER_LEVEL, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum PowerLevelUnit {
    #[default]
    #[value(DECIBEL_WATT)]
    DecibelWatt,
    #[value(DECIBEL_MILLIWATT)]
    DecibelMilliwatt,
}

/// The units of `PowerRate`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<POWER_RATE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<POWER_RATE, N>;
        fn value(&self, quantity: SIQuantity<POWER_RATE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum PowerRateUnit {
    #[default]
    #[value(WATT_PER_SECOND)]
    WattPerSecond,
    #[value(ERG_PER_SECOND_SQUARED)]
    ErgPerSecondSquared,
    #[value(FOOT_POUND_PER_SECOND_SQUARED)]
    FootPoundPerSecondSquared,
    #[value(HORSEPOWER_PER_SECOND)]
    HorsepowerPerSecond,
    #[value(HORSEPOWER_PER_SECOND_BOILER)]
    HorsepowerPerSecondBoiler,
    #[value(HORSEPOWER_PER_SECOND_ELECTRIC)]
    HorsepowerPerSecondElectric,
    #[value(HORSEPOWER_PER_SECOND_METRIC)]
    HorsepowerPerSecondMetric,
    #[value(HORSEPOWER_PER_SECOND_IMPERIAL)]
    HorsepowerPerSecondImperial,
    #[value(HYDRAULIC_HORSEPOWER_PER_SECOND)]
    HydraulicHorsepowerPerSecond,
}

/// The units of `Action`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<ACTION, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<ACTION, N>;
        fn value(&self, quantity: SIQuantity<ACTION, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ActionUnit {
    #[default]
    #[value(JOULE_SECOND)]
    JouleSecond,
    #[value(ATOMIC_UNIT_OF_ACTION)]
    AtomicUnitOfAction,
    #[value(PLANCK_CONSTANT)]
    PlanckConstant,
    #[value(ERG_SECOND)]
    ErgSecond,
    #[value(ELECTRONVOLT_SECOND)]
    ElectronvoltSecond,
}

/// The units of `Torque`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<TORQUE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<TORQUE, N>;
        fn value(&self, quantity: SIQuantity<TORQUE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum TorqueUnit {
    #[default]
    #[value(NEWTON_METRE)]
    NewtonMetre,
    #[value(DYNE_METRE)]
    DyneMetre,
    #[value(KILOGRAM_FORCE_METRE)]
    KilogramForceMetre,
    #[value(OUNCE_FORCE_INCH)]
    OunceForceInch,
    #[value(POUND_FORCE_FOOT)]
    PoundForceFoot,
    #[value(POUND_FORCE_INCH)]
    PoundForceInch,
}

/// The units of `MomentOfInertia`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<MOMENT_OF_INERTIA, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<MOMENT_OF_INERTIA, N>;
        fn value(&self, quantity: SIQuantity<MOMENT_OF_INERTIA, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum MomentOfInertiaUnit {
    #[default]
    #[value(KILOGRAM_SQUARE_METRE)]
    KilogramSquareMetre,
    #[value(DALTON_SQUARE_ANGSTROM)]
    DaltonSquareAngstrom,
}

/// The units of `Pressure`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<PRESSURE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<PRESSURE, N>;
        fn value(&self, quantity: SIQuantity<PRESSURE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum PressureUnit {
    #[default]
    #[value(PASCAL)]
    Pascal,
    #[value(STANDARD_ATMOSPHERE)]
    StandardAtmosphere,
    #[value(ATMOSPHERE_TECHNICAL)]
    AtmosphereTechnical,
    #[value(BAR)]
    Bar,
    #[value(CENTIMETRE_OF_MERCURY)]
    CentimetreOfMercury,
    #[value(CENTIMETRE_OF_WATER)]
    CentimetreOfWater,
    #[value(FOOT_OF_MERCURY)]
    FootOfMercury,
    #[value(FOOT_OF_WATER)]
    FootOfWater,
    #[value(INCH_OF_MERCURY)]
    InchOfMercury,
    #[value(INCH_OF_WATER)]
    InchOfWater,
    #[value(KIP_PER_SQUARE_INCH)]
    KipPerSquareInch,
    #[value(MILLIMETRE_OF_MERCURY)]
    MillimetreOfMercury,
    #[value(MILLIMETRE_OF_WATER)]
    MillimetreOfWater,
    #[value(POUNDAL_PER_SQUARE_FOOT)]
    PoundalPerSquareFoot,
    #[value(POUND_FORCE_PER_SQUARE_FOOT)]
    PoundForcePerSquareFoot,
    #[value(POUND_FORCE_PER_SQUARE_INCH)]
    PoundForcePerSquareInch,
    #[value(PSI)]
    Psi,
    #[value(TORR)]
    Torr,
    #[value(DYNE_PER_SQUARE_CENTIMETRE)]
    DynePerSquareCentimetre,
    #[value(NEWTON_PER_SQUARE_MILLIMETRE)]
    NewtonPerSquareMillimetre,
}

/// The units of `Frequency`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<FREQUENCY, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<FREQUENCY, N>;
        fn value(&self, quantity: SIQuantity<FREQUENCY, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum FrequencyUnit {
    #[default]
    #[value(HERTZ)]
    Hertz,
    #[value(CYCLE_PER_DAY)]
    CyclePerDay,
    #[value(CYCLE_PER_HOUR)]
    CyclePerHour,
    #[value(CYCLE_PER_MINUTE)]
    CyclePerMinute,
    #[value(CYCLE_PER_SHAKE)]
    CyclePerShake,
    #[value(CYCLE_PER_YEAR)]
    CyclePerYear,
}

/// The units of `FrequencyDrift`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<FREQUENCY_DRIFT, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<FREQUENCY_DRIFT, N>;
        fn value(&self, quantity: SIQuantity<FREQUENCY_DRIFT, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum FrequencyDriftUnit {
    #[default]
    #[value(HERTZ_PER_SECOND)]
    HertzPerSecond,
}

/// The units of `ReciprocalLength`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<RECIPROCAL_LENGTH, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<RECIPROCAL_LENGTH, N>;
        fn value(&self, quantity: SIQuantity<RECIPROCAL_LENGTH, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum ReciprocalLengthUnit {
    #[default]
    #[value(RECIPROCAL_METRE)]
    ReciprocalMetre,
    #[value(RECIPROCAL_ANGSTROM)]
    ReciprocalAngstrom,
    #[value(DIOPTER)]
    Diopter,
    #[value(RYDBERG_CONSTANT)]
    RydbergConstant,
}

/// The units of `Curvature`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<CURVATURE, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<CURVATURE, N>;
        fn value(&self, quantity: SIQuantity<CURVATURE, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum CurvatureUnit {
    #[default]
    #[value(RADIAN_PER_METRE)]
    RadianPerMetre,
    #[value(DEGREE_PER_METRE)]
    DegreePerMetre,
}

/// The units of `SpecificArea`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<SPECIFIC_AREA, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<SPECIFIC_AREA, N>;
        fn value(&self, quantity: SIQuantity<SPECIFIC_AREA, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum SpecificAreaUnit {
    #[default]
    #[value(SQUARE_METRE_PER_KILOGRAM)]
    SquareMetrePerKilogram,
    #[value(SQUARE_METRE_PER_GRAM)]
    SquareMetrePerGram,
}

/// The units of `SpecificVolume`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<SPECIFIC_VOLUME, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<SPECIFIC_VOLUME, N>;
        fn value(&self, quantity: SIQuantity<SPECIFIC_VOLUME, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum SpecificVolumeUnit {
    #[default]
    #[value(CUBIC_METRE_PER_KILOGRAM)]
    CubicMetrePerKilogram,
    #[value(CUBIC_METRE_PER_GRAM)]
    CubicMetrePerGram,
    #[value(CUBIC_METRE_PER_CARAT)]
    CubicMetrePerCarat,
    #[value(CUBIC_METRE_PER_GRAIN)]
    CubicMetrePerGrain,
    #[value(CUBIC_METRE_PER_HUNDREDWEIGHT_LONG)]
    CubicMetrePerHundredweightLong,
    #[value(CUBIC_METRE_PER_HUNDREDWEIGHT_SHORT)]
    CubicMetrePerHundredweightShort,
    #[value(CUBIC_METRE_PER_OUNCE)]
    CubicMetrePerOunce,
    #[value(CUBIC_METRE_PER_OUNCE_TROY)]
    CubicMetrePerOunceTroy,
    #[value(CUBIC_METRE_PER_PENNYWEIGHT)]
    CubicMetrePerPennyweight,
    #[value(CUBIC_METRE_PER_POUND)]
    CubicMetrePerPound,
    #[value(CUBIC_METRE_PER_POUND_TROY)]
    CubicMetrePerPoundTroy,
    #[value(CUBIC_METRE_PER_SLUG)]
    CubicMetrePerSlug,
    #[value(CUBIC_METRE_PER_TON_ASSAY)]
    CubicMetrePerTonAssay,
    #[value(CUBIC_METRE_PER_TON_LONG)]
    CubicMetrePerTonLong,
    #[value(CUBIC_METRE_PER_TON_SHORT)]
    CubicMetrePerTonShort,
    #[value(CUBIC_METRE_PER_TON)]
    CubicMetrePerTon,
    #[value(GALLON_PER_GRAIN)]
    GallonPerGrain,
    #[value(CUBIC_INCH_PER_OUNCE)]
    CubicInchPerOunce,
    #[value(GALLON_IMPERIAL_PER_OUNCE)]
    GallonImperialPerOunce,
    #[value(GALLON_PER_OUNCE)]
    GallonPerOunce,
    #[value(CUBIC_FOOT_PER_POUND)]
    CubicFootPerPound,
    #[value(CUBIC_INCH_PER_POUND)]
    CubicInchPerPound,
    #[value(CUBIC_YARD_PER_POUND)]
    CubicYardPerPound,
    #[value(GALLON_IMPERIAL_PER_POUND)]
    GallonImperialPerPound,
    #[value(GALLON_PER_POUND)]
    GallonPerPound,
    #[value(CUBIC_FOOT_PER_SLUG)]
    CubicFootPerSlug,
    #[value(CUBIC_YARD_PER_TON_LONG)]
    CubicYardPerTonLong,
    #[value(CUBIC_YARD_PER_TON_SHORT)]
    CubicYardPerTonShort,
    #[value(CUBIC_FOOT_PER_TON_LONG)]
    CubicFootPerTonLong,
    #[value(CUBIC_FOOT_PER_TON_SHORT)]
    CubicFootPerTonShort,
}

/// The units of `SpecificPower`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<SPECIFIC_POWER, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<SPECIFIC_POWER, N>;
        fn value(&self, quantity: SIQuantity<SPECIFIC_POWER, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum SpecificPowerUnit {
    #[default]
    #[value(WATT_PER_KILOGRAM)]
    WattPerKilogram,
    #[value(SQUARE_METRE_PER_CUBIC_SECOND)]
    SquareMetrePerCubicSecond,
    #[value(ERG_PER_SECOND_PER_KILOGRAM)]
    ErgPerSecondPerKilogram,
    #[value(FOOT_POUND_PER_HOUR_PER_POUND)]
    FootPoundPerHourPerPound,
    #[value(FOOT_POUND_PER_MINUTE_PER_POUND)]
    FootPoundPerMinutePerPound,
    #[value(FOOT_POUND_PER_SECOND_PER_POUND)]
    FootPoundPerSecondPerPound,
    #[value(HORSEPOWER_PER_POUND)]
    HorsepowerPerPound,
    #[value(HORSEPOWER_BOILER_PER_POUND)]
    HorsepowerBoilerPerPound,
    #[value(HORSEPOWER_ELECTRIC_PER_POUND)]
    HorsepowerElectricPerPound,
    #[value(HORSEPOWER_METRIC_PER_POUND)]
    HorsepowerMetricPerPound,
    #[value(HORSEPOWER_IMPERIAL_PER_POUND)]
    HorsepowerImperialPerPound,
    #[value(HYDRAULIC_HORSEPOWER_PER_POUND)]
    HydraulicHorsepowerPerPound,
}

/// The units of `SpecificGravitationalParameter`, in any number type they count in.
#[derive_const(Default)]
#[derive(dynamickit::Values, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[values(
    delegate(Unit<SIQuantity<SPECIFIC_GRAVITATIONAL_PARAMETER, N>, N> {
        fn quantity(&self, value: N) -> SIQuantity<SPECIFIC_GRAVITATIONAL_PARAMETER, N>;
        fn value(&self, quantity: SIQuantity<SPECIFIC_GRAVITATIONAL_PARAMETER, N>) -> N;
    }),
    into(DynamicSIUnit),
    for<N: From<Exact> + Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Div<Output = N>>
)]
pub enum SpecificGravitationalParameterUnit {
    #[default]
    #[value(CUBIC_METRE_PER_KILOGRAM_SECOND_SQUARED)]
    CubicMetrePerKilogramSecondSquared,
    #[value(GRAVITATIONAL_CONSTANT)]
    GravitationalConstant,
}
