use crate::{ISQ, StaticSIQuantity};
use consts::*;
use yadc::Dimension;

pub type Luminance = StaticSIQuantity<
    {
        ISQ {
            m: -2,
            cd: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type LuminousIntensity = StaticSIQuantity<
    {
        ISQ {
            cd: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type Illuminance = StaticSIQuantity<
    {
        ISQ {
            m: -2,
            cd: 1,
            rad: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type RadiantExposure = StaticSIQuantity<
    {
        ISQ {
            kg: 1,
            s: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

/// The constants of this module: the base constants of its quantities
/// (re-exported), then one per multiple, each defined from them - `KILOMETRE`
/// is `1000.0 * METRE`, a `Length` and a unit of its own (`KILOMETRE.value(d)`).
/// Quantity arithmetic is `const`, so they compose in `const` items:
/// `const LIMIT: Velocity = 50.0 * KILOMETRE / HOUR;`.
pub mod consts {
    use super::*;
    use crate::base::{KILOGRAM, METRE, RADIAN, SECOND};

    /// The candela, the SI base unit of luminous intensity.
    pub use crate::base::CANDELA;

    /// One `YottacandelaPerSquareMetre` (Ycd/m²), as a `Luminance`.
    pub const YOTTACANDELA_PER_SQUARE_METRE: Luminance =
        9.9999999999999998e+23 * CANDELA_PER_SQUARE_METRE;

    /// One `ZettacandelaPerSquareMetre` (Zcd/m²), as a `Luminance`.
    pub const ZETTACANDELA_PER_SQUARE_METRE: Luminance = 1e+21 * CANDELA_PER_SQUARE_METRE;

    /// One `ExacandelaPerSquareMetre` (Ecd/m²), as a `Luminance`.
    pub const EXACANDELA_PER_SQUARE_METRE: Luminance = 1e+18 * CANDELA_PER_SQUARE_METRE;

    /// One `PetacandelaPerSquareMetre` (Pcd/m²), as a `Luminance`.
    pub const PETACANDELA_PER_SQUARE_METRE: Luminance =
        1000000000000000.0 * CANDELA_PER_SQUARE_METRE;

    /// One `TeracandelaPerSquareMetre` (Tcd/m²), as a `Luminance`.
    pub const TERACANDELA_PER_SQUARE_METRE: Luminance = 1000000000000.0 * CANDELA_PER_SQUARE_METRE;

    /// One `GigacandelaPerSquareMetre` (Gcd/m²), as a `Luminance`.
    pub const GIGACANDELA_PER_SQUARE_METRE: Luminance = 1000000000.0 * CANDELA_PER_SQUARE_METRE;

    /// One `MegacandelaPerSquareMetre` (Mcd/m²), as a `Luminance`.
    pub const MEGACANDELA_PER_SQUARE_METRE: Luminance = 1000000.0 * CANDELA_PER_SQUARE_METRE;

    /// One `KilocandelaPerSquareMetre` (kcd/m²), as a `Luminance`.
    pub const KILOCANDELA_PER_SQUARE_METRE: Luminance = 1000.0 * CANDELA_PER_SQUARE_METRE;

    /// One `HectocandelaPerSquareMetre` (hcd/m²), as a `Luminance`.
    pub const HECTOCANDELA_PER_SQUARE_METRE: Luminance = 100.0 * CANDELA_PER_SQUARE_METRE;

    /// One `DecacandelaPerSquareMetre` (dacd/m²), as a `Luminance`.
    pub const DECACANDELA_PER_SQUARE_METRE: Luminance = 10.0 * CANDELA_PER_SQUARE_METRE;

    /// One `CandelaPerSquareMetre` (cd/m²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_METRE: Luminance = CANDELA / (METRE * METRE);

    /// One `DecicandelaPerSquareMetre` (dcd/m²), as a `Luminance`.
    pub const DECICANDELA_PER_SQUARE_METRE: Luminance =
        0.10000000000000001 * CANDELA_PER_SQUARE_METRE;

    /// One `CenticandelaPerSquareMetre` (ccd/m²), as a `Luminance`.
    pub const CENTICANDELA_PER_SQUARE_METRE: Luminance = 0.01 * CANDELA_PER_SQUARE_METRE;

    /// One `MillicandelaPerSquareMetre` (mcd/m²), as a `Luminance`.
    pub const MILLICANDELA_PER_SQUARE_METRE: Luminance = 0.001 * CANDELA_PER_SQUARE_METRE;

    /// One `MicrocandelaPerSquareMetre` (µcd/m²), as a `Luminance`.
    pub const MICROCANDELA_PER_SQUARE_METRE: Luminance =
        9.9999999999999995e-07 * CANDELA_PER_SQUARE_METRE;

    /// One `NanocandelaPerSquareMetre` (ncd/m²), as a `Luminance`.
    pub const NANOCANDELA_PER_SQUARE_METRE: Luminance =
        1.0000000000000001e-09 * CANDELA_PER_SQUARE_METRE;

    /// One `PicocandelaPerSquareMetre` (pcd/m²), as a `Luminance`.
    pub const PICOCANDELA_PER_SQUARE_METRE: Luminance =
        9.9999999999999998e-13 * CANDELA_PER_SQUARE_METRE;

    /// One `FemtocandelaPerSquareMetre` (fcd/m²), as a `Luminance`.
    pub const FEMTOCANDELA_PER_SQUARE_METRE: Luminance =
        1.0000000000000001e-15 * CANDELA_PER_SQUARE_METRE;

    /// One `AttocandelaPerSquareMetre` (acd/m²), as a `Luminance`.
    pub const ATTOCANDELA_PER_SQUARE_METRE: Luminance =
        1.0000000000000001e-18 * CANDELA_PER_SQUARE_METRE;

    /// One `ZeptocandelaPerSquareMetre` (zcd/m²), as a `Luminance`.
    pub const ZEPTOCANDELA_PER_SQUARE_METRE: Luminance =
        9.9999999999999991e-22 * CANDELA_PER_SQUARE_METRE;

    /// One `YoctocandelaPerSquareMetre` (ycd/m²), as a `Luminance`.
    pub const YOCTOCANDELA_PER_SQUARE_METRE: Luminance =
        9.9999999999999992e-25 * CANDELA_PER_SQUARE_METRE;

    /// One `CandelaPerSquarePicometre` (cd/pm²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_PICOMETRE: Luminance =
        9.9999999999999998e+23 * CANDELA_PER_SQUARE_METRE;

    /// One `CandelaPerSquareNanometre` (cd/nm²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_NANOMETRE: Luminance = 1e+18 * CANDELA_PER_SQUARE_METRE;

    /// One `CandelaPerSquareMicrometre` (cd/µm²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_MICROMETRE: Luminance = 1000000000000.0 * CANDELA_PER_SQUARE_METRE;

    /// One `CandelaPerSquareMillimetre` (cd/mm²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_MILLIMETRE: Luminance = 1000000.0 * CANDELA_PER_SQUARE_METRE;

    /// One `CandelaPerSquareCentimetre` (cd/cm²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_CENTIMETRE: Luminance = 10000.0 * CANDELA_PER_SQUARE_METRE;

    /// One `CandelaPerSquareKilometre` (cd/km²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_KILOMETRE: Luminance =
        9.9999999999999995e-07 * CANDELA_PER_SQUARE_METRE;

    /// One `CandelaPerSquareMegametre` (cd/Mm²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_MEGAMETRE: Luminance =
        9.9999999999999998e-13 * CANDELA_PER_SQUARE_METRE;

    /// One `CandelaPerSquareGigametre` (cd/Gm²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_GIGAMETRE: Luminance =
        1.0000000000000001e-18 * CANDELA_PER_SQUARE_METRE;

    /// One `CandelaPerSquareTerametre` (cd/Tm²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_TERAMETRE: Luminance =
        9.9999999999999992e-25 * CANDELA_PER_SQUARE_METRE;

    /// One `CandelaPerSquareInch` (cd/in²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_INCH: Luminance = 1550.0031000062002 * CANDELA_PER_SQUARE_METRE;

    /// One `CandelaPerSquareFoot` (cd/ft²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_FOOT: Luminance = 10.763910416709722 * CANDELA_PER_SQUARE_METRE;

    /// One `Footlambert` (fl), as a `Luminance`.
    pub const FOOTLAMBERT: Luminance = 3.4262590996353905 * CANDELA_PER_SQUARE_METRE;

    /// One `Lambert` (la), as a `Luminance`.
    pub const LAMBERT: Luminance = 3183.0988618379065 * CANDELA_PER_SQUARE_METRE;

    /// One `Stilb` (sb), as a `Luminance`.
    pub const STILB: Luminance = 10000.0 * CANDELA_PER_SQUARE_METRE;

    /// One `Yottacandela` (Ycd), as a `LuminousIntensity`.
    pub const YOTTACANDELA: LuminousIntensity = 9.9999999999999998e+23 * CANDELA;

    /// One `Zettacandela` (Zcd), as a `LuminousIntensity`.
    pub const ZETTACANDELA: LuminousIntensity = 1e+21 * CANDELA;

    /// One `Exacandela` (Ecd), as a `LuminousIntensity`.
    pub const EXACANDELA: LuminousIntensity = 1e+18 * CANDELA;

    /// One `Petacandela` (Pcd), as a `LuminousIntensity`.
    pub const PETACANDELA: LuminousIntensity = 1000000000000000.0 * CANDELA;

    /// One `Teracandela` (Tcd), as a `LuminousIntensity`.
    pub const TERACANDELA: LuminousIntensity = 1000000000000.0 * CANDELA;

    /// One `Gigacandela` (Gcd), as a `LuminousIntensity`.
    pub const GIGACANDELA: LuminousIntensity = 1000000000.0 * CANDELA;

    /// One `Megacandela` (Mcd), as a `LuminousIntensity`.
    pub const MEGACANDELA: LuminousIntensity = 1000000.0 * CANDELA;

    /// One `Kilocandela` (kcd), as a `LuminousIntensity`.
    pub const KILOCANDELA: LuminousIntensity = 1000.0 * CANDELA;

    /// One `Hectocandela` (hcd), as a `LuminousIntensity`.
    pub const HECTOCANDELA: LuminousIntensity = 100.0 * CANDELA;

    /// One `Decacandela` (dacd), as a `LuminousIntensity`.
    pub const DECACANDELA: LuminousIntensity = 10.0 * CANDELA;

    /// One `Decicandela` (dcd), as a `LuminousIntensity`.
    pub const DECICANDELA: LuminousIntensity = 0.10000000000000001 * CANDELA;

    /// One `Centicandela` (ccd), as a `LuminousIntensity`.
    pub const CENTICANDELA: LuminousIntensity = 0.01 * CANDELA;

    /// One `Millicandela` (mcd), as a `LuminousIntensity`.
    pub const MILLICANDELA: LuminousIntensity = 0.001 * CANDELA;

    /// One `Microcandela` (µcd), as a `LuminousIntensity`.
    pub const MICROCANDELA: LuminousIntensity = 9.9999999999999995e-07 * CANDELA;

    /// One `Nanocandela` (ncd), as a `LuminousIntensity`.
    pub const NANOCANDELA: LuminousIntensity = 1.0000000000000001e-09 * CANDELA;

    /// One `Picocandela` (pcd), as a `LuminousIntensity`.
    pub const PICOCANDELA: LuminousIntensity = 9.9999999999999998e-13 * CANDELA;

    /// One `Femtocandela` (fcd), as a `LuminousIntensity`.
    pub const FEMTOCANDELA: LuminousIntensity = 1.0000000000000001e-15 * CANDELA;

    /// One `Attocandela` (acd), as a `LuminousIntensity`.
    pub const ATTOCANDELA: LuminousIntensity = 1.0000000000000001e-18 * CANDELA;

    /// One `Zeptocandela` (zcd), as a `LuminousIntensity`.
    pub const ZEPTOCANDELA: LuminousIntensity = 9.9999999999999991e-22 * CANDELA;

    /// One `Yoctocandela` (ycd), as a `LuminousIntensity`.
    pub const YOCTOCANDELA: LuminousIntensity = 9.9999999999999992e-25 * CANDELA;

    /// One `Yottalux` (Ylx), as an `Illuminance`.
    pub const YOTTALUX: Illuminance = 9.9999999999999998e+23 * LUX;

    /// One `Zettalux` (Zlx), as an `Illuminance`.
    pub const ZETTALUX: Illuminance = 1e+21 * LUX;

    /// One `Exalux` (Elx), as an `Illuminance`.
    pub const EXALUX: Illuminance = 1e+18 * LUX;

    /// One `Petalux` (Plx), as an `Illuminance`.
    pub const PETALUX: Illuminance = 1000000000000000.0 * LUX;

    /// One `Teralux` (Tlx), as an `Illuminance`.
    pub const TERALUX: Illuminance = 1000000000000.0 * LUX;

    /// One `Gigalux` (Glx), as an `Illuminance`.
    pub const GIGALUX: Illuminance = 1000000000.0 * LUX;

    /// One `Megalux` (Mlx), as an `Illuminance`.
    pub const MEGALUX: Illuminance = 1000000.0 * LUX;

    /// One `Kilolux` (klx), as an `Illuminance`.
    pub const KILOLUX: Illuminance = 1000.0 * LUX;

    /// One `Hectolux` (hlx), as an `Illuminance`.
    pub const HECTOLUX: Illuminance = 100.0 * LUX;

    /// One `Decalux` (dalx), as an `Illuminance`.
    pub const DECALUX: Illuminance = 10.0 * LUX;

    /// One `Lux` (lx), as an `Illuminance`.
    pub const LUX: Illuminance = CANDELA * RADIAN * RADIAN / (METRE * METRE);

    /// One `Decilux` (dlx), as an `Illuminance`.
    pub const DECILUX: Illuminance = 0.10000000000000001 * LUX;

    /// One `Centilux` (clx), as an `Illuminance`.
    pub const CENTILUX: Illuminance = 0.01 * LUX;

    /// One `Millilux` (mlx), as an `Illuminance`.
    pub const MILLILUX: Illuminance = 0.001 * LUX;

    /// One `Microlux` (µlx), as an `Illuminance`.
    pub const MICROLUX: Illuminance = 9.9999999999999995e-07 * LUX;

    /// One `Nanolux` (nlx), as an `Illuminance`.
    pub const NANOLUX: Illuminance = 1.0000000000000001e-09 * LUX;

    /// One `Picolux` (plx), as an `Illuminance`.
    pub const PICOLUX: Illuminance = 9.9999999999999998e-13 * LUX;

    /// One `Femtolux` (flx), as an `Illuminance`.
    pub const FEMTOLUX: Illuminance = 1.0000000000000001e-15 * LUX;

    /// One `Attolux` (alx), as an `Illuminance`.
    pub const ATTOLUX: Illuminance = 1.0000000000000001e-18 * LUX;

    /// One `Zeptolux` (zlx), as an `Illuminance`.
    pub const ZEPTOLUX: Illuminance = 9.9999999999999991e-22 * LUX;

    /// One `Yoctolux` (ylx), as an `Illuminance`.
    pub const YOCTOLUX: Illuminance = 9.9999999999999992e-25 * LUX;

    /// One `Phot` (ph), as an `Illuminance`.
    pub const PHOT: Illuminance = 10000.0 * LUX;

    /// One `Footcandle` (fc), as an `Illuminance`.
    pub const FOOTCANDLE: Illuminance = (1.0 / (0.3048 * 0.3048)) * LUX;

    /// One `YottajoulePerSquareMetre` (YJ/m²), as a `RadiantExposure`.
    pub const YOTTAJOULE_PER_SQUARE_METRE: RadiantExposure =
        9.9999999999999998e+23 * JOULE_PER_SQUARE_METRE;

    /// One `ZettajoulePerSquareMetre` (ZJ/m²), as a `RadiantExposure`.
    pub const ZETTAJOULE_PER_SQUARE_METRE: RadiantExposure = 1e+21 * JOULE_PER_SQUARE_METRE;

    /// One `ExajoulePerSquareMetre` (EJ/m²), as a `RadiantExposure`.
    pub const EXAJOULE_PER_SQUARE_METRE: RadiantExposure = 1e+18 * JOULE_PER_SQUARE_METRE;

    /// One `PetajoulePerSquareMetre` (PJ/m²), as a `RadiantExposure`.
    pub const PETAJOULE_PER_SQUARE_METRE: RadiantExposure =
        1000000000000000.0 * JOULE_PER_SQUARE_METRE;

    /// One `TerajoulePerSquareMetre` (TJ/m²), as a `RadiantExposure`.
    pub const TERAJOULE_PER_SQUARE_METRE: RadiantExposure =
        1000000000000.0 * JOULE_PER_SQUARE_METRE;

    /// One `GigajoulePerSquareMetre` (GJ/m²), as a `RadiantExposure`.
    pub const GIGAJOULE_PER_SQUARE_METRE: RadiantExposure = 1000000000.0 * JOULE_PER_SQUARE_METRE;

    /// One `MegajoulePerSquareMetre` (MJ/m²), as a `RadiantExposure`.
    pub const MEGAJOULE_PER_SQUARE_METRE: RadiantExposure = 1000000.0 * JOULE_PER_SQUARE_METRE;

    /// One `KilojoulePerSquareMetre` (kJ/m²), as a `RadiantExposure`.
    pub const KILOJOULE_PER_SQUARE_METRE: RadiantExposure = 1000.0 * JOULE_PER_SQUARE_METRE;

    /// One `HectojoulePerSquareMetre` (hJ/m²), as a `RadiantExposure`.
    pub const HECTOJOULE_PER_SQUARE_METRE: RadiantExposure = 100.0 * JOULE_PER_SQUARE_METRE;

    /// One `DecajoulePerSquareMetre` (daJ/m²), as a `RadiantExposure`.
    pub const DECAJOULE_PER_SQUARE_METRE: RadiantExposure = 10.0 * JOULE_PER_SQUARE_METRE;

    /// One `JoulePerSquareMetre` (J/m²), as a `RadiantExposure`.
    pub const JOULE_PER_SQUARE_METRE: RadiantExposure = KILOGRAM / (SECOND * SECOND);

    /// One `DecijoulePerSquareMetre` (dJ/m²), as a `RadiantExposure`.
    pub const DECIJOULE_PER_SQUARE_METRE: RadiantExposure =
        0.10000000000000001 * JOULE_PER_SQUARE_METRE;

    /// One `CentijoulePerSquareMetre` (cJ/m²), as a `RadiantExposure`.
    pub const CENTIJOULE_PER_SQUARE_METRE: RadiantExposure = 0.01 * JOULE_PER_SQUARE_METRE;

    /// One `MillijoulePerSquareMetre` (mJ/m²), as a `RadiantExposure`.
    pub const MILLIJOULE_PER_SQUARE_METRE: RadiantExposure = 0.001 * JOULE_PER_SQUARE_METRE;

    /// One `MicrojoulePerSquareMetre` (µJ/m²), as a `RadiantExposure`.
    pub const MICROJOULE_PER_SQUARE_METRE: RadiantExposure =
        9.9999999999999995e-07 * JOULE_PER_SQUARE_METRE;

    /// One `NanojoulePerSquareMetre` (nJ/m²), as a `RadiantExposure`.
    pub const NANOJOULE_PER_SQUARE_METRE: RadiantExposure =
        1.0000000000000001e-09 * JOULE_PER_SQUARE_METRE;

    /// One `PicojoulePerSquareMetre` (pJ/m²), as a `RadiantExposure`.
    pub const PICOJOULE_PER_SQUARE_METRE: RadiantExposure =
        9.9999999999999998e-13 * JOULE_PER_SQUARE_METRE;

    /// One `FemtojoulePerSquareMetre` (fJ/m²), as a `RadiantExposure`.
    pub const FEMTOJOULE_PER_SQUARE_METRE: RadiantExposure =
        1.0000000000000001e-15 * JOULE_PER_SQUARE_METRE;

    /// One `AttojoulePerSquareMetre` (aJ/m²), as a `RadiantExposure`.
    pub const ATTOJOULE_PER_SQUARE_METRE: RadiantExposure =
        1.0000000000000001e-18 * JOULE_PER_SQUARE_METRE;

    /// One `ZeptojoulePerSquareMetre` (zJ/m²), as a `RadiantExposure`.
    pub const ZEPTOJOULE_PER_SQUARE_METRE: RadiantExposure =
        9.9999999999999991e-22 * JOULE_PER_SQUARE_METRE;

    /// One `YoctojoulePerSquareMetre` (yJ/m²), as a `RadiantExposure`.
    pub const YOCTOJOULE_PER_SQUARE_METRE: RadiantExposure =
        9.9999999999999992e-25 * JOULE_PER_SQUARE_METRE;

    /// One `WattHourPerSquareMetre` (W · h/m²), as a `RadiantExposure`.
    pub const WATT_HOUR_PER_SQUARE_METRE: RadiantExposure = 3600.0 * JOULE_PER_SQUARE_METRE;

    /// One `GigajoulePerSquareCentimetre` (GJ/cm²), as a `RadiantExposure`.
    pub const GIGAJOULE_PER_SQUARE_CENTIMETRE: RadiantExposure =
        10000000000000.0 * JOULE_PER_SQUARE_METRE;

    /// One `MegajoulePerSquareCentimetre` (MJ/cm²), as a `RadiantExposure`.
    pub const MEGAJOULE_PER_SQUARE_CENTIMETRE: RadiantExposure =
        10000000000.0 * JOULE_PER_SQUARE_METRE;

    /// One `KilojoulePerSquareCentimetre` (kJ/cm²), as a `RadiantExposure`.
    pub const KILOJOULE_PER_SQUARE_CENTIMETRE: RadiantExposure =
        10000000.0 * JOULE_PER_SQUARE_METRE;

    /// One `HectojoulePerSquareCentimetre` (hJ/cm²), as a `RadiantExposure`.
    pub const HECTOJOULE_PER_SQUARE_CENTIMETRE: RadiantExposure =
        1000000.0 * JOULE_PER_SQUARE_METRE;

    /// One `DecajoulePerSquareCentimetre` (daJ/cm²), as a `RadiantExposure`.
    pub const DECAJOULE_PER_SQUARE_CENTIMETRE: RadiantExposure = 100000.0 * JOULE_PER_SQUARE_METRE;

    /// One `JoulePerSquareCentimetre` (J/cm²), as a `RadiantExposure`.
    pub const JOULE_PER_SQUARE_CENTIMETRE: RadiantExposure = 10000.0 * JOULE_PER_SQUARE_METRE;

    /// One `DecijoulePerSquareCentimetre` (dJ/cm²), as a `RadiantExposure`.
    pub const DECIJOULE_PER_SQUARE_CENTIMETRE: RadiantExposure = 1000.0 * JOULE_PER_SQUARE_METRE;

    /// One `CentijoulePerSquareCentimetre` (cJ/cm²), as a `RadiantExposure`.
    pub const CENTIJOULE_PER_SQUARE_CENTIMETRE: RadiantExposure = 100.0 * JOULE_PER_SQUARE_METRE;

    /// One `MillijoulePerSquareCentimetre` (mJ/cm²), as a `RadiantExposure`.
    pub const MILLIJOULE_PER_SQUARE_CENTIMETRE: RadiantExposure = 10.0 * JOULE_PER_SQUARE_METRE;

    /// One `MicrojoulePerSquareCentimetre` (µJ/cm²), as a `RadiantExposure`.
    pub const MICROJOULE_PER_SQUARE_CENTIMETRE: RadiantExposure = 0.01 * JOULE_PER_SQUARE_METRE;

    /// One `NanojoulePerSquareCentimetre` (nJ/cm²), as a `RadiantExposure`.
    pub const NANOJOULE_PER_SQUARE_CENTIMETRE: RadiantExposure =
        1.0000000000000001e-05 * JOULE_PER_SQUARE_METRE;

    /// One `GigajoulePerSquareMillimetre` (GJ/mm²), as a `RadiantExposure`.
    pub const GIGAJOULE_PER_SQUARE_MILLIMETRE: RadiantExposure =
        1000000000000000.0 * JOULE_PER_SQUARE_METRE;

    /// One `MegajoulePerSquareMillimetre` (MJ/mm²), as a `RadiantExposure`.
    pub const MEGAJOULE_PER_SQUARE_MILLIMETRE: RadiantExposure =
        1000000000000.0 * JOULE_PER_SQUARE_METRE;

    /// One `KilojoulePerSquareMillimetre` (kJ/mm²), as a `RadiantExposure`.
    pub const KILOJOULE_PER_SQUARE_MILLIMETRE: RadiantExposure =
        1000000000.0 * JOULE_PER_SQUARE_METRE;

    /// One `HectojoulePerSquareMillimetre` (hJ/mm²), as a `RadiantExposure`.
    pub const HECTOJOULE_PER_SQUARE_MILLIMETRE: RadiantExposure =
        100000000.0 * JOULE_PER_SQUARE_METRE;

    /// One `DecajoulePerSquareMillimetre` (daJ/mm²), as a `RadiantExposure`.
    pub const DECAJOULE_PER_SQUARE_MILLIMETRE: RadiantExposure =
        10000000.0 * JOULE_PER_SQUARE_METRE;

    /// One `JoulePerSquareMillimetre` (J/mm²), as a `RadiantExposure`.
    pub const JOULE_PER_SQUARE_MILLIMETRE: RadiantExposure = 1000000.0 * JOULE_PER_SQUARE_METRE;

    /// One `DecijoulePerSquareMillimetre` (dJ/mm²), as a `RadiantExposure`.
    pub const DECIJOULE_PER_SQUARE_MILLIMETRE: RadiantExposure = 100000.0 * JOULE_PER_SQUARE_METRE;

    /// One `CentijoulePerSquareMillimetre` (cJ/mm²), as a `RadiantExposure`.
    pub const CENTIJOULE_PER_SQUARE_MILLIMETRE: RadiantExposure = 10000.0 * JOULE_PER_SQUARE_METRE;

    /// One `MillijoulePerSquareMillimetre` (mJ/mm²), as a `RadiantExposure`.
    pub const MILLIJOULE_PER_SQUARE_MILLIMETRE: RadiantExposure = 1000.0 * JOULE_PER_SQUARE_METRE;

    /// One `MicrojoulePerSquareMillimetre` (µJ/mm²), as a `RadiantExposure`.
    pub const MICROJOULE_PER_SQUARE_MILLIMETRE: RadiantExposure = JOULE_PER_SQUARE_METRE;

    /// One `NanojoulePerSquareMillimetre` (nJ/mm²), as a `RadiantExposure`.
    pub const NANOJOULE_PER_SQUARE_MILLIMETRE: RadiantExposure = 0.001 * JOULE_PER_SQUARE_METRE;
}

/// The units of `Luminance`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Luminance)]
pub enum LuminanceUnit {
    #[unit("Ycd/m²", "yottacandelas per square metre", by = YOTTACANDELA_PER_SQUARE_METRE)]
    YottacandelaPerSquareMetre,
    #[unit("Zcd/m²", "zettacandelas per square metre", by = ZETTACANDELA_PER_SQUARE_METRE)]
    ZettacandelaPerSquareMetre,
    #[unit("Ecd/m²", "exacandelas per square metre", by = EXACANDELA_PER_SQUARE_METRE)]
    ExacandelaPerSquareMetre,
    #[unit("Pcd/m²", "petacandelas per square metre", by = PETACANDELA_PER_SQUARE_METRE)]
    PetacandelaPerSquareMetre,
    #[unit("Tcd/m²", "teracandelas per square metre", by = TERACANDELA_PER_SQUARE_METRE)]
    TeracandelaPerSquareMetre,
    #[unit("Gcd/m²", "gigacandelas per square metre", by = GIGACANDELA_PER_SQUARE_METRE)]
    GigacandelaPerSquareMetre,
    #[unit("Mcd/m²", "megacandelas per square metre", by = MEGACANDELA_PER_SQUARE_METRE)]
    MegacandelaPerSquareMetre,
    #[unit("kcd/m²", "kilocandelas per square metre", by = KILOCANDELA_PER_SQUARE_METRE)]
    KilocandelaPerSquareMetre,
    #[unit("hcd/m²", "hectocandelas per square metre", by = HECTOCANDELA_PER_SQUARE_METRE)]
    HectocandelaPerSquareMetre,
    #[unit("dacd/m²", "decacandelas per square metre", by = DECACANDELA_PER_SQUARE_METRE)]
    DecacandelaPerSquareMetre,
    #[default]
    #[unit("cd/m²", "candelas per square metre", by = CANDELA_PER_SQUARE_METRE)]
    CandelaPerSquareMetre,
    #[unit("dcd/m²", "decicandelas per square metre", by = DECICANDELA_PER_SQUARE_METRE)]
    DecicandelaPerSquareMetre,
    #[unit("ccd/m²", "centicandelas per square metre", by = CENTICANDELA_PER_SQUARE_METRE)]
    CenticandelaPerSquareMetre,
    #[unit("mcd/m²", "millicandelas per square metre", by = MILLICANDELA_PER_SQUARE_METRE)]
    MillicandelaPerSquareMetre,
    #[unit("µcd/m²", "microcandelas per square metre", by = MICROCANDELA_PER_SQUARE_METRE)]
    MicrocandelaPerSquareMetre,
    #[unit("ncd/m²", "nanocandelas per square metre", by = NANOCANDELA_PER_SQUARE_METRE)]
    NanocandelaPerSquareMetre,
    #[unit("pcd/m²", "picocandelas per square metre", by = PICOCANDELA_PER_SQUARE_METRE)]
    PicocandelaPerSquareMetre,
    #[unit("fcd/m²", "femtocandelas per square metre", by = FEMTOCANDELA_PER_SQUARE_METRE)]
    FemtocandelaPerSquareMetre,
    #[unit("acd/m²", "attocandelas per square metre", by = ATTOCANDELA_PER_SQUARE_METRE)]
    AttocandelaPerSquareMetre,
    #[unit("zcd/m²", "zeptocandelas per square metre", by = ZEPTOCANDELA_PER_SQUARE_METRE)]
    ZeptocandelaPerSquareMetre,
    #[unit("ycd/m²", "yoctocandelas per square metre", by = YOCTOCANDELA_PER_SQUARE_METRE)]
    YoctocandelaPerSquareMetre,
    #[unit("cd/pm²", "candelas per square picometre", by = CANDELA_PER_SQUARE_PICOMETRE)]
    CandelaPerSquarePicometre,
    #[unit("cd/nm²", "candelas per square nanometre", by = CANDELA_PER_SQUARE_NANOMETRE)]
    CandelaPerSquareNanometre,
    #[unit("cd/µm²", "candelas per square micrometre", by = CANDELA_PER_SQUARE_MICROMETRE)]
    CandelaPerSquareMicrometre,
    #[unit("cd/mm²", "candelas per square millimetre", by = CANDELA_PER_SQUARE_MILLIMETRE)]
    CandelaPerSquareMillimetre,
    #[unit("cd/cm²", "candelas per square centimetre", by = CANDELA_PER_SQUARE_CENTIMETRE)]
    CandelaPerSquareCentimetre,
    #[unit("cd/km²", "candelas per square kilometre", by = CANDELA_PER_SQUARE_KILOMETRE)]
    CandelaPerSquareKilometre,
    #[unit("cd/Mm²", "candelas per square megametre", by = CANDELA_PER_SQUARE_MEGAMETRE)]
    CandelaPerSquareMegametre,
    #[unit("cd/Gm²", "candelas per square gigametre", by = CANDELA_PER_SQUARE_GIGAMETRE)]
    CandelaPerSquareGigametre,
    #[unit("cd/Tm²", "candelas per square terametre", by = CANDELA_PER_SQUARE_TERAMETRE)]
    CandelaPerSquareTerametre,
    #[unit("cd/in²", "candelas per square inch", by = CANDELA_PER_SQUARE_INCH)]
    CandelaPerSquareInch,
    #[unit("cd/ft²", "candelas per square foot", by = CANDELA_PER_SQUARE_FOOT)]
    CandelaPerSquareFoot,
    #[unit("fl", "footlamberts", by = FOOTLAMBERT)]
    Footlambert,
    #[unit("la", "lamberts", by = LAMBERT)]
    Lambert,
    #[unit("sb", "stilbs", by = STILB)]
    Stilb,
}

/// The units of `LuminousIntensity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LuminousIntensity)]
pub enum LuminousIntensityUnit {
    #[unit("Ycd", "yottacandelas", by = YOTTACANDELA)]
    Yottacandela,
    #[unit("Zcd", "zettacandelas", by = ZETTACANDELA)]
    Zettacandela,
    #[unit("Ecd", "exacandelas", by = EXACANDELA)]
    Exacandela,
    #[unit("Pcd", "petacandelas", by = PETACANDELA)]
    Petacandela,
    #[unit("Tcd", "teracandelas", by = TERACANDELA)]
    Teracandela,
    #[unit("Gcd", "gigacandelas", by = GIGACANDELA)]
    Gigacandela,
    #[unit("Mcd", "megacandelas", by = MEGACANDELA)]
    Megacandela,
    #[unit("kcd", "kilocandelas", by = KILOCANDELA)]
    Kilocandela,
    #[unit("hcd", "hectocandelas", by = HECTOCANDELA)]
    Hectocandela,
    #[unit("dacd", "decacandelas", by = DECACANDELA)]
    Decacandela,
    #[default]
    #[unit("cd", "candelas", by = CANDELA)]
    Candela,
    #[unit("dcd", "decicandelas", by = DECICANDELA)]
    Decicandela,
    #[unit("ccd", "centicandelas", by = CENTICANDELA)]
    Centicandela,
    #[unit("mcd", "millicandelas", by = MILLICANDELA)]
    Millicandela,
    #[unit("µcd", "microcandelas", by = MICROCANDELA)]
    Microcandela,
    #[unit("ncd", "nanocandelas", by = NANOCANDELA)]
    Nanocandela,
    #[unit("pcd", "picocandelas", by = PICOCANDELA)]
    Picocandela,
    #[unit("fcd", "femtocandelas", by = FEMTOCANDELA)]
    Femtocandela,
    #[unit("acd", "attocandelas", by = ATTOCANDELA)]
    Attocandela,
    #[unit("zcd", "zeptocandelas", by = ZEPTOCANDELA)]
    Zeptocandela,
    #[unit("ycd", "yoctocandelas", by = YOCTOCANDELA)]
    Yoctocandela,
}

/// The units of `Illuminance`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Illuminance)]
pub enum IlluminanceUnit {
    #[unit("Ylx", "yottalux", by = YOTTALUX)]
    Yottalux,
    #[unit("Zlx", "zettalux", by = ZETTALUX)]
    Zettalux,
    #[unit("Elx", "exalux", by = EXALUX)]
    Exalux,
    #[unit("Plx", "petalux", by = PETALUX)]
    Petalux,
    #[unit("Tlx", "teralux", by = TERALUX)]
    Teralux,
    #[unit("Glx", "gigalux", by = GIGALUX)]
    Gigalux,
    #[unit("Mlx", "megalux", by = MEGALUX)]
    Megalux,
    #[unit("klx", "kilolux", by = KILOLUX)]
    Kilolux,
    #[unit("hlx", "hectolux", by = HECTOLUX)]
    Hectolux,
    #[unit("dalx", "decalux", by = DECALUX)]
    Decalux,
    #[default]
    #[unit("lx", "lux", by = LUX)]
    Lux,
    #[unit("dlx", "decilux", by = DECILUX)]
    Decilux,
    #[unit("clx", "centilux", by = CENTILUX)]
    Centilux,
    #[unit("mlx", "millilux", by = MILLILUX)]
    Millilux,
    #[unit("µlx", "microlux", by = MICROLUX)]
    Microlux,
    #[unit("nlx", "nanolux", by = NANOLUX)]
    Nanolux,
    #[unit("plx", "picolux", by = PICOLUX)]
    Picolux,
    #[unit("flx", "femtolux", by = FEMTOLUX)]
    Femtolux,
    #[unit("alx", "attolux", by = ATTOLUX)]
    Attolux,
    #[unit("zlx", "zeptolux", by = ZEPTOLUX)]
    Zeptolux,
    #[unit("ylx", "yoctolux", by = YOCTOLUX)]
    Yoctolux,
    #[unit("ph", "phot", by = PHOT)]
    Phot,
    #[unit("fc", "footcandles", by = FOOTCANDLE)]
    Footcandle,
}

/// The units of `RadiantExposure`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = RadiantExposure)]
pub enum RadiantExposureUnit {
    #[unit("YJ/m²", "yottajoules per square metre", by = YOTTAJOULE_PER_SQUARE_METRE)]
    YottajoulePerSquareMetre,
    #[unit("ZJ/m²", "zettajoules per square metre", by = ZETTAJOULE_PER_SQUARE_METRE)]
    ZettajoulePerSquareMetre,
    #[unit("EJ/m²", "exajoules per square metre", by = EXAJOULE_PER_SQUARE_METRE)]
    ExajoulePerSquareMetre,
    #[unit("PJ/m²", "petajoules per square metre", by = PETAJOULE_PER_SQUARE_METRE)]
    PetajoulePerSquareMetre,
    #[unit("TJ/m²", "terajoules per square metre", by = TERAJOULE_PER_SQUARE_METRE)]
    TerajoulePerSquareMetre,
    #[unit("GJ/m²", "gigajoules per square metre", by = GIGAJOULE_PER_SQUARE_METRE)]
    GigajoulePerSquareMetre,
    #[unit("MJ/m²", "megajoules per square metre", by = MEGAJOULE_PER_SQUARE_METRE)]
    MegajoulePerSquareMetre,
    #[unit("kJ/m²", "kilojoules per square metre", by = KILOJOULE_PER_SQUARE_METRE)]
    KilojoulePerSquareMetre,
    #[unit("hJ/m²", "hectojoules per square metre", by = HECTOJOULE_PER_SQUARE_METRE)]
    HectojoulePerSquareMetre,
    #[unit("daJ/m²", "decajoules per square metre", by = DECAJOULE_PER_SQUARE_METRE)]
    DecajoulePerSquareMetre,
    #[default]
    #[unit("J/m²", "joules per square metre", by = JOULE_PER_SQUARE_METRE)]
    JoulePerSquareMetre,
    #[unit("dJ/m²", "decijoules per square metre", by = DECIJOULE_PER_SQUARE_METRE)]
    DecijoulePerSquareMetre,
    #[unit("cJ/m²", "centijoules per square metre", by = CENTIJOULE_PER_SQUARE_METRE)]
    CentijoulePerSquareMetre,
    #[unit("mJ/m²", "millijoules per square metre", by = MILLIJOULE_PER_SQUARE_METRE)]
    MillijoulePerSquareMetre,
    #[unit("µJ/m²", "microjoules per square metre", by = MICROJOULE_PER_SQUARE_METRE)]
    MicrojoulePerSquareMetre,
    #[unit("nJ/m²", "nanojoules per square metre", by = NANOJOULE_PER_SQUARE_METRE)]
    NanojoulePerSquareMetre,
    #[unit("pJ/m²", "picojoules per square metre", by = PICOJOULE_PER_SQUARE_METRE)]
    PicojoulePerSquareMetre,
    #[unit("fJ/m²", "femtojoules per square metre", by = FEMTOJOULE_PER_SQUARE_METRE)]
    FemtojoulePerSquareMetre,
    #[unit("aJ/m²", "attojoules per square metre", by = ATTOJOULE_PER_SQUARE_METRE)]
    AttojoulePerSquareMetre,
    #[unit("zJ/m²", "zeptojoules per square metre", by = ZEPTOJOULE_PER_SQUARE_METRE)]
    ZeptojoulePerSquareMetre,
    #[unit("yJ/m²", "yoctojoules per square metre", by = YOCTOJOULE_PER_SQUARE_METRE)]
    YoctojoulePerSquareMetre,
    #[unit("W · h/m²", "watt hours per square metre", by = WATT_HOUR_PER_SQUARE_METRE)]
    WattHourPerSquareMetre,
    #[unit("GJ/cm²", "gigajoules per square centimetre", by = GIGAJOULE_PER_SQUARE_CENTIMETRE)]
    GigajoulePerSquareCentimetre,
    #[unit("MJ/cm²", "megajoules per square centimetre", by = MEGAJOULE_PER_SQUARE_CENTIMETRE)]
    MegajoulePerSquareCentimetre,
    #[unit("kJ/cm²", "kilojoules per square centimetre", by = KILOJOULE_PER_SQUARE_CENTIMETRE)]
    KilojoulePerSquareCentimetre,
    #[unit("hJ/cm²", "hectojoules per square centimetre", by = HECTOJOULE_PER_SQUARE_CENTIMETRE)]
    HectojoulePerSquareCentimetre,
    #[unit("daJ/cm²", "decajoules per square centimetre", by = DECAJOULE_PER_SQUARE_CENTIMETRE)]
    DecajoulePerSquareCentimetre,
    #[unit("J/cm²", "joules per square centimetre", by = JOULE_PER_SQUARE_CENTIMETRE)]
    JoulePerSquareCentimetre,
    #[unit("dJ/cm²", "decijoules per square centimetre", by = DECIJOULE_PER_SQUARE_CENTIMETRE)]
    DecijoulePerSquareCentimetre,
    #[unit("cJ/cm²", "centijoules per square centimetre", by = CENTIJOULE_PER_SQUARE_CENTIMETRE)]
    CentijoulePerSquareCentimetre,
    #[unit("mJ/cm²", "millijoules per square centimetre", by = MILLIJOULE_PER_SQUARE_CENTIMETRE)]
    MillijoulePerSquareCentimetre,
    #[unit("µJ/cm²", "microjoules per square centimetre", by = MICROJOULE_PER_SQUARE_CENTIMETRE)]
    MicrojoulePerSquareCentimetre,
    #[unit("nJ/cm²", "nanojoules per square centimetre", by = NANOJOULE_PER_SQUARE_CENTIMETRE)]
    NanojoulePerSquareCentimetre,
    #[unit("GJ/mm²", "gigajoules per square millimetre", by = GIGAJOULE_PER_SQUARE_MILLIMETRE)]
    GigajoulePerSquareMillimetre,
    #[unit("MJ/mm²", "megajoules per square millimetre", by = MEGAJOULE_PER_SQUARE_MILLIMETRE)]
    MegajoulePerSquareMillimetre,
    #[unit("kJ/mm²", "kilojoules per square millimetre", by = KILOJOULE_PER_SQUARE_MILLIMETRE)]
    KilojoulePerSquareMillimetre,
    #[unit("hJ/mm²", "hectojoules per square millimetre", by = HECTOJOULE_PER_SQUARE_MILLIMETRE)]
    HectojoulePerSquareMillimetre,
    #[unit("daJ/mm²", "decajoules per square millimetre", by = DECAJOULE_PER_SQUARE_MILLIMETRE)]
    DecajoulePerSquareMillimetre,
    #[unit("J/mm²", "joules per square millimetre", by = JOULE_PER_SQUARE_MILLIMETRE)]
    JoulePerSquareMillimetre,
    #[unit("dJ/mm²", "decijoules per square millimetre", by = DECIJOULE_PER_SQUARE_MILLIMETRE)]
    DecijoulePerSquareMillimetre,
    #[unit("cJ/mm²", "centijoules per square millimetre", by = CENTIJOULE_PER_SQUARE_MILLIMETRE)]
    CentijoulePerSquareMillimetre,
    #[unit("mJ/mm²", "millijoules per square millimetre", by = MILLIJOULE_PER_SQUARE_MILLIMETRE)]
    MillijoulePerSquareMillimetre,
    #[unit("µJ/mm²", "microjoules per square millimetre", by = MICROJOULE_PER_SQUARE_MILLIMETRE)]
    MicrojoulePerSquareMillimetre,
    #[unit("nJ/mm²", "nanojoules per square millimetre", by = NANOJOULE_PER_SQUARE_MILLIMETRE)]
    NanojoulePerSquareMillimetre,
}
