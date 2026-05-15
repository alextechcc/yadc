use crate::{ISQ, StaticSIQuantity};
use yadc::{Dimension, Unit};

pub type MassDensity = StaticSIQuantity<
    {
        ISQ {
            m: -3,
            kg: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(MassDensity)]
pub enum MassDensityUnit {
    #[linear("Yg/m³", "yottagrams per cubic meter", 1e+21)]
    YottagramPerCubicMeter,
    #[linear("Zg/m³", "zettagrams per cubic meter", 1e+18)]
    ZettagramPerCubicMeter,
    #[linear("Eg/m³", "exagrams per cubic meter", 1000000000000000.0)]
    ExagramPerCubicMeter,
    #[linear("Pg/m³", "petagrams per cubic meter", 1000000000000.0)]
    PetagramPerCubicMeter,
    #[linear("Tg/m³", "teragrams per cubic meter", 1000000000.0)]
    TeragramPerCubicMeter,
    #[linear("Gg/m³", "gigagrams per cubic meter", 1000000.0)]
    GigagramPerCubicMeter,
    #[linear("Mg/m³", "megagrams per cubic meter", 1000.0)]
    MegagramPerCubicMeter,
    #[linear("kg/m³", "kilograms per cubic meter", 1.0)]
    KilogramPerCubicMeter,
    #[linear("hg/m³", "hectograms per cubic meter", 0.10000000000000001)]
    HectogramPerCubicMeter,
    #[linear("dag/m³", "decagrams per cubic meter", 0.01)]
    DecagramPerCubicMeter,
    #[linear("g/m³", "grams per cubic meter", 0.001)]
    GramPerCubicMeter,
    #[linear("dg/m³", "decigrams per cubic meter", 0.0001)]
    DecigramPerCubicMeter,
    #[linear("cg/m³", "centigrams per cubic meter", 1.0000000000000001e-05)]
    CentigramPerCubicMeter,
    #[linear("mg/m³", "milligrams per cubic meter", 9.9999999999999995e-07)]
    MilligramPerCubicMeter,
    #[linear("µg/m³", "micrograms per cubic meter", 9.9999999999999986e-10)]
    MicrogramPerCubicMeter,
    #[linear("ng/m³", "nanograms per cubic meter", 9.9999999999999998e-13)]
    NanogramPerCubicMeter,
    #[linear("pg/m³", "picograms per cubic meter", 1.0000000000000001e-15)]
    PicogramPerCubicMeter,
    #[linear("fg/m³", "femtograms per cubic meter", 1.0000000000000001e-18)]
    FemtogramPerCubicMeter,
    #[linear("ag/m³", "attograms per cubic meter", 1.0000000000000001e-21)]
    AttogramPerCubicMeter,
    #[linear("zg/m³", "zeptograms per cubic meter", 9.9999999999999992e-25)]
    ZeptogramPerCubicMeter,
    #[linear("yg/m³", "yoctograms per cubic meter", 9.9999999999999986e-28)]
    YoctogramPerCubicMeter,
    #[linear("ct/m³", "carats per cubic meter", 0.00020000000000000001)]
    CaratPerCubicMeter,
    #[linear("gr/m³", "grains per cubic meter", 6.4798909999999995e-05)]
    GrainPerCubicMeter,
    #[linear(
        "cwt long/m³",
        "hundredweight (long) per cubic meter",
        50.802349999999997
    )]
    HundredweightLongPerCubicMeter,
    #[linear("cwt short/m³", "hundredweight (short) per cubic meter", 45.35924)]
    HundredweightShortPerCubicMeter,
    #[linear("oz/m³", "ounces per cubic meter", 0.02834952)]
    OuncePerCubicMeter,
    #[linear("oz t/m³", "troy ounces per cubic meter", 0.031103479999999999)]
    OunceTroyPerCubicMeter,
    #[linear("dwt/m³", "pennyweight per cubic meter", 0.0015551740000000001)]
    PennyweightPerCubicMeter,
    #[linear("lb/m³", "pounds per cubic meter", 0.45359240000000001)]
    PoundPerCubicMeter,
    #[linear("lb t/m³", "troy pounds per cubic meter", 0.37324170000000001)]
    PoundTroyPerCubicMeter,
    #[linear("slug/m³", "slugs per cubic meter", 14.5939)]
    SlugPerCubicMeter,
    #[linear("AT/m³", "assay tons per cubic meter", 0.029166669999999999)]
    TonAssayPerCubicMeter,
    #[linear("2240 lb/m³", "long tons per cubic meter", 1016.047)]
    TonLongPerCubicMeter,
    #[linear("2000 lb/m³", "short tons per cubic meter", 907.18470000000002)]
    TonShortPerCubicMeter,
    #[linear("t/m³", "tons per cubic meter", 1000.0)]
    TonPerCubicMeter,
    #[linear("gr/gal", "grains per gallon", 0.01711806006849452)]
    GrainPerGallon,
    #[linear("g/cm³", "grams per cubic centimeter", 1000.0)]
    GramPerCubicCentimeter,
    #[linear("oz/in³", "ounces per cubic inch", 1729.9942759714065)]
    OuncePerCubicInch,
    #[linear("oz/gal (UK)", "ounces per Imperial gallon", 6.2360226040399551)]
    OuncePerGallonImperial,
    #[linear("oz/gal", "ounces per gallon", 7.4891504544287377)]
    OuncePerGallon,
    #[linear("lb/ft³", "pounds per cubic foot", 16.018462505539986)]
    PoundPerCubicFoot,
    #[linear("lb/in³", "pounds per cubic inch", 27679.913297443225)]
    PoundPerCubicInch,
    #[linear("lb/yd³", "pounds per cubic yard", 0.59327642789288249)]
    PoundPerCubicYard,
    #[linear("lb/gal (UK)", "pounds per Imperial gallon", 99.776379262179148)]
    PoundPerGallonImperial,
    #[linear("lb/gal", "pounds per gallon", 119.8264284046228)]
    PoundPerGallon,
    #[linear("slug/ft³", "slugs per cubic foot", 515.37865263968274)]
    SlugPerCubicFoot,
    #[linear("2240 lb/yd³", "long tons per cubic yard", 1328.9392298708699)]
    TonLongPerCubicYard,
    #[linear("2000 lb/yd³", "short tons per cubic yard", 1186.5527249907102)]
    TonShortPerCubicYard,
    #[linear("2240 lb/ft³", "long tons per cubic foot", 35881.356859961474)]
    TonLongPerCubicFoot,
    #[linear("2000 lb/ft³", "short tons per cubic foot", 32036.921479613728)]
    TonShortPerCubicFoot,
}

