use crate::{ISQ, StaticSIQuantity};
use yadc::{Dimension, Unit};

pub type AmountOfSubstance = StaticSIQuantity<
    {
        ISQ {
            mol: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(AmountOfSubstance)]
pub enum AmountOfSubstanceUnit {
    #[unit(9.9999999999999998e+23, "Ymol", "yottamoles")]
    Yottamole,
    #[unit(1e+21, "Zmol", "zettamoles")]
    Zettamole,
    #[unit(1e+18, "Emol", "examoles")]
    Examole,
    #[unit(1000000000000000.0, "Pmol", "petamoles")]
    Petamole,
    #[unit(1000000000000.0, "Tmol", "teramoles")]
    Teramole,
    #[unit(1000000000.0, "Gmol", "gigamoles")]
    Gigamole,
    #[unit(1000000.0, "Mmol", "megamoles")]
    Megamole,
    #[unit(1000.0, "kmol", "kilomoles")]
    Kilomole,
    #[unit(100.0, "hmol", "hectomoles")]
    Hectomole,
    #[unit(10.0, "damol", "decamoles")]
    Decamole,
    #[unit(1.0, "mol", "moles")]
    Mole,
    #[unit(0.10000000000000001, "dmol", "decimoles")]
    Decimole,
    #[unit(0.01, "cmol", "centimoles")]
    Centimole,
    #[unit(0.001, "mmol", "millimoles")]
    Millimole,
    #[unit(9.9999999999999995e-07, "µmol", "micromoles")]
    Micromole,
    #[unit(1.0000000000000001e-09, "nmol", "nanomoles")]
    Nanomole,
    #[unit(9.9999999999999998e-13, "pmol", "picomoles")]
    Picomole,
    #[unit(1.0000000000000001e-15, "fmol", "femtomoles")]
    Femtomole,
    #[unit(1.0000000000000001e-18, "amol", "attomoles")]
    Attomole,
    #[unit(9.9999999999999991e-22, "zmol", "zeptomoles")]
    Zeptomole,
    #[unit(9.9999999999999992e-25, "ymol", "yoctomoles")]
    Yoctomole,
    #[unit(1.6605390671738466e-24, "particle", "particles")]
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

#[derive(Copy, Clone, Unit)]
#[dim(MolarConcentration)]
pub enum MolarConcentrationUnit {
    #[unit(9.9999999999999998e+23, "Ymol/m³", "yottamoles per cubic meter")]
    YottamolePerCubicMeter,
    #[unit(1e+21, "Zmol/m³", "zettamoles per cubic meter")]
    ZettamolePerCubicMeter,
    #[unit(1e+18, "Emol/m³", "examoles per cubic meter")]
    ExamolePerCubicMeter,
    #[unit(1000000000000000.0, "Pmol/m³", "petamoles per cubic meter")]
    PetamolePerCubicMeter,
    #[unit(1000000000000.0, "Tmol/m³", "teramoles per cubic meter")]
    TeramolePerCubicMeter,
    #[unit(1000000000.0, "Gmol/m³", "gigamoles per cubic meter")]
    GigamolePerCubicMeter,
    #[unit(1000000.0, "Mmol/m³", "megamoles per cubic meter")]
    MegamolePerCubicMeter,
    #[unit(1000.0, "kmol/m³", "kilomoles per cubic meter")]
    KilomolePerCubicMeter,
    #[unit(100.0, "hmol/m³", "hectomoles per cubic meter")]
    HectomolePerCubicMeter,
    #[unit(10.0, "damol/m³", "decamoles per cubic meter")]
    DecamolePerCubicMeter,
    #[unit(1.0, "mol/m³", "moles per cubic meter")]
    MolePerCubicMeter,
    #[unit(0.10000000000000001, "dmol/m³", "decimoles per cubic meter")]
    DecimolePerCubicMeter,
    #[unit(0.01, "cmol/m³", "centimoles per cubic meter")]
    CentimolePerCubicMeter,
    #[unit(0.001, "mmol/m³", "millimoles per cubic meter")]
    MillimolePerCubicMeter,
    #[unit(9.9999999999999995e-07, "µmol/m³", "micromoles per cubic meter")]
    MicromolePerCubicMeter,
    #[unit(1.0000000000000001e-09, "nmol/m³", "nanomoles per cubic meter")]
    NanomolePerCubicMeter,
    #[unit(9.9999999999999998e-13, "pmol/m³", "picomoles per cubic meter")]
    PicomolePerCubicMeter,
    #[unit(1.0000000000000001e-15, "fmol/m³", "femtomoles per cubic meter")]
    FemtomolePerCubicMeter,
    #[unit(1.0000000000000001e-18, "amol/m³", "attomoles per cubic meter")]
    AttomolePerCubicMeter,
    #[unit(9.9999999999999991e-22, "zmol/m³", "zeptomoles per cubic meter")]
    ZeptomolePerCubicMeter,
    #[unit(9.9999999999999992e-25, "ymol/m³", "yoctomoles per cubic meter")]
    YoctomolePerCubicMeter,
    #[unit(1000000.0, "kmol/L", "kilomoles per liter")]
    KilomolePerLiter,
    #[unit(1000.0, "mol/L", "moles per liter")]
    MolePerLiter,
    #[unit(1.0, "mmol/L", "millimoles per liter")]
    MillimolePerLiter,
    #[unit(0.001, "μmol/L", "micromoles per liter")]
    MicromolePerLiter,
    #[unit(9.9999999999999995e-07, "nmol/L", "nanomoles per liter")]
    NanomolePerLiter,
    #[unit(1.0000000000000001e-09, "pmol/L", "picomoles per liter")]
    PicomolePerLiter,
    #[unit(9.9999999999999998e-13, "fmol/L", "femtomoles per liter")]
    FemtomolePerLiter,
    #[unit(1.6605390671738466e-24, "particle/m³", "particles per cubic meter")]
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

#[derive(Copy, Clone, Unit)]
#[dim(MolarEnergy)]
pub enum MolarEnergyUnit {
    #[unit(
        1.0,
        "kg · m²/(s² · mol)",
        "kilograms square meter per second squared mole"
    )]
    KilogramSquareMeterPerSecondSquaredMole,
    #[unit(9.9999999999999998e+23, "YJ/mol", "yottajoules per mole")]
    YottajoulePerMole,
    #[unit(1e+21, "ZJ/mol", "zettajoules per mole")]
    ZettajoulePerMole,
    #[unit(1e+18, "EJ/mol", "exajoules per mole")]
    ExajoulePerMole,
    #[unit(1000000000000000.0, "PJ/mol", "petajoules per mole")]
    PetajoulePerMole,
    #[unit(1000000000000.0, "TJ/mol", "terajoules per mole")]
    TerajoulePerMole,
    #[unit(1000000000.0, "GJ/mol", "gigajoules per mole")]
    GigajoulePerMole,
    #[unit(1000000.0, "MJ/mol", "megajoules per mole")]
    MegajoulePerMole,
    #[unit(1000.0, "kJ/mol", "kilojoules per mole")]
    KilojoulePerMole,
    #[unit(100.0, "hJ/mol", "hectojoules per mole")]
    HectojoulePerMole,
    #[unit(10.0, "daJ/mol", "decajoules per mole")]
    DecajoulePerMole,
    #[unit(1.0, "J/mol", "joules per mole")]
    JoulePerMole,
    #[unit(0.10000000000000001, "dJ/mol", "decijoules per mole")]
    DecijoulePerMole,
    #[unit(0.01, "cJ/mol", "centijoules per mole")]
    CentijoulePerMole,
    #[unit(0.001, "mJ/mol", "millijoules per mole")]
    MillijoulePerMole,
    #[unit(9.9999999999999995e-07, "µJ/mol", "microjoules per mole")]
    MicrojoulePerMole,
    #[unit(1.0000000000000001e-09, "nJ/mol", "nanojoules per mole")]
    NanojoulePerMole,
    #[unit(9.9999999999999998e-13, "pJ/mol", "picojoules per mole")]
    PicojoulePerMole,
    #[unit(1.0000000000000001e-15, "fJ/mol", "femtojoules per mole")]
    FemtojoulePerMole,
    #[unit(1.0000000000000001e-18, "aJ/mol", "attojoules per mole")]
    AttojoulePerMole,
    #[unit(9.9999999999999991e-22, "zJ/mol", "zeptojoules per mole")]
    ZeptojoulePerMole,
    #[unit(9.9999999999999992e-25, "yJ/mol", "yoctojoules per mole")]
    YoctojoulePerMole,
    #[unit(3.6e+18, "PW · h/mol", "petawatt hours per mole")]
    PetawattHourPerMole,
    #[unit(3600000000000000.0, "TW · h/mol", "terawatt hours per mole")]
    TerawattHourPerMole,
    #[unit(3600000000000.0, "GW · h/mol", "gigawatt hours per mole")]
    GigawattHourPerMole,
    #[unit(3600000000.0, "MW · h/mol", "megawatt hours per mole")]
    MegawattHourPerMole,
    #[unit(3600000.0, "kW · h/mol", "kilowatt hours per mole")]
    KilowattHourPerMole,
    #[unit(360000.0, "hW · h/mol", "hectowatt hours per mole")]
    HectowattHourPerMole,
    #[unit(36000.0, "daW · h/mol", "decawatt hours per mole")]
    DecawattHourPerMole,
    #[unit(3600.0, "W · h/mol", "watt hours per mole")]
    WattHourPerMole,
    #[unit(3.6000000000000001, "mW · h/mol", "milliwatt hours per mole")]
    MilliwattHourPerMole,
    #[unit(0.0035999999999999999, "µW · h/mol", "microwatt hours per mole")]
    MicrowattHourPerMole,
    #[unit(1055.056, "Btu (IT)/mol", "British thermal units (IT) per mole")]
    BtuItPerMole,
    #[unit(1054.3499999999999, "Btu/mol", "British thermal units per mole")]
    BtuPerMole,
    #[unit(4.1867999999999999, "cal (IT)/mol", "calories (IT) per mole")]
    CalorieItPerMole,
    #[unit(4.1840000000000002, "cal/mol", "calories per mole")]
    CaloriePerMole,
    #[unit(4186.8000000000002, "Cal (IT)/mol", "Calories (IT) per mole")]
    CalorieItNutritionPerMole,
    #[unit(4184.0, "Cal/mol", "Calories per mole")]
    CalorieNutritionPerMole,
    #[unit(1.6021766339999999e-19, "eV/mol", "electronvolts per mole")]
    ElectronvoltPerMole,
    #[unit(9.9999999999999995e-08, "erg/mol", "ergs per mole")]
    ErgPerMole,
    #[unit(0.042140110000000001, "ft · pdl/mol", "foot poundals per mole")]
    FootPoundalPerMole,
    #[unit(1.355818, "ft · lbf/mol", "foot pounds-force per mole")]
    FootPoundForcePerMole,
    #[unit(4186.8000000000002, "kcal (IT)/mol", "kilocalories (IT) per mole")]
    KilocalorieItPerMole,
    #[unit(4184.0, "kcal/mol", "kilocalories per mole")]
    KilocaloriePerMole,
    #[unit(1.055056e+18, "10¹⁵ Btu (IT)/mol", "quads per mole")]
    QuadPerMole,
    #[unit(105506000.0, "thm (EC)/mol", "therms (EC) per mole")]
    ThermEcPerMole,
    #[unit(105480400.0, "thm/mol", "therms per mole")]
    ThermUsPerMole,
    #[unit(4184000000.0, "t of TNT/mol", "tons of TNT per mole")]
    TonTntPerMole,
    #[unit(1.0, "W · s/mol", "watt seconds per mole")]
    WattSecondPerMole,
    #[unit(6.0221407599999999e+23, "J/particle", "joules per particle")]
    JoulePerParticle,
    #[unit(96485.332123310014, "eV/particle", "electronvolts per particle")]
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

#[derive(Copy, Clone, Unit)]
#[dim(MolarFlux)]
pub enum MolarFluxUnit {
    #[unit(1.0, "mol/(m² · s)", "moles per square meter second")]
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

#[derive(Copy, Clone, Unit)]
#[dim(MolarHeatCapacity)]
pub enum MolarHeatCapacityUnit {
    #[unit(9.9999999999999998e+23, "YJ/(K · mol)", "yottajoules per kelvin mole")]
    YottajoulePerKelvinMole,
    #[unit(1e+21, "ZJ/(K · mol)", "zettajoules per kelvin mole")]
    ZettajoulePerKelvinMole,
    #[unit(1e+18, "EJ/(K · mol)", "exajoules per kelvin mole")]
    ExajoulePerKelvinMole,
    #[unit(1000000000000000.0, "PJ/(K · mol)", "petajoules per kelvin mole")]
    PetajoulePerKelvinMole,
    #[unit(1000000000000.0, "TJ/(K · mol)", "terajoules per kelvin mole")]
    TerajoulePerKelvinMole,
    #[unit(1000000000.0, "GJ/(K · mol)", "gigajoules per kelvin mole")]
    GigajoulePerKelvinMole,
    #[unit(1000000.0, "MJ/(K · mol)", "megajoules per kelvin mole")]
    MegajoulePerKelvinMole,
    #[unit(1000.0, "kJ/(K · mol)", "kilojoules per kelvin mole")]
    KilojoulePerKelvinMole,
    #[unit(100.0, "hJ/(K · mol)", "hectojoules per kelvin mole")]
    HectojoulePerKelvinMole,
    #[unit(10.0, "daJ/(K · mol)", "decajoules per kelvin mole")]
    DecajoulePerKelvinMole,
    #[unit(1.0, "J/(K · mol)", "joules per kelvin mole")]
    JoulePerKelvinMole,
    #[unit(0.10000000000000001, "dJ/(K · mol)", "decijoules per kelvin mole")]
    DecijoulePerKelvinMole,
    #[unit(0.01, "cJ/(K · mol)", "centijoules per kelvin mole")]
    CentijoulePerKelvinMole,
    #[unit(0.001, "mJ/(K · mol)", "millijoules per kelvin mole")]
    MillijoulePerKelvinMole,
    #[unit(9.9999999999999995e-07, "µJ/(K · mol)", "microjoules per kelvin mole")]
    MicrojoulePerKelvinMole,
    #[unit(1.0000000000000001e-09, "nJ/(K · mol)", "nanojoules per kelvin mole")]
    NanojoulePerKelvinMole,
    #[unit(9.9999999999999998e-13, "pJ/(K · mol)", "picojoules per kelvin mole")]
    PicojoulePerKelvinMole,
    #[unit(1.0000000000000001e-15, "fJ/(K · mol)", "femtojoules per kelvin mole")]
    FemtojoulePerKelvinMole,
    #[unit(1.0000000000000001e-18, "aJ/(K · mol)", "attojoules per kelvin mole")]
    AttojoulePerKelvinMole,
    #[unit(9.9999999999999991e-22, "zJ/(K · mol)", "zeptojoules per kelvin mole")]
    ZeptojoulePerKelvinMole,
    #[unit(9.9999999999999992e-25, "yJ/(K · mol)", "yoctojoules per kelvin mole")]
    YoctojoulePerKelvinMole,
    #[unit(
        1055.056,
        "Btu (IT)/(K · mol)",
        "British thermal units (IT) per kelvin mole"
    )]
    BtuItPerKelvinMole,
    #[unit(
        1054.3499999999999,
        "Btu/(K · mol)",
        "British thermal units per kelvin mole"
    )]
    BtuPerKelvinMole,
    #[unit(
        4.1867999999999999,
        "cal (IT)/(K · mol)",
        "calories (IT) per kelvin mole"
    )]
    CalorieItPerKelvinMole,
    #[unit(4.1840000000000002, "cal/(K · mol)", "calories per kelvin mole")]
    CaloriePerKelvinMole,
    #[unit(
        4186.8000000000002,
        "Cal (IT)/(K · mol)",
        "Calories (IT) per kelvin mole"
    )]
    CalorieItNutritionPerKelvinMole,
    #[unit(4184.0, "Cal/(K · mol)", "Calories per kelvin mole")]
    CalorieNutritionPerKelvinMole,
    #[unit(
        4186.8000000000002,
        "kcal (IT)/(K · mol)",
        "kilocalories (IT) per kelvin mole"
    )]
    KilocalorieItPerKelvinMole,
    #[unit(4184.0, "kcal/(K · mol)", "kilocalories per kelvin mole")]
    KilocaloriePerKelvinMole,
    #[unit(
        6.0221407599999999e+23,
        "J/(K · particle)",
        "joules per kelvin particle"
    )]
    JoulePerKelvinParticle,
    #[unit(
        96485.332123310014,
        "eV/(K · particle)",
        "electronvolts per kelvin particle"
    )]
    ElectronvoltPerKelvinParticle,
    #[unit(8.3144626180000003, "R", "molar gas constants")]
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

