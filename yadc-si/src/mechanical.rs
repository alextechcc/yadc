use crate::{ISQ, StaticSIQuantity};
use yadc::{Dimension, Unit};

pub type Length = StaticSIQuantity<{ ISQ { m: 1, ..ISQ::ZERO } }>;

#[derive(Unit, Copy, Clone)]
#[dim(Length)]
pub enum LengthUnit {
    #[linear("Ym", "yottameters", 9.9999999999999998e+23)]
    Yottameter,
    #[linear("Zm", "zettameters", 1e+21)]
    Zettameter,
    #[linear("Em", "exameters", 1e+18)]
    Exameter,
    #[linear("Pm", "petameters", 1000000000000000.0)]
    Petameter,
    #[linear("Tm", "terameters", 1000000000000.0)]
    Terameter,
    #[linear("Gm", "gigameters", 1000000000.0)]
    Gigameter,
    #[linear("Mm", "megameters", 1000000.0)]
    Megameter,
    #[linear("km", "kilometers", 1000.0)]
    Kilometer,
    #[linear("hm", "hectometers", 100.0)]
    Hectometer,
    #[linear("dam", "decameters", 10.0)]
    Decameter,
    #[linear("m", "meters", 1.0)]
    Meter,
    #[linear("dm", "decimeters", 0.10000000000000001)]
    Decimeter,
    #[linear("cm", "centimeters", 0.01)]
    Centimeter,
    #[linear("mm", "millimeters", 0.001)]
    Millimeter,
    #[linear("µm", "micrometers", 9.9999999999999995e-07)]
    Micrometer,
    #[linear("nm", "nanometers", 1.0000000000000001e-09)]
    Nanometer,
    #[linear("pm", "picometers", 9.9999999999999998e-13)]
    Picometer,
    #[linear("fm", "femtometers", 1.0000000000000001e-15)]
    Femtometer,
    #[linear("am", "attometers", 1.0000000000000001e-18)]
    Attometer,
    #[linear("zm", "zeptometers", 9.9999999999999991e-22)]
    Zeptometer,
    #[linear("ym", "yoctometers", 9.9999999999999992e-25)]
    Yoctometer,
    #[linear("Å", "ångströms", 1e-10)]
    Angstrom,
    #[linear("a₀", "bohr radiuses", 5.2917721090299998e-11)]
    BohrRadius,
    #[linear("a.u. of length", "atomic units of length", 5.2917721090299998e-11)]
    AtomicUnitOfLength,
    #[linear("ua", "astronomical units", 149597870700.0)]
    AstronomicalUnit,
    #[linear("ch", "chains", 20.11684)]
    Chain,
    #[linear("DM", "data miles", 1828.8)]
    DataMile,
    #[linear("fathom", "fathoms", 1.8288040000000001)]
    Fathom,
    #[linear("fermi", "fermis", 1.0000000000000001e-15)]
    Fermi,
    #[linear("ft", "feet", 0.30480000000000002)]
    Foot,
    #[linear("ft (U.S. survey)", "feet (U.S. survey)", 0.30480059999999998)]
    FootSurvey,
    #[linear("in", "inches", 0.025399999999999999)]
    Inch,
    #[linear("l. y.", "light years", 9460730000000000.0)]
    LightYear,
    #[linear("μin", "microinches", 2.5399999999999999e-08)]
    Microinch,
    #[linear("μ", "microns", 9.9999999999999995e-07)]
    Micron,
    #[linear("0.001 in", "mils", 2.5400000000000001e-05)]
    Mil,
    #[linear("mi", "miles", 1609.3440000000001)]
    Mile,
    #[linear("mi (U.S. survey)", "miles (U.S. survey)", 1609.347)]
    MileSurvey,
    #[linear("M", "nautical miles", 1852.0)]
    NauticalMile,
    #[linear("pc", "parsecs", 30856780000000000.0)]
    Parsec,
    #[linear("1/6 in (computer)", "picas (computer)", 0.0042333333333333329)]
    PicaComputer,
    #[linear("1/6 in", "picas (printer's)", 0.004217518)]
    PicaPrinters,
    #[linear("1/72 in (computer)", "points (computer)", 0.00035277779999999998)]
    PointComputer,
    #[linear("1/72 in", "points (printer's)", 0.00035145979999999999)]
    PointPrinters,
    #[linear("rd", "rods", 5.02921)]
    Rod,
    #[linear("yd", "yards", 0.91439999999999999)]
    Yard,
    #[linear("leuga", "leugae", 2220.0)]
    Leuga,
    #[linear("mille passus", "mille passa", 1480.0)]
    MillePassus,
    #[linear("stadium", "stadia", 185.0)]
    Stadium,
    #[linear("actus", "acta", 35.520000000000003)]
    Actus,
    #[linear("decempeda", "decempedae", 2.96)]
    Decempeda,
    #[linear("passus", "passa", 1.48)]
    Passus,
    #[linear("gradus", "gradus", 0.73999999999999999)]
    Gradus,
    #[linear("cubitum", "cubita", 0.44400000000000001)]
    Cubitum,
    #[linear("palmipes", "palmipedes", 0.37)]
    Palmipes,
    #[linear("pes", "pedes", 0.29599999999999999)]
    Pes,
    #[linear("palmus maior", "palmas maior", 0.222)]
    PalmusMaior,
    #[linear("palmus", "palmas", 0.073999999999999996)]
    Palmus,
    #[linear("uncia", "unciae", 0.024666666666666663)]
    Uncia,
    #[linear("digitus", "digiti", 0.018499999999999999)]
    Digitus,
}

pub type Mass = StaticSIQuantity<{ ISQ { kg: 1, ..ISQ::ZERO } }>;

#[derive(Unit, Copy, Clone)]
#[dim(Mass)]
pub enum MassUnit {
    #[linear("Yg", "yottagrams", 1e+21)]
    Yottagram,
    #[linear("Zg", "zettagrams", 1e+18)]
    Zettagram,
    #[linear("Eg", "exagrams", 1000000000000000.0)]
    Exagram,
    #[linear("Pg", "petagrams", 1000000000000.0)]
    Petagram,
    #[linear("Tg", "teragrams", 1000000000.0)]
    Teragram,
    #[linear("Gg", "gigagrams", 1000000.0)]
    Gigagram,
    #[linear("Mg", "megagrams", 1000.0)]
    Megagram,
    #[linear("kg", "kilograms", 1.0)]
    Kilogram,
    #[linear("hg", "hectograms", 0.10000000000000001)]
    Hectogram,
    #[linear("dag", "decagrams", 0.01)]
    Decagram,
    #[linear("g", "grams", 0.001)]
    Gram,
    #[linear("dg", "decigrams", 0.0001)]
    Decigram,
    #[linear("cg", "centigrams", 1.0000000000000001e-05)]
    Centigram,
    #[linear("mg", "milligrams", 9.9999999999999995e-07)]
    Milligram,
    #[linear("µg", "micrograms", 9.9999999999999986e-10)]
    Microgram,
    #[linear("ng", "nanograms", 9.9999999999999998e-13)]
    Nanogram,
    #[linear("pg", "picograms", 1.0000000000000001e-15)]
    Picogram,
    #[linear("fg", "femtograms", 1.0000000000000001e-18)]
    Femtogram,
    #[linear("ag", "attograms", 1.0000000000000001e-21)]
    Attogram,
    #[linear("zg", "zeptograms", 9.9999999999999992e-25)]
    Zeptogram,
    #[linear("yg", "yoctograms", 9.9999999999999986e-28)]
    Yoctogram,
    #[linear("ct", "carats", 0.00020000000000000001)]
    Carat,
    #[linear("Da", "daltons", 1.6605390666e-27)]
    Dalton,
    #[linear("gr", "grains", 6.4798909999999995e-05)]
    Grain,
    #[linear("cwt long", "hundredweight (long)", 50.802349999999997)]
    HundredweightLong,
    #[linear("cwt short", "hundredweight (short)", 45.35924)]
    HundredweightShort,
    #[linear("oz", "ounces", 0.02834952)]
    Ounce,
    #[linear("oz t", "troy ounces", 0.031103479999999999)]
    OunceTroy,
    #[linear("dwt", "pennyweight", 0.0015551740000000001)]
    Pennyweight,
    #[linear("lb", "pounds", 0.45359240000000001)]
    Pound,
    #[linear("lb t", "troy pounds", 0.37324170000000001)]
    PoundTroy,
    #[linear("slug", "slugs", 14.5939)]
    Slug,
    #[linear("AT", "assay tons", 0.029166669999999999)]
    TonAssay,
    #[linear("2240 lb", "long tons", 1016.047)]
    TonLong,
    #[linear("2000 lb", "short tons", 907.18470000000002)]
    TonShort,
    #[linear("t", "tons", 1000.0)]
    Ton,
    #[linear("libra", "librae", 0.32890000000000003)]
    Libra,
    #[linear("deunx", "deunces", 0.30149166666666666)]
    Deunx,
    #[linear("dextans", "dextantes", 0.27408333333333335)]
    Dextans,
    #[linear("dodrans", "dodrantes", 0.24667500000000001)]
    Dodrans,
    #[linear("bes", "bessis", 0.21926666666666667)]
    Bes,
    #[linear("septunx", "septunces", 0.19185833333333335)]
    Septunx,
    #[linear("semis", "semisses", 0.16445000000000001)]
    Semis,
    #[linear("quincunx", "quincunx", 0.13704166666666667)]
    Quincunx,
    #[linear("triens", "trientes", 0.10963333333333333)]
    Triens,
    #[linear("quadrans teruncius", "quadrantes teruncius", 0.082225000000000006)]
    QuadransTeruncius,
    #[linear("sextans", "sextantes", 0.054816666666666701)]
    Sextans,
    #[linear("sescuncia", "sescunciae", 0.041112500000000003)]
    Sescuncia,
    #[linear("uncia", "unciae", 0.027408333333333337)]
    Uncia,
    #[linear("semuncia", "semunciae", 0.0137041666666667)]
    Semuncia,
    #[linear("duella", "duella", 0.0091361111111111105)]
    Duella,
    #[linear("sicilicus", "scilici", 0.0068520833333333298)]
    Sicilicus,
    #[linear("sextula", "sextulae", 0.0045680555555555596)]
    Sextula,
    #[linear("semisextula", "semisextulae", 0.0022840277777777798)]
    Semisextula,
    #[linear("scrupulum", "scrupula", 0.0011420138888888899)]
    Scrupulum,
    #[linear("obolus", "oboli", 0.00057100694444444397)]
    Obolus,
    #[linear("siliqua", "siliquae", 0.000190335648148148)]
    Siliqua,
}

pub type Time = StaticSIQuantity<{ ISQ { s: 1, ..ISQ::ZERO } }>;

