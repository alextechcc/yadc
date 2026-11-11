use crate::{Exact, ISQ, SIQuantity, counted};
use consts::*;
use yadc::Dimension;

pub type DynamicViscosity<N> = SIQuantity<
    {
        ISQ {
            m: -1,
            kg: 1,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type KinematicViscosity<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type SurfaceTension<N> = SIQuantity<
    {
        ISQ {
            kg: 1,
            s: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type CompressibilityCoefficient<N> = SIQuantity<
    {
        ISQ {
            m: 1,
            kg: -1,
            s: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MassRate<N> = SIQuantity<
    {
        ISQ {
            kg: 1,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type VolumeRate<N> = SIQuantity<
    {
        ISQ {
            m: 3,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MassFlux<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            kg: 1,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MassPerEnergy<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            s: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type DiffusionCoefficient<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            s: -1,
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
    use crate::base::{KILOGRAM, METRE, SECOND};

    /// One `PascalSecond` (Pa · s), as a `DynamicViscosity`.
    pub const PASCAL_SECOND<N: const From<Exact>>: DynamicViscosity<N> = counted(KILOGRAM::<Exact> / (METRE::<Exact> * SECOND::<Exact>));

    /// One `MillipascalSecond` (mPa · s), as a `DynamicViscosity`.
    pub const MILLIPASCAL_SECOND<N: const From<Exact>>: DynamicViscosity<N> = counted(Exact::dec(1, -3) * PASCAL_SECOND::<Exact>);

    /// One `MicropascalSecond` (µPa · s), as a `DynamicViscosity`.
    pub const MICROPASCAL_SECOND<N: const From<Exact>>: DynamicViscosity<N> = counted(Exact::dec(1, -6) * PASCAL_SECOND::<Exact>);

    /// One `Poise` (P), as a `DynamicViscosity`.
    pub const POISE<N: const From<Exact>>: DynamicViscosity<N> = counted(Exact::dec(1, -1) * PASCAL_SECOND::<Exact>);

    /// One `Centipoise` (cP), as a `DynamicViscosity`.
    pub const CENTIPOISE<N: const From<Exact>>: DynamicViscosity<N> = counted(Exact::dec(1, -3) * PASCAL_SECOND::<Exact>);

    /// One `PoundPerFootSecond` (lb/(ft · s)), as a `DynamicViscosity`.
    pub const POUND_PER_FOOT_SECOND<N: const From<Exact>>: DynamicViscosity<N> = counted((Exact::dec(45359237, -8) / Exact::dec(3048, -4)) * PASCAL_SECOND::<Exact>);

    /// One `SlugPerFootSecond` (slug/(ft · s)), as a `DynamicViscosity`.
    pub const SLUG_PER_FOOT_SECOND<N: const From<Exact>>: DynamicViscosity<N> = counted(((Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4)) / Exact::dec(3048, -4)) * PASCAL_SECOND::<Exact>);

    /// One `GramPerCentimetreSecond` (g/(cm · s)), as a `DynamicViscosity`.
    pub const GRAM_PER_CENTIMETRE_SECOND<N: const From<Exact>>: DynamicViscosity<N> = counted(Exact::dec(1, -1) * PASCAL_SECOND::<Exact>);

    /// One `SquareMetrePerSecond` (m²/s), as a `KinematicViscosity`.
    pub const KINEMATIC_VISCOSITY_SQUARE_METRE_PER_SECOND<N: const From<Exact>>: KinematicViscosity<N> = counted(METRE::<Exact> * METRE::<Exact> / SECOND::<Exact>);

    /// One `SquareCentimetrePerSecond` (cm²/s), as a `KinematicViscosity`.
    pub const KINEMATIC_VISCOSITY_SQUARE_CENTIMETRE_PER_SECOND<N: const From<Exact>>: KinematicViscosity<N> = counted(Exact::dec(1, -4) * KINEMATIC_VISCOSITY_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `SquareMillimetrePerSecond` (mm²/s), as a `KinematicViscosity`.
    pub const KINEMATIC_VISCOSITY_SQUARE_MILLIMETRE_PER_SECOND<N: const From<Exact>>: KinematicViscosity<N> = counted(Exact::dec(1, -6) * KINEMATIC_VISCOSITY_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `SquareMicrometrePerSecond` (µm²/s), as a `KinematicViscosity`.
    pub const KINEMATIC_VISCOSITY_SQUARE_MICROMETRE_PER_SECOND<N: const From<Exact>>: KinematicViscosity<N> = counted(Exact::dec(1, -12) * KINEMATIC_VISCOSITY_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `SquareNanometrePerSecond` (nm²/s), as a `KinematicViscosity`.
    pub const KINEMATIC_VISCOSITY_SQUARE_NANOMETRE_PER_SECOND<N: const From<Exact>>: KinematicViscosity<N> = counted(Exact::dec(1, -18) * KINEMATIC_VISCOSITY_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `Stokes` (St), as a `KinematicViscosity`.
    pub const KINEMATIC_VISCOSITY_STOKES<N: const From<Exact>>: KinematicViscosity<N> = counted(Exact::dec(1, -4) * KINEMATIC_VISCOSITY_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `YottanewtonPerMetre` (YN/m), as a `SurfaceTension`.
    pub const YOTTANEWTON_PER_METRE<N: const From<Exact>>: SurfaceTension<N> = counted(Exact::int(1000000000000000000000000) * NEWTON_PER_METRE::<Exact>);

    /// One `ZettanewtonPerMetre` (ZN/m), as a `SurfaceTension`.
    pub const ZETTANEWTON_PER_METRE<N: const From<Exact>>: SurfaceTension<N> = counted(Exact::int(1000000000000000000000) * NEWTON_PER_METRE::<Exact>);

    /// One `ExanewtonPerMetre` (EN/m), as a `SurfaceTension`.
    pub const EXANEWTON_PER_METRE<N: const From<Exact>>: SurfaceTension<N> = counted(Exact::int(1000000000000000000) * NEWTON_PER_METRE::<Exact>);

    /// One `PetanewtonPerMetre` (PN/m), as a `SurfaceTension`.
    pub const PETANEWTON_PER_METRE<N: const From<Exact>>: SurfaceTension<N> = counted(Exact::int(1000000000000000) * NEWTON_PER_METRE::<Exact>);

    /// One `TeranewtonPerMetre` (TN/m), as a `SurfaceTension`.
    pub const TERANEWTON_PER_METRE<N: const From<Exact>>: SurfaceTension<N> = counted(Exact::int(1000000000000) * NEWTON_PER_METRE::<Exact>);

    /// One `GiganewtonPerMetre` (GN/m), as a `SurfaceTension`.
    pub const GIGANEWTON_PER_METRE<N: const From<Exact>>: SurfaceTension<N> = counted(Exact::int(1000000000) * NEWTON_PER_METRE::<Exact>);

    /// One `MeganewtonPerMetre` (MN/m), as a `SurfaceTension`.
    pub const MEGANEWTON_PER_METRE<N: const From<Exact>>: SurfaceTension<N> = counted(Exact::int(1000000) * NEWTON_PER_METRE::<Exact>);

    /// One `KilonewtonPerMetre` (kN/m), as a `SurfaceTension`.
    pub const KILONEWTON_PER_METRE<N: const From<Exact>>: SurfaceTension<N> = counted(Exact::int(1000) * NEWTON_PER_METRE::<Exact>);

    /// One `HectonewtonPerMetre` (hN/m), as a `SurfaceTension`.
    pub const HECTONEWTON_PER_METRE<N: const From<Exact>>: SurfaceTension<N> = counted(Exact::int(100) * NEWTON_PER_METRE::<Exact>);

    /// One `DecanewtonPerMetre` (daN/m), as a `SurfaceTension`.
    pub const DECANEWTON_PER_METRE<N: const From<Exact>>: SurfaceTension<N> = counted(Exact::int(10) * NEWTON_PER_METRE::<Exact>);

    /// One `NewtonPerMetre` (N/m), as a `SurfaceTension`.
    pub const NEWTON_PER_METRE<N: const From<Exact>>: SurfaceTension<N> = counted(KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact>));

    /// One `DecinewtonPerMetre` (dN/m), as a `SurfaceTension`.
    pub const DECINEWTON_PER_METRE<N: const From<Exact>>: SurfaceTension<N> = counted(Exact::dec(1, -1) * NEWTON_PER_METRE::<Exact>);

    /// One `CentinewtonPerMetre` (cN/m), as a `SurfaceTension`.
    pub const CENTINEWTON_PER_METRE<N: const From<Exact>>: SurfaceTension<N> = counted(Exact::dec(1, -2) * NEWTON_PER_METRE::<Exact>);

    /// One `MillinewtonPerMetre` (mN/m), as a `SurfaceTension`.
    pub const MILLINEWTON_PER_METRE<N: const From<Exact>>: SurfaceTension<N> = counted(Exact::dec(1, -3) * NEWTON_PER_METRE::<Exact>);

    /// One `MicronewtonPerMetre` (µN/m), as a `SurfaceTension`.
    pub const MICRONEWTON_PER_METRE<N: const From<Exact>>: SurfaceTension<N> = counted(Exact::dec(1, -6) * NEWTON_PER_METRE::<Exact>);

    /// One `NanonewtonPerMetre` (nN/m), as a `SurfaceTension`.
    pub const NANONEWTON_PER_METRE<N: const From<Exact>>: SurfaceTension<N> = counted(Exact::dec(1, -9) * NEWTON_PER_METRE::<Exact>);

    /// One `PiconewtonPerMetre` (pN/m), as a `SurfaceTension`.
    pub const PICONEWTON_PER_METRE<N: const From<Exact>>: SurfaceTension<N> = counted(Exact::dec(1, -12) * NEWTON_PER_METRE::<Exact>);

    /// One `FemtonewtonPerMetre` (fN/m), as a `SurfaceTension`.
    pub const FEMTONEWTON_PER_METRE<N: const From<Exact>>: SurfaceTension<N> = counted(Exact::dec(1, -15) * NEWTON_PER_METRE::<Exact>);

    /// One `AttonewtonPerMetre` (aN/m), as a `SurfaceTension`.
    pub const ATTONEWTON_PER_METRE<N: const From<Exact>>: SurfaceTension<N> = counted(Exact::dec(1, -18) * NEWTON_PER_METRE::<Exact>);

    /// One `ZeptonewtonPerMetre` (zN/m), as a `SurfaceTension`.
    pub const ZEPTONEWTON_PER_METRE<N: const From<Exact>>: SurfaceTension<N> = counted(Exact::dec(1, -21) * NEWTON_PER_METRE::<Exact>);

    /// One `YoctonewtonPerMetre` (yN/m), as a `SurfaceTension`.
    pub const YOCTONEWTON_PER_METRE<N: const From<Exact>>: SurfaceTension<N> = counted(Exact::dec(1, -24) * NEWTON_PER_METRE::<Exact>);

    /// One `KilogramForcePerMetre` (kgf/m), as a `SurfaceTension`.
    pub const KILOGRAM_FORCE_PER_METRE<N: const From<Exact>>: SurfaceTension<N> = counted(Exact::dec(980665, -5) * NEWTON_PER_METRE::<Exact>);

    /// One `OunceForcePerInch` (ozf/in), as a `SurfaceTension`.
    pub const OUNCE_FORCE_PER_INCH<N: const From<Exact>>: SurfaceTension<N> = counted(((Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::int(16)) / Exact::dec(254, -4)) * NEWTON_PER_METRE::<Exact>);

    /// One `PoundalPerInch` (pdl/in), as a `SurfaceTension`.
    pub const POUNDAL_PER_INCH<N: const From<Exact>>: SurfaceTension<N> = counted(((Exact::dec(45359237, -8) * Exact::dec(3048, -4)) / Exact::dec(254, -4)) * NEWTON_PER_METRE::<Exact>);

    /// One `PoundForcePerInch` (lbf/in), as a `SurfaceTension`.
    pub const POUND_FORCE_PER_INCH<N: const From<Exact>>: SurfaceTension<N> = counted(((Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / Exact::dec(254, -4)) * NEWTON_PER_METRE::<Exact>);

    /// One `JoulePerSquareMetre` (J/m²), as a `SurfaceTension`.
    pub const JOULE_PER_SQUARE_METRE<N: const From<Exact>>: SurfaceTension<N> = counted(NEWTON_PER_METRE::<Exact>);

    /// One `PerYottapascal` (YPa⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_YOTTAPASCAL<N: const From<Exact>>: CompressibilityCoefficient<N> = counted(Exact::dec(1, -24) * PER_PASCAL::<Exact>);

    /// One `PerZettapascal` (ZPa⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_ZETTAPASCAL<N: const From<Exact>>: CompressibilityCoefficient<N> = counted(Exact::dec(1, -21) * PER_PASCAL::<Exact>);

    /// One `PerExapascal` (EPa⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_EXAPASCAL<N: const From<Exact>>: CompressibilityCoefficient<N> = counted(Exact::dec(1, -18) * PER_PASCAL::<Exact>);

    /// One `PerPetapascal` (PPa⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_PETAPASCAL<N: const From<Exact>>: CompressibilityCoefficient<N> = counted(Exact::dec(1, -15) * PER_PASCAL::<Exact>);

    /// One `PerTerapascal` (TPa⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_TERAPASCAL<N: const From<Exact>>: CompressibilityCoefficient<N> = counted(Exact::dec(1, -12) * PER_PASCAL::<Exact>);

    /// One `PerGigapascal` (GPa⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_GIGAPASCAL<N: const From<Exact>>: CompressibilityCoefficient<N> = counted(Exact::dec(1, -9) * PER_PASCAL::<Exact>);

    /// One `PerMegapascal` (MPa⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_MEGAPASCAL<N: const From<Exact>>: CompressibilityCoefficient<N> = counted(Exact::dec(1, -6) * PER_PASCAL::<Exact>);

    /// One `PerKilopascal` (kPa⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_KILOPASCAL<N: const From<Exact>>: CompressibilityCoefficient<N> = counted(Exact::dec(1, -3) * PER_PASCAL::<Exact>);

    /// One `PerHectopascal` (hPa⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_HECTOPASCAL<N: const From<Exact>>: CompressibilityCoefficient<N> = counted(Exact::dec(1, -2) * PER_PASCAL::<Exact>);

    /// One `PerDecapascal` (daPa⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_DECAPASCAL<N: const From<Exact>>: CompressibilityCoefficient<N> = counted(Exact::dec(1, -1) * PER_PASCAL::<Exact>);

    /// One `PerPascal` (Pa⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_PASCAL<N: const From<Exact>>: CompressibilityCoefficient<N> = counted(METRE::<Exact> * SECOND::<Exact> * SECOND::<Exact> / KILOGRAM::<Exact>);

    /// One `PerDecipascal` (dPa⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_DECIPASCAL<N: const From<Exact>>: CompressibilityCoefficient<N> = counted(Exact::int(10) * PER_PASCAL::<Exact>);

    /// One `PerCentipascal` (cPa⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_CENTIPASCAL<N: const From<Exact>>: CompressibilityCoefficient<N> = counted(Exact::int(100) * PER_PASCAL::<Exact>);

    /// One `PerMillipascal` (mPa⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_MILLIPASCAL<N: const From<Exact>>: CompressibilityCoefficient<N> = counted(Exact::int(1000) * PER_PASCAL::<Exact>);

    /// One `PerMicropascal` (µPa⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_MICROPASCAL<N: const From<Exact>>: CompressibilityCoefficient<N> = counted(Exact::int(1000000) * PER_PASCAL::<Exact>);

    /// One `PerNanopascal` (nPa⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_NANOPASCAL<N: const From<Exact>>: CompressibilityCoefficient<N> = counted(Exact::int(1000000000) * PER_PASCAL::<Exact>);

    /// One `PerPicopascal` (pPa⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_PICOPASCAL<N: const From<Exact>>: CompressibilityCoefficient<N> = counted(Exact::int(1000000000000) * PER_PASCAL::<Exact>);

    /// One `PerFemtopascal` (fPa⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_FEMTOPASCAL<N: const From<Exact>>: CompressibilityCoefficient<N> = counted(Exact::int(1000000000000000) * PER_PASCAL::<Exact>);

    /// One `PerAttopascal` (aPa⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_ATTOPASCAL<N: const From<Exact>>: CompressibilityCoefficient<N> = counted(Exact::int(1000000000000000000) * PER_PASCAL::<Exact>);

    /// One `PerZeptopascal` (zPa⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_ZEPTOPASCAL<N: const From<Exact>>: CompressibilityCoefficient<N> = counted(Exact::int(1000000000000000000000) * PER_PASCAL::<Exact>);

    /// One `PerYoctopascal` (yPa⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_YOCTOPASCAL<N: const From<Exact>>: CompressibilityCoefficient<N> = counted(Exact::int(1000000000000000000000000) * PER_PASCAL::<Exact>);

    /// One `PerAtmosphere` (atm⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_ATMOSPHERE<N: const From<Exact>>: CompressibilityCoefficient<N> = counted((Exact::int(1) / Exact::int(101325)) * PER_PASCAL::<Exact>);

    /// One `PerAtmosphereTechnical` (at⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_ATMOSPHERE_TECHNICAL<N: const From<Exact>>: CompressibilityCoefficient<N> = counted((Exact::int(1) / Exact::dec(980665, -1)) * PER_PASCAL::<Exact>);

    /// One `PerBar` (bar⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_BAR<N: const From<Exact>>: CompressibilityCoefficient<N> = counted(Exact::dec(1, -5) * PER_PASCAL::<Exact>);

    /// One `PerMillibar` (mbar⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_MILLIBAR<N: const From<Exact>>: CompressibilityCoefficient<N> = counted(Exact::dec(1, -2) * PER_PASCAL::<Exact>);

    /// One `YottagramPerSecond` (Yg/s), as a `MassRate`.
    pub const YOTTAGRAM_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::int(1000000000000000000000) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `ZettagramPerSecond` (Zg/s), as a `MassRate`.
    pub const ZETTAGRAM_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::int(1000000000000000000) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `ExagramPerSecond` (Eg/s), as a `MassRate`.
    pub const EXAGRAM_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::int(1000000000000000) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `PetagramPerSecond` (Pg/s), as a `MassRate`.
    pub const PETAGRAM_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::int(1000000000000) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `TeragramPerSecond` (Tg/s), as a `MassRate`.
    pub const TERAGRAM_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::int(1000000000) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `GigagramPerSecond` (Gg/s), as a `MassRate`.
    pub const GIGAGRAM_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::int(1000000) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `MegagramPerSecond` (Mg/s), as a `MassRate`.
    pub const MEGAGRAM_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::int(1000) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `KilogramPerSecond` (kg/s), as a `MassRate`.
    pub const KILOGRAM_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(KILOGRAM::<Exact> / SECOND::<Exact>);

    /// One `HectogramPerSecond` (hg/s), as a `MassRate`.
    pub const HECTOGRAM_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::dec(1, -1) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `DecagramPerSecond` (dag/s), as a `MassRate`.
    pub const DECAGRAM_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::dec(1, -2) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `GramPerSecond` (g/s), as a `MassRate`.
    pub const GRAM_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::dec(1, -3) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `DecigramPerSecond` (dg/s), as a `MassRate`.
    pub const DECIGRAM_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::dec(1, -4) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `CentigramPerSecond` (cg/s), as a `MassRate`.
    pub const CENTIGRAM_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::dec(1, -5) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `MilligramPerSecond` (mg/s), as a `MassRate`.
    pub const MILLIGRAM_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::dec(1, -6) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `MicrogramPerSecond` (µg/s), as a `MassRate`.
    pub const MICROGRAM_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::dec(1, -9) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `NanogramPerSecond` (ng/s), as a `MassRate`.
    pub const NANOGRAM_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::dec(1, -12) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `PicogramPerSecond` (pg/s), as a `MassRate`.
    pub const PICOGRAM_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::dec(1, -15) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `FemtogramPerSecond` (fg/s), as a `MassRate`.
    pub const FEMTOGRAM_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::dec(1, -18) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `AttogramPerSecond` (ag/s), as a `MassRate`.
    pub const ATTOGRAM_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::dec(1, -21) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `ZeptogramPerSecond` (zg/s), as a `MassRate`.
    pub const ZEPTOGRAM_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::dec(1, -24) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `YoctogramPerSecond` (yg/s), as a `MassRate`.
    pub const YOCTOGRAM_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::dec(1, -27) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `KilogramPerMinute` (kg/min), as a `MassRate`.
    pub const KILOGRAM_PER_MINUTE<N: const From<Exact>>: MassRate<N> = counted((Exact::int(1) / Exact::int(60)) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `KilogramPerHour` (kg/h), as a `MassRate`.
    pub const KILOGRAM_PER_HOUR<N: const From<Exact>>: MassRate<N> = counted((Exact::int(1) / Exact::int(3600)) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `KilogramPerDay` (kg/d), as a `MassRate`.
    pub const KILOGRAM_PER_DAY<N: const From<Exact>>: MassRate<N> = counted((Exact::int(1) / Exact::int(86400)) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `GramPerMinute` (g/min), as a `MassRate`.
    pub const GRAM_PER_MINUTE<N: const From<Exact>>: MassRate<N> = counted((Exact::dec(1, -3) / Exact::int(60)) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `GramPerHour` (g/h), as a `MassRate`.
    pub const GRAM_PER_HOUR<N: const From<Exact>>: MassRate<N> = counted((Exact::dec(1, -3) / Exact::int(3600)) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `GramPerDay` (g/d), as a `MassRate`.
    pub const GRAM_PER_DAY<N: const From<Exact>>: MassRate<N> = counted((Exact::dec(1, -3) / Exact::int(86400)) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `CaratPerSecond` (ct/s), as a `MassRate`.
    pub const CARAT_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::dec(2, -4) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `GrainPerSecond` (gr/s), as a `MassRate`.
    pub const GRAIN_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::dec(6479891, -11) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `HundredweightLongPerSecond` (cwt long/s), as a `MassRate`.
    pub const HUNDREDWEIGHT_LONG_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::dec(5080234544, -8) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `HundredweightShortPerSecond` (cwt short/s), as a `MassRate`.
    pub const HUNDREDWEIGHT_SHORT_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::dec(45359237, -6) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `OuncePerSecond` (oz/s), as a `MassRate`.
    pub const OUNCE_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::dec(28349523125, -12) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `OunceTroyPerSecond` (oz t/s), as a `MassRate`.
    pub const OUNCE_TROY_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::dec(311034768, -10) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `PennyweightPerSecond` (dwt/s), as a `MassRate`.
    pub const PENNYWEIGHT_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::dec(155517384, -11) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `PoundPerSecond` (lb/s), as a `MassRate`.
    pub const POUND_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::dec(45359237, -8) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `PoundPerMinute` (lb/min), as a `MassRate`.
    pub const POUND_PER_MINUTE<N: const From<Exact>>: MassRate<N> = counted((Exact::dec(45359237, -8) / Exact::int(60)) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `PoundPerHour` (lb/h), as a `MassRate`.
    pub const POUND_PER_HOUR<N: const From<Exact>>: MassRate<N> = counted((Exact::dec(45359237, -8) / Exact::int(3600)) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `PoundPerDay` (lb/d), as a `MassRate`.
    pub const POUND_PER_DAY<N: const From<Exact>>: MassRate<N> = counted((Exact::dec(45359237, -8) / Exact::int(86400)) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `PoundTroyPerSecond` (lb t/s), as a `MassRate`.
    pub const POUND_TROY_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::dec(3732417216, -10) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `SlugPerSecond` (slug/s), as a `MassRate`.
    pub const SLUG_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted((Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4)) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `TonAssayPerSecond` (AT/s), as a `MassRate`.
    pub const TON_ASSAY_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted((Exact::dec(175, -3) / Exact::int(6)) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `TonLongPerSecond` (2240 lb/s), as a `MassRate`.
    pub const TON_LONG_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::dec(10160469088, -7) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `TonShortPerSecond` (2000 lb/s), as a `MassRate`.
    pub const TON_SHORT_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::dec(90718474, -5) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `TonShortPerHour` (2000 lb/h), as a `MassRate`.
    pub const TON_SHORT_PER_HOUR<N: const From<Exact>>: MassRate<N> = counted((Exact::dec(90718474, -5) / Exact::int(3600)) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `TonPerSecond` (t/s), as a `MassRate`.
    pub const TON_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::int(1000) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `TonPerMinute` (t/min), as a `MassRate`.
    pub const TON_PER_MINUTE<N: const From<Exact>>: MassRate<N> = counted((Exact::int(1000) / Exact::int(60)) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `TonPerHour` (t/h), as a `MassRate`.
    pub const TON_PER_HOUR<N: const From<Exact>>: MassRate<N> = counted((Exact::int(1000) / Exact::int(3600)) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `TonPerDay` (t/d), as a `MassRate`.
    pub const TON_PER_DAY<N: const From<Exact>>: MassRate<N> = counted((Exact::int(1000) / Exact::int(86400)) * KILOGRAM_PER_SECOND::<Exact>);

    /// One `CubicMetrePerSecond` (m³/s), as a `VolumeRate`.
    pub const CUBIC_METRE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(METRE::<Exact> * METRE::<Exact> * METRE::<Exact> / SECOND::<Exact>);

    /// One `CubicMetrePerMinute` (m³/min), as a `VolumeRate`.
    pub const CUBIC_METRE_PER_MINUTE<N: const From<Exact>>: VolumeRate<N> = counted((Exact::int(1) / Exact::int(60)) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `CubicMetrePerHour` (m³/h), as a `VolumeRate`.
    pub const CUBIC_METRE_PER_HOUR<N: const From<Exact>>: VolumeRate<N> = counted((Exact::int(1) / Exact::int(3600)) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `YottalitrePerSecond` (YL/s), as a `VolumeRate`.
    pub const YOTTALITRE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::int(1000000000000000000000) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `ZettalitrePerSecond` (ZL/s), as a `VolumeRate`.
    pub const ZETTALITRE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::int(1000000000000000000) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `ExalitrePerSecond` (EL/s), as a `VolumeRate`.
    pub const EXALITRE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::int(1000000000000000) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `PetalitrePerSecond` (PL/s), as a `VolumeRate`.
    pub const PETALITRE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::int(1000000000000) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `TeralitrePerSecond` (TL/s), as a `VolumeRate`.
    pub const TERALITRE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::int(1000000000) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `GigalitrePerSecond` (GL/s), as a `VolumeRate`.
    pub const GIGALITRE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::int(1000000) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `MegalitrePerSecond` (ML/s), as a `VolumeRate`.
    pub const MEGALITRE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::int(1000) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `KilolitrePerSecond` (kL/s), as a `VolumeRate`.
    pub const KILOLITRE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `HectolitrePerSecond` (hL/s), as a `VolumeRate`.
    pub const HECTOLITRE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(1, -1) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `DecalitrePerSecond` (daL/s), as a `VolumeRate`.
    pub const DECALITRE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(1, -2) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `LitrePerSecond` (L/s), as a `VolumeRate`.
    pub const LITRE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(1, -3) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `DecilitrePerSecond` (dL/s), as a `VolumeRate`.
    pub const DECILITRE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(1, -4) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `CentilitrePerSecond` (cL/s), as a `VolumeRate`.
    pub const CENTILITRE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(1, -5) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `MillilitrePerSecond` (mL/s), as a `VolumeRate`.
    pub const MILLILITRE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(1, -6) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `MicrolitrePerSecond` (µL/s), as a `VolumeRate`.
    pub const MICROLITRE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(1, -9) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `NanolitrePerSecond` (nL/s), as a `VolumeRate`.
    pub const NANOLITRE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(1, -12) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `PicolitrePerSecond` (pL/s), as a `VolumeRate`.
    pub const PICOLITRE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(1, -15) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `FemtolitrePerSecond` (fL/s), as a `VolumeRate`.
    pub const FEMTOLITRE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(1, -18) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `AttolitrePerSecond` (aL/s), as a `VolumeRate`.
    pub const ATTOLITRE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(1, -21) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `ZeptolitrePerSecond` (zL/s), as a `VolumeRate`.
    pub const ZEPTOLITRE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(1, -24) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `YoctolitrePerSecond` (yL/s), as a `VolumeRate`.
    pub const YOCTOLITRE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(1, -27) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `LitrePerMinute` (L/min), as a `VolumeRate`.
    pub const LITRE_PER_MINUTE<N: const From<Exact>>: VolumeRate<N> = counted((Exact::dec(1, -3) / Exact::int(60)) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `AcreFootPerSecond` (ac · ft/s), as a `VolumeRate`.
    pub const ACRE_FOOT_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted((Exact::int(43560) * (Exact::int(1200) / Exact::int(3937)) * (Exact::int(1200) / Exact::int(3937)) * (Exact::int(1200) / Exact::int(3937))) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `BarrelPerSecond` (bbl/s), as a `VolumeRate`.
    pub const BARREL_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(158987294928, -12) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `BushelPerSecond` (bu/s), as a `VolumeRate`.
    pub const BUSHEL_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(3523907016688, -14) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `CordPerSecond` (cords/s), as a `VolumeRate`.
    pub const CORD_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted((Exact::int(128) * Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `CubicFootPerSecond` (ft³/s), as a `VolumeRate`.
    pub const CUBIC_FOOT_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted((Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `CubicFootPerMinute` (ft³/min), as a `VolumeRate`.
    pub const CUBIC_FOOT_PER_MINUTE<N: const From<Exact>>: VolumeRate<N> = counted((Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4) / Exact::int(60)) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `CubicInchPerSecond` (in³/s), as a `VolumeRate`.
    pub const CUBIC_INCH_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted((Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4)) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `CubicInchPerMinute` (in³/min), as a `VolumeRate`.
    pub const CUBIC_INCH_PER_MINUTE<N: const From<Exact>>: VolumeRate<N> = counted((Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4) / Exact::int(60)) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `CubicMilePerSecond` (mi³/s), as a `VolumeRate`.
    pub const CUBIC_MILE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted((Exact::dec(1609344, -3) * Exact::dec(1609344, -3) * Exact::dec(1609344, -3)) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `CubicYardPerSecond` (yd³/s), as a `VolumeRate`.
    pub const CUBIC_YARD_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted((Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4)) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `CubicYardPerMinute` (yd³/min), as a `VolumeRate`.
    pub const CUBIC_YARD_PER_MINUTE<N: const From<Exact>>: VolumeRate<N> = counted((Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4) / Exact::int(60)) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `CupPerSecond` (cup/s), as a `VolumeRate`.
    pub const CUP_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(2365882365, -13) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `FluidOuncePerSecond` (fl oz/s), as a `VolumeRate`.
    pub const FLUID_OUNCE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(295735295625, -16) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `FluidOunceImperialPerSecond` (fl oz (UK)/s), as a `VolumeRate`.
    pub const FLUID_OUNCE_IMPERIAL_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(284130625, -13) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `GallonImperialPerSecond` (gal (UK)/s), as a `VolumeRate`.
    pub const GALLON_IMPERIAL_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(454609, -8) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `GallonPerSecond` (gal/s), as a `VolumeRate`.
    pub const GALLON_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(3785411784, -12) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `GallonPerMinute` (gal/min), as a `VolumeRate`.
    pub const GALLON_PER_MINUTE<N: const From<Exact>>: VolumeRate<N> = counted((Exact::dec(3785411784, -12) / Exact::int(60)) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `GallonPerDay` (gal/d), as a `VolumeRate`.
    pub const GALLON_PER_DAY<N: const From<Exact>>: VolumeRate<N> = counted((Exact::dec(3785411784, -12) / Exact::int(86400)) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `GillImperialPerSecond` (gi (UK)/s), as a `VolumeRate`.
    pub const GILL_IMPERIAL_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(1420653125, -13) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `GillPerSecond` (gi/s), as a `VolumeRate`.
    pub const GILL_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(11829411825, -14) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `PeckPerSecond` (pk/s), as a `VolumeRate`.
    pub const PECK_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(880976754172, -14) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `PintDryPerSecond` (dry pt/s), as a `VolumeRate`.
    pub const PINT_DRY_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(5506104713575, -16) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `PintLiquidPerSecond` (liq pt/s), as a `VolumeRate`.
    pub const PINT_LIQUID_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(473176473, -12) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `QuartDryPerSecond` (dry qt/s), as a `VolumeRate`.
    pub const QUART_DRY_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(1101220942715, -15) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `QuartLiquidPerSecond` (liq qt/s), as a `VolumeRate`.
    pub const QUART_LIQUID_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(946352946, -12) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `SterePerSecond` (st/s), as a `VolumeRate`.
    pub const STERE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `TablespoonPerSecond` (tbsp/s), as a `VolumeRate`.
    pub const TABLESPOON_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(1478676478125, -17) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `TeaspoonPerSecond` (tsp/s), as a `VolumeRate`.
    pub const TEASPOON_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(492892159375, -17) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `RegisterTonPerSecond` (RT/s), as a `VolumeRate`.
    pub const REGISTER_TON_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted((Exact::int(100) * Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)) * CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `KilogramPerSquareMetreSecond` (kg/(m² · s)), as a `MassFlux`.
    pub const KILOGRAM_PER_SQUARE_METRE_SECOND<N: const From<Exact>>: MassFlux<N> = counted(KILOGRAM::<Exact> / (METRE::<Exact> * METRE::<Exact> * SECOND::<Exact>));

    /// One `TeragramPerJoule` (Tg/J), as a `MassPerEnergy`.
    pub const TERAGRAM_PER_JOULE<N: const From<Exact>>: MassPerEnergy<N> = counted(Exact::int(1000000000) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `GigagramPerJoule` (Gg/J), as a `MassPerEnergy`.
    pub const GIGAGRAM_PER_JOULE<N: const From<Exact>>: MassPerEnergy<N> = counted(Exact::int(1000000) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `MegagramPerJoule` (Mg/J), as a `MassPerEnergy`.
    pub const MEGAGRAM_PER_JOULE<N: const From<Exact>>: MassPerEnergy<N> = counted(Exact::int(1000) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `KilogramPerJoule` (kg/J), as a `MassPerEnergy`.
    pub const KILOGRAM_PER_JOULE<N: const From<Exact>>: MassPerEnergy<N> = counted(SECOND::<Exact> * SECOND::<Exact> / (METRE::<Exact> * METRE::<Exact>));

    /// One `HectogramPerJoule` (hg/J), as a `MassPerEnergy`.
    pub const HECTOGRAM_PER_JOULE<N: const From<Exact>>: MassPerEnergy<N> = counted(Exact::dec(1, -1) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `DecagramPerJoule` (dag/J), as a `MassPerEnergy`.
    pub const DECAGRAM_PER_JOULE<N: const From<Exact>>: MassPerEnergy<N> = counted(Exact::dec(1, -2) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `GramPerJoule` (g/J), as a `MassPerEnergy`.
    pub const GRAM_PER_JOULE<N: const From<Exact>>: MassPerEnergy<N> = counted(Exact::dec(1, -3) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `DecigramPerJoule` (dg/J), as a `MassPerEnergy`.
    pub const DECIGRAM_PER_JOULE<N: const From<Exact>>: MassPerEnergy<N> = counted(Exact::dec(1, -4) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `CentigramPerJoule` (cg/J), as a `MassPerEnergy`.
    pub const CENTIGRAM_PER_JOULE<N: const From<Exact>>: MassPerEnergy<N> = counted(Exact::dec(1, -5) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `MilligramPerJoule` (mg/J), as a `MassPerEnergy`.
    pub const MILLIGRAM_PER_JOULE<N: const From<Exact>>: MassPerEnergy<N> = counted(Exact::dec(1, -6) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `MicrogramPerJoule` (µg/J), as a `MassPerEnergy`.
    pub const MICROGRAM_PER_JOULE<N: const From<Exact>>: MassPerEnergy<N> = counted(Exact::dec(1, -9) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `PoundPerJoule` (lb/J), as a `MassPerEnergy`.
    pub const POUND_PER_JOULE<N: const From<Exact>>: MassPerEnergy<N> = counted(Exact::dec(45359237, -8) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `PoundPerGigawattHour` (lb/GWh), as a `MassPerEnergy`.
    pub const POUND_PER_GIGAWATT_HOUR<N: const From<Exact>>: MassPerEnergy<N> = counted((Exact::dec(45359237, -8) / Exact::int(3600000000000)) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `PoundPerMegawattHour` (lb/MWh), as a `MassPerEnergy`.
    pub const POUND_PER_MEGAWATT_HOUR<N: const From<Exact>>: MassPerEnergy<N> = counted((Exact::dec(45359237, -8) / Exact::int(3600000000)) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `PoundPerKilowattHour` (lb/kWh), as a `MassPerEnergy`.
    pub const POUND_PER_KILOWATT_HOUR<N: const From<Exact>>: MassPerEnergy<N> = counted((Exact::dec(45359237, -8) / Exact::int(3600000)) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `PoundPerWattHour` (lb/Wh), as a `MassPerEnergy`.
    pub const POUND_PER_WATT_HOUR<N: const From<Exact>>: MassPerEnergy<N> = counted((Exact::dec(45359237, -8) / Exact::int(3600)) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `KilogramPerGigawattHour` (kg/GWh), as a `MassPerEnergy`.
    pub const KILOGRAM_PER_GIGAWATT_HOUR<N: const From<Exact>>: MassPerEnergy<N> = counted((Exact::int(1) / Exact::int(3600000000000)) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `KilogramPerMegawattHour` (kg/MWh), as a `MassPerEnergy`.
    pub const KILOGRAM_PER_MEGAWATT_HOUR<N: const From<Exact>>: MassPerEnergy<N> = counted((Exact::int(1) / Exact::int(3600000000)) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `KilogramPerKilowattHour` (kg/kWh), as a `MassPerEnergy`.
    pub const KILOGRAM_PER_KILOWATT_HOUR<N: const From<Exact>>: MassPerEnergy<N> = counted((Exact::int(1) / Exact::int(3600000)) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `KilogramPerWattHour` (kg/Wh), as a `MassPerEnergy`.
    pub const KILOGRAM_PER_WATT_HOUR<N: const From<Exact>>: MassPerEnergy<N> = counted((Exact::int(1) / Exact::int(3600)) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `GramPerGigawattHour` (g/GWh), as a `MassPerEnergy`.
    pub const GRAM_PER_GIGAWATT_HOUR<N: const From<Exact>>: MassPerEnergy<N> = counted((Exact::dec(1, -3) / Exact::int(3600000000000)) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `GramPerMegawattHour` (g/MWh), as a `MassPerEnergy`.
    pub const GRAM_PER_MEGAWATT_HOUR<N: const From<Exact>>: MassPerEnergy<N> = counted((Exact::dec(1, -3) / Exact::int(3600000000)) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `GramPerKilowattHour` (g/kWh), as a `MassPerEnergy`.
    pub const GRAM_PER_KILOWATT_HOUR<N: const From<Exact>>: MassPerEnergy<N> = counted((Exact::dec(1, -3) / Exact::int(3600000)) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `GramPerWattHour` (g/Wh), as a `MassPerEnergy`.
    pub const GRAM_PER_WATT_HOUR<N: const From<Exact>>: MassPerEnergy<N> = counted((Exact::dec(1, -3) / Exact::int(3600)) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `SquareMetrePerSecond` (m²/s), as a `DiffusionCoefficient`.
    pub const DIFFUSION_COEFFICIENT_SQUARE_METRE_PER_SECOND<N: const From<Exact>>: DiffusionCoefficient<N> = counted(METRE::<Exact> * METRE::<Exact> / SECOND::<Exact>);

    /// One `SquareCentimetrePerSecond` (cm²/s), as a `DiffusionCoefficient`.
    pub const DIFFUSION_COEFFICIENT_SQUARE_CENTIMETRE_PER_SECOND<N: const From<Exact>>: DiffusionCoefficient<N> = counted(Exact::dec(1, -4) * DIFFUSION_COEFFICIENT_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `SquareMillimetrePerSecond` (mm²/s), as a `DiffusionCoefficient`.
    pub const DIFFUSION_COEFFICIENT_SQUARE_MILLIMETRE_PER_SECOND<N: const From<Exact>>: DiffusionCoefficient<N> = counted(Exact::dec(1, -6) * DIFFUSION_COEFFICIENT_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `SquareMicrometrePerSecond` (µm²/s), as a `DiffusionCoefficient`.
    pub const DIFFUSION_COEFFICIENT_SQUARE_MICROMETRE_PER_SECOND<N: const From<Exact>>: DiffusionCoefficient<N> = counted(Exact::dec(1, -12) * DIFFUSION_COEFFICIENT_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `SquareNanometrePerSecond` (nm²/s), as a `DiffusionCoefficient`.
    pub const DIFFUSION_COEFFICIENT_SQUARE_NANOMETRE_PER_SECOND<N: const From<Exact>>: DiffusionCoefficient<N> = counted(Exact::dec(1, -18) * DIFFUSION_COEFFICIENT_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `Stokes` (St), as a `DiffusionCoefficient`.
    pub const DIFFUSION_COEFFICIENT_STOKES<N: const From<Exact>>: DiffusionCoefficient<N> = counted(Exact::dec(1, -4) * DIFFUSION_COEFFICIENT_SQUARE_METRE_PER_SECOND::<Exact>);
}

/// The units of `DynamicViscosity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = DynamicViscosity<f64>)]
pub enum DynamicViscosityUnit {
    #[default]
    #[unit("Pa · s", "pascal seconds", by = PASCAL_SECOND)]
    PascalSecond,
    #[unit("mPa · s", "millipascal seconds", by = MILLIPASCAL_SECOND)]
    MillipascalSecond,
    #[unit("µPa · s", "micropascal seconds", by = MICROPASCAL_SECOND)]
    MicropascalSecond,
    #[unit("P", "poises", by = POISE)]
    Poise,
    #[unit("cP", "centipoises", by = CENTIPOISE)]
    Centipoise,
    #[unit("lb/(ft · s)", "pounds per foot second", by = POUND_PER_FOOT_SECOND)]
    PoundPerFootSecond,
    #[unit("slug/(ft · s)", "slugs per foot second", by = SLUG_PER_FOOT_SECOND)]
    SlugPerFootSecond,
    #[unit("g/(cm · s)", "grams per centimetre second", by = GRAM_PER_CENTIMETRE_SECOND)]
    GramPerCentimetreSecond,
}

/// The units of `KinematicViscosity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = KinematicViscosity<f64>)]
pub enum KinematicViscosityUnit {
    #[default]
    #[unit("m²/s", "square metres per second", by = KINEMATIC_VISCOSITY_SQUARE_METRE_PER_SECOND)]
    SquareMetrePerSecond,
    #[unit("cm²/s", "square centimetres per second", by = KINEMATIC_VISCOSITY_SQUARE_CENTIMETRE_PER_SECOND)]
    SquareCentimetrePerSecond,
    #[unit("mm²/s", "square millimetres per second", by = KINEMATIC_VISCOSITY_SQUARE_MILLIMETRE_PER_SECOND)]
    SquareMillimetrePerSecond,
    #[unit("µm²/s", "square micrometres per second", by = KINEMATIC_VISCOSITY_SQUARE_MICROMETRE_PER_SECOND)]
    SquareMicrometrePerSecond,
    #[unit("nm²/s", "square nanometres per second", by = KINEMATIC_VISCOSITY_SQUARE_NANOMETRE_PER_SECOND)]
    SquareNanometrePerSecond,
    #[unit("St", "stokes", by = KINEMATIC_VISCOSITY_STOKES)]
    Stokes,
}

/// The units of `SurfaceTension`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SurfaceTension<f64>)]
pub enum SurfaceTensionUnit {
    #[unit("YN/m", "yottanewtons per metre", by = YOTTANEWTON_PER_METRE)]
    YottanewtonPerMetre,
    #[unit("ZN/m", "zettanewtons per metre", by = ZETTANEWTON_PER_METRE)]
    ZettanewtonPerMetre,
    #[unit("EN/m", "exanewtons per metre", by = EXANEWTON_PER_METRE)]
    ExanewtonPerMetre,
    #[unit("PN/m", "petanewtons per metre", by = PETANEWTON_PER_METRE)]
    PetanewtonPerMetre,
    #[unit("TN/m", "teranewtons per metre", by = TERANEWTON_PER_METRE)]
    TeranewtonPerMetre,
    #[unit("GN/m", "giganewtons per metre", by = GIGANEWTON_PER_METRE)]
    GiganewtonPerMetre,
    #[unit("MN/m", "meganewtons per metre", by = MEGANEWTON_PER_METRE)]
    MeganewtonPerMetre,
    #[unit("kN/m", "kilonewtons per metre", by = KILONEWTON_PER_METRE)]
    KilonewtonPerMetre,
    #[unit("hN/m", "hectonewtons per metre", by = HECTONEWTON_PER_METRE)]
    HectonewtonPerMetre,
    #[unit("daN/m", "decanewtons per metre", by = DECANEWTON_PER_METRE)]
    DecanewtonPerMetre,
    #[default]
    #[unit("N/m", "newtons per metre", by = NEWTON_PER_METRE)]
    NewtonPerMetre,
    #[unit("dN/m", "decinewtons per metre", by = DECINEWTON_PER_METRE)]
    DecinewtonPerMetre,
    #[unit("cN/m", "centinewtons per metre", by = CENTINEWTON_PER_METRE)]
    CentinewtonPerMetre,
    #[unit("mN/m", "millinewtons per metre", by = MILLINEWTON_PER_METRE)]
    MillinewtonPerMetre,
    #[unit("µN/m", "micronewtons per metre", by = MICRONEWTON_PER_METRE)]
    MicronewtonPerMetre,
    #[unit("nN/m", "nanonewtons per metre", by = NANONEWTON_PER_METRE)]
    NanonewtonPerMetre,
    #[unit("pN/m", "piconewtons per metre", by = PICONEWTON_PER_METRE)]
    PiconewtonPerMetre,
    #[unit("fN/m", "femtonewtons per metre", by = FEMTONEWTON_PER_METRE)]
    FemtonewtonPerMetre,
    #[unit("aN/m", "attonewtons per metre", by = ATTONEWTON_PER_METRE)]
    AttonewtonPerMetre,
    #[unit("zN/m", "zeptonewtons per metre", by = ZEPTONEWTON_PER_METRE)]
    ZeptonewtonPerMetre,
    #[unit("yN/m", "yoctonewtons per metre", by = YOCTONEWTON_PER_METRE)]
    YoctonewtonPerMetre,
    #[unit("kgf/m", "kilograms-force per metre", by = KILOGRAM_FORCE_PER_METRE)]
    KilogramForcePerMetre,
    #[unit("ozf/in", "ounces-force per inch", by = OUNCE_FORCE_PER_INCH)]
    OunceForcePerInch,
    #[unit("pdl/in", "poundals per inch", by = POUNDAL_PER_INCH)]
    PoundalPerInch,
    #[unit("lbf/in", "pounds-force per inch", by = POUND_FORCE_PER_INCH)]
    PoundForcePerInch,
    #[unit("J/m²", "joules per square metre", by = JOULE_PER_SQUARE_METRE)]
    JoulePerSquareMetre,
}

/// The units of `CompressibilityCoefficient`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = CompressibilityCoefficient<f64>)]
pub enum CompressibilityCoefficientUnit {
    #[unit("YPa⁻¹", "per yottapascal", by = PER_YOTTAPASCAL)]
    PerYottapascal,
    #[unit("ZPa⁻¹", "per zettapascal", by = PER_ZETTAPASCAL)]
    PerZettapascal,
    #[unit("EPa⁻¹", "per exapascal", by = PER_EXAPASCAL)]
    PerExapascal,
    #[unit("PPa⁻¹", "per petapascal", by = PER_PETAPASCAL)]
    PerPetapascal,
    #[unit("TPa⁻¹", "per terapascal", by = PER_TERAPASCAL)]
    PerTerapascal,
    #[unit("GPa⁻¹", "per gigapascal", by = PER_GIGAPASCAL)]
    PerGigapascal,
    #[unit("MPa⁻¹", "per megapascal", by = PER_MEGAPASCAL)]
    PerMegapascal,
    #[unit("kPa⁻¹", "per kilopascal", by = PER_KILOPASCAL)]
    PerKilopascal,
    #[unit("hPa⁻¹", "per hectopascal", by = PER_HECTOPASCAL)]
    PerHectopascal,
    #[unit("daPa⁻¹", "per decapascal", by = PER_DECAPASCAL)]
    PerDecapascal,
    #[default]
    #[unit("Pa⁻¹", "per pascal", by = PER_PASCAL)]
    PerPascal,
    #[unit("dPa⁻¹", "per decipascal", by = PER_DECIPASCAL)]
    PerDecipascal,
    #[unit("cPa⁻¹", "per centipascal", by = PER_CENTIPASCAL)]
    PerCentipascal,
    #[unit("mPa⁻¹", "per millipascal", by = PER_MILLIPASCAL)]
    PerMillipascal,
    #[unit("µPa⁻¹", "per micropascal", by = PER_MICROPASCAL)]
    PerMicropascal,
    #[unit("nPa⁻¹", "per nanopascal", by = PER_NANOPASCAL)]
    PerNanopascal,
    #[unit("pPa⁻¹", "per picopascal", by = PER_PICOPASCAL)]
    PerPicopascal,
    #[unit("fPa⁻¹", "per femtopascal", by = PER_FEMTOPASCAL)]
    PerFemtopascal,
    #[unit("aPa⁻¹", "per attopascal", by = PER_ATTOPASCAL)]
    PerAttopascal,
    #[unit("zPa⁻¹", "per zeptopascal", by = PER_ZEPTOPASCAL)]
    PerZeptopascal,
    #[unit("yPa⁻¹", "per yoctopascal", by = PER_YOCTOPASCAL)]
    PerYoctopascal,
    #[unit("atm⁻¹", "per atmosphere", by = PER_ATMOSPHERE)]
    PerAtmosphere,
    #[unit("at⁻¹", "per atmosphere (technical)", by = PER_ATMOSPHERE_TECHNICAL)]
    PerAtmosphereTechnical,
    #[unit("bar⁻¹", "per bar", by = PER_BAR)]
    PerBar,
    #[unit("mbar⁻¹", "per millibar", by = PER_MILLIBAR)]
    PerMillibar,
}

/// The units of `MassRate`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassRate<f64>)]
pub enum MassRateUnit {
    #[unit("Yg/s", "yottagrams per second", by = YOTTAGRAM_PER_SECOND)]
    YottagramPerSecond,
    #[unit("Zg/s", "zettagrams per second", by = ZETTAGRAM_PER_SECOND)]
    ZettagramPerSecond,
    #[unit("Eg/s", "exagrams per second", by = EXAGRAM_PER_SECOND)]
    ExagramPerSecond,
    #[unit("Pg/s", "petagrams per second", by = PETAGRAM_PER_SECOND)]
    PetagramPerSecond,
    #[unit("Tg/s", "teragrams per second", by = TERAGRAM_PER_SECOND)]
    TeragramPerSecond,
    #[unit("Gg/s", "gigagrams per second", by = GIGAGRAM_PER_SECOND)]
    GigagramPerSecond,
    #[unit("Mg/s", "megagrams per second", by = MEGAGRAM_PER_SECOND)]
    MegagramPerSecond,
    #[default]
    #[unit("kg/s", "kilograms per second", by = KILOGRAM_PER_SECOND)]
    KilogramPerSecond,
    #[unit("hg/s", "hectograms per second", by = HECTOGRAM_PER_SECOND)]
    HectogramPerSecond,
    #[unit("dag/s", "decagrams per second", by = DECAGRAM_PER_SECOND)]
    DecagramPerSecond,
    #[unit("g/s", "grams per second", by = GRAM_PER_SECOND)]
    GramPerSecond,
    #[unit("dg/s", "decigrams per second", by = DECIGRAM_PER_SECOND)]
    DecigramPerSecond,
    #[unit("cg/s", "centigrams per second", by = CENTIGRAM_PER_SECOND)]
    CentigramPerSecond,
    #[unit("mg/s", "milligrams per second", by = MILLIGRAM_PER_SECOND)]
    MilligramPerSecond,
    #[unit("µg/s", "micrograms per second", by = MICROGRAM_PER_SECOND)]
    MicrogramPerSecond,
    #[unit("ng/s", "nanograms per second", by = NANOGRAM_PER_SECOND)]
    NanogramPerSecond,
    #[unit("pg/s", "picograms per second", by = PICOGRAM_PER_SECOND)]
    PicogramPerSecond,
    #[unit("fg/s", "femtograms per second", by = FEMTOGRAM_PER_SECOND)]
    FemtogramPerSecond,
    #[unit("ag/s", "attograms per second", by = ATTOGRAM_PER_SECOND)]
    AttogramPerSecond,
    #[unit("zg/s", "zeptograms per second", by = ZEPTOGRAM_PER_SECOND)]
    ZeptogramPerSecond,
    #[unit("yg/s", "yoctograms per second", by = YOCTOGRAM_PER_SECOND)]
    YoctogramPerSecond,
    #[unit("kg/min", "kilograms per minute", by = KILOGRAM_PER_MINUTE)]
    KilogramPerMinute,
    #[unit("kg/h", "kilograms per hour", by = KILOGRAM_PER_HOUR)]
    KilogramPerHour,
    #[unit("kg/d", "kilograms per day", by = KILOGRAM_PER_DAY)]
    KilogramPerDay,
    #[unit("g/min", "grams per minute", by = GRAM_PER_MINUTE)]
    GramPerMinute,
    #[unit("g/h", "grams per hour", by = GRAM_PER_HOUR)]
    GramPerHour,
    #[unit("g/d", "grams per day", by = GRAM_PER_DAY)]
    GramPerDay,
    #[unit("ct/s", "carats per second", by = CARAT_PER_SECOND)]
    CaratPerSecond,
    #[unit("gr/s", "grains per second", by = GRAIN_PER_SECOND)]
    GrainPerSecond,
    #[unit("cwt long/s", "hundredweight (long) per second", by = HUNDREDWEIGHT_LONG_PER_SECOND)]
    HundredweightLongPerSecond,
    #[unit("cwt short/s", "hundredweight (short) per second", by = HUNDREDWEIGHT_SHORT_PER_SECOND)]
    HundredweightShortPerSecond,
    #[unit("oz/s", "ounces per second", by = OUNCE_PER_SECOND)]
    OuncePerSecond,
    #[unit("oz t/s", "troy ounces per second", by = OUNCE_TROY_PER_SECOND)]
    OunceTroyPerSecond,
    #[unit("dwt/s", "pennyweight per second", by = PENNYWEIGHT_PER_SECOND)]
    PennyweightPerSecond,
    #[unit("lb/s", "pounds per second", by = POUND_PER_SECOND)]
    PoundPerSecond,
    #[unit("lb/min", "pounds per minute", by = POUND_PER_MINUTE)]
    PoundPerMinute,
    #[unit("lb/h", "pounds per hour", by = POUND_PER_HOUR)]
    PoundPerHour,
    #[unit("lb/d", "pounds per day", by = POUND_PER_DAY)]
    PoundPerDay,
    #[unit("lb t/s", "troy pounds per second", by = POUND_TROY_PER_SECOND)]
    PoundTroyPerSecond,
    #[unit("slug/s", "slugs per second", by = SLUG_PER_SECOND)]
    SlugPerSecond,
    #[unit("AT/s", "assay tons per second", by = TON_ASSAY_PER_SECOND)]
    TonAssayPerSecond,
    #[unit("2240 lb/s", "long tons per second", by = TON_LONG_PER_SECOND)]
    TonLongPerSecond,
    #[unit("2000 lb/s", "short tons per second", by = TON_SHORT_PER_SECOND)]
    TonShortPerSecond,
    #[unit("2000 lb/h", "short tons per hour", by = TON_SHORT_PER_HOUR)]
    TonShortPerHour,
    #[unit("t/s", "tons per second", by = TON_PER_SECOND)]
    TonPerSecond,
    #[unit("t/min", "tons per minute", by = TON_PER_MINUTE)]
    TonPerMinute,
    #[unit("t/h", "tons per hour", by = TON_PER_HOUR)]
    TonPerHour,
    #[unit("t/d", "tons per day", by = TON_PER_DAY)]
    TonPerDay,
}

/// The units of `VolumeRate`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = VolumeRate<f64>)]
pub enum VolumeRateUnit {
    #[default]
    #[unit("m³/s", "cubic metres per second", by = CUBIC_METRE_PER_SECOND)]
    CubicMetrePerSecond,
    #[unit("m³/min", "cubic metres per minute", by = CUBIC_METRE_PER_MINUTE)]
    CubicMetrePerMinute,
    #[unit("m³/h", "cubic metres per hour", by = CUBIC_METRE_PER_HOUR)]
    CubicMetrePerHour,
    #[unit("YL/s", "yottalitres per second", by = YOTTALITRE_PER_SECOND)]
    YottalitrePerSecond,
    #[unit("ZL/s", "zettalitres per second", by = ZETTALITRE_PER_SECOND)]
    ZettalitrePerSecond,
    #[unit("EL/s", "exalitres per second", by = EXALITRE_PER_SECOND)]
    ExalitrePerSecond,
    #[unit("PL/s", "petalitres per second", by = PETALITRE_PER_SECOND)]
    PetalitrePerSecond,
    #[unit("TL/s", "teralitres per second", by = TERALITRE_PER_SECOND)]
    TeralitrePerSecond,
    #[unit("GL/s", "gigalitres per second", by = GIGALITRE_PER_SECOND)]
    GigalitrePerSecond,
    #[unit("ML/s", "megalitres per second", by = MEGALITRE_PER_SECOND)]
    MegalitrePerSecond,
    #[unit("kL/s", "kilolitres per second", by = KILOLITRE_PER_SECOND)]
    KilolitrePerSecond,
    #[unit("hL/s", "hectolitres per second", by = HECTOLITRE_PER_SECOND)]
    HectolitrePerSecond,
    #[unit("daL/s", "decalitres per second", by = DECALITRE_PER_SECOND)]
    DecalitrePerSecond,
    #[unit("L/s", "litres per second", by = LITRE_PER_SECOND)]
    LitrePerSecond,
    #[unit("dL/s", "decilitres per second", by = DECILITRE_PER_SECOND)]
    DecilitrePerSecond,
    #[unit("cL/s", "centilitres per second", by = CENTILITRE_PER_SECOND)]
    CentilitrePerSecond,
    #[unit("mL/s", "millilitres per second", by = MILLILITRE_PER_SECOND)]
    MillilitrePerSecond,
    #[unit("µL/s", "microlitres per second", by = MICROLITRE_PER_SECOND)]
    MicrolitrePerSecond,
    #[unit("nL/s", "nanolitres per second", by = NANOLITRE_PER_SECOND)]
    NanolitrePerSecond,
    #[unit("pL/s", "picolitres per second", by = PICOLITRE_PER_SECOND)]
    PicolitrePerSecond,
    #[unit("fL/s", "femtolitres per second", by = FEMTOLITRE_PER_SECOND)]
    FemtolitrePerSecond,
    #[unit("aL/s", "attolitres per second", by = ATTOLITRE_PER_SECOND)]
    AttolitrePerSecond,
    #[unit("zL/s", "zeptolitres per second", by = ZEPTOLITRE_PER_SECOND)]
    ZeptolitrePerSecond,
    #[unit("yL/s", "yoctolitres per second", by = YOCTOLITRE_PER_SECOND)]
    YoctolitrePerSecond,
    #[unit("L/min", "litres per minute", by = LITRE_PER_MINUTE)]
    LitrePerMinute,
    #[unit("ac · ft/s", "acre-feet per second", by = ACRE_FOOT_PER_SECOND)]
    AcreFootPerSecond,
    #[unit("bbl/s", "barrels per second", by = BARREL_PER_SECOND)]
    BarrelPerSecond,
    #[unit("bu/s", "bushels per second", by = BUSHEL_PER_SECOND)]
    BushelPerSecond,
    #[unit("cords/s", "cords per second", by = CORD_PER_SECOND)]
    CordPerSecond,
    #[unit("ft³/s", "cubic feet per second", by = CUBIC_FOOT_PER_SECOND)]
    CubicFootPerSecond,
    #[unit("ft³/min", "cubic feet per minute", by = CUBIC_FOOT_PER_MINUTE)]
    CubicFootPerMinute,
    #[unit("in³/s", "cubic inches per second", by = CUBIC_INCH_PER_SECOND)]
    CubicInchPerSecond,
    #[unit("in³/min", "cubic inches per minute", by = CUBIC_INCH_PER_MINUTE)]
    CubicInchPerMinute,
    #[unit("mi³/s", "cubic miles per second", by = CUBIC_MILE_PER_SECOND)]
    CubicMilePerSecond,
    #[unit("yd³/s", "cubic yards per second", by = CUBIC_YARD_PER_SECOND)]
    CubicYardPerSecond,
    #[unit("yd³/min", "cubic yards per minute", by = CUBIC_YARD_PER_MINUTE)]
    CubicYardPerMinute,
    #[unit("cup/s", "cups per second", by = CUP_PER_SECOND)]
    CupPerSecond,
    #[unit("fl oz/s", "fluid ounces per second", by = FLUID_OUNCE_PER_SECOND)]
    FluidOuncePerSecond,
    #[unit("fl oz (UK)/s", "Imperial fluid ounces per second", by = FLUID_OUNCE_IMPERIAL_PER_SECOND)]
    FluidOunceImperialPerSecond,
    #[unit("gal (UK)/s", "Imperial gallons per second", by = GALLON_IMPERIAL_PER_SECOND)]
    GallonImperialPerSecond,
    #[unit("gal/s", "gallons per second", by = GALLON_PER_SECOND)]
    GallonPerSecond,
    #[unit("gal/min", "gallons per minute", by = GALLON_PER_MINUTE)]
    GallonPerMinute,
    #[unit("gal/d", "gallons per day", by = GALLON_PER_DAY)]
    GallonPerDay,
    #[unit("gi (UK)/s", "Imperial gills per second", by = GILL_IMPERIAL_PER_SECOND)]
    GillImperialPerSecond,
    #[unit("gi/s", "gills per second", by = GILL_PER_SECOND)]
    GillPerSecond,
    #[unit("pk/s", "pecks per second", by = PECK_PER_SECOND)]
    PeckPerSecond,
    #[unit("dry pt/s", "dry pints per second", by = PINT_DRY_PER_SECOND)]
    PintDryPerSecond,
    #[unit("liq pt/s", "liquid pints per second", by = PINT_LIQUID_PER_SECOND)]
    PintLiquidPerSecond,
    #[unit("dry qt/s", "dry quarts per second", by = QUART_DRY_PER_SECOND)]
    QuartDryPerSecond,
    #[unit("liq qt/s", "liquid quarts per second", by = QUART_LIQUID_PER_SECOND)]
    QuartLiquidPerSecond,
    #[unit("st/s", "steres per second", by = STERE_PER_SECOND)]
    SterePerSecond,
    #[unit("tbsp/s", "tablespoons per second", by = TABLESPOON_PER_SECOND)]
    TablespoonPerSecond,
    #[unit("tsp/s", "teaspoons per second", by = TEASPOON_PER_SECOND)]
    TeaspoonPerSecond,
    #[unit("RT/s", "register tons per second", by = REGISTER_TON_PER_SECOND)]
    RegisterTonPerSecond,
}

/// The units of `MassFlux`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassFlux<f64>)]
pub enum MassFluxUnit {
    #[default]
    #[unit("kg/(m² · s)", "kilograms per square metre second", by = KILOGRAM_PER_SQUARE_METRE_SECOND)]
    KilogramPerSquareMetreSecond,
}

/// The units of `MassPerEnergy`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassPerEnergy<f64>)]
pub enum MassPerEnergyUnit {
    #[unit("Tg/J", "teragrams per joule", by = TERAGRAM_PER_JOULE)]
    TeragramPerJoule,
    #[unit("Gg/J", "gigagrams per joule", by = GIGAGRAM_PER_JOULE)]
    GigagramPerJoule,
    #[unit("Mg/J", "megagrams per joule", by = MEGAGRAM_PER_JOULE)]
    MegagramPerJoule,
    #[default]
    #[unit("kg/J", "kilograms per joule", by = KILOGRAM_PER_JOULE)]
    KilogramPerJoule,
    #[unit("hg/J", "hectograms per joule", by = HECTOGRAM_PER_JOULE)]
    HectogramPerJoule,
    #[unit("dag/J", "decagrams per joule", by = DECAGRAM_PER_JOULE)]
    DecagramPerJoule,
    #[unit("g/J", "grams per joule", by = GRAM_PER_JOULE)]
    GramPerJoule,
    #[unit("dg/J", "decigrams per joule", by = DECIGRAM_PER_JOULE)]
    DecigramPerJoule,
    #[unit("cg/J", "centigrams per joule", by = CENTIGRAM_PER_JOULE)]
    CentigramPerJoule,
    #[unit("mg/J", "milligrams per joule", by = MILLIGRAM_PER_JOULE)]
    MilligramPerJoule,
    #[unit("µg/J", "micrograms per joule", by = MICROGRAM_PER_JOULE)]
    MicrogramPerJoule,
    #[unit("lb/J", "pounds per joule", by = POUND_PER_JOULE)]
    PoundPerJoule,
    #[unit("lb/GWh", "pounds per gigawatt hour", by = POUND_PER_GIGAWATT_HOUR)]
    PoundPerGigawattHour,
    #[unit("lb/MWh", "pounds per megawatt hour", by = POUND_PER_MEGAWATT_HOUR)]
    PoundPerMegawattHour,
    #[unit("lb/kWh", "pounds per kilowatt hour", by = POUND_PER_KILOWATT_HOUR)]
    PoundPerKilowattHour,
    #[unit("lb/Wh", "pounds per watt hour", by = POUND_PER_WATT_HOUR)]
    PoundPerWattHour,
    #[unit("kg/GWh", "kilograms per gigawatt hour", by = KILOGRAM_PER_GIGAWATT_HOUR)]
    KilogramPerGigawattHour,
    #[unit("kg/MWh", "kilograms per megawatt hour", by = KILOGRAM_PER_MEGAWATT_HOUR)]
    KilogramPerMegawattHour,
    #[unit("kg/kWh", "kilograms per kilowatt hour", by = KILOGRAM_PER_KILOWATT_HOUR)]
    KilogramPerKilowattHour,
    #[unit("kg/Wh", "kilograms per watt hour", by = KILOGRAM_PER_WATT_HOUR)]
    KilogramPerWattHour,
    #[unit("g/GWh", "grams per gigawatt hour", by = GRAM_PER_GIGAWATT_HOUR)]
    GramPerGigawattHour,
    #[unit("g/MWh", "grams per megawatt hour", by = GRAM_PER_MEGAWATT_HOUR)]
    GramPerMegawattHour,
    #[unit("g/kWh", "grams per kilowatt hour", by = GRAM_PER_KILOWATT_HOUR)]
    GramPerKilowattHour,
    #[unit("g/Wh", "grams per watt hour", by = GRAM_PER_WATT_HOUR)]
    GramPerWattHour,
}

/// The units of `DiffusionCoefficient`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = DiffusionCoefficient<f64>)]
pub enum DiffusionCoefficientUnit {
    #[default]
    #[unit("m²/s", "square metres per second", by = DIFFUSION_COEFFICIENT_SQUARE_METRE_PER_SECOND)]
    SquareMetrePerSecond,
    #[unit("cm²/s", "square centimetres per second", by = DIFFUSION_COEFFICIENT_SQUARE_CENTIMETRE_PER_SECOND)]
    SquareCentimetrePerSecond,
    #[unit("mm²/s", "square millimetres per second", by = DIFFUSION_COEFFICIENT_SQUARE_MILLIMETRE_PER_SECOND)]
    SquareMillimetrePerSecond,
    #[unit("µm²/s", "square micrometres per second", by = DIFFUSION_COEFFICIENT_SQUARE_MICROMETRE_PER_SECOND)]
    SquareMicrometrePerSecond,
    #[unit("nm²/s", "square nanometres per second", by = DIFFUSION_COEFFICIENT_SQUARE_NANOMETRE_PER_SECOND)]
    SquareNanometrePerSecond,
    #[unit("St", "Stokes", by = DIFFUSION_COEFFICIENT_STOKES)]
    Stokes,
}
