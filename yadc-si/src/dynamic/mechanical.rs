//! The units of `mechanical`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::base::{ONE, RADIAN};
use crate::mechanical::consts::*;
use crate::mechanical::*;
use crate::{Exact, Scale};
use core::marker::Destruct;
use yadc::{Unit, num_traits::Num};

/// The units of `Length`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Length<N>: [const] Unit<Length<N>, N> + [const] Destruct)]
pub enum LengthUnit {
    #[default]
    #[unit("m", "metres", by = Length::<N>::from(METRE))]
    Metre,
    #[unit("Å", "ångströms", by = Length::<N>::from(ANGSTROM))]
    Angstrom,
    #[unit("a₀", "bohr radiuses", by = Length::<N>::from(BOHR_RADIUS))]
    BohrRadius,
    #[unit("a.u. of length", "atomic units of length", by = Length::<N>::from(ATOMIC_UNIT_OF_LENGTH))]
    AtomicUnitOfLength,
    #[unit("ua", "astronomical units", by = Length::<N>::from(ASTRONOMICAL_UNIT))]
    AstronomicalUnit,
    #[unit("ch", "chains", by = Length::<N>::from(CHAIN))]
    Chain,
    #[unit("DM", "data miles", by = Length::<N>::from(DATA_MILE))]
    DataMile,
    #[unit("fathom", "fathoms", by = Length::<N>::from(FATHOM))]
    Fathom,
    #[unit("fermi", "fermis", by = Length::<N>::from(FERMI))]
    Fermi,
    #[unit("ft", "feet", by = Length::<N>::from(FOOT))]
    Foot,
    #[unit("ft (U.S. survey)", "feet (U.S. survey)", by = Length::<N>::from(FOOT_SURVEY))]
    FootSurvey,
    #[unit("in", "inches", by = Length::<N>::from(INCH))]
    Inch,
    #[unit("l. y.", "light years", by = Length::<N>::from(LIGHT_YEAR))]
    LightYear,
    #[unit("μ", "microns", by = Length::<N>::from(MICRON))]
    Micron,
    #[unit("0.001 in", "mils", by = Length::<N>::from(MIL))]
    Mil,
    #[unit("mi", "miles", by = Length::<N>::from(MILE))]
    Mile,
    #[unit("mi (U.S. survey)", "miles (U.S. survey)", by = Length::<N>::from(MILE_SURVEY))]
    MileSurvey,
    #[unit("M", "nautical miles", by = Length::<N>::from(NAUTICAL_MILE))]
    NauticalMile,
    #[unit("pc", "parsecs", by = Length::<N>::from(PARSEC))]
    Parsec,
    #[unit("1/6 in (computer)", "picas (computer)", by = Length::<N>::from(PICA_COMPUTER))]
    PicaComputer,
    #[unit("1/6 in", "picas (printer's)", by = Length::<N>::from(PICA_PRINTERS))]
    PicaPrinters,
    #[unit("1/72 in (computer)", "points (computer)", by = Length::<N>::from(POINT_COMPUTER))]
    PointComputer,
    #[unit("1/72 in", "points (printer's)", by = Length::<N>::from(POINT_PRINTERS))]
    PointPrinters,
    #[unit("rd", "rods", by = Length::<N>::from(ROD))]
    Rod,
    #[unit("yd", "yards", by = Length::<N>::from(YARD))]
    Yard,
    #[unit("leuga", "leugae", by = Length::<N>::from(LEUGA))]
    Leuga,
    #[unit("mille passus", "mille passa", by = Length::<N>::from(MILLE_PASSUS))]
    MillePassus,
    #[unit("stadium", "stadia", by = Length::<N>::from(STADIUM))]
    Stadium,
    #[unit("actus", "acta", by = Length::<N>::from(ACTUS))]
    Actus,
    #[unit("decempeda", "decempedae", by = Length::<N>::from(DECEMPEDA))]
    Decempeda,
    #[unit("passus", "passa", by = Length::<N>::from(PASSUS))]
    Passus,
    #[unit("gradus", "gradus", by = Length::<N>::from(GRADUS))]
    Gradus,
    #[unit("cubitum", "cubita", by = Length::<N>::from(CUBITUM))]
    Cubitum,
    #[unit("palmipes", "palmipedes", by = Length::<N>::from(PALMIPES))]
    Palmipes,
    #[unit("pes", "pedes", by = Length::<N>::from(PES))]
    Pes,
    #[unit("palmus maior", "palmas maior", by = Length::<N>::from(PALMUS_MAIOR))]
    PalmusMaior,
    #[unit("palmus", "palmas", by = Length::<N>::from(PALMUS))]
    Palmus,
    #[unit("uncia", "unciae", by = Length::<N>::from(LENGTH_UNCIA))]
    Uncia,
    #[unit("digitus", "digiti", by = Length::<N>::from(DIGITUS))]
    Digitus,
}

/// The units of `Mass`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Mass<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Mass<N>: [const] Unit<Mass<N>, N> + [const] Destruct)]
pub enum MassUnit {
    #[default]
    #[unit("kg", "kilograms", by = Mass::<N>::from(KILOGRAM))]
    Kilogram,
    #[unit("g", "grams", by = Mass::<N>::from(GRAM))]
    Gram,
    #[unit("ct", "carats", by = Mass::<N>::from(CARAT))]
    Carat,
    #[unit("Da", "daltons", by = Mass::<N>::from(DALTON))]
    Dalton,
    #[unit("mₑ", "electron masses", by = Mass::<N>::from(ELECTRON_MASS))]
    ElectronMass,
    #[unit("mₚ", "proton masses", by = Mass::<N>::from(PROTON_MASS))]
    ProtonMass,
    #[unit("gr", "grains", by = Mass::<N>::from(GRAIN))]
    Grain,
    #[unit("cwt long", "hundredweight (long)", by = Mass::<N>::from(HUNDREDWEIGHT_LONG))]
    HundredweightLong,
    #[unit("cwt short", "hundredweight (short)", by = Mass::<N>::from(HUNDREDWEIGHT_SHORT))]
    HundredweightShort,
    #[unit("oz", "ounces", by = Mass::<N>::from(OUNCE))]
    Ounce,
    #[unit("oz t", "troy ounces", by = Mass::<N>::from(OUNCE_TROY))]
    OunceTroy,
    #[unit("dwt", "pennyweight", by = Mass::<N>::from(PENNYWEIGHT))]
    Pennyweight,
    #[unit("lb", "pounds", by = Mass::<N>::from(POUND))]
    Pound,
    #[unit("lb t", "troy pounds", by = Mass::<N>::from(POUND_TROY))]
    PoundTroy,
    #[unit("slug", "slugs", by = Mass::<N>::from(SLUG))]
    Slug,
    #[unit("AT", "assay tons", by = Mass::<N>::from(TON_ASSAY))]
    TonAssay,
    #[unit("2240 lb", "long tons", by = Mass::<N>::from(TON_LONG))]
    TonLong,
    #[unit("2000 lb", "short tons", by = Mass::<N>::from(TON_SHORT))]
    TonShort,
    #[unit("t", "tons", by = Mass::<N>::from(TON))]
    Ton,
    #[unit("libra", "librae", by = Mass::<N>::from(LIBRA))]
    Libra,
    #[unit("deunx", "deunces", by = Mass::<N>::from(MASS_DEUNX))]
    Deunx,
    #[unit("dextans", "dextantes", by = Mass::<N>::from(MASS_DEXTANS))]
    Dextans,
    #[unit("dodrans", "dodrantes", by = Mass::<N>::from(MASS_DODRANS))]
    Dodrans,
    #[unit("bes", "bessis", by = Mass::<N>::from(MASS_BES))]
    Bes,
    #[unit("septunx", "septunces", by = Mass::<N>::from(MASS_SEPTUNX))]
    Septunx,
    #[unit("semis", "semisses", by = Mass::<N>::from(MASS_SEMIS))]
    Semis,
    #[unit("quincunx", "quincunx", by = Mass::<N>::from(MASS_QUINCUNX))]
    Quincunx,
    #[unit("triens", "trientes", by = Mass::<N>::from(MASS_TRIENS))]
    Triens,
    #[unit("quadrans teruncius", "quadrantes teruncius", by = Mass::<N>::from(QUADRANS_TERUNCIUS))]
    QuadransTeruncius,
    #[unit("sextans", "sextantes", by = Mass::<N>::from(MASS_SEXTANS))]
    Sextans,
    #[unit("sescuncia", "sescunciae", by = Mass::<N>::from(SESCUNCIA))]
    Sescuncia,
    #[unit("uncia", "unciae", by = Mass::<N>::from(MASS_UNCIA))]
    Uncia,
    #[unit("semuncia", "semunciae", by = Mass::<N>::from(SEMUNCIA))]
    Semuncia,
    #[unit("duella", "duella", by = Mass::<N>::from(DUELLA))]
    Duella,
    #[unit("sicilicus", "scilici", by = Mass::<N>::from(MASS_SICILICUS))]
    Sicilicus,
    #[unit("sextula", "sextulae", by = Mass::<N>::from(MASS_SEXTULA))]
    Sextula,
    #[unit("semisextula", "semisextulae", by = Mass::<N>::from(SEMISEXTULA))]
    Semisextula,
    #[unit("scrupulum", "scrupula", by = Mass::<N>::from(MASS_SCRUPULUM))]
    Scrupulum,
    #[unit("obolus", "oboli", by = Mass::<N>::from(OBOLUS))]
    Obolus,
    #[unit("siliqua", "siliquae", by = Mass::<N>::from(SILIQUA))]
    Siliqua,
}

