use crate::{ISQ, StaticSIQuantity};
use yadc::{Dimension, Unit};

pub type AmountOfSubstance = StaticSIQuantity<{ ISQ { mol: 1, ..ISQ::ZERO } }>;

#[derive(Unit, Copy, Clone)]
#[dim(AmountOfSubstance)]
pub enum AmountOfSubstanceUnit {
    #[linear("Ymol", "yottamoles", 9.9999999999999998e+23)]
    Yottamole,
    #[linear("Zmol", "zettamoles", 1e+21)]
    Zettamole,
    #[linear("Emol", "examoles", 1e+18)]
    Examole,
    #[linear("Pmol", "petamoles", 1000000000000000.0)]
    Petamole,
    #[linear("Tmol", "teramoles", 1000000000000.0)]
    Teramole,
    #[linear("Gmol", "gigamoles", 1000000000.0)]
    Gigamole,
    #[linear("Mmol", "megamoles", 1000000.0)]
    Megamole,
    #[linear("kmol", "kilomoles", 1000.0)]
    Kilomole,
    #[linear("hmol", "hectomoles", 100.0)]
    Hectomole,
    #[linear("damol", "decamoles", 10.0)]
    Decamole,
    #[linear("mol", "moles", 1.0)]
    Mole,
    #[linear("dmol", "decimoles", 0.10000000000000001)]
    Decimole,
    #[linear("cmol", "centimoles", 0.01)]
    Centimole,
    #[linear("mmol", "millimoles", 0.001)]
    Millimole,
    #[linear("µmol", "micromoles", 9.9999999999999995e-07)]
    Micromole,
    #[linear("nmol", "nanomoles", 1.0000000000000001e-09)]
    Nanomole,
    #[linear("pmol", "picomoles", 9.9999999999999998e-13)]
    Picomole,
    #[linear("fmol", "femtomoles", 1.0000000000000001e-15)]
    Femtomole,
    #[linear("amol", "attomoles", 1.0000000000000001e-18)]
    Attomole,
    #[linear("zmol", "zeptomoles", 9.9999999999999991e-22)]
    Zeptomole,
    #[linear("ymol", "yoctomoles", 9.9999999999999992e-25)]
    Yoctomole,
    #[linear("particle", "particles", 1.6605390671738466e-24)]
    Particle,
}

