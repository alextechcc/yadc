use crate::{ISQ, StaticSIQuantity};
use consts::*;
use yadc::Dimension;

pub type MassDensity = StaticSIQuantity<
    {
        ISQ {
            m: -3,
            kg: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type LinearMassDensity = StaticSIQuantity<
    {
        ISQ {
            m: -1,
            kg: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ArealMassDensity = StaticSIQuantity<
    {
        ISQ {
            m: -2,
            kg: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type LinearNumberDensity = StaticSIQuantity<
    {
        ISQ {
            m: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ArealNumberDensity = StaticSIQuantity<
    {
        ISQ {
            m: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type VolumetricNumberDensity = StaticSIQuantity<
    {
        ISQ {
            m: -3,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type LinearNumberRate = StaticSIQuantity<
    {
        ISQ {
            m: -1,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ArealNumberRate = StaticSIQuantity<
    {
        ISQ {
            m: -2,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type VolumetricNumberRate = StaticSIQuantity<
    {
        ISQ {
            m: -3,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type LinearDensityOfStates = StaticSIQuantity<
    {
        ISQ {
            m: -3,
            kg: -1,
            s: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ArealDensityOfStates = StaticSIQuantity<
    {
        ISQ {
            m: -4,
            kg: -1,
            s: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type VolumetricDensityOfStates = StaticSIQuantity<
    {
        ISQ {
            m: -5,
            kg: -1,
            s: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type LinearPowerDensity = StaticSIQuantity<
    {
        ISQ {
            m: 1,
            kg: 1,
            s: -3,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type VolumetricPowerDensity = StaticSIQuantity<
    {
        ISQ {
            m: -1,
            kg: 1,
            s: -3,
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
    use crate::base::{KILOGRAM, METRE, ONE, SECOND};

    /// One `YottagramPerCubicMetre` (Yg/m³), as a `MassDensity`.
    pub const YOTTAGRAM_PER_CUBIC_METRE: MassDensity = 1e+21 * KILOGRAM_PER_CUBIC_METRE;

    /// One `ZettagramPerCubicMetre` (Zg/m³), as a `MassDensity`.
    pub const ZETTAGRAM_PER_CUBIC_METRE: MassDensity = 1e+18 * KILOGRAM_PER_CUBIC_METRE;

    /// One `ExagramPerCubicMetre` (Eg/m³), as a `MassDensity`.
    pub const EXAGRAM_PER_CUBIC_METRE: MassDensity = 1000000000000000.0 * KILOGRAM_PER_CUBIC_METRE;

    /// One `PetagramPerCubicMetre` (Pg/m³), as a `MassDensity`.
    pub const PETAGRAM_PER_CUBIC_METRE: MassDensity = 1000000000000.0 * KILOGRAM_PER_CUBIC_METRE;

    /// One `TeragramPerCubicMetre` (Tg/m³), as a `MassDensity`.
    pub const TERAGRAM_PER_CUBIC_METRE: MassDensity = 1000000000.0 * KILOGRAM_PER_CUBIC_METRE;

    /// One `GigagramPerCubicMetre` (Gg/m³), as a `MassDensity`.
    pub const GIGAGRAM_PER_CUBIC_METRE: MassDensity = 1000000.0 * KILOGRAM_PER_CUBIC_METRE;

    /// One `MegagramPerCubicMetre` (Mg/m³), as a `MassDensity`.
    pub const MEGAGRAM_PER_CUBIC_METRE: MassDensity = 1000.0 * KILOGRAM_PER_CUBIC_METRE;

    /// One `KilogramPerCubicMetre` (kg/m³), as a `MassDensity`.
    pub const KILOGRAM_PER_CUBIC_METRE: MassDensity = KILOGRAM / (METRE * METRE * METRE);

    /// One `HectogramPerCubicMetre` (hg/m³), as a `MassDensity`.
    pub const HECTOGRAM_PER_CUBIC_METRE: MassDensity =
        0.10000000000000001 * KILOGRAM_PER_CUBIC_METRE;

    /// One `DecagramPerCubicMetre` (dag/m³), as a `MassDensity`.
    pub const DECAGRAM_PER_CUBIC_METRE: MassDensity = 0.01 * KILOGRAM_PER_CUBIC_METRE;

    /// One `GramPerCubicMetre` (g/m³), as a `MassDensity`.
    pub const GRAM_PER_CUBIC_METRE: MassDensity = 0.001 * KILOGRAM_PER_CUBIC_METRE;

    /// One `DecigramPerCubicMetre` (dg/m³), as a `MassDensity`.
    pub const DECIGRAM_PER_CUBIC_METRE: MassDensity = 0.0001 * KILOGRAM_PER_CUBIC_METRE;

    /// One `CentigramPerCubicMetre` (cg/m³), as a `MassDensity`.
    pub const CENTIGRAM_PER_CUBIC_METRE: MassDensity =
        1.0000000000000001e-05 * KILOGRAM_PER_CUBIC_METRE;

    /// One `MilligramPerCubicMetre` (mg/m³), as a `MassDensity`.
    pub const MILLIGRAM_PER_CUBIC_METRE: MassDensity =
        9.9999999999999995e-07 * KILOGRAM_PER_CUBIC_METRE;

    /// One `MicrogramPerCubicMetre` (µg/m³), as a `MassDensity`.
    pub const MICROGRAM_PER_CUBIC_METRE: MassDensity =
        9.9999999999999986e-10 * KILOGRAM_PER_CUBIC_METRE;

    /// One `NanogramPerCubicMetre` (ng/m³), as a `MassDensity`.
    pub const NANOGRAM_PER_CUBIC_METRE: MassDensity =
        9.9999999999999998e-13 * KILOGRAM_PER_CUBIC_METRE;

    /// One `PicogramPerCubicMetre` (pg/m³), as a `MassDensity`.
    pub const PICOGRAM_PER_CUBIC_METRE: MassDensity =
        1.0000000000000001e-15 * KILOGRAM_PER_CUBIC_METRE;

    /// One `FemtogramPerCubicMetre` (fg/m³), as a `MassDensity`.
    pub const FEMTOGRAM_PER_CUBIC_METRE: MassDensity =
        1.0000000000000001e-18 * KILOGRAM_PER_CUBIC_METRE;

    /// One `AttogramPerCubicMetre` (ag/m³), as a `MassDensity`.
    pub const ATTOGRAM_PER_CUBIC_METRE: MassDensity =
        1.0000000000000001e-21 * KILOGRAM_PER_CUBIC_METRE;

    /// One `ZeptogramPerCubicMetre` (zg/m³), as a `MassDensity`.
    pub const ZEPTOGRAM_PER_CUBIC_METRE: MassDensity =
        9.9999999999999992e-25 * KILOGRAM_PER_CUBIC_METRE;

    /// One `YoctogramPerCubicMetre` (yg/m³), as a `MassDensity`.
    pub const YOCTOGRAM_PER_CUBIC_METRE: MassDensity =
        9.9999999999999986e-28 * KILOGRAM_PER_CUBIC_METRE;

    /// One `CaratPerCubicMetre` (ct/m³), as a `MassDensity`.
    pub const CARAT_PER_CUBIC_METRE: MassDensity =
        0.00020000000000000001 * KILOGRAM_PER_CUBIC_METRE;

    /// One `GrainPerCubicMetre` (gr/m³), as a `MassDensity`.
    pub const GRAIN_PER_CUBIC_METRE: MassDensity =
        6.4798909999999995e-05 * KILOGRAM_PER_CUBIC_METRE;

    /// One `HundredweightLongPerCubicMetre` (cwt long/m³), as a `MassDensity`.
    pub const HUNDREDWEIGHT_LONG_PER_CUBIC_METRE: MassDensity =
        50.80234544 * KILOGRAM_PER_CUBIC_METRE;

    /// One `HundredweightShortPerCubicMetre` (cwt short/m³), as a `MassDensity`.
    pub const HUNDREDWEIGHT_SHORT_PER_CUBIC_METRE: MassDensity =
        45.359237 * KILOGRAM_PER_CUBIC_METRE;

    /// One `OuncePerCubicMetre` (oz/m³), as a `MassDensity`.
    pub const OUNCE_PER_CUBIC_METRE: MassDensity = 0.028349523125 * KILOGRAM_PER_CUBIC_METRE;

    /// One `OunceTroyPerCubicMetre` (oz t/m³), as a `MassDensity`.
    pub const OUNCE_TROY_PER_CUBIC_METRE: MassDensity = 0.0311034768 * KILOGRAM_PER_CUBIC_METRE;

    /// One `PennyweightPerCubicMetre` (dwt/m³), as a `MassDensity`.
    pub const PENNYWEIGHT_PER_CUBIC_METRE: MassDensity = 0.00155517384 * KILOGRAM_PER_CUBIC_METRE;

    /// One `PoundPerCubicMetre` (lb/m³), as a `MassDensity`.
    pub const POUND_PER_CUBIC_METRE: MassDensity = 0.45359237 * KILOGRAM_PER_CUBIC_METRE;

    /// One `PoundTroyPerCubicMetre` (lb t/m³), as a `MassDensity`.
    pub const POUND_TROY_PER_CUBIC_METRE: MassDensity = 0.3732417216 * KILOGRAM_PER_CUBIC_METRE;

    /// One `SlugPerCubicMetre` (slug/m³), as a `MassDensity`.
    pub const SLUG_PER_CUBIC_METRE: MassDensity =
        (0.45359237 * 9.80665 / 0.3048) * KILOGRAM_PER_CUBIC_METRE;

    /// One `TonAssayPerCubicMetre` (AT/m³), as a `MassDensity`.
    pub const TON_ASSAY_PER_CUBIC_METRE: MassDensity = (0.175 / 6.0) * KILOGRAM_PER_CUBIC_METRE;

    /// One `TonLongPerCubicMetre` (2240 lb/m³), as a `MassDensity`.
    pub const TON_LONG_PER_CUBIC_METRE: MassDensity = 1016.0469088 * KILOGRAM_PER_CUBIC_METRE;

    /// One `TonShortPerCubicMetre` (2000 lb/m³), as a `MassDensity`.
    pub const TON_SHORT_PER_CUBIC_METRE: MassDensity = 907.18474 * KILOGRAM_PER_CUBIC_METRE;

    /// One `TonPerCubicMetre` (t/m³), as a `MassDensity`.
    pub const TON_PER_CUBIC_METRE: MassDensity = 1000.0 * KILOGRAM_PER_CUBIC_METRE;

    /// One `GrainPerGallon` (gr/gal), as a `MassDensity`.
    pub const GRAIN_PER_GALLON: MassDensity =
        (0.00006479891 / 0.003785411784) * KILOGRAM_PER_CUBIC_METRE;

    /// One `GramPerCubicCentimetre` (g/cm³), as a `MassDensity`.
    pub const GRAM_PER_CUBIC_CENTIMETRE: MassDensity = 1000.0 * KILOGRAM_PER_CUBIC_METRE;

    /// One `OuncePerCubicInch` (oz/in³), as a `MassDensity`.
    pub const OUNCE_PER_CUBIC_INCH: MassDensity =
        (0.028349523125 / (0.0254 * 0.0254 * 0.0254)) * KILOGRAM_PER_CUBIC_METRE;

    /// One `OuncePerGallonImperial` (oz/gal (UK)), as a `MassDensity`.
    pub const OUNCE_PER_GALLON_IMPERIAL: MassDensity =
        (0.028349523125 / 0.00454609) * KILOGRAM_PER_CUBIC_METRE;

    /// One `OuncePerGallon` (oz/gal), as a `MassDensity`.
    pub const OUNCE_PER_GALLON: MassDensity =
        (0.028349523125 / 0.003785411784) * KILOGRAM_PER_CUBIC_METRE;

    /// One `PoundPerCubicFoot` (lb/ft³), as a `MassDensity`.
    pub const POUND_PER_CUBIC_FOOT: MassDensity = 16.018463373960138 * KILOGRAM_PER_CUBIC_METRE;

    /// One `PoundPerCubicInch` (lb/in³), as a `MassDensity`.
    pub const POUND_PER_CUBIC_INCH: MassDensity = 27679.90471020312 * KILOGRAM_PER_CUBIC_METRE;

    /// One `PoundPerCubicYard` (lb/yd³), as a `MassDensity`.
    pub const POUND_PER_CUBIC_YARD: MassDensity = 0.5932764212577829 * KILOGRAM_PER_CUBIC_METRE;

    /// One `PoundPerGallonImperial` (lb/gal (UK)), as a `MassDensity`.
    pub const POUND_PER_GALLON_IMPERIAL: MassDensity = 99.7763726631017 * KILOGRAM_PER_CUBIC_METRE;

    /// One `PoundPerGallon` (lb/gal), as a `MassDensity`.
    pub const POUND_PER_GALLON: MassDensity = 119.82642731689663 * KILOGRAM_PER_CUBIC_METRE;

    /// One `SlugPerCubicFoot` (slug/ft³), as a `MassDensity`.
    pub const SLUG_PER_CUBIC_FOOT: MassDensity =
        ((0.45359237 * 9.80665 / 0.3048) / (0.3048 * 0.3048 * 0.3048)) * KILOGRAM_PER_CUBIC_METRE;

    /// One `TonLongPerCubicYard` (2240 lb/yd³), as a `MassDensity`.
    pub const TON_LONG_PER_CUBIC_YARD: MassDensity = 1328.9391836174339 * KILOGRAM_PER_CUBIC_METRE;

    /// One `TonShortPerCubicYard` (2000 lb/yd³), as a `MassDensity`.
    pub const TON_SHORT_PER_CUBIC_YARD: MassDensity = 1186.552842515566 * KILOGRAM_PER_CUBIC_METRE;

    /// One `TonLongPerCubicFoot` (2240 lb/ft³), as a `MassDensity`.
    pub const TON_LONG_PER_CUBIC_FOOT: MassDensity = 35881.35795767071 * KILOGRAM_PER_CUBIC_METRE;

    /// One `TonShortPerCubicFoot` (2000 lb/ft³), as a `MassDensity`.
    pub const TON_SHORT_PER_CUBIC_FOOT: MassDensity = 32036.92674792028 * KILOGRAM_PER_CUBIC_METRE;

    /// One `KilogramPerMetre` (kg/m), as a `LinearMassDensity`.
    pub const KILOGRAM_PER_METRE: LinearMassDensity = KILOGRAM / METRE;

    /// One `GramPerKilometre` (g/km), as a `LinearMassDensity`.
    pub const GRAM_PER_KILOMETRE: LinearMassDensity = 9.9999999999999995e-07 * KILOGRAM_PER_METRE;

    /// One `GramPerCentimetre` (g/cm), as a `LinearMassDensity`.
    pub const GRAM_PER_CENTIMETRE: LinearMassDensity = 0.10000000000000001 * KILOGRAM_PER_METRE;

    /// One `OuncePerFoot` (oz/ft), as a `LinearMassDensity`.
    pub const OUNCE_PER_FOOT: LinearMassDensity = (0.028349523125 / 0.3048) * KILOGRAM_PER_METRE;

    /// One `OuncePerInch` (oz/in), as a `LinearMassDensity`.
    pub const OUNCE_PER_INCH: LinearMassDensity = (0.028349523125 / 0.0254) * KILOGRAM_PER_METRE;

    /// One `PoundPerYard` (lb/yd), as a `LinearMassDensity`.
    pub const POUND_PER_YARD: LinearMassDensity = 0.4960546478565179 * KILOGRAM_PER_METRE;

    /// One `PoundPerFoot` (lb/ft), as a `LinearMassDensity`.
    pub const POUND_PER_FOOT: LinearMassDensity = 1.4881639435695537 * KILOGRAM_PER_METRE;

    /// One `PoundPerInch` (lb/in), as a `LinearMassDensity`.
    pub const POUND_PER_INCH: LinearMassDensity = 17.857967322834646 * KILOGRAM_PER_METRE;

    /// One `KilogramPerSquareMetre` (kg/m²), as an `ArealMassDensity`.
    pub const KILOGRAM_PER_SQUARE_METRE: ArealMassDensity = KILOGRAM / (METRE * METRE);

    /// One `GramPerSquareMetre` (g/m²), as an `ArealMassDensity`.
    pub const GRAM_PER_SQUARE_METRE: ArealMassDensity = 0.001 * KILOGRAM_PER_SQUARE_METRE;

    /// One `OuncePerSquareFoot` (oz/ft²), as an `ArealMassDensity`.
    pub const OUNCE_PER_SQUARE_FOOT: ArealMassDensity =
        (0.028349523125 / (0.3048 * 0.3048)) * KILOGRAM_PER_SQUARE_METRE;

    /// One `PerKilometre` (km⁻¹), as a `LinearNumberDensity`.
    pub const PER_KILOMETRE: LinearNumberDensity = 0.001 * PER_METRE;

    /// One `PerMetre` (m⁻¹), as a `LinearNumberDensity`.
    pub const PER_METRE: LinearNumberDensity = ONE / METRE;

    /// One `PerDecimetre` (dm⁻¹), as a `LinearNumberDensity`.
    pub const PER_DECIMETRE: LinearNumberDensity = 10.0 * PER_METRE;

    /// One `PerCentimetre` (cm⁻¹), as a `LinearNumberDensity`.
    pub const PER_CENTIMETRE: LinearNumberDensity = 100.0 * PER_METRE;

    /// One `PerMillimetre` (mm⁻¹), as a `LinearNumberDensity`.
    pub const PER_MILLIMETRE: LinearNumberDensity = 1000.0 * PER_METRE;

    /// One `PerFoot` (ft⁻¹), as a `LinearNumberDensity`.
    pub const PER_FOOT: LinearNumberDensity = 3.280839895013123 * PER_METRE;

    /// One `PerFootSurvey` (ft (U.S. survey)), as a `LinearNumberDensity`.
    pub const PER_FOOT_SURVEY: LinearNumberDensity = (1.0 / (1200.0 / 3937.0)) * PER_METRE;

    /// One `PerInch` (in⁻¹), as a `LinearNumberDensity`.
    pub const PER_INCH: LinearNumberDensity = 39.370078740157481 * PER_METRE;

    /// One `PerMile` (mi⁻¹), as a `LinearNumberDensity`.
    pub const PER_MILE: LinearNumberDensity = 0.00062137119223733392 * PER_METRE;

    /// One `PerMileSurvey` (mi⁻¹ (U.S. survey)), as a `LinearNumberDensity`.
    pub const PER_MILE_SURVEY: LinearNumberDensity = (1.0 / (6336000.0 / 3937.0)) * PER_METRE;

    /// One `PerNauticalMile` (M⁻¹), as a `LinearNumberDensity`.
    pub const PER_NAUTICAL_MILE: LinearNumberDensity = 0.00053995680345572358 * PER_METRE;

    /// One `PerYard` (yd⁻¹), as a `LinearNumberDensity`.
    pub const PER_YARD: LinearNumberDensity = 1.0936132983377078 * PER_METRE;

    /// One `PerSquareMetre` (m⁻²), as an `ArealNumberDensity`.
    pub const PER_SQUARE_METRE: ArealNumberDensity = ONE / (METRE * METRE);

    /// One `PerAcre` (ac⁻²), as an `ArealNumberDensity`.
    pub const PER_ACRE: ArealNumberDensity =
        (1.0 / (43560.0 * (1200.0 / 3937.0) * (1200.0 / 3937.0))) * PER_SQUARE_METRE;

    /// One `PerAre` (a⁻²), as an `ArealNumberDensity`.
    pub const PER_ARE: ArealNumberDensity = 0.01 * PER_SQUARE_METRE;

    /// One `PerBarn` (b⁻²), as an `ArealNumberDensity`.
    pub const PER_BARN: ArealNumberDensity = 9.9999999999999996e+27 * PER_SQUARE_METRE;

    /// One `PerCircularMil` (cmil⁻²), as an `ArealNumberDensity`.
    pub const PER_CIRCULAR_MIL: ArealNumberDensity =
        (4.0 / (core::f64::consts::PI * 0.0000254 * 0.0000254)) * PER_SQUARE_METRE;

    /// One `PerHectare` (ha⁻²), as an `ArealNumberDensity`.
    pub const PER_HECTARE: ArealNumberDensity = 0.0001 * PER_SQUARE_METRE;

    /// One `PerSquareFoot` (ft⁻²), as an `ArealNumberDensity`.
    pub const PER_SQUARE_FOOT: ArealNumberDensity = 10.763910416709722 * PER_SQUARE_METRE;

    /// One `PerSquareInch` (in⁻²), as an `ArealNumberDensity`.
    pub const PER_SQUARE_INCH: ArealNumberDensity = 1550.0031000062002 * PER_SQUARE_METRE;

    /// One `PerSquareMile` (mi⁻²), as an `ArealNumberDensity`.
    pub const PER_SQUARE_MILE: ArealNumberDensity = 3.8610215854244582e-07 * PER_SQUARE_METRE;

    /// One `PerSquareYard` (yd⁻²), as an `ArealNumberDensity`.
    pub const PER_SQUARE_YARD: ArealNumberDensity = 1.1959900463010802 * PER_SQUARE_METRE;

    /// One `PerCubicMetre` (m⁻³), as a `VolumetricNumberDensity`.
    pub const PER_CUBIC_METRE: VolumetricNumberDensity = ONE / (METRE * METRE * METRE);

    /// One `PerCubicFoot` (ft⁻³), as a `VolumetricNumberDensity`.
    pub const PER_CUBIC_FOOT: VolumetricNumberDensity =
        (1.0 / (0.3048 * 0.3048 * 0.3048)) * PER_CUBIC_METRE;

    /// One `PerCubicInch` (in⁻³), as a `VolumetricNumberDensity`.
    pub const PER_CUBIC_INCH: VolumetricNumberDensity =
        (1.0 / (0.0254 * 0.0254 * 0.0254)) * PER_CUBIC_METRE;

    /// One `PerCubicMile` (mi⁻³), as a `VolumetricNumberDensity`.
    pub const PER_CUBIC_MILE: VolumetricNumberDensity =
        (1.0 / (1609.344 * 1609.344 * 1609.344)) * PER_CUBIC_METRE;

    /// One `PerCubicYard` (yd⁻³), as a `VolumetricNumberDensity`.
    pub const PER_CUBIC_YARD: VolumetricNumberDensity =
        (1.0 / (0.9144 * 0.9144 * 0.9144)) * PER_CUBIC_METRE;

    /// One `PerFluidOunce` (per fl oz), as a `VolumetricNumberDensity`.
    pub const PER_FLUID_OUNCE: VolumetricNumberDensity =
        (1.0 / 0.0000295735295625) * PER_CUBIC_METRE;

    /// One `PerFluidOunceImperial` (per fl oz (UK)), as a `VolumetricNumberDensity`.
    pub const PER_FLUID_OUNCE_IMPERIAL: VolumetricNumberDensity =
        (1.0 / 0.0000284130625) * PER_CUBIC_METRE;

    /// One `PerGallonImperial` (per gal (UK)), as a `VolumetricNumberDensity`.
    pub const PER_GALLON_IMPERIAL: VolumetricNumberDensity = 219.96924829908778 * PER_CUBIC_METRE;

    /// One `PerGallon` (per gal), as a `VolumetricNumberDensity`.
    pub const PER_GALLON: VolumetricNumberDensity = (1.0 / 0.003785411784) * PER_CUBIC_METRE;

    /// One `PerLitre` (L⁻¹), as a `VolumetricNumberDensity`.
    pub const PER_LITRE: VolumetricNumberDensity = 1000.0 * PER_CUBIC_METRE;

    /// One `PerKilometreSecond` (km⁻¹ · s⁻¹), as a `LinearNumberRate`.
    pub const PER_KILOMETRE_SECOND: LinearNumberRate = 0.001 * PER_METRE_SECOND;

    /// One `PerMetreSecond` (m⁻¹ · s⁻¹), as a `LinearNumberRate`.
    pub const PER_METRE_SECOND: LinearNumberRate = ONE / (METRE * SECOND);

    /// One `PerCentimetreSecond` (cm⁻¹ · s⁻¹), as a `LinearNumberRate`.
    pub const PER_CENTIMETRE_SECOND: LinearNumberRate = 100.0 * PER_METRE_SECOND;

    /// One `PerMillimetreSecond` (mm⁻¹ · s⁻¹), as a `LinearNumberRate`.
    pub const PER_MILLIMETRE_SECOND: LinearNumberRate = 1000.0 * PER_METRE_SECOND;

    /// One `PerFootSecond` (ft⁻¹ · s⁻¹), as a `LinearNumberRate`.
    pub const PER_FOOT_SECOND: LinearNumberRate = 3.280839895013123 * PER_METRE_SECOND;

    /// One `PerFootSurveySecond` (ft⁻¹ (U.S. survey) · s⁻¹), as a `LinearNumberRate`.
    pub const PER_FOOT_SURVEY_SECOND: LinearNumberRate =
        (1.0 / (1200.0 / 3937.0)) * PER_METRE_SECOND;

    /// One `PerInchSecond` (in⁻¹ · s⁻¹), as a `LinearNumberRate`.
    pub const PER_INCH_SECOND: LinearNumberRate = 39.370078740157481 * PER_METRE_SECOND;

    /// One `PerMileSecond` (mi⁻¹ · s⁻¹), as a `LinearNumberRate`.
    pub const PER_MILE_SECOND: LinearNumberRate = 0.00062137119223733392 * PER_METRE_SECOND;

    /// One `PerMileSurveySecond` (mi⁻¹ (U.S. survey) · s⁻¹), as a `LinearNumberRate`.
    pub const PER_MILE_SURVEY_SECOND: LinearNumberRate =
        (1.0 / (6336000.0 / 3937.0)) * PER_METRE_SECOND;

    /// One `PerYardSecond` (yd⁻¹ · s⁻¹), as a `LinearNumberRate`.
    pub const PER_YARD_SECOND: LinearNumberRate = 1.0936132983377078 * PER_METRE_SECOND;

    /// One `PerSquareMetreSecond` (m⁻² · s⁻¹), as an `ArealNumberRate`.
    pub const PER_SQUARE_METRE_SECOND: ArealNumberRate = ONE / (METRE * METRE * SECOND);

    /// One `PerAcreSecond` (ac⁻¹ · s⁻¹), as an `ArealNumberRate`.
    pub const PER_ACRE_SECOND: ArealNumberRate =
        (1.0 / (43560.0 * (1200.0 / 3937.0) * (1200.0 / 3937.0))) * PER_SQUARE_METRE_SECOND;

    /// One `PerAreSecond` (a⁻¹ · s⁻¹), as an `ArealNumberRate`.
    pub const PER_ARE_SECOND: ArealNumberRate = 0.01 * PER_SQUARE_METRE_SECOND;

    /// One `PerBarnSecond` (b⁻¹ · s⁻¹), as an `ArealNumberRate`.
    pub const PER_BARN_SECOND: ArealNumberRate = 9.9999999999999996e+27 * PER_SQUARE_METRE_SECOND;

    /// One `PerCircularMilSecond` (cmil⁻¹ · s⁻¹), as an `ArealNumberRate`.
    pub const PER_CIRCULAR_MIL_SECOND: ArealNumberRate =
        (4.0 / (core::f64::consts::PI * 0.0000254 * 0.0000254)) * PER_SQUARE_METRE_SECOND;

    /// One `PerHectareSecond` (ha⁻¹ · s⁻¹), as an `ArealNumberRate`.
    pub const PER_HECTARE_SECOND: ArealNumberRate = 0.0001 * PER_SQUARE_METRE_SECOND;

    /// One `PerSquareFootSecond` (ft⁻² · s⁻¹), as an `ArealNumberRate`.
    pub const PER_SQUARE_FOOT_SECOND: ArealNumberRate =
        10.763910416709722 * PER_SQUARE_METRE_SECOND;

    /// One `PerSquareInchSecond` (in⁻² · s⁻¹), as an `ArealNumberRate`.
    pub const PER_SQUARE_INCH_SECOND: ArealNumberRate =
        1550.0031000062002 * PER_SQUARE_METRE_SECOND;

    /// One `PerSquareMileSecond` (mi⁻² · s⁻¹), as an `ArealNumberRate`.
    pub const PER_SQUARE_MILE_SECOND: ArealNumberRate =
        3.8610215854244582e-07 * PER_SQUARE_METRE_SECOND;

    /// One `PerSquareYardSecond` (yd⁻² · s⁻¹), as an `ArealNumberRate`.
    pub const PER_SQUARE_YARD_SECOND: ArealNumberRate =
        1.1959900463010802 * PER_SQUARE_METRE_SECOND;

    /// One `PerCubicMetreSecond` (m⁻³ · s⁻¹), as a `VolumetricNumberRate`.
    pub const PER_CUBIC_METRE_SECOND: VolumetricNumberRate = ONE / (METRE * METRE * METRE * SECOND);

    /// One `PerCubicFootSecond` (ft⁻³ · s⁻¹), as a `VolumetricNumberRate`.
    pub const PER_CUBIC_FOOT_SECOND: VolumetricNumberRate =
        (1.0 / (0.3048 * 0.3048 * 0.3048)) * PER_CUBIC_METRE_SECOND;

    /// One `PerCubicInchSecond` (in⁻³ · s⁻¹), as a `VolumetricNumberRate`.
    pub const PER_CUBIC_INCH_SECOND: VolumetricNumberRate =
        (1.0 / (0.0254 * 0.0254 * 0.0254)) * PER_CUBIC_METRE_SECOND;

    /// One `PerCubicMileSecond` (mi⁻³ · s⁻¹), as a `VolumetricNumberRate`.
    pub const PER_CUBIC_MILE_SECOND: VolumetricNumberRate =
        (1.0 / (1609.344 * 1609.344 * 1609.344)) * PER_CUBIC_METRE_SECOND;

    /// One `PerCubicYardSecond` (yd⁻³ · s⁻¹), as a `VolumetricNumberRate`.
    pub const PER_CUBIC_YARD_SECOND: VolumetricNumberRate =
        (1.0 / (0.9144 * 0.9144 * 0.9144)) * PER_CUBIC_METRE_SECOND;

    /// One `PerFluidOunceSecond` (fl oz⁻¹ · s⁻¹), as a `VolumetricNumberRate`.
    pub const PER_FLUID_OUNCE_SECOND: VolumetricNumberRate =
        (1.0 / 0.0000295735295625) * PER_CUBIC_METRE_SECOND;

    /// One `PerFluidOunceImperialSecond` (fl oz⁻¹ (UK) · s⁻¹), as a `VolumetricNumberRate`.
    pub const PER_FLUID_OUNCE_IMPERIAL_SECOND: VolumetricNumberRate =
        (1.0 / 0.0000284130625) * PER_CUBIC_METRE_SECOND;

    /// One `PerGallonImperialSecond` (gal⁻¹ (UK) · s⁻¹), as a `VolumetricNumberRate`.
    pub const PER_GALLON_IMPERIAL_SECOND: VolumetricNumberRate =
        219.96924829908778 * PER_CUBIC_METRE_SECOND;

    /// One `PerGallonSecond` (gal⁻¹ · s⁻¹), as a `VolumetricNumberRate`.
    pub const PER_GALLON_SECOND: VolumetricNumberRate =
        (1.0 / 0.003785411784) * PER_CUBIC_METRE_SECOND;

    /// One `PerLitreSecond` (L⁻¹ · s⁻¹), as a `VolumetricNumberRate`.
    pub const PER_LITRE_SECOND: VolumetricNumberRate = 1000.0 * PER_CUBIC_METRE_SECOND;

    /// One `BecquerelPerCubicMetre` (Bq/m³), as a `VolumetricNumberRate`.
    pub const BECQUEREL_PER_CUBIC_METRE: VolumetricNumberRate = PER_CUBIC_METRE_SECOND;

    /// One `CuriePerCubicMetre` (Ci/m³), as a `VolumetricNumberRate`.
    pub const CURIE_PER_CUBIC_METRE: VolumetricNumberRate = 37000000000.0 * PER_CUBIC_METRE_SECOND;

    /// One `DisintegrationsPerMinutePerCubicMetre` (dpm/m³), as a `VolumetricNumberRate`.
    pub const DISINTEGRATIONS_PER_MINUTE_PER_CUBIC_METRE: VolumetricNumberRate =
        0.016666666666666666 * PER_CUBIC_METRE_SECOND;

    /// One `StatePerMetreJoule` (1/(m · J)), as a `LinearDensityOfStates`.
    pub const STATE_PER_METRE_JOULE: LinearDensityOfStates =
        SECOND * SECOND / (METRE * METRE * METRE * KILOGRAM);

    /// One `StatePerCentimetreJoule` (1/(cm · J)), as a `LinearDensityOfStates`.
    pub const STATE_PER_CENTIMETRE_JOULE: LinearDensityOfStates = 100.0 * STATE_PER_METRE_JOULE;

    /// One `StatePerSquareMetreJoule` (1/(m² · J)), as an `ArealDensityOfStates`.
    pub const STATE_PER_SQUARE_METRE_JOULE: ArealDensityOfStates =
        SECOND * SECOND / (METRE * METRE * METRE * METRE * KILOGRAM);

    /// One `StatePerCubicMetreJoule` (1/(m³ · J)), as a `VolumetricDensityOfStates`.
    pub const STATE_PER_CUBIC_METRE_JOULE: VolumetricDensityOfStates =
        SECOND * SECOND / (METRE * METRE * METRE * METRE * METRE * KILOGRAM);

    /// One `WattPerMetre` (W/m), as a `LinearPowerDensity`.
    pub const WATT_PER_METRE: LinearPowerDensity = METRE * KILOGRAM / (SECOND * SECOND * SECOND);

    /// One `WattPerCentimetre` (W/cm), as a `LinearPowerDensity`.
    pub const WATT_PER_CENTIMETRE: LinearPowerDensity = 100.0 * WATT_PER_METRE;

    /// One `WattPerMillimetre` (W/mm), as a `LinearPowerDensity`.
    pub const WATT_PER_MILLIMETRE: LinearPowerDensity = 1000.0 * WATT_PER_METRE;

    /// One `WattPerCubicMetre` (W/m³), as a `VolumetricPowerDensity`.
    pub const WATT_PER_CUBIC_METRE: VolumetricPowerDensity =
        KILOGRAM / (METRE * SECOND * SECOND * SECOND);
}

/// The units of `MassDensity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassDensity)]
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
#[unit(of = LinearMassDensity)]
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
#[unit(of = ArealMassDensity)]
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
#[unit(of = LinearNumberDensity)]
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
#[unit(of = ArealNumberDensity)]
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
#[unit(of = VolumetricNumberDensity)]
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
#[unit(of = LinearNumberRate)]
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
#[unit(of = ArealNumberRate)]
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
#[unit(of = VolumetricNumberRate)]
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
#[unit(of = LinearDensityOfStates)]
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
#[unit(of = ArealDensityOfStates)]
pub enum ArealDensityOfStatesUnit {
    #[default]
    #[unit("1/(m² · J)", "states per square metre joule", by = STATE_PER_SQUARE_METRE_JOULE)]
    StatePerSquareMetreJoule,
}

/// The units of `VolumetricDensityOfStates`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = VolumetricDensityOfStates)]
pub enum VolumetricDensityOfStatesUnit {
    #[default]
    #[unit("1/(m³ · J)", "states per cubic metre joule", by = STATE_PER_CUBIC_METRE_JOULE)]
    StatePerCubicMetreJoule,
}

/// The units of `LinearPowerDensity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LinearPowerDensity)]
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
#[unit(of = VolumetricPowerDensity)]
pub enum VolumetricPowerDensityUnit {
    #[default]
    #[unit("W/m³", "watts per cubic metre", by = WATT_PER_CUBIC_METRE)]
    WattPerCubicMetre,
}
