use crate::{ISQ, StaticSIQuantity};
use yadc::{Dimension, Unit};

pub type Length = StaticSIQuantity<{ ISQ { m: 1, ..ISQ::ZERO } }>;


#[derive(Copy, Clone, Unit)]
#[dim(Length)]
pub enum LengthUnit {
    #[unit(9.9999999999999998e+23, "Ym", "yottameters")]
    Yottameter,
    #[unit(1e+21, "Zm", "zettameters")]
    Zettameter,
    #[unit(1e+18, "Em", "exameters")]
    Exameter,
    #[unit(1000000000000000.0, "Pm", "petameters")]
    Petameter,
    #[unit(1000000000000.0, "Tm", "terameters")]
    Terameter,
    #[unit(1000000000.0, "Gm", "gigameters")]
    Gigameter,
    #[unit(1000000.0, "Mm", "megameters")]
    Megameter,
    #[unit(1000.0, "km", "kilometers")]
    Kilometer,
    #[unit(100.0, "hm", "hectometers")]
    Hectometer,
    #[unit(10.0, "dam", "decameters")]
    Decameter,
    #[unit(1.0, "m", "meters")]
    Meter,
    #[unit(0.10000000000000001, "dm", "decimeters")]
    Decimeter,
    #[unit(0.01, "cm", "centimeters")]
    Centimeter,
    #[unit(0.001, "mm", "millimeters")]
    Millimeter,
    #[unit(9.9999999999999995e-07, "µm", "micrometers")]
    Micrometer,
    #[unit(1.0000000000000001e-09, "nm", "nanometers")]
    Nanometer,
    #[unit(9.9999999999999998e-13, "pm", "picometers")]
    Picometer,
    #[unit(1.0000000000000001e-15, "fm", "femtometers")]
    Femtometer,
    #[unit(1.0000000000000001e-18, "am", "attometers")]
    Attometer,
    #[unit(9.9999999999999991e-22, "zm", "zeptometers")]
    Zeptometer,
    #[unit(9.9999999999999992e-25, "ym", "yoctometers")]
    Yoctometer,
    #[unit(1e-10, "Å", "ångströms")]
    Angstrom,
    #[unit(5.2917721090299998e-11, "a₀", "bohr radiuses")]
    BohrRadius,
    #[unit(5.2917721090299998e-11, "a.u. of length", "atomic units of length")]
    AtomicUnitOfLength,
    #[unit(149597870700.0, "ua", "astronomical units")]
    AstronomicalUnit,
    #[unit(20.11684, "ch", "chains")]
    Chain,
    #[unit(1828.8, "DM", "data miles")]
    DataMile,
    #[unit(1.8288040000000001, "fathom", "fathoms")]
    Fathom,
    #[unit(1.0000000000000001e-15, "fermi", "fermis")]
    Fermi,
    #[unit(0.30480000000000002, "ft", "feet")]
    Foot,
    #[unit(0.30480059999999998, "ft (U.S. survey)", "feet (U.S. survey)")]
    FootSurvey,
    #[unit(0.025399999999999999, "in", "inches")]
    Inch,
    #[unit(9460730000000000.0, "l. y.", "light years")]
    LightYear,
    #[unit(2.5399999999999999e-08, "μin", "microinches")]
    Microinch,
    #[unit(9.9999999999999995e-07, "μ", "microns")]
    Micron,
    #[unit(2.5400000000000001e-05, "0.001 in", "mils")]
    Mil,
    #[unit(1609.3440000000001, "mi", "miles")]
    Mile,
    #[unit(1609.347, "mi (U.S. survey)", "miles (U.S. survey)")]
    MileSurvey,
    #[unit(1852.0, "M", "nautical miles")]
    NauticalMile,
    #[unit(30856780000000000.0, "pc", "parsecs")]
    Parsec,
    #[unit(0.0042333333333333329, "1/6 in (computer)", "picas (computer)")]
    PicaComputer,
    #[unit(0.004217518, "1/6 in", "picas (printer's)")]
    PicaPrinters,
    #[unit(0.00035277779999999998, "1/72 in (computer)", "points (computer)")]
    PointComputer,
    #[unit(0.00035145979999999999, "1/72 in", "points (printer's)")]
    PointPrinters,
    #[unit(5.02921, "rd", "rods")]
    Rod,
    #[unit(0.91439999999999999, "yd", "yards")]
    Yard,
    #[unit(2220.0, "leuga", "leugae")]
    Leuga,
    #[unit(1480.0, "mille passus", "mille passa")]
    MillePassus,
    #[unit(185.0, "stadium", "stadia")]
    Stadium,
    #[unit(35.520000000000003, "actus", "acta")]
    Actus,
    #[unit(2.96, "decempeda", "decempedae")]
    Decempeda,
    #[unit(1.48, "passus", "passa")]
    Passus,
    #[unit(0.73999999999999999, "gradus", "gradus")]
    Gradus,
    #[unit(0.44400000000000001, "cubitum", "cubita")]
    Cubitum,
    #[unit(0.37, "palmipes", "palmipedes")]
    Palmipes,
    #[unit(0.29599999999999999, "pes", "pedes")]
    Pes,
    #[unit(0.222, "palmus maior", "palmas maior")]
    PalmusMaior,
    #[unit(0.073999999999999996, "palmus", "palmas")]
    Palmus,
    #[unit(0.024666666666666663, "uncia", "unciae")]
    Uncia,
    #[unit(0.018499999999999999, "digitus", "digiti")]
    Digitus,
}

pub type Mass = StaticSIQuantity<{ ISQ { kg: 1, ..ISQ::ZERO } }>;

#[derive(Copy, Clone, Unit)]
#[dim(Mass)]
pub enum MassUnit {
    #[unit(1e+21, "Yg", "yottagrams")]
    Yottagram,
    #[unit(1e+18, "Zg", "zettagrams")]
    Zettagram,
    #[unit(1000000000000000.0, "Eg", "exagrams")]
    Exagram,
    #[unit(1000000000000.0, "Pg", "petagrams")]
    Petagram,
    #[unit(1000000000.0, "Tg", "teragrams")]
    Teragram,
    #[unit(1000000.0, "Gg", "gigagrams")]
    Gigagram,
    #[unit(1000.0, "Mg", "megagrams")]
    Megagram,
    #[unit(1.0, "kg", "kilograms")]
    Kilogram,
    #[unit(0.10000000000000001, "hg", "hectograms")]
    Hectogram,
    #[unit(0.01, "dag", "decagrams")]
    Decagram,
    #[unit(0.001, "g", "grams")]
    Gram,
    #[unit(0.0001, "dg", "decigrams")]
    Decigram,
    #[unit(1.0000000000000001e-05, "cg", "centigrams")]
    Centigram,
    #[unit(9.9999999999999995e-07, "mg", "milligrams")]
    Milligram,
    #[unit(9.9999999999999986e-10, "µg", "micrograms")]
    Microgram,
    #[unit(9.9999999999999998e-13, "ng", "nanograms")]
    Nanogram,
    #[unit(1.0000000000000001e-15, "pg", "picograms")]
    Picogram,
    #[unit(1.0000000000000001e-18, "fg", "femtograms")]
    Femtogram,
    #[unit(1.0000000000000001e-21, "ag", "attograms")]
    Attogram,
    #[unit(9.9999999999999992e-25, "zg", "zeptograms")]
    Zeptogram,
    #[unit(9.9999999999999986e-28, "yg", "yoctograms")]
    Yoctogram,
    #[unit(0.00020000000000000001, "ct", "carats")]
    Carat,
    #[unit(1.6605390666e-27, "Da", "daltons")]
    Dalton,
    #[unit(6.4798909999999995e-05, "gr", "grains")]
    Grain,
    #[unit(50.802349999999997, "cwt long", "hundredweight (long)")]
    HundredweightLong,
    #[unit(45.35924, "cwt short", "hundredweight (short)")]
    HundredweightShort,
    #[unit(0.02834952, "oz", "ounces")]
    Ounce,
    #[unit(0.031103479999999999, "oz t", "troy ounces")]
    OunceTroy,
    #[unit(0.0015551740000000001, "dwt", "pennyweight")]
    Pennyweight,
    #[unit(0.45359240000000001, "lb", "pounds")]
    Pound,
    #[unit(0.37324170000000001, "lb t", "troy pounds")]
    PoundTroy,
    #[unit(14.5939, "slug", "slugs")]
    Slug,
    #[unit(0.029166669999999999, "AT", "assay tons")]
    TonAssay,
    #[unit(1016.047, "2240 lb", "long tons")]
    TonLong,
    #[unit(907.18470000000002, "2000 lb", "short tons")]
    TonShort,
    #[unit(1000.0, "t", "tons")]
    Ton,
    #[unit(0.32890000000000003, "libra", "librae")]
    Libra,
    #[unit(0.30149166666666666, "deunx", "deunces")]
    Deunx,
    #[unit(0.27408333333333335, "dextans", "dextantes")]
    Dextans,
    #[unit(0.24667500000000001, "dodrans", "dodrantes")]
    Dodrans,
    #[unit(0.21926666666666667, "bes", "bessis")]
    Bes,
    #[unit(0.19185833333333335, "septunx", "septunces")]
    Septunx,
    #[unit(0.16445000000000001, "semis", "semisses")]
    Semis,
    #[unit(0.13704166666666667, "quincunx", "quincunx")]
    Quincunx,
    #[unit(0.10963333333333333, "triens", "trientes")]
    Triens,
    #[unit(0.082225000000000006, "quadrans teruncius", "quadrantes teruncius")]
    QuadransTeruncius,
    #[unit(0.054816666666666701, "sextans", "sextantes")]
    Sextans,
    #[unit(0.041112500000000003, "sescuncia", "sescunciae")]
    Sescuncia,
    #[unit(0.027408333333333337, "uncia", "unciae")]
    Uncia,
    #[unit(0.0137041666666667, "semuncia", "semunciae")]
    Semuncia,
    #[unit(0.0091361111111111105, "duella", "duella")]
    Duella,
    #[unit(0.0068520833333333298, "sicilicus", "scilici")]
    Sicilicus,
    #[unit(0.0045680555555555596, "sextula", "sextulae")]
    Sextula,
    #[unit(0.0022840277777777798, "semisextula", "semisextulae")]
    Semisextula,
    #[unit(0.0011420138888888899, "scrupulum", "scrupula")]
    Scrupulum,
    #[unit(0.00057100694444444397, "obolus", "oboli")]
    Obolus,
    #[unit(0.000190335648148148, "siliqua", "siliquae")]
    Siliqua,
}

pub type Time = StaticSIQuantity<{ ISQ { s: 1, ..ISQ::ZERO } }>;

