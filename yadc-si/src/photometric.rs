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

#[derive(Unit, Copy, Clone)]
#[dim(Luminance)]
pub enum LuminanceUnit {
    #[linear("Ycd/m²", "yottacandelas per square meter", 9.9999999999999998e+23)]
    YottacandelaPerSquareMeter,
    #[linear("Zcd/m²", "zettacandelas per square meter", 1e+21)]
    ZettacandelaPerSquareMeter,
    #[linear("Ecd/m²", "exacandelas per square meter", 1e+18)]
    ExacandelaPerSquareMeter,
    #[linear("Pcd/m²", "petacandelas per square meter", 1000000000000000.0)]
    PetacandelaPerSquareMeter,
    #[linear("Tcd/m²", "teracandelas per square meter", 1000000000000.0)]
    TeracandelaPerSquareMeter,
    #[linear("Gcd/m²", "gigacandelas per square meter", 1000000000.0)]
    GigacandelaPerSquareMeter,
    #[linear("Mcd/m²", "megacandelas per square meter", 1000000.0)]
    MegacandelaPerSquareMeter,
    #[linear("kcd/m²", "kilocandelas per square meter", 1000.0)]
    KilocandelaPerSquareMeter,
    #[linear("hcd/m²", "hectocandelas per square meter", 100.0)]
    HectocandelaPerSquareMeter,
    #[linear("dacd/m²", "decacandelas per square meter", 10.0)]
    DecacandelaPerSquareMeter,
    #[linear("cd/m²", "candelas per square meter", 1.0)]
    CandelaPerSquareMeter,
    #[linear("dcd/m²", "decicandelas per square meter", 0.10000000000000001)]
    DecicandelaPerSquareMeter,
    #[linear("ccd/m²", "centicandelas per square meter", 0.01)]
    CenticandelaPerSquareMeter,
    #[linear("mcd/m²", "millicandelas per square meter", 0.001)]
    MillicandelaPerSquareMeter,
    #[linear("µcd/m²", "microcandelas per square meter", 9.9999999999999995e-07)]
    MicrocandelaPerSquareMeter,
    #[linear("ncd/m²", "nanocandelas per square meter", 1.0000000000000001e-09)]
    NanocandelaPerSquareMeter,
    #[linear("pcd/m²", "picocandelas per square meter", 9.9999999999999998e-13)]
    PicocandelaPerSquareMeter,
    #[linear("fcd/m²", "femtocandelas per square meter", 1.0000000000000001e-15)]
    FemtocandelaPerSquareMeter,
    #[linear("acd/m²", "attocandelas per square meter", 1.0000000000000001e-18)]
    AttocandelaPerSquareMeter,
    #[linear("zcd/m²", "zeptocandelas per square meter", 9.9999999999999991e-22)]
    ZeptocandelaPerSquareMeter,
    #[linear("ycd/m²", "yoctocandelas per square meter", 9.9999999999999992e-25)]
    YoctocandelaPerSquareMeter,
    #[linear("cd/pm²", "candelas per square picometer", 9.9999999999999998e+23)]
    CandelaPerSquarePicometer,
    #[linear("cd/nm²", "candelas per square nanometer", 1e+18)]
    CandelaPerSquareNanometer,
    #[linear("cd/µm²", "candelas per square micrometer", 1000000000000.0)]
    CandelaPerSquareMicrometer,
    #[linear("cd/mm²", "candelas per square millimeter", 1000000.0)]
    CandelaPerSquareMillimeter,
    #[linear("cd/cm²", "candelas per square centimeter", 10000.0)]
    CandelaPerSquareCentimeter,
    #[linear("cd/km²", "candelas per square kilometer", 9.9999999999999995e-07)]
    CandelaPerSquareKilometer,
    #[linear("cd/Mm²", "candelas per square megameter", 9.9999999999999998e-13)]
    CandelaPerSquareMegameter,
    #[linear("cd/Gm²", "candelas per square gigameter", 1.0000000000000001e-18)]
    CandelaPerSquareGigameter,
    #[linear("cd/Tm²", "candelas per square terameter", 9.9999999999999992e-25)]
    CandelaPerSquareTerameter,
    #[linear("cd/in²", "candelas per square inch", 1550.0031000062002)]
    CandelaPerSquareInch,
    #[linear("cd/ft²", "candelas per square foot", 10.763910416709722)]
    CandelaPerSquareFoot,
    #[linear("fl", "footlamberts", 3.4262590996353905)]
    Footlambert,
    #[linear("la", "lamberts", 3183.0988618379065)]
    Lambert,
    #[linear("sb", "stilbs", 10000.0)]
    Stilb,
}

pub type LuminousIntensity = StaticSIQuantity<{ ISQ { cd: 1, ..ISQ::ZERO } }>;

