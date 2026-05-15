use crate::{ISQ, StaticSIQuantity};
use yadc::{Dimension, Unit};

pub type DynamicViscosity = StaticSIQuantity<
    {
        ISQ {
            m: -1,
            kg: 1,
            s: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(DynamicViscosity)]
pub enum DynamicViscosityUnit {
    #[linear("Pa · s", "pascal seconds", 1.0)]
    PascalSecond,
    #[linear("mPa · s", "millipascal seconds", 0.001)]
    MillipascalSecond,
    #[linear("µPa · s", "micropascal seconds", 9.9999999999999995e-07)]
    MicropascalSecond,
    #[linear("P", "poises", 0.10000000000000001)]
    Poise,
    #[linear("cP", "centipoises", 0.001)]
    Centipoise,
    #[linear("lb/(ft · s)", "pounds per foot second", 1.4881640419947506)]
    PoundPerFootSecond,
    #[linear("slug/(ft · s)", "slugs per foot second", 47.88024934383202)]
    SlugPerFootSecond,
    #[linear("g/(cm · s)", "grams per centimeter second", 0.10000000000000001)]
    GramPerCentimeterSecond,
}

pub type KinematicViscosity = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            s: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(KinematicViscosity)]
pub enum KinematicViscosityUnit {
    #[linear("m²/s", "square meters per second", 1.0)]
    SquareMeterPerSecond,
    #[linear("cm²/s", "square centimeters per second", 0.0001)]
    SquareCentimeterPerSecond,
    #[linear("mm²/s", "square millimeters per second", 9.9999999999999995e-07)]
    SquareMillimeterPerSecond,
    #[linear("µm²/s", "square micrometers per second", 9.9999999999999998e-13)]
    SquareMicrometerPerSecond,
    #[linear("nm²/s", "square nanometers per second", 1.0000000000000001e-18)]
    SquareNanometerPerSecond,
    #[linear("St", "stokes", 0.0001)]
    Stokes,
}