/// The units of `Time`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Time<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Time<N>: [const] Unit<Time<N>, N> + [const] Destruct)]
pub enum TimeUnit {
    #[default]
    #[unit("s", "seconds", by = Time::<N>::from(SECOND))]
    Second,
    #[unit("s (sidereal)", "seconds (sidereal)", by = Time::<N>::from(SECOND_SIDEREAL))]
    SecondSidereal,
    #[unit("d", "days", by = Time::<N>::from(DAY))]
    Day,
    #[unit("d (sidereal)", "days (sidereal)", by = Time::<N>::from(DAY_SIDEREAL))]
    DaySidereal,
    #[unit("h", "hours", by = Time::<N>::from(HOUR))]
    Hour,
    #[unit("h (sidereal)", "hours (sidereal)", by = Time::<N>::from(HOUR_SIDEREAL))]
    HourSidereal,
    #[unit("min", "minutes", by = Time::<N>::from(MINUTE))]
    Minute,
    #[unit("10.0 ns", "shakes", by = Time::<N>::from(SHAKE))]
    Shake,
    #[unit("a", "years", by = Time::<N>::from(YEAR))]
    Year,
    #[unit("a (sidereal)", "years (sidereal)", by = Time::<N>::from(YEAR_SIDEREAL))]
    YearSidereal,
    #[unit("a (tropical)", "years (tropical)", by = Time::<N>::from(YEAR_TROPICAL))]
    YearTropical,
}

/// The units of `Instant`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Instant<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Scale<Instant<N>, Time<N>>: [const] Unit<Instant<N>, N> + [const] Destruct)]
pub enum InstantUnit {
    #[default]
    #[unit("s Unix", "seconds of Unix time", by = Scale::<Instant<N>, Time<N>>::from(UNIX_SECONDS))]
    UnixSecond,
    #[unit("ms Unix", "milliseconds of Unix time", by = Scale::<Instant<N>, Time<N>>::from(UNIX_MILLISECONDS))]
    UnixMillisecond,
    #[unit("s GPS", "seconds of GPS time", by = Scale::<Instant<N>, Time<N>>::from(GPS_SECONDS))]
    GpsSecond,
    #[unit("JD", "Julian date", by = Scale::<Instant<N>, Time<N>>::from(JULIAN_DATE))]
    JulianDate,
    #[unit("MJD", "modified Julian date", by = Scale::<Instant<N>, Time<N>>::from(MODIFIED_JULIAN_DATE))]
    ModifiedJulianDate,
}

/// The units of `Area`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Area<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Area<N>: [const] Unit<Area<N>, N> + [const] Destruct)]
pub enum AreaUnit {
    #[default]
    #[unit("m²", "square metres", by = Area::<N>::from(SQUARE_METRE))]
    SquareMetre,
    #[unit("ac", "acres", by = Area::<N>::from(ACRE))]
    Acre,
    #[unit("a", "ares", by = Area::<N>::from(ARE))]
    Are,
    #[unit("b", "barns", by = Area::<N>::from(BARN))]
    Barn,
    #[unit("cmil", "circular mils", by = Area::<N>::from(CIRCULAR_MIL))]
    CircularMil,
    #[unit("ha", "hectares", by = Area::<N>::from(HECTARE))]
    Hectare,
    #[unit("ft²", "square feet", by = Area::<N>::from(SQUARE_FOOT))]
    SquareFoot,
    #[unit("in²", "square inches", by = Area::<N>::from(SQUARE_INCH))]
    SquareInch,
    #[unit("mi²", "square miles", by = Area::<N>::from(SQUARE_MILE))]
    SquareMile,
    #[unit("yd²", "square yards", by = Area::<N>::from(SQUARE_YARD))]
    SquareYard,
    #[unit("saltus", "saltus", by = Area::<N>::from(SALTUS))]
    Saltus,
    #[unit("centuria", "centuriae", by = Area::<N>::from(CENTURIA))]
    Centuria,
    #[unit("heredium", "heredia", by = Area::<N>::from(HEREDIUM))]
    Heredium,
    #[unit("jugerum", "jugera", by = Area::<N>::from(JUGERUM))]
    Jugerum,
    #[unit("deunx", "deunces", by = Area::<N>::from(AREA_DEUNX))]
    Deunx,
    #[unit("dextans", "dextantes", by = Area::<N>::from(AREA_DEXTANS))]
    Dextans,
    #[unit("dodrans", "dodrantes", by = Area::<N>::from(AREA_DODRANS))]
    Dodrans,
    #[unit("bes", "besses", by = Area::<N>::from(AREA_BES))]
    Bes,
    #[unit("septunx", "septunces", by = Area::<N>::from(AREA_SEPTUNX))]
    Septunx,
    #[unit("semis", "semisses", by = Area::<N>::from(AREA_SEMIS))]
    Semis,
    #[unit("actus quadratus", "acta quadratus", by = Area::<N>::from(ACTUS_QUADRATUS))]
    ActusQuadratus,
    #[unit("quincunx", "quincunces", by = Area::<N>::from(AREA_QUINCUNX))]
    Quincunx,
    #[unit("triens", "trientes", by = Area::<N>::from(AREA_TRIENS))]
    Triens,
    #[unit("quadrans", "quadrantes", by = Area::<N>::from(QUADRANS))]
    Quadrans,
    #[unit("sextans", "sextantes", by = Area::<N>::from(AREA_SEXTANS))]
    Sextans,
    #[unit("clima", "climata", by = Area::<N>::from(CLIMA))]
    Clima,
    #[unit("uncia", "unciae", by = Area::<N>::from(AREA_UNCIA))]
    Uncia,
    #[unit("semiuncia", "semiunciae", by = Area::<N>::from(SEMIUNCIA))]
    Semiuncia,
    #[unit("sicilicus", "siculi", by = Area::<N>::from(AREA_SICILICUS))]
    Sicilicus,
    #[unit("actus simplex", "acta simplex", by = Area::<N>::from(ACTUS_SIMPLEX))]
    ActusSimplex,
    #[unit("sextula", "sextulae", by = Area::<N>::from(AREA_SEXTULA))]
    Sextula,
    #[unit("duo scrupulum", "duo scrupula", by = Area::<N>::from(DUO_SCRUPULUM))]
    DuoScrupulum,
    #[unit("scrupulum", "scrupula", by = Area::<N>::from(AREA_SCRUPULUM))]
    Scrupulum,
    #[unit("dimidium scrupulum", "dimidium scupula", by = Area::<N>::from(DIMIDIUM_SCRUPULUM))]
    DimidiumScrupulum,
    #[unit("pes quadratus", "pedes quadratus", by = Area::<N>::from(PES_QUADRATUS))]
    PesQuadratus,
}