pub type MolarConcentration = StaticSIQuantity<
    {
        ISQ {
            m: -3,
            mol: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(MolarConcentration)]
pub enum MolarConcentrationUnit {
    #[linear("Ymol/m³", "yottamoles per cubic meter", 9.9999999999999998e+23)]
    YottamolePerCubicMeter,
    #[linear("Zmol/m³", "zettamoles per cubic meter", 1e+21)]
    ZettamolePerCubicMeter,
    #[linear("Emol/m³", "examoles per cubic meter", 1e+18)]
    ExamolePerCubicMeter,
    #[linear("Pmol/m³", "petamoles per cubic meter", 1000000000000000.0)]
    PetamolePerCubicMeter,
    #[linear("Tmol/m³", "teramoles per cubic meter", 1000000000000.0)]
    TeramolePerCubicMeter,
    #[linear("Gmol/m³", "gigamoles per cubic meter", 1000000000.0)]
    GigamolePerCubicMeter,
    #[linear("Mmol/m³", "megamoles per cubic meter", 1000000.0)]
    MegamolePerCubicMeter,
    #[linear("kmol/m³", "kilomoles per cubic meter", 1000.0)]
    KilomolePerCubicMeter,
    #[linear("hmol/m³", "hectomoles per cubic meter", 100.0)]
    HectomolePerCubicMeter,
    #[linear("damol/m³", "decamoles per cubic meter", 10.0)]
    DecamolePerCubicMeter,
    #[linear("mol/m³", "moles per cubic meter", 1.0)]
    MolePerCubicMeter,
    #[linear("dmol/m³", "decimoles per cubic meter", 0.10000000000000001)]
    DecimolePerCubicMeter,
    #[linear("cmol/m³", "centimoles per cubic meter", 0.01)]
    CentimolePerCubicMeter,
    #[linear("mmol/m³", "millimoles per cubic meter", 0.001)]
    MillimolePerCubicMeter,
    #[linear("µmol/m³", "micromoles per cubic meter", 9.9999999999999995e-07)]
    MicromolePerCubicMeter,
    #[linear("nmol/m³", "nanomoles per cubic meter", 1.0000000000000001e-09)]
    NanomolePerCubicMeter,
    #[linear("pmol/m³", "picomoles per cubic meter", 9.9999999999999998e-13)]
    PicomolePerCubicMeter,
    #[linear("fmol/m³", "femtomoles per cubic meter", 1.0000000000000001e-15)]
    FemtomolePerCubicMeter,
    #[linear("amol/m³", "attomoles per cubic meter", 1.0000000000000001e-18)]
    AttomolePerCubicMeter,
    #[linear("zmol/m³", "zeptomoles per cubic meter", 9.9999999999999991e-22)]
    ZeptomolePerCubicMeter,
    #[linear("ymol/m³", "yoctomoles per cubic meter", 9.9999999999999992e-25)]
    YoctomolePerCubicMeter,
    #[linear("kmol/L", "kilomoles per liter", 1000000.0)]
    KilomolePerLiter,
    #[linear("mol/L", "moles per liter", 1000.0)]
    MolePerLiter,
    #[linear("mmol/L", "millimoles per liter", 1.0)]
    MillimolePerLiter,
    #[linear("μmol/L", "micromoles per liter", 0.001)]
    MicromolePerLiter,
    #[linear("nmol/L", "nanomoles per liter", 9.9999999999999995e-07)]
    NanomolePerLiter,
    #[linear("pmol/L", "picomoles per liter", 1.0000000000000001e-09)]
    PicomolePerLiter,
    #[linear("fmol/L", "femtomoles per liter", 9.9999999999999998e-13)]
    FemtomolePerLiter,
    #[linear("particle/m³", "particles per cubic meter", 1.6605390671738466e-24)]
    ParticlePerCubicMeter,
}

pub type MolarEnergy = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -2,
            mol: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(MolarEnergy)]