pub type SurfaceTension = StaticSIQuantity<
    {
        ISQ {
            kg: 1,
            s: -2,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(SurfaceTension)]
pub enum SurfaceTensionUnit {
    #[linear("YN/m", "yottanewtons per meter", 9.9999999999999998e+23)]
    YottanewtonPerMeter,
    #[linear("ZN/m", "zettanewtons per meter", 1e+21)]
    ZettanewtonPerMeter,
    #[linear("EN/m", "exanewtons per meter", 1e+18)]
    ExanewtonPerMeter,
    #[linear("PN/m", "petanewtons per meter", 1000000000000000.0)]
    PetanewtonPerMeter,
    #[linear("TN/m", "teranewtons per meter", 1000000000000.0)]
    TeranewtonPerMeter,
    #[linear("GN/m", "giganewtons per meter", 1000000000.0)]
    GiganewtonPerMeter,
    #[linear("MN/m", "meganewtons per meter", 1000000.0)]
    MeganewtonPerMeter,
    #[linear("kN/m", "kilonewtons per meter", 1000.0)]
    KilonewtonPerMeter,
    #[linear("hN/m", "hectonewtons per meter", 100.0)]
    HectonewtonPerMeter,
    #[linear("daN/m", "decanewtons per meter", 10.0)]
    DecanewtonPerMeter,
    #[linear("N/m", "newtons per meter", 1.0)]
    NewtonPerMeter,
    #[linear("dN/m", "decinewtons per meter", 0.10000000000000001)]
    DecinewtonPerMeter,
    #[linear("cN/m", "centinewtons per meter", 0.01)]
    CentinewtonPerMeter,
    #[linear("mN/m", "millinewtons per meter", 0.001)]
    MillinewtonPerMeter,
    #[linear("µN/m", "micronewtons per meter", 9.9999999999999995e-07)]
    MicronewtonPerMeter,
    #[linear("nN/m", "nanonewtons per meter", 1.0000000000000001e-09)]
    NanonewtonPerMeter,
    #[linear("pN/m", "piconewtons per meter", 9.9999999999999998e-13)]
    PiconewtonPerMeter,
    #[linear("fN/m", "femtonewtons per meter", 1.0000000000000001e-15)]
    FemtonewtonPerMeter,
    #[linear("aN/m", "attonewtons per meter", 1.0000000000000001e-18)]
    AttonewtonPerMeter,
    #[linear("zN/m", "zeptonewtons per meter", 9.9999999999999991e-22)]
    ZeptonewtonPerMeter,
    #[linear("yN/m", "yoctonewtons per meter", 9.9999999999999992e-25)]
    YoctonewtonPerMeter,
    #[linear("kgf/m", "kilograms-force per meter", 9.8066499999999994)]
    KilogramForcePerMeter,
    #[linear("ozf/in", "ounces-force per inch", 10.945429133858267)]
    OunceForcePerInch,
    #[linear("pdl/in", "poundals per inch", 5.4431102362204724)]
    PoundalPerInch,
    #[linear("lbf/in", "pounds-force per inch", 175.12685039370081)]
    PoundForcePerInch,
    #[linear("J/m²", "joules per square meter", 1.0)]
    JoulePerSquareMeter,
}

pub type CompressibilityCoefficient = StaticSIQuantity<
    {
        ISQ {
            m: 1,
            kg: -1,
            s: 2,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(CompressibilityCoefficient)]
pub enum CompressibilityCoefficientUnit {
    #[linear("YPa⁻¹", "per yottapascal", 1.0000000000000001e-24)]
    PerYottapascal,
    #[linear("ZPa⁻¹", "per zettapascal", 9.9999999999999991e-22)]
    PerZettapascal,
    #[linear("EPa⁻¹", "per exapascal", 1.0000000000000001e-18)]
    PerExapascal,
    #[linear("PPa⁻¹", "per petapascal", 1.0000000000000001e-15)]
    PerPetapascal,
    #[linear("TPa⁻¹", "per terapascal", 9.9999999999999998e-13)]
    PerTerapascal,
    #[linear("GPa⁻¹", "per gigapascal", 1.0000000000000001e-09)]
    PerGigapascal,
    #[linear("MPa⁻¹", "per megapascal", 9.9999999999999995e-07)]
    PerMegapascal,
    #[linear("kPa⁻¹", "per kilopascal", 0.001)]
    PerKilopascal,
    #[linear("hPa⁻¹", "per hectopascal", 0.01)]
    PerHectopascal,
    #[linear("daPa⁻¹", "per decapascal", 0.10000000000000001)]
    PerDecapascal,
    #[linear("Pa⁻¹", "per pascal", 1.0)]
    PerPascal,
    #[linear("dPa⁻¹", "per decipascal", 10.0)]
    PerDecipascal,
    #[linear("cPa⁻¹", "per centipascal", 100.0)]
    PerCentipascal,
    #[linear("mPa⁻¹", "per millipascal", 1000.0)]
    PerMillipascal,
    #[linear("µPa⁻¹", "per micropascal", 1000000.0)]
    PerMicropascal,
    #[linear("nPa⁻¹", "per nanopascal", 999999999.99999988)]
    PerNanopascal,
    #[linear("pPa⁻¹", "per picopascal", 1000000000000.0)]
    PerPicopascal,
    #[linear("fPa⁻¹", "per femtopascal", 999999999999999.88)]
    PerFemtopascal,
    #[linear("aPa⁻¹", "per attopascal", 9.9999999999999987e+17)]
    PerAttopascal,
    #[linear("zPa⁻¹", "per zeptopascal", 1.0000000000000001e+21)]
    PerZeptopascal,
    #[linear("yPa⁻¹", "per yoctopascal", 1.0000000000000001e+24)]
    PerYoctopascal,
    #[linear("atm⁻¹", "per atmosphere", 9.8692326671601285e-06)]
    PerAtmosphere,
    #[linear("at⁻¹", "per atmosphere (technical)", 1.0197162129779282e-05)]
    PerAtmosphereTechnical,
    #[linear("bar⁻¹", "per bar", 1.0000000000000001e-05)]
    PerBar,
    #[linear("mbar⁻¹", "per millibar", 0.01)]
    PerMillibar,
}

pub type MassRate = StaticSIQuantity<
    {
        ISQ {
            kg: 1,
            s: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(MassRate)]
pub enum MassRateUnit {
    #[linear("Yg/s", "yottagrams per second", 1e+21)]
    YottagramPerSecond,
    #[linear("Zg/s", "zettagrams per second", 1e+18)]
    ZettagramPerSecond,
    #[linear("Eg/s", "exagrams per second", 1000000000000000.0)]
    ExagramPerSecond,
    #[linear("Pg/s", "petagrams per second", 1000000000000.0)]
    PetagramPerSecond,
    #[linear("Tg/s", "teragrams per second", 1000000000.0)]
    TeragramPerSecond,
    #[linear("Gg/s", "gigagrams per second", 1000000.0)]
    GigagramPerSecond,
    #[linear("Mg/s", "megagrams per second", 1000.0)]
    MegagramPerSecond,
    #[linear("kg/s", "kilograms per second", 1.0)]
    KilogramPerSecond,
    #[linear("hg/s", "hectograms per second", 0.10000000000000001)]
    HectogramPerSecond,
    #[linear("dag/s", "decagrams per second", 0.01)]
    DecagramPerSecond,
    #[linear("g/s", "grams per second", 0.001)]
    GramPerSecond,
    #[linear("dg/s", "decigrams per second", 0.0001)]
    DecigramPerSecond,
    #[linear("cg/s", "centigrams per second", 1.0000000000000001e-05)]
    CentigramPerSecond,
    #[linear("mg/s", "milligrams per second", 9.9999999999999995e-07)]
    MilligramPerSecond,
    #[linear("µg/s", "micrograms per second", 9.9999999999999986e-10)]
    MicrogramPerSecond,
    #[linear("ng/s", "nanograms per second", 9.9999999999999998e-13)]
    NanogramPerSecond,
    #[linear("pg/s", "picograms per second", 1.0000000000000001e-15)]
    PicogramPerSecond,
    #[linear("fg/s", "femtograms per second", 1.0000000000000001e-18)]
    FemtogramPerSecond,
    #[linear("ag/s", "attograms per second", 1.0000000000000001e-21)]
    AttogramPerSecond,
    #[linear("zg/s", "zeptograms per second", 9.9999999999999992e-25)]
    ZeptogramPerSecond,
    #[linear("yg/s", "yoctograms per second", 9.9999999999999986e-28)]
    YoctogramPerSecond,
    #[linear("kg/min", "kilograms per minute", 0.016666666666666666)]
    KilogramPerMinute,
    #[linear("kg/h", "kilograms per hour", 0.00027777777777777778)]
    KilogramPerHour,
    #[linear("kg/d", "kilograms per day", 1.1574074074074073e-05)]
    KilogramPerDay,
    #[linear("g/min", "grams per minute", 1.6666666666666667e-05)]
    GramPerMinute,
    #[linear("g/h", "grams per hour", 2.7777777777777776e-07)]
    GramPerHour,
    #[linear("g/d", "grams per day", 1.1574074074074074e-08)]
    GramPerDay,
    #[linear("ct/s", "carats per second", 0.00020000000000000001)]
    CaratPerSecond,
    #[linear("gr/s", "grains per second", 6.4798909999999995e-05)]
    GrainPerSecond,
    #[linear("cwt long/s", "hundredweight (long) per second", 50.802349999999997)]
    HundredweightLongPerSecond,
    #[linear("cwt short/s", "hundredweight (short) per second", 45.35924)]
    HundredweightShortPerSecond,
    #[linear("oz/s", "ounces per second", 0.02834952)]
    OuncePerSecond,
    #[linear("oz t/s", "troy ounces per second", 0.031103479999999999)]
    OunceTroyPerSecond,
    #[linear("dwt/s", "pennyweight per second", 0.0015551740000000001)]
    PennyweightPerSecond,
    #[linear("lb/s", "pounds per second", 0.45359240000000001)]
    PoundPerSecond,
    #[linear("lb/min", "pounds per minute", 0.0075598733333333331)]
    PoundPerMinute,
    #[linear("lb/h", "pounds per hour", 0.00012599788888888888)]
    PoundPerHour,
    #[linear("lb/d", "pounds per day", 5.2499120370370372e-06)]
    PoundPerDay,
    #[linear("lb t/s", "troy pounds per second", 0.37324170000000001)]
    PoundTroyPerSecond,
    #[linear("slug/s", "slugs per second", 14.5939)]
    SlugPerSecond,
    #[linear("AT/s", "assay tons per second", 0.029166669999999999)]
    TonAssayPerSecond,
    #[linear("2240 lb/s", "long tons per second", 1016.047)]
    TonLongPerSecond,
    #[linear("2000 lb/s", "short tons per second", 907.18470000000002)]
    TonShortPerSecond,
    #[linear("2000 lb/h", "short tons per hour", 0.25199575000000002)]
    TonShortPerHour,
    #[linear("t/s", "tons per second", 1000.0)]
    TonPerSecond,
    #[linear("t/min", "tons per minute", 16.666666666666664)]
    TonPerMinute,
    #[linear("t/h", "tons per hour", 0.27777777777777779)]
    TonPerHour,
    #[linear("t/d", "tons per day", 0.011574074074074073)]
    TonPerDay,
}

pub type VolumeRate = StaticSIQuantity<
    {
        ISQ {
            m: 3,
            s: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(VolumeRate)]
pub enum VolumeRateUnit {
    #[linear("m³/s", "cubic meters per second", 1.0)]
    CubicMeterPerSecond,
    #[linear("m³/min", "cubic meters per minute", 0.016666666666666666)]
    CubicMeterPerMinute,
    #[linear("m³/h", "cubic meters per hour", 0.00027777777777777778)]
    CubicMeterPerHour,
    #[linear("YL/s", "yottaliters per second", 1e+21)]
    YottaliterPerSecond,
    #[linear("ZL/s", "zettaliters per second", 1e+18)]
    ZettaliterPerSecond,
    #[linear("EL/s", "exaliters per second", 1000000000000000.0)]
    ExaliterPerSecond,
    #[linear("PL/s", "petaliters per second", 1000000000000.0)]
    PetaliterPerSecond,
    #[linear("TL/s", "teraliters per second", 1000000000.0)]
    TeraliterPerSecond,
    #[linear("GL/s", "gigaliters per second", 1000000.0)]
    GigaliterPerSecond,
    #[linear("ML/s", "megaliters per second", 1000.0)]
    MegaliterPerSecond,
    #[linear("kL/s", "kiloliters per second", 1.0)]
    KiloliterPerSecond,
    #[linear("hL/s", "hectoliters per second", 0.10000000000000001)]
    HectoliterPerSecond,
    #[linear("daL/s", "decaliters per second", 0.01)]
    DecaliterPerSecond,
    #[linear("L/s", "liters per second", 0.001)]
    LiterPerSecond,
    #[linear("dL/s", "deciliters per second", 0.0001)]
    DeciliterPerSecond,
    #[linear("cL/s", "centiliters per second", 1.0000000000000001e-05)]
    CentiliterPerSecond,
    #[linear("mL/s", "milliliters per second", 9.9999999999999995e-07)]
    MilliliterPerSecond,
    #[linear("µL/s", "microliters per second", 1.0000000000000001e-09)]
    MicroliterPerSecond,
    #[linear("nL/s", "nanoliters per second", 1.0000000000000002e-12)]
    NanoliterPerSecond,
    #[linear("pL/s", "picoliters per second", 1.0000000000000001e-15)]
    PicoliterPerSecond,
    #[linear("fL/s", "femtoliters per second", 1.0000000000000001e-18)]
    FemtoliterPerSecond,
    #[linear("aL/s", "attoliters per second", 1.0000000000000001e-21)]
    AttoliterPerSecond,
    #[linear("zL/s", "zeptoliters per second", 9.9999999999999992e-25)]
    ZeptoliterPerSecond,
    #[linear("yL/s", "yoctoliters per second", 9.9999999999999986e-28)]
    YoctoliterPerSecond,
    #[linear("L/min", "liters per minute", 1.6666666666666667e-05)]
    LiterPerMinute,
    #[linear("ac · ft/s", "acre-feet per second", 1233.489)]
    AcreFootPerSecond,
    #[linear("bbl/s", "barrels per second", 0.1589873)]
    BarrelPerSecond,
    #[linear("bu/s", "bushels per second", 0.035239069999999997)]
    BushelPerSecond,
    #[linear("cords/s", "cords per second", 3.6245560000000001)]
    CordPerSecond,
    #[linear("ft³/s", "cubic feet per second", 0.028316850000000001)]
    CubicFootPerSecond,
    #[linear("ft³/min", "cubic feet per minute", 0.00047194750000000002)]
    CubicFootPerMinute,
    #[linear("in³/s", "cubic inches per second", 1.6387060000000002e-05)]
    CubicInchPerSecond,
    #[linear("in³/min", "cubic inches per minute", 2.7311766666666671e-07)]
    CubicInchPerMinute,
    #[linear("mi³/s", "cubic miles per second", 4168182000.0)]
    CubicMilePerSecond,
    #[linear("yd³/s", "cubic yards per second", 0.76455490000000004)]
    CubicYardPerSecond,
    #[linear("yd³/min", "cubic yards per minute", 0.012742581666666667)]
    CubicYardPerMinute,
    #[linear("cup/s", "cups per second", 0.00023658819999999999)]
    CupPerSecond,
    #[linear("fl oz/s", "fluid ounces per second", 2.9573529999999999e-05)]
    FluidOuncePerSecond,
    #[linear(
        "fl oz (UK)/s",
        "Imperial fluid ounces per second",
        2.8413060000000001e-05
    )]
    FluidOunceImperialPerSecond,
    #[linear("gal (UK)/s", "Imperial gallons per second", 0.00454609)]
    GallonImperialPerSecond,
    #[linear("gal/s", "gallons per second", 0.0037854120000000002)]
    GallonPerSecond,
    #[linear("gal/min", "gallons per minute", 6.3090200000000008e-05)]
    GallonPerMinute,
    #[linear("gal/d", "gallons per day", 4.3812638888888889e-08)]
    GallonPerDay,
    #[linear("gi (UK)/s", "Imperial gills per second", 0.0001420653)]
    GillImperialPerSecond,
    #[linear("gi/s", "gills per second", 0.0001182941)]
    GillPerSecond,
    #[linear("pk/s", "pecks per second", 0.0088097680000000008)]
    PeckPerSecond,
    #[linear("dry pt/s", "dry pints per second", 0.00055061050000000005)]
    PintDryPerSecond,
    #[linear("liq pt/s", "liquid pints per second", 0.00047317650000000002)]
    PintLiquidPerSecond,
    #[linear("dry qt/s", "dry quarts per second", 0.0011012210000000001)]
    QuartDryPerSecond,
    #[linear("liq qt/s", "liquid quarts per second", 0.0009463529)]
    QuartLiquidPerSecond,
    #[linear("st/s", "steres per second", 1.0)]
    SterePerSecond,
    #[linear("tbsp/s", "tablespoons per second", 1.4786759999999999e-05)]
    TablespoonPerSecond,
    #[linear("tsp/s", "teaspoons per second", 4.9289219999999997e-06)]
    TeaspoonPerSecond,
    #[linear("RT/s", "register tons per second", 2.8316849999999998)]
    RegisterTonPerSecond,
}

pub type MassFlux = StaticSIQuantity<
    {
        ISQ {
            m: -2,
            kg: 1,
            s: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(MassFlux)]
pub enum MassFluxUnit {
    #[linear("kg/(m² · s)", "kilograms per square meter second", 1.0)]
    KilogramPerSquareMeterSecond,
}

pub type MassPerEnergy = StaticSIQuantity<
    {
        ISQ {
            m: -2,
            s: 2,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(MassPerEnergy)]
pub enum MassPerEnergyUnit {
    #[linear("Tg/J", "teragrams per joule", 1000000000.0)]
    TeragramPerJoule,
    #[linear("Gg/J", "gigagrams per joule", 1000000.0)]
    GigagramPerJoule,
    #[linear("Mg/J", "megagrams per joule", 1000.0)]
    MegagramPerJoule,
    #[linear("kg/J", "kilograms per joule", 1.0)]
    KilogramPerJoule,
    #[linear("hg/J", "hectograms per joule", 0.10000000000000001)]
    HectogramPerJoule,
    #[linear("dag/J", "decagrams per joule", 0.01)]
    DecagramPerJoule,
    #[linear("g/J", "grams per joule", 0.001)]
    GramPerJoule,
    #[linear("dg/J", "decigrams per joule", 0.0001)]
    DecigramPerJoule,
    #[linear("cg/J", "centigrams per joule", 1.0000000000000001e-05)]
    CentigramPerJoule,
    #[linear("mg/J", "milligrams per joule", 9.9999999999999995e-07)]
    MilligramPerJoule,
    #[linear("µg/J", "micrograms per joule", 9.9999999999999986e-10)]
    MicrogramPerJoule,
    #[linear("lb/J", "pounds per joule", 0.45359240000000001)]
    PoundPerJoule,
    #[linear("lb/GWh", "pounds per gigawatt hour", 1.2599788888888889e-13)]
    PoundPerGigawattHour,
    #[linear("lb/MWh", "pounds per megawatt hour", 1.2599788888888888e-10)]
    PoundPerMegawattHour,
    #[linear("lb/kWh", "pounds per kilowatt hour", 1.259978888888889e-07)]
    PoundPerKilowattHour,
    #[linear("lb/Wh", "pounds per watt hour", 0.00012599788888888888)]
    PoundPerWattHour,
    #[linear("kg/GWh", "kilograms per gigawatt hour", 2.7777777777777779e-13)]
    KilogramPerGigawattHour,
    #[linear("kg/MWh", "kilograms per megawatt hour", 2.7777777777777777e-10)]
    KilogramPerMegawattHour,
    #[linear("kg/kWh", "kilograms per kilowatt hour", 2.7777777777777776e-07)]
    KilogramPerKilowattHour,
    #[linear("kg/Wh", "kilograms per watt hour", 0.00027777777777777778)]
    KilogramPerWattHour,
    #[linear("g/GWh", "grams per gigawatt hour", 2.777777777777778e-16)]
    GramPerGigawattHour,
    #[linear("g/MWh", "grams per megawatt hour", 2.7777777777777779e-13)]
    GramPerMegawattHour,
    #[linear("g/kWh", "grams per kilowatt hour", 2.7777777777777777e-10)]
    GramPerKilowattHour,
    #[linear("g/Wh", "grams per watt hour", 2.7777777777777776e-07)]
    GramPerWattHour,
}

pub type DiffusionCoefficient = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            s: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(DiffusionCoefficient)]
pub enum DiffusionCoefficientUnit {
    #[linear("m²/s", "square meters per second", 1.0)]
    SquareMeterPerSecond,
    #[linear("cm²/s", "square centimeters per second", 0.0001)]
    SquareCentimeterPerSecond,
    #[linear("mm²/s", "square millimeters per second", 9.9999999999999995e-07)]
    SquareMillimeterPerSecond,
    #[linear("µm²/s", "square micrometers per second", 9.9999999999999998e-13)]
    SquareMicrometerPerSecond,
    #[linear("nm²/s", "square nanometers per second", 1.0000000000000001e-18)]
    SquareNanometerPerSecond,
    #[linear("St", "Stokes", 0.0001)]
    Stokes,
}
