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

#[derive(Copy, Clone, Unit)]
#[dim(ThermodynamicTemperature)]
#[dim_delta(TemperatureInterval)]
pub enum ThermodynamicTemperatureUnit {
    #[unit(1.0, "K", "kelvin")]
    Kelvin,
    #[unit(1.0, "°C", "degrees Celsius", 273.15)]
    Celsius,
    #[unit(5.0 / 9.0, "°F", "degrees Fahrenheit", 459.67)]
    Fahrenheit,
    #[unit(5.0 / 9.0, "°R", "degrees Rankine")]
    Rankine,
}

pub type TemperatureInterval = StaticSIQuantity<{ ISQ { K: 1, ..ISQ::ZERO } }>;

#[derive(Copy, Clone, Unit)]
#[dim(TemperatureInterval)]
pub enum TemperatureIntervalUnit {
    #[unit(1.0, "K", "kelvin")]
    Kelvin,
    #[unit(1.0, "°C", "degrees Celsius")]
    Celsius,
    #[unit(5.0 / 9.0, "°F", "degrees Fahrenheit")]
    Fahrenheit,
    #[unit(5.0 / 9.0, "°R", "degrees Rankine")]
    Rankine,
}

pub type TemperatureCoefficient = StaticSIQuantity<{ ISQ { K: -1, ..ISQ::ZERO } }>;

#[derive(Copy, Clone, Unit)]
#[dim(TemperatureCoefficient)]
pub enum TemperatureCoefficientUnit {
    #[unit(1.0, "K⁻¹", "per kelvin")]
    PerKelvin,
    #[unit(9.9999999999999995e-07, "ppm/K", "ppm per kelvin")]
    PpmPerKelvin,
    #[unit(9.9999999999999995e-07, "ppm/°C", "ppm per degree Celsius")]
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

#[derive(Copy, Clone, Unit)]
#[dim(TemperatureGradient)]
pub enum TemperatureGradientUnit {
    #[unit(0.001, "K/km", "kelvins per kilometer")]
    KelvinPerKilometer,
    #[unit(0.01, "K/hm", "kelvins per hectometer")]
    KelvinPerHectometer,
    #[unit(1.0, "K/m", "kelvins per meter")]
    KelvinPerMeter,
    #[unit(100.0, "K/cm", "kelvins per centimeter")]
    KelvinPerCentimeter,
    #[unit(1000.0, "K/mm", "kelvins per millimeter")]
    KelvinPerMillimeter,
    #[unit(1000000.0, "K/µm", "kelvins per micrometer")]
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

#[derive(Copy, Clone, Unit)]
#[dim(HeatCapacity)]
pub enum HeatCapacityUnit {
    #[unit(
        1e+21,
        "Yg · m²/(s² · K)",
        "yottagram square meters per second squared kelvin"
    )]
    YottagramSquareMeterPerSecondSquaredKelvin,
    #[unit(
        1e+18,
        "Zg · m²/(s² · K)",
        "zettagram square meters per second squared kelvin"
    )]
    ZettagramSquareMeterPerSecondSquaredKelvin,
    #[unit(
        1000000000000000.0,
        "Eg · m²/(s² · K)",
        "exagram square meters per second squared kelvin"
    )]
    ExagramSquareMeterPerSecondSquaredKelvin,
    #[unit(
        1000000000000.0,
        "Pg · m²/(s² · K)",
        "petagram square meters per second squared kelvin"
    )]
    PetagramSquareMeterPerSecondSquaredKelvin,
    #[unit(
        1000000000.0,
        "Tg · m²/(s² · K)",
        "teragram square meters per second squared kelvin"
    )]
    TeragramSquareMeterPerSecondSquaredKelvin,
    #[unit(
        1000000.0,
        "Gg · m²/(s² · K)",
        "gigagram square meters per second squared kelvin"
    )]
    GigagramSquareMeterPerSecondSquaredKelvin,
    #[unit(
        1000.0,
        "Mg · m²/(s² · K)",
        "megagram square meters per second squared kelvin"
    )]
    MegagramSquareMeterPerSecondSquaredKelvin,
    #[unit(
        1.0,
        "kg · m²/(s² · K)",
        "kilogram square meters per second squared kelvin"
    )]
    KilogramSquareMeterPerSecondSquaredKelvin,
    #[unit(
        0.10000000000000001,
        "hg · m²/(s² · K)",
        "hectogram square meters per second squared kelvin"
    )]
    HectogramSquareMeterPerSecondSquaredKelvin,
    #[unit(
        0.01,
        "dag · m²/(s² · K)",
        "decagram square meters per second squared kelvin"
    )]
    DecagramSquareMeterPerSecondSquaredKelvin,
    #[unit(
        0.001,
        "g · m²/(s² · K)",
        "gram square meters per second squared kelvin"
    )]
    GramSquareMeterPerSecondSquaredKelvin,
    #[unit(
        0.0001,
        "dg · m²/(s² · K)",
        "decigram square meters per second squared kelvin"
    )]
    DecigramSquareMeterPerSecondSquaredKelvin,
    #[unit(
        1.0000000000000001e-05,
        "cg · m²/(s² · K)",
        "centigram square meters per second squared kelvin"
    )]
    CentigramSquareMeterPerSecondSquaredKelvin,
    #[unit(
        9.9999999999999995e-07,
        "mg · m²/(s² · K)",
        "milligram square meters per second squared kelvin"
    )]
    MilligramSquareMeterPerSecondSquaredKelvin,
    #[unit(
        9.9999999999999986e-10,
        "µg · m²/(s² · K)",
        "microgram square meters per second squared kelvin"
    )]
    MicrogramSquareMeterPerSecondSquaredKelvin,
    #[unit(
        9.9999999999999998e-13,
        "ng · m²/(s² · K)",
        "nanogram square meters per second squared kelvin"
    )]
    NanogramSquareMeterPerSecondSquaredKelvin,
    #[unit(
        1.0000000000000001e-15,
        "pg · m²/(s² · K)",
        "picogram square meters per second squared kelvin"
    )]
    PicogramSquareMeterPerSecondSquaredKelvin,
    #[unit(
        1.0000000000000001e-18,
        "fg · m²/(s² · K)",
        "femtogram square meters per second squared kelvin"
    )]
    FemtogramSquareMeterPerSecondSquaredKelvin,
    #[unit(
        1.0000000000000001e-21,
        "ag · m²/(s² · K)",
        "attogram square meters per second squared kelvin"
    )]
    AttogramSquareMeterPerSecondSquaredKelvin,
    #[unit(
        9.9999999999999992e-25,
        "zg · m²/(s² · K)",
        "zeptogram square meters per second squared kelvin"
    )]
    ZeptogramSquareMeterPerSecondSquaredKelvin,
    #[unit(
        9.9999999999999986e-28,
        "yg · m²/(s² · K)",
        "yoctogram square meters per second squared kelvin"
    )]
    YoctogramSquareMeterPerSecondSquaredKelvin,
    #[unit(9.9999999999999998e+23, "YJ/K", "yottajoules per kelvin")]
    YottajoulePerKelvin,
    #[unit(1e+21, "ZJ/K", "zettajoules per kelvin")]
    ZettajoulePerKelvin,
    #[unit(1e+18, "EJ/K", "exajoules per kelvin")]
    ExajoulePerKelvin,
    #[unit(1000000000000000.0, "PJ/K", "petajoules per kelvin")]
    PetajoulePerKelvin,
    #[unit(1000000000000.0, "TJ/K", "terajoules per kelvin")]
    TerajoulePerKelvin,
    #[unit(1000000000.0, "GJ/K", "gigajoules per kelvin")]
    GigajoulePerKelvin,
    #[unit(1000000.0, "MJ/K", "megajoules per kelvin")]
    MegajoulePerKelvin,
    #[unit(1000.0, "kJ/K", "kilojoules per kelvin")]
    KilojoulePerKelvin,
    #[unit(100.0, "hJ/K", "hectojoules per kelvin")]
    HectojoulePerKelvin,
    #[unit(10.0, "daJ/K", "decajoules per kelvin")]
    DecajoulePerKelvin,
    #[unit(1.0, "J/K", "joules per kelvin")]
    JoulePerKelvin,
    #[unit(0.10000000000000001, "dJ/K", "decijoules per kelvin")]
    DecijoulePerKelvin,
    #[unit(0.01, "cJ/K", "centijoules per kelvin")]
    CentijoulePerKelvin,
    #[unit(0.001, "mJ/K", "millijoules per kelvin")]
    MillijoulePerKelvin,
    #[unit(9.9999999999999995e-07, "µJ/K", "microjoules per kelvin")]
    MicrojoulePerKelvin,
    #[unit(1.0000000000000001e-09, "nJ/K", "nanojoules per kelvin")]
    NanojoulePerKelvin,
    #[unit(9.9999999999999998e-13, "pJ/K", "picojoules per kelvin")]
    PicojoulePerKelvin,
    #[unit(1.0000000000000001e-15, "fJ/K", "femtojoules per kelvin")]
    FemtojoulePerKelvin,
    #[unit(1.0000000000000001e-18, "aJ/K", "attojoules per kelvin")]
    AttojoulePerKelvin,
    #[unit(9.9999999999999991e-22, "zJ/K", "zeptojoules per kelvin")]
    ZeptojoulePerKelvin,
    #[unit(9.9999999999999992e-25, "yJ/K", "yoctojoules per kelvin")]
    YoctojoulePerKelvin,
    #[unit(1000.0, "kJ/°C", "kilojoules per degree celsius")]
    KilojoulePerDegreeCelsius,
    #[unit(1.0, "J/°C", "joules per degree celsius")]
    JoulePerDegreeCelsius,
    #[unit(0.001, "mJ/°C", "millijoules per degree celsius")]
    MillijoulePerDegreeCelsius,
    #[unit(
        1897.8299999999999,
        "Btu/°F",
        "British thermal units per degree Fahrenheit"
    )]
    BtuPerDegreeFahrenheit,
    #[unit(
        1899.1007999999999,
        "Btu (IT)/°F",
        "British thermal units (IT) per degree Fahrenheit"
    )]
    BtuItPerDegreeFahrenheit,
    #[unit(1.3806490000000001e-23, "k", "Boltzmann constants")]
    BoltzmannConstant,
    #[unit(4.1840000000000002, "cal/K", "calories per kelvin")]
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

