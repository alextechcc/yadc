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

#[derive(Copy, Clone, Unit)]
#[dim(MassDensity)]
pub enum MassDensityUnit {
    #[unit(1e+21, "Yg/m³", "yottagrams per cubic meter")]
    YottagramPerCubicMeter,
    #[unit(1e+18, "Zg/m³", "zettagrams per cubic meter")]
    ZettagramPerCubicMeter,
    #[unit(1000000000000000.0, "Eg/m³", "exagrams per cubic meter")]
    ExagramPerCubicMeter,
    #[unit(1000000000000.0, "Pg/m³", "petagrams per cubic meter")]
    PetagramPerCubicMeter,
    #[unit(1000000000.0, "Tg/m³", "teragrams per cubic meter")]
    TeragramPerCubicMeter,
    #[unit(1000000.0, "Gg/m³", "gigagrams per cubic meter")]
    GigagramPerCubicMeter,
    #[unit(1000.0, "Mg/m³", "megagrams per cubic meter")]
    MegagramPerCubicMeter,
    #[unit(1.0, "kg/m³", "kilograms per cubic meter")]
    KilogramPerCubicMeter,
    #[unit(0.10000000000000001, "hg/m³", "hectograms per cubic meter")]
    HectogramPerCubicMeter,
    #[unit(0.01, "dag/m³", "decagrams per cubic meter")]
    DecagramPerCubicMeter,
    #[unit(0.001, "g/m³", "grams per cubic meter")]
    GramPerCubicMeter,
    #[unit(0.0001, "dg/m³", "decigrams per cubic meter")]
    DecigramPerCubicMeter,
    #[unit(1.0000000000000001e-05, "cg/m³", "centigrams per cubic meter")]
    CentigramPerCubicMeter,
    #[unit(9.9999999999999995e-07, "mg/m³", "milligrams per cubic meter")]
    MilligramPerCubicMeter,
    #[unit(9.9999999999999986e-10, "µg/m³", "micrograms per cubic meter")]
    MicrogramPerCubicMeter,
    #[unit(9.9999999999999998e-13, "ng/m³", "nanograms per cubic meter")]
    NanogramPerCubicMeter,
    #[unit(1.0000000000000001e-15, "pg/m³", "picograms per cubic meter")]
    PicogramPerCubicMeter,
    #[unit(1.0000000000000001e-18, "fg/m³", "femtograms per cubic meter")]
    FemtogramPerCubicMeter,
    #[unit(1.0000000000000001e-21, "ag/m³", "attograms per cubic meter")]
    AttogramPerCubicMeter,
    #[unit(9.9999999999999992e-25, "zg/m³", "zeptograms per cubic meter")]
    ZeptogramPerCubicMeter,
    #[unit(9.9999999999999986e-28, "yg/m³", "yoctograms per cubic meter")]
    YoctogramPerCubicMeter,
    #[unit(0.00020000000000000001, "ct/m³", "carats per cubic meter")]
    CaratPerCubicMeter,
    #[unit(6.4798909999999995e-05, "gr/m³", "grains per cubic meter")]
    GrainPerCubicMeter,
    #[unit(
        50.802349999999997,
        "cwt long/m³",
        "hundredweight (long) per cubic meter"
    )]
    HundredweightLongPerCubicMeter,
    #[unit(45.35924, "cwt short/m³", "hundredweight (short) per cubic meter")]
    HundredweightShortPerCubicMeter,
    #[unit(0.02834952, "oz/m³", "ounces per cubic meter")]
    OuncePerCubicMeter,
    #[unit(0.031103479999999999, "oz t/m³", "troy ounces per cubic meter")]
    OunceTroyPerCubicMeter,
    #[unit(0.0015551740000000001, "dwt/m³", "pennyweight per cubic meter")]
    PennyweightPerCubicMeter,
    #[unit(0.45359240000000001, "lb/m³", "pounds per cubic meter")]
    PoundPerCubicMeter,
    #[unit(0.37324170000000001, "lb t/m³", "troy pounds per cubic meter")]
    PoundTroyPerCubicMeter,
    #[unit(14.5939, "slug/m³", "slugs per cubic meter")]
    SlugPerCubicMeter,
    #[unit(0.029166669999999999, "AT/m³", "assay tons per cubic meter")]
    TonAssayPerCubicMeter,
    #[unit(1016.047, "2240 lb/m³", "long tons per cubic meter")]
    TonLongPerCubicMeter,
    #[unit(907.18470000000002, "2000 lb/m³", "short tons per cubic meter")]
    TonShortPerCubicMeter,
    #[unit(1000.0, "t/m³", "tons per cubic meter")]
    TonPerCubicMeter,
    #[unit(0.01711806006849452, "gr/gal", "grains per gallon")]
    GrainPerGallon,
    #[unit(1000.0, "g/cm³", "grams per cubic centimeter")]
    GramPerCubicCentimeter,
    #[unit(1729.9942759714065, "oz/in³", "ounces per cubic inch")]
    OuncePerCubicInch,
    #[unit(6.2360226040399551, "oz/gal (UK)", "ounces per Imperial gallon")]
    OuncePerGallonImperial,
    #[unit(7.4891504544287377, "oz/gal", "ounces per gallon")]
    OuncePerGallon,
    #[unit(16.018462505539986, "lb/ft³", "pounds per cubic foot")]
    PoundPerCubicFoot,
    #[unit(27679.913297443225, "lb/in³", "pounds per cubic inch")]
    PoundPerCubicInch,
    #[unit(0.59327642789288249, "lb/yd³", "pounds per cubic yard")]
    PoundPerCubicYard,
    #[unit(99.776379262179148, "lb/gal (UK)", "pounds per Imperial gallon")]
    PoundPerGallonImperial,
    #[unit(119.8264284046228, "lb/gal", "pounds per gallon")]
    PoundPerGallon,
    #[unit(515.37865263968274, "slug/ft³", "slugs per cubic foot")]
    SlugPerCubicFoot,
    #[unit(1328.9392298708699, "2240 lb/yd³", "long tons per cubic yard")]
    TonLongPerCubicYard,
    #[unit(1186.5527249907102, "2000 lb/yd³", "short tons per cubic yard")]
    TonShortPerCubicYard,
    #[unit(35881.356859961474, "2240 lb/ft³", "long tons per cubic foot")]
    TonLongPerCubicFoot,
    #[unit(32036.921479613728, "2000 lb/ft³", "short tons per cubic foot")]
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