#[derive(Copy, Clone, Unit)]
#[dim(Time)]
pub enum TimeUnit {
    #[unit(9.9999999999999998e+23, "Ys", "yottaseconds")]
    Yottasecond,
    #[unit(1e+21, "Zs", "zettaseconds")]
    Zettasecond,
    #[unit(1e+18, "Es", "exaseconds")]
    Exasecond,
    #[unit(1000000000000000.0, "Ps", "petaseconds")]
    Petasecond,
    #[unit(1000000000000.0, "Ts", "teraseconds")]
    Terasecond,
    #[unit(1000000000.0, "Gs", "gigaseconds")]
    Gigasecond,
    #[unit(1000000.0, "Ms", "megaseconds")]
    Megasecond,
    #[unit(1000.0, "ks", "kiloseconds")]
    Kilosecond,
    #[unit(100.0, "hs", "hectoseconds")]
    Hectosecond,
    #[unit(10.0, "das", "decaseconds")]
    Decasecond,
    #[unit(1.0, "s", "seconds")]
    Second,
    #[unit(0.10000000000000001, "ds", "deciseconds")]
    Decisecond,
    #[unit(0.01, "cs", "centiseconds")]
    Centisecond,
    #[unit(0.001, "ms", "milliseconds")]
    Millisecond,
    #[unit(9.9999999999999995e-07, "µs", "microseconds")]
    Microsecond,
    #[unit(1.0000000000000001e-09, "ns", "nanoseconds")]
    Nanosecond,
    #[unit(9.9999999999999998e-13, "ps", "picoseconds")]
    Picosecond,
    #[unit(1.0000000000000001e-15, "fs", "femtoseconds")]
    Femtosecond,
    #[unit(1.0000000000000001e-18, "as", "attoseconds")]
    Attosecond,
    #[unit(9.9999999999999991e-22, "zs", "zeptoseconds")]
    Zeptosecond,
    #[unit(9.9999999999999992e-25, "ys", "yoctoseconds")]
    Yoctosecond,
    #[unit(0.99726959999999998, "s (sidereal)", "seconds (sidereal)")]
    SecondSidereal,
    #[unit(86400.0, "d", "days")]
    Day,
    #[unit(86164.089999999997, "d (sidereal)", "days (sidereal)")]
    DaySidereal,
    #[unit(3600.0, "h", "hours")]
    Hour,
    #[unit(3590.1700000000001, "h (sidereal)", "hours (sidereal)")]
    HourSidereal,
    #[unit(60.0, "min", "minutes")]
    Minute,
    #[unit(1e-08, "10.0 ns", "shakes")]
    Shake,
    #[unit(31536000.0, "a", "years")]
    Year,
    #[unit(31558150.0, "a (sidereal)", "years (sidereal)")]
    YearSidereal,
    #[unit(31556930.0, "a (tropical)", "years (tropical)")]
    YearTropical,
}

pub type Area = StaticSIQuantity<{ ISQ { m: 2, ..ISQ::ZERO } }>;

#[derive(Copy, Clone, Unit)]
#[dim(Area)]
pub enum AreaUnit {
    #[unit(1e+48, "Ym²", "square yottameters")]
    SquareYottameter,
    #[unit(1e+42, "Zm²", "square zettameters")]
    SquareZettameter,
    #[unit(1e+36, "Em²", "square exameters")]
    SquareExameter,
    #[unit(1e+30, "Pm²", "square petameters")]
    SquarePetameter,
    #[unit(9.9999999999999998e+23, "Tm²", "square terameters")]
    SquareTerameter,
    #[unit(1e+18, "Gm²", "square gigameters")]
    SquareGigameter,
    #[unit(1000000000000.0, "Mm²", "square megameters")]
    SquareMegameter,
    #[unit(1000000.0, "km²", "square kilometers")]
    SquareKilometer,
    #[unit(10000.0, "hm²", "square hectometers")]
    SquareHectometer,
    #[unit(100.0, "dam²", "square decameters")]
    SquareDecameter,
    #[unit(1.0, "m²", "square meters")]
    SquareMeter,
    #[unit(0.010000000000000002, "dm²", "square decimeters")]
    SquareDecimeter,
    #[unit(0.0001, "cm²", "square centimeters")]
    SquareCentimeter,
    #[unit(9.9999999999999995e-07, "mm²", "square millimeters")]
    SquareMillimeter,
    #[unit(9.9999999999999998e-13, "µm²", "square micrometers")]
    SquareMicrometer,
    #[unit(1.0000000000000001e-18, "nm²", "square nanometers")]
    SquareNanometer,
    #[unit(9.9999999999999992e-25, "pm²", "square picometers")]
    SquarePicometer,
    #[unit(1.0000000000000001e-30, "fm²", "square femtometers")]
    SquareFemtometer,
    #[unit(1.0000000000000001e-36, "am²", "square attometers")]
    SquareAttometer,
    #[unit(9.9999999999999988e-43, "zm²", "square zeptometers")]
    SquareZeptometer,
    #[unit(9.9999999999999982e-49, "ym²", "square yoctometers")]
    SquareYoctometer,
    #[unit(4046.873, "ac", "acres")]
    Acre,
    #[unit(100.0, "a", "ares")]
    Are,
    #[unit(9.9999999999999997e-29, "b", "barns")]
    Barn,
    #[unit(5.0670750000000003e-10, "cmil", "circular mils")]
    CircularMil,
    #[unit(10000.0, "ha", "hectares")]
    Hectare,
    #[unit(0.092903040000000006, "ft²", "square feet")]
    SquareFoot,
    #[unit(0.00064515999999999998, "in²", "square inches")]
    SquareInch,
    #[unit(2589988.1103360001, "mi²", "square miles")]
    SquareMile,
    #[unit(0.83612735999999999, "yd²", "square yards")]
    SquareYard,
    #[unit(2018672.6399999999, "saltus", "saltus")]
    Saltus,
    #[unit(504668.15999999997, "centuria", "centuriae")]
    Centuria,
    #[unit(5046.6815999999999, "heredium", "heredia")]
    Heredium,
    #[unit(2523.3407999999999, "jugerum", "jugera")]
    Jugerum,
    #[unit(2313.0623999999998, "deunx", "deunces")]
    Deunx,
    #[unit(2102.7840000000001, "dextans", "dextantes")]
    Dextans,
    #[unit(1892.5056, "dodrans", "dodrantes")]
    Dodrans,
    #[unit(1682.2272, "bes", "besses")]
    Bes,
    #[unit(1471.9487999999999, "septunx", "septunces")]
    Septunx,
    #[unit(1261.6704, "semis", "semisses")]
    Semis,
    #[unit(1261.6704, "actus quadratus", "acta quadratus")]
    ActusQuadratus,
    #[unit(1051.3920000000001, "quincunx", "quincunces")]
    Quincunx,
    #[unit(841.11360000000002, "triens", "trientes")]
    Triens,
    #[unit(630.83519999999999, "quadrans", "quadrantes")]
    Quadrans,
    #[unit(420.55680000000001, "sextans", "sextantes")]
    Sextans,
    #[unit(315.41759999999999, "clima", "climata")]
    Clima,
    #[unit(210.2784, "uncia", "unciae")]
    Uncia,
    #[unit(105.1392, "semiuncia", "semiunciae")]
    Semiuncia,
    #[unit(52.569600000000001, "sicilicus", "siculi")]
    Sicilicus,
    #[unit(42.055680000000002, "actus simplex", "acta simplex")]
    ActusSimplex,
    #[unit(35.046399999999998, "sextula", "sextulae")]
    Sextula,
    #[unit(17.523199999999999, "duo scrupulum", "duo scrupula")]
    DuoScrupulum,
    #[unit(8.7615999999999996, "scrupulum", "scrupula")]
    Scrupulum,
    #[unit(4.3807999999999998, "dimidium scrupulum", "dimidium scupula")]
    DimidiumScrupulum,
    #[unit(0.087615999999999999, "pes quadratus", "pedes quadratus")]
    PesQuadratus,
}

pub type Volume = StaticSIQuantity<{ ISQ { m: 3, ..ISQ::ZERO } }>;

#[derive(Copy, Clone, Unit)]
#[dim(Volume)]
pub enum VolumeUnit {
    #[unit(1.0, "m³", "cubic meters")]
    CubicMeter,
    #[unit(1233.489, "ac · ft", "acre-feet")]
    AcreFoot,
    #[unit(0.1589873, "bbl", "barrels")]
    Barrel,
    #[unit(0.035239069999999997, "bu", "bushels")]
    Bushel,
    #[unit(3.6245560000000001, "cords", "cords")]
    Cord,
    #[unit(0.028316850000000001, "ft³", "cubic feet")]
    CubicFoot,
    #[unit(1.6387060000000002e-05, "in³", "cubic inches")]
    CubicInch,
    #[unit(4168182000.0, "mi³", "cubic miles")]
    CubicMile,
    #[unit(0.76455490000000004, "yd³", "cubic yards")]
    CubicYard,
    #[unit(0.00023658819999999999, "cup", "cups")]
    Cup,
    #[unit(2.9573529999999999e-05, "fl oz", "fluid ounces")]
    FluidOunce,
    #[unit(2.8413060000000001e-05, "fl oz (UK)", "Imperial fluid ounces")]
    FluidOunceImperial,
    #[unit(0.00454609, "gal (UK)", "Imperial gallons")]
    GallonImperial,
    #[unit(0.0037854120000000002, "gal", "gallons")]
    Gallon,
    #[unit(0.0001420653, "gi (UK)", "Imperial gills")]
    GillImperial,
    #[unit(0.0001182941, "gi", "gills")]
    Gill,
    #[unit(1e+21, "YL", "yottaliters")]
    Yottaliter,
    #[unit(1e+18, "ZL", "zettaliters")]
    Zettaliter,
    #[unit(1000000000000000.0, "EL", "exaliters")]
    Exaliter,
    #[unit(1000000000000.0, "PL", "petaliters")]
    Petaliter,
    #[unit(1000000000.0, "TL", "teraliters")]
    Teraliter,
    #[unit(1000000.0, "GL", "gigaliters")]
    Gigaliter,
    #[unit(1000.0, "ML", "megaliters")]
    Megaliter,
    #[unit(1.0, "kL", "kiloliters")]
    Kiloliter,
    #[unit(0.10000000000000001, "hL", "hectoliters")]
    Hectoliter,
    #[unit(0.01, "daL", "decaliters")]
    Decaliter,
    #[unit(0.001, "L", "liters")]
    Liter,
    #[unit(0.0001, "dL", "deciliters")]
    Deciliter,
    #[unit(1.0000000000000001e-05, "cL", "centiliters")]
    Centiliter,
    #[unit(9.9999999999999995e-07, "mL", "milliliters")]
    Milliliter,
    #[unit(1.0000000000000001e-09, "µL", "microliters")]
    Microliter,
    #[unit(1.0000000000000002e-12, "nL", "nanoliters")]
    Nanoliter,
    #[unit(1.0000000000000001e-15, "pL", "picoliters")]
    Picoliter,
    #[unit(1.0000000000000001e-18, "fL", "femtoliters")]
    Femtoliter,
    #[unit(1.0000000000000001e-21, "aL", "attoliters")]
    Attoliter,
    #[unit(9.9999999999999992e-25, "zL", "zeptoliters")]
    Zeptoliter,
    #[unit(9.9999999999999986e-28, "yL", "yoctoliters")]
    Yoctoliter,
    #[unit(0.0088097680000000008, "pk", "pecks")]
    Peck,
    #[unit(0.00055061050000000005, "dry pt", "dry pints")]
    PintDry,
    #[unit(0.00047317650000000002, "liq pt", "liquid pints")]
    PintLiquid,
    #[unit(0.0011012210000000001, "dry qt", "dry quarts")]
    QuartDry,
    #[unit(0.0009463529, "liq qt", "liquid quarts")]
    QuartLiquid,
    #[unit(1.0, "st", "steres")]
    Stere,
    #[unit(1.4786759999999999e-05, "tbsp", "tablespoons")]
    Tablespoon,
    #[unit(4.9289219999999997e-06, "tsp", "teaspoons")]
    Teaspoon,
    #[unit(2.8316849999999998, "RT", "register tons")]
    RegisterTon,
    #[unit(0.51868672000000005, "culeus", "culei")]
    Culeus,
    #[unit(0.025934335999999999, "amphora quadrantal", "amphora quadrantal")]
    AmphoraQuadrantal,
    #[unit(0.012967167999999999, "urna", "urnae")]
    Urna,
    #[unit(0.012967167999999999, "modius castrensis", "modii castrensis")]
    ModiusCastrensis,
    #[unit(0.0086447786666666599, "modius", "modii")]
    Modius,
    #[unit(0.00432238933333333, "semimodius", "semimodii")]
    Semimodius,
    #[unit(0.0032417919999999999, "congius", "congii")]
    Congius,
    #[unit(0.000540298666666667, "sextarius", "sextarii")]
    Sextarius,
    #[unit(0.00027014933333333301, "hemina", "heminae")]
    Hemina,
    #[unit(0.00013507466666666699, "quartarius", "quartarii")]
    Quartarius,
    #[unit(6.7537333333333294e-05, "acetabulum", "acetabula")]
    Acetabulum,
    #[unit(4.5024888888888901e-05, "cyathus", "cyathi")]
    Cyathus,
    #[unit(1.1256222222222222e-05, "ligula", "ligula")]
    Ligula,
}