pub enum MolarEnergyUnit {
    #[linear(
        "kg · m²/(s² · mol)",
        "kilograms square meter per second squared mole",
        1.0
    )]
    KilogramSquareMeterPerSecondSquaredMole,
    #[linear("YJ/mol", "yottajoules per mole", 9.9999999999999998e+23)]
    YottajoulePerMole,
    #[linear("ZJ/mol", "zettajoules per mole", 1e+21)]
    ZettajoulePerMole,
    #[linear("EJ/mol", "exajoules per mole", 1e+18)]
    ExajoulePerMole,
    #[linear("PJ/mol", "petajoules per mole", 1000000000000000.0)]
    PetajoulePerMole,
    #[linear("TJ/mol", "terajoules per mole", 1000000000000.0)]
    TerajoulePerMole,
    #[linear("GJ/mol", "gigajoules per mole", 1000000000.0)]
    GigajoulePerMole,
    #[linear("MJ/mol", "megajoules per mole", 1000000.0)]
    MegajoulePerMole,
    #[linear("kJ/mol", "kilojoules per mole", 1000.0)]
    KilojoulePerMole,
    #[linear("hJ/mol", "hectojoules per mole", 100.0)]
    HectojoulePerMole,
    #[linear("daJ/mol", "decajoules per mole", 10.0)]
    DecajoulePerMole,
    #[linear("J/mol", "joules per mole", 1.0)]
    JoulePerMole,
    #[linear("dJ/mol", "decijoules per mole", 0.10000000000000001)]
    DecijoulePerMole,
    #[linear("cJ/mol", "centijoules per mole", 0.01)]
    CentijoulePerMole,
    #[linear("mJ/mol", "millijoules per mole", 0.001)]
    MillijoulePerMole,
    #[linear("µJ/mol", "microjoules per mole", 9.9999999999999995e-07)]
    MicrojoulePerMole,
    #[linear("nJ/mol", "nanojoules per mole", 1.0000000000000001e-09)]
    NanojoulePerMole,
    #[linear("pJ/mol", "picojoules per mole", 9.9999999999999998e-13)]
    PicojoulePerMole,
    #[linear("fJ/mol", "femtojoules per mole", 1.0000000000000001e-15)]
    FemtojoulePerMole,
    #[linear("aJ/mol", "attojoules per mole", 1.0000000000000001e-18)]
    AttojoulePerMole,
    #[linear("zJ/mol", "zeptojoules per mole", 9.9999999999999991e-22)]
    ZeptojoulePerMole,
    #[linear("yJ/mol", "yoctojoules per mole", 9.9999999999999992e-25)]
    YoctojoulePerMole,
    #[linear("PW · h/mol", "petawatt hours per mole", 3.6e+18)]
    PetawattHourPerMole,
    #[linear("TW · h/mol", "terawatt hours per mole", 3600000000000000.0)]
    TerawattHourPerMole,
    #[linear("GW · h/mol", "gigawatt hours per mole", 3600000000000.0)]
    GigawattHourPerMole,
    #[linear("MW · h/mol", "megawatt hours per mole", 3600000000.0)]
    MegawattHourPerMole,
    #[linear("kW · h/mol", "kilowatt hours per mole", 3600000.0)]
    KilowattHourPerMole,
    #[linear("hW · h/mol", "hectowatt hours per mole", 360000.0)]
    HectowattHourPerMole,
    #[linear("daW · h/mol", "decawatt hours per mole", 36000.0)]
    DecawattHourPerMole,
    #[linear("W · h/mol", "watt hours per mole", 3600.0)]
    WattHourPerMole,
    #[linear("mW · h/mol", "milliwatt hours per mole", 3.6000000000000001)]
    MilliwattHourPerMole,
    #[linear("µW · h/mol", "microwatt hours per mole", 0.0035999999999999999)]
    MicrowattHourPerMole,
    #[linear("Btu (IT)/mol", "British thermal units (IT) per mole", 1055.056)]
    BtuItPerMole,
    #[linear("Btu/mol", "British thermal units per mole", 1054.3499999999999)]
    BtuPerMole,
    #[linear("cal (IT)/mol", "calories (IT) per mole", 4.1867999999999999)]
    CalorieItPerMole,
    #[linear("cal/mol", "calories per mole", 4.1840000000000002)]
    CaloriePerMole,
    #[linear("Cal (IT)/mol", "Calories (IT) per mole", 4186.8000000000002)]
    CalorieItNutritionPerMole,
    #[linear("Cal/mol", "Calories per mole", 4184.0)]
    CalorieNutritionPerMole,
    #[linear("eV/mol", "electronvolts per mole", 1.6021766339999999e-19)]
    ElectronvoltPerMole,
    #[linear("erg/mol", "ergs per mole", 9.9999999999999995e-08)]
    ErgPerMole,
    #[linear("ft · pdl/mol", "foot poundals per mole", 0.042140110000000001)]
    FootPoundalPerMole,
    #[linear("ft · lbf/mol", "foot pounds-force per mole", 1.355818)]
    FootPoundForcePerMole,
    #[linear("kcal (IT)/mol", "kilocalories (IT) per mole", 4186.8000000000002)]
    KilocalorieItPerMole,
    #[linear("kcal/mol", "kilocalories per mole", 4184.0)]
    KilocaloriePerMole,
    #[linear("10¹⁵ Btu (IT)/mol", "quads per mole", 1.055056e+18)]
    QuadPerMole,
    #[linear("thm (EC)/mol", "therms (EC) per mole", 105506000.0)]
    ThermEcPerMole,
    #[linear("thm/mol", "therms per mole", 105480400.0)]
    ThermUsPerMole,
    #[linear("t of TNT/mol", "tons of TNT per mole", 4184000000.0)]
    TonTntPerMole,
    #[linear("W · s/mol", "watt seconds per mole", 1.0)]
    WattSecondPerMole,
    #[linear("J/particle", "joules per particle", 6.0221407599999999e+23)]
    JoulePerParticle,
    #[linear("eV/particle", "electronvolts per particle", 96485.332123310014)]
    ElectronvoltPerParticle,
}