#[derive(Copy, Clone, Unit)]
#[dim(LinearMassDensity)]
pub enum LinearMassDensityUnit {
    #[unit(1.0, "kg/m", "kilograms per meter")]
    KilogramPerMeter,
    #[unit(9.9999999999999995e-07, "g/km", "grams per kilometer")]
    GramPerKilometer,
    #[unit(0.10000000000000001, "g/cm", "grams per centimeter")]
    GramPerCentimeter,
    #[unit(0.093010236220472428, "oz/ft", "ounces per foot")]
    OuncePerFoot,
    #[unit(1.1161228346456693, "oz/in", "ounces per inch")]
    OuncePerInch,
    #[unit(0.49605468066491692, "lb/yd", "pounds per yard")]
    PoundPerYard,
    #[unit(1.4881640419947506, "lb/ft", "pounds per foot")]
    PoundPerFoot,
    #[unit(17.857968503937009, "lb/in", "pounds per inch")]
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

#[derive(Copy, Clone, Unit)]
#[dim(ArealMassDensity)]
pub enum ArealMassDensityUnit {
    #[unit(1.0, "kg/m²", "kilograms per square meter")]
    KilogramPerSquareMeter,
    #[unit(0.001, "g/m²", "grams per square meter")]
    GramPerSquareMeter,
    #[unit(0.30515169363672057, "oz/ft²", "ounces per square foot")]
    OuncePerSquareFoot,
}

pub type LinearNumberDensity = StaticSIQuantity<{ ISQ { m: -1, ..ISQ::ZERO } }>;

#[derive(Copy, Clone, Unit)]
#[dim(LinearNumberDensity)]
pub enum LinearNumberDensityUnit {
    #[unit(0.001, "km⁻¹", "per kilometer")]
    PerKilometer,
    #[unit(1.0, "m⁻¹", "per meter")]
    PerMeter,
    #[unit(10.0, "dm⁻¹", "per decimeter")]
    PerDecimeter,
    #[unit(100.0, "cm⁻¹", "per centimeter")]
    PerCentimeter,
    #[unit(1000.0, "mm⁻¹", "per millimeter")]
    PerMillimeter,
    #[unit(3.280839895013123, "ft⁻¹", "per foot")]
    PerFoot,
    #[unit(3.280833436679587, "ft (U.S. survey)", "foot (U.S. survey)")]
    PerFootSurvey,
    #[unit(39.370078740157481, "in⁻¹", "per inch")]
    PerInch,
    #[unit(0.00062137119223733392, "mi⁻¹", "per mile")]
    PerMile,
    #[unit(0.00062137003393301752, "mi⁻¹ (U.S. survey)", "per mile (U.S. survey)")]
    PerMileSurvey,
    #[unit(0.00053995680345572358, "M⁻¹", "per nautical mile")]
    PerNauticalMile,
    #[unit(1.0936132983377078, "yd⁻¹", "per yard")]
    PerYard,
}

pub type ArealNumberDensity = StaticSIQuantity<{ ISQ { m: -2, ..ISQ::ZERO } }>;

#[derive(Copy, Clone, Unit)]
#[dim(ArealNumberDensity)]
pub enum ArealNumberDensityUnit {
    #[unit(1.0, "m⁻²", "per square meter")]
    PerSquareMeter,
    #[unit(0.00024710436922532533, "ac⁻²", "per acre")]
    PerAcre,
    #[unit(0.01, "a⁻²", "per are")]
    PerAre,
    #[unit(9.9999999999999996e+27, "b⁻²", "per barn")]
    PerBarn,
    #[unit(1973525159.9788833, "cmil⁻²", "per circular mil")]
    PerCircularMil,
    #[unit(0.0001, "ha⁻²", "per hectare")]
    PerHectare,
    #[unit(10.763910416709722, "ft⁻²", "per square foot")]
    PerSquareFoot,
    #[unit(1550.0031000062002, "in⁻²", "per square inch")]
    PerSquareInch,
    #[unit(3.8610215854244582e-07, "mi⁻²", "per square mile")]
    PerSquareMile,
    #[unit(1.1959900463010802, "yd⁻²", "per square yard")]
    PerSquareYard,
}

pub type VolumetricNumberDensity = StaticSIQuantity<{ ISQ { m: -3, ..ISQ::ZERO } }>;

#[derive(Copy, Clone, Unit)]
#[dim(VolumetricNumberDensity)]
pub enum VolumetricNumberDensityUnit {
    #[unit(1.0, "m⁻³", "per cubic meter")]
    PerCubicMeter,
    #[unit(35.314662471284763, "ft⁻³", "per cubic foot")]
    PerCubicFoot,
    #[unit(61023.758990325288, "in⁻³", "per cubic inch")]
    PerCubicInch,
    #[unit(2.39912748531614e-10, "mi⁻³", "per cubic mile")]
    PerCubicMile,
    #[unit(1.3079505474361619, "yd⁻³", "per cubic yard")]
    PerCubicYard,
    #[unit(33814.0222016107, "per fl oz", "per fluid ounce")]
    PerFluidOunce,
    #[unit(35195.082824588411, "per fl oz (UK)", "per Imperial fluid ounce")]
    PerFluidOunceImperial,
    #[unit(219.96924829908778, "per gal (UK)", "per Imperial gallon")]
    PerGallonImperial,
    #[unit(264.17203728418463, "per gal", "per gallon")]
    PerGallon,
    #[unit(1000.0, "L⁻¹", "per liter")]
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

#[derive(Copy, Clone, Unit)]
#[dim(LinearNumberRate)]
pub enum LinearNumberRateUnit {
    #[unit(0.001, "km⁻¹ · s⁻¹", "per kilometer second")]
    PerKilometerSecond,
    #[unit(1.0, "m⁻¹ · s⁻¹", "per meter second")]
    PerMeterSecond,
    #[unit(100.0, "cm⁻¹ · s⁻¹", "per centimeter second")]
    PerCentimeterSecond,
    #[unit(1000.0, "mm⁻¹ · s⁻¹", "per millimeter second")]
    PerMillimeterSecond,
    #[unit(3.280839895013123, "ft⁻¹ · s⁻¹", "per foot second")]
    PerFootSecond,
    #[unit(
        3.280833436679587,
        "ft⁻¹ (U.S. survey) · s⁻¹",
        "per foot (U.S. survey) second"
    )]
    PerFootSurveySecond,
    #[unit(39.370078740157481, "in⁻¹ · s⁻¹", "per inch second")]
    PerInchSecond,
    #[unit(0.00062137119223733392, "mi⁻¹ · s⁻¹", "per mile second")]
    PerMileSecond,
    #[unit(
        0.00062137003393301752,
        "mi⁻¹ (U.S. survey) · s⁻¹",
        "per mile (U.S. survey) second"
    )]
    PerMileSurveySecond,
    #[unit(1.0936132983377078, "yd⁻¹ · s⁻¹", "per yard second")]
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