pub type LinearMassDensity = StaticSIQuantity<
    {
        ISQ {
            m: -1,
            kg: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(LinearMassDensity)]
pub enum LinearMassDensityUnit {
    #[linear("kg/m", "kilograms per meter", 1.0)]
    KilogramPerMeter,
    #[linear("g/km", "grams per kilometer", 9.9999999999999995e-07)]
    GramPerKilometer,
    #[linear("g/cm", "grams per centimeter", 0.10000000000000001)]
    GramPerCentimeter,
    #[linear("oz/ft", "ounces per foot", 0.093010236220472428)]
    OuncePerFoot,
    #[linear("oz/in", "ounces per inch", 1.1161228346456693)]
    OuncePerInch,
    #[linear("lb/yd", "pounds per yard", 0.49605468066491692)]
    PoundPerYard,
    #[linear("lb/ft", "pounds per foot", 1.4881640419947506)]
    PoundPerFoot,
    #[linear("lb/in", "pounds per inch", 17.857968503937009)]
    PoundPerInch,
}

pub type ArealMassDensity = StaticSIQuantity<
    {
        ISQ {
            m: -2,
            kg: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(ArealMassDensity)]
pub enum ArealMassDensityUnit {
    #[linear("kg/m²", "kilograms per square meter", 1.0)]
    KilogramPerSquareMeter,
    #[linear("g/m²", "grams per square meter", 0.001)]
    GramPerSquareMeter,
    #[linear("oz/ft²", "ounces per square foot", 0.30515169363672057)]
    OuncePerSquareFoot,
}

pub type LinearNumberDensity = StaticSIQuantity<{ ISQ { m: -1, ..ISQ::ZERO } }>;

#[derive(Unit, Copy, Clone)]
#[dim(LinearNumberDensity)]
pub enum LinearNumberDensityUnit {
    #[linear("km⁻¹", "per kilometer", 0.001)]
    PerKilometer,
    #[linear("m⁻¹", "per meter", 1.0)]
    PerMeter,
    #[linear("dm⁻¹", "per decimeter", 10.0)]
    PerDecimeter,
    #[linear("cm⁻¹", "per centimeter", 100.0)]
    PerCentimeter,
    #[linear("mm⁻¹", "per millimeter", 1000.0)]
    PerMillimeter,
    #[linear("ft⁻¹", "per foot", 3.280839895013123)]
    PerFoot,
    #[linear("ft (U.S. survey)", "foot (U.S. survey)", 3.280833436679587)]
    PerFootSurvey,
    #[linear("in⁻¹", "per inch", 39.370078740157481)]
    PerInch,
    #[linear("mi⁻¹", "per mile", 0.00062137119223733392)]
    PerMile,
    #[linear(
        "mi⁻¹ (U.S. survey)",
        "per mile (U.S. survey)",
        0.00062137003393301752
    )]
    PerMileSurvey,
    #[linear("M⁻¹", "per nautical mile", 0.00053995680345572358)]
    PerNauticalMile,
    #[linear("yd⁻¹", "per yard", 1.0936132983377078)]
    PerYard,
}