pub type Velocity = StaticSIQuantity<
    {
        ISQ {
            m: 1,
            s: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(Velocity)]
pub enum VelocityUnit {
    #[unit(9.9999999999999998e+23, "Ym/s", "yottameters per second")]
    YottameterPerSecond,
    #[unit(1e+21, "Zm/s", "zettameters per second")]
    ZettameterPerSecond,
    #[unit(1e+18, "Em/s", "exameters per second")]
    ExameterPerSecond,
    #[unit(1000000000000000.0, "Pm/s", "petameters per second")]
    PetameterPerSecond,
    #[unit(1000000000000.0, "Tm/s", "terameters per second")]
    TerameterPerSecond,
    #[unit(1000000000.0, "Gm/s", "gigameters per second")]
    GigameterPerSecond,
    #[unit(1000000.0, "Mm/s", "megameters per second")]
    MegameterPerSecond,
    #[unit(1000.0, "km/s", "kilometers per second")]
    KilometerPerSecond,
    #[unit(100.0, "hm/s", "hectometers per second")]
    HectometerPerSecond,
    #[unit(10.0, "dam/s", "decameters per second")]
    DecameterPerSecond,
    #[unit(1.0, "m/s", "meters per second")]
    MeterPerSecond,
    #[unit(0.10000000000000001, "dm/s", "decimeters per second")]
    DecimeterPerSecond,
    #[unit(0.01, "cm/s", "centimeters per second")]
    CentimeterPerSecond,
    #[unit(0.001, "mm/s", "millimeters per second")]
    MillimeterPerSecond,
    #[unit(9.9999999999999995e-07, "µm/s", "micrometers per second")]
    MicrometerPerSecond,
    #[unit(1.0000000000000001e-09, "nm/s", "nanometers per second")]
    NanometerPerSecond,
    #[unit(9.9999999999999998e-13, "pm/s", "picometers per second")]
    PicometerPerSecond,
    #[unit(1.0000000000000001e-15, "fm/s", "femtometers per second")]
    FemtometerPerSecond,
    #[unit(1.0000000000000001e-18, "am/s", "attometers per second")]
    AttometerPerSecond,
    #[unit(9.9999999999999991e-22, "zm/s", "zeptometers per second")]
    ZeptometerPerSecond,
    #[unit(9.9999999999999992e-25, "ym/s", "yoctometers per second")]
    YoctometerPerSecond,
    #[unit(8.4666666666666674e-05, "ft/h", "feet per hour")]
    FootPerHour,
    #[unit(0.0050800000000000003, "ft/min", "feet per minute")]
    FootPerMinute,
    #[unit(0.30480000000000002, "ft/s", "feet per second")]
    FootPerSecond,
    #[unit(0.025399999999999999, "in/s", "inches per second")]
    InchPerSecond,
    #[unit(0.00042333333333333334, "in/min", "inches per minute")]
    InchPerMinute,
    #[unit(0.27777777777777779, "km/h", "kilometers per hour")]
    KilometerPerHour,
    #[unit(0.51444444444444448, "kn", "knots")]
    Knot,
    #[unit(0.44703999999999999, "mi/h", "miles per hour")]
    MilePerHour,
    #[unit(26.822399999999998, "mi/min", "miles per minute")]
    MilePerMinute,
    #[unit(1609.3440000000001, "mi/s", "miles per second")]
    MilePerSecond,
    #[unit(1.6666666666666667e-05, "mm/min", "millimeters per minute")]
    MillimeterPerMinute,
    #[unit(2187691.26364, "a₀ · Eₕ/ħ", "atomic units of velocity")]
    AtomicUnitOfVelocity,
    #[unit(299792458.0, "c", "natural units of velocity")]
    NaturalUnitOfVelocity,
    #[unit(299792458.0, "c", "speeds of light in vacuum")]
    SpeedOfLightInVacuum,
}

pub type Acceleration = StaticSIQuantity<
    {
        ISQ {
            m: 1,
            s: -2,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(Acceleration)]
pub enum AccelerationUnit {
    #[unit(9.9999999999999998e+23, "Ym/s²", "yottameters per second squared")]
    YottameterPerSecondSquared,
    #[unit(1e+21, "Zm/s²", "zettameters per second squared")]
    ZettameterPerSecondSquared,
    #[unit(1e+18, "Em/s²", "exameters per second squared")]
    ExameterPerSecondSquared,
    #[unit(1000000000000000.0, "Pm/s²", "petameters per second squared")]
    PetameterPerSecondSquared,
    #[unit(1000000000000.0, "Tm/s²", "terameters per second squared")]
    TerameterPerSecondSquared,
    #[unit(1000000000.0, "Gm/s²", "gigameters per second squared")]
    GigameterPerSecondSquared,
    #[unit(1000000.0, "Mm/s²", "megameters per second squared")]
    MegameterPerSecondSquared,
    #[unit(1000.0, "km/s²", "kilometers per second squared")]
    KilometerPerSecondSquared,
    #[unit(100.0, "hm/s²", "hectometers per second squared")]
    HectometerPerSecondSquared,
    #[unit(10.0, "dam/s²", "decameters per second squared")]
    DecameterPerSecondSquared,
    #[unit(1.0, "m/s²", "meters per second squared")]
    MeterPerSecondSquared,
    #[unit(0.10000000000000001, "dm/s²", "decimeters per second squared")]
    DecimeterPerSecondSquared,
    #[unit(0.01, "cm/s²", "centimeters per second squared")]
    CentimeterPerSecondSquared,
    #[unit(0.001, "mm/s²", "millimeters per second squared")]
    MillimeterPerSecondSquared,
    #[unit(9.9999999999999995e-07, "µm/s²", "micrometers per second squared")]
    MicrometerPerSecondSquared,
    #[unit(1.0000000000000001e-09, "nm/s²", "nanometers per second squared")]
    NanometerPerSecondSquared,
    #[unit(9.9999999999999998e-13, "pm/s²", "picometers per second squared")]
    PicometerPerSecondSquared,
    #[unit(1.0000000000000001e-15, "fm/s²", "femtometers per second squared")]
    FemtometerPerSecondSquared,
    #[unit(1.0000000000000001e-18, "am/s²", "attometers per second squared")]
    AttometerPerSecondSquared,
    #[unit(9.9999999999999991e-22, "zm/s²", "zeptometers per second squared")]
    ZeptometerPerSecondSquared,
    #[unit(9.9999999999999992e-25, "ym/s²", "yoctometers per second squared")]
    YoctometerPerSecondSquared,
    #[unit(0.30480000000000002, "ft/s²", "feet per second squared")]
    FootPerSecondSquared,
    #[unit(0.01, "Gal", "galileos")]
    Galileo,
    #[unit(0.025399999999999999, "in/s²", "inches per second squared")]
    InchPerSecondSquared,
    #[unit(2.7777777777777776e-07, "mm/min²", "millimeters per minute squared")]
    MillimeterPerMinuteSquared,
    #[unit(9.8066499999999994, "g₀", "standard accelerations of gravity")]
    StandardGravity,
}

pub type Jerk = StaticSIQuantity<
    {
        ISQ {
            m: 1,
            s: -3,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(Jerk)]
pub enum JerkUnit {
    #[unit(9.9999999999999998e+23, "Ym/s³", "yottameters per second cubed")]
    YottameterPerSecondCubed,
    #[unit(1e+21, "Zm/s³", "zettameters per second cubed")]
    ZettameterPerSecondCubed,
    #[unit(1e+18, "Em/s³", "exameters per second cubed")]
    ExameterPerSecondCubed,
    #[unit(1000000000000000.0, "Pm/s³", "petameters per second cubed")]
    PetameterPerSecondCubed,
    #[unit(1000000000000.0, "Tm/s³", "terameters per second cubed")]
    TerameterPerSecondCubed,
    #[unit(1000000000.0, "Gm/s³", "gigameters per second cubed")]
    GigameterPerSecondCubed,
    #[unit(1000000.0, "Mm/s³", "megameters per second cubed")]
    MegameterPerSecondCubed,
    #[unit(1000.0, "km/s³", "kilometers per second cubed")]
    KilometerPerSecondCubed,
    #[unit(100.0, "hm/s³", "hectometers per second cubed")]
    HectometerPerSecondCubed,
    #[unit(10.0, "dam/s³", "decameters per second cubed")]
    DecameterPerSecondCubed,
    #[unit(1.0, "m/s³", "meters per second cubed")]
    MeterPerSecondCubed,
    #[unit(0.10000000000000001, "dm/s³", "decimeters per second cubed")]
    DecimeterPerSecondCubed,
    #[unit(0.01, "cm/s³", "centimeters per second cubed")]
    CentimeterPerSecondCubed,
    #[unit(0.001, "mm/s³", "millimeters per second cubed")]
    MillimeterPerSecondCubed,
    #[unit(9.9999999999999995e-07, "µm/s³", "micrometers per second cubed")]
    MicrometerPerSecondCubed,
    #[unit(1.0000000000000001e-09, "nm/s³", "nanometers per second cubed")]
    NanometerPerSecondCubed,
    #[unit(9.9999999999999998e-13, "pm/s³", "picometers per second cubed")]
    PicometerPerSecondCubed,
    #[unit(1.0000000000000001e-15, "fm/s³", "femtometers per second cubed")]
    FemtometerPerSecondCubed,
    #[unit(1.0000000000000001e-18, "am/s³", "attometers per second cubed")]
    AttometerPerSecondCubed,
    #[unit(9.9999999999999991e-22, "zm/s³", "zeptometers per second cubed")]
    ZeptometerPerSecondCubed,
    #[unit(9.9999999999999992e-25, "ym/s³", "yoctometers per second cubed")]
    YoctometerPerSecondCubed,
    #[unit(0.30480000000000002, "ft/s³", "feet per second cubed")]
    FootPerSecondCubed,
    #[unit(0.025399999999999999, "in/s³", "inches per second cubed")]
    InchPerSecondCubed,
    #[unit(0.0046296296296296294, "km/min³", "kilometers per minute cubed")]
    KilometerPerMinuteCubed,
}

pub type Absement = StaticSIQuantity<
    {
        ISQ {
            m: 1,
            s: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(Absement)]
pub enum AbsementUnit {
    #[unit(9.9999999999999998e+23, "Ym · s", "yottameter seconds")]
    YottameterSecond,
    #[unit(1e+21, "Zm · s", "zettameter seconds")]
    ZettameterSecond,
    #[unit(1e+18, "Em · s", "exameter seconds")]
    ExameterSecond,
    #[unit(1000000000000000.0, "Pm · s", "petameter seconds")]
    PetameterSecond,
    #[unit(1000000000000.0, "Tm · s", "terameter seconds")]
    TerameterSecond,
    #[unit(1000000000.0, "Gm · s", "gigameter seconds")]
    GigameterSecond,
    #[unit(1000000.0, "Mm · s", "megameter seconds")]
    MegameterSecond,
    #[unit(1000.0, "km · s", "kilometer seconds")]
    KilometerSecond,
    #[unit(100.0, "hm · s", "hectometer seconds")]
    HectometerSecond,
    #[unit(10.0, "dam · s", "decameter seconds")]
    DecameterSecond,
    #[unit(1.0, "m · s", "meter seconds")]
    MeterSecond,
    #[unit(0.10000000000000001, "dm · s", "decimeter seconds")]
    DecimeterSecond,
    #[unit(0.01, "cm · s", "centimeter seconds")]
    CentimeterSecond,
    #[unit(0.001, "mm · s", "millimeter seconds")]
    MillimeterSecond,
    #[unit(9.9999999999999995e-07, "µm · s", "micrometer seconds")]
    MicrometerSecond,
    #[unit(1.0000000000000001e-09, "nm · s", "nanometer seconds")]
    NanometerSecond,
    #[unit(9.9999999999999998e-13, "pm · s", "picometer seconds")]
    PicometerSecond,
    #[unit(1.0000000000000001e-15, "fm · s", "femtometer seconds")]
    FemtometerSecond,
    #[unit(1.0000000000000001e-18, "am · s", "attometer seconds")]
    AttometerSecond,
    #[unit(9.9999999999999991e-22, "zm · s", "zeptometer seconds")]
    ZeptometerSecond,
    #[unit(9.9999999999999992e-25, "ym · s", "yoctometer seconds")]
    YoctometerSecond,
    #[unit(0.30480000000000002, "ft · s", "foot seconds")]
    FootSecond,
    #[unit(0.025399999999999999, "in · s", "inch seconds")]
    InchSecond,
    #[unit(3600000.0, "km · h", "kilometer hours")]
    KilometerHour,
}

pub type InverseVelocity = StaticSIQuantity<
    {
        ISQ {
            m: -1,
            s: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(InverseVelocity)]
pub enum InverseVelocityUnit {
    #[unit(9.9999999999999998e+23, "Ys/m", "yottaseconds per meter")]
    YottasecondPerMeter,
    #[unit(1e+21, "Zs/m", "zettaseconds per meter")]
    ZettasecondPerMeter,
    #[unit(1e+18, "Es/m", "exaseconds per meter")]
    ExasecondPerMeter,
    #[unit(1000000000000000.0, "Ps/m", "petaseconds per meter")]
    PetasecondPerMeter,
    #[unit(1000000000000.0, "Ts/m", "teraseconds per meter")]
    TerasecondPerMeter,
    #[unit(1000000000.0, "Gs/m", "gigaseconds per meter")]
    GigasecondPerMeter,
    #[unit(1000000.0, "Ms/m", "megaseconds per meter")]
    MegasecondPerMeter,
    #[unit(1000.0, "ks/m", "kiloseconds per meter")]
    KilosecondPerMeter,
    #[unit(100.0, "hs/m", "hectoseconds per meter")]
    HectosecondPerMeter,
    #[unit(10.0, "das/m", "decaseconds per meter")]
    DecasecondPerMeter,
    #[unit(1.0, "s/m", "seconds per meter")]
    SecondPerMeter,
    #[unit(0.10000000000000001, "ds/m", "deciseconds per meter")]
    DecisecondPerMeter,
    #[unit(0.01, "cs/m", "centiseconds per meter")]
    CentisecondPerMeter,
    #[unit(0.001, "ms/m", "milliseconds per meter")]
    MillisecondPerMeter,
    #[unit(9.9999999999999995e-07, "µs/m", "microseconds per meter")]
    MicrosecondPerMeter,
    #[unit(1.0000000000000001e-09, "ns/m", "nanoseconds per meter")]
    NanosecondPerMeter,
    #[unit(9.9999999999999998e-13, "ps/m", "picoseconds per meter")]
    PicosecondPerMeter,
    #[unit(1.0000000000000001e-15, "fs/m", "femtoseconds per meter")]
    FemtosecondPerMeter,
    #[unit(1.0000000000000001e-18, "as/m", "attoseconds per meter")]
    AttosecondPerMeter,
    #[unit(9.9999999999999991e-22, "zs/m", "zeptoseconds per meter")]
    ZeptosecondPerMeter,
    #[unit(9.9999999999999992e-25, "ys/m", "yoctoseconds per meter")]
    YoctosecondPerMeter,
    #[unit(11811.023622047243, "h/ft", "hours per foot")]
    HourPerFoot,
    #[unit(196.85039370078738, "min/ft", "minutes per foot")]
    MinutePerFoot,
    #[unit(3.280839895013123, "s/ft", "seconds per foot")]
    SecondPerFoot,
    #[unit(39.370078740157481, "s/in", "seconds per inch")]
    SecondPerInch,
    #[unit(0.059999999999999998, "min/km", "minutes per kilometer")]
    MinutePerKilometer,
    #[unit(3.6000000000000001, "h/km", "hours per kilometer")]
    HourPerKilometer,
    #[unit(2.236936292054402, "h/mi", "hours per mile")]
    HourPerMile,
    #[unit(0.037282271530000001, "min/mi", "minutes per mile")]
    MinutePerMile,
    #[unit(0.00062137119223733392, "s/mi", "seconds per mile")]
    SecondPerMile,
}

pub type Force = StaticSIQuantity<
    {
        ISQ {
            m: 1,
            kg: 1,
            s: -2,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(Force)]
pub enum ForceUnit {
    #[unit(9.9999999999999998e+23, "YN", "yottanewtons")]
    Yottanewton,
    #[unit(1e+21, "ZN", "zettanewtons")]
    Zettanewton,
    #[unit(1e+18, "EN", "exanewtons")]
    Exanewton,
    #[unit(1000000000000000.0, "PN", "petanewtons")]
    Petanewton,
    #[unit(1000000000000.0, "TN", "teranewtons")]
    Teranewton,
    #[unit(1000000000.0, "GN", "giganewtons")]
    Giganewton,
    #[unit(1000000.0, "MN", "meganewtons")]
    Meganewton,
    #[unit(1000.0, "kN", "kilonewtons")]
    Kilonewton,
    #[unit(100.0, "hN", "hectonewtons")]
    Hectonewton,
    #[unit(10.0, "daN", "decanewtons")]
    Decanewton,
    #[unit(1.0, "N", "newtons")]
    Newton,
    #[unit(0.10000000000000001, "dN", "decinewtons")]
    Decinewton,
    #[unit(0.01, "cN", "centinewtons")]
    Centinewton,
    #[unit(0.001, "mN", "millinewtons")]
    Millinewton,
    #[unit(9.9999999999999995e-07, "µN", "micronewtons")]
    Micronewton,
    #[unit(1.0000000000000001e-09, "nN", "nanonewtons")]
    Nanonewton,
    #[unit(9.9999999999999998e-13, "pN", "piconewtons")]
    Piconewton,
    #[unit(1.0000000000000001e-15, "fN", "femtonewtons")]
    Femtonewton,
    #[unit(1.0000000000000001e-18, "aN", "attonewtons")]
    Attonewton,
    #[unit(9.9999999999999991e-22, "zN", "zeptonewtons")]
    Zeptonewton,
    #[unit(9.9999999999999992e-25, "yN", "yoctonewtons")]
    Yoctonewton,
    #[unit(1.0000000000000001e-05, "dyn", "dynes")]
    Dyne,
    #[unit(9.8066499999999994, "kgf", "kilograms-force")]
    KilogramForce,
    #[unit(4448.2219999999998, "kip", "kips")]
    Kip,
    #[unit(0.27801389999999998, "ozf", "ounces-force")]
    OunceForce,
    #[unit(0.13825499999999999, "pdl", "poundals")]
    Poundal,
    #[unit(4.4482220000000003, "lbf", "pounds-force")]
    PoundForce,
    #[unit(8896.4429999999993, "2000 lbf", "tons-force")]
    TonForce,
}

pub type Momentum = StaticSIQuantity<
    {
        ISQ {
            m: 1,
            kg: 1,
            s: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(Momentum)]
pub enum MomentumUnit {
    #[unit(1e+21, "Yg · m/s", "yottagram meters per second")]
    YottagramMeterPerSecond,
    #[unit(1e+18, "Zg · m/s", "zettagram meters per second")]
    ZettagramMeterPerSecond,
    #[unit(1000000000000000.0, "Eg · m/s", "exagram meters per second")]
    ExagramMeterPerSecond,
    #[unit(1000000000000.0, "Pg · m/s", "petagram meters per second")]
    PetagramMeterPerSecond,
    #[unit(1000000000.0, "Tg · m/s", "teragram meters per second")]
    TeragramMeterPerSecond,
    #[unit(1000000.0, "Gg · m/s", "gigagram meters per second")]
    GigagramMeterPerSecond,
    #[unit(1000.0, "Mg · m/s", "megagram meters per second")]
    MegagramMeterPerSecond,
    #[unit(1.0, "kg · m/s", "kilogram meters per second")]
    KilogramMeterPerSecond,
    #[unit(0.10000000000000001, "hg · m/s", "hectogram meters per second")]
    HectogramMeterPerSecond,
    #[unit(0.01, "dag · m/s", "decagram meters per second")]
    DecagramMeterPerSecond,
    #[unit(0.001, "g · m/s", "gram meters per second")]
    GramMeterPerSecond,
    #[unit(0.0001, "dg · m/s", "decigram meters per second")]
    DecigramMeterPerSecond,
    #[unit(1.0000000000000001e-05, "cg · m/s", "centigram meters per second")]
    CentigramMeterPerSecond,
    #[unit(9.9999999999999995e-07, "mg · m/s", "milligram meters per second")]
    MilligramMeterPerSecond,
    #[unit(9.9999999999999986e-10, "µg · m/s", "microgram meters per second")]
    MicrogramMeterPerSecond,
    #[unit(9.9999999999999998e-13, "ng · m/s", "nanogram meters per second")]
    NanogramMeterPerSecond,
    #[unit(1.0000000000000001e-15, "pg · m/s", "picogram meters per second")]
    PicogramMeterPerSecond,
    #[unit(1.0000000000000001e-18, "fg · m/s", "femtogram meters per second")]
    FemtogramMeterPerSecond,
    #[unit(1.0000000000000001e-21, "ag · m/s", "attogram meters per second")]
    AttogramMeterPerSecond,
    #[unit(9.9999999999999992e-25, "zg · m/s", "zeptogram meters per second")]
    ZeptogramMeterPerSecond,
    #[unit(9.9999999999999986e-28, "yg · m/s", "yoctogram meters per second")]
    YoctogramMeterPerSecond,
    #[unit(9.9999999999999998e+23, "kg · Ym/s", "kilogram yottameters per second")]
    KilogramYottameterPerSecond,
    #[unit(1e+21, "kg · Zm/s", "kilogram zettameters per second")]
    KilogramZettameterPerSecond,
    #[unit(1e+18, "kg · Em/s", "kilogram exameters per second")]
    KilogramExameterPerSecond,
    #[unit(1000000000000000.0, "kg · Pm/s", "kilogram petameters per second")]
    KilogramPetameterPerSecond,
    #[unit(1000000000000.0, "kg · Tm/s", "kilogram terameters per second")]
    KilogramTerameterPerSecond,
    #[unit(1000000000.0, "kg · Gm/s", "kilogram gigameters per second")]
    KilogramGigameterPerSecond,
    #[unit(1000000.0, "kg · Mm/s", "kilogram megameters per second")]
    KilogramMegameterPerSecond,
    #[unit(1000.0, "kg · km/s", "kilogram kilometers per second")]
    KilogramKilometerPerSecond,
    #[unit(100.0, "kg · hm/s", "kilogram hectometers per second")]
    KilogramHectometerPerSecond,
    #[unit(10.0, "kg · dam/s", "kilogram decameters per second")]
    KilogramDecameterPerSecond,
    #[unit(0.10000000000000001, "kg · dm/s", "kilogram decimeters per second")]
    KilogramDecimeterPerSecond,
    #[unit(0.01, "kg · cm/s", "kilogram centimeters per second")]
    KilogramCentimeterPerSecond,
    #[unit(0.001, "kg · mm/s", "kilogram millimeters per second")]
    KilogramMillimeterPerSecond,
    #[unit(9.9999999999999995e-07, "kg · µm/s", "kilogram micrometers per second")]
    KilogramMicrometerPerSecond,
    #[unit(1.0000000000000001e-09, "kg · nm/s", "kilogram nanometers per second")]
    KilogramNanometerPerSecond,
    #[unit(9.9999999999999998e-13, "kg · pm/s", "kilogram picometers per second")]
    KilogramPicometerPerSecond,
    #[unit(1.0000000000000001e-15, "kg · fm/s", "kilogram femtometers per second")]
    KilogramFemtometerPerSecond,
    #[unit(1.0000000000000001e-18, "kg · am/s", "kilogram attometers per second")]
    KilogramAttometerPerSecond,
    #[unit(9.9999999999999991e-22, "kg · zm/s", "kilogram zeptometers per second")]
    KilogramZeptometerPerSecond,
    #[unit(9.9999999999999992e-25, "kg · ym/s", "kilogram yoctometers per second")]
    KilogramYoctometerPerSecond,
    #[unit(1000.0, "t · m/s", "ton meters per second")]
    TonMeterPerSecond,
    #[unit(0.016666666666666666, "kg · m/min", "kilogram meters per minute")]
    KilogramMeterPerMinute,
    #[unit(0.00027777777777777778, "kg · m/h", "kilogram meters per hour")]
    KilogramMeterPerHour,
    #[unit(1.1574074074074073e-05, "kg · m/d", "kilogram meters per day")]
    KilogramMeterPerDay,
    #[unit(4.4482207200000001, "slug · ft/s", "slug feet per second")]
    SlugFootPerSecond,
    #[unit(0.37068506000000001, "slug · in/s", "slug inches per second")]
    SlugInchPerSecond,
    #[unit(0.13825496352, "lb · ft/s", "pound feet per second")]
    PoundFootPerSecond,
    #[unit(0.01152124696, "lb · in/s", "pound inches per second")]
    PoundInchPerSecond,
}

pub type Energy = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -2,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(Energy)]
pub enum EnergyUnit {
    #[unit(9.9999999999999998e+23, "YJ", "yottajoules")]
    Yottajoule,
    #[unit(1e+21, "ZJ", "zettajoules")]
    Zettajoule,
    #[unit(1e+18, "EJ", "exajoules")]
    Exajoule,
    #[unit(1000000000000000.0, "PJ", "petajoules")]
    Petajoule,
    #[unit(1000000000000.0, "TJ", "terajoules")]
    Terajoule,
    #[unit(1000000000.0, "GJ", "gigajoules")]
    Gigajoule,
    #[unit(1000000.0, "MJ", "megajoules")]
    Megajoule,
    #[unit(1000.0, "kJ", "kilojoules")]
    Kilojoule,
    #[unit(100.0, "hJ", "hectojoules")]
    Hectojoule,
    #[unit(10.0, "daJ", "decajoules")]
    Decajoule,
    #[unit(1.0, "J", "joules")]
    Joule,
    #[unit(0.10000000000000001, "dJ", "decijoules")]
    Decijoule,
    #[unit(0.01, "cJ", "centijoules")]
    Centijoule,
    #[unit(0.001, "mJ", "millijoules")]
    Millijoule,
    #[unit(9.9999999999999995e-07, "µJ", "microjoules")]
    Microjoule,
    #[unit(1.0000000000000001e-09, "nJ", "nanojoules")]
    Nanojoule,
    #[unit(9.9999999999999998e-13, "pJ", "picojoules")]
    Picojoule,
    #[unit(1.0000000000000001e-15, "fJ", "femtojoules")]
    Femtojoule,
    #[unit(1.0000000000000001e-18, "aJ", "attojoules")]
    Attojoule,
    #[unit(9.9999999999999991e-22, "zJ", "zeptojoules")]
    Zeptojoule,
    #[unit(9.9999999999999992e-25, "yJ", "yoctojoules")]
    Yoctojoule,
    #[unit(3.6e+18, "PW · h", "petawatt hours")]
    PetawattHour,
    #[unit(3600000000000000.0, "TW · h", "terawatt hours")]
    TerawattHour,
    #[unit(3600000000000.0, "GW · h", "gigawatt hours")]
    GigawattHour,
    #[unit(3600000000.0, "MW · h", "megawatt hours")]
    MegawattHour,
    #[unit(3600000.0, "kW · h", "kilowatt hours")]
    KilowattHour,
    #[unit(360000.0, "hW · h", "hectowatt hours")]
    HectowattHour,
    #[unit(36000.0, "daW · h", "decawatt hours")]
    DecawattHour,
    #[unit(3600.0, "W · h", "watt hours")]
    WattHour,
    #[unit(3.6000000000000001, "mW · h", "milliwatt hours")]
    MilliwattHour,
    #[unit(0.0035999999999999999, "µW · h", "microwatt hours")]
    MicrowattHour,
    #[unit(0.00016021766339999999, "PeV", "petaelectronvolts")]
    Petaelectronvolt,
    #[unit(1.6021766339999999e-07, "TeV", "teraelectronvolts")]
    Teraelectronvolt,
    #[unit(1.6021766340000001e-10, "GeV", "gigaelectronvolts")]
    Gigaelectronvolt,
    #[unit(1.6021766340000001e-13, "MeV", "megaelectronvolts")]
    Megaelectronvolt,
    #[unit(1.6021766339999999e-16, "keV", "kiloelectronvolts")]
    Kiloelectronvolt,
    #[unit(1.6021766339999999e-17, "heV", "hectoelectronvolts")]
    Hectoelectronvolt,
    #[unit(1.6021766339999999e-18, "daeV", "decaelectronvolts")]
    Decaelectronvolt,
    #[unit(1.6021766339999999e-19, "eV", "electronvolts")]
    Electronvolt,
    #[unit(4.3597447222071e-18, "Eₕ", "hartrees")]
    Hartree,
    #[unit(1055.056, "Btu (IT)", "British thermal units (IT)")]
    BtuIt,
    #[unit(1054.3499999999999, "Btu", "British thermal units")]
    Btu,
    #[unit(4.1867999999999999, "cal (IT)", "calories (IT)")]
    CalorieIt,
    #[unit(4.1840000000000002, "cal", "calories")]
    Calorie,
    #[unit(4186.8000000000002, "Cal (IT)", "Calories (IT)")]
    CalorieItNutrition,
    #[unit(4184.0, "Cal", "Calories")]
    CalorieNutrition,
    #[unit(9.9999999999999995e-08, "erg", "ergs")]
    Erg,
    #[unit(0.042140110000000001, "ft · pdl", "foot poundals")]
    FootPoundal,
    #[unit(1.355818, "ft · lbf", "foot pounds-force")]
    FootPound,
    #[unit(4186.8000000000002, "kcal (IT)", "kilocalories (IT)")]
    KilocalorieIt,
    #[unit(4184.0, "kcal", "kilocalories")]
    Kilocalorie,
    #[unit(1.055056e+18, "10¹⁵ Btu (IT)", "quads")]
    Quad,
    #[unit(105506000.0, "thm (EC)", "therms (EC)")]
    ThermEc,
    #[unit(105480400.0, "thm", "therms")]
    ThermUs,
    #[unit(4184000000.0, "t of TNT", "tons of TNT")]
    TonTnt,
    #[unit(1.0, "W · s", "watt seconds")]
    WattSecond,
}

pub type AvailableEnergy = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            s: -2,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(AvailableEnergy)]
pub enum AvailableEnergyUnit {
    #[unit(9.9999999999999998e+23, "YJ/kg", "yottajoules per kilogram")]
    YottajoulePerKilogram,
    #[unit(1e+21, "ZJ/kg", "zettajoules per kilogram")]
    ZettajoulePerKilogram,
    #[unit(1e+18, "EJ/kg", "exajoules per kilogram")]
    ExajoulePerKilogram,
    #[unit(1000000000000000.0, "PJ/kg", "petajoules per kilogram")]
    PetajoulePerKilogram,
    #[unit(1000000000000.0, "TJ/kg", "terajoules per kilogram")]
    TerajoulePerKilogram,
    #[unit(1000000000.0, "GJ/kg", "gigajoules per kilogram")]
    GigajoulePerKilogram,
    #[unit(1000000.0, "MJ/kg", "megajoules per kilogram")]
    MegajoulePerKilogram,
    #[unit(1000.0, "kJ/kg", "kilojoules per kilogram")]
    KilojoulePerKilogram,
    #[unit(100.0, "hJ/kg", "hectojoules per kilogram")]
    HectojoulePerKilogram,
    #[unit(10.0, "daJ/kg", "decajoules per kilogram")]
    DecajoulePerKilogram,
    #[unit(1.0, "J/kg", "joules per kilogram")]
    JoulePerKilogram,
    #[unit(0.10000000000000001, "dJ/kg", "decijoules per kilogram")]
    DecijoulePerKilogram,
    #[unit(0.01, "cJ/kg", "centijoules per kilogram")]
    CentijoulePerKilogram,
    #[unit(0.001, "mJ/kg", "millijoules per kilogram")]
    MillijoulePerKilogram,
    #[unit(9.9999999999999995e-07, "µJ/kg", "microjoules per kilogram")]
    MicrojoulePerKilogram,
    #[unit(1.0000000000000001e-09, "nJ/kg", "nanojoules per kilogram")]
    NanojoulePerKilogram,
    #[unit(9.9999999999999998e-13, "pJ/kg", "picojoules per kilogram")]
    PicojoulePerKilogram,
    #[unit(1.0000000000000001e-15, "fJ/kg", "femtojoules per kilogram")]
    FemtojoulePerKilogram,
    #[unit(1.0000000000000001e-18, "aJ/kg", "attojoules per kilogram")]
    AttojoulePerKilogram,
    #[unit(9.9999999999999991e-22, "zJ/kg", "zeptojoules per kilogram")]
    ZeptojoulePerKilogram,
    #[unit(9.9999999999999992e-25, "yJ/kg", "yoctojoules per kilogram")]
    YoctojoulePerKilogram,
    #[unit(9.9999999999999998e+23, "J/zg", "joules per zeptogram")]
    JoulePerZeptogram,
    #[unit(1e+21, "J/ag", "joules per attogram")]
    JoulePerAttogram,
    #[unit(1e+18, "J/fg", "joules per femtogram")]
    JoulePerFemtogram,
    #[unit(1000000000000000.0, "J/pg", "joules per picogram")]
    JoulePerPicogram,
    #[unit(1000000000000.0, "J/ng", "joules per nanogram")]
    JoulePerNanogram,
    #[unit(1000000000.0, "J/µg", "joules per microgram")]
    JoulePerMicrogram,
    #[unit(1000000.0, "J/mg", "joules per milligram")]
    JoulePerMilligram,
    #[unit(1000.0, "J/g", "joules per gram")]
    JoulePerGram,
    #[unit(0.001, "J/Mg", "joules per megagram")]
    JoulePerMegagram,
    #[unit(9.9999999999999995e-07, "J/Gg", "joules per gigagram")]
    JoulePerGigagram,
    #[unit(1.0000000000000001e-09, "J/Tg", "joules per teragram")]
    JoulePerTeragram,
    #[unit(9.9999999999999998e-13, "J/Pg", "joules per petagram")]
    JoulePerPetagram,
    #[unit(1.0000000000000001e-15, "J/Eg", "joules per exagram")]
    JoulePerExagram,
    #[unit(1.0000000000000001e-18, "J/Zg", "joules per zettagram")]
    JoulePerZettagram,
    #[unit(9.9999999999999991e-22, "J/Yg", "joules per yottagram")]
    JoulePerYottagram,
    #[unit(
        2326.0001710787042,
        "Btu (IT)/lb",
        "British thermal units (IT) per pound"
    )]
    BtuItPerPound,
    #[unit(2324.443707610621, "Btu/lb", "British thermal units per pound")]
    BtuPerPound,
    #[unit(4186.8000000000002, "cal (IT)/lb", "calories (IT) per gram")]
    CalorieItPerGram,
    #[unit(4184.0, "cal/lb", "calories per gram")]
    CaloriePerGram,
}

pub type Power = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -3,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(Power)]
pub enum PowerUnit {
    #[unit(9.9999999999999998e+23, "YW", "yottawatts")]
    Yottawatt,
    #[unit(1e+21, "ZW", "zettawatts")]
    Zettawatt,
    #[unit(1e+18, "EW", "exawatts")]
    Exawatt,
    #[unit(1000000000000000.0, "PW", "petawatts")]
    Petawatt,
    #[unit(1000000000000.0, "TW", "terawatts")]
    Terawatt,
    #[unit(1000000000.0, "GW", "gigawatts")]
    Gigawatt,
    #[unit(1000000.0, "MW", "megawatts")]
    Megawatt,
    #[unit(1000.0, "kW", "kilowatts")]
    Kilowatt,
    #[unit(100.0, "hW", "hectowatts")]
    Hectowatt,
    #[unit(10.0, "daW", "decawatts")]
    Decawatt,
    #[unit(1.0, "W", "watts")]
    Watt,
    #[unit(0.10000000000000001, "dW", "deciwatts")]
    Deciwatt,
    #[unit(0.01, "cW", "centiwatts")]
    Centiwatt,
    #[unit(0.001, "mW", "milliwatts")]
    Milliwatt,
    #[unit(9.9999999999999995e-07, "µW", "microwatts")]
    Microwatt,
    #[unit(1.0000000000000001e-09, "nW", "nanowatts")]
    Nanowatt,
    #[unit(9.9999999999999998e-13, "pW", "picowatts")]
    Picowatt,
    #[unit(1.0000000000000001e-15, "fW", "femtowatts")]
    Femtowatt,
    #[unit(1.0000000000000001e-18, "aW", "attowatts")]
    Attowatt,
    #[unit(9.9999999999999991e-22, "zW", "zeptowatts")]
    Zeptowatt,
    #[unit(9.9999999999999992e-25, "yW", "yoctowatts")]
    Yoctowatt,
    #[unit(9.9999999999999995e-08, "erg/s", "ergs per second")]
    ErgPerSecond,
    #[unit(0.00037661611111111112, "ft · lbf/h", "foot pounds-force per hour")]
    FootPoundPerHour,
    #[unit(0.022596966666666666, "ft · lbf/min", "foot pounds-force per minute")]
    FootPoundPerMinute,
    #[unit(1.355818, "ft · lbf/s", "foot pounds-force per second")]
    FootPoundPerSecond,
    #[unit(745.69989999999996, "hp", "horsepower")]
    Horsepower,
    #[unit(9809.5, "hp (S)", "horsepower (boiler)")]
    HorsepowerBoiler,
    #[unit(746.0, "hp (E)", "horsepower (electric)")]
    HorsepowerElectric,
    #[unit(735.49879999999996, "hp (M)", "metric horsepower")]
    HorsepowerMetric,
    #[unit(745.70000000000005, "hp (I)", "horsepower (Imperial)")]
    HorsepowerImperial,
    #[unit(746.04300000000001, "hp (hydraulic)", "hydraulic horsepower")]
    HydraulicHorsepower,
}

pub type PowerRate = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -4,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(PowerRate)]
pub enum PowerRateUnit {
    #[unit(9.9999999999999998e+23, "YW/s", "yottawatts per second")]
    YottawattPerSecond,
    #[unit(1e+21, "ZW/s", "zettawatts per second")]
    ZettawattPerSecond,
    #[unit(1e+18, "EW/s", "exawatts per second")]
    ExawattPerSecond,
    #[unit(1000000000000000.0, "PW/s", "petawatts per second")]
    PetawattPerSecond,
    #[unit(1000000000000.0, "TW/s", "terawatts per second")]
    TerawattPerSecond,
    #[unit(1000000000.0, "GW/s", "gigawatts per second")]
    GigawattPerSecond,
    #[unit(1000000.0, "MW/s", "megawatts per second")]
    MegawattPerSecond,
    #[unit(1000.0, "kW/s", "kilowatts per second")]
    KilowattPerSecond,
    #[unit(100.0, "hW/s", "hectowatts per second")]
    HectowattPerSecond,
    #[unit(10.0, "daW/s", "decawatts per second")]
    DecawattPerSecond,
    #[unit(1.0, "W/s", "watts per second")]
    WattPerSecond,
    #[unit(0.10000000000000001, "dW/s", "deciwatts per second")]
    DeciwattPerSecond,
    #[unit(0.01, "cW/s", "centiwatts per second")]
    CentiwattPerSecond,
    #[unit(0.001, "mW/s", "milliwatts per second")]
    MilliwattPerSecond,
    #[unit(9.9999999999999995e-07, "µW/s", "microwatts per second")]
    MicrowattPerSecond,
    #[unit(1.0000000000000001e-09, "nW/s", "nanowatts per second")]
    NanowattPerSecond,
    #[unit(9.9999999999999998e-13, "pW/s", "picowatts per second")]
    PicowattPerSecond,
    #[unit(1.0000000000000001e-15, "fW/s", "femtowatts per second")]
    FemtowattPerSecond,
    #[unit(1.0000000000000001e-18, "aW/s", "attowatts per second")]
    AttowattPerSecond,
    #[unit(9.9999999999999991e-22, "zW/s", "zeptowatts per second")]
    ZeptowattPerSecond,
    #[unit(9.9999999999999992e-25, "yW/s", "yoctowatts per second")]
    YoctowattPerSecond,
    #[unit(9.9999999999999995e-08, "erg/s²", "ergs per second squared")]
    ErgPerSecondSquared,
    #[unit(1.355818, "ft · lbf/s²", "foot pounds-force per second squared")]
    FootPoundPerSecondSquared,
    #[unit(745.69989999999996, "hp/s", "horsepower per second")]
    HorsepowerPerSecond,
    #[unit(9809.5, "hp/s (S)", "horsepower per second (boiler)")]
    HorsepowerPerSecondBoiler,
    #[unit(746.0, "hp/s (E)", "horsepower per second (electric)")]
    HorsepowerPerSecondElectric,
    #[unit(735.49879999999996, "hp/s (M)", "metric horsepower per second")]
    HorsepowerPerSecondMetric,
    #[unit(745.70000000000005, "hp/s (I)", "horsepower per second (Imperial)")]
    HorsepowerPerSecondImperial,
    #[unit(
        746.04300000000001,
        "hp/s (hydraulic)",
        "hydraulic horsepower per second"
    )]
    HydraulicHorsepowerPerSecond,
}