pub type MolarFlux = StaticSIQuantity<
    {
        ISQ {
            m: -2,
            s: -1,
            mol: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(MolarFlux)]
pub enum MolarFluxUnit {
    #[linear("mol/(m² · s)", "moles per square meter second", 1.0)]
    MolePerSquareMeterSecond,
}

pub type MolarHeatCapacity = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -2,
            K: -1,
            mol: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(MolarHeatCapacity)]
pub enum MolarHeatCapacityUnit {
    #[linear(
        "YJ/(K · mol)",
        "yottajoules per kelvin mole",
        9.9999999999999998e+23
    )]
    YottajoulePerKelvinMole,
    #[linear("ZJ/(K · mol)", "zettajoules per kelvin mole", 1e+21)]
    ZettajoulePerKelvinMole,
    #[linear("EJ/(K · mol)", "exajoules per kelvin mole", 1e+18)]
    ExajoulePerKelvinMole,
    #[linear("PJ/(K · mol)", "petajoules per kelvin mole", 1000000000000000.0)]
    PetajoulePerKelvinMole,
    #[linear("TJ/(K · mol)", "terajoules per kelvin mole", 1000000000000.0)]
    TerajoulePerKelvinMole,
    #[linear("GJ/(K · mol)", "gigajoules per kelvin mole", 1000000000.0)]
    GigajoulePerKelvinMole,
    #[linear("MJ/(K · mol)", "megajoules per kelvin mole", 1000000.0)]
    MegajoulePerKelvinMole,
    #[linear("kJ/(K · mol)", "kilojoules per kelvin mole", 1000.0)]
    KilojoulePerKelvinMole,
    #[linear("hJ/(K · mol)", "hectojoules per kelvin mole", 100.0)]
    HectojoulePerKelvinMole,
    #[linear("daJ/(K · mol)", "decajoules per kelvin mole", 10.0)]
    DecajoulePerKelvinMole,
    #[linear("J/(K · mol)", "joules per kelvin mole", 1.0)]
    JoulePerKelvinMole,
    #[linear("dJ/(K · mol)", "decijoules per kelvin mole", 0.10000000000000001)]
    DecijoulePerKelvinMole,
    #[linear("cJ/(K · mol)", "centijoules per kelvin mole", 0.01)]
    CentijoulePerKelvinMole,
    #[linear("mJ/(K · mol)", "millijoules per kelvin mole", 0.001)]
    MillijoulePerKelvinMole,
    #[linear(
        "µJ/(K · mol)",
        "microjoules per kelvin mole",
        9.9999999999999995e-07
    )]
    MicrojoulePerKelvinMole,
    #[linear("nJ/(K · mol)", "nanojoules per kelvin mole", 1.0000000000000001e-09)]
    NanojoulePerKelvinMole,
    #[linear("pJ/(K · mol)", "picojoules per kelvin mole", 9.9999999999999998e-13)]
    PicojoulePerKelvinMole,
    #[linear(
        "fJ/(K · mol)",
        "femtojoules per kelvin mole",
        1.0000000000000001e-15
    )]
    FemtojoulePerKelvinMole,
    #[linear("aJ/(K · mol)", "attojoules per kelvin mole", 1.0000000000000001e-18)]
    AttojoulePerKelvinMole,
    #[linear(
        "zJ/(K · mol)",
        "zeptojoules per kelvin mole",
        9.9999999999999991e-22
    )]
    ZeptojoulePerKelvinMole,
    #[linear(
        "yJ/(K · mol)",
        "yoctojoules per kelvin mole",
        9.9999999999999992e-25
    )]
    YoctojoulePerKelvinMole,
    #[linear(
        "Btu (IT)/(K · mol)",
        "British thermal units (IT) per kelvin mole",
        1055.056
    )]
    BtuItPerKelvinMole,
    #[linear(
        "Btu/(K · mol)",
        "British thermal units per kelvin mole",
        1054.3499999999999
    )]
    BtuPerKelvinMole,
    #[linear(
        "cal (IT)/(K · mol)",
        "calories (IT) per kelvin mole",
        4.1867999999999999
    )]
    CalorieItPerKelvinMole,
    #[linear("cal/(K · mol)", "calories per kelvin mole", 4.1840000000000002)]
    CaloriePerKelvinMole,
    #[linear(
        "Cal (IT)/(K · mol)",
        "Calories (IT) per kelvin mole",
        4186.8000000000002
    )]
    CalorieItNutritionPerKelvinMole,
    #[linear("Cal/(K · mol)", "Calories per kelvin mole", 4184.0)]
    CalorieNutritionPerKelvinMole,
    #[linear(
        "kcal (IT)/(K · mol)",
        "kilocalories (IT) per kelvin mole",
        4186.8000000000002
    )]
    KilocalorieItPerKelvinMole,
    #[linear("kcal/(K · mol)", "kilocalories per kelvin mole", 4184.0)]
    KilocaloriePerKelvinMole,
    #[linear(
        "J/(K · particle)",
        "joules per kelvin particle",
        6.0221407599999999e+23
    )]
    JoulePerKelvinParticle,
    #[linear(
        "eV/(K · particle)",
        "electronvolts per kelvin particle",
        96485.332123310014
    )]
    ElectronvoltPerKelvinParticle,
    #[linear("R", "molar gas constants", 8.3144626180000003)]
    MolarGasConstant,
}

