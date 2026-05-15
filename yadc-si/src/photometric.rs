use crate::{ISQ, StaticSIQuantity};
use yadc::{Dimension, Unit};

pub type Luminance = StaticSIQuantity<
    {
        ISQ {
            m: -2,
            cd: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(Luminance)]
pub enum LuminanceUnit {
    #[unit(9.9999999999999998e+23, "Ycd/m²", "yottacandelas per square meter")]
    YottacandelaPerSquareMeter,
    #[unit(1e+21, "Zcd/m²", "zettacandelas per square meter")]
    ZettacandelaPerSquareMeter,
    #[unit(1e+18, "Ecd/m²", "exacandelas per square meter")]
    ExacandelaPerSquareMeter,
    #[unit(1000000000000000.0, "Pcd/m²", "petacandelas per square meter")]
    PetacandelaPerSquareMeter,
    #[unit(1000000000000.0, "Tcd/m²", "teracandelas per square meter")]
    TeracandelaPerSquareMeter,
    #[unit(1000000000.0, "Gcd/m²", "gigacandelas per square meter")]
    GigacandelaPerSquareMeter,
    #[unit(1000000.0, "Mcd/m²", "megacandelas per square meter")]
    MegacandelaPerSquareMeter,
    #[unit(1000.0, "kcd/m²", "kilocandelas per square meter")]
    KilocandelaPerSquareMeter,
    #[unit(100.0, "hcd/m²", "hectocandelas per square meter")]
    HectocandelaPerSquareMeter,
    #[unit(10.0, "dacd/m²", "decacandelas per square meter")]
    DecacandelaPerSquareMeter,
    #[unit(1.0, "cd/m²", "candelas per square meter")]
    CandelaPerSquareMeter,
    #[unit(0.10000000000000001, "dcd/m²", "decicandelas per square meter")]
    DecicandelaPerSquareMeter,
    #[unit(0.01, "ccd/m²", "centicandelas per square meter")]
    CenticandelaPerSquareMeter,
    #[unit(0.001, "mcd/m²", "millicandelas per square meter")]
    MillicandelaPerSquareMeter,
    #[unit(9.9999999999999995e-07, "µcd/m²", "microcandelas per square meter")]
    MicrocandelaPerSquareMeter,
    #[unit(1.0000000000000001e-09, "ncd/m²", "nanocandelas per square meter")]
    NanocandelaPerSquareMeter,
    #[unit(9.9999999999999998e-13, "pcd/m²", "picocandelas per square meter")]
    PicocandelaPerSquareMeter,
    #[unit(1.0000000000000001e-15, "fcd/m²", "femtocandelas per square meter")]
    FemtocandelaPerSquareMeter,
    #[unit(1.0000000000000001e-18, "acd/m²", "attocandelas per square meter")]
    AttocandelaPerSquareMeter,
    #[unit(9.9999999999999991e-22, "zcd/m²", "zeptocandelas per square meter")]
    ZeptocandelaPerSquareMeter,
    #[unit(9.9999999999999992e-25, "ycd/m²", "yoctocandelas per square meter")]
    YoctocandelaPerSquareMeter,
    #[unit(9.9999999999999998e+23, "cd/pm²", "candelas per square picometer")]
    CandelaPerSquarePicometer,
    #[unit(1e+18, "cd/nm²", "candelas per square nanometer")]
    CandelaPerSquareNanometer,
    #[unit(1000000000000.0, "cd/µm²", "candelas per square micrometer")]
    CandelaPerSquareMicrometer,
    #[unit(1000000.0, "cd/mm²", "candelas per square millimeter")]
    CandelaPerSquareMillimeter,
    #[unit(10000.0, "cd/cm²", "candelas per square centimeter")]
    CandelaPerSquareCentimeter,
    #[unit(9.9999999999999995e-07, "cd/km²", "candelas per square kilometer")]
    CandelaPerSquareKilometer,
    #[unit(9.9999999999999998e-13, "cd/Mm²", "candelas per square megameter")]
    CandelaPerSquareMegameter,
    #[unit(1.0000000000000001e-18, "cd/Gm²", "candelas per square gigameter")]
    CandelaPerSquareGigameter,
    #[unit(9.9999999999999992e-25, "cd/Tm²", "candelas per square terameter")]
    CandelaPerSquareTerameter,
    #[unit(1550.0031000062002, "cd/in²", "candelas per square inch")]
    CandelaPerSquareInch,
    #[unit(10.763910416709722, "cd/ft²", "candelas per square foot")]
    CandelaPerSquareFoot,
    #[unit(3.4262590996353905, "fl", "footlamberts")]
    Footlambert,
    #[unit(3183.0988618379065, "la", "lamberts")]
    Lambert,
    #[unit(10000.0, "sb", "stilbs")]
    Stilb,
}

pub type LuminousIntensity = StaticSIQuantity<{ ISQ { cd: 1, ..ISQ::ZERO } }>;

#[derive(Copy, Clone, Unit)]
#[dim(LuminousIntensity)]
pub enum LuminousIntensityUnit {
    #[unit(9.9999999999999998e+23, "Ycd", "yottacandelas")]
    Yottacandela,
    #[unit(1e+21, "Zcd", "zettacandelas")]
    Zettacandela,
    #[unit(1e+18, "Ecd", "exacandelas")]
    Exacandela,
    #[unit(1000000000000000.0, "Pcd", "petacandelas")]
    Petacandela,
    #[unit(1000000000000.0, "Tcd", "teracandelas")]
    Teracandela,
    #[unit(1000000000.0, "Gcd", "gigacandelas")]
    Gigacandela,
    #[unit(1000000.0, "Mcd", "megacandelas")]
    Megacandela,
    #[unit(1000.0, "kcd", "kilocandelas")]
    Kilocandela,
    #[unit(100.0, "hcd", "hectocandelas")]
    Hectocandela,
    #[unit(10.0, "dacd", "decacandelas")]
    Decacandela,
    #[unit(1.0, "cd", "candelas")]
    Candela,
    #[unit(0.10000000000000001, "dcd", "decicandelas")]
    Decicandela,
    #[unit(0.01, "ccd", "centicandelas")]
    Centicandela,
    #[unit(0.001, "mcd", "millicandelas")]
    Millicandela,
    #[unit(9.9999999999999995e-07, "µcd", "microcandelas")]
    Microcandela,
    #[unit(1.0000000000000001e-09, "ncd", "nanocandelas")]
    Nanocandela,
    #[unit(9.9999999999999998e-13, "pcd", "picocandelas")]
    Picocandela,
    #[unit(1.0000000000000001e-15, "fcd", "femtocandelas")]
    Femtocandela,
    #[unit(1.0000000000000001e-18, "acd", "attocandelas")]
    Attocandela,
    #[unit(9.9999999999999991e-22, "zcd", "zeptocandelas")]
    Zeptocandela,
    #[unit(9.9999999999999992e-25, "ycd", "yoctocandelas")]
    Yoctocandela,
}

pub type Illuminance = StaticSIQuantity<
    {
        ISQ {
            m: -2,
            cd: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(Illuminance)]
pub enum IlluminanceUnit {
    #[unit(9.9999999999999998e+23, "Ylx", "yottalux")]
    Yottalux,
    #[unit(1e+21, "Zlx", "zettalux")]
    Zettalux,
    #[unit(1e+18, "Elx", "exalux")]
    Exalux,
    #[unit(1000000000000000.0, "Plx", "petalux")]
    Petalux,
    #[unit(1000000000000.0, "Tlx", "teralux")]
    Teralux,
    #[unit(1000000000.0, "Glx", "gigalux")]
    Gigalux,
    #[unit(1000000.0, "Mlx", "megalux")]
    Megalux,
    #[unit(1000.0, "klx", "kilolux")]
    Kilolux,
    #[unit(100.0, "hlx", "hectolux")]
    Hectolux,
    #[unit(10.0, "dalx", "decalux")]
    Decalux,
    #[unit(1.0, "lx", "lux")]
    Lux,
    #[unit(0.10000000000000001, "dlx", "decilux")]
    Decilux,
    #[unit(0.01, "clx", "centilux")]
    Centilux,
    #[unit(0.001, "mlx", "millilux")]
    Millilux,
    #[unit(9.9999999999999995e-07, "µlx", "microlux")]
    Microlux,
    #[unit(1.0000000000000001e-09, "nlx", "nanolux")]
    Nanolux,
    #[unit(9.9999999999999998e-13, "plx", "picolux")]
    Picolux,
    #[unit(1.0000000000000001e-15, "flx", "femtolux")]
    Femtolux,
    #[unit(1.0000000000000001e-18, "alx", "attolux")]
    Attolux,
    #[unit(9.9999999999999991e-22, "zlx", "zeptolux")]
    Zeptolux,
    #[unit(9.9999999999999992e-25, "ylx", "yoctolux")]
    Yoctolux,
    #[unit(10000.0, "ph", "phot")]
    Phot,
    #[unit(10.763909999999999, "fc", "footcandles")]
    Footcandle,
}

pub type RadiantExposure = StaticSIQuantity<
    {
        ISQ {
            kg: 1,
            s: -2,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(RadiantExposure)]
pub enum RadiantExposureUnit {
    #[unit(9.9999999999999998e+23, "YJ/m²", "yottajoules per square meter")]
    YottajoulePerSquareMeter,
    #[unit(1e+21, "ZJ/m²", "zettajoules per square meter")]
    ZettajoulePerSquareMeter,
    #[unit(1e+18, "EJ/m²", "exajoules per square meter")]
    ExajoulePerSquareMeter,
    #[unit(1000000000000000.0, "PJ/m²", "petajoules per square meter")]
    PetajoulePerSquareMeter,
    #[unit(1000000000000.0, "TJ/m²", "terajoules per square meter")]
    TerajoulePerSquareMeter,
    #[unit(1000000000.0, "GJ/m²", "gigajoules per square meter")]
    GigajoulePerSquareMeter,
    #[unit(1000000.0, "MJ/m²", "megajoules per square meter")]
    MegajoulePerSquareMeter,
    #[unit(1000.0, "kJ/m²", "kilojoules per square meter")]
    KilojoulePerSquareMeter,
    #[unit(100.0, "hJ/m²", "hectojoules per square meter")]
    HectojoulePerSquareMeter,
    #[unit(10.0, "daJ/m²", "decajoules per square meter")]
    DecajoulePerSquareMeter,
    #[unit(1.0, "J/m²", "joules per square meter")]
    JoulePerSquareMeter,
    #[unit(0.10000000000000001, "dJ/m²", "decijoules per square meter")]
    DecijoulePerSquareMeter,
    #[unit(0.01, "cJ/m²", "centijoules per square meter")]
    CentijoulePerSquareMeter,
    #[unit(0.001, "mJ/m²", "millijoules per square meter")]
    MillijoulePerSquareMeter,
    #[unit(9.9999999999999995e-07, "µJ/m²", "microjoules per square meter")]
    MicrojoulePerSquareMeter,
    #[unit(1.0000000000000001e-09, "nJ/m²", "nanojoules per square meter")]
    NanojoulePerSquareMeter,
    #[unit(9.9999999999999998e-13, "pJ/m²", "picojoules per square meter")]
    PicojoulePerSquareMeter,
    #[unit(1.0000000000000001e-15, "fJ/m²", "femtojoules per square meter")]
    FemtojoulePerSquareMeter,
    #[unit(1.0000000000000001e-18, "aJ/m²", "attojoules per square meter")]
    AttojoulePerSquareMeter,
    #[unit(9.9999999999999991e-22, "zJ/m²", "zeptojoules per square meter")]
    ZeptojoulePerSquareMeter,
    #[unit(9.9999999999999992e-25, "yJ/m²", "yoctojoules per square meter")]
    YoctojoulePerSquareMeter,
    #[unit(3600.0, "W · h/m²", "watt hours per square meter")]
    WattHourPerSquareMeter,
    #[unit(10000000000000.0, "GJ/cm²", "gigajoules per square centimeter")]
    GigajoulePerSquareCentimeter,
    #[unit(10000000000.0, "MJ/cm²", "megajoules per square centimeter")]
    MegajoulePerSquareCentimeter,
    #[unit(10000000.0, "kJ/cm²", "kilojoules per square centimeter")]
    KilojoulePerSquareCentimeter,
    #[unit(1000000.0, "hJ/cm²", "hectojoules per square centimeter")]
    HectojoulePerSquareCentimeter,
    #[unit(100000.0, "daJ/cm²", "decajoules per square centimeter")]
    DecajoulePerSquareCentimeter,
    #[unit(10000.0, "J/cm²", "joules per square centimeter")]
    JoulePerSquareCentimeter,
    #[unit(1000.0, "dJ/cm²", "decijoules per square centimeter")]
    DecijoulePerSquareCentimeter,
    #[unit(100.0, "cJ/cm²", "centijoules per square centimeter")]
    CentijoulePerSquareCentimeter,
    #[unit(10.0, "mJ/cm²", "millijoules per square centimeter")]
    MillijoulePerSquareCentimeter,
    #[unit(0.01, "µJ/cm²", "microjoules per square centimeter")]
    MicrojoulePerSquareCentimeter,
    #[unit(1.0000000000000001e-05, "nJ/cm²", "nanojoules per square centimeter")]
    NanojoulePerSquareCentimeter,
    #[unit(1000000000000000.0, "GJ/mm²", "gigajoules per square millimeter")]
    GigajoulePerSquareMillimeter,
    #[unit(1000000000000.0, "MJ/mm²", "megajoules per square millimeter")]
    MegajoulePerSquareMillimeter,
    #[unit(1000000000.0, "kJ/mm²", "kilojoules per square millimeter")]
    KilojoulePerSquareMillimeter,
    #[unit(100000000.0, "hJ/mm²", "hectojoules per square millimeter")]
    HectojoulePerSquareMillimeter,
    #[unit(10000000.0, "daJ/mm²", "decajoules per square millimeter")]
    DecajoulePerSquareMillimeter,
    #[unit(1000000.0, "J/mm²", "joules per square millimeter")]
    JoulePerSquareMillimeter,
    #[unit(100000.0, "dJ/mm²", "decijoules per square millimeter")]
    DecijoulePerSquareMillimeter,
    #[unit(10000.0, "cJ/mm²", "centijoules per square millimeter")]
    CentijoulePerSquareMillimeter,
    #[unit(1000.0, "mJ/mm²", "millijoules per square millimeter")]
    MillijoulePerSquareMillimeter,
    #[unit(1.0, "µJ/mm²", "microjoules per square millimeter")]
    MicrojoulePerSquareMillimeter,
    #[unit(0.001, "nJ/mm²", "nanojoules per square millimeter")]
    NanojoulePerSquareMillimeter,
}