pub type Action = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(Action)]
pub enum ActionUnit {
    #[unit(1.0, "J · s", "joule seconds")]
    JouleSecond,
    #[unit(1.054571817e-34, "ħ", "atomic units of action")]
    AtomicUnitOfAction,
    #[unit(1.054571817e-34, "ħ", "reduced planck constants")]
    ReducedPlanckConstant,
    #[unit(6.6260701499999998e-34, "h", "planck constants")]
    PlanckConstant,
    #[unit(9.9999999999999995e-08, "erg · s", "erg seconds")]
    ErgSecond,
    #[unit(1.6021766339999999e-19, "eV · s", "electronvolt seconds")]
    ElectronvoltSecond,
}

pub type Torque = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -2,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(Torque)]
pub enum TorqueUnit {
    #[unit(9.9999999999999998e+23, "YN · m", "yottanewton meters")]
    YottanewtonMeter,
    #[unit(1e+21, "ZN · m", "zettanewton meters")]
    ZettanewtonMeter,
    #[unit(1e+18, "EN · m", "exanewton meters")]
    ExanewtonMeter,
    #[unit(1000000000000000.0, "PN · m", "petanewton meters")]
    PetanewtonMeter,
    #[unit(1000000000000.0, "TN · m", "teranewton meters")]
    TeranewtonMeter,
    #[unit(1000000000.0, "GN · m", "giganewton meters")]
    GiganewtonMeter,
    #[unit(1000000.0, "MN · m", "meganewton meters")]
    MeganewtonMeter,
    #[unit(1000.0, "kN · m", "kilonewton meters")]
    KilonewtonMeter,
    #[unit(100.0, "hN · m", "hectonewton meters")]
    HectonewtonMeter,
    #[unit(10.0, "daN · m", "decanewton meters")]
    DecanewtonMeter,
    #[unit(1.0, "N · m", "newton meters")]
    NewtonMeter,
    #[unit(0.10000000000000001, "dN · m", "decinewton meters")]
    DecinewtonMeter,
    #[unit(0.01, "cN · m", "centinewton meters")]
    CentinewtonMeter,
    #[unit(0.001, "mN · m", "millinewton meters")]
    MillinewtonMeter,
    #[unit(9.9999999999999995e-07, "µN · m", "micronewton meters")]
    MicronewtonMeter,
    #[unit(1.0000000000000001e-09, "nN · m", "nanonewton meters")]
    NanonewtonMeter,
    #[unit(9.9999999999999998e-13, "pN · m", "piconewton meters")]
    PiconewtonMeter,
    #[unit(1.0000000000000001e-15, "fN · m", "femtonewton meters")]
    FemtonewtonMeter,
    #[unit(1.0000000000000001e-18, "aN · m", "attonewton meters")]
    AttonewtonMeter,
    #[unit(9.9999999999999991e-22, "zN · m", "zeptonewton meters")]
    ZeptonewtonMeter,
    #[unit(9.9999999999999992e-25, "yN · m", "yoctonewton meters")]
    YoctonewtonMeter,
    #[unit(9.9999999999999998e+23, "N · Ym", "newton yottameters")]
    NewtonYottameter,
    #[unit(1e+21, "N · Zm", "newton zettameters")]
    NewtonZettameter,
    #[unit(1e+18, "N · Em", "newton exameters")]
    NewtonExameter,
    #[unit(1000000000000000.0, "N · Pm", "newton petameters")]
    NewtonPetameter,
    #[unit(1000000000000.0, "N · Tm", "newton terameters")]
    NewtonTerameter,
    #[unit(1000000000.0, "N · Gm", "newton gigameters")]
    NewtonGigameter,
    #[unit(1000000.0, "N · Mm", "newton megameters")]
    NewtonMegameter,
    #[unit(1000.0, "N · km", "newton kilometers")]
    NewtonKilometer,
    #[unit(100.0, "N · hm", "newton hectometers")]
    NewtonHectometer,
    #[unit(10.0, "N · dam", "newton decameters")]
    NewtonDecameter,
    #[unit(0.10000000000000001, "N · dm", "newton decimeters")]
    NewtonDecimeter,
    #[unit(0.01, "N · cm", "newton centimeters")]
    NewtonCentimeter,
    #[unit(0.001, "N · mm", "newton millimeters")]
    NewtonMillimeter,
    #[unit(9.9999999999999995e-07, "N · µm", "newton micrometers")]
    NewtonMicrometer,
    #[unit(1.0000000000000001e-09, "N · nm", "newton nanometers")]
    NewtonNanometer,
    #[unit(9.9999999999999998e-13, "N · pm", "newton picometers")]
    NewtonPicometer,
    #[unit(1.0000000000000001e-15, "N · fm", "newton femtometers")]
    NewtonFemtometer,
    #[unit(1.0000000000000001e-18, "N · am", "newton attometers")]
    NewtonAttometer,
    #[unit(9.9999999999999991e-22, "N · zm", "newton zeptometers")]
    NewtonZeptometer,
    #[unit(9.9999999999999992e-25, "N · ym", "newton yoctometers")]
    NewtonYoctometer,
    #[unit(1.0000000000000001e-05, "dyn · m", "dyne meters")]
    DyneMeter,
    #[unit(9.9999999999999995e-08, "dyn · cm", "dyne centimeters")]
    DyneCentimeter,
    #[unit(9.8066499999999994, "kgf · m", "kilogram-force meters")]
    KilogramForceMeter,
    #[unit(0.0070615530599999997, "ozf · in", "ounces-force inches")]
    OunceForceInch,
    #[unit(1.3558180656000001, "lbf · ft", "pounds-force feet")]
    PoundForceFoot,
    #[unit(0.1129848388, "lbf · in", "pounds-force inches")]
    PoundForceInch,
}