/// The units of `Volume`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Volume<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Volume<N>: [const] Unit<Volume<N>, N> + [const] Destruct)]
pub enum VolumeUnit {
    #[default]
    #[unit("m³", "cubic metres", by = Volume::<N>::from(CUBIC_METRE))]
    CubicMetre,
    #[unit("ac · ft", "acre-feet", by = Volume::<N>::from(CUBIC_METRE * (Exact::int(43560) * (Exact::int(1200) / Exact::int(3937)) * (Exact::int(1200) / Exact::int(3937)) * (Exact::int(1200) / Exact::int(3937)))))]
    AcreFoot,
    #[unit("bbl", "barrels", by = Volume::<N>::from(BARREL))]
    Barrel,
    #[unit("bu", "bushels", by = Volume::<N>::from(BUSHEL))]
    Bushel,
    #[unit("cords", "cords", by = Volume::<N>::from(CORD))]
    Cord,
    #[unit("ft³", "cubic feet", by = Volume::<N>::from(CUBIC_FOOT))]
    CubicFoot,
    #[unit("in³", "cubic inches", by = Volume::<N>::from(CUBIC_INCH))]
    CubicInch,
    #[unit("mi³", "cubic miles", by = Volume::<N>::from(CUBIC_MILE))]
    CubicMile,
    #[unit("yd³", "cubic yards", by = Volume::<N>::from(CUBIC_YARD))]
    CubicYard,
    #[unit("cup", "cups", by = Volume::<N>::from(CUP))]
    Cup,
    #[unit("fl oz", "fluid ounces", by = Volume::<N>::from(FLUID_OUNCE))]
    FluidOunce,
    #[unit("fl oz (UK)", "Imperial fluid ounces", by = Volume::<N>::from(FLUID_OUNCE_IMPERIAL))]
    FluidOunceImperial,
    #[unit("gal (UK)", "Imperial gallons", by = Volume::<N>::from(GALLON_IMPERIAL))]
    GallonImperial,
    #[unit("gal", "gallons", by = Volume::<N>::from(GALLON))]
    Gallon,
    #[unit("gi (UK)", "Imperial gills", by = Volume::<N>::from(GILL_IMPERIAL))]
    GillImperial,
    #[unit("gi", "gills", by = Volume::<N>::from(GILL))]
    Gill,
    #[unit("L", "litres", by = Volume::<N>::from(LITRE))]
    Litre,
    #[unit("pk", "pecks", by = Volume::<N>::from(PECK))]
    Peck,
    #[unit("dry pt", "dry pints", by = Volume::<N>::from(PINT_DRY))]
    PintDry,
    #[unit("liq pt", "liquid pints", by = Volume::<N>::from(PINT_LIQUID))]
    PintLiquid,
    #[unit("dry qt", "dry quarts", by = Volume::<N>::from(QUART_DRY))]
    QuartDry,
    #[unit("liq qt", "liquid quarts", by = Volume::<N>::from(QUART_LIQUID))]
    QuartLiquid,
    #[unit("st", "steres", by = Volume::<N>::from(STERE))]
    Stere,
    #[unit("tbsp", "tablespoons", by = Volume::<N>::from(TABLESPOON))]
    Tablespoon,
    #[unit("tsp", "teaspoons", by = Volume::<N>::from(TEASPOON))]
    Teaspoon,
    #[unit("RT", "register tons", by = Volume::<N>::from(REGISTER_TON))]
    RegisterTon,
    #[unit("culeus", "culei", by = Volume::<N>::from(CULEUS))]
    Culeus,
    #[unit("amphora quadrantal", "amphora quadrantal", by = Volume::<N>::from(AMPHORA_QUADRANTAL))]
    AmphoraQuadrantal,
    #[unit("urna", "urnae", by = Volume::<N>::from(URNA))]
    Urna,
    #[unit("modius castrensis", "modii castrensis", by = Volume::<N>::from(MODIUS_CASTRENSIS))]
    ModiusCastrensis,
    #[unit("modius", "modii", by = Volume::<N>::from(MODIUS))]
    Modius,
    #[unit("semimodius", "semimodii", by = Volume::<N>::from(SEMIMODIUS))]
    Semimodius,
    #[unit("congius", "congii", by = Volume::<N>::from(CONGIUS))]
    Congius,
    #[unit("sextarius", "sextarii", by = Volume::<N>::from(SEXTARIUS))]
    Sextarius,
    #[unit("hemina", "heminae", by = Volume::<N>::from(HEMINA))]
    Hemina,
    #[unit("quartarius", "quartarii", by = Volume::<N>::from(QUARTARIUS))]
    Quartarius,
    #[unit("acetabulum", "acetabula", by = Volume::<N>::from(ACETABULUM))]
    Acetabulum,
    #[unit("cyathus", "cyathi", by = Volume::<N>::from(CYATHUS))]
    Cyathus,
    #[unit("ligula", "ligula", by = Volume::<N>::from(LIGULA))]
    Ligula,
}

/// The units of `Velocity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Velocity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Velocity<N>: [const] Unit<Velocity<N>, N> + [const] Destruct)]
pub enum VelocityUnit {
    #[default]
    #[unit("m/s", "metres per second", by = Velocity::<N>::from(METRE_PER_SECOND))]
    MetrePerSecond,
    #[unit("ft/h", "feet per hour", by = Velocity::<N>::from(METRE_PER_SECOND * (Exact::dec(3048, -4) / Exact::int(3600))))]
    FootPerHour,
    #[unit("ft/min", "feet per minute", by = Velocity::<N>::from(METRE_PER_SECOND * Exact::dec(508, -5)))]
    FootPerMinute,
    #[unit("ft/s", "feet per second", by = Velocity::<N>::from(METRE_PER_SECOND * Exact::dec(3048, -4)))]
    FootPerSecond,
    #[unit("in/s", "inches per second", by = Velocity::<N>::from(METRE_PER_SECOND * Exact::dec(254, -4)))]
    InchPerSecond,
    #[unit("in/min", "inches per minute", by = Velocity::<N>::from(METRE_PER_SECOND * (Exact::dec(254, -4) / Exact::int(60))))]
    InchPerMinute,
    #[unit("kn", "knots", by = Velocity::<N>::from(KNOT))]
    Knot,
    #[unit("mi/h", "miles per hour", by = Velocity::<N>::from(MILE_PER_HOUR))]
    MilePerHour,
    #[unit("mi/min", "miles per minute", by = Velocity::<N>::from(METRE_PER_SECOND * Exact::dec(268224, -4)))]
    MilePerMinute,
    #[unit("mi/s", "miles per second", by = Velocity::<N>::from(METRE_PER_SECOND * Exact::dec(1609344, -3)))]
    MilePerSecond,
    #[unit("a₀ · Eₕ/ħ", "atomic units of velocity", by = Velocity::<N>::from(METRE_PER_SECOND * Exact::dec(218769126364, -5)))]
    AtomicUnitOfVelocity,
    #[unit("c", "natural units of velocity", by = Velocity::<N>::from(NATURAL_UNIT_OF_VELOCITY))]
    NaturalUnitOfVelocity,
    #[unit("c", "speeds of light in vacuum", by = Velocity::<N>::from(SPEED_OF_LIGHT))]
    SpeedOfLight,
    #[unit("km/h", "kilometres per hour", by = Velocity::<N>::from(METRE_PER_SECOND * (Exact::int(1000) / Exact::int(3600))))]
    KilometrePerHour,
    #[unit("mm/min", "millimetres per minute", by = Velocity::<N>::from(METRE_PER_SECOND * (Exact::dec(1, -3) / Exact::int(60))))]
    MillimetrePerMinute,
}

