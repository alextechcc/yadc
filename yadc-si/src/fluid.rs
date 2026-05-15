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

#[derive(Copy, Clone, Unit)]
#[dim(DynamicViscosity)]
pub enum DynamicViscosityUnit {
    #[unit(1.0, "Pa · s", "pascal seconds")]
    PascalSecond,
    #[unit(0.001, "mPa · s", "millipascal seconds")]
    MillipascalSecond,
    #[unit(9.9999999999999995e-07, "µPa · s", "micropascal seconds")]
    MicropascalSecond,
    #[unit(0.10000000000000001, "P", "poises")]
    Poise,
    #[unit(0.001, "cP", "centipoises")]
    Centipoise,
    #[unit(1.4881640419947506, "lb/(ft · s)", "pounds per foot second")]
    PoundPerFootSecond,
    #[unit(47.88024934383202, "slug/(ft · s)", "slugs per foot second")]
    SlugPerFootSecond,
    #[unit(0.10000000000000001, "g/(cm · s)", "grams per centimeter second")]
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

#[derive(Copy, Clone, Unit)]
#[dim(KinematicViscosity)]
pub enum KinematicViscosityUnit {
    #[unit(1.0, "m²/s", "square meters per second")]
    SquareMeterPerSecond,
    #[unit(0.0001, "cm²/s", "square centimeters per second")]
    SquareCentimeterPerSecond,
    #[unit(9.9999999999999995e-07, "mm²/s", "square millimeters per second")]
    SquareMillimeterPerSecond,
    #[unit(9.9999999999999998e-13, "µm²/s", "square micrometers per second")]
    SquareMicrometerPerSecond,
    #[unit(1.0000000000000001e-18, "nm²/s", "square nanometers per second")]
    SquareNanometerPerSecond,
    #[unit(0.0001, "St", "stokes")]
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

#[derive(Copy, Clone, Unit)]
#[dim(SurfaceTension)]
pub enum SurfaceTensionUnit {
    #[unit(9.9999999999999998e+23, "YN/m", "yottanewtons per meter")]
    YottanewtonPerMeter,
    #[unit(1e+21, "ZN/m", "zettanewtons per meter")]
    ZettanewtonPerMeter,
    #[unit(1e+18, "EN/m", "exanewtons per meter")]
    ExanewtonPerMeter,
    #[unit(1000000000000000.0, "PN/m", "petanewtons per meter")]
    PetanewtonPerMeter,
    #[unit(1000000000000.0, "TN/m", "teranewtons per meter")]
    TeranewtonPerMeter,
    #[unit(1000000000.0, "GN/m", "giganewtons per meter")]
    GiganewtonPerMeter,
    #[unit(1000000.0, "MN/m", "meganewtons per meter")]
    MeganewtonPerMeter,
    #[unit(1000.0, "kN/m", "kilonewtons per meter")]
    KilonewtonPerMeter,
    #[unit(100.0, "hN/m", "hectonewtons per meter")]
    HectonewtonPerMeter,
    #[unit(10.0, "daN/m", "decanewtons per meter")]
    DecanewtonPerMeter,
    #[unit(1.0, "N/m", "newtons per meter")]
    NewtonPerMeter,
    #[unit(0.10000000000000001, "dN/m", "decinewtons per meter")]
    DecinewtonPerMeter,
    #[unit(0.01, "cN/m", "centinewtons per meter")]
    CentinewtonPerMeter,
    #[unit(0.001, "mN/m", "millinewtons per meter")]
    MillinewtonPerMeter,
    #[unit(9.9999999999999995e-07, "µN/m", "micronewtons per meter")]
    MicronewtonPerMeter,
    #[unit(1.0000000000000001e-09, "nN/m", "nanonewtons per meter")]
    NanonewtonPerMeter,
    #[unit(9.9999999999999998e-13, "pN/m", "piconewtons per meter")]
    PiconewtonPerMeter,
    #[unit(1.0000000000000001e-15, "fN/m", "femtonewtons per meter")]
    FemtonewtonPerMeter,
    #[unit(1.0000000000000001e-18, "aN/m", "attonewtons per meter")]
    AttonewtonPerMeter,
    #[unit(9.9999999999999991e-22, "zN/m", "zeptonewtons per meter")]
    ZeptonewtonPerMeter,
    #[unit(9.9999999999999992e-25, "yN/m", "yoctonewtons per meter")]
    YoctonewtonPerMeter,
    #[unit(9.8066499999999994, "kgf/m", "kilograms-force per meter")]
    KilogramForcePerMeter,
    #[unit(10.945429133858267, "ozf/in", "ounces-force per inch")]
    OunceForcePerInch,
    #[unit(5.4431102362204724, "pdl/in", "poundals per inch")]
    PoundalPerInch,
    #[unit(175.12685039370081, "lbf/in", "pounds-force per inch")]
    PoundForcePerInch,
    #[unit(1.0, "J/m²", "joules per square meter")]
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

#[derive(Copy, Clone, Unit)]
#[dim(CompressibilityCoefficient)]
pub enum CompressibilityCoefficientUnit {
    #[unit(1.0000000000000001e-24, "YPa⁻¹", "per yottapascal")]
    PerYottapascal,
    #[unit(9.9999999999999991e-22, "ZPa⁻¹", "per zettapascal")]
    PerZettapascal,
    #[unit(1.0000000000000001e-18, "EPa⁻¹", "per exapascal")]
    PerExapascal,
    #[unit(1.0000000000000001e-15, "PPa⁻¹", "per petapascal")]
    PerPetapascal,
    #[unit(9.9999999999999998e-13, "TPa⁻¹", "per terapascal")]
    PerTerapascal,
    #[unit(1.0000000000000001e-09, "GPa⁻¹", "per gigapascal")]
    PerGigapascal,
    #[unit(9.9999999999999995e-07, "MPa⁻¹", "per megapascal")]
    PerMegapascal,
    #[unit(0.001, "kPa⁻¹", "per kilopascal")]
    PerKilopascal,
    #[unit(0.01, "hPa⁻¹", "per hectopascal")]
    PerHectopascal,
    #[unit(0.10000000000000001, "daPa⁻¹", "per decapascal")]
    PerDecapascal,
    #[unit(1.0, "Pa⁻¹", "per pascal")]
    PerPascal,
    #[unit(10.0, "dPa⁻¹", "per decipascal")]
    PerDecipascal,
    #[unit(100.0, "cPa⁻¹", "per centipascal")]
    PerCentipascal,
    #[unit(1000.0, "mPa⁻¹", "per millipascal")]
    PerMillipascal,
    #[unit(1000000.0, "µPa⁻¹", "per micropascal")]
    PerMicropascal,
    #[unit(999999999.99999988, "nPa⁻¹", "per nanopascal")]
    PerNanopascal,
    #[unit(1000000000000.0, "pPa⁻¹", "per picopascal")]
    PerPicopascal,
    #[unit(999999999999999.88, "fPa⁻¹", "per femtopascal")]
    PerFemtopascal,
    #[unit(9.9999999999999987e+17, "aPa⁻¹", "per attopascal")]
    PerAttopascal,
    #[unit(1.0000000000000001e+21, "zPa⁻¹", "per zeptopascal")]
    PerZeptopascal,
    #[unit(1.0000000000000001e+24, "yPa⁻¹", "per yoctopascal")]
    PerYoctopascal,
    #[unit(9.8692326671601285e-06, "atm⁻¹", "per atmosphere")]
    PerAtmosphere,
    #[unit(1.0197162129779282e-05, "at⁻¹", "per atmosphere (technical)")]
    PerAtmosphereTechnical,
    #[unit(1.0000000000000001e-05, "bar⁻¹", "per bar")]
    PerBar,
    #[unit(0.01, "mbar⁻¹", "per millibar")]
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

#[derive(Copy, Clone, Unit)]
#[dim(MassRate)]
pub enum MassRateUnit {
    #[unit(1e+21, "Yg/s", "yottagrams per second")]
    YottagramPerSecond,
    #[unit(1e+18, "Zg/s", "zettagrams per second")]
    ZettagramPerSecond,
    #[unit(1000000000000000.0, "Eg/s", "exagrams per second")]
    ExagramPerSecond,
    #[unit(1000000000000.0, "Pg/s", "petagrams per second")]
    PetagramPerSecond,
    #[unit(1000000000.0, "Tg/s", "teragrams per second")]
    TeragramPerSecond,
    #[unit(1000000.0, "Gg/s", "gigagrams per second")]
    GigagramPerSecond,
    #[unit(1000.0, "Mg/s", "megagrams per second")]
    MegagramPerSecond,
    #[unit(1.0, "kg/s", "kilograms per second")]
    KilogramPerSecond,
    #[unit(0.10000000000000001, "hg/s", "hectograms per second")]
    HectogramPerSecond,
    #[unit(0.01, "dag/s", "decagrams per second")]
    DecagramPerSecond,
    #[unit(0.001, "g/s", "grams per second")]
    GramPerSecond,
    #[unit(0.0001, "dg/s", "decigrams per second")]
    DecigramPerSecond,
    #[unit(1.0000000000000001e-05, "cg/s", "centigrams per second")]
    CentigramPerSecond,
    #[unit(9.9999999999999995e-07, "mg/s", "milligrams per second")]
    MilligramPerSecond,
    #[unit(9.9999999999999986e-10, "µg/s", "micrograms per second")]
    MicrogramPerSecond,
    #[unit(9.9999999999999998e-13, "ng/s", "nanograms per second")]
    NanogramPerSecond,
    #[unit(1.0000000000000001e-15, "pg/s", "picograms per second")]
    PicogramPerSecond,
    #[unit(1.0000000000000001e-18, "fg/s", "femtograms per second")]
    FemtogramPerSecond,
    #[unit(1.0000000000000001e-21, "ag/s", "attograms per second")]
    AttogramPerSecond,
    #[unit(9.9999999999999992e-25, "zg/s", "zeptograms per second")]
    ZeptogramPerSecond,
    #[unit(9.9999999999999986e-28, "yg/s", "yoctograms per second")]
    YoctogramPerSecond,
    #[unit(0.016666666666666666, "kg/min", "kilograms per minute")]
    KilogramPerMinute,
    #[unit(0.00027777777777777778, "kg/h", "kilograms per hour")]
    KilogramPerHour,
    #[unit(1.1574074074074073e-05, "kg/d", "kilograms per day")]
    KilogramPerDay,
    #[unit(1.6666666666666667e-05, "g/min", "grams per minute")]
    GramPerMinute,
    #[unit(2.7777777777777776e-07, "g/h", "grams per hour")]
    GramPerHour,
    #[unit(1.1574074074074074e-08, "g/d", "grams per day")]
    GramPerDay,
    #[unit(0.00020000000000000001, "ct/s", "carats per second")]
    CaratPerSecond,
    #[unit(6.4798909999999995e-05, "gr/s", "grains per second")]
    GrainPerSecond,
    #[unit(50.802349999999997, "cwt long/s", "hundredweight (long) per second")]
    HundredweightLongPerSecond,
    #[unit(45.35924, "cwt short/s", "hundredweight (short) per second")]
    HundredweightShortPerSecond,
    #[unit(0.02834952, "oz/s", "ounces per second")]
    OuncePerSecond,
    #[unit(0.031103479999999999, "oz t/s", "troy ounces per second")]
    OunceTroyPerSecond,
    #[unit(0.0015551740000000001, "dwt/s", "pennyweight per second")]
    PennyweightPerSecond,
    #[unit(0.45359240000000001, "lb/s", "pounds per second")]
    PoundPerSecond,
    #[unit(0.0075598733333333331, "lb/min", "pounds per minute")]
    PoundPerMinute,
    #[unit(0.00012599788888888888, "lb/h", "pounds per hour")]
    PoundPerHour,
    #[unit(5.2499120370370372e-06, "lb/d", "pounds per day")]
    PoundPerDay,
    #[unit(0.37324170000000001, "lb t/s", "troy pounds per second")]
    PoundTroyPerSecond,
    #[unit(14.5939, "slug/s", "slugs per second")]
    SlugPerSecond,
    #[unit(0.029166669999999999, "AT/s", "assay tons per second")]
    TonAssayPerSecond,
    #[unit(1016.047, "2240 lb/s", "long tons per second")]
    TonLongPerSecond,
    #[unit(907.18470000000002, "2000 lb/s", "short tons per second")]
    TonShortPerSecond,
    #[unit(0.25199575000000002, "2000 lb/h", "short tons per hour")]
    TonShortPerHour,
    #[unit(1000.0, "t/s", "tons per second")]
    TonPerSecond,
    #[unit(16.666666666666664, "t/min", "tons per minute")]
    TonPerMinute,
    #[unit(0.27777777777777779, "t/h", "tons per hour")]
    TonPerHour,
    #[unit(0.011574074074074073, "t/d", "tons per day")]
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

#[derive(Copy, Clone, Unit)]
#[dim(VolumeRate)]
pub enum VolumeRateUnit {
    #[unit(1.0, "m³/s", "cubic meters per second")]
    CubicMeterPerSecond,
    #[unit(0.016666666666666666, "m³/min", "cubic meters per minute")]
    CubicMeterPerMinute,
    #[unit(0.00027777777777777778, "m³/h", "cubic meters per hour")]
    CubicMeterPerHour,
    #[unit(1e+21, "YL/s", "yottaliters per second")]
    YottaliterPerSecond,
    #[unit(1e+18, "ZL/s", "zettaliters per second")]
    ZettaliterPerSecond,
    #[unit(1000000000000000.0, "EL/s", "exaliters per second")]
    ExaliterPerSecond,
    #[unit(1000000000000.0, "PL/s", "petaliters per second")]
    PetaliterPerSecond,
    #[unit(1000000000.0, "TL/s", "teraliters per second")]
    TeraliterPerSecond,
    #[unit(1000000.0, "GL/s", "gigaliters per second")]
    GigaliterPerSecond,
    #[unit(1000.0, "ML/s", "megaliters per second")]
    MegaliterPerSecond,
    #[unit(1.0, "kL/s", "kiloliters per second")]
    KiloliterPerSecond,
    #[unit(0.10000000000000001, "hL/s", "hectoliters per second")]
    HectoliterPerSecond,
    #[unit(0.01, "daL/s", "decaliters per second")]
    DecaliterPerSecond,
    #[unit(0.001, "L/s", "liters per second")]
    LiterPerSecond,
    #[unit(0.0001, "dL/s", "deciliters per second")]
    DeciliterPerSecond,
    #[unit(1.0000000000000001e-05, "cL/s", "centiliters per second")]
    CentiliterPerSecond,
    #[unit(9.9999999999999995e-07, "mL/s", "milliliters per second")]
    MilliliterPerSecond,
    #[unit(1.0000000000000001e-09, "µL/s", "microliters per second")]
    MicroliterPerSecond,
    #[unit(1.0000000000000002e-12, "nL/s", "nanoliters per second")]
    NanoliterPerSecond,
    #[unit(1.0000000000000001e-15, "pL/s", "picoliters per second")]
    PicoliterPerSecond,
    #[unit(1.0000000000000001e-18, "fL/s", "femtoliters per second")]
    FemtoliterPerSecond,
    #[unit(1.0000000000000001e-21, "aL/s", "attoliters per second")]
    AttoliterPerSecond,
    #[unit(9.9999999999999992e-25, "zL/s", "zeptoliters per second")]
    ZeptoliterPerSecond,
    #[unit(9.9999999999999986e-28, "yL/s", "yoctoliters per second")]
    YoctoliterPerSecond,
    #[unit(1.6666666666666667e-05, "L/min", "liters per minute")]
    LiterPerMinute,
    #[unit(1233.489, "ac · ft/s", "acre-feet per second")]
    AcreFootPerSecond,
    #[unit(0.1589873, "bbl/s", "barrels per second")]
    BarrelPerSecond,
    #[unit(0.035239069999999997, "bu/s", "bushels per second")]
    BushelPerSecond,
    #[unit(3.6245560000000001, "cords/s", "cords per second")]
    CordPerSecond,
    #[unit(0.028316850000000001, "ft³/s", "cubic feet per second")]
    CubicFootPerSecond,
    #[unit(0.00047194750000000002, "ft³/min", "cubic feet per minute")]
    CubicFootPerMinute,
    #[unit(1.6387060000000002e-05, "in³/s", "cubic inches per second")]
    CubicInchPerSecond,
    #[unit(2.7311766666666671e-07, "in³/min", "cubic inches per minute")]
    CubicInchPerMinute,
    #[unit(4168182000.0, "mi³/s", "cubic miles per second")]
    CubicMilePerSecond,
    #[unit(0.76455490000000004, "yd³/s", "cubic yards per second")]
    CubicYardPerSecond,
    #[unit(0.012742581666666667, "yd³/min", "cubic yards per minute")]
    CubicYardPerMinute,
    #[unit(0.00023658819999999999, "cup/s", "cups per second")]
    CupPerSecond,
    #[unit(2.9573529999999999e-05, "fl oz/s", "fluid ounces per second")]
    FluidOuncePerSecond,
    #[unit(
        2.8413060000000001e-05,
        "fl oz (UK)/s",
        "Imperial fluid ounces per second"
    )]
    FluidOunceImperialPerSecond,
    #[unit(0.00454609, "gal (UK)/s", "Imperial gallons per second")]
    GallonImperialPerSecond,
    #[unit(0.0037854120000000002, "gal/s", "gallons per second")]
    GallonPerSecond,
    #[unit(6.3090200000000008e-05, "gal/min", "gallons per minute")]
    GallonPerMinute,
    #[unit(4.3812638888888889e-08, "gal/d", "gallons per day")]
    GallonPerDay,
    #[unit(0.0001420653, "gi (UK)/s", "Imperial gills per second")]
    GillImperialPerSecond,
    #[unit(0.0001182941, "gi/s", "gills per second")]
    GillPerSecond,
    #[unit(0.0088097680000000008, "pk/s", "pecks per second")]
    PeckPerSecond,
    #[unit(0.00055061050000000005, "dry pt/s", "dry pints per second")]
    PintDryPerSecond,
    #[unit(0.00047317650000000002, "liq pt/s", "liquid pints per second")]
    PintLiquidPerSecond,
    #[unit(0.0011012210000000001, "dry qt/s", "dry quarts per second")]
    QuartDryPerSecond,
    #[unit(0.0009463529, "liq qt/s", "liquid quarts per second")]
    QuartLiquidPerSecond,
    #[unit(1.0, "st/s", "steres per second")]
    SterePerSecond,
    #[unit(1.4786759999999999e-05, "tbsp/s", "tablespoons per second")]
    TablespoonPerSecond,
    #[unit(4.9289219999999997e-06, "tsp/s", "teaspoons per second")]
    TeaspoonPerSecond,
    #[unit(2.8316849999999998, "RT/s", "register tons per second")]
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

#[derive(Copy, Clone, Unit)]
#[dim(MassFlux)]
pub enum MassFluxUnit {
    #[unit(1.0, "kg/(m² · s)", "kilograms per square meter second")]
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

#[derive(Copy, Clone, Unit)]
#[dim(MassPerEnergy)]
pub enum MassPerEnergyUnit {
    #[unit(1000000000.0, "Tg/J", "teragrams per joule")]
    TeragramPerJoule,
    #[unit(1000000.0, "Gg/J", "gigagrams per joule")]
    GigagramPerJoule,
    #[unit(1000.0, "Mg/J", "megagrams per joule")]
    MegagramPerJoule,
    #[unit(1.0, "kg/J", "kilograms per joule")]
    KilogramPerJoule,
    #[unit(0.10000000000000001, "hg/J", "hectograms per joule")]
    HectogramPerJoule,
    #[unit(0.01, "dag/J", "decagrams per joule")]
    DecagramPerJoule,
    #[unit(0.001, "g/J", "grams per joule")]
    GramPerJoule,
    #[unit(0.0001, "dg/J", "decigrams per joule")]
    DecigramPerJoule,
    #[unit(1.0000000000000001e-05, "cg/J", "centigrams per joule")]
    CentigramPerJoule,
    #[unit(9.9999999999999995e-07, "mg/J", "milligrams per joule")]
    MilligramPerJoule,
    #[unit(9.9999999999999986e-10, "µg/J", "micrograms per joule")]
    MicrogramPerJoule,
    #[unit(0.45359240000000001, "lb/J", "pounds per joule")]
    PoundPerJoule,
    #[unit(1.2599788888888889e-13, "lb/GWh", "pounds per gigawatt hour")]
    PoundPerGigawattHour,
    #[unit(1.2599788888888888e-10, "lb/MWh", "pounds per megawatt hour")]
    PoundPerMegawattHour,
    #[unit(1.259978888888889e-07, "lb/kWh", "pounds per kilowatt hour")]
    PoundPerKilowattHour,
    #[unit(0.00012599788888888888, "lb/Wh", "pounds per watt hour")]
    PoundPerWattHour,
    #[unit(2.7777777777777779e-13, "kg/GWh", "kilograms per gigawatt hour")]
    KilogramPerGigawattHour,
    #[unit(2.7777777777777777e-10, "kg/MWh", "kilograms per megawatt hour")]
    KilogramPerMegawattHour,
    #[unit(2.7777777777777776e-07, "kg/kWh", "kilograms per kilowatt hour")]
    KilogramPerKilowattHour,
    #[unit(0.00027777777777777778, "kg/Wh", "kilograms per watt hour")]
    KilogramPerWattHour,
    #[unit(2.777777777777778e-16, "g/GWh", "grams per gigawatt hour")]
    GramPerGigawattHour,
    #[unit(2.7777777777777779e-13, "g/MWh", "grams per megawatt hour")]
    GramPerMegawattHour,
    #[unit(2.7777777777777777e-10, "g/kWh", "grams per kilowatt hour")]
    GramPerKilowattHour,
    #[unit(2.7777777777777776e-07, "g/Wh", "grams per watt hour")]
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

#[derive(Copy, Clone, Unit)]
#[dim(DiffusionCoefficient)]
pub enum DiffusionCoefficientUnit {
    #[unit(1.0, "m²/s", "square meters per second")]
    SquareMeterPerSecond,
    #[unit(0.0001, "cm²/s", "square centimeters per second")]
    SquareCentimeterPerSecond,
    #[unit(9.9999999999999995e-07, "mm²/s", "square millimeters per second")]
    SquareMillimeterPerSecond,
    #[unit(9.9999999999999998e-13, "µm²/s", "square micrometers per second")]
    SquareMicrometerPerSecond,
    #[unit(1.0000000000000001e-18, "nm²/s", "square nanometers per second")]
    SquareNanometerPerSecond,
    #[unit(0.0001, "St", "Stokes")]
    Stokes,
}