pub type ArealNumberDensity = StaticSIQuantity<{ ISQ { m: -2, ..ISQ::ZERO } }>;

#[derive(Unit, Copy, Clone)]
#[dim(ArealNumberDensity)]
pub enum ArealNumberDensityUnit {
    #[linear("m⁻²", "per square meter", 1.0)]
    PerSquareMeter,
    #[linear("ac⁻²", "per acre", 0.00024710436922532533)]
    PerAcre,
    #[linear("a⁻²", "per are", 0.01)]
    PerAre,
    #[linear("b⁻²", "per barn", 9.9999999999999996e+27)]
    PerBarn,
    #[linear("cmil⁻²", "per circular mil", 1973525159.9788833)]
    PerCircularMil,
    #[linear("ha⁻²", "per hectare", 0.0001)]
    PerHectare,
    #[linear("ft⁻²", "per square foot", 10.763910416709722)]
    PerSquareFoot,
    #[linear("in⁻²", "per square inch", 1550.0031000062002)]
    PerSquareInch,
    #[linear("mi⁻²", "per square mile", 3.8610215854244582e-07)]
    PerSquareMile,
    #[linear("yd⁻²", "per square yard", 1.1959900463010802)]
    PerSquareYard,
}

pub type VolumetricNumberDensity = StaticSIQuantity<{ ISQ { m: -3, ..ISQ::ZERO } }>;