pub type MolarMass = StaticSIQuantity<
    {
        ISQ {
            kg: 1,
            mol: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(MolarMass)]
pub enum MolarMassUnit {
    #[linear("Yg/mol", "yottagrams per mole", 1e+21)]
    YottagramPerMole,
    #[linear("Zg/mol", "zettagrams per mole", 1e+18)]
    ZettagramPerMole,
    #[linear("Eg/mol", "exagrams per mole", 1000000000000000.0)]
    ExagramPerMole,
    #[linear("Pg/mol", "petagrams per mole", 1000000000000.0)]
    PetagramPerMole,
    #[linear("Tg/mol", "teragrams per mole", 1000000000.0)]
    TeragramPerMole,
    #[linear("Gg/mol", "gigagrams per mole", 1000000.0)]
    GigagramPerMole,
    #[linear("Mg/mol", "megagrams per mole", 1000.0)]
    MegagramPerMole,
    #[linear("kg/mol", "kilograms per mole", 1.0)]
    KilogramPerMole,
    #[linear("hg/mol", "hectograms per mole", 0.10000000000000001)]
    HectogramPerMole,
    #[linear("dag/mol", "decagrams per mole", 0.01)]
    DecagramPerMole,
    #[linear("g/mol", "grams per mole", 0.001)]
    GramPerMole,
    #[linear("dg/mol", "decigrams per mole", 0.0001)]
    DecigramPerMole,
    #[linear("cg/mol", "centigrams per mole", 1.0000000000000001e-05)]
    CentigramPerMole,
    #[linear("mg/mol", "milligrams per mole", 9.9999999999999995e-07)]
    MilligramPerMole,
    #[linear("µg/mol", "micrograms per mole", 9.9999999999999986e-10)]
    MicrogramPerMole,
    #[linear("ng/mol", "nanograms per mole", 9.9999999999999998e-13)]
    NanogramPerMole,
    #[linear("pg/mol", "picograms per mole", 1.0000000000000001e-15)]
    PicogramPerMole,
    #[linear("fg/mol", "femtograms per mole", 1.0000000000000001e-18)]
    FemtogramPerMole,
    #[linear("ag/mol", "attograms per mole", 1.0000000000000001e-21)]
    AttogramPerMole,
    #[linear("zg/mol", "zeptograms per mole", 9.9999999999999992e-25)]
    ZeptogramPerMole,
    #[linear("yg/mol", "yoctograms per mole", 9.9999999999999986e-28)]
    YoctogramPerMole,
}

pub type MolarRadioactivity = StaticSIQuantity<
    {
        ISQ {
            s: -1,
            mol: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(MolarRadioactivity)]
pub enum MolarRadioactivityUnit {
    #[linear("Bq/mol", "becquerels per mole", 1.0)]
    BecquerelPerMole,
    #[linear("Ci/mol", "curies per mole", 37000000000.0)]
    CuriePerMole,
    #[linear(
        "dpm/mol",
        "disintegrations per minute per mole",
        0.016666666666666666
    )]
    DisintegrationsPerMinutePerMole,
}