#[derive(Unit, Copy, Clone)]
#[dim(Time)]
pub enum TimeUnit {
    #[linear("Ys", "yottaseconds", 9.9999999999999998e+23)]
    Yottasecond,
    #[linear("Zs", "zettaseconds", 1e+21)]
    Zettasecond,
    #[linear("Es", "exaseconds", 1e+18)]
    Exasecond,
    #[linear("Ps", "petaseconds", 1000000000000000.0)]
    Petasecond,
    #[linear("Ts", "teraseconds", 1000000000000.0)]
    Terasecond,
    #[linear("Gs", "gigaseconds", 1000000000.0)]
    Gigasecond,
    #[linear("Ms", "megaseconds", 1000000.0)]
    Megasecond,
    #[linear("ks", "kiloseconds", 1000.0)]
    Kilosecond,
    #[linear("hs", "hectoseconds", 100.0)]
    Hectosecond,
    #[linear("das", "decaseconds", 10.0)]
    Decasecond,
    #[linear("s", "seconds", 1.0)]
    Second,
    #[linear("ds", "deciseconds", 0.10000000000000001)]
    Decisecond,
    #[linear("cs", "centiseconds", 0.01)]
    Centisecond,
    #[linear("ms", "milliseconds", 0.001)]
    Millisecond,
    #[linear("µs", "microseconds", 9.9999999999999995e-07)]
    Microsecond,
    #[linear("ns", "nanoseconds", 1.0000000000000001e-09)]
    Nanosecond,
    #[linear("ps", "picoseconds", 9.9999999999999998e-13)]
    Picosecond,
    #[linear("fs", "femtoseconds", 1.0000000000000001e-15)]
    Femtosecond,
    #[linear("as", "attoseconds", 1.0000000000000001e-18)]
    Attosecond,
    #[linear("zs", "zeptoseconds", 9.9999999999999991e-22)]
    Zeptosecond,
    #[linear("ys", "yoctoseconds", 9.9999999999999992e-25)]
    Yoctosecond,
    #[linear("s (sidereal)", "seconds (sidereal)", 0.99726959999999998)]
    SecondSidereal,
    #[linear("d", "days", 86400.0)]
    Day,
    #[linear("d (sidereal)", "days (sidereal)", 86164.089999999997)]
    DaySidereal,
    #[linear("h", "hours", 3600.0)]
    Hour,
    #[linear("h (sidereal)", "hours (sidereal)", 3590.1700000000001)]
    HourSidereal,
    #[linear("min", "minutes", 60.0)]
    Minute,
    #[linear("10.0 ns", "shakes", 1e-08)]
    Shake,
    #[linear("a", "years", 31536000.0)]
    Year,
    #[linear("a (sidereal)", "years (sidereal)", 31558150.0)]
    YearSidereal,
    #[linear("a (tropical)", "years (tropical)", 31556930.0)]
    YearTropical,
}

pub type Area = StaticSIQuantity<{ ISQ { m: 2, ..ISQ::ZERO } }>;

#[derive(Unit, Copy, Clone)]
#[dim(Area)]
pub enum AreaUnit {
    #[linear("Ym²", "square yottameters", 1e+48)]
    SquareYottameter,
    #[linear("Zm²", "square zettameters", 1e+42)]
    SquareZettameter,
    #[linear("Em²", "square exameters", 1e+36)]
    SquareExameter,
    #[linear("Pm²", "square petameters", 1e+30)]
    SquarePetameter,
    #[linear("Tm²", "square terameters", 9.9999999999999998e+23)]
    SquareTerameter,
    #[linear("Gm²", "square gigameters", 1e+18)]
    SquareGigameter,
    #[linear("Mm²", "square megameters", 1000000000000.0)]
    SquareMegameter,
    #[linear("km²", "square kilometers", 1000000.0)]
    SquareKilometer,
    #[linear("hm²", "square hectometers", 10000.0)]
    SquareHectometer,
    #[linear("dam²", "square decameters", 100.0)]
    SquareDecameter,
    #[linear("m²", "square meters", 1.0)]
    SquareMeter,
    #[linear("dm²", "square decimeters", 0.010000000000000002)]
    SquareDecimeter,
    #[linear("cm²", "square centimeters", 0.0001)]
    SquareCentimeter,
    #[linear("mm²", "square millimeters", 9.9999999999999995e-07)]
    SquareMillimeter,
    #[linear("µm²", "square micrometers", 9.9999999999999998e-13)]
    SquareMicrometer,
    #[linear("nm²", "square nanometers", 1.0000000000000001e-18)]
    SquareNanometer,
    #[linear("pm²", "square picometers", 9.9999999999999992e-25)]
    SquarePicometer,
    #[linear("fm²", "square femtometers", 1.0000000000000001e-30)]
    SquareFemtometer,
    #[linear("am²", "square attometers", 1.0000000000000001e-36)]
    SquareAttometer,
    #[linear("zm²", "square zeptometers", 9.9999999999999988e-43)]
    SquareZeptometer,
    #[linear("ym²", "square yoctometers", 9.9999999999999982e-49)]
    SquareYoctometer,
    #[linear("ac", "acres", 4046.873)]
    Acre,
    #[linear("a", "ares", 100.0)]
    Are,
    #[linear("b", "barns", 9.9999999999999997e-29)]
    Barn,
    #[linear("cmil", "circular mils", 5.0670750000000003e-10)]
    CircularMil,
    #[linear("ha", "hectares", 10000.0)]
    Hectare,
    #[linear("ft²", "square feet", 0.092903040000000006)]
    SquareFoot,
    #[linear("in²", "square inches", 0.00064515999999999998)]
    SquareInch,
    #[linear("mi²", "square miles", 2589988.1103360001)]
    SquareMile,
    #[linear("yd²", "square yards", 0.83612735999999999)]
    SquareYard,
    #[linear("saltus", "saltus", 2018672.6399999999)]
    Saltus,
    #[linear("centuria", "centuriae", 504668.15999999997)]
    Centuria,
    #[linear("heredium", "heredia", 5046.6815999999999)]
    Heredium,
    #[linear("jugerum", "jugera", 2523.3407999999999)]
    Jugerum,
    #[linear("deunx", "deunces", 2313.0623999999998)]
    Deunx,
    #[linear("dextans", "dextantes", 2102.7840000000001)]
    Dextans,
    #[linear("dodrans", "dodrantes", 1892.5056)]
    Dodrans,
    #[linear("bes", "besses", 1682.2272)]
    Bes,
    #[linear("septunx", "septunces", 1471.9487999999999)]
    Septunx,
    #[linear("semis", "semisses", 1261.6704)]
    Semis,
    #[linear("actus quadratus", "acta quadratus", 1261.6704)]
    ActusQuadratus,
    #[linear("quincunx", "quincunces", 1051.3920000000001)]
    Quincunx,
    #[linear("triens", "trientes", 841.11360000000002)]
    Triens,
    #[linear("quadrans", "quadrantes", 630.83519999999999)]
    Quadrans,
    #[linear("sextans", "sextantes", 420.55680000000001)]
    Sextans,
    #[linear("clima", "climata", 315.41759999999999)]
    Clima,
    #[linear("uncia", "unciae", 210.2784)]
    Uncia,
    #[linear("semiuncia", "semiunciae", 105.1392)]
    Semiuncia,
    #[linear("sicilicus", "siculi", 52.569600000000001)]
    Sicilicus,
    #[linear("actus simplex", "acta simplex", 42.055680000000002)]
    ActusSimplex,
    #[linear("sextula", "sextulae", 35.046399999999998)]
    Sextula,
    #[linear("duo scrupulum", "duo scrupula", 17.523199999999999)]
    DuoScrupulum,
    #[linear("scrupulum", "scrupula", 8.7615999999999996)]
    Scrupulum,
    #[linear("dimidium scrupulum", "dimidium scupula", 4.3807999999999998)]
    DimidiumScrupulum,
    #[linear("pes quadratus", "pedes quadratus", 0.087615999999999999)]
    PesQuadratus,
}

pub type Volume = StaticSIQuantity<{ ISQ { m: 3, ..ISQ::ZERO } }>;