/// The units of `Acceleration`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Acceleration<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Acceleration<N>: [const] Unit<Acceleration<N>, N> + [const] Destruct)]
pub enum AccelerationUnit {
    #[default]
    #[unit("m/s²", "metres per second squared", by = Acceleration::<N>::from(METRE_PER_SECOND_SQUARED))]
    MetrePerSecondSquared,
    #[unit("ft/s²", "feet per second squared", by = Acceleration::<N>::from(METRE_PER_SECOND_SQUARED * Exact::dec(3048, -4)))]
    FootPerSecondSquared,
    #[unit("Gal", "galileos", by = Acceleration::<N>::from(GALILEO))]
    Galileo,
    #[unit("in/s²", "inches per second squared", by = Acceleration::<N>::from(METRE_PER_SECOND_SQUARED * Exact::dec(254, -4)))]
    InchPerSecondSquared,
    #[unit("g₀", "standard accelerations of gravity", by = Acceleration::<N>::from(STANDARD_GRAVITY))]
    StandardGravity,
    #[unit("mm/min²", "millimetres per minute squared", by = Acceleration::<N>::from(METRE_PER_SECOND_SQUARED * (Exact::dec(1, -3) / (Exact::int(60) * Exact::int(60)))))]
    MillimetrePerMinuteSquared,
}

/// The units of `Jerk`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Jerk<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Jerk<N>: [const] Unit<Jerk<N>, N> + [const] Destruct)]
pub enum JerkUnit {
    #[default]
    #[unit("m/s³", "metres per second cubed", by = Jerk::<N>::from(METRE / (SECOND * SECOND * SECOND)))]
    MetrePerSecondCubed,
    #[unit("ft/s³", "feet per second cubed", by = Jerk::<N>::from((METRE / (SECOND * SECOND * SECOND)) * Exact::dec(3048, -4)))]
    FootPerSecondCubed,
    #[unit("in/s³", "inches per second cubed", by = Jerk::<N>::from((METRE / (SECOND * SECOND * SECOND)) * Exact::dec(254, -4)))]
    InchPerSecondCubed,
    #[unit("km/min³", "kilometres per minute cubed", by = Jerk::<N>::from((METRE / (SECOND * SECOND * SECOND)) * (Exact::int(1000) / (Exact::int(60) * Exact::int(60) * Exact::int(60)))))]
    KilometrePerMinuteCubed,
}

/// The units of `Absement`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Absement<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Absement<N>: [const] Unit<Absement<N>, N> + [const] Destruct)]
pub enum AbsementUnit {
    #[default]
    #[unit("m · s", "metre seconds", by = Absement::<N>::from(METRE * SECOND))]
    MetreSecond,
    #[unit("ft · s", "foot seconds", by = Absement::<N>::from((METRE * SECOND) * Exact::dec(3048, -4)))]
    FootSecond,
    #[unit("in · s", "inch seconds", by = Absement::<N>::from((METRE * SECOND) * Exact::dec(254, -4)))]
    InchSecond,
    #[unit("km · h", "kilometre hours", by = Absement::<N>::from((METRE * SECOND) * Exact::int(3600000)))]
    KilometreHour,
}

/// The units of `InverseVelocity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = InverseVelocity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where InverseVelocity<N>: [const] Unit<InverseVelocity<N>, N> + [const] Destruct)]
pub enum InverseVelocityUnit {
    #[default]
    #[unit("s/m", "seconds per metre", by = InverseVelocity::<N>::from(SECOND / METRE))]
    SecondPerMetre,
    #[unit("h/ft", "hours per foot", by = InverseVelocity::<N>::from((SECOND / METRE) * (Exact::int(3600) / Exact::dec(3048, -4))))]
    HourPerFoot,
    #[unit("min/ft", "minutes per foot", by = InverseVelocity::<N>::from((SECOND / METRE) * (Exact::int(60) / Exact::dec(3048, -4))))]
    MinutePerFoot,
    #[unit("s/ft", "seconds per foot", by = InverseVelocity::<N>::from((SECOND / METRE) * (Exact::int(1) / Exact::dec(3048, -4))))]
    SecondPerFoot,
    #[unit("s/in", "seconds per inch", by = InverseVelocity::<N>::from((SECOND / METRE) * (Exact::int(1) / Exact::dec(254, -4))))]
    SecondPerInch,
    #[unit("h/mi", "hours per mile", by = InverseVelocity::<N>::from((SECOND / METRE) * (Exact::int(3600) / Exact::dec(1609344, -3))))]
    HourPerMile,
    #[unit("min/mi", "minutes per mile", by = InverseVelocity::<N>::from((SECOND / METRE) * (Exact::int(60) / Exact::dec(1609344, -3))))]
    MinutePerMile,
    #[unit("s/mi", "seconds per mile", by = InverseVelocity::<N>::from((SECOND / METRE) * (Exact::int(1) / Exact::dec(1609344, -3))))]
    SecondPerMile,
    #[unit("min/km", "minutes per kilometre", by = InverseVelocity::<N>::from((SECOND / METRE) * Exact::dec(6, -2)))]
    MinutePerKilometre,
    #[unit("h/km", "hours per kilometre", by = InverseVelocity::<N>::from((SECOND / METRE) * Exact::dec(36, -1)))]
    HourPerKilometre,
}

/// The units of `Force`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Force<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Force<N>: [const] Unit<Force<N>, N> + [const] Destruct)]
pub enum ForceUnit {
    #[default]
    #[unit("N", "newtons", by = Force::<N>::from(NEWTON))]
    Newton,
    #[unit("dyn", "dynes", by = Force::<N>::from(DYNE))]
    Dyne,
    #[unit("kgf", "kilograms-force", by = Force::<N>::from(KILOGRAM_FORCE))]
    KilogramForce,
    #[unit("kip", "kips", by = Force::<N>::from(KIP))]
    Kip,
    #[unit("ozf", "ounces-force", by = Force::<N>::from(OUNCE_FORCE))]
    OunceForce,
    #[unit("pdl", "poundals", by = Force::<N>::from(POUNDAL))]
    Poundal,
    #[unit("lbf", "pounds-force", by = Force::<N>::from(POUND_FORCE))]
    PoundForce,
    #[unit("2000 lbf", "tons-force", by = Force::<N>::from(TON_FORCE))]
    TonForce,
}