pub type MomentOfInertia = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(MomentOfInertia)]
pub enum MomentOfInertiaUnit {
    #[unit(1.0, "kg · m²", "kilogram square meters")]
    KilogramSquareMeter,
    #[unit(1.6605390666e-47, "Da · Å²", "dalton square ångströms")]
    DaltonSquareAngstrom,
}

pub type Pressure = StaticSIQuantity<
    {
        ISQ {
            m: -1,
            kg: 1,
            s: -2,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(Pressure)]
pub enum PressureUnit {
    #[unit(9.9999999999999998e+23, "YPa", "yottapascals")]
    Yottapascal,
    #[unit(1e+21, "ZPa", "zettapascals")]
    Zettapascal,
    #[unit(1e+18, "EPa", "exapascals")]
    Exapascal,
    #[unit(1000000000000000.0, "PPa", "petapascals")]
    Petapascal,
    #[unit(1000000000000.0, "TPa", "terapascals")]
    Terapascal,
    #[unit(1000000000.0, "GPa", "gigapascals")]
    Gigapascal,
    #[unit(1000000.0, "MPa", "megapascals")]
    Megapascal,
    #[unit(1000.0, "kPa", "kilopascals")]
    Kilopascal,
    #[unit(100.0, "hPa", "hectopascals")]
    Hectopascal,
    #[unit(10.0, "daPa", "decapascals")]
    Decapascal,
    #[unit(1.0, "Pa", "pascals")]
    Pascal,
    #[unit(0.10000000000000001, "dPa", "decipascals")]
    Decipascal,
    #[unit(0.01, "cPa", "centipascals")]
    Centipascal,
    #[unit(0.001, "mPa", "millipascals")]
    Millipascal,
    #[unit(9.9999999999999995e-07, "µPa", "micropascals")]
    Micropascal,
    #[unit(1.0000000000000001e-09, "nPa", "nanopascals")]
    Nanopascal,
    #[unit(9.9999999999999998e-13, "pPa", "picopascals")]
    Picopascal,
    #[unit(1.0000000000000001e-15, "fPa", "femtopascals")]
    Femtopascal,
    #[unit(1.0000000000000001e-18, "aPa", "attopascals")]
    Attopascal,
    #[unit(9.9999999999999991e-22, "zPa", "zeptopascals")]
    Zeptopascal,
    #[unit(9.9999999999999992e-25, "yPa", "yoctopascals")]
    Yoctopascal,
    #[unit(101325.0, "atm", "atmospheres")]
    Atmosphere,
    #[unit(98066.5, "at", "atmospheres (technical)")]
    AtmosphereTechnical,
    #[unit(100000.0, "bar", "bar")]
    Bar,
    #[unit(1333.2239999999999, "cm Hg", "centimeters of mercury")]
    CentimeterOfMercury,
    #[unit(98.066500000000005, "cm H₂O", "centimeters of water")]
    CentimeterOfWater,
    #[unit(0.10000000000000001, "dyn/cm²", "dynes per square centimeter")]
    DynePerSquareCentimeter,
    #[unit(40636.660000000003, "ft Hg", "feet of mercury")]
    FootOfMercury,
    #[unit(2989.067, "ft H₂O", "feet of water")]
    FootOfWater,
    #[unit(3386.3890000000001, "in Hg", "inches of mercury")]
    InchOfMercury,
    #[unit(249.0889, "in H₂O", "inches of water")]
    InchOfWater,
    #[unit(1000000.0, "N/mm²", "newtons per square millimeter")]
    NewtonPerSquareMillimeter,
    #[unit(6894757.889515779, "kip/in²", "kips per square inch")]
    KipPerSquareInch,
    #[unit(100.0, "mbar", "millibar")]
    Millibar,
    #[unit(133.32239999999999, "mm Hg", "millimeters of mercury")]
    MillimeterOfMercury,
    #[unit(9.8066499999999994, "mm H₂O", "millimeters of water")]
    MillimeterOfWater,
    #[unit(0.13332240000000001, "mTorr", "millitorr")]
    Millitorr,
    #[unit(1.4881644346622025, "pdl/ft²", "poundals per square foot")]
    PoundalPerSquareFoot,
    #[unit(47.880263121637356, "lbf/ft²", "pounds-force per square foot")]
    PoundForcePerSquareFoot,
    #[unit(6894.7578895157794, "lbf/in²", "pounds-force per square inch")]
    PoundForcePerSquareInch,
    #[unit(6894.7569999999996, "psi", "pounds-force per square inch")]
    Psi,
    #[unit(133.32239999999999, "Torr", "torr")]
    Torr,
}

pub type Frequency = StaticSIQuantity<{ ISQ { s: -1, ..ISQ::ZERO } }>;

#[derive(Copy, Clone, Unit)]
#[dim(Frequency)]
pub enum FrequencyUnit {
    #[unit(9.9999999999999998e+23, "YHz", "yottahertz")]
    Yottahertz,
    #[unit(1e+21, "ZHz", "zettahertz")]
    Zettahertz,
    #[unit(1e+18, "EHz", "exahertz")]
    Exahertz,
    #[unit(1000000000000000.0, "PHz", "petahertz")]
    Petahertz,
    #[unit(1000000000000.0, "THz", "terahertz")]
    Terahertz,
    #[unit(1000000000.0, "GHz", "gigahertz")]
    Gigahertz,
    #[unit(1000000.0, "MHz", "megahertz")]
    Megahertz,
    #[unit(1000.0, "kHz", "kilohertz")]
    Kilohertz,
    #[unit(100.0, "hHz", "hectohertz")]
    Hectohertz,
    #[unit(10.0, "daHz", "decahertz")]
    Decahertz,
    #[unit(1.0, "Hz", "hertz")]
    Hertz,
    #[unit(0.10000000000000001, "dHz", "decihertz")]
    Decihertz,
    #[unit(0.01, "cHz", "centihertz")]
    Centihertz,
    #[unit(0.001, "mHz", "millihertz")]
    Millihertz,
    #[unit(9.9999999999999995e-07, "µHz", "microhertz")]
    Microhertz,
    #[unit(1.0000000000000001e-09, "nHz", "nanohertz")]
    Nanohertz,
    #[unit(9.9999999999999998e-13, "pHz", "picohertz")]
    Picohertz,
    #[unit(1.0000000000000001e-15, "fHz", "femtohertz")]
    Femtohertz,
    #[unit(1.0000000000000001e-18, "aHz", "attohertz")]
    Attohertz,
    #[unit(9.9999999999999991e-22, "zHz", "zeptohertz")]
    Zeptohertz,
    #[unit(9.9999999999999992e-25, "yHz", "yoctohertz")]
    Yoctohertz,
    #[unit(1.1574074074074073e-05, "1/d", "cycles per day")]
    CyclePerDay,
    #[unit(0.00027777777777777772, "1/h", "cycles per hour")]
    CyclePerHour,
    #[unit(0.016666666666666666, "1/min", "cycles per minute")]
    CyclePerMinute,
    #[unit(100000000.0, "100 MHz", "cycles per shake")]
    CyclePerShake,
    #[unit(3.1709791983764579e-08, "1/a", "cycles per year")]
    CyclePerYear,
}

pub type FrequencyDrift = StaticSIQuantity<{ ISQ { s: -2, ..ISQ::ZERO } }>;

#[derive(Copy, Clone, Unit)]
#[dim(FrequencyDrift)]
pub enum FrequencyDriftUnit {
    #[unit(1000000000000.0, "THz/s", "terahertz per second")]
    TerahertzPerSecond,
    #[unit(1000000000.0, "GHz/s", "gigahertz per second")]
    GigahertzPerSecond,
    #[unit(1000000.0, "MHz/s", "megahertz per second")]
    MegahertzPerSecond,
    #[unit(1000.0, "kHz/s", "kilohertz per second")]
    KilohertzPerSecond,
    #[unit(1.0, "Hz/s", "hertz per second")]
    HertzPerSecond,
}

pub type ReciprocalLength = StaticSIQuantity<{ ISQ { m: -1, ..ISQ::ZERO } }>;

#[derive(Copy, Clone, Unit)]
#[dim(ReciprocalLength)]
pub enum ReciprocalLengthUnit {
    #[unit(0.001, "km⁻¹", "reciprocal kilometers")]
    ReciprocalKilometer,
    #[unit(1.0, "m⁻¹", "reciprocal meters")]
    ReciprocalMeter,
    #[unit(10.0, "dm⁻¹", "reciprocal decimeters")]
    ReciprocalDecimeter,
    #[unit(100.0, "cm⁻¹", "reciprocal centimeters")]
    ReciprocalCentimeter,
    #[unit(1000.0, "mm⁻¹", "reciprocal millimeters")]
    ReciprocalMillimeter,
    #[unit(1000000.0, "µm⁻¹", "reciprocal micrometers")]
    ReciprocalMicrometer,
    #[unit(999999999.99999988, "nm⁻¹", "reciprocal nanometers")]
    ReciprocalNanometer,
    #[unit(10000000000.0, "Å⁻¹", "reciprocal ångströms")]
    ReciprocalAngstrom,
    #[unit(1.0, "dpt", "diopters")]
    Diopter,
    #[unit(10973731.568159999, "R∞", "Rydberg constants")]
    RydbergConstant,
}

pub type Curvature = StaticSIQuantity<{ ISQ { m: -1, ..ISQ::ZERO } }>;

#[derive(Copy, Clone, Unit)]
#[dim(Curvature)]
pub enum CurvatureUnit {
    #[unit(1.0, "rad/m", "radians per meter")]
    RadianPerMeter,
    #[unit(0.017453292519943295, "°/m", "degrees per meter")]
    DegreePerMeter,
    #[unit(1000.0, "rad/mm", "radians per millimeter")]
    RadianPerMillimeter,
    #[unit(17.453292519943297, "°/mm", "degrees per millimeter")]
    DegreePerMillimeter,
}

pub type SpecificArea = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(SpecificArea)]
pub enum SpecificAreaUnit {
    #[unit(1.0, "m²/kg", "square meters per kilogram")]
    SquareMeterPerKilogram,
    #[unit(0.0001, "cm²/kg", "square centimeters per kilogram")]
    SquareCentimeterPerKilogram,
    #[unit(1000.0, "m²/g", "square meters per gram")]
    SquareMeterPerGram,
}