#[derive(Unit, Copy, Clone)]
#[dim(Volume)]
pub enum VolumeUnit {
    #[linear("m³", "cubic meters", 1.0)]
    CubicMeter,
    #[linear("ac · ft", "acre-feet", 1233.489)]
    AcreFoot,
    #[linear("bbl", "barrels", 0.1589873)]
    Barrel,
    #[linear("bu", "bushels", 0.035239069999999997)]
    Bushel,
    #[linear("cords", "cords", 3.6245560000000001)]
    Cord,
    #[linear("ft³", "cubic feet", 0.028316850000000001)]
    CubicFoot,
    #[linear("in³", "cubic inches", 1.6387060000000002e-05)]
    CubicInch,
    #[linear("mi³", "cubic miles", 4168182000.0)]
    CubicMile,
    #[linear("yd³", "cubic yards", 0.76455490000000004)]
    CubicYard,
    #[linear("cup", "cups", 0.00023658819999999999)]
    Cup,
    #[linear("fl oz", "fluid ounces", 2.9573529999999999e-05)]
    FluidOunce,
    #[linear("fl oz (UK)", "Imperial fluid ounces", 2.8413060000000001e-05)]
    FluidOunceImperial,
    #[linear("gal (UK)", "Imperial gallons", 0.00454609)]
    GallonImperial,
    #[linear("gal", "gallons", 0.0037854120000000002)]
    Gallon,
    #[linear("gi (UK)", "Imperial gills", 0.0001420653)]
    GillImperial,
    #[linear("gi", "gills", 0.0001182941)]
    Gill,
    #[linear("YL", "yottaliters", 1e+21)]
    Yottaliter,
    #[linear("ZL", "zettaliters", 1e+18)]
    Zettaliter,
    #[linear("EL", "exaliters", 1000000000000000.0)]
    Exaliter,
    #[linear("PL", "petaliters", 1000000000000.0)]
    Petaliter,
    #[linear("TL", "teraliters", 1000000000.0)]
    Teraliter,
    #[linear("GL", "gigaliters", 1000000.0)]
    Gigaliter,
    #[linear("ML", "megaliters", 1000.0)]
    Megaliter,
    #[linear("kL", "kiloliters", 1.0)]
    Kiloliter,
    #[linear("hL", "hectoliters", 0.10000000000000001)]
    Hectoliter,
    #[linear("daL", "decaliters", 0.01)]
    Decaliter,
    #[linear("L", "liters", 0.001)]
    Liter,
    #[linear("dL", "deciliters", 0.0001)]
    Deciliter,
    #[linear("cL", "centiliters", 1.0000000000000001e-05)]
    Centiliter,
    #[linear("mL", "milliliters", 9.9999999999999995e-07)]
    Milliliter,
    #[linear("µL", "microliters", 1.0000000000000001e-09)]
    Microliter,
    #[linear("nL", "nanoliters", 1.0000000000000002e-12)]
    Nanoliter,
    #[linear("pL", "picoliters", 1.0000000000000001e-15)]
    Picoliter,
    #[linear("fL", "femtoliters", 1.0000000000000001e-18)]
    Femtoliter,
    #[linear("aL", "attoliters", 1.0000000000000001e-21)]
    Attoliter,
    #[linear("zL", "zeptoliters", 9.9999999999999992e-25)]
    Zeptoliter,
    #[linear("yL", "yoctoliters", 9.9999999999999986e-28)]
    Yoctoliter,
    #[linear("pk", "pecks", 0.0088097680000000008)]
    Peck,
    #[linear("dry pt", "dry pints", 0.00055061050000000005)]
    PintDry,
    #[linear("liq pt", "liquid pints", 0.00047317650000000002)]
    PintLiquid,
    #[linear("dry qt", "dry quarts", 0.0011012210000000001)]
    QuartDry,
    #[linear("liq qt", "liquid quarts", 0.0009463529)]
    QuartLiquid,
    #[linear("st", "steres", 1.0)]
    Stere,
    #[linear("tbsp", "tablespoons", 1.4786759999999999e-05)]
    Tablespoon,
    #[linear("tsp", "teaspoons", 4.9289219999999997e-06)]
    Teaspoon,
    #[linear("RT", "register tons", 2.8316849999999998)]
    RegisterTon,
    #[linear("culeus", "culei", 0.51868672000000005)]
    Culeus,
    #[linear("amphora quadrantal", "amphora quadrantal", 0.025934335999999999)]
    AmphoraQuadrantal,
    #[linear("urna", "urnae", 0.012967167999999999)]
    Urna,
    #[linear("modius castrensis", "modii castrensis", 0.012967167999999999)]
    ModiusCastrensis,
    #[linear("modius", "modii", 0.0086447786666666599)]
    Modius,
    #[linear("semimodius", "semimodii", 0.00432238933333333)]
    Semimodius,
    #[linear("congius", "congii", 0.0032417919999999999)]
    Congius,
    #[linear("sextarius", "sextarii", 0.000540298666666667)]
    Sextarius,
    #[linear("hemina", "heminae", 0.00027014933333333301)]
    Hemina,
    #[linear("quartarius", "quartarii", 0.00013507466666666699)]
    Quartarius,
    #[linear("acetabulum", "acetabula", 6.7537333333333294e-05)]
    Acetabulum,
    #[linear("cyathus", "cyathi", 4.5024888888888901e-05)]
    Cyathus,
    #[linear("ligula", "ligula", 1.1256222222222222e-05)]
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

#[derive(Unit, Copy, Clone)]
#[dim(Velocity)]
pub enum VelocityUnit {
    #[linear("Ym/s", "yottameters per second", 9.9999999999999998e+23)]
    YottameterPerSecond,
    #[linear("Zm/s", "zettameters per second", 1e+21)]
    ZettameterPerSecond,
    #[linear("Em/s", "exameters per second", 1e+18)]
    ExameterPerSecond,
    #[linear("Pm/s", "petameters per second", 1000000000000000.0)]
    PetameterPerSecond,
    #[linear("Tm/s", "terameters per second", 1000000000000.0)]
    TerameterPerSecond,
    #[linear("Gm/s", "gigameters per second", 1000000000.0)]
    GigameterPerSecond,
    #[linear("Mm/s", "megameters per second", 1000000.0)]
    MegameterPerSecond,
    #[linear("km/s", "kilometers per second", 1000.0)]
    KilometerPerSecond,
    #[linear("hm/s", "hectometers per second", 100.0)]
    HectometerPerSecond,
    #[linear("dam/s", "decameters per second", 10.0)]
    DecameterPerSecond,
    #[linear("m/s", "meters per second", 1.0)]
    MeterPerSecond,
    #[linear("dm/s", "decimeters per second", 0.10000000000000001)]
    DecimeterPerSecond,
    #[linear("cm/s", "centimeters per second", 0.01)]
    CentimeterPerSecond,
    #[linear("mm/s", "millimeters per second", 0.001)]
    MillimeterPerSecond,
    #[linear("µm/s", "micrometers per second", 9.9999999999999995e-07)]
    MicrometerPerSecond,
    #[linear("nm/s", "nanometers per second", 1.0000000000000001e-09)]
    NanometerPerSecond,
    #[linear("pm/s", "picometers per second", 9.9999999999999998e-13)]
    PicometerPerSecond,
    #[linear("fm/s", "femtometers per second", 1.0000000000000001e-15)]
    FemtometerPerSecond,
    #[linear("am/s", "attometers per second", 1.0000000000000001e-18)]
    AttometerPerSecond,
    #[linear("zm/s", "zeptometers per second", 9.9999999999999991e-22)]
    ZeptometerPerSecond,
    #[linear("ym/s", "yoctometers per second", 9.9999999999999992e-25)]
    YoctometerPerSecond,
    #[linear("ft/h", "feet per hour", 8.4666666666666674e-05)]
    FootPerHour,
    #[linear("ft/min", "feet per minute", 0.0050800000000000003)]
    FootPerMinute,
    #[linear("ft/s", "feet per second", 0.30480000000000002)]
    FootPerSecond,
    #[linear("in/s", "inches per second", 0.025399999999999999)]
    InchPerSecond,
    #[linear("in/min", "inches per minute", 0.00042333333333333334)]
    InchPerMinute,
    #[linear("km/h", "kilometers per hour", 0.27777777777777779)]
    KilometerPerHour,
    #[linear("kn", "knots", 0.51444444444444448)]
    Knot,
    #[linear("mi/h", "miles per hour", 0.44703999999999999)]
    MilePerHour,
    #[linear("mi/min", "miles per minute", 26.822399999999998)]
    MilePerMinute,
    #[linear("mi/s", "miles per second", 1609.3440000000001)]
    MilePerSecond,
    #[linear("mm/min", "millimeters per minute", 1.6666666666666667e-05)]
    MillimeterPerMinute,
    #[linear("a₀ · Eₕ/ħ", "atomic units of velocity", 2187691.26364)]
    AtomicUnitOfVelocity,
    #[linear("c", "natural units of velocity", 299792458.0)]
    NaturalUnitOfVelocity,
    #[linear("c", "speeds of light in vacuum", 299792458.0)]
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

#[derive(Unit, Copy, Clone)]
#[dim(Acceleration)]
pub enum AccelerationUnit {
    #[linear("Ym/s²", "yottameters per second squared", 9.9999999999999998e+23)]
    YottameterPerSecondSquared,
    #[linear("Zm/s²", "zettameters per second squared", 1e+21)]
    ZettameterPerSecondSquared,
    #[linear("Em/s²", "exameters per second squared", 1e+18)]
    ExameterPerSecondSquared,
    #[linear("Pm/s²", "petameters per second squared", 1000000000000000.0)]
    PetameterPerSecondSquared,
    #[linear("Tm/s²", "terameters per second squared", 1000000000000.0)]
    TerameterPerSecondSquared,
    #[linear("Gm/s²", "gigameters per second squared", 1000000000.0)]
    GigameterPerSecondSquared,
    #[linear("Mm/s²", "megameters per second squared", 1000000.0)]
    MegameterPerSecondSquared,
    #[linear("km/s²", "kilometers per second squared", 1000.0)]
    KilometerPerSecondSquared,
    #[linear("hm/s²", "hectometers per second squared", 100.0)]
    HectometerPerSecondSquared,
    #[linear("dam/s²", "decameters per second squared", 10.0)]
    DecameterPerSecondSquared,
    #[linear("m/s²", "meters per second squared", 1.0)]
    MeterPerSecondSquared,
    #[linear("dm/s²", "decimeters per second squared", 0.10000000000000001)]
    DecimeterPerSecondSquared,
    #[linear("cm/s²", "centimeters per second squared", 0.01)]
    CentimeterPerSecondSquared,
    #[linear("mm/s²", "millimeters per second squared", 0.001)]
    MillimeterPerSecondSquared,
    #[linear("µm/s²", "micrometers per second squared", 9.9999999999999995e-07)]
    MicrometerPerSecondSquared,
    #[linear("nm/s²", "nanometers per second squared", 1.0000000000000001e-09)]
    NanometerPerSecondSquared,
    #[linear("pm/s²", "picometers per second squared", 9.9999999999999998e-13)]
    PicometerPerSecondSquared,
    #[linear("fm/s²", "femtometers per second squared", 1.0000000000000001e-15)]
    FemtometerPerSecondSquared,
    #[linear("am/s²", "attometers per second squared", 1.0000000000000001e-18)]
    AttometerPerSecondSquared,
    #[linear("zm/s²", "zeptometers per second squared", 9.9999999999999991e-22)]
    ZeptometerPerSecondSquared,
    #[linear("ym/s²", "yoctometers per second squared", 9.9999999999999992e-25)]
    YoctometerPerSecondSquared,
    #[linear("ft/s²", "feet per second squared", 0.30480000000000002)]
    FootPerSecondSquared,
    #[linear("Gal", "galileos", 0.01)]
    Galileo,
    #[linear("in/s²", "inches per second squared", 0.025399999999999999)]
    InchPerSecondSquared,
    #[linear("mm/min²", "millimeters per minute squared", 2.7777777777777776e-07)]
    MillimeterPerMinuteSquared,
    #[linear("g₀", "standard accelerations of gravity", 9.8066499999999994)]
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

#[derive(Unit, Copy, Clone)]
#[dim(Jerk)]
pub enum JerkUnit {
    #[linear("Ym/s³", "yottameters per second cubed", 9.9999999999999998e+23)]
    YottameterPerSecondCubed,
    #[linear("Zm/s³", "zettameters per second cubed", 1e+21)]
    ZettameterPerSecondCubed,
    #[linear("Em/s³", "exameters per second cubed", 1e+18)]
    ExameterPerSecondCubed,
    #[linear("Pm/s³", "petameters per second cubed", 1000000000000000.0)]
    PetameterPerSecondCubed,
    #[linear("Tm/s³", "terameters per second cubed", 1000000000000.0)]
    TerameterPerSecondCubed,
    #[linear("Gm/s³", "gigameters per second cubed", 1000000000.0)]
    GigameterPerSecondCubed,
    #[linear("Mm/s³", "megameters per second cubed", 1000000.0)]
    MegameterPerSecondCubed,
    #[linear("km/s³", "kilometers per second cubed", 1000.0)]
    KilometerPerSecondCubed,
    #[linear("hm/s³", "hectometers per second cubed", 100.0)]
    HectometerPerSecondCubed,
    #[linear("dam/s³", "decameters per second cubed", 10.0)]
    DecameterPerSecondCubed,
    #[linear("m/s³", "meters per second cubed", 1.0)]
    MeterPerSecondCubed,
    #[linear("dm/s³", "decimeters per second cubed", 0.10000000000000001)]
    DecimeterPerSecondCubed,
    #[linear("cm/s³", "centimeters per second cubed", 0.01)]
    CentimeterPerSecondCubed,
    #[linear("mm/s³", "millimeters per second cubed", 0.001)]
    MillimeterPerSecondCubed,
    #[linear("µm/s³", "micrometers per second cubed", 9.9999999999999995e-07)]
    MicrometerPerSecondCubed,
    #[linear("nm/s³", "nanometers per second cubed", 1.0000000000000001e-09)]
    NanometerPerSecondCubed,
    #[linear("pm/s³", "picometers per second cubed", 9.9999999999999998e-13)]
    PicometerPerSecondCubed,
    #[linear("fm/s³", "femtometers per second cubed", 1.0000000000000001e-15)]
    FemtometerPerSecondCubed,
    #[linear("am/s³", "attometers per second cubed", 1.0000000000000001e-18)]
    AttometerPerSecondCubed,
    #[linear("zm/s³", "zeptometers per second cubed", 9.9999999999999991e-22)]
    ZeptometerPerSecondCubed,
    #[linear("ym/s³", "yoctometers per second cubed", 9.9999999999999992e-25)]
    YoctometerPerSecondCubed,
    #[linear("ft/s³", "feet per second cubed", 0.30480000000000002)]
    FootPerSecondCubed,
    #[linear("in/s³", "inches per second cubed", 0.025399999999999999)]
    InchPerSecondCubed,
    #[linear("km/min³", "kilometers per minute cubed", 0.0046296296296296294)]
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

#[derive(Unit, Copy, Clone)]
#[dim(Absement)]
pub enum AbsementUnit {
    #[linear("Ym · s", "yottameter seconds", 9.9999999999999998e+23)]
    YottameterSecond,
    #[linear("Zm · s", "zettameter seconds", 1e+21)]
    ZettameterSecond,
    #[linear("Em · s", "exameter seconds", 1e+18)]
    ExameterSecond,
    #[linear("Pm · s", "petameter seconds", 1000000000000000.0)]
    PetameterSecond,
    #[linear("Tm · s", "terameter seconds", 1000000000000.0)]
    TerameterSecond,
    #[linear("Gm · s", "gigameter seconds", 1000000000.0)]
    GigameterSecond,
    #[linear("Mm · s", "megameter seconds", 1000000.0)]
    MegameterSecond,
    #[linear("km · s", "kilometer seconds", 1000.0)]
    KilometerSecond,
    #[linear("hm · s", "hectometer seconds", 100.0)]
    HectometerSecond,
    #[linear("dam · s", "decameter seconds", 10.0)]
    DecameterSecond,
    #[linear("m · s", "meter seconds", 1.0)]
    MeterSecond,
    #[linear("dm · s", "decimeter seconds", 0.10000000000000001)]
    DecimeterSecond,
    #[linear("cm · s", "centimeter seconds", 0.01)]
    CentimeterSecond,
    #[linear("mm · s", "millimeter seconds", 0.001)]
    MillimeterSecond,
    #[linear("µm · s", "micrometer seconds", 9.9999999999999995e-07)]
    MicrometerSecond,
    #[linear("nm · s", "nanometer seconds", 1.0000000000000001e-09)]
    NanometerSecond,
    #[linear("pm · s", "picometer seconds", 9.9999999999999998e-13)]
    PicometerSecond,
    #[linear("fm · s", "femtometer seconds", 1.0000000000000001e-15)]
    FemtometerSecond,
    #[linear("am · s", "attometer seconds", 1.0000000000000001e-18)]
    AttometerSecond,
    #[linear("zm · s", "zeptometer seconds", 9.9999999999999991e-22)]
    ZeptometerSecond,
    #[linear("ym · s", "yoctometer seconds", 9.9999999999999992e-25)]
    YoctometerSecond,
    #[linear("ft · s", "foot seconds", 0.30480000000000002)]
    FootSecond,
    #[linear("in · s", "inch seconds", 0.025399999999999999)]
    InchSecond,
    #[linear("km · h", "kilometer hours", 3600000.0)]
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

#[derive(Unit, Copy, Clone)]
#[dim(InverseVelocity)]
pub enum InverseVelocityUnit {
    #[linear("Ys/m", "yottaseconds per meter", 9.9999999999999998e+23)]
    YottasecondPerMeter,
    #[linear("Zs/m", "zettaseconds per meter", 1e+21)]
    ZettasecondPerMeter,
    #[linear("Es/m", "exaseconds per meter", 1e+18)]
    ExasecondPerMeter,
    #[linear("Ps/m", "petaseconds per meter", 1000000000000000.0)]
    PetasecondPerMeter,
    #[linear("Ts/m", "teraseconds per meter", 1000000000000.0)]
    TerasecondPerMeter,
    #[linear("Gs/m", "gigaseconds per meter", 1000000000.0)]
    GigasecondPerMeter,
    #[linear("Ms/m", "megaseconds per meter", 1000000.0)]
    MegasecondPerMeter,
    #[linear("ks/m", "kiloseconds per meter", 1000.0)]
    KilosecondPerMeter,
    #[linear("hs/m", "hectoseconds per meter", 100.0)]
    HectosecondPerMeter,
    #[linear("das/m", "decaseconds per meter", 10.0)]
    DecasecondPerMeter,
    #[linear("s/m", "seconds per meter", 1.0)]
    SecondPerMeter,
    #[linear("ds/m", "deciseconds per meter", 0.10000000000000001)]
    DecisecondPerMeter,
    #[linear("cs/m", "centiseconds per meter", 0.01)]
    CentisecondPerMeter,
    #[linear("ms/m", "milliseconds per meter", 0.001)]
    MillisecondPerMeter,
    #[linear("µs/m", "microseconds per meter", 9.9999999999999995e-07)]
    MicrosecondPerMeter,
    #[linear("ns/m", "nanoseconds per meter", 1.0000000000000001e-09)]
    NanosecondPerMeter,
    #[linear("ps/m", "picoseconds per meter", 9.9999999999999998e-13)]
    PicosecondPerMeter,
    #[linear("fs/m", "femtoseconds per meter", 1.0000000000000001e-15)]
    FemtosecondPerMeter,
    #[linear("as/m", "attoseconds per meter", 1.0000000000000001e-18)]
    AttosecondPerMeter,
    #[linear("zs/m", "zeptoseconds per meter", 9.9999999999999991e-22)]
    ZeptosecondPerMeter,
    #[linear("ys/m", "yoctoseconds per meter", 9.9999999999999992e-25)]
    YoctosecondPerMeter,
    #[linear("h/ft", "hours per foot", 11811.023622047243)]
    HourPerFoot,
    #[linear("min/ft", "minutes per foot", 196.85039370078738)]
    MinutePerFoot,
    #[linear("s/ft", "seconds per foot", 3.280839895013123)]
    SecondPerFoot,
    #[linear("s/in", "seconds per inch", 39.370078740157481)]
    SecondPerInch,
    #[linear("min/km", "minutes per kilometer", 0.059999999999999998)]
    MinutePerKilometer,
    #[linear("h/km", "hours per kilometer", 3.6000000000000001)]
    HourPerKilometer,
    #[linear("h/mi", "hours per mile", 2.236936292054402)]
    HourPerMile,
    #[linear("min/mi", "minutes per mile", 0.037282271530000001)]
    MinutePerMile,
    #[linear("s/mi", "seconds per mile", 0.00062137119223733392)]
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

#[derive(Unit, Copy, Clone)]
#[dim(Force)]
pub enum ForceUnit {
    #[linear("YN", "yottanewtons", 9.9999999999999998e+23)]
    Yottanewton,
    #[linear("ZN", "zettanewtons", 1e+21)]
    Zettanewton,
    #[linear("EN", "exanewtons", 1e+18)]
    Exanewton,
    #[linear("PN", "petanewtons", 1000000000000000.0)]
    Petanewton,
    #[linear("TN", "teranewtons", 1000000000000.0)]
    Teranewton,
    #[linear("GN", "giganewtons", 1000000000.0)]
    Giganewton,
    #[linear("MN", "meganewtons", 1000000.0)]
    Meganewton,
    #[linear("kN", "kilonewtons", 1000.0)]
    Kilonewton,
    #[linear("hN", "hectonewtons", 100.0)]
    Hectonewton,
    #[linear("daN", "decanewtons", 10.0)]
    Decanewton,
    #[linear("N", "newtons", 1.0)]
    Newton,
    #[linear("dN", "decinewtons", 0.10000000000000001)]
    Decinewton,
    #[linear("cN", "centinewtons", 0.01)]
    Centinewton,
    #[linear("mN", "millinewtons", 0.001)]
    Millinewton,
    #[linear("µN", "micronewtons", 9.9999999999999995e-07)]
    Micronewton,
    #[linear("nN", "nanonewtons", 1.0000000000000001e-09)]
    Nanonewton,
    #[linear("pN", "piconewtons", 9.9999999999999998e-13)]
    Piconewton,
    #[linear("fN", "femtonewtons", 1.0000000000000001e-15)]
    Femtonewton,
    #[linear("aN", "attonewtons", 1.0000000000000001e-18)]
    Attonewton,
    #[linear("zN", "zeptonewtons", 9.9999999999999991e-22)]
    Zeptonewton,
    #[linear("yN", "yoctonewtons", 9.9999999999999992e-25)]
    Yoctonewton,
    #[linear("dyn", "dynes", 1.0000000000000001e-05)]
    Dyne,
    #[linear("kgf", "kilograms-force", 9.8066499999999994)]
    KilogramForce,
    #[linear("kip", "kips", 4448.2219999999998)]
    Kip,
    #[linear("ozf", "ounces-force", 0.27801389999999998)]
    OunceForce,
    #[linear("pdl", "poundals", 0.13825499999999999)]
    Poundal,
    #[linear("lbf", "pounds-force", 4.4482220000000003)]
    PoundForce,
    #[linear("2000 lbf", "tons-force", 8896.4429999999993)]
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

#[derive(Unit, Copy, Clone)]
#[dim(Momentum)]
pub enum MomentumUnit {
    #[linear("Yg · m/s", "yottagram meters per second", 1e+21)]
    YottagramMeterPerSecond,
    #[linear("Zg · m/s", "zettagram meters per second", 1e+18)]
    ZettagramMeterPerSecond,
    #[linear("Eg · m/s", "exagram meters per second", 1000000000000000.0)]
    ExagramMeterPerSecond,
    #[linear("Pg · m/s", "petagram meters per second", 1000000000000.0)]
    PetagramMeterPerSecond,
    #[linear("Tg · m/s", "teragram meters per second", 1000000000.0)]
    TeragramMeterPerSecond,
    #[linear("Gg · m/s", "gigagram meters per second", 1000000.0)]
    GigagramMeterPerSecond,
    #[linear("Mg · m/s", "megagram meters per second", 1000.0)]
    MegagramMeterPerSecond,
    #[linear("kg · m/s", "kilogram meters per second", 1.0)]
    KilogramMeterPerSecond,
    #[linear("hg · m/s", "hectogram meters per second", 0.10000000000000001)]
    HectogramMeterPerSecond,
    #[linear("dag · m/s", "decagram meters per second", 0.01)]
    DecagramMeterPerSecond,
    #[linear("g · m/s", "gram meters per second", 0.001)]
    GramMeterPerSecond,
    #[linear("dg · m/s", "decigram meters per second", 0.0001)]
    DecigramMeterPerSecond,
    #[linear("cg · m/s", "centigram meters per second", 1.0000000000000001e-05)]
    CentigramMeterPerSecond,
    #[linear("mg · m/s", "milligram meters per second", 9.9999999999999995e-07)]
    MilligramMeterPerSecond,
    #[linear("µg · m/s", "microgram meters per second", 9.9999999999999986e-10)]
    MicrogramMeterPerSecond,
    #[linear("ng · m/s", "nanogram meters per second", 9.9999999999999998e-13)]
    NanogramMeterPerSecond,
    #[linear("pg · m/s", "picogram meters per second", 1.0000000000000001e-15)]
    PicogramMeterPerSecond,
    #[linear("fg · m/s", "femtogram meters per second", 1.0000000000000001e-18)]
    FemtogramMeterPerSecond,
    #[linear("ag · m/s", "attogram meters per second", 1.0000000000000001e-21)]
    AttogramMeterPerSecond,
    #[linear("zg · m/s", "zeptogram meters per second", 9.9999999999999992e-25)]
    ZeptogramMeterPerSecond,
    #[linear("yg · m/s", "yoctogram meters per second", 9.9999999999999986e-28)]
    YoctogramMeterPerSecond,
    #[linear(
        "kg · Ym/s",
        "kilogram yottameters per second",
        9.9999999999999998e+23
    )]
    KilogramYottameterPerSecond,
    #[linear("kg · Zm/s", "kilogram zettameters per second", 1e+21)]
    KilogramZettameterPerSecond,
    #[linear("kg · Em/s", "kilogram exameters per second", 1e+18)]
    KilogramExameterPerSecond,
    #[linear("kg · Pm/s", "kilogram petameters per second", 1000000000000000.0)]
    KilogramPetameterPerSecond,
    #[linear("kg · Tm/s", "kilogram terameters per second", 1000000000000.0)]
    KilogramTerameterPerSecond,
    #[linear("kg · Gm/s", "kilogram gigameters per second", 1000000000.0)]
    KilogramGigameterPerSecond,
    #[linear("kg · Mm/s", "kilogram megameters per second", 1000000.0)]
    KilogramMegameterPerSecond,
    #[linear("kg · km/s", "kilogram kilometers per second", 1000.0)]
    KilogramKilometerPerSecond,
    #[linear("kg · hm/s", "kilogram hectometers per second", 100.0)]
    KilogramHectometerPerSecond,
    #[linear("kg · dam/s", "kilogram decameters per second", 10.0)]
    KilogramDecameterPerSecond,
    #[linear("kg · dm/s", "kilogram decimeters per second", 0.10000000000000001)]
    KilogramDecimeterPerSecond,
    #[linear("kg · cm/s", "kilogram centimeters per second", 0.01)]
    KilogramCentimeterPerSecond,
    #[linear("kg · mm/s", "kilogram millimeters per second", 0.001)]
    KilogramMillimeterPerSecond,
    #[linear(
        "kg · µm/s",
        "kilogram micrometers per second",
        9.9999999999999995e-07
    )]
    KilogramMicrometerPerSecond,
    #[linear(
        "kg · nm/s",
        "kilogram nanometers per second",
        1.0000000000000001e-09
    )]
    KilogramNanometerPerSecond,
    #[linear(
        "kg · pm/s",
        "kilogram picometers per second",
        9.9999999999999998e-13
    )]
    KilogramPicometerPerSecond,
    #[linear(
        "kg · fm/s",
        "kilogram femtometers per second",
        1.0000000000000001e-15
    )]
    KilogramFemtometerPerSecond,
    #[linear(
        "kg · am/s",
        "kilogram attometers per second",
        1.0000000000000001e-18
    )]
    KilogramAttometerPerSecond,
    #[linear(
        "kg · zm/s",
        "kilogram zeptometers per second",
        9.9999999999999991e-22
    )]
    KilogramZeptometerPerSecond,
    #[linear(
        "kg · ym/s",
        "kilogram yoctometers per second",
        9.9999999999999992e-25
    )]
    KilogramYoctometerPerSecond,
    #[linear("t · m/s", "ton meters per second", 1000.0)]
    TonMeterPerSecond,
    #[linear("kg · m/min", "kilogram meters per minute", 0.016666666666666666)]
    KilogramMeterPerMinute,
    #[linear("kg · m/h", "kilogram meters per hour", 0.00027777777777777778)]
    KilogramMeterPerHour,
    #[linear("kg · m/d", "kilogram meters per day", 1.1574074074074073e-05)]
    KilogramMeterPerDay,
    #[linear("slug · ft/s", "slug feet per second", 4.4482207200000001)]
    SlugFootPerSecond,
    #[linear("slug · in/s", "slug inches per second", 0.37068506000000001)]
    SlugInchPerSecond,
    #[linear("lb · ft/s", "pound feet per second", 0.13825496352)]
    PoundFootPerSecond,
    #[linear("lb · in/s", "pound inches per second", 0.01152124696)]
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

