use crate::{Exact, ISQ, SIQuantity, counted};
use consts::*;
use yadc::Dimension;

pub type AmountOfSubstance<N> = SIQuantity<
    {
        ISQ {
            mol: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MolarConcentration<N> = SIQuantity<
    {
        ISQ {
            m: -3,
            mol: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MolarEnergy<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -2,
            mol: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MolarFlux<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            s: -1,
            mol: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MolarHeatCapacity<N> = SIQuantity<
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
    N,
>;

pub type MolarMass<N> = SIQuantity<
    {
        ISQ {
            kg: 1,
            mol: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MolarRadioactivity<N> = SIQuantity<
    {
        ISQ {
            s: -1,
            mol: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MolarVolume<N> = SIQuantity<
    {
        ISQ {
            m: 3,
            mol: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Molality<N> = SIQuantity<
    {
        ISQ {
            kg: -1,
            mol: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MassConcentration<N> = SIQuantity<
    {
        ISQ {
            m: -3,
            kg: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type CatalyticActivity<N> = SIQuantity<
    {
        ISQ {
            s: -1,
            mol: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type CatalyticActivityConcentration<N> = SIQuantity<
    {
        ISQ {
            m: -3,
            s: -1,
            mol: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Radioactivity<N> = SIQuantity<
    {
        ISQ {
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type SpecificRadioactivity<N> = SIQuantity<
    {
        ISQ {
            kg: -1,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type InverseAmountOfSubstance<N> = SIQuantity<
    {
        ISQ {
            mol: -1,
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
    use crate::base::{KELVIN, KILOGRAM, METRE, ONE, SECOND};

    /// The mole, the SI base unit of amount of substance.
    pub use crate::base::MOLE;

    /// One `Yottamole` (Ymol), as an `AmountOfSubstance`.
    pub const YOTTAMOLE<N: const From<Exact>>: AmountOfSubstance<N> = counted(Exact::int(1000000000000000000000000) * MOLE::<Exact>);

    /// One `Zettamole` (Zmol), as an `AmountOfSubstance`.
    pub const ZETTAMOLE<N: const From<Exact>>: AmountOfSubstance<N> = counted(Exact::int(1000000000000000000000) * MOLE::<Exact>);

    /// One `Examole` (Emol), as an `AmountOfSubstance`.
    pub const EXAMOLE<N: const From<Exact>>: AmountOfSubstance<N> = counted(Exact::int(1000000000000000000) * MOLE::<Exact>);

    /// One `Petamole` (Pmol), as an `AmountOfSubstance`.
    pub const PETAMOLE<N: const From<Exact>>: AmountOfSubstance<N> = counted(Exact::int(1000000000000000) * MOLE::<Exact>);

    /// One `Teramole` (Tmol), as an `AmountOfSubstance`.
    pub const TERAMOLE<N: const From<Exact>>: AmountOfSubstance<N> = counted(Exact::int(1000000000000) * MOLE::<Exact>);

    /// One `Gigamole` (Gmol), as an `AmountOfSubstance`.
    pub const GIGAMOLE<N: const From<Exact>>: AmountOfSubstance<N> = counted(Exact::int(1000000000) * MOLE::<Exact>);

    /// One `Megamole` (Mmol), as an `AmountOfSubstance`.
    pub const MEGAMOLE<N: const From<Exact>>: AmountOfSubstance<N> = counted(Exact::int(1000000) * MOLE::<Exact>);

    /// One `Kilomole` (kmol), as an `AmountOfSubstance`.
    pub const KILOMOLE<N: const From<Exact>>: AmountOfSubstance<N> = counted(Exact::int(1000) * MOLE::<Exact>);

    /// One `Hectomole` (hmol), as an `AmountOfSubstance`.
    pub const HECTOMOLE<N: const From<Exact>>: AmountOfSubstance<N> = counted(Exact::int(100) * MOLE::<Exact>);

    /// One `Decamole` (damol), as an `AmountOfSubstance`.
    pub const DECAMOLE<N: const From<Exact>>: AmountOfSubstance<N> = counted(Exact::int(10) * MOLE::<Exact>);

    /// One `Decimole` (dmol), as an `AmountOfSubstance`.
    pub const DECIMOLE<N: const From<Exact>>: AmountOfSubstance<N> = counted(Exact::dec(1, -1) * MOLE::<Exact>);

    /// One `Centimole` (cmol), as an `AmountOfSubstance`.
    pub const CENTIMOLE<N: const From<Exact>>: AmountOfSubstance<N> = counted(Exact::dec(1, -2) * MOLE::<Exact>);

    /// One `Millimole` (mmol), as an `AmountOfSubstance`.
    pub const MILLIMOLE<N: const From<Exact>>: AmountOfSubstance<N> = counted(Exact::dec(1, -3) * MOLE::<Exact>);

    /// One `Micromole` (µmol), as an `AmountOfSubstance`.
    pub const MICROMOLE<N: const From<Exact>>: AmountOfSubstance<N> = counted(Exact::dec(1, -6) * MOLE::<Exact>);

    /// One `Nanomole` (nmol), as an `AmountOfSubstance`.
    pub const NANOMOLE<N: const From<Exact>>: AmountOfSubstance<N> = counted(Exact::dec(1, -9) * MOLE::<Exact>);

    /// One `Picomole` (pmol), as an `AmountOfSubstance`.
    pub const PICOMOLE<N: const From<Exact>>: AmountOfSubstance<N> = counted(Exact::dec(1, -12) * MOLE::<Exact>);

    /// One `Femtomole` (fmol), as an `AmountOfSubstance`.
    pub const FEMTOMOLE<N: const From<Exact>>: AmountOfSubstance<N> = counted(Exact::dec(1, -15) * MOLE::<Exact>);

    /// One `Attomole` (amol), as an `AmountOfSubstance`.
    pub const ATTOMOLE<N: const From<Exact>>: AmountOfSubstance<N> = counted(Exact::dec(1, -18) * MOLE::<Exact>);

    /// One `Zeptomole` (zmol), as an `AmountOfSubstance`.
    pub const ZEPTOMOLE<N: const From<Exact>>: AmountOfSubstance<N> = counted(Exact::dec(1, -21) * MOLE::<Exact>);

    /// One `Yoctomole` (ymol), as an `AmountOfSubstance`.
    pub const YOCTOMOLE<N: const From<Exact>>: AmountOfSubstance<N> = counted(Exact::dec(1, -24) * MOLE::<Exact>);

    /// One `Particle` (particle), as an `AmountOfSubstance`.
    pub const PARTICLE<N: const From<Exact>>: AmountOfSubstance<N> = counted((Exact::int(1) / Exact::int(602214076000000000000000)) * MOLE::<Exact>);

    /// One `YottamolePerCubicMetre` (Ymol/m³), as a `MolarConcentration`.
    pub const YOTTAMOLE_PER_CUBIC_METRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::int(1000000000000000000000000) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `ZettamolePerCubicMetre` (Zmol/m³), as a `MolarConcentration`.
    pub const ZETTAMOLE_PER_CUBIC_METRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::int(1000000000000000000000) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `ExamolePerCubicMetre` (Emol/m³), as a `MolarConcentration`.
    pub const EXAMOLE_PER_CUBIC_METRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::int(1000000000000000000) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `PetamolePerCubicMetre` (Pmol/m³), as a `MolarConcentration`.
    pub const PETAMOLE_PER_CUBIC_METRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::int(1000000000000000) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `TeramolePerCubicMetre` (Tmol/m³), as a `MolarConcentration`.
    pub const TERAMOLE_PER_CUBIC_METRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::int(1000000000000) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `GigamolePerCubicMetre` (Gmol/m³), as a `MolarConcentration`.
    pub const GIGAMOLE_PER_CUBIC_METRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::int(1000000000) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `MegamolePerCubicMetre` (Mmol/m³), as a `MolarConcentration`.
    pub const MEGAMOLE_PER_CUBIC_METRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::int(1000000) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `KilomolePerCubicMetre` (kmol/m³), as a `MolarConcentration`.
    pub const KILOMOLE_PER_CUBIC_METRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::int(1000) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `HectomolePerCubicMetre` (hmol/m³), as a `MolarConcentration`.
    pub const HECTOMOLE_PER_CUBIC_METRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::int(100) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `DecamolePerCubicMetre` (damol/m³), as a `MolarConcentration`.
    pub const DECAMOLE_PER_CUBIC_METRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::int(10) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `MolePerCubicMetre` (mol/m³), as a `MolarConcentration`.
    pub const MOLE_PER_CUBIC_METRE<N: const From<Exact>>: MolarConcentration<N> = counted(MOLE::<Exact> / (METRE::<Exact> * METRE::<Exact> * METRE::<Exact>));

    /// One `DecimolePerCubicMetre` (dmol/m³), as a `MolarConcentration`.
    pub const DECIMOLE_PER_CUBIC_METRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::dec(1, -1) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `CentimolePerCubicMetre` (cmol/m³), as a `MolarConcentration`.
    pub const CENTIMOLE_PER_CUBIC_METRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::dec(1, -2) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `MillimolePerCubicMetre` (mmol/m³), as a `MolarConcentration`.
    pub const MILLIMOLE_PER_CUBIC_METRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::dec(1, -3) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `MicromolePerCubicMetre` (µmol/m³), as a `MolarConcentration`.
    pub const MICROMOLE_PER_CUBIC_METRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::dec(1, -6) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `NanomolePerCubicMetre` (nmol/m³), as a `MolarConcentration`.
    pub const NANOMOLE_PER_CUBIC_METRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::dec(1, -9) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `PicomolePerCubicMetre` (pmol/m³), as a `MolarConcentration`.
    pub const PICOMOLE_PER_CUBIC_METRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::dec(1, -12) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `FemtomolePerCubicMetre` (fmol/m³), as a `MolarConcentration`.
    pub const FEMTOMOLE_PER_CUBIC_METRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::dec(1, -15) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `AttomolePerCubicMetre` (amol/m³), as a `MolarConcentration`.
    pub const ATTOMOLE_PER_CUBIC_METRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::dec(1, -18) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `ZeptomolePerCubicMetre` (zmol/m³), as a `MolarConcentration`.
    pub const ZEPTOMOLE_PER_CUBIC_METRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::dec(1, -21) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `YoctomolePerCubicMetre` (ymol/m³), as a `MolarConcentration`.
    pub const YOCTOMOLE_PER_CUBIC_METRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::dec(1, -24) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `KilomolePerLitre` (kmol/L), as a `MolarConcentration`.
    pub const KILOMOLE_PER_LITRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::int(1000000) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `MolePerLitre` (mol/L), as a `MolarConcentration`.
    pub const MOLE_PER_LITRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::int(1000) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `MillimolePerLitre` (mmol/L), as a `MolarConcentration`.
    pub const MILLIMOLE_PER_LITRE<N: const From<Exact>>: MolarConcentration<N> = counted(MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `MicromolePerLitre` (μmol/L), as a `MolarConcentration`.
    pub const MICROMOLE_PER_LITRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::dec(1, -3) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `NanomolePerLitre` (nmol/L), as a `MolarConcentration`.
    pub const NANOMOLE_PER_LITRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::dec(1, -6) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `PicomolePerLitre` (pmol/L), as a `MolarConcentration`.
    pub const PICOMOLE_PER_LITRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::dec(1, -9) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `FemtomolePerLitre` (fmol/L), as a `MolarConcentration`.
    pub const FEMTOMOLE_PER_LITRE<N: const From<Exact>>: MolarConcentration<N> = counted(Exact::dec(1, -12) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `ParticlePerCubicMetre` (particle/m³), as a `MolarConcentration`.
    pub const PARTICLE_PER_CUBIC_METRE<N: const From<Exact>>: MolarConcentration<N> = counted((Exact::int(1) / Exact::int(602214076000000000000000)) * MOLE_PER_CUBIC_METRE::<Exact>);

    /// One `KilogramSquareMetrePerSecondSquaredMole` (kg · m²/(s² · mol)), as a `MolarEnergy`.
    pub const KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * MOLE::<Exact>));

    /// One `YottajoulePerMole` (YJ/mol), as a `MolarEnergy`.
    pub const YOTTAJOULE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(1000000000000000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `ZettajoulePerMole` (ZJ/mol), as a `MolarEnergy`.
    pub const ZETTAJOULE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(1000000000000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `ExajoulePerMole` (EJ/mol), as a `MolarEnergy`.
    pub const EXAJOULE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(1000000000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `PetajoulePerMole` (PJ/mol), as a `MolarEnergy`.
    pub const PETAJOULE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(1000000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `TerajoulePerMole` (TJ/mol), as a `MolarEnergy`.
    pub const TERAJOULE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(1000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `GigajoulePerMole` (GJ/mol), as a `MolarEnergy`.
    pub const GIGAJOULE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(1000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `MegajoulePerMole` (MJ/mol), as a `MolarEnergy`.
    pub const MEGAJOULE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(1000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `KilojoulePerMole` (kJ/mol), as a `MolarEnergy`.
    pub const KILOJOULE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(1000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `HectojoulePerMole` (hJ/mol), as a `MolarEnergy`.
    pub const HECTOJOULE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(100) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `DecajoulePerMole` (daJ/mol), as a `MolarEnergy`.
    pub const DECAJOULE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(10) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `JoulePerMole` (J/mol), as a `MolarEnergy`.
    pub const JOULE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `DecijoulePerMole` (dJ/mol), as a `MolarEnergy`.
    pub const DECIJOULE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::dec(1, -1) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `CentijoulePerMole` (cJ/mol), as a `MolarEnergy`.
    pub const CENTIJOULE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::dec(1, -2) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `MillijoulePerMole` (mJ/mol), as a `MolarEnergy`.
    pub const MILLIJOULE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::dec(1, -3) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `MicrojoulePerMole` (µJ/mol), as a `MolarEnergy`.
    pub const MICROJOULE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::dec(1, -6) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `NanojoulePerMole` (nJ/mol), as a `MolarEnergy`.
    pub const NANOJOULE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::dec(1, -9) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `PicojoulePerMole` (pJ/mol), as a `MolarEnergy`.
    pub const PICOJOULE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::dec(1, -12) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `FemtojoulePerMole` (fJ/mol), as a `MolarEnergy`.
    pub const FEMTOJOULE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::dec(1, -15) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `AttojoulePerMole` (aJ/mol), as a `MolarEnergy`.
    pub const ATTOJOULE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::dec(1, -18) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `ZeptojoulePerMole` (zJ/mol), as a `MolarEnergy`.
    pub const ZEPTOJOULE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::dec(1, -21) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `YoctojoulePerMole` (yJ/mol), as a `MolarEnergy`.
    pub const YOCTOJOULE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::dec(1, -24) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `PetawattHourPerMole` (PW · h/mol), as a `MolarEnergy`.
    pub const PETAWATT_HOUR_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(3600000000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `TerawattHourPerMole` (TW · h/mol), as a `MolarEnergy`.
    pub const TERAWATT_HOUR_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(3600000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `GigawattHourPerMole` (GW · h/mol), as a `MolarEnergy`.
    pub const GIGAWATT_HOUR_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(3600000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `MegawattHourPerMole` (MW · h/mol), as a `MolarEnergy`.
    pub const MEGAWATT_HOUR_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(3600000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `KilowattHourPerMole` (kW · h/mol), as a `MolarEnergy`.
    pub const KILOWATT_HOUR_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(3600000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `HectowattHourPerMole` (hW · h/mol), as a `MolarEnergy`.
    pub const HECTOWATT_HOUR_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(360000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `DecawattHourPerMole` (daW · h/mol), as a `MolarEnergy`.
    pub const DECAWATT_HOUR_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(36000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `WattHourPerMole` (W · h/mol), as a `MolarEnergy`.
    pub const WATT_HOUR_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(3600) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `MilliwattHourPerMole` (mW · h/mol), as a `MolarEnergy`.
    pub const MILLIWATT_HOUR_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::dec(36, -1) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `MicrowattHourPerMole` (µW · h/mol), as a `MolarEnergy`.
    pub const MICROWATT_HOUR_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::dec(36, -4) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `BtuItPerMole` (Btu (IT)/mol), as a `MolarEnergy`.
    pub const BTU_IT_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::dec(105505585262, -8) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `BtuPerMole` (Btu/mol), as a `MolarEnergy`.
    pub const BTU_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted((Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1)) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `CalorieItPerMole` (cal (IT)/mol), as a `MolarEnergy`.
    pub const CALORIE_IT_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::dec(41868, -4) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `CaloriePerMole` (cal/mol), as a `MolarEnergy`.
    pub const CALORIE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::dec(4184, -3) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `CalorieItNutritionPerMole` (Cal (IT)/mol), as a `MolarEnergy`.
    pub const CALORIE_IT_NUTRITION_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::dec(41868, -1) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `CalorieNutritionPerMole` (Cal/mol), as a `MolarEnergy`.
    pub const CALORIE_NUTRITION_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(4184) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `ElectronvoltPerMole` (eV/mol), as a `MolarEnergy`.
    pub const ELECTRONVOLT_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::dec(1602176634, -28) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `ErgPerMole` (erg/mol), as a `MolarEnergy`.
    pub const ERG_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::dec(1, -7) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `FootPoundalPerMole` (ft · pdl/mol), as a `MolarEnergy`.
    pub const FOOT_POUNDAL_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted((Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(3048, -4)) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `FootPoundForcePerMole` (ft · lbf/mol), as a `MolarEnergy`.
    pub const FOOT_POUND_FORCE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted((Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `KilocalorieItPerMole` (kcal (IT)/mol), as a `MolarEnergy`.
    pub const KILOCALORIE_IT_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::dec(41868, -1) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `KilocaloriePerMole` (kcal/mol), as a `MolarEnergy`.
    pub const KILOCALORIE_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(4184) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `QuadPerMole` (10¹⁵ Btu (IT)/mol), as a `MolarEnergy`.
    pub const QUAD_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(1055055852620000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `ThermEcPerMole` (thm (EC)/mol), as a `MolarEnergy`.
    pub const THERM_EC_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(105506000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `ThermUsPerMole` (thm/mol), as a `MolarEnergy`.
    pub const THERM_US_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(105480400) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `TonTntPerMole` (t of TNT/mol), as a `MolarEnergy`.
    pub const TON_TNT_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(4184000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `WattSecondPerMole` (W · s/mol), as a `MolarEnergy`.
    pub const WATT_SECOND_PER_MOLE<N: const From<Exact>>: MolarEnergy<N> = counted(KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `JoulePerParticle` (J/particle), as a `MolarEnergy`.
    pub const JOULE_PER_PARTICLE<N: const From<Exact>>: MolarEnergy<N> = counted(Exact::int(602214076000000000000000) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `ElectronvoltPerParticle` (eV/particle), as a `MolarEnergy`.
    pub const ELECTRONVOLT_PER_PARTICLE<N: const From<Exact>>: MolarEnergy<N> = counted((Exact::dec(1602176634, -28) * Exact::int(602214076000000000000000)) * KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE::<Exact>);

    /// One `MolePerSquareMetreSecond` (mol/(m² · s)), as a `MolarFlux`.
    pub const MOLE_PER_SQUARE_METRE_SECOND<N: const From<Exact>>: MolarFlux<N> = counted(MOLE::<Exact> / (METRE::<Exact> * METRE::<Exact> * SECOND::<Exact>));

    /// One `YottajoulePerKelvinMole` (YJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const YOTTAJOULE_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::int(1000000000000000000000000) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `ZettajoulePerKelvinMole` (ZJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const ZETTAJOULE_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::int(1000000000000000000000) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `ExajoulePerKelvinMole` (EJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const EXAJOULE_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::int(1000000000000000000) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `PetajoulePerKelvinMole` (PJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const PETAJOULE_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::int(1000000000000000) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `TerajoulePerKelvinMole` (TJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const TERAJOULE_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::int(1000000000000) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `GigajoulePerKelvinMole` (GJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const GIGAJOULE_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::int(1000000000) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `MegajoulePerKelvinMole` (MJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const MEGAJOULE_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::int(1000000) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `KilojoulePerKelvinMole` (kJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const KILOJOULE_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::int(1000) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `HectojoulePerKelvinMole` (hJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const HECTOJOULE_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::int(100) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `DecajoulePerKelvinMole` (daJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const DECAJOULE_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::int(10) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `JoulePerKelvinMole` (J/(K · mol)), as a `MolarHeatCapacity`.
    pub const JOULE_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * KELVIN::<Exact> * MOLE::<Exact>));

    /// One `DecijoulePerKelvinMole` (dJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const DECIJOULE_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::dec(1, -1) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `CentijoulePerKelvinMole` (cJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const CENTIJOULE_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::dec(1, -2) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `MillijoulePerKelvinMole` (mJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const MILLIJOULE_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::dec(1, -3) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `MicrojoulePerKelvinMole` (µJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const MICROJOULE_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::dec(1, -6) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `NanojoulePerKelvinMole` (nJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const NANOJOULE_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::dec(1, -9) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `PicojoulePerKelvinMole` (pJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const PICOJOULE_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::dec(1, -12) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `FemtojoulePerKelvinMole` (fJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const FEMTOJOULE_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::dec(1, -15) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `AttojoulePerKelvinMole` (aJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const ATTOJOULE_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::dec(1, -18) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `ZeptojoulePerKelvinMole` (zJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const ZEPTOJOULE_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::dec(1, -21) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `YoctojoulePerKelvinMole` (yJ/(K · mol)), as a `MolarHeatCapacity`.
    pub const YOCTOJOULE_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::dec(1, -24) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `BtuItPerKelvinMole` (Btu (IT)/(K · mol)), as a `MolarHeatCapacity`.
    pub const BTU_IT_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::dec(105505585262, -8) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `BtuPerKelvinMole` (Btu/(K · mol)), as a `MolarHeatCapacity`.
    pub const BTU_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted((Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1)) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `CalorieItPerKelvinMole` (cal (IT)/(K · mol)), as a `MolarHeatCapacity`.
    pub const CALORIE_IT_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::dec(41868, -4) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `CaloriePerKelvinMole` (cal/(K · mol)), as a `MolarHeatCapacity`.
    pub const CALORIE_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::dec(4184, -3) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `CalorieItNutritionPerKelvinMole` (Cal (IT)/(K · mol)), as a `MolarHeatCapacity`.
    pub const CALORIE_IT_NUTRITION_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::dec(41868, -1) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `CalorieNutritionPerKelvinMole` (Cal/(K · mol)), as a `MolarHeatCapacity`.
    pub const CALORIE_NUTRITION_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::int(4184) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `KilocalorieItPerKelvinMole` (kcal (IT)/(K · mol)), as a `MolarHeatCapacity`.
    pub const KILOCALORIE_IT_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::dec(41868, -1) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `KilocaloriePerKelvinMole` (kcal/(K · mol)), as a `MolarHeatCapacity`.
    pub const KILOCALORIE_PER_KELVIN_MOLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::int(4184) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `JoulePerKelvinParticle` (J/(K · particle)), as a `MolarHeatCapacity`.
    pub const JOULE_PER_KELVIN_PARTICLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::int(602214076000000000000000) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `ElectronvoltPerKelvinParticle` (eV/(K · particle)), as a `MolarHeatCapacity`.
    pub const ELECTRONVOLT_PER_KELVIN_PARTICLE<N: const From<Exact>>: MolarHeatCapacity<N> = counted((Exact::dec(1602176634, -28) * Exact::int(602214076000000000000000)) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `MolarGasConstant` (R), as a `MolarHeatCapacity`.
    pub const MOLAR_GAS_CONSTANT<N: const From<Exact>>: MolarHeatCapacity<N> = counted(Exact::dec(8314462618, -9) * JOULE_PER_KELVIN_MOLE::<Exact>);

    /// One `YottagramPerMole` (Yg/mol), as a `MolarMass`.
    pub const YOTTAGRAM_PER_MOLE<N: const From<Exact>>: MolarMass<N> = counted(Exact::int(1000000000000000000000) * KILOGRAM_PER_MOLE::<Exact>);

    /// One `ZettagramPerMole` (Zg/mol), as a `MolarMass`.
    pub const ZETTAGRAM_PER_MOLE<N: const From<Exact>>: MolarMass<N> = counted(Exact::int(1000000000000000000) * KILOGRAM_PER_MOLE::<Exact>);

    /// One `ExagramPerMole` (Eg/mol), as a `MolarMass`.
    pub const EXAGRAM_PER_MOLE<N: const From<Exact>>: MolarMass<N> = counted(Exact::int(1000000000000000) * KILOGRAM_PER_MOLE::<Exact>);

    /// One `PetagramPerMole` (Pg/mol), as a `MolarMass`.
    pub const PETAGRAM_PER_MOLE<N: const From<Exact>>: MolarMass<N> = counted(Exact::int(1000000000000) * KILOGRAM_PER_MOLE::<Exact>);

    /// One `TeragramPerMole` (Tg/mol), as a `MolarMass`.
    pub const TERAGRAM_PER_MOLE<N: const From<Exact>>: MolarMass<N> = counted(Exact::int(1000000000) * KILOGRAM_PER_MOLE::<Exact>);

    /// One `GigagramPerMole` (Gg/mol), as a `MolarMass`.
    pub const GIGAGRAM_PER_MOLE<N: const From<Exact>>: MolarMass<N> = counted(Exact::int(1000000) * KILOGRAM_PER_MOLE::<Exact>);

    /// One `MegagramPerMole` (Mg/mol), as a `MolarMass`.
    pub const MEGAGRAM_PER_MOLE<N: const From<Exact>>: MolarMass<N> = counted(Exact::int(1000) * KILOGRAM_PER_MOLE::<Exact>);

    /// One `KilogramPerMole` (kg/mol), as a `MolarMass`.
    pub const KILOGRAM_PER_MOLE<N: const From<Exact>>: MolarMass<N> = counted(KILOGRAM::<Exact> / MOLE::<Exact>);

    /// One `HectogramPerMole` (hg/mol), as a `MolarMass`.
    pub const HECTOGRAM_PER_MOLE<N: const From<Exact>>: MolarMass<N> = counted(Exact::dec(1, -1) * KILOGRAM_PER_MOLE::<Exact>);

    /// One `DecagramPerMole` (dag/mol), as a `MolarMass`.
    pub const DECAGRAM_PER_MOLE<N: const From<Exact>>: MolarMass<N> = counted(Exact::dec(1, -2) * KILOGRAM_PER_MOLE::<Exact>);

    /// One `GramPerMole` (g/mol), as a `MolarMass`.
    pub const GRAM_PER_MOLE<N: const From<Exact>>: MolarMass<N> = counted(Exact::dec(1, -3) * KILOGRAM_PER_MOLE::<Exact>);

    /// One `DecigramPerMole` (dg/mol), as a `MolarMass`.
    pub const DECIGRAM_PER_MOLE<N: const From<Exact>>: MolarMass<N> = counted(Exact::dec(1, -4) * KILOGRAM_PER_MOLE::<Exact>);

    /// One `CentigramPerMole` (cg/mol), as a `MolarMass`.
    pub const CENTIGRAM_PER_MOLE<N: const From<Exact>>: MolarMass<N> = counted(Exact::dec(1, -5) * KILOGRAM_PER_MOLE::<Exact>);

    /// One `MilligramPerMole` (mg/mol), as a `MolarMass`.
    pub const MILLIGRAM_PER_MOLE<N: const From<Exact>>: MolarMass<N> = counted(Exact::dec(1, -6) * KILOGRAM_PER_MOLE::<Exact>);

    /// One `MicrogramPerMole` (µg/mol), as a `MolarMass`.
    pub const MICROGRAM_PER_MOLE<N: const From<Exact>>: MolarMass<N> = counted(Exact::dec(1, -9) * KILOGRAM_PER_MOLE::<Exact>);

    /// One `NanogramPerMole` (ng/mol), as a `MolarMass`.
    pub const NANOGRAM_PER_MOLE<N: const From<Exact>>: MolarMass<N> = counted(Exact::dec(1, -12) * KILOGRAM_PER_MOLE::<Exact>);

    /// One `PicogramPerMole` (pg/mol), as a `MolarMass`.
    pub const PICOGRAM_PER_MOLE<N: const From<Exact>>: MolarMass<N> = counted(Exact::dec(1, -15) * KILOGRAM_PER_MOLE::<Exact>);

    /// One `FemtogramPerMole` (fg/mol), as a `MolarMass`.
    pub const FEMTOGRAM_PER_MOLE<N: const From<Exact>>: MolarMass<N> = counted(Exact::dec(1, -18) * KILOGRAM_PER_MOLE::<Exact>);

    /// One `AttogramPerMole` (ag/mol), as a `MolarMass`.
    pub const ATTOGRAM_PER_MOLE<N: const From<Exact>>: MolarMass<N> = counted(Exact::dec(1, -21) * KILOGRAM_PER_MOLE::<Exact>);

    /// One `ZeptogramPerMole` (zg/mol), as a `MolarMass`.
    pub const ZEPTOGRAM_PER_MOLE<N: const From<Exact>>: MolarMass<N> = counted(Exact::dec(1, -24) * KILOGRAM_PER_MOLE::<Exact>);

    /// One `YoctogramPerMole` (yg/mol), as a `MolarMass`.
    pub const YOCTOGRAM_PER_MOLE<N: const From<Exact>>: MolarMass<N> = counted(Exact::dec(1, -27) * KILOGRAM_PER_MOLE::<Exact>);

    /// One `BecquerelPerMole` (Bq/mol), as a `MolarRadioactivity`.
    pub const BECQUEREL_PER_MOLE<N: const From<Exact>>: MolarRadioactivity<N> = counted(ONE::<Exact> / (SECOND::<Exact> * MOLE::<Exact>));

    /// One `CuriePerMole` (Ci/mol), as a `MolarRadioactivity`.
    pub const CURIE_PER_MOLE<N: const From<Exact>>: MolarRadioactivity<N> = counted(Exact::int(37000000000) * BECQUEREL_PER_MOLE::<Exact>);

    /// One `DisintegrationsPerMinutePerMole` (dpm/mol), as a `MolarRadioactivity`.
    pub const DISINTEGRATIONS_PER_MINUTE_PER_MOLE<N: const From<Exact>>: MolarRadioactivity<N> = counted((Exact::int(1) / Exact::int(60)) * BECQUEREL_PER_MOLE::<Exact>);

    /// One `CubicMetrePerMole` (m³/mol), as a `MolarVolume`.
    pub const CUBIC_METRE_PER_MOLE<N: const From<Exact>>: MolarVolume<N> = counted(METRE::<Exact> * METRE::<Exact> * METRE::<Exact> / MOLE::<Exact>);

    /// One `CubicMetrePerParticle` (m³/particle), as a `MolarVolume`.
    pub const CUBIC_METRE_PER_PARTICLE<N: const From<Exact>>: MolarVolume<N> = counted(Exact::int(602214076000000000000000) * CUBIC_METRE_PER_MOLE::<Exact>);

    /// One `MolePerKilogram` (mol/kg), as a `Molality`.
    pub const MOLE_PER_KILOGRAM<N: const From<Exact>>: Molality<N> = counted(MOLE::<Exact> / KILOGRAM::<Exact>);

    /// One `YottagramPerCubicMetre` (Yg/m³), as a `MassConcentration`.
    pub const YOTTAGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassConcentration<N> = counted(Exact::int(1000000000000000000000) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `ZettagramPerCubicMetre` (Zg/m³), as a `MassConcentration`.
    pub const ZETTAGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassConcentration<N> = counted(Exact::int(1000000000000000000) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `ExagramPerCubicMetre` (Eg/m³), as a `MassConcentration`.
    pub const EXAGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassConcentration<N> = counted(Exact::int(1000000000000000) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `PetagramPerCubicMetre` (Pg/m³), as a `MassConcentration`.
    pub const PETAGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassConcentration<N> = counted(Exact::int(1000000000000) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `TeragramPerCubicMetre` (Tg/m³), as a `MassConcentration`.
    pub const TERAGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassConcentration<N> = counted(Exact::int(1000000000) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `GigagramPerCubicMetre` (Gg/m³), as a `MassConcentration`.
    pub const GIGAGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassConcentration<N> = counted(Exact::int(1000000) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `MegagramPerCubicMetre` (Mg/m³), as a `MassConcentration`.
    pub const MEGAGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassConcentration<N> = counted(Exact::int(1000) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `KilogramPerCubicMetre` (kg/m³), as a `MassConcentration`.
    pub const KILOGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassConcentration<N> = counted(KILOGRAM::<Exact> / (METRE::<Exact> * METRE::<Exact> * METRE::<Exact>));

    /// One `HectogramPerCubicMetre` (hg/m³), as a `MassConcentration`.
    pub const HECTOGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassConcentration<N> = counted(Exact::dec(1, -1) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `DecagramPerCubicMetre` (dag/m³), as a `MassConcentration`.
    pub const DECAGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassConcentration<N> = counted(Exact::dec(1, -2) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `GramPerCubicMetre` (g/m³), as a `MassConcentration`.
    pub const GRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassConcentration<N> = counted(Exact::dec(1, -3) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `DecigramPerCubicMetre` (dg/m³), as a `MassConcentration`.
    pub const DECIGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassConcentration<N> = counted(Exact::dec(1, -4) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `CentigramPerCubicMetre` (cg/m³), as a `MassConcentration`.
    pub const CENTIGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassConcentration<N> = counted(Exact::dec(1, -5) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `MilligramPerCubicMetre` (mg/m³), as a `MassConcentration`.
    pub const MILLIGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassConcentration<N> = counted(Exact::dec(1, -6) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `MicrogramPerCubicMetre` (µg/m³), as a `MassConcentration`.
    pub const MICROGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassConcentration<N> = counted(Exact::dec(1, -9) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `NanogramPerCubicMetre` (ng/m³), as a `MassConcentration`.
    pub const NANOGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassConcentration<N> = counted(Exact::dec(1, -12) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `PicogramPerCubicMetre` (pg/m³), as a `MassConcentration`.
    pub const PICOGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassConcentration<N> = counted(Exact::dec(1, -15) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `FemtogramPerCubicMetre` (fg/m³), as a `MassConcentration`.
    pub const FEMTOGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassConcentration<N> = counted(Exact::dec(1, -18) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `AttogramPerCubicMetre` (ag/m³), as a `MassConcentration`.
    pub const ATTOGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassConcentration<N> = counted(Exact::dec(1, -21) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `ZeptogramPerCubicMetre` (zg/m³), as a `MassConcentration`.
    pub const ZEPTOGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassConcentration<N> = counted(Exact::dec(1, -24) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `YoctogramPerCubicMetre` (yg/m³), as a `MassConcentration`.
    pub const YOCTOGRAM_PER_CUBIC_METRE<N: const From<Exact>>: MassConcentration<N> = counted(Exact::dec(1, -27) * KILOGRAM_PER_CUBIC_METRE::<Exact>);

    /// One `Yottakatal` (Ykat), as a `CatalyticActivity`.
    pub const YOTTAKATAL<N: const From<Exact>>: CatalyticActivity<N> = counted(Exact::int(1000000000000000000000000) * KATAL::<Exact>);

    /// One `Zettakatal` (Zkat), as a `CatalyticActivity`.
    pub const ZETTAKATAL<N: const From<Exact>>: CatalyticActivity<N> = counted(Exact::int(1000000000000000000000) * KATAL::<Exact>);

    /// One `Exakatal` (Ekat), as a `CatalyticActivity`.
    pub const EXAKATAL<N: const From<Exact>>: CatalyticActivity<N> = counted(Exact::int(1000000000000000000) * KATAL::<Exact>);

    /// One `Petakatal` (Pkat), as a `CatalyticActivity`.
    pub const PETAKATAL<N: const From<Exact>>: CatalyticActivity<N> = counted(Exact::int(1000000000000000) * KATAL::<Exact>);

    /// One `Terakatal` (Tkat), as a `CatalyticActivity`.
    pub const TERAKATAL<N: const From<Exact>>: CatalyticActivity<N> = counted(Exact::int(1000000000000) * KATAL::<Exact>);

    /// One `Gigakatal` (Gkat), as a `CatalyticActivity`.
    pub const GIGAKATAL<N: const From<Exact>>: CatalyticActivity<N> = counted(Exact::int(1000000000) * KATAL::<Exact>);

    /// One `Megakatal` (Mkat), as a `CatalyticActivity`.
    pub const MEGAKATAL<N: const From<Exact>>: CatalyticActivity<N> = counted(Exact::int(1000000) * KATAL::<Exact>);

    /// One `Kilokatal` (kkat), as a `CatalyticActivity`.
    pub const KILOKATAL<N: const From<Exact>>: CatalyticActivity<N> = counted(Exact::int(1000) * KATAL::<Exact>);

    /// One `Hectokatal` (hkat), as a `CatalyticActivity`.
    pub const HECTOKATAL<N: const From<Exact>>: CatalyticActivity<N> = counted(Exact::int(100) * KATAL::<Exact>);

    /// One `Decakatal` (dakat), as a `CatalyticActivity`.
    pub const DECAKATAL<N: const From<Exact>>: CatalyticActivity<N> = counted(Exact::int(10) * KATAL::<Exact>);

    /// One `Katal` (kat), as a `CatalyticActivity`.
    pub const KATAL<N: const From<Exact>>: CatalyticActivity<N> = counted(MOLE::<Exact> / SECOND::<Exact>);

    /// One `Decikatal` (dkat), as a `CatalyticActivity`.
    pub const DECIKATAL<N: const From<Exact>>: CatalyticActivity<N> = counted(Exact::dec(1, -1) * KATAL::<Exact>);

    /// One `Centikatal` (ckat), as a `CatalyticActivity`.
    pub const CENTIKATAL<N: const From<Exact>>: CatalyticActivity<N> = counted(Exact::dec(1, -2) * KATAL::<Exact>);

    /// One `Millikatal` (mkat), as a `CatalyticActivity`.
    pub const MILLIKATAL<N: const From<Exact>>: CatalyticActivity<N> = counted(Exact::dec(1, -3) * KATAL::<Exact>);

    /// One `Microkatal` (µkat), as a `CatalyticActivity`.
    pub const MICROKATAL<N: const From<Exact>>: CatalyticActivity<N> = counted(Exact::dec(1, -6) * KATAL::<Exact>);

    /// One `Nanokatal` (nkat), as a `CatalyticActivity`.
    pub const NANOKATAL<N: const From<Exact>>: CatalyticActivity<N> = counted(Exact::dec(1, -9) * KATAL::<Exact>);

    /// One `Picokatal` (pkat), as a `CatalyticActivity`.
    pub const PICOKATAL<N: const From<Exact>>: CatalyticActivity<N> = counted(Exact::dec(1, -12) * KATAL::<Exact>);

    /// One `Femtokatal` (fkat), as a `CatalyticActivity`.
    pub const FEMTOKATAL<N: const From<Exact>>: CatalyticActivity<N> = counted(Exact::dec(1, -15) * KATAL::<Exact>);

    /// One `Attokatal` (akat), as a `CatalyticActivity`.
    pub const ATTOKATAL<N: const From<Exact>>: CatalyticActivity<N> = counted(Exact::dec(1, -18) * KATAL::<Exact>);

    /// One `Zeptokatal` (zkat), as a `CatalyticActivity`.
    pub const ZEPTOKATAL<N: const From<Exact>>: CatalyticActivity<N> = counted(Exact::dec(1, -21) * KATAL::<Exact>);

    /// One `Yoctokatal` (ykat), as a `CatalyticActivity`.
    pub const YOCTOKATAL<N: const From<Exact>>: CatalyticActivity<N> = counted(Exact::dec(1, -24) * KATAL::<Exact>);

    /// One `ParticlePerSecond` (particle/s), as a `CatalyticActivity`.
    pub const PARTICLE_PER_SECOND<N: const From<Exact>>: CatalyticActivity<N> = counted((Exact::int(1) / Exact::int(602214076000000000000000)) * KATAL::<Exact>);

    /// One `MolePerSecond` (mol/s), as a `CatalyticActivity`.
    pub const MOLE_PER_SECOND<N: const From<Exact>>: CatalyticActivity<N> = counted(KATAL::<Exact>);

    /// One `YottakatalPerCubicMetre` (Ykat/m³), as a `CatalyticActivityConcentration`.
    pub const YOTTAKATAL_PER_CUBIC_METRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::int(1000000000000000000000000) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `ZettakatalPerCubicMetre` (Zkat/m³), as a `CatalyticActivityConcentration`.
    pub const ZETTAKATAL_PER_CUBIC_METRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::int(1000000000000000000000) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `ExakatalPerCubicMetre` (Ekat/m³), as a `CatalyticActivityConcentration`.
    pub const EXAKATAL_PER_CUBIC_METRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::int(1000000000000000000) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `PetakatalPerCubicMetre` (Pkat/m³), as a `CatalyticActivityConcentration`.
    pub const PETAKATAL_PER_CUBIC_METRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::int(1000000000000000) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `TerakatalPerCubicMetre` (Tkat/m³), as a `CatalyticActivityConcentration`.
    pub const TERAKATAL_PER_CUBIC_METRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::int(1000000000000) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `GigakatalPerCubicMetre` (Gkat/m³), as a `CatalyticActivityConcentration`.
    pub const GIGAKATAL_PER_CUBIC_METRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::int(1000000000) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `MegakatalPerCubicMetre` (Mkat/m³), as a `CatalyticActivityConcentration`.
    pub const MEGAKATAL_PER_CUBIC_METRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::int(1000000) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `KilokatalPerCubicMetre` (kkat/m³), as a `CatalyticActivityConcentration`.
    pub const KILOKATAL_PER_CUBIC_METRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::int(1000) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `HectokatalPerCubicMetre` (hkat/m³), as a `CatalyticActivityConcentration`.
    pub const HECTOKATAL_PER_CUBIC_METRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::int(100) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `DecakatalPerCubicMetre` (dakat/m³), as a `CatalyticActivityConcentration`.
    pub const DECAKATAL_PER_CUBIC_METRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::int(10) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `KatalPerCubicMetre` (kat/m³), as a `CatalyticActivityConcentration`.
    pub const KATAL_PER_CUBIC_METRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(MOLE::<Exact> / (METRE::<Exact> * METRE::<Exact> * METRE::<Exact> * SECOND::<Exact>));

    /// One `DecikatalPerCubicMetre` (dkat/m³), as a `CatalyticActivityConcentration`.
    pub const DECIKATAL_PER_CUBIC_METRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::dec(1, -1) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `CentikatalPerCubicMetre` (ckat/m³), as a `CatalyticActivityConcentration`.
    pub const CENTIKATAL_PER_CUBIC_METRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::dec(1, -2) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `MillikatalPerCubicMetre` (mkat/m³), as a `CatalyticActivityConcentration`.
    pub const MILLIKATAL_PER_CUBIC_METRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::dec(1, -3) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `MicrokatalPerCubicMetre` (µkat/m³), as a `CatalyticActivityConcentration`.
    pub const MICROKATAL_PER_CUBIC_METRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::dec(1, -6) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `NanokatalPerCubicMetre` (nkat/m³), as a `CatalyticActivityConcentration`.
    pub const NANOKATAL_PER_CUBIC_METRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::dec(1, -9) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `PicokatalPerCubicMetre` (pkat/m³), as a `CatalyticActivityConcentration`.
    pub const PICOKATAL_PER_CUBIC_METRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::dec(1, -12) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `FemtokatalPerCubicMetre` (fkat/m³), as a `CatalyticActivityConcentration`.
    pub const FEMTOKATAL_PER_CUBIC_METRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::dec(1, -15) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `AttokatalPerCubicMetre` (akat/m³), as a `CatalyticActivityConcentration`.
    pub const ATTOKATAL_PER_CUBIC_METRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::dec(1, -18) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `ZeptokatalPerCubicMetre` (zkat/m³), as a `CatalyticActivityConcentration`.
    pub const ZEPTOKATAL_PER_CUBIC_METRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::dec(1, -21) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `YoctokatalPerCubicMetre` (ykat/m³), as a `CatalyticActivityConcentration`.
    pub const YOCTOKATAL_PER_CUBIC_METRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::dec(1, -24) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `KilokatalPerLitre` (kkat/L), as a `CatalyticActivityConcentration`.
    pub const KILOKATAL_PER_LITRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::int(1000000) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `KatalPerLitre` (kat/L), as a `CatalyticActivityConcentration`.
    pub const KATAL_PER_LITRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::int(1000) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `MillikatalPerLitre` (mkat/L), as a `CatalyticActivityConcentration`.
    pub const MILLIKATAL_PER_LITRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `MicrokatalPerLitre` (μkat/L), as a `CatalyticActivityConcentration`.
    pub const MICROKATAL_PER_LITRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::dec(1, -3) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `NanokatalPerLitre` (nkat/L), as a `CatalyticActivityConcentration`.
    pub const NANOKATAL_PER_LITRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::dec(1, -6) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `PicokatalPerLitre` (pkat/L), as a `CatalyticActivityConcentration`.
    pub const PICOKATAL_PER_LITRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::dec(1, -9) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `FemtokatalPerLitre` (fkat/L), as a `CatalyticActivityConcentration`.
    pub const FEMTOKATAL_PER_LITRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted(Exact::dec(1, -12) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `ParticlePerSecondCubicMetre` (particle · s⁻¹ · m⁻³), as a `CatalyticActivityConcentration`.
    pub const PARTICLE_PER_SECOND_CUBIC_METRE<N: const From<Exact>>: CatalyticActivityConcentration<N> = counted((Exact::int(1) / Exact::int(602214076000000000000000)) * KATAL_PER_CUBIC_METRE::<Exact>);

    /// One `Yottabecquerel` (YBq), as a `Radioactivity`.
    pub const YOTTABECQUEREL<N: const From<Exact>>: Radioactivity<N> = counted(Exact::int(1000000000000000000000000) * BECQUEREL::<Exact>);

    /// One `Zettabecquerel` (ZBq), as a `Radioactivity`.
    pub const ZETTABECQUEREL<N: const From<Exact>>: Radioactivity<N> = counted(Exact::int(1000000000000000000000) * BECQUEREL::<Exact>);

    /// One `Exabecquerel` (EBq), as a `Radioactivity`.
    pub const EXABECQUEREL<N: const From<Exact>>: Radioactivity<N> = counted(Exact::int(1000000000000000000) * BECQUEREL::<Exact>);

    /// One `Petabecquerel` (PBq), as a `Radioactivity`.
    pub const PETABECQUEREL<N: const From<Exact>>: Radioactivity<N> = counted(Exact::int(1000000000000000) * BECQUEREL::<Exact>);

    /// One `Terabecquerel` (TBq), as a `Radioactivity`.
    pub const TERABECQUEREL<N: const From<Exact>>: Radioactivity<N> = counted(Exact::int(1000000000000) * BECQUEREL::<Exact>);

    /// One `Gigabecquerel` (GBq), as a `Radioactivity`.
    pub const GIGABECQUEREL<N: const From<Exact>>: Radioactivity<N> = counted(Exact::int(1000000000) * BECQUEREL::<Exact>);

    /// One `Megabecquerel` (MBq), as a `Radioactivity`.
    pub const MEGABECQUEREL<N: const From<Exact>>: Radioactivity<N> = counted(Exact::int(1000000) * BECQUEREL::<Exact>);

    /// One `Kilobecquerel` (kBq), as a `Radioactivity`.
    pub const KILOBECQUEREL<N: const From<Exact>>: Radioactivity<N> = counted(Exact::int(1000) * BECQUEREL::<Exact>);

    /// One `Hectobecquerel` (hBq), as a `Radioactivity`.
    pub const HECTOBECQUEREL<N: const From<Exact>>: Radioactivity<N> = counted(Exact::int(100) * BECQUEREL::<Exact>);

    /// One `Decabecquerel` (daBq), as a `Radioactivity`.
    pub const DECABECQUEREL<N: const From<Exact>>: Radioactivity<N> = counted(Exact::int(10) * BECQUEREL::<Exact>);

    /// One `Becquerel` (Bq), as a `Radioactivity`.
    pub const BECQUEREL<N: const From<Exact>>: Radioactivity<N> = counted(ONE::<Exact> / SECOND::<Exact>);

    /// One `Millibecquerel` (mBq), as a `Radioactivity`.
    pub const MILLIBECQUEREL<N: const From<Exact>>: Radioactivity<N> = counted(Exact::dec(1, -3) * BECQUEREL::<Exact>);

    /// One `Microbecquerel` (µBq), as a `Radioactivity`.
    pub const MICROBECQUEREL<N: const From<Exact>>: Radioactivity<N> = counted(Exact::dec(1, -6) * BECQUEREL::<Exact>);

    /// One `Nanobecquerel` (nBq), as a `Radioactivity`.
    pub const NANOBECQUEREL<N: const From<Exact>>: Radioactivity<N> = counted(Exact::dec(1, -9) * BECQUEREL::<Exact>);

    /// One `Gigacurie` (GCi), as a `Radioactivity`.
    pub const GIGACURIE<N: const From<Exact>>: Radioactivity<N> = counted(Exact::int(37000000000000000000) * BECQUEREL::<Exact>);

    /// One `Megacurie` (MCi), as a `Radioactivity`.
    pub const MEGACURIE<N: const From<Exact>>: Radioactivity<N> = counted(Exact::int(37000000000000000) * BECQUEREL::<Exact>);

    /// One `Kilocurie` (kCi), as a `Radioactivity`.
    pub const KILOCURIE<N: const From<Exact>>: Radioactivity<N> = counted(Exact::int(37000000000000) * BECQUEREL::<Exact>);

    /// One `Curie` (Ci), as a `Radioactivity`.
    pub const CURIE<N: const From<Exact>>: Radioactivity<N> = counted(Exact::int(37000000000) * BECQUEREL::<Exact>);

    /// One `Millicurie` (mCi), as a `Radioactivity`.
    pub const MILLICURIE<N: const From<Exact>>: Radioactivity<N> = counted(Exact::int(37000000) * BECQUEREL::<Exact>);

    /// One `Microcurie` (µCi), as a `Radioactivity`.
    pub const MICROCURIE<N: const From<Exact>>: Radioactivity<N> = counted(Exact::int(37000) * BECQUEREL::<Exact>);

    /// One `Nanocurie` (nCi), as a `Radioactivity`.
    pub const NANOCURIE<N: const From<Exact>>: Radioactivity<N> = counted(Exact::int(37) * BECQUEREL::<Exact>);

    /// One `DisintegrationsPerMinute` (dpm), as a `Radioactivity`.
    pub const DISINTEGRATIONS_PER_MINUTE<N: const From<Exact>>: Radioactivity<N> = counted((Exact::int(1) / Exact::int(60)) * BECQUEREL::<Exact>);

    /// One `BecquerelPerKilogram` (Bq/kg), as a `SpecificRadioactivity`.
    pub const BECQUEREL_PER_KILOGRAM<N: const From<Exact>>: SpecificRadioactivity<N> = counted(ONE::<Exact> / (KILOGRAM::<Exact> * SECOND::<Exact>));

    /// One `CuriePerKilogram` (Ci/kg), as a `SpecificRadioactivity`.
    pub const CURIE_PER_KILOGRAM<N: const From<Exact>>: SpecificRadioactivity<N> = counted(Exact::int(37000000000) * BECQUEREL_PER_KILOGRAM::<Exact>);

    /// One `DisintegrationsPerMinutePerKilogram` (dpm/kg), as a `SpecificRadioactivity`.
    pub const DISINTEGRATIONS_PER_MINUTE_PER_KILOGRAM<N: const From<Exact>>: SpecificRadioactivity<N> = counted((Exact::int(1) / Exact::int(60)) * BECQUEREL_PER_KILOGRAM::<Exact>);

    /// One `PerMole` (mol⁻¹), as an `InverseAmountOfSubstance`.
    pub const PER_MOLE<N: const From<Exact>>: InverseAmountOfSubstance<N> = counted(ONE::<Exact> / MOLE::<Exact>);

    /// One `AvogadroConstant` (N_A), as an `InverseAmountOfSubstance`.
    pub const AVOGADRO_CONSTANT<N: const From<Exact>>: InverseAmountOfSubstance<N> = counted(Exact::int(602214076000000000000000) * PER_MOLE::<Exact>);
}

/// The units of `AmountOfSubstance`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AmountOfSubstance<f64>)]
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
#[unit(of = MolarConcentration<f64>)]
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
#[unit(of = MolarEnergy<f64>)]
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
#[unit(of = MolarFlux<f64>)]
pub enum MolarFluxUnit {
    #[default]
    #[unit("mol/(m² · s)", "moles per square metre second", by = MOLE_PER_SQUARE_METRE_SECOND)]
    MolePerSquareMetreSecond,
}

/// The units of `MolarHeatCapacity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MolarHeatCapacity<f64>)]
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
#[unit(of = MolarMass<f64>)]
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
#[unit(of = MolarRadioactivity<f64>)]
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
#[unit(of = MolarVolume<f64>)]
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
#[unit(of = Molality<f64>)]
pub enum MolalityUnit {
    #[default]
    #[unit("mol/kg", "moles per kilogram", by = MOLE_PER_KILOGRAM)]
    MolePerKilogram,
}

/// The units of `MassConcentration`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassConcentration<f64>)]
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
#[unit(of = CatalyticActivity<f64>)]
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
#[unit(of = CatalyticActivityConcentration<f64>)]
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
#[unit(of = Radioactivity<f64>)]
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
#[unit(of = SpecificRadioactivity<f64>)]
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
#[unit(of = InverseAmountOfSubstance<f64>)]
pub enum InverseAmountOfSubstanceUnit {
    #[default]
    #[unit("mol⁻¹", "per mole", by = PER_MOLE)]
    PerMole,
    #[unit("N_A", "Avogadro constants", by = AVOGADRO_CONSTANT)]
    AvogadroConstant,
}
