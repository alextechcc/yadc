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
/// `INCH` is `0.0254 * METRE` - and counted in the number type its use asks
/// for, rounded once: `let d: Length<f64> = INCH;` is an `f64` length,
/// `Length<f32>` an `f32` one, `INCH::<Exact>` the exact value. Each is a
/// unit (`INCH(3.0)`, `INCH.value(d)`). A prefixed unit is a prefix times
/// one: `KILO * METRE`. They compose in `const` items:
/// `const LIMIT: Velocity<f64> = 50.0 * (KILO * METRE) / HOUR;`.
pub mod consts {
    use super::*;
    use crate::base::{KILOGRAM, METRE, SECOND};

    /// One `PascalSecond` (Pa · s), as a `DynamicViscosity`.
    pub const PASCAL_SECOND<N: const From<Exact>>: DynamicViscosity<N> = counted(KILOGRAM::<Exact> / (METRE::<Exact> * SECOND::<Exact>));

    /// One `Poise` (P), as a `DynamicViscosity`.
    pub const POISE<N: const From<Exact>>: DynamicViscosity<N> = counted(Exact::dec(1, -1) * PASCAL_SECOND::<Exact>);

    /// One `PoundPerFootSecond` (lb/(ft · s)), as a `DynamicViscosity`.
    pub const POUND_PER_FOOT_SECOND<N: const From<Exact>>: DynamicViscosity<N> = counted((Exact::dec(45359237, -8) / Exact::dec(3048, -4)) * PASCAL_SECOND::<Exact>);