#[derive(Copy, Clone, Unit)]
#[dim(ArealNumberRate)]
pub enum ArealNumberRateUnit {
    #[unit(1.0, "m⁻² · s⁻¹", "per square meter second")]
    PerSquareMeterSecond,
    #[unit(0.00024710436922532533, "ac⁻¹ · s⁻¹", "per acre second")]
    PerAcreSecond,
    #[unit(0.01, "a⁻¹ · s⁻¹", "per are second")]
    PerAreSecond,
    #[unit(9.9999999999999996e+27, "b⁻¹ · s⁻¹", "per barn second")]
    PerBarnSecond,
    #[unit(1973525159.9788833, "cmil⁻¹ · s⁻¹", "per circular mil second")]
    PerCircularMilSecond,
    #[unit(0.0001, "ha⁻¹ · s⁻¹", "per hectare second")]
    PerHectareSecond,
    #[unit(10.763910416709722, "ft⁻² · s⁻¹", "per square foot second")]
    PerSquareFootSecond,
    #[unit(1550.0031000062002, "in⁻² · s⁻¹", "per square inch second")]
    PerSquareInchSecond,
    #[unit(3.8610215854244582e-07, "mi⁻² · s⁻¹", "per square mile second")]
    PerSquareMileSecond,
    #[unit(1.1959900463010802, "yd⁻² · s⁻¹", "per square yard second")]
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

#[derive(Copy, Clone, Unit)]
#[dim(VolumetricNumberRate)]
pub enum VolumetricNumberRateUnit {
    #[unit(1.0, "m⁻³ · s⁻¹", "per cubic meter second")]
    PerCubicMeterSecond,
    #[unit(35.314662471284763, "ft⁻³ · s⁻¹", "per cubic foot second")]
    PerCubicFootSecond,
    #[unit(61023.758990325288, "in⁻³ · s⁻¹", "per cubic inch second")]
    PerCubicInchSecond,
    #[unit(2.39912748531614e-10, "mi⁻³ · s⁻¹", "per cubic mile second")]
    PerCubicMileSecond,
    #[unit(1.3079505474361619, "yd⁻³ · s⁻¹", "per cubic yard second")]
    PerCubicYardSecond,
    #[unit(33814.0222016107, "fl oz⁻¹ · s⁻¹", "per fluid ounce second")]
    PerFluidOunceSecond,
    #[unit(
        35195.082824588411,
        "fl oz⁻¹ (UK) · s⁻¹",
        "per Imperial fluid ounce second"
    )]
    PerFluidOunceImperialSecond,
    #[unit(219.96924829908778, "gal⁻¹ (UK) · s⁻¹", "per Imperial gallon second")]
    PerGallonImperialSecond,
    #[unit(264.17203728418463, "gal⁻¹ · s⁻¹", "per gallon second")]
    PerGallonSecond,
    #[unit(1000.0, "L⁻¹ · s⁻¹", "per liter second")]
    PerLiterSecond,
    #[unit(1.0, "Bq/m³", "becquerels per cubic meter")]
    BecquerelPerCubicMeter,
    #[unit(37000000000.0, "Ci/m³", "curies per cubic meter")]
    CuriePerCubicMeter,
    #[unit(
        0.016666666666666666,
        "dpm/m³",
        "disintegrations per minute per cubic meter"
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

