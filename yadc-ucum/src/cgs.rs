use crate::{Dim, Exact, Qty};
use consts::*;
use yadc::Dimension;

pub type Acceleration<N> = Qty<
    {
        Dim {
            m: 1,
            s: -2,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Brightness<N> = Qty<
    {
        Dim {
            m: -2,
            cd: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type DoseEquivalent<N> = Qty<
    {
        Dim {
            m: 2,
            s: -2,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type DynamicViscosity<N> = Qty<
    {
        Dim {
            m: -1,
            s: -1,
            g: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricCurrent<N> = Qty<
    {
        Dim {
            s: -1,
            C: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Energy<N> = Qty<
    {
        Dim {
            m: 2,
            s: -2,
            g: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type EnergyDose<N> = Qty<
    {
        Dim {
            m: 2,
            s: -2,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type FluxOfMagneticInduction<N> = Qty<
    {
        Dim {
            m: 2,
            s: -1,
            g: 1,
            C: -1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Force<N> = Qty<
    {
        Dim {
            m: 1,
            s: -2,
            g: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Illuminance<N> = Qty<
    {
        Dim {
            m: -2,
            rad: 2,
            cd: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type IonDose<N> = Qty<
    {
        Dim {
            g: -1,
            C: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type KinematicViscosity<N> = Qty<
    {
        Dim {
            m: 2,
            s: -1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type LineicNumber<N> = Qty<
    {
        Dim {
            m: -1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type LumIntensityDensity<N> = Qty<
    {
        Dim {
            m: -2,
            cd: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type MagneticFieldIntensity<N> = Qty<
    {
        Dim {
            m: -1,
            s: -1,
            C: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type MagneticFluxDensity<N> = Qty<
    {
        Dim {
            s: -1,
            g: 1,
            C: -1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type MagneticTension<N> = Qty<
    {
        Dim {
            s: -1,
            C: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Radioactivity<N> = Qty<
    {
        Dim {
            s: -1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

/// The constants of this module: the base constants of its quantities
/// (re-exported), then one per named unit, each defined exactly from them
/// (`INCH` is `0.0254 * METRE`). A unit written as others is not one:
/// `SLUG / CUBIC_FOOT`, `KILO * METRE` are exact too. Converting rounds,
/// once, where it is written - in a `const`, by the compiler:
/// `const MPH: Velocity<f64> = (MILE / HOUR).into();`, then `MPH(60.0)`.
pub mod consts {
    use super::*;
    use crate::base::{CANDELA, COULOMB, GRAM, METRE, ONE, RADIAN, SECOND};

    /// One `Gal` (`Gal`), as an `Acceleration`.
    pub const GAL: Acceleration<Exact> = (METRE / (SECOND * SECOND)) * Exact::dec(1, -2);

    /// One `Lambert` (`Lmb`), as a `Brightness`.
    pub const LAMBERT: Brightness<Exact> =
        (CANDELA / (METRE * METRE)) * (Exact::int(10000) / Exact::PI);

    /// One `RadiationEquivalentMan` (`REM`), as a `DoseEquivalent`.
    pub const RADIATION_EQUIVALENT_MAN: DoseEquivalent<Exact> =
        (METRE * METRE / (SECOND * SECOND)) * Exact::dec(1, -2);

    /// One `Poise` (`P`), as a `DynamicViscosity`.
    pub const POISE: DynamicViscosity<Exact> = (GRAM / (METRE * SECOND)) * Exact::int(100);

    /// One `Biot` (`Bi`), as an `ElectricCurrent`.
    pub const BIOT: ElectricCurrent<Exact> = (COULOMB / SECOND) * Exact::int(10);

    /// One `Erg` (`erg`), as an `Energy`.
    pub const ERG: Energy<Exact> = (METRE * METRE * GRAM / (SECOND * SECOND)) * Exact::dec(1, -4);

    /// One `RadiationAbsorbedDose` (`RAD`), as an `EnergyDose`.
    pub const RADIATION_ABSORBED_DOSE: EnergyDose<Exact> =
        (METRE * METRE / (SECOND * SECOND)) * Exact::dec(1, -2);

    /// One `Maxwell` (`Mx`), as a `FluxOfMagneticInduction`.
    pub const MAXWELL: FluxOfMagneticInduction<Exact> =
        (METRE * METRE * GRAM / (SECOND * COULOMB)) * Exact::dec(1, -5);

    /// One `Dyne` (`dyn`), as a `Force`.
    pub const DYNE: Force<Exact> = (METRE * GRAM / (SECOND * SECOND)) * Exact::dec(1, -2);

    /// One `Phot` (`ph`), as an `Illuminance`.
    pub const PHOT: Illuminance<Exact> =
        (RADIAN * RADIAN * CANDELA / (METRE * METRE)) * Exact::dec(1, -4);

    /// One `Roentgen` (`R`), as an `IonDose`.
    pub const ROENTGEN: IonDose<Exact> = (COULOMB / GRAM) * Exact::dec(258, -9);

    /// One `Stokes` (`St`), as a `KinematicViscosity`.
    pub const STOKES: KinematicViscosity<Exact> = (METRE * METRE / SECOND) * Exact::dec(1, -4);

    /// One `Kayser` (`Ky`), as a `LineicNumber`.
    pub const KAYSER: LineicNumber<Exact> = (ONE / METRE) * Exact::int(100);

    /// One `Stilb` (`sb`), as a `LumIntensityDensity`.
    pub const STILB: LumIntensityDensity<Exact> = (CANDELA / (METRE * METRE)) * Exact::int(10000);

    /// One `Oersted` (`Oe`), as a `MagneticFieldIntensity`.
    pub const OERSTED: MagneticFieldIntensity<Exact> =
        (COULOMB / (METRE * SECOND)) * (Exact::int(250) / Exact::PI);

    /// One `Gauss` (`G`), as a `MagneticFluxDensity`.
    pub const GAUSS: MagneticFluxDensity<Exact> = (GRAM / (SECOND * COULOMB)) * Exact::dec(1, -1);

    /// One `Gilbert` (`Gb`), as a `MagneticTension`.
    pub const GILBERT: MagneticTension<Exact> =
        (COULOMB / SECOND) * (Exact::dec(25, -1) / Exact::PI);

    /// One `Curie` (`Ci`), as a `Radioactivity`.
    pub const CURIE: Radioactivity<Exact> = (ONE / SECOND) * Exact::int(37000000000);
}

/// The units of `Acceleration`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Acceleration<N>, number = N)]
#[unit(for<N> where Acceleration<N>: [const] yadc::Unit<Acceleration<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum AccelerationUnit {
    #[default]
    #[unit("Gal", "Gal", by = Acceleration::<N>::from(GAL))]
    Gal,
}

/// The units of `Brightness`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Brightness<N>, number = N)]
#[unit(for<N> where Brightness<N>: [const] yadc::Unit<Brightness<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum BrightnessUnit {
    #[default]
    #[unit("Lmb", "Lambert", by = Brightness::<N>::from(LAMBERT))]
    Lambert,
}

/// The units of `DoseEquivalent`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = DoseEquivalent<N>, number = N)]
#[unit(for<N> where DoseEquivalent<N>: [const] yadc::Unit<DoseEquivalent<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum DoseEquivalentUnit {
    #[default]
    #[unit("REM", "radiation equivalent man", by = DoseEquivalent::<N>::from(RADIATION_EQUIVALENT_MAN))]
    RadiationEquivalentMan,
}

/// The units of `DynamicViscosity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = DynamicViscosity<N>, number = N)]
#[unit(for<N> where DynamicViscosity<N>: [const] yadc::Unit<DynamicViscosity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum DynamicViscosityUnit {
    #[default]
    #[unit("P", "Poise", by = DynamicViscosity::<N>::from(POISE))]
    Poise,
}

/// The units of `ElectricCurrent`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCurrent<N>, number = N)]
#[unit(for<N> where ElectricCurrent<N>: [const] yadc::Unit<ElectricCurrent<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ElectricCurrentUnit {
    #[default]
    #[unit("Bi", "Biot", by = ElectricCurrent::<N>::from(BIOT))]
    Biot,
}

/// The units of `Energy`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Energy<N>, number = N)]
#[unit(for<N> where Energy<N>: [const] yadc::Unit<Energy<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum EnergyUnit {
    #[default]
    #[unit("erg", "erg", by = Energy::<N>::from(ERG))]
    Erg,
}

/// The units of `EnergyDose`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = EnergyDose<N>, number = N)]
#[unit(for<N> where EnergyDose<N>: [const] yadc::Unit<EnergyDose<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum EnergyDoseUnit {
    #[default]
    #[unit("RAD", "radiation absorbed dose", by = EnergyDose::<N>::from(RADIATION_ABSORBED_DOSE))]
    RadiationAbsorbedDose,
}

/// The units of `FluxOfMagneticInduction`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = FluxOfMagneticInduction<N>, number = N)]
#[unit(for<N> where FluxOfMagneticInduction<N>: [const] yadc::Unit<FluxOfMagneticInduction<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum FluxOfMagneticInductionUnit {
    #[default]
    #[unit("Mx", "Maxwell", by = FluxOfMagneticInduction::<N>::from(MAXWELL))]
    Maxwell,
}

/// The units of `Force`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Force<N>, number = N)]
#[unit(for<N> where Force<N>: [const] yadc::Unit<Force<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ForceUnit {
    #[default]
    #[unit("dyn", "dyne", by = Force::<N>::from(DYNE))]
    Dyne,
}

/// The units of `Illuminance`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Illuminance<N>, number = N)]
#[unit(for<N> where Illuminance<N>: [const] yadc::Unit<Illuminance<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum IlluminanceUnit {
    #[default]
    #[unit("ph", "phot", by = Illuminance::<N>::from(PHOT))]
    Phot,
}

/// The units of `IonDose`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = IonDose<N>, number = N)]
#[unit(for<N> where IonDose<N>: [const] yadc::Unit<IonDose<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum IonDoseUnit {
    #[default]
    #[unit("R", "Roentgen", by = IonDose::<N>::from(ROENTGEN))]
    Roentgen,
}

/// The units of `KinematicViscosity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = KinematicViscosity<N>, number = N)]
#[unit(for<N> where KinematicViscosity<N>: [const] yadc::Unit<KinematicViscosity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum KinematicViscosityUnit {
    #[default]
    #[unit("St", "Stokes", by = KinematicViscosity::<N>::from(STOKES))]
    Stokes,
}

/// The units of `LineicNumber`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LineicNumber<N>, number = N)]
#[unit(for<N> where LineicNumber<N>: [const] yadc::Unit<LineicNumber<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum LineicNumberUnit {
    #[default]
    #[unit("Ky", "Kayser", by = LineicNumber::<N>::from(KAYSER))]
    Kayser,
}

/// The units of `LumIntensityDensity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LumIntensityDensity<N>, number = N)]
#[unit(for<N> where LumIntensityDensity<N>: [const] yadc::Unit<LumIntensityDensity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum LumIntensityDensityUnit {
    #[default]
    #[unit("sb", "stilb", by = LumIntensityDensity::<N>::from(STILB))]
    Stilb,
}

/// The units of `MagneticFieldIntensity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticFieldIntensity<N>, number = N)]
#[unit(for<N> where MagneticFieldIntensity<N>: [const] yadc::Unit<MagneticFieldIntensity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MagneticFieldIntensityUnit {
    #[default]
    #[unit("Oe", "Oersted", by = MagneticFieldIntensity::<N>::from(OERSTED))]
    Oersted,
}

/// The units of `MagneticFluxDensity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticFluxDensity<N>, number = N)]
#[unit(for<N> where MagneticFluxDensity<N>: [const] yadc::Unit<MagneticFluxDensity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MagneticFluxDensityUnit {
    #[default]
    #[unit("G", "Gauss", by = MagneticFluxDensity::<N>::from(GAUSS))]
    Gauss,
}

/// The units of `MagneticTension`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticTension<N>, number = N)]
#[unit(for<N> where MagneticTension<N>: [const] yadc::Unit<MagneticTension<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MagneticTensionUnit {
    #[default]
    #[unit("Gb", "Gilbert", by = MagneticTension::<N>::from(GILBERT))]
    Gilbert,
}

/// The units of `Radioactivity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Radioactivity<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Radioactivity<N>: [const] yadc::Unit<Radioactivity<N>, N> + [const] core::marker::Destruct)]
pub enum RadioactivityUnit {
    #[default]
    #[unit("Ci", "Curie", by = Radioactivity::<N>::from(CURIE))]
    Curie,
}