#[derive(Unit, Copy, Clone)]
#[dim(VolumetricNumberDensity)]
pub enum VolumetricNumberDensityUnit {
    #[linear("m⁻³", "per cubic meter", 1.0)]
    PerCubicMeter,
    #[linear("ft⁻³", "per cubic foot", 35.314662471284763)]
    PerCubicFoot,
    #[linear("in⁻³", "per cubic inch", 61023.758990325288)]
    PerCubicInch,
    #[linear("mi⁻³", "per cubic mile", 2.39912748531614e-10)]
    PerCubicMile,
    #[linear("yd⁻³", "per cubic yard", 1.3079505474361619)]
    PerCubicYard,
    #[linear("per fl oz", "per fluid ounce", 33814.0222016107)]
    PerFluidOunce,
    #[linear("per fl oz (UK)", "per Imperial fluid ounce", 35195.082824588411)]
    PerFluidOunceImperial,
    #[linear("per gal (UK)", "per Imperial gallon", 219.96924829908778)]
    PerGallonImperial,
    #[linear("per gal", "per gallon", 264.17203728418463)]
    PerGallon,
    #[linear("L⁻¹", "per liter", 1000.0)]
    PerLiter,
}

pub type LinearNumberRate = StaticSIQuantity<
    {
        ISQ {
            m: -1,
            s: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(LinearNumberRate)]
pub enum LinearNumberRateUnit {
    #[linear("km⁻¹ · s⁻¹", "per kilometer second", 0.001)]
    PerKilometerSecond,
    #[linear("m⁻¹ · s⁻¹", "per meter second", 1.0)]
    PerMeterSecond,
    #[linear("cm⁻¹ · s⁻¹", "per centimeter second", 100.0)]
    PerCentimeterSecond,
    #[linear("mm⁻¹ · s⁻¹", "per millimeter second", 1000.0)]
    PerMillimeterSecond,
    #[linear("ft⁻¹ · s⁻¹", "per foot second", 3.280839895013123)]
    PerFootSecond,
    #[linear(
        "ft⁻¹ (U.S. survey) · s⁻¹",
        "per foot (U.S. survey) second",
        3.280833436679587
    )]
    PerFootSurveySecond,
    #[linear("in⁻¹ · s⁻¹", "per inch second", 39.370078740157481)]
    PerInchSecond,
    #[linear("mi⁻¹ · s⁻¹", "per mile second", 0.00062137119223733392)]
    PerMileSecond,
    #[linear(
        "mi⁻¹ (U.S. survey) · s⁻¹",
        "per mile (U.S. survey) second",
        0.00062137003393301752
    )]
    PerMileSurveySecond,
    #[linear("yd⁻¹ · s⁻¹", "per yard second", 1.0936132983377078)]
    PerYardSecond,
}

