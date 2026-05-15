use crate::{ISQ, Kind, StaticSIQuantity};
use yadc::{Dimension, Unit};

pub type ThermodynamicTemperature = StaticSIQuantity<
    {
        ISQ {
            K: 1,
            kind: Kind::Affine,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(ThermodynamicTemperature)]
pub enum ThermodynamicTemperatureUnit {
    #[linear("K", "kelvin", 1.0)]
    Kelvin,
    #[affine("°C", "degrees Celsius", 1.0, 273.15)]
    Celsius,
    #[affine("°F", "degrees Fahrenheit", 5.0 / 9.0, 459.67)]
    Fahrenheit,
    #[linear("°R", "degrees Rankine", 5.0 / 9.0)]
    Rankine,
}

pub type TemperatureInterval = StaticSIQuantity<{ ISQ { K: 1, ..ISQ::ZERO } }>;

#[derive(Unit, Copy, Clone)]
#[dim(TemperatureInterval)]
pub enum TemperatureIntervalUnit {
    #[linear("K", "kelvin", 1.0)]
    Kelvin,
    #[linear("°C", "degrees Celsius", 1.0)]
    Celsius,
    #[linear("°F", "degrees Fahrenheit", 5.0 / 9.0)]
    Fahrenheit,
    #[linear("°R", "degrees Rankine", 5.0 / 9.0)]
    Rankine,
}

pub type TemperatureCoefficient = StaticSIQuantity<{ ISQ { K: -1, ..ISQ::ZERO } }>;

#[derive(Unit, Copy, Clone)]
#[dim(TemperatureCoefficient)]
pub enum TemperatureCoefficientUnit {
    #[linear("K⁻¹", "per kelvin", 1.0)]
    PerKelvin,
    #[linear("ppm/K", "ppm per kelvin", 9.9999999999999995e-07)]
    PpmPerKelvin,
    #[linear("ppm/°C", "ppm per degree Celsius", 9.9999999999999995e-07)]
    PpmPerDegreeCelsius,
}

pub type TemperatureGradient = StaticSIQuantity<
    {
        ISQ {
            m: -1,
            K: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(TemperatureGradient)]
pub enum TemperatureGradientUnit {
    #[linear("K/km", "kelvins per kilometer", 0.001)]
    KelvinPerKilometer,
    #[linear("K/hm", "kelvins per hectometer", 0.01)]
    KelvinPerHectometer,
    #[linear("K/m", "kelvins per meter", 1.0)]
    KelvinPerMeter,
    #[linear("K/cm", "kelvins per centimeter", 100.0)]
    KelvinPerCentimeter,
    #[linear("K/mm", "kelvins per millimeter", 1000.0)]
    KelvinPerMillimeter,
    #[linear("K/µm", "kelvins per micrometer", 1000000.0)]
    KelvinPerMicrometer,
}

pub type HeatCapacity = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -2,
            K: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(HeatCapacity)]
pub enum HeatCapacityUnit {
    #[linear(
        "Yg · m²/(s² · K)",
        "yottagram square meters per second squared kelvin",
        1e+21
    )]
    YottagramSquareMeterPerSecondSquaredKelvin,
    #[linear(
        "Zg · m²/(s² · K)",
        "zettagram square meters per second squared kelvin",
        1e+18
    )]
    ZettagramSquareMeterPerSecondSquaredKelvin,
    #[linear(
        "Eg · m²/(s² · K)",
        "exagram square meters per second squared kelvin",
        1000000000000000.0
    )]
    ExagramSquareMeterPerSecondSquaredKelvin,
    #[linear(
        "Pg · m²/(s² · K)",
        "petagram square meters per second squared kelvin",
        1000000000000.0
    )]
    PetagramSquareMeterPerSecondSquaredKelvin,
    #[linear(
        "Tg · m²/(s² · K)",
        "teragram square meters per second squared kelvin",
        1000000000.0
    )]
    TeragramSquareMeterPerSecondSquaredKelvin,
    #[linear(
        "Gg · m²/(s² · K)",
        "gigagram square meters per second squared kelvin",
        1000000.0
    )]
    GigagramSquareMeterPerSecondSquaredKelvin,
    #[linear(
        "Mg · m²/(s² · K)",
        "megagram square meters per second squared kelvin",
        1000.0
    )]
    MegagramSquareMeterPerSecondSquaredKelvin,
    #[linear(
        "kg · m²/(s² · K)",
        "kilogram square meters per second squared kelvin",
        1.0
    )]
    KilogramSquareMeterPerSecondSquaredKelvin,
    #[linear(
        "hg · m²/(s² · K)",
        "hectogram square meters per second squared kelvin",
        0.10000000000000001
    )]
    HectogramSquareMeterPerSecondSquaredKelvin,
    #[linear(
        "dag · m²/(s² · K)",
        "decagram square meters per second squared kelvin",
        0.01
    )]
    DecagramSquareMeterPerSecondSquaredKelvin,
    #[linear(
        "g · m²/(s² · K)",
        "gram square meters per second squared kelvin",
        0.001
    )]
    GramSquareMeterPerSecondSquaredKelvin,
    #[linear(
        "dg · m²/(s² · K)",
        "decigram square meters per second squared kelvin",
        0.0001
    )]
    DecigramSquareMeterPerSecondSquaredKelvin,
    #[linear(
        "cg · m²/(s² · K)",
        "centigram square meters per second squared kelvin",
        1.0000000000000001e-05
    )]
    CentigramSquareMeterPerSecondSquaredKelvin,
    #[linear(
        "mg · m²/(s² · K)",
        "milligram square meters per second squared kelvin",
        9.9999999999999995e-07
    )]
    MilligramSquareMeterPerSecondSquaredKelvin,
    #[linear(
        "µg · m²/(s² · K)",
        "microgram square meters per second squared kelvin",
        9.9999999999999986e-10
    )]
    MicrogramSquareMeterPerSecondSquaredKelvin,
    #[linear(
        "ng · m²/(s² · K)",
        "nanogram square meters per second squared kelvin",
        9.9999999999999998e-13
    )]
    NanogramSquareMeterPerSecondSquaredKelvin,
    #[linear(
        "pg · m²/(s² · K)",
        "picogram square meters per second squared kelvin",
        1.0000000000000001e-15
    )]
    PicogramSquareMeterPerSecondSquaredKelvin,
    #[linear(
        "fg · m²/(s² · K)",
        "femtogram square meters per second squared kelvin",
        1.0000000000000001e-18
    )]
    FemtogramSquareMeterPerSecondSquaredKelvin,
    #[linear(
        "ag · m²/(s² · K)",
        "attogram square meters per second squared kelvin",
        1.0000000000000001e-21
    )]
    AttogramSquareMeterPerSecondSquaredKelvin,
    #[linear(
        "zg · m²/(s² · K)",
        "zeptogram square meters per second squared kelvin",
        9.9999999999999992e-25
    )]
    ZeptogramSquareMeterPerSecondSquaredKelvin,
    #[linear(
        "yg · m²/(s² · K)",
        "yoctogram square meters per second squared kelvin",
        9.9999999999999986e-28
    )]
    YoctogramSquareMeterPerSecondSquaredKelvin,
    #[linear("YJ/K", "yottajoules per kelvin", 9.9999999999999998e+23)]
    YottajoulePerKelvin,
    #[linear("ZJ/K", "zettajoules per kelvin", 1e+21)]
    ZettajoulePerKelvin,
    #[linear("EJ/K", "exajoules per kelvin", 1e+18)]
    ExajoulePerKelvin,
    #[linear("PJ/K", "petajoules per kelvin", 1000000000000000.0)]
    PetajoulePerKelvin,
    #[linear("TJ/K", "terajoules per kelvin", 1000000000000.0)]
    TerajoulePerKelvin,
    #[linear("GJ/K", "gigajoules per kelvin", 1000000000.0)]
    GigajoulePerKelvin,
    #[linear("MJ/K", "megajoules per kelvin", 1000000.0)]
    MegajoulePerKelvin,
    #[linear("kJ/K", "kilojoules per kelvin", 1000.0)]
    KilojoulePerKelvin,
    #[linear("hJ/K", "hectojoules per kelvin", 100.0)]
    HectojoulePerKelvin,
    #[linear("daJ/K", "decajoules per kelvin", 10.0)]
    DecajoulePerKelvin,
    #[linear("J/K", "joules per kelvin", 1.0)]
    JoulePerKelvin,
    #[linear("dJ/K", "decijoules per kelvin", 0.10000000000000001)]
    DecijoulePerKelvin,
    #[linear("cJ/K", "centijoules per kelvin", 0.01)]
    CentijoulePerKelvin,
    #[linear("mJ/K", "millijoules per kelvin", 0.001)]
    MillijoulePerKelvin,
    #[linear("µJ/K", "microjoules per kelvin", 9.9999999999999995e-07)]
    MicrojoulePerKelvin,
    #[linear("nJ/K", "nanojoules per kelvin", 1.0000000000000001e-09)]
    NanojoulePerKelvin,
    #[linear("pJ/K", "picojoules per kelvin", 9.9999999999999998e-13)]
    PicojoulePerKelvin,
    #[linear("fJ/K", "femtojoules per kelvin", 1.0000000000000001e-15)]
    FemtojoulePerKelvin,
    #[linear("aJ/K", "attojoules per kelvin", 1.0000000000000001e-18)]
    AttojoulePerKelvin,
    #[linear("zJ/K", "zeptojoules per kelvin", 9.9999999999999991e-22)]
    ZeptojoulePerKelvin,
    #[linear("yJ/K", "yoctojoules per kelvin", 9.9999999999999992e-25)]
    YoctojoulePerKelvin,
    #[linear("kJ/°C", "kilojoules per degree celsius", 1000.0)]
    KilojoulePerDegreeCelsius,
    #[linear("J/°C", "joules per degree celsius", 1.0)]
    JoulePerDegreeCelsius,
    #[linear("mJ/°C", "millijoules per degree celsius", 0.001)]
    MillijoulePerDegreeCelsius,
    #[linear(
        "Btu/°F",
        "British thermal units per degree Fahrenheit",
        1897.8299999999999
    )]
    BtuPerDegreeFahrenheit,
    #[linear(
        "Btu (IT)/°F",
        "British thermal units (IT) per degree Fahrenheit",
        1899.1007999999999
    )]
    BtuItPerDegreeFahrenheit,
    #[linear("k", "Boltzmann constants", 1.3806490000000001e-23)]
    BoltzmannConstant,
    #[linear("cal/K", "calories per kelvin", 4.1840000000000002)]
    CaloriePerKelvin,
}