/// The units of `Momentum`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Momentum<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Momentum<N>: [const] Unit<Momentum<N>, N> + [const] Destruct)]
pub enum MomentumUnit {
    #[default]
    #[unit("kg · m/s", "kilogram metres per second", by = Momentum::<N>::from(METRE * KILOGRAM / SECOND))]
    KilogramMetrePerSecond,
    #[unit("g · m/s", "gram metres per second", by = Momentum::<N>::from((METRE * KILOGRAM / SECOND) * Exact::dec(1, -3)))]
    GramMetrePerSecond,
    #[unit("t · m/s", "ton metres per second", by = Momentum::<N>::from((METRE * KILOGRAM / SECOND) * Exact::int(1000)))]
    TonMetrePerSecond,
    #[unit("kg · m/min", "kilogram metres per minute", by = Momentum::<N>::from((METRE * KILOGRAM / SECOND) * (Exact::int(1) / Exact::int(60))))]
    KilogramMetrePerMinute,
    #[unit("kg · m/h", "kilogram metres per hour", by = Momentum::<N>::from((METRE * KILOGRAM / SECOND) * (Exact::int(1) / Exact::int(3600))))]
    KilogramMetrePerHour,
    #[unit("kg · m/d", "kilogram metres per day", by = Momentum::<N>::from((METRE * KILOGRAM / SECOND) * (Exact::int(1) / Exact::int(86400))))]
    KilogramMetrePerDay,
    #[unit("slug · ft/s", "slug feet per second", by = Momentum::<N>::from((METRE * KILOGRAM / SECOND) * ((Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4)) * Exact::dec(3048, -4))))]
    SlugFootPerSecond,
    #[unit("slug · in/s", "slug inches per second", by = Momentum::<N>::from((METRE * KILOGRAM / SECOND) * ((Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4)) * Exact::dec(254, -4))))]
    SlugInchPerSecond,
    #[unit("lb · ft/s", "pound feet per second", by = Momentum::<N>::from((METRE * KILOGRAM / SECOND) * Exact::dec(138254954376, -12)))]
    PoundFootPerSecond,
    #[unit("lb · in/s", "pound inches per second", by = Momentum::<N>::from((METRE * KILOGRAM / SECOND) * Exact::dec(11521246198, -12)))]
    PoundInchPerSecond,
}

/// The units of `Energy`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Energy<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Energy<N>: [const] Unit<Energy<N>, N> + [const] Destruct)]
pub enum EnergyUnit {
    #[default]
    #[unit("J", "joules", by = Energy::<N>::from(JOULE))]
    Joule,
    #[unit("W · h", "watt hours", by = Energy::<N>::from(JOULE * Exact::int(3600)))]
    WattHour,
    #[unit("eV", "electronvolts", by = Energy::<N>::from(ELECTRONVOLT))]
    Electronvolt,
    #[unit("Eₕ", "hartrees", by = Energy::<N>::from(HARTREE))]
    Hartree,
    #[unit("Btu (IT)", "British thermal units (IT)", by = Energy::<N>::from(BTU_IT))]
    BtuIt,
    #[unit("Btu", "British thermal units", by = Energy::<N>::from(BTU))]
    Btu,
    #[unit("cal (IT)", "calories (IT)", by = Energy::<N>::from(CALORIE_IT))]
    CalorieIt,
    #[unit("cal", "calories", by = Energy::<N>::from(CALORIE))]
    Calorie,
    #[unit("Cal (IT)", "Calories (IT)", by = Energy::<N>::from(CALORIE_IT_NUTRITION))]
    CalorieItNutrition,
    #[unit("Cal", "Calories", by = Energy::<N>::from(CALORIE_NUTRITION))]
    CalorieNutrition,
    #[unit("erg", "ergs", by = Energy::<N>::from(ERG))]
    Erg,
    #[unit("ft · pdl", "foot poundals", by = Energy::<N>::from(JOULE * (Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(3048, -4))))]
    FootPoundal,
    #[unit("ft · lbf", "foot pounds-force", by = Energy::<N>::from(JOULE * (Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5))))]
    FootPound,
    #[unit("10¹⁵ Btu (IT)", "quads", by = Energy::<N>::from(QUAD))]
    Quad,
    #[unit("thm (EC)", "therms (EC)", by = Energy::<N>::from(THERM_EC))]
    ThermEc,
    #[unit("thm", "therms", by = Energy::<N>::from(THERM_US))]
    ThermUs,
    #[unit("t of TNT", "tons of TNT", by = Energy::<N>::from(TON_TNT))]
    TonTnt,
    #[unit("W · s", "watt seconds", by = Energy::<N>::from(JOULE))]
    WattSecond,
}

/// The units of `AvailableEnergy`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AvailableEnergy<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where AvailableEnergy<N>: [const] Unit<AvailableEnergy<N>, N> + [const] Destruct)]
pub enum AvailableEnergyUnit {
    #[default]
    #[unit("J/kg", "joules per kilogram", by = AvailableEnergy::<N>::from(METRE * METRE / (SECOND * SECOND)))]
    JoulePerKilogram,
    #[unit("J/g", "joules per gram", by = AvailableEnergy::<N>::from((METRE * METRE / (SECOND * SECOND)) * Exact::int(1000)))]
    JoulePerGram,
    #[unit("Btu (IT)/lb", "British thermal units (IT) per pound", by = AvailableEnergy::<N>::from((METRE * METRE / (SECOND * SECOND)) * Exact::int(2326)))]
    BtuItPerPound,
    #[unit("Btu/lb", "British thermal units per pound", by = AvailableEnergy::<N>::from((METRE * METRE / (SECOND * SECOND)) * (Exact::int(4184) / Exact::dec(18, -1))))]
    BtuPerPound,
    #[unit("cal (IT)/g", "calories (IT) per gram", by = AvailableEnergy::<N>::from((METRE * METRE / (SECOND * SECOND)) * Exact::dec(41868, -1)))]
    CalorieItPerGram,
    #[unit("cal/g", "calories per gram", by = AvailableEnergy::<N>::from((METRE * METRE / (SECOND * SECOND)) * Exact::int(4184)))]
    CaloriePerGram,
}

/// The units of `Power`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Power<N>, number = N, non_const)]
#[unit(for<N: Num + const From<Exact>> where Power<N>: From<PowerLevel<N>>, Power<N>: Unit<Power<N>, N>, PowerLevel<N>: From<Power<N>>, Scale<PowerLevel<N>, Gain<N>>: Unit<PowerLevel<N>, N>)]
pub enum PowerUnit {
    #[default]
    #[unit("W", "watts", by = <Power<N> as From<Power<Exact>>>::from(WATT))]
    Watt,
    #[unit("dBW", "decibels-watt", by = <Scale<PowerLevel<N>, Gain<N>> as From<Scale<PowerLevel<Exact>, Gain<Exact>>>>::from(DBW), via = PowerLevel<N>)]
    DecibelWatt,
    #[unit("dBm", "decibels-milliwatt", by = <Scale<PowerLevel<N>, Gain<N>> as From<Scale<PowerLevel<Exact>, Gain<Exact>>>>::from(DBM), via = PowerLevel<N>)]
    DecibelMilliwatt,
    #[unit("erg/s", "ergs per second", by = <Power<N> as From<Power<Exact>>>::from(WATT * Exact::dec(1, -7)))]
    ErgPerSecond,
    #[unit("ft · lbf/h", "foot pounds-force per hour", by = <Power<N> as From<Power<Exact>>>::from(WATT * ((Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / Exact::int(3600))))]
    FootPoundPerHour,
    #[unit("ft · lbf/min", "foot pounds-force per minute", by = <Power<N> as From<Power<Exact>>>::from(WATT * ((Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / Exact::int(60))))]
    FootPoundPerMinute,
    #[unit("ft · lbf/s", "foot pounds-force per second", by = <Power<N> as From<Power<Exact>>>::from(WATT * (Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5))))]
    FootPoundPerSecond,
    #[unit("hp", "horsepower", by = <Power<N> as From<Power<Exact>>>::from(HORSEPOWER))]
    Horsepower,
    #[unit("hp (S)", "horsepower (boiler)", by = <Power<N> as From<Power<Exact>>>::from(HORSEPOWER_BOILER))]
    HorsepowerBoiler,
    #[unit("hp (E)", "horsepower (electric)", by = <Power<N> as From<Power<Exact>>>::from(HORSEPOWER_ELECTRIC))]
    HorsepowerElectric,
    #[unit("hp (M)", "metric horsepower", by = <Power<N> as From<Power<Exact>>>::from(HORSEPOWER_METRIC))]
    HorsepowerMetric,
    #[unit("hp (I)", "horsepower (Imperial)", by = <Power<N> as From<Power<Exact>>>::from(HORSEPOWER_IMPERIAL))]
    HorsepowerImperial,
    #[unit("hp (hydraulic)", "hydraulic horsepower", by = <Power<N> as From<Power<Exact>>>::from(HYDRAULIC_HORSEPOWER))]
    HydraulicHorsepower,
}