pub type ArealNumberRate = StaticSIQuantity<
    {
        ISQ {
            m: -2,
            s: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(ArealNumberRate)]
pub enum ArealNumberRateUnit {
    #[linear("m⁻² · s⁻¹", "per square meter second", 1.0)]
    PerSquareMeterSecond,
    #[linear("ac⁻¹ · s⁻¹", "per acre second", 0.00024710436922532533)]
    PerAcreSecond,
    #[linear("a⁻¹ · s⁻¹", "per are second", 0.01)]
    PerAreSecond,
    #[linear("b⁻¹ · s⁻¹", "per barn second", 9.9999999999999996e+27)]
    PerBarnSecond,
    #[linear("cmil⁻¹ · s⁻¹", "per circular mil second", 1973525159.9788833)]
    PerCircularMilSecond,
    #[linear("ha⁻¹ · s⁻¹", "per hectare second", 0.0001)]
    PerHectareSecond,
    #[linear("ft⁻² · s⁻¹", "per square foot second", 10.763910416709722)]
    PerSquareFootSecond,
    #[linear("in⁻² · s⁻¹", "per square inch second", 1550.0031000062002)]
    PerSquareInchSecond,
    #[linear("mi⁻² · s⁻¹", "per square mile second", 3.8610215854244582e-07)]
    PerSquareMileSecond,
    #[linear("yd⁻² · s⁻¹", "per square yard second", 1.1959900463010802)]
    PerSquareYardSecond,
}

pub type VolumetricNumberRate = StaticSIQuantity<
    {
        ISQ {
            m: -3,
            s: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(VolumetricNumberRate)]
pub enum VolumetricNumberRateUnit {
    #[linear("m⁻³ · s⁻¹", "per cubic meter second", 1.0)]
    PerCubicMeterSecond,
    #[linear("ft⁻³ · s⁻¹", "per cubic foot second", 35.314662471284763)]
    PerCubicFootSecond,
    #[linear("in⁻³ · s⁻¹", "per cubic inch second", 61023.758990325288)]
    PerCubicInchSecond,
    #[linear("mi⁻³ · s⁻¹", "per cubic mile second", 2.39912748531614e-10)]
    PerCubicMileSecond,
    #[linear("yd⁻³ · s⁻¹", "per cubic yard second", 1.3079505474361619)]
    PerCubicYardSecond,
    #[linear("fl oz⁻¹ · s⁻¹", "per fluid ounce second", 33814.0222016107)]
    PerFluidOunceSecond,
    #[linear(
        "fl oz⁻¹ (UK) · s⁻¹",
        "per Imperial fluid ounce second",
        35195.082824588411
    )]
    PerFluidOunceImperialSecond,
    #[linear("gal⁻¹ (UK) · s⁻¹", "per Imperial gallon second", 219.96924829908778)]
    PerGallonImperialSecond,
    #[linear("gal⁻¹ · s⁻¹", "per gallon second", 264.17203728418463)]
    PerGallonSecond,
    #[linear("L⁻¹ · s⁻¹", "per liter second", 1000.0)]
    PerLiterSecond,
    #[linear("Bq/m³", "becquerels per cubic meter", 1.0)]
    BecquerelPerCubicMeter,
    #[linear("Ci/m³", "curies per cubic meter", 37000000000.0)]
    CuriePerCubicMeter,
    #[linear(
        "dpm/m³",
        "disintegrations per minute per cubic meter",
        0.016666666666666666
    )]
    DisintegrationsPerMinutePerCubicMeter,
}

pub type LinearDensityOfStates = StaticSIQuantity<
    {
        ISQ {
            m: -3,
            kg: -1,
            s: 2,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(LinearDensityOfStates)]
pub enum LinearDensityOfStatesUnit {
    #[linear("1/(m · J)", "states per meter joule", 1.0)]
    StatePerMeterJoule,
    #[linear("1/(cm · J)", "states per centimeter joule", 100.0)]
    StatePerCentimeterJoule,
}

pub type ArealDensityOfStates = StaticSIQuantity<
    {
        ISQ {
            m: -4,
            kg: -1,
            s: 2,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(ArealDensityOfStates)]
pub enum ArealDensityOfStatesUnit {
    #[linear("1/(m² · J)", "states per square meter joule", 1.0)]
    StatePerSquareMeterJoule,
}

pub type VolumetricDensityOfStates = StaticSIQuantity<
    {
        ISQ {
            m: -5,
            kg: -1,
            s: 2,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(VolumetricDensityOfStates)]
pub enum VolumetricDensityOfStatesUnit {
    #[linear("1/(m³ · J)", "states per cubic meter joule", 1.0)]
    StatePerCubicMeterJoule,
}

pub type LinearPowerDensity = StaticSIQuantity<
    {
        ISQ {
            m: 1,
            kg: 1,
            s: -3,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(LinearPowerDensity)]
pub enum LinearPowerDensityUnit {
    #[linear("W/m", "watts per meter", 1.0)]
    WattPerMeter,
    #[linear("W/cm", "watts per centimeter", 100.0)]
    WattPerCentimeter,
    #[linear("W/mm", "watts per millimeter", 1000.0)]
    WattPerMillimeter,
}

pub type VolumetricPowerDensity = StaticSIQuantity<
    {
        ISQ {
            m: -1,
            kg: 1,
            s: -3,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(VolumetricPowerDensity)]
pub enum VolumetricPowerDensityUnit {
    #[linear("W/m³", "watts per cubic meter", 1.0)]
    WattPerCubicMeter,
}