pub type HeatFluxDensity = StaticSIQuantity<
    {
        ISQ {
            kg: 1,
            s: -3,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(HeatFluxDensity)]
pub enum HeatFluxDensityUnit {
    #[linear("YW/m²", "yottawatts per square meter", 9.9999999999999998e+23)]
    YottawattPerSquareMeter,
    #[linear("ZW/m²", "zettawatts per square meter", 1e+21)]
    ZettawattPerSquareMeter,
    #[linear("EW/m²", "exawatts per square meter", 1e+18)]
    ExawattPerSquareMeter,
    #[linear("PW/m²", "petawatts per square meter", 1000000000000000.0)]
    PetawattPerSquareMeter,
    #[linear("TW/m²", "terawatts per square meter", 1000000000000.0)]
    TerawattPerSquareMeter,
    #[linear("GW/m²", "gigawatts per square meter", 1000000000.0)]
    GigawattPerSquareMeter,
    #[linear("MW/m²", "megawatts per square meter", 1000000.0)]
    MegawattPerSquareMeter,
    #[linear("kW/m²", "kilowatts per square meter", 1000.0)]
    KilowattPerSquareMeter,
    #[linear("hW/m²", "hectowatts per square meter", 100.0)]
    HectowattPerSquareMeter,
    #[linear("daW/m²", "decawatts per square meter", 10.0)]
    DecawattPerSquareMeter,
    #[linear("W/m²", "watts per square meter", 1.0)]
    WattPerSquareMeter,
    #[linear("dW/m²", "deciwatts per square meter", 0.10000000000000001)]
    DeciwattPerSquareMeter,
    #[linear("cW/m²", "centiwatts per square meter", 0.01)]
    CentiwattPerSquareMeter,
    #[linear("mW/m²", "milliwatts per square meter", 0.001)]
    MilliwattPerSquareMeter,
    #[linear("µW/m²", "microwatts per square meter", 9.9999999999999995e-07)]
    MicrowattPerSquareMeter,
    #[linear("nW/m²", "nanowatts per square meter", 1.0000000000000001e-09)]
    NanowattPerSquareMeter,
    #[linear("pW/m²", "picowatts per square meter", 9.9999999999999998e-13)]
    PicowattPerSquareMeter,
    #[linear("fW/m²", "femtowatts per square meter", 1.0000000000000001e-15)]
    FemtowattPerSquareMeter,
    #[linear("aW/m²", "attowatts per square meter", 1.0000000000000001e-18)]
    AttowattPerSquareMeter,
    #[linear("zW/m²", "zeptowatts per square meter", 9.9999999999999991e-22)]
    ZeptowattPerSquareMeter,
    #[linear("yW/m²", "yoctowatts per square meter", 9.9999999999999992e-25)]
    YoctowattPerSquareMeter,
}

pub type HeatTransfer = StaticSIQuantity<
    {
        ISQ {
            kg: 1,
            s: -3,
            K: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(HeatTransfer)]
pub enum HeatTransferUnit {
    #[linear("Yg/(s³ · K)", "yottagrams per second cubed kelvin", 1e+21)]
    YottagramPerSecondCubedKelvin,
    #[linear("Zg/(s³ · K)", "zettagrams per second cubed kelvin", 1e+18)]
    ZettagramPerSecondCubedKelvin,
    #[linear(
        "Eg/(s³ · K)",
        "exagrams per second cubed kelvin",
        1000000000000000.0
    )]
    ExagramPerSecondCubedKelvin,
    #[linear("Pg/(s³ · K)", "petagrams per second cubed kelvin", 1000000000000.0)]
    PetagramPerSecondCubedKelvin,
    #[linear("Tg/(s³ · K)", "teragrams per second cubed kelvin", 1000000000.0)]
    TeragramPerSecondCubedKelvin,
    #[linear("Gg/(s³ · K)", "gigagrams per second cubed kelvin", 1000000.0)]
    GigagramPerSecondCubedKelvin,
    #[linear("Mg/(s³ · K)", "megagrams per second cubed kelvin", 1000.0)]
    MegagramPerSecondCubedKelvin,
    #[linear("kg/(s³ · K)", "kilograms per second cubed kelvin", 1.0)]
    KilogramPerSecondCubedKelvin,
    #[linear(
        "hg/(s³ · K)",
        "hectograms per second cubed kelvin",
        0.10000000000000001
    )]
    HectogramPerSecondCubedKelvin,
    #[linear("dag/(s³ · K)", "decagrams per second cubed kelvin", 0.01)]
    DecagramPerSecondCubedKelvin,
    #[linear("g/(s³ · K)", "grams per second cubed kelvin", 0.001)]
    GramPerSecondCubedKelvin,
    #[linear("dg/(s³ · K)", "decigrams per second cubed kelvin", 0.0001)]
    DecigramPerSecondCubedKelvin,
    #[linear(
        "cg/(s³ · K)",
        "centigrams per second cubed kelvin",
        1.0000000000000001e-05
    )]
    CentigramPerSecondCubedKelvin,
    #[linear(
        "mg/(s³ · K)",
        "milligrams per second cubed kelvin",
        9.9999999999999995e-07
    )]
    MilligramPerSecondCubedKelvin,
    #[linear(
        "µg/(s³ · K)",
        "micrograms per second cubed kelvin",
        9.9999999999999986e-10
    )]
    MicrogramPerSecondCubedKelvin,
    #[linear(
        "ng/(s³ · K)",
        "nanograms per second cubed kelvin",
        9.9999999999999998e-13
    )]
    NanogramPerSecondCubedKelvin,
    #[linear(
        "pg/(s³ · K)",
        "picograms per second cubed kelvin",
        1.0000000000000001e-15
    )]
    PicogramPerSecondCubedKelvin,
    #[linear(
        "fg/(s³ · K)",
        "femtograms per second cubed kelvin",
        1.0000000000000001e-18
    )]
    FemtogramPerSecondCubedKelvin,
    #[linear(
        "ag/(s³ · K)",
        "attograms per second cubed kelvin",
        1.0000000000000001e-21
    )]
    AttogramPerSecondCubedKelvin,
    #[linear(
        "zg/(s³ · K)",
        "zeptograms per second cubed kelvin",
        9.9999999999999992e-25
    )]
    ZeptogramPerSecondCubedKelvin,
    #[linear(
        "yg/(s³ · K)",
        "yoctograms per second cubed kelvin",
        9.9999999999999986e-28
    )]
    YoctogramPerSecondCubedKelvin,
    #[linear("kg/(s³ · °C)", "kilograms per second cubed degree celsius", 1.0)]
    KilogramPerSecondCubedDegreeCelsius,
    #[linear(
        "YW/(m² · K)",
        "yottawatts per square meter kelvin",
        9.9999999999999998e+23
    )]
    YottawattPerSquareMeterKelvin,
    #[linear("ZW/(m² · K)", "zettawatts per square meter kelvin", 1e+21)]
    ZettawattPerSquareMeterKelvin,
    #[linear("EW/(m² · K)", "exawatts per square meter kelvin", 1e+18)]
    ExawattPerSquareMeterKelvin,
    #[linear(
        "PW/(m² · K)",
        "petawatts per square meter kelvin",
        1000000000000000.0
    )]
    PetawattPerSquareMeterKelvin,
    #[linear("TW/(m² · K)", "terawatts per square meter kelvin", 1000000000000.0)]
    TerawattPerSquareMeterKelvin,
    #[linear("GW/(m² · K)", "gigawatts per square meter kelvin", 1000000000.0)]
    GigawattPerSquareMeterKelvin,
    #[linear("MW/(m² · K)", "megawatts per square meter kelvin", 1000000.0)]
    MegawattPerSquareMeterKelvin,
    #[linear("kW/(m² · K)", "kilowatts per square meter kelvin", 1000.0)]
    KilowattPerSquareMeterKelvin,
    #[linear("hW/(m² · K)", "hectowatts per square meter kelvin", 100.0)]
    HectowattPerSquareMeterKelvin,
    #[linear("daW/(m² · K)", "decawatts per square meter kelvin", 10.0)]
    DecawattPerSquareMeterKelvin,
    #[linear("W/(m² · K)", "watts per square meter kelvin", 1.0)]
    WattPerSquareMeterKelvin,
    #[linear(
        "dW/(m² · K)",
        "deciwatts per square meter kelvin",
        0.10000000000000001
    )]
    DeciwattPerSquareMeterKelvin,
    #[linear("cW/(m² · K)", "centiwatts per square meter kelvin", 0.01)]
    CentiwattPerSquareMeterKelvin,
    #[linear("mW/(m² · K)", "milliwatts per square meter kelvin", 0.001)]
    MilliwattPerSquareMeterKelvin,
    #[linear(
        "µW/(m² · K)",
        "microwatts per square meter kelvin",
        9.9999999999999995e-07
    )]
    MicrowattPerSquareMeterKelvin,
    #[linear(
        "nW/(m² · K)",
        "nanowatts per square meter kelvin",
        1.0000000000000001e-09
    )]
    NanowattPerSquareMeterKelvin,
    #[linear(
        "pW/(m² · K)",
        "picowatts per square meter kelvin",
        9.9999999999999998e-13
    )]
    PicowattPerSquareMeterKelvin,
    #[linear(
        "fW/(m² · K)",
        "femtowatts per square meter kelvin",
        1.0000000000000001e-15
    )]
    FemtowattPerSquareMeterKelvin,
    #[linear(
        "aW/(m² · K)",
        "attowatts per square meter kelvin",
        1.0000000000000001e-18
    )]
    AttowattPerSquareMeterKelvin,
    #[linear(
        "zW/(m² · K)",
        "zeptowatts per square meter kelvin",
        9.9999999999999991e-22
    )]
    ZeptowattPerSquareMeterKelvin,
    #[linear(
        "yW/(m² · K)",
        "yoctowatts per square meter kelvin",
        9.9999999999999992e-25
    )]
    YoctowattPerSquareMeterKelvin,
    #[linear("W/(m² · °C)", "watts per square meter degree celsius", 1.0)]
    WattPerSquareMeterDegreeCelsius,
    #[linear("J/(s · m² · K)", "joules per second square meter kelvin", 1.0)]
    JoulePerSecondSquareMeterKelvin,
    #[linear(
        "J/(s · m² · °C)",
        "joules per second square meter degree celsius",
        1.0
    )]
    JoulePerSecondSquareMeterDegreeCelsius,
    #[linear(
        "Btu (IT)/(hr · ft² · °F)",
        "British thermal units (IT) per hour square foot degree Fahrenheit",
        5.6782641343060458
    )]
    BtuItPerHourSquareFootDegreeFahrenheit,
    #[linear(
        "Btu (IT)/(hr · in² · °F)",
        "British thermal units (IT) per hour square inch degree Fahrenheit",
        817.67003534007074
    )]
    BtuItPerHourSquareInchDegreeFahrenheit,
    #[linear(
        "Btu (IT)/(min · ft² · °F)",
        "British thermal units (IT) per minute square foot degree Fahrenheit",
        340.69584805836274
    )]
    BtuItPerMinuteSquareFootDegreeFahrenheit,
    #[linear(
        "Btu (IT)/(min · in² · °F)",
        "British thermal units (IT) per minute square inch degree Fahrenheit",
        49060.202120404247
    )]
    BtuItPerMinuteSquareInchDegreeFahrenheit,
    #[linear(
        "Btu (IT)/(s · ft² · °F)",
        "British thermal units (IT) per second square foot degree Fahrenheit",
        20441.750883501765
    )]
    BtuItPerSecondSquareFootDegreeFahrenheit,
    #[linear(
        "Btu (IT)/(s · in² · °F)",
        "British thermal units (IT) per second square inch degree Fahrenheit",
        2943612.1272242544
    )]
    BtuItPerSecondSquareInchDegreeFahrenheit,
    #[linear(
        "Btu/(hr · ft² · °F)",
        "British thermal units per hour square foot degree Fahrenheit",
        5.6744644739289463
    )]
    BtuPerHourSquareFootDegreeFahrenheit,
    #[linear(
        "Btu/(hr · in² · °F)",
        "British thermal units per hour square inch degree Fahrenheit",
        817.12288424576843
    )]
    BtuPerHourSquareInchDegreeFahrenheit,
    #[linear(
        "Btu/(min · ft² · °F)",
        "British thermal units per minute square foot degree Fahrenheit",
        340.46786843573682
    )]
    BtuPerMinuteSquareFootDegreeFahrenheit,
    #[linear(
        "Btu/(min · in² · °F)",
        "British thermal units per minute square inch degree Fahrenheit",
        49027.373054746109
    )]
    BtuPerMinuteSquareInchDegreeFahrenheit,
    #[linear(
        "Btu/(s · ft² · °F)",
        "British thermal units per second square foot degree Fahrenheit",
        20428.072106144209
    )]
    BtuPerSecondSquareFootDegreeFahrenheit,
    #[linear(
        "Btu/(s · in² · °F)",
        "British thermal units per second square inch degree Fahrenheit",
        2941642.3832847662
    )]
    BtuPerSecondSquareInchDegreeFahrenheit,
}