#[derive(Copy, Clone, Unit)]
#[dim(HeatFluxDensity)]
pub enum HeatFluxDensityUnit {
    #[unit(9.9999999999999998e+23, "YW/m²", "yottawatts per square meter")]
    YottawattPerSquareMeter,
    #[unit(1e+21, "ZW/m²", "zettawatts per square meter")]
    ZettawattPerSquareMeter,
    #[unit(1e+18, "EW/m²", "exawatts per square meter")]
    ExawattPerSquareMeter,
    #[unit(1000000000000000.0, "PW/m²", "petawatts per square meter")]
    PetawattPerSquareMeter,
    #[unit(1000000000000.0, "TW/m²", "terawatts per square meter")]
    TerawattPerSquareMeter,
    #[unit(1000000000.0, "GW/m²", "gigawatts per square meter")]
    GigawattPerSquareMeter,
    #[unit(1000000.0, "MW/m²", "megawatts per square meter")]
    MegawattPerSquareMeter,
    #[unit(1000.0, "kW/m²", "kilowatts per square meter")]
    KilowattPerSquareMeter,
    #[unit(100.0, "hW/m²", "hectowatts per square meter")]
    HectowattPerSquareMeter,
    #[unit(10.0, "daW/m²", "decawatts per square meter")]
    DecawattPerSquareMeter,
    #[unit(1.0, "W/m²", "watts per square meter")]
    WattPerSquareMeter,
    #[unit(0.10000000000000001, "dW/m²", "deciwatts per square meter")]
    DeciwattPerSquareMeter,
    #[unit(0.01, "cW/m²", "centiwatts per square meter")]
    CentiwattPerSquareMeter,
    #[unit(0.001, "mW/m²", "milliwatts per square meter")]
    MilliwattPerSquareMeter,
    #[unit(9.9999999999999995e-07, "µW/m²", "microwatts per square meter")]
    MicrowattPerSquareMeter,
    #[unit(1.0000000000000001e-09, "nW/m²", "nanowatts per square meter")]
    NanowattPerSquareMeter,
    #[unit(9.9999999999999998e-13, "pW/m²", "picowatts per square meter")]
    PicowattPerSquareMeter,
    #[unit(1.0000000000000001e-15, "fW/m²", "femtowatts per square meter")]
    FemtowattPerSquareMeter,
    #[unit(1.0000000000000001e-18, "aW/m²", "attowatts per square meter")]
    AttowattPerSquareMeter,
    #[unit(9.9999999999999991e-22, "zW/m²", "zeptowatts per square meter")]
    ZeptowattPerSquareMeter,
    #[unit(9.9999999999999992e-25, "yW/m²", "yoctowatts per square meter")]
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

#[derive(Copy, Clone, Unit)]
#[dim(HeatTransfer)]
pub enum HeatTransferUnit {
    #[unit(1e+21, "Yg/(s³ · K)", "yottagrams per second cubed kelvin")]
    YottagramPerSecondCubedKelvin,
    #[unit(1e+18, "Zg/(s³ · K)", "zettagrams per second cubed kelvin")]
    ZettagramPerSecondCubedKelvin,
    #[unit(1000000000000000.0, "Eg/(s³ · K)", "exagrams per second cubed kelvin")]
    ExagramPerSecondCubedKelvin,
    #[unit(1000000000000.0, "Pg/(s³ · K)", "petagrams per second cubed kelvin")]
    PetagramPerSecondCubedKelvin,
    #[unit(1000000000.0, "Tg/(s³ · K)", "teragrams per second cubed kelvin")]
    TeragramPerSecondCubedKelvin,
    #[unit(1000000.0, "Gg/(s³ · K)", "gigagrams per second cubed kelvin")]
    GigagramPerSecondCubedKelvin,
    #[unit(1000.0, "Mg/(s³ · K)", "megagrams per second cubed kelvin")]
    MegagramPerSecondCubedKelvin,
    #[unit(1.0, "kg/(s³ · K)", "kilograms per second cubed kelvin")]
    KilogramPerSecondCubedKelvin,
    #[unit(
        0.10000000000000001,
        "hg/(s³ · K)",
        "hectograms per second cubed kelvin"
    )]
    HectogramPerSecondCubedKelvin,
    #[unit(0.01, "dag/(s³ · K)", "decagrams per second cubed kelvin")]
    DecagramPerSecondCubedKelvin,
    #[unit(0.001, "g/(s³ · K)", "grams per second cubed kelvin")]
    GramPerSecondCubedKelvin,
    #[unit(0.0001, "dg/(s³ · K)", "decigrams per second cubed kelvin")]
    DecigramPerSecondCubedKelvin,
    #[unit(
        1.0000000000000001e-05,
        "cg/(s³ · K)",
        "centigrams per second cubed kelvin"
    )]
    CentigramPerSecondCubedKelvin,
    #[unit(
        9.9999999999999995e-07,
        "mg/(s³ · K)",
        "milligrams per second cubed kelvin"
    )]
    MilligramPerSecondCubedKelvin,
    #[unit(
        9.9999999999999986e-10,
        "µg/(s³ · K)",
        "micrograms per second cubed kelvin"
    )]
    MicrogramPerSecondCubedKelvin,
    #[unit(
        9.9999999999999998e-13,
        "ng/(s³ · K)",
        "nanograms per second cubed kelvin"
    )]
    NanogramPerSecondCubedKelvin,
    #[unit(
        1.0000000000000001e-15,
        "pg/(s³ · K)",
        "picograms per second cubed kelvin"
    )]
    PicogramPerSecondCubedKelvin,
    #[unit(
        1.0000000000000001e-18,
        "fg/(s³ · K)",
        "femtograms per second cubed kelvin"
    )]
    FemtogramPerSecondCubedKelvin,
    #[unit(
        1.0000000000000001e-21,
        "ag/(s³ · K)",
        "attograms per second cubed kelvin"
    )]
    AttogramPerSecondCubedKelvin,
    #[unit(
        9.9999999999999992e-25,
        "zg/(s³ · K)",
        "zeptograms per second cubed kelvin"
    )]
    ZeptogramPerSecondCubedKelvin,
    #[unit(
        9.9999999999999986e-28,
        "yg/(s³ · K)",
        "yoctograms per second cubed kelvin"
    )]
    YoctogramPerSecondCubedKelvin,
    #[unit(1.0, "kg/(s³ · °C)", "kilograms per second cubed degree celsius")]
    KilogramPerSecondCubedDegreeCelsius,
    #[unit(
        9.9999999999999998e+23,
        "YW/(m² · K)",
        "yottawatts per square meter kelvin"
    )]
    YottawattPerSquareMeterKelvin,
    #[unit(1e+21, "ZW/(m² · K)", "zettawatts per square meter kelvin")]
    ZettawattPerSquareMeterKelvin,
    #[unit(1e+18, "EW/(m² · K)", "exawatts per square meter kelvin")]
    ExawattPerSquareMeterKelvin,
    #[unit(1000000000000000.0, "PW/(m² · K)", "petawatts per square meter kelvin")]
    PetawattPerSquareMeterKelvin,
    #[unit(1000000000000.0, "TW/(m² · K)", "terawatts per square meter kelvin")]
    TerawattPerSquareMeterKelvin,
    #[unit(1000000000.0, "GW/(m² · K)", "gigawatts per square meter kelvin")]
    GigawattPerSquareMeterKelvin,
    #[unit(1000000.0, "MW/(m² · K)", "megawatts per square meter kelvin")]
    MegawattPerSquareMeterKelvin,
    #[unit(1000.0, "kW/(m² · K)", "kilowatts per square meter kelvin")]
    KilowattPerSquareMeterKelvin,
    #[unit(100.0, "hW/(m² · K)", "hectowatts per square meter kelvin")]
    HectowattPerSquareMeterKelvin,
    #[unit(10.0, "daW/(m² · K)", "decawatts per square meter kelvin")]
    DecawattPerSquareMeterKelvin,
    #[unit(1.0, "W/(m² · K)", "watts per square meter kelvin")]
    WattPerSquareMeterKelvin,
    #[unit(
        0.10000000000000001,
        "dW/(m² · K)",
        "deciwatts per square meter kelvin"
    )]
    DeciwattPerSquareMeterKelvin,
    #[unit(0.01, "cW/(m² · K)", "centiwatts per square meter kelvin")]
    CentiwattPerSquareMeterKelvin,
    #[unit(0.001, "mW/(m² · K)", "milliwatts per square meter kelvin")]
    MilliwattPerSquareMeterKelvin,
    #[unit(
        9.9999999999999995e-07,
        "µW/(m² · K)",
        "microwatts per square meter kelvin"
    )]
    MicrowattPerSquareMeterKelvin,
    #[unit(
        1.0000000000000001e-09,
        "nW/(m² · K)",
        "nanowatts per square meter kelvin"
    )]
    NanowattPerSquareMeterKelvin,
    #[unit(
        9.9999999999999998e-13,
        "pW/(m² · K)",
        "picowatts per square meter kelvin"
    )]
    PicowattPerSquareMeterKelvin,
    #[unit(
        1.0000000000000001e-15,
        "fW/(m² · K)",
        "femtowatts per square meter kelvin"
    )]
    FemtowattPerSquareMeterKelvin,
    #[unit(
        1.0000000000000001e-18,
        "aW/(m² · K)",
        "attowatts per square meter kelvin"
    )]
    AttowattPerSquareMeterKelvin,
    #[unit(
        9.9999999999999991e-22,
        "zW/(m² · K)",
        "zeptowatts per square meter kelvin"
    )]
    ZeptowattPerSquareMeterKelvin,
    #[unit(
        9.9999999999999992e-25,
        "yW/(m² · K)",
        "yoctowatts per square meter kelvin"
    )]
    YoctowattPerSquareMeterKelvin,
    #[unit(1.0, "W/(m² · °C)", "watts per square meter degree celsius")]
    WattPerSquareMeterDegreeCelsius,
    #[unit(1.0, "J/(s · m² · K)", "joules per second square meter kelvin")]
    JoulePerSecondSquareMeterKelvin,
    #[unit(
        1.0,
        "J/(s · m² · °C)",
        "joules per second square meter degree celsius"
    )]
    JoulePerSecondSquareMeterDegreeCelsius,
    #[unit(
        5.6782641343060458,
        "Btu (IT)/(hr · ft² · °F)",
        "British thermal units (IT) per hour square foot degree Fahrenheit"
    )]
    BtuItPerHourSquareFootDegreeFahrenheit,
    #[unit(
        817.67003534007074,
        "Btu (IT)/(hr · in² · °F)",
        "British thermal units (IT) per hour square inch degree Fahrenheit"
    )]
    BtuItPerHourSquareInchDegreeFahrenheit,
    #[unit(
        340.69584805836274,
        "Btu (IT)/(min · ft² · °F)",
        "British thermal units (IT) per minute square foot degree Fahrenheit"
    )]
    BtuItPerMinuteSquareFootDegreeFahrenheit,
    #[unit(
        49060.202120404247,
        "Btu (IT)/(min · in² · °F)",
        "British thermal units (IT) per minute square inch degree Fahrenheit"
    )]
    BtuItPerMinuteSquareInchDegreeFahrenheit,
    #[unit(
        20441.750883501765,
        "Btu (IT)/(s · ft² · °F)",
        "British thermal units (IT) per second square foot degree Fahrenheit"
    )]
    BtuItPerSecondSquareFootDegreeFahrenheit,
    #[unit(
        2943612.1272242544,
        "Btu (IT)/(s · in² · °F)",
        "British thermal units (IT) per second square inch degree Fahrenheit"
    )]
    BtuItPerSecondSquareInchDegreeFahrenheit,
    #[unit(
        5.6744644739289463,
        "Btu/(hr · ft² · °F)",
        "British thermal units per hour square foot degree Fahrenheit"
    )]
    BtuPerHourSquareFootDegreeFahrenheit,
    #[unit(
        817.12288424576843,
        "Btu/(hr · in² · °F)",
        "British thermal units per hour square inch degree Fahrenheit"
    )]
    BtuPerHourSquareInchDegreeFahrenheit,
    #[unit(
        340.46786843573682,
        "Btu/(min · ft² · °F)",
        "British thermal units per minute square foot degree Fahrenheit"
    )]
    BtuPerMinuteSquareFootDegreeFahrenheit,
    #[unit(
        49027.373054746109,
        "Btu/(min · in² · °F)",
        "British thermal units per minute square inch degree Fahrenheit"
    )]
    BtuPerMinuteSquareInchDegreeFahrenheit,
    #[unit(
        20428.072106144209,
        "Btu/(s · ft² · °F)",
        "British thermal units per second square foot degree Fahrenheit"
    )]
    BtuPerSecondSquareFootDegreeFahrenheit,
    #[unit(
        2941642.3832847662,
        "Btu/(s · in² · °F)",
        "British thermal units per second square inch degree Fahrenheit"
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