#[derive(Unit, Copy, Clone)]
#[dim(LuminousIntensity)]
pub enum LuminousIntensityUnit {
    #[linear("Ycd", "yottacandelas", 9.9999999999999998e+23)]
    Yottacandela,
    #[linear("Zcd", "zettacandelas", 1e+21)]
    Zettacandela,
    #[linear("Ecd", "exacandelas", 1e+18)]
    Exacandela,
    #[linear("Pcd", "petacandelas", 1000000000000000.0)]
    Petacandela,
    #[linear("Tcd", "teracandelas", 1000000000000.0)]
    Teracandela,
    #[linear("Gcd", "gigacandelas", 1000000000.0)]
    Gigacandela,
    #[linear("Mcd", "megacandelas", 1000000.0)]
    Megacandela,
    #[linear("kcd", "kilocandelas", 1000.0)]
    Kilocandela,
    #[linear("hcd", "hectocandelas", 100.0)]
    Hectocandela,
    #[linear("dacd", "decacandelas", 10.0)]
    Decacandela,
    #[linear("cd", "candelas", 1.0)]
    Candela,
    #[linear("dcd", "decicandelas", 0.10000000000000001)]
    Decicandela,
    #[linear("ccd", "centicandelas", 0.01)]
    Centicandela,
    #[linear("mcd", "millicandelas", 0.001)]
    Millicandela,
    #[linear("µcd", "microcandelas", 9.9999999999999995e-07)]
    Microcandela,
    #[linear("ncd", "nanocandelas", 1.0000000000000001e-09)]
    Nanocandela,
    #[linear("pcd", "picocandelas", 9.9999999999999998e-13)]
    Picocandela,
    #[linear("fcd", "femtocandelas", 1.0000000000000001e-15)]
    Femtocandela,
    #[linear("acd", "attocandelas", 1.0000000000000001e-18)]
    Attocandela,
    #[linear("zcd", "zeptocandelas", 9.9999999999999991e-22)]
    Zeptocandela,
    #[linear("ycd", "yoctocandelas", 9.9999999999999992e-25)]
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

#[derive(Unit, Copy, Clone)]
#[dim(Illuminance)]
pub enum IlluminanceUnit {
    #[linear("Ylx", "yottalux", 9.9999999999999998e+23)]
    Yottalux,
    #[linear("Zlx", "zettalux", 1e+21)]
    Zettalux,
    #[linear("Elx", "exalux", 1e+18)]
    Exalux,
    #[linear("Plx", "petalux", 1000000000000000.0)]
    Petalux,
    #[linear("Tlx", "teralux", 1000000000000.0)]
    Teralux,
    #[linear("Glx", "gigalux", 1000000000.0)]
    Gigalux,
    #[linear("Mlx", "megalux", 1000000.0)]
    Megalux,
    #[linear("klx", "kilolux", 1000.0)]
    Kilolux,
    #[linear("hlx", "hectolux", 100.0)]
    Hectolux,
    #[linear("dalx", "decalux", 10.0)]
    Decalux,
    #[linear("lx", "lux", 1.0)]
    Lux,
    #[linear("dlx", "decilux", 0.10000000000000001)]
    Decilux,
    #[linear("clx", "centilux", 0.01)]
    Centilux,
    #[linear("mlx", "millilux", 0.001)]
    Millilux,
    #[linear("µlx", "microlux", 9.9999999999999995e-07)]
    Microlux,
    #[linear("nlx", "nanolux", 1.0000000000000001e-09)]
    Nanolux,
    #[linear("plx", "picolux", 9.9999999999999998e-13)]
    Picolux,
    #[linear("flx", "femtolux", 1.0000000000000001e-15)]
    Femtolux,
    #[linear("alx", "attolux", 1.0000000000000001e-18)]
    Attolux,
    #[linear("zlx", "zeptolux", 9.9999999999999991e-22)]
    Zeptolux,
    #[linear("ylx", "yoctolux", 9.9999999999999992e-25)]
    Yoctolux,
    #[linear("ph", "phot", 10000.0)]
    Phot,
    #[linear("fc", "footcandles", 10.763909999999999)]
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

#[derive(Unit, Copy, Clone)]
#[dim(RadiantExposure)]
pub enum RadiantExposureUnit {
    #[linear("YJ/m²", "yottajoules per square meter", 9.9999999999999998e+23)]
    YottajoulePerSquareMeter,
    #[linear("ZJ/m²", "zettajoules per square meter", 1e+21)]
    ZettajoulePerSquareMeter,
    #[linear("EJ/m²", "exajoules per square meter", 1e+18)]
    ExajoulePerSquareMeter,
    #[linear("PJ/m²", "petajoules per square meter", 1000000000000000.0)]
    PetajoulePerSquareMeter,
    #[linear("TJ/m²", "terajoules per square meter", 1000000000000.0)]
    TerajoulePerSquareMeter,
    #[linear("GJ/m²", "gigajoules per square meter", 1000000000.0)]
    GigajoulePerSquareMeter,
    #[linear("MJ/m²", "megajoules per square meter", 1000000.0)]
    MegajoulePerSquareMeter,
    #[linear("kJ/m²", "kilojoules per square meter", 1000.0)]
    KilojoulePerSquareMeter,
    #[linear("hJ/m²", "hectojoules per square meter", 100.0)]
    HectojoulePerSquareMeter,
    #[linear("daJ/m²", "decajoules per square meter", 10.0)]
    DecajoulePerSquareMeter,
    #[linear("J/m²", "joules per square meter", 1.0)]
    JoulePerSquareMeter,
    #[linear("dJ/m²", "decijoules per square meter", 0.10000000000000001)]
    DecijoulePerSquareMeter,
    #[linear("cJ/m²", "centijoules per square meter", 0.01)]
    CentijoulePerSquareMeter,
    #[linear("mJ/m²", "millijoules per square meter", 0.001)]
    MillijoulePerSquareMeter,
    #[linear("µJ/m²", "microjoules per square meter", 9.9999999999999995e-07)]
    MicrojoulePerSquareMeter,
    #[linear("nJ/m²", "nanojoules per square meter", 1.0000000000000001e-09)]
    NanojoulePerSquareMeter,
    #[linear("pJ/m²", "picojoules per square meter", 9.9999999999999998e-13)]
    PicojoulePerSquareMeter,
    #[linear("fJ/m²", "femtojoules per square meter", 1.0000000000000001e-15)]
    FemtojoulePerSquareMeter,
    #[linear("aJ/m²", "attojoules per square meter", 1.0000000000000001e-18)]
    AttojoulePerSquareMeter,
    #[linear("zJ/m²", "zeptojoules per square meter", 9.9999999999999991e-22)]
    ZeptojoulePerSquareMeter,
    #[linear("yJ/m²", "yoctojoules per square meter", 9.9999999999999992e-25)]
    YoctojoulePerSquareMeter,
    #[linear("W · h/m²", "watt hours per square meter", 3600.0)]
    WattHourPerSquareMeter,
    #[linear("GJ/cm²", "gigajoules per square centimeter", 10000000000000.0)]
    GigajoulePerSquareCentimeter,
    #[linear("MJ/cm²", "megajoules per square centimeter", 10000000000.0)]
    MegajoulePerSquareCentimeter,
    #[linear("kJ/cm²", "kilojoules per square centimeter", 10000000.0)]
    KilojoulePerSquareCentimeter,
    #[linear("hJ/cm²", "hectojoules per square centimeter", 1000000.0)]
    HectojoulePerSquareCentimeter,
    #[linear("daJ/cm²", "decajoules per square centimeter", 100000.0)]
    DecajoulePerSquareCentimeter,
    #[linear("J/cm²", "joules per square centimeter", 10000.0)]
    JoulePerSquareCentimeter,
    #[linear("dJ/cm²", "decijoules per square centimeter", 1000.0)]
    DecijoulePerSquareCentimeter,
    #[linear("cJ/cm²", "centijoules per square centimeter", 100.0)]
    CentijoulePerSquareCentimeter,
    #[linear("mJ/cm²", "millijoules per square centimeter", 10.0)]
    MillijoulePerSquareCentimeter,
    #[linear("µJ/cm²", "microjoules per square centimeter", 0.01)]
    MicrojoulePerSquareCentimeter,
    #[linear("nJ/cm²", "nanojoules per square centimeter", 1.0000000000000001e-05)]
    NanojoulePerSquareCentimeter,
    #[linear("GJ/mm²", "gigajoules per square millimeter", 1000000000000000.0)]
    GigajoulePerSquareMillimeter,
    #[linear("MJ/mm²", "megajoules per square millimeter", 1000000000000.0)]
    MegajoulePerSquareMillimeter,
    #[linear("kJ/mm²", "kilojoules per square millimeter", 1000000000.0)]
    KilojoulePerSquareMillimeter,
    #[linear("hJ/mm²", "hectojoules per square millimeter", 100000000.0)]
    HectojoulePerSquareMillimeter,
    #[linear("daJ/mm²", "decajoules per square millimeter", 10000000.0)]
    DecajoulePerSquareMillimeter,
    #[linear("J/mm²", "joules per square millimeter", 1000000.0)]
    JoulePerSquareMillimeter,
    #[linear("dJ/mm²", "decijoules per square millimeter", 100000.0)]
    DecijoulePerSquareMillimeter,
    #[linear("cJ/mm²", "centijoules per square millimeter", 10000.0)]
    CentijoulePerSquareMillimeter,
    #[linear("mJ/mm²", "millijoules per square millimeter", 1000.0)]
    MillijoulePerSquareMillimeter,
    #[linear("µJ/mm²", "microjoules per square millimeter", 1.0)]
    MicrojoulePerSquareMillimeter,
    #[linear("nJ/mm²", "nanojoules per square millimeter", 0.001)]
    NanojoulePerSquareMillimeter,
}
