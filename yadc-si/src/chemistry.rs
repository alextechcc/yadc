use crate::{ISQ, StaticSIQuantity};
use consts::*;
use yadc::Dimension;

pub type AmountOfSubstance = StaticSIQuantity<
    {
        ISQ {
            mol: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type MolarConcentration = StaticSIQuantity<
    {
        ISQ {
            m: -3,
            mol: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type MolarEnergy = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -2,
            mol: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type MolarFlux = StaticSIQuantity<
    {
        ISQ {
            m: -2,
            s: -1,
            mol: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type MolarHeatCapacity = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -2,
            K: -1,
            mol: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type MolarMass = StaticSIQuantity<
    {
        ISQ {
            kg: 1,
            mol: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type MolarRadioactivity = StaticSIQuantity<
    {
        ISQ {
            s: -1,
            mol: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type MolarVolume = StaticSIQuantity<
    {
        ISQ {
            m: 3,
            mol: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type Molality = StaticSIQuantity<
    {
        ISQ {
            kg: -1,
            mol: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type MassConcentration = StaticSIQuantity<
    {
        ISQ {
            m: -3,
            kg: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type CatalyticActivity = StaticSIQuantity<
    {
        ISQ {
            s: -1,
            mol: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type CatalyticActivityConcentration = StaticSIQuantity<
    {
        ISQ {
            m: -3,
            s: -1,
            mol: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type Radioactivity = StaticSIQuantity<
    {
        ISQ {
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type SpecificRadioactivity = StaticSIQuantity<
    {
        ISQ {
            kg: -1,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type InverseAmountOfSubstance = StaticSIQuantity<
    {
        ISQ {
            mol: -1,
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
    use crate::base::{KELVIN, KILOGRAM, METRE, ONE, SECOND};

    /// The mole, the SI base unit of amount of substance.
    pub use crate::base::MOLE;

    /// One `Yottamole` (Ymol), as an `AmountOfSubstance`.
    pub const YOTTAMOLE: AmountOfSubstance = 9.9999999999999998e+23 * MOLE;

    /// One `Zettamole` (Zmol), as an `AmountOfSubstance`.
    pub const ZETTAMOLE: AmountOfSubstance = 1e+21 * MOLE;

    /// One `Examole` (Emol), as an `AmountOfSubstance`.
    pub const EXAMOLE: AmountOfSubstance = 1e+18 * MOLE;

    /// One `Petamole` (Pmol), as an `AmountOfSubstance`.
    pub const PETAMOLE: AmountOfSubstance = 1000000000000000.0 * MOLE;

    /// One `Teramole` (Tmol), as an `AmountOfSubstance`.
    pub const TERAMOLE: AmountOfSubstance = 1000000000000.0 * MOLE;

    /// One `Gigamole` (Gmol), as an `AmountOfSubstance`.
    pub const GIGAMOLE: AmountOfSubstance = 1000000000.0 * MOLE;

    /// One `Megamole` (Mmol), as an `AmountOfSubstance`.
    pub const MEGAMOLE: AmountOfSubstance = 1000000.0 * MOLE;

    /// One `Kilomole` (kmol), as an `AmountOfSubstance`.
    pub const KILOMOLE: AmountOfSubstance = 1000.0 * MOLE;

    /// One `Hectomole` (hmol), as an `AmountOfSubstance`.
    pub const HECTOMOLE: AmountOfSubstance = 100.0 * MOLE;

    /// One `Decamole` (damol), as an `AmountOfSubstance`.
    pub const DECAMOLE: AmountOfSubstance = 10.0 * MOLE;

    /// One `Decimole` (dmol), as an `AmountOfSubstance`.
    pub const DECIMOLE: AmountOfSubstance = 0.10000000000000001 * MOLE;

    /// One `Centimole` (cmol), as an `AmountOfSubstance`.
    pub const CENTIMOLE: AmountOfSubstance = 0.01 * MOLE;

    /// One `Millimole` (mmol), as an `AmountOfSubstance`.
    pub const MILLIMOLE: AmountOfSubstance = 0.001 * MOLE;

    /// One `Micromole` (µmol), as an `AmountOfSubstance`.
    pub const MICROMOLE: AmountOfSubstance = 9.9999999999999995e-07 * MOLE;

    /// One `Nanomole` (nmol), as an `AmountOfSubstance`.
    pub const NANOMOLE: AmountOfSubstance = 1.0000000000000001e-09 * MOLE;

    /// One `Picomole` (pmol), as an `AmountOfSubstance`.
    pub const PICOMOLE: AmountOfSubstance = 9.9999999999999998e-13 * MOLE;

    /// One `Femtomole` (fmol), as an `AmountOfSubstance`.
    pub const FEMTOMOLE: AmountOfSubstance = 1.0000000000000001e-15 * MOLE;

    /// One `Attomole` (amol), as an `AmountOfSubstance`.
    pub const ATTOMOLE: AmountOfSubstance = 1.0000000000000001e-18 * MOLE;

    /// One `Zeptomole` (zmol), as an `AmountOfSubstance`.
    pub const ZEPTOMOLE: AmountOfSubstance = 9.9999999999999991e-22 * MOLE;

    /// One `Yoctomole` (ymol), as an `AmountOfSubstance`.
    pub const YOCTOMOLE: AmountOfSubstance = 9.9999999999999992e-25 * MOLE;

    /// One `Particle` (particle), as an `AmountOfSubstance`.
    pub const PARTICLE: AmountOfSubstance = 1.6605390671738466e-24 * MOLE;

    /// One `YottamolePerCubicMetre` (Ymol/m³), as a `MolarConcentration`.
    pub const YOTTAMOLE_PER_CUBIC_METRE: MolarConcentration =
        9.9999999999999998e+23 * MOLE_PER_CUBIC_METRE;

    /// One `ZettamolePerCubicMetre` (Zmol/m³), as a `MolarConcentration`.
    pub const ZETTAMOLE_PER_CUBIC_METRE: MolarConcentration = 1e+21 * MOLE_PER_CUBIC_METRE;

    /// One `ExamolePerCubicMetre` (Emol/m³), as a `MolarConcentration`.
    pub const EXAMOLE_PER_CUBIC_METRE: MolarConcentration = 1e+18 * MOLE_PER_CUBIC_METRE;

    /// One `PetamolePerCubicMetre` (Pmol/m³), as a `MolarConcentration`.
    pub const PETAMOLE_PER_CUBIC_METRE: MolarConcentration =
        1000000000000000.0 * MOLE_PER_CUBIC_METRE;

    /// One `TeramolePerCubicMetre` (Tmol/m³), as a `MolarConcentration`.
    pub const TERAMOLE_PER_CUBIC_METRE: MolarConcentration = 1000000000000.0 * MOLE_PER_CUBIC_METRE;

    /// One `GigamolePerCubicMetre` (Gmol/m³), as a `MolarConcentration`.
    pub const GIGAMOLE_PER_CUBIC_METRE: MolarConcentration = 1000000000.0 * MOLE_PER_CUBIC_METRE;

    /// One `MegamolePerCubicMetre` (Mmol/m³), as a `MolarConcentration`.
    pub const MEGAMOLE_PER_CUBIC_METRE: MolarConcentration = 1000000.0 * MOLE_PER_CUBIC_METRE;

    /// One `KilomolePerCubicMetre` (kmol/m³), as a `MolarConcentration`.
    pub const KILOMOLE_PER_CUBIC_METRE: MolarConcentration = 1000.0 * MOLE_PER_CUBIC_METRE;

    /// One `HectomolePerCubicMetre` (hmol/m³), as a `MolarConcentration`.
    pub const HECTOMOLE_PER_CUBIC_METRE: MolarConcentration = 100.0 * MOLE_PER_CUBIC_METRE;

    /// One `DecamolePerCubicMetre` (damol/m³), as a `MolarConcentration`.
    pub const DECAMOLE_PER_CUBIC_METRE: MolarConcentration = 10.0 * MOLE_PER_CUBIC_METRE;

    /// One `MolePerCubicMetre` (mol/m³), as a `MolarConcentration`.
    pub const MOLE_PER_CUBIC_METRE: MolarConcentration = MOLE / (METRE * METRE * METRE);

    /// One `DecimolePerCubicMetre` (dmol/m³), as a `MolarConcentration`.
    pub const DECIMOLE_PER_CUBIC_METRE: MolarConcentration =
        0.10000000000000001 * MOLE_PER_CUBIC_METRE;

    /// One `CentimolePerCubicMetre` (cmol/m³), as a `MolarConcentration`.
    pub const CENTIMOLE_PER_CUBIC_METRE: MolarConcentration = 0.01 * MOLE_PER_CUBIC_METRE;

    /// One `MillimolePerCubicMetre` (mmol/m³), as a `MolarConcentration`.
    pub const MILLIMOLE_PER_CUBIC_METRE: MolarConcentration = 0.001 * MOLE_PER_CUBIC_METRE;

    /// One `MicromolePerCubicMetre` (µmol/m³), as a `MolarConcentration`.
    pub const MICROMOLE_PER_CUBIC_METRE: MolarConcentration =
        9.9999999999999995e-07 * MOLE_PER_CUBIC_METRE;

    /// One `NanomolePerCubicMetre` (nmol/m³), as a `MolarConcentration`.
    pub const NANOMOLE_PER_CUBIC_METRE: MolarConcentration =
        1.0000000000000001e-09 * MOLE_PER_CUBIC_METRE;

    /// One `PicomolePerCubicMetre` (pmol/m³), as a `MolarConcentration`.
    pub const PICOMOLE_PER_CUBIC_METRE: MolarConcentration =
        9.9999999999999998e-13 * MOLE_PER_CUBIC_METRE;

    /// One `FemtomolePerCubicMetre` (fmol/m³), as a `MolarConcentration`.
    pub const FEMTOMOLE_PER_CUBIC_METRE: MolarConcentration =
        1.0000000000000001e-15 * MOLE_PER_CUBIC_METRE;

    /// One `AttomolePerCubicMetre` (amol/m³), as a `MolarConcentration`.
    pub const ATTOMOLE_PER_CUBIC_METRE: MolarConcentration =
        1.0000000000000001e-18 * MOLE_PER_CUBIC_METRE;

    /// One `ZeptomolePerCubicMetre` (zmol/m³), as a `MolarConcentration`.
    pub const ZEPTOMOLE_PER_CUBIC_METRE: MolarConcentration =
        9.9999999999999991e-22 * MOLE_PER_CUBIC_METRE;

    /// One `YoctomolePerCubicMetre` (ymol/m³), as a `MolarConcentration`.
    pub const YOCTOMOLE_PER_CUBIC_METRE: MolarConcentration =
        9.9999999999999992e-25 * MOLE_PER_CUBIC_METRE;

    /// One `KilomolePerLitre` (kmol/L), as a `MolarConcentration`.
    pub const KILOMOLE_PER_LITRE: MolarConcentration = 1000000.0 * MOLE_PER_CUBIC_METRE;

    /// One `MolePerLitre` (mol/L), as a `MolarConcentration`.
    pub const MOLE_PER_LITRE: MolarConcentration = 1000.0 * MOLE_PER_CUBIC_METRE;

    /// One `MillimolePerLitre` (mmol/L), as a `MolarConcentration`.
    pub const MILLIMOLE_PER_LITRE: MolarConcentration = MOLE_PER_CUBIC_METRE;

    /// One `MicromolePerLitre` (μmol/L), as a `MolarConcentration`.
    pub const MICROMOLE_PER_LITRE: MolarConcentration = 0.001 * MOLE_PER_CUBIC_METRE;

    /// One `NanomolePerLitre` (nmol/L), as a `MolarConcentration`.
    pub const NANOMOLE_PER_LITRE: MolarConcentration =
        9.9999999999999995e-07 * MOLE_PER_CUBIC_METRE;

    /// One `PicomolePerLitre` (pmol/L), as a `MolarConcentration`.
    pub const PICOMOLE_PER_LITRE: MolarConcentration =
        1.0000000000000001e-09 * MOLE_PER_CUBIC_METRE;

    /// One `FemtomolePerLitre` (fmol/L), as a `MolarConcentration`.
    pub const FEMTOMOLE_PER_LITRE: MolarConcentration =
        9.9999999999999998e-13 * MOLE_PER_CUBIC_METRE;

    /// One `ParticlePerCubicMetre` (particle/m³), as a `MolarConcentration`.
    pub const PARTICLE_PER_CUBIC_METRE: MolarConcentration =
        1.6605390671738466e-24 * MOLE_PER_CUBIC_METRE;

    /// One `KilogramSquareMetrePerSecondSquaredMole` (kg · m²/(s² · mol)), as a `MolarEnergy`.
    pub const KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE: MolarEnergy =
        METRE * METRE * KILOGRAM / (SECOND * SECOND * MOLE);

    /// One `YottajoulePerMole` (YJ/mol), as a `MolarEnergy`.
    pub const YOTTAJOULE_PER_MOLE: MolarEnergy =
        9.9999999999999998e+23 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `ZettajoulePerMole` (ZJ/mol), as a `MolarEnergy`.
    pub const ZETTAJOULE_PER_MOLE: MolarEnergy =
        1e+21 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `ExajoulePerMole` (EJ/mol), as a `MolarEnergy`.
    pub const EXAJOULE_PER_MOLE: MolarEnergy =
        1e+18 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `PetajoulePerMole` (PJ/mol), as a `MolarEnergy`.
    pub const PETAJOULE_PER_MOLE: MolarEnergy =
        1000000000000000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `TerajoulePerMole` (TJ/mol), as a `MolarEnergy`.
    pub const TERAJOULE_PER_MOLE: MolarEnergy =
        1000000000000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `GigajoulePerMole` (GJ/mol), as a `MolarEnergy`.
    pub const GIGAJOULE_PER_MOLE: MolarEnergy =
        1000000000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `MegajoulePerMole` (MJ/mol), as a `MolarEnergy`.
    pub const MEGAJOULE_PER_MOLE: MolarEnergy =
        1000000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `KilojoulePerMole` (kJ/mol), as a `MolarEnergy`.
    pub const KILOJOULE_PER_MOLE: MolarEnergy =
        1000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `HectojoulePerMole` (hJ/mol), as a `MolarEnergy`.
    pub const HECTOJOULE_PER_MOLE: MolarEnergy =
        100.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `DecajoulePerMole` (daJ/mol), as a `MolarEnergy`.
    pub const DECAJOULE_PER_MOLE: MolarEnergy =
        10.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `JoulePerMole` (J/mol), as a `MolarEnergy`.
    pub const JOULE_PER_MOLE: MolarEnergy = KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `DecijoulePerMole` (dJ/mol), as a `MolarEnergy`.
    pub const DECIJOULE_PER_MOLE: MolarEnergy =
        0.10000000000000001 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `CentijoulePerMole` (cJ/mol), as a `MolarEnergy`.
    pub const CENTIJOULE_PER_MOLE: MolarEnergy =
        0.01 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `MillijoulePerMole` (mJ/mol), as a `MolarEnergy`.
    pub const MILLIJOULE_PER_MOLE: MolarEnergy =
        0.001 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `MicrojoulePerMole` (µJ/mol), as a `MolarEnergy`.
    pub const MICROJOULE_PER_MOLE: MolarEnergy =
        9.9999999999999995e-07 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `NanojoulePerMole` (nJ/mol), as a `MolarEnergy`.
    pub const NANOJOULE_PER_MOLE: MolarEnergy =
        1.0000000000000001e-09 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `PicojoulePerMole` (pJ/mol), as a `MolarEnergy`.
    pub const PICOJOULE_PER_MOLE: MolarEnergy =
        9.9999999999999998e-13 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `FemtojoulePerMole` (fJ/mol), as a `MolarEnergy`.
    pub const FEMTOJOULE_PER_MOLE: MolarEnergy =
        1.0000000000000001e-15 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `AttojoulePerMole` (aJ/mol), as a `MolarEnergy`.
    pub const ATTOJOULE_PER_MOLE: MolarEnergy =
        1.0000000000000001e-18 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `ZeptojoulePerMole` (zJ/mol), as a `MolarEnergy`.
    pub const ZEPTOJOULE_PER_MOLE: MolarEnergy =
        9.9999999999999991e-22 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `YoctojoulePerMole` (yJ/mol), as a `MolarEnergy`.
    pub const YOCTOJOULE_PER_MOLE: MolarEnergy =
        9.9999999999999992e-25 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `PetawattHourPerMole` (PW · h/mol), as a `MolarEnergy`.
    pub const PETAWATT_HOUR_PER_MOLE: MolarEnergy =
        3.6e+18 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `TerawattHourPerMole` (TW · h/mol), as a `MolarEnergy`.
    pub const TERAWATT_HOUR_PER_MOLE: MolarEnergy =
        3600000000000000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `GigawattHourPerMole` (GW · h/mol), as a `MolarEnergy`.
    pub const GIGAWATT_HOUR_PER_MOLE: MolarEnergy =
        3600000000000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `MegawattHourPerMole` (MW · h/mol), as a `MolarEnergy`.
    pub const MEGAWATT_HOUR_PER_MOLE: MolarEnergy =
        3600000000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `KilowattHourPerMole` (kW · h/mol), as a `MolarEnergy`.
    pub const KILOWATT_HOUR_PER_MOLE: MolarEnergy =
        3600000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `HectowattHourPerMole` (hW · h/mol), as a `MolarEnergy`.
    pub const HECTOWATT_HOUR_PER_MOLE: MolarEnergy =
        360000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `DecawattHourPerMole` (daW · h/mol), as a `MolarEnergy`.
    pub const DECAWATT_HOUR_PER_MOLE: MolarEnergy =
        36000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `WattHourPerMole` (W · h/mol), as a `MolarEnergy`.
    pub const WATT_HOUR_PER_MOLE: MolarEnergy =
        3600.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `MilliwattHourPerMole` (mW · h/mol), as a `MolarEnergy`.
    pub const MILLIWATT_HOUR_PER_MOLE: MolarEnergy =
        3.6000000000000001 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `MicrowattHourPerMole` (µW · h/mol), as a `MolarEnergy`.
    pub const MICROWATT_HOUR_PER_MOLE: MolarEnergy =
        0.0035999999999999999 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `BtuItPerMole` (Btu (IT)/mol), as a `MolarEnergy`.
    pub const BTU_IT_PER_MOLE: MolarEnergy =
        1055.05585262 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `BtuPerMole` (Btu/mol), as a `MolarEnergy`.
    pub const BTU_PER_MOLE: MolarEnergy =
        (4184.0 * 0.45359237 / 1.8) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `CalorieItPerMole` (cal (IT)/mol), as a `MolarEnergy`.
    pub const CALORIE_IT_PER_MOLE: MolarEnergy =
        4.1867999999999999 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `CaloriePerMole` (cal/mol), as a `MolarEnergy`.
    pub const CALORIE_PER_MOLE: MolarEnergy =
        4.1840000000000002 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `CalorieItNutritionPerMole` (Cal (IT)/mol), as a `MolarEnergy`.
    pub const CALORIE_IT_NUTRITION_PER_MOLE: MolarEnergy =
        4186.8000000000002 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `CalorieNutritionPerMole` (Cal/mol), as a `MolarEnergy`.
    pub const CALORIE_NUTRITION_PER_MOLE: MolarEnergy =
        4184.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `ElectronvoltPerMole` (eV/mol), as a `MolarEnergy`.
    pub const ELECTRONVOLT_PER_MOLE: MolarEnergy =
        1.6021766339999999e-19 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `ErgPerMole` (erg/mol), as a `MolarEnergy`.
    pub const ERG_PER_MOLE: MolarEnergy =
        9.9999999999999995e-08 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `FootPoundalPerMole` (ft · pdl/mol), as a `MolarEnergy`.
    pub const FOOT_POUNDAL_PER_MOLE: MolarEnergy =
        (0.3048 * 0.45359237 * 0.3048) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `FootPoundForcePerMole` (ft · lbf/mol), as a `MolarEnergy`.
    pub const FOOT_POUND_FORCE_PER_MOLE: MolarEnergy =
        (0.3048 * 0.45359237 * 9.80665) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `KilocalorieItPerMole` (kcal (IT)/mol), as a `MolarEnergy`.
    pub const KILOCALORIE_IT_PER_MOLE: MolarEnergy =
        4186.8000000000002 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `KilocaloriePerMole` (kcal/mol), as a `MolarEnergy`.
    pub const KILOCALORIE_PER_MOLE: MolarEnergy =
        4184.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `QuadPerMole` (10¹⁵ Btu (IT)/mol), as a `MolarEnergy`.
    pub const QUAD_PER_MOLE: MolarEnergy =
        1.05505585262e18 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `ThermEcPerMole` (thm (EC)/mol), as a `MolarEnergy`.
    pub const THERM_EC_PER_MOLE: MolarEnergy =
        105506000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `ThermUsPerMole` (thm/mol), as a `MolarEnergy`.
    pub const THERM_US_PER_MOLE: MolarEnergy =
        105480400.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `TonTntPerMole` (t of TNT/mol), as a `MolarEnergy`.
    pub const TON_TNT_PER_MOLE: MolarEnergy =
        4184000000.0 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `WattSecondPerMole` (W · s/mol), as a `MolarEnergy`.
    pub const WATT_SECOND_PER_MOLE: MolarEnergy = KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `JoulePerParticle` (J/particle), as a `MolarEnergy`.
    pub const JOULE_PER_PARTICLE: MolarEnergy =
        6.0221407599999999e+23 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `ElectronvoltPerParticle` (eV/particle), as a `MolarEnergy`.
    pub const ELECTRONVOLT_PER_PARTICLE: MolarEnergy =
        96485.332123310014 * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE;

    /// One `MolePerSquareMetreSecond` (mol/(m² · s)), as a `MolarFlux`.
    pub const MOLE_PER_SQUARE_METRE_SECOND: MolarFlux = MOLE / (METRE * METRE * SECOND);

    /// One `YottajoulePerKelvinMole` (YJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const YOTTAJOULE_PER_KELVIN_MOLE: MolarHeatCapacity =
        9.9999999999999998e+23 * JOULE_PER_KELVIN_MOLE;

    /// One `ZettajoulePerKelvinMole` (ZJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const ZETTAJOULE_PER_KELVIN_MOLE: MolarHeatCapacity = 1e+21 * JOULE_PER_KELVIN_MOLE;

    /// One `ExajoulePerKelvinMole` (EJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const EXAJOULE_PER_KELVIN_MOLE: MolarHeatCapacity = 1e+18 * JOULE_PER_KELVIN_MOLE;

    /// One `PetajoulePerKelvinMole` (PJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const PETAJOULE_PER_KELVIN_MOLE: MolarHeatCapacity =
        1000000000000000.0 * JOULE_PER_KELVIN_MOLE;

    /// One `TerajoulePerKelvinMole` (TJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const TERAJOULE_PER_KELVIN_MOLE: MolarHeatCapacity =
        1000000000000.0 * JOULE_PER_KELVIN_MOLE;

    /// One `GigajoulePerKelvinMole` (GJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const GIGAJOULE_PER_KELVIN_MOLE: MolarHeatCapacity = 1000000000.0 * JOULE_PER_KELVIN_MOLE;

    /// One `MegajoulePerKelvinMole` (MJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const MEGAJOULE_PER_KELVIN_MOLE: MolarHeatCapacity = 1000000.0 * JOULE_PER_KELVIN_MOLE;

    /// One `KilojoulePerKelvinMole` (kJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const KILOJOULE_PER_KELVIN_MOLE: MolarHeatCapacity = 1000.0 * JOULE_PER_KELVIN_MOLE;

    /// One `HectojoulePerKelvinMole` (hJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const HECTOJOULE_PER_KELVIN_MOLE: MolarHeatCapacity = 100.0 * JOULE_PER_KELVIN_MOLE;

    /// One `DecajoulePerKelvinMole` (daJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const DECAJOULE_PER_KELVIN_MOLE: MolarHeatCapacity = 10.0 * JOULE_PER_KELVIN_MOLE;

    /// One `JoulePerKelvinMole` (J/(K · mol)), as a `MolarHeatCapacity`.
    pub const JOULE_PER_KELVIN_MOLE: MolarHeatCapacity =
        METRE * METRE * KILOGRAM / (SECOND * SECOND * KELVIN * MOLE);

    /// One `DecijoulePerKelvinMole` (dJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const DECIJOULE_PER_KELVIN_MOLE: MolarHeatCapacity =
        0.10000000000000001 * JOULE_PER_KELVIN_MOLE;

    /// One `CentijoulePerKelvinMole` (cJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const CENTIJOULE_PER_KELVIN_MOLE: MolarHeatCapacity = 0.01 * JOULE_PER_KELVIN_MOLE;

    /// One `MillijoulePerKelvinMole` (mJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const MILLIJOULE_PER_KELVIN_MOLE: MolarHeatCapacity = 0.001 * JOULE_PER_KELVIN_MOLE;

    /// One `MicrojoulePerKelvinMole` (µJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const MICROJOULE_PER_KELVIN_MOLE: MolarHeatCapacity =
        9.9999999999999995e-07 * JOULE_PER_KELVIN_MOLE;

    /// One `NanojoulePerKelvinMole` (nJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const NANOJOULE_PER_KELVIN_MOLE: MolarHeatCapacity =
        1.0000000000000001e-09 * JOULE_PER_KELVIN_MOLE;

    /// One `PicojoulePerKelvinMole` (pJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const PICOJOULE_PER_KELVIN_MOLE: MolarHeatCapacity =
        9.9999999999999998e-13 * JOULE_PER_KELVIN_MOLE;

    /// One `FemtojoulePerKelvinMole` (fJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const FEMTOJOULE_PER_KELVIN_MOLE: MolarHeatCapacity =
        1.0000000000000001e-15 * JOULE_PER_KELVIN_MOLE;

    /// One `AttojoulePerKelvinMole` (aJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const ATTOJOULE_PER_KELVIN_MOLE: MolarHeatCapacity =
        1.0000000000000001e-18 * JOULE_PER_KELVIN_MOLE;

    /// One `ZeptojoulePerKelvinMole` (zJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const ZEPTOJOULE_PER_KELVIN_MOLE: MolarHeatCapacity =
        9.9999999999999991e-22 * JOULE_PER_KELVIN_MOLE;

    /// One `YoctojoulePerKelvinMole` (yJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const YOCTOJOULE_PER_KELVIN_MOLE: MolarHeatCapacity =
        9.9999999999999992e-25 * JOULE_PER_KELVIN_MOLE;

    /// One `BtuItPerKelvinMole` (Btu (IT)/(K · mol)), as a `MolarHeatCapacity`.
    pub const BTU_IT_PER_KELVIN_MOLE: MolarHeatCapacity = 1055.05585262 * JOULE_PER_KELVIN_MOLE;

    /// One `BtuPerKelvinMole` (Btu/(K · mol)), as a `MolarHeatCapacity`.
    pub const BTU_PER_KELVIN_MOLE: MolarHeatCapacity =
        (4184.0 * 0.45359237 / 1.8) * JOULE_PER_KELVIN_MOLE;

    /// One `CalorieItPerKelvinMole` (cal (IT)/(K · mol)), as a `MolarHeatCapacity`.
    pub const CALORIE_IT_PER_KELVIN_MOLE: MolarHeatCapacity =
        4.1867999999999999 * JOULE_PER_KELVIN_MOLE;

    /// One `CaloriePerKelvinMole` (cal/(K · mol)), as a `MolarHeatCapacity`.
    pub const CALORIE_PER_KELVIN_MOLE: MolarHeatCapacity =
        4.1840000000000002 * JOULE_PER_KELVIN_MOLE;

    /// One `CalorieItNutritionPerKelvinMole` (Cal (IT)/(K · mol)), as a `MolarHeatCapacity`.
    pub const CALORIE_IT_NUTRITION_PER_KELVIN_MOLE: MolarHeatCapacity =
        4186.8000000000002 * JOULE_PER_KELVIN_MOLE;

    /// One `CalorieNutritionPerKelvinMole` (Cal/(K · mol)), as a `MolarHeatCapacity`.
    pub const CALORIE_NUTRITION_PER_KELVIN_MOLE: MolarHeatCapacity = 4184.0 * JOULE_PER_KELVIN_MOLE;

    /// One `KilocalorieItPerKelvinMole` (kcal (IT)/(K · mol)), as a `MolarHeatCapacity`.
    pub const KILOCALORIE_IT_PER_KELVIN_MOLE: MolarHeatCapacity =
        4186.8000000000002 * JOULE_PER_KELVIN_MOLE;

    /// One `KilocaloriePerKelvinMole` (kcal/(K · mol)), as a `MolarHeatCapacity`.
    pub const KILOCALORIE_PER_KELVIN_MOLE: MolarHeatCapacity = 4184.0 * JOULE_PER_KELVIN_MOLE;

    /// One `JoulePerKelvinParticle` (J/(K · particle)), as a `MolarHeatCapacity`.
    pub const JOULE_PER_KELVIN_PARTICLE: MolarHeatCapacity =
        6.0221407599999999e+23 * JOULE_PER_KELVIN_MOLE;

    /// One `ElectronvoltPerKelvinParticle` (eV/(K · particle)), as a `MolarHeatCapacity`.
    pub const ELECTRONVOLT_PER_KELVIN_PARTICLE: MolarHeatCapacity =
        96485.332123310014 * JOULE_PER_KELVIN_MOLE;

    /// One `MolarGasConstant` (R), as a `MolarHeatCapacity`.
    pub const MOLAR_GAS_CONSTANT: MolarHeatCapacity = 8.314462618 * JOULE_PER_KELVIN_MOLE;

    /// One `YottagramPerMole` (Yg/mol), as a `MolarMass`.
    pub const YOTTAGRAM_PER_MOLE: MolarMass = 1e+21 * KILOGRAM_PER_MOLE;

    /// One `ZettagramPerMole` (Zg/mol), as a `MolarMass`.
    pub const ZETTAGRAM_PER_MOLE: MolarMass = 1e+18 * KILOGRAM_PER_MOLE;

    /// One `ExagramPerMole` (Eg/mol), as a `MolarMass`.
    pub const EXAGRAM_PER_MOLE: MolarMass = 1000000000000000.0 * KILOGRAM_PER_MOLE;

    /// One `PetagramPerMole` (Pg/mol), as a `MolarMass`.
    pub const PETAGRAM_PER_MOLE: MolarMass = 1000000000000.0 * KILOGRAM_PER_MOLE;

    /// One `TeragramPerMole` (Tg/mol), as a `MolarMass`.
    pub const TERAGRAM_PER_MOLE: MolarMass = 1000000000.0 * KILOGRAM_PER_MOLE;

    /// One `GigagramPerMole` (Gg/mol), as a `MolarMass`.
    pub const GIGAGRAM_PER_MOLE: MolarMass = 1000000.0 * KILOGRAM_PER_MOLE;

    /// One `MegagramPerMole` (Mg/mol), as a `MolarMass`.
    pub const MEGAGRAM_PER_MOLE: MolarMass = 1000.0 * KILOGRAM_PER_MOLE;

    /// One `KilogramPerMole` (kg/mol), as a `MolarMass`.
    pub const KILOGRAM_PER_MOLE: MolarMass = KILOGRAM / MOLE;

    /// One `HectogramPerMole` (hg/mol), as a `MolarMass`.
    pub const HECTOGRAM_PER_MOLE: MolarMass = 0.10000000000000001 * KILOGRAM_PER_MOLE;

    /// One `DecagramPerMole` (dag/mol), as a `MolarMass`.
    pub const DECAGRAM_PER_MOLE: MolarMass = 0.01 * KILOGRAM_PER_MOLE;

    /// One `GramPerMole` (g/mol), as a `MolarMass`.
    pub const GRAM_PER_MOLE: MolarMass = 0.001 * KILOGRAM_PER_MOLE;

    /// One `DecigramPerMole` (dg/mol), as a `MolarMass`.
    pub const DECIGRAM_PER_MOLE: MolarMass = 0.0001 * KILOGRAM_PER_MOLE;

    /// One `CentigramPerMole` (cg/mol), as a `MolarMass`.
    pub const CENTIGRAM_PER_MOLE: MolarMass = 1.0000000000000001e-05 * KILOGRAM_PER_MOLE;

    /// One `MilligramPerMole` (mg/mol), as a `MolarMass`.
    pub const MILLIGRAM_PER_MOLE: MolarMass = 9.9999999999999995e-07 * KILOGRAM_PER_MOLE;

    /// One `MicrogramPerMole` (µg/mol), as a `MolarMass`.
    pub const MICROGRAM_PER_MOLE: MolarMass = 9.9999999999999986e-10 * KILOGRAM_PER_MOLE;

    /// One `NanogramPerMole` (ng/mol), as a `MolarMass`.
    pub const NANOGRAM_PER_MOLE: MolarMass = 9.9999999999999998e-13 * KILOGRAM_PER_MOLE;

    /// One `PicogramPerMole` (pg/mol), as a `MolarMass`.
    pub const PICOGRAM_PER_MOLE: MolarMass = 1.0000000000000001e-15 * KILOGRAM_PER_MOLE;

    /// One `FemtogramPerMole` (fg/mol), as a `MolarMass`.
    pub const FEMTOGRAM_PER_MOLE: MolarMass = 1.0000000000000001e-18 * KILOGRAM_PER_MOLE;

    /// One `AttogramPerMole` (ag/mol), as a `MolarMass`.
    pub const ATTOGRAM_PER_MOLE: MolarMass = 1.0000000000000001e-21 * KILOGRAM_PER_MOLE;

    /// One `ZeptogramPerMole` (zg/mol), as a `MolarMass`.
    pub const ZEPTOGRAM_PER_MOLE: MolarMass = 9.9999999999999992e-25 * KILOGRAM_PER_MOLE;

    /// One `YoctogramPerMole` (yg/mol), as a `MolarMass`.
    pub const YOCTOGRAM_PER_MOLE: MolarMass = 9.9999999999999986e-28 * KILOGRAM_PER_MOLE;

    /// One `BecquerelPerMole` (Bq/mol), as a `MolarRadioactivity`.
    pub const BECQUEREL_PER_MOLE: MolarRadioactivity = ONE / (SECOND * MOLE);

    /// One `CuriePerMole` (Ci/mol), as a `MolarRadioactivity`.
    pub const CURIE_PER_MOLE: MolarRadioactivity = 37000000000.0 * BECQUEREL_PER_MOLE;

    /// One `DisintegrationsPerMinutePerMole` (dpm/mol), as a `MolarRadioactivity`.
    pub const DISINTEGRATIONS_PER_MINUTE_PER_MOLE: MolarRadioactivity =
        0.016666666666666666 * BECQUEREL_PER_MOLE;

    /// One `CubicMetrePerMole` (m³/mol), as a `MolarVolume`.
    pub const CUBIC_METRE_PER_MOLE: MolarVolume = METRE * METRE * METRE / MOLE;

    /// One `CubicMetrePerParticle` (m³/particle), as a `MolarVolume`.
    pub const CUBIC_METRE_PER_PARTICLE: MolarVolume = 6.0221407599999999e+23 * CUBIC_METRE_PER_MOLE;

    /// One `MolePerKilogram` (mol/kg), as a `Molality`.
    pub const MOLE_PER_KILOGRAM: Molality = MOLE / KILOGRAM;

    /// One `YottagramPerCubicMetre` (Yg/m³), as a `MassConcentration`.
    pub const YOTTAGRAM_PER_CUBIC_METRE: MassConcentration = 1e+21 * KILOGRAM_PER_CUBIC_METRE;

    /// One `ZettagramPerCubicMetre` (Zg/m³), as a `MassConcentration`.
    pub const ZETTAGRAM_PER_CUBIC_METRE: MassConcentration = 1e+18 * KILOGRAM_PER_CUBIC_METRE;

    /// One `ExagramPerCubicMetre` (Eg/m³), as a `MassConcentration`.
    pub const EXAGRAM_PER_CUBIC_METRE: MassConcentration =
        1000000000000000.0 * KILOGRAM_PER_CUBIC_METRE;

    /// One `PetagramPerCubicMetre` (Pg/m³), as a `MassConcentration`.
    pub const PETAGRAM_PER_CUBIC_METRE: MassConcentration =
        1000000000000.0 * KILOGRAM_PER_CUBIC_METRE;

    /// One `TeragramPerCubicMetre` (Tg/m³), as a `MassConcentration`.
    pub const TERAGRAM_PER_CUBIC_METRE: MassConcentration = 1000000000.0 * KILOGRAM_PER_CUBIC_METRE;

    /// One `GigagramPerCubicMetre` (Gg/m³), as a `MassConcentration`.
    pub const GIGAGRAM_PER_CUBIC_METRE: MassConcentration = 1000000.0 * KILOGRAM_PER_CUBIC_METRE;

    /// One `MegagramPerCubicMetre` (Mg/m³), as a `MassConcentration`.
    pub const MEGAGRAM_PER_CUBIC_METRE: MassConcentration = 1000.0 * KILOGRAM_PER_CUBIC_METRE;

    /// One `KilogramPerCubicMetre` (kg/m³), as a `MassConcentration`.
    pub const KILOGRAM_PER_CUBIC_METRE: MassConcentration = KILOGRAM / (METRE * METRE * METRE);

    /// One `HectogramPerCubicMetre` (hg/m³), as a `MassConcentration`.
    pub const HECTOGRAM_PER_CUBIC_METRE: MassConcentration =
        0.10000000000000001 * KILOGRAM_PER_CUBIC_METRE;

    /// One `DecagramPerCubicMetre` (dag/m³), as a `MassConcentration`.
    pub const DECAGRAM_PER_CUBIC_METRE: MassConcentration = 0.01 * KILOGRAM_PER_CUBIC_METRE;

    /// One `GramPerCubicMetre` (g/m³), as a `MassConcentration`.
    pub const GRAM_PER_CUBIC_METRE: MassConcentration = 0.001 * KILOGRAM_PER_CUBIC_METRE;

    /// One `DecigramPerCubicMetre` (dg/m³), as a `MassConcentration`.
    pub const DECIGRAM_PER_CUBIC_METRE: MassConcentration = 0.0001 * KILOGRAM_PER_CUBIC_METRE;

    /// One `CentigramPerCubicMetre` (cg/m³), as a `MassConcentration`.
    pub const CENTIGRAM_PER_CUBIC_METRE: MassConcentration =
        1.0000000000000001e-05 * KILOGRAM_PER_CUBIC_METRE;

    /// One `MilligramPerCubicMetre` (mg/m³), as a `MassConcentration`.
    pub const MILLIGRAM_PER_CUBIC_METRE: MassConcentration =
        9.9999999999999995e-07 * KILOGRAM_PER_CUBIC_METRE;

    /// One `MicrogramPerCubicMetre` (µg/m³), as a `MassConcentration`.
    pub const MICROGRAM_PER_CUBIC_METRE: MassConcentration =
        9.9999999999999986e-10 * KILOGRAM_PER_CUBIC_METRE;

    /// One `NanogramPerCubicMetre` (ng/m³), as a `MassConcentration`.
    pub const NANOGRAM_PER_CUBIC_METRE: MassConcentration =
        9.9999999999999998e-13 * KILOGRAM_PER_CUBIC_METRE;

    /// One `PicogramPerCubicMetre` (pg/m³), as a `MassConcentration`.
    pub const PICOGRAM_PER_CUBIC_METRE: MassConcentration =
        1.0000000000000001e-15 * KILOGRAM_PER_CUBIC_METRE;

    /// One `FemtogramPerCubicMetre` (fg/m³), as a `MassConcentration`.
    pub const FEMTOGRAM_PER_CUBIC_METRE: MassConcentration =
        1.0000000000000001e-18 * KILOGRAM_PER_CUBIC_METRE;

    /// One `AttogramPerCubicMetre` (ag/m³), as a `MassConcentration`.
    pub const ATTOGRAM_PER_CUBIC_METRE: MassConcentration =
        1.0000000000000001e-21 * KILOGRAM_PER_CUBIC_METRE;

    /// One `ZeptogramPerCubicMetre` (zg/m³), as a `MassConcentration`.
    pub const ZEPTOGRAM_PER_CUBIC_METRE: MassConcentration =
        9.9999999999999992e-25 * KILOGRAM_PER_CUBIC_METRE;

    /// One `YoctogramPerCubicMetre` (yg/m³), as a `MassConcentration`.
    pub const YOCTOGRAM_PER_CUBIC_METRE: MassConcentration =
        9.9999999999999986e-28 * KILOGRAM_PER_CUBIC_METRE;

    /// One `Yottakatal` (Ykat), as a `CatalyticActivity`.
    pub const YOTTAKATAL: CatalyticActivity = 9.9999999999999998e+23 * KATAL;

    /// One `Zettakatal` (Zkat), as a `CatalyticActivity`.
    pub const ZETTAKATAL: CatalyticActivity = 1e+21 * KATAL;

    /// One `Exakatal` (Ekat), as a `CatalyticActivity`.
    pub const EXAKATAL: CatalyticActivity = 1e+18 * KATAL;

    /// One `Petakatal` (Pkat), as a `CatalyticActivity`.
    pub const PETAKATAL: CatalyticActivity = 1000000000000000.0 * KATAL;

    /// One `Terakatal` (Tkat), as a `CatalyticActivity`.
    pub const TERAKATAL: CatalyticActivity = 1000000000000.0 * KATAL;

    /// One `Gigakatal` (Gkat), as a `CatalyticActivity`.
    pub const GIGAKATAL: CatalyticActivity = 1000000000.0 * KATAL;

    /// One `Megakatal` (Mkat), as a `CatalyticActivity`.
    pub const MEGAKATAL: CatalyticActivity = 1000000.0 * KATAL;

    /// One `Kilokatal` (kkat), as a `CatalyticActivity`.
    pub const KILOKATAL: CatalyticActivity = 1000.0 * KATAL;

    /// One `Hectokatal` (hkat), as a `CatalyticActivity`.
    pub const HECTOKATAL: CatalyticActivity = 100.0 * KATAL;

    /// One `Decakatal` (dakat), as a `CatalyticActivity`.
    pub const DECAKATAL: CatalyticActivity = 10.0 * KATAL;

    /// One `Katal` (kat), as a `CatalyticActivity`.
    pub const KATAL: CatalyticActivity = MOLE / SECOND;

    /// One `Decikatal` (dkat), as a `CatalyticActivity`.
    pub const DECIKATAL: CatalyticActivity = 0.10000000000000001 * KATAL;

    /// One `Centikatal` (ckat), as a `CatalyticActivity`.
    pub const CENTIKATAL: CatalyticActivity = 0.01 * KATAL;

    /// One `Millikatal` (mkat), as a `CatalyticActivity`.
    pub const MILLIKATAL: CatalyticActivity = 0.001 * KATAL;

    /// One `Microkatal` (µkat), as a `CatalyticActivity`.
    pub const MICROKATAL: CatalyticActivity = 9.9999999999999995e-07 * KATAL;

    /// One `Nanokatal` (nkat), as a `CatalyticActivity`.
    pub const NANOKATAL: CatalyticActivity = 1.0000000000000001e-09 * KATAL;

    /// One `Picokatal` (pkat), as a `CatalyticActivity`.
    pub const PICOKATAL: CatalyticActivity = 9.9999999999999998e-13 * KATAL;

    /// One `Femtokatal` (fkat), as a `CatalyticActivity`.
    pub const FEMTOKATAL: CatalyticActivity = 1.0000000000000001e-15 * KATAL;

    /// One `Attokatal` (akat), as a `CatalyticActivity`.
    pub const ATTOKATAL: CatalyticActivity = 1.0000000000000001e-18 * KATAL;

    /// One `Zeptokatal` (zkat), as a `CatalyticActivity`.
    pub const ZEPTOKATAL: CatalyticActivity = 9.9999999999999991e-22 * KATAL;

    /// One `Yoctokatal` (ykat), as a `CatalyticActivity`.
    pub const YOCTOKATAL: CatalyticActivity = 9.9999999999999992e-25 * KATAL;

    /// One `ParticlePerSecond` (particle/s), as a `CatalyticActivity`.
    pub const PARTICLE_PER_SECOND: CatalyticActivity = 1.6605390671738466e-24 * KATAL;

    /// One `MolePerSecond` (mol/s), as a `CatalyticActivity`.
    pub const MOLE_PER_SECOND: CatalyticActivity = KATAL;

    /// One `YottakatalPerCubicMetre` (Ykat/m³), as a `CatalyticActivityConcentration`.
    pub const YOTTAKATAL_PER_CUBIC_METRE: CatalyticActivityConcentration =
        9.9999999999999998e+23 * KATAL_PER_CUBIC_METRE;

    /// One `ZettakatalPerCubicMetre` (Zkat/m³), as a `CatalyticActivityConcentration`.
    pub const ZETTAKATAL_PER_CUBIC_METRE: CatalyticActivityConcentration =
        1e+21 * KATAL_PER_CUBIC_METRE;

    /// One `ExakatalPerCubicMetre` (Ekat/m³), as a `CatalyticActivityConcentration`.
    pub const EXAKATAL_PER_CUBIC_METRE: CatalyticActivityConcentration =
        1e+18 * KATAL_PER_CUBIC_METRE;

    /// One `PetakatalPerCubicMetre` (Pkat/m³), as a `CatalyticActivityConcentration`.
    pub const PETAKATAL_PER_CUBIC_METRE: CatalyticActivityConcentration =
        1000000000000000.0 * KATAL_PER_CUBIC_METRE;

    /// One `TerakatalPerCubicMetre` (Tkat/m³), as a `CatalyticActivityConcentration`.
    pub const TERAKATAL_PER_CUBIC_METRE: CatalyticActivityConcentration =
        1000000000000.0 * KATAL_PER_CUBIC_METRE;

    /// One `GigakatalPerCubicMetre` (Gkat/m³), as a `CatalyticActivityConcentration`.
    pub const GIGAKATAL_PER_CUBIC_METRE: CatalyticActivityConcentration =
        1000000000.0 * KATAL_PER_CUBIC_METRE;

    /// One `MegakatalPerCubicMetre` (Mkat/m³), as a `CatalyticActivityConcentration`.
    pub const MEGAKATAL_PER_CUBIC_METRE: CatalyticActivityConcentration =
        1000000.0 * KATAL_PER_CUBIC_METRE;

    /// One `KilokatalPerCubicMetre` (kkat/m³), as a `CatalyticActivityConcentration`.
    pub const KILOKATAL_PER_CUBIC_METRE: CatalyticActivityConcentration =
        1000.0 * KATAL_PER_CUBIC_METRE;

    /// One `HectokatalPerCubicMetre` (hkat/m³), as a `CatalyticActivityConcentration`.
    pub const HECTOKATAL_PER_CUBIC_METRE: CatalyticActivityConcentration =
        100.0 * KATAL_PER_CUBIC_METRE;

    /// One `DecakatalPerCubicMetre` (dakat/m³), as a `CatalyticActivityConcentration`.
    pub const DECAKATAL_PER_CUBIC_METRE: CatalyticActivityConcentration =
        10.0 * KATAL_PER_CUBIC_METRE;

    /// One `KatalPerCubicMetre` (kat/m³), as a `CatalyticActivityConcentration`.
    pub const KATAL_PER_CUBIC_METRE: CatalyticActivityConcentration =
        MOLE / (METRE * METRE * METRE * SECOND);

    /// One `DecikatalPerCubicMetre` (dkat/m³), as a `CatalyticActivityConcentration`.
    pub const DECIKATAL_PER_CUBIC_METRE: CatalyticActivityConcentration =
        0.10000000000000001 * KATAL_PER_CUBIC_METRE;

    /// One `CentikatalPerCubicMetre` (ckat/m³), as a `CatalyticActivityConcentration`.
    pub const CENTIKATAL_PER_CUBIC_METRE: CatalyticActivityConcentration =
        0.01 * KATAL_PER_CUBIC_METRE;

    /// One `MillikatalPerCubicMetre` (mkat/m³), as a `CatalyticActivityConcentration`.
    pub const MILLIKATAL_PER_CUBIC_METRE: CatalyticActivityConcentration =
        0.001 * KATAL_PER_CUBIC_METRE;

    /// One `MicrokatalPerCubicMetre` (µkat/m³), as a `CatalyticActivityConcentration`.
    pub const MICROKATAL_PER_CUBIC_METRE: CatalyticActivityConcentration =
        9.9999999999999995e-07 * KATAL_PER_CUBIC_METRE;

    /// One `NanokatalPerCubicMetre` (nkat/m³), as a `CatalyticActivityConcentration`.
    pub const NANOKATAL_PER_CUBIC_METRE: CatalyticActivityConcentration =
        1.0000000000000001e-09 * KATAL_PER_CUBIC_METRE;

    /// One `PicokatalPerCubicMetre` (pkat/m³), as a `CatalyticActivityConcentration`.
    pub const PICOKATAL_PER_CUBIC_METRE: CatalyticActivityConcentration =
        9.9999999999999998e-13 * KATAL_PER_CUBIC_METRE;

    /// One `FemtokatalPerCubicMetre` (fkat/m³), as a `CatalyticActivityConcentration`.
    pub const FEMTOKATAL_PER_CUBIC_METRE: CatalyticActivityConcentration =
        1.0000000000000001e-15 * KATAL_PER_CUBIC_METRE;

    /// One `AttokatalPerCubicMetre` (akat/m³), as a `CatalyticActivityConcentration`.
    pub const ATTOKATAL_PER_CUBIC_METRE: CatalyticActivityConcentration =
        1.0000000000000001e-18 * KATAL_PER_CUBIC_METRE;

    /// One `ZeptokatalPerCubicMetre` (zkat/m³), as a `CatalyticActivityConcentration`.
    pub const ZEPTOKATAL_PER_CUBIC_METRE: CatalyticActivityConcentration =
        9.9999999999999991e-22 * KATAL_PER_CUBIC_METRE;

    /// One `YoctokatalPerCubicMetre` (ykat/m³), as a `CatalyticActivityConcentration`.
    pub const YOCTOKATAL_PER_CUBIC_METRE: CatalyticActivityConcentration =
        9.9999999999999992e-25 * KATAL_PER_CUBIC_METRE;

    /// One `KilokatalPerLitre` (kkat/L), as a `CatalyticActivityConcentration`.
    pub const KILOKATAL_PER_LITRE: CatalyticActivityConcentration =
        1000000.0 * KATAL_PER_CUBIC_METRE;

    /// One `KatalPerLitre` (kat/L), as a `CatalyticActivityConcentration`.
    pub const KATAL_PER_LITRE: CatalyticActivityConcentration = 1000.0 * KATAL_PER_CUBIC_METRE;

    /// One `MillikatalPerLitre` (mkat/L), as a `CatalyticActivityConcentration`.
    pub const MILLIKATAL_PER_LITRE: CatalyticActivityConcentration = KATAL_PER_CUBIC_METRE;

    /// One `MicrokatalPerLitre` (μkat/L), as a `CatalyticActivityConcentration`.
    pub const MICROKATAL_PER_LITRE: CatalyticActivityConcentration = 0.001 * KATAL_PER_CUBIC_METRE;

    /// One `NanokatalPerLitre` (nkat/L), as a `CatalyticActivityConcentration`.
    pub const NANOKATAL_PER_LITRE: CatalyticActivityConcentration =
        9.9999999999999995e-07 * KATAL_PER_CUBIC_METRE;

    /// One `PicokatalPerLitre` (pkat/L), as a `CatalyticActivityConcentration`.
    pub const PICOKATAL_PER_LITRE: CatalyticActivityConcentration =
        1.0000000000000001e-09 * KATAL_PER_CUBIC_METRE;

    /// One `FemtokatalPerLitre` (fkat/L), as a `CatalyticActivityConcentration`.
    pub const FEMTOKATAL_PER_LITRE: CatalyticActivityConcentration =
        9.9999999999999998e-13 * KATAL_PER_CUBIC_METRE;

    /// One `ParticlePerSecondCubicMetre` (particle · s⁻¹ · m⁻³), as a `CatalyticActivityConcentration`.
    pub const PARTICLE_PER_SECOND_CUBIC_METRE: CatalyticActivityConcentration =
        1.6605390671738466e-24 * KATAL_PER_CUBIC_METRE;

    /// One `Yottabecquerel` (YBq), as a `Radioactivity`.
    pub const YOTTABECQUEREL: Radioactivity = 9.9999999999999998e+23 * BECQUEREL;

    /// One `Zettabecquerel` (ZBq), as a `Radioactivity`.
    pub const ZETTABECQUEREL: Radioactivity = 1e+21 * BECQUEREL;

    /// One `Exabecquerel` (EBq), as a `Radioactivity`.
    pub const EXABECQUEREL: Radioactivity = 1e+18 * BECQUEREL;

    /// One `Petabecquerel` (PBq), as a `Radioactivity`.
    pub const PETABECQUEREL: Radioactivity = 1000000000000000.0 * BECQUEREL;

    /// One `Terabecquerel` (TBq), as a `Radioactivity`.
    pub const TERABECQUEREL: Radioactivity = 1000000000000.0 * BECQUEREL;

    /// One `Gigabecquerel` (GBq), as a `Radioactivity`.
    pub const GIGABECQUEREL: Radioactivity = 1000000000.0 * BECQUEREL;

    /// One `Megabecquerel` (MBq), as a `Radioactivity`.
    pub const MEGABECQUEREL: Radioactivity = 1000000.0 * BECQUEREL;

    /// One `Kilobecquerel` (kBq), as a `Radioactivity`.
    pub const KILOBECQUEREL: Radioactivity = 1000.0 * BECQUEREL;

    /// One `Hectobecquerel` (hBq), as a `Radioactivity`.
    pub const HECTOBECQUEREL: Radioactivity = 100.0 * BECQUEREL;

    /// One `Decabecquerel` (daBq), as a `Radioactivity`.
    pub const DECABECQUEREL: Radioactivity = 10.0 * BECQUEREL;

    /// One `Becquerel` (Bq), as a `Radioactivity`.
    pub const BECQUEREL: Radioactivity = ONE / SECOND;

    /// One `Millibecquerel` (mBq), as a `Radioactivity`.
    pub const MILLIBECQUEREL: Radioactivity = 0.001 * BECQUEREL;

    /// One `Microbecquerel` (µBq), as a `Radioactivity`.
    pub const MICROBECQUEREL: Radioactivity = 9.9999999999999995e-07 * BECQUEREL;

    /// One `Nanobecquerel` (nBq), as a `Radioactivity`.
    pub const NANOBECQUEREL: Radioactivity = 1.0000000000000001e-09 * BECQUEREL;

    /// One `Gigacurie` (GCi), as a `Radioactivity`.
    pub const GIGACURIE: Radioactivity = 3.7e+19 * BECQUEREL;

    /// One `Megacurie` (MCi), as a `Radioactivity`.
    pub const MEGACURIE: Radioactivity = 37000000000000000.0 * BECQUEREL;

    /// One `Kilocurie` (kCi), as a `Radioactivity`.
    pub const KILOCURIE: Radioactivity = 37000000000000.0 * BECQUEREL;

    /// One `Curie` (Ci), as a `Radioactivity`.
    pub const CURIE: Radioactivity = 37000000000.0 * BECQUEREL;

    /// One `Millicurie` (mCi), as a `Radioactivity`.
    pub const MILLICURIE: Radioactivity = 37000000.0 * BECQUEREL;

    /// One `Microcurie` (µCi), as a `Radioactivity`.
    pub const MICROCURIE: Radioactivity = 37000.0 * BECQUEREL;

    /// One `Nanocurie` (nCi), as a `Radioactivity`.
    pub const NANOCURIE: Radioactivity = 37.0 * BECQUEREL;

    /// One `DisintegrationsPerMinute` (dpm), as a `Radioactivity`.
    pub const DISINTEGRATIONS_PER_MINUTE: Radioactivity = 0.016666666666666666 * BECQUEREL;

    /// One `BecquerelPerKilogram` (Bq/kg), as a `SpecificRadioactivity`.
    pub const BECQUEREL_PER_KILOGRAM: SpecificRadioactivity = ONE / (KILOGRAM * SECOND);

    /// One `CuriePerKilogram` (Ci/kg), as a `SpecificRadioactivity`.
    pub const CURIE_PER_KILOGRAM: SpecificRadioactivity = 37000000000.0 * BECQUEREL_PER_KILOGRAM;

    /// One `DisintegrationsPerMinutePerKilogram` (dpm/kg), as a `SpecificRadioactivity`.
    pub const DISINTEGRATIONS_PER_MINUTE_PER_KILOGRAM: SpecificRadioactivity =
        0.016666666666666666 * BECQUEREL_PER_KILOGRAM;

    /// One `PerMole` (mol⁻¹), as an `InverseAmountOfSubstance`.
    pub const PER_MOLE: InverseAmountOfSubstance = ONE / MOLE;

    /// One `AvogadroConstant` (N_A), as an `InverseAmountOfSubstance`.
    pub const AVOGADRO_CONSTANT: InverseAmountOfSubstance = 6.02214076e23 * PER_MOLE;
}

/// The units of `AmountOfSubstance`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AmountOfSubstance)]
pub enum AmountOfSubstanceUnit {
    #[unit("Ymol", "yottamoles", by = YOTTAMOLE)]
    Yottamole,
    #[unit("Zmol", "zettamoles", by = ZETTAMOLE)]
    Zettamole,
    #[unit("Emol", "examoles", by = EXAMOLE)]
    Examole,
    #[unit("Pmol", "petamoles", by = PETAMOLE)]
    Petamole,
    #[unit("Tmol", "teramoles", by = TERAMOLE)]
    Teramole,
    #[unit("Gmol", "gigamoles", by = GIGAMOLE)]
    Gigamole,
    #[unit("Mmol", "megamoles", by = MEGAMOLE)]
    Megamole,
    #[unit("kmol", "kilomoles", by = KILOMOLE)]
    Kilomole,
    #[unit("hmol", "hectomoles", by = HECTOMOLE)]
    Hectomole,
    #[unit("damol", "decamoles", by = DECAMOLE)]
    Decamole,
    #[default]
    #[unit("mol", "moles", by = MOLE)]
    Mole,
    #[unit("dmol", "decimoles", by = DECIMOLE)]
    Decimole,
    #[unit("cmol", "centimoles", by = CENTIMOLE)]
    Centimole,
    #[unit("mmol", "millimoles", by = MILLIMOLE)]
    Millimole,
    #[unit("µmol", "micromoles", by = MICROMOLE)]
    Micromole,
    #[unit("nmol", "nanomoles", by = NANOMOLE)]
    Nanomole,
    #[unit("pmol", "picomoles", by = PICOMOLE)]
    Picomole,
    #[unit("fmol", "femtomoles", by = FEMTOMOLE)]
    Femtomole,
    #[unit("amol", "attomoles", by = ATTOMOLE)]
    Attomole,
    #[unit("zmol", "zeptomoles", by = ZEPTOMOLE)]
    Zeptomole,
    #[unit("ymol", "yoctomoles", by = YOCTOMOLE)]
    Yoctomole,
    #[unit("particle", "particles", by = PARTICLE)]
    Particle,
}

/// The units of `MolarConcentration`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarConcentration)]
pub enum MolarConcentrationUnit {
    #[unit("Ymol/m³", "yottamoles per cubic metre", by = YOTTAMOLE_PER_CUBIC_METRE)]
    YottamolePerCubicMetre,
    #[unit("Zmol/m³", "zettamoles per cubic metre", by = ZETTAMOLE_PER_CUBIC_METRE)]
    ZettamolePerCubicMetre,
    #[unit("Emol/m³", "examoles per cubic metre", by = EXAMOLE_PER_CUBIC_METRE)]
    ExamolePerCubicMetre,
    #[unit("Pmol/m³", "petamoles per cubic metre", by = PETAMOLE_PER_CUBIC_METRE)]
    PetamolePerCubicMetre,
    #[unit("Tmol/m³", "teramoles per cubic metre", by = TERAMOLE_PER_CUBIC_METRE)]
    TeramolePerCubicMetre,
    #[unit("Gmol/m³", "gigamoles per cubic metre", by = GIGAMOLE_PER_CUBIC_METRE)]
    GigamolePerCubicMetre,
    #[unit("Mmol/m³", "megamoles per cubic metre", by = MEGAMOLE_PER_CUBIC_METRE)]
    MegamolePerCubicMetre,
    #[unit("kmol/m³", "kilomoles per cubic metre", by = KILOMOLE_PER_CUBIC_METRE)]
    KilomolePerCubicMetre,
    #[unit("hmol/m³", "hectomoles per cubic metre", by = HECTOMOLE_PER_CUBIC_METRE)]
    HectomolePerCubicMetre,
    #[unit("damol/m³", "decamoles per cubic metre", by = DECAMOLE_PER_CUBIC_METRE)]
    DecamolePerCubicMetre,
    #[default]
    #[unit("mol/m³", "moles per cubic metre", by = MOLE_PER_CUBIC_METRE)]
    MolePerCubicMetre,
    #[unit("dmol/m³", "decimoles per cubic metre", by = DECIMOLE_PER_CUBIC_METRE)]
    DecimolePerCubicMetre,
    #[unit("cmol/m³", "centimoles per cubic metre", by = CENTIMOLE_PER_CUBIC_METRE)]
    CentimolePerCubicMetre,
    #[unit("mmol/m³", "millimoles per cubic metre", by = MILLIMOLE_PER_CUBIC_METRE)]
    MillimolePerCubicMetre,
    #[unit("µmol/m³", "micromoles per cubic metre", by = MICROMOLE_PER_CUBIC_METRE)]
    MicromolePerCubicMetre,
    #[unit("nmol/m³", "nanomoles per cubic metre", by = NANOMOLE_PER_CUBIC_METRE)]
    NanomolePerCubicMetre,
    #[unit("pmol/m³", "picomoles per cubic metre", by = PICOMOLE_PER_CUBIC_METRE)]
    PicomolePerCubicMetre,
    #[unit("fmol/m³", "femtomoles per cubic metre", by = FEMTOMOLE_PER_CUBIC_METRE)]
    FemtomolePerCubicMetre,
    #[unit("amol/m³", "attomoles per cubic metre", by = ATTOMOLE_PER_CUBIC_METRE)]
    AttomolePerCubicMetre,
    #[unit("zmol/m³", "zeptomoles per cubic metre", by = ZEPTOMOLE_PER_CUBIC_METRE)]
    ZeptomolePerCubicMetre,
    #[unit("ymol/m³", "yoctomoles per cubic metre", by = YOCTOMOLE_PER_CUBIC_METRE)]
    YoctomolePerCubicMetre,
    #[unit("kmol/L", "kilomoles per litre", by = KILOMOLE_PER_LITRE)]
    KilomolePerLitre,
    #[unit("mol/L", "moles per litre", by = MOLE_PER_LITRE)]
    MolePerLitre,
    #[unit("mmol/L", "millimoles per litre", by = MILLIMOLE_PER_LITRE)]
    MillimolePerLitre,
    #[unit("μmol/L", "micromoles per litre", by = MICROMOLE_PER_LITRE)]
    MicromolePerLitre,
    #[unit("nmol/L", "nanomoles per litre", by = NANOMOLE_PER_LITRE)]
    NanomolePerLitre,
    #[unit("pmol/L", "picomoles per litre", by = PICOMOLE_PER_LITRE)]
    PicomolePerLitre,
    #[unit("fmol/L", "femtomoles per litre", by = FEMTOMOLE_PER_LITRE)]
    FemtomolePerLitre,
    #[unit("particle/m³", "particles per cubic metre", by = PARTICLE_PER_CUBIC_METRE)]
    ParticlePerCubicMetre,
}

/// The units of `MolarEnergy`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarEnergy)]
pub enum MolarEnergyUnit {
    #[default]
    #[unit("kg · m²/(s² · mol)", "kilograms square metre per second squared mole", by = KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE)]
    KilogramSquareMetrePerSecondSquaredMole,
    #[unit("YJ/mol", "yottajoules per mole", by = YOTTAJOULE_PER_MOLE)]
    YottajoulePerMole,
    #[unit("ZJ/mol", "zettajoules per mole", by = ZETTAJOULE_PER_MOLE)]
    ZettajoulePerMole,
    #[unit("EJ/mol", "exajoules per mole", by = EXAJOULE_PER_MOLE)]
    ExajoulePerMole,
    #[unit("PJ/mol", "petajoules per mole", by = PETAJOULE_PER_MOLE)]
    PetajoulePerMole,
    #[unit("TJ/mol", "terajoules per mole", by = TERAJOULE_PER_MOLE)]
    TerajoulePerMole,
    #[unit("GJ/mol", "gigajoules per mole", by = GIGAJOULE_PER_MOLE)]
    GigajoulePerMole,
    #[unit("MJ/mol", "megajoules per mole", by = MEGAJOULE_PER_MOLE)]
    MegajoulePerMole,
    #[unit("kJ/mol", "kilojoules per mole", by = KILOJOULE_PER_MOLE)]
    KilojoulePerMole,
    #[unit("hJ/mol", "hectojoules per mole", by = HECTOJOULE_PER_MOLE)]
    HectojoulePerMole,
    #[unit("daJ/mol", "decajoules per mole", by = DECAJOULE_PER_MOLE)]
    DecajoulePerMole,
    #[unit("J/mol", "joules per mole", by = JOULE_PER_MOLE)]
    JoulePerMole,
    #[unit("dJ/mol", "decijoules per mole", by = DECIJOULE_PER_MOLE)]
    DecijoulePerMole,
    #[unit("cJ/mol", "centijoules per mole", by = CENTIJOULE_PER_MOLE)]
    CentijoulePerMole,
    #[unit("mJ/mol", "millijoules per mole", by = MILLIJOULE_PER_MOLE)]
    MillijoulePerMole,
    #[unit("µJ/mol", "microjoules per mole", by = MICROJOULE_PER_MOLE)]
    MicrojoulePerMole,
    #[unit("nJ/mol", "nanojoules per mole", by = NANOJOULE_PER_MOLE)]
    NanojoulePerMole,
    #[unit("pJ/mol", "picojoules per mole", by = PICOJOULE_PER_MOLE)]
    PicojoulePerMole,
    #[unit("fJ/mol", "femtojoules per mole", by = FEMTOJOULE_PER_MOLE)]
    FemtojoulePerMole,
    #[unit("aJ/mol", "attojoules per mole", by = ATTOJOULE_PER_MOLE)]
    AttojoulePerMole,
    #[unit("zJ/mol", "zeptojoules per mole", by = ZEPTOJOULE_PER_MOLE)]
    ZeptojoulePerMole,
    #[unit("yJ/mol", "yoctojoules per mole", by = YOCTOJOULE_PER_MOLE)]
    YoctojoulePerMole,
    #[unit("PW · h/mol", "petawatt hours per mole", by = PETAWATT_HOUR_PER_MOLE)]
    PetawattHourPerMole,
    #[unit("TW · h/mol", "terawatt hours per mole", by = TERAWATT_HOUR_PER_MOLE)]
    TerawattHourPerMole,
    #[unit("GW · h/mol", "gigawatt hours per mole", by = GIGAWATT_HOUR_PER_MOLE)]
    GigawattHourPerMole,
    #[unit("MW · h/mol", "megawatt hours per mole", by = MEGAWATT_HOUR_PER_MOLE)]
    MegawattHourPerMole,
    #[unit("kW · h/mol", "kilowatt hours per mole", by = KILOWATT_HOUR_PER_MOLE)]
    KilowattHourPerMole,
    #[unit("hW · h/mol", "hectowatt hours per mole", by = HECTOWATT_HOUR_PER_MOLE)]
    HectowattHourPerMole,
    #[unit("daW · h/mol", "decawatt hours per mole", by = DECAWATT_HOUR_PER_MOLE)]
    DecawattHourPerMole,
    #[unit("W · h/mol", "watt hours per mole", by = WATT_HOUR_PER_MOLE)]
    WattHourPerMole,
    #[unit("mW · h/mol", "milliwatt hours per mole", by = MILLIWATT_HOUR_PER_MOLE)]
    MilliwattHourPerMole,
    #[unit("µW · h/mol", "microwatt hours per mole", by = MICROWATT_HOUR_PER_MOLE)]
    MicrowattHourPerMole,
    #[unit("Btu (IT)/mol", "British thermal units (IT) per mole", by = BTU_IT_PER_MOLE)]
    BtuItPerMole,
    #[unit("Btu/mol", "British thermal units per mole", by = BTU_PER_MOLE)]
    BtuPerMole,
    #[unit("cal (IT)/mol", "calories (IT) per mole", by = CALORIE_IT_PER_MOLE)]
    CalorieItPerMole,
    #[unit("cal/mol", "calories per mole", by = CALORIE_PER_MOLE)]
    CaloriePerMole,
    #[unit("Cal (IT)/mol", "Calories (IT) per mole", by = CALORIE_IT_NUTRITION_PER_MOLE)]
    CalorieItNutritionPerMole,
    #[unit("Cal/mol", "Calories per mole", by = CALORIE_NUTRITION_PER_MOLE)]
    CalorieNutritionPerMole,
    #[unit("eV/mol", "electronvolts per mole", by = ELECTRONVOLT_PER_MOLE)]
    ElectronvoltPerMole,
    #[unit("erg/mol", "ergs per mole", by = ERG_PER_MOLE)]
    ErgPerMole,
    #[unit("ft · pdl/mol", "foot poundals per mole", by = FOOT_POUNDAL_PER_MOLE)]
    FootPoundalPerMole,
    #[unit("ft · lbf/mol", "foot pounds-force per mole", by = FOOT_POUND_FORCE_PER_MOLE)]
    FootPoundForcePerMole,
    #[unit("kcal (IT)/mol", "kilocalories (IT) per mole", by = KILOCALORIE_IT_PER_MOLE)]
    KilocalorieItPerMole,
    #[unit("kcal/mol", "kilocalories per mole", by = KILOCALORIE_PER_MOLE)]
    KilocaloriePerMole,
    #[unit("10¹⁵ Btu (IT)/mol", "quads per mole", by = QUAD_PER_MOLE)]
    QuadPerMole,
    #[unit("thm (EC)/mol", "therms (EC) per mole", by = THERM_EC_PER_MOLE)]
    ThermEcPerMole,
    #[unit("thm/mol", "therms per mole", by = THERM_US_PER_MOLE)]
    ThermUsPerMole,
    #[unit("t of TNT/mol", "tons of TNT per mole", by = TON_TNT_PER_MOLE)]
    TonTntPerMole,
    #[unit("W · s/mol", "watt seconds per mole", by = WATT_SECOND_PER_MOLE)]
    WattSecondPerMole,
    #[unit("J/particle", "joules per particle", by = JOULE_PER_PARTICLE)]
    JoulePerParticle,
    #[unit("eV/particle", "electronvolts per particle", by = ELECTRONVOLT_PER_PARTICLE)]
    ElectronvoltPerParticle,
}

/// The units of `MolarFlux`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarFlux)]
pub enum MolarFluxUnit {
    #[default]
    #[unit("mol/(m² · s)", "moles per square metre second", by = MOLE_PER_SQUARE_METRE_SECOND)]
    MolePerSquareMetreSecond,
}

/// The units of `MolarHeatCapacity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarHeatCapacity)]
pub enum MolarHeatCapacityUnit {
    #[unit("YJ/(K · mol)", "yottajoules per kelvin mole", by = YOTTAJOULE_PER_KELVIN_MOLE)]
    YottajoulePerKelvinMole,
    #[unit("ZJ/(K · mol)", "zettajoules per kelvin mole", by = ZETTAJOULE_PER_KELVIN_MOLE)]
    ZettajoulePerKelvinMole,
    #[unit("EJ/(K · mol)", "exajoules per kelvin mole", by = EXAJOULE_PER_KELVIN_MOLE)]
    ExajoulePerKelvinMole,
    #[unit("PJ/(K · mol)", "petajoules per kelvin mole", by = PETAJOULE_PER_KELVIN_MOLE)]
    PetajoulePerKelvinMole,
    #[unit("TJ/(K · mol)", "terajoules per kelvin mole", by = TERAJOULE_PER_KELVIN_MOLE)]
    TerajoulePerKelvinMole,
    #[unit("GJ/(K · mol)", "gigajoules per kelvin mole", by = GIGAJOULE_PER_KELVIN_MOLE)]
    GigajoulePerKelvinMole,
    #[unit("MJ/(K · mol)", "megajoules per kelvin mole", by = MEGAJOULE_PER_KELVIN_MOLE)]
    MegajoulePerKelvinMole,
    #[unit("kJ/(K · mol)", "kilojoules per kelvin mole", by = KILOJOULE_PER_KELVIN_MOLE)]
    KilojoulePerKelvinMole,
    #[unit("hJ/(K · mol)", "hectojoules per kelvin mole", by = HECTOJOULE_PER_KELVIN_MOLE)]
    HectojoulePerKelvinMole,
    #[unit("daJ/(K · mol)", "decajoules per kelvin mole", by = DECAJOULE_PER_KELVIN_MOLE)]
    DecajoulePerKelvinMole,
    #[default]
    #[unit("J/(K · mol)", "joules per kelvin mole", by = JOULE_PER_KELVIN_MOLE)]
    JoulePerKelvinMole,
    #[unit("dJ/(K · mol)", "decijoules per kelvin mole", by = DECIJOULE_PER_KELVIN_MOLE)]
    DecijoulePerKelvinMole,
    #[unit("cJ/(K · mol)", "centijoules per kelvin mole", by = CENTIJOULE_PER_KELVIN_MOLE)]
    CentijoulePerKelvinMole,
    #[unit("mJ/(K · mol)", "millijoules per kelvin mole", by = MILLIJOULE_PER_KELVIN_MOLE)]
    MillijoulePerKelvinMole,
    #[unit("µJ/(K · mol)", "microjoules per kelvin mole", by = MICROJOULE_PER_KELVIN_MOLE)]
    MicrojoulePerKelvinMole,
    #[unit("nJ/(K · mol)", "nanojoules per kelvin mole", by = NANOJOULE_PER_KELVIN_MOLE)]
    NanojoulePerKelvinMole,
    #[unit("pJ/(K · mol)", "picojoules per kelvin mole", by = PICOJOULE_PER_KELVIN_MOLE)]
    PicojoulePerKelvinMole,
    #[unit("fJ/(K · mol)", "femtojoules per kelvin mole", by = FEMTOJOULE_PER_KELVIN_MOLE)]
    FemtojoulePerKelvinMole,
    #[unit("aJ/(K · mol)", "attojoules per kelvin mole", by = ATTOJOULE_PER_KELVIN_MOLE)]
    AttojoulePerKelvinMole,
    #[unit("zJ/(K · mol)", "zeptojoules per kelvin mole", by = ZEPTOJOULE_PER_KELVIN_MOLE)]
    ZeptojoulePerKelvinMole,
    #[unit("yJ/(K · mol)", "yoctojoules per kelvin mole", by = YOCTOJOULE_PER_KELVIN_MOLE)]
    YoctojoulePerKelvinMole,
    #[unit("Btu (IT)/(K · mol)", "British thermal units (IT) per kelvin mole", by = BTU_IT_PER_KELVIN_MOLE)]
    BtuItPerKelvinMole,
    #[unit("Btu/(K · mol)", "British thermal units per kelvin mole", by = BTU_PER_KELVIN_MOLE)]
    BtuPerKelvinMole,
    #[unit("cal (IT)/(K · mol)", "calories (IT) per kelvin mole", by = CALORIE_IT_PER_KELVIN_MOLE)]
    CalorieItPerKelvinMole,
    #[unit("cal/(K · mol)", "calories per kelvin mole", by = CALORIE_PER_KELVIN_MOLE)]
    CaloriePerKelvinMole,
    #[unit("Cal (IT)/(K · mol)", "Calories (IT) per kelvin mole", by = CALORIE_IT_NUTRITION_PER_KELVIN_MOLE)]
    CalorieItNutritionPerKelvinMole,
    #[unit("Cal/(K · mol)", "Calories per kelvin mole", by = CALORIE_NUTRITION_PER_KELVIN_MOLE)]
    CalorieNutritionPerKelvinMole,
    #[unit("kcal (IT)/(K · mol)", "kilocalories (IT) per kelvin mole", by = KILOCALORIE_IT_PER_KELVIN_MOLE)]
    KilocalorieItPerKelvinMole,
    #[unit("kcal/(K · mol)", "kilocalories per kelvin mole", by = KILOCALORIE_PER_KELVIN_MOLE)]
    KilocaloriePerKelvinMole,
    #[unit("J/(K · particle)", "joules per kelvin particle", by = JOULE_PER_KELVIN_PARTICLE)]
    JoulePerKelvinParticle,
    #[unit("eV/(K · particle)", "electronvolts per kelvin particle", by = ELECTRONVOLT_PER_KELVIN_PARTICLE)]
    ElectronvoltPerKelvinParticle,
    #[unit("R", "molar gas constants", by = MOLAR_GAS_CONSTANT)]
    MolarGasConstant,
}

/// The units of `MolarMass`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarMass)]
pub enum MolarMassUnit {
    #[unit("Yg/mol", "yottagrams per mole", by = YOTTAGRAM_PER_MOLE)]
    YottagramPerMole,
    #[unit("Zg/mol", "zettagrams per mole", by = ZETTAGRAM_PER_MOLE)]
    ZettagramPerMole,
    #[unit("Eg/mol", "exagrams per mole", by = EXAGRAM_PER_MOLE)]
    ExagramPerMole,
    #[unit("Pg/mol", "petagrams per mole", by = PETAGRAM_PER_MOLE)]
    PetagramPerMole,
    #[unit("Tg/mol", "teragrams per mole", by = TERAGRAM_PER_MOLE)]
    TeragramPerMole,
    #[unit("Gg/mol", "gigagrams per mole", by = GIGAGRAM_PER_MOLE)]
    GigagramPerMole,
    #[unit("Mg/mol", "megagrams per mole", by = MEGAGRAM_PER_MOLE)]
    MegagramPerMole,
    #[default]
    #[unit("kg/mol", "kilograms per mole", by = KILOGRAM_PER_MOLE)]
    KilogramPerMole,
    #[unit("hg/mol", "hectograms per mole", by = HECTOGRAM_PER_MOLE)]
    HectogramPerMole,
    #[unit("dag/mol", "decagrams per mole", by = DECAGRAM_PER_MOLE)]
    DecagramPerMole,
    #[unit("g/mol", "grams per mole", by = GRAM_PER_MOLE)]
    GramPerMole,
    #[unit("dg/mol", "decigrams per mole", by = DECIGRAM_PER_MOLE)]
    DecigramPerMole,
    #[unit("cg/mol", "centigrams per mole", by = CENTIGRAM_PER_MOLE)]
    CentigramPerMole,
    #[unit("mg/mol", "milligrams per mole", by = MILLIGRAM_PER_MOLE)]
    MilligramPerMole,
    #[unit("µg/mol", "micrograms per mole", by = MICROGRAM_PER_MOLE)]
    MicrogramPerMole,
    #[unit("ng/mol", "nanograms per mole", by = NANOGRAM_PER_MOLE)]
    NanogramPerMole,
    #[unit("pg/mol", "picograms per mole", by = PICOGRAM_PER_MOLE)]
    PicogramPerMole,
    #[unit("fg/mol", "femtograms per mole", by = FEMTOGRAM_PER_MOLE)]
    FemtogramPerMole,
    #[unit("ag/mol", "attograms per mole", by = ATTOGRAM_PER_MOLE)]
    AttogramPerMole,
    #[unit("zg/mol", "zeptograms per mole", by = ZEPTOGRAM_PER_MOLE)]
    ZeptogramPerMole,
    #[unit("yg/mol", "yoctograms per mole", by = YOCTOGRAM_PER_MOLE)]
    YoctogramPerMole,
}

/// The units of `MolarRadioactivity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarRadioactivity)]
pub enum MolarRadioactivityUnit {
    #[default]
    #[unit("Bq/mol", "becquerels per mole", by = BECQUEREL_PER_MOLE)]
    BecquerelPerMole,
    #[unit("Ci/mol", "curies per mole", by = CURIE_PER_MOLE)]
    CuriePerMole,
    #[unit("dpm/mol", "disintegrations per minute per mole", by = DISINTEGRATIONS_PER_MINUTE_PER_MOLE)]
    DisintegrationsPerMinutePerMole,
}

/// The units of `MolarVolume`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarVolume)]
pub enum MolarVolumeUnit {
    #[default]
    #[unit("m³/mol", "cubic metres per mole", by = CUBIC_METRE_PER_MOLE)]
    CubicMetrePerMole,
    #[unit("m³/particle", "cubic metres per particle", by = CUBIC_METRE_PER_PARTICLE)]
    CubicMetrePerParticle,
}

/// The units of `Molality`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Molality)]
pub enum MolalityUnit {
    #[default]
    #[unit("mol/kg", "moles per kilogram", by = MOLE_PER_KILOGRAM)]
    MolePerKilogram,
}

/// The units of `MassConcentration`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassConcentration)]
pub enum MassConcentrationUnit {
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
}

/// The units of `CatalyticActivity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = CatalyticActivity)]
pub enum CatalyticActivityUnit {
    #[unit("Ykat", "yottakatals", by = YOTTAKATAL)]
    Yottakatal,
    #[unit("Zkat", "zettakatals", by = ZETTAKATAL)]
    Zettakatal,
    #[unit("Ekat", "exakatals", by = EXAKATAL)]
    Exakatal,
    #[unit("Pkat", "petakatals", by = PETAKATAL)]
    Petakatal,
    #[unit("Tkat", "terakatals", by = TERAKATAL)]
    Terakatal,
    #[unit("Gkat", "gigakatals", by = GIGAKATAL)]
    Gigakatal,
    #[unit("Mkat", "megakatals", by = MEGAKATAL)]
    Megakatal,
    #[unit("kkat", "kilokatals", by = KILOKATAL)]
    Kilokatal,
    #[unit("hkat", "hectokatals", by = HECTOKATAL)]
    Hectokatal,
    #[unit("dakat", "decakatals", by = DECAKATAL)]
    Decakatal,
    #[default]
    #[unit("kat", "katals", by = KATAL)]
    Katal,
    #[unit("dkat", "decikatals", by = DECIKATAL)]
    Decikatal,
    #[unit("ckat", "centikatals", by = CENTIKATAL)]
    Centikatal,
    #[unit("mkat", "millikatals", by = MILLIKATAL)]
    Millikatal,
    #[unit("µkat", "microkatals", by = MICROKATAL)]
    Microkatal,
    #[unit("nkat", "nanokatals", by = NANOKATAL)]
    Nanokatal,
    #[unit("pkat", "picokatals", by = PICOKATAL)]
    Picokatal,
    #[unit("fkat", "femtokatals", by = FEMTOKATAL)]
    Femtokatal,
    #[unit("akat", "attokatals", by = ATTOKATAL)]
    Attokatal,
    #[unit("zkat", "zeptokatals", by = ZEPTOKATAL)]
    Zeptokatal,
    #[unit("ykat", "yoctokatals", by = YOCTOKATAL)]
    Yoctokatal,
    #[unit("particle/s", "particles per second", by = PARTICLE_PER_SECOND)]
    ParticlePerSecond,
    #[unit("mol/s", "moles per second", by = MOLE_PER_SECOND)]
    MolePerSecond,
}

/// The units of `CatalyticActivityConcentration`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = CatalyticActivityConcentration)]
pub enum CatalyticActivityConcentrationUnit {
    #[unit("Ykat/m³", "yottakatals per cubic metre", by = YOTTAKATAL_PER_CUBIC_METRE)]
    YottakatalPerCubicMetre,
    #[unit("Zkat/m³", "zettakatals per cubic metre", by = ZETTAKATAL_PER_CUBIC_METRE)]
    ZettakatalPerCubicMetre,
    #[unit("Ekat/m³", "exakatals per cubic metre", by = EXAKATAL_PER_CUBIC_METRE)]
    ExakatalPerCubicMetre,
    #[unit("Pkat/m³", "petakatals per cubic metre", by = PETAKATAL_PER_CUBIC_METRE)]
    PetakatalPerCubicMetre,
    #[unit("Tkat/m³", "terakatals per cubic metre", by = TERAKATAL_PER_CUBIC_METRE)]
    TerakatalPerCubicMetre,
    #[unit("Gkat/m³", "gigakatals per cubic metre", by = GIGAKATAL_PER_CUBIC_METRE)]
    GigakatalPerCubicMetre,
    #[unit("Mkat/m³", "megakatals per cubic metre", by = MEGAKATAL_PER_CUBIC_METRE)]
    MegakatalPerCubicMetre,
    #[unit("kkat/m³", "kilokatals per cubic metre", by = KILOKATAL_PER_CUBIC_METRE)]
    KilokatalPerCubicMetre,
    #[unit("hkat/m³", "hectokatals per cubic metre", by = HECTOKATAL_PER_CUBIC_METRE)]
    HectokatalPerCubicMetre,
    #[unit("dakat/m³", "decakatals per cubic metre", by = DECAKATAL_PER_CUBIC_METRE)]
    DecakatalPerCubicMetre,
    #[default]
    #[unit("kat/m³", "katals per cubic metre", by = KATAL_PER_CUBIC_METRE)]
    KatalPerCubicMetre,
    #[unit("dkat/m³", "decikatals per cubic metre", by = DECIKATAL_PER_CUBIC_METRE)]
    DecikatalPerCubicMetre,
    #[unit("ckat/m³", "centikatals per cubic metre", by = CENTIKATAL_PER_CUBIC_METRE)]
    CentikatalPerCubicMetre,
    #[unit("mkat/m³", "millikatals per cubic metre", by = MILLIKATAL_PER_CUBIC_METRE)]
    MillikatalPerCubicMetre,
    #[unit("µkat/m³", "microkatals per cubic metre", by = MICROKATAL_PER_CUBIC_METRE)]
    MicrokatalPerCubicMetre,
    #[unit("nkat/m³", "nanokatals per cubic metre", by = NANOKATAL_PER_CUBIC_METRE)]
    NanokatalPerCubicMetre,
    #[unit("pkat/m³", "picokatals per cubic metre", by = PICOKATAL_PER_CUBIC_METRE)]
    PicokatalPerCubicMetre,
    #[unit("fkat/m³", "femtokatals per cubic metre", by = FEMTOKATAL_PER_CUBIC_METRE)]
    FemtokatalPerCubicMetre,
    #[unit("akat/m³", "attokatals per cubic metre", by = ATTOKATAL_PER_CUBIC_METRE)]
    AttokatalPerCubicMetre,
    #[unit("zkat/m³", "zeptokatals per cubic metre", by = ZEPTOKATAL_PER_CUBIC_METRE)]
    ZeptokatalPerCubicMetre,
    #[unit("ykat/m³", "yoctokatals per cubic metre", by = YOCTOKATAL_PER_CUBIC_METRE)]
    YoctokatalPerCubicMetre,
    #[unit("kkat/L", "kilokatals per litre", by = KILOKATAL_PER_LITRE)]
    KilokatalPerLitre,
    #[unit("kat/L", "katals per litre", by = KATAL_PER_LITRE)]
    KatalPerLitre,
    #[unit("mkat/L", "millikatals per litre", by = MILLIKATAL_PER_LITRE)]
    MillikatalPerLitre,
    #[unit("μkat/L", "microkatals per litre", by = MICROKATAL_PER_LITRE)]
    MicrokatalPerLitre,
    #[unit("nkat/L", "nanokatals per litre", by = NANOKATAL_PER_LITRE)]
    NanokatalPerLitre,
    #[unit("pkat/L", "picokatals per litre", by = PICOKATAL_PER_LITRE)]
    PicokatalPerLitre,
    #[unit("fkat/L", "femtokatals per litre", by = FEMTOKATAL_PER_LITRE)]
    FemtokatalPerLitre,
    #[unit("particle · s⁻¹ · m⁻³", "particles per second cubic metre", by = PARTICLE_PER_SECOND_CUBIC_METRE)]
    ParticlePerSecondCubicMetre,
}

/// The units of `Radioactivity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Radioactivity)]
pub enum RadioactivityUnit {
    #[unit("YBq", "yottabecquerels", by = YOTTABECQUEREL)]
    Yottabecquerel,
    #[unit("ZBq", "zettabecquerels", by = ZETTABECQUEREL)]
    Zettabecquerel,
    #[unit("EBq", "exabecquerels", by = EXABECQUEREL)]
    Exabecquerel,
    #[unit("PBq", "petabecquerels", by = PETABECQUEREL)]
    Petabecquerel,
    #[unit("TBq", "terabecquerels", by = TERABECQUEREL)]
    Terabecquerel,
    #[unit("GBq", "gigabecquerels", by = GIGABECQUEREL)]
    Gigabecquerel,
    #[unit("MBq", "megabecquerels", by = MEGABECQUEREL)]
    Megabecquerel,
    #[unit("kBq", "kilobecquerels", by = KILOBECQUEREL)]
    Kilobecquerel,
    #[unit("hBq", "hectobecquerels", by = HECTOBECQUEREL)]
    Hectobecquerel,
    #[unit("daBq", "decabecquerels", by = DECABECQUEREL)]
    Decabecquerel,
    #[default]
    #[unit("Bq", "becquerels", by = BECQUEREL)]
    Becquerel,
    #[unit("mBq", "millibecquerels", by = MILLIBECQUEREL)]
    Millibecquerel,
    #[unit("µBq", "microbecquerels", by = MICROBECQUEREL)]
    Microbecquerel,
    #[unit("nBq", "nanobecquerels", by = NANOBECQUEREL)]
    Nanobecquerel,
    #[unit("GCi", "gigacuries", by = GIGACURIE)]
    Gigacurie,
    #[unit("MCi", "megacuries", by = MEGACURIE)]
    Megacurie,
    #[unit("kCi", "kilocuries", by = KILOCURIE)]
    Kilocurie,
    #[unit("Ci", "curies", by = CURIE)]
    Curie,
    #[unit("mCi", "millicuries", by = MILLICURIE)]
    Millicurie,
    #[unit("µCi", "microcuries", by = MICROCURIE)]
    Microcurie,
    #[unit("nCi", "nanocuries", by = NANOCURIE)]
    Nanocurie,
    #[unit("dpm", "disintegrations per minute", by = DISINTEGRATIONS_PER_MINUTE)]
    DisintegrationsPerMinute,
}

/// The units of `SpecificRadioactivity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificRadioactivity)]
pub enum SpecificRadioactivityUnit {
    #[default]
    #[unit("Bq/kg", "becquerels per kilogram", by = BECQUEREL_PER_KILOGRAM)]
    BecquerelPerKilogram,
    #[unit("Ci/kg", "curie per kilogram", by = CURIE_PER_KILOGRAM)]
    CuriePerKilogram,
    #[unit("dpm/kg", "disintegrations per minute per kilogram", by = DISINTEGRATIONS_PER_MINUTE_PER_KILOGRAM)]
    DisintegrationsPerMinutePerKilogram,
}

/// The units of `InverseAmountOfSubstance`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = InverseAmountOfSubstance)]
pub enum InverseAmountOfSubstanceUnit {
    #[default]
    #[unit("mol⁻¹", "per mole", by = PER_MOLE)]
    PerMole,
    #[unit("N_A", "Avogadro constants", by = AVOGADRO_CONSTANT)]
    AvogadroConstant,
}