pub type SpecificVolume = StaticSIQuantity<
    {
        ISQ {
            m: 3,
            kg: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(SpecificVolume)]
pub enum SpecificVolumeUnit {
    #[unit(1.0000000000000001e-21, "m³/Yg", "cubic meters per yottagram")]
    CubicMeterPerYottagram,
    #[unit(1.0000000000000001e-18, "m³/Zg", "cubic meters per zettagram")]
    CubicMeterPerZettagram,
    #[unit(1.0000000000000001e-15, "m³/Eg", "cubic meters per exagram")]
    CubicMeterPerExagram,
    #[unit(9.9999999999999998e-13, "m³/Pg", "cubic meters per petagram")]
    CubicMeterPerPetagram,
    #[unit(1.0000000000000001e-09, "m³/Tg", "cubic meters per teragram")]
    CubicMeterPerTeragram,
    #[unit(9.9999999999999995e-07, "m³/Gg", "cubic meters per gigagram")]
    CubicMeterPerGigagram,
    #[unit(0.001, "m³/Mg", "cubic meters per megagram")]
    CubicMeterPerMegagram,
    #[unit(1.0, "m³/kg", "cubic meters per kilogram")]
    CubicMeterPerKilogram,
    #[unit(10.0, "m³/hg", "cubic meters per hectogram")]
    CubicMeterPerHectogram,
    #[unit(100.0, "m³/dag", "cubic meters per decagram")]
    CubicMeterPerDecagram,
    #[unit(1000.0, "m³/g", "cubic meters per gram")]
    CubicMeterPerGram,
    #[unit(10000.0, "m³/dg", "cubic meters per decigram")]
    CubicMeterPerDecigram,
    #[unit(100000.0, "m³/cg", "cubic meters per centigram")]
    CubicMeterPerCentigram,
    #[unit(1000000.0, "m³/mg", "cubic meters per milligram")]
    CubicMeterPerMilligram,
    #[unit(1000000000.0, "m³/µg", "cubic meters per microgram")]
    CubicMeterPerMicrogram,
    #[unit(999999999999.99988, "m³/ng", "cubic meters per nanogram")]
    CubicMeterPerNanogram,
    #[unit(1000000000000000.0, "m³/pg", "cubic meters per picogram")]
    CubicMeterPerPicogram,
    #[unit(9.9999999999999987e+17, "m³/fg", "cubic meters per femtogram")]
    CubicMeterPerFemtogram,
    #[unit(9.9999999999999987e+20, "m³/ag", "cubic meters per attogram")]
    CubicMeterPerAttogram,
    #[unit(1.0000000000000001e+24, "m³/zg", "cubic meters per zeptogram")]
    CubicMeterPerZeptogram,
    #[unit(1e+27, "m³/yg", "cubic meters per yoctogram")]
    CubicMeterPerYoctogram,
    #[unit(5000.0, "m³/ct", "cubic meters per carat")]
    CubicMeterPerCarat,
    #[unit(15432.358352941432, "m³/gr", "cubic meters per grain")]
    CubicMeterPerGrain,
    #[unit(
        0.019684128785380992,
        "m³/cwt long",
        "cubic meters per hundredweight (long)"
    )]
    CubicMeterPerHundredweightLong,
    #[unit(
        0.022046224760379582,
        "m³/cwt short",
        "cubic meters per hundredweight (short)"
    )]
    CubicMeterPerHundredweightShort,
    #[unit(35.273965837869568, "m³/oz", "cubic meters per ounce")]
    CubicMeterPerOunce,
    #[unit(32.150743260882706, "m³/oz t", "cubic meters per troy ounce")]
    CubicMeterPerOunceTroy,
    #[unit(643.01486521765401, "m³/dwt", "cubic meters per pennyweight")]
    CubicMeterPerPennyweight,
    #[unit(2.2046224760379585, "m³/lb", "cubic meters per pound")]
    CubicMeterPerPound,
    #[unit(2.6792290357695832, "m³/lb t", "cubic meters per troy pound")]
    CubicMeterPerPoundTroy,
    #[unit(0.068521779647661013, "m³/slug", "cubic meters per slug")]
    CubicMeterPerSlug,
    #[unit(34.285710367347392, "m³/AT", "cubic meters per assay ton")]
    CubicMeterPerTonAssay,
    #[unit(0.00098420643926904958, "m³/2240 lb", "cubic meters per long ton")]
    CubicMeterPerTonLong,
    #[unit(0.0011023113595279991, "m³/2000 lb", "cubic meters per short ton")]
    CubicMeterPerTonShort,
    #[unit(0.001, "m³/t", "cubic meters per ton")]
    CubicMeterPerTon,
    #[unit(58.417834497524737, "gal/gr", "gallons per grain")]
    GallonPerGrain,
    #[unit(0.00057803659462311895, "in³/oz", "cubic inches per ounce")]
    CubicInchPerOunce,
    #[unit(0.16035862335588044, "gal (UK)/oz", "Imperial gallons per ounce")]
    GallonImperialPerOunce,
    #[unit(0.1335264935702615, "gal/oz", "gallons per ounce")]
    GallonPerOunce,
    #[unit(0.06242796396059546, "ft³/lb", "cubic feet per pound")]
    CubicFootPerPound,
    #[unit(3.6127280792182591e-05, "in³/lb", "cubic inches per pound")]
    CubicInchPerPound,
    #[unit(1.6855549167049537, "yd³/lb", "cubic yards per pound")]
    CubicYardPerPound,
    #[unit(0.010022412192091401, "gal (UK)/lb", "Imperial gallons per pound")]
    GallonImperialPerPound,
    #[unit(0.0083454043762638003, "gal/lb", "gallons per pound")]
    GallonPerPound,
    #[unit(0.0019403209560158697, "ft³/slug", "cubic feett per slug")]
    CubicFootPerSlug,
    #[unit(0.00075247985575470431, "yd³/2240 lb", "cubic yards per long ton")]
    CubicYardPerTonLong,
    #[unit(0.00084277755125279337, "yd³/2000 lb", "cubic yards per short ton")]
    CubicYardPerTonShort,
    #[unit(2.7869626109815785e-05, "ft³/2240 lb", "cubic feet per long ton")]
    CubicFootPerTonLong,
    #[unit(3.1213985421050424e-05, "ft³/2000 lb", "cubic feet per short ton")]
    CubicFootPerTonShort,
}