pub type MolarVolume = StaticSIQuantity<
    {
        ISQ {
            m: 3,
            mol: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(MolarVolume)]
pub enum MolarVolumeUnit {
    #[linear("m³/mol", "cubic meters per mole", 1.0)]
    CubicMeterPerMole,
    #[linear("m³/particle", "cubic meters per particle", 6.0221407599999999e+23)]
    CubicMeterPerParticle,
}

pub type Molality = StaticSIQuantity<
    {
        ISQ {
            kg: -1,
            mol: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(Molality)]
pub enum MolalityUnit {
    #[linear("mol/kg", "moles per kilogram", 1.0)]
    MolePerKilogram,
}

pub type MassConcentration = StaticSIQuantity<
    {
        ISQ {
            m: -3,
            kg: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(MassConcentration)]
pub enum MassConcentrationUnit {
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
}

pub type CatalyticActivity = StaticSIQuantity<
    {
        ISQ {
            s: -1,
            mol: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(CatalyticActivity)]
pub enum CatalyticActivityUnit {
    #[linear("Ykat", "yottakatals", 9.9999999999999998e+23)]
    Yottakatal,
    #[linear("Zkat", "zettakatals", 1e+21)]
    Zettakatal,
    #[linear("Ekat", "exakatals", 1e+18)]
    Exakatal,
    #[linear("Pkat", "petakatals", 1000000000000000.0)]
    Petakatal,
    #[linear("Tkat", "terakatals", 1000000000000.0)]
    Terakatal,
    #[linear("Gkat", "gigakatals", 1000000000.0)]
    Gigakatal,
    #[linear("Mkat", "megakatals", 1000000.0)]
    Megakatal,
    #[linear("kkat", "kilokatals", 1000.0)]
    Kilokatal,
    #[linear("hkat", "hectokatals", 100.0)]
    Hectokatal,
    #[linear("dakat", "decakatals", 10.0)]
    Decakatal,
    #[linear("kat", "katals", 1.0)]
    Katal,
    #[linear("dkat", "decikatals", 0.10000000000000001)]
    Decikatal,
    #[linear("ckat", "centikatals", 0.01)]
    Centikatal,
    #[linear("mkat", "millikatals", 0.001)]
    Millikatal,
    #[linear("µkat", "microkatals", 9.9999999999999995e-07)]
    Microkatal,
    #[linear("nkat", "nanokatals", 1.0000000000000001e-09)]
    Nanokatal,
    #[linear("pkat", "picokatals", 9.9999999999999998e-13)]
    Picokatal,
    #[linear("fkat", "femtokatals", 1.0000000000000001e-15)]
    Femtokatal,
    #[linear("akat", "attokatals", 1.0000000000000001e-18)]
    Attokatal,
    #[linear("zkat", "zeptokatals", 9.9999999999999991e-22)]
    Zeptokatal,
    #[linear("ykat", "yoctokatals", 9.9999999999999992e-25)]
    Yoctokatal,
    #[linear("particle/s", "particles per second", 1.6605390671738466e-24)]
    ParticlePerSecond,
    #[linear("mol/s", "moles per second", 1.0)]
    MolePerSecond,
}

pub type CatalyticActivityConcentration = StaticSIQuantity<
    {
        ISQ {
            m: -3,
            s: -1,
            mol: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(CatalyticActivityConcentration)]
pub enum CatalyticActivityConcentrationUnit {
    #[linear("Ykat/m³", "yottakatals per cubic meter", 9.9999999999999998e+23)]
    YottakatalPerCubicMeter,
    #[linear("Zkat/m³", "zettakatals per cubic meter", 1e+21)]
    ZettakatalPerCubicMeter,
    #[linear("Ekat/m³", "exakatals per cubic meter", 1e+18)]
    ExakatalPerCubicMeter,
    #[linear("Pkat/m³", "petakatals per cubic meter", 1000000000000000.0)]
    PetakatalPerCubicMeter,
    #[linear("Tkat/m³", "terakatals per cubic meter", 1000000000000.0)]
    TerakatalPerCubicMeter,
    #[linear("Gkat/m³", "gigakatals per cubic meter", 1000000000.0)]
    GigakatalPerCubicMeter,
    #[linear("Mkat/m³", "megakatals per cubic meter", 1000000.0)]
    MegakatalPerCubicMeter,
    #[linear("kkat/m³", "kilokatals per cubic meter", 1000.0)]
    KilokatalPerCubicMeter,
    #[linear("hkat/m³", "hectokatals per cubic meter", 100.0)]
    HectokatalPerCubicMeter,
    #[linear("dakat/m³", "decakatals per cubic meter", 10.0)]
    DecakatalPerCubicMeter,
    #[linear("kat/m³", "katals per cubic meter", 1.0)]
    KatalPerCubicMeter,
    #[linear("dkat/m³", "decikatals per cubic meter", 0.10000000000000001)]
    DecikatalPerCubicMeter,
    #[linear("ckat/m³", "centikatals per cubic meter", 0.01)]
    CentikatalPerCubicMeter,
    #[linear("mkat/m³", "millikatals per cubic meter", 0.001)]
    MillikatalPerCubicMeter,
    #[linear("µkat/m³", "microkatals per cubic meter", 9.9999999999999995e-07)]
    MicrokatalPerCubicMeter,
    #[linear("nkat/m³", "nanokatals per cubic meter", 1.0000000000000001e-09)]
    NanokatalPerCubicMeter,
    #[linear("pkat/m³", "picokatals per cubic meter", 9.9999999999999998e-13)]
    PicokatalPerCubicMeter,
    #[linear("fkat/m³", "femtokatals per cubic meter", 1.0000000000000001e-15)]
    FemtokatalPerCubicMeter,
    #[linear("akat/m³", "attokatals per cubic meter", 1.0000000000000001e-18)]
    AttokatalPerCubicMeter,
    #[linear("zkat/m³", "zeptokatals per cubic meter", 9.9999999999999991e-22)]
    ZeptokatalPerCubicMeter,
    #[linear("ykat/m³", "yoctokatals per cubic meter", 9.9999999999999992e-25)]
    YoctokatalPerCubicMeter,
    #[linear("kkat/L", "kilokatals per liter", 1000000.0)]
    KilokatalPerLiter,
    #[linear("kat/L", "katals per liter", 1000.0)]
    KatalPerLiter,
    #[linear("mkat/L", "millikatals per liter", 1.0)]
    MillikatalPerLiter,
    #[linear("μkat/L", "microkatals per liter", 0.001)]
    MicrokatalPerLiter,
    #[linear("nkat/L", "nanokatals per liter", 9.9999999999999995e-07)]
    NanokatalPerLiter,
    #[linear("pkat/L", "picokatals per liter", 1.0000000000000001e-09)]
    PicokatalPerLiter,
    #[linear("fkat/L", "femtokatals per liter", 9.9999999999999998e-13)]
    FemtokatalPerLiter,
    #[linear(
        "particle · s⁻¹ · m⁻³",
        "particles per second cubic meter",
        1.6605390671738466e-24
    )]
    ParticlePerSecondCubicMeter,
}

pub type Radioactivity = StaticSIQuantity<{ ISQ { s: -1, ..ISQ::ZERO } }>;

#[derive(Unit, Copy, Clone)]
#[dim(Radioactivity)]
pub enum RadioactivityUnit {
    #[linear("YBq", "yottabecquerels", 9.9999999999999998e+23)]
    Yottabecquerel,
    #[linear("ZBq", "zettabecquerels", 1e+21)]
    Zettabecquerel,
    #[linear("EBq", "exabecquerels", 1e+18)]
    Exabecquerel,
    #[linear("PBq", "petabecquerels", 1000000000000000.0)]
    Petabecquerel,
    #[linear("TBq", "terabecquerels", 1000000000000.0)]
    Terabecquerel,
    #[linear("GBq", "gigabecquerels", 1000000000.0)]
    Gigabecquerel,
    #[linear("MBq", "megabecquerels", 1000000.0)]
    Megabecquerel,
    #[linear("kBq", "kilobecquerels", 1000.0)]
    Kilobecquerel,
    #[linear("hBq", "hectobecquerels", 100.0)]
    Hectobecquerel,
    #[linear("daBq", "decabecquerels", 10.0)]
    Decabecquerel,
    #[linear("Bq", "becquerels", 1.0)]
    Becquerel,
    #[linear("mBq", "millibecquerels", 0.001)]
    Millibecquerel,
    #[linear("µBq", "microbecquerels", 9.9999999999999995e-07)]
    Microbecquerel,
    #[linear("nBq", "nanobecquerels", 1.0000000000000001e-09)]
    Nanobecquerel,
    #[linear("GCi", "gigacuries", 3.7e+19)]
    Gigacurie,
    #[linear("MCi", "megacuries", 37000000000000000.0)]
    Megacurie,
    #[linear("kCi", "kilocuries", 37000000000000.0)]
    Kilocurie,
    #[linear("Ci", "curies", 37000000000.0)]
    Curie,
    #[linear("mCi", "millicuries", 37000000.0)]
    Millicurie,
    #[linear("µCi", "microcuries", 37000.0)]
    Microcurie,
    #[linear("nCi", "nanocuries", 37.0)]
    Nanocurie,
    #[linear("dpm", "disintegrations per minute", 0.016666666666666666)]
    DisintegrationsPerMinute,
}

pub type SpecificRadioactivity = StaticSIQuantity<
    {
        ISQ {
            kg: -1,
            s: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Unit, Copy, Clone)]
#[dim(SpecificRadioactivity)]
pub enum SpecificRadioactivityUnit {
    #[linear("Bq/kg", "becquerels per kilogram", 1.0)]
    BecquerelPerKilogram,
    #[linear("Ci/kg", "curie per kilogram", 37000000000.0)]
    CuriePerKilogram,
    #[linear(
        "dpm/kg",
        "disintegrations per minute per kilogram",
        0.016666666666666666
    )]
    DisintegrationsPerMinutePerKilogram,
}