#[derive(Copy, Clone, Unit)]
#[dim(LinearDensityOfStates)]
pub enum LinearDensityOfStatesUnit {
    #[unit(1.0, "1/(m · J)", "states per meter joule")]
    StatePerMeterJoule,
    #[unit(100.0, "1/(cm · J)", "states per centimeter joule")]
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

#[derive(Copy, Clone, Unit)]
#[dim(ArealDensityOfStates)]
pub enum ArealDensityOfStatesUnit {
    #[unit(1.0, "1/(m² · J)", "states per square meter joule")]
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

#[derive(Copy, Clone, Unit)]
#[dim(VolumetricDensityOfStates)]
pub enum VolumetricDensityOfStatesUnit {
    #[unit(1.0, "1/(m³ · J)", "states per cubic meter joule")]
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

#[derive(Copy, Clone, Unit)]
#[dim(LinearPowerDensity)]
pub enum LinearPowerDensityUnit {
    #[unit(1.0, "W/m", "watts per meter")]
    WattPerMeter,
    #[unit(100.0, "W/cm", "watts per centimeter")]
    WattPerCentimeter,
    #[unit(1000.0, "W/mm", "watts per millimeter")]
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

#[derive(Copy, Clone, Unit)]
#[dim(VolumetricPowerDensity)]
pub enum VolumetricPowerDensityUnit {
    #[unit(1.0, "W/m³", "watts per cubic meter")]
    WattPerCubicMeter,
}