    /// One `SlugPerFootSecond` (slug/(ft · s)), as a `DynamicViscosity`.
    pub const SLUG_PER_FOOT_SECOND<N: const From<Exact>>: DynamicViscosity<N> = counted(((Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4)) / Exact::dec(3048, -4)) * PASCAL_SECOND::<Exact>);

    /// One `GramPerCentimetreSecond` (g/(cm · s)), as a `DynamicViscosity`.
    pub const GRAM_PER_CENTIMETRE_SECOND<N: const From<Exact>>: DynamicViscosity<N> = counted(Exact::dec(1, -1) * PASCAL_SECOND::<Exact>);

    /// One `SquareMetrePerSecond` (m²/s), as a `KinematicViscosity`.
    pub const KINEMATIC_VISCOSITY_SQUARE_METRE_PER_SECOND<N: const From<Exact>>: KinematicViscosity<N> = counted(METRE::<Exact> * METRE::<Exact> / SECOND::<Exact>);

    /// One `Stokes` (St), as a `KinematicViscosity`.
    pub const KINEMATIC_VISCOSITY_STOKES<N: const From<Exact>>: KinematicViscosity<N> = counted(Exact::dec(1, -4) * KINEMATIC_VISCOSITY_SQUARE_METRE_PER_SECOND::<Exact>);

    /// One `NewtonPerMetre` (N/m), as a `SurfaceTension`.
    pub const NEWTON_PER_METRE<N: const From<Exact>>: SurfaceTension<N> = counted(KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact>));

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

    /// One `PerPascal` (Pa⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_PASCAL<N: const From<Exact>>: CompressibilityCoefficient<N> = counted(METRE::<Exact> * SECOND::<Exact> * SECOND::<Exact> / KILOGRAM::<Exact>);

    /// One `PerAtmosphere` (atm⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_ATMOSPHERE<N: const From<Exact>>: CompressibilityCoefficient<N> = counted((Exact::int(1) / Exact::int(101325)) * PER_PASCAL::<Exact>);

    /// One `PerAtmosphereTechnical` (at⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_ATMOSPHERE_TECHNICAL<N: const From<Exact>>: CompressibilityCoefficient<N> = counted((Exact::int(1) / Exact::dec(980665, -1)) * PER_PASCAL::<Exact>);

    /// One `PerBar` (bar⁻¹), as a `CompressibilityCoefficient`.
    pub const PER_BAR<N: const From<Exact>>: CompressibilityCoefficient<N> = counted(Exact::dec(1, -5) * PER_PASCAL::<Exact>);

    /// One `KilogramPerSecond` (kg/s), as a `MassRate`.
    pub const KILOGRAM_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(KILOGRAM::<Exact> / SECOND::<Exact>);

    /// One `GramPerSecond` (g/s), as a `MassRate`.
    pub const GRAM_PER_SECOND<N: const From<Exact>>: MassRate<N> = counted(Exact::dec(1, -3) * KILOGRAM_PER_SECOND::<Exact>);

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

    /// One `KilolitrePerSecond` (kL/s), as a `VolumeRate`.
    pub const KILOLITRE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(CUBIC_METRE_PER_SECOND::<Exact>);

    /// One `LitrePerSecond` (L/s), as a `VolumeRate`.
    pub const LITRE_PER_SECOND<N: const From<Exact>>: VolumeRate<N> = counted(Exact::dec(1, -3) * CUBIC_METRE_PER_SECOND::<Exact>);

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

    /// One `KilogramPerJoule` (kg/J), as a `MassPerEnergy`.
    pub const KILOGRAM_PER_JOULE<N: const From<Exact>>: MassPerEnergy<N> = counted(SECOND::<Exact> * SECOND::<Exact> / (METRE::<Exact> * METRE::<Exact>));

    /// One `GramPerJoule` (g/J), as a `MassPerEnergy`.
    pub const GRAM_PER_JOULE<N: const From<Exact>>: MassPerEnergy<N> = counted(Exact::dec(1, -3) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `PoundPerJoule` (lb/J), as a `MassPerEnergy`.
    pub const POUND_PER_JOULE<N: const From<Exact>>: MassPerEnergy<N> = counted(Exact::dec(45359237, -8) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `PoundPerWattHour` (lb/Wh), as a `MassPerEnergy`.
    pub const POUND_PER_WATT_HOUR<N: const From<Exact>>: MassPerEnergy<N> = counted((Exact::dec(45359237, -8) / Exact::int(3600)) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `GramPerWattHour` (g/Wh), as a `MassPerEnergy`.
    pub const GRAM_PER_WATT_HOUR<N: const From<Exact>>: MassPerEnergy<N> = counted((Exact::dec(1, -3) / Exact::int(3600)) * KILOGRAM_PER_JOULE::<Exact>);

    /// One `SquareMetrePerSecond` (m²/s), as a `DiffusionCoefficient`.
    pub const DIFFUSION_COEFFICIENT_SQUARE_METRE_PER_SECOND<N: const From<Exact>>: DiffusionCoefficient<N> = counted(METRE::<Exact> * METRE::<Exact> / SECOND::<Exact>);

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
    #[unit("P", "poises", by = POISE)]
    Poise,
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
    #[unit("St", "stokes", by = KINEMATIC_VISCOSITY_STOKES)]
    Stokes,
}

/// The units of `SurfaceTension`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SurfaceTension<f64>)]
pub enum SurfaceTensionUnit {
    #[default]
    #[unit("N/m", "newtons per metre", by = NEWTON_PER_METRE)]
    NewtonPerMetre,
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
    #[default]
    #[unit("Pa⁻¹", "per pascal", by = PER_PASCAL)]
    PerPascal,
    #[unit("atm⁻¹", "per atmosphere", by = PER_ATMOSPHERE)]
    PerAtmosphere,
    #[unit("at⁻¹", "per atmosphere (technical)", by = PER_ATMOSPHERE_TECHNICAL)]
    PerAtmosphereTechnical,
    #[unit("bar⁻¹", "per bar", by = PER_BAR)]
    PerBar,
}

/// The units of `MassRate`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassRate<f64>)]
pub enum MassRateUnit {
    #[default]
    #[unit("kg/s", "kilograms per second", by = KILOGRAM_PER_SECOND)]
    KilogramPerSecond,
    #[unit("g/s", "grams per second", by = GRAM_PER_SECOND)]
    GramPerSecond,
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
    #[unit("kL/s", "kilolitres per second", by = KILOLITRE_PER_SECOND)]
    KilolitrePerSecond,
    #[unit("L/s", "litres per second", by = LITRE_PER_SECOND)]
    LitrePerSecond,
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
    #[default]
    #[unit("kg/J", "kilograms per joule", by = KILOGRAM_PER_JOULE)]
    KilogramPerJoule,
    #[unit("g/J", "grams per joule", by = GRAM_PER_JOULE)]
    GramPerJoule,
    #[unit("lb/J", "pounds per joule", by = POUND_PER_JOULE)]
    PoundPerJoule,
    #[unit("lb/Wh", "pounds per watt hour", by = POUND_PER_WATT_HOUR)]
    PoundPerWattHour,
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
    #[unit("St", "Stokes", by = DIFFUSION_COEFFICIENT_STOKES)]
    Stokes,
}