/// The units of `Gain`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Gain<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Gain<N>: [const] Unit<Gain<N>, N> + [const] Destruct)]
pub enum GainUnit {
    #[default]
    #[unit("B", "bels", by = Gain::<N>::from(BEL))]
    Bel,
    #[unit("dB", "decibels", by = Gain::<N>::from(DECIBEL))]
    Decibel,
    #[unit("Np", "nepers", by = Gain::<N>::from(NEPER))]
    Neper,
}

/// The units of `PowerLevel`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = PowerLevel<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Scale<PowerLevel<N>, Gain<N>>: [const] Unit<PowerLevel<N>, N> + [const] Destruct)]
pub enum PowerLevelUnit {
    #[default]
    #[unit("dBW", "decibels-watt", by = Scale::<PowerLevel<N>, Gain<N>>::from(DBW))]
    DecibelWatt,
    #[unit("dBm", "decibels-milliwatt", by = Scale::<PowerLevel<N>, Gain<N>>::from(DBM))]
    DecibelMilliwatt,
}

/// The units of `PowerRate`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = PowerRate<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where PowerRate<N>: [const] Unit<PowerRate<N>, N> + [const] Destruct)]
pub enum PowerRateUnit {
    #[default]
    #[unit("W/s", "watts per second", by = PowerRate::<N>::from(METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * SECOND)))]
    WattPerSecond,
    #[unit("erg/s²", "ergs per second squared", by = PowerRate::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * SECOND)) * Exact::dec(1, -7)))]
    ErgPerSecondSquared,
    #[unit("ft · lbf/s²", "foot pounds-force per second squared", by = PowerRate::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * SECOND)) * (Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5))))]
    FootPoundPerSecondSquared,
    #[unit("hp/s", "horsepower per second", by = PowerRate::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * SECOND)) * (Exact::int(550) * Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5))))]
    HorsepowerPerSecond,
    #[unit("hp/s (S)", "horsepower per second (boiler)", by = PowerRate::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * SECOND)) * Exact::dec(98095, -1)))]
    HorsepowerPerSecondBoiler,
    #[unit("hp/s (E)", "horsepower per second (electric)", by = PowerRate::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * SECOND)) * Exact::int(746)))]
    HorsepowerPerSecondElectric,
    #[unit("hp/s (M)", "metric horsepower per second", by = PowerRate::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * SECOND)) * (Exact::int(75) * Exact::dec(980665, -5))))]
    HorsepowerPerSecondMetric,
    #[unit("hp/s (I)", "horsepower per second (Imperial)", by = PowerRate::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * SECOND)) * Exact::dec(7457, -1)))]
    HorsepowerPerSecondImperial,
    #[unit("hp/s (hydraulic)", "hydraulic horsepower per second", by = PowerRate::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * SECOND)) * Exact::dec(746043, -3)))]
    HydraulicHorsepowerPerSecond,
}

/// The units of `Action`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Action<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Action<N>: [const] Unit<Action<N>, N> + [const] Destruct)]
pub enum ActionUnit {
    #[default]
    #[unit("J · s", "joule seconds", by = Action::<N>::from(METRE * METRE * KILOGRAM / SECOND))]
    JouleSecond,
    #[unit("ħ", "atomic units of action", by = Action::<N>::from(ATOMIC_UNIT_OF_ACTION))]
    AtomicUnitOfAction,
    #[unit("h", "Planck constants", by = Action::<N>::from(PLANCK_CONSTANT))]
    PlanckConstant,
    #[unit("erg · s", "erg seconds", by = Action::<N>::from((METRE * METRE * KILOGRAM / SECOND) * Exact::dec(1, -7)))]
    ErgSecond,
    #[unit("eV · s", "electronvolt seconds", by = Action::<N>::from((METRE * METRE * KILOGRAM / SECOND) * Exact::dec(1602176634, -28)))]
    ElectronvoltSecond,
}

/// The units of `Torque`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Torque<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Torque<N>: [const] Unit<Torque<N>, N> + [const] Destruct)]
pub enum TorqueUnit {
    #[default]
    #[unit("N · m", "newton metres", by = Torque::<N>::from(METRE * METRE * KILOGRAM / (SECOND * SECOND * RADIAN)))]
    NewtonMetre,
    #[unit("dyn · m", "dyne metres", by = Torque::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * RADIAN)) * Exact::dec(1, -5)))]
    DyneMetre,
    #[unit("kgf · m", "kilogram-force metres", by = Torque::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * RADIAN)) * Exact::dec(980665, -5)))]
    KilogramForceMetre,
    #[unit("ozf · in", "ounces-force inches", by = Torque::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * RADIAN)) * ((Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::int(16)) * Exact::dec(254, -4))))]
    OunceForceInch,
    #[unit("lbf · ft", "pounds-force feet", by = Torque::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * RADIAN)) * ((Exact::dec(45359237, -8) * Exact::dec(980665, -5)) * Exact::dec(3048, -4))))]
    PoundForceFoot,
    #[unit("lbf · in", "pounds-force inches", by = Torque::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * RADIAN)) * ((Exact::dec(45359237, -8) * Exact::dec(980665, -5)) * Exact::dec(254, -4))))]
    PoundForceInch,
}

/// The units of `MomentOfInertia`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MomentOfInertia<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MomentOfInertia<N>: [const] Unit<MomentOfInertia<N>, N> + [const] Destruct)]
pub enum MomentOfInertiaUnit {
    #[default]
    #[unit("kg · m²", "kilogram square metres", by = MomentOfInertia::<N>::from(METRE * METRE * KILOGRAM / (RADIAN * RADIAN)))]
    KilogramSquareMetre,
    #[unit("Da · Å²", "dalton square ångströms", by = MomentOfInertia::<N>::from((METRE * METRE * KILOGRAM / (RADIAN * RADIAN)) * Exact::dec(16605390666, -57)))]
    DaltonSquareAngstrom,
}