pub type ThermalConductance = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -3,
            K: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(ThermalConductance)]
pub enum ThermalConductanceUnit {
    #[linear(
        "Yg · m²/(s³ · K)",
        "yottagrams meter squared per second cubed kelvin",
        1e+21
    )]
    YottagramMeterSquaredPerSecondCubedKelvin,
    #[linear(
        "Zg · m²/(s³ · K)",
        "zettagrams meter squared per second cubed kelvin",
        1e+18
    )]
    ZettagramMeterSquaredPerSecondCubedKelvin,
    #[linear(
        "Eg · m²/(s³ · K)",
        "exagrams meter squared per second cubed kelvin",
        1000000000000000.0
    )]
    ExagramMeterSquaredPerSecondCubedKelvin,
    #[linear(
        "Pg · m²/(s³ · K)",
        "petagrams meter squared per second cubed kelvin",
        1000000000000.0
    )]
    PetagramMeterSquaredPerSecondCubedKelvin,
    #[linear(
        "Tg · m²/(s³ · K)",
        "teragrams meter squared per second cubed kelvin",
        1000000000.0
    )]
    TeragramMeterSquaredPerSecondCubedKelvin,
    #[linear(
        "Gg · m²/(s³ · K)",
        "gigagrams meter squared per second cubed kelvin",
        1000000.0
    )]
    GigagramMeterSquaredPerSecondCubedKelvin,
    #[linear(
        "Mg · m²/(s³ · K)",
        "megagrams meter squared per second cubed kelvin",
        1000.0
    )]
    MegagramMeterSquaredPerSecondCubedKelvin,
    #[linear(
        "kg · m²/(s³ · K)",
        "kilograms meter squared per second cubed kelvin",
        1.0
    )]
    KilogramMeterSquaredPerSecondCubedKelvin,
    #[linear(
        "hg · m²/(s³ · K)",
        "hectograms meter squared per second cubed kelvin",
        0.10000000000000001
    )]
    HectogramMeterSquaredPerSecondCubedKelvin,
    #[linear(
        "dag · m²/(s³ · K)",
        "decagrams meter squared per second cubed kelvin",
        0.01
    )]
    DecagramMeterSquaredPerSecondCubedKelvin,
    #[linear(
        "g · m/(s³ · K)",
        "grams meter squared per second cubed kelvin",
        0.001
    )]
    GramMeterSquaredPerSecondCubedKelvin,
    #[linear(
        "dg · m²/(s³ · K)",
        "decigrams meter squared per second cubed kelvin",
        0.0001
    )]
    DecigramMeterSquaredPerSecondCubedKelvin,
    #[linear(
        "cg · m²/(s³ · K)",
        "centigrams meter squared per second cubed kelvin",
        1.0000000000000001e-05
    )]
    CentigramMeterSquaredPerSecondCubedKelvin,
    #[linear(
        "mg · m²/(s³ · K)",
        "milligrams meter squared per second cubed kelvin",
        9.9999999999999995e-07
    )]
    MilligramMeterSquaredPerSecondCubedKelvin,
    #[linear(
        "µg · m/(s³ · K)",
        "micrograms meter squared per second cubed kelvin",
        9.9999999999999986e-10
    )]
    MicrogramMeterSquaredPerSecondCubedKelvin,
    #[linear(
        "ng · m²/(s³ · K)",
        "nanograms meter squared per second cubed kelvin",
        9.9999999999999998e-13
    )]
    NanogramMeterSquaredPerSecondCubedKelvin,
    #[linear(
        "pg · m²/(s³ · K)",
        "picograms meter squared per second cubed kelvin",
        1.0000000000000001e-15
    )]
    PicogramMeterSquaredPerSecondCubedKelvin,
    #[linear(
        "fg · m²/(s³ · K)",
        "femtograms meter squared per second cubed kelvin",
        1.0000000000000001e-18
    )]
    FemtogramMeterSquaredPerSecondCubedKelvin,
    #[linear(
        "ag · m²/(s³ · K)",
        "attograms meter squared per second cubed kelvin",
        1.0000000000000001e-21
    )]
    AttogramMeterSquaredPerSecondCubedKelvin,
    #[linear(
        "zg · m²/(s³ · K)",
        "zeptograms meter squared per second cubed kelvin",
        9.9999999999999992e-25
    )]
    ZeptogramMeterSquaredPerSecondCubedKelvin,
    #[linear(
        "yg · m²/(s³ · K)",
        "yoctograms meter squared per second cubed kelvin",
        9.9999999999999986e-28
    )]
    YoctogramMeterSquaredPerSecondCubedKelvin,
    #[linear("YW/K", "yottawatts per kelvin", 9.9999999999999998e+23)]
    YottawattPerKelvin,
    #[linear("ZW/K", "zettawatts per kelvin", 1e+21)]
    ZettawattPerKelvin,
    #[linear("EW/K", "exawatts per kelvin", 1e+18)]
    ExawattPerKelvin,
    #[linear("PW/K", "petawatts per kelvin", 1000000000000000.0)]
    PetawattPerKelvin,
    #[linear("TW/K", "terawatts per kelvin", 1000000000000.0)]
    TerawattPerKelvin,
    #[linear("GW/K", "gigawatts per kelvin", 1000000000.0)]
    GigawattPerKelvin,
    #[linear("MW/K", "megawatts per kelvin", 1000000.0)]
    MegawattPerKelvin,
    #[linear("kW/K", "kilowatts per kelvin", 1000.0)]
    KilowattPerKelvin,
    #[linear("hW/K", "hectowatts per kelvin", 100.0)]
    HectowattPerKelvin,
    #[linear("daW/K", "decawatts per kelvin", 10.0)]
    DecawattPerKelvin,
    #[linear("W/K", "watts per kelvin", 1.0)]
    WattPerKelvin,
    #[linear("dW/K", "deciwatts per kelvin", 0.10000000000000001)]
    DeciwattPerKelvin,
    #[linear("cW/K", "centiwatts per kelvin", 0.01)]
    CentiwattPerKelvin,
    #[linear("mW/K", "milliwatts per kelvin", 0.001)]
    MilliwattPerKelvin,
    #[linear("µW/K", "microwatts per kelvin", 9.9999999999999995e-07)]
    MicrowattPerKelvin,
    #[linear("nW/K", "nanowatts per kelvin", 1.0000000000000001e-09)]
    NanowattPerKelvin,
    #[linear("pW/K", "picowatts per kelvin", 9.9999999999999998e-13)]
    PicowattPerKelvin,
    #[linear("fW/K", "femtowatts per kelvin", 1.0000000000000001e-15)]
    FemtowattPerKelvin,
    #[linear("aW/K", "attowatts per kelvin", 1.0000000000000001e-18)]
    AttowattPerKelvin,
    #[linear("zW/K", "zeptowatts per kelvin", 9.9999999999999991e-22)]
    ZeptowattPerKelvin,
    #[linear("yW/K", "yoctowatts per kelvin", 9.9999999999999992e-25)]
    YoctowattPerKelvin,
    #[linear(
        "kg · m²/(s³ · °C)",
        "kilograms meter squared per second cubed degree Celsius",
        1.0
    )]
    KilogramMeterSquaredPerSecondCubedDegreeCelsius,
    #[linear("kW/°C", "kilowatts per degree Celsius", 1000.0)]
    KilowattPerDegreeCelsius,
    #[linear("W/°C", "watts per degree Celsius", 1.0)]
    WattPerMeterDegreeCelsius,
    #[linear("mW/°C", "milliwatts per degree Celsius", 0.001)]
    MilliwattPerDegreeCelsius,
}

