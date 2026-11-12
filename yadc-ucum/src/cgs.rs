use crate::{Dim, Exact, Qty};
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