#[derive(Copy, Clone, Unit)]
#[dim(MolarMass)]
pub enum MolarMassUnit {
    #[unit(1e+21, "Yg/mol", "yottagrams per mole")]
    YottagramPerMole,
    #[unit(1e+18, "Zg/mol", "zettagrams per mole")]
    ZettagramPerMole,
    #[unit(1000000000000000.0, "Eg/mol", "exagrams per mole")]
    ExagramPerMole,
    #[unit(1000000000000.0, "Pg/mol", "petagrams per mole")]
    PetagramPerMole,
    #[unit(1000000000.0, "Tg/mol", "teragrams per mole")]
    TeragramPerMole,
    #[unit(1000000.0, "Gg/mol", "gigagrams per mole")]
    GigagramPerMole,
    #[unit(1000.0, "Mg/mol", "megagrams per mole")]
    MegagramPerMole,
    #[unit(1.0, "kg/mol", "kilograms per mole")]
    KilogramPerMole,
    #[unit(0.10000000000000001, "hg/mol", "hectograms per mole")]
    HectogramPerMole,
    #[unit(0.01, "dag/mol", "decagrams per mole")]
    DecagramPerMole,
    #[unit(0.001, "g/mol", "grams per mole")]
    GramPerMole,
    #[unit(0.0001, "dg/mol", "decigrams per mole")]
    DecigramPerMole,
    #[unit(1.0000000000000001e-05, "cg/mol", "centigrams per mole")]
    CentigramPerMole,
    #[unit(9.9999999999999995e-07, "mg/mol", "milligrams per mole")]
    MilligramPerMole,
    #[unit(9.9999999999999986e-10, "µg/mol", "micrograms per mole")]
    MicrogramPerMole,
    #[unit(9.9999999999999998e-13, "ng/mol", "nanograms per mole")]
    NanogramPerMole,
    #[unit(1.0000000000000001e-15, "pg/mol", "picograms per mole")]
    PicogramPerMole,
    #[unit(1.0000000000000001e-18, "fg/mol", "femtograms per mole")]
    FemtogramPerMole,
    #[unit(1.0000000000000001e-21, "ag/mol", "attograms per mole")]
    AttogramPerMole,
    #[unit(9.9999999999999992e-25, "zg/mol", "zeptograms per mole")]
    ZeptogramPerMole,
    #[unit(9.9999999999999986e-28, "yg/mol", "yoctograms per mole")]
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

#[derive(Copy, Clone, Unit)]
#[dim(MolarRadioactivity)]
pub enum MolarRadioactivityUnit {
    #[unit(1.0, "Bq/mol", "becquerels per mole")]
    BecquerelPerMole,
    #[unit(37000000000.0, "Ci/mol", "curies per mole")]
    CuriePerMole,
    #[unit(0.016666666666666666, "dpm/mol", "disintegrations per minute per mole")]
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

#[derive(Copy, Clone, Unit)]
#[dim(MolarVolume)]
pub enum MolarVolumeUnit {
    #[unit(1.0, "m³/mol", "cubic meters per mole")]
    CubicMeterPerMole,
    #[unit(6.0221407599999999e+23, "m³/particle", "cubic meters per particle")]
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

#[derive(Copy, Clone, Unit)]
#[dim(Molality)]
pub enum MolalityUnit {
    #[unit(1.0, "mol/kg", "moles per kilogram")]
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

#[derive(Copy, Clone, Unit)]
#[dim(MassConcentration)]
pub enum MassConcentrationUnit {
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

#[derive(Copy, Clone, Unit)]
#[dim(CatalyticActivity)]
pub enum CatalyticActivityUnit {
    #[unit(9.9999999999999998e+23, "Ykat", "yottakatals")]
    Yottakatal,
    #[unit(1e+21, "Zkat", "zettakatals")]
    Zettakatal,
    #[unit(1e+18, "Ekat", "exakatals")]
    Exakatal,
    #[unit(1000000000000000.0, "Pkat", "petakatals")]
    Petakatal,
    #[unit(1000000000000.0, "Tkat", "terakatals")]
    Terakatal,
    #[unit(1000000000.0, "Gkat", "gigakatals")]
    Gigakatal,
    #[unit(1000000.0, "Mkat", "megakatals")]
    Megakatal,
    #[unit(1000.0, "kkat", "kilokatals")]
    Kilokatal,
    #[unit(100.0, "hkat", "hectokatals")]
    Hectokatal,
    #[unit(10.0, "dakat", "decakatals")]
    Decakatal,
    #[unit(1.0, "kat", "katals")]
    Katal,
    #[unit(0.10000000000000001, "dkat", "decikatals")]
    Decikatal,
    #[unit(0.01, "ckat", "centikatals")]
    Centikatal,
    #[unit(0.001, "mkat", "millikatals")]
    Millikatal,
    #[unit(9.9999999999999995e-07, "µkat", "microkatals")]
    Microkatal,
    #[unit(1.0000000000000001e-09, "nkat", "nanokatals")]
    Nanokatal,
    #[unit(9.9999999999999998e-13, "pkat", "picokatals")]
    Picokatal,
    #[unit(1.0000000000000001e-15, "fkat", "femtokatals")]
    Femtokatal,
    #[unit(1.0000000000000001e-18, "akat", "attokatals")]
    Attokatal,
    #[unit(9.9999999999999991e-22, "zkat", "zeptokatals")]
    Zeptokatal,
    #[unit(9.9999999999999992e-25, "ykat", "yoctokatals")]
    Yoctokatal,
    #[unit(1.6605390671738466e-24, "particle/s", "particles per second")]
    ParticlePerSecond,
    #[unit(1.0, "mol/s", "moles per second")]
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

#[derive(Copy, Clone, Unit)]
#[dim(CatalyticActivityConcentration)]
pub enum CatalyticActivityConcentrationUnit {
    #[unit(9.9999999999999998e+23, "Ykat/m³", "yottakatals per cubic meter")]
    YottakatalPerCubicMeter,
    #[unit(1e+21, "Zkat/m³", "zettakatals per cubic meter")]
    ZettakatalPerCubicMeter,
    #[unit(1e+18, "Ekat/m³", "exakatals per cubic meter")]
    ExakatalPerCubicMeter,
    #[unit(1000000000000000.0, "Pkat/m³", "petakatals per cubic meter")]
    PetakatalPerCubicMeter,
    #[unit(1000000000000.0, "Tkat/m³", "terakatals per cubic meter")]
    TerakatalPerCubicMeter,
    #[unit(1000000000.0, "Gkat/m³", "gigakatals per cubic meter")]
    GigakatalPerCubicMeter,
    #[unit(1000000.0, "Mkat/m³", "megakatals per cubic meter")]
    MegakatalPerCubicMeter,
    #[unit(1000.0, "kkat/m³", "kilokatals per cubic meter")]
    KilokatalPerCubicMeter,
    #[unit(100.0, "hkat/m³", "hectokatals per cubic meter")]
    HectokatalPerCubicMeter,
    #[unit(10.0, "dakat/m³", "decakatals per cubic meter")]
    DecakatalPerCubicMeter,
    #[unit(1.0, "kat/m³", "katals per cubic meter")]
    KatalPerCubicMeter,
    #[unit(0.10000000000000001, "dkat/m³", "decikatals per cubic meter")]
    DecikatalPerCubicMeter,
    #[unit(0.01, "ckat/m³", "centikatals per cubic meter")]
    CentikatalPerCubicMeter,
    #[unit(0.001, "mkat/m³", "millikatals per cubic meter")]
    MillikatalPerCubicMeter,
    #[unit(9.9999999999999995e-07, "µkat/m³", "microkatals per cubic meter")]
    MicrokatalPerCubicMeter,
    #[unit(1.0000000000000001e-09, "nkat/m³", "nanokatals per cubic meter")]
    NanokatalPerCubicMeter,
    #[unit(9.9999999999999998e-13, "pkat/m³", "picokatals per cubic meter")]
    PicokatalPerCubicMeter,
    #[unit(1.0000000000000001e-15, "fkat/m³", "femtokatals per cubic meter")]
    FemtokatalPerCubicMeter,
    #[unit(1.0000000000000001e-18, "akat/m³", "attokatals per cubic meter")]
    AttokatalPerCubicMeter,
    #[unit(9.9999999999999991e-22, "zkat/m³", "zeptokatals per cubic meter")]
    ZeptokatalPerCubicMeter,
    #[unit(9.9999999999999992e-25, "ykat/m³", "yoctokatals per cubic meter")]
    YoctokatalPerCubicMeter,
    #[unit(1000000.0, "kkat/L", "kilokatals per liter")]
    KilokatalPerLiter,
    #[unit(1000.0, "kat/L", "katals per liter")]
    KatalPerLiter,
    #[unit(1.0, "mkat/L", "millikatals per liter")]
    MillikatalPerLiter,
    #[unit(0.001, "μkat/L", "microkatals per liter")]
    MicrokatalPerLiter,
    #[unit(9.9999999999999995e-07, "nkat/L", "nanokatals per liter")]
    NanokatalPerLiter,
    #[unit(1.0000000000000001e-09, "pkat/L", "picokatals per liter")]
    PicokatalPerLiter,
    #[unit(9.9999999999999998e-13, "fkat/L", "femtokatals per liter")]
    FemtokatalPerLiter,
    #[unit(
        1.6605390671738466e-24,
        "particle · s⁻¹ · m⁻³",
        "particles per second cubic meter"
    )]
    ParticlePerSecondCubicMeter,
}

pub type Radioactivity = StaticSIQuantity<{ ISQ { s: -1, ..ISQ::ZERO } }>;

#[derive(Copy, Clone, Unit)]
#[dim(Radioactivity)]
pub enum RadioactivityUnit {
    #[unit(9.9999999999999998e+23, "YBq", "yottabecquerels")]
    Yottabecquerel,
    #[unit(1e+21, "ZBq", "zettabecquerels")]
    Zettabecquerel,
    #[unit(1e+18, "EBq", "exabecquerels")]
    Exabecquerel,
    #[unit(1000000000000000.0, "PBq", "petabecquerels")]
    Petabecquerel,
    #[unit(1000000000000.0, "TBq", "terabecquerels")]
    Terabecquerel,
    #[unit(1000000000.0, "GBq", "gigabecquerels")]
    Gigabecquerel,
    #[unit(1000000.0, "MBq", "megabecquerels")]
    Megabecquerel,
    #[unit(1000.0, "kBq", "kilobecquerels")]
    Kilobecquerel,
    #[unit(100.0, "hBq", "hectobecquerels")]
    Hectobecquerel,
    #[unit(10.0, "daBq", "decabecquerels")]
    Decabecquerel,
    #[unit(1.0, "Bq", "becquerels")]
    Becquerel,
    #[unit(0.001, "mBq", "millibecquerels")]
    Millibecquerel,
    #[unit(9.9999999999999995e-07, "µBq", "microbecquerels")]
    Microbecquerel,
    #[unit(1.0000000000000001e-09, "nBq", "nanobecquerels")]
    Nanobecquerel,
    #[unit(3.7e+19, "GCi", "gigacuries")]
    Gigacurie,
    #[unit(37000000000000000.0, "MCi", "megacuries")]
    Megacurie,
    #[unit(37000000000000.0, "kCi", "kilocuries")]
    Kilocurie,
    #[unit(37000000000.0, "Ci", "curies")]
    Curie,
    #[unit(37000000.0, "mCi", "millicuries")]
    Millicurie,
    #[unit(37000.0, "µCi", "microcuries")]
    Microcurie,
    #[unit(37.0, "nCi", "nanocuries")]
    Nanocurie,
    #[unit(0.016666666666666666, "dpm", "disintegrations per minute")]
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

#[derive(Copy, Clone, Unit)]
#[dim(SpecificRadioactivity)]
pub enum SpecificRadioactivityUnit {
    #[unit(1.0, "Bq/kg", "becquerels per kilogram")]
    BecquerelPerKilogram,
    #[unit(37000000000.0, "Ci/kg", "curie per kilogram")]
    CuriePerKilogram,
    #[unit(
        0.016666666666666666,
        "dpm/kg",
        "disintegrations per minute per kilogram"
    )]
    DisintegrationsPerMinutePerKilogram,
}