/// The units of `Pressure`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Pressure<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Pressure<N>: [const] Unit<Pressure<N>, N> + [const] Destruct)]
pub enum PressureUnit {
    #[default]
    #[unit("Pa", "pascals", by = Pressure::<N>::from(PASCAL))]
    Pascal,
    #[unit("atm", "standard atmospheres", by = Pressure::<N>::from(STANDARD_ATMOSPHERE))]
    StandardAtmosphere,
    #[unit("at", "atmospheres (technical)", by = Pressure::<N>::from(ATMOSPHERE_TECHNICAL))]
    AtmosphereTechnical,
    #[unit("bar", "bar", by = Pressure::<N>::from(BAR))]
    Bar,
    #[unit("cm Hg", "centimetres of mercury", by = Pressure::<N>::from(CENTIMETRE_OF_MERCURY))]
    CentimetreOfMercury,
    #[unit("cm H₂O", "centimetres of water", by = Pressure::<N>::from(CENTIMETRE_OF_WATER))]
    CentimetreOfWater,
    #[unit("ft Hg", "feet of mercury", by = Pressure::<N>::from(FOOT_OF_MERCURY))]
    FootOfMercury,
    #[unit("ft H₂O", "feet of water", by = Pressure::<N>::from(FOOT_OF_WATER))]
    FootOfWater,
    #[unit("in Hg", "inches of mercury", by = Pressure::<N>::from(INCH_OF_MERCURY))]
    InchOfMercury,
    #[unit("in H₂O", "inches of water", by = Pressure::<N>::from(INCH_OF_WATER))]
    InchOfWater,
    #[unit("kip/in²", "kips per square inch", by = Pressure::<N>::from(PASCAL * ((Exact::int(1000) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / (Exact::dec(254, -4) * Exact::dec(254, -4)))))]
    KipPerSquareInch,
    #[unit("mm Hg", "millimetres of mercury", by = Pressure::<N>::from(MILLIMETRE_OF_MERCURY))]
    MillimetreOfMercury,
    #[unit("mm H₂O", "millimetres of water", by = Pressure::<N>::from(MILLIMETRE_OF_WATER))]
    MillimetreOfWater,
    #[unit("pdl/ft²", "poundals per square foot", by = Pressure::<N>::from(PASCAL * ((Exact::dec(45359237, -8) * Exact::dec(3048, -4)) / (Exact::dec(3048, -4) * Exact::dec(3048, -4)))))]
    PoundalPerSquareFoot,
    #[unit("lbf/ft²", "pounds-force per square foot", by = Pressure::<N>::from(PASCAL * ((Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / (Exact::dec(3048, -4) * Exact::dec(3048, -4)))))]
    PoundForcePerSquareFoot,
    #[unit("lbf/in²", "pounds-force per square inch", by = Pressure::<N>::from(PASCAL * ((Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / (Exact::dec(254, -4) * Exact::dec(254, -4)))))]
    PoundForcePerSquareInch,
    #[unit("psi", "pounds-force per square inch", by = Pressure::<N>::from(PSI))]
    Psi,
    #[unit("Torr", "torr", by = Pressure::<N>::from(TORR))]
    Torr,
    #[unit("dyn/cm²", "dynes per square centimetre", by = Pressure::<N>::from(PASCAL * Exact::dec(1, -1)))]
    DynePerSquareCentimetre,
    #[unit("N/mm²", "newtons per square millimetre", by = Pressure::<N>::from(PASCAL * Exact::int(1000000)))]
    NewtonPerSquareMillimetre,
}

/// The units of `Frequency`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Frequency<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Frequency<N>: [const] Unit<Frequency<N>, N> + [const] Destruct)]
pub enum FrequencyUnit {
    #[default]
    #[unit("Hz", "hertz", by = Frequency::<N>::from(HERTZ))]
    Hertz,
    #[unit("1/d", "cycles per day", by = Frequency::<N>::from(HERTZ * (Exact::int(1) / Exact::int(86400))))]
    CyclePerDay,
    #[unit("1/h", "cycles per hour", by = Frequency::<N>::from(HERTZ * (Exact::int(1) / Exact::int(3600))))]
    CyclePerHour,
    #[unit("1/min", "cycles per minute", by = Frequency::<N>::from(HERTZ * (Exact::int(1) / Exact::int(60))))]
    CyclePerMinute,
    #[unit("100 MHz", "cycles per shake", by = Frequency::<N>::from(CYCLE_PER_SHAKE))]
    CyclePerShake,
    #[unit("1/a", "cycles per year", by = Frequency::<N>::from(HERTZ * (Exact::int(1) / Exact::int(31536000))))]
    CyclePerYear,
}

/// The units of `FrequencyDrift`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = FrequencyDrift<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where FrequencyDrift<N>: [const] Unit<FrequencyDrift<N>, N> + [const] Destruct)]
pub enum FrequencyDriftUnit {
    #[default]
    #[unit("Hz/s", "hertz per second", by = FrequencyDrift::<N>::from(ONE / (SECOND * SECOND)))]
    HertzPerSecond,
}

/// The units of `ReciprocalLength`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ReciprocalLength<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ReciprocalLength<N>: [const] Unit<ReciprocalLength<N>, N> + [const] Destruct)]
pub enum ReciprocalLengthUnit {
    #[default]
    #[unit("m⁻¹", "reciprocal metres", by = ReciprocalLength::<N>::from(RECIPROCAL_METRE))]
    ReciprocalMetre,
    #[unit("Å⁻¹", "reciprocal ångströms", by = ReciprocalLength::<N>::from(RECIPROCAL_ANGSTROM))]
    ReciprocalAngstrom,
    #[unit("dpt", "diopters", by = ReciprocalLength::<N>::from(DIOPTER))]
    Diopter,
    #[unit("R∞", "Rydberg constants", by = ReciprocalLength::<N>::from(RYDBERG_CONSTANT))]
    RydbergConstant,
}

/// The units of `Curvature`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Curvature<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Curvature<N>: [const] Unit<Curvature<N>, N> + [const] Destruct)]
pub enum CurvatureUnit {
    #[default]
    #[unit("rad/m", "radians per metre", by = Curvature::<N>::from(RADIAN / METRE))]
    RadianPerMetre,
    #[unit("°/m", "degrees per metre", by = Curvature::<N>::from((RADIAN / METRE) * (Exact::PI / Exact::int(180))))]
    DegreePerMetre,
}

/// The units of `SpecificArea`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificArea<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where SpecificArea<N>: [const] Unit<SpecificArea<N>, N> + [const] Destruct)]
pub enum SpecificAreaUnit {
    #[default]
    #[unit("m²/kg", "square metres per kilogram", by = SpecificArea::<N>::from(METRE * METRE / KILOGRAM))]
    SquareMetrePerKilogram,
    #[unit("m²/g", "square metres per gram", by = SpecificArea::<N>::from((METRE * METRE / KILOGRAM) * Exact::int(1000)))]
    SquareMetrePerGram,
}