pub type ThermalConductivity = StaticSIQuantity<
    {
        ISQ {
            m: 1,
            kg: 1,
            s: -3,
            K: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(ThermalConductivity)]
pub enum ThermalConductivityUnit {
    #[linear("Yg · m/(s³ · K)", "yottagrams meter per second cubed kelvin", 1e+21)]
    YottagramMeterPerSecondCubedKelvin,
    #[linear("Zg · m/(s³ · K)", "zettagrams meter per second cubed kelvin", 1e+18)]
    ZettagramMeterPerSecondCubedKelvin,
    #[linear(
        "Eg · m/(s³ · K)",
        "exagrams meter per second cubed kelvin",
        1000000000000000.0
    )]
    ExagramMeterPerSecondCubedKelvin,
    #[linear(
        "Pg · m/(s³ · K)",
        "petagrams meter per second cubed kelvin",
        1000000000000.0
    )]
    PetagramMeterPerSecondCubedKelvin,
    #[linear(
        "Tg · m/(s³ · K)",
        "teragrams meter per second cubed kelvin",
        1000000000.0
    )]
    TeragramMeterPerSecondCubedKelvin,
    #[linear(
        "Gg · m/(s³ · K)",
        "gigagrams meter per second cubed kelvin",
        1000000.0
    )]
    GigagramMeterPerSecondCubedKelvin,
    #[linear("Mg · m/(s³ · K)", "megagrams meter per second cubed kelvin", 1000.0)]
    MegagramMeterPerSecondCubedKelvin,
    #[linear("kg · m/(s³ · K)", "kilograms meter per second cubed kelvin", 1.0)]
    KilogramMeterPerSecondCubedKelvin,
    #[linear(
        "hg · m/(s³ · K)",
        "hectograms meter per second cubed kelvin",
        0.10000000000000001
    )]
    HectogramMeterPerSecondCubedKelvin,
    #[linear("dag · m/(s³ · K)", "decagrams meter per second cubed kelvin", 0.01)]
    DecagramMeterPerSecondCubedKelvin,
    #[linear("g · m/(s³ · K)", "grams meter per second cubed kelvin", 0.001)]
    GramMeterPerSecondCubedKelvin,
    #[linear("dg · m/(s³ · K)", "decigrams meter per second cubed kelvin", 0.0001)]
    DecigramMeterPerSecondCubedKelvin,
    #[linear(
        "cg · m/(s³ · K)",
        "centigrams meter per second cubed kelvin",
        1.0000000000000001e-05
    )]
    CentigramMeterPerSecondCubedKelvin,
    #[linear(
        "mg · m/(s³ · K)",
        "milligrams meter per second cubed kelvin",
        9.9999999999999995e-07
    )]
    MilligramMeterPerSecondCubedKelvin,
    #[linear(
        "µg · m/(s³ · K)",
        "micrograms meter per second cubed kelvin",
        9.9999999999999986e-10
    )]
    MicrogramMeterPerSecondCubedKelvin,
    #[linear(
        "ng · m/(s³ · K)",
        "nanograms meter per second cubed kelvin",
        9.9999999999999998e-13
    )]
    NanogramMeterPerSecondCubedKelvin,
    #[linear(
        "pg · m/(s³ · K)",
        "picograms meter per second cubed kelvin",
        1.0000000000000001e-15
    )]
    PicogramMeterPerSecondCubedKelvin,
    #[linear(
        "fg · m/(s³ · K)",
        "femtograms meter per second cubed kelvin",
        1.0000000000000001e-18
    )]
    FemtogramMeterPerSecondCubedKelvin,
    #[linear(
        "ag · m/(s³ · K)",
        "attograms meter per second cubed kelvin",
        1.0000000000000001e-21
    )]
    AttogramMeterPerSecondCubedKelvin,
    #[linear(
        "zg · m/(s³ · K)",
        "zeptograms meter per second cubed kelvin",
        9.9999999999999992e-25
    )]
    ZeptogramMeterPerSecondCubedKelvin,
    #[linear(
        "yg · m/(s³ · K)",
        "yoctograms meter per second cubed kelvin",
        9.9999999999999986e-28
    )]
    YoctogramMeterPerSecondCubedKelvin,
    #[linear("YW/(m · K)", "yottawatts per meter kelvin", 9.9999999999999998e+23)]
    YottawattPerMeterKelvin,
    #[linear("ZW/(m · K)", "zettawatts per meter kelvin", 1e+21)]
    ZettawattPerMeterKelvin,
    #[linear("EW/(m · K)", "exawatts per meter kelvin", 1e+18)]
    ExawattPerMeterKelvin,
    #[linear("PW/(m · K)", "petawatts per meter kelvin", 1000000000000000.0)]
    PetawattPerMeterKelvin,
    #[linear("TW/(m · K)", "terawatts per meter kelvin", 1000000000000.0)]
    TerawattPerMeterKelvin,
    #[linear("GW/(m · K)", "gigawatts per meter kelvin", 1000000000.0)]
    GigawattPerMeterKelvin,
    #[linear("MW/(m · K)", "megawatts per meter kelvin", 1000000.0)]
    MegawattPerMeterKelvin,
    #[linear("kW/(m · K)", "kilowatts per meter kelvin", 1000.0)]
    KilowattPerMeterKelvin,
    #[linear("hW/(m · K)", "hectowatts per meter kelvin", 100.0)]
    HectowattPerMeterKelvin,
    #[linear("daW/(m · K)", "decawatts per meter kelvin", 10.0)]
    DecawattPerMeterKelvin,
    #[linear("W/(m · K)", "watts per meter kelvin", 1.0)]
    WattPerMeterKelvin,
    #[linear("dW/(m · K)", "deciwatts per meter kelvin", 0.10000000000000001)]
    DeciwattPerMeterKelvin,
    #[linear("cW/(m · K)", "centiwatts per meter kelvin", 0.01)]
    CentiwattPerMeterKelvin,
    #[linear("mW/(m · K)", "milliwatts per meter kelvin", 0.001)]
    MilliwattPerMeterKelvin,
    #[linear("µW/(m · K)", "microwatts per meter kelvin", 9.9999999999999995e-07)]
    MicrowattPerMeterKelvin,
    #[linear("nW/(m · K)", "nanowatts per meter kelvin", 1.0000000000000001e-09)]
    NanowattPerMeterKelvin,
    #[linear("pW/(m · K)", "picowatts per meter kelvin", 9.9999999999999998e-13)]
    PicowattPerMeterKelvin,
    #[linear("fW/(m · K)", "femtowatts per meter kelvin", 1.0000000000000001e-15)]
    FemtowattPerMeterKelvin,
    #[linear("aW/(m · K)", "attowatts per meter kelvin", 1.0000000000000001e-18)]
    AttowattPerMeterKelvin,
    #[linear("zW/(m · K)", "zeptowatts per meter kelvin", 9.9999999999999991e-22)]
    ZeptowattPerMeterKelvin,
    #[linear("yW/(m · K)", "yoctowatts per meter kelvin", 9.9999999999999992e-25)]
    YoctowattPerMeterKelvin,
    #[linear(
        "kg · m/(s³ · °C)",
        "kilograms meter per second cubed degree celsius",
        1.0
    )]
    KilogramMeterPerSecondCubedDegreeCelsius,
    #[linear("kW/(m · °C)", "kilowatts per meter degree celsius", 1000.0)]
    KilowattPerMeterDegreeCelsius,
    #[linear("W/(m · °C)", "watts per meter degree celsius", 1.0)]
    WattPerMeterDegreeCelsius,
    #[linear("mW/(m · °C)", "milliwatts per meter degree celsius", 0.001)]
    MilliwattPerMeterDegreeCelsius,
}