pub type SpecificPower = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            s: -3,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(SpecificPower)]
pub enum SpecificPowerUnit {
    #[unit(9.9999999999999998e+23, "YW/kg", "yottawatts per kilogram")]
    YottawattPerKilogram,
    #[unit(1e+21, "ZW/kg", "zettawatts per kilogram")]
    ZettawattPerKilogram,
    #[unit(1e+18, "EW/kg", "exawatts per kilogram")]
    ExawattPerKilogram,
    #[unit(1000000000000000.0, "PW/kg", "petawatts per kilogram")]
    PetawattPerKilogram,
    #[unit(1000000000000.0, "TW/kg", "terawatts per kilogram")]
    TerawattPerKilogram,
    #[unit(1000000000.0, "GW/kg", "gigawatts per kilogram")]
    GigawattPerKilogram,
    #[unit(1000000.0, "MW/kg", "megawatts per kilogram")]
    MegawattPerKilogram,
    #[unit(1000.0, "kW/kg", "kilowatts per kilogram")]
    KilowattPerKilogram,
    #[unit(100.0, "hW/kg", "hectowatts per kilogram")]
    HectowattPerKilogram,
    #[unit(10.0, "daW/kg", "decawatts per kilogram")]
    DecawattPerKilogram,
    #[unit(1.0, "W/kg", "watts per kilogram")]
    WattPerKilogram,
    #[unit(0.10000000000000001, "dW/kg", "deciwatts per kilogram")]
    DeciwattPerKilogram,
    #[unit(0.01, "cW/kg", "centiwatts per kilogram")]
    CentiwattPerKilogram,
    #[unit(0.001, "mW/kg", "milliwatts per kilogram")]
    MilliwattPerKilogram,
    #[unit(9.9999999999999995e-07, "µW/kg", "microwatts per kilogram")]
    MicrowattPerKilogram,
    #[unit(1.0000000000000001e-09, "nW/kg", "nanowatts per kilogram")]
    NanowattPerKilogram,
    #[unit(9.9999999999999998e-13, "pW/kg", "picowatts per kilogram")]
    PicowattPerKilogram,
    #[unit(1.0000000000000001e-15, "fW/kg", "femtowatts per kilogram")]
    FemtowattPerKilogram,
    #[unit(1.0000000000000001e-18, "aW/kg", "attowatts per kilogram")]
    AttowattPerKilogram,
    #[unit(9.9999999999999991e-22, "zW/kg", "zeptowatts per kilogram")]
    ZeptowattPerKilogram,
    #[unit(9.9999999999999992e-25, "yW/kg", "yoctowatts per kilogram")]
    YoctowattPerKilogram,
    #[unit(1.0, "m² · s⁻³", "square meters per cubic second")]
    SquareMeterPerCubicSecond,
    #[unit(9.9999999999999995e-08, "(erg/s)/kg", "ergs per second per kilogram")]
    ErgPerSecondPerKilogram,
    #[unit(
        0.00083029634339356457,
        "(ft · lbf/h)/lb",
        "foot pounds-force per hour per pound"
    )]
    FootPoundPerHourPerPound,
    #[unit(
        0.049817780603613873,
        "(ft · lbf/min)/lb",
        "foot pounds-force per minute per pound"
    )]
    FootPoundPerMinutePerPound,
    #[unit(
        2.9890668362168324,
        "(ft · lbf/s)/lb",
        "foot pounds-force per second per pound"
    )]
    FootPoundPerSecondPerPound,
    #[unit(1643.9867599192578, "hp/lb", "horsepower per pound")]
    HorsepowerPerPound,
    #[unit(21626.244178694353, "(hp (S))/lb", "horsepower (boiler) per pound")]
    HorsepowerBoilerPerPound,
    #[unit(1644.6483671243168, "(hp (E))/lb", "horsepower (electric) per pound")]
    HorsepowerElectricPerPound,
    #[unit(1621.497185578947, "(hp (M))/lb", "metric horsepower per pound")]
    HorsepowerMetricPerPound,
    #[unit(1643.9869803815056, "(hp (I))/lb", "horsepower (Imperial) per pound")]
    HorsepowerImperialPerPound,
    #[unit(
        1644.7431658907865,
        "(hp (hydraulic))/lb",
        "hydraulic horsepower per pound"
    )]
    HydraulicHorsepowerPerPound,
}