#[derive(Copy, Clone, Unit)]
#[dim(ThermalConductance)]
pub enum ThermalConductanceUnit {
    #[unit(
        1e+21,
        "Yg · m²/(s³ · K)",
        "yottagrams meter squared per second cubed kelvin"
    )]
    YottagramMeterSquaredPerSecondCubedKelvin,
    #[unit(
        1e+18,
        "Zg · m²/(s³ · K)",
        "zettagrams meter squared per second cubed kelvin"
    )]
    ZettagramMeterSquaredPerSecondCubedKelvin,
    #[unit(
        1000000000000000.0,
        "Eg · m²/(s³ · K)",
        "exagrams meter squared per second cubed kelvin"
    )]
    ExagramMeterSquaredPerSecondCubedKelvin,
    #[unit(
        1000000000000.0,
        "Pg · m²/(s³ · K)",
        "petagrams meter squared per second cubed kelvin"
    )]
    PetagramMeterSquaredPerSecondCubedKelvin,
    #[unit(
        1000000000.0,
        "Tg · m²/(s³ · K)",
        "teragrams meter squared per second cubed kelvin"
    )]
    TeragramMeterSquaredPerSecondCubedKelvin,
    #[unit(
        1000000.0,
        "Gg · m²/(s³ · K)",
        "gigagrams meter squared per second cubed kelvin"
    )]
    GigagramMeterSquaredPerSecondCubedKelvin,
    #[unit(
        1000.0,
        "Mg · m²/(s³ · K)",
        "megagrams meter squared per second cubed kelvin"
    )]
    MegagramMeterSquaredPerSecondCubedKelvin,
    #[unit(
        1.0,
        "kg · m²/(s³ · K)",
        "kilograms meter squared per second cubed kelvin"
    )]
    KilogramMeterSquaredPerSecondCubedKelvin,
    #[unit(
        0.10000000000000001,
        "hg · m²/(s³ · K)",
        "hectograms meter squared per second cubed kelvin"
    )]
    HectogramMeterSquaredPerSecondCubedKelvin,
    #[unit(
        0.01,
        "dag · m²/(s³ · K)",
        "decagrams meter squared per second cubed kelvin"
    )]
    DecagramMeterSquaredPerSecondCubedKelvin,
    #[unit(0.001, "g · m/(s³ · K)", "grams meter squared per second cubed kelvin")]
    GramMeterSquaredPerSecondCubedKelvin,
    #[unit(
        0.0001,
        "dg · m²/(s³ · K)",
        "decigrams meter squared per second cubed kelvin"
    )]
    DecigramMeterSquaredPerSecondCubedKelvin,
    #[unit(
        1.0000000000000001e-05,
        "cg · m²/(s³ · K)",
        "centigrams meter squared per second cubed kelvin"
    )]
    CentigramMeterSquaredPerSecondCubedKelvin,
    #[unit(
        9.9999999999999995e-07,
        "mg · m²/(s³ · K)",
        "milligrams meter squared per second cubed kelvin"
    )]
    MilligramMeterSquaredPerSecondCubedKelvin,
    #[unit(
        9.9999999999999986e-10,
        "µg · m/(s³ · K)",
        "micrograms meter squared per second cubed kelvin"
    )]
    MicrogramMeterSquaredPerSecondCubedKelvin,
    #[unit(
        9.9999999999999998e-13,
        "ng · m²/(s³ · K)",
        "nanograms meter squared per second cubed kelvin"
    )]
    NanogramMeterSquaredPerSecondCubedKelvin,
    #[unit(
        1.0000000000000001e-15,
        "pg · m²/(s³ · K)",
        "picograms meter squared per second cubed kelvin"
    )]
    PicogramMeterSquaredPerSecondCubedKelvin,
    #[unit(
        1.0000000000000001e-18,
        "fg · m²/(s³ · K)",
        "femtograms meter squared per second cubed kelvin"
    )]
    FemtogramMeterSquaredPerSecondCubedKelvin,
    #[unit(
        1.0000000000000001e-21,
        "ag · m²/(s³ · K)",
        "attograms meter squared per second cubed kelvin"
    )]
    AttogramMeterSquaredPerSecondCubedKelvin,
    #[unit(
        9.9999999999999992e-25,
        "zg · m²/(s³ · K)",
        "zeptograms meter squared per second cubed kelvin"
    )]
    ZeptogramMeterSquaredPerSecondCubedKelvin,
    #[unit(
        9.9999999999999986e-28,
        "yg · m²/(s³ · K)",
        "yoctograms meter squared per second cubed kelvin"
    )]
    YoctogramMeterSquaredPerSecondCubedKelvin,
    #[unit(9.9999999999999998e+23, "YW/K", "yottawatts per kelvin")]
    YottawattPerKelvin,
    #[unit(1e+21, "ZW/K", "zettawatts per kelvin")]
    ZettawattPerKelvin,
    #[unit(1e+18, "EW/K", "exawatts per kelvin")]
    ExawattPerKelvin,
    #[unit(1000000000000000.0, "PW/K", "petawatts per kelvin")]
    PetawattPerKelvin,
    #[unit(1000000000000.0, "TW/K", "terawatts per kelvin")]
    TerawattPerKelvin,
    #[unit(1000000000.0, "GW/K", "gigawatts per kelvin")]
    GigawattPerKelvin,
    #[unit(1000000.0, "MW/K", "megawatts per kelvin")]
    MegawattPerKelvin,
    #[unit(1000.0, "kW/K", "kilowatts per kelvin")]
    KilowattPerKelvin,
    #[unit(100.0, "hW/K", "hectowatts per kelvin")]
    HectowattPerKelvin,
    #[unit(10.0, "daW/K", "decawatts per kelvin")]
    DecawattPerKelvin,
    #[unit(1.0, "W/K", "watts per kelvin")]
    WattPerKelvin,
    #[unit(0.10000000000000001, "dW/K", "deciwatts per kelvin")]
    DeciwattPerKelvin,
    #[unit(0.01, "cW/K", "centiwatts per kelvin")]
    CentiwattPerKelvin,
    #[unit(0.001, "mW/K", "milliwatts per kelvin")]
    MilliwattPerKelvin,
    #[unit(9.9999999999999995e-07, "µW/K", "microwatts per kelvin")]
    MicrowattPerKelvin,
    #[unit(1.0000000000000001e-09, "nW/K", "nanowatts per kelvin")]
    NanowattPerKelvin,
    #[unit(9.9999999999999998e-13, "pW/K", "picowatts per kelvin")]
    PicowattPerKelvin,
    #[unit(1.0000000000000001e-15, "fW/K", "femtowatts per kelvin")]
    FemtowattPerKelvin,
    #[unit(1.0000000000000001e-18, "aW/K", "attowatts per kelvin")]
    AttowattPerKelvin,
    #[unit(9.9999999999999991e-22, "zW/K", "zeptowatts per kelvin")]
    ZeptowattPerKelvin,
    #[unit(9.9999999999999992e-25, "yW/K", "yoctowatts per kelvin")]
    YoctowattPerKelvin,
    #[unit(
        1.0,
        "kg · m²/(s³ · °C)",
        "kilograms meter squared per second cubed degree Celsius"
    )]
    KilogramMeterSquaredPerSecondCubedDegreeCelsius,
    #[unit(1000.0, "kW/°C", "kilowatts per degree Celsius")]
    KilowattPerDegreeCelsius,
    #[unit(1.0, "W/°C", "watts per degree Celsius")]
    WattPerMeterDegreeCelsius,
    #[unit(0.001, "mW/°C", "milliwatts per degree Celsius")]
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