pub type ThermalResistance = StaticSIQuantity<
    {
        ISQ {
            m: -2,
            kg: -1,
            s: 3,
            K: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(ThermalResistance)]
pub enum ThermalResistanceUnit {
    #[linear("K/YW", "kelvins per yottawatt", 1.0000000000000001e-24)]
    KelvinPerYottawatt,
    #[linear("K/ZW", "kelvins per zettawatt", 9.9999999999999991e-22)]
    KelvinPerZettawatt,
    #[linear("K/EW", "kelvins per exawatt", 1.0000000000000001e-18)]
    KelvinPerExawatt,
    #[linear("K/PW", "kelvins per petawatt", 1.0000000000000001e-15)]
    KelvinPerPetawatt,
    #[linear("K/TW", "kelvins per terawatt", 9.9999999999999998e-13)]
    KelvinPerTerawatt,
    #[linear("K/GW", "kelvins per gigawatt", 1.0000000000000001e-09)]
    KelvinPerGigawatt,
    #[linear("K/MW", "kelvins per megawatt", 9.9999999999999995e-07)]
    KelvinPerMegawatt,
    #[linear("K/kw", "kelvins per kilowatt", 0.001)]
    KelvinPerKilowatt,
    #[linear("K/hW", "kelvins per hectowatt", 0.01)]
    KelvinPerHectowatt,
    #[linear("K/daW", "kelvins per decawatt", 0.10000000000000001)]
    KelvinPerDecawatt,
    #[linear("K/W", "kelvins per watt", 1.0)]
    KelvinPerWatt,
    #[linear("K/dW", "kelvins per deciwatt", 10.0)]
    KelvinPerDeciwatt,
    #[linear("K/cW", "kelvins per centiwatt", 100.0)]
    KelvinPerCentiwatt,
    #[linear("K/mW", "kelvins per milliwatt", 1000.0)]
    KelvinPerMilliwatt,
    #[linear("K/µW", "kelvins per microwatt", 1000000.0)]
    KelvinPerMicrowatt,
    #[linear("K/nW", "kelvins per nanowatt", 999999999.99999988)]
    KelvinPerNanowatt,
    #[linear("K/pW", "kelvins per picowatt", 1000000000000.0)]
    KelvinPerPicowatt,
    #[linear("K/fW", "kelvins per femtowatt", 999999999999999.88)]
    KelvinPerFemtowatt,
    #[linear("K/aW", "kelvins per attowatt", 9.9999999999999987e+17)]
    KelvinPerAttowatt,
    #[linear("K/zW", "kelvins per zeptowatt", 1.0000000000000001e+21)]
    KelvinPerZeptowatt,
    #[linear("K/yW", "kelvins per yoctowatt", 1.0000000000000001e+24)]
    KelvinPerYoctowatt,
}

pub type SpecificHeatCapacity = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            s: -2,
            K: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(SpecificHeatCapacity)]