#[derive(Unit, Copy, Clone)]
#[dim(Energy)]
pub enum EnergyUnit {
    #[linear("YJ", "yottajoules", 9.9999999999999998e+23)]
    Yottajoule,
    #[linear("ZJ", "zettajoules", 1e+21)]
    Zettajoule,
    #[linear("EJ", "exajoules", 1e+18)]
    Exajoule,
    #[linear("PJ", "petajoules", 1000000000000000.0)]
    Petajoule,
    #[linear("TJ", "terajoules", 1000000000000.0)]
    Terajoule,
    #[linear("GJ", "gigajoules", 1000000000.0)]
    Gigajoule,
    #[linear("MJ", "megajoules", 1000000.0)]
    Megajoule,
    #[linear("kJ", "kilojoules", 1000.0)]
    Kilojoule,
    #[linear("hJ", "hectojoules", 100.0)]
    Hectojoule,
    #[linear("daJ", "decajoules", 10.0)]
    Decajoule,
    #[linear("J", "joules", 1.0)]
    Joule,
    #[linear("dJ", "decijoules", 0.10000000000000001)]
    Decijoule,
    #[linear("cJ", "centijoules", 0.01)]
    Centijoule,
    #[linear("mJ", "millijoules", 0.001)]
    Millijoule,
    #[linear("µJ", "microjoules", 9.9999999999999995e-07)]
    Microjoule,
    #[linear("nJ", "nanojoules", 1.0000000000000001e-09)]
    Nanojoule,
    #[linear("pJ", "picojoules", 9.9999999999999998e-13)]
    Picojoule,
    #[linear("fJ", "femtojoules", 1.0000000000000001e-15)]
    Femtojoule,
    #[linear("aJ", "attojoules", 1.0000000000000001e-18)]
    Attojoule,
    #[linear("zJ", "zeptojoules", 9.9999999999999991e-22)]
    Zeptojoule,
    #[linear("yJ", "yoctojoules", 9.9999999999999992e-25)]
    Yoctojoule,
    #[linear("PW · h", "petawatt hours", 3.6e+18)]
    PetawattHour,
    #[linear("TW · h", "terawatt hours", 3600000000000000.0)]
    TerawattHour,
    #[linear("GW · h", "gigawatt hours", 3600000000000.0)]
    GigawattHour,
    #[linear("MW · h", "megawatt hours", 3600000000.0)]
    MegawattHour,
    #[linear("kW · h", "kilowatt hours", 3600000.0)]
    KilowattHour,
    #[linear("hW · h", "hectowatt hours", 360000.0)]
    HectowattHour,
    #[linear("daW · h", "decawatt hours", 36000.0)]
    DecawattHour,
    #[linear("W · h", "watt hours", 3600.0)]
    WattHour,
    #[linear("mW · h", "milliwatt hours", 3.6000000000000001)]
    MilliwattHour,
    #[linear("µW · h", "microwatt hours", 0.0035999999999999999)]
    MicrowattHour,
    #[linear("PeV", "petaelectronvolts", 0.00016021766339999999)]
    Petaelectronvolt,
    #[linear("TeV", "teraelectronvolts", 1.6021766339999999e-07)]
    Teraelectronvolt,
    #[linear("GeV", "gigaelectronvolts", 1.6021766340000001e-10)]
    Gigaelectronvolt,
    #[linear("MeV", "megaelectronvolts", 1.6021766340000001e-13)]
    Megaelectronvolt,
    #[linear("keV", "kiloelectronvolts", 1.6021766339999999e-16)]
    Kiloelectronvolt,
    #[linear("heV", "hectoelectronvolts", 1.6021766339999999e-17)]
    Hectoelectronvolt,
    #[linear("daeV", "decaelectronvolts", 1.6021766339999999e-18)]
    Decaelectronvolt,
    #[linear("eV", "electronvolts", 1.6021766339999999e-19)]
    Electronvolt,
    #[linear("Eₕ", "hartrees", 4.3597447222071e-18)]
    Hartree,
    #[linear("Btu (IT)", "British thermal units (IT)", 1055.056)]
    BtuIt,
    #[linear("Btu", "British thermal units", 1054.3499999999999)]
    Btu,
    #[linear("cal (IT)", "calories (IT)", 4.1867999999999999)]
    CalorieIt,
    #[linear("cal", "calories", 4.1840000000000002)]
    Calorie,
    #[linear("Cal (IT)", "Calories (IT)", 4186.8000000000002)]
    CalorieItNutrition,
    #[linear("Cal", "Calories", 4184.0)]
    CalorieNutrition,
    #[linear("erg", "ergs", 9.9999999999999995e-08)]
    Erg,
    #[linear("ft · pdl", "foot poundals", 0.042140110000000001)]
    FootPoundal,
    #[linear("ft · lbf", "foot pounds-force", 1.355818)]
    FootPound,
    #[linear("kcal (IT)", "kilocalories (IT)", 4186.8000000000002)]
    KilocalorieIt,
    #[linear("kcal", "kilocalories", 4184.0)]
    Kilocalorie,
    #[linear("10¹⁵ Btu (IT)", "quads", 1.055056e+18)]
    Quad,
    #[linear("thm (EC)", "therms (EC)", 105506000.0)]
    ThermEc,
    #[linear("thm", "therms", 105480400.0)]
    ThermUs,
    #[linear("t of TNT", "tons of TNT", 4184000000.0)]
    TonTnt,
    #[linear("W · s", "watt seconds", 1.0)]
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

#[derive(Unit, Copy, Clone)]
#[dim(AvailableEnergy)]
pub enum AvailableEnergyUnit {
    #[linear("YJ/kg", "yottajoules per kilogram", 9.9999999999999998e+23)]
    YottajoulePerKilogram,
    #[linear("ZJ/kg", "zettajoules per kilogram", 1e+21)]
    ZettajoulePerKilogram,
    #[linear("EJ/kg", "exajoules per kilogram", 1e+18)]
    ExajoulePerKilogram,
    #[linear("PJ/kg", "petajoules per kilogram", 1000000000000000.0)]
    PetajoulePerKilogram,
    #[linear("TJ/kg", "terajoules per kilogram", 1000000000000.0)]
    TerajoulePerKilogram,
    #[linear("GJ/kg", "gigajoules per kilogram", 1000000000.0)]
    GigajoulePerKilogram,
    #[linear("MJ/kg", "megajoules per kilogram", 1000000.0)]
    MegajoulePerKilogram,
    #[linear("kJ/kg", "kilojoules per kilogram", 1000.0)]
    KilojoulePerKilogram,
    #[linear("hJ/kg", "hectojoules per kilogram", 100.0)]
    HectojoulePerKilogram,
    #[linear("daJ/kg", "decajoules per kilogram", 10.0)]
    DecajoulePerKilogram,
    #[linear("J/kg", "joules per kilogram", 1.0)]
    JoulePerKilogram,
    #[linear("dJ/kg", "decijoules per kilogram", 0.10000000000000001)]
    DecijoulePerKilogram,
    #[linear("cJ/kg", "centijoules per kilogram", 0.01)]
    CentijoulePerKilogram,
    #[linear("mJ/kg", "millijoules per kilogram", 0.001)]
    MillijoulePerKilogram,
    #[linear("µJ/kg", "microjoules per kilogram", 9.9999999999999995e-07)]
    MicrojoulePerKilogram,
    #[linear("nJ/kg", "nanojoules per kilogram", 1.0000000000000001e-09)]
    NanojoulePerKilogram,
    #[linear("pJ/kg", "picojoules per kilogram", 9.9999999999999998e-13)]
    PicojoulePerKilogram,
    #[linear("fJ/kg", "femtojoules per kilogram", 1.0000000000000001e-15)]
    FemtojoulePerKilogram,
    #[linear("aJ/kg", "attojoules per kilogram", 1.0000000000000001e-18)]
    AttojoulePerKilogram,
    #[linear("zJ/kg", "zeptojoules per kilogram", 9.9999999999999991e-22)]
    ZeptojoulePerKilogram,
    #[linear("yJ/kg", "yoctojoules per kilogram", 9.9999999999999992e-25)]
    YoctojoulePerKilogram,
    #[linear("J/zg", "joules per zeptogram", 9.9999999999999998e+23)]
    JoulePerZeptogram,
    #[linear("J/ag", "joules per attogram", 1e+21)]
    JoulePerAttogram,
    #[linear("J/fg", "joules per femtogram", 1e+18)]
    JoulePerFemtogram,
    #[linear("J/pg", "joules per picogram", 1000000000000000.0)]
    JoulePerPicogram,
    #[linear("J/ng", "joules per nanogram", 1000000000000.0)]
    JoulePerNanogram,
    #[linear("J/µg", "joules per microgram", 1000000000.0)]
    JoulePerMicrogram,
    #[linear("J/mg", "joules per milligram", 1000000.0)]
    JoulePerMilligram,
    #[linear("J/g", "joules per gram", 1000.0)]
    JoulePerGram,
    #[linear("J/Mg", "joules per megagram", 0.001)]
    JoulePerMegagram,
    #[linear("J/Gg", "joules per gigagram", 9.9999999999999995e-07)]
    JoulePerGigagram,
    #[linear("J/Tg", "joules per teragram", 1.0000000000000001e-09)]
    JoulePerTeragram,
    #[linear("J/Pg", "joules per petagram", 9.9999999999999998e-13)]
    JoulePerPetagram,
    #[linear("J/Eg", "joules per exagram", 1.0000000000000001e-15)]
    JoulePerExagram,
    #[linear("J/Zg", "joules per zettagram", 1.0000000000000001e-18)]
    JoulePerZettagram,
    #[linear("J/Yg", "joules per yottagram", 9.9999999999999991e-22)]
    JoulePerYottagram,
    #[linear(
        "Btu (IT)/lb",
        "British thermal units (IT) per pound",
        2326.0001710787042
    )]
    BtuItPerPound,
    #[linear("Btu/lb", "British thermal units per pound", 2324.443707610621)]
    BtuPerPound,
    #[linear("cal (IT)/lb", "calories (IT) per gram", 4186.8000000000002)]
    CalorieItPerGram,
    #[linear("cal/lb", "calories per gram", 4184.0)]
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