/// The units of `SpecificVolume`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificVolume<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where SpecificVolume<N>: [const] Unit<SpecificVolume<N>, N> + [const] Destruct)]
pub enum SpecificVolumeUnit {
    #[default]
    #[unit("m³/kg", "cubic metres per kilogram", by = SpecificVolume::<N>::from(METRE * METRE * METRE / KILOGRAM))]
    CubicMetrePerKilogram,
    #[unit("m³/g", "cubic metres per gram", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * Exact::int(1000)))]
    CubicMetrePerGram,
    #[unit("m³/ct", "cubic metres per carat", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * Exact::int(5000)))]
    CubicMetrePerCarat,
    #[unit("m³/gr", "cubic metres per grain", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::int(1) / Exact::dec(6479891, -11))))]
    CubicMetrePerGrain,
    #[unit("m³/cwt long", "cubic metres per hundredweight (long)", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::int(1) / Exact::dec(5080234544, -8))))]
    CubicMetrePerHundredweightLong,
    #[unit("m³/cwt short", "cubic metres per hundredweight (short)", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::int(1) / Exact::dec(45359237, -6))))]
    CubicMetrePerHundredweightShort,
    #[unit("m³/oz", "cubic metres per ounce", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::int(1) / Exact::dec(28349523125, -12))))]
    CubicMetrePerOunce,
    #[unit("m³/oz t", "cubic metres per troy ounce", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::int(1) / Exact::dec(311034768, -10))))]
    CubicMetrePerOunceTroy,
    #[unit("m³/dwt", "cubic metres per pennyweight", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::int(1) / Exact::dec(155517384, -11))))]
    CubicMetrePerPennyweight,
    #[unit("m³/lb", "cubic metres per pound", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::int(1) / Exact::dec(45359237, -8))))]
    CubicMetrePerPound,
    #[unit("m³/lb t", "cubic metres per troy pound", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::int(1) / Exact::dec(3732417216, -10))))]
    CubicMetrePerPoundTroy,
    #[unit("m³/slug", "cubic metres per slug", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::int(1) / (Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4)))))]
    CubicMetrePerSlug,
    #[unit("m³/AT", "cubic metres per assay ton", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::int(1) / (Exact::dec(175, -3) / Exact::int(6)))))]
    CubicMetrePerTonAssay,
    #[unit("m³/2240 lb", "cubic metres per long ton", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::int(1) / Exact::dec(10160469088, -7))))]
    CubicMetrePerTonLong,
    #[unit("m³/2000 lb", "cubic metres per short ton", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::int(1) / Exact::dec(90718474, -5))))]
    CubicMetrePerTonShort,
    #[unit("m³/t", "cubic metres per ton", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * Exact::dec(1, -3)))]
    CubicMetrePerTon,
    #[unit("gal/gr", "gallons per grain", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::dec(3785411784, -12) / Exact::dec(6479891, -11))))]
    GallonPerGrain,
    #[unit("in³/oz", "cubic inches per ounce", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4) / Exact::dec(28349523125, -12))))]
    CubicInchPerOunce,
    #[unit("gal (UK)/oz", "Imperial gallons per ounce", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::dec(454609, -8) / Exact::dec(28349523125, -12))))]
    GallonImperialPerOunce,
    #[unit("gal/oz", "gallons per ounce", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::dec(3785411784, -12) / Exact::dec(28349523125, -12))))]
    GallonPerOunce,
    #[unit("ft³/lb", "cubic feet per pound", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * ((Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)) / Exact::dec(45359237, -8))))]
    CubicFootPerPound,
    #[unit("in³/lb", "cubic inches per pound", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * ((Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4)) / Exact::dec(45359237, -8))))]
    CubicInchPerPound,
    #[unit("yd³/lb", "cubic yards per pound", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * ((Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4)) / Exact::dec(45359237, -8))))]
    CubicYardPerPound,
    #[unit("gal (UK)/lb", "Imperial gallons per pound", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::dec(454609, -8) / Exact::dec(45359237, -8))))]
    GallonImperialPerPound,
    #[unit("gal/lb", "gallons per pound", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::dec(3785411784, -12) / Exact::dec(45359237, -8))))]
    GallonPerPound,
    #[unit("ft³/slug", "cubic feett per slug", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4) / (Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4)))))]
    CubicFootPerSlug,
    #[unit("yd³/2240 lb", "cubic yards per long ton", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * ((Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4)) / Exact::dec(10160469088, -7))))]
    CubicYardPerTonLong,
    #[unit("yd³/2000 lb", "cubic yards per short ton", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * ((Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4)) / Exact::dec(90718474, -5))))]
    CubicYardPerTonShort,
    #[unit("ft³/2240 lb", "cubic feet per long ton", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * ((Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)) / Exact::dec(10160469088, -7))))]
    CubicFootPerTonLong,
    #[unit("ft³/2000 lb", "cubic feet per short ton", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * ((Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)) / Exact::dec(90718474, -5))))]
    CubicFootPerTonShort,
}

/// The units of `SpecificPower`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificPower<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where SpecificPower<N>: [const] Unit<SpecificPower<N>, N> + [const] Destruct)]
pub enum SpecificPowerUnit {
    #[default]
    #[unit("W/kg", "watts per kilogram", by = SpecificPower::<N>::from(METRE * METRE / (SECOND * SECOND * SECOND)))]
    WattPerKilogram,
    #[unit("m² · s⁻³", "square metres per cubic second", by = SpecificPower::<N>::from(METRE * METRE / (SECOND * SECOND * SECOND)))]
    SquareMetrePerCubicSecond,
    #[unit("(erg/s)/kg", "ergs per second per kilogram", by = SpecificPower::<N>::from((METRE * METRE / (SECOND * SECOND * SECOND)) * Exact::dec(1, -7)))]
    ErgPerSecondPerKilogram,
    #[unit("(ft · lbf/h)/lb", "foot pounds-force per hour per pound", by = SpecificPower::<N>::from((METRE * METRE / (SECOND * SECOND * SECOND)) * ((Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / (Exact::int(3600) * Exact::dec(45359237, -8)))))]
    FootPoundPerHourPerPound,
    #[unit("(ft · lbf/min)/lb", "foot pounds-force per minute per pound", by = SpecificPower::<N>::from((METRE * METRE / (SECOND * SECOND * SECOND)) * ((Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / (Exact::int(60) * Exact::dec(45359237, -8)))))]
    FootPoundPerMinutePerPound,
    #[unit("(ft · lbf/s)/lb", "foot pounds-force per second per pound", by = SpecificPower::<N>::from((METRE * METRE / (SECOND * SECOND * SECOND)) * ((Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / Exact::dec(45359237, -8))))]
    FootPoundPerSecondPerPound,
    #[unit("hp/lb", "horsepower per pound", by = SpecificPower::<N>::from((METRE * METRE / (SECOND * SECOND * SECOND)) * ((Exact::int(550) * Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / Exact::dec(45359237, -8))))]
    HorsepowerPerPound,
    #[unit("(hp (S))/lb", "horsepower (boiler) per pound", by = SpecificPower::<N>::from((METRE * METRE / (SECOND * SECOND * SECOND)) * (Exact::dec(98095, -1) / Exact::dec(45359237, -8))))]
    HorsepowerBoilerPerPound,
    #[unit("(hp (E))/lb", "horsepower (electric) per pound", by = SpecificPower::<N>::from((METRE * METRE / (SECOND * SECOND * SECOND)) * (Exact::int(746) / Exact::dec(45359237, -8))))]
    HorsepowerElectricPerPound,
    #[unit("(hp (M))/lb", "metric horsepower per pound", by = SpecificPower::<N>::from((METRE * METRE / (SECOND * SECOND * SECOND)) * ((Exact::int(75) * Exact::dec(980665, -5)) / Exact::dec(45359237, -8))))]
    HorsepowerMetricPerPound,
    #[unit("(hp (I))/lb", "horsepower (Imperial) per pound", by = SpecificPower::<N>::from((METRE * METRE / (SECOND * SECOND * SECOND)) * (Exact::dec(7457, -1) / Exact::dec(45359237, -8))))]
    HorsepowerImperialPerPound,
    #[unit("(hp (hydraulic))/lb", "hydraulic horsepower per pound", by = SpecificPower::<N>::from((METRE * METRE / (SECOND * SECOND * SECOND)) * (Exact::dec(746043, -3) / Exact::dec(45359237, -8))))]
    HydraulicHorsepowerPerPound,
}

/// The units of `SpecificGravitationalParameter`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificGravitationalParameter<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where SpecificGravitationalParameter<N>: [const] Unit<SpecificGravitationalParameter<N>, N> + [const] Destruct)]
pub enum SpecificGravitationalParameterUnit {
    #[default]
    #[unit("m³/(kg · s²)", "cubic metres per kilogram second squared", by = SpecificGravitationalParameter::<N>::from(METRE * METRE * METRE / (KILOGRAM * SECOND * SECOND)))]
    CubicMetrePerKilogramSecondSquared,
    #[unit("G", "Newtonian constants of gravitation", by = SpecificGravitationalParameter::<N>::from(GRAVITATIONAL_CONSTANT))]
    GravitationalConstant,
}