#[derive(Copy, Clone, Unit)]
#[dim(ThermalConductivity)]
pub enum ThermalConductivityUnit {
    #[unit(1e+21, "Yg · m/(s³ · K)", "yottagrams meter per second cubed kelvin")]
    YottagramMeterPerSecondCubedKelvin,
    #[unit(1e+18, "Zg · m/(s³ · K)", "zettagrams meter per second cubed kelvin")]
    ZettagramMeterPerSecondCubedKelvin,
    #[unit(
        1000000000000000.0,
        "Eg · m/(s³ · K)",
        "exagrams meter per second cubed kelvin"
    )]
    ExagramMeterPerSecondCubedKelvin,
    #[unit(
        1000000000000.0,
        "Pg · m/(s³ · K)",
        "petagrams meter per second cubed kelvin"
    )]
    PetagramMeterPerSecondCubedKelvin,
    #[unit(
        1000000000.0,
        "Tg · m/(s³ · K)",
        "teragrams meter per second cubed kelvin"
    )]
    TeragramMeterPerSecondCubedKelvin,
    #[unit(
        1000000.0,
        "Gg · m/(s³ · K)",
        "gigagrams meter per second cubed kelvin"
    )]
    GigagramMeterPerSecondCubedKelvin,
    #[unit(1000.0, "Mg · m/(s³ · K)", "megagrams meter per second cubed kelvin")]
    MegagramMeterPerSecondCubedKelvin,
    #[unit(1.0, "kg · m/(s³ · K)", "kilograms meter per second cubed kelvin")]
    KilogramMeterPerSecondCubedKelvin,
    #[unit(
        0.10000000000000001,
        "hg · m/(s³ · K)",
        "hectograms meter per second cubed kelvin"
    )]
    HectogramMeterPerSecondCubedKelvin,
    #[unit(0.01, "dag · m/(s³ · K)", "decagrams meter per second cubed kelvin")]
    DecagramMeterPerSecondCubedKelvin,
    #[unit(0.001, "g · m/(s³ · K)", "grams meter per second cubed kelvin")]
    GramMeterPerSecondCubedKelvin,
    #[unit(0.0001, "dg · m/(s³ · K)", "decigrams meter per second cubed kelvin")]
    DecigramMeterPerSecondCubedKelvin,
    #[unit(
        1.0000000000000001e-05,
        "cg · m/(s³ · K)",
        "centigrams meter per second cubed kelvin"
    )]
    CentigramMeterPerSecondCubedKelvin,
    #[unit(
        9.9999999999999995e-07,
        "mg · m/(s³ · K)",
        "milligrams meter per second cubed kelvin"
    )]
    MilligramMeterPerSecondCubedKelvin,
    #[unit(
        9.9999999999999986e-10,
        "µg · m/(s³ · K)",
        "micrograms meter per second cubed kelvin"
    )]
    MicrogramMeterPerSecondCubedKelvin,
    #[unit(
        9.9999999999999998e-13,
        "ng · m/(s³ · K)",
        "nanograms meter per second cubed kelvin"
    )]
    NanogramMeterPerSecondCubedKelvin,
    #[unit(
        1.0000000000000001e-15,
        "pg · m/(s³ · K)",
        "picograms meter per second cubed kelvin"
    )]
    PicogramMeterPerSecondCubedKelvin,
    #[unit(
        1.0000000000000001e-18,
        "fg · m/(s³ · K)",
        "femtograms meter per second cubed kelvin"
    )]
    FemtogramMeterPerSecondCubedKelvin,
    #[unit(
        1.0000000000000001e-21,
        "ag · m/(s³ · K)",
        "attograms meter per second cubed kelvin"
    )]
    AttogramMeterPerSecondCubedKelvin,
    #[unit(
        9.9999999999999992e-25,
        "zg · m/(s³ · K)",
        "zeptograms meter per second cubed kelvin"
    )]
    ZeptogramMeterPerSecondCubedKelvin,
    #[unit(
        9.9999999999999986e-28,
        "yg · m/(s³ · K)",
        "yoctograms meter per second cubed kelvin"
    )]
    YoctogramMeterPerSecondCubedKelvin,
    #[unit(9.9999999999999998e+23, "YW/(m · K)", "yottawatts per meter kelvin")]
    YottawattPerMeterKelvin,
    #[unit(1e+21, "ZW/(m · K)", "zettawatts per meter kelvin")]
    ZettawattPerMeterKelvin,
    #[unit(1e+18, "EW/(m · K)", "exawatts per meter kelvin")]
    ExawattPerMeterKelvin,
    #[unit(1000000000000000.0, "PW/(m · K)", "petawatts per meter kelvin")]
    PetawattPerMeterKelvin,
    #[unit(1000000000000.0, "TW/(m · K)", "terawatts per meter kelvin")]
    TerawattPerMeterKelvin,
    #[unit(1000000000.0, "GW/(m · K)", "gigawatts per meter kelvin")]
    GigawattPerMeterKelvin,
    #[unit(1000000.0, "MW/(m · K)", "megawatts per meter kelvin")]
    MegawattPerMeterKelvin,
    #[unit(1000.0, "kW/(m · K)", "kilowatts per meter kelvin")]
    KilowattPerMeterKelvin,
    #[unit(100.0, "hW/(m · K)", "hectowatts per meter kelvin")]
    HectowattPerMeterKelvin,
    #[unit(10.0, "daW/(m · K)", "decawatts per meter kelvin")]
    DecawattPerMeterKelvin,
    #[unit(1.0, "W/(m · K)", "watts per meter kelvin")]
    WattPerMeterKelvin,
    #[unit(0.10000000000000001, "dW/(m · K)", "deciwatts per meter kelvin")]
    DeciwattPerMeterKelvin,
    #[unit(0.01, "cW/(m · K)", "centiwatts per meter kelvin")]
    CentiwattPerMeterKelvin,
    #[unit(0.001, "mW/(m · K)", "milliwatts per meter kelvin")]
    MilliwattPerMeterKelvin,
    #[unit(9.9999999999999995e-07, "µW/(m · K)", "microwatts per meter kelvin")]
    MicrowattPerMeterKelvin,
    #[unit(1.0000000000000001e-09, "nW/(m · K)", "nanowatts per meter kelvin")]
    NanowattPerMeterKelvin,
    #[unit(9.9999999999999998e-13, "pW/(m · K)", "picowatts per meter kelvin")]
    PicowattPerMeterKelvin,
    #[unit(1.0000000000000001e-15, "fW/(m · K)", "femtowatts per meter kelvin")]
    FemtowattPerMeterKelvin,
    #[unit(1.0000000000000001e-18, "aW/(m · K)", "attowatts per meter kelvin")]
    AttowattPerMeterKelvin,
    #[unit(9.9999999999999991e-22, "zW/(m · K)", "zeptowatts per meter kelvin")]
    ZeptowattPerMeterKelvin,
    #[unit(9.9999999999999992e-25, "yW/(m · K)", "yoctowatts per meter kelvin")]
    YoctowattPerMeterKelvin,
    #[unit(
        1.0,
        "kg · m/(s³ · °C)",
        "kilograms meter per second cubed degree celsius"
    )]
    KilogramMeterPerSecondCubedDegreeCelsius,
    #[unit(1000.0, "kW/(m · °C)", "kilowatts per meter degree celsius")]
    KilowattPerMeterDegreeCelsius,
    #[unit(1.0, "W/(m · °C)", "watts per meter degree celsius")]
    WattPerMeterDegreeCelsius,
    #[unit(0.001, "mW/(m · °C)", "milliwatts per meter degree celsius")]
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

