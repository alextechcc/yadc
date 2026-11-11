use crate::{Exact, ISQ, SIQuantity, counted};
use consts::*;
use yadc::Dimension;

pub type Luminance<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            cd: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type LuminousIntensity<N> = SIQuantity<
    {
        ISQ {
            cd: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Illuminance<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            cd: 1,
            rad: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type RadiantExposure<N> = SIQuantity<
    {
        ISQ {
            kg: 1,
            s: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

/// The constants of this module: the base constants of its quantities
/// (re-exported), then one per multiple, each defined exactly from them -
/// `KILOMETRE` is `1000 * METRE` - and counted in the number type its use
/// asks for, rounded once: `let d: Length = KILOMETRE;` is an `f64` length,
/// `Length<f32>` an `f32` one, `KILOMETRE::<Exact>` the exact value. Each is
/// a unit (`KILOMETRE(3.0)`, `KILOMETRE.value(d)`), and they compose in
/// `const` items: `const LIMIT: Velocity = 50.0 * KILOMETRE / HOUR;`.
pub mod consts {
    use super::*;
    use crate::base::{KILOGRAM, METRE, RADIAN, SECOND};

    /// The candela, the SI base unit of luminous intensity.
    pub use crate::base::CANDELA;

    /// One `YottacandelaPerSquareMetre` (Ycd/m²), as a `Luminance`.
    pub const YOTTACANDELA_PER_SQUARE_METRE<N: const From<Exact>>: Luminance<N> = counted(Exact::int(1000000000000000000000000) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `ZettacandelaPerSquareMetre` (Zcd/m²), as a `Luminance`.
    pub const ZETTACANDELA_PER_SQUARE_METRE<N: const From<Exact>>: Luminance<N> = counted(Exact::int(1000000000000000000000) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `ExacandelaPerSquareMetre` (Ecd/m²), as a `Luminance`.
    pub const EXACANDELA_PER_SQUARE_METRE<N: const From<Exact>>: Luminance<N> = counted(Exact::int(1000000000000000000) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `PetacandelaPerSquareMetre` (Pcd/m²), as a `Luminance`.
    pub const PETACANDELA_PER_SQUARE_METRE<N: const From<Exact>>: Luminance<N> = counted(Exact::int(1000000000000000) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `TeracandelaPerSquareMetre` (Tcd/m²), as a `Luminance`.
    pub const TERACANDELA_PER_SQUARE_METRE<N: const From<Exact>>: Luminance<N> = counted(Exact::int(1000000000000) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `GigacandelaPerSquareMetre` (Gcd/m²), as a `Luminance`.
    pub const GIGACANDELA_PER_SQUARE_METRE<N: const From<Exact>>: Luminance<N> = counted(Exact::int(1000000000) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `MegacandelaPerSquareMetre` (Mcd/m²), as a `Luminance`.
    pub const MEGACANDELA_PER_SQUARE_METRE<N: const From<Exact>>: Luminance<N> = counted(Exact::int(1000000) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `KilocandelaPerSquareMetre` (kcd/m²), as a `Luminance`.
    pub const KILOCANDELA_PER_SQUARE_METRE<N: const From<Exact>>: Luminance<N> = counted(Exact::int(1000) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `HectocandelaPerSquareMetre` (hcd/m²), as a `Luminance`.
    pub const HECTOCANDELA_PER_SQUARE_METRE<N: const From<Exact>>: Luminance<N> = counted(Exact::int(100) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `DecacandelaPerSquareMetre` (dacd/m²), as a `Luminance`.
    pub const DECACANDELA_PER_SQUARE_METRE<N: const From<Exact>>: Luminance<N> = counted(Exact::int(10) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `CandelaPerSquareMetre` (cd/m²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_METRE<N: const From<Exact>>: Luminance<N> = counted(CANDELA::<Exact> / (METRE::<Exact> * METRE::<Exact>));

    /// One `DecicandelaPerSquareMetre` (dcd/m²), as a `Luminance`.
    pub const DECICANDELA_PER_SQUARE_METRE<N: const From<Exact>>: Luminance<N> = counted(Exact::dec(1, -1) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `CenticandelaPerSquareMetre` (ccd/m²), as a `Luminance`.
    pub const CENTICANDELA_PER_SQUARE_METRE<N: const From<Exact>>: Luminance<N> = counted(Exact::dec(1, -2) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `MillicandelaPerSquareMetre` (mcd/m²), as a `Luminance`.
    pub const MILLICANDELA_PER_SQUARE_METRE<N: const From<Exact>>: Luminance<N> = counted(Exact::dec(1, -3) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `MicrocandelaPerSquareMetre` (µcd/m²), as a `Luminance`.
    pub const MICROCANDELA_PER_SQUARE_METRE<N: const From<Exact>>: Luminance<N> = counted(Exact::dec(1, -6) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `NanocandelaPerSquareMetre` (ncd/m²), as a `Luminance`.
    pub const NANOCANDELA_PER_SQUARE_METRE<N: const From<Exact>>: Luminance<N> = counted(Exact::dec(1, -9) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `PicocandelaPerSquareMetre` (pcd/m²), as a `Luminance`.
    pub const PICOCANDELA_PER_SQUARE_METRE<N: const From<Exact>>: Luminance<N> = counted(Exact::dec(1, -12) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `FemtocandelaPerSquareMetre` (fcd/m²), as a `Luminance`.
    pub const FEMTOCANDELA_PER_SQUARE_METRE<N: const From<Exact>>: Luminance<N> = counted(Exact::dec(1, -15) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `AttocandelaPerSquareMetre` (acd/m²), as a `Luminance`.
    pub const ATTOCANDELA_PER_SQUARE_METRE<N: const From<Exact>>: Luminance<N> = counted(Exact::dec(1, -18) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `ZeptocandelaPerSquareMetre` (zcd/m²), as a `Luminance`.
    pub const ZEPTOCANDELA_PER_SQUARE_METRE<N: const From<Exact>>: Luminance<N> = counted(Exact::dec(1, -21) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `YoctocandelaPerSquareMetre` (ycd/m²), as a `Luminance`.
    pub const YOCTOCANDELA_PER_SQUARE_METRE<N: const From<Exact>>: Luminance<N> = counted(Exact::dec(1, -24) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `CandelaPerSquarePicometre` (cd/pm²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_PICOMETRE<N: const From<Exact>>: Luminance<N> = counted(Exact::int(1000000000000000000000000) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `CandelaPerSquareNanometre` (cd/nm²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_NANOMETRE<N: const From<Exact>>: Luminance<N> = counted(Exact::int(1000000000000000000) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `CandelaPerSquareMicrometre` (cd/µm²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_MICROMETRE<N: const From<Exact>>: Luminance<N> = counted(Exact::int(1000000000000) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `CandelaPerSquareMillimetre` (cd/mm²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_MILLIMETRE<N: const From<Exact>>: Luminance<N> = counted(Exact::int(1000000) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `CandelaPerSquareCentimetre` (cd/cm²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_CENTIMETRE<N: const From<Exact>>: Luminance<N> = counted(Exact::int(10000) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `CandelaPerSquareKilometre` (cd/km²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_KILOMETRE<N: const From<Exact>>: Luminance<N> = counted(Exact::dec(1, -6) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `CandelaPerSquareMegametre` (cd/Mm²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_MEGAMETRE<N: const From<Exact>>: Luminance<N> = counted(Exact::dec(1, -12) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `CandelaPerSquareGigametre` (cd/Gm²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_GIGAMETRE<N: const From<Exact>>: Luminance<N> = counted(Exact::dec(1, -18) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `CandelaPerSquareTerametre` (cd/Tm²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_TERAMETRE<N: const From<Exact>>: Luminance<N> = counted(Exact::dec(1, -24) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `CandelaPerSquareInch` (cd/in²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_INCH<N: const From<Exact>>: Luminance<N> = counted((Exact::int(1) / (Exact::dec(254, -4) * Exact::dec(254, -4))) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `CandelaPerSquareFoot` (cd/ft²), as a `Luminance`.
    pub const CANDELA_PER_SQUARE_FOOT<N: const From<Exact>>: Luminance<N> = counted((Exact::int(1) / (Exact::dec(3048, -4) * Exact::dec(3048, -4))) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `Footlambert` (fl), as a `Luminance`.
    pub const FOOTLAMBERT<N: const From<Exact>>: Luminance<N> = counted((Exact::int(1) / (Exact::PI * Exact::dec(3048, -4) * Exact::dec(3048, -4))) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `Lambert` (la), as a `Luminance`.
    pub const LAMBERT<N: const From<Exact>>: Luminance<N> = counted((Exact::int(10000) / Exact::PI) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `Stilb` (sb), as a `Luminance`.
    pub const STILB<N: const From<Exact>>: Luminance<N> = counted(Exact::int(10000) * CANDELA_PER_SQUARE_METRE::<Exact>);

    /// One `Yottacandela` (Ycd), as a `LuminousIntensity`.
    pub const YOTTACANDELA<N: const From<Exact>>: LuminousIntensity<N> = counted(Exact::int(1000000000000000000000000) * CANDELA::<Exact>);

    /// One `Zettacandela` (Zcd), as a `LuminousIntensity`.
    pub const ZETTACANDELA<N: const From<Exact>>: LuminousIntensity<N> = counted(Exact::int(1000000000000000000000) * CANDELA::<Exact>);

    /// One `Exacandela` (Ecd), as a `LuminousIntensity`.
    pub const EXACANDELA<N: const From<Exact>>: LuminousIntensity<N> = counted(Exact::int(1000000000000000000) * CANDELA::<Exact>);

    /// One `Petacandela` (Pcd), as a `LuminousIntensity`.
    pub const PETACANDELA<N: const From<Exact>>: LuminousIntensity<N> = counted(Exact::int(1000000000000000) * CANDELA::<Exact>);

    /// One `Teracandela` (Tcd), as a `LuminousIntensity`.
    pub const TERACANDELA<N: const From<Exact>>: LuminousIntensity<N> = counted(Exact::int(1000000000000) * CANDELA::<Exact>);

    /// One `Gigacandela` (Gcd), as a `LuminousIntensity`.
    pub const GIGACANDELA<N: const From<Exact>>: LuminousIntensity<N> = counted(Exact::int(1000000000) * CANDELA::<Exact>);

    /// One `Megacandela` (Mcd), as a `LuminousIntensity`.
    pub const MEGACANDELA<N: const From<Exact>>: LuminousIntensity<N> = counted(Exact::int(1000000) * CANDELA::<Exact>);

    /// One `Kilocandela` (kcd), as a `LuminousIntensity`.
    pub const KILOCANDELA<N: const From<Exact>>: LuminousIntensity<N> = counted(Exact::int(1000) * CANDELA::<Exact>);

    /// One `Hectocandela` (hcd), as a `LuminousIntensity`.
    pub const HECTOCANDELA<N: const From<Exact>>: LuminousIntensity<N> = counted(Exact::int(100) * CANDELA::<Exact>);

    /// One `Decacandela` (dacd), as a `LuminousIntensity`.
    pub const DECACANDELA<N: const From<Exact>>: LuminousIntensity<N> = counted(Exact::int(10) * CANDELA::<Exact>);

    /// One `Decicandela` (dcd), as a `LuminousIntensity`.
    pub const DECICANDELA<N: const From<Exact>>: LuminousIntensity<N> = counted(Exact::dec(1, -1) * CANDELA::<Exact>);

    /// One `Centicandela` (ccd), as a `LuminousIntensity`.
    pub const CENTICANDELA<N: const From<Exact>>: LuminousIntensity<N> = counted(Exact::dec(1, -2) * CANDELA::<Exact>);

    /// One `Millicandela` (mcd), as a `LuminousIntensity`.
    pub const MILLICANDELA<N: const From<Exact>>: LuminousIntensity<N> = counted(Exact::dec(1, -3) * CANDELA::<Exact>);

    /// One `Microcandela` (µcd), as a `LuminousIntensity`.
    pub const MICROCANDELA<N: const From<Exact>>: LuminousIntensity<N> = counted(Exact::dec(1, -6) * CANDELA::<Exact>);

    /// One `Nanocandela` (ncd), as a `LuminousIntensity`.
    pub const NANOCANDELA<N: const From<Exact>>: LuminousIntensity<N> = counted(Exact::dec(1, -9) * CANDELA::<Exact>);

    /// One `Picocandela` (pcd), as a `LuminousIntensity`.
    pub const PICOCANDELA<N: const From<Exact>>: LuminousIntensity<N> = counted(Exact::dec(1, -12) * CANDELA::<Exact>);

    /// One `Femtocandela` (fcd), as a `LuminousIntensity`.
    pub const FEMTOCANDELA<N: const From<Exact>>: LuminousIntensity<N> = counted(Exact::dec(1, -15) * CANDELA::<Exact>);

    /// One `Attocandela` (acd), as a `LuminousIntensity`.
    pub const ATTOCANDELA<N: const From<Exact>>: LuminousIntensity<N> = counted(Exact::dec(1, -18) * CANDELA::<Exact>);

    /// One `Zeptocandela` (zcd), as a `LuminousIntensity`.
    pub const ZEPTOCANDELA<N: const From<Exact>>: LuminousIntensity<N> = counted(Exact::dec(1, -21) * CANDELA::<Exact>);

    /// One `Yoctocandela` (ycd), as a `LuminousIntensity`.
    pub const YOCTOCANDELA<N: const From<Exact>>: LuminousIntensity<N> = counted(Exact::dec(1, -24) * CANDELA::<Exact>);

    /// One `Yottalux` (Ylx), as an `Illuminance`.
    pub const YOTTALUX<N: const From<Exact>>: Illuminance<N> = counted(Exact::int(1000000000000000000000000) * LUX::<Exact>);

    /// One `Zettalux` (Zlx), as an `Illuminance`.
    pub const ZETTALUX<N: const From<Exact>>: Illuminance<N> = counted(Exact::int(1000000000000000000000) * LUX::<Exact>);

    /// One `Exalux` (Elx), as an `Illuminance`.
    pub const EXALUX<N: const From<Exact>>: Illuminance<N> = counted(Exact::int(1000000000000000000) * LUX::<Exact>);

    /// One `Petalux` (Plx), as an `Illuminance`.
    pub const PETALUX<N: const From<Exact>>: Illuminance<N> = counted(Exact::int(1000000000000000) * LUX::<Exact>);

    /// One `Teralux` (Tlx), as an `Illuminance`.
    pub const TERALUX<N: const From<Exact>>: Illuminance<N> = counted(Exact::int(1000000000000) * LUX::<Exact>);

    /// One `Gigalux` (Glx), as an `Illuminance`.
    pub const GIGALUX<N: const From<Exact>>: Illuminance<N> = counted(Exact::int(1000000000) * LUX::<Exact>);

    /// One `Megalux` (Mlx), as an `Illuminance`.
    pub const MEGALUX<N: const From<Exact>>: Illuminance<N> = counted(Exact::int(1000000) * LUX::<Exact>);

    /// One `Kilolux` (klx), as an `Illuminance`.
    pub const KILOLUX<N: const From<Exact>>: Illuminance<N> = counted(Exact::int(1000) * LUX::<Exact>);

    /// One `Hectolux` (hlx), as an `Illuminance`.
    pub const HECTOLUX<N: const From<Exact>>: Illuminance<N> = counted(Exact::int(100) * LUX::<Exact>);

    /// One `Decalux` (dalx), as an `Illuminance`.
    pub const DECALUX<N: const From<Exact>>: Illuminance<N> = counted(Exact::int(10) * LUX::<Exact>);

    /// One `Lux` (lx), as an `Illuminance`.
    pub const LUX<N: const From<Exact>>: Illuminance<N> = counted(CANDELA::<Exact> * RADIAN::<Exact> * RADIAN::<Exact> / (METRE::<Exact> * METRE::<Exact>));

    /// One `Decilux` (dlx), as an `Illuminance`.
    pub const DECILUX<N: const From<Exact>>: Illuminance<N> = counted(Exact::dec(1, -1) * LUX::<Exact>);

    /// One `Centilux` (clx), as an `Illuminance`.
    pub const CENTILUX<N: const From<Exact>>: Illuminance<N> = counted(Exact::dec(1, -2) * LUX::<Exact>);

    /// One `Millilux` (mlx), as an `Illuminance`.
    pub const MILLILUX<N: const From<Exact>>: Illuminance<N> = counted(Exact::dec(1, -3) * LUX::<Exact>);

    /// One `Microlux` (µlx), as an `Illuminance`.
    pub const MICROLUX<N: const From<Exact>>: Illuminance<N> = counted(Exact::dec(1, -6) * LUX::<Exact>);

    /// One `Nanolux` (nlx), as an `Illuminance`.
    pub const NANOLUX<N: const From<Exact>>: Illuminance<N> = counted(Exact::dec(1, -9) * LUX::<Exact>);

    /// One `Picolux` (plx), as an `Illuminance`.
    pub const PICOLUX<N: const From<Exact>>: Illuminance<N> = counted(Exact::dec(1, -12) * LUX::<Exact>);

    /// One `Femtolux` (flx), as an `Illuminance`.
    pub const FEMTOLUX<N: const From<Exact>>: Illuminance<N> = counted(Exact::dec(1, -15) * LUX::<Exact>);

    /// One `Attolux` (alx), as an `Illuminance`.
    pub const ATTOLUX<N: const From<Exact>>: Illuminance<N> = counted(Exact::dec(1, -18) * LUX::<Exact>);

    /// One `Zeptolux` (zlx), as an `Illuminance`.
    pub const ZEPTOLUX<N: const From<Exact>>: Illuminance<N> = counted(Exact::dec(1, -21) * LUX::<Exact>);

    /// One `Yoctolux` (ylx), as an `Illuminance`.
    pub const YOCTOLUX<N: const From<Exact>>: Illuminance<N> = counted(Exact::dec(1, -24) * LUX::<Exact>);

    /// One `Phot` (ph), as an `Illuminance`.
    pub const PHOT<N: const From<Exact>>: Illuminance<N> = counted(Exact::int(10000) * LUX::<Exact>);

    /// One `Footcandle` (fc), as an `Illuminance`.
    pub const FOOTCANDLE<N: const From<Exact>>: Illuminance<N> = counted((Exact::int(1) / (Exact::dec(3048, -4) * Exact::dec(3048, -4))) * LUX::<Exact>);

    /// One `YottajoulePerSquareMetre` (YJ/m²), as a `RadiantExposure`.
    pub const YOTTAJOULE_PER_SQUARE_METRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(1000000000000000000000000) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `ZettajoulePerSquareMetre` (ZJ/m²), as a `RadiantExposure`.
    pub const ZETTAJOULE_PER_SQUARE_METRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(1000000000000000000000) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `ExajoulePerSquareMetre` (EJ/m²), as a `RadiantExposure`.
    pub const EXAJOULE_PER_SQUARE_METRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(1000000000000000000) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `PetajoulePerSquareMetre` (PJ/m²), as a `RadiantExposure`.
    pub const PETAJOULE_PER_SQUARE_METRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(1000000000000000) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `TerajoulePerSquareMetre` (TJ/m²), as a `RadiantExposure`.
    pub const TERAJOULE_PER_SQUARE_METRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(1000000000000) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `GigajoulePerSquareMetre` (GJ/m²), as a `RadiantExposure`.
    pub const GIGAJOULE_PER_SQUARE_METRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(1000000000) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `MegajoulePerSquareMetre` (MJ/m²), as a `RadiantExposure`.
    pub const MEGAJOULE_PER_SQUARE_METRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(1000000) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `KilojoulePerSquareMetre` (kJ/m²), as a `RadiantExposure`.
    pub const KILOJOULE_PER_SQUARE_METRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(1000) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `HectojoulePerSquareMetre` (hJ/m²), as a `RadiantExposure`.
    pub const HECTOJOULE_PER_SQUARE_METRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(100) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `DecajoulePerSquareMetre` (daJ/m²), as a `RadiantExposure`.
    pub const DECAJOULE_PER_SQUARE_METRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(10) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `JoulePerSquareMetre` (J/m²), as a `RadiantExposure`.
    pub const JOULE_PER_SQUARE_METRE<N: const From<Exact>>: RadiantExposure<N> = counted(KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact>));

    /// One `DecijoulePerSquareMetre` (dJ/m²), as a `RadiantExposure`.
    pub const DECIJOULE_PER_SQUARE_METRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::dec(1, -1) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `CentijoulePerSquareMetre` (cJ/m²), as a `RadiantExposure`.
    pub const CENTIJOULE_PER_SQUARE_METRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::dec(1, -2) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `MillijoulePerSquareMetre` (mJ/m²), as a `RadiantExposure`.
    pub const MILLIJOULE_PER_SQUARE_METRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::dec(1, -3) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `MicrojoulePerSquareMetre` (µJ/m²), as a `RadiantExposure`.
    pub const MICROJOULE_PER_SQUARE_METRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::dec(1, -6) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `NanojoulePerSquareMetre` (nJ/m²), as a `RadiantExposure`.
    pub const NANOJOULE_PER_SQUARE_METRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::dec(1, -9) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `PicojoulePerSquareMetre` (pJ/m²), as a `RadiantExposure`.
    pub const PICOJOULE_PER_SQUARE_METRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::dec(1, -12) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `FemtojoulePerSquareMetre` (fJ/m²), as a `RadiantExposure`.
    pub const FEMTOJOULE_PER_SQUARE_METRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::dec(1, -15) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `AttojoulePerSquareMetre` (aJ/m²), as a `RadiantExposure`.
    pub const ATTOJOULE_PER_SQUARE_METRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::dec(1, -18) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `ZeptojoulePerSquareMetre` (zJ/m²), as a `RadiantExposure`.
    pub const ZEPTOJOULE_PER_SQUARE_METRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::dec(1, -21) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `YoctojoulePerSquareMetre` (yJ/m²), as a `RadiantExposure`.
    pub const YOCTOJOULE_PER_SQUARE_METRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::dec(1, -24) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `WattHourPerSquareMetre` (W · h/m²), as a `RadiantExposure`.
    pub const WATT_HOUR_PER_SQUARE_METRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(3600) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `GigajoulePerSquareCentimetre` (GJ/cm²), as a `RadiantExposure`.
    pub const GIGAJOULE_PER_SQUARE_CENTIMETRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(10000000000000) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `MegajoulePerSquareCentimetre` (MJ/cm²), as a `RadiantExposure`.
    pub const MEGAJOULE_PER_SQUARE_CENTIMETRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(10000000000) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `KilojoulePerSquareCentimetre` (kJ/cm²), as a `RadiantExposure`.
    pub const KILOJOULE_PER_SQUARE_CENTIMETRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(10000000) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `HectojoulePerSquareCentimetre` (hJ/cm²), as a `RadiantExposure`.
    pub const HECTOJOULE_PER_SQUARE_CENTIMETRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(1000000) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `DecajoulePerSquareCentimetre` (daJ/cm²), as a `RadiantExposure`.
    pub const DECAJOULE_PER_SQUARE_CENTIMETRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(100000) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `JoulePerSquareCentimetre` (J/cm²), as a `RadiantExposure`.
    pub const JOULE_PER_SQUARE_CENTIMETRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(10000) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `DecijoulePerSquareCentimetre` (dJ/cm²), as a `RadiantExposure`.
    pub const DECIJOULE_PER_SQUARE_CENTIMETRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(1000) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `CentijoulePerSquareCentimetre` (cJ/cm²), as a `RadiantExposure`.
    pub const CENTIJOULE_PER_SQUARE_CENTIMETRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(100) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `MillijoulePerSquareCentimetre` (mJ/cm²), as a `RadiantExposure`.
    pub const MILLIJOULE_PER_SQUARE_CENTIMETRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(10) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `MicrojoulePerSquareCentimetre` (µJ/cm²), as a `RadiantExposure`.
    pub const MICROJOULE_PER_SQUARE_CENTIMETRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::dec(1, -2) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `NanojoulePerSquareCentimetre` (nJ/cm²), as a `RadiantExposure`.
    pub const NANOJOULE_PER_SQUARE_CENTIMETRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::dec(1, -5) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `GigajoulePerSquareMillimetre` (GJ/mm²), as a `RadiantExposure`.
    pub const GIGAJOULE_PER_SQUARE_MILLIMETRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(1000000000000000) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `MegajoulePerSquareMillimetre` (MJ/mm²), as a `RadiantExposure`.
    pub const MEGAJOULE_PER_SQUARE_MILLIMETRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(1000000000000) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `KilojoulePerSquareMillimetre` (kJ/mm²), as a `RadiantExposure`.
    pub const KILOJOULE_PER_SQUARE_MILLIMETRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(1000000000) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `HectojoulePerSquareMillimetre` (hJ/mm²), as a `RadiantExposure`.
    pub const HECTOJOULE_PER_SQUARE_MILLIMETRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(100000000) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `DecajoulePerSquareMillimetre` (daJ/mm²), as a `RadiantExposure`.
    pub const DECAJOULE_PER_SQUARE_MILLIMETRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(10000000) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `JoulePerSquareMillimetre` (J/mm²), as a `RadiantExposure`.
    pub const JOULE_PER_SQUARE_MILLIMETRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(1000000) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `DecijoulePerSquareMillimetre` (dJ/mm²), as a `RadiantExposure`.
    pub const DECIJOULE_PER_SQUARE_MILLIMETRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(100000) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `CentijoulePerSquareMillimetre` (cJ/mm²), as a `RadiantExposure`.
    pub const CENTIJOULE_PER_SQUARE_MILLIMETRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(10000) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `MillijoulePerSquareMillimetre` (mJ/mm²), as a `RadiantExposure`.
    pub const MILLIJOULE_PER_SQUARE_MILLIMETRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::int(1000) * JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `MicrojoulePerSquareMillimetre` (µJ/mm²), as a `RadiantExposure`.
    pub const MICROJOULE_PER_SQUARE_MILLIMETRE<N: const From<Exact>>: RadiantExposure<N> = counted(JOULE_PER_SQUARE_METRE::<Exact>);

    /// One `NanojoulePerSquareMillimetre` (nJ/mm²), as a `RadiantExposure`.
    pub const NANOJOULE_PER_SQUARE_MILLIMETRE<N: const From<Exact>>: RadiantExposure<N> = counted(Exact::dec(1, -3) * JOULE_PER_SQUARE_METRE::<Exact>);
}

/// The units of `Luminance`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Luminance<f64>)]
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
#[unit(of = LuminousIntensity<f64>)]
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
#[unit(of = Illuminance<f64>)]
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
#[unit(of = RadiantExposure<f64>)]
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