#[derive(Unit, Copy, Clone)]
#[dim(Power)]
pub enum PowerUnit {
    #[linear("YW", "yottawatts", 9.9999999999999998e+23)]
    Yottawatt,
    #[linear("ZW", "zettawatts", 1e+21)]
    Zettawatt,
    #[linear("EW", "exawatts", 1e+18)]
    Exawatt,
    #[linear("PW", "petawatts", 1000000000000000.0)]
    Petawatt,
    #[linear("TW", "terawatts", 1000000000000.0)]
    Terawatt,
    #[linear("GW", "gigawatts", 1000000000.0)]
    Gigawatt,
    #[linear("MW", "megawatts", 1000000.0)]
    Megawatt,
    #[linear("kW", "kilowatts", 1000.0)]
    Kilowatt,
    #[linear("hW", "hectowatts", 100.0)]
    Hectowatt,
    #[linear("daW", "decawatts", 10.0)]
    Decawatt,
    #[linear("W", "watts", 1.0)]
    Watt,
    #[log("dBW", "decibels-watt", 10.0, 0.1)]
    DecibelWatt,
    #[log("dBm", "decibels-milliwatt", 10.0, 0.1, reference = 0.001)]
    DecibelMilliwatt,
    #[linear("dW", "deciwatts", 0.10000000000000001)]
    Deciwatt,
    #[linear("cW", "centiwatts", 0.01)]
    Centiwatt,
    #[linear("mW", "milliwatts", 0.001)]
    Milliwatt,
    #[linear("µW", "microwatts", 9.9999999999999995e-07)]
    Microwatt,
    #[linear("nW", "nanowatts", 1.0000000000000001e-09)]
    Nanowatt,
    #[linear("pW", "picowatts", 9.9999999999999998e-13)]
    Picowatt,
    #[linear("fW", "femtowatts", 1.0000000000000001e-15)]
    Femtowatt,
    #[linear("aW", "attowatts", 1.0000000000000001e-18)]
    Attowatt,
    #[linear("zW", "zeptowatts", 9.9999999999999991e-22)]
    Zeptowatt,
    #[linear("yW", "yoctowatts", 9.9999999999999992e-25)]
    Yoctowatt,
    #[linear("erg/s", "ergs per second", 9.9999999999999995e-08)]
    ErgPerSecond,
    #[linear("ft · lbf/h", "foot pounds-force per hour", 0.00037661611111111112)]
    FootPoundPerHour,
    #[linear("ft · lbf/min", "foot pounds-force per minute", 0.022596966666666666)]
    FootPoundPerMinute,
    #[linear("ft · lbf/s", "foot pounds-force per second", 1.355818)]
    FootPoundPerSecond,
    #[linear("hp", "horsepower", 745.69989999999996)]
    Horsepower,
    #[linear("hp (S)", "horsepower (boiler)", 9809.5)]
    HorsepowerBoiler,
    #[linear("hp (E)", "horsepower (electric)", 746.0)]
    HorsepowerElectric,
    #[linear("hp (M)", "metric horsepower", 735.49879999999996)]
    HorsepowerMetric,
    #[linear("hp (I)", "horsepower (Imperial)", 745.70000000000005)]
    HorsepowerImperial,
    #[linear("hp (hydraulic)", "hydraulic horsepower", 746.04300000000001)]
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

#[derive(Unit, Copy, Clone)]
#[dim(PowerRate)]
pub enum PowerRateUnit {
    #[linear("YW/s", "yottawatts per second", 9.9999999999999998e+23)]
    YottawattPerSecond,
    #[linear("ZW/s", "zettawatts per second", 1e+21)]
    ZettawattPerSecond,
    #[linear("EW/s", "exawatts per second", 1e+18)]
    ExawattPerSecond,
    #[linear("PW/s", "petawatts per second", 1000000000000000.0)]
    PetawattPerSecond,
    #[linear("TW/s", "terawatts per second", 1000000000000.0)]
    TerawattPerSecond,
    #[linear("GW/s", "gigawatts per second", 1000000000.0)]
    GigawattPerSecond,
    #[linear("MW/s", "megawatts per second", 1000000.0)]
    MegawattPerSecond,
    #[linear("kW/s", "kilowatts per second", 1000.0)]
    KilowattPerSecond,
    #[linear("hW/s", "hectowatts per second", 100.0)]
    HectowattPerSecond,
    #[linear("daW/s", "decawatts per second", 10.0)]
    DecawattPerSecond,
    #[linear("W/s", "watts per second", 1.0)]
    WattPerSecond,
    #[linear("dW/s", "deciwatts per second", 0.10000000000000001)]
    DeciwattPerSecond,
    #[linear("cW/s", "centiwatts per second", 0.01)]
    CentiwattPerSecond,
    #[linear("mW/s", "milliwatts per second", 0.001)]
    MilliwattPerSecond,
    #[linear("µW/s", "microwatts per second", 9.9999999999999995e-07)]
    MicrowattPerSecond,
    #[linear("nW/s", "nanowatts per second", 1.0000000000000001e-09)]
    NanowattPerSecond,
    #[linear("pW/s", "picowatts per second", 9.9999999999999998e-13)]
    PicowattPerSecond,
    #[linear("fW/s", "femtowatts per second", 1.0000000000000001e-15)]
    FemtowattPerSecond,
    #[linear("aW/s", "attowatts per second", 1.0000000000000001e-18)]
    AttowattPerSecond,
    #[linear("zW/s", "zeptowatts per second", 9.9999999999999991e-22)]
    ZeptowattPerSecond,
    #[linear("yW/s", "yoctowatts per second", 9.9999999999999992e-25)]
    YoctowattPerSecond,
    #[linear("erg/s²", "ergs per second squared", 9.9999999999999995e-08)]
    ErgPerSecondSquared,
    #[linear("ft · lbf/s²", "foot pounds-force per second squared", 1.355818)]
    FootPoundPerSecondSquared,
    #[linear("hp/s", "horsepower per second", 745.69989999999996)]
    HorsepowerPerSecond,
    #[linear("hp/s (S)", "horsepower per second (boiler)", 9809.5)]
    HorsepowerPerSecondBoiler,
    #[linear("hp/s (E)", "horsepower per second (electric)", 746.0)]
    HorsepowerPerSecondElectric,
    #[linear("hp/s (M)", "metric horsepower per second", 735.49879999999996)]
    HorsepowerPerSecondMetric,
    #[linear("hp/s (I)", "horsepower per second (Imperial)", 745.70000000000005)]
    HorsepowerPerSecondImperial,
    #[linear(
        "hp/s (hydraulic)",
        "hydraulic horsepower per second",
        746.04300000000001
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

#[derive(Unit, Copy, Clone)]
#[dim(Action)]
pub enum ActionUnit {
    #[linear("J · s", "joule seconds", 1.0)]
    JouleSecond,
    #[linear("ħ", "atomic units of action", 1.054571817e-34)]
    AtomicUnitOfAction,
    #[linear("ħ", "reduced planck constants", 1.054571817e-34)]
    ReducedPlanckConstant,
    #[linear("h", "planck constants", 6.6260701499999998e-34)]
    PlanckConstant,
    #[linear("erg · s", "erg seconds", 9.9999999999999995e-08)]
    ErgSecond,
    #[linear("eV · s", "electronvolt seconds", 1.6021766339999999e-19)]
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

#[derive(Unit, Copy, Clone)]
#[dim(Torque)]
pub enum TorqueUnit {
    #[linear("YN · m", "yottanewton meters", 9.9999999999999998e+23)]
    YottanewtonMeter,
    #[linear("ZN · m", "zettanewton meters", 1e+21)]
    ZettanewtonMeter,
    #[linear("EN · m", "exanewton meters", 1e+18)]
    ExanewtonMeter,
    #[linear("PN · m", "petanewton meters", 1000000000000000.0)]
    PetanewtonMeter,
    #[linear("TN · m", "teranewton meters", 1000000000000.0)]
    TeranewtonMeter,
    #[linear("GN · m", "giganewton meters", 1000000000.0)]
    GiganewtonMeter,
    #[linear("MN · m", "meganewton meters", 1000000.0)]
    MeganewtonMeter,
    #[linear("kN · m", "kilonewton meters", 1000.0)]
    KilonewtonMeter,
    #[linear("hN · m", "hectonewton meters", 100.0)]
    HectonewtonMeter,
    #[linear("daN · m", "decanewton meters", 10.0)]
    DecanewtonMeter,
    #[linear("N · m", "newton meters", 1.0)]
    NewtonMeter,
    #[linear("dN · m", "decinewton meters", 0.10000000000000001)]
    DecinewtonMeter,
    #[linear("cN · m", "centinewton meters", 0.01)]
    CentinewtonMeter,
    #[linear("mN · m", "millinewton meters", 0.001)]
    MillinewtonMeter,
    #[linear("µN · m", "micronewton meters", 9.9999999999999995e-07)]
    MicronewtonMeter,
    #[linear("nN · m", "nanonewton meters", 1.0000000000000001e-09)]
    NanonewtonMeter,
    #[linear("pN · m", "piconewton meters", 9.9999999999999998e-13)]
    PiconewtonMeter,
    #[linear("fN · m", "femtonewton meters", 1.0000000000000001e-15)]
    FemtonewtonMeter,
    #[linear("aN · m", "attonewton meters", 1.0000000000000001e-18)]
    AttonewtonMeter,
    #[linear("zN · m", "zeptonewton meters", 9.9999999999999991e-22)]
    ZeptonewtonMeter,
    #[linear("yN · m", "yoctonewton meters", 9.9999999999999992e-25)]
    YoctonewtonMeter,
    #[linear("N · Ym", "newton yottameters", 9.9999999999999998e+23)]
    NewtonYottameter,
    #[linear("N · Zm", "newton zettameters", 1e+21)]
    NewtonZettameter,
    #[linear("N · Em", "newton exameters", 1e+18)]
    NewtonExameter,
    #[linear("N · Pm", "newton petameters", 1000000000000000.0)]
    NewtonPetameter,
    #[linear("N · Tm", "newton terameters", 1000000000000.0)]
    NewtonTerameter,
    #[linear("N · Gm", "newton gigameters", 1000000000.0)]
    NewtonGigameter,
    #[linear("N · Mm", "newton megameters", 1000000.0)]
    NewtonMegameter,
    #[linear("N · km", "newton kilometers", 1000.0)]
    NewtonKilometer,
    #[linear("N · hm", "newton hectometers", 100.0)]
    NewtonHectometer,
    #[linear("N · dam", "newton decameters", 10.0)]
    NewtonDecameter,
    #[linear("N · dm", "newton decimeters", 0.10000000000000001)]
    NewtonDecimeter,
    #[linear("N · cm", "newton centimeters", 0.01)]
    NewtonCentimeter,
    #[linear("N · mm", "newton millimeters", 0.001)]
    NewtonMillimeter,
    #[linear("N · µm", "newton micrometers", 9.9999999999999995e-07)]
    NewtonMicrometer,
    #[linear("N · nm", "newton nanometers", 1.0000000000000001e-09)]
    NewtonNanometer,
    #[linear("N · pm", "newton picometers", 9.9999999999999998e-13)]
    NewtonPicometer,
    #[linear("N · fm", "newton femtometers", 1.0000000000000001e-15)]
    NewtonFemtometer,
    #[linear("N · am", "newton attometers", 1.0000000000000001e-18)]
    NewtonAttometer,
    #[linear("N · zm", "newton zeptometers", 9.9999999999999991e-22)]
    NewtonZeptometer,
    #[linear("N · ym", "newton yoctometers", 9.9999999999999992e-25)]
    NewtonYoctometer,
    #[linear("dyn · m", "dyne meters", 1.0000000000000001e-05)]
    DyneMeter,
    #[linear("dyn · cm", "dyne centimeters", 9.9999999999999995e-08)]
    DyneCentimeter,
    #[linear("kgf · m", "kilogram-force meters", 9.8066499999999994)]
    KilogramForceMeter,
    #[linear("ozf · in", "ounces-force inches", 0.0070615530599999997)]
    OunceForceInch,
    #[linear("lbf · ft", "pounds-force feet", 1.3558180656000001)]
    PoundForceFoot,
    #[linear("lbf · in", "pounds-force inches", 0.1129848388)]
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

#[derive(Unit, Copy, Clone)]
#[dim(MomentOfInertia)]
pub enum MomentOfInertiaUnit {
    #[linear("kg · m²", "kilogram square meters", 1.0)]
    KilogramSquareMeter,
    #[linear("Da · Å²", "dalton square ångströms", 1.6605390666e-47)]
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

#[derive(Unit, Copy, Clone)]
#[dim(Pressure)]
pub enum PressureUnit {
    #[linear("YPa", "yottapascals", 9.9999999999999998e+23)]
    Yottapascal,
    #[linear("ZPa", "zettapascals", 1e+21)]
    Zettapascal,
    #[linear("EPa", "exapascals", 1e+18)]
    Exapascal,
    #[linear("PPa", "petapascals", 1000000000000000.0)]
    Petapascal,
    #[linear("TPa", "terapascals", 1000000000000.0)]
    Terapascal,
    #[linear("GPa", "gigapascals", 1000000000.0)]
    Gigapascal,
    #[linear("MPa", "megapascals", 1000000.0)]
    Megapascal,
    #[linear("kPa", "kilopascals", 1000.0)]
    Kilopascal,
    #[linear("hPa", "hectopascals", 100.0)]
    Hectopascal,
    #[linear("daPa", "decapascals", 10.0)]
    Decapascal,
    #[linear("Pa", "pascals", 1.0)]
    Pascal,
    #[linear("dPa", "decipascals", 0.10000000000000001)]
    Decipascal,
    #[linear("cPa", "centipascals", 0.01)]
    Centipascal,
    #[linear("mPa", "millipascals", 0.001)]
    Millipascal,
    #[linear("µPa", "micropascals", 9.9999999999999995e-07)]
    Micropascal,
    #[linear("nPa", "nanopascals", 1.0000000000000001e-09)]
    Nanopascal,
    #[linear("pPa", "picopascals", 9.9999999999999998e-13)]
    Picopascal,
    #[linear("fPa", "femtopascals", 1.0000000000000001e-15)]
    Femtopascal,
    #[linear("aPa", "attopascals", 1.0000000000000001e-18)]
    Attopascal,
    #[linear("zPa", "zeptopascals", 9.9999999999999991e-22)]
    Zeptopascal,
    #[linear("yPa", "yoctopascals", 9.9999999999999992e-25)]
    Yoctopascal,
    #[linear("atm", "atmospheres", 101325.0)]
    Atmosphere,
    #[linear("at", "atmospheres (technical)", 98066.5)]
    AtmosphereTechnical,
    #[linear("bar", "bar", 100000.0)]
    Bar,
    #[linear("cm Hg", "centimeters of mercury", 1333.2239999999999)]
    CentimeterOfMercury,
    #[linear("cm H₂O", "centimeters of water", 98.066500000000005)]
    CentimeterOfWater,
    #[linear("dyn/cm²", "dynes per square centimeter", 0.10000000000000001)]
    DynePerSquareCentimeter,
    #[linear("ft Hg", "feet of mercury", 40636.660000000003)]
    FootOfMercury,
    #[linear("ft H₂O", "feet of water", 2989.067)]
    FootOfWater,
    #[linear("in Hg", "inches of mercury", 3386.3890000000001)]
    InchOfMercury,
    #[linear("in H₂O", "inches of water", 249.0889)]
    InchOfWater,
    #[linear("N/mm²", "newtons per square millimeter", 1000000.0)]
    NewtonPerSquareMillimeter,
    #[linear("kip/in²", "kips per square inch", 6894757.889515779)]
    KipPerSquareInch,
    #[linear("mbar", "millibar", 100.0)]
    Millibar,
    #[linear("mm Hg", "millimeters of mercury", 133.32239999999999)]
    MillimeterOfMercury,
    #[linear("mm H₂O", "millimeters of water", 9.8066499999999994)]
    MillimeterOfWater,
    #[linear("mTorr", "millitorr", 0.13332240000000001)]
    Millitorr,
    #[linear("pdl/ft²", "poundals per square foot", 1.4881644346622025)]
    PoundalPerSquareFoot,
    #[linear("lbf/ft²", "pounds-force per square foot", 47.880263121637356)]
    PoundForcePerSquareFoot,
    #[linear("lbf/in²", "pounds-force per square inch", 6894.7578895157794)]
    PoundForcePerSquareInch,
    #[linear("psi", "pounds-force per square inch", 6894.7569999999996)]
    Psi,
    #[linear("Torr", "torr", 133.32239999999999)]
    Torr,
}

pub type Frequency = StaticSIQuantity<{ ISQ { s: -1, ..ISQ::ZERO } }>;

#[derive(Unit, Copy, Clone)]
#[dim(Frequency)]
pub enum FrequencyUnit {
    #[linear("YHz", "yottahertz", 9.9999999999999998e+23)]
    Yottahertz,
    #[linear("ZHz", "zettahertz", 1e+21)]
    Zettahertz,
    #[linear("EHz", "exahertz", 1e+18)]
    Exahertz,
    #[linear("PHz", "petahertz", 1000000000000000.0)]
    Petahertz,
    #[linear("THz", "terahertz", 1000000000000.0)]
    Terahertz,
    #[linear("GHz", "gigahertz", 1000000000.0)]
    Gigahertz,
    #[linear("MHz", "megahertz", 1000000.0)]
    Megahertz,
    #[linear("kHz", "kilohertz", 1000.0)]
    Kilohertz,
    #[linear("hHz", "hectohertz", 100.0)]
    Hectohertz,
    #[linear("daHz", "decahertz", 10.0)]
    Decahertz,
    #[linear("Hz", "hertz", 1.0)]
    Hertz,
    #[linear("dHz", "decihertz", 0.10000000000000001)]
    Decihertz,
    #[linear("cHz", "centihertz", 0.01)]
    Centihertz,
    #[linear("mHz", "millihertz", 0.001)]
    Millihertz,
    #[linear("µHz", "microhertz", 9.9999999999999995e-07)]
    Microhertz,
    #[linear("nHz", "nanohertz", 1.0000000000000001e-09)]
    Nanohertz,
    #[linear("pHz", "picohertz", 9.9999999999999998e-13)]
    Picohertz,
    #[linear("fHz", "femtohertz", 1.0000000000000001e-15)]
    Femtohertz,
    #[linear("aHz", "attohertz", 1.0000000000000001e-18)]
    Attohertz,
    #[linear("zHz", "zeptohertz", 9.9999999999999991e-22)]
    Zeptohertz,
    #[linear("yHz", "yoctohertz", 9.9999999999999992e-25)]
    Yoctohertz,
    #[linear("1/d", "cycles per day", 1.1574074074074073e-05)]
    CyclePerDay,
    #[linear("1/h", "cycles per hour", 0.00027777777777777772)]
    CyclePerHour,
    #[linear("1/min", "cycles per minute", 0.016666666666666666)]
    CyclePerMinute,
    #[linear("100 MHz", "cycles per shake", 100000000.0)]
    CyclePerShake,
    #[linear("1/a", "cycles per year", 3.1709791983764579e-08)]
    CyclePerYear,
}

pub type FrequencyDrift = StaticSIQuantity<{ ISQ { s: -2, ..ISQ::ZERO } }>;

#[derive(Unit, Copy, Clone)]
#[dim(FrequencyDrift)]
pub enum FrequencyDriftUnit {
    #[linear("THz/s", "terahertz per second", 1000000000000.0)]
    TerahertzPerSecond,
    #[linear("GHz/s", "gigahertz per second", 1000000000.0)]
    GigahertzPerSecond,
    #[linear("MHz/s", "megahertz per second", 1000000.0)]
    MegahertzPerSecond,
    #[linear("kHz/s", "kilohertz per second", 1000.0)]
    KilohertzPerSecond,
    #[linear("Hz/s", "hertz per second", 1.0)]
    HertzPerSecond,
}

pub type ReciprocalLength = StaticSIQuantity<{ ISQ { m: -1, ..ISQ::ZERO } }>;

#[derive(Unit, Copy, Clone)]
#[dim(ReciprocalLength)]
pub enum ReciprocalLengthUnit {
    #[linear("km⁻¹", "reciprocal kilometers", 0.001)]
    ReciprocalKilometer,
    #[linear("m⁻¹", "reciprocal meters", 1.0)]
    ReciprocalMeter,
    #[linear("dm⁻¹", "reciprocal decimeters", 10.0)]
    ReciprocalDecimeter,
    #[linear("cm⁻¹", "reciprocal centimeters", 100.0)]
    ReciprocalCentimeter,
    #[linear("mm⁻¹", "reciprocal millimeters", 1000.0)]
    ReciprocalMillimeter,
    #[linear("µm⁻¹", "reciprocal micrometers", 1000000.0)]
    ReciprocalMicrometer,
    #[linear("nm⁻¹", "reciprocal nanometers", 999999999.99999988)]
    ReciprocalNanometer,
    #[linear("Å⁻¹", "reciprocal ångströms", 10000000000.0)]
    ReciprocalAngstrom,
    #[linear("dpt", "diopters", 1.0)]
    Diopter,
    #[linear("R∞", "Rydberg constants", 10973731.568159999)]
    RydbergConstant,
}

pub type Curvature = StaticSIQuantity<{ ISQ { m: -1, ..ISQ::ZERO } }>;

#[derive(Unit, Copy, Clone)]
#[dim(Curvature)]
pub enum CurvatureUnit {
    #[linear("rad/m", "radians per meter", 1.0)]
    RadianPerMeter,
    #[linear("°/m", "degrees per meter", 0.017453292519943295)]
    DegreePerMeter,
    #[linear("rad/mm", "radians per millimeter", 1000.0)]
    RadianPerMillimeter,
    #[linear("°/mm", "degrees per millimeter", 17.453292519943297)]
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

#[derive(Unit, Copy, Clone)]
#[dim(SpecificArea)]
pub enum SpecificAreaUnit {
    #[linear("m²/kg", "square meters per kilogram", 1.0)]
    SquareMeterPerKilogram,
    #[linear("cm²/kg", "square centimeters per kilogram", 0.0001)]
    SquareCentimeterPerKilogram,
    #[linear("m²/g", "square meters per gram", 1000.0)]
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

#[derive(Unit, Copy, Clone)]
#[dim(SpecificVolume)]
pub enum SpecificVolumeUnit {
    #[linear("m³/Yg", "cubic meters per yottagram", 1.0000000000000001e-21)]
    CubicMeterPerYottagram,
    #[linear("m³/Zg", "cubic meters per zettagram", 1.0000000000000001e-18)]
    CubicMeterPerZettagram,
    #[linear("m³/Eg", "cubic meters per exagram", 1.0000000000000001e-15)]
    CubicMeterPerExagram,
    #[linear("m³/Pg", "cubic meters per petagram", 9.9999999999999998e-13)]
    CubicMeterPerPetagram,
    #[linear("m³/Tg", "cubic meters per teragram", 1.0000000000000001e-09)]
    CubicMeterPerTeragram,
    #[linear("m³/Gg", "cubic meters per gigagram", 9.9999999999999995e-07)]
    CubicMeterPerGigagram,
    #[linear("m³/Mg", "cubic meters per megagram", 0.001)]
    CubicMeterPerMegagram,
    #[linear("m³/kg", "cubic meters per kilogram", 1.0)]
    CubicMeterPerKilogram,
    #[linear("m³/hg", "cubic meters per hectogram", 10.0)]
    CubicMeterPerHectogram,
    #[linear("m³/dag", "cubic meters per decagram", 100.0)]
    CubicMeterPerDecagram,
    #[linear("m³/g", "cubic meters per gram", 1000.0)]
    CubicMeterPerGram,
    #[linear("m³/dg", "cubic meters per decigram", 10000.0)]
    CubicMeterPerDecigram,
    #[linear("m³/cg", "cubic meters per centigram", 100000.0)]
    CubicMeterPerCentigram,
    #[linear("m³/mg", "cubic meters per milligram", 1000000.0)]
    CubicMeterPerMilligram,
    #[linear("m³/µg", "cubic meters per microgram", 1000000000.0)]
    CubicMeterPerMicrogram,
    #[linear("m³/ng", "cubic meters per nanogram", 999999999999.99988)]
    CubicMeterPerNanogram,
    #[linear("m³/pg", "cubic meters per picogram", 1000000000000000.0)]
    CubicMeterPerPicogram,
    #[linear("m³/fg", "cubic meters per femtogram", 9.9999999999999987e+17)]
    CubicMeterPerFemtogram,
    #[linear("m³/ag", "cubic meters per attogram", 9.9999999999999987e+20)]
    CubicMeterPerAttogram,
    #[linear("m³/zg", "cubic meters per zeptogram", 1.0000000000000001e+24)]
    CubicMeterPerZeptogram,
    #[linear("m³/yg", "cubic meters per yoctogram", 1e+27)]
    CubicMeterPerYoctogram,
    #[linear("m³/ct", "cubic meters per carat", 5000.0)]
    CubicMeterPerCarat,
    #[linear("m³/gr", "cubic meters per grain", 15432.358352941432)]
    CubicMeterPerGrain,
    #[linear(
        "m³/cwt long",
        "cubic meters per hundredweight (long)",
        0.019684128785380992
    )]
    CubicMeterPerHundredweightLong,
    #[linear(
        "m³/cwt short",
        "cubic meters per hundredweight (short)",
        0.022046224760379582
    )]
    CubicMeterPerHundredweightShort,
    #[linear("m³/oz", "cubic meters per ounce", 35.273965837869568)]
    CubicMeterPerOunce,
    #[linear("m³/oz t", "cubic meters per troy ounce", 32.150743260882706)]
    CubicMeterPerOunceTroy,
    #[linear("m³/dwt", "cubic meters per pennyweight", 643.01486521765401)]
    CubicMeterPerPennyweight,
    #[linear("m³/lb", "cubic meters per pound", 2.2046224760379585)]
    CubicMeterPerPound,
    #[linear("m³/lb t", "cubic meters per troy pound", 2.6792290357695832)]
    CubicMeterPerPoundTroy,
    #[linear("m³/slug", "cubic meters per slug", 0.068521779647661013)]
    CubicMeterPerSlug,
    #[linear("m³/AT", "cubic meters per assay ton", 34.285710367347392)]
    CubicMeterPerTonAssay,
    #[linear("m³/2240 lb", "cubic meters per long ton", 0.00098420643926904958)]
    CubicMeterPerTonLong,
    #[linear("m³/2000 lb", "cubic meters per short ton", 0.0011023113595279991)]
    CubicMeterPerTonShort,
    #[linear("m³/t", "cubic meters per ton", 0.001)]
    CubicMeterPerTon,
    #[linear("gal/gr", "gallons per grain", 58.417834497524737)]
    GallonPerGrain,
    #[linear("in³/oz", "cubic inches per ounce", 0.00057803659462311895)]
    CubicInchPerOunce,
    #[linear("gal (UK)/oz", "Imperial gallons per ounce", 0.16035862335588044)]
    GallonImperialPerOunce,
    #[linear("gal/oz", "gallons per ounce", 0.1335264935702615)]
    GallonPerOunce,
    #[linear("ft³/lb", "cubic feet per pound", 0.06242796396059546)]
    CubicFootPerPound,
    #[linear("in³/lb", "cubic inches per pound", 3.6127280792182591e-05)]
    CubicInchPerPound,
    #[linear("yd³/lb", "cubic yards per pound", 1.6855549167049537)]
    CubicYardPerPound,
    #[linear("gal (UK)/lb", "Imperial gallons per pound", 0.010022412192091401)]
    GallonImperialPerPound,
    #[linear("gal/lb", "gallons per pound", 0.0083454043762638003)]
    GallonPerPound,
    #[linear("ft³/slug", "cubic feett per slug", 0.0019403209560158697)]
    CubicFootPerSlug,
    #[linear("yd³/2240 lb", "cubic yards per long ton", 0.00075247985575470431)]
    CubicYardPerTonLong,
    #[linear("yd³/2000 lb", "cubic yards per short ton", 0.00084277755125279337)]
    CubicYardPerTonShort,
    #[linear("ft³/2240 lb", "cubic feet per long ton", 2.7869626109815785e-05)]
    CubicFootPerTonLong,
    #[linear("ft³/2000 lb", "cubic feet per short ton", 3.1213985421050424e-05)]
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

#[derive(Unit, Copy, Clone)]
#[dim(SpecificPower)]
pub enum SpecificPowerUnit {
    #[linear("YW/kg", "yottawatts per kilogram", 9.9999999999999998e+23)]
    YottawattPerKilogram,
    #[linear("ZW/kg", "zettawatts per kilogram", 1e+21)]
    ZettawattPerKilogram,
    #[linear("EW/kg", "exawatts per kilogram", 1e+18)]
    ExawattPerKilogram,
    #[linear("PW/kg", "petawatts per kilogram", 1000000000000000.0)]
    PetawattPerKilogram,
    #[linear("TW/kg", "terawatts per kilogram", 1000000000000.0)]
    TerawattPerKilogram,
    #[linear("GW/kg", "gigawatts per kilogram", 1000000000.0)]
    GigawattPerKilogram,
    #[linear("MW/kg", "megawatts per kilogram", 1000000.0)]
    MegawattPerKilogram,
    #[linear("kW/kg", "kilowatts per kilogram", 1000.0)]
    KilowattPerKilogram,
    #[linear("hW/kg", "hectowatts per kilogram", 100.0)]
    HectowattPerKilogram,
    #[linear("daW/kg", "decawatts per kilogram", 10.0)]
    DecawattPerKilogram,
    #[linear("W/kg", "watts per kilogram", 1.0)]
    WattPerKilogram,
    #[linear("dW/kg", "deciwatts per kilogram", 0.10000000000000001)]
    DeciwattPerKilogram,
    #[linear("cW/kg", "centiwatts per kilogram", 0.01)]
    CentiwattPerKilogram,
    #[linear("mW/kg", "milliwatts per kilogram", 0.001)]
    MilliwattPerKilogram,
    #[linear("µW/kg", "microwatts per kilogram", 9.9999999999999995e-07)]
    MicrowattPerKilogram,
    #[linear("nW/kg", "nanowatts per kilogram", 1.0000000000000001e-09)]
    NanowattPerKilogram,
    #[linear("pW/kg", "picowatts per kilogram", 9.9999999999999998e-13)]
    PicowattPerKilogram,
    #[linear("fW/kg", "femtowatts per kilogram", 1.0000000000000001e-15)]
    FemtowattPerKilogram,
    #[linear("aW/kg", "attowatts per kilogram", 1.0000000000000001e-18)]
    AttowattPerKilogram,
    #[linear("zW/kg", "zeptowatts per kilogram", 9.9999999999999991e-22)]
    ZeptowattPerKilogram,
    #[linear("yW/kg", "yoctowatts per kilogram", 9.9999999999999992e-25)]
    YoctowattPerKilogram,
    #[linear("m² · s⁻³", "square meters per cubic second", 1.0)]
    SquareMeterPerCubicSecond,
    #[linear("(erg/s)/kg", "ergs per second per kilogram", 9.9999999999999995e-08)]
    ErgPerSecondPerKilogram,
    #[linear(
        "(ft · lbf/h)/lb",
        "foot pounds-force per hour per pound",
        0.00083029634339356457
    )]
    FootPoundPerHourPerPound,
    #[linear(
        "(ft · lbf/min)/lb",
        "foot pounds-force per minute per pound",
        0.049817780603613873
    )]
    FootPoundPerMinutePerPound,
    #[linear(
        "(ft · lbf/s)/lb",
        "foot pounds-force per second per pound",
        2.9890668362168324
    )]
    FootPoundPerSecondPerPound,
    #[linear("hp/lb", "horsepower per pound", 1643.9867599192578)]
    HorsepowerPerPound,
    #[linear("(hp (S))/lb", "horsepower (boiler) per pound", 21626.244178694353)]
    HorsepowerBoilerPerPound,
    #[linear("(hp (E))/lb", "horsepower (electric) per pound", 1644.6483671243168)]
    HorsepowerElectricPerPound,
    #[linear("(hp (M))/lb", "metric horsepower per pound", 1621.497185578947)]
    HorsepowerMetricPerPound,
    #[linear("(hp (I))/lb", "horsepower (Imperial) per pound", 1643.9869803815056)]
    HorsepowerImperialPerPound,
    #[linear(
        "(hp (hydraulic))/lb",
        "hydraulic horsepower per pound",
        1644.7431658907865
    )]
    HydraulicHorsepowerPerPound,
}