#[derive(Copy, Clone, Unit)]
#[dim(ThermalResistance)]
pub enum ThermalResistanceUnit {
    #[unit(1.0000000000000001e-24, "K/YW", "kelvins per yottawatt")]
    KelvinPerYottawatt,
    #[unit(9.9999999999999991e-22, "K/ZW", "kelvins per zettawatt")]
    KelvinPerZettawatt,
    #[unit(1.0000000000000001e-18, "K/EW", "kelvins per exawatt")]
    KelvinPerExawatt,
    #[unit(1.0000000000000001e-15, "K/PW", "kelvins per petawatt")]
    KelvinPerPetawatt,
    #[unit(9.9999999999999998e-13, "K/TW", "kelvins per terawatt")]
    KelvinPerTerawatt,
    #[unit(1.0000000000000001e-09, "K/GW", "kelvins per gigawatt")]
    KelvinPerGigawatt,
    #[unit(9.9999999999999995e-07, "K/MW", "kelvins per megawatt")]
    KelvinPerMegawatt,
    #[unit(0.001, "K/kw", "kelvins per kilowatt")]
    KelvinPerKilowatt,
    #[unit(0.01, "K/hW", "kelvins per hectowatt")]
    KelvinPerHectowatt,
    #[unit(0.10000000000000001, "K/daW", "kelvins per decawatt")]
    KelvinPerDecawatt,
    #[unit(1.0, "K/W", "kelvins per watt")]
    KelvinPerWatt,
    #[unit(10.0, "K/dW", "kelvins per deciwatt")]
    KelvinPerDeciwatt,
    #[unit(100.0, "K/cW", "kelvins per centiwatt")]
    KelvinPerCentiwatt,
    #[unit(1000.0, "K/mW", "kelvins per milliwatt")]
    KelvinPerMilliwatt,
    #[unit(1000000.0, "K/µW", "kelvins per microwatt")]
    KelvinPerMicrowatt,
    #[unit(999999999.99999988, "K/nW", "kelvins per nanowatt")]
    KelvinPerNanowatt,
    #[unit(1000000000000.0, "K/pW", "kelvins per picowatt")]
    KelvinPerPicowatt,
    #[unit(999999999999999.88, "K/fW", "kelvins per femtowatt")]
    KelvinPerFemtowatt,
    #[unit(9.9999999999999987e+17, "K/aW", "kelvins per attowatt")]
    KelvinPerAttowatt,
    #[unit(1.0000000000000001e+21, "K/zW", "kelvins per zeptowatt")]
    KelvinPerZeptowatt,
    #[unit(1.0000000000000001e+24, "K/yW", "kelvins per yoctowatt")]
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

#[derive(Copy, Clone, Unit)]
#[dim(SpecificHeatCapacity)]
pub enum SpecificHeatCapacityUnit {
    #[unit(
        1000000.0,
        "km²/(s² · K)",
        "square kilometers per second squared kelvin"
    )]
    SquareKilometerPerSecondSquaredKelvin,
    #[unit(1.0, "m²/(s² · K)", "square meters per second squared kelvin")]
    SquareMeterPerSecondSquaredKelvin,
    #[unit(0.0001, "cm²/(s² · K)", "square centimeters per second squared kelvin")]
    SquareCentimeterPerSecondSquaredKelvin,
    #[unit(
        9.9999999999999995e-07,
        "mm²/(s² · K)",
        "square millimeters per second squared kelvin"
    )]
    SquareMillimeterPerSecondSquaredKelvin,
    #[unit(
        9.9999999999999998e-13,
        "µm²/(s² · K)",
        "square micrometers per second squared kelvin"
    )]
    SquareMicrometerPerSecondSquaredKelvin,
    #[unit(
        9.9999999999999998e+23,
        "YJ/(kg · K)",
        "yottajoules per kilogram kelvin"
    )]
    YottajoulePerKilogramKelvin,
    #[unit(1e+21, "ZJ/(kg · K)", "zettajoules per kilogram kelvin")]
    ZettajoulePerKilogramKelvin,
    #[unit(1e+18, "EJ/(kg · K)", "exajoules per kilogram kelvin")]
    ExajoulePerKilogramKelvin,
    #[unit(1000000000000000.0, "PJ/(kg · K)", "petajoules per kilogram kelvin")]
    PetajoulePerKilogramKelvin,
    #[unit(1000000000000.0, "TJ/(kg · K)", "terajoules per kilogram kelvin")]
    TerajoulePerKilogramKelvin,
    #[unit(1000000000.0, "GJ/(kg · K)", "gigajoules per kilogram kelvin")]
    GigajoulePerKilogramKelvin,
    #[unit(1000000.0, "MJ/(kg · K)", "megajoules per kilogram kelvin")]
    MegajoulePerKilogramKelvin,
    #[unit(1000.0, "kJ/(kg · K)", "kilojoules per kilogram kelvin")]
    KilojoulePerKilogramKelvin,
    #[unit(100.0, "hJ/(kg · K)", "hectojoules per kilogram kelvin")]
    HectojoulePerKilogramKelvin,
    #[unit(10.0, "daJ/(kg · K)", "decajoules per kilogram kelvin")]
    DecajoulePerKilogramKelvin,
    #[unit(1.0, "J/(kg · K)", "joules per kilogram kelvin")]
    JoulePerKilogramKelvin,
    #[unit(0.10000000000000001, "dJ/(kg · K)", "decijoules per kilogram kelvin")]
    DecijoulePerKilogramKelvin,
    #[unit(0.01, "cJ/(kg · K)", "centijoules per kilogram kelvin")]
    CentijoulePerKilogramKelvin,
    #[unit(0.001, "mJ/(kg · K)", "millijoules per kilogram kelvin")]
    MillijoulePerKilogramKelvin,
    #[unit(
        9.9999999999999995e-07,
        "µJ/(kg · K)",
        "microjoules per kilogram kelvin"
    )]
    MicrojoulePerKilogramKelvin,
    #[unit(
        1.0000000000000001e-09,
        "nJ/(kg · K)",
        "nanojoules per kilogram kelvin"
    )]
    NanojoulePerKilogramKelvin,
    #[unit(
        9.9999999999999998e-13,
        "pJ/(kg · K)",
        "picojoules per kilogram kelvin"
    )]
    PicojoulePerKilogramKelvin,
    #[unit(
        1.0000000000000001e-15,
        "fJ/(kg · K)",
        "femtojoules per kilogram kelvin"
    )]
    FemtojoulePerKilogramKelvin,
    #[unit(
        1.0000000000000001e-18,
        "aJ/(kg · K)",
        "attojoules per kilogram kelvin"
    )]
    AttojoulePerKilogramKelvin,
    #[unit(
        9.9999999999999991e-22,
        "zJ/(kg · K)",
        "zeptojoules per kilogram kelvin"
    )]
    ZeptojoulePerKilogramKelvin,
    #[unit(
        9.9999999999999992e-25,
        "yJ/(kg · K)",
        "yoctojoules per kilogram kelvin"
    )]
    YoctojoulePerKilogramKelvin,
    #[unit(1000.0, "kJ/(kg · °C)", "kilojoules per kilogram degree celsius")]
    KilojoulePerKilogramDegreeCelsius,
    #[unit(1000000.0, "kJ/(g · °C)", "kilojoules per gram degree celsius")]
    KilojoulePerGramDegreeCelsius,
    #[unit(1.0, "J/(kg · °C)", "joules per kilogram degree celsius")]
    JoulePerKilogramDegreeCelsius,
    #[unit(1000.0, "J/(g · °C)", "joules per gram degree celsius")]
    JoulePerGramDegreeCelsius,
    #[unit(0.001, "mJ/(kg · °C)", "millijoules per kilogram degree celsius")]
    MillijoulePerKilogramDegreeCelsius,
    #[unit(1.0, "mJ/(g · °C)", "millijoules per gram degree celsius")]
    MillijoulePerGramDegreeCelsius,
    #[unit(
        66943.990586083979,
        "Btu/(oz · °F)",
        "British thermal units per ounce degree Fahrenheit"
    )]
    BtuPerOunceDegreeFahrenheit,
    #[unit(
        66988.81674187076,
        "Btu (IT)/(oz · °F)",
        "British thermal units (IT) per ounce degree Fahrenheit"
    )]
    BtuItPerOunceDegreeFahrenheit,
    #[unit(
        4183.9986736991177,
        "Btu/(lb · °F)",
        "British thermal units per pound degree Fahrenheit"
    )]
    BtuPerPoundDegreeFahrenheit,
    #[unit(
        4186.8003079416667,
        "Btu (IT)/(lb · °F)",
        "British thermal units (IT) per pound degree Fahrenheit"
    )]
    BtuItPerPoundDegreeFahrenheit,
    #[unit(
        1.8978299999999999,
        "Btu/(t · °F)",
        "British thermal units per ton degree Fahrenheit"
    )]
    BtuPerTonDegreeFahrenheit,
    #[unit(
        1.8991008,
        "Btu (IT)/(t · °F)",
        "British thermal units (IT) per ton degree Fahrenheit"
    )]
    BtuItPerTonDegreeFahrenheit,
    #[unit(4.1840000000000002, "cal/(kg · K)", "calories per kilogram kelvin")]
    CaloriePerKilogramKelvin,
    #[unit(4184.0, "cal/(g · K)", "calories per gram kelvin")]
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

#[derive(Copy, Clone, Unit)]
#[dim(ArealHeatCapacity)]
pub enum ArealHeatCapacityUnit {
    #[unit(1.0, "J/(m² · K)", "joules per square meter kelvin")]
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

#[derive(Copy, Clone, Unit)]
#[dim(VolumetricHeatCapacity)]
pub enum VolumetricHeatCapacityUnit {
    #[unit(1.0, "J/(m³ · K)", "joules per cubic meter kelvin")]
    JoulePerCubicMeterKelvin,
    #[unit(4.1840000000000002, "cal/(m³ · K)", "calories per cubic meter kelvin")]
    CaloriePerCubicMeterKelvin,
}
