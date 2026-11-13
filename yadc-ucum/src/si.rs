use crate::{Dim, Exact, Qty};
use yadc::Dimension;

pub type AmountOfSubstance<N> = Qty<{ Dim::DIMENSIONLESS }, N>;

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

pub type ElectricCapacitance<N> = Qty<
    {
        Dim {
            m: -2,
            s: 2,
            g: -1,
            C: 2,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricCharge<N> = Qty<
    {
        Dim {
            C: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricConductance<N> = Qty<
    {
        Dim {
            m: -2,
            s: 1,
            g: -1,
            C: 2,
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

pub type ElectricPotential<N> = Qty<
    {
        Dim {
            m: 2,
            s: -2,
            g: 1,
            C: -1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricResistance<N> = Qty<
    {
        Dim {
            m: 2,
            s: -1,
            g: 1,
            C: -2,
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

pub type Frequency<N> = Qty<
    {
        Dim {
            s: -1,
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

pub type Inductance<N> = Qty<
    {
        Dim {
            m: 2,
            g: 1,
            C: -2,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Length<N> = Qty<
    {
        Dim {
            m: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type LuminousFlux<N> = Qty<
    {
        Dim {
            rad: 2,
            cd: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type LuminousIntensity<N> = Qty<
    {
        Dim {
            cd: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type MagneticFlux<N> = Qty<
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

pub type Mass<N> = Qty<
    {
        Dim {
            g: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type PlaneAngle<N> = Qty<
    {
        Dim {
            rad: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Power<N> = Qty<
    {
        Dim {
            m: 2,
            s: -3,
            g: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Pressure<N> = Qty<
    {
        Dim {
            m: -1,
            s: -2,
            g: 1,
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

pub type SolidAngle<N> = Qty<
    {
        Dim {
            rad: 2,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Time<N> = Qty<
    {
        Dim {
            s: 1,
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
/// `const MPH: Velocity<f64> = (MILE / HOUR).into();`, then `MPH.quantity(60.0)`.
pub mod consts {
    use super::*;

    /// The number one: the unit of dimension one.
    pub use crate::base::ONE;

    /// The metre, UCUM's base unit of length.
    pub use crate::base::METRE;

    /// The second, UCUM's base unit of time.
    pub use crate::base::SECOND;

    /// The gram, UCUM's base unit of mass.
    pub use crate::base::GRAM;

    /// The radian, UCUM's base unit of plane angle.
    pub use crate::base::RADIAN;

    /// The coulomb, UCUM's base unit of electric charge.
    pub use crate::base::COULOMB;

    /// The candela, UCUM's base unit of luminous intensity.
    pub use crate::base::CANDELA;

    /// One `Mole` (`mol`), as an `AmountOfSubstance`.
    pub const MOLE: AmountOfSubstance<Exact> = ONE * Exact::int(602214076000000000000000);

    /// One `Sievert` (`Sv`), as a `DoseEquivalent`.
    pub const SIEVERT: DoseEquivalent<Exact> = METRE * METRE / (SECOND * SECOND);

    /// One `Farad` (`F`), as an `ElectricCapacitance`.
    pub const FARAD: ElectricCapacitance<Exact> =
        (SECOND * SECOND * COULOMB * COULOMB / (METRE * METRE * GRAM)) * Exact::dec(1, -3);

    /// One `Siemens` (`S`), as an `ElectricConductance`.
    pub const SIEMENS: ElectricConductance<Exact> =
        (SECOND * COULOMB * COULOMB / (METRE * METRE * GRAM)) * Exact::dec(1, -3);

    /// One `Ampere` (`A`), as an `ElectricCurrent`.
    pub const AMPERE: ElectricCurrent<Exact> = COULOMB / SECOND;

    /// One `Volt` (`V`), as an `ElectricPotential`.
    pub const VOLT: ElectricPotential<Exact> =
        (METRE * METRE * GRAM / (SECOND * SECOND * COULOMB)) * Exact::int(1000);

    /// One `Ohm` (`Ohm`), as an `ElectricResistance`.
    pub const OHM: ElectricResistance<Exact> =
        (METRE * METRE * GRAM / (SECOND * COULOMB * COULOMB)) * Exact::int(1000);

    /// One `Joule` (`J`), as an `Energy`.
    pub const JOULE: Energy<Exact> = (METRE * METRE * GRAM / (SECOND * SECOND)) * Exact::int(1000);

    /// One `Gray` (`Gy`), as an `EnergyDose`.
    pub const GRAY: EnergyDose<Exact> = METRE * METRE / (SECOND * SECOND);

    /// One `Newton` (`N`), as a `Force`.
    pub const NEWTON: Force<Exact> = (METRE * GRAM / (SECOND * SECOND)) * Exact::int(1000);

    /// One `Hertz` (`Hz`), as a `Frequency`.
    pub const HERTZ: Frequency<Exact> = ONE / SECOND;

    /// One `Lux` (`lx`), as an `Illuminance`.
    pub const LUX: Illuminance<Exact> = RADIAN * RADIAN * CANDELA / (METRE * METRE);

    /// One `Henry` (`H`), as an `Inductance`.
    pub const HENRY: Inductance<Exact> =
        (METRE * METRE * GRAM / (COULOMB * COULOMB)) * Exact::int(1000);

    /// One `Meter` (`m`), as a `Length`.
    pub const METER: Length<Exact> = METRE;

    /// One `Lumen` (`lm`), as a `LuminousFlux`.
    pub const LUMEN: LuminousFlux<Exact> = RADIAN * RADIAN * CANDELA;

    /// One `Weber` (`Wb`), as a `MagneticFlux`.
    pub const WEBER: MagneticFlux<Exact> =
        (METRE * METRE * GRAM / (SECOND * COULOMB)) * Exact::int(1000);

    /// One `Tesla` (`T`), as a `MagneticFluxDensity`.
    pub const TESLA: MagneticFluxDensity<Exact> = (GRAM / (SECOND * COULOMB)) * Exact::int(1000);

    /// One `Watt` (`W`), as a `Power`.
    pub const WATT: Power<Exact> =
        (METRE * METRE * GRAM / (SECOND * SECOND * SECOND)) * Exact::int(1000);

    /// One `Pascal` (`Pa`), as a `Pressure`.
    pub const PASCAL: Pressure<Exact> = (GRAM / (METRE * SECOND * SECOND)) * Exact::int(1000);

    /// One `Becquerel` (`Bq`), as a `Radioactivity`.
    pub const BECQUEREL: Radioactivity<Exact> = ONE / SECOND;

    /// One `Steradian` (`sr`), as a `SolidAngle`.
    pub const STERADIAN: SolidAngle<Exact> = RADIAN * RADIAN;
}