pub enum SpecificHeatCapacityUnit {
    #[linear(
        "km²/(s² · K)",
        "square kilometers per second squared kelvin",
        1000000.0
    )]
    SquareKilometerPerSecondSquaredKelvin,
    #[linear("m²/(s² · K)", "square meters per second squared kelvin", 1.0)]
    SquareMeterPerSecondSquaredKelvin,
    #[linear(
        "cm²/(s² · K)",
        "square centimeters per second squared kelvin",
        0.0001
    )]
    SquareCentimeterPerSecondSquaredKelvin,
    #[linear(
        "mm²/(s² · K)",
        "square millimeters per second squared kelvin",
        9.9999999999999995e-07
    )]
    SquareMillimeterPerSecondSquaredKelvin,
    #[linear(
        "µm²/(s² · K)",
        "square micrometers per second squared kelvin",
        9.9999999999999998e-13
    )]
    SquareMicrometerPerSecondSquaredKelvin,
    #[linear(
        "YJ/(kg · K)",
        "yottajoules per kilogram kelvin",
        9.9999999999999998e+23
    )]
    YottajoulePerKilogramKelvin,
    #[linear("ZJ/(kg · K)", "zettajoules per kilogram kelvin", 1e+21)]
    ZettajoulePerKilogramKelvin,
    #[linear("EJ/(kg · K)", "exajoules per kilogram kelvin", 1e+18)]
    ExajoulePerKilogramKelvin,
    #[linear("PJ/(kg · K)", "petajoules per kilogram kelvin", 1000000000000000.0)]
    PetajoulePerKilogramKelvin,
    #[linear("TJ/(kg · K)", "terajoules per kilogram kelvin", 1000000000000.0)]
    TerajoulePerKilogramKelvin,
    #[linear("GJ/(kg · K)", "gigajoules per kilogram kelvin", 1000000000.0)]
    GigajoulePerKilogramKelvin,
    #[linear("MJ/(kg · K)", "megajoules per kilogram kelvin", 1000000.0)]
    MegajoulePerKilogramKelvin,
    #[linear("kJ/(kg · K)", "kilojoules per kilogram kelvin", 1000.0)]
    KilojoulePerKilogramKelvin,
    #[linear("hJ/(kg · K)", "hectojoules per kilogram kelvin", 100.0)]
    HectojoulePerKilogramKelvin,
    #[linear("daJ/(kg · K)", "decajoules per kilogram kelvin", 10.0)]
    DecajoulePerKilogramKelvin,
    #[linear("J/(kg · K)", "joules per kilogram kelvin", 1.0)]
    JoulePerKilogramKelvin,
    #[linear("dJ/(kg · K)", "decijoules per kilogram kelvin", 0.10000000000000001)]
    DecijoulePerKilogramKelvin,
    #[linear("cJ/(kg · K)", "centijoules per kilogram kelvin", 0.01)]
    CentijoulePerKilogramKelvin,
    #[linear("mJ/(kg · K)", "millijoules per kilogram kelvin", 0.001)]
    MillijoulePerKilogramKelvin,
    #[linear(
        "µJ/(kg · K)",
        "microjoules per kilogram kelvin",
        9.9999999999999995e-07
    )]
    MicrojoulePerKilogramKelvin,
    #[linear(
        "nJ/(kg · K)",
        "nanojoules per kilogram kelvin",
        1.0000000000000001e-09
    )]
    NanojoulePerKilogramKelvin,
    #[linear(
        "pJ/(kg · K)",
        "picojoules per kilogram kelvin",
        9.9999999999999998e-13
    )]
    PicojoulePerKilogramKelvin,
    #[linear(
        "fJ/(kg · K)",
        "femtojoules per kilogram kelvin",
        1.0000000000000001e-15
    )]
    FemtojoulePerKilogramKelvin,
    #[linear(
        "aJ/(kg · K)",
        "attojoules per kilogram kelvin",
        1.0000000000000001e-18
    )]
    AttojoulePerKilogramKelvin,
    #[linear(
        "zJ/(kg · K)",
        "zeptojoules per kilogram kelvin",
        9.9999999999999991e-22
    )]
    ZeptojoulePerKilogramKelvin,
    #[linear(
        "yJ/(kg · K)",
        "yoctojoules per kilogram kelvin",
        9.9999999999999992e-25
    )]
    YoctojoulePerKilogramKelvin,
    #[linear("kJ/(kg · °C)", "kilojoules per kilogram degree celsius", 1000.0)]
    KilojoulePerKilogramDegreeCelsius,
    #[linear("kJ/(g · °C)", "kilojoules per gram degree celsius", 1000000.0)]
    KilojoulePerGramDegreeCelsius,
    #[linear("J/(kg · °C)", "joules per kilogram degree celsius", 1.0)]
    JoulePerKilogramDegreeCelsius,
    #[linear("J/(g · °C)", "joules per gram degree celsius", 1000.0)]
    JoulePerGramDegreeCelsius,
    #[linear("mJ/(kg · °C)", "millijoules per kilogram degree celsius", 0.001)]
    MillijoulePerKilogramDegreeCelsius,
    #[linear("mJ/(g · °C)", "millijoules per gram degree celsius", 1.0)]
    MillijoulePerGramDegreeCelsius,
    #[linear(
        "Btu/(oz · °F)",
        "British thermal units per ounce degree Fahrenheit",
        66943.990586083979
    )]
    BtuPerOunceDegreeFahrenheit,
    #[linear(
        "Btu (IT)/(oz · °F)",
        "British thermal units (IT) per ounce degree Fahrenheit",
        66988.81674187076
    )]
    BtuItPerOunceDegreeFahrenheit,
    #[linear(
        "Btu/(lb · °F)",
        "British thermal units per pound degree Fahrenheit",
        4183.9986736991177
    )]
    BtuPerPoundDegreeFahrenheit,
    #[linear(
        "Btu (IT)/(lb · °F)",
        "British thermal units (IT) per pound degree Fahrenheit",
        4186.8003079416667
    )]
    BtuItPerPoundDegreeFahrenheit,
    #[linear(
        "Btu/(t · °F)",
        "British thermal units per ton degree Fahrenheit",
        1.8978299999999999
    )]
    BtuPerTonDegreeFahrenheit,
    #[linear(
        "Btu (IT)/(t · °F)",
        "British thermal units (IT) per ton degree Fahrenheit",
        1.8991008
    )]
    BtuItPerTonDegreeFahrenheit,
    #[linear("cal/(kg · K)", "calories per kilogram kelvin", 4.1840000000000002)]
    CaloriePerKilogramKelvin,
    #[linear("cal/(g · K)", "calories per gram kelvin", 4184.0)]
    CaloriePerGramKelvin,
}

pub type ArealHeatCapacity = StaticSIQuantity<
    {
        ISQ {
            kg: 1,
            s: -2,
            K: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(ArealHeatCapacity)]
pub enum ArealHeatCapacityUnit {
    #[linear("J/(m² · K)", "joules per square meter kelvin", 1.0)]
    JoulePerSquareMeterKelvin,
}

pub type VolumetricHeatCapacity = StaticSIQuantity<
    {
        ISQ {
            m: -1,
            kg: 1,
            s: -2,
            K: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(VolumetricHeatCapacity)]
pub enum VolumetricHeatCapacityUnit {
    #[linear("J/(m³ · K)", "joules per cubic meter kelvin", 1.0)]
    JoulePerCubicMeterKelvin,
    #[linear(
        "cal/(m³ · K)",
        "calories per cubic meter kelvin",
        4.1840000000000002
    )]
    CaloriePerCubicMeterKelvin,
}
