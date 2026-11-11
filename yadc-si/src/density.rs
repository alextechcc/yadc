use crate::{Exact, ISQ, SIQuantity, counted};
use consts::*;
use yadc::Dimension;

pub type MassDensity<N> = SIQuantity<
    {
        ISQ {
            m: -3,
            kg: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type LinearMassDensity<N> = SIQuantity<
    {
        ISQ {
            m: -1,
            kg: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ArealMassDensity<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            kg: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type LinearNumberDensity<N> = SIQuantity<
    {
        ISQ {
            m: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ArealNumberDensity<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type VolumetricNumberDensity<N> = SIQuantity<
    {
        ISQ {
            m: -3,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type LinearNumberRate<N> = SIQuantity<
    {
        ISQ {
            m: -1,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ArealNumberRate<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type VolumetricNumberRate<N> = SIQuantity<
    {
        ISQ {
            m: -3,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type LinearDensityOfStates<N> = SIQuantity<
    {
        ISQ {
            m: -3,
            kg: -1,
            s: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ArealDensityOfStates<N> = SIQuantity<
    {
        ISQ {
            m: -4,
            kg: -1,
            s: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type VolumetricDensityOfStates<N> = SIQuantity<
    {
        ISQ {
            m: -5,
            kg: -1,
            s: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type LinearPowerDensity<N> = SIQuantity<
    {
        ISQ {
            m: 1,
            kg: 1,
            s: -3,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type VolumetricPowerDensity<N> = SIQuantity<
    {
        ISQ {
            m: -1,
            kg: 1,
            s: -3,
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
    use crate::base::{KILOGRAM, METRE, ONE, SECOND};

    /// One `YottagramPerCubicMetre` (Yg/m³), as a `MassDensity`.
    pub const YOTTAGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::int(1000000000000000000000) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `ZettagramPerCubicMetre` (Zg/m³), as a `MassDensity`.
    pub const ZETTAGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::int(1000000000000000000) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `ExagramPerCubicMetre` (Eg/m³), as a `MassDensity`.
    pub const EXAGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::int(1000000000000000) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `PetagramPerCubicMetre` (Pg/m³), as a `MassDensity`.
    pub const PETAGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::int(1000000000000) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `TeragramPerCubicMetre` (Tg/m³), as a `MassDensity`.
    pub const TERAGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::int(1000000000) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `GigagramPerCubicMetre` (Gg/m³), as a `MassDensity`.
    pub const GIGAGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::int(1000000) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `MegagramPerCubicMetre` (Mg/m³), as a `MassDensity`.
    pub const MEGAGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::int(1000) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `KilogramPerCubicMetre` (kg/m³), as a `MassDensity`.
    pub const KILOGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(KILOGRAM::<Exact> / (METRE::<Exact> * METRE::<Exact> * METRE::<Exact>));

    /// One `HectogramPerCubicMetre` (hg/m³), as a `MassDensity`.
    pub const HECTOGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::dec(1, -1) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `DecagramPerCubicMetre` (dag/m³), as a `MassDensity`.
    pub const DECAGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::dec(1, -2) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `GramPerCubicMetre` (g/m³), as a `MassDensity`.
    pub const GRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::dec(1, -3) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `DecigramPerCubicMetre` (dg/m³), as a `MassDensity`.
    pub const DECIGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::dec(1, -4) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `CentigramPerCubicMetre` (cg/m³), as a `MassDensity`.
    pub const CENTIGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::dec(1, -5) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `MilligramPerCubicMetre` (mg/m³), as a `MassDensity`.
    pub const MILLIGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::dec(1, -6) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `MicrogramPerCubicMetre` (µg/m³), as a `MassDensity`.
    pub const MICROGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::dec(1, -9) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `NanogramPerCubicMetre` (ng/m³), as a `MassDensity`.
    pub const NANOGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::dec(1, -12) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `PicogramPerCubicMetre` (pg/m³), as a `MassDensity`.
    pub const PICOGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::dec(1, -15) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `FemtogramPerCubicMetre` (fg/m³), as a `MassDensity`.
    pub const FEMTOGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::dec(1, -18) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `AttogramPerCubicMetre` (ag/m³), as a `MassDensity`.
    pub const ATTOGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::dec(1, -21) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `ZeptogramPerCubicMetre` (zg/m³), as a `MassDensity`.
    pub const ZEPTOGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::dec(1, -24) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `YoctogramPerCubicMetre` (yg/m³), as a `MassDensity`.
    pub const YOCTOGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::dec(1, -27) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `CaratPerCubicMetre` (ct/m³), as a `MassDensity`.
    pub const CARAT_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::dec(2, -4) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `GrainPerCubicMetre` (gr/m³), as a `MassDensity`.
    pub const GRAIN_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::dec(6479891, -11) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `HundredweightLongPerCubicMetre` (cwt long/m³), as a `MassDensity`.
    pub const HUNDREDWEIGHT_LONG_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::dec(5080234544, -8) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `HundredweightShortPerCubicMetre` (cwt short/m³), as a `MassDensity`.
    pub const HUNDREDWEIGHT_SHORT_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::dec(45359237, -6) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `OuncePerCubicMetre` (oz/m³), as a `MassDensity`.
    pub const OUNCE_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::dec(28349523125, -12) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `OunceTroyPerCubicMetre` (oz t/m³), as a `MassDensity`.
    pub const OUNCE_TROY_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::dec(311034768, -10) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `PennyweightPerCubicMetre` (dwt/m³), as a `MassDensity`.
    pub const PENNYWEIGHT_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::dec(155517384, -11) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `PoundPerCubicMetre` (lb/m³), as a `MassDensity`.
    pub const POUND_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::dec(45359237, -8) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `PoundTroyPerCubicMetre` (lb t/m³), as a `MassDensity`.
    pub const POUND_TROY_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::dec(3732417216, -10) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `SlugPerCubicMetre` (slug/m³), as a `MassDensity`.
    pub const SLUG_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted((Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4)) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `TonAssayPerCubicMetre` (AT/m³), as a `MassDensity`.
    pub const TON_ASSAY_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted((Exact::dec(175, -3) / Exact::int(6)) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `TonLongPerCubicMetre` (2240 lb/m³), as a `MassDensity`.
    pub const TON_LONG_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::dec(10160469088, -7) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `TonShortPerCubicMetre` (2000 lb/m³), as a `MassDensity`.
    pub const TON_SHORT_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::dec(90718474, -5) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `TonPerCubicMetre` (t/m³), as a `MassDensity`.
    pub const TON_PER_CUBIC_METRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::int(1000) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `GrainPerGallon` (gr/gal), as a `MassDensity`.
    pub const GRAIN_PER_GALLON<N: const From<Exact>>: MassDensity<N> = counted((Exact::dec(6479891, -11) / Exact::dec(3785411784, -12)) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `GramPerCubicCentimetre` (g/cm³), as a `MassDensity`.
    pub const GRAM_PER_CUBIC_CENTIMETRE<N: const From<Exact>>: MassDensity<N> = counted(Exact::int(1000) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `OuncePerCubicInch` (oz/in³), as a `MassDensity`.
    pub const OUNCE_PER_CUBIC_INCH<N: const From<Exact>>: MassDensity<N> = counted((Exact::dec(28349523125, -12) / (Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4))) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `OuncePerGallonImperial` (oz/gal (UK)), as a `MassDensity`.
    pub const OUNCE_PER_GALLON_IMPERIAL<N: const From<Exact>>: MassDensity<N> = counted((Exact::dec(28349523125, -12) / Exact::dec(454609, -8)) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `OuncePerGallon` (oz/gal), as a `MassDensity`.
    pub const OUNCE_PER_GALLON<N: const From<Exact>>: MassDensity<N> = counted((Exact::dec(28349523125, -12) / Exact::dec(3785411784, -12)) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `PoundPerCubicFoot` (lb/ft³), as a `MassDensity`.
    pub const POUND_PER_CUBIC_FOOT<N: const From<Exact>>: MassDensity<N> = counted((Exact::dec(45359237, -8) / (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4))) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `PoundPerCubicInch` (lb/in³), as a `MassDensity`.
    pub const POUND_PER_CUBIC_INCH<N: const From<Exact>>: MassDensity<N> = counted((Exact::dec(45359237, -8) / (Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4))) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `PoundPerCubicYard` (lb/yd³), as a `MassDensity`.
    pub const POUND_PER_CUBIC_YARD<N: const From<Exact>>: MassDensity<N> = counted((Exact::dec(45359237, -8) / (Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4))) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `PoundPerGallonImperial` (lb/gal (UK)), as a `MassDensity`.
    pub const POUND_PER_GALLON_IMPERIAL<N: const From<Exact>>: MassDensity<N> = counted((Exact::dec(45359237, -8) / Exact::dec(454609, -8)) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `PoundPerGallon` (lb/gal), as a `MassDensity`.
    pub const POUND_PER_GALLON<N: const From<Exact>>: MassDensity<N> = counted((Exact::dec(45359237, -8) / Exact::dec(3785411784, -12)) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `SlugPerCubicFoot` (slug/ft³), as a `MassDensity`.
    pub const SLUG_PER_CUBIC_FOOT<N: const From<Exact>>: MassDensity<N> = counted(((Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4)) / (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4))) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `TonLongPerCubicYard` (2240 lb/yd³), as a `MassDensity`.
    pub const TON_LONG_PER_CUBIC_YARD<N: const From<Exact>>: MassDensity<N> = counted((Exact::dec(10160469088, -7) / (Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4))) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `TonShortPerCubicYard` (2000 lb/yd³), as a `MassDensity`.
    pub const TON_SHORT_PER_CUBIC_YARD<N: const From<Exact>>: MassDensity<N> = counted((Exact::dec(90718474, -5) / (Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4))) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `TonLongPerCubicFoot` (2240 lb/ft³), as a `MassDensity`.
    pub const TON_LONG_PER_CUBIC_FOOT<N: const From<Exact>>: MassDensity<N> = counted((Exact::dec(10160469088, -7) / (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4))) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `TonShortPerCubicFoot` (2000 lb/ft³), as a `MassDensity`.
    pub const TON_SHORT_PER_CUBIC_FOOT<N: const From<Exact>>: MassDensity<N> = counted((Exact::dec(90718474, -5) / (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4))) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `KilogramPerMetre` (kg/m), as a `LinearMassDensity`.
    pub const KILOGRAM_PER_METRE<N: const From<Exact>>: LinearMassDensity<N> = counted(KILOGRAM::<Exact> / METRE::<Exact>);

    /// One `GramPerKilometre` (g/km), as a `LinearMassDensity`.
    pub const GRAM_PER_KILOMETRE<N: const From<Exact>>: LinearMassDensity<N> = counted(Exact::dec(1, -6) * KILOGRAM_PER_METRE::<Exact>);

    /// One `GramPerCentimetre` (g/cm), as a `LinearMassDensity`.
    pub const GRAM_PER_CENTIMETRE<N: const From<Exact>>: LinearMassDensity<N> = counted(Exact::dec(1, -1) * KILOGRAM_PER_METRE::<Exact>);

    /// One `OuncePerFoot` (oz/ft), as a `LinearMassDensity`.
    pub const OUNCE_PER_FOOT<N: const From<Exact>>: LinearMassDensity<N> = counted((Exact::dec(28349523125, -12) / Exact::dec(3048, -4)) * KILOGRAM_PER_METRE::<Exact>);

    /// One `OuncePerInch` (oz/in), as a `LinearMassDensity`.
    pub const OUNCE_PER_INCH<N: const From<Exact>>: LinearMassDensity<N> = counted((Exact::dec(28349523125, -12) / Exact::dec(254, -4)) * KILOGRAM_PER_METRE::<Exact>);

    /// One `PoundPerYard` (lb/yd), as a `LinearMassDensity`.
    pub const POUND_PER_YARD<N: const From<Exact>>: LinearMassDensity<N> = counted((Exact::dec(45359237, -8) / Exact::dec(9144, -4)) * KILOGRAM_PER_METRE::<Exact>);

    /// One `PoundPerFoot` (lb/ft), as a `LinearMassDensity`.
    pub const POUND_PER_FOOT<N: const From<Exact>>: LinearMassDensity<N> = counted((Exact::dec(45359237, -8) / Exact::dec(3048, -4)) * KILOGRAM_PER_METRE::<Exact>);

    /// One `PoundPerInch` (lb/in), as a `LinearMassDensity`.
    pub const POUND_PER_INCH<N: const From<Exact>>: LinearMassDensity<N> = counted((Exact::dec(45359237, -8) / Exact::dec(254, -4)) * KILOGRAM_PER_METRE::<Exact>);

    /// One `KilogramPerSquareMetre` (kg/m²), as an `ArealMassDensity`.
    pub const KILOGRAM_PER_SQUARE_METRE<N: const From<Exact>>: ArealMassDensity<N> = counted(KILOGRAM::<Exact> / (METRE::<Exact> * METRE::<Exact>));

    /// One `GramPerSquareMetre` (g/m²), as an `ArealMassDensity`.
    pub const GRAM_PER_SQUARE_METRE<N: const From<Exact>>: ArealMassDensity<N> = counted(Exact::dec(1, -3) * KILOGRAM_PER_SQUARE_METRE::<Exact>);

    /// One `OuncePerSquareFoot` (oz/ft²), as an `ArealMassDensity`.
    pub const OUNCE_PER_SQUARE_FOOT<N: const From<Exact>>: ArealMassDensity<N> = counted((Exact::dec(28349523125, -12) / (Exact::dec(3048, -4) * Exact::dec(3048, -4))) * KILOGRAM_PER_SQUARE_METRE::<Exact>);

    /// One `PerKilometre` (km⁻¹), as a `LinearNumberDensity`.
    pub const PER_KILOMETRE<N: const From<Exact>>: LinearNumberDensity<N> = counted(Exact::dec(1, -3) * PER_METRE::<Exact>);

    /// One `PerMetre` (m⁻¹), as a `LinearNumberDensity`.
    pub const PER_METRE<N: const From<Exact>>: LinearNumberDensity<N> = counted(ONE::<Exact> / METRE::<Exact>);

    /// One `PerDecimetre` (dm⁻¹), as a `LinearNumberDensity`.
    pub const PER_DECIMETRE<N: const From<Exact>>: LinearNumberDensity<N> = counted(Exact::int(10) * PER_METRE::<Exact>);

    /// One `PerCentimetre` (cm⁻¹), as a `LinearNumberDensity`.
    pub const PER_CENTIMETRE<N: const From<Exact>>: LinearNumberDensity<N> = counted(Exact::int(100) * PER_METRE::<Exact>);

    /// One `PerMillimetre` (mm⁻¹), as a `LinearNumberDensity`.
    pub const PER_MILLIMETRE<N: const From<Exact>>: LinearNumberDensity<N> = counted(Exact::int(1000) * PER_METRE::<Exact>);

    /// One `PerFoot` (ft⁻¹), as a `LinearNumberDensity`.
    pub const PER_FOOT<N: const From<Exact>>: LinearNumberDensity<N> = counted((Exact::int(1) / Exact::dec(3048, -4)) * PER_METRE::<Exact>);

    /// One `PerFootSurvey` (ft (U.S. survey)), as a `LinearNumberDensity`.
    pub const PER_FOOT_SURVEY<N: const From<Exact>>: LinearNumberDensity<N> = counted((Exact::int(1) / (Exact::int(1200) / Exact::int(3937))) * PER_METRE::<Exact>);

    /// One `PerInch` (in⁻¹), as a `LinearNumberDensity`.
    pub const PER_INCH<N: const From<Exact>>: LinearNumberDensity<N> = counted((Exact::int(1) / Exact::dec(254, -4)) * PER_METRE::<Exact>);

    /// One `PerMile` (mi⁻¹), as a `LinearNumberDensity`.
    pub const PER_MILE<N: const From<Exact>>: LinearNumberDensity<N> = counted((Exact::int(1) / Exact::dec(1609344, -3)) * PER_METRE::<Exact>);

    /// One `PerMileSurvey` (mi⁻¹ (U.S. survey)), as a `LinearNumberDensity`.
    pub const PER_MILE_SURVEY<N: const From<Exact>>: LinearNumberDensity<N> = counted((Exact::int(1) / (Exact::int(6336000) / Exact::int(3937))) * PER_METRE::<Exact>);

    /// One `PerNauticalMile` (M⁻¹), as a `LinearNumberDensity`.
    pub const PER_NAUTICAL_MILE<N: const From<Exact>>: LinearNumberDensity<N> = counted((Exact::int(1) / Exact::int(1852)) * PER_METRE::<Exact>);

    /// One `PerYard` (yd⁻¹), as a `LinearNumberDensity`.
    pub const PER_YARD<N: const From<Exact>>: LinearNumberDensity<N> = counted((Exact::int(1) / Exact::dec(9144, -4)) * PER_METRE::<Exact>);

    /// One `PerSquareMetre` (m⁻²), as an `ArealNumberDensity`.
    pub const PER_SQUARE_METRE<N: const From<Exact>>: ArealNumberDensity<N> = counted(ONE::<Exact> / (METRE::<Exact> * METRE::<Exact>));

    /// One `PerAcre` (ac⁻²), as an `ArealNumberDensity`.
    pub const PER_ACRE<N: const From<Exact>>: ArealNumberDensity<N> = counted((Exact::int(1) / (Exact::int(43560) * (Exact::int(1200) / Exact::int(3937)) * (Exact::int(1200) / Exact::int(3937)))) * PER_SQUARE_METRE::<Exact>);

    /// One `PerAre` (a⁻²), as an `ArealNumberDensity`.
    pub const PER_ARE<N: const From<Exact>>: ArealNumberDensity<N> = counted(Exact::dec(1, -2) * PER_SQUARE_METRE::<Exact>);

    /// One `PerBarn` (b⁻²), as an `ArealNumberDensity`.
    pub const PER_BARN<N: const From<Exact>>: ArealNumberDensity<N> = counted(Exact::int(10000000000000000000000000000) * PER_SQUARE_METRE::<Exact>);

    /// One `PerCircularMil` (cmil⁻²), as an `ArealNumberDensity`.
    pub const PER_CIRCULAR_MIL<N: const From<Exact>>: ArealNumberDensity<N> = counted((Exact::int(4) / (Exact::PI * Exact::dec(254, -7) * Exact::dec(254, -7))) * PER_SQUARE_METRE::<Exact>);

    /// One `PerHectare` (ha⁻²), as an `ArealNumberDensity`.
    pub const PER_HECTARE<N: const From<Exact>>: ArealNumberDensity<N> = counted(Exact::dec(1, -4) * PER_SQUARE_METRE::<Exact>);

    /// One `PerSquareFoot` (ft⁻²), as an `ArealNumberDensity`.
    pub const PER_SQUARE_FOOT<N: const From<Exact>>: ArealNumberDensity<N> = counted((Exact::int(1) / (Exact::dec(3048, -4) * Exact::dec(3048, -4))) * PER_SQUARE_METRE::<Exact>);

    /// One `PerSquareInch` (in⁻²), as an `ArealNumberDensity`.
    pub const PER_SQUARE_INCH<N: const From<Exact>>: ArealNumberDensity<N> = counted((Exact::int(1) / (Exact::dec(254, -4) * Exact::dec(254, -4))) * PER_SQUARE_METRE::<Exact>);

    /// One `PerSquareMile` (mi⁻²), as an `ArealNumberDensity`.
    pub const PER_SQUARE_MILE<N: const From<Exact>>: ArealNumberDensity<N> = counted((Exact::int(1) / (Exact::dec(1609344, -3) * Exact::dec(1609344, -3))) * PER_SQUARE_METRE::<Exact>);

    /// One `PerSquareYard` (yd⁻²), as an `ArealNumberDensity`.
    pub const PER_SQUARE_YARD<N: const From<Exact>>: ArealNumberDensity<N> = counted((Exact::int(1) / (Exact::dec(9144, -4) * Exact::dec(9144, -4))) * PER_SQUARE_METRE::<Exact>);

    /// One `PerCubicMetre` (m⁻³), as a `VolumetricNumberDensity`.
    pub const PER_CUBIC_METRE<N: const From<Exact>>: VolumetricNumberDensity<N> = counted(ONE::<Exact> / (METRE::<Exact> * METRE::<Exact> * METRE::<Exact>));

    /// One `PerCubicFoot` (ft⁻³), as a `VolumetricNumberDensity`.
    pub const PER_CUBIC_FOOT<N: const From<Exact>>: VolumetricNumberDensity<N> = counted((Exact::int(1) / (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4))) * PER_CUBIC_METRE::<Exact>);

    /// One `PerCubicInch` (in⁻³), as a `VolumetricNumberDensity`.
    pub const PER_CUBIC_INCH<N: const From<Exact>>: VolumetricNumberDensity<N> = counted((Exact::int(1) / (Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4))) * PER_CUBIC_METRE::<Exact>);

    /// One `PerCubicMile` (mi⁻³), as a `VolumetricNumberDensity`.
    pub const PER_CUBIC_MILE<N: const From<Exact>>: VolumetricNumberDensity<N> = counted((Exact::int(1) / (Exact::dec(1609344, -3) * Exact::dec(1609344, -3) * Exact::dec(1609344, -3))) * PER_CUBIC_METRE::<Exact>);

    /// One `PerCubicYard` (yd⁻³), as a `VolumetricNumberDensity`.
    pub const PER_CUBIC_YARD<N: const From<Exact>>: VolumetricNumberDensity<N> = counted((Exact::int(1) / (Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4))) * PER_CUBIC_METRE::<Exact>);

    /// One `PerFluidOunce` (per fl oz), as a `VolumetricNumberDensity`.
    pub const PER_FLUID_OUNCE<N: const From<Exact>>: VolumetricNumberDensity<N> = counted((Exact::int(1) / Exact::dec(295735295625, -16)) * PER_CUBIC_METRE::<Exact>);

    /// One `PerFluidOunceImperial` (per fl oz (UK)), as a `VolumetricNumberDensity`.
    pub const PER_FLUID_OUNCE_IMPERIAL<N: const From<Exact>>: VolumetricNumberDensity<N> = counted((Exact::int(1) / Exact::dec(284130625, -13)) * PER_CUBIC_METRE::<Exact>);

    /// One `PerGallonImperial` (per gal (UK)), as a `VolumetricNumberDensity`.
    pub const PER_GALLON_IMPERIAL<N: const From<Exact>>: VolumetricNumberDensity<N> = counted((Exact::int(1) / Exact::dec(454609, -8)) * PER_CUBIC_METRE::<Exact>);

    /// One `PerGallon` (per gal), as a `VolumetricNumberDensity`.
    pub const PER_GALLON<N: const From<Exact>>: VolumetricNumberDensity<N> = counted((Exact::int(1) / Exact::dec(3785411784, -12)) * PER_CUBIC_METRE::<Exact>);

    /// One `PerLitre` (L⁻¹), as a `VolumetricNumberDensity`.
    pub const PER_LITRE<N: const From<Exact>>: VolumetricNumberDensity<N> = counted(Exact::int(1000) * PER_CUBIC_METRE::<Exact>);

    /// One `PerKilometreSecond` (km⁻¹ · s⁻¹), as a `LinearNumberRate`.
    pub const PER_KILOMETRE_SECOND<N: const From<Exact>>: LinearNumberRate<N> = counted(Exact::dec(1, -3) * PER_METRE_SECOND::<Exact>);

    /// One `PerMetreSecond` (m⁻¹ · s⁻¹), as a `LinearNumberRate`.
    pub const PER_METRE_SECOND<N: const From<Exact>>: LinearNumberRate<N> = counted(ONE::<Exact> / (METRE::<Exact> * SECOND::<Exact>));

    /// One `PerCentimetreSecond` (cm⁻¹ · s⁻¹), as a `LinearNumberRate`.
    pub const PER_CENTIMETRE_SECOND<N: const From<Exact>>: LinearNumberRate<N> = counted(Exact::int(100) * PER_METRE_SECOND::<Exact>);

    /// One `PerMillimetreSecond` (mm⁻¹ · s⁻¹), as a `LinearNumberRate`.
    pub const PER_MILLIMETRE_SECOND<N: const From<Exact>>: LinearNumberRate<N> = counted(Exact::int(1000) * PER_METRE_SECOND::<Exact>);

    /// One `PerFootSecond` (ft⁻¹ · s⁻¹), as a `LinearNumberRate`.
    pub const PER_FOOT_SECOND<N: const From<Exact>>: LinearNumberRate<N> = counted((Exact::int(1) / Exact::dec(3048, -4)) * PER_METRE_SECOND::<Exact>);

    /// One `PerFootSurveySecond` (ft⁻¹ (U.S. survey) · s⁻¹), as a `LinearNumberRate`.
    pub const PER_FOOT_SURVEY_SECOND<N: const From<Exact>>: LinearNumberRate<N> = counted((Exact::int(1) / (Exact::int(1200) / Exact::int(3937))) * PER_METRE_SECOND::<Exact>);

    /// One `PerInchSecond` (in⁻¹ · s⁻¹), as a `LinearNumberRate`.
    pub const PER_INCH_SECOND<N: const From<Exact>>: LinearNumberRate<N> = counted((Exact::int(1) / Exact::dec(254, -4)) * PER_METRE_SECOND::<Exact>);

    /// One `PerMileSecond` (mi⁻¹ · s⁻¹), as a `LinearNumberRate`.
    pub const PER_MILE_SECOND<N: const From<Exact>>: LinearNumberRate<N> = counted((Exact::int(1) / Exact::dec(1609344, -3)) * PER_METRE_SECOND::<Exact>);

    /// One `PerMileSurveySecond` (mi⁻¹ (U.S. survey) · s⁻¹), as a `LinearNumberRate`.
    pub const PER_MILE_SURVEY_SECOND<N: const From<Exact>>: LinearNumberRate<N> = counted((Exact::int(1) / (Exact::int(6336000) / Exact::int(3937))) * PER_METRE_SECOND::<Exact>);

    /// One `PerYardSecond` (yd⁻¹ · s⁻¹), as a `LinearNumberRate`.
    pub const PER_YARD_SECOND<N: const From<Exact>>: LinearNumberRate<N> = counted((Exact::int(1) / Exact::dec(9144, -4)) * PER_METRE_SECOND::<Exact>);

    /// One `PerSquareMetreSecond` (m⁻² · s⁻¹), as an `ArealNumberRate`.
    pub const PER_SQUARE_METRE_SECOND<N: const From<Exact>>: ArealNumberRate<N> = counted(ONE::<Exact> / (METRE::<Exact> * METRE::<Exact> * SECOND::<Exact>));

    /// One `PerAcreSecond` (ac⁻¹ · s⁻¹), as an `ArealNumberRate`.
    pub const PER_ACRE_SECOND<N: const From<Exact>>: ArealNumberRate<N> = counted((Exact::int(1) / (Exact::int(43560) * (Exact::int(1200) / Exact::int(3937)) * (Exact::int(1200) / Exact::int(3937)))) * PER_SQUARE_METRE_SECOND::<Exact>);

    /// One `PerAreSecond` (a⁻¹ · s⁻¹), as an `ArealNumberRate`.
    pub const PER_ARE_SECOND<N: const From<Exact>>: ArealNumberRate<N> = counted(Exact::dec(1, -2) * PER_SQUARE_METRE_SECOND::<Exact>);

    /// One `PerBarnSecond` (b⁻¹ · s⁻¹), as an `ArealNumberRate`.
    pub const PER_BARN_SECOND<N: const From<Exact>>: ArealNumberRate<N> = counted(Exact::int(10000000000000000000000000000) * PER_SQUARE_METRE_SECOND::<Exact>);

    /// One `PerCircularMilSecond` (cmil⁻¹ · s⁻¹), as an `ArealNumberRate`.
    pub const PER_CIRCULAR_MIL_SECOND<N: const From<Exact>>: ArealNumberRate<N> = counted((Exact::int(4) / (Exact::PI * Exact::dec(254, -7) * Exact::dec(254, -7))) * PER_SQUARE_METRE_SECOND::<Exact>);

    /// One `PerHectareSecond` (ha⁻¹ · s⁻¹), as an `ArealNumberRate`.
    pub const PER_HECTARE_SECOND<N: const From<Exact>>: ArealNumberRate<N> = counted(Exact::dec(1, -4) * PER_SQUARE_METRE_SECOND::<Exact>);

    /// One `PerSquareFootSecond` (ft⁻² · s⁻¹), as an `ArealNumberRate`.
    pub const PER_SQUARE_FOOT_SECOND<N: const From<Exact>>: ArealNumberRate<N> = counted((Exact::int(1) / (Exact::dec(3048, -4) * Exact::dec(3048, -4))) * PER_SQUARE_METRE_SECOND::<Exact>);

    /// One `PerSquareInchSecond` (in⁻² · s⁻¹), as an `ArealNumberRate`.
    pub const PER_SQUARE_INCH_SECOND<N: const From<Exact>>: ArealNumberRate<N> = counted((Exact::int(1) / (Exact::dec(254, -4) * Exact::dec(254, -4))) * PER_SQUARE_METRE_SECOND::<Exact>);

    /// One `PerSquareMileSecond` (mi⁻² · s⁻¹), as an `ArealNumberRate`.
    pub const PER_SQUARE_MILE_SECOND<N: const From<Exact>>: ArealNumberRate<N> = counted((Exact::int(1) / (Exact::dec(1609344, -3) * Exact::dec(1609344, -3))) * PER_SQUARE_METRE_SECOND::<Exact>);

    /// One `PerSquareYardSecond` (yd⁻² · s⁻¹), as an `ArealNumberRate`.
    pub const PER_SQUARE_YARD_SECOND<N: const From<Exact>>: ArealNumberRate<N> = counted((Exact::int(1) / (Exact::dec(9144, -4) * Exact::dec(9144, -4))) * PER_SQUARE_METRE_SECOND::<Exact>);

    /// One `PerCubicMetreSecond` (m⁻³ · s⁻¹), as a `VolumetricNumberRate`.
    pub const PER_CUBIC_METRE_SECOND<N: const From<Exact>>: VolumetricNumberRate<N> = counted(ONE::<Exact> / (METRE::<Exact> * METRE::<Exact> * METRE::<Exact> * SECOND::<Exact>));

    /// One `PerCubicFootSecond` (ft⁻³ · s⁻¹), as a `VolumetricNumberRate`.
    pub const PER_CUBIC_FOOT_SECOND<N: const From<Exact>>: VolumetricNumberRate<N> = counted((Exact::int(1) / (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4))) * PER_CUBIC_METRE_SECOND::<Exact>);

    /// One `PerCubicInchSecond` (in⁻³ · s⁻¹), as a `VolumetricNumberRate`.
    pub const PER_CUBIC_INCH_SECOND<N: const From<Exact>>: VolumetricNumberRate<N> = counted((Exact::int(1) / (Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4))) * PER_CUBIC_METRE_SECOND::<Exact>);

    /// One `PerCubicMileSecond` (mi⁻³ · s⁻¹), as a `VolumetricNumberRate`.
    pub const PER_CUBIC_MILE_SECOND<N: const From<Exact>>: VolumetricNumberRate<N> = counted((Exact::int(1) / (Exact::dec(1609344, -3) * Exact::dec(1609344, -3) * Exact::dec(1609344, -3))) * PER_CUBIC_METRE_SECOND::<Exact>);

    /// One `PerCubicYardSecond` (yd⁻³ · s⁻¹), as a `VolumetricNumberRate`.
    pub const PER_CUBIC_YARD_SECOND<N: const From<Exact>>: VolumetricNumberRate<N> = counted((Exact::int(1) / (Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4))) * PER_CUBIC_METRE_SECOND::<Exact>);

    /// One `PerFluidOunceSecond` (fl oz⁻¹ · s⁻¹), as a `VolumetricNumberRate`.
    pub const PER_FLUID_OUNCE_SECOND<N: const From<Exact>>: VolumetricNumberRate<N> = counted((Exact::int(1) / Exact::dec(295735295625, -16)) * PER_CUBIC_METRE_SECOND::<Exact>);

    /// One `PerFluidOunceImperialSecond` (fl oz⁻¹ (UK) · s⁻¹), as a `VolumetricNumberRate`.
    pub const PER_FLUID_OUNCE_IMPERIAL_SECOND<N: const From<Exact>>: VolumetricNumberRate<N> = counted((Exact::int(1) / Exact::dec(284130625, -13)) * PER_CUBIC_METRE_SECOND::<Exact>);

    /// One `PerGallonImperialSecond` (gal⁻¹ (UK) · s⁻¹), as a `VolumetricNumberRate`.
    pub const PER_GALLON_IMPERIAL_SECOND<N: const From<Exact>>: VolumetricNumberRate<N> = counted((Exact::int(1) / Exact::dec(454609, -8)) * PER_CUBIC_METRE_SECOND::<Exact>);

    /// One `PerGallonSecond` (gal⁻¹ · s⁻¹), as a `VolumetricNumberRate`.
    pub const PER_GALLON_SECOND<N: const From<Exact>>: VolumetricNumberRate<N> = counted((Exact::int(1) / Exact::dec(3785411784, -12)) * PER_CUBIC_METRE_SECOND::<Exact>);

    /// One `PerLitreSecond` (L⁻¹ · s⁻¹), as a `VolumetricNumberRate`.
    pub const PER_LITRE_SECOND<N: const From<Exact>>: VolumetricNumberRate<N> = counted(Exact::int(1000) * PER_CUBIC_METRE_SECOND::<Exact>);

    /// One `BecquerelPerCubicMetre` (Bq/m³), as a `VolumetricNumberRate`.
    pub const BECQUEREL_PER_CUBIC_METRE<N: const From<Exact>>: VolumetricNumberRate<N> = counted(PER_CUBIC_METRE_SECOND::<Exact>);

    /// One `CuriePerCubicMetre` (Ci/m³), as a `VolumetricNumberRate`.
    pub const CURIE_PER_CUBIC_METRE<N: const From<Exact>>: VolumetricNumberRate<N> = counted(Exact::int(37000000000) * PER_CUBIC_METRE_SECOND::<Exact>);

    /// One `DisintegrationsPerMinutePerCubicMetre` (dpm/m³), as a `VolumetricNumberRate`.
    pub const DISINTEGRATIONS_PER_MINUTE_PER_CUBIC_METRE<N: const From<Exact>>: VolumetricNumberRate<N> = counted((Exact::int(1) / Exact::int(60)) * PER_CUBIC_METRE_SECOND::<Exact>);

    /// One `StatePerMetreJoule` (1/(m · J)), as a `LinearDensityOfStates`.
    pub const STATE_PER_METRE_JOULE<N: const From<Exact>>: LinearDensityOfStates<N> = counted(SECOND::<Exact> * SECOND::<Exact> / (METRE::<Exact> * METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact>));

    /// One `StatePerCentimetreJoule` (1/(cm · J)), as a `LinearDensityOfStates`.
    pub const STATE_PER_CENTIMETRE_JOULE<N: const From<Exact>>: LinearDensityOfStates<N> = counted(Exact::int(100) * STATE_PER_METRE_JOULE::<Exact>);

    /// One `StatePerSquareMetreJoule` (1/(m² · J)), as an `ArealDensityOfStates`.
    pub const STATE_PER_SQUARE_METRE_JOULE<N: const From<Exact>>: ArealDensityOfStates<N> = counted(SECOND::<Exact> * SECOND::<Exact> / (METRE::<Exact> * METRE::<Exact> * METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact>));

    /// One `StatePerCubicMetreJoule` (1/(m³ · J)), as a `VolumetricDensityOfStates`.
    pub const STATE_PER_CUBIC_METRE_JOULE<N: const From<Exact>>: VolumetricDensityOfStates<N> = counted(SECOND::<Exact> * SECOND::<Exact> / (METRE::<Exact> * METRE::<Exact> * METRE::<Exact> * METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact>));

    /// One `WattPerMetre` (W/m), as a `LinearPowerDensity`.
    pub const WATT_PER_METRE<N: const From<Exact>>: LinearPowerDensity<N> = counted(METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact>));

    /// One `WattPerCentimetre` (W/cm), as a `LinearPowerDensity`.
    pub const WATT_PER_CENTIMETRE<N: const From<Exact>>: LinearPowerDensity<N> = counted(Exact::int(100) * WATT_PER_METRE::<Exact>);

    /// One `WattPerMillimetre` (W/mm), as a `LinearPowerDensity`.
    pub const WATT_PER_MILLIMETRE<N: const From<Exact>>: LinearPowerDensity<N> = counted(Exact::int(1000) * WATT_PER_METRE::<Exact>);

    /// One `WattPerCubicMetre` (W/m³), as a `VolumetricPowerDensity`.
    pub const WATT_PER_CUBIC_METRE<N: const From<Exact>>: VolumetricPowerDensity<N> = counted(KILOGRAM::<Exact> / (METRE::<Exact> * SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact>));
}

/// The units of `MassDensity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassDensity<f64>)]
pub enum MassDensityUnit {
    #[unit("Yg/m³", "yottagrams per cubic metre", by = YOTTAGRAM_PER_CUBIC_METRE)]
    YottagramPerCubicMetre,
    #[unit("Zg/m³", "zettagrams per cubic metre", by = ZETTAGRAM_PER_CUBIC_METRE)]
    ZettagramPerCubicMetre,
    #[unit("Eg/m³", "exagrams per cubic metre", by = EXAGRAM_PER_CUBIC_METRE)]
    ExagramPerCubicMetre,
    #[unit("Pg/m³", "petagrams per cubic metre", by = PETAGRAM_PER_CUBIC_METRE)]
    PetagramPerCubicMetre,
    #[unit("Tg/m³", "teragrams per cubic metre", by = TERAGRAM_PER_CUBIC_METRE)]
    TeragramPerCubicMetre,
    #[unit("Gg/m³", "gigagrams per cubic metre", by = GIGAGRAM_PER_CUBIC_METRE)]
    GigagramPerCubicMetre,
    #[unit("Mg/m³", "megagrams per cubic metre", by = MEGAGRAM_PER_CUBIC_METRE)]
    MegagramPerCubicMetre,
    #[default]
    #[unit("kg/m³", "kilograms per cubic metre", by = KILOGRAM_PER_CUBIC_METRE)]
    KilogramPerCubicMetre,
    #[unit("hg/m³", "hectograms per cubic metre", by = HECTOGRAM_PER_CUBIC_METRE)]
    HectogramPerCubicMetre,
    #[unit("dag/m³", "decagrams per cubic metre", by = DECAGRAM_PER_CUBIC_METRE)]
    DecagramPerCubicMetre,
    #[unit("g/m³", "grams per cubic metre", by = GRAM_PER_CUBIC_METRE)]
    GramPerCubicMetre,
    #[unit("dg/m³", "decigrams per cubic metre", by = DECIGRAM_PER_CUBIC_METRE)]
    DecigramPerCubicMetre,
    #[unit("cg/m³", "centigrams per cubic metre", by = CENTIGRAM_PER_CUBIC_METRE)]
    CentigramPerCubicMetre,
    #[unit("mg/m³", "milligrams per cubic metre", by = MILLIGRAM_PER_CUBIC_METRE)]
    MilligramPerCubicMetre,
    #[unit("µg/m³", "micrograms per cubic metre", by = MICROGRAM_PER_CUBIC_METRE)]
    MicrogramPerCubicMetre,
    #[unit("ng/m³", "nanograms per cubic metre", by = NANOGRAM_PER_CUBIC_METRE)]
    NanogramPerCubicMetre,
    #[unit("pg/m³", "picograms per cubic metre", by = PICOGRAM_PER_CUBIC_METRE)]
    PicogramPerCubicMetre,
    #[unit("fg/m³", "femtograms per cubic metre", by = FEMTOGRAM_PER_CUBIC_METRE)]
    FemtogramPerCubicMetre,
    #[unit("ag/m³", "attograms per cubic metre", by = ATTOGRAM_PER_CUBIC_METRE)]
    AttogramPerCubicMetre,
    #[unit("zg/m³", "zeptograms per cubic metre", by = ZEPTOGRAM_PER_CUBIC_METRE)]
    ZeptogramPerCubicMetre,
    #[unit("yg/m³", "yoctograms per cubic metre", by = YOCTOGRAM_PER_CUBIC_METRE)]
    YoctogramPerCubicMetre,
    #[unit("ct/m³", "carats per cubic metre", by = CARAT_PER_CUBIC_METRE)]
    CaratPerCubicMetre,
    #[unit("gr/m³", "grains per cubic metre", by = GRAIN_PER_CUBIC_METRE)]
    GrainPerCubicMetre,
    #[unit("cwt long/m³", "hundredweight (long) per cubic metre", by = HUNDREDWEIGHT_LONG_PER_CUBIC_METRE)]
    HundredweightLongPerCubicMetre,
    #[unit("cwt short/m³", "hundredweight (short) per cubic metre", by = HUNDREDWEIGHT_SHORT_PER_CUBIC_METRE)]
    HundredweightShortPerCubicMetre,
    #[unit("oz/m³", "ounces per cubic metre", by = OUNCE_PER_CUBIC_METRE)]
    OuncePerCubicMetre,
    #[unit("oz t/m³", "troy ounces per cubic metre", by = OUNCE_TROY_PER_CUBIC_METRE)]
    OunceTroyPerCubicMetre,
    #[unit("dwt/m³", "pennyweight per cubic metre", by = PENNYWEIGHT_PER_CUBIC_METRE)]
    PennyweightPerCubicMetre,
    #[unit("lb/m³", "pounds per cubic metre", by = POUND_PER_CUBIC_METRE)]
    PoundPerCubicMetre,
    #[unit("lb t/m³", "troy pounds per cubic metre", by = POUND_TROY_PER_CUBIC_METRE)]
    PoundTroyPerCubicMetre,
    #[unit("slug/m³", "slugs per cubic metre", by = SLUG_PER_CUBIC_METRE)]
    SlugPerCubicMetre,
    #[unit("AT/m³", "assay tons per cubic metre", by = TON_ASSAY_PER_CUBIC_METRE)]
    TonAssayPerCubicMetre,
    #[unit("2240 lb/m³", "long tons per cubic metre", by = TON_LONG_PER_CUBIC_METRE)]
    TonLongPerCubicMetre,
    #[unit("2000 lb/m³", "short tons per cubic metre", by = TON_SHORT_PER_CUBIC_METRE)]
    TonShortPerCubicMetre,
    #[unit("t/m³", "tons per cubic metre", by = TON_PER_CUBIC_METRE)]
    TonPerCubicMetre,
    #[unit("gr/gal", "grains per gallon", by = GRAIN_PER_GALLON)]
    GrainPerGallon,
    #[unit("g/cm³", "grams per cubic centimetre", by = GRAM_PER_CUBIC_CENTIMETRE)]
    GramPerCubicCentimetre,
    #[unit("oz/in³", "ounces per cubic inch", by = OUNCE_PER_CUBIC_INCH)]
    OuncePerCubicInch,
    #[unit("oz/gal (UK)", "ounces per Imperial gallon", by = OUNCE_PER_GALLON_IMPERIAL)]
    OuncePerGallonImperial,
    #[unit("oz/gal", "ounces per gallon", by = OUNCE_PER_GALLON)]
    OuncePerGallon,
    #[unit("lb/ft³", "pounds per cubic foot", by = POUND_PER_CUBIC_FOOT)]
    PoundPerCubicFoot,
    #[unit("lb/in³", "pounds per cubic inch", by = POUND_PER_CUBIC_INCH)]
    PoundPerCubicInch,
    #[unit("lb/yd³", "pounds per cubic yard", by = POUND_PER_CUBIC_YARD)]
    PoundPerCubicYard,
    #[unit("lb/gal (UK)", "pounds per Imperial gallon", by = POUND_PER_GALLON_IMPERIAL)]
    PoundPerGallonImperial,
    #[unit("lb/gal", "pounds per gallon", by = POUND_PER_GALLON)]
    PoundPerGallon,
    #[unit("slug/ft³", "slugs per cubic foot", by = SLUG_PER_CUBIC_FOOT)]
    SlugPerCubicFoot,
    #[unit("2240 lb/yd³", "long tons per cubic yard", by = TON_LONG_PER_CUBIC_YARD)]
    TonLongPerCubicYard,
    #[unit("2000 lb/yd³", "short tons per cubic yard", by = TON_SHORT_PER_CUBIC_YARD)]
    TonShortPerCubicYard,
    #[unit("2240 lb/ft³", "long tons per cubic foot", by = TON_LONG_PER_CUBIC_FOOT)]
    TonLongPerCubicFoot,
    #[unit("2000 lb/ft³", "short tons per cubic foot", by = TON_SHORT_PER_CUBIC_FOOT)]
    TonShortPerCubicFoot,
}

/// The units of `LinearMassDensity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LinearMassDensity<f64>)]
pub enum LinearMassDensityUnit {
    #[default]
    #[unit("kg/m", "kilograms per metre", by = KILOGRAM_PER_METRE)]
    KilogramPerMetre,
    #[unit("g/km", "grams per kilometre", by = GRAM_PER_KILOMETRE)]
    GramPerKilometre,
    #[unit("g/cm", "grams per centimetre", by = GRAM_PER_CENTIMETRE)]
    GramPerCentimetre,
    #[unit("oz/ft", "ounces per foot", by = OUNCE_PER_FOOT)]
    OuncePerFoot,
    #[unit("oz/in", "ounces per inch", by = OUNCE_PER_INCH)]
    OuncePerInch,
    #[unit("lb/yd", "pounds per yard", by = POUND_PER_YARD)]
    PoundPerYard,
    #[unit("lb/ft", "pounds per foot", by = POUND_PER_FOOT)]
    PoundPerFoot,
    #[unit("lb/in", "pounds per inch", by = POUND_PER_INCH)]
    PoundPerInch,
}

/// The units of `ArealMassDensity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ArealMassDensity<f64>)]
pub enum ArealMassDensityUnit {
    #[default]
    #[unit("kg/m²", "kilograms per square metre", by = KILOGRAM_PER_SQUARE_METRE)]
    KilogramPerSquareMetre,
    #[unit("g/m²", "grams per square metre", by = GRAM_PER_SQUARE_METRE)]
    GramPerSquareMetre,
    #[unit("oz/ft²", "ounces per square foot", by = OUNCE_PER_SQUARE_FOOT)]
    OuncePerSquareFoot,
}

/// The units of `LinearNumberDensity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LinearNumberDensity<f64>)]
pub enum LinearNumberDensityUnit {
    #[unit("km⁻¹", "per kilometre", by = PER_KILOMETRE)]
    PerKilometre,
    #[default]
    #[unit("m⁻¹", "per metre", by = PER_METRE)]
    PerMetre,
    #[unit("dm⁻¹", "per decimetre", by = PER_DECIMETRE)]
    PerDecimetre,
    #[unit("cm⁻¹", "per centimetre", by = PER_CENTIMETRE)]
    PerCentimetre,
    #[unit("mm⁻¹", "per millimetre", by = PER_MILLIMETRE)]
    PerMillimetre,
    #[unit("ft⁻¹", "per foot", by = PER_FOOT)]
    PerFoot,
    #[unit("ft (U.S. survey)", "foot (U.S. survey)", by = PER_FOOT_SURVEY)]
    PerFootSurvey,
    #[unit("in⁻¹", "per inch", by = PER_INCH)]
    PerInch,
    #[unit("mi⁻¹", "per mile", by = PER_MILE)]
    PerMile,
    #[unit("mi⁻¹ (U.S. survey)", "per mile (U.S. survey)", by = PER_MILE_SURVEY)]
    PerMileSurvey,
    #[unit("M⁻¹", "per nautical mile", by = PER_NAUTICAL_MILE)]
    PerNauticalMile,
    #[unit("yd⁻¹", "per yard", by = PER_YARD)]
    PerYard,
}

/// The units of `ArealNumberDensity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ArealNumberDensity<f64>)]
pub enum ArealNumberDensityUnit {
    #[default]
    #[unit("m⁻²", "per square metre", by = PER_SQUARE_METRE)]
    PerSquareMetre,
    #[unit("ac⁻²", "per acre", by = PER_ACRE)]
    PerAcre,
    #[unit("a⁻²", "per are", by = PER_ARE)]
    PerAre,
    #[unit("b⁻²", "per barn", by = PER_BARN)]
    PerBarn,
    #[unit("cmil⁻²", "per circular mil", by = PER_CIRCULAR_MIL)]
    PerCircularMil,
    #[unit("ha⁻²", "per hectare", by = PER_HECTARE)]
    PerHectare,
    #[unit("ft⁻²", "per square foot", by = PER_SQUARE_FOOT)]
    PerSquareFoot,
    #[unit("in⁻²", "per square inch", by = PER_SQUARE_INCH)]
    PerSquareInch,
    #[unit("mi⁻²", "per square mile", by = PER_SQUARE_MILE)]
    PerSquareMile,
    #[unit("yd⁻²", "per square yard", by = PER_SQUARE_YARD)]
    PerSquareYard,
}

/// The units of `VolumetricNumberDensity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = VolumetricNumberDensity<f64>)]
pub enum VolumetricNumberDensityUnit {
    #[default]
    #[unit("m⁻³", "per cubic metre", by = PER_CUBIC_METRE)]
    PerCubicMetre,
    #[unit("ft⁻³", "per cubic foot", by = PER_CUBIC_FOOT)]
    PerCubicFoot,
    #[unit("in⁻³", "per cubic inch", by = PER_CUBIC_INCH)]
    PerCubicInch,
    #[unit("mi⁻³", "per cubic mile", by = PER_CUBIC_MILE)]
    PerCubicMile,
    #[unit("yd⁻³", "per cubic yard", by = PER_CUBIC_YARD)]
    PerCubicYard,
    #[unit("per fl oz", "per fluid ounce", by = PER_FLUID_OUNCE)]
    PerFluidOunce,
    #[unit("per fl oz (UK)", "per Imperial fluid ounce", by = PER_FLUID_OUNCE_IMPERIAL)]
    PerFluidOunceImperial,
    #[unit("per gal (UK)", "per Imperial gallon", by = PER_GALLON_IMPERIAL)]
    PerGallonImperial,
    #[unit("per gal", "per gallon", by = PER_GALLON)]
    PerGallon,
    #[unit("L⁻¹", "per litre", by = PER_LITRE)]
    PerLitre,
}

/// The units of `LinearNumberRate`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LinearNumberRate<f64>)]
pub enum LinearNumberRateUnit {
    #[unit("km⁻¹ · s⁻¹", "per kilometre second", by = PER_KILOMETRE_SECOND)]
    PerKilometreSecond,
    #[default]
    #[unit("m⁻¹ · s⁻¹", "per metre second", by = PER_METRE_SECOND)]
    PerMetreSecond,
    #[unit("cm⁻¹ · s⁻¹", "per centimetre second", by = PER_CENTIMETRE_SECOND)]
    PerCentimetreSecond,
    #[unit("mm⁻¹ · s⁻¹", "per millimetre second", by = PER_MILLIMETRE_SECOND)]
    PerMillimetreSecond,
    #[unit("ft⁻¹ · s⁻¹", "per foot second", by = PER_FOOT_SECOND)]
    PerFootSecond,
    #[unit("ft⁻¹ (U.S. survey) · s⁻¹", "per foot (U.S. survey) second", by = PER_FOOT_SURVEY_SECOND)]
    PerFootSurveySecond,
    #[unit("in⁻¹ · s⁻¹", "per inch second", by = PER_INCH_SECOND)]
    PerInchSecond,
    #[unit("mi⁻¹ · s⁻¹", "per mile second", by = PER_MILE_SECOND)]
    PerMileSecond,
    #[unit("mi⁻¹ (U.S. survey) · s⁻¹", "per mile (U.S. survey) second", by = PER_MILE_SURVEY_SECOND)]
    PerMileSurveySecond,
    #[unit("yd⁻¹ · s⁻¹", "per yard second", by = PER_YARD_SECOND)]
    PerYardSecond,
}

/// The units of `ArealNumberRate`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ArealNumberRate<f64>)]
pub enum ArealNumberRateUnit {
    #[default]
    #[unit("m⁻² · s⁻¹", "per square metre second", by = PER_SQUARE_METRE_SECOND)]
    PerSquareMetreSecond,
    #[unit("ac⁻¹ · s⁻¹", "per acre second", by = PER_ACRE_SECOND)]
    PerAcreSecond,
    #[unit("a⁻¹ · s⁻¹", "per are second", by = PER_ARE_SECOND)]
    PerAreSecond,
    #[unit("b⁻¹ · s⁻¹", "per barn second", by = PER_BARN_SECOND)]
    PerBarnSecond,
    #[unit("cmil⁻¹ · s⁻¹", "per circular mil second", by = PER_CIRCULAR_MIL_SECOND)]
    PerCircularMilSecond,
    #[unit("ha⁻¹ · s⁻¹", "per hectare second", by = PER_HECTARE_SECOND)]
    PerHectareSecond,
    #[unit("ft⁻² · s⁻¹", "per square foot second", by = PER_SQUARE_FOOT_SECOND)]
    PerSquareFootSecond,
    #[unit("in⁻² · s⁻¹", "per square inch second", by = PER_SQUARE_INCH_SECOND)]
    PerSquareInchSecond,
    #[unit("mi⁻² · s⁻¹", "per square mile second", by = PER_SQUARE_MILE_SECOND)]
    PerSquareMileSecond,
    #[unit("yd⁻² · s⁻¹", "per square yard second", by = PER_SQUARE_YARD_SECOND)]
    PerSquareYardSecond,
}

/// The units of `VolumetricNumberRate`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = VolumetricNumberRate<f64>)]
pub enum VolumetricNumberRateUnit {
    #[default]
    #[unit("m⁻³ · s⁻¹", "per cubic metre second", by = PER_CUBIC_METRE_SECOND)]
    PerCubicMetreSecond,
    #[unit("ft⁻³ · s⁻¹", "per cubic foot second", by = PER_CUBIC_FOOT_SECOND)]
    PerCubicFootSecond,
    #[unit("in⁻³ · s⁻¹", "per cubic inch second", by = PER_CUBIC_INCH_SECOND)]
    PerCubicInchSecond,
    #[unit("mi⁻³ · s⁻¹", "per cubic mile second", by = PER_CUBIC_MILE_SECOND)]
    PerCubicMileSecond,
    #[unit("yd⁻³ · s⁻¹", "per cubic yard second", by = PER_CUBIC_YARD_SECOND)]
    PerCubicYardSecond,
    #[unit("fl oz⁻¹ · s⁻¹", "per fluid ounce second", by = PER_FLUID_OUNCE_SECOND)]
    PerFluidOunceSecond,
    #[unit("fl oz⁻¹ (UK) · s⁻¹", "per Imperial fluid ounce second", by = PER_FLUID_OUNCE_IMPERIAL_SECOND)]
    PerFluidOunceImperialSecond,
    #[unit("gal⁻¹ (UK) · s⁻¹", "per Imperial gallon second", by = PER_GALLON_IMPERIAL_SECOND)]
    PerGallonImperialSecond,
    #[unit("gal⁻¹ · s⁻¹", "per gallon second", by = PER_GALLON_SECOND)]
    PerGallonSecond,
    #[unit("L⁻¹ · s⁻¹", "per litre second", by = PER_LITRE_SECOND)]
    PerLitreSecond,
    #[unit("Bq/m³", "becquerels per cubic metre", by = BECQUEREL_PER_CUBIC_METRE)]
    BecquerelPerCubicMetre,
    #[unit("Ci/m³", "curies per cubic metre", by = CURIE_PER_CUBIC_METRE)]
    CuriePerCubicMetre,
    #[unit("dpm/m³", "disintegrations per minute per cubic metre", by = DISINTEGRATIONS_PER_MINUTE_PER_CUBIC_METRE)]
    DisintegrationsPerMinutePerCubicMetre,
}

/// The units of `LinearDensityOfStates`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LinearDensityOfStates<f64>)]
pub enum LinearDensityOfStatesUnit {
    #[default]
    #[unit("1/(m · J)", "states per metre joule", by = STATE_PER_METRE_JOULE)]
    StatePerMetreJoule,
    #[unit("1/(cm · J)", "states per centimetre joule", by = STATE_PER_CENTIMETRE_JOULE)]
    StatePerCentimetreJoule,
}

/// The units of `ArealDensityOfStates`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ArealDensityOfStates<f64>)]
pub enum ArealDensityOfStatesUnit {
    #[default]
    #[unit("1/(m² · J)", "states per square metre joule", by = STATE_PER_SQUARE_METRE_JOULE)]
    StatePerSquareMetreJoule,
}

/// The units of `VolumetricDensityOfStates`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = VolumetricDensityOfStates<f64>)]
pub enum VolumetricDensityOfStatesUnit {
    #[default]
    #[unit("1/(m³ · J)", "states per cubic metre joule", by = STATE_PER_CUBIC_METRE_JOULE)]
    StatePerCubicMetreJoule,
}

/// The units of `LinearPowerDensity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LinearPowerDensity<f64>)]
pub enum LinearPowerDensityUnit {
    #[default]
    #[unit("W/m", "watts per metre", by = WATT_PER_METRE)]
    WattPerMetre,
    #[unit("W/cm", "watts per centimetre", by = WATT_PER_CENTIMETRE)]
    WattPerCentimetre,
    #[unit("W/mm", "watts per millimetre", by = WATT_PER_MILLIMETRE)]
    WattPerMillimetre,
}

/// The units of `VolumetricPowerDensity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = VolumetricPowerDensity<f64>)]
pub enum VolumetricPowerDensityUnit {
    #[default]
    #[unit("W/m³", "watts per cubic metre", by = WATT_PER_CUBIC_METRE)]
    WattPerCubicMetre,
}
